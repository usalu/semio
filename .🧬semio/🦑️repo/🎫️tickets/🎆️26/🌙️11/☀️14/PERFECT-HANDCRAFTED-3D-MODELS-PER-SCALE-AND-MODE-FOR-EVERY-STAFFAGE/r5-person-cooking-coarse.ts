/** 🍳 The 1:100, 1:200 and 1:500 models of `humans/person-cooking`, each stated anew for its smallest feature (15, 30, 75 mm): fewer and generalised sections, merged masses (hem, pocket and hood rim into the sweatshirt, cuffs into the sleeves, sole and upper into one shoe, fingers into a mitten, the face details into the head; the carcass of the unit into one box, then fronts, plinth and worktop into it; the pot from a thin shell with lugs to a plain vessel; the spoon to one rod), the same woman at the same hob with the same proportions. Thermal: one surface solid per region (layers merged, B2) and the four outlets at the scale's size floor; structure: a merged skeleton and the carcass on its legs (1:100), a skeleton of bars and hoops and a carcass of two cheeks (1:200) and a stick skeleton beside a portal of two cheeks under the worktop (1:500).
 * @see ./r5-person-cooking-s50.ts
 * @see ./r3-humans-person-cooking.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, aboutU, aboutW, box, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, pa, revolve, sa, st, thOf, toRight, tube, type St, type V3 } from "./r3-person-cooking-kit.ts";
import { C, G, MAT, TONE, headSt, skirtSt, torsoSt } from "./r5-person-cooking-s50.ts";
import { pelvisRow, type Slice } from "./r5-person-cooking-forms.ts";
import { POT, RIG, SPOON, UNIT } from "./r3-person-cooking-data.ts";

type Spec = {
  scale: 100 | 200 | 500;
  skirt: Slice[];
  trunk: Slice[];
  hood: Slice[];
  seat: Slice[];
  thigh: St[];
  shank: St[];
  sleeve: St[];
  fore: St[];
  shoe: [number, number, number, number, number][];
  toe: [number, number, number, number, number][];
  head: [number, number, number, number, number][];
  hair: [number, number, number, number, number][];
  neck: St[];
};

const S100: Spec = {
  scale: 100,
  skirt: [[0.836, -0.13, 0.088, 0.175, 2.45], [0.89, -0.14, 0.1, 0.186, 2.5]],
  trunk: [[0.99, -0.124, 0.106, 0.186, 2.5], [1.09, -0.119, 0.111, 0.184, 2.5], [1.19, -0.133, 0.128, 0.188, 2.45], [1.25, -0.139, 0.134, 0.192, 2.4], [1.33, -0.139, 0.104, 0.196, 2.4], [1.4, -0.122, 0.067, 0.183, 2.2], [1.435, -0.105, 0.045, 0.145, 2.05], [1.462, -0.09, 0.026, 0.084, 2.0]],
  hood: [[1.27, -0.152, -0.128, 0.052, 2.2], [1.34, -0.162, -0.124, 0.112, 2.4], [1.4, -0.152, -0.108, 0.126, 2.4], [1.45, -0.127, -0.068, 0.11, 2.3], [1.485, -0.105, -0.03, 0.088, 2.2]],
  seat: [[0.778, -0.086, 0.052, 0.138, 2.2], [0.83, -0.125, 0.075, 0.168, 2.35], [0.89, -0.124, 0.085, 0.168, 2.35], [0.98, -0.098, 0.086, 0.146, 2.35]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.006, 0.083, 0.09, 2.2), sa("thigh-l", 0.1, 0.004, -0.003, 0.08, 0.085, 2.1), sa("thigh-l", 0.26, 0.008, -0.004, 0.065, 0.07, 2), sa("thigh-l", 0.4, 0.006, 0, 0.051, 0.055, 2), sa("thigh-l", 0.445, 0.002, 0, 0.045, 0.047, 2)],
  shank: [sa("shank-l", -0.035, 0.002, 0, 0.044, 0.046, 2), sa("shank-l", 0.0, 0.004, 0, 0.05, 0.053, 2), sa("shank-l", 0.13, -0.012, 0.002, 0.05, 0.055, 2), sa("shank-l", 0.27, -0.006, 0, 0.041, 0.045, 2), sa("shank-l", 0.388, -0.004, 0, 0.041, 0.046, 2.1)],
  sleeve: [sa("arm-l", -0.02, -0.003, -0.014, 0.034, 0.042, 2), sa("arm-l", 0.03, -0.002, 0.0, 0.053, 0.058, 2), sa("arm-l", 0.19, 0, 0.003, 0.05, 0.052, 2), sa("arm-l", 0.318, -0.006, 0.002, 0.047, 0.049, 2)],
  fore: [sa("forearm-l", -0.034, -0.008, 0.002, 0.046, 0.048, 2), sa("forearm-l", 0.1, 0, 0.002, 0.043, 0.045, 2), sa("forearm-l", 0.2, 0, 0.001, 0.038, 0.038, 2), sa("forearm-l", 0.25, 0, 0, 0.03, 0.026, 2.2)],
  shoe: [[-0.066, 0.0, 0.082, 0.021, 2.4], [-0.05, 0.0, 0.098, 0.031, 2.6], [0.0, 0.0, 0.104, 0.036, 2.8], [0.04, 0.0, 0.094, 0.039, 2.9], [0.085, 0.0, 0.071, 0.044, 3], [0.127, 0.0, 0.058, 0.047, 3]],
  toe: [[-0.012, 0.0, 0.058, 0.047, 3], [0.03, 0.0, 0.05, 0.044, 2.8], [0.06, 0.0, 0.04, 0.034, 2.4], [0.076, 0.004, 0.03, 0.018, 2.1]],
  head: [[-0.068, 0.058, 0.09, 0.016, 2], [-0.045, -0.004, 0.097, 0.041, 2.1], [-0.01, -0.045, 0.1, 0.055, 2.2], [0.03, -0.074, 0.098, 0.067, 2.3], [0.075, -0.083, 0.099, 0.072, 2.3], [0.115, -0.075, 0.08, 0.066, 2.2], [0.14, -0.04, 0.035, 0.035, 2]],
  hair: [[0.152, -0.03, 0.022, 0.028, 2], [0.137, -0.08, 0.066, 0.064, 2.1], [0.11, -0.091, 0.087, 0.0745, 2.25], [0.075, -0.09, 0.03, 0.0748, 2.3], [0.027, -0.08, -0.022, 0.0685, 2.25], [-0.02, -0.045, -0.032, 0.047, 2.1]],
  neck: [sa("neck", -0.022, 0.008, 0, 0.054, 0.056, 2.1), sa("neck", 0.05, 0.013, 0, 0.049, 0.051, 2), sa("neck", 0.108, 0.004, 0, 0.049, 0.048, 2)],
};
const S200: Spec = {
  ...S100,
  scale: 200,
  skirt: [[0.838, -0.132, 0.09, 0.177, 2.45], [0.9, -0.14, 0.101, 0.187, 2.5]],
  trunk: [[1.0, -0.123, 0.106, 0.186, 2.5], [1.12, -0.123, 0.116, 0.185, 2.5], [1.24, -0.139, 0.134, 0.191, 2.4], [1.34, -0.138, 0.1, 0.196, 2.35], [1.41, -0.118, 0.06, 0.178, 2.15], [1.46, -0.092, 0.028, 0.088, 2.0]],
  hood: [[1.275, -0.155, -0.128, 0.056, 2.2], [1.36, -0.162, -0.12, 0.124, 2.4], [1.44, -0.135, -0.076, 0.116, 2.3], [1.485, -0.106, -0.03, 0.088, 2.2]],
  seat: [[0.778, -0.09, 0.055, 0.14, 2.2], [0.86, -0.126, 0.082, 0.168, 2.35], [0.98, -0.098, 0.086, 0.146, 2.35]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.006, 0.083, 0.09, 2.2), sa("thigh-l", 0.2, 0.008, -0.004, 0.071, 0.076, 2.05), sa("thigh-l", 0.44, 0.003, 0, 0.048, 0.05, 2)],
  shank: [sa("shank-l", -0.02, 0.003, 0, 0.048, 0.05, 2), sa("shank-l", 0.13, -0.012, 0.002, 0.05, 0.055, 2), sa("shank-l", 0.388, -0.004, 0, 0.041, 0.046, 2)],
  sleeve: [sa("arm-l", -0.02, -0.003, -0.013, 0.036, 0.044, 2), sa("arm-l", 0.04, -0.002, 0.0, 0.054, 0.059, 2), sa("arm-l", 0.318, -0.006, 0.002, 0.048, 0.05, 2)],
  fore: [sa("forearm-l", -0.034, -0.008, 0.002, 0.046, 0.048, 2), sa("forearm-l", 0.25, 0, 0, 0.033, 0.031, 2.1)],
  shoe: [[-0.066, 0.0, 0.084, 0.023, 2.4], [-0.03, 0.0, 0.103, 0.035, 2.7], [0.04, 0.0, 0.093, 0.04, 2.9], [0.127, 0.0, 0.058, 0.047, 3]],
  toe: [[-0.012, 0.0, 0.058, 0.047, 3], [0.045, 0.0, 0.046, 0.041, 2.6], [0.077, 0.004, 0.032, 0.02, 2.1]],
  head: [[-0.066, 0.05, 0.091, 0.02, 2], [-0.03, -0.025, 0.1, 0.05, 2.2], [0.03, -0.074, 0.099, 0.067, 2.3], [0.09, -0.082, 0.096, 0.071, 2.3], [0.138, -0.045, 0.04, 0.04, 2]],
  hair: [[0.152, -0.032, 0.026, 0.03, 2], [0.12, -0.09, 0.084, 0.073, 2.2], [0.07, -0.09, 0.026, 0.0748, 2.3], [-0.02, -0.048, -0.032, 0.048, 2]],
  neck: [sa("neck", -0.022, 0.008, 0, 0.054, 0.056, 2.1), sa("neck", 0.108, 0.004, 0, 0.05, 0.05, 2)],
};
const S500: Spec = {
  ...S200,
  scale: 500,
  skirt: [[0.84, -0.134, 0.092, 0.18, 2.45], [0.9, -0.141, 0.102, 0.188, 2.5]],
  trunk: [[1.0, -0.139, 0.113, 0.187, 2.5], [1.24, -0.161, 0.14, 0.192, 2.4], [1.37, -0.157, 0.088, 0.196, 2.35], [1.42, -0.135, 0.058, 0.17, 2.15], [1.46, -0.108, 0.03, 0.09, 2.0]],
  hood: [],
  seat: [[0.778, -0.092, 0.057, 0.142, 2.2], [0.87, -0.127, 0.083, 0.17, 2.35], [0.98, -0.1, 0.087, 0.148, 2.35]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.006, 0.084, 0.091, 2.2), sa("thigh-l", 0.44, 0.003, 0, 0.05, 0.052, 2)],
  shank: [sa("shank-l", -0.02, 0.003, 0, 0.05, 0.052, 2), sa("shank-l", 0.14, -0.012, 0.002, 0.051, 0.056, 2), sa("shank-l", 0.388, -0.004, 0, 0.042, 0.046, 2)],
  sleeve: [sa("arm-l", 0.0, -0.003, -0.01, 0.042, 0.04, 2), sa("arm-l", 0.318, -0.006, 0.002, 0.05, 0.042, 2)],
  fore: [sa("forearm-l", -0.034, -0.008, 0.002, 0.047, 0.042, 2), sa("forearm-l", 0.25, 0, 0, 0.04, 0.036, 2)],
  shoe: [[-0.066, 0.0, 0.088, 0.03, 2.4], [0.0, 0.0, 0.104, 0.04, 2.8], [0.127, 0.0, 0.075, 0.047, 3]],
  toe: [[-0.012, 0.0, 0.075, 0.047, 3], [0.078, 0.0, 0.05, 0.038, 2.2]],
  head: [[-0.066, 0.045, 0.092, 0.03, 2], [0.0, -0.06, 0.1, 0.06, 2.2], [0.08, -0.083, 0.098, 0.072, 2.3], [0.14, -0.04, 0.036, 0.04, 2]],
  hair: [[0.155, -0.04, 0.04, 0.04, 2], [0.1, -0.094, 0.09, 0.078, 2.2], [0.02, -0.082, -0.022, 0.068, 2.25], [-0.03, -0.06, -0.03, 0.045, 2.1]],
  neck: [sa("neck", -0.022, 0.01, 0, 0.054, 0.056, 2.1), sa("neck", 0.108, 0.004, 0, 0.051, 0.051, 2)],
};

const xSlice = (x: number, lo: number, hi: number, hw: number, e: number, l = 0, ground = G): St => ({ at: [x, ground + (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });

/** ✋ A mitten hand: the palm with the fingers merged into it, curled by `curl` towards the palm (1 = a fist), and at 1:100 a thumb. */
function hand(sp: Spec, curl: number, prefix: string): Json[] {
  const c = (id: string): Json => C(id, "hand-l", MAT.skin, TONE.hand);
  const t = (x: number): number => (sp.scale === 500 ? 0.0382 : sp.scale === 200 ? Math.max(x, 0.0155) : x);
  const palm: St[] = [sa("hand-l", -0.014, 0, 0, t(0.016), Math.max(0.026, t(0)), 2.3), sa("hand-l", 0.055, 0.002, -0.0015, t(0.0135), 0.037, 2.5), sa("hand-l", 0.1, 0, -0.004 - curl * 0.012, t(0.0125), 0.037, 2.4), sa("hand-l", 0.162 - curl * 0.045, 0, -0.01 - curl * 0.034, t(0.011), 0.032, 2.2)];
  const out = [loft(c(prefix), dome(sp.scale === 100 ? palm : [palm[0]!, palm[1]!, palm[3]!], 1, 0.012, 1))];
  if (sp.scale === 100) out.push(capsule(c(`${prefix}-thumb`), pa("hand-l", 0.014, 0.022, -0.006), pa("hand-l", 0.092 - curl * 0.02, 0.05 - curl * 0.02, -0.03), 0.0115, 0.0085));
  return out;
}

function figure(sp: Spec): Json[] {
  const out: Json[] = [];
  const trunk = [...sp.skirt.map((s) => skirtSt(s)), ...sp.trunk.map((s) => torsoSt(s))];
  out.push(loft(C("hoodie", "torso", MAT.fleece, TONE.hoodie), sp.scale === 100 ? dense(trunk, 2) : trunk));
  if (sp.hood.length > 0) out.push(loft(C("hood", "torso", MAT.fleece, TONE.hoodie), dome(sp.hood.map((s) => torsoSt(s)), -1, 0.01, 1)));
  if (sp.scale === 100) out.push(loft(C("kangaroo-pocket", "torso", MAT.fleece, TONE.pocket, "detail"), ([[0.905, 0.07, 0.115, 0.151, 3.2], [0.99, 0.072, 0.118, 0.14, 3.2], [1.058, 0.076, 0.118, 0.104, 3.0]] as Slice[]).map((s) => torsoSt(s))));
  out.push(loft(C("hips", "pelvis", MAT.denim, TONE.jeans), sp.seat.map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw, hd, e); })));
  out.push(loft(C("thigh-l", "thigh-l", MAT.denim, TONE.jeans), sp.thigh));
  out.push(loft(C("shank-l", "shank-l", MAT.denim, TONE.jeans), sp.shank));
  const elbow = sp.scale === 500 ? 0.034 : 0.02;
  out.push(loft(C("sleeve-l", "arm-l", MAT.fleece, TONE.hoodie), dome(dome(sp.sleeve, -1, 0.008, 1), 1, elbow, 2)));
  out.push(loft(C("forearm-l", "forearm-l", MAT.fleece, TONE.hoodie), dome(sp.fore, -1, elbow, 1)));
  out.push(loft(C("shoe-l", "foot-l", MAT.sneaker, TONE.canvas), sp.shoe.map(([x, lo, hi, hw, e]) => xSlice(x, lo, hi, hw, e))));
  out.push(loft(C("toe-cap-l", "toe-l", MAT.sneaker, TONE.rubber), dome(sp.toe.map(([x, lo, hi, hw, e], i) => xSlice(x, lo, hi, hw, e, -0.003 * i, 0)), 1, 0.004, 1)));
  for (const s of [...out].filter((q) => /-l$/.test(q.id))) out.push(mirrored(s, toRight));
  out.push(...hand(sp, 0.25, "hand-l"));
  out.push(...hand(sp, 1, "hand-l").map((s) => mirrored(s, toRight)));
  out.push(loft(C("head", "head", MAT.skin, TONE.skin), dome(sp.head.map(headSt), 1, 0.003, 1)));
  if (sp.scale === 100) {
    out.push(loft(C("nose", "head", MAT.skin, TONE.skin), [sa("head", 0.04, 0.094, 0, 0.0075, 0.0075, 2), sa("head", 0.0, 0.106, 0, 0.0145, 0.0085, 2)]));
    for (const side of [1, -1]) out.push(ellipsoid(C(`ear-${side > 0 ? "l" : "r"}`, "head", MAT.skin, TONE.skin), [0.029, 0.016, 0.0077], pa("head", 0.03, 0.002, side * 0.069), compose(aboutU(side * -14), aboutW(14))));
  }
  out.push(loft(C("hair", "head", MAT.hair, TONE.hair), dome(sp.hair.map(headSt), -1, 0.005, 1)));
  out.push(loft(C("neck", "neck", MAT.skin, TONE.skin), sp.neck));
  return out;
}

const K = "counter";
const U = UNIT;
const HW = U.width / 2;
const TOP = U.plinth + U.carcass;
const slab = (id: string, x0: number, x1: number, y0: number, y1: number, z0: number, z1: number, material: string, tone: number, role = "body"): Json => box(C(id, K, material, tone, role), [x1 - x0, y1 - y0, z1 - z0], [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2]);
const CUT = { x0: 0.055, x1: 0.545, z: 0.28 } as const;
/** 🗄️ The carcass as one hollow shell (C30): two cheeks, bottom and top rail plane of 16 mm board from `x0` to the back, open to the fronts and to the wall behind. */
const shellOf = (x0: number, tone: number): Json => extrude(C("carcass", K, MAT.board, tone), "yz", [[U.plinth, -HW], [TOP, -HW], [TOP, HW], [U.plinth, HW]], x0, U.depth, { holes: [[[U.plinth + 0.016, -HW + 0.016], [TOP - 0.016, -HW + 0.016], [TOP - 0.016, HW - 0.016], [U.plinth + 0.016, HW - 0.016]]] });
const worktop = (): Json => extrude(C("worktop", K, MAT.board, TONE.worktop), "zx", [[-HW, 0], [HW, 0], [HW, U.depth], [-HW, U.depth]], TOP, U.top, { holes: [[[-CUT.z, CUT.x0], [CUT.z, CUT.x0], [CUT.z, CUT.x1], [-CUT.z, CUT.x1]]] });

/** 🥄 The spoon as one slender rod of its true 12 mm. */
const rod = (id: string): Json => ({ ...capsule(C(id, "spoon", MAT.wood, TONE.wood), [-SPOON.back + 0.006, 0, 0], [SPOON.len - 0.006, 0, 0], 0.006, 0.006), slender: true });
/** 🗄️ The unit, the pot and the spoon of a coarse scale. */
function props(sp: Spec): Json[] {
  const out: Json[] = [];
  const x0 = U.front + U.door + 0.001;
  if (sp.scale === 100) {
    out.push(shellOf(x0, TONE.carcass));
    out.push(slab("carcass-back", U.depth - 0.02, U.depth - 0.012, U.plinth + 0.016, TOP - 0.016, -HW + 0.016, HW - 0.016, MAT.board, TONE.carcass));
    for (const [id, y0, y1, hy] of [["bottom", U.plinth, 0.432, 0.392], ["middle", 0.432, 0.722, 0.682], ["top", 0.722, TOP, 0.792]] as const) {
      out.push(slab(`drawer-front-${id}`, U.front, x0, y0, y1, -HW, HW, MAT.board, TONE.front));
      out.push({ ...capsule(C(`handle-${id}`, K, MAT.steel, TONE.steel, "detail"), [U.front - 0.032, hy, -0.16], [U.front - 0.032, hy, 0.16], 0.006, 0.006), slender: true });
    }
    out.push(slab("plinth", U.plinthBack, U.plinthBack + 0.016, 0, U.plinth, -HW, HW, MAT.board, TONE.plinth));
    for (const side of [1, -1]) out.push(slab(`plinth-side-${side > 0 ? "l" : "r"}`, U.plinthBack + 0.016, U.depth, 0, U.plinth, side > 0 ? HW - 0.016 : -HW, side > 0 ? HW : -HW + 0.016, MAT.board, TONE.plinth));
    out.push(worktop());
    out.push(slab("hob-plate", U.hobX0, U.hobX0 + U.hobDepth, U.top, U.top + U.glass, -U.hobWidth / 2, U.hobWidth / 2, MAT.glass, TONE.hob));
    out.push(slab("hob-tray", CUT.x0, CUT.x1, U.top - 0.046, U.top, -CUT.z, CUT.z, MAT.glass, TONE.steel));
    for (const [id, x, z, r] of [["zone-front-right", U.zoneFront, -U.zoneZ, 0.105], ["zone-rear-left", U.zoneRear, U.zoneZ, 0.105], ["zone-rear-right", U.zoneRear, -U.zoneZ, 0.0725], ["zone-front-left", U.zoneFront, U.zoneZ, 0.0725]] as const) out.push(revolve(C(id, K, MAT.glass, TONE.zone, "detail"), [[0, r - 0.006], [0.001, r - 0.006], [0.001, r], [0, r], [0, r - 0.006]], [x, U.top + U.glass - 0.0002, z], [0, 1, 0], { texture: true }));
    out.push(revolve(C("pot-shell", "pot", MAT.steel, TONE.steel), [[0, 0], [0, 0.104], [0.012, 0.11], [0.127, 0.11], [0.13, 0.116], [0.1312, 0.1152], [0.1275, 0.109], [0.014, 0.109], [0.008, 0.105], [0.008, 0]], [0, 0, 0], [0, 1, 0]));
    out.push(revolve(C("pot-water", "pot", MAT.water, TONE.water), [[0.008, 0], [0.008, 0.105], [0.014, 0.109], [POT.water, 0.109], [POT.water, 0]], [0, 0, 0], [0, 1, 0]));
    for (const side of [1, -1]) out.push(box(C(`pot-handle-${side > 0 ? "l" : "r"}`, "pot", MAT.steel, TONE.steel), [0.07, 0.015, 0.036], [0, 0.108, side * 0.127], undefined, 0.006));
    out.push({ ...loft(C("spoon-handle", "spoon", MAT.wood, TONE.wood), [st(-SPOON.back, 0, 0, 0.0056, 0.0056), st(0.04, 0, 0, 0.0058, 0.0058), st(0.165, 0, 0, 0.0056, 0.0056)]), slender: true });
    out.push(loft(C("spoon-bowl", "spoon", MAT.wood, TONE.wood), dome([st(0.158, 0, 0, 0.009, 0.0056), st(0.172, 0, 0, 0.02, 0.0056), st(0.195, 0, 0, 0.021, 0.0056), st(0.212, 0, 0, 0.011, 0.0056)], 1, 0.003, 1)));
    return out;
  }
  if (sp.scale === 200) {
    out.push(shellOf(U.front + 0.03, TONE.carcass));
    out.push(slab("carcass-back", U.depth - 0.03, U.depth, U.plinth + 0.016, TOP - 0.016, -HW + 0.016, HW - 0.016, MAT.board, TONE.carcass));
    out.push(slab("drawer-fronts", U.front, U.front + 0.03, U.plinth, TOP, -HW, HW, MAT.board, TONE.front));
    out.push(slab("plinth", U.plinthBack, U.depth, 0, U.plinth, -HW, HW, MAT.board, TONE.plinth));
    out.push(worktop());
    out.push(slab("hob-plate", CUT.x0, CUT.x1, TOP, U.top + U.glass, -CUT.z, CUT.z, MAT.glass, TONE.hob));
    out.push(revolve(C("pot-shell", "pot", MAT.steel, TONE.steel), [[0, 0], [0, 0.11], [0.13, 0.114], [0.13, 0.109], [0.007, 0.109], [0.007, 0]], [0, 0, 0], [0, 1, 0]));
    out.push(revolve(C("pot-water", "pot", MAT.water, TONE.water), [[0.007, 0], [0.007, 0.109], [POT.water, 0.109], [POT.water, 0]], [0, 0, 0], [0, 1, 0]));
  } else {
    out.push(slab("carcass", 0, U.depth, 0, U.top + U.glass - 0.075, -HW, HW, MAT.board, TONE.front));
    out.push(slab("hob-plate", 0, U.depth, U.top + U.glass - 0.075, U.top + U.glass, -HW, HW, MAT.glass, TONE.hob));
    out.push(revolve(C("pot-shell", "pot", MAT.steel, TONE.steel), [[0, 0], [0, 0.112], [0.13, 0.112], [0.13, 0]], [0, 0, 0], [0, 1, 0]));
  }
  if (sp.scale === 200) out.push(rod("spoon-handle"));
  return out;
}

/** 🌡️ One surface solid per region, the outlets at nostrils, mouth, crown and water surface sized to the scale's smallest feature. */
function thermal(b: Json[], sp: Spec): Json[] {
  const f = sp.scale * 0.00015;
  const out: Json[] = [];
  const head = b.find((s) => s.id === "head")!;
  out.push({ ...head, id: "th-face", region: "face-skin", sections: head.sections.filter((q: Json) => q.at[0] <= 0.1), capEnd: false });
  for (const id of ["nose", "ear-l", "ear-r", "neck"]) if (b.some((s) => s.id === id)) out.push(thOf(id, "face-skin"));
  out.push(thOf("hair", "hair", "hair"));
  for (const s of b.filter((q) => /^hand-[lr]/.test(q.id))) out.push(thOf(s.id, "hand-skin"));
  for (const s of b.filter((q) => q.material === MAT.fleece)) out.push(thOf(s.id, "hoodie", "sweatshirt"));
  for (const s of b.filter((q) => q.material === MAT.denim && (sp.scale !== 100 || q.id !== "hips"))) out.push(thOf(s.id, "jeans", "jeans"));
  if (sp.scale === 100) out.push({ ...loft(C("th-hips", "pelvis", MAT.denim, TONE.jeans), ([sp.seat[0]!, sp.seat[1]!, [0.866, -0.13, 0.083, 0.175, 2.38]] as Slice[]).map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw, hd, e); }), [true, true], { sectionFaces: ["+u"] }), region: "jeans", layer: "jeans" });
  for (const s of b.filter((q) => q.material === MAT.sneaker)) out.push(thOf(s.id, "sneakers", "sneaker"));
  for (const s of b.filter((q) => q.bone === K)) {
    if (s.id === "hob-tray") continue;
    if (s.id === "worktop" && sp.scale === 100) out.push({ ...slab("th-worktop", 0, U.depth, TOP, U.top, -HW, HW, MAT.board, TONE.worktop), region: "counter" });
    else if (s.id === "carcass-back") continue;
    else if (s.id === "carcass" && sp.scale !== 500) out.push({ ...slab("th-carcass", sp.scale === 100 ? U.front + U.door + 0.001 : U.front + 0.03, U.depth, U.plinth, TOP, -HW, HW, MAT.board, TONE.carcass), region: "counter" });
    else out.push(thOf(s.id, /^(hob-|zone-)/.test(s.id) ? "hob" : "counter"));
  }
  for (const s of b.filter((q) => q.bone === "pot")) out.push(thOf(s.id, "pot"));
  for (const s of b.filter((q) => q.bone === "spoon")) out.push(thOf(s.id, "spoon"));
  if (sp.scale === 500) out.push({ ...rod("th-spoon"), region: "spoon" });
  const t = Math.max(f, 0.0078) / 2 + 0.0002;
  out.push({ ...ellipsoid(C("th-nose-breath", "head", MAT.skin, TONE.skin, "detail"), [t, t, Math.max(t, 0.0085)], pa("head", -0.002, 0.106 - t + 0.004, 0)), region: "face-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", MAT.skin, TONE.lips, "detail"), [t, t, Math.max(t, 0.0115)], pa("head", -0.022, 0.1 - t + 0.004, 0)), region: "face-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-body-plume", "head", MAT.hair, TONE.hair, "detail"), [t, Math.max(t, 0.06), Math.max(t, 0.05)], pa("head", 0.155 - t + 0.004, 0, 0)), region: "hoodie", tags: ["emitter", "body-plume"] });
  out.push({ ...ellipsoid(C("th-steam", "pot", MAT.water, TONE.water, "detail"), [0.1, t, 0.1], [0, (sp.scale === 500 ? POT.h : POT.water) - t + 0.002, 0]), region: "pot", tags: ["emitter", "steam"] });
  return out;
}

const B = (id: string, bone: string): Json => C(id, bone, MAT.bone, TONE.bone);
const bar = (id: string, bone: string, pts: V3[], r: number[] | number, member = bone): Json => ({ ...tube(B(id, bone), pts, Array.isArray(r) ? r : pts.map(() => r)), member, part: "member" });
const el = (id: string, bone: string, radii: V3, at: V3, part: "member" | "joint" | "mass" = "member", member = bone): Json => ({ ...ellipsoid(C(id, bone, MAT.bone, part === "member" ? TONE.bone : part === "joint" ? TONE.joint : TONE.mass), radii, at), member, part });

/** 🦴 The skeleton of a coarse scale in this woman's own segment lengths: bones merged by scale, joints at the rig joints, masses at the centres of mass; then the load path of the unit, the pot and the spoon. */
function structure(sp: Spec, b: Json[]): Json[] {
  const fine = sp.scale === 100;
  const half = sp.scale * 0.00015 * 0.5 + 0.0003;
  const r = (x: number): number => Math.max(x, half);
  const out: Json[] = [];
  const T = RIG.torsoLen;
  const spine: V3[] = [[-0.01, 0.002], [0.09, 0.016], [0.19, -0.004], [0.3, -0.031], [0.4, -0.026], [T, 0]].map(([a, f]) => pa("torso", a!, f!, 0));
  out.push(bar("spine", "torso", spine, [0.023, 0.023, 0.02, 0.016, 0.014, 0.012].map(r)));
  const ribs: [number, number, number][] = fine ? [[0.425, 0.068, 0.06], [0.378, 0.098, 0.1], [0.328, 0.108, 0.112], [0.282, 0.114, 0.112], [0.236, 0.114, 0.095], [0.194, 0.1, 0.05]] : sp.scale === 200 ? [[0.39, 0.095, 0.09], [0.308, 0.111, 0.112], [0.23, 0.113, 0.095]] : [[0.316, 0.1, 0.11]];
  ribs.forEach(([a, w, drop], i) => {
    const pts: V3[] = [[a, -0.06, 0.02], [a, -0.07, w * 0.5], [a - drop * 0.4, -0.02, w * 0.97], [a - drop * 0.8, 0.06, w * 0.65], [a - drop * 0.92, 0.085, 0.02]].map(([aa, f, l]) => pa("torso", aa!, f!, l!));
    const rib = bar(`rib-${i + 1}-l`, "torso", pts, r(fine ? 0.0075 : 0.008));
    out.push(rib, mirrored(rib, toRight));
  });
  if (sp.scale !== 500) {
    out.push({ ...loft(B("sternum", "torso"), [sa("torso", 0.412, 0.07, 0, 0.022, r(0.009), 2.4), sa("torso", 0.236, 0.108, 0, r(0.014), r(0.008), 2.2)]), member: "torso", part: "member" });
    const girdle = bar("shoulder-girdle-l", "torso", [[0.416, 0.066, 0.02], [0.424, 0.052, 0.112], [0.428, 0.008, 0.155], [0.41, -0.066, 0.11], [0.305, -0.073, 0.088]].map(([a, f, l]) => pa("torso", a!, f!, l!)), r(fine ? 0.008 : 0.01));
    out.push(girdle, mirrored(girdle, toRight));
  }
  const ilium = { ...loft(B("hip-bone-l", "pelvis"), [sa("pelvis", 0.122, -0.012, 0.116, 0.074, r(0.008), 2.2), sa("pelvis", 0.056, -0.004, 0.108, 0.05, r(0.013), 2.2), sa("pelvis", -0.03, -0.006, 0.08, 0.03, r(0.02), 2)].map((q, i) => ({ ...q, yaw: [55, 42, 0][i] }))), member: "pelvis", part: "member" };
  out.push(ilium, mirrored(ilium, toRight));
  out.push({ ...loft(B("sacrum", "pelvis"), [sa("pelvis", 0.097, -0.032, 0, 0.054, r(0.02), 2.2, -28), sa("pelvis", -0.03, -0.098, 0, r(0.014), r(0.009), 2, -58)]), member: "pelvis", part: "member" });
  if (sp.scale !== 500) out.push(bar("pubic-arch", "pelvis", [[-0.01, 0.02, 0.08], [-0.03, 0.06, 0.0], [-0.01, 0.02, -0.08]].map(([a, f, l]) => pa("pelvis", a!, f!, l!)), r(0.011)));
  out.push(el("cranium", "head", [0.073, 0.087, 0.067], pa("head", 0.072, -0.004, 0)));
  if (sp.scale !== 500) out.push({ ...loft(B("facial-skeleton", "head"), [sa("head", 0.055, 0.062, 0, 0.056, 0.03, 2.2), sa("head", -0.045, 0.064, 0, 0.04, r(0.022), 2)]), member: "head", part: "member" });
  out.push(bar("cervical-spine", "neck", [pa("neck", -0.005, 0, 0), pa("neck", 0.05, 0.003, 0), pa("neck", RIG.neckLen - 0.004, 0.002, 0)], r(0.012)));
  const limbs: Json[] = [
    bar("femur-l", "thigh-l", [pa("thigh-l", 0, 0, 0), pa("thigh-l", 0.03, -0.006, 0.05), pa("thigh-l", 0.15, 0.007, 0.038), pa("thigh-l", RIG.thigh - 0.02, -0.004, 0)], [0.021, 0.016, 0.013, 0.024].map(r)),
    bar("tibia-l", "shank-l", [pa("shank-l", 0, 0, 0), pa("shank-l", 0.2, 0.009, -0.004), pa("shank-l", RIG.shank - 0.012, 0.002, -0.004)], [0.028, 0.0135, 0.016].map(r)),
    bar("foot-bones-l", "foot-l", [[-0.046, -0.04, 0.004], [0.0, -0.012, 0], [0.06, -0.03, 0], [0.12, -RIG.ankle + RIG.ball + 0.014, 0]], [0.018, 0.019, 0.02, 0.014].map(r)),
    bar("humerus-l", "arm-l", [pa("arm-l", 0, 0, -0.004), pa("arm-l", 0.15, 0.002, 0.003), pa("arm-l", RIG.upperArm - 0.012, -0.004, 0)], [0.021, 0.011, 0.02].map(r)),
    bar("forearm-bones-l", "forearm-l", [pa("forearm-l", -0.015, -0.01, -0.004), pa("forearm-l", 0.12, 0.003, 0), pa("forearm-l", RIG.forearm - 0.008, 0.002, 0)], [0.012, 0.011, 0.015].map(r)),
  ];
  if (fine) limbs.push(bar("toe-bones-l", "toe-l", [[0.0, RIG.ball + 0.014, -0.004], [0.046, RIG.ball + 0.01, -0.008]], 0.0115), bar("fibula-l", "shank-l", [pa("shank-l", 0.04, -0.012, 0.03), pa("shank-l", RIG.shank - 0.004, -0.008, 0.026)], 0.0078), el("patella-l", "shank-l", [0.021, 0.01, 0.019], pa("shank-l", -0.012, 0.04, 0)));
  for (const q of limbs) out.push(q, mirrored(q, toRight));
  const handBones = (curl: number): Json => ({ ...loft(B("hand-bones-l", "hand-l"), [sa("hand-l", 0.0, 0, -0.001, r(0.008), Math.max(r(0), 0.023), 2.2), sa("hand-l", 0.09, 0, -0.003 - curl * 0.006, r(0.007), Math.max(r(0), 0.03), 2.4), sa("hand-l", 0.14 - curl * 0.036, 0, -0.012 - curl * 0.026, r(0.006), Math.max(r(0), 0.026), 2)]), member: "hand-l", part: "member" });
  out.push(handBones(0.25), mirrored(handBones(1), toRight));
  const jr = sp.scale === 500 ? 0.04 : sp.scale === 200 ? 0.02 : 0.016;
  const J = (x: number): number => Math.max(x, half);
  out.push(el("joint-lumbosacral", "torso", [J(jr * 0.8), J(jr * 1.4), J(jr * 1.4)], [0, 0, 0], "joint"));
  out.push(el("joint-cervicothoracic", "neck", [J(jr * 0.7), J(jr), J(jr)], [0, 0, 0], "joint"));
  for (const s of ["l", "r"]) {
    out.push(el(`joint-hip-${s}`, `thigh-${s}`, [J(jr * 1.4), J(jr * 1.4), J(jr * 1.4)], [0, 0, 0], "joint"));
    out.push(el(`joint-knee-${s}`, `shank-${s}`, [J(jr), J(jr * 1.4), J(jr * 1.8)], [0, 0, 0], "joint"));
    out.push(el(`joint-shoulder-${s}`, `arm-${s}`, [J(jr * 1.3), J(jr * 1.3), J(jr * 1.3)], [0, 0, 0], "joint"));
    out.push(el(`joint-elbow-${s}`, `forearm-${s}`, [J(jr), J(jr), J(jr * 1.4)], [0, 0, 0], "joint"));
    if (sp.scale !== 500) {
      out.push(el(`joint-ankle-${s}`, `foot-${s}`, [J(jr), J(jr), J(jr * 1.3)], [0, 0, 0], "joint"));
      out.push(el(`joint-wrist-${s}`, `hand-${s}`, [J(jr * 0.7), J(jr), J(jr * 0.7)], [0, 0, 0], "joint"));
    }
  }
  for (const bn of ASSET.rig.bones as Json[]) if (bn.mass && bn.mass.kg >= (sp.scale === 500 ? 6 : 3) && !["counter", "pot"].includes(bn.id)) { const m = Math.max(r(0.022), sp.scale === 500 ? 0.04 : 0); out.push(el(`mass-${bn.id}`, bn.id, [m, m, m], [bn.mass.comX, bn.mass.comY ?? 0, 0], "mass")); }

  const t = sp.scale === 100 ? 0.016 : sp.scale === 200 ? 0.03 : 0.075;
  const x0 = sp.scale === 500 ? 0 : U.front + U.door + 0.001;
  const P = (id: string, x1: number, x2: number, y1: number, y2: number, z1: number, z2: number, member = K): Json => ({ ...slab(id, x1, x2, y1, y2, z1, z2, MAT.board, TONE.bone), member, part: "member" });
  for (const side of [1, -1]) out.push(P(`carcass-side-${side > 0 ? "l" : "r"}`, x0, U.depth, U.plinth, sp.scale === 500 ? U.top + U.glass - t : TOP, side > 0 ? HW - t : -HW, side > 0 ? HW : -HW + t));
  if (sp.scale !== 500) out.push(P("carcass-bottom", x0, U.depth, U.plinth, U.plinth + t, -HW + t, HW - t));
  if (sp.scale === 500) out.push(P("worktop-slab", 0, U.depth, U.top + U.glass - t, U.top + U.glass, -HW, HW));
  else out.push({ id: "member-worktop", of: "worktop", member: K, part: "member" });
  const lr = Math.max(0.016, half);
  for (const [i, x] of [0.13, 0.53].entries()) for (const side of [1, -1]) out.push({ ...revolve(C(`unit-leg-${i === 0 ? "front" : "back"}-${side > 0 ? "l" : "r"}`, K, MAT.board, TONE.bone), [[0, 0], [0, lr], [U.plinth, lr], [U.plinth, 0]], [x, 0, side * (HW - 0.06)], [0, 1, 0]), member: "counter-lift", part: "member" });
  const mr = sp.scale === 500 ? 0.06 : 0.045;
  out.push(el("mass-counter", K, [mr, mr, mr], [0.31, 0.47, 0], "mass"));
  out.push({ id: "member-pot-shell", of: "pot-shell", member: "pot", part: "member" });
  for (const s of b.filter((q) => q.bone === "spoon")) out.push({ id: `member-${s.id}`, of: s.id, member: "spoon", part: "member" });
  if (sp.scale === 500) out.push({ ...capsule(C("spoon-rod", "spoon", MAT.wood, TONE.wood), [-0.05, 0, 0], [SPOON.len - 0.006, 0, 0], 0.006, 0.006), slender: true, member: "spoon", part: "member" });
  return out;
}

/** 📚 The three coarse scales. */
export function coarseVolumes(): Json {
  const out: Json = {};
  for (const sp of [S100, S200, S500]) {
    const b = [...figure(sp), ...props(sp)];
    out[String(sp.scale)] = { status: "authored", denominator: sp.scale, body: b, thermal: thermal(b, sp), structure: structure(sp, b), decisions: [] };
  }
  return out;
}
