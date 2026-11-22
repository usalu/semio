/** 🍃️ PW-17 the leaf of Monstera deliciosa in [u, w] of its blade bone (u from the petiole insertion at the sinus to the apex, w across): a cordate-ovate blade whose half outline is a hand-drawn control table, pinnatifid slits that follow the lateral veins from the margin towards the midrib (V-shaped, opening 26 degrees), and one row of elliptic fenestrae between the slits; simplified per scale.
 * @see ./potted-monstera.ts */
import { ellipseRing, type P, type Scale } from "./lib.ts";

/** 🧾️ The hand-stated form of one blade: length, width over length, slits and holes per side, slit depth (share of the way to the midrib), and the shift of the left slits (asymmetry). */
export type MonsteraBlade = { L: number; wr: number; slits: number; holes: number; depth: number; shift: number };

/** ✏️ Control table of the right half margin: (share of the margin from sinus to apex, u / L, w / half width), drawn by hand from the form of adult leaves (basal lobes behind the insertion, widest at 0.2 L, short-acuminate apex). */
const MARGIN: [number, number, number][] = [
  [0, 0, 0], [0.04, -0.05, -0.18], [0.1, -0.1, -0.37], [0.17, -0.1, -0.56], [0.25, -0.04, -0.77], [0.33, 0.06, -0.92], [0.42, 0.2, -1], [0.52, 0.36, -0.98],
  [0.62, 0.52, -0.9], [0.72, 0.67, -0.74], [0.81, 0.8, -0.54], [0.89, 0.9, -0.32], [0.95, 0.96, -0.14], [1, 1, 0],
];

/** 〰️ The right margin at share t (Catmull-Rom through the control table). */
function margin(b: MonsteraBlade, t: number): P {
  const i = Math.max(1, MARGIN.findIndex((r) => r[0] >= t));
  const [p0, p1, p2, p3] = [MARGIN[Math.max(0, i - 2)]!, MARGIN[i - 1]!, MARGIN[i]!, MARGIN[Math.min(MARGIN.length - 1, i + 1)]!];
  const s = (t - p1[0]) / (p2[0] - p1[0] || 1);
  const cr = (a: number, b0: number, c: number, d: number) => 0.5 * (2 * b0 + (-a + c) * s + (2 * a - 5 * b0 + 4 * c - d) * s * s + (-a + 3 * b0 - 3 * c + d) * s * s * s);
  return [b.L * cr(p0[1], p1[1], p2[1], p3[1]), (b.L * b.wr / 2) * cr(p0[2], p1[2], p2[2], p3[2])];
}

/** 🔪️ The slits of one side: margin share of the slit, its inner end (towards the midrib and back along the vein) and its half opening along the margin. */
function slitsOf(b: MonsteraBlade, side: 1 | -1, scale: Scale): { t: number; dt: number; end: P }[] {
  if (scale >= 200 || b.slits === 0) return [];
  const n = b.slits;
  return Array.from({ length: n }, (_, i) => {
    const t = 0.3 + (0.85 - 0.3) * (i + 0.5 + (side === 1 ? b.shift : 0)) / n;
    const m = margin(b, t);
    const toward: P = [m[0] - 0.1 * b.L, 0];
    const depth = b.depth * (1 - 0.25 * Math.abs(t - 0.55));
    const end: P = [m[0] + depth * (toward[0] - m[0]), m[1] + depth * (toward[1] - m[1])];
    const reach = Math.hypot(end[0] - m[0], end[1] - m[1]);
    const half = reach * Math.tan((13 * Math.PI) / 180);
    const dt = half / (b.L * 1.9);
    return { t, dt, end };
  });
}

/** 📍️ Whether a point lies inside a ring (even-odd). */
const inside = (ring: readonly P[], p: P): boolean => {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const [a, b] = [ring[i]!, ring[j]!]; if (a[1] > p[1] !== b[1] > p[1] && p[0] < ((b[0] - a[0]) * (p[1] - a[1])) / (b[1] - a[1]) + a[0]) c = !c; }
  return c;
};

/** 📏️ The distance of a point to the edges of a ring. */
const clearance = (ring: readonly P[], p: P): number => Math.min(...ring.map((a, i) => { const b = ring[(i + 1) % ring.length]!; const [dx, dy] = [b[0] - a[0], b[1] - a[1]]; const t = Math.max(0, Math.min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / (dx * dx + dy * dy || 1))); return Math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy); }));

/** 🍃️ The outline and holes of a blade at a scale (1:50 every slit and hole, 1:100 slits on a coarser margin, 1:200 and 1:500 the entire margin). */
export function monsteraBlade(b: MonsteraBlade, scale: Scale): { outer: P[]; holes: P[][] } {
  const step = scale === 50 ? 0.012 : scale === 100 ? 0.03 : scale === 200 ? 0.07 : 0.14;
  const half = (side: 1 | -1): P[] => {
    const cuts = slitsOf(b, side, scale);
    const out: P[] = [];
    let t = 0;
    const ts: number[] = [];
    for (; t <= 1 + 1e-9; t += step) ts.push(Math.min(1, t));
    if (ts[ts.length - 1]! < 1) ts.push(1);
    for (const c of cuts) ts.push(c.t - c.dt, c.t + c.dt);
    const sorted = [...new Set(ts.map((x) => Math.round(x * 1e6) / 1e6))].sort((x, y) => x - y).filter((x) => !cuts.some((c) => x > c.t - c.dt + 1e-9 && x < c.t + c.dt - 1e-9));
    for (const x of sorted) {
      const m = margin(b, x);
      out.push([m[0], side === -1 ? m[1] : -m[1]]);
      const c = cuts.find((k) => Math.abs(k.t - k.dt - x) < 1e-6);
      if (c) out.push([c.end[0], side === -1 ? c.end[1] : -c.end[1]]);
    }
    return out;
  };
  const right = half(-1);
  const left = half(1).reverse();
  const outer = [...right.slice(0, -1), ...left.slice(0, -1)];
  const holes: P[][] = [];
  if (scale === 50 && b.holes > 0) for (const side of [-1, 1] as const) {
    const cuts = slitsOf(b, side === -1 ? -1 : 1, scale);
    for (let i = 0; i < Math.min(b.holes, cuts.length - 1); i++) {
      const [e0, e1] = [cuts[i]!.end, cuts[i + 1]!.end];
      const wa = Math.abs(e0[1]) * 0.55 + Math.abs(e1[1]) * 0.45;
      const cu = (e0[0] + e1[0]) / 2 + 0.02 * b.L;
      const cw = side * wa * 0.62 * (side === -1 ? 1 : 1);
      const gap = Math.hypot(e1[0] - e0[0], e1[1] - e0[1]);
      const a = Math.max(0.045, Math.min(0.075, 0.4 * gap));
      const r = Math.max(0.026, Math.min(0.034, 0.5 * a));
      for (let k = 1; k >= 0.6; k -= 0.1) {
        const ring = ellipseRing(cu, cw, a * k, r * k, 14, side * 0.45);
        if (ring.every((p) => inside(outer, p) && clearance(outer, p) >= 0.008)) { holes.push(ring); break; }
      }
    }
  }
  return { outer, holes };
}
