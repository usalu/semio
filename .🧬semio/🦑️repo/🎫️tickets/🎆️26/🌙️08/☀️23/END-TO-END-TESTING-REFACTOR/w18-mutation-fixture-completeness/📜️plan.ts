/** 🏃️ Emits a Plan JSON for one committed case, built by the platform's own parser. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { parseFeature } from "/Users/ueli/Documents/semio/./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";

const [featurePath, role, outDir] = process.argv.slice(2);
const feature = parseFeature(readFileSync(featurePath, "utf8"));
if (feature.errors.length > 0) {
  console.error("feature parse errors:", feature.errors);
  process.exit(2);
}
mkdirSync(outDir, { recursive: true });
const plan = {
  schemaVersion: 1,
  owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
  case: "ticket-local",
  featurePath,
  featureHash: "ticket-local",
  featureName: feature.name,
  capability: feature.capability,
  oracle: null,
  noOracleDecision: feature.noOracleDecision,
  comparison: feature.comparison,
  background: feature.background,
  scenarios: feature.scenarios,
  adapters: {},
  fixtures: [],
  role,
  implementation: "rust",
  level: "exhaustive",
  workDir: join(outDir, "work"),
  outputDir: join(outDir, "out"),
  resultsPath: join(outDir, "out", "📤️results.jsonl"),
};
mkdirSync(plan.workDir, { recursive: true });
mkdirSync(plan.outputDir, { recursive: true });
writeFileSync(join(outDir, "plan.json"), JSON.stringify(plan, null, 2));
console.log(`${feature.scenarios.length} scenario(s): ${feature.scenarios.map((s) => s.id).join(", ")}`);
