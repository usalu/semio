/** 📐️ Ruling E1 helper: per bone, the orthographic outline of that bone's body solids in `presentation.defaultPose`, simplified for the scale (closing and opening by a quarter of the smallest feature, Douglas-Peucker at a quarter of the smallest feature or finer on small parts, nothing smaller than one feature squared unless it is part of a larger body; see `outline`), expressed in the coordinate frame of the 2D shapes of that view and printed as paste-ready shape JSON.
 *
 * The frame is not re-derived: the product's own `evaluate` is the forward map (a probe shape with the id, bone and members of the shape to be replaced is placed by it), and the outline is solved through that map (Newton steps with the probe's Jacobian), so depth twins, rest-to-pose carry and the elevation's 2D bone matrix are exactly the product's. Every bone is proven by a round trip: the printed shape is placed by `evaluate` again and compared with the bone's projected solids (IoU, and the largest distance of a placed point from its target).
 *
 * Usage: `bun r3-project2d.ts <category>/<slug> --scale N --view elevation|front|plan [--bone ID] [--merge] [--check] [--candidate] [--quiet]`
 * Output: stdout (per bone a header and one JSON shape per line) and `🗑️generated/r3-project2d/<category>/<slug>.s<N>.<view>.json`; `--check` replaces every filled closed body, detail and glazing shape of the view in memory by the outlines and prints the product's silhouette numbers; `--candidate` also writes that asset document to `🗑️generated/r3-project2d/<category>/<slug>.s<N>.<view>/🔣️.json` for `r3-2d.ts --file` (for looking, not for splicing).
 * @see ./r3-tools.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts */
import { loadFile } from "./rework-05-load.ts";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { Mask, VIEWS, area, featureM, pointInRing, poseOf, projectWorld, round5, simplifyRing, writeJson, type Box, type Json, type P2, type Ring, type SolidMesh, type View } from "./volumes-lib.ts";
import { cli, listsIn, load, r3Out, rigAt } from "./r3-lib.ts";

const USAGE = "r3-project2d.ts <category>/<slug> --scale N --view elevation|front|plan [--bone ID] [--merge] [--check] [--candidate] [--quiet]";
const DRAWN_ROLES = new Set(["body", "detail", "glazing"]);
const META = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];

/** 🔺️ The union of the projected triangles of meshes on a mask (every triangle, front or back, sets its cells). */
export function projectMask(view: View, list: readonly SolidMesh[], cell: number, marginCells: number, section?: number): Mask | null {
  const cut = { elevation: { axis: 2, retained: 1 }, front: { axis: 0, retained: -1 }, plan: { axis: 1, retained: -1 } }[view];
  const tris: number[] = [];
  for (const m of list) {
    const w = m.world;
    for (let t = 0; t < m.indices.length; t += 3) {
      const P = [0, 1, 2].map((k) => [w[m.indices[t + k]! * 3]!, w[m.indices[t + k]! * 3 + 1]!, w[m.indices[t + k]! * 3 + 2]!] as [number, number, number]);
      let poly = P;
      if (section !== undefined) {
        const d = P.map((p) => cut.retained * (p[cut.axis]! - section));
        poly = [];
        for (let i = 0; i < 3; i++) {
          const j = (i + 1) % 3;
          if (d[i]! >= 0) poly.push(P[i]!);
          if (d[i]! >= 0 !== d[j]! >= 0) {
            const u = d[i]! / (d[i]! - d[j]!);
            poly.push([P[i]![0] + u * (P[j]![0] - P[i]![0]), P[i]![1] + u * (P[j]![1] - P[i]![1]), P[i]![2] + u * (P[j]![2] - P[i]![2])]);
          }
        }
      }
      for (let k = 1; k + 1 < poly.length; k++) for (const q of [poly[0]!, poly[k]!, poly[k + 1]!]) {
        const p = projectWorld(view, q);
        tris.push(p.x, p.y);
      }
    }
  }
  if (tris.length === 0) return null;
  const box: Box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (let i = 0; i < tris.length; i += 2) {
    box.minX = Math.min(box.minX, tris[i]!);
    box.maxX = Math.max(box.maxX, tris[i]!);
    box.minY = Math.min(box.minY, tris[i + 1]!);
    box.maxY = Math.max(box.maxY, tris[i + 1]!);
  }
  const mask = new Mask({ minX: box.minX - marginCells * cell, minY: box.minY - marginCells * cell, maxX: box.maxX + marginCells * cell, maxY: box.maxY + marginCells * cell }, cell);
  const { width, height, cells } = mask;
  for (let i = 0; i < tris.length; i += 6) {
    const [ax, ay, bx, by, cx, cy] = [0, 1, 2, 3, 4, 5].map((k) => (tris[i + k]! - (k % 2 === 0 ? mask.box.minX : mask.box.minY)) / cell);
    const den = (by! - cy!) * (ax! - cx!) + (cx! - bx!) * (ay! - cy!);
    if (Math.abs(den) < 1e-12) continue;
    const [x0, x1] = [Math.max(0, Math.floor(Math.min(ax!, bx!, cx!) - 0.5)), Math.min(width - 1, Math.ceil(Math.max(ax!, bx!, cx!) - 0.5))];
    const [y0, y1] = [Math.max(0, Math.floor(Math.min(ay!, by!, cy!) - 0.5)), Math.min(height - 1, Math.ceil(Math.max(ay!, by!, cy!) - 0.5))];
    for (let y = y0; y <= y1; y++) for (let x = x0; x <= x1; x++) {
      const l0 = ((by! - cy!) * (x + 0.5 - cx!) + (cx! - bx!) * (y + 0.5 - cy!)) / den;
      const l1 = ((cy! - ay!) * (x + 0.5 - cx!) + (ax! - cx!) * (y + 0.5 - cy!)) / den;
      if (l0 >= -1e-9 && l1 >= -1e-9 && 1 - l0 - l1 >= -1e-9) cells[y * width + x] = 1;
    }
  }
  return mask;
}

/** 🫧️ Grows the cells of `value` by `steps` cells (alternating the 4- and the 8-neighbourhood, an octagon); outside the mask counts as empty. */
function grow(cells: Uint8Array, width: number, height: number, steps: number, value: 0 | 1): Uint8Array {
  const [W, H] = [width + 2, height + 2];
  let now = new Uint8Array(W * H);
  for (let y = 0; y < height; y++) now.set(cells.subarray(y * width, (y + 1) * width), (y + 1) * W + 1);
  for (let s = 0; s < steps; s++) {
    const next = Uint8Array.from(now);
    const diagonal = s % 2 === 1;
    for (let y = 1; y <= height; y++) {
      let i = y * W + 1;
      for (let x = 1; x <= width; x++, i++) {
        if (now[i] === value) continue;
        if (now[i - 1] === value || now[i + 1] === value || now[i - W] === value || now[i + W] === value || (diagonal && (now[i - W - 1] === value || now[i - W + 1] === value || now[i + W - 1] === value || now[i + W + 1] === value))) next[i] = value;
      }
    }
    now = next;
  }
  const out = new Uint8Array(width * height);
  for (let y = 0; y < height; y++) out.set(now.subarray((y + 1) * W + 1, (y + 1) * W + 1 + width), y * width);
  return out;
}

/** 🧵️ The boundary rings of a mask in paper metres: outer rings counter-clockwise (positive area), holes clockwise. */
export function contours(cells: Uint8Array, width: number, height: number, box: Box, cell: number): Ring[] {
  const next = new Map<number, number[]>();
  const W = width + 1;
  const on = (x: number, y: number): boolean => x >= 0 && y >= 0 && x < width && y < height && cells[y * width + x] === 1;
  const edge = (x0: number, y0: number, x1: number, y1: number) => {
    const k = y0 * W + x0;
    (next.get(k) ?? next.set(k, []).get(k)!).push(y1 * W + x1);
  };
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    if (!on(x, y)) continue;
    if (!on(x, y - 1)) edge(x, y, x + 1, y);
    if (!on(x + 1, y)) edge(x + 1, y, x + 1, y + 1);
    if (!on(x, y + 1)) edge(x + 1, y + 1, x, y + 1);
    if (!on(x - 1, y)) edge(x, y + 1, x, y);
  }
  const rings: Ring[] = [];
  for (const start of [...next.keys()]) {
    while ((next.get(start)?.length ?? 0) > 0) {
      const ring: Ring = [];
      let at = start;
      for (;;) {
        ring.push({ x: box.minX + (at % W) * cell, y: box.minY + Math.floor(at / W) * cell });
        const out = next.get(at);
        if (out === undefined || out.length === 0) break;
        at = out.pop()!;
        if (at === start) break;
      }
      if (ring.length >= 4) rings.push(ring);
    }
  }
  return rings;
}

type Group = { outer: Ring; holes: Ring[] };

/** 🏝️ The 8-connected islands of the set cells: `label[i]` is the island of cell i (-1 where empty), `sizes[k]` its number of cells. */
function islands(cells: Uint8Array, width: number, height: number): { label: Int32Array; sizes: number[] } {
  const label = new Int32Array(cells.length).fill(-1);
  const sizes: number[] = [];
  const stack: number[] = [];
  for (let start = 0; start < cells.length; start++) {
    if (cells[start] !== 1 || label[start] !== -1) continue;
    const k = sizes.length;
    let n = 0;
    label[start] = k;
    stack.push(start);
    while (stack.length > 0) {
      const i = stack.pop()!;
      n++;
      const [x, y] = [i % width, Math.floor(i / width)];
      for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) {
        const [u, v] = [x + dx, y + dy];
        if (u < 0 || v < 0 || u >= width || v >= height) continue;
        const j = v * width + u;
        if (cells[j] === 1 && label[j] === -1) {
          label[j] = k;
          stack.push(j);
        }
      }
    }
    sizes.push(n);
  }
  return { label, sizes };
}

/** 🧍️ The whole body of the view as the part test of {@link outline}: the union of all its solids, closed by `steps` cells so that touching parts are one island; `areaAt(p)` is the area of the island at a paper point (0 outside the body). */
function bodyIslands(view: View, list: readonly SolidMesh[], cell: number, steps: number, section?: number): (p: P2) => number {
  const mask = projectMask(view, list, cell, 2 * steps + 4, section);
  if (mask === null) return () => 0;
  const { width, height, box } = mask;
  const { label, sizes } = islands(grow(grow(mask.cells, width, height, steps, 1), width, height, steps, 0), width, height);
  return (p) => {
    const [x, y] = [Math.floor((p.x - box.minX) / cell), Math.floor((p.y - box.minY) / cell)];
    let most = 0;
    for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) {
      const [u, v] = [x + dx, y + dy];
      if (u >= 0 && v >= 0 && u < width && v < height && label[v * width + u]! >= 0) most = Math.max(most, sizes[label[v * width + u]!]!);
    }
    return most * cell * cell;
  };
}

/** 🎯️ The share of its traced area a simplified ring must keep (intersection over union); a ring that loses more at the given tolerance is simplified within half of it, and so on down to half a raster cell. */
const PART_IOU = 0.95;

/** ✂️ The clean outline groups of a mask: closing then opening by `steps` cells, traced, simplified within `tolerance` or finer where a part is so small that the tolerance costs more than {@link PART_IOU} allows or collapses it to a line, holes attached to their outer ring, largest first. The cleaning removes spurs and specks, never a part the product compares: a piece the opening took away is put back when it covers a quarter of `minArea` or more (a limb thinner than twice the opening is a part, not a spur), and an outer ring under `minArea` is dropped only when the island of the whole body it lies in (`bodyAreaAt`) is under `minArea` too; `kept` counts the outer rings kept by that second test. */
function outline(mask: Mask, steps: number, tolerance: number, minArea: number, bodyAreaAt: (p: P2) => number): { groups: Group[]; opened: boolean; kept: number } {
  const { width, height } = mask;
  const closed = grow(grow(mask.cells, width, height, steps, 1), width, height, steps, 0);
  let cells = grow(grow(closed, width, height, steps, 0), width, height, steps, 1);
  let opened = true;
  if (!cells.some((c) => c === 1)) {
    cells = closed;
    opened = false;
  } else {
    const lost = closed.map((c, i) => (c === 1 && cells[i] === 0 ? 1 : 0));
    const { label, sizes } = islands(lost, width, height);
    for (let i = 0; i < lost.length; i++) if (lost[i] === 1 && sizes[label[i]!]! * mask.cell * mask.cell >= minArea / 4) cells[i] = 1;
  }
  const signed = (r: Ring): number => r.reduce((n, p, i) => n + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;
  const inBody = (r: Ring): boolean => {
    if (signed(r) <= 0) return false;
    for (let i = 0; i < cells.length; i++) {
      if (cells[i] !== 1) continue;
      const p = { x: mask.box.minX + ((i % width) + 0.5) * mask.cell, y: mask.box.minY + (Math.floor(i / width) + 0.5) * mask.cell };
      if (pointInRing(p, r)) return bodyAreaAt(p) >= minArea;
    }
    return false;
  };
  let kept = 0;
  const fit = (r: Ring): Ring => {
    const traced = new Mask(mask.box, mask.cell);
    traced.fill([r]);
    for (let within = tolerance; ; within /= 2) {
      const s = simplifyRing(r, within);
      if (within < mask.cell / 2) return s;
      if (s.length > 4 && s.length >= r.length - 1) continue;
      const drawn = new Mask(mask.box, mask.cell);
      drawn.fill([s]);
      if (iou(traced, drawn) >= PART_IOU) return s;
    }
  };
  const rings = contours(cells, width, height, mask.box, mask.cell).map(fit).filter((r) => {
    if (r.length < 3) return false;
    if (Math.abs(area(r)) >= minArea) return true;
    if (!inBody(r)) return false;
    kept++;
    return true;
  });
  const groups: Group[] = rings.filter((r) => signed(r) > 0).sort((a, b) => signed(b) - signed(a)).map((outer) => ({ outer, holes: [] }));
  for (const hole of rings.filter((r) => signed(r) < 0)) groups.filter((g) => pointInRing(hole[0]!, g.outer)).sort((a, b) => signed(a.outer) - signed(b.outer))[0]?.holes.push(hole);
  if (groups.length === 0 && rings.length > 0) return { groups: [{ outer: [...rings[0]!].reverse(), holes: [] }], opened, kept };
  return { groups, opened, kept };
}

const metaOf = (shape: Json): Json => Object.fromEntries(META.filter((k) => shape[k] !== undefined).map((k) => [k, shape[k]]));
const queryOfPose = (asset: Json): Json => ({ kind: "pose", pose: poseOf(asset) });

function withView(asset: Json, scale: number, view: View, shapes: Json[]): Json {
  const rep = asset.representations[String(scale)];
  return { ...asset, representations: { ...asset.representations, [String(scale)]: { ...rep, geometry: { ...rep.geometry, [view]: shapes } } } };
}

/** ➡️ The product's placement of probe polygons: each `{ meta, points }` becomes a closed poly shape of the view and comes back as its ring on the paper of the view in the default pose. */
export function forward(asset: Json, scale: number, view: View, probes: readonly { meta: Json; points: readonly P2[] }[]): P2[][] {
  const shapes = probes.map((p) => ({ ...p.meta, kind: "poly", points: p.points.map((q) => [q.x, q.y]), closed: true }));
  const state = evaluate(withView(asset, scale, view, shapes) as never, queryOfPose(asset) as never, { kind: "scale", scale: { denominator: scale } } as never) as Json;
  return state.shapes[view].map((s: Json, i: number) => {
    const ring: P2[] = s.rings[0] ?? [];
    if (ring.length !== probes[i]!.points.length) throw new Error(`probe "${probes[i]!.meta.id}" came back with ${ring.length} of ${probes[i]!.points.length} points`);
    return ring;
  });
}

/** 🔁️ The shape-frame points whose placement by the product equals the target paper points; `residualM` is the largest remaining distance, `edgeOn` says that the shape plane of the bone is seen edge-on in the view. */
export function inverse(asset: Json, scale: number, view: View, jobs: readonly { meta: Json; target: readonly P2[] }[]): { points: P2[]; residualM: number; edgeOn: boolean }[] {
  const basis = forward(asset, scale, view, jobs.map((j) => ({ meta: j.meta, points: [{ x: 0, y: 0 }, { x: 1, y: 0 }, { x: 0, y: 1 }] })));
  const solve = jobs.map((j, k) => {
    const [o, a, b] = basis[k]!;
    const [m00, m10, m01, m11] = [a!.x - o!.x, a!.y - o!.y, b!.x - o!.x, b!.y - o!.y];
    const det = m00 * m11 - m01 * m10;
    return { edgeOn: Math.abs(det) < 0.02, apply: (dx: number, dy: number): P2 => (Math.abs(det) < 1e-9 ? { x: 0, y: 0 } : { x: (m11 * dx - m01 * dy) / det, y: (-m10 * dx + m00 * dy) / det }), o: o! };
  });
  let points = jobs.map((j, k) => j.target.map((t) => solve[k]!.apply(t.x - solve[k]!.o.x, t.y - solve[k]!.o.y)));
  let residual = jobs.map(() => Infinity);
  let best = points;
  const h = 1e-4;
  for (let step = 0; step < 16; step++) {
    const placed = forward(asset, scale, view, jobs.map((j, k) => ({ meta: j.meta, points: points[k]!.flatMap((p) => [p, { x: p.x + h, y: p.y }, { x: p.x, y: p.y + h }]) })));
    const now = jobs.map((j, k) => Math.max(...j.target.map((t, i) => Math.hypot(t.x - placed[k]![i * 3]!.x, t.y - placed[k]![i * 3]!.y))));
    if (Math.max(...now) < Math.max(...residual)) best = points;
    residual = residual.map((r, k) => Math.min(r, now[k]!));
    if (Math.max(...now) < 1e-7) break;
    points = points.map((ps, k) => ps.map((p, i) => {
      const [o, a, b] = [placed[k]![i * 3]!, placed[k]![i * 3 + 1]!, placed[k]![i * 3 + 2]!];
      const [dx, dy] = [jobs[k]!.target[i]!.x - o.x, jobs[k]!.target[i]!.y - o.y];
      const [m00, m10, m01, m11] = [(a.x - o.x) / h, (a.y - o.y) / h, (b.x - o.x) / h, (b.y - o.y) / h];
      const det = m00 * m11 - m01 * m10;
      const d = Math.abs(det) < 0.02 ? solve[k]!.apply(dx, dy) : { x: (m11 * dx - m01 * dy) / det, y: (-m10 * dx + m00 * dy) / det };
      return { x: p.x + d.x, y: p.y + d.y };
    }));
  }
  points = best;
  return jobs.map((_, k) => ({ points: points[k]!, residualM: residual[k]!, edgeOn: solve[k]!.edgeOn }));
}

const iou = (a: Mask, b: Mask): number => {
  let [both, any] = [0, 0];
  for (let i = 0; i < a.cells.length; i++) {
    if (a.cells[i] || b.cells[i]) any++;
    if (a.cells[i] && b.cells[i]) both++;
  }
  return any === 0 ? 1 : both / any;
};

if (import.meta.main) {
  const { spec, flag, has } = cli(["--scale", "--view", "--bone"]);
  const merge = has("--merge");
  const { entry, asset } = loadFile(spec, USAGE);
  const scale = Number(flag("--scale"));
  const view = flag("--view") as View;
  if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)]?.status !== "authored" || !VIEWS.includes(view)) {
    process.stderr.write(`scale ${flag("--scale")} must be authored (volumes source and representation) and --view one of ${VIEWS.join(", ")}\nusage: ${USAGE}\n`);
    process.exit(2);
  }
  const say = (line: string) => process.stdout.write(`${line}\n`);
  const f = featureM(scale, asset);
  const rep = asset.representations[String(scale)];
  const drawnShapes: Json[] = rep.geometry[view] ?? [];
  const rig = rigAt(asset, scale, queryOfPose(asset));
  const sm = listsIn(asset, scale, rig.frames);
  for (const e of sm.errors) process.stderr.write(`NOT PROJECTED ${e}\n`);
  const long = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
  const cell = Math.max(f / 16, long / 6000);
  const steps = Math.max(1, Math.round(f / 4 / cell));
  const section: number | undefined = rep.sections?.[view]?.plane;
  const placedNow: Json[] = (rig.state.shapes?.[view] ?? []) as Json[];
  const areaOfShape = (i: number): number => ((placedNow[i]?.rings ?? []) as Ring[]).reduce((n, r) => n + Math.abs(area(r)), 0);
  const fillable = (s: Json): boolean => DRAWN_ROLES.has(s.role) && s.kind !== "member" && s.filled !== false && !(s.kind === "poly" && s.closed === false);
  const generated = (s: Json): boolean => s.solid !== undefined && view !== "elevation";
  const keepsSolid = (s: Json): boolean => view === "elevation" && s.solid?.kind === "extruded" && s.solid.sweep === "w" && Math.abs(rig.frames[s.bone]?.w[2] ?? 0) > 0.9999;
  const bodyAreaAt = bodyIslands(view, sm.solids.body.flatMap((s, i) => (s.texture === true || s.slender === true ? [] : [sm.body[i]!])), cell, steps, section);
  const bones: string[] = (asset.rig.bones as Json[]).map((b) => b.id).filter((id) => flag("--bone") === undefined || id === flag("--bone"));
  type Entry = { bone: string; solids: string[]; slender: string[]; target: Json | null; targetIndex: number; raw: Mask; groups: Group[]; notes: string[]; shapes: Json[]; iou?: number; residualM?: number };
  const entries: Entry[] = [];
  const undrawnBones: string[] = [];
  for (const bone of bones) {
    const index = sm.solids.body.map((s, i) => ({ s, i })).filter(({ s }) => s.bone === bone && s.texture !== true);
    const solidAll = index.filter(({ s }) => s.slender !== true);
    const candidates = drawnShapes.map((s, i) => ({ s, i })).filter(({ s }) => s.bone === bone && fillable(s) && !generated(s)).sort((a, b) => Number(b.s.role === "body") - Number(a.s.role === "body") || areaOfShape(b.i) - areaOfShape(a.i));
    if (solidAll.length === 0) {
      if (candidates.length > 0) undrawnBones.push(`${bone} (drawn: ${candidates.map((c) => c.s.id).join(", ")})`);
      continue;
    }
    const byId = new Map(candidates.map((c) => [c.s.id as string, c]));
    const sets = new Map<number, typeof solidAll>();
    for (const x of solidAll) {
      const k = merge ? (candidates[0]?.i ?? -1) : (byId.get(x.s.id)?.i ?? candidates[0]?.i ?? -1);
      (sets.get(k) ?? sets.set(k, []).get(k)!).push(x);
    }
    const used = new Set(sets.keys());
    const unused = candidates.filter((c) => !used.has(c.i));
    let first = true;
    for (const [k, solid] of sets) {
      const raw = projectMask(view, solid.map(({ i }) => sm.body[i]!), cell, 2 * steps + 4, section);
      if (raw === null) continue;
      const { groups, opened, kept } = outline(raw, steps, f / 4, f * f, bodyAreaAt);
      const notes: string[] = [];
      if (!opened) notes.push("thinner than half a feature in this view: outline kept without the sliver removal; consider a stroke instead of a filled shape");
      if (groups.length === 0) notes.push("smaller than one feature squared in this view and not part of a larger body: nothing to draw as a filled shape");
      if (kept > 0) notes.push(`${kept} outline(s) under one feature squared kept: part of the body silhouette the product compares`);
      const target = k < 0 ? null : drawnShapes[k]!;
      if (target === null) notes.push("no filled closed body, detail or glazing shape of this bone in this view: the shape below is new (choose id, z, tone, region; its frame is that of a shape without an elevation twin)");
      else {
        if (target.solid !== undefined && keepsSolid(target)) notes.push(`"${target.id}" is the profile of an extruded \`solid\` (kept in the shape below): its \`z0\`, \`z1\`, \`zTip\` still give the OLD lateral span, set them to the new body; the front and plan of this shape follow from them`);
        if (target.solid !== undefined && !keepsSolid(target)) notes.push(`"${target.id}" carries a \`solid\` (${target.solid.kind}${target.solid.sweep === undefined ? "" : `, sweep ${target.solid.sweep}`}) whose profile is not this outline point by point (revolved, swept along v, or the bone leaves the paper plane): REFERENCE ONLY in the flat frame, the shape below has no \`solid\`; redraw the profile by hand and keep \`solid\``);
        if (target.followTip === true) notes.push(`"${target.id}" has \`followTip\`: the outline is exact for the default pose only; the pasted poly drops \`followTip\``);
      }
      const made = drawnShapes.filter((x) => x.bone === bone && fillable(x) && generated(x));
      if (first && made.length > 0) notes.push(`this bone is also drawn here by \`solid\` shapes (${made.map((x) => x.id).join(", ")}): the product generates them in this view from their elevation profile and lateral span, they cannot be pasted over; correct the elevation profile and \`z0\`/\`z1\`, the outline below is for comparison${target === null ? " (it is NOT a missing shape)" : ""}`);
      if (first && unused.length > 0) notes.push(`filled shapes of this bone in this view without a solid of their id (redraw, keep or remove them yourself): ${unused.map((c) => c.s.id).join(", ")}`);
      if (first && index.length > solidAll.length) notes.push(`slender solids left out (drawn as strokes): ${index.filter(({ s }) => s.slender === true).map(({ s }) => s.id).join(", ")}`);
      entries.push({ bone, solids: solid.map(({ s }) => s.id), slender: first ? index.filter(({ s }) => s.slender === true).map(({ s }) => s.id) : [], target, targetIndex: k, raw, groups, notes, shapes: [] });
      first = false;
    }
  }
  const jobs = entries.flatMap((e) => e.groups.flatMap((g, gi) => [g.outer, ...g.holes].map((ring, ri) => ({ entry: e, gi, ri, meta: e.target === null ? { id: `r3-probe-${e.bone}`, bone: e.bone, role: "body", z: 1, filled: true, tone: 0.5 } : metaOf(e.target), target: ring }))));
  const solved = jobs.length === 0 ? [] : inverse(asset, scale, view, jobs);
  for (const e of entries) {
    const mine = jobs.map((j, k) => ({ j, r: solved[k]! })).filter(({ j }) => j.entry === e);
    if (mine.length === 0) continue;
    e.residualM = Math.max(...mine.map(({ r }) => r.residualM));
    if (e.residualM > f / 20) e.notes.push(`the frame was not solved exactly (largest distance of a placed point from its target ${(e.residualM * 1000).toFixed(2)} mm): check this bone in the overlay or draw it by hand`);
    if (mine.some(({ r }) => r.edgeOn)) e.notes.push("the shape plane of this bone is seen nearly edge-on in this view in the default pose: coordinates are ill-conditioned, draw this bone by hand");
    const meta = e.target === null ? { id: `r3-new-${e.bone}`, bone: e.bone, role: "body", z: 1, filled: true, tone: 0.5 } : { ...metaOf(e.target), ...(keepsSolid(e.target) ? { solid: e.target.solid } : {}) };
    e.groups.forEach((g, gi) => {
      const rings = mine.filter(({ j }) => j.gi === gi).sort((a, b) => a.j.ri - b.j.ri).map(({ r }) => r.points.map((p) => [round5(p.x), round5(p.y)]));
      const id = gi === 0 ? meta.id : `${meta.id}-part-${gi + 1}`;
      e.shapes.push(rings.length === 1 ? { ...meta, id, kind: "poly", points: rings[0], closed: true } : { ...meta, id, kind: "path", d: rings.map((r) => `${r.map((p, i) => `${i === 0 ? "M" : "L"} ${p[0]} ${p[1]}`).join(" ")} Z`).join(" ") });
    });
    if (e.groups.some((g) => g.holes.length > 0)) e.notes.push("the outline has holes: printed as one `path` with one subpath per ring (several rings of one shape fill even-odd)");
  }
  const withShapes = entries.filter((e) => e.shapes.length > 0);
  if (withShapes.length > 0) {
    const state = evaluate(withView(asset, scale, view, withShapes.flatMap((e) => e.shapes)) as never, queryOfPose(asset) as never, { kind: "scale", scale: { denominator: scale } } as never) as Json;
    let at = 0;
    for (const e of withShapes) {
      const back = new Mask(e.raw.box, e.raw.cell);
      for (const _ of e.shapes) back.fill(state.shapes[view][at++].rings as Ring[]);
      e.iou = iou(e.raw, back);
    }
  }
  say(`# ${entry.category}/${entry.slug} 1:${scale} ${view} pose ${poseOf(asset)}: feature ${(f * 1000).toFixed(1)} mm, simplified within ${((f / 4) * 1000).toFixed(2)} mm, raster ${(cell * 1000).toFixed(2)} mm${section === undefined ? "" : `, section at ${section} m (C12)`}`);
  say(`# frame: ${view === "elevation" ? "bone-local (x along the bone from its joint, y across it), placed by the bone's 2D matrix" : view === "front" ? "x = lateral (world z), y = height, of the REST pose with the root at its authored place; carried to the pose by the bone" : "x = ahead, y = lateral (world z), of the REST pose; carried to the pose by the bone"}. Replace the geometry members of the named shape (kind, points/d/x/y/..., closed) by those printed; keep the order of the list.`);
  for (const e of entries) {
    say(`\n## bone ${e.bone}: solids ${e.solids.join(", ")} -> ${e.target === null ? "NEW shape" : `over shape "${e.target.id}" (${e.target.kind}, index ${e.targetIndex} of geometry.${view})`}; ${e.shapes.length} outline(s), ${e.shapes.map((s) => (s.kind === "poly" ? `${s.points.length} points` : "path with holes")).join(" + ") || "none"}; round trip IoU ${e.iou === undefined ? "-" : e.iou.toFixed(4)}, frame residual ${e.residualM === undefined ? "-" : `${(e.residualM * 1000).toFixed(3)} mm`}`);
    for (const n of e.notes) say(`#   note: ${n}`);
    if (!has("--quiet")) for (const s of e.shapes) say(JSON.stringify(s));
  }
  if (undrawnBones.length > 0) say(`\n# bones with filled 2D shapes in this view but no body solid of their own at this scale (their form is in another bone's solids; redraw or remove by hand): ${undrawnBones.join("; ")}`);
  const replaced = new Set(entries.filter((e) => e.shapes.length > 0 && e.targetIndex >= 0).map((e) => e.targetIndex));
  const candidate: Json[] = [];
  drawnShapes.forEach((s, i) => {
    const e = entries.find((x) => x.targetIndex === i && x.shapes.length > 0);
    if (e !== undefined) candidate.push(...e.shapes);
    else if (!fillable(s) || generated(s) || flag("--bone") !== undefined) candidate.push(s);
  });
  for (const e of entries) if (e.targetIndex < 0 && !drawnShapes.some((x) => x.bone === e.bone && fillable(x) && generated(x))) candidate.push(...e.shapes);
  const out = r3Out("r3-project2d", entry, `${entry.slug}.s${scale}.${view}${flag("--bone") === undefined ? "" : `.${flag("--bone")}`}.json`);
  writeJson(out, { asset: `${entry.category}/${entry.slug}`, scale, view, pose: poseOf(asset), featureM: f, toleranceM: f / 4, bones: entries.map((e) => ({ bone: e.bone, solids: e.solids, slenderLeftOut: e.slender, replaces: e.target?.id ?? null, index: e.targetIndex, roundTripIou: e.iou ?? null, frameResidualM: e.residualM ?? null, notes: e.notes, shapes: e.shapes })) }, true);
  say(`\n# written ${out}`);
  if (has("--check") || has("--candidate")) {
    const doc = withView(asset, scale, view, candidate);
    if (has("--check")) {
      try {
        const ms = (measureVolumes(doc as never, { scale: scale as never, rules: ["volumes-silhouette"] as never }) as readonly Json[]).filter((m) => String(m.name).endsWith(`:${view}`));
        say(`# check (all ${replaced.size} replaced, every other filled closed body/detail/glazing shape of the view removed, strokes kept): ${ms.map((m) => `${m.name}=${Number(m.value).toFixed(4)}${m.ok === false ? "!" : ""}`).join("  ")}`);
      } catch (error) {
        say(`# check failed: ${(error as Error).message}`);
      }
    }
    if (has("--candidate")) {
      const path = r3Out("r3-project2d", entry, `${entry.slug}.s${scale}.${view}/🔣️.json`);
      mkdirSync(dirname(path), { recursive: true });
      const file = JSON.parse(readFileSync(process.env.R05_FILE!, "utf8"));
      writeFileSync(path, `${JSON.stringify({ ...file, asset: { ...file.asset, representations: doc.representations } })}\n`);
      say(`# candidate document (for r3-2d.ts --file, not for splicing) ${path}`);
    }
  }
}
