/** 📏️ Reference dimensions of the round 3 seated office worker in metres, world frame (x ahead, y up, z to the left): a man of 1.78 m and 76 kg in leather shoes, a task chair after EN 1335-1 on a five-star base with twin-wheel castors after EN 12529, a compact desk of 780 x 600 x 720 mm on four steel tube legs after EN 527-1 and a 15.6 inch notebook. Sources are named in `r3-humans-seated-office-worker.md`.
 * @see ./r3-seated-office-worker-roots.ts
 * @see ./r3-seated-office-worker-s50.ts */
export type V3 = readonly [number, number, number];

/** 🪑️ Task chair: column axis, hub, star arms, castors, gas lift, seat, backrest and armrests. */
export const CHAIR = {
  columnX: 0.04,
  hubY: 0.1,
  armReach: 0.3,
  armTipY: 0.078,
  castorTrail: 0.02,
  castorDrop: 0.053,
  wheelR: 0.025,
  seatY: 0.4,
  seatTop: 0.485,
  seatHalfDepth: 0.235,
  seatHalfWidth: 0.245,
  backPivot: [-0.21, 0.0] as readonly [number, number],
  backLength: 0.6,
  armrestTop: 0.68,
  armrestZ: 0.265,
} as const;
/** 🧭️ The five star arms: id and the direction of the arm seen from above, degrees from ahead towards the left. */
export const STAR: readonly (readonly [id: string, yawDeg: number])[] = [["front", 0], ["side-a", 72], ["back-a", 144], ["back-b", -144], ["side-b", -72]];

/** 🧍️ The sitter: hip joint above the seat bone, joint spacings, ankle height in the shoe, ball of the foot ahead of the ankle, segment lengths (Winter 2009 shares of 1.78 m). */
export const MAN = { hipAboveSeatY: 0.17, hipZ: 0.088, shoulderZ: 0.176, ankleY: 0.092, ballAhead: 0.14, thigh: 0.435, shank: 0.438, torso: 0.505, neck: 0.118, head: 0.132, shoulderOnTorso: 0.432, shoulderAhead: 0.042 } as const;

/** 🗄️ Desk: front edge, depth ahead, width across, top surface, board thickness, square leg tube, rail height and width, inset of the legs from the board edges. */
export const DESK = { x0: 0.28, depth: 0.78, width: 0.6, top: 0.72, board: 0.025, leg: 0.035, railH: 0.06, railW: 0.02, inset: 0.035 } as const;

/** 💻️ Notebook (15.6 inch business class): hinge axis, depth of base and lid, width, thickness of base, feet and lid, opening angle of the lid from the base. */
export const LAPTOP = { hingeX: 0.665, hingeY: 0.7335, depth: 0.235, lidLength: 0.226, width: 0.357, base: 0.0175, feet: 0.0015, lid: 0.0065, openDeg: 110 } as const;
