import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, posix } from "node:path";
import { analyze, parse, relSpec, ROOT, ts, type Decl, type FileAnalysis } from "./ast.ts";
import { resolveRel } from "./exports.ts";

export type Module = { dir: string; ranges: ([string, string] | [number, number])[]; names?: string[]; keep?: boolean };
export type Plan = { source: string; baseDir: string; modules: Module[]; leaf?: string; stateModule?: string };

const FE0F = "️";

export function seg(s: string): string {
  const m = s.match(/^([^|]+)\|(.+)$/u);
  if (!m) return s;
  const emoji = m[1].endsWith(FE0F) || m[1].includes("‍") ? m[1] : m[1] + FE0F;
  return emoji + m[2];
}

export function dirOf(plan: Plan, m: Module): string {
  return m.dir === "." ? plan.baseDir : join(plan.baseDir, ...m.dir.split("/").map(seg));
}

export type Built = {
  mod: Module;
  file: string;
  decls: Decl[];
  text: string;
  imports: Map<string, { ids: Map<string, boolean> }>;
  internalDeps: Map<string, Set<string>>;
};

const leafName = (plan: Plan) => plan.leaf ?? "🟦️.ts";

export function assign(plan: Plan, a: FileAnalysis): Map<number, Module> {
  const owner = new Map<number, Module>();
  for (const m of plan.modules) {
    const indexes = new Set<number>();
    for (const [from, to] of m.ranges) {
      if (typeof from === "number" && typeof to === "number") {
        a.decls.forEach((d, k) => {
          if (d.startLine >= from && d.endLine <= to) indexes.add(k);
        });
        continue;
      }
      const i = a.topNames.get(from as string);
      const j = a.topNames.get(to as string);
      if (i === undefined) throw new Error(`unknown name ${from} in ${m.dir}`);
      if (j === undefined) throw new Error(`unknown name ${to} in ${m.dir}`);
      for (let k = i; k <= j; k++) indexes.add(k);
    }
    for (const n of m.names ?? []) {
      const i = a.topNames.get(n);
      if (i === undefined) throw new Error(`unknown name ${n} in ${m.dir}`);
      indexes.add(i);
    }
    for (const k of indexes) {
      if (owner.has(k)) throw new Error(`decl ${a.decls[k].names.join(",")} assigned twice (${owner.get(k)!.dir} and ${m.dir})`);
      owner.set(k, m);
    }
  }
  return owner;
}

export function dropOrphanDocs(text: string, start: number, end: number): string {
  const ranges = (ts.getLeadingCommentRanges(text, Math.max(0, start - 1)) ?? []).filter((r) => r.pos >= start && r.end <= end);
  const cuts: [number, number][] = [];
  for (const r of ranges) {
    if (r.kind !== ts.SyntaxKind.MultiLineCommentTrivia || !text.startsWith("/**", r.pos)) continue;
    const after = text.slice(r.end, end);
    const gap = after.match(/^\s*/u)![0];
    if ((gap.match(/\n/gu) ?? []).length >= 3 || after.slice(gap.length).startsWith("/**")) cuts.push([r.pos, r.end + gap.length]);
  }
  let out = "";
  let cursor = start;
  for (const [a, b] of cuts) {
    out += text.slice(cursor, a);
    cursor = b;
  }
  out += text.slice(cursor, end);
  return out;
}

function balanceRegions(text: string): string {
  const lines = text.split("\n");
  const stack: { name: string; line: number }[] = [];
  const drop = new Set<number>();
  lines.forEach((l, i) => {
    const s = l.match(/^\s*\/\/#region\s*(.*)$/u);
    const e = l.match(/^\s*\/\/#endregion\s*(.*)$/u);
    if (s) stack.push({ name: s[1].trim(), line: i });
    else if (e) {
      const top = stack[stack.length - 1];
      if (top && top.name === e[1].trim()) stack.pop();
      else drop.add(i);
    }
  });
  for (const s of stack) drop.add(s.line);
  return lines.filter((_, i) => !drop.has(i)).join("\n");
}


/** 🧷 c7w: module-level `let` bindings written from a module other than the one that declares them are written through exported setters (ESM imports are read-only; reads stay live bindings). */
export function topLevelLets(a: FileAnalysis): Set<string> {
  const out = new Set<string>();
  for (const st of a.sf.statements) if (ts.isVariableStatement(st) && (st.declarationList.flags & ts.NodeFlags.Let)) for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) out.add(d.name.text);
  return out;
}

export const setterName = (name: string): string => "set" + name[0].toUpperCase() + name.slice(1);

export function rewriteLetWrites(text: string, lets: Set<string>, path: string): { text: string; written: Set<string> } {
  const sf = parse(path, text);
  type Edit = { start: number; end: number; build: (inner: (s: number, e: number) => string) => string };
  const edits: Edit[] = [];
  const written = new Set<string>();
  const ident = (e: import("typescript").Expression) => (ts.isIdentifier(e) && lets.has(e.text) ? e.text : undefined);
  const visit = (n: import("typescript").Node) => {
    if (ts.isBinaryExpression(n) && n.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && n.operatorToken.kind <= ts.SyntaxKind.LastAssignment) {
      const name = ident(n.left);
      if (name) {
        written.add(name);
        const op = n.operatorToken.getText(sf), rs = n.right.getStart(sf), re = n.right.getEnd();
        const set = setterName(name);
        if (op === "=") edits.push({ start: n.getStart(sf), end: n.getEnd(), build: (inner) => `${set}(${inner(rs, re)})` });
        else if (["??=", "||=", "&&="].includes(op)) edits.push({ start: n.getStart(sf), end: n.getEnd(), build: (inner) => `(${name} ${op.slice(0, -1)} ${set}(${inner(rs, re)}))` });
        else edits.push({ start: n.getStart(sf), end: n.getEnd(), build: (inner) => `${set}(${name} ${op.slice(0, -1)} (${inner(rs, re)}))` });
        ts.forEachChild(n.right, visit);
        return;
      }
    }
    if ((ts.isPrefixUnaryExpression(n) || ts.isPostfixUnaryExpression(n)) && (n.operator === ts.SyntaxKind.PlusPlusToken || n.operator === ts.SyntaxKind.MinusMinusToken)) {
      const name = ident(n.operand);
      if (name) {
        written.add(name);
        const plus = n.operator === ts.SyntaxKind.PlusPlusToken;
        const call = `${setterName(name)}(${name} ${plus ? "+" : "-"} 1)`;
        const postfixValue = ts.isPostfixUnaryExpression(n) && !ts.isExpressionStatement(n.parent);
        edits.push({ start: n.getStart(sf), end: n.getEnd(), build: () => (postfixValue ? `(${call} ${plus ? "-" : "+"} 1)` : call) });
        return;
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const render = (s: number, e: number): string => {
    let out = "", cursor = s;
    for (const ed of edits.filter((x) => x.start >= s && x.end <= e).sort((x, y) => x.start - y.start || y.end - x.end)) {
      if (ed.start < cursor) continue;
      out += text.slice(cursor, ed.start) + ed.build(render);
      cursor = ed.end;
    }
    return out + text.slice(cursor, e);
  };
  return { text: render(0, text.length), written };
}

export function build(plan: Plan, a: FileAnalysis, extraExported = new Set<string>()): Built[] {
  const owner = assign(plan, a);
  const unassigned = a.decls.map((d, i) => i).filter((i) => !owner.has(i));
  const moduleOf = new Map<string, Module>();
  a.decls.forEach((d, i) => {
    const m = owner.get(i);
    if (m) for (const n of d.names) moduleOf.set(n, m);
  });
  const lets = topLevelLets(a);
  const letOwner = new Map<string, Module>();
  a.decls.forEach((d, i) => { const m = owner.get(i); if (m) for (const n of d.names) if (lets.has(n)) letOwner.set(n, m); });
  const setterNeeds = new Map<Module, Set<string>>();
  const importBy = new Map<string, { spec: string; imported: string; typeOnly: boolean; kind: string }>();
  for (const imp of a.imports) for (const b of imp.bindings) importBy.set(b.local, { spec: imp.spec, imported: b.imported, typeOnly: b.typeOnly, kind: b.kind });
  const out: Built[] = [];
  const sourceDir = dirname(a.path);
  for (const m of plan.modules) {
    const file = join(dirOf(plan, m), leafName(plan));
    const idx = a.decls.map((d, i) => i).filter((i) => owner.get(i) === m);
    const decls = idx.map((i) => a.decls[i]);
    const own = new Set(decls.flatMap((d) => d.names));
    const imports = new Map<string, { ids: Map<string, boolean> }>();
    const internalDeps = new Map<string, Set<string>>();
    const addImport = (spec: string, id: string, typeOnly: boolean) => {
      const e = imports.get(spec) ?? imports.set(spec, { ids: new Map() }).get(spec)!;
      e.ids.set(id, (e.ids.get(id) ?? true) && typeOnly);
    };
    for (const d of decls) {
      if (lets.size) {
        const foreign = new Set([...lets].filter((l) => letOwner.get(l) !== m));
        const { written } = rewriteLetWrites(a.text.slice(d.start, d.end), foreign, a.path);
        for (const w of written) {
          const ownerMod = letOwner.get(w);
          if (!ownerMod) throw new Error(`let ${w} written in ${m.dir} has no owner module`);
          const ownerFile = join(dirOf(plan, ownerMod), leafName(plan));
          addImport(relSpec(file, ownerFile), setterName(w), false);
          (internalDeps.get(ownerFile) ?? internalDeps.set(ownerFile, new Set()).get(ownerFile)!).add(setterName(w));
          (setterNeeds.get(ownerMod) ?? setterNeeds.set(ownerMod, new Set()).get(ownerMod)!).add(w);
        }
      }
      for (const name of d.deps) {
        if (name.startsWith("?") || own.has(name)) continue;
        const other = moduleOf.get(name);
        if (other) {
          const spec = relSpec(file, join(dirOf(plan, other), leafName(plan)));
          const target = a.decls[a.topNames.get(name)!];
          addImport(spec, name, target.typeOnly);
          (internalDeps.get(join(dirOf(plan, other), leafName(plan))) ?? internalDeps.set(join(dirOf(plan, other), leafName(plan)), new Set()).get(join(dirOf(plan, other), leafName(plan)))!).add(name);
        } else {
          const imp = importBy.get(name);
          if (!imp) {
            if (a.topNames.has(name)) throw new Error(`decl ${name} used by ${m.dir} is unassigned`);
            continue;
          }
          const r = imp.spec.startsWith(".") ? resolveRel(a.path, imp.spec) : null;
          const spec = r ? relSpec(file, r) : imp.spec;
          const id = imp.kind === "default" ? `default:${name}` : imp.kind === "namespace" ? `ns:${name}` : imp.imported === name ? name : `${imp.imported} as ${name}`;
          addImport(spec, id, imp.typeOnly);
        }
      }
    }
    out.push({ mod: m, file, decls, text: "", imports, internalDeps });
  }
  const externallyNeeded = new Set<string>(extraExported);
  for (const b of out) for (const [target, names] of b.internalDeps) for (const n of names) externallyNeeded.add(n);
  for (const b of out) {
    const parts: string[] = [];
    for (const d of b.decls) {
      let t = dropOrphanDocs(a.text, d.start, d.end);
      const fileForSpecs = b.file;
      t = t.replace(/\bimport\(\s*(["'])(\.\.?\/[^"']+)\1\s*\)/gu, (_m, q: string, spec: string) => {
        const r = resolveRel(a.path, spec);
        return `import(${q}${r ? relSpec(fileForSpecs, r) : spec}${q})`;
      });
      const needs = !d.exported && d.names.some((n) => externallyNeeded.has(n));
      if (needs) {
        const node = a.sf.statements[d.index];
        const off = t.length - (d.end - node.getStart(a.sf));
        t = t.slice(0, off) + "export " + t.slice(off);
      }
      if (lets.size) {
        const foreign = new Set([...lets].filter((l) => letOwner.get(l) !== b.mod));
        t = rewriteLetWrites(t, foreign, a.path).text;
      }
      parts.push(t.trimEnd());
    }
    for (const w of setterNeeds.get(b.mod) ?? []) {
      parts.push(`/** 🧷 Setter for the module-level \`${w}\`: importers read the live binding and write through this call. */\nexport function ${setterName(w)}(value: typeof ${w}): typeof ${w} {\n  ${w} = value;\n  return value;\n}`);
    }
    let body = parts.join("\n\n");
    body = balanceRegions(body);
    for (const { ids } of b.imports.values()) for (const id of [...ids.keys()]) if (lets.has(id) && !new RegExp(`(?<![\\w$.])${id}(?![\\w$])`, "u").test(body)) ids.delete(id);
    const importLines: string[] = [];
    const sorted = [...b.imports.entries()].sort((x, y) => (x[0].startsWith(".") ? 1 : 0) - (y[0].startsWith(".") ? 1 : 0));
    for (const [spec, { ids }] of sorted) {
      const defaults = [...ids.keys()].filter((k) => k.startsWith("default:"));
      const namespaces = [...ids.keys()].filter((k) => k.startsWith("ns:"));
      const named = [...ids.entries()].filter(([k]) => !k.startsWith("default:") && !k.startsWith("ns:"));
      const allType = named.length > 0 && named.every(([, t]) => t) && defaults.length === 0 && namespaces.length === 0;
      const items = named.map(([k, t]) => (!allType && t ? `type ${k}` : k)).sort((x, y) => x.replace(/^type /u, "").localeCompare(y.replace(/^type /u, "")));
      if (defaults.length) importLines.push(`import ${defaults[0].slice(8)}${items.length ? `, { ${items.join(", ")} }` : ""} from "${spec}";`);
      else if (namespaces.length) importLines.push(`import * as ${namespaces[0].slice(3)} from "${spec}";`);
      else if (items.length) importLines.push(`import ${allType ? "type " : ""}{ ${items.join(", ")} } from "${spec}";`);
    }
    b.text = (importLines.length ? importLines.join("\n") + "\n\n" : "") + body + "\n";
    b.text = b.text.replace(/\n{3,}/gu, "\n\n");
  }
  if (unassigned.length) {
    console.log("UNASSIGNED:", unassigned.map((i) => a.decls[i].names.join(",") || a.decls[i].kind + "@" + a.decls[i].startLine).join(" | "));
  }
  return out;
}

export function moduleGraph(built: Built[]): { cycles: string[][] } {
  const nodes = built.map((b) => b.file);
  const adj = new Map<string, Set<string>>();
  for (const b of built) adj.set(b.file, new Set([...b.internalDeps.keys()].filter((k) => k !== b.file)));
  let idx = 0;
  const index = new Map<string, number>();
  const low = new Map<string, number>();
  const on = new Set<string>();
  const stack: string[] = [];
  const cycles: string[][] = [];
  const strong = (v: string) => {
    index.set(v, idx);
    low.set(v, idx);
    idx++;
    stack.push(v);
    on.add(v);
    for (const w of adj.get(v) ?? []) {
      if (!index.has(w)) {
        strong(w);
        low.set(v, Math.min(low.get(v)!, low.get(w)!));
      } else if (on.has(w)) low.set(v, Math.min(low.get(v)!, index.get(w)!));
    }
    if (low.get(v) === index.get(v)) {
      const comp: string[] = [];
      let w: string;
      do {
        w = stack.pop()!;
        on.delete(w);
        comp.push(w);
      } while (w !== v);
      if (comp.length > 1) cycles.push(comp);
    }
  };
  for (const n of nodes) if (!index.has(n)) strong(n);
  return { cycles };
}

export function write(built: Built[], dry: boolean, sourcePath = ""): void {
  for (const b of built) {
    if (dry) continue;
    mkdirSync(dirname(join(ROOT, b.file)), { recursive: true });
    if (existsSync(join(ROOT, b.file)) && b.file !== sourcePath) throw new Error(`refusing to overwrite ${b.file}`);
    writeFileSync(join(ROOT, b.file), b.text);
  }
}

export function surfaceOf(plan: Plan, a: FileAnalysis, built: Built[], exportMapOf: (file: string) => Map<string, { file: string; name: string; typeOnly: boolean }>): { map: Map<string, string>; warnings: string[]; missingStars: string[] } {
  const missingStars: string[] = [];
  const map = new Map<string, string>();
  const warnings: string[] = [];
  const importBy = new Map<string, { spec: string; imported: string }>();
  for (const imp of a.imports) for (const b of imp.bindings) importBy.set(b.local, { spec: imp.spec, imported: b.imported });
  const fileOfName = new Map<string, string>();
  for (const b of built) for (const d of b.decls) for (const n of d.names) fileOfName.set(n, b.file);
  for (const d of a.decls) if (d.exported) for (const n of d.names) {
    const f = fileOfName.get(n);
    if (f) map.set(n, f);
  }
  const sf = a.sf;
  for (const e of a.exportsFrom) {
    const st = sf.statements[e.index] as import("typescript").ExportDeclaration;
    if (e.spec === null) {
      if (!st.exportClause || !ts.isNamedExports(st.exportClause)) continue;
      for (const el of st.exportClause.elements) {
        const local = (el.propertyName ?? el.name).text;
        if (el.name.text !== local) warnings.push(`alias export ${local} as ${el.name.text}`);
        const imp = importBy.get(local);
        if (imp) {
          if (imp.imported !== local) warnings.push(`aliased import re-exported ${imp.imported} as ${local}`);
          const r = imp.spec.startsWith(".") ? resolveRel(a.path, imp.spec) : null;
          if (r) map.set(el.name.text, r);
          else warnings.push(`external re-export ${local} from ${imp.spec}`);
        } else {
          const f = fileOfName.get(local);
          if (f) map.set(el.name.text, f);
          else warnings.push(`local export ${local} has no module`);
        }
      }
    } else {
      const target = resolveRel(a.path, e.spec);
      if (!target) {
        warnings.push(`external export-from ${e.spec}`);
        continue;
      }
      if (e.star) continue;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const el of st.exportClause.elements) {
        if (el.propertyName && el.propertyName.text !== el.name.text) warnings.push(`alias export-from ${el.propertyName.text} as ${el.name.text}`);
        map.set(el.name.text, target);
      }
    }
  }
  for (const e of a.exportsFrom) {
    if (e.spec === null || !e.star) continue;
    const target = resolveRel(a.path, e.spec);
    if (!target) continue;
    try {
      for (const [n] of exportMapOf(target)) if (!map.has(n)) map.set(n, target);
    } catch {
      missingStars.push(target);
    }
  }
  return { map, warnings, missingStars };
}
