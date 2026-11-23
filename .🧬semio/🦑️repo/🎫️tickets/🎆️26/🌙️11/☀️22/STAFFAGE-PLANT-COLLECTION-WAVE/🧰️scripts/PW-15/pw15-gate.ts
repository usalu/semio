/** 🚦 PW-15 authoring aid: the catalog-silhouette gate of one asset file as the guard measures it, with the place of the worst boundary distance. Per authored scale and view: IoU and Hausdorff of `measureAsset`; for a failing view (or all with `--all`) the tiles of the banded measurement are walked again with the same windows, and the boundary cell farthest from the other mask is reported with its tile, whether the drawing or the body has the stray boundary there, and with `--png <dir>` an overlay of that neighbourhood (grey both, red body only, blue drawing only, after the small holes are filled; black marks the stray boundary).
 *
 * `bun pw15-gate.ts <asset document json> [--scale N] [--view v] [--all] [--png <dir>]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { Mask, TILE_CELLS, VIEWPOINTS, WINDOW_FEATURES, boundaryOf, comparisonCell, distanceField, fillSmallHoles, measureAsset, measureView, worldSolids, type Group, type Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const args = process.argv.slice(2);
const flag = (name: string) => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const asset = JSON.parse(readFileSync(args[0]!, "utf8")).asset;
const gate = JSON.parse(readFileSync(join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️catalog-silhouette/🔣️.json"), "utf8")).gate as { iou: number };
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of [50, 100, 200, 500]) {
  if (flag("--scale") && Number(flag("--scale")) !== scale) continue;
  const feature = featureOf(scale as never);
  const m = measureAsset(asset, scale, undefined, undefined, feature);
  const box = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "✗" : ""}`).join(" ");
  const line = `GATE 1:${scale} bbox ${box} | ${VIEWPOINTS.map((v) => `${v} iou ${m.views[v].iou.toFixed(3)}${m.views[v].iou < gate.iou ? "✗" : ""} h ${(m.views[v].hausdorffM * 1000).toFixed(0)}mm${m.views[v].hausdorffM > 2 * feature ? "✗" : ""}`).join(" | ")} (limit ${(2000 * feature).toFixed(0)} mm)`;
  if (line.includes("✗")) bad++;
  console.log(line);
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  for (const view of VIEWPOINTS as readonly Viewpoint[]) {
    if (flag("--view") && flag("--view") !== view) continue;
    const measured = m.views[view];
    if (!args.includes("--all") && measured.hausdorffM <= 2 * feature) continue;
    const groups = [...(measured.full as readonly Group[]), ...(measured.reference as readonly Group[]), ...(measured.filled as readonly Group[])];
    const bounds = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
    for (const g of groups) for (const r of g) for (const p of r) {
      if (p.x < bounds.minX) bounds.minX = p.x;
      if (p.x > bounds.maxX) bounds.maxX = p.x;
      if (p.y < bounds.minY) bounds.minY = p.y;
      if (p.y > bounds.maxY) bounds.maxY = p.y;
    }
    const cell = comparisonCell(feature, Math.max(bounds.maxX - bounds.minX, bounds.maxY - bounds.minY));
    const frame = { minX: bounds.minX - 2 * cell, minY: bounds.minY - 2 * cell, maxX: bounds.maxX + 2 * cell, maxY: bounds.maxY + 2 * cell };
    if (Math.max(frame.maxX - frame.minX, frame.maxY - frame.minY) / cell <= TILE_CELLS) { console.log(`[DEBUG]   ${view}: one raster, not tiled`); continue; }
    const side = TILE_CELLS * cell;
    const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
    const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
    const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
    const found: { d: number; x: number; y: number; who: string; tile: string }[] = [];
    for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
      const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
      const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
      const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
      const tile = measureView(asset, state as never, solids, view, scale, undefined, feature, Infinity, false, { core, box, cell });
      const volume = new Mask(box, cell);
      for (const g of measured.reference as readonly Group[]) volume.fill(g, true);
      volume.seal();
      const drawing = new Mask(box, cell);
      for (const g of measured.filled as readonly Group[]) drawing.fill(g);
      const [W, H] = [volume.width, volume.height];
      const [fa, fb] = [fillSmallHoles(drawing.cells, W, H, holes), fillSmallHoles(volume.cells, W, H, holes)];
      const [ba, bb] = [boundaryOf(fa, W, H), boundaryOf(fb, W, H)];
      const range = { i0: Math.round((core.minX - box.minX) / cell), i1: Math.round((core.maxX - box.minX) / cell), j0: Math.round((core.minY - box.minY) / cell), j1: Math.round((core.maxY - box.minY) / cell) };
      let worst = { d: 0, x: 0, y: 0, who: "", k: -1 };
      for (const [who, from, to] of [["drawing", ba, bb], ["body", bb, ba]] as const) {
        const field = distanceField(to, W, H);
        for (let y = range.j0; y < range.j1; y++) for (let x = range.i0; x < range.i1; x++) {
          const k = y * W + x;
          if (from[k] && Math.sqrt(field[k]!) * cell > worst.d) worst = { d: Math.sqrt(field[k]!) * cell, x: box.minX + (x + 0.5) * cell, y: box.minY + (y + 0.5) * cell, who, k };
        }
      }
      found.push({ d: worst.d, x: worst.x, y: worst.y, who: worst.who, tile: `${i},${j} (guard ${(tile.hausdorffM * 1000).toFixed(0)} mm)` });
      const dir = flag("--png");
      if (dir !== undefined && worst.d > 2 * feature) {
        const half = Math.round(Math.max(40, (worst.d / cell) * 1.6));
        const [cx, cy] = [worst.k % W, Math.floor(worst.k / W)];
        const [x0, x1, y0, y1] = [Math.max(0, cx - half), Math.min(W, cx + half), Math.max(0, cy - half), Math.min(H, cy + half)];
        const [w, h] = [x1 - x0, y1 - y0];
        const rgb = Buffer.alloc(w * h * 3, 255);
        const flip = view !== "plan";
        for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) {
          const k = y * W + x;
          const o = ((flip ? y1 - 1 - y : y - y0) * w + (x - x0)) * 3;
          const [d, v] = [fa[k]!, fb[k]!];
          const raw = drawing.cells[k]! !== volume.cells[k]!;
          const c = (worst.who === "drawing" ? ba[k] && !bb[k] : bb[k] && !ba[k]) && Math.hypot(x - cx, y - cy) < 3 ? [0, 0, 0] : raw ? (volume.cells[k] ? [255, 0, 0] : [0, 0, 255]) : d && v ? [200, 200, 200] : v ? [255, 150, 150] : d ? [150, 150, 255] : [255, 255, 255];
          rgb[o] = c[0]!; rgb[o + 1] = c[1]!; rgb[o + 2] = c[2]!;
        }
        const out = join(dir, `${asset.id}-${scale}-${view}-${i}-${j}.png`);
        await sharp(rgb, { raw: { width: w, height: h, channels: 3 } }).resize({ width: Math.min(1400, w * 8), kernel: "nearest" }).png().toFile(out);
        console.log(`[DEBUG]   overlay ${out} (${w}x${h} cells; light red body only after hole filling, light blue drawing only, strong red and blue: raw cell differences)`);
      }
    }
    found.sort((p, q) => q.d - p.d);
    for (const f of found.slice(0, 4)) console.log(`[DEBUG]   ${view} tile ${f.tile}: ${(f.d * 1000).toFixed(0)} mm, stray ${f.who} boundary at (${f.x.toFixed(3)}, ${f.y.toFixed(3)})`);
  }
}
console.log(`GATE ${asset.id}: ${bad} failing scale(s) at IoU ${gate.iou}`);
process.exitCode = bad === 0 ? 0 : 1;
