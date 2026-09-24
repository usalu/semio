import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { contributionSchemaProblems, discoverTestContributions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const schema = JSON.parse(readFileSync(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema, "p");
const map: Record<string, string> = { oracles: "OracleRegistryEntry", probes: "ProbeRegistryEntry", noOracleDecisions: "NoOracleDecision", comparisonProfiles: "ComparisonProfileSpec", comparisonPipelines: "ComparisonPipeline", toleranceProfiles: "ToleranceProfile", oracleHostPackages: "OracleHostPackage", mutationManifests: "MutationManifest", fixtureManifests: "FixtureManifest" };
let ours = 0, disagreements = 0, records = 0, ajvBad = 0;
const lines: string[] = [];
for (const c of discoverTestContributions(root)) {
  const raw = JSON.parse(readFileSync(`${root}/${c.manifestPath}`, "utf8"));
  const problems = contributionSchemaProblems(raw);
  ours += problems.length;
  for (const p of problems) lines.push(`${c.manifestPath} :: ${p}`);
  for (const [key, def] of Object.entries(map)) for (const [i, item] of (raw[key] ?? []).entries()) {
    records++;
    const ok = ajv.getSchema(`p#/$defs/${def}`)!(item) as boolean;
    if (!ok) ajvBad++;
    const oursOk = !problems.some((p) => p.startsWith(`${key}[${i}]`));
    if (ok !== oursOk) { disagreements++; lines.push(`DISAGREE ${c.manifestPath} ${key}[${i}] ajv=${ok} ours=${oursOk}`); }
  }
}
console.log(lines.join("\n"));
console.log(JSON.stringify({ records, ajvInvalidRecords: ajvBad, ourProblems: ours, disagreements }));
