/** 🔗 Lists, for every relative import edge between files under a prefix, the imported symbols and where they are truly declared.
 * Usage: bun 📜️script.ts <tsconfig> <prefix> [edges.txt]  (edges.txt lines: "from -> to"; omit for all edges)
 */
import ts from "typescript";
import { dirname, resolve } from "node:path";
import { readFileSync, existsSync } from "node:fs";

const [tsconfigPath, prefix, edgesFile] = process.argv.slice(2) as [string, string, string | undefined];
const cfg = ts.readConfigFile(tsconfigPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, dirname(resolve(tsconfigPath)));
const program = ts.createProgram(parsed.fileNames, parsed.options);
const checker = program.getTypeChecker();
const repo = process.cwd();
const rel = (f: string) => f.replace(repo + "/", "");
const wanted = edgesFile && existsSync(edgesFile)
  ? new Set(readFileSync(edgesFile, "utf8").split("\n").map((l) => l.trim()).filter(Boolean).map((l) => l.replace(/^\s+/, "")))
  : undefined;
for (const sf of program.getSourceFiles()) {
  const from = rel(sf.fileName);
  if (!from.startsWith(prefix)) continue;
  const visit = (node: ts.Node) => {
    let spec: string | undefined;
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) spec = node.moduleSpecifier.text;
    else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) spec = node.moduleSpecifier.text;
    if (spec === undefined) return;
    const target = ts.resolveModuleName(spec, sf.fileName, parsed.options, ts.sys).resolvedModule?.resolvedFileName;
    if (!target) return;
    const to = rel(target);
    if (wanted && !wanted.has(`${from} -> ${to}`)) return;
    const names: string[] = [];
    if (ts.isImportDeclaration(node)) {
      const c = node.importClause;
      if (!c) names.push("<side-effect>");
      else {
        if (c.name) names.push("default");
        if (c.namedBindings) {
          if (ts.isNamespaceImport(c.namedBindings)) names.push("* as " + c.namedBindings.name.text);
          else for (const el of c.namedBindings.elements) {
            let sym = checker.getSymbolAtLocation(el.name);
            let origin = "?";
            if (sym) {
              if (sym.flags & ts.SymbolFlags.Alias) sym = checker.getAliasedSymbol(sym);
              const d = sym.declarations?.[0];
              if (d) origin = rel(d.getSourceFile().fileName);
            }
            names.push(`${el.isTypeOnly || c.isTypeOnly ? "type " : ""}${(el.propertyName ?? el.name).text}@${origin === to ? "." : origin}`);
          }
        }
      }
    } else {
      names.push("<export-from>");
    }
    console.log(`${from} -> ${to} :: ${names.join(", ")}`);
  };
  ts.forEachChild(sf, visit);
}
