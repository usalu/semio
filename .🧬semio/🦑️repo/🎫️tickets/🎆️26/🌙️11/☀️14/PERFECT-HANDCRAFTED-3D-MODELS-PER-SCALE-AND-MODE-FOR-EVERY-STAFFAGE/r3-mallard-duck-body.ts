/** 🦆️ Round 3 mallard, body at 1:50 and the shaping helpers of every scale: each mass of the standing drake stated section by section by hand (trunk frame: `s` along the column from the hips, `t` towards the back; bone frames for neck, head, tail, legs and feet), after the biometrics of Cramp & Simmons (1977) and the long bones of Woelfle (1967).
 * @see ./r3-mallard-duck-kit.ts */
import { Bench, add, cross, dot, lerp, mul, sideBone, sideName, smooth, sub, unit, type HullStation, type Look, type PP, type Side, type Station, type V3 } from "./r3-mallard-duck-kit.ts";

export const FLANK: Look = { material: "plumage", tone: 0.82 };
export const BREAST: Look = { material: "plumage", tone: 0.42 };
export const RUMP: Look = { material: "plumage", tone: 0.14 };
export const GREEN: Look = { material: "plumage", tone: 0.24 };
export const RING: Look = { material: "plumage", tone: 0.97 };
export const TAIL: Look = { material: "plumage", tone: 0.92 };
export const WING: Look = { material: "wing-feathers", tone: 0.62 };
export const SCAPULAR: Look = { material: "wing-feathers", tone: 0.72 };
export const QUILL: Look = { material: "wing-feathers", tone: 0.5 };
export const SPECULUM: Look = { material: "wing-feathers", tone: 0.3 };
export const BILL: Look = { material: "bill-keratin", tone: 0.78 };
export const SKIN: Look = { material: "scaled-skin", tone: 0.6 };
export const EYE: Look = { material: "eye-tissue", tone: 0.04, role: "detail" };

/** 🥚️ Plumage hull of the trunk: [s, dorsal, ventral, half width]. Back 30 to 36 mm over the hip axis (synsacrum under skin and feathers), flat belly 89 mm under it, 126 mm wide at the wings, full breast ahead of the neck base, rump tapering under the tail. */
export const HULL_50: readonly HullStation[] = [
  [-0.152, 0.0255, 0.0115, 0.006],
  [-0.137, 0.0205, -0.003, 0.014],
  [-0.12, 0.018, -0.016, 0.021],
  [-0.105, 0.017, -0.028, 0.027],
  [-0.088, 0.019, -0.04, 0.034],
  [-0.06, 0.024, -0.057, 0.044],
  [-0.03, 0.028, -0.07, 0.052],
  [0.0, 0.031, -0.079, 0.058],
  [0.03, 0.034, -0.086, 0.062],
  [0.06, 0.0365, -0.089, 0.063],
  [0.09, 0.0385, -0.088, 0.061],
  [0.115, 0.04, -0.083, 0.057],
  [0.135, 0.04, -0.076, 0.051],
  [0.153, 0.0375, -0.066, 0.044],
  [0.167, 0.031, -0.054, 0.036],
  [0.177, 0.019, -0.039, 0.025],
  [0.183, 0.004, -0.022, 0.011],
];

export const between = (st: readonly HullStation[], lo: number, hi: number): HullStation[] => st.filter((q) => q[0] >= lo - 1e-9 && q[0] <= hi + 1e-9);

/** 🎨️ The hull in its three tone zones of the drake: black rump with the tail coverts, pale grey flanks and belly, chestnut breast. Each zone runs one station into its neighbour, 1 mm inside it, so that no seam opens. */
export function hull50(S: Bench): void {
  const st = smooth(HULL_50, 3);
  S.hull = st;
  S.hullLoft("body", between(st, -0.062, 0.127), FLANK);
  S.hullLoft("rump", between(st, -0.2, -0.05), RUMP, 0.0006);
  S.hullLoft("breast", between(st, 0.114, 0.2), BREAST, 0.0006);
}

/** 🦢️ One neck link in its bone frame: stations [a along the bone, b across it (towards the nape), sagittal half depth, half width]. */
export type NeckStation = readonly [a: number, b: number, rs: number, rl: number];
export function link(S: Bench, id: string, bone: string, hand: readonly NeckStation[], look: Look, fine = 1, e = 2): void {
  const st = fine > 1 ? smooth(hand, fine) : hand;
  S.path(id, bone, st.map(([a, b, rs, rl]): PP => ({ p: S.J(bone, a, b), rl, rs, e })), look);
}

/** 🧠️ Loft in the head frame with sections across the bill axis: [a along the axis from the skull joint, b towards the crown, sagittal half depth, half width]. */
export function headLoft(S: Bench, id: string, hand: readonly NeckStation[], look: Look, e = 2, fine = 1): void {
  const st = fine > 1 ? smooth(hand, fine) : hand;
  const f = S.frame("head");
  S.loft(id, "head", st.map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e })), look);
}

export const HEAD_50: readonly NeckStation[] = [
  [-0.022, 0.005, 0.007, 0.007],
  [-0.015, 0.0075, 0.0165, 0.015],
  [-0.003, 0.0105, 0.0222, 0.0186],
  [0.011, 0.0118, 0.0234, 0.0196],
  [0.025, 0.0104, 0.0214, 0.0184],
  [0.037, 0.0076, 0.0176, 0.016],
  [0.047, 0.0046, 0.0138, 0.0134],
  [0.055, 0.0024, 0.0112, 0.0114],
];
export const BILL_50: readonly NeckStation[] = [
  [0.051, 0.0026, 0.0098, 0.0098],
  [0.057, 0.002, 0.0102, 0.0108],
  [0.068, 0.001, 0.0092, 0.0112],
  [0.08, 0.0, 0.0076, 0.0117],
  [0.091, -0.001, 0.0062, 0.012],
  [0.099, -0.0016, 0.005, 0.0114],
  [0.1035, -0.002, 0.0037, 0.0092],
  [0.1056, -0.0024, 0.0022, 0.005],
];
export const EYE_AT: readonly [number, number, number] = [0.029, 0.0158, 0.0146];

export function head50(S: Bench): void {
  headLoft(S, "head", HEAD_50, GREEN, 2, 2);
  headLoft(S, "bill", BILL_50, BILL, 2.7, 2);
  const f = S.frame("head");
  for (const sd of [-1, 1] as const) S.ell(sd < 0 ? "eye" : "eye-f", "head", S.J("head", EYE_AT[0], EYE_AT[1], sd * EYE_AT[2]), [0.0042, 0.0042, 0.0036], EYE, [f.u as V3, f.v as V3]);
}

/** 🪽️ A strip of the folded wing lying on the flank: [s, upper edge t, lower edge t, half thickness, lateral override of both edges (0 = on the hull), lateral position of the upper edge alone]. Each section spans from the hull surface at the lower edge to the hull surface at the upper edge and is pushed out by the bulge of the flank between them, so the strip follows the body instead of standing off as a plate; over the rump the upper edge is carried inwards across the back (`zTop`), so the folded primaries lie on the back towards the other wing's tips. */
export type Strip = readonly [s: number, top: number, bottom: number, th: number, z?: number, zTop?: number];
export function strip(S: Bench, id: string, bone: string, sd: Side, hand: readonly Strip[], look: Look, proud: number, e = 2.0, fine = 1): void {
  const onHull = (q: Strip): number => (q[4] !== undefined && q[4] > 0 ? 1 : 0) + (q[5] !== undefined ? 1 : 0);
  const rows = hand.map((q) => {
    const zt = q[5] ?? (q[4] !== undefined && q[4] > 0 ? q[4] : S.flank(q[0], q[1] - 0.0015));
    const zb = q[4] !== undefined && q[4] > 0 ? q[4] : S.flank(q[0], q[2] + 0.0015);
    return [q[0], q[1], q[2], q[3], zt, zb, onHull(q)] as const;
  });
  const st = fine > 1 ? smooth(rows, fine) : rows;
  S.loft(id, bone, st.map(([s, t1, t0, th, zt, zb, k]): Station => {
    const [z1, z0] = Math.abs(k) > 1e-6 ? [zt, zb] : [S.flank(s, t1 - 0.0015), S.flank(s, t0 + 0.0015)];
    const zo = Math.abs(k) > 1e-6 ? 1 : undefined;
    const [top, bottom] = [S.T(s, t1, sd * z1), S.T(s, t0, sd * z0)];
    const up = unit(sub(top, bottom));
    let out = unit(cross(S.axis, up));
    if (out[2] * sd < 0) out = mul(out, -1);
    const mid = mul(add(top, bottom), 0.5);
    const bulge = zo !== undefined && zo > 0 ? 0 : Math.max(0, S.flank(s, (t1 + t0) / 2) - (z1 + z0) / 2) * Math.abs(out[2]);
    return { c: add(mid, mul(out, proud - th + 0.75 * bulge)), a: mul(out, Math.max(th, 0.0004)), b: mul(up, Math.hypot(...sub(top, bottom)) / 2), e };
  }), look);
}

export const SCAPULARS_50: readonly Strip[] = [
  [0.106, 0.02, 0.006, 0.0015],
  [0.098, 0.027, -0.004, 0.004],
  [0.078, 0.032, -0.009, 0.006],
  [0.045, 0.0325, -0.009, 0.0065],
  [0.01, 0.0285, -0.006, 0.0062],
  [-0.03, 0.025, -0.003, 0.0055],
  [-0.065, 0.021, 0.001, 0.0045],
  [-0.095, 0.0165, 0.006, 0.0032, 0.017],
  [-0.118, 0.014, 0.0095, 0.0014, 0.011],
];
export const WING_50: readonly Strip[] = [
  [0.116, -0.012, -0.022, 0.0018],
  [0.108, -0.004, -0.032, 0.005],
  [0.092, -0.001, -0.039, 0.007],
  [0.065, -0.002, -0.043, 0.0076],
  [0.03, -0.001, -0.042, 0.0076],
  [-0.01, 0.001, -0.037, 0.007],
  [-0.045, 0.003, -0.029, 0.006],
  [-0.075, 0.005, -0.019, 0.0045],
  [-0.095, 0.007, -0.011, 0.003],
  [-0.105, 0.008, -0.006, 0.0012],
];
export const SPECULUM_50: readonly Strip[] = [
  [-0.002, -0.014, -0.026, 0.0015],
  [-0.012, -0.01, -0.0335, 0.004],
  [-0.035, -0.006, -0.028, 0.0042],
  [-0.058, -0.002, -0.021, 0.004],
  [-0.072, 0.001, -0.016, 0.003],
  [-0.08, 0.002, -0.013, 0.0012],
];
export const PRIMARIES_50: readonly Strip[] = [
  [-0.036, 0.002, -0.02, 0.0025],
  [-0.05, 0.006, -0.019, 0.004],
  [-0.075, 0.014, -0.0095, 0.004],
  [-0.1, 0.0215, 0.002, 0.0036, 0, 0.013],
  [-0.12, 0.0215, 0.007, 0.003, 0, 0.0075],
  [-0.134, 0.0225, 0.011, 0.0024, 0, 0.0048],
  [-0.142, 0.0235, 0.0145, 0.0014, 0, 0.0032],
];

export function wing50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  strip(S, `${n}-primaries`, `wing-${b}`, sd, PRIMARIES_50, QUILL, 0.004, 1.9, 2);
  strip(S, `${n}-wing`, `wing-${b}`, sd, WING_50, WING, 0.0027, 2.0, 2);
  strip(S, `${n}-speculum`, `wing-${b}`, sd, SPECULUM_50, SPECULUM, 0.0038, 2.0, 2);
  strip(S, `${n}-scapulars`, `wing-${b}`, sd, SCAPULARS_50, SCAPULAR, 0.0052, 1.7, 2);
}

/** 🪶️ Tail in its bone frame: [q along the tail from the pygostyle, offset above the axis, half width, half thickness]. */
export type Feather = readonly [q: number, up: number, half: number, th: number];
export function fan(S: Bench, id: string, st: readonly Feather[], look: Look, e = 3): void {
  const up = mul(S.frame("tail").v as V3, -1);
  S.loft(id, "tail", st.map(([q, o, half, th]): Station => ({ c: add(S.J("tail", q), mul(up, o)), a: [0, 0, half], b: mul(up, th), e })), look);
}

export const TAIL_50: readonly Feather[] = [
  [-0.022, 0.002, 0.019, 0.009],
  [0.0, 0.002, 0.021, 0.008],
  [0.03, 0.002, 0.0215, 0.006],
  [0.055, 0.002, 0.018, 0.0045],
  [0.072, 0.002, 0.0125, 0.0036],
  [0.082, 0.002, 0.0065, 0.0028],
  [0.0855, 0.002, 0.0025, 0.0018],
];

/** 🌀️ The drake's curled central tail feathers: a ribbon rising from the upper tail coverts and curling forward. [q along the tail, height above its axis]. */
export const CURL_50: readonly (readonly [number, number, number, number])[] = [
  [0.002, 0.005, 0.0066, 0.0038],
  [0.015, 0.008, 0.0066, 0.0039],
  [0.027, 0.0125, 0.0064, 0.0039],
  [0.035, 0.02, 0.0062, 0.0039],
  [0.035, 0.028, 0.0058, 0.0038],
  [0.029, 0.0345, 0.0052, 0.0038],
  [0.021, 0.036, 0.0044, 0.0034],
  [0.015, 0.0335, 0.003, 0.0022],
];

export function tail50(S: Bench): void {
  fan(S, "tail", TAIL_50, TAIL);
  const up = mul(S.frame("tail").v as V3, -1);
  S.path("tail-curl", "tail", smooth(CURL_50, 2).map(([q, h, rl, rs]): PP => ({ p: add(S.J("tail", q), mul(up, h)), rl, rs, e: 2.4 })), RUMP);
}

/** 🦵️ A limb segment between two joints: [fraction of the way, half extent in the sagittal plane, half extent laterally]; the lateral position runs from `z0` to `z1`. */
export type Ring = readonly [f: number, rAhead: number, rLateral: number];
export function limb(S: Bench, id: string, bone: string, a: V3, b: V3, st: readonly Ring[], look: Look, e = 2): void {
  S.path(id, bone, st.map(([f, rs, rl]): PP => ({ p: [lerp(a[0], b[0], f), lerp(a[1], b[1], f), lerp(a[2], b[2], Math.max(0, Math.min(1, f)))], rl, rs, e })), look);
}

export const THIGH_50: readonly Ring[] = [[-0.2, 0.008, 0.008], [0.0, 0.016, 0.014], [0.4, 0.02, 0.016], [0.8, 0.018, 0.015], [1.0, 0.0145, 0.0125], [1.07, 0.007, 0.007]];
export const DRUMSTICK_50: readonly Ring[] = [[-0.12, 0.008, 0.008], [0.0, 0.0155, 0.013], [0.2, 0.0165, 0.013], [0.42, 0.0148, 0.012], [0.58, 0.0118, 0.0098], [0.68, 0.0074, 0.0064], [0.73, 0.0038, 0.0035]];
export const SHANK_50: readonly Ring[] = [[0.6, 0.0034, 0.0032], [0.72, 0.0036, 0.0034], [0.85, 0.0042, 0.0038], [0.95, 0.0058, 0.005], [1.0, 0.0062, 0.0052], [1.05, 0.004, 0.0036]];
export const TARSUS_50: readonly Ring[] = [[-0.06, 0.004, 0.0036], [0.05, 0.0058, 0.0048], [0.25, 0.0046, 0.0036], [0.75, 0.0044, 0.0035], [0.92, 0.0055, 0.0055], [1.03, 0.0042, 0.0042]];

/** 🦶️ Point of a foot frame: `a` along the foot from the toe joint, `b` above the bone axis (the sole lies 6 mm under it), `w` towards the inner side of the foot. */
export const footAt = (S: Bench, sd: Side, a: number, b: number, w = 0): V3 => S.J(`foot-${sideBone(sd)}`, a, b, -sd * w);

/** 🦆️ A toe from the toe joint: yaw towards the inner side (degrees), length, radius at its base. */
export function toe(S: Bench, id: string, sd: Side, yawDeg: number, len: number, r: number, look: Look): void {
  const [c, s] = [Math.cos((yawDeg * Math.PI) / 180), Math.sin((yawDeg * Math.PI) / 180)];
  const f = S.frame(`foot-${sideBone(sd)}`);
  const lat = unit(add(mul(f.u as V3, -s), mul(f.w as V3, -sd * c)));
  const at = (q: number, b: number): V3 => footAt(S, sd, c * q, b, s * q);
  S.path(id, `foot-${sideBone(sd)}`, [
    { p: at(-0.003, -0.0022), rl: r * 0.8, rs: r * 0.8 },
    { p: at(0.004, -0.0024), rl: r, rs: r },
    { p: at(len * 0.35, -0.0032), rl: r * 0.86, rs: r * 0.82 },
    { p: at(len * 0.7, -0.0039), rl: r * 0.7, rs: r * 0.62 },
    { p: at(len * 0.9, -0.0044), rl: r * 0.55, rs: r * 0.45 },
    { p: at(len, -0.0052), rl: r * 0.22, rs: r * 0.2 },
  ], look, lat);
}

/** 🕸️ A web between two toes: a thin triangle from the toe joint along the bisector `yawDeg`, opening by `halfDeg` to each side, to `len`. */
export function web(S: Bench, id: string, sd: Side, yawDeg: number, halfDeg: number, len: number, look: Look): void {
  const [c, s] = [Math.cos((yawDeg * Math.PI) / 180), Math.sin((yawDeg * Math.PI) / 180)];
  const f = S.frame(`foot-${sideBone(sd)}`);
  const lat = unit(add(mul(f.u as V3, -s), mul(f.w as V3, -sd * c)));
  const k = Math.tan((halfDeg * Math.PI) / 180);
  S.path(id, `foot-${sideBone(sd)}`, [0.06, 0.3, 0.6, 0.85, 1].map((q): PP => ({ p: footAt(S, sd, c * q * len, lerp(-0.0034, -0.0048, q), s * q * len), rl: q * len * k + 0.0012, rs: lerp(0.002, 0.0012, q), e: 4 })), look, lat);
}

export const TOES = { middle: [0, 0.058, 0.0034], inner: [25, 0.046, 0.003], outer: [-24, 0.054, 0.0031] } as const;

export function foot50(S: Bench, sd: Side): void {
  const n = sideName(sd);
  web(S, `${n}-web-inner`, sd, 12.5, 11.5, 0.041, SKIN);
  web(S, `${n}-web-outer`, sd, -12, 11, 0.046, SKIN);
  for (const [name, [yaw, len, r]] of Object.entries(TOES)) toe(S, `${n}-toe-${name}`, sd, yaw, len, r, SKIN);
  S.path(`${n}-hallux`, `foot-${sideBone(sd)}`, [{ p: footAt(S, sd, 0.0, 0.003, 0.003), rl: 0.0022, rs: 0.0022 }, { p: footAt(S, sd, -0.006, 0.001, 0.005), rl: 0.002, rs: 0.002 }, { p: footAt(S, sd, -0.012, -0.002, 0.0065), rl: 0.0015, rs: 0.0015 }, { p: footAt(S, sd, -0.0145, -0.0035, 0.007), rl: 0.0006, rs: 0.0006 }], SKIN);
}

export function leg50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
  limb(S, `${n}-thigh`, `thigh-${b}`, hip, knee, THIGH_50, FLANK);
  limb(S, `${n}-drumstick`, `shank-${b}`, knee, ankle, DRUMSTICK_50, FLANK);
  limb(S, `${n}-shank`, `shank-${b}`, knee, ankle, SHANK_50, SKIN);
  limb(S, `${n}-tarsus`, `tarsus-${b}`, ankle, mtp, TARSUS_50, SKIN, 2.3);
  foot50(S, sd);
}

export const NECK_1_50: readonly NeckStation[] = [[-0.022, 0, 0.026, 0.028], [-0.004, 0, 0.0325, 0.034], [0.016, 0, 0.0335, 0.034], [0.036, 0, 0.0285, 0.0285], [0.052, 0, 0.025, 0.0242], [0.063, 0, 0.021, 0.0202], [0.069, 0, 0.012, 0.012]];
export const NECK_2_50: readonly NeckStation[] = [[-0.016, 0, 0.013, 0.013], [-0.007, 0, 0.0225, 0.0215], [0.008, 0, 0.0245, 0.0228], [0.026, 0, 0.0222, 0.0202], [0.044, 0, 0.0198, 0.0178], [0.056, 0, 0.0186, 0.0166], [0.064, 0, 0.0125, 0.0118], [0.067, 0, 0.006, 0.006]];
export const NECK_3_50: readonly NeckStation[] = [[-0.013, 0, 0.011, 0.011], [-0.005, 0, 0.0176, 0.0158], [0.01, 0, 0.0173, 0.0152], [0.028, 0, 0.0172, 0.015], [0.044, 0, 0.0178, 0.0154], [0.055, 0, 0.0152, 0.0138], [0.061, 0, 0.008, 0.008]];
export const RING_50: readonly NeckStation[] = [[0.0245, 0, 0.0214, 0.0195], [0.0265, 0, 0.0228, 0.0208], [0.031, 0, 0.0223, 0.0203], [0.0355, 0, 0.0216, 0.0196], [0.0375, 0, 0.0204, 0.0185]];

export function body50(S: Bench): void {
  hull50(S);
  link(S, "neck-1", "neck-1", NECK_1_50, BREAST, 2);
  link(S, "neck-2", "neck-2", NECK_2_50, GREEN, 2);
  link(S, "neck-3", "neck-3", NECK_3_50, GREEN, 2);
  link(S, "neck-ring", "neck-2", RING_50, RING);
  head50(S);
  tail50(S);
  for (const sd of [-1, 1] as const) {
    wing50(S, sd);
    leg50(S, sd);
  }
}

export { dot };
