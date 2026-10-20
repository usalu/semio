// usage: node diag.cjs <file>... : full semantic diagnostics (types: [] like tsdiff) of the files with line numbers; TS_TYPES=node adds node types
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const REPO = "/Users/ueli/Documents/semio";
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: process.env.TS_TYPES ? process.env.TS_TYPES.split(",") : [] }, REPO);
const names = process.argv.slice(2).map((p) => path.resolve(p));
const program = ts.createProgram(names, options);
for (const n of names) { const sf = program.getSourceFile(n); for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) console.log(path.basename(path.dirname(n)) + ":" + (d.start !== undefined ? sf.getLineAndCharacterOfPosition(d.start).line + 1 : 0) + " " + d.code + " " + ts.flattenDiagnosticMessageText(d.messageText, "\n").replace(/\n/g, " | ").slice(0, 600)); }
