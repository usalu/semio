/** 🧮️ The whole work list of one view instead of its worst place: on a fine raster the pieces the drawing has and the body lacks (`excess`) and the pieces the body has and the drawing lacks (`missing`), slivers thinner than `--thin` (default half a smallest feature) dropped, each with its box, its area in square features, the drawn shapes that cover it and the solids that cover it, largest first.
 *  bun diff.ts <id|file.json> <scale> <view> [--ops ops.json] [--thin metres] [--min squareFeatures] [--count N] */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, drawnFor, measureView, project, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

type P = { x: number; y: number };
const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" }, thin: { type: "string" }, min: { type: "string" }, count: { type: "string" } } });
const [name, sc, view] = positionals as [string, string, string];
const scale = Number(sc), feature = featureOf(scale);
const { asset } = await Bun.file(fileOf(name)).json();
if (values.ops !== undefined) (await import("./ops.ts")).applyOps(asset, JSON.parse(await Bun.file(values.ops).text()));
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const worlds = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, worlds, view as never, scale, undefined, feature, Infinity, false);
const long = Math.max(m.box.maxX - m.box.minX, m.box.maxY - m.box.minY);
const cell = Math.max(feature / 8, long / 6000);
const box = { minX: m.box.minX - 4 * cell, minY: m.box.minY - 4 * cell, maxX: m.box.maxX + 4 * cell, maxY: m.box.maxY + 4 * cell };
const drawn = new Mask(box, cell), vol = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
for (const g of m.reference) vol.fill(g, true);
vol.seal();
const [W, H] = [drawn.width, drawn.height];
const thin = Math.max(1, Math.round(Number(values.thin ?? feature / 2) / cell / 2));
const open = (src: Uint8Array): Uint8Array => {
  let cur = src;
  for (let pass = 0; pass < 3; pass++) for (let step = 0; step < thin; step++) {
    const out = new Uint8Array(W * H);
    for (let j = 1; j < H - 1; j++) for (let i = 1; i < W - 1; i++) {
      const k = j * W + i, around = cur[k - 1]! + cur[k + 1]! + cur[k - W]! + cur[k + W]!;
      out[k] = pass === 0 ? (cur[k] && around === 4 ? 1 : 0) : (cur[k] || around > 0) && src[k] ? 1 : 0;
    }
    cur = out;
  }
  return cur;
};
const inRing = (p: P, ring: P[]): boolean => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > p.y) !== (ring[j]!.y > p.y) && p.x < ((ring[j]!.x - ring[i]!.x) * (p.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c; return c; };
const shapesAt = (p: P): string[] => state.shapes[view].filter((s: any) => s.closed && s.rings.filter((r: P[]) => inRing(p, r)).length % 2 === 1).map((s: any) => `${s.shape.id}[${s.shape.bone}${s.shape.filled === false ? ",outline" : ""}]`);
const solidsAt = (p: P): string[] => {
  const out: string[] = [];
  for (const w of worlds) for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }); if (inRing(p, t)) { out.push(`${w.id}[${w.bone}${w.slender ? ",slender" : ""}]`); break; } }
  return out;
};
void drawnFor; void referenceOf;
const minCells = (Number(values.min ?? 1) * feature * feature) / (cell * cell);
for (const [kind, a, b] of [["excess", drawn.cells, vol.cells], ["missing", vol.cells, drawn.cells]] as const) {
  const raw = new Uint8Array(W * H);
  for (let k = 0; k < W * H; k++) raw[k] = a[k] && !b[k] ? 1 : 0;
  const kept = open(raw);
  const label = new Uint8Array(W * H);
  const pieces: { n: number; x0: number; y0: number; x1: number; y1: number; seed: number }[] = [];
  for (let s = 0; s < W * H; s++) if (kept[s] && !label[s]) {
    const piece = { n: 0, x0: Infinity, y0: Infinity, x1: -Infinity, y1: -Infinity, seed: s };
    const stack = [s];
    label[s] = 1;
    let best = -1;
    while (stack.length > 0) {
      const k = stack.pop()!, i = k % W, j = Math.floor(k / W);
      piece.n++; piece.x0 = Math.min(piece.x0, i); piece.x1 = Math.max(piece.x1, i); piece.y0 = Math.min(piece.y0, j); piece.y1 = Math.max(piece.y1, j);
      const depth = Math.min(...[1, 2, 3, 4].map((d) => (kept[k - d] && kept[k + d] && kept[k - d * W] && kept[k + d * W] ? d : 0)).map((d, q) => (d === 0 ? q : 5)));
      if (depth > best) { best = depth; piece.seed = k; }
      for (const n of [k - 1, k + 1, k - W, k + W]) if (kept[n] && !label[n]) { label[n] = 1; stack.push(n); }
    }
    if (piece.n >= minCells) pieces.push(piece);
  }
  pieces.sort((p, q) => q.n - p.n);
  console.log(`== ${asset.id} 1:${scale} ${view} ${kind}: ${pieces.length} pieces, cell ${(cell * 1000).toFixed(2)} mm`);
  for (const p of pieces.slice(0, Number(values.count ?? 40))) {
    const at = { x: box.minX + ((p.seed % W) + 0.5) * cell, y: box.minY + (Math.floor(p.seed / W) + 0.5) * cell };
    const f = (i: number, j: number): string => `${(box.minX + i * cell).toFixed(3)},${(box.minY + j * cell).toFixed(3)}`;
    console.log(`  ${((p.n * cell * cell) / (feature * feature)).toFixed(1)} sqf  box ${f(p.x0, p.y0)} .. ${f(p.x1 + 1, p.y1 + 1)}  at (${at.x.toFixed(3)}, ${at.y.toFixed(3)})  drawn: ${shapesAt(at).join(" ")} | vol: ${solidsAt(at).join(" ")}`);
  }
}
