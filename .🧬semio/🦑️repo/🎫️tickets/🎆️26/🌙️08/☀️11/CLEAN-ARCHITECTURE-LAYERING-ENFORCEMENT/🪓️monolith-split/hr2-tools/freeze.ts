/** 🔒️ hr2 freeze probe: prints the new and removed third-party entries against the committed baseline (read-only). Usage: bun freeze.ts [--json out.json] */
import { writeFileSync } from "node:fs";
import { dependencyFreezeCheck } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
const result = dependencyFreezeCheck("/Users/ueli/Documents/semio");
console.log(`baseline=${result.baseline.entries.length} current=${result.current.length} new=${result.newDeps.length} removed=${result.removedDeps.length}`);
for (const e of result.newDeps) console.log("NEW", e.ecosystem, e.name, e.version, e.kinds.join(","), e.users.slice(0, 3).join(" | ").slice(0, 160));
for (const e of result.removedDeps) console.log("REMOVED", e.ecosystem, e.name);
const out = process.argv[3];
if (out) writeFileSync(out, JSON.stringify({ newDeps: result.newDeps, removedDeps: result.removedDeps }, null, 1));
