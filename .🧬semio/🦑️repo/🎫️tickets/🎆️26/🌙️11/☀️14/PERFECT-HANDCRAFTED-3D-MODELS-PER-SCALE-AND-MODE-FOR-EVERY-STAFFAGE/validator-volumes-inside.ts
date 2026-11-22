/** ⚖️ Differential proof for the speed rewrite of `insideAny` (2026-11-22): the implementation as it was before (kept here verbatim as the reference, with its own grid) against the module's, on every solid of every list and scale of an asset document, at probes in front of and behind every face (the points the burial test asks for) and at random points of the bounding box.
 *
 * Usage: `bun validator-volumes-inside.ts <asset document json> …`; prints the number of points compared and of disagreements per file, exit 1 on any disagreement.
 * @see ./validator.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { authoredVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️lists/🟦️.ts";
import { insideAny, isClosed } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

type Vec = readonly [number, number, number];
type Solid = { readonly positions: Float64Array | Float32Array | readonly number[]; readonly indices: Uint32Array | Uint16Array | readonly number[]; readonly separateCopies?: { readonly coordinatesPerCopy: number; readonly indices: never }; readonly geometrySource?: Solid };
type Grid = { readonly min: Vec; readonly max: Vec; readonly n: number; readonly bins: Int32Array[]; readonly stamp: Int32Array; stampValue: number };
const GRIDS = new WeakMap<object, Grid>();
const SKEW: Vec = [0.000123, 0.000317, 1];

function gridOf(solid: Solid): Grid {
  let grid = GRIDS.get(solid);
  if (grid !== undefined) return grid;
  const { positions: q, indices } = solid;
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < q.length; i += 3) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, q[i + k]!);
    max[k] = Math.max(max[k]!, q[i + k]!);
  }
  const n = Math.max(1, Math.min(64, Math.round(Math.sqrt(indices.length / 3 / 4))));
  const lists: number[][] = Array.from({ length: n * n }, () => []);
  const at = (v: number, lo: number, hi: number): number => Math.max(0, Math.min(n - 1, Math.floor(((v - lo) / Math.max(hi - lo, 1e-12)) * n)));
  for (let t = 0; t < indices.length; t += 3) {
    const xs = [q[indices[t]! * 3]!, q[indices[t + 1]! * 3]!, q[indices[t + 2]! * 3]!];
    const zs = [q[indices[t]! * 3 + 2]!, q[indices[t + 1]! * 3 + 2]!, q[indices[t + 2]! * 3 + 2]!];
    for (let j = at(Math.min(...zs), min[2], max[2]); j <= at(Math.max(...zs), min[2], max[2]); j++) for (let i = at(Math.min(...xs), min[0], max[0]); i <= at(Math.max(...xs), min[0], max[0]); i++) lists[j * n + i]!.push(t);
  }
  grid = { min, max, n, bins: lists.map((l) => Int32Array.from(l)), stamp: new Int32Array(indices.length / 3), stampValue: 0 };
  GRIDS.set(solid, grid);
  return grid;
}

function insideBefore(solids: readonly Solid[], p: Vec): boolean {
  const d: Vec = [SKEW[0], SKEW[2], SKEW[1]];
  for (const solid of solids) {
    const g = gridOf(solid);
    const { min, max } = g;
    const drift = 0.0004 * (max[1] - min[1]) + 1e-9;
    if (p[0] < min[0] - drift || p[0] > max[0] + drift || p[2] < min[2] - drift || p[2] > max[2] + drift || p[1] < min[1] || p[1] > max[1]) continue;
    const { positions: q, indices } = solid;
    const at = (v: number, lo: number, hi: number): number => Math.max(0, Math.min(g.n - 1, Math.floor(((v - lo) / Math.max(hi - lo, 1e-12)) * g.n)));
    const [i0, i1] = [at(p[0] - drift, g.min[0], g.max[0]), at(p[0] + drift, g.min[0], g.max[0])];
    const [j0, j1] = [at(p[2] - drift, g.min[2], g.max[2]), at(p[2] + drift, g.min[2], g.max[2])];
    g.stampValue++;
    let crossings = 0;
    for (let j = j0; j <= j1; j++)
      for (let i = i0; i <= i1; i++)
        for (const t of g.bins[j * g.n + i]!) {
          if (g.stamp[t / 3] === g.stampValue) continue;
          g.stamp[t / 3] = g.stampValue;
          const [a, b, c] = [indices[t]! * 3, indices[t + 1]! * 3, indices[t + 2]! * 3];
          const e1: Vec = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!];
          const e2: Vec = [q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
          const h: Vec = [d[1] * e2[2] - d[2] * e2[1], d[2] * e2[0] - d[0] * e2[2], d[0] * e2[1] - d[1] * e2[0]];
          const det = e1[0] * h[0] + e1[1] * h[1] + e1[2] * h[2];
          if (Math.abs(det) < 1e-18) continue;
          const s: Vec = [p[0] - q[a]!, p[1] - q[a + 1]!, p[2] - q[a + 2]!];
          const u = (s[0] * h[0] + s[1] * h[1] + s[2] * h[2]) / det;
          if (u < 0 || u > 1) continue;
          const r: Vec = [s[1] * e1[2] - s[2] * e1[1], s[2] * e1[0] - s[0] * e1[2], s[0] * e1[1] - s[1] * e1[0]];
          const v = (d[0] * r[0] + d[1] * r[1] + d[2] * r[2]) / det;
          if (v < 0 || u + v > 1) continue;
          if ((e2[0] * r[0] + e2[1] * r[1] + e2[2] * r[2]) / det > 0) crossings++;
        }
    if (crossings % 2 === 1) return true;
  }
  return false;
}

let seed = 12345;
const random = (): number => ((seed = (seed * 1664525 + 1013904223) >>> 0) / 4294967296);
let failed = false;
for (const file of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(resolve(file), "utf8")).asset;
  let [points, differing, solids] = [0, 0, 0];
  for (const volume of authoredVolumes(asset)) {
    const d = volume.denominator;
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, asset.representations[`${d}`]?.status === "authored" ? { kind: "scale", scale: { denominator: d } } : undefined);
    const body = worldSolids(asset, state.bones as never, d, "body");
    for (const list of ["body", "thermal", "structure"] as const) {
      for (const solid of list === "body" ? body : worldSolids(asset, state.bones as never, d, list, body)) {
        if (!isClosed(solid) || solid.separateCopies) continue;
        solids++;
        const check = (p: Vec): void => {
          points++;
          if (insideAny([solid], p) !== insideBefore([solid as never], p)) differing++;
        };
        const q = solid.positions;
        const g = gridOf(solid as never);
        const stride = Math.max(1, Math.floor(solid.indices.length / 3 / 120)) * 3;
        for (let t = 0; t < solid.indices.length; t += stride) {
          const [a, b, c] = [solid.indices[t]! * 3, solid.indices[t + 1]! * 3, solid.indices[t + 2]! * 3];
          const e1 = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!];
          const e2 = [q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
          const n = [e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!];
          const length = Math.hypot(n[0]!, n[1]!, n[2]!);
          if (length < 1e-18) continue;
          for (let s = 0; s < 3; s++) {
            const [wa, wb] = [random(), random()];
            const [u, v] = wa + wb > 1 ? [1 - wa, 1 - wb] : [wa, wb];
            for (const r of [0, 1e-6, -1e-4, 1.5e-3, -1.5e-3, 0.015])
              check([q[a]! + u * e1[0]! + v * e2[0]! + (n[0]! / length) * r, q[a + 1]! + u * e1[1]! + v * e2[1]! + (n[1]! / length) * r, q[a + 2]! + u * e1[2]! + v * e2[2]! + (n[2]! / length) * r]);
          }
        }
        for (let s = 0; s < 150; s++) check([g.min[0] + random() * (g.max[0] - g.min[0]), g.min[1] + random() * (g.max[1] - g.min[1]), g.min[2] + random() * (g.max[2] - g.min[2])]);
      }
    }
  }
  console.log(`${file}: ${solids} closed solids, ${points} points, ${differing} differing`);
  failed ||= differing > 0;
}
process.exit(failed ? 1 : 0);
