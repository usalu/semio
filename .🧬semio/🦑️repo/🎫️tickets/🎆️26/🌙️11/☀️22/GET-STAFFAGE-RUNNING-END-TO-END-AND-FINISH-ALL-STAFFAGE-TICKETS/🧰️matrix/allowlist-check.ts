/** 🗝️ Shows what the fixers' allowlist files would clear once merged into the fixture: `bun allowlist-check.ts <tests-dir> <file.json> [<file.json> …]` reads each file (an array of `derivedViewAllowlist` entries, or an object with `derivedViewAllowlist` and/or `bandCopyAllowlist`), and prints per asset the derived-view findings of every authored scale without and with the entries, and the entries that clear nothing. Nothing is written. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const [testsArg, ...files] = process.argv.slice(2) as [string, ...string[]];
const engine = await import(join(resolve(testsArg), "🔭️multiview-coverage", "🧰️.ts"));
const derived: any[] = [];
const bandCopy: any[] = [];
for (const file of files) {
  const document = JSON.parse(readFileSync(resolve(file), "utf8"));
  derived.push(...(Array.isArray(document) ? document : (document.derivedViewAllowlist ?? [])));
  bandCopy.push(...(Array.isArray(document) ? [] : (document.bandCopyAllowlist ?? [])));
}
const catalog = new Map<string, any>(engine.loadCatalog().map((asset: any) => [asset.id, asset]));
let before = 0;
let after = 0;
for (const id of [...new Set([...derived, ...bandCopy].map((entry) => entry.asset))].sort()) {
  const asset = catalog.get(id);
  if (asset === undefined) {
    console.log(`${id}: NOT IN THE CATALOG`);
    continue;
  }
  const keys = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
  const count = (allow: unknown): string[] => keys.flatMap((key) => engine.derivedFindings(asset, key, Number(key), undefined, allow).map((finding: any) => `${finding.check} 1:${key} ${finding.view} = ${finding.against}`));
  const without = count({ derived: [], bandCopy: [] });
  const withLists = count({ derived, bandCopy });
  before += without.length;
  after += withLists.length;
  console.log(`${id}: ${without.length} -> ${withLists.length}${withLists.length > 0 ? ` (left: ${withLists.join("; ")})` : ""}`);
}
console.log(`entries: ${derived.length} derivedViewAllowlist, ${bandCopy.length} bandCopyAllowlist; findings ${before} -> ${after}`);
