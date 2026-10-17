/** 🔒️ Transplants the semantic lock changes bun derives for this slice (changed workspace dependency lists, the new workspace, its package row) into the repo's own bun.lock text format. Dry-run by default.
 * Usage: bun lock.ts <scope-dir|.> <bun-generated.lock> [--write]
 * The generated lock comes from `bun install --lockfile-only --offline` in a scratch run whose output was moved aside; the checked-in lock is only ever edited here.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const [scope, generated] = process.argv.slice(2);
const write = process.argv.includes("--write");
const path = join(ROOT, scope, "bun.lock");
const text = readFileSync(path, "utf8");
const A = Bun.JSONC.parse(text), B = Bun.JSONC.parse(readFileSync(generated, "utf8"));
const roundtrip = JSON.stringify(A, null, 2) + "\n";
if (roundtrip !== text) { console.log(scope, "FAIL: checked-in lock is not JSON.stringify-stable; no edit"); process.exit(2); }
const cmpBytes = (a: string, b: string) => Buffer.from(a).compare(Buffer.from(b));
const orderedLike = (keys: string[]) => { const bytes = [...keys].sort(cmpBytes), locale = [...keys].sort((a, b) => a.localeCompare(b)); return { bytes: bytes.every((k, i) => k === keys[i]), locale: locale.every((k, i) => k === keys[i]) }; };
const insertAfterPredecessor = (obj: Record<string, unknown>, generatedKeys: string[], key: string, value: unknown) => {
  const entries = Object.entries(obj).filter(([k]) => k !== key);
  let at = 0;
  for (let i = generatedKeys.indexOf(key) - 1; i >= 0; i--) { const found = entries.findIndex(([k]) => k === generatedKeys[i]); if (found >= 0) { at = found + 1; break; } }
  entries.splice(at, 0, [key, value]);
  return Object.fromEntries(entries);
};
const wsOrder = orderedLike(Object.keys(A.workspaces).filter(k => k !== "")), pkOrder = orderedLike(Object.keys(A.packages));
const wsKeys = Object.keys(B.workspaces), pkKeys = Object.keys(B.packages);
let changed = 0, added = 0, addedPackages = 0;
const next = structuredClone(A);
for (const [k, v] of Object.entries<any>(B.workspaces)) {
  if (!(k in A.workspaces)) { next.workspaces = insertAfterPredecessor(next.workspaces, wsKeys, k, v); added++; continue; }
  if (JSON.stringify(A.workspaces[k]) !== JSON.stringify(v)) { next.workspaces[k] = v; changed++; }
}
for (const [k, v] of Object.entries<any>(B.packages)) if (!(k in A.packages)) { next.packages = insertAfterPredecessor(next.packages, pkKeys, k, v); addedPackages++; }
for (const k of Object.keys(A.workspaces)) if (!(k in B.workspaces)) console.log(scope, "NOTE: workspace only in checked-in lock", k.slice(-60));
for (const k of Object.keys(A.packages)) if (!(k in B.packages)) console.log(scope, "NOTE: package only in checked-in lock", k);
for (const k of Object.keys(A.packages)) if (k in B.packages && JSON.stringify(A.packages[k]) !== JSON.stringify(B.packages[k])) console.log(scope, "NOTE: package row differs", k);
const out = JSON.stringify(next, null, 2) + "\n";
console.log(scope.padEnd(14), `orderKnown ws=${wsOrder.bytes ? "bytes" : wsOrder.locale ? "locale" : "none"} pk=${pkOrder.bytes ? "bytes" : pkOrder.locale ? "locale" : "none"} workspacesChanged=${changed} workspacesAdded=${added} packagesAdded=${addedPackages} lines+${out.split("\n").length - text.split("\n").length}`);
if (write) writeFileSync(path, out);
