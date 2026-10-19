// 🔬 Prints the first rejecting validator lines for source text. usage: bun why-one.ts <file>
import {readFileSync} from "node:fs";
import {ecmaCommandRouterModule, WHY} from "./routes-why.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const g = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const ok = ecmaCommandRouterModule(readFileSync(process.argv[2]!, "utf8"), g);
console.log(ok, WHY.slice(0, 8));
