/** 🐏️ Round 5 sheep at 1:50 (round 3 tables, head, ears, ham, shoulder and cervical spine remodelled): the finest model. Body: fleece trunk in two overlapping lofts, wool ham and shoulder, neck fleece, skull and mandible, eyes, leaf ears, wool tail, per leg wool gaskin or forearm, point of hock, cannon with fetlock, dewclaws, pastern and a cloven hoof of two claws. Thermal: the body surfaces per region and the outlets. Structure: the skeleton.
 * Usage: bun r5-sheep-50.ts [--write]
 * @see ./r3-sheep-kit.ts
 * @see ./r3-animals-sheep.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Sheep, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-sheep-kit.ts";

const FLEECE: Look = { material: "fleece", tone: 0.3 };
const WOOL: Look = { material: "fleece", tone: 0.36 };
const HAIR: Look = { material: "short-hair", tone: 0.46 };
const FACE: Look = { material: "short-hair", tone: 0.42 };
const EAR: Look = { material: "bare-skin", tone: 0.5 };
const HORN: Look = { material: "hoof-horn", tone: 0.86 };
const EYE: Look = { material: "bare-skin", tone: 0.96 };
const SIDES = [-1, 1] as const;

export const TRUNK = [
  [-0.152, 0.6, 0.026, 0.026, 2.0],
  [-0.14, 0.597, 0.082, 0.076, 2.0],
  [-0.115, 0.59, 0.132, 0.122, 2.1],
  [-0.07, 0.58, 0.17, 0.168, 2.2],
  [-0.01, 0.566, 0.19, 0.194, 2.3],
  [0.07, 0.555, 0.202, 0.2, 2.3],
  [0.17, 0.545, 0.213, 0.214, 2.35],
  [0.29, 0.54, 0.218, 0.222, 2.35],
  [0.41, 0.541, 0.216, 0.219, 2.35],
  [0.52, 0.549, 0.208, 0.209, 2.3],
  [0.61, 0.56, 0.197, 0.196, 2.25],
  [0.685, 0.571, 0.182, 0.174, 2.2],
  [0.745, 0.588, 0.148, 0.135, 2.1],
  [0.79, 0.6, 0.095, 0.09, 2.0],
] as const;
export const NECK = [
  [-0.03, 0.03, 0.11, 0.1],
  [0.03, 0.036, 0.122, 0.106],
  [0.1, 0.03, 0.108, 0.092],
  [0.17, 0.018, 0.086, 0.076],
  [0.23, 0.01, 0.07, 0.065],
  [0.28, 0.0, 0.055, 0.054],
  [0.31, -0.008, 0.036, 0.04],
] as const;
export const SKULL = [
  [-0.02, 0.03, 0.032, 0.034],
  [0.0, 0.03, 0.046, 0.048],
  [0.035, 0.031, 0.054, 0.06],
  [0.07, 0.03, 0.054, 0.064],
  [0.105, 0.026, 0.05, 0.056],
  [0.14, 0.02, 0.046, 0.045],
  [0.175, 0.014, 0.043, 0.038],
  [0.205, 0.008, 0.04, 0.034],
  [0.226, 0.004, 0.035, 0.031],
  [0.24, 0.0, 0.026, 0.025],
  [0.247, -0.002, 0.012, 0.014],
] as const;
export const MANDIBLE = [
  [0.03, -0.03, 0.034, 0.05],
  [0.065, -0.032, 0.034, 0.052],
  [0.1, -0.033, 0.03, 0.044],
  [0.14, -0.031, 0.025, 0.034],
  [0.18, -0.03, 0.021, 0.027],
  [0.21, -0.028, 0.019, 0.025],
  [0.23, -0.026, 0.014, 0.021],
  [0.238, -0.024, 0.008, 0.013],
] as const;
export const HAM = [[-0.02, -0.02, 0.05, 0.035, 0.09], [0.05, -0.025, 0.088, 0.054, 0.103], [0.11, -0.026, 0.08, 0.052, 0.106], [0.17, -0.016, 0.058, 0.046, 0.108], [0.215, -0.008, 0.046, 0.042, 0.11]] as const;
export const GASKIN = [[-0.045, 0, 0.05, 0.04, 0.122], [0.0, 0, 0.058, 0.046, 0.118], [0.05, -0.004, 0.05, 0.04, 0.113], [0.11, -0.01, 0.038, 0.03, 0.107], [0.165, -0.014, 0.03, 0.022, 0.102], [0.205, -0.014, 0.024, 0.018, 0.098], [0.238, -0.01, 0.019, 0.0155, 0.097]] as const;
export const HIND_CANNON = [[-0.02, -0.004, 0.024, 0.019, 0.097], [0.02, -0.003, 0.022, 0.018, 0.097], [0.05, 0, 0.0165, 0.0135, 0.098], [0.1, 0, 0.0155, 0.013, 0.1], [0.145, 0, 0.0165, 0.0145, 0.102], [0.172, -0.002, 0.021, 0.0185, 0.103], [0.19, -0.002, 0.018, 0.017, 0.103]] as const;
export const PASTERN = [[-0.012, 0, 0.018, 0.0175], [0.03, 0, 0.0165, 0.019], [0.066, 0.002, 0.0165, 0.0215]] as const;
export const CLAW = [[-0.055, 0.01, 0.01, 0.008, 3, 0.011], [-0.044, 0.0145, 0.0145, 0.0095, 3, 0.0115], [-0.031, 0.0165, 0.0165, 0.01, 3, 0.012], [-0.018, 0.013, 0.013, 0.0095, 3, 0.0127], [-0.007, 0.0075, 0.0075, 0.0075, 3, 0.0133], [-0.001, 0.0045, 0.0045, 0.005, 3, 0.0136]] as const;
export const SHOULDER = [[-0.07, -0.02, 0.055, 0.036, 0.08], [0.0, -0.01, 0.074, 0.046, 0.09], [0.07, -0.012, 0.062, 0.04, 0.096], [0.13, -0.01, 0.046, 0.034, 0.1], [0.17, -0.007, 0.038, 0.031, 0.102]] as const;
export const FOREARM = [[-0.04, -0.012, 0.04, 0.03, 0.107], [0.0, -0.008, 0.043, 0.033, 0.105], [0.045, -0.004, 0.036, 0.029, 0.101], [0.1, 0, 0.027, 0.023, 0.096], [0.15, 0, 0.021, 0.0185, 0.09], [0.19, 0, 0.0185, 0.017, 0.088]] as const;
export const FORE_CANNON = [[-0.018, 0.001, 0.022, 0.0205, 0.088], [0.012, 0.002, 0.0215, 0.02, 0.088], [0.035, 0, 0.0155, 0.014, 0.088], [0.08, 0, 0.0145, 0.0135, 0.089], [0.112, 0, 0.0155, 0.0145, 0.09], [0.134, -0.002, 0.02, 0.0185, 0.09], [0.15, -0.002, 0.0175, 0.017, 0.09]] as const;
export const HIND_Z = 0.104;
export const FORE_Z = 0.09;
export const EAR_T = [[0, 0.012, 0.009], [0.15, 0.021, 0.007], [0.4, 0.028, 0.0052], [0.7, 0.025, 0.0048], [0.9, 0.015, 0.0045], [1, 0.006, 0.0042]] as const;

/** 🦻️ Ear leaf of side `sd` from its base on the head, carried horizontally out to the side, a little back and down. */
export function ear(S: Sheep, id: string, sd: -1 | 1, look: Look, len = 0.105, rows: readonly (readonly number[])[] = EAR_T, shape?: string): Json {
  const b = S.at("head", 0.022, 0.04, sd * 0.043);
  const d: V3 = [-0.26, -0.3, sd * 0.918];
  const pts: PP[] = rows.map(([t, rl, rs]) => ({ p: [b[0] + d[0] * len * t!, b[1] + d[1] * len * t!, b[2] + d[2] * len * t!] as V3, rl: rl!, rs: rs!, e: 2.3 }));
  return S.run(id, `ear-${sx(sd)}`, pts, look, { lat: [0.35, 0.94, 0], ...(shape === undefined ? {} : { shape }) });
}

export function body(S: Sheep): void {
  const T = dense(TRUNK, 2);
  const cut = (lo: number, hi: number, tuck: "start" | "end"): number[][] => {
    const rows = T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9).map((r) => [...r]);
    const k = tuck === "end" ? rows.length - 1 : 0;
    rows[k]![2]! -= 0.006;
    rows[k]![3]! -= 0.006;
    return rows;
  };
  S.hull("hindbody", "lumbar", cut(-0.2, 0.41, "end"), FLEECE, { shape: "hindbody" });
  S.hull("chest", "thorax", cut(0.29, 0.9, "start"), FLEECE, { shape: "chest" });
  S.limb("neck", "neck", dense(NECK, 2), FLEECE, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(SKULL, 2), FACE, { shape: "head", e: 2.4 });
  S.limb("jaw", "jaw", dense(MANDIBLE, 2), FACE, { frame: "head", shape: "jaw", e: 2.3 });
  S.run("tail-root", "tail-1", [{ p: [-0.095, 0.662, 0], rl: 0.03, rs: 0.026 }, { p: [-0.125, 0.615, 0], rl: 0.04, rs: 0.034 }, { p: [-0.142, 0.565, 0], rl: 0.041, rs: 0.035 }], WOOL, { shape: "tail-root" });
  S.run("tail-tip", "tail-2", [{ p: [-0.141, 0.578, 0], rl: 0.04, rs: 0.034 }, { p: [-0.148, 0.52, 0], rl: 0.036, rs: 0.03 }, { p: [-0.15, 0.482, 0], rl: 0.028, rs: 0.024 }, { p: [-0.151, 0.468, 0], rl: 0.014, rs: 0.012 }], WOOL, { shape: "tail-tip" });
  const hu = S.frame("head").u;
  S.ball("topknot", "head", S.at("head", 0.025, 0.062, 0), [0.05, 0.05, 0.026], WOOL, [hu[0]!, hu[1]!, 0], "head");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.ball(`${n}-eye`, "head", S.at("head", 0.088, 0.04, sd * 0.054), [0.0105, 0.013, 0.013], EYE, [0, 0, sd], sd < 0 ? "eye" : "eye-f");
    ear(S, `${n}-ear`, sd, EAR, 0.105, EAR_T, `ear-${k}`);
    S.limb(`${n}-ham`, `thigh-h${k}`, HAM, WOOL, { sd, ...(sd < 0 ? { shape: "fold-stifle" } : {}) });
    S.limb(`${n}-gaskin`, `shank-h${k}`, GASKIN, WOOL, { sd, shape: `gaskin-${k}` });
    S.run(`${n}-hock-point`, `metatarsus-h${k}`, [{ p: S.at(`metatarsus-h${k}`, 0.022, -0.012, sd * 0.097), rl: 0.013, rs: 0.013 }, { p: S.at(`metatarsus-h${k}`, -0.008, -0.022, sd * 0.097), rl: 0.012, rs: 0.012 }, { p: S.at(`metatarsus-h${k}`, -0.03, -0.03, sd * 0.097), rl: 0.0105, rs: 0.011 }], HAIR, { shape: `cannon-h-${k}` });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, HIND_CANNON, HAIR, { sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-shoulder`, `humerus-f${k}`, SHOULDER, FLEECE, { sd, ...(sd < 0 ? { shape: "fold-elbow" } : {}) });
    S.limb(`${n}-forearm`, `forearm-f${k}`, FOREARM, WOOL, { sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, FORE_CANNON, HAIR, { sd, shape: `cannon-f-${k}` });
    for (const [leg, cannon, s, z] of [["hind", `metatarsus-h${k}`, 0.182, HIND_Z], ["fore", `metacarpus-f${k}`, 0.143, FORE_Z]] as const) {
      const h = leg === "hind" ? "h" : "f";
      const hoof = `hoof-${h}${k}`;
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.ball(`${n}-${leg}-dewclaw-${j}`, cannon, S.at(cannon, s, -0.019, sd * z + w * 0.0105), [0.008, 0.0055, 0.0055], HORN, [S.frame(cannon).u[0]!, S.frame(cannon).u[1]!, 0], `cannon-${h}-${k}`);
      S.limb(`${n}-${leg}-pastern`, hoof, PASTERN.map((r) => [...r, z]), HAIR, { sd, shape: `hoof-${h}-${k}` });
      const f = S.frame(hoof);
      const len = S.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
      const toe = f.o[0]! + f.u[0]! * len;
      const ground = f.o[1]! + f.u[1]! * len;
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.hull(`${n}-${leg}-claw-${j}`, hoof, CLAW.map((r) => [r[0], r[1] + ground, r[2], r[3], r[4], sd * z + w * r[5]]), HORN, { x0: toe, sd: 1, shape: `hoof-${h}-${k}` });
    }
  }
}

export const REGION: readonly (readonly [RegExp, string, string?])[] = [
  [/^(hindbody|chest|trunk)$|-(ham|shoulder)$/, "fleece-trunk", "fleece"], [/^(neck|topknot)$/, "fleece-neck", "fleece"], [/-(gaskin|forearm|upper)$/, "wool-upper-legs", "wool"], [/-(hock-point|cannon|pastern|lower)$/, "lower-legs", "hair"],
  [/^(head|jaw)$|-eye$/, "face", "hair"], [/-ear$/, "ears"], [/^tail/, "tail", "wool"], [/-(claw|dewclaw)(-\d)?$|-hoof$/, "hooves"],
];

/** 🌡️ Thermal list of a scale: every body solid as the surface of its region, and the outlets (nostrils, mouth, plume patch on the back) as own solids. */
export function thermal(S: Sheep, T: Sheep, o: { nostrilR: number; mouthR: number; plume: readonly [V3, V3]; pair?: boolean; skip?: RegExp }): Json[] {
  const out: Json[] = S.body.filter((b) => o.skip === undefined || !o.skip.test(b.id)).map((b) => {
    const hit = REGION.find(([re]) => re.test(b.id));
    if (hit === undefined) throw new Error(`no region for ${b.id}`);
    return { id: `th-${b.id}`, of: b.id, region: hit[1], ...(hit[2] === undefined ? {} : { layer: hit[2] }) };
  });
  const hu = T.frame("head").u;
  const hv = T.frame("head").v;
  const tip = S.c.raw.rig.bones.find((b: Json) => b.id === "head").lengthM as number;
  const E: Look = { material: "bare-skin", tone: 0.9 };
  if (o.pair === false) T.ball("th-breath-nose", "head", T.at("head", tip + 0.004, 0.006, 0), [0.0045, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  else for (const sd of SIDES) T.ball(`th-breath-nose-${sx(sd)}`, "head", T.at("head", tip + 0.004, 0.006, sd * 0.011), [0.0045, o.nostrilR, o.nostrilR], E, [hu[0]!, hu[1]!, 0]);
  T.ball("th-breath-mouth", "head", T.at("head", tip - 0.004, -0.013, 0), [0.0045, o.mouthR, o.nostrilR], E, [hu[0]! - hv[0]! * 0.6, hu[1]! - hv[1]! * 0.6, 0]);
  T.ball("th-plume-back", "thorax", o.plume[0], o.plume[1], { material: "fleece", tone: 0.2 }, [1, 0, 0]);
  for (const t of T.body) out.push({ ...t, region: t.id === "th-plume-back" ? "fleece-trunk" : "face", tags: ["emitter", t.id === "th-plume-back" ? "plume-back" : t.id === "th-breath-mouth" ? "breath-mouth" : "breath-nose"] });
  return out;
}

export const BONE: Look = { material: "bone", tone: 0.18 };
const JOINT: Look = { material: "bone", tone: 0.5 };
const MASS: Look = { material: "fleece", tone: 0.7 };
/** Thoracic and lumbar vertebral axis (world, standing) and the dorsal spinous processes above it. */
export const SPINE_T: readonly V3[] = [[0.345, 0.66, 0], [0.43, 0.655, 0], [0.52, 0.64, 0], [0.6, 0.612, 0], [0.67, 0.575, 0]];
export const SPINE_C: readonly V3[] = [[0.67, 0.575, 0], [0.745, 0.588, 0], [0.8, 0.628, 0], [0.84, 0.684, 0], [0.874, 0.74, 0], [0.907, 0.774, 0]];
export const spineY = (x: number): number => {
  for (let i = 1; i < SPINE_T.length; i++) if (x <= SPINE_T[i]![0]) { const a = SPINE_T[i - 1]!; const b = SPINE_T[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return SPINE_T[SPINE_T.length - 1]![1];
};
/** Ribs 1 to 13: half width of the cage, depth from the spine to the ventral end, caudal sweep of the ventral end. */
export const RIBS = [[0.055, 0.15, -0.01], [0.07, 0.17, 0], [0.085, 0.19, 0.01], [0.098, 0.205, 0.02], [0.108, 0.215, 0.03], [0.116, 0.225, 0.04], [0.122, 0.23, 0.05], [0.127, 0.23, 0.055], [0.13, 0.22, 0.06], [0.13, 0.2, 0.06], [0.126, 0.17, 0.06], [0.118, 0.14, 0.055], [0.105, 0.11, 0.05]] as const;
export const ribPath = (i: number, sd: number, r: number): PP[] => {
  const [w, d, k] = RIBS[i]!;
  const x = ribX(i);
  const y = spineY(x) - 0.012;
  const bow: PP[] = ([[0, 0, 0.16], [0.2, 0.12, 0.72], [0.5, 0.42, 1], [0.8, 0.75, 0.88], [1, 1, 0.6]] as const).map(([a, b, c]) => ({ p: [x - a * k, y - b * d, sd * c * w] as V3, rl: r, rs: r }));
  if (i < STERNAL) { const xs = 0.69 - i * 0.026; bow.push({ p: [xs, sternumY(xs) + 0.004, sd * 0.012], rl: r, rs: r }); }
  return bow;
};
export const ribX = (i: number): number => 0.655 - i * 0.0242;
/** Sternum (world): manubrium to xiphoid; the first eight ribs reach it through their costal cartilages. */
export const STERNUM: readonly V3[] = [[0.705, 0.458, 0], [0.64, 0.415, 0], [0.56, 0.398, 0], [0.5, 0.405, 0]];
export const STERNAL = 8;
export function sternumY(x: number): number {
  for (let i = 1; i < STERNUM.length; i++) if (x >= STERNUM[i]![0]) { const a = STERNUM[i - 1]!; const b = STERNUM[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return STERNUM[STERNUM.length - 1]![1];
}
/** Spinous processes T1 to T13: height above the vertebral body, caudal lean of the tip. The long processes of T2 to T5 carry the withers under the fleece; T13 stands upright. */
export const SPINOUS_T = [[0.05, 0.02], [0.064, 0.028], [0.07, 0.032], [0.07, 0.032], [0.066, 0.03], [0.06, 0.026], [0.054, 0.022], [0.048, 0.018], [0.043, 0.014], [0.039, 0.01], [0.036, 0.006], [0.034, 0.002], [0.033, 0]] as const;
/** Lumbar vertebrae L1 to L6: x of the body, height of the spinous process, half span of the transverse processes. */
export const LUMBAR_V = [[0.325, 0.03, 0.048], [0.283, 0.03, 0.054], [0.241, 0.03, 0.058], [0.199, 0.03, 0.058], [0.157, 0.029, 0.054], [0.118, 0.027, 0.046]] as const;
export const lumbarY = (x: number): number => (x >= 0.225 ? 0.664 - ((x - 0.225) / 0.12) * 0.004 : 0.655 + ((x - 0.105) / 0.12) * 0.009);
/** 🪶️ One spinous process as a thin plate from the vertebral body to its tip. */
export function spinous(K: Sheep, id: string, bone: string, x: number, y: number, h: number, lean: number, half: number, thick: number): Json {
  return K.run(id, bone, [{ p: [x, y, 0], rl: thick, rs: half }, { p: [x - lean * 0.6, y + h * 0.6, 0], rl: thick, rs: half * 0.9 }, { p: [x - lean, y + h, 0], rl: thick, rs: half * 0.7 }], BONE, { floor: thick });
}
export const FEMUR = [[0, 0, 0.016, 0.016, 0.088], [0.025, 0, 0.019, 0.017, 0.091], [0.06, 0, 0.0115, 0.0115, 0.096], [0.13, 0, 0.0115, 0.0115, 0.105], [0.17, 0, 0.019, 0.018, 0.11], [0.195, 0.004, 0.021, 0.02, 0.114]] as const;
export const TIBIA = [[0, 0, 0.021, 0.02, 0.116], [0.03, 0.004, 0.016, 0.014, 0.113], [0.09, 0, 0.01, 0.0095, 0.107], [0.17, 0, 0.0095, 0.009, 0.101], [0.205, 0, 0.014, 0.0135, 0.098], [0.218, 0, 0.0135, 0.013, 0.097]] as const;
export const METATARSAL = [[-0.006, 0, 0.015, 0.014, 0.097], [0.028, 0, 0.0135, 0.013, 0.097], [0.05, 0, 0.0085, 0.009, 0.098], [0.14, 0, 0.0085, 0.009, 0.102], [0.165, 0, 0.0125, 0.014, 0.103], [0.178, 0, 0.012, 0.014, 0.103]] as const;
export const DIGIT = [[0.002, 0, 0.0085, 0.0075], [0.03, 0, 0.0065, 0.0065], [0.045, 0, 0.008, 0.0075], [0.066, 0, 0.0075, 0.007]] as const;
export const SCAPULA = [[-0.012, 0, 0.05, 0.0045, 0.05], [0.02, 0, 0.048, 0.0045, 0.055], [0.09, 0, 0.03, 0.005, 0.072], [0.15, 0, 0.014, 0.007, 0.088], [0.176, 0, 0.016, 0.012, 0.094]] as const;
export const HUMERUS = [[0, 0, 0.019, 0.018, 0.095], [0.02, 0.008, 0.019, 0.017, 0.096], [0.06, 0, 0.0115, 0.011, 0.099], [0.11, 0, 0.011, 0.011, 0.102], [0.14, 0, 0.017, 0.017, 0.104], [0.152, 0, 0.016, 0.018, 0.105]] as const;
export const RADIUS = [[0, 0, 0.014, 0.016, 0.105], [0.03, 0, 0.0095, 0.012, 0.102], [0.13, 0, 0.0095, 0.0115, 0.093], [0.17, 0, 0.0125, 0.0155, 0.089], [0.182, 0, 0.012, 0.015, 0.088]] as const;
export const METACARPAL = [[-0.004, 0, 0.013, 0.0155, 0.088], [0.018, 0, 0.0125, 0.015, 0.088], [0.03, 0, 0.008, 0.0105, 0.088], [0.11, 0, 0.008, 0.0105, 0.09], [0.128, 0, 0.011, 0.0135, 0.09], [0.138, 0, 0.0105, 0.0135, 0.09]] as const;
export const CRANIUM = [[-0.012, 0.03, 0.026, 0.026], [0.0, 0.03, 0.035, 0.035], [0.035, 0.032, 0.043, 0.046], [0.07, 0.031, 0.042, 0.05], [0.105, 0.03, 0.036, 0.042], [0.14, 0.026, 0.031, 0.032], [0.175, 0.02, 0.027, 0.026], [0.208, 0.013, 0.022, 0.022], [0.232, 0.005, 0.013, 0.016]] as const;
export const JAWBONE = [[0.034, -0.012, 0.02, 0.04], [0.05, -0.036, 0.03, 0.041], [0.095, -0.04, 0.018, 0.033], [0.14, -0.029, 0.013, 0.024], [0.185, -0.023, 0.01, 0.017], [0.218, -0.019, 0.008, 0.012]] as const;

/** 🦴️ Structure at 1:50: the skeleton, bone by bone, joints at the rig joints, the two trunk masses at their centres of mass. */
export function structure(K: Sheep): Json[] {
  const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });
  const fl = 0.0042;
  mem(K.run("lumbar-spine", "lumbar", [{ p: [0.105, 0.655, 0], rl: 0.019, rs: 0.017 }, { p: [0.225, 0.664, 0], rl: 0.019, rs: 0.017 }, { p: [0.345, 0.66, 0], rl: 0.018, rs: 0.016 }], BONE), "lumbar");
  for (const [i, [x, h, w]] of LUMBAR_V.entries()) {
    mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", x, lumbarY(x) + 0.012, h, -0.006, 0.014, fl), "lumbar");
    mem(K.run(`lumbar-transverse-${i + 1}`, "lumbar", [{ p: [x + 0.006, lumbarY(x) + 0.002, -w], rl: 0.009, rs: fl }, { p: [x, lumbarY(x) + 0.004, 0], rl: 0.012, rs: 0.006 }, { p: [x + 0.006, lumbarY(x) + 0.002, w], rl: 0.009, rs: fl }], BONE, { lat: [1, 0, 0], floor: fl }), "lumbar");
  }
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: 0.015, rs: 0.014 })), BONE), "thorax");
  for (const [i, [h, lean]] of SPINOUS_T.entries()) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.01, h, lean, 0.0085, fl), "thorax");
  mem(K.run("cervical-spine", "neck", SPINE_C.map((p, i) => ({ p, rl: i > 3 ? 0.022 : 0.02, rs: 0.016 })), BONE), "neck");
  mem(K.run("sacrum", "pelvis", [{ p: [0.105, 0.655, 0], rl: 0.032, rs: 0.016 }, { p: [0.03, 0.667, 0], rl: 0.022, rs: 0.014 }, { p: [-0.04, 0.668, 0], rl: 0.012, rs: 0.01 }, { p: [-0.078, 0.662, 0], rl: 0.008, rs: 0.007 }], BONE), "pelvis");
  mem(K.run("pubis", "pelvis", [{ p: [0.01, 0.566, -0.07], rl: 0.009, rs: 0.009 }, { p: [0.012, 0.556, 0], rl: 0.011, rs: 0.009 }, { p: [0.01, 0.566, 0.07], rl: 0.009, rs: 0.009 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: [0.012, 0.017, 0.018, 0.012][i]!, rs: [0.009, 0.01, 0.009, 0.006][i]! })), BONE), "thorax");
  mem(K.limb("skull", "head", CRANIUM, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", JAWBONE, BONE, { frame: "head", e: 2.3 }), "jaw");
  for (const [i, b] of ["tail-1", "tail-2"].entries()) { const f = K.frame(b); const l = i === 0 ? 0.1 : 0.088; mem(K.run(`caudal-${i + 1}`, b, [{ p: K.at(b, -0.004, 0), rl: 0.0065 - i * 0.0015, rs: 0.006 - i * 0.0012 }, { p: [f.o[0]! + f.u[0]! * l, f.o[1]! + f.u[1]! * l, 0], rl: 0.005 - i * 0.0008, rs: 0.0048 - i * 0.0006 }], BONE), b); }
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.118, 0.674, sd * 0.105], rl: 0.008, rs: 0.03 }, { p: [0.085, 0.662, sd * 0.07], rl: 0.007, rs: 0.026 }, { p: [0.045, 0.632, sd * 0.07], rl: 0.009, rs: 0.013 }, { p: [0.0, 0.592, sd * 0.078], rl: 0.016, rs: 0.02 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.0, 0.59, sd * 0.078], rl: 0.012, rs: 0.014 }, { p: [-0.05, 0.586, sd * 0.07], rl: 0.009, rs: 0.012 }, { p: [-0.092, 0.602, sd * 0.068], rl: 0.012, rs: 0.018 }], BONE), "pelvis");
    for (let i = 0; i < 13; i++) mem(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, 0.0045), BONE), "thorax");
    mem(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.515, 0.407, sd * 0.03], rl: fl, rs: fl }, ...[7, 8, 9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, fl)[4]!.p, rl: fl, rs: fl }))], BONE), "thorax");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.098, [[0.02, 0.008, fl], [0.4, 0.014, fl], [1, 0.006, fl]]), `ear-${k}`);
    mem(K.limb(`${n}-femur`, `thigh-h${k}`, FEMUR, BONE, { sd }), `thigh-h${k}`);
    mem(K.ball(`${n}-patella`, `thigh-h${k}`, K.at(`thigh-h${k}`, 0.186, 0.027, sd * 0.113), [0.012, 0.0085, 0.008], BONE, [K.frame(`thigh-h${k}`).u[0]!, K.frame(`thigh-h${k}`).u[1]!, 0]), `thigh-h${k}`);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, TIBIA, BONE, { sd }), `shank-h${k}`);
    mem(K.limb(`${n}-metatarsal`, `metatarsus-h${k}`, METATARSAL, BONE, { sd }), `metatarsus-h${k}`);
    mem(K.run(`${n}-calcaneus`, `metatarsus-h${k}`, [{ p: K.at(`metatarsus-h${k}`, 0.014, -0.01, sd * 0.097), rl: 0.0085, rs: 0.009 }, { p: K.at(`metatarsus-h${k}`, -0.036, -0.03, sd * 0.097), rl: 0.009, rs: 0.0095 }], BONE), `metatarsus-h${k}`);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, SCAPULA, BONE, { sd, e: 2.6, floor: fl }), `scapula-f${k}`);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    mem(K.limb(`${n}-radius`, `forearm-f${k}`, RADIUS, BONE, { sd }), `forearm-f${k}`);
    mem(K.run(`${n}-olecranon`, `forearm-f${k}`, [{ p: K.at(`forearm-f${k}`, 0.012, -0.014, sd * 0.105), rl: 0.008, rs: 0.0085 }, { p: K.at(`forearm-f${k}`, -0.038, -0.03, sd * 0.106), rl: 0.009, rs: 0.0095 }], BONE), `forearm-f${k}`);
    mem(K.limb(`${n}-metacarpal`, `metacarpus-f${k}`, METACARPAL, BONE, { sd }), `metacarpus-f${k}`);
    for (const [leg, h, z] of [["hind", "h", HIND_Z], ["fore", "f", FORE_Z]] as const) {
      const hoof = `hoof-${h}${k}`;
      const f = K.frame(hoof);
      const len = K.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
      const toe: V3 = [f.o[0]! + f.u[0]! * len, f.o[1]! + f.u[1]! * len, 0];
      for (const [j, w] of [[1, -1], [2, 1]] as const) {
        mem(K.limb(`${n}-${leg}-digit-${j}`, hoof, DIGIT.map((r) => [...r, z + sd * w * 0.0115]), BONE, { sd }), hoof);
        mem(K.run(`${n}-${leg}-pedal-${j}`, hoof, [{ p: [toe[0] - 0.04, toe[1] + 0.022, sd * z + w * 0.0125], rl: 0.007, rs: 0.0085 }, { p: [toe[0] - 0.008, toe[1] + 0.0075, sd * z + w * 0.014], rl: 0.0045, rs: 0.0045 }], BONE), hoof);
      }
    }
  }
  const jr: Record<string, number> = { pelvis: 0.02, lumbar: 0.019, thorax: 0.018, neck: 0.02, head: 0.021, jaw: 0.0095, "tail-1": 0.008, "tail-2": 0.007, ear: 0.007, thigh: 0.02, shank: 0.024, metatarsus: 0.0175, hoof: 0.0155, scapula: 0.008, humerus: 0.021, forearm: 0.02, metacarpus: 0.0175 };
  const jz: Record<string, number> = { thigh: 0.088, shank: 0.115, metatarsus: 0.097, "hoof-h": 0.103, scapula: 0.05, humerus: 0.095, forearm: 0.105, metacarpus: 0.088, "hoof-f": 0.09, ear: 0.043 };
  for (const b of K.c.raw.rig.bones as Json[]) {
    const stem = (b.id as string).replace(/-[hf]?[nf]$/, "");
    const key = stem === "hoof" ? `hoof-${(b.id as string).slice(5, 6)}` : stem;
    const sd = /-(h|f)?n$/.test(b.id) ? -1 : /-(h|f)?f$/.test(b.id) ? 1 : 0;
    const r = jr[stem]!;
    const o = K.frame(b.id).o;
    mem(K.ball(`joint-${b.id}`, b.id, [o[0]!, o[1]!, sd * (jz[key] ?? 0)], [r, r, r], JOINT), b.id, "joint");
  }
  mem(K.ball("mass-thorax", "thorax", [0.5, 0.535, 0], [0.034, 0.034, 0.034], MASS), "thorax", "mass");
  mem(K.ball("mass-lumbar", "lumbar", [0.225, 0.565, 0], [0.027, 0.027, 0.027], MASS), "lumbar", "mass");
  return K.body;
}

const DECISIONS = [
  {
    en: "Body at 1:50: 46 solids, every section stated by hand from the skeletal landmarks of a 70 kg wool ewe. The fleece trunk is two overlapping lofts (loin on the lumbar bone, chest on the thorax bone) with a rounded rump, a deep belly and a full brisket; the wool-covered ham and shoulder ride on thigh and upper arm inside the fleece; neck fleece, skull with a broad wool-capped poll, a slight Roman nose and a blunt muzzle, separate mandible, eyes, a wool topknot, leaf ears carried out and a little down and a docked wool tail in two parts. Each leg is a wool gaskin or forearm with the muscle belly at the top, a point of hock, a cannon with knee or hock block and fetlock, two dewclaws, a pastern and a cloven hoof of two claws; segments overlap at every joint so that poses stay closed.",
    de: "Körper bei 1:50: 46 Körper, jeder Schnitt von Hand nach den Skelettpunkten eines 70 kg schweren Wollschafs gesetzt. Der Vliesrumpf besteht aus zwei überlappenden Lofts (Lende am Lendenknochen, Brust am Brustknochen) mit runder Kruppe, tiefem Bauch und voller Vorderbrust; wollbedeckte Keule und Schulter sitzen an Oberschenkel und Oberarm im Vlies; Halsvlies, Schädel mit breitem, wollbedecktem Genick, leichter Ramsnase und stumpfem Maul, eigener Unterkiefer, Augen, Wollschopf, seitlich und leicht abwärts getragene Blattohren und ein kupierter Wollschwanz in zwei Teilen. Jedes Bein besteht aus wolligem Unterschenkel oder Unterarm mit dem Muskelbauch oben, Fersenhöcker, Röhre mit Vorderfußwurzel- oder Sprunggelenkblock und Fesselkopf, zwei Afterklauen, Fessel und einer gespaltenen Klaue aus zwei Zehen; die Segmente überlappen an jedem Gelenk, damit Posen geschlossen bleiben.",
  },
  {
    en: "Thermal at 1:50: every region has one declared layer, so each region is the body surface it covers (references to the body solids): trunk fleece with ham and shoulder, neck fleece with the topknot, wool gaskins and forearms, hair-covered cannons and pasterns, face with jaw and eyes, ears, tail, hoof horn of claws and dewclaws. The outlets are patches on the surface: two nostrils and the lips on the muzzle, and a patch of the back for the plume.",
    de: "Thermik bei 1:50: jede Region hat eine deklarierte Schicht, daher ist jede Region die Körperoberfläche, die sie bedeckt (Verweise auf die Körper): Rumpfvlies mit Keule und Schulter, Halsvlies mit Schopf, wollige Unterschenkel und Unterarme, behaarte Röhren und Fesseln, Gesicht mit Kiefer und Augen, Ohren, Schwanz, Klauenhorn von Klauen und Afterklauen. Die Auslässe sind Flächen auf der Oberfläche: zwei Nüstern und die Lippen am Maul sowie ein Rückenfeld für die Warmluftfahne.",
  },
  {
    en: "Structure at 1:50: the skeleton. Skull and mandible, cervical spine in its S curve, thoracic spine with one spinous process per vertebra (the long ones of the second to fifth carry the withers), thirteen ribs per side, the first eight reaching the sternum through their costal cartilages, the others joined in the costal arch, lumbar spine with a spinous and two transverse processes per vertebra, sacrum, ilium, ischium and pubis, caudal vertebrae, ear cartilages; per foreleg scapula, humerus, radius with olecranon, carpus and metacarpal, two digits with pedal bones; per hind leg femur with patella, tibia, calcaneus, tarsus and metatarsal, two digits with pedal bones. Joints sit at the rig joints at the size of the joint, the masses of thorax and loin at their centres of mass.",
    de: "Tragwerk bei 1:50: das Skelett. Schädel und Unterkiefer, Halswirbelsäule in ihrer S-Kurve, Brustwirbelsäule mit einem Dornfortsatz je Wirbel (die langen des zweiten bis fünften tragen den Widerrist), dreizehn Rippen je Seite, die ersten acht über ihre Rippenknorpel am Brustbein, die übrigen im Rippenbogen verbunden, Lendenwirbelsäule mit einem Dorn- und zwei Querfortsätzen je Wirbel, Kreuzbein, Darmbein, Sitzbein und Schambein, Schwanzwirbel, Ohrknorpel; je Vorderbein Schulterblatt, Oberarmbein, Speiche mit Ellbogenhöcker, Vorderfußwurzel und Röhrbein, zwei Zehen mit Klauenbeinen; je Hinterbein Oberschenkelbein mit Kniescheibe, Schienbein, Fersenbein, Fußwurzel und Röhrbein, zwei Zehen mit Klauenbeinen. Gelenke sitzen an den Gelenken des Skeletts in Gelenkgröße, die Massen von Brust und Lende in ihren Schwerpunkten.",
  },
];

if (import.meta.main) {
  const S = new Sheep(SPEC, 50);
  body(S);
  const th = thermal(S, new Sheep(SPEC, 50), { nostrilR: 0.0055, mouthR: 0.011, plume: [[0.44, 0.705, 0], [0.24, 0.15, 0.06]] });
  const st = structure(new Sheep(SPEC, 50));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
    writeFileSync(`${OUT}/shapes.50.json`, JSON.stringify(S.shapes));
  }
}
