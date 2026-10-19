import { policyBannedNameStemBreaches } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
const rows = policyBannedNameStemBreaches(process.cwd());
const byStem: Record<string, number> = {};
for (const r of rows) { const m = /stem "([^"]+)"/.exec(r.summary); byStem[m![1]] = (byStem[m![1]] ?? 0) + 1; }
console.log(JSON.stringify({ total: rows.length, byStem }));
console.log(rows.filter(r => /"base"/.test(r.summary)).slice(0, 5).map(r => r.scope).join("\n"));
