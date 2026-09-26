/** 🧮️ R8 S12-7 probe: the resolved Nx hash plan (every input a task hashes) of each plugin's describe/component-dev/materialize-dev,
 * computed by Nx's own HashPlanInspector — no task runs. Usage: bun nx-hash-plan-probe.ts <out.json> [project…] */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";
import { HashPlanInspector } from "nx/src/hasher/hash-plan-inspector";

process.chdir("/Users/ueli/Documents/semio");
const out = process.argv[2]!;
const only = process.argv.slice(3);
const started = Date.now();
const graph = await createProjectGraphAsync({ exitOnError: true });
const graphSecs = Math.round((Date.now() - started) / 1000);
const targets = ["describe", "component-dev", "materialize-dev"];
const projects = Object.values(graph.nodes).filter((node) => node.data.targets?.["component-dev"] && (!only.length || only.includes(node.name))).map((node) => node.name).sort();
const inspector = new HashPlanInspector(graph);
await inspector.init();
const plan = inspector.inspectHashPlan(projects, targets, undefined, {}, {}, true);
const roots = Object.fromEntries(Object.values(graph.nodes).map((node) => [node.name, node.data.root]));
const deps = Object.fromEntries(Object.entries(graph.dependencies).map(([name, edges]) => [name, edges.map((edge) => edge.target)]));
writeFileSync(out, JSON.stringify({ graphSecs, projects, roots, deps, plan }, null, 1));
console.log(`graph=${graphSecs}s projects=${projects.length} tasks=${Object.keys(plan).length} total=${Math.round((Date.now() - started) / 1000)}s`);
