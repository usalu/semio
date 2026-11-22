/** 🧪️ Experiment: plan/front/elevation IoU and Hausdorff of an asset with shapes and solids of given ids or bones removed: bun exp.ts <slug> <scale> <view> <comma ids-or-bones> */
import { join } from "node:path";
import { S, CATALOG } from "./lib.ts";
const { measureAsset } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const [slug, scale, view, drop] = process.argv.slice(2);
const base = (await Bun.file(join(CATALOG, "👤️humans", slug!, "🔣️.json")).json()).asset;
for (const group of [""].concat((drop ?? "").split(";").filter(Boolean))) {
  const a = structuredClone(base);
  const ids = new Set(group.split(",").filter(Boolean));
  const hit = (s: any) => ids.has(s.id) || ids.has(s.bone) || ids.has(s.of);
  for (const v of ["elevation", "front", "plan"]) a.representations[scale!].geometry[v] = a.representations[scale!].geometry[v].filter((s: any) => !hit(s));
  for (const k of ["body", "thermal", "structure"]) a.volumes[scale!][k] = a.volumes[scale!][k].filter((s: any) => !hit(s));
  const m = measureAsset(a, Number(scale), undefined, undefined, featureOf(Number(scale))).views[view!];
  console.log(`${group || "(none)"}\tiou ${m.iou.toFixed(4)} h ${(m.hausdorffM * 1000).toFixed(0)} p ${m.precision.toFixed(3)} r ${m.recall.toFixed(3)}`);
}
