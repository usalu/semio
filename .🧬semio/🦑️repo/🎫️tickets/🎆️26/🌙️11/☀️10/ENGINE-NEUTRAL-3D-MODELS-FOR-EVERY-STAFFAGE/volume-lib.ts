/** 🧰️ Ticket-local helpers of the volume drafting aids: catalog access, rest frames, 2D geometry, an owned rasteriser and the silhouette and bounding-box analysis of a volume.
 *
 * Read-only use of the staffage modules (`evaluate`, `flattenGeometry`, `restFrames`, `CAMERAS`) and of the neutral kernel tessellator, so the conventions are not re-derived.
 * @see ./design.md */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { CAMERAS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { evaluate, flattenGeometry } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

export { flattenGeometry };

export const TICKET = dirname(fileURLToPath(import.meta.url));
export const STAFFAGE = resolve(TICKET, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const CATALOG = join(STAFFAGE, "🗂️catalog");
export const GENERATED = join(TICKET, "🗑️generated");

export type V3 = [number, number, number];
export type P2 = { x: number; y: number };
export type Ring = P2[];
export type View = "elevation" | "front" | "plan";
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export type Json = any;
export type Frame = { o: V3; u: V3; v: V3; w: V3 };

/** 🛑️ A cooperative cancellation flag shared by the long runs; the CLIs set it from SIGINT and SIGTERM. */
export const cancel = { requested: false };

export class CancelledError extends Error {
  constructor() {
    super("cancelled");
    this.name = "CancelledError";
  }
}

export function checkCancelled(): void {
  if (cancel.requested) throw new CancelledError();
}

/** ⏳️ Lets pending signals run between two units of work. */
export const yieldToLoop = (): Promise<void> => new Promise((done) => setImmediate(done));

export function installCancellation(): void {
  const stop = () => {
    if (cancel.requested) process.exit(130);
    cancel.requested = true;
    process.stderr.write("[cancel] stopping after the current asset (press Ctrl-C again to force)\n");
  };
  for (const signal of ["SIGINT", "SIGTERM", process.platform === "win32" ? "SIGBREAK" : "SIGHUP"] as const) process.on(signal, stop);
}

export type CatalogEntry = { category: string; folder: string; slug: string; path: string };

export function catalogEntries(): CatalogEntry[] {
  const out: CatalogEntry[] = [];
  for (const folder of readdirSync(CATALOG).sort()) {
    const dir = join(CATALOG, folder);
    let slugs: string[];
    try {
      slugs = readdirSync(dir).sort();
    } catch {
      continue;
    }
    for (const slug of slugs) {
      const path = join(dir, slug, "🔣️.json");
      if (existsSync(path)) out.push({ category: folder.replace(/^[^a-z]+/u, ""), folder, slug, path });
    }
  }
  return out;
}

export function resolveAsset(spec: string): CatalogEntry {
  if (existsSync(spec)) {
    const parts = resolve(spec).split(/[\\/]/u);
    return { category: "adhoc", folder: "adhoc", slug: parts[parts.length - 2] ?? "asset", path: resolve(spec) };
  }
  const hits = catalogEntries().filter((e) => e.slug === spec || `${e.category}/${e.slug}` === spec);
  if (hits.length !== 1) throw new Error(`asset "${spec}" matches ${hits.length} catalog entries`);
  return hits[0]!;
}

export function loadAsset(path: string): Json {
  const parsed = JSON.parse(readFileSync(path, "utf8"));
  return parsed.asset ?? parsed;
}

const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const norm = (a: V3): V3 => {
  const l = Math.hypot(...a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

/** 🦴️ The rest frames of every bone (root at the origin), from the rig module. */
export function frames(asset: Json): Record<string, Frame> {
  const solved = restFrames(asset.rig) as Record<string, { frame: Frame }>;
  const out: Record<string, Frame> = {};
  for (const [id, s] of Object.entries(solved)) out[id] = s.frame;
  return out;
}

export const toWorld = (f: Frame, p: V3): V3 => [f.o[0] + p[0] * f.u[0] + p[1] * f.v[0] + p[2] * f.w[0], f.o[1] + p[0] * f.u[1] + p[1] * f.v[1] + p[2] * f.w[1], f.o[2] + p[0] * f.u[2] + p[1] * f.v[2] + p[2] * f.w[2]];
export const toLocal = (f: Frame, p: V3): V3 => {
  const d = sub(p, f.o);
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
};
export const dirToLocal = (f: Frame, d: V3): V3 => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];

export type Evaluated = { shape: Json; rings: Ring[]; closed: boolean };
export type RestState = { elevation: Evaluated[]; front: Evaluated[]; plan: Evaluated[]; degraded?: string };

/** 🧍️ The rest pose of one representation scale: every authored shape of the three views placed by the staffage evaluation (solids, members and flat shapes alike), root at the origin. */
export function restState(asset: Json, denominator: number): RestState {
  const pose = { id: "draft-rest", title: { en: "Rest", de: "Ruhe" }, description: { en: "Rest", de: "Ruhe" }, bones: {}, ground: "authored", contacts: [], thermal: asset.poses[0].thermal, loads: [] };
  const patched = { ...asset, poses: [...asset.poses, pose] };
  const run = (a: Json): Json => evaluate(a, { kind: "pose", pose: "draft-rest" }, { kind: "scale", scale: { denominator } }) as Json;
  try {
    const state = run(patched);
    return { elevation: state.shapes.elevation, front: state.shapes.front, plan: state.shapes.plan };
  } catch (error) {
    const strip = (list: Json[]): Json[] => list.map(({ solid: _solid, ...rest }) => rest);
    const rep = patched.representations[String(denominator)];
    const flat = { ...patched, representations: { ...patched.representations, [String(denominator)]: { ...rep, geometry: { ...rep.geometry, elevation: strip(rep.geometry.elevation), front: strip(rep.geometry.front), plan: strip(rep.geometry.plan) } } } };
    const state = run(flat);
    return { elevation: state.shapes.elevation, front: state.shapes.front, plan: state.shapes.plan, degraded: (error as Error).message };
  }
}

export const area = (ring: Ring): number => {
  let a = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!;
    const q = ring[(i + 1) % ring.length]!;
    a += p.x * q.y - q.x * p.y;
  }
  return a / 2;
};

export function bboxOf(points: readonly P2[]): { minX: number; minY: number; maxX: number; maxY: number } {
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const p of points) {
    if (p.x < b.minX) b.minX = p.x;
    if (p.x > b.maxX) b.maxX = p.x;
    if (p.y < b.minY) b.minY = p.y;
    if (p.y > b.maxY) b.maxY = p.y;
  }
  return b;
}

export function pointInRing(p: P2, ring: Ring): boolean {
  let inside = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!;
    const b = ring[j]!;
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) inside = !inside;
  }
  return inside;
}

/** ✂️ Douglas-Peucker simplification of a closed ring within `tolerance` metres. */
export function simplifyRing(ring: Ring, tolerance: number): Ring {
  const clean: Ring = [];
  for (const p of ring) {
    const last = clean[clean.length - 1];
    if (last === undefined || Math.hypot(p.x - last.x, p.y - last.y) > 1e-9) clean.push(p);
  }
  while (clean.length > 1 && Math.hypot(clean[0]!.x - clean[clean.length - 1]!.x, clean[0]!.y - clean[clean.length - 1]!.y) <= 1e-9) clean.pop();
  if (clean.length <= 4) return clean;
  let far = 1;
  let best = -1;
  for (let i = 1; i < clean.length; i++) {
    const d = Math.hypot(clean[i]!.x - clean[0]!.x, clean[i]!.y - clean[0]!.y);
    if (d > best) {
      best = d;
      far = i;
    }
  }
  const keep = new Set<number>([0, far]);
  const run = (lo: number, hi: number, points: Ring, offset: (i: number) => number) => {
    if (hi <= lo + 1) return;
    const a = points[lo]!;
    const b = points[hi]!;
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const len = Math.hypot(dx, dy) || 1e-12;
    let worst = -1;
    let at = -1;
    for (let i = lo + 1; i < hi; i++) {
      const p = points[i]!;
      const d = Math.abs((p.x - a.x) * dy - (p.y - a.y) * dx) / len;
      if (d > worst) {
        worst = d;
        at = i;
      }
    }
    if (worst > tolerance) {
      keep.add(offset(at));
      run(lo, at, points, offset);
      run(at, hi, points, offset);
    }
  };
  const first = clean.slice(0, far + 1);
  const second = [...clean.slice(far), clean[0]!];
  run(0, first.length - 1, first, (i) => i);
  run(0, second.length - 1, second, (i) => far + i);
  const out = [...keep].sort((a, b) => a - b).map((i) => clean[i]!);
  return out.length >= 3 ? out : clean;
}

export const round5 = (x: number): number => Math.round(x * 1e5) / 1e5;

export type Box = { minX: number; minY: number; maxX: number; maxY: number };

/** 🖼️ A binary coverage grid sampled at cell centres. */
export class Mask {
  readonly width: number;
  readonly height: number;
  readonly cells: Uint8Array;
  constructor(readonly box: Box, readonly cell: number) {
    this.width = Math.max(1, Math.ceil((box.maxX - box.minX) / cell));
    this.height = Math.max(1, Math.ceil((box.maxY - box.minY) / cell));
    this.cells = new Uint8Array(this.width * this.height);
  }

  /** 🖌️ Fills the even-odd interior of `rings`; calls `visit(index)` for every cell hit. */
  fill(rings: readonly (readonly P2[])[], visit?: (index: number) => void): void {
    const { box, cell, width, height, cells } = this;
    let lo = Infinity;
    let hi = -Infinity;
    for (const ring of rings) for (const p of ring) {
      if (p.y < lo) lo = p.y;
      if (p.y > hi) hi = p.y;
    }
    if (!Number.isFinite(lo)) return;
    const j0 = Math.max(0, Math.ceil((lo - box.minY) / cell - 0.5));
    const j1 = Math.min(height - 1, Math.floor((hi - box.minY) / cell - 0.5));
    const xs: number[] = [];
    for (let j = j0; j <= j1; j++) {
      const y = box.minY + (j + 0.5) * cell;
      xs.length = 0;
      for (const ring of rings) {
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
          const a = ring[k]!;
          const b = ring[i]!;
          if (a.y <= y !== b.y <= y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
        }
      }
      xs.sort((a, b) => a - b);
      for (let n = 0; n + 1 < xs.length; n += 2) {
        const i0 = Math.max(0, Math.ceil((xs[n]! - box.minX) / cell - 0.5));
        const i1 = Math.min(width - 1, Math.ceil((xs[n + 1]! - box.minX) / cell - 0.5) - 1);
        for (let i = i0; i <= i1; i++) {
          const index = j * width + i;
          cells[index] = 1;
          visit?.(index);
        }
      }
    }
  }
}

export const QUALITY_FOR = (maxDimensionM: number) => ({ chordM: Math.min(0.02, Math.max(0.001, maxDimensionM / 800)), minSegments: 12, maxSegments: 64 });

export type SolidMesh = { id: string; role: string; world: Float64Array; indices: Uint32Array };

/** 🧊️ Every solid tessellated by the neutral kernel and carried to the world at rest. */
export function meshes(asset: Json, solids: readonly Json[], maxDimensionM: number): SolidMesh[] {
  const frame = frames(asset);
  const quality = QUALITY_FOR(maxDimensionM);
  const out: SolidMesh[] = [];
  for (const solid of solids) {
    checkCancelled();
    const f = frame[solid.bone];
    if (f === undefined) throw new Error(`solid "${solid.id}" is bound to unknown bone "${solid.bone}"`);
    const geometry = tessellateSolid({ ...solid, node: solid.bone } as never, quality);
    const world = new Float64Array(geometry.positions.length);
    for (let i = 0; i < geometry.positions.length; i += 3) {
      const p = toWorld(f, [geometry.positions[i]!, geometry.positions[i + 1]!, geometry.positions[i + 2]!]);
      world[i] = p[0];
      world[i + 1] = p[1];
      world[i + 2] = p[2];
    }
    out.push({ id: solid.id, role: solid.role, world, indices: Uint32Array.from(geometry.indices) });
  }
  return out;
}

const camera = (view: View) => CAMERAS[view];
export const projectWorld = (view: View, p: readonly [number, number, number]): P2 => {
  const c = camera(view);
  const pick = (axis: "x" | "y" | "z") => (axis === "x" ? p[0] : axis === "y" ? p[1] : p[2]);
  return { x: c.paperX.sign * pick(c.paperX.axis), y: c.paperUp.sign * pick(c.paperUp.axis) };
};

export type Extent = { min: V3; max: V3 };

export function extentOf(list: readonly SolidMesh[]): Extent | null {
  const min: V3 = [Infinity, Infinity, Infinity];
  const max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const m of list) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) {
    const v = m.world[i + k]!;
    if (v < min[k]!) min[k] = v;
    if (v > max[k]!) max[k] = v;
  }
  return Number.isFinite(min[0]) ? { min, max } : null;
}

const AUTHORED_ROLES = new Set(["body", "detail", "glazing"]);

export type ViewMetrics = {
  iou: number;
  recall: number;
  precision: number;
  cellM: number;
  worstSolids: { id: string; outsideM2: number; precision: number }[];
  worstShapes: { id: string; missingM2: number; coverage: number }[];
};

/** 📐️ Silhouette agreement of the projected solids against the authored closed shapes of one view (rest pose): union masks on a shared grid, IoU, recall of the authored area, precision of the volume, and the objects with the largest disagreement. */
export function compareView(view: View, authored: readonly Evaluated[], list: readonly SolidMesh[]): ViewMetrics {
  const shapes = authored.filter((s) => s.closed && AUTHORED_ROLES.has(s.shape.role) && s.rings.length > 0);
  const points: P2[] = shapes.flatMap((s) => s.rings.flat());
  const volumePoints: P2[] = [];
  const projected = list.map((m) => {
    const out = new Float64Array((m.world.length / 3) * 2);
    for (let i = 0; i < m.world.length / 3; i++) {
      const p = projectWorld(view, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!]);
      out[i * 2] = p.x;
      out[i * 2 + 1] = p.y;
      volumePoints.push(p);
    }
    return out;
  });
  const bounds = bboxOf([...points, ...volumePoints]);
  if (!Number.isFinite(bounds.minX)) return { iou: 0, recall: 0, precision: 0, cellM: 0, worstSolids: [], worstShapes: [] };
  const cell = Math.max(1e-4, Math.max(bounds.maxX - bounds.minX, bounds.maxY - bounds.minY) / 640);
  const box: Box = { minX: bounds.minX - 2 * cell, minY: bounds.minY - 2 * cell, maxX: bounds.maxX + 2 * cell, maxY: bounds.maxY + 2 * cell };
  const truth = new Mask(box, cell);
  const volume = new Mask(box, cell);
  for (const s of shapes) truth.fill(s.rings);
  list.forEach((m, n) => {
    const xy = projected[n]!;
    for (let t = 0; t < m.indices.length; t += 3) {
      const tri: Ring = [0, 1, 2].map((k) => ({ x: xy[m.indices[t + k]! * 2]!, y: xy[m.indices[t + k]! * 2 + 1]! }));
      volume.fill([tri]);
    }
  });
  let both = 0;
  let either = 0;
  let a = 0;
  let v = 0;
  for (let i = 0; i < truth.cells.length; i++) {
    const x = truth.cells[i]!;
    const y = volume.cells[i]!;
    a += x;
    v += y;
    both += x & y;
    either += x | y;
  }
  const stamp = new Int32Array(truth.cells.length).fill(-1);
  const scratch = new Mask(box, cell);
  const cellM2 = cell * cell;
  const worstShapes = shapes.map((s, n) => {
    let total = 0;
    let covered = 0;
    scratch.fill(s.rings, (index) => {
      if (stamp[index] === n) return;
      stamp[index] = n;
      total++;
      covered += volume.cells[index]!;
    });
    return { id: s.shape.id as string, missingM2: (total - covered) * cellM2, coverage: total === 0 ? 1 : covered / total };
  });
  stamp.fill(-1);
  const worstSolids = list.map((m, n) => {
    const xy = projected[n]!;
    let total = 0;
    let inside = 0;
    for (let t = 0; t < m.indices.length; t += 3) {
      const tri: Ring = [0, 1, 2].map((k) => ({ x: xy[m.indices[t + k]! * 2]!, y: xy[m.indices[t + k]! * 2 + 1]! }));
      scratch.fill([tri], (index) => {
        if (stamp[index] === n) return;
        stamp[index] = n;
        total++;
        inside += truth.cells[index]!;
      });
    }
    return { id: m.id, outsideM2: (total - inside) * cellM2, precision: total === 0 ? 1 : inside / total };
  });
  worstSolids.sort((p, q) => q.outsideM2 - p.outsideM2);
  worstShapes.sort((p, q) => q.missingM2 - p.missingM2);
  return {
    iou: either === 0 ? 1 : both / either,
    recall: a === 0 ? 1 : both / a,
    precision: v === 0 ? 1 : both / v,
    cellM: cell,
    worstSolids: worstSolids.slice(0, 5).map((s) => ({ ...s, outsideM2: Number(s.outsideM2.toFixed(5)), precision: Number(s.precision.toFixed(3)) })),
    worstShapes: worstShapes.slice(0, 5).map((s) => ({ ...s, missingM2: Number(s.missingM2.toFixed(5)), coverage: Number(s.coverage.toFixed(3)) })),
  };
}

export type Analysis = {
  bboxM: V3 | null;
  dimensionsM: V3;
  bboxRelativeError: V3;
  views: Record<View, ViewMetrics>;
};

/** 🔬️ Bounding box (x ahead = `widthM`, y up = `heightM`, z lateral = `depthM`) and the three silhouette comparisons of a volume at rest against the 1:`denominator` representation. */
export function analyse(asset: Json, volume: Json, denominator: number, state: RestState = restState(asset, denominator)): Analysis {
  const d = asset.dimensions;
  const dimensionsM: V3 = [d.widthM, d.heightM, d.depthM];
  const list = meshes(asset, volume.solids, Math.max(...dimensionsM));
  const extent = extentOf(list);
  const bboxM: V3 | null = extent === null ? null : [extent.max[0] - extent.min[0], extent.max[1] - extent.min[1], extent.max[2] - extent.min[2]];
  const views = {} as Record<View, ViewMetrics>;
  for (const view of VIEWS) {
    checkCancelled();
    views[view] = compareView(view, state[view], list);
  }
  return { bboxM, dimensionsM, bboxRelativeError: bboxM === null ? [1, 1, 1] : [0, 1, 2].map((k) => (bboxM[k]! - dimensionsM[k]!) / dimensionsM[k]!) as V3, views };
}

export const median = (values: readonly number[]): number => {
  if (values.length === 0) return NaN;
  const s = [...values].sort((a, b) => a - b);
  const mid = s.length >> 1;
  return s.length % 2 === 1 ? s[mid]! : (s[mid - 1]! + s[mid]!) / 2;
};
