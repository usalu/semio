// 🔬 Clusters router-law rejections by the first rejecting validator line. usage: bun why.ts <list.txt> <out.json>
import {readFileSync, writeFileSync} from "node:fs";
import {ecmaCommandRouterModule, WHY} from "./routes-why.ts";
import {ecmaProgram} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/📦️packages/🟦️typescript/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const res: any[] = [];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const s = readFileSync(p, "utf8");
  WHY.length = 0;
  const parsed = !!ecmaProgram(s);
  const ok = parsed && ecmaCommandRouterModule(s, grammar);
  res.push({p, ok, parsed, why: [...WHY].slice(0, 6)});
}
writeFileSync(process.argv[3]!, JSON.stringify(res, null, 1));
const c = new Map<string, number>();
for (const r of res) { const k = !r.parsed ? "UNPARSEABLE" : r.ok ? "ok" : String(r.why[0] ?? "noreason"); c.set(k, (c.get(k) ?? 0) + 1); }
console.log([...c].sort((a, b) => b[1] - a[1]));
