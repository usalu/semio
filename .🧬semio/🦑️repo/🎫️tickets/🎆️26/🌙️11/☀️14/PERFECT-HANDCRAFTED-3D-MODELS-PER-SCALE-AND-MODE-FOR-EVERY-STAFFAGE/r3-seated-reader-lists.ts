/** 📋️ The lists of the round 3 seated reader per scale: which body is built, its thermal model (nested layer shells at 1:50 where a garment really lies, the bare skin of the face and of the neck above the roll neck, breath and plume as outlet patches), its structure (the skeleton of the sitter, the timber frame of the bench, notebook and cup) and the decisions.
 * @see ./r3-seated-reader-s50.ts
 * @see ./r3-seated-reader-coarse.ts */
import { Kit, aboutU, aboutW, inset, parts, quat, type Json, type V3 } from "./r3-seated-reader-kit.ts";
import { body100, body200, body500 } from "./r3-seated-reader-coarse.ts";
import { DECISIONS_TEXT } from "./r3-seated-reader-texts.ts";
import { CARRY, FINGERS, FOOT_DROP, body50, fingerPath } from "./r3-seated-reader-s50.ts";
import { PAD } from "./r3-seated-reader-ref.ts";

export const BODY: Record<number, (k: Kit) => void> = { 50: body50, 100: body100, 200: body200, 500: body500 };
export const DECISIONS: Record<number, Json[]> = DECISIONS_TEXT;
export const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const MATERIALS: Json[] = [
  { id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" },
  { id: "rubber-sole", title: { en: "Vulcanised rubber sole and foxing", de: "Vulkanisierte Gummisohle mit Rand" }, emissivity: 0.93, solarAbsorptivity: 0.35, source: "authored-estimate" },
  { id: "steel-zinc", title: { en: "Galvanised steel (bolts, wire comb)", de: "Verzinkter Stahl (Schrauben, Drahtkamm)" }, emissivity: 0.28, solarAbsorptivity: 0.65, source: "authored-estimate" },
];
const REFERENCES: Json[] = [
  { id: "ansur-ii-2014", citation: "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel: Methods and Summary Statistics (ANSUR II). Natick/TR-15/007; female breadths, depths and circumferences." },
  { id: "bs-8300-1-2018", citation: "BS 8300-1:2018 Design of an accessible and inclusive built environment, Part 1: External environment, Code of practice (public seating: seat height about 450-480 mm, backrest at least 300 mm above the seat)." },
  { id: "inclusive-mobility-2021", citation: "Department for Transport (2021). Inclusive Mobility: A Guide to Best Practice on Access to Pedestrian and Transport Infrastructure, section 13 Seating (seat height, seat depth and backrests of public benches)." },
  { id: "timber-bench-catalogues", citation: "Street-furniture catalogues of timber park benches: lengths 1.2, 1.5 and 1.8 m, seat 440-490 mm high and 380-450 mm deep, raked 2-10 degrees, seat-to-back angle 95-105 degrees, 40-50 mm slats of 80-120 mm width with 8-20 mm drainage gaps on end frames set in from the slat ends, carriage bolts through each slat." },
];
/** 🧾️ Root data that follow the model: the materials the solids name and the references of the new dimensions. */
export function finish(asset: Json): void {
  const have = new Set((asset.materials as Json[]).map((m) => m.id));
  asset.materials.push(...MATERIALS.filter((m) => !have.has(m.id)));
  const cited = new Set((asset.provenance.references as Json[]).map((r) => r.id));
  asset.provenance.references.push(...REFERENCES.filter((r) => !cited.has(r.id)));
}

/* ---------------------------------------------------------------- thermal */

const NO_SURFACE = /^(eye|mouth|brow)/;
/** 🧅️ Which layers of its region really lie under a body solid at 1:50 (innermost first); a solid that is not named carries every layer of its region. */
const LAYERS: readonly (readonly [RegExp, string[]])[] = [[/^sweater$/, ["underwear", "tee", "sweater"]], [/^sleeve-/, ["tee", "sweater"]], [/^(sweater-hem|cowl|forearm-|cuff-)/, ["sweater"]], [/^shoe-(sole|toe-sole)-/, ["sneaker"]]];
/** 🌡️ The thermal list of a scale: every body solid that is a surface of its region (texture and face details are no surfaces of their own); at 1:50 the garments and the sneakers as nested layers where each layer lies, the emitters as patches on the surface they leave. */
export function thermal(asset: Json, kit: Kit, scale: number): Json[] {
  const regions = new Map((asset.thermal.regions as Json[]).map((r) => [r.id, r]));
  const out: Json[] = [];
  for (const s of kit.solids) {
    const region = kit.region.get(s.id);
    if (region === undefined) throw new Error(`no region for ${s.id}`);
    if (s.texture === true || NO_SURFACE.test(s.id)) continue;
    if (region === "face-skin" && scale <= 100 && !/^ear-/.test(s.id)) continue;
    const all: Json[] = regions.get(region)!.layers ?? [];
    if (scale !== 50 || all.length === 0) {
      out.push({ id: `th-${s.id}`, of: s.id, region });
      continue;
    }
    const named = LAYERS.find(([re]) => re.test(s.id))?.[1];
    const layers = all.filter((l) => named === undefined || named.includes(l.id));
    let below = 0;
    for (let i = layers.length - 1; i >= 0; i--) {
      const l = layers[i]!;
      out.push(i === layers.length - 1 ? { id: `th-${s.id}-${l.id}`, of: s.id, region, layer: l.id } : { ...inset(s, below, `th-${s.id}-${l.id}`), role: "body", region, layer: l.id });
      below += l.thicknessM;
    }
  }
  if (scale <= 100) {
    const k = new Kit();
    k.ell("th-face", "head", "skin", 0.76, [-0.018, -0.042, 0], [0.086, 0.051, 0.066]);
    k.loft("th-neck", "neck", "skin", 0.72, [[0.04, -0.011, 0, 0.049, 0.052], [0.075, -0.012, 0, 0.047, 0.051], [0.104, -0.01, 0, 0.0475, 0.053]], undefined, { caps: [false, false] });
    out.push(...k.solids.map((s) => ({ ...s, region: "face-skin" })));
  }
  const f = FEATURE[scale]!;
  const k = new Kit();
  const patch = Math.max(f * 0.51, 0.004);
  const coarse = scale >= 200;
  k.ell("th-nose-breath", "head", "skin", 0.5, coarse ? [-0.03, -0.088, 0] : [-0.034, -0.104, 0], [patch, patch, Math.max(0.007, patch)]);
  k.ell("th-mouth-breath", "head", "skin", 0.5, coarse ? [-0.058, -0.08, 0] : [-0.056, -0.094, 0], [patch, patch, Math.max(0.01, patch)]);
  k.ell("th-body-plume", "head", "hair", 0.4, scale === 500 ? [0.13, 0.03, 0] : [0.129, 0.012, 0], [Math.max(0.004, f * 0.5), Math.max(0.028, f * 0.6), Math.max(0.024, f * 0.6)]);
  const emitter: Record<string, [string, string]> = { "th-nose-breath": ["face-skin", "nose-breath"], "th-mouth-breath": ["face-skin", "mouth-breath"], "th-body-plume": ["sweater", "body-plume"] };
  out.push(...k.solids.map((s) => ({ ...s, region: emitter[s.id]![0], tags: ["emitter", emitter[s.id]![1]] })));
  return out;
}

/* ---------------------------------------------------------------- structure */

const B = 0.62;
/** 📏️ The skeleton is stated at the lengths of a 1.78 m frame (the bones of the rig are 0.944 of it) and carried to her stature here; pelvis widths are stated for her. */
const K = 0.944;
const KF = 0.86;
function scaled(s: Json, k: number): Json {
  const m = (x: number): number => Math.round(x * k * 1e5) / 1e5;
  if (s.kind === "loft") for (const q of s.sections) [q.at, q.rx, q.ry] = [q.at.map(m), m(q.rx), m(q.ry)];
  else if (s.kind === "ellipsoid") s.radii = s.radii.map(m);
  else if (s.kind === "revolve") s.profile = s.profile.map((p: number[]) => p.map(m));
  if (s.transform?.translation !== undefined) s.transform.translation = s.transform.translation.map(m);
  return s;
}
/** 🦴️ The structure of a scale: the skeleton stated bone by bone in the rest frames (radii never below half the smallest feature, parts merged as the scale coarsens), the bench frame, slats, notebook and cup by reference to the body solids that are those members, masses at the de Leva centres. */
export function structure(asset: Json, scale: number, body: readonly Json[]): Json[] {
  const f = FEATURE[scale]! / 2 + 0.0002;
  const fine = scale === 50;
  const mid = scale <= 100;
  const out: Json[] = [];
  const member = (bone: string, build: (k: Kit, r: (x: number) => number) => void, part: "member" | "joint" | "mass" = "member", k0 = K): void => {
    const k = new Kit();
    build(k, (x) => Math.max(x, f / k0));
    out.push(...parts(k.solids.map((s) => scaled(s, k0)), bone, part));
  };
  member("pelvis", (k, r) => {
    k.bar("sacrum", "pelvis", "bone", B, [[0.1, 0.035, 0, r(0.028), r(0.02)], [0.05, 0.062, 0, r(0.027), r(0.015)], [0.0, 0.08, 0, r(0.014), r(0.011)], ...(fine ? [[-0.03, 0.082, 0, 0.008, 0.007]] : [])]);
    if (scale <= 200) for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      k.ell(`ilium-${s}`, "pelvis", "bone", B, [0.058, 0.03, side * 0.108], [mid ? 0.064 : 0.066, mid ? 0.056 : 0.05, r(fine ? 0.011 : 0.013)], undefined, { rotation: quat(aboutU(side * -40), aboutW(-10)) });
      if (mid) {
        k.bar(`pubis-${s}`, "pelvis", "bone", B, [[0.004, -0.02, side * 0.084, r(0.012)], [-0.02, -0.048, side * 0.044, r(0.01)], [-0.03, -0.058, side * 0.008, r(0.011)]]);
        k.bar(`ischium-${s}`, "pelvis", "bone", B, [[-0.008, 0.008, side * 0.088, r(0.014)], [-0.05, 0.022, side * 0.074, r(0.013)], [-0.082, 0.018, side * 0.064, r(0.015)], ...(fine ? [[-0.064, -0.028, side * 0.034, 0.009], [-0.038, -0.054, side * 0.01, 0.009]] : [])]);
      } else k.bar(`hip-bar-${s}`, "pelvis", "bone", B, [[0.0, 0.0, side * 0.09, r(0.02)], [-0.07, 0.02, side * 0.064, r(0.02)]]);
    }
    else k.bar("pelvic-bar", "pelvis", "bone", B, [[0.02, 0.02, -0.095, r(0.03)], [0.02, 0.02, 0.095, r(0.03)]]);
  });
  member("torso", (k, r) => {
    const spine = fine ? [[-0.004, 0.0, 0, 0.024], [0.06, -0.022, 0, 0.023], [0.14, -0.02, 0, 0.021], [0.23, 0.004, 0, 0.019], [0.32, 0.024, 0, 0.017], [0.41, 0.022, 0, 0.015], [0.475, 0.008, 0, 0.014], [0.505, 0.0, 0, 0.013]] : mid ? [[0, 0, 0, 0.024], [0.08, -0.022, 0, 0.022], [0.22, 0.002, 0, 0.019], [0.36, 0.024, 0, 0.017], [0.505, 0.0, 0, 0.014]] : [[0, 0, 0, r(0.024)], [0.2, 0.0, 0, r(0.02)], [0.36, 0.02, 0, r(0.018)], [0.505, 0, 0, r(0.016)]];
    k.bar("spine", "torso", "bone", B, spine);
    const ribs: number[][] = fine ? [[0.455, -0.045, 0.046, 0.066], [0.43, -0.052, 0.064, 0.094], [0.4, -0.058, 0.078, 0.11], [0.365, -0.062, 0.088, 0.121], [0.33, -0.064, 0.094, 0.127], [0.295, -0.064, 0.096, 0.13], [0.26, -0.062, 0.095, 0.129], [0.228, -0.058, 0.09, 0.125]] : mid ? [[0.445, -0.05, 0.054, 0.08], [0.395, -0.058, 0.08, 0.112], [0.335, -0.064, 0.094, 0.127], [0.275, -0.063, 0.096, 0.13], [0.225, -0.058, 0.089, 0.124]] : scale === 200 ? [[0.41, -0.056, 0.072, 0.103], [0.32, -0.064, 0.092, 0.125], [0.23, -0.058, 0.086, 0.121]] : [];
    ribs.forEach(([s, cv, d, w], i) => k.ring(`rib-${i + 1}`, "torso", "bone", B, s!, cv!, d!, w!, r(0.0055), -18));
    if (scale === 500) k.loft("rib-cage", "torso", "bone", B, [[0.2, -0.055, 0, 0.1, 0.075], [0.32, -0.062, 0, 0.12, 0.09], [0.44, -0.05, 0, 0.085, 0.06]]);
    if (mid) k.bar("sternum", "torso", "bone", B, [[0.43, -0.118, 0, r(0.019), r(0.0075)], [0.37, -0.14, 0, r(0.016), r(0.0075)], [0.3, -0.147, 0, r(0.012), r(0.0075)]], undefined, { e: 3 });
    for (const side of [1, -1]) {
      const s = side > 0 ? "l" : "r";
      if (mid) {
        k.bar(`clavicle-${s}`, "torso", "bone", B, [[0.446, -0.116, side * 0.022, r(0.0075)], [0.462, -0.103, side * 0.088, r(0.0065)], [0.466, -0.058, side * 0.166, r(0.0075)]]);
        { const h = fine ? 0.072 : 0.068, w = fine ? 0.048 : 0.046, t = r(fine ? 0.007 : 0.009); k.loft(`scapula-${s}`, "torso", "bone", B, [[h * 0.92, 0, 0, w * 0.94, t], [h * 0.6, 0, side * w * 0.04, w, t], [0, 0, side * -w * 0.22, w * 0.74, t], [-h * 0.6, 0, side * -w * 0.52, w * 0.42, t], [-h, 0, side * -w * 0.76, r(w * 0.2), t]], undefined, { translation: [0.37, 0.068, side * 0.098], rotation: quat(aboutU(side * 26), aboutW(4)) }); }
        if (fine) k.bar(`acromion-${s}`, "torso", "bone", B, [[0.43, 0.063, side * 0.108, 0.007], [0.46, 0.02, side * 0.156, 0.007], [0.466, -0.03, side * 0.18, 0.008]]);
      } else if (scale === 200) k.bar(`shoulder-bar-${s}`, "torso", "bone", B, [[0.45, -0.06, side * 0.02, r(0.012)], [0.453, -0.045, side * 0.172, r(0.012)]]);
    }
    if (scale === 500) k.bar("shoulder-bar", "torso", "bone", B, [[0.445, -0.04, -0.15, r(0.02)], [0.445, -0.04, 0.15, r(0.02)]]);
  });
  member("neck", (k, r) => k.bar("cervical-spine", "neck", "bone", B, mid ? [[0.0, 0.0, 0, r(0.013)], [0.06, -0.012, 0, r(0.012)], [0.118, -0.004, 0, r(0.012)]] : [[0, 0, 0, r(0.014)], [0.118, -0.004, 0, r(0.014)]]));
  member("head", (k, r) => {
    k.ell("cranium", "head", "bone", B, [0.045, scale === 500 ? -0.01 : 0.004, 0], scale === 500 ? [0.078, 0.088, 0.068] : mid ? [0.08, 0.09, 0.068] : [0.078, 0.088, 0.066]);
    if (scale <= 200) k.loft("face-bones", "head", "bone", B - 0.06, mid ? [[-0.05, -0.06, 0, 0.034, 0.03], [-0.02, -0.066, 0, 0.054, 0.032], [0.02, -0.072, 0, 0.058, 0.026]] : [[-0.05, -0.058, 0, 0.04, 0.034], [0.02, -0.068, 0, 0.056, 0.034]]);
    if (mid) k.bar("mandible", "head", "bone", B, [[-0.005, 0.01, 0.052, r(0.008)], [-0.05, 0.004, 0.047, r(0.008)], [-0.08, -0.045, 0.026, r(0.008)], [-0.092, -0.068, 0, r(0.009)], [-0.08, -0.045, -0.026, r(0.008)], [-0.05, 0.004, -0.047, r(0.008)], [-0.005, 0.01, -0.052, r(0.008)]]);
  });
  for (const [s, side] of [["l", 1], ["r", -1]] as const) {
    const t = -side;
    member(`arm-${s}`, (k, r) => {
      if (mid) k.ell(`humerus-head-${s}`, `arm-${s}`, "bone", B, [0.004, -0.004, 0], [0.023, 0.023, 0.023]);
      k.bar(`humerus-${s}`, `arm-${s}`, "bone", B, [[0.01, 0, 0, r(0.012)], [0.16, 0.002, 0, r(0.0105)], [0.31, 0, 0, r(0.011)]]);
      if (mid) k.ell(`humerus-condyles-${s}`, `arm-${s}`, "bone", B, [0.318, 0, 0], [r(0.0135), r(0.0135), 0.03]);
    });
    member(`forearm-${s}`, (k, r) => {
      if (mid) {
        k.bar(`radius-${s}`, `forearm-${s}`, "bone", B, [[0.012, 0.004, side * 0.017, r(0.008)], [0.13, 0.002, side * 0.019, r(0.0075)], [0.25, 0, side * 0.013, r(0.0105)]]);
        k.bar(`ulna-${s}`, `forearm-${s}`, "bone", B, [[-0.022, -0.012, side * -0.008, r(0.0105)], [0.02, -0.004, side * -0.013, r(0.01)], [0.248, 0, side * -0.017, r(0.0075)]]);
      } else k.bar(`forearm-bones-${s}`, `forearm-${s}`, "bone", B, [[0, 0, 0, r(0.014)], [0.25, 0, 0, r(0.012)]]);
    });
    member(`hand-${s}`, (k, r) => {
      if (scale === 500) k.bar(`hand-bones-${s}`, `hand-${s}`, "bone", B, [[0, 0.0, 0, r(0.02)], [0.1, -0.004, 0, r(0.02)]]);
      else if (fine) {
        k.ell(`carpals-${s}`, `hand-${s}`, "bone", B, [0.014, 0, 0], [0.015, 0.009, 0.025]);
        FINGERS.forEach(([w, sc], i) => {
          k.bar(`metacarpal-${i + 1}-${s}`, `hand-${s}`, "bone", B, [[0.028, 0, t * w * 0.55, 0.0042], [0.084, -0.002, t * w, 0.0042]]);
          k.bar(`phalanges-${i + 1}-${s}`, `hand-${s}`, "bone", B, fingerPath(t * w, sc, 0.0041, CARRY.flex[i] as [number, number, number]).slice(1).map((p) => [p[0]!, p[1]!, p[2]!, 0.004]));
        });
        k.bar(`thumb-bones-${s}`, `hand-${s}`, "bone", B, CARRY.thumb.map(([u, p, w]) => [u, p, t * w, 0.0042]));
      } else k.loft(`hand-bones-${s}`, `hand-${s}`, "bone", B, scale === 100 ? [[0.004, 0, 0, 0.022, r(0.008)], [0.05, -0.002, 0, 0.03, r(0.008)], [0.15, -0.003, 0, 0.03, r(0.008)]] : [[0.004, -0.002, 0, 0.024, r(0.01)], [0.15, -0.003, 0, 0.028, r(0.01)]], undefined, { e: 3 });
    }, "member", 1);
    member(`thigh-${s}`, (k, r) => {
      if (mid) k.ell(`femur-head-${s}`, `thigh-${s}`, "bone", B, [0, 0, 0], [0.024, 0.024, 0.024]);
      if (mid) k.bar(`femur-neck-${s}`, `thigh-${s}`, "bone", B, [[0, 0, 0, r(0.016)], [0.016, -0.004, side * 0.04, r(0.014)], [0.026, -0.008, side * 0.066, r(0.02)]]);
      k.bar(`femur-${s}`, `thigh-${s}`, "bone", B, mid ? [[0.022, -0.008, side * 0.064, r(0.017)], [0.11, -0.002, side * 0.048, r(0.0145)], [0.3, 0.004, side * 0.018, r(0.0145)], [0.41, 0, 0, r(0.02)]] : [[0, 0, side * 0.03, r(0.018)], [0.2, 0.004, side * 0.02, r(0.016)], [0.42, 0, 0, r(0.02)]]);
      if (mid) k.ell(`femur-condyles-${s}`, `thigh-${s}`, "bone", B, [0.422, -0.006, 0], [0.03, 0.033, 0.04]);
      if (mid) k.ell(`patella-${s}`, `thigh-${s}`, "bone", B, [0.462, 0.012, 0], [r(0.011), 0.021, 0.022]);
    });
    member(`shank-${s}`, (k, r) => {
      if (mid) k.ell(`tibia-plateau-${s}`, `shank-${s}`, "bone", B, [0.018, 0.002, 0], [0.02, 0.03, 0.038]);
      k.bar(`tibia-${s}`, `shank-${s}`, "bone", B, mid ? [[0.02, 0.01, side * -0.004, r(0.019)], [0.12, 0.014, side * -0.006, r(0.015)], [0.31, 0.01, side * -0.004, r(0.0125)], [0.43, 0.002, side * -0.006, r(0.017)]] : [[0, 0.006, 0, r(0.02)], [0.2, 0.012, 0, r(0.016)], [0.435, 0.002, 0, r(0.018)]]);
      if (mid) k.bar(`fibula-${s}`, `shank-${s}`, "bone", B, [[0.03, -0.014, side * 0.034, r(0.0085)], [0.22, -0.012, side * 0.032, r(0.006)], [0.437, -0.008, side * 0.03, r(0.0095)]]);
    });
    const level = { rotation: aboutW(FOOT_DROP) };
    member(`foot-${s}`, (k, r) => {
      if (mid) k.ell(`calcaneus-${s}`, `foot-${s}`, "bone", B, [-0.018, -0.05, 0], [0.04, 0.022, 0.018], undefined, level);
      k.loft(`tarsals-${s}`, `foot-${s}`, "bone", B, mid ? [[-0.004, -0.02, 0, 0.022, 0.018], [0.06, -0.044, 0, 0.03, 0.012], [0.145, -0.07, 0, 0.038, r(0.0085)]] : scale === 200 ? [[-0.05, -0.05, 0, 0.022, 0.02], [0.03, -0.04, 0, 0.03, 0.018], [0.145, -0.064, 0, 0.034, r(0.012)]] : [[-0.04, -0.046, 0, r(0.038), r(0.03)], [0.14, -0.046, 0, r(0.038), r(0.03)]], undefined, level);
    }, "member", KF);
    if (scale <= 200) member(`toe-${s}`, (k, r) => k.loft(`toe-bones-${s}`, `toe-${s}`, "bone", B, mid ? [[-0.004, 0.024, 0, 0.038, r(0.007)], [0.035, 0.022, 0, 0.034, r(0.0065)], [0.066, 0.02, 0, 0.026, r(0.006)]] : [[0.0, 0.026, 0, 0.034, r(0.01)], [0.068, 0.022, 0, 0.026, r(0.01)]], undefined, { e: 3 }), "member", KF);
  }
  const joints: [string, number, V3?][] = [["torso", 0.017], ["neck", 0.013], ["head", 0.012], ["arm-l", 0.022], ["arm-r", 0.022], ["forearm-l", 0.0155], ["forearm-r", 0.0155], ["hand-l", 0.012], ["hand-r", 0.012], ["thigh-l", 0.023], ["thigh-r", 0.023], ["shank-l", 0.019], ["shank-r", 0.019], ["foot-l", 0.0155], ["foot-r", 0.0155], ["toe-l", 0.009, [0, 0.019, 0]], ["toe-r", 0.009, [0, 0.019, 0]]];
  for (const [bone, radius, at] of joints) {
    if (scale >= 200 && /^(toe|hand)/.test(bone)) continue;
    if (scale === 500 && /^(head|neck|foot)/.test(bone)) continue;
    member(bone, (k, r) => k.ell(`joint-${bone}`, bone, "bone", 0.4, at ?? [0, 0, 0], [r(radius) * 1.04, r(radius) * 1.04, r(radius) * 1.04]), "joint", 1);
  }
  for (const bone of asset.rig.bones as Json[]) {
    const m = bone.mass;
    if (m === undefined || m.kg < 0.01) continue;
    if (scale === 100 && m.kg < 1) continue;
    if (scale === 200 && m.kg < 3) continue;
    if (scale === 500 && m.kg < 6) continue;
    const radius = Math.max(0.0125 * Math.cbrt(m.kg), f);
    member(bone.id, (k) => k.ell(`mass-${bone.id}`, bone.id, "bone", 0.2, [m.comX, m.comY ?? 0, 0], [radius, radius, radius], undefined, { role: "detail" }), "mass", 1);
  }
  const memberOf: readonly (readonly [RegExp, string | null])[] = [[/^bench-(post|leg|rail|stretcher)-/, null], [/^bench-tie$/, "frame-l"], [/^bench-seat/, "seat"], [/^bench-back/, "backrest"], [/^pad-board$/, "pad"], [/^cup-body$/, "cup"]];
  for (const s of body) {
    const hit = memberOf.find(([re]) => re.test(s.id));
    if (hit === undefined || /^bench-bolt/.test(s.id)) continue;
    out.push({ id: `st-${s.id}`, of: s.id, member: hit[1] ?? s.bone, part: "member" });
  }
  member("page", (k) => k.box("page-leaf", "page", "paper", B + 0.2, [PAD.page / 2 + 0.004, PAD.thick / 2 + 0.0008, 0], [PAD.page - 0.008, 0.0008, PAD.width - 0.006], 0.0002, undefined, { texture: true, role: "texture" }), "member", 1);
  return out;
}
