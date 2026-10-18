// ✂️ c5d TypeScript module splitter: top-level statements of a snapshot -> sibling taxonomy folders with `🟦️.ts` leaves.
// Imports are re-synthesised per file from the identifiers each file really references; a non-exported declaration that crosses a file boundary gets `export`; no `export *` barrel and no re-export is created.
// usage: bun tssplit.ts list <tag> | stage <tag> | lossless <tag>      spec: specs-ts/<tag>.ts (default export), snapshot: orig/<tag>.ts
import {readFileSync, writeFileSync, mkdirSync, rmSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {fileURLToPath} from "node:url";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const HERE = dirname(fileURLToPath(import.meta.url));
const REPO = "/Users/ueli/Documents/semio";
const SCR = process.env.C5D_SCR ?? "/Users/ueli/.cache/c5d";
export type Child = {folder: string; doc: string; ranges: [number, number][]};
export type Spec = {orig: string; file: string; children: Child[]; header?: [number, number]; drop?: [number, number][]; redirect?: Record<string, string>; patch?: {folder: string; old: string; new: string}[]};
const V = "️";

type Stmt = {start: number; end: number; first: number; last: number; kind: string; names: string[]; text: string; exported: boolean; decl: ts.Statement};

function parse(text: string, name: string) {
  return ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
}

function declNames(s: ts.Statement): string[] {
  if (ts.isVariableStatement(s)) return s.declarationList.declarations.flatMap(d => bindingNames(d.name));
  if (ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isEnumDeclaration(s)) return s.name ? [s.name.text] : [];
  return [];
}
function bindingNames(b: ts.BindingName): string[] {
  if (ts.isIdentifier(b)) return [b.text];
  return b.elements.flatMap(e => ts.isOmittedExpression(e) ? [] : bindingNames(e.name));
}
function isExported(s: ts.Statement): boolean {
  return !!(ts.canHaveModifiers(s) && ts.getModifiers(s)?.some(m => m.kind === ts.SyntaxKind.ExportKeyword));
}

export function statements(text: string): {sf: ts.SourceFile; lines: string[]; stmts: Stmt[]; imports: ts.ImportDeclaration[]} {
  const sf = parse(text, "x.ts");
  const lines = text.split("\n");
  const stmts: Stmt[] = [];
  const imports: ts.ImportDeclaration[] = [];
  let prevEnd = 0;
  const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  for (const s of sf.statements) {
    const endLine = lineOf(s.getEnd());
    const startTok = lineOf(s.getStart());
    if (ts.isImportDeclaration(s)) { imports.push(s); prevEnd = endLine; continue; }
    const first = leadingStart(lines, prevEnd, startTok);
    let last = endLine;
    while (last < lines.length && /^\s*\/\/\s?#endregion\b/.test(lines[last]!)) last += 1;
    stmts.push({start: startTok, end: endLine, first, last, kind: ts.SyntaxKind[s.kind], names: declNames(s), text: lines.slice(first - 1, last).join("\n"), exported: isExported(s), decl: s});
    prevEnd = last;
  }
  for (let i = 0; i + 1 < stmts.length; i++) {
    const a = stmts[i]!, b = stmts[i + 1]!;
    const stray = [] as number[];
    for (let k = a.last + 1; k < b.first; k++) if (lines[k - 1]!.trim() !== "") stray.push(k);
    if (!stray.length) continue;
    const ends = stray.filter(k => /^\s*\/\/\s?#endregion\b/.test(lines[k - 1]!));
    const rest = stray.filter(k => !ends.includes(k));
    if (ends.length) a.last = Math.max(...ends);
    if (rest.length) b.first = Math.min(...rest);
    a.text = lines.slice(a.first - 1, a.last).join("\n");
    b.text = lines.slice(b.first - 1, b.last).join("\n");
  }
  return {sf, lines, stmts, imports};
}
function leadingStart(lines: string[], prevEnd: number, startTok: number): number {
  let first = startTok;
  for (let k = startTok - 1; k > prevEnd; k -= 1) {
    if (lines[k - 1]!.trim() === "") break;
    first = k;
  }
  return first;
}

function listing(tag: string) {
  const text = readFileSync(`${HERE}/orig/${tag}.ts`, "utf8");
  const {stmts, imports, lines} = statements(text);
  console.log(`imports ${imports.length}, statements ${stmts.length}, lines ${lines.length}`);
  for (const [i, s] of stmts.entries()) console.log(`${i}\t${s.first}-${s.last}\t${s.last - s.first + 1}\t${s.exported ? "export" : "      "}\t${s.kind.replace("Declaration", "").replace("Statement", "")}\t${s.names.join(",").slice(0, 70) || s.text.split("\n").find(l => !/^\s*(\/|\*)/.test(l))?.slice(0, 60)}`);
}

function deps(tag: string) {
  const text = readFileSync(`${HERE}/orig/${tag}.ts`, "utf8");
  const {stmts} = statements(text);
  const declared = new Map<string, number>();
  stmts.forEach((s, i) => s.names.forEach(n => declared.set(n, i)));
  for (const [i, s] of stmts.entries()) {
    const refs = [...referenced(s.text)].filter(r => declared.has(r) && declared.get(r) !== i).map(r => `${declared.get(r)}:${r}`);
    console.log(`${i}\t${s.names.join(",") || "(anon)"}\t->\t${refs.join(" ")}`);
  }
}

async function loadSpec(tag: string): Promise<Spec> { return (await import(`${HERE}/specs-ts/${tag}.ts`)).default as Spec; }

const REGION = /^\s*\/\/\s?#(end)?region\b/;
const IDENT_SKIP = (n: ts.Identifier): boolean => {
  const p = n.parent;
  if (!p) return false;
  if (ts.isPropertyAccessExpression(p) && p.name === n) return true;
  if (ts.isQualifiedName(p) && p.right === n) return true;
  if ((ts.isPropertyAssignment(p) || ts.isPropertySignature(p) || ts.isMethodDeclaration(p) || ts.isPropertyDeclaration(p) || ts.isMethodSignature(p) || ts.isEnumMember(p)) && p.name === n) return true;
  if (ts.isBindingElement(p) && p.propertyName === n) return true;
  if (ts.isLabeledStatement(p) || ts.isBreakOrContinueStatement(p)) return true;
  if (ts.isImportSpecifier(p) || ts.isExportSpecifier(p)) return true;
  return false;
};
function referenced(text: string): Set<string> {
  const sf = parse(text, "r.ts");
  const out = new Set<string>();
  const walk = (n: ts.Node): void => {
    if (ts.isIdentifier(n) && !IDENT_SKIP(n)) out.add(n.text);
    ts.forEachChild(n, walk);
  };
  walk(sf);
  return out;
}

type Binding = {name: string; typeOnly: boolean; kind: "named" | "default" | "namespace"; alias?: string};
type ImportInfo = {spec: string; bindings: Binding[]; typeOnlyDecl: boolean; sideEffect: boolean};
function importInfo(d: ts.ImportDeclaration): ImportInfo {
  const spec = (d.moduleSpecifier as ts.StringLiteral).text;
  const bindings: Binding[] = [];
  const c = d.importClause;
  if (c?.name) bindings.push({name: c.name.text, typeOnly: false, kind: "default"});
  if (c?.namedBindings) {
    if (ts.isNamespaceImport(c.namedBindings)) bindings.push({name: c.namedBindings.name.text, typeOnly: false, kind: "namespace"});
    else for (const e of c.namedBindings.elements) bindings.push({name: e.name.text, alias: e.propertyName?.text, typeOnly: e.isTypeOnly, kind: "named"});
  }
  return {spec, bindings, typeOnlyDecl: !!c?.isTypeOnly, sideEffect: !c};
}
function rebase(spec: string, fromDir: string, toDir: string): string {
  if (!spec.startsWith(".")) return spec;
  const abs = resolve(fromDir, spec);
  let rel = relative(toDir, abs);
  if (!rel.startsWith(".")) rel = "./" + rel;
  return rel;
}

export async function stage(tag: string) {
  const sp = await loadSpec(tag);
  const text = readFileSync(`${HERE}/orig/${tag}.ts`, "utf8");
  const {lines, stmts, imports} = statements(text);
  const rootAbs = join(REPO, sp.file);
  const rootDir = dirname(rootAbs);
  const owner = new Map<number, number>();
  sp.children.forEach((c, ci) => {
    for (const [a, b] of c.ranges) {
      stmts.forEach((s, i) => {
        if (s.last < a || s.first > b) return;
        if (!(a <= s.first && s.last <= b)) throw new Error(`${c.folder}: range ${a}-${b} cuts statement ${s.first}-${s.last} ${s.names.join(",")}`);
        if (owner.has(i)) throw new Error(`statement ${i} in two children`);
        owner.set(i, ci);
      });
    }
  });
  const dropped = new Set<number>();
  for (const [a, b] of sp.drop ?? []) stmts.forEach((s, i) => { if (a <= s.first && s.last <= b) dropped.add(i); else if (!(s.last < a || s.first > b)) throw new Error(`drop ${a}-${b} cuts statement ${s.first}-${s.last}`); });
  const fileOf = (i: number): string => owner.has(i) ? sp.children[owner.get(i)!]!.folder : "<root>";
  const declFile = new Map<string, string>();
  stmts.forEach((s, i) => { if (!dropped.has(i)) s.names.forEach(n => declFile.set(n, fileOf(i))); });
  const infos = imports.map(importInfo);
  const importLocal = new Map<string, {info: ImportInfo; b: Binding}>();
  for (const info of infos) for (const b of info.bindings) importLocal.set(b.name, {info, b});

  const bodies = new Map<string, number[]>();
  bodies.set("<root>", []);
  sp.children.forEach(c => bodies.set(c.folder, []));
  stmts.forEach((_, i) => { if (!dropped.has(i)) bodies.get(fileOf(i))!.push(i); });

  const needsExport = new Set<number>();
  const refs = new Map<string, Set<string>>();
  for (const [file, idxs] of bodies) {
    const set = new Set<string>();
    for (const i of idxs) for (const r of referenced(stmts[i]!.text)) set.add(r);
    refs.set(file, set);
  }
  for (const [file, set] of refs) {
    for (const r of set) {
      const df = declFile.get(r);
      if (df && df !== file) {
        const si = stmts.findIndex(s => s.names.includes(r));
        if (!stmts[si]!.exported) needsExport.add(si);
      }
    }
  }
  const outFiles = new Map<string, string>();
  const dirOf = (file: string) => file === "<root>" ? rootDir : join(rootDir, file);
  const pathOf = (file: string) => file === "<root>" ? rootAbs : join(rootDir, file, "🟦️.ts");
  const importsFor = (file: string): string[] => {
    const set = refs.get(file)!;
    const out: string[] = [];
    const dir = dirOf(file);
    for (const info of infos) {
      const used = info.bindings.filter(b => set.has(b.name));
      if (!used.length) continue;
      const spec = rebase(info.spec, rootDir, dir);
      out.push(fmtImport(used, spec, info.typeOnlyDecl));
    }
    const byFile = new Map<string, string[]>();
    for (const r of [...set].sort()) {
      const df = declFile.get(r);
      if (df && df !== file && !(infos.some(i => i.bindings.some(b => b.name === r)))) {
        if (!byFile.has(df)) byFile.set(df, []);
        byFile.get(df)!.push(r);
      }
    }
    for (const [df, names] of byFile) {
      const spec = rebase(pathOf(df), dir + "/x", dir).replace(/^\.\/?/, "./");
      const rel = relative(dir, pathOf(df));
      const s = rel.startsWith(".") ? rel : "./" + rel;
      const typeOnlyNames = new Set(names.filter(n => { const st = stmts.find(x => x.names.includes(n))!; return st.kind === "TypeAliasDeclaration" || st.kind === "InterfaceDeclaration"; }));
      const bs: Binding[] = names.map(n => ({name: n, typeOnly: typeOnlyNames.has(n), kind: "named" as const}));
      out.push(fmtImport(bs, s, false));
      void spec;
    }
    return out;
  };
  const RG = /^\s*\/\/\s?#region\b/, ER = /^\s*\/\/\s?#endregion\b/;
  const regionStack: string[][] = [];
  const bodyOf: string[] = [];
  {
    const stack: string[] = [];
    for (const [i, s] of stmts.entries()) {
      const ls = s.text.split("\n");
      const opens: string[] = [];
      while (ls.length && RG.test(ls[0]!)) opens.push(ls.shift()!);
      const closes: string[] = [];
      while (ls.length && ER.test(ls[ls.length - 1]!) && stack.length + opens.length - closes.length > 0) closes.unshift(ls.pop()!);
      for (const o of opens) stack.push(o);
      regionStack[i] = [...stack];
      for (let k = 0; k < closes.length; k++) stack.pop();
      bodyOf[i] = ls.join("\n");
      void i;
    }
  }
  const patched = new Set<string>();
  const render = (file: string): string => {
    const idxs = bodies.get(file)!;
    const parts: string[] = [];
    let prev = -1;
    let cur: string[] = [];
    const endName = (open: string) => open.replace(/^(\s*\/\/\s?)#region/, "$1#endregion");
    for (const i of idxs) {
      const s = stmts[i]!;
      let t = bodyOf[i]!;
      if (needsExport.has(i)) t = exportize(t);
      for (const pt of sp.patch ?? []) if (pt.folder === file && t.includes(pt.old)) { t = t.split(pt.old).join(pt.new); patched.add(pt.folder + "|" + pt.old); }
      const target = regionStack[i]!;
      let common = 0;
      while (common < cur.length && common < target.length && cur[common] === target[common]) common++;
      const closers = cur.slice(common).reverse().map(endName);
      const openers = target.slice(common);
      if (closers.length) parts.push("\n" + closers.join("\n"));
      if (openers.length) t = openers.join("\n") + "\n" + t;
      cur = [...target];
      let sep = "\n\n";
      if (prev >= 0) {
        const gap = lines.slice(stmts[prev]!.last, s.first - 1);
        if (prev + 1 === i && gap.every(g => g.trim() === "") && gap.length === 0) sep = "\n";
      }
      parts.push(prev < 0 ? t : sep + t);
      prev = i;
    }
    if (cur.length) parts.push("\n" + cur.slice().reverse().map(endName).join("\n"));
    return parts.join("");
  };
  for (const file of bodies.keys()) {
    const head: string[] = [];
    const c = sp.children.find(x => x.folder === file);
    if (c) head.push(`/** ${c.doc} */`);
    const body = render(file);
    const imp = importsFor(file);
    outFiles.set(file, (file === "<root>" ? rootHead(text, lines, sp) : "") + (head.length ? head.join("\n") + "\n" : "") + (imp.length ? imp.join("\n") + "\n\n" : file === "<root>" ? "" : "") + body + "\n");
  }
  for (const pt of sp.patch ?? []) if (!patched.has(pt.folder + "|" + pt.old)) throw new Error(`patch not applied: ${pt.folder} ${pt.old.slice(0, 60)}`);
  const d = join(SCR, "stage-ts", tag);
  rmSync(d, {recursive: true, force: true});
  const wr = (file: string, t: string) => { const p = join(d, relative(REPO, pathOf(file))); mkdirSync(dirname(p), {recursive: true}); writeFileSync(p, t); };
  for (const [file, t] of outFiles) wr(file, t);
  const map: Record<string, string> = {};
  for (const [name, file] of declFile) { if (file !== "<root>" || true) map[name] = pathOf(file); }
  for (const [name, rel] of Object.entries(sp.redirect ?? {})) map[name] = join(REPO, rel);
  mkdirSync(join(SCR, "stage-ts"), {recursive: true});
  writeFileSync(join(SCR, "stage-ts", `${tag}.map.json`), JSON.stringify({old: rootAbs, map}, null, 1));
  const sizes: Record<string, number> = {};
  for (const [file, t] of outFiles) sizes[file] = t.split("\n").length - 1;
  console.log("staged", tag, JSON.stringify(sizes), "exportized", [...needsExport].map(i => stmts[i]!.names.join(",")).join(" "));
}
function rootHead(text: string, lines: string[], sp: Spec): string {
  return (lines[0]!.startsWith("#!") ? lines[0] + "\n" : "") + (sp.header ? lines.slice(sp.header[0] - 1, sp.header[1]).join("\n") + "\n" : "");
}
function fmtImport(bs: Binding[], spec: string, typeOnlyDecl: boolean): string {
  const def = bs.find(b => b.kind === "default"), ns = bs.find(b => b.kind === "namespace"), named = bs.filter(b => b.kind === "named");
  const parts: string[] = [];
  if (def) parts.push(def.name);
  if (ns) parts.push(`* as ${ns.name}`);
  if (named.length) parts.push(`{ ${named.map(b => `${b.typeOnly && !typeOnlyDecl ? "type " : ""}${b.alias ? `${b.alias} as ${b.name}` : b.name}`).join(", ")} }`);
  const allType = typeOnlyDecl || (!def && !ns && named.length > 0 && named.every(b => b.typeOnly));
  const inner = named.map(b => `${b.typeOnly && !allType ? "type " : ""}${b.alias ? `${b.alias} as ${b.name}` : b.name}`).join(", ");
  const head = [def?.name, ns ? `* as ${ns.name}` : undefined, named.length ? `{ ${inner} }` : undefined].filter(Boolean).join(", ");
  void parts;
  return `import ${allType ? "type " : ""}${head} from "${spec}";`;
}
function exportize(text: string): string {
  const sf = parse(text, "e.ts");
  const first = sf.statements[0];
  if (!first) throw new Error("exportize: no statement");
  const at = first.getStart();
  return text.slice(0, at) + "export " + text.slice(at);
}

async function lossless(tag: string) {
  const sp = await loadSpec(tag);
  const text = readFileSync(`${HERE}/orig/${tag}.ts`, "utf8");
  const {lines, stmts, imports} = statements(text);
  const importLines = new Set<number>();
  for (const d of imports) { const a = d.getSourceFile().getLineAndCharacterOfPosition(d.getStart()).line + 1, b = d.getSourceFile().getLineAndCharacterOfPosition(d.getEnd()).line + 1; for (let k = a; k <= b; k++) importLines.add(k); }
  const norm = (l: string) => { let x = l; for (const pt of sp.patch ?? []) x = x.split(pt.old).join(pt.new); return x.trim().replace(/^export\s+(?=(?:async\s+)?(?:function|const|let|class|type|interface)\b)/, ""); };
  const O = new Map<string, number>();
  const bump = (m: Map<string, number>, k: string, n = 1) => m.set(k, (m.get(k) ?? 0) + n);
  const dropped = new Set<number>();
  for (const [a, b] of sp.drop ?? []) for (let k = a; k <= b; k++) dropped.add(k);
  const hdr = new Set<number>(); if (sp.header) for (let k = sp.header[0]; k <= sp.header[1]; k++) hdr.add(k);
  const MARK = /^\s*\/\/\s?#(end)?region\b/;
  let origOpen = 0, origClose = 0;
  lines.forEach((l, i) => { const n = i + 1; if (l.trim() === "" || importLines.has(n) || dropped.has(n) || (i === 0 && l.startsWith("#!"))) return; if (MARK.test(l)) { if (/#endregion/.test(l)) origClose++; else origOpen++; return; } bump(O, norm(l)); });
  let newOpen = 0, newClose = 0; const unbalanced: string[] = [];
  const rootDir = dirname(join(REPO, sp.file));
  const d = join(SCR, "stage-ts", tag);
  const N = new Map<string, number>();
  const files = [join(d, sp.file), ...sp.children.map(c => join(d, dirname(sp.file), c.folder, "🟦️.ts"))];
  const extra = new Map<string, number>();
  for (const f of files) {
    const body = readFileSync(f, "utf8").split("\n");
    const {imports: im} = statements(readFileSync(f, "utf8"));
    const il = new Set<number>(); const sfx = parse(readFileSync(f, "utf8"), "n.ts");
    for (const x of im) { const a = sfx.getLineAndCharacterOfPosition(x.getStart()).line + 1, b = sfx.getLineAndCharacterOfPosition(x.getEnd()).line + 1; for (let k = a; k <= b; k++) il.add(k); }
    const childDoc = sp.children.find(c => join(d, dirname(sp.file), c.folder, "🟦️.ts") === f)?.doc;
    let depth = 0, fo = 0, fc = 0, minDepth = 0;
    body.forEach((l, i) => { const n = i + 1; if (l.trim() === "" || il.has(n) || (i === 0 && l.startsWith("#!"))) return; if (childDoc && l.trim() === `/** ${childDoc} */`) { bump(extra, "doc"); return; } if (MARK.test(l)) { if (/#endregion/.test(l)) { newClose++; fc++; depth--; minDepth = Math.min(minDepth, depth); } else { newOpen++; fo++; depth++; } return; } bump(N, norm(l)); });
    if (fo !== fc) unbalanced.push(`${f.slice(-60)} opens ${fo} closes ${fc}`);
  }
  let missing = 0, added = 0;
  for (const [k, v] of O) { const n = N.get(k) ?? 0; if (n < v) { missing += v - n; console.log("  - missing", v - n, k.slice(0, 140)); } }
  for (const [k, v] of N) { const o = O.get(k) ?? 0; if (v > o) { added += v - o; console.log("  + extra", v - o, k.slice(0, 140)); } }
  console.log("original lines", [...O.values()].reduce((a, b) => a + b, 0), "new lines", [...N.values()].reduce((a, b) => a + b, 0), "missing", missing, "extra", added, "docs", extra.get("doc") ?? 0);
  console.log("regions orig open/close", origOpen, origClose, "new open/close", newOpen, newClose, "files with unbalanced markers (an orphan endregion of the original is allowed)", unbalanced);
  const regionOk = unbalanced.length === (origClose - origOpen > 0 ? 1 : 0);
  console.log("LOSSLESS", missing === 0 && added === 0 && regionOk ? "OK" : "FAIL");
}

if (import.meta.main) {
  const [cmd, tag] = process.argv.slice(2);
  if (cmd === "list") listing(tag!);
  else if (cmd === "deps") deps(tag!);
  else if (cmd === "stage") await stage(tag!);
  else if (cmd === "lossless") await lossless(tag!);
  else throw new Error("usage: list|stage <tag>");
}
