/** 🔎 Which pose and viewpoint of a candidate document carries the thinnest visible feature of a named shape at one scale (the product's own print measure, one pose at a time).
 * Usage: `bun r5-runners-print-probe.ts <document json> <scale> <shape id> [pose|-] [regex of shape ids left out]`
 * @see ./r3-validate.ts */
import { readFileSync } from "node:fs";

import { measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, id] = process.argv.slice(2) as [string, string, string];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const rep = asset.representations[scale];
const profile = { minStrokeMm: 0.15, minGapMm: 0, minFeatureMm: 0.2, minHatchSpacingMm: 0 };
const only = process.argv[5] === "-" ? undefined : process.argv[5];
const drop = process.argv[6] === undefined ? null : new RegExp(process.argv[6]);
for (const pose of asset.poses.filter((p: { id: string }) => only === undefined || p.id === only)) {
  for (const view of ["elevation", "front", "plan"]) {
    const geometry = Object.fromEntries(Object.entries(rep.geometry).map(([k, v]) => [k, k === view ? (v as { id: string }[]).filter((x) => drop === null || !drop.test(x.id)) : []]));
    const one = { ...rep, geometry };
    const s = measureSuitability({ ...asset, poses: [pose], representations: { ...asset.representations, [scale]: one } }, one, Number(scale), profile as never);
    const v = s.violations.filter((x: { shapes: string[] }) => x.shapes.includes(id));
    if (v.length > 0) console.log(pose.id, view, JSON.stringify(v));
  }
}
