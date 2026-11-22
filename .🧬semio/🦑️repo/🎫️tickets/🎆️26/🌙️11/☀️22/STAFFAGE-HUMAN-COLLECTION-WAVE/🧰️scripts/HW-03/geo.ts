/** 📐️ Plane and solid geometry helpers of HW-03: stroke outlines of centre lines, rectangles, and the surface of an authored loft (to lay a tube or a strap on a garment). Pure functions, metres. */
import type { J } from "./lib.ts";

export type P2 = [number, number];
export type P3 = [number, number, number];

const r6 = (x: number): number => +x.toFixed(5);

/** ✏️ The closed outline of a centre line drawn `width` wide (mitred joints, square ends). */
export function stroke(points: P2[], width: number): P2[] {
  const h = width / 2;
  const normals = points.map((_, i) => {
    const a = points[Math.max(0, i - 1)]!, b = points[Math.min(points.length - 1, i + 1)]!;
    const dx = b[0] - a[0], dy = b[1] - a[1], l = Math.hypot(dx, dy) || 1;
    return [-dy / l, dx / l] as P2;
  });
  const left = points.map((p, i) => [r6(p[0] + normals[i]![0] * h), r6(p[1] + normals[i]![1] * h)] as P2);
  const right = points.map((p, i) => [r6(p[0] - normals[i]![0] * h), r6(p[1] - normals[i]![1] * h)] as P2);
  return [...left, ...right.reverse()];
}

/** ▭️ A rectangle outline from two corners. */
export const rect = (x0: number, y0: number, x1: number, y1: number): P2[] => [[r6(x0), r6(y0)], [r6(x1), r6(y0)], [r6(x1), r6(y1)], [r6(x0), r6(y1)]];

/** ⭕️ A polygon of `n` points on an ellipse. */
export const ellipse = (cx: number, cy: number, rx: number, ry: number, n = 16): P2[] => Array.from({ length: n }, (_, i) => [r6(cx + rx * Math.cos((2 * Math.PI * i) / n)), r6(cy + ry * Math.sin((2 * Math.PI * i) / n))] as P2);

/** 🔄️ A unit quaternion [x, y, z, w] applied to a vector. */
export function rotate(q: number[], p: P3): P3 {
  const [x, y, z, w] = q as [number, number, number, number];
  const ix = w * p[0] + y * p[2] - z * p[1], iy = w * p[1] + z * p[0] - x * p[2], iz = w * p[2] + x * p[1] - y * p[0], iw = -x * p[0] - y * p[1] - z * p[2];
  return [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x];
}

/** 🧥️ The bone-local surface point of a loft (running along its bone's u) at height `u` and lateral `w` on the side `sign` of v (-1 the low-v side, +1 the high-v side), or undefined outside its width. */
export function loftSurface(loft: J, u: number, w: number, sign: -1 | 1): number | undefined {
  const secs = loft.sections as J[];
  let i = secs.findIndex((s, k) => k < secs.length - 1 && (s.at[0] - u) * (secs[k + 1].at[0] - u) <= 0);
  if (i < 0) return undefined;
  const a = secs[i], b = secs[i + 1];
  const t = (u - a.at[0]) / (b.at[0] - a.at[0] || 1);
  const lerp = (p: number, q: number): number => p + (q - p) * t;
  const at = [0, 1, 2].map((k) => lerp(a.at[k], b.at[k])) as P3;
  const rx = lerp(a.rx, b.rx), ry = lerp(a.ry, b.ry), n = lerp(a.exponent ?? 2, b.exponent ?? 2);
  const q = a.rotation ?? [0, 0, 0, 1];
  const ax = rotate(q, [1, 0, 0]), ay = rotate(q, [0, 1, 0]);
  let best: number | undefined;
  for (let k = 0; k < 2880; k++) {
    const th = (2 * Math.PI * k) / 2880;
    const c = Math.cos(th), s = Math.sin(th);
    const X = rx * Math.sign(c) * Math.abs(c) ** (2 / n), Y = ry * Math.sign(s) * Math.abs(s) ** (2 / n);
    const p = [0, 1, 2].map((j) => at[j]! + X * ax[j]! + Y * ay[j]!) as P3;
    if (Math.abs(p[2] - w) > 0.002) continue;
    if (best === undefined || sign * p[1] > sign * best) best = p[1];
  }
  return best;
}
