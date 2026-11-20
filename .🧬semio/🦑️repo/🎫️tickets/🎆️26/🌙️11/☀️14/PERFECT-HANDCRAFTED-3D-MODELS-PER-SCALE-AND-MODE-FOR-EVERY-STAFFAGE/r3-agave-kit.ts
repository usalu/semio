/** 🧰️ Round 3, vegetation/agave: vector helpers, quaternions, bone-local lofts from world sections and the pose solver for one bone. It converts only; every form is stated in `r3-agave-plant.ts`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-agave.md */
import { dirToLocal, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";

export const D2R = Math.PI / 180;
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const lin = (a: V3, s: number, b: V3, t: number): V3 => [a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 🧭️ Unit direction of azimuth `az` (degrees from +x towards -z, as the sheets print it) and elevation `el` above the horizon. */
export const dirOf = (az: number, el: number): V3 => [Math.cos(el * D2R) * Math.cos(az * D2R), Math.sin(el * D2R), -Math.cos(el * D2R) * Math.sin(az * D2R)];

/** ⬆️ The upward normal of that direction in its vertical plane. */
export const upOf = (az: number, el: number): V3 => [-Math.sin(el * D2R) * Math.cos(az * D2R), Math.cos(el * D2R), Math.sin(el * D2R) * Math.sin(az * D2R)];

/** ↔️ The horizontal lateral of azimuth `az` (u x v of a bone pointing there with its normal up). */
export const sideOf = (az: number): V3 => [Math.sin(az * D2R), 0, Math.cos(az * D2R)];

/** 📈️ Piecewise linear interpolation in a table of [x, y] rows sorted by x. */
export function table(rows: readonly (readonly [number, number])[], x: number): number {
  if (x <= rows[0]![0]) return rows[0]![1];
  for (let i = 1; i < rows.length; i++) if (x <= rows[i]![0]) {
    const [a, b] = [rows[i - 1]!, rows[i]!];
    return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]);
  }
  return rows[rows.length - 1]![1];
}

/** 🧮️ The unit quaternion [x, y, z, w] of a rotation matrix given by its columns. */
export function quaternion(c0: V3, c1: V3, c2: V3): [number, number, number, number] {
  const [m00, m10, m20] = c0, [m01, m11, m21] = c1, [m02, m12, m22] = c2;
  const t = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (t > 0) {
    const s = Math.sqrt(t + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
  }
  const n = Math.hypot(...q);
  return q.map((x) => r5(x / n)) as [number, number, number, number];
}

/** 🧊️ The rotation that carries the local axes onto the world axes x, y, z, written in the frame `f`. */
export const localRotation = (f: Frame, x: V3, y: V3, z: V3): [number, number, number, number] => quaternion(unit(dirToLocal(f, x)), unit(dirToLocal(f, y)), unit(dirToLocal(f, z)));
export const localPoint = (f: Frame, p: V3): V3 => toLocal(f, p).map(r5) as V3;

/** 🎚️ A world section of a loft: centre, in-plane axes x (carries `a`) and y (carries `b`), superellipse exponent. */
export type Section = { p: V3; x: V3; y: V3; a: number; b: number; n?: number };

/** 🪵️ The bone-local loft of world sections. */
export function loftOf(id: string, bone: string, f: Frame, sections: readonly Section[], fields: Json): Json {
  return {
    id, kind: "loft", bone, ...fields,
    sections: sections.map((s) => ({ at: localPoint(f, s.p), rx: r5(Math.max(s.a, 2e-4)), ry: r5(Math.max(s.b, 2e-4)), ...(s.n !== undefined && s.n !== 2 ? { exponent: s.n } : {}), rotation: localRotation(f, s.x, s.y, cross(s.x, s.y)) })),
    capStart: true, capEnd: true,
  };
}

/** 🥚️ A bone-local ellipsoid at a world centre with world axes. */
export const ellipsoidOf = (id: string, bone: string, f: Frame, c: V3, axes: [V3, V3, V3], radii: V3, fields: Json): Json => ({ id, kind: "ellipsoid", bone, ...fields, radii: radii.map(r5), transform: { translation: localPoint(f, c), rotation: localRotation(f, axes[0], axes[1], axes[2]) } });

/** 💊️ A bone-local capsule between two world points. */
export const capsuleOf = (id: string, bone: string, f: Frame, p0: V3, p1: V3, r0: number, r1: number, fields: Json): Json => ({ id, kind: "capsule", bone, ...fields, p0: localPoint(f, p0), p1: localPoint(f, p1), r0: r5(r0), r1: r5(r1) });

/** 🦴️ Offsets (angle from rest, abduction, twist) of a bone under the parent frame `P` whose world axis is `d` with its normal `up` (the solver convention of the product rig). */
export function solveOffsets(P: { u: V3; v: V3; w: V3 }, d: V3, up: V3, restDeg: number): { angleDeg: number; abductionDeg: number; twistDeg: number } {
  const [du, dv, dw] = [dot(d, P.u), dot(d, P.v), dot(d, P.w)];
  const phi = Math.asin(Math.max(-1, Math.min(1, dw)));
  const theta = Math.atan2(dv, du);
  const u1 = lin(P.u, Math.cos(theta), P.v, Math.sin(theta));
  const v1 = lin(P.u, -Math.sin(theta), P.v, Math.cos(theta));
  const w2 = lin(u1, -Math.sin(phi), P.w, Math.cos(phi));
  const psi = Math.atan2(-dot(up, w2), dot(up, v1));
  let angle = theta / D2R - restDeg;
  while (angle > 180) angle -= 360;
  while (angle < -180) angle += 360;
  return { angleDeg: r4(angle), abductionDeg: r4(phi / D2R), twistDeg: r4(psi / D2R) };
}

/** 🧭️ Azimuth (degrees, from +x towards -z) and elevation of a world direction. */
export const azElOf = (u: V3): { az: number; el: number } => ({ az: Math.atan2(-u[2], u[0]) / D2R, el: Math.asin(Math.max(-1, Math.min(1, u[1]))) / D2R });
