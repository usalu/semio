import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";

const repoRoot = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧫️fixtures/🧱️command-source/🔣️.json"), "utf8"));
const paths: string[] = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
const program = ts.createProgram(paths, { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, allowImportingTsExtensions: true, allowJs: true, skipLibCheck: true, noEmit: true, types: ["node"] });
for (const path of paths) for (const d of [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))]) {
  const where = d.file ? d.file.getLineAndCharacterOfPosition(d.start!) : undefined;
  console.log("[DEBUG]", d.file?.fileName, where ? `${where.line + 1}:${where.character + 1}` : "", ts.flattenDiagnosticMessageText(d.messageText, "\n"));
}
