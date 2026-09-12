#!/usr/bin/env bun
/** 🔍️ Runs one or more exported Node oracles of a bundle `📜️script.ts` WITHOUT going through its
 * argv router (which prints usage and exits when handed no command). Ticket-owned probe:
 * `bun 🔍️oracle-probe.ts <abs path to 📜️script.ts> <exportName…>`. */
const [scriptPath, ...exports] = process.argv.slice(2);
if (!scriptPath || exports.length === 0) {
  console.error("usage: bun 🔍️oracle-probe.ts <script.ts> <exportName…>");
  process.exit(2);
}
/** 🧯️ `runBundleScriptMain` returns immediately for the `policy` segment (it defers the real work to a
 * `setTimeout`), so a neutered timer lets the module finish evaluating and expose its oracles without
 * running any registered command. */
const realSetTimeout = globalThis.setTimeout;
process.argv = [process.argv[0]!, scriptPath, "policy"];
(globalThis as unknown as { setTimeout: unknown }).setTimeout = (() => 0) as unknown;
const module = (await import(scriptPath)) as Record<string, (repoRoot: string) => number>;
(globalThis as unknown as { setTimeout: typeof realSetTimeout }).setTimeout = realSetTimeout;

const repoRoot = "/Users/ueli/Documents/semio";
let failed = 0;
for (const name of exports) {
  const oracle = module[name];
  if (typeof oracle !== "function") {
    console.log(`${name}: MISSING EXPORT`);
    failed += 1;
    continue;
  }
  try {
    console.log(`${name}: checks=${oracle(repoRoot)} clean`);
  } catch (error) {
    failed += 1;
    console.log(`${name}: FAILED — ${(error as Error).message}`);
  }
}
process.exit(failed === 0 ? 0 : 1);
