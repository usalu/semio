/** 🌡️ Thermal model and load path of the runner with blade prosthesis at every scale, and the bilingual decisions. Thermal: every region as the surface it names, nested shells at 1:50 (shoe: sock under upper; socket: gel liner under carbon shell), outlets as patches on nose, lips and crown. Structure: the anatomical skeleton down to the tibial stump, then socket shell, pyramid adapter and carbon blade as the prosthetic load path.
 * @see ./r3-runner-figure.ts */
import { RIG } from "./r3-runner-rig.ts";
import { Kit, inset, parts, type Json, type Row, type V3 } from "./r3-runner-kit.ts";
import { ADAPTER_Y, FOOT, M, SIDES, T, TOE, m } from "./r5-runner-with-blade-prosthesis-body50.ts";

/* ------------------------------------------------------------------ thermal */

const FACE: Record<number, Row[]> = {
  50: [[-0.088, -0.084, 0, 0.02, 0.016, 2.2], [-0.068, -0.068, 0, 0.044, 0.04, 2.2], [-0.041, -0.048, 0, 0.058, 0.062, 2.2], [-0.011, -0.03, 0, 0.068, 0.081], [0.029, -0.04, 0, 0.073, 0.071], [0.069, -0.072, 0, 0.068, 0.04], [0.1, -0.088, 0, 0.05, 0.012]],
  100: [[-0.088, -0.086, 0, 0.022, 0.018, 2.2], [-0.055, -0.058, 0, 0.052, 0.054, 2.2], [-0.012, -0.034, 0, 0.068, 0.084], [0.03, -0.042, 0, 0.073, 0.072], [0.1, -0.09, 0, 0.05, 0.014]],
  200: [[-0.088, -0.08, 0, 0.026, 0.024, 2.2], [-0.05, -0.052, 0, 0.056, 0.06, 2.2], [0.0, -0.034, 0, 0.071, 0.089], [0.1, -0.09, 0, 0.05, 0.016]],
  500: [[-0.085, -0.03, 0, 0.056, 0.07], [-0.02, -0.034, 0, 0.07, 0.092], [0.1, -0.085, 0, 0.05, 0.026]],
};
export function thermal(asset: Json, k: Kit, scale: number): Json[] {
  const out: Json[] = [];
  const nested = scale === 50;
  const t = new Kit();
  const ref = (s: Json, region: string, layer?: string): Json => ({ id: `th-${s.id}`, of: s.id, region, ...(nested && layer !== undefined ? { layer } : {}) });
  for (const s of k.solids) {
    const region = k.region.get(s.id)!;
    if (s.id === "head") continue;
    if (region === "running-shoe") {
      if (s.material === M.sock) out.push(ref(s, region, "socks"));
      else {
        out.push(ref(s, region, "shoe"));
        if (nested && /^shoe/.test(s.id)) out.push({ ...inset(s, 0.006, `th-${s.id}-socks`), region, layer: "socks" });
      }
    } else if (region === "socket") {
      if (s.material === M.liner) out.push(ref(s, region, "liner"));
      else {
        out.push(ref(s, region, "shell"));
        if (nested && s.id === "socket") out.push({ ...inset(s, 0.004, "th-socket-liner"), material: M.liner, tone: T.liner, region, layer: "liner" });
      }
    } else if (region === "singlet" || region === "shorts" || region === "hair") out.push(ref(s, region, region));
    else out.push(ref(s, region));
  }
  out.push({ ...t.loft("th-face", "head", M.skin, T.skin, FACE[scale]!), region: "face-skin" });
  if (scale === 500) {
    out.splice(out.findIndex((x) => x.id === "th-singlet"), 1);
    out.push({ ...t.loft("th-singlet", "torso", M.singlet, T.singlet, [[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [0.09, -0.005, 0, 0.143, 0.097, 2.3], [0.33, -0.002, 0, 0.17, 0.122, 2.3], [0.44, 0.002, 0, 0.186, 0.104, 2.2], [0.475, 0.004, 0, 0.142, 0.08, 2.1]]), region: "singlet" });
    out.push({ ...t.loft("th-neck", "head", M.skin, T.skin, [[-0.135, 0.016, 0, 0.064, 0.064], [-0.07, -0.004, 0, 0.056, 0.062]]), region: "face-skin" });
    out.splice(out.findIndex((x) => x.id === "th-thigh-r"), 1);
    out.push({ ...t.loft("th-thigh-r", "thigh-r", M.skin, T.skin, [[0.117, 0.0095, -0.001, 0.0815, 0.0925], [0.2, 0.011, 0, 0.078, 0.088], [0.35, 0.009, 0, 0.064, 0.072]]), region: "leg-skin" });
    out.push({ ...t.loft("th-liner-thigh", "thigh-r", M.liner, T.liner, [[0.35, 0.009, 0, 0.064, 0.072], [0.435, 0.007, 0, 0.056, 0.063], [0.47, 0.004, 0, 0.04, 0.042]]), region: "socket" });
  }
  const h = Math.max(0.004, (0.0077 * scale) / 100);
  const e = (id: string, at: V3, radii: V3, region: string, emitter: string, material: string): Json => ({ ...t.ell(id, "head", material, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(e("th-nose-breath", [-0.012, -0.121 + h, 0], [h, h, Math.max(h, 0.008)], "face-skin", "nose-breath", M.skin));
  out.push(e("th-mouth-breath", [-0.042, -0.1105 + h, 0], [h, h, Math.max(h, 0.0105)], "face-skin", "mouth-breath", M.skin));
  out.push(e("th-plume", [0.1595 - h, -0.008, 0], [h, Math.max(0.05, h), Math.max(0.042, h)], "singlet", "body-plume", M.hair));
  return out;
}

/* ------------------------------------------------------------------ structure */

const B = M.skin;
const C = M.socket;
function masses(asset: Json, least = 0): Json[] {
  const k = new Kit();
  const out: Json[] = [];
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < 0.1 * asset.dimensions.massKg) continue;
    const r = Math.max(0.011 * Math.cbrt(b.mass.kg), least);
    out.push(...parts([k.ell(`mass-${b.id}`, b.id, B, 0.4, [b.mass.comX, b.id === "torso" || b.id === "pelvis" ? 0.005 : (b.mass.comY ?? 0), 0], [r, r, r])], b.id, "mass"));
  }
  return out;
}
/** 🔩️ Joint solids at the real joints: discs between the vertebrae at L5/S1, C7/T1 and under the skull, balls at hip and shoulder, hinge pins through knee, elbow, wrist, ankle and toe joints, the bolted clamp at the adapter and the elastic bend of the blade; `f` enlarges them for a coarser scale. */
const JOINT: Record<string, [at: V3, radii: V3]> = { torso: [[0, 0.04, 0], [0.007, 0.022, 0.029]], neck: [[0, 0.03, 0], [0.006, 0.017, 0.02]], head: [[0, 0.012, 0], [0.006, 0.016, 0.024]], thigh: [[0, 0, 0], [0.027, 0.027, 0.027]], shank: [[0, 0, 0], [0.014, 0.014, 0.046]], foot: [[0, 0, 0], [0.011, 0.011, 0.031]], toe: [[0, 0.012, 0], [0.008, 0.008, 0.044]], arm: [[0, 0, 0], [0.026, 0.026, 0.026]], forearm: [[0, 0, 0], [0.01, 0.01, 0.037]], hand: [[0, 0, 0], [0.009, 0.026, 0.013]], blade: [[0, -0.004, 0], [0.01, 0.01, 0.026]], "blade-tip": [[0, 0, 0], [0.0095, 0.0095, 0.028]] };
function joints(ids: readonly string[], f: number): Json[] {
  const k = new Kit();
  return ids.flatMap((id) => {
    const [at, r] = JOINT[id.replace(/-[lr]$/, "")]!;
    return parts([k.ell(`joint-${id}`, id, id.startsWith("blade") ? C : B, 0.5, at, [r[0] * f, r[1] * f, Math.max(r[2], r[2] * (1 + (f - 1) / 3))])], id, "joint");
  });
}
const ALL_JOINTS = ["torso", "neck", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "toe-l", "arm-l", "arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r", "blade-r", "blade-tip-r"];
/** 🐍️ The vertebral column of the trunk bone from L5/S1 to C7/T1 with its real S-curve (round 4): lumbar lordosis with the apex at L3, 77 mm in front of the skin of the back, thoracic kyphosis with the apex at T6, 43 mm under it; the chord from S1 to C7 is crossed at T11. */
const SPINE: Row[] = [[-0.01, 0.044, 0, 0.026, 0.019], [0.04, 0.028, 0, 0.026, 0.019], [0.09, 0.02, 0, 0.025, 0.018], [0.14, 0.024, 0, 0.024, 0.018], [0.2, 0.04, 0, 0.022, 0.017], [0.27, 0.058, 0, 0.02, 0.016], [0.33, 0.066, 0, 0.018, 0.016], [0.4, 0.06, 0, 0.017, 0.015], [0.46, 0.046, 0, 0.016, 0.014], [0.515, 0.032, 0, 0.015, 0.013]];
/** 🦷️ The spinous processes of the seventeen vertebrae from L5 to T1: stations `[s, cv of the back of the vertebral body, length]`, each a short spur that points back and down (steeper and longer in the thoracic column, where they overlap like roof tiles) and ends under the skin of the back. */
const SPINOUS: readonly (readonly [number, number, number])[] = [[0.012, 0.056, 0.03], [0.044, 0.045, 0.031], [0.076, 0.039, 0.032], [0.108, 0.039, 0.032], [0.14, 0.042, 0.031], [0.17, 0.049, 0.03], [0.198, 0.056, 0.03], [0.225, 0.063, 0.03], [0.251, 0.069, 0.031], [0.276, 0.075, 0.031], [0.3, 0.079, 0.031], [0.324, 0.081, 0.03], [0.348, 0.081, 0.029], [0.372, 0.079, 0.027], [0.396, 0.075, 0.025], [0.42, 0.069, 0.024], [0.446, 0.062, 0.024]];
const put = (out: Json[], member: string, f: (k: Kit) => void): void => {
  const k = new Kit();
  f(k);
  out.push(...parts(k.solids, member, "member"));
};
const of = (id: string, member: string): Json => ({ id: `st-${id}`, of: id, member, part: "member" });

function structure50(asset: Json): Json[] {
  const out: Json[] = [];
  put(out, "head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.066, -0.01, 0], [0.076, 0.09, 0.07]);
    k.loft("sk-face", "head", B, T.bone, [[-0.084, -0.078, 0, 0.018, 0.014], [-0.061, -0.064, 0, 0.04, 0.036], [-0.031, -0.054, 0, 0.05, 0.048], [0.004, -0.054, 0, 0.058, 0.048], [0.032, -0.054, 0, 0.06, 0.046]]);
  });
  put(out, "neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, [[-0.012, 0.03, 0, 0.016, 0.014], [0.03, 0.02, 0, 0.015, 0.013], [0.07, 0.014, 0, 0.014, 0.013], [0.108, 0.014, 0, 0.017, 0.014]]));
  put(out, "torso", (k) => {
    k.loft("sk-spine", "torso", B, T.bone, SPINE);
    SPINOUS.forEach(([s, cv, len], i) => k.cap(`sk-spinous-${i + 1}`, "torso", B, T.bone, [s, cv - 0.004, 0], [s - (i < 5 ? 0.006 : 0.02), cv + (i < 5 ? len : len * 0.72), 0], 0.0055, 0.004));
    for (const [i, s, cv, d, w] of [[1, 0.455, 0.012, 0.055, 0.075], [2, 0.415, 0.007, 0.072, 0.102], [3, 0.375, 0.004, 0.086, 0.118], [4, 0.335, 0.002, 0.095, 0.128], [5, 0.295, 0, 0.099, 0.133], [6, 0.255, 0, 0.099, 0.134], [7, 0.215, 0.002, 0.094, 0.131], [8, 0.175, 0.006, 0.086, 0.124]] as const) k.ring(`sk-rib-${i}`, "torso", B, T.bone, s, cv, d, w, 0.0065, -14);
    k.cap("sk-sternum", "torso", B, T.bone, [0.26, -0.092, 0], [0.44, -0.064, 0], 0.012, 0.014);
    for (const [sfx, g] of SIDES) {
      k.cap(`sk-clavicle-${sfx}`, "torso", B, T.bone, [0.45, -0.056, 0.022 * g], [0.462, 0.004, 0.168 * g], 0.008, 0.0075);
      k.ell(`sk-scapula-${sfx}`, "torso", B, T.bone, [0.36, 0.078, 0.098 * g], [0.078, 0.008, 0.052], undefined, "body", [0, 0, 0.0872, 0.9962]);
      k.cap(`sk-acromion-${sfx}`, "torso", B, T.bone, [0.43, 0.068, 0.07 * g], [0.464, 0.016, 0.172 * g], 0.0065, 0.008);
    }
  });
  put(out, "pelvis", (k) => {
    k.loft("sk-sacrum", "pelvis", B, T.bone, [[0.105, 0.04, 0, 0.03, 0.02], [0.06, 0.058, 0, 0.05, 0.018], [0.01, 0.07, 0, 0.03, 0.014], [-0.03, 0.066, 0, 0.012, 0.01]]);
    for (const [sfx, g] of SIDES) {
      k.ell(`sk-ilium-${sfx}`, "pelvis", B, T.bone, [0.066, 0.012, 0.086 * g], [0.054, 0.074, 0.011], undefined, "body", [-0.3007 * g, 0, 0, 0.9537]);
      k.cap(`sk-iliac-crest-${sfx}`, "pelvis", B, T.bone, [0.114, -0.048, 0.128 * g], [0.114, 0.066, 0.046 * g], 0.009, 0.009);
      k.loft(`sk-ischium-${sfx}`, "pelvis", B, T.bone, m([[0.03, 0.008, 0.098, 0.02, 0.04], [0.0, 0.006, 0.094, 0.026, 0.036], [-0.035, 0.022, 0.072, 0.014, 0.028], [-0.066, 0.032, 0.055, 0.013, 0.02]], g));
      k.cap(`sk-pubis-${sfx}`, "pelvis", B, T.bone, [-0.015, -0.04, 0.062 * g], [-0.03, -0.07, 0.004 * g], 0.011, 0.01);
    }
  });
  for (const [sfx, g] of SIDES) {
    put(out, `thigh-${sfx}`, (k) => {
      k.ell(`sk-femur-head-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.024, 0.024, 0.024]);
      k.cap(`sk-femur-neck-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.032, 0, 0.048 * g], 0.015, 0.014);
      k.ell(`sk-trochanter-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.03, -0.004, 0.055 * g], [0.03, 0.02, 0.014]);
      k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[0.04, 0, 0.05, 0.016, 0.016], [0.12, 0.006, 0.04, 0.0145, 0.0145], [0.245, 0.01, 0.022, 0.014, 0.014], [0.35, 0.006, 0.008, 0.016, 0.015], [0.4, 0.002, 0.002, 0.028, 0.022], [0.435, 0, 0, 0.041, 0.032], [0.452, 0, 0, 0.034, 0.024]], g));
      k.ell(`sk-patella-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.426, 0.044, 0], [0.022, 0.01, 0.02]);
    });
    put(out, `arm-${sfx}`, (k) => {
      k.ell(`sk-humerus-head-${sfx}`, `arm-${sfx}`, B, T.bone, [0, 0, -0.003 * g], [0.023, 0.023, 0.023]);
      k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, [[0.015, 0, 0, 0.017, 0.017], [0.08, 0.002, 0, 0.012, 0.012], [0.25, 0, 0, 0.0105, 0.0105], [0.305, 0, 0, 0.02, 0.011], [0.33, 0, 0, 0.032, 0.014], [0.342, 0, 0, 0.026, 0.012]]);
    });
    put(out, `forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.254, -0.016, -0.002 * g], 0.011, 0.006);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.254, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(out, `hand-${sfx}`, (k) => {
      k.ell(`sk-carpus-${sfx}`, `hand-${sfx}`, B, T.bone, [0.012, 0.003, 0], [0.017, 0.024, 0.01]);
      k.loft(`sk-metacarpus-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.027, 0.004, -0.002, 0.007, 0.026], [0.093, 0.002, -0.005, 0.006, 0.035]], g), undefined, "body", 3);
      for (const [i, kv, ku, len] of [[1, 0.03, 0.094, 1], [2, 0.01, 0.097, 1.08], [3, -0.01, 0.094, 1], [4, -0.028, 0.087, 0.8]] as const) {
        const p1: V3 = [ku + 0.036 * len, kv, -0.02 * g];
        k.cap(`sk-phalanx-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, [ku, kv, -0.006 * g], p1, 0.0045, 0.004);
        k.cap(`sk-phalanx-tip-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p1, [ku + 0.026 * len, kv - 0.001, -0.05 * len * g - 0.004 * g], 0.004, 0.004);
      }
      k.cap(`sk-thumb-${sfx}`, `hand-${sfx}`, B, T.bone, [0.022, 0.03, -0.006 * g], [0.067, 0.048, -0.018 * g], 0.005, 0.0045);
      k.cap(`sk-thumb-tip-${sfx}`, `hand-${sfx}`, B, T.bone, [0.067, 0.048, -0.018 * g], [0.098, 0.04, -0.03 * g], 0.0045, 0.004);
    });
  }
  put(out, "shank-l", (k) => {
    k.loft("sk-tibia-l", "shank-l", B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.02, 0.004, -0.002, 0.03, 0.026], [0.07, 0.01, -0.004, 0.017, 0.019], [0.2, 0.01, -0.004, 0.013, 0.015], [0.35, 0.004, -0.004, 0.013, 0.013], [0.405, 0, -0.004, 0.02, 0.017], [0.43, 0, -0.008, 0.021, 0.016]]);
    k.cap("sk-fibula-l", "shank-l", B, T.bone, [0.035, -0.012, 0.034], [0.432, -0.008, 0.0255], 0.0075, 0.0065);
  });
  put(out, "foot-l", (k) => {
    k.loftPlane("sk-talus-l", "foot-l", FOOT, B, T.bone, [[-0.022, 0.062, 0, 0.018, 0.014], [0.005, 0.062, 0, 0.021, 0.017], [0.03, 0.056, 0, 0.018, 0.013]]);
    k.loftPlane("sk-calcaneus-l", "foot-l", FOOT, B, T.bone, [[-0.062, 0.04, 0.003, 0.012, 0.012], [-0.04, 0.042, 0.003, 0.016, 0.018], [-0.01, 0.046, 0.002, 0.016, 0.014], [0.015, 0.046, 0.002, 0.014, 0.01]]);
    k.loftPlane("sk-metatarsus-l", "foot-l", FOOT, B, T.bone, [[0.02, 0.054, 0, 0.024, 0.014], [0.07, 0.044, 0, 0.032, 0.011], [0.115, 0.034, 0, 0.04, 0.008], [0.14, 0.03, 0, 0.042, 0.008]], undefined, "body", 2.6);
  });
  put(out, "toe-l", (k) => k.loftPlane("sk-phalanges-l", "toe-l", TOE, B, T.bone, [[0.135, 0.03, 0, 0.04, 0.008], [0.17, 0.029, -0.004, 0.034, 0.007], [0.205, 0.028, -0.007, 0.02, 0.006]], undefined, "body", 2.6));
  put(out, "shank-r", (k) => {
    k.loft("sk-tibia-r", "shank-r", B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.02, 0.004, 0.002, 0.03, 0.026], [0.07, 0.01, 0.004, 0.017, 0.019], [0.12, 0.011, 0.004, 0.0145, 0.016], [0.133, 0.006, 0.004, 0.011, 0.01, 2, -25]]);
    k.cap("sk-fibula-r", "shank-r", B, T.bone, [0.035, -0.012, -0.034], [0.118, -0.011, -0.03], 0.0075, 0.0065);
  });
  out.push(of("socket", "shank-r"), of("socket-adapter", "shank-r"), of("socket-pyramid", "shank-r"), of("blade-upper", "blade-r"), of("blade-curl", "blade-tip-r"), of("blade-toe", "blade-tip-r"));
  out.push(...joints(ALL_JOINTS, 1), ...masses(asset));
  return out;
}

function structure100(asset: Json): Json[] {
  const out: Json[] = [];
  put(out, "head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.064, -0.01, 0], [0.078, 0.09, 0.07]);
    k.loft("sk-face", "head", B, T.bone, [[-0.084, -0.076, 0, 0.02, 0.016], [-0.046, -0.058, 0, 0.046, 0.042], [0.029, -0.054, 0, 0.06, 0.046]]);
  });
  put(out, "neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, [[-0.012, 0.03, 0, 0.016, 0.015], [0.05, 0.016, 0, 0.015, 0.015], [0.108, 0.014, 0, 0.017, 0.015]]));
  put(out, "torso", (k) => {
    k.loft("sk-spine", "torso", B, T.bone, [SPINE[0]!, SPINE[2]!, SPINE[4]!, SPINE[6]!, SPINE[9]!]);
    for (const [i, s, cv, d, w] of [[1, 0.44, 0.01, 0.062, 0.086], [2, 0.36, 0.003, 0.09, 0.122], [3, 0.275, 0, 0.099, 0.133], [4, 0.19, 0.004, 0.09, 0.128]] as const) k.ring(`sk-rib-${i}`, "torso", B, T.bone, s, cv, d, w, 0.009, -14);
    k.cap("sk-sternum", "torso", B, T.bone, [0.26, -0.09, 0], [0.44, -0.062, 0], 0.014, 0.015);
    for (const [sfx, g] of SIDES) {
      k.cap(`sk-clavicle-${sfx}`, "torso", B, T.bone, [0.45, -0.054, 0.022 * g], [0.462, 0.006, 0.17 * g], 0.009, 0.009);
      k.ell(`sk-scapula-${sfx}`, "torso", B, T.bone, [0.365, 0.076, 0.098 * g], [0.08, 0.009, 0.054]);
    }
  });
  put(out, "pelvis", (k) => {
    k.loft("sk-sacrum", "pelvis", B, T.bone, [[0.105, 0.04, 0, 0.03, 0.02], [0.05, 0.06, 0, 0.048, 0.018], [-0.03, 0.066, 0, 0.014, 0.012]]);
    for (const [sfx, g] of SIDES) k.loft(`sk-hip-bone-${sfx}`, "pelvis", B, T.bone, m([[0.118, 0.018, 0.122, 0.012, 0.062], [0.085, 0.02, 0.118, 0.013, 0.066], [0.0, 0.006, 0.096, 0.024, 0.036], [-0.064, 0.032, 0.055, 0.014, 0.022]], g));
    k.cap("sk-pubis", "pelvis", B, T.bone, [-0.02, -0.06, -0.055], [-0.02, -0.06, 0.055], 0.012, 0.012);
  });
  for (const [sfx, g] of SIDES) {
    put(out, `thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.022, 0, 0, 0.014, 0.014], [0, 0, 0.004, 0.024, 0.024], [0.03, 0, 0.038, 0.024, 0.02], [0.075, 0.004, 0.046, 0.016, 0.016], [0.245, 0.01, 0.022, 0.0145, 0.0145], [0.39, 0.003, 0.003, 0.024, 0.02], [0.435, 0, 0, 0.041, 0.032], [0.452, 0, 0, 0.034, 0.024]], g)));
    put(out, `arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, [[-0.02, 0, 0, 0.013, 0.013], [0, 0, 0, 0.023, 0.023], [0.035, 0, 0, 0.016, 0.016], [0.25, 0, 0, 0.011, 0.011], [0.33, 0, 0, 0.032, 0.014], [0.342, 0, 0, 0.026, 0.012]]));
    put(out, `forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.254, -0.016, -0.002 * g], 0.011, 0.0075);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.254, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(out, `hand-${sfx}`, (k) => {
      k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.003, 0, 0.011, 0.022], [0.03, 0.004, -0.002, 0.008, 0.026], [0.095, 0.002, -0.005, 0.0075, 0.035]], g), undefined, "body", 3);
      k.loft(`sk-fingers-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.095, 0.002, -0.008, 0.0075, 0.034], [0.12, 0.001, -0.02, 0.0075, 0.032], [0.13, 0, -0.032, 0.0075, 0.028]], g), undefined, "body", 3);
    });
  }
  put(out, "shank-l", (k) => {
    k.loft("sk-tibia-l", "shank-l", B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.05, 0.008, -0.003, 0.02, 0.021], [0.29, 0.006, -0.004, 0.0135, 0.014], [0.43, 0, -0.007, 0.022, 0.017]]);
    k.cap("sk-fibula-l", "shank-l", B, T.bone, [0.035, -0.012, 0.033], [0.432, -0.008, 0.025], 0.008, 0.0075);
  });
  put(out, "foot-l", (k) => k.loftPlane("sk-foot-l", "foot-l", FOOT, B, T.bone, [[-0.062, 0.04, 0, 0.012, 0.012], [-0.035, 0.046, 0, 0.02, 0.02], [0.005, 0.058, 0, 0.024, 0.02], [0.07, 0.044, 0, 0.032, 0.011], [0.14, 0.03, 0, 0.042, 0.008]], undefined, "body", 2.6));
  put(out, "toe-l", (k) => k.loftPlane("sk-phalanges-l", "toe-l", TOE, B, T.bone, [[0.135, 0.03, 0, 0.04, 0.008], [0.205, 0.028, -0.007, 0.022, 0.0075]], undefined, "body", 2.6));
  put(out, "shank-r", (k) => k.loft("sk-tibia-r", "shank-r", B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.05, 0.008, 0.003, 0.02, 0.021], [0.13, 0.008, 0.004, 0.014, 0.014]]));
  out.push(of("socket", "shank-r"), of("blade-upper", "blade-r"), of("blade-curl", "blade-tip-r"));
  out.push(...joints(ALL_JOINTS, 1.25), ...masses(asset));
  return out;
}

function structureCoarse(asset: Json, scale: 200 | 500): Json[] {
  const out: Json[] = [];
  const c = scale === 500;
  const r = c ? 0.024 : 0.017;
  const [rf, rh] = [0.038, 0.029];
  put(out, "head", (k) => k.loft("sk-skull", "head", B, T.bone, c ? [[-0.03, -0.03, 0, 0.05, 0.06], [0.07, -0.012, 0, 0.07, 0.088], [0.143, -0.008, 0, 0.03, 0.04]] : [[-0.082, -0.07, 0, 0.022, 0.02], [-0.03, -0.046, 0, 0.054, 0.058], [0.02, -0.024, 0, 0.066, 0.08], [0.074, -0.012, 0, 0.072, 0.09], [0.12, -0.008, 0, 0.052, 0.066], [0.143, -0.008, 0, 0.022, 0.03]]));
  put(out, "neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, c ? [[-0.012, 0.028, 0, 0.038, 0.038], [0.11, 0.014, 0, 0.038, 0.038]] : [[-0.012, 0.03, 0, 0.018, 0.017], [0.11, 0.014, 0, 0.018, 0.017]]));
  put(out, "torso", (k) => {
    k.loft("sk-spine", "torso", B, T.bone, c ? [[-0.01, 0.044, 0, 0.04, 0.038], [0.09, 0.026, 0, 0.04, 0.038], [0.33, 0.062, 0, 0.04, 0.038], [0.515, 0.03, 0, 0.04, 0.038]] : [[-0.01, 0.044, 0, 0.026, 0.02], [0.09, 0.02, 0, 0.025, 0.02], [0.2, 0.04, 0, 0.022, 0.019], [0.33, 0.066, 0, 0.02, 0.018], [0.515, 0.032, 0, 0.018, 0.016]]);
    k.loft("sk-ribcage", "torso", B, T.bone, c ? [[0.16, 0.006, 0, 0.122, 0.085], [0.3, 0, 0, 0.134, 0.098], [0.46, 0.012, 0, 0.07, 0.05]] : [[0.16, 0.008, 0, 0.118, 0.08], [0.22, 0.002, 0, 0.132, 0.094], [0.32, 0, 0, 0.132, 0.098], [0.42, 0.007, 0, 0.1, 0.072], [0.465, 0.013, 0, 0.06, 0.044]]);
    if (!c) k.cap("sk-shoulder-girdle", "torso", B, T.bone, [0.455, 0.014, -0.17], [0.455, 0.014, 0.17], 0.0155, 0.0155);
  });
  put(out, "pelvis", (k) => k.loft("sk-pelvis", "pelvis", B, T.bone, c ? [[-0.06, 0.03, 0, 0.07, 0.04], [0.03, 0.02, 0, 0.115, 0.06], [0.115, 0.02, 0, 0.13, 0.06]] : [[-0.065, 0.03, 0, 0.066, 0.03], [-0.02, 0.01, 0, 0.1, 0.06], [0.04, 0.02, 0, 0.118, 0.062], [0.118, 0.022, 0, 0.134, 0.058]], undefined, "body", 2.6));
  for (const [sfx, g] of SIDES) {
    put(out, `thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m(c ? [[-0.02, 0, 0.004, rf, rf], [0.06, 0.003, 0.022, rf, rf], [0.445, 0, 0, rf, rf]] : [[-0.022, 0, 0, 0.016, 0.016], [0.005, 0, 0.008, 0.024, 0.024], [0.06, 0.003, 0.044, r, r], [0.3, 0.009, 0.016, r - 0.001, r - 0.001], [0.435, 0, 0, 0.04, 0.03], [0.452, 0, 0, 0.034, 0.024]], g)));
    put(out, `arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, c ? [[-0.018, 0, 0, rh, rh], [0.34, 0, 0, rh, rh]] : [[-0.02, 0, 0, 0.014, 0.014], [0.0, 0, 0, 0.023, 0.023], [0.05, 0, 0, r - 0.002, r - 0.002], [0.26, 0, 0, r - 0.004, r - 0.004], [0.335, 0, 0, 0.03, 0.015]]));
    put(out, `forearm-${sfx}`, (k) => k.loft(`sk-forearm-${sfx}`, `forearm-${sfx}`, B, T.bone, c ? [[-0.012, 0, 0, 0.022, 0.022], [0.258, 0, 0, 0.022, 0.022]] : [[-0.014, -0.004, 0, 0.022, 0.016], [0.08, -0.002, 0, 0.018, 0.017], [0.258, 0, 0, 0.012, 0.022]]));
    put(out, `hand-${sfx}`, (k) => k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m(c ? [[0, 0.003, 0, 0.0225, 0.024], [0.124, 0.002, -0.016, 0.0225, 0.03]] : [[0, 0.003, 0, 0.0155, 0.022], [0.08, 0.004, -0.006, 0.0155, 0.034], [0.128, 0.002, -0.022, 0.0155, 0.028]], g), undefined, "body", 3));
  }
  put(out, "shank-l", (k) => k.loft("sk-tibia-l", "shank-l", B, T.bone, c ? [[-0.01, 0, 0, r + 0.008, r + 0.004], [0.12, 0.004, 0, r, r], [0.43, 0, 0, r - 0.002, r - 0.002]] : [[-0.012, 0, 0, 0.04, 0.03], [0.06, 0.006, 0, 0.022, 0.022], [0.29, 0.004, 0, r - 0.001, r - 0.001], [0.43, 0, 0, 0.026, 0.018]]));
  put(out, "foot-l", (k) => k.loftPlane("sk-foot-l", "foot-l", FOOT, B, T.bone, c ? [[-0.05, 0.045, 0, 0.038, 0.038], [0.13, 0.04, 0, 0.042, 0.038]] : [[-0.06, 0.042, 0, 0.014, 0.014], [-0.02, 0.05, 0, 0.022, 0.02], [0.01, 0.058, 0, 0.026, 0.02], [0.14, 0.03, 0, 0.042, 0.01]], undefined, "body", 2.6));
  put(out, "toe-l", (k) => k.loftPlane("sk-phalanges-l", "toe-l", TOE, B, T.bone, c ? [[0.135, 0.026, 0, 0.04, 0.026], [0.205, 0.024, -0.004, 0.03, 0.02]] : [[0.135, 0.03, 0, 0.04, 0.0155], [0.207, 0.028, -0.006, 0.024, 0.0155]], undefined, "body", 2.6));
  put(out, "shank-r", (k) => k.loft("sk-tibia-r", "shank-r", B, T.bone, c ? [[-0.01, 0, 0, r + 0.008, r + 0.004], [0.12, 0.004, 0, r, r]] : [[-0.012, 0, 0, 0.04, 0.03], [0.06, 0.006, 0, 0.022, 0.022], [0.13, 0.006, 0, 0.016, 0.016]]));
  put(out, "shank-r", (k) => k.loft("sk-pylon", "shank-r", C, T.metal, c ? [[0.11, 0, 0, 0.038, 0.038], [0.181, 0, 0, 0.038, 0.038]] : [[0.12, 0, 0, 0.03, 0.03, 3], [0.181, 0, 0, 0.024, 0.024, 3]]));
  out.push(of("blade-upper", "blade-r"), of("blade-curl", "blade-tip-r"));
  if (!c) out.push(...joints(["thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "arm-l", "arm-r", "forearm-l", "forearm-r", "blade-r", "blade-tip-r"], 1.6));
  out.push(...masses(asset, c ? 0.038 : 0));
  return out;
}

export const structure = (asset: Json, _k: Kit, scale: number): Json[] => (scale === 50 ? structure50(asset) : scale === 100 ? structure100(asset) : structureCoarse(asset, scale as 200 | 500));
void RIG;
void ADAPTER_Y;

/* ------------------------------------------------------------------ decisions */

export const DECISIONS: Record<number, Json[]> = {
  50: [
    { en: "Body 1:50: a lean 1.80 m runner of 68.5 kg modelled from anthropometric sections. Sleeveless running singlet as one garment from the flared hem over waist, chest and the sloping shoulder line to the collar rib; the bare deltoids, upper arms with biceps, forearms that turn from the wide elbow to the upright wrist, loosely closed fists with four curled fingers and the thumb. Head with jaw, chin, nose, ears and eyes under cropped hair. Short compression shorts with the gluteal masses and two tight legs; thighs, the left knee cap, the calf bulging to the back over a straight shin, a low sock and a running shoe with separate midsole, upper and rocker toe. Right leg: gel liner as a cuff over the lower thigh and the knee, carbon socket with a tilted brim (higher at the patellar tendon, lower in the hollow of the knee) and a rim, lamination plate and pyramid receiver, then the C-shaped carbon blade as a leaf spring 62 mm wide that tapers from 11 mm at the clamp plate to 6 mm at the toe, with a rubber sole pad. The blade is two solids that meet in a round joint at the bend the rig flexes.", de: "Körper 1:50: ein schlanker, 1,80 m grosser Läufer von 68,5 kg aus anthropometrischen Querschnitten. Ärmelloses Lauftop als ein Kleidungsstück vom ausgestellten Saum über Taille, Brust und die abfallende Schulterlinie bis zum Kragenbündchen; freie Deltamuskeln, Oberarme mit Bizeps, Unterarme, die sich vom breiten Ellbogen zum hochkant stehenden Handgelenk drehen, locker geschlossene Fäuste mit vier gekrümmten Fingern und Daumen. Kopf mit Kiefer, Kinn, Nase, Ohren und Augen unter kurzem Haar. Kurze Kompressionsshorts mit den Gesässmassen und zwei engen Beinen; Oberschenkel, linke Kniescheibe, Wade, die über dem geraden Schienbein nach hinten auslädt, niedrige Socke und Laufschuh mit eigener Zwischensohle, Schaft und aufgebogener Spitze. Rechtes Bein: Gelliner als Manschette über dem unteren Oberschenkel und dem Knie, Karbonschaft mit geneigtem Rand (vorn an der Patellasehne höher, in der Kniekehle tiefer) und Randwulst, Eingussplatte und Pyramidenaufnahme, dann die C-förmige Karbonfeder als 62 mm breite Blattfeder, die sich von 11 mm an der Klemmplatte auf 6 mm an der Spitze verjüngt, mit Gummisohle. Die Feder besteht aus zwei Körpern, die sich in einem runden Gelenk an der Biegestelle des Rigs treffen." },
    { en: "Thermal 1:50: every region is the surface it names. Singlet, shorts and hair are single shells; the shoe is two nested shells (sock under the 6 mm upper), the socket two (6 mm gel liner under the 4 mm carbon shell), and the liner shows as the cuff above the brim. The face is its own solid so that the scalp under the hair is not counted as face. The breath outlets are small patches under the nose tip and on the lips, the plume leaves from a patch on the crown.", de: "Thermik 1:50: Jede Region ist die Oberfläche, die sie benennt. Top, Shorts und Haar sind einzelne Schalen; der Schuh hat zwei ineinanderliegende Schalen (Socke unter dem 6 mm starken Schaft), der Prothesenschaft zwei (6 mm Gelliner unter der 4 mm starken Karbonschale), und der Liner zeigt sich als Manschette über dem Rand. Das Gesicht ist ein eigener Körper, damit die Kopfhaut unter dem Haar nicht als Gesicht zählt. Die Atemauslässe sind kleine Flächen unter der Nasenspitze und auf den Lippen, die Warmluftfahne verlässt den Körper an einer Fläche auf dem Scheitel." },
    { en: "Structure 1:50: the load path. Skull with facial block, cervical spine, thoracic and lumbar spine with their S-curve (lumbar lordosis, thoracic kyphosis) and the row of spinous processes, eight rib hoops with sternum, clavicles, shoulder blades with acromion, sacrum, iliac wings with crest, ischium and pubic rami, femora with head, neck, greater trochanter, bowed shaft and condyles, patellae, humeri, crossed radius and ulna, carpus, metacarpus and finger bones; on the left tibia, fibula, talus, calcaneus, metatarsus and toe phalanges. On the right the tibia and fibula end 13 cm below the knee; from there the load runs through the carbon socket shell, the lamination plate and the pyramid receiver into the two blade halves and the sole pad, which are the body solids themselves because they are the structural members. Joints sit at the rig joints (the bolted clamp and the elastic bend of the blade included), masses at the de Leva centres of pelvis, trunk and thighs. The structure declares no supports (inertial model).", de: "Tragwerk 1:50: der Lastpfad. Schädel mit Gesichtsblock, Halswirbelsäule, Brust- und Lendenwirbelsäule mit ihrer S-Krümmung (Lendenlordose, Brustkyphose) und der Reihe der Dornfortsätze, acht Rippenbögen mit Brustbein, Schlüsselbeine, Schulterblätter mit Schulterhöhe, Kreuzbein, Darmbeinschaufeln mit Kamm, Sitzbein und Schambeinäste, Oberschenkelknochen mit Kopf, Hals, grossem Rollhügel, gebogenem Schaft und Gelenkrollen, Kniescheiben, Oberarmknochen, gekreuzte Speiche und Elle, Handwurzel, Mittelhand und Fingerknochen; links Schien- und Wadenbein, Sprungbein, Fersenbein, Mittelfuss und Zehenglieder. Rechts enden Schien- und Wadenbein 13 cm unter dem Knie; von dort läuft die Last durch die Karbonschale des Schafts, die Eingussplatte und die Pyramidenaufnahme in die beiden Federhälften und die Sohle, die hier die Körper selbst sind, weil sie die tragenden Bauteile sind. Gelenkkörper sitzen an den Gelenken des Rigs (auch an der verschraubten Klemmung und an der elastischen Biegestelle der Feder), Massen in den Schwerpunkten nach de Leva von Becken, Rumpf und Oberschenkeln. Das Tragwerk hat keine Auflager (Trägheitsmodell)." },
  ],
  100: [
    { en: "Body 1:100: the same runner drawn anew with fewer, longer sections. Nose, ears, eyes, knee cap, collar rib, socket rim and sole pad fall below 15 mm and are left out; the gluteal masses are part of the shorts, sole and upper are one shoe form, the fingers are one closed fist with a separate thumb, lamination plate and pyramid receiver are the tapered end of the socket form, and the blade is a band of constant 12 mm.", de: "Körper 1:100: derselbe Läufer, neu gezeichnet mit weniger und längeren Querschnitten. Nase, Ohren, Augen, Kniescheibe, Kragenbündchen, Schaftrand und Sohle liegen unter 15 mm und entfallen; die Gesässmassen gehören zu den Shorts, Sohle und Schaft sind eine Schuhform, die Finger eine geschlossene Faust mit eigenem Daumen, Eingussplatte und Pyramidenaufnahme das verjüngte Ende der Schaftform, und die Feder ist ein Band von gleichbleibend 12 mm." },
    { en: "Thermal 1:100: one surface per region and body part, layers merged into it; liner cuff and socket shell stay two parts of the socket region; the outlets are enlarged to the scale.", de: "Thermik 1:100: eine Oberfläche je Region und Körperteil, die Schichten sind darin zusammengefasst; Linermanschette und Schaftschale bleiben zwei Teile der Schaftregion; die Auslässe sind auf den Massstab vergrössert." },
    { en: "Structure 1:100: skeleton with merged parts: skull and face, cervical spine, three spine sections, four rib hoops, sternum, clavicles and shoulder blades, sacrum, hip bones and one pubic bar, each femur one form from head over neck and shaft to the condyles, humerus, radius and ulna, hand and curled finger row; left tibia, fibula and one foot form; right the tibial stump, then socket and blade as members.", de: "Tragwerk 1:100: Skelett mit zusammengefassten Teilen: Schädel und Gesicht, Halswirbelsäule, drei Wirbelsäulenabschnitte, vier Rippenbögen, Brustbein, Schlüsselbeine und Schulterblätter, Kreuzbein, Hüftbeine und ein Schambeinstab, jeder Oberschenkelknochen eine Form vom Kopf über Hals und Schaft zu den Gelenkrollen, Oberarmknochen, Speiche und Elle, Hand und gekrümmte Fingerreihe; links Schien- und Wadenbein und eine Fussform; rechts der Schienbeinstumpf, dann Schaft und Feder als Bauteile." },
  ],
  200: [
    { en: "Body 1:200: masses of at least 30 mm. Head with jaw, neck, hair cap, singlet, shorts with their legs, thighs, the left shank with the sock merged into it, shoe and toe cap, arms, forearms and mitten fists; on the right the liner cuff on the thigh and one socket form from the knee to the adapter. The blade is drawn 18 mm thick so that the C stays a readable band.", de: "Körper 1:200: Massen ab 30 mm. Kopf mit Kiefer, Hals, Haarkappe, Top, Shorts mit ihren Beinen, Oberschenkel, linker Unterschenkel mit eingeschlossener Socke, Schuh und Zehenkappe, Arme, Unterarme und Fäustlinge; rechts die Linermanschette am Oberschenkel und eine Schaftform vom Knie bis zum Adapter. Die Feder ist 18 mm stark gezeichnet, damit das C ein lesbares Band bleibt." },
    { en: "Thermal 1:200: one surface per region and body part.", de: "Thermik 1:200: eine Oberfläche je Region und Körperteil." },
    { en: "Structure 1:200: skull, cervical column, spine, rib cage as one barrel, one shoulder girdle bar, pelvis basin, femur with offset head, humerus, forearm and hand as single bones, left tibia, foot wedge and toes; right the tibial stump, a pylon block for socket base and adapter, and the blade; joints at hip, knee, ankle, shoulder, elbow and at the blade.", de: "Tragwerk 1:200: Schädel, Halssäule, Wirbelsäule, Brustkorb als ein Fass, ein Schultergürtelstab, Beckenschale, Oberschenkelknochen mit versetztem Kopf, Oberarm, Unterarm und Hand als einzelne Knochen, links Schienbein, Fusskeil und Zehen; rechts der Schienbeinstumpf, ein Pylonblock für Schaftboden und Adapter und die Feder; Gelenke an Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen und an der Feder." },
  ],
  500: [
    { en: "Body 1:500: the figure in its fewest masses: head with neck, hair, singlet, shorts with their legs, and per side thigh, arm, forearm and fist; left shank, shoe and toe cap; right one socket form and the blade as a 24 mm band, so that blade and foot read as two different things at a glance.", de: "Körper 1:500: die Figur in ihren wenigsten Massen: Kopf mit Hals, Haar, Top, Shorts mit ihren Beinen und je Seite Oberschenkel, Arm, Unterarm und Faust; links Unterschenkel, Schuh und Zehenkappe; rechts eine Schaftform und die Feder als 24 mm starkes Band, sodass Feder und Fuss auf einen Blick als zwei verschiedene Dinge lesbar sind." },
    { en: "Thermal 1:500: one surface per region; head and neck, one form in the body, are split into face, neck and hair, and the right thigh, one form in the body, is split where the liner cuff begins.", de: "Thermik 1:500: eine Oberfläche je Region; Kopf und Hals, im Körper eine Form, sind in Gesicht, Hals und Haar geteilt, und der rechte Oberschenkel, im Körper eine Form, ist dort geteilt, wo die Linermanschette beginnt." },
    { en: "Structure 1:500: a stick skeleton of thickened bones, still the load path and never the body: skull, cervical column, spine, rib cage, pelvis and one bone per limb segment, the tibial stump, the pylon block and the blade, masses at the centres of pelvis, trunk and thighs.", de: "Tragwerk 1:500: ein Strichskelett aus verdickten Knochen, weiterhin der Lastpfad und nie der Körper: Schädel, Halssäule, Wirbelsäule, Brustkorb, Becken und ein Knochen je Gliedmassenabschnitt, der Schienbeinstumpf, der Pylonblock und die Feder, Massen in den Schwerpunkten von Becken, Rumpf und Oberschenkeln." },
  ],
};
