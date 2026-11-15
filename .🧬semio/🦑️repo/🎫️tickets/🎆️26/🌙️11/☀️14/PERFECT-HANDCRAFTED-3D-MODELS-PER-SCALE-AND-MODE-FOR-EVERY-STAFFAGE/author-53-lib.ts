/** 🌳️ Author 53: geometry helpers for hand-built trees from a scale's own elevation, front and plan drawings (world frame: x east, y up, z depth; elevation (x,y), front (z,y), plan (x,z)).
 * @see ./author-53.md */
import { openCtx, saveLists, drawn, type Ctx, type Json, type P2, type V3 } from "./kit-lib.ts";

export { openCtx, saveLists, drawn, type Ctx, type Json, type P2, type V3 };
export type Ring = P2[];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;

export const ROUND = (x: number): number => Math.round(x * 1e5) / 1e5;

export function toLocal(c: Ctx, bone: string, p: V3): V3 {
  const f = c.state.frames[bone]!;
  const d = sub(p, f.o);
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
}
export function dirLocal(c: Ctx, bone: string, d: V3): V3 {
  const f = c.state.frames[bone]!;
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
}

/** 🧭️ Quaternion [x, y, z, w] whose rotation maps the x, y, z axes to the given (right-handed) columns. */
export function quatOfColumns(x: V3, y: V3, z: V3): [number, number, number, number] {
  const m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]];
  const tr = m[0]![0]! + m[1]![1]! + m[2]![2]!;
  let q: [number, number, number, number];
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m[2]![1]! - m[1]![2]!) / s, (m[0]![2]! - m[2]![0]!) / s, (m[1]![0]! - m[0]![1]!) / s, 0.25 * s];
  } else if (m[0]![0]! > m[1]![1]! && m[0]![0]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[0]![0]! - m[1]![1]! - m[2]![2]!) * 2;
    q = [0.25 * s, (m[0]![1]! + m[1]![0]!) / s, (m[0]![2]! + m[2]![0]!) / s, (m[2]![1]! - m[1]![2]!) / s];
  } else if (m[1]![1]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[1]![1]! - m[0]![0]! - m[2]![2]!) * 2;
    q = [(m[0]![1]! + m[1]![0]!) / s, 0.25 * s, (m[1]![2]! + m[2]![1]!) / s, (m[0]![2]! - m[2]![0]!) / s];
  } else {
    const s = Math.sqrt(1 + m[2]![2]! - m[0]![0]! - m[1]![1]!) * 2;
    q = [(m[0]![2]! + m[2]![0]!) / s, (m[1]![2]! + m[2]![1]!) / s, 0.25 * s, (m[1]![0]! - m[0]![1]!) / s];
  }
  const n = Math.hypot(...q);
  return q.map((v) => v / n) as [number, number, number, number];
}

export type Sec = { at: V3; rx: number; ry: number; exponent: number; rotation: [number, number, number, number] };

/** 🎢️ A loft section in the bone frame whose plane is given in world axes (x, y world vectors, z = x cross y). */
export function sectionWorld(c: Ctx, bone: string, centre: V3, ax: V3, ay: V3, rx: number, ry: number, e: number): Sec {
  const az = cross(ax, ay);
  const q = quatOfColumns(dirLocal(c, bone, ax), dirLocal(c, bone, ay), dirLocal(c, bone, az));
  return { at: toLocal(c, bone, centre), rx, ry, exponent: e, rotation: q };
}

/** 🧵️ Section perpendicular to a world tangent: x = horizontal and perpendicular to the tangent (the lateral width), y the in-elevation direction. */
export function sectionAlong(c: Ctx, bone: string, centre: V3, tangent: V3, rLat: number, rVert: number, e = 2): Sec {
  const t = unit(tangent);
  const horizontal = Math.abs(t[1]) > 0.97;
  const x = horizontal ? ([0, 0, 1] as V3) : unit(cross([0, 1, 0], t));
  const y = cross(t, x);
  return sectionWorld(c, bone, centre, x, y, rLat, rVert, e);
}

/** 🟫️ Horizontal section at height y: x half rx along world x, y half ry along world z. */
export const sectionLevel = (c: Ctx, bone: string, centre: V3, rx: number, rz: number, e: number): Sec => sectionWorld(c, bone, centre, [1, 0, 0], [0, 0, -1], rx, rz, e);

export const loftSolid = (id: string, bone: string, material: string, tone: number, sections: Sec[], extra: Json = {}): Json => ({
  id,
  kind: "loft",
  bone,
  material,
  role: "body",
  tone,
  sections: sections.map((s) => ({ at: s.at.map(ROUND), rx: ROUND(s.rx), ry: ROUND(s.ry), exponent: s.exponent, rotation: s.rotation.map((v) => Math.round(v * 1e6) / 1e6) })),
  capStart: true,
  capEnd: true,
  ...extra,
});

export const sphereSolid = (c: Ctx, id: string, bone: string, material: string, tone: number, centre: V3, r: number, extra: Json = {}): Json => ({
  id,
  kind: "ellipsoid",
  bone,
  material,
  role: "detail",
  tone,
  radii: [r, r, r].map(ROUND),
  transform: { translation: toLocal(c, bone, centre).map(ROUND) },
  ...extra,
});

/* ---------------------------------------------------------------- 2D rings */

export const ringsOf = (c: Ctx, view: "elevation" | "front" | "plan", pick: (s: { id: string; bone: string; role: string; region?: string }) => boolean): Ring[] =>
  drawn(c, view).filter((s) => pick(s)).flatMap((s) => s.rings);

export function boxOf(rings: readonly Ring[]): { minX: number; maxX: number; minY: number; maxY: number } {
  const b = { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity };
  for (const r of rings) for (const p of r) {
    b.minX = Math.min(b.minX, p.x);
    b.maxX = Math.max(b.maxX, p.x);
    b.minY = Math.min(b.minY, p.y);
    b.maxY = Math.max(b.maxY, p.y);
  }
  return b;
}

/** ✂️ Span of the other coordinate where the line `axis = s` crosses the rings (outer hull); axis "y": the x interval at height s. */
export function spanAt(rings: readonly Ring[], axis: "x" | "y", s: number): [number, number] | null {
  let lo = Infinity;
  let hi = -Infinity;
  for (const r of rings) for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
    const a = r[k]!;
    const b = r[i]!;
    const [pa, pb, qa, qb] = axis === "y" ? [a.y, b.y, a.x, b.x] : [a.x, b.x, a.y, b.y];
    if (pa <= s !== pb <= s) {
      const q = qa + ((s - pa) / (pb - pa)) * (qb - qa);
      lo = Math.min(lo, q);
      hi = Math.max(hi, q);
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

export function inside(rings: readonly Ring[], x: number, y: number): boolean {
  let n = 0;
  for (const r of rings) for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
    const a = r[k]!;
    const b = r[i]!;
    if (a.y > y !== b.y > y && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x) n++;
  }
  return n % 2 === 1;
}

export type Stripe = { n: number; a: number[]; b: number[]; h: number[]; D: [number, number]; N: [number, number]; at: (t: number) => { p: [number, number]; h: number } };

/** 🪵️ The centre line and half width of a stripe-like outline: slices perpendicular to the principal axis (or to `dir`), start end nearest `hint`. */
export function stripe(rings: readonly Ring[], hint: [number, number], n = 28, dir?: [number, number], range?: [number, number]): Stripe | null {
  const pts = rings.flat();
  if (pts.length < 3) return null;
  const mx = pts.reduce((s, p) => s + p.x, 0) / pts.length;
  const my = pts.reduce((s, p) => s + p.y, 0) / pts.length;
  let D: [number, number];
  if (dir !== undefined) D = dir;
  else {
    let sxx = 0, sxy = 0, syy = 0;
    for (const p of pts) {
      sxx += (p.x - mx) ** 2;
      sxy += (p.x - mx) * (p.y - my);
      syy += (p.y - my) ** 2;
    }
    const th = 0.5 * Math.atan2(2 * sxy, sxx - syy);
    D = [Math.cos(th), Math.sin(th)];
  }
  const along = (p: P2): number => p.x * D[0] + p.y * D[1];
  let smin = Math.min(...pts.map(along));
  let smax = Math.max(...pts.map(along));
  if (range !== undefined) [smin, smax] = range;
  if (Math.abs(along({ x: hint[0], y: hint[1] }) - smin) > Math.abs(along({ x: hint[0], y: hint[1] }) - smax)) {
    D = [-D[0], -D[1]];
    [smin, smax] = [-smax, -smin];
  }
  const N: [number, number] = [-D[1], D[0]];
  const a: number[] = [];
  const b: number[] = [];
  const h: number[] = [];
  const rot: Ring[] = rings.map((r) => r.map((p) => ({ x: p.x * D[0] + p.y * D[1], y: p.x * N[0] + p.y * N[1] })));
  for (let i = 0; i < n; i++) {
    const s = smin + ((smax - smin) * (i + 0.5)) / n;
    const iv = spanAt(rot, "x", s);
    if (iv === null) continue;
    a.push(s);
    b.push((iv[0] + iv[1]) / 2);
    h.push((iv[1] - iv[0]) / 2);
  }
  if (a.length < 2) return null;
  const k = a.length;
  const at = (t: number): { p: [number, number]; h: number } => {
    const s = smin + (smax - smin) * t;
    let j = 0;
    while (j < k - 2 && a[j + 1]! < s) j++;
    const f = Math.min(1, Math.max(0, (s - a[j]!) / (a[j + 1]! - a[j]! || 1)));
    const bb = lerp(b[j]!, b[j + 1]!, f);
    const hh = lerp(h[j]!, h[j + 1]!, f);
    return { p: [s * D[0] + bb * N[0], s * D[1] + bb * N[1]], h: hh };
  };
  return { n: k, a, b, h, D, N, at };
}

/** ✂️ Douglas-Peucker over arbitrary feature vectors (kept indices). */
export function reduce(feat: readonly number[][], tol: number): number[] {
  const n = feat.length;
  const keep = new Set([0, n - 1]);
  const dist = (p: number[], q: number[]): number => Math.max(...p.map((x, i) => Math.abs(x - q[i]!)));
  for (;;) {
    const ks = [...keep].sort((p, q) => p - q);
    let worst = 0;
    let wi = -1;
    for (let j = 0; j + 1 < ks.length; j++) {
      const i0 = ks[j]!;
      const i1 = ks[j + 1]!;
      for (let i = i0 + 1; i < i1; i++) {
        const f = (i - i0) / (i1 - i0);
        const q = feat[i0]!.map((x, m) => lerp(x, feat[i1]![m]!, f));
        const e = dist(feat[i]!, q);
        if (e > worst) [worst, wi] = [e, i];
      }
    }
    if (wi < 0 || worst <= tol) break;
    keep.add(wi);
  }
  return [...keep].sort((p, q) => p - q);
}

export type Path = { P: V3; rLat: number; rVert: number }[];

/** 🌿️ A limb path in 3D: x, y from the elevation stripe, z from the front stripe (or `zOf` when the front draws nothing), radii from both stripes. */
export function limbPath(eRings: readonly Ring[], fRings: readonly Ring[], startXY: [number, number], startZ: number, zOf: (x: number, y: number) => number, n = 24): { path: Path; hasFront: boolean } | null {
  const E = stripe(eRings, startXY, n);
  if (E === null) return null;
  const F = fRings.length > 0 ? stripe(fRings, [startZ, startXY[1]], n) : null;
  const path: Path = [];
  for (let i = 0; i < n; i++) {
    const t = (i + 0.5) / n;
    const e = E.at(t);
    const f = F === null ? null : F.at(t);
    const lat = f === null ? e.h : Math.min(Math.max(f.h, e.h / 1.6), e.h * 1.6);
    path.push({ P: [e.p[0], e.p[1], f === null ? zOf(e.p[0], e.p[1]) : f.p[0]], rVert: e.h, rLat: lat });
  }
  return { path, hasFront: F !== null };
}

/** 🧶️ Simplifies a path (tolerance in metres on position and radii) and returns it with the first and last point extended to the stripe ends. */
export function simplify(path: Path, tol: number): Path {
  const keep = reduce(path.map((q) => [q.P[0], q.P[1], q.P[2], q.rLat, q.rVert]), tol);
  return keep.map((i) => path[i]!);
}

/** ↔️ Intervals of solid along the line `axis = s`: even-odd pairing per ring, then the union of the rings. */
export function intervalsAt(rings: readonly Ring[], axis: "x" | "y", s: number): [number, number][] {
  const all: [number, number][] = [];
  for (const r of rings) {
    const xs: number[] = [];
    for (let i = 0, k = r.length - 1; i < r.length; k = i++) {
      const a = r[k]!;
      const b = r[i]!;
      const [pa, pb, qa, qb] = axis === "y" ? [a.y, b.y, a.x, b.x] : [a.x, b.x, a.y, b.y];
      if (pa <= s !== pb <= s) xs.push(qa + ((s - pa) / (pb - pa)) * (qb - qa));
    }
    xs.sort((p, q) => p - q);
    for (let i = 0; i + 1 < xs.length; i += 2) all.push([xs[i]!, xs[i + 1]!]);
  }
  all.sort((p, q) => p[0] - q[0]);
  const out: [number, number][] = [];
  for (const iv of all) {
    const last = out[out.length - 1];
    if (last !== undefined && iv[0] <= last[1]) last[1] = Math.max(last[1], iv[1]);
    else out.push([iv[0], iv[1]]);
  }
  return out;
}

export function bestOverlap(iv: [number, number][], lo: number, hi: number): [number, number] | null {
  let best: [number, number] | null = null;
  let bw = 0;
  for (const [a, b] of iv) {
    const l = Math.max(a, lo);
    const h = Math.min(b, hi);
    if (h - l > bw) [bw, best] = [h - l, [l, h]];
  }
  return best;
}
