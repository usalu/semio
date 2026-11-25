/** 🧶️ The multiview repair of an animal by lofts, the last step of the chain of the asset: `bun loft.ts <id|file.json> (--propose | --ops <ops.json>) [--turns <turns.json>] [--write | --out <file>]`. Run it from `T`.
 *
 * A part that a pose turns is one body in the three views: a `loft` shape (D54) whose sections stand across u, with the v range of the ELEVATION outline and the lateral range of the body solids of the bone (or of the drawn front or plan) at every station, so the side elevation keeps its drawn outline and the front and the plan are projections of the same body.
 *
 * Ops (`scale` is the representation key):
 * - `{ "op": "loft", scale, bone, id, "merge"?: [id…], "add"?: ["front", "plan"], "front"?: id | null, "plan"?: id | null, "side"?: boolean, "solids"?: [id…], "width"?: "volume" | "body" | "front" | "plan", "axis"?: 0 | 1, "own"?: false, "tol"?: m, "exponent"?: n, "thin"?: m, "slim"?: m, "max"?: 0.5 }`: `id` is the elevation shape; the shapes `front` and `plan` (default the shape of the same id, null = leave the view alone) take the loft and keep their paint and place; `add` draws the part in views that left it out (paint of the elevation shape, just behind the shape of the nearest related bone); `merge` are further elevation shapes of the bone whose outline belongs to the body; with `side` the elevation shape becomes the loft too (a bone that leaves the sagittal plane). `width` names where the lateral range comes from (default the body solids `solids`, all of the bone when absent; without a solid the drawn views; `body` takes the sections of the largest loft solid as the volume states them, in the three views). The sections stand across u or across v (`axis`), or are the body's own (an ear, a wing that stands out sideways; `own: false` forbids it): the candidate that differs least from the solids in the front and the plan and from the drawn outline in the side is taken. A part whose best candidate misses more than `max` is left as it is (the outline and the solids are not the same part). `thin` narrows the body to that lateral extent. `slim` narrows the thinner of the two section axes so that the least dimension of the body (the measure of `volumes-feature`) is that many metres. The loft has the fewest sections (at most 32) whose linear outlines stay within `tol` (default 0.00001 m per unit of the denominator).
 * - `{ "op": "copy", scale, view, id, bone?, "from": view, "twin"?: id }`: the shape takes geometry and solid of the shape `twin` (default the same id) of another view.
 * - `{ "op": "set", scale, view, id, bone?, "set": {…} }`, `{ "op": "remove", scale, view, id, bone? }`, `{ "op": "anchor", id, "set": {…} }`: single fields by hand (a null value deletes the field).
 *
 * `--propose` prints the `loft` ops for every closed elevation shape that has a counterpart in the front or the plan which is not already the same solid (small details inside a larger outline stay flat). An elevation shape that is a loft already gives its own sections, so a second run changes nothing.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Loft shapes"
 * @see ../MR/gate.ts — the measurement */
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry, loftRings } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { loftFault } = await import(join(S, "../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts"));
const { copyMesh } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));

type P2 = readonly [number, number];
type V3 = readonly [number, number, number];
type Range = readonly [number, number];
type Section = { at: [number, number, number]; rx: number; ry: number; exponent?: number; rotation: number[] };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const frames = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
const turns = flag("turns") === undefined ? undefined : (JSON.parse(await Bun.file(flag("turns")!).text()) as Record<string, { min: number; max: number; lateral: number }>);
const VIEWS = ["elevation", "front", "plan"] as const;
const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "rotationDeg", "d", "followTip", "solid", "kind", "sections", "weights"];
const QUALITY = { chordM: 0.0005, minSegments: 24, maxSegments: 48 };
const LIMB = [0.5, 0.5, 0.5, 0.5];
const UPRIGHT = [-0.5, -0.5, -0.5, 0.5];
const THIN = 0.0003;
const MOST = 32;
const r5 = (value: number): number => Math.round(value * 1e5) / 1e5;
const list = (scale: string, view: string): any[] => asset.representations[scale].geometry[view];
const closed = (shape: any): boolean => shape.kind === "loft" || (shape.kind !== "member" && flattenGeometry(shape).closed && shape.filled !== false);
const find = (scale: string, view: string, id: string, bone?: string): any => list(scale, view).find((shape) => shape.id === id && (bone === undefined || shape.bone === bone));
const need = (scale: string, view: string, id: string, bone?: string): any => {
  const shape = find(scale, view, id, bone);
  if (shape === undefined) throw new Error(`1:${scale} ${view} has no shape "${id}"${bone ? ` on ${bone}` : ""}`);
  return shape;
};
const ringsOf = (shape: any): P2[][] => {
  if (shape.kind === "loft") {
    const s = shape.sections as Section[];
    return [[...s.map((q): P2 => [q.at[0], q.at[1] - q.rx]), ...[...s].reverse().map((q): P2 => [q.at[0], q.at[1] + q.rx])]];
  }
  return flattenGeometry(shape).rings.map((ring: any[]) => ring.map((p): P2 => [p.x, p.y]));
};

/** 🔪️ The range of closed rings across the first axis at `a`, or undefined where they are not. */
function cut(rings: readonly (readonly P2[])[], a: number): Range | undefined {
  let [low, high] = [Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY];
  for (const ring of rings)
    for (let i = 0; i < ring.length; i++) {
      const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
      if (Math.min(p[0], q[0]) > a || Math.max(p[0], q[0]) < a) continue;
      for (const b of p[0] === q[0] ? [p[1], q[1]] : [p[1] + ((q[1] - p[1]) * (a - p[0])) / (q[0] - p[0])]) {
        low = Math.min(low, b);
        high = Math.max(high, b);
      }
    }
  return high >= low ? [low, high] : undefined;
}
const spanOf = (rings: readonly (readonly P2[])[]): Range => [Math.min(...rings.flat().map((p) => p[0])), Math.max(...rings.flat().map((p) => p[0]))];

const gamma = (x: number): number => {
  const g = [676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7];
  if (x < 0.5) return Math.PI / (Math.sin(Math.PI * x) * gamma(1 - x));
  let a = 0.99999999999980993;
  for (let i = 0; i < g.length; i++) a += g[i]! / (x + i);
  const t = x + 6.5;
  return Math.sqrt(2 * Math.PI) * t ** (x - 0.5) * Math.exp(-t) * a;
};
const fullness = (n: number): number => gamma(1 + 1 / n) ** 2 / gamma(1 + 2 / n);
function exponentOf(target: number): number {
  let [lo, hi] = [1, 20];
  for (let i = 0; i < 40; i++) {
    const mid = (lo + hi) / 2;
    if (fullness(mid) < target) lo = mid;
    else hi = mid;
  }
  return Math.round(((lo + hi) / 2) * 10) / 10;
}
function hullArea(points: readonly P2[]): number {
  const sorted = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
  const turn = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (input: readonly P2[]): P2[] => {
    const chain: P2[] = [];
    for (const p of input) {
      while (chain.length >= 2 && turn(chain[chain.length - 2]!, chain[chain.length - 1]!, p) <= 0) chain.pop();
      chain.push(p);
    }
    chain.pop();
    return chain;
  };
  const hull = [...half(sorted), ...half([...sorted].reverse())];
  return Math.abs(hull.reduce((sum, p, i) => sum + p[0] * hull[(i + 1) % hull.length]![1] - hull[(i + 1) % hull.length]![0] * p[1], 0)) / 2;
}

const MESHES = new Map<any, V3[][]>();
const meshOf = (solid: any): V3[][] => {
  const known = MESHES.get(solid);
  if (known !== undefined) return known;
  const { positions, indices } = copyMesh(solid, QUALITY) as { positions: Float64Array; indices: Uint32Array };
  const triangles = Array.from({ length: indices.length / 3 }, (_, i) => [0, 1, 2].map((k): V3 => [positions[3 * indices[3 * i + k]!]!, positions[3 * indices[3 * i + k]! + 1]!, positions[3 * indices[3 * i + k]! + 2]!]));
  MESHES.set(solid, triangles);
  return triangles;
};
type Slice = { w: Range; exponent: number };
type Axis = 0 | 1;

/** 🍞️ The lateral range and the fullness of the body solids at stations along a bone axis (0 = u, 1 = v); outside the solids the nearest slice. */
function slicer(solids: readonly any[], axis: Axis): ((a: number) => Slice) | undefined {
  const triangles = solids.flatMap(meshOf);
  if (triangles.length === 0) return undefined;
  const other = axis === 0 ? 1 : 0;
  let [low, high] = [Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY];
  for (const t of triangles) for (const p of t) [low, high] = [Math.min(low, p[axis]), Math.max(high, p[axis])];
  const inset = Math.min(0.002, (high - low) / 12);
  return (value) => {
    const at = Math.min(Math.max(value, low + inset), high - inset);
    const points: P2[] = [];
    for (const [a, b, c] of triangles)
      for (const [p, q] of [[a!, b!], [b!, c!], [c!, a!]] as const) {
        if ((p[axis] - at) * (q[axis] - at) > 0 || p[axis] === q[axis]) continue;
        const t = (at - p[axis]) / (q[axis] - p[axis]);
        points.push([p[other] + t * (q[other] - p[other]), p[2] + t * (q[2] - p[2])]);
      }
    const ws = points.map((p) => p[1]);
    const vs = points.map((p) => p[0]);
    const w: Range = [Math.min(...ws), Math.max(...ws)];
    const box = (w[1] - w[0]) * (Math.max(...vs) - Math.min(...vs));
    return { w, exponent: box > 1e-9 ? Math.min(6, Math.max(1.8, exponentOf(Math.min(0.98, hullArea(points) / box)))) : 2 };
  };
}

/** 🪟️ The lateral range of a drawn flat front or plan shape at stations along u (by the height or the forward position of the middle of the elevation outline there). */
function drawn(scale: string, view: "front" | "plan", id: string, bone: string, rings: readonly (readonly P2[])[]): ((u: number) => Slice) | undefined {
  const shape = find(scale, view, id, bone);
  if (shape?.kind === undefined || shape.solid !== undefined || shape.kind === "loft" || shape.kind === "member") return undefined;
  const frame = frames[bone]!.frame;
  const axis = view === "front" ? 1 : 0;
  const outline = flattenGeometry(shape).rings.map((ring: any[]) => ring.map((p): P2 => (view === "front" ? [p.y, p.x - frame.o[2]] : [p.x, p.y - frame.o[2]])));
  const [low, high] = spanOf(outline);
  return (u) => {
    const v = cut(rings, u) ?? [0, 0];
    const key = frame.o[axis]! + u * frame.u[axis]! + ((v[0] + v[1]) / 2) * frame.v[axis]!;
    return { w: cut(outline, Math.min(Math.max(key, low + 1e-4), high - 1e-4))!, exponent: 2.4 };
  };
}

/** 🧺️ The sections of a body: the fewest stations whose linear interpolation of the four edges stays within the tolerance. */
function sectionsOf(outline: readonly (readonly P2[])[], axis: Axis, slice: (a: number) => Slice, tol: number, exponent: number | undefined): Section[] {
  const rings = axis === 0 ? outline : outline.map((ring) => ring.map((p): P2 => [p[1], p[0]]));
  const [u0, u1] = spanOf(rings);
  const edge = Math.min(1e-4, (u1 - u0) / 50);
  const vAt = (u: number): Range => cut(rings, Math.min(Math.max(u, u0 + edge), u1 - edge))!;
  const row = (u: number): number[] => {
    const [v, s] = [vAt(u), slice(u)];
    return [v[0], v[1], s.w[0], s.w[1]];
  };
  const inside = (u: number): boolean => cut(rings, Math.min(Math.max(u, u0 + edge), u1 - edge)) !== undefined;
  const vertices = rings.flat().map((p) => p[0]);
  const grid = Array.from({ length: 41 }, (_, i) => u0 + ((u1 - u0) * i) / 40);
  const candidates = [...new Set([...vertices.flatMap((u) => [u - edge, u, u + edge]), ...grid].map((u) => Math.min(Math.max(u, u0), u1)).map((u) => Math.round(u * 1e7) / 1e7).filter(inside))].sort((a, b) => a - b);
  const rows = candidates.map(row);
  const chosen = new Set<number>([0, candidates.length - 1]);
  for (;;) {
    const order = [...chosen].sort((a, b) => a - b);
    let [worst, at] = [tol, -1];
    for (let k = 0; k + 1 < order.length; k++) {
      const [i, j] = [order[k]!, order[k + 1]!];
      for (let m = i + 1; m < j; m++) {
        const t = (candidates[m]! - candidates[i]!) / (candidates[j]! - candidates[i]!);
        for (let e = 0; e < 4; e++) {
          const miss = Math.abs(rows[m]![e]! - (rows[i]![e]! + t * (rows[j]![e]! - rows[i]![e]!)));
          if (miss > worst) [worst, at] = [miss, m];
        }
      }
    }
    if (at < 0 || chosen.size >= MOST) break;
    chosen.add(at);
  }
  const order = [...chosen].sort((a, b) => a - b).filter((k, i, all) => i === 0 || candidates[k]! - candidates[all[i - 1]!]! > 2e-5);
  return order.map((k) => {
    const [v0, v1, w0, w1] = rows[k]!;
    const n = exponent ?? slice(candidates[k]!).exponent;
    const [a, b, c] = [r5(candidates[k]!), r5((v0! + v1!) / 2), r5((w0! + w1!) / 2)];
    const [half, wide] = [r5(Math.max(THIN, (v1! - v0!) / 2)), r5(Math.max(THIN, (w1! - w0!) / 2))];
    return axis === 0 ? { at: [a, b, c], rx: half, ry: wide, ...(n === 2 ? {} : { exponent: n }), rotation: LIMB } : { at: [b, a, c], rx: wide, ry: half, ...(n === 2 ? {} : { exponent: n }), rotation: UPRIGHT };
  });
}

const boxOf = (rings: readonly (readonly P2[])[]): [number, number, number, number] => {
  const points = rings.flat();
  return [Math.min(...points.map((p) => p[0])), Math.max(...points.map((p) => p[0])), Math.min(...points.map((p) => p[1])), Math.max(...points.map((p) => p[1]))];
};

/** ↔️ The lateral range of drawn flat front and plan shapes in the bone frame, or undefined. */
function sideOf(bone: string, targets: readonly (readonly [string, any])[]): Range | undefined {
  const values = targets.filter(([, shape]) => shape.kind !== undefined && shape.solid === undefined && shape.kind !== "loft" && shape.kind !== "member").flatMap(([view, shape]) => flattenGeometry(shape).rings.flat().map((p: any) => (view === "front" ? p.x : p.y) - frames[bone]!.frame.o[2]));
  return values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)];
}

/** 🎯️ The body solids of the bone that the elevation outline draws: all when the bone draws one closed shape, else those whose side box lies mostly inside the box of the outline and whose middle lies in the lateral range of the drawn counterparts (a pair of horns is two bodies). */
function solidsOf(scale: string, bone: string, rings: readonly (readonly P2[])[], single: boolean, side: Range | undefined): any[] {
  const own = (asset.volumes?.[scale]?.body ?? []).filter((solid: any) => solid.bone === bone && solid.weights === undefined);
  if (single) return own;
  const box = boxOf(rings);
  return own.filter((solid: any) => {
    const ws = meshOf(solid).flat().map((p) => p[2]);
    const middle = (ws.reduce((a, b) => Math.min(a, b)) + ws.reduce((a, b) => Math.max(a, b))) / 2;
    if (side !== undefined && (middle < side[0] - 0.005 || middle > side[1] + 0.005)) return false;
    const points = meshOf(solid).flat();
    const s = boxOf([points.map((p): P2 => [p[0], p[1]])]);
    const grown = 0.1 * Math.max(box[1] - box[0], box[3] - box[2]);
    const inside = Math.max(0, Math.min(s[1], box[1] + grown) - Math.max(s[0], box[0] - grown)) * Math.max(0, Math.min(s[3], box[3] + grown) - Math.max(s[2], box[2] - grown));
    const size = Math.max(1e-9, (s[1] - s[0]) * (s[3] - s[2]));
    return inside / size > 0.7;
  });
}

/** 🧮️ The side of a cell grid of a raster comparison. */
const CELLS = 90;
type Grid = { x0: number; y0: number; cell: number; nx: number; ny: number; on: Uint8Array };
const gridOf = (points: readonly P2[]): Grid => {
  const [x0, x1, y0, y1] = boxOf([points]);
  const cell = Math.max(x1 - x0, y1 - y0, 1e-6) / CELLS;
  const [nx, ny] = [Math.ceil((x1 - x0) / cell) + 1, Math.ceil((y1 - y0) / cell) + 1];
  return { x0, y0, cell, nx, ny, on: new Uint8Array(nx * ny) };
};
function fill(grid: Grid, polygon: readonly P2[]): void {
  const [a0, a1, b0, b1] = boxOf([polygon]);
  for (let j = Math.max(0, Math.floor((b0 - grid.y0) / grid.cell) - 1); j < Math.min(grid.ny, Math.ceil((b1 - grid.y0) / grid.cell) + 1); j++) {
    const y = grid.y0 + (j + 0.5) * grid.cell;
    const xs: number[] = [];
    for (let i = 0; i < polygon.length; i++) {
      const [p, q] = [polygon[i]!, polygon[(i + 1) % polygon.length]!];
      if (p[1] > y !== q[1] > y) xs.push(p[0] + ((q[0] - p[0]) * (y - p[1])) / (q[1] - p[1]));
    }
    xs.sort((s, t) => s - t);
    for (let k = 0; k + 1 < xs.length; k += 2) for (let i = Math.max(0, Math.round((xs[k]! - grid.x0) / grid.cell)); i < Math.min(grid.nx, Math.round((xs[k + 1]! - grid.x0) / grid.cell)); i++) grid.on[j * grid.nx + i] = 1;
  }
}
const hullOf = (points: readonly P2[]): P2[] => {
  const sorted = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
  const half = (input: readonly P2[]): P2[] => {
    const chain: P2[] = [];
    for (const p of input) {
      while (chain.length >= 2 && (chain[chain.length - 1]![0] - chain[chain.length - 2]![0]) * (p[1] - chain[chain.length - 2]![1]) - (chain[chain.length - 1]![1] - chain[chain.length - 2]![1]) * (p[0] - chain[chain.length - 2]![0]) <= 0) chain.pop();
      chain.push(p);
    }
    chain.pop();
    return chain;
  };
  return [...half(sorted), ...half([...sorted].reverse())];
};

/** ⚖️ How far a loft is from the body solids in the front and in the plan at rest, and from the drawn outline in the side elevation: the area they do not share over the area of the solids (of the outline), added up. */
function missOf(bone: string, sections: Section[], solids: readonly any[], outline: readonly (readonly P2[])[]): number {
  const { o, u, v, w } = frames[bone]!.frame;
  const world = (p: V3): V3 => [0, 1, 2].map((k) => o[k]! + p[0] * u[k]! + p[1] * v[k]! + p[2] * w[k]!) as unknown as V3;
  const rings = loftRings({ sections }) as V3[][];
  const triangles = solids.flatMap(meshOf);
  let total = 0;
  for (const view of VIEWS) {
    const flat = (p: V3): P2 => {
      const q = world(p);
      return view === "elevation" ? [q[0], q[1]] : view === "front" ? [q[2], q[1]] : [q[0], q[2]];
    };
    const bodies = rings.slice(1).map((ring, i) => hullOf([...rings[i]!, ...ring].map(flat)));
    const target = view === "elevation" ? outline.map((ring) => ring.map((p) => flat([p[0], p[1], 0]))) : triangles.map((t) => t.map(flat));
    const all = [...bodies.flat(), ...target.flat()];
    const [a, b] = [gridOf(all), gridOf(all)];
    for (const body of bodies) if (body.length >= 3) fill(a, body);
    for (const polygon of target) fill(b, polygon);
    let [differ, size] = [0, 0];
    for (let i = 0; i < a.on.length; i++) {
      differ += a.on[i]! ^ b.on[i]!;
      size += b.on[i]!;
    }
    total += differ / Math.max(1, size);
  }
  return total;
}

/** 🫀️ The sections of the largest loft solid as the volume states them. */
function ownSections(solids: readonly any[]): Section[] | undefined {
  const lofts = solids.filter((solid) => solid.kind === "loft" && solid.transform === undefined && solid.repeat === undefined);
  const size = (solid: any): number => {
    const points = meshOf(solid).flat();
    return [0, 1, 2].reduce((product, k) => product * (points.map((p) => p[k]!).reduce((a, b) => Math.max(a, b)) - points.map((p) => p[k]!).reduce((a, b) => Math.min(a, b))), 1);
  };
  return lofts.length === 0 ? undefined : structuredClone(lofts.map((solid) => [solid, size(solid)] as const).sort((a, b) => b[1] - a[1])[0]![0].sections);
}

const SIDE = new Set(["n", "f", "near", "far", "l", "r", "hn", "hf", "fn", "ff"]);
const family = (name: string): string => name.split("-").filter((token) => !SIDE.has(token)).join("-");

const PAINT_KEYS = ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const parentOf = (bone: string): string | undefined => asset.rig.bones.find((candidate: any) => candidate.id === bone)?.parent ?? undefined;

/** 🧑‍🤝‍🧑️ The closed shape of the nearest related bone in a view: its ancestors first, then its descendants. */
function neighbour(scale: string, view: string, bone: string): any {
  const on = (id: string): any => list(scale, view).find((shape) => shape.bone === id && shape.role === "body" && (shape.kind === "loft" || closed(shape)));
  for (let cursor = parentOf(bone), k = 0; cursor !== undefined && k < 3; cursor = parentOf(cursor), k++) if (on(cursor) !== undefined) return on(cursor);
  let level = [bone];
  for (let k = 0; k < 3; k++) {
    level = asset.rig.bones.filter((candidate: any) => level.includes(candidate.parent)).map((candidate: any) => candidate.id);
    for (const id of level) if (on(id) !== undefined) return on(id);
  }
  return undefined;
}

if (args.includes("--propose")) {
  const ops: any[] = [];
  for (const [scale, representation] of Object.entries(asset.representations as Record<string, any>)) {
    if (representation.status !== "authored" || !/^\d+$/.test(scale)) continue;
    const bones = [...new Set(list(scale, "elevation").map((shape) => shape.bone as string))];
    for (const bone of bones) {
      const on = (view: string): any[] => list(scale, view).filter((shape) => shape.bone === bone && shape.role !== "ground" && shape.kind !== "member" && closed(shape));
      const [e, f, p] = [on("elevation"), on("front"), on("plan")];
      const partner = (shape: any, others: any[]): any => others.find((other) => other.id === shape.id) ?? (e.length === 1 && others.length === 1 ? others[0] : others.find((other) => family(other.id) === family(shape.id) && !e.some((x) => x.id === other.id)));
      const boxes = new Map(e.map((shape) => [shape, boxOf(ringsOf(shape))]));
      const detail = (shape: any): boolean => {
        const b = boxes.get(shape)!;
        return e.some((other) => {
          const o = boxes.get(other)!;
          return other !== shape && b[0] > o[0] && b[1] < o[1] && b[2] > o[2] && b[3] < o[3] && (b[1] - b[0]) * (b[3] - b[2]) < 0.08 * (o[1] - o[0]) * (o[3] - o[2]);
        });
      };
      const pairs = e.filter((shape) => !detail(shape)).map((shape) => ({ shape, front: partner(shape, f), plan: partner(shape, p) }));
      const partnered = pairs.filter((pair) => pair.front !== undefined || pair.plan !== undefined);
      const rest = pairs.filter((pair) => pair.front === undefined && pair.plan === undefined);
      const missing = (["front", "plan"] as const).filter((view) => (view === "front" ? f : p).length === 0 && e.length === 1 && e[0].role === "body" && neighbour(scale, view, bone) !== undefined);
      const main = partnered.length > 0 ? partnered : missing.length > 0 ? pairs.slice(0, 1) : [];
      for (const pair of main) {
        const same = (other: any): boolean => other === undefined || (other.kind === pair.shape.kind && JSON.stringify(other.solid) === JSON.stringify(pair.shape.solid) && JSON.stringify(other.points) === JSON.stringify(pair.shape.points) && other.d === pair.shape.d);
        if (pair.shape.solid !== undefined && same(pair.front) && same(pair.plan)) continue;
        if (pair.shape.solid?.kind === "revolved") {
          for (const [view, other] of [["front", pair.front], ["plan", pair.plan]] as const) if (other !== undefined && !same(other)) ops.push({ op: "copy", scale, view, bone, id: other.id, from: "elevation", ...(other.id === pair.shape.id ? {} : { twin: pair.shape.id }) });
          continue;
        }
        const lateral = turns?.[bone]?.lateral ?? 0;
        const single = main.length === 1 && f.length <= 1 && p.length <= 1;
        ops.push({ op: "loft", scale, bone, id: pair.shape.id, ...(single && partnered.length > 0 && rest.length > 0 ? { merge: rest.map((r) => r.shape.id) } : {}), ...(missing.length > 0 ? { add: missing } : {}), ...(pair.front === undefined ? (missing.includes("front") ? {} : { front: null }) : pair.front.id === pair.shape.id ? {} : { front: pair.front.id }), ...(pair.plan === undefined ? (missing.includes("plan") ? {} : { plan: null }) : pair.plan.id === pair.shape.id ? {} : { plan: pair.plan.id }), ...(lateral > 4 || pair.shape.solid !== undefined ? { side: true } : {}) });
      }
    }
  }
  console.log(`[\n${ops.map((op) => " " + JSON.stringify(op)).join(",\n")}\n]`);
  process.exit(0);
}

const ops = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
let changed = 0;
let refused = 0;
const put = (shape: any, next: any): boolean => {
  const before = JSON.stringify(shape);
  for (const key of GEOMETRY_KEYS) delete shape[key];
  Object.assign(shape, next);
  return JSON.stringify(shape) !== before;
};
for (const op of ops) {
  const before = JSON.stringify(asset.representations[op.scale]?.geometry ?? asset.anchors);
  const label = `1:${op.scale ?? ""} ${op.bone ?? ""} ${op.id}`;
  if (op.op === "anchor") {
    const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
    if (anchor === undefined) throw new Error(`no anchor ${op.id}`);
    const was = JSON.stringify(anchor);
    for (const [key, value] of Object.entries(op.set)) if (value === null) delete anchor[key];
    else anchor[key] = value;
    if (JSON.stringify(anchor) !== was) changed++;
    continue;
  }
  if (op.op === "remove") {
    const shapes = list(op.scale, op.view);
    const at = shapes.findIndex((shape) => shape.id === op.id && (op.bone === undefined || shape.bone === op.bone));
    if (at >= 0) {
      shapes.splice(at, 1);
      changed++;
    }
    continue;
  }
  if (op.op === "set") {
    const shape = need(op.scale, op.view, op.id, op.bone);
    for (const [key, value] of Object.entries(op.set)) if (value === null) delete shape[key];
    else shape[key] = value;
  } else if (op.op === "copy") {
    const source = need(op.scale, op.from, op.twin ?? op.id, op.bone);
    put(need(op.scale, op.view, op.id, op.bone), Object.fromEntries(GEOMETRY_KEYS.filter((key) => source[key] !== undefined).map((key) => [key, structuredClone(source[key])])));
  } else if (op.op === "loft") {
    const main = need(op.scale, "elevation", op.id, op.bone);
    for (const view of (op.add ?? []) as string[]) {
      const shapes = list(op.scale, view);
      if (shapes.some((shape) => shape.id === op.id && shape.bone === op.bone)) continue;
      const near = neighbour(op.scale, view, op.bone);
      if (near === undefined) throw new Error(`${label}: no shape of a related bone in the ${view}`);
      const paint = Object.fromEntries(PAINT_KEYS.filter((key) => main[key] !== undefined).map((key) => [key, structuredClone(main[key])]));
      shapes.splice(shapes.indexOf(near), 0, { id: op.id, bone: op.bone, ...paint, z: Math.round((near.z - 0.05) * 100) / 100 });
    }
    const targets = (["front", "plan"] as const).filter((view) => op[view] !== null).map((view) => [view, need(op.scale, view, op[view] ?? op.id, op.bone)] as const);
    let sections: Section[] | undefined = main.kind === "loft" ? main.sections : undefined;
    if (sections === undefined && op.side !== true) sections = targets.map(([, shape]) => shape).find((shape) => shape.kind === "loft")?.sections;
    if (sections === undefined) {
      const rings = [main, ...(op.merge ?? []).map((id: string) => need(op.scale, "elevation", id, op.bone))].flatMap(ringsOf);
      const others = list(op.scale, "elevation").filter((shape) => shape.bone === op.bone && shape.kind !== "member" && closed(shape));
      const picked = op.solids === undefined ? solidsOf(op.scale, op.bone, rings, others.length === 1 + (op.merge ?? []).length, sideOf(op.bone, targets)) : (asset.volumes?.[op.scale]?.body ?? []).filter((solid: any) => solid.bone === op.bone && op.solids.includes(solid.id));
      const steep = Math.abs(frames[op.bone]!.frame.u[1]) >= Math.abs(frames[op.bone]!.frame.u[0]);
      const tol = op.tol ?? 0.00001 * Number(op.scale);
      const candidates: [string, Section[]][] = [];
      const plain = (op.width ?? "volume") === "volume" && picked.length > 0;
      if (plain) for (const axis of [0, 1] as const) if (op.axis === undefined || op.axis === axis) candidates.push([axis === 0 ? "across u" : "across v", sectionsOf(rings, axis, slicer(picked, axis)!, tol, op.exponent)]);
      if (op.width === "body") candidates.length = 0;
      if (op.width === "body" || (plain && op.axis === undefined && op.own !== false)) {
        const own = ownSections(picked);
        if (own !== undefined && loftFault(own) === null) candidates.push(["own", own]);
      }
      if (candidates.length === 0)
        for (const view of op.width !== undefined && op.width !== "volume" ? [op.width] : steep ? ["front", "plan"] : ["plan", "front"]) {
          const slice = op[view] === null ? undefined : drawn(op.scale, view, op[view] ?? op.id, op.bone, rings);
          if (slice === undefined) continue;
          candidates.push([`drawn ${view}`, sectionsOf(rings, 0, slice, tol, op.exponent)]);
          break;
        }
      if (candidates.length === 0) {
        console.log(`${label}: REFUSED, no body solid and no drawn width`);
        refused++;
        continue;
      }
      const scored = candidates.map(([name, found]) => ({ name, found, miss: picked.length === 0 ? 0 : missOf(op.bone, found, picked, rings) + (name === "own" ? 0.12 : 0) })).sort((p, q) => p.miss - q.miss);
      if (scored[0]!.miss > (op.max ?? 0.5)) {
        console.log(`${label}: LEFT, miss ${scored.map((c) => `${c.name} ${c.miss.toFixed(3)}`).join(", ")}; solids ${picked.map((solid: any) => solid.id).join("+")}`);
        continue;
      }
      sections = scored[0]!.found;
      if (scored[0]!.name === "own") op.side = true;
      if (op.thin !== undefined) {
        const extent = (key: 0 | 1 | 2): number => {
          const all = (loftRings({ sections }) as V3[][]).flat().map((p) => p[key]!);
          return Math.max(...all) - Math.min(...all);
        };
        const factor = op.thin / extent(2);
        if (factor < 1) for (const section of sections) section[section.rotation === UPRIGHT ? "rx" : "ry"] = r5(section[section.rotation === UPRIGHT ? "rx" : "ry"] * factor);
      }
      if (op.slim !== undefined) {
        const key = Math.max(...sections.map((q) => q.rx)) <= Math.max(...sections.map((q) => q.ry)) ? "rx" : "ry";
        const factor = op.slim / (2 * Math.max(...sections.map((q) => q[key])));
        if (factor < 1) for (const section of sections) section[key] = r5(section[key] * factor);
      }
      console.log(`${label}: ${sections.length} sections ${scored[0]!.name}, miss ${scored.map((c) => `${c.name} ${c.miss.toFixed(3)}`).join(", ")}; solids ${picked.map((solid: any) => solid.id).join("+")}`);
    }
    if (op.side === true && main.kind !== "loft") put(main, { kind: "loft", sections: structuredClone(sections) });
    for (const [, shape] of targets) put(shape, { kind: "loft", sections: structuredClone(sections) });
    for (const view of VIEWS) for (const shape of list(op.scale, view)) if (shape.kind === "loft") delete shape.solid;
  } else throw new Error(`unknown op ${op.op}`);
  if (JSON.stringify(asset.representations[op.scale].geometry) !== before) changed++;
}
console.log(`${asset.id}: ${ops.length} ops, ${changed} changed something, ${refused} refused`);
if (flag("out") !== undefined) writeDoc(read, flag("out"));
else if (args.includes("--write")) console.log(writeDoc(read) ? "written" : "file unchanged");
