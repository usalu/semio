/** 📊️ Aggregates a `.cpuprofile` (written by `bun --cpu-prof`) per function: self and total seconds, the top N by self time and by total time.
 *
 * Usage: `bun validator-volumes-top.ts <file.cpuprofile> [N=25]`; a line is `self s, total s, function, file:line`.
 * @see ./validator-volumes-prof.ts */
import { readFileSync } from "node:fs";

type Node = { id: number; callFrame: { functionName: string; url: string; lineNumber: number }; children?: number[] };
const profile = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { nodes: Node[]; samples: number[]; timeDeltas: number[] };
const top = Number(process.argv[3] ?? 25);
const byId = new Map(profile.nodes.map((n) => [n.id, n] as const));
const parent = new Map<number, number>();
for (const n of profile.nodes) for (const c of n.children ?? []) parent.set(c, n.id);
const keyOf = (n: Node): string => `${n.callFrame.functionName || "(anonymous)"}  ${decodeURIComponent(n.callFrame.url).split("/").slice(-2).join("/")}:${n.callFrame.lineNumber + 1}`;
const [self, total] = [new Map<string, number>(), new Map<string, number>()];
let all = 0;
profile.samples.forEach((id, i) => {
  const dt = (profile.timeDeltas[i] ?? 0) / 1e6;
  all += dt;
  const seen = new Set<string>();
  let node = byId.get(id);
  if (node !== undefined) self.set(keyOf(node), (self.get(keyOf(node)) ?? 0) + dt);
  while (node !== undefined) {
    const key = keyOf(node);
    if (!seen.has(key)) total.set(key, (total.get(key) ?? 0) + dt);
    seen.add(key);
    node = byId.get(parent.get(node.id) ?? -1);
  }
});
const print = (title: string, order: Map<string, number>): void => {
  console.log(`== ${title} (sampled ${all.toFixed(1)} s)`);
  for (const [key] of [...order].sort((a, b) => b[1] - a[1]).slice(0, top)) console.log(`${(self.get(key) ?? 0).toFixed(2).padStart(8)} ${(total.get(key) ?? 0).toFixed(2).padStart(8)}  ${key}`);
};
print("by self time", self);
print("by total time", total);
