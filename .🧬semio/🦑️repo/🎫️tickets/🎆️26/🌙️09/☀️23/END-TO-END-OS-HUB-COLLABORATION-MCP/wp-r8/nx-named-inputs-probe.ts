/** 🧮️ R8 S12-7 probe: every project's resolved namedInputs + target inputs (from the project graph), to locate over-broad
 * patterns behind the plugin hash plans. Usage: bun nx-named-inputs-probe.ts <out.json> */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";

process.chdir("/Users/ueli/Documents/semio");
const graph = await createProjectGraphAsync({ exitOnError: true });
const rows = Object.fromEntries(Object.values(graph.nodes).map((node) => [node.name, {
  root: node.data.root,
  namedInputs: node.data.namedInputs ?? {},
  targets: Object.fromEntries(Object.entries(node.data.targets ?? {}).map(([name, target]) => [name, { inputs: target.inputs, dependsOn: target.dependsOn }])),
}]));
writeFileSync(process.argv[2]!, JSON.stringify(rows, null, 1));
console.log(`projects=${Object.keys(rows).length}`);
