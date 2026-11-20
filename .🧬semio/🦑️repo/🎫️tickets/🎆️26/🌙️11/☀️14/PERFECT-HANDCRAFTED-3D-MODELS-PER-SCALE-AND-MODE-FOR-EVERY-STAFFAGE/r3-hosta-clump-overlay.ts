/** 🔍️ Round 3, vegetation/hosta-clump: a large overlay of the drawing and the projected body of one view (green both, red drawn only, blue body only). Usage: bun r3-hosta-clump-overlay.ts <scale> <view> [bone-prefix]
 * @see ./r3-vegetation-hosta-clump.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { Canvas } from "./volumes-raster.ts";
import { GENERATED, entryOf, iouOf, loadSource, merged, poseState, scaleMeshes, viewMasks, type View } from "./volumes-lib.ts";
const [scale, view, prefix] = [Number(process.argv[2]), process.argv[3] as View, process.argv[4]];
const { entry, asset: catalog } = entryOf("vegetation/hosta-clump");
const asset = merged(catalog, loadSource(entry)!);
const state = poseState(asset, scale);
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const keep = (bone: string) => prefix === undefined || bone.startsWith(prefix);
const body = sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true && keep(sm.solids.body[i]!.bone));
const m = viewMasks(view, { ...state, [view]: state[view].filter((s: { shape: { bone: string } }) => keep(s.shape.bone)) }, body);
const k = Math.max(1, Math.floor(1800 / m.truth.width));
const c = new Canvas(m.truth.width * k, m.truth.height * k, [247, 247, 244]);
for (let y = 0; y < m.truth.height; y++) for (let x = 0; x < m.truth.width; x++) {
  const [t, v] = [m.truth.cells[y * m.truth.width + x]!, m.volume.cells[y * m.truth.width + x]!];
  if (!t && !v) continue;
  for (let dy = 0; dy < k; dy++) for (let dx = 0; dx < k; dx++) c.set(x * k + dx, (m.truth.height - 1 - y) * k + dy, t && v ? [140, 200, 150] : t ? [225, 70, 70] : [60, 90, 220]);
}
mkdirSync(join(GENERATED, "r3-hosta-clump"), { recursive: true });
const out = join(GENERATED, "r3-hosta-clump", `overlay.s${scale}.${view}${prefix ? `.${prefix}` : ""}.png`);
writeFileSync(out, c.png());
console.log(`[DEBUG] iou ${iouOf(m.truth, m.volume).toFixed(4)} cell ${m.cell} ${out}`);
