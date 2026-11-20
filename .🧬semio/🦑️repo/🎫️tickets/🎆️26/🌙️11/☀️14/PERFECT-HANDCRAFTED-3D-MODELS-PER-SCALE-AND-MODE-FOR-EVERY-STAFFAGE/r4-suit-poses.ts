/** 🔬️ Print suitability of one catalog asset per pose: which pose makes a scale unsuitable, with the violations. Usage: bun r4-suit-poses.ts <category/slug | asset document json> [scale]
 * @see ./r3-pig-suit.ts */
import { existsSync, readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
const asset = JSON.parse(readFileSync(existsSync(process.argv[2]!) ? process.argv[2]! : entryOf(process.argv[2]!).entry.path, "utf8")).asset;
for (const scale of process.argv[3] === undefined ? [50, 100, 200, 500] : [Number(process.argv[3])]) {
  const rep = asset.representations[String(scale)];
  for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) {
    for (const end of new Set([scale, rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
      const s = measureSuitability({ ...asset, id: `${asset.id}-${pose.id}-${end}`, poses: [pose] }, rep, end, DEFAULT_PRINT_PROFILE as never);
      if (!s.suitable) console.log(`1:${scale} at 1:${end} pose ${pose.id}`, s.violations.map((v: unknown) => JSON.stringify(v)).join(" "));
    }
  }
}
console.log("done");
