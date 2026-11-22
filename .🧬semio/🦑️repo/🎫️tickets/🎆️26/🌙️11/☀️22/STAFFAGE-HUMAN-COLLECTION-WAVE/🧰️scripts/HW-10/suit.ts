/** 🖨️ Print suitability per scale with the violating shapes and their paper positions: bun suit.ts <file> [limit] */
import { DEFAULT_PRINT_PROFILE, loadAsset, measureSuitability, printProfile } from "./lib.ts";
const [file, limit] = process.argv.slice(2);
const asset = await loadAsset(file!);
for (const d of [500, 200, 100, 50]) {
  const rep = (asset.representations as any)[String(d)];
  const s = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
  console.log(`1:${d} suitable=${s.suitable} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
  for (const v of s.violations.slice(0, Number(limit ?? 8))) console.log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
}
