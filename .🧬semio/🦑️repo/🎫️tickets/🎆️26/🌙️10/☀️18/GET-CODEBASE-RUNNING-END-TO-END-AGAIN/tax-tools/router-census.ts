// 🔬 For every script rejected by the command-router validator, tests whether mechanical variants (new Error, merged register statements) would be accepted. usage: bun router-census.ts <inv.json>...
import {readFileSync} from "node:fs";
import {ecmaCommandRouterModule} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🛣️ecma-command-routes/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const paths = process.argv.slice(2).flatMap((f) => JSON.parse(readFileSync(f, "utf8")).violations.filter((v: any) => v.code === "fixed-source-disposition-unresolved").map((v: any) => v.path as string));
let total = 0, flips = 0; const flipped: string[] = [];
for (const p of paths) {
  total++;
  let src: string; try { src = readFileSync(p, "utf8"); } catch { continue; }
  const v1 = src.replace(/throw Error\(/g, "throw new Error(");
  if (v1 !== src && ecmaCommandRouterModule(v1, grammar)) { flips++; flipped.push(p); }
}
console.log(JSON.stringify({total, flipsWithNewError: flips}));
for (const p of flipped) console.log(p);
