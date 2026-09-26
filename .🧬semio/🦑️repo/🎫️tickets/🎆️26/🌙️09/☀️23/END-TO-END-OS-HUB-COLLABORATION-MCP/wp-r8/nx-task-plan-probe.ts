/** 🧮️ R8 S12-7 probe: Nx hash plan of individual tasks (no dependency tasks), summarised by top-level area, to attribute
 * over-broad inputs. Usage: bun nx-task-plan-probe.ts <out.json> <project:target>… */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";
import { HashPlanInspector } from "nx/src/hasher/hash-plan-inspector";

process.chdir("/Users/ueli/Documents/semio");
const [out, ...tasks] = process.argv.slice(2);
const graph = await createProjectGraphAsync({ exitOnError: true });
const inspector = new HashPlanInspector(graph);
await inspector.init();
const result: Record<string, unknown> = {};
for (const task of tasks) {
  const [project, target] = [task.slice(0, task.lastIndexOf(":")), task.slice(task.lastIndexOf(":") + 1)];
  const plan = inspector.inspectHashPlan([project], [target], undefined, {}, {}, true);
  const items = plan[task] ?? [];
  const areas: Record<string, number> = {};
  for (const item of items) {
    const key = item.startsWith("file:") ? `file:${item.slice(5).split("/").slice(0, 2).join("/")}` : item.split(":")[0];
    areas[key] = (areas[key] ?? 0) + 1;
  }
  result[task] = { total: items.length, areas: Object.fromEntries(Object.entries(areas).sort((a, b) => b[1] - a[1]).slice(0, 25)), nonFile: items.filter((item) => !item.startsWith("file:")) };
}
writeFileSync(out!, JSON.stringify(result, null, 1));
console.log(`tasks=${tasks.length}`);
