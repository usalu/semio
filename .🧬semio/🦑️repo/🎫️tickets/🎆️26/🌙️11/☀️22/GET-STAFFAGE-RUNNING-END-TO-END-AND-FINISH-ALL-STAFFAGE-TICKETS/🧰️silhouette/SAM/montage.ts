/** 🖼️ Stacks the solid-view rows (the lower half) of sheet PNGs under each other into one picture of a given width, for looking at several scales together: bun montage.ts <out.png> <width> <in.png>… */
import sharp from "sharp";

const [out, widthArg, ...inputs] = process.argv.slice(2) as string[];
const width = Number(widthArg);
const parts = await Promise.all(inputs.map(async (file) => {
  const meta = await sharp(file).metadata();
  const half = Math.floor(meta.height! * 0.487);
  const buffer = await sharp(file).extract({ left: 0, top: half, width: meta.width!, height: Math.floor(meta.height! * 0.3) }).resize({ width }).png().toBuffer();
  return { buffer, height: (await sharp(buffer).metadata()).height! };
}));
let top = 0;
const composite = parts.map((p) => { const at = { input: p.buffer, left: 0, top }; top += p.height; return at; });
await sharp({ create: { width, height: top, channels: 3, background: "#ffffff" } }).composite(composite).png().toFile(out!);
console.log(out);
