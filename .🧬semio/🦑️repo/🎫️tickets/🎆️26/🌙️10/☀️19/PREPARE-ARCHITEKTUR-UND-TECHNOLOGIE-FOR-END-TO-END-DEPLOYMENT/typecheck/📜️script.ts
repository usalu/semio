import ts from "typescript";
import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const workspace = process.cwd();
const expires = Date.now() + 180000;
const controller = new AbortController();
process.once("SIGINT", () => controller.abort(Error("Diagnostic cancelled")));
const check = () => { controller.signal.throwIfAborted(); if (Date.now() >= expires) throw Error("Diagnostic deadline exceeded"); };
const { workspaceTestAliasesV1 } = await import(pathToFileURL(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts")).href);
const aliases = await workspaceTestAliasesV1(workspace, { owner: "🎓️teaching", signal: controller.signal, advance: (progress: { completedOperations: number; stage: string }) => { check(); if (progress.completedOperations % 20000 === 0) console.log(`[DEBUG] TypeScript source ownership ${progress.stage}: ${progress.completedOperations}`); } });
const configPath = resolve(workspace, "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json");
const config = ts.readConfigFile(configPath, ts.sys.readFile);
if (config.error) throw Error(ts.flattenDiagnosticMessageText(config.error.messageText, "\n"));
const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, resolve(configPath, ".."));
if (parsed.errors.length) throw Error(ts.formatDiagnosticsWithColorAndContext(parsed.errors, { getCanonicalFileName: value => value, getCurrentDirectory: () => workspace, getNewLine: () => "\n" }));
const paths: Record<string, string[]> = {};
for (const alias of aliases) {
  const specifier = alias.find.source.slice(1, -1).replace(/\\(.)/gu, "$1");
  const declaration = alias.replacement.replace(/\.mjs$/u, ".d.mts");
  paths[specifier] = [declaration !== alias.replacement && existsSync(declaration) ? declaration : alias.replacement];
}
const options = { ...parsed.options, paths, noEmit: true, incremental: false };
const host = ts.createCompilerHost(options);
const read = host.readFile;
let reads = 0;
host.readFile = file => { check(); reads++; if (reads % 500 === 0) console.log(`[DEBUG] TypeScript source reads: ${reads}`); return read(file); };
const program = ts.createProgram(parsed.fileNames, options, host);
const token = { isCancellationRequested: () => controller.signal.aborted || Date.now() >= expires, throwIfCancellationRequested: check };
const diagnostics = ts.getPreEmitDiagnostics(program, undefined, token);
for (const item of diagnostics) {
  const position = item.file && item.start !== undefined ? item.file.getLineAndCharacterOfPosition(item.start) : undefined;
  console.log(JSON.stringify({ file: item.file?.fileName, line: position ? position.line + 1 : undefined, code: item.code, message: ts.flattenDiagnosticMessageText(item.messageText, "\n") }));
}
console.log(`[DEBUG] Current source TypeScript diagnostic: ${diagnostics.length}; ${reads} reads; existing public exports and declarations only`);
process.exitCode = diagnostics.length ? 1 : 0;
