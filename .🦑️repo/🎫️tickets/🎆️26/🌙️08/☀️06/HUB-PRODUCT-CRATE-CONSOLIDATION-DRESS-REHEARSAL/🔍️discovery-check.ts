#!/usr/bin/env bun
/**
 * 🔍️ Scratch verification (ticket 26/08/06/HUB-PRODUCT-CRATE-CONSOLIDATION-DRESS-REHEARSAL): proves
 * `discoverPackages()` actually resolves the merged `semio-hub` crate's `role = "hub"` marker
 * end-to-end — this discovery arm had zero real packages exercising the "hub" role value before this
 * merge (🌎️hub's area was, and remains, "legacy" in 🔣️taxonomy.json, so a markerless manifest there
 * is silently skipped rather than flagged — but a manifest WITH a marker is still fully resolved and
 * cataloged regardless of area maturity).
 */
import { discoverPackages } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🟦️discovery.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const packages = discoverPackages(repoRoot);
const hub = packages.filter((p) => p.ownerRel === "🌎️hub");

console.log(`Total discovered packages: ${packages.length}`);
console.log(`🌎️hub packages found: ${hub.length}`);
for (const p of hub) {
  console.log(JSON.stringify(p, null, 2));
}

if (hub.length !== 1) {
  console.error("FAIL: expected exactly 1 package under 🌎️hub");
  process.exit(1);
}
if (hub[0].role !== "hub") {
  console.error(`FAIL: expected role "hub", got "${hub[0].role}"`);
  process.exit(1);
}
if (hub[0].id !== "semio-hub") {
  console.error(`FAIL: expected id "semio-hub", got "${hub[0].id}"`);
  process.exit(1);
}
console.log("PASS: semio-hub discovered with role = \"hub\"");
