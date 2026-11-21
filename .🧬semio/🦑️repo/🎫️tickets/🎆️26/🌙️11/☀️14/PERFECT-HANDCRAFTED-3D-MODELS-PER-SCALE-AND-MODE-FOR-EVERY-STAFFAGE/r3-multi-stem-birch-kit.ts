/** 🌿️ World-space kit of the round-3 multi-stem birch: one part type (a loft through elliptic sections with stated axes) for stems, branches, strands and leaf plumes, its conversion to a bone-local solid and its orthographic outline (quaternion, smoothing, hull and union code taken over from the japanese-maple kit and owned here so that it cannot change under this asset).
 * Every form is stated by hand in `r3-multi-stem-birch-tree.ts`; this file only converts, smooths and projects.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-multi-stem-birch.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

export type View = "elevation" | "front" | "plan";
export type Sec = { p: V3; x: V3; y: V3; rx: number; ry: number };
export type Group = "stool" | "root" | "base" | "stem" | "leader" | "branch" | "twig" | "tip" | "strand" | "leaf";
/** 🧩️ A part is a loft through its sections, or, when `capsule` is set, the hull of two spheres (its two sections then only carry the end points and radii for measuring and drawing). */
export type Part = { id: string; bone: string; material: string; tone: number; group: Group; sections: Sec[]; flags?: Json; owner?: string; az?: number; capsule?: { p0: V3; p1: V3; r0: number; r1: number } };
export type Mark = { id: string; bone: string; material: string; tone: number; c: V3; axes: [V3, V3, V3]; radii: V3 };

export const D2R = Math.PI / 180;
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const dot3 = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const dirOf = (azDeg: number, elDeg: number): V3 => [Math.cos(elDeg * D2R) * Math.cos(azDeg * D2R), Math.sin(elDeg * D2R), Math.cos(elDeg * D2R) * Math.sin(azDeg * D2R)];

/** 🧮️ The unit quaternion [x, y, z, w] of a rotation matrix given by its three columns. */
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
  return q.map(r5) as [number, number, number, number];
}

/** 〰️ A Catmull-Rom curve through stations (point and radius), `n` sections per span; radii are interpolated linearly. */
export function smooth(points: { p: V3; r: number }[], n: number): { p: V3; r: number }[] {
  if (points.length < 3 || n <= 1) return points;
  const out: { p: V3; r: number }[] = [];
  const at = (i: number): V3 => points[Math.max(0, Math.min(points.length - 1, i))]!.p;
  for (let i = 0; i + 1 < points.length; i++) {
    const [p0, p1, p2, p3] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    for (let k = 0; k < n; k++) {
      const t = k / n, t2 = t * t, t3 = t2 * t;
      const p = [0, 1, 2].map((j) => 0.5 * (2 * p1[j]! + (-p0[j]! + p2[j]!) * t + (2 * p0[j]! - 5 * p1[j]! + 4 * p2[j]! - p3[j]!) * t2 + (-p0[j]! + 3 * p1[j]! - 3 * p2[j]! + p3[j]!) * t3)) as V3;
      out.push({ p, r: points[i]!.r + (points[i + 1]!.r - points[i]!.r) * t });
    }
  }
  out.push(points[points.length - 1]!);
  return out;
}

/** 🪵️ Round sections along a centre line: the section plane is normal to the line, its x axis as horizontal as the line allows. */
export function tubeSections(points: { p: V3; r: number }[]): Sec[] {
  const n = points.length;
  const travel = unit(sub(points[n - 1]!.p, points[0]!.p));
  const ref: V3 = Math.abs(travel[1]) < 0.8 ? [0, 1, 0] : [0, 0, 1];
  return points.map((s, i) => {
    const z = unit(sub(points[Math.min(n - 1, i + 1)]!.p, points[Math.max(0, i - 1)]!.p));
    const x = unit(sub(ref, mul(z, dot3(ref, z))));
    return { p: s.p, x, y: cross(z, x), rx: s.r, ry: s.r };
  });
}

/** 📏️ The point of a part's centre line at the fraction `f` of its length, measured along the line, so that the same fraction is the same place however many sections a scale gives the part. */
export function along(part: Part, f: number): V3 {
  const pts = part.sections.map((s) => s.p);
  const steps = pts.slice(1).map((q, i) => len(sub(q, pts[i]!)));
  let rest = Math.max(0, Math.min(1, f)) * steps.reduce((s, d) => s + d, 0);
  for (let i = 0; i < steps.length; i++) {
    if (rest <= steps[i]! || i === steps.length - 1) return add(pts[i]!, mul(sub(pts[i + 1]!, pts[i]!), steps[i]! > 0 ? Math.min(1, rest / steps[i]!) : 0));
    rest -= steps[i]!;
  }
  return pts[pts.length - 1]!;
}

/** 🧱️ The bone-local solid of a part: a loft, or a capsule. */
export function solidOf(part: Part, frames: Record<string, Frame>): Json {
  const f = frames[part.bone]!;
  if (part.capsule) return { id: part.id, kind: "capsule", bone: part.bone, material: part.material, role: "body", tone: part.tone, p0: toLocal(f, part.capsule.p0).map(r5), p1: toLocal(f, part.capsule.p1).map(r5), r0: r5(part.capsule.r0), r1: r5(part.capsule.r1), ...(part.flags ?? {}) };
  const sections = part.sections.map((s) => {
    const z = cross(s.x, s.y);
    return { at: toLocal(f, s.p).map(r5), rx: r5(s.rx), ry: r5(s.ry), exponent: 2, rotation: quaternion(unit(dirToLocal(f, s.x)), unit(dirToLocal(f, s.y)), unit(dirToLocal(f, z))) };
  });
  return { id: part.id, kind: "loft", bone: part.bone, material: part.material, role: "body", tone: part.tone, sections, capStart: true, capEnd: true, ...(part.flags ?? {}) };
}

/** 🔘️ The bone-local ellipsoid of a bark mark (texture). */
export function markSolid(m: Mark, frames: Record<string, Frame>): Json {
  const f = frames[m.bone]!;
  const cols = m.axes.map((a) => unit(dirToLocal(f, a))) as [V3, V3, V3];
  return { id: m.id, kind: "ellipsoid", bone: m.bone, material: m.material, role: "texture", tone: m.tone, radii: m.radii.map(r5), transform: { translation: toLocal(f, m.c).map(r5), rotation: quaternion(cols[0], cols[1], cols[2]) }, texture: true };
}

/** 🧭️ A world direction in the axes of a bone frame. */
export const dirToLocalSafe = (f: Frame, d: V3): V3 => dirToLocal(f, d);

export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });
const VIEW_AXES: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 👀️ How near a point is to the viewer of a view (larger is nearer). */
export const nearness = (view: View, p: V3): number => dot3(p, VIEW_AXES[view]);

/** 🔷️ The convex hull of points (monotone chain), counter-clockwise. */
export function hull(points: P2[]): P2[] {
  const p = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const cr = (o: P2, a: P2, b: P2): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [], upper: P2[] = [];
  for (const q of p) {
    while (lower.length >= 2 && cr(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  for (const q of p.reverse()) {
    while (upper.length >= 2 && cr(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

export const ringOf = (s: Sec, n = 20): V3[] => Array.from({ length: n }, (_, i) => {
  const t = (2 * Math.PI * (i + 0.5)) / n;
  return add(s.p, add(mul(s.x, s.rx * Math.cos(t)), mul(s.y, s.ry * Math.sin(t))));
});

/** 🌳️ The polygons whose union is the outline of a part in a view: the hull of every two consecutive section rings. */
export function partPolygons(view: View, part: Part, n = 20): P2[][] {
  if (part.capsule) {
    const { p0, p1, r0, r1 } = part.capsule;
    const disc = (c: P2, r: number): P2[] => Array.from({ length: n }, (_, i) => ({ x: c.x + r * Math.cos((2 * Math.PI * (i + 0.5)) / n), y: c.y + r * Math.sin((2 * Math.PI * (i + 0.5)) / n) }));
    return [hull([...disc(project(view, p0), r0), ...disc(project(view, p1), r1)])];
  }
  const rings = part.sections.map((s) => ringOf(s, n).map((p) => project(view, p)));
  const out: P2[][] = [];
  for (let i = 0; i + 1 < rings.length; i++) {
    const h = hull([...rings[i]!, ...rings[i + 1]!]);
    if (h.length >= 3) out.push(h);
  }
  return out;
}

const ringArea = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

/** 🖊️ The drawn outline of a set of polygons: their union, holes smaller than `minHole` m2 dropped, every ring simplified to `tolerance` m; outer rings first, largest first. */
export function outline(polygons: P2[][], tolerance: number, minHole: number): P2[][] {
  if (polygons.length === 0) return [];
  const ccw = polygons.map((p) => (ringArea(p) < 0 ? [...p].reverse() : p));
  const rings = unionPolygons(ccw).map((ring) => ({ ring, area: ringArea(ring) }));
  const kept = rings.filter((r) => (r.area > 0 ? r.area > minHole * 0.05 : -r.area > minHole));
  kept.sort((a, b) => (a.area > 0 ? 0 : 1) - (b.area > 0 ? 0 : 1) || Math.abs(b.area) - Math.abs(a.area));
  return kept.map((r) => simplifyRing(r.ring, tolerance)).filter((r) => r.length >= 3);
}

export const pathOf = (rings: P2[][], closed = true): string => rings.map((ring) => ring.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x)} ${r4(p.y)}`).join(" ") + (closed ? " Z" : "")).join(" ");
export const pointsOf = (ring: P2[]): [number, number][] => ring.map((p) => [r4(p.x), r4(p.y)]);

/** 🔄️ Paper points of a view carried to the drawing frame of a shape: bone-local (u, v) in the elevation, world rest coordinates in front and plan. */
export function toDrawing(view: View, frame: Frame, rings: P2[][]): P2[][] {
  if (view !== "elevation") return rings;
  return rings.map((ring) => ring.map((p) => {
    const l = toLocal(frame, [p.x, p.y, frame.o[2]]);
    return { x: l[0], y: l[1] };
  }));
}

export const inRing = (p: P2, ring: P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};

/** 📦️ The world bounding box of parts (section rings). */
export function bounds(parts: Part[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  const take = (p: V3, r: number): void => {
    for (let k = 0; k < 3; k++) {
      min[k] = Math.min(min[k]!, p[k]! - r);
      max[k] = Math.max(max[k]!, p[k]! + r);
    }
  };
  for (const part of parts) {
    if (part.capsule) {
      take(part.capsule.p0, part.capsule.r0);
      take(part.capsule.p1, part.capsule.r1);
    } else for (const s of part.sections) for (const p of ringOf(s, 16)) take(p, 0);
  }
  return { min, max };
}
