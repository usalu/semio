/** 🔎️ Leaves out a named set of 1:50 shapes of one view from the sleeping person's candidate and reports whether the product's print suitability measurement then runs (debug aid of round 5).
 * Usage: `bun r5-sleeping-person-culprit.ts <view> <id,id,...>` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-sleeping-person", "🔣️.json"), "utf8")).asset;
const [view, ids] = [process.argv[2] ?? "front", new Set((process.argv[3] ?? "").split(","))];
const rep = asset.representations["50"];
rep.geometry[view] = (rep.geometry[view] as any[]).filter((s) => !ids.has(s.id));
const trial = rep;
try {
  const s = measureSuitability(asset, trial, 50, DEFAULT_PRINT_PROFILE) as any;
  console.log("runs", s.suitable, s.minFeatureMm);
} catch (e) {
  console.log("throws", (e as Error).message.slice(0, 80));
}
