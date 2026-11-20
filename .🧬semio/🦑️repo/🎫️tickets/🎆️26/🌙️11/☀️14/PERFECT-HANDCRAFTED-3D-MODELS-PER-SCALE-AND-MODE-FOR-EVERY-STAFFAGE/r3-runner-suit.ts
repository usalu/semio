/** 🖨️ Prints the print-suitability violations of the candidate document at every scale under the default print profile.
 * Usage: `bun r3-runner-suit.ts [catalog]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability, printProfile } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { ASSET_FILE, OUT } from "./r3-runner-roots.ts";
const asset = JSON.parse(readFileSync(process.argv[2] === "catalog" ? ASSET_FILE : join(OUT, "🔣️.json"), "utf8")).asset;
const profile = typeof DEFAULT_PRINT_PROFILE === "string" ? (printProfile as any)(DEFAULT_PRINT_PROFILE) : DEFAULT_PRINT_PROFILE;
for (const scale of [50, 100, 200, 500]) {
  const s = measureSuitability(asset, asset.representations[String(scale)], scale, profile as never);
  console.log(scale, s.suitable, "minFeature", s.minFeatureMm.toFixed(3), "minGap", s.minGapMm.toFixed(3), JSON.stringify(s.violations).slice(0, 600));
}
