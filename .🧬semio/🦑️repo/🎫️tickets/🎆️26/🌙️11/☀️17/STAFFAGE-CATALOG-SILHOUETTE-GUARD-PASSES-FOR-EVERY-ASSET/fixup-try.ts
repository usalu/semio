/** 🧪 Owned against sharp IoU of one asset after an in-memory edit; the edit is a JavaScript function body over `asset`.
 *  bun fixup-try.ts <asset folder> <scale> "<edit>" */
import { SCALE_DENOMINATORS, type Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { sharpIou } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";

const [folder, scale, edit] = process.argv.slice(2) as [string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
new Function("asset", edit ?? "")(asset);
void SCALE_DENOMINATORS;
const m = measureAsset(asset, Number(scale), undefined, undefined, featureOf(Number(scale)));
for (const view of VIEWPOINTS) {
  const v = m.views[view];
  const s = await sharpIou(v);
  console.log(view, "owned", +v.iou.toFixed(4), "sharp", +s.toFixed(4), "diff", +Math.abs(s - v.iou).toFixed(4), "hausdorff", +v.hausdorffM.toFixed(5));
}
console.log("bbox", m.bbox.map((b) => +b.toFixed(4)).join(" "));
