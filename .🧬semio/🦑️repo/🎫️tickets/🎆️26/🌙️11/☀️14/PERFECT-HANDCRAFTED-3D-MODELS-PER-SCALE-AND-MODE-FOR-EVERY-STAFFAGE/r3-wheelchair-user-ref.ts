/** 📏️ Reference numbers of the round 3 wheelchair user (world coordinates of the rest pose: x ahead, y up, z to the left, metres; the rear axle is the origin of the `frame` bone at (0, 0.305)): a 1.75 m, 75 kg man (segment shares after Winter 2009 and Drillis-Contini, girths after ANSUR II) in a rigid-frame active wheelchair (24 inch pneumatic rear wheels 37-540 on 540 rims with 530 mm hand rims, 135 mm castors, 0 degree camber, 420 mm seat width and 430 mm depth, ISO 7176-7 measures).
 * @see ./r3-humans-wheelchair-user.md */
export type V3 = readonly [number, number, number];

export const H = 1.75;
export const AXLE: V3 = [0, 0.305, 0];

/** 🦽️ The chair: every value a world position or size in metres. */
export const CHAIR = {
  wheelR: 0.305,
  tyre: { centre: 0.2875, half: 0.0175, halfWidth: 0.0168 },
  rim: { inner: 0.249, outer: 0.2735, halfWidth: 0.0115 },
  wheelZ: 0.28,
  handRim: { r: 0.265, tube: 0.0095, z: 0.31 },
  hub: { r: 0.019, flange: 0.029, inner: 0.236, outer: 0.305, flangeIn: 0.25, flangeOut: 0.298 },
  spokes: 24,
  spokeR: 0.001,
  sideZ: 0.215,
  tube: 0.0127,
  seatRail: [[-0.066, 0.452], [0.33, 0.488]] as const,
  backPost: [[-0.068, 0.47], [-0.088, 0.86]] as const,
  pushGrip: [[-0.124, 0.884], [-0.205, 0.896]] as const,
  hanger: [[0.395, 0.462], [0.478, 0.118]] as const,
  lowerTube: [[-0.045, 0.33], [0.455, 0.175]] as const,
  castorAxle: [0.44, 0.0678] as const,
  castorR: 0.0678,
  castorZ: 0.215,
  stemX: 0.476,
  housing: [0.17, 0.252] as const,
  footplate: { x0: 0.49, x1: 0.715, top: 0.104, t: 0.008, half: 0.165 },
  cushion: { x0: -0.06, x1: 0.37, rear: 0.512, front: 0.548, half: 0.205 },
  sling: { bottom: 0.53, top: 0.86 },
} as const;

/** 🧍️ The man: joint positions in the rest pose and the segment lengths. */
export const MAN = {
  hip: [0.06, 0.6] as const,
  hipZ: 0.0875,
  pelvis: 0.098,
  lumbosacralBehind: 0.04,
  torsoLean: 12,
  torso: 0.495,
  shoulderOnTorso: 0.43,
  shoulderAhead: 0.035,
  shoulderZ: 0.176,
  neck: 0.116,
  neckAngle: -20,
  head: 0.13,
  headAngle: 10,
  upperArm: 0.3234,
  forearm: 0.2548,
  hand: 0.1862,
  thigh: 0.4263,
  shank: 0.43,
  ankleY: 0.186,
  ballAhead: 0.142,
  knee: [0.4863, 0.606] as const,
} as const;

/** 👄️ Emitter anchors on the head bone (u up from the atlanto-occipital joint, v backwards): lips, nostril openings under the nose tip, crown of the beanie. */
export const HEAD_ANCHORS = {
  mouth: { x: -0.061, y: -0.106 },
  nostrils: { x: -0.035, y: -0.108 },
  crown: { x: 0.142, y: 0.01 },
} as const;
