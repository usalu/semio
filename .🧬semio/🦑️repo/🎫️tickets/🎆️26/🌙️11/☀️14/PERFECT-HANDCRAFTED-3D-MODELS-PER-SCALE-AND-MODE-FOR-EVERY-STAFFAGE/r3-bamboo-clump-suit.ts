/** 🩺️ Prints the print-suitability violations of every authored bamboo-clump representation under the default print profile. */
import { DEFAULT_PRINT_PROFILE, measureSuitability, printProfile } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";

const file = process.argv[2];
const asset = loadAsset(file) as never as { representations: Record<string, never> };
const profile = typeof DEFAULT_PRINT_PROFILE === "string" ? printProfile(DEFAULT_PRINT_PROFILE as never) : DEFAULT_PRINT_PROFILE;
for (const scale of (process.argv[3] ?? "50,100,200,500").split(",")) {
  const s = measureSuitability(asset as never, asset.representations[scale], Number(scale), profile as never);
  console.log(`1:${scale} suitable=${s.suitable} minStroke=${s.minStrokeMm} minGap=${s.minGapMm} minFeature=${s.minFeatureMm}`);
  for (const v of s.violations.slice(0, 25)) console.log("  ", JSON.stringify(v).slice(0, 300));
  if (s.violations.length > 25) console.log(`   ... ${s.violations.length} violations`);
}
