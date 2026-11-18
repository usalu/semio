/** ➕ Adds a plain shape after another shape of a view, outlined by the projection of volume solids and fitted to a bone. Template fields from `after`.
 *  bun humansa-addsolid.ts <asset> <scale,scale> <view> <after> <newId> <bone> <solidId,...|prefix*> <ops.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { addWorldShape, applyOps, loadAsset, shapesOf, worldContour, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, after, newId, bone, solidArg, outFile] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) {
  const base = shapesOf(asset, scale, view).find((s) => s.id === after);
  const target = worldContour(asset, scale, view, solidArg.split(","));
  if (base === undefined || target === undefined) { console.log(scale, "skipped"); continue; }
  const template: Record<string, unknown> = { id: newId, bone, role: "body", z: base.z + 0.01, filled: true, tone: base.tone, ...(base.region === undefined ? {} : { region: base.region }), ...(base.material === undefined ? {} : { material: base.material }) };
  const shape = addWorldShape(asset, scale, view, after, template, target);
  if (shape === undefined) { console.log(scale, "fit failed"); continue; }
  ops.push({ op: "shape-add", scale: String(scale), view, after, shape });
  console.log(scale, newId, shape.points.length);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
