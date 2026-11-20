/** 🧰️ World-space kit of the round-3 hornbeam hedge: clipped panels (superelliptic lofts), stems (swept tubes), flares, cushions; their bone-local solids and their orthographic outlines for the 2D redraw.
 * Every form is stated by hand in `r3-hornbeam-hedge-model.ts`; this file only converts and projects.
 * @see ./round3-brief.md
 * @see ./design.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

export type View = "elevation" | "front" | "plan";
export type Look = { material: string; tone: number; role?: string; flags?: Json };
export type Tube = { id: string; bone: string; look: Look; points: { p: V3; r: number }[] };
export type Section = { y: number; cx: number; cz: number; hx: number; hz: number; e: number };
export type Panel = { id: string; bone: string; look: Look; sections: Section[] };
export type Blob = { id: string; bone: string; look: Look; c: V3; radii: V3 };
export type Flare = { id: string; bone: string; look: Look; c: V3; profile: [number, number][] };

export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
const dot3 = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross3 = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

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

/** 🌿️ A tube through hand-stated world stations (point, radius). */
export const tube = (id: string, bone: string, look: Look, points: [number, number, number, number][]): Tube => ({ id, bone, look, points: points.map(([x, y, z, r]) => ({ p: [x, y, z], r })) });

/** 🪵️ A thin ridge running on the surface of a tube at the azimuth `deg` about its axis, from fraction `f0` to `f1` of its stations, radius `rr`, standing proud by `proud`. */
export function ridge(id: string, of: Tube, look: Look, deg: number, f0: number, f1: number, rr: number, proud: number, twist = 0): Tube {
  const n = of.points.length;
  const pts: { p: V3; r: number }[] = [];
  for (let i = 0; i < n; i++) {
    const s = i / (n - 1);
    if (s < f0 - 1e-9 || s > f1 + 1e-9) continue;
    const t = unit(sub(of.points[Math.min(n - 1, i + 1)]!.p, of.points[Math.max(0, i - 1)]!.p));
    const ref: V3 = Math.abs(t[1]) < 0.8 ? [0, 1, 0] : [1, 0, 0];
    const x = unit(sub(ref, mul(t, dot3(ref, t))));
    const y = cross3(t, x);
    const a = ((deg + twist * s) * Math.PI) / 180;
    const off = add(mul(x, Math.cos(a)), mul(y, Math.sin(a)));
    pts.push({ p: add(of.points[i]!.p, mul(off, of.points[i]!.r + proud - rr)), r: rr });
  }
  return { id, bone: of.bone, look, points: pts };
}

const base = (id: string, bone: string, look: Look): Json => ({ id, kind: "", bone, material: look.material, role: look.role ?? "body", tone: look.tone });
const PANEL_ROTATION = (f: Frame): [number, number, number, number] => quaternion(unit(dirToLocal(f, [0, 0, 1])), unit(dirToLocal(f, [1, 0, 0])), unit(dirToLocal(f, [0, 1, 0])));

/** 🧊️ The bone-local loft of a tube. */
export function tubeSolid(t: Tube, frames: Record<string, Frame>): Json {
  const f = frames[t.bone]!;
  const loc = (p: V3): V3 => toLocal(f, p).map(r5) as V3;
  const n = t.points.length;
  const travel = unit(sub(t.points[n - 1]!.p, t.points[0]!.p));
  const ref: V3 = Math.abs(travel[1]) < 0.8 ? [0, 1, 0] : [0, 0, 1];
  const sections = t.points.map((s, i) => {
    const z = unit(sub(t.points[Math.min(n - 1, i + 1)]!.p, t.points[Math.max(0, i - 1)]!.p));
    const x = unit(sub(ref, mul(z, dot3(ref, z))));
    const y = cross3(z, x);
    return { at: loc(s.p), rx: r5(s.r), ry: r5(s.r), rotation: quaternion(unit(dirToLocal(f, x)), unit(dirToLocal(f, y)), unit(dirToLocal(f, z))) };
  });
  return { ...base(t.id, t.bone, t.look), kind: "loft", sections, capStart: true, capEnd: true, ...(t.look.flags ?? {}) };
}

/** 🧱️ The bone-local loft of a clipped panel: horizontal superelliptic sections (half thickness `hz` across the row, half length `hx` along it) stacked upwards. */
export function panelSolid(p: Panel, frames: Record<string, Frame>): Json {
  const f = frames[p.bone]!;
  const q = PANEL_ROTATION(f);
  const sections = p.sections.map((s) => ({ at: toLocal(f, [s.cx, s.y, s.cz]).map(r5), rx: r5(s.hz), ry: r5(s.hx), exponent: s.e, rotation: q }));
  return { ...base(p.id, p.bone, p.look), kind: "loft", sections, capStart: true, capEnd: true, ...(p.look.flags ?? {}) };
}

/** 🥚️ The bone-local ellipsoid of a blob with world semi-axes (x, y, z). */
export function blobSolid(b: Blob, frames: Record<string, Frame>): Json {
  const f = frames[b.bone]!;
  const cols = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].map((a) => unit(dirToLocal(f, a as V3))) as [V3, V3, V3];
  return { ...base(b.id, b.bone, b.look), kind: "ellipsoid", radii: b.radii.map(r5), transform: { translation: toLocal(f, b.c).map(r5), rotation: quaternion(cols[0], cols[1], cols[2]) }, ...(b.look.flags ?? {}) };
}

/** 🌰️ The bone-local revolve of a root flare about the world vertical through `c`: profile [height above c, radius]. */
export function flareSolid(fl: Flare, frames: Record<string, Frame>): Json {
  const f = frames[fl.bone]!;
  const profile = fl.profile.map(([a, r]) => [r5(a), r5(r)]);
  return { ...base(fl.id, fl.bone, fl.look), kind: "revolve", profile, axis: dirToLocal(f, [0, 1, 0]).map(r5), origin: toLocal(f, fl.c).map(r5), ...(fl.look.flags ?? {}) };
}

export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });

/** ⭕️ A superellipse ring |x/a|^e + |y/b|^e = 1 about `c`. */
export function superRing(c: P2, a: number, b: number, e: number, n = 72): P2[] {
  return Array.from({ length: n }, (_, i) => {
    const t = (2 * Math.PI * i) / n;
    const [ct, st] = [Math.cos(t), Math.sin(t)];
    return { x: c.x + a * Math.sign(ct) * Math.abs(ct) ** (2 / e), y: c.y + b * Math.sign(st) * Math.abs(st) ** (2 / e) };
  });
}

/** 🧱️ The outline polygons of a panel in a view: the exact side profile through the section extremes (elevation, front), the union of the section rings (plan). */
export function panelPolygons(view: View, p: Panel): P2[][] {
  const s = p.sections;
  if (view === "plan") return s.map((q) => superRing({ x: q.cx, y: q.cz }, q.hx, q.hz, q.e));
  const half = (q: Section) => (view === "elevation" ? [q.cx - q.hx, q.cx + q.hx] : [q.cz - q.hz, q.cz + q.hz]);
  return [[...s.map((q) => ({ x: half(q)[0]!, y: q.y })), ...[...s].reverse().map((q) => ({ x: half(q)[1]!, y: q.y }))]];
}

/** 🥚️ The outline of an axis-aligned blob in a view. */
export function blobPolygon(view: View, b: Blob, n = 48): P2[] {
  const c = project(view, b.c);
  const [ax, ay] = view === "elevation" ? [b.radii[0], b.radii[1]] : view === "front" ? [b.radii[2], b.radii[1]] : [b.radii[0], b.radii[2]];
  return superRing(c, ax, ay, 2, n);
}

/** 🌰️ The outline of a flare in a view (a disc in plan). */
export function flarePolygon(view: View, fl: Flare): P2[] {
  const rmax = Math.max(...fl.profile.map(([, r]) => r));
  if (view === "plan") return superRing({ x: fl.c[0], y: fl.c[2] }, rmax, rmax, 2, 36);
  const c = project(view, fl.c);
  return [...fl.profile.map(([a, r]) => ({ x: c.x - r, y: c.y + a })), ...[...fl.profile].reverse().map(([a, r]) => ({ x: c.x + r, y: c.y + a }))];
}

const circle = (c: P2, r: number, n = 20, phase = 0.13): P2[] => Array.from({ length: n }, (_, i) => ({ x: c.x + r * Math.cos(phase + (2 * Math.PI * i) / n), y: c.y + r * Math.sin(phase + (2 * Math.PI * i) / n) }));

/** 🪵️ The polygons whose union is the outline of a tube in a view. */
export function tubePolygons(view: View, t: Tube): P2[][] {
  const out: P2[][] = [];
  const pts = t.points.map((s) => ({ c: project(view, s.p), r: s.r, p: s.p }));
  for (let i = 0; i + 1 < pts.length; i++) {
    const [a, b] = [pts[i]!, pts[i + 1]!];
    const dx = b.c.x - a.c.x, dy = b.c.y - a.c.y, d = Math.hypot(dx, dy);
    const endOn = d < 0.5 * len(sub(b.p, a.p)) || d < 1e-6;
    if (d > 1e-6) {
      const [ux, uy] = [dx / d, dy / d];
      out.push([
        { x: a.c.x - uy * a.r, y: a.c.y + ux * a.r },
        { x: a.c.x + uy * a.r, y: a.c.y - ux * a.r },
        { x: b.c.x + uy * b.r, y: b.c.y - ux * b.r },
        { x: b.c.x - uy * b.r, y: b.c.y + ux * b.r },
      ]);
    }
    if (i > 0) out.push(circle(a.c, a.r * 0.999, 16, 0.13 + i));
    if (endOn) out.push(circle(a.c, a.r * 0.999, 20, 0.13 + i), circle(b.c, b.r * 0.999, 20, 0.31 + i));
  }
  return out;
}

const ringArea = (ring: readonly P2[]): number => ring.reduce((s, p, i) => s + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;

/** ✏️ The drawn outline of a set of polygons: their union, small holes dropped, every ring simplified to `tolerance` m; outer rings first, largest first. */
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

/** 🔁️ Paper points of a view carried to the drawing frame of a shape: bone-local (u, v) in the elevation, world rest coordinates in front and plan. */
export function toDrawing(view: View, frame: Frame, rings: P2[][]): P2[][] {
  if (view !== "elevation") return rings;
  return rings.map((ring) => ring.map((p) => {
    const l = toLocal(frame, [p.x, p.y, frame.o[2]]);
    return { x: l[0], y: l[1] };
  }));
}
