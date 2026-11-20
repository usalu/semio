/** 🎸️ Reference dimensions of the round 3 seated guitarist, stated once and read by the root fixes, the solids and the drawings: a dreadnought steel-string guitar (Martin D-18 / D-28 pattern: body 508 mm, lower bout 397 mm, waist 273 mm, upper bout 292 mm, depth 124 to 98 mm, scale 645 mm, nut 43 mm, neck joint at the 14th fret) and a 24 inch (610 mm) wooden stool with four splayed turned legs and a ring of rungs.
 * Guitar coordinates `[a, b, c]`: a along the strings from the tail block (0) to the head, b across the top (bass side +), c out of the top (top surface 0, back negative).
 * @see ./r3-humans-seated-guitarist.md */
export type V3 = [number, number, number];

export const GUITAR = {
  length: 1.03,
  body: 0.508,
  scale: 0.645,
  nut: 0.8657,
  saddle: 0.2207,
  soundhole: 0.372,
  soundholeR: 0.0508,
  fretboardEnd: 0.424,
  depthTail: 0.124,
  depthHeel: 0.098,
  pivot: [0.29, -0.1365, -0.055] as V3,
  massKg: 2.1,
  comA: 0.47,
} as const;

/** 🎼️ The distance of fret `n` from the tail along the strings (equal temperament). */
export const fretA = (n: number): number => GUITAR.nut - GUITAR.scale * (1 - 2 ** (-n / 12));

/** 🪵️ Half the width of the guitar body at `a` (outline of a dreadnought, stated at eleven stations). */
export const BOUT: readonly (readonly [a: number, half: number])[] = [[0, 0.085], [0.012, 0.13], [0.04, 0.17], [0.085, 0.1925], [0.14, 0.1985], [0.2, 0.185], [0.25, 0.156], [0.29, 0.1365], [0.33, 0.132], [0.38, 0.14], [0.43, 0.146], [0.468, 0.142], [0.494, 0.12], [0.508, 0.075]];
export function halfWidth(a: number): number {
  for (let i = 1; i < BOUT.length; i++) if (a <= BOUT[i]![0]) {
    const [a0, h0] = BOUT[i - 1]!;
    const [a1, h1] = BOUT[i]!;
    return h0 + ((h1 - h0) * (a - a0)) / (a1 - a0);
  }
  return BOUT[BOUT.length - 1]![1];
}
/** 📏️ The depth of the body (top to back) at `a`. */
export const depthAt = (a: number): number => GUITAR.depthTail + ((GUITAR.depthHeel - GUITAR.depthTail) * Math.min(Math.max(a, 0), GUITAR.body)) / GUITAR.body;

/** 🎯️ A point of the guitar `[a, b, c]` in the frame of the guitar bone (u along the strings from the pivot on the treble side of the waist, v towards the back, w towards the treble side). */
export const onGuitar = (a: number, b: number, c: number): V3 => [a - GUITAR.pivot[0], -(c - GUITAR.pivot[2]), -(b - GUITAR.pivot[1])];

export const STOOL = {
  seatTop: 0.61,
  seatR: 0.165,
  seatT: 0.038,
  legTop: 0.085,
  legFloor: 0.19,
  legR0: 0.019,
  legR1: 0.0155,
  rungR: 0.0115,
  rungFront: 0.25,
  rungSide: 0.33,
  massKg: 3.6,
} as const;
/** 🦵️ The offset of a leg axis from the stool axis (the same along x and along z) at height `y`. */
export const legAt = (y: number): number => STOOL.legFloor - ((STOOL.legFloor - STOOL.legTop) * y) / (STOOL.seatTop - STOOL.seatT);

export const HIP_ABOVE_SEAT = 0.09;
