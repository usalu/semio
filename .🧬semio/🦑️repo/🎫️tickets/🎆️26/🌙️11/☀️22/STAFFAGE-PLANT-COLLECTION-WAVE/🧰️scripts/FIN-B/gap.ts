/** 🕳️ Finisher diagnosis of a silhouette gap: for one scale and view of the default pose, the drawn shapes whose points lie farthest outside the projected body and the body solids whose projected vertices lie farthest outside the drawing (distances in millimetres with the point), so a Hausdorff failure of the catalog-silhouette gate can be traced to a shape or a solid.
 *
 * `repo tools exec bun -- gap.ts <asset document json> <scale> <elevation|front|plan> [--top 8] [--at x,y] [--png <file> [--zoom x,y,half]]` (`--at` lists the solids and shapes whose view box holds the point, `--png` writes the overlay, `--zoom` only the square of `half` metres around the view point x, y)
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import sharp from "sharp";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureAsset, project, worldSolids, type Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

type P = { x: number; y: number };
const args = process.argv.slice(2);
const asset = JSON.parse(readFileSync(args[0]!, "utf8")).asset;
const [scale, view] = [Number(args[1]), args[2] as Viewpoint];
const top = args.includes("--top") ? Number(args[args.indexOf("--top") + 1]) : 8;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
const m = measureAsset(asset, scale, undefined, undefined, featureOf(scale as never)).views[view];
console.log(`[DEBUG] ${asset.id} 1:${scale} ${view}: iou ${m.iou.toFixed(4)} recall ${m.recall.toFixed(4)} precision ${m.precision.toFixed(4)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm p95 ${(m.p95M * 1000).toFixed(1)} mm (limit ${(2000 * featureOf(scale as never)).toFixed(0)} mm)`);
const inside = (p: P, rings: readonly (readonly P[])[]): boolean => {
  let odd = false;
  for (const ring of rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
  }
  return odd;
};
const far = (p: P, groups: readonly (readonly (readonly P[])[])[]): number => {
  if (groups.some((g) => inside(p, g))) return 0;
  let best = Infinity;
  for (const g of groups) for (const ring of g) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[j]!, ring[i]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
    best = Math.min(best, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
  }
  return best;
};
const body = m.reference as never as P[][][];
const drawing = [...(m.filled as never as P[][][]), ...(m.outlines as never as P[][][])];
const rank = (rows: { id: string; d: number; p: P }[]) => rows.filter((r) => r.d > 0.001).sort((a, b) => b.d - a.d).slice(0, top).map((r) => `${r.id} ${(r.d * 1000).toFixed(1)} mm at (${r.p.x.toFixed(3)}, ${r.p.y.toFixed(3)})`);
const shapes = (state.shapes[view] as never as { shape: { id: string; role: string }; rings: P[][] }[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role)).map((s) => {
  let worst = { id: `${s.shape.id} (${s.shape.role})`, d: 0, p: { x: 0, y: 0 } };
  for (const ring of s.rings) for (const p of ring) { const d = far(p, body); if (d > worst.d) worst = { ...worst, d, p }; }
  return worst;
});
console.log(`[DEBUG] drawn outside the body:\n  ${rank(shapes).join("\n  ") || "none"}`);
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true).map((w) => {
  let worst = { id: w.id, d: 0, p: { x: 0, y: 0 } };
  for (let i = 0; i < w.positions.length; i += 3) { const p = project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); const d = far(p, drawing); if (d > worst.d) worst = { ...worst, d, p }; }
  return worst;
});
console.log(`[DEBUG] body outside the drawing:\n  ${rank(solids).join("\n  ") || "none"}`);
const f = featureOf(scale as never);
const flat = [...body, ...drawing].flat(2);
const frame = { minX: Math.min(...flat.map((q) => q.x)) - 0.05, minY: Math.min(...flat.map((q) => q.y)) - 0.05, maxX: Math.max(...flat.map((q) => q.x)) + 0.05, maxY: Math.max(...flat.map((q) => q.y)) + 0.05 };
const cell = Math.max(f / 4, Math.max(frame.maxX - frame.minX, frame.maxY - frame.minY) / 3000);
const volume = new Mask(frame, cell);
for (const g of body) volume.fill(g as never, true);
volume.seal();
const drawnMask = new Mask(frame, cell);
for (const g of drawing) drawnMask.fill(g as never);
const [W, H] = [volume.width, volume.height];
const holes = Math.max(1, Math.round(((4 * f) / cell) ** 2));
const [ba, bb] = [boundaryOf(fillSmallHoles(drawnMask.cells, W, H, holes), W, H), boundaryOf(fillSmallHoles(volume.cells, W, H, holes), W, H)];
const spots: { who: string; d: number; x: number; y: number }[] = [];
for (const [who, from, to] of [["drawn boundary far from body boundary", ba, bb], ["body boundary far from drawn boundary", bb, ba]] as const) {
  const field = distanceField(to, W, H);
  const cand: { d: number; i: number; j: number }[] = [];
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (from[j * W + i] && field[j * W + i]! * cell * cell > (1.2 * f) ** 2) cand.push({ d: Math.sqrt(field[j * W + i]!) * cell, i, j });
  cand.sort((u, v) => v.d - u.d);
  const kept: typeof cand = [];
  for (const c of cand) if (kept.length < top && kept.every((k) => Math.hypot(k.i - c.i, k.j - c.j) * cell > 0.12)) kept.push(c);
  for (const c of kept) spots.push({ who, d: c.d, x: frame.minX + (c.i + 0.5) * cell, y: frame.minY + (c.j + 0.5) * cell });
}
console.log(`[DEBUG] raster ${W}x${H} cell ${(cell * 1000).toFixed(2)} mm, holes up to ${holes} cells filled:\n  ${spots.map((s) => `${s.who}: ${(s.d * 1000).toFixed(0)} mm at (${s.x.toFixed(3)}, ${s.y.toFixed(3)})`).join("\n  ") || "no boundary beyond 1.2 features"}`);
if (args.includes("--at")) {
  const [px, py] = args[args.indexOf("--at") + 1]!.split(",").map(Number) as [number, number];
  const near = 0.004;
  const box = (pts: P[]) => ({ x0: Math.min(...pts.map((q) => q.x)), x1: Math.max(...pts.map((q) => q.x)), y0: Math.min(...pts.map((q) => q.y)), y1: Math.max(...pts.map((q) => q.y)) });
  const holds = (b: { x0: number; x1: number; y0: number; y1: number }) => px >= b.x0 - near && px <= b.x1 + near && py >= b.y0 - near && py <= b.y1 + near;
  const size = (b: { x0: number; x1: number; y0: number; y1: number }) => `${(b.x1 - b.x0).toFixed(3)} x ${(b.y1 - b.y0).toFixed(3)}`;
  const solidsAt = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true).map((w) => ({ id: w.id, slender: w.slender === true, b: box(Array.from({ length: w.positions.length / 3 }, (_, i) => project(view, w.positions[3 * i]!, w.positions[3 * i + 1]!, w.positions[3 * i + 2]!))) })).filter((w) => holds(w.b));
  const shapesAt = (state.shapes[view] as never as { shape: { id: string; role: string; filled?: boolean; penMm?: number }; rings: P[][]; closed: boolean }[]).filter((s) => s.rings.length > 0).map((s) => ({ s, b: box(s.rings.flat()) })).filter((x) => holds(x.b));
  const inTriangle = (a: P, b: P, c: P): boolean => { const s = (u: P, v: P) => (v.x - u.x) * (py - u.y) - (v.y - u.y) * (px - u.x); const [d1, d2, d3] = [s(a, b), s(b, c), s(c, a)]; return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0)); };
  const covering = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true).filter((w) => { for (let i = 0; i < w.indices.length; i += 3) { const [a, b, c] = [0, 1, 2].map((k) => { const j = w.indices[i + k]! * 3; return project(view, w.positions[j]!, w.positions[j + 1]!, w.positions[j + 2]!); }) as [P, P, P]; if (inTriangle(a, b, c)) return true; } return false; }).map((w) => w.id);
  const drawnOver = shapesAt.filter((x) => inside({ x: px, y: py }, x.s.rings)).map((x) => x.s.shape.id);
  console.log(`[DEBUG] covering (${px}, ${py}): solids ${covering.join(", ") || "none"} | shapes ${drawnOver.join(", ") || "none"}`);
  console.log(`[DEBUG] at (${px}, ${py}):\n  solids: ${solidsAt.map((w) => `${w.id}${w.slender ? " (slender)" : ""} ${size(w.b)}`).join("; ") || "none"}\n  shapes: ${shapesAt.map((x) => `${x.s.shape.id} ${x.s.shape.role}${x.s.closed ? "" : " open"}${x.s.shape.filled === false ? " unfilled" : ""}${x.s.shape.penMm ? ` pen ${x.s.shape.penMm}` : ""} ${size(x.b)}`).join("; ") || "none"}`);
}
if (args.includes("--near") && spots.length > 0) {
  const s = spots[0]!, reach = Number(args[args.indexOf("--near") + 1]);
  const only: Record<string, string[]> = { body: [], drawing: [] };
  for (let j = 0; j < H; j += 2) for (let i = 0; i < W; i += 2) {
    const [x, y] = [frame.minX + (i + 0.5) * cell, frame.minY + (j + 0.5) * cell];
    if (Math.hypot(x - s.x, y - s.y) > reach) continue;
    const k = j * W + i;
    if (volume.cells[k] && !drawnMask.cells[k]) only.body!.push(`${x.toFixed(3)},${y.toFixed(3)}`);
    if (!volume.cells[k] && drawnMask.cells[k]) only.drawing!.push(`${x.toFixed(3)},${y.toFixed(3)}`);
  }
  console.log(`[DEBUG] within ${reach} m of the worst spot: body only ${only.body!.length} cells ${only.body!.filter((_, i) => i % Math.max(1, Math.floor(only.body!.length / 6)) === 0).slice(0, 6).join(" ")} | drawing only ${only.drawing!.length} cells ${only.drawing!.filter((_, i) => i % Math.max(1, Math.floor(only.drawing!.length / 6)) === 0).slice(0, 6).join(" ")}`);
}
const png = args.includes("--png") ? args[args.indexOf("--png") + 1] : undefined;
if (png !== undefined) {
  const flip = view !== "plan";
  const rgb = Buffer.alloc(W * H * 3, 255);
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) {
    const k = j * W + i, o = ((flip ? H - 1 - j : j) * W + i) * 3;
    const [d, v] = [drawnMask.cells[k]!, volume.cells[k]!];
    const c = d && v ? [205, 205, 205] : v ? [225, 40, 40] : d ? [40, 60, 230] : [255, 255, 255];
    rgb[o] = c[0]!; rgb[o + 1] = c[1]!; rgb[o + 2] = c[2]!;
  }
  const marks = spots.map((s) => `<circle cx="${((s.x - frame.minX) / cell).toFixed(0)}" cy="${(flip ? H - (s.y - frame.minY) / cell : (s.y - frame.minY) / cell).toFixed(0)}" r="${Math.max(14, 0.03 / cell).toFixed(0)}" fill="none" stroke="#0a0" stroke-width="3"/>`).join("");
  await sharp(rgb, { raw: { width: W, height: H, channels: 3 } }).composite([{ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}">${marks}</svg>`), left: 0, top: 0 }]).png().toBuffer().then((whole) => {
    const zoom = args.includes("--zoom") ? args[args.indexOf("--zoom") + 1]!.split(",").map(Number) : undefined;
    if (zoom === undefined) return sharp(whole).resize({ width: Math.min(W, 1800), height: Math.min(H, 1800), fit: "inside" }).png().toFile(png);
    const [cx, cy, half] = zoom as [number, number, number];
    const [i0, i1] = [Math.max(0, Math.round((cx - half - frame.minX) / cell)), Math.min(W, Math.round((cx + half - frame.minX) / cell))];
    const [ja, jb] = [Math.max(0, Math.round((cy - half - frame.minY) / cell)), Math.min(H, Math.round((cy + half - frame.minY) / cell))];
    const [topRow, height] = flip ? [H - jb, jb - ja] : [ja, jb - ja];
    return sharp(whole).extract({ left: i0, top: topRow, width: i1 - i0, height }).resize({ width: 1200, kernel: "nearest" }).png().toFile(png);
  });
  console.log(`[DEBUG] overlay ${png} (grey both, red body only, blue drawing only, green rings the worst boundaries)`);
}
