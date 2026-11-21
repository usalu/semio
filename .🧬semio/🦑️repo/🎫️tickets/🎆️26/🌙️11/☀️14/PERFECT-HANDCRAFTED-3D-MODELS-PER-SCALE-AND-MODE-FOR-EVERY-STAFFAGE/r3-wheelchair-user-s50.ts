/** 🧑‍🦽️ The 1:50 body of the round 3 wheelchair user, every section stated by hand: a man of 1.75 m in a fleece hoodie with the hood down, kangaroo pocket and drawstrings, jeans, low canvas sneakers, half-finger leather gloves and a knitted beanie with a turned-up cuff, sitting in a rigid-frame active wheelchair (side tubes bent from the seat rail into the front hangers, lower tubes, axle plates with axle receivers, folding back posts with push handles, seat and back upholstery with a foam cushion, castor housings with forks, a footrest loop under an aluminium footplate, wheel locks; 24 inch wheels with 37-540 tyres, box rims, 24 spokes laced two-cross to a flanged hub, hand rims on six standoffs outboard of the rims; 135 mm castors).
 * Rows are `[s, cv, cw, rx, ry, e?]`: s along the bone, cv along its normal (pelvis, trunk, neck and head: backwards; arms and shank: forwards; thigh: upwards), cw to the left, rx the lateral and ry the normal semi-axis. Chair parts are stated in world coordinates (x ahead, y up, z left) and carried into the frame of their bone by `local`.
 * @see ./r3-wheelchair-user-ref.ts
 * @see ./r3-humans-wheelchair-user.md */
import { ACROSS_V, Kit, aboutW, dome, type Row, type V3 } from "./r3-wheelchair-user-kit.ts";
import { CHAIR } from "./r3-wheelchair-user-ref.ts";
import { local } from "./r3-wheelchair-user-frame.ts";
import { FOOT_DROP } from "./r3-wheelchair-user-roots.ts";

export const TONE = { skin: 0.27, eye: 0.9, brow: 0.12, lips: 0.18, hoodie: 0.4, hood: 0.35, rib: 0.36, cord: 0.85, denim: 0.6, canvas: 0.12, sole: 0.85, beanie: 0.35, glove: 0.15, alu: 0.5, dark: 0.22, rubber: 0.05, grip: 0.1, foam: 0.2, sling: 0.3, castor: 0.62 } as const;
const rad = (d: number): number => (d * Math.PI) / 180;
const SIDES = [["l", 1], ["r", -1]] as const;
/** 📍️ A world point in the frame of the `frame` bone (the rear axle at (0, 0.305)). */
export const F = (x: number, y: number, z: number): V3 => local("frame", [x, y, z]);
/** 📐️ The back post centre line at height `y`. */
export const postX = (y: number): number => CHAIR.backPost[0][0] + ((CHAIR.backPost[1][0] - CHAIR.backPost[0][0]) * (y - CHAIR.backPost[0][1])) / (CHAIR.backPost[1][1] - CHAIR.backPost[0][1]);

/* ---------------------------------------------------------------- wheels */

/** ⭕️ A closed loop of `n` points round an elliptic section (axial half `a`, radial half `b`) at axial `z`, radial `r`, as a revolve profile. */
export function loop(z: number, r: number, a: number, b: number, n = 14): [number, number][] {
  const out = Array.from({ length: n }, (_, i) => [z + a * Math.cos((2 * Math.PI * i) / n), r + b * Math.sin((2 * Math.PI * i) / n)] as [number, number]);
  return [...out, out[0]!];
}

/** 🛞️ The rear wheel pair: pneumatic 37-540 tyre, box-section rim, flanged hub with quick-release axle, 24 spokes laced two-cross (four families of six by symmetry), hand rim of 19 mm tube on six standoffs 30 mm outboard of the rim. */
export function rearWheels(k: Kit): void {
  const { tyre, rim, hub, handRim, wheelZ } = CHAIR;
  for (const [s, sd] of SIDES) {
    const z = sd * wheelZ;
    k.rev(`rear-tyre-${s}`, "rear-wheel", "rubber", TONE.rubber, loop(z, tyre.centre, tyre.halfWidth, tyre.half, 16), [0, 0, 1], [0, 0, 0], "tyres");
    k.rev(`rear-rim-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.12, [[z - rim.halfWidth, rim.outer], [z - rim.halfWidth + 0.003, rim.outer + 0.002], [z + rim.halfWidth - 0.003, rim.outer + 0.002], [z + rim.halfWidth, rim.outer], [z + rim.halfWidth, rim.inner + 0.006], [z + rim.halfWidth - 0.005, rim.inner], [z - rim.halfWidth + 0.005, rim.inner], [z - rim.halfWidth, rim.inner + 0.006], [z - rim.halfWidth, rim.outer]], [0, 0, 1], [0, 0, 0], "frame-metal");
    const [zi, zo] = [sd * 0.247, sd * 0.316];
    const [fi, fo] = [sd * hub.flangeIn, sd * hub.flangeOut];
    const prof: [number, number][] = [[zi, 0], [zi, 0.011], [zi + sd * 0.004, 0.016], [fi - sd * 0.002, 0.016], [fi - sd * 0.002, hub.flange], [fi + sd * 0.002, hub.flange], [fi + sd * 0.002, 0.015], [fo - sd * 0.002, 0.015], [fo - sd * 0.002, hub.flange], [fo + sd * 0.002, hub.flange], [fo + sd * 0.002, 0.014], [sd * 0.306, 0.011], [sd * 0.307, 0.009], [zo, 0.009], [zo, 0]];
    k.rev(`rear-hub-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.15, sd > 0 ? prof : [...prof].reverse(), [0, 0, 1], [0, 0, 0], "frame-metal");
    for (const [flange, fz] of [["in", fi], ["out", fo]] as const)
      for (const lead of [1, -1]) {
        const phi = rad((flange === "out" ? 15 : 0) + (lead > 0 ? 0 : 30));
        const psi = phi + lead * rad(60);
        const sp = k.cap(`spokes-${flange}-${lead > 0 ? "lead" : "trail"}-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.2, [hub.flange * 0.85 * Math.cos(phi), hub.flange * 0.85 * Math.sin(phi), fz], [(rim.inner + 0.002) * Math.cos(psi), (rim.inner + 0.002) * Math.sin(psi), z], CHAIR.spokeR, CHAIR.spokeR, "frame-metal", { slender: true, texture: true, role: "detail" });
        sp.repeat = { count: 6, rotationAxis: [0, 0, 1], rotationRad: 1.0472, origin: [0, 0, 0] };
      }
    k.rev(`hand-rim-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.08, loop(sd * handRim.z, handRim.r, handRim.tube, handRim.tube, 14), [0, 0, 1], [0, 0, 0], "frame-metal");
    const tab = k.cap(`hand-rim-tabs-${s}`, "rear-wheel", "aluminium", TONE.alu, [0, 0.259, z + sd * (rim.halfWidth - 0.002)], [0, 0.262, sd * (handRim.z - 0.004)], 0.0042, 0.0042, "frame-metal", { role: "detail" });
    tab.repeat = { count: 6, rotationAxis: [0, 0, 1], rotationRad: 1.0472, origin: [0, 0, 0] };
  }
}

/** 🛴️ The castor pair: polyurethane tyre on a polyamide disc wheel with a recessed web and a bearing boss; forks and housings belong to the frame. */
export function castors(k: Kit): void {
  for (const [s, sd] of SIDES) {
    const z = sd * CHAIR.castorZ;
    k.rev(`castor-tyre-${s}`, "castor", "rubber", TONE.rubber, [[z - 0.0125, 0.0485], [z - 0.0125, 0.06], [z - 0.0095, 0.0665], [z - 0.004, 0.0678], [z + 0.004, 0.0678], [z + 0.0095, 0.0665], [z + 0.0125, 0.06], [z + 0.0125, 0.0485], [z - 0.0125, 0.0485]], [0, 0, 1], [0, 0, 0], "tyres");
    const web: [number, number][] = [[z - 0.016, 0], [z - 0.016, 0.0105], [z - 0.0055, 0.0125], [z - 0.0045, 0.042], [z - 0.0105, 0.0455], [z - 0.0105, 0.05], [z + 0.0105, 0.05], [z + 0.0105, 0.0455], [z + 0.0045, 0.042], [z + 0.0055, 0.0125], [z + 0.016, 0.0105], [z + 0.016, 0]];
    k.rev(`castor-wheel-${s}`, "castor", "aluminium", TONE.castor, web, [0, 0, 1], [0, 0, 0], "frame-metal");
  }
}

/* ---------------------------------------------------------------- frame */

/** 🦽️ The rigid frame: per side one tube bent from the seat rail into the front hanger, a lower tube to the castor housing, an axle plate with the axle receiver, the folding back post with its push handle and grip, castor housing and fork, wheel lock; across: footrest loop, seat and back cross tubes, footplate, seat and back upholstery, cushion. */
export function frame(k: Kit): void {
  const t = CHAIR.tube;
  const bar = (id: string, pts: readonly (readonly [number, number, number])[], r: number, region = "frame-metal", tone: number = TONE.alu, material = "aluminium"): void => {
    k.bar(id, "frame", material, tone, pts.map(([x, y, z]) => [...F(x, y, z), r]), region);
  };
  for (const [s, sd] of SIDES) {
    const z = sd * CHAIR.sideZ;
    bar(`side-tube-${s}`, [[-0.082, 0.4515, z], [0.06, 0.4645, z], [0.3, 0.486, z], [0.335, 0.4852, z], [0.36, 0.476, z], [0.378, 0.458, z], [0.39, 0.432, z], [0.452, 0.278, z], [0.513, 0.126, z], [0.521, 0.1, z], [0.531, 0.086, z], [0.545, 0.0795, z * 0.97]], t);
    bar(`lower-tube-${s}`, [[-0.048, 0.338, z], [0.2, 0.264, z], [0.468, 0.18, z]], t * 0.9);
    bar(`back-post-${s}`, [[postX(0.4), 0.4, z], [postX(0.6), 0.6, z], [postX(0.86), 0.86, z], [-0.091, 0.876, z], [-0.099, 0.889, z], [-0.112, 0.896, z], [-0.135, 0.899, z]], t * 0.95);
    k.bar(`push-grip-${s}`, "frame", "rubber", TONE.grip, [[...F(-0.128, 0.8985, z), 0.011], [...F(-0.132, 0.899, z), 0.0165], [...F(-0.2, 0.9025, z), 0.0165], [...F(-0.206, 0.903, z), 0.0125], [...F(-0.209, 0.903, z), 0.0065]], "frame-metal");
    k.box(`axle-plate-${s}`, "frame", "aluminium", TONE.alu, F(-0.006, 0.385, sd * (CHAIR.sideZ + t + 0.0045)), [0.072, 0.15, 0.009], 0.004, "frame-metal");
    k.cap(`axle-receiver-${s}`, "frame", "aluminium", TONE.alu - 0.12, F(0, 0.305, sd * 0.2), F(0, 0.305, sd * 0.2465), 0.0125, 0.0125, "frame-metal");
    k.bar(`axle-strut-${s}`, "frame", "aluminium", TONE.alu - 0.08, [[...F(-0.004, 0.3, sd * (CHAIR.sideZ + t + 0.0045)), 0.016, 0.0045], [...F(-0.006, 0.338, sd * (CHAIR.sideZ + t + 0.0045)), 0.02, 0.0045]], "frame-metal");
    k.cap(`castor-housing-${s}`, "frame", "aluminium", TONE.alu - 0.1, F(CHAIR.stemX, CHAIR.housing[0], z), F(CHAIR.stemX, CHAIR.housing[1], z), 0.0185, 0.0185, "frame-metal");
    k.box(`fork-crown-${s}`, "frame", "aluminium", TONE.alu - 0.1, F(CHAIR.stemX - 0.004, 0.158, z), [0.034, 0.016, 0.054], 0.004, "frame-metal");
    k.cap(`fork-stem-${s}`, "frame", "aluminium", TONE.alu + 0.1, F(CHAIR.stemX, 0.158, z), F(CHAIR.stemX, CHAIR.housing[0] + 0.004, z), 0.0085, 0.0085, "frame-metal");
    const [ax, ay] = CHAIR.castorAxle;
    const legDir = Math.atan2(ay - 0.152, ax - (CHAIR.stemX - 0.008));
    const legLen = Math.hypot(ay - 0.152, ax - (CHAIR.stemX - 0.008)) + 0.012;
    for (const [leg, off] of [["a", 0.021], ["b", -0.021]] as const)
      k.box(`fork-leg-${leg}-${s}`, "frame", "aluminium", TONE.alu - 0.1, F((ax + CHAIR.stemX - 0.008) / 2, (ay + 0.152) / 2, z + off), [legLen, 0.02, 0.008], 0.003, "frame-metal", { rotation: aboutW((legDir * 180) / Math.PI) });
    k.cap(`castor-axle-${s}`, "frame", "aluminium", TONE.alu + 0.1, F(ax, ay, z - 0.028), F(ax, ay, z + 0.028), 0.0065, 0.0065, "frame-metal");
    k.box(`lock-clamp-${s}`, "frame", "aluminium", TONE.dark, F(0.236, 0.474, z + sd * 0.004), [0.045, 0.034, 0.036], 0.006, "frame-metal");
    k.cap(`lock-shoe-${s}`, "frame", "aluminium", TONE.dark, F(0.24, 0.462, sd * (CHAIR.sideZ + 0.018)), F(0.262, 0.452, sd * 0.268), 0.0075, 0.0075, "frame-metal");
    k.bar(`lock-lever-${s}`, "frame", "aluminium", TONE.dark, [[...F(0.238, 0.474, sd * 0.243), 0.006], [...F(0.27, 0.53, sd * 0.243), 0.0055], [...F(0.296, 0.582, sd * 0.243), 0.005]], "frame-metal");
    k.ell(`lock-knob-${s}`, "frame", "rubber", TONE.grip, F(0.3, 0.592, sd * 0.243), [0.014, 0.019, 0.012], "frame-metal", { rotation: aboutW(-28) });
  }
  k.bar("footrest-loop", "frame", "aluminium", TONE.alu, [[0.545, 0.0795, 0.2085], [0.553, 0.077, 0.19], [0.556, 0.0765, 0.16], [0.556, 0.0765, 0], [0.556, 0.0765, -0.16], [0.553, 0.077, -0.19], [0.545, 0.0795, -0.2085]].map(([x, y, z]) => [...F(x!, y!, z!), CHAIR.tube]), "frame-metal");
  k.cap("seat-cross", "frame", "aluminium", TONE.alu, F(0.3, 0.472, -CHAIR.sideZ), F(0.3, 0.472, CHAIR.sideZ), 0.011, 0.011, "frame-metal");
  k.cap("rear-cross", "frame", "aluminium", TONE.alu, F(-0.03, 0.375, -CHAIR.sideZ), F(-0.03, 0.375, CHAIR.sideZ), 0.011, 0.011, "frame-metal");
  k.cap("back-cross", "frame", "aluminium", TONE.alu, F(postX(0.75) - 0.032, 0.75, -CHAIR.sideZ), F(postX(0.75) - 0.032, 0.75, CHAIR.sideZ), 0.0095, 0.0095, "frame-metal");
  for (const [s, sd] of SIDES) k.bar(`back-cross-arm-${s}`, "frame", "aluminium", TONE.alu, [[...F(postX(0.75) - 0.002, 0.75, sd * CHAIR.sideZ), 0.0095], [...F(postX(0.75) - 0.032, 0.75, sd * (CHAIR.sideZ - 0.002)), 0.0095]], "frame-metal");
  const fp = CHAIR.footplate;
  k.ext("footplate", "frame", "aluminium", TONE.alu - 0.05, "zx", [[-fp.half, fp.x0 + 0.02], [-fp.half + 0.02, fp.x0], [fp.half - 0.02, fp.x0], [fp.half, fp.x0 + 0.02], [fp.half, fp.x1 - 0.03], [fp.half - 0.03, fp.x1], [-fp.half + 0.03, fp.x1], [-fp.half, fp.x1 - 0.03]], fp.top - fp.t - 0.305, fp.top - 0.305, "frame-metal");
  const c = CHAIR.cushion;
  const seat = (x: number): number => 0.4515 + ((0.486 - 0.4515) * (x + 0.082)) / 0.382 + CHAIR.tube;
  k.loft("seat-sling", "frame", "foam", TONE.sling, [-0.08, -0.05, 0.1, 0.25, 0.33].map((x) => [x, seat(x) - 0.305 - 0.002, 0, CHAIR.sideZ + 0.014, 0.005, 6] as Row), "cushion");
  const cushion: readonly (readonly [x: number, bottom: number, top: number, half: number, e: number])[] = [[c.x0, seat(c.x0) + 0.006, seat(c.x0) + 0.03, c.half - 0.02, 3], [c.x0 + 0.008, seat(c.x0 + 0.008) + 0.002, seat(c.x0) + 0.05, c.half - 0.004, 3.4], [c.x0 + 0.03, seat(c.x0 + 0.03) + 0.002, seat(c.x0) + 0.058, c.half, 3.6], [0.06, seat(0.06) + 0.002, seat(0.06) + 0.054, c.half, 3.6], [0.17, seat(0.17) + 0.002, seat(0.17) + 0.058, c.half, 3.6], [0.3, seat(0.3) + 0.002, seat(0.3) + 0.06, c.half, 3.6], [c.x1 - 0.022, seat(c.x1 - 0.022) + 0.002, seat(c.x1) + 0.054, c.half - 0.002, 3.4], [c.x1 - 0.006, seat(c.x1) + 0.006, seat(c.x1) + 0.042, c.half - 0.012, 3], [c.x1, seat(c.x1) + 0.014, seat(c.x1) + 0.03, c.half - 0.03, 2.6]];
  k.loft("seat-cushion", "frame", "foam", TONE.foam, cushion.map(([x, b, top, half, e]) => [x, (b + top) / 2 - 0.305, 0, half, (top - b) / 2, e] as Row), "cushion");
  const back: readonly (readonly [y: number, sag: number, half: number])[] = [[CHAIR.sling.bottom, 0.004, CHAIR.sideZ + 0.008], [CHAIR.sling.bottom + 0.01, 0.006, CHAIR.sideZ + 0.016], [0.6, 0.008, CHAIR.sideZ + 0.016], [0.72, 0.01, CHAIR.sideZ + 0.016], [0.82, 0.008, CHAIR.sideZ + 0.016], [CHAIR.sling.top - 0.01, 0.006, CHAIR.sideZ + 0.016], [CHAIR.sling.top, 0.004, CHAIR.sideZ + 0.008]];
  k.loft("back-sling", "frame", "foam", TONE.sling, back.map(([y, sag, half]) => [y - 0.305, postX(y) - sag, 0, 0.0095, half, 5] as Row), "cushion", { across: ACROSS_V });
}

/* ---------------------------------------------------------------- trunk, neck and head */

export const SEAT: Row[] = [[-0.099, 0.04, 0, 0.12, 0.05, 3], [-0.09, 0.036, 0, 0.172, 0.088, 2.8], [-0.065, 0.03, 0, 0.192, 0.1, 2.6], [-0.025, 0.022, 0, 0.197, 0.105, 2.5], [0.025, 0.012, 0, 0.186, 0.104, 2.4], [0.075, 0.002, 0, 0.17, 0.1, 2.4], [0.12, -0.006, 0, 0.16, 0.098, 2.4]];
export const HOODIE: Row[] = [[-0.035, -0.036, 0, 0.176, 0.11, 2.4], [0.0, -0.046, 0, 0.17, 0.114, 2.4], [0.06, -0.05, 0, 0.16, 0.116, 2.4], [0.14, -0.052, 0, 0.162, 0.119, 2.4], [0.22, -0.05, 0, 0.168, 0.124, 2.4], [0.29, -0.047, 0, 0.176, 0.13, 2.4], [0.35, -0.042, 0, 0.185, 0.13, 2.5], [0.4, -0.035, 0, 0.193, 0.122, 2.7], [0.432, -0.028, 0, 0.196, 0.11, 2.8], [0.455, -0.021, 0, 0.182, 0.094, 2.6], [0.472, -0.014, 0, 0.156, 0.082, 2.4], [0.488, -0.008, 0, 0.124, 0.071, 2.2], [0.5, -0.005, 0, 0.094, 0.064, 2.1], [0.509, -0.004, 0, 0.076, 0.061, 2]];
/** 👕️ The surface of the hoodie at height `s` (trunk bone) on the front (`side` -1) or back (+1), at lateral `w`. */
export function hoodieAt(s: number, w = 0, side: -1 | 1 = -1): number {
  for (let i = 1; i < HOODIE.length; i++) if (s <= HOODIE[i]![0]) {
    const [a, b] = [HOODIE[i - 1]!, HOODIE[i]!];
    const t = Math.max(0, (s - a[0]) / (b[0] - a[0]));
    const [cv, rx, ry, e] = [a[1] + (b[1] - a[1]) * t, a[3] + (b[3] - a[3]) * t, a[4] + (b[4] - a[4]) * t, a[5]! + (b[5]! - a[5]!) * t];
    return cv + side * ry * (1 - Math.abs(w / rx) ** e) ** (1 / e);
  }
  const l = HOODIE[HOODIE.length - 1]!;
  return l[1] + side * l[4];
}
export const NECK: Row[] = [[-0.035, -0.008, 0, 0.061, 0.062], [0.02, -0.012, 0, 0.056, 0.058], [0.07, -0.014, 0, 0.054, 0.057], [0.125, -0.008, 0, 0.056, 0.061]];
export const HEAD: Row[] = [[-0.104, -0.083, 0, 0.012, 0.01, 2], [-0.099, -0.078, 0, 0.025, 0.019, 2.2], [-0.088, -0.068, 0, 0.039, 0.031, 2.3], [-0.07, -0.052, 0, 0.051, 0.049, 2.3], [-0.045, -0.035, 0, 0.061, 0.07, 2.2], [-0.015, -0.017, 0, 0.07, 0.09, 2.1], [0.02, -0.007, 0, 0.075, 0.099, 2.1], [0.055, -0.003, 0, 0.077, 0.1, 2.1], [0.088, 0.0, 0, 0.071, 0.092, 2.1], [0.11, 0.003, 0, 0.057, 0.072, 2], [0.124, 0.005, 0, 0.035, 0.045, 2], [0.129, 0.005, 0, 0.012, 0.015, 2]];
export const BEANIE: Row[] = [[0.0, 0.007, 0, 0.076, 0.082, 2.2], [0.03, 0.0035, 0, 0.081, 0.0965, 2.2], [0.055, -0.001, 0, 0.083, 0.105, 2.1], [0.088, 0.0, 0, 0.078, 0.099, 2.1], [0.11, 0.003, 0, 0.064, 0.079, 2], [0.126, 0.005, 0, 0.048, 0.058, 2], [0.137, 0.005, 0, 0.026, 0.031, 2], [0.1425, 0.005, 0, 0.008, 0.01, 2]];
export const CUFF: Row[] = [[-0.004, 0.009, 0, 0.079, 0.086, 2.4], [0.0, 0.009, 0, 0.081, 0.088, 2.4], [0.02, 0.006, 0, 0.086, 0.097, 2.3], [0.04, 0.002, 0, 0.088, 0.105, 2.2], [0.046, 0.001, 0, 0.087, 0.104, 2.2]];

export function trunk(k: Kit): void {
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, SEAT, "jeans");
  k.loft("hoodie", "torso", "fleece", TONE.hoodie, HOODIE, "hoodie");
  k.loft("hoodie-hem", "torso", "fleece", TONE.rib, [[-0.047, -0.034, 0, 0.16, 0.096, 2.4], [-0.042, -0.035, 0, 0.176, 0.11, 2.4], [-0.036, -0.036, 0, 0.181, 0.115, 2.4], [0.0, -0.046, 0, 0.177, 0.119, 2.4], [0.006, -0.047, 0, 0.172, 0.114, 2.4]], "hoodie");
  k.loft("hood", "torso", "fleece", TONE.hood, [[0.33, 0.084, 0, 0.05, 0.008, 2.2], [0.35, 0.094, 0, 0.1, 0.011, 2.4], [0.385, 0.097, 0, 0.126, 0.015, 2.6], [0.435, 0.094, 0, 0.142, 0.018, 2.6], [0.475, 0.081, 0, 0.138, 0.019, 2.6], [0.505, 0.066, 0, 0.118, 0.02, 2.4], [0.522, 0.051, 0, 0.09, 0.017, 2.2], [0.53, 0.041, 0, 0.05, 0.01, 2]], "hoodie");
  const collar = Array.from({ length: 13 }, (_, i) => {
    const a = rad(-160 + (320 * i) / 12);
    const front = Math.max(0, -Math.cos(a));
    return [0.506 - 0.032 * front ** 1.5, -0.006 + 0.074 * Math.cos(a), 0.076 * Math.sin(a), 0.0125];
  });
  k.bar("hood-collar", "torso", "fleece", TONE.hood, collar, "hoodie");
  for (const [s, sd] of SIDES) {
    const w = sd * 0.026;
    k.bar(`drawstring-${s}`, "torso", "fleece", TONE.cord, [[0.485, hoodieAt(0.485, w) - 0.006, w, 0.0024], [0.44, hoodieAt(0.44, w) - 0.003, w * 1.04, 0.0024], [0.39, hoodieAt(0.39, w) - 0.003, w * 1.08, 0.0024], [0.345, hoodieAt(0.345, w) - 0.003, w * 1.08, 0.0024], [0.33, hoodieAt(0.33, w) - 0.004, w * 1.08, 0.0036]], "hoodie", { slender: true, role: "detail" });
  }
  const pocket = (s: number, half: number): Row => [s, hoodieAt(s, 0) + 0.0015, 0, half, 0.0045, 6];
  k.loft("hoodie-pocket", "torso", "fleece", TONE.hoodie - 0.04, [pocket(0.025, 0.112), pocket(0.07, 0.118), pocket(0.12, 0.11), pocket(0.165, 0.085), pocket(0.18, 0.072)], "hoodie", { texture: true, role: "detail" });
}

export function head(k: Kit): void {
  k.loft("neck", "neck", "skin", TONE.skin + 0.02, NECK, "face-skin");
  k.loft("head", "head", "skin", TONE.skin, HEAD, "face-skin");
  k.loft("beanie", "head", "knit-wool", TONE.beanie, BEANIE, "beanie");
  k.loft("beanie-cuff", "head", "knit-wool", TONE.beanie - 0.04, CUFF, "beanie");
  k.loft("nose", "head", "skin", TONE.skin + 0.03, [[0.024, -0.098, 0, 0.007, 0.006, 2], [0.0, -0.11, 0, 0.0095, 0.009, 2], [-0.022, -0.118, 0, 0.0135, 0.0115, 2], [-0.033, -0.112, 0, 0.0165, 0.012, 2], [-0.039, -0.101, 0, 0.0145, 0.01, 2]], "face-skin");
  k.ell("mouth", "head", "skin", TONE.lips, [-0.061, -0.1, 0], [0.0045, 0.006, 0.022], "face-skin", { role: "detail" });
  k.ell("chin-pad", "head", "skin", TONE.skin, [-0.093, -0.072, 0], [0.014, 0.012, 0.024], "face-skin");
  for (const [s, sd] of SIDES) {
    k.ell(`eye-${s}`, "head", "skin", TONE.eye, [0.019, -0.094, sd * 0.032], [0.006, 0.0065, 0.0125], "face-skin", { role: "detail" });
    k.bar(`brow-${s}`, "head", "hair", TONE.brow, [[0.034, -0.097, sd * 0.013, 0.0042], [0.039, -0.095, sd * 0.033, 0.0048], [0.035, -0.087, sd * 0.052, 0.0038]], "face-skin", { role: "detail" });
    k.ell(`earlobe-${s}`, "head", "skin", TONE.skin + 0.02, [-0.008, 0.012, sd * 0.071], [0.011, 0.008, 0.006], "face-skin");
  }
}

/* ---------------------------------------------------------------- limbs */

export const SLEEVE: Row[] = [...dome(-0.055, 0, 0, 0.006, 0.056, 0.058), [0, 0, 0.006, 0.056, 0.058], [0.05, 0.002, 0.008, 0.058, 0.057], [0.12, 0.004, 0.004, 0.052, 0.054], [0.19, 0.006, 0, 0.047, 0.053], [0.26, 0.003, 0, 0.045, 0.049], [0.31, 0, 0, 0.044, 0.045], ...dome(0.355, 0.31, -0.008, 0, 0.044, 0.045).reverse()];
export const FOREARM: Row[] = [...dome(-0.045, 0, 0, 0, 0.046, 0.044), [0, 0, 0, 0.047, 0.044], [0.05, 0.003, 0, 0.05, 0.045], [0.11, 0.002, 0, 0.045, 0.04], [0.17, 0, 0, 0.038, 0.033], [0.205, 0, 0, 0.034, 0.029]];
export const SLEEVE_CUFF: Row[] = [[0.198, 0, 0, 0.034, 0.029, 2.4], [0.205, 0, 0, 0.038, 0.032, 2.4], [0.24, 0, 0, 0.037, 0.031, 2.4], [0.246, 0, 0, 0.033, 0.027, 2.4]];
export const GLOVE_CUFF: Row[] = [[0.226, 0, 0, 0.0325, 0.0235, 2.6], [0.24, 0, 0, 0.034, 0.025, 2.6], [0.262, 0, 0, 0.033, 0.024, 2.6], [0.268, 0, 0, 0.03, 0.021, 2.6]];
export const THIGH: Row[] = [...dome(-0.07, 0, -0.004, 0.01, 0.09, 0.08), [0, -0.004, 0.01, 0.09, 0.08, 2.4], [0.08, -0.004, 0.008, 0.088, 0.078, 2.3], [0.18, -0.002, 0.004, 0.082, 0.073], [0.27, 0.0, 0, 0.073, 0.066], [0.35, 0.004, 0, 0.064, 0.059], [0.41, 0.008, 0, 0.059, 0.057], [0.445, 0.008, 0, 0.058, 0.056], ...dome(0.49, 0.445, 0.008, 0, 0.058, 0.056).reverse()];
export const SHANK: Row[] = [...dome(-0.05, 0, 0.002, 0, 0.056, 0.056), [0, 0.002, 0, 0.056, 0.056], [0.07, -0.006, 0, 0.057, 0.062], [0.14, -0.008, 0, 0.056, 0.062], [0.24, -0.004, 0, 0.052, 0.056], [0.32, 0.0, 0, 0.05, 0.053], [0.37, 0.002, 0, 0.051, 0.055], [0.385, 0.001, 0, 0.0495, 0.0525], [0.4, 0.003, 0, 0.0525, 0.0565], [0.412, 0.003, 0, 0.0515, 0.0555], [0.418, 0.003, 0, 0.048, 0.052]];
/** 👟️ The sneaker upper in a level frame at the ankle (s ahead, cv up; sole bottom at cv -0.085): rounded heel counter, padded collar, instep, vamp to the ball where the toe bone takes over. */
export const UPPER: Row[] = [[-0.07, -0.05, 0, 0.026, 0.028, 2.6], [-0.064, -0.044, 0, 0.036, 0.04, 2.6], [-0.045, -0.038, 0, 0.042, 0.048, 2.6], [-0.01, -0.038, 0, 0.044, 0.048, 2.6], [0.03, -0.047, 0, 0.047, 0.039, 2.6], [0.08, -0.054, 0, 0.05, 0.034, 2.6], [0.12, -0.058, 0, 0.052, 0.03, 2.6], [0.145, -0.059, 0, 0.0525, 0.029, 2.6]];
export const MIDSOLE: Row[] = [[-0.073, -0.0755, 0, 0.026, 0.0095, 4], [-0.066, -0.0755, 0, 0.038, 0.0095, 4], [-0.04, -0.0755, 0, 0.045, 0.0095, 4], [0.02, -0.0755, 0, 0.043, 0.0095, 4], [0.08, -0.0755, 0, 0.05, 0.0095, 4], [0.145, -0.0755, 0, 0.0545, 0.0095, 4]];
export const TOE: Row[] = [[-0.01, -0.016, 0, 0.0525, 0.029, 2.6], [0.03, -0.018, 0, 0.05, 0.026, 2.6], [0.055, -0.021, 0, 0.044, 0.021, 2.6], [0.07, -0.024, 0, 0.033, 0.015, 2.4], [0.077, -0.026, 0, 0.018, 0.009, 2.2]];
export const TOE_SOLE: Row[] = [[-0.01, -0.032, 0, 0.0545, 0.0095, 4], [0.035, -0.032, 0, 0.052, 0.0095, 4], [0.06, -0.031, 0, 0.042, 0.0105, 3.4], [0.073, -0.029, 0, 0.03, 0.012, 3], [0.079, -0.028, 0, 0.016, 0.01, 2.6]];

/** ✋️ One finger as a bent bar in the hand frame (u to the finger tips, v out of the back of the hand, w across), flexing towards the palm (-v): the points of the knuckle, the joints and the tip, with radii. */
export function fingerPath(w: number, scale: number, r: number, flex: readonly [number, number, number], knuckle = 0.096): number[][] {
  const lengths = [0.045, 0.027, 0.022].map((l) => l * scale);
  let [u, p, angle] = [knuckle, -0.002, 0];
  const out: number[][] = [[u - 0.014, p + 0.001, w * 0.9, r * 1.08], [u, p, w, r * 1.1]];
  lengths.forEach((l, i) => {
    const half = (flex[i]! * Math.PI) / 360;
    const corner = Math.min(0.008, l * 0.3);
    angle -= half;
    u += corner * Math.cos(angle);
    p += corner * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.06 * i)]);
    angle -= half;
    u += (l - corner) * Math.cos(angle);
    p += (l - corner) * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.06 * (i + 1)) * (i === 2 ? 0.85 : 1)]);
  });
  return out;
}
/** 🤏️ The relaxed half-closed hand that lies in the lap and closes round the hand rim: finger flexion (index to little finger) and the thumb path `[u, v, towards the thumb side, radius]`. */
export const GRIP_HAND = { flex: [[30, 50, 25], [32, 52, 25], [34, 52, 25], [36, 54, 25]], thumb: [[0.018, -0.008, 0.026, 0.0125], [0.044, -0.02, 0.043, 0.0115], [0.068, -0.032, 0.05, 0.0102], [0.09, -0.044, 0.046, 0.009]] } as const;
export const FINGERS: readonly (readonly [w: number, scale: number, r: number])[] = [[0.029, 0.94, 0.0088], [0.0095, 1, 0.009], [-0.0105, 0.95, 0.0085], [-0.029, 0.8, 0.0076]];

/** 🧤️ A hand in a half-finger leather glove: palm and back with the thenar under the leather, the proximal phalanges gloved, middle and distal phalanges and the thumb tip bare; thumb on the inner side (-w on the left hand, +w on the right). */
export function hand(k: Kit, s: "l" | "r"): void {
  const t = s === "l" ? -1 : 1;
  const bone = `hand-${s}`;
  k.loft(`glove-${s}`, bone, "leather-glove", TONE.glove, [[-0.008, 0, 0, 0.031, 0.022, 2.4], [0.03, -0.001, t * 0.003, 0.041, 0.0185, 2.6], [0.066, -0.002, t * 0.001, 0.046, 0.0165, 2.8], [0.092, -0.002, -t * 0.001, 0.045, 0.0145, 2.8], [0.104, -0.003, -t * 0.001, 0.042, 0.011, 2.5]], "gloves");
  k.ell(`glove-thenar-${s}`, bone, "leather-glove", TONE.glove, [0.038, -0.012, t * 0.027], [0.031, 0.015, 0.018], "gloves");
  FINGERS.forEach(([w, sc, r], i) => {
    const path = fingerPath(t * w, sc, r, GRIP_HAND.flex[i] as [number, number, number]);
    k.bar(`glove-finger-${i + 1}-${s}`, bone, "leather-glove", TONE.glove, path.slice(0, 4).map(([u, p, ww, rr], j) => [u!, p!, ww!, rr! * 1.08 + (j === 3 ? 0.0004 : 0)]), "gloves");
    k.bar(`finger-${i + 1}-${s}`, bone, "skin", TONE.skin, path.slice(3), "hand-skin");
  });
  const th = GRIP_HAND.thumb;
  k.bar(`glove-thumb-${s}`, bone, "leather-glove", TONE.glove, th.slice(0, 3).map(([u, p, w, r]) => [u, p, t * w, r * 1.08]), "gloves");
  k.bar(`thumb-${s}`, bone, "skin", TONE.skin, th.slice(2).map(([u, p, w, r]) => [u, p, t * w, r]), "hand-skin");
}

export function limbs(k: Kit): void {
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "fleece", TONE.hoodie - 0.02, SLEEVE.map(([a, cv, cw, rx, ry]) => [a, cv, side * cw, rx, ry] as Row), "hoodie");
    k.loft(`forearm-${s}`, `forearm-${s}`, "fleece", TONE.hoodie - 0.02, FOREARM, "hoodie");
    k.loft(`sleeve-cuff-${s}`, `forearm-${s}`, "fleece", TONE.rib, SLEEVE_CUFF, "hoodie");
    k.loft(`glove-cuff-${s}`, `forearm-${s}`, "leather-glove", TONE.glove - 0.04, GLOVE_CUFF, "gloves");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, THIGH.map(([a, cv, cw, rx, ry, e]) => (e === undefined ? [a, cv, side * cw, rx, ry] : [a, cv, side * cw, rx, ry, e]) as Row), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, SHANK, "jeans");
    const level = { rotation: aboutW(FOOT_DROP) };
    k.loft(`sneaker-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, UPPER, "shoes", level);
    k.loft(`midsole-${s}`, `foot-${s}`, "canvas-sneaker", TONE.sole, MIDSOLE, "shoes", level);
    k.loft(`sneaker-toe-${s}`, `toe-${s}`, "canvas-sneaker", TONE.canvas, TOE, "shoes");
    k.loft(`toe-sole-${s}`, `toe-${s}`, "canvas-sneaker", TONE.sole, TOE_SOLE, "shoes");
    hand(k, s);
  }
}

/** 🧱️ The whole 1:50 body. */
export function body50(k: Kit): void {
  rearWheels(k);
  castors(k);
  frame(k);
  trunk(k);
  head(k);
  limbs(k);
}
