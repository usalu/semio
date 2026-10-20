// 🧭️ Plans the normalization of one scope against HEAD with the live or the simulated engine and writes the plan JSON without touching the tree. usage: [TAX6_SIM=<scratch>] bun plan.ts <scope> <out.json>
import {writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
const C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const lib = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const L = process.env.TAX6_SIM ? `${C}/🗑️generated/${process.env.TAX6_SIM}/${lib}` : lib;
const N = `/Users/ueli/Documents/semio/${L}/🧹️normalization`;
const {inventoryTaxonomy} = await import(`${N}/🔍️inventory-planning/🟦️.ts`);
const {planTaxonomy} = await import(`${N}/🧠️planning/🟦️.ts`);
const scope = process.argv[2]!;
const baselineCommit = execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "rev-parse", "HEAD"], {encoding: "utf8"}).trim();
let lastAt = 0;
const progress = (p: any) => { const now = Date.now(); if (now - lastAt > 30000) { lastAt = now; console.error(`[progress] ${p.operation}/${p.phase} ${p.current}/${p.total}`); } };
const inventory = inventoryTaxonomy({repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: `${L}/🔣️taxonomy.json`, scope, progress});
const plan = planTaxonomy(inventory, {baselineCommit, excludedTreeDigests: [], progress});
writeFileSync(process.argv[3]!, JSON.stringify(plan, null, 1));
console.error(`[done] moves=${plan.moves.length} unresolved=${plan.unresolved.length} digest=${plan.planDigest}`);
for (const u of plan.unresolved.slice(0, 30)) console.error("[unresolved]", JSON.stringify(u).slice(0, 400));
