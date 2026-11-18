/** 🧪 Prints the covered fraction of every cell of the reference and of the drawing of one view (8 x 8 supersampling).
 *  bun fixup-cover.ts <asset folder> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { raster } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";

const [folder, scale, view, edit] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
new Function("asset", edit ?? "")(asset);
const m = measureAsset(asset, Number(scale), undefined, undefined, featureOf(Number(scale))).views[view];
const w = Math.max(1, Math.ceil((m.box.maxX - m.box.minX) / m.cell));
const h = Math.max(1, Math.ceil((m.box.maxY - m.box.minY) / m.cell));
const K = 8;
for (const [name, groups] of [["reference", m.reference], ["filled", m.filled]] as const) {
  const fine = await raster(groups, m.box, m.cell / K, w * K, h * K);
  console.log(name);
  for (let j = 0; j < h; j++) {
    let row = "";
    for (let i = 0; i < w; i++) { let s = 0; for (let b = 0; b < K; b++) for (let a = 0; a < K; a++) s += fine[(j * K + b) * w * K + i * K + a]!; row += String(Math.round((s / (K * K)) * 99)).padStart(3, " "); }
    console.log(row);
  }
}
