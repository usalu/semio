/** 🖼️ The product drawings of an asset before and after its repair, large: side elevation, front elevation and plan of the default pose at one scale as the product draws them (`evaluate`, `depict`, SVG export; light-line view, no annotations, each panel trimmed to the drawing and fitted to its frame), the document before in the upper row and the document after in the lower row, rasterised by sharp.
 *  bun sheet.ts <before.json> <after.json> <scale> <out.png> [view]   (view defaults to light-line) */
import sharp from "/Users/ueli/Documents/semio/🎓️teaching/node_modules/sharp";
import { defaultDepictionOptions, depict } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";

const [before, after, scaleArg, out, viewArg] = process.argv.slice(2) as string[];
const scale = Number(scaleArg), view = viewArg ?? "light-line";
const [pw, ph] = [520, 330];
const parts: { input: Buffer; left: number; top: number }[] = [];
for (const [r, file] of [before!, after!].entries()) {
  const asset = (await Bun.file(file).json()).asset;
  const selector = { kind: "scale", scale: { denominator: scale } } as const;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
  for (const [c, viewpoint] of (["elevation", "front", "plan"] as const).entries()) {
    const o = defaultDepictionOptions(view as never, "en", scale, viewpoint as never) as any;
    const svg = writeSvg(depict(state, { ...o, representation: selector, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: true } } as never, {}));
    const raster = await sharp(Buffer.from(svg), { density: Number(process.env.DENSITY ?? (scale >= 200 ? 1200 : 500)), limitInputPixels: false }).png().toBuffer();
    const panel = await sharp(await sharp(raster, { limitInputPixels: false }).trim({ threshold: 12 }).toBuffer(), { limitInputPixels: false }).resize(pw - 12, ph - 12, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer();
    parts.push({ input: panel, left: c * pw + 6, top: r * ph + 6 });
  }
}
await sharp({ create: { width: pw * 3, height: ph * 2, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out!);
console.log(out);
