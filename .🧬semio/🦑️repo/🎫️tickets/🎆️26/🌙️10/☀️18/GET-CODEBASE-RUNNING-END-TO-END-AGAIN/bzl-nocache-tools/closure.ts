/** 🧭 Task closure of root targets over the static graph: bun closure.ts <graph.json> <census.json> <project:target>... ; prints uncached tasks of the closure with reason, grouped. */
import { readFileSync } from "node:fs";
const graph = JSON.parse(readFileSync(process.argv[2]!, "utf8")), census = JSON.parse(readFileSync(process.argv[3]!, "utf8")).rows as any[];
const byId = new Map(census.map((r) => [r.id, r]));
const deps = new Map<string, string[]>();
for (const e of graph.dependencies) (deps.get(e.source) ?? deps.set(e.source, []).get(e.source)!).push(e.target);
const projects = graph.projects;
const resolve = (project: string, target: string, dep: any): string[] => {
  const spec = typeof dep === "string" ? dep : dep.target;
  if (typeof spec !== "string") return [];
  const ps: string[] = typeof dep === "object" && dep.projects ? (Array.isArray(dep.projects) ? dep.projects : dep.projects === "self" ? [project] : dep.projects === "dependencies" ? deps.get(project) ?? [] : [dep.projects]) : spec.startsWith("^") || (typeof dep === "object" && dep.dependencies) ? deps.get(project) ?? [] : [project];
  const name = spec.replace(/^\^/, "");
  const ix = name.lastIndexOf(":");
  if (ix > 0 && projects[name.slice(0, ix)]) return projects[name.slice(0, ix)].targets[name.slice(ix + 1)] ? [name] : [];
  return ps.filter((p) => projects[p]?.targets[name]).map((p) => `${p}:${name}`);
};
const seen = new Set<string>(), queue = process.argv.slice(4);
while (queue.length) {
  const id = queue.pop()!;
  if (seen.has(id)) continue; seen.add(id);
  const ix = id.lastIndexOf(":"), project = id.slice(0, ix), target = id.slice(ix + 1);
  const t = projects[project]?.targets[target]; if (!t) { console.error("missing", id); continue; }
  for (const d of t.dependsOn ?? []) queue.push(...resolve(project, target, d));
  if (typeof t.inputs !== "undefined") for (const i of t.inputs) if (i && typeof i === "object" && "dependentTasksOutputFiles" in i) for (const p of deps.get(project) ?? []) { /* transitive output consumers resolved through dependsOn only */ }
}
const rows = [...seen].map((i) => byId.get(i)).filter(Boolean);
const un = rows.filter((r) => r.disposition === "uncached");
console.log(`closure ${seen.size} tasks, ${rows.filter((r) => r.disposition === "interactive").length} interactive, ${un.length} uncached`);
const g = new Map<string, any[]>();
for (const r of un) (g.get(`${r.reason}|${r.decision}`) ?? g.set(`${r.reason}|${r.decision}`, []).get(`${r.reason}|${r.decision}`)!).push(r);
for (const [k, v] of [...g].sort((a, b) => b[1].length - a[1].length)) { console.log(`## ${k} (${v.length})`); for (const r of v.slice(0, Number(process.env.LIMIT ?? 12))) console.log(`  ${r.id}  [${r.family}${r.origin === "synthesised" ? ", synth" : ""}]`); }
