/** 🔬️ Semantic diagnostics of the root files of a tsconfig with pre-edit texts served from snapshot JSONs, to reproduce the "before" state of import-line-only edits. Usage: bun tsc-sem-before.ts <tsconfig.json> <out.txt> <snapshot.json>... */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const [cfgPath, out, ...snaps] = process.argv.slice(2);
const before = new Map<string, string>();
for (const s of snaps) for (const [f, text] of Object.entries(JSON.parse(readFileSync(s, "utf8")) as Record<string, string>)) before.set((ROOT + f).normalize("NFC"), text);
const cfg = ts.readConfigFile(cfgPath!, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, ROOT);
const host = ts.createCompilerHost(parsed.options);
const getSourceFile = host.getSourceFile.bind(host);
host.getSourceFile = (name, lang, onError, shouldCreate) => { const text = before.get(name.normalize("NFC")); return text !== undefined ? ts.createSourceFile(name, text, lang, true) : getSourceFile(name, lang, onError, shouldCreate); };
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options, host });
const lines: string[] = [];
for (const sf of program.getSourceFiles()) {
  if (!parsed.fileNames.includes(sf.fileName)) continue;
  for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) {
    const { line, character } = d.file ? d.file.getLineAndCharacterOfPosition(d.start ?? 0) : { line: 0, character: 0 };
    lines.push(`${sf.fileName.replace(ROOT, "")}(${line + 1},${character + 1}): error TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText, " ")}`);
  }
}
writeFileSync(out!, lines.join("\n") + "\n");
console.log(`root files=${parsed.fileNames.length} diagnostics=${lines.length}`);
