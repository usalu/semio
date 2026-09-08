#!/usr/bin/env bun
/** 🔣️ WP2 taxonomy edit: schema scope-owner vocabulary, export resolution, dialect, plugin-root slot,
 * declared facet kinds, and the removal of the vestigial `testContributionDirectoryOverrides` rows that
 * merely restate `testContributionDirName`. Idempotent; rewrites the file with the exact 2-space
 * `JSON.stringify(value, null, 2)` rendering the file already round-trips through. */
import { readFileSync, writeFileSync } from "node:fs";

const PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const raw = readFileSync(PATH, "utf8");
const taxonomy = JSON.parse(raw) as Record<string, unknown>;

const schemaJsonDialect = (taxonomy.mutationPayloadSchemaAuthority as { jsonSchemaDialect: string }).jsonSchemaDialect;

const additions: Record<string, unknown> = {
  schemaJsonDialect,
  schemaDefaultFacetKind: "🧬️data",
  schemaScopeOwnerLevels: {
    facetDirName: "🧬️schema",
    directoryKindId: "schema",
    levels: {
      "plugin-root": {
        pathPatterns: ["✏️s/🔌️plugins/*"],
        reason: "A plugin root owns its own identity contracts (plugin-identity, artifact-identity) that no single artifact may claim.",
      },
      "artifact-standard-subset": {
        pathPatterns: ["✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*"],
        reason: "A subset is the only level that owns artifact data: standards own subsets, subsets own schema, io and examples.",
      },
      "surface-state-lane": {
        pathPatterns: ["**/✏️editor/🎚️config", "**/✏️editor/👥️presence", "**/✏️editor/🫧️transient"],
        reason: "Persisted local-only, ephemeral shared and ephemeral local-only surface state each own their own contract.",
      },
      "framework-module": {
        pathPatterns: ["🧰️framework/🔨️modules/**"],
        reason: "A domain-neutral framework module owns the contracts its two independent consumers share.",
      },
      "product-module": {
        pathPatterns: ["🧰️framework/🛍️products/*/🔨️modules/**"],
        reason: "A product module owns the contracts of the product surface it implements.",
      },
      "hub-area-module": {
        pathPatterns: ["🌎️hub/**"],
        reason: "A hub area owns its protocol contracts; os and mcp mirrors are consumers, never co-owners.",
      },
      "mutation-leaf": {
        pathPatterns: ["**/🧬️schema/🧬️mutations/*"],
        reason: "mutationPayloadSchemaAuthority makes the leaf the authority for its own payload; aggregates are $ref unions.",
      },
    },
    excludedOwnerPathPatterns: [
      "**/🧱️elements",
      "**/🧱️elements/**",
      "**/🎯️targets",
      "**/🎯️targets/**",
      "**/📦️packages",
      "**/📦️packages/**",
      "**/🧪️*",
      "**/🧪️*/**",
      "**/🧫️*",
      "**/🧫️*/**",
      "**/🧬️contracts",
      "**/🧬️contracts/**",
    ],
  },
  schemaExportResolution: {
    uriScheme: "schema",
    uriPattern: "^schema://(?<scope>[a-z0-9]+(?:-[a-z0-9]+)*(?:\\.[a-z0-9]+(?:-[a-z0-9]+)*)*)/(?<export>[A-Z][A-Za-z0-9]*)$",
    idBase: "https://semio.tech/schema/",
    idFacetFilenamePattern: "^[a-z0-9]+(?:-[a-z0-9]+)*\\.json$",
    scopeIdSeparator: ".",
    scopeIdPattern: "^[a-z0-9]+(?:-[a-z0-9]+)*(?:\\.[a-z0-9]+(?:-[a-z0-9]+)*)*$",
    exportIdPattern: "^[A-Z][A-Za-z0-9]*$",
    rootExportKeyword: "title",
    exportsKeyword: "$defs",
    catalogPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
    catalogDocumentPath: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md",
    catalogContractId: "schema-scope-catalog-v1",
    generator: "bun ./📜️script.ts schema generate",
    forbiddenPlacementPatterns: ["**/*.schema.json", "**/🧬️schema.json", "**/🧬️contracts", "**/🧬️contracts/**"],
    placementExceptions: {},
  },
};

const facetKinds = taxonomy.schemaFacetKinds as Record<string, Record<string, unknown>>;
taxonomy.schemaFacetKinds = Object.fromEntries(
  Object.entries(facetKinds).map(([kindId, kind]) => [
    kindId,
    {
      normativeFormat: kind.normativeFormat,
      formats: kind.formats,
      facetPathIdentities: kindId === "📜️interface" ? ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema"] : [],
    },
  ]),
);

const pluginChildDirs = taxonomy.pluginChildDirs as string[];
if (!pluginChildDirs.includes("🧬️schema")) taxonomy.pluginChildDirs = [...pluginChildDirs, "🧬️schema"].sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));

const defaultContributionDir = taxonomy.testContributionDirName as string;
const overrides = taxonomy.testContributionDirectoryOverrides as Record<string, string>;
const keptOverrides = Object.fromEntries(Object.entries(overrides).filter(([, name]) => name !== defaultContributionDir));
const droppedOverrides = Object.keys(overrides).length - Object.keys(keptOverrides).length;
taxonomy.testContributionDirectoryOverrides = keptOverrides;

const rebuilt: Record<string, unknown> = {};
for (const [key, value] of Object.entries(taxonomy)) {
  if (key in additions) continue;
  rebuilt[key] = value;
  if (key === "schemaFacetKinds") for (const [additionKey, additionValue] of Object.entries(additions)) rebuilt[additionKey] = additionValue;
}
for (const key of Object.keys(additions)) if (!(key in rebuilt)) throw new Error(`Anchor key schemaFacetKinds is missing; ${key} was not inserted.`);

writeFileSync(PATH, `${JSON.stringify(rebuilt, null, 2)}\n`);
console.log(`[wp2-taxonomy-edit] wrote ${PATH}; dropped ${droppedOverrides} vestigial testContributionDirectoryOverrides rows, kept ${Object.keys(keptOverrides).length}.`);
