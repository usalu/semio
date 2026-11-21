/** 🚶 The 1:100, 1:200 and 1:500 models of `humans/walking-commuter`, each stated anew for its scale: fewer and longer sections, merged masses, the same man in the same stance with his bag. Smallest feature 15, 30 and 75 mm.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `standing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-walking-commuter-s50.ts
 * @see ./r3-humans-walking-commuter.md */
import type { Json } from "./volumes-lib.ts";
import { FRAMES, H, L, X, aboutU, aboutV, aboutW, boneDeg, box, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-walking-commuter-kit.ts";
import { axial, rod } from "./r4-g06-skeleton.ts";
import { B, BAG, BONE, FACE_TOP, FIT, RIBS, acromion, coatParts, Hd, N, OUTER, P, R, STRAP, T, TONE, WRIST_W, breath, coatX, jointBall, legTubes, massBall, near, plume, ribbon, ring, type Row } from "./r3-walking-commuter-s50.ts";

const pair = (out: Json[], solids: Json[]): void => {
  for (const s of solids) out.push(s);
  for (const s of solids) out.push(mirrored(s, toRight, near));
};
const skin = (id: string, bone = "head", tone: number = TONE.skin): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "skin", tone });
const wool = (id: string, bone = "torso", tone: number = TONE.coat, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone, material: "wool-coat", tone, role });
const trouser = (id: string, bone: string, tone: number = TONE.trousersFar): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "wool-trouser", tone });
const shoe = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "leather-shoe", tone: TONE.shoeFar });
const leather = (id: string, tone: number = TONE.bag): { id: string; bone: string; material: string; tone: number } => ({ id, bone: "bag", material: "leather-bag", tone });
const hs = (bone: string, table: readonly Row[]): St[] => table.map(([y, b, f, rw, e]) => H(bone, y, b, f, rw, e));
const xs = (bone: string, table: readonly Row[]): St[] => table.map(([x, b, t, rw, e]) => X(bone, x, b, t, rw, e));
const level = aboutW(90 - boneDeg("head"));
const upright = aboutW(-90 - boneDeg("bag"));
const turn = compose(aboutW(-boneDeg("hand-r")), aboutV(8));
const pick = (path: readonly V3[], index: readonly number[]): V3[] => index.map((i) => path[i]!);

/* ---------------------------------------------------------------- 1:100 */

const COAT_100: readonly Row[] = [
  [0.556, -0.15, 0.15, 0.212, 2.5],
  [0.7, -0.149, 0.146, 0.208, 2.5],
  [0.88, -0.146, 0.138, 0.199, 2.45],
  [0.98, -0.14, 0.136, 0.192, 2.4],
  [1.12, -0.116, 0.133, 0.174, 2.4],
  [1.28, -0.118, 0.143, 0.172, 2.4],
  [1.38, -0.122, 0.142, 0.182, 2.35],
  [1.462, -0.112, 0.126, 0.218, 2.3],
  [1.5, -0.095, 0.107, 0.216, 2.2],
  [1.53, -0.072, 0.087, 0.172, 2.15],
  [1.556, -0.048, 0.068, 0.092, 2],
];
const HEAD_100: readonly Row[] = [[1.57, 0.056, 0.098, 0.02, 2], [1.59, 0.004, 0.107, 0.042, 2], [1.625, -0.048, 0.109, 0.057, 2], [1.675, -0.077, 0.11, 0.068, 2], [1.718, -0.082, 0.116, 0.074, 2], [1.755, -0.074, 0.106, 0.067, 2], [1.782, -0.05, 0.076, 0.046, 2], [1.795, -0.014, 0.036, 0.018, 2]];
const HAIR_100: readonly Row[] = [[1.624, -0.072, -0.03, 0.046, 2], [1.66, -0.087, -0.014, 0.068, 2], [1.7, -0.091, 0.03, 0.0765, 2], [1.735, -0.09, 0.088, 0.08, 2], [1.76, -0.082, 0.102, 0.074, 2], [1.785, -0.058, 0.087, 0.055, 2], [1.8025, -0.014, 0.038, 0.02, 2]];

export function body100(): Json[] {
  const out: Json[] = [];
  out.push(loft(trouser("pelvis-trousers", "pelvis", TONE.trousers), hs("pelvis", [[0.842, -0.084, 0.07, 0.128, 2.3], [0.92, -0.12, 0.104, 0.17, 2.4], [1.0, -0.116, 0.115, 0.167, 2.3], [1.085, -0.1, 0.116, 0.156, 2.3]])));
  out.push(...coatParts(COAT_100, 2));
  for (const side of [1, -1]) {
    const lapel: [number, number, number][] = [[1.368, 0.004, 0.024], [1.42, 0.027, 0.074], [1.452, 0.042, 0.104], [1.476, 0.052, 0.12], [1.514, 0.058, 0.094]];
    out.push(loft(wool(`lapel-${side > 0 ? "l" : "r"}`, "torso", 0.26, "detail"), lapel.map(([y, zin, zout]) => H("torso", y, coatX(y, zout, 1) - 0.012, coatX(y, zin, 1) + 0.008, (zout - zin) / 2, 5, (side * (zin + zout)) / 2))));
  }
  const vee: [number, number][] = [[1.375, 0.016], [1.44, 0.05], [1.5, 0.07], [1.54, 0.074]];
  out.push(loft({ id: "scarf-front", bone: "torso", material: "wool-scarf", tone: TONE.scarf, role: "detail" }, vee.map(([y, rw]) => H("torso", y, coatX(y, 0, 1) - 0.027, coatX(y, 0, 1) + 0.004, rw, 2))));

  const leg: Json[] = [];
  leg.push(loft(trouser("thigh-l", "thigh-l"), dome(dense([st(0.0, -0.005, 0.008, 0.09, 0.1, 2.1), st(0.16, 0.0, 0.006, 0.084, 0.093, 2), st(0.31, 0.005, 0, 0.07, 0.078, 2), st(0.435, 0.004, 0, 0.061, 0.068, 2)], 2), 1, 0.04, 2)));
  leg.push(loft(trouser("shank-l", "shank-l"), dome(dense([st(0.0, 0.004, 0, 0.061, 0.068, 2), st(0.14, -0.008, 0, 0.059, 0.068, 2), st(0.32, -0.004, 0, 0.054, 0.062, 2), st(0.428, 0.004, 0, 0.053, 0.0635, 2, -8)], 2), -1, 0.04, 2)));
  leg.push(loft(shoe("shoe-rear-l", "foot-l"), dense(xs("foot-l", [[-0.061, 0.014, 0.066, 0.018, 2], [-0.046, 0, 0.088, 0.033, 2.4], [0.02, 0, 0.097, 0.043, 2.4], [0.1, 0, 0.074, 0.05, 2.5], [0.174, 0, 0.053, 0.0525, 2.6]]), 2)));
  leg.push(loft(shoe("shoe-toe-l", "toe-l"), [X("toe-l", 0.15, 0, 0.058, 0.0525, 2.6), X("toe-l", 0.212, 0, 0.042, 0.042, 2.4, -0.005), X("toe-l", 0.2345, 0.002, 0.026, 0.015, 2, -0.009)]));
  pair(out, leg);

  const arm: Json[] = [];
  arm.push(loft(wool("sleeve-l", "arm-l", TONE.coatFar), dome(dome(dense([st(-0.022, 0.0, -0.03, 0.04, 0.056), st(0.0, 0.0, -0.01, 0.056, 0.067), st(0.08, 0.003, 0.006, 0.06, 0.068), st(0.24, 0.0, 0.011, 0.055, 0.061), st(0.33, -0.004, 0.012, 0.053, 0.058)], 2), 1, 0.044, 2), -1, 0.01, 2)));
  arm.push(loft(wool("forearm-l", "forearm-l", TONE.coatFar), dome([st(0.0, -0.004, 0.012, 0.053, 0.058), st(0.07, 0.0, 0.017, 0.052, 0.056), st(0.16, 0.0, 0.023, 0.0475, 0.049), st(0.246, 0, 0.03, 0.0455, 0.0445)], -1, 0.044, 2)));
  arm.push(loft({ id: "wrist-l", bone: "forearm-l", material: "skin", tone: TONE.handFar }, [st(0.228, 0, 0.029, 0.0225, 0.03), st(0.278, 0, WRIST_W + 0.002, 0.0205, 0.029)]));
  pair(out, arm);
  for (const s of out) if (s.id === "wrist-r") s.tone = TONE.hand;
  const w0 = WRIST_W + 0.002;
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.01, 0, w0, 0.0205, 0.029), st(0.043, 0.002, w0, 0.017, 0.043), st(0.098, 0, w0 - 0.004, 0.0135, 0.044), st(0.145, 0, w0 - 0.02, 0.0115, 0.04), st(0.175, 0, w0 - 0.04, 0.0095, 0.032)], 1, 0.008, 2)));
  out.push(capsule(skin("hand-l-thumb", "hand-l", TONE.handFar), [0.021, 0.032, w0 - 0.01], [0.098, 0.064, w0 - 0.024], 0.012, 0.0095));
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.0165, 0.041, 0.05], R(0.012, 0.022, 0.046), turn));
  out.push(ellipsoid(skin("hand-r-fingers", "hand-r", TONE.hand), [0.028, 0.042, 0.024], R(-0.004, 0.019, 0.112), turn));
  out.push(capsule(skin("hand-r-thumb", "hand-r", TONE.hand), R(0.006, 0.05, 0.022), R(-0.014, 0.067, 0.086), 0.0125, 0.0095));

  out.push(loft(skin("neck", "neck", 0.76), hs("neck", [[1.5, -0.06, 0.058, 0.064, 2], [1.58, -0.054, 0.062, 0.057, 2], [1.66, -0.044, 0.066, 0.055, 2]])));
  out.push(loft({ id: "scarf", bone: "neck", material: "wool-scarf", tone: TONE.scarf }, dense(hs("neck", [[1.528, -0.078, 0.084, 0.086, 2.2], [1.566, -0.088, 0.098, 0.093, 2.2], [1.602, -0.082, 0.095, 0.088, 2.2], [1.622, -0.066, 0.082, 0.074, 2.1]]), 2)));
  out.push(loft(wool("collar", "neck", 0.26), [st(-0.048, 0.004, 0, 0.112, 0.1, 2.1, -20), st(0.004, -0.012, 0, 0.103, 0.095, 2.1, -20)]));
  out.push(loft(skin("head"), dome(dense(hs("head", HEAD_100), 2), -1, 0.005, 2)));
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dome(dense(hs("head", HAIR_100), 2), -1, 0.008, 2)));
  out.push(loft(skin("head-nose"), [H("head", 1.656, 0.1, 0.132, 0.0165), H("head", 1.668, 0.1, 0.139, 0.015), H("head", 1.706, 0.1, 0.1185, 0.0085)]));
  for (const side of [1, -1]) out.push(ellipsoid(skin(`head-ear-${side > 0 ? "l" : "r"}`, "head", 0.74), [0.031, 0.0155, 0.0075], Hd(0.008, 1.684, side * 0.0745), compose(level, aboutW(-10), aboutU(side * 16))));

  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth], B(BAG.x, BAG.y, BAG.z), upright, 0.02));
  out.push(box(leather("bag-flap", 0.4), [0.165, BAG.length - 0.004, 0.016], B(BAG.x, BAG.y + BAG.height / 2 - 0.0805, BAG.z - BAG.depth / 2 - 0.004), upright, 0.006));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("bag", pick(STRAP, [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15]), 0.018, 0.003), [true, true], { slender: true }));
  return out;
}

/** 🌡️ The thermal list of a coarser scale: every garment by reference in its outer layer, skins as open surfaces of the visible part, the breath outlets and the plume outlet. */
function thermalOf(scale: number, body: readonly Json[], headRows: number, scarfFrom: number): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json | undefined => body.find((b) => b.id === id);
  for (const s of body) {
    if (s.material === "wool-coat") out.push(thOf(s.id, "coat", OUTER.coat));
    else if (s.material === "wool-trouser" && s.id !== "pelvis-trousers") out.push(...legTubes(s, false).map((t) => ({ ...t, layer: OUTER.trousers })));
    else if (s.material === "leather-shoe") out.push(thOf(s.id, "shoes", OUTER.shoes));
    else if (s.material === "hair") out.push(thOf(s.id, "hair", OUTER.hair));
    else if (s.material === "leather-bag") out.push(thOf(s.id, "bag-leather"));
    else if (s.material === "skin" && /^(hand|wrist)/.test(s.id)) out.push(thOf(s.id, "hand-skin"));
    else if (s.material === "skin" && /^head-/.test(s.id)) out.push(thOf(s.id, "face-skin"));
    else if (s.id === "scarf-front") out.push(thOf(s.id, "scarf", OUTER.scarf, { sectionFaces: ["+v"] }));
  }
  const scarf = by("scarf");
  if (scarf) out.push({ ...scarf, id: "th-scarf", sections: scarf.sections.slice(scarfFrom), capStart: false, capEnd: false, region: "scarf", layer: OUTER.scarf });
  const head = by("head")!;
  out.push({ ...head, id: "th-head", sections: head.sections.slice(0, headRows), capStart: true, capEnd: false, region: "face-skin" });
  if (scale === 100) out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin", tone: 0.76 }, [H("neck", 1.612, -0.05, 0.064, 0.056), H("neck", 1.66, -0.044, 0.066, 0.055)], [false, false]), region: "face-skin" });
  out.push(...breath(scale), plume(scale === 100 ? 0.008 : scale === 200 ? 0.015 : 0.0375));
  return out;
}
export const thermal100 = (body: readonly Json[]): Json[] => thermalOf(100, body, 12, 3);

const bagMass = (k: number): Json => part(box({ id: "mass-bag", bone: "bag", material: "leather-bag", tone: 0.2, role: "detail" }, [0.07 * k, 0.11 * k, 0.05 * k], B(BAG.x, BAG.y, BAG.z), upright, 0.01), "bag", "mass");

export function structure100(): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  for (const q of axial(FIT, 100)) m(acromion(q), q.bone);
  const side: Json[] = [];
  side.push(part(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.036, -0.006, 0.058], 0.0165, 0.0175), "thigh-l", "member"));
  side.push(part(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.04, -0.005, 0.062], [0.18, 0.01, 0.04], [0.32, 0.008, 0.017], [0.416, 0.0, 0.004]], [0.017, 0.014, 0.0142, 0.017]), "thigh-l", "member"));
  side.push(part(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.022, 0.031, 0.041], [0.428, -0.004, 0]), "thigh-l", "member"));
  side.push(part(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.008, 0.0, -0.002], [0.05, 0.006, -0.003], [0.2, 0.012, -0.003], [0.41, 0.002, -0.002]], [0.03, 0.017, 0.0125, 0.0145]), "shank-l", "member"));
  side.push(part(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.042, -0.018, 0.034], [0.426, -0.012, 0.03], 0.0075, 0.0075), "shank-l", "member"));
  side.push(part(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.05, 0.03, 0.074, 0.02), X("foot-l", 0.0, 0.04, 0.094, 0.026), X("foot-l", 0.09, 0.03, 0.064, 0.037), X("foot-l", 0.166, 0.014, 0.036, 0.042)]), "foot-l", "member"));
  side.push(part(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.156, 0.014, 0.036, 0.042), X("toe-l", 0.224, 0.012, 0.028, 0.027, 2, -0.006)]), "toe-l", "member"));
  side.push(part(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.0, 0, 0], [0.03, 0, 0.004], [0.2, 0.004, 0.01], [0.322, 0, 0.012]], [0.023, 0.013, 0.0108, 0.015]), "arm-l", "member"));
  side.push(part(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.012, -0.017, 0.012], [0.252, -0.014, WRIST_W], 0.0085, 0.0075), "forearm-l", "member"));
  side.push(part(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.01, 0.016, 0.013], [0.256, 0.012, WRIST_W], 0.0075, 0.009), "forearm-l", "member"));
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.066, 0.094, 0.071], Hd(0.014, 1.727), level), "head");
  m(ellipsoid({ id: "jaw", bone: "head", ...BONE }, [0.04, 0.046, 0.052], Hd(0.064, 1.628), compose(level, aboutW(-14))), "head");
  const cervical: [number, number][] = [[-0.022, 1.556], [-0.004, 1.605], [0.013, 1.652]];
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, cervical.map(([x, y]) => N(x, y)), [0.013, 0.012, 0.0115]), "neck");
  const w0 = WRIST_W + 0.002;
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, w0, 0.009, 0.016), st(0.05, 0.002, w0, 0.008, 0.032), st(0.1, 0, w0 - 0.005, 0.0075, 0.034), st(0.17, 0, w0 - 0.036, 0.0075, 0.024)]), "hand-l");
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.009, 0.032, 0.042], R(0.012, 0.022, 0.05), turn), "hand-r");
  m(ellipsoid({ id: "finger-bones-r", bone: "hand-r", ...BONE }, [0.018, 0.034, 0.014], R(-0.004, 0.019, 0.112), turn), "hand-r");
  out.push({ id: "member-bag-strap", of: "bag-strap", member: "bag", part: "member" });
  out.push(bagMass(1));
  out.push(jointBall("torso", T(-0.036, 1.064), 0.017), jointBall("neck", N(-0.022, 1.556), 0.0135), jointBall("head", Hd(0.013, 1.652), 0.0125));
  for (const s of ["l", "r"]) {
    out.push(jointBall(`thigh-${s}`, [0, 0, 0], 0.0245), jointBall(`shank-${s}`, [0, 0, 0], 0.019), jointBall(`foot-${s}`, [0, 0.002, 0], 0.016), jointBall(`toe-${s}`, [-0.004, 0.024, 0], 0.01));
    out.push(jointBall(`arm-${s}`, [0.002, 0, 0], 0.0245), jointBall(`forearm-${s}`, [0, 0, s === "l" ? 0.012 : -0.012], 0.0155), jointBall(`hand-${s}`, [0, 0, s === "l" ? WRIST_W : -WRIST_W], 0.012));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) out.push(massBall(b));
  return out;
}

/* ---------------------------------------------------------------- 1:200 */

const COAT_200: readonly Row[] = [
  [0.556, -0.15, 0.15, 0.212, 2.5],
  [0.86, -0.146, 0.139, 0.2, 2.45],
  [1.0, -0.136, 0.135, 0.19, 2.4],
  [1.14, -0.115, 0.134, 0.173, 2.4],
  [1.34, -0.121, 0.143, 0.177, 2.35],
  [1.462, -0.112, 0.126, 0.218, 2.3],
  [1.51, -0.088, 0.1, 0.208, 2.2],
  [1.556, -0.048, 0.068, 0.092, 2],
];

export function body200(): Json[] {
  const out: Json[] = [];
  out.push(loft(trouser("pelvis-trousers", "pelvis", TONE.trousers), hs("pelvis", [[0.842, -0.084, 0.07, 0.128, 2.3], [0.93, -0.121, 0.106, 0.17, 2.4], [1.085, -0.1, 0.116, 0.156, 2.3]])));
  out.push(...coatParts(COAT_200, 1));
  const leg: Json[] = [];
  leg.push(loft(trouser("thigh-l", "thigh-l"), dome([st(0.0, -0.005, 0.008, 0.09, 0.1, 2.1), st(0.2, 0.002, 0.004, 0.08, 0.089, 2), st(0.435, 0.004, 0, 0.061, 0.068, 2)], 1, 0.04, 2)));
  leg.push(loft(trouser("shank-l", "shank-l"), dome([st(0.0, 0.004, 0, 0.061, 0.068, 2), st(0.16, -0.008, 0, 0.058, 0.067, 2), st(0.428, 0.004, 0, 0.053, 0.063, 2)], -1, 0.04, 2)));
  leg.push(loft(shoe("shoe-rear-l", "foot-l"), xs("foot-l", [[-0.061, 0.01, 0.07, 0.02, 2], [-0.04, 0, 0.09, 0.035, 2.4], [0.03, 0, 0.096, 0.044, 2.4], [0.11, 0, 0.07, 0.051, 2.5], [0.174, 0, 0.053, 0.0525, 2.6]])));
  leg.push(loft(shoe("shoe-toe-l", "toe-l"), [X("toe-l", 0.15, 0, 0.058, 0.0525, 2.6), X("toe-l", 0.215, 0, 0.042, 0.04, 2.4, -0.005), X("toe-l", 0.2345, 0, 0.032, 0.02, 2, -0.008)]));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft(wool("sleeve-l", "arm-l", TONE.coatFar), dome(dome([st(-0.02, 0.0, -0.028, 0.042, 0.058), st(0.03, 0.002, 0.0, 0.06, 0.068), st(0.18, 0.002, 0.01, 0.057, 0.063), st(0.33, -0.004, 0.012, 0.053, 0.058)], 1, 0.044, 2), -1, 0.01, 2)));
  arm.push(loft(wool("forearm-l", "forearm-l", TONE.coatFar), dome([st(0.0, -0.004, 0.012, 0.053, 0.058), st(0.12, 0.0, 0.02, 0.05, 0.052), st(0.25, 0, 0.03, 0.0455, 0.0445)], -1, 0.044, 2)));
  pair(out, arm);
  const w0 = WRIST_W + 0.002;
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.014, 0, w0, 0.021, 0.03), st(0.05, 0.004, w0, 0.019, 0.046), st(0.11, 0.004, w0 - 0.006, 0.017, 0.045), st(0.172, 0, w0 - 0.034, 0.015, 0.034)], 1, 0.008, 2)));
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.03, 0.045, 0.062], R(0.004, 0.022, 0.064), turn));
  out.push(loft(skin("neck", "neck", 0.76), hs("neck", [[1.5, -0.06, 0.058, 0.064, 2], [1.66, -0.044, 0.066, 0.055, 2]])));
  out.push(loft({ id: "scarf", bone: "neck", material: "wool-scarf", tone: TONE.scarf }, hs("neck", [[1.528, -0.08, 0.086, 0.09, 2.2], [1.575, -0.088, 0.098, 0.094, 2.2], [1.622, -0.068, 0.084, 0.077, 2.1]])));
  out.push(loft(skin("head"), hs("head", [[1.57, 0.05, 0.1, 0.024, 2], [1.6, -0.02, 0.11, 0.05, 2], [1.66, -0.074, 0.114, 0.067, 2], [1.72, -0.082, 0.118, 0.074, 2], [1.765, -0.068, 0.098, 0.062, 2], [1.795, -0.02, 0.044, 0.024, 2]])));
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, hs("head", [[1.626, -0.074, -0.026, 0.05, 2], [1.68, -0.09, 0.0, 0.074, 2], [1.74, -0.09, 0.094, 0.08, 2], [1.78, -0.066, 0.094, 0.062, 2], [1.8025, -0.02, 0.046, 0.026, 2]])));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth + 0.008], B(BAG.x, BAG.y, BAG.z - 0.004), upright, 0.02));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("bag", pick(STRAP, [0, 2, 4, 6, 8, 10, 12, 15]), 0.018, 0.003), [true, true], { slender: true }));
  return out;
}
export const thermal200 = (body: readonly Json[]): Json[] => thermalOf(200, body, 4, 1);

export function structure200(): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.092, 0.092, 0.07], Hd(0.022, 1.705), compose(level, aboutW(-14))), "head");
  m(capsule({ id: "cervical-spine", bone: "neck", ...BONE }, N(-0.022, 1.556), N(0.013, 1.652), 0.015, 0.015), "neck");
  for (const q of axial(FIT, 200)) m(q, q.bone);
  const side: Json[] = [];
  side.push(part(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0, 0, 0], [0.04, -0.004, 0.058], [0.2, 0.01, 0.036], [0.425, 0, 0.002]], [0.02, 0.017, 0.015, 0.02]), "thigh-l", "member"));
  side.push(part(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.01, 0.002, 0], [0.415, 0.002, 0], 0.02, 0.015), "shank-l", "member"));
  side.push(part(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.046, 0.03, 0.08, 0.022), X("foot-l", 0.02, 0.036, 0.092, 0.03), X("foot-l", 0.166, 0.01, 0.042, 0.042)]), "foot-l", "member"));
  side.push(part(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.156, 0.008, 0.04, 0.042), X("toe-l", 0.222, 0.006, 0.036, 0.028, 2, -0.006)]), "toe-l", "member"));
  side.push(part(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0, 0, 0], [0.324, 0, 0.012], 0.019, 0.015), "arm-l", "member"));
  side.push(part(capsule({ id: "radius-ulna-l", bone: "forearm-l", ...BONE }, [0, 0, 0.012], [0.254, 0, WRIST_W], 0.016, 0.015), "forearm-l", "member"));
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  const w0 = WRIST_W + 0.002;
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, w0, 0.015, 0.017), st(0.06, 0.002, w0, 0.015, 0.032), st(0.165, 0, w0 - 0.03, 0.015, 0.024)]), "hand-l");
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.016, 0.032, 0.048], R(0.004, 0.022, 0.064), turn), "hand-r");
  out.push({ id: "member-bag-strap", of: "bag-strap", member: "bag", part: "member" });
  out.push(bagMass(1.15));
  out.push(jointBall("neck", N(-0.022, 1.556), 0.018));
  for (const s of ["l", "r"]) {
    out.push(jointBall(`thigh-${s}`, [0, 0, 0], 0.0245), jointBall(`shank-${s}`, [0, 0, 0], 0.022), jointBall(`foot-${s}`, [0, 0.002, 0], 0.018));
    out.push(jointBall(`arm-${s}`, [0.002, 0, 0], 0.0245), jointBall(`forearm-${s}`, [0, 0, s === "l" ? 0.012 : -0.012], 0.019), jointBall(`hand-${s}`, [0, 0, s === "l" ? WRIST_W : -WRIST_W], 0.017));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r"]) out.push(massBall(b, 1.1));
  return out;
}

/* ---------------------------------------------------------------- 1:500 */

export function body500(): Json[] {
  const out: Json[] = [];
  out.push(loft(trouser("pelvis-trousers", "pelvis", TONE.trousers), hs("pelvis", [[0.85, -0.09, 0.076, 0.136, 2.3], [0.95, -0.12, 0.106, 0.168, 2.4], [1.08, -0.1, 0.116, 0.156, 2.3]])));
  out.push(...coatParts([[0.556, -0.15, 0.15, 0.212, 2.5], [0.95, -0.141, 0.136, 0.194, 2.4], [1.14, -0.116, 0.134, 0.174, 2.4], [1.36, -0.122, 0.143, 0.18, 2.35], [1.47, -0.11, 0.124, 0.22, 2.3], [1.52, -0.08, 0.094, 0.19, 2.2], [1.556, -0.05, 0.07, 0.1, 2]], 1, 0.0375));
  const leg: Json[] = [];
  leg.push(loft(trouser("thigh-l", "thigh-l"), dome([st(0.0, -0.004, 0.008, 0.09, 0.1, 2.1), st(0.435, 0.004, 0, 0.062, 0.069, 2)], 1, 0.04, 2)));
  leg.push(loft(trouser("shank-l", "shank-l"), dome([st(0.0, 0.004, 0, 0.062, 0.069, 2), st(0.428, 0.002, 0, 0.054, 0.064, 2)], -1, 0.04, 2)));
  leg.push(loft(shoe("shoe-l", "foot-l"), xs("foot-l", [[-0.061, 0, 0.076, 0.026, 2], [0.0, 0, 0.097, 0.042, 2.4], [0.12, 0, 0.07, 0.052, 2.5], [0.2, 0, 0.05, 0.048, 2.5], [0.2345, 0, 0.038, 0.03, 2]])));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft(wool("sleeve-l", "arm-l", TONE.coatFar), dome(dome([st(-0.016, 0.0, -0.022, 0.046, 0.06), st(0.06, 0.002, 0.004, 0.06, 0.068), st(0.33, -0.004, 0.012, 0.054, 0.059)], 1, 0.044, 2), -1, 0.01, 2)));
  arm.push(loft(wool("forearm-l", "forearm-l", TONE.coatFar), dome([st(0.0, -0.004, 0.012, 0.054, 0.059), st(0.25, 0, 0.03, 0.046, 0.046)], -1, 0.044, 2)));
  pair(out, arm);
  const w0 = WRIST_W + 0.002;
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.014, 0, w0, 0.039, 0.04), st(0.07, 0.004, w0 - 0.002, 0.039, 0.046), st(0.165, 0, w0 - 0.012, 0.039, 0.04)], 1, 0.01, 2)));
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.041, 0.05, 0.064], R(0.002, 0.022, 0.064), turn));
  out.push(loft({ id: "scarf", bone: "neck", material: "wool-scarf", tone: TONE.scarf }, hs("neck", [[1.528, -0.08, 0.088, 0.092, 2.2], [1.575, -0.088, 0.098, 0.094, 2.2], [1.622, -0.07, 0.086, 0.08, 2.1]])));
  out.push(loft(skin("head"), dense(hs("head", [[1.57, 0.04, 0.1, 0.03, 2], [1.63, -0.056, 0.112, 0.062, 2], [1.72, -0.082, 0.118, 0.074, 2], [1.78, -0.054, 0.084, 0.052, 2], [1.797, -0.02, 0.046, 0.026, 2]]), 2)));
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dense(hs("head", [[1.63, -0.078, -0.02, 0.054, 2], [1.73, -0.09, 0.088, 0.08, 2], [1.782, -0.064, 0.094, 0.06, 2], [1.8025, -0.022, 0.05, 0.028, 2]]), 2)));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth + 0.008], B(BAG.x, BAG.y, BAG.z - 0.004), upright, 0.012));
  out.push(loft(leather("bag-strap", TONE.strap), ribbon("bag", pick(STRAP, [0, 3, 6, 8, 10, 12, 15]), 0.018, 0.003), [true, true], { slender: true }));
  return out;
}
export const thermal500 = (body: readonly Json[]): Json[] => thermalOf(500, body, 5, 1);

/** 🎗️ 1:500: one rib hoop of the commuter's rib table `[height at the spine, centre ahead, half depth, half breadth]` as a slender closed ring of a real rib's thickness, sinking 30 mm from the spine to the sternum. */
function hoop500(id: string, [y, cx, hd, hw]: readonly [number, number, number, number]): Json {
  const path: V3[] = Array.from({ length: 17 }, (_, k) => {
    const a = Math.PI + (2 * Math.PI * k) / 16;
    return T(cx + hd * Math.cos(a), y - 0.03 * (1 + Math.cos(a)) / 2, hw * Math.sin(a));
  });
  return tube({ id, bone: "torso", ...BONE }, path, [0.006], { slender: true });
}
/** 🪽 1:500: the shoulder girdle of the left side as one line, sternal end of the collar bone, acromion, spine of the shoulder blade, inferior angle (figure coordinates). */
const GIRDLE_500: readonly V3[] = [[0.07, 1.48, 0.025], [0.062, 1.49, 0.1], [0.0, 1.51, 0.19], [-0.07, 1.49, 0.13], [-0.1, 1.33, 0.1]];
/** 🦴 1:500: the left hip bone as one line, sacroiliac joint, iliac crest, anterior superior spine, acetabulum, pubic symphysis (figure coordinates). */
const HIP_BONE_500: readonly V3[] = [[-0.07, 1.03, 0.045], [-0.03, 1.09, 0.135], [0.06, 1.04, 0.12], [0.0, 0.965, 0.09], [0.055, 0.92, 0.012]];

export function structure500(): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.09, 0.09, 0.068], Hd(0.02, 1.706), level), "head");
  m(rod(FIT, "cervical-spine", "neck", N(-0.026, 1.54), N(0.014, 1.66), 0.012), "neck");
  for (const q of axial(FIT, 500).filter((q) => q.id === "sk-spine")) m(q, q.bone);
  for (const [k, i] of [[1, 7], [2, 4], [3, 1]] as const) m(hoop500(`rib-hoop-${k}`, RIBS[i]!), "torso");
  m(rod(FIT, "sternum", "torso", T(0.07, 1.476), T(0.112, 1.262), 0.009), "torso");
  for (const g of [1, -1]) {
    const n = g > 0 ? "l" : "r";
    m(tube({ id: `shoulder-girdle-${n}`, bone: "torso", ...BONE }, GIRDLE_500.map(([x, y, z]) => T(x, y, g * z)), [0.0065], { slender: true }), "torso");
    m(tube({ id: `hip-bone-${n}`, bone: "pelvis", ...BONE }, HIP_BONE_500.map(([x, y, z]) => P(x, y, g * z)), [0.008], { slender: true }), "pelvis");
  }
  const side: Json[] = [];
  const fz = FRAMES["foot-l"]!.o[2];
  side.push(part(rod(FIT, "femur-l", "thigh-l", [0, 0, 0.04], [0.435, 0, 0], 0.014), "thigh-l", "member"));
  side.push(part(rod(FIT, "tibia-l", "shank-l", [0, 0.002, 0], [0.425, 0, 0], 0.0125), "shank-l", "member"));
  side.push(part(rod(FIT, "foot-bones-l", "foot-l", L("foot-l", [-0.045, 0.045, fz]), L("foot-l", [0.168, 0.026, fz]), 0.009), "foot-l", "member"));
  side.push(part(rod(FIT, "phalanges-l", "toe-l", L("toe-l", [0.168, 0.026, fz]), L("toe-l", [0.225, 0.02, fz]), 0.004), "toe-l", "member"));
  side.push(part(rod(FIT, "humerus-l", "arm-l", [0, 0, 0.002], [0.33, 0, 0.012], 0.0105), "arm-l", "member"));
  side.push(part(rod(FIT, "radius-ulna-l", "forearm-l", [0, 0, 0.012], [0.26, 0, WRIST_W], 0.008), "forearm-l", "member"));
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight));
  m(rod(FIT, "hand-bones-l", "hand-l", [0, 0, WRIST_W + 0.002], [0.17, 0.002, WRIST_W - 0.02], 0.006), "hand-l");
  m(rod(FIT, "hand-bones-r", "hand-r", R(0, 0, 0), R(0.004, 0.05, 0.12), 0.006), "hand-r");
  out.push({ id: "member-bag-strap", of: "bag-strap", member: "bag", part: "member" });
  for (const b of ["pelvis", "torso"]) out.push(massBall(b, 1.5));
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
    "100": { status: "authored", denominator: 100, body: b100, thermal: thermal100(b100), structure: structure100(), decisions: DECISIONS[100] },
    "200": { status: "authored", denominator: 200, body: b200, thermal: thermal200(b200), structure: structure200(), decisions: DECISIONS[200] },
    "500": { status: "authored", denominator: 500, body: b500, thermal: thermal500(b500), structure: structure500(), decisions: DECISIONS[500] },
  };
}
export { FACE_TOP, FRAMES, L, box };
