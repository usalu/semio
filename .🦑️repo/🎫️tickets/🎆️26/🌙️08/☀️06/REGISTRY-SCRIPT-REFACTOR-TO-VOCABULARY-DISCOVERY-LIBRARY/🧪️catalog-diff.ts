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
