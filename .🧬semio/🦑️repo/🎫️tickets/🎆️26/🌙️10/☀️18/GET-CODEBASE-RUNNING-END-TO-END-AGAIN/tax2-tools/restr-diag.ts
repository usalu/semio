// 🔬 Shows why restructured-but-still-rejected scripts fail. usage: bun restr-diag.ts <list.txt>
import {readFileSync} from "node:fs";
import {planRestructure} from "./restructure.ts";
import {ecmaCommandRouterModule, SPOT} from "./routes-why.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const g = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8");
  const plan = planRestructure(p, src);
  if (!plan.ok) continue;
  SPOT.length = 0;
  if (ecmaCommandRouterModule(plan.script, g)) continue;
  const sp = SPOT.filter((x) => x.line !== 285 && x.line !== 286).at(0) ?? SPOT[0];
  console.log(p.slice(-70).padEnd(70), "L" + sp?.line, (sp?.stmt ? plan.script.slice(sp.stmt[0], sp.stmt[1]) : "<no spot>").replace(/\s+/g, " ").slice(0, 160));
}
