#!/usr/bin/env bun
/** 🧬️ ST2 runner: executes the generator-contract block of one workspace's OWN cache-contracts law (extracted verbatim at
 * runtime from `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`, `const generators = …` through its `vectors.generators` loop)
 * against that workspace's inventory, policy and nx-contract vectors — the full suite is red earlier on unrelated
 * pre-existing faults, so this proves exactly the section the 1b hunk changes. usage: bun nx1b-generator-contracts.ts <root> */
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2];
const caching = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching");
const source = readFileSync(join(caching, "🧪️tests/⚡️cache-contracts/🟦️.ts"), "utf8");
const start = source.indexOf("    const generators = JSON.parse(");
const loop = source.indexOf("for (const id of vectors.generators) {", start);
assert.ok(start > 0 && loop > start, "generator block not found");
let depth = 0, end = loop;
for (let index = source.indexOf("{", loop); index < source.length; index++) {
  if (source[index] === "{") depth++;
  if (source[index] === "}" && --depth === 0) { end = index + 1; break; }
}
const block = new Bun.Transpiler({ loader: "ts" }).transformSync(source.slice(start, end));
const { inventory } = await import(join(caching, "📇️inventory/🧮️composition/🟦️.ts"));
const contracts = inventory(root).projects;
const policy = JSON.parse(readFileSync(join(caching, "🔣️policy.json"), "utf8"));
const vectors = JSON.parse(readFileSync(join(caching, "🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
new Function("assert", "join", "readFileSync", "existsSync", "root", "vectors", "contracts", "policy", block)(assert, join, readFileSync, existsSync, root, vectors, contracts, policy);
console.log(`🧬️ generator contracts ${vectors.generators.join(", ")} PASS (${root})`);
