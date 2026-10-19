// 🧾️ rg-plug owner-registry reconcile for one scope: bun owners.ts <scope-rel> <1|0 write>
import { reconcileBuildOwnerRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const r = reconcileBuildOwnerRegistry("/Users/ueli/Documents/semio", { scope: process.argv[2]!, write: process.argv[3] === "1" });
console.log(JSON.stringify({ changed: r.changed, added: r.added.map((x: any) => x.name), renamed: r.renamed, relocated: r.relocated, removed: r.removed, findings: r.findings.length }));
