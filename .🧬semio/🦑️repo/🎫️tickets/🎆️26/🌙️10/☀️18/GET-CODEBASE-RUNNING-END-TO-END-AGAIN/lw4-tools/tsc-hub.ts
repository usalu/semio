import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter((f) => /\.(ts|tsx)$/u.test(f)).map((f) => ROOT + f);
const cfg = ts.getParsedCommandLineOfConfigFile(ROOT + "tsconfig.json", {}, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: (d) => { throw new Error(String(d.messageText)); } })!;
const program = ts.createProgram({ rootNames: files, options: { ...cfg.options, noEmit: true, incremental: false, tsBuildInfoFile: undefined } });
const rows: string[] = [];
for (const sf of program.getSourceFiles()) {
  if (!sf.fileName.startsWith(ROOT + "🌎️hub/") || sf.fileName.includes("/node_modules/")) continue;
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) rows.push(`${sf.fileName.slice(ROOT.length)}\t${d.code}\t${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 200)}`);
}
rows.sort();
writeFileSync(process.argv[3]!, rows.join("\n") + "\n");
const codes: Record<string, number> = {};
for (const r of rows) { const c = r.split("\t")[1]!; codes[c] = (codes[c] ?? 0) + 1; }
console.log("diagnostics", rows.length, codes);
console.log("program files", program.getSourceFiles().length, "hub files", program.getSourceFiles().filter((s) => s.fileName.startsWith(ROOT + "🌎️hub/")).length);
