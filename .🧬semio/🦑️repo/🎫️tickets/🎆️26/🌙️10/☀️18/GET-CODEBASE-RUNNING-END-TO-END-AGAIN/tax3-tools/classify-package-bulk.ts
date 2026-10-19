// 🔬 Buckets package source files by live classifier role and evidence. usage: bun classify-package-bulk.ts <lang> <listfile>
import {readFileSync} from "node:fs";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import {classifyPackageSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
const t = loadCatalogTaxonomy() as any;
const [lang, list] = process.argv.slice(2);
const rule = Object.entries(t.packageBoundaryRules).find(([id]) => id.includes(lang === "rust" ? "🦀" : lang === "go" ? "🐹" : "🟦")) as any;
const grammar = t.packageGlueGrammar[rule[1].glueGrammarId];
const buckets = new Map<string, string[]>();
for (const f of readFileSync(list, "utf8").split("\n").filter(Boolean)) { const d = classifyPackageSource(readFileSync("/Users/ueli/Documents/semio/" + f, "utf8"), grammar); const k = d.role + " | " + d.evidence; buckets.set(k, [...(buckets.get(k) ?? []), f]); }
for (const [k, v] of [...buckets].sort((a, b) => b[1].length - a[1].length)) console.log(v.length, k, "e.g.", v[0].slice(-70));
