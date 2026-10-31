import { repositoryVitestPolicyV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🟦️.ts";
import { devToolingEnv } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { runVitestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";

const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript";
process.env.SEMIO_TEST_LEVEL = "long";
const policy = repositoryVitestPolicyV1(root, process.env, 2);
await runVitestV1(policy, process.argv.slice(2), "../../🧪️tests/🎚️config/🟦️.ts", devToolingEnv({ SEMIO_VITEST_POLICY: JSON.stringify(policy) }));
