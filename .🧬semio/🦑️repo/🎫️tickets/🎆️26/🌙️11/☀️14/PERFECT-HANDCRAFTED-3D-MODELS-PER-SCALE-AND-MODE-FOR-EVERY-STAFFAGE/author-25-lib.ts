/** Author 25 builder: lofts along a bone from the scale's own drawings with hand-chosen anatomy parameters (section exponent, lateral scale and offset, end rounding), in the bone-local frame (u along the bone, v sagittal, w lateral). */
import { drawn, drawnSpan, loftOf, selectStations, superellipseSection, type Ctx, type Json, type Station } from "./kit-lib.ts";

export type Lat = { view: "plan" | "front" | "auto"; bone?: string; ids?: string[]; scale?: number; min?: number; offset?: number; taper?: [number, number]; inward?: [number, number] } | { half: number; centre?: number; taper?: number };
export type Part = {
  id: string;
  bone: string;
  material: string;
  tone?: number;
  role?: string;
  elev?: string[];
  elevBone?: string;
  lat: Lat;
  e?: number;
  n?: number;
  tol?: number;
  bevel?: number;
  vScale?: number;
  uRange?: [number, number];
  roll?: number;
  thick?: number;
  tags?: string[];
  extra?: Json;
};

type Pt = [number, number];
const ALONG_U: [number, number, number, number] = [0.5, 0.5, 0.5, 0.5];
const toLocal = (c: Ctx, bone: string, x: number, y: number): Pt => {
  const f = c.state.frames[bone]!;
  const d = [x - f.o[0]!, y - f.o[1]!];
  return [d[0]! * f.u[0]! + d[1]! * f.u[1]!, d[0]! * f.v[0]! + d[1]! * f.v[1]!];
};

function scan(rings: Pt[][], u: number): [number, number] | null {
  let lo = Infinity;
  let hi = -Infinity;
  for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!;
    const b = ring[i]!;
    if (a[0] <= u !== b[0] <= u) {
      const v = a[1] + ((u - a[0]) / (b[0] - a[0])) * (b[1] - a[1]);
      lo = Math.min(lo, v);
      hi = Math.max(hi, v);
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

function alongRange(c: Ctx, view: "plan" | "front", bone: string, pick: (s: { id: string; role: string }) => boolean): [number, number] {
  let lo = Infinity;
  let hi = -Infinity;
  for (const s of drawn(c, view, bone)) if (pick(s as never)) for (const r of s.rings) for (const q of r) {
    const a = view === "plan" ? q.x : q.y;
    lo = Math.min(lo, a);
    hi = Math.max(hi, a);
  }
  const e = 1e-4;
  return [lo + e, hi - e];
}

/** Thinnest extent (smaller side of the bounding box) of a filled closed drawn body shape of the bone in any view (C13 limit of its solids). */
export function thinnest(c: Ctx, bone: string): number {
  let t = Infinity;
  const fr = c.state.frames[bone]!;
  for (const v of [Math.abs(fr.u[1]!) > Math.abs(fr.u[0]!) ? "front" : "plan"] as const) for (const s of drawn(c, v, bone)) {
    if (s.role !== "body") continue;
    let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
    for (const r of s.rings) for (const q of r) { x0 = Math.min(x0, q.x); x1 = Math.max(x1, q.x); y0 = Math.min(y0, q.y); y1 = Math.max(y1, q.y); }
    t = Math.min(t, x1 - x0, y1 - y0);
  }
  return Number.isFinite(t) ? t : Infinity;
}

/** Stations of one part: elevation gives the sagittal extent along u, plan or front the lateral extent at the section centre. */
export function stations(c: Ctx, p: Part): Station[] {
  const f = c.state.frames[p.bone]!;
  const eb = p.elevBone ?? p.bone;
  const shapes = drawn(c, "elevation", eb).filter((s) => (p.elev === undefined ? s.role === "body" : p.elev.includes(s.id)));
  const rings: Pt[][] = shapes.flatMap((s) => s.rings.map((r) => r.map((q) => toLocal(c, p.bone, q.x, q.y))));
  for (const e of c.state.elevation as Json[]) {
    if (e.closed || e.shape.role !== "body" || p.elev === undefined || !p.elev.includes(e.shape.id)) continue;
    const pen = ((e.shape.penMm ?? c.m.representations[String(c.scale)].pens?.body ?? 0.2) * c.scale) / 1000;
    for (const r of e.rings as { x: number; y: number }[][]) {
      const pts = r.map((q) => toLocal(c, p.bone, q.x, q.y));
      const l = pts.map((q, i) => {
        const a = pts[Math.max(0, i - 1)]!;
        const b = pts[Math.min(pts.length - 1, i + 1)]!;
        const d = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1;
        return [(-(b[1] - a[1]) / d) * (pen / 2), ((b[0] - a[0]) / d) * (pen / 2)] as Pt;
      });
      rings.push([...pts.map((q, i) => [q[0] + l[i]![0], q[1] + l[i]![1]] as Pt), ...pts.map((q, i) => [q[0] - l[i]![0], q[1] - l[i]![1]] as Pt).reverse()]);
    }
  }
  if (rings.length === 0) throw new Error(`${p.id}: no elevation shape on ${eb}`);
  let u0 = Infinity;
  let u1 = -Infinity;
  for (const r of rings) for (const q of r) {
    u0 = Math.min(u0, q[0]);
    u1 = Math.max(u1, q[0]);
  }
  if (p.uRange !== undefined) [u0, u1] = p.uRange;
  const n = p.n ?? 40;
  const eps = Math.max((u1 - u0) / (4 * n), 1e-4);
  const raw: Station[] = [];
  const lat = p.lat;
  const vertical = Math.abs(f.u[1]!) > Math.abs(f.u[0]!);
  const floor = 0.51 * Math.min(c.feature, thinnest(c, p.bone));
  let last: Station | undefined;
  for (let i = 0; i < n; i++) {
    const u = u0 + eps + ((u1 - u0 - 2 * eps) * i) / (n - 1);
    const sp = scan(rings, u);
    if (sp === null) continue;
    const vs = p.vScale ?? 1;
    const c1 = (sp[0] + sp[1]) / 2;
    const ry = Math.max(((sp[1] - sp[0]) / 2) * vs, floor);
    const wx = f.o[0]! + u * f.u[0]! + c1 * f.v[0]!;
    const wy = f.o[1]! + u * f.u[1]! + c1 * f.v[1]!;
    let rx: number;
    let c2: number;
    if ("half" in lat) {
      const k = lat.taper === undefined ? 1 : 1 + (lat.taper - 1) * ((u - u0) / (u1 - u0));
      rx = lat.half * k;
      c2 = lat.centre ?? 0;
    } else {
      const view = lat.view === "auto" ? (vertical ? "front" : "plan") : lat.view;
      const pick = lat.ids === undefined ? (s: { role: string }) => s.role === "body" : (s: { id: string }) => lat.ids!.includes(s.id);
      const [alo, ahi] = alongRange(c, view, lat.bone ?? p.bone, pick);
      const sp2 = view === "plan" ? drawnSpan(c, "plan", lat.bone ?? p.bone, "x", Math.min(Math.max(wx, alo), ahi), pick) : drawnSpan(c, "front", lat.bone ?? p.bone, "y", Math.min(Math.max(wy, alo), ahi), pick);
      if (sp2 === null) {
        if (last === undefined) continue;
        raw.push({ ...last, s: u, c1, ry, e: p.e ?? 2.4 });
        continue;
      }
      const t = (u - u0) / (u1 - u0 || 1);
      const lerp = (k: [number, number] | undefined): number => (k === undefined ? 1 : k[0] + (k[1] - k[0]) * t);
      rx = Math.max(((sp2[1] - sp2[0]) / 2) * (lat.scale ?? 1) * lerp(lat.taper), lat.min ?? floor);
      c2 = (((sp2[0] + sp2[1]) / 2 - f.o[2]!) * f.w[2]!) * lerp(lat.inward) + (lat.offset ?? 0);
    }
    last = { s: u, c1, c2, rx: Math.max(rx, floor), ry, e: p.e ?? 2.4 };
    raw.push(last);
  }
  if (raw.length < 2) throw new Error(`${p.id}: fewer than two stations`);
  const sel = selectStations(raw, p.tol ?? 0.45 * c.feature, floor).map((s) => ({ ...s, e: p.e ?? 2.4 }));
  const b = p.bevel ?? 0.8;
  for (const k of [0, sel.length - 1]) sel[k] = { ...sel[k]!, rx: Math.max(sel[k]!.rx * b, floor), ry: Math.max(sel[k]!.ry * b, floor) };
  return sel;
}

const qmul = (a: number[], b: number[]): [number, number, number, number] => [
  a[3]! * b[0]! + a[0]! * b[3]! + a[1]! * b[2]! - a[2]! * b[1]!,
  a[3]! * b[1]! - a[0]! * b[2]! + a[1]! * b[3]! + a[2]! * b[0]!,
  a[3]! * b[2]! + a[0]! * b[1]! - a[1]! * b[0]! + a[2]! * b[3]! ,
  a[3]! * b[3]! - a[0]! * b[0]! - a[1]! * b[1]! - a[2]! * b[2]!,
];

export function part(c: Ctx, p: Part): Json {
  const st = stations(c, p);
  const phi = ((p.roll ?? 0) * Math.PI) / 180;
  const rot = phi === 0 ? ALONG_U : qmul(ALONG_U, [0, 0, Math.sin(phi / 2), Math.cos(phi / 2)]);
  const sec = (s: Station) => {
    if (phi === 0) return superellipseSection([s.s, s.c1 ?? 0, s.c2 ?? 0], s.ry, s.rx, s.e ?? 2.4, rot);
    const th = p.thick ?? 0.011;
    return superellipseSection([s.s, s.c1 ?? 0, s.c2 ?? 0], Math.max((s.ry - th * Math.sin(phi)) / Math.cos(phi), th), th, s.e ?? 2.4, rot);
  };
  const solid = loftOf({ id: p.id, bone: p.bone, material: p.material, tone: p.tone ?? 0.6, role: p.role ?? "body" }, st.map(sec));
  return { ...solid, ...(p.tags === undefined ? {} : { tags: p.tags }), ...(p.extra ?? {}) };
}

/** Bone-local bounding box [u0, u1, v0, v1] of one drawn elevation shape. */
export function bboxLocal(c: Ctx, bone: string, shapeId: string): [number, number, number, number] | null {
  for (const s of drawn(c, "elevation")) {
    if (s.id !== shapeId) continue;
    let b: [number, number, number, number] = [Infinity, -Infinity, Infinity, -Infinity];
    for (const r of s.rings) for (const q of r) {
      const [u, v] = toLocal(c, bone, q.x, q.y);
      b = [Math.min(b[0], u), Math.max(b[1], u), Math.min(b[2], v), Math.max(b[3], v)];
    }
    return b;
  }
  return null;
}

/** Lower surface (v of the loft bottom) of a solid at bone-local u by linear interpolation of its sections. */
export function bottomAt(solid: Json, u: number): number {
  const s = solid.sections as { at: number[]; rx: number }[];
  for (let i = 0; i + 1 < s.length; i++) {
    const a = s[i]!;
    const b = s[i + 1]!;
    if (u >= a.at[0]! && u <= b.at[0]!) {
      const t = (u - a.at[0]!) / (b.at[0]! - a.at[0]! || 1);
      return a.at[1]! + (b.at[1]! - a.at[1]!) * t - (a.rx + (b.rx - a.rx) * t);
    }
  }
  const e = u < s[0]!.at[0]! ? s[0]! : s[s.length - 1]!;
  return e.at[1]! - e.rx;
}

/** 📐️ Surface area of an ellipsoid (Knud Thomsen approximation, error below 1.1 %). */
export const ellArea = (a: number, b: number, c: number): number => 4 * Math.PI * Math.pow(((a * b) ** 1.6075 + (a * c) ** 1.6075 + (b * c) ** 1.6075) / 3, 1 / 1.6075);

/** 📐️ Third radius of an ellipsoid with the given other two whose whole surface is `area` (bisection; the larger of the result and `min`). */
export function thirdRadius(a: number, b: number, area: number, min = 0): number {
  let lo = 1e-4;
  let hi = 1;
  for (let i = 0; i < 60; i++) {
    const mid = (lo + hi) / 2;
    if (ellArea(a, b, mid) < area) lo = mid;
    else hi = mid;
  }
  return Math.max((lo + hi) / 2, min);
}
