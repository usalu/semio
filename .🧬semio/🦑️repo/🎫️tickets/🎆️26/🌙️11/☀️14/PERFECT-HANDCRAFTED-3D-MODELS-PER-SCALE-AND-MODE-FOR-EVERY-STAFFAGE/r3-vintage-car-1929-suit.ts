/** 🖨️ Prints the print-suitability violations of every authored scale of an asset document under the default print profile. Usage: bun r3-vintage-car-1929-suit.ts [asset document json] */
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf, loadAsset } from "./volumes-lib.ts";

const file = process.argv[2];
const asset: any = file === undefined ? entryOf("vehicles/vintage-car-1929").asset : loadAsset(file);
for (const denominator of [50, 100, 200, 500]) {
  const r: any = resolveRepresentation(asset, { denominator } as never, DEFAULT_PRINT_PROFILE as never);
  process.stdout.write(`1:${denominator} ${r.kind} ${(r.violations ?? r.suitability?.violations ?? []).length}\n`);
  for (const v of r.violations ?? r.suitability?.violations ?? []) process.stdout.write(`  ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(1)},${v.at.yMm.toFixed(1)} mm` : ""}\n`);
}
