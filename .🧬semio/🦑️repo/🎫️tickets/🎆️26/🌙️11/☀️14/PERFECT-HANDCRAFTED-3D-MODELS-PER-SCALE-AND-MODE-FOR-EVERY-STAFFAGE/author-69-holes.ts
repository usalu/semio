/** 🕳️ Enclosed holes (cells not reachable from the border) of the plan/front/elevation mask of the body of one scale: bun author-69-holes.ts <asset> <scale> [view] */
import { poseState, viewMasks, entryOf, loadSource, merged, scaleMeshes, meshMask, projectedPoints, gridFor, VIEWS } from "./volumes-lib.ts";
const [spec, sc, only] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, Number(sc), m.volumes[sc]);
for (const view of VIEWS) {
  if (only && view !== only) continue;
  const pts = projectedPoints(view, sm.body);
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const p of pts) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  const feature = 0.0075 * Number(sc) / 50 * 2;
  const cell = feature / 8;
  const g = { box: { minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell }, cell };
  const vm = viewMasks(view, poseState(m, Number(sc)), sm.body);
  for (const [label, mask] of [["3D", vm.volume], ["2D", vm.truth]] as const) {
    const g = { box: vm.box, cell: vm.cell };
    const cell = g.cell;
  const { width: w, height: h, cells } = mask;
  const seen = new Uint8Array(w * h);
  const stack: number[] = [];
  const push = (k: number) => { if (!cells[k] && !seen[k]) { seen[k] = 1; stack.push(k); } };
  for (let i = 0; i < w; i++) { push(i); push((h - 1) * w + i); }
  for (let j = 0; j < h; j++) { push(j * w); push(j * w + w - 1); }
  while (stack.length) { const k = stack.pop()!; const i = k % w, j = (k / w) | 0; if (i > 0) push(k - 1); if (i < w - 1) push(k + 1); if (j > 0) push(k - w); if (j < h - 1) push(k + w); }
  const lab = new Int32Array(w * h).fill(-1);
  let n = 0;
  for (let k0 = 0; k0 < w * h; k0++) {
    if (cells[k0] || seen[k0] || lab[k0] >= 0) continue;
    let area = 0, sx = 0, sy = 0, x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9; const st = [k0]; lab[k0] = n;
    while (st.length) { const k = st.pop()!; area++; sx += k % w; sy += (k / w) | 0; x0 = Math.min(x0, k % w); x1 = Math.max(x1, k % w); y0 = Math.min(y0, (k / w) | 0); y1 = Math.max(y1, (k / w) | 0); for (const q of [k - 1, k + 1, k - w, k + w]) if (q >= 0 && q < w * h && !cells[q] && !seen[q] && lab[q] < 0) { lab[q] = n; st.push(q); } }
    n++;
    console.log(`1:${sc} ${view} ${label} hole ${n}: ${area} cells = ${(area * cell * cell * 1e4).toFixed(1)} cm2 bbox x ${(g.box.minX + x0 * cell).toFixed(3)}..${(g.box.minX + x1 * cell).toFixed(3)} y ${(g.box.minY + y0 * cell).toFixed(3)}..${(g.box.minY + y1 * cell).toFixed(3)} at (${(g.box.minX + (sx / area) * cell).toFixed(3)}, ${(g.box.minY + (sy / area) * cell).toFixed(3)})`);
  }
  }
}
