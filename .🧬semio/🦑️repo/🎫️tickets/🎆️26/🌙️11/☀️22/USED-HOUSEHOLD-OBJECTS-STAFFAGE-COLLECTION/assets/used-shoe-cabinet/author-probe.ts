/** 🔬 Prints the bounding box of every evaluated 2D shape and every projected body solid of one view and scale: author-probe.ts <asset json> <scale> <view> [pose] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, project } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, scaleArg, view, pose] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as any);
const box = (pts: { x: number; y: number }[]) => pts.reduce((b, p) => [Math.min(b[0], p.x), Math.min(b[1], p.y), Math.max(b[2], p.x), Math.max(b[3], p.y)], [Infinity, Infinity, -Infinity, -Infinity]).map((v) => v.toFixed(3)).join(" ");
for (const s of state.shapes[view!]) console.log("2D", s.shape.id, s.shape.bone, box(s.rings.flat()));
for (const w of worldSolids(asset, state.bones, scale)) {
  const pts: { x: number; y: number }[] = [];
  for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
  console.log("3D", w.id, box(pts));
}
