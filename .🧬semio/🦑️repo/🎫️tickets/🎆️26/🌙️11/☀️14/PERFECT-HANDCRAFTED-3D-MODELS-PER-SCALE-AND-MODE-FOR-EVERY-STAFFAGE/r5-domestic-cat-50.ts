/** 🐈️ Round 5 domestic cat at 1:50: the finest model. Body: trunk in four overlapping lofts (rump, loin, ribcage, chest), neck, skull with muzzle pads, nose leather, eyes, cupped ears, mandible and tongue, five tail segments, and per leg the muscle masses of thigh and shank or shoulder, upper arm and forearm, the hock with its heel, the metapodium and a paw of four toes on five pads.
 * Usage: bun r5-domestic-cat-50.ts [--write]
 * @see ./r3-domestic-cat-kit.ts
 * @see ./r3-animals-domestic-cat.md */
import { saveLists } from "./kit-lib.ts";
import { Cat, SIDES, dense, side, sx, type Json, type Look, type PP, type Row, type V3 } from "./r3-domestic-cat-kit.ts";

export const COAT: Look = { material: "short-fur", tone: 0.66 };
export const LIMB: Look = { material: "short-fur", tone: 0.7 };
export const FACE: Look = { material: "short-fur", tone: 0.72 };
export const EAR: Look = { material: "short-fur", tone: 0.6 };
export const PAD: Look = { material: "bare-skin", tone: 0.1, role: "detail" };
export const NOSE: Look = { material: "bare-skin", tone: 0.08 };
export const EYE: Look = { material: "cornea", tone: 0.04, role: "detail" };
export const TONGUE: Look = { material: "tongue-mucosa", tone: 0.42 };

/** Trunk, world planes x = const: x, centre height, half height, half width, exponent. */
export const TRUNK: readonly Row[] = [
  [-0.047, 0.226, 0.012, 0.014, 2.0],
  [-0.041, 0.225, 0.027, 0.03, 2.0],
  [-0.026, 0.221, 0.037, 0.039, 2.1],
  [0.0, 0.214, 0.049, 0.043, 2.15],
  [0.03, 0.21, 0.056, 0.047, 2.2],
  [0.06, 0.205, 0.062, 0.051, 2.2],
  [0.095, 0.2, 0.067, 0.057, 2.2],
  [0.13, 0.196, 0.07, 0.0625, 2.2],
  [0.165, 0.1925, 0.0715, 0.0635, 2.2],
  [0.2, 0.19, 0.071, 0.061, 2.2],
  [0.23, 0.1875, 0.0695, 0.055, 2.4],
  [0.255, 0.187, 0.066, 0.049, 2.6],
  [0.278, 0.1895, 0.0605, 0.046, 2.4],
  [0.296, 0.196, 0.052, 0.041, 2.2],
  [0.31, 0.205, 0.039, 0.03, 2.0],
  [0.319, 0.212, 0.018, 0.013, 2.0],
];
/** Neck in the neck bone frame: s, v, half depth, half width. */
export const NECK: readonly Row[] = [
  [-0.04, 0.016, 0.012, 0.012],
  [-0.03, 0.016, 0.034, 0.032],
  [0.0, 0.014, 0.045, 0.042],
  [0.022, 0.011, 0.045, 0.039],
  [0.045, 0.008, 0.042, 0.035],
  [0.065, 0.005, 0.035, 0.032],
  [0.082, 0.003, 0.024, 0.024],
  [0.09, 0.002, 0.008, 0.008],
];
/** Skull and face in the head bone frame. */
export const SKULL: readonly Row[] = [
  [-0.012, 0.014, 0.01, 0.012],
  [-0.006, 0.013, 0.025, 0.027],
  [0.006, 0.0105, 0.0335, 0.036],
  [0.022, 0.0075, 0.0395, 0.0405],
  [0.038, 0.0045, 0.0405, 0.041],
  [0.05, 0.002, 0.0385, 0.0395],
  [0.058, -0.001, 0.0335, 0.0355],
  [0.065, -0.004, 0.025, 0.0285],
  [0.073, -0.007, 0.018, 0.0205],
  [0.081, -0.0085, 0.0135, 0.015],
  [0.087, -0.009, 0.0095, 0.0105],
  [0.0895, -0.009, 0.005, 0.006],
];
export const MANDIBLE: readonly Row[] = [
  [0.006, -0.022, 0.006, 0.016],
  [0.02, -0.0245, 0.009, 0.023],
  [0.04, -0.026, 0.009, 0.0225],
  [0.056, -0.027, 0.0085, 0.017],
  [0.068, -0.0275, 0.0075, 0.0115],
  [0.076, -0.027, 0.006, 0.008],
  [0.0795, -0.0265, 0.003, 0.004],
];
export const TONGUE_ROWS: readonly Row[] = [[0.024, -0.019, 0.00375, 0.007], [0.038, -0.02, 0.00375, 0.0085], [0.054, -0.021, 0.00375, 0.0075], [0.066, -0.0215, 0.00375, 0.005]];
/** Ear leaf: fraction of the length, half width, half thickness. */
export const EAR_T: readonly Row[] = [[0, 0.0215, 0.0038], [0.2, 0.021, 0.0038], [0.45, 0.0175, 0.0038], [0.7, 0.0125, 0.0036], [0.9, 0.0066, 0.003], [1, 0.002, 0.002]];
export const TAIL_R = [0.0155, 0.013, 0.0118, 0.0108, 0.01, 0.009];

export const THIGH: readonly Row[] = [
  [-0.028, -0.008, 0.006, 0.005, 0.026],
  [-0.02, -0.008, 0.02, 0.015, 0.027],
  [-0.004, -0.009, 0.036, 0.021, 0.029],
  [0.02, -0.011, 0.043, 0.024, 0.031],
  [0.045, -0.011, 0.041, 0.024, 0.033],
  [0.068, -0.009, 0.035, 0.0225, 0.035],
  [0.088, -0.005, 0.028, 0.0205, 0.037],
  [0.102, -0.001, 0.021, 0.0185, 0.038],
  [0.112, 0.002, 0.012, 0.014, 0.038],
  [0.117, 0.003, 0.004, 0.004, 0.038],
];
export const SHANK: readonly Row[] = [
  [-0.016, 0.004, 0.005, 0.005, 0.042],
  [-0.008, 0.002, 0.016, 0.012, 0.042],
  [0.006, -0.003, 0.0235, 0.016, 0.042],
  [0.025, -0.007, 0.0235, 0.016, 0.041],
  [0.048, -0.007, 0.0195, 0.014, 0.039],
  [0.072, -0.006, 0.015, 0.012, 0.037],
  [0.093, -0.006, 0.013, 0.0112, 0.035],
  [0.106, -0.005, 0.013, 0.0115, 0.034],
  [0.116, -0.004, 0.006, 0.006, 0.034],
];
export const HOCK: readonly Row[] = [
  [-0.022, -0.011, 0.003, 0.004, 0.034],
  [-0.016, -0.0095, 0.008, 0.008, 0.034],
  [-0.004, -0.005, 0.013, 0.0118, 0.034],
  [0.01, -0.001, 0.012, 0.012, 0.034],
  [0.032, 0.0, 0.0102, 0.0122, 0.034],
  [0.054, 0.0, 0.0102, 0.013, 0.034],
  [0.068, 0.001, 0.011, 0.0145, 0.034],
  [0.077, 0.002, 0.006, 0.01, 0.034],
];
export const SHOULDER: readonly Row[] = [
  [-0.009, 0.0, 0.004, 0.003, 0.009],
  [-0.003, -0.001, 0.021, 0.011, 0.012],
  [0.012, -0.002, 0.031, 0.015, 0.018],
  [0.032, -0.003, 0.034, 0.017, 0.024],
  [0.05, -0.002, 0.032, 0.017, 0.028],
  [0.064, 0.0, 0.027, 0.0165, 0.03],
  [0.074, 0.002, 0.019, 0.012, 0.03],
  [0.079, 0.003, 0.004, 0.004, 0.03],
];
export const ARM: readonly Row[] = [
  [-0.017, -0.002, 0.004, 0.004, 0.037],
  [-0.009, -0.004, 0.014, 0.01, 0.037],
  [0.01, -0.008, 0.024, 0.0135, 0.038],
  [0.035, -0.01, 0.027, 0.0145, 0.039],
  [0.06, -0.008, 0.024, 0.014, 0.04],
  [0.08, -0.005, 0.019, 0.0125, 0.04],
  [0.092, -0.003, 0.015, 0.0115, 0.04],
  [0.101, -0.003, 0.006, 0.006, 0.04],
];
export const FOREARM: readonly Row[] = [
  [-0.017, -0.011, 0.006, 0.007, 0.04],
  [-0.006, -0.006, 0.0145, 0.0115, 0.04],
  [0.008, -0.002, 0.0175, 0.0135, 0.0395],
  [0.025, 0.0, 0.0165, 0.0135, 0.039],
  [0.045, 0.0, 0.014, 0.0125, 0.038],
  [0.065, 0.0, 0.012, 0.012, 0.0365],
  [0.08, -0.001, 0.012, 0.0128, 0.035],
  [0.088, -0.002, 0.0115, 0.0128, 0.034],
  [0.095, -0.002, 0.005, 0.007, 0.034],
];
export const WRIST: readonly Row[] = [
  [-0.009, -0.003, 0.007, 0.009, 0.034],
  [0.0, -0.004, 0.013, 0.013, 0.034],
  [0.012, -0.002, 0.0115, 0.0135, 0.034],
  [0.026, 0.0, 0.0105, 0.0145, 0.034],
  [0.038, 0.001, 0.011, 0.016, 0.034],
  [0.047, 0.002, 0.006, 0.011, 0.034],
];
/** Paw body behind the toes, relative to the toe tip (x) and the ground (y): x, centre height, half height, half width, exponent. */
export const PAW_H: readonly Row[] = [[-0.047, 0.0135, 0.005, 0.007, 2.2], [-0.041, 0.0145, 0.0105, 0.0125, 2.3], [-0.031, 0.015, 0.0125, 0.0148, 2.4], [-0.02, 0.013, 0.0108, 0.016, 2.5], [-0.011, 0.0105, 0.008, 0.015, 2.5], [-0.006, 0.009, 0.005, 0.012, 2.4]];
export const PAW_F: readonly Row[] = [[-0.043, 0.013, 0.005, 0.008, 2.2], [-0.037, 0.014, 0.0105, 0.014, 2.3], [-0.028, 0.0145, 0.0122, 0.0165, 2.4], [-0.018, 0.013, 0.0108, 0.018, 2.5], [-0.01, 0.0105, 0.008, 0.0168, 2.5], [-0.005, 0.009, 0.005, 0.0135, 2.4]];
/** Toes of a paw: lateral offset from the paw centre, set-back of the tip behind the leading toe. */
export const TOES_H = [[-0.0112, 0.0065], [-0.0038, 0.0], [0.0038, 0.0005], [0.0112, 0.0075]] as const;
export const TOES_F = [[-0.0128, 0.007], [-0.0044, 0.0], [0.0044, 0.0005], [0.0128, 0.008]] as const;

/** 🦻️ Ear leaf of side `sd`: a cupped triangle rising from the top of the skull, its broad face turned forward and outward. */
export function ear(S: Cat, id: string, sd: -1 | 1, look: Look, o: { len?: number; rows?: readonly Row[]; sink?: number; flat?: boolean } = {}): Json {
  const len = o.len ?? 0.054;
  const b = S.at("head", 0.011, 0.031 - (o.sink ?? 0), sd * 0.025);
  const lat: V3 = [-0.55, 0, sd * 0.83];
  const d: V3 = o.flat === true ? [lat[0] * 0.3, 0.954, lat[2] * 0.3] : [0.1, 0.96, sd * 0.26];
  const cup = o.flat === true ? 0 : 0.004;
  const pts: PP[] = (o.rows ?? EAR_T).map(([t, rl, rs]) => ({ p: [b[0] + d[0] * len * t! - cup * Math.sin(Math.PI * t!), b[1] + d[1] * len * t!, b[2] + d[2] * len * t!] as V3, rl: rl!, rs: rs!, e: 2.3 }));
  return S.run(id, `ear-${sx(sd)}`, pts, look, { lat, floor: 0.0015 });
}

/** 🐾️ A paw flat on the ground: the body behind the toes, four toes in their arc, the metapodial pad and four digital pads. */
export function paw(S: Cat, leg: "h" | "f", sd: -1 | 1): void {
  const k = sx(sd);
  const bone = `paw-${leg}${k}`;
  const tip = S.tipOf(bone);
  const f = S.frame(bone);
  const zc = f.o[2]!;
  const x0 = tip[0] + 0.0025;
  const rows = leg === "h" ? PAW_H : PAW_F;
  const toes = leg === "h" ? TOES_H : TOES_F;
  const w = leg === "h" ? 1 : 1.1;
  S.hull(`paw-${leg}${k}`, bone, rows.map((r) => [r[0]!, r[1]!, r[2]!, r[3]!, r[4]!, zc]), LIMB, { x0, sd: 1 });
  toes.forEach(([dz, back], i) => {
    const x = x0 - back - 0.0098;
    S.ball(`toe-${leg}${k}-${i + 1}`, bone, [x, 0.0088, zc + dz], [0.0098, 0.0052 * w, 0.0062], LIMB, [1, -0.12, 0]);
    S.ball(`toepad-${leg}${k}-${i + 1}`, bone, [x + 0.0015, 0.0038, zc + dz], [0.0052, 0.0044 * w, 0.0038], PAD, [1, 0, 0]);
  });
  S.ball(`pad-${leg}${k}`, bone, [x0 - (leg === "h" ? 0.027 : 0.025), 0.0042, zc], [0.0085, 0.0095 * w, 0.0042], PAD, [1, 0, 0]);
}

export function body(S: Cat): void {
  const T = dense(TRUNK, 2);
  const rowAt = (x: number): number[] => {
    const i = Math.max(1, T.findIndex((r) => r[0]! >= x));
    const [p, q] = [T[i - 1]!, T[i]!];
    const k = (x - p[0]!) / (q[0]! - p[0]!);
    return p.map((v, c) => v + (q[c]! - v) * k);
  };
  const tuck = (r: readonly number[]): number[] => [r[0]!, r[1]!, r[2]! - 0.003, r[3]! - 0.003, r[4]!];
  const cut = (lo: number, hi: number, from: number, to: number): number[][] => [
    ...T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! < from - 0.006).map(tuck),
    ...(from > lo ? [tuck(rowAt(from - 0.004)), rowAt(from)] : []),
    ...T.filter((r) => r[0]! > from + 0.002 && r[0]! < to - 0.002 && r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9),
    ...(to < hi ? [rowAt(to), tuck(rowAt(to + 0.004))] : []),
    ...T.filter((r) => r[0]! > to + 0.006 && r[0]! <= hi + 1e-9).map(tuck),
  ];
  S.hull("rump", "pelvis", cut(-0.05, 0.0775, -1, 0.04), COAT);
  S.hull("hindbody", "lumbar", cut(0.0, 0.23, 0.04, 0.19), COAT);
  S.hull("midbody", "thorax-a", cut(0.1475, 0.287, 0.19, 0.25), COAT);
  S.hull("chest", "thorax-b", cut(0.215, 0.32, 0.25, 1), COAT);
  S.limb("neck", "neck", dense(NECK, 2), COAT, { e: 2.1 });
  S.limb("head", "head", dense(SKULL, 2), FACE, { e: 2.5 });
  S.limb("jaw", "jaw", dense(MANDIBLE, 2), FACE, { frame: "head", e: 2.3 });
  S.limb("tongue", "jaw", TONGUE_ROWS, TONGUE, { frame: "head", e: 2.6 });
  const hu = S.frame("head").u;
  const hv = S.frame("head").v;
  S.ball("nose", "head", S.at("head", 0.0885, -0.0075, 0), [0.005, 0.0066, 0.0056], NOSE, [hu[0]!, hu[1]!, 0]);
  for (const sd of SIDES) {
    const k = sx(sd);
    S.ball(`muzzle-${k}`, "head", S.at("head", 0.0795, -0.0155, sd * 0.0092), [0.0115, 0.0112, 0.0102], FACE, [hu[0]!, hu[1]!, 0]);
    S.ball(`eye-${k}`, "head", S.at("head", 0.0665, 0.012, sd * 0.0195), [0.00375, 0.0045, 0.006], EYE, [hu[0]! * 0.62 + hv[0]! * 0.45, hu[1]! * 0.62 + hv[1]! * 0.45, sd * 0.64], [hv[0]!, hv[1]!, 0]);
    ear(S, `ear-${k}`, sd, EAR);
  }
  const tail = ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"];
  tail.forEach((b, i) => {
    const o = S.frame(b).o as V3;
    S.tubeDome(b, b, i === 0 ? [o[0] + 0.012, o[1] - 0.002, 0] : [o[0], o[1], 0], S.tipOf(b), TAIL_R[i]!, TAIL_R[i + 1]!, COAT);
  });
  for (const sd of SIDES) {
    const k = sx(sd);
    S.limb(`thigh-h${k}`, `thigh-h${k}`, dense(THIGH, 2), LIMB, { sd });
    S.limb(`shank-h${k}`, `shank-h${k}`, dense(SHANK, 2), LIMB, { sd });
    S.limb(`hock-h${k}`, `metatarsus-h${k}`, dense(HOCK, 2), LIMB, { sd, e: 2.4 });
    paw(S, "h", sd);
    S.limb(`shoulder-f${k}`, `scapula-f${k}`, dense(SHOULDER, 2), LIMB, { sd });
    S.limb(`arm-f${k}`, `humerus-f${k}`, dense(ARM, 2), LIMB, { sd });
    S.limb(`forearm-f${k}`, `forearm-f${k}`, dense(FOREARM, 2), LIMB, { sd });
    S.limb(`wrist-f${k}`, `metacarpus-f${k}`, dense(WRIST, 2), LIMB, { sd, e: 2.4 });
    paw(S, "f", sd);
  }
}

/** Region and layer of a body solid by its id. */
export const REGION: readonly (readonly [RegExp, string, string?])[] = [
  [/^(rump|hindbody|midbody|chest|trunk|neck)$/, "coat-trunk", "fur"], [/^(head|jaw|muzzle-[nf])$/, "coat-head", "fur"], [/^ear-[nf]$/, "ears", "fur"], [/^tail-/, "coat-tail", "fur"],
  [/^(thigh|shank|hock|paw|toe|foot|lower)-h[nf]/, "coat-hindlimbs", "fur"], [/^(shoulder|arm|forearm|wrist|paw|toe|foot|upper|lower)-f[nf]/, "coat-forelimbs", "fur"],
  [/^nose$/, "nose-skin"], [/^eye-[nf]$/, "eyes"], [/^tongue$/, "tongue"], [/^(pad|toepad|sole)-/, "pads"],
];
export const regionOf = (id: string): readonly [RegExp, string, string?] => {
  const hit = REGION.find(([re]) => re.test(id));
  if (hit === undefined) throw new Error(`no region for ${id}`);
  return hit;
};

/** 🌡️ Thermal list of a scale: every body solid as the surface of its region, the regions the body of that scale does not show as own small solids (`extra`, built into `T`), and the outlets as patches of the fur layer where the flow leaves (D2), each as thick as that layer: the nostrils on the nose, the lips, a patch of the back for the plume. */
export function thermal(S: Cat, T: Cat, o: { extra?: (T: Cat) => void } = {}): Json[] {
  const out: Json[] = S.body.map((b) => { const [, region, layer] = regionOf(b.id); return { id: `th-${b.id}`, of: b.id, region, ...(layer === undefined ? {} : { layer }) }; });
  o.extra?.(T);
  for (const t of T.body) { const [, region, layer] = regionOf(t.id); out.push({ ...t, id: `th-${t.id}`, region, ...(layer === undefined ? {} : { layer }) }); }
  const E = new Cat(S.scale);
  const hu = E.frame("head").u;
  const hv = E.frame("head").v;
  const down: V3 = [hu[0]! * 0.8 - hv[0]! * 0.6, hu[1]! * 0.8 - hv[1]! * 0.6, 0];
  const [nose, mouth, plume] = OUTLETS[S.scale]!;
  E.ball("th-breath-nose", "head", E.at("head", 0.0895, -0.0095, 0), [0.003, nose![0]!, nose![1]!], { material: "bare-skin", tone: 0.9 }, down);
  E.ball("th-breath-mouth", "jaw", E.at("head", 0.083, -0.0215, 0), [0.003, mouth![0]!, mouth![1]!], { material: "bare-skin", tone: 0.9 }, [hu[0]!, hu[1]!, 0]);
  E.ball("th-plume-back", S.scale >= 200 ? "thorax-b" : "thorax-a", [0.205, 0.2555, 0], [plume![0]!, plume![1]!, 0.006], { material: "short-fur", tone: 0.2 }, [1, 0, 0]);
  for (const e of E.body) out.push({ ...e, region: e.id === "th-plume-back" ? "coat-trunk" : "coat-head", layer: "fur", tags: ["emitter", e.id.slice(3)] });
  return out;
}

/** Outlet patches per scale: half width and half height of the nostril and lip patches, half length and half width of the back patch (generalised with the scale; the thickness is the fur layer). */
export const OUTLETS: Record<number, readonly (readonly number[])[]> = { 50: [[0.0045, 0.003], [0.0085, 0.003], [0.058, 0.022]], 100: [[0.0055, 0.0035], [0.01, 0.0035], [0.064, 0.026]], 200: [[0.007, 0.004], [0.012, 0.004], [0.072, 0.03]], 500: [[0.004, 0.0025], [0.0075, 0.0025], [0.05, 0.02]] };

export const BONE: Look = { material: "bone", tone: 0.18 };
export const JOINT: Look = { material: "bone", tone: 0.5 };
export const MASS: Look = { material: "short-fur", tone: 0.7 };
export const mem = (s: Json, member: string, part = "member"): Json => Object.assign(s, { member, part });
/** Vertebral axis in the default pose (world): lumbar, thoracic and cervical centra. */
export const SPINE_L: readonly V3[] = [[0.045, 0.25, 0], [0.11, 0.2535, 0], [0.178, 0.246, 0]];
export const SPINE_T: readonly V3[] = [[0.178, 0.246, 0], [0.212, 0.243, 0], [0.245, 0.236, 0], [0.28, 0.223, 0], [0.305, 0.208, 0]];
export const SPINE_C: readonly V3[] = [[0.305, 0.208, 0], [0.318, 0.218, 0], [0.33, 0.235, 0], [0.335, 0.254, 0], [0.3365, 0.27, 0]];
export const spineY = (x: number): number => {
  for (let i = 1; i < SPINE_T.length; i++) if (x <= SPINE_T[i]![0]) { const a = SPINE_T[i - 1]!; const b = SPINE_T[i]!; return a[1] + ((x - a[0]) / (b[0] - a[0])) * (b[1] - a[1]); }
  return SPINE_T[SPINE_T.length - 1]![1];
};
/** Ribs 1 to 13: half width of the cage, depth from the spine to the ventral end, caudal sweep of the ventral end. */
export const RIBS: readonly Row[] = [[0.018, 0.043, -0.004], [0.023, 0.054, 0.0], [0.028, 0.064, 0.004], [0.032, 0.073, 0.008], [0.036, 0.08, 0.012], [0.039, 0.085, 0.016], [0.042, 0.088, 0.02], [0.044, 0.088, 0.024], [0.046, 0.085, 0.027], [0.047, 0.077, 0.03], [0.046, 0.066, 0.03], [0.043, 0.054, 0.028], [0.039, 0.042, 0.025]];
export const ribPath = (i: number, sd: number, r: number): PP[] => {
  const [w, d, k] = RIBS[i]!;
  const x = 0.298 - i * 0.0098;
  const y = spineY(x) - 0.004;
  return ([[0, 0, 0.2], [0.2, 0.1, 0.74], [0.5, 0.4, 1], [0.8, 0.74, 0.9], [1, 1, 0.55]] as const).map(([a, b, c]) => ({ p: [x - a * k!, y - b * d!, sd * c * w!] as V3, rl: r, rs: r }));
};
const pathLength = (pts: readonly PP[]): number => pts.slice(1).reduce((s, q, i) => s + Math.hypot(q.p[0] - pts[i]!.p[0], q.p[1] - pts[i]!.p[1], q.p[2] - pts[i]!.p[2]), 0);
export const CRANIUM: readonly Row[] = [[-0.008, 0.016, 0.012, 0.014], [0.004, 0.016, 0.02, 0.022], [0.02, 0.015, 0.0215, 0.0235], [0.036, 0.012, 0.02, 0.029], [0.05, 0.008, 0.017, 0.027], [0.06, 0.003, 0.014, 0.018], [0.072, -0.002, 0.011, 0.0125], [0.083, -0.005, 0.008, 0.009], [0.087, -0.006, 0.004, 0.005]];
export const JAWBONE: readonly Row[] = [[0.012, -0.006, 0.009, 0.022], [0.022, -0.017, 0.008, 0.021], [0.04, -0.022, 0.0055, 0.017], [0.058, -0.024, 0.005, 0.011], [0.07, -0.0245, 0.0045, 0.007], [0.0765, -0.024, 0.003, 0.004]];
export const SCAPULA: readonly Row[] = [[-0.004, 0, 0.016, 0.0025, 0.018], [0.01, 0, 0.02, 0.0025, 0.023], [0.035, 0, 0.013, 0.003, 0.031], [0.055, 0, 0.0065, 0.004, 0.036], [0.064, 0, 0.007, 0.006, 0.037]];
export const HUMERUS: readonly Row[] = [[0, 0, 0.009, 0.008, 0.037], [0.012, 0.002, 0.008, 0.007, 0.038], [0.03, 0, 0.005, 0.005, 0.039], [0.07, 0, 0.005, 0.005, 0.04], [0.085, 0, 0.008, 0.009, 0.04], [0.091, 0, 0.007, 0.009, 0.04]];
export const RADIUS: readonly Row[] = [[-0.014, -0.009, 0.004, 0.004, 0.04], [0, -0.002, 0.008, 0.007, 0.04], [0.015, 0, 0.0055, 0.006, 0.0395], [0.06, 0, 0.005, 0.006, 0.037], [0.08, 0, 0.0065, 0.008, 0.035], [0.085, 0, 0.006, 0.008, 0.034]];
export const METACARPALS: readonly Row[] = [[-0.003, -0.001, 0.006, 0.009, 0.034], [0.006, 0, 0.005, 0.0095, 0.034], [0.02, 0, 0.004, 0.0105, 0.034], [0.036, 0, 0.005, 0.012, 0.034], [0.041, 0, 0.004, 0.011, 0.034]];
export const FEMUR: readonly Row[] = [[-0.004, 0, 0.008, 0.008, 0.03], [0.006, -0.004, 0.009, 0.008, 0.034], [0.02, 0, 0.0055, 0.0055, 0.036], [0.075, 0, 0.0055, 0.0055, 0.041], [0.092, 0, 0.009, 0.0095, 0.042], [0.099, 0.002, 0.008, 0.009, 0.042]];
export const TIBIA: readonly Row[] = [[0, 0, 0.009, 0.0095, 0.042], [0.012, 0.002, 0.007, 0.007, 0.042], [0.04, 0, 0.0048, 0.005, 0.04], [0.09, 0, 0.0045, 0.0048, 0.0355], [0.102, 0, 0.006, 0.007, 0.034], [0.105, 0, 0.005, 0.006, 0.034]];
export const METATARSALS: readonly Row[] = [[-0.002, 0, 0.007, 0.008, 0.034], [0.012, 0, 0.0055, 0.008, 0.034], [0.03, 0, 0.004, 0.008, 0.034], [0.062, 0, 0.0045, 0.0105, 0.034], [0.07, 0, 0.004, 0.01, 0.034]];
export const DIGITS: readonly Row[] = [[0.0, 0, 0.004, 0.0115, 0.034], [0.014, 0, 0.0035, 0.013, 0.034], [0.027, 0, 0.003, 0.012, 0.034]];
export const JOINT_R: Record<string, number> = { lumbar: 0.0085, "thorax-a": 0.0075, "thorax-b": 0.0075, neck: 0.008, head: 0.009, jaw: 0.005, ear: 0.004, "tail-1": 0.0055, "tail-2": 0.0045, "tail-3": 0.004, "tail-4": 0.0038, "tail-5": 0.0038, thigh: 0.009, shank: 0.0105, metatarsus: 0.008, paw: 0.006, scapula: 0.004, humerus: 0.0095, forearm: 0.009, metacarpus: 0.0075 };

/** 🦴️ Structure at 1:50: the skeleton, bone by bone, joints at the rig joints, the four trunk masses at their centres of mass. */
export function structure(K: Cat): Json[] {
  const fl = 0.00375;
  mem(K.run("sacrum", "pelvis", [{ p: [0.047, 0.25, 0], rl: 0.012, rs: 0.007 }, { p: [0.03, 0.2515, 0], rl: 0.01, rs: 0.0065 }, { p: [0.012, 0.252, 0], rl: 0.006, rs: 0.005 }, { p: [-0.03, 0.2505, 0], rl: 0.0045, rs: 0.004 }], BONE), "pelvis");
  mem(K.run("pubis", "pelvis", [{ p: [0.006, 0.219, -0.022], rl: 0.004, rs: 0.004 }, { p: [0.002, 0.212, 0], rl: 0.005, rs: 0.004 }, { p: [0.006, 0.219, 0.022], rl: 0.004, rs: 0.004 }], BONE, { lat: [1, 0, 0] }), "pelvis");
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.01, rs: 0.008 })), BONE), "lumbar");
  mem(K.run("lumbar-transverse", "lumbar", [{ p: [0.055, 0.247, 0], rl: 0.019, rs: fl }, { p: [0.11, 0.2495, 0], rl: 0.024, rs: fl }, { p: [0.17, 0.243, 0], rl: 0.02, rs: fl }], BONE), "lumbar");
  mem(K.run("lumbar-spinous", "lumbar", [{ p: [0.05, 0.2625, 0], rl: fl, rs: 0.0055 }, { p: [0.11, 0.266, 0], rl: fl, rs: 0.0055 }, { p: [0.172, 0.2595, 0], rl: fl, rs: 0.0055 }], BONE), "lumbar");
  mem(K.run("thoracic-spine-a", "thorax-a", SPINE_T.slice(0, 3).map((p) => ({ p, rl: 0.008, rs: 0.007 })), BONE), "thorax-a");
  mem(K.run("thoracic-spine-b", "thorax-b", SPINE_T.slice(2).map((p) => ({ p, rl: 0.008, rs: 0.007 })), BONE), "thorax-b");
  mem(K.run("thoracic-spinous-a", "thorax-a", ([[0.182, 0.0055], [0.212, 0.006], [0.243, 0.007]] as const).map(([x, h]) => ({ p: [x, spineY(x) + 0.006 + h, 0] as V3, rl: fl, rs: h })), BONE), "thorax-a");
  mem(K.run("withers", "thorax-b", ([[0.247, 0.007], [0.265, 0.009], [0.283, 0.011], [0.298, 0.0095]] as const).map(([x, h]) => ({ p: [x - h * 0.4, spineY(x) + 0.006 + h, 0] as V3, rl: fl, rs: h })), BONE), "thorax-b");
  mem(K.run("cervical-spine", "neck", SPINE_C.map((p, i) => ({ p, rl: i > 2 ? 0.012 : 0.0105, rs: 0.008 })), BONE), "neck");
  mem(K.run("sternum", "thorax-b", [{ p: [0.306, 0.166, 0], rl: 0.004, rs: 0.004 }, { p: [0.28, 0.147, 0], rl: 0.005, rs: 0.004 }, { p: [0.245, 0.137, 0], rl: 0.005, rs: 0.004 }, { p: [0.212, 0.135, 0], rl: 0.004, rs: 0.003 }], BONE), "thorax-b");
  mem(K.limb("skull", "head", CRANIUM, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", JAWBONE, BONE, { frame: "head", e: 2.3 }), "jaw");
  ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"].forEach((b, i) => { const o = K.frame(b).o as V3; mem(K.tubeDome(`caudal-${i + 1}`, b, [o[0], o[1], 0], K.tipOf(b), 0.0055 - i * 0.0003, 0.0052 - i * 0.0003, BONE), b); });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(K.run(`${n}-ilium`, "pelvis", [{ p: [0.052, 0.26, sd * 0.027], rl: 0.004, rs: 0.013 }, { p: [0.035, 0.253, sd * 0.026], rl: 0.004, rs: 0.011 }, { p: [0.016, 0.243, sd * 0.028], rl: 0.005, rs: 0.007 }, { p: [0.0, 0.235, sd * 0.03], rl: 0.007, rs: 0.009 }], BONE), "pelvis");
    mem(K.run(`${n}-ischium`, "pelvis", [{ p: [0.0, 0.235, sd * 0.03], rl: 0.006, rs: 0.007 }, { p: [-0.02, 0.231, sd * 0.028], rl: 0.004, rs: 0.006 }, { p: [-0.036, 0.228, sd * 0.027], rl: 0.005, rs: 0.008 }], BONE), "pelvis");
    for (let i = 0; i < 13; i++) { const r = Math.min(0.002, pathLength(ribPath(i, sd, 0.002)) / 44); mem(K.run(`${n}-rib-${i + 1}`, i < 6 ? "thorax-b" : "thorax-a", ribPath(i, sd, r), BONE, { floor: 0.001 }), i < 6 ? "thorax-b" : "thorax-a").slender = true; }
    mem(K.run(`${n}-costal-arch`, "thorax-a", [{ p: [0.214, 0.137, sd * 0.006], rl: 0.0018, rs: 0.0018 }, ...[8, 9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, 0.002)[4]!.p, rl: 0.0018, rs: 0.0018 }))], BONE, { floor: 0.001 }), "thorax-a").slender = true;
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, { len: 0.048, rows: [[0.04, 0.014, 0.00375], [0.45, 0.011, 0.00375], [1, 0.003, 0.003]] }), `ear-${k}`);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, SCAPULA, BONE, { sd, e: 2.6 }), `scapula-f${k}`);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, HUMERUS, BONE, { sd }), `humerus-f${k}`);
    mem(K.limb(`${n}-radius-ulna`, `forearm-f${k}`, RADIUS, BONE, { sd }), `forearm-f${k}`);
    mem(K.limb(`${n}-metacarpals`, `metacarpus-f${k}`, METACARPALS, BONE, { sd, e: 2.6 }), `metacarpus-f${k}`);
    mem(K.limb(`${n}-fore-digits`, `paw-f${k}`, DIGITS, BONE, { sd, e: 2.8 }), `paw-f${k}`);
    mem(K.limb(`${n}-femur`, `thigh-h${k}`, FEMUR, BONE, { sd }), `thigh-h${k}`);
    const tu = K.frame(`thigh-h${k}`).u;
    mem(K.ball(`${n}-patella`, `thigh-h${k}`, K.at(`thigh-h${k}`, 0.093, 0.0125, sd * 0.042), [0.006, 0.0042, 0.004], BONE, [tu[0]!, tu[1]!, 0]), `thigh-h${k}`);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, TIBIA, BONE, { sd }), `shank-h${k}`);
    mem(K.run(`${n}-calcaneus`, `metatarsus-h${k}`, [{ p: K.at(`metatarsus-h${k}`, 0.006, -0.003, sd * 0.034), rl: 0.0045, rs: 0.0045 }, { p: K.at(`metatarsus-h${k}`, -0.019, -0.0105, sd * 0.034), rl: 0.004, rs: 0.004 }], BONE), `metatarsus-h${k}`);
    mem(K.limb(`${n}-metatarsals`, `metatarsus-h${k}`, METATARSALS, BONE, { sd, e: 2.6 }), `metatarsus-h${k}`);
    mem(K.limb(`${n}-hind-digits`, `paw-h${k}`, DIGITS, BONE, { sd, e: 2.8 }), `paw-h${k}`);
  }
  joints(K, 1);
  for (const [b, at, r] of [["pelvis", [0.005, 0.232, 0], 0.0125], ["lumbar", [0.112, 0.214, 0], 0.0125], ["thorax-a", [0.21, 0.198, 0], 0.013], ["thorax-b", [0.272, 0.196, 0], 0.013]] as const) mem(K.ball(`mass-${b}`, b, [...at] as V3, [r, r, r], MASS), b, "mass");
  return K.body;
}

/** 🟠️ Joint solids at the rig joints (every bone but the root, or only those `only` accepts), sized by the joint and scaled by `k`. */
export function joints(K: Cat, k: number, only: (bone: string) => boolean = () => true, min = 0): void {
  for (const b of K.c.raw.rig.bones as Json[]) {
    if (b.parent === null || !only(b.id)) continue;
    const stem = (b.id as string).replace(/-[hf]?[nf]$/, "");
    const r = Math.max((JOINT_R[stem] ?? JOINT_R[b.id]!) * k, stem === "ear" ? 0 : min);
    const o = K.frame(b.id).o as V3;
    mem(K.ball(`joint-${b.id}`, b.id, [o[0], o[1], o[2]], [r, r, r], JOINT), b.id, "joint");
  }
}

const DECISIONS = [
  {
    en: "Body at 1:50 (smallest feature 7.5 mm): 74 solids, every section stated by hand from the skeletal landmarks of a 4.2 kg short-haired cat standing alert (withers 0.25 m, hip joint 0.235 m, nose to rump 0.47 m). The trunk is four overlapping lofts on pelvis, loin, hind and fore ribcage with a level back, a deep chest behind the elbows and a belly tucked up to the flank; the neck is short and thick. The head is a domed skull with a stop, a short muzzle carrying two whisker pads and the nose leather, forward-facing eyes, two cupped ears turned forward and outward, and a mandible with the tongue inside the closed mouth. Each hind leg is a broad flat thigh, a shank with the calf at the top, a metatarsus with the heel of the hock; each foreleg a shoulder over the blade, an upper arm with the triceps, a forearm with the point of the elbow and a metacarpus; every paw has a body, four toes in their arc, a metapodial pad and four digital pads on the ground. Segments reach into their neighbours at every joint (tail segments end in domes) so that poses and clips stay closed. Whiskers and claws are below the feature size and left out.",
    de: "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): 74 Körper, jeder Schnitt von Hand nach den Skelettpunkten einer 4,2 kg schweren Kurzhaarkatze im aufmerksamen Stand gesetzt (Widerrist 0,25 m, Hüftgelenk 0,235 m, Nase bis Kruppe 0,47 m). Der Rumpf besteht aus vier überlappenden Lofts an Becken, Lende, hinterem und vorderem Brustkorb mit geradem Rücken, tiefer Brust hinter den Ellbogen und zur Flanke aufgezogenem Bauch; der Hals ist kurz und kräftig. Der Kopf ist ein gewölbter Schädel mit Stirnabsatz, kurzer Schnauze mit zwei Schnurrhaarkissen und Nasenspiegel, nach vorn gerichteten Augen, zwei gewölbten, nach vorn außen gedrehten Ohren und einem Unterkiefer mit der Zunge im geschlossenen Maul. Jedes Hinterbein besteht aus breitem flachem Oberschenkel, Unterschenkel mit der Wade oben und Mittelfuß mit dem Fersenhöcker; jedes Vorderbein aus Schulter über dem Schulterblatt, Oberarm mit Trizeps, Unterarm mit Ellbogenhöcker und Mittelhand; jede Pfote hat einen Pfotenkörper, vier Zehen in ihrem Bogen, einen Sohlenballen und vier Zehenballen auf dem Boden. Die Segmente greifen an jedem Gelenk ineinander (Schwanzsegmente enden in Kuppeln), damit Posen und Clips geschlossen bleiben. Tasthaare und Krallen liegen unter der Merkmalgröße und entfallen.",
  },
  {
    en: "Thermal at 1:50: every region has at most one declared layer (the fur), so each region is the body surface it covers, as references to the body solids: trunk coat with the neck, head coat with jaw and whisker pads, ears, tail coat, coats of hind and fore limbs with the toes; the bare regions are their own solids: nose leather, the two corneas, the tongue inside the mouth and the twenty pads. The outlets are patches on the surface: the nostrils on the nose, the lips for the open mouth and a patch of the back for the plume, each a piece of the fur layer at its declared thickness (6 mm on the head, 12 mm on the trunk).",
    de: "Thermik bei 1:50: jede Region hat höchstens eine deklarierte Schicht (das Fell), daher ist jede Region die Körperoberfläche, die sie bedeckt, als Verweis auf die Körper: Rumpffell mit Hals, Kopffell mit Unterkiefer und Schnurrhaarkissen, Ohren, Schwanzfell, Fell der Hinter- und Vorderläufe mit den Zehen; die nackten Regionen sind eigene Körper: Nasenspiegel, die beiden Hornhäute, die Zunge im Maul und die zwanzig Ballen. Die Auslässe sind Flächen auf der Oberfläche: die Nasenlöcher an der Nase, die Lippen für das offene Maul und ein Rückenfeld für die Warmluftfahne, jeweils ein Stück der Fellschicht in ihrer deklarierten Dicke (6 mm am Kopf, 12 mm am Rumpf).",
  },
  {
    en: "Structure at 1:50: the skeleton. Skull and mandible, cervical spine in its S curve, thoracic spine with the spinous processes of the withers, thirteen ribs per side at their true 3 to 4 mm (slender) with costal arch and sternum, lumbar spine with transverse and spinous processes, sacrum, ilium, ischium and pubis, five caudal rods, ear cartilages; per foreleg scapula, humerus, radius and ulna with the olecranon, carpus with metacarpals, digits; per hind leg femur with patella, tibia, calcaneus, tarsus with metatarsals, digits. Carpals, metapodials and phalanges are each one flat bundle because single rays are 3 to 5 mm. Joints sit at the rig joints at joint size, the masses of pelvis, loin and the two halves of the ribcage at their centres of mass. The asset declares no structural supports.",
    de: "Tragwerk bei 1:50: das Skelett. Schädel und Unterkiefer, Halswirbelsäule in ihrer S-Kurve, Brustwirbelsäule mit den Dornfortsätzen des Widerrists, dreizehn Rippen je Seite in ihren wahren 3 bis 4 mm (schlank) mit Rippenbogen und Brustbein, Lendenwirbelsäule mit Quer- und Dornfortsätzen, Kreuzbein, Darmbein, Sitzbein und Schambein, fünf Schwanzwirbelstäbe, Ohrknorpel; je Vorderbein Schulterblatt, Oberarmbein, Speiche und Elle mit Ellbogenhöcker, Handwurzel mit Mittelhandknochen, Zehen; je Hinterbein Oberschenkelbein mit Kniescheibe, Schienbein, Fersenbein, Fußwurzel mit Mittelfußknochen, Zehen. Handwurzel, Mittelhand- und Mittelfußknochen sowie Zehenglieder sind je ein flaches Bündel, weil einzelne Strahlen 3 bis 5 mm messen. Gelenke sitzen an den Gelenken des Skeletts in Gelenkgröße, die Massen von Becken, Lende und den beiden Brustkorbhälften in ihren Schwerpunkten. Das Asset deklariert keine Auflager.",
  },
];

if (import.meta.main) {
  const S = new Cat(50);
  body(S);
  const th = thermal(S, new Cat(50));
  const st = structure(new Cat(50));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) saveLists(S.c, { body: S.body, thermal: th, structure: st, decisions: DECISIONS });
}
