import { loadOracleRegistry, profileTable, pipelineTable } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
const profiles = profileTable(registry);
for (const id of process.argv.slice(2)) {
  const p = profiles.get(id);
  console.log(id, JSON.stringify(p, null, 1));
  if (p?.pipeline) console.log(JSON.stringify(pipelineTable(registry).get(p.pipeline), null, 1));
}
console.log("pipelines:", registry.comparisonPipelines.map((x) => x.id).join(" "));
console.log("probes:", registry.probes.map((x) => x.id).join(" "));
