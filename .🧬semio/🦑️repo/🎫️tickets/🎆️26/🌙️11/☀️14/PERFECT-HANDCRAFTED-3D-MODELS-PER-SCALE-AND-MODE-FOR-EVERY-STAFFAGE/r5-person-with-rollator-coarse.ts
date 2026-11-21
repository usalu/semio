/** 🪆 The 1:100, 1:200 and 1:500 models of `humans/person-with-rollator`, each stated anew for its scale: fewer and plainer sections for the coat, limbs, head and beret, merged shoes and hands, the rollator reduced to the runs of its side frames, its seat, basket, bag and wheels; the thermal lists by reference with the face as an own skin, the structure as a skeleton that loses detail with the scale and the tube frame on its wheels.
 *
 * @see ./r5-person-with-rollator-s50.ts
 * @see ./r3-humans-person-with-rollator.md */
import type { Json } from "./volumes-lib.ts";
import { H, L, X, aboutU, aboutV, aboutW, box, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, part, revolve, st, thOf, toRight, tube, type Common, type St, type V3 } from "./r5-person-with-rollator-kit.ts";
import { ANKLE_X, BERET_TILT, GRIP, K_SLEEVE, KNUCKLE, TONE, WRAP, around, cross, mid, near, palmFrame, quatOf, sub, unit } from "./r5-person-with-rollator-figure.ts";
import { G, TONE_AID, frontU, handlePath, rearU, strapSide } from "./r5-person-with-rollator-aid.ts";
import { AID_MEMBER, AID_REGION, BONE, kgOf, supports } from "./r5-person-with-rollator-s50.ts";

type Scale = 100 | 200 | 500;
type Slice = readonly [number, number, number, number, number];
const ring = (u: number, rv: number, k = K_SLEEVE): St => st(u, 0, 0, rv * k, rv);

/** 🧥 The coat on the trunk per scale: horizontal slices [y, back, front, half-width, exponent]. */
const COAT: Record<Scale, readonly Slice[]> = {
  100: [[0.83, -0.07, 0.15, 0.165, 2.4], [0.89, -0.106, 0.188, 0.19, 2.5], [0.97, -0.078, 0.208, 0.162, 2.45], [1.07, -0.052, 0.23, 0.154, 2.4], [1.15, -0.038, 0.237, 0.158, 2.4], [1.225, -0.016, 0.22, 0.159, 2.35], [1.277, 0.014, 0.196, 0.143, 2.3], [1.305, 0.042, 0.176, 0.108, 2.2], [1.33, 0.07, 0.152, 0.066, 2]],
  200: [[0.84, -0.09, 0.165, 0.18, 2.4], [0.9, -0.106, 0.188, 0.19, 2.5], [1.05, -0.056, 0.226, 0.155, 2.4], [1.17, -0.032, 0.234, 0.159, 2.4], [1.27, 0.008, 0.2, 0.147, 2.3], [1.33, 0.07, 0.152, 0.066, 2]],
  500: [[0.85, -0.095, 0.17, 0.185, 2.4], [0.95, -0.085, 0.2, 0.178, 2.45], [1.15, -0.036, 0.236, 0.158, 2.4], [1.27, 0.008, 0.2, 0.147, 2.3], [1.33, 0.07, 0.152, 0.07, 2]],
};
const HIP: Record<Scale, readonly Slice[]> = {
  100: [[0.73, -0.07, 0.11, 0.15, 2.5], [0.8, -0.118, 0.156, 0.19, 2.6], [0.86, -0.118, 0.172, 0.194, 2.55], [0.96, -0.076, 0.188, 0.168, 2.45]],
  200: [[0.74, -0.08, 0.12, 0.16, 2.5], [0.82, -0.122, 0.162, 0.195, 2.6], [0.95, -0.08, 0.186, 0.17, 2.45]],
  500: [[0.74, -0.08, 0.12, 0.16, 2.5], [0.83, -0.122, 0.164, 0.196, 2.6], [0.94, -0.086, 0.184, 0.176, 2.45]],
};
const SKIRT: Record<Scale, St[]> = {
  100: [st(-0.01, 0.012, -0.018, 0.11, 0.128, 3.2), st(0.05, 0.008, -0.018, 0.12, 0.136, 3.2), st(0.2, 0.005, -0.018, 0.126, 0.137, 3.2), st(0.352, 0.004, -0.018, 0.1345, 0.141, 2.6)],
  200: [st(0.0, 0.01, -0.018, 0.112, 0.13, 3.2), st(0.15, 0.006, -0.018, 0.124, 0.137, 3.2), st(0.352, 0.004, -0.018, 0.1345, 0.141, 2.6)],
  500: [st(0.0, 0.01, -0.018, 0.112, 0.13, 3.2), st(0.352, 0.004, -0.018, 0.1345, 0.141, 2.6)],
};
const THIGH: Record<Scale, St[]> = {
  100: [st(0.02, -0.004, 0, 0.078, 0.09, 2.1), st(0.2, 0.001, 0, 0.071, 0.08, 2), st(0.3871, 0.004, 0, 0.0585, 0.065, 1.9)],
  200: [st(0.02, 0, 0, 0.078, 0.088), st(0.3871, 0.004, 0, 0.0585, 0.065)],
  500: [st(0.02, 0, 0, 0.078, 0.088), st(0.3871, 0.004, 0, 0.059, 0.065)],
};
const SHANK: Record<Scale, St[]> = {
  100: [st(0.0, 0.004, 0, 0.0575, 0.064, 1.9), st(0.1, -0.003, 0, 0.055, 0.062, 1.9), st(0.24, -0.005, 0, 0.0515, 0.058, 1.9), st(0.342, 0.004, 0, 0.05, 0.059, 2)],
  200: [st(0.0, 0.004, 0, 0.0575, 0.064), st(0.15, -0.004, 0, 0.054, 0.061), st(0.342, 0.004, 0, 0.05, 0.059)],
  500: [st(0.0, 0.004, 0, 0.0575, 0.064), st(0.342, 0.004, 0, 0.05, 0.059)],
};
/** 🥾 The shoe per scale as cross slices [x from the ankle, bottom, top, half-width, exponent], sole and heel merged into the last. */
const SHOE_REAR: Record<Scale, readonly Slice[]> = {
  100: [[-0.058, 0, 0.06, 0.02, 2.4], [-0.03, 0, 0.094, 0.034, 2.5], [0.012, 0, 0.1, 0.038, 2.5], [0.06, 0, 0.078, 0.0435, 2.6], [0.14, 0, 0.047, 0.046, 2.8]],
  200: [[-0.058, 0, 0.07, 0.024, 2.4], [0.0, 0, 0.1, 0.038, 2.5], [0.14, 0, 0.047, 0.046, 2.8]],
  500: [[-0.058, 0, 0.075, 0.03, 2.4], [0.0, 0, 0.1, 0.04, 2.5], [0.12, 0, 0.058, 0.046, 2.8], [0.191, 0, 0.034, 0.03, 2.4]],
};
const SHOE_TOE: Record<100 | 200, readonly Slice[]> = {
  100: [[0.122, 0, 0.051, 0.046, 2.8], [0.16, 0, 0.04, 0.041, 2.6], [0.191, 0, 0.02, 0.014, 2.2]],
  200: [[0.122, 0, 0.051, 0.046, 2.8], [0.191, 0, 0.03, 0.02, 2.4]],
};
const SLEEVE: Record<Scale, St[]> = {
  100: [ring(0.0, 0.039), ring(0.04, 0.052), ring(0.15, 0.054), ring(0.2939, 0.0495)],
  200: [ring(0.0, 0.04), ring(0.08, 0.054), ring(0.2939, 0.0495)],
  500: [ring(0.0, 0.042), ring(0.2939, 0.0495)],
};
const CUFF: Record<Scale, St[]> = {
  100: [ring(0.0, 0.0485), ring(0.1, 0.045), ring(0.198, 0.041)],
  200: [ring(0.0, 0.0485), ring(0.215, 0.04)],
  500: [ring(0.0, 0.0485), ring(0.22, 0.041)],
};
const HEAD: Record<Scale, St[]> = {
  100: [st(-0.027, -0.06, 0, 0.016, 0.014), st(-0.01, -0.042, 0, 0.04, 0.042), st(0.028, -0.013, 0, 0.059, 0.072), st(0.07, 0, 0, 0.068, 0.086), st(0.11, 0.007, 0, 0.0722, 0.091), st(0.15, 0.01, 0, 0.066, 0.08), st(0.178, 0.01, 0, 0.04, 0.045), st(0.1858, 0.01, 0, 0.013, 0.015)],
  200: [st(-0.027, -0.058, 0, 0.02, 0.018), st(0.0, -0.034, 0, 0.047, 0.05), st(0.05, -0.005, 0, 0.065, 0.082), st(0.11, 0.007, 0, 0.0722, 0.091), st(0.165, 0.01, 0, 0.055, 0.064), st(0.1858, 0.01, 0, 0.015, 0.017)],
  500: [st(-0.027, -0.055, 0, 0.025, 0.022), st(0.02, -0.02, 0, 0.056, 0.066), st(0.1, 0.006, 0, 0.072, 0.09), st(0.1858, 0.01, 0, 0.02, 0.022)],
};
const BERET: Record<Scale, readonly [number, number, number, number][]> = {
  100: [[0.108, 0.008, 0.0745, 0.0925], [0.125, 0.016, 0.092, 0.11], [0.147, 0.027, 0.112, 0.127], [0.168, 0.03, 0.095, 0.106], [0.184, 0.029, 0.03, 0.034]],
  200: [[0.108, 0.008, 0.0745, 0.0925], [0.135, 0.022, 0.106, 0.122], [0.165, 0.03, 0.098, 0.11], [0.184, 0.029, 0.03, 0.034]],
  500: [[0.108, 0.008, 0.0745, 0.0925], [0.145, 0.026, 0.11, 0.125], [0.184, 0.029, 0.035, 0.04]],
};
const NECK: Record<Scale, St[]> = {
  100: [st(-0.03, 0.006, 0, 0.058, 0.06), st(0.04, -0.001, 0, 0.052, 0.053), st(0.105, -0.008, 0, 0.05, 0.05)],
  200: [st(-0.03, 0.006, 0, 0.058, 0.06), st(0.105, -0.008, 0, 0.05, 0.05)],
  500: [st(-0.03, 0.006, 0, 0.058, 0.06), st(0.105, -0.008, 0, 0.05, 0.05)],
};
const DENSE: Record<Scale, number> = { 100: 3, 200: 2, 500: 2 };

/** ✊ The fingers of the left hand as one band wrapped over the outer side of the grip (1:100). */
function fingerBand(halfWidth: number, halfThick: number): Json[] {
  const s = 0.034;
  const pts = WRAP.map(([phi, r]) => around(s, phi, r));
  return pts.map((p, i) => {
    const ez = unit(sub(pts[Math.min(i + 1, pts.length - 1)]!, pts[Math.max(i - 1, 0)]!));
    const along = unit(sub(around(s + 0.02, WRAP[i]![0], WRAP[i]![1]), around(s - 0.02, WRAP[i]![0], WRAP[i]![1])));
    const ey = unit(cross(ez, along));
    const ex = cross(ey, ez);
    return { at: p.map((x) => Math.round(x * 1e5) / 1e5), rx: halfWidth, ry: halfThick, exponent: 2.6, rotation: quatOf(ex, ey, ez) };
  });
}

function figure(scale: Scale): Json[] {
  const out: Json[] = [];
  const k = DENSE[scale];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  const loden = (id: string, bone: string, tone: number = TONE.coat): Common => ({ id, bone, material: "loden-wool", tone });
  out.push(loft(loden("coat-hip", "pelvis"), dense(HIP[scale].map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e)), k)));
  out.push(loft(loden("coat", "torso"), dense(COAT[scale].map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e)), k)));
  const limb: Json[] = [];
  const wool = (id: string, bone: string): Common => ({ id, bone, material: "wool-trouser", tone: TONE.trousersFar });
  const shoe = (id: string, bone: string): Common => ({ id, bone, material: "leather-shoe", tone: TONE.shoeFar });
  limb.push(loft(loden("skirt-l", "thigh-l", TONE.coatFar), dome(dense(SKIRT[scale], k), -1, 0.03, 2)));
  limb.push(loft(wool("thigh-l", "thigh-l"), dome(dense(THIGH[scale], k), 1, 0.038, 2)));
  limb.push(loft(wool("shank-l", "shank-l"), dome(dense(SHANK[scale], k), -1, 0.038, 2)));
  limb.push(loft(shoe("shoe-l", "foot-l"), dense(SHOE_REAR[scale].map(([x, b, t, rw, e]) => X("foot-l", ANKLE_X + x, b, t, rw, e)), k)));
  if (scale !== 500) limb.push(loft(shoe("shoe-toe-l", "toe-l"), dense(SHOE_TOE[scale].map(([x, b, t, rw, e]) => X("toe-l", ANKLE_X + x, b, t, rw, e)), k)));
  limb.push(loft(loden("sleeve-l", "arm-l", TONE.coatFar), dome(dome(dense(SLEEVE[scale], k), 1, 0.04, 2), -1, 0.01, 2)));
  limb.push(loft(loden("forearm-l", "forearm-l", TONE.coatFar), dome(dense(CUFF[scale], k), -1, 0.04, 2)));
  const skinL = (id: string, bone: string): Common => ({ id, bone, material: "skin-aged", tone: TONE.handFar });
  if (scale === 100) {
    limb.push(loft(skinL("wrist-l", "forearm-l"), [ring(0.19, 0.026, 0.92), ring(0.24, 0.024, 0.92)]));
    const palm = palmFrame();
    limb.push(ellipsoid(skinL("hand-l", "hand-l"), palm.radii, palm.at, palm.rotation));
    limb.push({ kind: "loft", id: "hand-l-fingers", bone: "hand-l", material: "skin-aged", role: "body", tone: TONE.handFar, sections: fingerBand(0.04, 0.0085), capStart: true, capEnd: true });
    limb.push(capsule(skinL("hand-l-thumb", "hand-l"), [0.014, -0.004, -0.021], around(0.03, 170, 0.0265), 0.0115, 0.009));
  } else {
    const r: V3 = scale === 200 ? [0.046, 0.032, 0.04] : [0.05, 0.0375, 0.045];
    limb.push(ellipsoid(skinL("hand-l", "hand-l"), r, [GRIP.at[0] - 0.006, GRIP.at[1] + 0.014, 0], aboutW(GRIP.slopeDeg)));
  }
  both(limb);

  const skin = (id: string, bone = "head", tone: number = TONE.skin): Common => ({ id, bone, material: "skin-aged", tone });
  out.push(loft(skin("neck", "neck", 0.8), NECK[scale]));
  if (scale === 100) out.push(loft(loden("collar", "neck", 0.33), [st(-0.04, 0.012, 0, 0.09, 0.08, 2.1, 20), st(-0.008, 0.012, 0, 0.08, 0.074, 2.1, 20), st(0.024, 0.012, 0, 0.067, 0.066, 2.1, 20)]));
  out.push(loft(skin("head"), scale === 500 ? dense(HEAD[scale], 2) : dome(dense(HEAD[scale], k), -1, 0.005, 2)));
  if (scale === 100) {
    out.push(ellipsoid(skin("head-nose"), [0.021, 0.0125, 0.009], [0.064, -0.094, 0], aboutW(28)));
    for (const side of [1, -1]) {
      const n = side > 0 ? "l" : "r";
      out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.78), [0.03, 0.0155, 0.0078], [0.068, 0.014, side * 0.0735], compose(aboutW(-8), aboutU(side * 22))));
      out.push(ellipsoid({ id: `hair-temple-${n}`, bone: "head", material: "hair-white", tone: TONE.hair }, [0.03, 0.03, 0.0115], [0.092, 0.03, side * 0.0655]));
    }
  }
  if (scale === 200) out.push(ellipsoid(skin("head-nose"), [0.02, 0.016, 0.015], [0.062, -0.088, 0], aboutW(28)));
  out.push(ellipsoid({ id: "hair-nape", bone: "head", material: "hair-white", tone: TONE.hair }, [0.052, 0.024, 0.066], [0.066, 0.072, 0], aboutW(-7)));
  out.push(loft({ id: "beret", bone: "head", material: "wool-beret", tone: TONE.beret }, dense(BERET[scale].map(([u, v, rw, rv]) => st(u, v, 0, rw, rv, 2, BERET_TILT)), k)));
  return out;
}

/** 🎡 A wheel as tyre and one disc for rim, spokes and hub (1:100), or as one disc with its hub boss (1:200, 1:500). */
function wheel(scale: Scale, bone: string, name: string, at: number, far: boolean, bossTo: number): Json[] {
  const h = scale === 500 ? 0.0375 : G.tyreHalf;
  const w0 = scale === 500 && bone === "wheel-rear" ? Math.sign(at) * (G.rearTrack + G.tyreHalf - h) : at;
  if (scale === 100) {
    const tyre: [number, number][] = [[w0 - h, 0.076], [w0 - h, 0.094], [w0 - h + 0.006, 0.1], [w0 + h - 0.006, 0.1], [w0 + h, 0.094], [w0 + h, 0.076], [w0 - h, 0.076]];
    const disc: [number, number][] = [[w0 - 0.02, 0], [w0 - 0.02, 0.02], [w0 - 0.0085, 0.03], [w0 - 0.0085, 0.078], [w0 + 0.0085, 0.078], [w0 + 0.0085, 0.03], [w0 + 0.02, 0.02], [w0 + 0.02, 0]];
    return [revolve({ id: `tyre-${name}`, bone, material: "pu-tyre", tone: far ? 0.1 : TONE_AID.tyre }, tyre, [0, 0, 0], [0, 0, 1]), revolve({ id: `rim-${name}`, bone, material: "nylon-grey", tone: far ? 0.64 : TONE_AID.rim }, disc, [0, 0, 0], [0, 0, 1])];
  }
  const s = Math.sign(w0);
  const inner = w0 - s * h;
  const profile: [number, number][] = bossTo === 0 ? [[w0 - h, 0], [w0 - h, 0.094], [w0 - h + 0.006, 0.1], [w0 + h - 0.006, 0.1], [w0 + h, 0.094], [w0 + h, 0]] : s > 0 ? [[bossTo, 0], [bossTo, 0.022], [inner, 0.028], [inner, 0.094], [inner + 0.006, 0.1], [w0 + h - 0.006, 0.1], [w0 + h, 0.094], [w0 + h, 0]] : [[w0 - h, 0], [w0 - h, 0.094], [w0 - h + 0.006, 0.1], [inner - 0.006, 0.1], [inner, 0.094], [inner, 0.028], [bossTo, 0.022], [bossTo, 0]];
  return [revolve({ id: `tyre-${name}`, bone, material: "pu-tyre", tone: far ? 0.1 : TONE_AID.tyre }, profile, [0, 0, 0], [0, 0, 1])];
}

function aid(scale: Scale): Json[] {
  const out: Json[] = [];
  const S = G.side;
  const alu = (id: string, tone: number = TONE_AID.tubeFar): Common => ({ id, bone: "frame", material: "alu-red-paint", tone });
  const grey = (id: string, tone: number = TONE_AID.fork): Common => ({ id, bone: "frame", material: "nylon-grey", tone });
  const nylon = (id: string, tone: number): Common => ({ id, bone: "frame", material: "nylon-seat", tone });
  const lit = (t: number): number => (t === TONE_AID.tubeFar ? TONE_AID.tube : t);
  const at = (p: readonly [number, number], w: number): V3 => [p[0], p[1], w];
  const side: Json[] = [];
  const handle = handlePath(scale === 100 ? 5 : scale === 200 ? 4 : 3, scale === 500 ? 0.085 : G.bendR).map((p) => at(p, S));
  if (scale === 100) {
    side.push(tube(alu("main-tube-l"), [at([G.head[0], G.head[1] - 0.004], S), at([frontU(0.5), 0.5], S), at([frontU(0.727), 0.727], S)], [0.014]));
    side.push(tube(alu("handle-post-l", 0.66), handle, [0.011]));
    side.push(tube(alu("seat-rail-l"), [at([rearU(G.railV), G.railV], S), at([0.29, G.railV], S), at([frontU(G.railV), G.railV], S)], [0.011]));
  } else {
    side.push(tube(alu("main-tube-l"), [at([G.head[0], G.head[1] - 0.004], S), at([frontU(0.5), 0.5], S), ...handle.slice(1)], scale === 500 ? [0.021] : [0.014, 0.014, 0.0125, 0.0115, 0.0115]));
  }
  side.push(tube(alu("rear-leg-l"), [at([frontU(0.689) - 0.004, 0.689], S), at([rearU(0.4), 0.4], S), at([rearU(scale === 100 ? 0.118 : 0.1), scale === 100 ? 0.118 : 0.1], S)], [scale === 500 ? 0.021 : 0.0125]));
  if (scale !== 500) side.push(tube(alu("lower-rail-l"), [at([rearU(G.lowV), G.lowV], S), at([0.3, G.lowV], S), at([frontU(G.lowV), G.lowV], S)], [0.011]));
  side.push(capsule({ id: "grip-l", bone: "frame", material: "rubber-grip", tone: TONE_AID.grip }, [G.grip[0] + 0.012, G.gripV, S], [G.grip[1] - 0.014, G.gripV, S], 0.0178, 0.0178));
  side.push(revolve(grey("caster-head-l"), [[0.2, 0], [0.2, 0.0185], [0.264, 0.0185], [0.27, 0.0145], [0.27, 0]], [G.head[0], 0, S], [0, 1, 0]));
  if (scale === 100) {
    const blade = (id: string, w: number): Json => loft(grey(id), ([[G.head[0] - 0.006, 0.206], [0.535, 0.15], [G.wheelbase, 0.097]] as [number, number][]).map(([u, v]): St => ({ at: [u, v, w], rw: 0.0075, rv: 0.014, e: 4, n: 76 })));
    side.push(blade("caster-blade-out-l", S + 0.0265), blade("caster-blade-in-l", S - 0.0265));
    side.push(capsule(grey("axle-rear-l", 0.3), [0, G.wheelR, S - 0.006], [0, G.wheelR, G.rearTrack + 0.01], 0.0075, 0.0075));
    side.push(capsule(grey("axle-front-l", 0.3), [G.wheelbase, G.wheelR, S - 0.032], [G.wheelbase, G.wheelR, S + 0.032], 0.0075, 0.0075));
  }
  if (scale !== 500) {
    const s = G.strap;
    const path = strapSide();
    const thick = scale === 100 ? 0.0075 : 0.009;
    side.push({ kind: "loft", id: "backrest-l", bone: "frame", material: "nylon-seat", role: "body", tone: 0.3, sections: (scale === 100 ? path : [path[0]!, path[2]!, path[5]!, path[8]!]).map((p, i, all) => {
      const a = all[Math.max(0, i - 1)]!;
      const b = all[Math.min(all.length - 1, i + 1)]!;
      const l = Math.hypot(b[0] - a[0], b[1] - a[1]);
      const t: V3 = [(b[0] - a[0]) / l, 0, (b[1] - a[1]) / l];
      const ex: V3 = [0, 1, 0];
      return { at: [p[0], s.v, p[1]], rx: s.h, ry: thick, exponent: 4, rotation: quatOf(ex, cross(t, ex), t) };
    }), capStart: true, capEnd: true });
  }
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight, lit));

  if (scale !== 500) {
    out.push(capsule(alu("cross-lower", TONE_AID.tube), [frontU(G.lowV), G.lowV, -S], [frontU(G.lowV), G.lowV, S], 0.011, 0.011));
    const s = G.strap;
    const thick = scale === 100 ? 0.0075 : 0.009;
    out.push(box(nylon("backrest", 0.3), [2 * thick, 2 * s.h, 2 * (s.side - s.r) + 0.002], [s.uFront, s.v, 0], undefined, 0.004));
  }
  if (scale === 100) out.push(capsule(alu("cross-upper", TONE_AID.tube), [frontU(0.485), 0.485, -S], [frontU(0.485), 0.485, S], 0.011, 0.011));
  const seat = G.seat;
  const thickness = scale === 500 ? 0.075 : scale === 200 ? 0.03 : 0.024;
  out.push(box(nylon("seat", TONE_AID.seat), [seat.u1 - seat.u0, thickness, 2 * seat.half + (scale === 100 ? 0 : 0.04)], [(seat.u0 + seat.u1) / 2, seat.top - thickness / 2, 0], undefined, scale === 100 ? 0.008 : 0));
  const b = G.basket;
  const rect = (h: number, u0: number, u1: number, r: number): [number, number][] => (r === 0 ? [[h, u0], [h, u1], [-h, u1], [-h, u0]] : [[h - r, u0], [h, u0 + r], [h, u1 - r], [h - r, u1], [-h + r, u1], [-h, u1 - r], [-h, u0 + r], [-h + r, u0]]);
  const g = G.bag;
  const tote: Common = { id: "bag", bone: "frame", material: "cotton-tote", tone: TONE_AID.bag };
  if (scale === 500) {
    out.push(extrude(nylon("basket", TONE_AID.basket), "zx", rect(b.half, b.u0, b.u1, 0), b.v0, b.v1));
    out.push(extrude(tote, "xy", [[b.u0, b.v1 - 0.02], [b.u1, b.v1 - 0.02], [b.u1 - 0.012, g.v1 + 0.02], [b.u0 + 0.012, g.v1 + 0.02]], -b.half, b.half));
  } else {
    const r = scale === 100 ? 0.02 : 0;
    out.push(extrude(nylon("basket", TONE_AID.basket), "zx", rect(b.half, b.u0, b.u1, r), b.v0, b.v1, { holes: [rect(b.half - b.wall, b.u0 + b.wall, b.u1 - b.wall, r === 0 ? 0 : 0.017)] }));
    out.push(extrude(nylon("basket-floor", 0.5), "zx", rect(b.half - 0.001, b.u0 + 0.001, b.u1 - 0.001, r), b.v0, b.v0 + (scale === 100 ? 0.015 : 0.03)));
    const outline: [number, number][] = scale === 100 ? [[g.u0 + 0.008, g.v0], [g.u1 - 0.008, g.v0], [g.u1 + 0.004, g.v0 + 0.1], [g.u1 - 0.004, g.v1 - 0.05], [g.u1 - 0.03, g.v1], [g.u0 + 0.03, g.v1], [g.u0 + 0.004, g.v1 - 0.05], [g.u0 - 0.004, g.v0 + 0.1]] : [[g.u0, g.v0 + 0.02], [g.u1, g.v0 + 0.02], [g.u1 - 0.02, g.v1], [g.u0 + 0.02, g.v1]];
    out.push(extrude(tote, "xy", outline, -g.half, g.half));
  }
  const rearBoss = scale === 200 ? S + 0.011 : 0;
  out.push(...wheel(scale, "wheel-rear", "rear-l", G.rearTrack, true, rearBoss), ...wheel(scale, "wheel-rear", "rear-r", -G.rearTrack, false, -rearBoss));
  out.push(...wheel(scale, "wheel-front", "front-l", S, true, 0), ...wheel(scale, "wheel-front", "front-r", -S, false, 0));
  return out;
}

export const body = (scale: Scale): Json[] => [...figure(scale), ...aid(scale)];

/** 🌡️ The thermal list of a coarse scale: every region by reference to its body solids (layers merged into the outer one), the face as an own open skin of the front of the head, outlet patches for breath and plume. */
export function thermal(scale: Scale, b: readonly Json[]): Json[] {
  const out: Json[] = [];
  const regionOf = (s: Json): string | null => {
    const a = AID_REGION(s);
    if (a) return a;
    if (s.material === "loden-wool") return "coat";
    if (s.material === "wool-trouser") return /^shank/.test(s.id) ? "trousers" : null;
    if (s.material === "leather-shoe") return "shoes";
    if (s.material === "hair-white") return "hair";
    if (s.material === "wool-beret") return null;
    if (s.material === "skin-aged") return /^(hand|wrist)/.test(s.id) ? "hands" : /^head-/.test(s.id) ? "face-skin" : null;
    return null;
  };
  for (const s of b) { const region = regionOf(s); if (region) out.push(thOf(s.id, region)); }
  const skin = { bone: "head", material: "skin-aged", tone: TONE.skin };
  const face = (q: St): St => ({ ...q, at: [q.at[0], q.at[1] - q.rv / 2, q.at[2]], rv: q.rv / 2 });
  out.push({ ...loft({ id: "th-head", ...skin }, [...HEAD[scale].filter((q) => q.at[0] < 0.1), st(0.112, 0.008, 0, 0.0722, 0.091, 2, BERET_TILT)].map(face), [true, false]), region: "face-skin" });
  if (scale !== 500) out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin-aged", tone: 0.8 }, [st(0.024, 0.001, 0, 0.0505, 0.0525), st(0.082, -0.006, 0, 0.049, 0.0495)].map(face), [false, false]), region: "face-skin" });
  out.push({ ...loft({ id: "th-beret", bone: "head", material: "wool-beret", tone: TONE.beret }, dense(BERET[scale].map(([u, v, rw, rv]) => st(u, v, 0, rw, rv, 2, BERET_TILT)), DENSE[scale]), [false, true]), region: "beret" });
  const f = scale / 1000;
  const p = Math.max(0.004, (0.15 * f) / 2);
  out.push({ ...ellipsoid({ id: "th-breath-nose", ...skin }, [p, p, Math.max(0.009, p)], [0.0345, -0.0945 + (p - 0.004), 0]), region: "face-skin", tags: ["emitter", "breath-nose"] });
  out.push({ ...ellipsoid({ id: "th-breath-mouth", ...skin }, [p, p, Math.max(0.0062, p)], [0.019, -0.0818 + (p - 0.004), 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] });
  out.push({ ...ellipsoid({ id: "th-plume", bone: "head", material: "wool-beret", tone: 0.5 }, [p, 0.07, 0.065], [0.181 - (p - 0.004), 0.03, 0], aboutW(BERET_TILT)), region: "coat", tags: ["emitter", "plume"] });
  return out;
}

/** 🦴 The structure of a coarse scale: her skeleton with the detail the scale can show, the members of the rollator by reference, joints, supports and masses. */
export function structure(scale: Scale, b: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const r = scale === 100 ? 0.0075 : scale === 200 ? 0.015 : 0.0375;
  const fat = (x: number): number => Math.max(x, r);
  const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  const spine: [number, number, number][] = scale === 100 ? [[-0.022, 0.955, 0.019], [0.002, 1.04, 0.018], [0.016, 1.13, 0.0165], [0.04, 1.215, 0.015], [0.074, 1.28, 0.0135], [0.0975, 1.305, 0.0125]] : scale === 200 ? [[-0.022, 0.955, 0.019], [0.01, 1.09, 0.017], [0.045, 1.225, 0.015], [0.0975, 1.305, 0.015]] : [[-0.01, 0.955, r], [0.03, 1.15, r], [0.0975, 1.3, r]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => fat(q[2]))), "torso");
  const hoops: [number, number, number, number][] = scale === 100 ? [[1.06, 0.06, 0.076, 0.108], [1.115, 0.069, 0.086, 0.118], [1.17, 0.08, 0.088, 0.12], [1.225, 0.097, 0.081, 0.112], [1.275, 0.113, 0.062, 0.088]] : scale === 200 ? [[1.08, 0.063, 0.074, 0.106], [1.17, 0.08, 0.08, 0.112], [1.255, 0.106, 0.064, 0.092]] : [];
  hoops.forEach(([y, x, rd, rw], i) => {
    const drop = scale === 100 ? 0.032 + 0.006 * (hoops.length - 1 - i) : 0.04 + 0.01 * (hoops.length - 1 - i);
    for (const side of [1, -1]) {
      const path: V3[] = [];
      for (let k = 0; k <= 6; k++) {
        const th = (k / 6) * Math.PI;
        path.push(T(x - rd * Math.cos(th), y - (drop * (1 - Math.cos(th))) / 2, side * rw * Math.sin(th)));
      }
      m(tube({ id: `rib-${i}-${side > 0 ? "l" : "r"}`, bone: "torso", ...BONE }, path, [r]), "torso");
    }
  });
  if (scale === 100) {
    m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.135, 0.143, 0.158, 0.015), H("torso", 1.262, 0.164, 0.179, 0.022)]), "torso");
    for (const side of [1, -1]) {
      const n = side > 0 ? "l" : "r";
      m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.164, 1.272, side * 0.02), T(0.098, 1.287, side * 0.15), r, r), "torso");
      m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.068, 0.0078, 0.044], T(0.016, 1.218, side * 0.098), compose(aboutW(6), aboutU(side * 28))), "torso");
      m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.06, 0.068, 0.011], P(-0.014, 0.915, side * 0.1), compose(aboutU(side * 34), aboutV(side * -24))), "pelvis");
      m(capsule({ id: `ischiopubic-${n}`, bone: "pelvis", ...BONE }, P(0.05, 0.838, side * 0.014), P(-0.036, 0.775, side * 0.06), 0.011, 0.012), "pelvis");
    }
    m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.022, 0.955), P(-0.046, 0.912), P(-0.058, 0.86)], [0.027, 0.024, 0.012]), "pelvis");
  } else {
    m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.096, 1.283, -0.15), T(0.096, 1.283, 0.15), r, r), "torso");
    m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(0, 0.835, -0.0846), P(0, 0.835, 0.0846), fat(0.022), fat(0.022)), "pelvis");
    if (scale === 200) m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.022, 0.955), P(-0.05, 0.9), P(-0.03, 0.845)], [0.024, 0.022, 0.018]), "pelvis");
  }
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, scale === 500 ? [0.07, 0.082, 0.066] : [0.066, 0.084, 0.0655], scale === 500 ? [0.095, 0.004, 0] : [0.108, 0.01, 0]), "head");
  if (scale !== 500) m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.04, 0.034, 0.048], [0.042, -0.042, 0]), "head");
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, [[-0.004, 0.004, 0], [0.04, 0.006, 0], [0.078, 0.002, 0]], [fat(0.012)]), "neck");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  if (scale === 100) {
    lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0, 0, 0], [0.034, -0.006, 0.056], [0.115, 0.006, 0.044], [0.287, 0.008, 0.014], [0.37, 0.0, 0.003]], [0.016, 0.017, 0.0128, 0.0128, 0.016]), "thigh-l");
    lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.019, 0.03, 0.0375], [0.381, 0.002, 0]), "thigh-l");
    lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.006, 0, 0], [0.03, 0.004, 0.003], [0.118, 0.009, 0.003], [0.246, 0.007, 0.003], [0.38, 0.002, 0.002]], [0.03, 0.0165, 0.0122, 0.011, 0.014]), "shank-l");
    lm(loft({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [X("foot-l", ANKLE_X - 0.046, 0.016, 0.05, 0.018), X("foot-l", ANKLE_X, 0.03, 0.07, 0.024), X("foot-l", ANKLE_X + 0.07, 0.022, 0.052, 0.033), X("foot-l", ANKLE_X + 0.134, 0.011, 0.03, 0.038)]), "foot-l");
    lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", ANKLE_X + 0.126, 0.011, 0.03, 0.038), X("toe-l", ANKLE_X + 0.182, 0.011, 0.026, 0.026)]), "toe-l");
    lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.002, 0, -0.02], [0.02, 0, -0.014], [0.1, 0.003, -0.006], [0.2, 0.002, -0.002], [0.289, 0, 0]], [0.019, 0.011, 0.0098, 0.0094, 0.0125]), "arm-l");
    lm(loft({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [st(-0.012, -0.002, 0, 0.0085, 0.021), st(0.1, -0.001, 0, 0.0078, 0.017), st(0.229, -0.001, 0, 0.009, 0.02)]), "forearm-l");
    const palm = palmFrame();
    lm(ellipsoid({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [palm.radii[0] * 0.9, palm.radii[1] * 0.85, 0.0078], palm.at, palm.rotation), "hand-l");
  } else {
    const k2 = around(KNUCKLE.s, KNUCKLE.phi, KNUCKLE.r);
    lm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.3871, 0, 0], fat(0.016), fat(0.016)), "thigh-l");
    lm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0, 0, 0], [0.3887, 0, 0], fat(0.015), fat(0.013)), "shank-l");
    lm(capsule({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [0, 0, 0], scale === 200 ? [0.132, 0.014, 0] : [0.11, 0.03, 0], fat(0.015), fat(0.014)), "foot-l");
    lm(capsule({ id: "humerus-l", bone: "arm-l", ...BONE }, [0, 0, 0], [0.2939, 0, 0], fat(0.011), fat(0.011)), "arm-l");
    lm(capsule({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [0, 0, 0], [0.2307, 0, 0], fat(0.011), fat(0.011)), "forearm-l");
    lm(capsule({ id: "hand-bones-l", bone: "hand-l", ...BONE }, [0, 0, 0], mid([0, 0, 0], k2, scale === 200 ? 0.9 : 0.45), fat(0.011), fat(0.011)), "hand-l");
  }
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));
  for (const s of b) { const member = AID_MEMBER(s); if (member) out.push({ id: `member-${s.id}`, of: s.id, member, part: "member" }); }
  out.push(...supports().map((s) => (scale === 100 ? { ...s, size: [0.03, 0.015, s.size[2]], transform: { translation: [s.transform.translation[0], 0.0075, s.transform.translation[2]] } } : { ...s, size: [scale === 200 ? 0.04 : 0.075, scale === 200 ? 0.03 : 0.075, s.size[2]], transform: { translation: [s.transform.translation[0], scale === 200 ? 0.015 : 0.0375, s.transform.translation[2]] } })));

  const joint = (member: string, at: V3, radius: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [radius, radius, radius], at), member, "joint")); };
  const jr = (x: number): number => Math.max(x, r * 1.15);
  joint("torso", T(spine[0]![0], spine[0]![1]), jr(0.0165));
  joint("head", [0, 0.002, 0], jr(0.012));
  if (scale !== 500) joint("neck", [0, 0.002, 0], jr(0.013));
  for (const side of ["l", "r"]) {
    joint(`thigh-${side}`, [0, 0, 0], jr(0.0225));
    joint(`shank-${side}`, [0, 0, 0], jr(0.0175));
    joint(`arm-${side}`, [0, 0, 0], jr(0.0215));
    if (scale !== 500) joint(`foot-${side}`, [0, 0.002, 0], jr(0.0145));
    if (scale !== 500) joint(`forearm-${side}`, [0, 0, 0], jr(0.014));
    if (scale === 100) joint(`hand-${side}`, [0, 0, 0], jr(0.011));
  }
  const masses = scale === 100 ? ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r", "frame"] : scale === 200 ? ["pelvis", "torso", "head", "thigh-l", "thigh-r", "frame"] : ["pelvis", "torso"];
  for (const id of masses) {
    const q = kgOf(id);
    const radius = Math.max(0.0125 * Math.cbrt(q.kg), r * 1.15);
    out.push(part(ellipsoid({ id: `mass-${id}`, bone: id, material: id === "frame" ? "alu-red-paint" : "bone", tone: 0.2, role: "detail" }, [radius, radius, radius], [q.comX, q.comY, 0]), id, "mass"));
  }
  return out;
}
