/** 🦊️ Round 5 red fox at 1:50, remodelled by hand: one coat over the trunk with a deep brisket and a tucked flank, the shoulder and upper arm as one rounded mass that swells only a few millimetres out of the chest, the thigh as a ham that wraps the stifle, a head whose cheek ruffs and white lower jaw are part of the head mass (no flaps), cupped ears with a pale inner face, a white throat bib, a brush that grows out of the croup, and compact digitigrade paws with four knuckle lobes. Structure: the skeleton with a segmented vertebral column (C7, T13, L7, S3, 20 caudal) instead of rods and plates.
 * Usage: bun r5-red-fox-50.ts [--write]
 * @see ./r3-red-fox-50.ts
 * @see ./r3-animals-red-fox.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Fox, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-red-fox-kit.ts";
import { BLACK, BONE, CRANIUM, DARK, DIGITS, FEMUR, FORE_Z, HIND_Z, HUMERUS, METACARPAL, METATARSAL, RADIUS, RED, SCAPULA, SIDES, STERNUM, TIBIA, capped, jointsAndMasses, mem, ribPath } from "./r3-red-fox-50.ts";

type Rows = readonly (readonly number[])[];
export const WHITE: Look = { material: "red-fur", tone: 0.18 };

/** Trunk coat in planes x = const: x, centre height, half height, half width, exponent. Croup 0.436, loin 0.437, withers 0.442; flank tuck 0.27 above ground at x 0.12, brisket 0.212 at x 0.36, prosternum at x 0.47. */
export const TRUNK: Rows = [
  [-0.06, 0.392, 0.012, 0.012, 2],
  [-0.05, 0.389, 0.036, 0.034, 2],
  [-0.03, 0.381, 0.055, 0.054, 2.1],
  [0.0, 0.371, 0.065, 0.064, 2.2],
  [0.04, 0.364, 0.072, 0.065, 2.2],
  [0.09, 0.362, 0.074, 0.06, 2.2],
  [0.14, 0.359, 0.078, 0.059, 2.2],
  [0.19, 0.351, 0.087, 0.064, 2.25],
  [0.235, 0.343, 0.095, 0.07, 2.3],
  [0.28, 0.336, 0.101, 0.074, 2.3],
  [0.33, 0.331, 0.108, 0.074, 2.3],
  [0.37, 0.329, 0.112, 0.069, 2.25],
  [0.405, 0.333, 0.104, 0.06, 2.2],
  [0.435, 0.341, 0.088, 0.049, 2.1],
  [0.458, 0.35, 0.062, 0.034, 2],
  [0.471, 0.356, 0.03, 0.016, 2],
];
/** Neck with ruff along the neck bone: distance, dorsal offset, sagittal half, lateral half. */
export const NECK: Rows = [[-0.06, -0.002, 0.064, 0.047], [-0.02, -0.003, 0.072, 0.057], [0.02, -0.004, 0.066, 0.055], [0.06, -0.004, 0.056, 0.049], [0.1, -0.002, 0.047, 0.044], [0.135, 0.0, 0.041, 0.041], [0.162, 0.004, 0.032, 0.035]];
/** Throat bib along the neck bone (white): distance, dorsal offset (negative = ventral), sagittal half, lateral half. */
export const BIB: Rows = [[-0.07, -0.046, 0.022, 0.03], [-0.035, -0.05, 0.03, 0.04], [0.02, -0.044, 0.03, 0.04], [0.08, -0.033, 0.028, 0.034], [0.13, -0.025, 0.024, 0.03], [0.16, -0.022, 0.016, 0.022]];
/** Upper head (red) along the head bone, occiput to nose: s, v, sagittal half, lateral half, exponent. Braincase 7 cm deep with coat, cheek ruff 11 cm across at s 0.03, stop at s 0.078, muzzle 3.4 cm wide at s 0.1. */
export const SKULL: Rows = [
  [-0.016, 0.014, 0.02, 0.028, 2.2],
  [0.0, 0.014, 0.031, 0.043, 2.4],
  [0.025, 0.013, 0.034, 0.053, 2.5],
  [0.05, 0.013, 0.031, 0.047, 2.5],
  [0.07, 0.01, 0.025, 0.032, 2.4],
  [0.088, 0.006, 0.0195, 0.022, 2.3],
  [0.11, 0.002, 0.0165, 0.018, 2.3],
  [0.132, 0.0, 0.0138, 0.015, 2.3],
  [0.147, -0.001, 0.011, 0.0118, 2.2],
  [0.155, -0.002, 0.007, 0.0078, 2.1],
];
/** Lower jaw, chin and the white lower cheeks (head frame): s, v, sagittal half, lateral half. Its upper edge meets the upper head along the lips (v about -0.011). */
export const JAW: Rows = [
  [-0.004, -0.012, 0.014, 0.034],
  [0.02, -0.018, 0.02, 0.047],
  [0.045, -0.02, 0.019, 0.04],
  [0.07, -0.02, 0.0145, 0.026],
  [0.095, -0.0195, 0.0105, 0.0175],
  [0.12, -0.018, 0.0085, 0.0135],
  [0.137, -0.0155, 0.0066, 0.0098],
  [0.147, -0.0135, 0.0045, 0.0062],
];
/** Ear: fraction of the length, half breadth, half thickness (back of the ear with its coat). */
export const EAR_T: Rows = [[0, 0.02, 0.0095], [0.12, 0.0255, 0.0088], [0.3, 0.0268, 0.0075], [0.55, 0.0215, 0.0062], [0.8, 0.012, 0.0048], [0.95, 0.0055, 0.0043], [1, 0.0043, 0.0043]];
/** Pale inner face of the ear (the hair inside the concha), set forward along the opening normal. */
export const EAR_IN: Rows = [[0.06, 0.0128, 0.0041, 0.0062], [0.3, 0.0178, 0.0041, 0.0049], [0.6, 0.0125, 0.0041, 0.0035], [0.84, 0.0052, 0.0041, 0.0022]];
export const EAR_LEN = 0.092;
/** Brush along the tail bones: distance, offset, sagittal half, lateral half. The root grows out of the croup; widest (12 cm with hair) in the middle link; the last link is the white tip. */
export const BRUSH_ROOT: Rows = [[-0.024, 0.014, 0.015, 0.017], [0.0, 0.011, 0.022, 0.023], [0.03, 0.006, 0.031, 0.031], [0.08, 0.0, 0.043, 0.042], [0.13, -0.003, 0.052, 0.05], [0.165, -0.004, 0.056, 0.054]];
export const BRUSH_MID: Rows = [[-0.012, -0.004, 0.055, 0.053], [0.03, -0.004, 0.06, 0.058], [0.08, -0.003, 0.06, 0.058], [0.125, -0.002, 0.056, 0.054], [0.152, -0.002, 0.05, 0.048]];
export const BRUSH_TIP: Rows = [[-0.01, -0.002, 0.051, 0.049], [0.03, -0.001, 0.047, 0.045], [0.065, 0, 0.038, 0.036], [0.095, 0, 0.026, 0.025], [0.122, 0, 0.0135, 0.013], [0.134, 0, 0.0052, 0.0052]];
/** Limb tables: distance along the bone, cranial offset, sagittal half, lateral half, lateral position. */
export const THIGH: Rows = [[-0.03, -0.014, 0.034, 0.022, 0.034], [0.0, -0.014, 0.055, 0.031, 0.039], [0.04, -0.013, 0.057, 0.034, 0.042], [0.08, -0.008, 0.047, 0.031, 0.043], [0.115, -0.002, 0.034, 0.025, 0.044], [0.142, 0.003, 0.026, 0.02, 0.045]];
export const GASKIN: Rows = [[-0.016, -0.004, 0.021, 0.016, 0.045], [0.01, -0.008, 0.028, 0.019, 0.045], [0.045, -0.008, 0.024, 0.017, 0.043], [0.085, -0.007, 0.018, 0.0135, 0.041], [0.12, -0.007, 0.0148, 0.0118, 0.039], [0.142, -0.005, 0.0138, 0.0115, 0.038]];
export const HIND_CANNON: Rows = [[-0.01, -0.003, 0.012, 0.0105, 0.038], [0.015, -0.001, 0.0115, 0.0105, 0.038], [0.045, 0, 0.0105, 0.0105, 0.037], [0.072, 0, 0.0105, 0.0118, 0.036], [0.092, 0.002, 0.0118, 0.0135, 0.036]];
export const FOREARM: Rows = [[-0.02, -0.01, 0.015, 0.011, 0.039], [0.0, -0.004, 0.02, 0.0145, 0.039], [0.03, -0.001, 0.0175, 0.0135, 0.038], [0.07, 0.0, 0.0135, 0.0118, 0.036], [0.105, 0.0, 0.0122, 0.0115, 0.034], [0.124, 0.0, 0.0125, 0.012, 0.033]];
export const FORE_CANNON: Rows = [[-0.01, 0.0, 0.0118, 0.0112, 0.033], [0.01, -0.002, 0.012, 0.0115, 0.033], [0.035, 0, 0.0112, 0.012, 0.032], [0.055, 0, 0.0112, 0.013, 0.032], [0.07, 0.002, 0.0118, 0.0145, 0.032]];
/** Paw along the paw bone (metapodial joint to toe tip on the ground): distance, dorsal offset, sagittal half, lateral half. Compact oval foot, 5 cm long, 3.7 to 4 cm wide, the knuckles 2.4 cm high. */
export const PAW: Rows = [[-0.01, -0.001, 0.0122, 0.0115], [0.006, -0.004, 0.0158, 0.0145], [0.02, -0.0055, 0.0172, 0.0162], [0.032, -0.0035, 0.0142, 0.0168], [0.041, -0.001, 0.0102, 0.0152], [0.047, 0.001, 0.0062, 0.0122]];
/** Knuckle lobes (digits II to V): lateral position, setback of the lobe centre from the toe tip, half length. */
export const TOES: Rows = [[-0.0118, 0.0145, 0.0078], [-0.004, 0.0108, 0.0085], [0.004, 0.0108, 0.0085], [0.0118, 0.0145, 0.0078]];

/** 🦻️ Ear of side `sd`: erect, leaning a little forward and outward, the cup turned forward and outward; with `inner` the pale inner face set proud of the cup. */
export function ear(S: Fox, id: string, sd: -1 | 1, look: Look, len: number, rows: Rows, shape?: string, inner = 0): Json {
  const b = S.at("head", 0.02, 0.028, sd * 0.031);
  const d: V3 = [0.17, 0.951, sd * 0.259];
  const lat: V3 = [0.64, 0, -sd * 0.77];
  const n: V3 = [0.732 * 1, -0.297, sd * 0.609];
  const pts: PP[] = rows.map(([t, rl, rs, off]) => { const o = inner * (off ?? 0); return { p: [b[0] + d[0] * len * t! + n[0] * o, b[1] + d[1] * len * t! + n[1] * o, b[2] + d[2] * len * t! + n[2] * o] as V3, rl: rl!, rs: rs!, e: 2.3 }; });
  return S.run(id, `ear-${sx(sd)}`, pts, look, { lat, floor: 0.0041, ...(shape === undefined ? {} : { shape }) });
}

/** 🐾️ Paw of one leg: the oval pad mass and four knuckle lobes resting on the ground, the two middle ones leading. */
export function paw(S: Fox, n: string, leg: "hind" | "fore", bone: string, sd: -1 | 1, z: number, k: number, shape: string, rows: Rows = PAW, lobes = true): void {
  const f = S.frame(bone);
  const len = S.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
  const tx = f.o[0]! + f.u[0]! * len;
  S.limb(`${n}-${leg}-paw`, bone, dense(rows.map((r) => [r[0], r[1], r[2], r[3]! * k, z]), 2), DARK, { sd, shape, e: 2.5 });
  if (!lobes) return;
  for (const [j, [w, back, l]] of TOES.entries()) S.ball(`${n}-${leg}-toe-${j + 1}`, bone, [tx - back!, 0.0071, sd * z + w! * k], [l!, 0.0058 * k, 0.0071], DARK, [1, -0.12, 0], shape);
}

/** 🦊️ Body at 1:50. */
export function body(S: Fox): void {
  const T = dense(TRUNK, 2);
  const cut = (lo: number, hi: number): number[][] => T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9).map((r) => [...r]);
  const hind = cut(-0.1, 0.3);
  hind.slice(-4).forEach((r, i) => { r[2]! -= 0.0035 * (i + 1); r[3]! -= 0.0035 * (i + 1); });
  const fore = cut(0.17, 0.5);
  fore.slice(0, 4).forEach((r, i) => { r[2]! -= 0.0035 * (4 - i); r[3]! -= 0.0035 * (4 - i); });
  S.hull("hindbody", "lumbar", hind, RED, { shape: "hindbody" });
  S.hull("chest", "thorax", fore, RED, { shape: "chest" });
  S.limb("neck", "neck", dense(capped(NECK), 2), RED, { shape: "neck", e: 2.2 });
  S.limb("throat-bib", "neck", dense(capped(BIB), 2), WHITE, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(SKULL.map((r) => [r[0], r[1], r[2], r[3], 0, r[4]]), 2), RED, { shape: "head" });
  S.limb("jaw", "jaw", dense(JAW, 2), WHITE, { frame: "head", shape: "jaw", e: 2.3 });
  const hu = S.frame("head").u;
  S.ball("nose", "head", S.at("head", 0.1545, -0.0015, 0), [0.0068, 0.0085, 0.0075], BLACK, [hu[0]!, hu[1]!, 0], "nose");
  S.limb("brush-root", "tail-1", dense(capped(BRUSH_ROOT, false, true), 2), RED, { shape: "brush-root", e: 2.1 });
  S.limb("brush-mid", "tail-2", dense(capped(BRUSH_MID, true, true), 2), RED, { shape: "brush-mid", e: 2.1 });
  S.limb("brush-tip", "tail-3", dense(capped(BRUSH_TIP, true, false), 2), WHITE, { shape: "brush-tip", e: 2.1 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.ball(`${n}-eye`, "head", S.at("head", 0.071, 0.0212, sd * 0.0262), [0.0064, 0.0052, 0.0041], BLACK, [hu[0]! * 0.94, hu[1]! * 0.94 + 0.2, sd * 0.28], sd < 0 ? "eye" : "eye-f");
    ear(S, `${n}-ear`, sd, DARK, EAR_LEN, EAR_T, `ear-${k}`);
    ear(S, `${n}-ear-inner`, sd, WHITE, EAR_LEN, EAR_IN, `ear-${k}`, 1);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(capped(THIGH), 2), RED, { sd, shape: sd < 0 ? "thigh-n" : "hindbody" });
    S.limb(`${n}-gaskin`, `shank-h${k}`, dense(capped(GASKIN), 2), RED, { sd, shape: `gaskin-${k}` });
    const mt = `metatarsus-h${k}`;
    const mu = S.frame(mt).u;
    const mv = S.frame(mt).v;
    S.ball(`${n}-hock-point`, mt, S.at(mt, -0.003, -0.0105, sd * 0.038), [0.017, 0.0058, 0.0072], DARK, [mu[0]! * 0.8 + mv[0]! * 0.6, mu[1]! * 0.8 + mv[1]! * 0.6, 0], `cannon-h-${k}`);
    S.limb(`${n}-hind-cannon`, mt, capped(HIND_CANNON), DARK, { sd, shape: `cannon-h-${k}` });
    paw(S, n, "hind", `paw-h${k}`, sd, HIND_Z, 1, `paw-h-${k}`);
    const sc = `scapula-f${k}`;
    const hm = `humerus-f${k}`;
    S.run(`${n}-shoulder`, hm, [
      { p: S.at(sc, -0.008, -0.004, sd * 0.026), rl: 0.008, rs: 0.014, e: 2.1 },
      { p: S.at(sc, 0.012, -0.003, sd * 0.031), rl: 0.017, rs: 0.026, e: 2.1 },
      { p: S.at(sc, 0.05, -0.002, sd * 0.036), rl: 0.022, rs: 0.033, e: 2.1 },
      { p: S.at(hm, 0.004, 0.002, sd * 0.039), rl: 0.024, rs: 0.035, e: 2.1 },
      { p: S.at(hm, 0.05, -0.006, sd * 0.04), rl: 0.022, rs: 0.032, e: 2.1 },
      { p: S.at(hm, 0.095, -0.008, sd * 0.039), rl: 0.0175, rs: 0.025, e: 2.1 },
      { p: S.at(hm, 0.122, -0.006, sd * 0.038), rl: 0.0135, rs: 0.0175, e: 2.1 },
      { p: S.at(hm, 0.134, -0.005, sd * 0.038), rl: 0.008, rs: 0.01, e: 2.1 },
    ], RED, { shape: "chest" });
    S.limb(`${n}-forearm`, `forearm-f${k}`, dense(capped(FOREARM), 2), DARK, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, capped(FORE_CANNON), DARK, { sd, shape: `cannon-f-${k}` });
    paw(S, n, "fore", `paw-f${k}`, sd, FORE_Z, 1.05, `paw-f-${k}`);
  }
}

export const REGION: readonly (readonly [RegExp, string])[] = [
  [/^(hindbody|chest|neck|throat-bib|trunk|body)$|-(thigh|shoulder)$/, "coat-trunk"], [/-(gaskin|hock-point|cannon|forearm|leg)$/, "coat-legs"], [/^(head|jaw|nose)$|-eye$/, "face"], [/-ear(-inner)?$|^ears?$/, "ears"], [/^brush/, "brush"], [/-paw$|-toe-\d$/, "paws"],
];

/** 🌡️ Thermal list of a scale: every body solid as the fur surface of its region (each region declares the single layer `fur`), and the outlets as own patches on the surface: nostrils on the nose pad, the lips, a strip of the back coat for the plume. */
export function thermal(S: Fox, T: Fox, o: { nostrilR: number; mouthR: number; plume: readonly [V3, V3]; pair?: boolean }): Json[] {
  const out: Json[] = S.body.map((b) => {
    const hit = REGION.find(([re]) => re.test(b.id));
    if (hit === undefined) throw new Error(`no region for ${b.id}`);
    return { id: `th-${b.id}`, of: b.id, region: hit[1], layer: "fur" };
  });
  const hu = T.frame("head").u;
  const hv = T.frame("head").v;
  const E: Look = { material: "dark-fur", tone: 0.95, role: "detail" };
  if (o.pair === false) T.ball("th-breath-nose", "head", T.at("head", 0.159, -0.003, 0), [0.004, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  else for (const sd of SIDES) T.ball(`th-breath-nose-${sx(sd)}`, "head", T.at("head", 0.159, -0.003, sd * 0.0042), [0.004, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  T.ball("th-breath-mouth", "head", T.at("head", 0.141, -0.0128, 0), [0.0045, o.mouthR, 0.0042], E, [hu[0]! - hv[0]! * 0.3, hu[1]! - hv[1]! * 0.3, 0]);
  T.ball("th-plume-back", "thorax", o.plume[0], o.plume[1], { material: "red-fur", tone: 0.5 }, [1, 0, 0]);
  for (const t of T.body) out.push({ ...t, region: t.id === "th-plume-back" ? "coat-trunk" : "face", tags: ["emitter", t.id === "th-plume-back" ? "plume-back" : t.id === "th-breath-mouth" ? "breath-mouth" : "breath-nose"] });
  return out;
}

/** Vertebral axis in the standing pose (world): cervical S from the atlas to C7/T1, thoracic to the thoracolumbar joint, lumbar to the lumbosacral joint, sacrum to the first caudal. */
export const AXIS_C: readonly V3[] = [[0.5, 0.468, 0], [0.478, 0.447, 0], [0.457, 0.416, 0], [0.44, 0.388, 0], [0.425, 0.371, 0], [0.41, 0.362, 0]];
export const AXIS_T: readonly V3[] = [[0.41, 0.362, 0], [0.36, 0.379, 0], [0.3, 0.394, 0], [0.225, 0.404, 0]];
export const AXIS_L: readonly V3[] = [[0.225, 0.404, 0], [0.14, 0.409, 0], [0.05, 0.4, 0]];
/** 📏️ Point and tangent at fraction `t` of a polyline. */
export function along(P: readonly V3[], t: number): [V3, V3] {
  const seg: number[] = [];
  for (let i = 1; i < P.length; i++) seg.push(Math.hypot(P[i]![0] - P[i - 1]![0], P[i]![1] - P[i - 1]![1]));
  let d = t * seg.reduce((a, b) => a + b, 0);
  for (let i = 0; i < seg.length; i++) {
    if (d <= seg[i]! || i === seg.length - 1) {
      const a = P[i]!; const b = P[i + 1]!; const f = Math.min(1, d / seg[i]!);
      const tg: V3 = [(b[0] - a[0]) / seg[i]!, (b[1] - a[1]) / seg[i]!, 0];
      return [[a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f, 0], tg];
    }
    d -= seg[i]!;
  }
  return [P[P.length - 1]!, [1, 0, 0]];
}

/** 🦴️ One vertebra: the body as a short spool along the axis, the spinous process as a dorsal blade (height `sp`, leaning `lean` caudally), the transverse processes as a lateral bar (half span `tr`). */
function vertebra(K: Fox, id: string, bone: string, P: readonly V3[], t0: number, t1: number, r: number, sp: number, lean: number, tr: number): void {
  const [a, ta] = along(P, t0);
  const [b] = along(P, t1);
  const gap = 0.0006;
  const d = Math.hypot(b[0] - a[0], b[1] - a[1]);
  const ux = (b[0] - a[0]) / d; const uy = (b[1] - a[1]) / d;
  const A: V3 = [a[0] + ux * gap, a[1] + uy * gap, 0]; const B: V3 = [b[0] - ux * gap, b[1] - uy * gap, 0];
  const M: V3 = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, 0];
  mem(K.run(`${id}-body`, bone, [{ p: A, rl: r, rs: r * 0.9 }, { p: M, rl: r * 0.82, rs: r * 0.78 }, { p: B, rl: r, rs: r * 0.9 }], BONE, { floor: 0.0037 }), bone);
  const nx = -ta[1]; const ny = ta[0];
  if (sp > 0) mem(K.run(`${id}-spine`, bone, [{ p: [M[0] + nx * r * 0.6, M[1] + ny * r * 0.6, 0], rl: 0.0038, rs: 0.0055 }, { p: [M[0] + nx * (r + sp) - ux * lean, M[1] + ny * (r + sp) - uy * lean, 0], rl: 0.0038, rs: 0.0042 }], BONE, { floor: 0.0038 }), bone);
  if (tr > 0) mem(K.run(`${id}-transverse`, bone, [{ p: [M[0], M[1], -tr], rl: 0.0038, rs: 0.0042 }, { p: [M[0], M[1], 0], rl: 0.0038, rs: 0.0055 }, { p: [M[0], M[1], tr], rl: 0.0038, rs: 0.0042 }], BONE, { lat: [1, 0, 0], floor: 0.0038 }), bone);
}

/** 🦴️ Structure at 1:50: the skeleton inside the coat, the column vertebra by vertebra. */
export function structure(K: Fox): Json[] {
  const fl = 0.0042;
  for (let i = 0; i < 7; i++) vertebra(K, `c${i + 1}`, "neck", AXIS_C, i / 7, (i + 1) / 7, i === 0 ? 0.0095 : 0.0078, i === 1 ? 0.007 : i >= 5 ? 0.008 + (i - 5) * 0.006 : 0.0058, 0.002, i === 0 ? 0.017 : 0.011);
  const tsp = [0.026, 0.028, 0.027, 0.025, 0.022, 0.019, 0.016, 0.013, 0.011, 0.01, 0.009, 0.0085, 0.0085];
  for (let i = 0; i < 13; i++) vertebra(K, `t${i + 1}`, "thorax", AXIS_T, i / 13, (i + 1) / 13, 0.0062, tsp[i]!, 0.004 + 0.0003 * (12 - i), 0.0085);
  const ls = [0.0105, 0.011, 0.0115, 0.012, 0.0115, 0.0105, 0.009];
  for (let i = 0; i < 7; i++) vertebra(K, `l${i + 1}`, "lumbar", AXIS_L, i / 7, (i + 1) / 7, 0.0072, ls[i]!, -0.002, 0.02 + (i < 5 ? i * 0.0015 : (6 - i) * 0.003));
  mem(K.run("sacrum", "pelvis", [{ p: [0.05, 0.4, 0], rl: 0.016, rs: 0.0075 }, { p: [0.025, 0.407, 0], rl: 0.012, rs: 0.0065 }, { p: [-0.006, 0.415, 0], rl: 0.0075, rs: 0.0055 }], BONE), "pelvis");
  mem(K.run("sacral-crest", "pelvis", [{ p: [0.044, 0.41, 0], rl: 0.0038, rs: 0.0045 }, { p: [0.0, 0.422, 0], rl: 0.0038, rs: 0.0042 }], BONE, { floor: 0.0038 }), "pelvis");
  const tails: readonly (readonly [string, number])[] = [["tail-1", 7], ["tail-2", 7], ["tail-3", 6]];
  let c = 0;
  for (const [b, n] of tails) {
    const len = K.c.raw.rig.bones.find((x: Json) => x.id === b).lengthM as number;
    const f = K.frame(b);
    for (let i = 0; i < n; i++) {
      const r = Math.max(0.0039, 0.0052 - c * 0.00008);
      const s0 = (i / n) * len + 0.0007; const s1 = ((i + 1) / n) * len - 0.0007;
      const P0: V3 = [f.o[0]! + f.u[0]! * s0, f.o[1]! + f.u[1]! * s0, 0]; const P1: V3 = [f.o[0]! + f.u[0]! * s1, f.o[1]! + f.u[1]! * s1, 0];
      const M: V3 = [(P0[0] + P1[0]) / 2, (P0[1] + P1[1]) / 2, 0];
      mem(K.run(`caudal-${++c}`, b, [{ p: P0, rl: r, rs: r }, { p: M, rl: r * 0.97, rs: r * 0.97 }, { p: P1, rl: r, rs: r }], BONE, { floor: 0.0038 }), b);
    }
  }
  mem(K.run("pubis", "pelvis", [{ p: [0.004, 0.353, -0.03], rl: 0.005, rs: 0.0045 }, { p: [0.004, 0.347, 0], rl: 0.006, rs: 0.0045 }, { p: [0.004, 0.353, 0.03], rl: 0.005, rs: 0.0045 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM, BONE), "thorax");
  for (const sd of SIDES) {
    const n = side(sd);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.076, 0.412, sd * 0.034], rl: 0.0045, rs: 0.017 }, { p: [0.04, 0.401, sd * 0.03], rl: 0.0045, rs: 0.012 }, { p: [0.0, 0.376, sd * 0.03], rl: 0.007, rs: 0.011 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.002, 0.374, sd * 0.03], rl: 0.0055, rs: 0.009 }, { p: [-0.03, 0.368, sd * 0.03], rl: 0.005, rs: 0.0075 }, { p: [-0.05, 0.372, sd * 0.034], rl: 0.0065, rs: 0.011 }], BONE), "pelvis");
    for (let i = 0; i < 13; i++) mem(Object.assign(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, i === 0 ? 0.0018 : 0.0025), BONE, { floor: i === 0 ? 0.0018 : 0.0025 }), { slender: true }), "thorax");
    mem(Object.assign(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.3, 0.236, sd * 0.01], rl: 0.0025, rs: 0.0025 }, ...[9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, 0.0025)[4]!.p, rl: 0.0025, rs: 0.0025 }))], BONE, { floor: 0.0025 }), { slender: true }), "thorax");
  }
  headAndLimbs(K, fl);
  jointsAndMasses(K);
  return K.body;
}

/** 🦴️ Skull, mandible, zygomatic arches, ear cartilages and the limb skeleton at 1:50. */
function headAndLimbs(K: Fox, fl: number): void {
  mem(K.limb("skull", "head", dense(CRANIUM, 2), BONE, { e: 2.3 }), "head");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-zygomatic-arch`, "head", [{ p: K.at("head", 0.032, 0.006, sd * 0.027), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.052, 0.001, sd * 0.0365), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.074, 0.0, sd * 0.032), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.088, 0.002, sd * 0.02), rl: fl, rs: 0.0048 }], BONE), "head");
    mem(K.run(`${n}-mandible`, "jaw", [{ p: K.at("head", 0.03, -0.002, sd * 0.029), rl: fl, rs: 0.0095 }, { p: K.at("head", 0.042, -0.019, sd * 0.027), rl: fl, rs: 0.0085 }, { p: K.at("head", 0.072, -0.021, sd * 0.02), rl: fl, rs: 0.007 }, { p: K.at("head", 0.106, -0.019, sd * 0.012), rl: fl, rs: 0.0055 }, { p: K.at("head", 0.136, -0.017, sd * 0.0045), rl: fl, rs: 0.0045 }], BONE), "jaw");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.086, [[0.02, 0.014, fl], [0.3, 0.021, fl], [0.7, 0.012, fl], [1, 0.0045, fl]]), `ear-${k}`);
    mem(K.limb(`${n}-femur`, `thigh-h${k}`, FEMUR, BONE, { sd }), `thigh-h${k}`);
    const tu = K.frame(`thigh-h${k}`).u;
    mem(K.ball(`${n}-patella`, `thigh-h${k}`, K.at(`thigh-h${k}`, 0.128, 0.0125, sd * 0.045), [0.007, 0.0045, 0.0045], BONE, [tu[0]!, tu[1]!, 0]), `thigh-h${k}`);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, TIBIA, BONE, { sd }), `shank-h${k}`);
    const mt = `metatarsus-h${k}`;
    const mu = K.frame(mt).u;
    const mv = K.frame(mt).v;
    mem(K.ball(`${n}-calcaneus`, mt, K.at(mt, -0.005, -0.0095, sd * 0.038), [0.016, 0.0045, 0.0055], BONE, [mu[0]! * 0.8 + mv[0]! * 0.6, mu[1]! * 0.8 + mv[1]! * 0.6, 0]), mt);
    mem(K.limb(`${n}-metatarsals`, mt, METATARSAL, BONE, { sd, e: 2.6 }), mt);
    mem(K.limb(`${n}-hind-digits`, `paw-h${k}`, DIGITS.map((r) => [...r, HIND_Z]), BONE, { sd, e: 2.6 }), `paw-h${k}`);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, SCAPULA, BONE, { sd, e: 2.6, floor: fl }), `scapula-f${k}`);
    mem(K.run(`${n}-scapular-spine`, `scapula-f${k}`, [{ p: K.at(`scapula-f${k}`, 0.004, 0.0, sd * 0.036), rl: 0.0045, rs: 0.0038 }, { p: K.at(`scapula-f${k}`, 0.05, 0.0, sd * 0.043), rl: 0.0055, rs: 0.0038 }, { p: K.at(`scapula-f${k}`, 0.086, 0.002, sd * 0.047), rl: 0.0045, rs: 0.0038 }], BONE, { floor: 0.0038 }), `scapula-f${k}`);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    const fa = `forearm-f${k}`;
    mem(K.limb(`${n}-radius-ulna`, fa, RADIUS, BONE, { sd }), fa);
    mem(K.run(`${n}-olecranon`, fa, [{ p: K.at(fa, 0.008, -0.007, sd * 0.04), rl: 0.0048, rs: 0.0055 }, { p: K.at(fa, -0.021, -0.012, sd * 0.04), rl: 0.0045, rs: 0.005 }], BONE), fa);
    mem(K.limb(`${n}-metacarpals`, `metacarpus-f${k}`, METACARPAL, BONE, { sd, e: 2.6 }), `metacarpus-f${k}`);
    mem(K.limb(`${n}-fore-digits`, `paw-f${k}`, DIGITS.map((r) => [r[0]! * 0.92, r[1], r[2], r[3]! * 1.06, FORE_Z]), BONE, { sd, e: 2.6 }), `paw-f${k}`);
  }
}

const DECISIONS = [
  {
    en: "Body at 1:50: every section stated by hand from the skeletal landmarks of a 6 kg red fox in autumn coat (withers 0.44 m with coat, hip joint 0.375 m, elbow 0.217 m, hock 0.123 m). The trunk is two overlapping lofts with identical sections where they meet (loin on the lumbar bone, chest on the thorax bone): level back, deep brisket at elbow height, tucked flank. Shoulder blade and upper arm are one rounded mass per side that rises a few millimetres out of the chest, the thigh is a ham that wraps the stifle. The neck carries its ruff and a white throat bib; the head is one red mass with the cheek ruffs and a shallow stop running into a long narrow muzzle, the white lower jaw and lower cheeks meet it along the lips, nose pad and eyes sit in it. The ears are cupped, black-backed, with a pale inner face set proud of the cup. The brush grows out of the croup in three links that overlap with identical sections, the last one the white tip. Each leg has a gaskin or forearm with its muscle belly at the top, a point of hock, a slim cannon and a compact oval paw with four knuckle lobes on the ground.",
    de: "Körper bei 1:50: jeder Schnitt von Hand nach den Skelettpunkten eines 6 kg schweren Rotfuchses im Herbstfell gesetzt (Widerrist 0,44 m mit Fell, Hüftgelenk 0,375 m, Ellbogen 0,217 m, Sprunggelenk 0,123 m). Der Rumpf besteht aus zwei überlappenden Lofts mit gleichen Schnitten am Übergang (Lende am Lendenknochen, Brust am Brustknochen): gerader Rücken, tiefe Brust auf Ellbogenhöhe, aufgezogene Flanke. Schulterblatt und Oberarm sind je Seite eine gerundete Masse, die wenige Millimeter aus der Brust tritt, die Keule umschließt das Knie. Der Hals trägt seine Krause und einen weißen Kehllatz; der Kopf ist eine rote Masse mit Backenbart und flachem Stop, die in einen langen schmalen Fang läuft, Unterkiefer und untere Backen sind weiß und treffen ihn an den Lefzen, Nasenspiegel und Augen sitzen darin. Die Ohren sind gewölbt, hinten schwarz, mit heller Innenseite, die aus der Muschel tritt. Die Rute wächst aus der Kruppe in drei Gliedern mit gleichen Schnitten an den Übergängen, das letzte ist die weiße Spitze. Jedes Bein hat Unterschenkel oder Unterarm mit dem Muskelbauch oben, Fersenhöcker, schlanke Röhre und eine kompakte ovale Pfote mit vier Zehenballen am Boden.",
  },
  {
    en: "Thermal at 1:50: every region declares the single layer of its fur, so each region is the coat surface it covers (references to the body solids, layer fur): trunk coat with neck, throat bib, thighs and shoulders; leg coat of gaskins, hocks, forearms and cannons; face with jaw, nose pad and eyes; ears with their inner faces; brush; paws with knuckle lobes. The outlets lie on the surface: two nostrils on the nose pad, the lips below it, and a strip of the back coat above the thoracolumbar joint for the plume.",
    de: "Thermik bei 1:50: jede Region deklariert die eine Schicht ihres Fells, daher ist jede Region die Felloberfläche, die sie bedeckt (Verweise auf die Körper, Schicht fur): Rumpffell mit Hals, Kehllatz, Keulen und Schultern; Beinfell von Unterschenkeln, Fersen, Unterarmen und Röhren; Gesicht mit Unterkiefer, Nasenspiegel und Augen; Ohren mit ihren Innenseiten; Rute; Pfoten mit Zehenballen. Die Auslässe liegen auf der Oberfläche: zwei Nasenlöcher im Nasenspiegel, die Lippen darunter und ein Streifen des Rückenfells über dem Brust-Lenden-Übergang für die Warmluftfahne.",
  },
  {
    en: "Structure at 1:50: the skeleton inside the coat, the vertebral column vertebra by vertebra: seven cervical vertebrae in the S curve with the wings of the atlas, thirteen thoracic vertebrae whose spinous processes form the withers and lean back, seven lumbar vertebrae with long transverse processes, the sacrum with its crest and twenty caudal vertebrae along the brush. Skull with zygomatic arches and the two halves of the mandible, ear cartilages, thirteen ribs per side (nine sternal, four ending in the costal arch) at their real 5 mm as slender solids, sternum, ilium, ischium and pubis; per foreleg scapula with its spine, humerus, radius and ulna with olecranon, metacarpals and digits; per hind leg femur with patella, tibia, calcaneus, metatarsals and digits. Joints sit at the rig joints at the size of the real joint, the masses of thorax and loin at their centres of mass. The asset declares no supports, so there are no support solids.",
    de: "Tragwerk bei 1:50: das Skelett im Fell, die Wirbelsäule Wirbel für Wirbel: sieben Halswirbel in der S-Kurve mit den Flügeln des Atlas, dreizehn Brustwirbel, deren nach hinten geneigte Dornfortsätze den Widerrist bilden, sieben Lendenwirbel mit langen Querfortsätzen, das Kreuzbein mit seinem Kamm und zwanzig Schwanzwirbel entlang der Rute. Schädel mit Jochbögen und den beiden Unterkieferhälften, Ohrknorpel, dreizehn Rippen je Seite (neun echte, vier im Rippenbogen endend) in ihren wirklichen 5 mm als schlanke Körper, Brustbein, Darmbein, Sitzbein und Schambein; je Vorderbein Schulterblatt mit Gräte, Oberarmbein, Speiche und Elle mit Ellbogenhöcker, Mittelhandknochen und Zehen; je Hinterbein Oberschenkelbein mit Kniescheibe, Schienbein, Fersenbein, Mittelfußknochen und Zehen. Gelenke sitzen an den Gelenken des Skeletts in wirklicher Gelenkgröße, die Massen von Brust und Lende in ihren Schwerpunkten. Das Objekt deklariert keine Auflager, daher gibt es keine Auflagerkörper.",
  },
];

if (import.meta.main) {
  const S = new Fox(SPEC, 50);
  body(S);
  const th = thermal(S, new Fox(SPEC, 50), { nostrilR: 0.0042, mouthR: 0.008, plume: [[0.235, 0.4345, 0], [0.1, 0.03, 0.0075]] });
  const st = structure(new Fox(SPEC, 50));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
    writeFileSync(`${OUT}/shapes.50.json`, JSON.stringify(S.shapes));
  }
}
