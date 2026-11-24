/** 🧲️ What is drawn and what stands near a point of one view: the evaluated shapes of the representation (id, bone, role, filled or stroke with its pen in metres, bounding box in view metres) and the body solids of the volume (id, kind, bone, projected bounding box) whose boxes come within `radius` of the point.
 *  bun near.ts <slug|file.json> <scale> <view> <x> <y> [radius] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { boundsOf, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, view, xs, ys, rs] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), [x, y, r] = [Number(xs), Number(ys), Number(rs ?? 0.02)];
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const rep = asset.representations[String(scale)];
const hit = (b: readonly [number, number, number, number]): boolean => x > b[0] - r && x < b[1] + r && y > b[2] - r && y < b[3] + r;
const f = (n: number): string => n.toFixed(4);
for (const s of state.shapes[view] ?? []) {
  const b = boundsOf(s.rings);
  if (!hit(b)) continue;
  const pen = ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
  console.log(`  shape ${s.shape.id} [${s.shape.kind ?? ""}] bone ${s.shape.bone ?? "-"} role ${s.shape.role} ${s.closed ? (s.shape.filled === false ? "outline" : "filled") : "open"} pen ${f(pen)} x ${f(b[0])}..${f(b[1])} y ${f(b[2])}..${f(b[3])}`);
}
for (const w of worldSolids(asset, state.bones as never, scale)) {
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) {
    const p = project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!);
    x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y);
  }
  if (hit([x0, x1, y0, y1])) console.log(`  solid ${w.id} bone ${w.bone ?? "-"}${w.texture ? " texture" : ""}${w.slender ? " slender" : ""} x ${f(x0)}..${f(x1)} y ${f(y0)}..${f(y1)}`);
}
