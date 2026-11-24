/** 🌗️ How far the outer silhouettes of two views of an asset are apart: `bun outline.ts <tests-dir> <id> [scales] [file.json]` rasters the filled closed shapes of the front and of the side elevation of the default pose (400 cells along the longest side), takes the boundary cells of each union and prints per scale the normalised Hausdorff distance of the two boundaries under identity and under the mirror about the vertical axis, with the place of the worst point in metres. It tells whether a single outline per view can tell the two views apart at all. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const tests = resolve(process.argv[2]!);
const id = process.argv[3]!;
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(tests, "..", "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const asset = process.argv[5] ? JSON.parse(readFileSync(resolve(process.argv[5]), "utf8")).asset : engine.loadCatalog().find((x: any) => x.id === id);
const scales = process.argv[4] && process.argv[4] !== "-" ? process.argv[4].split(",") : Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
type P = { x: number; y: number };
const N = 400;
const boundary = (shapes: any[]): { points: P[]; side: number; cx: number; cy: number } => {
  const filled = shapes.filter((s) => s.shape.role !== "ground" && s.closed && s.shape.filled !== false && s.rings.length > 0);
  const all: P[] = filled.flatMap((s) => s.rings.flat());
  const minX = Math.min(...all.map((p) => p.x)), maxX = Math.max(...all.map((p) => p.x)), minY = Math.min(...all.map((p) => p.y)), maxY = Math.max(...all.map((p) => p.y));
  const side = Math.max(maxX - minX, maxY - minY), cell = side / N;
  const w = Math.ceil((maxX - minX) / cell) + 4, h = Math.ceil((maxY - minY) / cell) + 4;
  const mask = new Uint8Array(w * h);
  for (const s of filled) {
    const own = new Uint8Array(w * h);
    for (let j = 0; j < h; j++) {
      const y = minY + (j - 1.5) * cell;
      const xs: number[] = [];
      for (const ring of s.rings) for (let i = 0; i < ring.length; i++) {
        const a = ring[i], b = ring[(i + 1) % ring.length];
        if ((a.y <= y && b.y > y) || (b.y <= y && a.y > y)) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
      }
      xs.sort((u, v) => u - v);
      for (let k = 0; k + 1 < xs.length; k += 2) for (let i = Math.max(0, Math.ceil((xs[k]! - minX) / cell + 1.5)); i <= Math.min(w - 1, Math.floor((xs[k + 1]! - minX) / cell + 1.5)); i++) own[j * w + i] = 1;
    }
    for (let k = 0; k < own.length; k++) if (own[k]) mask[k] = 1;
  }
  const points: P[] = [];
  for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) if (mask[j * w + i] && !(mask[j * w + i - 1] && mask[j * w + i + 1] && mask[(j - 1) * w + i] && mask[(j + 1) * w + i])) points.push({ x: minX + (i - 1.5) * cell, y: minY + (j - 1.5) * cell });
  return { points, side, cx: (minX + maxX) / 2, cy: (minY + maxY) / 2 };
};
for (const key of scales) {
  const shapes = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
  const f = boundary(shapes.front), e = boundary(shapes.elevation);
  const norm = (b: typeof f, flip: number): P[] => b.points.map((p) => ({ x: (flip * (p.x - b.cx)) / b.side, y: (p.y - b.cy) / b.side }));
  const out: string[] = [];
  for (const [name, flip] of [["identity", 1], ["mirror-x", -1]] as const) {
    const A = norm(f, 1), B = norm(e, flip);
    const far = (from: P[], to: P[], raw: P[]): { d: number; at: P } => {
      let worst = { d: 0, at: raw[0]! };
      from.forEach((p, i) => {
        let best = Infinity;
        for (const q of to) { const d = (p.x - q.x) ** 2 + (p.y - q.y) ** 2; if (d < best) best = d; }
        if (best > worst.d) worst = { d: best, at: raw[i]! };
      });
      return { d: Math.sqrt(worst.d), at: worst.at };
    };
    const ab = far(A, B, f.points), ba = far(B, A, e.points);
    out.push(`${name} ${Math.max(ab.d, ba.d).toFixed(4)} (front ${ab.d.toFixed(4)} at ${ab.at.x.toFixed(2)},${ab.at.y.toFixed(2)}; elevation ${ba.d.toFixed(4)} at ${ba.at.x.toFixed(2)},${ba.at.y.toFixed(2)})`);
  }
  console.log(`${asset.id} 1:${key} outer silhouettes: ${out.join(" | ")}`);
}
