/** 🧩️ The reference polygons of the volume (projected triangles and the stroke quads of slender solids) of one view that touch a box around a point, with their corner coordinates: shows what a slender solid is compared as.
 *  bun refs.ts <id|file.json> <scale> <view> <x> <y> <r> [max-corners] */
import { S, fileOf } from "./lib.ts";
const { measureView, worldSolids } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const [name, scaleArg, view, xs, ys, rs, most = "4"] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, featureOf(scale), Infinity, false);
const [x, y, r] = [Number(xs), Number(ys), Number(rs)];
for (const g of m.reference) {
  const pts = g.flat();
  if (pts.length !== Number(most) && Number(most) !== 0) continue;
  if (pts.some((p: any) => Math.abs(p.x - x) <= r && Math.abs(p.y - y) <= r)) console.log(pts.map((p: any) => `(${p.x.toFixed(4)}, ${p.y.toFixed(4)})`).join(" "));
}
