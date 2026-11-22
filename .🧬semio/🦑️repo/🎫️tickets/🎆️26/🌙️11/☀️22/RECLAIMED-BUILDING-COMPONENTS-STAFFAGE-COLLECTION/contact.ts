/** 🖼️ Finisher review aid: lays rendered sheets side by side in one contact sheet (two columns), each panel reduced to 1200 px width with its file name above, so that every sheet of an asset is opened in a few images.
 *
 * `$BUN contact.ts <out.png> <sheet.png>…` */
import sharp from "sharp";
import { basename } from "node:path";

const [out, ...sheets] = process.argv.slice(2);
const columns = sheets.length === 1 ? 1 : 2;
const width = 1200;
const panels = await Promise.all(sheets.map(async (sheet) => {
  const image = await sharp(sheet).resize({ width }).png().toBuffer({ resolveWithObject: true });
  return { sheet, data: image.data, height: image.info.height };
}));
const rowHeight = Math.max(...panels.map((panel) => panel.height)) + 26;
const rows = Math.ceil(panels.length / columns);
const parts = panels.flatMap((panel, index) => {
  const left = (index % columns) * width;
  const top = Math.floor(index / columns) * rowHeight;
  const label = Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="26"><rect width="100%" height="100%" fill="#222"/><text x="8" y="19" font-family="Helvetica" font-size="17" fill="#fff">${basename(panel.sheet)}</text></svg>`);
  return [{ input: label, left, top }, { input: panel.data, left, top: top + 26 }];
});
await sharp({ create: { width: width * columns, height: rows * rowHeight, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out);
console.log(out);
