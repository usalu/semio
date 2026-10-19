// 🔭 Shows the planned skeleton script and the module diff of one lift. usage: bun lift-show.ts <substring> [--module]
import {mkdirSync, writeFileSync} from "node:fs";
import {planLift} from "./lift.ts";
import {LIFT_OPTIONS, liftList} from "./plans/lifts.ts";
import {LIB, C, Vfs} from "./vfs.ts";
const v = new Vfs(), key = process.argv[2]!;
const p = liftList().find((x) => x.includes(key))!;
const plan = planLift(p, v.read(p), v, LIFT_OPTIONS[p.replace(LIB + "/", "L/")] ?? {});
if (!plan.ok) { console.log("FAILED:", plan.reason); process.exit(1); }
const dir = `${C}/🗑️generated/tax-stage/show`; mkdirSync(dir, {recursive: true});
writeFileSync(`${dir}/script.ts`, plan.script); writeFileSync(`${dir}/module.ts`, plan.module);
console.log(`module: ${plan.modulePath.replace(LIB + "/", "L/")} merged=${plan.mergedInto} exported=${plan.exported} commands=${plan.commands}\n--- SCRIPT ---\n${plan.script}`);
if (process.argv.includes("--module")) console.log(`--- MODULE ---\n${plan.module}`);
