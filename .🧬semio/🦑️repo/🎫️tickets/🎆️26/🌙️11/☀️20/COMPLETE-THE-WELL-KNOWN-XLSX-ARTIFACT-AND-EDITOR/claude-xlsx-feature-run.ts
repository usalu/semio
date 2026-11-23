import { spawnSync } from "node:child_process";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";

const M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root = resolve(import.meta.dir, "../../../../../../..");
const { discoverTestCases, planExecution, readResults } = await import(resolve(root, M));
const [, , binary, role, level, caseDir] = process.argv;
const found = discoverTestCases(root).find((entry: { caseDir: string }) => entry.caseDir.endsWith(caseDir!));
if (!found) throw new Error("case not found: " + caseDir);
const planned = planExecution(root, found, level as never, role as never, "rust");
const run = spawnSync(binary!, ["--plan", planned.planPath, "--out", planned.plan.resultsPath], { cwd: root, encoding: "utf8", env: process.env, maxBuffer: 1 << 28 });
console.log("[DEBUG] exit", run.status, "scenarios", planned.plan.scenarios.length);
if (run.stderr) console.log(run.stderr.slice(-4000));
const results = existsSync(planned.plan.resultsPath) ? readResults(planned.plan.resultsPath).results : [];
const tally: Record<string, number> = {};
for (const result of results) tally[(result as { status: string }).status] = (tally[(result as { status: string }).status] ?? 0) + 1;
console.log("[DEBUG] tally", JSON.stringify(tally));
for (const result of results as { status: string; scenario?: string; message?: string }[]) if (result.status !== "passed") console.log("[DEBUG] not passed", JSON.stringify(result).slice(0, 600));
