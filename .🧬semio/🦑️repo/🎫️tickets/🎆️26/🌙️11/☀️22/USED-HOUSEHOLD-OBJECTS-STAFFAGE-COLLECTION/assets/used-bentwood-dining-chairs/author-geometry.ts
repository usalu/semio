/** 🪑️ Hand-chosen geometry of the used bentwood café chair (Viennese No. 14 / 214 type) in its own chair frame and the primitives the author script turns into solids and drawings.
 *
 * Chair frame: x ahead, y up from the floor, z to the chair's left, metres. Every number here is chosen by hand against the reference table of the report.
 *
 * @see ./author-bentwood.ts — the authoritative script that writes the asset document
 * @see ../../reports/used-bentwood-dining-chairs.md — reference table and measurements */

export type V3 = [number, number, number];
export type V2 = [number, number];

/** 🧵️ A round rod along a polyline with one radius per point. */
export type Tube = { kind: "tube"; pts: V3[]; r: number[] };

/** ⭕️ A horizontal ring of rectangular section (bent seat or leg ring), centre at the middle of its height. */
export type Hoop = { kind: "hoop"; c: V3; ri: number; ro: number; h: number; chamfer: number };

/** 💿️ A horizontal disc between two heights, optionally with holes (x, z rings). */
export type Disc = { kind: "disc"; c: V3; r: number; y0: number; y1: number; holes: V2[][] };

/** 🥏️ A filled horizontal seat body of the coarse scales: radius, bottom and top height, an optional shallow dish. */
export type Slab = { kind: "slab"; c: V3; r: number; y0: number; y1: number; dish?: { r: number; depth: number } };

/** 📦️ A box with rounded edges, centred. */
export type Box = { kind: "box"; c: V3; size: V3; corner: number };

export type Primitive = Tube | Hoop | Disc | Slab | Box;

/** 📏️ Seat ring: outer radius, inner radius, bottom and top height. */
export const SEAT = { ro: 0.21, ri: 0.185, y0: 0.422, y1: 0.46 };

/** 🧺️ Cane weave: radius (5 mm in the ring groove), bottom and top height (3 mm sheet, 3 mm below the ring top). */
export const CANE = { r: 0.19, y0: 0.454, y1: 0.457 };

/** 🦵️ Front legs: foot centre and top centre (left side, z > 0), foot and top radius. */
export const FRONT = { foot: [0.19, 0.0015, 0.17] as V3, top: [0.14, 0.428, 0.14] as V3, rFoot: 0.012, rTop: 0.015 };

/** 🔩️ Rear leg: foot centre (left), point beside the seat ring, radius at the foot and at the ring. */
export const REAR = { foot: [-0.215, 0.0015, 0.2] as V3, ring: [-0.1812, 0.44, 0.13] as V3, rFoot: 0.015, rRing: 0.0145 };

/** 🌂️ Back bow above the ring: slope dx/dy of the back plane, half width of the posts at the arch spring, spring height, arch half height, radius at the top. */
export const BOW = { slope: -0.3, zSpring: 0.125, ySpring: 0.72, archH: 0.108, rTop: 0.012 };

/** 🎀️ Inner back loop: start on the ring top, half width, spring height, arch half height, radius. */
export const INNER = { x0: -0.178, y0: 0.455, z: 0.085, ySpring: 0.58, archH: 0.085, r: 0.01 };

/** 🪢️ Leg ring: centre height, section height and radial width; centre and radius follow from the legs. */
export const LEG_RING = { y: 0.2, h: 0.024, w: 0.02 };

/** 🪟️ Height of the bone origin of every chair (middle of the seat ring) above the chair's floor. */
export const Y0 = (SEAT.y0 + SEAT.y1) / 2;

const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;
const lerp3 = (a: V3, b: V3, t: number): V3 => [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)];

/** 🦿️ The front leg of side s (+1 left, -1 right) as a tapered rod. */
export function frontLeg(s: number, r = [FRONT.rFoot, FRONT.rTop]): Tube {
  return { kind: "tube", pts: [[FRONT.foot[0], FRONT.foot[1], s * FRONT.foot[2]], [FRONT.top[0], FRONT.top[1], s * FRONT.top[2]]], r };
}

/** 🦵️ The rear leg of side s from the foot to the bend at the ring, `n` segments, plus the start of the bend so that the leg and the bow overlap inside the rod. */
export function rearLeg(s: number, n: number): Tube {
  const pts: V3[] = [];
  const r: number[] = [];
  for (let i = 0; i <= n; i++) {
    const t = i / n;
    const p = lerp3(REAR.foot, REAR.ring, t);
    pts.push([p[0], p[1], s * p[2]]);
    r.push(lerp(REAR.rFoot, REAR.rRing, t));
  }
  const up = bowPoint(s, 0.012);
  pts.push([up[0], up[1], up[2]]);
  r.push(REAR.rRing);
  return { kind: "tube", pts, r };
}

const backX = (y: number): number => REAR.ring[0] + BOW.slope * (y - REAR.ring[1]);

/** 🌂️ A point of the bow centre line: `d` metres of height above the ring on the post of side s while below the arch spring. */
function bowPoint(s: number, d: number): V3 {
  const y = REAR.ring[1] + d;
  const t = (y - REAR.ring[1]) / (BOW.ySpring - REAR.ring[1]);
  return [backX(y), y, s * lerp(REAR.ring[2], BOW.zSpring, t)];
}

/** 🌈️ The back bow from the ring on the left over the arch to the ring on the right, `postN` segments per post and `archN` over the arch, with the bend below the ring that overlaps the rear legs. */
export function bow(postN: number, archN: number, withBend = true): Tube {
  const pts: V3[] = [];
  const r: number[] = [];
  const rAt = (y: number): number => lerp(REAR.rRing, BOW.rTop, Math.min(1, (y - REAR.ring[1]) / (BOW.ySpring + BOW.archH - REAR.ring[1])));
  const post = (s: number): V3[] => Array.from({ length: postN + 1 }, (_, i) => bowPoint(s, ((BOW.ySpring - REAR.ring[1]) * i) / postN));
  const left = post(1);
  if (withBend) {
    const below = lerp3(REAR.ring, REAR.foot, 0.03);
    pts.push([below[0], below[1], below[2]]);
    r.push(REAR.rRing);
  }
  for (const p of left) (pts.push(p), r.push(rAt(p[1])));
  for (let i = 1; i < archN; i++) {
    const phi = (Math.PI * i) / archN;
    const y = BOW.ySpring + BOW.archH * Math.sin(phi);
    const p: V3 = [backX(y), y, BOW.zSpring * Math.cos(phi)];
    pts.push(p);
    r.push(rAt(y));
  }
  for (const p of post(-1).reverse()) (pts.push(p), r.push(rAt(p[1])));
  if (withBend) {
    const below = lerp3(REAR.ring, REAR.foot, 0.03);
    pts.push([below[0], below[1], -below[2]]);
    r.push(REAR.rRing);
  }
  return { kind: "tube", pts, r };
}

/** 🌉️ The whole frame bow of the coarse scales: left foot, left post, arch, right post, right foot as one rod. */
export function wholeBow(legN: number, postN: number, archN: number, radius?: number): Tube {
  const upper = bow(postN, archN, false);
  const legL = rearLeg(1, legN);
  const legR = rearLeg(-1, legN);
  const pts = [...legL.pts.slice(0, legN), ...upper.pts, ...legR.pts.slice(0, legN).reverse()];
  const r = [...legL.r.slice(0, legN), ...upper.r, ...legR.r.slice(0, legN).reverse()];
  return { kind: "tube", pts, r: radius === undefined ? r : r.map(() => radius) };
}

/** 🎀️ The inner back loop from the ring on the left over its arch to the ring on the right. */
export function innerLoop(postN: number, archN: number, radius = INNER.r): Tube {
  const x = (y: number): number => INNER.x0 + BOW.slope * (y - INNER.y0);
  const pts: V3[] = [];
  for (let i = 0; i <= postN; i++) {
    const y = lerp(INNER.y0, INNER.ySpring, i / postN);
    pts.push([x(y), y, INNER.z]);
  }
  for (let i = 1; i < archN; i++) {
    const phi = (Math.PI * i) / archN;
    const y = INNER.ySpring + INNER.archH * Math.sin(phi);
    pts.push([x(y), y, INNER.z * Math.cos(phi)]);
  }
  for (let i = postN; i >= 0; i--) {
    const y = lerp(INNER.y0, INNER.ySpring, i / postN);
    pts.push([x(y), y, -INNER.z]);
  }
  return { kind: "tube", pts, r: pts.map(() => radius) };
}

/** 🪢️ Centre (x) and centre-line radius of the leg ring, solved so that its outer face meets the inner faces of all four legs with 1 mm of glue line. */
export function legRing(): { cx: number; R: number } {
  const at = (a: V3, b: V3, y: number): V2 => {
    const t = (y - a[1]) / (b[1] - a[1]);
    return [lerp(a[0], b[0], t), lerp(a[2], b[2], t)];
  };
  const f = at(FRONT.foot, FRONT.top, LEG_RING.y);
  const rf = lerp(FRONT.rFoot, FRONT.rTop, (LEG_RING.y - FRONT.foot[1]) / (FRONT.top[1] - FRONT.foot[1]));
  const b = at(REAR.foot, REAR.ring, LEG_RING.y);
  const rb = lerp(REAR.rFoot, REAR.rRing, (LEG_RING.y - REAR.foot[1]) / (REAR.ring[1] - REAR.foot[1]));
  const gap = (cx: number): number => Math.hypot(f[0] - cx, f[1]) - rf - (Math.hypot(b[0] - cx, b[1]) - rb);
  let lo = -0.1;
  let hi = 0.1;
  for (let i = 0; i < 80; i++) {
    const mid = (lo + hi) / 2;
    if (gap(lo) * gap(mid) <= 0) hi = mid;
    else lo = mid;
  }
  const cx = (lo + hi) / 2;
  return { cx, R: Math.hypot(f[0] - cx, f[1]) - rf - LEG_RING.w / 2 + 0.001 };
}

/** 🕳️ The tear in the cane of the fourth chair: an irregular split near the front left of the seat (x, z). */
export const TEAR: V2[] = [[0.085, 0.035], [0.105, 0.028], [0.128, 0.04], [0.142, 0.062], [0.137, 0.088], [0.115, 0.095], [0.094, 0.082], [0.082, 0.058]];
