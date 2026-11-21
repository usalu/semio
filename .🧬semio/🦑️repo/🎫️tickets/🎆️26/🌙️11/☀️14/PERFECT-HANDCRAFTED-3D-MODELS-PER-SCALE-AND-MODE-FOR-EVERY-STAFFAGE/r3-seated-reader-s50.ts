/** 📖️ The 1:50 body of the round 3 seated reader, every section stated by hand: a woman of 1.68 m in a thick roll-neck sweater, straight jeans and canvas sneakers, her hair in a bun (rest frames of the rig: seated, arms hanging; the pose brings the open hands under the notebook), a 1.20 m painted timber park bench of two end frames and bolted slats, an A5 top-bound notebook and a paper cup with lid and sleeve.
 * Rows are `[s, cv, cw, rx, ry, e?]`: s along the bone, cv along its normal (pelvis, trunk, neck and head: backwards; arms and shank: forwards; thigh: upwards), cw to the left, rx the lateral and ry the normal semi-axis. Bench points are stated in world coordinates (x ahead, y up, z left) and carried into the frame of their bone by `local`.
 * @see ./r3-seated-reader-ref.ts
 * @see ./r3-humans-seated-reader.md */
import { Kit, aboutU, aboutW, dome, quat, type P2, type Row, type V3 } from "./r3-seated-reader-kit.ts";
import { BENCH, BENCH_LENGTH, BENCH_ZC, CUP, PAD, backFace, seatTop } from "./r3-seated-reader-ref.ts";
import { local } from "./r3-seated-reader-frame.ts";

export const TONE = { skin: 0.76, hair: 0.22, knit: 0.62, rib: 0.56, denim: 0.34, canvas: 0.8, rubber: 0.92, lace: 0.95, timber: 0.5, bolt: 0.3, paper: 0.93, board: 0.4, wire: 0.25, cup: 0.9, lid: 0.3, sleeve: 0.55, dark: 0.1 } as const;
const SIDES = [["l", 1], ["r", -1]] as const;
const FRAMES = [["l", "frame-l"], ["r", "frame-r"]] as const;

/* ---------------------------------------------------------------- bench */

/** 📐️ A side profile stated in world `[x, y]` as an extrude profile in the frame of a frame bone. */
export const inFrame = (bone: string, points: readonly P2[]): P2[] => points.map(([x, y]) => { const p = local(bone, [x, y, 0]); return [p[0], p[1]] as P2; });
/** 🪵️ The cranked rear post (60 x 45 mm): leg kicked back to the foot, back post leaning 8 degrees, its front face carrying the back slats. */
export const POST: P2[] = [[-0.185, 0], [BENCH.backX, BENCH.seatRear[1]], [backFace(0.842), 0.842], [backFace(0.842) - 0.008, BENCH.top], [backFace(BENCH.top) - 0.06, BENCH.top], [BENCH.backX - 0.06, BENCH.seatRear[1]], [-0.245, 0]];
/** 🪵️ The front leg (70 x 45 mm), cut on the rake under the slats. */
export const FRONT_LEG: P2[] = [[0.18, 0], [0.25, 0], [0.25, seatTop(0.25) - 0.04], [0.18, seatTop(0.18) - 0.04]];
/** 🪵️ The seat rail (80 x 45 mm) under the slats from the post to the front leg. */
export const SEAT_RAIL: P2[] = [[-0.2, seatTop(-0.2) - 0.12], [0.25, seatTop(0.25) - 0.12], [0.25, seatTop(0.25) - 0.04], [-0.2, seatTop(-0.2) - 0.04]];
/** 🪵️ The lower stretcher (60 x 35 mm) between the legs of an end frame. */
export const STRETCHER: P2[] = [[-0.215, 0.14], [0.2, 0.14], [0.2, 0.2], [-0.21, 0.2]];
export const SEAT_SLATS = [0, 1, 2, 3].map((i) => BENCH.slat[0] / 2 + i * (BENCH.slat[0] + BENCH.gap));
export const BACK_SLATS = [0, 1, 2].map((i) => BENCH.backSlat[0] / 2 + i * (BENCH.backSlat[0] + BENCH.backGap));

/** 🪑️ Park bench: two end frames (cranked rear post, front leg, seat rail, lower stretcher, all 45 mm timber in one plane with housed joints), a long stretcher between them, four seat slats and three back slats with eased edges, one carriage bolt per slat and frame. */
export function bench(k: Kit): void {
  const t = BENCH.frameT / 2;
  for (const [s, bone] of FRAMES) {
    k.ext(`bench-post-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, POST), -t, t, "bench");
    k.ext(`bench-leg-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, FRONT_LEG), -t, t, "bench");
    k.ext(`bench-rail-${s}`, bone, "wood-bench", TONE.timber - 0.1, "xy", inFrame(bone, SEAT_RAIL), -t + 0.0025, t - 0.0025, "bench");
    k.ext(`bench-stretcher-${s}`, bone, "wood-bench", TONE.timber - 0.1, "xy", inFrame(bone, STRETCHER), -0.0175, 0.0175, "bench");
  }
  k.ext("bench-tie", "frame-l", "wood-bench", TONE.timber - 0.12, "xy", inFrame("frame-l", [[-0.03, 0.14], [0.005, 0.14], [0.005, 0.2], [-0.03, 0.2]]), BENCH.frameZ[1] - BENCH.frameZ[0] + 0.0175, -0.0175, "bench");
  SEAT_SLATS.forEach((u, i) => k.box(`bench-seat-slat-${i + 1}`, "seat", "wood-bench", TONE.timber, [u, -BENCH.slat[1] / 2, BENCH_ZC], [BENCH.slat[0], BENCH.slat[1], BENCH_LENGTH], 0.005, "bench"));
  BACK_SLATS.forEach((u, i) => k.box(`bench-back-slat-${i + 1}`, "backrest", "wood-bench", TONE.timber, [u, -BENCH.backSlat[1] / 2, BENCH_ZC], [BENCH.backSlat[0], BENCH.backSlat[1], BENCH_LENGTH], 0.005, "bench"));
  for (const [s] of FRAMES) {
    const z = BENCH.frameZ[s === "l" ? 0 : 1];
    SEAT_SLATS.forEach((u, i) => k.ell(`bench-bolt-seat-${i + 1}-${s}`, "seat", "steel-zinc", TONE.bolt, [u, 0.0005, z], [0.01, 0.0035, 0.01], "bench", { texture: true, role: "detail" }));
    BACK_SLATS.forEach((u, i) => k.ell(`bench-bolt-back-${i + 1}-${s}`, "backrest", "steel-zinc", TONE.bolt, [u, -BENCH.backSlat[1] - 0.0005, z], [0.01, 0.0035, 0.01], "bench", { texture: true, role: "detail" }));
  }
}

/* ---------------------------------------------------------------- cup and notebook */

/** 🥤️ Paper cup: tapered body, corrugated sleeve, plastic lid with a raised rim and a sip dome. */
export function cup(k: Kit): void {
  const r = (a: number): number => CUP.r0 + ((CUP.r1 - CUP.r0) * a) / CUP.height;
  k.rev("cup-body", "cup", "paper-cup", TONE.cup, [[0, 0], [0, CUP.r0 - 0.002], [0.003, CUP.r0], [CUP.height, CUP.r1], [CUP.height, 0]], [1, 0, 0], [0, 0, 0], "cup");
  k.rev("cup-sleeve", "cup", "paper-cup", TONE.sleeve, [[0.035, r(0.035)], [0.035, r(0.035) + 0.002], [0.085, r(0.085) + 0.002], [0.085, r(0.085)], [0.035, r(0.035)]], [1, 0, 0], [0, 0, 0], "cup");
  k.rev("cup-lid", "cup", "paper-cup", TONE.lid, [[CUP.height - 0.004, 0], [CUP.height - 0.004, CUP.r1 + 0.0025], [CUP.height + 0.003, CUP.r1 + 0.0025], [CUP.height + 0.006, CUP.r1 - 0.002], [CUP.height + CUP.lid, CUP.r1 - 0.006], [CUP.height + CUP.lid, CUP.r1 - 0.012], [CUP.height + 0.006, CUP.r1 - 0.016], [CUP.height + 0.006, 0]], [1, 0, 0], [0, 0, 0], "cup");
}

/** 📓️ Notebook: paper block on a grey board back, wire comb along the top edge, the sheet that turns. The bone runs in the middle plane from the lower edge to the binding; the reading face looks to -v. */
export function notebook(k: Kit): void {
  k.box("pad-board", "pad", "paper", TONE.paper, [PAD.length / 2, 0, 0], [PAD.length, PAD.thick, PAD.width], 0.001, "notebook");
  k.box("pad-back", "pad", "paper", TONE.board, [PAD.length / 2, PAD.thick / 2 + 0.0004, 0], [PAD.length, 0.0012, PAD.width], 0.0004, "notebook", { texture: true, role: "texture" });
  k.cap("pad-binding", "pad", "steel-zinc", TONE.wire, [PAD.length - 0.004, -0.001, -PAD.width / 2 + 0.008], [PAD.length - 0.004, -0.001, PAD.width / 2 - 0.008], 0.0075, 0.0075, "notebook");
  k.box("page-sheet", "page", "paper", TONE.paper + 0.04, [PAD.page / 2 + 0.004, PAD.thick / 2 + 0.0008, 0], [PAD.page - 0.006, 0.0008, PAD.width - 0.004], 0.0002, "notebook", { texture: true, role: "texture" });
}

/* ---------------------------------------------------------------- trunk, neck and head */

export const SEAT: Row[] = [[-0.088, 0.024, 0, 0.125, 0.062, 3], [-0.079, 0.024, 0, 0.166, 0.086, 2.8], [-0.055, 0.027, 0, 0.194, 0.097, 2.6], [-0.02, 0.021, 0, 0.203, 0.108, 2.5], [0.03, 0.012, 0, 0.192, 0.112, 2.4], [0.075, 0.0, 0, 0.168, 0.108, 2.4], [0.11, -0.012, 0, 0.15, 0.1, 2.4], [0.128, -0.014, 0, 0.144, 0.096, 2.4]];
export const SWEATER: Row[] = [[-0.02, -0.024, 0, 0.19, 0.124, 2.5], [0.04, -0.032, 0, 0.17, 0.116, 2.4], [0.1, -0.04, 0, 0.153, 0.11, 2.4], [0.16, -0.04, 0, 0.15, 0.113, 2.4], [0.23, -0.04, 0, 0.154, 0.124, 2.4], [0.29, -0.042, 0, 0.16, 0.134, 2.4], [0.34, -0.034, 0, 0.167, 0.128, 2.5], [0.385, -0.024, 0, 0.178, 0.112, 2.7], [0.42, -0.018, 0, 0.187, 0.1, 2.7], [0.444, -0.01, 0, 0.178, 0.086, 2.6], [0.462, -0.002, 0, 0.142, 0.07, 2.4], [0.477, 0.004, 0, 0.095, 0.06, 2.2]];
export const HEM: Row[] = [[-0.062, -0.02, 0, 0.186, 0.116, 2.5], [-0.054, -0.02, 0, 0.199, 0.128, 2.5], [-0.024, -0.023, 0, 0.196, 0.127, 2.5], [-0.014, -0.024, 0, 0.189, 0.123, 2.5]];
export const COWL: Row[] = [[-0.05, 0.004, 0, 0.092, 0.078, 2.2], [-0.03, -0.002, 0, 0.1, 0.09, 2.2], [0.0, -0.007, 0, 0.097, 0.09, 2.1], [0.026, -0.01, 0, 0.086, 0.082, 2.1], [0.042, -0.011, 0, 0.07, 0.07, 2.1], [0.048, -0.011, 0, 0.058, 0.06, 2.1]];
export const NECK: Row[] = [[-0.02, -0.006, 0, 0.054, 0.056], [0.03, -0.01, 0, 0.049, 0.052], [0.075, -0.012, 0, 0.047, 0.051], [0.118, -0.008, 0, 0.048, 0.054]];
export const HEAD: Row[] = [[-0.094, -0.077, 0, 0.011, 0.009, 2], [-0.0902, -0.073, 0, 0.023, 0.018, 2.2], [-0.0818, -0.064, 0, 0.037, 0.03, 2.3], [-0.066, -0.045, 0, 0.049, 0.05, 2.3], [-0.0437, -0.028, 0, 0.057, 0.069, 2.2], [-0.0158, -0.011, 0, 0.065, 0.086, 2.1], [0.019, -0.003, 0, 0.07, 0.0935, 2.1], [0.051, 0.0, 0, 0.072, 0.0925, 2.1], [0.082, 0.003, 0, 0.066, 0.084, 2.1], [0.105, 0.006, 0, 0.051, 0.064, 2], [0.118, 0.0075, 0, 0.03, 0.037, 2], [0.1225, 0.0075, 0, 0.011, 0.014, 2]];
export const HAIR: Row[] = [[-0.055, 0.03, 0, 0.03, 0.012, 2.2], [-0.03, 0.038, 0, 0.045, 0.025, 2.2], [-0.005, 0.03, 0, 0.062, 0.057, 2.2], [0.025, 0.016, 0, 0.072, 0.082, 2.1], [0.05, 0.008, 0, 0.0765, 0.095, 2.1], [0.075, 0.006, 0, 0.0725, 0.092, 2.1], [0.095, 0.006, 0, 0.063, 0.08, 2.1], [0.112, 0.007, 0, 0.048, 0.06, 2], [0.124, 0.0075, 0, 0.024, 0.03, 2], [0.1295, 0.0075, 0, 0.008, 0.01, 2]];
export const BUN = { at: [0.112, 0.068, 0] as V3, radii: [0.038, 0.043, 0.045] as V3 };

export function trunk(k: Kit): void {
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, SEAT, "jeans");
  k.loft("sweater", "torso", "knit-wool", TONE.knit, SWEATER, "sweater");
  k.loft("sweater-hem", "torso", "knit-wool", TONE.rib, HEM, "sweater");
  k.loft("cowl", "neck", "knit-wool", TONE.knit + 0.04, COWL, "sweater");
}

export function head(k: Kit): void {
  k.loft("neck", "neck", "skin", TONE.skin - 0.04, NECK, "face-skin");
  k.loft("head", "head", "skin", TONE.skin, HEAD, "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, HAIR, "hair");
  k.ell("hair-bun", "head", "hair", TONE.hair - 0.04, BUN.at, BUN.radii, "hair");
  k.loft("nose", "head", "skin", TONE.skin, [[0.0206, -0.0935, 0, 0.0062, 0.0055, 2], [0, -0.1025, 0, 0.0085, 0.008, 2], [-0.0206, -0.108, 0, 0.0122, 0.0105, 2], [-0.0318, -0.1025, 0, 0.015, 0.0115, 2], [-0.0374, -0.0935, 0, 0.013, 0.009, 2]], "face-skin");
  k.ell("mouth", "head", "skin", TONE.skin - 0.26, [-0.056, -0.0925, 0], [0.004, 0.005, 0.02], "face-skin", { role: "detail" });
  for (const [s, side] of SIDES) {
    k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.002, 0.011, side * 0.069], [0.029, 0.017, 0.008], "face-skin", { rotation: quat(aboutU(side * -12), aboutW(-14)) });
    k.ell(`eye-${s}`, "head", "skin", TONE.dark + 0.08, [0.017, -0.0815, side * 0.03], [0.005, 0.0055, 0.0115], "face-skin", { role: "detail" });
    k.bar(`brow-${s}`, "head", "hair", TONE.hair, [[0.032, -0.088, side * 0.012, 0.004], [0.0355, -0.086, side * 0.031, 0.0042], [0.031, -0.0775, side * 0.049, 0.0038]], "hair", { role: "detail" });
  }
}

/* ---------------------------------------------------------------- limbs */

export const SHOULDER_CAP: Row[] = [[-0.046, 0, -0.028, 0.012, 0.026], [-0.04, 0, -0.02, 0.026, 0.038], [-0.03, 0, -0.011, 0.038, 0.047], [-0.017, 0, -0.004, 0.046, 0.052], [-0.004, 0, 0.0, 0.05, 0.054]];
export const UPPER_ARM: Row[] = [...SHOULDER_CAP, [0.012, 0, 0.002, 0.052, 0.055], [0.05, 0.002, 0.004, 0.05, 0.052], [0.12, 0.004, 0.002, 0.046, 0.049], [0.2, 0.004, 0, 0.043, 0.046], [0.26, 0.002, 0, 0.041, 0.043], [0.3115, 0, 0, 0.041, 0.041], ...dome(0.352, 0.3115, 0, 0, 0.041, 0.041).reverse()];
export const FOREARM: Row[] = [...dome(-0.038, -0.004, 0, 0, 0.034, 0.033), [0, 0, 0, 0.041, 0.04], [0.05, 0.002, 0, 0.043, 0.041], [0.12, 0.001, 0, 0.038, 0.035], [0.19, 0, 0, 0.034, 0.03], [0.208, 0, 0, 0.033, 0.029]];
export const CUFF: Row[] = [[0.198, 0, 0, 0.033, 0.029, 2.4], [0.203, 0, 0, 0.036, 0.032, 2.4], [0.236, 0, 0, 0.0345, 0.0305, 2.4], [0.241, 0, 0, 0.031, 0.027, 2.4]];
export const WRIST: Row[] = [[0.232, 0, 0, 0.0275, 0.0195], [0.246, 0, 0, 0.0265, 0.0185], [0.258, 0, 0, 0.026, 0.018]];
export const THIGH: Row[] = [...dome(-0.065, 0, 0, 0.008, 0.092, 0.082, 5, 2.2), [0, -0.004, 0.008, 0.097, 0.086, 2.2], [0.08, -0.004, 0.006, 0.092, 0.083, 2.2], [0.17, -0.001, 0.003, 0.083, 0.076, 2.2], [0.26, 0.003, 0, 0.072, 0.068, 2.2], [0.34, 0.006, 0, 0.063, 0.062, 2.2], [0.395, 0.009, 0, 0.058, 0.06, 2.2], [0.43, 0.009, 0, 0.057, 0.06, 2.2], ...dome(0.476, 0.43, 0.009, 0, 0.057, 0.06, 5, 2.2).reverse()];
export const SHANK: Row[] = [...dome(-0.04, 0, 0.003, 0, 0.047, 0.051), [0, 0.003, 0, 0.05, 0.055], [0.05, 0.0, 0, 0.055, 0.061], [0.13, -0.006, 0, 0.058, 0.064], [0.23, -0.004, 0, 0.055, 0.06], [0.32, 0.0, 0, 0.053, 0.057], [0.385, 0.002, 0, 0.053, 0.057], [0.398, 0.002, 0, 0.05, 0.054]];
/** 👟️ The sneaker in a level frame at the ankle (s ahead, cv up; floor at cv -0.078): padded collar, quarters, tongue and vamp to the ball, where the toe bone takes over. */
export const SHOE: Row[] = [[-0.068, -0.04, 0, 0.022, 0.03, 2.6], [-0.062, -0.038, 0, 0.032, 0.04, 2.6], [-0.04, -0.032, 0, 0.038, 0.048, 2.6], [-0.008, -0.032, 0, 0.04, 0.048, 2.6], [0.03, -0.042, 0, 0.042, 0.036, 2.6], [0.07, -0.05, 0, 0.045, 0.027, 2.6], [0.11, -0.055, 0, 0.047, 0.022, 2.6], [0.14, -0.057, 0, 0.047, 0.02, 2.6]];
export const SOLE: Row[] = [[-0.071, -0.0685, 0, 0.03, 0.0095, 4], [-0.061, -0.0685, 0, 0.038, 0.0095, 4], [0.0, -0.0685, 0, 0.042, 0.0095, 4], [0.07, -0.0685, 0, 0.0465, 0.0095, 4], [0.14, -0.0685, 0, 0.049, 0.0095, 4]];
export const TOE: Row[] = [[-0.012, 0.031, 0, 0.047, 0.02, 2.6], [0.025, 0.029, 0, 0.045, 0.019, 2.6], [0.048, 0.027, 0, 0.039, 0.017, 2.6], [0.061, 0.025, 0, 0.028, 0.013, 2.4], [0.068, 0.024, 0, 0.015, 0.009, 2.2]];
export const TOE_SOLE: Row[] = [[-0.012, 0.0095, 0, 0.049, 0.0095, 4], [0.03, 0.0095, 0, 0.047, 0.0095, 4], [0.055, 0.0105, 0, 0.038, 0.0095, 4], [0.07, 0.0115, 0, 0.021, 0.009, 3]];
export const FOOT_DROP = (Math.atan2(0.078, 0.13) * 180) / Math.PI;

/** ✋️ One finger as a bent bar in the hand frame (u to the finger tips, v out of the back of the hand, w across), flexing towards the palm (-v): knuckle, joints and tip from the flexion of the three joints in degrees. */
export function fingerPath(w: number, scale: number, r: number, flex: readonly [number, number, number], knuckle = 0.09): number[][] {
  const lengths = [0.042, 0.025, 0.021].map((l) => l * scale);
  let [u, p, angle] = [knuckle, -0.002, 0];
  const out: number[][] = [[u - 0.013, p + 0.001, w * 0.9, r * 1.08], [u, p, w, r * 1.1]];
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
/** 🤲️ The open carrying hand: finger flexion (index to little finger, nearly straight under the notebook, spreading a little) and the thumb path `[u, v, towards the thumb side, radius]` that comes round the edge of the notebook onto its reading face. */
export const CARRY = { flex: [[-3, 5, 4], [-3, 4, 4], [-3, 5, 4], [-2, 6, 5]], thumb: [[0.016, -0.004, 0.024, 0.0115], [0.04, -0.008, 0.043, 0.0105], [0.06, -0.022, 0.05, 0.0095], [0.076, -0.036, 0.044, 0.0085], [0.088, -0.038, 0.034, 0.0078]] } as const;
export const FINGERS: readonly (readonly [w: number, scale: number, r: number])[] = [[0.027, 0.94, 0.0082], [0.009, 1, 0.0084], [-0.0098, 0.95, 0.008], [-0.027, 0.8, 0.0071]];

/** 🖐️ A hand, palm to -v, thumb on -w of the left and +w of the right hand. */
export function hand(k: Kit, s: "l" | "r"): void {
  const t = s === "l" ? -1 : 1;
  const bone = `hand-${s}`;
  k.loft(`palm-${s}`, bone, "skin", TONE.skin, [[-0.006, 0, 0, 0.027, 0.0175, 2.4], [0.028, -0.001, t * 0.003, 0.036, 0.0152, 2.6], [0.062, -0.002, t * 0.001, 0.0405, 0.0132, 2.8], [0.086, -0.002, -t * 0.001, 0.0395, 0.0115, 2.8], [0.096, -0.002, -t * 0.001, 0.037, 0.0085, 2.5]], "hand-skin");
  k.ell(`thenar-${s}`, bone, "skin", TONE.skin, [0.034, -0.007, t * 0.026], [0.028, 0.011, 0.016], "hand-skin");
  FINGERS.forEach(([w, sc, r], i) => k.bar(`finger-${i + 1}-${s}`, bone, "skin", TONE.skin - 0.02 * (i % 2), fingerPath(t * w, sc, r, CARRY.flex[i] as [number, number, number]), "hand-skin"));
  k.bar(`thumb-${s}`, bone, "skin", TONE.skin, CARRY.thumb.map(([u, p, w, r]) => [u, p, t * w, r]), "hand-skin");
}

export function limbs(k: Kit): void {
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "knit-wool", TONE.knit - 0.03, UPPER_ARM.map(([a, cv, cw, rx, ry]) => [a, cv, side * cw, rx, ry] as Row), "sweater");
    k.loft(`forearm-${s}`, `forearm-${s}`, "knit-wool", TONE.knit - 0.03, FOREARM, "sweater");
    k.loft(`cuff-${s}`, `forearm-${s}`, "knit-wool", TONE.rib, CUFF, "sweater");
    k.loft(`wrist-${s}`, `forearm-${s}`, "skin", TONE.skin, WRIST, "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, THIGH.map(([a, cv, cw, rx, ry, e]) => (e === undefined ? [a, cv, side * cw, rx, ry] : [a, cv, side * cw, rx, ry, e]) as Row), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.02, SHANK, "jeans");
    const level = { rotation: aboutW(FOOT_DROP) };
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, SHOE, "shoes", level);
    k.loft(`shoe-sole-${s}`, `foot-${s}`, "rubber-sole", TONE.rubber, SOLE, "shoes", level);
    k.loft(`shoe-toe-${s}`, `toe-${s}`, "canvas-sneaker", TONE.canvas, TOE, "shoes");
    k.loft(`shoe-toe-sole-${s}`, `toe-${s}`, "rubber-sole", TONE.rubber, TOE_SOLE, "shoes");
    [0.03, 0.055, 0.08, 0.105].forEach((a, i) => k.bar(`shoe-lace-${i + 1}-${s}`, `foot-${s}`, "canvas-sneaker", TONE.lace, [[a, -0.008 - 0.34 * (a - 0.03), -0.017, 0.0028], [a + 0.004, -0.0035 - 0.34 * (a - 0.03), 0, 0.0028], [a, -0.008 - 0.34 * (a - 0.03), 0.017, 0.0028]], "shoes", { ...level, texture: true, role: "detail" }));
    hand(k, s);
  }
}

/** 🧱️ The whole 1:50 body. */
export function body50(k: Kit): void {
  bench(k);
  cup(k);
  notebook(k);
  trunk(k);
  head(k);
  limbs(k);
}
