/** 📋️ The lists of the round 3 wheelchair user per scale: which body is built, its thermal model (nested layer shells at 1:50, the bare face and neck below the beanie, breath and plume as outlet patches) and its structure (the skeleton of the sitter, the load path of the chair from the rear axles and castor axles through hubs, spokes, rims, forks, housings and the tube frame to the upholstery, masses at the de Leva centres).
 * @see ./r3-wheelchair-user-s50.ts
 * @see ./r3-wheelchair-user-coarse.ts */
import { Kit, aboutU, aboutW, parts, quat, thermalOf, type Json, type V3 } from "./r3-wheelchair-user-kit.ts";
import { HEAD100, body100, body200, body500 } from "./r3-wheelchair-user-coarse.ts";
import { DECISIONS_TEXT } from "./r3-wheelchair-user-texts.ts";
import { BEANIE, FINGERS, GRIP_HAND, HEAD, NECK, body50, fingerPath } from "./r3-wheelchair-user-s50.ts";
import { CHAIR } from "./r3-wheelchair-user-ref.ts";
import { FOOT_DROP } from "./r3-wheelchair-user-roots.ts";

export const BODY: Record<number, (k: Kit) => void> = { 50: body50, 100: body100, 200: body200, 500: body500 };
export const DECISIONS: Record<number, Json[]> = DECISIONS_TEXT;
export const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const MATERIALS: Json[] = [{ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" }];
/** 📐️ Total outer areas of passive regions corrected at the root from the honest 1:50 model (C16: within a factor 1.5 of the old value); filled in after measuring. */
export const AREAS: Record<string, number> = { cushion: 0.89 };
/** 🧾️ Root data that follow the model: the materials the solids name and the corrected passive areas. */
export function finish(asset: Json): void {
  const have = new Set((asset.materials as Json[]).map((m) => m.id));
  asset.materials.push(...MATERIALS.filter((m) => !have.has(m.id)));
  for (const region of asset.thermal.regions as Json[]) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];
}

/* ---------------------------------------------------------------- thermal */

const OUTER_ONLY = /^(hood|hood-collar|hoodie-hem|sleeve-cuff-[lr]|thigh-[lr]|shank-[lr]|midsole-[lr]|toe-sole-[lr])$/;
const NO_SURFACE = /^(eye|mouth|brow)-?/;
const FACE_PARTS = /^(head|neck|nose|chin-pad|earlobe-[lr]|beanie|beanie-cuff)$/;
/** 🌡️ The thermal list of a scale: every body solid that is a surface of its region (texture and slender parts are no surfaces of their own); at 1:50 garments and shoes as nested layers (T-shirt and fleece under the hoodie on trunk and sleeves, briefs only under the seat of the jeans, socks only inside the uppers); the face skin as the head and neck below the beanie and above the collar; the emitters as patches on the surface they leave. */
export function thermal(asset: Json, kit: Kit, scale: number, face: ((k: Kit) => void) | undefined, extra?: (k: Kit) => void): Json[] {
  const regions = new Map((asset.thermal.regions as Json[]).map((r) => [r.id, r]));
  const out: Json[] = [];
  for (const s of kit.solids) {
    const region = kit.region.get(s.id);
    if (region === undefined) throw new Error(`no region for ${s.id}`);
    if (s.texture === true || s.slender === true || NO_SURFACE.test(s.id) || (face !== undefined && FACE_PARTS.test(s.id))) continue;
    const r = regions.get(region)!;
    const layers: Json[] = r.layers ?? [];
    if (scale === 50 && layers.length > 1 && OUTER_ONLY.test(s.id)) out.push({ id: `th-${s.id}`, of: s.id, region, layer: layers[layers.length - 1]!.id });
    else if (scale === 50 && region === "jeans" && s.id !== "jeans-seat") out.push({ id: `th-${s.id}`, of: s.id, region, layer: "jeans" });
    else if (scale === 50 && region === "shoes" && !/^(sneaker|sneaker-toe)-/.test(s.id)) out.push({ id: `th-${s.id}`, of: s.id, region, layer: "sneaker" });
    else out.push(...thermalOf(s, r, scale === 50).map((t) => (s.id === "jeans-seat" && t.of !== undefined ? { ...t, sectionFaces: ["+u"] } : t)));
  }
  const k = new Kit();
  face?.(k);
  extra?.(k);
  out.push(...k.solids.map((s) => ({ ...s, region: s.id.startsWith("th-glove") ? "gloves" : s.id.startsWith("th-fingers") ? "hand-skin" : s.id.startsWith("th-beanie") ? "beanie" : "face-skin", ...(s.id.startsWith("th-beanie") ? { layer: "beanie" } : {}) })));
  const f = FEATURE[scale]!;
  const e = new Kit();
  const patch = Math.max(f * 0.51, 0.004);
  e.ell("th-nose-breath", "head", "skin", 0.5, scale >= 200 ? [-0.034, -0.1, 0] : [-0.036, -0.112, 0], [patch, patch, Math.max(0.0075, patch)]);
  e.ell("th-mouth-breath", "head", "skin", 0.5, scale >= 200 ? [-0.062, -0.094, 0] : [-0.061, -0.103, 0], [patch, patch, Math.max(0.011, patch)]);
  e.ell("th-body-plume", "head", "knit-wool", 0.4, [scale >= 500 ? 0.13 : 0.14, 0.005, 0], [Math.max(0.004, f * 0.5), Math.max(0.03, f * 0.6), Math.max(0.025, f * 0.6)]);
  const emitter: Record<string, [string, string]> = { "th-nose-breath": ["face-skin", "nose-breath"], "th-mouth-breath": ["face-skin", "mouth-breath"], "th-body-plume": ["hoodie", "body-plume"] };
  out.push(...e.solids.map((s) => ({ ...s, region: emitter[s.id]![0], tags: ["emitter", emitter[s.id]![1]] })));
  return out;
}

/** 🙂️ The face skin at 1:50 and 1:100: the head from the chin to the brim of the beanie (open at the top, where the beanie takes over) and the neck from the collar of the hoodie to the jaw (open at both ends). */
export function face50(k: Kit): void {
  k.loft("th-beanie", "head", "knit-wool", 0.35, [[0.04, 0.002, 0, 0.088, 0.105, 2.2], ...BEANIE.filter((r) => r[0] >= 0.055)], undefined, { caps: [false, true] });
  k.loft("th-face", "head", "skin", 0.27, HEAD.filter((r) => r[0] <= 0.02).concat([[0.034, -0.005, 0, 0.076, 0.0995, 2.1]]), undefined, { caps: [true, false] });
  k.loft("th-neck", "neck", "skin", 0.29, NECK.slice(1, 3).concat([[0.1, -0.012, 0, 0.055, 0.059]]), undefined, { caps: [false, false] });
}

/** 🙂️ The face skin solid builder of every scale. */
export const FACE: Record<number, ((k: Kit) => void) | undefined> = { 50: face50, 100: face100, 200: undefined, 500: undefined };
/** ➕️ Thermal-only solids of a scale (parts the body of that scale merges into a neighbouring mass). */
export const EXTRA: Record<number, ((k: Kit) => void) | undefined> = { 50: undefined, 100: undefined, 200: undefined, 500: hands500 };

/** 🙂️ The face skin at 1:100: the head of that scale from the chin to the brim of the beanie and the neck between collar and jaw. */
export function face100(k: Kit): void {
  k.loft("th-beanie", "head", "knit-wool", 0.35, [[0.04, 0.002, 0, 0.088, 0.106, 2.2], [0.088, 0.0, 0, 0.078, 0.099, 2.1], [0.122, 0.005, 0, 0.054, 0.065, 2], [0.142, 0.005, 0, 0.012, 0.014, 2]], undefined, { caps: [false, true] });
  k.loft("th-face", "head", "skin", 0.27, HEAD100.filter((r) => r[0] <= 0.03), undefined, { caps: [true, false] });
  k.loft("th-neck", "neck", "skin", 0.29, [[0.02, -0.012, 0, 0.056, 0.058], [0.1, -0.012, 0, 0.055, 0.059]], undefined, { caps: [false, false] });
}

/** 🧤️ At 1:500 the gloved hands are merged into the forearm masses of the body; the thermal model keeps the glove and the bare finger tips as their own small solids on the hand bones, inside the forearm mass. */
export function hands500(k: Kit): void {
  for (const s of ["l", "r"]) {
    k.ell(`th-glove-${s}`, `hand-${s}`, "leather-glove", 0.15, [0.055, -0.006, 0], [0.06, 0.038, 0.048]);
    k.ell(`th-fingers-${s}`, `hand-${s}`, "skin", 0.27, [0.14, -0.02, 0], [0.04, 0.038, 0.04]);
  }
}

/* ---------------------------------------------------------------- structure */

const B = 0.62;
/** 🦴️ The structure of a scale: the skeleton stated bone by bone in the rest frames (radii never below half the smallest feature, parts merged as the scale coarsens), the chair from the hubs and castor wheels through spokes and rims, axle receivers, forks and housings into the tube frame, the footrest and the back posts; masses at the de Leva centres and at the centres of the chair parts. Body solids that are themselves the load-bearing parts (tubes, plates, housings, forks, footplate, rims, hubs, spokes, castor wheels) are referenced. */
export function structure(asset: Json, scale: number, body: readonly Json[]): Json[] {
  const f = FEATURE[scale]! / 2 + 0.0002;
  const r = (x: number): number => Math.max(x, f);
  const fine = scale === 50;
  const mid = scale <= 100;
  const out: Json[] = [];
  const member = (bone: string, build: (k: Kit) => void, part: "member" | "joint" | "mass" = "member"): void => {
    const k = new Kit();
    build(k);
    out.push(...parts(k.solids, bone, part));
  };
  member("pelvis", (k) => {
    k.bar("sacrum", "pelvis", "bone", B, [[0.098, 0.04, 0, r(0.026), r(0.02)], [0.05, 0.066, 0, r(0.026), r(0.015)], [0.0, 0.082, 0, r(0.014), r(0.011)], ...(fine ? [[-0.03, 0.084, 0, 0.008, 0.007]] : [])]);
    if (scale <= 200) for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      k.ell(`ilium-${s}`, "pelvis", "bone", B, [0.062, 0.03, side * 0.1], [mid ? 0.068 : 0.07, mid ? 0.055 : 0.05, r(fine ? 0.011 : 0.013)], undefined, { rotation: quat(aboutU(side * -34), aboutW(-10)) });
      if (mid) {
        k.bar(`pubis-${s}`, "pelvis", "bone", B, [[0.004, -0.02, side * 0.08, r(0.012)], [-0.02, -0.048, side * 0.04, r(0.01)], [-0.03, -0.058, side * 0.008, r(0.011)]]);
        k.bar(`ischium-${s}`, "pelvis", "bone", B, [[-0.008, 0.008, side * 0.082, r(0.014)], [-0.05, 0.022, side * 0.066, r(0.013)], [-0.082, 0.018, side * 0.056, r(0.015)], ...(fine ? [[-0.064, -0.028, side * 0.03, 0.009], [-0.038, -0.054, side * 0.01, 0.009]] : [])]);
      } else k.bar(`hip-bar-${s}`, "pelvis", "bone", B, [[0.0, 0.0, side * 0.085, r(0.02)], [-0.07, 0.02, side * 0.058, r(0.02)]]);
    }
    else k.bar("pelvic-bar", "pelvis", "bone", B, [[0.02, 0.02, -0.09, r(0.03)], [0.02, 0.02, 0.09, r(0.03)]]);
  });
  member("torso", (k) => {
    const spine = fine ? [[-0.004, 0.0, 0, 0.024], [0.06, -0.022, 0, 0.023], [0.14, -0.02, 0, 0.021], [0.23, 0.004, 0, 0.019], [0.32, 0.026, 0, 0.017], [0.41, 0.024, 0, 0.015], [0.47, 0.01, 0, 0.014], [0.495, 0.0, 0, 0.013]] : mid ? [[0, 0, 0, 0.024], [0.08, -0.022, 0, 0.022], [0.22, 0.002, 0, 0.019], [0.36, 0.026, 0, 0.017], [0.495, 0.0, 0, 0.014]] : [[0, 0, 0, r(0.024)], [0.2, 0.0, 0, r(0.02)], [0.36, 0.022, 0, r(0.018)], [0.495, 0, 0, r(0.016)]];
    k.bar("spine", "torso", "bone", B, spine);
    if (fine) ([[0.01, 0.02, 0.03], [0.04, 0.008, 0.031], [0.07, 0.001, 0.032], [0.1, 0.001, 0.032], [0.13, 0.001, 0.031], [0.16, 0.006, 0.03], [0.19, 0.013, 0.03], [0.22, 0.021, 0.03], [0.245, 0.026, 0.03], [0.27, 0.032, 0.03], [0.295, 0.037, 0.03], [0.32, 0.043, 0.029], [0.345, 0.042, 0.028], [0.37, 0.041, 0.026], [0.395, 0.04, 0.025], [0.42, 0.036, 0.024], [0.445, 0.03, 0.024]] as const).forEach(([s, cv, len], i) => k.cap(`spinous-${i + 1}`, "torso", "bone", B, [s, cv - 0.004, 0], [s - (i < 5 ? 0.006 : 0.02), cv + (i < 5 ? len : len * 0.72), 0], 0.0055, 0.004));
    const ribs: number[][] = fine ? [[0.445, -0.04, 0.046, 0.068], [0.42, -0.048, 0.066, 0.098], [0.39, -0.054, 0.08, 0.116], [0.355, -0.058, 0.09, 0.128], [0.32, -0.06, 0.096, 0.134], [0.285, -0.06, 0.098, 0.137], [0.25, -0.058, 0.097, 0.136], [0.218, -0.054, 0.092, 0.132]] : mid ? [[0.435, -0.046, 0.054, 0.083], [0.385, -0.054, 0.082, 0.118], [0.325, -0.06, 0.096, 0.134], [0.265, -0.059, 0.098, 0.137], [0.215, -0.054, 0.091, 0.131]] : scale === 200 ? [[0.4, -0.052, 0.072, 0.108], [0.31, -0.06, 0.094, 0.132], [0.22, -0.054, 0.088, 0.128]] : [];
    ribs.forEach(([s, cv, d, w], i) => k.ring(`rib-${i + 1}`, "torso", "bone", B, s!, cv!, d!, w!, r(0.0055), 18));
    if (scale === 500) k.loft("rib-cage", "torso", "bone", B, [[0.2, -0.052, 0, 0.11, 0.075], [0.32, -0.058, 0, 0.13, 0.09], [0.44, -0.046, 0, 0.09, 0.06]]);
    if (mid) k.bar("sternum", "torso", "bone", B, [[0.42, -0.116, 0, r(0.02), r(0.0075)], [0.36, -0.139, 0, r(0.017), r(0.0075)], [0.28, -0.146, 0, r(0.012), r(0.0075)]], undefined, { e: 3 });
    for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      if (mid) {
        k.bar(`clavicle-${s}`, "torso", "bone", B, [[0.436, -0.114, side * 0.022, r(0.0075)], [0.452, -0.101, side * 0.09, r(0.0065)], [0.458, -0.056, side * 0.17, r(0.0075)]]);
        k.ell(`scapula-${s}`, "torso", "bone", B, [0.365, 0.072, side * 0.1], [fine ? 0.075 : 0.07, r(fine ? 0.007 : 0.009), fine ? 0.05 : 0.048], undefined, { rotation: quat(aboutU(side * 26), aboutW(4)) });
        if (fine) k.bar(`acromion-${s}`, "torso", "bone", B, [[0.425, 0.067, side * 0.11, 0.007], [0.452, 0.022, side * 0.16, 0.007], [0.458, -0.026, side * 0.185, 0.008]]);
      } else if (scale === 200) k.bar(`shoulder-bar-${s}`, "torso", "bone", B, [[0.44, -0.056, side * 0.02, r(0.012)], [0.445, -0.04, side * 0.176, r(0.012)]]);
    }
    if (scale === 500) k.bar("shoulder-bar", "torso", "bone", B, [[0.435, -0.036, -0.15, r(0.02)], [0.435, -0.036, 0.15, r(0.02)]]);
  });
  member("neck", (k) => k.bar("cervical-spine", "neck", "bone", B, mid ? [[0.0, 0.0, 0, r(0.013)], [0.06, -0.012, 0, r(0.012)], [0.116, -0.004, 0, r(0.012)]] : [[0, 0, 0, r(0.014)], [0.116, -0.004, 0, r(0.014)]]));
  member("head", (k) => {
    k.ell("cranium", "head", "bone", B, [0.045, scale === 500 ? -0.01 : 0.004, 0], scale === 500 ? [0.08, 0.09, 0.07] : mid ? [0.082, 0.092, 0.07] : [0.08, 0.09, 0.068]);
    if (scale <= 200) k.loft("face-bones", "head", "bone", B - 0.06, mid ? [[-0.05, -0.06, 0, 0.036, 0.03], [-0.02, -0.066, 0, 0.056, 0.032], [0.02, -0.072, 0, 0.06, 0.026]] : [[-0.05, -0.058, 0, 0.042, 0.034], [0.02, -0.068, 0, 0.058, 0.034]]);
    if (mid) k.bar("mandible", "head", "bone", B, [[-0.005, 0.01, 0.054, r(0.008)], [-0.05, 0.004, 0.05, r(0.008)], [-0.078, -0.045, 0.028, r(0.008)], [-0.09, -0.07, 0, r(0.009)], [-0.078, -0.045, -0.028, r(0.008)], [-0.05, 0.004, -0.05, r(0.008)], [-0.005, 0.01, -0.054, r(0.008)]]);
  });
  for (const [s, side] of [["l", 1], ["r", -1]] as const) {
    const t = -side;
    member(`arm-${s}`, (k) => {
      if (mid) k.ell(`humerus-head-${s}`, `arm-${s}`, "bone", B, [0.004, -0.004, 0], [0.024, 0.024, 0.024]);
      k.bar(`humerus-${s}`, `arm-${s}`, "bone", B, [[0.01, 0, 0, r(0.0125)], [0.16, 0.002, 0, r(0.011)], [0.31, 0, 0, r(0.0115)]]);
      if (mid) k.ell(`humerus-condyles-${s}`, `arm-${s}`, "bone", B, [0.315, 0, 0], [r(0.014), r(0.014), 0.031]);
    });
    member(`forearm-${s}`, (k) => {
      if (mid) {
        k.bar(`radius-${s}`, `forearm-${s}`, "bone", B, [[0.012, 0.004, side * 0.018, r(0.008)], [0.13, 0.002, side * 0.02, r(0.0075)], [0.25, 0, side * 0.014, r(0.011)]]);
        k.bar(`ulna-${s}`, `forearm-${s}`, "bone", B, [[-0.022, -0.012, side * -0.008, r(0.011)], [0.02, -0.004, side * -0.014, r(0.0105)], [0.248, 0, side * -0.018, r(0.0075)]]);
      } else k.bar(`forearm-bones-${s}`, `forearm-${s}`, "bone", B, [[0, 0, 0, r(0.014)], [0.25, 0, 0, r(0.012)]]);
    });
    member(`hand-${s}`, (k) => {
      if (scale === 500) k.bar(`hand-bones-${s}`, `hand-${s}`, "bone", B, [[0, 0.0, 0, r(0.02)], [0.1, -0.004, 0, r(0.02)]]);
      else if (fine) {
        k.ell(`carpals-${s}`, `hand-${s}`, "bone", B, [0.014, 0, 0], [0.015, 0.009, 0.025]);
        FINGERS.forEach(([w, sc], i) => {
          k.bar(`metacarpal-${i + 1}-${s}`, `hand-${s}`, "bone", B, [[0.028, 0, t * w * 0.55, 0.0042], [0.088, -0.002, t * w, 0.0042]]);
          k.bar(`phalanges-${i + 1}-${s}`, `hand-${s}`, "bone", B, fingerPath(t * w, sc, 0.0041, GRIP_HAND.flex[i] as [number, number, number]).slice(1).map((p) => [p[0]!, p[1]!, p[2]!, 0.004]));
        });
        k.bar(`thumb-bones-${s}`, `hand-${s}`, "bone", B, GRIP_HAND.thumb.map(([u, p, w]) => [u, p, t * w, 0.0045]));
      } else k.loft(`hand-bones-${s}`, `hand-${s}`, "bone", B, scale === 100 ? [[0.004, 0, 0, 0.024, r(0.008)], [0.05, -0.002, 0, 0.032, r(0.008)], [0.094, -0.004, 0, 0.034, r(0.008)]] : [[0.004, -0.002, 0, 0.026, r(0.01)], [0.09, -0.006, 0, 0.03, r(0.01)]], undefined, { e: 3 });
    });
    member(`thigh-${s}`, (k) => {
      if (mid) k.ell(`femur-head-${s}`, `thigh-${s}`, "bone", B, [0, 0, 0], [0.024, 0.024, 0.024]);
      if (mid) k.bar(`femur-neck-${s}`, `thigh-${s}`, "bone", B, [[0, 0, 0, r(0.016)], [0.016, -0.004, side * 0.04, r(0.014)], [0.026, -0.008, side * 0.066, r(0.02)]]);
      k.bar(`femur-${s}`, `thigh-${s}`, "bone", B, mid ? [[0.022, -0.008, side * 0.064, r(0.017)], [0.11, -0.002, side * 0.048, r(0.0145)], [0.3, 0.004, side * 0.018, r(0.0145)], [0.41, 0, 0, r(0.02)]] : [[0, 0, side * 0.03, r(0.018)], [0.2, 0.004, side * 0.02, r(0.016)], [0.42, 0, 0, r(0.02)]]);
      if (mid) k.ell(`femur-condyles-${s}`, `thigh-${s}`, "bone", B, [0.418, -0.006, 0], [0.03, 0.033, 0.042]);
      if (mid) k.ell(`patella-${s}`, `thigh-${s}`, "bone", B, [0.455, 0.014, 0], [r(0.011), 0.021, 0.022]);
    });
    member(`shank-${s}`, (k) => {
      if (mid) k.ell(`tibia-plateau-${s}`, `shank-${s}`, "bone", B, [0.018, 0.002, 0], [0.02, 0.03, 0.04]);
      k.bar(`tibia-${s}`, `shank-${s}`, "bone", B, mid ? [[0.02, 0.01, side * -0.004, r(0.019)], [0.12, 0.014, side * -0.006, r(0.015)], [0.31, 0.01, side * -0.004, r(0.0125)], [0.42, 0.002, side * -0.006, r(0.017)]] : [[0, 0.006, 0, r(0.02)], [0.2, 0.012, 0, r(0.016)], [0.425, 0.002, 0, r(0.018)]]);
      if (mid) k.bar(`fibula-${s}`, `shank-${s}`, "bone", B, [[0.03, -0.014, side * 0.034, r(0.0085)], [0.22, -0.012, side * 0.032, r(0.006)], [0.425, -0.008, side * 0.03, r(0.0095)]]);
    });
    const level = { rotation: aboutW(FOOT_DROP) };
    member(`foot-${s}`, (k) => {
      if (mid) k.ell(`calcaneus-${s}`, `foot-${s}`, "bone", B, [-0.03, -0.05, 0], [0.038, 0.021, 0.018], undefined, level);
      k.loft(`tarsals-${s}`, `foot-${s}`, "bone", B, mid ? [[-0.004, -0.02, 0, 0.022, 0.018], [0.06, -0.042, 0, 0.03, 0.012], [0.138, -0.062, 0, 0.036, r(0.0085)]] : scale === 200 ? [[-0.05, -0.05, 0, 0.022, 0.02], [0.03, -0.04, 0, 0.03, 0.018], [0.138, -0.062, 0, 0.034, r(0.012)]] : [[-0.04, -0.05, 0, 0.038, 0.038], [0.13, -0.05, 0, 0.038, 0.038]], undefined, level);
    });
    member(`toe-${s}`, (k) => k.loft(`toe-bones-${s}`, `toe-${s}`, "bone", B, mid ? [[-0.004, -0.02, 0, 0.036, r(0.007)], [0.035, -0.022, 0, 0.032, r(0.0065)], [0.062, -0.024, 0, 0.024, r(0.006)]] : scale === 200 ? [[0.0, -0.02, 0, 0.034, r(0.01)], [0.065, -0.022, 0, 0.026, r(0.01)]] : [[0.0, -0.02, 0, 0.038, 0.038], [0.08, -0.02, 0, 0.038, 0.038]], undefined, { e: 3 }));
  }
  const joints: [string, number, V3?][] = [["torso", 0.018], ["neck", 0.0135], ["head", 0.0125], ["arm-l", 0.0235], ["arm-r", 0.0235], ["forearm-l", 0.0165], ["forearm-r", 0.0165], ["hand-l", 0.0125], ["hand-r", 0.0125], ["thigh-l", 0.0245], ["thigh-r", 0.0245], ["shank-l", 0.02], ["shank-r", 0.02], ["foot-l", 0.0165], ["foot-r", 0.0165], ["toe-l", 0.0095], ["toe-r", 0.0095]];
  for (const [bone, radius, at] of joints) {
    if (scale >= 200 && /^(toe|hand)/.test(bone)) continue;
    if (scale === 500 && /^(head|neck|foot)/.test(bone)) continue;
    member(bone, (k) => k.ell(`joint-${bone}`, bone, "bone", 0.4, at ?? [0, 0, 0], [r(radius) * 1.04, r(radius) * 1.04, r(radius) * 1.04]), "joint");
  }
  for (const [s, sd] of [["l", 1], ["r", -1]] as const) {
    member("rear-wheel", (k) => k.cap(`wheel-bearing-${s}`, "rear-wheel", "aluminium", 0.4, [0, 0, sd * 0.236], [0, 0, sd * 0.262], r(0.014), r(0.014)), "joint");
    member("castor", (k) => k.cap(`castor-bearing-${s}`, "castor", "aluminium", 0.4, [0, 0, sd * (CHAIR.castorZ - 0.022)], [0, 0, sd * (CHAIR.castorZ + 0.022)], r(0.009), r(0.009)), "joint");
  }
  const pairs: Record<string, readonly V3[]> = { "rear-wheel": [[0, 0, CHAIR.wheelZ], [0, 0, -CHAIR.wheelZ]], castor: [[0, 0, CHAIR.castorZ], [0, 0, -CHAIR.castorZ]] };
  const frameParts: readonly (readonly [string, number, V3])[] = [["side-l", 1.1, [0.2, 0.171, 0.215]], ["side-r", 1.1, [0.2, 0.171, -0.215]], ["lower-l", 0.5, [0.2, -0.041, 0.215]], ["lower-r", 0.5, [0.2, -0.041, -0.215]], ["post-l", 0.6, [-0.08, 0.345, 0.215]], ["post-r", 0.6, [-0.08, 0.345, -0.215]], ["axle-l", 0.4, [-0.006, 0.08, 0.232]], ["axle-r", 0.4, [-0.006, 0.08, -0.232]], ["castor-housing-l", 0.25, [0.476, -0.1, 0.215]], ["castor-housing-r", 0.25, [0.476, -0.1, -0.215]], ["upholstery", 1.0, [0.15, 0.225, 0]], ["footrest", 0.8, [0.556, -0.2285, 0]]];
  for (const bone of asset.rig.bones as Json[]) {
    const m = bone.mass;
    if (m === undefined) continue;
    if (bone.id === "frame" && scale <= 200) {
      member("frame", (k) => frameParts.forEach(([id, kg, at]) => k.ell(`mass-frame-${id}`, "frame", "bone", 0.2, at, [r(0.0125 * Math.cbrt(kg)), r(0.0125 * Math.cbrt(kg)), r(0.0125 * Math.cbrt(kg))], undefined, { role: "detail" })), "mass");
      continue;
    }
    if (pairs[bone.id] !== undefined && !(scale >= 200 && bone.id === "castor")) {
      member(bone.id, (k) => pairs[bone.id]!.forEach((at, i) => k.ell(`mass-${bone.id}-${i === 0 ? "l" : "r"}`, bone.id, "bone", 0.2, at, [r(0.0125 * Math.cbrt(m.kg / 2)), r(0.0125 * Math.cbrt(m.kg / 2)), r(0.0125 * Math.cbrt(m.kg / 2))], undefined, { role: "detail" })), "mass");
      continue;
    }
    if (scale === 100 && m.kg < 1.1) continue;
    if (scale === 200 && m.kg < 3) continue;
    if (scale === 500 && m.kg < 6) continue;
    const radius = r(0.0125 * Math.cbrt(m.kg));
    member(bone.id, (k) => k.ell(`mass-${bone.id}`, bone.id, "bone", 0.2, [m.comX, m.comY ?? 0, 0], [radius, radius, radius], undefined, { role: "detail" }), "mass");
  }
  const memberOf: readonly RegExp[] = [/^(side-tube|lower-tube|back-post|axle-plate|axle-receiver|axle-strut|castor-housing|fork-crown|fork-stem|fork-leg-[ab]|castor-axle)-[lr]$/, /^(footrest-loop|seat-cross|rear-cross|back-cross|back-cross-arm-[lr]|footplate|frame-[a-z-]+)$/, /^(rear-rim|rear-hub|hand-rim|spokes-[a-z-]+|castor-wheel|wheel|wheel-disc|castor)-[lr]$/, /^(side-frame|front-frame|back-frame|seat-frame)(-[lr])?$/];
  for (const s of body) if (memberOf.some((re) => re.test(s.id))) out.push({ id: `st-${s.id}`, of: s.id, member: s.bone, part: "member" });
  return out;
}
