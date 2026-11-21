/** 🩼 The 1:100, 1:200 and 1:500 models of `humans/person-on-crutches`, each stated anew for its scale (smallest feature 15, 30 and 75 mm): merged masses and fewer sections, the same man in the same stance, never the finer list with parts taken away.
 *
 * @see ./r3-person-on-crutches-s50.ts
 * @see ./r3-humans-person-on-crutches.md */
import type { Json } from "./volumes-lib.ts";
import { H, L, aboutU, aboutW, boneDeg, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-person-on-crutches-kit.ts";
import { ARM_LEAN, FORE_LEAN, HANDLE_DIR, HANDLE_L, SHAFT, TONE, lean, leanAt, shaft, around, bootSt, crutchToForearm, footAt, footSt, forearmToCrutch, grip, mid, nearOf } from "./r3-person-on-crutches-parts.ts";
import { BONE, CUFF_TURN, OUTER, jointBall, massBall, outlets } from "./r3-person-on-crutches-s50.ts";

type C = { id: string; bone: string; material: string; tone: number; role?: string };
type Row = readonly [number, number, number, number, number];
type Slice = readonly [number, number, number, number];
const c = (id: string, bone: string, material: string, tone: number, role = "body"): C => ({ id, bone, material, tone, role });
const rows = (r: readonly Row[], bone: string): St[] => r.map(([y, b, f, rw, e]) => H(bone, y, mid(y) + b, mid(y) + f, rw, e));
const foot = (bone: "foot" | "toe", s: readonly Slice[], e: number): St[] => s.map(([x, b, t, rw]) => footSt(bone, x, b, t, rw, e));
const TOP: V3 = leanAt([0.062, -0.0635, 0], FORE_LEAN);

type Tables = {
  hoodie: Row[]; hood: Slice[]; seat: Row[]; shorts: St[]; thigh: St[]; shank: St[]; bootLeg: Slice[]; sleeve: St[]; fore: St[]; neck: St[]; head: St[]; hair: St[];
};
/** 🧱 The parts every coarse scale has, from its own tables: trunk, hood, shorts, bare legs, boot leg, sleeves, neck, head, hair. `k` is the densification of the lofts. */
function common(t: Tables, k: number, leftShank: number): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => { for (const s of solids) out.push(s); for (const s of solids) out.push(mirrored(s, toRight, nearOf)); };
  out.push(loft(c("hip", "pelvis", "polyester-short", TONE.shorts), dense(rows(t.seat, "pelvis"), k)));
  out.push(loft(c("torso", "torso", "fleece-hoodie", TONE.hoodie), dense(rows(t.hoodie, "torso"), k)));
  out.push(loft(c("hood", "torso", "fleece-hoodie", 0.38), dense(t.hood.map(([y, b, f, rw]) => H("torso", y, b, f, rw, 2.3)), k)));
  both([loft(c("skirt-l", "thigh-l", "polyester-short", TONE.shortsFar), dense(t.shorts, k)), loft(c("thigh-g-l", "thigh-l", "skin-light", TONE.legFar), dome(dense(t.thigh, k), 1, 0.012, 2))]);
  out.push(loft(c("shank-g-l", "shank-l", "skin-light", TONE.legFar), dome(dense(t.shank.slice(0, leftShank), k), -1, 0.045, 2)));
  out.push(mirrored(loft(c("shank-g-l", "shank-l", "skin-light", TONE.legFar), dome(dense(t.shank, k), -1, 0.045, 2)), toRight, nearOf));
  out.push(loft(c("boot-shaft-l", "shank-l", "boot-shell", TONE.boot), dense(t.bootLeg.map(([y, b, f, rw]) => bootSt(y, b, f, rw)), k)));
  both([loft(c("sleeve-up-l", "arm-l", "fleece-hoodie", TONE.hoodieFar), dome(dome(dense(lean(t.sleeve, ARM_LEAN), k), 1, 0.012, 2), -1, 0.03, 2)), loft(c("sleeve-fore-l", "forearm-l", "fleece-hoodie", TONE.hoodieFar), dome(dense(lean(t.fore, FORE_LEAN), k), -1, 0.04, 2))]);
  out.push(loft(c("neck", "neck", "skin-light", 0.78), t.neck));
  out.push(loft(c("head", "head", "skin-light", TONE.face), dome(dense(t.head, k), -1, 0.005, 2)));
  out.push(loft(c("hair", "head", "hair-dark", TONE.hair), dome(dense(t.hair, k), -1, 0.008, 2)));
  return out;
}
/** 🩼 The bent frame of the left crutch as one slender rod: cuff arm, junction, shaft down to the tip. */
const rod = (end = 0.852): Json => tube(c("crutch-tube-l", "crutch-l", "alu-anodised", TONE.frameFar), [forearmToCrutch(TOP), forearmToCrutch(leanAt([0.2, -0.049, 0], FORE_LEAN)), [0, 0, 0], shaft(0.3), shaft(0.6), shaft(end)], [0.011], { slender: true });
const cuff = (a0: number, a1: number, r0: number, r1: number): Json => revolve(c("crutch-cuff-l", "forearm-l", "cuff-polymer", TONE.cuff), [[a0, r0], [a1, r0], [a1, r1], [a0, r1], [a0, r0]], [0, 0, 0], [0.99703, 0, 0.07697], { sweepRad: 4.71239, transform: { rotation: aboutU(CUFF_TURN) } });
function pair(out: Json[], solids: Json[]): void {
  for (const s of solids) out.push(s);
  for (const s of solids) { const m = mirrored(s, toRight, nearOf); if (m.id === "crutch-cuff-r") m.transform.rotation = aboutU(CUFF_TURN); out.push(m); }
}

/* ---------------------------------------------------------------- 1:100 */

const T100: Tables = {
  hoodie: [[0.92, -0.136, 0.12, 0.19, 2.5], [0.975, -0.139, 0.126, 0.193, 2.5], [1.01, -0.138, 0.134, 0.193, 2.4], [1.1, -0.121, 0.133, 0.186, 2.4], [1.26, -0.126, 0.134, 0.186, 2.4], [1.38, -0.134, 0.135, 0.19, 2.4], [1.47, -0.126, 0.116, 0.19, 2.4], [1.525, -0.104, 0.084, 0.174, 2.3], [1.56, -0.078, 0.058, 0.112, 2.1], [1.58, -0.062, 0.05, 0.078, 2]],
  hood: [[1.45, -0.128, -0.07, 0.09], [1.52, -0.136, -0.03, 0.134], [1.585, -0.095, 0.012, 0.116], [1.615, -0.056, 0.016, 0.066]],
  seat: [[0.815, -0.075, 0.052, 0.152, 2.3], [0.86, -0.1, 0.078, 0.166, 2.4], [0.935, -0.106, 0.09, 0.17, 2.4], [1.045, -0.104, 0.11, 0.168, 2.4]],
  shorts: [st(-0.02, -0.006, -0.01, 0.09, 0.102, 2.3), st(0.05, -0.008, -0.008, 0.091, 0.108, 2.3), st(0.1, -0.001, -0.002, 0.09, 0.103, 2.3), st(0.245, 0.003, 0.003, 0.098, 0.108, 2.3)],
  thigh: [st(0.21, 0.006, 0, 0.075, 0.085, 2.1), st(0.27, 0.007, 0, 0.07, 0.079, 2.1), st(0.38, 0.007, 0.001, 0.059, 0.067, 2.1), st(0.4459, 0.008, 0, 0.056, 0.064, 2.2)],
  shank: [st(-0.02, 0.005, 0, 0.052, 0.057, 2.2), st(0.06, -0.002, -0.001, 0.053, 0.058, 2.1), st(0.15, -0.011, -0.004, 0.057, 0.064, 2), st(0.255, -0.008, -0.002, 0.044, 0.051, 2), st(0.35, -0.004, 0, 0.0335, 0.04, 2), st(0.39, -0.003, 0, 0.0325, 0.039, 2)],
  bootLeg: [[0.432, -0.082, 0.072, 0.074], [0.33, -0.09, 0.067, 0.075], [0.2, -0.073, 0.062, 0.066], [0.092, -0.092, 0.072, 0.0625]],
  sleeve: [st(-0.03, 0, -0.03, 0.038, 0.052), st(0.02, 0.001, -0.008, 0.052, 0.066), st(0.12, 0.004, 0, 0.053, 0.066), st(0.25, 0.001, 0, 0.05, 0.058), st(0.3385, -0.003, 0, 0.0475, 0.053)],
  fore: [st(-0.012, -0.004, 0, 0.043, 0.0485), st(0.06, 0, 0, 0.0445, 0.049), st(0.16, 0, 0, 0.041, 0.043), st(0.225, 0, 0, 0.036, 0.036), st(0.262, 0, 0, 0.0325, 0.029)],
  neck: [st(-0.035, 0.008, 0, 0.07, 0.07), st(0.03, 0.004, 0, 0.059, 0.062), st(0.118, -0.004, 0, 0.056, 0.058)],
  head: [st(-0.08, -0.064, 0, 0.021, 0.019), st(-0.06, -0.048, 0, 0.044, 0.043), st(-0.03, -0.026, 0, 0.06, 0.068), st(0.005, -0.008, 0, 0.0695, 0.087), st(0.05, -0.003, 0, 0.0755, 0.098), st(0.095, 0, 0, 0.0765, 0.097), st(0.135, 0.003, 0, 0.06, 0.074), st(0.156, 0.004, 0, 0.02, 0.024)],
  hair: [st(-0.012, 0.05, 0, 0.05, 0.036), st(0.025, 0.04, 0, 0.075, 0.058), st(0.07, 0.02, 0, 0.0825, 0.084), st(0.11, 0.008, 0, 0.081, 0.094), st(0.145, 0.004, 0, 0.06, 0.07), st(0.162, 0.004, 0, 0.02, 0.024)],
};
export function body100(): Json[] {
  const out = common(T100, 3, 3);
  const s = (id: string, bone: string, tone: number): C => c(id, bone, "skin-light", tone);
  out.push(loft(c("collar", "neck", "fleece-hoodie", 0.38), [st(-0.045, 0.012, 0, 0.09, 0.087, 2.1, -18), st(0.022, 0.017, 0, 0.067, 0.068, 2.1, -18)]));
  out.push(loft(c("sock-r", "shank-r", "cotton-sock", TONE.sock), [st(0.36, -0.004, 0, 0.0355, 0.0425), st(0.44, 0.002, 0, 0.039, 0.0455)]));
  const m = (id: string, bone: string, tone: number = TONE.sneaker): C => c(id, bone, "sneaker-mesh", tone);
  out.push(loft(m("shoe-rear-r", "foot-r"), dense(foot("foot", [[-0.086, 0.03, 0.08, 0.021], [-0.06, 0.012, 0.108, 0.037], [0, 0.012, 0.11, 0.044], [0.08, 0.012, 0.087, 0.052], [0.158, 0.012, 0.06, 0.0555]], 2.5), 2)));
  out.push(loft(m("shoe-front-r", "toe-r"), dense(foot("toe", [[0.14, 0.012, 0.064, 0.055], [0.195, 0.013, 0.05, 0.047], [0.2185, 0.018, 0.032, 0.016]], 2.5), 2)));
  out.push(loft(m("sole-r", "foot-r", TONE.sole), foot("foot", [[-0.091, 0.004, 0.031, 0.027], [-0.06, 0, 0.033, 0.04], [0.03, 0, 0.03, 0.0455], [0.158, 0, 0.023, 0.058]], 3.2)));
  out.push(loft(m("sole-front-r", "toe-r", TONE.sole), foot("toe", [[0.14, 0, 0.024, 0.058], [0.2, 0.002, 0.021, 0.048], [0.2195, 0.008, 0.021, 0.022]], 3.2)));
  const b = (id: string, bone: string, tone: number = TONE.boot, role = "body"): C => c(id, bone, "boot-shell", tone, role);
  out.push(loft(b("shoe-rear-l", "foot-l"), dense(foot("foot", [[-0.101, 0.03, 0.1, 0.03], [-0.07, 0.012, 0.135, 0.055], [0.01, 0.012, 0.152, 0.0615], [0.09, 0.012, 0.106, 0.0615], [0.158, 0.012, 0.08, 0.061]], 2.8), 2)));
  out.push(loft(b("shoe-front-l", "toe-l"), dense(foot("toe", [[0.14, 0.012, 0.084, 0.061], [0.195, 0.013, 0.07, 0.055], [0.217, 0.018, 0.05, 0.034]], 2.8), 2)));
  out.push(loft(b("sole-l", "foot-l", 0.1), foot("foot", [[-0.108, 0, 0.016, 0.04], [-0.07, -0.013, 0.016, 0.06], [0.06, -0.014, 0.016, 0.064], [0.158, -0.011, 0.016, 0.064]], 3.4)));
  out.push(loft(b("sole-front-l", "toe-l", 0.1), foot("toe", [[0.14, -0.012, 0.016, 0.064], [0.2, -0.001, 0.016, 0.056], [0.222, 0.008, 0.02, 0.034]], 3.4)));
  const strap: [number, number, number, number, number][] = [[0.405, -0.088, 0.075, 0.0785, 0], [0.315, -0.094, 0.071, 0.079, 1], [0.225, -0.08, 0.067, 0.0715, 2]];
  for (const [y, bk, fr, rw, i] of strap) out.push(loft(b(`boot-strap-${i}`, "shank-l", 0.6, "detail"), [bootSt(y + 0.022, bk, fr, rw), bootSt(y - 0.022, bk + 0.002, fr - 0.001, rw - 0.001)]));
  const arm: Json[] = [];
  arm.push(loft(s("wrist-l", "forearm-l", TONE.handFar), lean([st(0.24, 0, 0, 0.03, 0.0235), st(0.285, 0.001, 0, 0.032, 0.0235)], FORE_LEAN)));
  arm.push(ellipsoid(s("hand-l", "hand-l", TONE.handFar), [0.045, 0.017, 0.03], grip(-0.02, 0.03, 0.01), aboutW(30)));
  for (const [n, a] of [["1", 0.02], ["2", -0.002], ["3", -0.024], ["4", -0.045]] as const) {
    arm.push(capsule(s(`hand-l-finger-${n}`, "hand-l", TONE.handFar), around(a, 62, 0.036), around(a, 160, 0.029), 0.0105, 0.0095));
    arm.push(capsule(s(`hand-l-finger-${n}-tip`, "hand-l", TONE.handFar), around(a, 160, 0.029), around(a, 250, 0.0258), 0.0095, 0.0082));
  }
  arm.push(capsule(s("thumb-l", "hand-l", TONE.handFar), grip(-0.03, 0.024, -0.02), around(0.02, -95, 0.028), 0.012, 0.01));
  arm.push(revolve(c("crutch-tube-l", "crutch-l", "alu-anodised", TONE.frameFar), [[-0.012, 0], [-0.012, 0.0105], [0.858, 0.0105], [0.858, 0]], [0, 0, 0], SHAFT));
  arm.push(revolve(c("crutch-grip-l", "crutch-l", "rubber-black", TONE.grip), [[0.012, 0], [0.012, 0.0165], [0.1, 0.0165], [0.108, 0.0195], [HANDLE_L, 0.0195], [HANDLE_L, 0]], [0, 0, 0], HANDLE_DIR));
  arm.push(revolve(c("crutch-tip-l", "crutch-l", "rubber-black", TONE.tip), [[0.846, 0], [0.846, 0.013], [0.88, 0.016], [0.9, 0.0205], [0.9, 0]], [0, 0, 0], SHAFT));
  arm.push(capsule(c("crutch-bracket-l", "forearm-l", "alu-anodised", TONE.frameFar), crutchToForearm([0, 0, 0]), TOP, 0.0105, 0.0105));
  arm.push(cuff(0.06, 0.145, 0.053, 0.0575));
  pair(out, arm);
  out.push(ellipsoid(s("nose", "head", TONE.face), [0.024, 0.013, 0.012], [0.02, -0.105, 0], aboutW(26)));
  for (const side of [1, -1]) out.push(ellipsoid(s(side > 0 ? "ear-l" : "ear", "head", 0.78), [0.031, 0.017, 0.0078], [0.02, 0.014, side * 0.077], compose(aboutW(-10), aboutU(side * 22))));
  return out;
}

/* ---------------------------------------------------------------- 1:200 */

const T200: Tables = {
  hoodie: [[0.92, -0.137, 0.122, 0.191, 2.5], [1, -0.139, 0.132, 0.194, 2.4], [1.15, -0.12, 0.133, 0.185, 2.4], [1.38, -0.134, 0.135, 0.19, 2.4], [1.5, -0.116, 0.1, 0.184, 2.3], [1.58, -0.064, 0.05, 0.08, 2]],
  hood: [[1.46, -0.13, -0.06, 0.1], [1.53, -0.134, -0.02, 0.134], [1.61, -0.066, 0.016, 0.075]],
  seat: [[0.815, -0.075, 0.052, 0.152, 2.3], [0.9, -0.106, 0.088, 0.17, 2.4], [1.045, -0.104, 0.11, 0.168, 2.4]],
  shorts: [st(-0.02, -0.006, -0.01, 0.09, 0.102, 2.3), st(0.05, -0.008, -0.008, 0.091, 0.108, 2.3), st(0.245, 0.003, 0.003, 0.098, 0.108, 2.3)],
  thigh: [st(0.21, 0.006, 0, 0.075, 0.085), st(0.3, 0.007, 0, 0.067, 0.076), st(0.4459, 0.007, 0, 0.056, 0.063)],
  shank: [st(-0.02, 0.005, 0, 0.052, 0.057), st(0.14, -0.01, -0.004, 0.057, 0.064), st(0.3, -0.006, -0.001, 0.039, 0.046), st(0.42, -0.002, 0, 0.0345, 0.0405)],
  bootLeg: [[0.432, -0.082, 0.072, 0.074], [0.3, -0.088, 0.066, 0.073], [0.092, -0.09, 0.072, 0.063]],
  sleeve: [st(-0.03, 0, -0.03, 0.038, 0.052), st(0.03, 0.001, -0.006, 0.053, 0.067), st(0.2, 0.003, 0, 0.052, 0.062), st(0.3385, -0.003, 0, 0.0475, 0.053)],
  fore: [st(-0.012, -0.004, 0, 0.043, 0.0485), st(0.06, 0, 0, 0.0445, 0.049), st(0.2, 0, 0, 0.038, 0.039), st(0.275, 0, 0, 0.031, 0.026)],
  neck: [st(-0.035, 0.008, 0, 0.07, 0.07), st(0.118, -0.004, 0, 0.056, 0.058)],
  head: [st(-0.08, -0.062, 0, 0.024, 0.022), st(-0.045, -0.036, 0, 0.054, 0.057), st(0.005, -0.008, 0, 0.07, 0.088), st(0.07, -0.002, 0, 0.0765, 0.099), st(0.13, 0.003, 0, 0.064, 0.08), st(0.156, 0.004, 0, 0.02, 0.024)],
  hair: [st(-0.012, 0.05, 0, 0.05, 0.036), st(0.04, 0.036, 0, 0.0795, 0.066), st(0.1, 0.01, 0, 0.0825, 0.093), st(0.145, 0.004, 0, 0.06, 0.07), st(0.162, 0.004, 0, 0.02, 0.024)],
};
export function body200(): Json[] {
  const out = common(T200, 3, 2);
  out.push(loft(c("shoe-rear-r", "foot-r", "sneaker-mesh", TONE.sneaker), foot("foot", [[-0.09, 0.004, 0.08, 0.024], [-0.05, 0, 0.112, 0.042], [0.02, 0, 0.11, 0.047], [0.1, 0, 0.078, 0.055], [0.158, 0, 0.06, 0.058]], 2.6)));
  out.push(loft(c("shoe-front-r", "toe-r", "sneaker-mesh", TONE.sneaker), foot("toe", [[0.14, 0, 0.064, 0.058], [0.2, 0.002, 0.048, 0.046], [0.2195, 0.008, 0.03, 0.02]], 2.6)));
  out.push(loft(c("shoe-rear-l", "foot-l", "boot-shell", TONE.boot), foot("foot", [[-0.106, 0, 0.1, 0.034], [-0.06, -0.013, 0.14, 0.058], [0.02, -0.014, 0.15, 0.063], [0.158, -0.011, 0.08, 0.064]], 2.8)));
  out.push(loft(c("shoe-front-l", "toe-l", "boot-shell", TONE.boot), foot("toe", [[0.14, -0.012, 0.084, 0.064], [0.2, -0.001, 0.068, 0.056], [0.222, 0.008, 0.048, 0.034]], 2.8)));
  const arm: Json[] = [];
  arm.push(ellipsoid(c("hand-l", "hand-l", "skin-light", TONE.handFar), [0.047, 0.04, 0.035], grip(-0.014, 0.008, 0.006), aboutW(30)));
  arm.push(rod());
  arm.push(revolve(c("crutch-grip-l", "crutch-l", "rubber-black", TONE.grip), [[0.03, 0], [0.03, 0.0165], [HANDLE_L, 0.0185], [HANDLE_L, 0]], [0, 0, 0], HANDLE_DIR));
  arm.push(revolve(c("crutch-tip-l", "crutch-l", "rubber-black", TONE.tip), [[0.846, 0], [0.846, 0.015], [0.9, 0.0205], [0.9, 0]], [0, 0, 0], SHAFT));
  arm.push(cuff(0.06, 0.146, 0.052, 0.058));
  pair(out, arm);
  out.push(ellipsoid(c("nose", "head", "skin-light", TONE.face), [0.024, 0.015, 0.015], [0.018, -0.102, 0], aboutW(26)));
  return out;
}

/* ---------------------------------------------------------------- 1:500 */

const T500: Tables = {
  hoodie: [[0.92, -0.137, 0.122, 0.191, 2.5], [1.02, -0.137, 0.133, 0.193, 2.4], [1.3, -0.128, 0.134, 0.187, 2.4], [1.48, -0.124, 0.112, 0.19, 2.3], [1.58, -0.064, 0.05, 0.08, 2]],
  hood: [[1.46, -0.132, -0.05, 0.105], [1.54, -0.132, -0.015, 0.13], [1.61, -0.07, 0.02, 0.08]],
  seat: [[0.815, -0.08, 0.055, 0.155, 2.3], [0.9, -0.106, 0.088, 0.17, 2.4], [1.045, -0.104, 0.11, 0.168, 2.4]],
  shorts: [st(-0.02, -0.006, -0.01, 0.09, 0.102, 2.3), st(0.05, -0.008, -0.008, 0.091, 0.108, 2.3), st(0.245, 0.003, 0.003, 0.098, 0.108, 2.3)],
  thigh: [st(0.21, 0.006, 0, 0.075, 0.085), st(0.4459, 0.007, 0, 0.056, 0.063)],
  shank: [st(-0.02, 0.005, 0, 0.052, 0.057), st(0.15, -0.01, -0.004, 0.057, 0.064), st(0.42, -0.003, 0, 0.0375, 0.0405)],
  bootLeg: [[0.432, -0.082, 0.072, 0.074], [0.092, -0.09, 0.072, 0.064]],
  sleeve: [st(-0.03, 0, -0.03, 0.04, 0.053), st(0.04, 0.001, -0.004, 0.053, 0.067), st(0.3385, -0.003, 0, 0.0475, 0.053)],
  fore: [st(-0.012, -0.004, 0, 0.043, 0.0485), st(0.07, 0, 0, 0.0445, 0.049), st(0.275, 0, 0, 0.0375, 0.0375)],
  neck: [st(-0.035, 0.008, 0, 0.07, 0.07), st(0.118, -0.004, 0, 0.056, 0.058)],
  head: [st(-0.075, -0.055, 0, 0.03, 0.03), st(-0.02, -0.02, 0, 0.064, 0.075), st(0.06, -0.002, 0, 0.0765, 0.099), st(0.13, 0.003, 0, 0.064, 0.08), st(0.156, 0.004, 0, 0.02, 0.024)],
  hair: [st(-0.012, 0.05, 0, 0.05, 0.0375), st(0.05, 0.03, 0, 0.082, 0.074), st(0.1, 0.01, 0, 0.083, 0.094), st(0.14, 0.004, 0, 0.066, 0.078), st(0.164, 0.004, 0, 0.02, 0.024)],
};
export function body500(): Json[] {
  const out = common(T500, 2, 2);
  out.push(loft(c("shoe-r", "foot-r", "sneaker-mesh", TONE.sneaker), foot("foot", [[-0.09, 0, 0.085, 0.03], [-0.03, 0, 0.112, 0.045], [0.08, 0, 0.085, 0.054], [0.17, 0, 0.058, 0.056], [0.21, 0.004, 0.044, 0.042], [0.2195, 0.008, 0.034, 0.024]], 2.6)));
  out.push(loft(c("shoe-l", "foot-l", "boot-shell", TONE.boot), foot("foot", [[-0.106, -0.005, 0.1, 0.04], [-0.04, -0.014, 0.148, 0.062], [0.1, -0.013, 0.1, 0.064], [0.222, 0, 0.05, 0.04]], 2.8)));
  const arm: Json[] = [];
  arm.push(ellipsoid(c("hand-l", "hand-l", "skin-light", TONE.handFar), [0.047, 0.04, 0.0375], grip(-0.014, 0.008, 0.006), aboutW(30)));
  arm.push(rod(0.9));
  arm.push(cuff(0.058, 0.148, 0.05, 0.059));
  pair(out, arm);
  return out;
}

/* ---------------------------------------------------------------- thermal */

function regionOf(s: Json): string {
  if (s.material === "fleece-hoodie") return "hoodie";
  if (s.material === "polyester-short") return "shorts";
  if (s.material === "boot-shell") return "boot";
  if (s.material === "sneaker-mesh" || s.material === "cotton-sock") return "sneaker";
  if (s.material === "hair-dark") return "hair";
  if (s.material === "alu-anodised") return "crutch-frame";
  if (s.material === "cuff-polymer") return "crutch-cuffs";
  if (s.material === "rubber-black") return /grip/.test(s.id) ? "crutch-grips" : "crutch-tips";
  if (s.material === "skin-light") return /^(head|neck)$/.test(s.bone) ? "face-skin" : /^(hand|forearm)/.test(s.bone) ? "hands" : "legs-skin";
  throw new Error(`no region for ${s.id}`);
}
const mirrorZ = (p: V3): V3 => [p[0], p[1], -p[2]];
/** 🌡️ The thermal list of a coarse scale: every body solid by reference in its region and outer layer; at 1:100 face and neck as open skins of the visible part; the three outlets; at 1:500 the rubber grips and ferrules, which the body no longer shows (the rod runs to the ground), at the size floor. */
function thermal(body: readonly Json[], scale: 100 | 200 | 500, t: Tables): Json[] {
  const out: Json[] = [];
  for (const s of body) {
    const region = regionOf(s);
    if (scale === 100 && (s.id === "head" || s.id === "neck")) continue;
    out.push(thOf(s.id, region, s.material === "cotton-sock" ? "sneaker-sock" : OUTER[region]));
  }
  if (scale === 100) {
    out.push({ ...loft({ id: "th-head", bone: "head", material: "skin-light", tone: TONE.face }, dome(dense(t.head.slice(0, 5), 2), -1, 0.005, 2), [true, false]), region: "face-skin", sectionFaces: ["+v"] });
    out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin-light", tone: 0.78 }, t.neck.slice(1), [false, false]), region: "face-skin" });
  }
  out.push(...outlets((0.15 * scale) / 1000));
  if (scale === 500) {
    for (const side of ["l", "r"]) {
      out.push({ ...capsule(c(`th-crutch-grip-${side}`, `crutch-${side}`, "rubber-black", TONE.grip), [0.0375 * HANDLE_DIR[0], 0.0375 * HANDLE_DIR[1], 0], [0.085 * HANDLE_DIR[0], 0.085 * HANDLE_DIR[1], 0], 0.0375, 0.0375), region: "crutch-grips" });
      out.push({ ...capsule(c(`th-crutch-tip-${side}`, `crutch-${side}`, "rubber-black", TONE.tip), side === "l" ? shaft(0.8475) : mirrorZ(shaft(0.8475)), side === "l" ? shaft(0.8625) : mirrorZ(shaft(0.8625)), 0.0375, 0.0375), region: "crutch-tips" });
    }
  }
  return out;
}
export const thermal100 = (body: readonly Json[]): Json[] => thermal(body, 100, T100);
export const thermal200 = (body: readonly Json[]): Json[] => thermal(body, 200, T200);
export const thermal500 = (body: readonly Json[]): Json[] => thermal(body, 500, T500);

/* ---------------------------------------------------------------- structure */

const b = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, ...BONE });
const T = (x: number, y: number, z = 0): V3 => L("torso", [mid(y) + x, y, z]);
const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
const member = (solid: Json, bone: string): Json => part(solid, bone, "member");
const mirror2 = (out: Json[], limb: Json[]): void => { for (const s of limb) out.push(s); for (const s of limb) out.push(mirrored(s, toRight)); };
const crutchRefs = (ids: readonly string[]): Json[] => ids.flatMap((id) => [id, toRight(id)]).map((id) => ({ id: `member-${id}`, of: id, member: id.endsWith("-l") ? "crutch-l" : "crutch-r", part: "member" }));
function hoops(list: readonly [number, number, number][], r: number): Json[] {
  const tilt = 24 - (90 - boneDeg("torso"));
  return list.map(([y, rd, rw], i) => {
    const ring: [number, number][] = Array.from({ length: 7 }, (_, k) => [r * Math.cos((k * Math.PI) / 3), rd + r * Math.sin((k * Math.PI) / 3)] as [number, number]);
    return member(revolve(b(`rib-hoop-${i + 1}`, "torso"), ring, [0, 0, 0], [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000, transform: { translation: T(-0.008, y).map((x) => Math.round(x * 1e5) / 1e5), rotation: aboutW(-tilt) } }), "torso");
  });
}

export function structure100(): Json[] {
  const out: Json[] = [];
  const spine: [number, number, number][] = [[-0.044, 1.048, 0.022], [-0.024, 1.13, 0.02], [-0.03, 1.22, 0.019], [-0.056, 1.35, 0.017], [-0.058, 1.46, 0.015], [-0.022, 1.566, 0.013]];
  out.push(member(tube(b("spine", "torso"), spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso"));
  out.push(...hoops([[1.52, 0.054, 0.082], [1.46, 0.082, 0.126], [1.395, 0.098, 0.148], [1.33, 0.105, 0.157], [1.255, 0.098, 0.15]], 0.008));
  out.push(member(loft(b("sternum", "torso"), [H("torso", 1.3, mid(1.3) + 0.084, mid(1.3) + 0.1, 0.014), H("torso", 1.42, mid(1.42) + 0.086, mid(1.42) + 0.102, 0.02), H("torso", 1.515, mid(1.515) + 0.056, mid(1.515) + 0.072, 0.028)]), "torso"));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(member(capsule(b(`clavicle-${n}`, "torso"), T(0.058, 1.516, side * 0.022), T(-0.012, 1.534, side * 0.188), 0.0078, 0.0078), "torso"));
    out.push(member(ellipsoid(b(`scapula-${n}`, "torso"), [0.074, 0.0078, 0.05], T(-0.108, 1.43, side * 0.112), compose(aboutW(4), aboutU(side * 28))), "torso"));
    out.push(member(ellipsoid(b(`ilium-${n}`, "pelvis"), [0.06, 0.066, 0.012], P(-0.024, 1.018, side * 0.108), compose(aboutU(side * 30))), "pelvis"));
    out.push(member(capsule(b(`ilium-body-${n}`, "pelvis"), P(-0.062, 1.012, side * 0.034), P(0.002, 0.96, side * 0.086), 0.017, 0.019), "pelvis"));
    out.push(member(capsule(b(`pubis-${n}`, "pelvis"), P(0.012, 0.957, side * 0.084), P(0.062, 0.93, side * 0.012), 0.012, 0.011), "pelvis"));
    out.push(member(capsule(b(`ischium-${n}`, "pelvis"), P(-0.012, 0.955, side * 0.082), P(-0.046, 0.878, side * 0.06), 0.013, 0.015), "pelvis"));
    out.push(member(capsule(b(`mandible-${n}`, "head"), [0.0, -0.006, side * 0.048], [-0.066, -0.066, side * 0.01], 0.008, 0.009), "head"));
  }
  out.push(member(tube(b("sacrum", "pelvis"), [P(-0.044, 1.048), P(-0.072, 0.98), P(-0.07, 0.932)], [0.028, 0.022, 0.009]), "pelvis"));
  out.push(member(ellipsoid(b("cranium", "head"), [0.066, 0.09, 0.0685], [0.086, 0.01, 0]), "head"));
  out.push(member(ellipsoid(b("face-bones", "head"), [0.044, 0.038, 0.052], [0.012, -0.052, 0]), "head"));
  out.push(member(tube(b("cervical-spine", "neck"), [L("neck", [0.039, 1.566, 0]), L("neck", [0.053, 1.615, 0]), L("neck", [0.062, 1.664, 0])], [0.013, 0.012, 0.011]), "neck"));
  const limb: Json[] = [];
  limb.push(member(capsule(b("femur-neck-l", "thigh-l"), [0, 0, 0], [0.04, -0.004, 0.062], 0.019, 0.02), "thigh-l"));
  limb.push(member(tube(b("femur-l", "thigh-l"), [[0.046, -0.004, 0.062], [0.23, 0.014, 0.028], [0.425, 0.002, 0.002]], [0.016, 0.014, 0.018]), "thigh-l"));
  limb.push(member(ellipsoid(b("femur-condyles-l", "thigh-l"), [0.022, 0.032, 0.042], [0.438, -0.004, 0]), "thigh-l"));
  limb.push(member(tube(b("tibia-l", "shank-l"), [[0.012, 0, -0.002], [0.05, 0.008, -0.004], [0.28, 0.008, -0.004], [0.424, 0.002, -0.003]], [0.03, 0.016, 0.0125, 0.016]), "shank-l"));
  limb.push(member(capsule(b("fibula-l", "shank-l"), [0.036, -0.02, 0.034], [0.432, -0.012, 0.027], 0.0076, 0.0078), "shank-l"));
  limb.push(member(capsule(b("calcaneus-l", "foot-l"), footAt("foot", -0.052, 0.062), footAt("foot", 0.012, 0.072), 0.02, 0.019), "foot-l"));
  limb.push(member(loft(b("metatarsals-l", "foot-l"), [footSt("foot", 0.03, 0.05, 0.082, 0.026, 2.2), footSt("foot", 0.1, 0.04, 0.062, 0.037, 2.2), footSt("foot", 0.158, 0.03, 0.046, 0.044, 2.2)]), "foot-l"));
  limb.push(member(loft(b("phalanges-l", "toe-l"), [footSt("toe", 0.146, 0.03, 0.046, 0.044, 2.2), footSt("toe", 0.209, 0.03, 0.045, 0.03, 2.2)]), "toe-l"));
  limb.push(member(tube(b("humerus-l", "arm-l"), ([[0, 0, -0.012], [0.03, 0, -0.008], [0.2, 0.003, 0], [0.332, 0, 0]] as V3[]).map((q) => leanAt(q, ARM_LEAN)), [0.022, 0.0125, 0.0105, 0.014]), "arm-l"));
  limb.push(member(capsule(b("ulna-l", "forearm-l"), [-0.014, -0.018, 0], leanAt([0.262, -0.011, 0.006], FORE_LEAN), 0.0085, 0.0076), "forearm-l"));
  limb.push(member(capsule(b("radius-l", "forearm-l"), [0.012, 0.014, 0], leanAt([0.264, 0.008, -0.004], FORE_LEAN), 0.0076, 0.009), "forearm-l"));
  limb.push(member(ellipsoid(b("hand-plate-l", "hand-l"), [0.04, 0.009, 0.028], grip(-0.02, 0.031, 0.014), compose(aboutW(30), aboutU(-35))), "hand-l"));
  for (const [n, a] of [["1", 0.02], ["2", -0.002], ["3", -0.024], ["4", -0.045]] as const) limb.push(member(capsule(b(`finger-bones-l-${n}`, "hand-l"), around(a, 62, 0.036), around(a, 150, 0.03), 0.0076, 0.0076), "hand-l"));
  mirror2(out, limb);
  out.push(...crutchRefs(["crutch-tube-l", "crutch-bracket-l", "crutch-grip-l", "crutch-tip-l"]));
  out.push(jointBall("torso", T(-0.036, 1.066), 0.017), jointBall("neck", L("neck", [0.039, 1.566, 0]), 0.0135), jointBall("head", [0.004, -0.006, 0], 0.0125));
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.0245), jointBall(`shank-${side}`, [0, 0, 0], 0.019), jointBall(`foot-${side}`, [0, 0, 0], 0.016), jointBall(`arm-${side}`, [0, 0, k * -0.012], 0.0235), jointBall(`forearm-${side}`, [0, 0, 0], 0.0155), jointBall(`hand-${side}`, [0, 0, 0], 0.012));
  }
  for (const id of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) out.push(massBall(id));
  return out;
}

export function structure200(): Json[] {
  const out: Json[] = [];
  out.push(member(ellipsoid(b("skull", "head"), [0.105, 0.088, 0.068], [0.044, -0.008, 0]), "head"));
  out.push(member(capsule(b("cervical-spine", "neck"), L("neck", [0.04, 1.566, 0]), L("neck", [0.06, 1.65, 0]), 0.016, 0.015), "neck"));
  const spine: [number, number, number][] = [[-0.04, 1.05, 0.022], [-0.026, 1.16, 0.02], [-0.056, 1.38, 0.018], [-0.024, 1.56, 0.016]];
  out.push(member(tube(b("spine", "torso"), spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso"));
  out.push(...hoops([[1.49, 0.068, 0.106], [1.4, 0.094, 0.144], [1.29, 0.1, 0.152]], 0.015));
  out.push(member(capsule(b("shoulder-girdle", "torso"), T(-0.02, 1.528, -0.186), T(-0.02, 1.528, 0.186), 0.015, 0.015), "torso"));
  const ring: [number, number][] = Array.from({ length: 7 }, (_, k) => [0.018 * Math.cos((k * Math.PI) / 3), 0.07 + 0.018 * Math.sin((k * Math.PI) / 3)] as [number, number]);
  out.push(member(revolve(b("pelvic-ring", "pelvis"), ring, [0, 0, 0], [1, 0, 0], { lateralScale: 1.55, transform: { translation: P(-0.022, 0.99).map((x) => Math.round(x * 1e5) / 1e5), rotation: aboutW(-28) } }), "pelvis"));
  const limb: Json[] = [];
  limb.push(member(tube(b("femur-l", "thigh-l"), [[0, 0, 0], [0.04, -0.004, 0.06], [0.24, 0.012, 0.028], [0.43, 0.002, 0.002]], [0.022, 0.018, 0.016, 0.022]), "thigh-l"));
  limb.push(member(capsule(b("tibia-l", "shank-l"), [0.02, 0.004, -0.003], [0.42, 0.002, -0.003], 0.02, 0.0155), "shank-l"));
  limb.push(member(capsule(b("foot-bones-l", "foot-l"), footAt("foot", -0.045, 0.062), footAt("foot", 0.15, 0.044), 0.019, 0.0155), "foot-l"));
  limb.push(member(capsule(b("humerus-l", "arm-l"), [0, 0, -0.01], leanAt([0.33, 0, 0], ARM_LEAN), 0.019, 0.0155), "arm-l"));
  limb.push(member(capsule(b("forearm-bones-l", "forearm-l"), [0, -0.002, 0], leanAt([0.262, 0, 0], FORE_LEAN), 0.017, 0.0155), "forearm-l"));
  limb.push(member(capsule(b("hand-bones-l", "hand-l"), grip(-0.03, 0.034, 0.004), around(-0.012, 70, 0.032), 0.016, 0.0155), "hand-l"));
  mirror2(out, limb);
  out.push(...crutchRefs(["crutch-tube-l"]));
  out.push(jointBall("torso", T(-0.036, 1.066), 0.02), jointBall("head", [0.004, -0.006, 0], 0.017));
  for (const side of ["l", "r"]) out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.0245), jointBall(`shank-${side}`, [0, 0, 0], 0.022), jointBall(`foot-${side}`, [0, 0, 0], 0.018), jointBall(`arm-${side}`, [0, 0, side === "l" ? -0.01 : 0.01], 0.0235), jointBall(`forearm-${side}`, [0, 0, 0], 0.018), jointBall(`hand-${side}`, [0, 0, 0], 0.016));
  for (const id of ["pelvis", "torso", "head", "thigh-l", "thigh-r"]) out.push(massBall(id, 0.016));
  return out;
}

export function structure500(): Json[] {
  const out: Json[] = [];
  const R = 0.0375;
  out.push(member(ellipsoid(b("skull", "head"), [0.09, 0.075, 0.06], [0.05, -0.006, 0]), "head"));
  out.push(member(capsule(b("cervical-spine", "neck"), L("neck", [0.05, 1.575, 0]), L("neck", [0.06, 1.64, 0]), R, R), "neck"));
  const spine: [number, number][] = [[-0.036, 1.07], [-0.024, 1.17], [-0.05, 1.3], [-0.064, 1.41], [-0.036, 1.53]];
  out.push(member(loft(b("spine", "torso"), dome(dome(dense(spine.map(([x, y]) => H("torso", y, mid(y) + x - R, mid(y) + x + R, R)), 3), 1, R, 2), -1, R, 2)), "torso"));
  out.push(member(capsule(b("shoulder-girdle", "torso"), T(-0.015, 1.51, -0.16), T(-0.015, 1.51, 0.16), R, R), "torso"));
  out.push(member(capsule(b("pelvic-girdle", "pelvis"), P(-0.01, 0.985, -0.1), P(-0.01, 0.985, 0.1), R, R), "pelvis"));
  const limb: Json[] = [];
  limb.push(member(capsule(b("femur-l", "thigh-l"), [0.01, 0, 0.02], [0.43, 0.002, 0], R, R), "thigh-l"));
  limb.push(member(capsule(b("tibia-l", "shank-l"), [0.03, 0, 0], [0.39, -0.002, 0], R, 0.034), "shank-l"));
  limb.push(member(capsule(b("foot-bones-l", "foot-l"), footAt("foot", -0.03, 0.05), footAt("foot", 0.13, 0.045), R, 0.034), "foot-l"));
  limb.push(member(capsule(b("humerus-l", "arm-l"), [0.01, 0, -0.004], leanAt([0.32, 0, 0], ARM_LEAN), R, R), "arm-l"));
  limb.push(member(capsule(b("forearm-bones-l", "forearm-l"), [0.02, 0, 0], leanAt([0.24, 0, 0], FORE_LEAN), R, 0.034), "forearm-l"));
  limb.push(member(ellipsoid(b("hand-bones-l", "hand-l"), [R, 0.0362, 0.0362], grip(-0.014, 0.012, 0.006), aboutW(30)), "hand-l"));
  mirror2(out, limb);
  out.push(...crutchRefs(["crutch-tube-l"]));
  for (const side of ["l", "r"]) out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.04), jointBall(`shank-${side}`, [0, 0, 0], 0.04), jointBall(`arm-${side}`, [0, 0, 0], 0.04), jointBall(`forearm-${side}`, [0, 0, 0], 0.04));
  for (const id of ["pelvis", "torso"]) out.push(massBall(id, 0.04));
  return out;
}
export const DECISIONS: Record<number, Json[]> = {
  100: [
    { en: "Stated anew for the 15 mm feature: the same man and stance with fewer sections; the face keeps nose and ears only, the hood, collar, sleeves, shorts legs and bare legs are single lofts, the hands are a palm with four two-part fingers and a thumb around the handle.", de: "Neu für die kleinste Einzelheit von 15 mm angelegt: derselbe Mann in derselben Haltung mit weniger Schnitten; das Gesicht behält nur Nase und Ohren, Kapuze, Kragen, Ärmel, Hosenbeine und nackte Beine sind je ein Loft, die Hände eine Handfläche mit vier zweigliedrigen Fingern und Daumen um den Griff." },
    { en: "The boot keeps shell, sole and three straps, the sneaker sole and upper; each crutch is shaft tube, bracket, handle, ferrule and cuff (clip, holes and lower tube merged into the tube).", de: "Die Orthese behält Schale, Sohle und drei Riemen, der Turnschuh Sohle und Schaft; jede Krücke besteht aus Schaftrohr, Halter, Griff, Puffer und Manschette (Clip, Löcher und Unterrohr im Rohr aufgegangen)." },
    { en: "Thermal: every body solid in its region and outer layer, face and neck as open skins of the visible part; structure: skull, curved spine, five rib hoops, shoulder girdle, pelvis, long bones, simplified hands and feet, crutch frames, joints and segment masses.", de: "Thermik: jeder Körper in seiner Region und äußeren Lage, Gesicht und Hals als offene Häute des sichtbaren Teils; Tragwerk: Schädel, gekrümmte Wirbelsäule, fünf Rippenbögen, Schultergürtel, Becken, Röhrenknochen, vereinfachte Hände und Füße, Krückengestelle, Gelenke und Segmentmassen." },
  ],
  200: [
    { en: "Stated anew for the 30 mm feature: trunk with hood, shorts, legs, boot shaft and shoes, sleeves, head with nose and hair as merged masses; each hand is one mass around its handle, the crutch rod is a slender 22 mm stroke with grip, ferrule and cuff.", de: "Neu für die kleinste Einzelheit von 30 mm angelegt: Rumpf mit Kapuze, Hose, Beine, Orthesenschaft und Schuhe, Ärmel, Kopf mit Nase und Haar als zusammengefasste Massen; jede Hand ist eine Masse um ihren Griff, die Krückenstange ein schlanker Strich von 22 mm mit Griff, Puffer und Manschette." },
    { en: "Thermal follows the body solid by solid; structure: skull, spine, three rib hoops, shoulder girdle, pelvic ring, the long bones of arm and leg, hand and foot bones as one member each, the crutch rods, joints and the four main masses.", de: "Die Thermik folgt dem Körper Stück für Stück; Tragwerk: Schädel, Wirbelsäule, drei Rippenbögen, Schultergürtel, Beckenring, die Röhrenknochen von Arm und Bein, Hand- und Fußknochen als je ein Stab, die Krückenstangen, Gelenke und die vier Hauptmassen." },
  ],
  500: [
    { en: "Stated anew for the 75 mm feature: hooded trunk, shorts, legs, the boot as one shaft and shoe, a sneaker, sleeves, head and hair as few-section masses that keep the proportions; each hand is one mass, the crutch is one slender rod from cuff to ground with its cuff, because handle and ferrule fall below the feature.", de: "Neu für die kleinste Einzelheit von 75 mm angelegt: Rumpf mit Kapuze, Hose, Beine, die Orthese als ein Schaft mit Schuh, ein Turnschuh, Ärmel, Kopf und Haar als Massen mit wenigen Schnitten, welche die Proportionen halten; jede Hand ist eine Masse, die Krücke eine schlanke Stange von der Manschette bis zum Boden mit ihrer Manschette, weil Griff und Puffer unter die Einzelheit fallen." },
    { en: "Thermal keeps every region: grips and ferrules, which the body no longer shows, are surfaces at the size floor on the rod; structure: skull, spine, shoulder and pelvic girdles, long bones, one member each for hand and foot bones, the crutch rods, the joints of the limbs and the trunk masses.", de: "Die Thermik behält jede Region: Griffe und Puffer, die der Körper nicht mehr zeigt, sind Flächen in Mindestgröße an der Stange; Tragwerk: Schädel, Wirbelsäule, Schulter- und Beckengürtel, Röhrenknochen, je ein Stab für Hand- und Fußknochen, die Krückenstangen, die Gelenke der Glieder und die Rumpfmassen." },
  ],
};
