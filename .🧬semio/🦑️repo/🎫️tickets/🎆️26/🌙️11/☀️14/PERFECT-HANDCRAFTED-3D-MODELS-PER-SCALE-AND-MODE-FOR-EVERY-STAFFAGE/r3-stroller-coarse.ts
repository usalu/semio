/** 🔭️ The 1:100, 1:200 and 1:500 lists of `humans/parent-with-stroller`, each stated anew for its smallest feature (15, 30, 75 mm): masses merge, sections simplify, nothing is a finer list with parts removed.
 * 1:100: no quilting, eyes or cuffs; the hand is a fist with one curled finger slab and the thumb; shoe and sole are one last; the wheels keep tyre, web disc and hub. 1:200: one solid per limb segment, a fist ellipsoid, the chassis as 30 mm tubes, disc wheels. 1:500: trunk, limbs, fist and shoe masses; the chassis as two slender true-size tube runs, seat, canopy halves, basket and disc wheels.
 * @see ./r3-stroller-s50.ts */
import { B, C, E, L, P, R, T, X, capped, mirror, qaxis, shiftW, st, th, r5, type Json, type Row, type V3 } from "./r3-stroller-kit.ts";
import { adultSkeleton } from "./r3-stroller-s50-structure.ts";
import { LAYERS, faceMask, outerPart } from "./r3-stroller-s50-thermal.ts";
import { strip, HINGE, J } from "./r3-stroller-s50-pram.ts";
import { DECISIONS } from "./r3-stroller-decisions.ts";

const ZC = 0.1925, SKIN = 0.84, JACKET = 0.4, JEANS = 0.28, SHOE = 0.8, HAIR = 0.22, ALU = 0.8, DARK = 0.3, FAB = 0.45, BONE = 0.93;
type Tagged = Json;
const tag = (s: Json, r: string): Json => ({ ...s, _r: r });
const lr = (out: Json[], s: Json, r: string): void => { out.push(tag(s, r)); out.push(tag(mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone.replace(/-l$/, "-r")), r)); };
const pramLr = (out: Json[], s: Json, r: string): void => { const m = mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone); out.push(tag(shiftW(s, ZC), r)); out.push(tag(shiftW(m, ZC), r)); };
const pram1 = (out: Json[], s: Json, r: string): void => { out.push(tag(shiftW(s, ZC), r)); };
const bare = (s: Json): Json => { const { _r, ...rest } = s; return rest; };
const refs = (tagged: Tagged[], skip: RegExp): Json[] => tagged.filter((s) => !skip.test(s.id)).map((s) => { const layer = LAYERS[s._r]?.at(-1)?.[0]; return outerPart(s, `th-${s.id}`, s._r, layer) ?? th(`th-${s.id}`, s.id, s._r, layer); });
const outlet = (id: string, bone: string, at: V3, radii: V3, region: string): Json => ({ ...E(`th-${id}`, bone, "still-air", 0.9, at, radii, { role: "detail" }), region, tags: [id, "emitter"] });
const outlets = (k: number): Json[] => { const f = 0.0039 * k; return [outlet("breath-nose", "head", [0.107, -0.1, 0], [f, f, Math.max(f, 0.011)], "head-skin"), outlet("breath-mouth", "head", [0.083, -0.092, 0], [f, f, Math.max(f, 0.0062)], "head-skin"), outlet("plume", "head", [0.234, 0.013, 0], [Math.max(f, 0.046), Math.max(f, 0.07), Math.max(f, 0.06)], "torso-clothing"), outlet("child-nose-breath", "child", [0.056, 0.313, ZC], [f, f, f], "child-skin"), outlet("child-mouth-breath", "child", [0.045, 0.288, ZC], [f, f, f], "child-skin")]; };
const fan = (id: string, bone: string, c: [number, number], from: number, loop: number[][], sweep = 40): Json => ({ ...R(id, bone, "stroller-textile", 0.4, loop, [0, 0, 1], [0, 0, 0]), sweepRad: r5((sweep * Math.PI) / 180), transform: { rotation: qaxis([0, 0, 1], from).map(r5), translation: [c[0], c[1], 0] } });
const wheelDisc = (out: Json[], bone: string, n: string, z: number, R0: number, half: number, hubR: number, hubHalf: number, parts: 1 | 3): void => {
  if (parts === 1) { pram1(out, R(`wheel-${n}`, bone, "rubber", 0.14, [[-half, 0], [-half, R0 - 0.01], [-half * 0.5, R0], [half * 0.5, R0], [half, R0 - 0.01], [half, 0]], [0, 0, 1], [0, 0, z]), "tyres"); return; }
  const t = R0 * 0.2;
  pram1(out, R(`tyre-${n}`, bone, "rubber", 0.12, [[-half, R0 - t], [-half, R0 - 0.008], [-half * 0.5, R0], [half * 0.5, R0], [half, R0 - 0.008], [half, R0 - t], [-half, R0 - t]], [0, 0, 1], [0, 0, z]), "tyres");
  pram1(out, R(`web-${n}`, bone, "rubber", 0.65, [[-0.008, 0], [-0.008, R0 - t + 0.003], [0.008, R0 - t + 0.003], [0.008, 0]], [0, 0, 1], [0, 0, z]), "tyres");
  pram1(out, R(`hub-${n}`, bone, "rubber", 0.5, [[-hubHalf, 0], [-hubHalf, hubR], [hubHalf, hubR], [hubHalf, 0]], [0, 0, 1], [0, 0, z]), "tyres");
};
const HEAD100: Row[] = [[0.036, 0.022, 0.02, -0.068, 0, 2], [0.06, 0.05, 0.046, -0.046, 0, 2], [0.1, 0.066, 0.068, -0.024, 0, 2.2], [0.145, 0.073, 0.092, 0, 0, 2.3], [0.19, 0.0775, 0.0975, 0.004, 0, 2.3], [0.228, 0.07, 0.082, 0.008, 0, 2.3], [0.254, 0.046, 0.054, 0.01, 0, 2.1], [0.265, 0.014, 0.017, 0.01, 0, 2]];

function body100(): Tagged[] {
  const o: Tagged[] = [];
  o.push(tag(L("head", "head", "skin", SKIN, HEAD100), "head-skin"));
  o.push(tag(L("neck", "head", "skin", SKIN, [[-0.03, 0.062, 0.064, 0.002, 0, 2.2], [0.03, 0.057, 0.061, 0, 0, 2.1], [0.095, 0.054, 0.058, 0, 0, 2.1]]), "head-skin"));
  o.push(tag(P("nose", "head", "skin", SKIN, [[0.166, -0.083, 0, 0.008, 0.009], [0.128, -0.098, 0, 0.014, 0.017], [0.104, -0.09, 0, 0.014, 0.009]]), "head-skin"));
  lr(o, E("ear-l", "head", "skin", SKIN - 0.06, [0.143, 0.014, 0.078], [0.03, 0.017, 0.009], { role: "detail" }), "head-skin");
  o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.118, 0.077, 0.03, 0.074, 0, 2.2], [0.162, 0.085, 0.046, 0.064, 0, 2.2], [0.186, 0.087, 0.08, 0.032, 0, 2.2], [0.224, 0.083, 0.092, 0.017, 0, 2.2], [0.25, 0.064, 0.078, 0.011, 0, 2.1], [0.27, 0.036, 0.043, 0.011, 0, 2], [0.277, 0.014, 0.017, 0.011, 0, 2]]), "hair"));
  o.push(tag(P("hair-back", "head", "hair", HAIR, [[0.17, 0.09, 0, 0.078, 0.03, 2.6], [0.09, 0.08, 0, 0.098, 0.036, 2.8], [0.005, 0.078, 0, 0.102, 0.03, 2.8], [-0.042, 0.086, 0, 0.086, 0.014, 2.6]]), "hair"));
  o.push(tag(L("scarf", "head", "cotton-clothing", 0.62, [[-0.02, 0.068, 0.072, 0.004, 0, 2.2, -12], [0.008, 0.087, 0.091, 0.003, 0, 2.2, -13], [0.045, 0.083, 0.087, 0.002, 0, 2.2, -15], [0.076, 0.066, 0.07, 0.002, 0, 2.2, -16]]), "torso-clothing"));
  o.push(tag(L("jacket-body", "torso", "cotton-clothing", JACKET, [[-0.02, 0.181, 0.137, -0.004, 0, 2.5], [0.1, 0.18, 0.135, -0.007, 0, 2.5], [0.26, 0.187, 0.138, -0.008, 0, 2.5], [0.34, 0.193, 0.128, -0.004, 0, 2.5], [0.4, 0.201, 0.113, 0, 0, 2.4], [0.445, 0.199, 0.094, 0.005, 0, 2.3], [0.468, 0.165, 0.078, 0.008, 0, 2.2], [0.494, 0.076, 0.064, 0.009, 0, 2]]), "torso-clothing"));
  o.push(tag(L("jacket-collar", "torso", "cotton-clothing", 0.34, [[0.455, 0.08, 0.082, 0.008, 0, 2.2, 8], [0.52, 0.071, 0.077, 0.012, 0, 2.2, 12]]), "torso-clothing"));
  o.push(tag(L("jacket-skirt", "pelvis", "cotton-clothing", JACKET, [[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.04, 0.19, 0.146, 0.004, 0, 2.5], [0.02, 0.186, 0.14, 0.003, 0, 2.5], [0.045, 0.17, 0.12, 0, 0, 2.5], [0.125, 0.152, 0.1, -0.004, 0, 2.5]]), "torso-clothing"));
  o.push(tag(L("jeans-seat", "pelvis", "cotton-clothing", JEANS, [[-0.106, 0.06, 0.046, 0.008, 0, 2.2], [-0.07, 0.146, 0.09, 0.008, 0, 2.5], [-0.02, 0.168, 0.114, 0.008, 0, 2.6], [0.1, 0.15, 0.098, -0.002, 0, 2.6]]), "leg-clothing"));
  lr(o, L("sleeve-upper-l", "upper-arm-l", "cotton-clothing", JACKET, capped([[0, 0.064, 0.074, 0, 0, 2.2], [0.1, 0.065, 0.069, 0.001, 0.003, 2.2], [0.22, 0.058, 0.06, 0.002, 0, 2.2], [0.3082, 0.055, 0.057, 0, 0, 2.2]], 0.042, 0.02, 2)), "arm-clothing");
  lr(o, L("sleeve-lower-l", "forearm-l", "cotton-clothing", JACKET, capped([[0, 0.048, 0.05, 0, 0, 2.2], [0.07, 0.054, 0.054, 0.002, 0, 2.2], [0.18, 0.046, 0.044, 0, 0, 2.2], [0.24, 0.042, 0.039, 0, 0, 2.2], [0.276, 0.036, 0.032, 0, 0, 2.2]], 0.045, 0, 2)), "arm-clothing");
  lr(o, P("palm-l", "hand-l", "skin", SKIN, [[-0.006, 0, 0, 0.028, 0.019, 2.4], [0.04, 0.016, 0, 0.037, 0.016, 2.5], [0.096, 0.034, 0, 0.041, 0.013, 2.5]]), "hand-skin");
  lr(o, P("fingers-l", "hand-l", "skin", SKIN - 0.03, [[0.092, 0.036, 0, 0.04, 0.0095, 2.8], [0.1026, 0, 0, 0.04, 0.0095, 2.8], [0.0769, -0.026, 0, 0.04, 0.009, 2.8], [0.053, -0.011, 0, 0.038, 0.008, 2.8]]), "hand-skin");
  lr(o, T("thumb-l", "hand-l", "skin", SKIN - 0.03, [[0.012, 0.004, -0.03], [0.04, -0.016, -0.047], [0.078, -0.027, -0.049], [0.095, -0.022, -0.046]], [0.013, 0.012, 0.01, 0.008]), "hand-skin");
  lr(o, L("jeans-thigh-l", "thigh-l", "cotton-clothing", JEANS, capped([[0, 0.076, 0.088, -0.006, 0.002, 2.2], [0.1, 0.088, 0.098, -0.005, 0.004, 2.3], [0.25, 0.074, 0.08, 0.003, 0, 2.2], [0.38, 0.061, 0.067, 0.005, 0, 2.2], [0.4276, 0.059, 0.066, 0.005, 0, 2.2]], 0.045, 0.02, 2)), "leg-clothing");
  lr(o, L("jeans-shank-l", "shank-l", "cotton-clothing", JEANS, capped([[0, 0.052, 0.058, 0.004, 0, 2.2], [0.13, 0.061, 0.068, -0.006, 0, 2.2], [0.3, 0.057, 0.06, -0.005, 0, 2.1], [0.447, 0.057, 0.06, 0, 0, 2.1]], 0.05, 0, 2)), "leg-clothing");
  lr(o, L("shoe-l", "foot-l", "leather-shoe", SHOE, capped([[-0.075, 0.024, 0.034, -0.045, 0, 2.6], [-0.05, 0.036, 0.04, -0.039, 0, 2.8], [0, 0.04, 0.04, -0.0395, 0, 2.8], [0.045, 0.045, 0.038, -0.0415, 0, 2.8], [0.1, 0.051, 0.03, -0.0495, 0, 2.8], [0.1293, 0.052, 0.0265, -0.053, 0, 2.8]], 0, 0.012, 2)), "shoes");
  lr(o, L("shoe-toe-l", "toe-l", "leather-shoe", SHOE, [[-0.012, 0.052, 0.0265, 0.0265, 0, 2.8], [0.045, 0.047, 0.024, 0.024, 0, 2.7], [0.076, 0.03, 0.019, 0.019, 0, 2.4], [0.089, 0.012, 0.012, 0.014, 0, 2.2]]), "shoes");
  const c: Tagged[] = [];
  const cl = (s: Json, r: string): void => { const m = mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone); c.push(tag(shiftW(s, ZC), r)); c.push(tag(shiftW(m, ZC), r)); };
  c.push(tag(shiftW(P("child-trunk", "child", "snowsuit", 0.6, [[0.004, -0.058, 0, 0.065, 0.05, 2.3], [-0.006, 0, 0, 0.097, 0.077, 2.4], [-0.03, 0.15, 0, 0.096, 0.074, 2.4], [-0.042, 0.238, 0, 0.082, 0.058, 2.3], [-0.044, 0.262, 0, 0.05, 0.04, 2.2]]), ZC), "child-clothing"));
  cl(T("child-leg-l", "child", "snowsuit", 0.6, [[-0.02, -0.005, 0.05], [0.1, 0.005, 0.057], [0.172, 0.007, 0.06], [0.2, -0.014, 0.06], [0.215, -0.065, 0.06], [0.248, -0.122, 0.06]], [0.05, 0.051, 0.047, 0.044, 0.04, 0.035]), "child-clothing");
  cl(E("child-boot-l", "child", "snowsuit", 0.35, [0.283, -0.128, 0.06], [0.056, 0.03, 0.032], { rot: [0, 0, 0.1305, 0.9914] }), "child-clothing");
  cl(T("child-arm-l", "child", "snowsuit", 0.6, [[-0.038, 0.228, 0.094], [0.022, 0.138, 0.116], [0.122, 0.124, 0.088]], [0.035, 0.035, 0.027]), "child-clothing");
  cl(E("child-hand-l", "child", "skin", 0.88, [0.15, 0.122, 0.078], [0.024, 0.018, 0.02]), "child-skin");
  c.push(tag(shiftW(E("child-head", "child", "skin", 0.88, [-0.026, 0.335, 0], [0.079, 0.09, 0.064]), ZC), "child-skin"));
  c.push(tag(shiftW(E("child-hat", "child", "snowsuit", 0.3, [-0.037, 0.371, 0], [0.087, 0.072, 0.071]), ZC), "child-clothing"));
  c.push(tag(shiftW(E("child-pompom", "child", "snowsuit", 0.5, [-0.047, 0.452, 0], [0.024, 0.024, 0.024]), ZC), "child-clothing"));
  const s: Tagged[] = [];
  const side: V3[] = [[J[0], J[1], 0.245], [0.035, -0.049, 0.245], [0.008, -0.011, 0.238], [0, 0, 0.215]];
  pram1(s, T("push-bar", "stroller", "aluminium", ALU, [...side, ...side.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.0125), "stroller-frame");
  pram1(s, C("handle-grip", "stroller", "rubber", 0.25, [0, 0, -0.198], [0, 0, 0.198], 0.017), "stroller-frame");
  pramLr(s, R("joint-hub-l", "stroller", "rubber", DARK, [[-0.02, 0], [-0.02, 0.042], [0.02, 0.042], [0.02, 0]], [0, 0, 1], [J[0], J[1], 0.245]), "stroller-frame");
  pramLr(s, T("rear-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1], 0.245], [0.28, -0.8408, 0.245]], 0.0125), "stroller-frame");
  pram1(s, T("rear-axle", "stroller", "aluminium", ALU - 0.1, [[0.28, -0.8408, -0.262], [0.28, -0.8408, 0.262]], 0.011), "stroller-frame");
  pramLr(s, T("front-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1], 0.245], [0.8, -0.745, 0.215]], 0.0125), "stroller-frame");
  pram1(s, T("front-cross", "stroller", "aluminium", ALU - 0.1, [[0.8, -0.745, -0.215], [0.8, -0.745, 0.215]], 0.011), "stroller-frame");
  pramLr(s, C("caster-l", "stroller", "rubber", DARK, [0.826, -0.74, 0.215], [0.826, -0.83, 0.215], 0.019), "stroller-frame");
  pramLr(s, T("seat-rail-l", "stroller", "aluminium", ALU, [[0.146, -0.062, 0.185], [0.243, -0.53, 0.185], [0.455, -0.53, 0.185], [0.548, -0.694, 0.185]], 0.009), "stroller-frame");
  pram1(s, B("footrest", "stroller", "rubber", DARK, [0.566, -0.7, 0], [0.075, 0.016, 0.37]), "stroller-frame");
  const bb: V3[] = [[0.33, -0.416, 0.187], [0.44, -0.364, 0.18], [0.478, -0.36, 0.11]];
  pram1(s, T("bumper-bar", "stroller", "rubber", 0.25, [...bb, ...bb.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.011), "stroller-frame");
  pram1(s, X("seat-sling", "stroller", "stroller-textile", 0.55, "xy", strip([[0.15, -0.07], [0.246, -0.54], [0.455, -0.527], [0.54, -0.685]], 0.016), -0.178, 0.178), "stroller-fabric");
  pram1(s, X("basket", "stroller", "stroller-textile", FAB, "xy", strip([[0.315, -0.672], [0.315, -0.824], [0.735, -0.824], [0.735, -0.702]], 0.015), -0.194, 0.194), "stroller-fabric");
  pramLr(s, X("basket-side-l", "stroller", "stroller-textile", FAB, "xy", [[0.311, -0.83], [0.739, -0.83], [0.739, -0.7], [0.311, -0.67]], 0.18, 0.195), "stroller-fabric");
  const loop = [[-0.195, 0.03], [-0.195, 0.2], [-0.16, 0.268], [-0.07, 0.29], [0.07, 0.29], [0.16, 0.268], [0.195, 0.2], [0.195, 0.03], [0.18, 0.03], [0.18, 0.196], [0.15, 0.255], [0.07, 0.275], [-0.07, 0.275], [-0.15, 0.255], [-0.18, 0.196], [-0.18, 0.03], [-0.195, 0.03]];
  pram1(s, fan("canopy-rear", "stroller", HINGE, 65, loop), "stroller-fabric");
  pram1(s, fan("canopy-front", "stroller-hood", [0, 0], 25, loop), "stroller-fabric");
  for (const [n, z] of [["l", 0.277], ["r", -0.277]] as const) wheelDisc(s, "wheel-rear", `rear-${n}`, z, 0.1592, 0.022, 0.024, 0.028, 3);
  for (const [n, z] of [["l", 0.215], ["r", -0.215]] as const) wheelDisc(s, "wheel-front", `front-${n}`, z, 0.0796, 0.016, 0.016, 0.02, 3);
  return [...o, ...c, ...s];
}

function structureOf(tagged: Tagged[], adult: Json[], child: Json[], frame: string[], hood: string[], wheels: [string, string][], masses: [string, string, V3, number][]): Json[] {
  const ids = new Set(tagged.map((s) => s.id)), out = [...adult, ...child];
  const ref = (of: string, member: string, part = "member"): void => { if (!ids.has(of)) throw new Error(`no body solid ${of}`); out.push(st({ id: `m-${of}`, of }, member, part)); };
  frame.forEach((id) => ref(id, "stroller", id.startsWith("joint-hub") ? "joint" : "member"));
  hood.forEach((id) => ref(id, "stroller-hood"));
  wheels.forEach(([id, bone]) => ref(id, bone));
  masses.forEach(([id, bone, at, r]) => out.push(st(E(id, bone, bone === "child" ? "skin" : "aluminium", 0.5, at, [r, r, r]), bone, "mass")));
  return out;
}
/** 🦴️ A toddler rib hoop in the child frame (u ahead, v up): a bar of radius `bar` bent to an ellipse of half depth `d` and half width `w` about the reclined trunk axis, sloping down to the front. */
const childHoop = (id: string, u: number, v: number, d: number, w: number, bar: number): Json => ({ id, kind: "revolve", bone: "child", material: "skin", role: "body", tone: BONE, profile: Array.from({ length: 9 }, (_, k) => [r5(bar * Math.cos((k % 8) * (Math.PI / 4))), r5(d + bar * Math.sin((k % 8) * (Math.PI / 4)))]), axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: [u, v, 0], rotation: qaxis([0, 0, 1], 86).map(r5) } });
const childBones = (r: number, fine: boolean): Json[] => {
  const out: Json[] = [], one = (s: Json): void => { out.push(st(shiftW(s, ZC), "child", "member")); }, two = (s: Json): void => { const m = mirror(s, s.id.replace(/-l$/, "-r"), s.bone); one(s); one(m); };
  one(E("child-skull", "child", "skin", BONE, [-0.034, 0.35, 0], [0.07, 0.066, 0.056]));
  if (fine) one(T("child-mandible", "child", "skin", BONE, [[-0.012, 0.306, 0.036], [-0.008, 0.282, 0.032], [0.02, 0.272, 0.018], [0.03, 0.27, 0], [0.02, 0.272, -0.018], [-0.008, 0.282, -0.032], [-0.012, 0.306, -0.036]], r));
  one(P("child-spine", "child", "skin", BONE, [[-0.03, -0.012, 0, r * 1.2, r], [-0.058, 0.14, 0, r * 1.1, r], [-0.05, 0.285, 0, r, r], [-0.042, 0.312, 0, r, r]]));
  for (const [i, u, v, d, w] of (fine ? [[1, -0.02, 0.1, 0.047, 0.059], [2, -0.03, 0.18, 0.045, 0.056]] : [[1, -0.025, 0.14, 0.048, 0.06]]) as number[][]) one(childHoop(`child-rib-hoop-${i}`, u!, v!, d!, w!, r));
  one(T("child-pelvic-girdle", "child", "skin", BONE, [[0, 0, -0.05], [-0.022, 0.026, -0.046], [-0.04, 0.012, 0], [-0.022, 0.026, 0.046], [0, 0, 0.05]], r));
  two(T("child-leg-bones-l", "child", "skin", BONE, [[0, 0, 0.05], [0.172, 0.008, 0.06], [0.248, -0.122, 0.06]], r));
  two(T("child-arm-bones-l", "child", "skin", BONE, [[-0.038, 0.228, 0.094], [0.022, 0.138, 0.116], [0.122, 0.124, 0.088]], r));
  return out;
};

export function scale100(): { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[] } {
  const t = body100();
  const childFace: Json = { id: "th-child-face", kind: "loft", bone: "child", material: "skin", role: "body", tone: 0.88, sections: [[0.002, 0.326, 0.056, 0.064], [0.028, 0.324, 0.046, 0.052], [0.05, 0.322, 0.016, 0.018]].map(([u, v, rw, rv]) => ({ at: [u, v, ZC], rx: rw, ry: rv, exponent: 2.1, rotation: [0, 0.70711, 0, 0.70711] })), capStart: true, capEnd: true, region: "child-skin", sectionFaces: ["-u"] };
  const thermal = [...refs(t, /^(head|child-head)$/), faceMask("th-face", HEAD100, [-0.012, -0.04, -0.07, -0.088], "head-skin"), childFace, ...outlets(2)];
  const frame = ["push-bar", "joint-hub-l", "joint-hub-r", "rear-leg-l", "rear-leg-r", "rear-axle", "front-leg-l", "front-leg-r", "front-cross", "caster-l", "caster-r", "seat-rail-l", "seat-rail-r"];
  const wheels: [string, string][] = [["web-rear-l", "wheel-rear"], ["web-rear-r", "wheel-rear"], ["hub-rear-l", "wheel-rear"], ["hub-rear-r", "wheel-rear"], ["web-front-l", "wheel-front"], ["web-front-r", "wheel-front"]];
  const hoodRib = st(shiftW(T("hood-bow", "stroller-hood", "aluminium", DARK, [[0.02, 0.01, 0.19], [0.24, 0.112, 0.18], [0.262, 0.122, 0.1], [0.262, 0.122, -0.1], [0.24, 0.112, -0.18], [0.02, 0.01, -0.19]], 0.0076), ZC), "stroller-hood", "member");
  const structure = [...structureOf(t, adultSkeleton(100), childBones(0.0078, true), frame, [], wheels, [["mass-child", "child", [0.025, 0.11, ZC], 0.043], ["mass-stroller", "stroller", [0.45, -0.55, ZC], 0.02], ["mass-wheel-rear", "wheel-rear", [0, 0, ZC + 0.277], 0.02], ["mass-wheel-front", "wheel-front", [0, 0, ZC + 0.215], 0.012]]), hoodRib];
  return { body: t.map(bare), thermal, structure, decisions: DECISIONS["100"]! };
}

function body200(): Tagged[] {
  const o: Tagged[] = [];
  o.push(tag(L("head", "head", "skin", SKIN, [[-0.03, 0.06, 0.063, 0.002, 0, 2.1], [0.04, 0.056, 0.06, -0.004, 0, 2.1], [0.07, 0.058, 0.07, -0.03, 0, 2.1], [0.13, 0.071, 0.09, -0.004, 0, 2.2], [0.19, 0.077, 0.097, 0.004, 0, 2.3], [0.24, 0.062, 0.074, 0.009, 0, 2.2], [0.265, 0.02, 0.024, 0.01, 0, 2]]), "head-skin"));
  o.push(tag(L("hair-cap", "head", "hair", HAIR, [[0.118, 0.08, 0.032, 0.074, 0, 2.2], [0.17, 0.088, 0.052, 0.06, 0, 2.2], [0.2, 0.089, 0.09, 0.024, 0, 2.2], [0.24, 0.076, 0.09, 0.013, 0, 2.2], [0.277, 0.024, 0.03, 0.011, 0, 2]]), "hair"));
  o.push(tag(P("hair-back", "head", "hair", HAIR, [[0.16, 0.085, 0, 0.082, 0.034, 2.8], [0.04, 0.076, 0, 0.1, 0.034, 2.8], [-0.04, 0.084, 0, 0.088, 0.018, 2.6]]), "hair"));
  o.push(tag(L("scarf", "head", "cotton-clothing", 0.62, [[-0.02, 0.07, 0.074, 0.004, 0, 2.2, -12], [0.025, 0.087, 0.091, 0.003, 0, 2.2, -14], [0.074, 0.068, 0.072, 0.002, 0, 2.2, -16]]), "torso-clothing"));
  o.push(tag(L("jacket-body", "torso", "cotton-clothing", JACKET, [[-0.02, 0.182, 0.136, -0.004, 0, 2.5], [0.26, 0.187, 0.138, -0.008, 0, 2.5], [0.4, 0.201, 0.113, 0, 0, 2.4], [0.455, 0.192, 0.088, 0.006, 0, 2.3], [0.5, 0.075, 0.07, 0.01, 0, 2.1]]), "torso-clothing"));
  o.push(tag(L("pelvis", "pelvis", "cotton-clothing", JACKET, [[-0.075, 0.186, 0.142, 0.004, 0, 2.5], [-0.03, 0.19, 0.146, 0.004, 0, 2.5], [0.04, 0.17, 0.12, 0, 0, 2.5], [0.12, 0.152, 0.1, -0.004, 0, 2.5]]), "torso-clothing"));
  o.push(tag(L("jeans-seat", "pelvis", "cotton-clothing", JEANS, [[-0.105, 0.07, 0.05, 0.008, 0, 2.3], [-0.068, 0.146, 0.09, 0.008, 0, 2.5], [0.02, 0.164, 0.112, 0.005, 0, 2.5]]), "leg-clothing"));
  lr(o, L("sleeve-upper-l", "upper-arm-l", "cotton-clothing", JACKET, capped([[0, 0.064, 0.073, 0, 0, 2.2], [0.16, 0.061, 0.064, 0.002, 0.001, 2.2], [0.3082, 0.055, 0.057, 0, 0, 2.2]], 0.042, 0.02, 1)), "arm-clothing");
  lr(o, L("sleeve-lower-l", "forearm-l", "cotton-clothing", JACKET, capped([[0, 0.047, 0.049, 0, 0, 2.2], [0.12, 0.052, 0.051, 0.001, 0, 2.2], [0.275, 0.037, 0.033, 0, 0, 2.2]], 0.04, 0, 1)), "arm-clothing");
  lr(o, E("fist-l", "hand-l", "skin", SKIN, [0.07, 0.008, -0.004], [0.05, 0.036, 0.046]), "hand-skin");
  lr(o, L("jeans-thigh-l", "thigh-l", "cotton-clothing", JEANS, capped([[0, 0.076, 0.088, -0.006, 0.002, 2.2], [0.08, 0.087, 0.097, -0.005, 0.003, 2.3], [0.2, 0.08, 0.087, 0, 0.001, 2.2], [0.4276, 0.059, 0.066, 0.005, 0, 2.2]], 0.045, 0.02, 1)), "leg-clothing");
  lr(o, L("jeans-shank-l", "shank-l", "cotton-clothing", JEANS, capped([[0, 0.051, 0.057, 0.004, 0, 2.2], [0.14, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]], 0.045, 0, 1)), "leg-clothing");
  lr(o, L("shoe-l", "foot-l", "leather-shoe", SHOE, [[-0.076, 0.03, 0.036, -0.0435, 0, 2.8], [0, 0.04, 0.04, -0.0395, 0, 2.8], [0.07, 0.048, 0.036, -0.0435, 0, 2.8], [0.14, 0.052, 0.026, -0.0535, 0, 2.8]]), "shoes");
  lr(o, L("shoe-toe-l", "toe-l", "leather-shoe", SHOE, [[-0.015, 0.052, 0.026, 0.026, 0, 2.8], [0.05, 0.046, 0.023, 0.023, 0, 2.6], [0.089, 0.02, 0.015, 0.016, 0, 2.2]]), "shoes");
  const s: Tagged[] = [];
  pram1(s, P("child-trunk", "child", "snowsuit", 0.6, [[0.004, -0.058, 0, 0.07, 0.055, 2.3], [-0.01, 0.02, 0, 0.098, 0.078, 2.4], [-0.04, 0.23, 0, 0.086, 0.062, 2.3], [-0.044, 0.265, 0, 0.05, 0.04, 2.2]]), "child-clothing");
  pramLr(s, T("child-leg-l", "child", "snowsuit", 0.6, [[-0.02, -0.005, 0.052], [0.18, 0.008, 0.06], [0.25, -0.118, 0.06], [0.31, -0.12, 0.06]], [0.05, 0.047, 0.038, 0.03]), "child-clothing");
  pramLr(s, T("child-arm-l", "child", "snowsuit", 0.6, [[-0.038, 0.228, 0.094], [0.022, 0.138, 0.116], [0.14, 0.124, 0.084]], [0.035, 0.035, 0.026]), "child-clothing");
  pram1(s, E("child-head", "child", "skin", 0.88, [-0.026, 0.33, 0], [0.079, 0.09, 0.064]), "child-skin");
  pram1(s, E("child-hat", "child", "snowsuit", 0.3, [-0.038, 0.376, 0], [0.087, 0.078, 0.071]), "child-clothing");
  const r = 0.015, sd: V3[] = [[J[0], J[1], 0.245], [0.03, -0.042, 0.245], [0, 0, 0.225]];
  pram1(s, T("push-bar", "stroller", "aluminium", ALU, [...sd, ...sd.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], [r, r, 0.017, 0.017, r, r]), "stroller-frame");
  pramLr(s, T("rear-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1] + 0.03, 0.245], [0.28, -0.8408, 0.245]], r), "stroller-frame");
  pramLr(s, T("front-leg-l", "stroller", "aluminium", ALU, [[J[0] - 0.02, J[1] + 0.012, 0.245], [0.8, -0.745, 0.215], [0.826, -0.83, 0.215]], r), "stroller-frame");
  pramLr(s, R("joint-hub-l", "stroller", "rubber", DARK, [[-0.02, 0], [-0.02, 0.042], [0.02, 0.042], [0.02, 0]], [0, 0, 1], [J[0], J[1], 0.245]), "stroller-frame");
  pram1(s, T("rear-axle", "stroller", "aluminium", ALU - 0.1, [[0.28, -0.8408, -0.262], [0.28, -0.8408, 0.262]], r), "stroller-frame");
  pram1(s, X("seat-sling", "stroller", "stroller-textile", 0.55, "xy", strip([[0.146, -0.062], [0.243, -0.54], [0.455, -0.53], [0.548, -0.694]], 0.03), -0.19, 0.19), "stroller-fabric");
  pram1(s, B("basket", "stroller", "stroller-textile", FAB, [0.525, -0.755, 0], [0.43, 0.15, 0.39], 0.02), "stroller-fabric");
  const loop = [[-0.195, 0.03], [-0.195, 0.2], [-0.15, 0.275], [0, 0.29], [0.15, 0.275], [0.195, 0.2], [0.195, 0.03], [0.165, 0.03], [0.165, 0.19], [0.13, 0.245], [0, 0.26], [-0.13, 0.245], [-0.165, 0.19], [-0.165, 0.03], [-0.195, 0.03]];
  pram1(s, fan("canopy-rear", "stroller", HINGE, 65, loop), "stroller-fabric");
  pram1(s, fan("canopy-front", "stroller-hood", [0, 0], 25, loop), "stroller-fabric");
  for (const [n, z] of [["l", 0.277], ["r", -0.277]] as const) wheelDisc(s, "wheel-rear", `rear-${n}`, z, 0.1592, 0.022, 0, 0, 1);
  for (const [n, z] of [["l", 0.215], ["r", -0.215]] as const) wheelDisc(s, "wheel-front", `front-${n}`, z, 0.0796, 0.016, 0, 0, 1);
  return [...o, ...s];
}
/** 🩻️ The adult skeleton for a coarse scale: skull, one spine with its curves, rib cage as two hoops (1:200) or one keel (1:500), pelvic girdle, one bone per limb segment of radius `r`. */
function coarseSkeleton(r: number, hoops: boolean): Json[] {
  const out: Json[] = [];
  const one = (s: Json, member: string, part = "member"): void => { out.push(st(s, member, part)); };
  const two = (s: Json, member: string): void => { out.push(st(s, member, "member")); out.push(st(mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone.replace(/-l$/, "-r")), member.replace(/-l$/, "-r"), "member")); };
  one(E("skull", "head", "skin", BONE, [0.172, 0.004, 0], [0.09, 0.092, 0.071]), "head");
  one(T("cervical-spine", "head", "skin", BONE, [[-0.012, 0.032, 0], [0.09, 0.018, 0]], r), "head");
  one(T("spine", "torso", "skin", BONE, [[-0.012, 0.014, 0], [0.07, 0.006, 0], [0.28, 0.058, 0], [0.462, 0.032, 0]], r * 1.15), "torso");
  if (hoops) { for (const [i, sx, d, w] of [[1, 0.18, 0.09, 0.13], [2, 0.31, 0.092, 0.132]] as const) one({ id: `rib-hoop-${i}`, kind: "revolve", bone: "torso", material: "skin", role: "body", tone: BONE, profile: Array.from({ length: 9 }, (_, k) => [r5(r * Math.cos((k % 8) * (Math.PI / 4))), r5(d + r * Math.sin((k % 8) * (Math.PI / 4)))]), axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: [sx, 0, 0], rotation: qaxis([0, 0, 1], -14).map(r5) } }, "torso"); }
  if (!hoops) one({ id: "rib-hoop", kind: "revolve", bone: "torso", material: "skin", role: "body", tone: BONE, profile: Array.from({ length: 9 }, (_, k) => [r5(r * Math.cos((k % 8) * (Math.PI / 4))), r5(0.07 + r * Math.sin((k % 8) * (Math.PI / 4)))]), axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(0.108 / 0.07), transform: { translation: [0.25, 0.004, 0], rotation: qaxis([0, 0, 1], -14).map(r5) } }, "torso");
  one(T("shoulder-girdle", "torso", "skin", BONE, [[0.426, 0.004, -0.19], [0.402, -0.06, 0], [0.426, 0.004, 0.19]], r), "torso");
  one(T("pelvic-girdle", "pelvis", "skin", BONE, [[0, 0, -0.092], [0.07, 0.03, -0.11], [0.085, 0.05, 0], [0.07, 0.03, 0.11], [0, 0, 0.092]], r * 1.2), "pelvis");
  two(T("humerus-l", "upper-arm-l", "skin", BONE, [[0, 0, 0], [0.3082, 0, 0]], r), "upper-arm-l");
  two(T("forearm-bones-l", "forearm-l", "skin", BONE, [[0, 0, 0], [0.2685, 0, 0]], r), "forearm-l");
  two(T("hand-bones-l", "hand-l", "skin", BONE, [[0, 0, 0], [0.05, 0.018, 0], [0.092, 0.03, 0]], r), "hand-l");
  two(T("femur-l", "thigh-l", "skin", BONE, [[0, 0, 0], [0.05, 0, 0.042], [0.4276, 0, 0]], r * 1.1), "thigh-l");
  two(T("tibia-l", "shank-l", "skin", BONE, [[0, 0.004, 0], [0.4176, 0, 0]], r), "shank-l");
  two(T("foot-bones-l", "foot-l", "skin", BONE, [[-0.045, -0.04, 0], [0, -0.012, 0], [0.126, -0.048, 0]], r), "foot-l");
  return out;
}

export function scale200(): { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[] } {
  const t = body200();
  const sk = coarseSkeleton(0.015, true);
  sk.push(st(P("toe-bones-l", "toe-l", "skin", BONE, [[0, 0.03, 0, 0.034, 0.015], [0.066, 0.028, 0, 0.026, 0.015]]), "toe-l", "member"), st(P("toe-bones-r", "toe-r", "skin", BONE, [[0, 0.03, 0, 0.034, 0.015], [0.066, 0.028, 0, 0.026, 0.015]]), "toe-r", "member"));
  const hoodRib = st(shiftW(T("hood-bow", "stroller-hood", "aluminium", DARK, [[0.02, 0.01, 0.18], [0.25, 0.116, 0.16], [0.25, 0.116, -0.16], [0.02, 0.01, -0.18]], 0.015), ZC), "stroller-hood", "member");
  const structure = [...structureOf(t, sk, childBones(0.015, false), ["push-bar", "joint-hub-l", "joint-hub-r", "rear-leg-l", "rear-leg-r", "rear-axle", "front-leg-l", "front-leg-r"], [], [], [["mass-child", "child", [0.025, 0.11, ZC], 0.043], ["mass-stroller", "stroller", [0.45, -0.55, ZC], 0.02]]), hoodRib,
    st(shiftW(R("wheel-web-rear", "wheel-rear", "rubber", 0.6, [[-0.015, 0], [-0.015, 0.13], [0.015, 0.13], [0.015, 0]], [0, 0, 1], [0, 0, 0.277]), ZC), "wheel-rear", "member"), st(shiftW(R("wheel-web-rear-r", "wheel-rear", "rubber", 0.6, [[-0.015, 0], [-0.015, 0.13], [0.015, 0.13], [0.015, 0]], [0, 0, 1], [0, 0, -0.277]), ZC), "wheel-rear", "member"),
    ...[0.215, -0.215].map((z, i) => st(shiftW(R(`wheel-web-front${i ? "-r" : ""}`, "wheel-front", "rubber", 0.6, [[-0.015, 0], [-0.015, 0.066], [0.015, 0.066], [0.015, 0]], [0, 0, 1], [0, 0, z]), ZC), "wheel-front", "member")),
    ...[0.215, -0.215].map((z, i) => st(shiftW(T(`caster-stem${i ? "-r" : ""}`, "stroller", "aluminium", DARK, [[0.826, -0.74, z], [0.826, -0.84, z]], 0.015), ZC), "stroller", "member"))];
  return { body: t.map(bare), thermal: [...refs(t, /^$/), ...outlets(4)], structure, decisions: DECISIONS["200"]! };
}

function body500(): Tagged[] {
  const o: Tagged[] = [];
  o.push(tag(L("head", "head", "skin", SKIN, [[-0.03, 0.058, 0.062, 0, 0, 2.1], [0.06, 0.06, 0.07, -0.02, 0, 2.1], [0.17, 0.076, 0.096, 0.002, 0, 2.2], [0.262, 0.03, 0.036, 0.01, 0, 2]]), "head-skin"));
  o.push(tag(L("hair", "head", "hair", HAIR, [[0.1, 0.086, 0.04, 0.072, 0, 2.2], [0.165, 0.091, 0.058, 0.056, 0, 2.2], [0.2, 0.091, 0.096, 0.02, 0, 2.2], [0.245, 0.078, 0.092, 0.013, 0, 2.2], [0.282, 0.032, 0.0375, 0.011, 0, 2]]), "hair"));
  o.push(tag(L("jacket-body", "torso", "cotton-clothing", JACKET, [[-0.02, 0.184, 0.138, -0.004, 0, 2.5], [0.3, 0.19, 0.134, -0.006, 0, 2.5], [0.445, 0.2, 0.096, 0.004, 0, 2.3], [0.495, 0.08, 0.075, 0.009, 0, 2.1]]), "torso-clothing"));
  o.push(tag(L("pelvis", "pelvis", "cotton-clothing", JEANS, [[-0.1, 0.085, 0.064, 0.008, 0, 2.3], [-0.05, 0.16, 0.112, 0.008, 0, 2.5], [0.12, 0.158, 0.11, -0.004, 0, 2.5]]), "leg-clothing"));
  lr(o, L("sleeve-upper-l", "upper-arm-l", "cotton-clothing", JACKET, [[-0.052, 0.024, 0.028, 0, 0, 2.1], [-0.03, 0.052, 0.058, 0, 0, 2.1], [0.02, 0.066, 0.073, 0, 0, 2.2], [0.3, 0.056, 0.058, 0, 0, 2.2], [0.34, 0.052, 0.054, 0, 0, 2.1], [0.362, 0.03, 0.032, 0, 0, 2]]), "arm-clothing");
  lr(o, L("sleeve-lower-l", "forearm-l", "cotton-clothing", JACKET, [[-0.03, 0.045, 0.047, 0, 0, 2.2], [0.05, 0.05, 0.05, 0, 0, 2.2], [0.275, 0.04, 0.038, 0, 0, 2.2]]), "arm-clothing");
  lr(o, E("fist-l", "hand-l", "skin", SKIN, [0.068, 0.008, -0.004], [0.052, 0.04, 0.046]), "hand-skin");
  lr(o, L("jeans-thigh-l", "thigh-l", "cotton-clothing", JEANS, [[-0.04, 0.07, 0.08, -0.006, 0.004, 2.2], [0.04, 0.088, 0.1, -0.006, 0.004, 2.3], [0.46, 0.058, 0.064, 0.005, 0, 2.2]]), "leg-clothing");
  lr(o, L("jeans-shank-l", "shank-l", "cotton-clothing", JEANS, [[-0.035, 0.049, 0.055, 0.004, 0, 2.2], [0.13, 0.061, 0.067, -0.006, 0, 2.2], [0.447, 0.057, 0.06, 0, 0, 2.1]]), "leg-clothing");
  lr(o, L("shoe-l", "foot-l", "leather-shoe", SHOE, [[-0.08, 0.03, 0.038, -0.0415, 0, 2.3], [-0.05, 0.038, 0.042, -0.0375, 0, 2.4], [0.02, 0.042, 0.046, -0.0335, 0, 2.4], [0.138, 0.046, 0.0375, -0.042, 0, 2.4]]), "shoes");
  lr(o, L("shoe-toe-l", "toe-l", "leather-shoe", SHOE, [[-0.03, 0.04, 0.033, 0.033, 0, 2.4], [0.004, 0.04, 0.033, 0.033, 0, 2.4], [0.012, 0.046, 0.0375, 0.0375, 0, 2.4], [0.055, 0.044, 0.0375, 0.0375, 0, 2.4], [0.082, 0.038, 0.034, 0.034, 0, 2.2], [0.092, 0.024, 0.022, 0.022, 0, 2.1]]), "shoes");
  const s: Tagged[] = [];
  pram1(s, P("child-body", "child", "snowsuit", 0.6, [[0.25, -0.1, 0, 0.09, 0.04, 2.4], [0.16, -0.01, 0, 0.1, 0.05, 2.4], [0.0, -0.01, 0, 0.1, 0.07, 2.4], [-0.03, 0.12, 0, 0.098, 0.078, 2.4], [-0.044, 0.262, 0, 0.07, 0.05, 2.3]]), "child-clothing");
  pram1(s, E("child-head", "child", "skin", 0.88, [-0.03, 0.345, 0], [0.084, 0.095, 0.07]), "child-skin");
  const run: V3[] = [[0.28, -0.8408, 0.245], [J[0], J[1], 0.245], [0.03, -0.042, 0.245], [0, 0, 0.225]];
  pram1(s, T("push-frame", "stroller", "aluminium", ALU, [...run, ...run.map((p) => [p[0], p[1], -p[2]] as V3).reverse()], 0.0125, { extra: { slender: true } }), "stroller-frame");
  pramLr(s, T("front-leg-l", "stroller", "aluminium", ALU, [[J[0], J[1], 0.245], [0.8, -0.745, 0.215], [0.82, -0.84, 0.215]], 0.0125, { extra: { slender: true } }), "stroller-frame");
  pram1(s, X("seat", "stroller", "stroller-textile", 0.55, "xy", strip([[0.146, -0.062], [0.25, -0.56], [0.47, -0.55], [0.548, -0.694]], 0.075), -0.19, 0.19), "stroller-fabric");
  pram1(s, B("basket", "stroller", "stroller-textile", FAB, [0.525, -0.755, 0], [0.43, 0.15, 0.39], 0.03), "stroller-fabric");
  const wedge = [[-0.195, 0.03], [-0.195, 0.2], [-0.12, 0.285], [0.12, 0.285], [0.195, 0.2], [0.195, 0.03]];
  pram1(s, fan("canopy-rear", "stroller", HINGE, 65, wedge), "stroller-fabric");
  pram1(s, fan("canopy-front", "stroller-hood", [0, 0], 25, wedge), "stroller-fabric");
  pram1(s, R("wheels-rear", "wheel-rear", "rubber", 0.14, [[-0.0375, 0], [-0.0375, 0.15], [-0.02, 0.1592], [0.02, 0.1592], [0.0375, 0.15], [0.0375, 0]], [0, 0, 1], [0, 0, 0.2625]), "tyres");
  pram1(s, R("wheels-rear-r", "wheel-rear", "rubber", 0.14, [[-0.0375, 0], [-0.0375, 0.15], [-0.02, 0.1592], [0.02, 0.1592], [0.0375, 0.15], [0.0375, 0]], [0, 0, 1], [0, 0, -0.2625]), "tyres");
  pram1(s, R("wheels-front", "wheel-front", "rubber", 0.14, [[-0.0375, 0], [-0.0375, 0.0796], [0.0375, 0.0796], [0.0375, 0]], [0, 0, 1], [0, 0, 0.215]), "tyres");
  pram1(s, R("wheels-front-r", "wheel-front", "rubber", 0.14, [[-0.0375, 0], [-0.0375, 0.0796], [0.0375, 0.0796], [0.0375, 0]], [0, 0, 1], [0, 0, -0.215]), "tyres");
  return [...o, ...s];
}

export function scale500(): { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[] } {
  const t = body500();
  const sk = coarseSkeleton(0.0375, false).filter((x) => !/^(hand-bones|foot-bones)/.test(x.id));
  const two = (s: Json, member: string): void => { sk.push(st(s, member, "member")); sk.push(st(mirror(s, s.id.replace(/-l$/, "-r"), s.bone.replace(/-l$/, "-r")), member.replace(/-l$/, "-r"), "member")); };
  two(E("hand-bones-l", "hand-l", "skin", BONE, [0.06, 0.012, 0], [0.04, 0.0375, 0.04]), "hand-l");
  two(E("foot-bones-l", "foot-l", "skin", BONE, [0.05, -0.036, 0], [0.11, 0.0375, 0.0375]), "foot-l");
  const structure = [...structureOf(t, sk, [st(shiftW(T("child-spine", "child", "skin", BONE, [[-0.02, -0.01, 0], [-0.05, 0.14, 0], [-0.045, 0.29, 0]], 0.0375), ZC), "child", "member"), st(shiftW(E("child-skull", "child", "skin", BONE, [-0.034, 0.35, 0], [0.07, 0.066, 0.056]), ZC), "child", "member"), ...[0.056, -0.056].map((z, i) => st(shiftW(T(`child-leg-bones${i ? "-r" : "-l"}`, "child", "skin", BONE, [[0, 0, z], [0.172, 0.008, z], [0.248, -0.122, z]], 0.0375), ZC), "child", "member"))], ["push-frame", "front-leg-l", "front-leg-r"], [], [], [["mass-child", "child", [0.025, 0.11, ZC], 0.043]]),
    st(shiftW(T("hood-bow", "stroller-hood", "aluminium", DARK, [[0.03, 0.014, 0.15], [0.23, 0.107, 0.14], [0.23, 0.107, -0.14], [0.03, 0.014, -0.15]], 0.0375), ZC), "stroller-hood", "member"),
    st(shiftW(R("axle-rear", "wheel-rear", "rubber", 0.6, [[-0.3, 0], [-0.3, 0.0375], [0.3, 0.0375], [0.3, 0]], [0, 0, 1], [0, 0, 0]), ZC), "wheel-rear", "member"),
    ...[0.2625, -0.2625].map((z, i) => st(shiftW(R(`wheel-disc-rear${i ? "-r" : ""}`, "wheel-rear", "rubber", 0.6, [[-0.0375, 0], [-0.0375, 0.125], [0.0375, 0.125], [0.0375, 0]], [0, 0, 1], [0, 0, z]), ZC), "wheel-rear", "member")),
    ...[0.215, -0.215].map((z, i) => st(shiftW(R(`wheel-disc-front${i ? "-r" : ""}`, "wheel-front", "rubber", 0.6, [[-0.0375, 0], [-0.0375, 0.062], [0.0375, 0.062], [0.0375, 0]], [0, 0, 1], [0, 0, z]), ZC), "wheel-front", "member"))];
  return { body: t.map(bare), thermal: [...refs(t, /^$/), ...outlets(10)], structure, decisions: DECISIONS["500"]! };
}
