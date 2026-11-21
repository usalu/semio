/** 📏️ Checks all actual poses and views of three appliance drawings on the default print profile. */
import { resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import type { Asset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances");
const exhaustive = process.argv.includes("--exhaustive");
const requested = process.argv.slice(2).filter((arg) => arg !== "--exhaustive");
for (const id of requested.length ? requested : ["gas-meter", "laptop", "mvhr-unit"]) {
  const { asset } = await Bun.file(resolve(root, id, "🔣️.json")).json() as { asset: Asset };
  const drawing = asset.representations["200"];
  if (drawing.status !== "authored") throw new Error(`${id}: 1:200 drawing is unavailable`);
  const min = drawing.validRange.minDenominator;
  const max = drawing.validRange.maxDenominator;
  const scales = exhaustive ? Array.from({ length: max - min + 1 }, (_, index) => min + index) : [min, 200, Math.floor((min + max) / 2), 300, max];
  let failedScales = 0;
  for (const scale of scales) {
    const measurement = measureSuitability(asset, drawing, scale, DEFAULT_PRINT_PROFILE);
    if (measurement.suitable) continue;
    failedScales++;
    console.log(JSON.stringify({ id, scale, violations: measurement.violations }));
  }
  const issues = validateAsset(asset);
  console.log(JSON.stringify({ id, scales: scales.length, failedScales, issues: issues.map(({ code, path }) => ({ code, path })) }));
  if (min !== 150 || max !== 350 || failedScales || issues.length) process.exitCode = 1;
}
