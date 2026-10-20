/** ⚖️ Runs the cache-correctness law (`cacheCorrectnessRepositoryFindingsV1`) over a merged root: the live tree with the manifests of an overlay substituted. `bun law.ts <overlay>`; prints every finding. */
import { cpSync, mkdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
const ws = "/Users/ueli/Documents/semio", overlay = process.argv[2]!, merged = "/private/tmp/bzl-nocache/law-root";
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const tracked = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z", "*📋️project.json"], { cwd: ws }).stdout.toString().split("\0").filter((f) => f && !f.startsWith(".🧬semio/"));
for (const f of [...tracked, "nx.json", `${L}/⚡️caching/🔣️policy.json`, `${L}/🔣️taxonomy.json`]) { mkdirSync(dirname(join(merged, f)), { recursive: true }); cpSync(join(ws, f), join(merged, f)); }
const changed = Bun.spawnSync(["find", overlay, "-name", "📋️project.json"]).stdout.toString().split("\n").filter(Boolean);
for (const file of changed) { const rel = file.slice(overlay.length + 1); cpSync(file, join(merged, rel)); }
const law = await import(join(ws, L, "⚖️laws/🔒️cache-correctness/🟦️.ts"));
const plugin = await import(join(ws, L, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await plugin.libraryBootstrap;
const records = law.repositoryCacheRecordsV1(merged, plugin.cacheInternals, tracked);
const findings = law.cacheCorrectnessRepositoryFindingsV1(records);
console.log(JSON.stringify({ records: records.length, findings: findings.length }));
for (const f of findings.slice(0, 40)) console.log(f.rule, f.id, f.detail.slice(0, 200));
