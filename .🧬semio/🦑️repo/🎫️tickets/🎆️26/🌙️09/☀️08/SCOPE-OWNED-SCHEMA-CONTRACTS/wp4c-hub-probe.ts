/**
 * 🔬️ WP4c hub probe — the schema-contract findings of the `🌎️hub` partition only.
 *
 * `test schema --under 🌎️hub` cannot answer this: `schemaContractDiagnostics` runs
 * `schemaResolutionDiagnostics` (which owns `schema-export-incomplete`) only when `under` is empty,
 * because resolution is a whole-catalog question. This probe runs the same catalog pass and keeps the
 * rows whose scope or path belongs to the partition.
 *
 * bun <this file> [<repo root>]
 */
import { schemaResolutionDiagnostics } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = process.argv[2] ?? process.cwd();
const all = schemaResolutionDiagnostics(repoRoot);
const mine = all.filter((entry) => (entry.path ?? "").startsWith("🌎️hub") || (entry.scope ?? "").startsWith("hub."));
if (process.argv.includes("--json")) {
  console.log(JSON.stringify(mine, null, 2));
} else {
  const byCode = new Map<string, number>();
  for (const entry of mine) byCode.set(entry.code, (byCode.get(entry.code) ?? 0) + 1);
  console.log(`[wp4c-hub] ${mine.length} finding(s) of ${all.length} repo-wide`);
  for (const [code, count] of [...byCode].sort((a, b) => b[1] - a[1])) console.log(`[wp4c-hub]   ${String(count).padStart(4)} × ${code}`);
  for (const entry of mine) console.log(`[wp4c-hub]   ${entry.code} | ${entry.scope ?? ""} | ${entry.export ?? ""} | ${entry.format ?? ""} | ${entry.path ?? ""}`);
}
