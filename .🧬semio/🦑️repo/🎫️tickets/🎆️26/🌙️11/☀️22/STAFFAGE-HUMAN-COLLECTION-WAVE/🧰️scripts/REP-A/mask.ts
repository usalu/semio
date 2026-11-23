/** 🎭️ The drawing against the volume of one view of the default pose as an image: grey where both are, red where only the drawing is, blue where only the body is; a window `x0,y0,x1,y1` in metres crops it.
 *  bun mask.ts <asset.json> <scale> <view> <out.png> [x0,y0,x1,y1] [pixels per cell=2] */
import sharp from "sharp";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, measureView, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [path, sc, view, out, win, zoomArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const scale = Number(sc), feature = featureOf(scale as never);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: any) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature);
const drawn = new Mask(m.box, m.cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(m.box, m.cell);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const [x0, y0, x1, y1] = win && win !== "-" ? win.split(",").map(Number) as [number, number, number, number] : [m.box.minX, m.box.minY, m.box.maxX, m.box.maxY];
const [i0, i1, j0, j1] = [Math.max(0, Math.floor((x0 - m.box.minX) / m.cell)), Math.min(w, Math.ceil((x1 - m.box.minX) / m.cell)), Math.max(0, Math.floor((y0 - m.box.minY) / m.cell)), Math.min(h, Math.ceil((y1 - m.box.minY) / m.cell))];
const z = Number(zoomArg ?? 2), W = (i1 - i0) * z, H = (j1 - j0) * z;
const px = Buffer.alloc(W * H * 3, 255);
for (let j = j0; j < j1; j++) for (let i = i0; i < i1; i++) {
  const d = drawn.cells[j * w + i], v = vol.cells[j * w + i];
  if (!d && !v) continue;
  const c = d && v ? [190, 190, 190] : d ? [220, 40, 40] : [40, 80, 220];
  for (let a = 0; a < z; a++) for (let b = 0; b < z; b++) { const k = ((j1 - 1 - j) * z + a) * W * 3 + ((i - i0) * z + b) * 3; px[k] = c[0]!; px[k + 1] = c[1]!; px[k + 2] = c[2]!; }
}
await sharp(px, { raw: { width: W, height: H, channels: 3 } }).png().toFile(out!);
console.log(`${out} ${W}x${H} px, window x ${(m.box.minX + i0 * m.cell).toFixed(3)}..${(m.box.minX + i1 * m.cell).toFixed(3)} y ${(m.box.minY + j0 * m.cell).toFixed(3)}..${(m.box.minY + j1 * m.cell).toFixed(3)}, iou ${m.iou.toFixed(3)} haus ${(m.hausdorffM * 1000).toFixed(1)} mm`);
