/** 🖨️ Round 3 roe deer: print suitability of every authored representation of an asset document under the product's default print profile (strokes, gaps, features over all poses), with every violation. Usage: bun r3-roe-deer-print.ts <asset document json> */
import { measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { defaultDepictionOptions } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";
const asset = loadAsset(process.argv[2]!);
const profile = (defaultDepictionOptions as any)?.profile ?? (defaultDepictionOptions as any)().profile;
for (const scale of [50, 100, 200, 500]) {
  const s = measureSuitability(asset as never, asset.representations[String(scale)], scale, profile) as any;
  console.log(scale, s.suitable, "stroke", s.minStrokeMm, "gap", s.minGapMm, "feature", s.minFeatureMm);
  for (const v of s.violations.slice(0, 14)) console.log("  ", JSON.stringify(v));
  if (s.violations.length > 14) console.log("   ...", s.violations.length);
}
