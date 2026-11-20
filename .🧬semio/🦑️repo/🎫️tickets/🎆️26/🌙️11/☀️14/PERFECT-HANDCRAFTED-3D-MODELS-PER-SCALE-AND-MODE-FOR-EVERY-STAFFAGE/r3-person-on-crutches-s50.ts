/** 🩼 The 1:50 model of `humans/person-on-crutches`, stated solid by solid: body (garments over the body, bare legs, sneaker, walking boot, gripping hands, two forearm crutches), thermal model (layer shells, visible skins, outlets) and the skeleton with the two crutch frames as the load path.
 *
 * @see ./r3-person-on-crutches-parts.ts
 * @see ./r3-humans-person-on-crutches.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, H, L, aboutU, aboutV, aboutW, boneDeg, box, capsule, compose, dense, dome, ellipsoid, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-person-on-crutches-kit.ts";
import { CUFF, FORE, HAIR, HANDLE_DIR, HANDLE_L, HEAD, HOODIE, NECK, SEAT, SHANK, SHORTS_LEG, SLEEVE, THIGH, TONE, WRIST, ARM_LEAN, FORE_LEAN, SHAFT, around, bootSt, leanAt, shaft, crutchToForearm, footAt, footSt, grip, handToCrutch, mid, nearOf, rowsAt, surfaceX } from "./r3-person-on-crutches-parts.ts";

type C = { id: string; bone: string; material: string; tone: number; role?: string };
const c = (id: string, bone: string, material: string, tone: number, role = "body"): C => ({ id, bone, material, tone, role });

/* ---------------------------------------------------------------- hands, feet, crutch (left side, mirrored to the right) */

/** ✊ The left hand closed over the handle: carpus above the rear of the grip, four metacarpals over its outer top, three-part fingers wrapped under it to the inside, the thumb round the inside. */
export function hand(): Json[] {
  const s = (id: string): C => c(id, "hand-l", "skin-light", TONE.handFar);
  const out: Json[] = [];
  const rays: [string, number, number, number][] = [["1", 0.02, 1, 255], ["2", -0.002, 1.05, 258], ["3", -0.024, 1, 255], ["4", -0.045, 0.84, 238]];
  out.push(ellipsoid(s("hand-l"), [0.02, 0.0145, 0.0235], grip(-0.03, 0.036, 0.004), aboutW(30)));
  out.push(ellipsoid(s("thumb-l"), [0.027, 0.014, 0.0165], grip(-0.02, 0.026, -0.014), aboutW(30)));
  for (const [n, a, k, end] of rays) {
    const p = [grip(-0.03 + 0.35 * (a + 0.0125), 0.037, 0.006), around(a, 62, 0.036), around(a, 135, 0.031 * (0.94 + 0.06 * k)), around(a, 200, 0.027), around(a, end, 0.0255)];
    out.push(capsule(s(`hand-l-metacarpal-${n}`), p[0]!, p[1]!, 0.0125, 0.0115 * k));
    out.push(capsule(s(`hand-l-finger-${n}`), p[1]!, p[2]!, 0.0098 * k, 0.0092 * k));
    out.push(capsule(s(`hand-l-finger-${n}-mid`), p[2]!, p[3]!, 0.009 * k, 0.0084 * k));
    out.push(capsule(s(`hand-l-finger-${n}-tip`), p[3]!, p[4]!, 0.0084 * k, 0.0078 * k));
  }
  out.push(capsule(s("hand-l-thumb"), grip(-0.03, 0.024, -0.02), grip(-0.002, 0.016, -0.03), 0.0125, 0.0108));
  out.push(capsule(s("hand-l-thumb-tip"), grip(-0.002, 0.016, -0.03), around(0.024, -100, 0.027), 0.0104, 0.009));
  return out;
}

/** 🩼 The left forearm crutch: anodised tube frame (cuff arm, upper tube, clip collar, telescopic lower tube with its row of holes), moulded handle with rubber grip, open polymer cuff round the sleeve, rubber tip. */
export function crutch(): Json[] {
  const f = (id: string, tone: number = TONE.frameFar, role = "body"): C => c(id, "crutch-l", "alu-anodised", tone, role);
  const out: Json[] = [];
  out.push(revolve(f("crutch-tube-l"), [[-0.012, 0], [-0.012, 0.011], [0.468, 0.011], [0.468, 0]], [0, 0, 0], SHAFT));
  out.push(revolve(f("crutch-lower-l"), [[0.46, 0], [0.46, 0.0095], [0.856, 0.0095], [0.856, 0]], [0, 0, 0], SHAFT));
  out.push(revolve(f("crutch-clip-l", 0.3), [[0.452, 0], [0.452, 0.0138], [0.478, 0.0138], [0.484, 0.011], [0.484, 0]], [0, 0, 0], SHAFT));
  out.push(ellipsoid(f("crutch-joint-l", 0.3), [0.024, 0.02, 0.016], [0.002, 0.004, 0]));
  [0.52, 0.57, 0.62, 0.67, 0.72].forEach((u, i) => out.push(ellipsoid(f(`crutch-hole-${i}-l`, 0.1, "detail"), [0.004, 0.002, 0.004], shaft(u, 0.0088), undefined, { texture: true })));
  const g = HANDLE_DIR;
  out.push(revolve(c("crutch-grip-l", "crutch-l", "rubber-black", TONE.grip), [[0.012, 0], [0.012, 0.0165], [0.098, 0.0165], [0.105, 0.0195], [0.113, 0.0195], [HANDLE_L, 0.015], [HANDLE_L, 0]], [0, 0, 0], g));
  out.push(revolve(c("crutch-tip-l", "crutch-l", "rubber-black", TONE.tip), [[0.846, 0], [0.846, 0.0128], [0.868, 0.0135], [0.884, 0.0175], [0.895, 0.0205], [0.9, 0.0195], [0.9, 0]], [0, 0, 0], SHAFT));
  const top: V3 = leanAt([0.062, -0.0635, 0], FORE_LEAN);
  out.push(capsule(c("crutch-bracket-l", "forearm-l", "alu-anodised", TONE.frameFar), crutchToForearm([0, 0, 0]), top, 0.011, 0.011));
  out.push(revolve(c("crutch-cuff-l", "forearm-l", "cuff-polymer", TONE.cuff), [[0.058, 0.0528], [0.146, 0.0528], [0.146, 0.057], [0.058, 0.057], [0.058, 0.0528]], [0, 0, 0], [0.99703, 0, 0.07697], { sweepRad: 4.71239, transform: { rotation: aboutU(CUFF_TURN) } }));
  out.push(box(c("crutch-cuff-mount-l", "forearm-l", "cuff-polymer", TONE.cuff), [0.05, 0.02, 0.03], leanAt([0.1, -0.064, 0], FORE_LEAN), undefined, 0.004));
  return out;
}
/** 🔓 The turn of the cuff about the forearm that puts its opening ahead of the arm. */
export const CUFF_TURN = -45;

/** 👟 The right sneaker: mesh upper with heel counter, collar, tongue and toe box, a foam sole with heel stack and toe spring, laces. */
export function sneaker(): Json[] {
  const m = (id: string, bone: string, tone: number = TONE.sneaker, role = "body"): C => c(id, bone, "sneaker-mesh", tone, role);
  const out: Json[] = [];
  const rear: [number, number, number, number][] = [[-0.086, 0.034, 0.08, 0.021], [-0.072, 0.014, 0.1, 0.034], [-0.04, 0.012, 0.113, 0.0405], [0, 0.012, 0.11, 0.044], [0.04, 0.012, 0.106, 0.048], [0.08, 0.012, 0.087, 0.052], [0.12, 0.012, 0.07, 0.0545], [0.158, 0.012, 0.06, 0.0555]];
  const toe: [number, number, number, number][] = [[0.14, 0.012, 0.064, 0.055], [0.175, 0.012, 0.056, 0.053], [0.2, 0.013, 0.047, 0.044], [0.213, 0.015, 0.038, 0.03], [0.2185, 0.018, 0.03, 0.014]];
  out.push(loft(m("shoe-rear-r", "foot-r"), dense(rear.map(([x, b, t, rw]) => footSt("foot", x, b, t, rw, 2.5)), 3)));
  out.push(loft(m("shoe-front-r", "toe-r"), dense(toe.map(([x, b, t, rw]) => footSt("toe", x, b, t, rw, 2.5)), 3)));
  const sole: [number, number, number, number][] = [[-0.091, 0.006, 0.03, 0.026], [-0.076, 0, 0.033, 0.037], [-0.03, 0, 0.033, 0.0435], [0.03, 0, 0.03, 0.0455], [0.09, 0, 0.026, 0.053], [0.158, 0, 0.023, 0.058]];
  const fore: [number, number, number, number][] = [[0.14, 0, 0.024, 0.058], [0.18, 0, 0.022, 0.0555], [0.205, 0.003, 0.021, 0.045], [0.2195, 0.008, 0.021, 0.022]];
  out.push(loft(m("sole-r", "foot-r", TONE.sole), dense(sole.map(([x, b, t, rw]) => footSt("foot", x, b, t, rw, 3.2)), 2)));
  out.push(loft(m("sole-front-r", "toe-r", TONE.sole), dense(fore.map(([x, b, t, rw]) => footSt("toe", x, b, t, rw, 3.2)), 2)));
  out.push(loft(m("shoe-tongue-r", "foot-r", 0.8, "detail"), [footSt("foot", 0.012, 0.085, 0.126, 0.026, 2), footSt("foot", 0.04, 0.084, 0.118, 0.028, 2), footSt("foot", 0.075, 0.07, 0.097, 0.027, 2)]));
  [0.036, 0.058, 0.08, 0.102].forEach((x, i) => { const y = 0.117 - (x - 0.036) * 0.48; out.push(capsule(m(`lace-r${i}`, "foot-r", 0.5, "detail"), footAt("foot", x, y, -0.024), footAt("foot", x + 0.012, y - 0.004, 0.024), 0.003, 0.003, { texture: true })); });
  return out;
}

/** 🥾 The left walking boot, stated with the shank upright: rocker sole, padded foot part with closed toe, leg shell to below the knee, two side struts and five straps. */
export function boot(): Json[] {
  const b = (id: string, bone: string, tone: number = TONE.boot, role = "body"): C => c(id, bone, "boot-shell", tone, role);
  const out: Json[] = [];
  const leg: [number, number, number, number][] = [[0.432, -0.082, 0.072, 0.074], [0.36, -0.091, 0.068, 0.076], [0.28, -0.083, 0.064, 0.071], [0.2, -0.073, 0.062, 0.066], [0.135, -0.078, 0.066, 0.064], [0.092, -0.092, 0.072, 0.0625]];
  out.push(loft(b("boot-shaft-l", "shank-l"), dense(leg.map(([y, bk, fr, rw]) => bootSt(y, bk, fr, rw)), 3)));
  const rear: [number, number, number, number][] = [[-0.101, 0.03, 0.1, 0.03], [-0.085, 0.014, 0.125, 0.05], [-0.04, 0.012, 0.15, 0.06], [0.01, 0.012, 0.152, 0.0615], [0.05, 0.012, 0.132, 0.0615], [0.09, 0.012, 0.106, 0.0615], [0.125, 0.012, 0.09, 0.0615], [0.158, 0.012, 0.08, 0.061]];
  const toe: [number, number, number, number][] = [[0.14, 0.012, 0.084, 0.061], [0.18, 0.012, 0.075, 0.059], [0.205, 0.014, 0.064, 0.05], [0.217, 0.018, 0.05, 0.034]];
  out.push(loft(b("shoe-rear-l", "foot-l"), dense(rear.map(([x, bt, t, rw]) => footSt("foot", x, bt, t, rw, 2.8)), 3)));
  out.push(loft(b("shoe-front-l", "toe-l"), dense(toe.map(([x, bt, t, rw]) => footSt("toe", x, bt, t, rw, 2.8)), 3)));
  const sole: [number, number, number, number][] = [[-0.108, 0.0, 0.016, 0.04], [-0.09, -0.01, 0.016, 0.056], [-0.03, -0.014, 0.016, 0.063], [0.06, -0.014, 0.016, 0.064], [0.158, -0.011, 0.016, 0.064]];
  const fore: [number, number, number, number][] = [[0.14, -0.012, 0.016, 0.064], [0.185, -0.006, 0.016, 0.062], [0.21, 0.002, 0.017, 0.052], [0.222, 0.008, 0.02, 0.034]];
  out.push(loft(b("sole-l", "foot-l", 0.1), dense(sole.map(([x, bt, t, rw]) => footSt("foot", x, bt, t, rw, 3.4)), 2)));
  out.push(loft(b("sole-front-l", "toe-l", 0.1), dense(fore.map(([x, bt, t, rw]) => footSt("toe", x, bt, t, rw, 3.4)), 2)));
  for (const side of [1, -1]) out.push(box(b(`boot-strut-${side > 0 ? "o" : "i"}-l`, "shank-l", 0.3, "detail"), [0.3, 0.036, 0.008], [0.4477 - (0.275 - 0.083), -0.004, side * 0.0735], undefined, 0.003));
  const straps: [number, number][] = [[0.405, 0], [0.315, 1], [0.225, 2]];
  for (const [y, i] of straps) {
    const k = (h: number): St => { const s = dense(leg.map(([yy, bk, fr, rw]) => bootSt(yy, bk, fr, rw)), 6).reduce((a2, q) => (Math.abs(q.at[0] - (0.4477 - (h - 0.083))) < Math.abs(a2.at[0] - (0.4477 - (h - 0.083))) ? q : a2)); return { ...s, at: [0.4477 - (h - 0.083), s.at[1], 0], rw: s.rw + 0.004, rv: s.rv + 0.004 }; };
    out.push(loft(b(`boot-strap-${i}`, "shank-l", 0.6, "detail"), [k(y + 0.021), k(y), k(y - 0.021)]));
  }
  out.push(loft(b("boot-strap-3", "foot-l", 0.6, "detail"), [footSt("foot", 0.03, 0.011, 0.147, 0.0655, 2.8), footSt("foot", 0.05, 0.011, 0.137, 0.0655, 2.8), footSt("foot", 0.07, 0.011, 0.123, 0.0655, 2.8)]));
  out.push(loft(b("boot-strap-4", "foot-l", 0.6, "detail"), [footSt("foot", 0.1, 0.011, 0.105, 0.0655, 2.8), footSt("foot", 0.12, 0.011, 0.097, 0.0655, 2.8), footSt("foot", 0.138, 0.011, 0.09, 0.0655, 2.8)]));
  return out;
}

/* ---------------------------------------------------------------- body */

export function body50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, nearOf));
  };
  const right = (solids: Json[]): Json[] => solids.map((s) => mirrored(s, toRight, nearOf));
  const hoodie = (id: string, bone = "torso", tone: number = TONE.hoodie, role = "body"): C => c(id, bone, "fleece-hoodie", tone, role);
  const skin = (id: string, bone: string, tone: number, role = "body"): C => c(id, bone, "skin-light", tone, role);

  out.push(loft(c("hip", "pelvis", "polyester-short", TONE.shorts), dense(rowsAt(SEAT, "pelvis"), 3)));
  out.push(loft(hoodie("torso"), dense(rowsAt(HOODIE, "torso"), 4)));
  const hood: [number, number, number, number][] = [[1.438, -0.118, -0.082, 0.062], [1.47, -0.14, -0.06, 0.112], [1.52, -0.136, -0.03, 0.134], [1.565, -0.112, 0.004, 0.128], [1.598, -0.082, 0.02, 0.104], [1.616, -0.056, 0.016, 0.066]];
  out.push(loft(hoodie("hood", "torso", 0.38), dense(hood.map(([y, b, f, rw]) => H("torso", y, b, f, rw, 2.3)), 3)));
  out.push(loft(hoodie("collar", "neck", 0.38), [st(-0.05, 0.012, 0, 0.092, 0.088, 2.1, -18), st(-0.02, 0.014, 0, 0.083, 0.081, 2.1, -18), st(0.008, 0.016, 0, 0.073, 0.073, 2.1, -18), st(0.024, 0.017, 0, 0.066, 0.067, 2.1, -18)]));
  const pocket: [number, number][] = [[0.985, 0.142], [1.03, 0.14], [1.09, 0.122], [1.14, 0.096]];
  out.push(loft(hoodie("pocket", "torso", 0.38, "detail"), pocket.map(([y, rw]) => H("torso", y, surfaceX(HOODIE, y, 0, 1) - 0.12, surfaceX(HOODIE, y, 0, 1) + 0.007, rw, 2.6))));
  for (const side of [1, -1]) {
    const z = side * 0.026;
    const path = [1.545, 1.5, 1.45, 1.4, 1.35].map((y, i) => L("torso", [surfaceX(HOODIE, y, z, 1) + 0.0035, y, z * (1 + 0.1 * i)]));
    out.push(tube(hoodie(`drawstring-${side > 0 ? "l" : "r"}`, "torso", 0.7, "detail"), path, [0.003], { slender: true }));
  }

  const leg: Json[] = [];
  leg.push(loft(c("skirt-l", "thigh-l", "polyester-short", TONE.shortsFar), dense(SHORTS_LEG, 3)));
  leg.push(loft(skin("thigh-g-l", "thigh-l", TONE.legFar), dome(dense(THIGH, 3), 1, 0.012)));
  leg.push(ellipsoid(skin("knee-l", "thigh-l", TONE.legFar), [0.027, 0.0125, 0.024], [0.438, 0.055, 0]));
  both(leg);
  const shankR = right([loft(skin("shank-g-l", "shank-l", TONE.legFar), dome(dense(SHANK, 3), -1, 0.045))]);
  out.push(loft(skin("shank-g-l", "shank-l", TONE.legFar), dome(dense([...SHANK.slice(0, 3), st(0.125, -0.009, -0.003, 0.056, 0.062, 2)], 3), -1, 0.045)));
  out.push(...shankR);
  out.push(loft(c("sock-r", "shank-r", "cotton-sock", TONE.sock), [st(0.372, -0.004, 0, 0.0355, 0.0425), st(0.4, -0.003, 0, 0.0355, 0.042), st(0.43, -0.002, 0, 0.0385, 0.044, 2.1), st(0.452, 0.002, 0, 0.039, 0.0455, 2.2)]));
  out.push(...sneaker());
  out.push(...boot());

  const arm: Json[] = [];
  arm.push(loft(hoodie("sleeve-up-l", "arm-l", TONE.hoodieFar), dome(dome(dense(SLEEVE, 3), 1, 0.012), -1, 0.03, 3)));
  arm.push(loft(hoodie("sleeve-fore-l", "forearm-l", TONE.hoodieFar), dome(dense(FORE, 3), -1, 0.04)));
  arm.push(loft(hoodie("cuff-s-l", "forearm-l", 0.3), CUFF));
  arm.push(loft(skin("wrist-l", "forearm-l", TONE.handFar), WRIST));
  arm.push(...hand());
  arm.push(...crutch());
  both(arm);
  for (const s of out) if (s.id === "crutch-cuff-r") s.transform.rotation = aboutU(CUFF_TURN);

  out.push(loft(skin("neck", "neck", 0.78), dense(NECK, 2)));
  out.push(ellipsoid(skin("larynx", "neck", 0.78), [0.014, 0.01, 0.012], [0.052, -0.056, 0]));
  out.push(loft(skin("head", "head", TONE.face), dome(dense(HEAD, 3), -1, 0.005, 2)));
  out.push(tube(skin("nose", "head", TONE.face), [[0.047, -0.095, 0], [0.03, -0.102, 0], [0.013, -0.11, 0], [0.001, -0.1155, 0]], [0.0068, 0.008, 0.0105, 0.0122]));
  out.push(ellipsoid(skin("nostrils", "head", TONE.face), [0.008, 0.011, 0.0165], [-0.005, -0.108, 0]));
  out.push(ellipsoid(skin("brow", "head", TONE.face), [0.008, 0.01, 0.05], [0.056, -0.091, 0]));
  out.push(ellipsoid(skin("lips", "head", 0.66), [0.0068, 0.007, 0.024], [-0.034, -0.0905, 0]));
  out.push(ellipsoid(skin("chin", "head", TONE.face), [0.016, 0.0145, 0.022], [-0.064, -0.073, 0]));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`cheek-${n}`, "head", TONE.face), [0.02, 0.009, 0.016], [0.012, -0.063, side * 0.05]));
    out.push(ellipsoid(skin(n === "r" ? "ear" : "ear-l", "head", 0.78), [0.031, 0.017, 0.0068], [0.02, 0.014, side * 0.0775], compose(aboutW(-10), aboutU(side * 22))));
    out.push(ellipsoid(skin(`eye-${n}`, "head", 0.2, "detail"), [0.005, 0.0045, 0.012], [0.039, -0.0905, side * 0.032]));
    out.push(ellipsoid(c(`eyebrow-${n}`, "head", "hair-dark", TONE.hair, "detail"), [0.004, 0.0045, 0.017], [0.058, -0.0965, side * 0.033], aboutU(side * -14)));
  }
  out.push(loft(c("hair", "head", "hair-dark", TONE.hair), dome(dome(dense(HAIR, 3), -1, 0.008, 2), 1, 0.003, 2)));
  return out;
}

/* ---------------------------------------------------------------- thermal */

const LAYERS: Record<string, readonly (readonly [string, number])[]> = {
  hoodie: [["hoodie-tshirt", 0.005], ["hoodie-air", 0.006]],
  shorts: [["shorts-air", 0.001]],
  boot: [["boot-sock", 0.02], ["boot-air", 0.022]],
  sneaker: [["sneaker-sock", 0.002], ["sneaker-air", 0.004]],
};
export const OUTER: Record<string, string> = { hoodie: "hoodie-sweat", shorts: "shorts-poly", boot: "boot-shell", sneaker: "sneaker-upper", hair: "hair-pelt" };
/** 🧅 The shells of a garment loft under its outer layer: the same sections moved inwards by the thickness of the layers outside. */
export function shells(solid: Json, region: string): Json[] {
  return LAYERS[region]!.map(([layer, t], i) => ({
    ...solid,
    id: `th-${solid.id}-${layer}`,
    tone: Math.max(0.05, Math.round((solid.tone - 0.08 * (i + 1)) * 100) / 100),
    sections: solid.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) })),
    region,
    layer,
  }));
}
const sides = (ids: readonly string[]): string[] => ids.flatMap((id) => [id, toRight(id)]);
/** 🌬️ The three outlets: under the nostrils, on the lips, on the crown. */
export function outlets(feature = 0.0075): Json[] {
  const skin = { bone: "head", material: "skin-light", tone: TONE.face };
  const h = feature / 2 + 0.0001;
  return [
    { ...ellipsoid({ id: "th-breath-nose", ...skin }, [h, h, Math.max(0.0095, h)], [-0.0135, -0.107, 0]), region: "face-skin", tags: ["emitter", "breath-nose"] },
    { ...ellipsoid({ id: "th-breath-mouth", ...skin }, [h, h, Math.max(0.0062, h)], [-0.034, -0.0965, 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] },
    { ...ellipsoid({ id: "th-plume", bone: "head", material: "hair-dark", tone: 0.5 }, [h, Math.max(0.036, h), Math.max(0.034, h)], [0.1605 - (h - 0.004) * 0.6, 0.004, 0]), region: "hoodie", tags: ["emitter", "plume"] },
  ];
}

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json => { const s = body.find((b) => b.id === id); if (!s) throw new Error(`no body solid ${id}`); return s; };
  const layered = (id: string, region: string): void => { out.push(thOf(id, region, OUTER[region])); out.push(...shells(by(id), region)); };
  const ref = (ids: readonly string[], region: string, layer?: string): void => { for (const id of ids) { by(id); out.push(thOf(id, region, layer)); } };
  for (const id of ["torso", ...sides(["sleeve-up-l", "sleeve-fore-l"])]) layered(id, "hoodie");
  ref(["hood", "collar", "pocket", "drawstring-l", "drawstring-r", ...sides(["cuff-s-l"])], "hoodie", "hoodie-sweat");
  for (const id of ["hip", ...sides(["skirt-l"])]) layered(id, "shorts");
  ref(sides(["thigh-g-l", "knee-l", "shank-g-l"]), "legs-skin");
  for (const id of ["boot-shaft-l", "shoe-rear-l", "shoe-front-l"]) layered(id, "boot");
  ref(["sole-l", "sole-front-l", "boot-strut-o-l", "boot-strut-i-l", "boot-strap-0", "boot-strap-1", "boot-strap-2", "boot-strap-3", "boot-strap-4"], "boot", "boot-shell");
  for (const id of ["shoe-rear-r", "shoe-front-r"]) layered(id, "sneaker");
  ref(["sole-r", "sole-front-r", "shoe-tongue-r", "lace-r0", "lace-r1", "lace-r2", "lace-r3"], "sneaker", "sneaker-upper");
  ref(["sock-r"], "sneaker", "sneaker-sock");
  ref(["hair", "eyebrow-l", "eyebrow-r"], "hair", "hair-pelt");
  out.push({ ...loft({ id: "th-head", bone: "head", material: "skin-light", tone: TONE.face }, dome(dense(HEAD.slice(0, 7), 3), -1, 0.005, 2), [true, false]), region: "face-skin", sectionFaces: ["+v"] });
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin-light", tone: 0.78 }, dense(NECK.slice(1), 2), [false, false]), region: "face-skin" });
  ref(["larynx", "nose", "nostrils", "brow", "lips", "chin", "cheek-l", "cheek-r", "ear", "ear-l", "eye-l", "eye-r"], "face-skin");
  ref(body.filter((b) => b.material === "skin-light" && /^hand-/.test(b.bone)).map((b) => b.id), "hands");
  ref(sides(["wrist-l"]), "hands");
  ref(sides(["crutch-tube-l", "crutch-lower-l", "crutch-clip-l", "crutch-joint-l", "crutch-bracket-l", ...[0, 1, 2, 3, 4].map((i) => `crutch-hole-${i}-l`)]), "crutch-frame");
  ref(sides(["crutch-grip-l"]), "crutch-grips");
  ref(sides(["crutch-tip-l"]), "crutch-tips");
  ref(sides(["crutch-cuff-l", "crutch-cuff-mount-l"]), "crutch-cuffs");
  out.push(...outlets());
  return out;
}

/* ---------------------------------------------------------------- structure */

export const BONE = { material: "bone", tone: 0.62 };
export const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
/** ⚖️ A ball at the centre of mass of a bone, sized by the cube root of its mass. */
export function massBall(member: string, floor = 0): Json {
  const k = kgOf(member);
  const r = Math.max(floor, 0.0125 * Math.cbrt(k.kg));
  return part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], member.startsWith("crutch-") ? [k.comX * SHAFT[0], k.comY, (member.endsWith("-l") ? 1 : -1) * k.comX * SHAFT[2]] : [k.comX, k.comY, 0]), member, "mass");
}
export const jointBall = (member: string, at: V3, r: number): Json => part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint");
/** 🦴 The spine as points [x from the middle line, y, radius]: sacral promontory, lumbar lordosis, thoracic kyphosis, the cervico-thoracic junction. */
export const SPINE: readonly [number, number, number][] = [[-0.044, 1.048, 0.022], [-0.03, 1.09, 0.021], [-0.022, 1.14, 0.02], [-0.027, 1.2, 0.019], [-0.041, 1.27, 0.018], [-0.056, 1.35, 0.017], [-0.062, 1.42, 0.016], [-0.053, 1.49, 0.0145], [-0.036, 1.54, 0.0135], [-0.022, 1.566, 0.013]];
export const CERVICAL: readonly [number, number, number][] = [[0.039, 1.566, 0.013], [0.049, 1.6, 0.012], [0.057, 1.634, 0.0115], [0.062, 1.664, 0.011]];
/** 🫁 The ribs as hoops [y of the centre, half-depth, half-width], sloping 24 degrees down to the front. */
export const RIBS: readonly [number, number, number][] = [[1.535, 0.045, 0.066], [1.505, 0.062, 0.095], [1.475, 0.076, 0.118], [1.44, 0.088, 0.135], [1.405, 0.096, 0.146], [1.37, 0.102, 0.153], [1.335, 0.105, 0.157], [1.3, 0.104, 0.157], [1.265, 0.1, 0.152], [1.235, 0.094, 0.144]];

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const b = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, ...BONE });
  const T = (x: number, y: number, z = 0): V3 => L("torso", [mid(y) + x, y, z]);
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  m(tube(b("spine", "torso"), SPINE.map(([x, y]) => T(x, y)), SPINE.map((q) => q[2])), "torso");
  const lean = 90 - boneDeg("torso");
  const tilt = 24 - lean;
  RIBS.forEach(([y, rd, rw], i) => {
    const ring: [number, number][] = Array.from({ length: 9 }, (_, k) => [0.006 * Math.cos((k * Math.PI) / 4), rd + 0.006 * Math.sin((k * Math.PI) / 4)] as [number, number]);
    m(revolve(b(`rib-${i + 1}`, "torso"), ring, [0, 0, 0], [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000, transform: { translation: T(-0.008, y).map((x) => Math.round(x * 1e5) / 1e5), rotation: aboutW(-tilt) } }), "torso");
  });
  m(loft(b("sternum", "torso"), [H("torso", 1.292, mid(1.292) + 0.086, mid(1.292) + 0.098, 0.012), H("torso", 1.36, mid(1.36) + 0.094, mid(1.36) + 0.106, 0.018), H("torso", 1.44, mid(1.44) + 0.084, mid(1.44) + 0.096, 0.02), H("torso", 1.5, mid(1.5) + 0.064, mid(1.5) + 0.077, 0.03), H("torso", 1.52, mid(1.52) + 0.056, mid(1.52) + 0.068, 0.028)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule(b(`clavicle-${n}`, "torso"), T(0.058, 1.516, side * 0.022), T(-0.012, 1.534, side * 0.188), 0.0075, 0.007), "torso");
    m(ellipsoid(b(`scapula-${n}`, "torso"), [0.074, 0.007, 0.05], T(-0.108, 1.43, side * 0.112), compose(aboutW(4), aboutU(side * 28))), "torso");
    m(capsule(b(`scapula-spine-${n}`, "torso"), T(-0.112, 1.482, side * 0.09), T(-0.02, 1.534, side * 0.196), 0.007, 0.0085), "torso");
    m(ellipsoid(b(`ilium-${n}`, "pelvis"), [0.06, 0.066, 0.012], P(-0.024, 1.018, side * 0.108), compose(aboutU(side * 30), aboutV(side * -20))), "pelvis");
    m(capsule(b(`ilium-body-${n}`, "pelvis"), P(-0.062, 1.012, side * 0.034), P(0.002, 0.96, side * 0.086), 0.017, 0.019), "pelvis");
    m(capsule(b(`pubis-${n}`, "pelvis"), P(0.012, 0.957, side * 0.084), P(0.062, 0.93, side * 0.012), 0.012, 0.011), "pelvis");
    m(capsule(b(`ischium-${n}`, "pelvis"), P(-0.012, 0.955, side * 0.082), P(-0.046, 0.878, side * 0.06), 0.013, 0.015), "pelvis");
    m(capsule(b(`mandible-ramus-${n}`, "head"), [0.004, -0.004, side * 0.049], [-0.044, -0.01, side * 0.046], 0.0075, 0.008), "head");
    m(capsule(b(`mandible-body-${n}`, "head"), [-0.044, -0.01, side * 0.046], [-0.068, -0.07, side * 0.008], 0.008, 0.009), "head");
  }
  m(tube(b("sacrum", "pelvis"), [P(-0.044, 1.048), P(-0.064, 1.004), P(-0.076, 0.962), P(-0.07, 0.932)], [0.028, 0.026, 0.018, 0.009]), "pelvis");
  m(ellipsoid(b("cranium", "head"), [0.066, 0.09, 0.0685], [0.086, 0.01, 0]), "head");
  m(ellipsoid(b("face-bones", "head"), [0.044, 0.038, 0.052], [0.012, -0.052, 0]), "head");
  m(tube(b("cervical-spine", "neck"), CERVICAL.map(([x, y]) => L("neck", [x, y, 0])), CERVICAL.map((q) => q[2])), "neck");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule(b("femur-neck-l", "thigh-l"), [0, 0, 0], [0.036, -0.004, 0.058], 0.017, 0.018), "thigh-l");
  lm(ellipsoid(b("femur-trochanter-l", "thigh-l"), [0.024, 0.019, 0.015], [0.03, -0.006, 0.066]), "thigh-l");
  lm(tube(b("femur-l", "thigh-l"), [[0.046, -0.004, 0.062], [0.13, 0.008, 0.046], [0.23, 0.014, 0.028], [0.33, 0.01, 0.012], [0.425, 0.002, 0.002]], [0.016, 0.0148, 0.014, 0.0148, 0.018]), "thigh-l");
  lm(ellipsoid(b("femur-condyles-l", "thigh-l"), [0.022, 0.032, 0.042], [0.438, -0.004, 0]), "thigh-l");
  lm(ellipsoid(b("patella-l", "thigh-l"), [0.022, 0.01, 0.021], [0.436, 0.041, 0]), "thigh-l");
  lm(ellipsoid(b("tibia-plateau-l", "shank-l"), [0.016, 0.028, 0.04], [0.018, -0.002, 0]), "shank-l");
  lm(loft(b("tibia-l", "shank-l"), [st(0.026, 0.003, -0.003, 0.022, 0.02, 2), st(0.065, 0.009, -0.004, 0.017, 0.0175, 1.8), st(0.15, 0.012, -0.004, 0.013, 0.015, 1.7), st(0.3, 0.008, -0.004, 0.012, 0.013, 1.8), st(0.395, 0.004, -0.003, 0.013, 0.013, 2), st(0.436, 0.002, -0.003, 0.018, 0.015, 2)]), "shank-l");
  lm(ellipsoid(b("malleolus-l", "shank-l"), [0.015, 0.012, 0.0095], [0.44, 0, -0.026]), "shank-l");
  lm(capsule(b("fibula-l", "shank-l"), [0.036, -0.02, 0.034], [0.444, -0.012, 0.027], 0.0065, 0.0075), "shank-l");
  lm(capsule(b("calcaneus-l", "foot-l"), footAt("foot", -0.052, 0.062), footAt("foot", 0.0, 0.068), 0.02, 0.018), "foot-l");
  lm(ellipsoid(b("talus-l", "foot-l"), [0.026, 0.015, 0.02], footAt("foot", 0.012, 0.079), aboutW(29)), "foot-l");
  lm(loft(b("metatarsals-l", "foot-l"), [footSt("foot", 0.03, 0.05, 0.082, 0.026, 2.2), footSt("foot", 0.075, 0.044, 0.072, 0.033, 2.2), footSt("foot", 0.12, 0.036, 0.056, 0.04, 2.2), footSt("foot", 0.158, 0.03, 0.046, 0.044, 2.2)]), "foot-l");
  lm(loft(b("phalanges-l", "toe-l"), [footSt("toe", 0.146, 0.03, 0.046, 0.044, 2.2), footSt("toe", 0.185, 0.03, 0.042, 0.04, 2.2), footSt("toe", 0.209, 0.03, 0.04, 0.03, 2.2)]), "toe-l");
  lm(ellipsoid(b("humerus-head-l", "arm-l"), [0.023, 0.023, 0.023], [0, 0, -0.012]), "arm-l");
  lm(tube(b("humerus-l", "arm-l"), ([[0.014, 0, -0.01], [0.12, 0.004, -0.003], [0.24, 0.003, 0], [0.326, 0, 0]] as V3[]).map((q) => leanAt(q, ARM_LEAN)), [0.0122, 0.011, 0.0105, 0.0125]), "arm-l");
  lm(ellipsoid(b("humerus-condyles-l", "arm-l"), [0.012, 0.014, 0.032], leanAt([0.336, 0, 0], ARM_LEAN)), "arm-l");
  lm(capsule(b("ulna-l", "forearm-l"), [-0.014, -0.018, 0], leanAt([0.262, -0.011, 0.006], FORE_LEAN), 0.0085, 0.0052), "forearm-l");
  lm(capsule(b("radius-l", "forearm-l"), [0.012, 0.014, 0], leanAt([0.264, 0.008, -0.004], FORE_LEAN), 0.006, 0.009), "forearm-l");
  lm(ellipsoid(b("carpals-l", "hand-l"), [0.016, 0.011, 0.02], grip(-0.03, 0.036, 0.004), aboutW(30)), "hand-l");
  for (const ray of body.filter((s) => s.bone === "hand-l" && s.kind === "capsule")) lm(capsule(b(ray.id.replace("hand-l-", "bones-l-"), "hand-l"), ray.p0, ray.p1, 0.0046, 0.0042), "hand-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  for (const id of sides(["crutch-tube-l", "crutch-lower-l", "crutch-clip-l", "crutch-joint-l", "crutch-bracket-l", "crutch-grip-l", "crutch-tip-l"])) out.push({ id: `member-${id}`, of: id, member: id.endsWith("-l") ? "crutch-l" : "crutch-r", part: "member" });

  out.push(jointBall("torso", T(-0.036, 1.066), 0.017), jointBall("neck", L("neck", [0.039, 1.566, 0]), 0.0135), jointBall("head", [0.004, -0.006, 0], 0.0125));
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.0245), jointBall(`shank-${side}`, [0, 0, 0], 0.019), jointBall(`foot-${side}`, [0, 0, 0], 0.016), jointBall(`toe-${side}`, [-0.004, 0.036, 0], 0.01));
    out.push(jointBall(`arm-${side}`, [0, 0, k * -0.012], 0.0235), jointBall(`forearm-${side}`, [0, 0, 0], 0.0155), jointBall(`hand-${side}`, [0, 0, 0], 0.012), jointBall(`crutch-${side}`, [0, 0, 0], 0.0125));
  }
  for (const id of ["pelvis", "torso", "head", ...sides(["thigh-l", "shank-l", "foot-l", "arm-l", "forearm-l", "hand-l", "crutch-l"])]) out.push(massBall(id));
  return out;
}
export const DECISIONS_50: Json[] = [
  { en: "The man is modelled on the corrected rig (C7 at 0.86 of the 1.82 m stature, hip joint 0.95 m above the sole, knee 0.53 m, shoulder joint 1.49 m) with real cross-sections: skull and jaw with brow, nose, lips, chin, ears and eyes, a neck with larynx, a hooded sweatshirt over ribcage and shoulder mass with kangaroo pocket, collar and drawstrings, loose sports shorts over the gluteal mass, thighs, knees and calves shaped by their muscles.", de: "Der Mann ist auf dem korrigierten Skelett modelliert (C7 bei 0,86 der Körpergröße von 1,82 m, Hüftgelenk 0,95 m über der Sohle, Knie 0,53 m, Schultergelenk 1,49 m), mit echten Querschnitten: Schädel und Kiefer mit Brauen, Nase, Lippen, Kinn, Ohren und Augen, ein Hals mit Kehlkopf, ein Kapuzenpullover über Brustkorb und Schultermasse mit Bauchtasche, Kragen und Kordeln, weite Sporthosen über dem Gesäß, Ober- und Unterschenkel und Knie in ihrer Muskelform." },
  { en: "Both hands grip the handles with four fingers of three phalanges and a thumb; the left foot is off-loaded in a walker boot with sole, shell, two struts and five straps, the right foot stands in a laced sneaker.", de: "Beide Hände umgreifen die Handgriffe mit vier dreigliedrigen Fingern und dem Daumen; der linke Fuß ist in einer Orthese mit Sohle, Schale, zwei Schienen und fünf Riemen entlastet, der rechte Fuß steht in einem geschnürten Turnschuh." },
  { en: "Each forearm crutch is built as the real product: cuff arm and shaft tube of 22 mm aluminium, bracket, telescopic lower tube with its spring clip and adjustment holes, rubber handle, ferrule and an open polymer cuff below the elbow, rigid in the gripping hand.", de: "Jede Unterarmgehstütze ist wie das reale Produkt gebaut: Manschettenarm und Schaftrohr aus 22-mm-Aluminium, Halter, ausziehbares Unterrohr mit Federclip und Verstelllöchern, Gummigriff, Gummipuffer und eine offene Kunststoffmanschette unter dem Ellbogen, starr in der greifenden Hand." },
  { en: "Thermal: every region in space; the sweatshirt, shorts, boot and sneaker as nested shells of air gap, inner layer and outer fabric, face, hands and legs as skin, the crutch frame, grips, ferrules and cuffs as their own surfaces, nostril and mouth breath and the plume above the crown as outlet patches on the surface.", de: "Thermik: jede Region im Raum; Pullover, Hose, Orthese und Turnschuh als geschachtelte Schalen aus Luftspalt, innerer Lage und Außenstoff, Gesicht, Hände und Beine als Haut, Krückenrahmen, Griffe, Puffer und Manschetten als eigene Flächen, Atem aus Nase und Mund und die Wärmefahne über dem Scheitel als Auslassflächen auf der Oberfläche." },
  { en: "Structure: the anatomical skeleton (cranium, mandible, curved spine, twelve rib pairs as ten hoops with sternum, clavicles, scapulae, pelvis of ilium, ischium, pubis and sacrum, long bones with their heads and condyles, carpals, finger and foot bones) and both crutch frames as members; joints at the rig joints, masses at the segment centres after de Leva.", de: "Tragwerk: das anatomische Skelett (Hirnschädel, Unterkiefer, gekrümmte Wirbelsäule, Rippen als zehn Bögen mit Brustbein, Schlüsselbeine, Schulterblätter, Becken aus Darm-, Sitz- und Schambein und Kreuzbein, Röhrenknochen mit Köpfen und Gelenkrollen, Handwurzel-, Finger- und Fußknochen) und beide Krückengestelle als Stäbe; Gelenke an den Gelenken des Skeletts, Massen in den Segmentschwerpunkten nach de Leva." },
];
