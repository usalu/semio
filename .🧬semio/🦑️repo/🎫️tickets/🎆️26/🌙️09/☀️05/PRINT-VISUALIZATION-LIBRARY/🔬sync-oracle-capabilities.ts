#!/usr/bin/env bun
/**
 * 🔮️ Syncs the print oracle registry's capability sets with what the print test cases actually claim.
 *
 * The platform's contract phase refuses a feature whose `@capability-<id>` is absent from the
 * `@oracle-<id>` entry it names, so every namespace agent adding a case would otherwise have to
 * serialize on TESTS-HARNESS for one string. This reads every `🧪️tests/<case>/🥒️.feature` under the
 * print owner, pairs its capability tag with its oracle tag, and adds any missing capability to that
 * oracle. It never removes one — a capability a retired case introduced stays surveyed.
 *
 * Run from anywhere: `bun "<ticket>/🔬sync-oracle-capabilities.ts"`.
 */
import { readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "C:/git/semio";
const owner = join(repoRoot, "🧰️framework", "🛍️products", "📓️print");
const registryPath = join(owner, "🔮️oracle", "🔣️.json");
const testsRoot = join(owner, "🧪️tests");

type Registry = { oracles: { id: string; capabilities: string[] }[] };

const registry = JSON.parse(readFileSync(registryPath, "utf8")) as Registry;
const wanted = new Map<string, Set<string>>();

for (const entry of existsSync(testsRoot) ? readdirSync(testsRoot, { withFileTypes: true }) : []) {
  if (!entry.isDirectory()) continue;
  const feature = join(testsRoot, entry.name, "🥒️.feature");
  if (!existsSync(feature)) continue;
  const text = readFileSync(feature, "utf8");
  const oracle = text.match(/^@oracle-([a-z0-9-]+)\s*$/mu)?.[1];
  const capability = text.match(/^@capability-([a-z0-9-]+)\s*$/mu)?.[1];
  if (oracle === undefined || capability === undefined) continue;
  wanted.set(oracle, (wanted.get(oracle) ?? new Set()).add(capability));
}

let added = 0;
for (const oracle of registry.oracles) {
  const missing = [...(wanted.get(oracle.id) ?? [])].filter((capability) => !oracle.capabilities.includes(capability));
  if (missing.length === 0) continue;
  oracle.capabilities = [...oracle.capabilities, ...missing].sort();
  added += missing.length;
  console.log(`[DEBUG] ${oracle.id} += ${missing.join(", ")}`);
}
for (const [oracle, capabilities] of wanted) if (!registry.oracles.some((entry) => entry.id === oracle)) console.error(`[DEBUG] unregistered oracle @oracle-${oracle} claimed for ${[...capabilities].join(", ")}`);

if (added > 0) writeFileSync(registryPath, `${JSON.stringify(registry, null, 2)}\n`, "utf8");
console.log(`[DEBUG] oracle capability sync: ${added} added across ${wanted.size} oracle(s)`);
