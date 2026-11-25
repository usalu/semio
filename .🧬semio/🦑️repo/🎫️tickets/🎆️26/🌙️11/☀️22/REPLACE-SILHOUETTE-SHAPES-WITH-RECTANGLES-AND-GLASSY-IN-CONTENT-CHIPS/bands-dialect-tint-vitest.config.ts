import { defineConfig } from "vitest/config";
const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
export default defineConfig({
  resolve: { alias: { "bun:test": "vitest" } },
  define: { "import.meta.dir": JSON.stringify(`${ui}/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve`) },
  test: { include: [`${ui}/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts`, `${ui}/⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts`], root: "/Users/ueli/Documents/semio" },
});
