#!/usr/bin/env bun
/** 🧪️ Standalone runner for the schema scope-catalog cases, importing only `🔍️discovery/🟦️.ts`. It exists
 * because a concurrent worker's in-flight `📃️pageSchema` rename breaks the root `📜️script.ts` import graph
 * that `📦️packages/🟦️typescript/🔬️index.test.ts` pulls in; the same assertions live in that spec. */
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { inventorySchemaScopes, loadCatalogTaxonomy, parseSchemaExportUri, renderSchemaCatalog, resolveSchemaFacetKind, schemaScopeIdFromDocumentId, schemaScopeOwnerLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const repoRoot = process.cwd();
const base = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog");
const taxonomy = loadCatalogTaxonomy();
const failures: string[] = [];
const check = (label: string, actual: unknown, expected: unknown): void => {
  const left = JSON.stringify(actual);
  const right = JSON.stringify(expected);
  if (left !== right) failures.push(`${label}\n  actual   ${left}\n  expected ${right}`);
};

const cases = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
const authority = JSON.parse(readFileSync(join(base, "🛂️schema/🔣️.json"), "utf8"));
check("fixture validates against its own authority (ajv oracle)", new Ajv({ strict: true }).compile(authority)(cases), true);

for (const row of cases.cases) {
  const root = mkdtempSync(join(tmpdir(), "semio-schema-scope-"));
  try {
    for (const [rel, body] of Object.entries(row.files as Record<string, unknown>)) {
      const abs = join(root, rel);
      mkdirSync(dirname(abs), { recursive: true });
      writeFileSync(abs, typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\n`);
    }
    const inventory = inventorySchemaScopes(root, taxonomy);
    const scopes = Object.fromEntries(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [id, { path: scope.path, level: scope.level, exports: Object.fromEntries(Object.entries(scope.exports).map(([exportId, declaration]) => [exportId, { file: declaration.file, facet: declaration.facet }])), dependsOn: [...scope.dependsOn] }]));
    check(`[${row.id}] scopes`, scopes, row.expected.scopes);
    check(`[${row.id}] diagnostic codes`, inventory.diagnostics.map(({ code }) => code).sort(), [...row.expected.diagnosticCodes].sort());
    check(`[${row.id}] placement paths`, inventory.placement.map(({ path }) => path).sort(), [...row.expected.placementPaths].sort());
    check(`[${row.id}] deterministic rendering`, renderSchemaCatalog(inventory.catalog), renderSchemaCatalog(inventorySchemaScopes(root, taxonomy).catalog));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

check("scope id from artifact $id", schemaScopeIdFromDocumentId("https://semio.tech/schema/s/trinity/jack/artifact.json", taxonomy), "s.trinity.jack");
check("scope id from hub $id", schemaScopeIdFromDocumentId("https://semio.tech/schema/hub/inference/contract.json", taxonomy), "hub.inference");
check("urn $id is unaddressable", schemaScopeIdFromDocumentId("urn:semio:hub:inference", taxonomy), null);
check("facet-only $id is unaddressable", schemaScopeIdFromDocumentId("https://semio.tech/schema/contract.json", taxonomy), null);
check("export uri", parseSchemaExportUri("schema://hub.inference/InferenceApproval", taxonomy), { scopeId: "hub.inference", exportId: "InferenceApproval" });
check("camelCase export uri is rejected", parseSchemaExportUri("schema://hub.inference/inferenceApproval", taxonomy), null);
check("foreign scheme is rejected", parseSchemaExportUri("local://hub.inference/InferenceApproval", taxonomy), null);
check("plugin root level", schemaScopeOwnerLevel("✏️s/🔌️plugins/🔱️trinity", taxonomy), "plugin-root");
check("package fixture owner is ineligible", schemaScopeOwnerLevel("🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1", taxonomy), null);
check("declared interface facet kind", resolveSchemaFacetKind("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema", taxonomy), "📜️interface");
check("default facet kind", resolveSchemaFacetKind("🌎️hub/💡️inference/🧬️schema", taxonomy), taxonomy.schemaDefaultFacetKind);
check("single dialect", taxonomy.schemaJsonDialect, taxonomy.mutationPayloadSchemaAuthority.jsonSchemaDialect);

const probe = { $schema: taxonomy.schemaJsonDialect, $id: "https://semio.tech/schema/repo/library/probe.json", title: "Probe", type: "object", additionalProperties: false, properties: { id: { type: "string" } }, required: ["id"] };
const validate = new Ajv({ strict: true }).compile(probe);
check("ajv accepts the declared dialect (valid)", validate({ id: "a" }), true);
check("ajv accepts the declared dialect (invalid)", validate({ id: 1 }), false);

if (failures.length > 0) {
  console.error(`[wp2-schema-cases] ${failures.length} failing check(s):`);
  for (const failure of failures) console.error(failure);
  process.exit(1);
}
console.log(`[wp2-schema-cases] all ${cases.cases.length} fixture cases and 13 identity checks pass.`);
