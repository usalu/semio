/** 🔲️ A grid of pictures, each 520 px wide, with its file name: grid.ts <out.jpg> <columns> <picture>… */
import { basename } from "node:path";
import sharp from "sharp";
const [out, columns, ...parts] = process.argv.slice(2) as [string, string, ...string[]];
const W = 520, n = Number(columns);
const images = await Promise.all(parts.map(async (p) => ({ p, ...(await sharp(p).resize({ width: W }).png().toBuffer({ resolveWithObject: true })) })));
const H = Math.max(...images.map((i) => i.info.height)) + 18;
const layers = images.flatMap((i, k) => { const left = (k % n) * W, top = Math.floor(k / n) * H; return [{ input: i.data, left, top: top + 18 }, { input: Buffer.from(`<svg width="${W}" height="18"><text x="4" y="14" font-size="13" font-family="Arial">${basename(i.p)}</text></svg>`), left, top }]; });
await sharp({ create: { width: W * n, height: Math.ceil(images.length / n) * H, channels: 3, background: "white" } }).composite(layers).jpeg({ quality: 80 }).toFile(out);
