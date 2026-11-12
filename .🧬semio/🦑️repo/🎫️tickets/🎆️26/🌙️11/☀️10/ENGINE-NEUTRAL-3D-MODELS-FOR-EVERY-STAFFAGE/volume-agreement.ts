/** 🔬️ Silhouette agreement of every catalog volume in the presentation pose (`presentation.defaultPose`), open strokes stroked with their pen width.
 * Usage: `bun volume-agreement.ts <out.jsonl> [asset,asset] [--scale 50]`; the bone frames and the authored shapes both come from `evaluate` of the pose. */
import { appendFileSync, writeFileSync } from "node:fs";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, QUALITY_FOR, VIEWS, bboxOf, extentOf, loadAsset, projectWorld, toWorld, catalogEntries, type Box, type Frame, type P2, type Ring, type SolidMesh, type View } from "./volume-lib.ts";

const AUTHORED = new Set(["body", "detail", "glazing"]);
const [log, only] = process.argv.slice(2);
const scale = 50;

function strokeQuads(ring: readonly P2[], width: number): Ring[] {
  const out: Ring[] = [];
  const h = width / 2;
  for (let i = 0; i + 1 < ring.length; i++) {
    const a = ring[i]!;
    const b = ring[i + 1]!;
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const l = Math.hypot(dx, dy);
    if (l < 1e-12) {
      out.push([{ x: a.x - h, y: a.y - h }, { x: a.x + h, y: a.y - h }, { x: a.x + h, y: a.y + h }, { x: a.x - h, y: a.y + h }]);
      continue;
    }
    const nx = (-dy / l) * h;
    const ny = (dx / l) * h;
    const ex = (dx / l) * h;
    const ey = (dy / l) * h;
    out.push([{ x: a.x - ex + nx, y: a.y - ey + ny }, { x: b.x + ex + nx, y: b.y + ey + ny }, { x: b.x + ex - nx, y: b.y + ey - ny }, { x: a.x - ex - nx, y: a.y - ey - ny }]);
  }
  return out;
}

const v3 = (p: { x: number; y: number; z: number }): [number, number, number] => [p.x, p.y, p.z];

function analyse(asset: any, pose: string) {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } }) as any;
  const rep = asset.representations[String(scale)];
  const frames: Record<string, Frame> = {};
  for (const [id, b] of Object.entries<any>(state.bones)) frames[id] = { o: v3(b.frame.o), u: v3(b.frame.u), v: v3(b.frame.v), w: v3(b.frame.w) };
  const d = asset.dimensions;
  const dims = [d.widthM, d.heightM, d.depthM];
  const quality = QUALITY_FOR(Math.max(...dims));
  const list: SolidMesh[] = asset.volume.solids.filter((s: any) => s.maxDenominator === undefined || scale <= s.maxDenominator).map((solid: any) => {
    const f = frames[solid.bone]!;
    const g = tessellateSolid({ ...solid, node: solid.bone } as never, quality);
    const world = new Float64Array(g.positions.length);
    for (let i = 0; i < g.positions.length; i += 3) world.set(toWorld(f, [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!]), i);
    return { id: solid.id, role: solid.role, world, indices: Uint32Array.from(g.indices) };
  });
  const extent = extentOf(list)!;
  const bbox = [0, 1, 2].map((k) => extent.max[k]! - extent.min[k]!);
  const bboxError = bbox.map((b, k) => (b - dims[k]!) / dims[k]!);
  const views: Record<string, any> = {};
  for (const view of VIEWS) {
    const shapes = (state.shapes[view] as any[]).filter((s) => AUTHORED.has(s.shape.role) && s.rings.length > 0).map((s) => {
      const pen = ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) as number) * scale / 1000;
      const rings: Ring[] = s.closed ? (s.rings as Ring[]) : (s.rings as Ring[]).flatMap((r) => strokeQuads(r, Math.max(pen, 1e-4)));
      return { id: s.shape.id as string, rings, closed: s.closed as boolean };
    });
    const projected = list.map((m) => {
      const out = new Float64Array((m.world.length / 3) * 2);
      for (let i = 0; i < m.world.length / 3; i++) {
        const p = projectWorld(view as View, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!]);
        out[i * 2] = p.x;
        out[i * 2 + 1] = p.y;
      }
      return out;
    });
    const pts: P2[] = [...shapes.flatMap((s) => s.rings.flat()), ...projected.flatMap((xy) => Array.from({ length: xy.length / 2 }, (_, i) => ({ x: xy[i * 2]!, y: xy[i * 2 + 1]! })))];
    const b = bboxOf(pts);
    const cell = Math.max(1e-4, Math.max(b.maxX - b.minX, b.maxY - b.minY) / 640);
    const box: Box = { minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell };
    const truth = new Mask(box, cell);
    const volume = new Mask(box, cell);
    for (const s of shapes) truth.fill(s.rings);
    list.forEach((m, n) => {
      const xy = projected[n]!;
      for (let t = 0; t < m.indices.length; t += 3) volume.fill([[0, 1, 2].map((k) => ({ x: xy[m.indices[t + k]! * 2]!, y: xy[m.indices[t + k]! * 2 + 1]! }))]);
    });
    let both = 0, either = 0, a = 0, vv = 0;
    for (let i = 0; i < truth.cells.length; i++) {
      const x = truth.cells[i]!, y = volume.cells[i]!;
      a += x; vv += y; both += x & y; either += x | y;
    }
    const scratch = new Mask(box, cell);
    const stamp = new Int32Array(truth.cells.length).fill(-1);
    const worstShapes = shapes.map((s, n) => {
      let total = 0, covered = 0;
      scratch.fill(s.rings, (i) => { if (stamp[i] === n) return; stamp[i] = n; total++; covered += volume.cells[i]!; });
      return { id: s.id, closed: s.closed, missingM2: Number(((total - covered) * cell * cell).toFixed(5)), coverage: total === 0 ? 1 : Number((covered / total).toFixed(3)) };
    }).sort((p, q) => q.missingM2 - p.missingM2).slice(0, 4);
    views[view] = { iou: either === 0 ? 1 : both / either, recall: a === 0 ? 1 : both / a, precision: vv === 0 ? 1 : both / vv, drawnShapes: shapes.length, openStrokes: shapes.filter((s) => !s.closed).length, worstShapes };
  }
  return { bbox, bboxError, views };
}

writeFileSync(log!, "");
for (const e of catalogEntries()) {
  if (only && !only.split(",").includes(e.slug)) continue;
  const asset = loadAsset(e.path);
  const pose = asset.presentation.defaultPose;
  try {
    appendFileSync(log!, JSON.stringify({ asset: asset.id, category: e.category, pose, ...analyse(asset, pose) }) + "\n");
  } catch (error) {
    appendFileSync(log!, JSON.stringify({ asset: asset.id, category: e.category, pose, error: (error as Error).message }) + "\n");
  }
  console.error(asset.id);
}
