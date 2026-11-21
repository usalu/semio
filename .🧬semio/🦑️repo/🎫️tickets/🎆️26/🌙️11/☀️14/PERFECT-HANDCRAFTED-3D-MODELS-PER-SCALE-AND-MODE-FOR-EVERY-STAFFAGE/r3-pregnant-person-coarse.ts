/** 🤰 The 1:100, 1:200 and 1:500 models of `humans/pregnant-person`, each stated anew for its smallest feature (15, 30, 75 mm): fewer and generalised sections, merged masses (hem and collar into the sweater, cuffs into the sleeves, sole, heel and shaft into one boot, fingers into a mitten, the face details into the head), the same woman with the same abdomen, clothes and proportions. Thermal: one surface solid per region (layers merged, B2) and the three outlets at the scale's size floor; structure: a merged skeleton with the fetus (1:100), a skeleton of bars and hoops with the fetal head and body (1:200) and a stick skeleton with the fetal unit as one mass (1:500).
 * @see ./r3-pregnant-person-s50.ts
 * @see ./r3-humans-pregnant-person.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, aboutU, aboutW, capsule, compose, dense, dome, ellipsoid, loft, mirrored, pa, sa, thOf, toRight, tube, type St, type V3 } from "./r3-pregnant-person-kit.ts";
import { C, G, MAT, TONE, skirtSt, torsoSt } from "./r3-pregnant-person-s50.ts";
import { bellyRow, pelvisRow, type Cross, type Slice } from "./r3-pregnant-person-forms.ts";
import { RIG } from "./r3-pregnant-person-data.ts";

type Spec = {
  scale: 100 | 200 | 500;
  skirt: Slice[];
  trunk: Slice[];
  belly: Cross[];
  seat: Slice[];
  thigh: St[];
  shank: St[];
  sleeve: St[];
  fore: St[];
  boot: [number, number, number, number, number][];
  toe: [number, number, number, number, number][];
  head: [number, number, number, number, number][];
  hair: [number, number, number, number, number][];
  neck: St[];
};

const S100: Spec = {
  scale: 100,
  skirt: [[0.83, -0.172, 0.102, 0.197, 2.4], [0.9, -0.163, 0.115, 0.197, 2.4]],
  trunk: [[0.99, -0.12, 0.126, 0.187, 2.4], [1.1, -0.117, 0.13, 0.183, 2.4], [1.2, -0.133, 0.147, 0.187, 2.3], [1.25, -0.14, 0.157, 0.193, 2.3], [1.32, -0.139, 0.126, 0.204, 2.5], [1.39, -0.121, 0.078, 0.198, 2.7], [1.428, -0.098, 0.044, 0.135, 2.4], [1.455, -0.084, 0.024, 0.075, 2.1]],
  belly: [[0.12, 0.866, 1.207, 0.178, 2.25], [0.17, 0.882, 1.185, 0.16, 2.1], [0.215, 0.909, 1.135, 0.126, 2.0], [0.245, 0.942, 1.09, 0.083, 2.0], [0.262, 0.99, 1.042, 0.03, 2.0]],
  seat: [[0.785, -0.1, 0.06, 0.15, 2.2], [0.84, -0.155, 0.085, 0.182, 2.4], [0.9, -0.148, 0.102, 0.181, 2.4], [0.98, -0.106, 0.104, 0.158, 2.4]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.008, 0.088, 0.095, 2.2), sa("thigh-l", 0.1, 0.004, -0.004, 0.085, 0.088, 2.1), sa("thigh-l", 0.26, 0.008, -0.005, 0.069, 0.074, 2), sa("thigh-l", 0.4, 0.006, 0, 0.053, 0.056, 2), sa("thigh-l", 0.42, 0.004, 0, 0.0515, 0.0545, 2)],
  shank: [sa("shank-l", -0.035, 0.002, 0, 0.04, 0.042, 2), sa("shank-l", 0.0, 0.004, 0, 0.046, 0.0485, 2), sa("shank-l", 0.13, -0.013, 0.002, 0.053, 0.058, 2), sa("shank-l", 0.26, -0.005, 0, 0.04, 0.044, 2), sa("shank-l", 0.36, 0, 0, 0.031, 0.035, 2)],
  sleeve: [sa("arm-l", -0.006, -0.003, -0.004, 0.05, 0.057, 2.2), sa("arm-l", 0.05, -0.002, 0.004, 0.064, 0.066, 2.1), sa("arm-l", 0.19, 0, 0.003, 0.058, 0.06, 2), sa("arm-l", 0.315, -0.006, 0.002, 0.053, 0.055, 2)],
  fore: [sa("forearm-l", -0.035, -0.008, 0.002, 0.045, 0.047, 2), sa("forearm-l", 0.03, -0.003, 0.002, 0.05, 0.052, 2), sa("forearm-l", 0.1, 0, 0.002, 0.048, 0.05, 2), sa("forearm-l", 0.2, 0, 0.001, 0.042, 0.042, 2), sa("forearm-l", 0.248, 0, 0, 0.031, 0.027, 2.2)],
  boot: [[-0.069, 0.0, 0.14, 0.022, 2.2], [-0.05, 0.0, 0.152, 0.034, 2.4], [-0.01, 0.0, 0.155, 0.039, 2.5], [0.03, 0.0, 0.118, 0.042, 2.6], [0.07, 0.0, 0.092, 0.045, 2.7], [0.135, 0.0, 0.064, 0.049, 2.8]],
  toe: [[-0.012, 0.0, 0.064, 0.049, 2.8], [0.035, 0.0, 0.054, 0.045, 2.6], [0.062, 0.002, 0.043, 0.034, 2.3], [0.076, 0.008, 0.032, 0.018, 2.0]],
  head: [[-0.068, 0.058, 0.09, 0.016, 2], [-0.045, -0.004, 0.097, 0.041, 2.1], [-0.01, -0.045, 0.1, 0.055, 2.2], [0.03, -0.074, 0.098, 0.067, 2.3], [0.075, -0.083, 0.099, 0.072, 2.3], [0.115, -0.075, 0.08, 0.066, 2.2], [0.14, -0.04, 0.035, 0.035, 2]],
  hair: [[0.157, -0.03, 0.028, 0.028, 2], [0.138, -0.085, 0.08, 0.066, 2.1], [0.1, -0.099, 0.096, 0.083, 2.2], [0.06, -0.1, 0.035, 0.088, 2.3], [0.0, -0.097, -0.016, 0.089, 2.35], [-0.056, -0.086, -0.03, 0.082, 2.2], [-0.07, -0.078, -0.036, 0.072, 2.1]],
  neck: [sa("neck", -0.02, 0.008, 0, 0.056, 0.058, 2.1), sa("neck", 0.05, 0.013, 0, 0.051, 0.053, 2), sa("neck", 0.105, 0.004, 0, 0.051, 0.05, 2)],
};
const S200: Spec = {
  ...S100,
  scale: 200,
  skirt: [[0.832, -0.172, 0.104, 0.198, 2.4], [0.91, -0.16, 0.118, 0.196, 2.4]],
  trunk: [[1.0, -0.12, 0.127, 0.187, 2.4], [1.15, -0.124, 0.135, 0.185, 2.4], [1.25, -0.14, 0.156, 0.194, 2.35], [1.35, -0.135, 0.105, 0.206, 2.6], [1.42, -0.103, 0.052, 0.16, 2.5], [1.456, -0.084, 0.024, 0.075, 2.1]],
  belly: [[0.12, 0.866, 1.207, 0.178, 2.2], [0.2, 0.898, 1.157, 0.14, 2.05], [0.245, 0.942, 1.09, 0.085, 2.0], [0.264, 0.998, 1.038, 0.03, 2.0]],
  seat: [[0.785, -0.104, 0.064, 0.152, 2.2], [0.86, -0.156, 0.092, 0.184, 2.4], [0.98, -0.108, 0.104, 0.16, 2.4]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.008, 0.088, 0.095, 2.2), sa("thigh-l", 0.2, 0.008, -0.006, 0.077, 0.081, 2.05), sa("thigh-l", 0.42, 0.004, 0, 0.052, 0.054, 2)],
  shank: [sa("shank-l", -0.02, 0.003, 0, 0.044, 0.046, 2), sa("shank-l", 0.13, -0.013, 0.002, 0.053, 0.058, 2), sa("shank-l", 0.36, 0, 0, 0.033, 0.037, 2)],
  sleeve: [sa("arm-l", -0.006, -0.003, -0.004, 0.052, 0.058, 2.2), sa("arm-l", 0.06, -0.002, 0.004, 0.064, 0.066, 2.1), sa("arm-l", 0.315, -0.006, 0.002, 0.054, 0.056, 2)],
  fore: [sa("forearm-l", -0.035, -0.008, 0.002, 0.045, 0.047, 2), sa("forearm-l", 0.04, -0.002, 0.002, 0.05, 0.051, 2), sa("forearm-l", 0.248, 0, 0, 0.034, 0.031, 2.1)],
  boot: [[-0.069, 0.0, 0.14, 0.024, 2.2], [-0.03, 0.0, 0.155, 0.038, 2.5], [0.04, 0.0, 0.11, 0.044, 2.6], [0.135, 0.0, 0.064, 0.049, 2.8]],
  toe: [[-0.012, 0.0, 0.064, 0.049, 2.8], [0.05, 0.0, 0.05, 0.042, 2.5], [0.077, 0.008, 0.034, 0.02, 2.0]],
  head: [[-0.066, 0.05, 0.091, 0.02, 2], [-0.03, -0.025, 0.1, 0.05, 2.2], [0.03, -0.074, 0.099, 0.067, 2.3], [0.09, -0.082, 0.096, 0.071, 2.3], [0.138, -0.045, 0.04, 0.04, 2]],
  hair: [[0.157, -0.032, 0.03, 0.03, 2], [0.146, -0.07, 0.066, 0.056, 2.1], [0.12, -0.096, 0.094, 0.079, 2.2], [0.06, -0.1, 0.035, 0.088, 2.3], [0.01, -0.098, -0.014, 0.089, 2.3], [-0.04, -0.092, -0.024, 0.086, 2.3], [-0.07, -0.078, -0.036, 0.072, 2.1]],
  neck: [sa("neck", -0.02, 0.008, 0, 0.056, 0.058, 2.1), sa("neck", 0.105, 0.004, 0, 0.051, 0.051, 2)],
};
const S500: Spec = {
  ...S200,
  scale: 500,
  skirt: [[0.835, -0.173, 0.106, 0.199, 2.4], [0.91, -0.166, 0.118, 0.198, 2.4]],
  trunk: [[1.0, -0.122, 0.128, 0.188, 2.4], [1.25, -0.141, 0.155, 0.196, 2.4], [1.38, -0.126, 0.088, 0.205, 2.7], [1.455, -0.086, 0.026, 0.085, 2.1]],
  belly: [[0.12, 0.866, 1.207, 0.178, 2.2], [0.215, 0.906, 1.14, 0.128, 2.0], [0.263, 0.995, 1.04, 0.038, 2.0]],
  seat: [[0.785, -0.106, 0.066, 0.154, 2.2], [0.87, -0.157, 0.096, 0.185, 2.4], [0.98, -0.11, 0.105, 0.162, 2.4]],
  thigh: [sa("thigh-l", -0.06, -0.01, 0.008, 0.089, 0.096, 2.2), sa("thigh-l", 0.42, 0.004, 0, 0.054, 0.056, 2)],
  shank: [sa("shank-l", -0.02, 0.003, 0, 0.045, 0.047, 2), sa("shank-l", 0.14, -0.012, 0.002, 0.054, 0.058, 2), sa("shank-l", 0.36, 0, 0, 0.04, 0.042, 2)],
  sleeve: [sa("arm-l", -0.006, -0.003, -0.004, 0.056, 0.06, 2.2), sa("arm-l", 0.315, -0.006, 0.002, 0.058, 0.06, 2)],
  fore: [sa("forearm-l", -0.035, -0.008, 0.002, 0.047, 0.049, 2), sa("forearm-l", 0.05, -0.002, 0.002, 0.052, 0.053, 2), sa("forearm-l", 0.248, 0, 0, 0.04, 0.04, 2)],
  boot: [[-0.069, 0.0, 0.14, 0.03, 2.2], [0.0, 0.0, 0.155, 0.042, 2.5], [0.135, 0.0, 0.075, 0.05, 2.8]],
  toe: [[-0.012, 0.0, 0.075, 0.05, 2.8], [0.08, 0.0, 0.045, 0.038, 2.0]],
  head: [[-0.066, 0.045, 0.092, 0.03, 2], [0.0, -0.06, 0.1, 0.06, 2.2], [0.08, -0.083, 0.098, 0.072, 2.3], [0.14, -0.04, 0.036, 0.04, 2]],
  hair: [[0.158, -0.04, 0.04, 0.04, 2], [0.142, -0.078, 0.076, 0.066, 2.1], [0.1, -0.1, 0.097, 0.085, 2.2], [0.05, -0.1, 0.03, 0.089, 2.3], [0.0, -0.098, -0.016, 0.09, 2.35], [-0.072, -0.08, -0.03, 0.075, 2.1]],
  neck: [sa("neck", -0.02, 0.01, 0, 0.056, 0.058, 2.1), sa("neck", 0.105, 0.004, 0, 0.052, 0.052, 2)],
};

const headSt = ([a, b, f, hw, e]: readonly number[]): St => sa("head", a!, (b! + f!) / 2, 0, hw!, (f! - b!) / 2, e!);
const xSlice = (x: number, lo: number, hi: number, hw: number, e: number, l = 0, ground = G): St => ({ at: [x, ground + (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });

/** ✋ A mitten hand: the palm with the fingers merged into it, curled (`curl` along the palm side), and at 1:100 a thumb. */
function hand(sp: Spec, curl: number, prefix: string): Json[] {
  const c = (id: string): Json => C(id, "hand-l", MAT.skin, TONE.hand);
  const t = (x: number): number => (sp.scale === 500 ? 0.0382 : x);
  const palm: St[] = [sa("hand-l", -0.014, 0, 0, t(0.016), Math.max(0.026, t(0)), 2.3), sa("hand-l", 0.055, 0.002, -0.0015, t(0.0135), 0.037, 2.5), sa("hand-l", 0.11, 0, -0.004 - curl * 0.004, t(0.012), 0.037, 2.4), sa("hand-l", 0.162, 0, -0.01 - curl * 0.012, t(0.0105), 0.032, 2.2)];
  const out = [loft(c(prefix), dome(sp.scale === 100 ? palm : [palm[0]!, palm[1]!, palm[3]!], 1, 0.012, 1))];
  if (sp.scale === 100) out.push(capsule(c(`${prefix}-thumb`), pa("hand-l", 0.014, 0.022, -0.006), pa("hand-l", 0.098, 0.06, -0.03), 0.0115, 0.0085));
  return out;
}

function body(sp: Spec): Json[] {
  const out: Json[] = [];
  const tilt = (i: number): number => (i === 0 ? -12 : 0);
  out.push(loft(C("sweater", "torso", MAT.knit, TONE.sweater), dense([...sp.skirt.map((s, i) => skirtSt(s, tilt(i))), ...sp.trunk.map((s) => torsoSt(s))], sp.scale === 100 ? 2 : 1)));
  out.push(loft(C("belly-cover", "belly", MAT.knit, TONE.belly), dome(sp.belly.map((c) => { const [a, f, hw, hh, e] = bellyRow(c); return sa("belly", a, f, 0, hw, hh, e); }), 1, 0.003, 1), [false, true]));
  out.push(loft(C("leggings-seat", "pelvis", MAT.leggings, TONE.leggings), sp.seat.map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw, hd, e); }), [true, true], { sectionFaces: ["+u"] }));
  out.push(loft(C("thigh-l", "thigh-l", MAT.leggings, TONE.leggings), sp.thigh));
  out.push(loft(C("shank-l", "shank-l", MAT.leggings, TONE.leggings), sp.shank));
  out.push(loft(C("sleeve-l", "arm-l", MAT.knit, TONE.sweater), dome(sp.sleeve, -1, 0.008, 1)));
  out.push(loft(C("forearm-l", "forearm-l", MAT.knit, TONE.sweater), dome(sp.fore, -1, 0.02, 1)));
  out.push(loft(C("boot-l", "foot-l", MAT.boot, TONE.boot), sp.boot.map(([x, lo, hi, hw, e]) => xSlice(x, lo, hi, hw, e))));
  out.push(loft(C("boot-toe-l", "toe-l", MAT.boot, TONE.boot), dome(sp.toe.map(([x, lo, hi, hw, e], i) => xSlice(x, lo, hi, hw, e, -0.003 * i, 0)), 1, 0.004, 1)));
  for (const s of [...out].filter((q) => /-l$/.test(q.id))) out.push(mirrored(s, toRight));
  out.push(...hand(sp, 1, "hand-l"));
  out.push(...hand(sp, 0.4, "hand-l").map((s) => mirrored(s, toRight)));
  out.push(loft(C("head", "head", MAT.skin, TONE.skin), dome(sp.head.map(headSt), 1, 0.003, 1)));
  if (sp.scale === 100) {
    out.push(loft(C("nose", "head", MAT.skin, TONE.skin), [sa("head", 0.04, 0.094, 0, 0.0075, 0.0075, 2), sa("head", 0.0, 0.106, 0, 0.0145, 0.0085, 2)]));
    for (const side of [1, -1]) out.push(ellipsoid(C(`ear-${side > 0 ? "l" : "r"}`, "head", MAT.skin, TONE.skin), [0.029, 0.016, 0.0077], pa("head", 0.03, 0.002, side * 0.069), compose(aboutU(side * -14), aboutW(14))));
  }
  out.push(loft(C("hair", "head", MAT.hair, TONE.hair), dome(sp.hair.map(headSt), -1, 0.005, 1)));
  out.push(loft(C("neck", "neck", MAT.skin, TONE.skin), sp.neck));
  return out;
}

/** 🌡️ One surface solid per region, the outlets at nostrils, mouth and crown sized to the scale's smallest feature. */
function thermal(b: Json[], sp: Spec): Json[] {
  const f = sp.scale * 0.00015;
  const out: Json[] = [];
  const head = b.find((s) => s.id === "head")!;
  out.push({ ...head, id: "th-face", region: "face-skin", sections: head.sections.filter((q: Json) => q.at[0] <= 0.1), capEnd: false });
  for (const id of ["nose", "ear-l", "ear-r", "neck"]) if (b.some((s) => s.id === id)) out.push(thOf(id, "face-skin"));
  out.push(thOf("hair", "hair", "hair"));
  for (const s of b.filter((q) => /^hand-[lr]/.test(q.id))) out.push(thOf(s.id, "hand-skin"));
  for (const id of ["sweater", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r"]) out.push(thOf(id, "sweater", "knit"));
  out.push(thOf("belly-cover", "belly-sweater", "knit"));
  for (const id of ["leggings-seat", "thigh-l", "thigh-r", "shank-l", "shank-r"]) out.push(thOf(id, "leggings", "leggings"));
  for (const id of ["boot-l", "boot-r", "boot-toe-l", "boot-toe-r"]) out.push(thOf(id, "ankle-boots", "boot"));
  const t = Math.max(f, 0.0078) / 2 + 0.0002;
  out.push({ ...ellipsoid(C("th-nose-breath", "head", MAT.skin, TONE.skin, "detail"), [t, t, Math.max(t, 0.0085)], pa("head", -0.002, 0.106 - t + 0.004, 0)), region: "face-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", MAT.skin, TONE.lips, "detail"), [t, t, Math.max(t, 0.0115)], pa("head", -0.022, 0.1 - t + 0.004, 0)), region: "face-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-body-plume", "head", MAT.hair, TONE.hair, "detail"), [t, Math.max(t, 0.06), Math.max(t, 0.05)], pa("head", 0.158 - t + 0.004, 0, 0)), region: "sweater", tags: ["emitter", "body-plume"] });
  return out;
}

const B = (id: string, bone: string): Json => C(id, bone, MAT.bone, TONE.bone);
const bar = (id: string, bone: string, pts: V3[], r: number[] | number, member = bone): Json => ({ ...tube(B(id, bone), pts, Array.isArray(r) ? r : pts.map(() => r)), member, part: "member" });
const el = (id: string, bone: string, radii: V3, at: V3, part: "member" | "joint" | "mass" = "member", member = bone, rot?: [number, number, number, number]): Json => ({ ...ellipsoid(C(id, bone, MAT.bone, part === "member" ? TONE.bone : part === "joint" ? TONE.joint : TONE.mass), radii, at, rot), member, part });

/** 🦴 The skeleton of a coarse scale: bones merged by scale, joints at the rig joints, masses at the centres of mass, the fetus merged by scale. */
function structure(sp: Spec): Json[] {
  const fine = sp.scale === 100;
  const half = sp.scale * 0.00015 * 0.5 + 0.0003;
  const r = (x: number): number => Math.max(x, half);
  const out: Json[] = [];
  const spine: V3[] = [[-0.01, 0.002], [0.09, 0.022], [0.19, -0.004], [0.29, -0.031], [0.39, -0.026], [0.458, 0]].map(([a, f]) => pa("torso", a!, f!, 0));
  out.push(bar("spine", "torso", spine, [0.023, 0.023, 0.02, 0.016, 0.014, 0.012].map(r)));
  const ribs: [number, number, number][] = fine ? [[0.415, 0.07, 0.065], [0.37, 0.104, 0.1], [0.32, 0.116, 0.112], [0.275, 0.124, 0.112], [0.23, 0.127, 0.095], [0.19, 0.114, 0.05]] : sp.scale === 200 ? [[0.38, 0.1, 0.09], [0.3, 0.12, 0.112], [0.225, 0.125, 0.095]] : [[0.31, 0.105, 0.11]];
  ribs.forEach(([a, w, drop], i) => {
    const pts: V3[] = [[a, -0.06, 0.02], [a, -0.07, w * 0.5], [a - drop * 0.4, -0.02, w * 0.97], [a - drop * 0.8, 0.07, w * 0.65], [a - drop * 0.92, 0.1, 0.02]].map(([aa, f, l]) => pa("torso", aa!, f!, l!));
    const rib = bar(`rib-${i + 1}-l`, "torso", pts, r(fine ? 0.0075 : 0.008));
    out.push(rib, mirrored(rib, toRight));
  });
  if (sp.scale !== 500) {
    out.push({ ...loft(B("sternum", "torso"), [sa("torso", 0.408, 0.088, 0, 0.022, r(0.009), 2.4), sa("torso", 0.23, 0.118, 0, r(0.014), r(0.008), 2.2)]), member: "torso", part: "member" });
    const girdle = bar("shoulder-girdle-l", "torso", [[0.41, 0.083, 0.02], [0.425, 0.06, 0.115], [0.43, 0.01, 0.16], [0.405, -0.065, 0.11], [0.3, -0.073, 0.088]].map(([a, f, l]) => pa("torso", a!, f!, l!)), r(fine ? 0.008 : 0.01));
    out.push(girdle, mirrored(girdle, toRight));
  }
  const ilium = { ...loft(B("hip-bone-l", "pelvis"), [sa("pelvis", 0.12, -0.012, 0.118, 0.076, r(0.008), 2.2), sa("pelvis", 0.055, -0.004, 0.11, 0.05, r(0.013), 2.2), sa("pelvis", -0.03, -0.006, 0.08, 0.03, r(0.02), 2)].map((q, i) => ({ ...q, yaw: [55, 42, 0][i] }))), member: "pelvis", part: "member" };
  out.push(ilium, mirrored(ilium, toRight));
  out.push({ ...loft(B("sacrum", "pelvis"), [sa("pelvis", 0.095, -0.032, 0, 0.054, r(0.02), 2.2, -28), sa("pelvis", -0.03, -0.098, 0, r(0.014), r(0.009), 2, -58)]), member: "pelvis", part: "member" });
  if (sp.scale !== 500) out.push(bar("pubic-arch", "pelvis", [[-0.01, 0.02, 0.08], [-0.03, 0.064, 0.0], [-0.01, 0.02, -0.08]].map(([a, f, l]) => pa("pelvis", a!, f!, l!)), r(0.011)));
  out.push(el("cranium", "head", [0.073, 0.087, 0.067], pa("head", 0.072, -0.004, 0)));
  if (sp.scale !== 500) out.push(el("facial-skeleton", "head", [0.042, 0.03, 0.05], pa("head", 0.024, 0.058, 0)), bar("mandible", "head", [[0.015, -0.006, 0.05], [-0.025, -0.004, 0.047], [-0.05, 0.05, 0.03], [-0.058, 0.078, 0.0], [-0.05, 0.05, -0.03], [-0.025, -0.004, -0.047], [0.015, -0.006, -0.05]].map(([a, f, l]) => pa("head", a!, f!, l!)), r(0.0085)));
  out.push(bar("cervical-spine", "neck", [pa("neck", -0.005, 0, 0), pa("neck", 0.05, 0.003, 0), pa("neck", 0.098, 0.002, 0)], r(0.012)));
  const limbs: Json[] = [
    bar("femur-l", "thigh-l", [pa("thigh-l", 0, 0, 0), pa("thigh-l", 0.03, -0.006, 0.05), pa("thigh-l", 0.15, 0.007, 0.038), pa("thigh-l", 0.385, -0.004, 0)], [0.021, 0.016, 0.013, 0.024].map(r)),
    bar("tibia-l", "shank-l", [pa("shank-l", 0, 0, 0), pa("shank-l", 0.2, 0.009, -0.004), pa("shank-l", 0.385, 0.002, -0.004)], [0.028, 0.0135, 0.016].map(r)),
    bar("foot-bones-l", "foot-l", [[-0.048, -0.042, 0.004], [0.0, -0.012, 0], [0.06, -0.03, 0], [0.122, -RIG.ankle + 0.026, 0]], [0.018, 0.019, 0.02, 0.014].map(r)),
    bar("humerus-l", "arm-l", [pa("arm-l", 0, 0, -0.004), pa("arm-l", 0.15, 0.002, 0.003), pa("arm-l", 0.298, -0.004, 0)], [0.021, 0.011, 0.02].map(r)),
    bar("forearm-bones-l", "forearm-l", [pa("forearm-l", -0.015, -0.01, -0.004), pa("forearm-l", 0.12, 0.003, 0), pa("forearm-l", 0.238, 0.002, 0)], [0.012, 0.011, 0.015].map(r)),
    { ...loft(B("hand-bones-l", "hand-l"), [sa("hand-l", 0.0, 0, -0.001, r(0.008), Math.max(r(0), 0.023), 2.2), sa("hand-l", 0.09, 0, -0.003, r(0.007), Math.max(r(0), 0.032), 2.4), sa("hand-l", 0.14, 0, -0.012, r(0.006), Math.max(r(0), 0.027), 2)]), member: "hand-l", part: "member" },
  ];
  if (fine) limbs.push(bar("toe-bones-l", "toe-l", [[0.0, RIG.ball + 0.012, -0.004], [0.048, RIG.ball + 0.008, -0.008]], 0.0115), bar("fibula-l", "shank-l", [pa("shank-l", 0.04, -0.012, 0.03), pa("shank-l", 0.395, -0.008, 0.026)], 0.0078), el("patella-l", "shank-l", [0.021, 0.01, 0.019], pa("shank-l", -0.012, 0.042, 0)));
  for (const q of limbs) out.push(q, mirrored(q, toRight));
  const P = (a: number, f: number, l: number): V3 => pa("belly", a, f, l);
  if (sp.scale === 100) {
    out.push(el("fetal-skull", "belly", [0.055, 0.046, 0.045], P(0.115, -0.115, 0.004), "member", "belly", aboutW(-62)));
    out.push(bar("fetal-spine", "belly", [P(0.13, -0.065, 0.03), P(0.175, 0.01, 0.055), P(0.165, 0.1, 0.045), P(0.145, 0.13, 0.03)], 0.0078));
    out.push(el("fetal-trunk", "belly", [0.05, 0.04, 0.036], P(0.16, 0.03, 0.02)));
    out.push(bar("fetal-legs", "belly", [P(0.14, 0.12, 0.015), P(0.19, 0.075, -0.02), P(0.13, 0.135, -0.055)], 0.0078));
  } else if (sp.scale === 200) {
    out.push(el("fetal-skull", "belly", [0.055, 0.046, 0.045], P(0.115, -0.115, 0.004), "member", "belly", aboutW(-62)));
    out.push(el("fetal-body", "belly", [0.1, 0.05, 0.05], P(0.16, 0.04, 0.02), "member", "belly", aboutW(80)));
  } else out.push(el("fetal-unit", "belly", [0.085, 0.15, 0.09], P(0.15, -0.01, 0.01), "member", "belly", aboutW(80)));
  const jr = sp.scale === 500 ? 0.028 : sp.scale === 200 ? 0.02 : 0.016;
  const J = (x: number): number => Math.max(x, half);
  out.push(el("joint-lumbosacral", "torso", [J(jr * 0.8), J(jr * 1.4), J(jr * 1.4)], [0, 0, 0], "joint"));
  out.push(el("joint-cervicothoracic", "neck", [J(jr * 0.7), J(jr), J(jr)], [0, 0, 0], "joint"));
  if (sp.scale !== 500) out.push(el("joint-abdomen", "belly", [J(jr * 0.8), J(jr), J(jr * 1.2)], pa("belly", 0.035, 0, 0), "joint"));
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
  for (const bn of ASSET.rig.bones as Json[]) if (bn.mass && bn.mass.kg >= (sp.scale === 500 ? 6 : 3)) { const m = Math.max(r(0.022), sp.scale === 500 ? 0.04 : 0); out.push(el(`mass-${bn.id}`, bn.id, [m, m, m], [bn.mass.comX, bn.mass.comY ?? 0, 0], "mass")); }
  return out;
}

/** 📚 The three coarse scales. */
export function coarseVolumes(): Json {
  const out: Json = {};
  for (const sp of [S100, S200, S500]) {
    const b = body(sp);
    out[String(sp.scale)] = { status: "authored", denominator: sp.scale, body: b, thermal: thermal(b, sp), structure: structure(sp), decisions: [] };
  }
  return out;
}
void dome;
void dense;
