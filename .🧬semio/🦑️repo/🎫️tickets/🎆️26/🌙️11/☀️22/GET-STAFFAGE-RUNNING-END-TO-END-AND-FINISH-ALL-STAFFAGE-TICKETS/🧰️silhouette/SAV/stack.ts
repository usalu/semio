/** 🗂️ Stacks PNG sheets of `sheet.ts` under one another at a reduced width with a caption each, for a review of many sheets at once.
 *  bun stack.ts <out.png> <width> <caption=file.png>… */
import sharp from "/Users/ueli/Documents/semio/🎓️teaching/node_modules/sharp";

const [out, widthArg, ...items] = process.argv.slice(2) as string[];
const width = Number(widthArg), label = 14;
const parts: { input: Buffer; left: number; top: number }[] = [];
let top = 0;
for (const item of items) {
  const [caption, file] = item.split("=") as [string, string];
  const image = await sharp(file).resize({ width }).png().toBuffer({ resolveWithObject: true });
  parts.push({ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${label}"><text x="3" y="11" font-family="Helvetica" font-size="11" fill="#222">${caption}</text></svg>`), left: 0, top });
  parts.push({ input: image.data, left: 0, top: top + label });
  top += label + image.info.height;
}
await sharp({ create: { width, height: top, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out!);
console.log(out);
