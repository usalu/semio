/** 🎯️ Pose solving of HW-03: the rig's own 3D chain (`solveChain`) and a Nelder-Mead search over chosen bone offsets, so authored targets (a hand on a board, a pole base on the ground) are met by the engine's kinematics.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts — `solveChain`, `carry` */
import { type J, S } from "./lib.ts";

const rig = await import(`${S}/🔨️modules/🦴️rig/🟦️.ts`);

export type V3 = [number, number, number];
export type Frames = Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;

/** 🧊️ World frames of every bone for a set of offsets (root at the rig's own rest placement). */
export function framesOf(a: J, offsets: Record<string, J>, root = { x: 0, y: 0, angleDeg: 0 }): Frames {
  return rig.solveChain(a.rig, offsets, root);
}

/** 📍️ A bone-local point (u, v, w from the bone origin) in world coordinates. */
export function worldOf(f: Frames, bone: string, p: V3): V3 {
  const { o, u, v, w } = f[bone]!.frame;
  return [0, 1, 2].map((k) => o[k]! + p[0] * u[k]! + p[1] * v[k]! + p[2] * w[k]!) as V3;
}

/** 🔻️ Nelder-Mead minimisation of `cost` from `x0` with initial step `step`. */
export function minimise(cost: (x: number[]) => number, x0: number[], step = 5, iterations = 600): { x: number[]; f: number } {
  const n = x0.length;
  let simplex = [x0, ...x0.map((_, i) => x0.map((v, j) => (i === j ? v + step : v)))].map((x) => ({ x, f: cost(x) }));
  for (let it = 0; it < iterations; it++) {
    simplex.sort((p, q) => p.f - q.f);
    const best = simplex[0]!, worst = simplex[n]!;
    if (Math.abs(worst.f - best.f) < 1e-12) break;
    const c = x0.map((_, j) => simplex.slice(0, n).reduce((s, p) => s + p.x[j]!, 0) / n);
    const at = (k: number): number[] => c.map((v, j) => v + k * (worst.x[j]! - v));
    const r = { x: at(-1), f: 0 };
    r.f = cost(r.x);
    if (r.f < best.f) {
      const e = { x: at(-2), f: 0 };
      e.f = cost(e.x);
      simplex[n] = e.f < r.f ? e : r;
    } else if (r.f < simplex[n - 1]!.f) simplex[n] = r;
    else {
      const k = { x: at(0.5), f: 0 };
      k.f = cost(k.x);
      if (k.f < worst.f) simplex[n] = k;
      else simplex = simplex.map((p, i) => (i === 0 ? p : { x: p.x.map((v, j) => best.x[j]! + 0.5 * (v - best.x[j]!)), f: 0 })).map((p, i) => (i === 0 ? p : { ...p, f: cost(p.x) }));
    }
  }
  simplex.sort((p, q) => p.f - q.f);
  return simplex[0]!;
}

/** 🦾️ Solves the named offset fields (e.g. `arm-r.angleDeg`) of a pose so that `cost(frames)` is smallest, starting from the pose's own values. */
export function solvePose(a: J, base: Record<string, J>, fields: string[], cost: (f: Frames) => number, step = 5): Record<string, J> {
  const read = (o: Record<string, J>, field: string): number => {
    const [b, k] = field.split(".") as [string, string];
    return o[b]?.[k] ?? 0;
  };
  const write = (x: number[]): Record<string, J> => {
    const o: Record<string, J> = structuredClone(base);
    fields.forEach((field, i) => {
      const [b, k] = field.split(".") as [string, string];
      o[b] = { ...(o[b] ?? {}), [k]: +x[i]!.toFixed(4) };
    });
    return o;
  };
  const limits = fields.map((field) => {
    const [b, k] = field.split(".") as [string, string];
    const j = a.rig.bones.find((x: J) => x.id === b).joint;
    return k === "angleDeg" ? [j.minDeg ?? -180, j.maxDeg ?? 180] : k === "twistDeg" ? [j.twistMinDeg ?? -180, j.twistMaxDeg ?? 180] : [-180, 180];
  });
  const over = (x: number[]): number => x.reduce((s, v, i) => s + Math.max(0, limits[i]![0]! + 1 - v) ** 2 + Math.max(0, v - limits[i]![1]! + 1) ** 2, 0);
  const best = minimise((x) => cost(framesOf(a, write(x))) + 0.01 * over(x), fields.map((f) => read(base, f)), step);
  return write(best.x);
}

export const dist = (p: V3, q: V3): number => Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2]);
export const dot = (p: V3, q: V3): number => p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
