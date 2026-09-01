// 🔬️ Standalone reproduction: mutation-payload-schema-authority-invalid real condition.
// Builds an isolated fixture matching mutationPayloadSchemaProjection.ownerPathPattern,
// runs the real inventoryTaxonomy against it, and prints the payload.schema.json entry's
// fileKind + violations before/after the guard fix at normalization/🟦️.ts ~line 6489.
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const { inventoryTaxonomy } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"));
const owner = "✏️s/🔌️plugins/🧪️zzverify/🗿️artifacts/🧪️zzverify/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️add-thing";

const fixtureParent = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🗑️temp");
mkdirSync(fixtureParent, { recursive: true });
const root = mkdtempSync(join(fixtureParent, "semio-payload-schema-verify-"));
const put = (path: string, bytes: string): void => {
  mkdirSync(join(root, path, "..").replace(/\/\.\.$/u, ""), { recursive: true });
  const full = join(root, path);
  mkdirSync(full.slice(0, full.lastIndexOf("/")), { recursive: true });
  writeFileSync(full, bytes);
};
put(schemaPath, readFileSync(join(repoRoot, schemaPath), "utf8"));
put(`${owner}/🔣️payload.schema.json`, JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", type: "object", additionalProperties: false, properties: { title: { type: "string" } } }));
put(`${owner}/🔣️.json`, JSON.stringify({ schemaVersion: 1, owner, semanticKind: "add-thing", payloadSchema: "🔣️payload.schema.json" }));

Bun.spawnSync(["git", "init", "--quiet"], { cwd: root });

const inventory = inventoryTaxonomy({ repoRoot: root, scope: "✏️s/🔌️plugins/🧪️zzverify", workers: 1 });
console.log("entries:", inventory.entries.length);
for (const entry of inventory.entries) {
  if (entry.sourcePath.endsWith("payload.schema.json")) {
    console.log(JSON.stringify({ sourcePath: entry.sourcePath, normalizedPath: entry.normalizedPath, fileKind: entry.fileKind, nodeKind: entry.nodeKind, violations: entry.violations }, null, 2));
  }
}
