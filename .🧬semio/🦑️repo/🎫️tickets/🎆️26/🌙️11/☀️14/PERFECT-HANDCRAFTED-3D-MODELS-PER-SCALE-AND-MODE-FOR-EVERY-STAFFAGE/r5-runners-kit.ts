/** 🪡 Round 5 constructors shared by `humans/jogger` and `humans/running-child`: smooth lofts through hand-stated key stations (a uniform Catmull-Rom spline through the stated rows, so a limb is one continuous anatomical form without crease facets between stations), tubes whose sections stand perpendicular to a hand-stated curved path (curled finger groups, thumbs, straps), and the section frame of every loft section as the product builds it (for the 2D outlines).
 * @see ./r3-jogger-kit.ts
 * @see ./r5-brief.md */
import { Kit, ROT, type Json, type Row, type V3 } from "./r3-jogger-kit.ts";

export { Kit, ROT, type Json, type Row, type V3 };

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const cr = (p0: number, p1: number, p2: number, p3: number, t: number): number => 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t);

/** 〰️ The rows of a spline through the stated key rows: `k` rows between each pair of keys (the keys themselves stay exact); exponents follow linearly. */
export function smooth(rows: readonly Row[], k = 2): Row[] {
  if (rows.length < 3 || k === 0) return [...rows];
  const at = (i: number): Row => rows[Math.max(0, Math.min(rows.length - 1, i))]!;
  const out: Row[] = [];
  for (let i = 0; i < rows.length - 1; i++) {
    const [a, b, c, d] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    for (let j = 0; j <= k; j++) {
      if (j === 0 && i > 0) continue;
      const t = j / (k + 1);
      const v = [0, 1, 2, 3, 4].map((n) => r5(cr(a[n]!, b[n]!, c[n]!, d[n]!, t)));
      const e = (b[5] ?? 2) + ((c[5] ?? 2) - (b[5] ?? 2)) * t;
      out.push([v[0]!, v[1]!, v[2]!, Math.max(v[3]!, 0.0005), Math.max(v[4]!, 0.0005), r5(e)] as Row);
    }
  }
  out.push(rows[rows.length - 1]!);
  return out;
}

/** 🧵 A loft along a curved path of `[u, v, w, rx, ry, e?]` rows whose sections stand perpendicular to the path (the product's frame: x = the bone axis projected across the tangent, y = tangent x x). */
export function tube(k: Kit, id: string, bone: string, material: string, tone: number, rows: readonly Row[], region?: string, role = "body", e = 2): Json {
  const sections = rows.map(([u, v, w, rx, ry, ex]) => ({ at: [r5(u), r5(v), r5(w)], rx: r5(rx), ry: r5(ry), exponent: ex ?? e }));
  const solid = { id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true };
  k.solids.push(solid);
  if (region !== undefined) k.region.set(id, region);
  return solid;
}

/** 🌀 A smooth loft: `k.loft` through the spline of the stated key rows. */
export const sloft = (k: Kit, id: string, bone: string, material: string, tone: number, keys: readonly Row[], region?: string, role = "body", e = 2, n = 2): Json => k.loft(id, bone, material, tone, smooth(keys, n), region, role, e);

type Q = readonly number[];
const rot = (q: Q, p: V3): V3 => {
  const [x, y, z, w] = q as [number, number, number, number];
  const [tx, ty, tz] = [2 * (y * p[2] - z * p[1]), 2 * (z * p[0] - x * p[2]), 2 * (x * p[1] - y * p[0])];
  return [p[0] + w * tx + (y * tz - z * ty), p[1] + w * ty + (z * tx - x * tz), p[2] + w * tz + (x * ty - y * tx)];
};
const norm = (a: V3): V3 => {
  const l = Math.hypot(a[0], a[1], a[2]) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 🧭 The x and y axes of every section of a loft in its bone frame, as the product builds them. */
export function sectionAxes(sections: readonly Json[]): [V3, V3][] {
  return sections.map((s, i) => {
    if (s.rotation !== undefined) return [rot(s.rotation, [1, 0, 0]), rot(s.rotation, [0, 1, 0])];
    const [a, b] = [sections[Math.max(0, i - 1)]!.at as V3, sections[Math.min(sections.length - 1, i + 1)]!.at as V3];
    const t = norm([b[0] - a[0], b[1] - a[1], b[2] - a[2]]);
    const d = t[0];
    let x: V3 = [1 - d * t[0], -d * t[1], -d * t[2]];
    if (Math.hypot(...x) < 1e-6) x = [0, 0, 1];
    x = norm(x);
    return [x, cross(t, x)];
  });
}

/** ⭕ The superelliptic ring of one loft section in its bone frame, `n` points. */
export function ringOf(s: Json, axes: [V3, V3], n = 48): V3[] {
  const f = (x: number): number => Math.sign(x) * Math.abs(x) ** (2 / (s.exponent ?? 2));
  const [x, y] = axes;
  return Array.from({ length: n }, (_, i) => {
    const a = (2 * Math.PI * i) / n;
    const [c, d] = [s.rx * f(Math.cos(a)), s.ry * f(Math.sin(a))];
    return [s.at[0] + c * x[0] + d * y[0], s.at[1] + c * x[1] + d * y[1], s.at[2] + c * x[2] + d * y[2]] as V3;
  });
}
