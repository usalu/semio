/** 🧶 The three lists of the 1:50 model of `humans/person-with-rollator`: the body (figure and rollator), the thermal model as nested garment shells, open skins and outlet patches, and the structure as her skeleton and the tube frame of the rollator on its four wheels.
 *
 * @see ./r5-person-with-rollator-figure.ts
 * @see ./r5-person-with-rollator-aid.ts */
import type { Json } from "./volumes-lib.ts";
import { ASSET, H, L, X, aboutU, aboutV, aboutW, box, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r5-person-with-rollator-kit.ts";
import { ANKLE_X, BERET_TILT, HEAD, TONE, beretStations, figure50, mid } from "./r5-person-with-rollator-figure.ts";
import { G, aid50 } from "./r5-person-with-rollator-aid.ts";

export const body50 = (): Json[] => [...figure50(), ...aid50()];

/* ---------------------------------------------------------------- thermal */

/** 🧅 The layers under the outer layer of every garment: [layer, depth of its outer face under the garment surface], outermost first. */
export const LAYERS = {
  coat: [["coat-cardigan", 0.006], ["coat-blouse", 0.01], ["coat-air", 0.0108]],
  trousers: [["trousers-tights", 0.003], ["trousers-air", 0.004]],
  shoes: [["shoes-socks", 0.003], ["shoes-air", 0.006]],
  beret: [["beret-air", 0.004]],
} as const;
export const OUTER = { coat: "coat-loden", trousers: "trousers-wool", shoes: "shoes-leather", beret: "beret-felt" } as const;
type Layered = keyof typeof LAYERS;
function shells(solid: Json, region: Layered): Json[] {
  return LAYERS[region].map(([layer, t], i) => ({
    ...solid,
    id: `th-${solid.id}-${layer}`,
    tone: Math.max(0.05, Math.round((solid.tone - 0.08 * (i + 1)) * 100) / 100),
    sections: solid.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) })),
    region,
    layer,
  }));
}
export const AID_REGION = (s: Json): string | null => {
  if (s.bone !== "frame" && !/^wheel-/.test(s.bone)) return null;
  if (s.material === "pu-tyre") return "tyres";
  if (s.material === "rubber-grip") return "grips";
  if (s.material === "cotton-tote") return "bag";
  if (s.material === "nylon-seat" || /^basket-hook/.test(s.id)) return "seat-basket";
  return "frame";
};

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json => { const s = body.find((b) => b.id === id); if (!s) throw new Error(`no body solid ${id}`); return s; };
  const layered = (id: string, region: Layered): void => { out.push(thOf(id, region, OUTER[region])); out.push(...shells(by(id), region)); };
  const ref = (ids: readonly string[], region: string, layer?: string): void => { for (const id of ids) { by(id); out.push(thOf(id, region, layer)); } };
  const sides = (ids: readonly string[]): string[] => ids.flatMap((id) => [id, toRight(id)]);

  for (const id of ["coat", "coat-hip", "collar", ...sides(["skirt-l", "sleeve-l", "forearm-l"])]) layered(id, "coat");
  ref(["coat-placket", "coat-placket-hip", "coat-button-0", "coat-button-1", "coat-button-2", "coat-button-3", ...sides(["coat-lapel-l", "coat-pocket-l"])], "coat", "coat-loden");
  ref(sides(["cardigan-cuff-l"]), "coat", "coat-cardigan");
  ref(["blouse-collar"], "coat", "coat-blouse");
  for (const id of sides(["shank-l"])) layered(id, "trousers");
  for (const id of sides(["shoe-rear-l", "shoe-toe-l"])) layered(id, "shoes");
  ref(sides(["shoe-heel-l", "shoe-sole-l", "shoe-toe-sole-l", "shoe-lace-0-l", "shoe-lace-1-l", "shoe-lace-2-l"]), "shoes", "shoes-leather");
  ref(sides(["sock-l"]), "shoes", "shoes-socks");

  const felt = { bone: "head", material: "wool-beret", tone: TONE.beret };
  out.push({ ...loft({ id: "th-beret", ...felt }, dense(beretStations(), 3), [false, true]), region: "beret", layer: "beret-felt" });
  out.push({ ...loft({ id: "th-beret-beret-air", bone: "head", material: "still-air", tone: 0.2 }, dense(beretStations(), 3).map((q) => ({ ...q, rw: Math.max(0.003, q.rw - 0.004), rv: Math.max(0.003, q.rv - 0.004) })), [false, true]), region: "beret", layer: "beret-air" });
  ref(["beret-stalk"], "beret", "beret-felt");
  ref(["hair-nape", "hair-temple-l", "hair-temple-r"], "hair", "hair-pelt");

  const skin = { bone: "head", material: "skin-aged", tone: TONE.skin };
  const face = [...HEAD.filter((q) => q.at[0] < 0.09), st(0.112, 0.008, 0, 0.0722, 0.091, 2, BERET_TILT)];
  const front = (q: St): St => ({ ...q, at: [q.at[0], q.at[1] - q.rv / 2, q.at[2]], rv: q.rv / 2 });
  out.push({ ...loft({ id: "th-head", ...skin }, dome(dense(face, 3), -1, 0.005, 2).map(front), [true, false]), region: "face-skin" });
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin-aged", tone: 0.8 }, [st(0.024, 0.001, 0, 0.0505, 0.0525), st(0.055, -0.003, 0, 0.049, 0.05), st(0.082, -0.006, 0, 0.049, 0.0495)].map(front), [false, false]), region: "face-skin" });
  ref(["head-nose", "head-nostrils", "head-brow", "head-lips", "head-chin", ...["cheek", "ear", "eye"].flatMap((k) => [`head-${k}-l`, `head-${k}-r`]), "glasses-rim-l", "glasses-rim-r", "glasses-temple-l", "glasses-temple-r", "glasses-bar"], "face-skin");
  ref(body.filter((b) => b.material === "skin-aged" && /^(hand|wrist)/.test(b.id)).map((b) => b.id), "hands");
  for (const s of body) { const region = AID_REGION(s); if (region) out.push(thOf(s.id, region)); }

  out.push({ ...ellipsoid({ id: "th-breath-nose", ...skin }, [0.004, 0.0045, 0.009], [0.0345, -0.0945, 0]), region: "face-skin", tags: ["emitter", "breath-nose"] });
  out.push({ ...ellipsoid({ id: "th-breath-mouth", ...skin }, [0.004, 0.004, 0.0062], [0.019, -0.0818, 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] });
  out.push({ ...ellipsoid({ id: "th-plume", bone: "head", material: "wool-beret", tone: 0.5 }, [0.004, 0.07, 0.065], [0.181, 0.03, 0], aboutW(BERET_TILT)), region: "coat", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

export const BONE = { material: "bone", tone: 0.62 };
export const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
/** 🦴 The spine in the stooped stance [x, y, radius]: lumbar lordosis flattened, thoracic kyphosis, up to the base of the neck. */
export const SPINE: readonly [number, number, number][] = [[-0.022, 0.955, 0.019], [-0.006, 1.0, 0.0185], [0.006, 1.06, 0.0175], [0.014, 1.12, 0.0165], [0.027, 1.18, 0.0155], [0.048, 1.235, 0.0145], [0.074, 1.28, 0.0135], [0.0975, 1.305, 0.0125]];
/** 🦴 The point and radius of the spine axis at the height `y` of the stooped stance. */
export function spineAt(y: number): [number, number, number] {
  let i = 0;
  while (i + 2 < SPINE.length && SPINE[i + 1]![1] < y) i++;
  const a = SPINE[i]!, b = SPINE[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[1]) / (b[1] - a[1])));
  return [a[0] + (b[0] - a[0]) * t, y, a[2] + (b[2] - a[2]) * t];
}
/** 🧱 The 17 vertebral bodies from L5 to T1 along the stooped spine (lumbar bodies 1.35 times as high as thoracic ones), each a short column of the spine's radius with closed discs, and the dorsal direction `n` in which its spinous process points. */
export function vertebrae(): { name: string; a: [number, number]; b: [number, number]; c: [number, number]; n: [number, number]; t: [number, number]; r: number }[] {
  const names = ["l5", "l4", "l3", "l2", "l1", "t12", "t11", "t10", "t9", "t8", "t7", "t6", "t5", "t4", "t3", "t2", "t1"];
  const y0 = SPINE[0]![1], y1 = SPINE[SPINE.length - 1]![1];
  const heights = names.map((n) => (n.startsWith("l") ? 1.35 : 1));
  const total = heights.reduce((x, h) => x + h, 0);
  const out = [];
  let y = y0;
  for (const [i, name] of names.entries()) {
    const h = ((y1 - y0) * heights[i]!) / total;
    const p = spineAt(y), q = spineAt(y + h), m = spineAt(y + h / 2);
    const len = Math.hypot(q[0] - p[0], q[1] - p[1]);
    const t: [number, number] = [(q[0] - p[0]) / len, (q[1] - p[1]) / len];
    const r = m[2];
    const g = Math.max(0.001, len / 2 - r * 0.9);
    const c: [number, number] = [m[0], m[1]];
    out.push({ name, a: [c[0] - t[0] * g, c[1] - t[1] * g] as [number, number], b: [c[0] + t[0] * g, c[1] + t[1] * g] as [number, number], c, n: [-t[1], t[0]] as [number, number], t, r });
    y += h;
  }
  return out;
}
/** 🫁 The rib hoops [y, x of the centre, sagittal half-depth, half-width]. */
export const RIBS: readonly [number, number, number, number][] = [[1.05, 0.058, 0.074, 0.106], [1.085, 0.064, 0.082, 0.114], [1.12, 0.07, 0.087, 0.119], [1.155, 0.077, 0.089, 0.121], [1.19, 0.086, 0.087, 0.119], [1.222, 0.096, 0.082, 0.113], [1.252, 0.105, 0.073, 0.103], [1.278, 0.114, 0.061, 0.087], [1.298, 0.121, 0.047, 0.066]];
/** 🧲 Which body solids of the rollator are its structural members. */
export const AID_MEMBER = (s: Json): string | null => {
  if (/^wheel-/.test(s.bone)) return s.bone;
  if (s.bone !== "frame") return null;
  return /^(main-tube|rear-leg|lower-rail|seat-rail|handle-post|post-clamp|cross-|seat$|axle-|caster-)/.test(s.id) ? "frame" : null;
};
/** 🛞 The support pads under the two wheel pairs: one solid per support anchor, repeated to the other wheel. */
export function supports(): Json[] {
  const pad = (anchor: string, u: number, track: number): Json => ({ ...box({ id: `support-${anchor}`, bone: "frame", material: "pu-tyre", tone: 0.2, role: "detail" }, [0.024, 0.0075, 2 * G.tyreHalf], [u, 0.00375, -track], undefined, 0, { repeat: { count: 2, translation: [0, 0, 2 * track] } }), member: "frame", part: "support", anchor });
  return [pad("wheel-rear-contact", 0, G.rearTrack), pad("wheel-front-contact", G.wheelbase, G.side)];
}

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const joint = (member: string, at: V3, r: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint")); };
  const mass = (member: string): void => {
    const k = kgOf(member);
    const r = 0.0125 * Math.cbrt(k.kg);
    out.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: member === "frame" || /^wheel/.test(member) ? "alu-red-paint" : "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass"));
  };
  const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
  for (const v of vertebrae()) {
    m(capsule({ id: `vertebra-${v.name}`, bone: "torso", ...BONE }, T(v.a[0], v.a[1]), T(v.b[0], v.b[1]), v.r, v.r), "torso");
    m(capsule({ id: `spinous-${v.name}`, bone: "torso", ...BONE }, T(v.c[0] + v.n[0] * v.r * 0.8, v.c[1] + v.n[1] * v.r * 0.8), T(v.c[0] + v.n[0] * v.r * 2.4 - v.t[0] * v.r * 0.9, v.c[1] + v.n[1] * v.r * 2.4 - v.t[1] * v.r * 0.9), 0.0045, 0.0042), "torso");
  }
  RIBS.forEach(([y, x, rd, rw], i) => {
    const drop = 0.03 + 0.004 * i;
    const back = spineAt(y);
    for (const side of [1, -1]) {
      const path: V3[] = [];
      for (let k = 0; k <= 8; k++) {
        const th = (k / 8) * (i < 2 ? 0.86 : 1) * Math.PI;
        const px = x - rd * Math.cos(th);
        path.push(T(k === 0 ? back[0] + 0.004 : px, y - drop * (1 - Math.cos(th)) / 2 + (k === 0 ? 0.004 : 0), side * rw * Math.sin(th)));
      }
      m(tube({ id: `rib-${10 - i}-${side > 0 ? "l" : "r"}`, bone: "torso", ...BONE }, path, [0.0048, 0.0052, 0.0055, 0.0055, 0.0055, 0.005, 0.0045, 0.0042, 0.004]), "torso");
    }
  });
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.135, 0.146, 0.157, 0.014), H("torso", 1.2, 0.163, 0.174, 0.018), H("torso", 1.262, 0.166, 0.177, 0.022)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.164, 1.272, side * 0.02), T(0.098, 1.287, side * 0.15), 0.007, 0.0065), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.068, 0.0065, 0.044], T(0.016, 1.218, side * 0.098), compose(aboutW(6), aboutU(side * 28))), "torso");
  }
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.022, 0.955), P(-0.046, 0.912), P(-0.06, 0.872), P(-0.055, 0.846)], [0.027, 0.025, 0.016, 0.008]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.058, 0.062, 0.0085], P(-0.012, 0.905, side * 0.098), compose(aboutU(side * 30), aboutV(side * -26))), "pelvis");
    m(tube({ id: `iliac-crest-${n}`, bone: "pelvis", ...BONE }, [P(-0.05, 0.93, side * 0.04), P(-0.045, 0.955, side * 0.085), P(-0.018, 0.963, side * 0.122), P(0.022, 0.945, side * 0.128), P(0.04, 0.915, side * 0.118)], [0.008, 0.0095, 0.0095, 0.009, 0.0085]), "pelvis");
    m(capsule({ id: `sacroiliac-${n}`, bone: "pelvis", ...BONE }, P(-0.05, 0.9, side * 0.018), P(-0.044, 0.915, side * 0.06), 0.013, 0.012), "pelvis");
    m(ellipsoid({ id: `acetabulum-${n}`, bone: "pelvis", ...BONE }, [0.027, 0.027, 0.014], P(0, 0.838, side * 0.077), aboutU(side * 0)), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.008, 0.85, side * 0.078), P(0.056, 0.836, side * 0.012), 0.011, 0.01), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.008, 0.828, side * 0.078), P(-0.04, 0.768, side * 0.058), 0.012, 0.013), "pelvis");
  }
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.066, 0.084, 0.0655], [0.108, 0.01, 0]), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.036, 0.033, 0.048], [0.05, -0.043, 0]), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, [0.046, -0.006, side * 0.047], [0.002, -0.01, side * 0.043], 0.007, 0.0075), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, [0.002, -0.01, side * 0.043], [-0.016, -0.056, side * 0.008], 0.0075, 0.0085), "head");
  }
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, [[-0.004, 0.004, 0], [0.025, 0.006, 0], [0.052, 0.005, 0], [0.078, 0.002, 0]], [0.012, 0.0115, 0.011, 0.0105]), "neck");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.032, -0.006, 0.054], 0.015, 0.016), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.021, 0.0165, 0.014], [0.028, -0.008, 0.061]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.043, -0.005, 0.057], [0.115, 0.006, 0.044], [0.2, 0.011, 0.029], [0.287, 0.008, 0.014], [0.368, 0.0, 0.003]], [0.014, 0.0128, 0.0122, 0.0128, 0.0155]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.019, 0.0275, 0.0375], [0.381, -0.005, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.019, 0.0085, 0.018], [0.379, 0.036, 0]), "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.014, 0.024, 0.035], [0.019, -0.002, 0]), "shank-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.029, 0.004, 0.003], [0.118, 0.009, 0.003], [0.246, 0.007, 0.003], [0.372, 0.002, 0.002]], [0.0155, 0.0122, 0.011, 0.0122]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.013, 0.011, 0.0085], [0.384, 0.0, -0.025]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.039, -0.017, 0.031], [0.39, -0.011, 0.027], 0.0062, 0.0066), "shank-l");
  const fz = 0.0846;
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, L("foot-l", [ANKLE_X - 0.042, 0.032, fz]), L("foot-l", [ANKLE_X + 0.006, 0.042, fz]), 0.019, 0.017), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [X("foot-l", ANKLE_X - 0.004, 0.032, 0.068, 0.022), X("foot-l", ANKLE_X + 0.05, 0.026, 0.058, 0.03), X("foot-l", ANKLE_X + 0.095, 0.019, 0.043, 0.035), X("foot-l", ANKLE_X + 0.134, 0.013, 0.03, 0.038)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", ANKLE_X + 0.126, 0.013, 0.03, 0.038), X("toe-l", ANKLE_X + 0.162, 0.012, 0.026, 0.034), X("toe-l", ANKLE_X + 0.182, 0.012, 0.022, 0.024)]), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "arm-l", ...BONE }, [0.02, 0.02, 0.02], [0.002, 0, -0.02]), "arm-l");
  lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.012, 0, -0.016], [0.096, 0.003, -0.007], [0.192, 0.002, -0.002], [0.282, 0, 0]], [0.011, 0.0098, 0.0094, 0.0112]), "arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "arm-l", ...BONE }, [0.011, 0.0125, 0.028], [0.289, 0, 0]), "arm-l");
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.013, -0.015, 0], [0.225, -0.012, 0], 0.0074, 0.0047), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.01, 0.013, 0], [0.229, 0.01, 0], 0.0054, 0.008), "forearm-l");
  const rays = body.filter((b) => b.bone === "hand-l" && b.kind === "capsule");
  const carpus: V3 = [0.012, 0.0, 0];
  lm(ellipsoid({ id: "carpals-l", bone: "hand-l", ...BONE }, [0.013, 0.012, 0.02], carpus), "hand-l");
  for (const ray of rays) {
    lm(capsule({ id: ray.id.replace("hand-", "bones-"), bone: "hand-l", ...BONE }, ray.p0, ray.p1, 0.004, 0.0038), "hand-l");
    if (/finger-\d$|thumb$/.test(ray.id)) lm(capsule({ id: `${ray.id.replace("hand-", "bones-")}-metacarpal`, bone: "hand-l", ...BONE }, mid(carpus, ray.p0, 0.22), ray.p0, 0.0045, 0.0042), "hand-l");
  }
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));

  for (const s of body) { const member = AID_MEMBER(s); if (member) out.push({ id: `member-${s.id}`, of: s.id, member, part: "member" }); }
  out.push(...supports());

  joint("torso", T(SPINE[0]![0], SPINE[0]![1]), 0.0165);
  joint("neck", [0, 0.002, 0], 0.013);
  joint("head", [0, 0.002, 0], 0.012);
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    joint(`thigh-${side}`, [0, 0, 0], 0.0225);
    joint(`shank-${side}`, [0, 0, 0], 0.0175);
    joint(`foot-${side}`, [0, 0.002, 0], 0.0145);
    joint(`toe-${side}`, [-0.004, 0.019, 0], 0.009);
    joint(`arm-${side}`, [0.002, 0, k * -0.02], 0.0215);
    joint(`forearm-${side}`, [0, 0, 0], 0.014);
    joint(`hand-${side}`, [0, 0, 0], 0.011);
  }
  for (const [id, w] of [["wheel-rear", G.rearTrack], ["wheel-front", G.side]] as [string, number][]) for (const side of [1, -1]) out.push(part(ellipsoid({ id: `joint-${id}-${side > 0 ? "l" : "r"}`, bone: id, material: "nylon-grey", tone: 0.3 }, [0.012, 0.012, 0.012], [0, 0, side * w]), id, "joint"));
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r", "hand-l", "hand-r", "frame"]) mass(b);
  for (const [id, w] of [["wheel-rear", G.rearTrack], ["wheel-front", G.side]] as [string, number][]) for (const side of [1, -1]) {
    const r = 0.0125 * Math.cbrt(kgOf(id).kg / 2);
    out.push(part(ellipsoid({ id: `mass-${id}-${side > 0 ? "l" : "r"}`, bone: id, material: "nylon-grey", tone: 0.2, role: "detail" }, [r, r, r], [0, 0, side * w]), id, "mass"));
  }
  return out;
}
