/** 📐 Round 5: walks the denominators of one representation band and prints the print-suitability violations at each, so a band end is chosen where every drawn feature stays printable: `bun r5-machinery-range.ts <category/slug> <from> <to> <step>`.
 * @see ./r3-tools.md */
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const [id, from, to, step] = process.argv.slice(2);
const asset = entryOf(id!).asset;
for (let d = Number(from); d <= Number(to); d += Number(step)) {
  const resolved = (resolveRepresentation as any)(asset, { denominator: d }, DEFAULT_PRINT_PROFILE);
  const violations = resolved.suitability?.violations ?? resolved.violations ?? [];
  const worst = violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${Number(v.measuredMm).toFixed(3)} ${v.view ?? ""} ${v.pose ?? ""}`).slice(0, 8);
  console.log(`1:${d} ${resolved.kind} ${resolved.representation?.denominator ?? ""} violations ${violations.length} ${worst.join(" | ")}`);
}
