/** 📐️ Round 5: prints every print-profile violation of humans/cyclist at the preset and both ends of the valid range of every authored representation, with the viewpoint place of the thinnest spot (catalog file, or `--file` candidate).
 * Usage: `bun r5-cyclist-ranges.ts [--file <asset document json>]`
 * @see ./r3-cyclist-suit.ts */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf, loadAsset, type Json } from "./volumes-lib.ts";

const i = process.argv.indexOf("--file");
const asset: Json = i < 0 ? entryOf("humans/cyclist").asset : loadAsset(process.argv[i + 1]!);
let bad = 0;
for (const scale of [50, 100, 200, 500]) {
  const rep = asset.representations[String(scale)];
  for (const end of new Set([scale, rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
    for (const v of (measureSuitability(asset as never, rep, end, DEFAULT_PRINT_PROFILE) as Json).violations) {
      bad++;
      console.log(`1:${scale} at 1:${end} ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm} ${JSON.stringify({ ...v, shapes: undefined, check: undefined, measuredMm: undefined, requiredMm: undefined })}`);
    }
  }
}
console.log(bad === 0 ? "ranges clean" : `${bad} violation(s)`);
