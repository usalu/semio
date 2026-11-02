import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { rmSync } from "node:fs";

const root = process.cwd();
const generated = resolve(dirname(fileURLToPath(import.meta.url)), "🗑️generated/input");
const toolPath = resolve(root, "node_modules/vitest/vitest.mjs");
const policy = { version: 1, cwd: root, toolPath, runtime: process.execPath, coverageRuntime: process.execPath, cacheRoot: generated, coverageDirectory: resolve(generated, "coverage"), budgetMs: 60000, maxWorkers: 1 };
try {
  const child = Bun.spawn([process.execPath, toolPath, "run", "--configLoader", "native", "--config", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts", "✏️Input/🧪️tests/🧩️component", "--maxWorkers=1"], { env: { ...process.env, SEMIO_VITEST_POLICY: JSON.stringify(policy) }, stdout: "inherit", stderr: "inherit" });
  process.exitCode = await child.exited;
} finally {
  rmSync(generated, { recursive: true, force: true });
}
