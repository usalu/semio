/** ✏️ Ruler of author 12: draws the 2D shapes a tree's drawing plan names from the rest-pose projection of the body polyhedra of the same scale, in the product's own shape frames (probed through `evaluate`).
 *
 * Usage: `bun rework-12-draw2d.ts <candidate asset document> <plan json> <output asset document>`
 * The plan states per scale and view which solids one shape outlines, its bone, role, tone, region and the raster cell and tolerance in smallest features; the paint order is the depth order of the view.
 *
 * @see ./r3-tools.md
 * @see ./rework-12-kit.py
 */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";

const [docPath, planPath, outPath] = process.argv.slice(2);
if (outPath === undefined) {
  process.stderr.write("usage: rework-12-draw2d.ts <candidate asset document> <plan json> <output asset document>\n");
  process.exit(2);
}
const doc: Json = JSON.parse(readFileSync(docPath!, "utf8"));
const plan: Json = JSON.parse(readFileSync(planPath!, "utf8"));
const asset: Json = doc.asset;
const VIEWS: readonly View[] = ["elevation", "front", "plan"];
const frames = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const NEAR: Record<View, (p: readonly number[]) => number> = { elevation: (p) => -p[2]!, front: (p) => p[0]!, plan: (p) => p[1]! };

type Tri = { pts: Float64Array; depth: number };

/** 🌍 The projected triangles of the named body solids of a scale and their mean nearness to the viewer. */
function projected(scale: string, view: View, ids: readonly string[]): Tri {
  const body = new Map((asset.volumes[scale].body as Json[]).map((s) => [s.id, s]));
  const out: number[] = [];
  let [sum, count] = [0, 0];
  for (const id of ids) {
    const solid = body.get(id);
    if (solid === undefined) throw new Error(`plan names unknown solid "${id}" at 1:${scale}`);
    const f = frames[solid.bone]!.frame;
    const world = (solid.vertices as number[][]).map(([a, b, c]) => [0, 1, 2].map((k) => f.o[k]! + a! * f.u[k]! + b! * f.v[k]! + c! * f.w[k]!));
    for (const p of world) {
      sum += NEAR[view](p);
      count++;
    }
    for (const t of solid.triangles as number[][]) for (const i of t) {
      const q = projectWorld(view, world[i]! as [number, number, number]);
      out.push(q.x, q.y);
    }
  }
  return { pts: Float64Array.from(out), depth: count === 0 ? 0 : sum / count };
}

/** 🧵 The outline rings (outer counter-clockwise, holes clockwise) of the union of projected triangles on a raster of `cell`, closed by `close` cells. */
function outline(tris: Float64Array, cell: number, close: number): Ring[] {
  let [minX, minY, maxX, maxY] = [Infinity, Infinity, -Infinity, -Infinity];
  for (let i = 0; i < tris.length; i += 2) {
    minX = Math.min(minX, tris[i]!);
    maxX = Math.max(maxX, tris[i]!);
    minY = Math.min(minY, tris[i + 1]!);
    maxY = Math.max(maxY, tris[i + 1]!);
  }
  const pad = close + 2;
  minX -= pad * cell;
  minY -= pad * cell;
  const width = Math.ceil((maxX - minX) / cell) + pad;
  const height = Math.ceil((maxY - minY) / cell) + pad;
  let cells = new Uint8Array(width * height);
  for (let i = 0; i < tris.length; i += 6) {
    const [ax, ay, bx, by, cx, cy] = [0, 1, 2, 3, 4, 5].map((k) => (tris[i + k]! - (k % 2 === 0 ? minX : minY)) / cell);
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
  const grow = (from: Uint8Array, value: 0 | 1): Uint8Array => {
    const next = Uint8Array.from(from);
    for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
      const i = y * width + x;
      if (from[i] === value) continue;
      if ((x > 0 && from[i - 1] === value) || (x < width - 1 && from[i + 1] === value) || (y > 0 && from[i - width] === value) || (y < height - 1 && from[i + width] === value)) next[i] = value;
    }
    return next;
  };
  for (let s = 0; s < close; s++) cells = grow(cells, 1);
  for (let s = 0; s < close; s++) cells = grow(cells, 0);
  const W = width + 1;
  const next = new Map<number, number[]>();
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
        ring.push({ x: minX + (at % W) * cell, y: minY + Math.floor(at / W) * cell });
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

const signed = (r: Ring): number => r.reduce((n, p, i) => n + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🔭 The affine placement of the shape frame of every bone in every view of a scale, read from the product through probe shapes. */
function placements(scale: string): Record<View, Record<string, { to: (p: P2) => P2; det: number }>> {
  const bones = (asset.rig.bones as Json[]).map((b) => b.id as string);
  const probes = bones.map((bone) => ({ id: `probe-${bone}`, bone, role: "body", z: 1, filled: true, tone: 0.5, kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true }));
  const rep = asset.representations[scale];
  const probed = { ...asset, representations: { ...asset.representations, [scale]: { ...rep, geometry: { elevation: probes, front: probes, plan: probes } } } };
  const state = evaluate(probed as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never) as Json;
  const out = {} as Record<View, Record<string, { to: (p: P2) => P2; det: number }>>;
  for (const view of VIEWS) {
    out[view] = {};
    (state.shapes[view] as Json[]).forEach((s) => {
      const [o, a, b] = s.rings[0] as P2[];
      const [m00, m10, m01, m11] = [a!.x - o!.x, a!.y - o!.y, b!.x - o!.x, b!.y - o!.y];
      const det = m00 * m11 - m01 * m10;
      out[view][s.shape.bone] = { det, to: (p) => ({ x: (m11 * (p.x - o!.x) - m01 * (p.y - o!.y)) / det, y: (-m10 * (p.x - o!.x) + m00 * (p.y - o!.y)) / det }) };
    });
  }
  return out;
}

const PARENT = new Map((asset.rig.bones as Json[]).map((b) => [b.id as string, b.parent as string | null]));

/** 🧷 The bone a shape of `bone` is bound to in a view: the bone itself, or its nearest ancestor whose shape plane is not seen edge-on there (a hanging whip seen from the side has no drawable plane of its own). A plan with `elevationCarrierDet` (e.g. 0.999999) binds every elevation shape to the nearest ancestor that lies in the paper, as `lateral-ambiguous` asks for flat shapes. */
function carrier(place: Record<string, { to: (p: P2) => P2; det: number }>, bone: string, least = 0.35): string {
  let at: string | null = bone;
  while (at !== null && Math.abs(place[at]!.det) < least) at = PARENT.get(at) ?? null;
  return at ?? bone;
}

const pathOf = (rings: readonly Ring[]): string => rings.map((r) => `M ${r.map((p) => `${r4(p.x)} ${r4(p.y)}`).join(" L ")} Z`).join(" ");
let points = 0;
for (const scale of Object.keys(plan.shapes)) {
  const feature = (0.15 * Number(scale)) / 1000;
  const place = placements(scale);
  for (const view of VIEWS) {
    const drawn: { depth: number; shape: Json; order: number }[] = [];
    (plan.shapes[scale][view] as Json[]).forEach((spec, order) => {
      if (spec.raw !== undefined) {
        drawn.push({ depth: spec.depth ?? 0, shape: spec.raw, order });
        return;
      }
      const tri = projected(scale, view, spec.solids);
      const cell = feature * (spec.cellF ?? 0.25);
      const rings = outline(tri.pts, cell, spec.closeCells ?? 1).map((r) => simplifyRing(r, feature * (spec.tolF ?? 0.25))).filter((r) => r.length >= 3 && Math.abs(signed(r)) >= (spec.minAreaF2 ?? 1) * feature * feature);
      if (!rings.some((r) => signed(r) > 0)) return;
      if (place[view][spec.bone] === undefined) throw new Error(`no placement for bone ${spec.bone}`);
      const bone = carrier(place[view], spec.bone, view === "elevation" ? (plan.elevationCarrierDet ?? 0.35) : 0.35);
      const local = rings.map((r) => r.map(place[view][bone]!.to));
      points += local.reduce((n, r) => n + r.length, 0);
      const { solids, cellF, tolF, minAreaF2, closeCells, layer, ...meta } = spec;
      drawn.push({ depth: tri.depth + (layer ?? 0), shape: { ...meta, bone, kind: "path", d: pathOf(local) }, order });
    });
    drawn.sort((a, b) => a.depth - b.depth || a.order - b.order);
    asset.representations[scale].geometry[view] = drawn.map((d, i) => ({ ...d.shape, z: r4(1 + i * 0.01) }));
  }
}
for (const symbol of (plan.symbols ?? []) as Json[]) {
  const target = (asset.symbols as Json[]).find((s) => s.id === symbol.id);
  if (target === undefined) throw new Error(`unknown symbol ${symbol.id}`);
  for (const view of VIEWS) {
    target.geometry[view] = (symbol.geometry[view] as Json[]).map((spec) => {
      if (spec.solids === undefined) return spec;
      const tri = projected(String(symbol.scale), view, spec.solids);
      const rings = outline(tri.pts, symbol.cellM, 2).map((r) => simplifyRing(r, symbol.tolM)).filter((r) => signed(r) > 0).sort((a, b) => signed(b) - signed(a));
      const { solids, ...meta } = spec;
      return { ...meta, kind: "poly", points: rings[0]!.map((p) => [r4(p.x), r4(p.y)]), closed: true };
    });
  }
}
writeFileSync(outPath!, `${JSON.stringify(doc)}\n`);
process.stdout.write(`drawn ${points} points -> ${outPath}\n`);
