/** 🧪 Vitest config for `@semio-tech/framework-os-dev` — no in-source unit tests live in this
 * bundle root today (its `js/index.ts` boot entry is guarded by `!import.meta.vitest` rather than
 * carrying a test block), so this exists purely so `bun ./script.ts test` (→ `runVitest`) has a
 * config to resolve; `--passWithNoTests` keeps it green until real coverage lands here. */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/framework-os-dev",
    environment: "jsdom",
    coverage: { include: ["js/index.ts"] },
  },
});
