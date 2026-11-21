/** 📐️ Tests candidate range boundaries against the actual printed drawing in all poses and views. */
import { Glob } from "bun";
import { resolve } from "node:path";
import { uncoveredDenominators } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import type { Asset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const requested = new Set(process.argv.slice(2));
for (const assetPath of [...new Glob("*/*/🔣️.json").scanSync(root)].sort()) {
  if (requested.size && !requested.has(assetPath.split("/")[1]!)) continue;
  const { asset } = await Bun.file(resolve(root, assetPath)).json() as { asset: Asset };
  for (const { from, to, border } of uncoveredDenominators(asset)) {
    const key = String(border) as keyof Asset["representations"];
    const representation = asset.representations[key];
    if (representation?.status !== "authored") throw new Error(`${asset.id}: missing authored range ${key}`);
    const previous = representation.validRange;
    const validRange = { minDenominator: Math.min(previous.minDenominator, from), maxDenominator: Math.max(previous.maxDenominator, to) };
    const candidate = { ...representation, validRange };
    const proposed = { ...asset, representations: { ...asset.representations, [key]: candidate } };
    const ends = [validRange.minDenominator, validRange.maxDenominator].filter((end) => end < previous.minDenominator || end > previous.maxDenominator);
    for (const end of ends) {
      const { violations } = measureSuitability(proposed, candidate, end, DEFAULT_PRINT_PROFILE);
      console.log(JSON.stringify({ asset: asset.id, preset: border, end, violations }));
    }
  }
}
