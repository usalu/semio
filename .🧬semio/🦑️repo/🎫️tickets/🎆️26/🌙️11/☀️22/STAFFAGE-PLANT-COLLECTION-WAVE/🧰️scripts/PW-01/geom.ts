/** 📐️ Plane geometry of the PW-01 maple generator: the exact orthographic shadow of every world-space part (ellipsoid lobe or swept tube) in a view, smooth union outlines traced from a signed-distance raster, and the painter raster that says which drawn group is visible where.
 * The world-space parts and their bone-local conversion come from the round-3 kit, imported from its place.
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts
 * @see ../../📋️brief.md */
import { project, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts";
import { simplifyRing, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";

export type Ellipse = { kind: "ellipse"; cx: number; cy: number; ra: number; rb: number; th: number };
export type Capsules = { kind: "capsules"; pts: { x: number; y: number; r: number }[] };
export type Prim = Ellipse | Capsules;

const AXES: Record<View, [V3, V3, V3]> = { elevation: [[1, 0, 0], [0, 1, 0], [0, 0, -1]], front: [[0, 0, 1], [0, 1, 0], [1, 0, 0]], plan: [[1, 0, 0], [0, 0, 1], [0, 1, 0]] };
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

/** 👁️ How near a world point is to the viewer of a view (larger is nearer). */
export const near = (view: View, p: V3): number => dot(p, AXES[view][2]);

/** 🥚️ The exact shadow ellipse of an ellipsoid lobe in a view. */
export function ellipseOf(view: View, l: Lobe): Ellipse {
  const [e1, e2] = AXES[view];
  const cols = l.axes.map((a, i) => [dot(a, e1) * l.radii[i]!, dot(a, e2) * l.radii[i]!]);
  const q11 = cols.reduce((s, c) => s + c[0]! * c[0]!, 0), q12 = cols.reduce((s, c) => s + c[0]! * c[1]!, 0), q22 = cols.reduce((s, c) => s + c[1]! * c[1]!, 0);
  const th = 0.5 * Math.atan2(2 * q12, q11 - q22);
  const mean = (q11 + q22) / 2, diff = Math.hypot((q11 - q22) / 2, q12);
  const c = project(view, l.c);
  return { kind: "ellipse", cx: c.x, cy: c.y, ra: Math.sqrt(mean + diff), rb: Math.sqrt(Math.max(1e-12, mean - diff)), th };
}

/** 🪵️ The shadow of a tube in a view as a chain of tapered capsules through its projected sections. */
export const capsulesOf = (view: View, t: Tube): Capsules => ({ kind: "capsules", pts: t.points.map((s) => ({ ...project(view, s.p), r: s.r })) });

export const primOf = (view: View, part: Lobe | Tube): Prim => (part.kind === "lobe" ? ellipseOf(view, part) : capsulesOf(view, part));

type Box = { minX: number; minY: number; maxX: number; maxY: number };
function boxOf(p: Prim): Box {
  if (p.kind === "ellipse") {
    const [c, s] = [Math.cos(p.th), Math.sin(p.th)];
    const hx = Math.hypot(p.ra * c, p.rb * s), hy = Math.hypot(p.ra * s, p.rb * c);
    return { minX: p.cx - hx, maxX: p.cx + hx, minY: p.cy - hy, maxY: p.cy + hy };
  }
  return p.pts.reduce((b, q) => ({ minX: Math.min(b.minX, q.x - q.r), maxX: Math.max(b.maxX, q.x + q.r), minY: Math.min(b.minY, q.y - q.r), maxY: Math.max(b.maxY, q.y + q.r) }), { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity });
}

/** 📏️ Approximate signed distance of a paper point to a primitive (negative inside), exact on the boundary. */
function sdf(p: Prim, x: number, y: number): number {
  if (p.kind === "ellipse") {
    const [c, s] = [Math.cos(p.th), Math.sin(p.th)];
    const dx = x - p.cx, dy = y - p.cy;
    const u = (dx * c + dy * s) / p.ra, v = (-dx * s + dy * c) / p.rb;
    const f = u * u + v * v - 1;
    const g = 2 * Math.hypot(u / p.ra, v / p.rb);
    return g < 1e-9 ? -Math.min(p.ra, p.rb) : Math.max(f / g, -Math.min(p.ra, p.rb));
  }
  let best = Infinity;
  for (let i = 0; i + 1 < p.pts.length; i++) {
    const a = p.pts[i]!, b = p.pts[i + 1]!;
    const ex = b.x - a.x, ey = b.y - a.y, l2 = ex * ex + ey * ey;
    const raw = l2 < 1e-12 ? 0 : ((x - a.x) * ex + (y - a.y) * ey) / l2;
    const t = Math.max(0, Math.min(1, raw));
    let d = Math.hypot(x - a.x - ex * t, y - a.y - ey * t) - (a.r + (b.r - a.r) * t);
    const len = Math.sqrt(l2);
    if (len > 1.5 * Math.max(a.r, b.r) && ((i === 0 && raw < 0) || (i + 2 === p.pts.length && raw > 1))) {
      const s = raw < 0 ? raw * len : (raw - 1) * len;
      const r = raw < 0 ? a.r : b.r;
      const perp = Math.abs(((x - a.x) * ey - (y - a.y) * ex) / len);
      d = Math.max(perp - r, Math.abs(s));
    }
    if (d < best) best = d;
  }
  return best;
}

const ringArea = (r: readonly P2[]): number => r.reduce((s, p, i) => s + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;

/** 🫧️ A signed-distance raster of the union of primitives: cell centres at `x0 + (i + 0.5) cell`. */
export type Field = { x0: number; y0: number; cell: number; nx: number; ny: number; f: Float32Array };

/** 🧮️ The union field of primitives on a raster of pitch `cell`, cut at the ground (`floorY`) when given. */
export function fieldOf(prims: readonly Prim[], cell: number, floorY?: number, box?: Box): Field {
  const bb = box ?? prims.map(boxOf).reduce((a, b) => ({ minX: Math.min(a.minX, b.minX), minY: Math.min(a.minY, b.minY), maxX: Math.max(a.maxX, b.maxX), maxY: Math.max(a.maxY, b.maxY) }));
  const x0 = bb.minX - 3 * cell, y0 = bb.minY - 3 * cell;
  const nx = Math.ceil((bb.maxX - bb.minX) / cell) + 6, ny = Math.ceil((bb.maxY - bb.minY) / cell) + 6;
  const f = new Float32Array(nx * ny).fill(4 * cell);
  for (const p of prims) {
    const b = boxOf(p);
    const i0 = Math.max(0, Math.floor((b.minX - x0) / cell) - 2), i1 = Math.min(nx - 1, Math.ceil((b.maxX - x0) / cell) + 2);
    const j0 = Math.max(0, Math.floor((b.minY - y0) / cell) - 2), j1 = Math.min(ny - 1, Math.ceil((b.maxY - y0) / cell) + 2);
    for (let j = j0; j <= j1; j++) {
      const y = y0 + (j + 0.5) * cell;
      for (let i = i0; i <= i1; i++) {
        const d = sdf(p, x0 + (i + 0.5) * cell, y);
        const k = j * nx + i;
        if (d < f[k]!) f[k] = d;
      }
    }
  }
  if (floorY !== undefined) for (let j = 0; j < ny; j++) {
    const cut = floorY - (y0 + (j + 0.5) * cell);
    if (cut > -4 * cell) for (let i = 0; i < nx; i++) f[j * nx + i] = Math.max(f[j * nx + i]!, cut);
  }
  return { x0, y0, cell, nx, ny, f };
}

/** ✏️ The contour rings of a field at level 0 by marching squares with linear interpolation: outer rings counter-clockwise, holes clockwise, each simplified to `tolerance`; rings below `minArea` (outer) or `minHole` (holes) are dropped. */
export function contours(fd: Field, tolerance: number, minArea: number, minHole: number): P2[][] {
  const { x0, y0, cell, nx, ny, f } = fd;
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= nx || j >= ny ? 1 : f[j * nx + i]!);
  const px = (i: number): number => x0 + (i + 0.5) * cell, py = (j: number): number => y0 + (j + 0.5) * cell;
  const next = new Map<string, string>();
  const point = new Map<string, P2>();
  const edge = (i: number, j: number, horizontal: boolean): string => {
    const key = `${horizontal ? "h" : "v"}${i},${j}`;
    if (!point.has(key)) {
      if (horizontal) {
        const a = at(i, j), b = at(i + 1, j), t = a / (a - b);
        point.set(key, { x: px(i) + t * cell, y: py(j) });
      } else {
        const a = at(i, j), b = at(i, j + 1), t = a / (a - b);
        point.set(key, { x: px(i), y: py(j) + t * cell });
      }
    }
    return key;
  };
  for (let j = -1; j < ny; j++) for (let i = -1; i < nx; i++) {
    const a = at(i, j) < 0 ? 1 : 0, b = at(i + 1, j) < 0 ? 1 : 0, c = at(i + 1, j + 1) < 0 ? 1 : 0, d = at(i, j + 1) < 0 ? 1 : 0;
    const code = a | (b << 1) | (c << 2) | (d << 3);
    if (code === 0 || code === 15) continue;
    const S = (): string => edge(i, j, true), E = (): string => edge(i + 1, j, false), N = (): string => edge(i, j + 1, true), W = (): string => edge(i, j, false);
    const link = (p: string, q: string): void => void next.set(p, q);
    const centre = (at(i, j) + at(i + 1, j) + at(i + 1, j + 1) + at(i, j + 1)) / 4 < 0;
    switch (code) {
      case 1: link(W(), S()); break;
      case 2: link(S(), E()); break;
      case 3: link(W(), E()); break;
      case 4: link(E(), N()); break;
      case 5: if (centre) { link(W(), N()); link(E(), S()); } else { link(W(), S()); link(E(), N()); } break;
      case 6: link(S(), N()); break;
      case 7: link(W(), N()); break;
      case 8: link(N(), W()); break;
      case 9: link(N(), S()); break;
      case 10: if (centre) { link(S(), W()); link(N(), E()); } else { link(S(), E()); link(N(), W()); } break;
      case 11: link(N(), E()); break;
      case 12: link(E(), W()); break;
      case 13: link(E(), S()); break;
      case 14: link(S(), W()); break;
    }
  }
  const rings: P2[][] = [];
  const used = new Set<string>();
  for (const start of next.keys()) {
    if (used.has(start)) continue;
    const ring: P2[] = [];
    let k: string | undefined = start;
    while (k !== undefined && !used.has(k)) {
      used.add(k);
      ring.push(point.get(k)!);
      k = next.get(k);
    }
    if (ring.length >= 3) rings.push(ring.reverse());
  }
  const out: { ring: P2[]; area: number }[] = [];
  for (const r of rings) {
    const area = ringArea(r);
    if (area > 0 ? area < minArea : -area < minHole) continue;
    const s = simplifyRing(r, tolerance);
    if (s.length >= 3) out.push({ ring: s, area });
  }
  out.sort((p, q) => (p.area > 0 ? 0 : 1) - (q.area > 0 ? 0 : 1) || Math.abs(q.area) - Math.abs(p.area));
  return out.map((o) => o.ring);
}

/** 🖼️ The smooth union outline of parts in a view. */
export function outlineOf(view: View, parts: readonly (Lobe | Tube)[], cell: number, tolerance: number, minArea: number, minHole: number): P2[][] {
  if (parts.length === 0) return [];
  return contours(fieldOf(parts.map((p) => primOf(view, p)), cell, view === "plan" ? undefined : 0), tolerance, minArea, minHole);
}

/** 🎭️ Which of several groups is painted on top at every cell of a view (index into `groups`, -1 for none), groups listed from far to near. */
export type Painter = { x0: number; y0: number; cell: number; nx: number; ny: number; top: Int16Array };

export function painterOf(view: View, groups: readonly (readonly (Lobe | Tube)[])[], cell: number): Painter {
  const all = groups.flat().map((p) => primOf(view, p));
  const whole = fieldOf(all, cell, view === "plan" ? undefined : 0);
  const top = new Int16Array(whole.nx * whole.ny).fill(-1);
  groups.forEach((g, gi) => {
    if (g.length === 0) return;
    const fd = fieldOf(g.map((p) => primOf(view, p)), cell, view === "plan" ? undefined : 0, { minX: whole.x0 + 3 * cell, minY: whole.y0 + 3 * cell, maxX: whole.x0 + (whole.nx - 3) * cell, maxY: whole.y0 + (whole.ny - 3) * cell });
    for (let j = 0; j < whole.ny; j++) for (let i = 0; i < whole.nx; i++) {
      const ii = i + Math.round((whole.x0 - fd.x0) / cell), jj = j + Math.round((whole.y0 - fd.y0) / cell);
      if (ii < 0 || jj < 0 || ii >= fd.nx || jj >= fd.ny) continue;
      if (fd.f[jj * fd.nx + ii]! < 0) top[j * whole.nx + i] = gi;
    }
  });
  return { x0: whole.x0, y0: whole.y0, cell, nx: whole.nx, ny: whole.ny, top };
}

/** 🔎️ The group painted on top at a paper point (-1 for none). */
export function topAt(pt: Painter, x: number, y: number): number {
  const i = Math.floor((x - pt.x0) / pt.cell), j = Math.floor((y - pt.y0) / pt.cell);
  return i < 0 || j < 0 || i >= pt.nx || j >= pt.ny ? -1 : pt.top[j * pt.nx + i]!;
}

/** 🧿️ True when every point of a disc of radius `r` about (x, y) is painted by group `g`. */
export function clearIn(pt: Painter, g: number, x: number, y: number, r: number): boolean {
  if (topAt(pt, x, y) !== g) return false;
  for (let k = 0; k < 12; k++) {
    const a = (k * Math.PI) / 6;
    if (topAt(pt, x + r * Math.cos(a), y + r * Math.sin(a)) !== g) return false;
    if (topAt(pt, x + 0.5 * r * Math.cos(a + 0.26), y + 0.5 * r * Math.sin(a + 0.26)) !== g) return false;
  }
  return true;
}

/** 🎲️ A small deterministic generator (mulberry32) so every layout jitter is reproducible from the species seed. */
export function rng(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** 📐️ The area of a ring (positive counter-clockwise). */
export const areaOf = ringArea;
