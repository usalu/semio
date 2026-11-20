/** 📐 Shared data of the round 3 model of `humans/pregnant-person`: a woman of 1.66 m barefoot in week 36, in leather ankle boots with a 25 mm block heel and a 12 mm forefoot sole. Landmark heights after Winter 2009 (table 4.1) and de Leva 1996 as shares of the barefoot stature, raised by the lift of the boot at the ankle; segment lengths of thigh, shank and arm kept from the asset (de Leva female fractions).
 * @see ./r3-pregnant-person-roots.ts
 * @see ./r3-humans-pregnant-person.md */
const H = 1.66;
const HEEL = 0.025;
const BALL = 0.012;
const ANKLE_BARE = 0.039 * H;
/** 👢 The ankle centre over a 25 mm heel and a 12 mm forefoot sole: the heel contact lies 0.04 m behind the ankle, the ball 0.128 m ahead of it, so the ankle rises by the sole height interpolated between the two. */
const BALL_X = 0.128;
const LIFT = BALL + (HEEL - BALL) * (BALL_X / (BALL_X + 0.04));
const ANKLE = ANKLE_BARE + LIFT;
const THIGH = 0.40673;
const SHANK = 0.39738;
const HIP = ANKLE + SHANK + THIGH;
const PELVIS_LEN = 0.09275;
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
  shoulderZ: 0.152,
  torsoLen: Math.hypot(...torsoVec),
  torsoRel: torsoWorld - 90,
  neckLen: Math.hypot(...neckVec),
  neckRel: neckWorld - torsoWorld,
  headLen: 0.145,
  headRel: 90 - neckWorld,
  footLen: BALL_X,
  footRel: 90,
  toeLen: 0.075,
  hipZ: 0.085,
  bellyAt: 0.045,
  bellyLen: 0.28,
  bellyCom: 0.15,
  upperArm: 0.3102,
  forearm: 0.2444,
  hand: 0.1786,
} as const;
