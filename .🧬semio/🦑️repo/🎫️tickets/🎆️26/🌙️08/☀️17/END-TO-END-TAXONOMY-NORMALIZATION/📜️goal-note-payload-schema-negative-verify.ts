import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const { inventoryTaxonomy } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"));

const owner = "✏️s/🔌️plugins/🧪️zzverify2/🗿️artifacts/🧪️zzverify2/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-thing";
const fixtureParent = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🗑️temp");
mkdirSync(fixtureParent, { recursive: true });
const root = mkdtempSync(join(fixtureParent, "semio-payload-schema-negative-"));
const put = (path: string, bytes: string): void => {
  const full = join(root, path);
  mkdirSync(full.slice(0, full.lastIndexOf("/")), { recursive: true });
  writeFileSync(full, bytes);
};
put(schemaPath, readFileSync(join(repoRoot, schemaPath), "utf8"));
// wrong $schema dialect -> must still be rejected with a real, specific reason
put(`${owner}/🔣️payload.schema.json`, JSON.stringify({ $schema: "https://unowned.invalid/schema", type: "object" }));
put(`${owner}/🔣️.json`, JSON.stringify({ schemaVersion: 1, owner, semanticKind: "add-thing", payloadSchema: "🔣️payload.schema.json" }));
Bun.spawnSync(["git", "init", "--quiet"], { cwd: root });
const inventory = inventoryTaxonomy({ repoRoot: root, scope: "✏️s/🔌️plugins/🧪️zzverify2", workers: 1 });
for (const entry of inventory.entries) if (entry.sourcePath.endsWith("payload.schema.json")) console.log(JSON.stringify({ normalizedPath: entry.normalizedPath, violations: entry.violations }, null, 2));
