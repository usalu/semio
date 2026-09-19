#!/usr/bin/env bun
/** 🔬️ Runs the exported entry points T2c edited whose own suites are gated behind a live Nx graph or cargo,
 * so each edit is proved by execution rather than by typecheck alone. */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const workspace = "/Users/ueli/Documents/semio";
const generated = mkdtempSync(join(tmpdir(), "t2c-probe-"));

const cases: readonly [string, () => unknown | Promise<unknown>][] = [
  ["🃏️glob verifyFixtureGlobOracle", async () => (await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🟦️.ts"))).verifyFixtureGlobOracle()],
  ["🚀️runtime-bootstrap testContainerRuntimeBootstrap", async () => (await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts"))).testContainerRuntimeBootstrap(workspace)],
  ["🧩️host-build testExtensionHostBuild", async () => (await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts"))).testExtensionHostBuild(workspace, generated)],
];

let failures = 0;
for (const [label, run] of cases) {
  try {
    await run();
    console.log(`PASS ${label}`);
  } catch (error) {
    failures += 1;
    console.log(`FAIL ${label}: ${error instanceof Error ? error.message : String(error)}`);
  }
}
process.exit(failures === 0 ? 0 : 1);
