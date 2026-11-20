/** 🧩️ The 1:100, 1:200 and 1:500 bodies of the round 3 wheelchair user, each stated anew for its smallest feature (15 / 30 / 75 mm): fewer and simpler sections, merged masses, the chair reduced to what reads at that scale and generalised where the paper cannot hold the real thickness (stated in the decisions of each scale).
 * @see ./r3-wheelchair-user-s50.ts
 * @see ./r3-humans-wheelchair-user.md */
import { ACROSS_V, Kit, aboutW, dome, type Row, type V3 } from "./r3-wheelchair-user-kit.ts";
import { CHAIR } from "./r3-wheelchair-user-ref.ts";
import { FOOT_DROP } from "./r3-wheelchair-user-roots.ts";
import { F, TONE, loop, postX } from "./r3-wheelchair-user-s50.ts";

const SIDES = [["l", 1], ["r", -1]] as const;
type P = readonly [number, number, number];
const seatTop = (x: number): number => 0.4515 + ((0.486 - 0.4515) * (x + 0.082)) / 0.382 + CHAIR.tube;
const tubeBar = (k: Kit, id: string, pts: readonly P[], r: number, o: { slender?: boolean; material?: string; tone?: number } = {}): void => {
  k.bar(id, "frame", o.material ?? "aluminium", o.tone ?? TONE.alu, pts.map(([x, y, z]) => [...F(x, y, z), r]), "frame-metal", o.slender === true ? { slender: true } : {});
};
const sidePath = (z: number, n: "fine" | "mid" | "coarse"): P[] =>
  n === "fine" ? [[-0.082, 0.4515, z], [0.3, 0.486, z], [0.345, 0.482, z], [0.375, 0.462, z], [0.39, 0.432, z], [0.513, 0.126, z], [0.525, 0.094, z], [0.545, 0.0795, z * 0.97]] : n === "mid" ? [[-0.082, 0.4515, z], [0.31, 0.486, z], [0.37, 0.466, z], [0.392, 0.428, z], [0.515, 0.12, z], [0.545, 0.0795, z * 0.97]] : [[-0.09, 0.45, z], [0.33, 0.486, z], [0.39, 0.432, z], [0.52, 0.1, z], [0.548, 0.08, z * 0.94]];
const postPath = (z: number): P[] => [[postX(0.4), 0.4, z], [postX(0.86), 0.86, z], [-0.095, 0.884, z], [-0.135, 0.899, z]];

/* ---------------------------------------------------------------- 1:100 */

/** 🧱️ 1:100: every assembly of the chair kept with simplified sections (tyre, rim, hub barrel, 12 spokes, hand rim touching the rim, castor wheel in one profile, one housing-and-fork block, tube frame, footrest, footplate, cushion on the seat sling, back sling, grips, wheel lock levers); the man with mitten hands (gloved palm, bare finger block, thumb), shoes in one upper and one sole, no eyes, brows, cords or ear lobes. */
export function body100(k: Kit): void {
  const { tyre, rim, handRim, wheelZ } = CHAIR;
  for (const [s, sd] of SIDES) {
    const z = sd * wheelZ;
    k.rev(`rear-tyre-${s}`, "rear-wheel", "rubber", TONE.rubber, loop(z, tyre.centre, tyre.halfWidth, tyre.half, 10), [0, 0, 1], [0, 0, 0], "tyres");
    k.rev(`rear-rim-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.12, [[z - rim.halfWidth, rim.outer + 0.002], [z + rim.halfWidth, rim.outer + 0.002], [z + rim.halfWidth, rim.inner], [z - rim.halfWidth, rim.inner], [z - rim.halfWidth, rim.outer + 0.002]], [0, 0, 1], [0, 0, 0], "frame-metal");
    k.rev(`rear-hub-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.15, sd > 0 ? [[0.247, 0], [0.247, 0.022], [0.312, 0.022], [0.312, 0]] : [[-0.312, 0], [-0.312, 0.022], [-0.247, 0.022], [-0.247, 0]], [0, 0, 1], [0, 0, 0], "frame-metal");
    for (const lead of [1, -1]) {
      const phi = lead > 0 ? 0 : Math.PI / 6;
      const psi = phi + lead * (Math.PI / 3);
      const sp = k.cap(`spokes-${lead > 0 ? "lead" : "trail"}-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.2, [0.02 * Math.cos(phi), 0.02 * Math.sin(phi), sd * (lead > 0 ? 0.258 : 0.298)], [rim.inner * Math.cos(psi), rim.inner * Math.sin(psi), z], 0.0012, 0.0012, "frame-metal", { slender: true, texture: true, role: "detail" });
      sp.repeat = { count: 6, rotationAxis: [0, 0, 1], rotationRad: 1.0472, origin: [0, 0, 0] };
    }
    k.rev(`hand-rim-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.08, loop(sd * (handRim.z - 0.004), handRim.r, 0.0105, 0.0105, 10), [0, 0, 1], [0, 0, 0], "frame-metal");
    const cz = sd * CHAIR.castorZ;
    k.rev(`castor-wheel-${s}`, "castor", "rubber", TONE.rubber, [[cz - 0.013, 0], [cz - 0.013, 0.058], [cz - 0.008, 0.0678], [cz + 0.008, 0.0678], [cz + 0.013, 0.058], [cz + 0.013, 0]], [0, 0, 1], [0, 0, 0], "tyres");
    const side = sd * CHAIR.sideZ;
    tubeBar(k, `side-tube-${s}`, sidePath(side, "fine"), CHAIR.tube);
    tubeBar(k, `lower-tube-${s}`, [[-0.048, 0.338, side], [0.468, 0.18, side]], 0.0115);
    tubeBar(k, `back-post-${s}`, postPath(side), 0.012);
    k.bar(`push-grip-${s}`, "frame", "rubber", TONE.grip, [[...F(-0.13, 0.899, side), 0.016], [...F(-0.208, 0.903, side), 0.016]], "frame-metal");
    k.box(`axle-plate-${s}`, "frame", "aluminium", TONE.alu, F(-0.006, 0.385, sd * (CHAIR.sideZ + 0.017)), [0.072, 0.15, 0.015], 0.006, "frame-metal");
    k.cap(`castor-housing-${s}`, "frame", "aluminium", TONE.alu - 0.1, F(CHAIR.stemX, 0.15, side), F(CHAIR.stemX, CHAIR.housing[1], side), 0.019, 0.019, "frame-metal");
    const [ax, ay] = CHAIR.castorAxle;
    k.box(`castor-fork-${s}`, "frame", "aluminium", TONE.alu - 0.1, F((ax + CHAIR.stemX) / 2, (ay + 0.15) / 2, side), [0.1, 0.022, 0.05], 0.008, "frame-metal", { rotation: aboutW((Math.atan2(0.15 - ay, CHAIR.stemX - ax) * 180) / Math.PI) });
    k.bar(`lock-lever-${s}`, "frame", "aluminium", TONE.dark, [[...F(0.236, 0.474, sd * 0.235), 0.009], [...F(0.296, 0.582, sd * 0.243), 0.009], [...F(0.302, 0.6, sd * 0.243), 0.011]], "frame-metal");
  }
  tubeBar(k, "footrest-loop", [[0.545, 0.0795, 0.2085], [0.556, 0.0765, 0.17], [0.556, 0.0765, -0.17], [0.545, 0.0795, -0.2085]], CHAIR.tube);
  k.cap("back-cross", "frame", "aluminium", TONE.alu, F(postX(0.75) - 0.02, 0.75, -CHAIR.sideZ), F(postX(0.75) - 0.02, 0.75, CHAIR.sideZ), 0.0095, 0.0095, "frame-metal");
  const fp = CHAIR.footplate;
  k.box("footplate", "frame", "aluminium", TONE.alu - 0.05, F((fp.x0 + fp.x1) / 2, fp.top - 0.005, 0), [fp.x1 - fp.x0, 0.0115, 2 * fp.half], 0.004, "frame-metal");
  const c = CHAIR.cushion;
  k.loft("seat-cushion", "frame", "foam", TONE.foam, [[c.x0, seatTop(c.x0) + 0.022 - 0.305, 0, c.half - 0.01, 0.026, 3], [c.x0 + 0.03, seatTop(c.x0 + 0.03) + 0.03 - 0.305, 0, c.half + 0.006, 0.032, 3.6], [0.3, seatTop(0.3) + 0.03 - 0.305, 0, c.half + 0.006, 0.032, 3.6], [c.x1, seatTop(c.x1) + 0.024 - 0.305, 0, c.half - 0.01, 0.022, 3]], "cushion");
  k.loft("back-sling", "frame", "foam", TONE.sling, [[CHAIR.sling.bottom - 0.305, postX(CHAIR.sling.bottom) - 0.004, 0, 0.0095, CHAIR.sideZ + 0.012, 5], [0.72 - 0.305, postX(0.72) - 0.01, 0, 0.0095, CHAIR.sideZ + 0.016, 5], [CHAIR.sling.top - 0.305, postX(CHAIR.sling.top) - 0.004, 0, 0.0095, CHAIR.sideZ + 0.012, 5]], "cushion", { across: ACROSS_V });
  man100(k);
}

const HOODIE100: Row[] = [[-0.042, -0.035, 0, 0.18, 0.114, 2.4], [0.06, -0.05, 0, 0.162, 0.117, 2.4], [0.22, -0.05, 0, 0.168, 0.124, 2.4], [0.35, -0.042, 0, 0.185, 0.13, 2.5], [0.432, -0.028, 0, 0.196, 0.11, 2.8], [0.455, -0.021, 0, 0.182, 0.094, 2.6], [0.472, -0.014, 0, 0.156, 0.082, 2.4], [0.49, -0.008, 0, 0.118, 0.07, 2.2], [0.509, -0.004, 0, 0.076, 0.061, 2]];
export const HEAD100: Row[] = [[-0.104, -0.08, 0, 0.016, 0.014, 2], [-0.09, -0.068, 0, 0.038, 0.031, 2.3], [-0.06, -0.045, 0, 0.056, 0.058, 2.3], [-0.015, -0.017, 0, 0.07, 0.09, 2.1], [0.03, -0.006, 0, 0.076, 0.1, 2.1], [0.09, 0.0, 0, 0.07, 0.091, 2.1], [0.122, 0.005, 0, 0.038, 0.048, 2], [0.129, 0.005, 0, 0.012, 0.015, 2]];
const BEANIE100: Row[] = [[-0.004, 0.009, 0, 0.081, 0.088, 2.4], [0.04, 0.002, 0, 0.088, 0.106, 2.2], [0.088, 0.0, 0, 0.078, 0.099, 2.1], [0.122, 0.005, 0, 0.054, 0.065, 2], [0.142, 0.005, 0, 0.012, 0.014, 2]];

/** 🧍️ The 1:100 man. */
function man100(k: Kit): void {
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.099, 0.04, 0, 0.13, 0.055, 3], [-0.075, 0.032, 0, 0.19, 0.098, 2.6], [-0.025, 0.022, 0, 0.197, 0.105, 2.5], [0.06, 0.004, 0, 0.172, 0.1, 2.4], [0.12, -0.006, 0, 0.16, 0.098, 2.4]], "jeans");
  k.loft("hoodie", "torso", "fleece", TONE.hoodie, HOODIE100, "hoodie");
  k.loft("hood", "torso", "fleece", TONE.hood, [[0.33, 0.1, 0, 0.06, 0.009, 2.2], [0.385, 0.104, 0, 0.128, 0.02, 2.6], [0.455, 0.096, 0, 0.145, 0.026, 2.6], [0.512, 0.06, 0, 0.11, 0.024, 2.3], [0.53, 0.04, 0, 0.05, 0.012, 2]], "hoodie");
  k.loft("hoodie-pocket", "torso", "fleece", TONE.hoodie - 0.04, [[0.025, -0.162, 0, 0.112, 0.0045, 6], [0.12, -0.17, 0, 0.11, 0.0045, 6], [0.18, -0.169, 0, 0.072, 0.0045, 6]], "hoodie", { texture: true, role: "detail" });
  k.loft("neck", "neck", "skin", TONE.skin + 0.02, [[-0.035, -0.008, 0, 0.061, 0.062], [0.07, -0.014, 0, 0.054, 0.057], [0.125, -0.008, 0, 0.056, 0.061]], "face-skin");
  k.loft("head", "head", "skin", TONE.skin, HEAD100, "face-skin");
  k.loft("beanie", "head", "knit-wool", TONE.beanie, BEANIE100, "beanie");
  k.loft("nose", "head", "skin", TONE.skin + 0.03, [[0.02, -0.1, 0, 0.008, 0.008, 2], [-0.022, -0.118, 0, 0.014, 0.012, 2], [-0.038, -0.102, 0, 0.015, 0.01, 2]], "face-skin");
  for (const [s, side] of SIDES) {
    const t = s === "l" ? -1 : 1;
    k.loft(`sleeve-${s}`, `arm-${s}`, "fleece", TONE.hoodie - 0.02, [...dome(-0.055, 0, 0, side * 0.006, 0.056, 0.058, 3), [0.05, 0.002, side * 0.008, 0.058, 0.057], [0.19, 0.006, 0, 0.047, 0.053], [0.31, 0, 0, 0.044, 0.045], ...dome(0.35, 0.31, -0.006, 0, 0.044, 0.045, 3).reverse()], "hoodie");
    k.loft(`forearm-${s}`, `forearm-${s}`, "fleece", TONE.hoodie - 0.02, [...dome(-0.045, 0, 0, 0, 0.046, 0.044, 3), [0.05, 0.003, 0, 0.05, 0.045], [0.17, 0, 0, 0.038, 0.033], [0.205, 0, 0, 0.037, 0.031], [0.244, 0, 0, 0.035, 0.029]], "hoodie");
    k.loft(`glove-${s}`, `hand-${s}`, "leather-glove", TONE.glove, [[-0.03, 0, 0, 0.033, 0.024, 2.6], [0.0, 0, 0, 0.033, 0.022, 2.4], [0.05, -0.002, t * 0.002, 0.045, 0.019, 2.8], [0.1, -0.003, 0, 0.045, 0.016, 2.8], [0.115, -0.008, 0, 0.042, 0.015, 2.6]], "gloves");
    k.loft(`fingers-${s}`, `hand-${s}`, "skin", TONE.skin, [[0.112, -0.01, 0, 0.038, 0.011, 2.8], [0.135, -0.022, 0, 0.037, 0.0115, 2.8], [0.15, -0.038, 0, 0.033, 0.01, 2.6], [0.155, -0.048, 0, 0.027, 0.008, 2.4]], "hand-skin");
    k.bar(`thumb-${s}`, `hand-${s}`, "leather-glove", TONE.glove, [[0.018, -0.008, t * 0.026, 0.0135], [0.044, -0.02, t * 0.043, 0.012], [0.068, -0.032, t * 0.05, 0.0105], [0.088, -0.043, t * 0.046, 0.009]], "gloves");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [...dome(-0.07, 0, -0.004, side * 0.01, 0.09, 0.08, 3), [0.08, -0.004, side * 0.008, 0.088, 0.078], [0.27, 0.0, 0, 0.073, 0.066], [0.41, 0.008, 0, 0.059, 0.057], ...dome(0.49, 0.43, 0.008, 0, 0.058, 0.056, 3).reverse()], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [...dome(-0.05, 0, 0.002, 0, 0.056, 0.056, 3), [0.1, -0.007, 0, 0.057, 0.062], [0.3, -0.001, 0, 0.05, 0.054], [0.41, 0.003, 0, 0.052, 0.056], [0.418, 0.003, 0, 0.048, 0.052]], "jeans");
    const level = { rotation: aboutW(FOOT_DROP) };
    k.loft(`sneaker-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.07, -0.052, 0, 0.03, 0.03, 2.6], [-0.045, -0.042, 0, 0.042, 0.046, 2.6], [0.03, -0.05, 0, 0.047, 0.037, 2.6], [0.145, -0.061, 0, 0.0525, 0.027, 2.6]], "shoes", level);
    k.loft(`midsole-${s}`, `foot-${s}`, "canvas-sneaker", TONE.sole, [[-0.073, -0.0755, 0, 0.03, 0.0095, 4], [-0.04, -0.0755, 0, 0.045, 0.0095, 4], [0.145, -0.0755, 0, 0.0545, 0.0095, 4]], "shoes", level);
    k.loft(`sneaker-toe-${s}`, `toe-${s}`, "canvas-sneaker", TONE.canvas, [[-0.01, -0.018, 0, 0.0525, 0.028, 2.6], [0.05, -0.022, 0, 0.045, 0.022, 2.6], [0.077, -0.028, 0, 0.022, 0.011, 2.2]], "shoes");
    k.loft(`toe-sole-${s}`, `toe-${s}`, "canvas-sneaker", TONE.sole, [[-0.01, -0.032, 0, 0.0545, 0.0095, 4], [0.06, -0.031, 0, 0.042, 0.0105, 3.4], [0.079, -0.028, 0, 0.018, 0.01, 2.6]], "shoes");
  }
}

/* ---------------------------------------------------------------- 1:200 */

/** 🧱️ 1:200: each rear wheel one ring of tyre, rim and hand rim with a hub and six spokes, castors as discs, the frame as 30 mm tubes (side tube, lower tube, back post with grip, one block for housing and fork, footrest), a 12 mm footplate, cushion and back sling; the man in fourteen masses (seat, hoodie with the hood, neck, head, beanie, sleeves, forearms, mittens with a bare finger block, thighs, shanks, shoes). */
export function body200(k: Kit): void {
  for (const [s, sd] of SIDES) {
    const z = sd * CHAIR.wheelZ;
    const left: [number, number][] = [[0.263, 0.252], [0.263, 0.29], [0.272, 0.305], [0.288, 0.305], [0.297, 0.29], [0.309, 0.276], [0.32, 0.276], [0.32, 0.252], [0.263, 0.252]];
    k.rev(`wheel-${s}`, "rear-wheel", "rubber", TONE.rubber, sd > 0 ? left : left.map(([a, r]) => [-a, r] as [number, number]).reverse(), [0, 0, 1], [0, 0, 0], "tyres");
    k.rev(`rear-hub-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.15, sd > 0 ? [[0.247, 0], [0.247, 0.03], [0.315, 0.03], [0.315, 0]] : [[-0.315, 0], [-0.315, 0.03], [-0.247, 0.03], [-0.247, 0]], [0, 0, 1], [0, 0, 0], "frame-metal");
    const sp = k.cap(`spokes-${s}`, "rear-wheel", "aluminium", TONE.alu + 0.2, [0.025, 0, sd * 0.28], [0.254, 0, z], 0.0015, 0.0015, "frame-metal", { slender: true, texture: true, role: "detail" });
    sp.repeat = { count: 6, rotationAxis: [0, 0, 1], rotationRad: 1.0472, origin: [0, 0, 0] };
    const cz = sd * CHAIR.castorZ;
    k.rev(`castor-wheel-${s}`, "castor", "rubber", TONE.rubber, [[cz - 0.015, 0], [cz - 0.015, 0.0678], [cz + 0.015, 0.0678], [cz + 0.015, 0]], [0, 0, 1], [0, 0, 0], "tyres");
    const side = sd * CHAIR.sideZ;
    tubeBar(k, `side-tube-${s}`, sidePath(side, "mid"), 0.0145, { slender: true });
    tubeBar(k, `lower-tube-${s}`, [[-0.05, 0.34, side], [0.47, 0.18, side]], 0.013, { slender: true });
    tubeBar(k, `back-post-${s}`, [...postPath(side), [-0.21, 0.903, side]], 0.0145, { slender: true });
    const [ax, ay] = CHAIR.castorAxle;
    k.box(`castor-fork-${s}`, "frame", "aluminium", TONE.alu - 0.1, F((ax + CHAIR.stemX) / 2 + 0.01, 0.15, side), [0.05, 0.19, 0.032], 0.012, "frame-metal");
  }
  tubeBar(k, "footrest-loop", [[0.548, 0.0795, 0.2], [0.556, 0.076, 0.17], [0.556, 0.076, -0.17], [0.548, 0.0795, -0.2]], 0.015);
  const fp = CHAIR.footplate;
  k.box("footplate", "frame", "aluminium", TONE.alu - 0.05, F((fp.x0 + fp.x1) / 2, fp.top - 0.006, 0), [fp.x1 - fp.x0, 0.012, 2 * fp.half], 0.005, "frame-metal");
  const c = CHAIR.cushion;
  k.loft("seat-cushion", "frame", "foam", TONE.foam, [[c.x0, seatTop(c.x0) + 0.022 - 0.305, 0, CHAIR.sideZ, 0.034, 3.4], [0.3, seatTop(0.3) + 0.026 - 0.305, 0, CHAIR.sideZ + 0.006, 0.038, 3.6], [c.x1, seatTop(c.x1) + 0.022 - 0.305, 0, CHAIR.sideZ, 0.03, 3.2]], "cushion");
  k.loft("back-sling", "frame", "foam", TONE.sling, [[CHAIR.sling.bottom - 0.305, postX(CHAIR.sling.bottom) - 0.006, 0, 0.016, CHAIR.sideZ + 0.016, 5], [CHAIR.sling.top - 0.305, postX(CHAIR.sling.top) - 0.006, 0, 0.016, CHAIR.sideZ + 0.016, 5]], "cushion", { across: ACROSS_V });
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.098, 0.038, 0, 0.15, 0.065, 2.8], [-0.04, 0.026, 0, 0.197, 0.104, 2.5], [0.12, -0.006, 0, 0.162, 0.1, 2.4]], "jeans");
  k.loft("hoodie", "torso", "fleece", TONE.hoodie, [[-0.042, -0.035, 0, 0.18, 0.114, 2.4], [0.1, -0.05, 0, 0.164, 0.118, 2.4], [0.33, -0.044, 0, 0.183, 0.135, 2.5], [0.43, -0.028, 0, 0.197, 0.115, 2.7], [0.465, -0.017, 0, 0.17, 0.088, 2.4], [0.492, -0.007, 0, 0.115, 0.068, 2.2], [0.51, -0.004, 0, 0.076, 0.064, 2]], "hoodie");
  k.loft("hood", "torso", "fleece", TONE.hood, [[0.34, 0.1, 0, 0.08, 0.016, 2.4], [0.44, 0.1, 0, 0.145, 0.028, 2.6], [0.52, 0.055, 0, 0.1, 0.022, 2.2]], "hoodie");
  k.loft("neck", "neck", "skin", TONE.skin + 0.02, [[-0.035, -0.008, 0, 0.06, 0.062], [0.125, -0.008, 0, 0.056, 0.06]], "face-skin");
  k.loft("head", "head", "skin", TONE.skin, [[-0.104, -0.078, 0, 0.022, 0.018, 2], [-0.07, -0.05, 0, 0.052, 0.054, 2.3], [-0.015, -0.017, 0, 0.07, 0.092, 2.1], [0.05, -0.004, 0, 0.077, 0.101, 2.1], [0.12, 0.005, 0, 0.04, 0.05, 2]], "face-skin");
  k.loft("beanie", "head", "knit-wool", TONE.beanie, [[-0.004, 0.009, 0, 0.082, 0.09, 2.4], [0.045, 0.001, 0, 0.088, 0.107, 2.2], [0.105, 0.003, 0, 0.068, 0.084, 2.1], [0.142, 0.005, 0, 0.015, 0.017, 2]], "beanie");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "fleece", TONE.hoodie - 0.02, [[-0.05, 0, side * 0.006, 0.03, 0.032], [0.0, 0, side * 0.007, 0.058, 0.058], [0.18, 0.005, 0, 0.049, 0.053], [0.32, 0, 0, 0.044, 0.045], [0.35, -0.006, 0, 0.03, 0.03]], "hoodie");
    k.loft(`forearm-${s}`, `forearm-${s}`, "fleece", TONE.hoodie - 0.02, [[-0.04, 0, 0, 0.03, 0.03], [0.0, 0, 0, 0.047, 0.044], [0.12, 0.002, 0, 0.045, 0.04], [0.245, 0, 0, 0.036, 0.03]], "hoodie");
    k.loft(`glove-${s}`, `hand-${s}`, "leather-glove", TONE.glove, [[-0.03, 0, 0, 0.033, 0.024, 2.6], [0.05, -0.002, 0, 0.045, 0.019, 2.8], [0.112, -0.006, 0, 0.044, 0.016, 2.8]], "gloves");
    k.loft(`fingers-${s}`, `hand-${s}`, "skin", TONE.skin, [[0.108, -0.008, 0, 0.042, 0.016, 2.8], [0.14, -0.026, 0, 0.04, 0.016, 2.8], [0.156, -0.048, 0, 0.032, 0.012, 2.4]], "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [[-0.06, -0.004, side * 0.01, 0.06, 0.055], [0.0, -0.004, side * 0.01, 0.09, 0.08], [0.25, 0.0, side * 0.002, 0.075, 0.068], [0.44, 0.008, 0, 0.058, 0.056], [0.485, 0.008, 0, 0.034, 0.033]], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [[-0.045, 0.002, 0, 0.034, 0.034], [0.0, 0.002, 0, 0.056, 0.056], [0.12, -0.007, 0, 0.057, 0.062], [0.418, 0.003, 0, 0.051, 0.055]], "jeans");
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.073, -0.06, 0, 0.032, 0.03, 2.8], [-0.04, -0.045, 0, 0.045, 0.045, 2.8], [0.05, -0.06, 0, 0.05, 0.03, 3], [0.15, -0.066, 0, 0.054, 0.02, 3], [0.205, -0.07, 0, 0.042, 0.016, 2.8], [0.222, -0.07, 0, 0.02, 0.011, 2.4]], "shoes", { rotation: aboutW(FOOT_DROP) });
  }
}

/* ---------------------------------------------------------------- 1:500 */

/** 🧱️ 1:500: the chair as two wheel rings with hub discs, two castor discs that stand for wheel, fork and swivel, the side tubes and back posts with grips as slender bars, a slender footrest bar, a 75 mm seat block and back block; the man as head, beanie, one trunk with the hood, sleeves, forearms with the gloved hands, thighs, shanks and shoes. Everything thinner than 75 mm is either a slender bar drawn as a line or merged into the next mass. */
export function body500(k: Kit): void {
  for (const [s, sd] of SIDES) {
    k.rev(`wheel-${s}`, "rear-wheel", "rubber", TONE.rubber, sd > 0 ? [[0.245, 0.23], [0.245, 0.305], [0.32, 0.305], [0.32, 0.23], [0.245, 0.23]] : [[-0.32, 0.23], [-0.32, 0.305], [-0.245, 0.305], [-0.245, 0.23], [-0.32, 0.23]], [0, 0, 1], [0, 0, 0], "tyres");
    k.rev(`rear-hub-${s}`, "rear-wheel", "aluminium", TONE.alu, sd > 0 ? [[0.24, 0], [0.24, 0.04], [0.316, 0.04], [0.316, 0]] : [[-0.316, 0], [-0.316, 0.04], [-0.24, 0.04], [-0.24, 0]], [0, 0, 1], [0, 0, 0], "frame-metal");
    const cz = sd * CHAIR.castorZ;
    k.rev(`castor-wheel-${s}`, "castor", "rubber", TONE.rubber, [[cz - 0.038, 0], [cz - 0.038, 0.0678], [cz + 0.038, 0.0678], [cz + 0.038, 0]], [0, 0, 1], [0, 0, 0], "tyres");
    const side = sd * CHAIR.sideZ;
    tubeBar(k, `side-tube-${s}`, sidePath(side, "coarse"), 0.0135, { slender: true });
    tubeBar(k, `back-post-${s}`, [...postPath(side), [-0.21, 0.903, side]], 0.0135, { slender: true });
  }
  tubeBar(k, "footrest-loop", [[0.555, 0.078, 0.215], [0.555, 0.078, -0.215]], 0.0105, { slender: true });
  const c = CHAIR.cushion;
  k.box("seat-block", "frame", "foam", TONE.foam, F((c.x0 + c.x1) / 2, 0.5, 0), [c.x1 - c.x0 + 0.02, 0.076, 2 * CHAIR.sideZ + 0.03], 0.02, "cushion", { rotation: aboutW(5) });
  k.loft("back-block", "frame", "foam", TONE.sling, [[CHAIR.sling.bottom - 0.305, postX(CHAIR.sling.bottom) - 0.03, 0, 0.038, CHAIR.sideZ + 0.015, 4], [CHAIR.sling.top - 0.305, postX(CHAIR.sling.top) - 0.03, 0, 0.038, CHAIR.sideZ + 0.015, 4]], "cushion", { across: ACROSS_V });
  k.loft("hoodie", "torso", "fleece", TONE.hoodie, [[-0.06, -0.03, 0, 0.19, 0.115, 2.4], [0.2, -0.04, 0, 0.17, 0.135, 2.4], [0.42, -0.005, 0, 0.198, 0.13, 2.6], [0.47, 0.0, 0, 0.16, 0.1, 2.3], [0.51, 0.01, 0, 0.09, 0.08, 2.1]], "hoodie");
  k.loft("head", "head", "skin", TONE.skin, [[-0.13, -0.045, 0, 0.06, 0.06, 2], [-0.1, -0.06, 0, 0.065, 0.075, 2.2], [-0.03, -0.015, 0, 0.074, 0.098, 2.1], [0.03, -0.006, 0, 0.076, 0.1, 2.1]], "face-skin");
  k.loft("beanie", "head", "knit-wool", TONE.beanie, [[0.01, 0.006, 0, 0.086, 0.1, 2.3], [0.08, 0.0, 0, 0.08, 0.102, 2.1], [0.142, 0.005, 0, 0.03, 0.035, 2]], "beanie");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "fleece", TONE.hoodie - 0.02, [[-0.05, 0, side * 0.01, 0.04, 0.04], [0.02, 0, side * 0.008, 0.058, 0.058], [0.33, -0.004, 0, 0.045, 0.045], [0.36, -0.006, 0, 0.032, 0.032]], "hoodie");
    k.loft(`forearm-${s}`, `forearm-${s}`, "fleece", TONE.hoodie - 0.02, [[-0.035, 0, 0, 0.038, 0.038], [0.0, 0, 0, 0.047, 0.044], [0.25, 0, 0, 0.04, 0.038], [0.31, 0, 0, 0.042, 0.038], [0.34, 0, 0, 0.03, 0.03]], "hoodie");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [[-0.05, -0.006, side * 0.012, 0.075, 0.07], [0.04, -0.004, side * 0.01, 0.09, 0.08], [0.44, 0.008, 0, 0.06, 0.058], [0.49, 0.008, 0, 0.04, 0.04]], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [[-0.04, 0.0, 0, 0.042, 0.042], [0.02, 0.0, 0, 0.056, 0.058], [0.42, 0.002, 0, 0.052, 0.056]], "jeans");
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.075, -0.05, 0, 0.04, 0.04, 2.8], [-0.04, -0.047, 0, 0.05, 0.05, 3], [0.15, -0.05, 0, 0.055, 0.04, 3.2], [0.215, -0.052, 0, 0.042, 0.038, 3]], "shoes", { rotation: aboutW(FOOT_DROP) });
  }
}

export type { V3 };
