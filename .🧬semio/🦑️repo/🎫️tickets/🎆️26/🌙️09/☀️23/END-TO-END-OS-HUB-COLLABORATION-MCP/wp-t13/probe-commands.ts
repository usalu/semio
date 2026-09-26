import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const probe of registry.probes) console.log(probe.id, "|", probe.ecosystem, "|", (probe.command ?? []).join(" ").slice(0, 160), "|", probe.qualification?.status ?? "-");
const profiles = registry.comparisonProfiles.filter((p) => p.pipeline);
console.log("profiles with pipelines:", profiles.map((p) => `${p.id}->${p.pipeline}`).join(" "));
