/** 🌲️ PW-08 plant kit: world-space meshes for trunks, limbs and foliage masses, a sagittal botanical rig, orthographic projection of the per-scale bodies into the three drawing frames and the assembly of a `semio.staffage/v2` asset document from hand-chosen species data.
 *
 * @see ../../📋️brief.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

export type V3 = [number, number, number];
export type P2 = [number, number];
export type Text = { en: string; de: string };
export type Mesh = { p: V3[]; t: [number, number, number][] };
export type View = "elevation" | "front" | "plan";
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export const SCALES = [50, 100, 200, 500] as const;
export type Scale = (typeof SCALES)[number];
export const FEATURE_M: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
export const BANDS: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
export const RAD = Math.PI / 180;

export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, s: number): V3 => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;
export const lerp3 = (a: V3, b: V3, t: number): V3 => [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)];
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
export const r2 = (x: number): number => Math.round(x * 100) / 100;
export const r1 = (x: number): number => Math.round(x * 10) / 10;
export const smooth = (t: number): number => t * t * (3 - 2 * t);
export const clamp = (x: number, a: number, b: number): number => Math.min(b, Math.max(a, x));

/** 🎲️ A deterministic pseudo-random stream (mulberry32), so every hand-chosen irregularity is reproducible. */
export function rng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** 🧭️ A station of a {@link tube}: centre, radius function of the azimuth θ (radians, 0 at the station normal) and an optional elliptic lateral factor. */
export type Station = { c: V3; r: number | ((theta: number) => number) };

/** 🪵️ A closed tube through `stations` with `n` points per ring: a station of radius 0 at an end becomes an apex, any other end a flat cap; frames are parallel-transported from `up` so rings never twist. */
export function tube(stations: readonly Station[], n: number, up: V3 = [1, 0, 0]): Mesh {
  const p: V3[] = [];
  const t: [number, number, number][] = [];
  const m = stations.length;
  const tangent = (i: number): V3 => unit(sub(stations[Math.min(m - 1, i + 1)]!.c, stations[Math.max(0, i - 1)]!.c));
  let T = tangent(0);
  let N = unit(sub(up, mul(T, dot(up, T))));
  if (len(sub(up, mul(T, dot(up, T)))) < 1e-6) N = unit(cross(T, [0, 0, 1]));
  const rings: (number | null)[] = [];
  const apexes: (number | null)[] = [];
  for (let i = 0; i < m; i++) {
    const Ti = tangent(i);
    const axis = cross(T, Ti);
    const s = len(axis);
    if (s > 1e-9) {
      const k = unit(axis);
      const angle = Math.atan2(s, dot(T, Ti));
      N = add(add(mul(N, Math.cos(angle)), mul(cross(k, N), Math.sin(angle))), mul(k, dot(k, N) * (1 - Math.cos(angle))));
    }
    T = Ti;
    const B = cross(N, T);
    const st = stations[i]!;
    const rad = (th: number): number => (typeof st.r === "number" ? st.r : st.r(th));
    const isApex = (i === 0 || i === m - 1) && typeof st.r === "number" && st.r === 0;
    if (isApex) {
      apexes.push(p.length);
      rings.push(null);
      p.push([...st.c] as V3);
      continue;
    }
    apexes.push(null);
    rings.push(p.length);
    for (let j = 0; j < n; j++) {
      const th = (2 * Math.PI * j) / n;
      const r = rad(th);
      p.push(add(st.c, add(mul(N, r * Math.cos(th)), mul(B, r * Math.sin(th)))));
    }
  }
  for (let i = 0; i + 1 < m; i++) {
    const a = rings[i];
    const b = rings[i + 1];
    if (a !== null && b !== null) {
      for (let j = 0; j < n; j++) {
        const j1 = (j + 1) % n;
        t.push([a! + j, a! + j1, b! + j1], [a! + j, b! + j1, b! + j]);
      }
    } else if (a === null && b !== null) {
      for (let j = 0; j < n; j++) t.push([apexes[i]!, b! + ((j + 1) % n), b! + j]);
    } else if (a !== null && b === null) {
      for (let j = 0; j < n; j++) t.push([a! + j, a! + ((j + 1) % n), apexes[i + 1]!]);
    }
  }
  if (rings[0] !== null) {
    const c = p.length;
    p.push([...stations[0]!.c] as V3);
    for (let j = 0; j < n; j++) t.push([c, rings[0]! + ((j + 1) % n), rings[0]! + j]);
  }
  if (rings[m - 1] !== null) {
    const c = p.length;
    p.push([...stations[m - 1]!.c] as V3);
    for (let j = 0; j < n; j++) t.push([c, rings[m - 1]! + j, rings[m - 1]! + ((j + 1) % n)]);
  }
  return orient({ p, t });
}

/** 🫧️ A closed star-shaped mass about `c`: semi-axes `radii` (x, y, z) modulated by `f(θ, φ)` (θ azimuth about y from +x towards +z, φ polar angle from +y), `nLat` latitude bands and `nLon` meridians, poles on the y axis. */
export function blob(c: V3, radii: V3, f: (theta: number, phi: number) => number, nLat: number, nLon: number): Mesh {
  const p: V3[] = [];
  const t: [number, number, number][] = [];
  const at = (theta: number, phi: number): V3 => {
    const k = f(theta, phi);
    return [c[0] + radii[0] * k * Math.sin(phi) * Math.cos(theta), c[1] + radii[1] * k * Math.cos(phi), c[2] + radii[2] * k * Math.sin(phi) * Math.sin(theta)];
  };
  p.push(at(0, 0));
  for (let i = 1; i < nLat; i++) for (let j = 0; j < nLon; j++) p.push(at((2 * Math.PI * j) / nLon, (Math.PI * i) / nLat));
  p.push(at(0, Math.PI));
  const ring = (i: number, j: number): number => 1 + (i - 1) * nLon + (j % nLon);
  for (let j = 0; j < nLon; j++) t.push([0, ring(1, j + 1), ring(1, j)]);
  for (let i = 1; i + 1 < nLat; i++) for (let j = 0; j < nLon; j++) t.push([ring(i, j), ring(i, j + 1), ring(i + 1, j + 1)], [ring(i, j), ring(i + 1, j + 1), ring(i + 1, j)]);
  const last = p.length - 1;
  for (let j = 0; j < nLon; j++) t.push([last, ring(nLat - 1, j), ring(nLat - 1, j + 1)]);
  return orient({ p, t });
}

/** 🔄️ The mesh with outward triangles (positive signed volume). */
export function orient(m: Mesh): Mesh {
  return volume(m) >= 0 ? m : { p: m.p, t: m.t.map(([a, b, c]) => [a, c, b]) };
}

/** 🧊️ Signed volume of a closed mesh (m³). */
export function volume(m: Mesh): number {
  let v = 0;
  for (const [a, b, c] of m.t) v += dot(m.p[a]!, cross(m.p[b]!, m.p[c]!)) / 6;
  return v;
}

/** 📐️ Surface area of a mesh (m²). */
export function area(m: Mesh): number {
  let s = 0;
  for (const [a, b, c] of m.t) s += len(cross(sub(m.p[b]!, m.p[a]!), sub(m.p[c]!, m.p[a]!))) / 2;
  return s;
}

/** ⚖️ The centroid of the enclosed volume. */
export function centroid(m: Mesh): V3 {
  let v = 0;
  let c: V3 = [0, 0, 0];
  for (const [a, b, d] of m.t) {
    const w = dot(m.p[a]!, cross(m.p[b]!, m.p[d]!)) / 6;
    v += w;
    c = add(c, mul(add(add(m.p[a]!, m.p[b]!), m.p[d]!), w / 4));
  }
  return mul(c, 1 / v);
}

/** 📦️ The axis-aligned bounds of meshes. */
export function bounds(ms: readonly Mesh[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity];
  const max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const m of ms) for (const q of m.p) for (let i = 0; i < 3; i++) {
    min[i] = Math.min(min[i]!, q[i]!);
    max[i] = Math.max(max[i]!, q[i]!);
  }
  return { min, max };
}

/** 🎯️ Is `q` inside the closed mesh (ray parity along a skew direction)? */
export function inside(m: Mesh, q: V3): boolean {
  const d: V3 = unit([0.5773, 0.6123, 0.5401]);
  let hits = 0;
  for (const [a, b, c] of m.t) {
    const A = m.p[a]!, B = m.p[b]!, C = m.p[c]!;
    const e1 = sub(B, A), e2 = sub(C, A);
    const h = cross(d, e2);
    const det = dot(e1, h);
    if (Math.abs(det) < 1e-14) continue;
    const s = sub(q, A);
    const u = dot(s, h) / det;
    if (u < 0 || u > 1) continue;
    const qq = cross(s, e1);
    const v = dot(d, qq) / det;
    if (v < 0 || u + v > 1) continue;
    if (dot(e2, qq) / det > 1e-12) hits++;
  }
  return hits % 2 === 1;
}

/** 🌫️ Outer area of the union of meshes: area samples of every triangle that lie inside no other mesh of the set. */
export function unionArea(ms: readonly Mesh[], samplesPerM2 = 40): number {
  const boxes = ms.map((m) => bounds([m]));
  let total = 0;
  ms.forEach((m, k) => {
    for (const [a, b, c] of m.t) {
      const A = m.p[a]!, B = m.p[b]!, C = m.p[c]!;
      const ar = len(cross(sub(B, A), sub(C, A))) / 2;
      const n = Math.max(1, Math.min(12, Math.round(ar * samplesPerM2)));
      let free = 0;
      for (let s = 0; s < n; s++) {
        let u = ((s * 0.618034 + 0.31) % 1), v = ((s * 0.754878 + 0.17) % 1);
        if (u + v > 1) { u = 1 - u; v = 1 - v; }
        const probe = add(A, add(mul(sub(B, A), u), mul(sub(C, A), v)));
        const buried = ms.some((o, j) => j !== k && probe[0] >= boxes[j]!.min[0] && probe[0] <= boxes[j]!.max[0] && probe[1] >= boxes[j]!.min[1] && probe[1] <= boxes[j]!.max[1] && probe[2] >= boxes[j]!.min[2] && probe[2] <= boxes[j]!.max[2] && inside(o, probe));
        if (!buried) free++;
      }
      total += (ar * free) / n;
    }
  });
  return total;
}

/** 🔁️ A mesh rotated about the world y axis by `deg` (counter-clockwise seen from above: +x towards -z) about `pivot`. */
export function rotY(m: Mesh, deg: number, pivot: V3 = [0, 0, 0]): Mesh {
  const c = Math.cos(deg * RAD), s = Math.sin(deg * RAD);
  return { p: m.p.map((q) => { const d = sub(q, pivot); return add(pivot, [c * d[0] + s * d[2], d[1], -s * d[0] + c * d[2]]); }), t: m.t };
}

/** ↗️ A mesh moved by `d`. */
export const shift = (m: Mesh, d: V3): Mesh => ({ p: m.p.map((q) => add(q, d)), t: m.t });

//#region 🦴️ Rig
export type BoneSpec = {
  id: string;
  parent: string | null;
  title: Text;
  o: V3;
  angle: number;
  length: number;
  joint: { type: "fixed" } | { type: "revolute"; minDeg: number; maxDeg: number };
  section?: { d: number; material: string; title: Text };
};
export type Frame = { o: V3; u: V3; v: V3; w: V3; angle: number };

/** 🧭️ The rest frame of a sagittal bone at world angle `angle` (degrees, CCW from +x in the side elevation). */
export const frameOf = (o: V3, angle: number): Frame => ({ o, angle, u: [Math.cos(angle * RAD), Math.sin(angle * RAD), 0], v: [-Math.sin(angle * RAD), Math.cos(angle * RAD), 0], w: [0, 0, 1] });
export const toLocal = (f: Frame, q: V3): V3 => { const d = sub(q, f.o); return [dot(d, f.u), dot(d, f.v), dot(d, f.w)]; };
export const toWorld = (f: Frame, q: V3): V3 => add(f.o, add(add(mul(f.u, q[0]), mul(f.v, q[1])), mul(f.w, q[2])));
export const tipOf = (b: BoneSpec): V3 => add(b.o, [Math.cos(b.angle * RAD) * b.length, Math.sin(b.angle * RAD) * b.length, 0]);
export const frames = (bones: readonly BoneSpec[]): Record<string, Frame> => Object.fromEntries(bones.map((b) => [b.id, frameOf(b.o, b.angle)]));
//#endregion 🦴️ Rig

//#region 🖼️ Projection and raster
/** 🎥️ The paper coordinates of a world point in a view (the product's `CAMERAS`: elevation x, y; front z, y; plan x, z). */
export const paper = (view: View, q: V3): P2 => (view === "elevation" ? [q[0], q[1]] : view === "front" ? [q[2], q[1]] : [q[0], q[2]]);
/** 👁️ How near a world point is to the viewer of a view (larger is nearer): smaller z in the elevation, larger x in the front, higher y in the plan. */
export const nearness = (view: View, q: V3): number => (view === "elevation" ? -q[2] : view === "front" ? q[0] : q[1]);

/** 🧱️ A binary raster on paper coordinates (metres). */
export class Grid {
  readonly w: number;
  readonly h: number;
  readonly d: Uint8Array;
  constructor(readonly x0: number, readonly y0: number, readonly cell: number, w: number, h: number) {
    this.w = Math.max(1, w);
    this.h = Math.max(1, h);
    this.d = new Uint8Array(this.w * this.h);
  }
  static around(points: readonly P2[], cell: number, margin: number): Grid {
    let [a, b, c, d] = [Infinity, Infinity, -Infinity, -Infinity];
    for (const [x, y] of points) {
      a = Math.min(a, x); b = Math.min(b, y); c = Math.max(c, x); d = Math.max(d, y);
    }
    const x0 = Math.floor((a - margin) / cell) * cell;
    const y0 = Math.floor((b - margin) / cell) * cell;
    return new Grid(x0, y0, cell, Math.ceil((c + margin - x0) / cell) + 1, Math.ceil((d + margin - y0) / cell) + 1);
  }
  like(): Grid { return new Grid(this.x0, this.y0, this.cell, this.w, this.h); }
  /** 🔺️ Sets every cell whose centre lies in the triangle. */
  tri(a: P2, b: P2, c: P2): void {
    const { x0, y0, cell, w, h, d } = this;
    const i0 = Math.max(0, Math.floor((Math.min(a[0], b[0], c[0]) - x0) / cell - 0.5));
    const i1 = Math.min(w - 1, Math.ceil((Math.max(a[0], b[0], c[0]) - x0) / cell - 0.5));
    const j0 = Math.max(0, Math.floor((Math.min(a[1], b[1], c[1]) - y0) / cell - 0.5));
    const j1 = Math.min(h - 1, Math.ceil((Math.max(a[1], b[1], c[1]) - y0) / cell - 0.5));
    const area2 = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    if (Math.abs(area2) < 1e-14) return;
    for (let j = j0; j <= j1; j++) {
      const y = y0 + (j + 0.5) * cell;
      for (let i = i0; i <= i1; i++) {
        const x = x0 + (i + 0.5) * cell;
        const w0 = ((b[0] - x) * (c[1] - y) - (b[1] - y) * (c[0] - x)) / area2;
        const w1 = ((c[0] - x) * (a[1] - y) - (c[1] - y) * (a[0] - x)) / area2;
        const w2 = 1 - w0 - w1;
        if (w0 >= -1e-9 && w1 >= -1e-9 && w2 >= -1e-9) d[j * w + i] = 1;
      }
    }
  }
  mesh(view: View, m: Mesh): void {
    const q = m.p.map((x) => paper(view, x));
    for (const [a, b, c] of m.t) this.tri(q[a]!, q[b]!, q[c]!);
  }
  ring(r: readonly P2[]): void {
    const { x0, y0, cell, w, h, d } = this;
    for (let j = 0; j < h; j++) {
      const y = y0 + (j + 0.5) * cell;
      const xs: number[] = [];
      for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
        const a = r[k]!, b = r[i]!;
        if (a[1] <= y !== b[1] <= y) xs.push(a[0] + ((y - a[1]) / (b[1] - a[1])) * (b[0] - a[0]));
      }
      xs.sort((p, q) => p - q);
      for (let k = 0; k + 1 < xs.length; k += 2) {
        const i0 = Math.max(0, Math.ceil((xs[k]! - x0) / cell - 0.5));
        const i1 = Math.min(w - 1, Math.floor((xs[k + 1]! - x0) / cell - 0.5));
        for (let i = i0; i <= i1; i++) d[j * w + i] = 1;
      }
    }
  }
  count(): number { let n = 0; for (const x of this.d) n += x; return n; }
  or(o: Grid): void { for (let i = 0; i < this.d.length; i++) this.d[i] = this.d[i]! | o.d[i]!; }
  andNot(o: Grid): void { for (let i = 0; i < this.d.length; i++) if (o.d[i]) this.d[i] = 0; }
  copy(): Grid { const g = this.like(); g.d.set(this.d); return g; }
  at(x: number, y: number): number {
    const i = Math.floor((x - this.x0) / this.cell), j = Math.floor((y - this.y0) / this.cell);
    return i < 0 || j < 0 || i >= this.w || j >= this.h ? 0 : this.d[j * this.w + i]!;
  }
}

/** 📏️ Squared Euclidean distance (cells²) of every cell to the nearest cell where `on` is false (Felzenszwalb and Huttenlocher 2012). */
export function edt(g: Grid, on: (v: number) => boolean): Float64Array {
  const { w, h } = g;
  const INF = 1e20;
  const f = new Float64Array(w * h);
  for (let i = 0; i < w * h; i++) f[i] = on(g.d[i]!) ? INF : 0;
  const line = (get: (k: number) => number, set: (k: number, v: number) => void, n: number) => {
    const v = new Int32Array(n);
    const z = new Float64Array(n + 1);
    const ff = new Float64Array(n);
    for (let q = 0; q < n; q++) ff[q] = get(q);
    let k = 0;
    v[0] = 0; z[0] = -INF; z[1] = INF;
    for (let q = 1; q < n; q++) {
      let s = (ff[q]! + q * q - (ff[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!);
      while (s <= z[k]!) {
        k--;
        s = (ff[q]! + q * q - (ff[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!);
      }
      k++;
      v[k] = q; z[k] = s; z[k + 1] = INF;
    }
    k = 0;
    for (let q = 0; q < n; q++) {
      while (z[k + 1]! < q) k++;
      set(q, (q - v[k]!) * (q - v[k]!) + ff[v[k]!]!);
    }
  };
  for (let x = 0; x < w; x++) line((k) => f[k * w + x]!, (k, val) => { f[k * w + x] = val; }, h);
  for (let y = 0; y < h; y++) line((k) => f[y * w + k]!, (k, val) => { f[y * w + k] = val; }, w);
  return f;
}

/** 🎈️ Dilation (radius > 0) or erosion (radius < 0) by a disc of `radius` metres. */
export function morph(g: Grid, radius: number): Grid {
  const r = Math.abs(radius) / g.cell;
  const out = g.like();
  if (radius >= 0) {
    const dist = edt(g, (v) => v === 0);
    for (let i = 0; i < dist.length; i++) out.d[i] = dist[i]! <= r * r ? 1 : 0;
  } else {
    const dist = edt(g, (v) => v === 1);
    for (let i = 0; i < dist.length; i++) out.d[i] = dist[i]! > r * r ? 1 : 0;
  }
  return out;
}

/** 🕳️ The grid with every enclosed hole filled (flood of the background from the border). */
export function fillHoles(g: Grid): Grid {
  const { w, h } = g;
  const seen = new Uint8Array(w * h);
  const stack: number[] = [];
  for (let i = 0; i < w; i++) stack.push(i, (h - 1) * w + i);
  for (let j = 0; j < h; j++) stack.push(j * w, j * w + w - 1);
  while (stack.length) {
    const k = stack.pop()!;
    if (seen[k] || g.d[k]) continue;
    seen[k] = 1;
    const x = k % w, y = (k - x) / w;
    if (x > 0) stack.push(k - 1);
    if (x < w - 1) stack.push(k + 1);
    if (y > 0) stack.push(k - w);
    if (y < h - 1) stack.push(k + w);
  }
  const out = g.like();
  for (let i = 0; i < w * h; i++) out.d[i] = seen[i] ? 0 : 1;
  return out;
}

/** 🏝️ The 4-connected islands of a grid as separate grids, largest first. */
export function islands(g: Grid): Grid[] {
  const { w, h } = g;
  const label = new Int32Array(w * h).fill(-1);
  const out: Grid[] = [];
  for (let s = 0; s < w * h; s++) {
    if (!g.d[s] || label[s]! >= 0) continue;
    const isl = g.like();
    const stack = [s];
    label[s] = out.length;
    while (stack.length) {
      const k = stack.pop()!;
      isl.d[k] = 1;
      const x = k % w, y = (k - x) / w;
      for (const n of [x > 0 ? k - 1 : -1, x < w - 1 ? k + 1 : -1, y > 0 ? k - w : -1, y < h - 1 ? k + w : -1]) if (n >= 0 && g.d[n] && label[n]! < 0) { label[n] = out.length; stack.push(n); }
    }
    out.push(isl);
  }
  return out.sort((a, b) => b.count() - a.count());
}

/** 〰️ The outer boundary of a single filled island as a ring of cell corners (counter-clockwise). */
export function outerRing(g: Grid): P2[] {
  const { w, h, x0, y0, cell } = g;
  const on = (x: number, y: number) => x >= 0 && y >= 0 && x < w && y < h && g.d[y * w + x] === 1;
  let sx = -1, sy = -1;
  outer: for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) if (on(x, y)) { sx = x; sy = y; break outer; }
  if (sx < 0) return [];
  const next = new Map<number, number[]>();
  const W = w + 1;
  const edge = (ax: number, ay: number, bx: number, by: number) => { const k = ay * W + ax; (next.get(k) ?? next.set(k, []).get(k)!).push(by * W + bx); };
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
    if (!on(x, y)) continue;
    if (!on(x, y - 1)) edge(x, y, x + 1, y);
    if (!on(x + 1, y)) edge(x + 1, y, x + 1, y + 1);
    if (!on(x, y + 1)) edge(x + 1, y + 1, x, y + 1);
    if (!on(x - 1, y)) edge(x, y + 1, x, y);
  }
  const start = sy * W + sx;
  const ring: P2[] = [];
  let at = start;
  let prev = -1;
  for (let guard = 0; guard < 4 * w * h + 8; guard++) {
    ring.push([x0 + (at % W) * cell, y0 + Math.floor(at / W) * cell]);
    const outs = next.get(at)!;
    let pick = outs[0]!;
    if (outs.length > 1 && prev >= 0) {
      const [px, py] = [prev % W, Math.floor(prev / W)];
      const [ax, ay] = [at % W, Math.floor(at / W)];
      const din = [ax - px, ay - py];
      const turn = (k: number) => { const [bx, by] = [k % W, Math.floor(k / W)]; const dout = [bx - ax, by - ay]; return din[0]! * dout[1]! - din[1]! * dout[0]!; };
      pick = outs.reduce((best, k) => (turn(k) > turn(best) ? k : best), outs[0]!);
    }
    prev = at;
    at = pick;
    if (at === start) break;
  }
  return ring;
}

export const ringArea = (r: readonly P2[]): number => { let s = 0; for (let i = 0, k = r.length - 1; i < r.length; k = i++) s += r[k]![0] * r[i]![1] - r[i]![0] * r[k]![1]; return s / 2; };

/** ✂️ Douglas-Peucker simplification of a closed ring within `tol`. */
export function simplify(ring: readonly P2[], tol: number): P2[] {
  const n = ring.length;
  if (n <= 4) return [...ring];
  let far = 0, best = -1;
  for (let i = 1; i < n; i++) { const d = Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]); if (d > best) { best = d; far = i; } }
  const keep = new Uint8Array(n);
  keep[0] = 1; keep[far] = 1;
  const run = (lo: number, hi: number) => {
    if (hi <= lo + 1) return;
    const a = ring[lo % n]!, b = ring[hi % n]!;
    const dx = b[0] - a[0], dy = b[1] - a[1], L = Math.hypot(dx, dy) || 1e-12;
    let worst = -1, at = -1;
    for (let i = lo + 1; i < hi; i++) { const p = ring[i % n]!; const d = Math.abs((p[0] - a[0]) * dy - (p[1] - a[1]) * dx) / L; if (d > worst) { worst = d; at = i; } }
    if (worst > tol) { keep[at % n] = 1; run(lo, at); run(at, hi); }
  };
  run(0, far);
  run(far, n);
  return ring.filter((_, i) => keep[i]);
}

/** 🌀️ Chaikin corner cutting, `k` rounds: a hand-drawn soft outline from a polygon. */
export function chaikin(ring: readonly P2[], k: number): P2[] {
  let r = [...ring];
  for (let s = 0; s < k; s++) {
    const o: P2[] = [];
    for (let i = 0; i < r.length; i++) {
      const a = r[i]!, b = r[(i + 1) % r.length]!;
      o.push([0.75 * a[0] + 0.25 * b[0], 0.75 * a[1] + 0.25 * b[1]], [0.25 * a[0] + 0.75 * b[0], 0.25 * a[1] + 0.75 * b[1]]);
    }
    r = o;
  }
  return r;
}
//#endregion 🖼️ Projection and raster

//#region 📄️ Output
export function writeJson(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value)}\n`);
}
export const t = (en: string, de: string): Text => ({ en, de });
//#endregion 📄️ Output

/** 🧭️ A mesh rotated about `pivot` so that the direction `from` turns onto `to` (Rodrigues). */
export function rotateTo(m: Mesh, from: V3, to: V3, pivot: V3): Mesh {
  const a = unit(from), b = unit(to);
  const k = cross(a, b);
  const s = len(k), c = dot(a, b);
  if (s < 1e-9) return m;
  const n = mul(k, 1 / s);
  const rot = (q: V3): V3 => add(add(mul(q, c), mul(cross(n, q), s)), mul(n, dot(n, q) * (1 - c)));
  return { p: m.p.map((q) => add(pivot, rot(sub(q, pivot)))), t: m.t };
}

/** 〰️ Points of the quadratic Bézier from `a` over `c` to `b` at parameter `u`. */
export const bezier = (a: V3, c: V3, b: V3, u: number): V3 => add(add(mul(a, (1 - u) * (1 - u)), mul(c, 2 * u * (1 - u))), mul(b, u * u));
