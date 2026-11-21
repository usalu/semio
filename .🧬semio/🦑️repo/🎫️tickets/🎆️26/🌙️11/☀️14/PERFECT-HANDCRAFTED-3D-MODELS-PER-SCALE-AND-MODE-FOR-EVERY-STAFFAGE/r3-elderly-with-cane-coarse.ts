/** 🧓 The 1:100, 1:200 and 1:500 models of `humans/elderly-with-cane`, each stated anew for its scale: fewer and longer sections, merged masses, the same man in the same stooped stance. Smallest feature 15, 30 and 75 mm.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `stooped-standing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-elderly-with-cane-s50.ts
 * @see ./r3-humans-elderly-with-cane.md */
import type { Json } from "./volumes-lib.ts";
import { H, L, X, aboutU, aboutV, aboutW, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-elderly-with-cane-kit.ts";
import { BONE, CANE_L, CROOK_R, JACKET, OUTER, SEAT, TONE, crookPath, fingerBand, hr, jacketX, kgOf, palmFrame } from "./r3-elderly-with-cane-s50.ts";

type Row = readonly [number, number, number, number, number];
const rows = (table: readonly Row[], pick: readonly number[]): Row[] => pick.map((i) => table[i]!);
const near = (t: number): number => (t === TONE.jacketFar ? TONE.jacket : t === TONE.trousersFar ? TONE.trousers : t === TONE.shoeFar ? TONE.shoe : t === TONE.handFar ? TONE.hand : t);
const pair = (out: Json[], solids: Json[]): void => {
  for (const s of solids) out.push(s);
  for (const s of solids) out.push(mirrored(s, toRight, near));
};
const skin = (id: string, bone = "head", tone: number = TONE.skin): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "skin", tone });
const wood = { bone: "cane", material: "ash-wood", tone: TONE.wood };
const cap = { bone: "head", material: "cap-tweed", tone: TONE.cap };
const hair = { bone: "head", material: "hair", tone: TONE.hair };
const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
const N = (x: number, y: number): V3 => L("neck", [x, y, 0]);
const ring = (rd: number, r: number): [number, number][] => Array.from({ length: 9 }, (_, k) => [r * Math.cos((k * Math.PI) / 4), rd + r * Math.sin((k * Math.PI) / 4)] as [number, number]);
/** 🩼 The whole cane as one centre line in the cane frame: nose of the crook, apex, shaft, tip. */
const canePath = (n: number): V3[] => [...crookPath(n).reverse(), [0.25, 0, 0], [0.5, 0, 0], [CANE_L, 0, 0]];

/* ---------------------------------------------------------------- 1:100 */

export function body100(): Json[] {
  const out: Json[] = [];
  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, SEAT.map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e))));
  out.push(loft({ id: "jacket", bone: "torso", material: "tweed", tone: TONE.jacket }, dense(rows(JACKET, [0, 2, 3, 5, 7, 8, 9, 10, 11]).map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e)), 2)));
  const front: [number, number][] = [[1.154, 0.012], [1.234, 0.03], [1.314, 0.042], [1.374, 0.04], [1.404, 0.034]];
  out.push(loft({ id: "shirt-front", bone: "torso", material: "cotton-flannel", tone: TONE.shirt, role: "detail" }, front.map(([y, rw]) => H("torso", y, jacketX(y, 0, 1) - 0.013, jacketX(y, 0, 1) + 0.003, rw, 2))));

  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense([st(0.0, -0.01, 0, 0.085, 0.1, 2.1), st(0.16, -0.002, 0, 0.08, 0.089, 2), st(0.3, 0.001, 0, 0.07, 0.077, 1.9), st(0.4046, 0.004, 0, 0.066, 0.072, 1.9)], 2), 1, 0.04, 2)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense([st(0.0, 0.004, 0, 0.0645, 0.0705, 1.9), st(0.14, -0.006, 0, 0.064, 0.071, 1.9), st(0.3, -0.002, 0, 0.0625, 0.07, 1.9), st(0.39, 0.006, 0, 0.063, 0.074, 2, -10)], 2), -1, 0.04, 2)));
  const rear: Row[] = [[-0.066, 0.012, 0.062, 0.017, 2], [-0.05, 0, 0.083, 0.031, 2.4], [0.02, 0, 0.094, 0.043, 2.4], [0.1, 0, 0.066, 0.05, 2.5], [0.166, 0, 0.048, 0.051, 2.6]];
  const toe: Row[] = [[0.146, 0, 0.053, 0.0515, 2.6], [0.2, 0, 0.038, 0.04, 2.4], [0.2195, 0.002, 0.024, 0.014, 2]];
  leg.push(loft({ id: "shoe-rear-l", bone: "foot-l", material: "leather-shoe", tone: TONE.shoeFar }, dense(rear.map(([x, b, t, rw, e]) => X("foot-l", x, b, t, rw, e)), 2)));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", material: "leather-shoe", tone: TONE.shoeFar }, toe.map(([x, b, t, rw, e]) => X("toe-l", x, b, t, rw, e))));
  pair(out, leg);

  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "arm-l", material: "tweed", tone: TONE.jacketFar }, dome(dome(dense([st(-0.022, 0.0, -0.036, 0.034, 0.05), st(0.0, 0.0, -0.022, 0.05, 0.061), st(0.1, 0.004, -0.005, 0.052, 0.061), st(0.27, -0.002, 0, 0.0455, 0.0515), st(0.3069, -0.004, 0, 0.0455, 0.051)], 2), 1, 0.042, 2), -1, 0.016, 2)));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", material: "tweed", tone: TONE.jacketFar }, dome([st(0.0, -0.004, 0, 0.044, 0.049), st(0.05, 0.0, 0, 0.044, 0.048), st(0.13, 0.0, 0, 0.0405, 0.043), st(0.213, 0, 0, 0.0375, 0.039)], -1, 0.042, 2)));
  arm.push(loft({ id: "wrist-l", bone: "forearm-l", material: "skin", tone: TONE.handFar }, [st(0.2, 0, 0, 0.0235, 0.0295), st(0.252, 0, 0, 0.0198, 0.0278)]));
  pair(out, arm);
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), dome([st(-0.01, 0, 0, 0.0195, 0.0275), st(0.04, 0.002, 0, 0.016, 0.04), st(0.092, 0, -0.004, 0.0125, 0.041), st(0.135, 0, -0.019, 0.0105, 0.037), st(0.163, 0, -0.038, 0.009, 0.03)], 1, 0.008, 2)));
  out.push(capsule(skin("hand-l-thumb", "hand-l", TONE.handFar), [0.02, 0.03, -0.01], [0.092, 0.06, -0.023], 0.0115, 0.009));
  const palm = palmFrame();
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.047, 0.044, 0.0145], palm.at, palm.rotation));
  out.push({ id: "hand-r-fingers", kind: "loft", bone: "hand-r", material: "skin", role: "body", tone: TONE.hand, sections: fingerBand(0.04, 0.0092), capStart: true, capEnd: true });
  out.push(capsule(skin("hand-r-thumb", "hand-r", TONE.hand), hr([0.006, -0.012, 0.004]), hr([0.058, -0.052, -0.018]), 0.0125, 0.0092));

  out.push(loft(skin("neck", "neck", 0.78), [st(-0.035, 0.004, 0, 0.066, 0.068), st(0.02, 0.0, 0, 0.059, 0.061), st(0.06, -0.005, 0, 0.056, 0.057), st(0.112, -0.01, 0, 0.055, 0.054)]));
  out.push(loft({ id: "collar", bone: "neck", material: "tweed", tone: 0.45 }, [st(-0.03, 0.02, 0, 0.088, 0.08, 2.1, 24), st(-0.004, 0.022, 0, 0.081, 0.073, 2.1, 24), st(0.018, 0.024, 0, 0.073, 0.066, 2.1, 24)]));
  out.push(loft(skin("head"), dome(dense([st(-0.045, -0.057, 0, 0.02, 0.018), st(-0.022, -0.038, 0, 0.047, 0.048), st(0.01, -0.013, 0, 0.062, 0.076), st(0.05, 0.001, 0, 0.0705, 0.0905), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)], 2), -1, 0.006, 2)));
  out.push(ellipsoid(skin("head-nose"), [0.026, 0.0125, 0.0105], [0.05, -0.1035, 0], aboutW(32)));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.76), [0.032, 0.017, 0.0078], [0.046, 0.016, side * 0.078], compose(aboutW(-8), aboutU(side * 24))));
    out.push(ellipsoid({ id: `hair-temple-${n}`, ...hair }, [0.024, 0.036, 0.012], [0.074, 0.034, side * 0.068]));
  }
  out.push(ellipsoid({ id: "hair-white", ...hair }, [0.04, 0.02, 0.066], [0.05, 0.078, 0], aboutW(-6)));
  const crown: [number, number, number, number][] = [[0.117, -0.123, 0.044, 0.0075], [0.126, -0.075, 0.087, 0.025], [0.1335, 0.0, 0.099, 0.047], [0.12, 0.08, 0.083, 0.047], [0.1, 0.118, 0.03, 0.014]];
  out.push(loft({ id: "cap", ...cap }, dense(crown.map(([u, v, rw, rh]) => st(u, v, 0, rw, rh, 2.3, 90)), 2)));
  out.push(loft({ id: "cap-brim", ...cap }, [st(0.109, -0.08, 0, 0.076, 0.0075, 2, 90), st(0.104, -0.112, 0, 0.069, 0.0075, 2, 90), st(0.098, -0.147, 0, 0.032, 0.0075, 2, 90)]));

  out.push(tube({ id: "cane-shaft", ...wood }, [[-0.004, 0, 0], [0.35, 0, 0], [0.7, 0, 0]], [0.011, 0.0103, 0.009]));
  out.push(tube({ id: "cane-crook", ...wood, tone: 0.4 }, crookPath(8), [0.011]));
  out.push(revolve({ id: "cane-ferrule", bone: "cane", material: "rubber", tone: 0.05 }, [[CANE_L - 0.042, 0], [CANE_L - 0.042, 0.0122], [CANE_L - 0.006, 0.014], [CANE_L, 0.0132], [CANE_L, 0]], [0, 0, 0], [1, 0, 0]));
  return out;
}

/** 🙂 The visible skin of head and neck as open skins: [head stations, neck stations]. */
const FACE: Record<number, [St[], St[]]> = {
  100: [[st(-0.045, -0.057, 0, 0.02, 0.018), st(-0.022, -0.038, 0, 0.047, 0.048), st(0.01, -0.013, 0, 0.062, 0.076), st(0.05, 0.001, 0, 0.0705, 0.0905), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)], [st(0.03, -0.001, 0, 0.0582, 0.06), st(0.088, -0.008, 0, 0.0555, 0.0555)]],
  200: [[st(-0.045, -0.057, 0, 0.022, 0.02), st(-0.012, -0.03, 0, 0.053, 0.058), st(0.035, -0.004, 0, 0.068, 0.087), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)], [st(0.03, -0.001, 0, 0.0582, 0.06), st(0.088, -0.008, 0, 0.0555, 0.0555)]],
  500: [[st(-0.04, -0.05, 0, 0.038, 0.038), st(0.02, -0.01, 0, 0.066, 0.082), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)], [st(0.02, 0.0, 0, 0.066, 0.064), st(0.098, -0.009, 0, 0.0555, 0.055)]],
};
/** 🌡️ The thermal list of a coarser scale: every garment by reference in its outer layer, the visible skin of head and neck as open skins, outlet patches on the surface. */
function thermalOf(scale: 100 | 200 | 500, body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const region = (s: Json): [string, string?] | null => {
    if (/^(head|neck|pelvis-trousers)$/.test(s.id)) return null;
    if (s.material === "tweed" || s.material === "cotton-flannel") return ["jacket", OUTER.jacket];
    if (s.material === "wool-trouser") return ["trousers", OUTER.trousers];
    if (s.material === "leather-shoe") return ["shoes", OUTER.shoes];
    if (s.material === "cap-tweed") return ["cap", OUTER.cap];
    if (s.material === "hair") return ["hair", "hair-pelt"];
    if (s.bone === "cane") return ["cane-wood"];
    if (s.material === "skin") return [/^(hand|wrist)/.test(s.id) ? "hand-skin" : "face-skin"];
    throw new Error(`no region for ${s.id}`);
  };
  for (const s of body) {
    const r = region(s);
    if (r) out.push(thOf(s.id, r[0], r[1], scale === 100 && s.id === "cap" ? { sectionFaces: ["-u"] } : {}));
  }
  const [head, neck] = FACE[scale];
  out.push({ ...loft({ id: "th-head", ...skin("th-head") }, scale === 500 ? head : dome(head, -1, 0.006, 2), [true, false]), region: "face-skin" });
  out.push({ ...loft({ id: "th-neck", ...skin("th-neck", "neck", 0.78) }, neck, [false, false]), region: "face-skin" });
  const f = (scale * 0.15) / 2000;
  out.push({ ...ellipsoid({ id: "th-breath-nose", ...skin("th-breath-nose") }, [f, f, Math.max(f, 0.0095)], [0.0195 + f / 2, scale === 100 ? -0.1 : -0.088 + f / 2, 0]), region: "face-skin", tags: ["emitter", "breath-nose"] });
  out.push({ ...ellipsoid({ id: "th-breath-mouth", ...skin("th-breath-mouth") }, [f, f, f], [-0.004, scale === 100 ? -0.083 : -0.075 + f / 2, 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] });
  out.push({ ...ellipsoid({ id: "th-plume", bone: "head", material: "cap-tweed", tone: 0.5 }, [f, 0.07, 0.07], [0.181 - f, 0.025, 0], aboutW(-5)), region: "jacket", tags: ["emitter", "plume"] });
  return out;
}
export const thermal100 = (body: readonly Json[]): Json[] => thermalOf(100, body);
export const thermal200 = (body: readonly Json[]): Json[] => thermalOf(200, body);
export const thermal500 = (body: readonly Json[]): Json[] => thermalOf(500, body);

const joint = (out: Json[], member: string, at: V3, r: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint")); };
const mass = (out: Json[], member: string, k = 0.0125): void => {
  const m = kgOf(member);
  const r = k * Math.cbrt(m.kg);
  out.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [m.comX, m.comY, 0]), member, "mass"));
};
const caneMembers = (out: Json[], body: readonly Json[]): void => { for (const s of body) if (s.bone === "cane") out.push({ id: `member-${s.id}`, of: s.id, member: "cane", part: "member" }); };

export function structure100(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const spine: [number, number, number][] = [[-0.03, 0.989, 0.0195], [-0.03, 1.104, 0.018], [-0.026, 1.249, 0.0155], [0.0, 1.314, 0.0145], [0.04, 1.369, 0.0135], [0.08, 1.406, 0.0125]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const hoops: [number, number, number, number][] = [[1.124, 0.045, 0.093, 0.125], [1.184, 0.048, 0.103, 0.138], [1.244, 0.052, 0.104, 0.139], [1.304, 0.056, 0.092, 0.127], [1.354, 0.063, 0.07, 0.097]];
  hoops.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${5 - i}`, bone: "torso", ...BONE }, ring(rd, 0.0076), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.209, 0.141, 0.157, 0.016), H("torso", 1.336, 0.139, 0.155, 0.024)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.146, 1.346, side * 0.022), T(0.098, 1.372, side * 0.168), 0.008, 0.008), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.075, 0.0078, 0.048], T(-0.03, 1.289, side * 0.105), compose(aboutW(8), aboutU(side * 28))), "torso");
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.062, 0.07, 0.012], P(-0.022, 0.954, side * 0.092), compose(aboutU(side * 32), aboutV(side * -22))), "pelvis");
    m(capsule({ id: `ilium-body-${n}`, bone: "pelvis", ...BONE }, P(-0.05, 0.954, side * 0.034), P(0.0, 0.892, side * 0.084), 0.017, 0.019), "pelvis");
    m(capsule({ id: `ischiopubic-${n}`, bone: "pelvis", ...BONE }, P(0.04, 0.87, side * 0.02), P(-0.036, 0.826, side * 0.058), 0.0125, 0.014), "pelvis");
  }
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.03, 0.989), P(-0.058, 0.924), P(-0.062, 0.876)], [0.028, 0.022, 0.0085]), "pelvis");
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.07, 0.09, 0.0685], [0.086, 0.011, 0]), "head");
  m(ellipsoid({ id: "jaw", bone: "head", ...BONE }, [0.046, 0.04, 0.05], [0.012, -0.04, 0]), "head");
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, [N(0.08, 1.406), N(0.12, 1.44), N(0.158, 1.471)], [0.0125, 0.012, 0.011]), "neck");
  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.06], 0.0175, 0.02), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.062], [0.15, 0.009, 0.042], [0.29, 0.009, 0.018], [0.385, 0.0, 0.004]], [0.0155, 0.0138, 0.014, 0.017]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.021, 0.03, 0.041], [0.398, -0.006, 0]), "thigh-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.012, 0.0, 0.002], [0.05, 0.006, 0.003], [0.2, 0.009, 0.003], [0.385, 0.002, 0.0]], [0.027, 0.0165, 0.012, 0.0145]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.018, 0.034], [0.396, -0.012, 0.03], 0.0076, 0.0078), "shank-l");
  lm(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.052, 0.018, 0.066, 0.021), X("foot-l", 0.0, 0.03, 0.074, 0.026), X("foot-l", 0.08, 0.024, 0.056, 0.036), X("foot-l", 0.16, 0.012, 0.03, 0.042)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.15, 0.012, 0.03, 0.042), X("toe-l", 0.211, 0.01, 0.026, 0.027)]), "toe-l");
  lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.0, 0, -0.022], [0.03, 0.001, -0.016], [0.15, 0.004, -0.004], [0.3, 0, 0]], [0.021, 0.0125, 0.0105, 0.0145]), "arm-l");
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.014, -0.017, 0], [0.236, -0.014, 0], 0.0085, 0.0076), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.01, 0.015, 0], [0.24, 0.011, 0], 0.0076, 0.009), "forearm-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.0076, 0.02), st(0.05, 0.002, 0, 0.0076, 0.034), st(0.094, 0, -0.004, 0.0076, 0.036), st(0.135, 0, -0.019, 0.0076, 0.031), st(0.16, 0, -0.036, 0.0076, 0.022)]), "hand-l");
  const palm = palmFrame();
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.04, 0.036, 0.0076], palm.at, palm.rotation), "hand-r");
  m({ id: "finger-bones-r", kind: "loft", bone: "hand-r", ...BONE, role: "body", sections: fingerBand(0.033, 0.0076), capStart: true, capEnd: true }, "hand-r");
  caneMembers(out, body);
  joint(out, "torso", T(-0.03, 0.989), 0.017);
  joint(out, "neck", N(0.08, 1.406), 0.0135);
  joint(out, "head", L("head", [0.158, 1.471, 0]), 0.0125);
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    joint(out, `thigh-${side}`, [0, 0, 0], 0.0245);
    joint(out, `shank-${side}`, [0, 0, 0], 0.019);
    joint(out, `foot-${side}`, [0, 0.002, 0], 0.016);
    joint(out, `toe-${side}`, [-0.004, 0.02, 0], 0.01);
    joint(out, `arm-${side}`, [0.002, 0, k * -0.022], 0.0235);
    joint(out, `forearm-${side}`, [0, 0, 0], 0.0155);
    joint(out, `hand-${side}`, [0, 0, 0], 0.012);
  }
  joint(out, "cane", [-CROOK_R, -CROOK_R, 0], 0.0125);
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) mass(out, b);
  return out;
}

/* ---------------------------------------------------------------- 1:200 */

export function body200(): Json[] {
  const out: Json[] = [];
  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, rows(SEAT, [0, 2, 4, 5]).map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e))));
  out.push(loft({ id: "jacket", bone: "torso", material: "tweed", tone: TONE.jacket }, rows(JACKET, [0, 2, 4, 7, 9, 11]).map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e))));
  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", material: "wool-trouser", tone: TONE.trousersFar }, dome([st(0.0, -0.01, 0, 0.085, 0.1, 2.1), st(0.2, 0.0, 0, 0.077, 0.085, 2), st(0.4046, 0.004, 0, 0.066, 0.072, 1.9)], 1, 0.04, 2)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", material: "wool-trouser", tone: TONE.trousersFar }, dome([st(0.0, 0.004, 0, 0.0645, 0.0705, 1.9), st(0.2, -0.006, 0, 0.063, 0.07, 1.9), st(0.39, 0.006, 0, 0.063, 0.074, 2, -10)], -1, 0.04, 2)));
  const rear: Row[] = [[-0.066, 0.01, 0.066, 0.02, 2], [-0.03, 0, 0.09, 0.038, 2.4], [0.06, 0, 0.084, 0.047, 2.4], [0.166, 0, 0.048, 0.051, 2.6]];
  const toe: Row[] = [[0.146, 0, 0.053, 0.0515, 2.6], [0.2, 0, 0.04, 0.04, 2.4], [0.2195, 0, 0.03, 0.02, 2]];
  leg.push(loft({ id: "shoe-rear-l", bone: "foot-l", material: "leather-shoe", tone: TONE.shoeFar }, rear.map(([x, b, t, rw, e]) => X("foot-l", x, b, t, rw, e))));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", material: "leather-shoe", tone: TONE.shoeFar }, toe.map(([x, b, t, rw, e]) => X("toe-l", x, b, t, rw, e))));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "arm-l", material: "tweed", tone: TONE.jacketFar }, dome([st(-0.03, 0.0, -0.04, 0.022, 0.034), st(-0.012, 0.0, -0.03, 0.042, 0.056), st(0.03, 0.002, -0.012, 0.054, 0.063), st(0.19, 0.002, 0, 0.048, 0.055), st(0.3069, -0.004, 0, 0.0455, 0.051)], 1, 0.042, 2)));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", material: "tweed", tone: TONE.jacketFar }, dome([st(0.0, -0.004, 0, 0.044, 0.049), st(0.1, 0.0, 0, 0.042, 0.045), st(0.215, 0, 0, 0.0375, 0.039)], -1, 0.042, 2)));
  pair(out, arm);
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), [st(-0.035, 0, 0, 0.02, 0.028), st(0.05, 0.002, 0, 0.017, 0.042), st(0.12, 0, -0.012, 0.015, 0.04), st(0.165, 0, -0.036, 0.015, 0.03)]));
  const palm = palmFrame();
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.052, 0.045, 0.026], palm.at, palm.rotation));
  out.push({ id: "hand-r-fingers", kind: "loft", bone: "hand-r", material: "skin", role: "body", tone: TONE.hand, sections: fingerBand(0.04, 0.015), capStart: true, capEnd: true });
  out.push(loft(skin("neck", "neck", 0.78), [st(-0.035, 0.004, 0, 0.066, 0.068), st(0.06, -0.005, 0, 0.056, 0.057), st(0.112, -0.01, 0, 0.055, 0.054)]));
  out.push(loft({ id: "collar", bone: "neck", material: "tweed", tone: 0.45 }, [st(-0.03, 0.02, 0, 0.088, 0.08, 2.1, 24), st(0.018, 0.024, 0, 0.073, 0.066, 2.1, 24)]));
  out.push(loft(skin("head"), [st(-0.05, -0.06, 0, 0.012, 0.011), st(-0.04, -0.052, 0, 0.03, 0.028), st(-0.012, -0.03, 0, 0.053, 0.058), st(0.035, -0.004, 0, 0.068, 0.087), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)]));
  out.push(ellipsoid({ id: "hair-white", ...hair }, [0.04, 0.02, 0.066], [0.05, 0.078, 0], aboutW(-6)));
  const crown: [number, number, number, number][] = [[0.101, -0.147, 0.034, 0.008], [0.11, -0.112, 0.07, 0.012], [0.126, -0.068, 0.089, 0.028], [0.133, 0.005, 0.099, 0.048], [0.119, 0.083, 0.081, 0.046], [0.1, 0.118, 0.03, 0.014]];
  out.push(loft({ id: "cap", ...cap }, crown.map(([u, v, rw, rh]) => st(u, v, 0, rw, rh, 2.3, 90))));
  out.push(tube({ id: "cane", ...wood }, canePath(6), [0.011], { slender: true }));
  return out;
}

export function structure200(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.088, 0.088, 0.068], [0.07, 0.002, 0], aboutW(-25)), "head");
  m(capsule({ id: "cervical-spine", bone: "neck", ...BONE }, N(0.08, 1.406), N(0.158, 1.471), 0.015, 0.015), "neck");
  const spine: [number, number, number][] = [[-0.03, 0.989, 0.02], [-0.032, 1.134, 0.018], [-0.02, 1.274, 0.016], [0.08, 1.406, 0.015]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const hoops: [number, number, number, number][] = [[1.144, 0.046, 0.094, 0.126], [1.234, 0.051, 0.1, 0.134], [1.324, 0.058, 0.08, 0.11]];
  hoops.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${3 - i}`, bone: "torso", ...BONE }, ring(rd, 0.015), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.1, 1.364, -0.168), T(0.1, 1.364, 0.168), 0.015, 0.015), "torso");
  m(revolve({ id: "pelvic-ring", bone: "pelvis", ...BONE }, ring(0.068, 0.016), P(-0.022, 0.939), [1, 0, 0], { lateralScale: 1.75 }), "pelvis");
  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0, 0, 0], [0.04, -0.005, 0.058], [0.2, 0.01, 0.032], [0.4, 0, 0.002]], [0.02, 0.018, 0.016, 0.02]), "thigh-l");
  lm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.012, 0.003, 0], [0.385, 0.002, 0], 0.02, 0.016), "shank-l");
  lm(loft({ id: "tarsus-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.05, 0.012, 0.068, 0.022), X("foot-l", 0.02, 0.014, 0.076, 0.03), X("foot-l", 0.16, 0.006, 0.038, 0.04)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.15, 0.006, 0.038, 0.04), X("toe-l", 0.211, 0.004, 0.034, 0.026)]), "toe-l");
  lm(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0.0, 0, -0.022], [0.3, 0, 0], 0.017, 0.015), "arm-l");
  lm(capsule({ id: "radius-ulna-l", bone: "forearm-l", ...BONE }, [0.0, 0, 0], [0.238, 0, 0], 0.0165, 0.015), "forearm-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.015, 0.02), st(0.09, 0, -0.004, 0.015, 0.034), st(0.16, 0, -0.034, 0.015, 0.024)]), "hand-l");
  const palm = palmFrame();
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.045, 0.038, 0.015], palm.at, palm.rotation), "hand-r");
  caneMembers(out, body);
  joint(out, "neck", N(0.08, 1.406), 0.018);
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    joint(out, `thigh-${side}`, [0, 0, 0], 0.026);
    joint(out, `shank-${side}`, [0, 0, 0], 0.023);
    joint(out, `foot-${side}`, [0, 0.002, 0], 0.019);
    joint(out, `arm-${side}`, [0.002, 0, k * -0.022], 0.024);
    joint(out, `forearm-${side}`, [0, 0, 0], 0.019);
    joint(out, `hand-${side}`, [0, 0, 0], 0.017);
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r"]) mass(out, b, 0.014);
  return out;
}

/* ---------------------------------------------------------------- 1:500 */

export function body500(): Json[] {
  const out: Json[] = [];
  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, rows(SEAT, [0, 2, 5]).map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e))));
  out.push(loft({ id: "jacket", bone: "torso", material: "tweed", tone: TONE.jacket }, rows(JACKET, [0, 2, 4, 8, 11]).map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e))));
  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", material: "wool-trouser", tone: TONE.trousersFar }, [st(0.0, -0.01, 0, 0.085, 0.1, 2.1), st(0.4046, 0.004, 0, 0.066, 0.072, 1.9), st(0.44, 0.004, 0, 0.05, 0.055, 1.9)]));
  leg.push(loft({ id: "shank-l", bone: "shank-l", material: "wool-trouser", tone: TONE.trousersFar }, [st(-0.035, 0.004, 0, 0.05, 0.055, 1.9), st(0.0, 0.004, 0, 0.0645, 0.0705, 1.9), st(0.39, 0.004, 0, 0.063, 0.074, 2, -10)]));
  const shoe: Row[] = [[-0.066, 0.008, 0.07, 0.024, 2], [-0.01, 0, 0.092, 0.042, 2.4], [0.12, 0, 0.06, 0.051, 2.5], [0.2195, 0, 0.036, 0.03, 2.2]];
  leg.push(loft({ id: "shoe-l", bone: "foot-l", material: "leather-shoe", tone: TONE.shoeFar }, shoe.map(([x, b, t, rw, e]) => X("foot-l", x, b, t, rw, e))));
  pair(out, leg);
  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "arm-l", material: "tweed", tone: TONE.jacketFar }, [st(-0.03, 0.0, -0.04, 0.024, 0.036), st(0.01, 0.001, -0.018, 0.052, 0.062), st(0.3069, -0.004, 0, 0.0455, 0.051), st(0.345, -0.004, 0, 0.03, 0.034)]));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", material: "tweed", tone: TONE.jacketFar }, [st(-0.036, -0.004, 0, 0.03, 0.034), st(0.0, -0.004, 0, 0.044, 0.049), st(0.215, 0, 0, 0.0375, 0.039)]));
  pair(out, arm);
  out.push(loft(skin("hand-l", "hand-l", TONE.handFar), [st(-0.035, 0, 0, 0.027, 0.03), st(0.02, 0.002, 0, 0.03, 0.04), st(0.09, 0.003, -0.002, 0.03, 0.046), st(0.15, 0.002, -0.004, 0.027, 0.038), st(0.178, 0, -0.005, 0.017, 0.022)]));
  const palm = palmFrame();
  out.push(ellipsoid(skin("hand-r", "hand-r", TONE.hand), [0.054, 0.046, 0.04], [palm.at[0] + 0.006, palm.at[1] - 0.006, palm.at[2]], palm.rotation));
  out.push(loft(skin("neck", "neck", 0.78), [st(-0.035, 0.008, 0, 0.074, 0.072), st(0.112, -0.01, 0, 0.055, 0.054)]));
  out.push(loft(skin("head"), [st(-0.045, -0.055, 0, 0.022, 0.022), st(-0.012, -0.03, 0, 0.053, 0.058), st(0.04, -0.003, 0, 0.069, 0.088), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)]));
  out.push(ellipsoid({ id: "hair-white", ...hair }, [0.04, 0.022, 0.066], [0.05, 0.079, 0]));
  const crown: [number, number, number, number][] = [[0.104, -0.146, 0.034, 0.008], [0.122, -0.07, 0.088, 0.027], [0.13, 0.04, 0.096, 0.052], [0.1, 0.118, 0.03, 0.014]];
  out.push(loft({ id: "cap", ...cap }, crown.map(([u, v, rw, rh]) => st(u, v, 0, rw, rh, 2.3, 90))));
  out.push(tube({ id: "cane", ...wood }, canePath(4), [0.011], { slender: true }));
  return out;
}

export function structure500(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const R = 0.0375;
  m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.088, 0.086, 0.066], [0.07, 0.002, 0], aboutW(-25)), "head");
  m(capsule({ id: "cervical-spine", bone: "neck", ...BONE }, N(0.09, 1.406), N(0.158, 1.471), R, R), "neck");
  m(tube({ id: "spine", bone: "torso", ...BONE }, [T(-0.02, 0.989), T(-0.012, 1.144), T(0.0, 1.284), T(0.085, 1.399)], [R]), "torso");
  m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.1, 1.354, -0.15), T(0.1, 1.354, 0.15), R, R), "torso");
  m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(-0.01, 0.914, -0.1), P(-0.01, 0.914, 0.1), 0.045, 0.045), "pelvis");
  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0.01, 0, 0.012], [0.4, 0, 0], R, R), "thigh-l");
  lm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.01, 0.002, 0], [0.36, 0.002, 0], R, R), "shank-l");
  lm(loft({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.045, 0.006, 0.074, 0.026), X("foot-l", 0.02, 0.004, 0.08, 0.038), X("foot-l", 0.2, 0.002, 0.034, 0.036)]), "foot-l");
  lm(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0.01, 0, -0.012], [0.3, 0, 0], R, 0.035), "arm-l");
  lm(capsule({ id: "radius-ulna-l", bone: "forearm-l", ...BONE }, [0.0, 0, 0], [0.2, 0, 0], 0.0376, 0.0376), "forearm-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  m(loft({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [st(0.0, 0, 0, 0.029, 0.029), st(0.08, 0.002, 0, 0.029, 0.036), st(0.15, 0, 0, 0.029, 0.029)]), "hand-l");
  const palm = palmFrame();
  m(ellipsoid({ id: "hand-bones-r", bone: "hand-r", ...BONE }, [0.046, 0.04, 0.0376], palm.at, palm.rotation), "hand-r");
  caneMembers(out, body);
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    joint(out, `thigh-${side}`, [0, 0, 0], 0.046);
    joint(out, `shank-${side}`, [0, 0, 0], 0.044);
    joint(out, `arm-${side}`, [0.004, 0, k * -0.014], 0.044);
    joint(out, `forearm-${side}`, [0, 0, 0], 0.04);
  }
  for (const b of ["pelvis", "torso"]) mass(out, b, 0.02);
  return out;
}

export const DECISIONS: Record<number, Json[]> = {
  100: [
    { en: "1:100 body, stated anew with about half the sections of 1:50: jacket, seat, one trouser tube per leg bone, shoes with the sole and heel merged into the upper, sleeves and cuffs, the shirt and waistcoat front as one wedge between the jacket fronts. Lapels, pockets, buttons, the sock, glasses, eyes, brow, lips and cheeks are below 15 mm and are left out; nose and ears stay as single forms. The hanging hand is a mitten with a thumb, the gripping hand a palm with the four fingers as one band wrapped over the crook and a thumb. The flat cap keeps its peak (15 mm thick), white hair shows at the nape and temples. The cane is shaft, crook and ferrule.", de: "Körper 1:100, neu gesetzt mit etwa halb so vielen Schnitten wie 1:50: Jacke, Hosensitz, je Beinknochen ein Hosenrohr, Schuhe mit in das Oberteil aufgenommener Sohle und Absatz, Ärmel und Manschetten, Hemd- und Westenbrust als ein Keil zwischen den Jackenkanten. Revers, Taschen, Knöpfe, Socke, Brille, Augen, Brauen, Lippen und Wangen liegen unter 15 mm und entfallen; Nase und Ohren bleiben als einzelne Formen. Die hängende Hand ist ein Fäustling mit Daumen, die greifende eine Handfläche mit den vier Fingern als ein um den Krückgriff gelegtes Band und einem Daumen. Die Schiebermütze behält ihren Schirm (15 mm dick), weißes Haar zeigt sich an Nacken und Schläfen. Der Stock besteht aus Schaft, Krücke und Zwinge." },
    { en: "1:100 thermal: every garment form is the outer layer of its region by reference (no nested shells at this scale); the visible skin of head and neck are open skins, so the scalp under the cap is not counted as face, and the underside of the cap is marked as a face against the head; breath and plume outlets are 15 mm patches on nose, lips and cap.", de: "Thermik 1:100: Jede Kleidungsform ist per Verweis die äußere Schicht ihrer Region (keine geschachtelten Schalen in diesem Maßstab); die sichtbare Haut von Kopf und Hals sind offene Häute, damit die Kopfhaut unter der Mütze nicht als Gesicht zählt, und die Unterseite der Mütze ist als Fläche am Kopf markiert; Atem- und Fahnenauslässe sind 15-mm-Flächen auf Nase, Lippen und Mütze." },
    { en: "1:100 structure: the stooped skeleton with merged parts: spine with kyphosis, five rib hoops, sternum, clavicles, scapulae, sacrum, iliac wings and one ischiopubic bar per side, cranium and jaw, cervical spine; femur with neck and condyles, tibia with a widened head, fibula, tarsus and toe bones; humerus with its head, radius and ulna; the hand bones as one plate, the gripping fingers as one band. Ball joints at the rig joints, masses at the centres of mass, the cane by reference.", de: "Tragwerk 1:100: das gebeugte Skelett mit zusammengefassten Teilen: Wirbelsäule mit Kyphose, fünf Rippenbögen, Brustbein, Schlüsselbeine, Schulterblätter, Kreuzbein, Darmbeinschaufeln und je Seite ein Sitz-Schambein-Stab, Hirnschädel und Kiefer, Halswirbelsäule; Oberschenkelknochen mit Hals und Kondylen, Schienbein mit verbreitertem Kopf, Wadenbein, Fußwurzel und Zehenknochen; Oberarmknochen mit Kopf, Speiche und Elle; die Handknochen als eine Platte, die greifenden Finger als ein Band. Kugelgelenke an den Rig-Gelenken, Massen in den Schwerpunkten, der Stock per Verweis." },
  ],
  200: [
    { en: "1:200 body, stated anew for a 30 mm feature: jacket with six sections that still carry the rounded back and the hanging front, seat, a three-section tube per leg bone, one shoe form per foot bone, sleeves, cuffs, a mitten hand and a palm with a finger band on the crook, neck with collar, head with chin and jaw, nape hair, the cap drawn forward into its peak as one form. Nose, ears, thumbs, temples, shirt front and wrists are merged or left out. The cane is one slender bent rod of its true 22 mm.", de: "Körper 1:200, neu gesetzt für 30 mm Mindestmaß: Jacke mit sechs Schnitten, die weiter den runden Rücken und die hängende Front tragen, Hosensitz, je Beinknochen ein Rohr aus drei Schnitten, je Fußknochen eine Schuhform, Ärmel, Manschetten, eine Fäustlingshand und eine Handfläche mit Fingerband am Krückgriff, Hals mit Kragen, Kopf mit Kinn und Kiefer, Nackenhaar, die Mütze als eine Form nach vorn in ihren Schirm gezogen. Nase, Ohren, Daumen, Schläfen, Hemdbrust und Handgelenke sind aufgenommen oder entfallen. Der Stock ist ein schlanker gebogener Stab in seinen wahren 22 mm." },
    { en: "1:200 thermal: one reference per body form in its region, open skins for face and neck, outlet patches at the size floor. Structure: skull, cervical bar, spine with the kyphosis, three rib hoops, a shoulder girdle bar, a pelvic ring, one bone per limb segment (the femur still runs inwards from the trochanter), foot and toe bones, hand plates, twelve limb joints and the neck base, five masses.", de: "Thermik 1:200: je Körperform ein Verweis in ihrer Region, offene Häute für Gesicht und Hals, Auslassflächen am Mindestmaß. Tragwerk: Schädel, Halsstab, Wirbelsäule mit Kyphose, drei Rippenbögen, ein Schultergürtelstab, ein Beckenring, je Gliedmaßensegment ein Knochen (der Oberschenkelknochen läuft weiter vom Rollhügel nach innen), Fuß- und Zehenknochen, Handplatten, zwölf Gliedmaßengelenke und die Halsbasis, fünf Massen." },
  ],
  500: [
    { en: "1:500 body, stated anew for a 75 mm feature and already minimal: head, cap pulled into its peak, nape hair, neck, the jacket in five sections (the stoop is the identifying feature), seat, thigh and shank per leg, one shoe per foot without a separate toe, sleeve and forearm per arm, the hands as single forms, the cane as one slender rod.", de: "Körper 1:500, neu gesetzt für 75 mm Mindestmaß und bereits minimal: Kopf, in den Schirm gezogene Mütze, Nackenhaar, Hals, die Jacke in fünf Schnitten (die Beugung ist das Erkennungsmerkmal), Hosensitz, Ober- und Unterschenkel je Bein, je Fuß ein Schuh ohne eigenen Vorfuß, Ärmel und Unterarm je Arm, die Hände als einzelne Formen, der Stock als ein schlanker Stab." },
    { en: "1:500 thermal: references per region, open skins for face and neck, outlet patches at the size floor. Structure: a stick skeleton, not the body: skull, neck bar, bent spine, shoulder and pelvic bars, one bar per limb segment at the 75 mm floor, foot bones, hand plates, hip, knee, shoulder and elbow joints, the masses of pelvis and trunk, the cane by reference.", de: "Thermik 1:500: Verweise je Region, offene Häute für Gesicht und Hals, Auslassflächen am Mindestmaß. Tragwerk: ein Stabskelett, nicht der Körper: Schädel, Halsstab, gebogene Wirbelsäule, Schulter- und Beckenstab, je Gliedmaßensegment ein Stab am Mindestmaß von 75 mm, Fußknochen, Handplatten, Hüft-, Knie-, Schulter- und Ellbogengelenke, die Massen von Becken und Rumpf, der Stock per Verweis." },
  ],
};
