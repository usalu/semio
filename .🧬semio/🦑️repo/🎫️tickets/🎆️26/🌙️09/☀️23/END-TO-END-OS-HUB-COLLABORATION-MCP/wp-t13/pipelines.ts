import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const id of ["bcf-2-1-jszip-compare-v1", "docx-ecma-376-jszip-compare-v1", "obj-3-0-document-compare-v1", "gltf-2-0-three-compare-v1"]) {
  const p = registry.comparisonPipelines.find((x) => x.id === id);
  console.log(id, JSON.stringify(p?.stages.map((s) => ({ probe: s.probe, inputs: s.inputs, assertions: s.assertions }))));
}
for (const p of registry.comparisonProfiles.filter((x) => x.pipeline && /bcf|docx|obj|gltf/.test(x.id))) console.log("profile", p.id, "->", p.pipeline);
for (const o of ["jszip-bcf-2-1-mutate-reader", "jszip-docx-ecma-376-mutate-reader", "three-obj-3-0-document-reader", "three-gltf-2-0-mutate-reader"]) console.log(o, registry.oracles.find((x) => x.id === o)?.comparisonProfiles.join(","));
