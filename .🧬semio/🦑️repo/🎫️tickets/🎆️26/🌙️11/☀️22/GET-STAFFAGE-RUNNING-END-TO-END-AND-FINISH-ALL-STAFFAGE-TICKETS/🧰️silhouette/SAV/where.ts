/** 🔎️ Where a drawing and its body disagree: for every failing scale and view of an asset the worst boundary mismatches (side, place in view coordinates, distance) with the nearest drawn shapes and volume solids, and the body and drawn extents per view.
 *  bun where.ts <id|file.json> [scale] [view] [--all]   (`--all` lists passing views too) */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, measureView, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

type P = { x: number; y: number };
const args = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const all = process.argv.includes("--all");
const [name, scaleArg, viewArg] = args as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const dseg = (p: P, a: P, b: P): number => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((p.x - a.x) * dx + (p.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy); };
const inRing = (p: P, r: readonly P[]): boolean => { let c = false; for (let i = 0, j = r.length - 1; i < r.length; j = i++) if ((r[i]!.y > p.y) !== (r[j]!.y > p.y) && p.x < ((r[j]!.x - r[i]!.x) * (p.y - r[i]!.y)) / (r[j]!.y - r[i]!.y) + r[i]!.x) c = !c; return c; };
for (const scale of [50, 100, 200, 500]) {
  if (scaleArg !== undefined && scaleArg !== "-" && Number(scaleArg) !== scale) continue;
  if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)].status !== "authored") continue;
  const feature = featureOf(scale), limit = 2 * feature;
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  for (const view of VIEWPOINTS) {
    if (viewArg !== undefined && viewArg !== view) continue;
    const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
    const failing = m.iou < 0.92 || m.hausdorffM > limit;
    if (!failing && !all) continue;
    const { cell, box } = m;
    const drawn = new Mask(box, cell), vol = new Mask(box, cell);
    for (const g of m.filled) drawn.fill(g);
    for (const g of m.outlines) drawn.fill(g);
    for (const g of m.reference) vol.fill(g, true);
    if (m.section) for (const g of m.visible) vol.fill(g, true);
    vol.seal();
    const [w, h] = [vol.width, vol.height];
    const holes = Math.round((4 * feature / cell) ** 2);
    const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
    const toVol = distanceField(bv, w, h), toDrawn = distanceField(bd, w, h);
    const ext = (cells: Uint8Array): string => { let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity; for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (cells[j * w + i]) { x0 = Math.min(x0, i); x1 = Math.max(x1, i + 1); y0 = Math.min(y0, j); y1 = Math.max(y1, j + 1); } return `x ${(box.minX + x0 * cell).toFixed(3)}..${(box.minX + x1 * cell).toFixed(3)} y ${(box.minY + y0 * cell).toFixed(3)}..${(box.minY + y1 * cell).toFixed(3)}`; };
    let dOnly = 0, vOnly = 0, both = 0;
    for (let k = 0; k < w * h; k++) { const a = drawn.cells[k], b = vol.cells[k]; if (a && b) both++; else if (a) dOnly++; else if (b) vOnly++; }
    console.log(`1:${scale} ${view}${m.section ? " (section)" : ""} iou ${m.iou.toFixed(4)} H ${(m.hausdorffM * 1000).toFixed(1)} mm (limit ${limit * 1000}) cell ${(cell * 1000).toFixed(2)} mm | drawn-only ${(dOnly * cell * cell * 1e4).toFixed(1)} cm2, volume-only ${(vOnly * cell * cell * 1e4).toFixed(1)} cm2, both ${(both * cell * cell * 1e4).toFixed(1)} cm2`);
    console.log(`   drawn ${ext(drawn.cells)} | volume ${ext(vol.cells)}`);
    const shapes = (state.shapes[view] as any[]).map((s) => ({ id: s.shape.id as string, bone: s.shape.bone as string, role: s.shape.role as string, filled: s.shape.filled, rings: s.rings as P[][] }));
    const vols = solids.map((s) => { const pts: P[] = []; for (let i = 0; i < s.positions.length; i += 3) pts.push(project(view, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!)); return { id: s.id, bone: s.bone, pts }; });
    const clusters: { side: string; x: number; y: number; d: number }[] = [];
    for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
      const list: { i: number; j: number; d: number }[] = [];
      for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > (m.hausdorffM > limit ? limit : 0.6 * limit)) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
      list.sort((a, b) => b.d - a.d);
      const seen = new Set<number>();
      const grid = Math.max(4 * feature, 0.04) / cell;
      for (const c of list) { const key = Math.round(c.i / grid) * 100000 + Math.round(c.j / grid); if (seen.has(key)) continue; seen.add(key); clusters.push({ side, x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell, d: c.d }); }
    }
    clusters.sort((a, b) => b.d - a.d);
    for (const c of clusters.slice(0, Number(process.env.N ?? 8))) {
      const p = { x: c.x, y: c.y };
      const near = shapes.map((s) => ({ s, d: Math.min(...s.rings.flatMap((r) => r.map((q, i) => dseg(p, q, r[(i + 1) % r.length]!)))), inside: s.rings.some((r) => inRing(p, r)) })).filter((x) => Number.isFinite(x.d)).sort((a, b) => a.d - b.d).slice(0, 4);
      const nearV = vols.map((s) => ({ s, d: Math.min(...s.pts.map((q) => Math.hypot(q.x - p.x, q.y - p.y))) })).sort((a, b) => a.d - b.d).slice(0, 4);
      console.log(`   ${c.side} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${(c.d * 1000).toFixed(1)} mm | shapes ${near.map((n) => `${n.s.id}@${n.s.bone}[${n.s.role}${n.s.filled === false ? ",open" : ""}] ${(n.d * 1000).toFixed(0)}${n.inside ? "in" : ""}`).join(", ")} | solids ${nearV.map((n) => `${n.s.id}@${n.s.bone} ${(n.d * 1000).toFixed(0)}`).join(", ")}`);
    }
  }
}
