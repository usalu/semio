/** 🔥️ Summarises a V8/JSC .cpuprofile: top self and inclusive time per function, or with env CALLERS_OF=<substring> the inclusive time per direct caller of matching frames, or with CALLEES_OF=<substring> per direct callee of the innermost matching frame; argv: <profile> [top] [filterSubstring]. */
import { readFileSync } from "node:fs";
const [file, topArg, filter] = process.argv.slice(2);
const profile = JSON.parse(readFileSync(file!, "utf8")) as { nodes: { id: number; callFrame: { functionName: string; url: string; lineNumber: number }; children?: number[] }[]; samples: number[]; timeDeltas: number[] };
const byId = new Map(profile.nodes.map((node) => [node.id, node])), parent = new Map<number, number>();
for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
const label = (id: number): string => { const f = byId.get(id)!.callFrame; return `${f.functionName || "(anon)"} ${f.url.split("/").pop()}:${f.lineNumber + 1}`; };
const self = new Map<string, number>(), inclusive = new Map<string, number>();
let total = 0;
profile.samples.forEach((id, index) => {
  const dt = profile.timeDeltas[index] ?? 0;
  total += dt;
  self.set(label(id), (self.get(label(id)) ?? 0) + dt);
  const seen = new Set<string>();
  for (let at: number | undefined = id; at !== undefined; at = parent.get(at)) { const key = label(at); if (seen.has(key)) continue; seen.add(key); inclusive.set(key, (inclusive.get(key) ?? 0) + dt); }
});
const callersOf = process.env.CALLERS_OF;
if (callersOf) {
  const callers = new Map<string, number>();
  profile.samples.forEach((id, index) => {
    const dt = profile.timeDeltas[index] ?? 0, seen = new Set<string>();
    for (let at: number | undefined = id; at !== undefined; at = parent.get(at)) if (label(at).includes(callersOf)) { const up = parent.get(at), key = up === undefined ? "(root)" : label(up); if (!seen.has(key)) { seen.add(key); callers.set(key, (callers.get(key) ?? 0) + dt); } }
  });
  for (const [key, us] of [...callers].sort((a, b) => b[1] - a[1]).slice(0, Number(topArg ?? 20))) console.log(`${(us / 1e6).toFixed(2).padStart(8)}s ${key}`);
  process.exit(0);
}
const calleesOf = process.env.CALLEES_OF;
if (calleesOf) {
  const callees = new Map<string, number>();
  profile.samples.forEach((id, index) => {
    const dt = profile.timeDeltas[index] ?? 0;
    let child: number | undefined;
    for (let at: number | undefined = id; at !== undefined; child = at, at = parent.get(at)) if (label(at).includes(calleesOf)) { const key = child === undefined ? "(self)" : label(child); callees.set(key, (callees.get(key) ?? 0) + dt); break; }
  });
  for (const [key, us] of [...callees].sort((a, b) => b[1] - a[1]).slice(0, Number(topArg ?? 20))) console.log(`${(us / 1e6).toFixed(2).padStart(8)}s ${key}`);
  process.exit(0);
}
const top = Number(topArg ?? 40), show = (map: Map<string, number>, title: string): void => {
  console.log(`== ${title} (total ${(total / 1e6).toFixed(1)}s)`);
  for (const [key, us] of [...map].filter(([key]) => !filter || key.includes(filter)).sort((a, b) => b[1] - a[1]).slice(0, top)) console.log(`${(us / 1e6).toFixed(2).padStart(8)}s ${(100 * us / total).toFixed(1).padStart(5)}% ${key}`);
};
show(self, "self");
show(inclusive, "inclusive");
