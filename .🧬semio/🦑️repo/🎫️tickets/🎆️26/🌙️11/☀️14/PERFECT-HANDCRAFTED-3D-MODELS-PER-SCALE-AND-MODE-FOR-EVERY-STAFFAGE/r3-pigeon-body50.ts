/** 🐦️ Round 3 pigeon, body at 1:50: every mass of the standing rock pigeon stated section by section in world metres (default pose), after the skeleton of Johnston (1990) and the external measures of Gibbs et al. (2001).
 * @see ./r3-pigeon-kit.ts */
import { Bench, add, lerp, mix, mul, sideBone, sideName, smooth, sub, unit, type Look, type PP, type Side, type Station, type TrunkStation, type V3 } from "./r3-pigeon-kit.ts";

export const PLUMAGE: Look = { material: "plumage", tone: 0.72 };
export const WING: Look = { material: "plumage", tone: 0.6 };
export const QUILL: Look = { material: "plumage", tone: 0.5 };
export const TAIL: Look = { material: "plumage", tone: 0.62 };
export const SKIN: Look = { material: "bare-skin", tone: 0.66 };
export const HORN: Look = { material: "beak-keratin", tone: 0.3 };
export const CERE: Look = { material: "cere-wax", tone: 0.9 };
export const EYE: Look = { material: "eye-tissue", tone: 0.04, role: "detail" };

/** 🥚️ Trunk envelope with its plumage: [s along the column from the tail base, dorsal offset, ventral offset, half width]. Deep keeled chest, level belly just above the ankles, back 10 mm over the column. */
export const TRUNK_50: readonly TrunkStation[] = [
  [-0.016, 0.004, -0.012, 0.011],
  [0.0, 0.007, -0.017, 0.018],
  [0.02, 0.009, -0.03, 0.027],
  [0.045, 0.01, -0.049, 0.035],
  [0.07, 0.011, -0.066, 0.0395],
  [0.095, 0.0115, -0.076, 0.041],
  [0.118, 0.011, -0.079, 0.0405],
  [0.138, 0.01, -0.075, 0.038],
  [0.155, 0.008, -0.064, 0.033],
  [0.168, 0.004, -0.05, 0.0265],
  [0.178, -0.004, -0.038, 0.019],
  [0.184, -0.014, -0.028, 0.009],
];

export function trunk(S: Bench, hand: readonly TrunkStation[], id = "body", fine = 1): void {
  const stations = fine > 1 ? smooth(hand, fine) : hand;
  S.trunkStations = stations;
  S.loft(id, "trunk", stations.map(([s, td, tv, half]): Station => ({ c: S.T(s, (td + tv) / 2), a: [0, 0, half], b: mul(S.dorsal, (td - tv) / 2), e: S.trunkExponent })), PLUMAGE);
}

/** 🦢️ One neck link: stations [x, y, sagittal half depth, half width]. */
export function neckLink(S: Bench, bone: string, st: readonly (readonly [number, number, number, number])[]): void {
  S.path(bone, bone, st.map(([x, y, rs, rl]): PP => ({ p: [x, y, 0], rl, rs, e: 2 })), PLUMAGE);
}

/** 🪢️ Round 4: the neck as one smooth hand-stated form from the breast to the nape, carried by the three neck bones. The stated stations (`i2` at the joint of the second, `i3` at the joint of the third neck bone) are cut at the two joints into three links that share the joint section and its direction exactly, so that the default pose shows one continuous neck with a hairline at each joint instead of rings or ragged overlaps; beyond its joint section every link runs on 3 mm inside its neighbour as a dome, which keeps every pose closed where the neck bends. */
export function neckChain(S: Bench, hand: readonly (readonly [number, number, number, number])[], i2: number, i3: number, fine: number): void {
  const st = smooth(hand, fine);
  const [k2, k3] = [i2 * fine, i3 * fine];
  const last = st.length - 1;
  const dir = (k: number): V3 => unit(sub([st[Math.min(k + 1, last)]![0], st[Math.min(k + 1, last)]![1], 0], [st[Math.max(k - 1, 0)]![0], st[Math.max(k - 1, 0)]![1], 0]));
  const row = (k: number, shrink = 0, scale = 1, along = 0): Station => {
    const [x, y, rs, rl] = st[k]!;
    const t = dir(k);
    return { c: [x + t[0] * along, y + t[1] * along, 0], a: [0, 0, Math.max((rl - shrink) * scale, 0.002)], b: mul([t[1], -t[0], 0], Math.max((rs - shrink) * scale, 0.002)), e: 2 };
  };
  const full = (from: number, to: number): Station[] => Array.from({ length: to - from + 1 }, (_, n) => row(from + n));
  const after = (k: number): Station[] => [row(k, 0.003, 1, 0.0008), row(k + 1, 0.003), row(Math.min(k + 2, last), 0, 0.4)];
  const before = (k: number): Station[] => [row(k - 2, 0, 0.4), row(k - 1, 0.003), row(k, 0.003, 1, -0.0008)];
  S.loft("neck-1", "neck-1", [...full(0, k2), ...after(k2)], PLUMAGE);
  S.loft("neck-2", "neck-2", [...before(k2), ...full(k2, k3), ...after(k3)], PLUMAGE);
  S.loft("neck-3", "neck-3", [...before(k3), ...full(k3, last)], PLUMAGE);
}

/** 🧠️ Head-local point: `a` along the bill axis from the skull joint, `b` towards the crown, `z` lateral. */
export const H = (S: Bench, a: number, b: number, z = 0): V3 => S.J("head", a, b, z);

export function headLoft(S: Bench, id: string, bone: string, hand: readonly (readonly [number, number, number, number])[], look: Look, e = 2, fine = 1): void {
  const st = fine > 1 ? smooth(hand, fine) : hand;
  const f = S.frame("head");
  S.loft(id, bone, st.map(([a, b, rs, rl]): Station => ({ c: H(S, a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e })), look);
}

export function head50(S: Bench): void {
  headLoft(S, "head", "head", [
    [-0.0205, 0.0015, 0.007, 0.0072],
    [-0.016, 0.002, 0.0126, 0.013],
    [-0.008, 0.0026, 0.016, 0.0162],
    [0.002, 0.0032, 0.0172, 0.0164],
    [0.012, 0.0038, 0.0172, 0.016],
    [0.021, 0.0045, 0.0162, 0.0152],
    [0.028, 0.004, 0.0142, 0.013],
    [0.0335, 0.0025, 0.0105, 0.0092],
    [0.037, 0.001, 0.006, 0.005],
  ], PLUMAGE, 2, 2);
  headLoft(S, "beak", "head", [
    [0.031, 0.0002, 0.0034, 0.004],
    [0.036, 0.0003, 0.003, 0.0036],
    [0.041, 0.0, 0.0026, 0.003],
    [0.047, -0.0006, 0.0021, 0.0024],
    [0.051, -0.0012, 0.0016, 0.0018],
    [0.0535, -0.0019, 0.001, 0.001],
    [0.0545, -0.0024, 0.0004, 0.0004],
  ], HORN, 2.2);
  headLoft(S, "jaw", "jaw", [
    [0.03, -0.0045, 0.0019, 0.0034],
    [0.036, -0.0044, 0.0018, 0.003],
    [0.043, -0.0041, 0.0015, 0.0024],
    [0.049, -0.0038, 0.0011, 0.0017],
    [0.0528, -0.0034, 0.0004, 0.0005],
  ], HORN, 2.2);
  headLoft(S, "cere", "head", [
    [0.034, 0.0028, 0.0012, 0.0026],
    [0.0365, 0.0036, 0.002, 0.0037],
    [0.0395, 0.0035, 0.0021, 0.0037],
    [0.0425, 0.0028, 0.0015, 0.003],
    [0.0445, 0.0019, 0.0008, 0.0018],
  ], CERE, 2.4);
  const f = S.frame("head");
  for (const sd of [-1, 1] as const) S.ell(sd < 0 ? "eye" : "eye-f", "head", H(S, 0.022, 0.006, sd * 0.0128), [0.0046, 0.0046, 0.004], EYE, [f.u as V3, f.v as V3]);
}

/** 🪽️ Folded wing of one side. Arm wing (coverts, secondaries, tertials; forearm): [x, top y, bottom y, half thickness]; hand wing (primaries; hand): [q along the hand from the wrist, centre above the axis, half height, half thickness, lateral centre (0 = on the flank), roll]. */
export type Shield = readonly [x: number, top: number, bottom: number, th: number];
export type Blade = readonly [q: number, c: number, h: number, th: number, z: number, roll: number];

export function shield(S: Bench, id: string, bone: string, sd: Side, hand: readonly Shield[], look: Look, proud: number, e = 2.5, fine = 1): void {
  const st = fine > 1 ? smooth(hand, fine) : hand;
  S.loft(id, bone, st.map(([x, top, bottom, th]): Station => {
    const zt = S.flank(x, top - 0.002) || S.flank(x, (top + bottom) / 2) * 0.6;
    const zb = S.flank(x, bottom + 0.002) || zt;
    const rho = 0.8 * Math.atan2(zb - zt, top - bottom);
    const up: V3 = [0, Math.cos(rho), -sd * Math.sin(rho)];
    const out: V3 = [0, Math.sin(rho), sd * Math.cos(rho)];
    const mid: V3 = [x, (top + bottom) / 2, (sd * (zt + zb)) / 2];
    return { c: add(mid, mul(out, proud - th)), a: mul(out, th), b: mul(up, (top - bottom) / 2 / Math.cos(rho)), e };
  }), look);
}

export function blade(S: Bench, id: string, bone: string, sd: Side, st: readonly Blade[], look: Look, carrier = bone): void {
  const wrist = S.J(bone);
  const d = S.frame(bone).u as V3;
  const up: V3 = [d[1], -d[0], 0];
  S.loft(id, carrier, st.map(([q, c, h, th, z, roll]): Station => {
    const p = add(add(wrist, mul(d, q)), mul(up, c));
    const zc = z > 0 ? z : S.flank(p[0], p[1]) + 0.0025 + z;
    const b: V3 = [up[0] * Math.cos(roll), up[1] * Math.cos(roll), -sd * Math.sin(roll)];
    const a: V3 = [up[0] * Math.sin(roll), up[1] * Math.sin(roll), sd * Math.cos(roll)];
    return { c: [p[0], p[1], sd * zc], a: mul(a, th), b: mul(b, h), e: 2.6 };
  }), look);
}

export const SHIELD_50: readonly Shield[] = [
  [0.0625, 0.104, 0.094, 0.002],
  [0.0605, 0.11, 0.089, 0.0045],
  [0.056, 0.119, 0.086, 0.0075],
  [0.045, 0.127, 0.085, 0.0095],
  [0.025, 0.1255, 0.081, 0.01],
  [0.0, 0.1145, 0.0765, 0.0095],
  [-0.025, 0.1005, 0.072, 0.008],
  [-0.05, 0.0865, 0.0675, 0.006],
  [-0.072, 0.0745, 0.064, 0.004],
  [-0.086, 0.0675, 0.0635, 0.0015],
];
export const BLADE_50: readonly Blade[] = [
  [-0.008, -0.002, 0.006, 0.003, 0, 0.3],
  [0.006, -0.0015, 0.011, 0.0042, 0, 0.3],
  [0.05, -0.001, 0.0125, 0.004, 0, 0.3],
  [0.1, 0.0, 0.0115, 0.0034, 0.028, 0.38],
  [0.14, 0.0005, 0.009, 0.0027, 0.02, 0.5],
  [0.17, 0.0005, 0.0062, 0.002, 0.0145, 0.65],
  [0.19, 0.0, 0.0035, 0.0014, 0.011, 0.8],
  [0.2, 0.0, 0.0012, 0.0008, 0.0095, 0.85],
];
export const SCAPULARS_50: readonly Shield[] = [
  [0.054, 0.1305, 0.1265, 0.0006],
  [0.049, 0.1355, 0.1235, 0.0028],
  [0.04, 0.1385, 0.1215, 0.0045],
  [0.02, 0.1305, 0.1105, 0.0055],
  [-0.005, 0.117, 0.097, 0.0055],
  [-0.03, 0.103, 0.0855, 0.0045],
  [-0.05, 0.091, 0.0785, 0.003],
  [-0.062, 0.084, 0.0775, 0.0012],
];

export function wing50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  shield(S, `${n}-coverts`, `forearm-${b}`, sd, SHIELD_50, WING, 0.0085, 2.5, 2);
  blade(S, `${n}-primaries`, `hand-${b}`, sd, BLADE_50, QUILL);
  shield(S, `${n}-shoulder`, `humerus-${b}`, sd, SCAPULARS_50, WING, 0.003, 2.5, 2);
}

/** 🪶️ Tail: [q along the tail from its base, offset above the axis, half width, half thickness]. */
export type Feather = readonly [q: number, up: number, half: number, th: number];
export function fan(S: Bench, id: string, bone: string, st: readonly Feather[], look: Look, e = 3): void {
  const f = S.frame("tail");
  const up = mul(f.v as V3, -1);
  S.loft(id, bone, st.map(([q, o, half, th]): Station => ({ c: add(S.J("tail", q), mul(up, o)), a: [0, 0, half], b: mul(up, th), e })), look);
}

export function tail50(S: Bench): void {
  fan(S, "tail", "tail", [
    [-0.012, 0.0015, 0.012, 0.005],
    [0.01, 0.0015, 0.014, 0.0048],
    [0.04, 0.0015, 0.0165, 0.004],
    [0.07, 0.0015, 0.0185, 0.0032],
    [0.092, 0.0012, 0.0195, 0.0025],
    [0.101, 0.001, 0.0165, 0.002],
    [0.105, 0.001, 0.009, 0.0012],
  ], TAIL);
  fan(S, "far-tail", "tail-f", [
    [-0.008, -0.0025, 0.014, 0.0045],
    [0.02, -0.0025, 0.0175, 0.0042],
    [0.05, -0.0025, 0.021, 0.0036],
    [0.078, -0.0022, 0.0235, 0.003],
    [0.092, -0.002, 0.022, 0.0024],
    [0.099, -0.0018, 0.013, 0.0014],
  ], { ...TAIL, tone: 0.55 });
  fan(S, "undertail", "trunk", [
    [-0.03, -0.014, 0.016, 0.0115],
    [-0.01, -0.0125, 0.0175, 0.0105],
    [0.012, -0.0105, 0.0165, 0.0078],
    [0.034, -0.008, 0.0135, 0.0045],
    [0.05, -0.0058, 0.008, 0.0016],
  ], PLUMAGE, 2.2);
}

/** 🦵️ Leg of one side, joint to joint: thigh mass inside the flank, feathered drumstick, bare shank end with the ankle, scaled tarsus, three front toes and the hallux flat on the ground. */
export type Ring = readonly [f: number, rAhead: number, rLateral: number];
export function limb(S: Bench, id: string, bone: string, a: V3, b: V3, st: readonly Ring[], look: Look, z0: number, z1: number, e = 2): void {
  const d = sub(b, a);
  S.path(id, bone, st.map(([f, rs, rl]): PP => ({ p: [a[0] + d[0] * f, a[1] + d[1] * f, lerp(z0, z1, Math.max(0, Math.min(1, f)))], rl, rs, e })), look);
}

export type Toe = { id: string; yawDeg: number; st: readonly (readonly [number, number, number, number])[] };
export function toe(S: Bench, id: string, bone: string, m: V3, sd: Side, yawDeg: number, st: readonly (readonly [number, number, number, number])[], look: Look): void {
  const yaw = (yawDeg * Math.PI) / 180;
  const dir: V3 = [Math.cos(yaw), 0, -sd * Math.sin(yaw)];
  const lat: V3 = [-dir[2], 0, dir[0]];
  S.path(id, bone, st.map(([q, y, half, th]): PP => ({ p: [m[0] + dir[0] * q, y, m[2] + dir[2] * q], rl: half, rs: th, e: 2.2 })), look, lat);
}

export function leg50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`toes-${b}`)];
  const zr = hip[2];
  limb(S, `${n}-thigh`, `thigh-${b}`, hip, knee, [[-0.34, 0.0035, 0.003], [-0.26, 0.0075, 0.0065], [-0.12, 0.0105, 0.009], [0.1, 0.0155, 0.0125], [0.45, 0.016, 0.0135], [0.8, 0.0135, 0.012], [1.0, 0.0112, 0.0105], [1.14, 0.0065, 0.006]], PLUMAGE, zr + sd * 0.006, zr + sd * 0.005);
  limb(S, `${n}-drumstick`, `shank-${b}`, knee, ankle, [[-0.17, 0.003, 0.003], [-0.1, 0.0065, 0.006], [0.0, 0.0105, 0.01], [0.25, 0.0098, 0.0092], [0.5, 0.0075, 0.007], [0.7, 0.0052, 0.0048], [0.82, 0.0034, 0.0032], [0.87, 0.0022, 0.0021]], PLUMAGE, zr + sd * 0.005, zr + sd * 0.001);
  limb(S, `${n}-shank`, `shank-${b}`, knee, ankle, [[0.76, 0.0021, 0.002], [0.88, 0.0023, 0.0021], [0.96, 0.0029, 0.0025], [1.0, 0.0032, 0.0028], [1.035, 0.002, 0.0018]], SKIN, zr, zr);
  limb(S, `${n}-tarsus`, `tarsus-${b}`, ankle, mtp, [[-0.05, 0.0022, 0.002], [0.04, 0.003, 0.0026], [0.2, 0.0023, 0.002], [0.8, 0.0022, 0.0019], [0.9, 0.0026, 0.0026], [1.0, 0.0024, 0.0024]], SKIN, zr, zr);
  toe(S, `${n}-toes`, `toes-${b}`, mtp, sd, 0, [[-0.0035, 0.0026, 0.0016, 0.0016], [0.0, 0.0026, 0.0024, 0.0024], [0.006, 0.0021, 0.0021, 0.002], [0.016, 0.0019, 0.002, 0.0018], [0.026, 0.0017, 0.0017, 0.0015], [0.0305, 0.0016, 0.0011, 0.0011], [0.0352, 0.0006, 0.0003, 0.0003]], SKIN);
  toe(S, `${n}-toe-inner`, `toes-${b}`, mtp, sd, -38, [[0.0, 0.0025, 0.0021, 0.0021], [0.006, 0.002, 0.0019, 0.0019], [0.014, 0.0017, 0.0017, 0.0016], [0.0205, 0.0015, 0.0013, 0.0013], [0.0225, 0.0015, 0.001, 0.001], [0.0265, 0.0005, 0.0003, 0.0003]], SKIN);
  toe(S, `${n}-toe-outer`, `toes-${b}`, mtp, sd, 36, [[0.0, 0.0025, 0.0021, 0.0021], [0.006, 0.002, 0.0019, 0.0019], [0.015, 0.0017, 0.0017, 0.0016], [0.0225, 0.0015, 0.0013, 0.0013], [0.0245, 0.0015, 0.001, 0.001], [0.0285, 0.0005, 0.0003, 0.0003]], SKIN);
  toe(S, `${n}-hallux`, `toes-${b}`, mtp, sd, 192, [[0.0, 0.0025, 0.002, 0.002], [0.005, 0.0019, 0.0017, 0.0017], [0.011, 0.0015, 0.0014, 0.0014], [0.013, 0.0015, 0.001, 0.001], [0.0165, 0.0005, 0.0003, 0.0003]], SKIN);
}

export function body50(S: Bench): void {
  trunk(S, TRUNK_50, "body", 3);
  neckChain(S, [[0.05, 0.103, 0.014, 0.012], [0.054, 0.109, 0.027, 0.024], [0.06, 0.118, 0.034, 0.03], [0.069, 0.1315, 0.0335, 0.0295], [0.0765, 0.1435, 0.03, 0.0265], [0.0812, 0.155, 0.0258, 0.0232], [0.0839, 0.1648, 0.0226, 0.0204], [0.0838, 0.178, 0.0204, 0.0185], [0.0822, 0.1897, 0.0188, 0.017], [0.087, 0.1965, 0.018, 0.0162], [0.0915, 0.202, 0.0146, 0.0128], [0.0955, 0.207, 0.005, 0.005]], 6, 8, 2);
  head50(S);
  tail50(S);
  for (const sd of [-1, 1] as const) {
    wing50(S, sd);
    leg50(S, sd);
  }
}
