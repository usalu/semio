#!/usr/bin/env bun
/** 🔣️ ST2 runner: `wp-coord/taxonomy-load-probe.ts` against any workspace root (the overlay) — loads the root's own taxonomy. */
import { join } from "node:path";

const root = process.argv[2] ?? "/Users/ueli/Documents/semio";
const { loadTaxonomy, loadCatalogTaxonomy } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
const started = performance.now();
loadTaxonomy();
loadCatalogTaxonomy();
console.log(`🔣️ taxonomy valid in ${Math.round(performance.now() - started)} ms (${root})`);
