/** 🤲 Moves the shoulder joints of a human figure to a new lateral offset and re-solves the arm angles of every pose and of every clip key that states the arms (a key's unstated arm angles start from its pose), so that each hand keeps its old place and orientation in the world (damped least squares through the product's own `evaluate`, regularised towards the old angles, inside the joint limits). Writes `{ rig, poses, clips }` for the asset's own candidate script.
 *
 * Usage: `bun r5-person-with-rollator-arms.ts <category/slug> --z <metres> --anchor <anchor id of the left hand> --params arm:angleDeg,abductionDeg,twistDeg;forearm:angleDeg;hand:angleDeg --out <roots.json> [--base <roots.json>]`
 * @see ./r5-brief.md */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { cli } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { spec, flag } = cli(["--z", "--anchor", "--params", "--out", "--base"]);
const asset0 = { ...(entryOf(spec!).asset as Json), volumes: undefined } as Json;
if (flag("--base")) Object.assign(asset0, JSON.parse(readFileSync(flag("--base")!, "utf8")));
const Z = Number(flag("--z"));
const anchorL = flag("--anchor")!;
const params = flag("--params")!.split(";").map((p) => { const [seg, list] = p.split(":"); return list!.split(",").map((k) => [seg!, k] as [string, string]); }).flat();
type Q = { kind: "pose"; pose: string } | { kind: "clip"; clip: string; timeS: number };
type Target = { p: number[]; u: number[]; v: number[] };
const V = (o: Json): number[] => [o.x, o.y, o.z ?? 0];
function handOf(asset: Json, q: Q, side: "l" | "r"): Target {
  const fresh = { ...asset, poses: structuredClone(asset.poses), clips: structuredClone(asset.clips), rig: structuredClone(asset.rig) };
  const s = evaluate(fresh, q as never) as Json;
  const a = s.anchors[side === "l" ? anchorL : anchorL.replace(/-l$/, "-r")];
  const f = s.bones[`hand-${side}`].frame;
  return { p: [a.at.x, a.at.y, a.z ?? 0], u: V(f.u), v: V(f.v) };
}
const limits = (asset: Json, bone: string, key: string): [number, number] => {
  const b = (asset.rig.bones as Json[]).find((x) => x.id === bone)!;
  const j = b.joint ?? {};
  if (key === "angleDeg") return [j.minDeg ?? -180, j.maxDeg ?? 180];
  if (key === "abductionDeg") return [j.abductionMinDeg ?? -90, j.abductionMaxDeg ?? 90];
  if (key === "twistDeg") return [j.twistMinDeg ?? -180, j.twistMaxDeg ?? 180];
  return [-180, 180];
};
function solve(asset: Json, holder: Json, q: Q, side: "l" | "r", target: Target, base: Json = {}): { err: number; rot: number } {
  const keys = params.map(([seg, k]) => [`${seg}-${side}`, k] as [string, string]);
  for (const [b] of keys) holder[b] ??= {};
  for (const [b, k] of keys) if (holder[b][k] === undefined && base[b]?.[k] !== undefined) holder[b][k] = base[b][k];
  const x0 = keys.map(([b, k]) => holder[b][k] ?? 0);
  const get = (): number[] => keys.map(([b, k]) => holder[b][k] ?? 0);
  const set = (x: number[]): void => keys.forEach(([b, k], i) => { const [lo, hi] = limits(asset, b, k); holder[b][k] = Math.round(Math.min(hi, Math.max(lo, x[i]!)) * 1e4) / 1e4; });
  const res = (): number[] => {
    const h = handOf(asset, q, side);
    const L = 0.08;
    return [...[0, 1, 2].map((i) => h.p[i]! - target.p[i]!), ...[0, 1, 2].map((i) => (h.u[i]! - target.u[i]!) * L), ...[0, 1, 2].map((i) => (h.v[i]! - target.v[i]!) * L)];
  };
  let x = get();
  let r = res();
  let lambda = 1e-3;
  for (let it = 0; it < 40; it++) {
    const J: number[][] = [];
    for (let i = 0; i < x.length; i++) {
      const xi = [...x]; xi[i]! += 0.05; set(xi);
      const ri = res();
      J.push(ri.map((v, k) => (v - r[k]!) / 0.05));
    }
    set(x);
    const n = x.length;
    const reg = 1e-7;
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => J[i]!.reduce((a, v, k) => a + v * J[j]![k]!, 0) + (i === j ? lambda + reg : 0)));
    const g = Array.from({ length: n }, (_, i) => -J[i]!.reduce((a, v, k) => a + v * r[k]!, 0) - reg * (x[i]! - x0[i]!));
    for (let i = 0; i < n; i++) { let p = i; for (let k = i + 1; k < n; k++) if (Math.abs(A[k]![i]!) > Math.abs(A[p]![i]!)) p = k; [A[i], A[p]] = [A[p]!, A[i]!]; [g[i], g[p]] = [g[p]!, g[i]!]; for (let k = i + 1; k < n; k++) { const f = A[k]![i]! / A[i]![i]!; for (let j = i; j < n; j++) A[k]![j]! -= f * A[i]![j]!; g[k]! -= f * g[i]!; } }
    const d = Array(n).fill(0);
    for (let i = n - 1; i >= 0; i--) d[i] = (g[i]! - A[i]!.slice(i + 1).reduce((a, v, k) => a + v * d[i + 1 + k], 0)) / A[i]![i]!;
    const xn = x.map((v, i) => v + d[i]);
    set(xn);
    const rn = res();
    const nn = (a: number[]): number => Math.hypot(...a);
    if (nn(rn) < nn(r)) { x = get(); r = rn; lambda *= 0.3; } else { set(x); lambda *= 10; }
    if (nn(r) < 1e-5) break;
  }
  set(x);
  return { err: Math.hypot(r[0]!, r[1]!, r[2]!), rot: Math.hypot(...r.slice(3)) / 0.08 };
}
const old = structuredClone(asset0);
const asset = structuredClone(asset0);
for (const b of asset.rig.bones as Json[]) if (/^arm-[lr]$/.test(b.id)) b.z = b.id === "arm-l" ? Z : -Z;
const report: string[] = [];
for (const pose of asset.poses as Json[]) for (const side of ["l", "r"] as const) {
  const q: Q = { kind: "pose", pose: pose.id };
  const t = handOf(old, q, side);
  const r = solve(asset, pose.bones, q, side, t);
  report.push(`pose ${pose.id} ${side}: hand miss ${(r.err * 1000).toFixed(2)} mm, rotation miss ${(r.rot * 57.3).toFixed(2)} deg`);
}
for (const clip of asset.clips as Json[]) for (const [ki, key] of (clip.keys as Json[]).entries()) {
  if (!key.bones || !Object.keys(key.bones).some((b) => /^(arm|forearm|hand)-/.test(b))) continue;
  const tEval = clip.loop && key.t >= clip.durationS ? null : key.t;
  if (tEval === null) {
    const first = (clip.keys as Json[])[0]!;
    for (const b of Object.keys(key.bones)) if (/^(arm|forearm|hand)-/.test(b) && first.bones[b]) key.bones[b] = structuredClone(first.bones[b]);
    report.push(`clip ${clip.id} key ${key.t}: closing key takes the arms of key 0`);
    continue;
  }
  for (const side of ["l", "r"] as const) {
    const q: Q = { kind: "clip", clip: clip.id, timeS: tEval ?? key.t - 1e-6 };
    const t = handOf(old, q, side);
    const r = solve(asset, key.bones, q, side, t, (asset.poses as Json[]).find((p) => p.id === key.pose)?.bones ?? {});
    report.push(`clip ${clip.id} key ${key.t} ${side}: hand miss ${(r.err * 1000).toFixed(2)} mm, rotation miss ${(r.rot * 57.3).toFixed(2)} deg`);
  }
}
writeFileSync(flag("--out")!, JSON.stringify({ rig: asset.rig, poses: asset.poses, clips: asset.clips }, null, 1));
console.log(report.join("\n"));
