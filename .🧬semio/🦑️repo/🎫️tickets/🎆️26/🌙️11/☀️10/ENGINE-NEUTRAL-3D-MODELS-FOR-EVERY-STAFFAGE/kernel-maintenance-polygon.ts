/** 🔬️ Stress harness of the cap triangulator: random star polygons with collinear and near-duplicate points and holes; reports area, vertex-use and edge-pairing failures.
 * Usage: `bun kernel-maintenance-polygon.ts [count]`. */
import { signedArea, triangulatePolygon } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🔺️polygon/🟦️.ts";

type P = [number, number];
let seed = 12345;
const rnd = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
const star = (n: number, cx: number, cy: number, r0: number, r1: number): P[] => {
  const out: P[] = [];
  for (let i = 0; i < n; i++) {
    const a = (2 * Math.PI * i) / n + rnd() * 0.3 * ((2 * Math.PI) / n);
    const r = r0 + rnd() * (r1 - r0);
    out.push([cx + r * Math.cos(a), cy + r * Math.sin(a)]);
  }
  return out;
};
const decorate = (ring: P[], collinear: number, near: number, scale: number): P[] => {
  const out: P[] = [];
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
    out.push(a);
    if (rnd() < collinear) {
      const k = 1 + Math.floor(rnd() * 2);
      for (let q = 1; q <= k; q++) { const t = q / (k + 1); out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]); }
    }
    if (rnd() < near) {
      const d = 10 ** (-9 + rnd() * 3) * scale;
      out.push([a[0] + (b[0] - a[0]) * 1e-3 + d, a[1] + (b[1] - a[1]) * 1e-3]);
    }
  }
  return out;
};
const count = Number(process.argv[2] ?? 4000);
const stats: Record<string, number> = {};
const fails: string[] = [];
for (let n = 0; n < count; n++) {
  const scale = [1, 0.01, 100][n % 3]!;
  const sides = 4 + Math.floor(rnd() * 10);
  const kind = n % 4;
  const collinear = kind === 1 || kind === 3 ? 0.5 : 0;
  const near = kind === 2 || kind === 3 ? 0.4 : 0;
  let outer = decorate(star(sides, 0, 0, 0.6 * scale, scale), collinear, near, scale);
  if (signedArea(outer) < 0) outer.reverse();
  const holes: P[][] = [];
  if (n % 5 === 0) {
    let h = decorate(star(4 + Math.floor(rnd() * 4), 0, 0, 0.1 * scale, 0.3 * scale), collinear, near, scale);
    if (signedArea(h) > 0) h.reverse();
    holes.push(h);
  }
  const all = [...outer, ...holes.flat()];
  const tri = triangulatePolygon(outer, holes);
  const expectedArea = Math.abs(signedArea(outer)) - holes.reduce((s, h) => s + Math.abs(signedArea(h)), 0);
  let area = 0;
  const used = new Set<number>();
  const directed = new Map<string, number>();
  let bad = 0;
  for (let t = 0; t < tri.length; t += 3) {
    const [a, b, c] = [tri[t]!, tri[t + 1]!, tri[t + 2]!];
    const A = all[a]!, B = all[b]!, C = all[c]!;
    const cr = (B[0] - A[0]) * (C[1] - A[1]) - (B[1] - A[1]) * (C[0] - A[0]);
    if (cr < 0) bad++;
    area += cr / 2;
    for (const v of [a, b, c]) used.add(v);
    for (const [u, v] of [[a, b], [b, c], [c, a]] as const) directed.set(`${u}>${v}`, (directed.get(`${u}>${v}`) ?? 0) + 1);
  }
  const problems: string[] = [];
  if (Math.abs(area - expectedArea) > 1e-9 * scale * scale) problems.push(`area ${area} vs ${expectedArea}`);
  if (bad > 0) problems.push(`${bad} clockwise`);
  if (used.size !== all.length) problems.push(`unused vertices ${all.length - used.size}`);
  const ringEdges = new Set<string>();
  let base = 0;
  for (const ring of [outer, ...holes]) {
    for (let i = 0; i < ring.length; i++) ringEdges.add(`${base + i}>${base + ((i + 1) % ring.length)}`);
    base += ring.length;
  }
  for (const key of ringEdges) if (directed.get(key) !== 1) { problems.push(`boundary edge ${key} used ${directed.get(key) ?? 0}x`); break; }
  for (const [key, c] of directed) {
    const [u, v] = key.split(">") as [string, string];
    if (ringEdges.has(key)) continue;
    if (ringEdges.has(`${v}>${u}`)) { problems.push(`boundary reversed ${key}`); break; }
    if (c !== 1 || directed.get(`${v}>${u}`) !== 1) { problems.push(`interior edge ${key} unpaired`); break; }
  }
  const tag = `kind${kind}${holes.length ? "+hole" : ""}`;
  stats[tag] = (stats[tag] ?? 0) + 1;
  if (problems.length) {
    stats[`FAIL ${tag}`] = (stats[`FAIL ${tag}`] ?? 0) + 1;
    if (fails.length < 8) fails.push(`${tag} scale ${scale} n=${all.length}: ${problems.join("; ")}`);
  }
}
console.log(stats);
for (const f of fails) console.log(f);
