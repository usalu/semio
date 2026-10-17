import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const [outFile, listFile] = process.argv.slice(2);
const entries = JSON.parse(fs.readFileSync(listFile, "utf8"));
const program = ts.createProgram(entries.map(e => R + e), { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, types: [], customConditions: ["semio-source"] });
const checker = program.getTypeChecker();
const out = {};
for (const e of entries) {
  const sf = program.getSourceFile(R + e);
  const sym = sf && checker.getSymbolAtLocation(sf);
  if (!sym) { out[e] = {}; continue; }
  const o = {};
  for (const x of checker.getExportsOfModule(sym)) {
    let s = x; if (s.flags & ts.SymbolFlags.Alias) { try { s = checker.getAliasedSymbol(s); } catch {} }
    const d = s.declarations?.[0]; o[x.name] = d ? d.getSourceFile().fileName.replace(R, "") + "#" + (d.pos) : "?";
  }
  out[e] = o;
}
fs.writeFileSync(outFile, JSON.stringify(out));
console.log(entries.length, "entries");
