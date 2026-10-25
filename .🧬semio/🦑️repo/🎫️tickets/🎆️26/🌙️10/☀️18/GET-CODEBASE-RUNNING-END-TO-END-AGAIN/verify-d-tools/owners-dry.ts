import { reconcileBuildOwnerRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const write = process.argv[2] === "write";
const result = reconcileBuildOwnerRegistry("/Users/ueli/Documents/semio", { write });
console.log(JSON.stringify({ write, removed: result.removed, added: result.added, renamed: result.renamed, relocated: result.relocated, repackaged: result.repackaged, findings: result.findings }, null, 1));
