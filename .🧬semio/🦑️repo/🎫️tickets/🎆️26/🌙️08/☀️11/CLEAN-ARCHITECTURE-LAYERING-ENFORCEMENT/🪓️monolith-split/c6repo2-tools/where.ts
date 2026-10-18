import { assign, exportTable, type Plan } from "./split.ts";
import { readFileSync } from "node:fs";
import { dirname, basename } from "node:path";
const [planFile, backupParent, ...names] = process.argv.slice(2);
const plan: Plan = JSON.parse(readFileSync(planFile, "utf8"));
const a = assign({ ...plan, file: backupParent, dir: plan.dir ?? dirname(plan.file) });
const t = exportTable(a);
for (const n of names) {
  const e = t.get(n);
  if (!e) { console.log(`${n}: ?`); continue; }
  console.log(`${n}: ${e.chunk !== undefined ? a.plan.chunks[e.chunk].dir : JSON.stringify(e.reexport)}`);
}
