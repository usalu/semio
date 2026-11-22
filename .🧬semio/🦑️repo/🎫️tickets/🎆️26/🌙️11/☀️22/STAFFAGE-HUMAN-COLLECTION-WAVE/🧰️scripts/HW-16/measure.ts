/** 📏️ Measures the silhouette agreement (bbox, IoU, Hausdorff per view) of human assets at the given scales, the numbers the catalog-silhouette guard gates.
 *  bun measure.ts <slug|file.json> [scales=50,100,200,500] */
import { join, resolve } from "node:path";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [target, scalesArg] = process.argv.slice(2);
const file = target!.endsWith(".json") ? target! : join(catalog, target!, "🔣️.json");
const asset = (await Bun.file(file).json()).asset;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
for (const scale of (scalesArg ?? "50,100,200,500").split(",").map(Number)) {
  const t0 = performance.now();
  const feature = featureOf(scale as never);
  const m = measureAsset(asset, scale, process.env.POSE ?? undefined, undefined, feature);
  const bb = m.bbox.map((b, k) => `${b.toFixed(3)}(${(b - dims[k]!) >= 0 ? "+" : ""}${((b - dims[k]!) * 1000).toFixed(0)}mm)`).join(" ");
  const views = (["elevation", "front", "plan"] as const).map((v) => `${v} iou ${m.views[v].iou.toFixed(3)} H ${(m.views[v].hausdorffM * 1000).toFixed(0)}mm p ${m.views[v].precision.toFixed(3)} r ${m.views[v].recall.toFixed(3)}`).join(" | ");
  console.log(`1:${scale} feat ${(feature * 1000).toFixed(1)}mm bbox ${bb}\n   ${views}  (${Math.round(performance.now() - t0)} ms)`);
}
