/** 🧱️ (after `🧰️silhouette/SC/tiles.ts`) The tiles of a view as the guard measures a large object (512 cells a side, a margin of six smallest features) whose boundary distance exceeds the limit: the core box of each tile and its distance, and the clusters of far boundary cells inside the tile window (D drawing, V volume).; with the nearest drawn shapes and solids of each worst place.
 *  bun tiles.ts <id|file.json> <scale> <view> */
import { S, fileOf } from "./files.ts";
const { boundaryOf, fillSmallHoles, measureView, worldSolids, distanceField, Mask, TILE_CELLS, WINDOW_FEATURES } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { project } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);

const [name, scaleArg, view] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale), limit = 2 * feature;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const whole = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const real = measureView(asset, state, solids, view as never, scale, undefined, feature);
const { cell, box: frame } = whole;
console.log(`${asset.id} 1:${scale} ${view}: guard H ${(real.hausdorffM * 1000).toFixed(1)} mm, one raster H ${(whole.hausdorffM * 1000).toFixed(1)} mm, cell ${(cell * 1000).toFixed(2)} mm`);
const side = TILE_CELLS * cell;
const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
for (let j = 0; j < Math.ceil((frame.maxY - frame.minY) / side); j++) for (let i = 0; i < Math.ceil((frame.maxX - frame.minX) / side); i++) {
  const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
  const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
  const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
  const tile = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false, { core, box, cell });
  if (!(tile.hausdorffM > limit)) continue;
  console.log(`  tile ${i},${j} core x ${core.minX.toFixed(3)}..${core.maxX.toFixed(3)} y ${core.minY.toFixed(3)}..${core.maxY.toFixed(3)} H ${(tile.hausdorffM * 1000).toFixed(1)} mm`);
  const drawn = new Mask(box, cell), vol = new Mask(box, cell);
  for (const g of tile.filled) drawn.fill(g);
  for (const g of tile.outlines) drawn.fill(g);
  for (const g of tile.reference) vol.fill(g, true);
  if (tile.section) for (const g of tile.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const [ci0, ci1, cj0, cj1] = [Math.round((core.minX - box.minX) / cell), Math.round((core.maxX - box.minX) / cell), Math.round((core.minY - box.minY) / cell), Math.round((core.maxY - box.minY) / cell)];
  for (const [label, from, to] of [["D", bd, bv], ["V", bv, bd]] as const) {
    const field = distanceField(to, w, h);
    let [n, worst, wx, wy, x0, x1, y0, y1] = [0, 0, 0, 0, Infinity, -Infinity, Infinity, -Infinity];
    for (let b = cj0; b < cj1; b++) for (let a = ci0; a < ci1; a++) {
      const k = b * w + a;
      if (!from[k]) continue;
      const d = Math.sqrt(field[k]!) * cell;
      if (!(d > limit)) continue;
      const [x, y] = [box.minX + (a + 0.5) * cell, box.minY + (b + 0.5) * cell];
      n++;
      if (d > worst) [worst, wx, wy] = [d, x, y];
      [x0, x1, y0, y1] = [Math.min(x0, x), Math.max(x1, x), Math.min(y0, y), Math.max(y1, y)];
    }
    if (n > 0) {
      const p = { x: wx, y: wy };
      const dseg = (a: any, b: any): number => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let u = l === 0 ? 0 : ((p.x - a.x) * dx + (p.y - a.y) * dy) / l; u = Math.max(0, Math.min(1, u)); return Math.hypot(p.x - a.x - u * dx, p.y - a.y - u * dy); };
      const near = (state.shapes[view!] as any[]).map((s) => ({ id: `${s.shape.id}@${s.shape.bone}[${s.shape.role}${s.shape.filled === false ? ",open" : ""}]`, d: Math.min(...s.rings.flatMap((r: any[]) => r.map((q, k) => dseg(q, r[(k + 1) % r.length])))) })).filter((x) => Number.isFinite(x.d)).sort((a, b) => a.d - b.d).slice(0, 4);
      const nearV = solids.map((s: any) => { let m = Infinity; for (let k = 0; k < s.positions.length; k += 3) { const q = project(view as never, s.positions[k], s.positions[k + 1], s.positions[k + 2]); m = Math.min(m, Math.hypot(q.x - p.x, q.y - p.y)); } return { id: `${s.id}@${s.bone}`, d: m }; }).sort((a: any, b: any) => a.d - b.d).slice(0, 4);
      console.log(`      shapes ${near.map((x) => `${x.id} ${(x.d * 1000).toFixed(0)}`).join(", ")} | solids ${nearV.map((x: any) => `${x.id} ${(x.d * 1000).toFixed(0)}`).join(", ")}`);
    }
    if (n > 0) console.log(`    ${label} ${n} cells beyond the limit, worst ${(worst * 1000).toFixed(1)} mm at (${wx.toFixed(4)}, ${wy.toFixed(4)}), within x ${x0.toFixed(4)}..${x1.toFixed(4)} y ${y0.toFixed(4)}..${y1.toFixed(4)}`);
  }
}
