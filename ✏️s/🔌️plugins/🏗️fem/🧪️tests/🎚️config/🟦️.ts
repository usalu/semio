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
    include: ["🧪️tests/🧱️material-receiving/🟦️.ts", "🧪️tests/🌀️geometric-cells/🟦️.ts", "🧪️tests/⚖️combination-stress/🟦️.ts", "🧪️tests/📖️mutation-grammar/🟦️.ts", "🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🧪️tests/🧪️shared-authority/🟦️.ts", "🗿️artifacts/*/🧪️tests/🧱️field-intent/🟦️.ts", "../../🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts", "../../🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🕹️interaction/🟦️.ts"],
    environment: "node",
    passWithNoTests: false,
  },
});
