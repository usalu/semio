/** 🖼️ Rasterises SVG files to PNG with sharp (the repo's oracle rasteriser), at a given width in pixels.
 *  bun png.ts <width> <file.svg> ... */
const sharp = (await import("../../../../../../../../../node_modules/sharp/lib/index.js")).default;
const [width, ...files] = process.argv.slice(2);
for (const f of files) {
  const out = f.replace(/\.svg$/, ".png");
  await sharp(f, { density: 300 }).resize({ width: Number(width) }).flatten({ background: "#ffffff" }).png().toFile(out);
  console.log(out);
}
