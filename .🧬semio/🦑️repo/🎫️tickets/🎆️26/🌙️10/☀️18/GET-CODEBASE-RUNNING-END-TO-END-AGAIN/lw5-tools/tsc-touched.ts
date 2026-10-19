import ts from "typescript";
import { readFileSync, existsSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((f) => R + f).filter((f) => existsSync(f));
const cfg = ts.getParsedCommandLineOfConfigFile(R + "tsconfig.json", {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const program = ts.createProgram({ rootNames: files, options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
let total = 0;
for (const f of files) {
  const sf = program.getSourceFile(f)!;
  const ds = [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)];
  for (const d of ds) { total++; const { line } = sf.getLineAndCharacterOfPosition(d.start ?? 0); console.log(f.slice(R.length).slice(-70) + ":" + (line + 1), "TS" + d.code, ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 200)); }
}
console.log("files", files.length, "diagnostics", total);
