import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
let entries = process.argv.slice(3);
if (entries[0] && entries[0].endsWith(".json")) entries = JSON.parse(fs.readFileSync(entries[0], "utf8")).map(x => x[1]);
const program = ts.createProgram(entries.map(e => R + e), { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, types: [], customConditions: ["semio-source"] });
const checker = program.getTypeChecker();
const out = {};
for (const e of entries) {
  const sf = program.getSourceFile(R + e);
  const sym = checker.getSymbolAtLocation(sf);
  out[e] = sym ? checker.getExportsOfModule(sym).map(s => s.name) : [];
}
fs.writeFileSync(process.argv[2], JSON.stringify(out));
console.log(Object.entries(out).map(([k, v]) => k + ": " + v.length).join("\n"));
