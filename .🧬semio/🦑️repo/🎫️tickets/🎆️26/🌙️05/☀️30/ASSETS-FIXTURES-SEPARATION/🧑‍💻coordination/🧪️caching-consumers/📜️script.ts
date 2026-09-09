import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!;
const output = process.env.SEMIO_FIXTURE_OUTPUT!;
mkdirSync(output, { recursive: true });
const cases = [
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts",
    "name": "testGeneratorInputReceipt"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🟦️.ts",
    "name": "testWasmOptimizer"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts",
    "name": "testTrunkLockfile"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🌿️environment/🧪️tests/🌿️workflow-context/🟦️.ts",
    "name": "testCiEnvironment"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🏃️resolve/🧪️tests/🧭️baseline-resolution/🟦️.ts",
    "name": "testCiResolution"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts",
    "name": "testCiBaseline"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧪️tests/🚦️baseline-command/🟦️.ts",
    "name": "testCiBaselineCommand"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🐙️github/🧪️tests/🐙️workflow-history/🟦️.ts",
    "name": "testGithubHistory"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🔏️tool-fingerprint/🟦️.ts",
    "name": "testWasmToolFingerprint"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧪️tests/🛠️binaryen-toolchain/🟦️.ts",
    "name": "testBinaryenToolchain"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧩️extension-attach/🟦️.ts",
    "name": "testExtensionAttach"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts",
    "name": "testDevcontainerContext"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔒️persistent-state/🟦️.ts",
    "name": "testContainerPersistentState"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts",
    "name": "testContainerRuntimeBootstrap"
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🧪️tests/🔒️playground-preferences/🟦️.ts",
    "name": "testPlaygroundPreferences"
  }
];
let failed = 0;
for (const row of cases) {
 console.log(`[DEBUG] START ${row.name}`);
 try {
  const api = await import(pathToFileURL(join(root, row.path)).href);
  const args = ["testContainerPersistentState", "testTrunkLockfile"].includes(row.name) ? [root, false] : [root, output];
  await api[row.name](...args);
  console.log(`[DEBUG] PASS ${row.name}`);
 } catch (error) { failed++; console.error(`[DEBUG] FAIL ${row.name}`, error); }
}
console.log(`[DEBUG] Caching consumers: ${cases.length - failed} passed, ${failed} failed`);
process.exitCode = failed ? 1 : 0;
