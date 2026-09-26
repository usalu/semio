/** 🧮️ R8 S12-7 probe: attributes a task's hash plan to its individual inputs by re-planning the task with ONE input at a time
 * (in-memory graph edit, nothing written). Usage: bun nx-input-attribution-probe.ts <out.json> <project> <target> */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";
import { HashPlanInspector } from "nx/src/hasher/hash-plan-inspector";

process.chdir("/Users/ueli/Documents/semio");
const [out, project, target] = process.argv.slice(2) as [string, string, string];
const graph = await createProjectGraphAsync({ exitOnError: true });
const node = graph.nodes[project]!;
const inputs = [...(node.data.targets![target]!.inputs ?? [])];
const expand = (input: unknown): unknown[] => typeof input === "string" && !input.startsWith("^") && !input.includes("{") && node.data.namedInputs?.[input] ? (node.data.namedInputs[input] as unknown[]) : [input];
const rows: unknown[] = [];
for (const input of inputs.flatMap(expand)) {
  node.data.targets![target]!.inputs = [input as never];
  const inspector = new HashPlanInspector(graph);
  await inspector.init();
  const items = inspector.inspectHashPlan([project], [target], undefined, {}, {}, true)[`${project}:${target}`] ?? [];
  const files = items.filter((item) => item.startsWith("file:"));
  const areas: Record<string, number> = {};
  for (const item of files) { const key = item.slice(5).split("/").slice(0, 2).join("/"); areas[key] = (areas[key] ?? 0) + 1; }
  rows.push({ input, files: files.length, areas: Object.fromEntries(Object.entries(areas).sort((a, b) => b[1] - a[1]).slice(0, 6)) });
}
writeFileSync(out, JSON.stringify(rows, null, 1));
console.log(`inputs=${rows.length}`);
