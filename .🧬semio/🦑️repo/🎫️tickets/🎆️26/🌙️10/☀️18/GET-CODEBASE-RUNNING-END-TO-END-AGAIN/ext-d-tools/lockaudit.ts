import { dependencyJsLockAudit } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
const a = dependencyJsLockAudit(process.cwd(), () => 0);
const refused = a.mismatches.filter((m: any) => m.frozen);
console.log("scopes", a.scopes, "mismatches", a.mismatches.length, "frozen-refusals", refused.length);
for (const m of refused.slice(0, 20)) console.log(JSON.stringify(m).slice(0, 300));
