/** 🩺️ Lists which recorded facts of a persisted inference observation no longer hold (first N) and the totals per kind.
 * argv: <cacheFile> [limit] */
import * as fs from "node:fs";
import { createHash } from "node:crypto";
const [cacheFile, limit = "20"] = process.argv.slice(2);
const text = fs.readFileSync(cacheFile, "utf8"), header = JSON.parse(text.slice(0, text.indexOf("\n")));
const kinds = new Map(), stale = [];
for (const { kind, path, fact } of header.observations) {
  kinds.set(kind, (kinds.get(kind) ?? 0) + 1);
  if (kind !== "file") continue;
  let ok;
  if (fact.error !== undefined) ok = !fs.existsSync(path);
  else { try { const s = fs.statSync(path); ok = s.size === fact.size && (s.mtimeMs === fact.mtimeMs || createHash("sha256").update(fs.readFileSync(path)).digest("hex") === fact.sha256); } catch { ok = false; } }
  if (!ok) stale.push(path);
}
console.log(`[DEBUG] observations=${header.observations.length} ${JSON.stringify(Object.fromEntries(kinds))} staleFiles=${stale.length}`);
for (const path of stale.slice(0, Number(limit))) console.log(`[DEBUG] stale ${path}`);
