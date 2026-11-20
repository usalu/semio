/** 🔎️ Round 3 heat pump diagnosis: `bun r3-heat-pump-outdoor-unit-diag.ts <asset file> <scale> <view> [x0 y0 x1 y1]` draws what the product's silhouette rule compares in one viewpoint (red = drawn only, blue = body only, grey = both) to 🗑️generated/r3-heat-pump-outdoor-unit/diag and prints the counts; a scale pair `50:100` compares the two bodies instead (red = the coarser body only, blue = the finer body only).
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts
 */
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";

import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [file, scale, view] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const [finer, coarser] = scale!.split(":");
const m0 = (measureAsset(asset, Number(finer)) as any).views[view!];
const m = coarser === undefined ? m0 : { ...m0, filled: (measureAsset(asset, Number(coarser)) as any).views[view!].visible };
const win = process.argv.slice(5, 9).map(Number);
const b = win.length === 4 ? { minX: win[0]!, minY: win[1]!, maxX: win[2]!, maxY: win[3]! } : m.box;
const k = 2200 / (b.maxX - b.minX);
const d = (groups: any[]): string => groups.map((g) => g.map((r: any[]) => "M" + r.map((p) => `${((p.x - b.minX) * k).toFixed(1)},${((b.maxY - p.y) * k).toFixed(1)}`).join("L") + "Z").join("")).map((p) => `<path d="${p}"/>`).join("");
const h = Math.ceil((b.maxY - b.minY) * k);
const layer = async (groups: any[], rule: string): Promise<Buffer> => sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="2200" height="${h}"><rect width="100%" height="100%" fill="#000"/><g fill="#fff" fill-rule="${rule}" shape-rendering="crispEdges">${d(groups)}</g></svg>`)).greyscale().raw().toBuffer();
const [drawn, volume] = [await layer(m.filled, coarser === undefined ? "evenodd" : "nonzero"), await layer(m.reference, "nonzero")];
const rgb = Buffer.alloc(2200 * h * 3, 255);
for (let i = 0; i < 2200 * h; i++) {
  const [p, q] = [drawn[i]! > 127, volume[i]! > 127];
  const c = p && q ? [205, 205, 205] : p ? [230, 30, 30] : q ? [30, 60, 230] : [255, 255, 255];
  rgb[i * 3] = c[0]!;
  rgb[i * 3 + 1] = c[1]!;
  rgb[i * 3 + 2] = c[2]!;
}
const out = join(import.meta.dir, "🗑️generated", "r3-heat-pump-outdoor-unit", "diag");
mkdirSync(out, { recursive: true });
const path = join(out, `s${scale!.replace(":", "-")}.${view}${win.length === 4 ? ".crop" : ""}.png`);
await sharp(rgb, { raw: { width: 2200, height: h, channels: 3 } }).png().toFile(path);
console.log(path, "iou", m.iou.toFixed(4), "p95", m.p95M, "counts", JSON.stringify(m.counts), "filled groups", m.filled.length, "visible groups", m.visible.length);
