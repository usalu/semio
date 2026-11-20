/** 🌵️ Round 3, vegetation/agave: the century plant (Agave americana) stated by hand in world space in the default pose `summer-turgid`. Twenty-eight leaves in the 137.5 degree spiral of the rosette, oldest (lowest, spreading) first, each with its insertion on the short stem, its rise and recurve, its length, width and thickness; the leaf blade as a channelled (V) section with a rounded keel, a clasping base, a narrow neck above the base, the widest part at mid-leaf and an acuminate tip with the terminal spine. Sources in `r3-vegetation-agave.md` (Gentry 1982, Irish & Irish 2000, Nobel 1988).
 * @see ./r3-vegetation-agave.md */
import type { Frame, V3 } from "./volumes-lib.ts";
import { add, cross, dirOf, lin, mul, sideOf, sub, table, unit, upOf } from "./r3-agave-kit.ts";

/** 🍃️ One leaf as stated by hand: spiral index k (0 oldest), azimuth, insertion height on the stem and radius from its axis, elevation of the blade at the base and at the tip, recurve exponent, length along the blade, greatest width, base and mid-blade thickness. */
export type Leaf = { k: number; az: number; h: number; r: number; e0: number; e1: number; p: number; L: number; W: number; Tb: number; Tm: number };

/** 📋️ The rosette. Lowest eight spread and arch down to the bed (tips 2 to 12 cm above it), the next eight rise and level out, seven stand at 50 to 65 degrees with a gentle recurve, the youngest five stand almost upright round the spear. Azimuths follow the golden angle (137.5 degrees) from 10 degrees; lengths shrink and widths narrow towards the heart, as in the field (leaves 1-2 m x 15-25 cm, Gentry 1982). */
export const LEAVES: readonly Leaf[] = [
  { k: 0, az: 10.0, h: 0.05, r: 0.1, e0: 15, e1: -23.1, p: 1.5, L: 1.181, W: 0.215, Tb: 0.085, Tm: 0.026 },
  { k: 1, az: 147.5, h: 0.058, r: 0.099, e0: 17, e1: -26.1, p: 1.6, L: 1.173, W: 0.215, Tb: 0.085, Tm: 0.026 },
  { k: 2, az: -75.0, h: 0.067, r: 0.097, e0: 14, e1: -22.6, p: 1.5, L: 1.126, W: 0.21, Tb: 0.084, Tm: 0.026 },
  { k: 3, az: 62.5, h: 0.075, r: 0.095, e0: 18, e1: -27.2, p: 1.6, L: 1.150, W: 0.215, Tb: 0.083, Tm: 0.026 },
  { k: 4, az: -160.0, h: 0.084, r: 0.093, e0: 16, e1: -25.0, p: 1.5, L: 1.186, W: 0.21, Tb: 0.082, Tm: 0.025 },
  { k: 5, az: -22.5, h: 0.092, r: 0.091, e0: 19, e1: -28.4, p: 1.6, L: 1.197, W: 0.21, Tb: 0.081, Tm: 0.025 },
  { k: 6, az: 115.0, h: 0.101, r: 0.089, e0: 17, e1: -27.7, p: 1.6, L: 1.153, W: 0.21, Tb: 0.08, Tm: 0.025 },
  { k: 7, az: -107.4, h: 0.109, r: 0.087, e0: 20, e1: -29.9, p: 1.7, L: 1.159, W: 0.205, Tb: 0.079, Tm: 0.025 },
  { k: 8, az: 30.1, h: 0.118, r: 0.085, e0: 29, e1: -8, p: 1.8, L: 1.16, W: 0.205, Tb: 0.078, Tm: 0.025 },
  { k: 9, az: 167.6, h: 0.126, r: 0.084, e0: 31, e1: -5, p: 1.8, L: 1.15, W: 0.2, Tb: 0.077, Tm: 0.024 },
  { k: 10, az: -54.9, h: 0.135, r: 0.082, e0: 33, e1: -2, p: 1.8, L: 1.15, W: 0.2, Tb: 0.076, Tm: 0.024 },
  { k: 11, az: 82.6, h: 0.143, r: 0.08, e0: 36, e1: 2, p: 1.8, L: 1.17, W: 0.2, Tb: 0.075, Tm: 0.024 },
  { k: 12, az: -139.9, h: 0.152, r: 0.078, e0: 38, e1: 5, p: 1.8, L: 1.13, W: 0.195, Tb: 0.074, Tm: 0.024 },
  { k: 13, az: -2.4, h: 0.16, r: 0.076, e0: 41, e1: 8, p: 1.8, L: 1.12, W: 0.195, Tb: 0.073, Tm: 0.023 },
  { k: 14, az: 135.1, h: 0.169, r: 0.074, e0: 43, e1: 11, p: 1.8, L: 1.11, W: 0.19, Tb: 0.072, Tm: 0.023 },
  { k: 15, az: -87.4, h: 0.177, r: 0.073, e0: 46, e1: 14, p: 1.8, L: 1.1, W: 0.19, Tb: 0.071, Tm: 0.023 },
  { k: 16, az: 50.1, h: 0.186, r: 0.071, e0: 52, e1: 26, p: 1.6, L: 1.08, W: 0.185, Tb: 0.07, Tm: 0.023 },
  { k: 17, az: -172.4, h: 0.194, r: 0.069, e0: 54, e1: 29, p: 1.6, L: 1.07, W: 0.18, Tb: 0.069, Tm: 0.022 },
  { k: 18, az: -34.9, h: 0.203, r: 0.067, e0: 56, e1: 32, p: 1.6, L: 1.06, W: 0.18, Tb: 0.068, Tm: 0.022 },
  { k: 19, az: 102.7, h: 0.211, r: 0.065, e0: 59, e1: 36, p: 1.6, L: 1.05, W: 0.175, Tb: 0.067, Tm: 0.022 },
  { k: 20, az: -119.8, h: 0.22, r: 0.063, e0: 61, e1: 40, p: 1.6, L: 1.04, W: 0.17, Tb: 0.066, Tm: 0.022 },
  { k: 21, az: 17.7, h: 0.228, r: 0.061, e0: 63, e1: 44, p: 1.6, L: 1.03, W: 0.17, Tb: 0.065, Tm: 0.021 },
  { k: 22, az: 155.2, h: 0.237, r: 0.06, e0: 65, e1: 48, p: 1.6, L: 1.02, W: 0.165, Tb: 0.064, Tm: 0.021 },
  { k: 23, az: -67.3, h: 0.245, r: 0.058, e0: 70, e1: 56, p: 1.4, L: 1.0, W: 0.16, Tb: 0.062, Tm: 0.021 },
  { k: 24, az: 70.2, h: 0.254, r: 0.056, e0: 73, e1: 61, p: 1.4, L: 0.98, W: 0.155, Tb: 0.06, Tm: 0.02 },
  { k: 25, az: -152.3, h: 0.262, r: 0.054, e0: 76, e1: 66, p: 1.4, L: 0.96, W: 0.15, Tb: 0.058, Tm: 0.02 },
  { k: 26, az: -14.8, h: 0.271, r: 0.052, e0: 79, e1: 71, p: 1.4, L: 0.94, W: 0.145, Tb: 0.056, Tm: 0.02 },
  { k: 27, az: 122.7, h: 0.279, r: 0.05, e0: 82, e1: 76, p: 1.4, L: 0.92, W: 0.14, Tb: 0.054, Tm: 0.02 },
];

/** ✂️ Share of the blade length at the knee between the proximal (a) and distal (b) bone of every leaf. */
export const KNEE = 0.5;

/** 🗡️ Terminal spine: length and base radius (Agave americana: 3-5 cm, dark brown, conical, Gentry 1982). */
export const SPINE = { length: 0.04, radius: 0.004 };

/** 🌱️ The stem (caudex) under the rosette: height of the bone, and the profile [height, radius] of the short, thick stem that the leaf bases hide. */
export const CAUDEX = { height: 0.3, profile: [[0, 0.13], [0.06, 0.135], [0.18, 0.123], [0.26, 0.105], [0.3, 0.07]] as [number, number][] };

/** 🗼️ The spear: the cone of tightly furled young leaves in the heart, from the stem top to the declared height (1.445 m with its spine), profile [along the bone, radius]. */
export const SPEAR = { base: 0.3, length: 1.145, profile: [[0, 0.072], [0.2, 0.07], [0.45, 0.062], [0.7, 0.049], [0.9, 0.032], [1.04, 0.016], [1.105, 0.007], [1.145, 0]] as [number, number][] };

export const leafId = (k: number): string => `leaf-${String(k + 1).padStart(2, "0")}`;
export const regionOf = (k: number): "leaves-spreading" | "leaves-erect" => (k < 16 ? "leaves-spreading" : "leaves-erect");

/** 📏️ Half width of the blade as a share of the greatest width, along the blade: broad clasping base, narrow neck, widest at mid-leaf, acuminate tip. */
export const WIDTH: readonly [number, number][] = [[0, 0.56], [0.05, 0.44], [0.11, 0.36], [0.22, 0.41], [0.4, 0.49], [0.55, 0.5], [0.7, 0.45], [0.82, 0.36], [0.91, 0.23], [0.97, 0.1], [1, 0.03]];

/** 🫓️ Thickness at the midline: thick fleshy base to a thin tip. */
export const thickness = (l: Leaf, s: number): number => table([[0, l.Tb], [0.05, l.Tb * 0.72], [0.14, l.Tm * 1.7], [0.3, l.Tm * 1.2], [0.55, l.Tm], [0.8, l.Tm * 0.75], [0.93, l.Tm * 0.55], [1, l.Tm * 0.35]], s);

/** 〰️ Half angle of the channel (upper face guttered, wings rising to the margins): deep at the clasping base and towards the folded tip. */
export const gammaDeg = (s: number): number => table([[0, 30], [0.15, 25], [0.5, 17], [0.8, 19], [0.95, 27], [1, 34]], s);

export const halfWidth = (l: Leaf, s: number): number => l.W * table(WIDTH, s);

/** 🌀️ The blade of one leaf in world space: centre line by arc length, tangent, upward normal, lateral, and the knee and tip. */
export type Blade = { leaf: Leaf; at: (s: number) => V3; tangent: (s: number) => V3; normal: (s: number) => V3; side: V3; p0: V3; knee: V3; tip: V3 };

const STEPS = 400;

/** 🌀️ Integrates the centre line of a leaf from its insertion with the elevation law el(s) = e0 + (e1 - e0) s^p. */
export function blade(l: Leaf): Blade {
  const el = (s: number): number => l.e0 + (l.e1 - l.e0) * Math.pow(s, l.p);
  const p0: V3 = [l.r * Math.cos(l.az * Math.PI / 180), l.h, -l.r * Math.sin(l.az * Math.PI / 180)];
  const pts: V3[] = [p0];
  for (let i = 1; i <= STEPS; i++) {
    const s = (i - 0.5) / STEPS;
    pts.push(add(pts[i - 1]!, mul(dirOf(l.az, el(s)), l.L / STEPS)));
  }
  const at = (s: number): V3 => {
    const x = Math.max(0, Math.min(1, s)) * STEPS;
    const i = Math.min(STEPS - 1, Math.floor(x));
    const f = x - i;
    const base = lin(pts[i]!, 1 - f, pts[i + 1]!, f);
    return s > 1 ? add(base, mul(dirOf(l.az, el(1)), (s - 1) * l.L)) : s < 0 ? add(base, mul(dirOf(l.az, el(0)), s * l.L)) : base;
  };
  return { leaf: l, at, tangent: (s) => dirOf(l.az, el(Math.max(0, Math.min(1, s)))), normal: (s) => upOf(l.az, el(Math.max(0, Math.min(1, s)))), side: sideOf(l.az), p0, knee: at(KNEE), tip: at(1) };
}

/** 🦴️ The two bone frames of a leaf in the default pose: proximal along the chord insertion-knee, distal along knee-tip, normals upward in the leaf's vertical plane. */
export function leafFrames(b: Blade): { a: Frame; b: Frame; La: number; Lb: number } {
  const ua = unit(sub(b.knee, b.p0));
  const ub = unit(sub(b.tip, b.knee));
  const w = b.side;
  const fa: Frame = { o: b.p0, u: ua, v: cross(w, ua), w };
  const fb: Frame = { o: b.knee, u: ub, v: cross(w, ub), w };
  return { a: fa, b: fb, La: Math.hypot(...sub(b.knee, b.p0)), Lb: Math.hypot(...sub(b.tip, b.knee)) };
}
