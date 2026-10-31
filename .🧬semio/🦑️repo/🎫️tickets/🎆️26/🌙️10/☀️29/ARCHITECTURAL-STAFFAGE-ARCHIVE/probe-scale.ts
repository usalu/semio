/** 🖨️ Prints how the default print profile resolves a few assets at 1:100 and at their authored default scale. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation, type AssetDocument } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const ids = ["walking-commuter", "labrador-dog", "date-palm", "diesel-generator", "cargo-bike", "dining-table", "english-oak", "city-bicycle"];
const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const { readdirSync } = await import("node:fs");
const { join } = await import("node:path");

function find(id: string): string | undefined {
  for (const category of readdirSync(root, { withFileTypes: true })) {
    if (!category.isDirectory()) continue;
    const dir = join(root, category.name, id);
    try {
      const file = readdirSync(dir).find((name) => name.endsWith(".json"));
      if (file !== undefined) return join(dir, file);
    } catch { /* absent */ }
  }
  return undefined;
}

for (const id of ids) {
  const path = find(id);
  if (path === undefined) { console.log(`MISS ${id}`); continue; }
  const asset = (JSON.parse(readFileSync(path, "utf8")) as AssetDocument).asset;
  const scales = [100, asset.presentation.defaultScale].filter((scale, index, all) => scale !== undefined && all.indexOf(scale) === index);
  for (const denominator of scales) {
    const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
    const first = resolution.kind === "unsuitable" ? resolution.suitability.violations[0] : undefined;
    console.log(`${id} 1:${denominator} default=${asset.presentation.defaultScale} ${resolution.kind}${first === undefined ? "" : ` ${first.check} ${first.measuredMm.toFixed(3)}<${first.requiredMm} ${first.shapes.join("+")}`}`);
  }
}
