import { loftOf, superellipseSection, selectStations, type Json, type Section, type Station, type V3 } from "./kit-lib.ts";

export type Pt = readonly [number, number];

/** 📐️ Crossings of a closed polygon with the line coordinate `k` (0 = first, 1 = second) equal to `s`; returns min and max of the other coordinate or null. */
export function spanAt(poly: readonly Pt[], k: 0 | 1, s: number): [number, number] | null {
  const o = (1 - k) as 0 | 1;
  const ys: number[] = [];
  for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
    const a = poly[j]!;
    const b = poly[i]!;
    if (a[k] <= s !== b[k] <= s) ys.push(a[o] + ((s - a[k]) / (b[k] - a[k])) * (b[o] - a[o]));
  }
  return ys.length < 2 ? null : [Math.min(...ys), Math.max(...ys)];
}

export const extent = (poly: readonly Pt[], k: 0 | 1): [number, number] => [Math.min(...poly.map((p) => p[k])), Math.max(...poly.map((p) => p[k]))];

export type Prof = { s: number; lo: number; hi: number };

/** 📈️ Smoothed span profile of a polygon along coordinate `k`: `n` stations, triangular moving average of half width `win` (ends kept exact, so tips stay tips). */
export function profile(poly: readonly Pt[], k: 0 | 1, n: number, win: number, clip?: number, peak = 0): Prof[] {
  const [a, b0] = extent(poly, k);
  const b = clip === undefined ? b0 : Math.min(b0, clip);
  const eps = (b - a) * 1e-4;
  const dense: Prof[] = [];
  const m = n * 4;
  for (let i = 0; i <= m; i++) {
    const s = a + eps + ((b - a - 2 * eps) * i) / m;
    const sp = spanAt(poly, k, s);
    if (sp !== null) dense.push({ s, lo: sp[0], hi: sp[1] });
  }
  const out: Prof[] = [];
  for (let i = 0; i < n; i++) {
    const s = a + ((b - a) * i) / (n - 1);
    if (i === 0 || i === n - 1) {
      const nearest = dense.reduce((p, q) => (Math.abs(q.s - s) < Math.abs(p.s - s) ? q : p));
      out.push({ s, lo: nearest.lo, hi: nearest.hi });
      continue;
    }
    let w = 0;
    let lo = 0;
    let hi = 0;
    let mn = Infinity;
    let mx = -Infinity;
    for (const d of dense) {
      const t = 1 - Math.abs(d.s - s) / win;
      if (t <= 0) continue;
      w += t;
      lo += t * d.lo;
      hi += t * d.hi;
      mn = Math.min(mn, d.lo);
      mx = Math.max(mx, d.hi);
    }
    if (w > 0) out.push({ s, lo: lo / w + peak * (mn - lo / w), hi: hi / w + peak * (mx - hi / w) });
  }
  return out;
}

export type Mat = { material: string; tone?: number };

/** 🍃️ A blade loft along bone axis u: width (w, the polygon's second coordinate) from the polygon's smoothed span, thickness (v) from `thick(s)`, section exponent from `exp(s)`. */
export function bladeLoft(id: string, bone: string, m: Mat, poly: readonly Pt[], o: { n: number; win: number; tol: number; fmin: number; tmin?: number; gap?: number; clip?: number; peak?: number; thick: (t: number) => number; exp: (t: number) => number; role?: string }): Json {
  const pr = profile(poly, 0, o.n, o.win, o.clip, o.peak ?? 0);
  const s0 = pr[0]!.s;
  const s1 = pr[pr.length - 1]!.s;
  const raw: Station[] = pr.map((p) => {
    const t = (p.s - s0) / (s1 - s0 || 1);
    return { s: p.s, c1: 0, c2: (p.lo + p.hi) / 2, rx: Math.max((p.hi - p.lo) / 2, o.fmin), ry: Math.max(o.thick(t), o.tmin ?? o.fmin), e: o.exp(t) };
  });
  const st = withMaxGap(raw, selectStations(raw, o.tol, 0), o.gap ?? 0.12).map((q) => ({ ...q, rx: Math.max(q.rx, o.fmin), ry: Math.max(q.ry, o.tmin ?? o.fmin) }));
  const sections: Section[] = st.map((q) => ({ ...superellipseSection([q.s, q.c1 ?? 0, q.c2 ?? 0] as V3, q.rx, q.ry, q.e ?? 2, quatOf([0, 0, 1], [0, 1, 0], [-1, 0, 0])) }));
  return { ...loftOf({ id, bone, material: m.material, tone: m.tone, role: "body" }, sections) };
}

/** 🪡️ Adds raw stations wherever two kept stations lie farther apart than `gap` (metres), so a loft never spans a long ruled surface. */
export function withMaxGap(raw: readonly Station[], kept: readonly Station[], gap: number): Station[] {
  const out: Station[] = [];
  for (let i = 0; i < kept.length; i++) {
    out.push(kept[i]!);
    const nxt = kept[i + 1];
    if (nxt === undefined) continue;
    const n = Math.ceil((nxt.s - kept[i]!.s) / gap);
    for (let k = 1; k < n; k++) {
      const s = kept[i]!.s + ((nxt.s - kept[i]!.s) * k) / n;
      out.push(raw.reduce((a, b) => (Math.abs(b.s - s) < Math.abs(a.s - s) ? b : a)));
    }
  }
  return out.filter((q, i) => i === 0 || q.s > out[i - 1]!.s);
}

export type V = readonly [number, number, number];
export const vadd = (a: V, b: V): V => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const vsub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const vmul = (a: V, k: number): V => [a[0] * k, a[1] * k, a[2] * k];
export const vdot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const vcross = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const vlen = (a: V): number => Math.hypot(a[0], a[1], a[2]);
export const vnorm = (a: V): V => vmul(a, 1 / (vlen(a) || 1));

/** 🧭️ Quaternion `[x, y, z, w]` of the rotation whose columns are the orthonormal axes `x`, `y`, `z` (a loft section with this `rotation` lies in the plane spanned by `x` and `y`). */
export function quatOf(x: V, y: V, z: V): [number, number, number, number] {
  const m00 = x[0], m10 = x[1], m20 = x[2], m01 = y[0], m11 = y[1], m21 = y[2], m02 = z[0], m12 = z[1], m22 = z[2];
  const tr = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s];
  }
  return q;
}

/** 🖋️ Samples an SVG path (`M`, `C`, `L`, `Z` absolute only, as the catalog draws) into a closed polygon with `steps` points per curve. */
export function pathPoints(d: string, steps = 4): Pt[] {
  const tok = d.match(/[MCLZ]|-?\d*\.?\d+(?:e-?\d+)?/gi) ?? [];
  const pts: Pt[] = [];
  let i = 0;
  let cur: Pt = [0, 0];
  const num = (): number => Number(tok[i++]);
  while (i < tok.length) {
    const cmd = tok[i++]!;
    if (cmd === "M" || cmd === "L") {
      cur = [num(), num()];
      pts.push(cur);
    } else if (cmd === "C") {
      for (;;) {
        const nxt = tok[i];
        if (nxt === undefined || /[A-Za-z]/.test(nxt)) break;
        const [x1, y1, x2, y2, x3, y3] = [num(), num(), num(), num(), num(), num()];
        for (let k = 1; k <= steps; k++) {
          const t = k / steps, m = 1 - t;
          pts.push([m * m * m * cur[0] + 3 * m * m * t * x1 + 3 * m * t * t * x2 + t * t * t * x3, m * m * m * cur[1] + 3 * m * m * t * y1 + 3 * m * t * t * y2 + t * t * t * y3]);
        }
        cur = [x3, y3];
      }
    }
  }
  return pts;
}

export type Q = readonly [number, number, number, number];
export const qmul = (a: Q, b: Q): [number, number, number, number] => [
  a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
  a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
  a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
  a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
];
export const qconj = (a: Q): [number, number, number, number] => [-a[0], -a[1], -a[2], a[3]];
