import { defineConfig } from "vitest/config";
export default defineConfig({
  resolve: { alias: { "bun:test": "vitest" } },
  test: { include: ["/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts"], root: "/Users/ueli/Documents/semio" },
});
