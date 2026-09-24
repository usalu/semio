import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const oracle of registry.oracles) {
  const missing = ["testOnly", "license", "capabilities", "comparisonProfiles", "rationale"].filter((key) => (oracle as Record<string, unknown>)[key] === undefined);
  if (missing.length > 0 || oracle.testOnly !== true) console.log(oracle.id, missing, oracle.testOnly);
}
for (const decision of registry.noOracleDecisions) if (!decision.rationale || !decision.substitutes?.length) console.log("decision", decision.id, JSON.stringify(decision).slice(0, 200));
console.log("contribution problems:", registry.contributions.filter((c) => c.problems.length).map((c) => `${c.manifestPath}: ${c.problems.slice(0, 2).join("; ")}`).slice(0, 10));
