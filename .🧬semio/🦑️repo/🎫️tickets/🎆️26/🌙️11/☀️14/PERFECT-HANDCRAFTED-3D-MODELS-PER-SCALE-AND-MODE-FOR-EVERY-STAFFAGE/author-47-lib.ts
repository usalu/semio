/** 🌿 Author 47: visual-hull pieces from a scale's own drawn polygons for plants (a blade, a culm, a plume, a curtain sheet, a crown lobe group): a loft along one world axis whose section centres and half widths are read from two views, expressed in the frame of any (planar-rotated) bone with explicit section rotations.
 * @see ./kit.md */
import { loftOf, type Ctx, type Json, type P2, type Ring, type View, drawn } from "./kit-lib.ts";

export type Poly = Ring;
export type Axis3 = 0 | 1 | 2;
const VIEW_OF: Record<string, { view: View; pa: Axis3; pb: Axis3 }> = { "0,1": { view: "elevation", pa: 0, pb: 1 }, "2,1": { view: "front", pa: 2, pb: 1 }, "0,2": { view: "plan", pa: 0, pb: 2 } };

/** 🔎️ The view holding world axes (a, b) and which of them is the paper x. */
export function viewFor(a: Axis3, b: Axis3): { view: View; swap: boolean } {
  const direct = VIEW_OF[`${a},${b}`];
  if (direct !== undefined) return { view: direct.view, swap: false };
  return { view: VIEW_OF[`${b},${a}`]!.view, swap: true };
}

export const polyOf = (c: Ctx, view: View, id: string): Poly | null => drawn(c, view).find((s) => s.id === id)?.rings[0] ?? null;
export const polysOf = (c: Ctx, view: View, test: (id: string, s: { bone: string; region?: string }) => boolean): Poly[] => drawn(c, view).filter((s) => test(s.id, s)).map((s) => s.rings[0]!);

/** ✂️ A polygon clipped to the half plane `paper[axis] >= value` (sign +1) or `<= value` (sign -1); paper axis "x" | "y". */
export function clipHalf(ring: Poly, axis: "x" | "y", sign: 1 | -1, value = 0): Poly {
  const inside = (p: P2): boolean => (sign > 0 ? p[axis] >= value : p[axis] <= value);
  const out: P2[] = [];
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!;
    const b = ring[(i + 1) % ring.length]!;
    if (inside(a)) out.push(a);
    if (inside(a) !== inside(b)) {
      const t = (value - a[axis]) / (b[axis] - a[axis]);
      out.push({ x: a.x + t * (b.x - a.x), y: a.y + t * (b.y - a.y) });
    }
  }
  return out;
}

/** ↕️ Union span of the other paper axis of several polygons at paper coordinate `at` along `along`. */
export function spanOf(polys: readonly Poly[], along: "x" | "y", at: number): [number, number] | null {
  let lo = Infinity;
  let hi = -Infinity;
  const other = along === "x" ? "y" : "x";
  for (const ring of polys) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!;
    const b = ring[i]!;
    if (a[along] <= at !== b[along] <= at) {
      const q = a[other] + ((at - a[along]) / (b[along] - a[along])) * (b[other] - a[other]);
      lo = Math.min(lo, q);
      hi = Math.max(hi, q);
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

export const extentOf = (polys: readonly Poly[], axis: "x" | "y"): [number, number] => [Math.min(...polys.flatMap((r) => r.map((p) => p[axis]))), Math.max(...polys.flatMap((r) => r.map((p) => p[axis])))];

type V3 = [number, number, number];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 🔄️ Quaternion [x, y, z, w] of the rotation matrix with columns (xa, ya, xa x ya). */
export function quatOfBasis(xa: V3, ya: V3): [number, number, number, number] {
  const za = cross(xa, ya);
  const m = [[xa[0], ya[0], za[0]], [xa[1], ya[1], za[1]], [xa[2], ya[2], za[2]]];
  const tr = m[0]![0]! + m[1]![1]! + m[2]![2]!;
  let q: [number, number, number, number];
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m[2]![1]! - m[1]![2]!) / s, (m[0]![2]! - m[2]![0]!) / s, (m[1]![0]! - m[0]![1]!) / s, s / 4];
  } else if (m[0]![0]! > m[1]![1]! && m[0]![0]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[0]![0]! - m[1]![1]! - m[2]![2]!) * 2;
    q = [s / 4, (m[0]![1]! + m[1]![0]!) / s, (m[0]![2]! + m[2]![0]!) / s, (m[2]![1]! - m[1]![2]!) / s];
  } else if (m[1]![1]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[1]![1]! - m[0]![0]! - m[2]![2]!) * 2;
    q = [(m[0]![1]! + m[1]![0]!) / s, s / 4, (m[1]![2]! + m[2]![1]!) / s, (m[0]![2]! - m[2]![0]!) / s];
  } else {
    const s = Math.sqrt(1 + m[2]![2]! - m[0]![0]! - m[1]![1]!) * 2;
    q = [(m[0]![2]! + m[2]![0]!) / s, (m[1]![2]! + m[2]![1]!) / s, s / 4, (m[1]![0]! - m[0]![1]!) / s];
  }
  const n = Math.hypot(...q);
  return q.map((x) => x / n) as [number, number, number, number];
}

const norm = (a: V3): V3 => { const l = Math.hypot(...a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const unit = (k: Axis3): V3 => (k === 0 ? [1, 0, 0] : k === 1 ? [0, 1, 0] : [0, 0, 1]);

export type PieceOpts = {
  id: string;
  bone: string;
  material: string;
  tone?: number;
  role?: string;
  region?: string;
  axis: Axis3;
  /** polygons per view (only the two views containing the axis are used) */
  polys: Partial<Record<View, readonly Poly[]>>;
  n?: number;
  e?: number;
  minR?: number;
  range?: [number, number];
  slender?: boolean;
  ribbon?: boolean;
  thick?: number;
  wscale?: number;
  tk?: number;
  edge?: number;
  inset?: number;
  custom?: { lo: number; hi: number; at: (s: number) => { ib: [number, number]; ic: [number, number] } | null };
  extra?: Json;
};

/** 🪢️ A hull loft along world axis `axis` through stations whose centre and half extents on the other two world axes are the spans of the given polygons in the two views that contain the axis. Sections carry explicit rotations so that the loft sits in any planar bone frame. `null` when the polygons share no extent. */
export function pieceHull(c: Ctx, o: PieceOpts): Json | null {
  const f = c.state.frames[o.bone]!;
  const A = o.axis;
  const [B, C] = ([0, 1, 2] as Axis3[]).filter((k) => k !== A) as [Axis3, Axis3];
  const vb = viewFor(A, B);
  const vc = viewFor(A, C);
  const pb = o.polys[vb.view] ?? [];
  const pc = o.polys[vc.view] ?? [];
  const alongB = vb.swap ? "y" : "x";
  const alongC = vc.swap ? "y" : "x";
  let lo: number;
  let hi: number;
  if (o.custom !== undefined) {
    lo = Math.max(o.custom.lo, o.range?.[0] ?? -Infinity);
    hi = Math.min(o.custom.hi, o.range?.[1] ?? Infinity);
  } else {
    if (pb.length === 0 || pc.length === 0) return null;
    const eb = extentOf(pb, alongB);
    const ec = extentOf(pc, alongC);
    lo = Math.max(eb[0], ec[0], o.range?.[0] ?? -Infinity);
    hi = Math.min(eb[1], ec[1], o.range?.[1] ?? Infinity);
  }
  if (!(hi > lo)) return null;
  const n = o.n ?? 14;
  const minR = o.minR ?? 0.4 * c.feature;
  const L = (d: V3): V3 => [dot(d, f.u as V3), dot(d, f.v as V3), dot(d, f.w as V3)];
  const eB = unit(B);
  const eC = unit(C);
  const eps = o.edge ?? 0.002;
  type St = { s: number; ib: [number, number]; ic: [number, number] };
  const st: St[] = [];
  for (let i = 0; i <= n; i++) {
    const s = lo + eps + ((hi - lo - 2 * eps) * i) / n;
    const sp = o.custom !== undefined ? o.custom.at(s) : (() => { const ib = spanOf(pb, alongB, s); const ic = spanOf(pc, alongC, s); return ib !== null && ic !== null ? { ib, ic } : null; })();
    if (sp !== null) st.push({ s, ib: sp.ib, ic: sp.ic });
  }
  const centre = (t: St): V3 => {
    const w: V3 = [0, 0, 0];
    w[A] = t.s;
    w[B] = (t.ib[0] + t.ib[1]) / 2;
    w[C] = (t.ic[0] + t.ic[1]) / 2;
    return w;
  };
  const sections: { at: V3; rx: number; ry: number; exponent: number; rotation: [number, number, number, number] }[] = [];
  const k = o.inset ?? 0;
  st.forEach((t, i) => {
    const w = centre(t);
    const d: V3 = [w[0] - f.o[0]!, w[1] - f.o[1]!, w[2] - f.o[2]!];
    if (o.ribbon !== true) {
      sections.push({ at: L(d), rx: Math.max((t.ib[1] - t.ib[0]) / 2 - k, minR), ry: Math.max((t.ic[1] - t.ic[0]) / 2 - k, minR), exponent: o.e ?? 2.6, rotation: quatOfBasis(L(eB), L(eC)) });
      return;
    }
    const p0 = centre(st[Math.max(0, i - 1)]!);
    const p1 = centre(st[Math.min(st.length - 1, i + 1)]!);
    const T = norm([p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]]);
    const hw = cross(T, [0, 1, 0]);
    const Wd = Math.hypot(...hw) < 1e-4 ? ([0, 0, 1] as V3) : norm(hw);
    const Nr = norm(cross(T, Wd));
    let rx: number;
    let ry: number;
    if (A === 1) {
      const wx = Math.abs(Wd[0]);
      const wz = Math.abs(Wd[2]);
      const det = wx * wx - wz * wz;
      const ix = t.ib[1] - t.ib[0];
      const iz = t.ic[1] - t.ic[0];
      let wd: number;
      let th: number;
      if (Math.abs(det) > 0.05) {
        wd = (wx * ix - wz * iz) / det;
        th = (wx * iz - wz * ix) / det;
      } else {
        wd = (ix + iz) / (2 * (wx + wz));
        th = 0;
      }
      rx = Math.max((wd * (o.wscale ?? 1)) / 2 - k, minR);
      ry = Math.min(Math.max(th / 2 - k, o.thick ?? 0, minR), Math.max(rx, minR));
    } else {
      const horizB = B !== 1;
      const hInt = horizB ? t.ib : t.ic;
      const vInt = horizB ? t.ic : t.ib;
      const hk = horizB ? B : C;
      const dA = p1[A] - p0[A] || 1;
      const m = (p1[hk] - p0[hk]) / dA;
      const wid = ((hInt[1] - hInt[0]) / Math.sqrt(1 + m * m)) * (o.wscale ?? 1);
      rx = Math.max(wid / 2 - k, minR);
      const vk = horizB ? C : B;
      const my = Math.abs((p1[vk] - p0[vk]) / dA);
      const spread = my * wid * Math.abs(Wd[A]);
      const denom = 2 * (Math.abs(Nr[vk]) + Math.abs(Nr[A]) * my);
      ry = Math.min(Math.max((((vInt[1] - vInt[0]) - spread) / (denom || 1)) * (o.tk ?? 1) - k, o.thick ?? 0, minR), Math.max(rx, minR));
    }
    sections.push({ at: L(d), rx, ry, exponent: o.e ?? 2.6, rotation: quatOfBasis(L(Wd), L(Nr)) });
  });
  if (sections.length < 2) return null;
  const solid = loftOf({ id: o.id, bone: o.bone, material: o.material, tone: o.tone, role: o.role ?? "body" }, sections);
  if (o.region !== undefined) solid.region = o.region;
  if (o.slender === true) solid.slender = true;
  return o.extra === undefined ? solid : { ...solid, ...o.extra };
}

/** ✂️ Ramer-Douglas-Peucker simplification of a closed ring to tolerance `tol`. */
export function simplifyRing(ring: Poly, tol: number): Poly {
  const dp = (pts: P2[]): P2[] => {
    if (pts.length < 3) return pts;
    const a = pts[0]!;
    const b = pts[pts.length - 1]!;
    const L = Math.hypot(b.x - a.x, b.y - a.y) || 1e-12;
    let wi = 0;
    let wd = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const d = Math.abs((b.x - a.x) * (a.y - pts[i]!.y) - (a.x - pts[i]!.x) * (b.y - a.y)) / L;
      if (d > wd) [wd, wi] = [d, i];
    }
    if (wd <= tol) return [a, b];
    return [...dp(pts.slice(0, wi + 1)).slice(0, -1), ...dp(pts.slice(wi))];
  };
  let half = Math.floor(ring.length / 2);
  const left = dp(ring.slice(0, half + 1));
  const right = dp([...ring.slice(half), ring[0]!]);
  return [...left.slice(0, -1), ...right.slice(0, -1)];
}

/** 🧮️ Point in the union of polygons. */
export function inUnion(polys: readonly Poly[], x: number, y: number): boolean {
  for (const r of polys) {
    let w = false;
    for (let i = 0, j = r.length - 1; i < r.length; j = i++) if (r[i]!.y > y !== r[j]!.y > y && x < ((r[j]!.x - r[i]!.x) * (y - r[i]!.y)) / (r[j]!.y - r[i]!.y) + r[i]!.x) w = !w;
    if (w) return true;
  }
  return false;
}
