#!/usr/bin/env bun
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const { policy } = await import(join(repoRoot, "📜️script.ts"));
const breaches = await policy({});
const p3 = breaches.filter((b) => String(b.kind).startsWith("handcrafted-grammar/"));
const kinds = {};
for (const b of p3) kinds[b.kind] = (kinds[b.kind] || 0) + 1;
const report = {
  totalBreaches: breaches.length,
  p3: p3.length,
  p3High: p3.filter((b) => b.priority === "high").length,
  kinds,
  sample: p3.slice(0, 10),
};
writeFileSync(join(ticketDir, "🧪p3-policy-run.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ totalBreaches: report.totalBreaches, p3: report.p3, p3High: report.p3High, kinds }, null, 2));
if (report.p3High > 0) {
  console.error("[DEBUG] P3 high breaches still firing:");
  for (const b of p3.slice(0, 20)) console.error(`  ${b.kind}: ${b.summary}`);
  process.exit(1);
}
