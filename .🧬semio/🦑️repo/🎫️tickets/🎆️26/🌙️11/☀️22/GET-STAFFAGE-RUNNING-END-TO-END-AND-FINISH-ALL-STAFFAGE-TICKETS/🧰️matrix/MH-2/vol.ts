/** 🧱️ Front and plan solids made from the volume of the band (batch MH-2, the parts whose flat stride outline no fitted prism replaced within the silhouette guard): `bun vol.ts <id|file.json> --plan <plan.json> [--out <file> | --write] [--decided <plan-out.json>] [--report] [--budget <s>]`.
 * A plan entry is `{ scale, view: "front" | "plan", id, bone?, solids?: [volume solid id, a trailing * is a prefix], make | try: [make…] }`. The body solids named (default: the solid of the shape's id and those whose id starts with `<id>-`) are meshed in the default pose and read in the frame their bone has there (the body is rigid on its bone, so these are its rest coordinates) (u along the bone, v its normal, w lateral); the shape becomes one solid of that body:
 * - `sweep-u`: the (v, w) outline of the body swept along the bone over its u-range (true where the view looks along the bone: the plan of a hanging hand);
 * - `sweep-v`: the (u, w) outline swept along the normal over the v-range of the body, the range running linearly from the joint to the tip (`zTip`);
 * - `sweep-w`: the (u, v) outline swept laterally over the w-range, linear likewise;
 * - `revolve-w`, `revolve-v`, `revolve-m`: a body of revolution about the bone axis through the middle of the body whose radius at sixteen stations is half the lateral width, half the thickness or their mean.
 * - `ellipse`: for a limb of elliptic section, a body of revolution of the smaller half axis and beneath it a slab `<id>-core` of the outline along the larger one (two shapes on the bone: the matrix reports the bone in the group tier, the whole-figure extents stay hard).
 * An entry `{ scale, hand: "hand-l" }` rebuilds a hand in the three views from its volume: the palm as the prism of its outline across its thin direction, every finger segment as a body of revolution about its own finger bone (`putHand`).
 * With `make` the entry is built and nothing is measured (the reproducible, idempotent step: the result depends on the volume and the rig only, never on the shape it replaces). With `try` the makes are ranked by the local misfit of the part in the default pose (Hausdorff distance on a raster between the drawn part and its projected body), the best of every entry is applied, and while the silhouette guard of the scale (`measureAsset`, the gate of `🧊️catalog-silhouette`) fails or the matrix's own `geometryFindings` over every subject of the scale report a hard finding on the bone, the next makes are tried and kept when (guard failures, hard findings) gets smaller. `--decided` writes the plan with the chosen `make` per entry; `--report` only prints guard and gate of the document per authored scale.
 * @see ./fit2.ts — the ops that rebuild a twin from the elevation outline */
import { join } from "node:path";
import { writeFileSync } from "node:fs";
import { S, dot, readDoc, round, writeDoc, type V3 } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { measureAsset, worldSolids, project, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const engine = await import(join(T, "🧰️.ts"));

type P2 = readonly [number, number];
type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Entry = { scale: string; hand?: string; optional?: boolean; view: "front" | "plan"; id: string; bone?: string; solids?: string[]; make?: string; try?: string[]; remove?: boolean };
type Score = { guard: number; hard: number; text: string; keys: Map<string, { n: number; mm: number; subject: string }> };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const started = Date.now();
const budgetS = Number(flag("budget") ?? 3000);
const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"];
const log = (text: string): void => console.log(text);
const minOf = (values: readonly number[]): number => values.reduce((a, b) => (b < a ? b : a), Infinity);
const maxOf = (values: readonly number[]): number => values.reduce((a, b) => (b > a ? b : a), -Infinity);

/** 🧊️ The vertices and triangles of the named body solids of a scale at rest, in the frame of a bone. */
function bodyOf(asset: any, scale: string, bone: string, patterns: readonly string[]): { points: V3[]; triangles: [number, number, number][]; frame: Frame; length: number } {
  const frame = (restFrames(asset.rig) as Record<string, { frame: Frame }>)[bone]!.frame;
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
  const arrayOf = (p: { x: number; y: number; z: number }): V3 => [p.x, p.y, p.z];
  const posed = state.bones[bone].frame;
  const now: Frame = { o: arrayOf(posed.o), u: arrayOf(posed.u), v: arrayOf(posed.v), w: arrayOf(posed.w) };
  const worlds = (worldSolids(asset, state.bones as never, Number(scale)) as any[]).filter((world) => world.texture !== true && patterns.some((pattern) => (pattern.endsWith("*") ? world.id.startsWith(pattern.slice(0, -1)) : world.id === pattern)));
  if (worlds.length === 0) throw new Error(`1:${scale}: no body solid matches ${patterns.join(", ")}`);
  const points: V3[] = [];
  const triangles: [number, number, number][] = [];
  for (const world of worlds) {
    const base = points.length;
    for (let i = 0; i < world.positions.length; i += 3) {
      const d: V3 = [world.positions[i] - now.o[0], world.positions[i + 1] - now.o[1], world.positions[i + 2] - now.o[2]];
      points.push([dot(d, now.u), dot(d, now.v), dot(d, now.w)]);
    }
    for (let t = 0; t < world.indices.length; t += 3) triangles.push([base + world.indices[t], base + world.indices[t + 1], base + world.indices[t + 2]]);
  }
  if (points.length === 0 || points.some((p) => !Number.isFinite(p[0] + p[1] + p[2]))) throw new Error(`1:${scale}: the body of ${patterns.join(", ")} on ${bone} has ${points.length} vertices, some not finite`);
  return { points, triangles, frame, length: asset.rig.bones.find((candidate: any) => candidate.id === bone).lengthM };
}

/** 🏁️ A raster over a box. */
class Grid {
  readonly width: number;
  readonly height: number;
  readonly cells: Uint8Array;
  constructor(readonly minX: number, readonly minY: number, maxX: number, maxY: number, readonly cell: number) {
    this.width = Math.ceil((maxX - minX) / cell) + 4;
    this.height = Math.ceil((maxY - minY) / cell) + 4;
    this.minX -= 2 * cell;
    this.minY -= 2 * cell;
    this.cells = new Uint8Array(this.width * this.height);
  }
  triangle(a: P2, b: P2, c: P2): void {
    const area = (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]);
    if (Math.abs(area) < 1e-14) return;
    const i0 = Math.max(0, Math.floor((Math.min(a[0], b[0], c[0]) - this.minX) / this.cell));
    const i1 = Math.min(this.width - 1, Math.ceil((Math.max(a[0], b[0], c[0]) - this.minX) / this.cell));
    const j0 = Math.max(0, Math.floor((Math.min(a[1], b[1], c[1]) - this.minY) / this.cell));
    const j1 = Math.min(this.height - 1, Math.ceil((Math.max(a[1], b[1], c[1]) - this.minY) / this.cell));
    for (let j = j0; j <= j1; j++)
      for (let i = i0; i <= i1; i++) {
        const [x, y] = [this.minX + (i + 0.5) * this.cell, this.minY + (j + 0.5) * this.cell];
        const w0 = ((b[0] - x) * (c[1] - y) - (c[0] - x) * (b[1] - y)) / area;
        const w1 = ((c[0] - x) * (a[1] - y) - (a[0] - x) * (c[1] - y)) / area;
        if (w0 >= -1e-9 && w1 >= -1e-9 && w0 + w1 <= 1 + 1e-9) this.cells[j * this.width + i] = 1;
      }
  }
  ring(ring: readonly P2[]): void {
    for (let j = 0; j < this.height; j++) {
      const y = this.minY + (j + 0.5) * this.cell;
      const hits: number[] = [];
      for (let k = 0; k < ring.length; k++) {
        const [p, q] = [ring[k]!, ring[(k + 1) % ring.length]!];
        if (p[1] <= y === q[1] <= y) continue;
        hits.push(p[0] + ((q[0] - p[0]) * (y - p[1])) / (q[1] - p[1]));
      }
      hits.sort((x, z) => x - z);
      for (let k = 0; k + 1 < hits.length; k += 2) {
        const i0 = Math.max(0, Math.ceil((hits[k]! - this.minX) / this.cell - 0.5));
        const i1 = Math.min(this.width - 1, Math.floor((hits[k + 1]! - this.minX) / this.cell - 0.5));
        for (let i = i0; i <= i1; i++) this.cells[j * this.width + i] = 1;
      }
    }
  }
  /** 🧵️ The outline of the largest filled region, as the crack between filled and empty cells, simplified to a tolerance. */
  outline(tolerance: number): P2[] {
    const { width, height, cells } = this;
    const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= width || j >= height ? 0 : cells[j * width + i]!);
    const next = new Map<number, number[]>();
    const key = (i: number, j: number): number => j * (width + 1) + i;
    const edge = (i0: number, j0: number, i1: number, j1: number): void => {
      const list = next.get(key(i0, j0));
      if (list === undefined) next.set(key(i0, j0), [key(i1, j1)]);
      else list.push(key(i1, j1));
    };
    for (let j = 0; j < height; j++)
      for (let i = 0; i < width; i++) {
        if (!at(i, j)) continue;
        if (!at(i, j - 1)) edge(i, j, i + 1, j);
        if (!at(i + 1, j)) edge(i + 1, j, i + 1, j + 1);
        if (!at(i, j + 1)) edge(i + 1, j + 1, i, j + 1);
        if (!at(i - 1, j)) edge(i, j + 1, i, j);
      }
    let best: P2[] = [];
    let bestArea = 0;
    while (next.size > 0) {
      const start = next.keys().next().value as number;
      const loop: number[] = [];
      let current = start;
      for (let guard = 0; guard < 4 * (width + 1) * (height + 1); guard++) {
        loop.push(current);
        const list = next.get(current);
        if (list === undefined) break;
        const to = list.pop()!;
        if (list.length === 0) next.delete(current);
        current = to;
        if (current === start) break;
      }
      const ring = loop.map((k) => [this.minX + (k % (width + 1)) * this.cell, this.minY + Math.floor(k / (width + 1)) * this.cell] as const);
      let area = 0;
      for (let k = 0; k < ring.length; k++) area += ring[k]![0] * ring[(k + 1) % ring.length]![1] - ring[(k + 1) % ring.length]![0] * ring[k]![1];
      if (Math.abs(area) > bestArea) (bestArea = Math.abs(area)), (best = area < 0 ? [...ring].reverse() : ring);
    }
    return simplify(best, tolerance);
  }
}

/** ✂️ A closed ring simplified by Douglas and Peucker between its two most distant points. */
function simplify(ring: readonly P2[], tolerance: number): P2[] {
  if (ring.length < 8) return [...ring];
  let far = 0;
  for (let i = 1; i < ring.length; i++) if (Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i;
  const part = (points: readonly P2[]): P2[] => {
    if (points.length < 3) return [...points];
    const [a, b] = [points[0]!, points[points.length - 1]!];
    const length = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
    let worst = 0;
    let index = 0;
    for (let i = 1; i < points.length - 1; i++) {
      const d = Math.abs((b[0] - a[0]) * (a[1] - points[i]![1]) - (a[0] - points[i]![0]) * (b[1] - a[1])) / length;
      if (d > worst) (worst = d), (index = i);
    }
    if (worst <= tolerance) return [a, b];
    return [...part(points.slice(0, index + 1)).slice(0, -1), ...part(points.slice(index))];
  };
  return [...part(ring.slice(0, far + 1)).slice(0, -1), ...part([...ring.slice(far), ring[0]!]).slice(0, -1)];
}

/** 🔪️ The extent of a body across the bone at one station: the cut of every triangle with the plane u = const. */
function sectionAt(points: readonly V3[], triangles: readonly (readonly [number, number, number])[], u: number): { v: [number, number]; w: [number, number] } | undefined {
  let [v0, v1, w0, w1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const triangle of triangles)
    for (let k = 0; k < 3; k++) {
      const [p, q] = [points[triangle[k]!]!, points[triangle[(k + 1) % 3]!]!];
      if (p[0] <= u === q[0] <= u) continue;
      const t = (u - p[0]) / (q[0] - p[0]);
      const [v, w] = [p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t];
      if (v < v0) v0 = v;
      if (v > v1) v1 = v;
      if (w < w0) w0 = w;
      if (w > w1) w1 = w;
    }
  return v0 > v1 ? undefined : { v: [v0, v1], w: [w0, w1] };
}

/** 📏️ The stations of a body along its bone (`n` evenly spaced and one just inside each end) with their exact extents, and the straight lines through the lower and the upper bound of one coordinate at the joint (u = 0) and at the tip (u = length). */
function spanAlong(points: readonly V3[], triangles: readonly (readonly [number, number, number])[], axis: 1 | 2, length: number, n = 20): { z0: number; z1: number; tip: [number, number]; stations: { u: number; low: number; high: number }[] } {
  const us = points.map((p) => p[0]);
  const [u0, u1] = [minOf(us), maxOf(us)];
  const stations: { u: number; low: number; high: number }[] = [];
  for (const f of [0.012, ...Array.from({ length: n }, (_, k) => (k + 0.5) / n), 0.988]) {
    const u = u0 + (u1 - u0) * f;
    const cut = sectionAt(points, triangles, u);
    if (cut !== undefined) stations.push({ u, low: (axis === 1 ? cut.v : cut.w)[0], high: (axis === 1 ? cut.v : cut.w)[1] });
  }
  if (stations.length < 3) throw new Error("the body has no section along its bone");
  const core = stations.length > 8 ? stations.slice(2, -2) : stations;
  const line = (pick: (s: { u: number; low: number; high: number }) => number): [number, number] => {
    const mu = core.reduce((sum, s) => sum + s.u, 0) / core.length;
    const mv = core.reduce((sum, s) => sum + pick(s), 0) / core.length;
    const sxx = core.reduce((sum, s) => sum + (s.u - mu) ** 2, 0);
    const slope = sxx < 1e-12 ? 0 : core.reduce((sum, s) => sum + (s.u - mu) * (pick(s) - mv), 0) / sxx;
    return [mv - slope * mu, slope];
  };
  const [low, high] = [line((s) => s.low), line((s) => s.high)];
  const reach = length > 0 ? length : 1;
  return { z0: low[0], z1: high[0], tip: [low[0] + low[1] * reach, high[0] + high[1] * reach], stations };
}

/** 🏗️ The geometry of one make for a body in a bone frame. */
function build(body: ReturnType<typeof bodyOf>, bone: string, make: string, feature: number): { points: number[][]; solid: Record<string, unknown> } {
  const { points, triangles, frame, length } = body;
  const cell = Math.min(0.002, Math.max(0.0005, feature / 8));
  const tolerance = Math.max(1.3 * cell, feature / 10);
  const outline = (a: 0 | 1 | 2, b: 0 | 1 | 2): number[][] => {
    const xs = points.map((p) => p[a]);
    const ys = points.map((p) => p[b]);
    const grid = new Grid(minOf(xs), minOf(ys), maxOf(xs), maxOf(ys), cell);
    for (const [i, j, k] of triangles) grid.triangle([points[i]![a], points[i]![b]], [points[j]![a], points[j]![b]], [points[k]![a], points[k]![b]]);
    return grid.outline(tolerance).map(([x, y]) => [round(x, 4), round(y, 4)]);
  };
  const gap = (low: number, high: number): [number, number] => (high - low < 0.004 ? [(low + high) / 2 - 0.002, (low + high) / 2 + 0.002] : [low, high]);
  if (make === "sweep-u") {
    const us = points.map((p) => p[0]);
    return { points: outline(1, 2), solid: { kind: "extruded", sweep: "u", z0: round(minOf(us)), z1: round(maxOf(us)) } };
  }
  if (make === "sweep-v" || make === "sweep-w") {
    const lateral = make === "sweep-w";
    const span = spanAlong(points, triangles, lateral ? 2 : 1, length);
    const base = lateral ? frame.o[2] : 0;
    const [z0, z1] = gap(span.z0, span.z1);
    const tip = gap(span.tip[0], span.tip[1]);
    return { points: lateral ? outline(0, 1) : outline(0, 2), solid: { kind: "extruded", sweep: lateral ? "w" : "v", z0: round(z0 + base), z1: round(z1 + base), zTip: [round(tip[0] + base), round(tip[1] + base)] } };
  }
  if (make.startsWith("revolve-")) {
    const [sv, sw] = [spanAlong(points, triangles, 1, length), spanAlong(points, triangles, 2, length)];
    const mid = (stations: { low: number; high: number }[]): number => stations.reduce((sum, s) => sum + (s.low + s.high) / 2, 0) / stations.length;
    const [av, aw] = [mid(sv.stations), mid(sw.stations)];
    const us = points.map((p) => p[0]);
    const [u0, u1] = [minOf(us), maxOf(us)];
    const radius = (k: number): number => {
      const [hv, hw] = [Math.max(sv.stations[k]!.high - av, av - sv.stations[k]!.low), Math.max(sw.stations[k]!.high - aw, aw - sw.stations[k]!.low)];
      return Math.max(0.001, make === "revolve-w" ? hw : make === "revolve-v" ? hv : make === "revolve-min" ? Math.min(hv, hw) : (hv + hw) / 2);
    };
    const last = sv.stations.length - 1;
    const radial: P2[] = [[u0, radius(0) * 0.55], ...sv.stations.map((s, k) => [s.u, radius(k)] as const), [u1, radius(last) * 0.55]];
    const level = Math.abs(frame.u[1]) < 0.05;
    const t = level ? 0 : -(av * frame.v[1] + aw * frame.w[1]) / frame.u[1];
    const offset: V3 = [av * frame.v[0] + aw * frame.w[0] + t * frame.u[0], 0, av * frame.v[2] + aw * frame.w[2] + t * frame.u[2]];
    if (Math.abs(dot(offset, frame.v) - av) > 0.0015) throw new Error(`bone ${bone} lies level: the axis of a body of revolution cannot be offset along its normal by ${(av * 1000).toFixed(1)} mm`);
    const held = dot(offset, frame.v);
    const skew = round(offset[2]) - aw;
    return { points: [...radial.map(([u, r]) => [round(u, 4), round(held + Math.sqrt(Math.max(0.000001, r * r - skew * skew)), 4)]), [round(u1, 4), round(held, 4)], [round(u0, 4), round(held, 4)]], solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: round(offset[0]), axisZ: round(offset[2]), mirror: true } };
  }
  if (make === "slab-v" || make === "slab-w") {
    const lateral = make === "slab-w";
    const span = spanAlong(points, triangles, lateral ? 2 : 1, length);
    const base = lateral ? frame.o[2] : 0;
    const [m0, mt] = [(span.z0 + span.z1) / 2, (span.tip[0] + span.tip[1]) / 2];
    const [h0, ht] = [Math.max(0.003, (span.z1 - span.z0) / 4), Math.max(0.003, (span.tip[1] - span.tip[0]) / 4)];
    return { points: lateral ? outline(0, 1) : outline(0, 2), solid: { kind: "extruded", sweep: lateral ? "w" : "v", z0: round(m0 - h0 + base), z1: round(m0 + h0 + base), zTip: [round(mt - ht + base), round(mt + ht + base)] } };
  }
  throw new Error(`unknown make "${make}"`);
}

const shapeOf = (asset: any, entry: Entry): any => {
  const shape = asset.representations[entry.scale].geometry[entry.view].find((candidate: any) => candidate.id === entry.id && (entry.bone === undefined || candidate.bone === entry.bone));
  if (shape === undefined) throw new Error(`1:${entry.scale} ${entry.view} has no shape "${entry.id}"`);
  return shape;
};
const patternsOf = (entry: Entry): string[] => entry.solids ?? [entry.id, `${entry.id}-*`];
const built = new Map<string, { points: number[][]; solid: Record<string, unknown> }>();

const PAINT_SKIP = new Set([...GEOMETRY_KEYS, "kind", "id", "bone"]);
const cached = (asset: any, scale: string, bone: string, patterns: string[], make: string): { points: number[][]; solid: Record<string, unknown> } => {
  const key = `${scale}/${bone}/${patterns.join("+")}/${make}`;
  if (!built.has(key)) built.set(key, build(bodyOf(asset, scale, bone, patterns), bone, make, featureOf(Number(scale))));
  return built.get(key)!;
};
const setGeometry = (shape: any, geometry: { points: number[][]; solid: Record<string, unknown> }): void => {
  for (const name of GEOMETRY_KEYS) delete shape[name];
  Object.assign(shape, { kind: "poly", points: structuredClone(geometry.points), closed: true, solid: structuredClone(geometry.solid) });
};

/** 🖊️ Writes one make of an entry into an asset; says whether the document changed. `ellipse` is a body of revolution of the smaller half axis with a slab `<id>-core` of the outline along the larger one beneath it (a limb of elliptic section); every other make but `keep` removes such a core. */
function put(asset: any, entry: Entry, make: string): boolean {
  if (make === "keep") return false;
  const list = asset.representations[entry.scale].geometry[entry.view] as any[];
  const before = JSON.stringify(list);
  if (make === "remove") {
    const index = list.findIndex((candidate) => candidate.id === entry.id && (entry.bone === undefined || candidate.bone === entry.bone));
    if (index >= 0) list.splice(index, 1);
    return index >= 0;
  }
  const shape = shapeOf(asset, entry);
  const patterns = patternsOf(entry);
  const coreAt = list.findIndex((candidate) => candidate.id === `${entry.id}-core` && candidate.bone === shape.bone);
  if (make === "ellipse") {
    const body = bodyOf(asset, entry.scale, shape.bone, patterns);
    const [sv, sw] = [spanAlong(body.points, body.triangles, 1, body.length), spanAlong(body.points, body.triangles, 2, body.length)];
    const wide = (span: typeof sv): number => span.stations.reduce((sum, s) => sum + (s.high - s.low), 0);
    setGeometry(shape, cached(asset, entry.scale, shape.bone, patterns, "revolve-min"));
    const core = coreAt >= 0 ? list[coreAt] : { ...Object.fromEntries(Object.entries(shape).filter(([name]) => !PAINT_SKIP.has(name))), id: `${entry.id}-core`, bone: shape.bone, ...(typeof shape.z === "number" ? { z: round(shape.z - 0.01, 4) } : {}) };
    setGeometry(core, cached(asset, entry.scale, shape.bone, patterns, wide(sv) > wide(sw) ? "slab-w" : "slab-v"));
    if (coreAt < 0) list.splice(list.indexOf(shape), 0, core);
  } else {
    if (coreAt >= 0) list.splice(coreAt, 1);
    setGeometry(shape, cached(asset, entry.scale, shape.bone, patterns, make));
  }
  return JSON.stringify(list) !== before;
}

/** ✋️ A hand as the solids of its volume in the three views: the palm (the solids `<hand>`, `<hand>-thenar`, `<hand>-hypothenar`) as the prism of its outline across its thin direction on the hand bone, and every finger segment `<hand>-<finger>-<phalanx>` as a body of revolution about its own bone; the shapes of the three views are the same bodies, so the views agree in every pose. */
function putHand(asset: any, scale: string, hand: string): boolean {
  const geometry = asset.representations[scale].geometry;
  const before = JSON.stringify(geometry);
  const bodies = (asset.volumes[scale].body as any[]).filter((solid) => solid.id === hand || solid.id.startsWith(`${hand}-`));
  const palmIds = bodies.filter((solid) => solid.bone === hand).map((solid) => solid.id as string);
  const fingers = bodies.filter((solid) => solid.bone !== hand);
  const palmBody = bodyOf(asset, scale, hand, palmIds);
  const [sv, sw] = [spanAlong(palmBody.points, palmBody.triangles, 1, palmBody.length), spanAlong(palmBody.points, palmBody.triangles, 2, palmBody.length)];
  const wide = (span: typeof sv): number => span.stations.reduce((sum, s) => sum + (s.high - s.low), 0);
  const palm = cached(asset, scale, hand, palmIds, wide(sv) < wide(sw) ? "sweep-v" : "sweep-w");
  for (const view of ["elevation", "front", "plan"]) {
    const list = geometry[view] as any[];
    let shape = list.find((candidate) => candidate.id === hand && candidate.bone === hand);
    if (shape === undefined) {
      const like = (geometry.elevation as any[]).find((candidate) => candidate.id === hand && candidate.bone === hand);
      if (like === undefined) throw new Error(`1:${scale}: no shape "${hand}" to take the paint from`);
      shape = { ...Object.fromEntries(Object.entries(like).filter(([name]) => !PAINT_SKIP.has(name))), id: hand, bone: hand };
      list.push(shape);
    }
    setGeometry(shape, palm);
    let after = list.indexOf(shape);
    for (const solid of fingers) {
      let finger = list.find((candidate) => candidate.id === solid.id && candidate.bone === solid.bone);
      if (finger === undefined) {
        finger = { ...Object.fromEntries(Object.entries(shape).filter(([name]) => !PAINT_SKIP.has(name))), id: solid.id, bone: solid.bone };
        list.splice(++after, 0, finger);
      }
      setGeometry(finger, cached(asset, scale, solid.bone, [solid.id], "revolve-m"));
    }
  }
  return JSON.stringify(geometry) !== before;
}

/** 🎯️ The Hausdorff distance in metres between the drawn shape of an entry and its projected body in the default pose. */
function misfit(asset: any, entry: Entry): number {
  const scale = Number(entry.scale);
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const shape = shapeOf(asset, entry);
  const drawn = (state.shapes[entry.view] as any[]).filter((candidate) => candidate.shape.id === entry.id && candidate.shape.bone === shape.bone).flatMap((candidate) => candidate.rings as { x: number; y: number }[][]);
  const patterns = patternsOf(entry);
  const worlds = (worldSolids(asset, state.bones as never, scale) as any[]).filter((world) => world.texture !== true && patterns.some((pattern) => (pattern.endsWith("*") ? world.id.startsWith(pattern.slice(0, -1)) : world.id === pattern)));
  const flat: P2[][] = worlds.map((world) => {
    const out: P2[] = [];
    for (let i = 0; i < world.positions.length; i += 3) {
      const p = project(entry.view, world.positions[i], world.positions[i + 1], world.positions[i + 2]);
      out.push([p.x, p.y]);
    }
    return out;
  });
  const xs = [...drawn.flat().map((p) => p.x), ...flat.flat().map((p) => p[0])];
  const ys = [...drawn.flat().map((p) => p.y), ...flat.flat().map((p) => p[1])];
  const cell = Math.max(0.001, featureOf(scale) / 6);
  const [a, b] = [new Grid(minOf(xs), minOf(ys), maxOf(xs), maxOf(ys), cell), new Grid(minOf(xs), minOf(ys), maxOf(xs), maxOf(ys), cell)];
  for (const ring of drawn) a.ring(ring.map((p) => [p.x, p.y] as const));
  worlds.forEach((world, n) => {
    for (let t = 0; t < world.indices.length; t += 3) b.triangle(flat[n]![world.indices[t]]!, flat[n]![world.indices[t + 1]]!, flat[n]![world.indices[t + 2]]!);
  });
  const directed = (from: Grid, to: Grid): number => {
    const targets: number[] = [];
    for (let k = 0; k < to.cells.length; k++) if (to.cells[k]) targets.push(k);
    if (targets.length === 0) return Infinity;
    const edge = targets.filter((k) => !to.cells[k - 1] || !to.cells[k + 1] || !to.cells[k - to.width] || !to.cells[k + to.width]);
    let worst = 0;
    for (let k = 0; k < from.cells.length; k++) {
      if (!from.cells[k] || to.cells[k]) continue;
      const [i, j] = [k % from.width, Math.floor(k / from.width)];
      let best = Infinity;
      for (const e of edge) {
        const d = (i - (e % to.width)) ** 2 + (j - Math.floor(e / to.width)) ** 2;
        if (d < best) best = d;
      }
      if (best > worst) worst = best;
    }
    return Math.sqrt(worst) * cell;
  };
  return Math.max(directed(a, b), directed(b, a));
}

/** 🚦️ Silhouette guard and hard findings of one scale. */
function score(asset: any, scaleKey: string, withGate = true, guardLimit = Infinity): Score {
  const scale = Number(scaleKey);
  const feature = featureOf(scale);
  let guard = 0;
  let text: string;
  const keys = new Map<string, { n: number; mm: number; subject: string }>();
  try {
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
    const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}${Math.abs(b - dims[k]!) > feature ? (guard++, "!") : ""}`).join(" ");
    text = `bbox ${bb} ` + (VIEWPOINTS as string[]).map((v) => `${v} ${m.views[v].iou.toFixed(4)}${m.views[v].iou < 0.92 ? (guard++, "!") : ""}/${(m.views[v].hausdorffM * 1000).toFixed(1)}${m.views[v].hausdorffM > 2 * feature ? (guard++, "!") : ""}`).join(" ");
  } catch (error) {
    guard = 99;
    text = `guard error: ${(error as Error).message.slice(0, 160)}`;
  }
  let hard = 0;
  if (guard > guardLimit) return { guard, hard: Infinity, text, keys };
  if (withGate)
    for (const subject of engine.subjects(asset, scale)) {
      const note = (key: string, mm: number): void => {
        hard++;
        const known = keys.get(key) ?? { n: 0, mm: -1, subject: subject.id };
        known.n++;
        if (mm > known.mm) Object.assign(known, { mm, subject: subject.id });
        keys.set(key, known);
      };
      let state: any;
      try {
        state = evaluate(asset, subject.query, engine.scaleSelector(scale));
      } catch (error) {
        const culprits: string[] = [];
        for (const view of ["elevation", "front", "plan"])
          for (const shape of asset.representations[scaleKey].geometry[view] as any[]) {
            if (shape.solid === undefined) continue;
            const single = structuredClone(asset);
            for (const other of ["front", "plan"]) if (other !== view) single.representations[scaleKey].geometry[other] = [];
            single.representations[scaleKey].geometry[view] = view === "elevation" ? single.representations[scaleKey].geometry[view] : [structuredClone(shape)];
            if (view === "elevation") single.representations[scaleKey].geometry[view] = (single.representations[scaleKey].geometry[view] as any[]).filter((candidate) => candidate.solid === undefined || candidate.id === shape.id);
            try {
              evaluate(single, subject.query, engine.scaleSelector(scale));
            } catch {
              culprits.push(`${view}/${shape.id}`);
            }
          }
        note(`evaluate-error [${culprits.join(", ")}] ${(error as Error).message.slice(0, 60)}`, 0);
        continue;
      }
      for (const finding of engine.geometryFindings(asset, state, subject.isDefault)) if (finding.tier === "hard") note(`${finding.check} ${finding.anchor ?? finding.bone ?? finding.views.join("/")}`, finding.delta?.deltaMm ?? 0);
    }
  return { guard, hard, text, keys };
}
const better = (a: Score, b: Score): boolean => a.guard < b.guard || (a.guard === b.guard && a.hard < b.hard);
const show = (label: string, s: Score): void => {
  log(`${label}: guard failures ${s.guard}, hard ${s.hard} | ${s.text}`);
  for (const [key, value] of [...s.keys].sort((x, y) => y[1].mm - x[1].mm)) log(`    ${key} ×${value.n} ${value.mm.toFixed(0)} mm @ ${value.subject}`);
};

let asset = read.doc.asset;
const authored = Object.keys(asset.representations).filter((key) => asset.representations[key].status === "authored");
if (args.includes("--report")) {
  for (const key of authored) show(`${asset.id} 1:${key}`, score(structuredClone(asset), key));
  process.exit(0);
}
const plan = JSON.parse(await Bun.file(flag("plan")!).text()) as Entry[];
const decided: Entry[] = [];
let changed = 0;
for (const key of authored) {
  const entries = plan.filter((entry) => entry.scale === key);
  if (entries.length === 0) continue;
  const fixed = entries.filter((entry) => entry.make !== undefined || entry.hand !== undefined);
  for (const entry of fixed) {
    try {
      if (entry.hand !== undefined) {
        const held = structuredClone(asset);
        const was = score(structuredClone(asset), key);
        const did = putHand(asset, key, entry.hand);
        const now = score(structuredClone(asset), key);
        show(`${asset.id} 1:${key} hand ${entry.hand} before`, was);
        show(`${asset.id} 1:${key} hand ${entry.hand} as palm and finger solids in three views`, now);
        if (entry.optional === true && better(was, now)) {
          asset = held;
          read.doc.asset = held;
          log(`${asset.id} 1:${key} hand ${entry.hand}: not kept (optional, and worse)`);
          continue;
        }
        if (did) changed++;
        decided.push({ scale: entry.scale, hand: entry.hand } as Entry);
        continue;
      }
      if (put(asset, entry, entry.make!)) changed++;
      decided.push(entry);
      log(`${asset.id} 1:${key} ${entry.view} ${entry.id}: ${entry.make}`);
    } catch (error) {
      log(`${asset.id} 1:${key} ${entry.view} ${entry.id}: REFUSED ${(error as Error).message}`);
    }
  }
  const open = entries.filter((entry) => entry.make === undefined && entry.hand === undefined);
  if (open.length === 0) {
    show(`${asset.id} 1:${key} after the fixed makes`, score(structuredClone(asset), key));
    continue;
  }
  show(`${asset.id} 1:${key} before`, score(structuredClone(asset), key));
  const ranks = new Map<Entry, { make: string; mm: number }[]>();
  for (const entry of open) {
    const rank: { make: string; mm: number }[] = [];
    for (const make of entry.try!) {
      try {
        const trial = structuredClone(asset);
        put(trial, entry, make);
        rank.push({ make, mm: make === "remove" ? 0 : misfit(trial, entry) * 1000 });
      } catch (error) {
        log(`    1:${key} ${entry.view} ${entry.id} ${make}: refused, ${(error as Error).message.slice(0, 140)}`);
      }
    }
    let now = NaN;
    try {
      now = misfit(structuredClone(asset), entry) * 1000;
    } catch {}
    rank.sort((x, y) => x.mm - y.mm);
    ranks.set(entry, rank);
    log(`  1:${key} ${entry.view} ${entry.id}: as it is ${now.toFixed(1)} mm; ${rank.map((r) => `${r.make} ${r.mm.toFixed(1)}`).join(", ")}`);
  }
  const choice = new Map<Entry, string>();
  const assemble = (): any => {
    const trial = structuredClone(asset);
    for (const [entry, make] of choice) put(trial, entry, make);
    return trial;
  };
  for (const entry of open) if (ranks.get(entry)!.length > 0) choice.set(entry, ranks.get(entry)![0]!.make);
  let current = score(assemble(), key);
  show(`${asset.id} 1:${key} best local makes`, current);
  for (let pass = 0; pass < 2 && (current.guard > 0 || current.hard > 0); pass++) {
    let improved = false;
    for (const entry of open) {
      if ((Date.now() - started) / 1000 > budgetS) break;
      const bone = shapeOf(asset, entry).bone as string;
      const whole = entry.view === "plan" ? ["sync-forward", "sync-lateral", "dimension-plan"] : ["sync-height", "sync-lateral", "dimension-front"];
      const named = [...current.keys.keys()].some((k) => k.endsWith(` ${bone}`) || k.includes(`/${entry.id}`) || (pass === 0 && whole.some((code) => k.startsWith(code))));
      if (current.guard === 0 && !named) continue;
      for (const { make, mm } of ranks.get(entry)!) {
        if (make === choice.get(entry) || mm > 6000 * featureOf(Number(key))) continue;
        const held = choice.get(entry)!;
        choice.set(entry, make);
        const next = score(assemble(), key, true, current.guard);
        log(`    try 1:${key} ${entry.view} ${entry.id} ${make}: guard ${next.guard}, hard ${next.hard} | ${next.text}`);
        if (better(next, current)) (current = next), (improved = true);
        else choice.set(entry, held);
      }
    }
    if (!improved) break;
  }
  show(`${asset.id} 1:${key} chosen`, current);
  for (const [entry, make] of choice) {
    if (put(asset, entry, make)) changed++;
    if (make !== "keep") decided.push({ scale: entry.scale, view: entry.view, id: entry.id, ...(entry.bone ? { bone: entry.bone } : {}), ...(entry.solids ? { solids: entry.solids } : {}), make });
    log(`${asset.id} 1:${key} ${entry.view} ${entry.id}: ${make}`);
  }
}
try {
  const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
  const { authoredScales, consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
  for (const [label, doc] of [["before", JSON.parse(read.text).asset], ["after", structuredClone(asset)]] as const) {
    const issues = validateAsset(doc) as { code: string }[];
    const findings = authoredScales(doc).flatMap((scale: number) => [...sweep(doc, scale), ...consistency(doc, scale)]) as { check: string }[];
    const count = (list: string[]): string => [...new Set(list)].map((code) => `${code}×${list.filter((c) => c === code).length}`).join(", ");
    log(`${asset.id} ${label}: validate ${issues.length} issues ${count(issues.map((issue) => issue.code))}; three-views sweep ${findings.length} findings ${count(findings.map((finding) => finding.check))}`);
  }
} catch (error) {
  log(`validate not run: ${(error as Error).message.slice(0, 200)}`);
}
if (flag("decided") !== undefined) writeFileSync(flag("decided")!, "[\n" + decided.map((entry) => "  " + JSON.stringify(entry)).join(",\n") + "\n]\n");
if (args.includes("--write") || flag("out") !== undefined) log(writeDoc(read, flag("out")) ? `written: ${flag("out") ?? read.file}` : "file unchanged");
log(`${asset.id}: ${plan.length} entries, ${changed} changed something`);
