#!/usr/bin/env bun
/** 🧊️ K3: freezes the `surface-schema-projection` fixture from the live projector. Supersedes
 * `🐍️v3a-freeze-fixture.ts` by adding the two cases this slice's contract change introduced — a
 * 64-bit integer lane (`u64`/`i64`) and a nested-`$defs` lane — so the committed vectors remain the
 * projector's real bytes and any later drift fails the registry's own vitest. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { projectSurfaceSchemaLane, type SurfaceSchemaLane, type SurfaceSchemaOwner } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const fixtureAbs = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json");

type Definition = { typeName: string; source: string };
type Case = { id: string; lane: SurfaceSchemaLane; appId: string; typeName: string; sourceRel: string; source: string; definitions?: Definition[]; expected: Record<string, string> };

const inputs: { id: string; lane: SurfaceSchemaLane; appId: string; typeName: string; source: string; definitions?: Definition[] }[] = [
  { id: "empty-config", lane: "config", appId: "s.demo.thing", typeName: "NoConfig", source: "pub struct NoConfig {}\n" },
  {
    id: "scalar-presence", lane: "presence", appId: "s.demo.thing", typeName: "ThingPresence",
    source: "pub struct ThingPresence {\n    pub cursor_x: f64,\n    pub label: String,\n    pub visible: bool,\n    pub revision: u32,\n}\n",
  },
  {
    id: "collection-config", lane: "config", appId: "s.demo.other", typeName: "OtherConfig",
    source: "pub struct OtherConfig {\n    pub tags: Vec<String>,\n    pub limit: Option<i32>,\n    pub ratio: Option<f32>,\n}\n",
  },
  {
    id: "wide-integer-config", lane: "config", appId: "s.demo.ledger", typeName: "LedgerConfig",
    source: "pub struct LedgerConfig {\n    pub generation: u64,\n    pub drift: i64,\n    pub label: String,\n}\n",
  },
  {
    id: "nested-config", lane: "config", appId: "s.demo.index", typeName: "IndexConfig",
    source: "pub struct IndexConfig {\n    pub visibility: String,\n    pub rows: Vec<IndexRow>,\n}\n",
    definitions: [
      { typeName: "IndexDialect", source: "pub struct IndexDialect {\n    pub kind: String,\n}\n" },
      { typeName: "IndexRow", source: "pub struct IndexRow {\n    pub id: String,\n    pub dialect: IndexDialect,\n    pub created_at_ms: u64,\n}\n" },
    ],
  },
];

const cases: Case[] = inputs.map((input) => {
  const sourceRel = `🧫️fixtures/🧬️surface-schema/${input.id}/🦀️.rs`;
  const owner: SurfaceSchemaOwner = {
    pluginId: "🧪️demo", pluginRoot: join(repoRoot, "✏️s/🔌️plugins/🧪️demo"),
    ownerAbs: join(repoRoot, "✏️s/🔌️plugins/🧪️demo"), ownerLabel: "fixture", surfaceAbs: null,
    lane: input.lane, laneAbs: join(repoRoot, "✏️s/🔌️plugins/🧪️demo", input.lane === "config" ? "🎚️config" : "👥️presence"),
    facetAbs: join(repoRoot, "✏️s/🔌️plugins/🧪️demo", input.lane === "config" ? "🎚️config" : "👥️presence", "🧬️schema"),
    typeName: input.typeName, appId: input.appId,
  };
  const extract = policyExtractRustSchemaFields(input.source, input.typeName);
  if (extract.typeName !== input.typeName) throw new Error(`${input.id}: source does not declare ${input.typeName}`);
  const definitions = (input.definitions ?? []).map((definition) => {
    const nested = policyExtractRustSchemaFields(definition.source, definition.typeName);
    if (nested.typeName !== definition.typeName) throw new Error(`${input.id}: nested source does not declare ${definition.typeName}`);
    return nested;
  });
  const row: Case = { id: input.id, lane: input.lane, appId: input.appId, typeName: input.typeName, sourceRel, source: input.source, expected: projectSurfaceSchemaLane(repoRoot, owner, extract, sourceRel, definitions) };
  if (input.definitions) row.definitions = input.definitions;
  return row;
});

writeFileSync(fixtureAbs, `${JSON.stringify({ version: 1, cases }, null, 2)}\n`);
console.log(`froze ${cases.length} case(s) into ${fixtureAbs}`);
for (const row of cases) console.log(`  ${row.id}: ${Object.keys(row.expected).join(" ")}`);
