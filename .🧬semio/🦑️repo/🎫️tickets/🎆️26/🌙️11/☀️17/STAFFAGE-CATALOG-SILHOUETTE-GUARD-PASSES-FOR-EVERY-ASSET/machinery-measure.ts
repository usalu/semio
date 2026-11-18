/** 📏 The measurement of the guard for one asset and scale: bbox, IoU and Hausdorff per view.
 *  bun machinery-measure.ts <asset> <scale> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg] = process.argv.slice(2) as [string, string];
const entry = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const scale = Number(scaleArg);
const feature = featureOf(scale);
const m = measureAsset(asset, scale, undefined, undefined, feature);
console.log("bbox", m.bbox.map((x) => +x.toFixed(4)));
for (const v of VIEWPOINTS) console.log(v, "iou", m.views[v].iou.toFixed(4), "hausdorff", m.views[v].hausdorffM.toFixed(4));
