import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const [barrel, out, destPrefix] = process.argv.slice(2);
const program = ts.createProgram([R + barrel], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, types: [], customConditions: ["semio-source"] });
const checker = program.getTypeChecker();
const sf = program.getSourceFile(R + barrel); const sym = checker.getSymbolAtLocation(sf);
const names = {}; const stats = new Map();
for (const x of checker.getExportsOfModule(sym)) {
  let s = x; if (s.flags & ts.SymbolFlags.Alias) { try { s = checker.getAliasedSymbol(s); } catch {} }
  const d = s.declarations?.[0]; if (!d) continue;
  const f = d.getSourceFile().fileName.replace(R, "").normalize("NFC");
  stats.set(f, (stats.get(f) ?? 0) + 1);
  if (f.startsWith(destPrefix.normalize("NFC"))) names[x.name] = f;
}
fs.writeFileSync(out, JSON.stringify({ names }, null, 1));
console.log("exports", [...stats.values()].reduce((a, b) => a + b, 0), "mapped", Object.keys(names).length);
for (const [f, n] of [...stats].sort((a, b) => b[1] - a[1]).slice(0, 40)) console.log(String(n).padStart(4), f.replace("🧰️framework/🛍️products/💻️os/", "OS/").replace("🧰️framework/", "F/"));
