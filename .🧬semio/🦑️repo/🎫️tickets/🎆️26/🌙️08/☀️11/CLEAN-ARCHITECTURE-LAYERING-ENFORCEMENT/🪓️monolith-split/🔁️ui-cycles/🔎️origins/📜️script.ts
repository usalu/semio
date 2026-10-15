/** 🔎 For every import of a barrel module, resolve where each imported symbol is truly declared.
 * Usage: bun 📜️script.ts <tsconfig> <barrelFile> <importerPrefix>
 */
import ts from "typescript";
import { dirname, resolve } from "node:path";

const [tsconfigPath, barrel, importerPrefix] = process.argv.slice(2) as [string, string, string];
const cfg = ts.readConfigFile(tsconfigPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, dirname(resolve(tsconfigPath)));
const program = ts.createProgram(parsed.fileNames, parsed.options);
const checker = program.getTypeChecker();
const barrelAbs = resolve(barrel);
const repo = process.cwd();
const rel = (f: string) => f.replace(repo + "/", "");
for (const sf of program.getSourceFiles()) {
  if (!sf.fileName.includes(importerPrefix) || sf.fileName === barrelAbs) continue;
  ts.forEachChild(sf, (node) => {
    if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier)) return;
    const target = ts.resolveModuleName(node.moduleSpecifier.text, sf.fileName, parsed.options, ts.sys).resolvedModule?.resolvedFileName;
    if (target !== barrelAbs) return;
    const clause = node.importClause;
    if (!clause) return;
    const named = clause.namedBindings;
    if (!named || !ts.isNamedImports(named)) { console.log(`${rel(sf.fileName)}\t<non-named import>`); return; }
    for (const el of named.elements) {
      let sym = checker.getSymbolAtLocation(el.name);
      if (!sym) continue;
      if (sym.flags & ts.SymbolFlags.Alias) sym = checker.getAliasedSymbol(sym);
      const decl = sym.declarations?.[0];
      const origin = decl ? rel(decl.getSourceFile().fileName) : "?";
      console.log(`${rel(sf.fileName)}\t${(el.propertyName ?? el.name).text}\t${el.isTypeOnly || clause.isTypeOnly ? "type" : "value"}\t${origin}`);
    }
  });
}
