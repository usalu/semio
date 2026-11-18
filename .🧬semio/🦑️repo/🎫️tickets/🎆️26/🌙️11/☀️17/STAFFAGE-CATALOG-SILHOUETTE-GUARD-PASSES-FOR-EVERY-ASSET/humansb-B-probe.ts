/** 🧪 Response of one vertex of one shape to rest-frame steps. bun humansb-B-probe.ts <asset> <scale> <view> <patch> <shapeId> <vertex> */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, sc, view, patch, id, vs] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const find = (list: any[], i: string) => list.find((x) => x.id === i);
for (const o of JSON.parse(readFileSync(patch!, "utf8"))) {
  if (!o.op.startsWith("shape")) continue;
  const list: any[] = (asset as any).representations[o.scale].geometry[o.view];
  if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
  else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
  else list[list.indexOf(find(list, o.id))] = o.shape;
}
const shape = find((asset as any).representations[sc!].geometry[view!], id!);
const k = Number(vs);
const run = (dx: number, dy: number) => {
  const a = structuredClone(asset) as any;
  const s = find(a.representations[sc!].geometry[view!], id!);
  s.points = s.points.map((p: number[], i: number) => (i === k ? [p[0]! + dx, p[1]! + dy] : p));
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never) as any;
  return st.shapes[view!].find((x: any) => x.shape.id === id).rings[0][k];
};
console.log("shape", JSON.stringify(shape.points[k]), "bone", shape.bone);
for (const [dx, dy] of [[0, 0], [0.001, 0], [0, 0.001], [0, 0.005], [0, 0.02], [0, -0.02], [0.02, 0]]) { const p = run(dx!, dy!); console.log(dx, dy, "->", p.x.toFixed(4), p.y.toFixed(4)); }
