/** 🩹 Applies one clip or grow patch of a plain shape at a cluster (iterated fit) and appends it as an op; env OPS applies ops first.
 *  bun humansa-applypatch.ts <asset> <scale> <view> <shapeId[,twin]> <x> <y> <d> <clip|grow> <radius> <marginFeatures> <ops.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { applyOps, applyShape, clustersOf, loadAsset, patchShape, type Viewpoint } from "./humansa-lib.ts";
const [name, sc, view, ids, xs, ys, ds, side, rad, mg, outFile] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, "clip" | "grow", string, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const scale = Number(sc);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
const c = { side: side === "clip" ? "drawn-only" as const : "volume-only" as const, x: Number(xs), y: Number(ys), d: Number(ds) };
const before = clustersOf(asset, scale, view);
for (const id of ids.split(",")) {
  const shape = patchShape(asset, scale, view, id, c, Number(rad), Number(mg), 0.12, side, true);
  if (shape === undefined) { console.log(id, "no patch"); continue; }
  applyShape(asset, scale, view, shape);
  ops.push({ op: "shape", scale: String(scale), view, id, shape });
}
const after = clustersOf(asset, scale, view);
console.log(`h ${(before.h * 1000).toFixed(1)} -> ${(after.h * 1000).toFixed(1)} excess ${before.excess.toFixed(2)} -> ${after.excess.toFixed(2)}`);
writeFileSync(outFile, JSON.stringify(ops, null, 1));
