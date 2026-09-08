import ts from "typescript";
import { writeFile, mkdir } from "node:fs/promises";

type Literal = { value: string; context: string; line: number };
type Pair = { oldPath: string; newPath: string };
const root = process.env.SEMIO_LAYOUT_REPO_ROOT ?? process.cwd();
process.chdir(root);
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR";
const base = "6152f9ca6a0fbb55aa61992077837a230996b51d";
await mkdir(`${ticket}/🗑️generated/typescript-finish`, { recursive: true });
const diff = Bun.spawnSync(["git", "diff", base, "--name-status", "-z"], { cwd: root, stdout: "pipe" });
if (diff.exitCode !== 0) throw new Error(diff.stderr.toString());
const fields = diff.stdout.toString().split("\0").filter(Boolean);
const pairMap = new Map<string, Pair>();
const addPair = (oldPath: string, newPath: string) => pairMap.set(`${oldPath}\0${newPath}`, { oldPath, newPath });
for (let i = 0; i < fields.length;) {
  const status = fields[i++]!;
  if (!status.startsWith("R")) { i++; continue; }
  const oldPath = fields[i++]!, newPath = fields[i++]!;
  const own = newPath === "vitest.config.ts" || newPath.includes("/vitest.config.ts") || newPath.includes("/🧪️tests/📚️storybook/") || newPath.includes("/🧪️tests/🖼️webgpu-surface/") || newPath.includes("/🧪️tests/🌐️browser-declaration/") || newPath.includes("/🧪️tests/🔎️scalar-witness/") || newPath.includes("/🏪️store/🧪️tests/") || newPath.includes("/📇️directory/🧪️tests/") || newPath.includes("/🔌️plugin/🧪️tests/") || newPath.includes("/🗣️dsl/🧪️tests/") || newPath.includes("/📚️library/⚡️caching/🧪️tests/") || newPath === "🧪️tests/🗿️artifact-runner/🟦️.ts" || newPath.includes("/🧪️tests/📚️storybook-types/") || newPath.includes("/🧪️tests/🧹️react-environment/");
  if (own && /\.(?:[cm]?js|tsx?)$/.test(newPath)) addPair(oldPath, newPath);
}
const baseFiles = Bun.spawnSync(["git", "ls-tree", "-r", "--name-only", base], { cwd: root, stdout: "pipe" }).stdout.toString().split("\n").filter(Boolean);
for (const oldPath of baseFiles) {
  let candidate: string | undefined;
  if (oldPath.endsWith("/🧪️tests/🟦️.ts")) candidate = oldPath.slice(0, -"/🧪️tests/🟦️.ts".length) + "/vitest.config.ts";
  else if (oldPath.endsWith("/🧪️vitest.config.ts") || oldPath.endsWith("/⚡️vitest.config.ts")) candidate = oldPath.replace(/[^/]+$/, "vitest.config.ts");
  else if (oldPath.endsWith("/🧪️.story.tsx")) candidate = oldPath.slice(0, -"/🧪️.story.tsx".length) + "/🧪️tests/📚️storybook/🟦️.tsx";
  else if (["🧪️tests/🟦️.ts", "🧪️vitest.config.ts", "⚡️vitest.config.ts"].includes(oldPath)) candidate = "vitest.config.ts";
  if (candidate && await Bun.file(candidate).exists()) addPair(oldPath, candidate);
}
const explicitPairs: Pair[] = [
  { oldPath: "🗿️artifact.ts", newPath: "🧪️tests/🗿️artifact-runner/🟦️.ts" },
  { oldPath: "🧰️framework/🔨️modules/🖱️ui/🧪️story.ts", newPath: "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-types/🟦️.ts" },
  { oldPath: "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️tests/🖼️surface/🟨️.js", newPath: "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js" },
  { oldPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport/🟦️.ts", newPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts" },
  { oldPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration/🟦️.ts", newPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts" },
  { oldPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts", newPath: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts" },
  { oldPath: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts", newPath: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts" },
  { oldPath: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts", newPath: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts" },
  { oldPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts", newPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts" },
];
for (const pair of explicitPairs) if (await Bun.file(pair.newPath).exists()) addPair(pair.oldPath, pair.newPath);
const pairs = [...pairMap.values()];
await writeFile(`${ticket}/🗑️generated/typescript-finish/move-pairs.json`, JSON.stringify(pairs, null, 2));

const kind = (node: ts.Node): string => {
  let cursor: ts.Node | undefined = node;
  while (cursor) {
    if (ts.isImportDeclaration(cursor) || ts.isExportDeclaration(cursor) || ts.isImportTypeNode(cursor)) return "module";
    if (ts.isCallExpression(cursor)) {
      if (cursor.expression.kind === ts.SyntaxKind.ImportKeyword) return "module";
      const name = cursor.expression.getText().split(".").at(-1);
      if (["resolve", "join", "readFile", "readFileSync", "writeFile", "writeFileSync", "existsSync", "mkdir", "rm", "stat"].includes(name ?? "")) return "path";
      if (["toBe", "toEqual", "toContain", "toMatch", "deepEqual", "equal", "strictEqual", "assert"].includes(name ?? "")) return "expectation";
    }
    cursor = cursor.parent;
  }
  return "ordinary";
};
const literals = (path: string, source: string): Literal[] => {
  const file = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, path.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const out: Literal[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isStringLiteralLike(node)) out.push({ value: node.text, context: kind(node), line: file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1 });
    ts.forEachChild(node, visit);
  };
  visit(file);
  return out;
};
const count = (xs: Literal[]) => {
  const out = new Map<string, Literal[]>();
  for (const x of xs) out.set(x.value, [...(out.get(x.value) ?? []), x]);
  return out;
};
const strip = (value: string) => value.replace(/^(?:\.\.\/|\.\/)+/, "");
const reports = [];
for (const pair of pairs) {
  const oldResult = Bun.spawnSync(["git", "show", `${base}:${pair.oldPath}`], { cwd: root, stdout: "pipe", stderr: "pipe" });
  if (oldResult.exitCode !== 0) continue;
  const oldValues = count(literals(pair.oldPath, oldResult.stdout.toString()));
  const newValues = count(literals(pair.newPath, await Bun.file(pair.newPath).text()));
  const rebases = [];
  for (const [oldValue, oldItems] of oldValues) {
    if (!/^(?:\.\.\/|\.\/)/.test(oldValue)) continue;
    const oldRemaining = oldItems.length - (newValues.get(oldValue)?.length ?? 0);
    if (oldRemaining <= 0) continue;
    for (const [newValue, newItems] of newValues) {
      if (newValue === oldValue || strip(newValue) !== strip(oldValue)) continue;
      const newRemaining = newItems.length - (oldValues.get(newValue)?.length ?? 0);
      if (newRemaining <= 0) continue;
      rebases.push({ oldValue, newValue, oldItems: oldItems.slice(0, oldRemaining), newItems: newItems.slice(0, newRemaining) });
    }
  }
  if (rebases.length) reports.push({ ...pair, rebases });
}
const result = { pairCount: pairs.length, reports, suspicious: reports.flatMap(report => report.rebases.filter(rebase => rebase.oldItems.some(item => !["module", "path"].includes(item.context)) || rebase.newItems.some(item => !["module", "path"].includes(item.context))).map(rebase => ({ oldPath: report.oldPath, newPath: report.newPath, ...rebase }))) };
await writeFile(`${ticket}/🗑️generated/typescript-finish/literal-anchor-audit.json`, JSON.stringify(result, null, 2));
console.log(JSON.stringify({ pairCount: result.pairCount, changedPairs: reports.length, suspicious: result.suspicious }, null, 2));
