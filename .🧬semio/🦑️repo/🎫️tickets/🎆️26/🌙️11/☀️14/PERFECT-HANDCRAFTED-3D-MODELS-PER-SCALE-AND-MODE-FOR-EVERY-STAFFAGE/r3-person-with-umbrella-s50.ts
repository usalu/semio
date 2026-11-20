/** ☔ The 1:50 model of `humans/person-with-umbrella`, stated section by section: an adult of 1.70 m (1.72 m in shoes) and 70 kg with the hair in a low bun, in a knee-length single-breasted raincoat over shirt and trousers and leather derby shoes, the right fist round the crook handle of an open stick umbrella with eight ribs, eight stretchers and a runner, the left arm hanging.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `umbrella-up-still`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-person-with-umbrella-kit.ts
 * @see ./r3-humans-person-with-umbrella.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, FRAMES, H, L, X, aboutU, aboutW, box, capsule, dense, dome, ellipsoid, extrude, flip, freeLoft, inset, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-person-with-umbrella-kit.ts";
import { AZIMUTH, RIB, STRETCHER } from "./r3-person-with-umbrella-roots.ts";

export const TONE = { coat: 0.56, coatFar: 0.5, trousers: 0.26, trousersFar: 0.21, shoe: 0.13, shoeFar: 0.1, skin: 0.84, hand: 0.82, handFar: 0.76, hair: 0.24, shirt: 0.92, canopy: 0.3, metal: 0.2, wood: 0.4, button: 0.3, bone: 0.9 } as const;
export const far = (t: number): number => (t === TONE.coat ? TONE.coatFar : t === TONE.trousers ? TONE.trousersFar : t === TONE.shoe ? TONE.shoeFar : t === TONE.hand ? TONE.handFar : t);
export const COAT = { material: "rain-shell", tone: TONE.coat } as const;
export const SKIN = { material: "skin", tone: TONE.skin } as const;
export const BONE = { material: "bone", tone: TONE.bone } as const;
type Slice = readonly [number, number, number, number, number];

/* ---------------------------------------------------------------- stations shared by body, thermal and the coarser scales */

/** 🧥 The raincoat on the trunk above the waist as horizontal slices [y, back, front, half-width, exponent]: shoulder blades, chest, the shoulder line falling from the neck to the sleeve heads. */
export const COAT_UPPER: readonly Slice[] = [
  [1.0, -0.142, 0.142, 0.186, 2.3],
  [1.06, -0.135, 0.143, 0.183, 2.3],
  [1.15, -0.132, 0.148, 0.184, 2.3],
  [1.25, -0.138, 0.155, 0.19, 2.3],
  [1.33, -0.14, 0.155, 0.196, 2.3],
  [1.385, -0.132, 0.14, 0.205, 2.4],
  [1.42, -0.115, 0.115, 0.195, 2.5],
  [1.445, -0.095, 0.09, 0.155, 2.3],
  [1.462, -0.075, 0.07, 0.1, 2.1],
];
/** 🧥 The skirt of the coat from the waist to the hem above the knee: it clears the buttocks and flares to an A line. */
export const COAT_SKIRT: readonly Slice[] = [
  [1.05, -0.135, 0.143, 0.183, 2.3],
  [0.98, -0.15, 0.144, 0.19, 2.3],
  [0.92, -0.162, 0.146, 0.198, 2.3],
  [0.84, -0.166, 0.15, 0.204, 2.35],
  [0.7, -0.172, 0.156, 0.212, 2.4],
  [0.6, -0.178, 0.162, 0.22, 2.4],
  [0.535, -0.182, 0.166, 0.226, 2.4],
];
export const slices = (bone: string, t: readonly Slice[]): St[] => t.map(([y, b, f, rw, e]) => H(bone, y, b, f, rw, e));
/** 📏 The surface of a sliced garment at the height `y` and the lateral position `z`: x of the front (+1) or of the back (-1). */
export function surfaceX(t: readonly Slice[], y: number, z: number, side: 1 | -1): number {
  const s = [...t].sort((a, b) => a[0] - b[0]);
  let i = 0;
  while (i + 2 < s.length && s[i + 1]![0] < y) i++;
  const a = s[i]!;
  const b = s[i + 1]!;
  const k = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (j: number): number => a[j]! + (b[j]! - a[j]!) * k;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const c = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * c * (front - back)) / 2;
}
/** 🩹 A strip lying on a sliced garment: at every height a thin section `proud` of the surface, `half` wide about the lateral position `z`. */
const onSurface = (bone: string, t: readonly Slice[], side: 1 | -1, rows: readonly (readonly [number, number, number])[], proud: number, sunk = 0.006): St[] =>
  rows.map(([y, z, half]) => {
    const x = surfaceX(t, y, z, side);
    return H(bone, y, side > 0 ? x - sunk : x - proud, side > 0 ? x + proud : x + sunk, half, 3, z);
  });

/** 🗣️ Head and face as horizontal slices [y, back, front, half-width, exponent]: chin, jaw, cheeks, zygoma, brow, forehead, vault; 0.227 m from chin to vertex, 0.196 m long, 0.151 m broad. */
export const HEAD: readonly Slice[] = [
  [1.493, 0.05, 0.086, 0.02, 2],
  [1.504, 0.022, 0.097, 0.039, 2],
  [1.524, -0.014, 0.1, 0.052, 2.1],
  [1.545, -0.046, 0.1, 0.0605, 2.1],
  [1.566, -0.07, 0.098, 0.066, 2.1],
  [1.59, -0.082, 0.098, 0.071, 2.1],
  [1.615, -0.088, 0.101, 0.074, 2.1],
  [1.64, -0.09, 0.106, 0.0755, 2.1],
  [1.665, -0.088, 0.103, 0.075, 2.1],
  [1.69, -0.078, 0.09, 0.069, 2],
  [1.708, -0.058, 0.064, 0.055, 2],
  [1.7175, -0.03, 0.036, 0.032, 2],
];
export const NECK: readonly Slice[] = [
  [1.47, -0.057, 0.051, 0.055, 2],
  [1.5, -0.054, 0.048, 0.054, 2],
  [1.52, -0.052, 0.046, 0.054, 2],
  [1.56, -0.05, 0.042, 0.055, 2],
];
/** 💇 The hair as cross slices from the hairline to the nape [x, bottom, top, half-width]: swept back over the vault, free ears, down to the nape where the bun sits. */
export const HAIR: readonly (readonly [number, number, number, number])[] = [
  [0.097, 1.668, 1.697, 0.05],
  [0.07, 1.655, 1.717, 0.071],
  [0.03, 1.641, 1.727, 0.0805],
  [-0.01, 1.632, 1.728, 0.0825],
  [-0.05, 1.588, 1.722, 0.0805],
  [-0.08, 1.562, 1.702, 0.068],
  [-0.097, 1.568, 1.666, 0.045],
];
export const BUN = { at: [-0.112, 1.6, 0] as V3, radii: [0.034, 0.03, 0.036] as V3 };

/** 🧥 The left sleeve on the upper arm: sleeve head inwards of the joint under the shoulder seam, deltoid, the elbow carried 35 mm outwards. */
export const SLEEVE_L: St[] = [st(-0.03, 0, -0.03, 0.042, 0.056), st(0.0, 0, -0.008, 0.058, 0.066), st(0.05, 0.002, 0.004, 0.062, 0.068), st(0.12, 0.004, 0.012, 0.058, 0.064), st(0.2, 0.002, 0.022, 0.054, 0.059), st(0.27, 0, 0.03, 0.052, 0.056), st(0.316, -0.002, 0.035, 0.051, 0.055)];
/** 💪 The left sleeve on the forearm down to its hem 26 mm above the wrist; the wrist lies 20 mm outwards of the elbow. */
export const FORE_L: St[] = [st(0.0, 0, 0, 0.0475, 0.0505), st(0.05, 0, 0.004, 0.047, 0.05), st(0.13, 0, 0.011, 0.044, 0.046), st(0.2, 0, 0.016, 0.041, 0.042), st(0.222, 0, 0.018, 0.041, 0.042)];
export const SHIRT_CUFF_L: St[] = [st(0.21, 0, 0.017, 0.031, 0.034), st(0.25, 0, 0.02, 0.029, 0.032)];
/** 👖 The left trouser leg: thigh with the quadriceps in front, knee, the calf pushing the cloth back, the hem breaking on the instep. */
export const THIGH_L: St[] = [st(0.0, -0.01, 0, 0.082, 0.098, 2.1), st(0.1, -0.006, 0, 0.08, 0.092, 2.1), st(0.2, 0, -0.002, 0.072, 0.082, 2), st(0.3, 0.002, -0.004, 0.065, 0.073, 2), st(0.38, 0.004, -0.004, 0.06, 0.067, 2), st(0.4202, 0.004, -0.004, 0.058, 0.065, 2)];
export const SHANK_L: St[] = [st(-0.02, 0.004, -0.004, 0.0545, 0.0615), st(0.06, 0.002, -0.004, 0.055, 0.062), st(0.16, -0.004, -0.004, 0.056, 0.064), st(0.26, -0.004, -0.003, 0.054, 0.061), st(0.33, 0, -0.002, 0.053, 0.062), st(0.372, 0.005, -0.002, 0.054, 0.066, 2, -5), st(0.395, 0.006, -0.002, 0.053, 0.065, 2, -9)];

/** 🥾 The left derby as cross slices [x, bottom, top, half-width]: quarters and heel counter on the foot bone, vamp and toe on the toe bone, overlapping at the ball; the upper sits on a sole that rises onto a 22 mm heel. */
export const SHOE_REAR: readonly (readonly [number, number, number, number])[] = [
  [-0.071, 0.046, 0.082, 0.017],
  [-0.058, 0.029, 0.097, 0.031],
  [-0.03, 0.025, 0.106, 0.038],
  [0.01, 0.019, 0.106, 0.042],
  [0.05, 0.014, 0.092, 0.046],
  [0.09, 0.01, 0.072, 0.049],
  [0.127, 0.009, 0.06, 0.051],
  [0.15, 0.009, 0.055, 0.0505],
];
export const SHOE_TOE: readonly (readonly [number, number, number, number])[] = [
  [0.12, 0.009, 0.061, 0.0505],
  [0.16, 0.009, 0.053, 0.049],
  [0.19, 0.009, 0.045, 0.043],
  [0.21, 0.009, 0.036, 0.031],
  [0.2205, 0.011, 0.026, 0.014],
];
export const SOLE_REAR: readonly (readonly [number, number, number, number])[] = [
  [-0.073, 0.02, 0.03, 0.032],
  [-0.02, 0.019, 0.029, 0.041],
  [0.04, 0.009, 0.019, 0.045],
  [0.1, 0.0, 0.01, 0.0505],
  [0.135, 0.0, 0.01, 0.0525],
];
export const SOLE_TOE: readonly (readonly [number, number, number, number])[] = [
  [0.12, 0.0, 0.01, 0.0525],
  [0.17, 0.0, 0.01, 0.0505],
  [0.2, 0.0, 0.01, 0.04],
  [0.223, 0.002, 0.01, 0.018],
];
export const crossSlices = (bone: string, t: readonly (readonly [number, number, number, number])[], e: number, z = 0): St[] => t.map(([x, b, top, rw]) => X(bone, x, b, top, rw, e, z));

/* ---------------------------------------------------------------- the umbrella */

const UF = FRAMES.umbrella!;
const add = (a: V3, b: V3, k: number): V3 => [a[0] + b[0] * k, a[1] + b[1] * k, a[2] + b[2] * k];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => [a[0] / len(a), a[1] / len(a), a[2] / len(a)];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const NOTCH: V3 = add(UF.o, UF.u, RIB.notch);
/** 🌂 A point of the open canopy in the figure: on the meridian at the azimuth `az` (degrees, 0 ahead, 90 left), `s` along the rib chord, pulled towards the axis by `shrink` (the straight cloth between two ribs). */
export function canopyPoint(az: number, s: number, shrink = 1): V3 {
  const th = (RIB.openDeg * Math.PI) / 180;
  const b = 4 * RIB.bow * (s / RIB.chord) * (1 - s / RIB.chord);
  const r = (s * Math.sin(th) + b * Math.cos(th)) * shrink;
  const a = -s * Math.cos(th) + b * Math.sin(th);
  const p = (az * Math.PI) / 180;
  return [0, 1, 2].map((k) => NOTCH[k]! + a * UF.u[k]! + r * (-Math.cos(p) * UF.v[k]! + Math.sin(p) * UF.w[k]!)) as V3;
}
/** 🪡 Half a canopy panel on the rib `key`: the cloth from the rib to the middle of the panel on the side `turn` (+1 towards larger azimuth), a straight chord at every station as the tensioned cloth lies, `t` thick. */
export function halfGore(key: string, turn: 1 | -1, stations: readonly number[], t: number, id: string): Json {
  const az = AZIMUTH[key]!;
  const bone = `rib-${key}`;
  const mid = (s: number): V3 => { const p = canopyPoint(az, s); const q = canopyPoint(az + turn * 45, s); return [(p[0] + q[0]) / 2, (p[1] + q[1]) / 2, (p[2] + q[2]) / 2]; };
  const centre = (s: number): V3 => { const p = canopyPoint(az, s); const m = mid(s); return [(p[0] + m[0]) / 2, (p[1] + m[1]) / 2, (p[2] + m[2]) / 2]; };
  const f = FRAMES[bone]!;
  const local = (d: V3): V3 => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
  return freeLoft({ id, bone, material: "umbrella-fabric", tone: TONE.canopy }, stations.map((s) => {
    const p = canopyPoint(az, s);
    const m = mid(s);
    const ex = unit(sub(m, p));
    const travel = unit(sub(centre(s + 0.004), centre(s - 0.004)));
    let ey = unit(cross(travel, ex));
    if (dot(cross(ex, ey), travel) < 0) ey = [-ey[0], -ey[1], -ey[2]];
    return { at: L(bone, centre(s)), rx: (len(sub(m, p)) / 2) * 1.05, ry: t / 2, ex: local(ex), ey: local(ey) };
  }));
}
const RIB_STATIONS = [0.014, 0.07, 0.14, 0.21, 0.28, 0.35, 0.42, 0.49, 0.556];
const GORE_STATIONS = [0.02, 0.07, 0.14, 0.22, 0.31, 0.4, 0.48, 0.556];
export const CROOK_R = 0.035;
/** 🪝 The centre line of the crook handle in the umbrella frame (u up the shaft, v towards the person): the straight grip, the half turn and the nose. */
export const crookPath = (n = 10): V3[] => [[0.058, 0, 0], [0.0, 0, 0], [-0.055, 0, 0], ...Array.from({ length: n }, (_, i) => [-0.055 - CROOK_R * Math.sin((Math.PI * (i + 1)) / n), CROOK_R - CROOK_R * Math.cos((Math.PI * (i + 1)) / n), 0] as V3), [-0.036, 2 * CROOK_R, 0]];

function umbrella(): Json[] {
  const out: Json[] = [];
  const metal = { material: "umbrella-metal", tone: TONE.metal } as const;
  out.push(capsule({ id: "shaft", bone: "umbrella", ...metal }, [0.05, 0, 0], [0.8, 0, 0], 0.005, 0.005, { slender: true }));
  out.push(tube({ id: "handle", bone: "umbrella", material: "umbrella-handle", tone: TONE.wood }, crookPath(), [0.0115, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.012, 0.0115, 0.0105]));
  out.push(revolve({ id: "handle-collar", bone: "umbrella", ...metal, role: "detail" }, [[0.056, 0], [0.056, 0.0125], [0.07, 0.0125], [0.076, 0.007], [0.076, 0]], [0, 0, 0], [1, 0, 0]));
  out.push(revolve({ id: "runner", bone: "runner", ...metal, role: "detail" }, [[-0.034, 0], [-0.034, 0.0085], [-0.008, 0.0085], [-0.008, 0.013], [0.006, 0.013], [0.006, 0.0085], [0.012, 0.0085], [0.012, 0]], [0, 0, 0], [1, 0, 0]));
  out.push(revolve({ id: "notch", bone: "umbrella", ...metal, role: "detail" }, [[0.77, 0], [0.77, 0.0085], [0.776, 0.014], [0.786, 0.014], [0.786, 0]], [0, 0, 0], [1, 0, 0]));
  out.push(revolve({ id: "ferrule", bone: "umbrella", ...metal, role: "detail" }, [[0.786, 0], [0.786, 0.019], [0.793, 0.017], [0.803, 0.0075], [0.832, 0.0048], [0.84, 0.0035], [0.84, 0]], [0, 0, 0], [1, 0, 0]));
  for (const key of Object.keys(AZIMUTH)) {
    const bone = `rib-${key}`;
    out.push(tube({ id: `rib-${key}`, bone, ...metal, role: "detail" }, RIB_STATIONS.map((s) => L(bone, canopyPoint(AZIMUTH[key]!, s, 0.995))), [0.0022], { slender: true, ...(key === "front" || key === "back" ? { texture: true } : {}) }));
    out.push(ellipsoid({ id: `rib-tip-${key}`, bone, ...metal, role: "detail" }, [0.0075, 0.0045, 0.0045], L(bone, canopyPoint(AZIMUTH[key]!, RIB.chord))));
    out.push(capsule({ id: `stretcher-${key}`, bone: `stretcher-${key}`, ...metal, role: "detail" }, [0, 0, 0], [STRETCHER, 0, 0], 0.0018, 0.0018, { slender: true }));
    out.push(halfGore(key, 1, GORE_STATIONS, 0.0076, `canopy-${key}-a`));
    out.push(halfGore(key, -1, GORE_STATIONS, 0.0076, `canopy-${key}-b`));
  }
  return out;
}

/* ---------------------------------------------------------------- the hands */

/** ✋ The hanging left hand: palm towards the thigh, thumb ahead, the fingers a little curled. */
function leftHand(): Json[] {
  const p = { bone: "hand-l", material: "skin", tone: TONE.hand } as const;
  const out: Json[] = [];
  out.push(loft({ id: "hand-l", ...p }, dome([st(0.0, 0, 0, 0.019, 0.028, 2.4), st(0.03, 0.002, 0, 0.016, 0.038, 2.6), st(0.07, 0.003, 0, 0.014, 0.043, 2.6), st(0.097, 0.002, 0, 0.0125, 0.042, 2.6)], 1, 0.008, 2)));
  out.push(ellipsoid({ id: "hand-l-thenar", ...p }, [0.03, 0.017, 0.012], [0.04, 0.024, -0.009]));
  const finger = (name: string, v: number, length: number, r: number): void => {
    const pts: V3[] = [[0.09, v, 0]];
    let a = 0;
    for (const [share, bend] of [[0.45, 18], [0.3, 34], [0.25, 22]] as const) {
      a += bend;
      const q = pts[pts.length - 1]!;
      pts.push([q[0] + length * share * Math.cos((a * Math.PI) / 180), v * (1 + 0.02 * pts.length), q[2] - length * share * Math.sin((a * Math.PI) / 180)]);
    }
    out.push(tube({ id: `hand-l-${name}`, ...p }, pts, [r, r * 0.96, r * 0.9, r * 0.8]));
  };
  finger("index", 0.03, 0.072, 0.0088);
  finger("middle", 0.0105, 0.08, 0.009);
  finger("ring", -0.009, 0.075, 0.0086);
  finger("little", -0.0275, 0.06, 0.0076);
  out.push(tube({ id: "hand-l-thumb", ...p }, [[0.03, 0.03, -0.006], [0.062, 0.047, -0.013], [0.09, 0.053, -0.02], [0.108, 0.054, -0.026]], [0.0115, 0.0105, 0.0092, 0.0078]));
  return out;
}
const GRIP_DEG = 75;
/** ✊ The right fist round the handle: the handle lies in the palm 26 mm medial of the wrist line, the four fingers wrap it from the knuckles, the thumb closes over the index finger. */
function rightHand(): Json[] {
  const p = { bone: "hand-r", material: "skin", tone: TONE.hand } as const;
  const out: Json[] = [];
  out.push(loft({ id: "hand-r", ...p }, dome([st(0.0, 0, 0, 0.019, 0.028, 2.4), st(0.03, 0.002, -0.002, 0.016, 0.039, 2.6), st(0.065, 0.004, -0.003, 0.014, 0.044, 2.6), st(0.09, 0.005, -0.003, 0.013, 0.043, 2.6)], 1, 0.008, 2)));
  out.push(ellipsoid({ id: "hand-r-thenar", ...p }, [0.03, 0.018, 0.013], [0.036, 0.022, 0.006]));
  const s: V3 = [Math.cos((GRIP_DEG * Math.PI) / 180), Math.sin((GRIP_DEG * Math.PI) / 180), 0];
  const q: V3 = [s[1], -s[0], 0];
  const C: V3 = [0.075, 0, 0.026];
  const wrap = (t: number, r: number, from: number, to: number): V3[] => [from, from + (to - from) * 0.2, from + (to - from) * 0.4, from + (to - from) * 0.6, from + (to - from) * 0.8, to].map((phi) => { const a = (phi * Math.PI) / 180; return [C[0] + s[0] * t + q[0] * r * Math.cos(a), C[1] + s[1] * t + q[1] * r * Math.cos(a), C[2] + r * Math.sin(a)] as V3; });
  const finger = (name: string, t: number, r: number, to: number): void => { out.push(tube({ id: `hand-r-${name}`, ...p }, wrap(t, 0.012 + r, -62, to), [r * 1.05, r, r, r, r * 0.95, r * 0.85])); };
  finger("index", 0.0335, 0.0088, 168);
  finger("middle", 0.0125, 0.009, 176);
  finger("ring", -0.0085, 0.0086, 172);
  finger("little", -0.0285, 0.0076, 164);
  out.push(tube({ id: "hand-r-thumb", ...p }, [[0.03, 0.03, 0.006], [0.05, 0.047, 0.028], [0.07, 0.055, 0.046], [0.09, 0.055, 0.054]], [0.012, 0.011, 0.0098, 0.0085]));
  return out;
}

/* ---------------------------------------------------------------- body */

const pairs = (left: Json[], tone = true): Json[] => left.flatMap((s) => [mirrored(s, toRight), tone ? { ...s, tone: far(s.tone) } : s]);

export function body50(): Json[] {
  const out: Json[] = [];
  const head = { bone: "head", ...SKIN } as const;
  out.push(loft({ id: "neck", ...head }, dense(slices("head", NECK), 2)));
  out.push(loft({ id: "head", ...head }, dome(dense(slices("head", HEAD), 3), 1, 0.0035, 2)));
  out.push(loft({ id: "head-nose", ...head }, [H("head", 1.556, 0.088, 0.121, 0.0165), H("head", 1.5675, 0.088, 0.1285, 0.0135), H("head", 1.59, 0.088, 0.1145, 0.0092), H("head", 1.617, 0.09, 0.1065, 0.008)]));
  out.push(ellipsoid({ id: "head-lips", ...head, tone: 0.66, role: "detail" }, [0.0075, 0.006, 0.023], L("head", [0.0965, 1.5345, 0])));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid({ id: `head-ear-${n}`, ...head }, [0.031, 0.016, 0.009], L("head", [-0.006, 1.597, side * 0.0765]), aboutU(side * -14)));
    out.push(ellipsoid({ id: `head-eye-${n}`, bone: "head", material: "skin", tone: 0.2, role: "detail" }, [0.0055, 0.0045, 0.0125], L("head", [0.0955, 1.612, side * 0.0315])));
    out.push(ellipsoid({ id: `head-brow-${n}`, bone: "head", material: "hair", tone: TONE.hair, role: "detail" }, [0.0038, 0.005, 0.019], L("head", [0.1, 1.6285, side * 0.033])));
  }
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dome(dome(dense(HAIR.map(([x, b, t, rw]) => X("head", x, b, t, rw, 2.3)), 2), -1, 0.008, 2), 1, 0.01, 2)));
  out.push(ellipsoid({ id: "hair-bun", bone: "head", material: "hair", tone: TONE.hair }, BUN.radii, L("head", BUN.at)));

  out.push(loft({ id: "shirt-collar", bone: "head", material: "cotton-clothing", tone: TONE.shirt }, [H("head", 1.458, -0.066, 0.064, 0.066, 2), H("head", 1.485, -0.064, 0.058, 0.063, 2), { ...H("head", 1.506, -0.062, 0.052, 0.061, 2), n: 90 - 90 + 7 }]));
  out.push(loft({ id: "coat", bone: "torso", ...COAT }, dense(slices("torso", COAT_UPPER), 2)));
  out.push(loft({ id: "coat-skirt", bone: "pelvis", ...COAT }, dense(slices("pelvis", COAT_SKIRT), 2)));
  out.push(loft({ id: "coat-collar", bone: "torso", ...COAT, tone: 0.5 }, [H("torso", 1.45, -0.088, 0.084, 0.108, 2.2), H("torso", 1.474, -0.084, 0.07, 0.088, 2.1), { ...H("torso", 1.494, -0.078, 0.052, 0.074, 2), n: 9 }]));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(loft({ id: `coat-lapel-${n}`, bone: "torso", ...COAT, tone: 0.5, role: "detail" }, [H("torso", 1.456, 0.02, 0.088, 0.031, 2.6, side * 0.076), H("torso", 1.432, 0.058, 0.124, 0.037, 2.6, side * 0.068), H("torso", 1.408, 0.1, 0.147, 0.029, 2.6, side * 0.059), H("torso", 1.388, 0.132, 0.154, 0.013, 2.4, side * 0.05)]));
    out.push(loft({ id: `coat-pocket-${n}`, bone: "pelvis", ...COAT, tone: 0.46, role: "detail" }, onSurface("pelvis", COAT_SKIRT, 1, [[0.89, side * 0.152, 0.012], [0.95, side * 0.139, 0.0125], [1.01, side * 0.126, 0.012]], 0.004)));
    out.push(loft({ id: `coat-fold-front-${n}`, bone: "pelvis", ...COAT, role: "detail" }, onSurface("pelvis", COAT_SKIRT, 1, [[0.537, side * 0.118, 0.026], [0.62, side * 0.114, 0.021], [0.72, side * 0.11, 0.014], [0.82, side * 0.107, 0.007]], 0.009, 0.012)));
    out.push(loft({ id: `coat-fold-back-${n}`, bone: "pelvis", ...COAT, role: "detail" }, onSurface("pelvis", COAT_SKIRT, -1, [[0.537, side * 0.1, 0.028], [0.62, side * 0.097, 0.022], [0.72, side * 0.093, 0.015], [0.83, side * 0.09, 0.007]], 0.009, 0.012)));
  }
  out.push(loft({ id: "coat-yoke", bone: "torso", ...COAT, tone: 0.52, role: "detail" }, [H("torso", 1.238, -0.1425, -0.04, 0.152, 2.3), H("torso", 1.3, -0.1435, -0.04, 0.166, 2.3), H("torso", 1.36, -0.1395, -0.04, 0.172, 2.3), H("torso", 1.405, -0.1215, -0.04, 0.162, 2.3)]));
  out.push(loft({ id: "coat-placket", bone: "torso", ...COAT, tone: 0.5, role: "detail" }, onSurface("torso", COAT_UPPER, 1, [[1.03, -0.014, 0.026], [1.15, -0.014, 0.026], [1.3, -0.014, 0.026], [1.385, -0.014, 0.026]], 0.0035)));
  out.push(loft({ id: "coat-placket-skirt", bone: "pelvis", ...COAT, tone: 0.5, role: "detail" }, onSurface("pelvis", COAT_SKIRT, 1, [[1.045, -0.014, 0.026], [0.9, -0.014, 0.026], [0.7, -0.014, 0.026], [0.537, -0.014, 0.026]], 0.0035)));
  out.push(loft({ id: "coat-belt", bone: "pelvis", ...COAT, tone: 0.5 }, [H("pelvis", 0.99, -0.156, 0.151, 0.196, 2.3), H("pelvis", 1.0175, -0.153, 0.1515, 0.194, 2.3), H("pelvis", 1.045, -0.146, 0.1505, 0.1915, 2.3)]));
  out.push(box({ id: "coat-buckle", bone: "pelvis", material: "rain-shell", tone: TONE.button, role: "detail" }, [0.008, 0.05, 0.05], L("pelvis", [0.1535, 1.0175, -0.03]), undefined, 0.004));
  for (const [i, y] of [1.345, 1.215, 1.085].entries()) out.push(ellipsoid({ id: `coat-button-${i + 1}`, bone: "torso", material: "rain-shell", tone: TONE.button, role: "detail" }, [0.011, 0.004, 0.011], L("torso", [surfaceX(COAT_UPPER, y, 0.014, 1) + 0.0045, y, -0.014])));
  out.push(ellipsoid({ id: "coat-button-4", bone: "pelvis", material: "rain-shell", tone: TONE.button, role: "detail" }, [0.011, 0.004, 0.011], L("pelvis", [surfaceX(COAT_SKIRT, 0.955, 0.014, 1) + 0.0045, 0.955, -0.014])));

  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "upper-arm-l", ...COAT }, dome(dense(SLEEVE_L, 2), 1, 0.012, 3)));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", ...COAT }, dome(dense(FORE_L, 2), -1, 0.04, 3)));
  arm.push(loft({ id: "sleeve-strap-l", bone: "forearm-l", ...COAT, tone: 0.48, role: "detail" }, [st(0.166, 0, 0.0135, 0.0462, 0.0478), st(0.194, 0, 0.0158, 0.045, 0.0462)]));
  arm.push(loft({ id: "shirt-cuff-l", bone: "forearm-l", material: "cotton-clothing", tone: TONE.shirt }, SHIRT_CUFF_L, [false, false]));
  out.push(...pairs(arm));
  out.push(...leftHand().map((s) => ({ ...s, tone: far(s.tone) })), ...rightHand());

  const leg: Json[] = [];
  const cloth = { material: "cotton-clothing", tone: TONE.trousers } as const;
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", ...cloth }, dome(dense(THIGH_L, 2), 1, 0.012, 2)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", ...cloth }, dome(dense(SHANK_L, 2), -1, 0.04, 3)));
  const shoe = { material: "leather-shoe", tone: TONE.shoe } as const;
  leg.push(loft({ id: "shoe-l", bone: "foot-l", ...shoe }, dense(crossSlices("foot-l", SHOE_REAR, 2.5), 2)));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", ...shoe }, dense(crossSlices("toe-l", SHOE_TOE, 2.5), 2)));
  leg.push(loft({ id: "shoe-sole-l", bone: "foot-l", ...shoe, tone: 0.08 }, crossSlices("foot-l", SOLE_REAR, 5)));
  leg.push(loft({ id: "shoe-toe-sole-l", bone: "toe-l", ...shoe, tone: 0.08 }, crossSlices("toe-l", SOLE_TOE, 5)));
  leg.push(extrude({ id: "shoe-heel-l", bone: "foot-l", ...shoe, tone: 0.08 }, "zx", [[-0.03, -0.006], [0.03, -0.006], [0.033, -0.04], [0.026, -0.064], [0.012, -0.0735], [-0.012, -0.0735], [-0.026, -0.064], [-0.033, -0.04]], -0.086, -0.0655));
  out.push(...pairs(leg));
  out.push(...umbrella());
  return out;
}

/* ---------------------------------------------------------------- thermal */

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const has = (id: string): boolean => body.some((s) => s.id === id);
  const of = (id: string, region: string, layer?: string): void => { if (!has(id)) throw new Error(`no body solid ${id}`); out.push(thOf(id, region, layer)); };
  for (const id of ["neck", "head", "head-nose", "head-lips", "head-ear-l", "head-ear-r", "head-eye-l", "head-eye-r"]) of(id, "head-skin");
  for (const id of ["hair", "hair-bun", "head-brow-l", "head-brow-r"]) of(id, "hair", "hair-pack");
  of("shirt-collar", "torso-clothing", "collar");
  for (const s of body) if (s.material === "rain-shell") of(s.id, "coat", "coat-shell");
  const shell = { material: "rain-shell", tone: 0.7 } as const;
  const shirt = { material: "cotton-clothing", tone: TONE.shirt } as const;
  const AIR = 0.0006;
  const SHIRT = 0.0126;
  out.push({ ...loft({ id: "th-coat-air", bone: "torso", ...shell }, inset(dense(slices("torso", COAT_UPPER), 2), AIR)), region: "coat", layer: "coat-air" });
  out.push({ ...loft({ id: "th-coat-shirt", bone: "torso", ...shirt }, inset(dense(slices("torso", COAT_UPPER), 2), SHIRT)), region: "coat", layer: "shirt-under" });
  out.push({ ...loft({ id: "th-coat-skirt-air", bone: "pelvis", ...shell }, inset(dense(slices("pelvis", COAT_SKIRT), 2), AIR)), region: "coat", layer: "coat-air" });
  out.push({ ...loft({ id: "th-coat-skirt-shirt", bone: "pelvis", ...shirt }, inset(slices("pelvis", COAT_SKIRT.slice(0, 3)), SHIRT + 0.012)), region: "coat", layer: "shirt-under" });
  const layered: Json[] = [];
  layered.push({ ...loft({ id: "th-sleeve-l-air", bone: "upper-arm-l", ...shell }, dome(inset(dense(SLEEVE_L, 2), AIR), 1, 0.011, 3)), region: "coat", layer: "coat-air" });
  layered.push({ ...loft({ id: "th-sleeve-l-shirt", bone: "upper-arm-l", ...shirt }, dome(inset(dense(SLEEVE_L, 2), SHIRT), 1, 0.01, 3)), region: "coat", layer: "shirt-under" });
  layered.push({ ...loft({ id: "th-forearm-l-air", bone: "forearm-l", ...shell }, dome(inset(dense(FORE_L, 2), AIR), -1, 0.039, 3)), region: "coat", layer: "coat-air" });
  layered.push({ ...loft({ id: "th-forearm-l-shirt", bone: "forearm-l", ...shirt }, dome(inset(dense(FORE_L, 2), SHIRT), -1, 0.03, 3)), region: "coat", layer: "shirt-under" });
  layered.push(thOf("shirt-cuff-l", "arm-clothing", "cuff"));
  for (const id of ["hand-l", "hand-l-thenar", "hand-l-index", "hand-l-middle", "hand-l-ring", "hand-l-little", "hand-l-thumb"]) of(id, "hand-skin");
  for (const id of ["hand-r", "hand-r-thenar", "hand-r-index", "hand-r-middle", "hand-r-ring", "hand-r-little", "hand-r-thumb"]) of(id, "hand-skin");
  const cloth = { material: "cotton-clothing", tone: TONE.trousers } as const;
  layered.push(thOf("shank-l", "leg-clothing", "trousers"));
  layered.push({ ...loft({ id: "th-shank-l-air", bone: "shank-l", ...cloth, tone: 0.5 }, inset(dense(SHANK_L, 2), 0.0012)), region: "leg-clothing", layer: "trouser-air" });
  for (const id of ["shoe-l", "shoe-toe-l", "shoe-sole-l", "shoe-toe-sole-l", "shoe-heel-l"]) layered.push(thOf(id, "shoes", "shoe-upper"));
  const leather = { material: "leather-shoe", tone: 0.4 } as const;
  layered.push({ ...loft({ id: "th-shoe-l-air", bone: "foot-l", ...leather }, inset(dense(crossSlices("foot-l", SHOE_REAR, 2.5), 2), 0.003)), region: "shoes", layer: "shoe-air" });
  layered.push({ ...loft({ id: "th-shoe-l-sock", bone: "foot-l", material: "cotton-clothing", tone: 0.7 }, inset(dense(crossSlices("foot-l", SHOE_REAR, 2.5), 2), 0.011)), region: "shoes", layer: "sock-layer" });
  layered.push({ ...loft({ id: "th-shoe-toe-l-air", bone: "toe-l", ...leather }, inset(dense(crossSlices("toe-l", SHOE_TOE, 2.5), 2), 0.003)), region: "shoes", layer: "shoe-air" });
  layered.push({ ...loft({ id: "th-shoe-toe-l-sock", bone: "toe-l", material: "cotton-clothing", tone: 0.7 }, inset(dense(crossSlices("toe-l", SHOE_TOE.slice(0, 4), 2.5), 2), 0.011)), region: "shoes", layer: "sock-layer" });
  for (const s of layered) out.push(s, mirrored(s, toRight));
  for (const s of body) {
    if (s.material === "umbrella-fabric") out.push(thOf(s.id, "canopy"));
    if (s.material === "umbrella-metal" || s.material === "umbrella-handle") out.push(thOf(s.id, "umbrella-frame"));
  }
  out.push({ ...ellipsoid({ id: "th-nose-breath", bone: "head", ...SKIN }, [0.0038, 0.0048, 0.0105], L("head", [0.1165, 1.5565, 0])), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid({ id: "th-mouth-breath", bone: "head", ...SKIN }, [0.0045, 0.004, 0.0075], L("head", [0.1015, 1.5345, 0])), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...loft({ id: "th-plume", bone: "torso", ...shell }, [H("torso", 1.4, -0.142, -0.02, 0.165, 2.6), H("torso", 1.43, -0.118, -0.02, 0.185, 2.6), H("torso", 1.448, -0.097, -0.02, 0.152, 2.4)]), region: "coat", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

export const kgOf = (bone: string): { kg: number; comX: number; comY: number } => (ASSET.rig.bones.find((b: Json) => b.id === bone) as Json).mass;
export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const joint = (list: Json[], member: string, at: V3, r: number): void => { list.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint")); };
  const mass = (list: Json[], member: string): void => {
    const k = kgOf(member);
    const r = 0.0125 * Math.cbrt(k.kg);
    list.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass"));
  };
  const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  const HD = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
  const spine: [number, number, number][] = [[-0.036, 1.03, 0.02], [-0.026, 1.07, 0.0195], [-0.022, 1.115, 0.019], [-0.03, 1.165, 0.018], [-0.048, 1.225, 0.0165], [-0.062, 1.285, 0.0155], [-0.064, 1.345, 0.0145], [-0.056, 1.4, 0.0135], [-0.044, 1.45, 0.0125]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const RIB_TILT = 20;
  const hoops: [number, number, number, number][] = [[1.128, 0.022, 0.084, 0.112], [1.16, 0.022, 0.094, 0.124], [1.192, 0.02, 0.101, 0.132], [1.224, 0.018, 0.105, 0.137], [1.256, 0.014, 0.106, 0.138], [1.288, 0.01, 0.103, 0.135], [1.318, 0.004, 0.096, 0.127], [1.346, -0.002, 0.086, 0.114], [1.371, -0.008, 0.073, 0.097], [1.392, -0.014, 0.057, 0.075]];
  hoops.forEach(([y, x, rd, rw], i) => {
    const ring: [number, number][] = Array.from({ length: 8 }, (_, k) => [0.0058 * Math.cos((k * Math.PI) / 4), rd + 0.0058 * Math.sin((k * Math.PI) / 4)] as [number, number]);
    m(revolve({ id: `rib-cage-${10 - i}`, bone: "torso", ...BONE }, ring, [0, 0, 0], [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000, transform: { translation: T(x, y).map((q) => Math.round(q * 1e5) / 1e5), rotation: aboutW(-RIB_TILT) } }), "torso");
  });
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.228, 0.112, 0.123, 0.012), H("torso", 1.27, 0.111, 0.123, 0.018), H("torso", 1.34, 0.094, 0.106, 0.02), H("torso", 1.395, 0.064, 0.077, 0.027)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.066, 1.403, side * 0.022), T(-0.012, 1.432, side * 0.178), 0.008, 0.0075), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.078, 0.007, 0.05], T(-0.078, 1.335, side * 0.104), aboutU(side * 26)), "torso");
  }
  joint(out, "torso", [0, 0, 0], 0.02);
  mass(out, "torso");
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.038, 1.03), P(-0.06, 0.985), P(-0.074, 0.945), P(-0.07, 0.915)], [0.028, 0.026, 0.017, 0.008]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.066, 0.072, 0.012], P(-0.026, 0.99, side * 0.098), aboutU(side * 32)), "pelvis");
    m(capsule({ id: `ilium-body-${n}`, bone: "pelvis", ...BONE }, P(-0.056, 0.992, side * 0.034), P(-0.002, 0.927, side * 0.084), 0.017, 0.019), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.004, 0.925, side * 0.084), P(0.052, 0.905, side * 0.012), 0.012, 0.011), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.01, 0.92, side * 0.082), P(-0.042, 0.858, side * 0.058), 0.013, 0.014), "pelvis");
  }
  mass(out, "pelvis");
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.066, 0.09, 0.0685], HD(0.004, 1.648)), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.04, 0.034, 0.05], HD(0.058, 1.585)), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, HD(0.004, 1.588, side * 0.05), HD(0.0, 1.535, side * 0.046), 0.0075, 0.008), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, HD(0.0, 1.535, side * 0.046), HD(0.07, 1.508, side * 0.008), 0.008, 0.009), "head");
  }
  m(tube({ id: "cervical-spine", bone: "head", ...BONE }, [HD(-0.044, 1.45), HD(-0.03, 1.492), HD(-0.02, 1.535), HD(-0.014, 1.578)], [0.0125, 0.012, 0.0115, 0.011]), "head");
  joint(out, "head", [0, 0, 0], 0.016);
  mass(out, "head");

  const side: Json[] = [];
  const lm = (solid: Json, member: string): void => { side.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.058], 0.0165, 0.0175), "thigh-l");
  lm(ellipsoid({ id: "femur-head-l", bone: "thigh-l", ...BONE }, [0.024, 0.024, 0.024], [0, 0, 0]), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.024, 0.018, 0.015], [0.03, -0.008, 0.066]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.062], [0.13, 0.008, 0.046], [0.22, 0.013, 0.03], [0.31, 0.008, 0.014], [0.398, 0.0, 0.002]], [0.0155, 0.0142, 0.0135, 0.0142, 0.017]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.021, 0.031, 0.041], [0.412, -0.006, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.021, 0.0095, 0.02], [0.41, 0.042, 0]), "thigh-l");
  mass(side, "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.015, 0.027, 0.038], [0.02, -0.002, 0]), "shank-l");
  lm(loft({ id: "tibia-l", bone: "shank-l", ...BONE }, [st(0.026, 0.002, -0.002, 0.021, 0.019, 2), st(0.062, 0.008, -0.003, 0.016, 0.0165, 1.8), st(0.14, 0.012, -0.003, 0.0125, 0.0145, 1.7), st(0.27, 0.009, -0.003, 0.0115, 0.0125, 1.8), st(0.366, 0.004, -0.002, 0.0125, 0.0125, 2), st(0.408, 0.002, -0.002, 0.017, 0.0145, 2)]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.014, 0.012, 0.0095], [0.416, 0.0, -0.03]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.018, 0.032], [0.424, -0.012, 0.028], 0.0068, 0.0072), "shank-l");
  joint(side, "shank-l", [0, 0, 0], 0.02);
  mass(side, "shank-l");
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, L("foot-l", [-0.048, 0.05, FRAMES["foot-l"]!.o[2]]), L("foot-l", [0.006, 0.06, FRAMES["foot-l"]!.o[2]]), 0.021, 0.019), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.004, 0.044, 0.088, 0.024), X("foot-l", 0.05, 0.034, 0.078, 0.032), X("foot-l", 0.1, 0.024, 0.054, 0.038), X("foot-l", 0.142, 0.016, 0.038, 0.042)]), "foot-l");
  joint(side, "foot-l", [0, 0, 0], 0.016);
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.13, 0.016, 0.038, 0.042), X("toe-l", 0.175, 0.014, 0.03, 0.038), X("toe-l", 0.205, 0.014, 0.025, 0.027)]), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "upper-arm-l", ...BONE }, [0.023, 0.023, 0.023], [0.002, 0, -0.012]), "upper-arm-l");
  lm(tube({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [[0.014, 0, -0.008], [0.1, 0.004, 0.008], [0.2, 0.003, 0.02], [0.3, 0, 0.033]], [0.012, 0.0108, 0.0104, 0.0125]), "upper-arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "upper-arm-l", ...BONE }, [0.012, 0.014, 0.031], [0.31, 0, 0.035]), "upper-arm-l");
  joint(side, "upper-arm-l", [0, 0, 0], 0.014);
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.016, -0.014, 0.004], [0.236, -0.004, 0.01], 0.0085, 0.0052), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.012, 0.004, 0.022], [0.242, 0.006, 0.03], 0.006, 0.009), "forearm-l");
  joint(side, "forearm-l", [0, 0, 0], 0.014);
  for (const s of side) out.push(s, mirrored(s, toRight));
  const hand = (n: "l" | "r", palmSide: number, bodyIds: string[]): void => {
    const b = `hand-${n}`;
    m(loft({ id: `carpus-${n}`, bone: b, ...BONE }, [st(0.004, 0, 0, 0.009, 0.022, 2.4), st(0.03, 0.001, 0, 0.008, 0.026, 2.4)]), b);
    for (const [k, v] of [0.027, 0.009, -0.009, -0.026].entries()) m(capsule({ id: `metacarpal-${n}-${k + 2}`, bone: b, ...BONE }, [0.032, v * 0.75, 0], [0.09, v, 0], 0.0048, 0.0055), b);
    m(capsule({ id: `metacarpal-${n}-1`, bone: b, ...BONE }, [0.02, 0.022, palmSide * 0.004], [0.052, 0.04, palmSide * 0.012], 0.0055, 0.0055), b);
    for (const id of bodyIds) {
      const f = body.find((s) => s.id === id)!;
      out.push(part({ ...f, id: id.replace("hand-", "phalanges-"), material: "bone", tone: TONE.bone, sections: f.sections.map((q: Json) => ({ ...q, rx: 0.0042, ry: 0.0042 })) }, b, "member"));
    }
    joint(out, b, [0, 0, 0], 0.011);
  };
  hand("l", -1, ["hand-l-index", "hand-l-middle", "hand-l-ring", "hand-l-little", "hand-l-thumb"]);
  hand("r", 1, ["hand-r-index", "hand-r-middle", "hand-r-ring", "hand-r-little", "hand-r-thumb"]);
  out.push({ id: "member-shaft", of: "shaft", member: "umbrella", part: "member" });
  out.push({ id: "member-handle", of: "handle", member: "umbrella", part: "member" });
  out.push({ id: "member-notch", of: "notch", member: "umbrella", part: "joint" });
  out.push({ id: "member-runner", of: "runner", member: "umbrella", part: "joint" });
  for (const key of Object.keys(AZIMUTH)) {
    out.push({ id: `member-rib-${key}`, of: `rib-${key}`, member: `rib-${key}`, part: "member" });
    out.push({ id: `member-stretcher-${key}`, of: `stretcher-${key}`, member: `stretcher-${key}`, part: "member" });
    out.push(part(ellipsoid({ id: `joint-stretcher-${key}`, bone: `stretcher-${key}`, material: "umbrella-metal", tone: 0.4 }, [0.004, 0.004, 0.004], [STRETCHER, 0, 0]), `stretcher-${key}`, "joint"));
  }
  return out;
}

export const DECISIONS_50: Json[] = [
  { en: "1:50 body: every part is a loft through hand-stated sections: head with jaw, nose, lips, ears, eyes and brows, hair swept back into a low bun, neck in a shirt collar; the raincoat as upper part, skirt flaring to the hem above the knee, collar with two leaves, back yoke, fly placket with four buttons, two slanted welt pockets, a belt with a buckle at the waist and four hanging folds; sleeves with the elbow carried outwards, cuff straps and shirt cuffs; the left hand hanging with curled fingers, the right fist with four fingers and thumb round the handle; trouser legs with knee and calf, derby shoes on sole and heel; the umbrella with crook handle, steel shaft, runner, notch, ferrule, eight bowed ribs with tips, eight stretchers pinned to them and sixteen half panels of cloth lying straight between the ribs.", de: "1:50 Körper: Jedes Teil ist ein Loft durch von Hand gesetzte Schnitte: Kopf mit Kiefer, Nase, Lippen, Ohren, Augen und Brauen, das Haar zurückgenommen in einen tiefen Knoten, der Hals im Hemdkragen; der Regenmantel als Oberteil, zum Saum über dem Knie ausgestellter Schoß, Kragen mit zwei Kragenspitzen, Rückenpasse, verdeckte Knopfleiste mit vier Knöpfen, zwei schräge Paspeltaschen, ein Gürtel mit Schnalle in der Taille und vier Fallfalten; Ärmel mit nach außen getragenem Ellbogen, Ärmelriegel und Hemdmanschetten; die linke Hand hängend mit gekrümmten Fingern, die rechte Faust mit vier Fingern und Daumen um den Griff; Hosenbeine mit Knie und Wade, Derbyschuhe auf Sohle und Absatz; der Schirm mit Rundhakengriff, Stahlstock, Schieber, Krone, Zwinge, acht gebogenen Rippen mit Spitzen, acht daran angelenkten Streben und sechzehn halben Stoffbahnen, die gerade zwischen den Rippen liegen." },
  { en: "1:50 thermal: the coat shell is the outer layer by reference; the air of the loose coat and the shirt lie inside it as nested lofts on trunk, skirt and sleeves (the shirt ends at the hip); trousers and their air layer are modelled on the shanks, where they are exposed, the thighs under the skirt belong to the coat region; shoes carry air and sock inside the upper; face, hair, collar, cuffs and hands are their own surfaces, the canopy is the cloth itself with both faces, the frame is shaft, handle, ribs and stretchers; breath leaves at nostril and mouth patches, the plume from a patch on the shoulders of the coat.", de: "1:50 thermisch: Die Mantelhülle ist als Verweis die äußere Schicht; die Luft des weiten Mantels und das Hemd liegen als geschachtelte Lofts darin an Rumpf, Schoß und Ärmeln (das Hemd endet an der Hüfte); Hose und ihre Luftschicht sind an den Unterschenkeln modelliert, wo sie frei liegen, die Oberschenkel unter dem Schoß gehören zur Mantelregion; die Schuhe tragen Luft und Socke im Schaft; Gesicht, Haar, Kragen, Manschetten und Hände sind eigene Flächen, das Schirmdach ist der Stoff selbst mit beiden Seiten, das Gestell sind Stock, Griff, Rippen und Streben; der Atem tritt an Nasen- und Mundfeld aus, die Warmluftfahne an einem Feld auf den Mantelschultern." },
  { en: "1:50 structure: the skeleton carries the body (skull with jaw, spine with its curves, ten rib hoops, sternum, clavicles, scapulae, pelvis, femur with neck and trochanter, patella, tibia, fibula, foot bones, humerus, ulna, radius, carpus, metacarpals and finger rays), joints at the rig joints and masses at the segment centres; the umbrella carries the wind through its real frame: the cloth loads the eight ribs, each propped by its stretcher from the runner, into notch and shaft and through the handle into the fist.", de: "1:50 Tragwerk: Das Skelett trägt den Körper (Schädel mit Unterkiefer, Wirbelsäule mit ihren Krümmungen, zehn Rippenbögen, Brustbein, Schlüsselbeine, Schulterblätter, Becken, Oberschenkelknochen mit Hals und Rollhügel, Kniescheibe, Schien- und Wadenbein, Fußknochen, Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerstrahlen), Gelenke an den Rig-Gelenken und Massen in den Segmentschwerpunkten; der Schirm trägt den Wind über sein echtes Gestell: der Stoff belastet die acht Rippen, jede vom Schieber aus durch ihre Strebe gestützt, in Krone und Stock und über den Griff in die Faust." },
];
