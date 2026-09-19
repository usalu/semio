/** 🔍️ Cross-checks V3b's `repo-test-adapter` classification against the repository test platform's
 * own `discoverTestCases`, so the claim "this leaf's compilation owner is the generated cache-local
 * host crate" rests on the discovery that actually materializes those hosts, not on a regex.
 *
 * Usage: `bun 🐍️v3b-adapter-crosscheck.ts <repoRoot> <census.txt>`
 */
import { readFileSync } from "node:fs";
import { join, relative } from "node:path";

const repoRoot = process.argv[2]!;
const censusPath = process.argv[3]!;

const { discoverTestCases } = (await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts"))) as {
  discoverTestCases: (root: string) => { caseDir: string; adapters: Record<string, string> }[];
};

const cases = discoverTestCases(repoRoot);
const rustAdapters = new Set(cases.map((entry) => entry.adapters["rust"]).filter((value): value is string => typeof value === "string"));
console.log(`discoverTestCases: cases=${cases.length} rustAdapters=${rustAdapters.size}`);
if (cases.length === 0) throw new Error("discovery returned zero cases — refusing to judge on an empty set");

const lines = readFileSync(censusPath, "utf8").split("\n");
const rows: string[] = [];
let inside = false;
for (const line of lines) {
  if (line === "=== repo-test-adapter rows ===") {
    inside = true;
    continue;
  }
  if (inside && line.startsWith("=== ")) break;
  if (inside && line.includes("\t")) rows.push(line);
}
if (rows.length === 0) throw new Error("census has no populated `repo-test-adapter rows` section");

let matched = 0;
const misses: string[] = [];
for (const row of rows) {
  const [plugin, rel] = row.split("\t") as [string, string];
  const repoRel = relative(repoRoot, join(repoRoot, "✏️s/🔌️plugins", plugin, rel)).replaceAll("\\", "/");
  if (rustAdapters.has(repoRel)) matched++;
  else misses.push(repoRel);
}
console.log(`claimed=${rows.length} confirmedByDiscovery=${matched} misses=${misses.length}`);
for (const miss of misses.slice(0, 20)) console.log("  MISS " + miss);
