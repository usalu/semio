/** 📋️ The lists of the round 3 seated office worker per scale: which body is built, its thermal model (nested layer shells at 1:50, the bare skin of face and neck, breath, plume and fan exhaust as outlet patches), its structure (the skeleton of the sitter, the frame of the chair from the castors to the backrest, the desk frame and the notebook) and the decisions.
 * @see ./r3-seated-office-worker-s50.ts
 * @see ./r3-seated-office-worker-coarse.ts */
import { Kit, aboutU, aboutW, parts, quat, thermalOf, type Json, type V3 } from "./r3-seated-office-worker-kit.ts";
import { body100, body200, body500 } from "./r3-seated-office-worker-coarse.ts";
import { DECISIONS_TEXT } from "./r3-seated-office-worker-texts.ts";
import { FOOT_DROP, TYPING, body50, fingerPath } from "./r3-seated-office-worker-s50.ts";
import { CHAIR, DESK, STAR } from "./r3-seated-office-worker-ref.ts";
import { local } from "./r3-seated-office-worker-frame.ts";

export const BODY: Record<number, (k: Kit) => void> = { 50: body50, 100: body100, 200: body200, 500: body500 };
export const DECISIONS: Record<number, Json[]> = DECISIONS_TEXT;
export const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const MATERIALS: Json[] = [{ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" }];
/** 📐️ Total outer areas of passive regions corrected at the root from the honest 1:50 model (C16: within a factor 1.5 of the old value; C29: faces glued to another part are C25 b section faces): upholstery cover on seat top, seat edges, backrest front and edges; ten tyre rings of the twin-wheel castors with their bores on the wheel cores. */
export const AREAS: Record<string, number> = { "chair-fabric": 0.51, "castor-rubber": 0.053 };
/** 🧾️ Root data that follow the model: the materials the solids name and the corrected passive areas. */
export function finish(asset: Json): void {
  const have = new Set((asset.materials as Json[]).map((m) => m.id));
  asset.materials.push(...MATERIALS.filter((m) => !have.has(m.id)));
  for (const region of asset.thermal.regions as Json[]) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];
}

/* ---------------------------------------------------------------- thermal */

const OUTER_ONLY = /^(thigh|shank|shoe-heel|shoe-sole|shoe-toe-sole|belt|thigh-[lr]|shank-[lr])/;
const NO_SURFACE = /^(eye|mouth|brow|belt-buckle)/;
/** 🧷️ Faces of upholstery and shells that are glued to each other (C25 b): the cushion on its seat shell, the backrest pad on its shell. */
const GLUED: Record<string, string[]> = { "chair-seat-cushion": ["-v"], "chair-seat-shell": ["+v"], "chair-backrest-pad": ["+v"], "chair-backrest-shell": ["-v"] };
/** 🌡️ The thermal list of a scale: every body solid that is a surface of its region (texture and face details are no surfaces of their own); at 1:50 garments, shoes, upholstery and shells as nested layers (briefs only under the seat of the trousers, socks only inside the uppers), the emitters as patches on the surface they leave. */
export function thermal(asset: Json, kit: Kit, scale: number): Json[] {
  const regions = new Map((asset.thermal.regions as Json[]).map((r) => [r.id, r]));
  const out: Json[] = [];
  for (const s of kit.solids) {
    const region = kit.region.get(s.id);
    if (region === undefined) throw new Error(`no region for ${s.id}`);
    if (s.texture === true || NO_SURFACE.test(s.id)) continue;
    const r = regions.get(region)!;
    const layers: Json[] = r.layers ?? [];
    if (region === "face-skin" && scale <= 100) continue;
    if (scale === 50 && layers.length > 1 && OUTER_ONLY.test(s.id)) out.push({ id: `th-${s.id}`, of: s.id, region, layer: layers[layers.length - 1]!.id });
    else out.push(...thermalOf(s, r, scale === 50).map((t) => (GLUED[s.id] === undefined || t.of === undefined ? t : { ...t, sectionFaces: GLUED[s.id] })));
  }
  if (scale <= 100) {
    const k = new Kit();
    k.ell("th-face", "head", "skin", 0.74, [-0.024, -0.046, 0], [0.084, 0.052, 0.066]);
    k.loft("th-neck", "neck", "skin", 0.7, [[0.0, -0.008, 0, 0.058, 0.06], [0.05, -0.011, 0, 0.054, 0.057], [0.1, -0.011, 0, 0.053, 0.057]], undefined, { caps: [false, false] });
    out.push(...k.solids.map((s) => ({ ...s, region: "face-skin" })));
  }
  const f = FEATURE[scale]!;
  const k = new Kit();
  const patch = Math.max(f * 0.51, 0.004);
  const coarse = scale >= 200;
  k.ell("th-nose-breath", "head", "skin", 0.5, coarse ? [-0.03, -0.096, 0] : [-0.036, -0.112, 0], [patch, patch, Math.max(0.0075, patch)]);
  k.ell("th-mouth-breath", "head", "skin", 0.5, coarse ? [-0.062, -0.088, 0] : [-0.06, -0.102, 0], [patch, patch, Math.max(0.011, patch)]);
  k.ell("th-body-plume", "head", "hair", 0.4, [coarse ? 0.13 : 0.1405, 0.016, 0], [Math.max(0.004, f * 0.5), Math.max(0.03, f * 0.6), Math.max(0.025, f * 0.6)]);
  const vent = { 50: 0.0076, 100: 0.0152, 200: 0.0305, 500: 0.0305 }[scale]!;
  k.box("th-laptop-vent", "laptop", "laptop-aluminium", 0.3, [-0.008 - vent * 0.3, 0.0135 - vent / 2, 0.06], [vent, vent, 0.06], 0.001);
  const emitter: Record<string, [string, string]> = { "th-nose-breath": ["face-skin", "nose-breath"], "th-mouth-breath": ["face-skin", "mouth-breath"], "th-body-plume": ["shirt", "body-plume"], "th-laptop-vent": ["laptop", "laptop-vent"] };
  out.push(...k.solids.map((s) => ({ ...s, region: emitter[s.id]![0], tags: ["emitter", emitter[s.id]![1]] })));
  return out;
}

/* ---------------------------------------------------------------- structure */

const B = 0.62;
const rad = (d: number): number => (d * Math.PI) / 180;
/** 🦴️ The structure of a scale: the skeleton stated bone by bone in the rest frames (radii never below half the smallest feature, parts merged as the scale coarsens), the chair from the floor contacts through castors, star arms, gas lift and mechanism to seat, backrest and armrests, the desk frame and board, the notebook; masses at the de Leva centres. Body solids that are themselves the load-bearing parts (star arms, column, mechanism, shells, legs, rails, board, notebook) are referenced. */
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
    k.bar("sacrum", "pelvis", "bone", B, [[0.098, 0.036, 0, r(0.026), r(0.02)], [0.05, 0.062, 0, r(0.026), r(0.015)], [0.0, 0.08, 0, r(0.014), r(0.011)], ...(fine ? [[-0.03, 0.082, 0, 0.008, 0.007]] : [])]);
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
    const spine = fine ? [[-0.004, 0.0, 0, 0.024], [0.06, -0.022, 0, 0.023], [0.14, -0.02, 0, 0.021], [0.23, 0.004, 0, 0.019], [0.32, 0.024, 0, 0.017], [0.41, 0.022, 0, 0.015], [0.475, 0.008, 0, 0.014], [0.505, 0.0, 0, 0.013]] : mid ? [[0, 0, 0, 0.024], [0.08, -0.022, 0, 0.022], [0.22, 0.002, 0, 0.019], [0.36, 0.024, 0, 0.017], [0.505, 0.0, 0, 0.014]] : [[0, 0, 0, r(0.024)], [0.2, 0.0, 0, r(0.02)], [0.36, 0.02, 0, r(0.018)], [0.505, 0, 0, r(0.016)]];
    k.bar("spine", "torso", "bone", B, spine);
    const ribs: number[][] = fine ? [[0.455, -0.045, 0.048, 0.07], [0.43, -0.052, 0.068, 0.1], [0.4, -0.058, 0.082, 0.118], [0.365, -0.062, 0.092, 0.13], [0.33, -0.064, 0.098, 0.136], [0.295, -0.064, 0.1, 0.139], [0.26, -0.062, 0.099, 0.138], [0.228, -0.058, 0.094, 0.134]] : mid ? [[0.445, -0.05, 0.056, 0.085], [0.395, -0.058, 0.084, 0.12], [0.335, -0.064, 0.098, 0.136], [0.275, -0.063, 0.1, 0.139], [0.225, -0.058, 0.093, 0.133]] : scale === 200 ? [[0.41, -0.056, 0.074, 0.11], [0.32, -0.064, 0.096, 0.134], [0.23, -0.058, 0.09, 0.13]] : [];
    ribs.forEach(([s, cv, d, w], i) => k.ring(`rib-${i + 1}`, "torso", "bone", B, s!, cv!, d!, w!, r(0.0055), -18));
    if (scale === 500) k.loft("rib-cage", "torso", "bone", B, [[0.2, -0.055, 0, 0.11, 0.075], [0.32, -0.062, 0, 0.13, 0.09], [0.44, -0.05, 0, 0.09, 0.06]]);
    if (mid) k.bar("sternum", "torso", "bone", B, [[0.43, -0.12, 0, r(0.02), r(0.0075)], [0.37, -0.143, 0, r(0.017), r(0.0075)], [0.29, -0.15, 0, r(0.012), r(0.0075)]], undefined, { e: 3 });
    for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      if (mid) {
        k.bar(`clavicle-${s}`, "torso", "bone", B, [[0.446, -0.118, side * 0.022, r(0.0075)], [0.462, -0.105, side * 0.09, r(0.0065)], [0.468, -0.06, side * 0.17, r(0.0075)]]);
        { const h = fine ? 0.075 : 0.07, w = fine ? 0.05 : 0.048, t = r(fine ? 0.007 : 0.009); k.loft(`scapula-${s}`, "torso", "bone", B, [[h * 0.92, 0, 0, w * 0.94, t], [h * 0.6, 0, side * w * 0.04, w, t], [0, 0, side * -w * 0.22, w * 0.74, t], [-h * 0.6, 0, side * -w * 0.52, w * 0.42, t], [-h, 0, side * -w * 0.76, r(w * 0.2), t]], undefined, { translation: [0.37, 0.07, side * 0.1], rotation: quat(aboutU(side * 26), aboutW(4)) }); }
        if (fine) k.bar(`acromion-${s}`, "torso", "bone", B, [[0.43, 0.065, side * 0.11, 0.007], [0.462, 0.02, side * 0.16, 0.007], [0.468, -0.03, side * 0.185, 0.008]]);
      } else if (scale === 200) k.bar(`shoulder-bar-${s}`, "torso", "bone", B, [[0.45, -0.06, side * 0.02, r(0.012)], [0.455, -0.045, side * 0.176, r(0.012)]]);
    }
    if (scale === 500) k.bar("shoulder-bar", "torso", "bone", B, [[0.445, -0.04, -0.15, r(0.02)], [0.445, -0.04, 0.15, r(0.02)]]);
  });
  member("neck", (k) => k.bar("cervical-spine", "neck", "bone", B, mid ? [[0.0, 0.0, 0, r(0.013)], [0.06, -0.012, 0, r(0.012)], [0.118, -0.004, 0, r(0.012)]] : [[0, 0, 0, r(0.014)], [0.118, -0.004, 0, r(0.014)]]));
  member("head", (k) => {
    k.ell("cranium", "head", "bone", B, [0.045, scale === 500 ? -0.01 : 0.004, 0], scale === 500 ? [0.08, 0.09, 0.07] : mid ? [0.082, 0.092, 0.07] : [0.08, 0.09, 0.068]);
    if (scale <= 200) k.loft("face-bones", "head", "bone", B - 0.06, mid ? [[-0.05, -0.062, 0, 0.036, 0.03], [-0.02, -0.068, 0, 0.056, 0.032], [0.02, -0.074, 0, 0.06, 0.026]] : [[-0.05, -0.06, 0, 0.042, 0.034], [0.02, -0.07, 0, 0.058, 0.034]]);
    if (mid) k.bar("mandible", "head", "bone", B, [[-0.005, 0.01, 0.054, r(0.008)], [-0.05, 0.004, 0.05, r(0.008)], [-0.08, -0.045, 0.028, r(0.008)], [-0.092, -0.07, 0, r(0.009)], [-0.08, -0.045, -0.028, r(0.008)], [-0.05, 0.004, -0.05, r(0.008)], [-0.005, 0.01, -0.054, r(0.008)]]);
  });
  for (const [s, side] of [["l", 1], ["r", -1]] as const) {
    const t = -side;
    member(`arm-${s}`, (k) => {
      if (mid) k.ell(`humerus-head-${s}`, `arm-${s}`, "bone", B, [0.004, -0.004, 0], [0.024, 0.024, 0.024]);
      k.bar(`humerus-${s}`, `arm-${s}`, "bone", B, [[0.01, 0, 0, r(0.0125)], [0.16, 0.002, 0, r(0.011)], [0.31, 0, 0, r(0.0115)]]);
      if (mid) k.ell(`humerus-condyles-${s}`, `arm-${s}`, "bone", B, [0.318, 0, 0], [r(0.014), r(0.014), 0.031]);
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
        [0.029, 0.0095, -0.0105, -0.029].forEach((w, i) => {
          k.bar(`metacarpal-${i + 1}-${s}`, `hand-${s}`, "bone", B, [[0.028, 0, t * w * 0.55, 0.0042], [0.088, -0.002, t * w, 0.0042]]);
          k.bar(`phalanges-${i + 1}-${s}`, `hand-${s}`, "bone", B, fingerPath(t * w, [0.94, 1, 0.95, 0.8][i]!, 0.0041, TYPING.flex[i] as [number, number, number]).slice(1).map((p) => [p[0]!, p[1]!, p[2]!, 0.004]));
        });
        k.bar(`thumb-bones-${s}`, `hand-${s}`, "bone", B, TYPING.thumb.map(([u, p, w]) => [u, p, t * w, 0.0045]));
      } else k.loft(`hand-bones-${s}`, `hand-${s}`, "bone", B, scale === 100 ? [[0.004, 0, 0, 0.024, r(0.008)], [0.05, -0.002, 0, 0.032, r(0.008)], [0.094, -0.004, 0, 0.034, r(0.008)]] : [[0.004, -0.002, 0, 0.026, r(0.01)], [0.09, -0.006, 0, 0.03, r(0.01)]], undefined, { e: 3 });
    });
    member(`thigh-${s}`, (k) => {
      if (mid) k.ell(`femur-head-${s}`, `thigh-${s}`, "bone", B, [0, 0, 0], [0.024, 0.024, 0.024]);
      if (mid) k.bar(`femur-neck-${s}`, `thigh-${s}`, "bone", B, [[0, 0, 0, r(0.016)], [0.016, -0.004, side * 0.04, r(0.014)], [0.026, -0.008, side * 0.066, r(0.02)]]);
      k.bar(`femur-${s}`, `thigh-${s}`, "bone", B, mid ? [[0.022, -0.008, side * 0.064, r(0.017)], [0.11, -0.002, side * 0.048, r(0.0145)], [0.3, 0.004, side * 0.018, r(0.0145)], [0.41, 0, 0, r(0.02)]] : [[0, 0, side * 0.03, r(0.018)], [0.2, 0.004, side * 0.02, r(0.016)], [0.42, 0, 0, r(0.02)]]);
      if (mid) k.ell(`femur-condyles-${s}`, `thigh-${s}`, "bone", B, [0.422, -0.006, 0], [0.03, 0.033, 0.042]);
      if (mid) k.ell(`patella-${s}`, `thigh-${s}`, "bone", B, [0.462, 0.012, 0], [r(0.011), 0.021, 0.022]);
    });
    member(`shank-${s}`, (k) => {
      if (mid) k.ell(`tibia-plateau-${s}`, `shank-${s}`, "bone", B, [0.018, 0.002, 0], [0.02, 0.03, 0.04]);
      k.bar(`tibia-${s}`, `shank-${s}`, "bone", B, mid ? [[0.02, 0.01, side * -0.004, r(0.019)], [0.12, 0.014, side * -0.006, r(0.015)], [0.31, 0.01, side * -0.004, r(0.0125)], [0.425, 0.002, side * -0.006, r(0.017)]] : [[0, 0.006, 0, r(0.02)], [0.2, 0.012, 0, r(0.016)], [0.43, 0.002, 0, r(0.018)]]);
      if (mid) k.bar(`fibula-${s}`, `shank-${s}`, "bone", B, [[0.03, -0.014, side * 0.034, r(0.0085)], [0.22, -0.012, side * 0.032, r(0.006)], [0.432, -0.008, side * 0.03, r(0.0095)]]);
    });
    const level = { rotation: aboutW(FOOT_DROP) };
    member(`foot-${s}`, (k) => {
      if (mid) k.ell(`calcaneus-${s}`, `foot-${s}`, "bone", B, [-0.03, -0.052, 0], [0.04, 0.022, 0.018], undefined, level);
      k.loft(`tarsals-${s}`, `foot-${s}`, "bone", B, mid ? [[-0.004, -0.02, 0, 0.022, 0.018], [0.06, -0.044, 0, 0.03, 0.012], [0.135, -0.07, 0, 0.038, r(0.0085)]] : scale === 200 ? [[-0.05, -0.05, 0, 0.022, 0.02], [0.03, -0.04, 0, 0.03, 0.018], [0.135, -0.066, 0, 0.034, r(0.012)]] : [[-0.04, -0.05, 0, 0.038, 0.038], [0.13, -0.05, 0, 0.038, 0.038]], undefined, level);
    });
    member(`toe-${s}`, (k) => k.loft(`toe-bones-${s}`, `toe-${s}`, "bone", B, mid ? [[-0.004, 0.022, 0, 0.038, r(0.007)], [0.035, 0.02, 0, 0.034, r(0.0065)], [0.066, 0.018, 0, 0.026, r(0.006)]] : scale === 200 ? [[0.0, 0.022, 0, 0.034, r(0.01)], [0.07, 0.02, 0, 0.026, r(0.01)]] : [[0.0, 0.038, 0, 0.038, 0.038], [0.077, 0.038, 0, 0.038, 0.038]], undefined, { e: 3 }));
  }
  const joints: [string, number, V3?][] = [["torso", 0.018], ["neck", 0.0135], ["head", 0.0125], ["arm-l", 0.0235], ["arm-r", 0.0235], ["forearm-l", 0.0165], ["forearm-r", 0.0165], ["hand-l", 0.0125], ["hand-r", 0.0125], ["thigh-l", 0.0245], ["thigh-r", 0.0245], ["shank-l", 0.02], ["shank-r", 0.02], ["foot-l", 0.0165], ["foot-r", 0.0165], ["toe-l", 0.0095, [0, 0.022, 0]], ["toe-r", 0.0095, [0, 0.022, 0]], ["chair-backrest", 0.014, [0.016, 0, 0]], ["screen", 0.008, [0, -0.002, 0]]];
  for (const [bone, radius, at] of joints) {
    if (scale >= 200 && /^(toe|hand|screen)/.test(bone)) continue;
    if (scale === 500 && /^(head|neck|foot|chair-backrest)/.test(bone)) continue;
    member(bone, (k) => k.ell(`joint-${bone}`, bone, "bone", 0.4, at ?? [0, 0, 0], [r(radius) * 1.04, r(radius) * 1.04, r(radius) * 1.04]), "joint");
  }
  for (const bone of asset.rig.bones as Json[]) {
    const m = bone.mass;
    if (m === undefined) continue;
    if (scale === 100 && m.kg < 1.1) continue;
    if (scale === 200 && m.kg < 3) continue;
    if (scale === 500 && m.kg < 6) continue;
    const radius = r(0.0125 * Math.cbrt(m.kg));
    if (bone.id === "desk-leg") {
      const lx = DESK.inset + DESK.leg / 2;
      const legs = [[DESK.x0 + lx, 1], [DESK.x0 + lx, -1], [DESK.x0 + DESK.depth - lx, 1], [DESK.x0 + DESK.depth - lx, -1]] as const;
      member("desk-leg", (k) => legs.forEach(([x, sd], i) => k.ell(`mass-desk-leg-${i + 1}`, "desk-leg", "bone", 0.2, local("desk-leg", [x, 0.4, sd * (DESK.width / 2 - lx)]), [r(0.016), r(0.016), r(0.016)], undefined, { role: "detail" })), "mass");
      continue;
    }
    member(bone.id, (k) => k.ell(`mass-${bone.id}`, bone.id, "bone", 0.2, [m.comX, m.comY ?? 0, 0], [radius, radius, radius], undefined, { role: "detail" }), "mass");
  }
  const memberOf: readonly (readonly [RegExp, string | null])[] = [[/^chair-arm-/, null], [/^chair-hub$/, "chair-base"], [/^chair-column-cover$/, "chair-column-sleeve"], [/^chair-column$/, "chair-column"], [/^chair-(mechanism|seat-shell|seat)$/, "chair-seat"], [/^chair-armrest-post-/, "chair-seat"], [/^chair-backrest-(shell|strut|frame)$/, "chair-backrest"], [/^chair-stem-/, null], [/^desk-(leg|rail|frame)/, "desk-leg"], [/^desk-top$/, "desk-top"], [/^laptop-base$/, "laptop"], [/^laptop-lid$/, "screen"]];
  for (const s of body) {
    const hit = memberOf.find(([re]) => re.test(s.id));
    if (hit === undefined) continue;
    out.push({ id: `st-${s.id}`, of: s.id, member: hit[1] ?? s.bone, part: "member" });
  }
  if (scale === 500) {
    member("chair-column-sleeve", (k) => k.loft("column-tube", "chair-column-sleeve", "steel-painted", B, [[0.03, 0, 0, 0.038, 0.038], [0.24, 0, 0, 0.038, 0.038]]));
    member("chair-seat", (k) => k.box("seat-plate", "chair-seat", "pa-plastic", B, [0, 0.045, 0], [0.4, 0.075, 0.38], 0.01));
    member("chair-backrest", (k) => k.box("backrest-frame", "chair-backrest", "pa-plastic", B, [0.355, 0.004, 0], [0.45, 0.075, 0.3], 0.01));
  }
  member("floor", (k) => {
    const pad = Math.max(f * 1.05, 0.004);
    for (const [id, yaw] of STAR) {
      const c = [CHAIR.columnX + (CHAIR.armReach - CHAIR.castorTrail) * Math.cos(rad(yaw)), pad, (CHAIR.armReach - CHAIR.castorTrail) * Math.sin(rad(yaw))] as const;
      k.ell(`floor-castor-${id}`, "floor", "bone", 0.3, [c[0], c[1], c[2]], [Math.max(0.012, pad * 1.5), pad, Math.max(0.012, pad * 1.5)]);
    }
    const lx = DESK.inset + DESK.leg / 2;
    for (const [id, x, z] of [["fl", DESK.x0 + lx, DESK.width / 2 - lx], ["fr", DESK.x0 + lx, -(DESK.width / 2 - lx)], ["bl", DESK.x0 + DESK.depth - lx, DESK.width / 2 - lx], ["br", DESK.x0 + DESK.depth - lx, -(DESK.width / 2 - lx)]] as const) k.ell(`floor-desk-${id}`, "floor", "bone", 0.3, [x, pad, z], [Math.max(0.012, pad * 1.5), pad, Math.max(0.012, pad * 1.5)]);
  });
  return out;
}
