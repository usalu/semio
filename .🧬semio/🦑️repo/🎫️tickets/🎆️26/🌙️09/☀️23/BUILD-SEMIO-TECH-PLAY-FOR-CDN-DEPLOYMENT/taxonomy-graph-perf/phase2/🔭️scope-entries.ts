/** 🔭️ Prints every inventoried entry of one scope whose normalized path differs from its source path, plus its violations.
 * argv: <repoRoot> <scope> */
const [repoRoot, scope] = process.argv.slice(2);
const normalization = await import(`${repoRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`);
const inventory = normalization.inventoryTaxonomy({ repoRoot, scope });
for (const entry of inventory.entries) if (entry.sourcePath !== entry.normalizedPath || entry.violations?.length) console.log(`[DEBUG] ${entry.sourcePath} -> ${entry.normalizedPath} kind=${entry.fileKind ?? entry.nodeKind} ${(entry.violations ?? []).map((row: { code: string }) => row.code).join(",")}`);
for (const row of inventory.violations.slice(0, 40)) console.log(`[DEBUG] violation ${row.code} ${row.path} ${String(row.message).slice(0, 200)}`);
