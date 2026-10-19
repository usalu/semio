// 🔬 Classifies package source files with the live glue grammar and prints role and evidence. usage: bun classify-package.ts <lang: rust|go|typescript> <file>...
import {readFileSync} from "node:fs";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import {classifyPackageSource} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🏷️package-source-classification/🟦️.ts";
const t = loadCatalogTaxonomy() as any;
const [lang, ...files] = process.argv.slice(2);
const rule = Object.entries(t.packageBoundaryRules).find(([id]) => id.includes(lang === "rust" ? "🦀" : lang === "go" ? "🐹" : "🟦")) as any;
const grammar = t.packageGlueGrammar[rule[1].glueGrammarId];
for (const f of files) { const d = classifyPackageSource(readFileSync(f, "utf8"), grammar); console.log(d.role, "|", d.evidence, "|", f.slice(-80)); }
