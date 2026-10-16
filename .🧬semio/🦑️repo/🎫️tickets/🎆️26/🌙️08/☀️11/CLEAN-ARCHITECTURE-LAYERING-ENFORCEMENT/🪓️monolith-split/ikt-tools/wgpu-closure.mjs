import fs from "node:fs";
import path from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const R = "/Users/ueli/Documents/semio";
const [entry, newFilesFile] = process.argv.slice(2);
const mine = new Set(fs.readFileSync(newFilesFile, "utf8").split("\n").filter(Boolean));
mine.add(path.resolve(entry));
const seen = new Set();
const todo = [path.resolve(entry)];
while (todo.length) {
  const f = todo.pop();
  if (seen.has(f)) continue;
  seen.add(f);
  const emitted = ts.transpileModule(fs.readFileSync(f, "utf8"), { compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.Preserve } }).outputText;
  const sf = ts.createSourceFile(f, emitted, ts.ScriptTarget.Latest, true);
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || !st.moduleSpecifier.text.startsWith(".")) continue;
    const t = path.resolve(path.dirname(f), st.moduleSpecifier.text);
    if (mine.has(t)) todo.push(t);
  }
}
console.log([...seen].map((f) => path.relative(R, f)).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))).join("\n"));
