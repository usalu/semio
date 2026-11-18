/** 🔬 Tiled mismatch blobs of tower-crane at one scale and view: reproduces the banded measurement and lists the connected mismatch blobs (farther than the limit) of every failing tile with the solids and shapes nearby.
 *  bun crane-tiles.ts <scale> <view,view> [asset=tower-crane] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids, project, TILE_CELLS, WINDOW_FEATURES, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [scaleArg, views, nameArg] = process.argv.slice(2) as [string, string, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === (nameArg ?? "tower-crane"))!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const limit = 2 * feature;
for (const view of views.split(",") as Viewpoint[]) {
  const coarse = measureView(asset, state, solids, view, scale, 2000, feature, Infinity, true);
  const edge = coarse.edge!;
  const cell = feature / 4;
  const side = TILE_CELLS * cell;
  const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
  const frame = { minX: edge.minX, minY: edge.minY, maxX: edge.minX + edge.width * edge.cell, maxY: edge.minY + edge.height * edge.cell };
  const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
  console.log(`== ${view} 1:${scale} coarse iou ${coarse.iou.toFixed(3)} tiles ${nx}x${ny} limit ${limit}`);
  const hasEdge = (b: any): boolean => {
    const [i0, i1] = [Math.max(0, Math.floor((b.minX - edge.minX) / edge.cell)), Math.min(edge.width - 1, Math.floor((b.maxX - edge.minX) / edge.cell))];
    const [j0, j1] = [Math.max(0, Math.floor((b.minY - edge.minY) / edge.cell)), Math.min(edge.height - 1, Math.floor((b.maxY - edge.minY) / edge.cell))];
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) if (edge.mask[j * edge.width + i]) return true;
    return false;
  };
  const shapes = (state.shapes[view] as any[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0);
  const shapeBoxes = shapes.map((s) => { const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); return { id: String(s.shape.id), x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys), filled: s.shape.filled !== false }; });
  const solidBoxes = solids.map((s) => {
    const xs: number[] = [], ys: number[] = [];
    for (let i = 0; i < s.positions.length; i += 3) { const p = project(view, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); xs.push(p.x); ys.push(p.y); }
    return { id: s.id, slender: s.slender === true, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) };
  });
  let worst = 0;
  const only = process.env.COLS?.split(",").map(Number);
  for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
    if (only !== undefined && (i < only[0]! || i > only[1]!)) continue;
    const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
    const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
    const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
    if (!hasEdge({ minX: core.minX - edge.cell, minY: core.minY - edge.cell, maxX: core.maxX + edge.cell, maxY: core.maxY + edge.cell })) continue;
    const t = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false, { core, box, cell });
    worst = Math.max(worst, t.hausdorffM);
    if (t.hausdorffM <= limit) continue;
    console.log(`tile ${i},${j} core x ${core.minX.toFixed(2)}..${core.maxX.toFixed(2)} y ${core.minY.toFixed(2)}..${core.maxY.toFixed(2)} h ${t.hausdorffM.toFixed(4)}`);
    const drawn = new Mask(box, cell);
    for (const g of t.filled) drawn.fill(g);
    for (const g of t.outlines) drawn.fill(g);
    const vol = new Mask(box, cell);
    for (const g of t.reference) vol.fill(g, true);
    if (t.section) for (const g of t.visible) vol.fill(g, true);
    vol.seal();
    const [w, h] = [vol.width, vol.height];
    const holes = Math.round((4 * feature / cell) ** 2);
    const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
    const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
    const toVol = distanceField(bv, w, h);
    const toDrawn = distanceField(bd, w, h);
    const ci = { i0: Math.round((core.minX - box.minX) / cell), i1: Math.round((core.maxX - box.minX) / cell), j0: Math.round((core.minY - box.minY) / cell), j1: Math.round((core.maxY - box.minY) / cell) };
    const patch = process.env.PATCH?.split(",").map(Number);
    if (patch !== undefined && patch[0]! >= core.minX && patch[0]! < core.maxX && patch[1]! >= core.minY && patch[1]! < core.maxY) {
      const r = patch[2]!;
      const ry = patch[3] ?? r;
      const step = Math.max(1, Math.round((2 * r) / cell / 100));
      const stepY = patch[3] === undefined ? step * 2 : 1;
      for (let y = patch[1]! + ry; y > patch[1]! - ry; y -= stepY * cell) {
        let line = "";
        for (let x = patch[0]! - r; x < patch[0]! + r; x += step * cell) {
          const ii = Math.floor((x - box.minX) / cell), jj = Math.floor((y - box.minY) / cell);
          const [a, b] = [drawn.cells[jj * w + ii], vol.cells[jj * w + ii]];
          line += a && b ? "#" : a ? "D" : b ? "V" : ".";
        }
        console.log(line);
      }
    }
    for (const [side2, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
      const flag = new Uint8Array(w * h);
      for (let jj = ci.j0; jj < ci.j1; jj++) for (let ii = ci.i0; ii < ci.i1; ii++) if (from[jj * w + ii] && Math.sqrt(field[jj * w + ii]!) * cell > limit) flag[jj * w + ii] = 1;
      const seen = new Uint8Array(w * h);
      const reach = Math.max(1, Math.round((limit * 0.75) / cell));
      const blobs: { i0: number; i1: number; j0: number; j1: number; d: number; n: number; wi: number; wj: number }[] = [];
      for (let jj = 0; jj < h; jj++) for (let ii = 0; ii < w; ii++) {
        if (!flag[jj * w + ii] || seen[jj * w + ii]) continue;
        const b = { i0: ii, i1: ii, j0: jj, j1: jj, d: 0, n: 0, wi: ii, wj: jj };
        const stack = [[ii, jj]];
        seen[jj * w + ii] = 1;
        while (stack.length > 0) {
          const [ci2, cj] = stack.pop()!;
          b.i0 = Math.min(b.i0, ci2!); b.i1 = Math.max(b.i1, ci2!); b.j0 = Math.min(b.j0, cj!); b.j1 = Math.max(b.j1, cj!); b.n++;
          { const dd = Math.sqrt(field[cj! * w + ci2!]!) * cell; if (dd > b.d) { b.d = dd; b.wi = ci2!; b.wj = cj!; } }
          for (let dj = -reach; dj <= reach; dj++) for (let di = -reach; di <= reach; di++) {
            const [ni, nj] = [ci2! + di, cj! + dj];
            if (ni < 0 || nj < 0 || ni >= w || nj >= h || !flag[nj * w + ni] || seen[nj * w + ni]) continue;
            seen[nj * w + ni] = 1;
            stack.push([ni, nj]);
          }
        }
        blobs.push(b);
      }
      blobs.sort((a, b) => b.d - a.d);
      for (const b of blobs.slice(0, 8)) {
        const [x0, x1, y0, y1] = [box.minX + b.i0 * cell, box.minX + (b.i1 + 1) * cell, box.minY + b.j0 * cell, box.minY + (b.j1 + 1) * cell];
        const near = (o: { x0: number; x1: number; y0: number; y1: number }) => o.x1 >= x0 - limit && o.x0 <= x1 + limit && o.y1 >= y0 - limit && o.y0 <= y1 + limit;
        const list = side2 === "volume-only" ? solidBoxes.filter(near).map((o) => `${o.id}${o.slender ? "*" : ""}[${o.x0.toFixed(2)},${o.x1.toFixed(2)}|${o.y0.toFixed(2)},${o.y1.toFixed(2)}]`) : shapeBoxes.filter(near).map((o) => `${o.id}${o.filled ? "" : "~"}[${o.x0.toFixed(2)},${o.x1.toFixed(2)}|${o.y0.toFixed(2)},${o.y1.toFixed(2)}]`);
        console.log(`  ${side2} d=${b.d.toFixed(3)} worst@(${(box.minX + b.wi * cell).toFixed(3)},${(box.minY + b.wj * cell).toFixed(3)}) n=${b.n} x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)} near: ${list.slice(0, Number(process.env.NEAR ?? 6)).join(" ")}`);
      }
    }
  }
  console.log(`== ${view} 1:${scale} H ${worst.toFixed(4)} limit ${limit}`);
}
