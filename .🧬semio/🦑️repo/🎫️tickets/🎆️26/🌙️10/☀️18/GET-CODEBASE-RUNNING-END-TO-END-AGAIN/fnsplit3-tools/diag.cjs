// usage: node diag.cjs <file> : prints all diagnostics of one file with line numbers
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const REPO = "/Users/ueli/Documents/semio";
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: [] }, REPO);
const files = process.argv.slice(2).map((f) => (path.isAbsolute(f) ? f : path.join(REPO, f)));
const program = ts.createProgram(files, options);
for (const f of files) { const sf = program.getSourceFile(f); for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) console.log(path.basename(path.dirname(f)).slice(0,22), sf.getLineAndCharacterOfPosition(d.start).line + 1, d.code, ts.flattenDiagnosticMessageText(d.messageText, "\n").split("\n")[0].slice(0, 200)); }
