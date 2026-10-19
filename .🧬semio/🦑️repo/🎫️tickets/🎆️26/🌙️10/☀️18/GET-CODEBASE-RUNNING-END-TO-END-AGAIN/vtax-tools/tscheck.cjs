// usage: node tscheck.cjs old|new  -> prints diagnostics (file-relative) of the authority module family
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const mode = process.argv[2];
const DISCOVERY = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/";
const FAKE = DISCOVERY + "🔐️taxonomy-authority/🟦️.ts";
const files = mode === "old" ? [FAKE] : [FAKE, ...fs.readdirSync(DISCOVERY).filter((n) => /audit/.test(n)).map((n) => DISCOVERY + n + "/🟦️.ts")];
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, lib: ["lib.esnext.d.ts", "lib.dom.d.ts"], moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, esModuleInterop: true, isolatedModules: true, resolveJsonModule: true, allowImportingTsExtensions: true, skipLibCheck: true, noEmit: true, types: ["node"], typeRoots: ["/Users/ueli/Documents/semio/node_modules/@types"] };
const host = ts.createCompilerHost(opts);
if (mode === "old") {
  const orig = host.getSourceFile.bind(host), origRead = host.readFile.bind(host);
  const text = fs.readFileSync(path.join(__dirname, "orig/taxonomy-authority.ts"), "utf8");
  host.getSourceFile = (f, lv, ...r) => (f === FAKE ? ts.createSourceFile(f, text, lv, true, ts.ScriptKind.TS) : orig(f, lv, ...r));
  host.readFile = (f) => (f === FAKE ? text : origRead(f));
}
const program = ts.createProgram(files, opts, host);
console.error("program files", program.getSourceFiles().length);
const out = [];
for (const f of files) {
  const sf = program.getSourceFile(f);
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    out.push(`${path.basename(path.dirname(f))}:${sf.getLineAndCharacterOfPosition(d.start ?? 0).line + 1} TS${d.code} ${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 220)}`);
  }
}
console.log(out.join("\n"));
console.error(`${mode}: ${out.length} diagnostics across ${files.length} files`);
