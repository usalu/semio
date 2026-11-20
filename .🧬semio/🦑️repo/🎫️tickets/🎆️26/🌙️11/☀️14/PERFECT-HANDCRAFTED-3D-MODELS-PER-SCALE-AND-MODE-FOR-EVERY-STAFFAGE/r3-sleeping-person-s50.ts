/** 🛌️ The 1:50 body of the round 3 sleeping person, every section stated by hand (smallest feature 7.5 mm): a woman of 1.70 m on her back in a long-sleeved pyjama top and shorts, the head with face, nose, lips and loose hair on a 40 x 80 cm pillow, the right arm on the duvet with the hand on the belly (four curled fingers and a thumb), the left arm alongside the body under the duvet, bare feet with heel, arch, malleoli and five toes out of the folded foot end; the 135 x 200 cm duvet as half-revolved mounds over chest, hips, thighs and shins with flat parts on the mattress, rolled edges over the mattress sides, side drapes that hang shorter where the body takes more cloth, and the foot end folded back over the shins with its own drapes and a rolled fold.
 * Rows are `[s, cv, cw, rx, ry, e]` in the frame of the bone (s along the bone, cv along its normal, cw lateral) or world rows `[x, y, z, rx, ry, e]` of the supine pose.
 * @see ./r3-sleeping-person-kit.ts
 * @see ./r3-sleeping-person-ref.ts */
import { BED } from "./r3-sleeping-person-ref.ts";
import { ACROSS_U, Kit, type Row, type WRow } from "./r3-sleeping-person-kit.ts";

export const T = { skin: 0.82, lip: 0.6, hair: 0.22, pyjama: 0.62, cuff: 0.5, duvet: 0.9, fold: 0.86, pillow: 0.95 } as const;
export const Y0 = BED.top;

/** 🗿️ The head from chin to vertex in the frame of `head` (u to the vertex, -v the face, w to her left): `[u, cv, rx, ry]`. */
export const HEAD: readonly (readonly [number, number, number, number])[] = [
  [-0.094, -0.05, 0.012, 0.01],
  [-0.088, -0.046, 0.026, 0.025],
  [-0.074, -0.032, 0.043, 0.05],
  [-0.052, -0.02, 0.054, 0.064],
  [-0.025, -0.01, 0.063, 0.077],
  [0, -0.004, 0.069, 0.087],
  [0.025, -0.001, 0.072, 0.091],
  [0.05, 0.002, 0.074, 0.093],
  [0.075, 0.004, 0.072, 0.088],
  [0.098, 0.006, 0.063, 0.074],
  [0.117, 0.007, 0.047, 0.053],
  [0.13, 0.007, 0.025, 0.028],
  [0.135, 0.007, 0.007, 0.008],
];
/** 🎭️ The face mask of the thermal region `face-skin`: the front of every head row up to the hairline, 46 mm deep. */
export const FACE: Row[] = HEAD.filter(([u]) => u <= 0.06).map(([u, cv, rx, ry]) => {
  const front = cv - ry;
  const d = Math.min(0.046, ry);
  return [u, front + d, 0, rx * 0.93, d, 2.2] as Row;
});

/** 👕️ The trunk in the pyjama top along `torso` (s from the lumbar joint towards C7; cv positive is down to the mattress). */
const TRUNK: Row[] = [
  [-0.02, 0.0005, 0, 0.14, 0.0925, 2.5],
  [0, 0.0005, 0, 0.14, 0.0925, 2.5],
  [0.08, 0.003, 0, 0.13, 0.09, 2.5],
  [0.16, -0.0045, 0, 0.14, 0.0975, 2.5],
  [0.24, -0.0145, 0, 0.15, 0.1075, 2.4],
  [0.31, -0.012, 0, 0.155, 0.105, 2.4],
  [0.37, -0.0045, 0, 0.166, 0.0975, 2.4],
  [0.405, -0.002, 0, 0.168, 0.094, 2.3],
  [0.432, 0.003, 0, 0.162, 0.088, 2.2],
  [0.452, 0.012, 0, 0.145, 0.076, 2.1],
  [0.468, 0.018, 0, 0.115, 0.066, 2.1],
  [0.482, 0.023, 0, 0.088, 0.055, 2.1],
  [0.497, 0.025, 0, 0.07, 0.05, 2.1],
];
/** 🩳️ The pelvis in the shorts along `pelvis` (s from the hip joints towards the waist). */
const PELVIS: Row[] = [
  [-0.15, 0.02, 0, 0.155, 0.068, 2.4],
  [-0.1, 0.017, 0, 0.17, 0.076, 2.5],
  [-0.03, 0.0105, 0, 0.18, 0.0825, 2.6],
  [0.04, 0.0055, 0, 0.17, 0.0875, 2.6],
  [0.1, 0, 0, 0.14, 0.0925, 2.5],
];
const THIGH: Row[] = [
  [0, 0.0, 0, 0.09, 0.085],
  [0.1, 0.0, 0, 0.09, 0.087, 2.2],
  [0.22, 0.003, 0, 0.078, 0.075, 2.2],
  [0.33, 0.006, 0, 0.063, 0.062],
  [0.4, 0.004, 0, 0.052, 0.052],
  [0.42, 0.002, 0, 0.05, 0.05],
];
const SHANK: Row[] = [
  [-0.02, 0.003, 0, 0.05, 0.05],
  [0.0, 0.0, 0, 0.051, 0.051],
  [0.09, -0.008, 0, 0.055, 0.057],
  [0.2, -0.005, 0, 0.048, 0.047],
  [0.3, -0.001, 0, 0.037, 0.036],
  [0.345, 0, 0, 0.034, 0.033],
];
const ANKLE: Row[] = [
  [0.33, 0, 0, 0.035, 0.034],
  [0.38, -0.004, 0, 0.03, 0.031],
  [0.41, -0.006, 0, 0.031, 0.034],
  [0.43, -0.01, 0, 0.032, 0.038],
];
/** 🦶️ The foot along `foot` (s from the ankle towards the ball, cv to the dorsum). */
const FOOT: Row[] = [
  [-0.056, -0.024, 0, 0.012, 0.012],
  [-0.05, -0.023, 0, 0.022, 0.025],
  [-0.03, -0.026, 0, 0.031, 0.046],
  [0, -0.0225, 0, 0.033, 0.0525],
  [0.03, -0.017, 0, 0.035, 0.049],
  [0.07, -0.011, 0, 0.04, 0.039],
  [0.11, -0.005, 0, 0.045, 0.029],
  [0.145, -0.003, 0, 0.047, 0.021],
  [0.162, -0.004, 0, 0.044, 0.016],
];
/** 🦶️ Toes from the ball: `[lateral offset towards the big toe side, radius at the base, radius at the tip, length]`. */
const TOES: readonly (readonly [number, number, number, number])[] = [
  [0.026, 0.0125, 0.011, 0.05],
  [0.005, 0.0092, 0.0082, 0.041],
  [-0.011, 0.0088, 0.008, 0.036],
  [-0.025, 0.0083, 0.0077, 0.031],
  [-0.037, 0.008, 0.0076, 0.026],
];
const SLEEVE: Row[] = [
  [-0.058, 0, 0, 0.018, 0.017],
  [-0.052, 0, 0, 0.034, 0.032],
  [-0.035, 0, 0, 0.047, 0.045],
  [0, 0, 0, 0.053, 0.052],
  [0.06, 0, 0, 0.051, 0.05],
  [0.15, 0, 0, 0.047, 0.045],
  [0.24, 0, 0, 0.043, 0.041],
  [0.3, 0, 0, 0.041, 0.038],
  [0.335, 0, 0, 0.04, 0.037],
  [0.352, 0, 0, 0.034, 0.031],
  [0.362, 0, 0, 0.02, 0.018],
];
const FORESLEEVE: Row[] = [
  [-0.048, 0, 0, 0.02, 0.018],
  [-0.04, 0, 0, 0.033, 0.03],
  [-0.025, 0, 0, 0.04, 0.037],
  [0.04, 0, 0, 0.041, 0.035],
  [0.12, 0, 0, 0.036, 0.029],
  [0.19, 0, 0, 0.031, 0.025],
  [0.205, 0, 0, 0.03, 0.024],
];
const CUFF: Row[] = [
  [0.198, 0, 0, 0.032, 0.026],
  [0.205, 0, 0, 0.034, 0.028],
  [0.238, 0, 0, 0.033, 0.027],
  [0.245, 0, 0, 0.029, 0.023],
];
/** ✋️ Palm and back of the hand along `hand` (palm on -v); `t` is +1 when the thumb lies on +w. */
const palm = (t: number): Row[] => [
  [-0.03, 0, 0, 0.024, 0.018],
  [0, 0, 0, 0.027, 0.017],
  [0.03, -0.001, 0.002 * t, 0.035, 0.0155],
  [0.065, -0.002, 0, 0.04, 0.0135],
  [0.095, -0.003, -0.002 * t, 0.04, 0.012],
  [0.106, -0.003, -0.002 * t, 0.036, 0.011],
];
/** 🖐️ Fingers from the knuckles: `[lateral towards the thumb, proximal length, distal length, radius]`. */
const FINGERS: readonly (readonly [number, number, number, number])[] = [
  [0.024, 0.042, 0.038, 0.0092],
  [0.006, 0.047, 0.042, 0.0094],
  [-0.012, 0.044, 0.04, 0.0089],
  [-0.028, 0.034, 0.03, 0.008],
];
const PYJAMA = "cotton-underwear";

/** 🧍️ The woman's arm on one side: sleeve, forearm sleeve, cuff, palm, thenar, fingers (curled by `curl` degrees at the two joints) and thumb. */
function arm(k: Kit, side: "l" | "r", curl: readonly [number, number]): void {
  const t = side === "r" ? 1 : -1;
  k.loft(`sleeve-${side}`, `arm-${side}`, PYJAMA, T.pyjama, SLEEVE, "top");
  k.loft(`foresleeve-${side}`, `forearm-${side}`, PYJAMA, T.pyjama, FORESLEEVE, "top");
  k.loft(`cuff-${side}`, `forearm-${side}`, PYJAMA, T.cuff, CUFF, "top");
  const hand = `hand-${side}`;
  k.loft(`palm-${side}`, hand, "skin", T.skin, palm(t), "hand-skin");
  k.ell(`thenar-${side}`, hand, "skin", T.skin, [0.032, -0.007, 0.022 * t], [0.03, 0.012, 0.016], "hand-skin");
  const rad = (d: number): number => (d * Math.PI) / 180;
  FINGERS.forEach(([cw, l1, l2, r], i) => {
    const p0 = [0.095, -0.004, cw * t] as const;
    const p1 = [p0[0] + l1 * Math.cos(rad(curl[0])), p0[1] - l1 * Math.sin(rad(curl[0])), p0[2]] as const;
    const p2 = [p1[0] + l2 * Math.cos(rad(curl[0] + curl[1])), p1[1] - l2 * Math.sin(rad(curl[0] + curl[1])), p1[2]] as const;
    k.cap(`finger-${i + 1}-${side}`, hand, "skin", T.skin, p0, p1, r, r * 0.92, "hand-skin");
    k.cap(`finger-${i + 1}-tip-${side}`, hand, "skin", T.skin, p1, p2, r * 0.92, r * 0.8, "hand-skin");
  });
  k.cap(`thumb-${side}`, hand, "skin", T.skin, [0.012, -0.009, 0.026 * t], [0.046, -0.016, 0.047 * t], 0.0122, 0.0105, "hand-skin");
  k.cap(`thumb-tip-${side}`, hand, "skin", T.skin, [0.046, -0.016, 0.047 * t], [0.074, -0.02, 0.056 * t], 0.0105, 0.0085, "hand-skin");
}

/** 🦵️ The bare leg below the duvet's fold on one side: ankle, malleoli, foot and five toes (big toe on the medial side). */
function foot(k: Kit, side: "l" | "r"): void {
  const m = side === "l" ? -1 : 1;
  k.loft(`ankle-${side}`, `shank-${side}`, "skin", T.skin, ANKLE, "feet-skin");
  k.loft(`foot-${side}`, `foot-${side}`, "skin", T.skin, FOOT, "feet-skin");
  k.ell(`malleolus-out-${side}`, `foot-${side}`, "skin", T.skin, [-0.012, -0.006, -0.031 * m], [0.013, 0.015, 0.009], "feet-skin");
  k.ell(`malleolus-in-${side}`, `foot-${side}`, "skin", T.skin, [0.0, 0.004, 0.029 * m], [0.013, 0.015, 0.009], "feet-skin");
  TOES.forEach(([cw, r0, r1, l], i) => k.cap(`toe-${i + 1}-${side}`, `toe-${side}`, "skin", T.skin, [-0.004, -0.004, cw * m], [l, -0.012, cw * m * 0.97], r0, r1, "feet-skin"));
}

/** 🛏️ Rows of a flat solid along world x from `x0` to `x1` whose ends are rounded over `round` metres. */
export function along(x0: number, x1: number, y: number, z: number, rx: number, ry: number, e: number, round: number, inner: readonly (readonly [number, number, number?])[] = []): WRow[] {
  const ends = [0, 0.35, 0.75, 1].map((t) => [t, Math.sqrt(1 - (1 - t) ** 2)] as const);
  const head = ends.map(([t, k]) => [x0 + round * t, y, z, rx, Math.max(ry * k, 0.004), e] as WRow);
  const tail = [...ends].reverse().map(([t, k]) => [x1 - round * t, y, z, rx, Math.max(ry * k, 0.004), e] as WRow);
  return [...head, ...inner.map(([x, yy, ryy]) => [x, yy, z, rx, ryy ?? ry, e] as WRow), ...tail];
}

/** 🪂️ A side drape hanging from the mattress edge down to `bottom(x)` (world rows across x, lateral half-thickness `t`). */
export function drape(x0: number, x1: number, z: number, t: number, length: readonly (readonly [number, number])[], round: number, e = 4): WRow[] {
  const at = (x: number): number => {
    for (let i = 1; i < length.length; i++) if (x <= length[i]![0]) {
      const [xa, la] = length[i - 1]!;
      const [xb, lb] = length[i]!;
      return la + ((x - xa) / (xb - xa)) * (lb - la);
    }
    return length[length.length - 1]![1];
  };
  const xs = [x0, x0 + round * 0.4, x0 + round, ...length.map(([x]) => x).filter((x) => x > x0 + round && x < x1 - round), x1 - round, x1 - round * 0.4, x1];
  return xs.map((x, i) => {
    const l = at(x);
    const k = i === 0 || i === xs.length - 1 ? 0.35 : i === 1 || i === xs.length - 2 ? 0.8 : 1;
    return [x, Y0 + 0.004 - l / 2, z, Math.max(t * k, 0.004), l / 2, e] as WRow;
  });
}

/** 🛌️ Lateral centre of the trunk hill: the duvet lies over her left arm and stops at the right arm, which rests beside it on the cloth. */
export const MOUND_Z = 0.04;
/** 🏔️ The hills of the duvet as it lies on the bed in the supine pose, half revolves with their flat face on the mattress, `[world x, half-width]`: each fades over 0.2 m inside its neighbour so that the surface runs on without a step. They stay with the duvet on the bed; the cloth that goes with the body is `CLOTH`. */
export const HILLS = {
  trunk: [[0.594, 0], [0.598, 0.14], [0.603, 0.19], [0.612, 0.215], [0.63, 0.24], [0.66, 0.257], [0.72, 0.262], [0.78, 0.258], [0.84, 0.248], [0.9, 0.234], [0.95, 0.224], [1.0, 0.215], [1.05, 0.205], [1.1, 0.19], [1.15, 0.165], [1.19, 0.125], [1.21, 0.07], [1.215, 0]],
  thighs: [[1.0, 0], [1.004, 0.07], [1.012, 0.13], [1.03, 0.2], [1.05, 0.245], [1.075, 0.272], [1.1, 0.28], [1.2, 0.276], [1.27, 0.262], [1.33, 0.245], [1.4, 0.225], [1.44, 0.215], [1.5, 0.208], [1.51, 0]],
  shins: [[1.44, 0], [1.445, 0.19], [1.46, 0.212], [1.5, 0.208], [1.6, 0.207], [1.7, 0.195], [1.76, 0.188], [1.78, 0.18], [1.79, 0.165], [1.797, 0.13], [1.8, 0]],
  fold: [[1.212, 0], [1.215, 0.17], [1.221, 0.24], [1.232, 0.278], [1.25, 0.29], [1.27, 0.288], [1.33, 0.273], [1.4, 0.256], [1.47, 0.246], [1.52, 0.241], [1.6, 0.239], [1.7, 0.229], [1.76, 0.221], [1.78, 0.21], [1.792, 0.19], [1.799, 0.15], [1.802, 0]],
} as const;
export const HILL_LATERAL = { trunk: 1, thighs: 0.76, shins: 0.76, fold: 0.8 } as const;
/** 🧣️ The cloth that goes with the body, one closed loft around each segment (rows in the frame of the bone: s along it, cv to its normal, cw lateral, half-width, half-depth). In the supine pose every one lies with its underside on the mattress inside the hills; when she turns or draws up her knees they come out of the hills as the duvet over trunk, hips, thighs and shins, with long overlaps at waist, hip and knee so the cloth stays closed. */
export const CLOTH: Record<"trunk" | "hips" | "thighs" | "shins", { bone: string; region: string; rows: Row[] }> = {
  trunk: { bone: "torso", region: "top", rows: [[0.358, -0.02, 0.035, 0.03, 0.02], [0.354, -0.02, 0.035, 0.12, 0.07], [0.34, -0.025, 0.035, 0.18, 0.105], [0.31, -0.027, 0.035, 0.205, 0.12], [0.24, -0.027, 0.035, 0.205, 0.12], [0.16, -0.02, 0.035, 0.2, 0.113], [0.08, -0.012, 0.035, 0.195, 0.105], [0, -0.008, 0.035, 0.195, 0.1], [-0.06, -0.006, 0.03, 0.19, 0.098], [-0.1, -0.006, 0.02, 0.15, 0.08], [-0.12, -0.006, 0.02, 0.06, 0.03]] },
  hips: { bone: "pelvis", region: "bottom", rows: [[0.13, -0.006, 0, 0.06, 0.03], [0.11, -0.006, 0, 0.15, 0.08], [0.06, -0.008, 0, 0.195, 0.1], [0, -0.008, 0, 0.2, 0.1], [-0.08, -0.004, 0, 0.2, 0.096], [-0.14, 0, 0, 0.19, 0.09], [-0.17, 0, 0, 0.12, 0.06], [-0.185, 0, 0, 0.04, 0.02]] },
  thighs: { bone: "thigh-r", region: "bottom", rows: [[-0.075, 0.002, 0.087, 0.04, 0.02], [-0.06, 0.002, 0.087, 0.12, 0.06], [-0.03, 0.004, 0.087, 0.18, 0.09], [0, 0.005, 0.087, 0.2, 0.098], [0.1, 0.011, 0.087, 0.2, 0.1], [0.2, 0.011, 0.087, 0.198, 0.095], [0.3, 0.008, 0.087, 0.19, 0.088], [0.4, 0.0065, 0.087, 0.175, 0.082], [0.44, 0.006, 0.087, 0.13, 0.065], [0.458, 0.006, 0.087, 0.04, 0.02]] },
  shins: { bone: "shank-r", region: "duvet-edge", rows: [[-0.07, 0.002, 0.087, 0.04, 0.02], [-0.055, 0.003, 0.087, 0.12, 0.055], [-0.025, 0.004, 0.087, 0.16, 0.074], [0, 0.004, 0.087, 0.17, 0.08], [0.1, 0.009, 0.087, 0.165, 0.078], [0.2, 0.0086, 0.087, 0.155, 0.07], [0.3, 0.008, 0.087, 0.15, 0.062], [0.335, 0.006, 0.087, 0.12, 0.05], [0.35, 0.004, 0.087, 0.04, 0.02]] },
};
/** 🏔️ The duvet beside the body as hand-stated sections across the bed: `[distance from the centre line, height above the mattress]`, falling from the flank of the mounds to the flat part on the mattress. */
export const SLOPE: Record<"l" | "r", readonly (readonly [number, number])[]> = {
  l: [[0.165, 0.1], [0.2, 0.094], [0.24, 0.08], [0.28, 0.062], [0.32, 0.047], [0.37, 0.038], [0.42, 0.035], [0.455, 0.035]],
  r: [[0.165, 0.085], [0.2, 0.076], [0.24, 0.062], [0.28, 0.048], [0.32, 0.041], [0.37, 0.036], [0.42, 0.035], [0.455, 0.035]],
};
/** 🛏️ World rows of the duvet beside the body from `xa` to `xb` on one side, `lift` thicker (the folded part), sections across the bed. */
export function slope(xa: number, xb: number, side: "l" | "r", lift = 0, pick: (i: number) => boolean = () => true): WRow[] {
  const s = side === "l" ? 1 : -1;
  const rows = SLOPE[side].filter((_, i) => pick(i)).map(([z, h]) => [(xa + xb) / 2, Y0 + (h + lift) / 2, s * z, (xb - xa) / 2, (h + lift) / 2, 8] as WRow);
  return s > 0 ? rows : rows.reverse();
}
const ACROSS = { along: [0, 0, 1], xHint: [1, 0, 0] } as const;

/** 🛏️ The 1:50 body list. */
export function body50(k: Kit): void {
  k.loft("head", "head", "skin", T.skin, HEAD.map(([u, cv, rx, ry]) => [u, cv, 0, rx, ry, 2.2] as Row), "face-skin");
  k.ell("brow", "head", "skin", T.skin, [0.041, -0.083, 0], [0.011, 0.011, 0.052], "face-skin");
  k.loft("nose", "head", "skin", T.skin, [[0.036, -0.087, 0, 0.008, 0.006], [0.022, -0.094, 0, 0.0085, 0.008], [0.006, -0.101, 0, 0.0095, 0.01], [-0.006, -0.106, 0, 0.012, 0.011], [-0.014, -0.106, 0, 0.0135, 0.0105], [-0.021, -0.099, 0, 0.012, 0.007], [-0.025, -0.092, 0, 0.008, 0.004]], "face-skin");
  for (const [side, w] of [["l", 1], ["r", -1]] as const) {
    k.ell(`ala-${side}`, "head", "skin", T.skin, [-0.016, -0.097, 0.0135 * w], [0.008, 0.008, 0.0075], "face-skin");
    k.ell(`eyelid-${side}`, "head", "skin", T.skin, [0.022, -0.0815, 0.032 * w], [0.0085, 0.009, 0.0155], "face-skin");
    k.ell(`cheek-${side}`, "head", "skin", T.skin, [-0.006, -0.071, 0.044 * w], [0.02, 0.016, 0.022], "face-skin");
  }
  k.ell("lips", "head", "skin", T.lip, [-0.0395, -0.0885, 0], [0.0055, 0.0085, 0.022], "face-skin");
  k.ell("lips-lower", "head", "skin", T.lip, [-0.05, -0.0865, 0], [0.0062, 0.0085, 0.019], "face-skin");
  k.ell("chin", "head", "skin", T.skin, [-0.08, -0.066, 0], [0.015, 0.015, 0.024], "face-skin");
  k.loft("hair", "head", "hair", T.hair, [[-0.03, 0.04, 0, 0.068, 0.058], [0, 0.028, 0, 0.078, 0.075], [0.025, 0.018, 0, 0.081, 0.088], [0.05, 0.008, 0, 0.083, 0.1], [0.075, 0.004, 0, 0.081, 0.097], [0.098, 0.006, 0, 0.072, 0.083], [0.117, 0.007, 0, 0.056, 0.063], [0.132, 0.007, 0, 0.034, 0.04], [0.142, 0.007, 0, 0.012, 0.014]], "hair", { e: 2.2 });
  k.ell("hair-spread", "head", "hair", T.hair, [0.03, 0.08, 0], [0.1, 0.03, 0.105], "hair");
  k.loft("neck", "neck", "skin", T.skin, [[-0.03, 0, 0, 0.06, 0.056], [0, 0, 0, 0.057, 0.053], [0.04, 0.002, 0, 0.053, 0.05], [0.08, 0.006, 0, 0.053, 0.051], [0.11, 0.012, 0, 0.056, 0.056]], "face-skin");
  k.loft("trunk", "torso", PYJAMA, T.pyjama, TRUNK, "top");
  k.loft("collar", "torso", PYJAMA, T.cuff, [[0.468, 0.022, 0, 0.083, 0.058], [0.474, 0.022, 0, 0.083, 0.06], [0.5, 0.023, 0, 0.071, 0.056], [0.506, 0.023, 0, 0.066, 0.052]], "top");
  k.loft("shorts", "pelvis", PYJAMA, T.pyjama, PELVIS, "bottom");
  for (const side of ["l", "r"] as const) {
    k.loft(`thigh-${side}`, `thigh-${side}`, "skin", T.skin, THIGH, "bottom");
    k.loft(`shank-${side}`, `shank-${side}`, "skin", T.skin, SHANK, "bottom");
    foot(k, side);
  }
  arm(k, "r", [18, 40]);
  arm(k, "l", [12, 30]);
  k.wloft("pillow", "pillow", "duvet-cover", T.pillow, ([[-0.4, 0.012, 0.17], [-0.385, 0.035, 0.19], [-0.35, 0.055, 0.198], [-0.28, 0.064, 0.2], [-0.18, 0.058, 0.2], [-0.1, 0.042, 0.2], [0, 0.031, 0.2]] as const).flatMap(([z, h, rx], i, a) => (i === a.length - 1 ? [[z, h, rx]] : [[z, h, rx], [-z, h, rx]])).sort((p, q) => p[0]! - q[0]!).map(([z, h, rx]) => [0.27, Y0 + h!, z!, rx!, h!, 2] as WRow), "pillow", { along: [0, 0, 1], xHint: [1, 0, 0] });
    k.mound("duvet-trunk", "duvet", "duvet-cover", T.duvet, HILLS.trunk, Y0, MOUND_Z, HILL_LATERAL.trunk, "top");
  k.mound("duvet-thighs", "duvet", "duvet-cover", T.duvet, HILLS.thighs, Y0, 0, HILL_LATERAL.thighs, "bottom");
  k.mound("duvet-shins", "duvet", "duvet-cover", T.duvet, HILLS.shins, Y0, 0, HILL_LATERAL.shins, "duvet-edge");
  k.mound("fold-legs", "duvet", "duvet-cover", T.fold, HILLS.fold, Y0 + 0.004, 0, HILL_LATERAL.fold, "duvet-edge");
  for (const [part, c] of Object.entries(CLOTH)) k.loft(`cloth-${part}`, c.bone, "duvet-cover", T.duvet, c.rows, c.region, { e: 2.3 });
  const { x0, fold, x1, flat } = BED.duvet;
  const skirt = [[0.6, 0.07], [0.72, 0.052], [0.9, 0.068], [1.05, 0.065], [1.2, 0.085], [1.45, 0.105], [1.6, 0.11], [1.8, 0.12]] as const;
  for (const [side, s] of [["l", 1], ["r", -1]] as const) {
    k.wloft(`flat-${side}`, "duvet", "duvet-cover", T.duvet, slope(x0, x1, side), "duvet-edge", ACROSS);
    k.wloft(`edge-${side}`, "duvet", "duvet-cover", T.duvet, along(x0, x1, Y0 + flat / 2, s * (BED.mattressHalf + flat / 2), flat / 2, flat / 2, 2, 0.012).map(([x, y, z, , ry, e]) => [x, y, z, ry, ry, e] as WRow), "duvet-edge");
    k.wloft(`drape-${side}`, "duvet", "duvet-cover", T.duvet, drape(x0, x1, s * (BED.mattressHalf + flat / 2), flat / 2, skirt, 0.012).map(([x, y, z, rx, ry, e]) => [x, y + 0.006, z, rx, ry + 0.006, e] as WRow), "duvet-edge");
    k.wloft(`fold-flat-${side}`, "duvet", "duvet-cover", T.fold, slope(fold, x1 + 0.002, side, flat), "duvet-edge", ACROSS);
    k.wloft(`fold-edge-${side}`, "duvet", "duvet-cover", T.fold, along(fold, x1 + 0.002, Y0 + flat, s * (BED.mattressHalf + flat), flat, flat, 2, 0.02).map(([x, y, z, , ry, e]) => [x, y, z, ry, ry, e] as WRow), "duvet-edge");
    k.wloft(`fold-drape-${side}`, "duvet", "duvet-cover", T.fold, drape(fold, x1 + 0.002, s * (BED.mattressHalf + 1.5 * flat), flat / 2, [[fold, 0.09], [1.5, 0.1], [1.8, 0.11]], 0.014).map(([x, y, z, rx, ry, e]) => [x, y + 0.016, z, rx, ry + 0.016, e] as WRow), "duvet-edge");
  }
}
export { ACROSS_U };
