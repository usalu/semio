/** 🧪️ Type-checks the staged worker modules against the actual workspace module resolver. */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../..");
const overlay = JSON.parse(readFileSync(resolve(import.meta.dir, "🗑️generated/raw-hub-host-overlay.json"), "utf8"));
const sources = new Map<string, string>(Object.entries(overlay.candidate).filter(([path]) => path.endsWith(".ts")).map(([path, source]) => [resolve(root, path), String(source)]));
assert.equal(existsSync(resolve(root, "tsconfig.json")), true);
const settings = ts.readConfigFile(resolve(root, "tsconfig.json"), ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(settings.config, ts.sys, root);
const options = { ...parsed.options, incremental: false, noEmit: true };
const host = ts.createCompilerHost(options);
const staged = new Set([...sources.keys()].map(dirname));
const read = host.readFile.bind(host);
const exists = host.fileExists.bind(host);
const dir = host.directoryExists?.bind(host);
host.readFile = file => sources.get(resolve(file)) ?? read(file);
host.fileExists = file => sources.has(resolve(file)) || exists(file);
host.directoryExists = path => staged.has(resolve(path)) || dir?.(path) === true;
const program = ts.createProgram([...sources.keys()], options, host);
const diagnostics = ts.getPreEmitDiagnostics(program);
const relevant = diagnostics.filter(item => item.file !== undefined && sources.has(resolve(item.file.fileName)));
for (const item of relevant) {
  const file = item.file!;
  const line = item.start === undefined ? 0 : file.getLineAndCharacterOfPosition(item.start).line + 1;
  console.error(`[DEBUG] ${file.fileName}:${line} TS${item.code} ${ts.flattenDiagnosticMessageText(item.messageText, "\\n")}`);
}
if (relevant.length !== 0) process.exitCode = 1;
else console.log(`[DEBUG] Checked ${sources.size} staged Store/Hub TypeScript modules; ${diagnostics.length} diagnostics outside staged modules`);
