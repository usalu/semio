/** 🍁️ World-space kit of the round-3 japanese maple: tubes (swept lofts), ellipsoid leaf sprays, their conversion to bone-local solids and their orthographic outlines (conversion code taken over from the apple-tree kit, owned here so that it cannot change under this asset).
 * Every form is stated by hand in `r3-japanese-maple-tree.ts`; this file only converts, smooths and projects.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-japanese-maple.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

export type View = "elevation" | "front" | "plan";
export type Tube = { kind: "tube"; id: string; bone: string; material: string; tone: number; points: { p: V3; r: number }[]; flags?: Json; group: string };
export type Lobe = { kind: "lobe"; id: string; bone: string; material: string; tone: number; c: V3; axes: [V3, V3, V3]; radii: V3; group: string; pad?: boolean };

/** 🥞️ Profile of a leaf-spray pad (height in units of the vertical semi-axis, scale of the horizontal semi-axes): a flattish underside, the widest ring a little below the middle, a domed top; concave, so the pad is convex. */
export const PAD: [number, number][] = [[-1, 0.8], [-0.75, 0.93], [-0.35, 1], [0.15, 0.97], [0.55, 0.8], [0.85, 0.48], [1, 0.1]];
export type Part = Tube | Lobe;

export const D2R = Math.PI / 180;
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const dot3 = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
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

/** 🍃️ A lobe on a ring about the stem axis: centre at azimuth `az` (degrees from +x towards +z), `reach` is the outer radius it touches, `y` its height; semi-axes `a` radial, `b` vertical, `c` tangential; `tilt` droops the radial axis outwards. */
export function ringLobe(id: string, bone: string, material: string, tone: number, group: string, az: number, reach: number, y: number, a: number, b: number, c: number, tilt = 0, axisX = 0): Lobe {
  const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const up: V3 = [0, 1, 0];
  const tan: V3 = [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
  const [ct, st] = [Math.cos(tilt * D2R), Math.sin(tilt * D2R)];
  const rad1 = sub(mul(rad, ct), mul(up, st));
  const up1 = add(mul(up, ct), mul(rad, st));
  const extent = Math.hypot(a * ct, b * st);
  const rc = reach - extent;
  return { kind: "lobe", id, bone, material, tone, group, c: [axisX + rc * rad[0], y, rc * rad[2]], axes: [rad1, up1, tan], radii: [a, b, c] };
}

/** 📌️ The point of a lobe's surface in the world direction `d` from its centre. */
export function surfacePoint(l: Lobe, d: V3): V3 {
  const q = Math.hypot(dot3(d, l.axes[0]) / l.radii[0], dot3(d, l.axes[1]) / l.radii[1], dot3(d, l.axes[2]) / l.radii[2]);
  return add(l.c, mul(d, 1 / q));
}

/** 🌿️ A side branch as a tube: from `start` in direction azimuth `az`, elevation `e0` turning to `e1` and azimuth turning by `bend` over the length `L`, radius `r0` to `r1`. */
export function branch(id: string, bone: string, material: string, tone: number, group: string, start: V3, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend = 0, n = 6, flags?: Json): Tube {
  const points: { p: V3; r: number }[] = [{ p: start, r: r0 * 1.12 }];
  let p = start;
  for (let i = 1; i <= n; i++) {
    const s = (i - 0.5) / n;
    p = add(p, mul(dirOf(az + bend * s, e0 + (e1 - e0) * s), L / n));
    points.push({ p, r: r0 + (r1 - r0) * (i / n) });
  }
  return { kind: "tube", id, bone, material, tone, group, points, ...(flags ? { flags } : {}) };
}

/** 📏️ The point of a tube's centre line at the fraction `f` of its length (by section index). */
export function along(t: Tube, f: number): V3 {
  const x = f * (t.points.length - 1);
  const i = Math.min(t.points.length - 2, Math.floor(x));
  return add(t.points[i]!.p, mul(sub(t.points[i + 1]!.p, t.points[i]!.p), x - i));
}

/** 🧱️ The bone-local solid of a part. */
export function solidOf(part: Part, frames: Record<string, Frame>): Json {
  const f = frames[part.bone]!;
  const loc = (p: V3): V3 => toLocal(f, p).map(r5) as V3;
  if (part.kind === "tube") {
    const n = part.points.length;
    const travel = unit(sub(part.points[n - 1]!.p, part.points[0]!.p));
    const ref: V3 = Math.abs(travel[1]) < 0.8 ? [0, 1, 0] : [0, 0, 1];
    const sections = part.points.map((s, i) => {
      const z = unit(sub(part.points[Math.min(n - 1, i + 1)]!.p, part.points[Math.max(0, i - 1)]!.p));
      const x = unit(sub(ref, mul(z, dot3(ref, z))));
      const y: V3 = [z[1] * x[2] - z[2] * x[1], z[2] * x[0] - z[0] * x[2], z[0] * x[1] - z[1] * x[0]];
      return { at: loc(s.p), rx: r5(s.r), ry: r5(s.r), rotation: quaternion(unit(dirToLocal(f, x)), unit(dirToLocal(f, y)), unit(dirToLocal(f, z))) };
    });
    return { id: part.id, kind: "loft", bone: part.bone, material: part.material, role: "body", tone: part.tone, sections, capStart: true, capEnd: true, ...(part.flags ?? {}) };
  }
  const cols = part.axes.map((a) => unit(dirToLocal(f, a))) as [V3, V3, V3];
  if (part.pad) {
    const q = quaternion(cols[2], cols[0], cols[1]);
    const sections = PAD.map(([h, k]) => ({ at: loc(add(part.c, mul(part.axes[1], h * part.radii[1]))), rx: r5(part.radii[2] * k), ry: r5(part.radii[0] * k), rotation: q }));
    return { id: part.id, kind: "loft", bone: part.bone, material: part.material, role: "body", tone: part.tone, sections, capStart: true, capEnd: true };
  }
  return { id: part.id, kind: "ellipsoid", bone: part.bone, material: part.material, role: "body", tone: part.tone, radii: part.radii.map(r5), transform: { translation: loc(part.c), rotation: quaternion(cols[0], cols[1], cols[2]) } };
}

export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });
const VIEW_AXES: Record<View, [V3, V3, V3]> = { elevation: [[1, 0, 0], [0, 1, 0], [0, 0, -1]], front: [[0, 0, 1], [0, 1, 0], [1, 0, 0]], plan: [[1, 0, 0], [0, 0, 1], [0, 1, 0]] };

/** 👀️ How near a point is to the viewer of a view (larger is nearer). */
export const nearness = (view: View, p: V3): number => dot3(p, VIEW_AXES[view][2]);

/** 🔵️ The outline of a lobe in a view: the exact orthographic shadow of the ellipsoid, as a polygon; `from`..`to` (degrees, counter-clockwise from paper +x) gives an open arc of it. */
export function lobeOutline(view: View, l: Lobe, n = 72, from = 0, to = 360): P2[] {
  if (l.pad) return padOutline(view, l, n);
  const [e1, e2] = VIEW_AXES[view];
  const cols = l.axes.map((a, i) => [dot3(a, e1) * l.radii[i]!, dot3(a, e2) * l.radii[i]!]);
  const q11 = cols.reduce((s, c) => s + c[0]! * c[0]!, 0), q12 = cols.reduce((s, c) => s + c[0]! * c[1]!, 0), q22 = cols.reduce((s, c) => s + c[1]! * c[1]!, 0);
  const theta = 0.5 * Math.atan2(2 * q12, q11 - q22);
  const mean = (q11 + q22) / 2, diff = Math.hypot((q11 - q22) / 2, q12);
  const [ra, rb] = [Math.sqrt(mean + diff), Math.sqrt(Math.max(1e-12, mean - diff))];
  const c = project(view, l.c);
  const open = to - from < 360;
  const out: P2[] = [];
  const steps = open ? n : n - 1;
  for (let i = 0; i <= steps; i++) {
    const phi = (from + ((to - from) * i) / (open ? n : n)) * D2R;
    const dx = Math.cos(phi), dy = Math.sin(phi);
    const lx = dx * Math.cos(theta) + dy * Math.sin(theta), ly = -dx * Math.sin(theta) + dy * Math.cos(theta);
    const rho = 1 / Math.hypot(lx / ra, ly / rb);
    out.push({ x: c.x + rho * dx, y: c.y + rho * dy });
  }
  return out;
}

const circle = (c: P2, r: number, n = 20, phase = 0.13): P2[] => Array.from({ length: n }, (_, i) => ({ x: c.x + r * Math.cos(phase + (2 * Math.PI * i) / n), y: c.y + r * Math.sin(phase + (2 * Math.PI * i) / n) }));

/** 🌳️ The polygons whose union is the outline of a tube in a view: one trapezoid per segment, the two wedges that close a bend at every inner section, and a disc where a segment is seen end-on. */
export function tubePolygons(view: View, t: Tube): P2[][] {
  const out: P2[][] = [];
  const pts = t.points.map((s) => ({ c: project(view, s.p), r: s.r, p: s.p }));
  let last: { left: P2; right: P2 } | null = null;
  for (let i = 0; i + 1 < pts.length; i++) {
    const [a, b] = [pts[i]!, pts[i + 1]!];
    const dx = b.c.x - a.c.x, dy = b.c.y - a.c.y, d = Math.hypot(dx, dy);
    const true3 = len(sub(b.p, a.p));
    const endOn = d < 0.5 * true3 || d < 1e-6;
    if (d > 1e-6) {
      const [ux, uy] = [dx / d, dy / d];
      const e = 0.0007;
      const left = { x: a.c.x - uy * a.r, y: a.c.y + ux * a.r }, right = { x: a.c.x + uy * a.r, y: a.c.y - ux * a.r };
      out.push([
        { x: left.x - ux * e, y: left.y - uy * e },
        { x: right.x - ux * e, y: right.y - uy * e },
        { x: b.c.x + ux * e + uy * b.r, y: b.c.y + uy * e - ux * b.r },
        { x: b.c.x + ux * e - uy * b.r, y: b.c.y + uy * e + ux * b.r },
      ]);
      if (last && !endOn) for (const [p, q] of [[last.left, left], [last.right, right]] as [P2, P2][]) if (Math.abs((p.x - a.c.x) * (q.y - a.c.y) - (q.x - a.c.x) * (p.y - a.c.y)) > 2e-6) out.push([a.c, p, q]);
      last = { left: { x: b.c.x - uy * b.r, y: b.c.y + ux * b.r }, right: { x: b.c.x + uy * b.r, y: b.c.y - ux * b.r } };
    } else last = null;
    if (endOn) out.push(circle(a.c, a.r * 0.999, 20, 0.13 + i));
    if (endOn && i + 2 === pts.length) out.push(circle(b.c, b.r * 0.999, 20, 0.31 + i));
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
  return kept.map((r) => simplifyRing(r.ring, tolerance));
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

/** 〰️ A Catmull-Rom curve through the stations of a tube, `n` sections per span; radii are interpolated linearly. */
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

/** 🔷️ The convex hull of points (monotone chain), counter-clockwise. */
export function hull(points: P2[]): P2[] {
  const p = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const cross = (o: P2, a: P2, b: P2): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [], upper: P2[] = [];
  for (const q of p) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  for (const q of p.reverse()) {
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** 🥞️ The outline of a pad in a view: the hull of its section rings (the pad is convex). */
export function padOutline(view: View, l: Lobe, n = 48): P2[] {
  const pts: P2[] = [];
  for (const [h, k] of PAD) for (let i = 0; i < n; i++) {
    const t = (2 * Math.PI * i) / n;
    pts.push(project(view, add(add(l.c, mul(l.axes[1], h * l.radii[1])), add(mul(l.axes[2], Math.cos(t) * l.radii[2] * k), mul(l.axes[0], Math.sin(t) * l.radii[0] * k)))));
  }
  return hull(pts);
}

/** 🎯️ How deep a point lies in a lobe: below 1 inside (0 at the centre line), for pads measured against the pad profile at the point's height. */
export function depthIn(l: Lobe, p: V3): number {
  const d = sub(p, l.c);
  const [ra, h, rt] = l.axes.map((a, i) => dot3(d, a) / l.radii[i]!) as V3;
  if (!l.pad) return Math.hypot(ra, h, rt);
  if (Math.abs(h) >= 1) return 1 + Math.abs(h);
  const i = PAD.findIndex(([y]) => y >= h);
  const [[y0, k0], [y1, k1]] = [PAD[Math.max(0, i - 1)]!, PAD[Math.max(1, i)]!];
  return Math.hypot(ra, rt) / (k0 + ((k1 - k0) * (h - y0)) / (y1 - y0));
}
