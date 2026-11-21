/** 🖨️ Runs the product's print suitability per scale of the sleeping person's candidate document and names the scale whose measurement throws (debug aid of round 5).
 * Usage: `bun r5-sleeping-person-print.ts` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-sleeping-person", "🔣️.json"), "utf8")).asset;
for (const [scale, rep] of Object.entries(asset.representations as Record<string, any>)) {
  try {
    const s = measureSuitability(asset, rep, Number(scale), DEFAULT_PRINT_PROFILE) as any;
    console.log(scale, s.suitable, s.minFeatureMm, JSON.stringify(s.violations?.slice(0, 4)));
  } catch (e) {
    console.log(scale, "THREW", (e as Error).message.slice(0, 120));
  }
}
