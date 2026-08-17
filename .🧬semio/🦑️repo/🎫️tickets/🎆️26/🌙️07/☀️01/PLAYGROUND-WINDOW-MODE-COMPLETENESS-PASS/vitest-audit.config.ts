/** @emoji 🧪️ Vitest config for cross-playground completeness audit. */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node",
    include: ["audit-playground-completeness.test.ts"],
    passWithNoTests: false,
  },
});
