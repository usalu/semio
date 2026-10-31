import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const only = process.env.SCALES?.split(",").map(Number);
for (const file of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  console.log("== " + asset.id);
  for (const i of validateAsset(asset, {})) console.log("  VALID", i.code, i.path, i.message.en);
  for (const d of [500, 200, 100, 50]) {
    if (only && !only.includes(d)) continue;
    const rep = asset.representations[String(d)];
    if (rep.status !== "authored") continue;
    const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    console.log(`  1:${d} ${s.suitable ? "ok" : "BAD"} stroke ${s.minStrokeMm.toFixed(3)} gap ${s.minGapMm.toFixed(3)} feature ${s.minFeatureMm.toFixed(3)}`);
    for (const v of s.violations) console.log(`    ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}${(v as any).at ? " at " + (v as any).at.xMm.toFixed(2) + "," + (v as any).at.yMm.toFixed(2) : ""}`);
  }
}
