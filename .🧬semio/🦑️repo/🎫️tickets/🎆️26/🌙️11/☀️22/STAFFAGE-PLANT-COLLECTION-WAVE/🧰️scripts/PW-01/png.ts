/** 🖼️ Rasterises preview SVGs to PNG through sharp (the preview CLI needs rsvg-convert, which is not installed here). */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
for (const f of process.argv.slice(2)) { await sharp(f, { density: Number(process.env.DPI ?? 150) }).flatten({ background: "#ffffff" }).png().toFile(f.replace(/\.svg$/, ".png")); console.log("ok", f); }
