/** 🧩️ Which shapes of a refitted file leave the drawing it had before, in the default pose: `bun outside.ts <before.json> <after.json> <view> [scale…]` tests, per authored scale, every drawn point of every filled shape of the second file whose id the first file does not have against the filled shapes of the first file (even-odd, point by point, no polygon union) and prints each shape with a point outside (id, bone, solid, the number of points outside, the farthest of them with its distance to the nearest edge of the former drawing). The fact behind a silhouette guard that fails after a repair which must not change the rest drawing. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
type Q = { x: number; y: number };
const [beforeArg, afterArg, view, ...keys] = process.argv.slice(2);
const [before, after] = [readDoc(beforeArg!).doc.asset, readDoc(afterArg!).doc.asset];
const filled = (asset: any, key: string): { shape: any; rings: Q[][] }[] => {
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
  return (state.shapes[view!] as any[]).filter((s) => s.shape.filled === true && s.shape.role !== "ground").map((s) => ({ shape: s.shape, rings: (s.rings as Q[][]).filter((ring) => ring.length > 2) }));
};
const inside = (p: Q, rings: Q[][]): boolean => {
  let odd = false;
  for (const ring of rings)
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [a, b] = [ring[i]!, ring[j]!];
      if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
    }
  return odd;
};
const distance = (p: Q, rings: Q[][]): number => {
  let least = Infinity;
  for (const ring of rings)
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const [a, b] = [ring[i]!, ring[j]!];
      const l = (b.x - a.x) ** 2 + (b.y - a.y) ** 2;
      const t = l === 0 ? 0 : Math.min(1, Math.max(0, ((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / l));
      least = Math.min(least, Math.hypot(p.x - a.x - t * (b.x - a.x), p.y - a.y - t * (b.y - a.y)));
    }
  return least;
};
for (const key of Object.keys(after.representations).filter((k) => engine.authoredOf(after, k) !== undefined && (keys.length === 0 || keys.includes(k)))) {
  const was = filled(before, key);
  const ids = new Set(was.map((s) => `${s.shape.bone}/${s.shape.id}`));
  const fresh = filled(after, key).filter((s) => !ids.has(`${s.shape.bone}/${s.shape.id}`));
  let count = 0, worst = 0;
  for (const s of fresh) {
    const out = s.rings.flat().filter((p) => !was.some((w) => inside(p, w.rings))).map((p) => ({ p, d: Math.min(...was.map((w) => distance(p, w.rings))) })).filter((o) => o.d > 0.001).sort((a, b) => b.d - a.d);
    if (out.length === 0) continue;
    count++;
    worst = Math.max(worst, out[0]!.d);
    if (count <= 12) console.log(`1:${key} ${view} "${s.shape.id}" on ${s.shape.bone}${s.shape.solid ? ` (${s.shape.solid.kind} ${s.shape.solid.sweep} ${s.shape.solid.z0})` : ""}: ${out.length} of ${s.rings.flat().length} points outside, farthest ${(out[0]!.d * 1000).toFixed(0)} mm at (${out[0]!.p.x.toFixed(3)}, ${out[0]!.p.y.toFixed(3)})`);
  }
  console.log(`${after.id} 1:${key} ${view}: ${count} of ${fresh.length} new shapes leave the former drawing, farthest ${(worst * 1000).toFixed(0)} mm`);
}
