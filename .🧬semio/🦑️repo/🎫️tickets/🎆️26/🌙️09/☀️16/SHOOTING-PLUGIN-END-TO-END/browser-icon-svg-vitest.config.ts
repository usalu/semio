import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  root: resolve(import.meta.dirname, "../../../../../../..", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"),
  test: { include: ["🧪️tests/🖼️icon-svg-preview/🟦️.ts", "🧪️tests/📨️browser-frame-transport/🟦️.ts"], environment: "node" },
});
