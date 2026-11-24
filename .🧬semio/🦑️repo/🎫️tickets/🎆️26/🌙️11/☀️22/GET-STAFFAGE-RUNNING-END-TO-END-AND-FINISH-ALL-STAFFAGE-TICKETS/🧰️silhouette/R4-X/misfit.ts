/** 🧮️ Which parts disagree in one view: per body solid the cells of what the guard compares it as (its projection, or its strokes when it is slender) that the drawing leaves empty, per drawn shape the cells it covers where no body is (one raster at the cell of the guard over the whole view or a window, no hole filling), largest first; optional ops applied first.
 *  bun misfit.ts <id|file.json> <scale> <view> [ops.json|-] [min cells] [x0,x1,y0,y1] */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, bodyOutline, comparisonCell, drawnFor, project, strokeDrawings, strokesOf, strokeQuad, worldSolids, type Box } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";
type P = { x: number; y: number };
const [name, sc, view, opsArg, minArg, winArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
const scale = Number(sc), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true);
const followable = strokeDrawings(view as never, solids as never, drawnFor(asset, state, view as never, scale));
const rep = asset.representations[sc!];
const boxOf = (groups: P[][][]): Box => { const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity }; for (const g of groups) for (const r of g) for (const p of r) { if (p.x < b.minX) b.minX = p.x; if (p.x > b.maxX) b.maxX = p.x; if (p.y < b.minY) b.minY = p.y; if (p.y > b.maxY) b.maxY = p.y; } return b; };
const parts: { kind: "vol" | "drawn"; label: string; groups: P[][][]; box: Box }[] = [];
for (const w of solids) {
  const groups: P[][][] = [];
  let label = `${w.id}[${w.bone}]`;
  if (w.slender === true) {
    const quads = strokesOf(view as never, w, followable) as P[][];
    for (const ring of quads) groups.push([ring]);
    label += `~ as ${(Math.min(...quads.map((q) => Math.min(Math.hypot(q[0]!.x - q[1]!.x, q[0]!.y - q[1]!.y), Math.hypot(q[1]!.x - q[2]!.x, q[1]!.y - q[2]!.y)))) * 1000).toFixed(0)} mm`;
  } else {
    const q: P[] = [];
    for (let i = 0; i < w.positions.length; i += 3) q.push(project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
    for (let t = 0; t < w.indices.length; t += 3) groups.push([[q[w.indices[t]]!, q[w.indices[t + 1]]!, q[w.indices[t + 2]]!]]);
  }
  parts.push({ kind: "vol", label, groups, box: boxOf(groups) });
}
for (const s of state.shapes[view!]) {
  if (s.rings.length === 0 || !["body", "detail", "glazing"].includes(s.shape.role)) continue;
  const fills = s.closed && (s.shape.filled !== false || bodyOutline(view as never, solids as never, s.rings));
  const groups: P[][][] = [];
  if (fills) groups.push(s.rings);
  else {
    const pen = Math.max(((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000, 1e-4);
    for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) groups.push([strokeQuad(r[i], r[i + 1], pen) as P[]]); }
  }
  parts.push({ kind: "drawn", label: `${s.shape.id}[${s.shape.bone}] ${s.shape.kind} ${s.shape.role}${fills ? (s.shape.filled === false ? " body-outline" : "") : s.closed ? " outline" : " stroke"}`, groups, box: boxOf(groups) });
}
const all = boxOf(parts.map((p) => [[{ x: p.box.minX, y: p.box.minY }, { x: p.box.maxX, y: p.box.maxY }]]));
const cell = comparisonCell(feature, Math.max(all.maxX - all.minX, all.maxY - all.minY));
const w4 = winArg === undefined ? undefined : winArg.split(",").map(Number);
const win: Box = w4 === undefined ? { minX: all.minX - 2 * cell, minY: all.minY - 2 * cell, maxX: all.maxX + 2 * cell, maxY: all.maxY + 2 * cell } : { minX: w4[0]!, maxX: w4[1]!, minY: w4[2]!, maxY: w4[3]! };
const hit = (b: Box): boolean => b.minX <= win.maxX && b.maxX >= win.minX && b.minY <= win.maxY && b.maxY >= win.minY;
const drawn = new Mask(win, cell), vol = new Mask(win, cell);
for (const p of parts) if (hit(p.box)) for (const g of p.groups) (p.kind === "vol" ? vol : drawn).fill(g, p.kind === "vol");
vol.seal();
let [both, either] = [0, 0];
for (let k = 0; k < drawn.cells.length; k++) { both += drawn.cells[k]! & vol.cells[k]!; either += drawn.cells[k]! | vol.cells[k]!; }
const out: { n: number; text: string }[] = [];
for (const p of parts) {
  if (!hit(p.box)) continue;
  const [i0, j0] = [Math.max(0, Math.floor((p.box.minX - win.minX) / cell) - 1), Math.max(0, Math.floor((p.box.minY - win.minY) / cell) - 1)];
  const [i1, j1] = [Math.min(drawn.width, Math.ceil((p.box.maxX - win.minX) / cell) + 1), Math.min(drawn.height, Math.ceil((p.box.maxY - win.minY) / cell) + 1)];
  if (i1 <= i0 || j1 <= j0) continue;
  const one = new Mask({ minX: win.minX + i0 * cell, minY: win.minY + j0 * cell, maxX: win.minX + i1 * cell - cell / 2, maxY: win.minY + j1 * cell - cell / 2 }, cell);
  for (const g of p.groups) one.fill(g);
  const other = p.kind === "vol" ? drawn : vol;
  let [n, of, sx, sy] = [0, 0, 0, 0];
  for (let j = 0; j < one.height; j++) for (let i = 0; i < one.width; i++) if (one.cells[j * one.width + i]) {
    of++;
    if (!other.cells[(j + j0) * other.width + i + i0]) { n++; sx += i + i0; sy += j + j0; }
  }
  if (n > 0) out.push({ n, text: `${p.kind === "vol" ? "vol-only  " : "drawn-only"} ${String(n).padStart(6)} of ${String(of).padStart(7)} at (${(win.minX + (sx / n + 0.5) * cell).toFixed(3)}, ${(win.minY + (sy / n + 0.5) * cell).toFixed(3)}) ${p.label}` });
}
console.log(`== ${asset.id} 1:${scale} ${view} cell ${(cell * 1000).toFixed(2)} mm window ${win.minX.toFixed(2)},${win.maxX.toFixed(2)},${win.minY.toFixed(2)},${win.maxY.toFixed(2)} iou ${(either === 0 ? 1 : both / either).toFixed(4)}`);
for (const o of out.sort((a, b) => b.n - a.n)) if (o.n >= Number(minArg ?? 8)) console.log(o.text);
