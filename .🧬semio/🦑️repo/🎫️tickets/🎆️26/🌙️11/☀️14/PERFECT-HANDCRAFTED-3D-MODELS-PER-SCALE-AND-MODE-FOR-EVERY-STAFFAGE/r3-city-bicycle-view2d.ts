/** 🔍 Draws one product 2D view of the city bicycle candidate (or the catalog asset) large: the drawing area of the SVG cropped to the bicycle and rasterised at 2000 px width, so strokes and paint order can be judged.
 *
 * Usage: `bun r3-city-bicycle-view2d.ts <scale> <elevation|front|plan> [light-line|light-solid|...] [--file <asset document json>] [--pose ID]`
 * Output: `🗑️generated/r3-city-bicycle/view2d.s<scale>.<viewpoint>.<view>[.<pose>].png`
 * @see ./r3-2d.ts */
import { join } from "node:path";
import sharp from "sharp";

import { drawingSvg } from "./r3-2d.ts";
import { GENERATED, entryOf, loadAsset, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const flag = (n: string): string | undefined => (args.indexOf(n) < 0 ? undefined : args[args.indexOf(n) + 1]);
const [scale, viewpoint] = [Number(args[0]), args[1]!];
const view = args[2] !== undefined && !args[2].startsWith("--") ? args[2] : "light-line";
const file = flag("--file");
const asset: Json = file === undefined ? entryOf("vehicles/city-bicycle").asset : loadAsset(file);
const pose = flag("--pose") ?? asset.presentation.defaultPose;
const svg = drawingSvg(asset, scale, { kind: "pose", pose }, view, viewpoint);
const out = join(GENERATED, "r3-city-bicycle", `view2d.s${scale}.${viewpoint}.${view}${flag("--pose") === undefined ? "" : `.${pose}`}.png`);
const head = /<svg[^>]*>/u.exec(svg)![0];
const probe = await sharp(Buffer.from(svg), { density: 72 }).metadata();
const density = 600;
const big = await sharp(Buffer.from(svg), { density, limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toBuffer();
const meta = await sharp(big).metadata();
const w = meta.width!;
const h = meta.height!;
await sharp(big).extract({ left: 0, top: 0, width: Math.round(w * 0.86), height: Math.round(h * 0.64) }).resize(2000).png().toFile(out);
if (args.includes("--head")) process.stdout.write(`${head} ${probe.width}x${probe.height} density ${density}\n`);
process.stdout.write(`${out}\n`);
