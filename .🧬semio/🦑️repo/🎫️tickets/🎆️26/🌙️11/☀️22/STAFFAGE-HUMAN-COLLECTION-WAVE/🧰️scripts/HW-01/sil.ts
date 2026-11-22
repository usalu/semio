/** 🌓️ IoU and Hausdorff of drawing against volume per scale and view (the catalog-silhouette guard), for an asset JSON file; with a view
 *  and scale also the worst boundary points.  bun sil.ts <asset.json> [scale view] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [path, onlyScale] = process.argv.slice(2);
const asset = (await Bun.file(path!).json()).asset;
void evaluate;
for (const scale of [50, 100, 200, 500]) {
  if (onlyScale && Number(onlyScale) !== scale) continue;
  const feature = featureOf(scale as never);
  const m = measureAsset(asset, scale as never, undefined, undefined, feature);
  const line = VIEWPOINTS.map((v) => `${v} iou ${m.views[v].iou.toFixed(3)} haus ${(m.views[v].hausdorffM * 1000).toFixed(0)}mm`).join(" | ");
  console.log(`1:${scale} (2f=${(2 * feature * 1000).toFixed(0)}mm) bbox ${m.bbox.map((b: number) => b.toFixed(3)).join(",")} | ${line}`);
}
