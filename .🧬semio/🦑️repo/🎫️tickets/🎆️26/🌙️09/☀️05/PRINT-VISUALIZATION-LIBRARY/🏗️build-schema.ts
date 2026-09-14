#!/usr/bin/env bun
/** 🏗️ Ticket-local: writes `🧬️schema/🔣️.json` — the catalogue contract plus the family option
 * vocabulary and the demo-table declarations, assembled from the handcrafted family module. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { FAMILY_DIMENSIONS, SECTIONS } from "./🏗️catalog-families.ts";
import { GROUP_TITLES_DE } from "./🏗️catalog-groups-de.ts";
import { SECTION_TITLES_DE } from "./🏗️catalog-sections-de.ts";

const PRODUCT = process.argv[2] ?? "C:/git/semio/🧰️framework/🛍️products/📓️print";

type CatalogEntry = { readonly family: string; readonly options: Record<string, string | number | boolean>; readonly id: string };
const catalog = JSON.parse(readFileSync(join(PRODUCT, "🖼️assets/🔣️viz-catalog.json"), "utf8")) as { readonly kinds: CatalogEntry[] };

const owners = new Map<string, string>();
for (const section of Object.values(SECTIONS)) {
  owners.set(section.family, section.owner);
  for (const group of Object.values(section.groups ?? {})) owners.set(group.family, group.owner ?? section.owner);
}
owners.set("spatial", "GEO-SPATIAL");

const familyOptions: Record<string, unknown> = {};
for (const family of [...new Set(catalog.kinds.map((kind) => kind.family))].sort()) {
  const options: Record<string, unknown> = {
    variant: {
      type: "string",
      description: {
        en: "Sub-kind the family renders; the catalogue slug of the chart kind. The family must draw a different geometry for every variant it declares.",
        de: "Untertyp, den die Familie zeichnet; der Katalog-Slug der Diagrammart. Die Familie muss für jede deklarierte Variante eine andere Geometrie zeichnen.",
      },
    },
  };
  for (const [key, dimension] of Object.entries(FAMILY_DIMENSIONS[family] ?? {})) {
    options[key] = { type: dimension.type, default: dimension.default, ...(dimension.enum ? { enum: dimension.enum } : {}), description: { en: dimension.en, de: dimension.de } };
  }
  familyOptions[family] = { owner: owners.get(family) ?? "CATALOG", options };
}

const demoTables = [
  { name: "demo", columns: ["cat", "val", "val2", "grp"], description: { en: "Five categorical rows with two measures and a grouping column.", de: "Fünf kategoriale Zeilen mit zwei Messgrößen und einer Gruppierungsspalte." } },
  { name: "demo-series", columns: ["t", "s1", "s2", "s3"], description: { en: "Three numeric series over a shared ordinal index.", de: "Drei numerische Reihen über einem gemeinsamen Ordinalindex." } },
  { name: "demo-time", columns: ["date", "open", "high", "low", "close", "volume"], description: { en: "A daily time series with an open-high-low-close-volume shape.", de: "Eine tägliche Zeitreihe in Eröffnung-Hoch-Tief-Schluss-Volumen-Form." } },
  { name: "demo-distribution", columns: ["group", "value"], description: { en: "Repeated observations per group for distribution renderers.", de: "Wiederholte Beobachtungen je Gruppe für Verteilungsdarstellungen." } },
  { name: "demo-parts", columns: ["part", "value"], description: { en: "Parts of one whole, summing to a hundred.", de: "Teile eines Ganzen, die sich zu hundert summieren." } },
  { name: "demo-hierarchy", columns: ["id", "parent", "value"], description: { en: "A three-level hierarchy with leaf weights.", de: "Eine dreistufige Hierarchie mit Blattgewichten." } },
  { name: "demo-graph", columns: ["source", "target", "weight"], description: { en: "A small directed weighted edge list.", de: "Eine kleine gerichtete, gewichtete Kantenliste." } },
  { name: "demo-flow", columns: ["source", "target", "value", "stage"], description: { en: "Staged flows between nodes for sankey-like renderers.", de: "Stufenweise Flüsse zwischen Knoten für Sankey-artige Darstellungen." } },
  { name: "demo-matrix", columns: ["row", "col", "value"], description: { en: "A dense row/column/value matrix.", de: "Eine dicht besetzte Zeilen-Spalten-Wert-Matrix." } },
  { name: "demo-geo", columns: ["region", "lon", "lat", "value"], description: { en: "Ring and point geometry with one measure per region.", de: "Ring- und Punktgeometrie mit einer Messgröße je Region." } },
  { name: "demo-field", columns: ["x", "y", "u", "v"], description: { en: "A sampled vector/scalar field on a regular grid.", de: "Ein abgetastetes Vektor- bzw. Skalarfeld auf einem regelmäßigen Gitter." } },
];

const sections = readFileSync(join(PRODUCT, "🖼️assets/📊️viz-taxonomy.md"), "utf8").split("\n")
  .map((line) => line.match(/^##\s+(\d+)\.\s+(.*?)\s*$/))
  .filter((match): match is RegExpMatchArray => match !== null)
  .map((match) => ({ section: match[1]!, title: { en: match[2]!, de: SECTION_TITLES_DE[match[1]!]! } }));

const groups = [...new Set(readFileSync(join(PRODUCT, "🖼️assets/📊️viz-taxonomy.md"), "utf8").split("\n")
  .map((line) => line.match(/^###\s+(.*?)\s*$/)).filter((match): match is RegExpMatchArray => match !== null).map((match) => match[1]!))]
  .sort().map((group) => ({ title: { en: group, de: GROUP_TITLES_DE[group]! } }));

const schema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  $id: "https://semio.tech/schema/print/viz/catalog.json",
  $comment:
    "📊️ Schema-first contract of the print visualization library: the chart-kind catalogue (🖼️assets/🔣️viz-catalog.json), the family option vocabularies, the demo data tables and the numeric probe protocol. Owned by the CATALOG agent; every family owner extends `x-semio-family-options` for its own families only. The `x-` annotations carry the vocabulary that pure JSON Schema cannot express per family; `🔨️modules/📊️visualization-gallery/🟦️.ts` enforces them.",
  type: "object",
  required: ["schemaVersion", "kinds"],
  additionalProperties: false,
  properties: {
    schemaVersion: { const: 1 },
    kinds: { type: "array", minItems: 1, items: { $ref: "#/$defs/CatalogEntry" } },
  },
  "x-semio-family-options": familyOptions,
  "x-semio-demo-tables": demoTables,
  "x-semio-taxonomy-sections": sections,
  "x-semio-taxonomy-groups": groups,
  $defs: {
    LocalizedText: {
      $comment: "🌍 Every user-visible string exists in both languages; the document language selects one, there is no default.",
      type: "object",
      required: ["en", "de"],
      properties: { en: { type: "string", minLength: 1 }, de: { type: "string", minLength: 1 } },
      additionalProperties: false,
    },
    TaxonomyLeafId: { $comment: "🍃 `<taxonomy section>/<leaf slug>`, the identity of one line of 🖼️assets/📊️viz-taxonomy.md.", type: "string", pattern: "^[0-9]{1,2}/[a-z0-9]+(-[a-z0-9]+)*$" },
    Slug: { $comment: "🏷️ Global, section-free chart-kind name.", type: "string", pattern: "^[a-z0-9]+(-[a-z0-9]+)*$" },
    Kind: { $comment: "🗂️ What the entry renders: a mark primitive, a chart preset, or a demonstrated kernel capability.", enum: ["mark", "chart", "layout", "axis", "scale"] },
    Namespace: {
      $comment: "📦 Taxonomy §76 namespace the renderer lives in, matching the `semio-viz-<namespace>` packages.",
      type: "string",
      pattern: "^(kernel|charts|hierarchy|network|flow|geo|spatial|matrix|diagram|scientific|timeline|table|annotation|dashboard|infographic|layout|scale|axis|legend|label|interactionstate|theme)(/[a-z0-9-]+)?$",
    },
    DemoTable: { $comment: "🧪 Name of a demo table declared in a `%region 🔖️DemoData` block.", type: "string", pattern: "^demo(-[a-z0-9]+)*$" },
    FamilyOptions: {
      $comment: "🎛️ One family's option vocabulary: `variant` is required, further keys are the real dimensions the family exposes.",
      type: "object",
      required: ["owner", "options"],
      properties: {
        owner: { type: "string", minLength: 1 },
        options: {
          type: "object",
          required: ["variant"],
          additionalProperties: {
            type: "object",
            required: ["description"],
            properties: {
              type: { enum: ["string", "number", "boolean"] },
              default: { type: ["string", "number", "boolean"] },
              enum: { type: "array", minItems: 1, uniqueItems: true, items: { type: "string" } },
              description: { $ref: "#/$defs/LocalizedText" },
            },
            additionalProperties: false,
          },
        },
      },
      additionalProperties: false,
    },
    TaxonomyGroup: {
      $comment: "📑 One group heading of the taxonomy, titled in both document languages.",
      type: "object",
      required: ["title"],
      properties: { title: { $ref: "#/$defs/LocalizedText" } },
      additionalProperties: false,
    },
    TaxonomySection: {
      $comment: "📚 One chapter of the taxonomy, titled in both document languages for the generated gallery.",
      type: "object",
      required: ["section", "title"],
      properties: { section: { type: "string", pattern: "^[0-9]{1,2}$" }, title: { $ref: "#/$defs/LocalizedText" } },
      additionalProperties: false,
    },
    DemoTableDeclaration: {
      $comment: "🧫 A named demo table every catalogue entry may point at through its `data` key.",
      type: "object",
      required: ["name", "columns", "description"],
      properties: {
        name: { $ref: "#/$defs/DemoTable" },
        columns: { type: "array", minItems: 1, uniqueItems: true, items: { type: "string", minLength: 1 } },
        description: { $ref: "#/$defs/LocalizedText" },
      },
      additionalProperties: false,
    },
    CatalogEntry: {
      $comment: "📇 One chart kind: a family plus its default options, covering one or more taxonomy leaves.",
      type: "object",
      required: ["id", "slug", "title", "kind", "namespace", "family", "options", "data", "covers"],
      properties: {
        id: { $ref: "#/$defs/TaxonomyLeafId" },
        slug: { $ref: "#/$defs/Slug" },
        title: { $ref: "#/$defs/LocalizedText" },
        kind: { $ref: "#/$defs/Kind" },
        namespace: { $ref: "#/$defs/Namespace" },
        family: { $ref: "#/$defs/Slug" },
        options: { type: "object", required: ["variant"], additionalProperties: { type: ["string", "number", "boolean"] } },
        data: { $ref: "#/$defs/DemoTable" },
        covers: { type: "array", minItems: 1, uniqueItems: true, items: { $ref: "#/$defs/TaxonomyLeafId" } },
      },
      additionalProperties: false,
    },
    ProbeRecord: {
      $comment: "🔬 One line of `\\jobname.probe.jsonl`, written by semio-viz-probe.sty and read by 🔨️modules/🧪️viz-probe.",
      type: "object",
      required: ["case", "scenario", "key", "values"],
      properties: {
        case: { type: "string", minLength: 1 },
        scenario: { type: "string", minLength: 1 },
        key: { type: "string", minLength: 1 },
        values: { type: "array", items: { type: ["number", "string"] } },
      },
      additionalProperties: false,
    },
  },
};

writeFileSync(join(PRODUCT, "🧬️schema/🔣️.json"), `${JSON.stringify(schema, null, 2)}\n`, "utf8");
console.log(`[DEBUG] schema: ${Object.keys(familyOptions).length} families, ${demoTables.length} demo tables`);
