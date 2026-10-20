/** 🧪 Runs the staged Nx-contract policy vectors through the staged target-policy overlay and Nx's own `isCacheableTask` (independent third-party oracle). */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { load } from "./overlay.ts";
const WS = "/Users/ueli/Documents/semio", L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vectors = JSON.parse(readFileSync(join(import.meta.dir, "staged", L, "⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json"), "utf8"));
const { isCacheableTask } = createRequire(join(WS, "package.json"))("nx/src/tasks-runner/utils");
const { module: m, POLICY } = await load("staged");
let n = 0;
for (const row of vectors.policies) for (const authored of [true, false]) {
  const target = m.targetPolicy(row.target, { cache: authored, options: { command: row.command ?? `bun ./📜️script.ts ${row.target}` } }, POLICY);
  const expected = authored ? row.cache : row.disabledCache;
  if (target.cache !== expected || (target.continuous ?? false) !== row.continuous) throw Error(`${row.target} authored ${authored}: got ${target.cache}`);
  if (isCacheableTask({ cache: target.cache, continuous: target.continuous === true, target: { project: "probe", target: row.target }, overrides: {} }) !== expected) throw Error(`nx disagrees on ${row.target}`);
  n++;
}
console.log(`vectors ok: ${n} policy checks agree with Nx isCacheableTask`);
