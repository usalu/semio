// Usage: node tscheck.mjs <tsconfig> <overlay.json|-> [--tag name]
// Type-checks a tsconfig program with staged file contents served in place of the originals (single copy per file).
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
const ts = createRequire(root + "/package.json")("typescript");
const [config, overlayPath] = process.argv.slice(2);
const overlay = overlayPath && overlayPath !== "-" ? JSON.parse(readFileSync(overlayPath, "utf8")) : {};
const absolute = resolve(config);
const read = ts.readConfigFile(absolute, ts.sys.readFile);
if (read.error) { console.log(ts.flattenDiagnosticMessageText(read.error.messageText, "\n")); process.exit(2); }
const parsed = ts.parseJsonConfigFileContent(read.config, ts.sys, dirname(absolute), { noEmit: true, incremental: false, tsBuildInfoFile: undefined }, absolute);
const host = ts.createCompilerHost(parsed.options, true);
const originalRead = host.readFile.bind(host), originalGet = host.getSourceFile.bind(host);
host.readFile = (name) => (name in overlay ? overlay[name] : originalRead(name));
host.getSourceFile = (name, languageVersion, onError, shouldCreate) => name in overlay ? ts.createSourceFile(name, overlay[name], languageVersion, true) : originalGet(name, languageVersion, onError, shouldCreate);
const start = Date.now();
const program = ts.createProgram({ rootNames: parsed.fileNames, options: parsed.options, projectReferences: parsed.projectReferences, host });
const diagnostics = ts.getPreEmitDiagnostics(program);
for (const d of diagnostics) {
  const where = d.file ? (() => { const { line, character } = d.file.getLineAndCharacterOfPosition(d.start); return `${d.file.fileName}(${line + 1},${character + 1}): `; })() : "";
  console.log(`${where}error TS${d.code}: ${ts.flattenDiagnosticMessageText(d.messageText, "\n")}`);
}
console.log(`Files: ${program.getSourceFiles().length}`);
console.log(`Errors: ${diagnostics.length} in ${Math.round((Date.now() - start) / 1000)}s`);
