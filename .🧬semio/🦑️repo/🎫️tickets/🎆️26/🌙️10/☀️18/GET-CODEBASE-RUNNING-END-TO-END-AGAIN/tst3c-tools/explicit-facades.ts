import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio/";
const packageDir = root + "🧰️framework/📦️packages/🟦️typescript/";
const manifest = JSON.parse(readFileSync(packageDir + "package.json", "utf8")) as { exports: Record<string, string> };
const apply = process.argv[2] === "apply";
const targets = Object.entries(manifest.exports).filter(([k]) => k !== ".").map(([, t]) => join(packageDir, t)).filter(p => existsSync(p) && /^\s*export\s*\*/m.test(readFileSync(p, "utf8")));
const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, resolveJsonModule: true, isolatedModules: true, esModuleInterop: true, skipLibCheck: true, noEmit: true, types: ["bun"] };
const program = ts.createProgram(targets, options);
const checker = program.getTypeChecker();
for (const target of targets) {
  const source = program.getSourceFile(target)!;
  let text = source.text;
  const edits: { start: number; end: number; text: string }[] = [];
  for (const statement of source.statements) {
    if (!ts.isExportDeclaration(statement) || statement.exportClause || !statement.moduleSpecifier) continue;
    const moduleSymbol = checker.getSymbolAtLocation(statement.moduleSpecifier);
    if (!moduleSymbol) { console.log("UNRESOLVED", target, statement.moduleSpecifier.getText()); continue; }
    const values: string[] = [], types: string[] = [];
    for (const symbol of checker.getExportsOfModule(moduleSymbol)) {
      if (symbol.name === "default") { console.log("DEFAULT export skipped", target); continue; }
      const resolved = symbol.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(symbol) : symbol;
      ((resolved.flags & ts.SymbolFlags.Value) !== 0 ? values : types).push(symbol.name);
    }
    values.sort(); types.sort();
    const specifier = statement.moduleSpecifier.getText();
    const out = [values.length ? `export { ${values.join(", ")} } from ${specifier};` : "", types.length ? `export type { ${types.join(", ")} } from ${specifier};` : ""].filter(Boolean).join("\n");
    edits.push({ start: statement.getStart(), end: statement.getEnd(), text: out });
  }
  for (const e of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, e.start) + e.text + text.slice(e.end);
  console.log(apply ? "WRITE" : "DRY", target.slice(packageDir.length), edits.length, text.length);
  if (apply && edits.length) writeFileSync(target, text);
}
