/** 🧱️ PW-16 diagnostic: the banded silhouette measure of the catalog guard tile by tile (same tiling as `measureBanded`), with the worst boundary cells of the worst tile.
 *
 * `repo tools exec bun -- tilediag.ts <asset.json> <scale> <view>` */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { Mask, TILE_CELLS, WINDOW_FEATURES, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [file, scaleText, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
const scale = Number(scaleText);
const f = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale as never } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const whole = measureView(asset, state, solids, view, scale, undefined, f, 1e9);
const cell = whole.cell;
const frame = whole.box;
const side = TILE_CELLS * cell;
const margin = Math.ceil((f * WINDOW_FEATURES) / cell) * cell;
const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
let worst = { h: 0, core: frame, box: frame };
for (let j = 0; j < ny; j++)
  for (let i = 0; i < nx; i++) {
    const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
    const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
    const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
    const t = measureView(asset, state, solids, view, scale, undefined, f, Infinity, false, { core, box, cell });
    console.log(`[DEBUG] tile ${i},${j} x ${core.minX.toFixed(2)}..${core.maxX.toFixed(2)} y ${core.minY.toFixed(2)}..${core.maxY.toFixed(2)} h ${(t.hausdorffM * 1000).toFixed(0)} mm`);
    if (t.hausdorffM > worst.h) worst = { h: t.hausdorffM, core, box };
  }
const { box } = worst;
const t = measureView(asset, state, solids, view, scale, undefined, f, Infinity, false, { core: worst.core, box, cell });
const [vol, area, ink] = [new Mask(box, cell), new Mask(box, cell), new Mask(box, cell)];
for (const g of t.reference) vol.fill(g, true);
vol.seal();
for (const g of t.filled) area.fill(g);
for (const g of t.outlines) ink.fill(g);
const [W, H] = [vol.width, vol.height];
const holes = Math.round((4 * f / cell) ** 2);
const dm = area.cells.map((x, i) => x | ink.cells[i]!);
const [ed, ev] = [boundaryOf(fillSmallHoles(dm, W, H, holes), W, H), boundaryOf(fillSmallHoles(vol.cells, W, H, holes), W, H)];
const [fd, fv] = [distanceField(ed, W, H), distanceField(ev, W, H)];
const report = (name: string, from: Uint8Array, field: Float64Array): void => {
  const list: [number, number][] = [];
  from.forEach((x, k) => { if (x) list.push([Math.sqrt(field[k]!) * cell, k]); });
  list.sort((p, q) => q[0] - p[0]);
  console.log(`[DEBUG] ${name}: ` + list.slice(0, 5).map(([d, k]) => `${(d * 1000).toFixed(0)}mm@(${(box.minX + (k % W + 0.5) * cell).toFixed(3)},${(box.minY + (Math.floor(k / W) + 0.5) * cell).toFixed(3)})`).join(" "));
};
report("drawn edge far from body edge", ed, fv);
report("body edge far from drawn edge", ev, fd);
if (process.env.PNG) {
  const sharp = (await import("/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js")).default;
  const [cx, cy, half] = process.env.PNG.split(",").map(Number) as [number, number, number];
  const n = Math.round((2 * half) / cell);
  const px = new Uint8Array(n * n * 3);
  const fd2 = fillSmallHoles(dm, W, H, holes), fv2 = fillSmallHoles(vol.cells, W, H, holes);
  for (let j = 0; j < n; j++) for (let i = 0; i < n; i++) {
    const [x, y] = [cx - half + (i + 0.5) * cell, cy + half - (j + 0.5) * cell];
    const [a, b] = [Math.floor((x - box.minX) / cell), Math.floor((y - box.minY) / cell)];
    if (a < 0 || b < 0 || a >= W || b >= H) continue;
    const k = b * W + a;
    const o = (j * n + i) * 3;
    px[o] = dm[k] ? 0 : fd2[k] ? 120 : 255; px[o + 1] = vol.cells[k] && dm[k] ? 120 : 255; px[o + 2] = vol.cells[k] ? 0 : fv2[k] ? 120 : 255;
    if (!dm[k] && vol.cells[k]) { px[o] = 255; px[o + 1] = 0; px[o + 2] = 0; }
    if (dm[k] && !vol.cells[k]) { px[o] = 0; px[o + 1] = 0; px[o + 2] = 255; }
  }
  await sharp(px, { raw: { width: n, height: n, channels: 3 } }).resize({ width: 800, kernel: "nearest" }).png().toFile(process.env.OUT!);
}
