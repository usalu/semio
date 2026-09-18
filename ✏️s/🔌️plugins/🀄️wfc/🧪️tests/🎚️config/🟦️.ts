import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

/** 🀄️ Runs the committed WFC example tests and the cross-language schema fixture oracles of the five
 * artifacts (bitmap, grid2d, wfc2d, grid3d, wfc3d). */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/wfc-js",
    environment: "node",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts"],
    passWithNoTests: false,
  },
});
