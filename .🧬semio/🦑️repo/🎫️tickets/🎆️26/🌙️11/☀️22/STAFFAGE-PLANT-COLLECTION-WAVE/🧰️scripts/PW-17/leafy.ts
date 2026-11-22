/** 🌿️ PW-17 parts of leafy potted plants (monstera, fiddle-leaf fig): stalks (stems, poles, petioles, aerial roots) that are drawn filled where they print at the end of the band and as true-width strokes where they do not, leaf plates whose holes and fill are decided per view by their printed size, and the merged crown of the coarse scales (the projection of the scale's own model, closed and opened on a raster).
 * @see ./lib.ts */
import { BAND, FEATURE, VIEWS, hull2, meshesOnFrames, outlineOf, plainShape, plateShape, plateSolid, project, r5, revolveSolid, revolvedShape, toShapeCoords, toWorld, type Frame, type Json, type Meta, type P, type Placed, type Scale, type SolidMeta, type V3, type View } from "./lib.ts";

/** 📏️ The smallest width of a 2D point set over all directions (rotating calipers over its hull). */
export function minWidth(points: readonly P[]): number {
  const h = hull2(points);
  if (h.length < 3) return 0;
  let best = Infinity;
  for (let i = 0; i < h.length; i++) {
    const [a, b] = [h[i]!, h[(i + 1) % h.length]!];
    const [dx, dy] = [b[0] - a[0], b[1] - a[1]];
    const l = Math.hypot(dx, dy) || 1;
    let far = 0;
    for (const p of h) far = Math.max(far, Math.abs((p[0] - a[0]) * dy - (p[1] - a[1]) * dx) / l);
    best = Math.min(best, far);
  }
  return best;
}

/** 📐️ The area of a closed 2D ring (shoelace, unsigned). */
export const ringArea = (ring: readonly P[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;

/** 🖊️ Paper millimetres of a real length at the far end of the band of a scale. */
export const atBandEnd = (scale: Scale, m: number): number => (m * 1000) / BAND[scale][1];

/** 🪵️ A stalk along its bone axis from u0 to u1 with radii r0 and r1: the solid (true section; declared `slender` when at least 20 times as long as thick and thinner than the smallest feature, which is the only case that needs the declaration; a short thin stalk is thickened to the smallest feature) and its drawing per view (filled when its mean width prints 0.22 mm at the end of the band, so a tapering stem keeps its true outline, else a stroke along the axis whose pen is the true width on paper, at least 0.1 mm). */
export function stalk(id: string, bone: string, f: Frame, u0: number, u1: number, r0: number, r1: number, scale: Scale, m: Meta, sm: SolidMeta, draw = true): { placed: Placed[]; solid: Json; stroke: boolean } {
  const fe = FEATURE[scale];
  const length = u1 - u0;
  const slender = length / (2 * Math.max(r0, r1)) >= 20 && 2 * Math.min(r0, r1) < fe;
  const [a0, a1] = slender ? [r0, r1] : [Math.max(r0, fe / 2 + 0.0005), Math.max(r1, fe / 2 + 0.0005)];
  const profile: P[] = [[u0, a0], [u1, a1]];
  const solid = revolveSolid(id, bone, profile, { ...sm, ...(slender ? { slender: true } : {}) });
  if (!draw) return { placed: [], solid, stroke: false };
  const centre = toWorld(f, [(u0 + u1) / 2, 0, 0]);
  const thin = atBandEnd(scale, a0 + a1) < 0.22;
  if (!thin) return { placed: [{ shape: revolvedShape(id, bone, profile, m), at: centre }], solid, stroke: false };
  const pen = Math.min(0.35, Math.max(0.1, Math.round(((a0 + a1) * 1000 / scale) * 100) / 100));
  const meta = { ...m, filled: false, penMm: pen };
  const ends: V3[] = [toWorld(f, [u0, 0, 0]), toWorld(f, [u1, 0, 0])];
  const seen = (view: View) => { const [a, b] = ends.map((p) => project(view, p)); return Math.hypot(a![0] - b![0], a![1] - b![1]) > Math.max(2 * Math.max(a0, a1), 0.004); };
  const placed: Placed[] = VIEWS.filter(seen).map((view) => ({ view, at: centre, shape: plainShape(id, bone, [view === "elevation" ? [[u0, 0], [u1, 0]] : ends.map((p) => project(view, p))], meta) }));
  for (const p of placed) { p.shape.closed = false; if (p.shape.kind === "path") throw new Error("stroke must be a poly"); }
  return { placed, solid, stroke: true };
}

/** 🕳️ What the silhouette comparison of the product sees of a hole in one view, in units of its limit: the seen area less a rim of four tenths of a raster cell (a quarter feature; the body is rasterised conservatively), over the sixteen smallest features squared below which an enclosed hole is no outline. Above 1 the hole is outline in body and drawing, below 1 in neither; near 1 the two rasters decide differently. */
export const holeShare = (seen: readonly P[], scale: Scale): number => {
  const perimeter = seen.reduce((sum, p, i) => sum + Math.hypot(seen[(i + 1) % seen.length]![0] - p[0], seen[(i + 1) % seen.length]![1] - p[1]), 0);
  return Math.max(0, ringArea(seen) - 0.4 * (FEATURE[scale] / 4) * perimeter) / (16 * FEATURE[scale] ** 2);
};

/** 🍃️ A leaf plate on its bone (outline and holes in [u, w], thickness v0..v1). A hole whose seen size is undecided in some view ({@link holeShare} between 0.75 and 1.3) is left out of the plant, solid and drawings alike; every other hole is in the solid and is drawn in the views where it is outline (share above 1) or prints (narrowest projected width minus the pen at least 0.17 mm at the end of the band). The plate is filled only where it is at least 0.3 mm wide at the end of the band in that view. */
export function plate(id: string, bone: string, f: Frame, outer: P[], holes: P[][], v0: number, v1: number, scale: Scale, m: Meta, sm: SolidMeta, pen: number): { placed: Placed[]; solid: Json; dropped: Record<View, number>; unfilled: View[]; undecided: number } {
  const centre = toWorld(f, [outer.reduce((s, p) => s + p[0], 0) / outer.length, 0, 0]);
  const world = (r: P[]) => r.map(([u, w]) => toWorld(f, [u, (v0 + v1) / 2, w]));
  const seenIn = (h: P[], view: View) => world(h).map((p) => project(view, p));
  const decided = holes.filter((h) => VIEWS.every((view) => { const share = holeShare(seenIn(h, view), scale); return share < 0.75 || share > 1.3; }));
  const solid = plateSolid(id, bone, outer, decided, v0, v1, sm);
  const placed: Placed[] = [];
  const dropped = {} as Record<View, number>;
  const unfilled: View[] = [];
  for (const view of VIEWS) {
    const wide = atBandEnd(scale, minWidth(world(outer).map((p) => project(view, p))));
    const kept = decided.filter((h) => holeShare(seenIn(h, view), scale) > 1 || atBandEnd(scale, minWidth(seenIn(h, view))) - pen >= 0.17);
    dropped[view] = decided.length - kept.length;
    if (wide < 0.3) unfilled.push(view);
    placed.push({ view, at: centre, shape: plateShape(id, bone, outer, wide < 0.3 ? [] : kept, v0, v1, { ...m, filled: wide >= 0.3 }) });
  }
  return { placed, solid, dropped, unfilled, undecided: holes.length - decided.length };
}

/** ☁️ The merged crown of a coarse scale: the projection of the given body solids (rest frames = default pose), closed so that gaps under the printable gap at the end of the band vanish, opened so that parts under the printable feature there go, holes and specks under (twice the gap)² dropped; one shape per component per view, bound to the root bone `bone` whose rest frame is `f`. */
export function crown(asset: Json, frames: Record<string, Frame>, scale: Scale, solids: readonly Json[], bone: string, f: Frame, pen: number, m: Meta, idPrefix: string): Placed[] {
  const end = BAND[scale][1];
  const gapM = ((0.15 + pen + 0.02) * end) / 1000;
  const featureM = (0.22 * end) / 1000;
  const cell = Math.max(0.002, FEATURE[scale] / 4);
  const meshes = meshesOnFrames(asset, frames, solids, scale);
  const out: Placed[] = [];
  for (const view of VIEWS) {
    const groups = outlineOf(view, meshes, cell, gapM / 2 + cell, featureM / 2, cell * 0.8, gapM * gapM, featureM * featureM);
    groups.forEach((g, i) => {
      const rings = [g.outer, ...g.holes].map((r) => toShapeCoords(view, f, r));
      const c = g.outer.reduce((s, p) => [s[0] + p[0] / g.outer.length, s[1] + p[1] / g.outer.length] as P, [0, 0] as P);
      const at: V3 = view === "elevation" ? [c[0], c[1], 0] : view === "front" ? [0, c[1], c[0]] : [c[0], 0, c[1]];
      out.push({ view, at, shape: plainShape(`${idPrefix}${i === 0 ? "" : `-${i + 1}`}`, bone, rings, m) });
    });
  }
  return out;
}

export { r5 };
