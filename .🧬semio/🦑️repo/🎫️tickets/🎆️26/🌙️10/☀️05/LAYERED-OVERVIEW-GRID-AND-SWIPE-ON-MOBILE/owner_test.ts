/** 🧪️ Runs an owner's own `📜️script.ts test` with the repository's Vitest policy while the Nx bootstrap refuses this shell's bun.
 * Run: bun owner_test.ts <owner package dir> [test filters...] */
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import { repositoryVitestPolicyV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";

const [owner, ...filters] = process.argv.slice(2);
if (owner === undefined) throw new Error("owner package directory required");
const cwd = resolve(owner);
const env = { ...process.env, SEMIO_TEST_BUDGET_MS: process.env.SEMIO_TEST_BUDGET_MS ?? "600000", SEMIO_VITEST_POLICY: JSON.stringify(repositoryVitestPolicyV1(cwd, { ...process.env, SEMIO_TEST_BUDGET_MS: process.env.SEMIO_TEST_BUDGET_MS ?? "600000" })) };
const result = spawnSync("bun", ["./📜️script.ts", "test", ...filters], { cwd, env, stdio: "inherit" });
process.exit(result.status ?? 1);
