/** 🦴️ Structure list at 1:50: the load path. Adult: skull with jaw, cervical and thoracolumbar spine with lordosis and kyphosis, six rib hoops with sternum, clavicles and scapulae, iliac wings with sacrum and pubic bar, humerus, radius and ulna, carpus with metacarpals and the finger rays around the bar, femur with neck and condyles, tibia and fibula, patella, calcaneus, metatarsus and phalanges; joints at the rig joints, masses at the segment centres of mass. Toddler: simplified skeleton. Stroller: its own tubes, hubs, forks, bows, rims, spokes and wheel hubs.
 * @see ./r3-stroller-s50-figure.ts */
import { C, E, B, P, T, mirror, shiftW, st, qaxis, r5, type Json, type V3 } from "./r3-stroller-kit.ts";

const BONE = 0.93, ZC = 0.1925;
const polar = (c: [number, number], rho: number, deg: number, w: number): V3 => [c[0] + rho * Math.cos((deg * Math.PI) / 180), c[1] + rho * Math.sin((deg * Math.PI) / 180), w];
/** 🦴️ A rib hoop: a round bar bent to an ellipse about the bone axis at `s` (half depth `d`, half width `w`), tilted about the lateral axis. */
const ring = (id: string, bone: string, s: number, cv: number, d: number, w: number, bar: number, tilt: number): Json => ({ id, kind: "revolve", bone, material: "skin", role: "body", tone: BONE, profile: Array.from({ length: 9 }, (_, i) => [r5(bar * Math.cos((i % 8) * (Math.PI / 4))), r5(d + bar * Math.sin((i % 8) * (Math.PI / 4)))]), axis: [1, 0, 0], origin: [0, 0, 0], lateralScale: r5(w / d), transform: { translation: [s, cv, 0], rotation: qaxis([0, 0, 1], tilt).map(r5) } });

export function adultSkeleton(detail: 50 | 100): Json[] {
  const out: Json[] = [], fine = detail === 50;
  const one = (s: Json, member: string, part = "member"): void => { out.push(st(s, member, part)); };
  const both = (s: Json, member: string, part = "member"): void => { out.push(st(s, member, part)); out.push(st(mirror(s, s.id.replace(/-l(-|$)/, "-r$1"), s.bone.replace(/-l$/, "-r")), member.replace(/-l$/, "-r"), part)); };
  const b = (id: string, bone: string, p0: V3, p1: V3, r0: number, r1 = r0, ls?: number): Json => C(id, bone, "skin", BONE, p0, p1, r0, r1, ls ? { extra: { lateralScale: ls } } : {});
  const e = (id: string, bone: string, at: V3, radii: V3, tone = BONE): Json => E(id, bone, "skin", tone, at, radii);
  one(e("cranium", "head", [0.19, 0.008, 0], [0.072, 0.088, 0.069]), "head");
  one(e("facial-bones", "head", [0.128, -0.05, 0], [0.036, 0.03, 0.05]), "head");
  one(T("mandible", "head", "skin", BONE, [[0.128, -0.004, 0.052], [0.072, -0.006, 0.047], [0.056, -0.05, 0.032], [0.05, -0.078, 0], [0.056, -0.05, -0.032], [0.072, -0.006, -0.047], [0.128, -0.004, -0.052]], fine ? 0.007 : 0.009), "head");
  if (fine) both(b("zygomatic-arch-l", "head", [0.136, -0.058, 0.056], [0.134, 0.0, 0.066], 0.0045), "head");
  one(P("cervical-spine", "head", "skin", BONE, [[-0.012, 0.034, 0, 0.015, 0.013], [0.03, 0.026, 0, 0.014, 0.012], [0.07, 0.02, 0, 0.013, 0.012], [0.106, 0.018, 0, 0.013, 0.012]]), "head");
  one(e("joint-neck", "head", [0, 0.03, 0], [0.016, 0.016, 0.016], 0.6), "head", "joint");
  one(P("spine", "torso", "skin", BONE, [[-0.012, 0.014, 0, 0.022, 0.018], [0.03, 0.006, 0, 0.022, 0.018], [0.08, 0.006, 0, 0.021, 0.017], [0.15, 0.028, 0, 0.019, 0.016], [0.22, 0.052, 0, 0.018, 0.015], [0.29, 0.06, 0, 0.017, 0.014], [0.36, 0.052, 0, 0.016, 0.013], [0.42, 0.04, 0, 0.015, 0.013], [0.462, 0.032, 0, 0.015, 0.013]]), "torso");
  one(e("joint-lumbar", "torso", [0, 0.012, 0], [0.02, 0.02, 0.02], 0.6), "torso", "joint");
  const ribs: number[][] = fine ? [[0.14, 0.085, 0.125], [0.185, 0.095, 0.135], [0.23, 0.1, 0.14], [0.275, 0.1, 0.14], [0.32, 0.092, 0.132], [0.362, 0.078, 0.115]] : [[0.16, 0.09, 0.13], [0.24, 0.1, 0.14], [0.32, 0.09, 0.13]];
  ribs.forEach(([s, d, w], i) => one(ring(`rib-${i + 1}`, "torso", s!, -0.002 + i * 0.002, d!, w!, fine ? 0.006 : 0.009, -14), "torso"));
  one(B("sternum", "torso", "skin", BONE, [0.31, -0.094, 0], [0.15, fine ? 0.012 : 0.016, 0.036], 0.004), "torso");
  both(b("clavicle-l", "torso", [0.4, -0.078, 0.022], [0.426, 0.004, 0.186], fine ? 0.007 : 0.009), "torso");
  both(e("scapula-l", "torso", [0.34, 0.078, 0.1], [0.075, 0.009, 0.055]), "torso");
  one(e("mass-torso", "torso", [0.2088, 0, 0], [0.056, 0.056, 0.056], 0.5), "torso", "mass");
  both(E("ilium-l", "pelvis", "skin", BONE, [0.06, 0.012, 0.108], [0.072, 0.062, 0.022], { rot: qaxis([1, 0, 0], 20) }), "pelvis");
  one(e("sacrum", "pelvis", [0.05, 0.05, 0], [0.058, 0.02, 0.048]), "pelvis");
  one(b("pubis", "pelvis", [-0.03, -0.062, -0.055], [-0.03, -0.062, 0.055], 0.012), "pelvis");
  both(b("ischium-l", "pelvis", [0.0, 0.0, 0.092], [-0.05, 0.03, 0.06], 0.014), "pelvis");
  one(e("mass-pelvis", "pelvis", [0.0497, 0, 0], [0.039, 0.039, 0.039], 0.5), "pelvis", "mass");
  both(b("humerus-l", "upper-arm-l", [0, 0, 0], [0.3082, 0, 0], 0.012, 0.011), "upper-arm-l");
  both(e("humerus-head-l", "upper-arm-l", [0.004, 0, -0.006], [0.024, 0.024, 0.024]), "upper-arm-l");
  both(e("joint-shoulder-l", "upper-arm-l", [0, 0, 0], [0.016, 0.03, 0.016], 0.6), "upper-arm-l", "joint");
  both(b("radius-l", "forearm-l", [0, 0.012, 0], [0.2685, 0.006, -0.012], fine ? 0.007 : 0.0078), "forearm-l");
  both(b("ulna-l", "forearm-l", [-0.012, -0.012, 0], [0.2685, -0.004, 0.014], 0.008, 0.006), "forearm-l");
  both(e("joint-elbow-l", "forearm-l", [0, 0, 0], [0.02, 0.02, 0.026], 0.6), "forearm-l", "joint");
  both(e("carpus-l", "hand-l", [0.012, 0.004, 0], [0.016, 0.01, 0.026]), "hand-l");
  both(e("joint-wrist-l", "hand-l", [0, 0, 0], [0.012, 0.012, 0.02], 0.6), "hand-l", "joint");
  const bar: [number, number] = [0.0746, 0];
  if (fine) {
    [-0.03, -0.01, 0.01, 0.029].forEach((w, i) => { both(b(`metacarpal-l-${i + 1}`, "hand-l", [0.022, 0.008, w * 0.6], polar(bar, 0.04, 64, w), 0.0045), "hand-l"); both(T(`phalanges-l-${i + 1}`, "hand-l", "skin", BONE, [64, 35, 0, -40, -85, -125].map((d, j) => polar(bar, [0.04, 0.033, 0.028, 0.027, 0.026, 0.026][j]!, d, w)), 0.004), "hand-l"); });
    both(T("thumb-ray-l", "hand-l", "skin", BONE, [[0.012, 0.004, -0.03], [0.034, -0.012, -0.046], [0.058, -0.023, -0.05], [0.078, -0.027, -0.049], [0.092, -0.023, -0.047]], 0.0045), "hand-l");
  } else both(P("hand-rays-l", "hand-l", "skin", BONE, [[0.02, 0.006, 0, 0.03, 0.008], [0.055, 0.022, 0, 0.036, 0.008], [0.092, 0.034, 0, 0.038, 0.008], [0.106, 0.016, 0, 0.036, 0.008], [0.1, -0.012, 0, 0.034, 0.008]]), "hand-l");
  both(T("femur-l", "thigh-l", "skin", BONE, [[0, 0, 0], [0.022, 0, 0.034], [0.06, 0, 0.046], [0.2, 0, 0.026], [0.4276, 0, 0]], [0.02, 0.014, 0.017, 0.014, 0.017]), "thigh-l");
  both(e("femur-condyles-l", "thigh-l", [0.42, 0, 0], [0.026, 0.032, 0.042]), "thigh-l");
  both(e("joint-hip-l", "thigh-l", [0, 0, 0], [0.027, 0.027, 0.027], 0.6), "thigh-l", "joint");
  both(e("mass-thigh-l", "thigh-l", [0.1751, 0, 0], [0.042, 0.042, 0.042], 0.5), "thigh-l", "mass");
  both(b("tibia-l", "shank-l", [0.012, 0.008, -0.004], [0.4176, 0, 0], 0.018, 0.012), "shank-l");
  both(b("fibula-l", "shank-l", [0.035, -0.012, 0.036], [0.42, -0.008, 0.03], fine ? 0.006 : 0.0078, fine ? 0.0055 : 0.0078), "shank-l");
  both(e("patella-l", "shank-l", [-0.006, 0.046, 0], [0.022, 0.01, 0.02]), "shank-l");
  both(e("joint-knee-l", "shank-l", [0, 0, 0], [0.018, 0.022, 0.034], 0.6), "shank-l", "joint");
  both(e("calcaneus-l", "foot-l", [-0.03, -0.03, 0], [0.04, 0.022, 0.022]), "foot-l");
  both(P("metatarsus-l", "foot-l", "skin", BONE, [[0.005, -0.014, 0, 0.026, 0.014], [0.07, -0.03, 0, 0.032, 0.011], [0.126, -0.044, 0, 0.038, 0.008]]), "foot-l");
  both(e("joint-ankle-l", "foot-l", [0, 0, 0], [0.018, 0.018, 0.026], 0.6), "foot-l", "joint");
  both(P("phalanges-foot-l", "toe-l", "skin", BONE, [[0, 0.034, 0, 0.036, fine ? 0.007 : 0.008], [0.035, 0.032, 0, 0.033, fine ? 0.006 : 0.008], [0.066, 0.03, 0, 0.024, fine ? 0.005 : 0.008]]), "toe-l");
  both(e("joint-toe-l", "toe-l", [0, 0.034, 0], [0.01, 0.01, 0.034], 0.6), "toe-l", "joint");
  return out;
}

export function childSkeleton(): Json[] {
  const out: Json[] = [];
  const one = (s: Json, part = "member"): void => { out.push(st(shiftW(s, ZC), "child", part)); };
  const both = (s: Json): void => { const m = mirror(s, s.id.replace(/-l$/, "-r"), s.bone); one(s); one(m); };
  const b = (id: string, p0: V3, p1: V3, r: number): Json => C(id, "child", "skin", BONE, p0, p1, r);
  one(E("child-skull", "child", "skin", BONE, [-0.034, 0.352, 0], [0.07, 0.066, 0.056]));
  one(E("child-facial-bones", "child", "skin", BONE, [0.008, 0.31, 0], [0.026, 0.024, 0.036]));
  one(T("child-mandible", "child", "skin", BONE, [[-0.012, 0.306, 0.036], [-0.008, 0.282, 0.032], [0.02, 0.272, 0.018], [0.03, 0.27, 0], [0.02, 0.272, -0.018], [-0.008, 0.282, -0.032], [-0.012, 0.306, -0.036]], 0.005));
  one(P("child-spine", "child", "skin", BONE, [[-0.036, -0.012, 0, 0.011, 0.009], [-0.05, 0.06, 0, 0.01, 0.009], [-0.062, 0.15, 0, 0.01, 0.008], [-0.064, 0.235, 0, 0.009, 0.008], [-0.05, 0.285, 0, 0.009, 0.008], [-0.042, 0.312, 0, 0.009, 0.008]]));
  ([[1, -0.016, 0.095, 0.046, 0.058], [2, -0.021, 0.13, 0.05, 0.062], [3, -0.027, 0.165, 0.048, 0.06], [4, -0.033, 0.2, 0.04, 0.05]] as number[][]).forEach(([i, u, v, d, w]) => one({ ...ring(`child-rib-${i}`, "child", u!, v!, d!, w!, 0.004, 86) }));
  one(b("child-sternum", [0.03, 0.105, 0], [0.012, 0.2, 0], 0.005));
  both(b("child-clavicle-l", [-0.006, 0.222, 0.008], [-0.036, 0.228, 0.088], 0.004));
  both(E("child-ilium-l", "child", "skin", BONE, [-0.02, 0.016, 0.044], [0.024, 0.03, 0.009]));
  one(E("child-sacrum", "child", "skin", BONE, [-0.04, 0.0, 0], [0.01, 0.026, 0.022]));
  one(b("child-pubis", [0.014, -0.018, -0.028], [0.014, -0.018, 0.028], 0.005));
  both(b("child-foot-bones-l", [0.248, -0.122, 0.06], [0.31, -0.134, 0.06], 0.006));
  both(b("child-hand-bones-l", [0.122, 0.124, 0.088], [0.16, 0.12, 0.076], 0.006));
  both(b("child-femur-l", [0, 0, 0.05], [0.172, 0.008, 0.06], 0.008));
  both(b("child-tibia-l", [0.172, 0.008, 0.06], [0.248, -0.122, 0.06], 0.007));
  both(b("child-humerus-l", [-0.038, 0.228, 0.094], [0.022, 0.138, 0.116], 0.006));
  both(b("child-forearm-l", [0.022, 0.138, 0.116], [0.122, 0.124, 0.088], 0.0055));
  one(E("mass-child", "child", "skin", 0.5, [0.025, 0.11, 0], [0.043, 0.043, 0.043]), "mass");
  return out;
}

export function skeleton50(tagged: Json[]): Json[] {
  const out = [...adultSkeleton(50), ...childSkeleton()];
  const ids = new Set(tagged.map((s) => s.id));
  const ref = (of: string, member: string, part = "member"): void => { if (!ids.has(of)) throw new Error(`no body solid ${of}`); out.push(st({ id: `m-${of}`, of }, member, part)); };
  for (const id of ["push-bar", "rear-axle", "front-cross", "seat-top-bow", "canopy-bow-rear", "canopy-bow-mid"]) ref(id, "stroller");
  for (const id of ["handle-sleeve", "rear-leg", "front-leg", "swivel", "fork-crown", "fork-prong-a", "fork-prong-b", "seat-rail", "seat-mount"]) for (const s of ["l", "r"]) ref(`${id}-${s}`, "stroller");
  for (const s of ["l", "r"]) ref(`joint-hub-${s}`, "stroller", "joint");
  ref("canopy-bow-front", "stroller-hood");
  for (const s of ["l", "r"]) { ref(`canopy-hinge-${s}`, "stroller-hood", "joint"); for (const p of ["rim", "spokes", "hub"]) { ref(`${p}-rear-${s}`, "wheel-rear"); ref(`${p}-front-${s}`, "wheel-front"); } }
  const mass = (id: string, bone: string, at: V3, r: number): void => { out.push(st(E(id, bone, "aluminium", 0.5, at, [r, r, r]), bone, "mass")); };
  mass("mass-stroller", "stroller", [0.45, -0.55, ZC], 0.02);
  mass("mass-hood", "stroller-hood", [0.085, 0.18, ZC], 0.012);
  for (const [n, k] of [["l", 1], ["r", -1]] as const) { mass(`mass-wheel-rear-${n}`, "wheel-rear", [0, 0, ZC + k * 0.277], 0.02); mass(`mass-wheel-front-${n}`, "wheel-front", [0, 0, ZC + k * 0.215], 0.012); }
  return out;
}
