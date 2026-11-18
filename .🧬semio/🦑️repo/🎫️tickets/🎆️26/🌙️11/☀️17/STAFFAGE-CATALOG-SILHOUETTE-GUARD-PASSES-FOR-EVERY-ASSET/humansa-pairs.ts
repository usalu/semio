/** 🔗 Local points and their evaluated world points of a plain poly shape. bun humansa-pairs.ts <asset> <scale> <view> <shapeId> */
import { applyOps, loadAsset, shapesOf, stateOf, type Viewpoint } from "./humansa-lib.ts";
const [name, sc, view, id] = process.argv.slice(2) as [string, string, Viewpoint, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const s = shapesOf(asset, Number(sc), view).find((x) => x.id === id);
const ev = (stateOf(asset, Number(sc)).shapes as any)[view].find((x: any) => x.shape.id === id);
const ring = ev.rings[0] as { x: number; y: number }[];
console.log("points", s.points.length, "ring", ring.length);
s.points.forEach((p: number[], i: number) => console.log(i, JSON.stringify(p), ring[i] ? `-> (${ring[i]!.x.toFixed(3)}, ${ring[i]!.y.toFixed(3)})` : ""));
