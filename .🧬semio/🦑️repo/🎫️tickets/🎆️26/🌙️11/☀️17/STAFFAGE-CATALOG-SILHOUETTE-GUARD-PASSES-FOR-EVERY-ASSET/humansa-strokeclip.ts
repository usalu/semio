/** ✂️ Clips unfilled stroke shapes to the volume. bun humansa-strokeclip.ts <asset> <scale,scale> <view> <shapeId,shapeId> <ops.json> [erodeFeatures=0] */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { applyOps, clipStroke, loadAsset, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, ids, outFile, er] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string | undefined];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) for (const id of ids.split(",")) {
  const shape = clipStroke(asset, scale, view, id, Number(er ?? 0));
  if (shape === undefined) { console.log(scale, id, "unchanged"); continue; }
  ops.push({ op: "shape", scale: String(scale), view, id, shape });
  console.log(scale, id, shape.points ?? shape.d);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
