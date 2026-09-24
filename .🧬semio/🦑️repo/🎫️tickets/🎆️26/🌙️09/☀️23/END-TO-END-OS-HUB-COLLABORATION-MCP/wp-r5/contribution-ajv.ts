import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { discoverTestContributions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const schema = JSON.parse(readFileSync(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema, "p");
const map: Record<string, string> = { oracles: "OracleRegistryEntry", probes: "ProbeRegistryEntry", noOracleDecisions: "NoOracleDecision", mutationManifests: "MutationManifest", fixtureManifests: "FixtureManifest", comparisonProfiles: "ComparisonProfileSpec", comparisonPipelines: "ComparisonPipeline", toleranceProfiles: "ToleranceProfile", oracleHostPackages: "OracleHostPackage" };
const counts: Record<string, number> = {};
for (const c of discoverTestContributions(root)) {
  const raw = JSON.parse(readFileSync(`${root}/${c.manifestPath}`, "utf8"));
  for (const [key, def] of Object.entries(map)) {
    for (const [i, item] of (raw[key] ?? []).entries()) {
      const v = ajv.getSchema(`p#/$defs/${def}`)!;
      if (!v(item)) { counts[key] = (counts[key] ?? 0) + 1; if (process.argv[2] === key || process.argv[2] === "all") console.log(c.manifestPath, key, i, item.id ?? "", JSON.stringify(v.errors!.slice(0, 3).map((e) => `${e.instancePath} ${e.message} ${JSON.stringify(e.params)}`))); }
    }
  }
  for (const k of Object.keys(raw)) if (!(k in map) && !["$schema", "schemaVersion", "_comment", "mutationCatalogs", "migrationStatus"].includes(k)) console.log("unknown key", c.manifestPath, k);
}
console.log(counts);
