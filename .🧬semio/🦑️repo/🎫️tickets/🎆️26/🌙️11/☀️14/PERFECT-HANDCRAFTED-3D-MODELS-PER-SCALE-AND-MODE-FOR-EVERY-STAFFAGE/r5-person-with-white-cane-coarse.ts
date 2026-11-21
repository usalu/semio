/** 🚶‍♀️ The 1:100, 1:200 and 1:500 models of `humans/person-with-white-cane`, each stated anew for its scale: fewer and longer sections, merged masses, the same woman in the same stance with her bag and her cane. Smallest feature 15, 30 and 75 mm.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `constant-contact-stance`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r5-person-with-white-cane-s50.ts
 * @see ./r3-humans-person-with-white-cane.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, H, aboutU, aboutV, aboutW, boneDeg, box, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r5-person-with-white-cane-kit.ts";
import { B, BAG, HEM, between, BONE, CANE, Hd, MAT, N, OUTER, OUTLET, P, STRAP, strapPath, T, TONE, WRIST_W, breath, c, caneBody, caneMember, headY, hs, jointBall, legTubes, level, massBall, near, onCane, plume, ribbon, ring, support, xa, type Common, type Row } from "./r5-person-with-white-cane-s50.ts";

const pair = (out: Json[], solids: Json[]): void => {
  for (const s of solids) out.push(s);
  for (const s of solids) out.push(mirrored(s, toRight, near));
};
const skin = (id: string, bone = "head", tone: number = TONE.skin): Common => c(id, bone, MAT.skin, tone);
const denim = (id: string, bone = "torso", tone: number = TONE.jacket, role = "body"): Common => c(id, bone, MAT.jacket, tone, role);
const jeans = (id: string, bone: string, tone: number = TONE.jeansFar): Common => c(id, bone, MAT.jeans, tone);
const boot = (id: string, bone: string): Common => c(id, bone, MAT.boot, TONE.bootFar);
const leather = (id: string, tone: number = TONE.bag): Common => c(id, "torso", MAT.bag, tone);
const upright = aboutW(90 - boneDeg("torso"));
const pick = (path: readonly V3[], index: readonly number[]): V3[] => index.map((i) => path[i]!);
const END = CANE.end;
/** 🦯 The cane of a coarse scale as slender rods at their true 12 mm: the grip length, the white shaft and the red end. */
const rods = (bandFrom: number, tipEnd: number, band: string): Json[] => [
  capsule(c("cane-grip", "cane", MAT.grip, TONE.grip), [CANE.top + 0.006, 0, 0], [0.19, 0, 0], CANE.shaft, CANE.shaft, { slender: true }),
  capsule(c("cane-shaft", "cane", MAT.cane, TONE.cane), [0.19, 0, 0], [bandFrom, 0, 0], CANE.shaft, CANE.shaft, { slender: true }),
  capsule(c(band, "cane", MAT.cane, TONE.band), [bandFrom, 0, 0], [tipEnd, 0, 0], CANE.shaft, CANE.shaft, { slender: true }),
];
/** 🕶️ The sunglasses of a coarse scale: one dark band across both eyes. */
const visor = (half: number, thick: number, wrap = false): Json => ({ ...c("sunglasses", "head", MAT.lens, TONE.lens, "glazing"), kind: "loft", sections: (wrap ? [[0.063, -0.061], [0.081, -0.054], [0.1095, -0.03], [0.119, 0], [0.1095, 0.03], [0.081, 0.054], [0.063, 0.061]] : [-0.062, -0.031, 0, 0.031, 0.062].map((z) => [0.1135 - Math.abs(z) * 0.2, z])).map(([x, z]) => ({ at: Hd(x!, 1.597, z!).map((q) => Math.round(q * 1e5) / 1e5), rx: half, ry: thick, exponent: 2.6 })), capStart: true, capEnd: true });

/* ---------------------------------------------------------------- 1:100 */

const JACKET_100: readonly Row[] = [
  [0.953, -0.12, 0.112, 0.179, 2.5],
  [1.0, -0.118, 0.111, 0.177, 2.5],
  [1.09, -0.1, 0.11, 0.158, 2.4],
  [1.17, -0.101, 0.132, 0.16, 2.4],
  [1.25, -0.109, 0.159, 0.168, 2.4],
  [1.33, -0.112, 0.137, 0.181, 2.3],
  [1.38, -0.104, 0.11, 0.197, 2.3],
  [1.412, -0.09, 0.094, 0.186, 2.25],
  [1.438, -0.068, 0.077, 0.14, 2.1],
  [1.456, -0.052, 0.064, 0.09, 2],
];
const HEAD_100: readonly Row[] = [[1.483, 0.064, 0.101, 0.018, 2], [1.5, 0.018, 0.109, 0.037, 2], [1.53, -0.032, 0.111, 0.052, 2], [1.575, -0.063, 0.111, 0.063, 2], [1.618, -0.071, 0.115, 0.07, 2], [1.655, -0.066, 0.108, 0.069, 2], [1.682, -0.048, 0.083, 0.052, 2], [1.698, -0.01, 0.032, 0.017, 2]];
const HAIR_100: readonly Row[] = [[1.495, -0.083, 0.02, 0.07, 2.05], [1.515, -0.094, 0.033, 0.085, 2.0], [1.57, -0.096, 0.041, 0.089, 2.0], [1.632, -0.09, 0.056, 0.0865, 2.0], [1.664, -0.081, 0.112, 0.081, 2.0], [1.69, -0.06, 0.087, 0.066, 2.0], [1.708, -0.013, 0.028, 0.026, 2.0]];

export function body100(): Json[] {
  const out: Json[] = [];
  out.push(loft(jeans("hips", "pelvis", TONE.jeans), hs("pelvis", [[0.822, -0.07, 0.058, 0.108, 2.3], [0.85, -0.104, 0.08, 0.166, 2.4], [0.88, -0.122, 0.094, 0.184, 2.4], [0.92, -0.128, 0.103, 0.186, 2.4], [0.98, -0.11, 0.104, 0.169, 2.3], [1.04, -0.094, 0.103, 0.153, 2.3]])));
  out.push(loft(denim("jacket"), dense(hs("torso", JACKET_100), 2)));
  const leg: Json[] = [];
  leg.push(loft(jeans("thigh-l", "thigh-l"), dome(dense([st(0.0, -0.008, 0.005, 0.088, 0.1, 2.1), st(0.14, 0.0, 0.004, 0.087, 0.093, 2), st(0.29, 0.006, -0.002, 0.067, 0.074, 2), st(0.4116, 0.004, -0.005, 0.057, 0.062, 2)], 2), 1, 0.036, 2)));
  leg.push(loft(jeans("shank-l", "shank-l"), dome(dense([st(0.0, 0.004, -0.005, 0.057, 0.062, 2), st(0.12, -0.008, -0.004, 0.054, 0.0585, 2), st(0.27, -0.005, -0.001, 0.043, 0.049, 2), st(0.362, 0.0, 0, 0.04, 0.045, 2)], 2), -1, 0.036, 2)));
  leg.push(loft(boot("boot-shaft-l", "shank-l"), [st(0.33, 0.0, 0, 0.036, 0.041, 2), st(0.39, 0.001, 0, 0.0355, 0.042, 2), st(0.432, 0.004, 0, 0.035, 0.044, 2)]));
  leg.push(loft(boot("boot-l", "foot-l"), dense(xa("foot-l", [[-0.0635, 0.012, 0.104, 0.019, 2], [-0.05, 0, 0.12, 0.032, 2.3], [0.0, 0, 0.128, 0.0375, 2.3], [0.04, 0, 0.108, 0.039, 2.3], [0.09, 0, 0.073, 0.045, 2.5], [0.146, 0, 0.055, 0.0485, 2.6]]), 2)));
  leg.push(loft(boot("boot-toe-l", "toe-l"), xa("toe-l", [[0.126, 0, 0.06, 0.0485, 2.6, 0], [0.185, 0, 0.042, 0.038, 2.4, -0.004], [0.2055, 0.002, 0.026, 0.014, 2, -0.007]])));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft(denim("sleeve-l", "arm-l", TONE.jacketFar), dome(dome(dense([st(-0.02, 0.0, -0.028, 0.036, 0.05), st(0.0, 0.0, -0.009, 0.05, 0.059), st(0.07, 0.003, 0, 0.054, 0.059), st(0.22, 0.0, 0, 0.047, 0.052), st(0.3125, -0.003, 0, 0.044, 0.049)], 2), 1, 0.038, 2), -1, 0.009, 2)));
  arm.push(loft(denim("forearm-l", "forearm-l", TONE.jacketFar), dome([st(0.0, -0.003, 0.0, 0.044, 0.049), st(0.07, 0.0, 0, 0.0425, 0.046), st(0.16, 0.0, 0, 0.038, 0.0395), st(0.236, 0, 0, 0.037, 0.0375)], -1, 0.038, 2)));
  arm.push(loft(c("wrist-l", "forearm-l", MAT.skin, TONE.handFar), [st(0.215, 0, 0, 0.02, 0.0275), st(0.262, 0, 0, 0.0185, 0.0265)]));
  pair(out, arm);
  for (const s of out) if (s.id === "wrist-r") s.tone = TONE.hand;
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.01, 0, 0, 0.0185, 0.0265), st(0.04, 0.002, 0, 0.0155, 0.039), st(0.09, 0, -0.004, 0.0125, 0.04), st(0.138, 0, -0.015, 0.0105, 0.037), st(0.172, 0, -0.03, 0.009, 0.03)], 1, 0.007, 2)));
  out.push(capsule(skin("hand-l-thumb", "hand-l", TONE.handFar), [0.02, 0.029, -0.009], [0.09, 0.058, -0.022], 0.011, 0.0088));
  out.push(loft(skin("hand-r", "hand-r", TONE.hand), dome([st(-0.01, 0, 0, 0.0185, 0.0265), st(0.04, 0.002, 0, 0.015, 0.039), st(0.093, 0, 0.002, 0.012, 0.039)], 1, 0.007, 2)));
  out.push(capsule(skin("hand-r-index", "hand-r", TONE.hand), [0.088, 0.03, 0.005], onCane(0.132, 0.016, -0.011), 0.009, 0.0078));
  out.push(ellipsoid(skin("hand-r-fingers", "hand-r", TONE.hand), [0.026, 0.036, 0.021], [0.072, -0.004, 0.03], aboutW(ASSET.rig.bones.find((b: Json) => b.id === "cane").angleDeg)));
  out.push(capsule(skin("hand-r-thumb", "hand-r", TONE.hand), [0.018, 0.03, 0.006], onCane(0.03, 0.0215, 0.003), 0.011, 0.0085));

  out.push(loft(skin("neck", "neck", 0.76), hs("neck", [[1.425, -0.056, 0.044, 0.056, 2], [1.495, -0.047, 0.053, 0.051, 2], [1.565, -0.038, 0.06, 0.05, 2]])));
  out.push(loft(denim("collar", "neck", 0.4), [st(-0.034, 0.004, 0, 0.094, 0.086, 2.1, -18), st(0.012, -0.008, 0, 0.081, 0.074, 2.1, -18)]));
  out.push(loft(skin("head"), dome(dense(hs("head", HEAD_100), 2), -1, 0.005, 2)));
  out.push(loft(c("hair", "head", MAT.hair, TONE.hair), dome(dense(hs("head", HAIR_100), 2), -1, 0.006, 2)));
  out.push(loft(skin("head-nose"), [H("head", 1.5535, 0.094, 0.123, 0.0152), H("head", 1.563, 0.094, 0.13, 0.014), H("head", 1.6, 0.094, 0.111, 0.008)]));
  out.push(visor(0.0195, 0.0042));

  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth], B(BAG.x, BAG.y, BAG.z), upright, 0.018));
  out.push(box(leather("bag-flap", 0.44), [0.142, BAG.length - 0.004, 0.016], B(BAG.x, BAG.y + BAG.height / 2 - 0.069, BAG.z - BAG.depth / 2 - 0.004), upright, 0.006));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("torso", strapPath(0.006), 0.0145, 0.003), [true, true], { slender: true }));
  out.push(...caneBody(100));
  return out;
}

/** 🌡️ The thermal list of a coarser scale: every garment by reference in its outer layer, the skin of the face as the head below the hairline without its hair-covered back, the hips below the jacket hem, the breath outlets and the plume outlet. */
function thermalOf(scale: number, body: readonly Json[]): Json[] {
  const out: Json[] = [];
  for (const s of body) {
    if (s.id === "hips") out.push({ ...s, id: "th-hips", sections: between(s.sections, -1, HEM), capEnd: false, region: "jeans", layer: OUTER.jeans });
    else if (s.material === MAT.jacket) out.push(thOf(s.id, "jacket", OUTER.jacket));
    else if (s.material === MAT.jeans) out.push(...legTubes(s, false).map((t) => ({ ...t, layer: OUTER.jeans })));
    else if (s.material === MAT.boot) out.push(thOf(s.id, "boots", OUTER.boots, s.id.startsWith("boot-shaft") ? { sectionFaces: ["-u"] } : {}));
    else if (s.material === MAT.hair) out.push(thOf(s.id, "hair", OUTER.hair));
    else if (s.material === MAT.bag) out.push(thOf(s.id, "bag"));
    else if (s.material === MAT.lens) out.push(thOf(s.id, "sunglasses"));
    else if (s.material === MAT.skin && /^(hand|wrist)/.test(s.id)) out.push(thOf(s.id, "hands"));
    else if (s.material === MAT.skin && /^head-/.test(s.id)) out.push(thOf(s.id, "face-skin"));
    else if (s.id === "neck" && scale <= 200) out.push({ ...loft({ id: "th-neck", bone: "neck", material: MAT.skin, tone: 0.76 }, hs("neck", [[1.478, -0.049, 0.051, 0.0515, 2], [1.52, -0.044, 0.056, 0.05, 2]]), [false, false]), region: "face-skin" });
    else if (s.bone === "cane") out.push(thOf(s.id, s.material === MAT.grip ? "cane-grip" : s.material === MAT.roller || s.id === "cane-band-tip" ? "cane-tip" : "cane-shaft"));
  }
  const head = body.find((b) => b.id === "head")!;
  out.push({ ...head, id: "th-head", sections: head.sections.filter((q: Json) => headY(q) <= 1.66), capStart: true, capEnd: false, sectionFaces: ["+v"], region: "face-skin" });
  out.push(...breath(scale), plume(scale === 100 ? 0.008 : scale === 200 ? 0.015 : 0.0375));
  return out;
}
export const thermal100 = (body: readonly Json[]): Json[] => thermalOf(100, body);

export function structure100(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const spine: [number, number, number][] = [[-0.032, 1.0, 0.019], [-0.006, 1.08, 0.018], [-0.022, 1.19, 0.016], [-0.052, 1.31, 0.014], [-0.04, 1.4, 0.013], [-0.02, 1.464, 0.012]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const hoops: [number, number, number, number][] = [[1.14, 0.01, 0.08, 0.111], [1.2, 0.009, 0.088, 0.122], [1.26, 0.006, 0.087, 0.123], [1.322, -0.001, 0.076, 0.11], [1.378, -0.01, 0.054, 0.076]];
  hoops.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${5 - i}`, bone: "torso", ...BONE }, ring(rd, 0.0078), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.235, 0.082, 0.098, 0.014), H("torso", 1.315, 0.077, 0.093, 0.018), H("torso", 1.384, 0.047, 0.063, 0.024)]), "torso");
  const side: Json[] = [];
  const sm = (solid: Json, member: string): void => { side.push(part(solid, member, "member")); };
  sm(capsule({ id: "clavicle-l", bone: "torso", ...BONE }, T(0.05, 1.39, 0.02), T(-0.004, 1.414, 0.168), 0.0076, 0.0076), "torso");
  sm(ellipsoid({ id: "scapula-l", bone: "torso", ...BONE }, [0.066, 0.0076, 0.044], T(-0.076, 1.332, 0.096), compose(aboutW(4), aboutU(28))), "torso");
  sm(ellipsoid({ id: "ilium-l", bone: "pelvis", ...BONE }, [0.056, 0.07, 0.011], P(-0.02, 0.972, 0.116), compose(aboutU(36), aboutV(-24))), "pelvis");
  sm(capsule({ id: "ischiopubic-l", bone: "pelvis", ...BONE }, P(0.002, 0.902, 0.08), P(0.03, 0.852, 0.03), 0.012, 0.012), "pelvis");
  sm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.052], 0.0155, 0.0165), "thigh-l");
  sm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.038, -0.005, 0.056], [0.17, 0.01, 0.034], [0.3, 0.008, 0.012], [0.394, 0.0, -0.002]], [0.016, 0.0132, 0.0134, 0.016]), "thigh-l");
  sm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.02, 0.029, 0.038], [0.405, -0.004, -0.005]), "thigh-l");
  sm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.008, 0.0, -0.005], [0.048, 0.006, -0.006], [0.19, 0.011, -0.004], [0.398, 0.002, -0.002]], [0.028, 0.016, 0.0118, 0.0136]), "shank-l");
  sm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.017, 0.027], [0.413, -0.011, 0.027], 0.0075, 0.0075), "shank-l");
  sm(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, xa("foot-l", [[-0.046, 0.03, 0.068, 0.019], [0.0, 0.038, 0.086, 0.024], [0.075, 0.028, 0.06, 0.034], [0.134, 0.014, 0.035, 0.039]])), "foot-l");
  sm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, xa("toe-l", [[0.13, 0.014, 0.035, 0.039], [0.196, 0.012, 0.027, 0.025, 2, -0.005]])), "toe-l");
  sm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.0, 0, 0], [0.028, 0, 0], [0.19, 0.004, 0], [0.305, 0, 0]], [0.0215, 0.012, 0.01, 0.014]), "arm-l");
  sm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.011, -0.016, 0], [0.238, -0.013, 0], 0.008, 0.0075), "forearm-l");
  sm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.009, 0.015, 0], [0.242, 0.011, 0], 0.0075, 0.0085), "forearm-l");
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.032, 1.0), P(-0.062, 0.95), P(-0.072, 0.892)], [0.027, 0.024, 0.009]), "pelvis");
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.061, 0.087, 0.066], Hd(0.02, 1.632), level), "head");
  m(ellipsoid({ id: "jaw", bone: "head", ...BONE }, [0.037, 0.043, 0.048], Hd(0.068, 1.535), compose(level, aboutW(-14))), "head");
  const cervical: [number, number][] = [[-0.02, 1.464], [-0.004, 1.508], [0.012, 1.548]];
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, cervical.map(([x, y]) => N(x, y)), [0.012, 0.011, 0.0105]), "neck");
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.0085, 0.015), st(0.046, 0.002, 0, 0.0078, 0.03), st(0.092, 0, -0.005, 0.0075, 0.032), st(0.158, 0, -0.034, 0.0075, 0.022)]), "hand-l");
  m(loft({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [st(0.0, 0, 0, 0.0085, 0.015), st(0.046, 0.002, 0, 0.0078, 0.03), st(0.092, 0, 0.002, 0.0075, 0.03)]), "hand-r");
  m(capsule({ id: "index-bones-r", bone: "hand-r", ...BONE }, [0.088, 0.03, 0.005], onCane(0.13, 0.016, -0.011), 0.0075, 0.0075), "hand-r");
  out.push(...caneMember(body, 0.0095), support(0.0085));
  out.push(jointBall("torso", T(-0.032, 1.0), 0.016), jointBall("neck", N(-0.02, 1.464), 0.0125), jointBall("head", Hd(0.012, 1.548), 0.0115));
  for (const s of ["l", "r"]) {
    out.push(jointBall(`thigh-${s}`, [0, 0, 0], 0.023), jointBall(`shank-${s}`, [0, 0, s === "l" ? -0.005 : 0.005], 0.018), jointBall(`foot-${s}`, [0, 0.002, 0], 0.015), jointBall(`toe-${s}`, [-0.004, 0.022, 0], 0.0095));
    out.push(jointBall(`arm-${s}`, [0.002, 0, 0], 0.023), jointBall(`forearm-${s}`, [0, 0, 0], 0.0145), jointBall(`hand-${s}`, [0, 0, 0], 0.0115));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) out.push(massBall(b));
  return out;
}

/* ---------------------------------------------------------------- 1:200 */

const JACKET_200: readonly Row[] = [
  [0.953, -0.12, 0.112, 0.179, 2.5],
  [1.02, -0.113, 0.111, 0.173, 2.45],
  [1.1, -0.1, 0.111, 0.158, 2.4],
  [1.25, -0.109, 0.158, 0.168, 2.4],
  [1.35, -0.111, 0.128, 0.188, 2.3],
  [1.39, -0.1, 0.105, 0.197, 2.3],
  [1.43, -0.076, 0.083, 0.158, 2.15],
  [1.456, -0.052, 0.064, 0.09, 2],
];

export function body200(): Json[] {
  const out: Json[] = [];
  out.push(loft(jeans("hips", "pelvis", TONE.jeans), hs("pelvis", [[0.822, -0.07, 0.058, 0.108, 2.3], [0.862, -0.115, 0.088, 0.178, 2.4], [0.9, -0.127, 0.1, 0.186, 2.4], [0.975, -0.112, 0.104, 0.17, 2.3], [1.04, -0.094, 0.103, 0.153, 2.3]])));
  out.push(loft(denim("jacket"), hs("torso", JACKET_200)));
  const leg: Json[] = [];
  leg.push(loft(jeans("thigh-l", "thigh-l"), dome([st(0.0, -0.008, 0.005, 0.088, 0.1, 2.1), st(0.18, 0.002, 0.003, 0.084, 0.088, 2), st(0.4116, 0.004, -0.005, 0.057, 0.062, 2)], 1, 0.036, 2)));
  leg.push(loft(jeans("shank-l", "shank-l"), dome([st(0.0, 0.004, -0.005, 0.057, 0.062, 2), st(0.13, -0.007, -0.004, 0.053, 0.058, 2), st(0.3, -0.003, 0, 0.042, 0.047, 2), st(0.425, 0.003, 0, 0.038, 0.045, 2)], -1, 0.036, 2)));
  leg.push(loft(boot("boot-l", "foot-l"), xa("foot-l", [[-0.0635, 0.01, 0.1, 0.02, 2], [-0.04, 0, 0.122, 0.035, 2.3], [0.02, 0, 0.122, 0.038, 2.3], [0.09, 0, 0.073, 0.045, 2.5], [0.146, 0, 0.055, 0.0485, 2.6]])));
  leg.push(loft(boot("boot-toe-l", "toe-l"), xa("toe-l", [[0.126, 0, 0.06, 0.0485, 2.6, 0], [0.188, 0, 0.042, 0.037, 2.4, -0.004], [0.2055, 0, 0.032, 0.018, 2, -0.007]])));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft(denim("sleeve-l", "arm-l", TONE.jacketFar), dome(dome([st(-0.018, 0.0, -0.026, 0.038, 0.052), st(0.03, 0.002, 0, 0.054, 0.06), st(0.17, 0.002, 0, 0.05, 0.055), st(0.3125, -0.003, 0, 0.044, 0.049)], 1, 0.038, 2), -1, 0.009, 2)));
  arm.push(loft(denim("forearm-l", "forearm-l", TONE.jacketFar), dome([st(0.0, -0.003, 0.0, 0.044, 0.049), st(0.12, 0.0, 0, 0.04, 0.043), st(0.24, 0, 0, 0.037, 0.0375)], -1, 0.038, 2)));
  pair(out, arm);
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.014, 0, 0, 0.019, 0.028), st(0.045, 0.004, 0, 0.017, 0.042), st(0.1, 0.004, -0.006, 0.0155, 0.041), st(0.17, 0, -0.028, 0.015, 0.031)], 1, 0.007, 2)));
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.058, 0.041, 0.027], [0.058, 0.002, 0.012]));
  out.push(loft(skin("neck", "neck", 0.76), hs("neck", [[1.425, -0.056, 0.044, 0.056, 2], [1.565, -0.038, 0.06, 0.05, 2]])));
  out.push(loft(skin("head"), dome(hs("head", [[1.483, 0.056, 0.103, 0.022, 2], [1.508, 0.0, 0.11, 0.043, 2], [1.56, -0.057, 0.113, 0.06, 2], [1.618, -0.071, 0.116, 0.07, 2], [1.668, -0.06, 0.1, 0.064, 2], [1.698, -0.016, 0.04, 0.022, 2]]), -1, 0.005, 2)));
  out.push(loft(c("hair", "head", MAT.hair, TONE.hair), dense(hs("head", [[1.495, -0.084, 0.022, 0.072, 2.05], [1.56, -0.096, 0.041, 0.089, 2.0], [1.635, -0.09, 0.058, 0.0865, 2.0], [1.664, -0.081, 0.112, 0.081, 2.0], [1.708, -0.016, 0.034, 0.03, 2.0]]), 2)));
  out.push(visor(0.024, 0.0042, true));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth + 0.008], B(BAG.x, BAG.y, BAG.z - 0.004), upright, 0.018));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("torso", pick(strapPath(0.009), [0, 1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 14, 16, 17, 18]), 0.0145, 0.003), [true, true], { slender: true }));
  out.push(...rods(CANE.band, END - 0.026, "cane-band"));
  out.push(ellipsoid(c("cane-roller", "cane", MAT.roller, TONE.roller), [0.017, 0.015, 0.015], [END - 0.0165, 0, 0]));
  return out;
}
export const thermal200 = (body: readonly Json[]): Json[] => thermalOf(200, body);

export function structure200(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.086, 0.086, 0.065], Hd(0.026, 1.61), compose(level, aboutW(-14))), "head");
  m(capsule({ id: "cervical-spine", bone: "neck", ...BONE }, N(-0.02, 1.464), N(0.012, 1.548), 0.015, 0.015), "neck");
  const spine: [number, number][] = [[-0.032, 1.0], [-0.006, 1.09], [-0.048, 1.28], [-0.02, 1.464]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), [0.019, 0.018, 0.016, 0.015]), "torso");
  const hoops: [number, number, number, number][] = [[1.16, 0.01, 0.08, 0.113], [1.25, 0.007, 0.084, 0.119], [1.345, -0.004, 0.062, 0.09]];
  hoops.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${3 - i}`, bone: "torso", ...BONE }, ring(rd, 0.015), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(-0.002, 1.405, -0.165), T(-0.002, 1.405, 0.165), 0.015, 0.015), "torso");
  m(revolve({ id: "pelvic-ring", bone: "pelvis", ...BONE }, ring(0.078, 0.017), P(-0.02, 0.945), [1, 0, 0], { lateralScale: 1.55 }), "pelvis");
  const side: Json[] = [];
  const sm = (solid: Json, member: string): void => { side.push(part(solid, member, "member")); };
  sm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0, 0, 0], [0.038, -0.004, 0.052], [0.19, 0.01, 0.03], [0.402, 0, -0.004]], [0.019, 0.016, 0.015, 0.019]), "thigh-l");
  sm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.01, 0.002, -0.004], [0.4, 0.002, 0], 0.019, 0.015), "shank-l");
  sm(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, xa("foot-l", [[-0.044, 0.028, 0.074, 0.021], [0.016, 0.034, 0.086, 0.028], [0.134, 0.009, 0.041, 0.039]])), "foot-l");
  sm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, xa("toe-l", [[0.13, 0.008, 0.039, 0.039], [0.195, 0.006, 0.036, 0.026, 2, -0.005]])), "toe-l");
  sm(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0, 0, 0], [0.306, 0, 0], 0.018, 0.015), "arm-l");
  sm(capsule({ id: "radius-ulna-l", bone: "forearm-l", ...BONE }, [0, 0, 0], [0.24, 0, 0], 0.016, 0.015), "forearm-l");
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.015, 0.016), st(0.055, 0.002, 0, 0.015, 0.03), st(0.15, 0, -0.028, 0.015, 0.022)]), "hand-l");
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.046, 0.03, 0.015], [0.056, 0.002, 0.008]), "hand-r");
  out.push(...caneMember(body, 0.015).filter((s) => s.id !== "member-cane-core"), { id: "member-cane-grip", of: "cane-grip", member: "cane", part: "member" }, support(0.015));
  out.push(jointBall("neck", N(-0.02, 1.464), 0.017));
  for (const s of ["l", "r"]) {
    out.push(jointBall(`thigh-${s}`, [0, 0, 0], 0.023), jointBall(`shank-${s}`, [0, 0, s === "l" ? -0.005 : 0.005], 0.021), jointBall(`foot-${s}`, [0, 0.002, 0], 0.017));
    out.push(jointBall(`arm-${s}`, [0.002, 0, 0], 0.023), jointBall(`forearm-${s}`, [0, 0, 0], 0.018), jointBall(`hand-${s}`, [0, 0, 0], 0.016));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r"]) out.push(massBall(b, 1.1));
  return out;
}

/* ---------------------------------------------------------------- 1:500 */

export function body500(): Json[] {
  const out: Json[] = [];
  out.push(loft(jeans("hips", "pelvis", TONE.jeans), hs("pelvis", [[0.83, -0.078, 0.064, 0.118, 2.3], [0.87, -0.118, 0.092, 0.182, 2.4], [0.91, -0.127, 0.101, 0.186, 2.4], [1.03, -0.097, 0.103, 0.156, 2.3]])));
  out.push(loft(denim("jacket"), hs("torso", [[0.953, -0.12, 0.112, 0.179, 2.5], [1.09, -0.101, 0.112, 0.159, 2.4], [1.25, -0.109, 0.157, 0.168, 2.4], [1.38, -0.105, 0.11, 0.196, 2.3], [1.425, -0.08, 0.086, 0.168, 2.2], [1.456, -0.054, 0.066, 0.096, 2]])));
  const leg: Json[] = [];
  leg.push(loft(jeans("thigh-l", "thigh-l"), dome([st(0.0, -0.007, 0.005, 0.088, 0.1, 2.1), st(0.2, 0.002, 0.002, 0.083, 0.087, 2), st(0.4116, 0.004, -0.005, 0.058, 0.063, 2)], 1, 0.036, 2)));
  leg.push(loft(jeans("shank-l", "shank-l"), dome([st(0.0, 0.004, -0.005, 0.058, 0.063, 2), st(0.15, -0.006, -0.003, 0.052, 0.057, 2), st(0.425, 0.003, 0, 0.039, 0.046, 2)], -1, 0.036, 2)));
  leg.push(loft(boot("boot-l", "foot-l"), xa("foot-l", [[-0.0635, 0, 0.1, 0.026, 2], [0.0, 0, 0.122, 0.039, 2.3], [0.1, 0, 0.07, 0.05, 2.5], [0.175, 0, 0.05, 0.047, 2.5], [0.2055, 0, 0.038, 0.03, 2]])));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft(denim("sleeve-l", "arm-l", TONE.jacketFar), dome(dome([st(-0.014, 0.0, -0.02, 0.042, 0.054), st(0.06, 0.002, 0, 0.054, 0.06), st(0.3125, -0.003, 0, 0.045, 0.05)], 1, 0.038, 2), -1, 0.009, 2)));
  arm.push(loft(denim("forearm-l", "forearm-l", TONE.jacketFar), dome([st(0.0, -0.003, 0.0, 0.045, 0.05), st(0.24, 0, 0, 0.039, 0.04)], -1, 0.038, 2)));
  pair(out, arm);
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.014, 0, 0, 0.038, 0.039), st(0.065, 0.004, -0.002, 0.038, 0.043), st(0.152, 0, -0.008, 0.038, 0.039)], 1, 0.009, 2)));
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.072, 0.06, 0.054], [0.062, 0.002, 0.012]));
  out.push(loft(skin("neck", "neck", 0.76), hs("neck", [[1.43, -0.056, 0.046, 0.056, 2], [1.56, -0.04, 0.06, 0.05, 2]])));
  out.push(loft(skin("head"), dome(dense(hs("head", [[1.483, 0.04, 0.104, 0.03, 2], [1.54, -0.046, 0.113, 0.057, 2], [1.618, -0.071, 0.116, 0.07, 2], [1.675, -0.054, 0.09, 0.057, 2], [1.698, -0.018, 0.042, 0.024, 2]]), 2), -1, 0.005, 2)));
  out.push(loft(c("hair", "head", MAT.hair, TONE.hair), dense(hs("head", [[1.495, -0.086, 0.026, 0.078, 2.05], [1.58, -0.096, 0.044, 0.089, 2.0], [1.664, -0.082, 0.11, 0.081, 2.0], [1.708, -0.02, 0.042, 0.034, 2.0]]), 2)));
  out.push(visor(0.024, 0.0042, true));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth + 0.008], B(BAG.x, BAG.y, BAG.z - 0.004), upright, 0.012));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("torso", pick(strapPath(0.013), [0, 1, 3, 5, 7, 8, 9, 10, 11, 13, 15, 17, 18]), 0.0145, 0.003), [true, true], { slender: true }));
  out.push(...rods(END - 0.28, END - 0.006, "cane-band-tip"));
  return out;
}
export const thermal500 = (body: readonly Json[]): Json[] => thermalOf(500, body);

export function structure500(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const r = 0.0375;
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.084, 0.084, 0.064], Hd(0.024, 1.612), level), "head");
  m(capsule({ id: "cervical-spine", bone: "neck", ...BONE }, N(-0.008, 1.475), N(0.012, 1.54), r, r), "neck");
  const spine: [number, number][] = [[-0.028, 1.01], [-0.01, 1.12], [-0.036, 1.3], [-0.016, 1.44]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), [r, r, r, r]), "torso");
  m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.0, 1.39, -0.15), T(0.0, 1.39, 0.15), r, r), "torso");
  m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(-0.012, 0.935, -0.095), P(-0.012, 0.935, 0.095), 0.043, 0.043), "pelvis");
  const side: Json[] = [];
  const sm = (solid: Json, member: string): void => { side.push(part(solid, member, "member")); };
  sm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0.01, 0, 0.008], [0.4, 0.002, -0.004], r, r), "thigh-l");
  sm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.015, 0, -0.004], [0.385, 0, 0], r, r), "shank-l");
  sm(loft({ id: "foot-bones-l", bone: "foot-l", ...BONE }, xa("foot-l", [[-0.024, 0.008, 0.084, 0.0375], [0.07, 0.004, 0.08, 0.0375], [0.17, 0.002, 0.045, 0.0375]])), "foot-l");
  sm(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0.01, 0, 0], [0.3, 0, 0], r, r), "arm-l");
  sm(capsule({ id: "radius-ulna-l", bone: "forearm-l", ...BONE }, [0.01, 0, 0], [0.226, 0, 0], r, r), "forearm-l");
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.0375, 0.0375), st(0.065, 0.004, -0.002, 0.0375, 0.039), st(0.14, 0, -0.01, 0.0375, 0.0375)]), "hand-l");
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.05, 0.0375, 0.0375], [0.056, 0.002, 0.01]), "hand-r");
  for (const id of ["cane-grip", "cane-shaft", "cane-band", "cane-band-tip"]) if (body.some((b) => b.id === id)) out.push({ id: `member-${id}`, of: id, member: "cane", part: "member" });
  out.push(support(0.0375));
  for (const s of ["l", "r"]) out.push(jointBall(`thigh-${s}`, [0, 0, 0], 0.043), jointBall(`shank-${s}`, [0, 0, 0], 0.04), jointBall(`arm-${s}`, [0.002, 0, 0], 0.043), jointBall(`forearm-${s}`, [0, 0, 0], 0.04));
  for (const b of ["pelvis", "torso"]) out.push(massBall(b, 1.55));
  return out;
}

export const DECISIONS: Record<number, Json[]> = {
  100: [{ en: "1:100: draft.", de: "1:100: Entwurf." }],
  200: [{ en: "1:200: draft.", de: "1:200: Entwurf." }],
  500: [{ en: "1:500: draft.", de: "1:500: Entwurf." }],
};

/** 📚 The three coarser scales. */
export function coarseVolumes(): Json {
  const b100 = body100();
  const b200 = body200();
  const b500 = body500();
  return {
    "100": { status: "authored", denominator: 100, body: b100, thermal: thermal100(b100), structure: structure100(b100), decisions: DECISIONS[100] },
    "200": { status: "authored", denominator: 200, body: b200, thermal: thermal200(b200), structure: structure200(b200), decisions: DECISIONS[200] },
    "500": { status: "authored", denominator: 500, body: b500, thermal: thermal500(b500), structure: structure500(b500), decisions: DECISIONS[500] },
  };
}
export { OUTLET, aboutV, plume };
