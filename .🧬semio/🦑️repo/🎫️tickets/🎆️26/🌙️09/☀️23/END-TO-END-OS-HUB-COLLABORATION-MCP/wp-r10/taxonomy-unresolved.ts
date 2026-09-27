#!/usr/bin/env bun
/**
 * 🗺️ R10 item 1: inventories taxonomy directory kinds for several scopes (read-only) against the live taxonomy or a
 * candidate file (`--taxonomy <path>`), and prints every violation grouped by code plus the unresolved directory list.
 * Usage: bun taxonomy-unresolved.ts [--taxonomy <candidate.json>] [--json <out>] <scope…>
 */
import { writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const { inventoryTaxonomy } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`);
const args = process.argv.slice(2);
const option = (name: string) => { const index = args.indexOf(name); if (index < 0) return undefined; const value = args[index + 1]; args.splice(index, 2); return value; };
const taxonomyPath = option("--taxonomy");
const jsonOut = option("--json");
const result: Record<string, { ms: number; entries: number; codes: Record<string, number>; unresolved: string[]; other: string[] }> = {};
for (const scope of args) {
  const started = performance.now();
  const inventory = inventoryTaxonomy({ repoRoot: ROOT, scope, ...(taxonomyPath ? { taxonomyPath } : {}) });
  const codes: Record<string, number> = {};
  for (const violation of inventory.violations) codes[violation.code] = (codes[violation.code] ?? 0) + 1;
  const unresolved = inventory.violations.filter((violation: { code: string }) => violation.code === "directory-kind-unresolved" || violation.code === "directory-kind-ambiguous").map((violation: { code: string; path: string; message: string }) => `${violation.code} ${violation.path} ${violation.code === "directory-kind-ambiguous" ? violation.message : ""}`.trim()).sort();
  const other = inventory.violations.filter((violation: { code: string }) => violation.code !== "directory-kind-unresolved" && violation.code !== "directory-kind-ambiguous").map((violation: { code: string; path: string; message: string }) => `${violation.code} ${violation.path}: ${violation.message}`).sort();
  result[scope] = { ms: Math.round(performance.now() - started), entries: inventory.entries.length, codes, unresolved, other };
  console.log(`[taxonomy-unresolved] scope=${scope} ms=${result[scope].ms} entries=${inventory.entries.length} codes=${JSON.stringify(codes)}`);
  for (const line of unresolved) console.log(`  ${line}`);
  for (const line of other) console.log(`  other ${line}`);
}
if (jsonOut) writeFileSync(jsonOut, JSON.stringify(result, null, 2));
