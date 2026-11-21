/** 🦊️ Round 3 red fox at 1:50: the finest model. Body: coat trunk in two overlapping lofts, thigh, shoulder and upper arm masses, neck with ruff, skull with stop and long muzzle, mandible, cheek ruffs, nose pad, eyes, cupped leaf ears, brush in three bushy links, per leg gaskin or forearm, point of hock, cannon, paw with four toes. Thermal: the coat surfaces per region and the outlets. Structure: the skeleton.
 * Usage: bun r3-red-fox-50.ts [--write]
 * @see ./r3-red-fox-kit.ts
 * @see ./r3-animals-red-fox.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Fox, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-red-fox-kit.ts";

export const RED: Look = { material: "red-fur", tone: 0.7 };
export const PALE: Look = { material: "red-fur", tone: 0.3 };
export const DARK: Look = { material: "dark-fur", tone: 0.85 };
export const BLACK: Look = { material: "dark-fur", tone: 1, role: "detail" };
export const SIDES = [-1, 1] as const;

/** Trunk coat in planes x = const: x, centre height, half height, half width, exponent. Croup 0.436, loin 0.438, withers 0.441; groin 0.304, flank tuck 0.262, brisket 0.209. */
export const TRUNK = [
  [-0.056, 0.39, 0.01, 0.01, 2],
  [-0.047, 0.388, 0.036, 0.032, 2],
  [-0.028, 0.381, 0.054, 0.052, 2.1],
  [0.0, 0.372, 0.064, 0.064, 2.2],
  [0.04, 0.366, 0.072, 0.066, 2.2],
  [0.09, 0.362, 0.076, 0.061, 2.2],
  [0.14, 0.357, 0.081, 0.059, 2.2],
  [0.19, 0.35, 0.088, 0.064, 2.25],
  [0.235, 0.343, 0.094, 0.07, 2.3],
  [0.28, 0.337, 0.1, 0.074, 2.3],
  [0.33, 0.333, 0.105, 0.074, 2.3],
  [0.375, 0.332, 0.108, 0.069, 2.25],
  [0.41, 0.335, 0.101, 0.06, 2.2],
  [0.44, 0.343, 0.083, 0.047, 2.1],
  [0.462, 0.352, 0.055, 0.03, 2],
  [0.472, 0.357, 0.027, 0.014, 2],
] as const;
/** Neck with ruff along the neck bone: distance, dorsal offset, sagittal half, lateral half. */
export const NECK = [
  [-0.055, -0.004, 0.06, 0.044],
  [-0.02, -0.004, 0.071, 0.056],
  [0.02, -0.004, 0.064, 0.053],
  [0.06, -0.004, 0.052, 0.046],
  [0.1, -0.003, 0.044, 0.041],
  [0.135, -0.002, 0.039, 0.039],
  [0.16, 0.002, 0.03, 0.033],
] as const;
/** Skull with coat along the head bone (occiput to nose): braincase, zygomatic width, stop, long muzzle above the mouth line (v = -0.012). */
export const SKULL = [
  [-0.012, 0.016, 0.017, 0.022, 2.4],
  [0.0, 0.017, 0.027, 0.037, 2.6],
  [0.02, 0.018, 0.031, 0.046, 2.8],
  [0.045, 0.017, 0.031, 0.048, 2.8],
  [0.065, 0.014, 0.027, 0.04, 2.6],
  [0.082, 0.009, 0.0215, 0.028, 2.4],
  [0.1, 0.005, 0.0175, 0.0195, 2.3],
  [0.125, 0.002, 0.015, 0.0165, 2.3],
  [0.145, 0.0, 0.0125, 0.014, 2.3],
  [0.154, -0.001, 0.0085, 0.0095, 2.2],
] as const;
export const MANDIBLE = [
  [0.012, -0.013, 0.011, 0.028],
  [0.035, -0.022, 0.0155, 0.033],
  [0.06, -0.0235, 0.0125, 0.027],
  [0.085, -0.0215, 0.0095, 0.019],
  [0.11, -0.0195, 0.0078, 0.014],
  [0.13, -0.018, 0.0062, 0.011],
  [0.139, -0.017, 0.0045, 0.0065],
] as const;
/** Cheek ruff of one side: the lateral fur wedge that makes the fox face a broad triangle from the front. */
export const CHEEK = [[-0.02, 0.0, 0.009, 0.0045, 0.047], [-0.006, 0.002, 0.017, 0.0075, 0.048], [0.018, 0.004, 0.021, 0.0105, 0.045], [0.045, 0.004, 0.019, 0.01, 0.039], [0.068, 0.003, 0.013, 0.0075, 0.03], [0.086, 0.002, 0.008, 0.0055, 0.021]] as const;
/** Ear leaf: fraction of the length, half breadth, half thickness. */
export const EAR_T = [[0, 0.019, 0.0065], [0.12, 0.0245, 0.005], [0.3, 0.0265, 0.0043], [0.55, 0.0215, 0.0043], [0.8, 0.012, 0.0043], [0.95, 0.0052, 0.0043], [1, 0.0043, 0.0043]] as const;
export const EAR_LEN = 0.092;
/** Brush links along the tail bones: distance, offset, sagittal half, lateral half. */
export const BRUSH_ROOT = [[-0.008, -0.012, 0.02, 0.02], [0.012, -0.011, 0.027, 0.027], [0.04, -0.008, 0.035, 0.034], [0.09, -0.005, 0.047, 0.045], [0.15, -0.004, 0.055, 0.053], [0.172, -0.004, 0.047, 0.045]] as const;
export const BRUSH_MID = [[0.0, -0.004, 0.055, 0.053], [0.05, -0.004, 0.059, 0.057], [0.1, -0.003, 0.058, 0.056], [0.14, -0.002, 0.054, 0.052], [0.162, -0.002, 0.045, 0.043]] as const;
export const BRUSH_TIP = [[0.0, -0.002, 0.054, 0.052], [0.04, -0.001, 0.05, 0.048], [0.08, 0, 0.04, 0.038], [0.11, 0, 0.028, 0.027], [0.135, 0, 0.0145, 0.014], [0.146, 0, 0.005, 0.005]] as const;
/** Limb tables: distance along the bone, cranial offset, sagittal half, lateral half, lateral position. */
export const THIGH = [[-0.028, -0.012, 0.034, 0.02, 0.036], [0.0, -0.012, 0.054, 0.031, 0.04], [0.04, -0.012, 0.054, 0.034, 0.042], [0.08, -0.008, 0.043, 0.03, 0.043], [0.115, -0.002, 0.03, 0.022, 0.044], [0.142, 0.004, 0.02, 0.016, 0.044]] as const;
export const GASKIN = [[-0.022, -0.004, 0.026, 0.018, 0.046], [0.01, -0.008, 0.029, 0.02, 0.045], [0.045, -0.008, 0.024, 0.017, 0.043], [0.085, -0.007, 0.018, 0.013, 0.041], [0.12, -0.007, 0.015, 0.0115, 0.039], [0.144, -0.005, 0.0145, 0.012, 0.038]] as const;
export const HIND_CANNON = [[-0.012, -0.002, 0.0115, 0.0095, 0.038], [0.01, -0.001, 0.012, 0.0105, 0.038], [0.04, 0, 0.0105, 0.0105, 0.037], [0.07, 0, 0.0105, 0.012, 0.036], [0.093, 0.002, 0.012, 0.0145, 0.036]] as const;
export const SHOULDER = [[-0.004, -0.004, 0.019, 0.012, 0.032], [0.02, -0.004, 0.032, 0.021, 0.041], [0.06, -0.002, 0.036, 0.024, 0.047], [0.095, 0.0, 0.031, 0.023, 0.047], [0.116, 0.002, 0.022, 0.018, 0.045]] as const;
export const UPPER_ARM = [[-0.012, 0.0, 0.02, 0.016, 0.046], [0.02, -0.006, 0.031, 0.021, 0.046], [0.06, -0.01, 0.034, 0.021, 0.044], [0.1, -0.008, 0.027, 0.017, 0.042], [0.126, -0.004, 0.019, 0.0145, 0.04]] as const;
export const FOREARM = [[-0.022, -0.011, 0.014, 0.0095, 0.04], [0.0, -0.004, 0.021, 0.0145, 0.04], [0.03, -0.001, 0.0175, 0.0135, 0.038], [0.07, 0.0, 0.0135, 0.0115, 0.036], [0.105, 0.0, 0.012, 0.0115, 0.034], [0.124, 0.0, 0.0125, 0.012, 0.033]] as const;
export const FORE_CANNON = [[-0.012, 0.0, 0.011, 0.0105, 0.033], [0.0, -0.002, 0.0125, 0.0115, 0.033], [0.025, 0, 0.0115, 0.012, 0.032], [0.05, 0, 0.0115, 0.0135, 0.032], [0.071, 0.002, 0.012, 0.0155, 0.032]] as const;
/** Paw along the paw bone (metapodial joint to toe tip on the ground): distance, dorsal offset, sagittal half, lateral half; the pad mass hangs below the bone. */
export const PAW = [[-0.008, -0.002, 0.012, 0.0115], [0.006, -0.005, 0.0165, 0.015], [0.018, -0.0065, 0.019, 0.018], [0.03, -0.0035, 0.0145, 0.0185], [0.04, 0.0, 0.0095, 0.017], [0.047, 0.002, 0.006, 0.0135]] as const;
/** Toes: lateral position within the paw, setback of the toe tip, length. */
export const TOES = [[-0.0135, 0.0075, 0.022], [-0.0046, 0, 0.026], [0.0046, 0, 0.026], [0.0135, 0.0075, 0.022]] as const;
export const HIND_Z = 0.036;
export const FORE_Z = 0.032;

/** 🥚️ Rounds the open ends of a limb table: a smaller closing row before the first and after the last hand row, so that a flat cap never shows at a bent joint.
 * @example capped([[0, 0, 0.02, 0.02], [0.1, 0, 0.02, 0.02]]).length // 4 */
export function capped(rows: readonly (readonly number[])[], start = true, end = true): number[][] {
  const out = rows.map((r) => [...r]);
  const close = (r: readonly number[], dir: -1 | 1): number[] => { const c = [...r]; c[0] = r[0]! + dir * 0.55 * Math.min(r[2]!, r[3]!); c[2] = r[2]! * 0.6; c[3] = r[3]! * 0.6; return c; };
  if (start) out.unshift(close(rows[0]!, -1));
  if (end) out.push(close(rows[rows.length - 1]!, 1));
  return out;
}

/** 🦻️ Ear leaf of side `sd`: erect, leaning a little forward and outward, the opening turned forward and outward. */
export function ear(S: Fox, id: string, sd: -1 | 1, look: Look, len = EAR_LEN, rows: readonly (readonly number[])[] = EAR_T, shape?: string, z = 0.031): Json {
  const b = S.at("head", 0.02, 0.028, sd * z);
  const d: V3 = [0.17, 0.951, sd * 0.259];
  const pts: PP[] = rows.map(([t, rl, rs]) => ({ p: [b[0] + d[0] * len * t!, b[1] + d[1] * len * t!, b[2] + d[2] * len * t!] as V3, rl: rl!, rs: rs!, e: 2.3 }));
  return S.run(id, `ear-${sx(sd)}`, pts, look, { lat: [0.64, 0, -sd * 0.77], floor: 0.0043, ...(shape === undefined ? {} : { shape }) });
}

/** 🐾️ Paw of one leg: the pad mass in planes x = const behind the toe tip and four toes with the two middle ones leading. */
export function paw(S: Fox, n: string, leg: "hind" | "fore", bone: string, sd: -1 | 1, z: number, k: number, shape: string, toes = true): void {
  const f = S.frame(bone);
  const len = S.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
  const toe = f.o[0]! + f.u[0]! * len;
  const ground = f.o[1]! + f.u[1]! * len;
  S.limb(`${n}-${leg}-paw`, bone, dense(PAW.map((r) => [r[0], r[1], r[2], r[3] * k, z]), 2), DARK, { sd, shape, e: 2.5 });
  if (!toes) return;
  for (const [j, [w, back, l]] of TOES.entries()) {
    S.run(`${n}-${leg}-toe-${j + 1}`, bone, [
      { p: [toe - back! - l!, ground + 0.0095, sd * z + w! * k], rl: 0.0052 * k, rs: 0.0085, e: 2.4 },
      { p: [toe - back! - l! * 0.45, ground + 0.008, sd * z + w! * k * 1.04], rl: 0.0054 * k, rs: 0.008, e: 2.4 },
      { p: [toe - back! - 0.004, ground + 0.006, sd * z + w! * k * 1.06], rl: 0.0048 * k, rs: 0.006, e: 2.4 },
      { p: [toe - back! + 0.002, ground + 0.0045, sd * z + w! * k * 1.06], rl: 0.0042, rs: 0.0043, e: 2.4 },
    ], DARK, { shape });
  }
}

export function body(S: Fox): void {
  const T = dense(TRUNK, 2);
  const cut = (lo: number, hi: number, tuck: readonly number[], at: "start" | "end"): number[][] => {
    const rows = T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9).map((r) => [...r]);
    tuck.forEach((d, i) => { const row = rows[at === "end" ? rows.length - 1 - i : i]!; row[2]! -= d; row[3]! -= d; });
    return rows;
  };
  S.hull("hindbody", "lumbar", cut(-0.1, 0.258, [0.012, 0.006], "end"), RED, { shape: "hindbody" });
  S.hull("chest", "thorax", cut(0.2125, 0.5, [], "start"), RED, { shape: "chest" });
  S.limb("neck", "neck", dense(capped(NECK), 2), RED, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(SKULL.map((r) => [r[0], r[1], r[2], r[3], 0, r[4]]), 2), RED, { shape: "head" });
  S.limb("jaw", "jaw", dense(MANDIBLE, 2), PALE, { frame: "head", shape: "jaw", e: 2.3 });
  const hu = S.frame("head").u;
  S.ball("nose", "head", S.at("head", 0.1535, -0.0015, 0), [0.0068, 0.0082, 0.0072], BLACK, [hu[0]!, hu[1]!, 0], "nose");
  S.limb("brush-root", "tail-1", dense(capped(BRUSH_ROOT, false, true), 2), RED, { shape: "brush-root", e: 2.1 });
  S.limb("brush-mid", "tail-2", dense(capped(BRUSH_MID, true, true), 2), RED, { shape: "brush-mid", e: 2.1 });
  S.limb("brush-tip", "tail-3", dense(capped(BRUSH_TIP, true, false), 2), PALE, { shape: "brush-tip", e: 2.1 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.limb(`${n}-cheek`, "head", dense(CHEEK, 2), RED, { sd, shape: "head", e: 2.1 });
    S.ball(`${n}-eye`, "head", S.at("head", 0.071, 0.0235, sd * 0.03), [0.0045, 0.0068, 0.0052], BLACK, [0, 0, sd], sd < 0 ? "eye" : "eye-f");
    ear(S, `${n}-ear`, sd, DARK, EAR_LEN, EAR_T, `ear-${k}`);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(capped(THIGH), 2), RED, { sd, shape: sd < 0 ? "thigh-n" : "hindbody" });
    S.limb(`${n}-gaskin`, `shank-h${k}`, dense(capped(GASKIN), 2), DARK, { sd, shape: `gaskin-${k}` });
    const mt = `metatarsus-h${k}`;
    const mu = S.frame(mt).u;
    const mv = S.frame(mt).v;
    S.ball(`${n}-hock-point`, mt, S.at(mt, -0.004, -0.011, sd * 0.038), [0.02, 0.0062, 0.0078], DARK, [mu[0]! * 0.8 + mv[0]! * 0.6, mu[1]! * 0.8 + mv[1]! * 0.6, 0], `cannon-h-${k}`);
    S.limb(`${n}-hind-cannon`, mt, capped(HIND_CANNON), DARK, { sd, shape: `cannon-h-${k}` });
    paw(S, n, "hind", `paw-h${k}`, sd, HIND_Z, 1, `paw-h-${k}`);
    S.limb(`${n}-shoulder`, `scapula-f${k}`, dense(capped(SHOULDER), 2), RED, { sd, shape: "chest" });
    S.limb(`${n}-upper-arm`, `humerus-f${k}`, dense(capped(UPPER_ARM), 2), RED, { sd, shape: "chest" });
    S.limb(`${n}-forearm`, `forearm-f${k}`, dense(capped(FOREARM), 2), DARK, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, capped(FORE_CANNON), DARK, { sd, shape: `cannon-f-${k}` });
    paw(S, n, "fore", `paw-f${k}`, sd, FORE_Z, 1.08, `paw-f-${k}`);
  }
}

export const REGION: readonly (readonly [RegExp, string])[] = [
  [/^(hindbody|chest|neck|trunk)$|-(thigh|shoulder|upper-arm)$/, "coat-trunk"], [/-(gaskin|hock-point|cannon|forearm|leg|upper|lower)$/, "coat-legs"], [/^(head|jaw|nose)$|-(eye|cheek)$/, "face"], [/-ear$/, "ears"], [/^brush/, "brush"], [/-paw$|-toe-\d$/, "paws"],
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
  if (o.pair === false) T.ball("th-breath-nose", "head", T.at("head", 0.158, -0.003, 0), [0.004, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  else for (const sd of SIDES) T.ball(`th-breath-nose-${sx(sd)}`, "head", T.at("head", 0.158, -0.003, sd * 0.0042), [0.004, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  T.ball("th-breath-mouth", "head", T.at("head", 0.142, -0.0135, 0), [0.0045, o.mouthR, 0.0042], E, [hu[0]! - hv[0]! * 0.3, hu[1]! - hv[1]! * 0.3, 0]);
  T.ball("th-plume-back", "thorax", o.plume[0], o.plume[1], { material: "red-fur", tone: 0.5 }, [1, 0, 0]);
  for (const t of T.body) out.push({ ...t, region: t.id === "th-plume-back" ? "coat-trunk" : "face", tags: ["emitter", t.id === "th-plume-back" ? "plume-back" : t.id === "th-breath-mouth" ? "breath-mouth" : "breath-nose"] });
  return out;
}

export const BONE: Look = { material: "bone", tone: 0.18 };
export const JOINT: Look = { material: "bone", tone: 0.5 };
export const MASS: Look = { material: "red-fur", tone: 0.7 };
/** Vertebral axis in the standing pose (world): cervical S from the occiput to C7/T1, thoracic to the thoracolumbar joint, lumbar to the lumbosacral joint. */
export const SPINE_C: readonly V3[] = [[0.503, 0.473, 0], [0.482, 0.452, 0], [0.458, 0.418, 0], [0.438, 0.388, 0], [0.424, 0.37, 0], [0.41, 0.362, 0]];
export const SPINE_T: readonly V3[] = [[0.41, 0.362, 0], [0.36, 0.379, 0], [0.3, 0.394, 0], [0.225, 0.404, 0]];
export const SPINE_L: readonly V3[] = [[0.225, 0.404, 0], [0.14, 0.409, 0], [0.05, 0.4, 0]];
export const spineY = (x: number): number => {
  for (let i = 1; i < SPINE_T.length; i++) if (x >= SPINE_T[i]![0]) { const a = SPINE_T[i - 1]!; const b = SPINE_T[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return SPINE_T[SPINE_T.length - 1]![1];
};
/** Ribs 1 to 13: half width of the cage, depth from the spine to the ventral end, caudal sweep of the ventral end. Nine sternal ribs meet the sternum, four end in the costal arch. */
export const RIBS = [[0.022, 0.07, -0.02], [0.03, 0.095, -0.012], [0.038, 0.115, -0.004], [0.045, 0.128, 0.004], [0.05, 0.136, 0.012], [0.054, 0.14, 0.02], [0.057, 0.14, 0.028], [0.059, 0.135, 0.034], [0.06, 0.125, 0.04], [0.059, 0.112, 0.044], [0.056, 0.098, 0.046], [0.052, 0.084, 0.046], [0.046, 0.07, 0.044]] as const;
export const ribX = (i: number): number => 0.402 - i * 0.0138;
export const ribPath = (i: number, sd: number, r: number): PP[] => {
  const [w, d, k] = RIBS[i]!;
  const x = ribX(i);
  const y = spineY(x) - 0.006;
  const end = i < 9 ? 0.2 : 0.72;
  return ([[0, 0, 0.2], [0.15, 0.1, 0.7], [0.45, 0.4, 1], [0.8, 0.75, 0.86], [1, 1, end]] as const).map(([a, b, c]) => ({ p: [x - a * k, y - b * d, sd * c * w] as V3, rl: r, rs: r }));
};
export const STERNUM: readonly PP[] = [{ p: [0.436, 0.292, 0], rl: 0.0055, rs: 0.0045 }, { p: [0.4, 0.254, 0], rl: 0.007, rs: 0.0048 }, { p: [0.36, 0.236, 0], rl: 0.0075, rs: 0.0048 }, { p: [0.32, 0.231, 0], rl: 0.007, rs: 0.0045 }, { p: [0.293, 0.234, 0], rl: 0.005, rs: 0.0042 }];
export const FEMUR = [[0, 0, 0.009, 0.009, 0.034], [0.012, -0.004, 0.011, 0.01, 0.04], [0.035, 0, 0.0065, 0.0065, 0.043], [0.1, 0, 0.0065, 0.0065, 0.045], [0.122, 0.002, 0.0095, 0.0105, 0.045], [0.132, 0.003, 0.009, 0.01, 0.045]] as const;
export const TIBIA = [[0, 0, 0.0105, 0.0105, 0.045], [0.02, 0.002, 0.008, 0.0075, 0.044], [0.06, 0, 0.0055, 0.0055, 0.042], [0.12, 0, 0.005, 0.005, 0.039], [0.138, 0, 0.007, 0.0075, 0.038]] as const;
export const METATARSAL = [[-0.004, 0, 0.0075, 0.008, 0.038], [0.012, 0, 0.0065, 0.0075, 0.038], [0.03, 0, 0.0045, 0.0075, 0.037], [0.07, 0, 0.0045, 0.0085, 0.036], [0.086, 0, 0.0055, 0.0095, 0.036]] as const;
export const DIGITS = [[0.0, -0.001, 0.0055, 0.0095], [0.02, -0.002, 0.0045, 0.0105], [0.04, -0.001, 0.0042, 0.0105], [0.052, 0, 0.0042, 0.008]] as const;
export const SCAPULA = [[-0.004, 0, 0.024, 0.0042, 0.03], [0.015, 0, 0.026, 0.0042, 0.034], [0.05, 0, 0.017, 0.0042, 0.04], [0.08, 0, 0.009, 0.005, 0.044], [0.094, 0, 0.0095, 0.0075, 0.045]] as const;
export const HUMERUS = [[0, 0, 0.0095, 0.009, 0.045], [0.012, 0.004, 0.0105, 0.009, 0.046], [0.04, 0, 0.0062, 0.006, 0.046], [0.09, 0, 0.006, 0.006, 0.043], [0.112, 0, 0.009, 0.0095, 0.041], [0.121, 0, 0.0085, 0.0095, 0.04]] as const;
export const RADIUS = [[0, 0, 0.0075, 0.008, 0.04], [0.02, 0, 0.006, 0.0068, 0.039], [0.09, 0, 0.0055, 0.0065, 0.035], [0.112, 0, 0.007, 0.008, 0.033], [0.119, 0, 0.007, 0.008, 0.033]] as const;
export const METACARPAL = [[-0.004, 0, 0.0065, 0.0085, 0.033], [0.008, 0, 0.0055, 0.0085, 0.033], [0.02, 0, 0.0045, 0.008, 0.032], [0.052, 0, 0.0045, 0.009, 0.032], [0.065, 0, 0.0052, 0.01, 0.032]] as const;
export const CRANIUM = [[-0.004, 0.012, 0.012, 0.014], [0.006, 0.014, 0.02, 0.0225], [0.028, 0.016, 0.0225, 0.0245], [0.05, 0.014, 0.021, 0.023], [0.07, 0.01, 0.017, 0.017], [0.09, 0.005, 0.013, 0.0125], [0.12, 0.002, 0.0105, 0.0105], [0.143, 0.0, 0.0085, 0.009], [0.149, -0.001, 0.006, 0.007]] as const;
export const JOINT_R: Record<string, number> = { pelvis: 0.008, lumbar: 0.009, thorax: 0.008, neck: 0.008, head: 0.009, jaw: 0.005, "tail-1": 0.006, "tail-2": 0.005, "tail-3": 0.0045, ear: 0.0045, thigh: 0.0095, shank: 0.0105, metatarsus: 0.008, "paw-h": 0.0065, scapula: 0.005, humerus: 0.0095, forearm: 0.0085, metacarpus: 0.0075, "paw-f": 0.0065 };
export const JOINT_Z: Record<string, number> = { thigh: 0.034, shank: 0.045, metatarsus: 0.038, "paw-h": 0.036, scapula: 0.03, humerus: 0.045, forearm: 0.04, metacarpus: 0.033, "paw-f": 0.032, ear: 0.031 };
export const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });

/** 🔩️ Joint solids at every rig joint (size of the real joint) and the two trunk masses at their centres of mass. `k` scales the joints for coarser scales. */
export function jointsAndMasses(K: Fox, k = 1, massR: readonly [number, number] = [0.028, 0.022]): void {
  for (const b of K.c.raw.rig.bones as Json[]) {
    const id = b.id as string;
    const stem = id.replace(/-[hf]?[nf]$/, "");
    const key = stem === "paw" ? `paw-${id.slice(4, 5)}` : stem;
    const sd = /-(h|f)?n$/.test(id) ? -1 : /-(h|f)?f$/.test(id) ? 1 : 0;
    const r = Math.max(JOINT_R[key]! * k, 0.56 * K.feature);
    const o = K.frame(id).o;
    mem(K.ball(`joint-${id}`, id, [o[0]!, o[1]!, sd * (JOINT_Z[key] ?? 0)], [r, r, r], JOINT), id, "joint");
  }
  mem(K.ball("mass-thorax", "thorax", [0.33, 0.325, 0], [massR[0], massR[0], massR[0]], MASS), "thorax", "mass");
  mem(K.ball("mass-lumbar", "lumbar", [0.14, 0.355, 0], [massR[1], massR[1], massR[1]], MASS), "lumbar", "mass");
}

/** 🦴️ Structure at 1:50: the skeleton of the fox, bone by bone, inside the coat. */
export function structure(K: Fox): Json[] {
  const fl = 0.0042;
  const thin = { floor: 0.0025 };
  mem(K.limb("skull", "head", dense(CRANIUM, 2), BONE, { e: 2.3 }), "head");
  mem(K.run("cervical-spine", "neck", SPINE_C.map((p, i) => ({ p, rl: i === 0 ? 0.016 : i === 1 ? 0.012 : 0.0105, rs: 0.0085 })), BONE), "neck");
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: 0.008, rs: 0.007 })), BONE), "thorax");
  mem(K.run("thoracic-spinous", "thorax", ([[0.405, 0.011], [0.385, 0.02], [0.35, 0.016], [0.3, 0.011], [0.25, 0.009], [0.228, 0.008]] as const).map(([x, h]) => ({ p: [x, spineY(x) + 0.006 + h, 0] as V3, rl: fl, rs: h })), BONE), "thorax");
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.0095, rs: 0.008 })), BONE), "lumbar");
  mem(K.run("lumbar-transverse", "lumbar", [{ p: [0.208, 0.4, 0], rl: 0.022, rs: fl }, { p: [0.14, 0.404, 0], rl: 0.027, rs: fl }, { p: [0.068, 0.397, 0], rl: 0.024, rs: fl }], BONE), "lumbar");
  mem(K.run("lumbar-spinous", "lumbar", [{ p: [0.215, 0.419, 0], rl: fl, rs: 0.008 }, { p: [0.14, 0.425, 0], rl: fl, rs: 0.009 }, { p: [0.06, 0.417, 0], rl: fl, rs: 0.008 }], BONE), "lumbar");
  mem(K.run("sacrum", "pelvis", [{ p: [0.05, 0.4, 0], rl: 0.014, rs: 0.008 }, { p: [0.01, 0.412, 0], rl: 0.011, rs: 0.007 }, { p: [-0.03, 0.422, 0], rl: 0.007, rs: 0.006 }], BONE), "pelvis");
  mem(K.run("pubis", "pelvis", [{ p: [0.004, 0.353, -0.03], rl: 0.005, rs: 0.0045 }, { p: [0.004, 0.347, 0], rl: 0.006, rs: 0.0045 }, { p: [0.004, 0.353, 0.03], rl: 0.005, rs: 0.0045 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM, BONE), "thorax");
  const tails = [["tail-1", 0.15, 0.006, 0.005], ["tail-2", 0.14, 0.005, 0.0045], ["tail-3", 0.11, 0.0045, fl]] as const;
  for (const [i, [b, l, r0, r1]] of tails.entries()) mem(K.run(`caudal-${i + 1}`, b, [{ p: K.at(b, -0.004, 0), rl: r0, rs: r0 }, { p: K.at(b, l, 0), rl: r1, rs: r1 }], BONE), b);
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-zygomatic-arch`, "head", [{ p: K.at("head", 0.032, 0.006, sd * 0.027), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.052, 0.001, sd * 0.0365), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.074, 0.0, sd * 0.032), rl: fl, rs: 0.0048 }, { p: K.at("head", 0.088, 0.002, sd * 0.02), rl: fl, rs: 0.0048 }], BONE), "head");
    mem(K.run(`${n}-mandible`, "jaw", [{ p: K.at("head", 0.03, -0.002, sd * 0.029), rl: fl, rs: 0.0095 }, { p: K.at("head", 0.042, -0.019, sd * 0.027), rl: fl, rs: 0.0085 }, { p: K.at("head", 0.072, -0.021, sd * 0.02), rl: fl, rs: 0.007 }, { p: K.at("head", 0.106, -0.019, sd * 0.012), rl: fl, rs: 0.0055 }, { p: K.at("head", 0.136, -0.017, sd * 0.0045), rl: fl, rs: 0.0045 }], BONE), "jaw");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.086, [[0.02, 0.014, fl], [0.3, 0.021, fl], [0.7, 0.012, fl], [1, 0.0045, fl]]), `ear-${k}`);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.074, 0.41, sd * 0.036], rl: 0.0045, rs: 0.017 }, { p: [0.04, 0.4, sd * 0.03], rl: 0.0045, rs: 0.012 }, { p: [0.0, 0.376, sd * 0.03], rl: 0.007, rs: 0.011 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.002, 0.374, sd * 0.03], rl: 0.0055, rs: 0.009 }, { p: [-0.03, 0.368, sd * 0.03], rl: 0.005, rs: 0.0075 }, { p: [-0.05, 0.372, sd * 0.034], rl: 0.0065, rs: 0.011 }], BONE), "pelvis");
    for (let i = 0; i < 13; i++) mem(Object.assign(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, i === 0 ? 0.0018 : 0.0025), BONE, i === 0 ? { floor: 0.0018 } : thin), { slender: true }), "thorax");
    mem(Object.assign(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.3, 0.236, sd * 0.01], rl: 0.0025, rs: 0.0025 }, ...[9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, 0.0025)[4]!.p, rl: 0.0025, rs: 0.0025 }))], BONE, thin), { slender: true }), "thorax");
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
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    const fa = `forearm-f${k}`;
    mem(K.limb(`${n}-radius-ulna`, fa, RADIUS, BONE, { sd }), fa);
    mem(K.run(`${n}-olecranon`, fa, [{ p: K.at(fa, 0.008, -0.007, sd * 0.04), rl: 0.0048, rs: 0.0055 }, { p: K.at(fa, -0.021, -0.012, sd * 0.04), rl: 0.0045, rs: 0.005 }], BONE), fa);
    mem(K.limb(`${n}-metacarpals`, `metacarpus-f${k}`, METACARPAL, BONE, { sd, e: 2.6 }), `metacarpus-f${k}`);
    mem(K.limb(`${n}-fore-digits`, `paw-f${k}`, DIGITS.map((r) => [r[0] * 0.92, r[1], r[2], r[3] * 1.06, FORE_Z]), BONE, { sd, e: 2.6 }), `paw-f${k}`);
  }
  jointsAndMasses(K);
  return K.body;
}

const DECISIONS = [
  {
    en: "Body at 1:50: 51 solids, every section stated by hand from the skeletal landmarks of a 6 kg red fox in autumn coat (withers 0.44 m with coat, hip joint 0.375 m, elbow 0.217 m, hock 0.123 m). The trunk is two overlapping lofts (loin on the lumbar bone, chest on the thorax bone) with a level back, a deep narrow chest and a tucked flank; thigh, shoulder and upper arm are muscle masses that swell out of the trunk; the neck carries its ruff; the head is a flat broad skull with a shallow stop, a long narrow muzzle, a separate pale mandible, cheek ruffs that make the face a wedge from the front, a nose pad and two eyes; the ears are large cupped leaves turned forward and outward; the brush is three bushy links, the last one the pale tip. Each leg has a gaskin or forearm with the muscle belly at the top, a point of hock, a slim cannon and a paw with pad mass and four toes, the two middle toes leading; segments overlap at every joint so that poses stay closed.",
    de: "Körper bei 1:50: 51 Körper, jeder Schnitt von Hand nach den Skelettpunkten eines 6 kg schweren Rotfuchses im Herbstfell gesetzt (Widerrist 0,44 m mit Fell, Hüftgelenk 0,375 m, Ellbogen 0,217 m, Sprunggelenk 0,123 m). Der Rumpf besteht aus zwei überlappenden Lofts (Lende am Lendenknochen, Brust am Brustknochen) mit geradem Rücken, tiefer schmaler Brust und aufgezogener Flanke; Keule, Schulter und Oberarm sind Muskelmassen, die aus dem Rumpf hervortreten; der Hals trägt seine Krause; der Kopf ist ein flacher breiter Schädel mit schwachem Stop, langem schmalem Fang, eigenem hellem Unterkiefer, Backenbart, der das Gesicht von vorn zum Keil macht, Nasenspiegel und zwei Augen; die Ohren sind große gewölbte Blätter, nach vorn und außen gedreht; die Rute besteht aus drei buschigen Gliedern, das letzte ist die helle Spitze. Jedes Bein hat Unterschenkel oder Unterarm mit dem Muskelbauch oben, Fersenhöcker, schlanke Röhre und eine Pfote mit Ballenmasse und vier Zehen, die beiden mittleren voran; die Segmente überlappen an jedem Gelenk, damit Posen geschlossen bleiben.",
  },
  {
    en: "Thermal at 1:50: every region declares the single layer of its fur, so each region is the coat surface it covers (references to the body solids, layer fur): trunk coat with neck, thighs, shoulders and upper arms; leg coat of gaskins, hocks, forearms and cannons; face with mandible, cheeks, nose pad and eyes; ears; brush; paws with toes. The outlets lie on the surface: two nostrils on the nose pad, the lips below it, and a strip of the back coat above the thoracolumbar joint for the plume.",
    de: "Thermik bei 1:50: jede Region deklariert die eine Schicht ihres Fells, daher ist jede Region die Felloberfläche, die sie bedeckt (Verweise auf die Körper, Schicht fur): Rumpffell mit Hals, Keulen, Schultern und Oberarmen; Beinfell von Unterschenkeln, Fersen, Unterarmen und Röhren; Gesicht mit Unterkiefer, Backen, Nasenspiegel und Augen; Ohren; Rute; Pfoten mit Zehen. Die Auslässe liegen auf der Oberfläche: zwei Nasenlöcher im Nasenspiegel, die Lippen darunter und ein Streifen des Rückenfells über dem Brust-Lenden-Übergang für die Warmluftfahne.",
  },
  {
    en: "Structure at 1:50: the skeleton inside the coat. Skull with zygomatic arches and the two halves of the mandible, cervical spine in its S curve, thoracic spine with the spinous processes of the withers, thirteen ribs per side (nine sternal, four ending in the costal arch) at their real 5 mm as slender solids, sternum, lumbar spine with transverse and spinous processes, sacrum, ilium, ischium and pubis, caudal vertebrae along the brush, ear cartilages; per foreleg scapula, humerus, radius and ulna with olecranon, metacarpals and digits; per hind leg femur with patella, tibia, calcaneus, metatarsals and digits. Joints sit at the rig joints at the size of the real joint, the masses of thorax and loin at their centres of mass. The asset declares no supports, so there are no support solids.",
    de: "Tragwerk bei 1:50: das Skelett im Fell. Schädel mit Jochbögen und den beiden Unterkieferhälften, Halswirbelsäule in ihrer S-Kurve, Brustwirbelsäule mit den Dornfortsätzen des Widerrists, dreizehn Rippen je Seite (neun echte, vier im Rippenbogen endend) in ihren wirklichen 5 mm als schlanke Körper, Brustbein, Lendenwirbelsäule mit Quer- und Dornfortsätzen, Kreuzbein, Darmbein, Sitzbein und Schambein, Schwanzwirbel entlang der Rute, Ohrknorpel; je Vorderbein Schulterblatt, Oberarmbein, Speiche und Elle mit Ellbogenhöcker, Mittelhandknochen und Zehen; je Hinterbein Oberschenkelbein mit Kniescheibe, Schienbein, Fersenbein, Mittelfußknochen und Zehen. Gelenke sitzen an den Gelenken des Skeletts in wirklicher Gelenkgröße, die Massen von Brust und Lende in ihren Schwerpunkten. Das Objekt deklariert keine Auflager, daher gibt es keine Auflagerkörper.",
  },
];

if (import.meta.main) {
  const S = new Fox(SPEC, 50);
  body(S);
  const th = thermal(S, new Fox(SPEC, 50), { nostrilR: 0.0042, mouthR: 0.008, plume: [[0.235, 0.4335, 0], [0.1, 0.03, 0.0075]] });
  const st = structure(new Fox(SPEC, 50));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
    writeFileSync(`${OUT}/shapes.50.json`, JSON.stringify(S.shapes));
  }
}
