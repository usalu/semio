import { scanTestLayout } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT;
if (!root) throw new Error("SEMIO_FIXTURE_REPO_ROOT is required");
const ticket = dirname(dirname(import.meta.dir));
const lane = join(ticket, "🗑️generated", "coordinator");
mkdirSync(lane, { recursive: true });
const controller = new AbortController();
process.on("SIGINT", () => controller.abort());
process.on("SIGTERM", () => controller.abort());
let last = 0, phase = "", total = 0;
const findings = await scanTestLayout(root, { concurrency: 32, signal: controller.signal, progress: progress => {
  if (progress.phase === "inspect") total = progress.total;
  if (phase !== progress.phase || progress.completed - last >= 5000 || progress.completed === progress.total) {
    phase = progress.phase; last = progress.completed;
    console.log(`[DEBUG] fixture layout ${phase}: ${progress.completed}/${progress.total}`);
  }
} });
const counts = Object.fromEntries([...new Set(findings.map(row => row.code))].map(code => [code, findings.filter(row => row.code === code).length]));
writeFileSync(join(lane, "fixture-layout-current.json"), JSON.stringify(findings, null, 2) + "\n");
writeFileSync(join(ticket, "📓️fixture-layout-current-2026-09-09.md"), "# Current Fixture and Test Layout Snapshot\n\nCompleted at " + new Date().toISOString() + ". Authored file paths inspected: " + total + ". This snapshot covers physical layout; production dependency and fixture-content classification require their separate audits.\n\n```json\n" + JSON.stringify({ counts, findings }, null, 2) + "\n```\n");
console.log(JSON.stringify({ files: total, findings: findings.length, counts }));
process.exitCode = findings.length ? 1 : 0;
