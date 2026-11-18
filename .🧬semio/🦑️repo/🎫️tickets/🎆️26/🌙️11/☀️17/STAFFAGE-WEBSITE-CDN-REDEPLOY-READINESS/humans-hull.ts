/** 🔶 Convex hull (view coordinates) of the projection of solids, and the evaluated rings of one drawn shape. bun humans-hull.ts <asset> <scale> <view> <shapeId|-> <solidId,...> [maxPoints] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, shapeId, ids, mp] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined];
const scale = Number(sc);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
if (shapeId !== "-") { const s = (state.shapes as any)[view].find((x: any) => x.shape.id === shapeId); const r = s.rings[0]; console.log(JSON.stringify(r.map((p: any) => [+p.x.toFixed(4), +p.y.toFixed(4)])).slice(0, 400)); console.log("ring n", r.length, "bbox", Math.min(...r.map((p: any) => p.x)), Math.max(...r.map((p: any) => p.x)), Math.min(...r.map((p: any) => p.y)), Math.max(...r.map((p: any) => p.y)), "first", r[0], r[Math.floor(r.length / 4)]); }
const want = new Set(ids.split(","));
const pts: { x: number; y: number }[] = [];
for (const w of worldSolids(asset, state.bones as never, scale)) if (want.has(w.id)) for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
pts.sort((a, b) => a.x - b.x || a.y - b.y);
const cross = (o: any, a: any, b: any) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
const half = (list: any[]) => { const h: any[] = []; for (const p of list) { while (h.length >= 2 && cross(h[h.length - 2], h[h.length - 1], p) <= 0) h.pop(); h.push(p); } h.pop(); return h; };
const hull = [...half(pts), ...half([...pts].reverse())];
console.log("hull n", hull.length, "bbox", Math.min(...hull.map((p) => p.x)).toFixed(4), Math.max(...hull.map((p) => p.x)).toFixed(4), Math.min(...hull.map((p) => p.y)).toFixed(4), Math.max(...hull.map((p) => p.y)).toFixed(4));
console.log(JSON.stringify(hull.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)])));
