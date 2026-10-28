/** ⏲️ Real-host witness of batch `clock`: bun slowhost.ts <execution-module> <budgetMs> <childMs> [hang] — runs a node child that needs childMs of wall time (or hangs) under the module's budget on THIS host's real load, prints the outcome as JSON. */
import { loadavg, availableParallelism } from "node:os";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { join } from "node:path";
const [module, budget, child, mode] = process.argv.slice(2) as [string, string, string, string?];
const api = await import(module);
const art = process.env.SEMIO_TEST_ARTIFACT_DIR!; mkdirSync(art, { recursive: true });
const directory = mkdtempSync(join(art, "slowhost-"));
const started = performance.now(); let outcome = "survived"; const lines: string[] = [];
const original = console.error; console.error = (...v: unknown[]) => { lines.push(v.map(String).join(" ")); };
try { await api.runBudgetedTestCommand("node", ["-e", mode === "hang" ? "setInterval(()=>{},1000)" : `setTimeout(()=>{},${child})`], { cwd: directory, budgetMs: Number(budget), throwOnFailure: true }); } catch (error) { outcome = String(error); }
console.error = original; rmSync(directory, { recursive: true, force: true });
console.log(JSON.stringify({ module: module.includes("/sm/") ? "mirror(new)" : "live(base)", load1: loadavg()[0], cores: availableParallelism(), budgetMs: Number(budget), childMs: Number(child), mode: mode ?? "finite", outcome, wallMs: Math.round(performance.now() - started), notes: lines.filter(l => l.includes("Busy host")).slice(0, 1).map(l => l.slice(0, 120)) }));
