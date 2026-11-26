import { defineConfig } from "vitest/config";
const engine = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
export default defineConfig({
  test: {
    environment: "jsdom",
    maxWorkers: 3,
    testTimeout: 30000,
    include: [
      engine + "/🧪️tests/{🖋️ink-canvas-editing,🖋️ink-canvas-domain-interaction,🖋️ink-canvas-clipboard,📂️local-files,👥️scoped-presence}/🟦️.tsx",
      engine + "/🧱️elements/✏️TextEditor/🧪️tests/{📝️explicit-draft,📄️value-source}/🟦️.tsx",
      engine + "/🧱️elements/🌐️World3dHost/🧪️tests/📏️modelling/🟦️.tsx",
    ],
  },
});
