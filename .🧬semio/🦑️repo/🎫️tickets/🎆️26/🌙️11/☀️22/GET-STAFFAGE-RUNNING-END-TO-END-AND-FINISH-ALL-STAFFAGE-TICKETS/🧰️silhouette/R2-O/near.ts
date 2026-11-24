/** 🔭️ The body solids and the drawn shapes of one view whose projected box meets a window, each with that box, optional ops applied first.
 *  bun near.ts <id|file.json> <scale> <view> <x> <y> <radius> [ops.json|-] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";
const [name, sc, view, xs, ys, rs, opsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const [x, y, r] = [Number(xs), Number(ys), Number(rs)];
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const f = (n: number): string => n.toFixed(4);
const hit = (b: number[]): boolean => b[0]! <= x + r && b[1]! >= x - r && b[2]! <= y + r && b[3]! >= y - r;
for (const w of worldSolids(asset, state.bones as never, Number(sc)) as any[]) {
  const b = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]); b[0] = Math.min(b[0]!, p.x); b[1] = Math.max(b[1]!, p.x); b[2] = Math.min(b[2]!, p.y); b[3] = Math.max(b[3]!, p.y); }
  if (hit(b)) console.log(`vol   ${w.id}[${w.bone}]${w.slender ? "~" : ""}${w.texture ? " texture" : ""} x ${f(b[0]!)} ${f(b[1]!)} y ${f(b[2]!)} ${f(b[3]!)}`);
}
for (const s of state.shapes[view!]) {
  const pts = s.rings.flat();
  if (pts.length === 0) continue;
  const b = [Math.min(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.y))];
  if (hit(b)) console.log(`drawn ${s.shape.id}[${s.shape.bone}] ${s.shape.role} ${s.closed ? (s.shape.filled === false ? "outline" : "filled") : "stroke"} pen ${s.shape.penMm ?? "-"} x ${f(b[0]!)} ${f(b[1]!)} y ${f(b[2]!)} ${f(b[3]!)}`);
}
