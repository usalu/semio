/** 🪚️ Checks the 1:50 bench and table drawings on the default print profile across the complete band. */
import { resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import type { Asset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture");
const exhaustive = process.argv.includes("--exhaustive");
for (const id of ["park-bench", "picnic-table"]) {
  const { asset } = await Bun.file(resolve(root, id, "🔣️.json")).json() as { asset: Asset };
  const authored = asset.representations["50"];
  if (authored.status !== "authored") throw new Error(`${id}: 1:50 drawing is unavailable`);
  const maximum = authored.validRange.maxDenominator;
  const drawing = { ...authored, validRange: { minDenominator: 20, maxDenominator: maximum } };
  const next = asset.representations["100"];
  const following = next.status === "authored" ? { ...next, validRange: { ...next.validRange, minDenominator: maximum + 1 } } : next;
  const proposed = { ...asset, representations: { ...asset.representations, "50": drawing, "100": following } };
  const scales = exhaustive ? Array.from({ length: maximum - 19 }, (_, index) => 20 + index) : [...new Set([20, 25, 30, 40, 50, maximum])];
  let smallestGap = Infinity;
  let smallestGapAt = 0;
  let failedScales = 0;
  for (const scale of scales) {
    const measurement = measureSuitability(proposed, drawing, scale, DEFAULT_PRINT_PROFILE);
    if (measurement.minGapMm < smallestGap) {
      smallestGap = measurement.minGapMm;
      smallestGapAt = scale;
    }
    if (measurement.suitable) continue;
    failedScales++;
    console.log(JSON.stringify({ id, scale, violations: measurement.violations }));
  }
  const issues = validateAsset(asset);
  console.log(JSON.stringify({ id, scales: scales.length, failedScales, smallestGap, smallestGapAt, issues: issues.map(({ code, path }) => ({ code, path })) }));
  if (authored.validRange.minDenominator !== 20 || failedScales || issues.length) process.exitCode = 1;
}
