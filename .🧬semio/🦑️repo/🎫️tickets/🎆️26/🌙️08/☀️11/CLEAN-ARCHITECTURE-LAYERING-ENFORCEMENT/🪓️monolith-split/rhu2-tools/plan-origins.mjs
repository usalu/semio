import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const entries = plan.map(o => o.to).filter(t => /\.(ts|tsx)$/.test(t));
const program = ts.createProgram(entries.map(e => R + e), { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, resolveJsonModule: true, types: [], customConditions: ["semio-source"] });
const checker = program.getTypeChecker();
const OS = "🧰️framework/🛍️products/💻️os/";
for (const o of plan) {
  if (!/\.(ts|tsx)$/.test(o.to)) continue;
  const sf = program.getSourceFile(R + o.to); const sym = sf && checker.getSymbolAtLocation(sf); if (!sym) continue;
  const ex = new Map(checker.getExportsOfModule(sym).map(x => [x.name, x]));
  const by = new Map();
  for (const n of o.names) { let s = ex.get(n.name); if (!s) { by.set("?", [...(by.get("?") ?? []), n.name]); continue; } if (s.flags & ts.SymbolFlags.Alias) { try { s = checker.getAliasedSymbol(s); } catch {} } const d = s.declarations?.[0]; const f = d ? d.getSourceFile().fileName.replace(R, "").replace(OS, "OS/").replace("🧰️framework/", "F/") : "?"; by.set(f, [...(by.get(f) ?? []), (n.type ? "t:" : "") + n.name]); }
  console.log("## " + o.to.replace(OS, "OS/") + "  (" + o.files.length + " files)");
  for (const [f, ns] of by) console.log("   " + f + "\n      " + ns.join(" "));
}
