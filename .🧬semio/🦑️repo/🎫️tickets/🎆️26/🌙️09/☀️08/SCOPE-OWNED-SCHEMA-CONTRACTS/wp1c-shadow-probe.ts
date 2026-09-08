// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

/**
 * 🔬️ Runs the harness's catalog rules against a shadow root carrying the row-43 catalog shape.
 *
 * The live catalog is still the old array shape, so `test schema` can only report it malformed. This
 * probe answers the question the live run cannot yet: with the regenerated shape in place, what do the
 * resolution, completeness and `x-semio-formats` rules actually say about the real modules?
 *
 * Usage: bun wp1c-shadow-probe.ts <shadow root> [uri…]
 */
import { writeFileSync } from "node:fs";
import { readSchemaCatalog, resolveSchemaExport, schemaResolutionDiagnostics } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const [root, ...uris] = process.argv.slice(2);
if (root === undefined) throw new Error("usage: bun wp1c-shadow-probe.ts <shadow root> [uri…]");

const { catalog, diagnostics } = readSchemaCatalog(root);
console.log(`[DEBUG] catalog scopes ${catalog === null ? "none" : Object.keys(catalog.scopes).length} load diagnostics ${diagnostics.length}`);
for (const entry of diagnostics.slice(0, 5)) console.log(`[DEBUG]   ${entry.code} ${entry.detail}`);

for (const uri of uris) {
  const { resolved, diagnostics: refusals } = resolveSchemaExport(root, uri);
  console.log(`[DEBUG] ${uri} → ${resolved === null ? `REFUSED ${refusals.map((entry) => entry.code).join(", ")}` : `${resolved.path} ${resolved.digest.slice(0, 12)}`}`);
}

const found = schemaResolutionDiagnostics(root);
writeFileSync(new URL("🗑️generated/wp1c-shadow-findings.json", import.meta.url), `${JSON.stringify(found, null, 2)}\n`);
const counts = new Map<string, number>();
for (const entry of found) counts.set(entry.code, (counts.get(entry.code) ?? 0) + 1);
console.log(`[DEBUG] resolution findings ${found.length}`);
for (const [code, count] of [...counts].sort((a, b) => b[1] - a[1])) console.log(`[DEBUG]   ${String(count).padStart(6)} × ${code}`);
const owner = (path: string | null): string => {
  if (path === null) return "(none)";
  if (path.startsWith("✏️s/🔌️plugins/")) return path.split("/").slice(0, 3).join("/");
  return path.split("/").slice(0, 2).join("/");
};
const byOwner = new Map<string, Map<string, number>>();
for (const entry of found) {
  const bucket = byOwner.get(owner(entry.path)) ?? new Map<string, number>();
  bucket.set(entry.code, (bucket.get(entry.code) ?? 0) + 1);
  byOwner.set(owner(entry.path), bucket);
}
console.log("[DEBUG] by owner root:");
for (const [key, bucket] of [...byOwner].sort((a, b) => [...b[1].values()].reduce((x, y) => x + y, 0) - [...a[1].values()].reduce((x, y) => x + y, 0))) {
  console.log(`[DEBUG]   ${key}: ${[...bucket].sort((a, b) => b[1] - a[1]).map(([code, count]) => `${code}=${count}`).join(" ")}`);
}
const samples = new Map<string, string>();
for (const entry of found) if (!samples.has(entry.code)) samples.set(entry.code, `${entry.path ?? ""} — ${entry.detail}`);
console.log("[DEBUG] one example per code:");
for (const [code, sample] of [...samples].sort()) console.log(`[DEBUG]   ${code}: ${sample}`);
