import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { loadOracleRegistry, mutationVectorRegistryBreaches, testTaxonomy } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
console.log("[DEBUG] loading real mutation registry");
const registry = loadOracleRegistry(root);
const catalogs = registry.contributions.flatMap(owner => owner.mutationCatalogs);
console.log("[DEBUG] validating " + catalogs.length + " catalogs");
const findings = mutationVectorRegistryBreaches(root, registry, testTaxonomy(root));
const counts: Record<string, number> = {};
for (const finding of findings) counts[finding.id] = (counts[finding.id] ?? 0) + 1;
const result = { contributions: registry.contributions.length, catalogs: catalogs.length, scenarios: catalogs.reduce((sum, catalog) => sum + catalog.vectors.reduce((total, vector) => total + vector.scenarios.length, 0), 0), findings: findings.length, counts };
writeFileSync(join(ticket, "🗑️generated/coordinator/mutation-registry-current.json"), JSON.stringify({ ...result, violations: findings }, null, 2) + "\n");
writeFileSync(join(ticket, "📓️mutation-fixture-registry-current-2026-09-09.md"), "# Real Mutation Fixture Registry\n\nThe actual production registry loader and canonical test/fixture-pair validator inspected current declared catalogs and their physical implementation/data trees.\n\n\x60\x60\x60json\n" + JSON.stringify(result, null, 2) + "\n\x60\x60\x60\n\n" + (findings.length ? "This is a failing progress snapshot. Full violations are in generated/coordinator/mutation-registry-current.json.\n" : "All inspected catalog-owned canonical pairs passed.\n"));
console.log("[DEBUG] " + JSON.stringify(result));
process.exitCode = findings.length ? 1 : 0;
