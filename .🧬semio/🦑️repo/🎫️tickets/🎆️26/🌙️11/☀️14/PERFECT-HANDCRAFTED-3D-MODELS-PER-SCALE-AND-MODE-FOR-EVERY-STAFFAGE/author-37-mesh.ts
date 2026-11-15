import { entryOf, loadSource, meshesAt } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
const scale = process.argv[3]!;
const ids = process.argv.slice(4);
const v = src.volumes[scale];
const solids = v.body.filter((s: any) => ids.includes(s.id));
const ms = meshesAt(asset, Number(scale), solids);
solids.forEach((s: any, i: number) => {
  const m: any = ms[i]!;
  const p = m.world;
  let a = 0, vol = 0;
  const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let t = 0; t < m.indices.length; t += 3) {
    const q = [0, 1, 2].map((k) => [p[m.indices[t + k] * 3], p[m.indices[t + k] * 3 + 1], p[m.indices[t + k] * 3 + 2]]);
    const u = q[1].map((x: number, j: number) => x - q[0][j]), w = q[2].map((x: number, j: number) => x - q[0][j]);
    const c = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
    a += Math.hypot(c[0], c[1], c[2]) / 2;
    vol += (q[0][0] * (q[1][1] * q[2][2] - q[1][2] * q[2][1]) - q[0][1] * (q[1][0] * q[2][2] - q[1][2] * q[2][0]) + q[0][2] * (q[1][0] * q[2][1] - q[1][1] * q[2][0])) / 6;
    for (const x of q) for (let j = 0; j < 3; j++) { lo[j] = Math.min(lo[j], x[j]); hi[j] = Math.max(hi[j], x[j]); }
  }
  console.log(s.id, "tris", m.indices.length / 3, "area", a.toFixed(4), "vol", vol.toExponential(3), "lo", lo.map((x) => x.toFixed(3)).join(","), "hi", hi.map((x) => x.toFixed(3)).join(","));
});
if (process.env.DUMP) {
  const s: any = v.body.find((x: any) => x.id === process.env.DUMP)!;
  const m: any = meshesAt(asset, Number(scale), [s])[0];
  console.log("verts", m.world.length / 3, "tris", m.indices.length / 3);
  const seen = new Set<string>();
  for (let i = 0; i < m.world.length; i += 3) { const k = [m.world[i], m.world[i + 1] - 0.74, m.world[i + 2]].map((x: number) => x.toFixed(4)).join(","); if (Math.abs(m.world[i + 2]) < 1e-6) seen.add(k); }
  console.log([...seen].join("\n"));
}
