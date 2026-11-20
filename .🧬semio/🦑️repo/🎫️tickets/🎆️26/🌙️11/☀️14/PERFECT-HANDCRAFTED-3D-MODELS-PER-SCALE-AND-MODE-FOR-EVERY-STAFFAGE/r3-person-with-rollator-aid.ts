/** 🛒 The 1:50 rollator of `humans/person-with-rollator`, stated member by member in the frame of its bone (u ahead from the rear tread patch, v up, w left): two lambda side frames of aluminium tube joined by two cross tubes and the seat, telescopic handle posts bent back into the grips, loop brake levers with cables to the rear wheels, four 200 mm wheels (the front pair in swivel forks), a padded seat, a backrest strap, a mesh basket between the front legs and the shopping bag in it.
 *
 * @see ./r3-person-with-rollator-kit.ts
 * @see ./r3-humans-person-with-rollator.md */
import type { Json } from "./volumes-lib.ts";
import { box, capsule, extrude, loft, mirrored, revolve, toRight, tube, type Common, type Quat, type St, type V3 } from "./r3-person-with-rollator-kit.ts";
import { quatOf } from "./r3-person-with-rollator-figure.ts";

export const TONE_AID = { tube: 0.6, tubeFar: 0.52, seat: 0.3, basket: 0.62, bag: 0.8, tyre: 0.12, rim: 0.72, grip: 0.14, fork: 0.45 } as const;
/** 📐 The geometry every list and scale shares. */
export const G = {
  side: 0.235,
  wheelbase: 0.52,
  wheelR: 0.1,
  tyreHalf: 0.018,
  rearTrack: 0.283,
  head: [0.553, 0.265] as [number, number],
  rake: 0.625,
  apex: [0.273, 0.713] as [number, number],
  gripV: 0.775,
  grip: [0.085, 0.205] as [number, number],
  bendR: 0.05,
  seat: { u0: 0.17, u1: 0.42, top: 0.523, half: 0.215 },
  railV: 0.5,
  lowV: 0.3,
  basket: { u0: 0.44, u1: 0.64, v0: 0.3, v1: 0.5, half: 0.18, wall: 0.004 },
  bag: { u0: 0.482, u1: 0.598, v0: 0.308, v1: 0.555, half: 0.15 },
  strap: { v: 0.7, h: 0.035, t: 0.006, u0: 0.283, uFront: 0.5, r: 0.06, side: 0.229 },
};
/** 📏 The front leg (main tube) at the height `v`: its position ahead. */
export const frontU = (v: number): number => G.head[0] - (v - G.head[1]) * G.rake;
/** 📏 The rear leg at the height `v`. */
export const rearU = (v: number): number => 0.012 + (v - 0.127) * (0.25 / 0.562);
/** 🪝 The centre line of the handle post from the apex of the side frame to the rear end of the handle: straight in the line of the front leg, bent back to the level handle. */
export function handlePath(steps = 6, bendR = G.bendR): [number, number][] {
  const a = Math.atan2(1, G.rake);
  const centre: [number, number] = [0, G.gripV - bendR];
  const t1v = centre[1] + bendR * Math.cos(a);
  centre[0] = frontU(t1v) - bendR * Math.sin(a);
  const out: [number, number][] = [[frontU(0.69), 0.69], [frontU(t1v), t1v]];
  for (let i = 1; i <= steps; i++) {
    const b = (Math.PI / 2 - a) * (1 - i / steps);
    out.push([centre[0] + bendR * Math.sin(b), centre[1] + bendR * Math.cos(b)]);
  }
  out.push([G.grip[0] + 0.004, G.gripV]);
  return out;
}
const at = (p: readonly [number, number], w: number): V3 => [p[0], p[1], w];

/** 🎡 One wheel on its bone (origin at the axle, w along the axle): polyurethane tyre, rim ring, five spokes, hub. */
export function wheel(bone: string, name: string, w0: number, far: boolean): Json[] {
  const h = G.tyreHalf;
  const tyre: [number, number][] = [[w0 - h, 0.076], [w0 - h, 0.093], [w0 - h + 0.007, 0.1], [w0 + h - 0.007, 0.1], [w0 + h, 0.093], [w0 + h, 0.076], [w0 - h, 0.076]];
  const rim: [number, number][] = [[w0 - 0.015, 0.06], [w0 - 0.015, 0.078], [w0 + 0.015, 0.078], [w0 + 0.015, 0.06], [w0 - 0.015, 0.06]];
  const hub: [number, number][] = [[w0 - 0.021, 0], [w0 - 0.021, 0.016], [w0 - 0.016, 0.021], [w0 + 0.016, 0.021], [w0 + 0.021, 0.016], [w0 + 0.021, 0]];
  const grey = (id: string, tone: number, role = "body"): Common => ({ id: `${id}-${name}`, bone, material: "nylon-grey", tone, role });
  return [
    revolve({ id: `tyre-${name}`, bone, material: "pu-tyre", tone: far ? 0.1 : TONE_AID.tyre }, tyre, [0, 0, 0], [0, 0, 1]),
    revolve(grey("rim", far ? 0.64 : TONE_AID.rim), rim, [0, 0, 0], [0, 0, 1]),
    revolve(grey("hub", 0.4), hub, [0, 0, 0], [0, 0, 1]),
    box(grey("spoke", far ? 0.64 : TONE_AID.rim, "detail"), [0.046, 0.013, 0.022], [0.041, 0, w0], undefined, 0.004, { repeat: { count: 5, rotationAxis: [0, 0, 1], rotationRad: (2 * Math.PI) / 5, origin: [0, 0, w0] } }),
  ];
}

/** 🎗️ The backrest strap as a path in plan [u, w] with its outward normal: from the left main tube ahead, round the corner and across the front. */
function strapSections(path: readonly [number, number][]): Json[] {
  const s = G.strap;
  return path.map((p, i) => {
    const a = path[Math.max(0, i - 1)]!;
    const b = path[Math.min(path.length - 1, i + 1)]!;
    const l = Math.hypot(b[0] - a[0], b[1] - a[1]);
    const t: V3 = [(b[0] - a[0]) / l, 0, (b[1] - a[1]) / l];
    const ex: V3 = [0, 1, 0];
    const ey: V3 = [t[1] * ex[2] - t[2] * ex[1], t[2] * ex[0] - t[0] * ex[2], t[0] * ex[1] - t[1] * ex[0]];
    return { at: [p[0], s.v, p[1]].map((x) => Math.round(x * 1e5) / 1e5), rx: s.h, ry: s.t, exponent: 4, rotation: quatOf(ex, ey, t) as Quat };
  });
}
export function strapSide(): [number, number][] {
  const s = G.strap;
  const out: [number, number][] = [[s.u0, s.side], [0.36, s.side], [s.uFront - s.r, s.side]];
  for (let i = 1; i <= 6; i++) {
    const a = (i / 6) * (Math.PI / 2);
    out.push([s.uFront - s.r + s.r * Math.sin(a), s.side - s.r + s.r * Math.cos(a)]);
  }
  return out;
}

/** 🧺 The body solids of the rollator at 1:50. */
export function aid50(): Json[] {
  const out: Json[] = [];
  const alu = (id: string, tone: number = TONE_AID.tubeFar, role = "body"): Common => ({ id, bone: "frame", material: "alu-red-paint", tone, role });
  const grey = (id: string, tone: number = TONE_AID.fork, role = "body"): Common => ({ id, bone: "frame", material: "nylon-grey", tone, role });
  const nylon = (id: string, tone: number, role = "body"): Common => ({ id, bone: "frame", material: "nylon-seat", tone, role });
  const S = G.side;
  const lit = (t: number): number => (t === TONE_AID.tubeFar ? TONE_AID.tube : t);

  const side: Json[] = [];
  side.push(tube(alu("main-tube-l"), [at([G.head[0], G.head[1] - 0.004], S), at([frontU(0.5), 0.5], S), at([frontU(0.727), 0.727], S)], [0.014]));
  side.push(tube(alu("rear-leg-l"), [at([frontU(0.689) - 0.004, 0.689], S), at([rearU(0.4), 0.4], S), at([rearU(0.118), 0.118], S)], [0.0125]));
  side.push(tube(alu("lower-rail-l"), [at([rearU(G.lowV), G.lowV], S), at([0.3, G.lowV], S), at([frontU(G.lowV), G.lowV], S)], [0.011]));
  side.push(tube(alu("seat-rail-l"), [at([rearU(G.railV), G.railV], S), at([0.29, G.railV], S), at([frontU(G.railV), G.railV], S)], [0.011]));
  side.push(tube(alu("handle-post-l", 0.66), handlePath().map((p) => at(p, S)), [0.011]));
  side.push(revolve(grey("post-clamp-l", 0.3), [[-0.016, 0], [-0.016, 0.0175], [0.016, 0.0175], [0.016, 0]], at([frontU(0.722), 0.722], S), [-G.rake / Math.hypot(1, G.rake), 1 / Math.hypot(1, G.rake), 0]));
  side.push(capsule(grey("post-knob-l", 0.2, "detail"), at([frontU(0.722), 0.722], S + 0.016), at([frontU(0.722), 0.722], S + 0.034), 0.009, 0.011));
  side.push(revolve({ id: "grip-l", bone: "frame", material: "rubber-grip", tone: TONE_AID.grip }, [[G.grip[0] - 0.002, 0], [G.grip[0] - 0.002, 0.0195], [G.grip[0] + 0.005, 0.021], [G.grip[0] + 0.012, 0.0178], [0.14, 0.0172], [0.19, 0.0165], [G.grip[1] - 0.007, 0.019], [G.grip[1], 0.019], [G.grip[1], 0]], [0, G.gripV, S], [1, 0, 0]));
  side.push(box(grey("brake-housing-l", 0.3), [0.03, 0.02, 0.02], [0.222, 0.757, S], undefined, 0.004));
  side.push(tube(grey("brake-lever-l", 0.3), [[0.224, 0.75, S], [0.2, 0.74, S], [0.15, 0.735, S], [0.108, 0.74, S], [0.098, 0.747, S]], [0.0055, 0.005, 0.005, 0.005, 0.0055]));
  side.push(tube({ ...grey("brake-cable-l", 0.15, "detail") }, [[0.236, 0.752, S + 0.004], [0.268, 0.742, S + 0.012], [0.284, 0.71, S + 0.018], [0.262, 0.652, S + 0.018], [0.15, 0.4, S + 0.018], [0.07, 0.24, S + 0.02], [0.045, 0.214, S + 0.026]], [0.002], { slender: true }));
  side.push(box(grey("brake-block-l", 0.3), [0.034, 0.016, 0.05], [0.032, 0.2075, S + 0.03], undefined, 0.004));
  side.push(box(alu("axle-plate-l"), [0.032, 0.05, 0.012], [0.006, 0.104, S], undefined, 0.004));
  side.push(capsule(grey("axle-rear-l", 0.3, "detail"), [0, G.wheelR, S - 0.008], [0, G.wheelR, G.rearTrack + 0.012], 0.006, 0.006));
  side.push(revolve(grey("caster-head-l"), [[0.2, 0], [0.2, 0.0185], [0.264, 0.0185], [0.27, 0.0145], [0.27, 0]], [G.head[0], 0, S], [0, 1, 0]));
  side.push(box(grey("caster-crown-l"), [0.046, 0.012, 0.062], [G.head[0] - 0.006, 0.2065, S], undefined, 0.004));
  const blade = (id: string, w: number): Json => loft(grey(id), ([[G.head[0] - 0.006, 0.204], [0.535, 0.15], [G.wheelbase, 0.097]] as [number, number][]).map(([u, v], i): St => ({ at: [u, v, w], rw: 0.005, rv: i === 2 ? 0.014 : 0.013, e: 4, n: 76 })));
  side.push(blade("caster-blade-out-l", S + 0.026));
  side.push(blade("caster-blade-in-l", S - 0.026));
  side.push(capsule(grey("axle-front-l", 0.3, "detail"), [G.wheelbase, G.wheelR, S - 0.03], [G.wheelbase, G.wheelR, S + 0.03], 0.005, 0.005));
  side.push({ kind: "loft", id: "backrest-l", bone: "frame", material: "nylon-seat", role: "body", tone: 0.3, sections: strapSections(strapSide()), capStart: true, capEnd: true });
  for (const s of side) out.push(s);
  for (const s of side) out.push(mirrored(s, toRight, lit));

  out.push(capsule(alu("cross-upper", TONE_AID.tube), [frontU(0.485), 0.485, -S], [frontU(0.485), 0.485, S], 0.011, 0.011));
  out.push(capsule(alu("cross-lower", TONE_AID.tube), [frontU(G.lowV), G.lowV, -S], [frontU(G.lowV), G.lowV, S], 0.011, 0.011));
  const seat = G.seat;
  out.push(box(nylon("seat", TONE_AID.seat), [seat.u1 - seat.u0, 0.024, 2 * seat.half], [(seat.u0 + seat.u1) / 2, seat.top - 0.012, 0], undefined, 0.01));
  const s = G.strap;
  out.push(box(nylon("backrest", 0.3), [2 * s.t, 2 * s.h, 2 * (s.side - s.r) + 0.002], [s.uFront, s.v, 0], undefined, 0.005));

  const b = G.basket;
  const rect = (h: number, u0: number, u1: number, r: number): [number, number][] => [[h - r, u0], [h, u0 + r], [h, u1 - r], [h - r, u1], [-h + r, u1], [-h, u1 - r], [-h, u0 + r], [-h + r, u0]];
  out.push(extrude(nylon("basket", TONE_AID.basket), "zx", rect(b.half, b.u0, b.u1, 0.02), b.v0, b.v1, { holes: [rect(b.half - b.wall, b.u0 + b.wall, b.u1 - b.wall, 0.017)] }));
  out.push(extrude(nylon("basket-floor", 0.5), "zx", rect(b.half - 0.001, b.u0 + 0.001, b.u1 - 0.001, 0.02), b.v0, b.v0 + 0.008));
  out.push(extrude(nylon("basket-rim", 0.4, "detail"), "zx", rect(b.half + 0.003, b.u0 - 0.003, b.u1 + 0.003, 0.022), b.v1 - 0.008, b.v1 + 0.002, { holes: [rect(b.half - b.wall - 0.003, b.u0 + b.wall + 0.003, b.u1 - b.wall - 0.003, 0.015)] }));
  for (const side2 of [1, -1]) out.push(tube(grey(`basket-hook-${side2 > 0 ? "l" : "r"}`, 0.3, "detail"), [[b.u0 + 0.004, b.v1 - 0.004, side2 * 0.12], [b.u0 - 0.012, b.v1 + 0.004, side2 * 0.12], [frontU(0.485) - 0.004, 0.499, side2 * 0.12], [frontU(0.485) - 0.013, 0.486, side2 * 0.12]], [0.004]));

  const g = G.bag;
  const tote: Common = { id: "bag", bone: "frame", material: "cotton-tote", tone: TONE_AID.bag };
  const outline: [number, number][] = [[g.u0 + 0.008, g.v0], [g.u1 - 0.008, g.v0], [g.u1, g.v0 + 0.012], [g.u1 + 0.004, g.v0 + 0.1], [g.u1 - 0.004, g.v1 - 0.05], [g.u1 - 0.022, g.v1 - 0.004], [g.u1 - 0.03, g.v1], [g.u0 + 0.03, g.v1], [g.u0 + 0.022, g.v1 - 0.004], [g.u0 + 0.004, g.v1 - 0.05], [g.u0 - 0.004, g.v0 + 0.1], [g.u0, g.v0 + 0.012]];
  out.push(extrude(tote, "xy", outline, -g.half, g.half));
  const loop = (id: string, u: number, lean: number): Json => tube({ ...tote, id, tone: 0.7, role: "detail" }, Array.from({ length: 9 }, (_, i) => { const a = (i / 8) * Math.PI; return [u + lean * Math.sin(a), g.v1 - 0.004 + 0.066 * Math.sin(a), -0.045 * Math.cos(a)] as V3; }), [0.0045], { slender: true });
  out.push(loop("bag-handle-rear", g.u0 + 0.034, -0.012));
  out.push(loop("bag-handle-front", g.u1 - 0.034, 0.014));

  out.push(...wheel("wheel-rear", "rear-l", G.rearTrack, true), ...wheel("wheel-rear", "rear-r", -G.rearTrack, false));
  out.push(...wheel("wheel-front", "front-l", S, true), ...wheel("wheel-front", "front-r", -S, false));
  return out;
}
