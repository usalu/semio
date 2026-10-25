import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";

const root = "/Users/ueli/Documents/semio";
const [mode, source, dest, ...names] = process.argv.slice(2) as string[];
const apply = mode === "apply";
let moved = new Set(names);
const rel = (from: string, to: string) => { let r = relative(dirname(join(root, from)), join(root, to)).split("\\").join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const text = readFileSync(join(root, source!), "utf8");
const sf = ts.createSourceFile(source!, text, ts.ScriptTarget.Latest, true);
const declNames = (st: ts.Statement): string[] => {
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.flatMap((d) => ts.isIdentifier(d.name) ? [d.name.text] : []);
  if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) return [st.name.text];
  return [];
};
const isTypeDecl = (st: ts.Statement) => ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st);
const topDecl = new Map<string, ts.Statement>();
for (const st of sf.statements) for (const n of declNames(st)) topDecl.set(n, st);
const importOf = new Map<string, { st: ts.ImportDeclaration; spec: ts.ImportSpecifier | null; kind: "named" | "default" | "ns"; typeOnly: boolean }>();
for (const st of sf.statements) if (ts.isImportDeclaration(st)) {
  const c = st.importClause; if (!c) continue;
  if (c.name) importOf.set(c.name.text, { st, spec: null, kind: "default", typeOnly: c.isTypeOnly });
  if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) importOf.set(c.namedBindings.name.text, { st, spec: null, kind: "ns", typeOnly: c.isTypeOnly }); else for (const e of c.namedBindings.elements) importOf.set(e.name.text, { st, spec: e, kind: "named", typeOnly: c.isTypeOnly || e.isTypeOnly }); }
}
const statements = [...new Set(names.map((n) => { const st = topDecl.get(n); if (!st) throw Error("top-level declaration not found: " + n); return st; }))].sort((a, b) => a.getStart() - b.getStart());
const stSet = new Set(statements);
moved = new Set(statements.flatMap(declNames));
const allNames = [...moved];
const refsOf = (nodes: readonly ts.Node[]): Set<string> => {
  const out = new Set<string>();
  const visit = (n: ts.Node) => {
    if (ts.isIdentifier(n)) {
      const p = n.parent;
      const skip = (ts.isPropertyAccessExpression(p) && p.name === n) || (ts.isPropertyAssignment(p) && p.name === n) || (ts.isPropertySignature(p) && p.name === n) || (ts.isMethodDeclaration(p) && p.name === n) || (ts.isMethodSignature(p) && p.name === n) || (ts.isBindingElement(p) && p.propertyName === n) || (ts.isQualifiedName(p) && p.right === n) || (ts.isPropertyDeclaration(p) && p.name === n) || ts.isImportSpecifier(p) || (ts.isExportSpecifier(p) && !!(p.parent.parent as ts.ExportDeclaration).moduleSpecifier) || (ts.isLabeledStatement(p));
      if (!skip) out.add(n.text);
    }
    ts.forEachChild(n, visit);
  };
  for (const n of nodes) visit(n);
  return out;
};
const movedRefs = refsOf(statements);
const exportedInSource = (st: ts.Statement) => (ts.getCombinedModifierFlags(st as ts.Declaration) & ts.ModifierFlags.Export) !== 0;
const blockedAll = [...movedRefs].filter((r) => !moved.has(r) && topDecl.has(r));
const blocked = blockedAll.filter((r) => !exportedInSource(topDecl.get(r)!));
const fromSource = blockedAll.filter((r) => exportedInSource(topDecl.get(r)!));
if (blocked.length) { console.log("BLOCKED by non-moved, non-exported top-level declarations:", blocked.join(",")); process.exit(1); }
const remaining = sf.statements.filter((st) => !stSet.has(st) && !ts.isImportDeclaration(st));
const remainingRefs = refsOf(remaining);
const needExport = new Set<string>();
for (const n of allNames) if (exportedInSource(topDecl.get(n)!) || remainingRefs.has(n)) needExport.add(n);

const specText = (e: ts.ImportSpecifier) => e.getText(sf);
const buildImports = (refs: Set<string>, fromFile: string, toFile: string): string[] => {
  const grouped = new Map<ts.ImportDeclaration, { def?: string; ns?: string; named: string[]; typeOnlyAll: boolean }>();
  for (const r of refs) {
    const im = importOf.get(r); if (!im) continue;
    const g = grouped.get(im.st) ?? grouped.set(im.st, { named: [], typeOnlyAll: im.st.importClause!.isTypeOnly }).get(im.st)!;
    if (im.kind === "default") g.def = r; else if (im.kind === "ns") g.ns = r; else g.named.push(specText(im.spec!));
  }
  const out: string[] = [];
  for (const [st, g] of grouped) {
    const spec = (st.moduleSpecifier as ts.StringLiteral).text; const target = spec.startsWith(".") ? rel(toFile, normalize(join(dirname(fromFile), spec.split("?")[0]!))) + (spec.includes("?") ? "?" + spec.split("?")[1] : "") : spec;
    const attr = st.attributes ? " " + st.attributes.getText(sf) : "";
    const type = g.typeOnlyAll ? "type " : "";
    if (g.ns) out.push(`import ${type}* as ${g.ns} from "${target}"${attr};`);
    else out.push(`import ${type}${g.def ?? ""}${g.def && g.named.length ? "," : ""}${g.named.length ? "{ " + g.named.join(", ") + " }" : ""} from "${target}"${attr};`);
  }
  return out;
};
const destImports = buildImports(movedRefs, source!, dest!);
if (fromSource.length) { const values = fromSource.filter((n) => !isTypeDecl(topDecl.get(n)!)), types = fromSource.filter((n) => isTypeDecl(topDecl.get(n)!)); destImports.push(`import { ${[...values, ...types.map((n) => "type " + n)].join(", ")} } from "${rel(dest!, source!)}";`); }
const destBody = statements.map((st) => {
  let t = text.slice(st.getFullStart(), st.getEnd()).replace(/^\s*\n/u, "");
  const decl = declNames(st);
  if (!exportedInSource(st) && decl.some((n) => needExport.has(n))) t = t.replace(/^(\s*(?:\/\*\*[\s\S]*?\*\/\s*)?)/u, "$1export ");
  return t.trim();
});
const dts = "";
const destText = [...destImports, "", ...destBody].join("\n") + "\n" + dts;

const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?|d\.mts)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
const edits = new Map<string, { start: number; end: number; text: string }[]>();
const manual: string[] = [];
for (const from of files) {
  if (from === source || from === dest) continue;
  const t = readFileSync(join(root, from), "utf8");
  if (!allNames.some((n) => t.includes(n))) continue;
  const f = ts.createSourceFile(from, t, ts.ScriptTarget.Latest, true);
  const newSpec = rel(from, dest!);
  const visit = (st: ts.Node) => {
    if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text.startsWith(".") && normalize(join(dirname(from), st.moduleSpecifier.text)) === source) {
      const elements = ts.isImportDeclaration(st) ? (st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) ? st.importClause.namedBindings.elements : null) : (st.exportClause && ts.isNamedExports(st.exportClause) ? st.exportClause.elements : null);
      if (!elements) { if (allNames.some((n) => t.includes(n))) manual.push(from + ": non-named import/export of source"); return; }
      const mv = elements.filter((e) => moved.has((e.propertyName ?? e.name).text)); if (!mv.length) return;
      const keep = elements.filter((e) => !mv.includes(e));
      const quote = t[st.moduleSpecifier.getStart(f)]!;
      const origSpec = t.slice(st.moduleSpecifier.getStart(f), st.moduleSpecifier.getEnd());
      const attr = (st as ts.ImportDeclaration).attributes ? " " + (st as ts.ImportDeclaration).attributes!.getText(f) : "";
      let replacement: string;
      if (ts.isImportDeclaration(st)) {
        const c = st.importClause!; const typeOnly = c.isTypeOnly; const parts: string[] = [];
        if (keep.length || c.name) parts.push(`import ${typeOnly ? "type " : ""}${c.name ? c.name.text + (keep.length ? ", " : "") : ""}${keep.length ? "{ " + keep.map((e) => e.getText(f)).join(", ") + " }" : ""} from ${origSpec}${attr};`);
        parts.push(`import ${typeOnly ? "type " : ""}{ ${mv.map((e) => e.getText(f)).join(", ")} } from ${quote}${newSpec}${quote}${attr};`);
        replacement = parts.join(" ");
      } else {
        const parts: string[] = [];
        if (keep.length) parts.push(`export ${st.isTypeOnly ? "type " : ""}{ ${keep.map((e) => e.getText(f)).join(", ")} } from ${origSpec};`);
        parts.push(`export ${st.isTypeOnly ? "type " : ""}{ ${mv.map((e) => e.getText(f)).join(", ")} } from ${quote}${newSpec}${quote};`);
        replacement = parts.join(" ");
      }
      (edits.get(from) ?? edits.set(from, []).get(from)!).push({ start: st.getStart(f), end: st.getEnd(), text: replacement });
    } else if (ts.isCallExpression(st) && st.arguments[0] && ts.isStringLiteralLike(st.arguments[0]) && st.arguments[0].text.startsWith(".") && normalize(join(dirname(from), st.arguments[0].text)) === source && (st.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(st.expression) && st.expression.text === "require"))) {
      manual.push(from + ": dynamic/require of source (line " + (f.getLineAndCharacterOfPosition(st.getStart(f)).line + 1) + ")");
    } else if (ts.isImportTypeNode(st) && ts.isLiteralTypeNode(st.argument) && ts.isStringLiteral(st.argument.literal) && st.argument.literal.text.startsWith(".") && normalize(join(dirname(from), st.argument.literal.text)) === source) {
      manual.push(from + ": import() type of source (line " + (f.getLineAndCharacterOfPosition(st.getStart(f)).line + 1) + ")");
    }
    ts.forEachChild(st, visit);
  };
  visit(f);
}

const backNames = allNames.filter((n) => remainingRefs.has(n));
let out = text;
const cuts = statements.map((st) => ({ start: st.getFullStart(), end: st.getEnd() })).sort((a, b) => b.start - a.start);
for (const c of cuts) out = out.slice(0, c.start) + out.slice(c.end);
if (backNames.length) {
  const values = backNames.filter((n) => !isTypeDecl(topDecl.get(n)!)), types = backNames.filter((n) => isTypeDecl(topDecl.get(n)!));
  const spec = [...values, ...types.map((n) => "type " + n)].join(", ");
  const sfOut = ts.createSourceFile(source!, out, ts.ScriptTarget.Latest, true);
  const lastImport = [...sfOut.statements].filter(ts.isImportDeclaration).pop();
  const line = `\nimport { ${spec} } from "${rel(source!, dest!)}";`;
  out = lastImport ? out.slice(0, lastImport.getEnd()) + line + out.slice(lastImport.getEnd()) : line.trimStart() + "\n" + out;
}
// prune imports of the source that are no longer referenced
{
  const sf2 = ts.createSourceFile(source!, out, ts.ScriptTarget.Latest, true);
  const bodyRefs = refsOf(sf2.statements.filter((st) => !ts.isImportDeclaration(st)));
  const cuts2: { start: number; end: number; text: string }[] = [];
  for (const st of sf2.statements) if (ts.isImportDeclaration(st) && st.importClause) {
    const c = st.importClause; const used = (n: string) => bodyRefs.has(n) || [...bodyRefs].some(() => false);
    const keepDefault = c.name && used(c.name.text) ? c.name.text : undefined;
    let keepNamed: ts.ImportSpecifier[] = []; let ns: string | undefined;
    if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) ns = used(c.namedBindings.name.text) ? c.namedBindings.name.text : undefined; else keepNamed = c.namedBindings.elements.filter((e) => used(e.name.text)); }
    const before = (c.name ? 1 : 0) + (c.namedBindings ? (ts.isNamespaceImport(c.namedBindings) ? 1 : c.namedBindings.elements.length) : 0);
    const after = (keepDefault ? 1 : 0) + (ns ? 1 : 0) + keepNamed.length;
    if (after === before) continue;
    if (!after) { cuts2.push({ start: st.getFullStart(), end: st.getEnd(), text: "" }); continue; }
    const origSpec = out.slice(st.moduleSpecifier.getStart(sf2), st.moduleSpecifier.getEnd()); const attr = st.attributes ? " " + st.attributes.getText(sf2) : "";
    const body = ns ? `* as ${ns}` : `${keepDefault ?? ""}${keepDefault && keepNamed.length ? "," : ""}${keepNamed.length ? "{ " + keepNamed.map((e) => e.getText(sf2)).join(", ") + " }" : ""}`;
    cuts2.push({ start: st.getStart(sf2), end: st.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}${body} from ${origSpec}${attr};` });
  }
  for (const c of cuts2.sort((a, b) => b.start - a.start)) out = out.slice(0, c.start) + c.text + out.slice(c.end);
}
console.log("moved", allNames.length, "decls; exported in dest:", [...needExport].join(","), "; source-back-imports:", backNames.length, "; importers re-pointed:", edits.size);
if (manual.length) console.log("MANUAL:\n" + manual.join("\n"));
if (apply) {
  for (const [from, list] of edits) { let t = readFileSync(join(root, from), "utf8"); for (const e of list.sort((a, b) => b.start - a.start)) t = t.slice(0, e.start) + e.text + t.slice(e.end); writeFileSync(join(root, from), t); }
  mkdirSync(dirname(join(root, dest!)), { recursive: true });
  if (existsSync(join(root, dest!))) throw Error("destination exists: " + dest);
  writeFileSync(join(root, dest!), destText);
  writeFileSync(join(root, source!), out);
}
