/** 🖼️ Rasterises SVG files to PNG next to them with sharp (librsvg): annotations and texts are removed and the figure is trimmed (CROP=0 keeps the sheet), scaled to the given pixel width.
 *  bun png.ts <widthPx> <file.svg> [...] */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [widthArg, ...files] = process.argv.slice(2);
const strip = (svg: string): string => {
  const start = svg.indexOf('<g id="staffage-annotations"');
  if (start >= 0) {
    let depth = 0, i = start;
    const re = /<g\b|<\/g>/g;
    re.lastIndex = start;
    for (let m = re.exec(svg); m; m = re.exec(svg)) { depth += m[0] === "</g>" ? -1 : 1; if (depth === 0) { i = m.index + 4; break; } }
    svg = svg.slice(0, start) + svg.slice(i);
  }
  return svg.replace(/<text[\s\S]*?<\/text>/g, "");
};
for (const file of files) {
  const crop = process.env.CROP !== "0";
  let text = crop ? strip(await Bun.file(file).text()) : await Bun.file(file).text();
  if (crop) {
    const vb = text.match(/viewBox="([-\d.e]+) ([-\d.e]+) ([-\d.e]+) ([-\d.e]+)"/)!.slice(1).map(Number);
    const probeW = 3000, probeD = 72 * probeW / (await sharp(Buffer.from(text), { density: 72, limitInputPixels: false }).metadata()).width!;
    const { info } = await sharp(Buffer.from(text), { density: probeD, limitInputPixels: false }).flatten({ background: "#ffffff" }).trim({ threshold: 4 }).toBuffer({ resolveWithObject: true });
    const sx = vb[2]! / info.width! * 0 + vb[2]! / (await sharp(Buffer.from(text), { density: probeD, limitInputPixels: false }).metadata()).width!;
    const pad = 0.02 * Math.max(info.width!, info.height!) * sx;
    const [x, y, w, h] = [vb[0]! - info.trimOffsetLeft! * sx - pad, vb[1]! - info.trimOffsetTop! * sx - pad, info.width! * sx + 2 * pad, info.height! * sx + 2 * pad];
    text = text.replace(/viewBox="[^"]*"/, `viewBox="${x} ${y} ${w} ${h}"`).replace(/width="[^"]*mm" height="[^"]*mm"/, `width="${w}mm" height="${h}mm"`);
  }
  const input = Buffer.from(text);
  const meta = await sharp(input, { density: 72, limitInputPixels: false }).metadata();
  const density = Math.min(72 * Number(widthArg) / meta.width!, 72 * 30000 / Math.max(meta.width!, meta.height!));
  await sharp(input, { density, limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/, ".png"));
  console.log(file.replace(/\.svg$/, ".png"));
}
