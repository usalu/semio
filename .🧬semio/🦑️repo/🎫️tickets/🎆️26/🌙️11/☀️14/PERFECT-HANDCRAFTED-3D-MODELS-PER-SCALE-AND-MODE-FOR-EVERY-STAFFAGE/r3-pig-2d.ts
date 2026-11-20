/** ✏️ Round 3 pig, ruling E1: redraws every closed 2D shape of every scale and view as the true orthographic outline of the body solids that the model files assign to it (`🗑️generated/r3-pig/shapes.<N>.json`, "a|b" = the first of these shapes that the view draws), same ids, bones, roles, order, tones and hatches; the mouth line is redrawn by hand along the lip.
 * Works on a candidate document: the outline is first written in view coordinates, then mapped into each shape's own frame through the evaluated drawing (affine fit, three passes so that front and plan follow their elevation twins). Writes `🗑️generated/r3-pig/🔣️.json` (the candidate asset document) and `🗑️generated/r3-pig/representations.json`.
 * Usage: bun r3-pig-2d.ts [--dimensions h w d]
 * @see ./r3-pig-apply.py */
import { readFileSync, writeFileSync } from "node:fs";
import { openCtx, type Ctx } from "./kit-lib.ts";
import { entryOf, meshesAt, merged, poseState } from "./volumes-lib.ts";
import { polyMap, type P } from "./fauna-affine.ts";
import { OUT, SPEC } from "./r3-pig-kit.ts";

const AX = { elevation: [0, 1], front: [2, 1], plan: [0, 2] } as const;
const GEOMETRY_KEYS = ["cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "d", "segments", "w", "h", "x", "y", "r", "rotationDeg", "width", "height"];
type View = keyof typeof AX;
type Mesh = { id: string; world: Float64Array; indices: Uint32Array };

/** 🖍️ Outline of the largest connected region of the projected meshes, traced on a raster of `cell` and simplified by Douglas-Peucker at `tol`. */
function outline(meshes: readonly Mesh[], a: number, b: number, cell: number, tol: number): P[] {
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
  for (const k of [0, 1]) for (const sgn of [1, -1]) { let bi = 0; for (let i = 1; i < pts.length; i++) if (sgn * pts[i]![k]! > sgn * pts[bi]![k]!) bi = i; keep.add(bi); }
  const ks = [...keep].sort((p, q) => p - q);
  keep.clear();
  for (let j = 0; j < ks.length; j++) { keep.add(ks[j]!); dp(loop, ks[j]!, j + 1 < ks.length ? ks[j + 1]! : loop.length - 1, keep); }
  return [...keep].sort((p, q) => p - q).map((i) => [Math.round(pts[i]![0] * 1e5) / 1e5, Math.round(pts[i]![1] * 1e5) / 1e5] as P);
}

const doc = JSON.parse(readFileSync(entryOf(SPEC).entry.path, "utf8"));
const asset = doc.asset;
const reps = JSON.parse(JSON.stringify(asset.representations));
const dimArg = process.argv.indexOf("--dimensions");
if (dimArg > 0) asset.dimensions = { ...asset.dimensions, heightM: Number(process.argv[dimArg + 1]), widthM: Number(process.argv[dimArg + 2]), depthM: Number(process.argv[dimArg + 3]) };
let count = 0;
let worst = 0;
for (const scale of [50, 100, 200, 500]) {
  const c = openCtx(SPEC, scale);
  const map = JSON.parse(readFileSync(`${OUT}/shapes.${scale}.json`, "utf8")) as Record<string, string>;
  const meshes = meshesAt(c.m, scale, c.vol.body) as Mesh[];
  const geo = reps[String(scale)].geometry as Record<View, any[]>;
  if (scale === 50) geo.plan = geo.plan.filter((s) => s.id !== "eye" && s.id !== "eye-f");
  const world: Record<string, P[]> = {};
  for (const view of ["elevation", "front", "plan"] as const) {
    const ids = new Set(geo[view].map((s) => s.id));
    const groups: Record<string, Mesh[]> = {};
    for (const m of meshes) {
      const alt = map[m.id];
      if (alt === undefined) continue;
      const hit = alt.split("|").find((id) => ids.has(id));
      if (hit !== undefined) (groups[hit] ??= []).push(m);
    }
    for (const shape of geo[view]) {
      if (!shape.closed && shape.kind !== "ellipse" && shape.kind !== "capsule") continue;
      const ms = groups[shape.id];
      if (ms === undefined) { console.log(`no solids for 1:${scale} ${view} ${shape.id}`); continue; }
      const pts = outline(ms, AX[view][0], AX[view][1], c.feature / 12, c.feature * 0.2);
      world[`${view}/${shape.id}`] = pts;
      for (const key of GEOMETRY_KEYS) delete shape[key];
      Object.assign(shape, { kind: "poly", closed: true, points: pts });
      count++;
    }
  }
  const mouth = scale === 50 ? geo.elevation.find((s) => s.id === "mouth-line") : undefined;
  const head = c.state.frames.head!;
  const mouthWorld: P[] = ([[0.262, -0.024], [0.235, -0.03], [0.195, -0.036], [0.15, -0.04]] as const).map(([s, v]) => [head.o[0]! + s * head.u[0]! + v * head.v[0]!, head.o[1]! + s * head.u[1]! + v * head.v[1]!]);
  for (let pass = 0; pass < 3; pass++) {
    const cand = { ...asset, representations: reps };
    const m = merged(cand, c.source);
    const ctx: Ctx = { ...c, raw: cand, m, state: poseState(m, scale) };
    for (const view of ["elevation", "front", "plan"] as const) for (const shape of geo[view]) {
      const w = world[`${view}/${shape.id}`];
      if (w === undefined) continue;
      const fit = polyMap(ctx, view, shape.id, shape.points);
      if (pass === 2) worst = Math.max(worst, fit.err);
      shape.points = fit.toLocal(w);
      if (mouth !== undefined && view === "elevation" && shape.id === "head") mouth.points = fit.toLocal(mouthWorld);
    }
  }
}
asset.representations = reps;
asset.volumes = openCtx(SPEC, 50).source.volumes;
writeFileSync(`${OUT}/🔣️.json`, JSON.stringify(doc, null, 2));
writeFileSync(`${OUT}/representations.json`, JSON.stringify({ representations: reps, dimensions: asset.dimensions }));
console.log(`${count} shapes redrawn, worst affine residual ${worst.toFixed(6)} m`);
