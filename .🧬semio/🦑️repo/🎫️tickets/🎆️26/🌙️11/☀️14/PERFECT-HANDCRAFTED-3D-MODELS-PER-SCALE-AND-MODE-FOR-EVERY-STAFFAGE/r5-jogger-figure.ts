/** 🏃‍♂️ The round 5 jogger, remodelled by hand: body of every scale from `r5-jogger-body.ts` (smooth anatomical lofts through hand-stated key sections), thermal model and skeleton on the corrected rig (upper arm 0.336 m, hand 0.195 m) for a lean 1.82 m (shod) male runner of 72 kg. Trunk bones: u up, v to the back, w to the left. Limb bones: u distal, v to the front, w to the left. Foot and toe: u ahead, v up.
 * Usage: `bun r5-jogger-figure.ts` writes `volumes/humans/jogger.json`.
 * @see ./r5-jogger-body.ts
 * @see ./r3-humans-jogger.md */
import { writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, parts, thermalOf, type Json, type Row, type V3 } from "./r3-jogger-kit.ts";
import { SLEEVE100, SLEEVE500, body50, body100, body200, body500 } from "./r5-jogger-body.ts";
import { smooth, tube } from "./r5-runners-kit.ts";

/** ✂️ The rows of a loft between u = `a` and u = `b`, with sections interpolated exactly at the two cuts. */
function cut(rows: readonly Row[], a: number, b: number): Row[] {
  const at = (u: number): Row => {
    const i = rows.findIndex((r) => r[0] >= u);
    const [p, q] = [rows[i - 1]!, rows[i]!];
    const t = (u - p[0]) / (q[0] - p[0]);
    return [0, 1, 2, 3, 4].map((n) => p[n]! + (q[n]! - p[n]!) * t).concat([p[5] ?? 2]) as unknown as Row;
  };
  const inner = rows.filter((r) => r[0] > Math.max(a, rows[0]![0]) && r[0] < Math.min(b, rows[rows.length - 1]![0]));
  return [...(a > rows[0]![0] ? [at(a)] : [rows[0]!]), ...inner, ...(b < rows[rows.length - 1]![0] ? [at(b)] : [rows[rows.length - 1]!])];
}

import { axial, face, rod, type Fit } from "./r4-g06-skeleton.ts";
import { structureText } from "./r4-g06-texts.ts";
const SKIN = "skin";
const SHIRT = "nylon-clothing";
const SOCK = "cotton-clothing";
const SHOE = "leather-shoe";
const HAIR = "hair";
const T = { skin: 0.72, shirt: 0.88, shorts: 0.22, hair: 0.15, band: 0.9, shoe: 0.92, sole: 0.55, sock: 0.95, arm: 0.25, eye: 0.1, bone: 0.85 };
const SIDES = [["l", 1], ["r", -1]] as const;
const m = (rows: readonly Row[], g: number): Row[] => rows.map(([s, cv, cw, rx, ry, e]) => [s, cv, cw * g, rx, ry, e] as Row);

/* ------------------------------------------------------------------ thermal */

/** 🕳️ Body solids whose thermal copy drops flat end faces (start, end); none: the validator measures open shells as more area, not less, so every thermal solid stays closed. */
const OPEN: [RegExp, boolean, boolean][] = [];

function thermal(asset: Json, k: Kit, scale: number): Json[] {
  const regions = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  const out: Json[] = [];
  const t = new Kit();
  const fine = true;
  const own = scale >= 100 ? /^sleeve-/ : /^$/;
  for (const s of k.solids) {
    const region = regions.get(k.region.get(s.id)!)!;
    if ((fine && (s.id === "head" || s.id === "neck")) || own.test(s.id)) continue;
    const open = OPEN.find(([re]) => re.test(s.id));
    if (open === undefined) out.push(...thermalOf(s, region, scale === 50));
    else out.push(...thermalOf({ ...s, capStart: open[1], capEnd: open[2] }, region, scale === 50).map((x) => (x.of === undefined ? x : { ...s, capStart: open[1], capEnd: open[2], id: x.id, region: x.region, ...(x.layer === undefined ? {} : { layer: x.layer }) })));
  }
  if (fine) {
    out.push({ ...t.loft("th-face", "head", SKIN, T.skin, [[0.008, -0.076, 0, 0.026, 0.016, 2.2], [0.028, -0.06, 0, 0.05, 0.04, 2.2], [0.055, -0.04, 0, 0.061, 0.062, 2.2], [0.085, -0.022, 0, 0.068, 0.081], [0.125, -0.032, 0, 0.073, 0.071], [0.165, -0.064, 0, 0.068, 0.04], [0.196, -0.08, 0, 0.05, 0.012]]), region: "head-skin" });
    out.push({ ...t.loft("th-neck", "head", SKIN, T.skin, [[0.0, 0.008, 0, 0.061, 0.062], [0.04, 0.002, 0, 0.057, 0.06], [0.085, -0.006, 0, 0.056, 0.062]]), region: "head-skin" });
  }
  if (scale >= 100)
    for (const [sfx, g] of SIDES) {
      const c = scale === 500;
      const sleeve = c ? SLEEVE500 : smooth(SLEEVE100, 2);
      out.push({ ...t.loft(`th-shoulder-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(cut(sleeve, -1, 0.066), g)), region: "torso-clothing" });
      out.push({ ...t.loft(`th-sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(cut(sleeve, 0.066, 1), g)), region: "arm-clothing" });
    }
  const h = (0.0077 * scale) / 100;
  const e = (id: string, at: V3, radii: V3, region: string, emitter: string): Json => ({ ...t.ell(id, "head", region === "torso-clothing" ? HAIR : SKIN, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(e("th-nose-breath", [0.081, -0.115 + h, 0], [h, h, Math.max(h, 0.008)], "head-skin", "nose-breath"));
  out.push(e("th-mouth-breath", [0.052, -0.1035 + h, 0], [h, h, Math.max(h, 0.0105)], "head-skin", "mouth-breath"));
  out.push(e("th-plume", [0.2555 - h, 0, 0], [h, Math.max(0.05, h), Math.max(0.042, h)], "torso-clothing", "plume"));
  return out;
}

/* ------------------------------------------------------------------ structure */

const B = SKIN;
function masses(asset: Json, least = 0): Json[] {
  const k = new Kit();
  const out: Json[] = [];
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < 0.1 * asset.dimensions.massKg) continue;
    const r = Math.max(0.011 * Math.cbrt(b.mass.kg), least);
    out.push(...parts([k.ell(`mass-${b.id}`, b.id, B, 0.4, [b.mass.comX, b.mass.comY ?? 0, 0], [r, r, r])], b.id, "mass"));
  }
  return out;
}
/** 🔩️ Joint solids at the real joints: balls at hip and shoulder, hinge pins through knee, elbow, ankle and the toe joints, discs between the vertebrae at L5/S1 and C7/T1; `f` enlarges them for a coarser scale. */
const JOINT: Record<string, [at: V3, radii: V3]> = { torso: [[0, 0.04, 0], [0.007, 0.022, 0.029]], head: [[0, 0.031, 0], [0.006, 0.017, 0.02]], thigh: [[0, 0, 0], [0.027, 0.027, 0.027]], shank: [[0, 0, 0], [0.014, 0.014, 0.046]], foot: [[0, 0, 0], [0.011, 0.011, 0.031]], toe: [[0, 0.03, 0], [0.008, 0.008, 0.046]], "upper-arm": [[0, 0, 0], [0.026, 0.026, 0.026]], forearm: [[0, 0, 0], [0.01, 0.01, 0.037]], hand: [[0, 0, 0], [0.009, 0.026, 0.013]] };
function joints(ids: readonly string[], f: number): Json[] {
  const k = new Kit();
  return ids.flatMap((id) => {
    const [at, r] = JOINT[id.replace(/-[lr]$/, "")]!;
    return parts([k.ell(`joint-${id}`, id, B, 0.5, at, [r[0] * f, r[1] * f, Math.max(r[2], r[2] * (1 + (f - 1) / 3))])], id, "joint");
  });
}
const LIMB_JOINTS = ["thigh", "shank", "foot", "toe", "upper-arm", "forearm", "hand"].flatMap((b) => SIDES.map(([s]) => `${b}-${s}`));
const SPINE: Row[] = [[-0.01, 0.04, 0, 0.025, 0.018], [0.08, 0.03, 0, 0.024, 0.018], [0.17, 0.04, 0, 0.021, 0.017], [0.3, 0.062, 0, 0.018, 0.016], [0.42, 0.05, 0, 0.016, 0.014], [0.515, 0.032, 0, 0.015, 0.013]];

/** 📐 The fit of the shared adult skeleton table to the jogger: it is the reference figure itself. */
const FIT: Fit = { material: B, tone: T.bone, ks: 1, kv: 1, kw: 1, dv: 0, kp: 1, kpv: 1, drop: 1, shoulder: 0.4447, torso: "torso", pelvis: "pelvis" };
/** 💀 Face mass `[centre, radii]`, jaw joint, jaw angle and chin of the left side in the head bone (the bone starts at the base of the neck). */
const FACE: readonly [readonly [V3, V3], V3, V3, V3] = [[[0.096, -0.054, 0], [0.038, 0.032, 0.05]], [0.106, -0.008, 0.052], [0.05, -0.012, 0.047], [0.024, -0.076, 0.012]];
const byBone = (solids: readonly Json[]): Json[] => solids.map((s) => ({ ...s, member: s.bone, part: "member" }));

function structure50(asset: Json): Json[] {
  const out: Json[] = [];
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.162, -0.002, 0], [0.076, 0.09, 0.07]);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.031, 0, 0.016, 0.014], [0.03, 0.02, 0, 0.015, 0.013], [0.06, 0.014, 0, 0.014, 0.013], [0.095, 0.016, 0, 0.017, 0.014]]);
  });
  out.push(...parts(face(FIT, "head", 50, FACE[0], FACE[1], FACE[2], FACE[3], 0.008), "head", "member"), ...byBone(axial(FIT, 50)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => {
      k.ell(`sk-femur-head-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.024, 0.024, 0.024]);
      k.cap(`sk-femur-neck-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.032, 0, 0.048 * g], 0.015, 0.014);
      k.ell(`sk-trochanter-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.03, -0.004, 0.055 * g], [0.03, 0.02, 0.014]);
      k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[0.04, 0, 0.05, 0.016, 0.016], [0.12, 0.006, 0.04, 0.0145, 0.0145], [0.25, 0.01, 0.022, 0.014, 0.014], [0.36, 0.006, 0.008, 0.016, 0.015], [0.41, 0.002, 0.002, 0.028, 0.022], [0.4447, 0, 0, 0.041, 0.032], [0.462, 0, 0, 0.034, 0.024]], g));
      k.ell(`sk-patella-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.436, 0.044, 0], [0.022, 0.01, 0.02]);
    });
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.012, 0, 0, 0.04, 0.03], [0.02, 0.004, -0.002, 0.03, 0.026], [0.07, 0.01, -0.004, 0.017, 0.019], [0.2, 0.01, -0.004, 0.013, 0.015], [0.36, 0.004, -0.004, 0.013, 0.013], [0.415, 0, -0.004, 0.02, 0.017], [0.44, 0, -0.008, 0.021, 0.016]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.035, -0.012, 0.034 * g], [0.442, -0.008, 0.0255 * g], 0.0075, 0.0065);
    });
    put(`foot-${sfx}`, (k) => {
      k.ell(`sk-talus-${sfx}`, `foot-${sfx}`, B, T.bone, [0.005, -0.014, 0], [0.028, 0.016, 0.02]);
      k.ell(`sk-calcaneus-${sfx}`, `foot-${sfx}`, B, T.bone, [-0.034, -0.035, 0.003 * g], [0.034, 0.017, 0.016]);
      k.loft(`sk-metatarsus-${sfx}`, `foot-${sfx}`, B, T.bone, [[0.02, -0.028, 0, 0.026, 0.016], [0.07, -0.04, 0, 0.032, 0.011], [0.115, -0.05, 0, 0.04, 0.008], [0.138, -0.054, 0, 0.042, 0.008]], undefined, "body", 2.6);
    });
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.0285, 0, 0.04, 0.008], [0.035, 0.0275, -0.004, 0.034, 0.007], [0.07, 0.0265, -0.007, 0.02, 0.006]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => {
      k.ell(`sk-humerus-head-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [0, 0, -0.003 * g], [0.023, 0.023, 0.023]);
      k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[0.0157, 0, 0, 0.017, 0.017], [0.0838, 0.002, 0, 0.012, 0.012], [0.2515, 0, 0, 0.0105, 0.0105], [0.3092, 0, 0, 0.02, 0.011], [0.336, 0, 0, 0.032, 0.014], [0.3479, 0, 0, 0.026, 0.012]]);
    });
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.272, -0.016, -0.002 * g], 0.011, 0.006);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.272, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(`hand-${sfx}`, (k) => {
      k.ell(`sk-carpus-${sfx}`, `hand-${sfx}`, B, T.bone, [0.014, 0.003, 0], [0.017, 0.024, 0.01]);
      k.loft(`sk-metacarpus-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.03, 0.004, -0.002, 0.007, 0.026], [0.097, 0.002, -0.005, 0.006, 0.035]], g), undefined, "body", 3);
      for (const [i, kv, f] of [[1, 0.026, 1], [2, 0.006, 1.06], [3, -0.014, 1], [4, -0.031, 0.84]] as const) {
        const p: V3[] = [[0.102, kv, -0.006 * g], [0.102 + 0.025 * f, kv, -0.012 * g], [0.102 + 0.038 * f, kv, (-0.012 - 0.017 * f) * g], [0.102 + 0.034 * f, kv, (-0.012 - 0.036 * f) * g]];
        k.cap(`sk-phalanx-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p[0]!, p[1]!, 0.0045, 0.004);
        k.cap(`sk-phalanx-mid-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p[1]!, p[2]!, 0.004, 0.0038);
        k.cap(`sk-phalanx-tip-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p[2]!, p[3]!, 0.0038, 0.0038);
      }
      k.cap(`sk-thumb-${sfx}`, `hand-${sfx}`, B, T.bone, [0.02, 0.022, -0.008 * g], [0.062, 0.04, -0.018 * g], 0.005, 0.0045);
      k.cap(`sk-thumb-tip-${sfx}`, `hand-${sfx}`, B, T.bone, [0.062, 0.04, -0.018 * g], [0.104, 0.042, -0.034 * g], 0.0045, 0.004);
    });
  }
  out.push(...joints(["torso", "head", ...LIMB_JOINTS], 1), ...masses(asset));
  return out;
}

function structure100(asset: Json): Json[] {
  const out: Json[] = [];
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.16, -0.002, 0], [0.078, 0.09, 0.07]);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.031, 0, 0.016, 0.015], [0.045, 0.016, 0, 0.015, 0.015], [0.095, 0.016, 0, 0.017, 0.015]]);
  });
  out.push(...parts(face(FIT, "head", 100, FACE[0], FACE[1], FACE[2], FACE[3], 0.008), "head", "member"), ...byBone(axial(FIT, 100)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.022, 0, 0, 0.014, 0.014], [0, 0, 0.004, 0.024, 0.024], [0.03, 0, 0.038, 0.024, 0.02], [0.075, 0.004, 0.046, 0.016, 0.016], [0.25, 0.01, 0.022, 0.0145, 0.0145], [0.4, 0.003, 0.003, 0.024, 0.02], [0.4447, 0, 0, 0.041, 0.032], [0.462, 0, 0, 0.034, 0.024]], g)));
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.012, 0, 0, 0.04, 0.03], [0.05, 0.008, -0.003, 0.02, 0.021], [0.3, 0.006, -0.004, 0.0135, 0.014], [0.44, 0, -0.007, 0.022, 0.017]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.035, -0.012, 0.033 * g], [0.442, -0.008, 0.025 * g], 0.008, 0.0075);
    });
    put(`foot-${sfx}`, (k) => k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [[-0.066, -0.036, 0, 0.012, 0.012], [-0.035, -0.034, 0, 0.022, 0.019], [0.005, -0.02, 0, 0.026, 0.022], [0.07, -0.04, 0, 0.032, 0.011], [0.138, -0.054, 0, 0.042, 0.008]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.0285, 0, 0.04, 0.008], [0.07, 0.0265, -0.007, 0.022, 0.0075]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[-0.021, 0, 0, 0.013, 0.013], [0.0, 0, 0, 0.023, 0.023], [0.0367, 0, 0, 0.016, 0.016], [0.2515, 0, 0, 0.011, 0.011], [0.336, 0, 0, 0.032, 0.014], [0.3479, 0, 0, 0.026, 0.012]]));
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.272, -0.016, -0.002 * g], 0.011, 0.0075);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.272, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(`hand-${sfx}`, (k) => {
      k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.003, 0, 0.011, 0.022], [0.03, 0.004, -0.002, 0.008, 0.026], [0.098, 0.002, -0.005, 0.0075, 0.035]], g), undefined, "body", 3);
      tube(k, `sk-fingers-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.1, 0, -0.006, 0.0075, 0.034, 3], [0.126, 0, -0.012, 0.0075, 0.032, 3], [0.14, 0, -0.029, 0.0075, 0.03, 3], [0.136, 0, -0.047, 0.0075, 0.026, 3]], g));
    });
  }
  out.push(...joints(["torso", "head", ...LIMB_JOINTS], 1.25), ...masses(asset));
  return out;
}

function structureCoarse(asset: Json, scale: 200 | 500): Json[] {
  const out: Json[] = [];
  const c = scale === 500;
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  const r = 0.017;
  put("head", (k) => k.loft("sk-skull", "head", B, T.bone, c ? [[0.02, -0.04, 0, 0.0375, 0.0375], [0.1, -0.02, 0, 0.06, 0.07], [0.17, -0.004, 0, 0.07, 0.088], [0.238, 0, 0, 0.0375, 0.04]] : [[0.018, -0.06, 0, 0.02, 0.018], [0.05, -0.036, 0, 0.044, 0.046], [0.11, -0.022, 0, 0.062, 0.072], [0.17, -0.004, 0, 0.072, 0.09], [0.215, 0, 0, 0.052, 0.066], [0.238, 0, 0, 0.022, 0.03]]));
  if (c) out.push(...byBone([rod(FIT, "sk-cervical", "head", [-0.01, 0.034, 0], [0.11, 0.012, 0], 0.012)]));
  else put("head", (k) => k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.032, 0, 0.017, 0.016], [0.04, 0.018, 0, 0.016, 0.0155], [0.09, 0.014, 0, 0.017, 0.016]]));
  out.push(...byBone(axial(FIT, scale)));
  for (const [sfx, g] of SIDES) {
    if (c) {
      out.push(
        ...byBone([
          rod(FIT, `sk-femur-${sfx}`, `thigh-${sfx}`, [0, 0, 0.04 * g], [0.4447, 0, 0], 0.014),
          rod(FIT, `sk-tibia-${sfx}`, `shank-${sfx}`, [0, 0.002, 0], [0.4343, 0, 0], 0.0125),
          rod(FIT, `sk-foot-${sfx}`, `foot-${sfx}`, [-0.05, -0.04, 0], [0.138, -0.052, 0], 0.009),
          rod(FIT, `sk-phalanges-${sfx}`, `toe-${sfx}`, [0, 0.0285, 0], [0.072, 0.0265, -0.004 * g], 0.004),
          rod(FIT, `sk-humerus-${sfx}`, `upper-arm-${sfx}`, [0, 0, 0], [0.336, 0, 0], 0.0105),
          rod(FIT, `sk-forearm-${sfx}`, `forearm-${sfx}`, [0, 0, 0], [0.2792, 0, 0], 0.008),
          rod(FIT, `sk-hand-${sfx}`, `hand-${sfx}`, [0, 0.003, 0], [0.16, 0.002, -0.022 * g], 0.006),
        ]),
      );
      continue;
    }
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.022, 0, 0, 0.016, 0.016], [0.005, 0, 0.008, 0.024, 0.024], [0.06, 0.003, 0.044, r, r], [0.3, 0.009, 0.016, r - 0.001, r - 0.001], [0.4447, 0, 0, 0.04, 0.03], [0.462, 0, 0, 0.034, 0.024]], g)));
    put(`shank-${sfx}`, (k) => k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.06, 0.006, 0, 0.022, 0.022], [0.3, 0.004, 0, r - 0.001, r - 0.001], [0.44, 0, 0, 0.026, 0.018]]));
    put(`foot-${sfx}`, (k) => k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [[-0.064, -0.036, 0, 0.014, 0.014], [-0.02, -0.03, 0, 0.024, 0.02], [0.01, -0.02, 0, 0.026, 0.022], [0.138, -0.053, 0, 0.042, 0.01]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.029, 0, 0.04, 0.0155], [0.072, 0.027, -0.006, 0.024, 0.0155]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[-0.021, 0, 0, 0.014, 0.014], [0.0, 0, 0, 0.023, 0.023], [0.0524, 0, 0, r - 0.002, r - 0.002], [0.262, 0, 0, r - 0.004, r - 0.004], [0.3406, 0, 0, 0.03, 0.015]]));
    put(`forearm-${sfx}`, (k) => k.loft(`sk-forearm-${sfx}`, `forearm-${sfx}`, B, T.bone, [[-0.014, -0.004, 0, 0.022, 0.016], [0.08, -0.002, 0, 0.018, 0.017], [0.276, 0, 0, 0.012, 0.022]]));
    put(`hand-${sfx}`, (k) => k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.003, 0, 0.0155, 0.022], [0.09, 0.004, -0.006, 0.0155, 0.034], [0.148, 0.002, -0.025, 0.0155, 0.028]], g), undefined, "body", 3));
  }
  if (!c) out.push(...joints(["thigh", "shank", "foot", "upper-arm", "forearm"].flatMap((b) => SIDES.map(([s]) => `${b}-${s}`)), 1.6));
  out.push(...masses(asset, c ? 0.038 : 0));
  return out;
}

/* ------------------------------------------------------------------ decisions */

const DECISIONS: Record<number, Json[]> = {
  50: [
    { en: "Body 1:50: a lean 1.82 m runner modelled by hand from anthropometric key sections (DIN 33402, ANSUR II), every limb one smooth loft through them. The loose running shirt runs from a flared hem over the shorts over waist, ribcage and chest to the trapezius slope that rises at about 24 degrees from the acromion to the collar; the deltoid caps ride on the upper arms and continue that slope into the short sleeves, so the shoulder stays closed in every arm swing. Head with jaw, chin, nose ridge, ears and eyes, a short hair cap and the head-band; shorts with the gluteal mass and two flared legs; thighs with the quadriceps and the low vastus medialis, knee caps, calves that bulge to the back over a straight shin, ankle socks; running shoes with a flared midsole, heel counter, padded collar, instep and a rocker toe; upper arms with biceps and triceps, forearms turning from the wide elbow to the upright wrist, loosely closed running hands with the four fingers grouped and curled into the palm and the thumb resting on the index finger; an armband with phone pouch on the left arm.", de: "Körper 1:50: ein schlanker, 1,82 m grosser Läufer, von Hand aus anthropometrischen Leitquerschnitten modelliert (DIN 33402, ANSUR II), jede Gliedmasse ein glatter Loft durch sie hindurch. Das weite Laufshirt läuft von einem ausgestellten Saum über den Shorts über Taille, Brustkorb und Brust bis zur Trapezlinie, die vom Schulterdach mit etwa 24 Grad zum Kragen ansteigt; die Deltakappen sitzen auf den Oberarmen und führen diese Linie in die kurzen Ärmel weiter, sodass die Schulter in jedem Armschwung geschlossen bleibt. Kopf mit Kiefer, Kinn, Nasenrücken, Ohren und Augen, kurzer Haarkappe und Stirnband; Shorts mit der Gesässmasse und zwei ausgestellten Beinen; Oberschenkel mit dem Quadrizeps und dem tief liegenden Vastus medialis, Kniescheiben, Waden, die über dem geraden Schienbein nach hinten ausladen, Knöchelsocken; Laufschuhe mit ausgestellter Zwischensohle, Fersenkappe, gepolstertem Kragen, Rist und aufgebogener Spitze; Oberarme mit Bizeps und Trizeps, Unterarme, die sich vom breiten Ellbogen zum hochkant stehenden Handgelenk drehen, locker geschlossene Laufhände mit den vier Fingern zusammengefasst und in die Handfläche gekrümmt und dem Daumen auf dem Zeigefinger; ein Armband mit Telefontasche am linken Arm." },
    { en: "Thermal 1:50: every region is the body surface it names; shirt, sleeves and shorts are two nested shells (air gap under the fabric), the shoes three (sock, air, upper), each inset by the real layer thickness. Skin runs only a few centimetres under a garment. The breath outlets are small patches under the nose tip and on the lips, the plume leaves from a patch on the crown.", de: "Thermik 1:50: Jede Region ist die Körperoberfläche, die sie benennt; Shirt, Ärmel und Shorts sind zwei ineinanderliegende Schalen (Luftspalt unter dem Stoff), die Schuhe drei (Socke, Luft, Schaft), jeweils um die echte Schichtdicke eingerückt. Haut läuft nur wenige Zentimeter unter ein Kleidungsstück. Die Atemauslässe sind kleine Flächen unter der Nasenspitze und auf den Lippen, die Warmluftfahne verlässt den Körper an einer Fläche auf dem Scheitel." },
    structureText(50, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  100: [
    { en: "Body 1:100: the same runner drawn anew with fewer, longer key sections (smallest feature 15 mm). Nose, ears, eyes, knee caps and the phone pouch fall below the feature and are left out, the knee cap is part of the shank front; shoulder cap and sleeve are one garment, midsole and upper are one shoe form, palm and curled fingers are one mitten swept along the curl, the thumb stays its own mass.", de: "Körper 1:100: derselbe Läufer, neu gezeichnet mit weniger und längeren Leitquerschnitten (kleinstes Merkmal 15 mm). Nase, Ohren, Augen, Kniescheiben und Telefontasche liegen unter dem Merkmal und entfallen, die Kniescheibe gehört zur Vorderseite des Unterschenkels; Schulterkappe und Ärmel sind ein Kleidungsstück, Zwischensohle und Schaft eine Schuhform, Handfläche und gekrümmte Finger ein entlang der Krümmung gezogener Fäustling, der Daumen bleibt eine eigene Masse." },
    { en: "Thermal 1:100: one surface per region and body part, layers merged into it; shoulder cap and sleeve, one form in the body, are split at the sleeve seam; the outlets are enlarged to the scale.", de: "Thermik 1:100: eine Oberfläche je Region und Körperteil, die Schichten sind darin zusammengefasst; Schulterkappe und Ärmel, im Körper eine Form, sind an der Ärmelnaht geteilt; die Auslässe sind auf den Massstab vergrössert." },
    structureText(100, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  200: [
    { en: "Body 1:200: masses of at least 30 mm. Head and neck are one form under the hair cap, the head-band and armband are gone, the sock is part of the shank, hand and thumb are one curled mitten; shirt, shorts, sleeves and shoes keep their own forms because their tones carry the figure.", de: "Körper 1:200: Massen ab 30 mm. Kopf und Hals sind eine Form unter der Haarkappe, Stirnband und Armband entfallen, die Socke gehört zum Unterschenkel, Hand und Daumen sind ein gekrümmter Fäustling; Shirt, Shorts, Ärmel und Schuhe behalten eigene Formen, weil ihre Tonwerte die Figur tragen." },
    { en: "Thermal 1:200: one surface per region and body part; shoulder cap and sleeve, one form in the body, are split at the sleeve seam.", de: "Thermik 1:200: eine Oberfläche je Region und Körperteil; Schulterkappe und Ärmel, im Körper eine Form, sind an der Ärmelnaht geteilt." },
    structureText(200, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  500: [
    { en: "Body 1:500: the figure in its fewest masses: head with neck under a blunt hair dome, shirt, shorts, and per side shorts leg, thigh, shank, shoe, toe cap, sleeve with shoulder cap, upper arm, forearm and hand. The shorts reach a little lower than at the finer scales so that their band stays printable in every pose. Proportions and the running pose still read at a glance.", de: "Körper 1:500: die Figur in ihren wenigsten Massen: Kopf mit Hals unter einer stumpfen Haarkuppel, Shirt, Shorts und je Seite Hosenbein, Oberschenkel, Unterschenkel, Schuh, Zehenkappe, Ärmel mit Schulterkappe, Oberarm, Unterarm und Hand. Die Shorts reichen etwas tiefer als in den feineren Massstäben, damit ihr Band in jeder Haltung druckbar bleibt. Proportionen und Laufhaltung bleiben auf einen Blick lesbar." },
    { en: "Thermal 1:500: one surface per region; shoulder cap and sleeve, one form in the body, are split at the sleeve seam.", de: "Thermik 1:500: eine Oberfläche je Region; Schulterkappe und Ärmel, im Körper eine Form, sind an der Ärmelnaht geteilt." },
    structureText(500, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
};

/* ------------------------------------------------------------------ assembly */


export const SCALES = [50, 100, 200, 500] as const;
export type Built = { volume: Json; kit: Kit };

/** 🏗️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  ({ 50: body50, 100: body100, 200: body200, 500: body500 })[scale](kit);
  const structure = (scale === 50 ? structure50(asset) : scale === 100 ? structure100(asset) : structureCoarse(asset, scale));
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale), structure, decisions: DECISIONS[scale]! } };
}

export const ASSET_FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/jogger/🔣️.json";

if (import.meta.main) {
  const asset = JSON.parse(await Bun.file(ASSET_FILE).text()).asset as Json;
  const volumes = Object.fromEntries(SCALES.map((s) => [String(s), build(asset, s).volume]));
  writeFileSync(join(import.meta.dir, "volumes", "humans", "jogger.json"), `${JSON.stringify({ volumes })}\n`);
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
