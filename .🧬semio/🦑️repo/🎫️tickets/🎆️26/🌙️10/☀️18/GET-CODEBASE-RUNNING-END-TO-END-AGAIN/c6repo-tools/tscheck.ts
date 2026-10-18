import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, ts } from "./ast.ts";
const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const [listFile, outFile] = process.argv.slice(2);
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const options: import("typescript").CompilerOptions = {
  target: ts.ScriptTarget.ESNext,
  module: ts.ModuleKind.ESNext,
  moduleResolution: ts.ModuleResolutionKind.Bundler,
  allowImportingTsExtensions: true,
  noEmit: true,
  skipLibCheck: true,
  strict: true,
  typeRoots: [S + "/types"],
  types: ["node"],
  allowJs: false,
  resolveJsonModule: true,
  esModuleInterop: true,
  baseUrl: ROOT,
};
const program = ts.createProgram({ rootNames: files.map((f) => join(ROOT, f)), options });
const want = new Set(files.map((f) => join(ROOT, f)));
const lines: string[] = [];
for (const f of files) {
  const sf = program.getSourceFile(join(ROOT, f));
  if (!sf) { lines.push(`${f}\tMISSING\t0\tsource file not in program`); continue; }
  const diags = [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)];
  for (const d of diags) {
    const { line } = d.start !== undefined ? sf.getLineAndCharacterOfPosition(d.start) : { line: 0 };
    lines.push(`${f}\tTS${d.code}\t${line + 1}\t${ts.flattenDiagnosticMessageText(d.messageText, " ").slice(0, 220)}`);
  }
}
writeFileSync(outFile, lines.join("\n") + "\n");
console.log("files", files.length, "diagnostics", lines.length);
