/** ➕ Adds a plain polygon shape after another shape of a view, given in world metres of the view, fitted to the bone placement. Template: the shape `after` (role, z, tone, region, material are copied; id, bone given).
 *  bun humansa-addworld.ts <asset> <scale,scale> <view> <after> <newId> <bone> '<[[x,y],...]>' <ops.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { addWorldShape, applyOps, loadAsset, shapesOf, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, after, newId, bone, polyArg, outFile] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
const target = (JSON.parse(polyArg) as [number, number][]).map(([x, y]) => ({ x, y }));
for (const scale of scalesArg.split(",").map(Number)) {
  const base = shapesOf(asset, scale, view).find((s) => s.id === after);
  if (base === undefined) { console.log(scale, "no template"); continue; }
  const template: Record<string, unknown> = { id: newId, bone, role: "body", z: base.z + 0.01, filled: true, tone: base.tone, ...(base.region === undefined ? {} : { region: base.region }), ...(base.material === undefined ? {} : { material: base.material }) };
  const shape = addWorldShape(asset, scale, view, after, template, target);
  if (shape === undefined) { console.log(scale, "fit failed"); continue; }
  ops.push({ op: "shape-add", scale: String(scale), view, after, shape });
  console.log(scale, newId, JSON.stringify(shape.points));
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
