import { loadOracleRegistry, pipelineTable, probeTable, profileTable } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const c of registry.contributions.filter((entry) => /💬️bcf|📜️docx|🗽️obj|🧊️gltf|🀄️wfc|🗺️surface/.test(entry.owner))) if (c.problems.length > 0) console.log(c.manifestPath, c.problems);
const pipelines = pipelineTable(registry), probes = probeTable(registry), profiles = profileTable(registry);
for (const id of ["semantic-bcf-jszip-v1", "semantic-docx-ecma-376-jszip-v1", "semantic-obj-document-v1", "semantic-gltf-reader-v1"]) {
  const pipeline = pipelines.get(profiles.get(id)?.pipeline ?? "");
  console.log(id, "->", pipeline?.id, pipeline?.stages.map((s) => `${s.probe}[${probes.has(s.probe) ? "registered" : "MISSING"}](${s.inputs.join(",")})`).join(" "));
}
for (const o of ["jszip-bcf-2-1-mutate-reader", "jszip-docx-ecma-376-mutate-reader", "three-obj-3-0-document-reader", "three-gltf-2-0-mutate-reader"]) console.log(o, registry.oracles.find((x) => x.id === o)?.comparisonProfiles.join(","));
