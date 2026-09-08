//#region 🔌️Adapters
import { defineConfig } from "vitest/config";
//#endregion 🔌️Adapters

/** 🧪️ Tests for the demonstrator task router. */
export default defineConfig({
  test: {
    name: "@semio-tech/mit-bestand-demonstrator",
    environment: "node",
    include: [],
    includeSource: ["./📜️script.ts", "./🪧️brand.ts"],
    passWithNoTests: false,
  },
});
