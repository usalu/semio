/** 🧪️ Throwaway vitest config of slice acd: runs the wiring tests against the pre-decomposition client snapshot. */
import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript";
const here = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/acd-tools/orig";

export default defineConfig({
  root,
  resolve: { alias: [{ find: /^@semio-tech\/framework-os$/u, replacement: resolve(root, "🟦️.ts") }] },
  test: { root, name: "acd-orig", environment: "node", include: [resolve(here, "wiring-orig.test.ts")], passWithNoTests: false },
});
