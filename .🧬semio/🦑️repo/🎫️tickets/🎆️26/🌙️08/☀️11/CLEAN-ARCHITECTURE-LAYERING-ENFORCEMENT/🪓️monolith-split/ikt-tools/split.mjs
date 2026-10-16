import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { analyze } from "./analyze.mjs";

const REGION = /^\/\/\s*#(end)?region\b.*$/gm;

export async function run(cfgPath, mode) {
  const cfg = (await import(cfgPath)).default;
  const file = cfg.file;
  const P = path.dirname(file);
  const a = analyze(file);
  const hash = crypto.createHash("sha256").update(a.text).digest("hex");
  const { units, text } = a;

  // same-line trailing comments belong to the previous unit
  const spans = units.map((u) => ({ full: u.fullStart, end: u.end }));
  for (let i = 1; i < units.length; i++) {
    const m = /^[ \t]*(\/\/[^\n]*|\/\*[^\n]*?\*\/[ \t]*)/.exec(text.slice(units[i].fullStart));
    if (m && (i === 0 || !/\n/.test(m[0]))) {
      const prev = spans[i - 1];
      if (prev.end === units[i].fullStart || /^[ \t]*$/.test(text.slice(prev.end, units[i].fullStart))) {
        spans[i - 1].end = units[i].fullStart + m[0].length;
        spans[i].full = units[i].fullStart + m[0].length;
      }
    }
  }

  const modKey = new Map();
  for (const u of units) {
    const k = cfg.assign(u, a);
    if (!k) throw new Error(`unassigned unit ${u.idx} ${u.names} line ${a.lineOf(u.start)}`);
    modKey.set(u.idx, k);
  }
  const groupMod = new Map();
  for (const u of units) {
    const root = a.find(u.idx);
    const k = modKey.get(u.idx);
    if (groupMod.has(root) && groupMod.get(root) !== k) throw new Error(`merged group split across modules: ${u.names} ${groupMod.get(root)} vs ${k}`);
    groupMod.set(root, k);
  }
  const modules = cfg.modules;
  const byKey = new Map(modules.map((m) => [m.key, m]));
  const keep = cfg.keepKey ?? "parent";
  const modUnits = new Map([[keep, []], ...modules.map((m) => [m.key, []])]);
  for (const u of units) {
    if (!modUnits.has(modKey.get(u.idx))) throw new Error(`unknown module ${modKey.get(u.idx)}`);
    modUnits.get(modKey.get(u.idx)).push(u);
  }

  const dirOf = (k) => (k === keep ? P : path.join(P, byKey.get(k).dir));
  const extOf = (k) => {
    if (k === keep) return path.extname(file);
    const m = byKey.get(k);
    if (m.ext) return m.ext;
    return modUnits.get(k).some((u) => a.jsx[u.idx]) ? ".tsx" : ".ts";
  };
  const fileOf = (k) => (k === keep ? file : path.join(dirOf(k), "🟦️" + extOf(k)));
  const rel = (fromDir, toAbs) => {
    let r = path.relative(fromDir, toAbs);
    if (!r.startsWith(".")) r = "./" + r;
    return r;
  };

  // cross-module requirements
  const needExport = new Set();
  const crossNames = new Map(); // module -> Map(targetModule -> Map(name -> typeOnly))
  for (const k of modUnits.keys()) crossNames.set(k, new Map());
  for (const u of units) {
    const from = modKey.get(u.idx);
    for (const [tIdx, names] of a.nameRefs[u.idx]) {
      const to = modKey.get(tIdx);
      if (to === from) continue;
      const tu = units[tIdx];
      if (!tu.exported) needExport.add(tIdx);
      const per = crossNames.get(from);
      if (!per.has(to)) per.set(to, new Map());
      for (const n of names) {
        const group = units.filter((x) => a.find(x.idx) === a.find(tIdx));
        const typeOnly = group.every((x) => x.isType);
        const cur = per.get(to).get(n);
        per.get(to).set(n, cur === undefined ? typeOnly : cur && typeOnly);
      }
    }
  }
  for (const idx of [...needExport]) {
    const root = a.find(idx);
    for (const x of units) if (a.find(x.idx) === root && !x.exported && x.names.some((n) => units[idx].names.includes(n))) needExport.add(x.idx);
  }

  const unitText = (u) => {
    const sp = spans[u.idx];
    let seg = text.slice(sp.full, sp.end);
    if (needExport.has(u.idx) && !u.exported) {
      const off = u.start - sp.full;
      seg = seg.slice(0, off) + "export " + seg.slice(off);
    }
    return seg.replace(REGION, "");
  };

  const importStmt = (d, used, fromDir) => {
    const spec = d.spec.startsWith(".") ? rel(fromDir, path.resolve(P, d.spec)) : d.spec;
    const named = d.names.filter((e) => used.has(e.local));
    const def = d.defaultName && used.has(d.defaultName) ? d.defaultName : null;
    const ns = d.nsName && used.has(d.nsName) ? d.nsName : null;
    if (!named.length && !def && !ns) return null;
    const fmt = (e) => `${e.typeOnly && !d.typeOnly ? "type " : ""}${e.imported === e.local ? e.local : `${e.imported} as ${e.local}`}`;
    const attrs = d.attrs ? ` ${d.attrs}` : "";
    const parts = [];
    if (def) parts.push(def);
    if (ns) parts.push(`* as ${ns}`);
    let clause = parts.join(", ");
    if (named.length) {
      const body = named.map(fmt);
      const braces = body.length > 4 || body.join(", ").length > 120 ? `{\n${body.map((b) => "  " + b).join(",\n")}\n}` : `{ ${body.join(", ")} }`;
      clause = clause ? `${clause}, ${braces}` : braces;
    }
    return `import ${d.typeOnly ? "type " : ""}${clause} from ${JSON.stringify(spec)}${attrs};`;
  };

  for (const [from, per] of crossNames) for (const to of per.keys()) if (to === keep && from !== keep) throw new Error(`child ${from} depends on kept parent`);
  const out = new Map();
  const report = { hash, modules: {} };
  for (const k of modUnits.keys()) {
    const us = modUnits.get(k);
    const ext = us.length ? null : null;
    const used = new Set();
    for (const u of us) for (const n of a.importRefs[u.idx]) used.add(n);
    const imps = [];
    for (const d of a.importDecls) {
      const s = importStmt(d, used, dirOf(k));
      if (s) imps.push(s);
    }
    const sibs = [];
    const per = crossNames.get(k);
    const order = [...modUnits.keys()];
    for (const to of order) {
      if (!per.has(to)) continue;
      const names = [...per.get(to)].sort((x, y) => x[0].localeCompare(y[0]));
      const body = names.map(([n, t]) => `${t ? "type " : ""}${n}`);
      const spec = rel(dirOf(k), fileOf(to));
      const braces = body.length > 4 || body.join(", ").length > 120 ? `{\n${body.map((b) => "  " + b).join(",\n")}\n}` : `{ ${body.join(", ")} }`;
      sibs.push(`import ${braces} from ${JSON.stringify(spec)};`);
    }
    const code = us.filter((u) => !u.exportDecl).map((u) => unitText(u)).join("");
    out.set(k, { imps, sibs, code, units: us });
    report.modules[k] = { units: us.length, lines: code.split("\n").length, deps: [...per.keys()] };
  }

  // barrel exports
  const barrel = [];
  for (const m of modules) {
    const us = modUnits.get(m.key).filter((u) => u.exported);
    if (!us.length) continue;
    const seen = new Set();
    const body = [];
    for (const u of us) for (const n of u.names) {
      if (seen.has(n)) continue;
      seen.add(n);
      const root = a.find(u.idx);
      const typeOnly = units.filter((x) => a.find(x.idx) === root).every((x) => x.isType);
      body.push(`${typeOnly ? "type " : ""}${n}`);
    }
    body.sort((x, y) => x.replace("type ", "").localeCompare(y.replace("type ", "")));
    const spec = rel(P, fileOf(m.key));
    barrel.push(`export {\n${body.map((b) => "  " + b).join(",\n")}\n} from ${JSON.stringify(spec)};`);
  }

  // export declarations (kept in parent)
  const keptExport = [];
  for (const u of modUnits.get(keep)) {
    if (!u.exportDecl) continue;
    const st = u.stmt;
    let seg = text.slice(spans[u.idx].full, spans[u.idx].end).replace(REGION, "");
    if (!st.moduleSpecifier && st.exportClause) {
      const names = st.exportClause.elements;
      const bySpec = new Map();
      for (const el of names) {
        const local = (el.propertyName ?? el.name).text;
        const imp = a.imports.get(local);
        if (!imp) throw new Error(`export of non-import ${local}`);
        const d = imp.d;
        const key = d.spec + "|" + (st.isTypeOnly ? "t" : "v");
        if (!bySpec.has(key)) bySpec.set(key, { d, items: [] });
        const e = imp.e;
        bySpec.get(key).items.push(`${el.isTypeOnly && !st.isTypeOnly ? "type " : ""}${e && e.imported !== el.name.text ? `${e.imported} as ${el.name.text}` : el.name.text}`);
      }
      const doc = seg.slice(0, seg.indexOf(st.getText()));
      seg = doc + [...bySpec.values()].map(({ d, items }) => `export ${st.isTypeOnly ? "type " : ""}{ ${items.join(", ")} } from ${JSON.stringify(d.spec)};`).join("\n");
    }
    keptExport.push(seg);
  }
  return { cfg, a, out, barrel, keptExport, modUnits, fileOf, dirOf, extOf, report, hash, spans, keep, P, file, modules };
}

export function render(res, k) {
  const { cfg, out } = res;
  const o = out.get(k);
  const m = res.modules.find((x) => x.key === k);
  const chunks = [];
  if (k === res.keep) {
    chunks.push(cfg.parentHeader(res));
    if (o.imps.length || o.sibs.length) chunks.push([...o.imps, ...o.sibs].join("\n"));
    if (res.barrel.length && !cfg.noBarrel) chunks.push(res.barrel.join("\n\n"));
    if (res.keptExport.length) chunks.push(res.keptExport.map((s) => s.replace(/^\s+/, "")).join("\n"));
    chunks.push(o.code.replace(/^\s+/, "").replace(/\s+$/, "") );
  } else {
    chunks.push(cfg.childDoc ? cfg.childDoc(m) : `/** ${m.emoji} ${m.doc}\n *\n * 🧱️ Child module of {@link ../🟦️${path.extname(res.file)} | ${cfg.parentName}}; importers import this module directly.\n */`);
    if (o.imps.length || o.sibs.length) chunks.push([...o.imps, ...o.sibs].join("\n"));
    chunks.push(o.code.replace(/^\s+/, "").replace(/\s+$/, ""));
  }
  const text = chunks.filter(Boolean).join("\n\n").replace(/\n{3,}/g, "\n\n") + "\n";
  return cfg.postProcess ? cfg.postProcess(k, text, res) : text;
}
