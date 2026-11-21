/** 📏 Prints the print-suitability violations of every authored representation of vehicles/vintage-car-1929 at its nominal scale and at both ends of its valid range, with the place on the sheet. Usage: bun r5-vintage-car-1929-ranges.ts [asset document json] */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf, loadAsset } from "./volumes-lib.ts";

const file = process.argv[2];
const asset: any = file === undefined ? entryOf("vehicles/vintage-car-1929").asset : loadAsset(file);
for (const denominator of [50, 100, 200, 500]) {
  const rep = asset.representations[`${denominator}`];
  for (const end of [...new Set([denominator, rep.validRange.minDenominator, rep.validRange.maxDenominator])]) {
    const s: any = measureSuitability(asset, rep, end, DEFAULT_PRINT_PROFILE as never);
    for (const v of s.violations) process.stdout.write(`rep ${denominator} at 1:${end} ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm} ${JSON.stringify({ ...v, shapes: undefined })}\n`);
  }
}
