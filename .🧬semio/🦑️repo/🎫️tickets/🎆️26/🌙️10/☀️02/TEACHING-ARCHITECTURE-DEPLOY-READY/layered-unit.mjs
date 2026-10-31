import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const { repositoryVitestPolicyV1 } = await import(pathToFileURL(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🟦️.ts")).href);
const policy = repositoryVitestPolicyV1(resolve(root, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"));
process.env.SEMIO_VITEST_POLICY = JSON.stringify(policy);
process.argv = [process.argv[0], policy.toolPath, ...process.argv.slice(2)];
await import(pathToFileURL(policy.toolPath).href);
