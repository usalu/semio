import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { configDefaults, defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/⚡️caching/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(root, "../../../../../..");

/** 🧪️ Suites that run under `bun test` (the owner's `test-*-contract` scripts), plus the playground proof modules and the composition config, which carry no Vitest suite. */
const NOT_VITEST_SUITES = ["🔎️discovery/🧪️tests/**", "🛂️descriptor-verification/🧪️tests/**", "🧪️tests/🧩️composition/🟦️.ts", "🧪️tests/🧩️composition/🎚️config/**", "🧪️tests/🧬️semantic-source-pairs/**", "🧪️tests/🔁️rebuild/**", "📣️publication/🧪️tests/**", "📣️publication/🏗️build/🧪️tests/**", "📥️runtime/🧪️tests/**", "🎮️playground/**/🧪️tests/**"];

export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "plugin-registry"),
  test: {
    root: testRoot,
    name: "@semio-tech/plugin-registry",
    environment: "node",
    include: ["**/🧪️tests/**/🟦️.ts"],
    env: { SEMIO_TEST_ARTIFACT_DIR: repoTestArtifactEnvironment(repoRoot, "plugin-registry").SEMIO_TEST_ARTIFACT_DIR! },
    exclude: [...configDefaults.exclude, "dist/**", "🧪️tests/🎚️config/🟦️.ts", "🧪️tests/📚️storybook-plugins/**", ...NOT_VITEST_SUITES],
    testTimeout: 120_000,
  },
});
