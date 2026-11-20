/** ✏️ Round 3 red fox, ruling E1: redraws every 2D shape of every authored scale and view as the true orthographic outline of the body solids that the model files assign to it (same ids, roles, bones, order, tones), simplified to a quarter of the scale's smallest feature.
 * The fox bends its spine, neck, head and brush sideways when it curls up, so the validator demands solids on those bones in every view. Each view therefore states its own outline as a solid hint that draws exactly that outline in the default pose: elevation = profile in the bone plane (extruded over the lateral span on bones that leave the plane), front = a meridian profile (`revolved` without mirror about the bone's own axis), plan = a 1 mm slab in the (u, w) plane (`extruded`, sweep v); upright limb segments and ears, whose plan slab would be seen edge-on, are a tapered prism over the side profile in the plan and, on bones that leave the plane, in the elevation.
 * Lateral members are stated relative to the lateral position of the shape's bone (round 4: the limb and ear bones carry `z`).
 * Usage: bun r3-red-fox-2d.ts   then python3 r3-red-fox-2d.py [--apply]
 * @see ./r3-red-fox-2d.py */
import { readFileSync, writeFileSync } from "node:fs";
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
import { OUT, SPEC } from "./r3-red-fox-kit.ts";

type P = [number, number];
const AX = { elevation: [0, 1], front: [2, 1], plan: [0, 2] } as const;
const GEOMETRY_KEYS = ["cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "d", "solid", "segments", "w", "h", "x", "y", "r", "rotationDeg", "width", "height", "followTip", "points", "closed"];
/** Solids of a shape that a view does not draw go into the outline of this shape instead. */
const FALLBACK: Record<string, string> = { "thigh-n": "hindbody", "ear-far": "ear" };
/** Shapes removed from a view: the 1:200 plan drew a `nose` body shape that no other view of that scale has. */
const REMOVE: readonly string[] = ["200/plan/nose"];
/** Mouth line of the 1:50 head, drawn by hand in the head frame along the lips to the corner of the mouth. */
const MOUTH: P[] = [[0.148, -0.0128], [0.112, -0.013], [0.078, -0.0125], [0.06, -0.008]];

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


const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const ops: unknown[] = [];
for (const scale of [50, 100, 200]) {
  const c = openCtx(SPEC, scale);
  const map = JSON.parse(readFileSync(`${OUT}/shapes.${scale}.json`, "utf8")) as Record<string, string>;
  const meshes = meshesAt(c.m, scale, c.vol.body);
  const spread = new Set<string>((c.raw.representations[String(scale)].geometry.elevation as any[]).filter((s) => s.solid !== undefined).map((s) => s.bone));
  for (const view of ["elevation", "front", "plan"] as const) {
    const list = c.raw.representations[String(scale)].geometry[view] as any[];
    const ids = new Set(list.map((s) => s.id));
    const groups: Record<string, typeof meshes> = {};
    for (const m of meshes) {
      let id = map[m.id];
      if (id === undefined) continue;
      if (!ids.has(id)) id = FALLBACK[id] ?? id;
      if (ids.has(id)) (groups[id] ??= []).push(m);
    }
    for (const shape of list) {
      if (REMOVE.includes(`${scale}/${view}/${shape.id}`)) { ops.push({ scale, view, id: shape.id, remove: true }); continue; }
      const f = c.state.frames[shape.bone]!;
      const len = c.raw.rig.bones.find((b: any) => b.id === shape.bone).lengthM as number;
      const Z = f.o[2] ?? 0;
      const toLocal = (w: P[]): P[] => w.map(([x, y]) => [r5((x - f.o[0]!) * f.u[0]! + (y - f.o[1]!) * f.u[1]!), r5((x - f.o[0]!) * f.v[0]! + (y - f.o[1]!) * f.v[1]!)] as P);
      if (shape.id === "mouth-line") { ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: false, points: MOUTH, solid: { kind: "extruded", sweep: "w", z0: -0.018, z1: 0.018 } }, del: GEOMETRY_KEYS }); continue; }
      const ms = groups[shape.id];
      if (ms === undefined) { console.log(`no solids for ${scale} ${view} ${shape.id}`); continue; }
      const cell = c.feature / 12;
      const tol = c.feature * 0.25;
      let [z0, z1] = [Infinity, -Infinity];
      const base: number[] = [];
      const tip: number[] = [];
      for (const m of ms) for (let i = 0; i < m.world.length; i += 3) {
        const z = m.world[i + 2]! - Z;
        z0 = Math.min(z0, z); z1 = Math.max(z1, z);
        const a = (m.world[i]! - f.o[0]!) * f.u[0]! + (m.world[i + 1]! - f.o[1]!) * f.u[1]!;
        if (a < 0.25 * len) base.push(z);
        if (a > 0.85 * len) tip.push(z);
      }
      const upright = Math.abs(f.u[1]!) > 0.94;
      const side = toLocal(outline(ms, 0, 1, cell, tol));
      const prism = { kind: "extruded", sweep: "w", z0: r5(Math.min(...base)), z1: r5(Math.max(...base)), zTip: [r5(Math.min(...tip)), r5(Math.max(...tip))] };
      if (view === "elevation") {
        const solid = spread.has(shape.bone) ? (upright ? prism : { kind: "extruded", sweep: "w", z0: r5(z0), z1: r5(z1) }) : undefined;
        ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: true, points: side, ...(solid === undefined ? {} : { solid }) }, del: GEOMETRY_KEYS });
      } else if (view === "plan" && upright) {
        ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: true, points: side, solid: prism }, del: GEOMETRY_KEYS });
      } else if (view === "front") {
        const world = outline(ms, 2, 1, cell, tol);
        const axis = Math.abs(f.v[1]!) >= Math.abs(f.u[1]!) ? "v" : "u";
        const ax = axis === "v" ? f.v : f.u;
        const other = axis === "v" ? f.u : f.v;
        const fx = [1 - ax[0]! * ax[0]!, -ax[0]! * ax[1]!];
        const sign = Math.sign(fx[0]! * other[0]! + fx[1]! * other[1]!) || 1;
        const points = world.map(([z, y]) => { const h = r5((y - f.o[1]!) / ax[1]!); const rho = r5(sign * (z - Z)); return (axis === "v" ? [rho, h] : [h, rho]) as P; });
        ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: true, points, solid: { kind: "revolved", axisBone: shape.bone, axis, axisX: 0 } }, del: GEOMETRY_KEYS });
      } else {
        const world = outline(ms, 0, 2, cell, tol);
        if (Math.abs(f.u[0]!) < 0.3) throw new Error(`plan slab of ${shape.id}: bone too steep`);
        const points = world.map(([x, z]) => [r5((x - f.o[0]!) / f.u[0]!), r5((z - Z) / f.w[2]!)] as P);
        ops.push({ scale, view, id: shape.id, set: { kind: "poly", closed: true, points, solid: { kind: "extruded", sweep: "v", z0: -0.0005, z1: 0.0005 } }, del: GEOMETRY_KEYS });
      }
    }
  }
}
writeFileSync(`${OUT}/ops2d.json`, JSON.stringify(ops));
console.log(`${ops.length} shapes`);
