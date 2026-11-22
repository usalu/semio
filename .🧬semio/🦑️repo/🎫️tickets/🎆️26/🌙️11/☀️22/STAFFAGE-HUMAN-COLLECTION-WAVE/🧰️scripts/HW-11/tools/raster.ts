/** 🖼️ Stand-in for `rsvg-convert -d dpi -p dpi -f png -o out in` on a host without librsvg: sharp (librsvg inside) from the repo's node_modules. */
import sharp from "../../../../../../../../../../node_modules/sharp/lib/index.js";
const a = process.argv.slice(2);
const at = (f: string) => a[a.indexOf(f) + 1]!;
const dpi = Number(a.includes("-d") ? at("-d") : 96);
await sharp(a[a.length - 1]!, { density: dpi, limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(at("-o"));
