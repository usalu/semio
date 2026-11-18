/** ➕ Adds an extruded solid shape to a view after another shape: the outline of volume solids seen in the elevation (in bone coordinates) swept laterally z0..z1, so the plan or front shows it by the 3D frame of the bone.
 *  bun humansa-addextruded.ts <asset> <scale,scale> <view> <after> <newId> <bone> <solidId,...> <z0> <z1> <ops.json> */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { applyOps, boneLocal, contourOf, loadAsset, shapesOf, type Viewpoint } from "./humansa-lib.ts";
const [name, scalesArg, view, after, newId, bone, solidArg, z0, z1, outFile] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string, string, string];
const asset = await loadAsset(name);
applyOps(asset, process.env.OPS);
const ops: any[] = existsSync(outFile) ? JSON.parse(readFileSync(outFile, "utf8")) : [];
for (const scale of scalesArg.split(",").map(Number)) {
  const base = shapesOf(asset, scale, view).find((s) => s.id === after);
  const world = contourOf(asset, scale, "elevation", solidArg.split(","));
  if (base === undefined || world === undefined) { console.log(scale, "skipped"); continue; }
  const local = boneLocal(asset, scale, "elevation", bone);
  const points = world.map((p) => local(p).map((v) => +v.toFixed(4)));
  const shape = { kind: "poly", points, closed: true, id: newId, bone, role: "body", z: base.z + 0.01, filled: true, tone: base.tone, ...(base.region === undefined ? {} : { region: base.region }), ...(base.material === undefined ? {} : { material: base.material }), solid: { kind: "extruded", sweep: "w", z0: Number(z0), z1: Number(z1) } };
  ops.push({ op: "shape-add", scale: String(scale), view, after, shape });
  console.log(scale, newId, points.length);
}
writeFileSync(outFile, JSON.stringify(ops, null, 1));
