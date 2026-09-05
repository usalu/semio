//#region 🔌️Adapters
import { defineConfig } from "vitest/config";
//#endregion 🔌️Adapters

/** @emoji 🧪️ In-source tests for the demonstrator task router. */
export default defineConfig({
  test: {
    name: "@semio-tech/mit-bestand-demonstrator",
    environment: "node",
    include: [],
    includeSource: ["./📜️script.ts", "./🪧️brand.ts"],
    passWithNoTests: false,
  },
});
