/** 📐 Prints the print-suitability violations of one band of an asset document at one denominator, per pose and viewpoint: `bun r5-courier-with-hand-truck-ends.ts <asset document json> <band> <denominator>`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const text = readFileSync(process.argv[2]!, "utf8");
const [band, end] = [process.argv[3]!, Number(process.argv[4])];
for (const pose of JSON.parse(text).asset.poses)
  for (const view of ["elevation", "front", "plan"]) {
    const asset = JSON.parse(text).asset;
    asset.poses = asset.poses.filter((p: any) => p.id === pose.id);
    for (const other of ["elevation", "front", "plan"]) if (other !== view) asset.representations[band].geometry[other] = [];
    const s = measureSuitability(asset, asset.representations[band], end, DEFAULT_PRINT_PROFILE) as any;
    if (s.violations.length > 0) console.log(band, end, pose.id, view, JSON.stringify(s.violations).slice(0, 300));
  }
