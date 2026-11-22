/** 🪚️ Chisel helpers of the used classical guitar: quaternions, outlines, the kernel tessellation of a solid carried to the world by the rest frames of the rig, its orthographic projection into the three paper frames (elevation bone-local, front lateral by height, plan ahead by lateral), the union of the projected triangles and its Douglas-Peucker simplification, and mesh area, volume and centroid for the mass and area tables.
 *
 * @see ./author-guitar.ts — the hand-chosen numbers
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — section "Front and plan views"
 * @see ../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts — the kernel tessellation */
import polygonClipping from "polygon-clipping";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";

export type Json = Record<string, any>;
export type V2 = [number, number];
export type V3 = [number, number, number];
export type Q = [number, number, number, number];
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
export type View = "elevation" | "front" | "plan";

export const r5 = (x: number): number => Math.round((x + 0) * 1e5) / 1e5 + 0;
export const r6 = (x: number): number => Math.round((x + 0) * 1e6) / 1e6 + 0;
export const p2 = (p: readonly number[]): V2 => [r5(p[0]!), r5(p[1]!)];
export const p3 = (p: readonly number[]): V3 => [r6(p[0]!), r6(p[1]!), r6(p[2]!)];
const RAD = Math.PI / 180;

/** 🌀️ The unit quaternion `[x, y, z, w]` of a turn by `deg` about the bone-local axis `axis`. */
export function about(axis: "u" | "v" | "w", deg: number): Q {
  const s = Math.sin((deg * RAD) / 2);
  const c = Math.cos((deg * RAD) / 2);
  return [axis === "u" ? s : 0, axis === "v" ? s : 0, axis === "w" ? s : 0, c].map(r6) as Q;
}

/** ✖️ The product `a · b` (first `b`, then `a`). */
export function qmul(a: Q, b: Q): Q {
  const [ax, ay, az, aw] = a;
  const [bx, by, bz, bw] = b;
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz].map(r6) as Q;
}

/** 🔄️ A point turned by a unit quaternion. */
export function qrot(q: Q, p: V3): V3 {
  const [x, y, z, w] = q;
  const [px, py, pz] = p;
  const tx = 2 * (y * pz - z * py);
  const ty = 2 * (z * px - x * pz);
  const tz = 2 * (x * py - y * px);
  return [px + w * tx + (y * tz - z * ty), py + w * ty + (z * tx - x * tz), pz + w * tz + (x * ty - y * tx)];
}

/** 〰️ A closed Catmull-Rom ring through `points`, `per` samples per span. */
export function smoothRing(points: readonly V2[], per: number): V2[] {
  const n = points.length;
  const out: V2[] = [];
  for (let i = 0; i < n; i++) {
    const [a, b, c, d] = [points[(i - 1 + n) % n]!, points[i]!, points[(i + 1) % n]!, points[(i + 2) % n]!];
    for (let k = 0; k < per; k++) {
      const t = k / per;
      const t2 = t * t;
      const t3 = t2 * t;
      const f = (j: 0 | 1): number => 0.5 * (2 * b[j] + (-a[j] + c[j]) * t + (2 * a[j] - 5 * b[j] + 4 * c[j] - d[j]) * t2 + (-a[j] + 3 * b[j] - 3 * c[j] + d[j]) * t3);
      out.push([f(0), f(1)]);
    }
  }
  return out;
}

/** 🎻️ A symmetric outline from a half-width table `[along, half]` (first and last half 0): the closed ring `[lateral, along]` counter-clockwise, smoothed with `per` samples per span. */
export function symmetricOutline(table: readonly V2[], per: number): V2[] {
  const right = table.map(([s, h]) => [-h, s] as V2);
  const left = [...table].reverse().slice(1, -1).map(([s, h]) => [h, s] as V2);
  const ring = smoothRing([...right, ...left], per);
  return ring.map(p2);
}

/** 📐️ The signed area of a ring. */
export function ringArea(ring: readonly V2[]): number {
  let a = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!;
    const q = ring[(i + 1) % ring.length]!;
    a += p[0] * q[1] - q[0] * p[1];
  }
  return a / 2;
}

/** 🪞️ A ring made counter-clockwise. */
export const ccw = (ring: readonly V2[]): V2[] => (ringArea(ring) < 0 ? [...ring].reverse() : [...ring]);

/** ↘️ A ring moved inwards by `d` along the averaged edge normals (counter-clockwise rings). */
export function inset(ring: readonly V2[], d: number): V2[] {
  const r = ccw(ring);
  const n = r.length;
  return r.map((p, i) => {
    const a = r[(i - 1 + n) % n]!;
    const b = r[(i + 1) % n]!;
    const e1 = [p[0] - a[0], p[1] - a[1]];
    const e2 = [b[0] - p[0], b[1] - p[1]];
    const l1 = Math.hypot(e1[0]!, e1[1]!) || 1;
    const l2 = Math.hypot(e2[0]!, e2[1]!) || 1;
    const n1 = [-e1[1]! / l1, e1[0]! / l1];
    const n2 = [-e2[1]! / l2, e2[0]! / l2];
    const m = [n1[0]! + n2[0]!, n1[1]! + n2[1]!];
    const lm = Math.hypot(m[0]!, m[1]!) || 1;
    const cos = (m[0]! * n1[0]! + m[1]! * n1[1]!) / lm;
    const k = d / Math.max(0.5, cos);
    return p2([p[0] + (m[0]! / lm) * k, p[1] + (m[1]! / lm) * k]);
  });
}

/** ⭕️ A circle ring of `n` points. */
export const circle = (cx: number, cy: number, r: number, n: number): V2[] => Array.from({ length: n }, (_, i) => p2([cx + r * Math.cos((2 * Math.PI * i) / n), cy + r * Math.sin((2 * Math.PI * i) / n)]));

/** ▢️ A rounded rectangle ring centred on (cx, cy). */
export function roundRect(cx: number, cy: number, w: number, h: number, r: number, per = 4): V2[] {
  const out: V2[] = [];
  const corners: [number, number, number][] = [[cx + w / 2 - r, cy + h / 2 - r, 0], [cx - w / 2 + r, cy + h / 2 - r, 90], [cx - w / 2 + r, cy - h / 2 + r, 180], [cx + w / 2 - r, cy - h / 2 + r, 270]];
  for (const [x, y, a0] of corners) for (let k = 0; k <= per; k++) out.push(p2([x + r * Math.cos((a0 + (90 * k) / per) * RAD), y + r * Math.sin((a0 + (90 * k) / per) * RAD)]));
  return out.filter((p, i) => { const q = out[(i + 1) % out.length]!; return Math.hypot(p[0] - q[0], p[1] - q[1]) > 1e-7; });
}

/** 🦴️ The rest frames of the rig, keyed by bone. */
export function frames(rig: Json): Record<string, Frame> {
  const solved = restFrames(rig as never) as Record<string, { frame: Frame }>;
  return Object.fromEntries(Object.entries(solved).map(([id, s]) => [id, s.frame]));
}

/** 🌍️ A bone-local point in the world at rest. */
export const toWorld = (f: Frame, p: readonly number[]): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3;
/** 🏠️ A world point in the frame of a bone. */
export function toLocal(f: Frame, p: readonly number[]): V3 {
  const d = [p[0]! - f.o[0], p[1]! - f.o[1], p[2]! - f.o[2]];
  const dot = (a: V3) => d[0]! * a[0] + d[1]! * a[1] + d[2]! * a[2];
  return [dot(f.u), dot(f.v), dot(f.w)];
}

export type Mesh = { id: string; bone: string; world: V3[]; tris: [number, number, number][] };

/** 🧊️ The kernel tessellation of a solid, carried to the world at rest. */
export function meshOf(solid: Json, f: Frame, quality: Json): Mesh {
  const g = tessellateSolid({ ...solid, node: solid.bone } as never, quality as never) as { positions: ArrayLike<number>; indices: ArrayLike<number> };
  const world: V3[] = [];
  for (let i = 0; i < g.positions.length; i += 3) world.push(toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]));
  const tris: [number, number, number][] = [];
  for (let i = 0; i < g.indices.length; i += 3) tris.push([g.indices[i]!, g.indices[i + 1]!, g.indices[i + 2]!]);
  return { id: solid.id, bone: solid.bone, world, tris };
}

/** 📏️ Surface area, enclosed volume and the volume centroid of a closed mesh. */
export function meshStats(m: Mesh): { area: number; volume: number; centroid: V3 } {
  let area = 0;
  let volume = 0;
  const c: V3 = [0, 0, 0];
  for (const [i, j, k] of m.tris) {
    const a = m.world[i]!;
    const b = m.world[j]!;
    const d = m.world[k]!;
    const e1 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const e2 = [d[0] - a[0], d[1] - a[1], d[2] - a[2]];
    const cr = [e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!];
    area += Math.hypot(cr[0]!, cr[1]!, cr[2]!) / 2;
    const v6 = a[0] * (b[1] * d[2] - b[2] * d[1]) - a[1] * (b[0] * d[2] - b[2] * d[0]) + a[2] * (b[0] * d[1] - b[1] * d[0]);
    volume += v6 / 6;
    for (let q = 0; q < 3; q++) c[q] += (v6 / 6) * ((a[q]! + b[q]! + d[q]!) / 4);
  }
  return { area, volume: Math.abs(volume), centroid: volume === 0 ? [0, 0, 0] : (c.map((x) => x / volume) as V3) };
}

/** 🗺️ Paper coordinates of a world point in a view: elevation in the rest frame of the drawing bone, front (lateral, height), plan (ahead, lateral). */
export function paper(view: View, p: V3, f: Frame): V2 {
  if (view === "elevation") {
    const l = toLocal(f, p);
    return [l[0], l[1]];
  }
  return view === "front" ? [p[2], p[1]] : [p[0], p[2]];
}

/** 🎯️ Depth of a world point towards the viewer of a view (larger is nearer): elevation seen from -z, front from +x, plan from above. */
export const nearness = (view: View, p: V3): number => (view === "elevation" ? -p[2] : view === "front" ? p[0] : p[1]);

/** ✂️ Douglas-Peucker of an open polyline. */
function dp(points: V2[], eps: number): V2[] {
  if (points.length < 3) return points;
  const [a, b] = [points[0]!, points[points.length - 1]!];
  let best = -1;
  let idx = 0;
  const L = Math.hypot(b[0] - a[0], b[1] - a[1]);
  for (let i = 1; i < points.length - 1; i++) {
    const p = points[i]!;
    const d = L < 1e-12 ? Math.hypot(p[0] - a[0], p[1] - a[1]) : Math.abs((b[0] - a[0]) * (a[1] - p[1]) - (a[0] - p[0]) * (b[1] - a[1])) / L;
    if (d > best) [best, idx] = [d, i];
  }
  if (best <= eps) return [a, b];
  return [...dp(points.slice(0, idx + 1), eps).slice(0, -1), ...dp(points.slice(idx), eps)];
}

/** ✂️ A closed ring simplified within `eps`, split at its two farthest points so the result keeps its extent. */
export function simplifyRing(ring: V2[], eps: number): V2[] {
  const r = ring.length > 1 && ring[0]![0] === ring[ring.length - 1]![0] && ring[0]![1] === ring[ring.length - 1]![1] ? ring.slice(0, -1) : ring;
  if (r.length <= 4) return r.map(p2);
  let far = 0;
  let fd = -1;
  for (let i = 0; i < r.length; i++) {
    const d = Math.hypot(r[i]![0] - r[0]![0], r[i]![1] - r[0]![1]);
    if (d > fd) [fd, far] = [d, i];
  }
  const a = dp(r.slice(0, far + 1), eps);
  const b = dp([...r.slice(far), r[0]!], eps);
  const out = [...a.slice(0, -1), ...b.slice(0, -1)].map(p2);
  return out.length >= 3 ? out : r.map(p2);
}

/** 🧮️ The union of triangles by polygon-clipping on a grid of `grid` metres, in chunks. */
function clipUnion(raw: V2[][], grid: number): V2[][][] {
  const snap = (v: number) => Math.round(v / grid) * grid;
  const tris = raw.map((t) => [ccw(t.map(([x, y]) => [snap(x), snap(y)] as V2))]).filter((t) => Math.abs(ringArea(t[0]!)) > grid * grid);
  if (tris.length === 0) return [];
  let acc: V2[][][] = [tris[0]!];
  const CHUNK = 300;
  for (let s = 1; s < tris.length; s += CHUNK) acc = polygonClipping.union(acc as never, ...(tris.slice(s, s + CHUNK) as never[])) as unknown as V2[][][];
  return acc;
}

/** 🌑️ The projected outline of meshes in a view: the union of every projected triangle as polygons of rings (outer first, then holes), simplified within `eps` metres; rings smaller than `minArea` are dropped. polygon-clipping on a micrometre grid, then a coarser grid, then the product's own slab union when the sweep line fails. */
export function outline(meshes: readonly Mesh[], view: View, f: Frame, eps: number, minArea = 1e-7): V2[][][] {
  const raw: V2[][] = [];
  for (const m of meshes)
    for (const [i, j, k] of m.tris) {
      const t = [paper(view, m.world[i]!, f), paper(view, m.world[j]!, f), paper(view, m.world[k]!, f)];
      if (Math.abs(ringArea(t)) < 1e-11) continue;
      raw.push(t);
    }
  if (raw.length === 0) return [];
  let acc: V2[][][] | null = null;
  for (const grid of [1e-6, 4e-6, 2e-5]) {
    try {
      acc = clipUnion(raw, grid);
      break;
    } catch {
      acc = null;
    }
  }
  if (acc === null) {
    const rings = unionPolygons(raw.map((t) => t.map(([x, y]) => ({ x, y }))));
    const outer = rings.filter((r) => polyArea(r) > 0).map((r) => r.map((p) => [p.x, p.y] as V2));
    const holes = rings.filter((r) => polyArea(r) < 0).map((r) => r.map((p) => [p.x, p.y] as V2));
    acc = outer.map((o) => [o, ...holes.filter((h) => inside(o, h[0]!))]);
  }
  const out: V2[][][] = [];
  for (const poly of acc) {
    const rings = poly.map((ring) => simplifyRing(ring as V2[], eps)).filter((ring) => Math.abs(ringArea(ring)) >= minArea);
    if (rings.length > 0 && Math.abs(ringArea(poly[0] as V2[])) >= minArea) out.push(rings);
  }
  return out;
}
const polyArea = (r: readonly { x: number; y: number }[]) => ringArea(r.map((p) => [p.x, p.y] as V2));
/** 📍️ Even-odd point in ring. */
function inside(ring: readonly V2[], p: V2): boolean {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a[1] > p[1] !== b[1] > p[1] && p[0] < ((b[0] - a[0]) * (p[1] - a[1])) / (b[1] - a[1]) + a[0]) c = !c;
  }
  return c;
}

/** ✏️ The shape geometry of projected polygons: one closed poly for a single ring, else an even-odd path with one subpath per ring. */
export function shapeGeometry(polys: V2[][][]): Json {
  const rings = polys.flat();
  if (rings.length === 1) return { kind: "poly", points: rings[0]!.map(p2), closed: true };
  return { kind: "path", d: rings.map((ring) => `M ${ring.map(([x, y]) => `${r5(x)} ${r5(y)}`).join(" L ")} Z`).join(" ") };
}

/** 📦️ World bounds of meshes. */
export function bounds(meshes: readonly Mesh[]): { lo: V3; hi: V3 } {
  const lo: V3 = [Infinity, Infinity, Infinity];
  const hi: V3 = [-Infinity, -Infinity, -Infinity];
  for (const m of meshes)
    for (const p of m.world)
      for (let k = 0; k < 3; k++) {
        lo[k] = Math.min(lo[k]!, p[k]!);
        hi[k] = Math.max(hi[k]!, p[k]!);
      }
  return { lo, hi };
}
