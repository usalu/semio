/**
 * 📏️ W6d partition measurement: every `schema-*` finding whose owner scope or path lives under
 * `✏️s/**` outside `🧬️schema/🧬️mutations/**`, computed against a FRESHLY inventoried catalog rather
 * than the tracked one, so an edit is measurable without regenerating a file another partition owns.
 *
 * @see 📋️execution-contract.md §A/§B, 📓️wp1d-harness-rules.md §2
 * Usage: bun <this> [--json <out>]
 */
import { writeFileSync } from "node:fs";
import { inventorySchemaScopes, loadCatalogTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import {
  schemaExportCompletenessDiagnostics,
  schemaFixtureIsolationDiagnostics,
  schemaOwnerEligibilityDiagnostics,
  schemaPlacementDiagnostics,
  schemaTreeFiles,
  normativeSchemaFormat,
  scopeNormativeDocuments,
  type SchemaDiagnostic,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = new URL("../../../../../../../", import.meta.url).pathname.replace(/\/$/u, "");
const MUTATIONS = "/🧬️mutations/";

const mine = (path: string | undefined | null): boolean => typeof path === "string" && path.startsWith("✏️s/") && !path.includes(MUTATIONS);

const inventory = inventorySchemaScopes(repoRoot, loadCatalogTaxonomy());
const catalog = inventory.catalog;
const normative = normativeSchemaFormat(repoRoot);

const found: SchemaDiagnostic[] = [];
for (const [id, scope] of Object.entries(catalog.scopes)) {
  if (!mine(scope.path)) continue;
  found.push(...schemaExportCompletenessDiagnostics(repoRoot, id, scope));
  void scopeNormativeDocuments(scope, normative);
}
const files = schemaTreeFiles(repoRoot, "✏️s");
for (const entry of [...schemaPlacementDiagnostics(repoRoot, files), ...schemaOwnerEligibilityDiagnostics(repoRoot, files), ...schemaFixtureIsolationDiagnostics(repoRoot, files)]) {
  if (mine(entry.path)) found.push(entry);
}

const byCode = new Map<string, number>();
for (const entry of found) byCode.set(entry.code, (byCode.get(entry.code) ?? 0) + 1);
const summary = { total: found.length, scopes: Object.values(catalog.scopes).filter((scope) => mine(scope.path)).length, byCode: Object.fromEntries([...byCode].sort((a, b) => b[1] - a[1])) };
const at = process.argv.indexOf("--json");
if (at >= 0 && process.argv[at + 1] !== undefined) writeFileSync(process.argv[at + 1]!, `${JSON.stringify({ summary, findings: found }, null, 1)}\n`);
console.log(JSON.stringify(summary, null, 1));
