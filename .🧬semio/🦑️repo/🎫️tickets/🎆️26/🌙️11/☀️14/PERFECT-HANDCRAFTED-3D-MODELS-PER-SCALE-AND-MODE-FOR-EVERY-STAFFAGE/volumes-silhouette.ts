/** 🌓️ Owned silhouette, bounds, view-agreement and structure-containment measurements: tiled and resolution-independent (C7), limits recalibrated (C9: IoU 0.90 elevation / 0.88 front and plan, 95th percentile of the boundary distance in both directions at most 2 / 4 smallest features, the maximum is reported only, bounds within max(one feature, 2 % of the axis)), soft masses with the relaxed distance limit (C8b), `texture: true` solids left out (C10), 2D polygons by non-zero winding, open strokes widened to their pen.
 *
 * @see ./volumes-tiles.ts
 * @see ./design.md */
import { LIMITS, SCALES, VIEWS, projectWorld, extentOf, featureM, isAuthored, measure, poseOf, poseState, scaleMeshes, type Json, type Measurement, type View } from "./volumes-lib.ts";
import { addMeshes, addPoly, addStroke, emptySet, rasterize, tileCompare, type ShapeSet } from "./volumes-tiles.ts";

export const SOFT_KINDS: ReadonlySet<string> = new Set(["foliage", "fur", "feather", "hair"]);
export const IOU_LIMIT: Record<View, number> = { elevation: 0.9, front: 0.88, plan: 0.88 };
export const PERCENTILE_FEATURES: Record<View, number> = { elevation: 2, front: 4, plan: 4 };

export const report = (rule: string, scale: number, list: Measurement["list"], name: string, value: number, note?: string): Measurement => ({ ...measure(rule, scale, list, name, value, 0, "max", `reported, not limited${note === undefined ? "" : `; ${note}`}`), ok: true });

/** 🍃️ An asset whose thermal regions are mostly soft masses (foliage, fur, feather, hair) gets the relaxed distance limit of C8b. */
export function softShare(asset: Json): number {
  const regions: Json[] = asset.thermal?.regions ?? [];
  const total = regions.reduce((n, r) => n + (r.areaM2 ?? 0), 0);
  return total <= 0 ? 0 : regions.filter((r) => SOFT_KINDS.has(r.kind)).reduce((n, r) => n + (r.areaM2 ?? 0), 0) / total;
}

/** ✏️ The drawn side of one view of the default pose. */
export function drawnSet(asset: Json, scale: number, view: View, volume?: ShapeSet): ShapeSet {
  const rep = asset.representations[String(scale)];
  const set = emptySet();
  let probe: ReturnType<typeof rasterize> | undefined;
  const filledBy3D = (rings: { x: number; y: number }[][]): boolean => {
    if (volume === undefined) return false;
    const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
    for (const r of rings) for (const p of r) {
      box.minX = Math.min(box.minX, p.x);
      box.maxX = Math.max(box.maxX, p.x);
      box.minY = Math.min(box.minY, p.y);
      box.maxY = Math.max(box.maxY, p.y);
    }
    const cell = Math.max(1e-4, Math.max(box.maxX - box.minX, box.maxY - box.minY) / 120);
    const shape = rasterize({ tris: [], polys: [{ rings, box }] }, box, cell);
    probe = rasterize({ tris: volume.tris.length > 0 ? volume.tris : [], polys: volume.polys }, box, cell);
    let inside = 0;
    let filled = 0;
    for (let i = 0; i < shape.cells.length; i++) if (shape.cells[i]) {
      inside++;
      filled += probe.cells[i]!;
    }
    return inside > 0 && filled >= inside / 2;
  };
  for (const s of poseState(asset, scale)[view]) {
    if (!["body", "detail", "glazing"].includes(s.shape.role) || s.rings.length === 0) continue;
    if (s.closed && (s.shape.filled !== false || filledBy3D(s.rings))) addPoly(set, s.rings);
    else {
      const pen = (((s.shape.penMm ?? rep.pens?.[s.shape.role]) as number | null | undefined) ?? 0.2) * (scale / 1000);
      for (const ring of s.rings) addStroke(set, ring, Math.max(pen, 1e-4), s.closed);
    }
  }
  return set;
}

/** 📏️ The raster cell of a comparison: a quarter of the smallest feature (C7), coarser only where the occupied area would exceed about 6 million cells (then at most 2/3 of a feature); `--fast` uses half a feature, at least 1/400 of the long side. */
export const cellFor = (asset: Json, scale: number, long: number, fast: boolean, occupiedM2?: number): number => {
  const f = featureM(scale, asset);
  if (fast) return Math.max(1e-4, Math.max(f / 2, long / 400));
  const want = Math.max(1e-4, Math.min(f / 4, long / 1000));
  if (occupiedM2 === undefined) return want;
  return Math.max(want, Math.min((f * 2) / 3, Math.sqrt(occupiedM2 / 6e6)));
};

/** 🧮️ The occupied area of a shape set pair, from a cheap coarse pass. */
export function occupiedArea(a: ShapeSet, b: ShapeSet, long: number): number {
  const c0 = Math.max(1e-3, long / 300);
  const r = tileCompare(a, b, { cell: c0, capM: c0 * 4 });
  return Math.max(r.cellsA, r.cellsB) * c0 * c0;
}

/** 📐️ C8a: the 2D viewpoints of one scale must agree on the extent they share (heights between elevation and front, lengths between elevation and plan, widths between front and plan) per bone; the worst disagreement is reported, a defect of the 2D drawing when above one feature. */
export function viewAgreement(asset: Json, scale: number): { worstM: number; note: string } {
  const per = new Map<string, Record<View, { x0: number; x1: number; y0: number; y1: number } | undefined>>();
  const st = poseState(asset, scale);
  for (const view of VIEWS) for (const s of st[view]) {
    if (!["body", "detail", "glazing"].includes(s.shape.role) || typeof s.shape.bone !== "string") continue;
    const e = per.get(s.shape.bone) ?? per.set(s.shape.bone, { elevation: undefined, front: undefined, plan: undefined }).get(s.shape.bone)!;
    const cur = e[view] ?? { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
    for (const r of s.rings) for (const p of r) {
      cur.x0 = Math.min(cur.x0, p.x);
      cur.x1 = Math.max(cur.x1, p.x);
      cur.y0 = Math.min(cur.y0, p.y);
      cur.y1 = Math.max(cur.y1, p.y);
    }
    e[view] = cur;
  }
  let worst = 0;
  let note = "";
  const len = (a: number, b: number) => b - a;
  for (const [bone, e] of per) {
    const [el, fr, pl] = [e.elevation, e.front, e.plan];
    const pairs: [string, number | undefined, number | undefined][] = [
      ["height elevation/front", el && len(el.y0, el.y1), fr && len(fr.y0, fr.y1)],
      ["length elevation/plan", el && len(el.x0, el.x1), pl && len(pl.x0, pl.x1)],
      ["width front/plan", fr && len(fr.x0, fr.x1), pl && len(pl.y0, pl.y1)],
    ];
    for (const [what, a, b] of pairs) if (a !== undefined && b !== undefined && Math.abs(a - b) > worst) {
      worst = Math.abs(a - b);
      note = `${bone}: ${what} ${(a * 1000).toFixed(0)} vs ${(b * 1000).toFixed(0)} mm`;
    }
  }
  return { worstM: worst, note };
}

/** 📐️ C11: the extents of the whole asset as the three drawn viewpoints of a scale give them: height (elevation, front), length (elevation, plan), width (front, plan). */
export function wholeExtents(asset: Json, scale: number): [string, number, number][] {
  const st = poseState(asset, scale);
  const box = (view: View) => {
    const b = { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
    for (const s of st[view]) {
      if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
      const stroke = !(s.closed && s.shape.filled !== false);
      const half = stroke ? ((((s.shape.penMm ?? asset.representations[String(scale)].pens?.[s.shape.role]) as number | null | undefined) ?? 0.2) * (scale / 1000)) / 2 : 0;
      for (const r of s.rings) for (const p of r) {
        b.x0 = Math.min(b.x0, p.x - half);
        b.x1 = Math.max(b.x1, p.x + half);
        b.y0 = Math.min(b.y0, p.y - half);
        b.y1 = Math.max(b.y1, p.y + half);
      }
    }
    return { w: b.x1 - b.x0, h: b.y1 - b.y0 };
  };
  const [el, fr, pl] = [box("elevation"), box("front"), box("plan")];
  return [["height elevation/front", el.h, fr.h], ["length elevation/plan", el.w, pl.w], ["width front/plan", fr.w, pl.h]];
}

/** ✂️ C12: the retained part of a body at a section plane (the cut faces and everything beyond the plane in the viewing direction), as clipped triangles. */
export function addSectioned(set: ShapeSet, view: View, list: readonly SolidMesh[], plane: number): void {
  const cut = { elevation: { axis: 2, retained: 1 }, front: { axis: 0, retained: -1 }, plan: { axis: 1, retained: -1 } }[view];
  for (const m of list) {
    const w = m.world;
    for (let t = 0; t < m.indices.length; t += 3) {
      const P: number[][] = [0, 1, 2].map((k) => [w[m.indices[t + k]! * 3]!, w[m.indices[t + k]! * 3 + 1]!, w[m.indices[t + k]! * 3 + 2]!]);
      const d = P.map((p) => cut.retained * (p[cut.axis]! - plane));
      const poly: number[][] = [];
      for (let i = 0; i < 3; i++) {
        const j = (i + 1) % 3;
        if (d[i]! >= 0) poly.push(P[i]!);
        if (d[i]! >= 0 !== d[j]! >= 0) {
          const u = d[i]! / (d[i]! - d[j]!);
          poly.push([P[i]![0]! + u * (P[j]![0]! - P[i]![0]!), P[i]![1]! + u * (P[j]![1]! - P[i]![1]!), P[i]![2]! + u * (P[j]![2]! - P[i]![2]!)]);
        }
      }
      for (let k = 1; k + 1 < poly.length; k++) {
        const q = [poly[0]!, poly[k]!, poly[k + 1]!].map((p) => projectWorld(view, [p[0]!, p[1]!, p[2]!]));
        set.tris.push(q[0]!.x, q[0]!.y, q[1]!.x, q[1]!.y, q[2]!.x, q[2]!.y);
      }
    }
  }
}

export type SilhouetteOptions = { scale?: number; rules?: readonly string[]; fast?: boolean };

export function silhouetteMeasurements(asset: Json, source: Json, options: SilhouetteOptions = {}): Measurement[] {
  const out: Measurement[] = [];
  const want = (rule: string) => options.rules === undefined || options.rules.includes(rule);
  const volumes: Json = source.volumes ?? {};
  const fast = options.fast === true;
  const dims: [string, number][] = [["widthM", asset.dimensions.widthM], ["heightM", asset.dimensions.heightM], ["depthM", asset.dimensions.depthM]];
  const long = Math.max(...dims.map((d) => d[1]));
  const soft = softShare(asset) > 0.3;
  for (const scale of SCALES) {
    if (!isAuthored(volumes[String(scale)]) || (options.scale !== undefined && scale !== options.scale)) continue;
    const sm = scaleMeshes(asset, scale, volumes[String(scale)]);
    const f = featureM(scale, asset);
    const body = sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true);
    if (body.length === 0) {
      if (want("volumes-silhouette")) out.push(measure("volumes-silhouette", scale, "body", "iou", 0, IOU_LIMIT.elevation, "min", "no body solids"));
      continue;
    }
    if (want("volumes-bounds")) {
      const e = extentOf(sm.body)!;
      dims.forEach(([name, value], k) => out.push(measure("volumes-bounds", scale, "body", `bboxDeviationM:${name}`, Math.abs(e.max[k]! - e.min[k]! - value), Math.max(LIMITS.boundsFeatures * f, 0.02 * value), "max", `bbox ${(e.max[k]! - e.min[k]!).toFixed(4)} vs ${value}`)));
    }
    if (want("volumes-views-disagree")) {
      const w = wholeExtents(asset, scale);
      for (const [name, a, b] of w) out.push({ ...measure("volumes-views-disagree", scale, "body", `extentDifferenceM:${name}`, Math.abs(a - b), Math.max(f, 0.02 * Math.max(a, b)), "max", `${(a * 1000).toFixed(0)} vs ${(b * 1000).toFixed(0)} mm drawn in the two viewpoints (C11: the 2D drawings contradict each other, repair at the root)`) });
      if (!fast) {
        const v = viewAgreement(asset, scale);
        out.push(report("volumes-views-disagree", scale, "body", "perBoneDisagreementM", v.worstM, v.note));
      }
    }
    for (const view of VIEWS) {
      const vol = emptySet();
      const section = asset.representations[String(scale)]?.sections?.[view];
      if (section === undefined) addMeshes(vol, view, body);
      else addSectioned(vol, view, body, section.plane);
      const drawn = drawnSet(asset, scale, view, vol);
      const cell = cellFor(asset, scale, long, fast, fast ? undefined : occupiedArea(drawn, vol, long));
      const extent = Math.max(...(view === "elevation" ? [asset.dimensions.widthM, asset.dimensions.heightM] : view === "front" ? [asset.dimensions.depthM, asset.dimensions.heightM] : [asset.dimensions.widthM, asset.dimensions.depthM]));
      const base = PERCENTILE_FEATURES[view] * f;
      const limit = soft ? Math.max(base, 0.02 * extent) : base;
      if (want("volumes-silhouette")) {
        const r = tileCompare(drawn, vol, { cell, capM: Math.max(1.5 * limit, 6 * f), percentile: 0.95 });
        out.push(measure("volumes-silhouette", scale, "body", `iou:${view}`, r.iou, IOU_LIMIT[view], "min", `pose ${poseOf(asset)}${section === undefined ? "" : `, section ${section.plane} m (C12)`}, cell ${(r.cell * 1000).toFixed(2)} mm, ${r.tiles} tiles`));
        out.push(measure("volumes-silhouette", scale, "body", `boundaryP95M:${view}`, Math.max(r.pAB, r.pBA), limit, "max", `95th percentile drawn to body ${(r.pAB * 1000).toFixed(1)} mm, body to drawn ${(r.pBA * 1000).toFixed(1)} mm${soft ? ", soft-mass limit (C8b)" : ""}`));
        out.push(report("volumes-silhouette", scale, "body", `hausdorffMaxM:${view}`, Math.max(r.maxAB, r.maxBA), `drawn to body ${(r.maxAB * 1000).toFixed(1)} mm, body to drawn ${(r.maxBA * 1000).toFixed(1)} mm (capped at ${(r.capM * 1000).toFixed(0)} mm)`));
      }
      if (want("volumes-structure-coverage") && sm.structure.length > 0) {
        const nonSupport = sm.structure.filter((_, i) => sm.solids.structure[i]?.part !== "support");
        if (nonSupport.length > 0) {
          const st = emptySet();
          addMeshes(st, view, nonSupport);
          const bodyAll = emptySet();
          addMeshes(bodyAll, view, sm.body);
          const r = tileCompare(st, bodyAll, { cell, capM: 4 * f });
          out.push(measure("volumes-structure-coverage", scale, "structure", `outsideBodyM:${view}`, r.maxAB, f, "max"));
        }
      }
    }
  }
  return out;
}
