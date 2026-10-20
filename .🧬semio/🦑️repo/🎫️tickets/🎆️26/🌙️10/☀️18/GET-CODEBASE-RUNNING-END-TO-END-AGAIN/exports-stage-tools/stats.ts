/**
 * 📈 Prints the numbers of the result report from batch.json and the generated analysis files.
 *
 *   bun stats.ts
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, TICKET } from "./lib.ts";
import { packageOfFile, reaches } from "./cycles.ts";

const batch = JSON.parse(readFileSync(resolve(TICKET, "exports-stage-tools/batch.json"), "utf8"));
const cands = JSON.parse(readFileSync(resolve(GENERATED, "candidates.json"), "utf8")).candidates as any[];
const sim = JSON.parse(readFileSync(resolve(GENERATED, "simulation.json"), "utf8"));
const E = batch.entries as any[];
const exp = E.filter(e => e.field === "exports");
console.log("summary", JSON.stringify(batch.summary));
console.log("export adds applied:", exp.filter(e => e.op === "add" && e.apply).length, "removes applied:", exp.filter(e => e.op === "remove" && e.apply).length, "blocked:", exp.filter(e => !e.apply).length);
const by = new Map<string, { keys: number; edges: number }>();
for (const e of exp.filter(e => e.op === "add" && e.apply)) { const r = by.get(e.package) ?? by.set(e.package, { keys: 0, edges: 0 }).get(e.package)!; r.keys++; r.edges += e.justification.importerEdges ?? 0; }
console.log("packages receiving keys:", by.size);
for (const [k, r] of [...by].sort((a, b) => b[1].keys - a[1].keys).slice(0, 14)) console.log("  ", String(r.keys).padStart(4), String(r.edges).padStart(5), k);
const mf = new Set(E.filter(e => e.apply && ["export", "move", "removal", "third-party", "types", "requested", "tst6"].includes(e.tier)).map(e => e.packageJson));
console.log("manifests touched by default:", mf.size);
const deps = E.filter(e => e.field !== "exports");
const dt = new Map<string, number>(); for (const d of deps) dt.set(d.tier + (d.apply ? "" : "/off"), (dt.get(d.tier + (d.apply ? "" : "/off")) ?? 0) + 1);
console.log("dependency entries by tier:", [...dt].map(x => x.join("=")).join(" "));
const ui = cands.filter(c => c.conflicts.some((x: string) => x.startsWith("ui-no-framework-packages")));
console.log("candidates whose alias would trip ui-no-framework-packages (glue form of the ui module missing in the allow-list):", ui.length, "edges", ui.reduce((n, c) => n + c.edges, 0));
const carried = new Map<string, number>(); for (const c of cands) for (const r of c.carriedRules) carried.set(r, (carried.get(r) ?? 0) + c.edges);
console.log("rules that keep firing after the alias (edges):", [...carried].map(x => x.join("=")).join(" "));
const cyc = new Map<string, number>();
for (const c of cands) for (const f of c.importers) { const mo = packageOfFile(f); if (mo && mo !== c.pkg && reaches(c.pkg, mo)) cyc.set(mo + " <- " + c.pkg, (cyc.get(mo + " <- " + c.pkg) ?? 0) + 1); }
console.log("importer package <- entry package pairs that would close a declared-dependency cycle:", [...cyc].map(x => x.join(" x")).join("; "));
console.log("simulation:", JSON.stringify(sim.byClass), "cruise-style", sim.cruiseStyleNow, "->", sim.cruiseStyleAfterBatchAndRewrite);
