/** 🌿️ Author 67: limbs as true round tubes. The elevation outline gives the centre line and the perpendicular half width (the projection of a tube is its centre line offset by the radius), the front strip gives the lateral position, the section lies perpendicular to the 3D path.
 * @see ./author-31-lib.ts */
import type { V3 } from "./volumes-lib.ts";
import { Kit, ivalsAll, bboxOfShapes, type Meta, type Sec, type Shapes } from "./author-02-lib.ts";

const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => { const n = Math.hypot(...a) || 1; return [a[0] / n, a[1] / n, a[2] / n]; };

/** 🪵️ Round tube along an elevation outline whose lateral position follows `F` (front strips; none: z = `zOf(x)` or 0). Returns the section path. */
export function limbTube(k: Kit, id: string, bone: string, E: Shapes, F: Shapes | null, m: Meta, o: { adaptive?: number; zOf?: (x: number) => number; list?: "body" | "thermal" | "structure"; extra?: Record<string, unknown>; endK?: number } = {}): Sec[] {
  const b = bboxOfShapes(E);
  const lo = b.minX, hi = b.maxX;
  const adaptive = o.adaptive ?? k.feat * 0.5;
  const N = Math.max(80, Math.ceil((hi - lo) / Math.max(0.004, k.feat * 0.4)));
  const ts = Array.from({ length: N + 1 }, (_, i) => lo + ((hi - lo) * i) / N);
  const clampT = (t: number) => Math.min(hi - 1e-4, Math.max(lo + 1e-4, t));
  const edge = ts.map((t) => { const iv = ivalsAll(E, "x", clampT(t)); return iv.length ? [iv[0]![0], iv[iv.length - 1]![1]] : [0, 0]; });
  const keep = new Set<number>([0, N]);
  for (const c of [0, 1]) {
    const rec = (a: number, z: number) => {
      let w = -1, wi = -1;
      for (let i = a + 1; i < z; i++) { const f = (ts[i]! - ts[a]!) / (ts[z]! - ts[a]!); const d = Math.abs(edge[a]![c]! + f * (edge[z]![c]! - edge[a]![c]!) - edge[i]![c]!); if (d > w) { w = d; wi = i; } }
      if (w > adaptive && z - a > 1) { keep.add(wi); rec(a, wi); rec(wi, z); }
    };
    rec(0, N);
  }
  const xs = [...keep].sort((p, q) => p - q).map((i) => ts[i]!);
  let pts = xs.map((x) => { const iv = ivalsAll(E, "x", clampT(x)); const w = iv.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a)); return { x, mid: (w[0] + w[1]) / 2, half: (w[1] - w[0]) / 2 }; });
  const endK = o.endK ?? 0.6;
  while (pts.length > 3 && pts[0]!.half < endK * pts[1]!.half) pts = pts.slice(1);
  while (pts.length > 3 && pts[pts.length - 1]!.half < endK * pts[pts.length - 2]!.half) pts = pts.slice(0, -1);
  const cen: V3[] = pts.map((p) => {
    let z = o.zOf ? o.zOf(p.x) : 0;
    if (F && F.length) { const zi = ivalsAll(F, "y", Math.min(bboxOfShapes(F).maxY - 1e-4, Math.max(bboxOfShapes(F).minY + 1e-4, p.mid))); if (zi.length) { const w = zi.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a)); z = (w[0] + w[1]) / 2; } }
    return [p.x, p.mid, z];
  });
  const secs: Sec[] = cen.map((c, i) => {
    const a = cen[Math.max(0, i - 1)]!, z = cen[Math.min(cen.length - 1, i + 1)]!;
    const t = unit(sub(z, a));
    const e = unit(sub([z[0], z[1], 0], [a[0], a[1], 0]));
    const r = pts[i]!.half * Math.abs(e[0]);
    const up: V3 = Math.abs(t[1]) > 0.95 ? [1, 0, 0] : [0, 1, 0];
    const ax = unit(cross(up, t));
    const ay = cross(t, ax);
    return { c, ax, ay, rx: r, ry: r, exp: 2 };
  });
  k.loft(o.list ?? "body", id, bone, secs, m, o.extra ?? {});
  return secs;
}

/** 🪵️ A straight tapered round tube along a bone from its origin to its tip (radii at both ends), `n` sections. Returns the section path. */
export function boneTube(k: Kit, id: string, bone: string, r0: number, r1: number, m: Meta, o: { n?: number; list?: "body" | "thermal" | "structure"; extra?: Record<string, unknown>; from?: V3; to?: V3 } = {}): Sec[] {
  const a = o.from ?? k.origin(bone), b = o.to ?? k.tip(bone);
  const n = o.n ?? 4;
  const t = unit(sub(b, a));
  const up: V3 = Math.abs(t[1]) > 0.95 ? [1, 0, 0] : [0, 1, 0];
  const ax = unit(cross(up, t));
  const ay = cross(t, ax);
  const secs: Sec[] = Array.from({ length: n }, (_, i) => {
    const f = i / (n - 1), r = r0 + (r1 - r0) * f;
    return { c: [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f] as V3, ax, ay, rx: r, ry: r, exp: 2 };
  });
  k.loft(o.list ?? "body", id, bone, secs, m, o.extra ?? {});
  return secs;
}
