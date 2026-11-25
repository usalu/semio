/** 🪆️ The multiview repair of an animal by nested prisms, the last step of the chain of the asset: `bun nest.ts <id|file.json> (--propose | --ops <ops.json>) [--turns <turns.json>] [--write | --out <file>]`. Run it from `T`.
 *
 * A front or plan shape of a part that turns is a picture of a body whose side is the ELEVATION twin. The op `nest` keeps the outline the view draws in the rest pose and makes the body from it: a few `extruded` prisms swept along the bone axis nearest to the view direction (`u` or `v`), each the authored outline cut to a band across the other axis and swept over the stretch along which the elevation twin is that thick. The (u, v) footprints of the prisms are rectangles inscribed in the hull of the elevation twin, so the three views keep the same extents at every sagittal angle; the prism of the full band carries the id and the paint of the shape and is painted last, so the rest drawing stays the authored outline; the inner prisms (`<id>-nest-<n>`) lie below it.
 *
 * Ops (`scale` is the representation key):
 * - `{ "op": "nest", scale, view, id, bone?, twin?, twinBone?, "max"?: 5, "target"?: 0.35, "stretch"?: false }`: `twin` and `twinBone` name the elevation shape when it has another id or lies on the mirror bone; levels are added until the footprint mismatch over the directions the bone takes (`--turns`) is below `target` (share of the tolerance of the asset) or `max` prisms are reached; the outline is stretched to the extent of the elevation twin along the view's own axis unless `"stretch": false`. A shape that is nested already is left alone, so a second run changes nothing.
 * - `{ "op": "copy", scale, view, id, bone?, twin? }`: the shape takes the geometry and the solid of its elevation twin (a revolved body is one body in three views).
 * - `{ "op": "lateral", scale, id, bone?, "to": "front" | "plan" }`: the nested prisms of the other view are scaled across the bone to the lateral range of the named view.
 * - `{ "op": "side", scale, id, bone?, "from": "front" | "plan" }`: the ELEVATION shape becomes an `extruded` solid swept along `w` over the lateral range of the twin of that view (for a part that a pose turns out of the sagittal plane).
 * - `{ "op": "set", scale, view, id, bone?, "set": {…} }`, `{ "op": "remove", scale, view, id, bone? }`, `{ "op": "anchor", id, "set": {…} }`: single fields by hand.
 *
 * `--propose` prints the `nest` ops for every closed front and plan shape with an elevation twin on a bone that a pose or clip turns.
 * @see ./fit.ts — the single-solid ops of the first pass
 * @see ../MR/gate.ts — the measurement */
import { join } from "node:path";
import { S, readDoc, round, support, writeDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry, evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const coverage = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));

type P2 = readonly [number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const frames = restFrames(asset.rig) as Record<string, { frame: Frame }>;
const turns = flag("turns") === undefined ? undefined : (JSON.parse(await Bun.file(flag("turns")!).text()) as Record<string, { min: number; max: number; lateral: number }>);
const toleranceM = Math.max(0.015, 0.02 * Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM));
const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"];
const PAINT_KEYS = ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const ANGLES = Array.from({ length: 72 }, (_, i) => (2 * Math.PI * i) / 72);
const SIDE = new Set(["n", "f", "near", "far", "l", "r", "hn", "hf", "fn", "ff"]);
const family = (name: string): string => name.split("-").filter((token) => !SIDE.has(token)).join("-");

const shapesOf = (scale: string, view: string): any[] => asset.representations[scale].geometry[view];
const closed = (shape: any): boolean => !(shape.kind === "poly" && shape.closed !== true) && shape.kind !== "member";
const pointsOf = (shape: any): P2[] => (shape.kind === "poly" ? (shape.points as P2[]) : flattenGeometry(shape).rings.flat().map((p: any) => [p.x, p.y] as const));
const rangeOf = (values: readonly number[]): [number, number] => [Math.min(...values), Math.max(...values)];
const need = (scale: string, view: string, id: string, bone?: string): any => {
  const shape = shapesOf(scale, view).find((candidate) => candidate.id === id && (bone === undefined || candidate.bone === bone));
  if (shape === undefined) throw new Error(`1:${scale} ${view} has no shape "${id}"${bone ? ` on ${bone}` : ""}`);
  return shape;
};

/** 🥚️ The convex hull of a point set, counter-clockwise. */
function hullOf(points: readonly P2[]): P2[] {
  const sorted = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
  const turn = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: readonly P2[]): P2[] => {
    const chain: P2[] = [];
    for (const p of list) {
      while (chain.length >= 2 && turn(chain[chain.length - 2]!, chain[chain.length - 1]!, p) <= 1e-12) chain.pop();
      chain.push(p);
    }
    chain.pop();
    return chain;
  };
  return sorted.length < 3 ? sorted : [...half(sorted), ...half([...sorted].reverse())];
}

/** ✂️ The interval a closed ring covers on the line `first coordinate = at` (lowest and highest crossing), undefined beside the ring. */
function chord(ring: readonly P2[], at: number): [number, number] | undefined {
  const hits: number[] = [];
  for (let i = 0; i < ring.length; i++) {
    const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
    if (Math.min(p[0], q[0]) > at || Math.max(p[0], q[0]) < at) continue;
    if (q[0] === p[0]) hits.push(p[1], q[1]);
    else hits.push(p[1] + ((q[1] - p[1]) * (at - p[0])) / (q[0] - p[0]));
  }
  return hits.length === 0 ? undefined : [Math.min(...hits), Math.max(...hits)];
}

/** 🎯️ Minimises a function of a few parameters by a pattern search with halving steps. */
function search(start: readonly number[], cost: (p: readonly number[]) => number, step: number, floor = 0.0005): number[] {
  const p = [...start];
  let best = cost(p);
  for (let s = step; s >= floor; s /= 2)
    for (let moved = true, guard = 0; moved && guard < 100; guard++) {
      moved = false;
      for (let i = 0; i < p.length; i++)
        for (const d of [s, -s]) {
          p[i]! += d;
          const c = cost(p);
          if (c < best - 1e-9) (best = c), (moved = true);
          else p[i]! -= d;
        }
    }
  return p;
}

/** 🧭️ The directions of the (u, v) plane of a bone in which the three views measure it: the world axes seen from the bone over the sagittal angles it takes in the poses, clips and transitions (with a margin of three degrees); undefined (every direction) for a bone that also leaves its plane or without `--turns`. */
function aim(bone: string, frame: Frame): number[] | undefined {
  const turn = turns?.[bone];
  if (turn === undefined || turn.lateral >= 10 || turn.max - turn.min >= 170) return undefined;
  const base = Math.atan2(frame.v[0]!, frame.u[0]!);
  const out: number[] = [];
  const [from, to] = [turn.min - 3, turn.max + 3];
  const steps = Math.max(2, Math.ceil((to - from) / 3));
  for (let i = 0; i <= steps; i++) for (let quarter = 0; quarter < 4; quarter++) out.push(base - ((from + ((to - from) * i) / steps) * Math.PI) / 180 + (quarter * Math.PI) / 2);
  return out;
}
const mismatch = (a: readonly P2[], b: readonly P2[], aimed: readonly number[] | undefined): number => Math.max(...(aimed ?? ANGLES).map((angle) => Math.abs(support(a, angle) - support(b, angle))));

const probes = new Map<string, any>();
/** 🛌️ The outline a shape draws in a view with every bone at rest, in rest-pose asset metres (the largest ring the evaluator draws in a pose without offsets, the travel of the root taken out); read once per scale before the scale is changed. */
function restRing(scale: string, view: "front" | "plan", shape: any): P2[] {
  let state = probes.get(scale);
  if (state === undefined) {
    const base = asset.poses.find((pose: any) => pose.id === asset.presentation.defaultPose);
    asset.poses.push({ ...structuredClone(base), id: "rest-probe", bones: {} });
    try {
      state = evaluate(asset, { kind: "pose", pose: "rest-probe" }, coverage.scaleSelector(Number(scale)));
    } finally {
      asset.poses.pop();
    }
    probes.set(scale, state);
  }
  const drawn = state.shapes[view].find((entry: any) => entry.shape === shape);
  if (drawn === undefined) throw new Error(`"${shape.id}" is not drawn in the ${view}`);
  const now = state.bones[shape.bone].frame.o;
  const rest = frames[shape.bone]!.frame.o;
  const shift = view === "front" ? [now.z - rest[2]!, now.y - rest[1]!] : [now.x - rest[0]!, now.z - rest[2]!];
  const area = (ring: any[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p.x * ring[(i + 1) % ring.length].y - ring[(i + 1) % ring.length].x * p.y, 0));
  const ring = [...drawn.rings].sort((x: any[], y: any[]) => area(y) - area(x))[0] as any[];
  return ring.map((p) => [p.x - shift[0]!, p.y - shift[1]!] as const);
}

const mirrorBone = (bone: string): string | undefined => {
  const other = bone.replace(/(^|-)(h|f)?(n|f)$/u, (_all: string, dash: string, limb: string | undefined, side: string) => `${dash}${limb ?? ""}${side === "n" ? "f" : "n"}`);
  return other !== bone && frames[other] !== undefined ? other : undefined;
};

/** 👯️ The elevation twin of a front or plan shape: the shape of the same id on the bone, else the only closed shape of the same role on the bone, else the one of the same family name, else the same on the mirror bone when the bone itself draws nothing in the elevation. */
function twinOf(scale: string, shape: any): { id: string; bone: string } | undefined {
  const list = shapesOf(scale, "elevation").filter((candidate) => closed(candidate) && candidate.role !== "ground");
  const on = (bone: string): any | undefined => {
    const same = list.filter((candidate) => candidate.bone === bone);
    const exact = same.find((candidate) => candidate.id === shape.id);
    if (exact !== undefined) return exact;
    const role = same.filter((candidate) => (candidate.role ?? "body") === (shape.role ?? "body"));
    const named = role.filter((candidate) => family(candidate.id) === family(shape.id));
    if (named.length === 1) return named[0];
    return role.length === 1 && (shape.role ?? "body") === "body" && shapesOf(scale, "front").concat(shapesOf(scale, "plan")).filter((candidate) => candidate.bone === bone && closed(candidate) && (candidate.role ?? "body") === "body" && candidate.id !== shape.id).every((candidate) => family(candidate.id) === family(shape.id)) ? role[0] : undefined;
  };
  const own = on(shape.bone);
  if (own !== undefined) return { id: own.id, bone: own.bone };
  if (list.some((candidate) => candidate.bone === shape.bone)) return undefined;
  const mirror = mirrorBone(shape.bone);
  const other = mirror === undefined ? undefined : on(mirror);
  return other === undefined ? undefined : { id: other.id, bone: other.bone };
}

type Span = readonly [number, number];
type Level = { band: [number, number]; zA: Span; zB: Span };
const THIN = 0.004;

/** 📐️ The interval of a line `p + c·d` inside a convex polygon (counter-clockwise), undefined when it misses it. */
function lineIn(polygon: readonly P2[], p: P2, d: P2): [number, number] | undefined {
  let [lo, hi] = [Number.NEGATIVE_INFINITY, Number.POSITIVE_INFINITY];
  for (let i = 0; i < polygon.length; i++) {
    const [a, b] = [polygon[i]!, polygon[(i + 1) % polygon.length]!];
    const n: P2 = [-(b[1] - a[1]), b[0] - a[0]];
    const [f, g] = [n[0] * (p[0] - a[0]) + n[1] * (p[1] - a[1]), n[0] * d[0] + n[1] * d[1]];
    if (Math.abs(g) < 1e-12) {
      if (f < -1e-9) return undefined;
      continue;
    }
    if (g > 0) lo = Math.max(lo, -f / g);
    else hi = Math.min(hi, -f / g);
  }
  return hi - lo > 0 ? [lo, hi] : undefined;
}

/** 🧵️ The intervals of a line `p + c·d` inside a closed ring (even-odd), sorted. */
function lineInRing(ring: readonly P2[], p: P2, d: P2): [number, number][] {
  const hits: number[] = [];
  for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    const e: P2 = [b[0] - a[0], b[1] - a[1]];
    const q = d[0] * e[1] - d[1] * e[0];
    if (Math.abs(q) < 1e-14) continue;
    const m = ((a[0] - p[0]) * d[1] - (a[1] - p[1]) * d[0]) / q;
    if (m < 0 || m >= 1) continue;
    hits.push(((a[0] - p[0]) * e[1] - (a[1] - p[1]) * e[0]) / q);
  }
  hits.sort((x, y) => x - y);
  const out: [number, number][] = [];
  for (let i = 0; i + 1 < hits.length; i += 2) out.push([hits[i]!, hits[i + 1]!]);
  return out;
}

/** 📏️ The distance from a point to a closed ring, 0 inside. */
function outside(ring: readonly P2[], p: P2): number {
  let inside = false;
  let best = Number.POSITIVE_INFINITY;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a[1] > p[1] !== b[1] > p[1] && p[0] < ((b[0] - a[0]) * (p[1] - a[1])) / (b[1] - a[1]) + a[0]) inside = !inside;
    const [dx, dy] = [b[0] - a[0], b[1] - a[1]];
    const k = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / (dx * dx + dy * dy)));
    best = Math.min(best, Math.hypot(p[0] - a[0] - k * dx, p[1] - a[1] - k * dy));
  }
  return inside ? 0 : best;
}

function nest(op: any): string {
  const view = op.view as "front" | "plan";
  const list = shapesOf(op.scale, view);
  const shape = need(op.scale, view, op.id, op.bone);
  const kept = shape.solid;
  if (kept?.kind === "extruded" && kept.sweep !== "w" && kept.z1 - kept.z0 <= THIN + 0.0002) return "unchanged: nested already";
  const frame = frames[shape.bone]!.frame;
  const length = asset.rig.bones.find((bone: any) => bone.id === shape.bone).lengthM as number;
  const twin = need(op.scale, "elevation", op.twin ?? op.id, op.twinBone ?? shape.bone);
  if (twin.solid?.kind === "revolved") throw new Error(`the elevation twin "${twin.id}" is a revolved solid`);
  const h = view === "front" ? 1 : 0;
  const d = view === "front" ? 0 : 1;
  const sweep: "u" | "v" = Math.abs(frame.u[d]!) >= Math.abs(frame.v[d]!) ? "u" : "v";
  const [tAxis, sAxis] = sweep === "u" ? [frame.v, frame.u] : [frame.u, frame.v];
  const [T, Sv, W, O]: P2[] = [[tAxis[2]!, tAxis[h]!], [sAxis[2]!, sAxis[h]!], [frame.w[2]!, frame.w[h]!], [frame.o[2]!, frame.o[h]!]];
  const det = T[0] * W[1] - T[1] * W[0];
  if (Math.abs(det) < 0.3) throw new Error(`the profile plane of bone "${shape.bone}" is nearly edge-on in the ${view}`);
  const side = pointsOf(twin);
  const hull = hullOf(side.map(([u, v]) => (sweep === "u" ? ([v, u] as const) : ([u, v] as const))));
  if (hull.length < 3) throw new Error(`the elevation twin "${twin.id}" has no area`);
  const toSide = ([t, s]: P2): P2 => (sweep === "u" ? [s, t] : [t, s]);
  const sideHull = hull.map(toSide);
  const [t0, t1] = rangeOf(hull.map((p) => p[0]));
  const heights = rangeOf(side.map(([u, v]) => frame.o[h]! + u * frame.u[h]! + v * frame.v[h]!));
  const drawn = (item: any, flip: 1 | -1): P2[] => restRing(op.scale, view, item).map(([a, b]) => (view === "front" ? ([flip * a, b] as const) : ([flip * b, a] as const)));
  const offOf = (points: readonly P2[]): number => Math.max(Math.abs(rangeOf(points.map((p) => p[1]))[0] - heights[0]), Math.abs(rangeOf(points.map((p) => p[1]))[1] - heights[1]));
  const small = Math.min(toleranceM, 0.00015 * Number(op.scale));
  let ring = drawn(shape, 1);
  let borrowed = "";
  if (offOf(ring) > small && op.stretch === undefined && op.mirror !== false) {
    const other = mirrorBone(shape.bone);
    const twinShape = other === undefined ? undefined : list.find((candidate) => candidate.bone === other && family(candidate.id) === family(shape.id) && closed(candidate));
    const copy = twinShape === undefined ? undefined : drawn(twinShape, -1);
    if (copy !== undefined && offOf(copy) <= small) (ring = copy), (borrowed = `, outline of the mirror shape "${twinShape.id}" (its own was ${(offOf(drawn(shape, 1)) * 1000).toFixed(0)} mm off the twin at rest)`);
  }
  const own = rangeOf(ring.map((p) => p[1]));
  const off = offOf(ring);
  const stretched = op.stretch !== false && own[1] - own[0] > 1e-6 && off > 0.0005 && (op.stretch === true || off <= small);
  const outline = stretched ? ring.map(([z, y]) => [z, heights[0] + ((y - own[0]) * (heights[1] - heights[0])) / (own[1] - own[0])] as const) : ring;
  const outlineHull = hullOf(outline);
  const tau = (t: number): number => (length > 0 ? Math.min(Math.max(t / length, 0), 1) : 0);
  const at = (level: Level, t: number): Span => [level.zA[0] + (level.zB[0] - level.zA[0]) * tau(t), level.zA[1] + (level.zB[1] - level.zA[1]) * tau(t)];
  const spanAt = (t: number): Span => chord(hull, Math.min(Math.max(t, t0 + 1e-7), t1 - 1e-7)) ?? [0, 0];
  const cap = (x: Span, y: Span): Span => {
    const inner: Span = [Math.max(x[0], y[0]), Math.min(x[1], y[1])];
    if (inner[1] - inner[0] >= THIN) return inner;
    const mid = Math.min(Math.max((x[0] + x[1]) / 2, y[0] + THIN / 2), Math.max(y[1] - THIN / 2, y[0] + THIN / 2));
    return [mid - THIN / 2, mid + THIN / 2];
  };
  const inscribed = (a: number, b: number): Level => {
    const [a1, b1] = [Math.min(Math.max(a, 0), length), Math.min(Math.max(b, 0), length)];
    const flat = cap(spanAt(a), spanAt(b));
    if (b1 - a1 < 0.005) return { band: [a, b], zA: flat, zB: flat };
    const [low, high] = [a1 === a ? cap(spanAt(a), spanAt(a)) : cap(spanAt(a), spanAt(a1)), b1 === b ? cap(spanAt(b), spanAt(b)) : cap(spanAt(b), spanAt(b1))];
    const [ta, tb] = [tau(a1), tau(b1)];
    const z = (k: 0 | 1, x: number): number => low[k] + ((high[k] - low[k]) * (x - ta)) / (tb - ta);
    const level: Level = { band: [a, b], zA: [z(0, 0), z(1, 0)], zB: [z(0, 1), z(1, 1)] };
    return level.zA[1] - level.zA[0] < 0.001 || level.zB[1] - level.zB[0] < 0.001 ? { band: [a, b], zA: flat, zB: flat } : level;
  };
  const footprint = (level: Level, band: readonly [number, number] = level.band): P2[] => [...new Set([band[0], Math.min(Math.max(0, band[0]), band[1]), Math.max(Math.min(length, band[1]), band[0]), band[1]])].flatMap((t) => at(level, t).map((s) => toSide([t, s])));
  const aimed = aim(shape.bone, frame);
  const full = inscribed(t0, t1);
  const centre = (span: Span): Span => [(span[0] + span[1]) / 2 - THIN / 2, (span[0] + span[1]) / 2 + THIN / 2];
  const rise = (p: P2): number => p[0] * T[1] + p[1] * Sv[1];
  const extreme = (sign: 1 | -1): P2 => {
    const best = Math.max(...hull.map((p) => sign * rise(p)));
    const tied = hull.filter((p) => sign * rise(p) >= best - 0.0005);
    return [tied.reduce((sum, p) => sum + p[0], 0) / tied.length, tied.reduce((sum, p) => sum + p[1], 0) / tied.length];
  };
  const [low, high] = [extreme(-1), extreme(1)];
  const [tauLow, tauHigh] = [tau(low[0]), tau(high[0])];
  const lean = Math.abs(tauHigh - tauLow) < 1e-3 ? 0 : (high[1] - low[1]) / (tauHigh - tauLow);
  const start = lean === 0 ? (low[1] + high[1]) / 2 : low[1] - lean * tauLow;
  let keeper: Level = { band: [Math.min(low[0], high[0]), Math.max(low[0], high[0])], zA: centre([start, start]), zB: centre([start + lean, start + lean]) };
  const sheared = (keeper.zB[0] - keeper.zA[0]) / Math.max(length, 1e-9);
  const folded = (T[0] + sheared * Sv[0]) * W[1] - (T[1] + sheared * Sv[1]) * W[0];
  if (Math.sign(folded) !== Math.sign(det) || Math.abs(folded) < 0.15) keeper = { band: keeper.band, zA: centre([(low[1] + high[1]) / 2, (low[1] + high[1]) / 2]), zB: centre([(low[1] + high[1]) / 2, (low[1] + high[1]) / 2]) };
  const slope = (keeper.zB[0] - keeper.zA[0]) / Math.max(length, 1e-9);
  const middle = (t: number): number => (at(keeper, t)[0] + at(keeper, t)[1]) / 2;
  const invert = (p: P2): P2 => {
    const solve = (s0: number, k: number): P2 => {
      const [bx, by] = [p[0] - O[0] - s0 * Sv[0], p[1] - O[1] - s0 * Sv[1]];
      const [ax, ay] = [T[0] + k * Sv[0], T[1] + k * Sv[1]];
      const q = ax * W[1] - ay * W[0];
      return [(bx * W[1] - by * W[0]) / q, (ax * by - ay * bx) / q];
    };
    const mid = solve(middle(0), slope);
    if (length <= 0 || (mid[0] >= 0 && mid[0] <= length)) return length <= 0 ? solve(middle(0), 0) : mid;
    return mid[0] < 0 ? solve(middle(0), 0) : solve(middle(length), 0);
  };
  const mapped: P2[] = [];
  for (let i = 0; i < outline.length; i++) {
    const [p, q] = [outline[i]!, outline[(i + 1) % outline.length]!];
    const [a, b] = [invert(p), invert(q)];
    mapped.push(a);
    const cuts = [0, length].filter((kink) => slope !== 0 && (a[0] - kink) * (b[0] - kink) < 0).map((kink) => {
      let [lo, hi] = [0, 1];
      const along = (x: number): P2 => invert([p[0] + (q[0] - p[0]) * x, p[1] + (q[1] - p[1]) * x]);
      for (let n = 0; n < 30; n++) {
        const x = (lo + hi) / 2;
        if ((along(x)[0] - kink) * (a[0] - kink) > 0) lo = x;
        else hi = x;
      }
      return lo;
    });
    for (const x of cuts.sort((m, n) => m - n)) mapped.push(invert([p[0] + (q[0] - p[0]) * x, p[1] + (q[1] - p[1]) * x]));
  }
  const keeperBand = rangeOf(mapped.map((p) => p[0]));
  let longest = (t0 + t1) / 2;
  for (let i = 1; i < 40; i++) {
    const t = t0 + ((t1 - t0) * i) / 40;
    if (spanAt(t)[1] - spanAt(t)[0] > spanAt(longest)[1] - spanAt(longest)[0]) longest = t;
  }
  const thin = Math.min(Math.max(0.002, 0.02 * (t1 - t0)), (t1 - t0) / 2);
  const spine: Level = { band: [Math.max(t0, longest - thin), Math.min(t1, longest + thin)], zA: spanAt(longest), zB: spanAt(longest) };
  const banded = (p: readonly number[]): Level[] => {
    const out: Level[] = [keeper, full, spine];
    for (let i = 0; i + 1 < p.length; i += 2) {
      const [a, b] = [Math.min(Math.max(p[i]!, t0), longest), Math.max(Math.min(p[i + 1]!, t1), longest)];
      if (b - a > 0.003) out.push(inscribed(a, b));
    }
    return out;
  };
  const cost = (levels: readonly Level[]): number => mismatch(levels.flatMap((level) => footprint(level)), sideHull, aimed);
  const target = (op.target ?? 0.3) * toleranceM;
  const most = op.max ?? 5;
  let levels: Level[] = [keeper];
  for (const count of [2, 3, 4, 5]) {
    if (cost(levels) <= target || count > most) break;
    const bands = count - 3;
    const start = Array.from({ length: Math.max(bands, 0) }, (_, i) => [longest - ((longest - t0) * (i + 1)) / (bands + 1), longest + ((t1 - longest) * (i + 1)) / (bands + 1)]).flat();
    const next = count === 2 ? [keeper, full] : bands === 0 ? [keeper, full, spine] : banded(search(start, (p) => cost(banded(p)), (t1 - t0) / 8));
    if (cost(next) < cost(levels) - 0.0005) levels = next;
  }
  const limit = op.excess ?? 0.0001 * Number(op.scale);
  const build = (level: Level): { points: P2[]; level: Level; band: [number, number]; excess: number } | undefined => {
    const interval = (t: number): [number, number] | undefined => {
      const [s0, s1] = at(level, t);
      let common: [number, number][] | undefined;
      for (const s of [s0, (s0 + s1) / 2, s1]) {
        const cuts = lineInRing(outline, [O[0] + t * T[0] + s * Sv[0], O[1] + t * T[1] + s * Sv[1]], W);
        common = common === undefined ? cuts : common.flatMap(([a, b]) => cuts.map(([c, e]) => [Math.max(a, c), Math.min(b, e)] as [number, number]).filter(([c, e]) => e - c > 0));
      }
      const cut = (common ?? []).sort((x, y) => y[1] - y[0] - (x[1] - x[0]))[0];
      return cut !== undefined && cut[1] - cut[0] > 0.0003 ? cut : undefined;
    };
    const [a, b] = level.band;
    const stations = [...new Set([...Array.from({ length: 13 }, (_, i) => a + ((b - a) * i) / 12), ...[0, length].filter((kink) => kink > a && kink < b)])].sort((x, y) => x - y);
    let run: { t: number; cut: [number, number] }[] = [];
    let best: typeof run = [];
    stations.forEach((t, i) => {
      const cut = interval(t);
      if (cut !== undefined) run.push({ t, cut });
      if (cut === undefined || i === stations.length - 1) {
        const width = (r: typeof run): number => (r.length === 0 ? -1 : r[r.length - 1]!.t - r[0]!.t);
        if (width(run) > width(best)) best = run;
        run = [];
      }
    });
    if (best.length === 0) return undefined;
    const edge = (inside: number, outside: number): { t: number; cut: [number, number] } => {
      let [x, y] = [inside, outside];
      for (let i = 0; i < 14; i++) {
        const mid = (x + y) / 2;
        if (interval(mid) !== undefined) x = mid;
        else y = mid;
      }
      return { t: x, cut: interval(x)! };
    };
    const first = stations.indexOf(best[0]!.t);
    const last = stations.indexOf(best[best.length - 1]!.t);
    if (first > 0) best.unshift(edge(best[0]!.t, stations[first - 1]!));
    if (last < stations.length - 1) best.push(edge(best[best.length - 1]!.t, stations[last + 1]!));
    if (best[best.length - 1]!.t - best[0]!.t < 0.0005) return undefined;
    const chain = [...best.map(({ t, cut }) => [t, cut[0]] as const), ...[...best].reverse().map(({ t, cut }) => [t, cut[1]] as const)];
    const points: P2[] = [];
    for (const p of chain) {
      const q = [round(p[0], 5), round(p[1], 5)] as const;
      if (points.length === 0 || Math.hypot(q[0] - points[points.length - 1]![0], q[1] - points[points.length - 1]![1]) > 0.0002) points.push(q);
    }
    if (points.length > 1 && Math.hypot(points[0]![0] - points[points.length - 1]![0], points[0]![1] - points[points.length - 1]![1]) <= 0.0002) points.pop();
    if (points.length < 3) return undefined;
    const excessOf = (list: readonly P2[]): number => Math.max(...list.flatMap(([t, c]) => at(level, t).map((s) => outside(outline, [O[0] + t * T[0] + s * Sv[0] + c * W[0], O[1] + t * T[1] + s * Sv[1] + c * W[1]]))));
    const convex = hullOf(points).map(([t, c]) => [round(t, 5), round(c, 5)] as const);
    const smooth = convex.length >= 3 && excessOf(convex) <= limit;
    return { points: smooth ? convex : points, level, band: [best[0]!.t, best[best.length - 1]!.t], excess: smooth ? 0 : excessOf(points) };
  };
  const wide = (level: Level): boolean => Math.max(level.zA[1] - level.zA[0], level.zB[1] - level.zB[0]) > 1.5 * THIN;
  const inner = levels.slice(1).filter(wide).map(build).filter((entry): entry is NonNullable<typeof entry> => entry !== undefined && entry.excess <= limit);
  const dropped = levels.slice(1).filter(wide).length - inner.length;
  const solidOf = (level: Level): any => {
    const [z0, z1, e0, e1] = [round(level.zA[0], 5), round(level.zA[1], 5), round(level.zB[0], 5), round(level.zB[1], 5)];
    return { kind: "extruded", sweep, z0, z1, ...(z0 === e0 && z1 === e1 ? {} : { zTip: [e0, e1] }) };
  };
  const index = list.indexOf(shape);
  const paint = Object.fromEntries(PAINT_KEYS.filter((key) => shape[key] !== undefined).map((key) => [key, structuredClone(shape[key])]));
  for (const key of GEOMETRY_KEYS) delete shape[key];
  const points: P2[] = [];
  for (const p of mapped) {
    const q = [round(p[0], 5), round(p[1], 5)] as const;
    if (points.length === 0 || Math.hypot(q[0] - points[points.length - 1]![0], q[1] - points[points.length - 1]![1]) > 0.00005) points.push(q);
  }
  Object.assign(shape, { kind: "poly", points, closed: true, solid: solidOf(keeper) });
  list.splice(index, 0, ...inner.map((entry, k) => ({ id: `${shape.id}-nest-${k + 1}`, bone: shape.bone, ...paint, kind: "poly", points: entry.points, closed: true, solid: solidOf(entry.level) })));
  const left = mismatch([...footprint(keeper, keeperBand), ...inner.flatMap((entry) => footprint(entry.level, entry.band))], sideHull, aimed);
  return `nested: ${inner.length + 1} prism(s) along ${sweep}, twin "${twin.id}"${twin.bone === shape.bone ? "" : ` of ${twin.bone}`}, footprint mismatch ${(left * 1000).toFixed(1)} mm${aimed ? "" : " (every direction)"}${dropped > 0 ? `, ${dropped} level(s) dropped (no room in the outline)` : ""}${borrowed}${stretched ? `, stretched [${own.map((v) => v.toFixed(3))}] to [${heights.map((v) => v.toFixed(3))}]` : off > 0.0005 && op.stretch !== false ? `, REST EXTENT DIFFERS by ${(off * 1000).toFixed(0)} mm: [${own.map((v) => v.toFixed(3))}] against the twin [${heights.map((v) => v.toFixed(3))}]` : ""}${left > toleranceM ? " OVER TOLERANCE" : ""}`;
}

/** ↔️ The lateral range (bone-local w) of a nested or flat shape of a view at rest. */
function lateralOf(scale: string, view: "front" | "plan", shape: any): [number, number] {
  if (shape.solid?.kind === "extruded" && shape.solid.sweep !== "w") return rangeOf((shape.points as P2[]).map((p) => p[1]));
  const frame = frames[shape.bone]!.frame;
  return rangeOf(restRing(scale, view, shape).map((p) => ((view === "front" ? p[0] : p[1]) - frame.o[2]!) / frame.w[2]!));
}

function lateral(op: any): string {
  const moved = op.to === "front" ? "plan" : "front";
  const target = need(op.scale, op.to, op.otherId ?? op.id, op.bone);
  const group = shapesOf(op.scale, moved).filter((shape) => shape.bone === target.bone && (shape.id === op.id || shape.id.startsWith(`${op.id}-nest-`)));
  if (group.length === 0) throw new Error(`1:${op.scale} ${moved} has no shape "${op.id}"`);
  if (group.some((shape) => !(shape.solid?.kind === "extruded" && shape.solid.sweep !== "w"))) throw new Error(`1:${op.scale} ${moved} "${op.id}" is not nested`);
  const to = lateralOf(op.scale, op.to, target);
  const from = rangeOf(group.flatMap((shape) => (shape.points as P2[]).map((p) => p[1])));
  if (Math.max(Math.abs(from[0] - to[0]), Math.abs(from[1] - to[1])) <= 0.0005) return "unchanged: lateral ranges agree";
  const k = (to[1] - to[0]) / (from[1] - from[0]);
  for (const shape of group) shape.points = (shape.points as P2[]).map(([t, w]) => [t, round(to[0] + (w - from[0]) * k, 5)]);
  return `matched: ${moved} lateral [${from.map((v) => v.toFixed(3))}] to the ${op.to} [${to.map((v) => v.toFixed(3))}]`;
}

function sideSolid(op: any): string {
  const shape = need(op.scale, "elevation", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const other = need(op.scale, op.from, op.otherId ?? op.id, op.otherBone ?? shape.bone);
  const group = shapesOf(op.scale, op.from).filter((candidate) => candidate.bone === other.bone && (candidate === other || candidate.id.startsWith(`${other.id}-nest-`)));
  const ranges = group.map((candidate) => lateralOf(op.scale, op.from, candidate));
  const mirrored = other.bone === shape.bone ? 1 : -1;
  const [w0, w1] = rangeOf(ranges.flat().map((w) => w * mirrored));
  const [z0, z1] = rangeOf([frame.o[2]! + w0 * frame.w[2]!, frame.o[2]! + w1 * frame.w[2]!]);
  const solid = { kind: "extruded", sweep: "w", z0: round(z0, 5), z1: round(z1, 5) };
  if (shape.kind !== "poly") {
    const points = pointsOf(shape).map(([u, v]) => [round(u, 5), round(v, 5)]);
    for (const key of GEOMETRY_KEYS) delete shape[key];
    Object.assign(shape, { kind: "poly", points, closed: true });
  }
  if (JSON.stringify(shape.solid) === JSON.stringify(solid)) return "unchanged";
  shape.solid = solid;
  return `set: the elevation shape is swept laterally over [${z0.toFixed(3)}, ${z1.toFixed(3)}]`;
}

function propose(): any[] {
  const parents = new Map<string, string | undefined>(asset.rig.bones.map((bone: any) => [bone.id, bone.parent ?? undefined]));
  const moved = new Set<string>();
  const visit = (node: unknown): void => {
    if (Array.isArray(node)) return node.forEach(visit);
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node as Record<string, unknown>)) {
      if (key === "bones" && value !== null && typeof value === "object" && !Array.isArray(value)) for (const [bone, offset] of Object.entries(value as Record<string, any>)) if (["angleDeg", "abductionDeg", "twistDeg"].some((field) => Math.abs(offset?.[field] ?? 0) >= 1)) moved.add(bone);
      visit(value);
    }
  };
  visit(asset.poses);
  visit(asset.clips);
  const turned = (bone: string): boolean => {
    for (let at: string | undefined = bone; at !== undefined; at = parents.get(at)) if (moved.has(at)) return true;
    return false;
  };
  const out: any[] = [];
  for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
    if (rep.status !== "authored") continue;
    for (const view of ["front", "plan"] as const)
      for (const shape of rep.geometry[view]) {
        if (shape.role === "ground" || !closed(shape) || !turned(shape.bone)) continue;
        const twin = twinOf(scale, shape);
        if (twin === undefined) {
          console.error(`${asset.id} 1:${scale} ${view} "${shape.id}" (${shape.bone}, ${shape.role}): no elevation twin, left as it is`);
          continue;
        }
        const side = rep.geometry.elevation.find((candidate: any) => candidate.id === twin.id && candidate.bone === twin.bone);
        if (side.solid?.kind === "revolved") {
          if (shape.solid?.kind !== "revolved" && twin.bone === shape.bone) out.push({ op: "copy", scale, view, id: shape.id, bone: shape.bone, twin: twin.id });
          continue;
        }
        out.push({ op: "nest", scale, view, id: shape.id, bone: shape.bone, ...(twin.id === shape.id && twin.bone === shape.bone ? {} : { twin: twin.id, twinBone: twin.bone }) });
      }
  }
  return out;
}

function apply(op: any): string {
  switch (op.op) {
    case "nest": return nest(op);
    case "lateral": return lateral(op);
    case "copy": {
      const shape = need(op.scale, op.view, op.id, op.bone);
      const twin = need(op.scale, "elevation", op.twin ?? op.id, shape.bone);
      const before = JSON.stringify(shape);
      for (const key of GEOMETRY_KEYS) delete shape[key];
      shape.kind = twin.kind;
      for (const key of GEOMETRY_KEYS) if (twin[key] !== undefined) shape[key] = structuredClone(twin[key]);
      return JSON.stringify(shape) === before ? "unchanged" : `copied: the body of the elevation twin "${twin.id}"`;
    }
    case "side": return sideSolid(op);
    case "set": {
      const shape = need(op.scale, op.view, op.id, op.bone);
      const before = JSON.stringify(shape);
      for (const [key, value] of Object.entries(op.set)) value === null ? delete shape[key] : (shape[key] = structuredClone(value));
      return JSON.stringify(shape) === before ? "unchanged" : "set";
    }
    case "remove": {
      const list = shapesOf(op.scale, op.view);
      const index = list.findIndex((shape) => shape.id === op.id && (op.bone === undefined || shape.bone === op.bone));
      if (index < 0) return "unchanged";
      list.splice(index, 1);
      return "removed";
    }
    case "anchor": {
      const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
      if (anchor === undefined) throw new Error(`no anchor "${op.id}"`);
      const before = JSON.stringify(anchor);
      for (const [key, value] of Object.entries(op.set)) value === null ? delete anchor[key] : (anchor[key] = value);
      return JSON.stringify(anchor) === before ? "unchanged" : "set";
    }
    default: throw new Error(`unknown op "${op.op}"`);
  }
}

if (args.includes("--propose")) {
  console.log(JSON.stringify(propose(), null, 1));
  process.exit(0);
}
const ops = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
let changed = 0;
let refused = 0;
for (const op of ops) {
  let result: string;
  try {
    result = apply(op);
  } catch (error) {
    refused++;
    result = `unchanged, REFUSED: ${(error as Error).message}`;
  }
  if (!result.startsWith("unchanged")) changed++;
  console.log(`${asset.id} ${op.op} ${op.scale ? `1:${op.scale} ` : ""}${op.view ?? ""}${op.view ? " " : ""}${op.id}${op.bone ? ` (${op.bone})` : ""}: ${result}`);
}
if (args.includes("--write") || flag("out") !== undefined) console.log(writeDoc(read, flag("out")) ? `written: ${flag("out") ?? read.file}` : "file unchanged");
console.log(`${asset.id}: ${ops.length} ops, ${changed} changed something, ${refused} refused`);
process.exit(refused === 0 ? 0 : 1);
