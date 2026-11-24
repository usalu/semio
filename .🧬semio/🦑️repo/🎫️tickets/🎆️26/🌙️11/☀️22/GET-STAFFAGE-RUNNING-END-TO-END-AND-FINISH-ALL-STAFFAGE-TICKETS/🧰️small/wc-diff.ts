/** 🗺️ Compares the filled silhouette of two documents of one asset in one view, scale and pose on a 2 mm raster: where the second covers ground the first leaves free and the other way round, with the farthest such cell from the other silhouette (the directed Hausdorff distances) and the shapes of the second document that cover it: bun wc-diff.ts <first json> <second json> <scale> <view> <pose> */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [first, second, scale, view, pose] = process.argv.slice(2);
const CELL = 0.002;
type Ring = readonly { readonly x: number; readonly y: number }[];
const inside = (ring: Ring, x: number, y: number): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if (ring[i]!.y > y !== ring[j]!.y > y && x < ((ring[j]!.x - ring[i]!.x) * (y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) hit = !hit;
  return hit;
};
const shapesOf = (file: string) => {
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  return evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(scale) } }).shapes[view as "plan"].filter((e) => e.closed && e.shape.filled && e.shape.role !== "ground");
};
const [a, b] = [shapesOf(first!), shapesOf(second!)];
const points = [...a, ...b].flatMap((e) => e.rings.flat());
const [x0, y0, x1, y1] = [Math.min(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.x)), Math.max(...points.map((p) => p.y))];
const [nx, ny] = [Math.ceil((x1 - x0) / CELL) + 1, Math.ceil((y1 - y0) / CELL) + 1];
const raster = (shapes: typeof a): Uint8Array => {
  const grid = new Uint8Array(nx * ny);
  for (const e of shapes)
    for (const ring of e.rings) {
      const [rx0, rx1, ry0, ry1] = [Math.min(...ring.map((p) => p.x)), Math.max(...ring.map((p) => p.x)), Math.min(...ring.map((p) => p.y)), Math.max(...ring.map((p) => p.y))];
      for (let j = Math.max(0, Math.floor((ry0 - y0) / CELL)); j <= Math.min(ny - 1, Math.ceil((ry1 - y0) / CELL)); j++) for (let i = Math.max(0, Math.floor((rx0 - x0) / CELL)); i <= Math.min(nx - 1, Math.ceil((rx1 - x0) / CELL)); i++) if (inside(ring, x0 + i * CELL, y0 + j * CELL)) grid[j * nx + i] = 1;
    }
  return grid;
};
const [ga, gb] = [raster(a), raster(b)];
const cells = (grid: Uint8Array): [number, number][] => [...grid].flatMap((v, k) => (v ? [[k % nx, Math.floor(k / nx)] as [number, number]] : []));
const [ca, cb] = [cells(ga), cells(gb)];
const farthest = (from: Uint8Array, other: Uint8Array, others: [number, number][]) => {
  let worst = { d: 0, i: 0, j: 0, n: 0 };
  for (let k = 0; k < from.length; k++) {
    if (!from[k] || other[k]) continue;
    worst.n++;
    const [i, j] = [k % nx, Math.floor(k / nx)];
    let best = Infinity;
    for (const [oi, oj] of others) best = Math.min(best, (oi - i) ** 2 + (oj - j) ** 2);
    if (Math.sqrt(best) * CELL > worst.d) worst = { ...worst, d: Math.sqrt(best) * CELL, i, j };
  }
  return worst;
};
const report = (name: string, w: { d: number; i: number; j: number; n: number }, shapes: typeof a): void => {
  const [x, y] = [x0 + w.i * CELL, y0 + w.j * CELL];
  console.log(`${name}: ${(w.n * CELL * CELL * 1e4).toFixed(1)} cm2, farthest ${w.d.toFixed(4)} m at (${x.toFixed(3)}, ${y.toFixed(3)}) under ${shapes.filter((e) => e.rings.some((ring) => inside(ring, x, y))).map((e) => e.shape.id).join(" ")}`);
};
report("only in second", farthest(gb, ga, ca), b);
report("only in first", farthest(ga, gb, cb), a);
