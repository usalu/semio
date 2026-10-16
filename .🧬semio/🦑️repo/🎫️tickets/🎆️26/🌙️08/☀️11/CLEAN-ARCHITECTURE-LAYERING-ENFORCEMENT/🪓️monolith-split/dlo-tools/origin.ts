/** 🧭️ Resolves where each imported name of a file is actually declared (follows re-export aliases). Read-only. Usage: bun origin.ts <repo-relative file> */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const file = process.argv[2];
const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"], jsx: ts.JsxEmit.ReactJSX, skipLibCheck: true, noEmit: true, types: [], esModuleInterop: true };
const program = ts.createProgram([join(ROOT, file)], options);
const checker = program.getTypeChecker();
const sf = program.getSourceFile(join(ROOT, file))!;
for (const st of sf.statements) {
  if (!ts.isImportDeclaration(st) || !st.importClause) continue;
  const spec = (st.moduleSpecifier as ts.StringLiteral).text;
  const nb = st.importClause.namedBindings;
  if (!nb || !ts.isNamedImports(nb)) continue;
  for (const el of nb.elements) {
    const sym = checker.getSymbolAtLocation(el.name);
    const target = sym && (sym.flags & ts.SymbolFlags.Alias) ? checker.getAliasedSymbol(sym) : sym;
    const decl = target?.declarations?.[0];
    console.log(`${spec}\t${(el.propertyName ?? el.name).text}\t${decl ? relative(ROOT, decl.getSourceFile().fileName) : "?"}`);
  }
}
