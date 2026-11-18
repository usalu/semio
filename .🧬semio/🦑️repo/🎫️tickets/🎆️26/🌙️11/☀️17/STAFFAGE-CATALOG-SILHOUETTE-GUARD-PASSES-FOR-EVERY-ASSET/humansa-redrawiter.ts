/** ✏️ Redraw of a plain shape whose placement is not affine (tilted bones, depth profiles) by fixed-point iteration of the polygon in bone coordinates.
 *  bun humansa-redrawiter.ts <asset> <scale,scale> <view> <shapeId> <solidId,...|prefix*> <ops.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { applyOps, loadAsset, redrawIter, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, shapeId, solidArg, outFile] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) {
  const shape = redrawIter(asset, scale, view, shapeId, solidArg.split(","), 8, 0.12, process.env.DROPSOLID === "1");
  if (shape === undefined) { console.log(scale, shapeId, "skipped"); continue; }
  ops.push({ op: "shape", scale: String(scale), view, id: shapeId, shape });
  console.log(scale, shapeId, shape.points.length);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
