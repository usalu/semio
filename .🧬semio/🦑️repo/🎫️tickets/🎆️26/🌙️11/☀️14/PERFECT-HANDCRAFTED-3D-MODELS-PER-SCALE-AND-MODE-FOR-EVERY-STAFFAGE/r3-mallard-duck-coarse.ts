/** 🐤️ Round 3 mallard, bodies at 1:100, 1:200 and (round 4) 1:500: authored anew for each scale with merged masses and fewer, own sections (never the 1:50 list thinned out).
 * @see ./r3-mallard-duck-body.ts */
import { Bench, sideBone, sideName, smooth, type HullStation, type PP } from "./r3-mallard-duck-kit.ts";
import { BILL, BREAST, FLANK, GREEN, QUILL, RUMP, SCAPULAR, SKIN, TAIL, WING, between, fan, headLoft, limb, strip, toe, web } from "./r3-mallard-duck-body.ts";

/** 🥚️ 1:100: one hull with fewer stations in its three tone zones; the thighs lie inside it. */
const HULL_100: readonly HullStation[] = [
  [-0.152, 0.0255, 0.0115, 0.006],
  [-0.125, 0.0185, -0.012, 0.019],
  [-0.09, 0.019, -0.039, 0.033],
  [-0.045, 0.026, -0.064, 0.048],
  [0.0, 0.031, -0.08, 0.058],
  [0.05, 0.036, -0.089, 0.063],
  [0.1, 0.039, -0.087, 0.06],
  [0.135, 0.04, -0.076, 0.051],
  [0.16, 0.035, -0.061, 0.041],
  [0.176, 0.021, -0.041, 0.027],
  [0.183, 0.004, -0.022, 0.011],
];

type NeckPoint = readonly [bone: string, a: number, rs: number, rl: number];
const neck = (S: Bench, id: string, bone: string, pts: readonly NeckPoint[], look: typeof GREEN): void => {
  S.path(id, bone, pts.map(([b, a, rs, rl]): PP => ({ p: S.J(b, a), rl, rs })), look);
};

export function body100(S: Bench): void {
  S.hull = smooth(HULL_100, 2);
  S.hullLoft("body", between(S.hull, -0.07, 0.13), FLANK);
  S.hullLoft("rump", between(S.hull, -0.2, -0.045), RUMP, 0.0006);
  S.hullLoft("breast", between(S.hull, 0.11, 0.2), BREAST, 0.0006);
  neck(S, "neck-lower", "neck-1", [["neck-1", -0.022, 0.026, 0.028], ["neck-1", 0.004, 0.0335, 0.0345], ["neck-1", 0.034, 0.0305, 0.03], ["neck-1", 0.056, 0.0255, 0.0242], ["neck-2", 0.014, 0.0238, 0.022], ["neck-2", 0.03, 0.0218, 0.0198], ["neck-2", 0.038, 0.012, 0.012]], BREAST);
  neck(S, "neck-upper", "neck-3", [["neck-2", 0.016, 0.012, 0.012], ["neck-2", 0.025, 0.0215, 0.0196], ["neck-2", 0.046, 0.0196, 0.0176], ["neck-3", 0.012, 0.0176, 0.0155], ["neck-3", 0.043, 0.0178, 0.0154], ["neck-3", 0.057, 0.0145, 0.0135]], GREEN);
  headLoft(S, "head", [[-0.022, 0.005, 0.0072, 0.00691], [-0.015, 0.0076, 0.0166, 0.0144], [-0.002, 0.0106, 0.0224, 0.01805], [0.012, 0.0118, 0.0233, 0.01882], [0.029, 0.0096, 0.02, 0.0169], [0.045, 0.0052, 0.0146, 0.01325], [0.055, 0.0024, 0.0112, 0.01094]], GREEN, 2, 2);
  headLoft(S, "bill", [[0.051, 0.0026, 0.0098, 0.00931], [0.066, 0.0012, 0.0094, 0.01055], [0.09, -0.0009, 0.0064, 0.0114], [0.1035, -0.002, 0.0038, 0.00874], [0.1056, -0.0024, 0.0022, 0.00475]], BILL, 2.7);
  fan(S, "tail", [[-0.022, 0.002, 0.019, 0.009], [0.012, 0.002, 0.0215, 0.0072], [0.05, 0.002, 0.0195, 0.0048], [0.076, 0.002, 0.0095, 0.0032], [0.0855, 0.002, 0.003, 0.0018]], TAIL);
  for (const sd of [-1, 1] as const) {
    const [n, b] = [sideName(sd), sideBone(sd)];
    strip(S, `${n}-primaries`, `wing-${b}`, sd, [[-0.036, 0.002, -0.02, 0.0025], [-0.06, 0.009, -0.016, 0.0076], [-0.1, 0.0215, 0.002, 0.0062, 0, 0.013], [-0.128, 0.022, 0.009, 0.0042, 0, 0.0065], [-0.142, 0.0235, 0.0145, 0.0014, 0, 0.0032]], QUILL, 0.004);
    strip(S, `${n}-wing`, `wing-${b}`, sd, [[0.116, -0.012, -0.022, 0.002], [0.1, -0.002, -0.036, 0.0076], [0.055, -0.002, -0.043, 0.0092], [0.0, 0.0, -0.039, 0.0086], [-0.05, 0.0035, -0.027, 0.0066], [-0.09, 0.0065, -0.013, 0.0036], [-0.105, 0.008, -0.006, 0.0012]], WING, 0.0042);
    strip(S, `${n}-scapulars`, `wing-${b}`, sd, [[0.106, 0.02, 0.006, 0.0015], [0.09, 0.0295, -0.007, 0.0066], [0.045, 0.0325, -0.009, 0.0082], [-0.01, 0.027, -0.005, 0.0076], [-0.065, 0.021, 0.001, 0.005], [-0.118, 0.014, 0.0095, 0.0014, 0.011]], SCAPULAR, 0.0074);
    const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
    limb(S, `${n}-drumstick`, `shank-${b}`, knee, ankle, [[-0.1, 0.008, 0.008], [0.08, 0.0162, 0.013], [0.42, 0.0132, 0.0106], [0.62, 0.0078, 0.0066], [0.71, 0.004, 0.0036]], FLANK);
    limb(S, `${n}-shank`, `shank-${b}`, knee, ankle, [[0.6, 0.0034, 0.0032], [0.97, 0.006, 0.0052], [1.05, 0.004, 0.0036]], SKIN);
    limb(S, `${n}-tarsus`, `tarsus-${b}`, ankle, mtp, [[-0.06, 0.004, 0.0036], [0.12, 0.0052, 0.0042], [0.6, 0.0044, 0.0035], [0.94, 0.0054, 0.0054], [1.03, 0.0042, 0.0042]], SKIN, 2.3);
    web(S, `${n}-web`, sd, 0.5, 24.5, 0.046, SKIN);
    toe(S, `${n}-toe-middle`, sd, 0, 0.058, 0.0034, SKIN);
  }
}

/** 🥚️ 1:200: hull with thighs and drumsticks in one mass. */
const HULL_200: readonly HullStation[] = [
  [-0.152, 0.0255, 0.0115, 0.006],
  [-0.12, 0.018, -0.016, 0.021],
  [-0.08, 0.021, -0.045, 0.037],
  [-0.02, 0.029, -0.074, 0.054],
  [0.05, 0.036, -0.089, 0.063],
  [0.115, 0.04, -0.083, 0.057],
  [0.158, 0.036, -0.063, 0.042],
  [0.183, 0.004, -0.022, 0.011],
];

export function body200(S: Bench): void {
  S.hull = smooth(HULL_200, 2);
  S.hullLoft("body", S.hull, FLANK);
  neck(S, "neck-lower", "neck-1", [["neck-1", -0.022, 0.026, 0.028], ["neck-1", 0.01, 0.0335, 0.0345], ["neck-1", 0.05, 0.027, 0.0258], ["neck-2", 0.018, 0.024, 0.0224], ["neck-2", 0.036, 0.0152, 0.0152]], BREAST);
  neck(S, "neck-upper", "neck-3", [["neck-2", 0.018, 0.0152, 0.0152], ["neck-2", 0.042, 0.0198, 0.0178], ["neck-3", 0.026, 0.0175, 0.0154], ["neck-3", 0.056, 0.015, 0.0142]], GREEN);
  headLoft(S, "head", [[-0.022, 0.005, 0.0075, 0.0072], [-0.016, 0.0075, 0.0168, 0.01459], [-0.004, 0.0104, 0.0222, 0.01786], [0.014, 0.0116, 0.0232, 0.01862], [0.033, 0.0088, 0.0192, 0.01651], [0.047, 0.0046, 0.014, 0.01286], [0.055, 0.0024, 0.0112, 0.01094]], GREEN, 2, 3);
  headLoft(S, "bill", [[0.051, 0.0026, 0.0098, 0.00931], [0.07, 0.0008, 0.009, 0.01073], [0.095, -0.0013, 0.0056, 0.01121], [0.1025, -0.002, 0.0036, 0.00893], [0.1056, -0.0024, 0.0022, 0.00532]], BILL, 2.7);
  fan(S, "tail", [[-0.022, 0.002, 0.019, 0.009], [0.03, 0.002, 0.0215, 0.006], [0.07, 0.002, 0.014, 0.0036], [0.0855, 0.002, 0.004, 0.0018]], TAIL);
  for (const sd of [-1, 1] as const) {
    const [n, b] = [sideName(sd), sideBone(sd)];
    strip(S, `${n}-wing`, `wing-${b}`, sd, [[0.116, -0.012, -0.022, 0.002], [0.08, -0.001, -0.041, 0.0088], [0.0, 0.0, -0.039, 0.0088], [-0.07, 0.008, -0.019, 0.006], [-0.115, 0.0215, 0.006, 0.0034, 0, 0.009], [-0.142, 0.0235, 0.0145, 0.0014, 0, 0.0032]], WING, 0.0056);
    strip(S, `${n}-scapulars`, `wing-${b}`, sd, [[0.106, 0.02, 0.006, 0.0015], [0.065, 0.0325, -0.01, 0.0088], [-0.03, 0.025, -0.003, 0.0066], [-0.118, 0.014, 0.0095, 0.0014, 0.011]], SCAPULAR, 0.0074);
    const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
    const exit: [number, number, number] = [knee[0] + (ankle[0] - knee[0]) * 0.5, knee[1] + (ankle[1] - knee[1]) * 0.5, knee[2]];
    limb(S, `${n}-leg`, `shank-${b}`, exit, ankle, [[0.0, 0.001, 0.001], [0.5, 0.001, 0.001], [1.0, 0.001, 0.001]], SKIN);
    (S.body[S.body.length - 1] as { slender?: boolean }).slender = true;
    S.path(`${n}-foot`, `tarsus-${b}`, [{ p: [ankle[0], ankle[1], ankle[2]], rl: 0.0024, rs: 0.0024 }, { p: [mtp[0] - 0.0003, mtp[1] + 0.0012, mtp[2]], rl: 0.0024, rs: 0.0024 }, { p: [mtp[0] + 0.004, mtp[1] - 0.0034, mtp[2]], rl: 0.0024, rs: 0.0024 }, { p: [mtp[0] + 0.056, mtp[1] - 0.0036, mtp[2]], rl: 0.0022, rs: 0.0022 }], SKIN);
    (S.body[S.body.length - 1] as { slender?: boolean }).slender = true;
  }
}

/** 🥚️ 1:500 (digital scale, the drake is 0.9 mm long on paper): the fewest masses that still read as a mallard: one hull, a lower and an upper neck piece, head, bill, tail, one folded wing per side from the scapulars to the primary tips, and per leg a shank stroke and a tarsus-and-toe stroke. */
const HULL_500: readonly HullStation[] = [
  [-0.152, 0.0255, 0.0115, 0.006],
  [-0.11, 0.0185, -0.024, 0.026],
  [-0.04, 0.027, -0.066, 0.05],
  [0.05, 0.036, -0.089, 0.063],
  [0.125, 0.04, -0.08, 0.054],
  [0.168, 0.03, -0.052, 0.034],
  [0.183, 0.004, -0.022, 0.011],
];

export function body500(S: Bench): void {
  S.hull = smooth(HULL_500, 2);
  S.hullLoft("body", S.hull, FLANK);
  neck(S, "neck-lower", "neck-1", [["neck-1", -0.022, 0.026, 0.028], ["neck-1", 0.02, 0.033, 0.033], ["neck-2", 0.01, 0.0245, 0.023], ["neck-2", 0.036, 0.0152, 0.0152]], BREAST);
  neck(S, "neck-upper", "neck-3", [["neck-2", 0.018, 0.0152, 0.0152], ["neck-2", 0.045, 0.0195, 0.0176], ["neck-3", 0.056, 0.015, 0.0142]], GREEN);
  headLoft(S, "head", [[-0.022, 0.005, 0.008, 0.00768], [-0.015, 0.0078, 0.0172, 0.01498], [0.002, 0.011, 0.0228, 0.01824], [0.026, 0.0098, 0.0206, 0.01728], [0.045, 0.0052, 0.0148, 0.01344], [0.055, 0.0024, 0.0112, 0.01094]], GREEN, 2, 3);
  headLoft(S, "bill", [[0.051, 0.0026, 0.0098, 0.00931], [0.075, 0.0004, 0.0084, 0.01083], [0.099, -0.0016, 0.005, 0.01083], [0.1056, -0.0024, 0.0024, 0.00589]], BILL, 2.7);
  fan(S, "tail", [[-0.022, 0.002, 0.019, 0.009], [0.04, 0.002, 0.02, 0.0052], [0.0855, 0.002, 0.004, 0.0018]], TAIL);
  for (const sd of [-1, 1] as const) {
    const [n, b] = [sideName(sd), sideBone(sd)];
    strip(S, `${n}-wing`, `wing-${b}`, sd, [[0.112, 0.018, -0.022, 0.002], [0.07, 0.032, -0.041, 0.009], [0.0, 0.027, -0.039, 0.0088], [-0.07, 0.02, -0.019, 0.006], [-0.115, 0.0215, 0.006, 0.0034, 0, 0.009], [-0.142, 0.0235, 0.0145, 0.0014, 0, 0.0032]], WING, 0.0085);
    const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
    const exit: [number, number, number] = [knee[0] + (ankle[0] - knee[0]) * 0.55, knee[1] + (ankle[1] - knee[1]) * 0.55, knee[2]];
    limb(S, `${n}-leg`, `shank-${b}`, exit, ankle, [[0.0, 0.0009, 0.0009], [1.0, 0.0009, 0.0009]], SKIN);
    (S.body[S.body.length - 1] as { slender?: boolean }).slender = true;
    S.path(`${n}-foot`, `tarsus-${b}`, [{ p: [ankle[0], ankle[1], ankle[2]], rl: 0.0022, rs: 0.0022 }, { p: [mtp[0], mtp[1] + 0.001, mtp[2]], rl: 0.0022, rs: 0.0022 }, { p: [mtp[0] + 0.005, mtp[1] - 0.0034, mtp[2]], rl: 0.0022, rs: 0.0022 }, { p: [mtp[0] + 0.056, mtp[1] - 0.0036, mtp[2]], rl: 0.002, rs: 0.002 }], SKIN);
    (S.body[S.body.length - 1] as { slender?: boolean }).slender = true;
  }
}
