import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.cwd();
const owner = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript");
const { repositoryVitestPolicyV1 } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🟦️.ts")).href);
const { runVitestV1 } = await import(pathToFileURL(resolve(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts")).href);
const environment = { ...process.env, SEMIO_TEST_LEVEL: "long", SEMIO_TEST_BUDGET_MS: "60000" };
const policy = repositoryVitestPolicyV1(owner, environment, 1);
await runVitestV1(policy, [resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/⏳️attachment/🟦️.tsx"), "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts", { ...environment, SEMIO_VITEST_POLICY: JSON.stringify(policy) });
