import { readFileSync } from "node:fs";
const graph = JSON.parse(readFileSync("C:/git/semio/.nx/workspace-data/project-graph.json", "utf8"));
const nodes = graph.nodes ?? {};
const ids = process.argv.slice(2);
for (const id of ids) {
  const [project, target, configuration] = id.split(":");
  const node = nodes[project];
  if (!node) { console.log("NO PROJECT", id); continue; }
  const t = node.data.targets?.[target];
  if (!t) { console.log("NO TARGET ", id); continue; }
  const configs = Object.keys(t.configurations ?? {});
  console.log(configuration && !configs.includes(configuration) ? "NO CONFIG " : "ok        ", id, t.continuous ? "continuous" : "", configs.length ? "configs=" + configs.join(",") : "");
}
