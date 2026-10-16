/** 🔬️ Semantic diagnostics of the root files of a tsconfig (syntax errors in the import closure do not suppress them, unlike `tsc -p`). Prints `file(line,col): error TSxxxx: message` for root files only. Usage: bun tsc-sem.ts <tsconfig.json> <out.txt> */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { writeFileSync } from "node:fs";
const [cfgPath, out] = process.argv.slice(2);
const cfg = ts.readConfigFile(cfgPath!, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, "/Users/ueli/Documents/semio");
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options });
const lines: string[] = [];
for (const sf of program.getSourceFiles()) {
  if (!parsed.fileNames.includes(sf.fileName)) continue;
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    const { line, character } = d.file ? d.file.getLineAndCharacterOfPosition(d.start ?? 0) : { line: 0, character: 0 };
    lines.push(`${sf.fileName.replace("/Users/ueli/Documents/semio/", "")}(${line + 1},${character + 1}): error TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText, " ")}`);
  }
}
writeFileSync(out!, lines.join("\n") + "\n");
console.log(`root files=${parsed.fileNames.length} diagnostics=${lines.length}`);
