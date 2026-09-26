/** ⏱️ R8 probe: per-law wall time of the repo-lib `test quick` file (junit), without the file budget, to find the laws that
 * push the quick gate over its 300 s budget under fleet load. Usage: bun quick-law-timings.ts <junit-out> */
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { repoTestArtifactEnvironment } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const cwd = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript");
const out = process.argv[2]!;
const env = { ...repoTestArtifactEnvironment(repoRoot, "workspace-contract"), SEMIO_TEST_LEVEL: "quick" };
const started = Date.now();
const result = spawnSync(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", "--timeout", "60000", "--reporter=junit", `--reporter-outfile=${out}`], { cwd, env, stdio: ["ignore", "ignore", "ignore"] });
console.log(`exit=${result.status} secs=${Math.round((Date.now() - started) / 1000)}`);
