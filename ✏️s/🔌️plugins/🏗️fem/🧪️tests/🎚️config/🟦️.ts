import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");

export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/fem-js",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts"],
    environment: "node",
    passWithNoTests: false,
  },
});
