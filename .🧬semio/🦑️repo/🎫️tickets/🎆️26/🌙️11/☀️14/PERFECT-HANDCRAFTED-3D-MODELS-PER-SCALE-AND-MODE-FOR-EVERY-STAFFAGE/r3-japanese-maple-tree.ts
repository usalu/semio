/** 🍁️ The round-3 japanese maple (*Acer palmatum*, 35-year-old three-stemmed garden specimen) stated by hand in world metres (x east = ahead, y up, z lateral; azimuth from +x towards +z): root collar with surface roots, three sinuous stems that splay in plan, the leader, the tier limbs that fan out from the stems, the twigs under the sprays, and the layered crown of flat, domed leaf sprays in three tiers, a top and two shaded cores, per scale.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-japanese-maple.md */
import { toWorld, type Frame, type V3 } from "./volumes-lib.ts";
import { D2R, add, depthIn, mul, smooth, sub, type Lobe, type Tube } from "./r3-japanese-maple-kit.ts";

export const BARK = "maple-bark", YOUNG = "maple-bark-young", SUN = "maple-leaf-sun", SHADE = "maple-leaf-shade";
export const TONE = { collar: 0.36, stem: 0.4, limb: 0.42, twig: 0.3, sun: 0.6, shade: 0.45 };

type Row4 = [u: number, v: number, w: number, r: number];

/** 🪵️ Stem stations in the planar bone frame of the stem: u along the bone, v across it in the drawing plane, w lateral (world z), r radius. */
export const STEMS: Record<string, Row4[]> = {
  "stem-c": [[-0.2, 0, 0, 0.1], [0.15, 0, -0.005, 0.088], [0.55, 0.012, -0.015, 0.083], [1.05, 0.018, -0.035, 0.077], [1.55, 0.006, -0.06, 0.07], [2.05, -0.012, -0.085, 0.06], [2.45, -0.008, -0.1, 0.052], [2.72, 0, -0.11, 0.044]],
  "stem-w": [[-0.18, 0, 0, 0.09], [0.3, 0, 0.03, 0.076], [0.8, 0.025, 0.12, 0.07], [1.3, 0.035, 0.27, 0.064], [1.8, 0.025, 0.38, 0.055], [2.0, 0.015, 0.43, 0.05], [2.42, 0, 0.52, 0.03]],
  "stem-e": [[-0.18, 0, 0, 0.086], [0.3, 0, -0.02, 0.072], [0.8, -0.02, -0.08, 0.066], [1.5, -0.03, -0.2, 0.056], [2.0, -0.02, -0.28, 0.049], [2.2, -0.012, -0.31, 0.046], [2.62, 0, -0.4, 0.029]],
  "stem-c-top": [[-0.04, 0, 0, 0.045], [0.35, 0.015, -0.015, 0.039], [0.7, 0.005, -0.005, 0.03], [1.0, -0.01, 0.02, 0.02], [1.2, -0.015, 0.03, 0.012]],
};
export const STEMS_100: Record<string, Row4[]> = {
  "stem-c": [[-0.2, 0, 0, 0.1], [0.15, 0, -0.005, 0.088], [0.8, 0.016, -0.025, 0.08], [1.55, 0.006, -0.06, 0.07], [2.25, -0.012, -0.093, 0.056], [2.72, 0, -0.11, 0.044]],
  "stem-w": [[-0.18, 0, 0, 0.09], [0.3, 0, 0.03, 0.076], [1.05, 0.032, 0.2, 0.067], [1.8, 0.025, 0.38, 0.055], [2.42, 0, 0.52, 0.03]],
  "stem-e": [[-0.18, 0, 0, 0.086], [0.3, 0, -0.02, 0.072], [1.15, -0.027, -0.14, 0.062], [2.0, -0.02, -0.28, 0.052], [2.62, 0, -0.4, 0.029]],
  "stem-c-top": [[-0.04, 0, 0, 0.048], [0.5, 0.01, -0.01, 0.035], [1.2, -0.015, 0.03, 0.015]],
};
export const STEMS_200: Record<string, Row4[]> = {
  "stem-c": [[-0.2, 0, 0, 0.098], [0.2, 0, -0.006, 0.087], [1.3, 0.012, -0.048, 0.074], [2.72, 0, -0.11, 0.044]],
  "stem-w": [[-0.18, 0, 0, 0.088], [0.3, 0, 0.03, 0.076], [1.3, 0.035, 0.27, 0.064], [2.42, 0, 0.52, 0.036]],
  "stem-e": [[-0.18, 0, 0, 0.084], [0.3, 0, -0.02, 0.072], [1.4, -0.03, -0.18, 0.06], [2.62, 0, -0.4, 0.034]],
  "stem-c-top": [[-0.04, 0, 0, 0.046], [1.2, -0.015, 0.03, 0.02]],
};
export const STEMS_500: Record<string, Row4[]> = {
  "stem-c": [[-0.2, 0, 0, 0.098], [0.3, 0, -0.01, 0.086], [2.72, 0, -0.11, 0.05]],
  "stem-w": [[-0.18, 0, 0, 0.088], [0.3, 0, 0.03, 0.075], [2.42, 0, 0.52, 0.04]],
  "stem-e": [[-0.18, 0, 0, 0.084], [0.3, 0, -0.02, 0.071], [2.62, 0, -0.4, 0.04]],
  "stem-c-top": [[-0.04, 0, 0, 0.045], [1.0, 0, 0.02, 0.04]],
};

/** 📍️ The lateral offset (world z) of a stem's centre line at the bone coordinate u, from its stations. */
export function stemW(rows: Row4[], u: number): number {
  for (let i = 0; i + 1 < rows.length; i++) {
    const [a, b] = [rows[i]!, rows[i + 1]!];
    if (u <= b[0] || i + 2 === rows.length) return a[2] + ((b[2] - a[2]) * (u - a[0])) / (b[0] - a[0]);
  }
  return rows[0]![2];
}
/** 📐️ The across-bone offset (drawing plane) of a stem's centre line at u. */
export function stemV(rows: Row4[], u: number): number {
  for (let i = 0; i + 1 < rows.length; i++) {
    const [a, b] = [rows[i]!, rows[i + 1]!];
    if (u <= b[0] || i + 2 === rows.length) return a[1] + ((b[1] - a[1]) * (u - a[0])) / (b[0] - a[0]);
  }
  return rows[0]![1];
}

/** 🦴️ Lateral bone offsets (rig `z`, relative to the parent) that carry every joint onto the splayed stems. */
export function boneOffsets(): Record<string, number> {
  const c = STEMS["stem-c"]!, w = STEMS["stem-w"]!, e = STEMS["stem-e"]!;
  const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
  return {
    "stem-c-top": r4(stemW(c, 2.7)), t3w: r4(stemW(c, 2.5)), t3e: r4(stemW(c, 2.6)), "fol-core-low": r4(stemW(c, 1.699)), "fol-core-up": r4(stemW(c, 2.7)),
    t1w: r4(stemW(w, 1.3)), t2w: r4(stemW(w, 2.0)), t1e: r4(stemW(e, 1.5)), t2e: r4(stemW(e, 2.2)),
  };
}

const up = (f: Frame): V3 => (f.v[1] > 0 ? f.v : mul(f.v, -1));
const inFrame = (f: Frame, [u, v, w, r]: Row4): { p: V3; r: number } => ({ p: add(add(add(f.o, mul(f.u, u)), mul(up(f), v)), [0, 0, w]), r });
const polar = (r: number, az: number, y: number): V3 => [r * Math.cos(az * D2R), y, r * Math.sin(az * D2R)];

/** 🎯️ The world point of a stem's centre line (1:50 stations) at the bone coordinate u. */
export function stemPoint(F: Record<string, Frame>, bone: string, u: number): V3 {
  const rows = STEMS[bone]!;
  return inFrame(F[bone]!, [u, stemV(rows, u), stemW(rows, u), 0]).p;
}

/** 🌳️ A stem, leader or limb as a tube along stations in its bone frame, smoothed by `n` sections per span. */
export function framed(id: string, bone: string, material: string, tone: number, group: string, F: Record<string, Frame>, rows: Row4[], n: number, frameOf = bone): Tube {
  return { kind: "tube", id, bone, material, tone, group, points: smooth(rows.map((r) => inFrame(F[frameOf]!, r)), n) };
}

/** 🌱️ The root collar: a flared foot that holds the three stem bases (stations: height, radius). */
const COLLAR: Record<number, [number, number][]> = {
  50: [[0, 0.3], [0.04, 0.255], [0.1, 0.208], [0.17, 0.18], [0.26, 0.16]],
  100: [[0, 0.33], [0.05, 0.255], [0.12, 0.196], [0.22, 0.16]],
  200: [[0, 0.34], [0.08, 0.23], [0.22, 0.165]],
  500: [[0, 0.34], [0.22, 0.17]],
};
/** 🪨️ Four shallow surface roots of the fibrous maple root plate (azimuth, reach factor). */
const ROOTS: [number, number][] = [[28, 1], [118, 0.86], [205, 1.05], [292, 0.92]];

/** 🌿️ An arm of a tier: from the joint of its bone through stations given around the tree axis (radius, azimuth, height, tube radius). */
type Arm = { id: string; bone: string; via: [r: number, az: number, y: number, rad: number][]; r0: number };
const ARMS: Arm[] = [
  { id: "t1w-a135", bone: "t1w", r0: 0.014, via: [[0.9, 141, 1.55, 0.011], [1.6, 137, 1.64, 0.0085], [2.22, 134, 1.66, 0.006]] },
  { id: "t1w-a225", bone: "t1w", r0: 0.014, via: [[0.9, 214, 1.52, 0.011], [1.6, 221, 1.63, 0.0085], [2.22, 226, 1.65, 0.006]] },
  { id: "t1w-a270", bone: "t1w", r0: 0.014, via: [[0.72, 238, 1.5, 0.011], [1.48, 261, 1.62, 0.0085], [2.15, 271, 1.64, 0.006]] },
  { id: "t1e-a45", bone: "t1e", r0: 0.014, via: [[0.98, 35, 1.65, 0.011], [1.6, 43, 1.71, 0.0085], [2.22, 46, 1.73, 0.006]] },
  { id: "t1e-a315", bone: "t1e", r0: 0.014, via: [[0.98, 331, 1.64, 0.011], [1.6, 318, 1.7, 0.0085], [2.22, 314, 1.72, 0.006]] },
  { id: "t1e-a90", bone: "t1e", r0: 0.014, via: [[0.8, 58, 1.63, 0.011], [1.5, 83, 1.7, 0.0085], [2.15, 91, 1.72, 0.006]] },
  { id: "t2w-a120", bone: "t2w", r0: 0.014, via: [[1.3, 128, 2.27, 0.011], [1.72, 121, 2.33, 0.0085], [2.02, 119, 2.35, 0.006]] },
  { id: "t2w-a240", bone: "t2w", r0: 0.014, via: [[1.05, 198, 2.2, 0.011], [1.55, 231, 2.3, 0.0085], [2.02, 241, 2.34, 0.006]] },
  { id: "t2e-a60", bone: "t2e", r0: 0.014, via: [[1.12, 22, 2.33, 0.011], [1.6, 53, 2.41, 0.0085], [2.02, 61, 2.43, 0.006]] },
  { id: "t2e-a300", bone: "t2e", r0: 0.014, via: [[1.38, 318, 2.32, 0.011], [1.72, 304, 2.4, 0.0085], [2.02, 299, 2.42, 0.006]] },
  { id: "t3w-a270", bone: "t3w", r0: 0.014, via: [[0.5, 258, 2.92, 0.011], [1.0, 267, 3.05, 0.0085], [1.38, 271, 3.1, 0.006]] },
  { id: "t3e-a90", bone: "t3e", r0: 0.014, via: [[0.5, 72, 2.98, 0.011], [1.0, 86, 3.1, 0.0085], [1.38, 90, 3.14, 0.006]] },
];
/** 🌿️ The main limb of each tier bone along the rig (u, bow up, lateral wiggle, radius) per scale. */
const MAIN: Record<number, Record<string, Row4[]>> = {
  50: {
    t1w: [[-0.05, 0, 0, 0.036], [0.1, 0, 0, 0.031], [0.5, 0.03, 0.03, 0.027], [0.95, 0.04, -0.02, 0.022], [1.4, 0.02, 0.02, 0.016], [1.85, -0.03, 0, 0.011]],
    t1e: [[-0.05, 0, 0, 0.036], [0.1, 0, 0, 0.031], [0.5, 0.03, -0.03, 0.027], [0.95, 0.035, 0.02, 0.022], [1.4, 0.015, -0.025, 0.016], [1.85, -0.03, 0, 0.011]],
    t2w: [[-0.05, 0, 0, 0.035], [0.1, 0, 0, 0.03], [0.45, 0.03, 0.025, 0.025], [0.9, 0.03, -0.02, 0.019], [1.5, -0.01, 0.01, 0.012]],
    t2e: [[-0.05, 0, 0, 0.035], [0.1, 0, 0, 0.03], [0.45, 0.025, -0.02, 0.025], [0.9, 0.03, 0.02, 0.019], [1.45, -0.01, 0, 0.012]],
    t3w: [[-0.05, 0, 0, 0.034], [0.1, 0, 0, 0.029], [0.5, 0.025, 0.02, 0.022], [0.9, 0.015, -0.015, 0.016], [1.2, -0.01, 0, 0.011]],
    t3e: [[-0.05, 0, 0, 0.034], [0.1, 0, 0, 0.029], [0.5, 0.025, -0.02, 0.022], [0.9, 0.015, 0.015, 0.016], [1.25, -0.01, 0, 0.011]],
  },
  100: {
    t1w: [[-0.05, 0, 0, 0.035], [0.1, 0, 0, 0.031], [0.7, 0.035, 0.01, 0.025], [1.4, 0.02, 0.02, 0.016], [1.85, -0.03, 0, 0.011]],
    t1e: [[-0.05, 0, 0, 0.035], [0.1, 0, 0, 0.031], [0.7, 0.032, -0.01, 0.025], [1.4, 0.015, -0.025, 0.016], [1.85, -0.03, 0, 0.011]],
    t2w: [[-0.05, 0, 0, 0.034], [0.1, 0, 0, 0.03], [0.7, 0.03, 0.005, 0.022], [1.5, -0.01, 0.01, 0.012]],
    t2e: [[-0.05, 0, 0, 0.034], [0.1, 0, 0, 0.03], [0.7, 0.028, 0, 0.022], [1.45, -0.01, 0, 0.012]],
    t3w: [[-0.05, 0, 0, 0.033], [0.1, 0, 0, 0.029], [0.7, 0.02, 0, 0.019], [1.2, -0.01, 0, 0.011]],
    t3e: [[-0.05, 0, 0, 0.033], [0.1, 0, 0, 0.029], [0.7, 0.02, 0, 0.019], [1.25, -0.01, 0, 0.011]],
  },
  200: {
    t1w: [[-0.05, 0, 0, 0.034], [0.8, 0.035, 0, 0.024], [1.85, -0.03, 0, 0.016]],
    t1e: [[-0.05, 0, 0, 0.034], [0.8, 0.032, 0, 0.024], [1.85, -0.03, 0, 0.016]],
    t2w: [[-0.05, 0, 0, 0.033], [0.75, 0.03, 0, 0.022], [1.5, -0.01, 0, 0.016]],
    t2e: [[-0.05, 0, 0, 0.033], [0.75, 0.028, 0, 0.022], [1.45, -0.01, 0, 0.016]],
    t3w: [[-0.05, 0, 0, 0.032], [0.6, 0.02, 0, 0.022], [1.2, -0.01, 0, 0.016]],
    t3e: [[-0.05, 0, 0, 0.032], [0.6, 0.02, 0, 0.022], [1.25, -0.01, 0, 0.016]],
  },
  500: {
    t1w: [[-0.05, 0, 0, 0.03], [1.85, -0.03, 0, 0.03]], t1e: [[-0.05, 0, 0, 0.03], [1.85, -0.03, 0, 0.03]],
    t2w: [[-0.05, 0, 0, 0.03], [1.5, -0.01, 0, 0.03]], t2e: [[-0.05, 0, 0, 0.03], [1.45, -0.01, 0, 0.03]],
    t3w: [[-0.05, 0, 0, 0.03], [1.2, -0.01, 0, 0.03]], t3e: [[-0.05, 0, 0, 0.03], [1.25, -0.01, 0, 0.03]],
  },
};
/** 🍂️ Twigs under the sprays (1:50 only): arm, fraction along it, turn in plan (degrees), length, rise. */
const TWIGS: [arm: string, f: number, dAz: number, L: number, rise: number][] = [
  ["t1w", 0.62, 26, 0.5, 0.05], ["t1w", 0.8, -24, 0.42, 0.04], ["t1e", 0.62, -26, 0.5, 0.05], ["t1e", 0.8, 24, 0.42, 0.04],
  ["t2w", 0.6, -27, 0.46, 0.05], ["t2w", 0.8, 25, 0.38, 0.04], ["t2e", 0.6, 27, 0.46, 0.05], ["t2e", 0.8, -25, 0.38, 0.04],
  ["t3w", 0.62, 30, 0.38, 0.05], ["t3e", 0.62, -30, 0.38, 0.05],
  ["t1w-a135", 0.6, -28, 0.45, 0.04], ["t1w-a135", 0.82, 22, 0.36, 0.03], ["t1w-a225", 0.6, 28, 0.45, 0.04], ["t1w-a225", 0.82, -22, 0.36, 0.03],
  ["t1w-a270", 0.62, -26, 0.44, 0.04], ["t1e-a45", 0.6, 28, 0.45, 0.04], ["t1e-a45", 0.82, -22, 0.36, 0.03], ["t1e-a315", 0.6, -28, 0.45, 0.04],
  ["t1e-a315", 0.82, 22, 0.36, 0.03], ["t1e-a90", 0.62, 26, 0.44, 0.04], ["t2w-a120", 0.62, -26, 0.4, 0.04], ["t2w-a240", 0.62, 26, 0.4, 0.04],
  ["t2e-a60", 0.62, 26, 0.4, 0.04], ["t2e-a300", 0.62, -26, 0.4, 0.04], ["t3w-a270", 0.6, 30, 0.34, 0.04], ["t3e-a90", 0.6, -30, 0.34, 0.04],
];
/** 🌾️ Twigs of the leader into the top sprays (1:50 only): height on the leader, azimuth, length, rise. */
const TOP_TWIGS: [u: number, az: number, L: number, rise: number][] = [[0.45, 20, 0.6, 0.3], [0.5, 140, 0.58, 0.28], [0.55, 260, 0.58, 0.27], [0.7, 80, 0.48, 0.2], [0.75, 200, 0.48, 0.19], [0.8, 320, 0.46, 0.18]];

/** 🍃️ A leaf spray: a flat, domed ellipsoid with its radial axis drooping by `droop` degrees outwards; `reach` is the radius of its outer edge about the tier centre. */
function spray(id: string, bone: string, tier: string, centre: [number, number], az: number, reach: number, y: number, a: number, b: number, c: number, droop: number, shade: boolean): Lobe & { tier: string; az: number } {
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const [ct, st] = [Math.cos(droop * D2R), Math.sin(droop * D2R)];
  const rad1: V3 = [rad[0] * ct, -st, rad[2] * ct];
  const up1: V3 = [rad[0] * st, ct, rad[2] * st];
  const tan: V3 = [-rad[2], 0, rad[0]];
  const rc = reach - Math.hypot(a * ct, b * st);
  return { kind: "lobe", id, bone, material: shade ? SHADE : SUN, tone: shade ? TONE.shade : TONE.sun, group: shade ? "shade" : "sun", pad: !shade, c: [centre[0] + rc * rad[0], y, centre[1] + rc * rad[2]], axes: [rad1, up1, tan], radii: [a, b, c], tier, az };
}
export type Spray = ReturnType<typeof spray>;

type Tier = { key: string; e: string; w: string; eastOf: (az: number) => boolean; centre: [number, number]; rx: number; rz: number; tilt: number; phase: number };
const TIERS: Record<string, Tier> = {
  t1: { key: "t1", e: "fol-t1e", w: "fol-t1w", eastOf: (az) => az >= 292.5 || az < 112.5, centre: [0, 0], rx: 2.647, rz: 2.594, tilt: 0.07, phase: 160 },
  t2: { key: "t2", e: "fol-t2e", w: "fol-t2w", eastOf: (az) => az >= 270 || az < 90, centre: [0, 0], rx: 2.5, rz: 2.32, tilt: 0.06, phase: 290 },
  t3: { key: "t3", e: "fol-t3e", w: "fol-t3w", eastOf: (az) => az >= 315 || az < 135, centre: [0.05, -0.08], rx: 1.78, rz: 1.7, tilt: 0.05, phase: 40 },
};
/** 📐️ How far a tier's wood is lowered at a world point, so that limbs follow the dip of their leaf layer (full dip at the tier rim). */
export function tierDip(key: string, p: V3): number {
  const t = TIERS[key];
  if (!t) return 0;
  const [x, z] = [p[0] - t.centre[0], p[2] - t.centre[1]];
  const az = Math.atan2(z, x) / D2R;
  return t.tilt * Math.min(1, Math.hypot(x, z) / envelope(t, az)) * Math.cos((az - t.phase) * D2R);
}
const dipped = (key: string, q: { p: V3; r: number }): { p: V3; r: number } => ({ p: add(q.p, [0, tierDip(key, q.p), 0]), r: q.r });
const envelope = (t: Tier, az: number): number => 1 / Math.hypot(Math.cos(az * D2R) / t.rx, Math.sin(az * D2R) / t.rz);

/** 🍃️ Row of a tier spray: suffix, azimuth, reach factor of the tier envelope (or fixed reach when above 10, minus 10), height, semi-axes radial, vertical, tangential, droop. A tier is not level: it dips by `tilt` towards the azimuth `phase` + 180 (sprays of the outer ring fully, inner ones by half). */
type SprayRow = [s: string, az: number, k: number, y: number, a: number, b: number, c: number, droop: number];
const tierSprays = (t: Tier, rows: SprayRow[]): Spray[] => rows.map(([s, az, k, y, a, b, c, droop]) => spray(`${t.key}-${s}`, t.eastOf(((az % 360) + 360) % 360) ? t.e : t.w, t.key, t.centre, az, k > 10 ? k - 10 : envelope(t, az) * k, y + t.tilt * (k > 10 ? 0.5 : 1) * Math.cos((az - t.phase) * D2R), a, b, c, droop, false));
/** 💠️ A ring of sprays; `sizes` scales the horizontal semi-axes per spray (the vertical by its square root), so neighbouring sprays differ as real leaf layers do. */
const ring = (s: string, azs: number[], ks: number[], ys: number[], a: number, b: number, c: number, droop: number, sizes: number[] = [1]): SprayRow[] =>
  azs.map((az, i) => {
    const f = sizes[i % sizes.length]!;
    return [`${s}${i + 1}`, az, ks[i % ks.length]!, ys[i % ys.length]!, a * f, b * Math.sqrt(f), c * f, droop];
  });
const fixed = (r: number): number => 10 + r;

/** 🍁️ The leaf sprays of one scale. */
export function sprays(scale: number): Spray[] {
  const { t1, t2, t3 } = TIERS as Record<string, Tier>;
  const out: Spray[] = [];
  const step = (n: number, from = 0): number[] => Array.from({ length: n }, (_, i) => from + (360 * i) / n);
  if (scale === 50) {
    out.push(...tierSprays(t1!, [
      ...ring("o", step(12), [1, 0.96, 0.975, 1, 0.97, 0.955, 1, 0.962, 0.972, 1, 0.958, 0.97], [1.7, 1.74, 1.67, 1.71, 1.69, 1.75, 1.7, 1.66, 1.72, 1.7, 1.75, 1.68], 0.6, 0.21, 0.62, 8, [1, 0.88, 1.08, 1, 0.92, 1.1, 1, 0.9, 1.06, 1, 0.86, 1.04]),
      ...ring("f", step(12, 15), [0.955, 0.94, 0.95, 0.945, 0.935, 0.95], [1.6, 1.63, 1.57, 1.61, 1.55], 0.3, 0.12, 0.32, 16, [1, 1.25, 0.85, 1.15, 0.9]),
      ...ring("i", step(8, 22.5), [fixed(1.7), fixed(1.66), fixed(1.72), fixed(1.68)], [1.82, 1.86, 1.8, 1.84], 0.5, 0.2, 0.55, 3, [1, 0.9, 1.12, 0.95]),
    ]));
    out.push(...tierSprays(t2!, [
      ...ring("o", step(10), [1, 0.965, 0.975, 0.96, 0.97], [2.38, 2.42, 2.35, 2.4, 2.43], 0.56, 0.2, 0.6, 8, [1, 0.9, 1.1, 0.94, 1.06]),
      ...ring("f", step(10, 18), [0.95, 0.94, 0.955, 0.945], [2.29, 2.31, 2.26], 0.28, 0.11, 0.3, 16, [1.2, 0.9, 1.1, 0.85]),
      ...ring("i", step(6, 30), [fixed(1.51), fixed(1.47), fixed(1.53)], [2.5, 2.54, 2.48], 0.46, 0.19, 0.5, 3, [1, 1.1, 0.9]),
    ]));
    out.push(...tierSprays(t3!, [
      ...ring("o", step(8), [1, 0.97, 0.98, 0.965], [3.14, 3.18, 3.12, 3.16], 0.5, 0.19, 0.52, 8, [1, 0.9, 1.1, 0.93]),
      ...ring("f", step(8, 22.5), [0.95, 0.94], [3.06, 3.08, 3.04], 0.26, 0.1, 0.28, 16, [1.15, 0.9, 1.05]),
      ...ring("i", step(4, 45), [fixed(1.12), fixed(1.1)], [3.24, 3.26], 0.42, 0.19, 0.45, 3),
    ]));
    out.push(...topSprays([[0, 1.05, 3.9], [60, 1.02, 3.92], [120, 1.05, 3.89], [180, 1.03, 3.91], [240, 1.05, 3.9], [300, 1.02, 3.92]], 0.44, 0.25, 0.48, [[30, 0.98, 3.79], [90, 0.97, 3.8], [150, 0.98, 3.78], [210, 0.97, 3.8], [270, 0.98, 3.79], [330, 0.97, 3.8]], 0.26, 0.13, 0.28, 0.55, 0.31));
    out.push(...cores([0, 60, 120, 180, 240, 300], 0.95, 2.08, 0.4, 0.17, 0.45, [45, 135, 225, 315], 0.81, 2.86, 0.36, 0.16, 0.4));
  } else if (scale === 100) {
    out.push(...tierSprays(t1!, [
      ...ring("o", step(12), [1, 0.96, 0.975, 1, 0.97, 0.955, 1, 0.962, 0.972, 1, 0.958, 0.97], [1.69, 1.73, 1.66, 1.7, 1.68, 1.74, 1.69, 1.65, 1.71, 1.69, 1.74, 1.67], 0.64, 0.23, 0.82, 10, [1, 0.88, 1.08, 1, 0.92, 1.1, 1, 0.9, 1.06, 1, 0.86, 1.04]),
      ...ring("i", step(6, 30), [fixed(1.72), fixed(1.68), fixed(1.72)], [1.82, 1.84, 1.82], 0.58, 0.21, 0.66, 3),
    ]));
    out.push(...tierSprays(t2!, [
      ...ring("o", step(10), [1, 0.965, 0.975, 0.96, 0.97], [2.37, 2.41, 2.34, 2.39, 2.42], 0.59, 0.21, 0.76, 9, [1, 0.9, 1.1, 0.94, 1.06]),
      ...ring("i", step(4, 45), [fixed(1.52), fixed(1.5)], [2.5, 2.5], 0.55, 0.2, 0.62, 3),
    ]));
    out.push(...tierSprays(t3!, [
      ...ring("o", step(8), [1, 0.97, 0.98, 0.965], [3.13, 3.17, 3.11, 3.15], 0.53, 0.2, 0.64, 9, [1, 0.9, 1.1, 0.93]),
      ...ring("i", step(2, 90), [fixed(1.12)], [3.25], 0.5, 0.2, 0.62, 3),
    ]));
    out.push(...topSprays([[0, 1.05, 3.88], [72, 1.03, 3.9], [144, 1.05, 3.88], [216, 1.03, 3.9], [288, 1.05, 3.88]], 0.48, 0.27, 0.56, [], 0, 0, 0, 0.58, 0.32));
    out.push(...cores([30, 150, 270], 0.97, 2.08, 0.46, 0.18, 0.62, [90, 270], 0.83, 2.86, 0.42, 0.17, 0.52));
  } else if (scale === 200) {
    out.push(...tierSprays(t1!, [
      ...ring("o", step(8), [1, 0.975, 1, 0.975], [1.69, 1.7], 0.78, 0.26, 1.05, 9),
      ...ring("i", step(4, 45), [fixed(1.74)], [1.82], 0.7, 0.22, 0.8, 3),
    ]));
    out.push(...tierSprays(t2!, [
      ...ring("o", step(6), [1, 0.975, 0.975], [2.37, 2.38], 0.72, 0.24, 1.1, 9),
      ...ring("i", step(3, 60), [fixed(1.55)], [2.5], 0.62, 0.21, 0.85, 3),
    ]));
    out.push(...tierSprays(t3!, [...ring("o", step(4), [1, 0.985], [3.14], 0.64, 0.22, 0.95, 8), ...ring("i", step(2, 90), [fixed(1.1)], [3.25], 0.5, 0.2, 0.7, 3)]));
    out.push(...topSprays([[0, 1.05, 3.87], [120, 1.05, 3.87], [240, 1.05, 3.87]], 0.55, 0.28, 0.8, [], 0, 0, 0, 0.6, 0.33));
    out.push(...cores([90, 270], 1.0, 2.08, 0.5, 0.19, 0.8, [0], 0.0, 2.86, 0.62, 0.18, 0.62));
  } else {
    const crown = (s: Spray): Spray => ({ ...s, bone: "stem-c" });
    out.push(...tierSprays(t1!, [...ring("x", step(4), [1], [1.7, 1.71], 1.1, 0.29, 1.25, 7), ...ring("o", step(4, 45), [0.97], [1.72, 1.71], 1.0, 0.28, 1.0, 7)]).map(crown));
    out.push(...tierSprays(t2!, [...ring("x", step(4), [1], [2.38, 2.39], 1.0, 0.27, 1.15, 7), ...ring("o", step(4, 45), [0.97], [2.39, 2.4], 0.9, 0.26, 0.95, 7)]).map(crown));
    out.push(...tierSprays(t3!, ring("o", step(4, 45), [1], [3.17], 0.85, 0.25, 1.0, 6)).map(crown));
    out.push(...topSprays([[0, 1.05, 3.87], [120, 1.05, 3.87], [240, 1.05, 3.87]], 0.6, 0.3, 0.85, [], 0, 0, 0, 0.62, 0.34).map(crown));
    out.push(...cores([0], 0.0, 2.08, 0.95, 0.24, 0.95, [0], 0.0, 2.86, 0.75, 0.22, 0.75).map(crown));
  }
  return out;
}

const TOP_CENTRE: [number, number] = [0.07, -0.15];
/** 🔝️ The domed top: a ring of sprays (azimuth, reach, height), a fringe below it and a cap whose crown is the tree top (4.429 m). */
function topSprays(ringRows: [number, number, number][], a: number, b: number, c: number, fringe: [number, number, number][], fa: number, fb: number, fc: number, capR: number, capB: number): Spray[] {
  const out = ringRows.map(([az, reach, y], i) => ({ ...spray(`top-r${i + 1}`, "fol-top", "top", TOP_CENTRE, az, reach, y, a, b, c, 10, false) }));
  out.push(...fringe.map(([az, reach, y], i) => spray(`top-f${i + 1}`, "fol-top", "top", TOP_CENTRE, az, reach, y, fa, fb, fc, 16, false)));
  out.push(spray("top-cap", "fol-top", "top", TOP_CENTRE, 0, capR, 4.429 - capB, capR, capB, capR, 0, false));
  return out;
}
/** 🌑️ The shaded inner crown: sprays round the stems between tier 1 and tier 2 (core-low) and between tier 2 and tier 3 (core-up). */
function cores(lowAz: number[], lowReach: number, lowY: number, la: number, lb: number, lc: number, upAz: number[], upReach: number, upY: number, ua: number, ub: number, uc: number): Spray[] {
  const out: Spray[] = [];
  lowAz.forEach((az, i) => out.push(spray(`core-low-${i + 1}`, "fol-core-low", "core-low", [0, 0], az, lowReach > 0 ? lowReach : la, lowY, la, lb, lc, 0, true)));
  upAz.forEach((az, i) => out.push(spray(`core-up-${i + 1}`, "fol-core-up", "core-up", [0.05, -0.1], az, upReach > 0 ? upReach : ua, upY, ua, ub, uc, 0, true)));
  return out;
}

/** 🪺️ Seats the tip of a limb or twig in the leaf spray it carries: when the tip lies outside or in the skin of every spray (deeper than `limit`), it is drawn towards the centre of the nearest spray, and the last part of the tube follows with a linear ramp. */
function seat(t: Tube, lobes: Lobe[], limit: number, ramp = 0.45): Tube {
  const tip = t.points[t.points.length - 1]!.p;
  const l = lobes.reduce((best, x) => (depthIn(x, tip) < depthIn(best, tip) ? x : best));
  if (depthIn(l, tip) <= limit) return t;
  const target: V3 = add(l.c, mul(l.axes[1], -0.25 * l.radii[1]));
  let f = 0;
  for (; f < 1 && depthIn(l, add(tip, mul(sub(target, tip), f))) > limit; f += 0.02);
  const delta = mul(sub(target, tip), f);
  if (Math.hypot(...delta) > 0.25) console.log(`[DEBUG] ${t.id}: tip would move ${Math.hypot(...delta).toFixed(2)} m into ${l.id}; restate its stations`);
  const n = t.points.length, from = Math.floor(n * (1 - ramp));
  return { ...t, points: t.points.map((q, i) => (i < from ? q : { p: add(q.p, mul(delta, (i - from + 1) / (n - from))), r: q.r })) };
}

/** 🌲️ The woody parts of one scale: collar, roots, stems, leader, limbs and twigs. */
export function wood(scale: number, F: Record<string, Frame>, lobes: Lobe[]): { wood: Tube[]; twigs: Tube[] } {
  const n = scale === 50 ? 3 : scale === 100 ? 2 : 1;
  const out: Tube[] = [];
  const collar = COLLAR[scale]!;
  out.push({ kind: "tube", id: "root-collar", bone: "root-plate", material: BARK, tone: TONE.collar, group: "collar", points: collar.map(([y, rx]) => ({ p: [0.004, y, 0] as V3, r: rx })) });
  if (scale === 50) ROOTS.forEach(([az, k], i) => {
    const d = (r: number, y: number): V3 => [r * k * Math.cos(az * D2R), y, r * k * Math.sin(az * D2R)];
    out.push({ kind: "tube", id: `root-${i + 1}`, bone: "root-plate", material: BARK, tone: TONE.collar, group: "collar", points: smooth([{ p: d(0.1, 0.15), r: 0.06 }, { p: d(0.2, 0.08), r: 0.05 }, { p: d(0.3, 0.036), r: 0.034 }, { p: d(0.4, 0.016), r: 0.015 }], 2) });
  });
  const stems = scale === 50 ? STEMS : scale === 100 ? STEMS_100 : scale === 200 ? STEMS_200 : STEMS_500;
  for (const [id, rows] of Object.entries(stems)) out.push(framed(id === "stem-c-top" ? "wood-stem-c-top" : `wood-${id}`, id, BARK, TONE.stem, "stem", F, rows, n));
  const main = MAIN[scale]!;
  for (const [id, rows] of Object.entries(main)) {
    const t = framed(`limb-${id}`, id, YOUNG, TONE.limb, "limb", F, rows, n);
    out.push(seat({ ...t, points: t.points.map((q) => dipped(id.slice(0, 2), q)) }, lobes, 0.7));
  }
  if (scale <= 100) for (const arm of ARMS) {
    const o = F[arm.bone]!.o;
    const via = scale === 50 ? arm.via : [arm.via[0]!, arm.via[2]!];
    const pts = [{ p: add(o, [0, 0.005, 0]), r: arm.r0 }, ...via.map(([r, az, y, rad]) => ({ p: polar(r, az, y), r: scale === 50 ? rad : Math.max(rad, 0.0085) }))];
    out.push(seat({ kind: "tube", id: `limb-${arm.id}`, bone: arm.bone, material: YOUNG, tone: TONE.limb, group: "limb", points: smooth(pts.map((q) => dipped(arm.bone.slice(0, 2), q)), scale === 50 ? 4 : 3) }, lobes, 0.7));
  }
  const twigs: Tube[] = [];
  if (scale === 50) {
    const byId = (id: string): Tube => out.find((t) => t.id === `limb-${id}`)!;
    TWIGS.forEach(([arm, f, dAz, L, rise], i) => {
      const t = byId(arm);
      const k = Math.round(f * (t.points.length - 1));
      const [p, q] = [t.points[k]!.p, t.points[Math.min(t.points.length - 1, k + 1)]!.p];
      const az = Math.atan2(q[2] - p[2], q[0] - p[0]) / D2R + dAz;
      const end: V3 = add(p, [L * Math.cos(az * D2R), rise, L * Math.sin(az * D2R)]);
      const mid: V3 = add(add(p, mul(add(end, mul(p, -1)), 0.5)), [0, rise * 0.4, 0]);
      twigs.push(seat({ kind: "tube", id: `twig-${i + 1}`, bone: t.bone, material: YOUNG, tone: TONE.twig, group: "twig", flags: { slender: true, texture: true }, points: [{ p, r: 0.0075 }, { p: mid, r: 0.006 }, { p: end, r: 0.0045 }] }, lobes, 0.75, 0.7));
    });
    const top = F["stem-c-top"]!;
    TOP_TWIGS.forEach(([u, az, L, rise], i) => {
      const p = toWorld(top, [u, 0, 0]);
      const end: V3 = add(p, [L * Math.cos(az * D2R), rise, L * Math.sin(az * D2R)]);
      twigs.push(seat({ kind: "tube", id: `twig-top-${i + 1}`, bone: "stem-c-top", material: YOUNG, tone: TONE.twig, group: "twig", flags: { slender: true, texture: true }, points: [{ p, r: 0.0075 }, { p: add(add(p, mul(add(end, mul(p, -1)), 0.5)), [0, rise * 0.3, 0]), r: 0.006 }, { p: end, r: 0.0045 }] }, lobes, 0.75, 0.7));
    });
  }
  return { wood: out, twigs };
}
