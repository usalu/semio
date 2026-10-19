import ts from "typescript";
import { resolve } from "node:path";
import { ROOT, rel } from "./lib.ts";
const files = process.argv.slice(2).map(f => resolve(ROOT, f));
const raw = ts.readConfigFile(resolve(ROOT, "tsconfig.json"), ts.sys.readFile);
const options = { ...ts.parseJsonConfigFileContent(raw.config, ts.sys, ROOT).options, noEmit: true, incremental: false, skipLibCheck: true };
const program = ts.createProgram({ rootNames: files, options });
let bad = 0;
for (const f of files) {
  const sf = program.getSourceFile(f);
  if (!sf) { console.log("missing", f); bad++; continue; }
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    bad++;
    const { line } = sf.getLineAndCharacterOfPosition(d.start ?? 0);
    console.log(`${rel(f).split("/").slice(-3).join("/")}:${line + 1} TS${d.code} ${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 200)}`);
  }
}
console.log(`files ${files.length}, program ${program.getSourceFiles().length}, diagnostics ${bad}`);
