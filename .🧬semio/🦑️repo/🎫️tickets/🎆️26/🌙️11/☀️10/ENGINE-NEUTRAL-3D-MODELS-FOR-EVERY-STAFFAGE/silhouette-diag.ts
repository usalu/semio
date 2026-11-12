/** 🔬️ Diagnostic PNG of the library measure: grey both, blue authored only, red volume only, per view, plus the numbers.
 * Usage: bun silhouette-diag.ts <slug> [--pose id] [--scale 50] [--out name] [--views elevation,front,plan] [--px 460] */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { deflateSync } from "node:zlib";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, VIEWPOINTS, measureView, worldSolids, type Viewpoint } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { GENERATED, catalogEntries, loadAsset } from "./volume-lib.ts";

const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const slug = a[0]!;
const asset = loadAsset(slug.endsWith(".json") ? slug : catalogEntries().find((x) => x.slug === slug)!.path);
const pose = opt("--pose") ?? asset.presentation.defaultPose;
const scale = Number(opt("--scale") ?? 50);
const px = Number(opt("--px") ?? 460);
const views = (opt("--views")?.split(",") ?? VIEWPOINTS) as Viewpoint[];
const state = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const solids = worldSolids(asset, state.bones, scale);
const panels: { w: number; h: number; rgb: Uint8Array }[] = [];
for (const view of views) {
  const m = measureView(asset, state, solids, view, scale, px);
  const crop = opt("--crop")?.split(",").map(Number);
  const cb = crop && view === (opt("--view") ?? view) ? { minX: crop[0]!, minY: crop[1]!, maxX: crop[2]!, maxY: crop[3]! } : m.box;
  const cc = crop && view === (opt("--view") ?? view) ? Math.max(cb.maxX - cb.minX, cb.maxY - cb.minY) / px : m.cell;
  const A = new Mask(cb, cc), V = new Mask(cb, cc);
  for (const g of [...m.filled, ...m.outlines]) A.fill(g);
  for (const g of (m.section ? m.visible : m.reference)) V.fill(g);
  const rgb = new Uint8Array(A.width * A.height * 3);
  for (let j = 0; j < A.height; j++) for (let i = 0; i < A.width; i++) {
    const idx = j * A.width + i, o = ((A.height - 1 - j) * A.width + i) * 3;
    const t = A.cells[idx]!, v = V.cells[idx]!;
    const c = t && v ? [150, 150, 150] : t ? [40, 90, 230] : v ? [230, 50, 40] : [255, 255, 255];
    rgb[o] = c[0]!; rgb[o + 1] = c[1]!; rgb[o + 2] = c[2]!;
  }
  console.log(`${view}: IoU ${m.iou.toFixed(3)} recall ${m.recall.toFixed(3)} precision ${m.precision.toFixed(3)} cell ${(m.cell * 1000).toFixed(1)} mm, ${m.shapes} shapes (${m.openStrokes} open)`);
  panels.push({ w: A.width, h: A.height, rgb });
}
const W = panels.reduce((s, p) => s + p.w + 6, 0), H = Math.max(...panels.map((p) => p.h));
const img = new Uint8Array(W * H * 3).fill(255);
let x0 = 0;
for (const p of panels) { for (let j = 0; j < p.h; j++) img.set(p.rgb.subarray(j * p.w * 3, (j + 1) * p.w * 3), (j * W + x0) * 3); x0 += p.w + 6; }
const crcTable = Array.from({ length: 256 }, (_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
const crc = (buf: Uint8Array) => { let c = 0xffffffff; for (const b of buf) c = crcTable[(c ^ b) & 255]! ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
const chunk = (type: string, data: Uint8Array) => { const out = new Uint8Array(12 + data.length); const dv = new DataView(out.buffer); dv.setUint32(0, data.length); for (let i = 0; i < 4; i++) out[4 + i] = type.charCodeAt(i); out.set(data, 8); dv.setUint32(8 + data.length, crc(out.subarray(4, 8 + data.length))); return out; };
const raw = new Uint8Array(H * (W * 3 + 1));
for (let j = 0; j < H; j++) { raw[j * (W * 3 + 1)] = 0; raw.set(img.subarray(j * W * 3, (j + 1) * W * 3), j * (W * 3 + 1) + 1); }
const ihdr = new Uint8Array(13); const dv = new DataView(ihdr.buffer); dv.setUint32(0, W); dv.setUint32(4, H); ihdr[8] = 8; ihdr[9] = 2;
const out = join(GENERATED, "silhouette", `${opt("--out") ?? slug.replace(/[^a-z0-9-]/gi, "_")}.png`);
writeFileSync(out, Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", new Uint8Array(0))]));
console.log(out);
