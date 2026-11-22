/** 🖨️ Print suitability per named pose (each pose alone) at the given scales: bun suit.ts <file.json> [scale…] */
import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [file, ...sc] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
for (const d of sc.length ? sc.map(Number) : [500, 200, 100, 50]) for (const p of asset.poses) {
  const one = { ...asset, poses: [p], clips: [], presentation: { ...asset.presentation, defaultPose: p.id } };
  const s = measureSuitability(one, asset.representations[String(d)], d, DEFAULT_PRINT_PROFILE);
  if (!s.suitable) console.log(`1:${d} ${p.id}: ${s.violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} at ${JSON.stringify(v.at)} ${v.view ?? ""}`).join("; ")}`);
}
