/** 🎯 Which drawn shapes (after an optional ops patch) and volume solids cover points along a horizontal line of a view. bun humansb-D-probe.ts <asset> <scale> <view> <ops.json|-> <y> <x0> <x1> <step> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, patch, ys, x0s, x1s, steps] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (patch !== "-") for (const o of JSON.parse(await Bun.file(patch!).text())) { const l = (asset as any).representations[o.scale].geometry[o.view]; if (o.op === "shape-add") l.splice(l.findIndex((x: any) => x.id === o.after) + 1, 0, o.shape); else if (o.op === "shape") l[l.findIndex((x: any) => x.id === o.id)] = o.shape; else if (o.op === "shape-set") Object.assign(l.find((x: any) => x.id === o.id), o.set); }
const scale = Number(sc);
const st: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, st.bones as never, scale).filter((w) => w.texture !== true);
const tris = solids.map((w) => { const t: any[] = []; for (let k = 0; k < w.indices.length; k += 3) t.push([0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); })); return { id: w.id, t }; });
const inTri = (P: any, [a, b, c]: any[]) => { const s = (p: any, q: any, r: any) => (p.x - r.x) * (q.y - r.y) - (q.x - r.x) * (p.y - r.y); const d1 = s(P, a, b), d2 = s(P, b, c), d3 = s(P, c, a); return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0)); };
const inside = (P: any, ring: any[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i].y > P.y) !== (ring[j].y > P.y) && P.x < ((ring[j].x - ring[i].x) * (P.y - ring[i].y)) / (ring[j].y - ring[i].y) + ring[i].x) c = !c; return c; };
const y = Number(ys);
for (let x = Number(x0s); x <= Number(x1s) + 1e-9; x += Number(steps)) {
  const P = { x, y };
  const d = st.shapes[view].filter((s: any) => s.shape.filled !== false && s.rings.some((r: any) => inside(P, r))).map((s: any) => s.shape.id);
  const v = tris.filter((s) => s.t.some((t: any) => inTri(P, t))).map((s) => s.id);
  console.log(x.toFixed(3), "D:", d.join(","), "| V:", v.join(","));
}
