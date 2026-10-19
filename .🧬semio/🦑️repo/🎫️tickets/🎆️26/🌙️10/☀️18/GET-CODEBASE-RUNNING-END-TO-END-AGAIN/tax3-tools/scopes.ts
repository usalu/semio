// 🔬 Runs verifyTaxonomyScopesClosed over the listed scopes in ONE repository read and prints one JSON line per scope verdict. usage: bun scopes.ts <scope>...
import {verifyTaxonomyScopesClosed} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/✅️verification/🟦️.ts";
const scopes = process.argv.slice(2);
let last = "";
for (const v of verifyTaxonomyScopesClosed({repoRoot: "/Users/ueli/Documents/semio", scopes, progress: (p: any) => { const k = `${p.operation}/${p.phase}`; if (k !== last) { last = k; console.error(`[progress] ${k} ${p.current}/${p.total}`); } }} as any)) {
  console.log(JSON.stringify({scope: v.scope, clean: v.clean, closure: v.closure, violations: v.violations}));
}
console.error("[done]");
