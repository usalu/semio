/** 🦌️ Round 5 roe deer at 1:50 (round 3 tables, scapula top lowered under the withers): the finest model of a 25 kg roe buck in summer coat. Body: croup, loin and chest lofts, neck, skull and mandible, nose leather, eyes, leaf ears, six-point antlers on burrs, white rump patch, per leg thigh or shoulder and upper arm, gaskin or forearm, point of hock, cannon with joint blocks and fetlock, dewclaws, pastern and a cloven hoof of two claws. Thermal: the body surfaces per region and the outlets. Structure: the skeleton.
 * Usage: bun r5-roe-deer-50.ts [--write]
 * @see ./r3-roe-deer-kit.ts
 * @see ./r3-animals-roe-deer.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Roe, dense, side, sx, type Json, type Look, type PP, type V3 } from "./r3-roe-deer-kit.ts";

export const COAT: Look = { material: "summer-coat", tone: 0.72 };
export const LEG: Look = { material: "summer-coat", tone: 0.7 };
export const FACE: Look = { material: "summer-coat", tone: 0.72 };
export const EARL: Look = { material: "summer-coat", tone: 0.62 };
export const WHITE: Look = { material: "white-coat", tone: 0.95 };
export const NOSE: Look = { material: "bare-skin", tone: 0.08 };
export const EYE: Look = { material: "bare-skin", tone: 0.05 };
export const HORN: Look = { material: "hoof-horn", tone: 0.2 };
export const ANTLER: Look = { material: "antler-bone", tone: 0.4 };
export const SIDES = [-1, 1] as const;

/** Trunk in planes x = const: x, top line, bottom line, half width, exponent. */
export const TRUNK_LINES = [
  [-0.109, 0.672, 0.628, 0.02, 2.0],
  [-0.102, 0.706, 0.598, 0.046, 2.0],
  [-0.085, 0.736, 0.574, 0.067, 2.1],
  [-0.05, 0.759, 0.556, 0.081, 2.2],
  [0.0, 0.772, 0.547, 0.088, 2.3],
  [0.05, 0.776, 0.541, 0.088, 2.3],
  [0.1, 0.774, 0.537, 0.084, 2.3],
  [0.15, 0.769, 0.531, 0.079, 2.3],
  [0.2, 0.762, 0.519, 0.078, 2.3],
  [0.25, 0.756, 0.499, 0.084, 2.35],
  [0.3, 0.751, 0.477, 0.091, 2.4],
  [0.35, 0.748, 0.461, 0.096, 2.4],
  [0.4, 0.748, 0.451, 0.096, 2.4],
  [0.45, 0.752, 0.446, 0.091, 2.35],
  [0.5, 0.754, 0.445, 0.082, 2.3],
  [0.55, 0.742, 0.45, 0.071, 2.2],
  [0.59, 0.716, 0.467, 0.06, 2.1],
  [0.62, 0.686, 0.494, 0.047, 2.0],
  [0.64, 0.656, 0.524, 0.034, 2.0],
  [0.648, 0.615, 0.565, 0.018, 2.0],
] as const;
export const lines = (rows: readonly (readonly number[])[]): number[][] => rows.map(([x, top, bottom, w, e]) => [x!, (top! + bottom!) / 2, (top! - bottom!) / 2, w!, e!]);
export const NECK = [
  [-0.05, -0.008, 0.094, 0.056],
  [0.0, 0.012, 0.1, 0.057],
  [0.06, 0.013, 0.08, 0.05],
  [0.12, 0.007, 0.062, 0.043],
  [0.18, 0.003, 0.049, 0.038],
  [0.24, 0.001, 0.043, 0.035],
  [0.29, 0.0, 0.041, 0.034],
  [0.33, -0.004, 0.034, 0.03],
] as const;
export const SKULL = [
  [-0.024, 0.014, 0.02, 0.02],
  [-0.008, 0.014, 0.036, 0.032],
  [0.025, 0.012, 0.048, 0.041],
  [0.06, 0.008, 0.053, 0.046],
  [0.09, 0.008, 0.046, 0.044],
  [0.12, 0.0075, 0.0355, 0.035],
  [0.15, 0.006, 0.028, 0.027],
  [0.18, 0.004, 0.023, 0.0225],
  [0.203, 0.003, 0.019, 0.0195],
  [0.217, 0.002, 0.012, 0.014],
] as const;
export const MANDIBLE = [
  [0.03, -0.03, 0.016, 0.026],
  [0.07, -0.036, 0.014, 0.027],
  [0.11, -0.031, 0.012, 0.022],
  [0.15, -0.026, 0.0105, 0.017],
  [0.19, -0.0225, 0.009, 0.013],
  [0.209, -0.021, 0.006, 0.008],
] as const;
export const THIGH = [[-0.06, -0.012, 0.062, 0.028, 0.046], [-0.01, -0.022, 0.092, 0.041, 0.051], [0.04, -0.027, 0.104, 0.046, 0.053], [0.09, -0.026, 0.097, 0.044, 0.056], [0.135, -0.02, 0.077, 0.038, 0.061], [0.175, -0.01, 0.055, 0.032, 0.066], [0.205, -0.002, 0.036, 0.026, 0.068]] as const;
export const GASKIN = [[-0.04, 0.004, 0.036, 0.027, 0.066], [0.01, -0.002, 0.048, 0.032, 0.069], [0.06, -0.008, 0.045, 0.03, 0.07], [0.11, -0.009, 0.034, 0.024, 0.07], [0.16, -0.009, 0.026, 0.018, 0.069], [0.2, -0.012, 0.0225, 0.015, 0.068], [0.232, -0.016, 0.022, 0.0135, 0.068]] as const;
export const HIND_CANNON = [[-0.022, -0.004, 0.022, 0.0155, 0.068], [0.012, -0.003, 0.021, 0.015, 0.068], [0.04, 0, 0.0125, 0.0105, 0.067], [0.1, 0, 0.011, 0.0095, 0.065], [0.17, 0, 0.011, 0.0098, 0.063], [0.205, -0.001, 0.013, 0.012, 0.062], [0.226, -0.002, 0.0155, 0.014, 0.061], [0.244, -0.001, 0.0125, 0.0125, 0.061]] as const;
export const PASTERN = [[-0.012, 0, 0.0135, 0.013], [0.02, 0, 0.012, 0.0135], [0.042, 0, 0.0125, 0.0145], [0.056, 0, 0.0115, 0.014]] as const;
/** Claw in planes x = const behind the toe tip: x, centre height, half height, half width, exponent, lateral centre from the foot axis. */
export const CLAW = [[-0.044, 0.006, 0.006, 0.005, 2.5, 0.0044], [-0.0365, 0.0095, 0.0095, 0.0058, 2.5, 0.005], [-0.0275, 0.012, 0.012, 0.006, 2.5, 0.0058], [-0.0185, 0.0095, 0.0095, 0.0056, 2.5, 0.0067], [-0.0095, 0.0062, 0.0062, 0.0049, 2.5, 0.0076], [-0.001, 0.0042, 0.0042, 0.0042, 2.5, 0.0083]] as const;
export const SCAPULA = [[0.03, 0.002, 0.036, 0.014, 0.05], [0.05, 0.002, 0.046, 0.018, 0.053], [0.08, 0.002, 0.05, 0.02, 0.056], [0.13, 0.0, 0.046, 0.021, 0.059], [0.165, -0.004, 0.04, 0.021, 0.061], [0.19, -0.008, 0.03, 0.018, 0.062]] as const;
export const UPPER_ARM = [[-0.01, -0.008, 0.03, 0.018, 0.058], [0.02, -0.014, 0.046, 0.023, 0.062], [0.07, -0.02, 0.052, 0.025, 0.065], [0.115, -0.016, 0.044, 0.024, 0.068], [0.15, -0.008, 0.033, 0.022, 0.07], [0.18, -0.002, 0.023, 0.018, 0.071]] as const;
export const FOREARM = [[-0.04, -0.016, 0.022, 0.016, 0.072], [-0.005, -0.008, 0.031, 0.021, 0.072], [0.04, -0.003, 0.027, 0.02, 0.071], [0.09, 0, 0.02, 0.016, 0.069], [0.14, 0, 0.0155, 0.013, 0.067], [0.18, 0, 0.015, 0.0135, 0.066], [0.205, 0, 0.0155, 0.014, 0.065]] as const;
export const FORE_CANNON = [[-0.02, 0.001, 0.0165, 0.015, 0.065], [0.01, 0.001, 0.016, 0.0145, 0.065], [0.032, 0, 0.011, 0.0098, 0.064], [0.09, 0, 0.0102, 0.0092, 0.063], [0.135, 0, 0.0115, 0.0108, 0.062], [0.162, -0.001, 0.015, 0.0135, 0.061], [0.182, -0.001, 0.0125, 0.0125, 0.061]] as const;
export const HIND_Z = 0.059;
export const FORE_Z = 0.059;
export const EAR_T = [[0, 0.012, 0.01], [0.1, 0.02, 0.008], [0.3, 0.03, 0.0055], [0.55, 0.033, 0.0045], [0.78, 0.025, 0.0042], [0.93, 0.013, 0.0042], [1, 0.0055, 0.0042]] as const;

/** 🦻️ Ear leaf of side `sd` along its bone, splayed outward, the open side turned forward and outward. */
export function ear(S: Roe, id: string, sd: -1 | 1, look: Look, len = 0.135, rows: readonly (readonly number[])[] = EAR_T, o: { shape?: string; splay?: number; z0?: number } = {}): Json {
  const bone = `ear-${sx(sd)}`;
  const f = S.frame(bone);
  const b: V3 = [f.o[0]!, f.o[1]!, sd * (o.z0 ?? 0.03)];
  const k = o.splay ?? 0.42;
  const n = Math.hypot(1, k);
  const d: V3 = [f.u[0]! / n, f.u[1]! / n, (sd * k) / n];
  const pts: PP[] = rows.map(([t, rl, rs]) => ({ p: [b[0] + d[0] * len * t!, b[1] + d[1] * len * t!, b[2] + d[2] * len * t!] as V3, rl: rl!, rs: rs!, e: 2.3 }));
  return S.run(id, bone, pts, look, { lat: [0.5, -0.1, sd * 0.86], ...(o.shape === undefined ? {} : { shape: o.shape }) });
}

/** 🦌️ Point of the antler of side `sd`: `t` along the beam (0 burr, 1 tip of the top tine), in the head frame: the beam leans a little back and outward and bows forward in its lower half. */
export function beamAt(S: Roe, sd: -1 | 1, t: number, len: number): V3 {
  const f = S.frame("head");
  const base = S.at("head", 0.047, 0.048, sd * 0.021);
  const up: V3 = [-0.33 * f.u[0]! + 0.944 * f.v[0]!, -0.33 * f.u[1]! + 0.944 * f.v[1]!, 0];
  const bow = 0.014 * Math.sin(Math.PI * Math.min(1, t * 1.25)) - 0.012 * t * t;
  const out = 0.05 * (1 - (1 - t) * (1 - t)) - 0.012 * t * t * t;
  return [base[0] + up[0] * len * t + f.u[0]! * bow, base[1] + up[1] * len * t + f.u[1]! * bow, base[2] + sd * out];
}

export function antler(S: Roe, sd: -1 | 1, look: Look): void {
  const n = side(sd);
  const f = S.frame("head");
  const L = 0.2;
  const shape = `${n}-antler`;
  const dir = (a: number, b: number, z: number): V3 => [a * f.u[0]! + b * f.v[0]!, a * f.u[1]! + b * f.v[1]!, sd * z];
  const from = (p: V3, d: V3, l: number): V3 => [p[0] + d[0] * l, p[1] + d[1] * l, p[2] + d[2] * l];
  S.run(`${n}-antler-pedicle`, "head", [{ p: S.at("head", 0.05, 0.03, sd * 0.02), rl: 0.0115, rs: 0.0115 }, { p: beamAt(S, sd, 0.04, L), rl: 0.0105, rs: 0.0105 }], FACE, { shape: "head" });
  S.run(`${n}-antler-burr`, "head", [{ p: beamAt(S, sd, 0.035, L), rl: 0.012, rs: 0.012 }, { p: beamAt(S, sd, 0.06, L), rl: 0.0165, rs: 0.0165 }, { p: beamAt(S, sd, 0.1, L), rl: 0.0165, rs: 0.0165 }, { p: beamAt(S, sd, 0.125, L), rl: 0.011, rs: 0.011 }], look, { shape });
  S.run(`${n}-antler-beam`, "head", ([[0.09, 0.0105], [0.2, 0.0098], [0.35, 0.009], [0.5, 0.0088], [0.62, 0.008], [0.75, 0.0078], [0.86, 0.0062], [0.95, 0.005], [1, 0.0042]] as const).map(([t, r]) => ({ p: beamAt(S, sd, t, L), rl: r, rs: r })), look, { shape });
  const b1 = beamAt(S, sd, 0.5, L);
  const d1 = dir(0.78, 0.62, 0.05);
  S.run(`${n}-antler-brow`, "head", [{ p: from(b1, d1, -0.004), rl: 0.0075, rs: 0.0075 }, { p: from(b1, d1, 0.022), rl: 0.0062, rs: 0.0062 }, { p: from(from(b1, d1, 0.042), dir(0, 1, 0), 0.004), rl: 0.0048, rs: 0.0048 }, { p: from(from(b1, d1, 0.054), dir(0, 1, 0), 0.009), rl: 0.0042, rs: 0.0042 }], look, { shape });
  const b2 = beamAt(S, sd, 0.76, L);
  const d2 = dir(-0.74, 0.67, 0.02);
  S.run(`${n}-antler-back`, "head", [{ p: from(b2, d2, -0.004), rl: 0.007, rs: 0.007 }, { p: from(b2, d2, 0.02), rl: 0.0058, rs: 0.0058 }, { p: from(from(b2, d2, 0.038), dir(0, 1, 0), 0.003), rl: 0.0046, rs: 0.0046 }, { p: from(from(b2, d2, 0.048), dir(0, 1, 0), 0.007), rl: 0.0042, rs: 0.0042 }], look, { shape });
}

/** ✂️ Piece of the trunk table between `lo` and `hi`: full size where the piece owns the surface (`own0` to `own1`), 5 mm under the neighbour's surface where it reaches into the neighbour, so that no two trunk surfaces coincide and a flexed spine stays closed. */
export const piece = (T: number[][], lo: number, hi: number, own0: number, own1: number, tuck = 0.005): number[][] =>
  T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9).map((r) => (r[0]! < own0 - 1e-9 || r[0]! > own1 + 1e-9 ? [r[0]!, r[1]!, r[2]! - tuck, r[3]! - tuck, r[4]!] : [...r]));

/** 🥚️ Closes both ends of a limb table with two shrinking rows each, so that a segment ends in a rounded dome inside its neighbour instead of a flat cut. */
export const domed = (rows: readonly (readonly number[])[], start = true, end = true): number[][] => {
  const out = rows.map((r) => [...r]);
  const cap = (r: number[], dir: number): number[][] => {
    const m = Math.min(r[2]!, r[3]!);
    return [[r[0]! + dir * 0.5 * m, r[1]!, r[2]! * 0.82, r[3]! * 0.82, ...r.slice(4)], [r[0]! + dir * 0.85 * m, r[1]!, r[2]! * 0.45, r[3]! * 0.45, ...r.slice(4)]];
  };
  const first = out[0]!;
  const last = out[out.length - 1]!;
  return [...(start ? cap(first, -1).reverse() : []), ...out, ...(end ? cap(last, 1) : [])];
};

export function body(S: Roe): void {
  const T = dense(lines(TRUNK_LINES), 2);
  S.hull("croup", "pelvis", piece(T, -0.2, 0.15, -1, 0.1), COAT, { shape: "croup" });
  S.hull("hindbody", "lumbar", piece(T, 0.1, 0.35, 0.1, 0.3), COAT, { shape: "hindbody" });
  S.hull("chest", "thorax", piece(T, 0.25, 0.7, 0.3, 1), COAT, { shape: "chest" });
  S.limb("neck", "neck", dense(NECK, 2), COAT, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(SKULL, 2), FACE, { shape: "head", e: 2.5 });
  S.limb("jaw", "jaw", dense(MANDIBLE, 2), FACE, { frame: "head", shape: "jaw", e: 2.3 });
  const hu = S.frame("head").u;
  S.ball("nose-leather", "head", S.at("head", 0.2085, -0.003, 0), [0.013, 0.018, 0.0168], NOSE, [hu[0]!, hu[1]!, 0], "nose-leather");
  S.ball("rump-patch", "pelvis", [-0.06, 0.672, 0], [0.056, 0.07, 0.058], WHITE, [1, 0, 0], "rump-patch");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.ball(`${n}-eye`, "head", S.at("head", 0.089, 0.03, sd * 0.0405), [0.0041, 0.008, 0.008], EYE, [0, 0, sd], sd < 0 ? "eye" : "eye-f");
    ear(S, `${n}-ear`, sd, EARL, 0.135, EAR_T, { shape: `${n}-ear` });
    antler(S, sd, ANTLER);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(domed(THIGH), 2), COAT, { sd, shape: `${n}-thigh`, e: 2.0 });
    S.limb(`${n}-shank`, `shank-h${k}`, dense(domed(GASKIN, true, false), 2), LEG, { sd, shape: `${n}-shank` });
    const hc = `cannon-h${k}`;
    S.run(`${n}-hock-point`, hc, [{ p: S.at(hc, 0.035, -0.004, sd * 0.068), rl: 0.01, rs: 0.0115 }, { p: S.at(hc, 0.006, -0.013, sd * 0.068), rl: 0.0105, rs: 0.0125 }, { p: S.at(hc, -0.02, -0.021, sd * 0.068), rl: 0.0095, rs: 0.011 }, { p: S.at(hc, -0.031, -0.024, sd * 0.068), rl: 0.0065, rs: 0.0075 }], LEG, { shape: `${n}-hind-cannon` });
    S.limb(`${n}-hind-cannon`, hc, domed(HIND_CANNON), LEG, { sd, shape: `${n}-hind-cannon` });
    S.limb(`${n}-scapula`, `scapula-f${k}`, dense(domed(SCAPULA), 2), COAT, { sd, shape: `${n}-scapula`, e: 2.0 });
    S.limb(`${n}-humerus`, `humerus-f${k}`, dense(domed(UPPER_ARM), 2), COAT, { sd, shape: `${n}-humerus`, e: 2.0 });
    S.limb(`${n}-forearm`, `forearm-f${k}`, dense(domed(FOREARM), 2), LEG, { sd, shape: `${n}-forearm` });
    S.limb(`${n}-fore-cannon`, `cannon-f${k}`, domed(FORE_CANNON), LEG, { sd, shape: `${n}-fore-cannon` });
    for (const [leg, cannon, s, z, grow] of [["hind", hc, 0.232, HIND_Z, 1], ["fore", `cannon-f${k}`, 0.17, FORE_Z, 1.01]] as const) {
      const h = leg === "hind" ? "h" : "f";
      const hoof = `hoof-${h}${k}`;
      const cu = S.frame(cannon).u;
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.ball(`${n}-${leg}-dewclaw-${j}`, cannon, S.at(cannon, s, -0.0155, sd * (z + 0.002) + w * 0.0085), [0.0062, 0.0043, 0.0043], HORN, [cu[0]!, cu[1]!, 0], `${n}-${leg}-cannon`);
      S.limb(`${n}-${leg}-pastern`, hoof, domed(PASTERN.map((r) => [...r, z])), LEG, { sd, shape: `${n}-${leg}-hoof` });
      const f = S.frame(hoof);
      const len = S.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
      const toe = f.o[0]! + f.u[0]! * len;
      const ground = f.o[1]! + f.u[1]! * len;
      for (const [j, w] of [[1, -1], [2, 1]] as const) S.hull(`${n}-${leg}-claw-${j}`, hoof, CLAW.map((r) => [r[0] * grow, r[1] * grow + ground, r[2] * grow, r[3] * grow, r[4], sd * z + w * r[5] * grow]), HORN, { x0: toe, sd: 1, shape: `${n}-${leg}-hoof` });
    }
  }
}

export const REGION: readonly (readonly [RegExp, string, string?])[] = [
  [/^(croup|hindbody|chest|trunk|neck)$/, "coat-trunk", "coat"], [/^rump-patch$/, "rump-patch", "coat"], [/^(head|jaw)$|-antler-pedicle$/, "coat-head", "coat"], [/-ear(-tip)?$|^ears$/, "ears", "coat"], [/^nose-leather$/, "nose-skin"], [/-eye$/, "eyes"],
  [/-(claw|dewclaw)(-\d)?$|-hoof$/, "hooves"], [/-(thigh|shank|hock-point|hock|hind-cannon|hind-pastern|hind-leg|hind-lower)$/, "coat-hindlimbs", "coat"], [/-(scapula|humerus|forearm|carpus|fore-cannon|fore-pastern|fore-upper|fore-leg|fore-lower)$/, "coat-forelimbs", "coat"],
];

/** 🌡️ Thermal list of a scale: every body solid as the surface of its region (antlers are dead bone outside the thermal model), and the outlets as own solids: nostrils on the nose leather, a patch of the back for the plume. `T` collects the extra solids. */
export function thermal(S: Roe, T: Roe, o: { nostril: V3; pair?: boolean; plume: readonly [V3, V3] }): Json[] {
  const out: Json[] = [];
  for (const b of S.body) {
    if (/-antler-(burr|beam|brow|back)$|-antler$/.test(b.id)) continue;
    const hit = REGION.find(([re]) => re.test(b.id));
    if (hit === undefined) throw new Error(`no region for ${b.id}`);
    out.push({ id: `th-${b.id}`, of: b.id, region: hit[1], ...(hit[2] === undefined ? {} : { layer: hit[2] }), ...(/-claw-\d$/.test(b.id) ? { sectionFaces: ["-u"] } : {}) });
  }
  const hu = T.frame("head").u;
  const E: Look = { material: "bare-skin", tone: 1 };
  const at = (z: number): V3 => T.at("head", 0.2175, 0.0035, z);
  if (o.pair === false) T.ball("th-breath-nose", "head", at(0), o.nostril, E, [hu[0]!, hu[1]!, 0]);
  else for (const sd of SIDES) T.ball(`th-breath-nose-${sx(sd)}`, "head", at(sd * 0.0085), o.nostril, E, [hu[0]!, hu[1]!, 0]);
  T.ball("th-plume-back", "thorax", o.plume[0], o.plume[1], { material: "summer-coat", tone: 0.2 }, [1, 0, 0]);
  for (const t of T.body) {
    if (t.id === "th-plume-back") out.push({ ...t, region: "coat-trunk", tags: ["emitter", "plume-back"] });
    else if (t.id.startsWith("th-breath-nose")) out.push({ ...t, region: "nose-skin", tags: ["emitter", "breath-nose"] });
    else throw new Error(`thermal extra ${t.id} without region`);
  }
  return out;
}

const DECISIONS = [
  {
    en: "Body at 1:50: every section stated by hand from the skeletal landmarks of a 25 kg roe buck. The trunk is three overlapping lofts on pelvis, lumbar and thoracic bones: a croup that stands above the withers, a short loin with the tucked-up flank and a deep narrow chest with the brisket between the forelegs. The thigh with its buttock mass hangs from the hip, the gaskin tapers to the tendon above the hock, the hock carries its point, the long cannon has a joint block above and the fetlock below, with two dewclaws, a sloping pastern and a cloven hoof of two claws of 44 mm; the foreleg has a flat shoulder over the ribs, the upper arm with the triceps, a forearm that tapers to the carpus and the fore cannon. Long neck with a deep base, skull with separate mandible, black nose leather, eyes, leaf ears of 135 mm and a pair of six-point antlers of 0.2 m on pedicles with burrs, beam, brow tine and back tine. The white rump patch is a shallow cap on the buttocks; the roe has no visible tail. Segments overlap at every joint so that poses stay closed.",
    de: "Körper bei 1:50: jeder Schnitt von Hand nach den Skelettpunkten eines 25 kg schweren Rehbocks gesetzt. Der Rumpf besteht aus drei überlappenden Lofts an Becken-, Lenden- und Brustknochen: eine Kruppe, die über dem Widerrist steht, eine kurze Lende mit aufgezogener Flanke und eine tiefe schmale Brust mit dem Brustbein zwischen den Vorderläufen. Die Keule mit der Sitzbeinmasse hängt an der Hüfte, der Unterschenkel verjüngt sich zur Sehne über dem Sprunggelenk, das Sprunggelenk trägt seinen Fersenhöcker, die lange Röhre hat oben einen Gelenkblock und unten den Fesselkopf, mit zwei Afterklauen, schräger Fessel und einer gespaltenen Schale aus zwei Klauen von 44 mm; der Vorderlauf hat eine flache Schulter über den Rippen, den Oberarm mit dem Trizeps, einen zur Vorderfußwurzel verjüngten Unterarm und die vordere Röhre. Langer Hals mit tiefer Basis, Schädel mit eigenem Unterkiefer, schwarzer Nasenspiegel, Augen, Blattohren von 135 mm und ein Sechsergehörn von 0,2 m auf Rosenstöcken mit Rosen, Stange, Vordersprosse und Hintersprosse. Der weiße Spiegel ist eine flache Kappe auf den Keulen; das Reh hat keinen sichtbaren Wedel. Die Segmente überlappen an jedem Gelenk, damit Posen geschlossen bleiben.",
  },
  {
    en: "Thermal at 1:50: every region declares one layer, so each region is the body surface it covers (references to the body solids): trunk coat with croup, loin, chest and neck, the white rump patch, head coat with jaw and pedicles, foreleg and hindleg coats down to the pasterns, ears, nose leather, eyes and the horn of claws and dewclaws. The antlers are dead bone outside the thermal model. The outlets are patches on the surface: two nostrils on the nose leather and a patch of the back for the plume.",
    de: "Thermik bei 1:50: jede Region hat eine deklarierte Schicht, daher ist jede Region die Körperoberfläche, die sie bedeckt (Verweise auf die Körper): Rumpfdecke mit Kruppe, Lende, Brust und Hals, der weiße Spiegel, Kopffell mit Kiefer und Rosenstöcken, Fell der Vorder- und Hinterläufe bis zu den Fesseln, Lauscher, Nasenspiegel, Augen und das Horn der Schalen und Afterklauen. Das Gehörn ist toter Knochen außerhalb des Wärmemodells. Die Auslässe sind Flächen auf der Oberfläche: zwei Nasenlöcher auf dem Nasenspiegel und ein Rückenfeld für die Warmluftfahne.",
  },
  {
    en: "Structure at 1:50: the skeleton. Skull with the antler pedicles and mandible, cervical spine in its S curve, thoracic spine with one spinous process per vertebra (the long ones of the second to fifth carry the withers), thirteen ribs per side, the first eight reaching the sternum through their costal cartilages, the others joined in the costal arch, lumbar spine with a spinous and two transverse processes per vertebra, sacrum, ilium, ischium and pubis, ear cartilages; per foreleg scapula with its spine, humerus, radius with olecranon, carpus and metacarpal, two digits with pedal bones; per hind leg femur with patella, tibia, calcaneus, tarsus and metatarsal, two digits with pedal bones. Joints sit at the rig joints at the size of the joint, the masses of chest, loin and pelvis at their centres of mass.",
    de: "Tragwerk bei 1:50: das Skelett. Schädel mit den Rosenstöcken und Unterkiefer, Halswirbelsäule in ihrer S-Kurve, Brustwirbelsäule mit einem Dornfortsatz je Wirbel (die langen des zweiten bis fünften tragen den Widerrist), dreizehn Rippen je Seite, die ersten acht über ihre Rippenknorpel am Brustbein, die übrigen im Rippenbogen verbunden, Lendenwirbelsäule mit einem Dorn- und zwei Querfortsätzen je Wirbel, Kreuzbein, Darmbein, Sitzbein und Schambein, Ohrknorpel; je Vorderlauf Schulterblatt mit Gräte, Oberarmbein, Speiche mit Ellbogenhöcker, Vorderfußwurzel und Röhrbein, zwei Zehen mit Klauenbeinen; je Hinterlauf Oberschenkelbein mit Kniescheibe, Schienbein, Fersenbein, Fußwurzel und Röhrbein, zwei Zehen mit Klauenbeinen. Gelenke sitzen an den Gelenken des Skeletts in Gelenkgröße, die Massen von Brust, Lende und Becken in ihren Schwerpunkten.",
  },
];

if (import.meta.main) {
  const { structure } = await import("./r3-roe-deer-bones.ts");
  const S = new Roe(SPEC, 50);
  body(S);
  const th = thermal(S, new Roe(SPEC, 50), { nostril: [0.0042, 0.0045, 0.0055], plume: [[0.36, 0.741, 0], [0.11, 0.05, 0.012]] });
  const st = structure(new Roe(SPEC, 50), 50, (id) => S.body.some((b) => b.id === id));
  console.log(`1:50 body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS });
    writeFileSync(`${OUT}/shapes.50.json`, JSON.stringify(S.shapes));
  }
}
