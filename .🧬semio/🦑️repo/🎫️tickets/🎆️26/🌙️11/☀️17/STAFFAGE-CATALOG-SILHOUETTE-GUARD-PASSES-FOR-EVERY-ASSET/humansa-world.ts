/** 🌍 Replaces a front or plan shape with a plain polygon in world metres of the contour of volume solids, on a (static) bone, dropping its `solid` hint.
 *  bun humansa-world.ts <asset> <scale,scale> <view> <shapeId> <solidId,...|prefix*> <bone> <ops.json> [tolFeatures] */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { applyOps, loadAsset, shapesOf, worldContour, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, shapeId, solidArg, bone, outFile, tolArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string | undefined];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) {
  const original = shapesOf(asset, scale, view).find((s) => s.id === shapeId);
  const pts = worldContour(asset, scale, view, solidArg.split(","), Number(tolArg ?? 0.12));
  if (original === undefined || pts === undefined) { console.log(scale, "skipped"); continue; }
  const shape: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(original)) {
    if (k === "kind") { shape.kind = "poly"; shape.points = pts.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]); shape.closed = true; }
    else if (["d", "points", "closed", "followTip", "rx", "ry", "cx", "cy", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "rotationDeg", "solid"].includes(k)) continue;
    else shape[k] = k === "bone" ? bone : v;
  }
  ops.push({ op: "shape", scale: String(scale), view, id: shapeId, shape });
  console.log(scale, shapeId, "points", pts.length);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
