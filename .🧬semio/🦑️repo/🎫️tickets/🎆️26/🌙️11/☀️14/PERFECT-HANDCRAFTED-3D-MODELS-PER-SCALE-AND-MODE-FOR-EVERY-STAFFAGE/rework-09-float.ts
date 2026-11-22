/** 🎈️ Author 09 ruler: which solids of a list float free. Every solid is tessellated in the world of a pose, its surface sampled into voxels, and solids that share or neighbour a voxel (or lie inside another's box) are joined; every group apart from the largest is printed with its box.
 *
 * Usage: bun rework-09-float.ts <asset> --scale N [--list body|thermal|structure] [--pose ID] [--h 0.012] [--boxes]
 * @see ./rework-brief.md */
import { cli, load, rigAt, queryOf, listsIn } from "./r3-lib.ts";

const { spec, flag, has } = cli(["--scale", "--list", "--pose", "--clip", "--t", "--h"]);
const { asset } = load(spec, "rework-09-float.ts <asset> --scale N [--list body] [--pose ID] [--h M] [--boxes]");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as "body" | "thermal" | "structure";
const h = Number(flag("--h") ?? 0.012);
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")));
const lists = listsIn(asset, scale, rig.frames);
const meshes = lists[list];
const solids = lists.solids[list];
const parent = meshes.map((_, i) => i);
const find = (i: number): number => (parent[i] === i ? i : (parent[i] = find(parent[i]!)));
const join = (a: number, b: number) => { parent[find(a)] = find(b); };
const boxes = meshes.map((m) => {
  const b = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (let k = 0; k < m.world.length; k += 3) for (let a = 0; a < 3; a++) { b[a] = Math.min(b[a]!, m.world[k + a]!); b[a + 3] = Math.max(b[a + 3]!, m.world[k + a]!); }
  return b;
});
const owner = new Map<number, number>();
const key = (x: number, y: number, z: number) => ((x + 4096) * 8192 + (y + 4096)) * 8192 + (z + 4096);
meshes.forEach((m, s) => {
  const seen = new Set<number>();
  const put = (px: number, py: number, pz: number) => {
    const [cx, cy, cz] = [Math.floor(px / h), Math.floor(py / h), Math.floor(pz / h)];
    const k0 = key(cx, cy, cz);
    if (seen.has(k0)) return;
    seen.add(k0);
    for (let dx = -1; dx <= 1; dx++) for (let dy = -1; dy <= 1; dy++) for (let dz = -1; dz <= 1; dz++) {
      const o = owner.get(key(cx + dx, cy + dy, cz + dz));
      if (o !== undefined && o !== s) join(o, s);
    }
  };
  for (let t = 0; t < m.indices.length; t += 3) {
    const [a, b, c] = [0, 1, 2].map((j) => { const i = m.indices[t + j]! * 3; return [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]; });
    const n = Math.max(1, Math.ceil(Math.max(Math.hypot(b![0]! - a![0]!, b![1]! - a![1]!, b![2]! - a![2]!), Math.hypot(c![0]! - a![0]!, c![1]! - a![1]!, c![2]! - a![2]!)) / (h * 0.6)));
    for (let i = 0; i <= n; i++) for (let j = 0; j <= n - i; j++) {
      const [u, v] = [i / n, j / n];
      put(a![0]! + u * (b![0]! - a![0]!) + v * (c![0]! - a![0]!), a![1]! + u * (b![1]! - a![1]!) + v * (c![1]! - a![1]!), a![2]! + u * (b![2]! - a![2]!) + v * (c![2]! - a![2]!));
    }
  }
  for (const k of seen) owner.set(k, s);
});
for (let i = 0; i < boxes.length; i++) for (let j = 0; j < boxes.length; j++) {
  if (i === j) continue;
  const [a, b] = [boxes[i]!, boxes[j]!];
  if (a[0]! >= b[0]! && a[1]! >= b[1]! && a[2]! >= b[2]! && a[3]! <= b[3]! && a[4]! <= b[4]! && a[5]! <= b[5]! && solids[i]!.bone === solids[j]!.bone && has("--inside")) join(i, j);
}
const groups = new Map<number, number[]>();
meshes.forEach((_, i) => groups.set(find(i), [...(groups.get(find(i)) ?? []), i]));
const sorted = [...groups.values()].sort((a, b) => b.length - a.length);
const f = (n: number) => n.toFixed(3).padStart(7);
console.log(`${spec} 1:${scale} ${list}: ${meshes.length} solids, ${sorted.length} group(s), h ${h} m, errors ${lists.errors.length}`);
sorted.forEach((g, n) => {
  if (n === 0 && !has("--boxes")) { console.log(`  main group: ${g.length} solids`); return; }
  for (const i of g) { const b = boxes[i]!; console.log(`  ${n === 0 ? "main " : "FREE" + n} ${String(solids[i]!.id).padEnd(28)} ${String(solids[i]!.bone).padEnd(12)} x ${f(b[0]!)}${f(b[3]!)} y ${f(b[1]!)}${f(b[4]!)} z ${f(b[2]!)}${f(b[5]!)}`); }
});
