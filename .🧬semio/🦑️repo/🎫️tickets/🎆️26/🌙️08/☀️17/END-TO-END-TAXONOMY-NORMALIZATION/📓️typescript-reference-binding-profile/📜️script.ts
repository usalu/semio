import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import ts from "typescript";

const repoRoot = process.cwd();
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const normalizerPath = `${library}/🧹️normalization/🟦️.ts`;
const hash = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const read = (path: string): Buffer => {
  if (path.startsWith("/") || path.includes("\\") || path.split("/").some((segment) => ["", ".", "..", "compose"].includes(segment))) throw new Error("Unsafe diagnostic input");
  const absolute = join(repoRoot, path);
  for (let parent = dirname(absolute); parent !== dirname(repoRoot); parent = dirname(parent)) {
    const state = lstatSync(parent);
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error("Unsafe diagnostic ancestor");
  }
  const state = lstatSync(absolute);
  if (!state.isFile() || state.isSymbolicLink()) throw new Error("Unsafe diagnostic leaf");
  return readFileSync(absolute);
};
const source = read(normalizerPath).toString();
const ast = ts.createSourceFile(normalizerPath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const owner = ast.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "typescriptTokens");
if (!owner || !ts.isFunctionDeclaration(owner) || !owner.body) throw new Error("Missing actual TypeScript token parser");
const declaration = owner.body.statements.find(ts.isVariableStatement)?.declarationList.declarations[0]?.initializer;
if (!declaration || !ts.isCallExpression(declaration) || declaration.expression.getText(ast) !== "regexTokens") throw new Error("Unexpected actual token parser structure");
const patterns = declaration.arguments[3];
if (!patterns || !ts.isArrayLiteralExpression(patterns) || patterns.elements.length !== 4) throw new Error("Unexpected actual token pattern set");
const original = new Function(`return (${patterns.elements[2]!.getText(ast)});`)() as RegExp;
if (!original.source.startsWith("\\b") || original.flags !== "giu") throw new Error("Unexpected actual binding pattern");
const projected = new RegExp("\\b(?=[^\\s=:\"']+\\s*(?:=|:)\\s*[\"'])" + original.source.slice(2), original.flags);
const matches = (pattern: RegExp, content: string): string => JSON.stringify([...content.matchAll(pattern)].map((row) => [row.index, ...row]));
const fixtures = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as { schemaVersion: number; cases: readonly string[] };
if (fixtures.schemaVersion !== 1 || fixtures.cases.length === 0) throw new Error("Missing authored diagnostic cases");
let cancelled = false;
process.on("SIGINT", () => { cancelled = true; });
const check = (): void => { if (cancelled) throw new Error("Diagnostic cancelled"); };
for (const content of fixtures.cases) if (matches(original, content) !== matches(projected, content)) throw new Error(`Authored case mismatch: ${JSON.stringify({ content, original: original.toString(), projected: projected.toString(), before: matches(original, content), after: matches(projected, content) })}`);
const inputs = [normalizerPath, "📜️script.ts", `${library}/🔍️discovery/🟦️component.ts`, `${library}/🧪️tests/🧪️artifact-support-leaf-authority/🟦️.test.ts`];
for (const path of inputs) {
  check();
  const bytes = read(path), content = bytes.toString(), before = hash(bytes), baseline = matches(original, content);
  if (matches(projected, content) !== baseline) throw new Error(`Actual input match mismatch: ${path}`);
  const rounds: { originalMs: number; projectedMs: number }[] = [];
  for (let index = 0; index < 11; index++) {
    check();
    const timing: Record<string, number> = {};
    for (const [name, pattern] of index % 2 === 0 ? [["original", original], ["projected", projected]] as const : [["projected", projected], ["original", original]] as const) {
      const started = performance.now(), actual = matches(pattern, content);
      timing[name] = performance.now() - started;
      if (actual !== baseline) throw new Error(`Timed exact match mismatch: ${path}`);
    }
    rounds.push({ originalMs: timing.original!, projectedMs: timing.projected! });
  }
  const median = (key: "originalMs" | "projectedMs"): number => rounds.map((row) => row[key]).sort((a, b) => a - b)[5]!;
  console.log("[DEBUG] TypeScript binding regex diagnostic", JSON.stringify({ path, bytes: bytes.length, beforeSha256: before, afterSha256: hash(read(path)), exactMatches: JSON.parse(baseline).length, originalMedianMs: median("originalMs"), projectedMedianMs: median("projectedMs"), rounds }));
}
console.log("[DEBUG] TypeScript binding diagnostic complete", JSON.stringify({ authoredCases: fixtures.cases.length, actualInputs: inputs.length, sourceSha256: hash(Buffer.from(source)), original: original.toString(), projected: projected.toString() }));
