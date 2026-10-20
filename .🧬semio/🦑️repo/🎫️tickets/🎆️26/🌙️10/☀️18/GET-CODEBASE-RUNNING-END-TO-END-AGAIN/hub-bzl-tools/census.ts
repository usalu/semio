import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const C = join(import.meta.dir, "..");
const g = JSON.parse(readFileSync(join(C, "🗑️generated/hub-bzl/graph.json"), "utf8"));
const strip = (s: string) => s.replace(/️/g, "");
const hub = Object.values<any>(g.projects).filter((p) => strip(p.root).startsWith(strip("🌎️hub/")));
const rows: any[] = [];
for (const p of hub) for (const [t, v] of Object.entries<any>(p.targets)) {
  const cmd = v.execution.command ?? (v.execution.commands ?? []).join(" && ") ?? v.execution.kind;
  rows.push({ project: p.name, root: p.root, target: t, cache: v.cache, continuous: !!v.continuous, outputs: v.outputs, inputs: v.inputs, deps: v.dependsOn, cmd: v.execution.kind === "noop" ? "(noop)" : cmd, meta: v.metadata });
}
writeFileSync(join(C, "🗑️generated/hub-bzl/hub-rows.json"), JSON.stringify(rows));
console.log("projects", hub.length, "targets", rows.length);
const by = (f: (r: any) => string) => { const m = new Map<string, number>(); for (const r of rows) m.set(f(r), (m.get(f(r)) ?? 0) + 1); return [...m].sort((a, b) => b[1] - a[1]); };
console.log("cache:", by((r) => String(r.cache)));
console.log("continuous:", by((r) => String(r.continuous)));
console.log("target names:", by((r) => r.target).slice(0, 60));
console.log("cmd heads:", by((r) => r.cmd.split(/\s+/).slice(0, 2).join(" ")).slice(0, 50));
