/** 📐 Shared data of the round 3 model of `humans/person-cooking`: a woman of 1.69 m barefoot and 58 kg in canvas sneakers (24 mm under the heel, 16 mm under the ball) at a 0.60 m hob base unit with a 0.90 m worktop. Landmark heights after Winter 2009 (table 4.1) and de Leva 1996 as shares of the barefoot stature, raised by the lift of the sneaker at the ankle; segment lengths of thigh, shank and arm kept from the asset (de Leva female fractions). The kitchen unit after the 600 mm grid of DIN EN 1116 and a 580 x 510 mm four-zone glass-ceramic hob.
 * @see ./r3-person-cooking-roots.ts
 * @see ./r3-humans-person-cooking.md */
const H = 1.69;
const HEEL = 0.024;
const BALL = 0.016;
const ANKLE_BARE = 0.039 * H;
const BALL_X = 0.125;
/** 👟 The ankle centre over the sneaker sole: the heel contact lies 0.04 m behind the ankle, the ball 0.125 m ahead of it, so the ankle rises by the sole height interpolated between the two. */
const LIFT = BALL + (HEEL - BALL) * (BALL_X / (BALL_X + 0.04));
const ANKLE = ANKLE_BARE + LIFT;
const THIGH = 0.41325;
const SHANK = 0.40375;
const HIP = ANKLE + SHANK + THIGH;
const PELVIS_LEN = 0.0951;
const LUMBO_BACK = 0.03;
const C7: [number, number] = [-0.02, 0.855 * H + LIFT];
const AO: [number, number] = [0.012, H + LIFT - 0.145];
const SHOULDER: [number, number] = [0.0, 0.818 * H + LIFT];
const deg = (r: number): number => (r * 180) / Math.PI;
const torsoVec: [number, number] = [C7[0] + LUMBO_BACK, C7[1] - (HIP + PELVIS_LEN)];
const neckVec: [number, number] = [AO[0] - C7[0], AO[1] - C7[1]];
const torsoWorld = deg(Math.atan2(torsoVec[1], torsoVec[0]));
const neckWorld = deg(Math.atan2(neckVec[1], neckVec[0]));

export const RIG = {
  stature: H,
  heel: HEEL,
  ball: BALL,
  lift: LIFT,
  ankle: ANKLE,
  ballX: BALL_X,
  thigh: THIGH,
  shank: SHANK,
  hip: HIP,
  pelvisLen: PELVIS_LEN,
  lumboBack: LUMBO_BACK,
  c7: C7,
  ao: AO,
  shoulder: SHOULDER,
  shoulderZ: 0.155,
  torsoLen: Math.hypot(...torsoVec),
  torsoRel: torsoWorld - 90,
  neckLen: Math.hypot(...neckVec),
  neckRel: neckWorld - torsoWorld,
  headLen: 0.145,
  headRel: 90 - neckWorld,
  footLen: BALL_X,
  footRel: 90,
  toeLen: 0.07,
  hipZ: 0.0875,
  upperArm: 0.3135,
  forearm: 0.247,
  hand: 0.1805,
} as const;

/** 🍳 The kitchen unit in the frame of the `counter` bone (x into the unit from the front edge of the worktop, y up from the floor, z left): a 600 mm hob base unit with three drawer fronts on a recessed plinth, a 38 mm worktop and a 580 x 510 mm glass-ceramic hob 6.5 mm proud of it. */
export const UNIT = {
  ahead: 0.25,
  width: 0.6,
  depth: 0.6,
  plinth: 0.142,
  plinthBack: 0.07,
  carcass: 0.72,
  front: 0.02,
  door: 0.019,
  worktop: 0.038,
  top: 0.9,
  hobX0: 0.045,
  hobDepth: 0.51,
  hobWidth: 0.58,
  glass: 0.0065,
  zoneFront: 0.19,
  zoneRear: 0.41,
  zoneZ: 0.14,
} as const;
/** 🍲 The pot: 22 cm stainless-steel stockpot, 0.13 m high, 1 mm wall, 6 mm sandwich base, on the front right zone, with 2.5 litres of water (68 mm deep). The `pot` bone has its origin at the centre of the base on the glass. */
export const POT = { x: UNIT.zoneFront, y: UNIT.top + UNIT.glass, z: -UNIT.zoneZ, r: 0.11, h: 0.13, wall: 0.001, base: 0.006, water: 0.074 } as const;
/** 🥄 The cooking spoon of beech: 0.30 m long, the grip 0.085 m from the handle end. The `spoon` bone starts at the grip in the right hand and runs to the tip of the bowl. */
export const SPOON = { back: 0.085, len: 0.215, bowl: 0.185, gripX: 0.09, gripY: 0.004, gripZ: 0.026, rest: -55 } as const;
