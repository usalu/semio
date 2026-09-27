#!/usr/bin/env bun
/**
 * 📊️ R10 item 1: whole-repository taxonomy inventory census (read-only) — every violation as `code<TAB>path`, sorted, for
 * the live taxonomy or a candidate (`--taxonomy <path>`), so a candidate is proven to only remove violations.
 * Usage: bun taxonomy-census.ts [--taxonomy <candidate.json>] <out.tsv>
 */
import { writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const { inventoryTaxonomy } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`);
const args = process.argv.slice(2);
const at = args.indexOf("--taxonomy");
const taxonomyPath = at >= 0 ? args.splice(at, 2)[1] : undefined;
const started = performance.now();
let last = 0;
const inventory = inventoryTaxonomy({ repoRoot: ROOT, ...(taxonomyPath ? { taxonomyPath } : {}), progress: (progress: { operation: string; phase: string; current: number; total: number }) => {
  if (Date.now() - last < 30_000) return;
  last = Date.now();
  console.error(`[census] ${progress.operation}/${progress.phase} ${progress.current}/${progress.total}`);
} });
const rows = inventory.violations.map((violation: { code: string; path: string }) => `${violation.code}\t${violation.path}`).sort();
writeFileSync(args[0]!, `${rows.join("\n")}\n`);
const codes: Record<string, number> = {};
for (const violation of inventory.violations) codes[violation.code] = (codes[violation.code] ?? 0) + 1;
console.log(JSON.stringify({ seconds: Math.round((performance.now() - started) / 1000), entries: inventory.entries.length, violations: rows.length, codes }));
