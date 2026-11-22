/** 🌓️ Hausdorff and IoU of one scale and view for every pose, optionally without the shapes and solids of some bones: bun sil1.ts <slug> <scale> <view> [bone,bone] */
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { readAsset } from "./lib.ts";
const [src, sc, view, drop] = process.argv.slice(2);
const asset = await readAsset(src!);
const gone = new Set((drop ?? "").split(",").filter(Boolean));
if (gone.size) {
  for (const rep of Object.values(asset.representations) as any[]) for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: any) => !gone.has(s.bone) && !gone.has(s.id));
  for (const vol of Object.values(asset.volumes) as any[]) for (const l of ["body", "thermal", "structure"]) vol[l] = vol[l].filter((s: any) => !gone.has(s.bone) && !gone.has(s.id) && !gone.has(s.of));
}
for (const p of asset.poses) {
  const m = measureAsset(asset, Number(sc), p.id, undefined, featureOf(Number(sc) as 50));
  console.log(p.id.padEnd(22), `iou ${m.views[view as "elevation"].iou.toFixed(3)} h ${m.views[view as "elevation"].hausdorffM.toFixed(4)}`);
}
