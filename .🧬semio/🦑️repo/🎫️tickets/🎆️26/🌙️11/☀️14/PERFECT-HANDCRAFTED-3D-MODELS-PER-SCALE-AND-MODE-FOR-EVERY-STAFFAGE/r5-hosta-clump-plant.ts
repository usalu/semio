/** 🌿️ Round 5, vegetation/hosta-clump: the clump (Hosta sieboldiana type) stated by hand in world space in the default pose `summer-full`. Four shoots (eyes) on the rhizome crown carry 25 leaves in three tiers: ten spreading outer leaves whose blades arch down to the bed, nine rising leaves, six upright heart leaves. Each leaf is a curved petiole from its insertion in the shoot and a cordate-ovate, cupped blade with basal lobes, sinus, midrib and campylodromous veins. Sources in `r3-vegetation-hosta-clump.md` (Schmid 1991, Grenfell and Shadrack 2009).
 * @see ./r3-vegetation-hosta-clump.md (Round 5 handcraft) */
import type { Frame, V3 } from "./volumes-lib.ts";
import { add, cross, dirOf, lin, mul, sideOf, sub, table, unit, upOf } from "./r3-agave-kit.ts";

/** 🌱️ A shoot (eye) of the clump: azimuth and radius of its centre on the rhizome crown, radius of the bundle of clasping petiole bases at the bed, height of the bundle. */
export type Shoot = { n: number; az: number; r: number; neck: number; h: number };
export const SHOOTS: readonly Shoot[] = [
  { n: 1, az: 42, r: 0.022, neck: 0.032, h: 0.064 },
  { n: 2, az: 137, r: 0.02, neck: 0.028, h: 0.058 },
  { n: 3, az: 226, r: 0.023, neck: 0.033, h: 0.066 },
  { n: 4, az: 318, r: 0.021, neck: 0.029, h: 0.06 },
];
export const shootCentre = (s: Shoot): V3 => mul(dirOf(s.az, 0), s.r);

/** 🍃️ One leaf as stated by hand: index, tier (a outer, b rising, c heart), shoot, azimuth, petiole length and elevation at its base and top, blade length and width, blade elevation at its base and tip, cup angle of the blade halves at the base, phase of the margin wave. */
export type Leaf = { k: number; tier: "a" | "b" | "c"; shoot: number; az: number; Lp: number; e0: number; e1: number; Lb: number; W: number; b0: number; b1: number; cup: number; wave: number };

/** 📋️ The clump. Outer leaves (the oldest, smallest) lie almost flat and reach the bed with their tips; rising leaves carry the broadest blades over them; heart leaves stand steeply on the longest petioles. Azimuths are spread by hand so that every tier closes the ring and the tiers fill each other's gaps. */
export const LEAVES: readonly Leaf[] = [
  { k: 0, tier: "a", shoot: 0, az: 8, Lp: 0.262, e0: 60, e1: 16, Lb: 0.3, W: 0.25, b0: -4, b1: -52, cup: 10, wave: 0.3 },
  { k: 1, tier: "a", shoot: 0, az: 47, Lp: 0.232, e0: 64, e1: 24, Lb: 0.292, W: 0.242, b0: 4, b1: -46, cup: 11, wave: 1.9 },
  { k: 2, tier: "a", shoot: 0, az: 80, Lp: 0.258, e0: 59, e1: 14, Lb: 0.304, W: 0.252, b0: -6, b1: -54, cup: 10, wave: 4.1 },
  { k: 3, tier: "a", shoot: 1, az: 118, Lp: 0.236, e0: 63, e1: 23, Lb: 0.294, W: 0.244, b0: 3, b1: -46, cup: 11, wave: 2.6 },
  { k: 4, tier: "a", shoot: 1, az: 152, Lp: 0.222, e0: 60, e1: 15, Lb: 0.298, W: 0.248, b0: -5, b1: -53, cup: 10, wave: 5.3 },
  { k: 5, tier: "a", shoot: 2, az: 190, Lp: 0.27, e0: 64, e1: 24, Lb: 0.296, W: 0.246, b0: 3, b1: -46, cup: 10, wave: 0.9 },
  { k: 6, tier: "a", shoot: 2, az: 224, Lp: 0.23, e0: 65, e1: 26, Lb: 0.29, W: 0.24, b0: 5, b1: -44, cup: 11, wave: 3.4 },
  { k: 7, tier: "a", shoot: 2, az: 262, Lp: 0.258, e0: 59, e1: 14, Lb: 0.304, W: 0.252, b0: -6, b1: -54, cup: 10, wave: 1.2 },
  { k: 8, tier: "a", shoot: 3, az: 298, Lp: 0.234, e0: 63, e1: 24, Lb: 0.294, W: 0.244, b0: 3, b1: -46, cup: 11, wave: 4.7 },
  { k: 9, tier: "a", shoot: 3, az: 334, Lp: 0.222, e0: 60, e1: 16, Lb: 0.298, W: 0.248, b0: -4, b1: -52, cup: 10, wave: 2.2 },
  { k: 10, tier: "b", shoot: 0, az: 28, Lp: 0.315, e0: 81, e1: 50, Lb: 0.312, W: 0.262, b0: 22, b1: -36, cup: 13, wave: 0.6 },
  { k: 11, tier: "b", shoot: 0, az: 66, Lp: 0.28, e0: 77, e1: 42, Lb: 0.306, W: 0.256, b0: 12, b1: -40, cup: 12, wave: 3.0 },
  { k: 12, tier: "b", shoot: 1, az: 103, Lp: 0.322, e0: 82, e1: 52, Lb: 0.316, W: 0.266, b0: 24, b1: -34, cup: 14, wave: 5.0 },
  { k: 13, tier: "b", shoot: 1, az: 140, Lp: 0.284, e0: 78, e1: 43, Lb: 0.308, W: 0.258, b0: 13, b1: -40, cup: 12, wave: 1.5 },
  { k: 14, tier: "b", shoot: 2, az: 182, Lp: 0.312, e0: 80, e1: 49, Lb: 0.312, W: 0.262, b0: 20, b1: -37, cup: 13, wave: 4.3 },
  { k: 15, tier: "b", shoot: 2, az: 221, Lp: 0.278, e0: 77, e1: 41, Lb: 0.304, W: 0.254, b0: 11, b1: -41, cup: 12, wave: 2.4 },
  { k: 16, tier: "b", shoot: 2, az: 258, Lp: 0.318, e0: 81, e1: 51, Lb: 0.316, W: 0.266, b0: 23, b1: -35, cup: 14, wave: 0.1 },
  { k: 17, tier: "b", shoot: 3, az: 301, Lp: 0.284, e0: 78, e1: 44, Lb: 0.308, W: 0.258, b0: 14, b1: -39, cup: 12, wave: 3.8 },
  { k: 18, tier: "b", shoot: 3, az: 342, Lp: 0.308, e0: 80, e1: 48, Lb: 0.31, W: 0.26, b0: 19, b1: -37, cup: 13, wave: 5.6 },
  { k: 19, tier: "c", shoot: 0, az: 15, Lp: 0.4, e0: 85, e1: 68, Lb: 0.292, W: 0.244, b0: 40, b1: -22, cup: 15, wave: 1.1 },
  { k: 20, tier: "c", shoot: 0, az: 78, Lp: 0.388, e0: 84, e1: 64, Lb: 0.296, W: 0.246, b0: 35, b1: -26, cup: 14, wave: 3.6 },
  { k: 21, tier: "c", shoot: 1, az: 304, Lp: 0.43, e0: 87, e1: 76, Lb: 0.282, W: 0.236, b0: 55, b1: -6, cup: 16, wave: 5.2 },
  { k: 22, tier: "c", shoot: 2, az: 198, Lp: 0.392, e0: 84, e1: 65, Lb: 0.294, W: 0.246, b0: 37, b1: -24, cup: 14, wave: 2.0 },
  { k: 23, tier: "c", shoot: 2, az: 255, Lp: 0.408, e0: 85, e1: 70, Lb: 0.288, W: 0.24, b0: 45, b1: -16, cup: 16, wave: 4.5 },
  { k: 24, tier: "c", shoot: 3, az: 118, Lp: 0.436, e0: 87, e1: 77, Lb: 0.28, W: 0.234, b0: 57, b1: -4, cup: 16, wave: 0.8 },
];

export const leafNo = (k: number): string => String(k + 1).padStart(2, "0");
export const petioleId = (k: number): string => `petiole-${leafNo(k)}`;
export const bladeId = (k: number): string => `blade-${leafNo(k)}`;
export const regionOf = (l: Leaf): "blades-outer" | "blades-inner" => (l.tier === "a" ? "blades-outer" : "blades-inner");

/** 📏️ Outer margin of a blade half as a share of half the blade width, along the midrib (0 at the petiole junction, 1 at the tip, negative behind the junction in the basal lobe): cordate base, widest at a quarter, drawn out into an acuminate tip. */
export const OUTER: readonly [number, number][] = [[-0.13, 0.3], [-0.122, 0.42], [-0.105, 0.55], [-0.08, 0.67], [-0.05, 0.76], [-0.02, 0.83], [0.02, 0.89], [0.07, 0.94], [0.13, 0.98], [0.2, 1], [0.28, 0.995], [0.36, 0.96], [0.45, 0.9], [0.54, 0.81], [0.63, 0.7], [0.71, 0.58], [0.79, 0.45], [0.86, 0.32], [0.92, 0.2], [0.96, 0.115], [0.985, 0.055], [1, 0.02]];

/** 🕳️ Inner edge of a blade half in the sinus behind the petiole junction (0 from the junction onward: the halves join at the midrib). */
export const INNER: readonly [number, number][] = [[-0.13, 0.3], [-0.122, 0.2], [-0.105, 0.13], [-0.08, 0.08], [-0.05, 0.045], [-0.02, 0.02], [0, 0]];

/** 🪜️ The stations along the midrib that every outline, loft and drawing of the finest scale shares. */
export const STATIONS: readonly number[] = OUTER.map(([s]) => s);

/** 〰️ The margin with its gentle wave (five waves along the blade, 3 % of the half width), as at 1:50. */
export const outerWavy = (l: Leaf, s: number): number => table(OUTER, s) * (1 + (s > -0.1 && s < 0.97 ? 0.03 * Math.sin(2 * Math.PI * 5 * s + l.wave) : 0));

/** 🥣️ Cup angle of a blade half above the midrib plane: deepest at the junction, flattening towards the tip and in the basal lobes, which lift behind the junction into the plane of the blade's chord (the raised auricles of a cupped hosta leaf). */
export const cupDeg = (l: Leaf, s: number): number => l.cup * table([[-0.13, 0.12], [-0.05, 0.3], [0, 1], [0.5, 0.75], [0.85, 0.55], [1, 0.45]], s);

/** 🪵️ Petiole half width and half depth along the petiole (share of its length): broad clasping base, slender channelled stalk, slightly widened where it runs into the blade. */
export const petioleA = (l: Leaf, t: number): number => (l.tier === "a" ? 0.9 : 1) * table([[0, 0.0145], [0.06, 0.011], [0.16, 0.0076], [0.4, 0.0062], [0.85, 0.0057], [1, 0.0063]], t);
export const petioleB = (l: Leaf, t: number): number => (l.tier === "a" ? 0.9 : 1) * table([[0, 0.0076], [0.08, 0.0062], [0.25, 0.0052], [1, 0.0046]], t);

/** 🌀️ A curve in the vertical plane of a leaf: centre line by arc share, tangent, upward normal. */
export type Curve = { at: (s: number) => V3; tangent: (s: number) => V3; normal: (s: number) => V3; el: (s: number) => number };

const STEPS = 300;

function curve(p0: V3, az: number, length: number, el: (s: number) => number, backEl?: number): Curve {
  const pts: V3[] = [p0];
  for (let i = 1; i <= STEPS; i++) pts.push(add(pts[i - 1]!, mul(dirOf(az, el((i - 0.5) / STEPS)), length / STEPS)));
  const clamp = (s: number) => Math.max(0, Math.min(1, s));
  const at = (s: number): V3 => {
    const x = clamp(s) * STEPS;
    const i = Math.min(STEPS - 1, Math.floor(x));
    const base = lin(pts[i]!, 1 - (x - i), pts[i + 1]!, x - i);
    return s > 1 ? add(base, mul(dirOf(az, el(1)), (s - 1) * length)) : s < 0 ? add(base, mul(dirOf(az, backEl ?? el(0)), s * length)) : base;
  };
  const elAt = (s: number) => (s < 0 && backEl !== undefined ? backEl : el(clamp(s)));
  return { at, tangent: (s) => dirOf(az, elAt(s)), normal: (s) => upOf(az, elAt(s)), el: elAt };
}

/** 🍀️ A leaf in world space: insertion, petiole curve, junction, blade midrib, tip, lateral, and the two bone frames (petiole along the chord insertion-junction, blade along the chord junction-tip, normals upward in the leaf's vertical plane). */
export type Built = { leaf: Leaf; p0: V3; j: V3; tip: V3; side: V3; pet: Curve; mid: Curve; fa: Frame; fb: Frame; La: number; Lb: number; elA: number; elB: number };

const INSET: Record<Leaf["tier"], { r: number; y: number }> = { a: { r: 0.021, y: 0.012 }, b: { r: 0.012, y: 0.022 }, c: { r: 0.004, y: 0.032 } };

export function build(l: Leaf): Built {
  const shoot = SHOOTS[l.shoot]!;
  const c = shootCentre(shoot);
  const p0: V3 = add(add(c, mul(dirOf(l.az, 0), INSET[l.tier].r)), [0, INSET[l.tier].y, 0]);
  const pet = curve(p0, l.az, l.Lp, (t) => l.e0 + (l.e1 - l.e0) * Math.pow(t, 1.4));
  const j = pet.at(1);
  const law = (s: number) => l.b0 + (l.b1 - l.b0) * Math.pow(s, 1.6);
  const tip = curve(j, l.az, l.Lb, law).at(1);
  const chord = unit(sub(tip, j));
  const mid = curve(j, l.az, l.Lb, law, (Math.asin(chord[1]) * 180) / Math.PI);
  const side = sideOf(l.az);
  const ua = unit(sub(j, p0)), ub = unit(sub(tip, j));
  const el = (u: V3) => (Math.asin(Math.max(-1, Math.min(1, u[1]))) * 180) / Math.PI;
  return { leaf: l, p0, j, tip, side, pet, mid, fa: { o: p0, u: ua, v: cross(side, ua), w: side }, fb: { o: j, u: ub, v: cross(side, ub), w: side }, La: Math.hypot(...sub(j, p0)), Lb: Math.hypot(...sub(tip, j)), elA: el(ua), elB: el(ub) };
}

/** 🪽️ The lateral axis of a blade half (sign +1 left, -1 right) at a station: the leaf's side turned up by the cup angle. */
export const wingAxis = (b: Built, s: number, sign: 1 | -1): V3 => {
  const g = (cupDeg(b.leaf, s) * Math.PI) / 180;
  return lin(b.side, sign * Math.cos(g), b.mid.normal(s), Math.sin(g));
};

/** 📍️ A point of a blade half: station s, lateral share d of half the blade width from the midrib, lift along the upper normal of the half. */
export function wingPoint(b: Built, s: number, sign: 1 | -1, d: number, lift = 0): V3 {
  const x = wingAxis(b, s, sign);
  const up = mul(cross(b.mid.tangent(s), x), -sign);
  return add(add(b.mid.at(s), mul(x, d * b.leaf.W * 0.5)), mul(up, lift));
}

export const ALL: readonly Built[] = LEAVES.map(build);
