/** @emoji 🧪️ Vitest for `@semio-tech/flow-js`. `includeSource` enables the in-source `import.meta.vitest`
 * block in `../../🔨️modules/🧮️compute/🟦️component.ts` (the barrel itself has no logic of its own to test). */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/flow-js",
    environment: "node",
    include: ["📦️index.ts", "../../🔨️modules/🧮️compute/🟦️component.ts"],
    includeSource: ["../../🔨️modules/🧮️compute/🟦️component.ts"],
    coverage: { include: ["📦️index.ts", "../../🔨️modules/🧮️compute/🟦️component.ts"] },
  },
});
