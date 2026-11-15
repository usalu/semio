/** ⚖️ Owned measurements of the coordinator rulings C1-C6 (design.md) for the authoring check, used until the product `checkVolumes` lands them: exposed thermal area with `sectionFaces` and contact planes (C1), `slender` solids (C2), tolerant cross-scale (C3), monotonic counts at one fixed tessellation quality (C4), support solids with `anchor` (C6). C5 (silhouette Hausdorff) is the product `measureAsset`, unchanged.
 *
 * `volumes-check.ts` switches to the product measurements by itself as soon as `VOLUME_RULES` of the product module lists `volumes-slender`.
 * @see ./design.md */
import { LIMITS as VOLUME_LIMITS, LISTS, SCALES, VIEWS, checkCancelled, expandedCount, extentOf, featureM, isAuthored, localGeometry, measure, minExtent, poseState, qualityOfAsset, scaleMeshes, toWorld, unionVolume, type Json, type Measurement, type SolidMesh, type V3 } from "./volumes-lib.ts";
import { cellFor, occupiedArea } from "./volumes-silhouette.ts";
import { addMeshes, addPoly, emptySet, hullOf, tileCompare, type ShapeSet } from "./volumes-tiles.ts";

export const FIXED_QUALITY = { chordM: 0.002, minSegments: 16, maxSegments: 16 } as const;
export const SLENDER_RATIO = 20;

type Box = { min: V3; max: V3 };
const boxOf = (m: SolidMesh): Box => extentOf([m]) ?? { min: [0, 0, 0], max: [0, 0, 0] };
const BARY: [number, number, number][] = [[1 / 3, 1 / 3, 1 / 3], [0.6, 0.2, 0.2], [0.2, 0.6, 0.2], [0.2, 0.2, 0.6], [0.45, 0.45, 0.1], [0.1, 0.45, 0.45], [0.45, 0.1, 0.45]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

/** 🔎️ A uniform xy grid over the triangles of one closed mesh, so that a vertical ray visits only the triangles above or below its point. */
class RayIndex {
  readonly n: number;
  readonly cells: Int32Array[] = [];
  constructor(readonly mesh: SolidMesh, readonly box: Box) {
    const tris = mesh.indices.length / 3;
    this.n = Math.max(2, Math.min(64, Math.round(Math.sqrt(tris) / 2)));
    const lists: number[][] = Array.from({ length: this.n * this.n }, () => []);
    const w = mesh.world;
    const [sx, sy] = [(box.max[0] - box.min[0]) || 1, (box.max[1] - box.min[1]) || 1];
    for (let t = 0; t < tris; t++) {
      const [a, b, c] = [mesh.indices[t * 3]! * 3, mesh.indices[t * 3 + 1]! * 3, mesh.indices[t * 3 + 2]! * 3];
      const x0 = Math.min(w[a]!, w[b]!, w[c]!), x1 = Math.max(w[a]!, w[b]!, w[c]!);
      const y0 = Math.min(w[a + 1]!, w[b + 1]!, w[c + 1]!), y1 = Math.max(w[a + 1]!, w[b + 1]!, w[c + 1]!);
      const [i0, i1] = [Math.max(0, Math.floor(((x0 - box.min[0]) / sx) * this.n)), Math.min(this.n - 1, Math.floor(((x1 - box.min[0]) / sx) * this.n))];
      const [j0, j1] = [Math.max(0, Math.floor(((y0 - box.min[1]) / sy) * this.n)), Math.min(this.n - 1, Math.floor(((y1 - box.min[1]) / sy) * this.n))];
      for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) lists[j * this.n + i]!.push(t);
    }
    for (const l of lists) this.cells.push(Int32Array.from(l));
  }

  /** 🔎️ Parity of the crossings of the vertical ray above the point (nudged by a micrometre to dodge edges). */
  inside(p: V3): boolean {
    const w = this.mesh.world;
    const [px, py] = [p[0] + 1.3e-6, p[1] + 2.9e-6];
    const [sx, sy] = [(this.box.max[0] - this.box.min[0]) || 1, (this.box.max[1] - this.box.min[1]) || 1];
    const i = Math.min(this.n - 1, Math.max(0, Math.floor(((px - this.box.min[0]) / sx) * this.n)));
    const j = Math.min(this.n - 1, Math.max(0, Math.floor(((py - this.box.min[1]) / sy) * this.n)));
    let crossings = 0;
    for (const t of this.cells[j * this.n + i]!) {
      const [a, b, c] = [this.mesh.indices[t * 3]! * 3, this.mesh.indices[t * 3 + 1]! * 3, this.mesh.indices[t * 3 + 2]! * 3];
      const [ax, ay, bx, by, cx, cy] = [w[a]!, w[a + 1]!, w[b]!, w[b + 1]!, w[c]!, w[c + 1]!];
      const den = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
      if (Math.abs(den) < 1e-18) continue;
      const l0 = ((by - cy) * (px - cx) + (cx - bx) * (py - cy)) / den;
      const l1 = ((cy - ay) * (px - cx) + (ax - cx) * (py - cy)) / den;
      const l2 = 1 - l0 - l1;
      if (l0 < 0 || l1 < 0 || l2 < 0) continue;
      if (l0 * w[a + 2]! + l1 * w[b + 2]! + l2 * w[c + 2]! > p[2]) crossings++;
    }
    return crossings % 2 === 1;
  }
}

type Item = { id: string; base: string; mesh: SolidMesh; box: Box; solid: Json; frame: { u: V3; v: V3; w: V3 }; index?: RayIndex };
const indexOf = (o: Item): RayIndex => (o.index ??= new RayIndex(o.mesh, o.box));
const AXES: Record<string, (f: Item["frame"]) => V3> = { "-u": (f) => f.u.map((x) => -x) as V3, "+u": (f) => f.u, "-v": (f) => f.v.map((x) => -x) as V3, "+v": (f) => f.v, "-w": (f) => f.w.map((x) => -x) as V3, "+w": (f) => f.w };

/** 🧿️ C1: the exposed area of a set of items: faces that are not section faces, not in the contact plane of a support/contact anchor and not buried (the point `0.02 x feature` outside the face lies in no solid of `others` with another base). */
export function exposedArea(items: readonly Item[], others: readonly Item[], planes: readonly { p: V3; n: V3 }[], feature: number, ignoreSection = false): { area: number; total: number } {
  let area = 0;
  let total = 0;
  const eps = feature * 0.02;
  for (const it of items) {
    checkCancelled();
    const w = it.mesh.world;
    const cut = ignoreSection ? [] : ((it.solid.sectionFaces ?? []) as string[]).map((k) => AXES[k]!(it.frame));
    const near = others.filter((o) => o.base !== it.base && (o.mesh as { open?: boolean }).open !== true);
    const triCount = it.mesh.indices.length / 3;
    const stride = Math.max(1, Math.ceil(triCount / Math.max(150, Math.floor(900 / Math.max(1, items.length)))));
    for (let t = 0; t < triCount; t += stride) {
      const [a, b, c] = [it.mesh.indices[t * 3]! * 3, it.mesh.indices[t * 3 + 1]! * 3, it.mesh.indices[t * 3 + 2]! * 3];
      const e1: V3 = [w[b]! - w[a]!, w[b + 1]! - w[a + 1]!, w[b + 2]! - w[a + 2]!];
      const e2: V3 = [w[c]! - w[a]!, w[c + 1]! - w[a + 1]!, w[c + 2]! - w[a + 2]!];
      const nx: V3 = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
      const len = Math.hypot(...nx);
      if (len < 1e-18) continue;
      const tri = (0.5 * len) * stride;
      const n: V3 = [nx[0] / len, nx[1] / len, nx[2] / len];
      total += tri;
      if (cut.some((axis) => dot(n, axis) >= Math.SQRT1_2)) continue;
      let exposed = 0;
      for (const [u, v, x] of BARY) {
        const cen: V3 = [u * w[a]! + v * w[b]! + x * w[c]!, u * w[a + 1]! + v * w[b + 1]! + x * w[c + 1]!, u * w[a + 2]! + v * w[b + 2]! + x * w[c + 2]!];
        if (planes.some((pl) => Math.abs(dot([cen[0] - pl.p[0], cen[1] - pl.p[1], cen[2] - pl.p[2]], pl.n)) <= feature && Math.abs(dot(n, pl.n)) >= 0.7)) continue;
        const q: V3 = [cen[0] + eps * n[0], cen[1] + eps * n[1], cen[2] + eps * n[2]];
        if (near.some((o) => q[0] >= o.box.min[0] && q[0] <= o.box.max[0] && q[1] >= o.box.min[1] && q[1] <= o.box.max[1] && q[2] >= o.box.min[2] && q[2] <= o.box.max[2] && indexOf(o).inside(q))) continue;
        exposed++;
      }
      area += (tri * exposed) / BARY.length;
    }
  }
  return { area, total };
}

function itemsOf(asset: Json, scale: number, sm: ReturnType<typeof scaleMeshes>, volume: Json): Record<"body" | "thermal" | "structure", Item[]> {
  const frames = poseState(asset, scale).frames;
  const out = { body: [] as Item[], thermal: [] as Item[], structure: [] as Item[] };
  for (const l of LISTS) {
    sm.solids[l].forEach((solid, i) => {
      const mesh = sm.solidMeshes[l][i];
      if (mesh === undefined) return;
      const raw = (volume[l] as Json[])?.find((s) => s.id === solid.id);
      const f = frames[solid.bone] ?? { u: [1, 0, 0] as V3, v: [0, 1, 0] as V3, w: [0, 0, 1] as V3 };
      out[l].push({ id: solid.id, base: raw?.of ?? solid.id, mesh, box: boxOf(mesh), solid, frame: f });
    });
  }
  return out;
}

function contactPlanes(asset: Json, scale: number): { p: V3; n: V3 }[] {
  const frames = poseState(asset, scale).frames;
  const supports = new Set<string>(asset.structure?.supports ?? []);
  const planes: { p: V3; n: V3 }[] = [];
  for (const a of asset.anchors ?? []) {
    if (!(a.kind === "support" || a.kind === "contact" || supports.has(a.id)) || typeof a.normalDeg !== "number") continue;
    const f = frames[a.bone];
    if (f === undefined) continue;
    const t = (a.normalDeg * Math.PI) / 180;
    planes.push({ p: toWorld(f, [a.x, a.y, 0]), n: [f.u[0] * Math.cos(t) + f.v[0] * Math.sin(t), f.u[1] * Math.cos(t) + f.v[1] * Math.sin(t), f.u[2] * Math.cos(t) + f.v[2] * Math.sin(t)] });
  }
  return planes;
}

const fixedTriangles = (_asset: Json, solids: readonly Json[]): number => solids.reduce((n, s) => n + localGeometry(s, FIXED_QUALITY).indices.length / 3, 0);

const decisionsMinimal = (volume: Json): boolean => (volume.decisions ?? []).some((d: Json) => String(d?.en ?? d).startsWith(VOLUME_LIMITS.minimalDecision));

/** ⚖️ The measurements of C1-C4 and C6 for the requested scales. */
export function rulingMeasurements(asset: Json, source: Json, options: { scale?: number; rules?: readonly string[]; fast?: boolean } = {}, sms?: Map<number, ReturnType<typeof scaleMeshes>>): Measurement[] {
  const out: Measurement[] = [];
  const want = (rule: string) => options.rules === undefined || options.rules.includes(rule);
  const volumes: Json = source.volumes ?? {};
  const authored = SCALES.filter((s) => isAuthored(volumes[String(s)]));
  const cache = sms ?? new Map<number, ReturnType<typeof scaleMeshes>>();
  const at = (s: number) => cache.get(s) ?? (cache.set(s, scaleMeshes(asset, s, volumes[String(s)])), cache.get(s)!);
  const targets = authored.filter((s) => options.scale === undefined || s === options.scale);
  for (const scale of targets) {
    checkCancelled();
    const volume = volumes[String(scale)];
    const feature = featureM(scale, asset);
    const sm = at(scale);
    if (want("volumes-feature") || want("volumes-slender"))
      for (const l of LISTS) {
        const list = sm.solids[l];
        let worst = Infinity;
        let who = "";
        let below = 0;
        const drawnMin = drawnMinExtents(asset, scale);
        list.forEach((s) => {
          const thin = (VOLUME_LIMITS.thinRoles as readonly string[]).includes(s.role);
          const { repeat: _r, ...single } = s;
          if (s.texture === true && want("volumes-feature")) return;
          if (s.slender === true) {
            if (!want("volumes-slender")) return;
            const ext = extentsOf(asset, scale, single);
            out.push(measure("volumes-slender", scale, l, `slenderRatio:${s.id}`, ext[2] / Math.max(ext[1], 1e-9), SLENDER_RATIO, "min", `length ${(ext[2] * 1000).toFixed(1)} mm, largest cross extent ${(ext[1] * 1000).toFixed(1)} mm`));
            return;
          }
          if (thin || !want("volumes-feature")) return;
          const d = minExtent(asset, scale, s);
          const limit = Math.min(feature, drawnMin.get(s.bone) ?? Infinity);
          if (d < limit - 1e-9) below++;
          if ((d / limit) * feature < worst) {
            worst = (d / limit) * feature;
            who = limit < feature ? `${s.id} (limit ${(limit * 1000).toFixed(1)} mm = its thinnest drawn 2D extent, C13)` : s.id;
          }
        });
        if (list.length > 0 && want("volumes-feature")) {
          out.push(measure("volumes-feature", scale, l, "smallestFeatureM", worst, feature, "min", `smallest: ${who}`));
          out.push(measure("volumes-feature", scale, l, "solidsBelowFeature", below, 0, "max"));
        }
      }
    if (want("volumes-thermal-coverage") && sm.thermal.length > 0 && options.fast !== true) {
      const it = itemsOf(asset, scale, sm, volume);
      const planes = contactPlanes(asset, scale);
      const all = [...it.body, ...it.thermal, ...it.structure];
      const tolerance = scale === 50 ? VOLUME_LIMITS.thermalAreaFine : VOLUME_LIMITS.thermalAreaCoarse;
      const perRegion = scale <= 100;
      let sumArea = 0;
      let sumDeclared = 0;
      for (const r of (asset.thermal?.regions ?? []) as Json[]) {
        const own = it.thermal.filter((x) => x.solid.region === r.id);
        if (own.length === 0 || !(r.areaM2 > 0)) continue;
        const layers: string[] = (r.layers ?? []).map((l: Json) => l.id);
        const rank = (x: Item) => (x.solid.layer === undefined ? -1 : layers.indexOf(x.solid.layer));
        const top = Math.max(...own.map(rank));
        const outer = own.filter((x) => rank(x) === top);
        const exchange = r.model === "leaf" || ["foliage", "fur", "feather", "hair"].includes(r.kind);
        if (exchange) {
          out.push({ ...measure("volumes-thermal-coverage", scale, "thermal", `areaDeviation:${r.id}`, 0, tolerance, "max", `exchange area, not compared (C16): areaM2 ${r.areaM2} is a leaf/fur/feather/hair exchange area, not a modelled surface`), ok: true });
          continue;
        }
        const { area, total } = exposedArea(outer, r.hidden === true ? [] : all, planes, feature);
        sumArea += area;
        sumDeclared += r.areaM2;
        const warn = area / r.areaM2 > 1.5 || area / r.areaM2 < 1 / 1.5 ? `WARNING: measured/declared = ${(area / r.areaM2).toFixed(2)}, outside the factor 1.5: check the model and the declared area before anything else; ` : "";
        const m = measure("volumes-thermal-coverage", scale, "thermal", `areaDeviation:${r.id}`, Math.abs(area / r.areaM2 - 1), tolerance, "max", `${warn}exposed ${area.toFixed(4)} m2 of ${total.toFixed(4)} m2 mesh, areaM2 ${r.areaM2}${perRegion ? "" : "; per region reported only at this scale (C15)"}`);
        out.push(perRegion ? m : { ...m, ok: true });
      }
      if (!perRegion && sumDeclared > 0) out.push(measure("volumes-thermal-coverage", scale, "thermal", "areaDeviationTotal", Math.abs(sumArea / sumDeclared - 1), VOLUME_LIMITS.thermalAreaCoarse, "max", `exposed ${sumArea.toFixed(4)} m2 over all regions vs areaM2 sum ${sumDeclared.toFixed(4)} m2 (C15)`));
    }
    if (want("volumes-structure-coverage")) {
      const it = itemsOf(asset, scale, sm, volume);
      const supports: string[] = asset.structure?.supports ?? [];
      const frames = poseState(asset, scale).frames;
      const e = extentOf(sm.body);
      for (const id of supports) {
        const mine = it.structure.filter((x) => x.solid.part === "support" && x.solid.anchor === id);
        out.push(measure("volumes-structure-coverage", scale, "structure", `supportSolidsOfAnchor:${id}`, mine.length, 1, "min", mine.length === 0 ? `no support solid with anchor "${id}"` : ""));
        out.push(measure("volumes-structure-coverage", scale, "structure", `supportSolidsExtra:${id}`, Math.max(0, mine.length - 1), 0, "max"));
        const a = (asset.anchors ?? []).find((x: Json) => x.id === id);
        for (const x of mine) {
          const f = frames[a?.bone];
          const p = a === undefined || f === undefined ? null : toWorld(f, [a.x, a.y, 0]);
          const b = x.box;
          const atAnchor = p !== null && [0, 1].every((k) => p[k]! >= b.min[k]! - feature && p[k]! <= b.max[k]! + feature);
          const ground = e !== null && Math.abs(b.min[1] - e.min[1]) <= feature;
          const off = atAnchor || ground ? 0 : p === null ? Infinity : Math.hypot(Math.max(0, b.min[0] - p[0], p[0] - b.max[0]), Math.max(0, b.min[1] - p[1], p[1] - b.max[1]));
          out.push(measure("volumes-structure-coverage", scale, "structure", `supportOffM:${id}`, off, feature, "max"));
        }
      }
      const withoutAnchor = it.structure.filter((x) => x.solid.part === "support" && !supports.includes(x.solid.anchor)).map((x) => x.id);
      out.push(measure("volumes-structure-coverage", scale, "structure", "supportSolidsWithoutAnchor", withoutAnchor.length, 0, "max", withoutAnchor.slice(0, 5).join(", ")));
    }
  }
  for (let i = 1; i < authored.length; i++) {
    const [a, b] = [authored[i - 1]!, authored[i]!];
    if (options.scale !== undefined && b !== options.scale) continue;
    const [ca, cb] = [at(a), at(b)];
    if (want("volumes-monotonic")) {
      const minimal = decisionsMinimal(volumes[String(b)]);
      for (const l of LISTS) {
        const [na, nb] = [expandedCount(ca.solids[l]), expandedCount(cb.solids[l])];
        const [ta, tb] = [fixedTriangles(asset, ca.solids[l]), fixedTriangles(asset, cb.solids[l])];
        out.push(measure("volumes-monotonic", b, l, "solidCountDelta", nb - na, 0, "max", `${na} -> ${nb}`));
        out.push(measure("volumes-monotonic", b, l, "triangleCountDelta", tb - ta, 0, "max", `${ta} -> ${tb} at fixed quality (chord 2 mm, 16 segments)`));
        out.push(measure("volumes-monotonic", b, l, "strictDrop", nb < na || tb < ta || minimal ? 1 : 0, 1, "min", minimal ? "declared Minimal:" : `needs a drop or a decision starting "${VOLUME_LIMITS.minimalDecision}"`));
      }
    }
    if (want("volumes-cross-scale")) {
      const envelopeBones = new Set<string>(cb.solids.body.filter((x) => x.envelope === true).map((x) => x.bone));
      const keepFine = (): SolidMesh[] => ca.body.filter((_, k) => ca.solids.body[k]!.slender !== true && ca.solids.body[k]!.texture !== true && !envelopeBones.has(ca.solids.body[k]!.bone));
      const keepCoarse = (): SolidMesh[] => cb.body.filter((_, k) => cb.solids.body[k]!.slender !== true && cb.solids.body[k]!.texture !== true);
      const [fineBody, coarseBody] = [keepFine(), keepCoarse()];
      const fineEnvelope = [...envelopeBones].map((bone) => ca.body.filter((_, k) => ca.solids.body[k]!.bone === bone && ca.solids.body[k]!.slender !== true && ca.solids.body[k]!.texture !== true));
      if ((fineBody.length === 0 && fineEnvelope.every((x) => x.length === 0)) || coarseBody.length === 0) {
        out.push(measure("volumes-cross-scale", b, "body", "iou", 0, VOLUME_LIMITS.crossIou, "min", "empty body (slender, texture and envelope solids excluded)"));
        continue;
      }
      const f = featureM(b, asset);
      const long = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
      for (const view of VIEWS) {
        const fine = emptySet();
        addMeshes(fine, view, fineBody);
        for (const group of fineEnvelope) {
          const pts: { x: number; y: number }[] = [];
          const tmp = emptySet();
          addMeshes(tmp, view, group);
          for (let i = 0; i < tmp.tris.length; i += 2) pts.push({ x: tmp.tris[i]!, y: tmp.tris[i + 1]! });
          const hull = hullOf(pts);
          if (hull.length >= 3) addPoly(fine, [hull]);
        }
        const coarse = emptySet();
        addMeshes(coarse, view, coarseBody);
        const cell = cellFor(asset, b, long, false, occupiedArea(coarse, fine, long));
        const r = tileCompare(coarse, fine, { cell, tolerance: f, capM: 6 * f, percentile: 0.95 });
        const pair = pair2D(asset, a, b, view, cell);
        const iouLimit = Math.min(VOLUME_LIMITS.crossIou, pair.iou - 0.05);
        const distLimit = Math.max(VOLUME_LIMITS.crossHausdorffFeatures * f, pair.p95 + f);
        out.push(measure("volumes-cross-scale", b, "body", `iou:${view}`, r.tolerantIou, iouLimit, "min", `1:${a} vs 1:${b}; limit min(${VOLUME_LIMITS.crossIou}, 2D pair IoU ${pair.iou.toFixed(3)} - 0.05) (C14); mismatch within ${(f * 1000).toFixed(1)} mm forgiven${envelopeBones.size > 0 ? "; envelope bones as convex hulls" : ""}`));
        out.push(measure("volumes-cross-scale", b, "body", `boundaryP95M:${view}`, Math.max(r.pAB, r.pBA), distLimit, "max", `2D pair p95 ${(pair.p95 * 1000).toFixed(1)} mm, limit max(2 features, p95 + 1 feature) (C14)`));
        out.push({ ...measure("volumes-cross-scale", b, "body", `hausdorffMaxM:${view}`, Math.max(r.maxAB, r.maxBA), 0, "max", "reported, not limited"), ok: true });
      }
      const [vf, vc] = [unionVolume(fineBody), unionVolume(coarseBody)];
      const itFine = itemsOf(asset, a, ca, volumes[String(a)]).body.filter((x) => x.solid.slender !== true && x.solid.texture !== true && !envelopeBones.has(x.solid.bone));
      const surface = exposedArea(itFine, itFine, [], f, true).area;
      const low = Math.max(0, vf - surface * f);
      const high = vf + surface * f;
      out.push(measure("volumes-cross-scale", b, "body", "unionVolumeBandExcess", Math.max(0, low - vc, vc - high) / Math.max(vf, 1e-12), 0, "max", `coarse ${vc.toFixed(5)} m3 within [${low.toFixed(5)}, ${high.toFixed(5)}] m3 (fine ${vf.toFixed(5)} m3 eroded/dilated by ${(f * 1000).toFixed(1)} mm, first order by the surface ${surface.toFixed(3)} m2)`));
    }
  }
  return out;
}

/** 📏️ C13: per bone the thinnest extent of a filled closed 2D shape drawn at that scale (the limit of the bone's solids is never above it). */
export function drawnMinExtents(asset: Json, scale: number): Map<string, number> {
  const out = new Map<string, number>();
  const st = poseState(asset, scale);
  for (const view of VIEWS) for (const s of st[view]) {
    if (!s.closed || s.shape.filled === false || !["body", "detail", "glazing"].includes(s.shape.role) || typeof s.shape.bone !== "string" || s.rings.length === 0) continue;
    const b = { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
    for (const p of s.rings[0]!) {
      b.x0 = Math.min(b.x0, p.x);
      b.x1 = Math.max(b.x1, p.x);
      b.y0 = Math.min(b.y0, p.y);
      b.y1 = Math.max(b.y1, p.y);
    }
    const t = Math.min(b.x1 - b.x0, b.y1 - b.y0);
    if (t > 0) out.set(s.shape.bone, Math.min(out.get(s.shape.bone) ?? Infinity, t));
  }
  return out;
}

const pairCache = new Map<string, { iou: number; p95: number }>();

/** 🖼️ C14: the 2D drawings' own agreement between two scales in one view (filled closed shapes, non-zero winding). */
export function pair2D(asset: Json, a: number, b: number, view: (typeof VIEWS)[number], cell: number): { iou: number; p95: number } {
  const key = `${asset.id}|${a}|${b}|${view}|${cell}`;
  const hit = pairCache.get(key);
  if (hit !== undefined) return hit;
  const filled = (scale: number): ShapeSet => {
    const set = emptySet();
    for (const s of poseState(asset, scale)[view]) if (s.closed && s.shape.filled !== false && ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0) addPoly(set, s.rings);
    return set;
  };
  const f = featureM(b, asset);
  const r = tileCompare(filled(b), filled(a), { cell, capM: 6 * f, percentile: 0.95 });
  const out = { iou: r.iou, p95: Math.max(r.pAB, r.pBA) };
  pairCache.set(key, out);
  return out;
}

function extentsOf(asset: Json, scale: number, solid: Json): [number, number, number] {
  const m = scaleMeshes(asset, scale, { body: [solid], thermal: [], structure: [] }).body[0];
  if (m === undefined) return [0, 0, 0];
  const n = m.world.length / 3;
  const mean = [0, 0, 0];
  for (let i = 0; i < n; i++) for (let k = 0; k < 3; k++) mean[k]! += m.world[i * 3 + k]! / n;
  const c = [0, 0, 0, 0, 0, 0];
  for (let i = 0; i < n; i++) {
    const d = [0, 1, 2].map((k) => m.world[i * 3 + k]! - mean[k]!);
    c[0]! += d[0]! * d[0]!; c[1]! += d[0]! * d[1]!; c[2]! += d[0]! * d[2]!; c[3]! += d[1]! * d[1]!; c[4]! += d[1]! * d[2]!; c[5]! += d[2]! * d[2]!;
  }
  const a = [[c[0]!, c[1]!, c[2]!], [c[1]!, c[3]!, c[4]!], [c[2]!, c[4]!, c[5]!]];
  const v = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  for (let sweep = 0; sweep < 30; sweep++) for (const [p, q] of [[0, 1], [0, 2], [1, 2]] as const) {
    if (Math.abs(a[p]![q]!) < 1e-14) continue;
    const theta = (a[q]![q]! - a[p]![p]!) / (2 * a[p]![q]!);
    const t = Math.sign(theta || 1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
    const cs = 1 / Math.sqrt(t * t + 1);
    const sn = t * cs;
    for (let k = 0; k < 3; k++) { const x = a[k]![p]!, y = a[k]![q]!; a[k]![p] = cs * x - sn * y; a[k]![q] = sn * x + cs * y; }
    for (let k = 0; k < 3; k++) { const x = a[p]![k]!, y = a[q]![k]!; a[p]![k] = cs * x - sn * y; a[q]![k] = sn * x + cs * y; }
    for (let k = 0; k < 3; k++) { const x = v[k]![p]!, y = v[k]![q]!; v[k]![p] = cs * x - sn * y; v[k]![q] = sn * x + cs * y; }
  }
  const ext = [0, 1, 2].map((axis) => {
    let lo = Infinity, hi = -Infinity;
    for (let i = 0; i < n; i++) {
      const d = m.world[i * 3]! * v[0]![axis]! + m.world[i * 3 + 1]! * v[1]![axis]! + m.world[i * 3 + 2]! * v[2]![axis]!;
      if (d < lo) lo = d;
      if (d > hi) hi = d;
    }
    return hi - lo;
  }).sort((x, y) => x - y);
  return [ext[0]!, ext[1]!, ext[2]!];
}


/** 🧿️ The buried-only outer area of a region's outermost-layer solids (no section faces, no contact planes): what an independent oracle can reproduce. */
export function bareOuterArea(asset: Json, scale: number, volume: Json, regionId: string): number {
  const sm = scaleMeshes(asset, scale, volume);
  const it = itemsOf(asset, scale, sm, volume);
  const r = (asset.thermal.regions as Json[]).find((x) => x.id === regionId);
  const layers: string[] = (r?.layers ?? []).map((l: Json) => l.id);
  const rank = (x: Item) => (x.solid.layer === undefined ? -1 : layers.indexOf(x.solid.layer));
  const own = it.thermal.filter((x) => x.solid.region === regionId);
  if (own.length === 0) return 0;
  const top = Math.max(...own.map(rank));
  return exposedArea(own.filter((x) => rank(x) === top), r?.hidden === true ? [] : [...it.body, ...it.thermal, ...it.structure], [], featureM(scale, asset), true).area;
}
