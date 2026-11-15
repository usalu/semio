/** 🧩️ Resolution-independent 2D comparison (rulings C7, C9): two shape sets are compared on a raster whose cell is at most a quarter of the smallest feature (a coarser one with `--fast`), tiled so that a 72 m crane costs the same per cell as a 0.3 m laptop; empty tiles are skipped and every tile carries a halo wide enough for the distances the rules look at.
 *
 * A set is `tris` (flat x0 y0 x1 y1 x2 y2 per triangle, even-odd, from tessellated solids and stroke quads) plus `polys` (closed drawn shapes by non-zero winding). Typed arrays only: nothing is allocated per triangle or per pixel.
 * Reports IoU, the IoU in which mismatch cells within `tolerance` of the other set do not count (C3), and per direction the maximum and a percentile of the distance of the boundary cells to the other set, capped at the halo.
 * @see ./design.md */
import { distanceField, Mask, type Box, type P2, type SolidMesh, type View, projectWorld } from "./volumes-lib.ts";

export type Poly = { rings: P2[][]; box: Box };
export type ShapeSet = { tris: number[]; polys: Poly[] };

export const emptySet = (): ShapeSet => ({ tris: [], polys: [] });

export function addPoly(set: ShapeSet, rings: P2[][]): void {
  const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const r of rings) for (const p of r) {
    if (p.x < box.minX) box.minX = p.x;
    if (p.x > box.maxX) box.maxX = p.x;
    if (p.y < box.minY) box.minY = p.y;
    if (p.y > box.maxY) box.maxY = p.y;
  }
  if (Number.isFinite(box.minX)) set.polys.push({ rings, box });
}

/** 📐️ The projected triangles of meshes in one view. */
export function addMeshes(set: ShapeSet, view: View, list: readonly SolidMesh[]): void {
  const o = projectWorld(view, [0, 0, 0]);
  const col = (k: number): [number, number] => {
    const e: [number, number, number] = [0, 0, 0];
    e[k] = 1;
    const p = projectWorld(view, e);
    return [p.x - o.x, p.y - o.y];
  };
  const [cx, cy, cz] = [col(0), col(1), col(2)];
  const R = [cx[0], cy[0], cz[0]];
  const U = [cx[1], cy[1], cz[1]];
  const V = [R[1]! * U[2]! - R[2]! * U[1]!, R[2]! * U[0]! - R[0]! * U[2]!, R[0]! * U[1]! - R[1]! * U[0]!];
  for (const m of list) {
    const closed = (m as { open?: boolean }).open !== true;
    const w = m.world;
    const n = m.world.length / 3;
    const xs = new Float64Array(n);
    const ys = new Float64Array(n);
    for (let i = 0; i < n; i++) {
      const p = projectWorld(view, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!]);
      xs[i] = p.x;
      ys[i] = p.y;
    }
    for (let t = 0; t < m.indices.length; t += 3) {
      const [a, b, c] = [m.indices[t]!, m.indices[t + 1]!, m.indices[t + 2]!];
      if (closed) {
        const [e1x, e1y, e1z, e2x, e2y, e2z] = [w[b * 3]! - w[a * 3]!, w[b * 3 + 1]! - w[a * 3 + 1]!, w[b * 3 + 2]! - w[a * 3 + 2]!, w[c * 3]! - w[a * 3]!, w[c * 3 + 1]! - w[a * 3 + 1]!, w[c * 3 + 2]! - w[a * 3 + 2]!];
        const facing = (e1y * e2z - e1z * e2y) * V[0]! + (e1z * e2x - e1x * e2z) * V[1]! + (e1x * e2y - e1y * e2x) * V[2]!;
        if (facing < 0) continue;
      }
      set.tris.push(xs[a]!, ys[a]!, xs[b]!, ys[b]!, xs[c]!, ys[c]!);
    }
  }
}

/** 🔺️ Triangles of a convex hull of 2D points (fan), for envelope comparisons. */
export function hullOf(points: readonly P2[]): P2[] {
  const pts = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  if (pts.length < 3) return pts;
  const cross = (o: P2, a: P2, b: P2) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [];
  for (const p of pts) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, p) <= 0) lower.pop();
    lower.push(p);
  }
  const upper: P2[] = [];
  for (const p of [...pts].reverse()) {
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, p) <= 0) upper.pop();
    upper.push(p);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** ✏️ A stroke of `width` metres along a polyline as one quad (two triangles) per segment, square caps. */
export function addStroke(set: ShapeSet, points: readonly P2[], width: number, closed: boolean): void {
  const h = width / 2;
  const n = closed ? points.length : points.length - 1;
  for (let i = 0; i < n; i++) {
    const [p, q] = [points[i]!, points[(i + 1) % points.length]!];
    const l = Math.hypot(q.x - p.x, q.y - p.y);
    const [ux, uy] = l < 1e-12 ? [1, 0] : [(q.x - p.x) / l, (q.y - p.y) / l];
    const [nx, ny] = [-uy * h, ux * h];
    const [px, py, qx, qy] = [p.x - ux * h, p.y - uy * h, q.x + ux * h, q.y + uy * h];
    set.tris.push(px + nx, py + ny, qx + nx, qy + ny, qx - nx, qy - ny, px + nx, py + ny, qx - nx, qy - ny, px - nx, py - ny);
  }
}

function fillTri(cells: Uint8Array, width: number, height: number, minX: number, minY: number, cell: number, x0: number, y0: number, x1: number, y1: number, x2: number, y2: number): void {
  const lo = Math.min(y0, y1, y2);
  const hi = Math.max(y0, y1, y2);
  const j0 = Math.max(0, Math.ceil((lo - minY) / cell - 0.5));
  const j1 = Math.min(height - 1, Math.floor((hi - minY) / cell - 0.5));
  for (let j = j0; j <= j1; j++) {
    const y = minY + (j + 0.5) * cell;
    let xa = Infinity;
    let xb = -Infinity;
    if (y0 <= y !== y1 <= y) { const x = x0 + ((y - y0) / (y1 - y0)) * (x1 - x0); if (x < xa) xa = x; if (x > xb) xb = x; }
    if (y1 <= y !== y2 <= y) { const x = x1 + ((y - y1) / (y2 - y1)) * (x2 - x1); if (x < xa) xa = x; if (x > xb) xb = x; }
    if (y2 <= y !== y0 <= y) { const x = x2 + ((y - y2) / (y0 - y2)) * (x0 - x2); if (x < xa) xa = x; if (x > xb) xb = x; }
    if (xb < xa) continue;
    const i0 = Math.max(0, Math.ceil((xa - minX) / cell - 0.5));
    const i1 = Math.min(width - 1, Math.ceil((xb - minX) / cell - 0.5) - 1);
    for (let i = i0; i <= i1; i++) cells[j * width + i] = 1;
  }
}

function fillNonZero(mask: Mask, rings: readonly P2[][]): void {
  const { box, cell, width, height, cells } = mask;
  let lo = Infinity;
  let hi = -Infinity;
  for (const r of rings) for (const p of r) {
    if (p.y < lo) lo = p.y;
    if (p.y > hi) hi = p.y;
  }
  const j0 = Math.max(0, Math.ceil((lo - box.minY) / cell - 0.5));
  const j1 = Math.min(height - 1, Math.floor((hi - box.minY) / cell - 0.5));
  const xs: [number, number][] = [];
  for (let j = j0; j <= j1; j++) {
    const y = box.minY + (j + 0.5) * cell;
    xs.length = 0;
    for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const [a, b] = [ring[k]!, ring[i]!];
      if (a.y <= y !== b.y <= y) xs.push([a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x), b.y > a.y ? 1 : -1]);
    }
    xs.sort((p, q) => p[0] - q[0]);
    let wind = 0;
    for (let n = 0; n + 1 < xs.length; n++) {
      wind += xs[n]![1];
      if (wind === 0) continue;
      const i0 = Math.max(0, Math.ceil((xs[n]![0] - box.minX) / cell - 0.5));
      const i1 = Math.min(width - 1, Math.ceil((xs[n + 1]![0] - box.minX) / cell - 0.5) - 1);
      for (let i = i0; i <= i1; i++) cells[j * width + i] = 1;
    }
  }
}

const ZERO = { length: 0 } as unknown as Float32Array;
export type TileResult = { iou: number; tolerantIou: number; maxAB: number; maxBA: number; pAB: number; pBA: number; capM: number; cell: number; tiles: number; cellsA: number; cellsB: number };
export type TileOptions = { cell: number; tolerance?: number; capM?: number; tile?: number; percentile?: number };

/** 🔬️ Compares set A with set B tile by tile; distances are `Infinity` when one set is empty and the other is not. */
export function tileCompare(A: ShapeSet, B: ShapeSet, options: TileOptions): TileResult {
  const T0 = Date.now();
  const cell = options.cell;
  const tile = options.tile ?? 768;
  const tol = options.tolerance ?? 0;
  const q = options.percentile ?? 0.95;
  const capM = Math.max(options.capM ?? 0, tol * 1.5, cell * 4);
  const halo = Math.ceil(capM / cell) + 1;
  const total = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  const grow = (x: number, y: number) => {
    if (x < total.minX) total.minX = x;
    if (x > total.maxX) total.maxX = x;
    if (y < total.minY) total.minY = y;
    if (y > total.maxY) total.maxY = y;
  };
  for (const s of [A, B]) {
    for (let i = 0; i < s.tris.length; i += 2) grow(s.tris[i]!, s.tris[i + 1]!);
    for (const p of s.polys) {
      grow(p.box.minX, p.box.minY);
      grow(p.box.maxX, p.box.maxY);
    }
  }
  const nA = A.tris.length + A.polys.length;
  const nB = B.tris.length + B.polys.length;
  const base: TileResult = { iou: 1, tolerantIou: 1, maxAB: 0, maxBA: 0, pAB: 0, pBA: 0, capM, cell, tiles: 0, cellsA: 0, cellsB: 0 };
  if (!Number.isFinite(total.minX)) return base;
  if (nA === 0 || nB === 0) return { ...base, iou: 0, tolerantIou: 0, maxAB: nB === 0 ? Infinity : 0, maxBA: nA === 0 ? Infinity : 0, pAB: nB === 0 ? Infinity : 0, pBA: nA === 0 ? Infinity : 0 };
  const span = tile * cell;
  const nx = Math.max(1, Math.ceil((total.maxX - total.minX) / span));
  const ny = Math.max(1, Math.ceil((total.maxY - total.minY) / span));
  const h = halo * cell;
  type Bucket = { ta: number[]; tb: number[]; pa: number[]; pb: number[] };
  const buckets = new Map<number, Bucket>();
  const get = (key: number): Bucket => buckets.get(key) ?? buckets.set(key, { ta: [], tb: [], pa: [], pb: [] }).get(key)!;
  const range = (x0: number, y0: number, x1: number, y1: number, f: (key: number) => void) => {
    const i0 = Math.max(0, Math.floor((x0 - h - total.minX) / span));
    const i1 = Math.min(nx - 1, Math.floor((x1 + h - total.minX) / span));
    const j0 = Math.max(0, Math.floor((y0 - h - total.minY) / span));
    const j1 = Math.min(ny - 1, Math.floor((y1 + h - total.minY) / span));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) f(j * nx + i);
  };
  const bucketTris = (s: ShapeSet, which: "ta" | "tb") => {
    const t = s.tris;
    for (let k = 0; k < t.length; k += 6) {
      const x0 = Math.min(t[k]!, t[k + 2]!, t[k + 4]!);
      const x1 = Math.max(t[k]!, t[k + 2]!, t[k + 4]!);
      const y0 = Math.min(t[k + 1]!, t[k + 3]!, t[k + 5]!);
      const y1 = Math.max(t[k + 1]!, t[k + 3]!, t[k + 5]!);
      range(x0, y0, x1, y1, (key) => get(key)[which].push(k));
    }
  };
  bucketTris(A, "ta");
  bucketTris(B, "tb");
  A.polys.forEach((p, i) => range(p.box.minX, p.box.minY, p.box.maxX, p.box.maxY, (key) => get(key).pa.push(i)));
  B.polys.forEach((p, i) => range(p.box.minX, p.box.minY, p.box.maxX, p.box.maxY, (key) => get(key).pb.push(i)));
  let both = 0;
  let either = 0;
  let forgiven = 0;
  let cellsA = 0;
  let cellsB = 0;
  let maxAB = 0;
  let maxBA = 0;
  const bins = Math.ceil(capM / cell) + 2;
  const histAB = new Float64Array(bins);
  const histBA = new Float64Array(bins);
  let boundA = 0;
  let boundB = 0;
  let tiles = 0;
  const prof = { bucket: Date.now() - T0, fill: 0, dist: 0, scan: 0 };
  for (const [key, bk] of buckets) {
    let t1 = Date.now();
    const [ti, tj] = [key % nx, Math.floor(key / nx)];
    const minX = total.minX + ti * span - h;
    const minY = total.minY + tj * span - h;
    const box: Box = { minX, minY, maxX: minX + (tile + 2 * halo) * cell, maxY: minY + (tile + 2 * halo) * cell };
    const [a, b] = [new Mask(box, cell), new Mask(box, cell)];
    const w = a.width;
    const ht = a.height;
    for (const k of bk.ta) fillTri(a.cells, w, ht, minX, minY, cell, A.tris[k]!, A.tris[k + 1]!, A.tris[k + 2]!, A.tris[k + 3]!, A.tris[k + 4]!, A.tris[k + 5]!);
    for (const k of bk.tb) fillTri(b.cells, w, ht, minX, minY, cell, B.tris[k]!, B.tris[k + 1]!, B.tris[k + 2]!, B.tris[k + 3]!, B.tris[k + 4]!, B.tris[k + 5]!);
    for (const i of bk.pa) fillNonZero(a, A.polys[i]!.rings);
    for (const i of bk.pb) fillNonZero(b, B.polys[i]!.rings);
    prof.fill += Date.now() - t1;
    t1 = Date.now();
    let mismatch = false;
    for (let j = halo - 1; j < Math.min(ht, halo + tile + 1) && !mismatch; j++) for (let i = halo - 1; i < Math.min(w, halo + tile + 1); i++) if (a.cells[j * w + i]! !== b.cells[j * w + i]!) { mismatch = true; break; }
    const da = mismatch ? distanceField(a) : ZERO;
    const db = mismatch ? distanceField(b) : ZERO;
    prof.dist += Date.now() - t1;
    t1 = Date.now();
    tiles++;
    const jEnd = Math.min(ht - halo, halo + tile);
    const iEnd = Math.min(w - halo, halo + tile);
    for (let j = halo; j < jEnd; j++) for (let i = halo; i < iEnd; i++) {
      const k = j * w + i;
      const [x, y] = [a.cells[k]!, b.cells[k]!];
      if (x) cellsA++;
      if (y) cellsB++;
      if (x && y) {
        both++;
        either++;
      } else if (x) {
        if (tol > 0 && (db === ZERO ? 0 : db[k]!) <= tol) forgiven++;
        else either++;
      } else if (y) {
        if (tol > 0 && (da === ZERO ? 0 : da[k]!) <= tol) forgiven++;
        else either++;
      }
      if (x && (a.cells[k - 1]! === 0 || a.cells[k + 1]! === 0 || a.cells[k - w]! === 0 || a.cells[k + w]! === 0)) {
        const d = Math.min((db === ZERO ? 0 : db[k]!), capM);
        if (d > maxAB) maxAB = d;
        histAB[Math.min(bins - 1, Math.floor(d / cell))]!++;
        boundA++;
      }
      if (y && (b.cells[k - 1]! === 0 || b.cells[k + 1]! === 0 || b.cells[k - w]! === 0 || b.cells[k + w]! === 0)) {
        const d = Math.min((da === ZERO ? 0 : da[k]!), capM);
        if (d > maxBA) maxBA = d;
        histBA[Math.min(bins - 1, Math.floor(d / cell))]!++;
        boundB++;
      }
    }
  }
  if (process.env.VOLUMES_PROFILE) console.error("[tiles]", JSON.stringify(prof), "tiles", tiles);
  const pct = (hist: Float64Array, n: number): number => {
    if (n === 0) return 0;
    let acc = 0;
    for (let i = 0; i < hist.length; i++) {
      acc += hist[i]!;
      if (acc >= q * n) return i * cell;
    }
    return capM;
  };
  const plain = either + forgiven;
  return { iou: plain === 0 ? 1 : both / plain, tolerantIou: either === 0 ? 1 : both / either, maxAB: Math.max(0, maxAB - cell), maxBA: Math.max(0, maxBA - cell), pAB: Math.max(0, pct(histAB, boundA) - cell), pBA: Math.max(0, pct(histBA, boundB) - cell), capM, cell, tiles, cellsA, cellsB };
}

/** 🖼️ A shape set on a raster (for overlays): triangles even-odd, polygons non-zero. */
export function rasterize(set: ShapeSet, box: Box, cell: number): Mask {
  const m = new Mask(box, cell);
  for (let k = 0; k < set.tris.length; k += 6) fillTri(m.cells, m.width, m.height, box.minX, box.minY, cell, set.tris[k]!, set.tris[k + 1]!, set.tris[k + 2]!, set.tris[k + 3]!, set.tris[k + 4]!, set.tris[k + 5]!);
  for (const p of set.polys) fillNonZero(m, p.rings);
  return m;
}
