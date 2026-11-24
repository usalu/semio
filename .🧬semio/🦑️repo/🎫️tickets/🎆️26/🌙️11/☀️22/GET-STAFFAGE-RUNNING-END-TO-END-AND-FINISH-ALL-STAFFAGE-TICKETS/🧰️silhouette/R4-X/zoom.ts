/** 🔬️ A part of a product drawing at any magnification: reads an SVG that svg.ts wrote, sets its viewBox to a part given in fractions of the drawing and rasterises it with sharp (librsvg): bun zoom.ts <in.svg> <out.png> <x0> <y0> <x1> <y1> [width px]   (run from the staffage typescript package, sharp lives there) */
import sharp from "sharp";
const [input, output, x0, y0, x1, y1, widthArg] = process.argv.slice(2) as string[];
const svg = await Bun.file(input!).text();
const box = /viewBox="([-\d.e]+)[ ,]+([-\d.e]+)[ ,]+([-\d.e]+)[ ,]+([-\d.e]+)"/u.exec(svg)!.slice(1).map(Number) as [number, number, number, number];
const [x, y, w, h] = [box[0] + Number(x0) * box[2], box[1] + Number(y0) * box[3], (Number(x1) - Number(x0)) * box[2], (Number(y1) - Number(y0)) * box[3]];
const width = Number(widthArg ?? 1400);
const cut = svg.replace(/viewBox="[^"]*"/u, `viewBox="${x} ${y} ${w} ${h}"`).replace(/<svg([^>]*?)\swidth="[^"]*"/u, "<svg$1").replace(/<svg([^>]*?)\sheight="[^"]*"/u, "<svg$1").replace("<svg", `<svg width="${width}" height="${Math.round((width * h) / w)}"`);
await sharp(Buffer.from(cut), { limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(output!);
console.log(`${output} viewBox ${box.join(" ")}`);
