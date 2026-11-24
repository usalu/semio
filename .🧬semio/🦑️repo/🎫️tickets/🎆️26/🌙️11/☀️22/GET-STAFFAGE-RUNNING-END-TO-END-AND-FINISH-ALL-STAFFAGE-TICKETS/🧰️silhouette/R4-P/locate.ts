/** 🔎️ Where one view of a plant fails the largest boundary distance: every cluster of offending boundary cells with its place, its side (the drawn or the volume boundary that has no partner), the drawn shapes and the solids next to it, and optionally a PNG of the worst clusters (drawn-only red, volume-only blue, both grey).
 *  bun locate.ts <id|file.json> <scale> <view> [pngDir] [maxPng] */
import { deflateSync } from "node:zlib";
import { writeFileSync, mkdirSync } from "node:fs";
import { project } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { clusters, edgeDistance, inside, offences, scene, tileMasks, tiles, type Offence } from "./lib.ts";
import { fileOf } from "./plants.ts";

const [name, scaleArg, view, pngDir, maxPng] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const sc = scene(asset, Number(scaleArg), view!);
const limit = 2 * sc.feature;
const all: Offence[] = [];
let worst = 0;
for (const t of tiles(sc)) {
  const { m, drawn, vol, width, height } = tileMasks(sc, t);
  worst = Math.max(worst, m.hausdorffM);
  if (m.hausdorffM > limit) all.push(...offences(sc, t, drawn, vol, width, height, limit));
}
const groups = clusters(all, 6 * sc.feature);
console.log(`${asset.id} 1:${sc.scale} ${view}: largest distance ${(worst * 1000).toFixed(1)} mm against ${(limit * 1000).toFixed(1)} mm, cell ${(sc.cell * 1000).toFixed(2)} mm, ${all.length} offending cells in ${groups.length} clusters`);
const png = (file: string, cx: number, cy: number, r: number): void => {
  const box = { minX: cx - r, minY: cy - r, maxX: cx + r, maxY: cy + r };
  const cell = Math.max(sc.cell, (2 * r) / 900);
  const sub = { ...sc, cell };
  const t = { core: box, box, reference: sc.reference.filter((g) => g.some((ring) => ring.some((p) => p.x > box.minX - 1 && p.x < box.maxX + 1 && p.y > box.minY - 1 && p.y < box.maxY + 1))), evaluated: sc.evaluated };
  const { drawn, vol, width, height } = tileMasks(sub as never, t);
  const px = Math.max(1, Math.floor(900 / width));
  const [W, H] = [width * px, height * px];
  const raw = Buffer.alloc((W * 3 + 1) * H);
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
    const k = (height - 1 - Math.floor(y / px)) * width + Math.floor(x / px);
    const [a, b] = [drawn[k], vol[k]];
    const col = a && b ? [205, 205, 205] : a ? [235, 60, 60] : b ? [60, 90, 235] : [255, 255, 255];
    const o = y * (W * 3 + 1) + 1 + x * 3;
    raw[o] = col[0]!; raw[o + 1] = col[1]!; raw[o + 2] = col[2]!;
  }
  const crc = (buf: Buffer): number => { let c = ~0; for (const b of buf) { c ^= b; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
  const chunk = (tag: string, d: Buffer): Buffer => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(tag), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
  const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 2;
  writeFileSync(file, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]));
};
groups.slice(0, 40).forEach((g, n) => {
  const top = g[0]!;
  const [x0, x1, y0, y1] = [Math.min(...g.map((o) => o.x)), Math.max(...g.map((o) => o.x)), Math.min(...g.map((o) => o.y)), Math.max(...g.map((o) => o.y))];
  const near = sc.evaluated.map((s: any) => ({ id: s.shape.id as string, bone: s.shape.bone as string, filled: s.closed && s.shape.filled !== false, in: s.closed && inside(top, s.rings), d: edgeDistance(top, s.rings, s.closed) })).filter((s) => s.in || s.d < 4 * sc.feature).sort((a, b) => a.d - b.d).slice(0, 5);
  const solids = (sc.solids as any[]).map((w) => {
    let best = Infinity;
    for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]); best = Math.min(best, Math.hypot(p.x - top.x, p.y - top.y)); }
    return { id: w.id as string, bone: w.bone as string, d: best };
  }).sort((a, b) => a.d - b.d).slice(0, 4);
  console.log(`#${n} ${g.length} cells, worst ${(top.d * 1000).toFixed(1)} mm at (${top.x.toFixed(3)}, ${top.y.toFixed(3)}) on the ${top.side} boundary (${g.filter((o) => o.side === "drawn").length} drawn, ${g.filter((o) => o.side === "volume").length} volume cells), box x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)}`);
  console.log(`    shapes: ${near.map((s) => `${s.id}@${s.bone}${s.filled ? "" : " (stroke)"}${s.in ? " in" : ""} ${(s.d * 1000).toFixed(0)}mm`).join("; ")}`);
  console.log(`    solids: ${solids.map((s) => `${s.id}@${s.bone} ${(s.d * 1000).toFixed(0)}mm`).join("; ")}`);
  if (pngDir && n < Number(maxPng ?? 3)) { mkdirSync(pngDir, { recursive: true }); const r = Math.max(8 * sc.feature, (x1 - x0) / 2 + 4 * sc.feature, (y1 - y0) / 2 + 4 * sc.feature); const f = `${pngDir}/${asset.id}-${sc.scale}-${view}-${n}.png`; png(f, (x0 + x1) / 2, (y0 + y1) / 2, r); console.log(`    ${f} (half side ${r.toFixed(3)} m)`); }
});
