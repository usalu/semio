/** 🧪 Prints the owned and the sharp raster of the drawing and the reference of one view.
 *  bun fixup-grid.ts <asset folder> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { Mask, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { raster } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";

const [folder, scale, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === folder)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const m = measureAsset(asset, Number(scale), undefined, undefined, featureOf(Number(scale))).views[view];
const w = Math.max(1, Math.ceil((m.box.maxX - m.box.minX) / m.cell));
const h = Math.max(1, Math.ceil((m.box.maxY - m.box.minY) / m.cell));
const own = (groups: typeof m.filled): Uint8Array => { const k = new Mask(m.box, m.cell); for (const g of groups) k.fill(g as never); return k.cells; };
const show = (name: string, a: Uint8Array): void => { console.log(name); for (let j = 0; j < h; j++) console.log(Array.from(a.slice(j * w, (j + 1) * w), (v) => (v ? "#" : ".")).join("")); };
console.log(JSON.stringify({ box: m.box, cell: m.cell, w, h }));
show("owned filled", own(m.filled));
show("sharp filled", await raster(m.filled, m.box, m.cell, w, h));
show("reference", own(m.reference));
show("sharp reference", await raster(m.reference, m.box, m.cell, w, h));
console.log(JSON.stringify(m.filled));
console.log(JSON.stringify(m.reference).slice(0, 600));
