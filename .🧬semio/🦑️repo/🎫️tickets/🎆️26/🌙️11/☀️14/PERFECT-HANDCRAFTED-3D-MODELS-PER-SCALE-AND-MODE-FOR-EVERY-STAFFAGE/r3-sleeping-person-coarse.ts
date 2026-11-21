/** 🛌️ The 1:100, 1:200 and 1:500 bodies of the round 3 sleeping person, each stated anew for its smallest feature (15, 30, 75 mm): fewer and merged sections, the hidden body under the duvet left out, the face reduced to head, nose and hair, the hand to a mitten, the duvet to fewer mounds and, from 1:200, to one flat-and-drape profile per side; 1:500 keeps head, hair, shoulders, the arm on the duvet, the feet, the pillow, two mounds and one generalised duvet slab.
 * @see ./r3-sleeping-person-s50.ts */
import { BED } from "./r3-sleeping-person-ref.ts";
import { HEAD, MOUND_Z, T, Y0, drape, slope } from "./r3-sleeping-person-s50.ts";
import { Kit, W, type Row, type WRow } from "./r3-sleeping-person-kit.ts";

const PYJAMA = "cotton-underwear";
const { x0, fold, x1, flat } = BED.duvet;
const SKIRT = [[0.6, 0.07], [0.72, 0.052], [0.9, 0.068], [1.05, 0.065], [1.2, 0.085], [1.45, 0.105], [1.6, 0.11], [1.8, 0.12]] as const;
const TRUNK_MOUND = [[0.594, 0], [0.6, 0.165], [0.612, 0.205], [0.63, 0.227], [0.66, 0.243], [0.72, 0.25], [0.84, 0.233], [0.95, 0.218], [1.1, 0.214], [1.15, 0.208], [1.17, 0]] as const;

/** 🛏️ The flat parts and drapes of one side as one profile pushed along the bed (1:200): flat, rolled edge and drape, `extra` thicker for the folded part. */
function profileSide(k: Kit, id: string, side: number, from: number, to: number, t: number, drop: number, tone: number): void {
  const w = (z: number): number => side * z;
  const pts: [number, number][] = [[0, w(0.17)], [t + 0.05, w(0.17)], [t + 0.03, w(0.25)], [t, w(0.34)], [t, w(0.45)], [t * 0.8, w(0.45 + t * 0.55)], [t * 0.35, w(0.45 + t)], [-drop, w(0.45 + t)], [-drop, w(0.45)], [0, w(0.45)]];
  k.ext(id, "duvet", "duvet-cover", tone, "yz", side > 0 ? pts : [...pts].reverse(), from - x0, to - x0, "duvet-edge");
}

/** 🛏️ 1:100: head with nose, hair cap and hair on the pillow, neck, the part of the trunk outside the duvet, right arm with mitten hand and thumb, bare feet with merged toes, pillow, four mounds, flats and drapes, fold. */
export function body100(k: Kit): void {
  k.loft("head", "head", "skin", T.skin, HEAD.filter((_, i) => i % 2 === 0 || i === 12).map(([u, cv, rx, ry]) => [u, cv, 0, rx, ry, 2.2] as Row), "face-skin");
  k.ell("nose", "head", "skin", T.skin, [0.005, -0.095, 0], [0.026, 0.0145, 0.0135], "face-skin");
  k.loft("hair", "head", "hair", T.hair, [[-0.03, 0.04, 0, 0.068, 0.058], [0.025, 0.018, 0, 0.081, 0.088], [0.075, 0.004, 0, 0.081, 0.097], [0.117, 0.007, 0, 0.056, 0.063], [0.142, 0.007, 0, 0.012, 0.014]], "hair", { e: 2.2 });
  k.ell("hair-spread", "head", "hair", T.hair, [0.03, 0.08, 0], [0.1, 0.03, 0.105], "hair");
  k.loft("neck", "neck", "skin", T.skin, [[-0.03, 0, 0, 0.06, 0.056], [0.05, 0.003, 0, 0.053, 0.05], [0.11, 0.012, 0, 0.056, 0.056]], "face-skin");
  k.loft("trunk", "torso", PYJAMA, T.pyjama, [[0.3, -0.012, 0, 0.155, 0.105, 2.4], [0.37, -0.0045, 0, 0.19, 0.0975, 2.4], [0.42, 0.004, 0, 0.19, 0.09, 2.2], [0.46, 0.014, 0, 0.13, 0.072, 2.1], [0.497, 0.025, 0, 0.07, 0.05, 2.1]], "top");
  k.loft("sleeve-r", "arm-r", PYJAMA, T.pyjama, [[-0.035, 0, 0, 0.05, 0.048], [0.03, 0, 0, 0.052, 0.051], [0.2, 0, 0, 0.045, 0.043], [0.335, 0, 0, 0.04, 0.037]], "top");
  k.loft("foresleeve-r", "forearm-r", PYJAMA, T.pyjama, [[-0.025, 0, 0, 0.04, 0.037], [0.06, 0, 0, 0.04, 0.034], [0.2, 0, 0, 0.032, 0.026], [0.245, 0, 0, 0.032, 0.026]], "top");
  k.loft("hand-r", "hand-r", "skin", T.skin, [[-0.03, 0, 0, 0.026, 0.018], [0.04, -0.001, 0, 0.038, 0.015], [0.1, -0.004, 0, 0.039, 0.012], [0.14, -0.016, -0.002, 0.036, 0.01], [0.168, -0.036, -0.002, 0.03, 0.009]], "hand-skin");
  k.cap("thumb-r", "hand-r", "skin", T.skin, [0.012, -0.009, 0.026], [0.072, -0.02, 0.055], 0.012, 0.0085, "hand-skin");
  for (const side of ["l", "r"] as const) {
    const m = side === "l" ? -1 : 1;
    k.loft(`ankle-${side}`, `shank-${side}`, "skin", T.skin, [[0.33, 0, 0, 0.035, 0.034], [0.43, -0.01, 0, 0.032, 0.038]], "feet-skin");
    k.loft(`foot-${side}`, `foot-${side}`, "skin", T.skin, [[-0.054, -0.024, 0, 0.016, 0.018], [-0.03, -0.026, 0, 0.031, 0.046], [0.01, -0.02, 0, 0.034, 0.052], [0.07, -0.011, 0, 0.04, 0.039], [0.145, -0.003, 0, 0.047, 0.021], [0.16, -0.004, 0, 0.044, 0.016]], "feet-skin");
    k.loft(`toes-${side}`, `toe-${side}`, "skin", T.skin, [[-0.005, -0.004, 0.0, 0.046, 0.012, 2.6], [0.025, -0.008, 0.006 * m, 0.04, 0.0105, 2.6], [0.045, -0.011, 0.014 * m, 0.022, 0.009]], "feet-skin");
  }
  k.wloft("pillow", "pillow", "duvet-cover", T.pillow, ([[-0.4, 0.012, 0.17], [-0.37, 0.05, 0.196], [-0.26, 0.064, 0.2], [-0.12, 0.045, 0.2], [0, 0.031, 0.2], [0.12, 0.045, 0.2], [0.26, 0.064, 0.2], [0.37, 0.05, 0.196], [0.4, 0.012, 0.17]] as const).map(([z, h, rx]) => [0.27, Y0 + h, z, rx, h, 2] as WRow), "pillow", { along: [0, 0, 1], xHint: [1, 0, 0] });
  k.mound("duvet-trunk", "torso", "duvet-cover", T.duvet, TRUNK_MOUND, Y0, MOUND_Z, 0.98, "top");
  k.mound("duvet-thighs", "thigh-r", "duvet-cover", T.duvet, [[1.12, 0], [1.125, 0.22], [1.14, 0.252], [1.27, 0.255], [1.4, 0.222], [1.5, 0.2], [1.51, 0]], Y0, 0, 0.8, "bottom");
  k.mound("fold-legs", "shank-r", "duvet-cover", T.fold, [[1.212, 0], [1.218, 0.25], [1.235, 0.29], [1.3, 0.282], [1.4, 0.254], [1.52, 0.235], [1.7, 0.227], [1.78, 0.21], [1.795, 0.17], [1.802, 0]], Y0 + 0.004, 0, 0.8, "duvet-edge");
  for (const [side, s] of [["l", 1], ["r", -1]] as const) {
    k.wloft(`flat-${side}`, "duvet", "duvet-cover", T.duvet, slope(x0, fold + 0.02, side, 0, (i) => i % 2 === 0 || i === 7), "duvet-edge", { along: [0, 0, 1], xHint: [1, 0, 0] });
    k.wloft(`drape-${side}`, "duvet", "duvet-cover", T.duvet, drape(x0, fold + 0.02, s * (BED.mattressHalf + flat / 2), flat / 2, SKIRT.filter(([x]) => x <= 1.25).concat([[fold + 0.02, 0.09]]), 0.012).map(([x, y, z, rx, ry, e]) => [x, y + 0.009, z, rx, ry + 0.009, e] as WRow), "duvet-edge");
    k.wloft(`fold-flat-${side}`, "duvet", "duvet-cover", T.fold, slope(fold, x1 + 0.002, side, flat, (i) => i % 2 === 0 || i === 7), "duvet-edge", { along: [0, 0, 1], xHint: [1, 0, 0] });
    k.wloft(`fold-drape-${side}`, "duvet", "duvet-cover", T.fold, drape(fold, x1 + 0.002, s * (BED.mattressHalf + flat), flat, [[fold, 0.11], [1.5, 0.115], [1.8, 0.125]], 0.016).map(([x, y, z, rx, ry, e]) => [x, y + 0.018, z, rx, ry + 0.018, e] as WRow), "duvet-edge");
  }
}

/** 🛏️ 1:200: head, hair, neck, shoulders, the right arm in three segments with a mitten, feet with toes, pillow, three mounds (the fold included in the legs), one flat-and-drape profile per side and its folded part. */
export function body200(k: Kit): void {
  k.loft("head", "head", "skin", T.skin, [[-0.094, -0.05, 0, 0.015, 0.015], [-0.074, -0.032, 0, 0.043, 0.05], [-0.025, -0.01, 0, 0.063, 0.077], [0.025, -0.001, 0, 0.072, 0.091], [0.075, 0.004, 0, 0.072, 0.088], [0.117, 0.007, 0, 0.047, 0.053], [0.135, 0.007, 0, 0.016, 0.016]], "face-skin", { e: 2.2 });
  k.loft("hair", "head", "hair", T.hair, [[-0.035, 0.06, 0, 0.09, 0.07], [0.03, 0.03, 0, 0.1, 0.1], [0.1, 0.01, 0, 0.08, 0.085], [0.145, 0.007, 0, 0.02, 0.02]], "hair");
  k.cap("neck", "neck", "skin", T.skin, [-0.02, 0, 0], [0.1, 0.01, 0], 0.056, 0.056, "face-skin");
  k.loft("trunk", "torso", PYJAMA, T.pyjama, [[0.3, -0.012, 0, 0.155, 0.105, 2.4], [0.4, 0.0, 0, 0.19, 0.092, 2.2], [0.497, 0.025, 0, 0.07, 0.05, 2]], "top");
  k.cap("sleeve-r", "arm-r", PYJAMA, T.pyjama, [-0.02, 0, 0], [0.33, 0, 0], 0.05, 0.04, "top");
  k.cap("foresleeve-r", "forearm-r", PYJAMA, T.pyjama, [-0.01, 0, 0], [0.24, 0, 0], 0.039, 0.03, "top");
  k.ell("hand-r", "hand-r", "skin", T.skin, [0.075, -0.008, 0.008], [0.095, 0.016, 0.045], "hand-skin");
  for (const side of ["l", "r"] as const) k.loft(`ankle-${side}`, `shank-${side}`, "skin", T.skin, [[0.32, 0, 0, 0.036, 0.035], [0.44, -0.01, 0, 0.033, 0.038]], "feet-skin");
  for (const side of ["l", "r"] as const) k.loft(`foot-${side}`, `foot-${side}`, "skin", T.skin, [[-0.054, -0.024, 0, 0.018, 0.02], [-0.02, -0.025, 0, 0.033, 0.05], [0.07, -0.011, 0, 0.04, 0.039], [0.15, -0.004, 0, 0.046, 0.02], [0.2, -0.012, 0, 0.032, 0.015]], "feet-skin");
  k.wloft("pillow", "pillow", "duvet-cover", T.pillow, ([[-0.4, 0.015, 0.17], [-0.3, 0.064, 0.2], [0, 0.034, 0.2], [0.3, 0.064, 0.2], [0.4, 0.015, 0.17]] as const).map(([z, h, rx]) => [0.27, Y0 + h, z, rx, h, 2] as WRow), "pillow", { along: [0, 0, 1], xHint: [1, 0, 0] });
  k.mound("duvet-trunk", "torso", "duvet-cover", T.duvet, TRUNK_MOUND.filter((_, i) => i !== 2 && i !== 7), Y0, MOUND_Z, 0.98, "top");
  k.mound("duvet-thighs", "thigh-r", "duvet-cover", T.duvet, [[1.12, 0], [1.13, 0.245], [1.21, 0.256], [1.215, 0.29], [1.3, 0.283], [1.51, 0.232], [1.52, 0]], Y0, 0, 0.8, "bottom");
  k.mound("duvet-shins", "shank-r", "duvet-cover", T.duvet, [[1.44, 0], [1.45, 0.236], [1.6, 0.239], [1.76, 0.22], [1.795, 0.17], [1.802, 0]], Y0, 0, 0.8, "bottom");
  for (const [side, s] of [["l", 1], ["r", -1]] as const) {
    profileSide(k, `drape-${side}`, s, x0, x1, flat, 0.085, T.duvet);
    profileSide(k, `fold-drape-${side}`, s, fold, x1 + 0.002, 2 * flat, 0.11, T.fold);
  }
}

/** 🛏️ 1:500: head and hair, shoulders, the right arm and its hand, the feet, the pillow, two mounds and one generalised duvet slab of 75 mm with its drapes. */
/** 🥚️ A coarse ellipsoidal mass as a loft of four rows along u (cheaper than an ellipsoid at the fixed tessellation of C4). */
function blob(k: Kit, id: string, bone: string, material: string, tone: number, c: readonly [number, number, number], r: readonly [number, number, number], region: string): void {
  k.loft(id, bone, material, tone, [-0.92, -0.45, 0.45, 0.92].map((t) => [c[0] + t * r[0], c[1], c[2], r[2] * Math.sqrt(1 - t * t), r[1] * Math.sqrt(1 - t * t)] as Row), region);
}

export function body500(k: Kit): void {
  blob(k, "head", "head", "skin", T.skin, [0.02, -0.01, 0], [0.12, 0.095, 0.075], "face-skin");
  blob(k, "hair", "head", "hair", T.hair, [0.05, 0.045, 0], [0.105, 0.0675, 0.09], "hair");
  k.wloft("shoulders", "torso", PYJAMA, T.pyjama, [[0.46, 0.548, 0, 0.12, 0.05], [0.51, 0.548, 0, 0.19, 0.08], [0.6, 0.548, 0, 0.2, 0.085], [0.67, 0.548, 0, 0.18, 0.08]], "top");
  k.loft("sleeve-r", "arm-r", PYJAMA, T.pyjama, [[0, 0, 0, 0.045, 0.045], [0.32, 0, 0, 0.0385, 0.0385]], "top");
  k.loft("foresleeve-r", "forearm-r", PYJAMA, T.pyjama, [[-0.01, 0, 0, 0.0385, 0.0385], [0.24, 0, 0, 0.0385, 0.0385]], "top");
  blob(k, "hand-r", "hand-r", "skin", T.skin, [0.08, 0.008, 0.008], [0.09, 0.0375, 0.05], "hand-skin");
  for (const side of ["l", "r"] as const) k.loft(`ankle-${side}`, `shank-${side}`, "skin", T.skin, [[0.3, 0, 0, 0.0385, 0.0385], [0.45, -0.008, 0, 0.0385, 0.0385]], "feet-skin");
  for (const side of ["l", "r"] as const) blob(k, `foot-${side}`, `foot-${side}`, "skin", T.skin, [0.06, -0.012, 0], [0.13, 0.045, 0.0425], "feet-skin");
  k.wloft("pillow", "pillow", "duvet-cover", T.pillow, [[0.27, Y0 + 0.04, -0.4, 0.17, 0.04, 2.5], [0.27, Y0 + 0.06, -0.3, 0.2, 0.06, 2.5], [0.27, Y0 + 0.06, 0.3, 0.2, 0.06, 2.5], [0.27, Y0 + 0.04, 0.4, 0.17, 0.04, 2.5]], "pillow", { along: [0, 0, 1], xHint: [1, 0, 0] });
  k.mound("duvet-trunk", "torso", "duvet-cover", T.duvet, [[0.594, 0], [0.605, 0.2], [0.66, 0.255], [0.72, 0.262], [0.84, 0.248], [0.95, 0.224], [1.15, 0.21], [1.17, 0]], Y0, MOUND_Z, 1, "top");
  k.mound("duvet-legs", "thigh-r", "duvet-cover", T.duvet, [[1.12, 0], [1.13, 0.245], [1.215, 0.29], [1.3, 0.283], [1.46, 0.2525], [1.50, 0.2225], [1.52, 0]], Y0, 0, 0.8, "bottom");
  k.mound("duvet-calves", "shank-r", "duvet-cover", T.duvet, [[1.434, 0], [1.456, 0.20375], [1.495, 0.2525], [1.61, 0.23875], [1.747, 0.2225], [1.782, 0.1875], [1.8, 0]], Y0, 0, 0.8, "bottom");
  const t = 0.075;
  const pts: [number, number][] = [[t, -0.45 - t], [t, 0.45 + t], [-0.115, 0.45 + t], [-0.115, 0.45], [0, 0.45], [0, -0.45], [-0.115, -0.45], [-0.115, -0.45 - t]];
  k.ext("duvet-slab", "duvet", "duvet-cover", T.duvet, "yz", pts, 0, x1 - x0, "duvet-edge");
  void W;
}
