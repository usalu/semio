/** 🔍️ One panel of a product sheet at a larger size, to judge fine lines: `bun zoom.ts <asset document json> <scale> <pose> <elevation|front|plan> <out.png> [height in pixels, default 1500] [left top width height as fractions of the panel]` (run from the folder of the 3D ticket, whose `r3-2d.ts` and `sharp` it uses) draws the line view of that viewpoint exactly as the product does and rasterises it, optionally cut to a part.
 * @see ./pairs.sh — the whole sheets */
import { readFileSync } from "node:fs";
const sharp: any = (await import(Bun.resolveSync("sharp", `${process.cwd()}/`))).default;
const { drawingSvg } = await import(`${process.cwd()}/r3-2d.ts`);
const [file, scale, pose, viewpoint, out, height, ...part] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const svg: string = drawingSvg(asset, Number(scale), { kind: "pose", pose }, "light-line", viewpoint!);
let image = sharp(Buffer.from(svg), { density: Number(process.env.DENSITY ?? 1200), limitInputPixels: false }).flatten({ background: "#ffffff" }).resize({ height: Number(height ?? 1500) });
if (part.length === 4) {
  const { data, info } = await image.png().toBuffer({ resolveWithObject: true });
  const [l, t, w, h] = part.map(Number) as [number, number, number, number];
  image = sharp(data).extract({ left: Math.round(l * info.width), top: Math.round(t * info.height), width: Math.round(w * info.width), height: Math.round(h * info.height) });
}
await image.png().toFile(out!);
console.log(out);
