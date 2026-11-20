/** 🧾 Prints the print-suitability violations of every authored scale of the cargo bike (catalog file or `--file` candidate): the product refuses to draw an unsuitable scale, which `validateAsset` does not report.
 *
 * Usage: `bun r3-cargo-bike-suit.ts [--file <asset document json>]`
 * @see ./r3-tools.md */
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf, loadAsset, type Json } from "./volumes-lib.ts";

const i = process.argv.indexOf("--file");
const asset: Json = i < 0 ? entryOf("vehicles/cargo-bike").asset : loadAsset(process.argv[i + 1]!);
let bad = 0;
for (const scale of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset as never, { denominator: scale } as never, DEFAULT_PRINT_PROFILE) as Json;
  const s = r.suitability;
  process.stdout.write(`1:${scale} ${r.kind} minStroke ${s?.minStrokeMm?.toFixed(3)} minGap ${s?.minGapMm?.toFixed(3)} minFeature ${s?.minFeatureMm?.toFixed(3)}\n`);
  for (const v of s?.violations ?? []) {
    bad++;
    process.stdout.write(`  ${v.check} ${v.shapes.join(",")} measured ${v.measuredMm.toFixed(3)} required ${v.requiredMm}${v.at === undefined ? "" : ` at ${v.at.xMm.toFixed(1)},${v.at.yMm.toFixed(1)} mm`}\n`);
  }
}
process.exitCode = bad > 0 ? 1 : 0;
