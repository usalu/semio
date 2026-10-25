// Usage: node tscheck.mjs <root-file>... : type-checks the closure of the given files with the root tsconfig, prints diagnostics located in the roots.
import { createRequire } from "node:module";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
const ts = createRequire(root + "/package.json")("typescript");
const absolute = resolve(root, "tsconfig.json");
const read = ts.readConfigFile(absolute, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(read.config, ts.sys, dirname(absolute), { noEmit: true, incremental: false, tsBuildInfoFile: undefined }, absolute);
const roots = process.argv.slice(2).map((file) => resolve(root, file));
const start = Date.now();
const program = ts.createProgram({ rootNames: roots, options: parsed.options });
let count = 0;
for (const file of roots) {
  const source = program.getSourceFile(file);
  if (!source) { console.log("MISSING root " + file); continue; }
  for (const d of [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)]) {
    count++;
    const { line, character } = source.getLineAndCharacterOfPosition(d.start);
    console.log(`${file.slice(root.length + 1).split("/").slice(-4).join("/")}(${line + 1},${character + 1}): error TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText, "\n").split("\n").slice(0, 3).join(" | ")}`);
  }
}
console.log(`Files: ${program.getSourceFiles().length} Errors in roots: ${count} in ${Math.round((Date.now() - start) / 1000)}s`);
