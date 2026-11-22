/** 🖼️ Rasterises every SVG below a folder to PNG next to it with sharp (rsvg-convert is not installed here).
 *  bun png.ts <dir> [dpi=200] */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [dir, dpiArg] = process.argv.slice(2);
const dpi = Number(dpiArg ?? 200);
const walk = (d: string): string[] => readdirSync(d).flatMap((n) => (statSync(join(d, n)).isDirectory() ? walk(join(d, n)) : n.endsWith(".svg") ? [join(d, n)] : []));
for (const svg of walk(dir!)) await sharp(svg, { density: dpi }).flatten({ background: "#ffffff" }).png().toFile(svg.replace(/\.svg$/, ".png"));
console.log("done");
