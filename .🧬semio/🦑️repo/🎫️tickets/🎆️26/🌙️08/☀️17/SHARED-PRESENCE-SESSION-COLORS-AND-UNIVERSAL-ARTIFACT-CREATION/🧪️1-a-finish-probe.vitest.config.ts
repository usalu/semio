import { defineConfig } from "vitest/config";

const root = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest";

export default defineConfig({
  root,
  test: {
    name: "probe-manifest-component",
    mode: "test",
    environment: "node",
    include: ["🟦️component.ts"],
    includeSource: ["🟦️component.ts"],
    passWithNoTests: false,
  },
});
