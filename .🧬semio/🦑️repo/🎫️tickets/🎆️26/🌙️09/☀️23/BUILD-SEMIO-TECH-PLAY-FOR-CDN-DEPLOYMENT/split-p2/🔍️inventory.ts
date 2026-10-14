#!/usr/bin/env bun
/**
 * 🔍️ Runs the canonical taxonomy normalization inventory over one scope and prints its violations, so a new
 * directory or package placement can be admitted before the cut lands (lane P2).
 *
 * usage: bun 🔍️inventory.ts <repo-relative scope> [codeRegex]
 */
import { resolve } from "node:path";
import { inventoryTaxonomy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const [scope, pattern] = process.argv.slice(2);
const started = Date.now();
const inventory = inventoryTaxonomy({ repoRoot, scope: scope!, workers: 1 });
const rows = inventory.entries.flatMap((entry) => entry.violations.map((violation) => `${violation.code}\t${entry.sourcePath}`)).filter((row) => !pattern || new RegExp(pattern, "u").test(row));
const counts = new Map<string, number>();
for (const row of rows) counts.set(row.split("\t")[0]!, (counts.get(row.split("\t")[0]!) ?? 0) + 1);
console.log(`[DEBUG] entries=${inventory.entries.length} violations=${rows.length} ms=${Date.now() - started}`);
for (const [code, count] of [...counts].sort()) console.log(`${count}\t${code}`);
for (const row of rows) console.log(row);
