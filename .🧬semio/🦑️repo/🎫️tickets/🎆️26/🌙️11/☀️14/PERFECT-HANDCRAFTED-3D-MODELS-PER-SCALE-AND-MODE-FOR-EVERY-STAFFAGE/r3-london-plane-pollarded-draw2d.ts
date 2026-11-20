/** 🖍️ Round 3 london plane: draws the 2D shapes of one scale (three views, default pose) into one PNG, fills by tone, outlines and strokes dark, optional per-bone colours. Usage: bun r3-london-plane-pollarded-draw2d.ts <scale> [--bones] [--file asset.json] */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, VIEWS, entryOf, poseState } from "./volumes-lib.ts";
import { Canvas, hashHue, hsl, type RGB } from "./volumes-raster.ts";

const scale = Number(process.argv[2] ?? 50);
const byBone = process.argv.includes("--bones");
const fi = process.argv.indexOf("--file");
const { asset: catalogAsset } = entryOf("vegetation/london-plane-pollarded");
const asset = fi > 0 ? JSON.parse(readFileSync(process.argv[fi + 1]!, "utf8")).asset : catalogAsset;
const state = poseState(asset, scale);
const W = 900, H = 1000, PAD = 30;
const canvas = new Canvas(W * 3, H, [250, 250, 246]);
VIEWS.forEach((view, vi) => {
  const list = [...state[view]].sort((a, b) => (a.shape.z ?? 0) - (b.shape.z ?? 0));
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const e of list) for (const r of e.rings) for (const p of r) { minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y); }
  const k = Math.min((W - 2 * PAD) / (maxX - minX), (H - 2 * PAD - 20) / (maxY - minY));
  const X = (x: number) => vi * W + PAD + (x - minX) * k, Y = (y: number) => H - PAD - (y - minY) * k;
  for (const e of list) {
    const t = e.shape.tone ?? 0.5;
    const g = Math.round(255 * (1 - 0.75 * t));
    const fill: RGB = byBone ? hsl(hashHue(e.shape.bone), 0.55, 0.7) : [g, g, g];
    if (e.closed && e.shape.filled) {
      const rings = e.rings.map((r) => r.map((p) => [X(p.x), Y(p.y)] as [number, number]));
      const ys = rings.flat().map((p) => p[1]);
      for (let y = Math.max(0, Math.floor(Math.min(...ys))); y <= Math.min(H - 1, Math.ceil(Math.max(...ys))); y++) {
        const xs: number[] = [];
        for (const r of rings) for (let i = 0; i < r.length; i++) {
          const a = r[i]!, b = r[(i + 1) % r.length]!;
          if ((a[1] <= y + 0.5) !== (b[1] <= y + 0.5)) xs.push(a[0] + ((y + 0.5 - a[1]) * (b[0] - a[0])) / (b[1] - a[1]));
        }
        xs.sort((a, b) => a - b);
        for (let i = 0; i + 1 < xs.length; i += 2) for (let x = Math.round(xs[i]!); x < Math.round(xs[i + 1]!); x++) canvas.set(x, y, fill);
      }
    }
    const ink: RGB = e.shape.role === "body" ? [20, 20, 20] : e.shape.role === "detail" ? [70, 70, 110] : [150, 110, 80];
    for (const r of e.rings) for (let i = 0; i + (e.closed ? 0 : 1) < r.length; i++) {
      const a = r[i]!, b = r[(i + 1) % r.length]!;
      canvas.line(X(a.x), Y(a.y), X(b.x), Y(b.y), ink);
    }
  }
  canvas.text(vi * W + 8, 6, `${view} 1:${scale}  x ${minX.toFixed(2)}..${maxX.toFixed(2)}  y ${minY.toFixed(2)}..${maxY.toFixed(2)}`, [20, 20, 20], 2);
});
const out = join(GENERATED, "r3-london-plane-pollarded", `draw2d.s${scale}${byBone ? ".bones" : ""}${fi > 0 ? ".new" : ""}.png`);
writeFileSync(out, canvas.png());
console.log(out);
