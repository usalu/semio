import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio/";
const [, , specPath, mode] = process.argv;
const apply = mode === "apply";
const spec = JSON.parse(readFileSync(specPath!, "utf8"));
const targets = new Map<string, Set<string>>();
for (const rows of Object.values<any>(spec.byPackage)) for (const r of Object.values<any>(rows)) for (const i of r.importers) (targets.get(i) ?? targets.set(i, new Set()).get(i)!).add(ROOT + r.targetFile);
const escape = (s: string): string => s.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
let removedStatements = 0, removedNames = 0, edgesRemoved = 0, filesTouched = 0, docOnly = 0;
const log: any[] = [];
for (const [file, tset] of targets) {
  const abs = ROOT + file;
  const src = readFileSync(abs, "utf8");
  const sf = ts.createSourceFile(abs, src, ts.ScriptTarget.Latest, true);
  const ast = new Set<string>();
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isIdentifier(n)) ast.add(n.text);
    if (ts.isExportSpecifier(n)) ast.add((n.propertyName ?? n.name).text);
    if (ts.isShorthandPropertyAssignment(n)) ast.add(n.name.text);
    ts.forEachChild(n, visit);
  };
  sf.forEachChild(visit);
  const importRanges = sf.statements.filter(ts.isImportDeclaration).map((s) => [s.getStart(sf), s.getEnd()] as const);
  const outside = (name: string): boolean => {
    const re = new RegExp(`(?<![\\p{L}\\p{N}_$])${escape(name)}(?![\\p{L}\\p{N}_$])`, "gu");
    for (const m of src.matchAll(re)) if (!importRanges.some(([a, b]) => m.index! >= a && m.index! < b)) return true;
    return false;
  };
  const edits: { start: number; end: number; text: string }[] = [];
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !ts.isStringLiteral(st.moduleSpecifier) || !st.moduleSpecifier.text.startsWith(".")) continue;
    if (!tset.has(resolve(dirname(abs), st.moduleSpecifier.text))) continue;
    const c = st.importClause; if (!c) continue;
    const elems: { name: string; node: ts.Node }[] = [];
    if (c.name) elems.push({ name: c.name.text, node: c.name });
    if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) elems.push({ name: c.namedBindings.name.text, node: c.namedBindings }); else for (const e of c.namedBindings.elements) elems.push({ name: e.name.text, node: e }); }
    const dead = elems.filter((e) => !ast.has(e.name));
    if (dead.length === 0) continue;
    const textUsed = dead.filter((e) => outside(e.name));
    const removable = dead.filter((e) => !outside(e.name));
    docOnly += textUsed.length;
    if (removable.length === 0) continue;
    if (removable.length === elems.length) {
      let end = st.getEnd(); while (src[end] === " " || src[end] === "\t") end++; if (src[end] === "\r") end++; if (src[end] === "\n") end++;
      edits.push({ start: st.getStart(sf), end, text: "" }); removedStatements++; edgesRemoved++;
    } else {
      const keep = elems.filter((e) => !removable.includes(e));
      const defaultName = c.name && keep.find((e) => e.node === c.name)?.name;
      const named = c.namedBindings && ts.isNamedImports(c.namedBindings) ? c.namedBindings.elements.filter((e) => keep.some((k) => k.node === e)).map((e) => e.getText(sf)) : [];
      const ns = c.namedBindings && ts.isNamespaceImport(c.namedBindings) && keep.some((k) => k.node === c.namedBindings) ? c.namedBindings.getText(sf) : "";
      const parts = [defaultName, ns, named.length ? `{ ${named.join(", ")} }` : ""].filter(Boolean).join(", ");
      edits.push({ start: st.getStart(sf), end: st.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}${parts} from ${st.moduleSpecifier.getText(sf)};` });
    }
    removedNames += removable.length;
    log.push({ file, spec: st.moduleSpecifier.text, removed: removable.map((r) => r.name), whole: removable.length === elems.length });
  }
  if (edits.length) {
    filesTouched++;
    if (apply) { let s = src; for (const e of edits.sort((a, b) => b.start - a.start)) s = s.slice(0, e.start) + e.text + s.slice(e.end); writeFileSync(abs, s); }
  }
}
console.log(JSON.stringify({ mode: apply ? "apply" : "dry", filesTouched, removedStatements, removedNames, docOnlyKept: docOnly }));
writeFileSync(process.argv[4] ?? "/dev/null", JSON.stringify(log, null, 1));
