import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const entry = process.argv[3];
const program = ts.createProgram([R + entry], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, types: [], customConditions: ["semio-source"] });
const checker = program.getTypeChecker();
const sf = program.getSourceFile(R + entry);
const sym = checker.getSymbolAtLocation(sf);
const out = {};
for (const e of checker.getExportsOfModule(sym)) {
  let s = e; if (s.flags & ts.SymbolFlags.Alias) s = checker.getAliasedSymbol(s);
  const d = s.declarations?.[0]; out[e.name] = d ? d.getSourceFile().fileName.replace(R, "") : "?";
}
fs.writeFileSync(process.argv[2], JSON.stringify(out));
console.log(Object.keys(out).length);
