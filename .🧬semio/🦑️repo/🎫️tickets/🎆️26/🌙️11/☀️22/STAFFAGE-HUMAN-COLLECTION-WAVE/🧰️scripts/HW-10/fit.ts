/** 🎯️ Fits pose offsets (or rest placements) so that bone points and axes reach world targets, by Nelder–Mead on `evaluate`.
 *  bun fit.ts <spec.json>; spec: { file, pose, vars: [{ scope: "pose"|"rest", bone, field, init, lo, hi }], targets: [{ kind: "point", bone, local: [u,v,w], world: [x,y,z], weight } | { kind: "axis", bone, axis, world: [x,y,z], weight }], regular?: number, iterations?: number } */
import { loadAsset, type Asset } from "./lib.ts";
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

type Var = { scope: "pose" | "rest"; bone: string; field: string; init: number; lo: number; hi: number };
type Target = { kind: "point"; bone: string; local: number[]; world: number[]; weight: number } | { kind: "axis"; bone: string; axis: "u" | "v" | "w"; world: number[]; weight: number } | { kind: "perp"; bone: string; axis: "u" | "v" | "w"; other: string; otherAxis: "u" | "v" | "w"; weight: number } | { kind: "above"; bone: string; local: number[]; minY: number; weight: number };
const spec = await Bun.file(process.argv[2]!).json();
const base: Asset = spec.asset ?? (await loadAsset(spec.file));
const vars: Var[] = spec.vars;
const targets: Target[] = spec.targets;
const regular = spec.regular ?? 1e-5;

const light = { rig: base.rig, poses: base.poses.filter((p: any) => p.id === spec.pose) } as any;
function apply(x: number[]): Asset {
  const asset = structuredClone(light) as any;
  const pose = asset.poses.find((p: any) => p.id === spec.pose);
  vars.forEach((v, i) => {
    const value = Math.min(v.hi, Math.max(v.lo, x[i]!));
    if (v.scope === "rest") asset.rig.bones.find((b: any) => b.id === v.bone)[v.field] = value;
    else (pose.bones[v.bone] ??= {})[v.field] = value;
  });
  return asset;
}
const V = (a: readonly number[]) => ({ x: a[0]!, y: a[1]!, z: a[2]! });
function frames(asset: any) {
  const pose = asset.poses.find((p: any) => p.id === spec.pose);
  const chain = solveChain(asset.rig, pose.bones, { x: 0, y: spec.rootY ?? 0, angleDeg: 0 });
  return { bones: Object.fromEntries(Object.entries(chain).map(([id, f]) => [id, { frame: { o: V(f.frame.o), u: V(f.frame.u), v: V(f.frame.v), w: V(f.frame.w) } }])) } as any;
}
function cost(x: number[]): number {
  const state = frames(apply(x));
  let c = 0;
  for (const t of targets) {
    const f = state.bones[t.bone]!.frame;
    if (t.kind === "point") {
      const p = [0, 1, 2].map((k) => [f.o.x, f.o.y, f.o.z][k]! + t.local[0]! * [f.u.x, f.u.y, f.u.z][k]! + t.local[1]! * [f.v.x, f.v.y, f.v.z][k]! + t.local[2]! * [f.w.x, f.w.y, f.w.z][k]!);
      c += t.weight * p.reduce((s, pk, k) => s + (pk - t.world[k]!) ** 2, 0);
    } else if (t.kind === "axis") {
      const a = f[t.axis];
      const n = Math.hypot(...t.world);
      c += t.weight * (1 - (a.x * t.world[0]! + a.y * t.world[1]! + a.z * t.world[2]!) / n);
    } else if (t.kind === "perp") {
      const a = f[t.axis], b = state.bones[t.other]!.frame[t.otherAxis];
      c += t.weight * (a.x * b.x + a.y * b.y + a.z * b.z) ** 2;
    } else {
      const y = f.o.y + t.local[0]! * f.u.y + t.local[1]! * f.v.y + t.local[2]! * f.w.y;
      if (y < t.minY) c += t.weight * (t.minY - y) ** 2;
    }
  }
  vars.forEach((v, i) => { c += regular * (x[i]! - v.init) ** 2; if (x[i]! < v.lo) c += (v.lo - x[i]!) ** 2; if (x[i]! > v.hi) c += (x[i]! - v.hi) ** 2; });
  return c;
}
function nelderMead(x0: number[], steps: number[], iterations: number) {
  let simplex = [x0, ...x0.map((_, i) => x0.map((v, j) => (i === j ? v + steps[i]! : v)))].map((x) => ({ x, c: cost(x) }));
  for (let it = 0; it < iterations; it++) {
    simplex.sort((a, b) => a.c - b.c);
    const n = x0.length;
    const centroid = x0.map((_, k) => simplex.slice(0, n).reduce((s, p) => s + p.x[k]!, 0) / n);
    const worst = simplex[n]!;
    const at = (t: number) => centroid.map((c, k) => c + t * (worst.x[k]! - c));
    const r = { x: at(-1), c: 0 }; r.c = cost(r.x);
    if (r.c < simplex[0]!.c) { const e = { x: at(-2), c: 0 }; e.c = cost(e.x); simplex[n] = e.c < r.c ? e : r; }
    else if (r.c < simplex[n - 1]!.c) simplex[n] = r;
    else { const k = { x: at(0.5), c: 0 }; k.c = cost(k.x); if (k.c < worst.c) simplex[n] = k; else simplex = simplex.map((p, i) => (i === 0 ? p : { x: p.x.map((v, j) => simplex[0]!.x[j]! + 0.5 * (v - simplex[0]!.x[j]!)), c: 0 })).map((p, i) => (i === 0 ? p : { ...p, c: cost(p.x) })); }
  }
  simplex.sort((a, b) => a.c - b.c);
  return simplex[0]!;
}
let seed = 12345;
const random = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const polish = (x0: number[]) => { let b = { x: x0, c: cost(x0) }; for (let round = 0; round < (spec.rounds ?? 4); round++) b = nelderMead(b.x, vars.map((v) => (v.field === "x" || v.field === "y" || v.field === "z" ? 0.02 : 8) / (round + 1)), spec.iterations ?? 600); return b; };
let best = polish(vars.map((v) => v.init));
for (let r = 0; r < (spec.restarts ?? 0); r++) {
  const x0 = vars.map((v) => v.lo + random() * (v.hi - v.lo));
  const b = polish(x0);
  if (b.c < best.c) best = b;
}
const state = frames(apply(best.x));
const report = targets.map((t) => {
  const f = state.bones[t.bone]!.frame;
  if (t.kind === "axis") return { ...t, got: [f[t.axis].x, f[t.axis].y, f[t.axis].z].map((n) => +n.toFixed(4)) };
  if (t.kind === "perp") { const a = f[t.axis], b = state.bones[t.other]!.frame[t.otherAxis]; return { ...t, world: "perp", got: +(a.x * b.x + a.y * b.y + a.z * b.z).toFixed(4) }; }
  if (t.kind === "above") return { ...t, world: t.minY, got: 0 };
  const p = [0, 1, 2].map((k) => [f.o.x, f.o.y, f.o.z][k]! + t.local[0]! * [f.u.x, f.u.y, f.u.z][k]! + t.local[1]! * [f.v.x, f.v.y, f.v.z][k]! + t.local[2]! * [f.w.x, f.w.y, f.w.z][k]!);
  return { ...t, got: p.map((n) => +n.toFixed(4)) };
});
console.log(JSON.stringify({ cost: best.c, values: vars.map((v, i) => ({ scope: v.scope, bone: v.bone, field: v.field, value: +Math.min(v.hi, Math.max(v.lo, best.x[i]!)).toFixed(v.field.length === 1 ? 5 : 3) })), report }));
