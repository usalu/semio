import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  root: resolve(import.meta.dirname, "../../.."),
  test: {
    name: "@semio-tech/fem-js",
    include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"],
    environment: "node",
    passWithNoTests: false,
  },
});
