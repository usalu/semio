/** 🧾️ The thermal model and the skeleton of the round 5 running child (the bodies of every scale are in `r5-running-child-body.ts`). Frames as in the body file.
 * @see ./r3-running-child-body.ts
 * @see ./r3-running-child-texts.ts */
import { FOOT, HAIR, PACK, SHIRT, SHOE, SHORTS, SIDES, SKIN, SOCK, STRAP, T, m } from "./r3-running-child-body.ts";
import { axial, face as faceBones, type Fit } from "./r4-g06-skeleton.ts";
import { Kit, parts, thermalOf, type Json, type Row, type V3 } from "./r3-running-child-kit.ts";

/** 🎒️ The strap path with fewer points for the coarser scales. */
export const STRAP_COARSE: V3[] = [STRAP[0]!, STRAP[2]!, [0.279, 0.001, 0.066], STRAP[5]!, STRAP[7]!, STRAP[9]!, STRAP[11]!, STRAP[13]!];

/* ------------------------------------------------------------------ thermal */

/** 🎭️ The face as its own surface (the head solid is half under the hair, which is its own region): a mask over forehead, cheeks, mouth and chin that lies in the surface of the head; finer rows at the finer scales. */
function face(scale: number): Row[] {
  if (scale === 50) return [[-0.051, -0.059, 0, 0.02, 0.011, 2.2], [-0.04, -0.058, 0, 0.035, 0.02, 2.2], [-0.022, -0.055, 0, 0.047, 0.026, 2.2], [0, -0.052, 0, 0.055, 0.03], [0.03, -0.054, 0, 0.059, 0.032], [0.06, -0.06, 0, 0.058, 0.027], [0.082, -0.066, 0, 0.042, 0.013]];
  if (scale === 100) return [[-0.051, -0.059, 0, 0.022, 0.013, 2.2], [-0.028, -0.056, 0, 0.044, 0.025, 2.2], [0.004, -0.053, 0, 0.056, 0.031], [0.045, -0.057, 0, 0.06, 0.03], [0.082, -0.066, 0, 0.042, 0.013]];
  if (scale === 200) return [[-0.049, -0.036, 0, 0.03, 0.015, 2.2], [0, -0.066, 0, 0.049, 0.02], [0.05, -0.07, 0, 0.052, 0.017], [0.082, -0.065, 0, 0.036, 0.015]];
  return [[-0.04, -0.03, 0, 0.034, 0.0375], [0.02, -0.046, 0, 0.05, 0.0375], [0.08, -0.05, 0, 0.04, 0.0375]];
}

/** ✂️ The end faces of a garment or skin segment that are cuts through the continuing body (C25 a), not surface: the hem and collar openings of the shirt, the waist and leg openings of the shorts, the ends of a skin segment under a garment, the two faces where the shoe continues across the toe joint, the back of the face mask inside the head. */
const CUTS: [RegExp, string[]][] = [[/^(th-)?(shirt|collar|shorts|neck|sock|sleeve|shorts-leg)\b/, ["-u", "+u"]], [/^(th-)?(thigh|upper-arm|arm)-/, ["-u"]], [/^(th-)?shank-/, ["+u"]], [/^(sole|shoe)-[lr]$/, ["+u"]], [/^(sole|shoe)-toe-[lr]$/, ["-u"]], [/^th-face$/, ["+v"]]];
const cut = (s: Json): Json => {
  const faces = CUTS.find(([re]) => re.test(s.of ?? s.id))?.[1];
  return faces === undefined ? s : { ...s, sectionFaces: faces };
};

export function thermal(asset: Json, k: Kit, scale: number): Json[] {
  return surfaces(asset, k, scale).map(cut);
}

function surfaces(asset: Json, k: Kit, scale: number): Json[] {
  const regions = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  const out: Json[] = [];
  const t = new Kit();
  for (const s of k.solids) {
    if (s.id === "head" || (scale <= 100 && s.id === "neck")) continue;
    out.push(...thermalOf(s, regions.get(k.region.get(s.id)!)!, scale === 50));
  }
  out.push({ ...t.loft("th-face", "head", SKIN, T.skin, face(scale)), region: "face-skin" });
  if (scale <= 100) out.push({ ...t.loft("th-neck", "neck", SKIN, T.skin, scale === 50 ? [[-0.004, 0.003, 0, 0.042, 0.042], [0.02, 0.002, 0, 0.04, 0.041], [0.045, 0, 0, 0.039, 0.041], [0.062, 0, 0, 0.0395, 0.0415]] : [[-0.005, 0.003, 0, 0.042, 0.042], [0.03, 0.001, 0, 0.0395, 0.041], [0.063, 0, 0, 0.0395, 0.0415]]), region: "face-skin" });
  if (scale >= 200) {
    out.push({ ...t.loft("th-neck", "neck", SKIN, T.skin, scale === 200 ? [[-0.004, 0.003, 0, 0.042, 0.042], [0.03, 0.001, 0, 0.04, 0.041], [0.068, 0, 0, 0.04, 0.042]] : [[-0.006, 0.003, 0, 0.042, 0.042], [0.07, 0, 0, 0.04, 0.042]]), region: "face-skin" });
  }
  const h = { 50: 0.004, 100: 0.0078, 200: 0.0155, 500: 0.031 }[scale]!;
  const outlet = (id: string, at: V3, radii: V3, region: string, emitter: string, material: string): Json => ({ ...t.ell(id, "head", material, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(outlet("th-nose-breath", [scale === 500 ? 0.012 : 0.001, -0.0905 + h, 0], [h, h, Math.max(h, 0.0065)], "face-skin", "nose-breath", SKIN));
  out.push(outlet("th-mouth-breath", [scale === 500 ? -0.012 : -0.02, -0.081 + h, 0], [h, h, Math.max(h, 0.009)], "face-skin", "mouth-breath", SKIN));
  out.push(outlet("th-body-plume", [0.142 - Math.max(h, 0.008), 0.006, 0], [Math.max(h, 0.008), Math.max(0.03, h), Math.max(0.026, h)], "tshirt", "body-plume", HAIR));
  return out;
}

/* ------------------------------------------------------------------ structure */

const B = SKIN;
/** 🦶️ A point given in the upright axes of the foot (x ahead, y up from the ankle) in the frame of the sloping foot bone. */
const ft = (x: number, y: number, w = 0): V3 => {
  const a = (FOOT * Math.PI) / 180;
  return [x * Math.cos(a) - y * Math.sin(a), x * Math.sin(a) + y * Math.cos(a), w];
};

function masses(asset: Json, least = 0): Json[] {
  const k = new Kit();
  const out: Json[] = [];
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < 0.1 * asset.dimensions.massKg) continue;
    const r = Math.max(0.011 * Math.cbrt(b.mass.kg), least);
    out.push(...parts([k.ell(`mass-${b.id}`, b.id, B, 0.4, [b.mass.comX, b.mass.comY ?? 0, 0], [r, r, r])], b.id, "mass"));
  }
  const pack = (asset.rig.bones as Json[]).find((b) => b.id === "pack")!;
  const r = Math.max(0.011 * Math.cbrt(pack.mass.kg), least);
  out.push(...parts([k.ell("mass-pack", "pack", B, 0.4, [pack.mass.comX, pack.mass.comY ?? 0, 0], [r, r, r])], "pack", "mass"));
  return out;
}

/** 🔩️ Joint solids at the real joints: balls at hip and shoulder, hinge pins through knee, elbow, ankle and the toe joints, an ellipsoid at the wrist, discs between the vertebrae at L5/S1 and C7/T1, the condyles under the skull; `f` enlarges them for a coarser scale. */
const JOINT: Record<string, [at: V3, radii: V3]> = { torso: [[0, 0.03, 0], [0.005, 0.014, 0.018]], neck: [[0, 0.016, 0], [0.0045, 0.011, 0.013]], head: [[0, 0.008, 0], [0.005, 0.009, 0.015]], thigh: [[0, 0, 0], [0.0165, 0.0165, 0.0165]], shank: [[0, 0, 0], [0.009, 0.009, 0.03]], foot: [[0, 0, 0], [0.007, 0.007, 0.02]], toe: [[0, 0.018, 0], [0.005, 0.005, 0.028]], arm: [[0, 0, 0], [0.0155, 0.0155, 0.0155]], forearm: [[0, 0, 0], [0.0065, 0.0065, 0.022]], hand: [[0, 0, 0], [0.006, 0.016, 0.008]] };
function joints(ids: readonly string[], f: number, least = 0): Json[] {
  const k = new Kit();
  return ids.flatMap((id) => {
    const [at, r] = JOINT[id.replace(/-[lr]$/, "")]!;
    return parts([k.ell(`joint-${id}`, id, B, 0.5, at, [Math.max(r[0] * f, least), Math.max(r[1] * f, least), Math.max(r[2], r[2] * (1 + (f - 1) / 3), least)])], id, "joint");
  });
}
const sided = (bones: readonly string[]): string[] => bones.flatMap((b) => SIDES.map(([s]) => `${b}-${s}`));
const SPINE: Row[] = [[-0.008, 0.032, 0, 0.017, 0.013], [0.05, 0.024, 0, 0.016, 0.013], [0.11, 0.03, 0, 0.015, 0.012], [0.19, 0.042, 0, 0.013, 0.011], [0.25, 0.036, 0, 0.012, 0.01], [0.294, 0.018, 0, 0.011, 0.01]];
type Put = (member: string, f: (k: Kit) => void) => void;
function builder(out: Json[]): Put {
  return (member, f) => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
}
/** 🎒️ The load path of the backpack: the stiff back panel and the two shoulder straps that hang it from the shoulders (the straps are the body solids themselves where the body has them). */
function pack(put: Put, out: Json[], scale: number): void {
  put("pack", (k) => k.box("sk-pack-panel", "pack", PACK, T.bone, [0.045, { 50: 0.04, 100: 0.043, 200: 0.051, 500: 0.074 }[scale]!, 0], [0.24, { 50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075 }[scale]!, 0.17], scale === 50 ? 0.003 : 0.006));
  if (scale <= 100) for (const [sfx] of SIDES) out.push({ id: `sk-strap-${sfx}`, of: `strap-${sfx}`, member: "pack", part: "member" });
  else
    put("pack", (k) =>
      SIDES.forEach(([sfx, g]) => {
        k.sweep(`sk-strap-${sfx}`, "torso", PACK, T.bone, (scale === 200 ? STRAP_COARSE : [STRAP[0]!, STRAP[3]!, STRAP[7]!, STRAP[10]!, STRAP[13]!]).map(([u, v, w]) => [u, v, w * g] as V3), 0.011, 0.004, undefined, "body", 3).slender = true;
      }),
    );
}

/** 📐 The fit of the shared adult skeleton table to the six-year-old: trunk 0.569, depth 0.6, width 0.63, pelvis 0.62 of the man's; flatter ribs; the shoulder joints 65 mm under the seventh cervical vertebra. */
const FIT: Fit = { material: B, tone: T.bone, ks: 0.569, kv: 0.6, kw: 0.63, dv: 0.003, kp: 0.62, kpv: 0.6, drop: 0.8, shoulder: 0.2255, torso: "torso", pelvis: "pelvis" };
/** 💀 Face mass `[centre, radii]`, jaw joint, jaw angle and chin of the left side in the head bone (the bone starts at the base of the skull). */
const FACE: readonly [readonly [V3, V3], V3, V3, V3] = [[[0.022, -0.048, 0], [0.03, 0.026, 0.044]], [0.012, -0.004, 0.043], [-0.028, -0.008, 0.038], [-0.043, -0.058, 0.008]];
const byBone = (solids: readonly Json[]): Json[] => solids.map((s) => ({ ...s, member: s.bone, part: "member" }));

function structure50(asset: Json): Json[] {
  const out: Json[] = [];
  const put = builder(out);
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.064, 0.004, 0], [0.064, 0.082, 0.063]);
  });
  put("neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, [[-0.012, 0.018, 0, 0.012, 0.011], [0.03, 0.01, 0, 0.0105, 0.0095], [0.06, 0.008, 0, 0.0105, 0.0095], [0.08, 0.012, 0, 0.013, 0.01]]));
  out.push(...parts(faceBones(FIT, "head", 50, FACE[0], FACE[1], FACE[2], FACE[3], 0.0055), "head", "member"));
  out.push(...byBone(axial(FIT, 50)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => {
      k.ell(`sk-femur-head-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.015, 0.015, 0.015]);
      k.cap(`sk-femur-neck-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.02, 0, 0.03 * g], 0.0095, 0.009);
      k.ell(`sk-trochanter-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.02, -0.003, 0.036 * g], [0.018, 0.012, 0.009]);
      k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[0.026, 0, 0.032, 0.0105, 0.0105], [0.07, 0.004, 0.026, 0.0095, 0.0095], [0.15, 0.006, 0.014, 0.009, 0.009], [0.21, 0.004, 0.005, 0.0105, 0.01], [0.238, 0.001, 0.001, 0.019, 0.015], [0.2565, 0, 0, 0.027, 0.021], [0.267, 0, 0, 0.022, 0.016]], g));
      k.ell(`sk-patella-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.252, 0.03, 0], [0.014, 0.0065, 0.013]);
    });
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.007, 0, 0, 0.027, 0.02], [0.012, 0.003, -0.001, 0.02, 0.017], [0.04, 0.007, -0.003, 0.0115, 0.013], [0.12, 0.007, -0.003, 0.009, 0.0105], [0.21, 0.003, -0.003, 0.009, 0.009], [0.245, 0, -0.003, 0.0135, 0.0115], [0.262, 0, -0.005, 0.014, 0.011]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.02, -0.008, 0.022 * g], [0.264, -0.005, 0.017 * g], 0.005, 0.0045);
    });
    put(`foot-${sfx}`, (k) => {
      k.ell(`sk-talus-${sfx}`, `foot-${sfx}`, B, T.bone, ft(0.004, -0.01), [0.019, 0.011, 0.014]);
      k.turned(`sk-calcaneus-${sfx}`, `foot-${sfx}`, B, T.bone, FOOT, [[-0.04, -0.03, 0, 0.011, 0.01], [-0.02, -0.027, 0, 0.013, 0.013], [0.005, -0.024, 0, 0.013, 0.011]]);
      k.turned(`sk-metatarsus-${sfx}`, `foot-${sfx}`, B, T.bone, FOOT, m([[0.012, -0.02, 0, 0.018, 0.011], [0.045, -0.029, 0.001, 0.022, 0.008], [0.07, -0.036, 0.002, 0.027, 0.006], [0.083, -0.038, 0.002, 0.028, 0.0055]], g), undefined, "body", 2.6);
    });
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.018, 0.002, 0.027, 0.0055], [0.022, 0.017, 0, 0.023, 0.005], [0.044, 0.016, -0.002, 0.014, 0.0045]], g), undefined, "body", 2.6));
    put(`arm-${sfx}`, (k) => {
      k.ell(`sk-humerus-head-${sfx}`, `arm-${sfx}`, B, T.bone, [0, 0, -0.002 * g], [0.0145, 0.0145, 0.0145]);
      k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, [[0.01, 0, 0, 0.0105, 0.0105], [0.05, 0.001, 0, 0.0078, 0.0078], [0.14, 0, 0, 0.007, 0.007], [0.17, 0, 0, 0.013, 0.0075], [0.1848, 0, 0, 0.02, 0.009], [0.192, 0, 0, 0.016, 0.008]]);
    });
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.008, -0.008, -0.004 * g], [0.143, -0.01, 0], 0.0065, 0.004);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.008, 0.002, 0.011 * g], [0.143, 0.008, 0], 0.005, 0.0065);
    });
    put(`hand-${sfx}`, (k) => {
      k.ell(`sk-carpus-${sfx}`, `hand-${sfx}`, B, T.bone, [0.009, 0.001, 0], [0.011, 0.016, 0.0065]);
      k.loft(`sk-metacarpus-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.018, 0.002, -0.001, 0.0045, 0.018], [0.064, 0.001, -0.003, 0.004, 0.024]], g), undefined, "body", 3);
      for (const [i, kv, len] of [[1, 0.019, 1], [2, 0.0065, 1.08], [3, -0.006, 1], [4, -0.018, 0.8]] as const) {
        const p1: V3 = [0.064 + 0.024 * len, kv, -0.013 * g];
        k.cap(`sk-phalanx-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, [0.064, kv, -0.003 * g], p1, 0.00375, 0.00375);
        k.cap(`sk-phalanx-tip-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p1, [0.064 + 0.03 * len, kv, (-0.034 * len - 0.002) * g], 0.00375, 0.00375);
      }
      k.cap(`sk-thumb-${sfx}`, `hand-${sfx}`, B, T.bone, [0.018, 0.022, -0.004 * g], [0.066, 0.036, -0.018 * g], 0.004, 0.0038);
    });
  }
  pack(put, out, 50);
  out.push(...joints(["torso", "neck", "head", ...sided(["thigh", "shank", "foot", "toe", "arm", "forearm", "hand"])], 1), ...masses(asset));
  return out;
}

function structure100(asset: Json): Json[] {
  const out: Json[] = [];
  const put = builder(out);
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.063, 0.004, 0], [0.065, 0.082, 0.063]);
  });
  put("neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, [[-0.012, 0.018, 0, 0.012, 0.011], [0.04, 0.009, 0, 0.011, 0.01], [0.08, 0.012, 0, 0.013, 0.0105]]));
  out.push(...parts(faceBones(FIT, "head", 100, FACE[0], FACE[1], FACE[2], FACE[3], 0.0055), "head", "member"));
  out.push(...byBone(axial(FIT, 100)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.014, 0, -0.002, 0.009, 0.009], [0, 0, 0.002, 0.015, 0.015], [0.019, 0, 0.026, 0.015, 0.013], [0.05, 0.003, 0.03, 0.0105, 0.0105], [0.15, 0.006, 0.014, 0.0095, 0.0095], [0.232, 0.002, 0.002, 0.016, 0.014], [0.2565, 0, 0, 0.027, 0.021], [0.267, 0, 0, 0.022, 0.016]], g)));
    put(`shank-${sfx}`, (k) => k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.007, 0, 0, 0.027, 0.02], [0.03, 0.006, -0.002, 0.014, 0.015], [0.18, 0.005, -0.003, 0.0095, 0.01], [0.262, 0, -0.004, 0.015, 0.012]], g)));
    put(`foot-${sfx}`, (k) => k.turned(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, FOOT, m([[-0.04, -0.029, 0, 0.011, 0.011], [-0.018, -0.024, 0, 0.014, 0.015], [0.006, -0.014, 0, 0.016, 0.016], [0.045, -0.028, 0.001, 0.022, 0.009], [0.083, -0.0365, 0.002, 0.028, 0.0075]], g), undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.0195, 0.002, 0.027, 0.0075], [0.044, 0.018, -0.002, 0.016, 0.0075]], g), undefined, "body", 2.6));
    put(`arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, [[-0.013, 0, 0, 0.009, 0.009], [0, 0, 0, 0.0145, 0.0145], [0.025, 0.001, 0, 0.0095, 0.0095], [0.14, 0, 0, 0.0078, 0.0078], [0.1848, 0, 0, 0.02, 0.009], [0.192, 0, 0, 0.016, 0.008]]));
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.008, -0.009, -0.003 * g], [0.143, -0.0105, 0], 0.0078, 0.0075);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.008, 0.007, 0.007 * g], [0.143, 0.0105, 0], 0.0075, 0.0078);
    });
    put(`hand-${sfx}`, (k) => k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.001, 0, 0.0078, 0.015], [0.03, 0.004, -0.001, 0.0075, 0.022], [0.066, 0.003, -0.003, 0.0075, 0.025], [0.09, 0.001, -0.011, 0.0075, 0.023], [0.102, 0, -0.02, 0.0078, 0.019]], g), undefined, "body", 3));
  }
  pack(put, out, 100);
  out.push(...joints(["torso", "neck", "head", ...sided(["thigh", "shank", "foot", "toe", "arm", "forearm", "hand"])], 1.25, 0.0078), ...masses(asset));
  return out;
}

function structure200(asset: Json): Json[] {
  const out: Json[] = [];
  const put = builder(out);
  put("head", (k) => k.loft("sk-skull", "head", B, T.bone, [[-0.046, -0.05, 0, 0.018, 0.016], [-0.015, -0.03, 0, 0.04, 0.045], [0.02, -0.004, 0, 0.056, 0.074], [0.065, 0.004, 0, 0.063, 0.082], [0.105, 0.005, 0, 0.049, 0.062], [0.128, 0.006, 0, 0.02, 0.026]]));
  put("neck", (k) => k.loft("sk-cervical", "neck", B, T.bone, [[-0.012, 0.016, 0, 0.0155, 0.0155], [0.08, 0.01, 0, 0.0155, 0.0155]]));
  out.push(...byBone(axial(FIT, 200)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.016, 0, 0, 0.0155, 0.0155], [0.02, 0, 0.024, 0.0165, 0.0155], [0.06, 0.003, 0.026, 0.0155, 0.0155], [0.2, 0.004, 0.006, 0.0155, 0.0155], [0.2565, 0, 0, 0.027, 0.021], [0.267, 0, 0, 0.022, 0.017]], g)));
    put(`shank-${sfx}`, (k) => k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, [[-0.007, 0, 0, 0.027, 0.02], [0.04, 0.004, 0, 0.0165, 0.016], [0.262, 0, 0, 0.0155, 0.0155]]));
    put(`foot-${sfx}`, (k) => k.turned(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, FOOT, [[-0.04, -0.027, 0, 0.015, 0.0155], [0.004, -0.018, 0, 0.018, 0.019], [0.083, -0.03, 0, 0.028, 0.0155]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.026, 0.002, 0.027, 0.0155], [0.042, 0.022, -0.001, 0.018, 0.0155]], g), undefined, "body", 2.6));
    put(`arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `arm-${sfx}`, B, T.bone, [[-0.016, 0, 0, 0.0155, 0.0155], [0.1, 0, 0, 0.0155, 0.0155], [0.19, 0, 0, 0.019, 0.0155]]));
    put(`forearm-${sfx}`, (k) => k.loft(`sk-forearm-${sfx}`, `forearm-${sfx}`, B, T.bone, [[-0.01, -0.001, 0, 0.0155, 0.0155], [0.145, 0, 0, 0.0125, 0.016]]));
    put(`hand-${sfx}`, (k) => k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.001, 0, 0.0155, 0.0165], [0.05, 0.004, -0.002, 0.0155, 0.024], [0.098, 0.001, -0.015, 0.0155, 0.02]], g), undefined, "body", 3));
  }
  pack(put, out, 200);
  out.push(...joints(sided(["thigh", "shank", "foot", "arm", "forearm"]), 1.6, 0.0155), ...masses(asset, 0.0155));
  return out;
}

/** 🩻️ 1:500: a stick skeleton at the true bone thicknesses. The long bones are slender rods (C2), so they keep their real section instead of swelling to the 75 mm feature, which would turn the skeleton into the body. */
function structure500(asset: Json): Json[] {
  const out: Json[] = [];
  const put = builder(out);
  const rod = (k: Kit, id: string, bone: string, p0: V3, p1: V3, r: number): void => {
    k.cap(id, bone, B, T.bone, p0, p1, r, r).slender = true;
  };
  put("head", (k) => k.loft("sk-skull", "head", B, T.bone, [[-0.04, -0.04, 0, 0.0375, 0.0375], [0.03, -0.002, 0, 0.058, 0.076], [0.09, 0.004, 0, 0.058, 0.076], [0.128, 0.006, 0, 0.0375, 0.0375]]));
  put("neck", (k) => rod(k, "sk-cervical", "neck", [-0.05, 0.022, 0], [0.13, 0.004, 0], 0.0045));
  out.push(...byBone(axial(FIT, 500)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => rod(k, `sk-femur-${sfx}`, `thigh-${sfx}`, [0, 0, 0.012 * g], [0.2565, 0.002, 0], 0.0062));
    put(`shank-${sfx}`, (k) => rod(k, `sk-tibia-${sfx}`, `shank-${sfx}`, [0, 0.002, 0], [0.262, 0, 0], 0.0062));
    put(`foot-${sfx}`, (k) => {
      const foot = k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [
        [...ft(-0.045, -0.03), 0.0041, 0.0039],
        [-0.009, -0.026, 0, 0.0041, 0.0039], [0.001, -0.002, 0, 0.0041, 0.0039],
        [0.026, -0.008, 0, 0.0041, 0.0039], [0.070, 0.006, 0.002 * g, 0.0041, 0.0039],
        [...ft(0.105, -0.04), 0.0041, 0.0039],
      ]);
      foot.slender = true;
      for (const section of foot.sections) {
        delete section.rotation;
        delete section.exponent;
      }
    });
    put(`arm-${sfx}`, (k) => rod(k, `sk-humerus-${sfx}`, `arm-${sfx}`, [0, 0, 0], [0.1848, 0, 0], 0.0045));
    put(`forearm-${sfx}`, (k) => rod(k, `sk-forearm-${sfx}`, `forearm-${sfx}`, [0, 0, 0], [0.1456, 0, 0], 0.0036));
    put(`hand-${sfx}`, (k) => rod(k, `sk-hand-${sfx}`, `hand-${sfx}`, [-0.03, 0, 0], [0.1, 0.001, -0.012 * g], 0.0032));
  }
  pack(put, out, 500);
  out.push(...masses(asset, 0.0375));
  return out;
}

export function structure(asset: Json, scale: number): Json[] {
  return scale === 50 ? structure50(asset) : scale === 100 ? structure100(asset) : scale === 200 ? structure200(asset) : structure500(asset);
}
