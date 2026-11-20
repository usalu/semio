/** 🔍️ Magnifies the tealight in the product's 2D SVG panels (it is under a millimetre on paper) into one montage for looking.
 *
 * Usage: `bun r3-tealight-zoom.ts <out.png> <panel.svg>...` (panels written by `r3-2d.ts --svg`) */
import { readFileSync } from "node:fs";
import sharp from "sharp";

const [out, ...panels] = process.argv.slice(2);
const [cell, halfMm] = [400, 0.75];
const tiles = await Promise.all(
  panels.map(async (path, index) => {
    const svg = readFileSync(path, "utf8");
    const [ox, oy] = [Number(/origin-x-mm="([^"]+)"/.exec(svg)![1]), Number(/origin-y-mm="([^"]+)"/.exec(svg)![1])];
    const plan = path.includes(".plan.");
    const cy = plan ? oy : oy - 0.42;
    const cropped = svg.replace(/<svg ([^>]*?)width="[^"]+" height="[^"]+" viewBox="[^"]+"/, `<svg $1width="${cell}px" height="${cell}px" viewBox="${ox - halfMm} ${cy - halfMm} ${2 * halfMm} ${2 * halfMm}"`);
    const input = await sharp(Buffer.from(cropped)).resize(cell, cell).png().toBuffer();
    return { input, left: (index % 4) * cell, top: Math.floor(index / 4) * cell };
  }),
);
await sharp({ create: { width: 4 * cell, height: Math.ceil(panels.length / 4) * cell, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
