/** ✏️ Redraws one shape of a view as the outline of the projected volume solids, in the coordinates of the shape's bone, as an ops file for humans-ops.py and humans-try.ts.
 *  bun humansa-redraw.ts <asset> <scale,scale> <view> <shapeId> <solidId,solidId|*> <ops.json> [contour|hull] [tolFeatures] [keepExpr in x,y of the view, e.g. "y>0"] */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { loadAsset, redrawShape, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, shapeId, solidArg, outFile, modeArg, tolArg, keepArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string | undefined, string | undefined, string | undefined];
const keep = keepArg === undefined ? undefined : (new Function("x", "y", `return ${keepArg};`) as (x: number, y: number) => boolean);
const asset = await loadAsset(name);
const ops: unknown[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) {
  const shape = redrawShape(asset, scale, view, shapeId, solidArg.split(","), modeArg ?? "contour", Number(tolArg ?? 0.12), keep);
  if (shape === undefined) { console.log(scale, "skipped", shapeId); continue; }
  ops.push({ op: "shape", scale: String(scale), view, id: shapeId, shape });
  console.log(scale, shapeId, "points", (shape.points ?? shape.d.split(" L ")).length);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
