import { resolve } from "node:path";
import { repositoryVitestPolicyV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/📋️context/🔣️policy/🟦️.ts";
import { vitestArgumentsV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
// vt-cmd.ts <root> <config> <scenario>: scenario = default (no bound) | w<N> (explicit bound) | host (policy, no grant) | grant<MB> (policy under one job of <MB> memory)
const [root, config, scenario] = process.argv.slice(2);
const environment: Record<string, string> = { SEMIO_TEST_LEVEL: "quick" };
if (scenario!.startsWith("grant")) environment.SEMIO_HOST_CAPACITY_PLAN = JSON.stringify({ version: 1, id: "k".repeat(22), jobs: 1, cpu: 1, memoryMegabytes: Number(scenario!.slice(5)), fixedMegabytes: 1536, floorMegabytes: 2048 });
const policy: { maxWorkers?: number } & ReturnType<typeof repositoryVitestPolicyV1> = repositoryVitestPolicyV1(resolve(root!), environment);
if (scenario === "default") delete policy.maxWorkers;
else if (/^w\d+$/u.test(scenario!)) policy.maxWorkers = Number(scenario!.slice(1));
console.log(JSON.stringify({ workers: policy.maxWorkers ?? null, argv: [policy.runtime, ...vitestArgumentsV1(policy, ["--configLoader", "native"], config!, false)], env: { SEMIO_VITEST_POLICY: JSON.stringify(policy) }, cwd: policy.cwd }));
