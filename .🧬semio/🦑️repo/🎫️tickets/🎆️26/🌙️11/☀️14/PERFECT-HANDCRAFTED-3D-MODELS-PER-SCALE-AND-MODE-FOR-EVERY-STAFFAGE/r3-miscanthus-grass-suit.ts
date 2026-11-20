/** 🌾 Prints the print-suitability violations of the miscanthus drawings per scale. */
import { resolveRepresentation, DEFAULT_PRINT_PROFILE } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";
const file = process.argv[2];
const asset = loadAsset(file) as never;
for (const d of [50, 100, 200, 500]) {
  try {
    const r = (resolveRepresentation as never as (...a: unknown[]) => { kind: string; suitability?: { violations: unknown[] } })(asset, { denominator: d }, DEFAULT_PRINT_PROFILE);
    console.log(d, r.kind, JSON.stringify(r.suitability?.violations ?? []).slice(0, 1500));
  } catch (e) { console.log(d, "threw", String(e).slice(0, 300)); }
}
