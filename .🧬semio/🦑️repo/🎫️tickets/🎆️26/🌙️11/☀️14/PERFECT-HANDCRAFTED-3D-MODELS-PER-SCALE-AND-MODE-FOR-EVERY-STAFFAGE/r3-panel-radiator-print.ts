/** 🖨️ Round 3 panel radiator: the print suitability of every authored drawing of an asset document (violations per scale with the shapes and measured widths), for judging 2D root fixes before they reach the catalog.
 *
 * Usage: bun r3-panel-radiator-print.ts <asset document json>
 * @see ./r3-components-panel-radiator.md */
import { readFileSync } from "node:fs";

import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const document = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const asset = document.asset;
for (const [scale, representation] of Object.entries(asset.representations) as [string, { status: string }][]) {
  if (representation.status !== "authored") continue;
  const s = measureSuitability(asset, representation as never, Number(scale), DEFAULT_PRINT_PROFILE);
  console.log(`1:${scale} suitable=${s.suitable} minGapMm=${s.minGapMm.toFixed(3)} minFeatureMm=${s.minFeatureMm.toFixed(3)}`);
  for (const v of s.violations) console.log(`  ${JSON.stringify(v)}`);
}
