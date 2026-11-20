/** 🪆 The 1:100, 1:200 and 1:500 models of `humans/parent-carrying-toddler`, each stated anew for its smallest feature (15, 30, 75 mm): fewer and coarser sections, merged parts (mitten-like gloves, one boot per foot, the toddler's boot shaft in its leg), the same proportions and the same carry.
 *
 * @see ./r3-parent-carrying-toddler-s50.ts
 * @see ./r3-humans-parent-carrying-toddler.md */
import type { Json } from "./volumes-lib.ts";
import { FRAMES, H, L, X, aboutU, aboutW, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-parent-carrying-toddler-kit.ts";
import { TONE, coatX, lean, near, spine, type Row } from "./r3-parent-carrying-toddler-s50.ts";
import { OUTER, emitters, heightOf, jointBall, massBall, regionOf } from "./r3-parent-carrying-toddler-lists50.ts";

type C = { id: string; bone: string; material: string; tone: number; role?: string };
type Scale = 100 | 200 | 500;
const ANKLE_Y = FRAMES["foot-l"]!.o[1];
const rows = (bone: string, table: readonly Row[], shift = (_y: number) => 0): St[] => table.map(([y, b, f, rw, e]) => H(bone, y, b + shift(y), f + shift(y), rw, e));

/* ---------------------------------------------------------------- stated tables per scale */

const COAT: Record<Scale, Row[]> = {
  100: [[0.94, -0.162, 0.118, 0.19, 2.5], [1.04, -0.15, 0.123, 0.181, 2.45], [1.12, -0.133, 0.125, 0.169, 2.4], [1.24, -0.13, 0.138, 0.175, 2.4], [1.33, -0.135, 0.142, 0.184, 2.35], [1.41, -0.13, 0.126, 0.21, 2.3], [1.448, -0.115, 0.104, 0.222, 2.25], [1.475, -0.09, 0.08, 0.19, 2.2], [1.5, -0.058, 0.057, 0.09, 2]],
  200: [[0.94, -0.162, 0.118, 0.19, 2.5], [1.08, -0.14, 0.124, 0.172, 2.4], [1.27, -0.132, 0.14, 0.179, 2.4], [1.41, -0.13, 0.126, 0.212, 2.3], [1.455, -0.11, 0.1, 0.22, 2.2], [1.495, -0.065, 0.062, 0.11, 2]],
  500: [[0.94, -0.162, 0.118, 0.19, 2.5], [1.15, -0.13, 0.128, 0.172, 2.4], [1.38, -0.134, 0.135, 0.2, 2.3], [1.45, -0.112, 0.102, 0.222, 2.2], [1.5, -0.06, 0.06, 0.1, 2]],
};
const SKIRT: Record<Scale, Row[]> = {
  100: [[1.0, -0.153, 0.113, 0.181, 2.8], [0.93, -0.169, 0.121, 0.197, 2.8], [0.76, -0.164, 0.126, 0.203, 2.8], [0.578, -0.156, 0.133, 0.21, 2.8]],
  200: [[1.0, -0.153, 0.113, 0.181, 2.8], [0.9, -0.17, 0.12, 0.198, 2.8], [0.578, -0.156, 0.133, 0.21, 2.8]],
  500: [[1.0, -0.153, 0.113, 0.181, 2.8], [0.88, -0.17, 0.121, 0.199, 2.8], [0.578, -0.156, 0.133, 0.21, 2.8]],
};
const THIGH: Record<Scale, St[]> = {
  100: [st(0.0, -0.006, 0.012, 0.086, 0.095, 2.1), st(0.14, 0.004, 0.008, 0.08, 0.087), st(0.3, 0.008, 0.002, 0.064, 0.07), st(0.4, 0.006, 0, 0.056, 0.06)],
  200: [st(0.0, -0.006, 0.012, 0.086, 0.095, 2.1), st(0.2, 0.006, 0.006, 0.074, 0.081), st(0.4, 0.006, 0, 0.057, 0.061)],
  500: [st(0.0, -0.006, 0.012, 0.086, 0.095, 2.1), st(0.4, 0.006, 0, 0.058, 0.062)],
};
const SHANK: Record<Scale, St[]> = {
  100: [st(0.0, 0.006, 0, 0.056, 0.06), st(0.14, -0.008, 0, 0.055, 0.062), st(0.3, -0.003, 0, 0.051, 0.056), st(0.395, 0.004, 0, 0.052, 0.062, 2, -6)],
  200: [st(0.0, 0.006, 0, 0.056, 0.06), st(0.16, -0.008, 0, 0.055, 0.062), st(0.395, 0.002, 0, 0.052, 0.06)],
  500: [st(0.0, 0.006, 0, 0.057, 0.061), st(0.395, 0.0, 0, 0.054, 0.061)],
};
const BOOT: Record<Scale, Row[]> = {
  100: [[-0.072, 0.03, 0.08, 0.022, 2], [-0.055, 0.0, 0.148, 0.039, 2.3], [-0.01, 0.0, 0.152, 0.045, 2.3], [0.03, 0.0, 0.14, 0.047, 2.4], [0.08, 0.0, 0.09, 0.05, 2.5], [0.135, 0.0, 0.068, 0.051, 2.6]],
  200: [[-0.072, 0.0, 0.09, 0.03, 2], [-0.04, 0.0, 0.15, 0.044, 2.3], [0.03, 0.0, 0.14, 0.047, 2.4], [0.135, 0.0, 0.068, 0.051, 2.6]],
  500: [[-0.074, 0.0, 0.11, 0.04, 2.2], [0.0, 0.0, 0.152, 0.046, 2.3], [0.135, 0.0, 0.07, 0.051, 2.6]],
};
const TOE: Record<Scale, Row[]> = {
  100: [[0.115, 0.0, 0.07, 0.051, 2.6], [0.17, 0.0, 0.056, 0.046, 2.5], [0.2, 0.004, 0.046, 0.032, 2.2], [0.212, 0.012, 0.036, 0.016, 2]],
  200: [[0.115, 0.0, 0.07, 0.051, 2.6], [0.185, 0.0, 0.05, 0.042, 2.4], [0.212, 0.008, 0.038, 0.02, 2]],
  500: [[0.115, 0.0, 0.08, 0.051, 2.6], [0.212, 0.0, 0.077, 0.04, 2.3]],
};
const HEAD: Record<Scale, Row[]> = {
  100: [[1.496, 0.012, 0.052, 0.022, 2], [1.515, -0.04, 0.06, 0.046, 2.2], [1.545, -0.09, 0.063, 0.059, 2.2], [1.59, -0.12, 0.064, 0.066, 2.1], [1.644, -0.128, 0.071, 0.072, 2], [1.68, -0.12, 0.06, 0.066, 2], [1.706, -0.098, 0.033, 0.048, 2], [1.72, -0.06, -0.01, 0.018, 2]],
  200: [[1.497, -0.005, 0.054, 0.03, 2], [1.54, -0.088, 0.063, 0.058, 2.2], [1.61, -0.125, 0.064, 0.069, 2.1], [1.67, -0.124, 0.065, 0.07, 2], [1.71, -0.09, 0.024, 0.042, 2], [1.72, -0.055, -0.012, 0.016, 2]],
  500: [[1.497, -0.01, 0.054, 0.032, 2], [1.58, -0.118, 0.064, 0.066, 2.1], [1.665, -0.126, 0.068, 0.071, 2], [1.718, -0.07, 0.0, 0.03, 2]],
};
const THEAD: Record<Scale, Row[]> = {
  100: [[1.5, 0.206, 0.238, 0.026, 2], [1.52, 0.15, 0.248, 0.053, 2.2], [1.55, 0.112, 0.256, 0.066, 2.1], [1.6, 0.092, 0.254, 0.0675, 2], [1.645, 0.093, 0.245, 0.065, 2], [1.672, 0.11, 0.22, 0.05, 2], [1.684, 0.136, 0.194, 0.025, 2]],
  200: [[1.5, 0.195, 0.24, 0.032, 2], [1.54, 0.118, 0.255, 0.064, 2.1], [1.61, 0.09, 0.254, 0.068, 2], [1.665, 0.104, 0.23, 0.056, 2], [1.684, 0.14, 0.19, 0.024, 2]],
  500: [[1.5, 0.19, 0.24, 0.035, 2], [1.58, 0.094, 0.256, 0.068, 2], [1.67, 0.105, 0.228, 0.054, 2]],
};
const SUIT: Record<Scale, Row[]> = {
  100: [[1.152, -0.04, 0.035, 0.07, 2.2], [1.175, -0.06, 0.07, 0.104, 2.3], [1.24, -0.063, 0.094, 0.11, 2.3], [1.33, -0.046, 0.099, 0.106, 2.2], [1.42, -0.035, 0.087, 0.113, 2.3], [1.46, -0.03, 0.064, 0.1, 2.2], [1.488, -0.024, 0.04, 0.05, 2]],
  200: [[1.155, -0.045, 0.045, 0.08, 2.2], [1.22, -0.064, 0.09, 0.11, 2.3], [1.35, -0.044, 0.098, 0.107, 2.2], [1.445, -0.032, 0.075, 0.112, 2.3], [1.49, -0.024, 0.04, 0.05, 2]],
  500: [[1.155, -0.05, 0.05, 0.085, 2.2], [1.27, -0.06, 0.097, 0.11, 2.3], [1.44, -0.034, 0.078, 0.113, 2.3], [1.49, -0.024, 0.04, 0.05, 2]],
};

/* ---------------------------------------------------------------- body */

function body(scale: Scale): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => { for (const s of solids) out.push(s); for (const s of solids) out.push(mirrored(s, toRight, near)); };
  const wool = (id: string, bone: string, tone: number = TONE.coat): C => ({ id, bone, material: "wool", tone });
  const denim = (id: string, bone: string, tone: number = TONE.jeans): C => ({ id, bone, material: "denim", tone });
  const leather = (id: string, bone: string, tone: number = TONE.boot): C => ({ id, bone, material: "leather", tone });
  const glove = (id: string, bone: string, tone: number = TONE.glove): C => ({ id, bone, material: "leather-glove", tone });
  const k = scale === 100 ? 2 : 1;
  if (scale !== 500) out.push(loft(denim("jeans-seat", "pelvis"), rows("pelvis", scale === 100 ? [[0.8, -0.105, 0.075, 0.16, 2.3], [0.9, -0.146, 0.094, 0.178, 2.4], [1.03, -0.116, 0.096, 0.155, 2.3]] : [[0.8, -0.11, 0.075, 0.162, 2.3], [1.0, -0.13, 0.096, 0.165, 2.3]])));
  out.push(loft(wool("coat", "torso"), dense(rows("torso", COAT[scale], lean), k)));
  for (const side of ["l", "r"] as const) {
    const s = side === "l" ? 1 : -1;
    out.push(loft(wool(`coat-skirt-${side}`, `thigh-${side}`, side === "l" ? TONE.coatFar : TONE.coat), SKIRT[scale].map(([y, b, f, rw, e]) => H(`thigh-${side}`, y, b, f, rw - 0.066, e, s * 0.07 - FRAMES[`thigh-${side}`]!.o[2]))));
  }
  const leg: Json[] = [];
  const lx = FRAMES["foot-l"]!.o[0];
  leg.push(loft(denim("thigh-l", "thigh-l", TONE.jeansFar), dome(dense(THIGH[scale], k), 1, 0.035, scale === 500 ? 2 : 3)));
  leg.push(loft(denim("shank-l", "shank-l", TONE.jeansFar), dome(dense(SHANK[scale], k), -1, 0.035, scale === 500 ? 2 : 3)));
  leg.push(loft(leather("boot-l", "foot-l", TONE.bootFar), BOOT[scale].map(([x, b, t, rw, e]) => X("foot-l", lx + x, b, t, rw, e))));
  leg.push(loft(leather("boot-toe-l", "toe-l", TONE.bootFar), TOE[scale].map(([x, b, t, rw, e]) => X("toe-l", lx + x, b, t, rw, e))));
  if (scale === 100) leg.push(extrude(leather("boot-heel-l", "foot-l", TONE.sole), "zx", [[0, -0.074], [0.031, -0.06], [0.037, -0.03], [0.037, -0.004], [-0.037, -0.004], [-0.037, -0.03], [-0.031, -0.06]], -ANKLE_Y, -ANKLE_Y + 0.025));
  both(leg);
  const slv = (drift: number, side: 1 | -1): St[] => scale === 500 ? [st(-0.02, 0, -0.02 * side, 0.046, 0.054), st(0.316, -0.004, drift, 0.05, 0.055)] : scale === 200 ? [st(-0.022, 0, -0.02 * side, 0.044, 0.052), st(0.05, 0.002, 0.006 * side + drift * 0.15, 0.059, 0.065), st(0.316, -0.004, drift, 0.049, 0.054)] : [st(-0.025, 0, -0.02 * side, 0.042, 0.05), st(0.045, 0.002, 0.006 * side + drift * 0.14, 0.06, 0.066), st(0.17, 0.003, drift * 0.54, 0.054, 0.06), st(0.316, -0.004, drift, 0.049, 0.054)];
  const fore = (drift: number): St[] => scale === 500 ? [st(-0.01, -0.004, 0, 0.05, 0.055), st(0.24, 0, drift * 0.97, 0.042, 0.043)] : [st(-0.01, -0.004, 0, 0.049, 0.054), st(0.12, 0, drift * 0.48, 0.045, 0.048), st(0.235, 0, drift * 0.95, 0.042, 0.043)];
  out.push(loft(wool("sleeve-l", "upper-arm-l", TONE.coatFar), dome(slv(0.04, 1), 1, 0.04, 2)));
  out.push(loft(wool("forearm-l", "forearm-l", TONE.coatFar), dome(fore(0.03), -1, 0.04, 2)));
  out.push(loft(wool("sleeve-r", "upper-arm-r"), dome(slv(-0.03, -1), 1, 0.04, 2)));
  out.push(loft(wool("forearm-r", "forearm-r"), dome(fore(0.05), -1, 0.04, 2)));
  if (scale === 500) {
    out.push(loft(glove("glove-l", "hand-l", TONE.gloveFar), [st(-0.01, 0.004, 0, 0.038, 0.042), st(0.06, 0.008, 0, 0.039, 0.05), st(0.13, 0.004, -0.004, 0.038, 0.042), st(0.15, 0.002, -0.004, 0.03, 0.03)]));
    out.push(loft(glove("glove-r", "hand-r"), [st(-0.01, 0.006, 0, 0.04, 0.036), st(0.06, 0.008, 0, 0.044, 0.038), st(0.13, 0.01, 0, 0.042, 0.03), st(0.15, 0.012, 0, 0.03, 0.02)]));
  } else {
    const kk = scale === 100;
    out.push(loft(glove("glove-l", "hand-l", TONE.gloveFar), [st(-0.012, 0, 0, 0.024, 0.031), st(0.05, 0.002, -0.001, 0.02, 0.043), st(0.1, 0.0, -0.004, 0.016, 0.042), ...(kk ? [st(0.135, -0.004, -0.018, 0.017, 0.038), st(0.15, -0.008, -0.026, 0.012, 0.03)] : [st(0.14, -0.006, -0.022, 0.015, 0.034)])]));
    out.push(capsule(glove("glove-l-thumb", "hand-l", TONE.gloveFar), [0.02, 0.03, -0.01], [0.09, 0.062, -0.022], scale === 100 ? 0.012 : 0.015, scale === 100 ? 0.0095 : 0.015));
    out.push(loft(glove("glove-r", "hand-r"), [st(-0.012, 0.004, 0, 0.03, 0.019), st(0.05, 0.009, 0, 0.042, 0.016), st(0.1, 0.009, 0, 0.042, 0.016), ...(kk ? [st(0.14, 0.012, 0, 0.04, 0.012), st(0.16, 0.016, 0, 0.034, 0.009)] : [st(0.155, 0.014, 0, 0.038, 0.015)])]));
    out.push(capsule(glove("glove-r-thumb", "hand-r"), [0.022, 0.006, -0.038], [0.086, 0.03, -0.06], scale === 100 ? 0.012 : 0.015, scale === 100 ? 0.0095 : 0.015));
  }
  const skin = (id: string, bone = "head", tone: number = TONE.skin): C => ({ id, bone, material: "skin", tone });
  const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
  const level = aboutW(90 - boneDeg("head"));
  out.push(loft(skin("head"), dome(dense(rows("head", HEAD[scale]), scale === 100 ? 2 : 3), -1, 0.004, 2)));
  const edge = 1.655;
  const capRows: Row[] = scale === 500 ? [[edge, -0.14, 0.08, 0.084, 2], [1.7, -0.124, 0.05, 0.072, 2], [1.73, -0.06, -0.01, 0.028, 2]] : scale === 200 ? [[edge, -0.138, 0.081, 0.082, 2], [1.69, -0.13, 0.068, 0.078, 2], [1.718, -0.105, 0.035, 0.056, 2], [1.731, -0.06, -0.012, 0.022, 2]] : [[edge - 0.003, -0.144, 0.087, 0.088, 2], [edge + 0.04, -0.142, 0.08, 0.086, 2], [1.7, -0.13, 0.066, 0.078, 2], [1.72, -0.106, 0.034, 0.056, 2], [1.731, -0.064, -0.01, 0.026, 2]];
  out.push(loft({ id: "beanie", bone: "head", material: "knit-hat", tone: TONE.beanie }, rows("head", capRows)));
  out.push(loft({ id: "scarf", bone: "torso", material: "knit-hat", tone: TONE.scarf }, rows("torso", scale === 500 ? [[1.45, -0.125, 0.04, 0.09, 2.2], [1.54, -0.12, 0.045, 0.085, 2.2]] : [[1.452, -0.124, 0.036, 0.088, 2.2], [1.495, -0.135, 0.053, 0.095, 2.2], [1.547, -0.113, 0.037, 0.076, 2.1]])));
  if (scale === 100) {
    out.push(loft({ id: "scarf-end", bone: "torso", material: "knit-hat", tone: TONE.scarf - 0.03 }, [1.475, 1.38, 1.29].map((y, j) => H("torso", y, coatX(y, 0.04, 1) - 0.006, coatX(y, 0.04, 1) + 0.016, 0.034, 2.3, 0.04 + j * 0.004))));
    out.push(loft(wool("collar", "torso", 0.27), [H("torso", 1.465, -0.108 + lean(1.465), 0.084 + lean(1.465), 0.105, 2.2), H("torso", 1.505, -0.108 + lean(1.505), 0.058 + lean(1.505), 0.096, 2.2)]));
    out.push(loft(skin("neck", "head", 0.76), rows("head", [[1.43, -0.106, 0.012, 0.062, 2], [1.586, -0.09, 0.02, 0.054, 2]])));
  }
  if (scale !== 500) {
    out.push(loft(skin("head-nose"), [[1.632, 0.072, 0.009], [1.596, 0.09, 0.0135], [1.582, 0.086, 0.016]].map(([y, f, rw]) => H("head", y!, 0.054, f!, rw!, 2)).reverse()));
    for (const side of [1, -1]) out.push(ellipsoid(skin(`head-ear-${side > 0 ? "l" : "r"}`, "head", 0.74), [0.03, 0.016, scale === 100 ? 0.0075 : 0.015], Hd(-0.038, 1.612, side * (scale === 100 ? 0.072 : 0.07)), level));
  }
  out.push(...toddler(scale));
  return out;
}

function toddler(scale: Scale): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => { for (const s of solids) out.push(s); for (const s of solids) out.push(mirrored(s, toRight, near)); };
  const suit = (id: string, bone: string, tone: number = TONE.suit): C => ({ id, bone, material: "padded-suit", tone });
  const k = scale === 100 ? 2 : 1;
  out.push(loft(suit("toddler-suit", "toddler-torso"), dome(dense(SUIT[scale].map(([y, b, f, rw, e]) => H("toddler-torso", y, spine(y) + b, spine(y) + f, rw, e)), k), -1, 0.012, 2)));
  const limb: Json[] = [];
  const lo = scale === 500;
  limb.push(loft(suit("toddler-thigh-l", "toddler-thigh-l", TONE.suitFar), dome(lo ? [st(-0.01, 0.004, 0, 0.0616, 0.0638), st(0.185, -0.002, 0, 0.0462, 0.0462)] : [st(-0.01, 0.004, 0, 0.0616, 0.0638), st(0.1, 0.0, 0, 0.0539, 0.0539), st(0.185, -0.002, 0, 0.0451, 0.0451)], 1, 0.03, 2)));
  const shankEnd = scale === 100 ? 0.158 : 0.17;
  limb.push(loft(suit("toddler-shank-l", "toddler-shank-l", TONE.suitFar), dome(lo ? [st(-0.01, 0, 0, 0.0462, 0.0462), st(0.17, 0, 0, 0.0418, 0.044)] : [st(-0.01, 0, 0, 0.0451, 0.0451), st(0.07, -0.005, 0, 0.044, 0.0462), st(shankEnd, 0, 0, 0.0407, 0.0429)], -1, 0.03, 2)));
  if (scale === 100) limb.push(loft({ id: "toddler-boot-shaft-l", bone: "toddler-shank-l", material: "rubber", tone: TONE.wellyFar }, [st(0.12, 0, 0, 0.0365, 0.0385), st(0.165, 0, 0, 0.036, 0.039)]));
  limb.push(loft({ id: "toddler-boot-l", bone: "toddler-foot-l", material: "rubber", tone: TONE.wellyFar }, scale === 500 ? [st(-0.045, -0.02, 0, 0.034, 0.04), st(0.04, -0.026, 0, 0.038, 0.038), st(0.113, -0.034, 0, 0.026, 0.026)] : [st(-0.045, -0.024, 0, 0.026, 0.03), st(-0.012, -0.014, 0, 0.037, 0.044), st(0.055, -0.028, 0, 0.038, 0.028), st(0.113, -0.035, 0, 0.016, 0.016)]));
  limb.push(loft(suit("toddler-sleeve-l", "toddler-arm-l", TONE.suitFar), dome(lo ? [st(-0.012, 0, -0.01, 0.0425, 0.0448), st(0.135, 0, 0.03, 0.0448, 0.0448)] : [st(-0.012, 0, -0.012, 0.0354, 0.0401), st(0.04, 0.001, 0.006, 0.0437, 0.0448), st(0.135, 0, 0.03, 0.0389, 0.0401)], -1, 0.02, 2)));
  limb.push(loft(suit("toddler-forearm-l", "toddler-forearm-l", TONE.suitFar), dome(lo ? [st(-0.01, 0, 0, 0.0448, 0.0448), st(0.1, 0, -0.03, 0.0448, 0.0448)] : [st(-0.01, 0, 0, 0.0378, 0.0389), st(0.1, 0, -0.03, 0.0342, 0.0342)], -1, 0.02, 2)));
  if (!lo) limb.push(loft({ id: "toddler-mitten-l", bone: "toddler-forearm-l", material: "knit-hat", tone: TONE.mitten }, scale === 100 ? [st(0.098, 0, -0.03, 0.026, 0.024), st(0.135, 0.004, -0.034, 0.032, 0.02), st(0.172, 0.004, -0.032, 0.017, 0.015)] : [st(0.095, 0, -0.032, 0.03, 0.03), st(0.165, 0.004, -0.034, 0.024, 0.024)]));
  else limb.push(ellipsoid({ id: "toddler-mitten-l", bone: "toddler-forearm-l", material: "knit-hat", tone: TONE.mitten }, [0.045, 0.038, 0.038], [0.13, 0.004, -0.032]));
  both(limb);
  const skin = (id: string, tone: number = TONE.tskin): C => ({ id, bone: "toddler-head", material: "skin", tone });
  const tz = FRAMES["toddler-head"]!.o[2];
  const P = (x: number, y: number, z = 0): V3 => L("toddler-head", [x, y, tz + z]);
  const level = aboutW(90 - boneDeg("toddler-head"));
  out.push(loft(skin("toddler-head"), dome(dense(THEAD[scale].map(([y, b, f, rw, e]) => H("toddler-head", y, b, f, rw, e)), scale === 100 ? 2 : 3), -1, 0.004, 2)));
  const hatRows: Row[] = scale === 500 ? [[1.6, 0.077, 0.27, 0.083, 2], [1.66, 0.085, 0.256, 0.078, 2], [1.7, 0.125, 0.205, 0.04, 2]] : scale === 200 ? [[1.603, 0.076, 0.27, 0.083, 2], [1.645, 0.08, 0.26, 0.079, 2], [1.678, 0.096, 0.235, 0.062, 2], [1.698, 0.13, 0.198, 0.03, 2]] : [[1.602, 0.073, 0.273, 0.085, 2], [1.633, 0.073, 0.27, 0.085, 2], [1.66, 0.083, 0.252, 0.075, 2], [1.684, 0.1, 0.228, 0.058, 2], [1.699, 0.135, 0.193, 0.028, 2]];
  out.push(loft({ id: "toddler-hat", bone: "toddler-head", material: "knit-hat", tone: TONE.hat }, hatRows.map(([y, b, f, rw, e]) => H("toddler-head", y, b, f, rw, e))));
  if (scale !== 500) out.push(ellipsoid({ id: "toddler-pompom", bone: "toddler-head", material: "knit-hat", tone: TONE.hat + 0.1 }, [0.021, 0.019, 0.021], P(0.163, 1.705), level));
  if (scale === 100) {
    out.push(ellipsoid(skin("toddler-nose"), [0.0095, 0.0085, 0.0095], P(0.258, 1.566), level));
    for (const side of [1, -1]) out.push(ellipsoid(skin(`toddler-ear-${side > 0 ? "l" : "r"}`, 0.8), [0.021, 0.012, 0.0075], P(0.16, 1.585, side * 0.068), level));
    out.push(loft(skin("toddler-neck", 0.8), [H("toddler-head", 1.47, 0.12, 0.2, 0.04, 2), H("toddler-head", 1.51, 0.122, 0.205, 0.04, 2)]));
  }
  return out;
}

/* ---------------------------------------------------------------- thermal */

function thermal(scale: Scale, b: readonly Json[]): Json[] {
  const out: Json[] = [];
  for (const s of b) {
    const region = regionOf(s);
    if (/^(neck|toddler-neck)$/.test(s.id)) continue;
    if (s.id === "head" || s.id === "toddler-head") {
      const edge = s.id === "head" ? 1.655 : 1.605;
      const band = s.sections.filter((q: Json) => heightOf(s.bone, q) >= 1.495 && heightOf(s.bone, q) <= edge + (s.id === "head" ? 0.004 : 0.012));
      out.push({ ...s, id: `th-${s.id}`, sections: band.length >= 2 ? band : s.sections.slice(1, 3), capStart: false, capEnd: false, region });
      continue;
    }
    out.push(thOf(s.id, region, OUTER[region]));
  }
  out.push(...emitters(scale === 100 ? 1.9 : scale === 200 ? 3.8 : 9.4));
  return out;
}

/* ---------------------------------------------------------------- structure */

const BONE = { material: "bone", tone: 0.62 };
const T = (x: number, y: number, z = 0): V3 => L("torso", [x + lean(y), y, z]);
const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
const ring = (rd: number, r: number): [number, number][] => Array.from({ length: 9 }, (_, k) => [r * Math.cos((k * Math.PI) / 4), rd + r * Math.sin((k * Math.PI) / 4)] as [number, number]);

function structure(scale: Scale): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const f = scale === 100 ? 0.0075 : scale === 200 ? 0.015 : 0.0375;
  const r = (x: number): number => Math.max(x, f);
  const level = aboutW(90 - boneDeg("head"));
  const spine: [number, number][] = scale === 500 ? [[-0.04, 1.012], [-0.05, 1.26], [-0.045, 1.484]] : scale === 200 ? [[-0.04, 1.012], [-0.016, 1.11], [-0.06, 1.3], [-0.045, 1.484]] : [[-0.04, 1.012], [-0.014, 1.11], [-0.045, 1.24], [-0.072, 1.37], [-0.045, 1.484]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), [r(0.018)]), "torso");
  const hoops: [number, number, number, number][] = scale === 500 ? [] : scale === 200 ? [[1.22, 0.004, 0.09, 0.126], [1.36, -0.006, 0.08, 0.116]] : [[1.19, 0.005, 0.088, 0.122], [1.26, 0.004, 0.094, 0.131], [1.33, -0.002, 0.09, 0.126], [1.4, -0.012, 0.068, 0.1]];
  hoops.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-hoop-${i + 1}`, bone: "torso", ...BONE }, ring(rd, r(0.008)), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  if (scale === 500) m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(-0.02, 1.44, 0.15), T(-0.02, 1.44, -0.15), f, f), "torso");
  else for (const side of [1, -1]) m(capsule({ id: `clavicle-${side > 0 ? "l" : "r"}`, bone: "torso", ...BONE }, T(0.05, 1.425, side * 0.022), T(-0.012, 1.448, side * 0.165), r(0.008), r(0.008)), "torso");
  m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(-0.02, 0.97, 0.1), P(-0.02, 0.97, -0.1), r(0.016), r(0.016)), "pelvis");
  if (scale !== 500) m(capsule({ id: "sacrum", bone: "pelvis", ...BONE }, P(-0.04, 1.01), P(-0.07, 0.92), r(0.022), r(0.012)), "pelvis");
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, scale === 500 ? [0.07, 0.08, 0.06] : [0.068, 0.088, 0.064], Hd(-0.025, scale === 500 ? 1.63 : 1.64), level), "head");
  if (scale !== 500) m(capsule({ id: "jaw", bone: "head", ...BONE }, Hd(-0.02, 1.58), Hd(0.045, 1.512), r(0.01), r(0.01)), "head");
  m(capsule({ id: "cervical-spine", bone: "head", ...BONE }, Hd(-0.065, 1.486), Hd(-0.035, 1.58), r(0.012), r(0.012)), "head");
  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0.0, 0, 0.03], [0.39, 0, 0], r(0.014), r(0.016)), "thigh-l");
  lm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.01, 0.004, 0], [0.405, 0.002, 0], r(0.014), r(0.012)), "shank-l");
  lm(capsule({ id: "foot-bones-l", bone: "foot-l", ...BONE }, L("foot-l", [FRAMES["foot-l"]!.o[0] - 0.045, 0.06, FRAMES["foot-l"]!.o[2]]), L("foot-l", [FRAMES["foot-l"]!.o[0] + 0.13, scale === 500 ? 0.0375 : 0.035, FRAMES["foot-l"]!.o[2]]), r(0.016), r(0.014)), "foot-l");
  lm(capsule({ id: "toe-bones-l", bone: "toe-l", ...BONE }, L("toe-l", [FRAMES["foot-l"]!.o[0] + 0.13, scale === 500 ? 0.0375 : 0.035, FRAMES["foot-l"]!.o[2]]), L("toe-l", [FRAMES["foot-l"]!.o[0] + 0.19, scale === 500 ? 0.0375 : 0.03, FRAMES["foot-l"]!.o[2]]), r(0.012), r(0.01)), "toe-l");
  if (scale !== 500) for (const [b, at, rr] of [["thigh-l", [0, 0, 0], 0.022], ["shank-l", [0, 0, 0], 0.018]] as [string, V3, number][]) limb.push(jointBall(b, at, r(rr)));
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  for (const [side, up, fore] of [["l", 0.04, 0.03], ["r", -0.03, 0.05]] as const) {
    m(capsule({ id: `humerus-${side}`, bone: `upper-arm-${side}`, ...BONE }, [0.005, 0, 0], [0.305, 0, up], r(0.012), r(0.012)), `upper-arm-${side}`);
    m(capsule({ id: `forearm-bones-${side}`, bone: `forearm-${side}`, ...BONE }, [0.005, 0, 0], [0.24, 0, fore], r(0.01), r(0.01)), `forearm-${side}`);
    m(capsule({ id: `hand-bones-${side}`, bone: `hand-${side}`, ...BONE }, [0.01, side === "r" ? 0.008 : 0, 0], [scale === 500 ? 0.1 : 0.12, side === "r" ? 0.01 : -0.003, side === "l" ? -0.004 : 0], r(0.009), r(0.008)), `hand-${side}`);
    if (scale !== 500) { out.push(jointBall(`upper-arm-${side}`, [0, 0, 0], r(0.022))); out.push(jointBall(`forearm-${side}`, [0, 0, 0], r(0.014))); }
  }
  if (scale !== 500) out.push(jointBall("torso", T(-0.04, 1.012), r(0.016)));
  for (const b of ["pelvis", "torso", "head"]) out.push(massBall(b, scale === 500 ? 2.6 : 1));
  const tz = FRAMES["toddler-torso"]!.o[2];
  const Tt = (dx: number, y: number, z = 0): V3 => L("toddler-torso", [spineX(y) + dx, y, tz + z]);
  m(tube({ id: "toddler-spine", bone: "toddler-torso", ...BONE }, (scale === 500 ? [[-0.03, 1.2], [-0.026, 1.47]] : [[-0.035, 1.2], [-0.024, 1.33], [-0.026, 1.47]]).map(([x, y]) => Tt(x!, y!)), [r(0.011)]), "toddler-torso");
  if (scale === 100) m(revolve({ id: "toddler-rib-hoop", bone: "toddler-torso", ...BONE }, ring(0.06, r(0.006)), Tt(0.025, 1.35), [1, 0, 0], { lateralScale: 1.3 }), "toddler-torso");
  m(capsule({ id: "toddler-girdles", bone: "toddler-torso", ...BONE }, Tt(0.0, 1.23, 0.055), Tt(0.0, 1.23, -0.055), r(0.009), r(0.009)), "toddler-torso");
  if (scale !== 500) m(capsule({ id: "toddler-shoulders", bone: "toddler-torso", ...BONE }, Tt(0.01, 1.445, 0.085), Tt(0.01, 1.445, -0.085), r(0.006), r(0.006)), "toddler-torso");
  const th = aboutW(90 - boneDeg("toddler-head"));
  const hz = FRAMES["toddler-head"]!.o[2];
  m(ellipsoid({ id: "toddler-skull", bone: "toddler-head", ...BONE }, scale === 500 ? [0.06, 0.07, 0.05] : [0.07, 0.078, 0.06], L("toddler-head", [0.172, 1.605, hz]), th), "toddler-head");
  const tl: Json[] = [];
  const tm = (solid: Json, member: string): void => { tl.push(part(solid, member, "member")); };
  tm(capsule({ id: "toddler-femur-l", bone: "toddler-thigh-l", ...BONE }, [0, 0, 0], [0.172, -0.002, 0], r(0.01), r(0.01)), "toddler-thigh-l");
  tm(capsule({ id: "toddler-tibia-l", bone: "toddler-shank-l", ...BONE }, [0.008, 0.002, 0], [0.165, 0.0, 0], r(0.009), r(0.008)), "toddler-shank-l");
  tm(capsule({ id: "toddler-foot-bones-l", bone: "toddler-foot-l", ...BONE }, [-0.025, -0.018, 0], [0.085, -0.028, 0], r(0.011), r(0.009)), "toddler-foot-l");
  tm(capsule({ id: "toddler-humerus-l", bone: "toddler-arm-l", ...BONE }, [0.004, 0, 0], [0.128, 0, 0.026], r(0.008), r(0.008)), "toddler-arm-l");
  tm(capsule({ id: "toddler-forearm-bones-l", bone: "toddler-forearm-l", ...BONE }, [0.004, 0, 0], [scale === 500 ? 0.12 : 0.15, 0.002, -0.03], r(0.007), r(0.007)), "toddler-forearm-l");
  for (const s of tl) out.push(s);
  for (const s of tl) out.push(mirrored(s, toRight));
  out.push(jointBall("toddler-torso", [0, 0, 0], r(0.013), "joint-toddler-seat"));
  out.push(massBall("toddler-torso", scale === 500 ? 2.2 : 0.85));
  return out;
}
const spineX = spine;

export function coarseVolumes(): Json {
  const out: Json = {};
  for (const scale of [100, 200, 500] as Scale[]) {
    const b = body(scale);
    out[String(scale)] = { status: "authored", denominator: scale, body: b, thermal: thermal(scale, b), structure: structure(scale), decisions: [{ en: `1:${scale}: draft.`, de: `1:${scale}: Entwurf.` }] };
  }
  return out;
}
export { aboutU, compose };
