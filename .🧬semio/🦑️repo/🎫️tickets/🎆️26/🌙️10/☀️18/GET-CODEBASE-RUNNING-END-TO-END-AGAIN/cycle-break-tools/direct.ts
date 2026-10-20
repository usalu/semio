/** 🎯 Writes a plan to a delta part file and optionally applies it to the live tree (direct, non-manifest edits only). */
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyPlan, type PlanJson } from "./apply-plan.ts";
import { GENERATED, ROOT, TICKET } from "./lib.ts";
import type { Plan } from "./plan.ts";

export function finish(plan: Plan, name: string, argv: string[]): void {
  const json = JSON.parse(JSON.stringify(plan)) as PlanJson & { flags: unknown[] };
  mkdirSync(resolve(import.meta.dir, "delta-parts"), { recursive: true });
  writeFileSync(resolve(import.meta.dir, `delta-parts/${name}.json`), JSON.stringify(json, null, 1));
  const blockers = plan.flags.filter(f => f.severity === "blocker");
  console.log(`${name}: manifestOps ${json.manifestOps.length} newFiles ${json.newFiles.length} edits ${json.edits.length} fileOps ${json.fileOps.length} flags ${plan.flags.length} (blockers ${blockers.length})`);
  for (const f of plan.flags.slice(0, 40)) console.log(`  [${f.severity}] ${f.wave}: ${f.message.slice(0, 220)}`);
  if (!argv.includes("--apply")) return;
  if (blockers.length) { console.error("REFUSED: blockers present"); process.exit(3); }
  const backup = resolve(GENERATED, `applied-${name}-${new Date().toISOString().replace(/[:.]/gu, "-")}`);
  const r = applyPlan(json, ROOT, { write: false });
  if (r.problems.length) { console.error("REFUSED:\n- " + r.problems.slice(0, 40).join("\n- ")); process.exit(3); }
  const w = applyPlan(json, ROOT, { write: true, backup });
  console.log(`applied ${name}: touched ${w.touched}; backup ${backup}`);
}
void readFileSync; void rmSync; void TICKET;
