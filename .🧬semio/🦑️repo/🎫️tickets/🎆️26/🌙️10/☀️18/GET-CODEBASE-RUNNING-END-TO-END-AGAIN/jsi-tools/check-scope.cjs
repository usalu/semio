// 🔬 usage: node check-scope.cjs <stage-dir>: TypeScript semantic diagnostics for undefined names and unused imports across the staged JS files (imports stay unresolved).
const ts = require("/Users/ueli/.cache/jsi/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const dir = path.resolve(process.argv[2]);
const files = [];
(function walk(d) { for (const e of fs.readdirSync(d, { withFileTypes: true })) { const p = path.join(d, e.name); if (e.isDirectory()) walk(p); else if (p.endsWith(".mjs")) files.push(p); } })(dir);
const program = ts.createProgram(files, { allowJs: true, checkJs: true, noEmit: true, noResolve: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, noUnusedLocals: true, noImplicitAny: false, strict: false, types: [] });
const keep = new Set([2304, 2552, 6133, 6192, 6196, 2451, 2448, 2454]);
let n = 0;
for (const f of files) {
  const sf = program.getSourceFile(f);
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    if (!keep.has(d.code) && !(d.code >= 1000 && d.code < 2000)) continue;
    const { line, character } = sf.getLineAndCharacterOfPosition(d.start);
    const msg = ts.flattenDiagnosticMessageText(d.messageText, "\n");
    if (/Cannot find global type|Cannot find name '(Promise|Map|Set|Object|Array|JSON|Buffer|process|console|URL|Error|Symbol|String|Number|Boolean|Date|RegExp|Math|WeakMap|WeakSet|Function|performance|AbortSignal|BigInt|Uint8Array|Reflect|globalThis|structuredClone)'/.test(msg)) continue;
    console.log(`${path.relative(dir, f)}:${line + 1}:${character + 1} TS${d.code} ${msg}`);
    n++;
  }
}
console.log(n ? `PROBLEMS ${n}` : "SCOPE OK");
