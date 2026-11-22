/** 🎯️ Solves bone offsets of one pose so that bone-local points reach world targets (Nelder–Mead on the rig chain, regularised towards the start).
 *  bun ik.ts <spec.json>  — spec: { asset, base: {bone: offset}, root?: {x,y,angleDeg}, params: [[bone, field, start, min, max]], targets: [{bone, at:[u,v,w], world:[x,y,z], weight?}], reg? }
 *  Prints the solved offsets (base merged with the parameters) and the residual per target. */
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

type Spec = { asset: string; base: Record<string, Record<string, number>>; root?: { x: number; y: number; angleDeg: number }; params: [string, string, number, number, number][]; targets: { bone: string; at: [number, number, number]; world: [number, number, number]; weight?: number }[]; reg?: number; restarts?: number };
const spec: Spec = await Bun.file(process.argv[2]!).json();
const asset = (await Bun.file(spec.asset).json()).asset;
const rootBone = asset.rig.bones.find((b: { parent: string | null }) => b.parent === null);
const root = spec.root ?? { x: 0, y: 0, angleDeg: 0 };
const offsetsOf = (p: number[]): Record<string, Record<string, number>> => {
  const out: Record<string, Record<string, number>> = JSON.parse(JSON.stringify(spec.base));
  spec.params.forEach(([bone, field, , lo, hi], i) => ((out[bone] ??= {})[field] = Math.max(lo, Math.min(hi, p[i]!))));
  return out;
};
const pointOf = (frames: ReturnType<typeof solveChain>, bone: string, at: [number, number, number]): [number, number, number] => {
  const f = frames[bone]!.frame;
  return [0, 1, 2].map((k) => f.o[k]! + at[0] * f.u[k]! + at[1] * f.v[k]! + at[2] * f.w[k]!) as [number, number, number];
};
const start = spec.params.map((p) => p[2]);
const reg = spec.reg ?? 1e-6;
const cost = (p: number[]): number => {
  const frames = solveChain(asset.rig, offsetsOf(p), root);
  let c = 0;
  for (const t of spec.targets) {
    const q = pointOf(frames, t.bone, t.at);
    c += (t.weight ?? 1) * ((q[0] - t.world[0]) ** 2 + (q[1] - t.world[1]) ** 2 + (q[2] - t.world[2]) ** 2);
  }
  spec.params.forEach(([, , s, lo, hi], i) => (c += reg * (p[i]! - s) ** 2 + (p[i]! < lo || p[i]! > hi ? 1 : 0)));
  return c;
};
const n = start.length;
const nelder = (from: number[]): [number[], number] => {
  let simplex = [from, ...from.map((_, i) => from.map((v, j) => (j === i ? v + 8 : v)))];
  let values = simplex.map(cost);
  for (let iter = 0; iter < 6000; iter++) {
    const order = values.map((v, i) => [v, i] as const).sort((a, b) => a[0] - b[0]).map(([, i]) => i);
    simplex = order.map((i) => simplex[i]!);
    values = order.map((i) => values[i]!);
    if (values[n]! - values[0]! < 1e-14 && iter > 200) break;
    const centroid = from.map((_, j) => simplex.slice(0, n).reduce((s, p) => s + p[j]!, 0) / n);
    const along = (t: number) => centroid.map((c, j) => c + t * (simplex[n]![j]! - c));
    const r = along(-1), fr = cost(r);
    if (fr < values[0]!) { const e = along(-2), fe = cost(e); [simplex[n], values[n]] = fe < fr ? [e, fe] : [r, fr]; }
    else if (fr < values[n - 1]!) { simplex[n] = r; values[n] = fr; }
    else { const k = along(0.5), fk = cost(k); if (fk < values[n]!) { simplex[n] = k; values[n] = fk; } else { simplex = simplex.map((p) => p.map((v, j) => simplex[0]![j]! + 0.5 * (v - simplex[0]![j]!))); values = simplex.map(cost); } }
  }
  return [simplex[0]!, values[0]!];
};
let seed = 7;
const rand = () => ((seed = (seed * 16807) % 2147483647) / 2147483647);
let [bestP, bestV] = nelder(start);
for (let k = 0; k < (spec.restarts ?? 0); k++) {
  const from = spec.params.map(([, , s, lo, hi]) => Math.max(lo, Math.min(hi, s + (rand() - 0.5) * (hi - lo) * 0.6)));
  const [p, v] = nelder(nelder(from)[0]);
  if (v < bestV) [bestP, bestV] = [p, v];
}
const best = bestP;
const frames = solveChain(asset.rig, offsetsOf(best), root);
for (const t of spec.targets) {
  const q = pointOf(frames, t.bone, t.at);
  console.error(`[DEBUG] ${t.bone} -> (${q.map((v) => v.toFixed(4)).join(" ")}) target (${t.world.join(" ")}) err ${(Math.hypot(q[0] - t.world[0], q[1] - t.world[1], q[2] - t.world[2]) * 1000).toFixed(1)} mm`);
}
const result: Record<string, Record<string, number>> = {};
spec.params.forEach(([bone, field], i) => ((result[bone] ??= {})[field] = Math.round(offsetsOf(best)[bone]![field]! * 100) / 100));
console.log(JSON.stringify(result));
void rootBone;
