/** 👯️ Drawn shapes and body solids of one view that carry the same id, with the differences of their bounding boxes in millimetres (left, right, bottom, top; drawing minus volume): finds the member a drawing places differently from its solid.
 *  bun pairs.ts <id|file.json> <scale> <view> [min-mm] */
import { S, fileOf } from "./lib.ts";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { worldSolids, project } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const [name, scaleArg, view, least = "2"] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const boxes = new Map<string, number[]>();
for (const w of worldSolids(asset, state.bones, scale)) {
  let b = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) {
    const p = project(view, w.positions[i], w.positions[i + 1], w.positions[i + 2]);
    b = [Math.min(b[0]!, p.x), Math.max(b[1]!, p.x), Math.min(b[2]!, p.y), Math.max(b[3]!, p.y)];
  }
  boxes.set(w.id, b);
}
let pairs = 0;
for (const s of state.shapes[view!]) {
  const v = boxes.get(s.shape.id);
  const pts = s.rings.flat();
  if (v === undefined || pts.length === 0) continue;
  pairs++;
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  const d = [Math.min(...xs) - v[0]!, Math.max(...xs) - v[1]!, Math.min(...ys) - v[2]!, Math.max(...ys) - v[3]!].map((x) => x * 1000);
  if (Math.max(...d.map(Math.abs)) >= Number(least)) console.log(`${s.shape.id} ${s.shape.kind} d ${d.map((x) => x.toFixed(1)).join(" ")} mm; volume x ${v[0]!.toFixed(4)}..${v[1]!.toFixed(4)} y ${v[2]!.toFixed(4)}..${v[3]!.toFixed(4)}`);
}
console.log(`${pairs} pairs of the same id`);
