/** 🦾️ Numeric inverse kinematics on the rig chain of the library (`solveChain`): Levenberg–Marquardt over bone offsets within the joint limits, frame algebra for carried props.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Rest placement of a bone" */
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import type { Bones, Json } from "./lib.ts";

export type V = [number, number, number];
export type F = { o: V; u: V; v: V; w: V };
export const add = (a: V, b: V): V => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V, s: number): V => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: V, b: V) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const unit = (a: V): V => mul(a, 1 / Math.hypot(...a));
/** 📍️ A point given in frame-local (u, v, w) coordinates, in world. */
export const at = (f: F, p: V): V => add(f.o, add(mul(f.u, p[0]), add(mul(f.v, p[1]), mul(f.w, p[2]))));
/** 📍️ A world point in frame-local coordinates. */
export const local = (f: F, p: V): V => { const d = sub(p, f.o); return [dot(d, f.u), dot(d, f.v), dot(d, f.w)]; };
/** 🧭️ A world direction in frame-local coordinates. */
export const localDir = (f: F, d: V): V => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
/** 🧭️ A frame-local direction in world. */
export const dir = (f: F, d: V): V => add(mul(f.u, d[0]), add(mul(f.v, d[1]), mul(f.w, d[2])));
/** 🗺️ A child frame given in parent-local terms, in world. */
export const compose = (f: F, c: F): F => ({ o: at(f, c.o), u: dir(f, c.u), v: dir(f, c.v), w: dir(f, c.w) });
/** 🗺️ A world frame in parent-local terms. */
export const relative = (f: F, c: F): F => ({ o: local(f, c.o), u: localDir(f, c.u), v: localDir(f, c.v), w: localDir(f, c.w) });
/** 🗺️ A right frame from an origin, a u direction and an approximate w direction. */
export const frameOf = (o: V, u: V, wApprox: V): F => { const uu = unit(u); const w = unit(sub(wApprox, mul(uu, dot(wApprox, uu)))); return { o, u: uu, v: cross(w, uu), w }; };

/** 🦴️ World frames of every bone for the given offsets (root at the origin). */
export function frames(asset: Json, offsets: Bones): Record<string, F> {
  const s = solveChain(asset.rig, offsets, { x: 0, y: 0, angleDeg: 0 });
  return Object.fromEntries(Object.entries(s).map(([id, f]) => [id, { o: [...f.frame.o] as V, u: [...f.frame.u] as V, v: [...f.frame.v] as V, w: [...f.frame.w] as V }]));
}

/** 🔢️ The rest placement (angle, abduction, twist in degrees) of a rotation whose axes are given in the parent frame: R = Rw(a)·Rv(b)·Ru(c). */
export function decompose(c: F): { angleDeg: number; abductionDeg: number; twistDeg: number } {
  const b = Math.asin(Math.max(-1, Math.min(1, c.u[2])));
  const a = Math.atan2(c.u[1], c.u[0]);
  const ca = Math.cos(a), sa = Math.sin(a), cb = Math.cos(b), sb = Math.sin(b);
  const v1: V = [-sa, ca, 0];
  const w1: V = [-ca * sb, -sa * sb, cb];
  const tw = Math.atan2(-dot(c.v, w1), dot(c.v, v1));
  const d = (x: number) => Math.round((x * 180) / Math.PI * 1e4) / 1e4;
  return { angleDeg: d(a), abductionDeg: d(b), twistDeg: d(tw) };
}

export type Param = { bone: string; field: "angleDeg" | "abductionDeg" | "twistDeg"; min?: number; max?: number };

/** 🎯️ Levenberg–Marquardt: offsets (starting from `start`) of the given parameters that minimise the residual vector; parameters stay inside their joint limits and a soft pull keeps them near the start. */
export function solve(asset: Json, start: Bones, params: Param[], residual: (f: Record<string, F>) => number[], iterations = 150, pull = 0.0002): { bones: Bones; error: number } {
  const bones: Bones = structuredClone(start);
  const limits = params.map((p) => {
    const j = asset.rig.bones.find((b: Json) => b.id === p.bone).joint;
    const lo = p.min ?? (p.field === "angleDeg" ? j.minDeg ?? -180 : p.field === "twistDeg" ? j.twistMinDeg ?? -180 : -180);
    const hi = p.max ?? (p.field === "angleDeg" ? j.maxDeg ?? 180 : p.field === "twistDeg" ? j.twistMaxDeg ?? 180 : 180);
    return [lo, hi] as const;
  });
  const get = () => params.map((p) => (bones[p.bone]?.[p.field] as number | undefined) ?? 0);
  const x0 = get();
  const set = (x: number[]) => params.forEach((p, i) => { bones[p.bone] = { ...bones[p.bone], [p.field]: Math.round(Math.max(limits[i]![0], Math.min(limits[i]![1], x[i]!)) * 100) / 100 }; });
  const r = (x: number[]) => { set(x); return [...residual(frames(asset, bones)), ...x.map((xi, i) => pull * (xi - x0[i]!))]; };
  const norm = (v: number[]) => v.reduce((s, e) => s + e * e, 0);
  let x = get(), cur = r(x), lambda = 1e-2;
  for (let it = 0; it < iterations; it++) {
    const J = params.map((_, j) => { const h = 0.05; const xp = [...x]; xp[j]! += h; const rp = r(xp); return rp.map((e, k) => (e - cur[k]!) / h); });
    const n = params.length;
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, k) => J[i]!.reduce((s, e, m) => s + e * J[k]![m]!, 0)));
    const g = Array.from({ length: n }, (_, i) => J[i]!.reduce((s, e, m) => s + e * cur[m]!, 0));
    let improved = false;
    for (let tries = 0; tries < 8 && !improved; tries++) {
      const M = A.map((row, i) => row.map((e, k) => e + (i === k ? lambda * (1 + A[i]![i]!) : 0)));
      const step = gauss(M, g.map((e) => -e));
      const xn = x.map((xi, i) => Math.max(limits[i]![0], Math.min(limits[i]![1], xi + step[i]!)));
      const rn = r(xn);
      if (norm(rn) < norm(cur)) { x = xn; cur = rn; lambda *= 0.3; improved = true; } else lambda *= 4;
    }
    if (!improved) break;
  }
  set(x);
  return { bones, error: Math.sqrt(norm(residual(frames(asset, bones)))) };
}

function gauss(M: number[][], b: number[]): number[] {
  const n = b.length, a = M.map((row, i) => [...row, b[i]!]);
  for (let i = 0; i < n; i++) {
    let p = i; for (let k = i + 1; k < n; k++) if (Math.abs(a[k]![i]!) > Math.abs(a[p]![i]!)) p = k;
    [a[i], a[p]] = [a[p]!, a[i]!];
    for (let k = i + 1; k < n; k++) { const f = a[k]![i]! / a[i]![i]!; for (let m = i; m <= n; m++) a[k]![m]! -= f * a[i]![m]!; }
  }
  const x = new Array(n).fill(0);
  for (let i = n - 1; i >= 0; i--) x[i] = (a[i]![n]! - a[i]!.slice(i + 1, n).reduce((s, e, k) => s + e * x[i + 1 + k], 0)) / a[i]![i]!;
  return x;
}
