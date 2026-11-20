/** 🖨️ Round 3, machinery/diesel-generator: print suitability of every authored scale of an asset document under the default print profile (which shapes are too thin or too close on paper). Usage: bun r3-diesel-generator-suit.ts <asset document json> */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const doc = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const asset = doc.asset;
for (const [key, rep] of Object.entries(asset.representations) as [string, { status: string; denominator: number }][]) {
  if (rep.status !== "authored") continue;
  const s = measureSuitability(asset, rep as never, rep.denominator, DEFAULT_PRINT_PROFILE);
  console.log(`1:${key} suitable=${s.suitable} minGap=${s.minGapMm.toFixed(3)} minFeature=${s.minFeatureMm.toFixed(3)}`);
  for (const v of s.violations) console.log("   ", JSON.stringify(v));
}
