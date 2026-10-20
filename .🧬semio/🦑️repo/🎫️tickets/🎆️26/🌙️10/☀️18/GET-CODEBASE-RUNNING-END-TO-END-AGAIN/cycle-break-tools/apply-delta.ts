/** 🚀 Applies the merged staged delta to the live tree after the freeze is lifted: guard pass first (sha256 of every moved or deleted file, exact anchors, round-trip manifests), then one write pass with backups.
 *
 *   bun apply-delta.ts            guard pass only
 *   bun apply-delta.ts --write    guard pass, then write
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPlan, type PlanJson } from "./apply-plan.ts";
import { GENERATED, ROOT, TICKET } from "./lib.ts";
const delta = JSON.parse(readFileSync(resolve(TICKET, "cycle-break-delta.json"), "utf8")) as PlanJson & { flags: { severity: string; message: string }[] };
const blockers = delta.flags.filter(f => f.severity === "blocker");
if (blockers.length) { console.error("REFUSED: blockers\n- " + blockers.map(b => b.message).join("\n- ")); process.exit(3); }
const guard = applyPlan(delta, ROOT, { write: false });
if (guard.problems.length) { console.error(`REFUSED: ${guard.problems.length} problems\n- ` + guard.problems.slice(0, 60).join("\n- ")); process.exit(3); }
console.log(`guard pass clean: ${guard.touched} files`);
if (!process.argv.includes("--write")) process.exit(0);
const backup = resolve(GENERATED, `applied-delta-${new Date().toISOString().replace(/[:.]/gu, "-")}`);
const w = applyPlan(delta, ROOT, { write: true, backup });
console.log(`applied: ${w.touched} files; backup ${backup}`);
