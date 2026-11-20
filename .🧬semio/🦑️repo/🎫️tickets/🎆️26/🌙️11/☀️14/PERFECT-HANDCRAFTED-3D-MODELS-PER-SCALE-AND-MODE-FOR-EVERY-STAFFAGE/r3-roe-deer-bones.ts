/** 🦴️ Round 3 roe deer: the structure lists, the skeleton of a roe buck at every scale. 1:50 bone by bone with epiphyses, processes, thirteen ribs and two digits per foot; 1:100 the same bones simplified with six rib hoops; 1:200 plain shafts with three rib hoops and one pelvic girdle; 1:500 one bar per bone at the size floor. Every table is stated by hand in world metres of the default pose (x ahead of the hip joint, y above ground) or along its bone.
 * @see ./r3-roe-deer-50.ts
 * @see ./r3-roe-deer-coarse.ts */
import { Roe, side, sx, type Json, type Look, type PP, type V3 } from "./r3-roe-deer-kit.ts";
import { ear } from "./r3-roe-deer-50.ts";

const BONE: Look = { material: "bone", tone: 0.18 };
const JOINT: Look = { material: "bone", tone: 0.5 };
const MASS: Look = { material: "summer-coat", tone: 0.7 };
const SIDES = [-1, 1] as const;
const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });

/** Vertebral axis of the trunk and the neck (world, alert). */
export const SPINE_L: readonly V3[] = [[0.09, 0.709, 0], [0.185, 0.713, 0], [0.28, 0.703, 0]];
export const SPINE_T: readonly V3[] = [[0.28, 0.703, 0], [0.35, 0.697, 0], [0.42, 0.68, 0], [0.49, 0.652, 0], [0.56, 0.609, 0]];
export const SPINE_C: readonly V3[] = [[0.56, 0.609, 0], [0.602, 0.632, 0], [0.64, 0.69, 0], [0.664, 0.76, 0], [0.682, 0.828, 0], [0.699, 0.871, 0], [0.706, 0.883, 0]];
export const spineY = (x: number): number => {
  for (let i = 1; i < SPINE_T.length; i++) if (x <= SPINE_T[i]![0]) { const a = SPINE_T[i - 1]!; const b = SPINE_T[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return SPINE_T[SPINE_T.length - 1]![1];
};
/** Ribs 1 to 13: half width of the cage, depth from the spine to the ventral end, caudal sweep of the ventral end. */
export const RIBS = [[0.034, 0.108, -0.012], [0.045, 0.13, -0.002], [0.055, 0.15, 0.008], [0.063, 0.166, 0.017], [0.07, 0.179, 0.026], [0.076, 0.189, 0.034], [0.081, 0.196, 0.042], [0.085, 0.197, 0.048], [0.087, 0.19, 0.052], [0.087, 0.172, 0.054], [0.085, 0.148, 0.054], [0.08, 0.122, 0.05], [0.072, 0.096, 0.045]] as const;
export const ribPath = (i: number, sd: number, r: number): PP[] => {
  const [w, d, k] = RIBS[i]!;
  const x = ribX(i);
  const y = spineY(x) - 0.009;
  const bow: PP[] = ([[0, 0, 0.16], [0.2, 0.12, 0.72], [0.5, 0.42, 1], [0.8, 0.75, 0.88], [1, 1, 0.58]] as const).map(([a, b, c]) => ({ p: [x - a * k, y - b * d, sd * c * w] as V3, rl: r, rs: r }));
  if (i < STERNAL) { const xs = 0.565 - i * 0.02; bow.push({ p: [xs, sternumY(xs) + 0.003, sd * 0.008], rl: r, rs: r }); }
  return bow;
};
export const ribX = (i: number): number => 0.548 - i * 0.0205;
/** Sternum (world): manubrium to xiphoid; the first eight ribs reach it through their costal cartilages. */
export const STERNUM: readonly V3[] = [[0.578, 0.502, 0], [0.54, 0.47, 0], [0.48, 0.459, 0], [0.42, 0.463, 0]];
export const STERNAL = 8;
export function sternumY(x: number): number {
  for (let i = 1; i < STERNUM.length; i++) if (x >= STERNUM[i]![0]) { const a = STERNUM[i - 1]!; const b = STERNUM[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return STERNUM[STERNUM.length - 1]![1];
}
/** Spinous processes T1 to T13: height above the vertebral body, caudal lean of the tip. The long processes of T2 to T5 carry the withers; T12 and T13 stand upright. */
export const SPINOUS_T = [[0.045, 0.016], [0.06, 0.023], [0.068, 0.027], [0.07, 0.027], [0.068, 0.025], [0.062, 0.022], [0.054, 0.018], [0.046, 0.014], [0.04, 0.011], [0.036, 0.008], [0.033, 0.005], [0.031, 0.002], [0.03, 0]] as const;
/** Lumbar vertebrae L1 to L6: x of the body, height of the spinous process, half span of the transverse processes. */
export const LUMBAR_V = [[0.268, 0.03, 0.034], [0.236, 0.03, 0.039], [0.204, 0.03, 0.042], [0.172, 0.03, 0.042], [0.14, 0.029, 0.039], [0.108, 0.027, 0.033]] as const;
export const lumbarY = (x: number): number => (x >= 0.185 ? 0.713 - ((x - 0.185) / 0.095) * 0.01 : 0.709 + ((x - 0.09) / 0.095) * 0.004);
/** 🪶️ One spinous process as a thin plate from the vertebral body to its tip. */
export function spinous(K: Roe, id: string, bone: string, x: number, y: number, h: number, lean: number, half: number, thick: number): Json {
  return K.run(id, bone, [{ p: [x, y, 0], rl: thick, rs: half }, { p: [x - lean * 0.6, y + h * 0.6, 0], rl: thick, rs: half * 0.9 }, { p: [x - lean, y + h, 0], rl: thick, rs: half * 0.7 }], BONE, { floor: thick });
}
const FEMUR = [[0, 0, 0.013, 0.013, 0.05], [0.02, 0, 0.015, 0.014, 0.053], [0.05, 0, 0.0092, 0.0092, 0.057], [0.12, 0, 0.009, 0.009, 0.064], [0.16, 0, 0.015, 0.014, 0.067], [0.183, 0.003, 0.017, 0.016, 0.068]] as const;
const TIBIA = [[0, 0, 0.017, 0.016, 0.068], [0.03, 0.003, 0.0125, 0.011, 0.069], [0.09, 0, 0.0082, 0.0078, 0.07], [0.17, 0, 0.0075, 0.0072, 0.069], [0.21, 0, 0.011, 0.0105, 0.068], [0.225, 0, 0.011, 0.0105, 0.068]] as const;
const METATARSAL = [[-0.006, 0, 0.0125, 0.0115, 0.068], [0.028, 0, 0.0115, 0.0105, 0.068], [0.05, 0, 0.0075, 0.0075, 0.067], [0.19, 0, 0.0072, 0.0075, 0.063], [0.215, 0, 0.0102, 0.0115, 0.062], [0.23, 0, 0.01, 0.0115, 0.061]] as const;
const DIGIT = [[0.002, 0, 0.007, 0.0062], [0.03, 0, 0.0055, 0.0052], [0.04, 0, 0.0065, 0.006], [0.058, 0, 0.006, 0.0056]] as const;
const SCAPULA = [[-0.01, 0, 0.036, 0.0042, 0.045], [0.02, 0, 0.036, 0.0042, 0.05], [0.08, 0, 0.024, 0.0045, 0.06], [0.135, 0, 0.0115, 0.006, 0.066], [0.162, 0, 0.013, 0.0105, 0.068]] as const;
const HUMERUS = [[0, 0, 0.0155, 0.0145, 0.068], [0.02, 0.006, 0.0155, 0.014, 0.069], [0.055, 0, 0.0092, 0.009, 0.07], [0.11, 0, 0.009, 0.009, 0.071], [0.14, 0, 0.014, 0.014, 0.071], [0.153, 0, 0.013, 0.0145, 0.071]] as const;
const RADIUS = [[0, 0, 0.0115, 0.013, 0.071], [0.03, 0, 0.0078, 0.0098, 0.07], [0.14, 0, 0.0076, 0.0095, 0.067], [0.18, 0, 0.0102, 0.0125, 0.066], [0.195, 0, 0.01, 0.0125, 0.065]] as const;
const METACARPAL = [[-0.006, 0, 0.0108, 0.0125, 0.065], [0.016, 0, 0.0105, 0.012, 0.065], [0.03, 0, 0.0068, 0.008, 0.064], [0.135, 0, 0.0066, 0.008, 0.062], [0.157, 0, 0.0095, 0.011, 0.061], [0.17, 0, 0.009, 0.011, 0.061]] as const;
const CRANIUM = [[-0.018, 0.014, 0.017, 0.017], [-0.004, 0.016, 0.029, 0.027], [0.025, 0.02, 0.035, 0.035], [0.06, 0.021, 0.034, 0.039], [0.09, 0.016, 0.03, 0.037], [0.12, 0.01, 0.024, 0.028], [0.15, 0.005, 0.02, 0.021], [0.18, 0.002, 0.016, 0.017], [0.205, 0, 0.012, 0.013], [0.214, 0, 0.007, 0.008]] as const;
const JAWBONE = [[0.03, -0.006, 0.014, 0.031], [0.042, -0.026, 0.02, 0.032], [0.075, -0.03, 0.013, 0.028], [0.11, -0.026, 0.0095, 0.021], [0.15, -0.023, 0.0075, 0.015], [0.185, -0.021, 0.0062, 0.011], [0.206, -0.02, 0.0045, 0.0065]] as const;
const HOOF_Z = 0.059;

function toeOf(K: Roe, hoof: string): V3 {
  const f = K.frame(hoof);
  const len = K.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
  return [f.o[0]! + f.u[0]! * len, f.o[1]! + f.u[1]! * len, 0];
}

function antlerRefs(K: Roe, has: (id: string) => boolean, out: Json[]): void {
  for (const sd of SIDES) for (const part of ["antler-burr", "antler-beam", "antler-brow", "antler-back", "antler", "antler-tip"]) if (has(`${side(sd)}-${part}`)) out.push({ id: `bone-${side(sd)}-${part}`, of: `${side(sd)}-${part}`, member: "head", part: "member" });
}

function joints(K: Roe, jr: Record<string, number>, jz: Record<string, number>): void {
  for (const b of K.c.raw.rig.bones as Json[]) {
    const stem = (b.id as string).replace(/-[hf]?[nf]$/, "");
    const key = stem === "hoof" || stem === "cannon" ? `${stem}-${(b.id as string).slice(stem.length + 1, stem.length + 2)}` : stem;
    const sd = /-(h|f)?n$/.test(b.id) ? -1 : /-(h|f)?f$/.test(b.id) ? 1 : 0;
    const r = jr[key] ?? jr[stem];
    if (r === undefined) continue;
    const o = K.frame(b.id).o;
    mem(K.ball(`joint-${b.id}`, b.id, [o[0]!, o[1]!, sd * (jz[key] ?? jz[stem] ?? 0)], [r, r, r], JOINT), b.id, "joint");
  }
}

const JZ: Record<string, number> = { thigh: 0.05, shank: 0.068, "cannon-h": 0.068, "hoof-h": 0.061, scapula: 0.045, humerus: 0.068, forearm: 0.071, "cannon-f": 0.065, "hoof-f": 0.061, ear: 0.03 };

function masses(K: Roe, r: readonly [number, number, number]): void {
  mem(K.ball("mass-thorax", "thorax", [0.425, 0.597, 0], [r[0], r[0], r[0]], MASS), "thorax", "mass");
  if (r[1] > 0) mem(K.ball("mass-lumbar", "lumbar", [0.185, 0.637, 0], [r[1], r[1], r[1]], MASS), "lumbar", "mass");
  if (r[2] > 0) mem(K.ball("mass-pelvis", "pelvis", [0.015, 0.652, 0], [r[2], r[2], r[2]], MASS), "pelvis", "mass");
}

/** 🦴️ 1:50: the skeleton bone by bone. */
function s50(K: Roe, has: (id: string) => boolean): Json[] {
  const fl = 0.0042;
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.014, rs: 0.012 })), BONE), "lumbar");
  for (const [i, [x, h, w]] of LUMBAR_V.entries()) {
    mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", x, lumbarY(x) + 0.009, h, -0.005, 0.011, fl), "lumbar");
    mem(K.run(`lumbar-transverse-${i + 1}`, "lumbar", [{ p: [x + 0.005, lumbarY(x) + 0.002, -w], rl: 0.007, rs: fl }, { p: [x, lumbarY(x) + 0.004, 0], rl: 0.009, rs: 0.005 }, { p: [x + 0.005, lumbarY(x) + 0.002, w], rl: 0.007, rs: fl }], BONE, { lat: [1, 0, 0], floor: fl }), "lumbar");
  }
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: 0.0115, rs: 0.0105 })), BONE), "thorax");
  for (const [i, [h, lean]] of SPINOUS_T.entries()) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.008, h, lean, 0.007, fl), "thorax");
  mem(K.run("cervical-spine", "neck", SPINE_C.map((p, i) => ({ p, rl: i > 3 ? 0.0155 : 0.014, rs: 0.0115 })), BONE), "neck");
  mem(K.run("sacrum", "pelvis", [{ p: [0.09, 0.709, 0], rl: 0.025, rs: 0.012 }, { p: [0.04, 0.722, 0], rl: 0.018, rs: 0.011 }, { p: [-0.02, 0.724, 0], rl: 0.01, rs: 0.008 }, { p: [-0.06, 0.716, 0], rl: 0.006, rs: 0.0055 }], BONE), "pelvis");
  mem(K.run("pubis", "pelvis", [{ p: [0.01, 0.626, -0.046], rl: 0.007, rs: 0.007 }, { p: [0.012, 0.617, 0], rl: 0.0085, rs: 0.007 }, { p: [0.01, 0.626, 0.046], rl: 0.007, rs: 0.007 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: [0.008, 0.0115, 0.0125, 0.0085][i]!, rs: [0.0065, 0.0075, 0.007, 0.005][i]! })), BONE), "thorax");
  mem(K.limb("skull", "head", CRANIUM, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", JAWBONE, BONE, { frame: "head", e: 2.3 }), "jaw");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-pedicle`, "head", [{ p: K.at("head", 0.05, 0.032, sd * 0.02), rl: 0.009, rs: 0.009 }, { p: K.at("head", 0.045, 0.06, sd * 0.022), rl: 0.0085, rs: 0.0085 }], BONE), "head");
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.108, 0.726, sd * 0.066], rl: 0.006, rs: 0.023 }, { p: [0.078, 0.714, sd * 0.046], rl: 0.0055, rs: 0.02 }, { p: [0.04, 0.685, sd * 0.045], rl: 0.007, rs: 0.0105 }, { p: [0.0, 0.659, sd * 0.05], rl: 0.012, rs: 0.0155 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.0, 0.657, sd * 0.05], rl: 0.009, rs: 0.011 }, { p: [-0.05, 0.648, sd * 0.046], rl: 0.007, rs: 0.0095 }, { p: [-0.095, 0.655, sd * 0.048], rl: 0.009, rs: 0.014 }], BONE), "pelvis");
    for (let i = 0; i < 13; i++) mem(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, fl), BONE), "thorax");
    mem(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.43, 0.466, sd * 0.02], rl: fl, rs: fl }, ...[7, 8, 9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, fl)[4]!.p, rl: fl, rs: fl }))], BONE), "thorax");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.128, [[0.02, 0.008, fl], [0.4, 0.016, fl], [1, 0.0055, fl]]), `ear-${k}`);
    const th = `thigh-h${k}`;
    mem(K.limb(`${n}-femur`, th, FEMUR, BONE, { sd }), th);
    mem(K.ball(`${n}-patella`, th, K.at(th, 0.176, 0.022, sd * 0.068), [0.0095, 0.0065, 0.006], BONE, [K.frame(th).u[0]!, K.frame(th).u[1]!, 0]), th);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, TIBIA, BONE, { sd }), `shank-h${k}`);
    const hc = `cannon-h${k}`;
    mem(K.limb(`${n}-metatarsal`, hc, METATARSAL, BONE, { sd }), hc);
    mem(K.run(`${n}-calcaneus`, hc, [{ p: K.at(hc, 0.012, -0.008, sd * 0.068), rl: 0.0068, rs: 0.0072 }, { p: K.at(hc, -0.038, -0.027, sd * 0.068), rl: 0.007, rs: 0.0075 }], BONE), hc);
    const sc = `scapula-f${k}`;
    mem(K.limb(`${n}-scapula`, sc, SCAPULA, BONE, { sd, e: 2.6, floor: fl }), sc);
    mem(K.run(`${n}-scapular-spine`, sc, [{ p: K.at(sc, 0.015, 0.003, sd * 0.055), rl: fl, rs: fl }, { p: K.at(sc, 0.08, 0.003, sd * 0.067), rl: fl, rs: 0.005 }, { p: K.at(sc, 0.14, 0.002, sd * 0.074), rl: fl, rs: fl }], BONE), sc);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    const fa = `forearm-f${k}`;
    mem(K.limb(`${n}-radius`, fa, RADIUS, BONE, { sd }), fa);
    mem(K.run(`${n}-olecranon`, fa, [{ p: K.at(fa, 0.01, -0.012, sd * 0.072), rl: 0.0066, rs: 0.007 }, { p: K.at(fa, -0.036, -0.026, sd * 0.072), rl: 0.0068, rs: 0.0072 }], BONE), fa);
    mem(K.limb(`${n}-metacarpal`, `cannon-f${k}`, METACARPAL, BONE, { sd }), `cannon-f${k}`);
    for (const [leg, h] of [["hind", "h"], ["fore", "f"]] as const) {
      const hoof = `hoof-${h}${k}`;
      const toe = toeOf(K, hoof);
      for (const [j, w] of [[1, -1], [2, 1]] as const) {
        mem(K.limb(`${n}-${leg}-digit-${j}`, hoof, DIGIT.map((r) => [...r, HOOF_Z + sd * w * 0.0078]), BONE, { sd, floor: fl }), hoof);
        mem(K.run(`${n}-${leg}-pedal-${j}`, hoof, [{ p: [toe[0] - 0.034, toe[1] + 0.016, sd * HOOF_Z + w * 0.0074], rl: 0.0048, rs: 0.0062 }, { p: [toe[0] - 0.007, toe[1] + 0.0065, sd * HOOF_Z + w * 0.0086], rl: fl, rs: fl }], BONE), hoof);
      }
    }
  }
  joints(K, { pelvis: 0.012, lumbar: 0.0145, thorax: 0.0135, neck: 0.0145, head: 0.0155, jaw: 0.008, ear: 0.0055, thigh: 0.015, shank: 0.0185, "cannon-h": 0.0135, "hoof-h": 0.0122, scapula: 0.006, humerus: 0.0165, forearm: 0.015, "cannon-f": 0.0135, "hoof-f": 0.0118 }, JZ);
  masses(K, [0.03, 0.021, 0.022]);
  const out = [...K.body];
  antlerRefs(K, has, out);
  return out;
}

/** Bars of a coarse skeleton: bone, station along the bone as a fraction of its length, sagittal offset, radius (m), per scale 100 / 200 / 500. Epiphyses at 1:100, plain shafts at 1:200, one straight bar at 1:500. */
const LIMB_BARS: Record<string, { z: readonly [number, number]; rows: Record<number, readonly (readonly [number, number, number])[]> }> = {
  thigh: { z: [0.05, 0.068], rows: { 100: [[0, 0, 0.014], [0.3, 0, 0.0092], [0.75, 0, 0.0092], [1, 0.002, 0.016]], 200: [[0, 0, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  shank: { z: [0.068, 0.068], rows: { 100: [[0, 0, 0.016], [0.25, 0.002, 0.0092], [0.85, 0, 0.0085], [1, 0, 0.0105]], 200: [[0, 0, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  "cannon-h": { z: [0.068, 0.061], rows: { 100: [[-0.16, -0.022, 0.0085], [0.08, 0, 0.012], [0.25, 0, 0.0085], [0.85, 0, 0.0085], [1, 0, 0.0108]], 200: [[-0.12, -0.016, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  "hoof-h": { z: [0.061, 0.059], rows: { 100: [[0, 0, 0.0095], [0.6, 0, 0.0085], [0.93, 0.004, 0.0083]], 200: [[0.05, 0, 0.0165], [0.8, 0.004, 0.0165]], 500: [[-0.35, 0, 0.038], [0.5, 0, 0.038]] } },
  scapula: { z: [0.047, 0.068], rows: { 100: [[-0.04, 0, 0.03], [0.5, 0, 0.02], [1, 0, 0.012]], 200: [[0, 0, 0.026], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  humerus: { z: [0.068, 0.071], rows: { 100: [[0, 0.003, 0.0155], [0.35, 0, 0.0092], [0.75, 0, 0.0092], [1, 0, 0.014]], 200: [[0, 0, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  forearm: { z: [0.071, 0.065], rows: { 100: [[-0.17, -0.024, 0.0083], [0.06, 0, 0.012], [0.3, 0, 0.0085], [0.85, 0, 0.0085], [1, 0, 0.0115]], 200: [[-0.12, -0.016, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  "cannon-f": { z: [0.065, 0.061], rows: { 100: [[-0.02, 0, 0.0118], [0.2, 0, 0.0083], [0.85, 0, 0.0083], [1, 0, 0.0105]], 200: [[0, 0, 0.0165], [1, 0, 0.0165]], 500: [[0, 0, 0.038], [1, 0, 0.038]] } },
  "hoof-f": { z: [0.061, 0.059], rows: { 100: [[0, 0, 0.0095], [0.6, 0, 0.0085], [0.93, 0.004, 0.0083]], 200: [[0.05, 0, 0.0165], [0.8, 0.004, 0.0165]], 500: [[-0.35, 0, 0.038], [0.5, 0, 0.038]] } },
};
const BONE_NAME: Record<string, string> = { thigh: "femur", shank: "tibia", "cannon-h": "metatarsal", "hoof-h": "hind-digit", scapula: "scapula", humerus: "humerus", forearm: "radius", "cannon-f": "metacarpal", "hoof-f": "fore-digit" };

function limbBars(K: Roe, scale: number): void {
  for (const sd of SIDES) for (const [key, bar] of Object.entries(LIMB_BARS)) {
    const fore = ["scapula", "humerus", "forearm", "cannon-f", "hoof-f"].includes(key);
    const stem = key.replace(/-[hf]$/, "");
    const bone = `${stem}-${fore ? "f" : "h"}${sx(sd)}`;
    const len = K.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
    const rows = bar.rows[scale]!;
    const flat = key === "scapula" && scale < 500;
    const pts: PP[] = rows.map(([t, v, r]) => ({ p: K.at(bone, t * len, v, sd * (bar.z[0] + (bar.z[1] - bar.z[0]) * Math.max(0, Math.min(1, t)))), rl: flat ? (scale === 100 ? 0.0083 : 0.0165) : r, rs: r }));
    mem(K.run(`${side(sd)}-${BONE_NAME[key]}`, bone, pts, BONE), bone);
  }
}

/** 🦴️ 1:100: the same skeleton with simplified bones: spine in three members with one blade of spinous processes, six rib hoops per side, sternum, pelvis as ilium and ischium bars, long bones with epiphyses, one digit per foot. */
function s100(K: Roe, has: (id: string) => boolean): Json[] {
  const fl = 0.0083;
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.03, rs: 0.012 })), BONE), "lumbar");
  mem(K.run("thoracic-spine", "thorax", SPINE_T.map((p) => ({ p, rl: 0.0115, rs: 0.0105 })), BONE), "thorax");
  for (const i of [0, 2, 4, 6, 9, 12]) mem(spinous(K, `thoracic-spinous-${i + 1}`, "thorax", ribX(i), spineY(ribX(i)) + 0.008, SPINOUS_T[i]![0], SPINOUS_T[i]![1], fl, fl), "thorax");
  for (const i of [0, 2, 4]) mem(spinous(K, `lumbar-spinous-${i + 1}`, "lumbar", LUMBAR_V[i]![0], lumbarY(LUMBAR_V[i]![0]) + 0.009, LUMBAR_V[i]![1], -0.005, fl, fl), "lumbar");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[4]!, SPINE_C[6]!].map((p) => ({ p, rl: 0.0145, rs: 0.0115 })), BONE), "neck");
  mem(K.run("sacrum", "pelvis", [{ p: [0.09, 0.709, 0], rl: 0.024, rs: 0.012 }, { p: [0.02, 0.722, 0], rl: 0.015, rs: 0.01 }, { p: [-0.05, 0.716, 0], rl: fl, rs: fl }], BONE), "pelvis");
  mem(K.run("sternum", "thorax", STERNUM.map((p, i) => ({ p, rl: i === 1 || i === 2 ? 0.012 : 0.009, rs: fl })), BONE), "thorax");
  mem(K.limb("skull", "head", [[-0.012, 0.015, 0.02, 0.02], [0.03, 0.02, 0.035, 0.035], [0.075, 0.019, 0.032, 0.038], [0.13, 0.008, 0.022, 0.025], [0.185, 0.002, 0.015, 0.016], [0.212, 0, 0.009, 0.0095]], BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", [[0.032, -0.012, 0.016, 0.03], [0.07, -0.029, 0.014, 0.028], [0.13, -0.025, 0.0085, 0.018], [0.2, -0.021, fl, 0.009]], BONE, { frame: "head", e: 2.3 }), "jaw");
  for (const sd of SIDES) {
    const n = side(sd);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.105, 0.724, sd * 0.062], rl: fl, rs: 0.022 }, { p: [0.05, 0.692, sd * 0.045], rl: fl, rs: 0.012 }, { p: [0.0, 0.658, sd * 0.05], rl: 0.012, rs: 0.014 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.0, 0.657, sd * 0.05], rl: 0.009, rs: 0.011 }, { p: [-0.093, 0.654, sd * 0.047], rl: 0.009, rs: 0.013 }], BONE), "pelvis");
    for (const [j, i] of [0, 2, 4, 6, 9, 12].entries()) mem(K.run(`${n}-rib-${j + 1}`, "thorax", ribPath(i, sd, fl), BONE), "thorax");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.126, [[0.03, 0.009, fl], [0.45, 0.016, fl], [1, fl, fl]]), `ear-${sx(sd)}`);
  }
  limbBars(K, 100);
  joints(K, { lumbar: 0.014, thorax: 0.013, neck: 0.014, head: 0.015, jaw: 0.0083, thigh: 0.015, shank: 0.018, "cannon-h": 0.013, "hoof-h": 0.012, humerus: 0.016, forearm: 0.015, "cannon-f": 0.013, "hoof-f": 0.0115 }, JZ);
  masses(K, [0.03, 0.021, 0.022]);
  const out = [...K.body];
  antlerRefs(K, has, out);
  return out;
}

/** 🦴️ 1:200: plain shafts: skull with jaw bar, three spine members, one pelvic girdle, three rib hoops per side on a sternum, one bar per limb bone, joints at the large joints, the trunk masses. */
function s200(K: Roe, has: (id: string) => boolean): Json[] {
  const fl = 0.0165;
  mem(K.run("lumbar-spine", "lumbar", [SPINE_L[0]!, SPINE_L[2]!].map((p) => ({ p, rl: 0.022, rs: fl })), BONE), "lumbar");
  mem(K.run("thoracic-spine", "thorax", [[0.28, 0.705, 0], [0.42, 0.692, 0], [0.5, 0.672, 0], [0.56, 0.615, 0]].map((p) => ({ p: p as V3, rl: fl, rs: p[0]! > 0.4 && p[0]! < 0.55 ? 0.03 : 0.02 })), BONE), "thorax");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[3]!, SPINE_C[6]!].map((p) => ({ p, rl: fl, rs: fl })), BONE), "neck");
  mem(K.run("pelvic-girdle", "pelvis", [{ p: [0.1, 0.716, 0], rl: 0.06, rs: 0.02 }, { p: [0.0, 0.668, 0], rl: 0.058, rs: 0.02 }, { p: [-0.09, 0.656, 0], rl: 0.05, rs: fl }], BONE), "pelvis");
  mem(K.run("sternum", "thorax", [{ p: [0.57, 0.5, 0], rl: fl, rs: fl }, { p: [0.44, 0.472, 0], rl: fl, rs: fl }], BONE), "thorax");
  mem(K.limb("skull", "head", [[-0.008, 0.015, 0.024, 0.024], [0.05, 0.019, 0.034, 0.037], [0.13, 0.008, 0.022, 0.024], [0.208, 0.001, fl, fl]], BONE, { e: 2.3 }), "head");
  mem(K.limb("mandible", "jaw", [[0.035, -0.02, fl, 0.027], [0.2, -0.019, fl, fl]], BONE, { frame: "head" }), "jaw");
  for (const sd of SIDES) {
    const n = side(sd);
    for (const [j, i] of [0, 3, 6, 9, 12].entries()) mem(K.run(`${n}-rib-${j + 1}`, "thorax", [0, 2, 4].map((q) => ({ ...ribPath(i, sd, fl)[q]!, p: ribPath(i, sd * 0.9, fl)[q]!.p })), BONE), "thorax");
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.12, [[0.06, fl, fl], [1, fl, fl]]), `ear-${sx(sd)}`);
  }
  limbBars(K, 200);
  joints(K, { thorax: 0.02, neck: 0.02, head: 0.02, thigh: 0.02, shank: 0.021, "cannon-h": 0.02, humerus: 0.021, forearm: 0.02, "cannon-f": 0.02 }, JZ);
  masses(K, [0.03, 0, 0.024]);
  const out = [...K.body];
  antlerRefs(K, has, out);
  return out;
}

/** 🦴️ 1:500: one bar per bone at the size floor: skull with the antler spikes, jaw, neck, back in two members, pelvis bar, ear cartilages, four limb chains; the chest mass. */
function s500(K: Roe, has: (id: string) => boolean): Json[] {
  const fl = 0.038;
  mem(K.run("lumbar-spine", "lumbar", [SPINE_L[0]!, SPINE_L[2]!].map((p) => ({ p: [p[0], p[1] - 0.004, 0] as V3, rl: fl, rs: fl })), BONE), "lumbar");
  mem(K.run("thoracic-spine", "thorax", [[0.28, 0.699, 0], [0.45, 0.682, 0], [0.56, 0.62, 0]].map((p) => ({ p: p as V3, rl: fl, rs: fl })), BONE), "thorax");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[3]!, SPINE_C[6]!].map((p) => ({ p, rl: fl, rs: fl })), BONE), "neck");
  mem(K.run("pelvic-girdle", "pelvis", [{ p: [0.09, 0.705, 0], rl: 0.058, rs: fl }, { p: [-0.06, 0.66, 0], rl: 0.05, rs: fl }], BONE), "pelvis");
  mem(K.limb("skull", "head", [[0, 0.012, fl, fl], [0.2, 0.002, fl, fl]], BONE), "head");
  mem(K.limb("mandible", "jaw", [[0.04, -0.012, fl, fl], [0.19, -0.008, fl, fl]], BONE, { frame: "head" }), "jaw");
  for (const sd of SIDES) mem(ear(K, `${side(sd)}-ear-cartilage`, sd, BONE, 0.1, [[0.1, fl, fl], [1, fl, fl]]), `ear-${sx(sd)}`);
  limbBars(K, 500);
  masses(K, [0.04, 0, 0]);
  const out = [...K.body];
  antlerRefs(K, has, out);
  return out;
}

/** 🦴️ The structure list of a scale; `has` tells which antler solids the body of that scale has (the antler is bone, so the body solid is the member). */
export function structure(K: Roe, scale: number, has: (id: string) => boolean = () => true): Json[] {
  if (scale === 50) return s50(K, has);
  if (scale === 100) return s100(K, has);
  if (scale === 200) return s200(K, has);
  return s500(K, has);
}
