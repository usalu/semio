import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { startVitest } from "vitest/node";
import staffageConfig from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
import { isolatedVitePackageResolver } from "./isolated-browser-runner.ts";

const tests = process.env.STAFFAGE_VITEST_CASES?.split(",").filter(Boolean) ?? [];
const scratch = process.env.STAFFAGE_SCRATCH ?? process.cwd();
const workspace = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const cacheDir = resolve(scratch, "staffage-vitest-cache");
const output = process.env.STAFFAGE_VITEST_REPORT;
console.error("[DEBUG] Staffage Vitest configuration loaded");
const task = await startVitest("test", tests, {
  ...staffageConfig.test,
  config: false,
  maxWorkers: 2,
  watch: false,
  setupFiles: [],
  ...(output === undefined ? {} : { reporters: ["json"], outputFile: output }),
}, {
  ...staffageConfig,
  cacheDir,
  envFile: false,
  optimizeDeps: { noDiscovery: true, include: [] },
  server: { ...staffageConfig.server, watch: null, fs: { allow: [workspace, scratch] } },
  plugins: [isolatedVitePackageResolver(scratch), ...(staffageConfig.plugins ?? [])],
});
await task?.close();
