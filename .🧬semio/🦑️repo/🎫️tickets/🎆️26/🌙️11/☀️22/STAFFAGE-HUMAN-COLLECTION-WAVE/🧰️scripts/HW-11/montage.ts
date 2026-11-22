/** 🖼️ Joins PNGs side by side (crop box optional as fractions x0,y0,x1,y1): bun montage.ts out.png [--crop 0,0.3,1,1] a.png b.png … */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const args = process.argv.slice(2);
const out = args.shift()!;
let crop: number[] | undefined;
if (args[0] === "--crop") { args.shift(); crop = args.shift()!.split(",").map(Number); }
const parts = await Promise.all(args.map(async (f) => {
  let img = sharp(f);
  const m = await img.metadata();
  if (crop) img = sharp(f).extract({ left: Math.round(crop[0]! * m.width!), top: Math.round(crop[1]! * m.height!), width: Math.round((crop[2]! - crop[0]!) * m.width!), height: Math.round((crop[3]! - crop[1]!) * m.height!) });
  const buf = await img.png().toBuffer();
  const mm = await sharp(buf).metadata();
  return { buf, w: mm.width!, h: mm.height! };
}));
const W = parts.reduce((s, p) => s + p.w, 0), Hh = Math.max(...parts.map((p) => p.h));
let x = 0;
await sharp({ create: { width: W, height: Hh, channels: 3, background: "#ffffff" } }).composite(parts.map((p) => { const c = { input: p.buf, left: x, top: 0 }; x += p.w; return c; })).png().toFile(out);
