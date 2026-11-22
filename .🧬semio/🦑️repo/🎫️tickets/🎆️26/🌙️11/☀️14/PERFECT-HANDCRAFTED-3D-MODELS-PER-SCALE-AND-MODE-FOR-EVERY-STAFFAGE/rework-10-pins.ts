/** 📌️ Pin closure of a mechanical rig: for every pose and sampled clip instant the world distance between two bone-local points that must be one pin.
 *
 * Usage: bun rework-10-pins.ts <asset> [--file <asset document>] '<json [[label, boneA, [u,v,w], boneB, [u,v,w]], ...]>' [--samples N]
 * @see ./r3-lib.ts */
import { cli, load, rigAt, type Frame, type Query } from "./r3-lib.ts";

import { readFileSync } from "node:fs";
const { spec, flag } = cli(["--samples", "--file"]);
const asset = flag("--file") !== undefined ? JSON.parse(readFileSync(flag("--file") as string, "utf8")).asset : load(spec, "rework-10-pins.ts <asset> <pairs json> [--file doc]").asset;
const pairs = JSON.parse(process.argv.slice(2).find((a) => a.startsWith("[")) ?? "[]") as [string, string, number[], string, number[]][];
const samples = Number(flag("--samples") ?? 24);
const at = (f: Frame, p: number[]): number[] => [0, 1, 2].map((i) => f.o[i] + f.u[i] * p[0] + f.v[i] * p[1] + f.w[i] * (p[2] ?? 0));
const queries: [string, Query][] = asset.poses.map((p: { id: string }) => [`pose ${p.id}`, { kind: "pose", pose: p.id }]);
for (const c of asset.clips) for (let i = 0; i < samples; i++) queries.push([`clip ${c.id} t=${((c.durationS * i) / samples).toFixed(2)}`, { kind: "clip", clip: c.id, timeS: (c.durationS * i) / samples }]);
const worst: Record<string, [number, string]> = {};
for (const [name, q] of queries) {
  const rig = rigAt(asset, 50, q);
  for (const [label, a, pa, b, pb] of pairs) {
    const A = at(rig.frames[a], pa), B = at(rig.frames[b], pb);
    const d = Math.hypot(A[0] - B[0], A[1] - B[1], A[2] - B[2]);
    if (!(label in worst) || d > worst[label][0]) worst[label] = [d, name];
    if (process.argv.includes("--all")) console.log(name, label, (d * 1000).toFixed(1), "mm", A.map((x) => x.toFixed(3)).join(","));
  }
}
for (const [label, [d, name]] of Object.entries(worst)) console.log(`${label}: worst ${(d * 1000).toFixed(1)} mm at ${name}`);
