// Usage: node tscheck-files.mjs <tsconfig> <staged-dir> <root-file>...
// Type-checks only the closure of the given root files; every file of <staged-dir>/index.json is served from its staged copy (single copy per file). Prints diagnostics located in the root files.
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
const ts = createRequire(root + "/package.json")("typescript");
const [config, stagedDir, ...rootFiles] = process.argv.slice(2);
const index = JSON.parse(readFileSync(join(stagedDir, "index.json"), "utf8"));
const overlay = Object.fromEntries(Object.entries(index).map(([flat, relative]) => [resolve(root, relative), readFileSync(join(stagedDir, flat), "utf8")]));
const absolute = resolve(config);
const read = ts.readConfigFile(absolute, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(read.config, ts.sys, dirname(absolute), { noEmit: true, incremental: false, tsBuildInfoFile: undefined }, absolute);
const host = ts.createCompilerHost(parsed.options, true);
const originalRead = host.readFile.bind(host), originalGet = host.getSourceFile.bind(host);
host.readFile = (name) => (name in overlay ? overlay[name] : originalRead(name));
host.getSourceFile = (name, languageVersion, onError, shouldCreate) => name in overlay ? ts.createSourceFile(name, overlay[name], languageVersion, true) : originalGet(name, languageVersion, onError, shouldCreate);
const roots = rootFiles.map((file) => resolve(root, file));
const start = Date.now();
const program = ts.createProgram({ rootNames: roots, options: parsed.options, host });
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
