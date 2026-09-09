import assert from "node:assert/strict";
import fs from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { mock } from "bun:test";

const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const paths = ["🚨️transaction-sentinel-cases", "💉️ticket-important-exact-mutations"].map(name => library + "/🖼️assets/" + name + "/🔣️.json");
const observed = new Set<string>(), read = fs.readFileSync;
mock.module("node:fs", () => ({ ...fs, readFileSync: (...args: Parameters<typeof fs.readFileSync>) => {
  const path = String(args[0]);
  for (const catalog of paths) if (path === join(root, catalog)) observed.add(catalog);
  return Reflect.apply(read, fs, args);
} }));
try {
  const { inventoryTaxonomy, planTaxonomy } = await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
  const baseline = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" });
  assert.equal(baseline.status, 0);
  const baselineCommit = baseline.stdout.trim();
  console.log("[DEBUG] observed baseline=" + JSON.stringify(baselineCommit));
  assert.match(baselineCommit, /^[a-f0-9]{40}$/u);
  console.log("[DEBUG] inventorying actual normalization assets");
  const inventory = inventoryTaxonomy({ repoRoot: root, scope: library + "/🖼️assets" });
  observed.clear();
  console.log("[DEBUG] planning actual normalization assets");
  let planningFailure: unknown;
  try { planTaxonomy(inventory, { baselineCommit, excludedTreeDigests: [] }); }
  catch (error) { planningFailure = error; }
  assert.deepEqual([...observed].sort(), paths.slice().sort());
  if (planningFailure) assert.match(String(planningFailure), /Current compiler input manifest compiler inputs are not path-sorted/u);
  console.log("[DEBUG] actual inventory/plan consumed both current asset catalogs; entries=" + inventory.entries.length);
  console.log("[DEBUG] later planner outcome=" + (planningFailure ? String(planningFailure) : "completed"));
} finally { mock.restore(); }
