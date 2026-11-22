/** 🖨️ Print suitability violations of an asset at the given scales with their full data. */
import { loadDoc } from "./lib.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [slug, ...scales] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
for (const sc of scales.length ? scales : ["500", "200", "100", "50"]) {
  const s = measureSuitability(a, a.representations[sc], Number(sc) as never, DEFAULT_PRINT_PROFILE);
  console.log(sc, s.suitable, s.minStrokeMm, s.minGapMm, s.minFeatureMm);
  for (const v of s.violations.slice(0, 8)) console.log("  ", JSON.stringify(v).slice(0, 400));
}
