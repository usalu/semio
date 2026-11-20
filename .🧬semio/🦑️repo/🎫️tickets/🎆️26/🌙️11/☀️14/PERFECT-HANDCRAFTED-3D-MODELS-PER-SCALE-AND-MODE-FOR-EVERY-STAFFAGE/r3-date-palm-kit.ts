/** 🧰️ World-space kit of the round-3 date palm: vector helpers, frames, quaternions, tube and ellipsoid outlines, polygon union and simplification. It only converts and projects; every form is stated by hand in `r3-date-palm-model.ts`.
 * @see ./round3-brief.md
 * @see ./design.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, projectWorld, simplifyRing, toLocal, type Frame, type Json, type P2, type V3, type View } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

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
export const dirOf = (azDeg: number, elDeg: number): V3 => [Math.cos(elDeg * D2R) * Math.cos(azDeg * D2R), Math.sin(elDeg * D2R), -Math.cos(elDeg * D2R) * Math.sin(azDeg * D2R)];
export const upOf = (azDeg: number, elDeg: number): V3 => [-Math.sin(elDeg * D2R) * Math.cos(azDeg * D2R), Math.cos(elDeg * D2R), Math.sin(elDeg * D2R) * Math.sin(azDeg * D2R)];

/** 📈️ Piecewise linear interpolation in a table of [x, y] rows sorted by x. */
export function table(rows: readonly (readonly [number, number])[], x: number): number {
  if (x <= rows[0]![0]) return rows[0]![1];
  for (let i = 1; i < rows.length; i++) if (x <= rows[i]![0]) {
    const [a, b] = [rows[i - 1]!, rows[i]!];
    return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]);
  }
  return rows[rows.length - 1]![1];
}

/** 🧭️ The unit quaternion [x, y, z, w] of a rotation matrix given by its three columns. */
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

/** 🧊️ The quaternion of the world axes (x, y, z) expressed in the bone frame `f`. */
export const localRotation = (f: Frame, x: V3, y: V3, z: V3): [number, number, number, number] => quaternion(unit(dirToLocal(f, x)), unit(dirToLocal(f, y)), unit(dirToLocal(f, z)));
export const localPoint = (f: Frame, p: V3): V3 => toLocal(f, p).map(r5) as V3;

/** 🎚️ One section of a world-space tube: centre `p`, half extents `a` along `x` and `b` along `y`, section plane normal `t` (x, y, t right-handed). */
export type Section = { p: V3; x: V3; y: V3; a: number; b: number; n?: number };

/** 🪵️ A world-space loft: ordered sections, the bone that carries it and its solid fields. */
export type Tube = { id: string; bone: string; sections: Section[]; fields: Json };

/** 🧊️ The bone-local loft of a tube. */
export function loftOf(tube: Tube, f: Frame): Json {
  const sections = tube.sections.map((s) => {
    const z = cross(s.x, s.y);
    return { at: localPoint(f, s.p), rx: r5(Math.max(s.a, 1e-4)), ry: r5(Math.max(s.b, 1e-4)), ...(s.n !== undefined && s.n !== 2 ? { exponent: s.n } : {}), rotation: localRotation(f, s.x, s.y, z) };
  });
  return { id: tube.id, kind: "loft", bone: tube.bone, ...tube.fields, sections, capStart: true, capEnd: true };
}

/** ⭕️ Sections along a polyline of centres with circular-ish radii: tangent from neighbours, x axis from a reference direction projected perpendicular. */
export function sweep(points: readonly { p: V3; a: number; b?: number; n?: number }[], ref: V3): Section[] {
  return points.map((s, i) => {
    const t = unit(sub(points[Math.min(points.length - 1, i + 1)]!.p, points[Math.max(0, i - 1)]!.p));
    let r = sub(ref, mul(t, dot(ref, t)));
    if (len(r) < 1e-6) r = Math.abs(t[1]) < 0.9 ? sub([0, 1, 0], mul(t, t[1])) : sub([1, 0, 0], mul(t, t[0]));
    const x = unit(r);
    return { p: s.p, x, y: cross(t, x), a: s.a, b: s.b ?? s.a, n: s.n };
  });
}

export const project = (view: View, p: V3): P2 => projectWorld(view, p);
const VIEW_TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 👁️ How near a point is to the viewer of a view (larger is nearer). */
export const nearness = (view: View, p: V3): number => dot(p, VIEW_TOWARD[view]);

const ringArea = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

/** 🔷️ The outline of one tube section projected to a view: the projected superellipse (an ellipse for exponent 2). */
function sectionRing(view: View, s: Section, n = 20): P2[] {
  const e = s.n ?? 2;
  return Array.from({ length: n }, (_, i) => {
    const t = (2 * Math.PI * i) / n + 0.11;
    const [c, d] = [Math.cos(t), Math.sin(t)];
    const k = Math.pow(Math.pow(Math.abs(c), e) + Math.pow(Math.abs(d), e), -1 / e);
    return project(view, add(s.p, lin(s.x, s.a * c * k, s.y, s.b * d * k)));
  });
}

const hull = (pts: P2[]): P2[] => {
  const p = [...pts].sort((a, b) => a.x - b.x || a.y - b.y);
  const crossZ = (o: P2, a: P2, b: P2) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [];
  for (const q of p) {
    while (lower.length >= 2 && crossZ(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  const upper: P2[] = [];
  for (const q of [...p].reverse()) {
    while (upper.length >= 2 && crossZ(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
};

/** 🪵️ The polygons whose union is the outline of a tube in a view: the convex hull of every pair of neighbouring projected sections. */
export function tubePolygons(view: View, tube: Tube): P2[][] {
  const rings = tube.sections.map((s) => sectionRing(view, s));
  const out: P2[][] = [];
  for (let i = 0; i + 1 < rings.length; i++) out.push(hull([...rings[i]!, ...rings[i + 1]!]));
  if (rings.length === 1) out.push(hull(rings[0]!));
  return out;
}

/** 🥚️ An ellipsoid in world space. */
export type Blob = { id: string; bone: string; c: V3; axes: [V3, V3, V3]; radii: V3; fields: Json };

/** 🥚️ The bone-local ellipsoid of a blob. */
export const ellipsoidOf = (b: Blob, f: Frame): Json => ({ id: b.id, kind: "ellipsoid", bone: b.bone, ...b.fields, radii: b.radii.map(r5), transform: { translation: localPoint(f, b.c), rotation: localRotation(f, b.axes[0], b.axes[1], b.axes[2]) } });

/** ⭕️ The exact orthographic outline of an ellipsoid in a view. */
export function blobOutline(view: View, b: Blob, n = 40): P2[] {
  const pts: P2[] = [];
  for (let i = 0; i < 24; i++) for (let j = 1; j < 12; j++) {
    const [th, ph] = [(2 * Math.PI * i) / 24, (Math.PI * j) / 12];
    pts.push(project(view, add(b.c, add(add(mul(b.axes[0], b.radii[0] * Math.sin(ph) * Math.cos(th)), mul(b.axes[1], b.radii[1] * Math.sin(ph) * Math.sin(th))), mul(b.axes[2], b.radii[2] * Math.cos(ph))))));
  }
  for (const s of [-1, 1]) pts.push(project(view, add(b.c, mul(b.axes[2], s * b.radii[2]))));
  return hull(pts).slice(0, Math.max(n, 3) * 10);
}

/** ✏️ The drawn outline of a set of polygons: their union, small holes dropped, every ring simplified to `tolerance`; outer rings first, largest first. */
export function outline(polygons: P2[][], tolerance: number, minArea: number): P2[][] {
  if (polygons.length === 0) return [];
  const ccw = polygons.filter((p) => p.length >= 3 && Math.abs(ringArea(p)) > 1e-10).map((p) => (ringArea(p) < 0 ? [...p].reverse() : p));
  const rings = unionPolygons(ccw).map((ring) => ({ ring, area: ringArea(ring) }));
  const kept = rings.filter((r) => Math.abs(r.area) > minArea);
  kept.sort((a, b) => (a.area > 0 ? 0 : 1) - (b.area > 0 ? 0 : 1) || Math.abs(b.area) - Math.abs(a.area));
  return kept.map((r) => simplifyRing(r.ring, tolerance)).filter((r) => r.length >= 3);
}

export const hullOf = hull;
export const pathOf = (rings: P2[][], closed = true): string => rings.map((ring) => ring.map((p, i) => `${i === 0 ? "M" : "L"}${r4(p.x)} ${r4(p.y)}`).join("") + (closed ? "Z" : "")).join("");
export const pointsOf = (ring: P2[]): [number, number][] => ring.map((p) => [r4(p.x), r4(p.y)]);
