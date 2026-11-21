/** 🧰️ Shared helpers of the per-scale volume authoring tools: sources, merging over the asset, `of` resolution, member bones (amendment B1), measurements of every design rule on the owned kernel and the owned raster.
 *
 * Builds on `./volumes-base.ts` (catalog access, rest frames, masks, meshes). The product-side `checkVolumes` of `schema-api.md` is the final truth; these measurements keep the authors unblocked and use the same rule ids.
 * @see ./design.md
 * @see ./volumes-tools.md */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { VOLUME_LIMITS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { CancelledError, GENERATED, Mask, TICKET, VIEWS, checkCancelled, extentOf, frames, loadAsset, meshes, projectWorld, resolveAsset, restState, toWorld, type Box, type CatalogEntry, type Json, type P2, type RestState, type Ring, type SolidMesh, type V3, type View } from "./volumes-base.ts";

export * from "./volumes-base.ts";

const PRODUCT_LIMITS = VOLUME_LIMITS as unknown as Record<string, unknown>;

/** 🚦️ The limits of the design; the product values win, these defaults keep the tool running while the product module is edited. */
export const LIMITS = { silhouetteIou: 0.92, silhouetteHausdorffFeatures: 2, boundsFeatures: 1, crossIou: 0.85, crossHausdorffFeatures: 2, crossVolume: 0.15, thermalAreaFine: 0.15, thermalAreaCoarse: 0.3, thermalBoxFeatures: 1, thinRoles: ["glazing", "texture", "ground"], minimalDecision: "Minimal:", minFeatureMm: 0.15, ...PRODUCT_LIMITS } as typeof VOLUME_LIMITS & { crossVolume: number };

export const SCALES = [50, 100, 200, 500] as const;
export type Scale = (typeof SCALES)[number];
export type ListName = "body" | "thermal" | "structure";
export const LISTS: readonly ListName[] = ["body", "thermal", "structure"];
export const TOOLS_OUT = join(GENERATED, "volumes-tools");
export const SOURCES = join(TICKET, "volumes");
export const THIN_SHEET_ROLES: ReadonlySet<string> = new Set(LIMITS.thinRoles);

/** 📏️ The smallest printable feature at a scale: 0.15 mm on paper (print profile `office-laser`), unless the asset says otherwise. */
export const featureM = (scale: number, asset?: Json): number => ((asset?.presentation?.printProfile?.minFeatureMm ?? asset?.printProfile?.minFeatureMm ?? LIMITS.minFeatureMm) * scale) / 1000;

export const sourcePath = (entry: CatalogEntry): string => join(SOURCES, entry.category, `${entry.slug}.json`);
export const outPath = (kind: string, entry: CatalogEntry, name: string): string => join(TOOLS_OUT, kind, entry.category, name);

export function writeJson(path: string, value: unknown, pretty = false): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, pretty ? 1 : undefined)}\n`);
}

export function loadSource(entry: CatalogEntry): Json | null {
  const path = sourcePath(entry);
  return existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : null;
}

export function entryOf(spec: string): { entry: CatalogEntry; asset: Json } {
  const entry = resolveAsset(spec);
  return { entry, asset: loadAsset(entry.path) };
}

/** 🔀️ The asset with the source `volumes` merged over it and the old `volume` removed (no splice needed). */
export function merged(asset: Json, source: Json): Json {
  const { volume: _old, ...rest } = asset;
  return { ...rest, volumes: source.volumes };
}

export const isAuthored = (v: Json | undefined): boolean => v !== undefined && v.status === "authored";

/** 🔗️ A list of a scale with every `{ of }` reference resolved to the full body solid plus its own tags; `of` that does not resolve is reported through `unresolved`. */
export function resolveList(volume: Json, list: ListName, unresolved: string[] = []): Json[] {
  const raw: Json[] = volume?.[list] ?? [];
  if (list === "body") return raw;
  const body = new Map<string, Json>((volume?.body ?? []).map((s: Json) => [s.id, s]));
  return raw.flatMap((s) => {
    if (s.of === undefined) return [s];
    const base = body.get(s.of);
    if (base === undefined) {
      unresolved.push(`${list}:${s.id} -> ${s.of}`);
      return [];
    }
    const { id: _i, of: _o, ...tags } = s;
    const { region: _r, regions: _rs, maxDenominator: _m, ...geometry } = base;
    return [{ ...geometry, ...tags, id: s.id, role: s.role ?? base.role }];
  });
}

export const expandedCount = (solids: readonly Json[]): number => solids.reduce((n, s) => n + (s.repeat?.count ?? 1), 0);

/** 🦴️ Amendment B1: the bones that carry a member solid: `eligibleBones` of the 2D structural view at the drawn height, plus the bones of the 2D `member` shapes of the scale. */
export function memberBones(asset: Json, scale: number): string[] {
  const height = Math.max(1e-6, asset.dimensions.heightM);
  const eligible = (id: string): boolean => asset.rig.bones.some((b: Json) => b.id === id && (b.section !== undefined || b.lengthM >= 0.05 * height));
  const ids = new Set<string>(asset.rig.bones.filter((b: Json) => eligible(b.id)).map((b: Json) => b.id));
  for (const s of memberShapes(asset, scale)) ids.add(s.bone);
  return [...ids];
}

/** 📊️ The distinct 2D `member` shapes of a scale (one per id, all views), bone-local ends and section. */
export function memberShapes(asset: Json, scale: number): Json[] {
  const rep = asset.representations?.[String(scale)];
  if (rep?.status !== "authored") return [];
  const seen = new Map<string, Json>();
  for (const view of VIEWS) for (const s of rep.geometry[view] ?? []) if (s.kind === "member" && !seen.has(s.id)) seen.set(s.id, s);
  return [...seen.values()];
}

export function shapeBones(asset: Json, scale: number): Set<string> {
  const rep = asset.representations?.[String(scale)];
  const out = new Set<string>();
  if (rep?.status !== "authored") return out;
  for (const view of VIEWS) for (const s of rep.geometry[view] ?? []) if (["body", "detail", "glazing"].includes(s.role) && typeof s.bone === "string") out.add(s.bone);
  return out;
}

export const poseOf = (asset: Json): string => asset.presentation?.defaultPose;

export type PoseState = RestState & { frames: Record<string, Frame>; pose: string };
type Frame = { o: V3; u: V3; v: V3; w: V3 };
const poseCache = new WeakMap<object, Map<number, PoseState>>();

/** 🧍️ Amendment A6: the evaluated `presentation.defaultPose` at 1:`scale` (2D shapes of the three views and the bone frames of the pose), cached per asset object. */
export function poseState(asset: Json, scale: number): PoseState {
  const hit = poseCache.get(asset)?.get(scale);
  if (hit !== undefined) return hit;
  const pose = poseOf(asset);
  const drawn = Object.entries((asset.representations ?? {}) as Record<string, Json>).filter(([, r]) => r.geometry !== undefined).map(([d]) => Number(d));
  const drawnScale = drawn.includes(scale) || drawn.length === 0 ? scale : drawn.reduce((best, d) => (Math.abs(Math.log(d / scale)) < Math.abs(Math.log(best / scale)) ? d : best));
  if (drawnScale !== scale) {
    const near = poseState(asset, drawnScale);
    const out: PoseState = { ...near, elevation: [], front: [], plan: [] };
    (poseCache.get(asset) ?? poseCache.set(asset, new Map()).get(asset)!).set(scale, out);
    return out;
  }
  const run = (a: Json): Json => evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } }) as Json;
  let state: Json;
  let degraded: string | undefined;
  try {
    state = run(asset);
  } catch (error) {
    const strip = (list: Json[]): Json[] => list.map(({ solid: _solid, ...rest }) => rest);
    const rep = asset.representations[String(scale)];
    state = run({ ...asset, representations: { ...asset.representations, [String(scale)]: { ...rep, geometry: { ...rep.geometry, elevation: strip(rep.geometry.elevation), front: strip(rep.geometry.front), plan: strip(rep.geometry.plan) } } } });
    degraded = (error as Error).message;
  }
  const frameMap: Record<string, Frame> = {};
  for (const [id, b] of Object.entries(state.bones as Record<string, { frame: { o: Json; u: Json; v: Json; w: Json } }>)) {
    const f = b.frame;
    frameMap[id] = { o: [f.o.x, f.o.y, f.o.z], u: [f.u.x, f.u.y, f.u.z], v: [f.v.x, f.v.y, f.v.z], w: [f.w.x, f.w.y, f.w.z] };
  }
  const out: PoseState = { elevation: state.shapes.elevation, front: state.shapes.front, plan: state.shapes.plan, frames: frameMap, pose, ...(degraded === undefined ? {} : { degraded }) };
  const per = poseCache.get(asset) ?? poseCache.set(asset, new Map()).get(asset)!;
  per.set(scale, out);
  return out;
}

const GEOMETRY_META = new Set(["id", "material", "tone", "role", "region", "regions", "layer", "tags", "member", "part", "anchor", "of", "maxDenominator", "slender", "texture", "envelope", "sectionFaces", "bone", "node", "weights"]);
const tessMemory = new Map<string, { positions: Float64Array; indices: Uint32Array }>();
export const quality = { fast: false };

/** 🗄️ Content key of a solid's geometry: everything that shapes the mesh and nothing that only labels it. */
export function geometryKey(solid: Json, q: object): string {
  const keep: Record<string, unknown> = {};
  for (const k of Object.keys(solid).sort()) if (!GEOMETRY_META.has(k)) keep[k] = solid[k];
  return createHash("sha1").update(JSON.stringify([keep, q])).digest("hex");
}

/** ⚡️ The tessellation of one solid in its bone frame (transform and repeat applied), from memory, else from the disk cache under `🗑️generated/volumes-tools/cache/tess`, else computed once by the kernel and stored. */
export function localGeometry(solid: Json, q: TessellationQuality): { positions: Float64Array; indices: Uint32Array } {
  const key = geometryKey(solid, q);
  const hit = tessMemory.get(key);
  if (hit !== undefined) return hit;
  const path = join(TOOLS_OUT, "cache", "tess", key.slice(0, 2), `${key}.bin`);
  let out: { positions: Float64Array; indices: Uint32Array } | undefined;
  try {
    const raw = readFileSync(path);
    const head = new Uint32Array(raw.buffer.slice(raw.byteOffset, raw.byteOffset + 8));
    const positions = new Float32Array(raw.buffer.slice(raw.byteOffset + 8, raw.byteOffset + 8 + head[0]! * 4));
    const indices = new Uint32Array(raw.buffer.slice(raw.byteOffset + 8 + head[0]! * 4, raw.byteOffset + 8 + head[0]! * 4 + head[1]! * 4));
    out = { positions: Float64Array.from(positions), indices };
  } catch {
    out = undefined;
  }
  if (out === undefined) {
    const g = tessellateSolid({ ...solid, node: solid.bone } as never, q);
    out = { positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
    try {
      mkdirSync(dirname(path), { recursive: true });
      const body = Buffer.alloc(8 + out.positions.length * 4 + out.indices.length * 4);
      body.writeUInt32LE(out.positions.length, 0);
      body.writeUInt32LE(out.indices.length, 4);
      Buffer.from(Float32Array.from(out.positions).buffer).copy(body, 8);
      Buffer.from(out.indices.buffer, out.indices.byteOffset, out.indices.byteLength).copy(body, 8 + out.positions.length * 4);
      const tmp = `${path}.${process.pid}.tmp`;
      writeFileSync(tmp, body);
      renameSync(tmp, path);
    } catch {
      void 0;
    }
  }
  tessMemory.set(key, out);
  return out;
}

export type TessellationQuality = { chordM: number; minSegments: number; maxSegments: number; round?: { radiusM: number; segments: number } };

/** 🎚️ The product's own tessellation quality (`qualityOf`, the rule of the viewer and of every 3D export): at 1:`scale` what the product shows at that scale, without a scale the reference the catalog validator measures with; `--fast` is a coarser iteration quality. */
export const qualityOfAsset = (asset: Json, scale?: number): TessellationQuality => {
  const q = qualityOf(asset as never, scale);
  return quality.fast ? { chordM: q.chordM * 2.5, minSegments: 10, maxSegments: 32 } : q;
};

/** 🧊️ Solids carried to the world by the bone frames of the default pose at 1:`scale`, from the tessellation cache. */
export function meshesAt(asset: Json, scale: number, solids: readonly Json[]): SolidMesh[] {
  const frame = poseState(asset, scale).frames;
  const q = qualityOfAsset(asset, scale);
  return solids.map((solid) => {
    checkCancelled();
    const f = frame[solid.bone];
    if (f === undefined) throw new Error(`solid "${solid.id}" is bound to unknown bone "${solid.bone}"`);
    const g = localGeometry(solid, q);
    const world = new Float64Array(g.positions.length);
    const [o, u, v, w] = [f.o, f.u, f.v, f.w];
    for (let i = 0; i < g.positions.length; i += 3) {
      const [a, b, c] = [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!];
      world[i] = o[0] + a * u[0] + b * v[0] + c * w[0];
      world[i + 1] = o[1] + a * u[1] + b * v[1] + c * w[1];
      world[i + 2] = o[2] + a * u[2] + b * v[2] + c * w[2];
    }
    const open = (solid.kind === "loft" && (solid.capStart === false || solid.capEnd === false)) || (solid.kind === "revolve" && solid.sweepRad !== undefined && solid.sweepRad < 2 * Math.PI - 1e-6) || solid.open === true;
    return { id: solid.id, role: solid.role, world, indices: g.indices, ...(open ? { open: true } : {}) } as SolidMesh;
  });
}

export type Measurement = { rule: string; scale?: number; list?: ListName; name: string; value: number; limit: number; bound: "min" | "max"; ok: boolean; note?: string };

export const measure = (rule: string, scale: number | undefined, list: ListName | undefined, name: string, value: number, limit: number, bound: "min" | "max", note?: string): Measurement => ({
  rule,
  ...(scale === undefined ? {} : { scale }),
  ...(list === undefined ? {} : { list }),
  name,
  value: Number.isFinite(value) ? Number(value.toPrecision(5)) : value,
  limit: Number((Number.isFinite(limit) ? limit : 0).toPrecision(5)),
  bound,
  ok: Number.isFinite(value) && (bound === "min" ? value >= limit : value <= limit),
  ...(note === undefined ? {} : { note }),
});

const triArea = (w: Float64Array, a: number, b: number, c: number): number => {
  const ux = w[b * 3]! - w[a * 3]!;
  const uy = w[b * 3 + 1]! - w[a * 3 + 1]!;
  const uz = w[b * 3 + 2]! - w[a * 3 + 2]!;
  const vx = w[c * 3]! - w[a * 3]!;
  const vy = w[c * 3 + 1]! - w[a * 3 + 1]!;
  const vz = w[c * 3 + 2]! - w[a * 3 + 2]!;
  return 0.5 * Math.hypot(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx);
};

export const meshArea = (m: SolidMesh): number => {
  let a = 0;
  for (let t = 0; t < m.indices.length; t += 3) a += triArea(m.world, m.indices[t]!, m.indices[t + 1]!, m.indices[t + 2]!);
  return a;
};

const boundsOf = (m: SolidMesh): { min: V3; max: V3 } => extentOf([m]) ?? { min: [0, 0, 0], max: [0, 0, 0] };

/** 📐️ Smallest extent of a solid over its three principal axes (covariance eigenvectors of the vertices), repeat copies ignored. */
export function minExtent(asset: Json, scale: number, solid: Json): number {
  const { repeat: _r, ...single } = solid;
  const [m] = meshesAt(asset, scale, [single]);
  if (m === undefined || m.world.length === 0) return 0;
  const n = m.world.length / 3;
  const mean = [0, 0, 0];
  for (let i = 0; i < n; i++) for (let k = 0; k < 3; k++) mean[k]! += m.world[i * 3 + k]! / n;
  const c = [0, 0, 0, 0, 0, 0];
  for (let i = 0; i < n; i++) {
    const d = [0, 1, 2].map((k) => m.world[i * 3 + k]! - mean[k]!);
    c[0]! += d[0]! * d[0]!;
    c[1]! += d[0]! * d[1]!;
    c[2]! += d[0]! * d[2]!;
    c[3]! += d[1]! * d[1]!;
    c[4]! += d[1]! * d[2]!;
    c[5]! += d[2]! * d[2]!;
  }
  const a = [[c[0]!, c[1]!, c[2]!], [c[1]!, c[3]!, c[4]!], [c[2]!, c[4]!, c[5]!]];
  const v = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  for (let sweep = 0; sweep < 30; sweep++) {
    for (const [p, q] of [[0, 1], [0, 2], [1, 2]] as const) {
      if (Math.abs(a[p]![q]!) < 1e-14) continue;
      const theta = (a[q]![q]! - a[p]![p]!) / (2 * a[p]![q]!);
      const t = Math.sign(theta || 1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
      const cs = 1 / Math.sqrt(t * t + 1);
      const sn = t * cs;
      for (let k = 0; k < 3; k++) {
        const akp = a[k]![p]!;
        const akq = a[k]![q]!;
        a[k]![p] = cs * akp - sn * akq;
        a[k]![q] = sn * akp + cs * akq;
      }
      for (let k = 0; k < 3; k++) {
        const apk = a[p]![k]!;
        const aqk = a[q]![k]!;
        a[p]![k] = cs * apk - sn * aqk;
        a[q]![k] = sn * apk + cs * aqk;
      }
      for (let k = 0; k < 3; k++) {
        const vkp = v[k]![p]!;
        const vkq = v[k]![q]!;
        v[k]![p] = cs * vkp - sn * vkq;
        v[k]![q] = sn * vkp + cs * vkq;
      }
    }
  }
  let best = Infinity;
  for (let axis = 0; axis < 3; axis++) {
    let lo = Infinity;
    let hi = -Infinity;
    for (let i = 0; i < n; i++) {
      const d = m.world[i * 3]! * v[0]![axis]! + m.world[i * 3 + 1]! * v[1]![axis]! + m.world[i * 3 + 2]! * v[2]![axis]!;
      if (d < lo) lo = d;
      if (d > hi) hi = d;
    }
    best = Math.min(best, hi - lo);
  }
  return best;
}

export const triangleCount = (list: readonly SolidMesh[]): number => list.reduce((n, m) => n + m.indices.length / 3, 0);

/** 🧮️ Volume of the union of closed solids: parity ray casts along x on a (y, z) grid per solid, interval union across solids. */
export function unionVolume(list: readonly SolidMesh[], cells = 220): number {
  const e = extentOf(list);
  if (e === null) return 0;
  const cell = Math.max(1e-5, Math.max(e.max[1] - e.min[1], e.max[2] - e.min[2]) / cells);
  const w = Math.max(1, Math.ceil((e.max[1] - e.min[1]) / cell));
  const h = Math.max(1, Math.ceil((e.max[2] - e.min[2]) / cell));
  const union: number[][] = new Array(w * h);
  for (const m of list) {
    checkCancelled();
    const hits = new Map<number, number[]>();
    const px = (i: number) => (m.world[i * 3 + 1]! - e.min[1]) / cell;
    const py = (i: number) => (m.world[i * 3 + 2]! - e.min[2]) / cell;
    for (let t = 0; t < m.indices.length; t += 3) {
      const [a, b, c] = [m.indices[t]!, m.indices[t + 1]!, m.indices[t + 2]!];
      const x0 = px(a), y0 = py(a), x1 = px(b), y1 = py(b), x2 = px(c), y2 = py(c);
      const den = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2);
      if (Math.abs(den) < 1e-12) continue;
      const i0 = Math.max(0, Math.floor(Math.min(x0, x1, x2)));
      const i1 = Math.min(w - 1, Math.ceil(Math.max(x0, x1, x2)));
      const j0 = Math.max(0, Math.floor(Math.min(y0, y1, y2)));
      const j1 = Math.min(h - 1, Math.ceil(Math.max(y0, y1, y2)));
      for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
        const qx = i + 0.5;
        const qy = j + 0.5;
        const l0 = ((y1 - y2) * (qx - x2) + (x2 - x1) * (qy - y2)) / den;
        const l1 = ((y2 - y0) * (qx - x2) + (x0 - x2) * (qy - y2)) / den;
        const l2 = 1 - l0 - l1;
        if (l0 < 0 || l1 < 0 || l2 < 0) continue;
        const x = l0 * m.world[a * 3]! + l1 * m.world[b * 3]! + l2 * m.world[c * 3]!;
        const key = j * w + i;
        const arr = hits.get(key);
        if (arr === undefined) hits.set(key, [x]);
        else arr.push(x);
      }
    }
    for (const [key, xs] of hits) {
      xs.sort((p, q) => p - q);
      for (let k = 0; k + 1 < xs.length; k += 2) (union[key] ??= []).push(xs[k]!, xs[k + 1]!);
    }
  }
  let total = 0;
  for (let key = 0; key < union.length; key++) {
    const iv = union[key];
    if (iv === undefined) continue;
    const pairs: [number, number][] = [];
    for (let k = 0; k < iv.length; k += 2) pairs.push([iv[k]!, iv[k + 1]!]);
    pairs.sort((p, q) => p[0] - q[0]);
    let lo = pairs[0]![0];
    let hi = pairs[0]![1];
    for (const [s, t] of pairs) {
      if (s > hi) {
        total += hi - lo;
        lo = s;
        hi = t;
      } else if (t > hi) hi = t;
    }
    total += hi - lo;
  }
  return total * cell * cell;
}

/** 📏️ Distance (in metres, chamfer approximation) of every cell to the nearest set cell of `mask`. */
export function distanceField(mask: Mask): Float32Array {
  const { width: w, height: h, cells } = mask;
  const d = new Float32Array(w * h);
  for (let i = 0; i < d.length; i++) d[i] = cells[i] ? 0 : 1e9;
  const s = Math.SQRT2;
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
    const i = y * w + x;
    let v = d[i]!;
    if (x > 0) v = Math.min(v, d[i - 1]! + 1);
    if (y > 0) {
      v = Math.min(v, d[i - w]! + 1);
      if (x > 0) v = Math.min(v, d[i - w - 1]! + s);
      if (x < w - 1) v = Math.min(v, d[i - w + 1]! + s);
    }
    d[i] = v;
  }
  for (let y = h - 1; y >= 0; y--) for (let x = w - 1; x >= 0; x--) {
    const i = y * w + x;
    let v = d[i]!;
    if (x < w - 1) v = Math.min(v, d[i + 1]! + 1);
    if (y < h - 1) {
      v = Math.min(v, d[i + w]! + 1);
      if (x < w - 1) v = Math.min(v, d[i + w + 1]! + s);
      if (x > 0) v = Math.min(v, d[i + w - 1]! + s);
    }
    d[i] = v;
  }
  for (let i = 0; i < d.length; i++) d[i] = d[i]! * mask.cell;
  return d;
}

/** 📐️ Symmetric Hausdorff distance (metres) between two masks of one grid, minus one cell of raster quantisation. */
export function hausdorff(a: Mask, b: Mask): number {
  const run = (from: Mask, to: Mask): number => {
    if (!to.cells.some((c) => c) || !from.cells.some((c) => c)) return Infinity;
    const field = distanceField(to);
    let worst = 0;
    for (let i = 0; i < from.cells.length; i++) if (from.cells[i]! && field[i]! > worst) worst = field[i]!;
    return worst;
  };
  return Math.max(0, Math.max(run(a, b), run(b, a)) - a.cell);
}

const AUTHORED_ROLES = new Set(["body", "detail", "glazing"]);

export const gridFor = (box: Box, longSide: number, cells = 640): { box: Box; cell: number } => {
  const cell = Math.max(1e-4, longSide / cells);
  return { box: { minX: box.minX - 2 * cell, minY: box.minY - 2 * cell, maxX: box.maxX + 2 * cell, maxY: box.maxY + 2 * cell }, cell };
};

export function projectedPoints(view: View, list: readonly SolidMesh[]): P2[] {
  const pts: P2[] = [];
  for (const m of list) for (let i = 0; i < m.world.length; i += 3) pts.push(projectWorld(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]));
  return pts;
}

export function meshMask(view: View, list: readonly SolidMesh[], box: Box, cell: number): Mask {
  const mask = new Mask(box, cell);
  for (const m of list) {
    const xy = new Float64Array((m.world.length / 3) * 2);
    for (let i = 0; i < m.world.length / 3; i++) {
      const p = projectWorld(view, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!]);
      xy[i * 2] = p.x;
      xy[i * 2 + 1] = p.y;
    }
    for (let t = 0; t < m.indices.length; t += 3) {
      const tri: Ring = [0, 1, 2].map((k) => ({ x: xy[m.indices[t + k]! * 2]!, y: xy[m.indices[t + k]! * 2 + 1]! }));
      mask.fill([tri]);
    }
  }
  return mask;
}

export type ViewMasks = { truth: Mask; volume: Mask; box: Box; cell: number };

/** 🖼️ The drawing and the projected body of one view on a shared grid (rest pose). */
export function viewMasks(view: View, state: RestState, list: readonly SolidMesh[]): ViewMasks {
  const shapes = state[view].filter((s) => s.closed && AUTHORED_ROLES.has(s.shape.role) && s.rings.length > 0);
  const pts = [...shapes.flatMap((s) => s.rings.flat()), ...projectedPoints(view, list)];
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const p of pts) {
    b.minX = Math.min(b.minX, p.x);
    b.maxX = Math.max(b.maxX, p.x);
    b.minY = Math.min(b.minY, p.y);
    b.maxY = Math.max(b.maxY, p.y);
  }
  if (!Number.isFinite(b.minX)) {
    const box = { minX: 0, minY: 0, maxX: 1, maxY: 1 };
    return { truth: new Mask(box, 1), volume: new Mask(box, 1), box, cell: 1 };
  }
  const g = gridFor(b, Math.max(b.maxX - b.minX, b.maxY - b.minY));
  const truth = new Mask(g.box, g.cell);
  for (const s of shapes) truth.fill(s.rings);
  return { truth, volume: meshMask(view, list, g.box, g.cell), box: g.box, cell: g.cell };
}

export const iouOf = (a: Mask, b: Mask): number => {
  let both = 0;
  let either = 0;
  for (let i = 0; i < a.cells.length; i++) {
    both += a.cells[i]! & b.cells[i]!;
    either += a.cells[i]! | b.cells[i]!;
  }
  return either === 0 ? 1 : both / either;
};

const maxDim = (asset: Json): number => Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);

export type ScaleMeshes = { body: SolidMesh[]; thermal: SolidMesh[]; structure: SolidMesh[]; solids: Record<ListName, Json[]>; solidMeshes: Record<ListName, (SolidMesh | undefined)[]>; unresolved: string[] };

export function scaleMeshes(asset: Json, scale: number, volume: Json): ScaleMeshes {
  const unresolved: string[] = [];
  const solids = { body: resolveList(volume, "body"), thermal: resolveList(volume, "thermal", unresolved), structure: resolveList(volume, "structure", unresolved) };
  const build = (s: Json[]): SolidMesh[] => {
    try {
      return meshesAt(asset, scale, s);
    } catch (error) {
      if (error instanceof CancelledError) throw error;
      return [];
    }
  };
  const [body, thermal, structure] = [build(solids.body), build(solids.thermal), build(solids.structure)];
  return { body, thermal, structure, solids, solidMeshes: { body, thermal, structure }, unresolved };
}

export type CheckOptions = { scale?: number; rules?: readonly string[]; skipTiled?: boolean };

const R = {
  availability: "volumes-availability",
  silhouette: "volumes-silhouette",
  views: "volumes-views-disagree",
  bounds: "volumes-bounds",
  feature: "volumes-feature",
  monotonic: "volumes-monotonic",
  cross: "volumes-cross-scale",
  thermal: "volumes-thermal-coverage",
  structure: "volumes-structure-coverage",
  bones: "volumes-bones",
  solids: "volumes-solids",
} as const;
export const RULES: readonly string[] = [...Object.values(R), "volumes-slender", "volumes-copied"];

const decisionsSay = (volume: Json, pattern: RegExp): boolean => (volume.decisions ?? []).some((d: Json) => pattern.test(typeof d === "string" ? d : `${d?.en ?? ""} ${d?.de ?? ""}`));

export function checkAsset(asset: Json, source: Json, options: CheckOptions = {}, progress: (message: string) => void = () => {}): Measurement[] {
  const out: Measurement[] = [];
  const want = (rule: string) => options.rules === undefined || options.rules.includes(rule);
  const volumes: Json = source.volumes ?? {};
  const authored = SCALES.filter((s) => isAuthored(volumes[String(s)]));
  const target = options.scale === undefined ? SCALES : SCALES.filter((s) => s === options.scale);
  const cache = new Map<number, ScaleMeshes>();
  const at = (s: number): ScaleMeshes => {
    let c = cache.get(s);
    if (c === undefined) {
      c = scaleMeshes(asset, s, volumes[String(s)]);
      cache.set(s, c);
    }
    return c;
  };
  const rest = (s: number): PoseState => poseState(asset, s);
  const dims: [string, number][] = [["widthM", asset.dimensions.widthM], ["heightM", asset.dimensions.heightM], ["depthM", asset.dimensions.depthM]];

  for (const scale of target) {
    checkCancelled();
    const volume = volumes[String(scale)];
    const rep = asset.representations?.[String(scale)];
    const feature = featureM(scale, asset);
    if (want(R.availability)) {
      const expected = rep?.status === "authored" ? "authored" : "unavailable";
      out.push(measure(R.availability, scale, undefined, "status-mirrors-representations", volume?.status === expected && volume?.denominator === scale ? 1 : 0, 1, "min", `volumes.status=${volume?.status ?? "missing"} representations.status=${rep?.status ?? "missing"}`));
    }
    if (!isAuthored(volume)) continue;
    progress(`1:${scale}`);
    const sm = at(scale);
    const nonEmpty = LISTS.map((l) => ({ l, n: (volume[l] ?? []).length }));
    if (want(R.solids)) {
      for (const { l, n } of nonEmpty) out.push(measure(R.solids, scale, l, "solids", n, 1, "min"));
      out.push(measure(R.solids, scale, undefined, "decisions", (volume.decisions ?? []).length, 1, "min"));
      out.push(measure(R.solids, scale, undefined, "unresolved-of", sm.unresolved.length, 0, "max", sm.unresolved.slice(0, 5).join("; ")));
      const frame = poseState(asset, scale).frames;
      const materials = new Set<string>((asset.materials ?? []).map((m: Json) => m.id));
      for (const l of LISTS) {
        const ids = new Set<string>();
        let dup = 0;
        let bad = 0;
        const notes: string[] = [];
        for (const s of volume[l] ?? []) {
          if (ids.has(s.id)) dup++;
          ids.add(s.id);
          const source = s.of === undefined ? s : sm.solids.body.find((b) => b.id === s.of);
          if (source === undefined) continue;
          if (frame[source.bone] === undefined || (materials.size > 0 && !materials.has(source.material))) {
            bad++;
            notes.push(`${s.id}: bone/material`);
          }
          try {
            const g = tessellateSolid({ ...source, node: source.bone } as never, qualityOfAsset(asset, scale));
            if (g.indices.length === 0 || g.positions.some((p) => !Number.isFinite(p))) throw new Error("empty or non-finite mesh");
          } catch (error) {
            bad++;
            notes.push(`${s.id}: ${(error as Error).message}`);
          }
        }
        out.push(measure(R.solids, scale, l, "duplicate-ids", dup, 0, "max"));
        out.push(measure(R.solids, scale, l, "invalid-solids", bad, 0, "max", notes.slice(0, 4).join("; ")));
      }
    }
    if (want(R.feature)) {
      for (const l of LISTS) {
        const list = sm.solids[l];
        let worst = Infinity;
        let who = "";
        let below = 0;
        list.forEach((s) => {
          if (THIN_SHEET_ROLES.has(s.role)) return;
          const d = minExtent(asset, scale, s);
          if (d < feature) below++;
          if (d < worst) {
            worst = d;
            who = s.id;
          }
        });
        if (list.length > 0) {
          out.push(measure(R.feature, scale, l, "smallestFeatureM", worst, feature, "min", `smallest: ${who}`));
          out.push(measure(R.feature, scale, l, "solidsBelowFeature", below, 0, "max"));
        }
      }
    }
    if (want(R.bones)) {
      const have = new Set(sm.solids.body.map((s) => s.bone));
      const missing = [...shapeBones(asset, scale)].filter((b) => !have.has(b));
      out.push(measure(R.bones, scale, "body", "bonesWithoutSolid", missing.length, 0, "max", missing.slice(0, 8).join(", ")));
    }
    if (want(R.thermal)) {
      const regions: Json[] = asset.thermal?.regions ?? [];
      const ids = new Set<string>(regions.map((r) => r.id));
      const solids = sm.solids.thermal;
      const byRegion = new Map<string, { solid: Json; mesh: SolidMesh }[]>();
      solids.forEach((s, i) => {
        const m = sm.thermal[i];
        if (m !== undefined) (byRegion.get(s.region) ?? byRegion.set(s.region, []).get(s.region)!).push({ solid: s, mesh: m });
      });
      const unnamed = solids.filter((s) => !ids.has(s.region)).map((s) => s.id);
      out.push(measure(R.thermal, scale, "thermal", "solidsWithUnknownRegion", unnamed.length, 0, "max", unnamed.slice(0, 5).join(", ")));
      const empty = regions.filter((r) => !byRegion.has(r.id)).map((r) => r.id);
      out.push(measure(R.thermal, scale, "thermal", "regionsWithoutSolid", empty.length, 0, "max", empty.join(", ")));
      const badLayer = solids.filter((s) => s.layer !== undefined && !(regions.find((r) => r.id === s.region)?.layers ?? []).some((l: Json) => l.id === s.layer)).map((s) => s.id);
      out.push(measure(R.thermal, scale, "thermal", "solidsWithUnknownLayer", badLayer.length, 0, "max", badLayer.slice(0, 5).join(", ")));
      if (scale === 50) {
        const lacking: string[] = [];
        for (const r of regions) if ((r.layers ?? []).length > 1 && byRegion.has(r.id)) for (const l of r.layers) if (!(byRegion.get(r.id) ?? []).some((x) => x.solid.layer === l.id) && !(l.thicknessM === undefined && decisionsSay(volume, /merged|zusammengefasst/i))) lacking.push(`${r.id}/${l.id}`);
        out.push(measure(R.thermal, scale, "thermal", "layersWithoutShell", lacking.length, 0, "max", lacking.slice(0, 6).join(", ")));
      }
      const named = (s: Json, id: string): boolean => s.id.includes(id) || (s.tags ?? []).includes(id);
      const noEmitter = (asset.thermal?.emitters ?? []).filter((em: Json) => !solids.some((s) => named(s, em.id))).map((em: Json) => em.id);
      out.push(measure(R.thermal, scale, "thermal", "emittersWithoutSolid", noEmitter.length, 0, "max", noEmitter.slice(0, 6).join(", ")));
      const e = extentOf(sm.body);
      const te = extentOf(sm.thermal);
      if (e !== null && te !== null) {
        const stray = [0, 1, 2].reduce((m, k) => Math.max(m, e.min[k]! - te.min[k]!, te.max[k]! - e.max[k]!), 0);
        out.push(measure(R.thermal, scale, "thermal", "outsideBodyBoxM", Math.max(0, stray), LIMITS.thermalBoxFeatures * feature, "max"));
      }
    }
    if (want(R.structure)) {
      const members = new Set<string>(sm.solids.structure.filter((s) => s.part === "member").map((s) => s.member));
      const need = memberBones(asset, scale);
      const missing = need.filter((b) => !members.has(b));
      out.push(measure(R.structure, scale, "structure", "membersWithoutSolid", missing.length, 0, "max", missing.slice(0, 8).join(", ")));
      const noSupport = (asset.structure?.supports ?? []).filter((id: string) => !sm.solids.structure.some((s) => s.part === "support" && (s.id.includes(id) || (s.tags ?? []).includes(id)))).map((id: string) => id);
      out.push(measure(R.structure, scale, "structure", "supportsWithoutSolid", noSupport.length, 0, "max", noSupport.join(", ")));
      const unknown = sm.solids.structure.filter((s) => !asset.rig.bones.some((b: Json) => b.id === s.member)).map((s) => s.id);
      out.push(measure(R.structure, scale, "structure", "solidsWithUnknownMember", unknown.length, 0, "max", unknown.slice(0, 5).join(", ")));
      const parts = new Set(["member", "joint", "support", "mass"]);
      out.push(measure(R.structure, scale, "structure", "solidsWithBadPart", sm.solids.structure.filter((s) => !parts.has(s.part)).length, 0, "max"));
      const e = extentOf(sm.body);
      const supports = sm.solids.structure.map((s, i) => ({ s, m: sm.structure[i] })).filter((x) => x.s.part === "support" && x.m !== undefined);
      for (const x of supports) {
        const off = e !== null && boundsOf(x.m!).min[1] > e.min[1] + feature && !anchorNear(asset, scale, x.s, x.m!, feature) ? Math.abs(boundsOf(x.m!).min[1] - e.min[1]) : 0;
        out.push(measure(R.structure, scale, "structure", `supportOffM:${x.s.id}`, off, feature, "max"));
      }
    }
  }

  if (want(R.monotonic)) {
    for (let i = 1; i < authored.length; i++) {
      const [a, b] = [authored[i - 1]!, authored[i]!];
      if (options.scale !== undefined && b !== options.scale) continue;
      const [ca, cb] = [at(a), at(b)];
      const minimal = (volumes[String(b)].decisions ?? []).some((d: Json) => String(d?.en ?? d).startsWith(LIMITS.minimalDecision));
      for (const l of LISTS) {
        const [na, nb] = [expandedCount(ca.solids[l]), expandedCount(cb.solids[l])];
        const [ta, tb] = [triangleCount(ca[l]), triangleCount(cb[l])];
        out.push(measure(R.monotonic, b, l, "solidCountDelta", nb - na, 0, "max", `${na} -> ${nb}`));
        out.push(measure(R.monotonic, b, l, "triangleCountDelta", tb - ta, 0, "max", `${ta} -> ${tb}`));
        out.push(measure(R.monotonic, b, l, "strictDrop", nb < na || tb < ta || minimal ? 1 : 0, 1, "min", minimal ? "declared Minimal:" : `needs a decision starting "${LIMITS.minimalDecision}" or a drop`));
      }
    }
  }
  if (want(R.cross)) {
    for (let i = 1; i < authored.length; i++) {
      const [a, b] = [authored[i - 1]!, authored[i]!];
      if (options.scale !== undefined && b !== options.scale) continue;
      const [ca, cb] = [at(a), at(b)];
      if (ca.body.length === 0 || cb.body.length === 0) {
        out.push(measure(R.cross, b, "body", "iou", 0, 0.85, "min", "empty body"));
        continue;
      }
      for (const view of VIEWS) {
        const pts = [...projectedPoints(view, ca.body), ...projectedPoints(view, cb.body)];
        const bb = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
        for (const p of pts) {
          bb.minX = Math.min(bb.minX, p.x);
          bb.maxX = Math.max(bb.maxX, p.x);
          bb.minY = Math.min(bb.minY, p.y);
          bb.maxY = Math.max(bb.maxY, p.y);
        }
        const g = gridFor(bb, Math.max(bb.maxX - bb.minX, bb.maxY - bb.minY));
        const ma = meshMask(view, ca.body, g.box, g.cell);
        const mb = meshMask(view, cb.body, g.box, g.cell);
        out.push(measure(R.cross, b, "body", `iou:${view}`, iouOf(ma, mb), LIMITS.crossIou, "min", `1:${a} vs 1:${b}`));
        out.push(measure(R.cross, b, "body", `hausdorffM:${view}`, hausdorff(ma, mb), LIMITS.crossHausdorffFeatures * featureM(b, asset), "max"));
      }
      const [va, vb] = [unionVolume(ca.body), unionVolume(cb.body)];
      out.push(measure(R.cross, b, "body", "unionVolumeChange", Math.abs(vb / (va || 1) - 1), LIMITS.crossVolume, "max", `${va.toFixed(5)} m3 -> ${vb.toFixed(5)} m3`));
    }
  }
  return out;
}

function anchorNear(asset: Json, scale: number, solid: Json, mesh: SolidMesh, tol: number): boolean {
  const anchor = (asset.anchors ?? []).find((a: Json) => a.bone === solid.member && (a.kind === "support" || a.support !== undefined) && Math.hypot(0, 0) === 0 && solid.id.includes(a.id)) ?? (asset.anchors ?? []).find((a: Json) => solid.id.includes(a.id));
  if (anchor === undefined) return false;
  const f = poseState(asset, scale).frames[anchor.bone];
  if (f === undefined) return false;
  const p = toWorld(f, [anchor.x, anchor.y, 0]);
  const b = boundsOf(mesh);
  return [0, 1].every((k) => p[k]! >= b.min[k]! - tol && p[k]! <= b.max[k]! + tol);
}

export type ScaleSummary = { rule: string; scale?: number; ok: boolean; items: Measurement[] };

/** 🧾️ One summary per (rule, scale). */
export function summarise(ms: readonly Measurement[]): ScaleSummary[] {
  const map = new Map<string, ScaleSummary>();
  for (const m of ms) {
    const key = `${m.rule}|${m.scale ?? ""}`;
    const s = map.get(key) ?? { rule: m.rule, ...(m.scale === undefined ? {} : { scale: m.scale }), ok: true, items: [] };
    s.items.push(m);
    s.ok = s.ok && m.ok;
    map.set(key, s);
  }
  return [...map.values()].sort((a, b) => RULES.indexOf(a.rule) - RULES.indexOf(b.rule) || (a.scale ?? 0) - (b.scale ?? 0));
}
