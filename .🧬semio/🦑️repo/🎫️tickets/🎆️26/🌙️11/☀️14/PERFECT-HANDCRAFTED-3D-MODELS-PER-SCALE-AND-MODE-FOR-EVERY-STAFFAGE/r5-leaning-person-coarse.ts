/** 🧍 The 1:100, 1:200 and 1:500 models of `humans/leaning-person`, each stated anew for its smallest feature (15, 30, 75 mm): fewer and simpler sections, merged masses (hem into hoodie, cuff into sleeve, fingers into a mitten, the hood into the back at 1:200 and 1:500), the same man in the same clothes and proportions. Thermal: one surface solid per region (the layers are merged, B2) and the three outlets; structure: a merged skeleton (1:100), a skeleton of bars and hoops (1:200) and a stick skeleton (1:500), with the wall as the member of its bones.
 * @see ./r5-leaning-person-s50.ts
 * @see ./r3-humans-leaning-person.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, aboutU, aboutW, box, capsule, compose, dome, ellipsoid, extrude, loft, mirrored, pa, sa, thOf, toRight, tube, type St, type V3 } from "./r3-leaning-person-kit.ts";
import { PHONE } from "./r3-leaning-person-data.ts";
import { PELVIS_LEN, TONE, hood50, trunkF, wall } from "./r5-leaning-person-s50.ts";

type Row = readonly [number, number, number, number, number];
const C = (id: string, bone: string, material: string, tone: number, role = "body") => ({ id, bone, material, tone, role });
const tSt = ([h, b, f, hw, e]: Row): St => sa("torso", h - PELVIS_LEN, (b + f) / 2, 0, hw, (f - b) / 2, e);
const pSt = ([a, b, f, hw, e]: Row): St => sa("pelvis", a, (b + f) / 2, 0, hw, (f - b) / 2, e);
const hSt = ([a, b, f, hw, e]: Row): St => sa("head", a, (b + f) / 2, 0, hw, (f - b) / 2, e);
const fSt = ([x, lo, hi, hw, e]: Row, l = 0): St => ({ at: [x, (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });
const G = -0.088;
const SIDES = ["l", "r"] as const;

/** 🦴 Rules for one coarse scale: the trunk, seat, head and limb stations, and how far the detail goes. */
type Spec = {
  scale: 100 | 200 | 500;
  trunk: Row[];
  hood: "pouch" | "merged";
  seat: Row[];
  thigh: St[];
  shank: St[];
  sleeve: St[];
  fore: St[];
  head: Row[];
  beanie: St[];
  hair: St[];
  neck: St[];
  shoe: Row[];
  toe: Row[];
  face: boolean;
  fingers: boolean;
};

const S100: Spec = {
  scale: 100,
  trunk: [[-0.092, -0.123, 0.113, 0.168, 2.4], [-0.02, -0.13, 0.122, 0.178, 2.5], [0.1, -0.121, 0.126, 0.171, 2.5], [0.25, -0.113, 0.138, 0.168, 2.5], [0.36, -0.115, 0.145, 0.175, 2.45], [0.455, -0.114, 0.129, 0.19, 2.35], [0.51, -0.102, 0.108, 0.192, 2.3], [0.55, -0.082, 0.085, 0.17, 2.15], [0.578, -0.067, 0.069, 0.125, 2.0], [0.597, -0.058, 0.062, 0.076, 2.0]],
  hood: "pouch",
  seat: [[-0.098, -0.076, 0.058, 0.118, 2.2], [-0.05, -0.115, 0.091, 0.16, 2.3], [0.03, -0.108, 0.1, 0.152, 2.3], [0.1, -0.095, 0.1, 0.145, 2.3]],
  thigh: [sa("thigh-l", -0.05, -0.006, -0.002, 0.086, 0.098, 2.2), sa("thigh-l", 0.12, 0.004, -0.008, 0.084, 0.09, 2.1), sa("thigh-l", 0.28, 0.007, -0.006, 0.072, 0.078, 2), sa("thigh-l", 0.425, 0.003, -0.002, 0.057, 0.061, 2)],
  shank: [sa("shank-l", -0.012, 0.004, -0.002, 0.058, 0.062, 2), sa("shank-l", 0.13, -0.01, 0, 0.058, 0.062, 2), sa("shank-l", 0.28, -0.006, 0, 0.054, 0.057, 2), sa("shank-l", 0.392, -0.01, 0, 0.056, 0.06, 2, -12)],
  sleeve: [sa("upper-arm-l", -0.02, -0.003, -0.016, 0.036, 0.044, 2.1), sa("upper-arm-l", 0.0, -0.002, -0.004, 0.049, 0.054, 2.1), sa("upper-arm-l", 0.16, 0.004, 0.003, 0.051, 0.054, 2), sa("upper-arm-l", 0.335, -0.004, 0.002, 0.045, 0.047, 2)],
  fore: [sa("forearm-l", -0.03, -0.006, 0.002, 0.045, 0.047, 2), sa("forearm-l", 0.1, 0.0, 0.002, 0.041, 0.042, 2), sa("forearm-l", 0.2, 0.0, 0.001, 0.036, 0.034, 2), sa("forearm-l", 0.236, 0, 0, 0.032, 0.026, 2)],
  head: [[0.02, 0.036, 0.086, 0.024, 2], [0.046, -0.018, 0.098, 0.05, 2.2], [0.08, -0.045, 0.102, 0.06, 2.2], [0.12, -0.085, 0.099, 0.07, 2.3], [0.16, -0.097, 0.099, 0.075, 2.3], [0.2, -0.095, 0.087, 0.074, 2.2], [0.235, -0.066, 0.05, 0.052, 2]],
  beanie: [sa("head", 0.164, -0.004, 0, 0.0855, 0.1085, 2.2, -13), sa("head", 0.19, -0.005, 0, 0.0845, 0.107, 2.2, -11), sa("head", 0.225, -0.012, 0, 0.072, 0.09, 2.05, -5), sa("head", 0.25, -0.02, 0, 0.046, 0.058, 2, 0)],
  hair: [sa("head", 0.1, -0.05, 0, 0.058, 0.03, 2), sa("head", 0.135, -0.02, 0, 0.076, 0.079, 2.2), sa("head", 0.18, -0.022, 0, 0.0775, 0.08, 2.2)],
  neck: [sa("head", -0.06, -0.006, 0, 0.066, 0.064, 2.1), sa("head", 0.04, 0.005, 0, 0.057, 0.056, 2), sa("head", 0.115, -0.012, 0, 0.052, 0.05, 2)],
  shoe: [[-0.073, G + 0.0, G + 0.074, 0.03, 2.4], [-0.04, G + 0.0, G + 0.08, 0.04, 2.6], [0.02, G + 0.0, G + 0.088, 0.045, 2.6], [0.06, G + 0.0, G + 0.098, 0.048, 2.6], [0.11, G + 0.0, G + 0.066, 0.053, 2.8], [0.15, G + 0.0, G + 0.058, 0.054, 2.8]],
  toe: [[-0.012, 0.0, 0.058, 0.054, 2.8], [0.035, 0.0, 0.049, 0.05, 2.6], [0.065, 0.002, 0.042, 0.04, 2.3], [0.086, 0.007, 0.033, 0.028, 2.1]],
  face: true,
  fingers: true,
};
const S200: Spec = {
  ...S100,
  scale: 200,
  trunk: [[-0.092, -0.124, 0.114, 0.17, 2.4], [0.05, -0.128, 0.126, 0.176, 2.5], [0.3, -0.12, 0.142, 0.17, 2.5], [0.43, -0.13, 0.133, 0.186, 2.5], [0.51, -0.124, 0.108, 0.192, 2.3], [0.56, -0.106, 0.082, 0.16, 2.15], [0.6, -0.08, 0.066, 0.09, 2.0]],
  hood: "merged",
  seat: [[-0.098, -0.078, 0.06, 0.12, 2.2], [-0.04, -0.113, 0.092, 0.158, 2.3], [0.08, -0.1, 0.098, 0.15, 2.3]],
  thigh: [sa("thigh-l", -0.05, -0.006, -0.006, 0.081, 0.093, 2.2), sa("thigh-l", 0.2, 0.006, -0.007, 0.079, 0.085, 2.1), sa("thigh-l", 0.425, 0.003, -0.002, 0.058, 0.062, 2)],
  shank: [sa("shank-l", -0.012, 0.004, -0.002, 0.059, 0.063, 2), sa("shank-l", 0.15, -0.01, 0, 0.058, 0.062, 2), sa("shank-l", 0.382, -0.008, 0, 0.056, 0.06, 2, -10)],
  sleeve: [sa("upper-arm-l", -0.02, -0.003, -0.014, 0.038, 0.045, 2.1), sa("upper-arm-l", 0.04, 0.0, 0.002, 0.054, 0.057, 2.1), sa("upper-arm-l", 0.335, -0.004, 0.002, 0.046, 0.048, 2)],
  fore: [sa("forearm-l", -0.03, -0.006, 0.002, 0.046, 0.048, 2), sa("forearm-l", 0.236, 0, 0, 0.034, 0.03, 2)],
  head: [[0.022, 0.03, 0.088, 0.03, 2], [0.06, -0.03, 0.1, 0.055, 2.2], [0.12, -0.086, 0.1, 0.07, 2.3], [0.18, -0.098, 0.096, 0.076, 2.3], [0.23, -0.072, 0.056, 0.056, 2]],
  beanie: [sa("head", 0.164, -0.004, 0, 0.0855, 0.1085, 2.2, -13), sa("head", 0.21, -0.008, 0, 0.079, 0.099, 2.1, -8), sa("head", 0.25, -0.02, 0, 0.047, 0.06, 2, 0)],
  hair: [sa("head", 0.1, -0.048, 0, 0.06, 0.034, 2), sa("head", 0.17, -0.022, 0, 0.078, 0.082, 2.2)],
  neck: [sa("head", -0.06, -0.006, 0, 0.066, 0.064, 2.1), sa("head", 0.11, -0.01, 0, 0.054, 0.052, 2)],
  shoe: [[-0.073, G + 0.0, G + 0.08, 0.034, 2.4], [0.0, G + 0.0, G + 0.093, 0.046, 2.6], [0.06, G + 0.0, G + 0.101, 0.049, 2.6], [0.15, G + 0.0, G + 0.064, 0.055, 2.8]],
  toe: [[-0.012, 0.0, 0.058, 0.054, 2.8], [0.055, 0.0, 0.045, 0.046, 2.5], [0.088, 0.007, 0.034, 0.03, 2.1]],
  face: false,
  fingers: false,
};
const S500: Spec = {
  ...S200,
  scale: 500,
  trunk: [[-0.094, -0.125, 0.115, 0.172, 2.4], [0.18, -0.12, 0.135, 0.172, 2.5], [0.44, -0.13, 0.13, 0.19, 2.5], [0.53, -0.122, 0.095, 0.183, 2.3], [0.6, -0.088, 0.07, 0.095, 2.0]],
  seat: [[-0.098, -0.08, 0.062, 0.122, 2.2], [-0.03, -0.112, 0.093, 0.158, 2.3], [0.08, -0.1, 0.098, 0.15, 2.3]],
  thigh: [sa("thigh-l", -0.05, -0.006, -0.006, 0.08, 0.092, 2.2), sa("thigh-l", 0.425, 0.003, -0.002, 0.06, 0.064, 2)],
  shank: [sa("shank-l", -0.012, 0.004, -0.002, 0.06, 0.064, 2), sa("shank-l", 0.382, -0.008, 0, 0.056, 0.06, 2, -10)],
  head: [[0.024, 0.025, 0.088, 0.04, 2], [0.09, -0.06, 0.103, 0.065, 2.2], [0.17, -0.098, 0.098, 0.076, 2.3], [0.228, -0.075, 0.058, 0.058, 2]],
  beanie: [sa("head", 0.166, -0.005, 0, 0.086, 0.109, 2.2, -12), sa("head", 0.218, -0.01, 0, 0.078, 0.097, 2.1, -6), sa("head", 0.25, -0.02, 0, 0.05, 0.062, 2, 0)],
  hair: [sa("head", 0.1, -0.05, 0, 0.062, 0.036, 2), sa("head", 0.165, -0.025, 0, 0.078, 0.08, 2.2)],
  shoe: [[-0.074, G + 0.0, G + 0.077, 0.036, 2.4], [0.04, G + 0.0, G + 0.095, 0.047, 2.6], [0.132, G + 0.0, G + 0.06, 0.053, 2.8]],
  toe: [[0.002, 0.0, 0.056, 0.054, 2.8], [0.06, 0.0, 0.046, 0.05, 2.5], [0.09, 0.008, 0.035, 0.034, 2.1]],
};

/** 🙋 A coarse body: the same garments as at 1:50 with the scale's merged parts. */
function body(sp: Spec): Json[] {
  const out: Json[] = [];
  const back = (h: number): number => trunkF(sp.trunk, h, 0, -1);
  out.push(loft(C("hoodie", "torso", "cotton-clothing", TONE.hoodie), sp.trunk.map(tSt)));
  if (sp.hood === "pouch") out.push(...hood50(100, sp.trunk));
  if (sp.scale === 100) for (const side of [1, -1]) out.push({ ...tube(C(`hood-string-${side > 0 ? "l" : "r"}`, "torso", "cotton-clothing", TONE.string, "detail"), [0.565, 0.47, 0.37].map((h) => pa("torso", h - PELVIS_LEN, trunkF(sp.trunk, h, side * 0.04, 1) + 0.005, side * 0.04)), [0.004, 0.004, 0.004]), slender: true });
  out.push(loft(C("jeans-seat", "pelvis", "cotton-clothing", TONE.jeans), sp.seat.map(pSt)));
  out.push(loft(C("thigh-l", "thigh-l", "cotton-clothing", TONE.jeans), dome(sp.thigh, 1, 0.03, 2)));
  out.push(loft(C("shank-l", "shank-l", "cotton-clothing", TONE.jeans), sp.shank));
  out.push(loft(C("upper-arm-l", "upper-arm-l", "cotton-clothing", TONE.hoodie), dome(dome(sp.sleeve, -1, 0.008, 2), 1, 0.018, 2)));
  out.push(loft(C("forearm-l", "forearm-l", "cotton-clothing", TONE.hoodie), sp.fore));
  const ww = sp.scale === 500 ? [0.034, 0.032] : sp.scale === 200 ? [0.03, 0.03] : [0.0285, 0.0285];
  out.push(loft(C("wrist-l", "forearm-l", "skin", TONE.hand), [sa("forearm-l", 0.226, 0, 0, ww[0]!, Math.max(0.022, ww[1]! - 0.008), 2.2), sa("forearm-l", 0.262, 0, 0, ww[0]!, Math.max(0.02, ww[1]! - 0.01), 2.3)]));
  out.push(loft(C("shoe-l", "foot-l", "leather-shoe", TONE.shoe), sp.shoe.map((r) => fSt(r))));
  out.push(loft(C("shoe-toe-l", "toe-l", "leather-shoe", TONE.shoe), dome(sp.toe.map((r, i) => fSt(r, -0.003 * i)), 1, 0.004, 1)));
  out.push(...leftHand(sp));
  for (const s of out.filter((q) => /-l$/.test(q.id) && /^(thigh|shank|upper-arm|forearm|wrist|shoe)/.test(q.id))) out.push(mirrored(s, toRight));
  out.push(...rightHand(sp));
  out.push(loft(C("neck", "head", "skin", TONE.skin), sp.neck));
  out.push(loft(C("head", "head", "skin", TONE.skin), dome(sp.head.map(hSt), 1, 0.004, 1)));
  if (sp.face) {
    out.push(loft(C("nose", "head", "skin", TONE.skin), [sa("head", 0.142, 0.096, 0, 0.0085, 0.0075, 2), sa("head", 0.1, 0.11, 0, 0.0135, 0.0105, 2)]));
    for (const side of [1, -1]) out.push(ellipsoid(C(`ear-${side > 0 ? "l" : "r"}`, "head", "skin", TONE.skin), [0.031, 0.018, 0.009], pa("head", 0.126, -0.013, side * 0.073), compose(aboutU(side * -12), aboutW(16))));
  }
  out.push(loft(C("hair", "head", "hair", TONE.hair), sp.hair));
  out.push(loft(C("beanie", "head", "wool-hat", TONE.beanie), dome(sp.beanie, 1, 0.004, 1)));
  out.push(wall());
  return out;
}
/** ✋ The relaxed left hand as a mitten with a thumb (1:100 with a finger mass that curls in). */
function leftHand(sp: Spec): Json[] {
  const c = (id: string): Json => C(id, "hand-l", "skin", TONE.hand);
  const k = sp.scale === 500 ? 1.8 : 1;
  const palm: St[] = [sa("hand-l", -0.012, 0, 0, 0.0175 * k, 0.028 * k, 2.3), sa("hand-l", 0.06, 0.001, -0.002, 0.0155 * k, 0.042, 2.5), sa("hand-l", 0.1, 0.0, -0.006, 0.014 * k, 0.04, 2.4), sa("hand-l", 0.145, 0.0, -0.016, 0.012 * k, 0.036, 2.2), sa("hand-l", 0.17, 0.0, -0.026, 0.01 * k, 0.03, 2)];
  const out = [loft(c("hand-l"), dome(sp.fingers ? palm : [palm[0]!, palm[1]!, palm[3]!], 1, 0.008, 1))];
  if (sp.scale === 100) out.push(capsule(c("hand-l-thumb"), pa("hand-l", 0.015, 0.026, -0.006), pa("hand-l", 0.075, 0.05, -0.024), 0.012, 0.0095));
  return out;
}
/** 📱 The right hand as a mitten closed on the phone, the thumb on its near edge. */
function rightHand(sp: Spec): Json[] {
  const c = (id: string): Json => C(id, "hand-r", "skin", TONE.hand);
  const out: Json[] = [];
  const edge = PHONE.u + PHONE.width / 2;
  const k = sp.scale === 500 ? 1.6 : 1;
  out.push(loft(c("hand-r"), dome([sa("hand-r", -0.012, 0, 0, 0.0175 * k, 0.028, 2.3), sa("hand-r", 0.06, 0.001, 0.0, 0.0155 * k, 0.042, 2.5), sa("hand-r", 0.1, 0.0, 0.006, 0.016 * k, 0.04, 2.4), sa("hand-r", edge - 0.004, 0.0, 0.014, 0.018 * k, 0.036, 2.2)], 1, 0.01, 1)));
  if (sp.scale === 100) out.push(capsule(c("hand-r-thumb"), pa("hand-r", 0.015, 0.026, 0.006), [PHONE.u - PHONE.width / 2 + 0.01, 0.05, PHONE.w + PHONE.thick / 2 + 0.004], 0.012, 0.0095));
  const th = sp.scale === 100 ? PHONE.thick : 0.0095;
  out.push(box(C("phone", "hand-r", "phone-glass", TONE.phone), [PHONE.width, PHONE.length, th], [PHONE.u, PHONE.v, PHONE.w + (th - PHONE.thick) / 2], undefined, 0.0035));
  return out;
}

/** 🌡️ One surface solid per region; the outlets at nostrils, mouth and crown, sized to the scale's smallest feature. */
function thermal(b: Json[], sp: Spec): Json[] {
  const f = sp.scale * 0.00015;
  const out: Json[] = [];
  for (const id of ["hoodie", "hood", "hood-roll", "hood-string-l", "hood-string-r"]) if (b.some((s) => s.id === id)) out.push(thOf(id, "torso-clothing", "hoodie"));
  for (const s of SIDES) {
    const w = b.find((q) => q.id === `wrist-${s}`)!;
    out.push(sp.scale === 100 ? { ...w, id: `th-wrist-${s}`, region: "arm-skin", sections: w.sections.map((q: Json, i: number) => ({ ...q, at: [i === 0 ? 0.234 : 0.256, q.at[1], q.at[2]] })), capStart: false, capEnd: false } : thOf(`wrist-${s}`, "arm-skin"));
    out.push(thOf(`upper-arm-${s}`, "arm-clothing", "sleeve"), thOf(`forearm-${s}`, "arm-clothing", "sleeve"), thOf(`thigh-${s}`, "leg-clothing", "denim"), thOf(`shank-${s}`, "leg-clothing", "denim"), thOf(`shoe-${s}`, "shoes", "shoe-upper"), thOf(`shoe-toe-${s}`, "shoes", "shoe-upper"));
  }
  out.push(thOf("jeans-seat", "leg-clothing", "denim"));
  for (const s of b.filter((q) => /^hand-[lr]/.test(q.id))) out.push(thOf(s.id, "hand-skin"));
  const head = b.find((s) => s.id === "head")!;
  out.push({ ...head, id: "th-face", region: "head-skin", sections: head.sections.filter((q: Json) => q.at[0] <= 0.17), capEnd: false });
  for (const id of ["nose", "ear-l", "ear-r"]) if (b.some((s) => s.id === id)) out.push(thOf(id, "head-skin"));
  out.push(thOf("hair", "hair", "hair-under"), thOf("beanie", "hair", "beanie"), thOf("phone", "phone-surface"));
  out.push({ ...box(C("th-wall", "wall", "masonry", TONE.wall, "ground"), [0.38, 1.9, 1.2], [-0.19, 0.95, 0]), region: "wall-face", layer: "brick", sectionFaces: ["+v", "-v", "-u"], texture: true });
  const t = Math.max(f, 0.0076) / 2 + 0.0002;
  out.push({ ...ellipsoid(C("th-nose-breath", "head", "skin", TONE.skin), [t, Math.max(t, 0.0048), Math.max(t, 0.0075)], pa("head", 0.092, 0.104, 0)), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", "skin", TONE.lips), [t, t, Math.max(t, 0.0105)], pa("head", 0.068, 0.1, 0)), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-plume", "head", "wool-hat", TONE.beanie), [t, 0.05, 0.045], pa("head", 0.26, -0.02, 0)), region: "torso-clothing", tags: ["emitter", "plume"] });
  return out;
}

const BONE_M = "bone";
const bar = (id: string, bone: string, pts: V3[], r: number[] | number, member = bone): Json => ({ ...tube(C(id, bone, BONE_M, TONE.bone), pts, Array.isArray(r) ? r : pts.map(() => r)), member, part: "member" });
const el = (id: string, bone: string, radii: V3, at: V3, part: "member" | "joint" | "mass" = "member", member = bone, rot?: [number, number, number, number]): Json => ({ ...ellipsoid(C(id, bone, BONE_M, part === "member" ? TONE.bone : part === "joint" ? TONE.joint : TONE.mass), radii, at, rot), member, part });

/** 🦴 The skeleton of a coarse scale: bones merged by scale, joints at the rig joints, masses at the centres of mass. */
function structure(sp: Spec): Json[] {
  const fine = sp.scale === 100;
  const r = (x: number): number => Math.max(x, sp.scale * 0.00015 * 0.5);
  const out: Json[] = [];
  const spine: V3[] = [[-0.035, -0.07, 0], [0.05, -0.03, 0], [0.15, -0.04, 0], [0.3, -0.068, 0], [0.45, -0.048, 0]].map(([a, f]) => pa("torso", a, f, 0));
  out.push(bar("spine", "torso", spine, sp.scale === 500 ? 0.038 : [0.024, 0.022, 0.02, 0.017, 0.015].map(r)));
  const ribs: [number, number, number][] = fine ? [[0.42, 0.075, 0.07], [0.37, 0.11, 0.1], [0.32, 0.124, 0.115], [0.27, 0.131, 0.12], [0.22, 0.13, 0.11], [0.17, 0.115, 0.06]] : sp.scale === 200 ? [[0.38, 0.105, 0.09], [0.3, 0.125, 0.115], [0.22, 0.125, 0.1]] : [[0.32, 0.105, 0.11]];
  ribs.forEach(([a, w, drop], i) => {
    const pts: V3[] = [[a, -0.05, 0.022], [a, -0.09, w * 0.5], [a - drop * 0.4, -0.01, w], [a - drop * 0.8, 0.06, w * 0.6], [a - drop * 0.9, 0.083, 0.02]].map(([aa, f, l]) => pa("torso", aa, f, l));
    const rib = bar(`rib-${i + 1}-l`, "torso", pts, r(fine ? 0.0075 : 0.008));
    out.push(rib, mirrored(rib, toRight));
  });
  if (sp.scale !== 500) {
    out.push({ ...loft(C("sternum", "torso", BONE_M, TONE.bone), [sa("torso", 0.415, 0.084, 0, 0.022, r(0.009), 2.4), sa("torso", 0.25, 0.095, 0, r(0.014), r(0.008), 2.2)]), member: "torso", part: "member" });
    const girdle = bar("shoulder-girdle-l", "torso", [[0.412, 0.075, 0.02], [0.44, 0.04, 0.13], [0.447, -0.01, 0.18], [0.42, -0.095, 0.11], [0.33, -0.1, 0.09]].map(([a, f, l]) => pa("torso", a, f, l)), r(fine ? 0.008 : 0.01));
    out.push(girdle, mirrored(girdle, toRight));
  }
  const ilium = { ...loft(C("hip-bone-l", "pelvis", BONE_M, TONE.bone), [sa("pelvis", 0.135, 0.0, 0.098, 0.07, r(0.008), 2.2), sa("pelvis", 0.06, 0.004, 0.093, 0.045, r(0.013), 2.2), sa("pelvis", -0.03, 0.0, 0.07, 0.03, r(0.02), 2)].map((q, i) => ({ ...q, yaw: [50, 40, 0][i] }))), member: "pelvis", part: "member" };
  out.push(ilium, mirrored(ilium, toRight));
  out.push({ ...loft(C("sacrum", "pelvis", BONE_M, TONE.bone), [sa("pelvis", 0.12, -0.055, 0, 0.048, r(0.02), 2.2, -25), sa("pelvis", 0.01, -0.09, 0, r(0.016), r(0.01), 2, -45)]), member: "pelvis", part: "member" });
  out.push(el("cranium", "head", [0.074, 0.086, 0.064], pa("head", 0.162, -0.003, 0)));
  if (sp.scale !== 500) out.push({ ...loft(C("face-skeleton", "head", BONE_M, TONE.bone), [sa("head", 0.14, 0.062, 0, 0.058, 0.03, 2.2), sa("head", 0.05, 0.06, 0, 0.042, r(0.025), 2)]), member: "head", part: "member" });
  out.push(bar("cervical-spine", "head", [pa("head", -0.04, -0.04, 0), pa("head", 0.05, -0.016, 0), pa("head", 0.11, -0.01, 0)], r(0.012)));
  const legs: Json[] = [
    bar("femur-l", "thigh-l", [pa("thigh-l", 0.0, 0, 0), pa("thigh-l", 0.03, -0.006, 0.055), pa("thigh-l", 0.15, 0.006, 0.042), pa("thigh-l", 0.39, -0.004, 0)], [0.022, 0.017, 0.0135, 0.025].map(r)),
    bar("tibia-l", "shank-l", [pa("shank-l", 0.0, 0, 0), pa("shank-l", 0.2, 0.008, -0.004), pa("shank-l", 0.395, 0.0, -0.004)], [0.03, 0.015, 0.018].map(r)),
    bar("foot-bones-l", "foot-l", [[-0.05, -0.052, 0.004], [0.0, -0.012, 0], [0.06, -0.03, 0], [0.124, -0.062, 0]], [0.019, 0.02, 0.022, 0.016].map(r)),
    bar("humerus-l", "upper-arm-l", [pa("upper-arm-l", 0, 0, -0.004), pa("upper-arm-l", 0.15, 0.002, 0.004), pa("upper-arm-l", 0.3, -0.004, 0)], [0.022, 0.012, 0.022].map(r)),
    bar("forearm-bones-l", "forearm-l", [pa("forearm-l", -0.015, -0.01, -0.005), pa("forearm-l", 0.12, 0.003, 0), pa("forearm-l", 0.245, 0.002, 0)], [0.013, 0.012, 0.016].map(r)),
  ];
  if (fine) legs.push(bar("toe-bones-l", "toe-l", [[0.0, 0.03, -0.004], [0.05, 0.026, -0.008]], 0.012), bar("fibula-l", "shank-l", [pa("shank-l", 0.04, -0.012, 0.033), pa("shank-l", 0.405, -0.008, 0.028)], 0.0075), el("patella-l", "shank-l", [0.022, 0.011, 0.02], pa("shank-l", -0.012, 0.044, 0)));
  for (const q of legs) out.push(q, mirrored(q, toRight));
  out.push({ ...loft(C("hand-bones-l", "hand-l", BONE_M, TONE.bone), [sa("hand-l", 0.0, 0, -0.001, r(0.009), 0.024, 2.2), sa("hand-l", 0.09, 0, -0.003, r(0.007), 0.034, 2.4), sa("hand-l", 0.15, 0, -0.016, r(0.006), 0.028, 2)]), member: "hand-l", part: "member" });
  out.push({ ...loft(C("hand-bones-r", "hand-r", BONE_M, TONE.bone), [sa("hand-r", 0.0, 0, 0.001, r(0.009), 0.024, 2.2), sa("hand-r", 0.09, 0, 0.004, r(0.007), 0.034, 2.4), sa("hand-r", PHONE.u + PHONE.width / 2 - 0.006, 0, 0.012, r(0.006), 0.03, 2)]), member: "hand-r", part: "member" });
  const jr = sp.scale === 500 ? 0.04 : sp.scale === 200 ? 0.02 : 0.016;
  const J = (x: number): number => Math.max(x, sp.scale * 0.00015 * 0.5 + 0.0005);
  out.push(el("joint-lumbosacral", "torso", [J(jr * 0.8), J(jr * 1.4), J(jr * 1.4)], pa("torso", 0, -0.042, 0), "joint"));
  out.push(el("joint-neck", "head", [J(jr * 0.7), J(jr), J(jr)], pa("head", 0, -0.03, 0), "joint"));
  for (const s of SIDES) {
    out.push(el(`joint-hip-${s}`, `thigh-${s}`, [jr * 1.4, jr * 1.4, jr * 1.4], [0, 0, 0], "joint"));
    out.push(el(`joint-knee-${s}`, `shank-${s}`, [jr, jr * 1.4, jr * 1.8], [0, 0, 0], "joint"));
    out.push(el(`joint-shoulder-${s}`, `upper-arm-${s}`, [jr * 1.3, jr * 1.3, jr * 1.3], [0, 0, 0], "joint"));
    out.push(el(`joint-elbow-${s}`, `forearm-${s}`, [jr, jr, jr * 1.4], [0, 0, 0], "joint"));
    if (sp.scale !== 500) {
      out.push(el(`joint-ankle-${s}`, `foot-${s}`, [jr, jr, jr * 1.3], [0, 0, 0], "joint"));
      out.push(el(`joint-wrist-${s}`, `hand-${s}`, [J(jr * 0.7), J(jr), J(jr * 0.7)], [0, 0, 0], "joint"));
    }
  }
  for (const b of ASSET.rig.bones as Json[]) if (b.mass && b.mass.kg >= (sp.scale === 500 ? 9 : 4)) out.push(el(`mass-${b.id}`, b.id, [r(0.022), r(0.022), r(0.022)].map((x) => Math.max(x, sp.scale === 500 ? 0.04 : x)) as V3, [b.mass.comX, b.mass.comY ?? 0, 0], "mass"));
  for (const member of ["wall", "wall-x", "wall-frame"]) out.push({ id: `member-${member}`, of: "wall", member, part: "member" });
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
void extrude;
