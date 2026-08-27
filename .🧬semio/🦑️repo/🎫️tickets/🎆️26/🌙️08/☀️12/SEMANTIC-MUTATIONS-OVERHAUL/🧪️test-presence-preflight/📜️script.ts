import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

//#region 🧪️TestPresencePreflight
const workspace = process.cwd();
const { policyMutationStructuralBreaches } = await import(pathToFileURL(join(workspace, "📜️script.ts")).href);
const vectors = JSON.parse(readFileSync(join(import.meta.dir, "🔣️vectors.json"), "utf8")) as {
  schemaVersion: number;
  mutationRoot: string;
  leaf: string;
  cases: { name: string; source: string; files: Record<string, string>; emptyTestDirectory?: boolean; runnableTests: number }[];
};
if (vectors.schemaVersion !== 1) throw new Error("Unexpected test-presence fixture schema");
const run = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const results: unknown[] = [];
let mismatches = 0;
for (const vector of vectors.cases) {
  const root = join(run, vector.name);
  const leaf = join(root, vectors.mutationRoot, vectors.leaf);
  mkdirSync(leaf, { recursive: true });
  if (vector.emptyTestDirectory) mkdirSync(join(leaf, "🧪️tests"));
  writeFileSync(join(leaf, "🦀️.rs"), vector.source);
  const aggregate = join(root, vectors.mutationRoot, "🦀️.rs");
  writeFileSync(aggregate, `#[path = "${vectors.leaf}/🦀️.rs"] pub mod insert_page; pub enum ProbeMutation { InsertPage(insert_page::InsertPage) }\n`);
  for (const [path, source] of Object.entries(vector.files)) {
    const target = join(leaf, path);
    mkdirSync(dirname(target), { recursive: true });
    writeFileSync(target, source);
  }
  const executable = join(root, process.platform === "win32" ? "oracle.exe" : "oracle");
  const compile = spawnSync("rustc", ["--test", "--edition", "2021", "--crate-name", "test_presence_oracle", aggregate, "-o", executable], { encoding: "utf8" });
  writeFileSync(join(root, "compiler.stdout.log"), compile.stdout ?? "");
  writeFileSync(join(root, "compiler.stderr.log"), compile.stderr ?? "");
  if (compile.status !== 0) throw new Error(`${vector.name}: compiler oracle failed`);
  const execution = spawnSync(executable, ["--nocapture"], { encoding: "utf8" });
  writeFileSync(join(root, "runtime.stdout.log"), execution.stdout ?? "");
  writeFileSync(join(root, "runtime.stderr.log"), execution.stderr ?? "");
  const runnable = Number(/test result: ok\. (\d+) passed;/u.exec(execution.stdout)?.[1]);
  if (execution.status !== 0 || runnable !== vector.runnableTests) throw new Error(`${vector.name}: actual compiler test count disagrees with fixture`);
  const findings = policyMutationStructuralBreaches(root, [vectors.mutationRoot]).filter(({ kind }: { kind: string }) => kind === "mutation/test-presence");
  const accepted = findings.length === 0;
  const expected = vector.runnableTests > 0;
  if (accepted !== expected) mismatches += 1;
  const result = { name: vector.name, runnable, expected, accepted, findings };
  results.push(result);
  console.log(`[DEBUG] ${JSON.stringify(result)}`);
}
writeFileSync(join(run, "🔣️results.json"), JSON.stringify({ mismatches, results }, null, 2));
console.log(`[DEBUG] test-presence cases=${results.length} mismatches=${mismatches} artifacts=${run}`);
process.exitCode = mismatches === 0 ? 0 : 1;
//#endregion 🧪️TestPresencePreflight
