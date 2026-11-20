/** 🌊️ Round 4 herring gull, 1:500: body and wire skeleton of the digital scale, authored anew for a smallest feature of 75 mm (the drawing is unavailable: the standing bird is 0.7 mm high on paper). The gull stays recognisable by its masses: full breast, long tapering rear with the primaries beyond the tail, upright neck, head with the long bill, two leg strokes.
 * @see ./r3-herring-gull-coarse.ts
 * @see ./r3-herring-gull-skeleton.ts */
import { Bench, mul, sideBone, sideName, smooth, type HullStation, type Look, type PP, type Station, type V3 } from "./r3-herring-gull-kit.ts";
import { BILL, COVERT, PRIMARY, SKIN, TAIL, WHITE, fan, headLoft, strip } from "./r3-herring-gull-body.ts";
import { wire } from "./r3-herring-gull-skeleton.ts";

const BONE: Look = { material: "bone", tone: 0.88 };
const SHAFT: Look = { material: "wing-feathers", tone: 0.7 };
const MUSCLE: Look = { material: "plumage", tone: 0.5 };
const HULL_500: readonly HullStation[] = [
  [-0.152, 0.022, 0.006, 0.009],
  [-0.09, 0.028, -0.02, 0.027],
  [-0.01, 0.038, -0.07, 0.049],
  [0.07, 0.0425, -0.087, 0.0555],
  [0.13, 0.036, -0.07, 0.046],
  [0.162, -0.004, -0.018, 0.01],
];

/** 🐦️ 13 masses: hull, the neck as one bent column, head, bill, tail, per wing the covert panel and the primaries, per leg one stroke and the foot. */
export function body500(S: Bench): void {
  S.hull = smooth(HULL_500, 3);
  S.hullLoft("body", S.hull, WHITE);
  S.path("neck-1", "neck-1", [{ p: S.J("neck-1", -0.034, -0.008), rl: 0.027, rs: 0.027 }, { p: S.J("neck-1", 0.002, -0.002), rl: 0.035, rs: 0.038 }, { p: S.J("neck-2", 0.004, 0.001), rl: 0.0235, rs: 0.0255 }, { p: S.J("neck-2", 0.05, 0.004), rl: 0.0195, rs: 0.0212 }, { p: S.J("neck-2", 0.09, 0.007), rl: 0.0152, rs: 0.0152 }], WHITE);
  headLoft(S, "head", [[-0.03, 0.006, 0.016, 0.016], [0.006, 0.012, 0.025, 0.0218], [0.066, 0.0005, 0.0102, 0.0088]], WHITE, 2, 2);
  headLoft(S, "bill", [[0.064, 0.0002, 0.01, 0.0078], [0.102, -0.0038, 0.009, 0.0076], [0.1255, -0.0106, 0.0018, 0.0018]], BILL, 2.4);
  fan(S, "tail", [[-0.03, 0.002, 0.016, 0.0152], [0.06, 0.0014, 0.028, 0.0076], [0.17, -0.0004, 0.024, 0.004]], TAIL, 3.2);
  for (const sd of [-1, 1] as const) {
    const [n, b] = [sideName(sd), sideBone(sd)];
    strip(S, `${n}-primaries`, `hand-${b}`, sd, [[-0.03, 0.012, -0.024, 0.006], [-0.12, 0.033, 0.012, 0.0152, 0.022, 0.012], [-0.29, 0.052, 0.045, 0.0152, 0.008, 0.004], [-0.318, 0.052, 0.049, 0.004, 0.003, 0.0018]], PRIMARY, 0.0062, 1.9, 2);
    strip(S, `${n}-coverts`, `humerus-${b}`, sd, [[0.112, 0.018, 0.002, 0.004], [0.04, 0.036, -0.042, 0.0152], [-0.07, 0.026, -0.01, 0.0152], [-0.106, 0.021, 0.012, 0.004, 0.023]], COVERT, 0.0062, 1.8, 2);
    const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
    const exit: V3 = [knee[0] + (ankle[0] - knee[0]) * 0.5, knee[1] + (ankle[1] - knee[1]) * 0.5, knee[2]];
    S.path(`${n}-leg`, `tarsus-${b}`, [exit, ankle, mtp].map((p): PP => ({ p, rl: 0.0017, rs: 0.0017 })), SKIN);
    S.body[S.body.length - 1]!.slender = true;
    S.path(`${n}-foot`, `foot-${b}`, [{ p: [mtp[0] - 0.002, mtp[1] - 0.0015, mtp[2]], rl: 0.00135, rs: 0.00135 }, { p: [mtp[0] + 0.057, mtp[1] - 0.0035, mtp[2]], rl: 0.0012, rs: 0.0012 }], SKIN);
    S.body[S.body.length - 1]!.slender = true;
  }
}

/** 🦴️ The wire skeleton of 1:500: skull, synsacrum and sternum with its keel as masses, the dorsal spine, the neck in its two links, coracoids, and one slender bar per wing and leg bone; the trunk mass. */
export function skeleton500(S: Bench): void {
  const from = S.body.length;
  const f = S.frame("head");
  S.loft("skull", "head", ([[-0.018, 0.009, 0.0095, 0.0105], [0.012, 0.0105, 0.0158, 0.0155], [0.062, 0.0005, 0.0152, 0.0152], [0.1235, -0.0094, 0.0018, 0.0018]] as const).map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.2 })), BONE);
  wire(S, "cervical-1", "neck-1", [S.J("neck-1", -0.004), S.J("neck-1", 0.073)], 0.0045);
  wire(S, "cervical-2", "neck-2", [S.J("neck-2", -0.004), S.J("neck-2", 0.068)], 0.0045);
  wire(S, "spine", "trunk", [S.T(0.103, 0.014), S.T(0.068, 0.031), S.T(0.036, 0.029)], 0.0045);
  S.loft("synsacrum", "trunk", ([[0.046, 0.026, 0.0078, 0.0078], [-0.004, 0.016, 0.0225, 0.0152], [-0.07, 0.016, 0.0078, 0.0078]] as const).map(([s, t, half, th]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, th), e: 2.4 })), BONE);
  wire(S, "pygostyle", "tail", [S.T(-0.064, 0.016), S.J("tail", 0.0), S.J("tail", 0.158)], 0.003, SHAFT);
  S.loft("sternum", "trunk", ([[0.128, -0.05, 0.0152, 0.0152], [0.082, -0.061, 0.0245, 0.0158], [0.04, -0.056, 0.0152, 0.0152]] as const).map(([s, t, half, depth]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, depth), e: 1.6 })), BONE);
  for (const sd of [-1, 1] as const) {
    const [n, b] = [sideName(sd), sideBone(sd)];
    wire(S, `${n}-coracoid`, "trunk", [S.T(0.095, 0.016, sd * 0.026), S.T(0.126, -0.032, sd * 0.012)], 0.003);
    const [shoulder, elbow, wrist] = [S.J(`humerus-${b}`), S.J(`forearm-${b}`), S.J(`hand-${b}`)];
    wire(S, `${n}-humerus`, `humerus-${b}`, [shoulder, elbow], 0.004);
    wire(S, `${n}-ulna-radius`, `forearm-${b}`, [elbow, wrist], 0.0035);
    wire(S, `${n}-hand-bones`, `hand-${b}`, [wrist, S.J(`hand-${b}`, 0.1), S.T(-0.17, 0.034, sd * 0.017), S.T(-0.296, 0.049, sd * 0.0055)], 0.0035, SHAFT);
    const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
    wire(S, `${n}-femur`, `thigh-${b}`, [hip, knee], 0.0035);
    wire(S, `${n}-tibiotarsus`, `shank-${b}`, [knee, ankle], 0.0035);
    wire(S, `${n}-tarsometatarsus`, `tarsus-${b}`, [ankle, mtp], 0.0035);
    wire(S, `${n}-digit-middle`, `foot-${b}`, [[mtp[0], mtp[1] - 0.0022, mtp[2]], [mtp[0] + 0.056, mtp[1] - 0.0038, mtp[2]]], 0.0016);
  }
  const trunkParts = /spine|synsacrum|sternum|coracoid/;
  S.toStructure(from, (s) => (trunkParts.test(s.id) ? "trunk" : s.bone));
  const m0 = S.body.length;
  const t = S.frame("trunk");
  S.ell("mass-trunk", "trunk", S.T(0.035, -0.03), [0.034, 0.0158, 0.0225], MUSCLE, [t.u as V3, t.v as V3]);
  S.toStructure(m0, (s) => s.bone, "mass");
}
