/** 🔬 Which pose and viewpoint of a candidate document carries the thinnest visible feature of a named shape of one representation, measured at any denominator of its range (the product's own print measure, one pose and one viewpoint at a time).
 * Usage: `bun r5-vehicles-print-probe.ts <document json> <scale> <denominator> <shape id>`
 * @see ./r5-runners-print-probe.ts */
import { readFileSync } from "node:fs";

import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, at, id] = process.argv.slice(2) as [string, string, string, string];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const rep = asset.representations[scale];
for (const pose of asset.poses.filter((p: { id: string }) => p.id === (process.env.POSE ?? p.id))) {
  for (const view of ["elevation", "front", "plan"]) {
    const geometry = Object.fromEntries(Object.entries(rep.geometry).map(([k, v]) => [k, k === view ? v : []]));
    const one = { ...rep, geometry };
    const s = measureSuitability({ ...asset, poses: [pose], representations: { ...asset.representations, [scale]: one } }, one, Number(at), DEFAULT_PRINT_PROFILE);
    const v = s.violations.filter((x: { shapes: string[] }) => x.shapes.includes(id));
    if (v.length > 0) console.log(pose.id, view, JSON.stringify(v));
  }
}
