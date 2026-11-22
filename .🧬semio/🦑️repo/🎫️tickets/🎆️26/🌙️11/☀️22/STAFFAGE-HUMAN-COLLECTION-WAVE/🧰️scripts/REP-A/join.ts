/** 🔗️ Joins volume solids that `volumes-connected` reports apart: for each pair `A:B` the solid A of the list is moved in the frame of its bone along the shortest way to B until they overlap by `overlap` (default a tenth of the smallest feature), in the default pose. Prints the gap and appends `solid-set` ops (the `transform.translation` of A) for `apply.py`. A structure entry that references a body solid (`of`) moves that body solid.
 *  bun join.ts <asset.json> <ops.json> <scale> <body|structure> <A:B[:overlapMm]>… */
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

type V = [number, number, number];
const [path, opsFile, sc, list, ...pairs] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const scale = Number(sc);
const ops: any[] = (await Bun.file(opsFile!).exists()) ? JSON.parse(await Bun.file(opsFile!).text()) : [];
const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V, b: V) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const closest = (p: V, a: V, b: V, c: V): V => {
  const ab = sub(b, a), ac = sub(c, a), ap = sub(p, a);
  const d1 = dot(ab, ap), d2 = dot(ac, ap);
  if (d1 <= 0 && d2 <= 0) return a;
  const bp = sub(p, b), d3 = dot(ab, bp), d4 = dot(ac, bp);
  if (d3 >= 0 && d4 <= d3) return b;
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) { const t = d1 / (d1 - d3); return [a[0] + t * ab[0], a[1] + t * ab[1], a[2] + t * ab[2]]; }
  const cp = sub(p, c), d5 = dot(ab, cp), d6 = dot(ac, cp);
  if (d6 >= 0 && d5 <= d6) return c;
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) { const t = d2 / (d2 - d6); return [a[0] + t * ac[0], a[1] + t * ac[1], a[2] + t * ac[2]]; }
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) { const t = (d4 - d3) / (d4 - d3 + (d5 - d6)); return [b[0] + t * (c[0] - b[0]), b[1] + t * (c[1] - b[1]), b[2] + t * (c[2] - b[2])]; }
  const den = 1 / (va + vb + vc), v = vb * den, w = vc * den;
  return [a[0] + ab[0] * v + ac[0] * w, a[1] + ab[1] * v + ac[1] * w, a[2] + ab[2] * v + ac[2] * w];
};
const vert = (s: any, i: number): V => [s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!];
const nearest = (from: any, to: any): { d: number; p: V; q: V } => {
  let best = { d: Infinity, p: [0, 0, 0] as V, q: [0, 0, 0] as V };
  const n = from.positions.length / 3, step = Math.max(1, Math.floor(n / 1500));
  for (let i = 0; i < n; i += step) {
    const p = vert(from, i);
    for (let k = 0; k < to.indices.length; k += 3) {
      const q = closest(p, vert(to, to.indices[k]!), vert(to, to.indices[k + 1]!), vert(to, to.indices[k + 2]!));
      const d = Math.hypot(...sub(p, q));
      if (d < best.d) best = { d, p, q };
    }
  }
  return best;
};
for (const pair of pairs) {
  const [a, b, over] = pair.split(":") as [string, string, string | undefined];
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const body = worldSolids(asset, state.bones as never, scale as never);
  const worlds: readonly any[] = list === "body" ? body : worldSolids(asset, state.bones as never, scale as never, list as never, body);
  const [wa, wb] = [worlds.find((w) => w.id === a), worlds.find((w) => w.id === b)];
  if (!wa || !wb) { console.log(`${pair}: solid not found`); continue; }
  const [x, y] = [nearest(wa, wb), nearest(wb, wa)];
  const best = x.d <= y.d ? { d: x.d, from: x.p, to: x.q } : { d: y.d, from: y.q, to: y.p };
  const entry = asset.volumes[sc][list].find((s: any) => s.id === a);
  const target = entry.of === undefined ? { list, solid: entry } : { list: "body", solid: asset.volumes[sc].body.find((s: any) => s.id === entry.of) };
  const overlap = over === undefined ? featureOf(scale as never) / 10 : Number(over) / 1000;
  if (best.d < 1e-6) { console.log(`${pair}: already touching`); continue; }
  const move = sub(best.to, best.from).map((c) => c * (1 + overlap / best.d)) as V;
  const f = state.bones[target.solid.bone].frame;
  const local = [f.u, f.v, f.w].map((ax: any) => move[0] * ax.x + move[1] * ax.y + move[2] * ax.z);
  const old = target.solid.transform?.translation ?? [0, 0, 0];
  const transform = { ...(target.solid.transform ?? {}), translation: old.map((c: number, i: number) => +(c + local[i]!).toFixed(5)) };
  target.solid.transform = transform;
  ops.push({ op: "solid-set", scale: sc, list: target.list, id: target.solid.id, set: { transform } });
  console.log(`${pair}: gap ${(best.d * 1000).toFixed(1)} mm, ${target.list} solid "${target.solid.id}" moved by ${local.map((c) => (c * 1000).toFixed(1)).join(", ")} mm (u, v, w of ${target.solid.bone})`);
}
await Bun.write(opsFile!, JSON.stringify(ops, null, 1));
