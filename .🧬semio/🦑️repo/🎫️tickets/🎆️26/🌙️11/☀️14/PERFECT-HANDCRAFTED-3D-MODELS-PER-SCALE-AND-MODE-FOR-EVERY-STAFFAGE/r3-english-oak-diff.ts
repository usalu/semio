/** 🧮️ Round 3 English oak: where the outlines of two scales' bodies part in one view. Rasterises both bodies (texture solids left out) at `cellM`, takes the boundary cells of each mask, and lists the 0.5 m bins whose boundary cells lie farther than `farM` from the other boundary, with the solids under them. Usage: bun r3-english-oak-diff.ts <fine> <coarse> <elevation|front|plan> [cellM] [farM] [--slender] */
import { listsIn, load, rigAt } from "./r3-lib.ts";

const [fine, coarse] = [Number(process.argv[2]), Number(process.argv[3])];
const view = process.argv[4] as "elevation" | "front" | "plan";
const cell = Number(process.argv[5] ?? 0.015);
const far = Number(process.argv[6] ?? 0.04);
const withSlender = process.argv.includes("--slender");
const { asset } = load("vegetation/english-oak", "r3-english-oak-diff.ts");
const AX = { elevation: [0, 1], front: [2, 1], plan: [0, 2] }[view] as [number, number];
const [X0, Y0, W, H] = [-10, -8, Math.ceil(20 / cell), Math.ceil(28 / cell)];

function mask(scale: number): { m: Uint8Array; who: Int32Array; ids: string[] } {
  const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
  const lists = listsIn(asset, scale, rig.frames);
  const m = new Uint8Array(W * H);
  const who = new Int32Array(W * H).fill(-1);
  const ids: string[] = [];
  lists.body.forEach((mesh, n) => {
    const s = lists.solids.body[n]!;
    if (s.texture === true || (!withSlender && s.slender === true)) return;
    ids[n] = s.id;
    const w = mesh.world;
    for (let t = 0; t < mesh.indices.length; t += 3) {
      const p = [0, 1, 2].map((k) => [(w[mesh.indices[t + k]! * 3 + AX[0]]! - X0) / cell, (w[mesh.indices[t + k]! * 3 + AX[1]]! - Y0) / cell]);
      const [a, b, c] = p as [number, number][];
      const den = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
      if (Math.abs(den) < 1e-12) continue;
      for (let y = Math.max(0, Math.floor(Math.min(a[1], b[1], c[1]))); y <= Math.min(H - 1, Math.ceil(Math.max(a[1], b[1], c[1]))); y++)
        for (let x = Math.max(0, Math.floor(Math.min(a[0], b[0], c[0]))); x <= Math.min(W - 1, Math.ceil(Math.max(a[0], b[0], c[0]))); x++) {
          const l0 = ((b[1] - c[1]) * (x + 0.5 - c[0]) + (c[0] - b[0]) * (y + 0.5 - c[1])) / den;
          const l1 = ((c[1] - a[1]) * (x + 0.5 - c[0]) + (a[0] - c[0]) * (y + 0.5 - c[1])) / den;
          if (l0 >= 0 && l1 >= 0 && l0 + l1 <= 1) {
            m[y * W + x] = 1;
            who[y * W + x] = n;
          }
        }
    }
  });
  return { m, who, ids };
}
const edge = (m: Uint8Array): number[] => {
  const out: number[] = [];
  for (let y = 1; y < H - 1; y++) for (let x = 1; x < W - 1; x++) {
    const k = y * W + x;
    if (m[k] && (!m[k - 1] || !m[k + 1] || !m[k - W] || !m[k + W])) out.push(k);
  }
  return out;
};
function distances(from: number[], to: Uint8Array): Float32Array {
  const d = new Float32Array(W * H).fill(1e9);
  const queue: number[] = [];
  for (let y = 1; y < H - 1; y++) for (let x = 1; x < W - 1; x++) {
    const k = y * W + x;
    if (to[k] && (!to[k - 1] || !to[k + 1] || !to[k - W] || !to[k + W])) {
      d[k] = 0;
      queue.push(k);
    }
  }
  for (let q = 0; q < queue.length; q++) {
    const k = queue[q]!;
    for (const n of [k - 1, k + 1, k - W, k + W]) if (n > 0 && n < W * H && d[n]! > d[k]! + 1) {
      d[n] = d[k]! + 1;
      queue.push(n);
    }
  }
  return d;
}
const [A, B] = [mask(fine), mask(coarse)];
const [ea, eb] = [edge(A.m), edge(B.m)];
const [dToB, dToA] = [distances(ea, B.m), distances(eb, A.m)];
const all = [...ea.map((k) => dToB[k]! * cell), ...eb.map((k) => dToA[k]! * cell)].sort((a, b) => a - b);
console.log(`${view} 1:${coarse} vs 1:${fine}: boundary cells ${all.length}, p95 ${all[Math.floor(0.95 * all.length)]!.toFixed(4)} m (4-neighbour distance, approximate), over ${far} m: ${all.filter((v) => v > far).length}`);
const bins = new Map<string, { n: number; max: number; solids: Map<string, number> }>();
const note = (k: number, dist: number, src: { who: Int32Array; ids: string[] }, tag: string): void => {
  if (dist <= far) return;
  const [x, y] = [X0 + (k % W) * cell, Y0 + Math.floor(k / W) * cell];
  const key = `${(Math.floor(x * 2) / 2).toFixed(1)},${(Math.floor(y * 2) / 2).toFixed(1)}`;
  const b = bins.get(key) ?? bins.set(key, { n: 0, max: 0, solids: new Map() }).get(key)!;
  b.n++;
  b.max = Math.max(b.max, dist);
  const id = `${tag}:${src.ids[src.who[k]!] ?? "?"}`;
  b.solids.set(id, (b.solids.get(id) ?? 0) + 1);
};
for (const k of ea) note(k, dToB[k]! * cell, A, `1:${fine}`);
for (const k of eb) note(k, dToA[k]! * cell, B, `1:${coarse}`);
for (const [key, b] of [...bins.entries()].sort((p, q) => q[1].n - p[1].n).slice(0, 25)) console.log(key.padEnd(12), String(b.n).padStart(5), b.max.toFixed(3), [...b.solids.entries()].sort((p, q) => q[1] - p[1]).slice(0, 4).map(([s, n]) => `${s}(${n})`).join(" "));
if (process.argv.includes("--holes")) {
  const seen = new Uint8Array(W * H);
  const holes: [number, number, number][] = [];
  for (let s = 0; s < W * H; s++) {
    if (A.m[s] || seen[s]) continue;
    const stack = [s];
    seen[s] = 1;
    let [n, open, sx, sy] = [0, false, 0, 0];
    while (stack.length > 0) {
      const k = stack.pop()!;
      n++;
      const [x, y] = [k % W, Math.floor(k / W)];
      sx += x;
      sy += y;
      if (x === 0 || y === 0 || x === W - 1 || y === H - 1) open = true;
      for (const q of [k - 1, k + 1, k - W, k + W]) if (q >= 0 && q < W * H && !A.m[q] && !seen[q]) {
        seen[q] = 1;
        stack.push(q);
      }
    }
    if (!open) holes.push([n, X0 + (sx / n) * cell, Y0 + (sy / n) * cell]);
  }
  console.log(`holes in 1:${fine}: ${holes.length}`, holes.sort((p, q) => q[0] - p[0]).slice(0, 12).map(([n, x, y]) => `${n} cells at ${x.toFixed(2)},${y.toFixed(2)}`).join("; "));
}
