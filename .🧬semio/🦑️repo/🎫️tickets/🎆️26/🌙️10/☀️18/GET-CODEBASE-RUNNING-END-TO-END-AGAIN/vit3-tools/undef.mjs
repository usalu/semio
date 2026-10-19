import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const dir = process.argv[2];
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const files = [];
const walk = (d) => { for (const e of readdirSync(d)) { if (e === "node_modules" || e === "🧪️tests") continue; const p = join(d, e); statSync(p).isDirectory() ? walk(p) : /\.tsx?$/.test(e) && files.push(p); } };
walk(dir);
const program = ts.createProgram(files, { noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ES2022, jsx: ts.JsxEmit.ReactJSX, strict: false, types: [], noResolve: false, resolveJsonModule: true });
const codes = new Set([2304, 2552, 2305, 2724, 2614]);
let n = 0;
for (const f of files) { const sf = program.getSourceFile(f); if (!sf) continue; for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) { if (!codes.has(d.code)) continue; n++; const { line } = sf.getLineAndCharacterOfPosition(d.start); console.log(f.replace(dir, "."), line + 1, ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 160)); } }
console.log("undefined-name diagnostics", n);
