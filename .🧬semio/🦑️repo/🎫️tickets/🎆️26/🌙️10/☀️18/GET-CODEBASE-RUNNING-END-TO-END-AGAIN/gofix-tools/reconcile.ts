import { join, resolve } from "node:path";
const root = resolve(process.argv[2] ?? "");
const scope = process.argv.slice(3);
const { reconcileBuildOwnerRegistry } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts"));
const result = reconcileBuildOwnerRegistry(root, { scope: scope.length ? scope : undefined, write: process.env.GOFIX_WRITE === "1" });
console.log(JSON.stringify({ changed: result.changed, findings: result.findings }, null, 1));
