import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const root = resolve(import.meta.dirname, "../../../../../../..", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost");

export default defineConfig({
  root,
  esbuild: { jsx: "automatic" },
  test: {
    include: ["🧪️tests/📤️svg-export/🟦️.ts", "🧪️tests/🏷️status/🟦️.tsx", "🧪️tests/🚚️request/🟦️.tsx"],
    environment: "jsdom",
  },
});
