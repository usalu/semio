// 🔍️ Runs the repo taxonomy verifier over one scope directly from the normalization module (the root 📜️script.ts is not importable while peers churn it).
import {verifyTaxonomyScopesClosed} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const scope = process.argv[2]!;
const started = Date.now();
const out = verifyTaxonomyScopesClosed({repoRoot: "/Users/ueli/Documents/semio", scopes: [scope]});
for (const verification of out) {
  const errors = verification.violations.filter(v => v.severity === "error");
  console.log(`clean=${verification.clean} errors=${errors.length} warnings=${verification.violations.length - errors.length} scope=${verification.scope} ms=${Date.now() - started}`);
  const byCode = new Map<string, number>();
  for (const v of verification.violations) byCode.set(v.severity + " " + v.code, (byCode.get(v.severity + " " + v.code) ?? 0) + 1);
  for (const [code, n] of byCode) console.log("code", code, n);
  for (const v of verification.violations.slice(0, Number(process.env.SHOW ?? 20))) console.log(v.severity, v.code, v.path, "::", v.message.slice(0, 200));
}
