import { defineConfig } from "vitest/config";
import { resolve } from "node:path";
export default defineConfig({
  cacheDir: resolve(import.meta.dirname, "🗑️generated/field-vitest"),
  esbuild: { jsx: "automatic" },
  test: { environment: "jsdom", include: ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️suite/🟦️.tsx","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx"] },
});
