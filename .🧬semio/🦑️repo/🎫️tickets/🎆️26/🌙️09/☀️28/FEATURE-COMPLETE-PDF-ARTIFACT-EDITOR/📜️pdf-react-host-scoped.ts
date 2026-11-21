/** 🎛️ Run the actual React runtime and opening tests with one worker and ticket-owned cache. */
import assert from "node:assert/strict";
import { mkdirSync, realpathSync } from "node:fs";
import { join, resolve } from "node:path";

const cwd = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript");
const generated = join(import.meta.dir, "🗑️generated", "pdf-react-host-test");
const cacheRoot = join(generated, "cache");
const coverageDirectory = join(generated, "coverage");
mkdirSync(cacheRoot, { recursive: true });

const vitest = realpathSync("node_modules/vitest/vitest.mjs");
const bun = Bun.which("bun");
const node = Bun.which("node");
assert.ok(bun && node);
const policy = { version: 1, cwd, toolPath: vitest, runtime: bun, coverageRuntime: node,
    cacheRoot, coverageDirectory, budgetMs: 120_000, maxWorkers: 1 };
const { readVitestPolicyV1 } = await import(resolve("🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"));
const env = {
    ...process.env, SEMIO_TEST_LEVEL: "long", SEMIO_TEST_BUDGET_MS: String(policy.budgetMs),
    SEMIO_VITEST_POLICY: JSON.stringify(policy),
};
readVitestPolicyV1(env, cwd);
const config = resolve(cwd, "../../🧪️tests/🎚️config/🟦️.ts");
const child = Bun.spawn(["repo", "tools", "exec", "--light", "vitest", "--", "run", "--config", config,
    "--configLoader", "native", "--maxWorkers", "1", "plugin-runtime-suite", "opening"], { cwd, env, stdout: "inherit", stderr: "inherit" });
process.exit(await child.exited);
