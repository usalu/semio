/** 🖍️ Author 29: rasterises the drawn 2D shapes (default pose) of the three views of one scale into one PNG (colour per bone, grid every 0.1 m) so the 2D can be read.
 * Usage: bun author-29-draw.ts <asset> <scale> [pxPerM=1800] */
import { openCtx } from "./kit-lib.ts";
import { deflateSync } from "node:zlib";
import { writeFileSync, mkdirSync } from "node:fs";
const [spec, scaleS, pxS] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
const ppm = Number(pxS ?? 1800);
type R = { x: number; y: number }[];
const views = ["elevation", "front", "plan"] as const;
const bb: Record<string, [number, number, number, number]> = {};
for (const v of views) { let b: [number, number, number, number] = [1e9, 1e9, -1e9, -1e9]; for (const s of c.state[v] as any[]) { if (!["body", "detail", "glazing"].includes(s.shape.role)) continue; for (const r of s.rings) for (const p of r) b = [Math.min(b[0], p.x), Math.min(b[1], p.y), Math.max(b[2], p.x), Math.max(b[3], p.y)]; } bb[v] = b; }
const pad = 0.02;
const dims = views.map((v) => [Math.ceil((bb[v]![2] - bb[v]![0] + 2 * pad) * ppm), Math.ceil((bb[v]![3] - bb[v]![1] + 2 * pad) * ppm)] as [number, number]);
const W = Math.max(...dims.map((d) => d[0])), H = dims.reduce((a, d) => a + d[1] + 6, 0);
const img = new Uint8Array(W * H * 3).fill(255);
const col = (id: string): [number, number, number] => { let h = 0; for (const ch of id) h = (h * 31 + ch.charCodeAt(0)) >>> 0; return [60 + (h % 150), 60 + ((h >> 8) % 150), 60 + ((h >> 16) % 150)]; };
let y0 = 0;
views.forEach((v, vi) => {
  const [w, h] = dims[vi]!;
  const px = (x: number) => Math.round((x - bb[v]![0] + pad) * ppm);
  const py = (y: number) => h - 1 - Math.round((y - bb[v]![1] + pad) * ppm);
  const set = (x: number, y: number, rgb: [number, number, number], a = 1) => { if (x < 0 || y < 0 || x >= w || y >= h) return; const i = ((y0 + y) * W + x) * 3; for (let k = 0; k < 3; k++) img[i + k] = Math.round(img[i + k]! * (1 - a) + rgb[k]! * a); };
  for (let g = Math.ceil(bb[v]![0] * 10) / 10; g <= bb[v]![2]; g += 0.1) for (let y = 0; y < h; y++) set(px(g), y, [200, 200, 255], 0.6);
  for (let g = Math.ceil(bb[v]![1] * 10) / 10; g <= bb[v]![3]; g += 0.1) for (let x = 0; x < w; x++) set(x, py(g), [200, 200, 255], 0.6);
  const list = (c.state[v] as any[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role)).sort((a, b) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  for (const s of list) {
    const rgb = col(s.shape.bone);
    const filled = s.closed && s.shape.filled !== false;
    for (const ring of s.rings as R) {
      if (filled) {
        const xs = ring.map((p) => px(p.x)), ys = ring.map((p) => py(p.y));
        const lo = Math.max(0, Math.min(...ys)), hi = Math.min(h - 1, Math.max(...ys));
        for (let y = lo; y <= hi; y++) { const xsx: number[] = []; for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) if ((ys[i]! <= y) !== (ys[k]! <= y)) xsx.push(xs[k]! + ((y - ys[k]!) / (ys[i]! - ys[k]!)) * (xs[i]! - xs[k]!)); xsx.sort((a, b) => a - b); for (let q = 0; q + 1 < xsx.length; q += 2) for (let x = Math.ceil(xsx[q]!); x <= xsx[q + 1]!; x++) set(x, y, rgb, 0.45); }
      }
      const n = ring.length;
      for (let i = 0; i < (s.closed ? n : n - 1); i++) { const a = ring[i]!, b = ring[(i + 1) % n]!; const steps = Math.max(1, Math.ceil(Math.hypot(px(b.x) - px(a.x), py(b.y) - py(a.y)))); for (let t = 0; t <= steps; t++) set(Math.round(px(a.x) + ((px(b.x) - px(a.x)) * t) / steps), Math.round(py(a.y) + ((py(b.y) - py(a.y)) * t) / steps), [rgb[0] * 0.5, rgb[1] * 0.5, rgb[2] * 0.5]); }
    }
  }
  y0 += h + 6;
});
const crc = (() => { const t = new Uint32Array(256); for (let n = 0; n < 256; n++) { let k = n; for (let i = 0; i < 8; i++) k = k & 1 ? 0xedb88320 ^ (k >>> 1) : k >>> 1; t[n] = k >>> 0; } return (b: Uint8Array) => { let k = 0xffffffff; for (const x of b) k = t[(k ^ x) & 255]! ^ (k >>> 8); return (k ^ 0xffffffff) >>> 0; }; })();
const chunk = (type: string, data: Uint8Array) => { const o = new Uint8Array(12 + data.length); const dv = new DataView(o.buffer); dv.setUint32(0, data.length); for (let i = 0; i < 4; i++) o[4 + i] = type.charCodeAt(i); o.set(data, 8); dv.setUint32(8 + data.length, crc(o.subarray(4, 8 + data.length))); return o; };
const raw = new Uint8Array((W * 3 + 1) * H); for (let y = 0; y < H; y++) { raw[y * (W * 3 + 1)] = 0; raw.set(img.subarray(y * W * 3, (y + 1) * W * 3), y * (W * 3 + 1) + 1); }
const ihdr = new Uint8Array(13); const dv = new DataView(ihdr.buffer); dv.setUint32(0, W); dv.setUint32(4, H); ihdr[8] = 8; ihdr[9] = 2;
const png = Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))]);
mkdirSync("🗑️generated/author-29", { recursive: true });
const out = `🗑️generated/author-29/draw-${spec!.replace("/", "-")}.s${scaleS}.png`;
writeFileSync(out, png);
console.log(out, W, H);
