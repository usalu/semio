#!/usr/bin/env bun
/** @emoji 🛝️ Runtime check: playgrounds load manifestId-only fixtures without crashing. */
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../../");

function assert(cond, msg) {
  if (!cond) throw new Error(msg);
}

const trinityJson = readFileSync(join(root, "trinity/fixture/nakagin-capsule-tower.trinity.json"), "utf8");
const trinityFixture = JSON.parse(trinityJson);
assert(trinityFixture.manifestId === "nakagin", "trinity manifestId");
assert(!trinityFixture.manifest, "trinity inline manifest removed");

const initTrinity = (await import(join(root, "trinity/rewrite/engine/pkg/trinity_rewrite.js"))).default;
const { TrinitySession } = await import(join(root, "trinity/rewrite/engine/pkg/trinity_rewrite.js"));
await initTrinity();
const session = new TrinitySession();
session.loadFixtureJson(trinityJson);
const roundTrip = JSON.parse(session.fixtureJson());
assert(roundTrip.manifestId === "nakagin" || roundTrip.manifest?.nodeKinds?.length > 0, "trinity wasm manifest hydrated");
console.log("[DEBUG] trinity wasm loadFixtureJson ok");

const puzzleJson = readFileSync(join(root, "puzzle/2d/fixture/nakagin-capsule-tower.2d.json"), "utf8");
const puzzleFixture = JSON.parse(puzzleJson);
assert(puzzleFixture.meta?.manifestId === "nakagin", "puzzle manifestId");

const { fixtureMetaKindCatalogBundle } = await import(join(root, "puzzle/2d/react/index.tsx"));
const catalog = fixtureMetaKindCatalogBundle(puzzleFixture.meta);
assert((catalog.nodes?.length ?? 0) > 0, "puzzle catalog nodes");
console.log("[DEBUG] puzzle 2d catalog nodes=" + (catalog.nodes?.length ?? 0));

const { DRAWLAYERS_LAYER_IDS, WRITERLANGUAGES_LANGUAGE_IDS } = await import(join(root, "mathematical/graph/manifest/core/index.ts"));
assert(DRAWLAYERS_LAYER_IDS.includes("shape"), "draw layers");
assert(WRITERLANGUAGES_LANGUAGE_IDS.includes("jack"), "writer languages");

console.log("[DEBUG] playground manifest fixture check passed");
