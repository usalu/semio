import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { defineConfig } from "vitest/config";
import rendererConfig from "@semio-tech/framework-renderer-react/testing/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

export default defineConfig({
  ...rendererConfig,
  root,
  cacheDir: `${rendererConfig.cacheDir}-layout`,
  test: {
    ...rendererConfig.test,
    root,
    name: "layout-renderer-contract",
    include: ["🧪️tests/🛍️canvas-catalogue/🟦️.ts"],
    includeSource: [],
    passWithNoTests: false,
    testNamePattern: undefined,
  },
});
