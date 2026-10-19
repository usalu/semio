// 🔬 Runs an instrumented copy of the command-router validator and prints the validator source lines at which each refusal fired (last ones first). usage: bun trace.ts <file>
import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {C, LIB, REPO} from "./vfs.ts";
const src = readFileSync(`${REPO}/${LIB}/🔍️discovery/🛣️ecma-command-routes/🟦️.ts`, "utf8").split("\n");
const out = src.map((line, i) => line.replace(/return false/g, `return __f(${i + 1})`).replace(/return "invalid"/g, `return __i(${i + 1})`).replace(/\? "data" : "invalid"/g, `? "data" : __i(${i + 1})`).replace(/\? "closure" : "invalid"/g, `? "closure" : __i(${i + 1})`).replace(/\? "finite" : "invalid"/g, `? "finite" : __i(${i + 1})`).replace(/\? "receipt" : "invalid"/g, `? "receipt" : __i(${i + 1})`).replace(/\? "router" : "invalid"/g, `? "router" : __i(${i + 1})`));
const dir = `${C}/🗑️generated/tax-stage`;
mkdirSync(dir, {recursive: true});
writeFileSync(`${dir}/trace-validator.ts`, `export const hits: number[] = [];\nconst __f = (n: number): false => { hits.push(n); return false; };\nconst __i = (n: number): "invalid" => { hits.push(n); return "invalid"; };\n` + out.join("\n").replace(/from "\.\.\/📜️taxonomy-contracts\/🟦️\.ts"/, `from "${REPO}/${LIB}/🔍️discovery/📜️taxonomy-contracts/🟦️.ts"`));
const mod = await import(`${dir}/trace-validator.ts`);
const {loadCatalogTaxonomy} = await import(`${REPO}/${LIB}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const text = readFileSync(process.argv[2]!, "utf8");
mod.hits.length = 0;
const ok = mod.ecmaCommandRouterModule(text, grammar);
console.log("accepted:", ok);
const lines = readFileSync(`${dir}/trace-validator.ts`, "utf8").split("\n");
const seen = new Set<number>();
for (const h of [...mod.hits].reverse()) { if (seen.has(h)) continue; seen.add(h); console.log(`  validator line ${h}: ${src[h - 1]!.trim().slice(0, 170)}`); if (seen.size >= 12) break; }
