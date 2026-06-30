#!/usr/bin/env bun
/** @emoji 🛡️ Runtime check: compile-time graph manifests load and validate sample fixtures. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  MANIFEST_IDS,
  manifestById,
  nakaginManifestCatalogBundle,
  puzzle2d_defaultManifestCatalogBundle,
  WRITERLANGUAGES_LANGUAGE_IDS,
  DRAWLAYERS_LAYER_IDS,
} from "../../../../../../mathematical/graph/manifest/core/index.ts";

const root = join(import.meta.dir, "../../../../../../");

function assert(cond, msg) {
  if (!cond) throw new Error(msg);
}

console.log("[DEBUG] manifest ids:", MANIFEST_IDS.join(", "));
assert(MANIFEST_IDS.includes("nakagin"), "nakagin manifest registered");
assert(manifestById("nakagin")?.id === "nakagin", "manifestById nakagin");
const nakaginCatalog = nakaginManifestCatalogBundle();
assert((nakaginCatalog.nodes?.length ?? 0) > 40, "nakagin catalog nodes");
assert(puzzle2d_defaultManifestCatalogBundle().handles?.some((h) => h.id === "port"), "default port handle");

const trinityFixture = JSON.parse(readFileSync(join(root, "trinity/fixture/nakagin-capsule-tower.trinity.json"), "utf8"));
assert(trinityFixture.manifestId === "nakagin", "trinity fixture manifestId");
assert(!trinityFixture.manifest, "trinity inline manifest removed");

const puzzleFixture = JSON.parse(readFileSync(join(root, "puzzle/2d/fixture/nakagin-capsule-tower.2d.json"), "utf8"));
assert(puzzleFixture.meta?.manifestId === "nakagin", "puzzle fixture manifestId");
assert(!puzzleFixture.meta?.kindCatalogs, "puzzle inline kindCatalogs removed");

assert(WRITERLANGUAGES_LANGUAGE_IDS.includes("jack"), "writer jack language");
assert(DRAWLAYERS_LAYER_IDS.includes("shape"), "draw shape layer");

console.log("[DEBUG] graph manifest compile-time check passed");
