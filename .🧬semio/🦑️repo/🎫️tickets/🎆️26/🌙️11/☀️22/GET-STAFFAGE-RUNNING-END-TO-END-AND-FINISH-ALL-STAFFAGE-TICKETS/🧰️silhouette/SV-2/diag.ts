/** 🔎️ Where one view of a plant disagrees with its volume as the guard rasterises it: the worst boundary clusters beyond the limit (side, place, distance), the drawn shapes and the body solids nearest to each, and optionally a PNG (drawn-only red, volume-only blue, both grey).
 *  bun diag.ts <slug|file.json> <scale> <view> [out.png] [x y radius] */
import { deflateSync } from "node:zlib";
import { writeFileSync } from "node:fs";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./measure-lib.ts";

const [name, scaleArg, view, out, xs, ys, rs] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale), limit = Number(process.env.LIMIT ?? 2 * feature);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const m: any = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
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
console.log(`${asset.id} 1:${scale} ${view} iou ${m.iou.toFixed(4)} H ${m.hausdorffM.toFixed(4)} limit ${limit} cell ${cell.toFixed(5)} box x ${box.minX.toFixed(3)}..${box.maxX.toFixed(3)} y ${box.minY.toFixed(3)}..${box.maxY.toFixed(3)}`);
type C = { side: string; x: number; y: number; d: number };
const clusters: C[] = [];
const grid = Number(process.env.GRID ?? Math.max(0.08, 6 * feature));
for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
  const list: { i: number; j: number; d: number }[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
  list.sort((a, b) => b.d - a.d);
  const seen = new Set<string>();
  for (const c of list) { const key = `${Math.round(c.i * cell / grid)},${Math.round(c.j * cell / grid)}`; if (seen.has(key)) continue; seen.add(key); clusters.push({ side, x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell, d: c.d }); }
}
clusters.sort((a, b) => b.d - a.d);
const segD = (p: any, a: any, b: any): number => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l)); return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy); };
const inside = (p: any, ring: any[]): boolean => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const a = ring[i], b = ring[j]; if ((a.y > p.y) !== (b.y > p.y) && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c; } return c; };
const shapesNear = (p: any): string => (state.shapes[view] as any[]).map((e) => { let d = Infinity, ins = false; for (const r of e.rings) { for (let i = 0; i < r.length - (e.closed ? 0 : 1); i++) d = Math.min(d, segD(p, r[i], r[(i + 1) % r.length])); if (e.closed && inside(p, r)) ins = !ins; } return { id: e.shape.id, bone: e.shape.bone, role: e.shape.role, filled: e.shape.filled, d, ins }; }).sort((a, b) => a.d - b.d).slice(0, 3).map((s) => `${s.id}[${s.bone},${s.role}${s.filled === false ? ",open" : ""}${s.ins ? ",in" : ""}] ${s.d.toFixed(3)}`).join("; ");
const solidsNear = (p: any): string => solids.map((s: any) => { let d = Infinity; for (let i = 0; i < s.positions.length; i += 3) { const q = project(view as never, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); d = Math.min(d, Math.hypot(q.x - p.x, q.y - p.y)); } return { id: s.id, bone: s.bone, d }; }).sort((a: any, b: any) => a.d - b.d).slice(0, 3).map((s: any) => `${s.id}[${s.bone}] ${s.d.toFixed(3)}`).join("; ");
for (const c of clusters.slice(0, Number(process.env.N ?? 12))) console.log(`  ${c.side} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${c.d.toFixed(4)}\n     shapes: ${shapesNear(c)}\n     solids: ${solidsNear(c)}`);
console.log(`${clusters.length} clusters beyond the limit`);
if (out !== undefined && out !== "-") {
  const win = xs === undefined ? { x0: box.minX, x1: box.maxX, y0: box.minY, y1: box.maxY } : { x0: Number(xs) - Number(rs), x1: Number(xs) + Number(rs), y0: Number(ys) - Number(rs), y1: Number(ys) + Number(rs) };
  const px = Math.max(1, Math.min(8, Math.round(900 / (Math.max(win.x1 - win.x0, win.y1 - win.y0) / cell))));
  const step = Math.max(1, Math.ceil(Math.max(win.x1 - win.x0, win.y1 - win.y0) / cell / 1400));
  const W = Math.round((win.x1 - win.x0) / cell / step) * px, H = Math.round((win.y1 - win.y0) / cell / step) * px;
  const raw = Buffer.alloc((W * 3 + 1) * H);
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
    const wx = win.x0 + (x / px + 0.5) * cell * step, wy = win.y1 - (y / px + 0.5) * cell * step;
    const i = Math.floor((wx - box.minX) / cell), j = Math.floor((wy - box.minY) / cell);
    const ok = i >= 0 && j >= 0 && i < w && j < h, k = j * w + i;
    const [a, b] = ok ? [drawn.cells[k], vol.cells[k]] : [0, 0];
    let col = a && b ? [205, 205, 205] : a ? [240, 110, 110] : b ? [110, 130, 240] : [255, 255, 255];
    if (step === 1) { if (ok && bd[k] && bv[k]) col = [60, 60, 60]; else if (ok && bd[k]) col = [190, 0, 0]; else if (ok && bv[k]) col = [0, 0, 190]; }
    const o = y * (W * 3 + 1) + 1 + x * 3;
    raw[o] = col[0]!; raw[o + 1] = col[1]!; raw[o + 2] = col[2]!;
  }
  const crc = (buf: Buffer): number => { let c = ~0; for (const b of buf) { c ^= b; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
  const chunk = (t: string, d: Buffer): Buffer => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
  const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 2;
  writeFileSync(out, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
  console.log(`${out} ${W}x${H} x ${win.x0.toFixed(3)}..${win.x1.toFixed(3)} y ${win.y0.toFixed(3)}..${win.y1.toFixed(3)}`);
}
