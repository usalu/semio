#!/usr/bin/env bun
/** ⚖️ ST2 runner: the nx-contract policy vectors (the cache-contracts loop) against one workspace's 🟨️.mjs + policy. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const workspace = process.argv[2];
const caching = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching");
const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
const policy = JSON.parse(readFileSync(join(caching, "🔣️policy.json"), "utf8"));
const vectors = JSON.parse(readFileSync(join(caching, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
let rows = 0;
for (const row of vectors.policies) {
  const command = `bun ./📜️script.ts ${row.target}`;
  const enabled = cacheInternals.targetPolicy(row.target, { cache: true, options: { command } }, policy);
  const disabled = cacheInternals.targetPolicy(row.target, { cache: false, options: { command } }, policy);
  assert.deepEqual({ cache: enabled.cache, continuous: enabled.continuous ?? false }, { cache: row.cache, continuous: row.continuous }, row.target);
  assert.equal(disabled.cache, row.authored ? false : row.cache, row.target);
  rows++;
}
assert.equal(cacheInternals.cacheableFamily("generator-inputs"), false);
console.log(`⚖️ nx-contract policy vectors ${rows}/${vectors.policies.length} PASS`);
