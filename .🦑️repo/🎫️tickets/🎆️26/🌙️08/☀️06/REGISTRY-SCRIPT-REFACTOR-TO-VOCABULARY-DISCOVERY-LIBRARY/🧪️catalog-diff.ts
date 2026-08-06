#!/usr/bin/env bun
/** 🧪️ Ticket-local before/after harness: renders the plugin+playground catalog with the pre-refactor
 * baseline copy of the registry script and with the live (refactored) one, and diffs both JSON views.
 * Never writes into `🤖️generated/` — verification only. */
import { getWorkspaceRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import * as baseline from "./📜️baseline-script.ts";
import * as current from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts";

const root = getWorkspaceRoot();
const render = (mod: typeof baseline) => ({
  plugins: mod.generatePluginRegistry(root),
  playgrounds: mod.generatePlaygroundRegistry(root),
});

const before = render(baseline);
const after = render(current as unknown as typeof baseline);

const dump = (label: string, value: unknown) => `${label}\n${JSON.stringify(value, null, 2)}\n`;
await Bun.write(`${import.meta.dir}/📋️catalog-before.json`, dump("", before).trim() + "\n");
await Bun.write(`${import.meta.dir}/📋️catalog-after.json`, dump("", after).trim() + "\n");

const ids = (rows: readonly { pluginId: string }[]) => rows.map((row) => row.pluginId).sort();
const variants = (rows: readonly { variant: string }[]) => rows.map((row) => row.variant).sort();
console.log(`plugins  before=${before.plugins.length} after=${after.plugins.length}`);
console.log(`playgrounds before=${before.playgrounds.length} after=${after.playgrounds.length}`);
const onlyBefore = ids(before.plugins).filter((id) => !ids(after.plugins).includes(id));
const onlyAfter = ids(after.plugins).filter((id) => !ids(before.plugins).includes(id));
console.log(`plugin ids only-before: ${JSON.stringify(onlyBefore)}`);
console.log(`plugin ids only-after:  ${JSON.stringify(onlyAfter)}`);
console.log(`playground variants only-before: ${JSON.stringify(variants(before.playgrounds).filter((v) => !variants(after.playgrounds).includes(v)))}`);
console.log(`playground variants only-after:  ${JSON.stringify(variants(after.playgrounds).filter((v) => !variants(before.playgrounds).includes(v)))}`);
const same = JSON.stringify(before) === JSON.stringify(after);
console.log(same ? "IDENTICAL catalogs (byte-for-byte JSON)" : "DIFFERENT — inspect 📋️catalog-before.json vs 📋️catalog-after.json");

// 🔬️ Per-row equality for everything present on BOTH sides, keyed by (pluginId, packageName) so a
// crate that physically moved on disk mid-run is compared on its identity fields rather than its path.
const key = (row: { pluginId: string; packageName: string }) => `${row.pluginId}|${row.packageName}`;
const beforeByKey = new Map(before.plugins.map((row) => [key(row), row]));
const afterByKey = new Map(after.plugins.map((row) => [key(row), row]));
let mismatches = 0;
for (const [k, beforeRow] of beforeByKey) {
  const afterRow = afterByKey.get(k);
  if (!afterRow) {
    console.log(`DROPPED  ${k}  (${beforeRow.cratePath})`);
    mismatches++;
    continue;
  }
  const strip = (row: typeof beforeRow) => ({ ...row, cratePath: undefined });
  if (JSON.stringify(strip(beforeRow)) !== JSON.stringify(strip(afterRow))) {
    console.log(`CHANGED  ${k}\n  before ${JSON.stringify(beforeRow)}\n  after  ${JSON.stringify(afterRow)}`);
    mismatches++;
  } else if (beforeRow.cratePath !== afterRow.cratePath) {
    console.log(`MOVED    ${k}  ${beforeRow.cratePath} -> ${afterRow.cratePath}`);
  }
}
for (const [k, afterRow] of afterByKey) if (!beforeByKey.has(k)) console.log(`RECOVERED ${k}  (${afterRow.cratePath})`);
console.log(`shared rows with a real field mismatch: ${mismatches}`);

const pgKey = (row: { variant: string; pluginId: string }) => `${row.pluginId}|${row.variant}`;
const beforePg = new Map(before.playgrounds.map((row) => [pgKey(row), row]));
let pgMismatches = 0;
for (const row of after.playgrounds) {
  const beforeRow = beforePg.get(pgKey(row));
  if (!beforeRow) {
    console.log(`PLAYGROUND-NEW  ${pgKey(row)}`);
    pgMismatches++;
    continue;
  }
  const strip = (r: typeof row) => ({ ...r, cratePath: undefined });
  if (JSON.stringify(strip(beforeRow)) !== JSON.stringify(strip(row))) {
    console.log(`PLAYGROUND-CHANGED ${pgKey(row)}\n  before ${JSON.stringify(beforeRow)}\n  after  ${JSON.stringify(row)}`);
    pgMismatches++;
  }
}
console.log(`playground rows with a real field mismatch: ${pgMismatches}`);
