/** 🖨️ Prints the print-suitability violations of every authored representation of `machinery/concrete-mixer-truck` under the default print profile: `bun r3-concrete-mixer-truck-print.ts`. */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const asset = entryOf("machinery/concrete-mixer-truck").asset as any;
for (const d of [50, 100, 200, 500]) {
  const s = measureSuitability(asset, asset.representations[String(d)], d, DEFAULT_PRINT_PROFILE as never) as any;
  console.log(`1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm} gap=${s.minGapMm} feature=${s.minFeatureMm}`);
  for (const v of s.violations) console.log("  ", JSON.stringify(v).slice(0, 300));
}
