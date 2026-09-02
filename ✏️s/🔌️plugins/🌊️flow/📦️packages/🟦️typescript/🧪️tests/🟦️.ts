import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export default defineConfig({
  root,
  test: {
    name: "@semio-tech/flow-js",
    environment: "node",
    include: ["🟦️.ts"],
    coverage: { include: ["🟦️.ts"] },
    passWithNoTests: true,
  },
});
