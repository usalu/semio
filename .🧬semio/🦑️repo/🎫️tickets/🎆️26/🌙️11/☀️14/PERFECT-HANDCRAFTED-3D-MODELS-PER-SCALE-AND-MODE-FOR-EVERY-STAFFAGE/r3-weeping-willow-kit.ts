/** 🧰️ World-space kit of the round-3 weeping willow: lofts with stated sections (stem, limbs, branches, hanging tresses), ellipsoid lobes, their conversion to bone-local solids and their exact orthographic outlines.
 * Every form is stated by hand in `r3-weeping-willow-model.ts`; this file only converts and projects.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-weeping-willow.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

export type View = "elevation" | "front" | "plan";
export type Sec = { p: V3; rx: number; ry: number; x: V3 };
export type Loft = { kind: "loft"; id: string; bone: string; material: string; tone: number; group: string; secs: Sec[]; flags?: Json };
export type Lobe = { kind: "lobe"; id: string; bone: string; material: string; tone: number; group: string; c: V3; axes: [V3, V3, V3]; radii: V3 };
export type Part = Loft | Lobe;

export const D2R = Math.PI / 180;
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const dot3 = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross3 = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const radial = (az: number): V3 => [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
export const tangential = (az: number): V3 => [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
export const dirOf = (az: number, el: number): V3 => [Math.cos(el * D2R) * Math.cos(az * D2R), Math.sin(el * D2R), Math.cos(el * D2R) * Math.sin(az * D2R)];

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
  return q.map(r5) as [number, number, number, number];
}

/** 📐️ The orthonormal frame (x, y, travel) of section `i` of a loft: travel from the neighbouring sections, x the stated across-axis made square to it. */
export function secFrame(l: Loft, i: number): [V3, V3, V3] {
  const n = l.secs.length;
  const z = unit(sub(l.secs[Math.min(n - 1, i + 1)]!.p, l.secs[Math.max(0, i - 1)]!.p));
  const x = unit(sub(l.secs[i]!.x, mul(z, dot3(l.secs[i]!.x, z))));
  return [x, cross3(z, x), z];
}

/** 🎯️ The world point of section `i` of a loft at the section angle `phi` (radians, from its x-axis). */
export function secPoint(l: Loft, i: number, phi: number): V3 {
  const [x, y] = secFrame(l, i);
  const s = l.secs[i]!;
  return add(add(s.p, mul(x, s.rx * Math.cos(phi))), mul(y, s.ry * Math.sin(phi)));
}

/** 🌿️ An ellipsoid lobe on the crown: centre at azimuth `az` about the crown axis (`cx`, `cz`) whose outer face reaches `reach`, at height `y`; semi-axes `a` radial, `b` vertical, `c` tangential; `tilt` droops the radial axis outwards. */
export function ringLobe(id: string, bone: string, material: string, tone: number, group: string, cx: number, cz: number, az: number, reach: number, y: number, a: number, b: number, c: number, tilt = 0): Lobe {
  const rad = radial(az), up: V3 = [0, 1, 0], tan = tangential(az);
  const [ct, st] = [Math.cos(tilt * D2R), Math.sin(tilt * D2R)];
  const rad1 = sub(mul(rad, ct), mul(up, st));
  const up1 = add(mul(up, ct), mul(rad, st));
  const rc = reach - Math.hypot(a * ct, b * st);
  return { kind: "lobe", id, bone, material, tone, group, c: [cx + rc * rad[0], y, cz + rc * rad[2]], axes: [rad1, up1, tan], radii: [a, b, c] };
}

/** 🎯️ The point of a lobe's surface in the world direction `d` from its centre. */
export function surfacePoint(l: Lobe, d: V3): V3 {
  const q = Math.hypot(dot3(d, l.axes[0]) / l.radii[0], dot3(d, l.axes[1]) / l.radii[1], dot3(d, l.axes[2]) / l.radii[2]);
  return add(l.c, mul(d, 1 / q));
}

/** 🧪️ Whether a world point lies inside a part (lobes exactly, lofts by their nearest section ellipse). */
export function contains(part: Part, p: V3, grow = 0): boolean {
  if (part.kind === "lobe") return Math.hypot(...([0, 1, 2].map((i) => dot3(sub(p, part.c), part.axes[i]!) / (part.radii[i]! + grow)) as [number, number, number])) <= 1;
  for (let i = 0; i + 1 < part.secs.length; i++) {
    const [a, b] = [part.secs[i]!, part.secs[i + 1]!];
    const ab = sub(b.p, a.p);
    const t = dot3(sub(p, a.p), ab) / (dot3(ab, ab) || 1);
    if (t < 0 || t > 1) continue;
    const c = add(a.p, mul(ab, t));
    const [x, y] = secFrame(part, t < 0.5 ? i : i + 1);
    const rx = a.rx + (b.rx - a.rx) * t + grow, ry = a.ry + (b.ry - a.ry) * t + grow;
    const d = sub(p, c);
    if (Math.hypot(dot3(d, x) / rx, dot3(d, y) / ry) <= 1) return true;
  }
  return false;
}

/** 🧊️ The bone-local solid of a part. */
export function solidOf(part: Part, frames: Record<string, Frame>): Json {
  const f = frames[part.bone]!;
  const loc = (p: V3): V3 => toLocal(f, p).map(r5) as V3;
  if (part.kind === "loft") {
    const sections = part.secs.map((s, i) => {
      const [x, y, z] = secFrame(part, i);
      return { at: loc(s.p), rx: r5(s.rx), ry: r5(s.ry), rotation: quaternion(unit(dirToLocal(f, x)), unit(dirToLocal(f, y)), unit(dirToLocal(f, z))) };
    });
    return { id: part.id, kind: "loft", bone: part.bone, material: part.material, role: "body", tone: part.tone, sections, capStart: true, capEnd: true, ...(part.flags ?? {}) };
  }
  const cols = part.axes.map((a) => unit(dirToLocal(f, a))) as [V3, V3, V3];
  return { id: part.id, kind: "ellipsoid", bone: part.bone, material: part.material, role: "body", tone: part.tone, radii: part.radii.map(r5), transform: { translation: loc(part.c), rotation: quaternion(cols[0], cols[1], cols[2]) } };
}

export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });
export const VIEW_AXES: Record<View, [V3, V3, V3]> = { elevation: [[1, 0, 0], [0, 1, 0], [0, 0, -1]], front: [[0, 0, 1], [0, 1, 0], [1, 0, 0]], plan: [[1, 0, 0], [0, 0, 1], [0, 1, 0]] };

/** 👁️ How near a point is to the viewer of a view (larger is nearer). */
export const nearness = (view: View, p: V3): number => dot3(p, VIEW_AXES[view][2]);

/** ⭕️ The exact orthographic outline of a lobe in a view; `from`..`to` (degrees) gives an open arc of it. */
export function lobeOutline(view: View, l: Lobe, n = 60, from = 0, to = 360): P2[] {
  const [e1, e2] = VIEW_AXES[view];
  const cols = l.axes.map((a, i) => [dot3(a, e1) * l.radii[i]!, dot3(a, e2) * l.radii[i]!]);
  const q11 = cols.reduce((s, c) => s + c[0]! * c[0]!, 0), q12 = cols.reduce((s, c) => s + c[0]! * c[1]!, 0), q22 = cols.reduce((s, c) => s + c[1]! * c[1]!, 0);
  const theta = 0.5 * Math.atan2(2 * q12, q11 - q22);
  const mean = (q11 + q22) / 2, diff = Math.hypot((q11 - q22) / 2, q12);
  const [ra, rb] = [Math.sqrt(mean + diff), Math.sqrt(Math.max(1e-12, mean - diff))];
  const c = project(view, l.c);
  const open = to - from < 360;
  const out: P2[] = [];
  for (let i = 0; i < (open ? n + 1 : n); i++) {
    const phi = (from + ((to - from) * i) / n) * D2R;
    const dx = Math.cos(phi), dy = Math.sin(phi);
    const lx = dx * Math.cos(theta) + dy * Math.sin(theta), ly = -dx * Math.sin(theta) + dy * Math.cos(theta);
    const rho = 1 / Math.hypot(lx / ra, ly / rb);
    out.push({ x: c.x + rho * dx, y: c.y + rho * dy });
  }
  return out;
}

/** 🛡️ Convex hull (counter-clockwise) of points. */
export function hull(points: P2[]): P2[] {
  const p = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  if (p.length < 3) return p;
  const turn = (o: P2, a: P2, b: P2) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [], upper: P2[] = [];
  for (const q of p) {
    while (lower.length >= 2 && turn(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  for (const q of [...p].reverse()) {
    while (upper.length >= 2 && turn(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** 🪵️ The polygons whose union is the outline of a loft in a view: the hull of every pair of neighbouring sections. */
export function loftPolygons(view: View, l: Loft, n = 28): P2[][] {
  const rings = l.secs.map((_, i) => Array.from({ length: n }, (_, k) => project(view, secPoint(l, i, (2 * Math.PI * k) / n))));
  const out: P2[][] = [];
  for (let i = 0; i + 1 < rings.length; i++) out.push(hull([...rings[i]!, ...rings[i + 1]!]));
  return out;
}

/** 📍️ The centre-line point of a loft at the fraction `f` of its sections. */
export function along3(l: Loft, f: number): V3 {
  const x = f * (l.secs.length - 1);
  const i = Math.min(l.secs.length - 2, Math.floor(x));
  return add(l.secs[i]!.p, mul(sub(l.secs[i + 1]!.p, l.secs[i]!.p), x - i));
}

export const partPolygons = (view: View, part: Part): P2[][] => (part.kind === "lobe" ? [lobeOutline(view, part)] : loftPolygons(view, part));

const ringArea = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

/** ✏️ The drawn outline of a set of polygons: their union, holes below `minHole` m2 dropped, rings simplified to `tolerance` m; outer rings first, largest first. */
export function outline(polygons: P2[][], tolerance: number, minHole: number): P2[][] {
  if (polygons.length === 0) return [];
  const ccw = polygons.map((p) => (ringArea(p) < 0 ? [...p].reverse() : p));
  const rings = unionPolygons(ccw).map((ring) => ({ ring, area: ringArea(ring) }));
  const kept = rings.filter((r) => (r.area > 0 ? r.area > minHole * 0.05 : -r.area > minHole));
  kept.sort((a, b) => (a.area > 0 ? 0 : 1) - (b.area > 0 ? 0 : 1) || Math.abs(b.area) - Math.abs(a.area));
  return kept.map((r) => simplifyRing(r.ring, tolerance));
}

export const ringAreaOf = ringArea;
export const pathOf = (rings: P2[][], closed = true): string => rings.map((ring) => ring.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x)} ${r4(p.y)}`).join(" ") + (closed ? " Z" : "")).join(" ");
export const pointsOf = (ring: P2[]): [number, number][] => ring.map((p) => [r4(p.x), r4(p.y)]);

/** 🔁️ Paper points of a view carried to the drawing frame of a shape: bone-local (u, v) in the elevation, world rest coordinates in front and plan. */
export function toDrawing(view: View, frame: Frame, rings: P2[][]): P2[][] {
  if (view !== "elevation") return rings;
  return rings.map((ring) => ring.map((p) => {
    const l = toLocal(frame, [p.x, p.y, frame.o[2]]);
    return { x: l[0], y: l[1] };
  }));
}

/** 📍️ Whether a paper point lies inside a ring. */
export function inRing(p: P2, ring: P2[]): boolean {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
}
