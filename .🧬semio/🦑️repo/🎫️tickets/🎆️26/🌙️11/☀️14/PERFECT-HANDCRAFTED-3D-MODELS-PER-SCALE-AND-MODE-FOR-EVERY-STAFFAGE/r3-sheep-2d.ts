/** ✏️ Round 3 sheep, ruling E1: redraws every 2D shape of every scale and view as the true orthographic outline of the body solids that the model files assign to it (same ids, roles, bones, order), simplified to a fifth of the scale's smallest feature.
 * Pass 1 writes the outlines in view coordinates, pass 2 maps them into each shape's own frame through the evaluated drawing (affine fit). Usage: bun r3-sheep-2d.ts <1|2>   then python3 r3-sheep-2d.py
 * @see ./r3-sheep-2d.py */
import { readFileSync, writeFileSync } from "node:fs";
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
import { polyMap, type P } from "./fauna-affine.ts";
import { OUT, SPEC } from "./r3-sheep-kit.ts";

const pass = Number(process.argv[2]);
const AX = { elevation: [0, 1], front: [2, 1], plan: [0, 2] } as const;
const GEOMETRY_KEYS = ["cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "d", "solid", "segments", "w", "h", "x", "y", "r", "rotationDeg", "width", "height"];

function outline(meshes: { world: Float64Array; indices: Uint32Array }[], a: number, b: number, cell: number, tol: number): P[] {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) { const x = m.world[i + a]!; const y = m.world[i + b]!; x0 = Math.min(x0, x); x1 = Math.max(x1, x); y0 = Math.min(y0, y); y1 = Math.max(y1, y); }
  x0 -= 2 * cell; y0 -= 2 * cell; x1 += 2 * cell; y1 += 2 * cell;
  const W = Math.ceil((x1 - x0) / cell);
  const H = Math.ceil((y1 - y0) / cell);
  const g = new Uint8Array(W * H);
  for (const m of meshes) for (let t = 0; t < m.indices.length; t += 3) {
    const p = [0, 1, 2].map((k) => { const i = m.indices[t + k]! * 3; return [(m.world[i + a]! - x0) / cell, (y1 - m.world[i + b]!) / cell] as P; });
    const [A, B, C] = p as [P, P, P];
    const d = (B[0] - A[0]) * (C[1] - A[1]) - (B[1] - A[1]) * (C[0] - A[0]);
    if (Math.abs(d) < 1e-12) continue;
    const [ix0, ix1] = [Math.max(0, Math.floor(Math.min(A[0], B[0], C[0]))), Math.min(W - 1, Math.ceil(Math.max(A[0], B[0], C[0])))];
    const [iy0, iy1] = [Math.max(0, Math.floor(Math.min(A[1], B[1], C[1]))), Math.min(H - 1, Math.ceil(Math.max(A[1], B[1], C[1])))];
    for (let y = iy0; y <= iy1; y++) for (let x = ix0; x <= ix1; x++) {
      const px = x + 0.5; const py = y + 0.5;
      const u = ((B[0] - A[0]) * (py - A[1]) - (B[1] - A[1]) * (px - A[0])) / d;
      const v = ((px - A[0]) * (C[1] - A[1]) - (py - A[1]) * (C[0] - A[0])) / d;
      if (u >= -1e-9 && v >= -1e-9 && u + v <= 1 + 1e-9) g[y * W + x] = 1;
    }
  }
  const label = new Int32Array(W * H);
  let best = 0; let bestN = 0; let next = 0;
  for (let s = 0; s < W * H; s++) {
    if (g[s] !== 1 || label[s] !== 0) continue;
    next++;
    let n = 0;
    const stack = [s];
    label[s] = next;
    while (stack.length > 0) { const q = stack.pop()!; n++; const qx = q % W; for (const r of [q - 1, q + 1, q - W, q + W]) { if (r < 0 || r >= W * H || g[r] !== 1 || label[r] !== 0) continue; if ((r === q - 1 && qx === 0) || (r === q + 1 && qx === W - 1)) continue; label[r] = next; stack.push(r); } }
    if (n > bestN) { bestN = n; best = next; }
  }
  const on = (x: number, y: number): boolean => x >= 0 && y >= 0 && x < W && y < H && label[y * W + x] === best;
  let sx = 0; let sy = 0;
  search: for (sy = 0; sy < H; sy++) for (sx = 0; sx < W; sx++) if (on(sx, sy)) break search;
  const ring: P[] = [];
  let [cx, cy, dx, dy] = [sx, sy, 1, 0];
  for (let guard = 0; guard < 8 * W * H; guard++) {
    ring.push([cx, cy]);
    cx += dx; cy += dy;
    const right = dx === 1 ? on(cx, cy) : dy === 1 ? on(cx - 1, cy) : dx === -1 ? on(cx - 1, cy - 1) : on(cx, cy - 1);
    const left = dx === 1 ? on(cx, cy - 1) : dy === 1 ? on(cx, cy) : dx === -1 ? on(cx - 1, cy) : on(cx - 1, cy - 1);
    if (left) [dx, dy] = [dy, -dx];
    else if (!right) [dx, dy] = [-dy, dx];
    if (cx === sx && cy === sy && dx === 1 && dy === 0) break;
  }
  const pts = ring.map(([x, y]) => [x0 + x * cell, y1 - y * cell] as P);
  const dp = (q: P[], lo: number, hi: number, keep: Set<number>): void => {
    let worst = 0; let wi = -1;
    const [ax, ay] = q[lo]!; const [bx, by] = q[hi]!;
    const l = Math.hypot(bx - ax, by - ay) || 1e-12;
    for (let i = lo + 1; i < hi; i++) { const e = Math.abs((q[i]![0] - ax) * (by - ay) - (q[i]![1] - ay) * (bx - ax)) / l; if (e > worst) { worst = e; wi = i; } }
    if (worst > tol && wi > 0) { keep.add(wi); dp(q, lo, wi, keep); dp(q, wi, hi, keep); }
  };
  let far = 0;
  for (let i = 1; i < pts.length; i++) if (Math.hypot(pts[i]![0] - pts[0]![0], pts[i]![1] - pts[0]![1]) > Math.hypot(pts[far]![0] - pts[0]![0], pts[far]![1] - pts[0]![1])) far = i;
  const loop = [...pts, pts[0]!];
  const keep = new Set([0, far]);
  dp(loop, 0, far, keep);
  dp(loop, far, loop.length - 1, keep);
  return [...keep].sort((p, q) => p - q).map((i) => [Math.round(pts[i]![0] * 1e5) / 1e5, Math.round(pts[i]![1] * 1e5) / 1e5] as P);
}

const ops: unknown[] = [];
let worst = 0;
for (const scale of [50, 100, 200, 500]) {
  const c = openCtx(SPEC, scale);
  const map = JSON.parse(readFileSync(`${OUT}/shapes.${scale}.json`, "utf8")) as Record<string, string>;
  const meshes = meshesAt(c.m, scale, c.vol.body);
  const groups: Record<string, typeof meshes> = {};
  for (const m of meshes) if (map[m.id] !== undefined) (groups[map[m.id]!] ??= []).push(m);
  for (const view of ["elevation", "front", "plan"] as const) {
    for (const shape of c.raw.representations[String(scale)].geometry[view] as any[]) {
      if (shape.id === "mouth-line") { if (pass === 1) ops.push({ scale, view, id: shape.id, set: { points: [[0.224, -0.008], [0.19, -0.0085], [0.155, -0.006], [0.128, -0.001]] } }); continue; }
      const ms = groups[shape.id];
      if (ms === undefined) { console.log(`no solids for ${scale} ${view} ${shape.id}`); continue; }
      const world = outline(ms, AX[view][0], AX[view][1], c.feature / 12, c.feature * 0.2);
      if (pass === 1) ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: true, points: world }, del: GEOMETRY_KEYS });
      else {
        const m = polyMap(c, view, shape.id, shape.points);
        worst = Math.max(worst, m.err);
        if (m.err > 1e-4) console.log(`affine error ${m.err.toFixed(5)} at ${scale} ${view} ${shape.id}`);
        ops.push({ scale, view, id: shape.id, set: { points: m.toLocal(world) } });
      }
    }
  }
}
writeFileSync(`${OUT}/ops2d.json`, JSON.stringify(ops));
console.log(`pass ${pass}: ${ops.length} shapes, worst affine error ${worst.toFixed(6)}`);
