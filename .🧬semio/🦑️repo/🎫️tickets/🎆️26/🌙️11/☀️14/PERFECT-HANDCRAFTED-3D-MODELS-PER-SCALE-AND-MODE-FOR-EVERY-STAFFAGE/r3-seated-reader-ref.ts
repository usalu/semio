/** 📏️ Reference dimensions of the round 3 seated reader in metres, world frame (x ahead, y up, z to the left): a woman of 1.68 m and 61 kg in a thick sweater, straight jeans and canvas sneakers, a 1.20 m painted timber park bench of two end frames and bolted slats, an A5 top-bound notebook and a 0.3 l paper cup. Sources are named in `r3-humans-seated-reader.md`.
 * @see ./r3-seated-reader-roots.ts
 * @see ./r3-seated-reader-s50.ts */
export type V3 = readonly [number, number, number];

const RAKE = (5 * Math.PI) / 180;
const LEAN = (8 * Math.PI) / 180;
/** 🪑️ Park bench: seat of four 100 x 40 mm slats with 10 mm gaps raked 5 degrees (front edge 0.44 m high), three 80 x 30 mm back slats on posts leaning 8 degrees, end frames of 45 mm timber, the frames 0.13 m in from the slat ends, the sitter 0.44 m from the left end. */
export const BENCH = {
  rakeDeg: 5,
  leanDeg: 8,
  seatRear: [-0.155, 0.4025] as readonly [number, number],
  seatDepth: 0.43,
  slat: [0.1, 0.04] as readonly [number, number],
  gap: 0.01,
  backX: -0.165,
  backFrom: 0.56,
  backLength: 0.29,
  backSlat: [0.08, 0.03] as readonly [number, number],
  backGap: 0.025,
  top: 0.85,
  z0: -0.76,
  z1: 0.44,
  frameZ: [0.31, -0.63] as readonly [number, number],
  frameT: 0.045,
  footX: -0.215,
  frontLegX: 0.215,
} as const;
/** 📐️ Height of the seat surface at `x`. */
export const seatTop = (x: number): number => BENCH.seatRear[1] + (x - BENCH.seatRear[0]) * Math.tan(RAKE);
/** 📐️ The front face of the back posts (the plane the back slats are bolted to) at height `y`. */
export const backFace = (y: number): number => BENCH.backX - (y - BENCH.seatRear[1]) * Math.tan(LEAN);
export const BENCH_ZC = (BENCH.z0 + BENCH.z1) / 2;
export const BENCH_LENGTH = BENCH.z1 - BENCH.z0;

/** 🧍‍♀️ The sitter: hip joint 90 mm above the timber under the seat bones, joint spacings, ankle joint in a sneaker with a 12 mm sole, ball of the foot ahead of the ankle, segment lengths (Winter 2009 shares of 1.68 m). */
export const WOMAN = { hipAboveSeat: 0.09, hipZ: 0.085, shoulderZ: 0.165, ankleY: 0.078, ballAhead: 0.13, thigh: 0.4106, shank: 0.413, pelvis: 0.0944, torso: 0.477, neck: 0.111, head: 0.125, shoulderOnTorso: 0.406, shoulderAhead: 0.04, arm: 0.3115, forearm: 0.2454, hand: 0.1794 } as const;
export const HIP_Y = Math.round((seatTop(-0.02) + WOMAN.hipAboveSeat) * 1000) / 1000;

const TURN = (22 * Math.PI) / 180;
/** 📓️ Notebook (A5, top-bound with a wire comb): length along the bone, width, thickness of the block with its board, the turning sheet. It lies on the open left hand, whose fingers point `turnDeg` inwards under it: the middle of the palm (0.06 m from the wrist) carries the point 0.07 m up the notebook and 0.04 m left of its centre line; `inHand` is the lower edge of the notebook in the frame of the left hand (the palm faces -v). */
export const PAD = { length: 0.21, width: 0.148, thick: 0.012, page: 0.205, turnDeg: 22, palm: 0.06, up: 0.07, off: 0.04, below: 0.022, inHand: [0.06 - 0.07 * Math.cos(TURN) + 0.04 * Math.sin(TURN), -0.022, 0.07 * Math.sin(TURN) + 0.04 * Math.cos(TURN)].map((x) => Math.round(x * 1e4) / 1e4) as unknown as V3 } as const;
/** 🥤️ Paper cup (0.3 l, 12 oz): base and rim radius, height, lid; it stands on the seat `alongSeat` from the rear edge, `z` to the right of the sitter. */
export const CUP = { r0: 0.03, r1: 0.044, height: 0.108, lid: 0.012, alongSeat: 0.07, z: -0.34 } as const;
