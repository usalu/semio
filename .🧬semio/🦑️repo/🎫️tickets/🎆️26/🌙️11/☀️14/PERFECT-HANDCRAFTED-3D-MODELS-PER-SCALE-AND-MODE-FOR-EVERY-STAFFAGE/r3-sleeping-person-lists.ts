/** 🌡️ Thermal model and structure of the round 3 sleeping person at 1:50: every region from the body solids that carry it (the duvet mounds as the outer layer of `top` and `bottom` with the pyjama inside them, the face as its own skin mask in front of the scalp, the flat duvet, its rolled edges, drapes and fold as `duvet-edge`), the three emitters as outlet patches on the nostrils, the lips and the duvet over the chest; and the skeleton that carries her weight to the mattress: skull and mandible, cervical, thoracic and lumbar spine with their curves, rib hoops, sternum, clavicles, scapulae, pelvic ring with ilia and sacrum, femurs with necks, patellae, tibiae and fibulae, foot skeletons and toe rays, humeri, radii and ulnae, metacarpal plates and finger rays, joints at the rig joints, masses at the de Leva centres, the contact band of the mattress top, the pillow core under the head and the duvet's membrane along its crest.
 * @see ./r3-sleeping-person-s50.ts */
import { BED } from "./r3-sleeping-person-ref.ts";
import { FACE, T, Y0 } from "./r3-sleeping-person-s50.ts";
import { Kit, POSE, W, type Json, type Row } from "./r3-sleeping-person-kit.ts";

/** 🌬️ The emitter outlets of a scale: nostrils and lips on the face, a patch of the duvet over the chest; `r` is the patch radius floor of the scale. */
export function emitters(k: Kit, floor: number, plumeTop: number): Json[] {
  const e = new Kit();
  const r = Math.max(0.0045, floor / 2);
  e.ell("th-nose-breath", "head", "skin", T.skin, [-0.02, -0.096, 0], [r, r, r * 1.6], "face-skin");
  e.ell("th-mouth-breath", "head", "skin", T.lip, [-0.044, -0.091, 0], [r, r, Math.max(r, 0.012)], "face-skin");
  e.well("th-body-plume", "torso", "duvet-cover", T.duvet, [0.7, plumeTop, 0.028], [0.15, Math.max(0.006, floor / 2), 0.15], "top");
  const tags: Record<string, string> = { "th-nose-breath": "nose-breath", "th-mouth-breath": "mouth-breath", "th-body-plume": "body-plume" };
  void k;
  return e.solids.map((s) => ({ ...s, region: e.region.get(s.id), tags: ["emitter", tags[s.id]!] }));
}

/** 🌡️ The thermal list of a scale from the region of every body solid; at 1:50 the clothing regions get their two layers (pyjama inside, duvet outside) and the face its own mask. */
export function thermal(asset: Json, k: Kit, scale: number): Json[] {
  const out: Json[] = [];
  const regions = new Map((asset.thermal.regions as Json[]).map((r) => [r.id as string, r]));
  const duvet = (id: string): boolean => /^(duvet-|cloth-|fold-(thighs|shins|legs)|mound)/.test(id);
  for (const s of k.solids) {
    const region = k.region.get(s.id);
    if (region === undefined || s.id === "head") continue;
    const layered = scale === 50 && (regions.get(region)!.layers as Json[]).length > 1;
    const layer = !layered ? undefined : duvet(s.id) ? "duvet" : region === "top" ? "pyjama-top" : "pyjama-shorts";
    out.push({ id: `th-${s.id}`, of: s.id, region, ...(layer === undefined ? {} : { layer }) });
  }
  if (scale <= 100) {
    const own = new Kit();
    own.loft("th-face", "head", "skin", T.skin, scale === 50 ? FACE : FACE.filter((_, i) => i % 2 === 0), "face-skin");
    if (scale === 100) {
      own.loft("th-hand-l", "hand-l", "skin", T.skin, [[-0.03, 0, 0, 0.026, 0.018], [0.04, -0.001, 0, 0.038, 0.015], [0.1, -0.004, 0, 0.039, 0.012], [0.14, -0.012, 0.002, 0.036, 0.01], [0.17, -0.026, 0.002, 0.03, 0.009]], "hand-skin");
      own.cap("th-thumb-l", "hand-l", "skin", T.skin, [0.012, -0.009, -0.026], [0.07, -0.02, -0.054], 0.012, 0.0085, "hand-skin");
    }
    out.push(...own.solids.map((x) => ({ ...x, region: own.region.get(x.id) })));
  } else out.push({ id: "th-head", of: "head", region: "face-skin" });
  const top = scale === 500 ? 0.693 : 0.704;
  out.push(...emitters(k, { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 }[scale]!, top));
  return out;
}

const BONE = "bone";
/** 🦴️ Structural parts of a list under construction: members, joints and masses with their member bone. */
class Skeleton extends Kit {
  readonly part = new Map<string, [string, "member" | "joint" | "mass" | "support"]>();
  mark(member: string, part: "member" | "joint" | "mass"): void {
    for (const s of this.solids) if (!this.part.has(s.id)) this.part.set(s.id, [member, part]);
  }
  list(): Json[] {
    return this.solids.map((s) => ({ ...s, member: this.part.get(s.id)![0], part: this.part.get(s.id)![1] }));
  }
}

const JOINTS: Record<string, number> = { torso: 0.02, neck: 0.016, head: 0.016, "arm-l": 0.024, "arm-r": 0.024, "forearm-l": 0.018, "forearm-r": 0.018, "hand-l": 0.014, "hand-r": 0.014, "thigh-l": 0.025, "thigh-r": 0.025, "shank-l": 0.025, "shank-r": 0.025, "foot-l": 0.018, "foot-r": 0.018, "toe-l": 0.012, "toe-r": 0.012 };

/** 🦴️ Joints at the rig joints and masses at the centres of mass of every bone of at least `minKg`, sized by the scale. */
function jointsAndMasses(k: Skeleton, asset: Json, floor: number, minKg: number, joints = true): void {
  if (joints) for (const [bone, r] of Object.entries(JOINTS)) {
    const q = Math.max(r, floor / 2);
    k.ell(`joint-${bone}`, bone, BONE, 0.85, [0, 0, 0], [q, q, q]);
    k.mark(bone, "joint");
  }
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < minKg) continue;
    const r = Math.max(floor / 2, 0.012 * Math.cbrt(b.mass.kg));
    k.ell(`mass-${b.id}`, b.id, BONE, 0.3, [b.mass.comX, b.mass.comY, 0], [r, r, r]);
    k.mark(b.id, "mass");
  }
}

/** 🛏️ The bedding members of a scale: the contact band of the mattress top under the body, the pillow core under the head, the duvet's membrane along its crest. */
function bedding(k: Skeleton, t: number, crest: readonly (readonly [number, number])[]): void {
  k.box("member-bed-surface", "bed-surface", "duvet-fill", 0.6, [0.96, t / 2, 0], [1.68, t, 0.14], t / 2.5);
  k.mark("bed-surface", "member");
  k.box("member-pillow", "pillow", "duvet-fill", 0.6, [0.22, 0.028, 0], [0.16, Math.max(t, 0.03), 0.16], Math.max(t, 0.03) / 2.5);
  k.mark("pillow", "member");
  k.wloft("member-duvet", "duvet", "duvet-fill", 0.6, crest.map(([x, y]) => [x, y, 0, 0.06, t / 2, 4] as const), undefined);
  k.mark("duvet", "member");
}

const CREST = [[0.62, 0.64], [0.72, 0.687], [0.9, 0.658], [1.1, 0.647], [1.3, 0.67], [1.5, 0.624], [1.7, 0.613], [1.78, 0.6]] as const;

/** 🦴️ The 1:50 skeleton. */
export function structure50(asset: Json): Json[] {
  const k = new Skeleton();
  k.ell("skull", "head", BONE, 0.85, [0.04, 0.004, 0], [0.088, 0.083, 0.066]);
  k.bar("mandible", "head", BONE, 0.85, [[-0.035, 0.012, 0.047, 0.009], [-0.07, -0.025, 0.04, 0.009], [-0.087, -0.055, 0.0, 0.01], [-0.07, -0.025, -0.04, 0.009], [-0.035, 0.012, -0.047, 0.009]]);
  k.mark("head", "member");
  k.bar("cervical-spine", "neck", BONE, 0.85, [[-0.01, 0.015, 0, 0.012], [0.05, 0.02, 0, 0.011], [0.105, 0.012, 0, 0.012]]);
  k.mark("neck", "member");
  k.bar("thoracolumbar-spine", "torso", BONE, 0.85, [[-0.02, 0.035, 0, 0.018], [0.1, 0.04, 0, 0.017], [0.2, 0.055, 0, 0.015], [0.3, 0.064, 0, 0.013], [0.4, 0.06, 0, 0.012], [0.485, 0.038, 0, 0.011]]);
  ([[0.21, 0.08, 0.125], [0.25, 0.088, 0.135], [0.29, 0.092, 0.14], [0.33, 0.092, 0.14], [0.37, 0.088, 0.135], [0.41, 0.075, 0.115]] as const).forEach(([s, d, w], i) => k.ring(`rib-${i + 1}`, "torso", BONE, 0.85, s, 0.0, d, w, 0.0045, -20));
  k.bar("sternum", "torso", BONE, 0.85, [[0.26, -0.098, 0, 0.012, 0.005], [0.43, -0.077, 0, 0.013, 0.005]]);
  for (const s of [1, -1]) {
    k.bar(`clavicle-${s > 0 ? "l" : "r"}`, "torso", BONE, 0.85, [[0.44, -0.066, 0.022 * s, 0.006], [0.435, -0.058, 0.09 * s, 0.006], [0.418, -0.02, 0.15 * s, 0.007]]);
    k.ell(`scapula-${s > 0 ? "l" : "r"}`, "torso", BONE, 0.85, [0.37, 0.072, 0.095 * s], [0.07, 0.006, 0.05]);
  }
  k.mark("torso", "member");
  k.ring("pelvic-ring", "pelvis", BONE, 0.85, 0.03, 0.005, 0.07, 0.12, 0.011, 15);
  for (const s of [1, -1]) k.ell(`ilium-${s > 0 ? "l" : "r"}`, "pelvis", BONE, 0.85, [0.075, 0.025, 0.105 * s], [0.06, 0.01, 0.045]);
  k.ell("sacrum", "pelvis", BONE, 0.85, [0.06, 0.065, 0], [0.055, 0.015, 0.045]);
  k.mark("pelvis", "member");
  for (const side of ["l", "r"] as const) {
    const lat = side === "l" ? 1 : -1;
    const thigh = `thigh-${side}`;
    k.bar(`femur-neck-${side}`, thigh, BONE, 0.85, [[0, 0, 0, 0.014], [0.035, 0.0, 0.045 * lat, 0.015]]);
    k.bar(`femur-${side}`, thigh, BONE, 0.85, [[0.03, 0.0, 0.045 * lat, 0.016], [0.2, 0.006, 0.015 * lat, 0.0135], [0.37, 0.002, 0.0, 0.016], [0.395, 0, 0, 0.022, 0.018]]);
    k.ell(`patella-${side}`, thigh, BONE, 0.85, [0.39, 0.042, 0], [0.022, 0.01, 0.02]);
    k.mark(thigh, "member");
    const shank = `shank-${side}`;
    k.bar(`tibia-${side}`, shank, BONE, 0.85, [[0.015, 0.0, 0, 0.022, 0.018], [0.06, 0.006, 0, 0.015], [0.3, 0.004, 0, 0.011], [0.405, 0, -0.004 * lat, 0.014]]);
    k.bar(`fibula-${side}`, shank, BONE, 0.85, [[0.03, -0.012, 0.026 * lat, 0.0065], [0.25, -0.01, 0.024 * lat, 0.005], [0.41, -0.006, 0.027 * lat, 0.0075]]);
    k.mark(shank, "member");
    const foot = `foot-${side}`;
    k.loft(`foot-bones-${side}`, foot, BONE, 0.85, ([[-0.045, -0.03, 0, 0.018, 0.022], [-0.015, -0.022, 0, 0.022, 0.03], [0.03, -0.012, 0, 0.026, 0.026], [0.09, -0.006, 0, 0.032, 0.016], [0.145, -0.002, 0, 0.034, 0.008]] as Row[]));
    k.mark(foot, "member");
    const toe = `toe-${side}`;
    ([0.026, 0.005, -0.011, -0.025, -0.037] as const).forEach((cw, i) => k.cap(`toe-ray-${i + 1}-${side}`, toe, BONE, 0.85, [0, -0.004, cw * -lat], [[0.045, 0.036, 0.031, 0.026, 0.021][i]!, -0.01, cw * -lat * 0.97], 0.0042, 0.0038));
    k.mark(toe, "member");
    const arm = `arm-${side}`;
    k.bar(`humerus-${side}`, arm, BONE, 0.85, [[0.02, 0, 0, 0.013], [0.16, 0, 0, 0.0105], [0.29, 0, 0, 0.013], [0.31, 0, 0, 0.02, 0.012]]);
    k.mark(arm, "member");
    const fore = `forearm-${side}`;
    const t = side === "r" ? 1 : -1;
    k.bar(`ulna-${side}`, fore, BONE, 0.85, [[-0.01, 0, -0.012 * t, 0.009], [0.12, 0, -0.012 * t, 0.0065], [0.24, 0, -0.014 * t, 0.006]]);
    k.bar(`radius-${side}`, fore, BONE, 0.85, [[0.015, 0, 0.012 * t, 0.0065], [0.12, 0, 0.013 * t, 0.007], [0.24, 0, 0.016 * t, 0.0095]]);
    k.mark(fore, "member");
    const hand = `hand-${side}`;
    k.ell(`metacarpals-${side}`, hand, BONE, 0.85, [0.05, -0.001, 0], [0.05, 0.0045, 0.033]);
    const curl = side === "r" ? [18, 40] : [12, 30];
    const rad = (d: number): number => (d * Math.PI) / 180;
    ([[0.024, 0.042, 0.038], [0.006, 0.047, 0.042], [-0.012, 0.044, 0.04], [-0.028, 0.034, 0.03]] as const).forEach(([cw, l1, l2], i) => {
      const p0 = [0.095, -0.004, cw * t];
      const p1 = [p0[0]! + l1 * Math.cos(rad(curl[0]!)), p0[1]! - l1 * Math.sin(rad(curl[0]!)), p0[2]!];
      const p2 = [p1[0]! + l2 * Math.cos(rad(curl[0]! + curl[1]!)), p1[1]! - l2 * Math.sin(rad(curl[0]! + curl[1]!)), p1[2]!];
      k.bar(`finger-ray-${i + 1}-${side}`, hand, BONE, 0.85, [[...p0, 0.0042], [...p1, 0.0039], [...p2, 0.0038]]);
    });
    k.bar(`thumb-ray-${side}`, hand, BONE, 0.85, [[0.012, -0.009, 0.026 * t, 0.0045], [0.046, -0.016, 0.047 * t, 0.0042], [0.072, -0.02, 0.055 * t, 0.0038]]);
    k.mark(hand, "member");
  }
  bedding(k, 0.008, CREST);
  jointsAndMasses(k, asset, 0.0075, 1);
  return k.list();
}

/** 🦴️ A simplified skeleton for a coarser scale: spine as one bent bar, a rib-cage loft (hoops at 1:100), pelvis, long bones as bars, hands and feet as plates, all at least one smallest feature thick. */
export function structureCoarse(asset: Json, scale: 100 | 200 | 500): Json[] {
  const k = new Skeleton();
  const f = { 100: 0.015, 200: 0.03, 500: 0.075 }[scale];
  const r = (x: number): number => Math.max(x, f / 2);
  k.ell("skull", "head", BONE, 0.85, [0.03, 0.0, 0], [r(0.088), r(0.083), r(0.066)]);
  k.mark("head", "member");
  k.bar("cervical-spine", "neck", BONE, 0.85, [[-0.01, 0.015, 0, r(0.012)], [0.105, 0.012, 0, r(0.012)]]);
  k.mark("neck", "member");
  k.bar("spine", "torso", BONE, 0.85, scale === 500 ? [[0.0, 0.03, 0, r(0.02)], [0.48, 0.03, 0, r(0.02)]] : [[-0.02, 0.035, 0, r(0.018)], [0.2, 0.055, 0, r(0.015)], [0.4, 0.06, 0, r(0.012)], [0.485, 0.038, 0, r(0.011)]]);
  if (scale === 100) ([[0.23, 0.085, 0.13], [0.3, 0.092, 0.14], [0.37, 0.088, 0.135]] as const).forEach(([s, d, w], i) => k.ring(`rib-${i + 1}`, "torso", BONE, 0.85, s, 0, d, w, 0.0075, -20));
  else k.loft("rib-cage", "torso", BONE, 0.85, [[0.2, 0.0, 0, 0.12, 0.08], [0.3, -0.005, 0, 0.14, 0.092], [0.42, 0.0, 0, 0.11, 0.075]] as Row[]);
  if (scale !== 500) for (const s of [1, -1]) k.bar(`clavicle-${s > 0 ? "l" : "r"}`, "torso", BONE, 0.85, [[0.44, -0.06, 0.022 * s, r(0.006)], [0.418, -0.02, 0.15 * s, r(0.007)]]);
  k.mark("torso", "member");
  k.ell("pelvic-girdle", "pelvis", BONE, 0.85, [0.04, 0.02, 0], [r(0.07), r(0.06), 0.13]);
  k.mark("pelvis", "member");
  for (const side of ["l", "r"] as const) {
    const lat = side === "l" ? 1 : -1;
    k.bar(`femur-${side}`, `thigh-${side}`, BONE, 0.85, [[0, 0, 0, r(0.015)], [0.035, 0, 0.04 * lat, r(0.016)], [0.39, 0, 0, r(0.02)]]);
    k.mark(`thigh-${side}`, "member");
    k.bar(`tibia-${side}`, `shank-${side}`, BONE, 0.85, [[0.01, 0, 0, r(0.02)], [0.41, 0, 0, r(0.013)]]);
    if (scale === 100) k.bar(`fibula-${side}`, `shank-${side}`, BONE, 0.85, [[0.03, -0.012, 0.026 * lat, 0.0075], [0.41, -0.006, 0.027 * lat, 0.0075]]);
    k.mark(`shank-${side}`, "member");
    k.loft(`foot-bones-${side}`, `foot-${side}`, BONE, 0.85, [[-0.04, -0.028, 0, r(0.018), r(0.02)], [0.03, -0.012, 0, r(0.026), r(0.024)], [0.145, -0.002, 0, r(0.033), r(0.008)]] as Row[]);
    k.mark(`foot-${side}`, "member");
    k.cap(`toe-rays-${side}`, `toe-${side}`, BONE, 0.85, [0, -0.005, 0], [scale === 500 ? 0.01 : 0.032, -0.01, 0], r(0.012), r(0.01));
    k.mark(`toe-${side}`, "member");
    k.bar(`humerus-${side}`, `arm-${side}`, BONE, 0.85, [[0.01, 0, 0, r(0.013)], [0.305, 0, 0, r(0.014)]]);
    k.mark(`arm-${side}`, "member");
    k.bar(`forearm-bones-${side}`, `forearm-${side}`, BONE, 0.85, [[0.0, 0, 0, r(0.012), r(0.009)], [0.24, 0, 0, r(0.017), r(0.009)]]);
    k.mark(`forearm-${side}`, "member");
    k.ell(`hand-bones-${side}`, `hand-${side}`, BONE, 0.85, [0.07, -0.004, 0], [scale === 500 ? 0.075 : 0.07, r(0.006), r(0.028)]);
    k.mark(`hand-${side}`, "member");
  }
  bedding(k, f, CREST.map(([x, y]) => [x, y - (scale === 500 ? 0.025 : scale === 200 ? 0.008 : 0)] as const));
  jointsAndMasses(k, asset, f, scale === 500 ? 5 : 1, scale !== 500);
  return k.list();
}

export { POSE, W, BED };
