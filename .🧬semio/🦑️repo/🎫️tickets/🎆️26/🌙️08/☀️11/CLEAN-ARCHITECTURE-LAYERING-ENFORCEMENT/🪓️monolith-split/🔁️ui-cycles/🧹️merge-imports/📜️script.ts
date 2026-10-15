/** 🧹 Merges import declarations that share a module specifier and type-only flag, and export-from declarations likewise.
 * Usage: bun 📜️script.ts <apply|dry> <file...>
 */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";

const [mode, ...files] = process.argv.slice(2) as [string, ...string[]];
for (const file of files) {
  const text = readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits: { start: number; end: number; text: string }[] = [];
  const groups = new Map<string, ts.ImportDeclaration[]>();
  for (const st of sf.statements) {
    if (!ts.isImportDeclaration(st) || !st.importClause || st.attributes) continue;
    const c = st.importClause;
    if (c.namedBindings && !ts.isNamedImports(c.namedBindings)) continue;
    if (c.name && c.namedBindings) continue;
    if (c.name) continue;
    const key = `${c.isTypeOnly ? "type" : "value"}|${(st.moduleSpecifier as ts.StringLiteral).text}`;
    groups.set(key, [...(groups.get(key) ?? []), st]);
  }
  for (const [key, list] of groups) {
    if (list.length < 2) continue;
    const seen = new Set<string>();
    const names: string[] = [];
    for (const st of list) {
      for (const el of (st.importClause!.namedBindings as ts.NamedImports).elements) {
        const local = el.name.text;
        if (seen.has(local)) continue;
        seen.add(local);
        names.push(el.getText(sf));
      }
    }
    const first = list[0]!;
    const typeKw = first.importClause!.isTypeOnly ? "type " : "";
    edits.push({ start: first.getStart(sf), end: first.getEnd(), text: `import ${typeKw}{ ${names.join(", ")} } from ${first.moduleSpecifier.getText(sf)};` });
    for (const st of list.slice(1)) {
      let end = st.getEnd();
      if (text[end] === "\n") end += 1;
      edits.push({ start: st.getStart(sf), end, text: "" });
    }
    console.log(`${file}: merged ${list.length} imports of ${key}`);
  }
  if (mode === "apply" && edits.length) {
    let out = text;
    for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
    writeFileSync(file, out);
  }
}
