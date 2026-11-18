/** 🧪 Sweeps a scalar over an in-memory edit and prints owned and sharp IoU per view.
 *  bun fixup-sweep.ts <asset folder> <scale> "<edit using s and asset>" s0 s1 ... */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { sharpIou } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";

const [folder, scale, edit, ...values] = process.argv.slice(2) as [string, string, string, ...string[]];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const text = JSON.stringify(((await readStreamedAsset(entry)) as { asset: Asset }).asset);
for (const s of values.map(Number)) {
  const asset = JSON.parse(text) as Asset;
  new Function("asset", "s", edit)(asset, s);
  const m = measureAsset(asset, Number(scale), undefined, undefined, featureOf(Number(scale)));
  const cells: string[] = [];
  for (const view of VIEWPOINTS) {
    const v = m.views[view];
    const sh = await sharpIou(v);
    cells.push(view[0] + " o" + v.iou.toFixed(3) + " s" + sh.toFixed(3) + " d" + Math.abs(sh - v.iou).toFixed(3));
  }
  console.log("s=" + s, cells.join(" | "), "H", Math.max(...VIEWPOINTS.map((w) => m.views[w].hausdorffM)).toFixed(4), "bbox", m.bbox.map((b) => b.toFixed(4)).join(","));
}
