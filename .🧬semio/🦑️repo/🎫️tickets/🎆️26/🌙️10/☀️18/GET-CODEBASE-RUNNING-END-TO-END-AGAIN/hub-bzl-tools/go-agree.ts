// 🐹️ Cross-checks the staged Go classifier against the live coverage classifier over the planned tasks of the static graph.
import { readFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio", C = join(import.meta.dir, "..");
const api: any = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"));
const go: any = await import("/private/tmp/hubbzl/ov/🏗️build/🔧️tools/🏗️bazel/🐹️go/🟦️.ts");
const graph = JSON.parse(readFileSync(join(C, "🗑️generated/hub-bzl/graph.json"), "utf8"));
const names = new Set<string>(); for (const p of Object.values<any>(graph.projects)) for (const t of Object.keys(p.targets)) names.add(p.name + ":" + t);
for (const p of Object.values<any>(graph.projects)) for (const t of Object.values<any>(p.targets)) if (Array.isArray(t.dependsOn)) t.dependsOn = t.dependsOn.filter((d: any) => { const s = (typeof d === "string" ? d : d.target).replace(/^\^/, ""); return !s.includes(":") || names.has(s); });
const plan = api.planBazelTasksV1(graph, { tasks: [...names], args: [] });
let agree = 0, goTasks = 0, mismatch: string[] = [];
for (const task of Object.values<any>(plan.tasks)) {
  const emitted = api.bazelActionsV1(graph, { ...plan, tasks: { [task.id]: { ...task, dependencies: [] } }, selected: [task.id] }, false);
  const action = emitted.get(task.id); if (!action) continue;
  const family = api.bazelTaskCoverageV1(task, action.cache).family === "go", mine = go.bazelGoClassV1(task);
  if (family === mine) agree++; else mismatch.push(task.id);
  if (mine) goTasks++;
}
console.log(JSON.stringify({ tasks: Object.keys(plan.tasks).length, agree, goTasks, mismatch: mismatch.slice(0, 10) }));
