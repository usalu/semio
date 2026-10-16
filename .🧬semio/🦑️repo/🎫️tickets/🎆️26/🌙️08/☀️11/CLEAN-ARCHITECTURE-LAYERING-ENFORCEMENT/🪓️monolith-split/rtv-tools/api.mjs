import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { dirname } from "node:path";
const [tsconfigPath, moduleFile] = process.argv.slice(2);
const cfg = ts.readConfigFile(tsconfigPath, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, dirname(tsconfigPath));
const program = ts.createProgram([moduleFile], { ...parsed.options, noEmit: true });
const checker = program.getTypeChecker();
const sf = program.getSourceFile(moduleFile);
if (!sf) throw new Error("no source file " + moduleFile);
const sym = checker.getSymbolAtLocation(sf);
const rows = [];
for (const e of checker.getExportsOfModule(sym)) {
  let s = e; if (s.flags & ts.SymbolFlags.Alias) s = checker.getAliasedSymbol(s);
  const f = s.flags; const kind = [(f & ts.SymbolFlags.Value) ? "value" : "", (f & ts.SymbolFlags.Type) ? "type" : ""].filter(Boolean).join("+");
  rows.push(`${e.name}\t${kind}\t${checker.typeToString(checker.getTypeOfSymbolAtLocation(s, sf.statements[0] ?? sf)).slice(0, 4000)}`);
}
rows.sort(); console.log(rows.join("\n"));
