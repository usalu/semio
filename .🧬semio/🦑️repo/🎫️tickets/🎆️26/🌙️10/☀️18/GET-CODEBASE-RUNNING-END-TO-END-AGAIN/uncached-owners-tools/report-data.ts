/** 📈 Cross-tabulates the census rows of both states for the report: the owner candidates and the host-state-by-name group of bzl-coverage, the classification scope, and the whole graph. Writes `🗑️generated/uncached-owners/report-data.json` and prints markdown tables. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const C = join(import.meta.dir, ".."), G = join(C, "🗑️generated/uncached-owners");
const before = JSON.parse(readFileSync(join(G, "census-before.json"), "utf8")), after = JSON.parse(readFileSync(join(G, "census-after.json"), "utf8"));
const b = new Map<string, any>(before.rows.map((r: any) => [r.id, r])), a = new Map<string, any>(after.rows.map((r: any) => [r.id, r]));
const cls = JSON.parse(readFileSync(join(import.meta.dir, "classification.json"), "utf8"));
const state = (r: any) => (r.reason ? `${r.disposition}:${r.reason}` : r.disposition);
const table = (title: string, ids: string[]) => {
  const m: Record<string, number> = {};
  for (const id of ids) { const k = `${state(b.get(id)!)} -> ${state(a.get(id)!)}`; m[k] = (m[k] ?? 0) + 1; }
  console.log(`\n### ${title} (${ids.length})`);
  for (const [k, v] of Object.entries(m).sort()) console.log(`| ${k} | ${v} |`);
  return m;
};
const md = readFileSync(join(C, "bzl-coverage-owner-candidates.md"), "utf8");
const candidates: string[] = []; let project = "";
for (const line of md.split("\n")) { const m = /^- `([^`]+)`: (.*)$/.exec(line); if (!m) continue; for (const t of m[2]!.matchAll(/`([^`]+)`/g)) candidates.push(`${m[1]}:${t[1]}`); }
const missing = candidates.filter((id) => !b.has(id));
const nonRust = (r: any) => !["cargo", "wasm", "aggregate"].includes(r.family);
const declaredNonRustBefore = [...b.values()].filter((r) => nonRust(r) && r.reason === "declared").map((r) => r.id);
const hostState = declaredNonRustBefore.filter((id) => !candidates.includes(id));
const out = {
  candidates: table("owner candidates (cache false, verification verb)", candidates.filter((id) => b.has(id))),
  hostState: table("host-state by name (non-Rust declared, not a candidate)", hostState),
  scope: table("classification scope (authored cache false without a decision)", cls.map((r: any) => r.id).filter((id: string) => b.has(id))),
  missing,
};
console.log("\ncandidates not found in the graph:", missing.length, missing.slice(0, 12));
const flipped = cls.filter((r: any) => r.disposition === "cached").map((r: any) => r.id);
console.log("flipped to cached:", flipped.length, "cached/test in the after census:", flipped.filter((id: string) => !a.get(id).reason).length);
writeFileSync(join(G, "report-data.json"), JSON.stringify(out, null, 1));
