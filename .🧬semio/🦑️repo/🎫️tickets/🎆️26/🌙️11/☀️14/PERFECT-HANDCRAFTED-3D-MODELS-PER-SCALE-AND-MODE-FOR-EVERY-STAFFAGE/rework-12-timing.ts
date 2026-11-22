/** ⏱️ Author 12: times the parts of the whole-asset validation of a candidate document, to see what a large tree costs: `bun rework-12-timing.ts <asset document>`. */
import { readFileSync } from "node:fs";
import { validateAsset, validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { analyticRules } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const time = (name: string, run: () => unknown): void => {
  const start = performance.now();
  let note = "";
  try {
    const out = run();
    note = Array.isArray(out) ? `${out.length} items` : "";
  } catch (error) {
    note = `threw ${(error as Error).message.slice(0, 200)}`;
  }
  process.stdout.write(`[DEBUG] ${name}: ${((performance.now() - start) / 1000).toFixed(1)} s ${note}\n`);
};
time("analyticRules", () => analyticRules(asset as never));
time("validateRanges", () => validateRanges(asset as never, DEFAULT_PRINT_PROFILE));
for (const scale of ["50", "100", "200", "500"]) time(`measureSuitability ${scale}`, () => measureSuitability(asset as never, asset.representations[scale] as never, Number(scale), DEFAULT_PRINT_PROFILE));
time("validateAsset", () => validateAsset(asset as never, { assets: [asset] as never, environments: [] as never }));
