/** 📋️ The lists of the round 3 seated guitarist per scale: which body is built, its thermal model (nested layer shells at 1:50, the face as the skin that is really bare, breath and plume as outlet patches), its structure (the skeleton of the player, the load path of the guitar and the frame of the stool), the decisions, and the root data that follow the model (materials).
 * @see ./r3-seated-guitarist-s50.ts
 * @see ./r3-seated-guitarist-coarse.ts */
import { Kit, aboutU, aboutW, parts, quat, thermalOf, type Json } from "./r3-seated-guitarist-kit.ts";
import { body100, body200, body500 } from "./r3-seated-guitarist-coarse.ts";
import { ANSUR, DECISIONS_TEXT, DRAWING, GUITAR_SPEC, NOTES, STRUCTURE_ASSUMPTION, SUMMARY, WINTER } from "./r3-seated-guitarist-texts.ts";
import { GRIPS, body50, fingerPath } from "./r3-seated-guitarist-s50.ts";

export const BODY: Record<number, (k: Kit) => void> = { 50: body50, 100: body100, 200: body200, 500: body500 };
export const DECISIONS: Record<number, Json[]> = DECISIONS_TEXT;
const MATERIALS: Json[] = [
  { id: "cotton-jersey", title: { en: "Cotton jersey T-shirt", de: "T-Shirt aus Baumwolljersey" }, emissivity: 0.93, solarAbsorptivity: 0.35, source: "authored-estimate", reference: "iso-9920-2007" },
  { id: "rubber", title: { en: "Rubber boot sole", de: "Stiefelsohle aus Gummi" }, emissivity: 0.94, solarAbsorptivity: 0.9, densityKgM3: 1100, source: "authored-estimate" },
  { id: "steel", title: { en: "Nickel-plated steel (strings, frets, machine heads)", de: "Vernickelter Stahl (Saiten, Bünde, Mechaniken)" }, emissivity: 0.3, solarAbsorptivity: 0.45, densityKgM3: 7850, source: "authored-estimate" },
  { id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" },
];
/** 🧾️ Root data that follow the model: the materials the solids name, the stool area, the summary, provenance, structural assumption and the drawing decisions of every scale. */
export function finish(asset: Json): void {
  const have = new Set((asset.materials as Json[]).map((m) => m.id));
  asset.materials.push(...MATERIALS.filter((m) => !have.has(m.id)));
  for (const region of asset.thermal.regions as Json[]) if (region.id === "stool") region.areaM2 = 0.515;
  asset.summary = SUMMARY;
  asset.provenance.notes = NOTES;
  const references = asset.provenance.references as Json[];
  references.find((r) => r.id === "guitar-spec")!.citation = GUITAR_SPEC;
  for (const reference of [WINTER, ANSUR]) if (!references.some((r) => r.id === reference.id)) references.push(reference);
  asset.structure.assumptions = [STRUCTURE_ASSUMPTION];
  for (const [scale, decisions] of Object.entries(DRAWING)) asset.representations[scale].decisions = decisions;
}

/* ---------------------------------------------------------------- thermal */

const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
/** 🌡️ The thermal list of a scale: every body solid that is a surface of its region (texture, strings and face details are no surfaces of their own), garments as nested shells at 1:50, the bare skin of face and neck as its own solids, and the three emitters as patches on the surface they leave. */
export function thermal(asset: Json, kit: Kit, scale: number): Json[] {
  const regions = new Map((asset.thermal.regions as Json[]).map((r) => [r.id, r]));
  const out: Json[] = [];
  const unlayered = /^(boot-heel|boot-sole|boot-toe-sole|tee-neck)/;
  for (const s of kit.solids) {
    const region = kit.region.get(s.id);
    if (region === undefined) throw new Error(`no region for ${s.id}`);
    if (s.texture === true || s.slender === true || /^boot-shaft/.test(s.id)) continue;
    if (region === "face" && scale <= 100) continue;
    if (/^(eye|mouth|brow)/.test(s.id)) continue;
    out.push(...thermalOf(s, regions.get(region)!, scale === 50 && !unlayered.test(s.id)));
  }
  if (scale <= 100) {
    const k = new Kit();
    k.ell("th-face", "head", "skin", 0.74, scale === 50 ? [0.103, -0.076, 0] : [0.1, -0.074, 0], scale === 50 ? [0.084, 0.047, 0.063] : [0.08, 0.05, 0.062]);
    k.loft("th-neck", "head", "skin", 0.7, scale === 50 ? [[-0.004, -0.03, 0, 0.0588, 0.0588], [0.03, -0.033, 0, 0.0566, 0.0566], [0.058, -0.03, 0, 0.0558, 0.0575]] : [[0, -0.03, 0, 0.059, 0.059], [0.056, -0.03, 0, 0.056, 0.058]], undefined, { caps: [false, false] });
    out.push(...k.solids.map((s) => ({ ...s, region: "face" })));
  }
  const f = FEATURE[scale]!;
  const k = new Kit();
  const patch = f * 0.51;
  const face: Record<number, [number, number]> = { 50: [-0.1235, -0.1115], 100: [-0.122, -0.11], 200: [-0.114, -0.098], 500: [-0.1, -0.09] };
  k.ell("th-nose-breath", "head", "skin", 0.5, [scale >= 200 ? 0.1 : 0.0855, face[scale]![0], 0], [patch, patch, Math.max(0.0095, patch)]);
  k.ell("th-mouth-breath", "head", "skin", 0.5, [scale >= 200 ? 0.045 : 0.066, face[scale]![1], 0], [patch, patch, Math.max(0.0124, patch)]);
  k.ell("th-plume", "head", "hair", 0.4, [scale === 500 ? 0.235 : 0.2555, -0.024, 0], [Math.max(0.005, f * 0.5), Math.max(0.03, f * 0.6), Math.max(0.025, f * 0.6)]);
  const emitter: Record<string, [string, string]> = { "th-nose-breath": ["face", "nose-breath"], "th-mouth-breath": ["face", "mouth-breath"], "th-plume": ["shirt-torso", "plume"] };
  out.push(...k.solids.map((s) => ({ ...s, region: emitter[s.id]![0], tags: ["emitter", emitter[s.id]![1]] })));
  return out;
}

/* ---------------------------------------------------------------- structure */

const MASS: readonly (readonly [bone: string, kg: number, x: number, y: number])[] = [["pelvis", 8.266, 0.05, 0], ["torso", 23.895, 0.2101, 0], ["head", 5.136, 0.12, 0], ["upper-arm-l", 2.005, 0.179, 0], ["upper-arm-r", 2.005, 0.179, 0], ["forearm-l", 1.199, 0.1235, 0], ["forearm-r", 1.199, 0.1235, 0], ["thigh-l", 10.478, 0.1761, 0], ["thigh-r", 10.478, 0.1761, 0], ["shank-l", 3.204, 0.194, 0], ["shank-r", 3.204, 0.194, 0], ["foot-l", 1.014, 0.11, -0.0435], ["foot-r", 1.014, 0.11, -0.0435], ["guitar", 2.1, 0.18, 0.02]];
const B = 0.62;

/** 🦴️ The structure of a scale: the skeleton stated bone by bone in the rest frames of the rig (radii never below half the smallest feature of the scale, parts merged as the scale coarsens), the guitar's load path (strings, bridge, sound box, neck, head) and the stool's own frame by reference. */
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
    k.bar("sacrum", "pelvis", "bone", B, [[0.1, 0.04, 0, r(0.024), r(0.02)], [0.05, 0.07, 0, r(0.028), r(0.015)], [0.005, 0.082, 0, r(0.012), r(0.01)]]);
    if (scale <= 200) for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      k.ell(`ilium-${s}`, "pelvis", "bone", B, [0.08, 0.022, side * 0.108], [mid ? 0.066 : 0.07, mid ? 0.052 : 0.05, r(fine ? 0.011 : 0.013)], undefined, { rotation: quat(aboutU(side * -32), aboutW(12)) });
      if (mid) {
        k.bar(`pubis-${s}`, "pelvis", "bone", B, [[0.004, -0.004, side * 0.085, r(0.012)], [-0.012, -0.045, side * 0.04, r(0.01)], [-0.018, -0.066, side * 0.006, r(0.011)]]);
        k.bar(`ischium-${s}`, "pelvis", "bone", B, [[-0.004, 0.012, side * 0.086, r(0.014)], [-0.05, 0.032, side * 0.068, r(0.013)], [-0.076, 0.03, side * 0.058, r(0.015)], ...(fine ? [[-0.058, -0.02, side * 0.03, r(0.009)], [-0.028, -0.06, side * 0.008, r(0.009)]] : [])]);
      } else k.bar(`hip-bar-${s}`, "pelvis", "bone", B, [[0.002, 0.004, side * 0.088, r(0.02)], [-0.06, 0.02, side * 0.06, r(0.02)]]);
    }
    else k.bar("pelvic-bar", "pelvis", "bone", B, [[0.03, 0.01, -0.09, r(0.03)], [0.03, 0.01, 0.09, r(0.03)]]);
  });
  member("torso", (k) => {
    const spine = fine ? [[0, 0.045, 0, 0.023], [0.07, 0.03, 0, 0.022], [0.15, 0.04, 0, 0.02], [0.25, 0.064, 0, 0.018], [0.35, 0.074, 0, 0.016], [0.43, 0.058, 0, 0.014], [0.475, 0.034, 0, 0.013]] : mid ? [[0, 0.045, 0, 0.024], [0.08, 0.03, 0, 0.022], [0.2, 0.052, 0, 0.02], [0.34, 0.074, 0, 0.017], [0.475, 0.034, 0, 0.014]] : [[0, 0.04, 0, r(0.024)], [0.2, 0.05, 0, r(0.02)], [0.36, 0.066, 0, r(0.018)], [0.475, 0.034, 0, r(0.016)]];
    k.bar("spine", "torso", "bone", B, spine);
    const ribs: number[][] = fine ? [[0.415, 0, 0.05, 0.075], [0.385, -0.005, 0.07, 0.105], [0.35, -0.01, 0.085, 0.125], [0.31, -0.012, 0.095, 0.138], [0.27, -0.012, 0.1, 0.145], [0.23, -0.01, 0.1, 0.146], [0.195, -0.008, 0.096, 0.142], [0.165, -0.004, 0.088, 0.135]] : mid ? [[0.41, -0.002, 0.054, 0.082], [0.36, -0.01, 0.082, 0.122], [0.3, -0.012, 0.096, 0.14], [0.24, -0.01, 0.098, 0.144], [0.18, -0.006, 0.09, 0.136]] : scale === 200 ? [[0.38, -0.004, 0.058, 0.1], [0.29, -0.01, 0.08, 0.126], [0.2, -0.006, 0.076, 0.124]] : [];
    ribs.forEach(([s, cv, d, w], i) => k.ring(`rib-${i + 1}`, "torso", "bone", B, s!, cv!, d!, w!, r(0.0055), -18));
    if (scale === 500) k.loft("rib-cage", "torso", "bone", B, [[0.16, 0, 0, 0.085, 0.06], [0.28, 0, 0, 0.1, 0.07], [0.4, 0.004, 0, 0.07, 0.05]]);
    if (mid) k.bar("sternum", "torso", "bone", B, [[0.4, -0.066, 0, r(0.02), r(0.0075)], [0.33, -0.1, 0, r(0.018), r(0.0075)], [0.262, -0.118, 0, r(0.012), r(0.0075)]], undefined, { e: 3 });
    for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      if (mid) {
        k.bar(`clavicle-${s}`, "torso", "bone", B, [[0.412, -0.062, side * 0.016, r(0.0075)], [0.43, -0.05, side * 0.09, r(0.0065)], [0.448, -0.004, side * 0.178, r(0.0075)]]);
        { const h = fine ? 0.075 : 0.07, w = fine ? 0.05 : 0.048, t = r(fine ? 0.007 : 0.009); k.loft(`scapula-${s}`, "torso", "bone", B, [[h * 0.92, 0, 0, w * 0.94, t], [h * 0.6, 0, side * w * 0.04, w, t], [0, 0, side * -w * 0.22, w * 0.74, t], [-h * 0.6, 0, side * -w * 0.52, w * 0.42, t], [-h, 0, side * -w * 0.76, r(w * 0.2), t]], undefined, { translation: [0.355, 0.082, side * 0.112], rotation: quat(aboutU(side * 24), aboutW(-6)) }); }
        if (fine) k.bar(`acromion-${s}`, "torso", "bone", B, [[0.41, 0.07, side * 0.11, 0.007], [0.44, 0.045, side * 0.165, 0.007], [0.452, 0.006, side * 0.196, 0.008]]);
      } else if (scale === 200) k.bar(`shoulder-bar-${s}`, "torso", "bone", B, [[0.43, 0.02, side * 0.02, r(0.012)], [0.44, 0.004, side * 0.186, r(0.012)]]);
    }
    if (scale === 500) k.bar("shoulder-bar", "torso", "bone", B, [[0.432, 0.01, -0.15, r(0.02)], [0.432, 0.01, 0.15, r(0.02)]]);
  });
  member("head", (k) => {
    k.bar("cervical-spine", "head", "bone", B, mid ? [[0, 0.034, 0, r(0.013)], [0.05, 0.012, 0, r(0.012)], [0.1, 0.002, 0, r(0.012)]] : [[0, 0.03, 0, r(0.014)], [0.1, 0.002, 0, r(0.014)]]);
    k.ell("cranium", "head", "bone", B, [0.166, scale === 500 ? -0.022 : -0.016, 0], scale === 500 ? [0.075, 0.08, 0.062] : mid ? [0.08, 0.086, 0.067] : [0.078, 0.084, 0.064]);
    if (scale <= 200) k.loft("face-bones", "head", "bone", B - 0.06, mid ? [[0.076, -0.078, 0, 0.034, 0.03], [0.104, -0.078, 0, 0.056, 0.036], [0.14, -0.082, 0, 0.058, 0.03]] : [[0.07, -0.07, 0, 0.04, 0.034], [0.14, -0.076, 0, 0.056, 0.034]]);
    if (mid) k.bar("mandible", "head", "bone", B, [[0.1, -0.024, 0.056, r(0.008)], [0.056, -0.026, 0.05, r(0.008)], [0.034, -0.07, 0.03, r(0.008)], [0.03, -0.092, 0, r(0.009)], [0.034, -0.07, -0.03, r(0.008)], [0.056, -0.026, -0.05, r(0.008)], [0.1, -0.024, -0.056, r(0.008)]]);
  });
  for (const [s, side] of [["l", 1], ["r", -1]] as const) {
    member(`upper-arm-${s}`, (k) => {
      k.bar(`humerus-${s}`, `upper-arm-${s}`, "bone", B, [[0, 0, 0, r(0.0125)], [0.15, 0.002, 0, r(0.011)], [0.295, 0, 0, r(0.0115)]]);
      if (mid) k.ell(`humerus-condyles-${s}`, `upper-arm-${s}`, "bone", B, [0.304, 0, 0], [r(0.014), r(0.014), 0.031]);
    });
    member(`forearm-${s}`, (k) => {
      if (mid) {
        k.bar(`radius-${s}`, `forearm-${s}`, "bone", B, [[0.012, 0.004, side * 0.018, r(0.008)], [0.14, 0.002, side * 0.022, r(0.0075)], [0.266, 0, side * 0.016, r(0.011)]]);
        k.bar(`ulna-${s}`, `forearm-${s}`, "bone", B, [[-0.022, -0.014, side * -0.008, r(0.011)], [0.02, -0.004, side * -0.014, r(0.0105)], [0.264, 0, side * -0.019, r(0.0075)]]);
      } else k.bar(`forearm-bones-${s}`, `forearm-${s}`, "bone", B, [[0, 0, 0, r(0.014)], [0.266, 0, 0, r(0.012)]]);
    });
    member(`hand-${s}`, (k) => {
      if (scale === 500) k.bar(`hand-bones-${s}`, `hand-${s}`, "bone", B, [[0, 0.004, 0, r(0.02)], [0.1, 0.012, 0, r(0.02)]]);
      else if (fine) {
        k.ell(`carpals-${s}`, `hand-${s}`, "bone", B, [0.014, 0, 0], [0.015, 0.009, 0.025]);
        const flex = GRIPS[s === "l" ? "fret" : "strum"].flex;
        [0.03, 0.0095, -0.011, -0.03].forEach((w, i) => {
          k.bar(`metacarpal-${i + 1}-${s}`, `hand-${s}`, "bone", B, [[0.028, 0, side * w * 0.55, 0.0042], [0.09, -0.003, side * w, 0.0042]]);
          k.bar(`phalanges-${i + 1}-${s}`, `hand-${s}`, "bone", B, fingerPath(side * w, [0.94, 1, 0.95, 0.78][i]!, 0.0041, flex[i] as [number, number, number]).slice(1).map((p) => [p[0]!, p[1]!, p[2]!, 0.004]));
        });
        k.bar(`thumb-bones-${s}`, `hand-${s}`, "bone", B, GRIPS[s === "l" ? "fret" : "strum"].thumb.map(([u, p, w]) => [u, p, side * w, 0.0045]));
      } else k.loft(`hand-bones-${s}`, `hand-${s}`, "bone", B, scale === 100 ? [[0.004, 0, 0, 0.024, r(0.008)], [0.05, 0.002, 0, 0.032, r(0.008)], [0.094, 0.004, 0, 0.034, r(0.008)]] : [[0.004, 0.004, 0, 0.026, r(0.01)], [0.09, 0.008, 0, 0.03, r(0.01)]], undefined, { e: 3 });
    });
    member(`thigh-${s}`, (k) => {
      if (mid) k.bar(`femur-neck-${s}`, `thigh-${s}`, "bone", B, [[0, 0, 0, r(0.016)], [0.02, 0, side * 0.04, r(0.014)], [0.034, 0, side * 0.068, r(0.02)]]);
      k.bar(`femur-${s}`, `thigh-${s}`, "bone", B, mid ? [[0.03, 0, side * 0.066, r(0.017)], [0.12, 0.006, side * 0.05, r(0.0145)], [0.3, 0.006, side * 0.016, r(0.0145)], [0.41, 0, 0, r(0.02)]] : [[0, 0, side * 0.03, r(0.018)], [0.2, 0.006, side * 0.02, r(0.016)], [0.42, 0, 0, r(0.02)]]);
      if (mid) k.ell(`femur-condyles-${s}`, `thigh-${s}`, "bone", B, [0.424, -0.004, 0], [0.03, 0.033, 0.042]);
      if (mid) k.ell(`patella-${s}`, `thigh-${s}`, "bone", B, [0.456, 0.026, 0], [0.021, r(0.011), 0.022]);
    });
    member(`shank-${s}`, (k) => {
      if (mid) k.ell(`tibia-plateau-${s}`, `shank-${s}`, "bone", B, [0.016, 0.002, 0], [0.02, 0.03, 0.04]);
      k.bar(`tibia-${s}`, `shank-${s}`, "bone", B, mid ? [[0.02, 0.01, side * -0.004, r(0.019)], [0.12, 0.014, side * -0.006, r(0.015)], [0.3, 0.01, side * -0.004, r(0.0125)], [0.425, 0.002, side * -0.004, r(0.017)]] : [[0, 0.006, 0, r(0.02)], [0.2, 0.012, 0, r(0.016)], [0.428, 0.002, 0, r(0.018)]]);
      if (mid) k.bar(`fibula-${s}`, `shank-${s}`, "bone", B, [[0.03, -0.014, side * 0.036, r(0.0085)], [0.22, -0.012, side * 0.034, r(0.006)], [0.438, -0.008, side * 0.03, r(0.0095)]]);
    });
    member(`foot-${s}`, (k) => {
      if (mid) k.ell(`calcaneus-${s}`, `foot-${s}`, "bone", B, [-0.034, -0.034, 0], [0.04, 0.022, 0.018]);
      k.loft(`tarsals-${s}`, `foot-${s}`, "bone", B, mid ? [[-0.002, -0.014, 0, 0.022, 0.018], [0.06, -0.036, 0, 0.03, 0.012], [0.132, -0.052, 0, 0.04, r(0.0085)]] : scale === 200 ? [[-0.05, -0.03, 0, 0.022, 0.02], [0.03, -0.03, 0, 0.03, 0.018], [0.2, -0.05, 0, 0.034, r(0.012)]] : [[-0.04, -0.04, 0, 0.038, 0.038], [0.19, -0.04, 0, 0.038, 0.038]]);
    });
    member(`toe-${s}`, (k) => k.loft(`toe-bones-${s}`, `toe-${s}`, "bone", B, mid ? [[0, 0.027, 0, 0.038, r(0.007)], [0.04, 0.024, 0, 0.034, r(0.0065)], [0.074, 0.021, 0, 0.026, r(0.006)]] : scale === 200 ? [[0.004, 0.024, 0, 0.034, r(0.01)], [0.078, 0.022, 0, 0.026, r(0.01)]] : [[0.006, 0.0385, 0, 0.038, 0.038], [0.086, 0.0385, 0, 0.038, 0.038]], undefined, { e: 3 }));
  }
  const joints: [string, number, number[]?][] = [["torso", 0.018, [0, 0.04, 0]], ["head", 0.0135, [0, 0.032, 0]], ["upper-arm-l", 0.0235], ["upper-arm-r", 0.0235], ["forearm-l", 0.0165], ["forearm-r", 0.0165], ["hand-l", 0.0125], ["hand-r", 0.0125], ["thigh-l", 0.0245], ["thigh-r", 0.0245], ["shank-l", 0.02], ["shank-r", 0.02], ["foot-l", 0.0165], ["foot-r", 0.0165], ["toe-l", 0.0095, [0, 0.026, 0]], ["toe-r", 0.0095, [0, 0.026, 0]], ["guitar", 0.012]];
  for (const [bone, radius, at] of joints) {
    if (scale >= 200 && /^(toe|hand)/.test(bone)) continue;
    if (scale === 500 && /^(head|foot|guitar)/.test(bone)) continue;
    member(bone, (k) => k.ell(`joint-${bone}`, bone, "bone", 0.4, (at ?? [0, 0, 0]) as [number, number, number], [r(radius) * 1.04, r(radius) * 1.04, r(radius) * 1.04]), "joint");
  }
  for (const [bone, kg, x, y] of MASS) {
    if (scale === 100 && kg < 1.1) continue;
    if (scale === 200 && kg < 3) continue;
    if (scale === 500 && kg < 8) continue;
    const radius = r(0.0125 * Math.cbrt(kg));
    member(bone, (k) => k.ell(`mass-${bone}`, bone, "bone", 0.2, [x, y, 0], [radius, radius, radius], undefined, { role: "detail" }), "mass");
  }
  const has = new Set(body.map((s) => s.id));
  for (const s of body) {
    if (s.bone === "stool") out.push({ id: `st-${s.id}`, of: s.id, member: "stool", part: "member" });
    if (/^guitar-(body|neck|heel|head|bridge|string-\d)$/.test(s.id)) out.push({ id: `st-${s.id}`, of: s.id, member: "guitar", part: "member" });
  }
  if (!has.has("guitar-body")) throw new Error("the guitar has no sound box");
  return out;
}
