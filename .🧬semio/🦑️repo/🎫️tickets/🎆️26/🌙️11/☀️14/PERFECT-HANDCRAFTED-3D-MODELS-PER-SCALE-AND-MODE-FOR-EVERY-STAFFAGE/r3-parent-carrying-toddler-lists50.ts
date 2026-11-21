/** 🌡️ The 1:50 thermal and structural lists of `humans/parent-carrying-toddler`: every region of both bodies as its outer garment or skin with the nested layer shells of real thickness, the four breath outlets and the two plume patches on the surface; the parent's and the toddler's skeletons with joints at the rig joints and masses at the segment centres.
 *
 * @see ./r3-parent-carrying-toddler-s50.ts
 * @see ./design.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, D, FRAMES, H, L, X, aboutU, aboutV, aboutW, boneDeg, capsule, compose, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type V3 } from "./r3-parent-carrying-toddler-kit.ts";
import { HEAD, THEAD, TONE, lean, spine } from "./r3-parent-carrying-toddler-s50.ts";

/* ---------------------------------------------------------------- thermal */

/** 🧅 The layers under the outer garment of a region, outermost first: [layer id, inset from the outer surface]. */
export const SHELLS: Record<string, [string, number][]> = {
  "coat-torso": [["sweater", 0.006], ["torso-air", 0.009]],
  "coat-sleeves": [["sleeve-air", 0.008]],
  jeans: [["leg-air", 0.001]],
  boots: [["boot-air", 0.004], ["sock", 0.012]],
  beanie: [["beanie-air", 0.004]],
  "toddler-hat": [["hat-air", 0.006]],
  "toddler-suit": [["suit-air", 0.012]],
  "toddler-boots": [["boot-air", 0.004], ["sock", 0.01]],
};
export const OUTER: Record<string, string> = { "coat-torso": "coat", "coat-sleeves": "sleeve", jeans: "denim", boots: "boot", beanie: "beanie", scarf: "scarf", gloves: "glove", "toddler-hat": "hat", "toddler-suit": "suit", "toddler-mittens": "mitten", "toddler-boots": "boot" };
const shrink = (q: Json, t: number): Json => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) });
/** 🧅 The shells of a garment loft under its outer layer: the same sections moved inwards by the thickness of the layers outside. */
export function shells(solid: Json, region: string): Json[] {
  return SHELLS[region]!.map(([layer, t], i) => ({ ...solid, id: `th-${solid.id}-${layer}`, tone: Math.min(0.95, Math.round((solid.tone + 0.12 * (i + 1)) * 100) / 100), sections: solid.sections.map((q: Json) => shrink(q, t)), region, layer }));
}
/** 🎯 The region of a body solid of any scale, from its material and id. */
export function regionOf(s: Json): string {
  const tod = s.bone.startsWith("toddler");
  if (s.material === "skin") return tod ? "toddler-face" : /^hair/.test(s.id) ? "beanie" : "face";
  if (s.material === "padded-suit") return "toddler-suit";
  if (s.material === "rubber") return "toddler-boots";
  if (s.material === "knit-hat") return tod ? (/mitten/.test(s.id) ? "toddler-mittens" : "toddler-hat") : /scarf/.test(s.id) ? "scarf" : "beanie";
  if (s.material === "wool") return /sleeve|forearm/.test(s.id) ? "coat-sleeves" : "coat-torso";
  if (s.material === "leather-glove") return "gloves";
  if (s.material === "denim") return "jeans";
  if (s.material === "leather") return "boots";
  throw new Error(`no region for ${s.id}`);
}
/** 🌬️ The breath outlets on the lips and under the nose of both faces and the plume patches on the crowns of the two caps, flush with the surface (D2); `k` widens them at coarser scales to the feature floor. */
export function emitters(k: number, face = "face", tFace = "toddler-face"): Json[] {
  const ph = aboutW(90 - boneDeg("head"));
  const th = aboutW(90 - boneDeg("toddler-head"));
  const tz = FRAMES["toddler-head"]!.o[2];
  const e = (id: string, bone: string, material: string, region: string, tag: string, r: V3, at: V3, rot: number[]): Json => ({ ...ellipsoid({ id, bone, material, tone: 0.5 }, r, at, rot as never), region, tags: ["emitter", tag] });
  const f = Math.max(1, k);
  return [
    e("th-nose-breath", "head", "skin", face, "nose-breath", [0.004 * f, 0.004 * f, Math.max(0.0048, 0.004 * f)], L("head", [0.08, 1.581, 0]), ph),
    e("th-mouth-breath", "head", "skin", face, "mouth-breath", [0.004 * f, 0.004 * f, Math.max(0.0062, 0.004 * f)], L("head", [0.065, 1.5515, 0]), ph),
    e("th-toddler-nose-breath", "toddler-head", "skin", tFace, "toddler-nose-breath", [0.004 * f, 0.004 * f, 0.004 * f], L("toddler-head", [0.262, 1.561, tz]), th),
    e("th-toddler-mouth-breath", "toddler-head", "skin", tFace, "toddler-mouth-breath", [0.004 * f, 0.004 * f, 0.004 * f], L("toddler-head", [0.25, 1.537, tz]), th),
    e("th-plume", "head", "knit-hat", "coat-torso", "plume", [0.004 * f, 0.05, 0.045], L("head", [-0.036, 1.73, 0]), ph),
    e("th-toddler-plume", "toddler-head", "knit-hat", "toddler-suit", "toddler-plume", [0.004 * f, Math.max(0.03, 0.004 * f), Math.max(0.03, 0.004 * f)], L("toddler-head", [0.163, 1.697, tz]), th),
  ];
}
/** 📐 The height of a section of a head loft in the default pose. */
export function heightOf(bone: string, q: Json): number {
  const f = FRAMES[bone]!;
  return f.o[1] + q.at[0] * f.u[1] + q.at[1] * f.v[1];
}

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  for (const s of body) {
    if (s.texture) continue;
    const region = regionOf(s);
    if (/^(neck|toddler-neck)$/.test(s.id)) continue;
    if (s.id === "head" || s.id === "toddler-head") {
      const bone = s.bone;
      const edge = s.id === "head" ? 1.655 : 1.605;
      out.push({ ...s, id: `th-${s.id}`, sections: s.sections.filter((q: Json) => heightOf(bone, q) >= 1.5 && heightOf(bone, q) <= edge + 0.004), capStart: false, capEnd: false, region });
      out.push({ ...s, id: `th-${s.id}-scalp`, sections: s.sections.filter((q: Json) => heightOf(bone, q) >= edge - 0.004), capStart: false, region: s.id === "head" ? "beanie" : "toddler-hat", layer: s.id === "head" ? "beanie-air" : "hat-air", tone: 0.7 });
      continue;
    }
    const layer = OUTER[region];
    out.push(thOf(s.id, region, layer));
    if (SHELLS[region] && s.kind === "loft" && !/cuff|collar|pocket|button|scarf|hair|skirt|heel|sole|pompom|seat/.test(s.id) && !(region === "beanie" && s.id !== "beanie")) out.push(...shells(s, region));
  }
  out.push(...emitters(1));
  return out;
}

/* ---------------------------------------------------------------- structure */

const BONE = { material: "bone", tone: 0.62 };
const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
/** 🦴 A point of the upright trunk (x before the lean) in the torso frame. */
const T = (x: number, y: number, z = 0): V3 => L("torso", [x + lean(y), y, z]);
const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
const ring = (rd: number, r: number): [number, number][] => Array.from({ length: 9 }, (_, k) => [r * Math.cos((k * Math.PI) / 4), rd + r * Math.sin((k * Math.PI) / 4)] as [number, number]);
export const massBall = (member: string, k = 1): Json => {
  const m = kgOf(member);
  const r = Math.max(0.004, 0.0125 * Math.cbrt(m.kg) * k);
  return part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [m.comX, m.comY, 0]), member, "mass");
};
export const jointBall = (member: string, at: V3, r: number, id = `joint-${member}`): Json => part(ellipsoid({ id, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint");
/** 🦴 The spine from the sacral base to C7 (upright x, y, radius): lumbar lordosis, thoracic kyphosis. */
export const SPINE: readonly [number, number, number][] = [[-0.04, 1.012, 0.019], [-0.022, 1.06, 0.019], [-0.014, 1.11, 0.0185], [-0.022, 1.17, 0.018], [-0.045, 1.24, 0.0165], [-0.065, 1.31, 0.015], [-0.072, 1.37, 0.0145], [-0.066, 1.43, 0.0135], [-0.052, 1.47, 0.0125], [-0.042, 1.484, 0.012]];
/** 🫁 Rib hoops, tenth to second rib: [y, x of the centre, depth radius, half-width] upright. */
export const RIBS: readonly [number, number, number, number][] = [[1.17, 0.005, 0.085, 0.118], [1.205, 0.005, 0.09, 0.126], [1.24, 0.004, 0.093, 0.13], [1.275, 0.003, 0.094, 0.131], [1.31, 0.0, 0.092, 0.129], [1.345, -0.004, 0.087, 0.123], [1.375, -0.004, 0.074, 0.112], [1.405, 0.003, 0.064, 0.096], [1.43, 0.003, 0.058, 0.074]];
/** 📐 The plane normal of a rib hoop in the upright figure: ribs fall about 24 degrees from their heads at the spine to the costal cartilages in front. */
export const RIB_NORMAL: V3 = [Math.sin((24 * Math.PI) / 180), Math.cos((24 * Math.PI) / 180), 0];

function parentSkeleton(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(tube({ id: "spine", bone: "torso", ...BONE }, SPINE.map(([x, y]) => T(x, y)), SPINE.map((q) => q[2])), "torso");
  RIBS.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${10 - i}`, bone: "torso", ...BONE }, ring(rd, 0.006), T(x, y), D("torso", RIB_NORMAL), { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.25, 0.085 + lean(1.25), 0.097 + lean(1.25), 0.013), H("torso", 1.33, 0.083 + lean(1.33), 0.095 + lean(1.33), 0.018), H("torso", 1.4, 0.062 + lean(1.4), 0.074 + lean(1.4), 0.024), H("torso", 1.425, 0.05 + lean(1.425), 0.062 + lean(1.425), 0.026)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.05, 1.425, side * 0.022), T(-0.012, 1.448, side * 0.165), 0.0078, 0.0072), "torso");
    m(loft({ id: `scapula-${n}`, bone: "torso", ...BONE }, [st(-0.07, 0, side * -0.022, 0.012, 0.007, 2), st(-0.035, 0, side * -0.01, 0.03, 0.007, 2.2), st(0.005, 0, 0, 0.045, 0.007, 2.4), st(0.045, 0, side * 0.008, 0.05, 0.007, 2.4), st(0.068, 0, side * 0.005, 0.036, 0.007, 2.2)], [true, true], { transform: { translation: T(-0.088, 1.35, side * 0.102), rotation: compose(aboutW(4), aboutU(side * 28)) } }), "torso");
    m(capsule({ id: `scapular-spine-${n}`, bone: "torso", ...BONE }, T(-0.1, 1.4, side * 0.07), T(-0.03, 1.446, side * 0.165), 0.0065, 0.0075), "torso");
  }
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.04, 1.012), P(-0.064, 0.97), P(-0.076, 0.93), P(-0.07, 0.9)], [0.027, 0.025, 0.016, 0.008]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(loft({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [st(-0.06, 0, 0, 0.0085, 0.02, 2), st(-0.03, 0, 0, 0.0085, 0.036, 2.2), st(0.01, 0, 0, 0.0085, 0.058, 2.4), st(0.045, 0, 0, 0.0085, 0.07, 2.6), st(0.062, 0, 0, 0.0085, 0.06, 2.6), st(0.072, 0, 0, 0.0075, 0.034, 2.2)], [true, true], { transform: { translation: P(-0.022, 0.985, side * 0.106), rotation: compose(aboutU(side * 32), aboutV(side * -22)) } }), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.004, 0.94, side * 0.078), P(0.052, 0.915, side * 0.012), 0.012, 0.011), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.008, 0.93, side * 0.076), P(-0.036, 0.865, side * 0.055), 0.013, 0.014), "pelvis");
  }
  const level = aboutW(90 - boneDeg("head"));
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.064, 0.091, 0.067], Hd(-0.03, 1.646), level), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.034, 0.042, 0.052], Hd(0.026, 1.588), level), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, Hd(-0.02, 1.59, side * 0.05), Hd(-0.026, 1.532, side * 0.046), 0.0075, 0.008), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, Hd(-0.026, 1.532, side * 0.046), Hd(0.044, 1.508, side * 0.01), 0.008, 0.009), "head");
  }
  m(tube({ id: "cervical-spine", bone: "head", ...BONE }, [Hd(-0.065, 1.486), Hd(-0.052, 1.52), Hd(-0.042, 1.55), Hd(-0.035, 1.578)], [0.012, 0.0118, 0.0114, 0.011]), "head");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.054], 0.016, 0.017), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.023, 0.017, 0.014], [0.03, -0.008, 0.062]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.058], [0.12, 0.006, 0.045], [0.21, 0.011, 0.03], [0.3, 0.008, 0.015], [0.382, 0.0, 0.004]], [0.015, 0.0138, 0.013, 0.0138, 0.0165]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.021, 0.03, 0.039], [0.392, -0.006, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.021, 0.009, 0.02], [0.39, 0.04, 0]), "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.015, 0.026, 0.037], [0.02, -0.002, 0]), "shank-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.031, 0.004, -0.003], [0.13, 0.012, -0.003], [0.265, 0.01, -0.003], [0.396, 0.002, -0.002]], [0.0165, 0.013, 0.0117, 0.013]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.013, 0.012, 0.009], [0.41, 0.0, -0.026]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.018, 0.033], [0.419, -0.012, 0.029], 0.0066, 0.007), "shank-l");
  const fx = FRAMES["foot-l"]!.o[0];
  const fz = FRAMES["foot-l"]!.o[2];
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, L("foot-l", [fx - 0.05, 0.058, fz]), L("foot-l", [fx - 0.002, 0.07, fz]), 0.02, 0.018), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [X("foot-l", fx - 0.01, 0.06, 0.098, 0.023), X("foot-l", fx + 0.05, 0.047, 0.082, 0.031), X("foot-l", fx + 0.1, 0.033, 0.06, 0.037), X("foot-l", fx + 0.135, 0.025, 0.045, 0.04)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", fx + 0.125, 0.025, 0.045, 0.04), X("toe-l", fx + 0.17, 0.024, 0.04, 0.036), X("toe-l", fx + 0.197, 0.024, 0.036, 0.024)]), "toe-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));

  for (const [side, up, fore] of [["l", 0.04, 0.03], ["r", -0.03, 0.05]] as const) {
    m(ellipsoid({ id: `humerus-head-${side}`, bone: `upper-arm-${side}`, ...BONE }, [0.022, 0.022, 0.022], [0.002, 0, 0]), `upper-arm-${side}`);
    m(tube({ id: `humerus-${side}`, bone: `upper-arm-${side}`, ...BONE }, [[0.014, 0, up * 0.04], [0.105, 0.004, up * 0.33], [0.21, 0.003, up * 0.66], [0.303, 0, up * 0.96]], [0.0115, 0.0104, 0.01, 0.012]), `upper-arm-${side}`);
    m(ellipsoid({ id: `humerus-condyles-${side}`, bone: `upper-arm-${side}`, ...BONE }, [0.012, 0.014, 0.03], [0.312, 0, up]), `upper-arm-${side}`);
    m(capsule({ id: `ulna-${side}`, bone: `forearm-${side}`, ...BONE }, [-0.013, -0.016, 0.0], [0.24, -0.013, fore * 0.97], 0.008, 0.005), `forearm-${side}`);
    m(capsule({ id: `radius-${side}`, bone: `forearm-${side}`, ...BONE }, [0.01, 0.014, 0.0], [0.244, 0.011, fore], 0.0058, 0.0085), `forearm-${side}`);
  }
  for (const hand of ["hand-l", "hand-r"]) {
    const rays = body.filter((b) => b.bone === hand && b.kind === "capsule");
    const carpus: V3 = hand === "hand-l" ? [0.014, 0, 0] : [0.014, 0.006, 0];
    m(ellipsoid({ id: `carpals-${hand.slice(-1)}`, bone: hand, ...BONE }, [0.014, 0.012, 0.016], carpus), hand);
    for (const ray of rays) {
      m(capsule({ id: ray.id.replace("glove-", "bones-"), bone: hand, ...BONE }, ray.p0, ray.p1, 0.0042, 0.0038), hand);
      if (/finger-\d$|thumb$/.test(ray.id)) m(capsule({ id: `${ray.id.replace("glove-", "bones-")}-metacarpal`, bone: hand, ...BONE }, [0, 1, 2].map((k) => carpus[k]! + (ray.p0[k] - carpus[k]!) * 0.25) as V3, ray.p0, 0.0046, 0.0044), hand);
    }
  }
  out.push(jointBall("torso", T(-0.04, 1.012), 0.017));
  out.push(jointBall("head", Hd(-0.065, 1.486), 0.013));
  for (const side of ["l", "r"] as const) {
    out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.024));
    out.push(jointBall(`shank-${side}`, [0, 0, 0], 0.019));
    out.push(jointBall(`foot-${side}`, [0, 0.002, 0], 0.016));
    out.push(jointBall(`toe-${side}`, [-0.004, 0.022, 0], 0.01));
    out.push(jointBall(`upper-arm-${side}`, [0.002, 0, 0], 0.024));
    out.push(jointBall(`forearm-${side}`, [0, 0, 0], 0.015));
    out.push(jointBall(`hand-${side}`, [0, 0, 0], 0.012));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) out.push(massBall(b));
  return out;
}

/** 🧒 The toddler's skeleton: rounded cranium on a short cervical spine, a barrel rib cage of four hoops, a broad shallow pelvis, short long bones with large cartilaginous ends, the feet in the boots and the hands in the mittens. */
function toddlerSkeleton(): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const tz = FRAMES["toddler-torso"]!.o[2];
  const Tt = (dx: number, y: number, z = 0): V3 => L("toddler-torso", [spine(y) + dx, y, tz + z]);
  m(tube({ id: "toddler-spine", bone: "toddler-torso", ...BONE }, [Tt(-0.035, 1.2), Tt(-0.022, 1.26), Tt(-0.024, 1.33), Tt(-0.034, 1.4), Tt(-0.026, 1.47)], [0.012, 0.012, 0.0105, 0.0095, 0.0085]), "toddler-torso");
  [[1.3, 0.06, 0.076], [1.335, 0.062, 0.08], [1.37, 0.06, 0.078], [1.405, 0.052, 0.068]].forEach(([y, rd, rw], i) => m(revolve({ id: `toddler-rib-${i + 1}`, bone: "toddler-torso", ...BONE }, ring(rd!, 0.0045), Tt(0.025, y!), D("toddler-torso", RIB_NORMAL), { lateralScale: Math.round((rw! / rd!) * 1000) / 1000 }), "toddler-torso"));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `toddler-ilium-${n}`, bone: "toddler-torso", ...BONE }, [0.032, 0.036, 0.008], Tt(0.0, 1.235, side * 0.058), compose(aboutU(side * 30))), "toddler-torso");
    m(capsule({ id: `toddler-clavicle-${n}`, bone: "toddler-torso", ...BONE }, Tt(0.035, 1.435, side * 0.012), Tt(0.01, 1.445, side * 0.085), 0.0055, 0.005), "toddler-torso");
  }
  m(capsule({ id: "toddler-pubis", bone: "toddler-torso", ...BONE }, Tt(0.03, 1.19, 0.04), Tt(0.03, 1.19, -0.04), 0.007, 0.007), "toddler-torso");
  const th = aboutW(90 - boneDeg("toddler-head"));
  const hz = FRAMES["toddler-head"]!.o[2];
  m(ellipsoid({ id: "toddler-cranium", bone: "toddler-head", ...BONE }, [0.07, 0.078, 0.062], L("toddler-head", [0.168, 1.618, hz]), th), "toddler-head");
  m(ellipsoid({ id: "toddler-face-bones", bone: "toddler-head", ...BONE }, [0.026, 0.03, 0.042], L("toddler-head", [0.218, 1.55, hz]), th), "toddler-head");
  m(capsule({ id: "toddler-mandible", bone: "toddler-head", ...BONE }, L("toddler-head", [0.16, 1.535, hz]), L("toddler-head", [0.225, 1.513, hz]), 0.008, 0.008), "toddler-head");
  m(capsule({ id: "toddler-cervical-spine", bone: "toddler-head", ...BONE }, L("toddler-head", [0.13, 1.475, hz]), L("toddler-head", [0.15, 1.54, hz]), 0.008, 0.008), "toddler-head");
  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "toddler-femur-l", bone: "toddler-thigh-l", ...BONE }, [0.0, 0, 0], [0.172, -0.002, 0], 0.0095, 0.011), "toddler-thigh-l");
  lm(capsule({ id: "toddler-tibia-l", bone: "toddler-shank-l", ...BONE }, [0.008, 0.004, -0.004], [0.165, 0.002, -0.004], 0.009, 0.008), "toddler-shank-l");
  lm(capsule({ id: "toddler-fibula-l", bone: "toddler-shank-l", ...BONE }, [0.015, -0.01, 0.012], [0.162, -0.008, 0.01], 0.004, 0.004), "toddler-shank-l");
  lm(capsule({ id: "toddler-tarsus-l", bone: "toddler-foot-l", ...BONE }, [-0.03, -0.018, 0], [0.05, -0.024, 0], 0.012, 0.01), "toddler-foot-l");
  lm(capsule({ id: "toddler-metatarsals-l", bone: "toddler-foot-l", ...BONE }, [0.05, -0.024, 0], [0.095, -0.03, 0], 0.0105, 0.008), "toddler-foot-l");
  lm(capsule({ id: "toddler-humerus-l", bone: "toddler-arm-l", ...BONE }, [0.004, 0, 0], [0.128, 0, 0.026], 0.008, 0.0085), "toddler-arm-l");
  lm(capsule({ id: "toddler-radius-ulna-l", bone: "toddler-forearm-l", ...BONE }, [0.004, 0, 0], [0.1, 0, -0.03], 0.0075, 0.007), "toddler-forearm-l");
  lm(capsule({ id: "toddler-hand-bones-l", bone: "toddler-forearm-l", ...BONE }, [0.105, 0.002, -0.03], [0.155, 0.004, -0.031], 0.009, 0.008), "toddler-forearm-l");
  for (const [bone, at, r] of [["toddler-thigh-l", [0, 0, 0], 0.014], ["toddler-shank-l", [0, 0, 0], 0.012], ["toddler-foot-l", [0, 0, 0], 0.01], ["toddler-arm-l", [0, 0, 0], 0.012], ["toddler-forearm-l", [0, 0, 0], 0.01]] as [string, V3, number][]) limb.push(jointBall(bone, at, r));
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  out.push(jointBall("toddler-torso", [0, 0, 0], 0.014, "joint-toddler-seat"));
  out.push(jointBall("toddler-head", [0, 0, 0], 0.011));
  for (const b of (ASSET.rig.bones as Json[]).filter((x) => x.id.startsWith("toddler") && x.mass)) out.push(massBall(b.id, 0.85));
  return out;
}

export function structure50(body: readonly Json[]): Json[] {
  return [...parentSkeleton(body), ...toddlerSkeleton()];
}
export { HEAD, THEAD, TONE };
