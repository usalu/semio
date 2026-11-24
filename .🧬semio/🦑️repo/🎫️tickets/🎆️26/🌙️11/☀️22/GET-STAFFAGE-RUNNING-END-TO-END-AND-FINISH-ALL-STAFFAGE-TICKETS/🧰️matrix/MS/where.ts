/** 📍️ Where two views of an asset differ most: `bun where.ts <tests-dir> <id> <scale> [a=front] [b=elevation] [file.json]` prints, under the best transform of the derived-view check, the sampled points of each view that lie farthest from the other (view metres, with the normalised distance and the shape that draws them), and the extents of both views. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const tests = resolve(process.argv[2]!);
const [id, scale, a = "front", b = "elevation", file] = process.argv.slice(3);
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { TRANSFORMS, MAX_POINTS } = await import(join(tests, "🔭️multiview-coverage", "🪞️derived.ts"));
const { evaluate } = await import(join(tests, "..", "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const asset = file ? JSON.parse(readFileSync(resolve(file), "utf8")).asset : engine.loadCatalog().find((x: any) => x.id === id);
const shapes = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(scale))).shapes;
type Q = { x: number; y: number; id: string; nx: number; ny: number };
const set = (view: string): { points: Q[]; side: number; box: number[] } => {
  const raw: Q[] = [];
  for (const evaluated of shapes[view]) {
    if (evaluated.shape.role === "ground") continue;
    for (const ring of evaluated.rings)
      ring.forEach((p: any, i: number) => {
        raw.push({ x: p.x, y: p.y, id: evaluated.shape.id, nx: 0, ny: 0 });
        const next = ring[i + 1] ?? (evaluated.closed ? ring[0] : undefined);
        if (next !== undefined) raw.push({ x: (p.x + next.x) / 2, y: (p.y + next.y) / 2, id: evaluated.shape.id, nx: 0, ny: 0 });
      });
  }
  const xs = raw.map((p) => p.x), ys = raw.map((p) => p.y);
  const box = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  const side = Math.max(box[1]! - box[0]!, box[3]! - box[2]!);
  const stride = Math.max(1, Math.ceil(raw.length / MAX_POINTS));
  return { points: raw.filter((_, i) => i % stride === 0).map((p) => ({ ...p, nx: (p.x - (box[0]! + box[1]!) / 2) / side, ny: (p.y - (box[2]! + box[3]!) / 2) / side })), side, box };
};
const A = set(a), B = set(b);
console.log(`${a}: ${A.points.length} points, box x [${A.box[0]!.toFixed(3)}, ${A.box[1]!.toFixed(3)}] y [${A.box[2]!.toFixed(3)}, ${A.box[3]!.toFixed(3)}] side ${A.side.toFixed(3)}`);
console.log(`${b}: ${B.points.length} points, box x [${B.box[0]!.toFixed(3)}, ${B.box[1]!.toFixed(3)}] y [${B.box[2]!.toFixed(3)}, ${B.box[3]!.toFixed(3)}] side ${B.side.toFixed(3)}`);
let best: any;
for (const { name, m } of TRANSFORMS) {
  const moved = B.points.map((p) => ({ ...p, nx: m[0] * p.nx + m[1] * p.ny, ny: m[2] * p.nx + m[3] * p.ny }));
  const far = (from: Q[], to: Q[]): { d: number; p: Q }[] => from.map((p) => ({ p, d: Math.sqrt(Math.min(...to.map((q) => (p.nx - q.nx) ** 2 + (p.ny - q.ny) ** 2))) })).sort((u, v) => v.d - u.d);
  const ab = far(A.points, moved), ba = far(moved, A.points);
  const distance = Math.max(ab[0]!.d, ba[0]!.d);
  if (best === undefined || distance < best.distance) best = { name, distance, ab, ba };
}
console.log(`best ${best.name} ${best.distance.toFixed(4)}`);
for (const [label, list] of [[`${a} far from ${b}`, best.ab], [`${b} far from ${a}`, best.ba]] as const) {
  console.log(label);
  for (const { d, p } of list.slice(0, 8)) console.log(`  ${d.toFixed(4)} (${p.x.toFixed(3)}, ${p.y.toFixed(3)}) ${p.id}`);
}
