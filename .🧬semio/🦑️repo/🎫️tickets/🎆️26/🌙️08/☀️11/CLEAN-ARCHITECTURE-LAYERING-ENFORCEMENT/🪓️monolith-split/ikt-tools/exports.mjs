import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [file, origCopy, out] = process.argv.slice(2);
const opts = { jsx: ts.JsxEmit.ReactJSX, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], lib: [] };
const host = ts.createCompilerHost(opts);
if (origCopy !== "-") {
  const rf = host.readFile.bind(host), gs = host.getSourceFile.bind(host);
  host.readFile = (f) => (f === file ? fs.readFileSync(origCopy, "utf8") : rf(f));
  host.getSourceFile = (f, l, ...r) => (f === file ? ts.createSourceFile(f, fs.readFileSync(origCopy, "utf8"), l, true, ts.ScriptKind.TSX) : gs(f, l, ...r));
}
const program = ts.createProgram([file], opts, host);
const checker = program.getTypeChecker();
const sf = program.getSourceFile(file);
const sym = checker.getSymbolAtLocation(sf);
const rows = checker.getExportsOfModule(sym).map((s) => {
  let r = s;
  if (s.flags & ts.SymbolFlags.Alias) { try { r = checker.getAliasedSymbol(s); } catch {} }
  const isType = !(r.flags & ts.SymbolFlags.Value) && !!(r.flags & ts.SymbolFlags.Type);
  const unresolved = r.flags === ts.SymbolFlags.Alias || !r.declarations;
  return `${s.name}\t${unresolved ? "unresolved" : isType ? "type" : "value"}`;
}).sort();
fs.writeFileSync(out, rows.join("\n") + "\n");
console.log(rows.length, "exports");
