const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { reconcileBuildOwnerRegistry } = await import(`${L}/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts`);
const write = process.env.WRITE === "1";
const r = reconcileBuildOwnerRegistry("/Users/ueli/Documents/semio", { write });
console.log(`[sync] write=${write} removed=${r.removed.length} added=${r.added.length} renamed=${r.renamed.length} relocated=${r.relocated.length} repackaged=${r.repackaged.length} findings=${r.findings.length}`);
for (const x of r.relocated) console.log("[sync] relocated", JSON.stringify(x).slice(0, 300));
for (const x of r.removed) console.log("[sync] removed", JSON.stringify(x).slice(0, 300));
for (const x of r.added) console.log("[sync] added", JSON.stringify(x).slice(0, 300));
for (const f of r.findings) console.log("[sync] finding", f.kind, f.path ?? f.owner);
