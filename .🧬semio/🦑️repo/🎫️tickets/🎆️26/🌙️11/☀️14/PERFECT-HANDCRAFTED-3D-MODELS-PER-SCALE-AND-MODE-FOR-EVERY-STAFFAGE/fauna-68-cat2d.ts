/** 🐈️ Author 68: 2D root repair of the domestic cat head and ears (Goldfinger "Animal Anatomy for Artists"; Ellenberger/Dittrich/Baum): a round cranium with a clear stop, a short blunt muzzle with a rounded nose and chin (the drawn head ran in one straight slope to a pointed snout), cheeks widest at the zygomatic arches, ears as pointed triangles set wide at the upper corners with a thin depth.
 * Usage: bun fauna-68-cat2d.ts */
import { repair, type Op } from "./fauna-68-2d.ts";
import type { P } from "./fauna-affine.ts";
const ops: Op[] = [];
const elevHead: P[] = [[0.352, 0.272], [0.353, 0.289], [0.356, 0.302], [0.363, 0.314], [0.373, 0.322], [0.385, 0.326], [0.397, 0.326], [0.409, 0.322], [0.418, 0.315], [0.425, 0.308], [0.433, 0.303], [0.443, 0.300], [0.453, 0.297], [0.462, 0.292], [0.468, 0.285], [0.469, 0.278], [0.466, 0.271], [0.459, 0.266], [0.450, 0.263], [0.440, 0.261], [0.425, 0.257], [0.410, 0.252], [0.394, 0.248], [0.380, 0.246], [0.367, 0.245], [0.360, 0.247], [0.355, 0.252], [0.353, 0.259]];
const half: P[] = [[0.340, 0.0], [0.343, 0.018], [0.350, 0.031], [0.365, 0.040], [0.385, 0.043], [0.400, 0.044], [0.412, 0.041], [0.424, 0.035], [0.434, 0.030], [0.442, 0.028], [0.452, 0.026], [0.460, 0.020], [0.466, 0.012], [0.470, 0.0]];
const planHead: P[] = [...half.slice(0, -1).map(([x, z]) => [x, -z] as P), [0.470, 0], ...[...half].reverse().slice(1).map(([x, z]) => [x, z] as P)];
const earElev = (dx: number, scale: number): P[] => scale === 200
  ? [[0.382 + dx, 0.304], [0.383 + dx, 0.326], [0.386 + dx, 0.346], [0.389 + dx, 0.358], [0.392 + dx, 0.346], [0.395 + dx, 0.326], [0.396 + dx, 0.304]]
  : [[0.376 + dx, 0.304], [0.377 + dx, 0.322], [0.380 + dx, 0.340], [0.385 + dx, 0.357], [0.392 + dx, 0.346], [0.399 + dx, 0.328], [0.402 + dx, 0.314], [0.401 + dx, 0.304], [0.390 + dx, 0.302]];
for (const scale of [50, 100, 200]) {
  ops.push({ scale, view: "elevation", id: "head", world: elevHead });
  ops.push({ scale, view: "plan", id: "head", world: planHead });
  ops.push({ scale, view: "front", id: "head", world: Array.from({ length: 32 }, (_, i) => { const a = (2 * Math.PI * i) / 32; return [0.044 * Math.cos(a), 0.2845 + 0.0415 * Math.sin(a)] as P; }) });
  for (const [id, sd, dx] of [["near-ear", -1, 0], ["far-ear", 1, 0.0155]] as const) {
    ops.push({ scale, view: "elevation", id, world: earElev(dx, scale) });
    ops.push({ scale, view: "front", id, world: [[sd * 0.048, 0.302], [sd * 0.034, 0.358], [sd * 0.010, 0.302]] });
    ops.push({ scale, view: "plan", id, world: scale === 200 ? [[0.382 + dx, sd * 0.048], [0.396 + dx, sd * 0.048], [0.396 + dx, sd * 0.010], [0.382 + dx, sd * 0.010]] : [[0.374 + dx, sd * 0.048], [0.402 + dx, sd * 0.048], [0.402 + dx, sd * 0.010], [0.374 + dx, sd * 0.010]] });
  }
}
repair("domestic-cat", ops);
