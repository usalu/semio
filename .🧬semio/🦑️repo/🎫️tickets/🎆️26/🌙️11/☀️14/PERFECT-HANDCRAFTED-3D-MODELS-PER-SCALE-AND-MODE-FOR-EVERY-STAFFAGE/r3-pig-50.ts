/** 🐖️ Round 3 pig at 1:50: the finest model of a 100 kg finishing pig. Body: barrel in two overlapping lofts, ham and shoulder masses, upper arm, short deep neck, dished skull with jowled lower jaw, rooting disc, eyes, lop ears, corkscrew tail, per leg gaskin or forearm, point of hock, cannon with fetlock, two dewclaws, pastern and two claws. Thermal: the body surfaces per region and the outlets. Structure: the skeleton.
 * Usage: bun r3-pig-50.ts [--write]
 * @see ./r3-pig-kit.ts
 * @see ./r3-animals-pig.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Pig, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-pig-kit.ts";

export const SKIN: Look = { material: "pig-skin", tone: 0.34 };
export const LEG: Look = { material: "pig-skin", tone: 0.38 };
export const FACE: Look = { material: "pig-skin", tone: 0.36 };
export const DISC: Look = { material: "pig-skin", tone: 0.52 };
export const EAR: Look = { material: "pig-skin", tone: 0.44 };
export const TAIL: Look = { material: "pig-skin", tone: 0.4 };
export const HORN: Look = { material: "horn", tone: 0.82 };
export const EYE: Look = { material: "pig-skin", tone: 0.95 };
export const SIDES = [-1, 1] as const;

/** Barrel: x, top of the back, underline, half width, section exponent. */
export const TRUNK = [
  [-0.118, 0.612, 0.5, 0.06, 2.0],
  [-0.105, 0.632, 0.44, 0.1, 2.1],
  [-0.075, 0.65, 0.39, 0.128, 2.2],
  [-0.035, 0.662, 0.355, 0.147, 2.3],
  [0.02, 0.671, 0.333, 0.157, 2.4],
  [0.1, 0.678, 0.318, 0.161, 2.5],
  [0.2, 0.682, 0.304, 0.163, 2.5],
  [0.3, 0.684, 0.296, 0.164, 2.5],
  [0.4, 0.682, 0.292, 0.164, 2.5],
  [0.5, 0.675, 0.29, 0.162, 2.5],
  [0.58, 0.664, 0.29, 0.158, 2.45],
  [0.65, 0.652, 0.296, 0.15, 2.35],
  [0.71, 0.642, 0.31, 0.138, 2.25],
  [0.76, 0.634, 0.335, 0.126, 2.15],
  [0.8, 0.626, 0.37, 0.108, 2.05],
  [0.83, 0.6, 0.43, 0.075, 2.0],
  [0.85, 0.57, 0.48, 0.035, 2.0],
] as const;
export const hullRows = (rows: readonly (readonly number[])[]): number[][] => rows.map(([x, top, bot, hw, e]) => [x!, (top! + bot!) / 2, (top! - bot!) / 2, hw!, e!]);
/** Neck on the neck bone: s, v, half depth, half width. */
export const NECK = [
  [-0.04, -0.03, 0.13, 0.112],
  [0.03, -0.03, 0.13, 0.114],
  [0.08, -0.028, 0.122, 0.106],
  [0.14, -0.025, 0.104, 0.094],
  [0.19, -0.024, 0.084, 0.084],
] as const;
/** Head on the head bone: s, v, half depth, half width. */
export const SKULL = [
  [-0.045, 0.012, 0.07, 0.074],
  [0.0, 0.012, 0.076, 0.082],
  [0.04, 0.011, 0.073, 0.084],
  [0.08, 0.01, 0.066, 0.08],
  [0.12, 0.008, 0.055, 0.068],
  [0.16, 0.006, 0.044, 0.053],
  [0.2, 0.005, 0.036, 0.042],
  [0.235, 0.005, 0.031, 0.036],
  [0.252, 0.005, 0.029, 0.033],
] as const;
/** Jowl and lower jaw in the head frame: s, v, half depth, half width. */
export const MANDIBLE = [
  [-0.01, -0.04, 0.045, 0.06],
  [0.035, -0.058, 0.055, 0.08],
  [0.085, -0.06, 0.05, 0.076],
  [0.135, -0.05, 0.037, 0.058],
  [0.18, -0.042, 0.026, 0.042],
  [0.215, -0.036, 0.02, 0.032],
  [0.238, -0.032, 0.013, 0.024],
] as const;
export const DISC_T = [[0.243, 0.005, 0.026, 0.03], [0.256, 0.005, 0.0305, 0.035], [0.267, 0.005, 0.029, 0.0335]] as const;
/** Ham: world x of the centre, height, fore-aft half extent, lateral half extent, lateral centre. */
export const HAM = [[-0.02, 0.6, 0.06, 0.045, 0.07], [-0.035, 0.55, 0.095, 0.066, 0.085], [-0.037, 0.48, 0.112, 0.075, 0.09], [-0.027, 0.41, 0.108, 0.074, 0.092], [-0.008, 0.35, 0.09, 0.066, 0.093], [0.012, 0.3, 0.066, 0.055, 0.094], [0.022, 0.268, 0.046, 0.045, 0.095]] as const;
export const GASKIN = [[0.045, -0.005, 0.048, 0.043, 0.095], [0.08, -0.006, 0.046, 0.041, 0.095], [0.11, -0.008, 0.037, 0.033, 0.095], [0.15, -0.006, 0.03, 0.028, 0.095], [0.185, -0.002, 0.027, 0.026, 0.095], [0.2, 0, 0.026, 0.025, 0.095]] as const;
export const HIND_CANNON = [[-0.018, -0.003, 0.026, 0.025, 0.095], [0.02, -0.001, 0.0235, 0.0235, 0.095], [0.07, 0, 0.021, 0.0215, 0.095], [0.108, 0, 0.0225, 0.023, 0.095], [0.13, 0, 0.022, 0.023, 0.095]] as const;
export const PASTERN = [[-0.012, 0, 0.0215, 0.023], [0.025, 0, 0.0195, 0.024], [0.055, 0.002, 0.019, 0.026]] as const;
export const CLAW = [[-0.052, 0.012, 0.012, 0.009, 3, 0.0125], [-0.042, 0.0165, 0.0165, 0.0115, 3, 0.013], [-0.03, 0.0175, 0.0175, 0.012, 3, 0.0135], [-0.017, 0.0135, 0.0135, 0.0115, 3, 0.014], [-0.007, 0.008, 0.008, 0.009, 3, 0.0145], [-0.001, 0.0045, 0.0045, 0.006, 3, 0.015]] as const;
export const SHOULDER = [[-0.03, 0, 0.034, 0.03, 0.1], [0.03, -0.004, 0.062, 0.05, 0.108], [0.1, -0.008, 0.074, 0.056, 0.11], [0.16, -0.008, 0.064, 0.05, 0.106], [0.21, -0.004, 0.044, 0.038, 0.098]] as const;
export const UPPER_ARM = [[-0.01, 0, 0.042, 0.034, 0.096], [0.06, -0.008, 0.054, 0.04, 0.098], [0.13, -0.012, 0.054, 0.042, 0.098], [0.185, -0.008, 0.044, 0.035, 0.095]] as const;
export const FOREARM = [[-0.005, -0.006, 0.045, 0.04, 0.095], [0.02, 0.003, 0.044, 0.039, 0.094], [0.06, 0.004, 0.036, 0.033, 0.094], [0.1, 0.002, 0.029, 0.027, 0.093], [0.13, 0.001, 0.026, 0.025, 0.093], [0.15, 0, 0.027, 0.026, 0.093]] as const;
export const FORE_CANNON = [[-0.01, 0, 0.0265, 0.0255, 0.093], [0.03, 0, 0.023, 0.023, 0.093], [0.062, 0, 0.022, 0.022, 0.093], [0.088, 0, 0.0235, 0.024, 0.093], [0.1, 0, 0.0225, 0.0235, 0.093]] as const;
export const HIND_Z = 0.095;
export const FORE_Z = 0.093;
/** Lop ear on its ear bone: s, v, lateral centre, half width, half thickness. */
export const EAR_T = [[-0.015, -0.004, 0.055, 0.024, 0.009], [0.015, 0.0, 0.072, 0.042, 0.0065], [0.055, 0.002, 0.087, 0.055, 0.005], [0.095, 0.0, 0.093, 0.052, 0.0047], [0.135, -0.004, 0.092, 0.036, 0.0045], [0.168, -0.008, 0.088, 0.013, 0.0045]] as const;
/** Corkscrew tail: world point and radius. */
export const TAIL_T = [[-0.098, 0.612, 0, 0.013], [-0.125, 0.626, 0, 0.0115], [-0.146, 0.622, 0.008, 0.0105], [-0.152, 0.6, 0.016, 0.0095], [-0.14, 0.585, 0.016, 0.0085], [-0.128, 0.594, 0.006, 0.0075], [-0.13, 0.61, -0.004, 0.0065], [-0.142, 0.614, -0.01, 0.0052]] as const;

/** 👂️ Lop ear of side `sd`: a leaf that leaves the poll and hangs forward and down over the eye, sloping outward like a roof over the face. */
export function ear(S: Pig, id: string, sd: -1 | 1, look: Look, rows: readonly (readonly number[])[] = EAR_T, shape?: string, thick = 0): Json {
  const b = `ear-${sx(sd)}`;
  const pts: PP[] = rows.map(([s, v, z, rl, rs]) => ({ p: S.at(b, s!, v!, sd * z!), rl: rl!, rs: Math.max(rs!, thick), e: 2.3 }));
  return S.run(id, b, pts, look, { lat: [0, -0.72, sd * 0.69], ...(shape === undefined ? {} : { shape }) });
}

/** 🍖️ Ham of side `sd`: a hanging mass from the rump to just above the hock, proud of the flank and behind the pin bone. */
export function ham(S: Pig, id: string, sd: -1 | 1, look: Look, rows: readonly (readonly number[])[] = HAM, shape?: string): Json {
  return S.run(id, `thigh-h${sx(sd)}`, rows.map(([x, y, rs, rl, z]) => ({ p: [x!, y!, sd * z!] as V3, rl: rl!, rs: rs!, e: 2.3 })), look, { lat: [0, 0, 1], ...(shape === undefined ? {} : { shape }) });
}

export function body(S: Pig): void {
  const T = dense(hullRows(TRUNK), 2);
  const hind = T.filter((r) => r[0]! <= 0.4 + 1e-9).map((r) => (r[0]! > 0.341 ? [r[0]!, r[1]!, r[2]! - 0.006, r[3]! - 0.006, r[4]!] : [...r]));
  const fore = T.filter((r) => r[0]! >= 0.34 - 1e-9).map((r) => [...r]);
  S.hull("hindbody", "lumbar", hind, SKIN, { shape: "hindbody" });
  S.hull("chest", "thorax", fore, SKIN, { shape: "chest" });
  S.limb("neck", "neck", dense(NECK, 2), SKIN, { shape: "neck", e: 2.3 });
  S.limb("head", "head", dense(SKULL, 2), FACE, { shape: "head", e: 2.5 });
  S.limb("jaw", "jaw", dense(MANDIBLE, 2), FACE, { frame: "head", shape: "jaw", e: 2.3 });
  S.limb("snout", "head", DISC_T, DISC, { shape: "snout", e: 2.3 });
  S.run("tail-root", "tail-1", TAIL_T.slice(0, 3).map(([x, y, z, r]) => ({ p: [x, y, z] as V3, rl: r, rs: r })), TAIL, { shape: "tail-root" });
  S.run("tail-curl", "tail-2", TAIL_T.slice(2).map(([x, y, z, r]) => ({ p: [x, y, z] as V3, rl: r, rs: r })), TAIL, { shape: "tail-curl" });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.ball(`${n}-eye`, "head", S.at("head", 0.118, 0.042, sd * 0.065), [0.008, 0.012, 0.011], EYE, [0, 0, sd], sd < 0 ? "eye" : "eye-f|head");
    ear(S, `${n}-ear`, sd, EAR, EAR_T, `ear-${k}`);
    ham(S, `${n}-ham`, sd, SKIN, dense(HAM, 2), sd < 0 ? "ham-n|hindbody" : "hindbody");
    S.limb(`${n}-gaskin`, `shank-h${k}`, dense(GASKIN, 2), LEG, { sd, shape: `gaskin-${k}` });
    const mt = `metatarsus-h${k}`;
    S.run(`${n}-hock-point`, mt, [{ p: S.at(mt, 0.035, -0.008, sd * HIND_Z), rl: 0.016, rs: 0.016 }, { p: S.at(mt, 0.0, -0.02, sd * HIND_Z), rl: 0.015, rs: 0.016 }, { p: S.at(mt, -0.03, -0.024, sd * HIND_Z), rl: 0.012, rs: 0.012 }], LEG, { shape: `cannon-h-${k}` });
    S.limb(`${n}-hind-cannon`, mt, dense(HIND_CANNON, 2), LEG, { sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-shoulder`, `scapula-f${k}`, dense(SHOULDER, 2), SKIN, { sd, shape: sd < 0 ? "shoulder-n|chest" : "chest", e: 2.4 });
    S.limb(`${n}-upper-arm`, `humerus-f${k}`, UPPER_ARM, SKIN, { sd, shape: sd < 0 ? "shoulder-n|chest" : "chest", e: 2.3 });
    S.limb(`${n}-forearm`, `forearm-f${k}`, dense(FOREARM, 2), LEG, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, dense(FORE_CANNON, 2), LEG, { sd, shape: `cannon-f-${k}` });
    for (const [leg, cannon, z] of [["hind", mt, HIND_Z], ["fore", `metacarpus-f${k}`, FORE_Z]] as const) {
      const h = leg === "hind" ? "h" : "f";
      const hoof = `hoof-${h}${k}`;
      const f = S.frame(hoof);
      const len = S.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
      const toe = f.o[0]! + f.u[0]! * len;
      const ground = f.o[1]! + f.u[1]! * len;
      const fet = f.o;
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.ball(`${n}-${leg}-dewclaw-${j}`, hoof, [fet[0]! - 0.018, fet[1]! - 0.034, sd * z + w * 0.017], [0.016, 0.0085, 0.009], HORN, [-0.35, -1, 0], `hoof-${h}-${k}`);
      void cannon;
      S.limb(`${n}-${leg}-pastern`, hoof, PASTERN.map((r) => [...r, z]), LEG, { sd, shape: `hoof-${h}-${k}` });
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.hull(`${n}-${leg}-claw-${j}`, hoof, CLAW.map((r) => [r[0], r[1] + ground, r[2], r[3], r[4], sd * z + w * r[5]]), HORN, { x0: toe, sd: 1, shape: `hoof-${h}-${k}` });
    }
  }
}

export const REGION: readonly (readonly [RegExp, string, string?])[] = [
  [/^(hindbody|chest|trunk|neck)$|-(ham|shoulder|upper-arm)$/, "skin-trunk", "bristles"], [/-(gaskin|forearm|upper|hock-point|cannon|pastern|lower|leg)$/, "skin-legs", "bristles"],
  [/^(head|jaw)$|-eye$/, "skin-head", "bristles"], [/^snout$/, "snout"], [/-ear$|^ears$/, "ears"], [/^tail/, "tail"], [/-(claw|dewclaw)(-\d)?$|-hoof$/, "hooves"],
];

/** 🌡️ Thermal list of a scale: every body solid as the surface of its region, and the outlets (nostrils in the rooting disc, mouth, plume patch on the back) as own solids. */
export function thermal(S: Pig, T: Pig, o: { nostrilR: number; mouthR: number; plume: readonly [V3, V3]; pair?: boolean; skip?: RegExp; discS?: number }): Json[] {
  const out: Json[] = S.body.filter((b) => o.skip === undefined || !o.skip.test(b.id)).map((b) => {
    const hit = REGION.find(([re]) => re.test(b.id));
    if (hit === undefined) throw new Error(`no region for ${b.id}`);
    return { id: `th-${b.id}`, of: b.id, region: hit[1], ...(hit[2] === undefined ? {} : { layer: hit[2] }) };
  });
  const hu = T.frame("head").u;
  const hv = T.frame("head").v;
  const s = o.discS ?? 0.268;
  const E: Look = { material: "pig-skin", tone: 0.9 };
  if (o.pair === false) T.ball("th-breath-nose", "head", T.at("head", s, 0.007, 0), [0.004, o.nostrilR * 1.6, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  else for (const sd of SIDES) T.ball(`th-breath-nose-${sx(sd)}`, "head", T.at("head", s, 0.007, sd * 0.012), [0.004, o.nostrilR, o.nostrilR * 1.2], E, [hu[0]!, hu[1]!, 0]);
  T.ball("th-breath-mouth", "head", T.at("head", s - 0.03, -0.026, 0), [0.0045, o.mouthR, o.nostrilR], E, [hu[0]! - hv[0]! * 0.8, hu[1]! - hv[1]! * 0.8, 0]);
  T.ball("th-plume-back", "thorax", o.plume[0], o.plume[1], { material: "pig-skin", tone: 0.2 }, [1, 0, 0]);
  for (const t of T.body) out.push({ ...t, region: t.id === "th-plume-back" ? "skin-trunk" : t.id === "th-breath-mouth" ? "skin-head" : "snout", ...(t.id === "th-plume-back" || t.id === "th-breath-mouth" ? { layer: "bristles" } : {}), tags: ["emitter", t.id === "th-plume-back" ? "plume-back" : t.id === "th-breath-mouth" ? "breath-mouth" : "breath-nose"] });
  return out;
}

export const BONE: Look = { material: "bone", tone: 0.18 };
export const JOINT: Look = { material: "bone", tone: 0.5 };
export const MASS: Look = { material: "pig-skin", tone: 0.7 };
/** Vertebral axis (world, standing): thoracic and cervical. */
export const SPINE_L: readonly V3[] = [[0.1, 0.6, 0], [0.22, 0.612, 0], [0.345, 0.615, 0]];
export const SPINE_T: readonly V3[] = [[0.345, 0.615, 0], [0.43, 0.607, 0], [0.52, 0.585, 0], [0.61, 0.552, 0], [0.69, 0.515, 0]];
export const SPINE_C: readonly V3[] = [[0.69, 0.515, 0], [0.735, 0.505, 0], [0.785, 0.51, 0], [0.825, 0.522, 0], [0.855, 0.535, 0]];
export const spineY = (x: number): number => {
  for (let i = 1; i < SPINE_T.length; i++) if (x <= SPINE_T[i]![0]) { const a = SPINE_T[i - 1]!; const b = SPINE_T[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return SPINE_T[SPINE_T.length - 1]![1];
};
/** Ribs 1 to 14: half width of the cage, depth from the spine to the ventral end, caudal sweep of the ventral end. */
export const RIBS = [[0.05, 0.15, -0.01], [0.066, 0.175, 0], [0.082, 0.2, 0.01], [0.095, 0.215, 0.02], [0.106, 0.228, 0.03], [0.114, 0.24, 0.04], [0.12, 0.25, 0.05], [0.125, 0.25, 0.055], [0.128, 0.24, 0.06], [0.13, 0.225, 0.06], [0.13, 0.2, 0.06], [0.127, 0.17, 0.06], [0.12, 0.14, 0.055], [0.11, 0.11, 0.05]] as const;
export const ribPath = (i: number, sd: number, r: number): PP[] => {
  const [w, d, k] = RIBS[i]!;
  const x = ribX(i);
  const y = spineY(x) - 0.012;
  const bow: PP[] = ([[0, 0, 0.16], [0.2, 0.12, 0.72], [0.5, 0.42, 1], [0.8, 0.75, 0.88], [1, 1, 0.6]] as const).map(([a, b, c]) => ({ p: [x - a * k, y - b * d, sd * c * w] as V3, rl: r, rs: r }));
  if (i < STERNAL) { const xs = 0.7 - i * 0.028; bow.push({ p: [xs, sternumY(xs) + 0.004, sd * 0.012], rl: r, rs: r }); }
  return bow;
};
/** Sternum (world): manubrium to xiphoid; the first seven ribs reach it through their costal cartilages. */
export const STERNUM: readonly V3[] = [[0.728, 0.38, 0], [0.665, 0.335, 0], [0.58, 0.318, 0], [0.5, 0.32, 0]];
export const STERNAL = 7;
export function sternumY(x: number): number {
  for (let i = 1; i < STERNUM.length; i++) if (x >= STERNUM[i]![0]) { const a = STERNUM[i - 1]!; const b = STERNUM[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return STERNUM[STERNUM.length - 1]![1];
}
/** Spinous processes T1 to T14: height above the vertebral body, caudal lean of the tip. The long processes of T2 to T5 carry the withers; T12 stands upright (anticlinal). */
export const SPINOUS_T = [[0.05, 0.02], [0.066, 0.028], [0.074, 0.032], [0.076, 0.032], [0.074, 0.03], [0.07, 0.027], [0.065, 0.023], [0.06, 0.019], [0.056, 0.015], [0.052, 0.011], [0.049, 0.007], [0.046, 0.003], [0.044, 0], [0.043, -0.002]] as const;
/** Lumbar vertebrae L1 to L6: x of the body, height of the spinous process (leaning forward), half span of the transverse processes. */
export const LUMBAR_V = [[0.325, 0.042, 0.05], [0.283, 0.042, 0.056], [0.241, 0.042, 0.06], [0.199, 0.04, 0.06], [0.157, 0.038, 0.056], [0.118, 0.034, 0.048]] as const;
export const ribX = (i: number): number => 0.675 - i * 0.0242;
export const lumbarY = (x: number): number => (x >= 0.22 ? 0.612 + ((x - 0.22) / 0.125) * 0.003 : 0.6 + ((x - 0.1) / 0.12) * 0.012);
/** 🪶️ One spinous process as a thin plate from the vertebral body to its tip. */
export function spinous(K: Pig, id: string, bone: string, x: number, y: number, h: number, lean: number, half: number, thick: number): Json {
  return K.run(id, bone, [{ p: [x, y, 0], rl: thick, rs: half }, { p: [x - lean * 0.6, y + h * 0.6, 0], rl: thick, rs: half * 0.9 }, { p: [x - lean, y + h, 0], rl: thick, rs: half * 0.7 }], BONE, { floor: thick });
}
export const JZ: Record<string, number> = { thigh: 0.078, shank: 0.1, metatarsus: 0.096, "hoof-h": 0.095, scapula: 0.07, humerus: 0.105, forearm: 0.098, metacarpus: 0.093, "hoof-f": 0.093, ear: 0.056 };
export const FEMUR = [[0, 0, 0.022, 0.022, 0.078], [0.025, 0, 0.024, 0.022, 0.082], [0.06, 0, 0.0145, 0.0145, 0.087], [0.14, 0, 0.0145, 0.0145, 0.095], [0.18, 0, 0.024, 0.024, 0.099], [0.203, 0.004, 0.026, 0.026, 0.1]] as const;
export const TIBIA = [[0, 0, 0.026, 0.026, 0.1], [0.03, 0.004, 0.02, 0.018, 0.099], [0.08, 0, 0.0125, 0.012, 0.098], [0.15, 0, 0.012, 0.0115, 0.096], [0.178, 0, 0.017, 0.017, 0.096], [0.19, 0, 0.016, 0.016, 0.096]] as const;
export const TARSUS = [[-0.008, 0, 0.019, 0.02, 0.096], [0.02, 0, 0.018, 0.019, 0.096], [0.036, 0, 0.015, 0.018, 0.096]] as const;
export const METAPODIAL_H = [[0.03, 0, 0.0085, 0.0078], [0.045, 0, 0.0062, 0.0062], [0.1, 0, 0.0062, 0.0062], [0.12, 0, 0.0085, 0.008], [0.131, 0, 0.008, 0.008]] as const;
export const DIGIT = [[0.002, 0, 0.0085, 0.0078], [0.03, 0, 0.0065, 0.0065], [0.045, 0, 0.008, 0.0075], [0.062, 0, 0.0075, 0.007]] as const;
export const SCAPULA = [[-0.012, 0, 0.055, 0.0045, 0.07], [0.02, 0, 0.052, 0.0045, 0.076], [0.09, 0, 0.034, 0.005, 0.09], [0.15, 0, 0.018, 0.008, 0.1], [0.184, 0, 0.02, 0.014, 0.104]] as const;
export const HUMERUS = [[0, 0, 0.024, 0.022, 0.105], [0.02, 0.008, 0.024, 0.021, 0.105], [0.06, 0, 0.015, 0.014, 0.103], [0.13, 0, 0.0145, 0.014, 0.1], [0.165, 0, 0.021, 0.021, 0.098], [0.18, 0, 0.02, 0.022, 0.098]] as const;
export const RADIUS = [[0, 0.004, 0.016, 0.019, 0.098], [0.03, 0.005, 0.012, 0.014, 0.097], [0.1, 0.005, 0.012, 0.014, 0.094], [0.128, 0.003, 0.016, 0.019, 0.093], [0.138, 0.002, 0.015, 0.019, 0.093]] as const;
export const CARPUS = [[-0.006, 0, 0.017, 0.02, 0.093], [0.012, 0, 0.017, 0.02, 0.093], [0.026, 0, 0.015, 0.018, 0.093]] as const;
export const METAPODIAL_F = [[0.022, 0, 0.0085, 0.0078], [0.034, 0, 0.0062, 0.0062], [0.07, 0, 0.0062, 0.0062], [0.084, 0, 0.0085, 0.008], [0.092, 0, 0.008, 0.008]] as const;
export const CRANIUM = [[-0.012, 0.035, 0.04, 0.035], [0.0, 0.03, 0.052, 0.045], [0.04, 0.025, 0.055, 0.062], [0.08, 0.02, 0.05, 0.064], [0.12, 0.015, 0.042, 0.052], [0.16, 0.012, 0.032, 0.036], [0.2, 0.01, 0.026, 0.028], [0.24, 0.008, 0.022, 0.024], [0.258, 0.006, 0.016, 0.02]] as const;
export const JAWBONE = [[0.028, -0.02, 0.03, 0.058], [0.05, -0.05, 0.045, 0.062], [0.1, -0.058, 0.03, 0.05], [0.15, -0.048, 0.02, 0.036], [0.2, -0.04, 0.014, 0.024], [0.236, -0.036, 0.01, 0.016]] as const;

/** 🦴️ Structure at 1:50: the skeleton, bone by bone, joints at the rig joints, the two trunk masses at their centres of mass. */
export function structure(K: Pig): Json[] {
  const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });
  const fl = 0.0042;
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p, i) => ({ p, rl: 0.021 - i * 0.001, rs: 0.018 })), BONE), "lumbar");
  for (const [i, [x, h, w]] of LUMBAR_V.entries()) {
    mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", x, lumbarY(x) + 0.012, h, -0.008, 0.014, fl), "lumbar");
    mem(K.run(`lumbar-transverse-${i + 1}`, "lumbar", [{ p: [x + 0.006, lumbarY(x) + 0.004, -w], rl: 0.009, rs: fl }, { p: [x, lumbarY(x) + 0.006, 0], rl: 0.012, rs: 0.006 }, { p: [x + 0.006, lumbarY(x) + 0.004, w], rl: 0.009, rs: fl }], BONE, { lat: [1, 0, 0], floor: fl }), "lumbar");
  }
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: 0.017, rs: 0.016 })), BONE), "thorax");
  for (const [i, [h, lean]] of SPINOUS_T.entries()) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.01, h, lean, 0.0085, fl), "thorax");
  mem(K.run("cervical-spine", "neck", SPINE_C.map((p, i) => ({ p, rl: i > 2 ? 0.03 : 0.025, rs: 0.02 })), BONE), "neck");
  mem(K.run("sacrum", "pelvis", [{ p: [0.1, 0.6, 0], rl: 0.036, rs: 0.018 }, { p: [0.03, 0.612, 0], rl: 0.026, rs: 0.015 }, { p: [-0.04, 0.616, 0], rl: 0.014, rs: 0.011 }, { p: [-0.098, 0.612, 0], rl: 0.009, rs: 0.008 }], BONE), "pelvis");
  mem(K.run("pubis", "pelvis", [{ p: [0.012, 0.478, -0.068], rl: 0.011, rs: 0.011 }, { p: [0.014, 0.466, 0], rl: 0.014, rs: 0.011 }, { p: [0.012, 0.478, 0.068], rl: 0.011, rs: 0.011 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: [0.012, 0.02, 0.023, 0.014][i]!, rs: [0.011, 0.011, 0.01, 0.007][i]! })), BONE), "thorax");
  mem(K.limb("skull", "head", CRANIUM, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", JAWBONE, BONE, { frame: "head", e: 2.3 }), "jaw");
  mem(K.run("caudal-1", "tail-1", TAIL_T.slice(0, 3).map(([x, y, z, r]) => ({ p: [x, y, z] as V3, rl: Math.max(fl, r * 0.5), rs: Math.max(fl, r * 0.5) })), BONE), "tail-1");
  mem(K.run("caudal-2", "tail-2", TAIL_T.slice(2).map(([x, y, z]) => ({ p: [x, y, z] as V3, rl: fl, rs: fl })), BONE), "tail-2");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.128, 0.626, sd * 0.098], rl: 0.009, rs: 0.036 }, { p: [0.09, 0.606, sd * 0.07], rl: 0.008, rs: 0.03 }, { p: [0.045, 0.56, sd * 0.068], rl: 0.011, rs: 0.016 }, { p: [0.0, 0.508, sd * 0.076], rl: 0.02, rs: 0.024 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.0, 0.506, sd * 0.076], rl: 0.015, rs: 0.017 }, { p: [-0.055, 0.508, sd * 0.07], rl: 0.011, rs: 0.014 }, { p: [-0.1, 0.526, sd * 0.066], rl: 0.014, rs: 0.021 }], BONE), "pelvis");
    for (let i = 0; i < 14; i++) mem(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, 0.0048), BONE), "thorax");
    mem(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.515, 0.322, sd * 0.03], rl: fl, rs: fl }, ...[7, 8, 9, 10, 11, 12, 13].map((i) => ({ p: ribPath(i, sd, fl)[4]!.p, rl: fl, rs: fl }))], BONE), "thorax");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, [[0.0, 0.0, 0.05, 0.012, fl], [0.04, 0.003, 0.058, 0.03, fl], [0.09, 0.0, 0.067, 0.032, fl], [0.15, -0.007, 0.075, 0.014, fl]]), `ear-${k}`);
    mem(K.limb(`${n}-femur`, `thigh-h${k}`, FEMUR, BONE, { sd }), `thigh-h${k}`);
    const th = K.frame(`thigh-h${k}`).u;
    mem(K.ball(`${n}-patella`, `thigh-h${k}`, K.at(`thigh-h${k}`, 0.19, 0.034, sd * 0.1), [0.014, 0.01, 0.009], BONE, [th[0]!, th[1]!, 0]), `thigh-h${k}`);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, TIBIA, BONE, { sd }), `shank-h${k}`);
    mem(K.run(`${n}-fibula`, `shank-h${k}`, [{ p: K.at(`shank-h${k}`, 0.012, -0.018, sd * 0.113), rl: 0.005, rs: 0.006 }, { p: K.at(`shank-h${k}`, 0.09, -0.016, sd * 0.11), rl: fl, rs: 0.0048 }, { p: K.at(`shank-h${k}`, 0.18, -0.008, sd * 0.109), rl: 0.005, rs: 0.007 }], BONE), `shank-h${k}`);
    const mt = `metatarsus-h${k}`;
    const mc = `metacarpus-f${k}`;
    mem(K.limb(`${n}-tarsus`, mt, TARSUS, BONE, { sd, e: 2.6 }), mt);
    mem(K.run(`${n}-calcaneus`, mt, [{ p: K.at(mt, 0.016, -0.01, sd * 0.096), rl: 0.0095, rs: 0.0105 }, { p: K.at(mt, -0.032, -0.024, sd * 0.096), rl: 0.009, rs: 0.0095 }], BONE), mt);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, SCAPULA, BONE, { sd, e: 2.6, floor: fl }), `scapula-f${k}`);
    mem(K.run(`${n}-scapular-spine`, `scapula-f${k}`, [{ p: K.at(`scapula-f${k}`, 0.03, 0.004, sd * 0.086), rl: 0.008, rs: fl }, { p: K.at(`scapula-f${k}`, 0.1, 0.004, sd * 0.101), rl: 0.009, rs: fl }, { p: K.at(`scapula-f${k}`, 0.155, 0.002, sd * 0.108), rl: 0.005, rs: fl }], BONE), `scapula-f${k}`);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    mem(K.limb(`${n}-radius`, `forearm-f${k}`, RADIUS, BONE, { sd }), `forearm-f${k}`);
    mem(K.run(`${n}-ulna`, `forearm-f${k}`, [{ p: K.at(`forearm-f${k}`, -0.05, -0.028, sd * 0.1), rl: 0.011, rs: 0.013 }, { p: K.at(`forearm-f${k}`, -0.005, -0.017, sd * 0.1), rl: 0.01, rs: 0.011 }, { p: K.at(`forearm-f${k}`, 0.07, -0.012, sd * 0.098), rl: 0.007, rs: 0.0075 }, { p: K.at(`forearm-f${k}`, 0.134, -0.01, sd * 0.096), rl: 0.007, rs: 0.0075 }], BONE), `forearm-f${k}`);
    mem(K.limb(`${n}-carpus`, mc, CARPUS, BONE, { sd, e: 2.6 }), mc);
    for (const [leg, h, z, cannon, table] of [["hind", "h", HIND_Z, mt, METAPODIAL_H], ["fore", "f", FORE_Z, mc, METAPODIAL_F]] as const) {
      const hoof = `hoof-${h}${k}`;
      const f = K.frame(hoof);
      const len = K.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
      const toe: V3 = [f.o[0]! + f.u[0]! * len, f.o[1]! + f.u[1]! * len, 0];
      for (const [j, w] of [[1, -1], [2, 1]] as const) {
        mem(K.limb(`${n}-${leg}-metapodial-${j}`, cannon, table.map((r) => [...r, z + sd * w * 0.0095]), BONE, { sd }), cannon);
        mem(K.limb(`${n}-${leg}-digit-${j}`, hoof, DIGIT.map((r) => [...r, z + sd * w * 0.0125]), BONE, { sd }), hoof);
        mem(K.run(`${n}-${leg}-pedal-${j}`, hoof, [{ p: [toe[0] - 0.038, toe[1] + 0.023, sd * z + w * 0.0135], rl: 0.0075, rs: 0.009 }, { p: [toe[0] - 0.008, toe[1] + 0.0078, sd * z + w * 0.0145], rl: 0.0045, rs: 0.0045 }], BONE), hoof);
      }
    }
  }
  const jr: Record<string, number> = { pelvis: 0.024, lumbar: 0.021, thorax: 0.02, neck: 0.022, head: 0.024, jaw: 0.012, "tail-1": 0.009, "tail-2": 0.007, ear: 0.008, thigh: 0.024, shank: 0.028, metatarsus: 0.021, hoof: 0.019, scapula: 0.009, humerus: 0.026, forearm: 0.024, metacarpus: 0.021 };
  for (const b of K.c.raw.rig.bones as Json[]) {
    const stem = (b.id as string).replace(/-[hf]?[nf]$/, "");
    const key = stem === "hoof" ? `hoof-${(b.id as string).slice(5, 6)}` : stem;
    const sd = /-(h|f)?n$/.test(b.id) ? -1 : /-(h|f)?f$/.test(b.id) ? 1 : 0;
    const r = jr[stem]!;
    const o = K.frame(b.id).o;
    mem(K.ball(`joint-${b.id}`, b.id, [o[0]!, o[1]!, sd * (JZ[key] ?? 0) + (b.id === "tail-2" ? 0.008 : 0)], [r, r, r], JOINT), b.id, "joint");
  }
  mem(K.ball("mass-thorax", "thorax", [0.5, 0.46, 0], [0.04, 0.04, 0.04], MASS), "thorax", "mass");
  mem(K.ball("mass-lumbar", "lumbar", [0.22, 0.49, 0], [0.032, 0.032, 0.032], MASS), "lumbar", "mass");
  return K.body;
}

const DECISIONS = [
  {
    en: "Body at 1:50: 50 solids, every section stated by hand from the skeletal landmarks of a 100 kg finishing pig (back 0.68 m, withers 0.65 m, trunk 0.83 m from pin bone to point of shoulder). The barrel is two overlapping lofts (loin on the lumbar bone, chest on the thorax bone) with a rounded rump, a level slightly arched back, a long straight underline and a deep brisket; the ham hangs from the thigh down to the stifle and stands a little proud of the flank, the shoulder lies on the shoulder blade and the upper arm closes the elbow. The neck is short and as deep as the chest; the head has the high poll, the dished profile, the jowled lower jaw as its own solid, the rooting disc, small high eyes and lop ears that fold forward over them; the tail is a corkscrew of one and a half turns. Each leg is a gaskin or forearm with the muscle at the top, a point of hock, a short cannon with fetlock, two dewclaws behind it, a steep pastern and two claws; segments overlap at every joint so that poses stay closed.",
    de: "Körper bei 1:50: 50 Körper, jeder Schnitt von Hand nach den Skelettpunkten eines 100 kg schweren Mastschweins gesetzt (Rücken 0,68 m, Widerrist 0,65 m, Rumpf 0,83 m vom Sitzbeinhöcker bis zum Buggelenk). Die Tonne besteht aus zwei überlappenden Lofts (Lende am Lendenknochen, Brust am Brustknochen) mit runder Kruppe, ebenem, leicht gewölbtem Rücken, langer gerader Unterlinie und tiefer Vorderbrust; der Schinken hängt am Oberschenkel bis zum Knie und steht etwas über die Flanke vor, die Schulter liegt auf dem Schulterblatt, der Oberarm schließt den Ellbogen. Der Hals ist kurz und so tief wie die Brust; der Kopf hat das hohe Genick, das eingesattelte Profil, den Unterkiefer mit Backe als eigenen Körper, die Rüsselscheibe, kleine hoch sitzende Augen und Schlappohren, die nach vorn über sie fallen; der Schwanz ist ein Ringel von anderthalb Windungen. Jedes Bein besteht aus Unterschenkel oder Unterarm mit dem Muskel oben, Fersenhöcker, kurzer Röhre mit Fesselkopf, zwei Afterklauen dahinter, steiler Fessel und zwei Klauen; die Segmente überlappen an jedem Gelenk, damit Posen geschlossen bleiben.",
  },
  {
    en: "Thermal at 1:50: every region has at most one declared layer (3 mm of sparse bristles), so each region is the body surface it covers (references to the body solids): trunk skin with neck, hams, shoulders and upper arms, leg skin from gaskin and forearm down to the pastern, head skin with jaw and eyes, the moist rooting disc, both ears, the tail, and the horn of claws and dewclaws. The outlets are patches on the surface: two nostrils in the rooting disc, the lips below it, and a patch of the back for the plume.",
    de: "Thermik bei 1:50: jede Region hat höchstens eine deklarierte Schicht (3 mm spärliche Borsten), daher ist jede Region die Körperoberfläche, die sie bedeckt (Verweise auf die Körper): Rumpfhaut mit Hals, Schinken, Schultern und Oberarmen, Beinhaut von Unterschenkel und Unterarm bis zur Fessel, Kopfhaut mit Kiefer und Augen, die feuchte Rüsselscheibe, beide Ohren, der Schwanz und das Horn von Klauen und Afterklauen. Die Auslässe sind Flächen auf der Oberfläche: zwei Nasenlöcher in der Rüsselscheibe, die Lippen darunter und ein Rückenfeld für die Warmluftfahne.",
  },
  {
    en: "Structure at 1:50: the skeleton. Skull with the high nuchal crest and the mandible, cervical spine, thoracic spine with one spinous process per vertebra (the long ones of the second to fifth carry the withers), fourteen ribs per side, the first seven reaching the sternum through their costal cartilages, the others joined in the costal arch, lumbar spine with a spinous and two transverse processes per vertebra, sacrum, ilium, ischium and pubis, caudal vertebrae along the curl, ear cartilages; per foreleg scapula with its spine, humerus, radius and ulna with the olecranon, carpus, the two weight-bearing metacarpals, two digits with pedal bones; per hind leg femur with patella, tibia and fibula, tarsus with calcaneus, the two weight-bearing metatarsals, two digits with pedal bones. The accessory digits of the dewclaws are below the smallest feature and left out. Joints sit at the rig joints at the size of the joint, the masses of thorax and loin at their centres of mass.",
    de: "Tragwerk bei 1:50: das Skelett. Schädel mit hohem Genickkamm und Unterkiefer, Halswirbelsäule, Brustwirbelsäule mit einem Dornfortsatz je Wirbel (die langen des zweiten bis fünften tragen den Widerrist), vierzehn Rippen je Seite, die ersten sieben über ihre Rippenknorpel am Brustbein, die übrigen im Rippenbogen verbunden, Lendenwirbelsäule mit einem Dorn- und zwei Querfortsätzen je Wirbel, Kreuzbein, Darmbein, Sitzbein und Schambein, Schwanzwirbel entlang des Ringels, Ohrknorpel; je Vorderbein Schulterblatt mit Gräte, Oberarmbein, Speiche und Elle mit Ellbogenhöcker, Vorderfußwurzel, die zwei tragenden Mittelhandknochen, zwei Zehen mit Klauenbeinen; je Hinterbein Oberschenkelbein mit Kniescheibe, Schien- und Wadenbein, Fußwurzel mit Fersenbein, die zwei tragenden Mittelfußknochen, zwei Zehen mit Klauenbeinen. Die Nebenzehen der Afterklauen liegen unter der kleinsten Abmessung und entfallen. Gelenke sitzen an den Gelenken des Skeletts in Gelenkgröße, die Massen von Brust und Lende in ihren Schwerpunkten.",
  },
];

if (import.meta.main) {
  const S = new Pig(SPEC, 50);
  body(S);
  const th = thermal(S, new Pig(SPEC, 50), { nostrilR: 0.005, mouthR: 0.014, plume: [[0.42, 0.663, 0], [0.22, 0.13, 0.02]] });
  const st = structure(new Pig(SPEC, 50));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
    writeFileSync(`${OUT}/shapes.50.json`, JSON.stringify(S.shapes));
  }
}
