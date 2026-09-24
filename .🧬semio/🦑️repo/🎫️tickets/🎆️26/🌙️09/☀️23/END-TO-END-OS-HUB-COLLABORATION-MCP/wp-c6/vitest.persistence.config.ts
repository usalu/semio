import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const os = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os';
const pkg = resolve(os, "📦️packages/🟦️typescript");

export default defineConfig({
  root: pkg,
  resolve: { alias: { "@semio-tech/framework-os": resolve(os, "🟦️.ts") } },
  test: {
    root: pkg,
    name: "@semio-tech/framework-os-persistence-data-class",
    environment: "node",
    include: ['/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/persistence-data-class/🟦️.ts'],
    includeSource: [],
    passWithNoTests: false,
    testTimeout: 60000,
  },
});
