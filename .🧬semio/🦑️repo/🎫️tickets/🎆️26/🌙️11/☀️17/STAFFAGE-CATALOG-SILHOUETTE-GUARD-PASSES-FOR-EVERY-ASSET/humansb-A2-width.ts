/** 📏 Lateral extent per height level of chosen volume solids (and optionally of a drawn shape) in one view. bun humansb-A2-width.ts <asset> <scale> <view> <solidIds,..> <y0> <y1> <step> [shapeId,..] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, ids, a, b, st, shapeId] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string | undefined];
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (process.env.PATCH) {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const want = new Set(ids.split(","));
const segs: [number, number, number, number][] = [];
const sw = (p: { x: number; y: number }) => (process.env.SWAP ? { x: p.y, y: p.x } : p);
for (const w of worldSolids(asset, state.bones as never, scale)) if (want.has(w.id)) for (let k = 0; k < w.indices.length; k += 3) {
  const p = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return sw(project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!)); });
  for (let j = 0; j < 3; j++) segs.push([p[j]!.x, p[j]!.y, p[(j + 1) % 3]!.x, p[(j + 1) % 3]!.y]);
}
const ringList = shapeId ? shapeId.split(",").flatMap((id) => (state.shapes as any)[view].find((x: any) => x.shape.id === id).rings as { x: number; y: number }[][]) : [];
const ring = ringList.length ? ringList : undefined;
const ext = (list: [number, number, number, number][], y: number) => {
  let lo = Infinity, hi = -Infinity;
  for (const [x0, y0, x1, y1] of list) if ((y0 - y) * (y1 - y) <= 0 && y0 !== y1) { const x = x0 + ((y - y0) / (y1 - y0)) * (x1 - x0); lo = Math.min(lo, x); hi = Math.max(hi, x); }
  return [lo, hi];
};
const rs: [number, number, number, number][] = [];
for (const r of ringList) for (let i = 0; i < r.length; i++) { const p = sw(r[i]!), q = sw(r[(i + 1) % r.length]!); rs.push([p.x, p.y, q.x, q.y]); }
for (let y = Number(a); y <= Number(b) + 1e-9; y += Number(st)) {
  const [l, h] = ext(segs, y);
  const d = ring ? ext(rs, y) : [NaN, NaN];
  console.log(y.toFixed(3), "vol", l.toFixed(4), h.toFixed(4), ring ? `drawn ${d[0]!.toFixed(4)} ${d[1]!.toFixed(4)}` : "");
}
