/** 🩹️ Draws what the body has and the drawing lacks: in one view of one scale (default pose) the part of the guard's reference of the volume that no drawn shape covers is found on a fine raster, slivers thinner than `--thin` (default half a smallest feature) are dropped, and every remaining piece of at least `--min` square features (default 1) becomes a filled shape on the bone of the solid that covers most of it, in the tone and material of that solid, under the shapes that are already drawn, optionally widened by `--grow` metres where a seam of a raster cell would stay between it and its neighbours; the ops are merged into `ops/<id>.json` (earlier ops of that file are applied first).
 *  bun patch.ts <id> <scale> <view> [--thin metres] [--min squareFeatures] [--tol metres] [--grow metres] [--only solid,solid…] [--dry]
 *  A shape is named after its solid (`<solid>`, or `<solid>-part`, `<solid>-part-2`, … when that id is drawn already). */
import { parseArgs } from "node:util";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, drawnFor, measureView, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";

type P = { x: number; y: number };
const { values, positionals } = parseArgs({ allowPositionals: true, options: { thin: { type: "string" }, min: { type: "string" }, tol: { type: "string" }, only: { type: "string" }, grow: { type: "string" }, dry: { type: "boolean" } } });
const [name, sc, view] = positionals as [string, string, string];
const scale = Number(sc), feature = featureOf(scale);
const opsFile = `${import.meta.dir}/ops/${name}.json`;
const { asset } = (await Bun.file(fileOf(name)).json()) as { asset: Asset };
const existing = await readOps(opsFile);
applyOps(asset, existing);
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = asset.presentation.defaultPose;
const state: any = evaluate(asset, { kind: "pose", pose }, sel);
const worlds = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, worlds, view as never, scale, undefined, feature, Infinity, false);
const long = Math.max(m.box.maxX - m.box.minX, m.box.maxY - m.box.minY);
const cell = Math.max(feature / 8, long / 6000);
const box = { minX: m.box.minX - 4 * cell, minY: m.box.minY - 4 * cell, maxX: m.box.maxX + 4 * cell, maxY: m.box.maxY + 4 * cell };
const drawn = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const [W, H] = [drawn.width, drawn.height];
const drawing = drawnFor(asset, state, view as never, scale);
const only = values.only?.split(",");
const owner = new Int32Array(W * H).fill(-1);
const missing = new Uint8Array(W * H);
worlds.forEach((w, index) => {
  if (only !== undefined && !only.includes(w.id)) return;
  const mask = new Mask(box, cell);
  for (const g of referenceOf(view as never, [w], undefined, drawing).cut) mask.fill(g, true);
  mask.seal();
  for (let k = 0; k < W * H; k++) if (mask.cells[k] && !drawn.cells[k]) { missing[k] = 1; if (owner[k] < 0) owner[k] = index; }
});
const thin = Math.max(1, Math.round(Number(values.thin ?? feature / 2) / cell / 2));
const morph = (src: Uint8Array, grow: boolean, within?: Uint8Array): Uint8Array => {
  let cur = src;
  for (let step = 0; step < thin; step++) {
    const out = new Uint8Array(W * H);
    for (let j = 1; j < H - 1; j++) for (let i = 1; i < W - 1; i++) {
      const k = j * W + i;
      const around = cur[k - 1]! + cur[k + 1]! + cur[k - W]! + cur[k + W]!;
      out[k] = grow ? (cur[k] || around > 0 ? (within === undefined || within[k] ? 1 : 0) : 0) : cur[k] && around === 4 ? 1 : 0;
    }
    cur = out;
  }
  return cur;
};
const kept = morph(morph(morph(missing, false), true, missing), true, missing);
const label = new Int32Array(W * H).fill(-1);
const pieces: { cells: number[]; owners: Map<number, number> }[] = [];
for (let s = 0; s < W * H; s++) if (kept[s] && label[s] < 0) {
  const piece = { cells: [] as number[], owners: new Map<number, number>() };
  const stack = [s];
  label[s] = pieces.length;
  while (stack.length > 0) {
    const k = stack.pop()!;
    piece.cells.push(k);
    piece.owners.set(owner[k]!, (piece.owners.get(owner[k]!) ?? 0) + 1);
    for (const n of [k - 1, k + 1, k - W, k + W]) if (kept[n] && label[n]! < 0) { label[n] = pieces.length; stack.push(n); }
  }
  pieces.push(piece);
}
const minCells = (Number(values.min ?? 1) * feature * feature) / (cell * cell);
const cross = (p: P, q: P, r: P): number => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const dist = (p: P, q: P, r: P): number => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: P[], t: number): P[] => { if (l.length < 3) return l; let mi = 0, md = 0; for (let i = 1; i < l.length - 1; i++) { const d = dist(l[i]!, l[0]!, l[l.length - 1]!); if (d > md) { md = d; mi = i; } } return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0]!, l[l.length - 1]!]; };
const rdpRing = (r: P[], t: number): P[] => { let a = 0, b = 0; for (let i = 0; i < r.length; i++) { if (r[i]!.x < r[a]!.x) a = i; if (r[i]!.x > r[b]!.x) b = i; } if (a === b) return r; const rot = [...r.slice(a), ...r.slice(0, a)]; const k = rot.indexOf(r[b]!); return [...rdp(rot.slice(0, k + 1), t).slice(0, -1), ...rdp([...rot.slice(k), rot[0]!], t).slice(0, -1)]; };
const tol = Number(values.tol ?? Math.max(feature / 12, 1.5 * cell));
const grow = Number(values.grow ?? 0);
const offsetRing = (r: P[], d: number): P[] => d === 0 ? r : r.map((p, i) => { const a = r[(i + r.length - 1) % r.length]!, b = r[(i + 1) % r.length]!; const n1 = { x: p.y - a.y, y: a.x - p.x }, n2 = { x: b.y - p.y, y: p.x - b.x }; const l1 = Math.hypot(n1.x, n1.y) || 1, l2 = Math.hypot(n2.x, n2.y) || 1; const nx = n1.x / l1 + n2.x / l2, ny = n1.y / l1 + n2.y / l2; const nl = Math.hypot(nx, ny) || 1; return { x: p.x + (nx / nl) * d, y: p.y + (ny / nl) * d }; });
const outline = (index: number): P[] | undefined => {
  const at = (i: number, j: number): boolean => i >= 0 && j >= 0 && i < W && j < H && label[j * W + i] === index;
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (W + 1) + i;
  const add = (a: number, b: number): void => { const l = next.get(a); if (l) l.push(b); else next.set(a, [b]); };
  for (const k of pieces[index]!.cells) {
    const [i, j] = [k % W, Math.floor(k / W)];
    if (!at(i, j - 1)) add(key(i, j), key(i + 1, j));
    if (!at(i + 1, j)) add(key(i + 1, j), key(i + 1, j + 1));
    if (!at(i, j + 1)) add(key(i + 1, j + 1), key(i, j + 1));
    if (!at(i - 1, j)) add(key(i, j + 1), key(i, j));
  }
  let best: P[] | undefined, bestArea = 0;
  while (next.size > 0) {
    const [start] = next.keys();
    let cur = start!;
    const loop: number[] = [];
    for (;;) { loop.push(cur); const l = next.get(cur)!; const n = l.pop()!; if (l.length === 0) next.delete(cur); cur = n; if (cur === start || !next.has(cur)) break; }
    const ring = loop.map((k) => ({ x: box.minX + (k % (W + 1)) * cell, y: box.minY + Math.floor(k / (W + 1)) * cell }));
    let area = 0;
    for (let i = 0; i < ring.length; i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; area += p.x * q.y - q.x * p.y; }
    if (area > bestArea) [best, bestArea] = [offsetRing(rdpRing(ring, tol), grow), area];
  }
  return best;
};
const geo = (a: any): any[] => a.representations[sc].geometry[view];
const frames = new Map<string, ((p: P) => P) | null>();
const forward = (bone: string, points: P[]): P[] => {
  const probe: any = structuredClone(asset);
  geo(probe).push({ kind: "poly", points: points.map((p) => [p.x, p.y]), closed: true, id: "patch-probe", bone, role: "body", z: 1, filled: true, tone: 0.5 });
  const ps: any = evaluate(probe, { kind: "pose", pose }, sel);
  return ps.shapes[view].find((s: any) => s.shape.id === "patch-probe").rings[0].slice(0, points.length);
};
const local = (bone: string, targets: P[]): P[] | undefined => {
  const [o, a, b] = forward(bone, [{ x: 0, y: 0 }, { x: 0.05, y: 0 }, { x: 0, y: 0.05 }]) as [P, P, P];
  const f = { a: (a.x - o.x) / 0.05, b: (a.y - o.y) / 0.05, c: (b.x - o.x) / 0.05, d: (b.y - o.y) / 0.05 };
  const det = f.a * f.d - f.b * f.c;
  if (Math.abs(det) < 0.15) return undefined;
  const inv = (p: P): P => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (f.d * dx - f.c * dy) / det, y: (-f.b * dx + f.a * dy) / det }; };
  let points = targets.map(inv);
  for (let it = 0; it < 8; it++) {
    const got = forward(bone, points);
    const err = targets.map((t, i) => ({ x: t.x - got[i]!.x, y: t.y - got[i]!.y }));
    if (Math.max(...err.map((e) => Math.hypot(e.x, e.y))) < 1e-4) return points;
    points = points.map((l, i) => { const d = inv({ x: o.x + err[i]!.x, y: o.y + err[i]!.y }); return { x: l.x + d.x, y: l.y + d.y }; });
  }
  return undefined;
};
void frames;
const parents = new Map<string, string | null>((asset as any).rig.bones.map((b: any) => [b.id, b.parent]));
const solidsById = new Map<string, any>((asset as any).volumes[sc].body.map((s: any) => [s.id, s]));
const lowest = Math.min(...geo(asset).filter((s: any) => ["body", "detail", "glazing"].includes(s.role)).map((s: any) => s.z ?? 0));
const first = geo(asset)[0]?.id;
const regionOf = (bone: string): Record<string, unknown> => { const like = geo(asset).find((s: any) => s.bone === bone && (s.region !== undefined || s.regions !== undefined)) ?? geo(asset).find((s: any) => s.region !== undefined || s.regions !== undefined); return like === undefined ? {} : like.region !== undefined ? { region: like.region } : { regions: like.regions }; };
const digits = scale >= 200 ? 3 : 4;
const added: Op[] = [];
const used = new Set<string>(geo(asset).map((s: any) => s.id));
pieces.forEach((piece, index) => {
  if (piece.cells.length < minCells) return;
  const world = worlds[[...piece.owners].sort((p, q) => q[1] - p[1])[0]![0]]!;
  const ring = outline(index);
  if (ring === undefined || ring.length < 3) return;
  let bone: string | null = world.bone ?? null, points: P[] | undefined;
  while (bone !== null && (points = local(bone, ring)) === undefined) bone = parents.get(bone) ?? null;
  if (bone === null || points === undefined) { console.log(`  no drawing frame for ${world.id}`); return; }
  let id = world.id;
  for (let n = 1; used.has(id); n++) id = n === 1 ? `${world.id}-part` : `${world.id}-part-${n}`;
  used.add(id);
  const solid = solidsById.get(world.id) ?? {};
  const shape = { id, bone, role: "body", z: Number((lowest - 0.05).toFixed(3)), filled: true, tone: solid.tone ?? 0.5, ...regionOf(bone), ...(solid.material === undefined ? {} : { material: solid.material }), kind: "poly", points: points.map((p) => [Number(p.x.toFixed(digits)), Number(p.y.toFixed(digits))]), closed: true };
  added.push({ op: "shape-add", scale: sc, view, ...(first === undefined ? {} : { after: first }), shape });
  console.log(`  ${id}[${bone}] ${ring.length} points, ${((piece.cells.length * cell * cell) * 1e4).toFixed(1)} cm²`);
});
console.log(`${name} 1:${sc} ${view}: ${added.length} shapes${values.dry ? " (dry)" : ""}, cell ${(cell * 1000).toFixed(2)} mm`);
if (!values.dry && added.length > 0) await writeOps(opsFile, mergeOps(existing, added));
