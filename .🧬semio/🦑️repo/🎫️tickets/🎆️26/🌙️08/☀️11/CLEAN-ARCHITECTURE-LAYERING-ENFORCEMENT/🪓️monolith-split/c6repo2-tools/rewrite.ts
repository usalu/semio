import { ts } from "./lib.ts";
import { findSites, resolveSpec } from "./sites.ts";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

export type Target = { kind: "file"; abs: string; imported: string; typeOnly?: boolean } | { kind: "bare"; spec: string; imported: string; typeOnly?: boolean };
export type Lookup = (name: string) => Target | undefined;

interface Edit { start: number; end: number; text: string; }
export interface RewriteResult { file: string; edits: number; manual: string[]; }

function relSpec(fromFile: string, toAbs: string): string {
  let r = relative(dirname(fromFile), toAbs).split("\\").join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
}

function lineOf(sf: import("typescript").SourceFile, pos: number) {
  return sf.getLineAndCharacterOfPosition(pos).line + 1;
}


function nsUsages(sf: import("typescript").SourceFile, scope: import("typescript").Node, id: string, skip: import("typescript").Node): { names: Set<string>; nodes: { start: number; end: number; name: string }[] } | undefined {
  const names = new Set<string>();
  const nodes: { start: number; end: number; name: string }[] = [];
  let ok = true;
  const visit = (n: import("typescript").Node) => {
    if (n === skip) return;
    if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === id) {
      names.add(n.name.text);
      nodes.push({ start: n.getStart(sf), end: n.getEnd(), name: n.name.text });
      return;
    }
    if (ts.isQualifiedName(n) && ts.isIdentifier(n.left) && n.left.text === id) {
      names.add(n.right.text);
      nodes.push({ start: n.getStart(sf), end: n.getEnd(), name: n.right.text });
      return;
    }
    if (ts.isIdentifier(n) && n.text === id) {
      const par = n.parent;
      if (par && ts.isPropertyAccessExpression(par) && par.name === n) return;
      if (par && ts.isQualifiedName(par) && par.right === n) return;
      if (par && (ts.isPropertyAssignment(par) || ts.isPropertySignature(par)) && par.name === n) return;
      ok = false;
    }
    ts.forEachChild(n, visit);
  };
  visit(scope);
  return ok ? { names, nodes } : undefined;
}

export function rewriteFile(file: string, parents: Map<string, Lookup>, write: boolean): RewriteResult {
  const text = readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const sites = findSites(file, new Set(parents.keys()));
  const manual: string[] = [];
  const edits: Edit[] = [];
  const collides = (name: string, _nsId: string) => {
    let hit = false;
    const v = (n: import("typescript").Node) => {
      if (hit) return;
      if (ts.isIdentifier(n) && n.text === name) {
        const par = n.parent;
        const keyPos = par && ((ts.isPropertyAccessExpression(par) && par.name === n) || (ts.isQualifiedName(par) && par.right === n) || ((ts.isPropertyAssignment(par) || ts.isPropertySignature(par) || ts.isMethodSignature(par) || ts.isPropertyDeclaration(par) || ts.isMethodDeclaration(par) || ts.isEnumMember(par)) && par.name === n) || (ts.isBindingElement(par) && par.propertyName === n));
        if (!keyPos) { hit = true; return; }
      }
      ts.forEachChild(n, v);
    };
    v(sf);
    return hit;
  };
  if (!sites.length) return { file, edits: 0, manual };
  const existingBindings = new Map<string, string>();
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !st.importClause?.namedBindings || !ts.isNamedImports(st.importClause.namedBindings)) continue;
    const abs = resolveSpec(file, (st.moduleSpecifier as import("typescript").StringLiteral).text);
    if (!abs) continue;
    for (const el of st.importClause.namedBindings.elements) existingBindings.set(el.name.text, abs + "\u0000" + (el.propertyName ?? el.name).text);
  }
  const alreadyBound = (nm: string, t: Target) => t.kind === "file" && existingBindings.get(nm) === t.abs + "\u0000" + t.imported;
  const siteAt = new Map<number, (typeof sites)[number]>();
  sites.forEach((s) => siteAt.set(s.start, s));

  const findNode = (start: number): import("typescript").Node | undefined => {
    let found: import("typescript").Node | undefined;
    const v = (n: import("typescript").Node) => {
      if (found) return;
      if (n.getStart(sf) === start && ts.isStringLiteralLike(n)) { found = n; return; }
      if (n.pos <= start && start < n.end) ts.forEachChild(n, v);
    };
    v(sf);
    return found;
  };

  const specFor = (parentAbs: string, t: Target): string => (t.kind === "file" ? relSpec(file, t.abs) : t.spec);
  const targetKey = (t: Target) => (t.kind === "file" ? t.abs : "bare:" + t.spec);
  const quoteOf = (lit: import("typescript").Node) => (lit.getText(sf)[0] === "'" ? "'" : '"');

  // static imports/exports: per-file grouping
  type Group = { key: string; spec: string; items: string[]; typeClause: boolean };
  const staticTouched: { node: import("typescript").ImportDeclaration | import("typescript").ExportDeclaration; groups: Group[]; q: string; semi: boolean }[] = [];

  for (const s of sites) {
    const lit = findNode(s.start);
    if (!lit) { manual.push(`${file}: cannot locate literal at ${s.start}`); continue; }
    const parentAbs = resolveSpec(file, s.spec)!;
    const lookup = parents.get(parentAbs)!;
    const node = lit.parent;
    if (s.kind === "import" && ts.isImportDeclaration(node)) {
      const ic = node.importClause;
      if (!ic) { manual.push(`${file}:${lineOf(sf, s.start)} side-effect import of split module`); continue; }
      if (ic.name) { manual.push(`${file}:${lineOf(sf, s.start)} default import`); continue; }
      if (!ic.namedBindings) continue;
      if (ts.isNamespaceImport(ic.namedBindings)) {
        const id = ic.namedBindings.name.text;
        const use = nsUsages(sf, sf, id, node);
        if (!use) { manual.push(`${file}:${lineOf(sf, s.start)} namespace import ${id} used other than by member access`); continue; }
        const groups = new Map<string, Group>();
        let bad = false;
        for (const nm of use.names) {
          const t = lookup(nm);
          if (!t) { manual.push(`${file}:${lineOf(sf, s.start)} unknown export ${nm}`); bad = true; continue; }
          if (t.imported !== nm) { manual.push(`${file}:${lineOf(sf, s.start)} renamed reexport ${nm}`); bad = true; continue; }
          if (alreadyBound(nm, t)) continue;
          if (collides(nm, id)) { manual.push(`${file}:${lineOf(sf, s.start)} name collision ${nm}`); bad = true; continue; }
          const key = targetKey(t);
          const g = groups.get(key) ?? groups.set(key, { key, spec: specFor(parentAbs, t), items: [], typeClause: ic.isTypeOnly }).get(key)!;
          g.items.push((t.typeOnly && !ic.isTypeOnly ? "type " : "") + nm);
        }
        if (bad) continue;
        use.nodes.forEach((u) => edits.push({ start: u.start, end: u.end, text: u.name }));
        staticTouched.push({ node, groups: [...groups.values()], q: quoteOf(lit), semi: text[node.getEnd() - 1] === ";" });
        continue;
      }
      const groups = new Map<string, Group>();
      let bad = false;
      for (const el of ic.namedBindings.elements) {
        const imported = (el.propertyName ?? el.name).text;
        const t = lookup(imported);
        if (!t) { manual.push(`${file}:${lineOf(sf, s.start)} unknown export ${imported}`); bad = true; continue; }
        const key = targetKey(t);
        const g = groups.get(key) ?? groups.set(key, { key, spec: specFor(parentAbs, t), items: [], typeClause: ic.isTypeOnly }).get(key)!;
        const tname = t.imported;
        const local = el.name.text;
        const typeMod = el.isTypeOnly || (t.typeOnly && !ic.isTypeOnly) ? "type " : "";
        g.items.push(`${typeMod}${tname === local ? local : `${tname} as ${local}`}`);
      }
      if (bad) continue;
      staticTouched.push({ node, groups: [...groups.values()], q: quoteOf(lit), semi: text[node.getEnd() - 1] === ";" });
    } else if (s.kind === "export" && ts.isExportDeclaration(node)) {
      if (!node.exportClause || !ts.isNamedExports(node.exportClause)) { manual.push(`${file}:${lineOf(sf, s.start)} export star`); continue; }
      const groups = new Map<string, Group>();
      let bad = false;
      for (const el of node.exportClause.elements) {
        const imported = (el.propertyName ?? el.name).text;
        const t = lookup(imported);
        if (!t) { manual.push(`${file}:${lineOf(sf, s.start)} unknown export ${imported}`); bad = true; continue; }
        const key = targetKey(t);
        const g = groups.get(key) ?? groups.set(key, { key, spec: specFor(parentAbs, t), items: [], typeClause: node.isTypeOnly }).get(key)!;
        const local = el.name.text;
        const typeMod = el.isTypeOnly || (t.typeOnly && !node.isTypeOnly) ? "type " : "";
        g.items.push(`${typeMod}${t.imported === local ? local : `${t.imported} as ${local}`}`);
      }
      if (bad) continue;
      staticTouched.push({ node, groups: [...groups.values()], q: quoteOf(lit), semi: text[node.getEnd() - 1] === ";" });
    } else if (s.kind === "dynamic" && ts.isCallExpression(node)) {
      handleDynamic(node, lit as import("typescript").StringLiteralLike, parentAbs, lookup);
    } else {
      manual.push(`${file}:${lineOf(sf, s.start)} ${s.kind} ${s.node}: ${text.slice(Math.max(0, s.start - 40), s.end + 20).replace(/\n/g, " ")}`);
    }
  }

  function handleDynamic(call: import("typescript").CallExpression, lit: import("typescript").StringLiteralLike, parentAbs: string, lookup: Lookup) {
    const where = `${file}:${lineOf(sf, lit.getStart(sf))}`;
    let expr: import("typescript").Node = call;
    let p = expr.parent;
    if (p && ts.isAwaitExpression(p)) { expr = p; p = expr.parent; }
    const q = quoteOf(lit);
    const replaceSpec = (t: Target) => edits.push({ start: lit.getStart(sf), end: lit.getEnd(), text: q + specFor(parentAbs, t) + q });
    if (p && ts.isVariableDeclaration(p) && p.initializer === expr) {
      if (ts.isObjectBindingPattern(p.name)) {
        const groups = new Map<string, { t: Target; els: string[] }>();
        for (const el of p.name.elements) {
          if (el.dotDotDotToken || !ts.isIdentifier(el.name) || (el.propertyName && !ts.isIdentifier(el.propertyName))) { manual.push(`${where} complex binding`); return; }
          const imported = (el.propertyName ?? el.name).text;
          const t = lookup(imported);
          if (!t) { manual.push(`${where} unknown export ${imported}`); return; }
          const k = targetKey(t);
          const g = groups.get(k) ?? groups.set(k, { t, els: [] }).get(k)!;
          const local = el.name.text;
          g.els.push(t.imported === local ? local : `${t.imported}: ${local}`);
        }
        if (groups.size === 1) { replaceSpec([...groups.values()][0].t); return; }
        const pieces = [...groups.values()].map((g) => `{ ${g.els.join(", ")} } = ${ts.isAwaitExpression(expr) ? "await " : ""}import(${q}${specFor(parentAbs, g.t)}${q})`);
        edits.push({ start: p.getStart(sf), end: p.getEnd(), text: pieces.join(", ") });
        return;
      }
      if (ts.isIdentifier(p.name)) {
        const id = p.name.text;
        const names = new Set<string>();
        let other = false;
        const scope = p.parent.parent.parent;
        const visit = (n: import("typescript").Node) => {
          if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === id) names.add(n.name.text);
          else if (ts.isElementAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === id) other = true;
          else if (ts.isIdentifier(n) && n.text === id && n !== p.name && !(n.parent && ts.isPropertyAccessExpression(n.parent) && n.parent.expression === n)) {
            if (!(n.parent && ts.isPropertyAccessExpression(n.parent) && n.parent.name === n)) other = true;
          }
          ts.forEachChild(n, visit);
        };
        visit(scope && ts.isBlock(scope) ? scope : sf);
        if (other) { manual.push(`${where} namespace variable ${id} used other than by property access`); return; }
        const ts_: Target[] = [];
        for (const nm of names) { const t = lookup(nm); if (!t) { manual.push(`${where} unknown export ${nm}`); return; } ts_.push(t); }
        const keys = new Set(ts_.map(targetKey));
        if (keys.size === 1) {
          const names2 = new Set(ts_.map((t) => t.imported));
          if (ts_.every((t, i) => t.imported === [...names][i])) { replaceSpec(ts_[0]); return; }
          void names2;
        }
        if (keys.size === 0) { replaceSpec({ kind: "file", abs: parentAbs, imported: "" } as Target); manual.push(`${where} namespace variable ${id} unused`); return; }
        const use = nsUsages(sf, scope && ts.isBlock(scope) ? scope : sf, id, p);
        if (!use) { manual.push(`${where} namespace variable ${id} spans ${keys.size} targets: ${[...names].join(",")}`); return; }
        const groups = new Map<string, { t: Target; els: string[] }>();
        for (const nm of use.names) {
          const t = lookup(nm)!;
          if (t.imported !== nm) { manual.push(`${where} renamed reexport ${nm}`); return; }
          if (alreadyBound(nm, t)) continue;
          if (collides(nm, id)) { manual.push(`${where} name collision ${nm}`); return; }
          const k = targetKey(t);
          (groups.get(k) ?? groups.set(k, { t, els: [] }).get(k)!).els.push(nm);
        }
        if (groups.size === 0) {
          const list = p.parent;
          if (ts.isVariableDeclarationList(list)) {
            if (list.declarations.length === 1) {
              const stmt = list.parent;
              let end = stmt.getEnd();
              if (text[end] === "\n") end++;
              const ls = text.lastIndexOf("\n", stmt.getStart(sf) - 1) + 1;
              edits.push({ start: /^\s*$/.test(text.slice(ls, stmt.getStart(sf))) ? ls : stmt.getStart(sf), end, text: "" });
            } else {
              const idx = list.declarations.indexOf(p);
              if (idx < list.declarations.length - 1) edits.push({ start: p.getStart(sf), end: list.declarations[idx + 1]!.getStart(sf), text: "" });
              else edits.push({ start: list.declarations[idx - 1]!.getEnd(), end: p.getEnd(), text: "" });
            }
            use.nodes.forEach((u) => edits.push({ start: u.start, end: u.end, text: u.name }));
            return;
          }
        }
        const pieces = [...groups.values()].map((g) => `{ ${g.els.join(", ")} } = ${ts.isAwaitExpression(expr) ? "await " : ""}import(${q}${specFor(parentAbs, g.t)}${q})`);
        edits.push({ start: p.getStart(sf), end: p.getEnd(), text: pieces.join(", ") });
        use.nodes.forEach((u) => edits.push({ start: u.start, end: u.end, text: u.name }));
        return;
      }
    }
    if (p && ts.isParenthesizedExpression(p) && ts.isAwaitExpression(call.parent) ) { /* handled below */ }
    // (await import(P)).name
    let q2: import("typescript").Node = expr;
    if (q2.parent && ts.isParenthesizedExpression(q2.parent)) q2 = q2.parent;
    if (q2.parent && ts.isPropertyAccessExpression(q2.parent) && q2.parent.expression === q2) {
      const t = lookup(q2.parent.name.text);
      if (!t) { manual.push(`${where} unknown export ${q2.parent.name.text}`); return; }
      if (t.imported !== q2.parent.name.text) { manual.push(`${where} renamed reexport ${q2.parent.name.text}`); return; }
      replaceSpec(t);
      return;
    }
    manual.push(`${where} dynamic import context: ${text.slice(Math.max(0, call.getStart(sf) - 50), call.getEnd() + 40).replace(/\n/g, " ")}`);
  }

  // dedupe statements across file, merge with existing untouched imports
  const touchedNodes = new Set(staticTouched.map((t) => t.node));
  const existing = new Map<string, import("typescript").ImportDeclaration>();
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || touchedNodes.has(st)) continue;
    const ic = st.importClause;
    if (!ic || ic.name || !ic.namedBindings || !ts.isNamedImports(ic.namedBindings)) continue;
    const abs = resolveSpec(file, (st.moduleSpecifier as import("typescript").StringLiteral).text);
    if (!abs) continue;
    const key = `${ic.isTypeOnly ? "T" : "V"}|import|${abs}`;
    if (!existing.has(key)) existing.set(key, st);
  }
  const mergedInto = new Map<string, string[]>();
  const firstEmit = new Map<string, { idx: number }>();
  const outputs: { t: (typeof staticTouched)[number]; lines: string[] }[] = staticTouched.map((t) => ({ t, lines: [] }));
  const specAbs = (spec: string) => resolveSpec(file, spec);
  outputs.forEach((o, oi) => {
    const isImport = ts.isImportDeclaration(o.t.node);
    for (const g of o.t.groups) {
      const abs = specAbs(g.spec);
      const kind = isImport ? "import" : "export";
      const key = abs ? `${g.typeClause ? "T" : "V"}|${kind}|${abs}` : `${g.typeClause ? "T" : "V"}|${kind}|bare:${g.spec}`;
      if (isImport && existing.has(key)) {
        const arr = mergedInto.get(key) ?? mergedInto.set(key, []).get(key)!;
        for (const it of g.items) if (!arr.includes(it)) arr.push(it);
        continue;
      }
      const prior = firstEmit.get(key);
      if (prior) {
        const po = outputs[prior.idx];
        const line = po.lines.findIndex((l) => l.startsWith(`${kind === "import" ? "import" : "export"}${g.typeClause ? " type" : ""} { `) && l.includes(`from ${o.t.q}${g.spec}${o.t.q}`));
        if (line >= 0) {
          const m = po.lines[line].match(/^(.* \{ )(.*)( \} from .*)$/)!;
          const have = m[2].split(", ");
          for (const it of g.items) if (!have.includes(it)) have.push(it);
          po.lines[line] = m[1] + have.join(", ") + m[3];
          continue;
        }
      }
      if (!prior) firstEmit.set(key, { idx: oi });
      const head = isImport ? `import${g.typeClause ? " type" : ""}` : `export${g.typeClause ? " type" : ""}`;
      o.lines.push(`${head} { ${g.items.join(", ")} } from ${o.t.q}${g.spec}${o.t.q}${o.t.semi ? ";" : ""}`);
    }
  });
  outputs.forEach((o) => {
    const node = o.t.node;
    if (o.lines.length) {
      const indent = text.slice(text.lastIndexOf("\n", node.getStart(sf) - 1) + 1, node.getStart(sf)).match(/^\s*/)![0];
      edits.push({ start: node.getStart(sf), end: node.getEnd(), text: o.lines.join("\n" + indent) });
    } else {
      let end = node.getEnd();
      if (text[end] === "\n") end++;
      let start = node.getStart(sf);
      const ls = text.lastIndexOf("\n", start - 1) + 1;
      if (/^\s*$/.test(text.slice(ls, start))) start = ls;
      edits.push({ start, end, text: "" });
    }
  });
  for (const [key, items] of mergedInto) {
    const st = existing.get(key)!;
    const nb = st.importClause!.namedBindings as import("typescript").NamedImports;
    const have = new Set(nb.elements.map((e) => e.getText(sf)));
    const add = items.filter((i) => !have.has(i));
    if (!add.length) continue;
    const last = nb.elements[nb.elements.length - 1];
    edits.push({ start: last.getEnd(), end: last.getEnd(), text: ", " + add.join(", ") });
  }
  if (write && edits.length) {
    edits.sort((a, b) => b.start - a.start);
    let out = text;
    for (const e of edits) out = out.slice(0, e.start) + e.text + out.slice(e.end);
    writeFileSync(file, out);
  }
  return { file, edits: edits.length, manual };
}

export function lookupFrom(table: Map<string, any>, parentAbs: string, chunkFiles: string[]): Lookup {
  return (name: string) => {
    const e = table.get(name);
    if (!e) return undefined;
    if (e.chunk !== undefined) return { kind: "file", abs: chunkFiles[e.chunk], imported: e.localAlias ?? name };
    const r = e.reexport;
    if (r.spec.startsWith(".")) return { kind: "file", abs: resolve(dirname(parentAbs), r.spec), imported: r.imported, typeOnly: r.typeOnly };
    return { kind: "bare", spec: r.spec, imported: r.imported, typeOnly: r.typeOnly };
  };
}
