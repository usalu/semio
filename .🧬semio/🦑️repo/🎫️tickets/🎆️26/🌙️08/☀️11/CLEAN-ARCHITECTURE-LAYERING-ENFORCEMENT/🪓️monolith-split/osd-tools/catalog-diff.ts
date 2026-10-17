// 🗂️ c4p43: renders the schema catalog from the live tree (the exact `schema check` renderer) and prints a line diff against the tracked catalog text. Writes nothing. usage: bun catalog-diff.ts [out-file]
import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {inventorySchemaScopes, renderSchemaCatalog, loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const taxonomy = loadCatalogTaxonomy();
const rel = taxonomy.schemaExportResolution.catalogPath;
const rendered = renderSchemaCatalog(inventorySchemaScopes(root, taxonomy).catalog);
const tracked = readFileSync(join(root, rel), "utf8");
if (process.argv[2]) writeFileSync(process.argv[2], rendered);
const a = tracked.split("\n"), b = rendered.split("\n");
console.log("tracked lines", a.length, "rendered lines", b.length, "identical", tracked === rendered);
const setA = new Map<string, number>(), setB = new Map<string, number>();
for (const l of a) setA.set(l, (setA.get(l) ?? 0) + 1);
for (const l of b) setB.set(l, (setB.get(l) ?? 0) + 1);
let shown = 0;
for (const [l, c] of setA) if ((setB.get(l) ?? 0) < c && shown++ < 40) console.log("- tracked only:", c - (setB.get(l) ?? 0), l.slice(0, 140));
shown = 0;
for (const [l, c] of setB) if ((setA.get(l) ?? 0) < c && shown++ < 40) console.log("+ rendered only:", c - (setA.get(l) ?? 0), l.slice(0, 140));
