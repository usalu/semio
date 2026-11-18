/** 🔩 The 3D bars of the slender volume solids (and plain cylinder ties) of one asset scale in bone-local coordinates: JSON list {id, bone, from, to, d} with d the largest cross diameter; the axis is the principal axis of the copy's vertices.
 *  bun machinery-bars.ts <asset> <scale> <out.json> <idRegex> */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, out, regex] = process.argv.slice(2) as [string, string, string, string];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const re = new RegExp(regex);
type V = [number, number, number];
const res: unknown[] = [];
const r4 = (v: number) => +v.toFixed(4);
function bar(id: string, bone: string, pts: V[]): void {
  const n = pts.length;
  const c: V = [0, 1, 2].map((k) => pts.reduce((s, p) => s + p[k]!, 0) / n) as V;
  const cov = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
  for (const p of pts) for (let a = 0; a < 3; a++) for (let b = 0; b < 3; b++) cov[a]![b]! += (p[a]! - c[a]!) * (p[b]! - c[b]!);
  let v: V = [1, 0.3, 0.2];
  for (let i = 0; i < 60; i++) {
    const w: V = [0, 1, 2].map((a) => cov[a]![0]! * v[0] + cov[a]![1]! * v[1] + cov[a]![2]! * v[2]) as V;
    const l = Math.hypot(...w) || 1;
    v = [w[0] / l, w[1] / l, w[2] / l];
  }
  let [lo, hi, rad] = [Infinity, -Infinity, 0];
  const offs: number[] = [];
  for (const p of pts) {
    const d: V = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
    const t = d[0] * v[0] + d[1] * v[1] + d[2] * v[2];
    lo = Math.min(lo, t); hi = Math.max(hi, t);
    offs.push(Math.hypot(d[0] - t * v[0], d[1] - t * v[1], d[2] - t * v[2]));
  }
  offs.sort((a, b) => a - b);
  rad = offs[offs.length - 1]!;
  const f = state.bones[bone].frame;
  const loc = (t: number): V => {
    const q: V = [c[0] + t * v[0] - f.o.x, c[1] + t * v[1] - f.o.y, c[2] + t * v[2] - f.o.z];
    return [r4(q[0] * f.u.x + q[1] * f.u.y + q[2] * f.u.z), r4(q[0] * f.v.x + q[1] * f.v.y + q[2] * f.v.z), r4(q[0] * f.w.x + q[1] * f.w.y + q[2] * f.w.z)];
  };
  res.push({ id, bone, from: loc(lo), to: loc(hi), d: r4(2 * rad) });
}
for (const s of worldSolids(asset, state.bones as never, scale)) {
  if (s.texture === true || !re.test(s.id) || s.bone === undefined) continue;
  if (s.slender === true) {
    const per = s.copyVertices ?? s.positions.length / 3;
    const copies = Math.max(1, Math.round(s.positions.length / 3 / per));
    for (let k = 0; k < copies; k++) {
      const pts: V[] = [];
      for (let i = 0; i < per; i++) pts.push([s.positions[(k * per + i) * 3]!, s.positions[(k * per + i) * 3 + 1]!, s.positions[(k * per + i) * 3 + 2]!]);
      bar(`${s.id}#${k}`, s.bone, pts);
    }
  } else {
    const nv = s.positions.length / 3;
    const parent = Array.from({ length: nv }, (_, i) => i);
    const find = (i: number): number => { while (parent[i] !== i) { parent[i] = parent[parent[i]!]!; i = parent[i]!; } return i; };
    for (let t = 0; t < s.indices.length; t += 3) { parent[find(s.indices[t + 1]!)] = find(s.indices[t]!); parent[find(s.indices[t + 2]!)] = find(s.indices[t]!); }
    const groups = new Map<number, V[]>();
    for (let i = 0; i < nv; i++) { const k = find(i); if (!groups.has(k)) groups.set(k, []); groups.get(k)!.push([s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!]); }
    const list = [...groups.values()];
    const ext = (g: V[]) => Math.max(...[0, 1, 2].map((a) => Math.max(...g.map((p) => p[a]!)) - Math.min(...g.map((p) => p[a]!))));
    const big = Math.max(...list.map(ext));
    list.filter((g) => ext(g) >= 0.5 * big).forEach((g, k) => bar(`${s.id}#${k}`, s.bone!, g));
  }
}
writeFileSync(out, JSON.stringify(res));
console.log("bars", res.length);
