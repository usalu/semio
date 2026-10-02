/** 🧭️ Resumable scoped taxonomy report over `<scopes>`: runs the engine inventory behind `verify taxonomy report --scope` (kind, pairing, fixture, statute and path-budget findings; a non-canonical `normalizedPath` is reported as the plan's `normalization-move-required`) and appends one JSON row per scope to `<out>` with only the findings on paths new since the ticket baseline (`<new-dirs>`); stops starting scopes after the per-call budget. Scopes must contain both mutation roots of a subset, because case pairing reads in-scope entries only. */
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { inventoryTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";
const [scopesPath, newDirsPath, outPath, budgetSeconds = "420"] = process.argv.slice(2);
const repoRoot = process.cwd(), deadline = Date.now() + Number(budgetSeconds) * 1000;
const scopes = readFileSync(scopesPath!, "utf8").split("\n").filter(Boolean);
const fresh = new Set(readFileSync(newDirsPath!, "utf8").split("\n").filter(Boolean));
const done = new Set(existsSync(outPath!) ? readFileSync(outPath!, "utf8").split("\n").filter(Boolean).map((line) => JSON.parse(line).scope) : []);
const isFresh = (path: string): boolean => { for (let current = path; current.includes("/"); current = current.slice(0, current.lastIndexOf("/"))) if (fresh.has(current)) return true; return fresh.has(path); };
for (const scope of scopes) {
  if (done.has(scope)) continue;
  if (Date.now() > deadline) break;
  const started = Date.now();
  try {
    const inventory = inventoryTaxonomy({ repoRoot, scope, workers: 3 });
    const findings = [
      ...inventory.violations.filter((violation) => isFresh(violation.path)).map(({ severity, code, path, message }) => ({ severity, code, path, message })),
      ...inventory.entries.filter((entry) => isFresh(entry.sourcePath)).flatMap((entry) => [
        ...entry.violations.map(({ severity, code, message }) => ({ severity, code, path: entry.sourcePath, message })),
        ...(entry.normalizedPath !== entry.sourcePath ? [{ severity: "error", code: "normalization-move-required", path: entry.sourcePath, message: `Path must move to ${entry.normalizedPath}` }] : []),
      ]),
    ];
    const unique = [...new Map(findings.map((finding) => [`${finding.code}\0${finding.path}`, finding])).values()];
    appendFileSync(outPath!, JSON.stringify({ scope, seconds: Math.round((Date.now() - started) / 1000), entries: inventory.entries.length, findings: unique }) + "\n");
  } catch (error) {
    if (/discovery contract validation failed|Invalid taxonomy schema/u.test(String(error))) { console.log("[DEBUG] taxonomy invalid, stopping: " + String(error).slice(0, 300)); break; }
    appendFileSync(outPath!, JSON.stringify({ scope, seconds: Math.round((Date.now() - started) / 1000), crash: error instanceof Error ? error.message.slice(0, 2000) : String(error) }) + "\n");
  }
}
console.log(`[DEBUG] ${readFileSync(outPath!, "utf8").split("\n").filter(Boolean).length}/${scopes.length} scopes reported`);
