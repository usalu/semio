/** 🧰️ World-space kit of the round-3 ivy on trellis: primitives stated in world metres (x along the trellis, y up, z through it; the elevation looks from -z), their bone-local solids, and their orthographic outlines for the 2D redraw (ruling E1).
 * Every form is stated by hand in `r3-ivy-on-trellis-model.ts`; this file only converts and projects.
 * @see ./round3-brief.md
 * @see ./design.md */
import { join } from "node:path";
import { STAFFAGE, dirToLocal, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly P2[])[]) => P2[][] };

export type View = "elevation" | "front" | "plan";
export type Look = { material: string; tone: number; role?: string; flags?: Json };
type Base = { id: string; bone: string; look: Look };
export type Ell = Base & { k: "ell"; c: V3; r: V3; turn?: number };
export type Box = Base & { k: "box"; c: V3; size: V3; corner?: number };
export type Sec = { c: V3; a: number; b: number; e?: number; ax?: V3; n?: V3 };
export type Loft = Base & { k: "loft"; secs: Sec[] };
export type Ext = Base & { k: "ext"; profile: [number, number][]; holes?: [number, number][][]; o: V3; ex: V3; ey: V3; from: number; to: number };
export type Rev = Base & { k: "rev"; c: V3; profile: [number, number][] };
export type Prim = Ell | Box | Loft | Ext | Rev;

export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const dot3 = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross3 = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const X: V3 = [1, 0, 0], Y: V3 = [0, 1, 0], Z: V3 = [0, 0, 1];

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

const rot = (f: Frame, c0: V3, c1: V3, c2: V3) => quaternion(unit(dirToLocal(f, c0)), unit(dirToLocal(f, c1)), unit(dirToLocal(f, c2)));
const loc = (f: Frame, p: V3): V3 => toLocal(f, p).map(r5) as V3;
const turnAxes = (deg = 0): [V3, V3] => {
  const t = (deg * Math.PI) / 180;
  return [[Math.cos(t), Math.sin(t), 0], [-Math.sin(t), Math.cos(t), 0]];
};

/** 🎛️ The section frame of a loft section: `ax` the direction of half extent `a`, `n` the normal (path tangent), `b` along n x ax. */
function secAxes(s: Sec, tangent: V3): [V3, V3, V3] {
  const n = unit(s.n ?? tangent);
  const ref = s.ax ?? (Math.abs(n[1]) < 0.8 ? Y : Z);
  const ax = unit(sub(ref, mul(n, dot3(ref, n))));
  return [ax, cross3(n, ax), n];
}
const tangentAt = (secs: Sec[], i: number): V3 => unit(sub(secs[Math.min(secs.length - 1, i + 1)]!.c, secs[Math.max(0, i - 1)]!.c));

/** 🌿️ A round tube (loft) through world stations [x, y, z, r]. */
export const tube = (id: string, bone: string, look: Look, pts: [number, number, number, number][]): Loft => ({ k: "loft", id, bone, look, secs: pts.map(([x, y, z, r]) => ({ c: [x, y, z], a: r, b: r, e: 2 })) });

/** 🧊️ The bone-local solid of a primitive. */
export function solidOf(p: Prim, frames: Record<string, Frame>): Json {
  const f = frames[p.bone];
  if (!f) throw new Error(`no frame for bone ${p.bone} (${p.id})`);
  const head: Json = { id: p.id, kind: "", bone: p.bone, material: p.look.material, role: p.look.role ?? "body", tone: p.look.tone };
  const tail: Json = p.look.flags ?? {};
  if (p.k === "ell") {
    const [a, b] = turnAxes(p.turn);
    return { ...head, kind: "ellipsoid", radii: p.r.map(r5), transform: { translation: loc(f, p.c), rotation: rot(f, a, b, Z) }, ...tail };
  }
  if (p.k === "box") return { ...head, kind: "box", size: p.size.map(r5), ...(p.corner ? { cornerRadius: r5(p.corner) } : {}), transform: { translation: loc(f, p.c), rotation: rot(f, X, Y, Z) }, ...tail };
  if (p.k === "loft") {
    const sections = p.secs.map((s, i) => {
      const [ax, by, n] = secAxes(s, tangentAt(p.secs, i));
      return { at: loc(f, s.c), rx: r5(s.a), ry: r5(s.b), exponent: s.e ?? 2, rotation: rot(f, ax, by, n) };
    });
    return { ...head, kind: "loft", sections, capStart: true, capEnd: true, ...tail };
  }
  if (p.k === "ext") {
    const ez = unit(cross3(p.ex, p.ey));
    return { ...head, kind: "extrude", plane: "xy", profile: p.profile.map(([a, b]) => [r5(a), r5(b)]), ...(p.holes ? { holes: p.holes.map((h) => h.map(([a, b]) => [r5(a), r5(b)])) } : {}), from: r5(p.from), to: r5(p.to), transform: { translation: loc(f, p.o), rotation: rot(f, unit(p.ex), unit(p.ey), ez) }, ...tail };
  }
  return { ...head, kind: "revolve", profile: p.profile.map(([a, r]) => [r5(a), r5(r)]), axis: dirToLocal(f, Y).map(r5), origin: loc(f, p.c), ...tail };
}

const ring = (n: number, fn: (t: number) => V3): V3[] => Array.from({ length: n }, (_, i) => fn((2 * Math.PI * i) / n));
const se = (t: number, e: number): [number, number] => {
  const [c, s] = [Math.cos(t), Math.sin(t)];
  return [Math.sign(c) * Math.abs(c) ** (2 / e), Math.sign(s) * Math.abs(s) ** (2 / e)];
};

/** 🫧️ World point sets whose convex hulls united are the primitive (a loft is the hull of each pair of consecutive sections). */
export function pieces(p: Prim): V3[][] {
  if (p.k === "ell") {
    const [a, b] = turnAxes(p.turn);
    const out: V3[] = [];
    for (let i = 1; i < 12; i++) {
      const ph = (Math.PI * i) / 12;
      for (const q of ring(32, (t) => [Math.sin(ph) * Math.cos(t), Math.sin(ph) * Math.sin(t), Math.cos(ph)])) out.push(add(p.c, add(add(mul(a, q[0] * p.r[0]), mul(b, q[1] * p.r[1])), mul(Z, q[2] * p.r[2]))));
    }
    for (let i = 0; i < 64; i++) {
      const t = (2 * Math.PI * i) / 64;
      out.push(add(p.c, add(mul(a, Math.cos(t) * p.r[0]), mul(b, Math.sin(t) * p.r[1]))));
      out.push(add(p.c, add(mul(a, Math.cos(t) * p.r[0]), mul(Z, Math.sin(t) * p.r[2]))));
      out.push(add(p.c, add(mul(b, Math.cos(t) * p.r[1]), mul(Z, Math.sin(t) * p.r[2]))));
    }
    return [out];
  }
  if (p.k === "box") {
    const out: V3[] = [];
    for (const sx of [-1, 1]) for (const sy of [-1, 1]) for (const sz of [-1, 1]) out.push([p.c[0] + (sx * p.size[0]) / 2, p.c[1] + (sy * p.size[1]) / 2, p.c[2] + (sz * p.size[2]) / 2]);
    return [out];
  }
  if (p.k === "loft") {
    const rings = p.secs.map((s, i) => {
      const [ax, by] = secAxes(s, tangentAt(p.secs, i));
      return ring(48, (t) => {
        const [c, d] = se(t, s.e ?? 2);
        return add(s.c, add(mul(ax, c * s.a), mul(by, d * s.b)));
      });
    });
    return rings.slice(1).map((r, i) => [...rings[i]!, ...r]);
  }
  if (p.k === "ext") {
    const ez = unit(cross3(p.ex, p.ey));
    const at = (a: number, b: number, h: number): V3 => add(p.o, add(add(mul(unit(p.ex), a), mul(unit(p.ey), b)), mul(ez, h)));
    const tri = triangles(p.profile);
    return tri.map((t) => t.flatMap(([a, b]) => [at(a, b, p.from), at(a, b, p.to)]));
  }
  const pr = p.profile;
  return pr.slice(1).map((q, i) => [pr[i]!, q].flatMap(([h, r]) => ring(36, (t) => [p.c[0] + r * Math.cos(t), p.c[1] + h, p.c[2] + r * Math.sin(t)])));
}

/** 🔺️ Ear-clipping triangulation of a simple polygon. */
function triangles(poly: [number, number][]): [number, number][][] {
  const area = poly.reduce((s, p, i) => s + p[0] * poly[(i + 1) % poly.length]![1] - poly[(i + 1) % poly.length]![0] * p[1], 0);
  const idx = poly.map((_, i) => i);
  if (area < 0) idx.reverse();
  const out: [number, number][][] = [];
  const crossz = (a: [number, number], b: [number, number], c: [number, number]) => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
  const inside = (p: [number, number], a: [number, number], b: [number, number], c: [number, number]) => crossz(a, b, p) >= 0 && crossz(b, c, p) >= 0 && crossz(c, a, p) >= 0;
  let guard = 0;
  while (idx.length > 3 && guard++ < 10000) {
    let cut = false;
    for (let i = 0; i < idx.length; i++) {
      const [a, b, c] = [poly[idx[(i + idx.length - 1) % idx.length]!]!, poly[idx[i]!]!, poly[idx[(i + 1) % idx.length]!]!];
      if (crossz(a, b, c) <= 1e-12) continue;
      if (idx.some((j) => ![a, b, c].includes(poly[j]!) && inside(poly[j]!, a, b, c))) continue;
      out.push([a, b, c]);
      idx.splice(i, 1);
      cut = true;
      break;
    }
    if (!cut) break;
  }
  if (idx.length >= 3) out.push(idx.map((i) => poly[i]!));
  return out;
}

export const project = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });

/** 🪢️ The convex hull of points (monotone chain), counter-clockwise. */
export function hull(points: P2[]): P2[] {
  const ps = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  if (ps.length < 3) return ps;
  const cr = (o: P2, a: P2, b: P2) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lo: P2[] = [], hi: P2[] = [];
  for (const p of ps) {
    while (lo.length >= 2 && cr(lo[lo.length - 2]!, lo[lo.length - 1]!, p) <= 0) lo.pop();
    lo.push(p);
  }
  for (const p of [...ps].reverse()) {
    while (hi.length >= 2 && cr(hi[hi.length - 2]!, hi[hi.length - 1]!, p) <= 0) hi.pop();
    hi.push(p);
  }
  return [...lo.slice(0, -1), ...hi.slice(0, -1)];
}

const ringArea = (r: readonly P2[]): number => r.reduce((s, p, i) => s + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;

/** ✏️ The drawn outline of primitives in a view: union of the projected hulls, small parts dropped, rings simplified to `tol`; outer rings first, largest first. */
export function outline(view: View, prims: Prim[], tol: number, minArea: number): P2[][] {
  const polys = prims.flatMap((p) => pieces(p).map((pc) => hull(pc.map((q) => project(view, q))))).filter((h) => h.length >= 3 && Math.abs(ringArea(h)) > 1e-10);
  if (polys.length === 0) return [];
  const rings = unionPolygons(polys.map((h) => (ringArea(h) < 0 ? [...h].reverse() : h))).map((r) => ({ r, a: ringArea(r) }));
  const kept = rings.filter((x) => Math.abs(x.a) > minArea);
  kept.sort((a, b) => (a.a > 0 ? 0 : 1) - (b.a > 0 ? 0 : 1) || Math.abs(b.a) - Math.abs(a.a));
  return kept.map((x) => simplifyRing(x.r, tol));
}

/** 🔁️ Paper points of a view in the drawing frame of a shape's bone: bone-local (u, v) in the elevation, world rest coordinates in front and plan. */
export function toDrawing(view: View, f: Frame, rings: P2[][]): P2[][] {
  if (view !== "elevation") return rings;
  return rings.map((r) => r.map((p) => {
    const l = toLocal(f, [p.x, p.y, f.o[2]]);
    return { x: l[0], y: l[1] };
  }));
}

export const pathOf = (rings: P2[][], closed = true): string => rings.map((r) => r.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x)} ${r4(p.y)}`).join(" ") + (closed ? " Z" : "")).join(" ");
export const pointsOf = (r: P2[]): [number, number][] => r.map((p) => [r4(p.x), r4(p.y)]);
