/** 🧭️ Where the guard measures its largest boundary distances: the worst mismatch clusters per view exactly as `measureAsset` finds them (one raster for a small object, tiles of 512 cells with a margin of six smallest features for a large one, enclosed holes of at most 16 square features filled per raster or tile), each with the nearest drawn shapes and volume solids; optional ops applied first; optional PNG of the raster or tile around a point (drawn-only red, volume-only blue, both grey, boundaries dark).
 *  bun where.ts <id|file.json> <scale> <view|all> [ops.json|-] [count] [png-out x y radius]   (PROBE="x,y;x,y" lists the drawn shapes and the solids that cover those points) */
import { deflateSync } from "node:zlib";
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, TILE_CELLS, VIEWPOINTS, WINDOW_FEATURES, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids, type Box } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

const [name, scaleArg, only, patch, cnt, pngOut, px, py, pr] = process.argv.slice(2);
const scale = Number(scaleArg);
const { asset } = (await Bun.file(fileOf(name!)).json()) as { asset: Asset };
if (patch && patch !== "-") (await import("./ops.ts")).applyOps(asset, JSON.parse(await Bun.file(patch).text()));
const feature = featureOf(scale);
const limit = 2 * feature;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const dseg = (P: any, a: any, b: any) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((P.x - a.x) * dx + (P.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(P.x - a.x - t * dx, P.y - a.y - t * dy); };
const near = (view: any, P: any, R: number): string => {
  const d: [number, string][] = [];
  for (const s of state.shapes[view]) { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(P, ring[i], ring[(i + 1) % ring.length])); if (m < R) d.push([m, `${s.shape.id}[${s.shape.bone}]${(m * 1000).toFixed(0)}`]); }
  const v: [number, string][] = [];
  for (const w of solids) { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [any, any, any]; m = Math.min(m, distanceToTriangle(P, t)); if (m === 0) break; } if (m < R) v.push([m, `${w.id}[${w.bone}]${(m * 1000).toFixed(0)}`]); }
  const top = (l: [number, string][]): string => l.sort((a, b) => a[0] - b[0]).slice(0, 6).map((x) => x[1]).join(" ");
  return `drawn: ${top(d)} | vol: ${top(v)}`;
};
const crc = (buf: Buffer): number => { let c = ~0; for (const b of buf) { c ^= b; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
const chunk = (t: string, d: Buffer): Buffer => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
const inRing = (P: any, ring: any[]): boolean => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i].y > P.y) !== (ring[j].y > P.y) && P.x < ((ring[j].x - ring[i].x) * (P.y - ring[i].y)) / (ring[j].y - ring[i].y) + ring[i].x) c = !c; return c; };
const covering = (view: any, P: any): string => {
  const d = state.shapes[view].filter((s: any) => s.closed && s.rings.filter((r: any) => inRing(P, r)).length % 2 === 1).map((s: any) => `${s.shape.id}[${s.shape.bone}${s.shape.filled === false ? ",outline" : ""},${s.shape.role}]`);
  const v: string[] = [];
  for (const w of solids) for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }); if (inRing(P, t)) { v.push(`${w.id}[${w.bone}${w.slender ? ",slender" : ""}]`); break; } }
  return `drawn: ${d.join(" ")} | vol: ${v.join(" ")}`;
};
for (const view of VIEWPOINTS) {
  if (only !== undefined && only !== "all" && only !== view) continue;
  for (const q of (process.env.PROBE ?? "").split(";").filter((x) => x !== "")) { const [x, y] = q.split(",").map(Number); console.log(`  at (${x}, ${y}) ${covering(view, { x, y })}`); }
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const { cell, box: frame } = m;
  const holes = Math.round((4 * feature / cell) ** 2);
  const banded = Math.max(frame.maxX - frame.minX, frame.maxY - frame.minY) / cell > TILE_CELLS;
  const side = TILE_CELLS * cell;
  const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
  const tiles: { core: Box; box: Box }[] = [];
  if (!banded) tiles.push({ core: frame, box: frame });
  else for (let j = 0; j < Math.ceil((frame.maxY - frame.minY) / side); j++) for (let i = 0; i < Math.ceil((frame.maxX - frame.minX) / side); i++) {
    const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
    const core: Box = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
    tiles.push({ core, box: { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin } });
  }
  const clusters: { side: string; x: number; y: number; d: number }[] = [];
  let worst = 0;
  for (const { core, box } of tiles) {
    const drawn = new Mask(box, cell), vol = new Mask(box, cell);
    for (const g of m.filled) drawn.fill(g);
    for (const g of m.outlines) drawn.fill(g);
    for (const g of m.reference) vol.fill(g, true);
    if (m.section) for (const g of m.visible) vol.fill(g, true);
    vol.seal();
    const [w, h] = [vol.width, vol.height];
    const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
    const cap = banded ? margin : Infinity;
    const [i0, i1, j0, j1] = banded ? [Math.round((core.minX - box.minX) / cell), Math.round((core.maxX - box.minX) / cell), Math.round((core.minY - box.minY) / cell), Math.round((core.maxY - box.minY) / cell)] : [0, w, 0, h];
    const anyD = bd.some((x) => x === 1), anyV = bv.some((x) => x === 1);
    const toVol = anyV ? distanceField(bv, w, h) : undefined, toDrawn = anyD ? distanceField(bd, w, h) : undefined;
    const list: { side: string; i: number; j: number; d: number }[] = [];
    for (const [sideName, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const)
      for (let j = j0; j < j1; j++) for (let i = i0; i < i1; i++) if (from[j * w + i]) {
        const d = Math.min(cap, field === undefined ? cap : Math.sqrt(field[j * w + i]!) * cell);
        if (d > worst) worst = d;
        if (d > limit * 0.8) list.push({ side: sideName, i, j, d });
      }
    list.sort((a, b) => b.d - a.d);
    const seen = new Set<string>();
    const grid = Math.max(0.1, 12 * feature);
    for (const c of list) {
      const [x, y] = [box.minX + (c.i + 0.5) * cell, box.minY + (c.j + 0.5) * cell];
      const key = `${c.side}:${Math.round(x / grid)}:${Math.round(y / grid)}`;
      if (seen.has(key)) continue;
      seen.add(key);
      clusters.push({ side: c.side, x: +x.toFixed(3), y: +y.toFixed(3), d: +c.d.toFixed(4) });
    }
    if (pngOut !== undefined && Number(px) >= core.minX && Number(px) < core.maxX && Number(py) >= core.minY && Number(py) < core.maxY) {
      const r = Number(pr ?? 0.1);
      const win = { x0: Math.max(box.minX, Number(px) - r), x1: Math.min(box.maxX, Number(px) + r), y0: Math.max(box.minY, Number(py) - r), y1: Math.min(box.maxY, Number(py) + r) };
      const k = Math.max(1, Math.min(10, Math.round(800 / (Math.max(win.x1 - win.x0, win.y1 - win.y0) / cell))));
      const W = Math.round((win.x1 - win.x0) / cell) * k, H = Math.round((win.y1 - win.y0) / cell) * k;
      const raw = Buffer.alloc((W * 3 + 1) * H);
      for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
        const i = Math.floor((win.x0 + (x / k + 0.5) * cell - box.minX) / cell), j = Math.floor((win.y1 - (y / k + 0.5) * cell - box.minY) / cell);
        const inside = i >= 0 && j >= 0 && i < w && j < h, q = j * w + i;
        const [a, b] = inside ? [drawn.cells[q], vol.cells[q]] : [0, 0];
        let col = a && b ? [205, 205, 205] : a ? [240, 130, 130] : b ? [130, 150, 240] : [255, 255, 255];
        if (inside && bd[q] && bv[q]) col = [60, 60, 60]; else if (inside && bd[q]) col = [190, 0, 0]; else if (inside && bv[q]) col = [0, 0, 190];
        const o = y * (W * 3 + 1) + 1 + x * 3;
        raw[o] = col[0]!; raw[o + 1] = col[1]!; raw[o + 2] = col[2]!;
      }
      const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 2;
      writeFileSync(pngOut, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
      console.log(`  png ${W}x${H} x ${win.x0.toFixed(3)}..${win.x1.toFixed(3)} y ${win.y0.toFixed(3)}..${win.y1.toFixed(3)}`);
    }
  }
  console.log(`== ${view} 1:${scale} iou ${m.iou.toFixed(4)} worst ${(worst * 1000).toFixed(1)} mm limit ${limit * 1000} mm cell ${(cell * 1000).toFixed(2)} mm ${banded ? `${tiles.length} tiles` : "one raster"}`);
  for (const c of clusters.sort((a, b) => b.d - a.d).slice(0, Number(cnt ?? 6))) console.log(`  ${c.side} (${c.x}, ${c.y}) ${(c.d * 1000).toFixed(1)}mm${c.d > limit ? "!" : ""}  ${near(view, c, Math.max(0.05, 4 * feature))}`);
}
