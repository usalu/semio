import { catalogEntries, loadAsset } from "./volumes-lib.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const requested = process.argv.slice(2);
for (const name of requested.length ? requested : ["stepladder"]) {
  const asset = loadAsset(catalogEntries().find(entry => entry.slug === name)!.path);
  let samples = 0, failures = 0;
  for (const scale of [50, 100, 200, 500] as const) {
    const representation = asset.representations[scale];
    if (representation.status !== "authored") continue;
    const { minDenominator, maxDenominator } = representation.validRange;
    for (let i = 0; i <= 12; i++) {
      const denominator = Math.round(minDenominator + (maxDenominator - minDenominator) * i / 12);
      const result = measureSuitability(asset, representation, denominator, DEFAULT_PRINT_PROFILE);
      samples++;
      if (!result.suitable) {
        failures++;
        console.log(name, scale, denominator, JSON.stringify(result.violations.slice(0, 4)));
      }
    }
  }
  console.log(JSON.stringify({ name, samples, failures }));
  if (failures) process.exitCode = 1;
}
