/** 🕊️ Round 3 herring gull, body at 1:50 and the shaping helpers of every scale: each mass of the standing adult (Larus argentatus) stated section by section by hand (trunk frame: `s` along the column from the hips, `t` towards the back; bone frames for neck, head, tail, legs and feet), after the biometrics of Cramp (1983) and the long bones of IMNH R-976.
 * The folded wing is three strips on three bones, each with its trailing side on the bone's -v (the anatomical fold: the humerus' trailing edge faces up, the forearm's down, the hand's up), so that the spread poses swing the same strips out as inner wing, arm wing and hand wing.
 * @see ./r3-herring-gull-kit.ts
 * @see ./r3_herring_gull_roots.py */
import { Bench, add, cross, dot, lerp, mul, sideBone, sideName, smooth, sub, unit, type HullStation, type Look, type PP, type Side, type Station, type V3 } from "./r3-herring-gull-kit.ts";

export const WHITE: Look = { material: "plumage", tone: 0.95 };
export const MANTLE: Look = { material: "wing-feathers", tone: 0.66 };
export const COVERT: Look = { material: "wing-feathers", tone: 0.64 };
export const TERTIAL: Look = { material: "wing-feathers", tone: 0.86 };
export const SKIRT: Look = { material: "wing-feathers", tone: 0.9 };
export const PRIMARY: Look = { material: "wing-feathers", tone: 0.1 };
export const TAIL: Look = { material: "plumage", tone: 0.95 };
export const BILL: Look = { material: "bill-keratin", tone: 0.8 };
export const SPOT: Look = { material: "bill-keratin", tone: 0.32, role: "detail" };
export const SKIN: Look = { material: "scaled-skin", tone: 0.68 };
export const EYE: Look = { material: "eye-tissue", tone: 0.5, role: "detail" };

/** 🥚️ Plumage hull of the trunk: [s, dorsal, ventral, half width]. Back 40-43 mm over the hip axis, belly 88 mm under it (13 cm deep), 112 mm wide without the wings, the full breast 16 cm ahead of the hips, the shoulders rising into the nape, the under-tail coverts tapering 8 cm along the underside of the tail. */
export const HULL_50: readonly HullStation[] = [
  [-0.158, 0.021, 0.011, 0.005],
  [-0.145, 0.023, 0.004, 0.011],
  [-0.125, 0.025, -0.004, 0.017],
  [-0.105, 0.027, -0.013, 0.022],
  [-0.085, 0.029, -0.024, 0.027],
  [-0.065, 0.031, -0.036, 0.033],
  [-0.04, 0.034, -0.054, 0.042],
  [-0.01, 0.038, -0.069, 0.049],
  [0.02, 0.041, -0.08, 0.054],
  [0.05, 0.043, -0.087, 0.056],
  [0.08, 0.043, -0.088, 0.056],
  [0.105, 0.04, -0.083, 0.053],
  [0.125, 0.037, -0.073, 0.048],
  [0.14, 0.032, -0.061, 0.041],
  [0.152, 0.023, -0.046, 0.032],
  [0.159, 0.01, -0.029, 0.02],
  [0.162, -0.006, -0.016, 0.008],
];

export const between = (st: readonly HullStation[], lo: number, hi: number): HullStation[] => st.filter((q) => q[0] >= lo - 1e-9 && q[0] <= hi + 1e-9);

/** 🩶️ The grey mantle: a lens over the back between the folded wings, [s, half width at its edge, thickness]; it lies 0.6 mm proud of the hull top and dives under the wing strips at its sides. */
export type MantleStation = readonly [s: number, half: number, th: number];
export function mantle(S: Bench, id: string, st: readonly MantleStation[], look: Look): void {
  S.loft(id, "trunk", st.map(([s, half, th]): Station => {
    const td = topOf(S, s);
    return { c: S.T(s, td - th + 0.0006), a: [0, 0, half], b: mul(S.dorsal, th), e: 2.3 };
  }), look);
}

/** 📈️ Dorsal edge of the hull at `s`. */
export function topOf(S: Bench, s: number): number {
  const st = S.hull;
  const i = Math.max(1, st.findIndex((q) => q[0] >= s));
  const [a, b] = [st[i - 1]!, st[i]!];
  return lerp(a[1], b[1], Math.max(0, Math.min(1, (s - a[0]) / (b[0] - a[0]))));
}

/** 🦢️ One neck or head piece in its bone frame: stations [a along the bone, b across it, sagittal half depth, half width]. */
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

/** 🧠️ Head of the gull: long flat crown sloping into the bill, rounded nape, 51 mm deep and 44 mm wide over the feathers. */
export const HEAD_50: readonly NeckStation[] = [
  [-0.031, 0.004, 0.011, 0.011],
  [-0.021, 0.0065, 0.0185, 0.018],
  [-0.006, 0.0115, 0.0235, 0.0205],
  [0.008, 0.012, 0.025, 0.0218],
  [0.022, 0.011, 0.0238, 0.0208],
  [0.036, 0.0085, 0.0205, 0.0182],
  [0.048, 0.0055, 0.0162, 0.0148],
  [0.058, 0.0025, 0.0122, 0.0112],
  [0.066, 0.0005, 0.0101, 0.0086],
];
/** 🟡️ Bill: culmen 58 mm from the feathering (the bone runs 125 mm from the skull joint to the tip), 19.6 mm deep at the base, 18 mm at the gonys angle, 8.4 mm wide; the culmen curves down into the hook. */
export const BILL_50: readonly NeckStation[] = [
  [0.067, -0.0002, 0.0097, 0.0052],
  [0.074, -0.0008, 0.0094, 0.0049],
  [0.084, -0.0017, 0.0088, 0.0045],
  [0.094, -0.0027, 0.0085, 0.0041],
  [0.103, -0.004, 0.0087, 0.0037],
  [0.11, -0.0054, 0.0079, 0.0032],
  [0.1165, -0.0071, 0.006, 0.0026],
  [0.1215, -0.0088, 0.004, 0.002],
  [0.1255, -0.0106, 0.0016, 0.0011],
];
export const EYE_AT: readonly [number, number, number] = [0.03, 0.0155, 0.0172];

export function head50(S: Bench): void {
  headLoft(S, "head", HEAD_50, WHITE, 2, 2);
  headLoft(S, "bill", BILL_50, BILL, 2.6, 2);
  const f = S.frame("head");
  for (const sd of [-1, 1] as const) S.ell(sd < 0 ? "eye" : "eye-f", "head", S.J("head", EYE_AT[0], EYE_AT[1], sd * EYE_AT[2]), [0.0041, 0.0039, 0.0038], EYE, [f.u as V3, f.v as V3]);
  const spot = S.loft("bill-spot", "head", [0.0975, 0.1015, 0.1055, 0.1095].map((a, i): Station => {
    const low = S.J("head", a, -0.004 - [0.0086, 0.0089, 0.0088, 0.0083][i]! + 0.0024);
    return { c: low, a: [0, 0, [0.0032, 0.0039, 0.0039, 0.0033][i]!], b: mul(f.v as V3, [0.0016, 0.0032, 0.0032, 0.0016][i]!), e: 2.2 };
  }), SPOT);
  spot.texture = true;
}

/** 🪽️ A strip of the folded wing lying on the flank: [s, upper edge t, lower edge t, half thickness, lateral override of both edges (0 = on the hull), lateral position of the upper edge alone]. Each section spans from the hull surface at the lower edge to the hull surface at the upper edge and is pushed out by the bulge of the flank between them; behind the hull (over the tail) both edges take the stated lateral positions. */
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
    const free = Math.abs(k) > 1e-6;
    const [top, bottom] = [S.T(s, t1, sd * z1), S.T(s, t0, sd * z0)];
    const up = unit(sub(top, bottom));
    let out = unit(cross(S.axis, up));
    if (out[2] * sd < 0) out = mul(out, -1);
    const mid = mul(add(top, bottom), 0.5);
    const bulge = free ? 0 : Math.max(0, S.flank(s, (t1 + t0) / 2) - (z1 + z0) / 2) * Math.abs(out[2]);
    return { c: add(mid, mul(out, proud - th + 0.75 * bulge)), a: mul(out, Math.max(th, 0.0004)), b: mul(up, Math.hypot(...sub(top, bottom)) / 2), e };
  }), look);
}

/** 🩶️ Humerus strip: the upper panel of the folded wing (lesser and median coverts, scapular edge), from the shoulder back past the elbow; its trailing side faces up, so the spread pose turns it into the inner wing behind the humerus. */
export const COVERTS_50: readonly Strip[] = [
  [0.112, 0.018, 0.002, 0.0015],
  [0.104, 0.03, -0.008, 0.0045],
  [0.085, 0.036, -0.011, 0.007],
  [0.055, 0.037, -0.012, 0.0075],
  [0.02, 0.036, -0.01, 0.0075],
  [-0.015, 0.033, -0.006, 0.007],
  [-0.045, 0.029, -0.001, 0.006],
  [-0.068, 0.025, 0.004, 0.0045],
  [-0.084, 0.022, 0.009, 0.0025],
  [-0.092, 0.021, 0.012, 0.001],
];
/** 🤍️ Tertials: the white crescent at the rear top of the folded wing, over the bases of the primaries. */
export const TERTIALS_50: readonly Strip[] = [
  [-0.05, 0.03, 0.022, 0.0012],
  [-0.065, 0.028, 0.012, 0.0032],
  [-0.082, 0.025, 0.008, 0.0039],
  [-0.096, 0.023, 0.009, 0.003, 0.027],
  [-0.106, 0.021, 0.012, 0.001, 0.023],
];
/** 🩶️ Forearm strip: the greater coverts and secondaries below the forearm (trailing side down), whose lower edge is the line of the folded wing on the flank. */
export const SECONDARIES_50: readonly Strip[] = [
  [0.102, -0.006, -0.02, 0.0015],
  [0.094, -0.004, -0.031, 0.0045],
  [0.07, -0.004, -0.04, 0.0065],
  [0.04, -0.004, -0.044, 0.007],
  [0.005, -0.002, -0.042, 0.0068],
  [-0.03, 0.002, -0.035, 0.006],
  [-0.055, 0.008, -0.024, 0.0044],
  [-0.07, 0.012, -0.014, 0.0026],
  [-0.078, 0.014, -0.006, 0.001],
];
/** 🤍️ The white tips of the secondaries: a thin band along the lower edge of the folded wing. */
export const SKIRT_50: readonly Strip[] = [
  [0.078, -0.032, -0.04, 0.0008],
  [0.055, -0.035, -0.0435, 0.0022],
  [0.02, -0.035, -0.0438, 0.0024],
  [-0.015, -0.03, -0.039, 0.0022],
  [-0.045, -0.021, -0.03, 0.0019],
  [-0.07, -0.011, -0.019, 0.0008],
];
/** 🖤️ Hand strip: the stacked black primaries from under the tertials to the tips, 75 mm beyond the tail; behind the hull they lie over the tail, the tips of both wings crossing. */
export const PRIMARIES_50: readonly Strip[] = [
  [-0.03, 0.012, -0.024, 0.0024],
  [-0.05, 0.018, -0.017, 0.0042],
  [-0.075, 0.024, -0.006, 0.005],
  [-0.1, 0.03, 0.007, 0.005, 0, 0.018],
  [-0.13, 0.037, 0.021, 0.0046, 0.027, 0.013],
  [-0.163, 0.042, 0.029, 0.0042, 0.023, 0.01],
  [-0.205, 0.047, 0.035, 0.0038, 0.019, 0.007],
  [-0.248, 0.051, 0.04, 0.0032, 0.014, 0.005],
  [-0.286, 0.053, 0.045, 0.0026, 0.01, 0.004],
  [-0.307, 0.053, 0.0475, 0.0018, 0.006, 0.003],
  [-0.318, 0.052, 0.049, 0.0008, 0.003, 0.0018],
];
/** ⚪️ White tips of the outer primaries ("mirrors"), a flat patch near each wing tip. */
export const MIRROR_50: readonly Strip[] = [
  [-0.282, 0.0532, 0.0448, 0.0008, 0.0108, 0.0046],
  [-0.292, 0.0534, 0.0458, 0.0014, 0.0092, 0.0042],
  [-0.301, 0.0533, 0.0466, 0.0008, 0.0076, 0.0036],
];

/** 🫥️ The membranes of the spread wing that the folded wing tucks inside the body: lens sheets in the bone plane (z 36 mm) under the flank, [s, upper edge t, lower edge t, half thickness]. Folded they lie wholly inside the plumage hull; spread they widen the wing: the propatagium ahead of the humerus, the secondaries below the forearm, the inner primaries above the hand. */
export type Web = readonly [s: number, top: number, bottom: number, th: number];
export function tucked(S: Bench, id: string, bone: string, sd: Side, rows: readonly Web[], look: Look): void {
  strip(S, id, bone, sd, rows.map(([s, t1, t0, th]): Strip => [s, t1, t0, th, 0.031, 0.031]), look, 0, 2.0, 2);
  const last = S.body[S.body.length - 1]!;
  for (const q of last.sections as { at: number[] }[]) q.at = q.at.map((v) => Math.round(v * 1e5) / 1e5);
}
export const PROPATAGIUM_50: readonly Web[] = [[0.098, 0.018, -0.006, 0.0022], [0.085, 0.02, -0.02, 0.0039], [0.05, 0.021, -0.027, 0.0039], [0.0, 0.02, -0.025, 0.0039], [-0.03, 0.015, -0.016, 0.0039], [-0.045, 0.01, -0.006, 0.0022]];
export const ARM_WEB_50: readonly Web[] = [[-0.042, 0.01, -0.006, 0.0022], [-0.028, 0.012, -0.033, 0.0039], [0.0, 0.012, -0.053, 0.0039], [0.04, 0.0, -0.062, 0.0039], [0.075, -0.01, -0.06, 0.0039], [0.1, -0.016, -0.046, 0.0022]];
export const HAND_WEB_50: readonly Web[] = [[0.1, -0.008, -0.024, 0.0022], [0.07, 0.008, -0.02, 0.0039], [0.03, 0.016, -0.014, 0.0039], [-0.01, 0.018, -0.008, 0.0039], [-0.04, 0.012, -0.002, 0.0022]];

export function wing50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  tucked(S, `${n}-propatagium`, `humerus-${b}`, sd, PROPATAGIUM_50, COVERT);
  tucked(S, `${n}-arm-web`, `forearm-${b}`, sd, ARM_WEB_50, COVERT);
  tucked(S, `${n}-hand-web`, `hand-${b}`, sd, HAND_WEB_50, COVERT);
  strip(S, `${n}-primaries`, `hand-${b}`, sd, PRIMARIES_50, PRIMARY, 0.0042, 1.9, 2);
  const m = S.body.length;
  strip(S, `${n}-mirror`, `hand-${b}`, sd, MIRROR_50, SKIRT, 0.0072, 2.0, 1);
  S.body[m]!.texture = true;
  strip(S, `${n}-secondaries`, `forearm-${b}`, sd, SECONDARIES_50, COVERT, 0.0034, 2.0, 2);
  const k = S.body.length;
  strip(S, `${n}-skirt`, `forearm-${b}`, sd, SKIRT_50, SKIRT, 0.0036, 2.0, 2);
  S.body[k]!.texture = true;
  strip(S, `${n}-coverts`, `humerus-${b}`, sd, COVERTS_50, COVERT, 0.0046, 1.8, 2);
  strip(S, `${n}-tertials`, `humerus-${b}`, sd, TERTIALS_50, TERTIAL, 0.0058, 1.9, 2);
}

/** 🪶️ Tail in its bone frame: [q along the tail from the pygostyle, offset above its axis, half width, half thickness]. */
export type Feather = readonly [q: number, up: number, half: number, th: number];
export function fan(S: Bench, id: string, st: readonly Feather[], look: Look, e = 3): void {
  const up = mul(S.frame("tail").v as V3, -1);
  S.loft(id, "tail", st.map(([q, o, half, th]): Station => ({ c: add(S.J("tail", q), mul(up, o)), a: [0, 0, half], b: mul(up, th), e })), look);
}

/** 🪶️ The closed tail: twelve rectrices 170 mm long, 58 mm wide, square-ended. */
export const TAIL_50: readonly Feather[] = [
  [-0.03, 0.002, 0.016, 0.012],
  [0.0, 0.002, 0.022, 0.0115],
  [0.04, 0.0018, 0.027, 0.008],
  [0.09, 0.0012, 0.029, 0.0052],
  [0.13, 0.0006, 0.029, 0.004],
  [0.158, 0.0, 0.028, 0.0034],
  [0.17, -0.0004, 0.024, 0.0024],
];

/** 🦵️ A limb segment between two joints: [fraction of the way, half extent in the sagittal plane, half extent laterally]. */
export type Ring = readonly [f: number, rAhead: number, rLateral: number];
export function limb(S: Bench, id: string, bone: string, a: V3, b: V3, st: readonly Ring[], look: Look, e = 2): void {
  S.path(id, bone, st.map(([f, rs, rl]): PP => ({ p: [lerp(a[0], b[0], f), lerp(a[1], b[1], f), lerp(a[2], b[2], Math.max(0, Math.min(1, f)))], rl, rs, e })), look);
}

export const THIGH_50: readonly Ring[] = [[-0.2, 0.01, 0.01], [0.0, 0.019, 0.016], [0.4, 0.023, 0.018], [0.8, 0.021, 0.017], [1.0, 0.017, 0.014], [1.08, 0.008, 0.008]];
export const DRUMSTICK_50: readonly Ring[] = [[-0.1, 0.009, 0.009], [0.0, 0.017, 0.014], [0.2, 0.017, 0.014], [0.38, 0.014, 0.012], [0.52, 0.0108, 0.009], [0.64, 0.0078, 0.0066], [0.73, 0.0056, 0.0048], [0.79, 0.0036, 0.0032]];
export const SHANK_50: readonly Ring[] = [[0.74, 0.003, 0.0025], [0.82, 0.003, 0.0025], [0.9, 0.0034, 0.0028], [0.96, 0.0047, 0.0038], [1.0, 0.005, 0.004], [1.04, 0.0034, 0.003]];
export const TARSUS_50: readonly Ring[] = [[-0.04, 0.0034, 0.0028], [0.05, 0.0045, 0.0034], [0.22, 0.0033, 0.0022], [0.78, 0.0032, 0.0022], [0.93, 0.004, 0.0034], [1.03, 0.003, 0.0028]];

/** 🦶️ Point of a foot frame: `a` along the foot from the toe joint, `b` above the bone axis (the sole lies 5.5 mm under it), `w` towards the inner side of the foot. */
export const footAt = (S: Bench, sd: Side, a: number, b: number, w = 0): V3 => S.J(`foot-${sideBone(sd)}`, a, b, -sd * w);

/** 🦶️ A toe from the toe joint: yaw towards the inner side (degrees), length with claw, radius at its base. */
export function toe(S: Bench, id: string, sd: Side, yawDeg: number, len: number, r: number, look: Look): void {
  const [c, s] = [Math.cos((yawDeg * Math.PI) / 180), Math.sin((yawDeg * Math.PI) / 180)];
  const f = S.frame(`foot-${sideBone(sd)}`);
  const lat = unit(add(mul(f.u as V3, -s), mul(f.w as V3, -sd * c)));
  const at = (q: number, b: number): V3 => footAt(S, sd, c * q, b, s * q);
  S.path(id, `foot-${sideBone(sd)}`, [
    { p: at(-0.003, -0.0018), rl: r * 0.85, rs: r * 0.85 },
    { p: at(0.004, -0.002), rl: r, rs: r },
    { p: at(len * 0.35, -0.0028), rl: r * 0.86, rs: r * 0.8 },
    { p: at(len * 0.7, -0.0034), rl: r * 0.72, rs: r * 0.62 },
    { p: at(len * 0.9, -0.0038), rl: r * 0.55, rs: r * 0.45 },
    { p: at(len, -0.0046), rl: r * 0.22, rs: r * 0.2 },
  ], look, lat);
}

/** 🕸️ A web between two toes: a thin plate from the toe joint along the bisector `yawDeg`, opening by `halfDeg` to each side, to `len`, its free edge slightly concave. */
export function web(S: Bench, id: string, sd: Side, yawDeg: number, halfDeg: number, len: number, look: Look): void {
  const [c, s] = [Math.cos((yawDeg * Math.PI) / 180), Math.sin((yawDeg * Math.PI) / 180)];
  const f = S.frame(`foot-${sideBone(sd)}`);
  const lat = unit(add(mul(f.u as V3, -s), mul(f.w as V3, -sd * c)));
  const k = Math.tan((halfDeg * Math.PI) / 180);
  S.path(id, `foot-${sideBone(sd)}`, [0.06, 0.3, 0.6, 0.85, 1].map((q): PP => ({ p: footAt(S, sd, c * q * len, lerp(-0.003, -0.0042, q), s * q * len), rl: q * len * k * (q > 0.9 ? 0.94 : 1) + 0.0012, rs: lerp(0.0011, 0.0007, q), e: 4 })), look, lat);
}

/** 🦶️ Toes of the gull: [yaw towards the inner side, length with claw, radius]: middle 58 mm, outer 53 mm, inner 43 mm, spread 25 degrees to each side. */
export const TOES = { middle: [0, 0.058, 0.0016], inner: [26, 0.043, 0.0014], outer: [-24, 0.053, 0.0015] } as const;

export function foot50(S: Bench, sd: Side): void {
  const n = sideName(sd);
  web(S, `${n}-web-inner`, sd, 13, 10.5, 0.037, SKIN);
  web(S, `${n}-web-outer`, sd, -12, 10, 0.045, SKIN);
  for (const [name, [yaw, len, r]] of Object.entries(TOES)) toe(S, `${n}-toe-${name}`, sd, yaw, len, r, SKIN);
}

export function leg50(S: Bench, sd: Side): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
  limb(S, `${n}-thigh`, `thigh-${b}`, hip, knee, THIGH_50, WHITE);
  limb(S, `${n}-drumstick`, `shank-${b}`, knee, ankle, DRUMSTICK_50, WHITE);
  limb(S, `${n}-shank`, `shank-${b}`, knee, ankle, SHANK_50, SKIN);
  limb(S, `${n}-tarsus`, `tarsus-${b}`, ankle, mtp, TARSUS_50, SKIN, 2.3);
  foot50(S, sd);
}

/** 🦢️ Neck: the lower neck runs forward and up out of the breast, the upper neck rises to the skull; thick feathered column 62 mm wide at the base, 40 mm under the head. */
export const NECK_1_50: readonly NeckStation[] = [[-0.036, -0.008, 0.026, 0.026], [-0.008, -0.004, 0.039, 0.036], [0.018, -0.001, 0.036, 0.032], [0.042, 0.002, 0.03, 0.0272], [0.064, 0.002, 0.0255, 0.0235], [0.082, 0.001, 0.0225, 0.021]];
export const NECK_2_50: readonly NeckStation[] = [[-0.032, -0.002, 0.012, 0.012], [-0.014, 0.0, 0.0228, 0.0212], [0.01, 0.001, 0.0224, 0.0208], [0.032, 0.002, 0.0218, 0.0202], [0.052, 0.003, 0.0212, 0.0194], [0.068, 0.004, 0.0205, 0.0188], [0.082, 0.006, 0.019, 0.0175], [0.093, 0.007, 0.012, 0.012]];
export const MANTLE_50: readonly MantleStation[] = [[0.112, 0.008, 0.002], [0.1, 0.018, 0.004], [0.075, 0.025, 0.0055], [0.04, 0.026, 0.006], [0.0, 0.024, 0.006], [-0.04, 0.02, 0.0055], [-0.07, 0.016, 0.0045], [-0.092, 0.011, 0.003], [-0.104, 0.006, 0.0015]];

export function hull50(S: Bench): void {
  S.hull = smooth(HULL_50, 3);
  S.hullLoft("body", S.hull, WHITE);
}

export function body50(S: Bench): void {
  hull50(S);
  mantle(S, "mantle", smooth(MANTLE_50, 2), MANTLE);
  link(S, "neck-1", "neck-1", NECK_1_50, WHITE, 3);
  link(S, "neck-2", "neck-2", NECK_2_50, WHITE, 2);
  head50(S);
  fan(S, "tail", TAIL_50, TAIL, 3.2);
  for (const sd of [-1, 1] as const) {
    wing50(S, sd);
    leg50(S, sd);
  }
}

export { dot };
