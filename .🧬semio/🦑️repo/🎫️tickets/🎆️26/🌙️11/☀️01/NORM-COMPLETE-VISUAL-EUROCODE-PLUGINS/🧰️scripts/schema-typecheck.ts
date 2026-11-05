import ts from "typescript";
import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repo = process.cwd();
const plugin = join(repo, "✏️s/🔌️plugins/📕️norm");
const artifacts = join(plugin, "🗿️artifacts");
const roots = readdirSync(artifacts, { withFileTypes: true }).filter(row => row.isDirectory()).flatMap(row => ["", "📸️snapshot", "🔺️diff", "🧬️mutations"].map(facet => join(artifacts, row.name, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema", facet, "🟦️.ts"))).filter(existsSync);
const options: ts.CompilerOptions = { noEmit: true, strict: true, skipLibCheck: true, resolveJsonModule: true, esModuleInterop: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ESNext, types: [] };
const program = ts.createProgram(roots, options);
const diagnostics = ts.getPreEmitDiagnostics(program).map(row => ({ file: row.file?.fileName.replace(repo + "/", ""), line: row.file && row.start !== undefined ? row.file.getLineAndCharacterOfPosition(row.start).line + 1 : undefined, code: row.code, message: ts.flattenDiagnosticMessageText(row.messageText, "\n") }));
const output = resolve(import.meta.dir, "../🗑️generated/schema-typecheck.json");
mkdirSync(resolve(output, ".."), { recursive: true });
writeFileSync(output, JSON.stringify({ roots: roots.length, files: program.getSourceFiles().length, diagnostics }, null, 2) + "\n");
console.log(JSON.stringify({ roots: roots.length, files: program.getSourceFiles().length, diagnostics: diagnostics.length, first: diagnostics.slice(0, 80), output }, null, 2));
process.exitCode = diagnostics.length ? 1 : 0;
