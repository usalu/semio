/** 📦 The three lists and the decisions of every scale of `humans/elderly-with-cane`: 1:50 from `-s50.ts`, the coarser bodies stated here for their own smallest feature (15, 30, 75 mm), the thermal lists (garment shells, skin, emitters) and the skeleton of `-skeleton.ts`.
 *
 * @see ./rework-04-elderly-with-cane-build.ts */
import type { Json } from "./volumes-lib.ts";
import { capsule, dense, dome, ellipsoid, loft, mirrored, part, revolve, shell, thOf, tubeStations, type St, type V3 } from "./rework-04-kit.ts";
import { ANKLE_X, ASSET, CANE_L, CAP, FORESLEEVE_L, HAIR, HANDLE, HEAD, JACKET, SEAT, SHANK_L, SHOE_REAR, SHOE_TOE, SLEEVE_L, THIGH_L, TONE, WAIST, body50, cane, far, hand, headSt, jacketRows, sides, slice, st, xs } from "./rework-04-elderly-with-cane-s50.ts";
import { skeleton } from "./rework-04-elderly-with-cane-skeleton.ts";
import { DECISIONS } from "./rework-04-elderly-with-cane-texts.ts";

const FEATURE: Record<number, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
const pick = <T>(rows: readonly T[], idx: readonly number[]): T[] => idx.map((i) => rows[i]!);
const fat = (s: readonly St[], min: number): St[] => s.map((q) => ({ ...q, rx: Math.max(q.rx, min / 2), ry: Math.max(q.ry, min / 2) }));

/* ---------------------------------------------------------------- coarser bodies */

/** 🧍 The body at 1:100, 1:200 or 1:500: the same stations as 1:50 where a part survives (a stated subset of rows), merged forms where it does not. */
function bodyCoarse(scale: 100 | 200 | 500): Json[] {
  const f = FEATURE[scale]!;
  const out: Json[] = [];
  const tweed = (id: string, bone: string, tone: number = TONE.jacket): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "tweed", tone });
  const wool = { material: "wool-trouser", tone: TONE.trousers };
  const leather = { material: "leather-shoe", tone: TONE.shoe };

  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", ...wool }, pick(SEAT, scale === 100 ? [0, 1, 2, 3, 5] : [0, 1, 3, 5]).map((r) => slice("pelvis", r))));
  const rows = scale === 100 ? jacketRows(2) : scale === 200 ? [...JACKET] : pick(JACKET, [0, 2, 5, 7, 9, 10, 11, 13]);
  const waist = rows.findIndex((r) => r[0] === JACKET[WAIST]![0]);
  out.push(loft(tweed("jacket-skirt", "torso"), rows.slice(0, waist + 1).map((r) => slice("torso", r))));
  out.push(loft(tweed("jacket", "chest"), rows.slice(waist).map((r) => slice("chest", r))));
  out.push(loft({ id: "neck", bone: "neck", material: "skin", tone: 0.78 }, [st(-0.035, -0.008, 0, 0.064, 0.068), st(0.065, -0.012, 0, 0.0555, 0.058), st(0.128, -0.012, 0, 0.052, 0.054)]));

  const k = scale === 100 ? 2 : 1;
  out.push(...sides(loft({ id: "thigh-l", bone: "thigh-l", ...wool }, dome(dense(scale === 500 ? pick(THIGH_L, [0, 2, 5]) : THIGH_L, k), 1, 0.03, scale === 100 ? 2 : 1))));
  out.push(...sides(loft({ id: "shank-l", bone: "shank-l", ...wool }, dome(dense(scale === 500 ? pick(SHANK_L, [0, 2, 4, 6]) : SHANK_L, k), -1, 0.03, scale === 100 ? 2 : 1))));
  const grounded = (r: readonly [number, number, number, number, number]): [number, number, number, number, number] => [r[0], 0, r[2], r[3] + 0.002, r[4]];
  if (scale === 500) {
    out.push(...sides(loft({ id: "shoe-rear-l", bone: "foot-l", ...leather }, xs("foot-l", [...pick(SHOE_REAR, [0, 2, 4, 6]), ...pick(SHOE_TOE, [1, 3])].map(grounded)))));
  } else {
    out.push(...sides(loft({ id: "shoe-rear-l", bone: "foot-l", ...leather }, xs("foot-l", pick(SHOE_REAR, scale === 100 ? [0, 1, 2, 3, 4, 5, 6, 8] : [0, 2, 4, 6, 8]).map(grounded)))));
    out.push(...sides(loft({ id: "shoe-toe-l", bone: "toe-l", ...leather }, dome(xs("toe-l", pick(SHOE_TOE, scale === 100 ? [0, 1, 2, 3] : [0, 2, 3]).map(grounded)), 1, 0.006, 1))));
  }

  out.push(...sides(loft(tweed("sleeve-l", "arm-l"), dome(dense(scale === 500 ? pick(SLEEVE_L, [0, 2, 4, 6]) : SLEEVE_L, k), 1, 0.03, scale === 100 ? 2 : 1))));
  out.push(...sides(loft(tweed("forearm-l", "forearm-l"), dome(scale === 500 ? pick(FORESLEEVE_L, [0, 2, 4]) : FORESLEEVE_L, -1, 0.03, scale === 100 ? 2 : 1))));
  for (const side of ["l", "r"] as const) {
    const s = side === "l" ? 1 : -1;
    const tone = side === "l" ? TONE.handFar : TONE.hand;
    const skin = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "skin", tone });
    const h = `hand-${side}`;
    const t = Math.max(f, scale === 500 ? 0.055 : 0);
    const palm: St[] = [st(-0.03, 0, 0, 0.019, 0.028), st(0.008, 0.001, -0.001 * s, 0.018, 0.03), st(0.058, 0.001, -0.002 * s, 0.0142, 0.042), st(0.09, -0.001, 0, 0.0112, 0.0395)];
    if (scale === 500) {
      out.push(loft(skin(h, h), dome(fat([st(-0.03, 0, 0, 0.03, 0.03), st(0.05, 0, 0, 0.03, 0.04), st(side === "l" ? 0.13 : 0.09, 0, side === "l" ? -0.01 : 0.02, 0.03, 0.036)], t), 1, 0.02, 1)));
      continue;
    }
    if (scale === 100) {
      out.push(...hand(side, tone).map((q) => (q.kind === "capsule" ? { ...q, r0: Math.max(q.r0, f / 2), r1: Math.max(q.r1, f / 2) } : q)));
      continue;
    }
    out.push(loft(skin(h, h), dome(fat(palm, f), 1, 0.006, 1)));
    const mitt = (bone: string, len: number, r0: number, r1: number, id: string): Json => loft(skin(id, bone), dome(fat([st(0, -0.011, 0, r0, 0.036), st(len, -0.011, 0, r1, 0.033)], f), 1, 0.006, 1));
    out.push(mitt(`middle-proximal-${side}`, 0.044, 0.0095, 0.0085, `${h}-fingers`));
    out.push(mitt(`middle-middle-${side}`, 0.045, 0.0082, 0.0075, `${h}-fingertips`));
    out.push(capsule(skin(`thumb-metacarpal-${side}`, `thumb-metacarpal-${side}`), [0, 0, 0], [0.042, 0, 0], f / 2, f / 2));
    out.push(capsule(skin(`thumb-proximal-${side}`, `thumb-proximal-${side}`), [0, 0, 0], [0.05, 0, 0], f / 2, f / 2));
  }

  const head = { bone: "head", material: "skin", tone: TONE.skin };
  out.push(loft({ id: "head", ...head }, dome(dome(scale === 500 ? headSt(pick(HEAD, [0, 2, 5, 7, 9, 10])) : dense(headSt(HEAD), k), -1, 0.005, 1), 1, 0.004, 1)));
  if (scale <= 200) out.push(loft({ id: "head-nose", ...head, tone: 0.76 }, fat([st(0.028, -0.099, 0, 0.0075, 0.007, 2, 0), st(-0.012, -0.112, 0, 0.0145, 0.014, 2, 0), st(-0.024, -0.108, 0, 0.016, 0.011, 2, 0)], f)));
  if (scale === 100) {
    const ear = ellipsoid({ id: "head-ear-l", ...head, tone: 0.76 }, [0.03, 0.015, 0.0078], [0.006, 0.014, 0.075]);
    out.push(ear, mirrored(ear));
  }
  out.push(loft({ id: "hair-white", bone: "head", material: "hair", tone: TONE.hair }, (scale === 500 ? pick(HAIR, [0, 2, 4]) : [...HAIR]).map(([u, v, rw, rv, e]) => st(u, v, 0, rw, rv, e))));
  const capRows = (scale === 500 ? pick(CAP, [0, 2, 4, 5]) : [...CAP]).map(([u, v, rw, rv, e]) => st(u, v, 0, rw, rv, e));
  if (scale === 100) {
    out.push(loft({ id: "cap", bone: "head", material: "cap-tweed", tone: TONE.cap }, dome(dense(capRows, 2), 1, 0.006, 1)));
    out.push(loft({ id: "cap-brim", bone: "head", material: "cap-tweed", tone: 0.26 }, [st(0.058, -0.16, 0, 0.034, 0.0076, 2.6), st(0.061, -0.142, 0, 0.068, 0.0078, 2.6), st(0.066, -0.105, 0, 0.08, 0.008, 2.6), st(0.07, -0.075, 0, 0.078, 0.008, 2.6)].map((q) => ({ ...q, ex: [0, 0, 1] as V3, ey: [1, 0, 0] as V3 }))));
  } else {
    out.push(loft({ id: "cap", bone: "head", material: "cap-tweed", tone: TONE.cap }, dome([st(0.068, -0.03, 0, 0.082, 0.132, 2.4), ...capRows.slice(1).map((q, i) => (i === 0 ? { ...q, at: [q.at[0], q.at[1] - 0.012, 0] as V3, ry: q.ry + 0.012 } : q))], 1, 0.006, 1)));
  }

  if (scale === 100) out.push(...cane().filter((s) => s.id !== "cane-collar"));
  else {
    const path: V3[] = [[HANDLE[0]![1], HANDLE[0]![0], 0], [HANDLE[3]![1], HANDLE[3]![0], 0], [-0.004, -0.012, 0], [0.012, 0, 0], [0.4, 0, 0], [CANE_L, 0, 0]];
    out.push(loft({ id: "cane-shaft", bone: "cane", material: "ash-wood", tone: TONE.wood }, tubeStations(path, [0.011, 0.0135, 0.0125, 0.0114, 0.0104, 0.012], [-1, 0, 0]), [true, true], { slender: true }));
  }
  return out;
}

/* ---------------------------------------------------------------- thermal */

const LAYERS = {
  jacket: [["jk-waistcoat", 0.004], ["jk-shirt", 0.007], ["jk-air", 0.0082]],
  trousers: [["tr-underwear", 0.003], ["tr-air", 0.004]],
  shoes: [["sh-air", 0.004], ["sh-sock", 0.008]],
  cap: [["cap-air", 0.003], ["cap-hair", 0.009]],
} as const;
const OUTER = { jacket: "jk-tweed", trousers: "tr-wool", shoes: "sh-leather", cap: "cap-tweed" } as const;
type Layered = keyof typeof LAYERS;
const REGION_OF: [RegExp, string, string?][] = [
  [/^(jacket|collar|sleeve-|forearm-|lapel-|jacket-)/, "jacket", "jk-tweed"],
  [/^waistcoat/, "jacket", "jk-waistcoat"],
  [/^shirt-/, "jacket", "jk-shirt"],
  [/^(thigh-|shank-|pelvis-trousers)/, "trousers", "tr-wool"],
  [/^sock-/, "shoes", "sh-sock"],
  [/^shoe-/, "shoes", "sh-leather"],
  [/^cap/, "cap", "cap-tweed"],
  [/^hair/, "hair", "hair-pelt"],
  [/^glasses-/, "spectacle-frame"],
  [/^cane-/, "cane-wood"],
  [/^(hand-|wrist-|thumb-|index-|middle-|ring-|little-)/, "hand-skin"],
  [/^(head|neck)/, "face-skin"],
];
export const regionOf = (id: string): [string, string | undefined] | null => { const r = REGION_OF.find(([re]) => re.test(id)); return r ? [r[1], r[2]] : null; };

function thermal(scale: number, body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const f = FEATURE[scale]!;
  for (const s of body) {
    const r = regionOf(s.id);
    if (r === null || s.id === "head" || s.id === "neck" || s.id === "pelvis-trousers") continue;
    const [region, layer] = r;
    out.push(thOf(s.id, region, layer, s.id === "cap" ? { sectionFaces: ["-u"] } : {}));
    if (scale === 50 && s.kind === "loft" && s.role !== "detail" && region in LAYERS && layer === OUTER[region as Layered] && !/sole|heel|brim/.test(s.id)) LAYERS[region as Layered].forEach(([inner, t], i) => out.push(shell(s, `th-${s.id}-${inner}`, t, region, inner, Math.max(0.05, Math.round((s.tone - 0.1 * (i + 1)) * 100) / 100))));
  }
  const skin = { bone: "head", material: "skin", tone: TONE.skin };
  const face = HEAD.filter((row) => row[0] <= 0.075);
  out.push({ ...loft({ id: "th-head", ...skin }, dome(scale === 50 ? dense(headSt(face), 3) : headSt(face), -1, 0.005, 1), [true, false]), region: "face-skin" });
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin", tone: 0.78 }, [st(scale === 500 ? 0.028 : 0.04, -0.011, 0, 0.0572, 0.0598), st(0.065, -0.012, 0, 0.0555, 0.058), st(0.105, -0.012, 0, 0.0545, 0.058)], [false, false]), region: "face-skin" });
  if (scale !== 50) for (const g of body50().filter((s) => (scale === 100 ? /^glasses-(rim|temple)/ : /^glasses-rim/).test(s.id))) out.push({ ...g, id: `th-${g.id}`, region: "spectacle-frame" });
  const anchor = (id: string): Json => (ASSET.anchors as Json[]).find((a) => a.id === id);
  const [nose, mouth, crown] = [anchor("nose"), anchor("mouth"), anchor("crown")];
  const e = (x: number): number => Math.max(x, scale === 50 ? 0 : f / 2);
  out.push({ ...ellipsoid({ id: "th-breath-nose", ...skin }, [e(0.004), e(0.004), e(0.0095)], [nose.x, nose.y + 0.002, 0]), region: "face-skin", tags: ["emitter", "breath-nose"] });
  out.push({ ...ellipsoid({ id: "th-breath-mouth", ...skin }, [e(0.004), e(0.004), e(0.0062)], [mouth.x, mouth.y + 0.006, 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] });
  out.push({ ...ellipsoid({ id: "th-plume", bone: "head", material: "cap-tweed", tone: 0.5 }, [e(0.005), 0.07, 0.07], [crown.x - 0.006, crown.y, 0]), region: "jacket", tags: ["emitter", "plume"] });
  return out;
}

function structure(scale: 50 | 100 | 200 | 500, body: readonly Json[]): Json[] {
  const out = skeleton({ scale, floor: FEATURE[scale]! });
  for (const s of body) if (s.bone === "cane") out.push({ id: `st-${s.id}`, of: s.id, member: "cane", part: "member" });
  if (scale <= 200) out.push(part(ellipsoid({ id: "joint-cane", bone: "cane", material: "bone", tone: 0.4 }, [Math.max(0.0125, FEATURE[scale]! / 2), Math.max(0.0125, FEATURE[scale]! / 2), Math.max(0.0125, FEATURE[scale]! / 2)], [0, -0.045, 0]), "cane", "joint"));
  return out;
}

export function volumes(): Record<string, Json> {
  const bodies: Record<number, Json[]> = { 50: body50(), 100: bodyCoarse(100), 200: bodyCoarse(200), 500: bodyCoarse(500) };
  const out: Record<string, Json> = {};
  for (const scale of [50, 100, 200, 500] as const) out[String(scale)] = { status: "authored", denominator: scale, body: bodies[scale], thermal: thermal(scale, bodies[scale]!), structure: structure(scale, bodies[scale]!), decisions: DECISIONS[scale] };
  return out;
}
void revolve;
void far;
void ANKLE_X;
