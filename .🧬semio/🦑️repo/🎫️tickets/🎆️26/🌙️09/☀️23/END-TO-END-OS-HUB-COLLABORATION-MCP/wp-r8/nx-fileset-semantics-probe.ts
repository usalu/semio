/** 🧮️ R8 S12-7 probe: how Nx's hash planner combines positive and negated filesets (workspace- vs project-rooted), measured
 * by re-planning one task with explicit input lists (in-memory graph edit). Usage: bun nx-fileset-semantics-probe.ts <out.json> */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";
import { HashPlanInspector } from "nx/src/hasher/hash-plan-inspector";

process.chdir("/Users/ueli/Documents/semio");
const project = "@semio-tech/note-plugin", target = "materialize-dev";
const bundle = "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle";
const cases: Record<string, string[]> = {
  workspacePositive: [`${bundle}/**/*.ts`],
  workspacePositiveMinusTests: [`${bundle}/**/*.ts`, `!${bundle}/**/🧪️tests/**/*.ts`],
  workspaceNegationAlone: [`!${bundle}/**/🧪️tests/**/*.ts`],
  projectPositive: ["{projectRoot}/**/*"],
  projectPositiveMinusScript: ["{projectRoot}/**/*", "!{projectRoot}/📜️script.ts"],
  projectNegationAlone: ["!{projectRoot}/📜️script.ts"],
};
const graph = await createProjectGraphAsync({ exitOnError: true });
const result: Record<string, number> = {};
for (const [name, inputs] of Object.entries(cases)) {
  graph.nodes[project]!.data.targets![target]!.inputs = inputs as never;
  const inspector = new HashPlanInspector(graph);
  await inspector.init();
  result[name] = (inspector.inspectHashPlan([project], [target], undefined, {}, {}, true)[`${project}:${target}`] ?? []).filter((item) => item.startsWith("file:")).length;
}
writeFileSync(process.argv[2]!, JSON.stringify(result, null, 1));
console.log(JSON.stringify(result));
