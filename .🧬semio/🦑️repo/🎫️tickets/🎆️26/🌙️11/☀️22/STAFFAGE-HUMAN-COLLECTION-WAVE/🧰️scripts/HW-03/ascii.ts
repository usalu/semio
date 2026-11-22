/** 🔡️ ASCII map of drawing (d) against volume (v) and both (#) of one view around a point, at the measure cell of the scale.
 *  bun ascii.ts <file.json> <scale> <view> <x> <y> [halfWidthM] */
import { S } from "./lib.ts";
const sil = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);
const [file, sc, view, xs, ys, hs] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const m = sil.measureAsset(asset, Number(sc), undefined, undefined, featureOf(Number(sc))).views[view!];
const drawn = new sil.Mask(m.box, m.cell), vol = new sil.Mask(m.box, m.cell);
for (const g of [...m.filled, ...m.outlines]) drawn.fill(g);
for (const g of m.reference) vol.fill(g);
const h = Number(hs ?? 0.12), cx = Number(xs), cy = Number(ys);
for (let y = cy + h; y >= cy - h; y -= m.cell) {
  let row = "";
  for (let x = cx - h; x <= cx + h; x += m.cell) {
    const i = Math.floor((x - m.box.minX) / m.cell), j = Math.floor((y - m.box.minY) / m.cell);
    const k = j * drawn.width + i;
    const d = drawn.cells[k], v = vol.cells[k];
    row += d && v ? "#" : d ? "d" : v ? "v" : ".";
  }
  console.log(row);
}
