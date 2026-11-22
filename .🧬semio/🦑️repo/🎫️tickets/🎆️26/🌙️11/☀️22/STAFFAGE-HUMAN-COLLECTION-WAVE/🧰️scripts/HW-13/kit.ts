/** 🦾️ HW-13 posing kit: evaluates draft poses with the library's `evaluate` and solves staged inverse-kinematics specs (Nelder–Mead on joint offsets against world point, direction and line targets).
 *  bun kit.ts solve <asset.json> <spec.json> <out.json>   |   bun kit.ts frames <asset.json> <pose|-> <out.json>
 *  @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

type V = [number, number, number];
type Json = any;
type Frame = { o: V; u: V; v: V; w: V };
const vec = (p: Json): V => [p.x, p.y, p.z ?? 0];
const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const len = (a: V): number => Math.sqrt(dot(a, a));
const unit = (a: V): V => { const l = len(a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
export const toWorld = (f: Frame, p: V): V => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V;

/** 🧍️ Frames of every bone of the asset in a pose given by its bone offsets (ground auto on the contacts); `null` bones = the rest pose. */
export function framesOf(asset: Json, bones: Json | null, contacts: string[], ground: "auto" | "authored" = "auto"): { frames: Record<string, Frame>; anchors: Record<string, V> } {
  const pose = { id: "hw13-probe", title: { en: "Probe", de: "Probe" }, description: { en: "Probe", de: "Probe" }, bones: bones ?? {}, ground: bones === null ? "authored" : ground, contacts: bones === null ? [] : contacts, thermal: asset.poses[0].thermal, loads: [] };
  const s: Json = evaluate({ ...asset, poses: [...asset.poses, pose] }, { kind: "pose", pose: "hw13-probe" });
  const frames: Record<string, Frame> = {};
  for (const [id, b] of Object.entries<Json>(s.bones)) frames[id] = { o: vec(b.frame.o), u: vec(b.frame.u), v: vec(b.frame.v), w: vec(b.frame.w) };
  const anchors: Record<string, V> = {};
  for (const [id, a] of Object.entries<Json>(s.anchors)) anchors[id] = [a.at.x, a.at.y, a.z ?? 0];
  return { frames, anchors };
}

function nelderMead(f: (x: number[]) => number, x0: number[], step: number[], iterations = 4000): number[] {
  const n = x0.length;
  let simplex = [x0, ...x0.map((_, i) => x0.map((v, j) => (i === j ? v + step[i]! : v)))].map((x) => ({ x, f: f(x) }));
  for (let it = 0; it < iterations; it++) {
    simplex.sort((a, b) => a.f - b.f);
    if (Math.abs(simplex[n]!.f - simplex[0]!.f) < 1e-12 && it > 50) break;
    const c = x0.map((_, j) => simplex.slice(0, n).reduce((s, p) => s + p.x[j]!, 0) / n);
    const at = (t: number): number[] => c.map((v, j) => v + t * (simplex[n]!.x[j]! - v));
    const r = at(-1), fr = f(r);
    if (fr < simplex[0]!.f) { const e = at(-2), fe = f(e); simplex[n] = fe < fr ? { x: e, f: fe } : { x: r, f: fr }; }
    else if (fr < simplex[n - 1]!.f) simplex[n] = { x: r, f: fr };
    else { const k = at(fr < simplex[n]!.f ? -0.5 : 0.5), fk = f(k); if (fk < Math.min(fr, simplex[n]!.f)) simplex[n] = { x: k, f: fk }; else simplex = simplex.map((p, i) => (i === 0 ? p : { x: p.x.map((v, j) => simplex[0]!.x[j]! + 0.5 * (v - simplex[0]!.x[j]!)), f: 0 })).map((p, i) => (i === 0 ? p : { x: p.x, f: f(p.x) })); }
  }
  simplex.sort((a, b) => a.f - b.f);
  return simplex[0]!.x;
}

/** 🎯️ One objective term: `point` puts a bone-local point on a world point, `line` puts it on a world line (point + direction), `axis` aligns a bone axis with a world direction, `level` keeps a bone axis perpendicular to a direction, `pair` puts one world coordinate of a point of `bone` at that of a point of `other` (plus `offset`). */
type Term = { kind: "point" | "line" | "axis" | "level" | "pair"; bone: string; local?: V; axis?: "u" | "v" | "w"; target?: V; through?: V; direction?: V; weight?: number; other?: string; otherLocal?: V; coordinate?: number; offset?: number };
type Stage = { vars: [string, string, number, number][]; terms: Term[]; regularise?: number };
type Spec = { poses: { id: string; bones: Json; contacts: string[]; stages: Stage[] }[] };

function cost(fr: Record<string, Frame>, terms: Term[]): number {
  let c = 0;
  for (const t of terms) {
    const f = fr[t.bone]!;
    const w = t.weight ?? 1;
    if (t.kind === "point") c += w * dot(sub(toWorld(f, t.local!), t.target!), sub(toWorld(f, t.local!), t.target!)) * 1e4;
    else if (t.kind === "line") { const d = unit(t.direction!); const r = sub(toWorld(f, t.local!), t.through!); const along = dot(r, d); const perp = sub(r, [d[0] * along, d[1] * along, d[2] * along]); c += w * dot(perp, perp) * 1e4; }
    else if (t.kind === "axis") c += w * (1 - dot(unit(f[t.axis!]), unit(t.direction!)));
    else if (t.kind === "level") c += w * dot(unit(f[t.axis!]), unit(t.direction!)) ** 2;
    else { const k = t.coordinate ?? 1; c += w * (toWorld(f, t.local!)[k]! - toWorld(fr[t.other!]!, t.otherLocal!)[k]! - (t.offset ?? 0)) ** 2 * 1e4; }
  }
  return c;
}

export function solve(asset: Json, spec: Spec): Record<string, { bones: Json; residual: number[] }> {
  const out: Record<string, { bones: Json; residual: number[] }> = {};
  for (const pose of spec.poses) {
    const bones: Json = structuredClone(pose.bones);
    const residual: number[] = [];
    for (const stage of pose.stages) {
      const get = ([b, k]: [string, string, number, number]): number => bones[b]?.[k] ?? 0;
      const x0 = stage.vars.map(get);
      const set = (x: number[]): void => stage.vars.forEach(([b, k, lo, hi], i) => { bones[b] = { ...(bones[b] ?? {}), [k]: Math.min(hi, Math.max(lo, x[i]!)) }; });
      const f = (x: number[]): number => { set(x); const penalty = stage.vars.reduce((s, [, , lo, hi], i) => s + (x[i]! < lo ? lo - x[i]! : x[i]! > hi ? x[i]! - hi : 0) ** 2, 0); return cost(framesOf(asset, bones, pose.contacts).frames, stage.terms) + (stage.regularise ?? 1e-5) * x.reduce((s, v, i) => s + (v - x0[i]!) ** 2, 0) + penalty; };
      let x = nelderMead(f, x0, x0.map(() => 8));
      x = nelderMead(f, x, x0.map(() => 1.5));
      x = nelderMead(f, x, x0.map(() => 0.3));
      set(x);
      for (const [b, k] of stage.vars) bones[b][k] = Math.round(bones[b][k] * 100) / 100;
      residual.push(cost(framesOf(asset, bones, pose.contacts).frames, stage.terms));
    }
    out[pose.id] = { bones, residual };
    console.error(`[DEBUG] ${pose.id} residuals ${residual.map((r) => r.toExponential(2)).join(" ")}`);
  }
  return out;
}

if (import.meta.main) {
  const [cmd, assetPath, a, b] = process.argv.slice(2);
  const asset = JSON.parse(readFileSync(assetPath!, "utf8")).asset ?? JSON.parse(readFileSync(assetPath!, "utf8"));
  if (cmd === "solve") writeFileSync(b!, JSON.stringify(solve(asset, JSON.parse(readFileSync(a!, "utf8"))), null, 1));
  else if (cmd === "frames") {
    const result: Json = {};
    for (const id of a!.split(",")) {
      const p = asset.poses.find((q: Json) => q.id === id);
      result[id] = id === "-" ? framesOf(asset, null, []) : framesOf(asset, p.bones, p.contacts, p.ground);
    }
    writeFileSync(b!, JSON.stringify(result));
  }
}
