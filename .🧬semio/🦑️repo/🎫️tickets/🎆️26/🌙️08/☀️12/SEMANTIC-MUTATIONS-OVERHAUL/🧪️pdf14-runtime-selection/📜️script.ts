import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

//#region 🧪️Pdf14RuntimeSelection
const workspace = process.cwd();
const metadataPath = join(import.meta.dir, "../🧪️pdf14-laws-registered-retry-artifacts/semio-nextest-IGAI2l/binaries-metadata.json");
const metadata = JSON.parse(readFileSync(metadataPath, "utf8"));
const binary = metadata["rust-binaries"]["semio-s-plugin-stdio"]["binary-path"] as string;
const sha256 = createHash("sha256").update(readFileSync(binary)).digest("hex");
const { runTestBudgeted, runTestCapturedBudgeted } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts")).href);
const filter = "test(language_neutral_forward_and_concrete_inverse)";
const listed = await runTestCapturedBudgeted("cargo", ["nextest", "list", "--binaries-metadata", metadataPath, "--profile", "quick", "--filter-expr", filter, "--message-format", "json"], { cwd: workspace, env: process.env, budgetMs: 60_000 });
writeFileSync(join(import.meta.dir, "🔣️selected-tests.json"), listed);
const suites = JSON.parse(listed)["rust-suites"] as Record<string, { testcases: Record<string, { ignored: boolean; "filter-match": { status: string } }> }>;
const selected = Object.values(suites).flatMap((suite) => Object.entries(suite.testcases).filter(([, test]) => test["filter-match"].status === "matches" && !test.ignored).map(([name]) => name));
if (selected.length !== 9 || selected.some((name) => !name.includes("artifacts::pdf::") || !name.endsWith("::language_neutral_forward_and_concrete_inverse"))) throw new Error(`Expected exactly nine direct PDF laws, selected ${JSON.stringify(selected)}`);
console.log(`[DEBUG] ${JSON.stringify({ metadataPath, binary, sha256, selectedCount: selected.length, selected })}`);
let passed = false;
try {
  await runTestBudgeted("cargo", ["nextest", "run", "--binaries-metadata", metadataPath, "--profile", "quick", "--filter-expr", filter, "--no-tests", "fail", "--no-fail-fast", "--test-threads", "4", "--status-level", "all", "--final-status-level", "all"], { cwd: workspace, env: process.env, budgetMs: 180_000 });
  passed = true;
} finally {
  const unchanged = createHash("sha256").update(readFileSync(binary)).digest("hex") === sha256;
  writeFileSync(join(import.meta.dir, "🔣️results.json"), JSON.stringify({ metadataPath, binary, sha256, selected, passed, executableUnchanged: unchanged }, null, 2));
  console.log(`[DEBUG] ${JSON.stringify({ passed, executableUnchanged: unchanged })}`);
  if (!unchanged) throw new Error("Compiled PDF test binary changed during execution");
}
//#endregion 🧪️Pdf14RuntimeSelection
