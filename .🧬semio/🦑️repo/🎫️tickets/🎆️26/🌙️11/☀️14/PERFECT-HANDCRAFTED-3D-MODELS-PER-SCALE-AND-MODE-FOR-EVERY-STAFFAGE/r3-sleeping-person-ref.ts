/** 📏️ Reference numbers of the round 3 sleeping person (a 1.70 m, 64 kg woman lying supine on a soft mattress, 40 x 80 cm pillow, 135 x 200 cm duvet with the foot end folded back), in metres and degrees, world frame of the supine pose: x along the bed from the head end, y up from the floor, z to her left.
 * Sources: Winter 2009 and the round 3 brief (segment lengths as shares of stature), de Leva 1996 (female centres of mass), ANSUR II women (girths, breadths, depths; my knowledge), EN 1334 (mattress 0.90 x 2.00 m), DIN EN 12934 / trade sizes (duvet 135 x 200, pillow 40 x 80), the catalog single-bed (mattress top 0.46 m under a sleeper, head end of the mattress at x 0.06).
 * @see ./r3-humans-sleeping-person.md */
export type V3 = readonly [number, number, number];

const H = 1.7;
/** 🧍️ Body: lengths of the rig, joint heights of the lying figure and the solved arm and foot targets. */
export const WOMAN = {
  H,
  hipY: 0.553,
  pelvis: 0.0955,
  torso: 0.4825,
  neck: 0.105,
  head: 0.133,
  neckRiseDeg: 25,
  headRiseDeg: 10,
  shoulderOnTorso: 0.4013,
  shoulderBelowLine: 0.013,
  shoulderZ: 0.158,
  hipZ: 0.087,
  thigh: 0.4,
  shank: 0.4185,
  foot: 0.145,
  toe: 0.062,
  hand: 0.1816,
  thighDeg: 177.5,
  kneeDeg: -1.7,
  footDeg: 60,
  ankle: [1.866, 0.505, 0.105] as V3,
  toeTip: [1.98, 0.668, 0.14] as V3,
  arms: {
    lElbow: [0.861, 0.507, 0.196] as V3,
    lWrist: [1.106, 0.492, 0.222] as V3,
    lTip: [1.284, 0.484, 0.206] as V3,
    lPalm: [0, 0, -1] as V3,
    rElbow: [0.85, 0.58, -0.255] as V3,
    rWrist: [0.98, 0.691, -0.075] as V3,
    rTip: [1.08, 0.682, 0.076] as V3,
    rPalm: [0.08, -0.997, 0] as V3,
  },
} as const;

/** 🛏️ Bedding: mattress datum, pillow and duvet. */
export const BED = {
  top: 0.46,
  mattressHalf: 0.45,
  pillow: { x0: 0.07, length: 0.4, halfWidth: 0.4, loft: 0.13, underHead: 0.06, areaM2: 0.7 },
  duvet: { x0: 0.6, fold: 1.22, x1: 1.8, width: 1.35, flat: 0.035, edgeAreaM2: 2.95 },
} as const;

/** ⚓️ Anchors in world coordinates of the supine pose and the bone that carries them. */
export const ANCHORS: Record<string, { bone: string; world: V3 }> = {
  "head-pillow": { bone: "head", world: [0.314, 0.522, 0] },
  "shoulders-mattress": { bone: "torso", world: [0.64, 0.46, 0] },
  "back-mattress": { bone: "torso", world: [0.82, 0.46, 0] },
  "pelvis-mattress": { bone: "pelvis", world: [1.06, 0.46, 0] },
  "calf-mattress": { bone: "shank-r", world: [1.62, 0.46, -0.1] },
  "heel-mattress": { bone: "foot-r", world: [1.88, 0.46, -0.105] },
  mouth: { bone: "head", world: [0.436, 0.678, 0] },
  nostrils: { bone: "head", world: [0.415, 0.687, 0] },
  "chest-plume": { bone: "torso", world: [0.7, 0.713, 0.028] },
};
