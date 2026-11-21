/** 🧍️ The parent and the toddler at 1:50, stated by hand: skull with jaw, nose, ears, eyes and lips, hair, scarf, quilted padded jacket (body on the trunk, skirt on the pelvis, sleeves with elastic cuffs), hands gripping the push bar with four fingers and the thumb, straight jeans with stacking folds, sneakers with their soles; the toddler in a padded snowsuit with hat and pompom.
 * Anthropometry: adult 1.75 m, 68 kg (see the reference table of the report); every solid carries `_r`, its thermal region.
 * @see ./r3-stroller-kit.ts */
import { B, C, E, L, P, T, X, capped, dense, mirror, type Json, type Row, type V3 } from "./r3-stroller-kit.ts";

const SKIN = 0.84, JACKET = 0.4, JEANS = 0.28, SHOE = 0.8, HAIR = 0.22;
const tag = (s: Json, r: string): Json => ({ ...s, _r: r });
const at = (rows: Row[], u: number): Row => { const d = dense(rows, 3); for (let i = 0; i + 1 < d.length; i++) if (u <= d[i + 1]![0]!) { const a = d[i]!, b = d[i + 1]!, k = Math.max(0, (u - a[0]!) / (b[0]! - a[0]!)); return a.map((x, j) => x + ((b[j] ?? x) - x) * k); } return d[d.length - 1]!; };
/** 🪡️ Quilting: the stated outline with a seam pulled in by `depth` at every seam station and the full baffle between. */
export function quilt(rows: Row[], seams: number[], depth: number): Row[] {
  const lo = rows[0]![0]!, hi = rows[rows.length - 1]![0]!;
  const us = new Set<number>(rows.map((r) => r[0]!).filter((u) => seams.every((s) => Math.abs(u - s) > 0.018)));
  const ss = [...seams].sort((a, b) => a - b);
  for (const s of ss) us.add(s);
  for (let i = 0; i + 1 < ss.length; i++) { us.add(ss[i]! + 0.3 * (ss[i + 1]! - ss[i]!)); us.add(ss[i]! + 0.7 * (ss[i + 1]! - ss[i]!)); }
  return [...us].filter((u) => u >= lo && u <= hi).sort((a, b) => a - b).map((u) => { const r = at(rows, u); if (seams.includes(u)) { r[1] = r[1]! - depth; r[2] = r[2]! - depth; } return r; });
}
const polar = (c: [number, number], rho: number, deg: number, w: number): V3 => [c[0] + rho * Math.cos((deg * Math.PI) / 180), c[1] + rho * Math.sin((deg * Math.PI) / 180), w];

export const HEAD_ROWS: Row[] = [
  [0.0355, 0.02, 0.018, -0.07, 0, 2], [0.05, 0.042, 0.04, -0.05, 0, 2], [0.075, 0.058, 0.047, -0.041, 0, 2.1], [0.105, 0.066, 0.07, -0.022, 0, 2.2], [0.135, 0.07, 0.086, -0.004, 0, 2.2],
  [0.16, 0.074, 0.092, 0.004, 0, 2.3], [0.178, 0.0765, 0.0985, 0.004, 0, 2.3], [0.2, 0.0775, 0.0965, 0.006, 0, 2.3], [0.222, 0.072, 0.087, 0.01, 0, 2.3], [0.245, 0.058, 0.068, 0.01, 0, 2.2], [0.258, 0.04, 0.046, 0.01, 0, 2.1], [0.265, 0.012, 0.015, 0.01, 0, 2],
];

export function adult50(): Json[] {
  const out: Json[] = [];
  const both = (s: Json, r: string): void => { out.push(tag(s, r)); out.push(tag(mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone.replace(/-l$/, "-r")), r)); };

  out.push(tag(L("head", "head", "skin", SKIN, HEAD_ROWS, { k: 1 }), "head-skin"));
  out.push(tag(L("neck", "head", "skin", SKIN, [[-0.03, 0.062, 0.064, 0.002, 0, 2.2], [0.02, 0.058, 0.062, 0, 0, 2.1], [0.06, 0.056, 0.06, -0.002, 0, 2.1], [0.095, 0.054, 0.058, 0, 0, 2.1]]), "head-skin"));
  out.push(tag(P("nose", "head", "skin", SKIN, [[0.158, -0.084, 0, 0.0075, 0.007], [0.143, -0.091, 0, 0.0085, 0.011], [0.127, -0.098, 0, 0.0105, 0.015], [0.114, -0.1, 0, 0.0155, 0.014], [0.104, -0.092, 0, 0.0145, 0.008]]), "head-skin"));
  both(E("ear-l", "head", "skin", SKIN - 0.06, [0.143, 0.014, 0.079], [0.031, 0.017, 0.008], { role: "detail" }), "head-skin");
  both(E("eye-l", "head", "skin", 0.15, [0.152, -0.0845, 0.032], [0.006, 0.004, 0.012], { role: "detail" }), "head-skin");
  out.push(tag(E("lips", "head", "skin", 0.68, [0.083, -0.0895, 0], [0.006, 0.005, 0.022], { role: "detail" }), "head-skin"));
  out.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.163, 0.082, 0.04, 0.062, 0, 2.2], [0.185, 0.0865, 0.08, 0.032, 0, 2.2], [0.205, 0.0865, 0.09, 0.022, 0, 2.2], [0.224, 0.083, 0.092, 0.017, 0, 2.2], [0.242, 0.072, 0.088, 0.012, 0, 2.2], [0.258, 0.055, 0.067, 0.011, 0, 2.1], [0.27, 0.034, 0.041, 0.011, 0, 2], [0.277, 0.012, 0.015, 0.011, 0, 2]]), "hair"));
  out.push(tag(L("hair-fall", "head", "hair", HAIR, dense([[0.19, 0.088, 0.07, 0.03, 0, 2.2], [0.15, 0.09, 0.068, 0.038, 0, 2.3], [0.11, 0.088, 0.062, 0.044, 0, 2.3], [0.07, 0.084, 0.054, 0.05, 0, 2.3], [0.03, 0.08, 0.046, 0.056, 0, 2.3], [-0.005, 0.077, 0.038, 0.064, 0, 2.2], [-0.03, 0.07, 0.03, 0.07, 0, 2.2], [-0.042, 0.056, 0.022, 0.072, 0, 2.1], [-0.05, 0.03, 0.012, 0.073, 0, 2]], 1)), "hair"));
  out.push(tag(L("scarf", "head", "cotton-clothing", 0.62, [[-0.022, 0.066, 0.07, 0.004, 0, 2.2, -12], [-0.008, 0.082, 0.086, 0.004, 0, 2.2, -12], [0.01, 0.088, 0.092, 0.003, 0, 2.2, -12], [0.026, 0.08, 0.084, 0.002, 0, 2.2, -14], [0.036, 0.076, 0.08, 0.002, 0, 2.2, -14], [0.05, 0.083, 0.087, 0.002, 0, 2.2, -16], [0.064, 0.08, 0.084, 0.002, 0, 2.2, -16], [0.076, 0.066, 0.07, 0.002, 0, 2.2, -16]]), "torso-clothing"));

  const body: Row[] = [[-0.02, 0.181, 0.137, -0.004, 0, 2.5], [0.05, 0.18, 0.135, -0.006, 0, 2.5], [0.15, 0.182, 0.135, -0.008, 0, 2.5], [0.26, 0.187, 0.138, -0.008, 0, 2.5], [0.33, 0.192, 0.13, -0.004, 0, 2.5], [0.39, 0.2, 0.116, 0, 0, 2.4], [0.43, 0.203, 0.104, 0.004, 0, 2.4], [0.452, 0.195, 0.09, 0.006, 0, 2.3], [0.468, 0.165, 0.078, 0.008, 0, 2.2], [0.482, 0.115, 0.07, 0.009, 0, 2.1], [0.494, 0.076, 0.064, 0.009, 0, 2]];
  out.push(tag(L("jacket-body", "torso", "cotton-clothing", JACKET, quilt(body, [0.03, 0.105, 0.18, 0.255, 0.33], 0.006)), "torso-clothing"));
  out.push(tag(L("jacket-collar", "torso", "cotton-clothing", 0.34, [[0.455, 0.08, 0.082, 0.008, 0, 2.2, 8], [0.475, 0.078, 0.082, 0.008, 0, 2.2, 10], [0.505, 0.074, 0.08, 0.01, 0, 2.2, 12], [0.522, 0.07, 0.076, 0.012, 0, 2.2, 12]]), "torso-clothing"));
  const skirt: Row[] = [[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.055, 0.188, 0.144, 0.004, 0, 2.5], [-0.035, 0.19, 0.146, 0.004, 0, 2.5], [0, 0.188, 0.144, 0.002, 0, 2.5], [0.04, 0.184, 0.141, 0, 0, 2.5], [0.08, 0.18, 0.137, -0.003, 0, 2.5], [0.11, 0.177, 0.134, -0.004, 0, 2.5], [0.14, 0.172, 0.124, -0.008, 0, 2.5], [0.175, 0.16, 0.106, -0.014, 0, 2.5]];
  out.push(tag(L("jacket-skirt", "pelvis", "cotton-clothing", JACKET, quilt(skirt, [-0.05, 0.03], 0.006)), "torso-clothing"));
  out.push(tag(L("jeans-seat", "pelvis", "cotton-clothing", JEANS, [[-0.108, 0.05, 0.042, 0.008, 0, 2.2], [-0.1, 0.085, 0.064, 0.008, 0, 2.3], [-0.07, 0.146, 0.09, 0.008, 0, 2.5], [-0.03, 0.168, 0.114, 0.008, 0, 2.6], [0.02, 0.166, 0.114, 0.005, 0, 2.6], [0.07, 0.156, 0.104, 0.001, 0, 2.6], [0.1, 0.15, 0.098, -0.002, 0, 2.6]], { k: 1 }), "leg-clothing"));

  const su: Row[] = [[0, 0.064, 0.074, 0, 0, 2.2], [0.05, 0.066, 0.072, 0, 0.003, 2.2], [0.13, 0.062, 0.066, 0.002, 0.002, 2.2], [0.21, 0.058, 0.06, 0.002, 0, 2.2], [0.28, 0.056, 0.058, 0, 0, 2.2], [0.3082, 0.055, 0.057, 0, 0, 2.2]];
  both(L("sleeve-upper-l", "upper-arm-l", "cotton-clothing", JACKET, capped(quilt(su, [0.09, 0.165, 0.24], 0.005), 0.042, 0.02)), "arm-clothing");
  const sl: Row[] = [[0, 0.048, 0.05, 0, 0, 2.2], [0.06, 0.054, 0.054, 0.002, 0, 2.2], [0.13, 0.051, 0.05, 0, 0, 2.2], [0.2, 0.045, 0.043, 0, 0, 2.2], [0.238, 0.043, 0.04, 0, 0, 2.2]];
  both(L("sleeve-lower-l", "forearm-l", "cotton-clothing", JACKET, capped(quilt(sl, [0.075, 0.15], 0.005), 0.045, 0)), "arm-clothing");
  both(L("cuff-l", "forearm-l", "cotton-clothing", 0.34, [[0.232, 0.037, 0.034, 0, 0, 2.2], [0.25, 0.036, 0.032, 0, 0, 2.2], [0.278, 0.035, 0.031, 0, 0, 2.2]]), "arm-clothing");

  const bar: [number, number] = [0.0746, 0];
  both(P("palm-l", "hand-l", "skin", SKIN, [[-0.006, 0, 0, 0.028, 0.019, 2.4], [0.025, 0.01, 0, 0.034, 0.017, 2.5], [0.055, 0.024, 0, 0.04, 0.015, 2.6], [0.085, 0.034, 0, 0.042, 0.014, 2.6], [0.099, 0.034, 0, 0.04, 0.012, 2.4]]), "hand-skin");
  const finger = (i: number, w: number, r: number, last: number): Json => { const a = [64, 35, 0, -40, -85, -125, -155].filter((x) => x >= last), rho = [0.04, 0.033, 0.028, 0.027, 0.026, 0.026, 0.024]; return T(`finger-l-${i}`, "hand-l", "skin", SKIN - 0.03, a.map((d, j) => polar(bar, rho[j]! + (r - 0.009), d, w)), a.map((_, j) => r * (1 - 0.035 * j))); };
  both(finger(1, -0.03, 0.009, -155), "hand-skin");
  both(finger(2, -0.01, 0.0095, -155), "hand-skin");
  both(finger(3, 0.01, 0.009, -155), "hand-skin");
  both(finger(4, 0.029, 0.008, -125), "hand-skin");
  both(T("thumb-l", "hand-l", "skin", SKIN - 0.03, [[0.012, 0.004, -0.03], [0.034, -0.012, -0.046], [0.058, -0.023, -0.05], [0.078, -0.027, -0.049], [0.095, -0.022, -0.046]], [0.013, 0.012, 0.0105, 0.0095, 0.008]), "hand-skin");

  const th: Row[] = [[0, 0.076, 0.088, -0.006, 0.002, 2.2], [0.08, 0.088, 0.098, -0.006, 0.004, 2.3], [0.16, 0.084, 0.092, -0.002, 0.002, 2.3], [0.25, 0.074, 0.08, 0.003, 0, 2.2], [0.34, 0.064, 0.069, 0.004, 0, 2.2], [0.4, 0.06, 0.066, 0.004, 0, 2.2], [0.4276, 0.059, 0.065, 0.003, 0, 2.2]];
  both(L("jeans-thigh-l", "thigh-l", "cotton-clothing", JEANS, capped(dense(th, 1), 0.045, 0.02)), "leg-clothing");
  const sh: Row[] = [[0, 0.058, 0.064, 0.003, 0, 2.2], [0.05, 0.059, 0.065, 0, 0, 2.2], [0.13, 0.061, 0.068, -0.006, 0, 2.2], [0.24, 0.058, 0.062, -0.006, 0, 2.1], [0.33, 0.056, 0.059, -0.004, 0, 2.1], [0.385, 0.055, 0.058, -0.002, 0, 2.1], [0.4, 0.059, 0.062, -0.002, 0, 2.1], [0.414, 0.055, 0.058, -0.001, 0, 2.1], [0.43, 0.059, 0.062, 0, 0, 2.1], [0.447, 0.056, 0.059, 0, 0, 2.1]];
  both(L("jeans-shank-l", "shank-l", "cotton-clothing", JEANS, capped(sh, 0.05, 0)), "leg-clothing");

  both(L("shoe-l", "foot-l", "leather-shoe", SHOE, capped([[-0.074, 0.02, 0.025, -0.037, 0, 2.4], [-0.06, 0.032, 0.03, -0.032, 0, 2.6], [-0.03, 0.036, 0.031, -0.031, 0, 2.6], [0, 0.038, 0.031, -0.031, 0, 2.6], [0.03, 0.041, 0.031, -0.031, 0, 2.6], [0.07, 0.046, 0.028, -0.034, 0, 2.6], [0.105, 0.05, 0.021, -0.041, 0, 2.6], [0.1293, 0.05, 0.0175, -0.0445, 0, 2.6]], 0, 0.012, 2)), "shoes");
  both(X("sole-l", "foot-l", "leather-shoe", 0.55, "zx", [[0, -0.076], [0.026, -0.07], [0.035, -0.045], [0.036, 0], [0.042, 0.05], [0.052, 0.1], [0.053, 0.1293], [-0.05, 0.1293], [-0.048, 0.09], [-0.04, 0.04], [-0.036, 0], [-0.035, -0.045], [-0.026, -0.07]], -0.0795, -0.054), "shoes");
  both(L("shoe-toe-l", "toe-l", "leather-shoe", SHOE, [[-0.012, 0.05, 0.0175, 0.035, 0, 2.6], [0.03, 0.048, 0.016, 0.034, 0, 2.6], [0.06, 0.04, 0.0135, 0.0305, 0, 2.5], [0.08, 0.026, 0.01, 0.026, 0, 2.3], [0.088, 0.012, 0.006, 0.023, 0, 2.2]]), "shoes");
  both(X("sole-toe-l", "toe-l", "leather-shoe", 0.55, "zx", [[0.053, -0.006], [0.052, 0.03], [0.045, 0.06], [0.03, 0.082], [0.008, 0.09], [-0.015, 0.088], [-0.036, 0.072], [-0.047, 0.04], [-0.05, -0.006]], 0, 0.022), "shoes");
  return out;
}

/** 👶️ The toddler (12 months, about 0.76 m, 10 kg) seated in the stroller, child-bone frame (u ahead, v up, hip joints at the origin), centred on w = 0. */
export function child50(): Json[] {
  const out: Json[] = [];
  const SUIT = 0.6, both = (s: Json, r: string): void => { out.push(tag(s, r)); out.push(tag(mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone), r)); };
  out.push(tag(P("child-trunk", "child", "snowsuit", SUIT, [[0.006, -0.058, 0, 0.06, 0.045, 2.3], [0.002, -0.035, 0, 0.09, 0.07, 2.4], [-0.012, 0.03, 0, 0.098, 0.078, 2.4], [-0.025, 0.11, 0, 0.096, 0.076, 2.4], [-0.036, 0.19, 0, 0.094, 0.07, 2.4], [-0.042, 0.238, 0, 0.082, 0.058, 2.3], [-0.044, 0.262, 0, 0.05, 0.04, 2.2]]), "child-clothing"));
  both(P("child-leg-l", "child", "snowsuit", SUIT, [[-0.02, -0.005, 0.05, 0.05, 0.05], [0.06, 0.002, 0.054, 0.052, 0.052], [0.13, 0.008, 0.059, 0.049, 0.049], [0.168, 0.004, 0.06, 0.047, 0.046], [0.19, -0.012, 0.06, 0.045, 0.044], [0.204, -0.038, 0.06, 0.043, 0.042], [0.216, -0.068, 0.06, 0.04, 0.04], [0.232, -0.095, 0.06, 0.037, 0.037], [0.248, -0.122, 0.06, 0.035, 0.035]]), "child-clothing");
  both(E("child-boot-l", "child", "snowsuit", 0.35, [0.283, -0.128, 0.06], [0.056, 0.03, 0.032], { rot: [0, 0, 0.1305, 0.9914] }), "child-clothing");
  both(T("child-arm-l", "child", "snowsuit", SUIT, [[-0.038, 0.228, 0.094], [-0.02, 0.19, 0.108], [0.022, 0.138, 0.116], [0.07, 0.128, 0.104], [0.122, 0.124, 0.088]], [0.034, 0.037, 0.034, 0.03, 0.026]), "child-clothing");
  both(E("child-hand-l", "child", "skin", 0.88, [0.15, 0.122, 0.078], [0.024, 0.018, 0.02]), "child-skin");
  out.push(tag(E("child-head", "child", "skin", 0.88, [-0.03, 0.345, 0], [0.08, 0.086, 0.064]), "child-skin"));
  out.push(tag(E("child-face", "child", "skin", 0.88, [-0.01, 0.302, 0], [0.058, 0.046, 0.057]), "child-skin"));
  out.push(tag(E("child-nose", "child", "skin", 0.84, [0.05, 0.318, 0], [0.008, 0.008, 0.009], { role: "detail" }), "child-skin"));
  both(E("child-eye-l", "child", "skin", 0.12, [0.044, 0.339, 0.026], [0.004, 0.006, 0.007], { role: "detail" }), "child-skin");
  out.push(tag(E("child-hat", "child", "snowsuit", 0.3, [-0.037, 0.371, 0], [0.087, 0.072, 0.071]), "child-clothing"));
  out.push(tag(E("child-pompom", "child", "snowsuit", 0.5, [-0.047, 0.452, 0], [0.024, 0.024, 0.024]), "child-clothing"));
  return out;
}
