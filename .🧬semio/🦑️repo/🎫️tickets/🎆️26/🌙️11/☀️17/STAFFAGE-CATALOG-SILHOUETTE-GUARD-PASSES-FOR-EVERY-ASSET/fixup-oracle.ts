/** 🧪 Owned against sharp IoU per scale and view, with the vertex counts of the raster inputs.
 *  bun fixup-oracle.ts <asset folder> [scale ...] */
import { SCALE_DENOMINATORS, type Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { sharpIou } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";

const [folder, ...only] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const scales = SCALE_DENOMINATORS.filter((d) => (asset.volumes as Record<string, { status: string }>)[String(d)]?.status === "authored").filter((d) => only.length === 0 || only.includes(String(d)));
const count = (g: readonly (readonly (readonly unknown[])[])[]): number => g.reduce((s, r) => s + r.reduce((a, x) => a + x.length, 0), 0);
for (const scale of scales) {
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale, undefined, undefined, feature);
  for (const view of VIEWPOINTS) {
    const v = m.views[view];
    const sizes = { filled: count(v.filled as never), outlines: count(v.outlines as never), reference: count(v.reference as never), cell: v.cell, w: (v.box.maxX - v.box.minX) / v.cell, h: (v.box.maxY - v.box.minY) / v.cell };
    let s = NaN;
    try { s = await sharpIou(v); } catch (e) { s = -1; console.log(String(e).slice(0, 80)); }
    console.log(JSON.stringify({ scale, view, owned: +v.iou.toFixed(4), sharp: +s.toFixed(4), diff: +Math.abs(s - v.iou).toFixed(4), ...sizes }));
  }
}
