/** 🩻️ Drawing against volume, one PNG per view: drawn only red, volume only blue, both grey; prints the shapes and solids nearest to the worst cells.
 *  bun diag.ts <slug> <scale> [pose] [out-dir] */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { Mask, measureView, worldSolids, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { HUMANS, TICKET } from "./lib.ts";
const [slug, scaleArg, poseArg, outArg] = process.argv.slice(2);
const scale = Number(scaleArg);
const asset = (await Bun.file(`${HUMANS}/${slug}/🔣️.json`).json()).asset;
const pose = poseArg && poseArg !== "-" ? poseArg : asset.presentation.defaultPose;
const out = outArg ?? `${TICKET}/🗑️generated/HW-12/diag`;
await Bun.write(`${out}/.keep`, "");
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: { texture?: boolean }) => w.texture !== true);
for (const view of VIEWPOINTS) {
  const m = measureView(asset, state as never, solids, view, scale, 900, undefined, Infinity);
  const drawn = new Mask(m.box, m.cell), vol = new Mask(m.box, m.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  for (const g of m.reference) vol.fill(g);
  const px = Buffer.alloc(drawn.width * drawn.height * 3, 255);
  for (let j = 0; j < drawn.height; j++) for (let i = 0; i < drawn.width; i++) {
    const k = j * drawn.width + i, o = ((drawn.height - 1 - j) * drawn.width + i) * 3;
    const [d, v] = [drawn.cells[k], vol.cells[k]];
    if (d && v) px.fill(170, o, o + 3); else if (d) { px[o] = 230; px[o + 1] = 40; px[o + 2] = 40; } else if (v) { px[o] = 40; px[o + 1] = 80; px[o + 2] = 230; }
  }
  await sharp(px, { raw: { width: drawn.width, height: drawn.height, channels: 3 } }).png().toFile(`${out}/${slug}-${scale}-${pose}-${view}.png`);
  console.log(`${view}: iou ${m.iou.toFixed(3)} H ${(m.hausdorffM * 1000).toFixed(0)} mm, box x ${m.box.minX.toFixed(3)}..${m.box.maxX.toFixed(3)} y ${m.box.minY.toFixed(3)}..${m.box.maxY.toFixed(3)}, cell ${(m.cell * 1000).toFixed(2)} mm, ${drawn.width}x${drawn.height}`);
}
