/** 🧩️ Which parts of an asset carry a low agreement: per key the IoU and the cell counts (drawn, volume, both) of one view when only the body solids and the drawn shapes whose id contains the key are kept (the same raster and reference as the guard, on a copy of the asset).
 *  bun parts.ts <slug|file.json> <scale> <view> <key,key…> */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, view, keys] = process.argv.slice(2) as string[];
const text = await Bun.file(fileOf(name!)).text();
for (const key of keys!.split(",")) {
  const asset = JSON.parse(text).asset;
  const scale = Number(scaleArg);
  asset.volumes[scaleArg!].body = asset.volumes[scaleArg!].body.filter((s: any) => s.id.includes(key));
  for (const v of ["elevation", "front", "plan"]) asset.representations[scaleArg!].geometry[v] = asset.representations[scaleArg!].geometry[v].filter((s: any) => s.id.includes(key));
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view as never, scale, undefined, featureOf(scale));
  console.log(`${key}: iou ${m.iou.toFixed(3)} drawn ${m.counts.drawn} volume ${m.counts.volume} both ${m.counts.both} H ${(m.hausdorffM * 1000).toFixed(1)} mm`);
}
