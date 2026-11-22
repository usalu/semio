/** ✂️ Renders an SVG at a width and keeps the region of the figure (left, top, width, height as fractions of the page).
 *  bun crop.ts <file.svg> <out.png> <width> <l> <t> <w> <h> */
const sharp = (await import("../../../../../../../../../node_modules/sharp/lib/index.js")).default;
const [f, out, width, l, t, w, h] = process.argv.slice(2);
const img = sharp(f, { density: 600 }).resize({ width: Number(width) }).flatten({ background: "#ffffff" });
const meta = await img.clone().png().toBuffer({ resolveWithObject: true });
const W = meta.info.width, H = meta.info.height;
await sharp(meta.data).extract({ left: Math.round(Number(l) * W), top: Math.round(Number(t) * H), width: Math.round(Number(w) * W), height: Math.round(Number(h) * H) }).png().toFile(out!);
console.log(out);
