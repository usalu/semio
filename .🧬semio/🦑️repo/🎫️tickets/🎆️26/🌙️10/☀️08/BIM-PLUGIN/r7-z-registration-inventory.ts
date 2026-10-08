import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const repoRoot = process.cwd();
const scope = process.argv[2]!;
const started = Date.now();
const inventory = inventoryTaxonomy({ repoRoot, scope });
console.error(`inventory ${inventory.entries.length} entries in ${Date.now() - started}ms`);
const dirs = inventory.entries.filter((e) => e.nodeKind === "directory");
const rows = dirs.map((e) => ({ path: e.sourcePath, kind: e.fileKind, stem: e.semanticStem, codes: e.violations.map((v) => v.code) }));
await Bun.write(process.argv[3]!, JSON.stringify(rows, null, 1));
