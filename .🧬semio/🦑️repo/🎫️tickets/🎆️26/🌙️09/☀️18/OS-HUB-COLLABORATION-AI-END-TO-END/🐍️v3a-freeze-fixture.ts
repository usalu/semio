#!/usr/bin/env bun
/** 🧊️ V3a: freezes the `surface-schema-projection` fixture from the live projector, so the committed
 * vectors are the projector's real bytes and any later drift fails the registry's own vitest. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { projectSurfaceSchemaLane, type SurfaceSchemaLane, type SurfaceSchemaOwner } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const fixtureAbs = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🧬️surface-schema/🔣️.json");

type Case = { id: string; lane: SurfaceSchemaLane; appId: string; typeName: string; sourceRel: string; source: string; expected: Record<string, string> };

const inputs: { id: string; lane: SurfaceSchemaLane; appId: string; typeName: string; source: string }[] = [
  { id: "empty-config", lane: "config", appId: "s.demo.thing", typeName: "NoConfig", source: "pub struct NoConfig {}\n" },
  {
    id: "scalar-presence", lane: "presence", appId: "s.demo.thing", typeName: "ThingPresence",
    source: "pub struct ThingPresence {\n    pub cursor_x: f64,\n    pub label: String,\n    pub visible: bool,\n    pub revision: u32,\n}\n",
  },
  {
    id: "collection-config", lane: "config", appId: "s.demo.other", typeName: "OtherConfig",
    source: "pub struct OtherConfig {\n    pub tags: Vec<String>,\n    pub limit: Option<i32>,\n    pub ratio: Option<f32>,\n}\n",
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
  return { id: input.id, lane: input.lane, appId: input.appId, typeName: input.typeName, sourceRel, source: input.source, expected: projectSurfaceSchemaLane(repoRoot, owner, extract, sourceRel) };
});

writeFileSync(fixtureAbs, `${JSON.stringify({ version: 1, cases }, null, 2)}\n`);
console.log(`froze ${cases.length} case(s) into ${fixtureAbs}`);
for (const row of cases) console.log(`  ${row.id}: ${Object.keys(row.expected).join(" ")}`);
