import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
//#region 🔌️Adapters
import { defineConfig } from "vitest/config";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
//#endregion 🔌️Adapters

/** 🧪️ Tests for the demonstrator task router. */
export default defineConfig({
  root: testRoot,
  test: {
    root: testRoot,
    name: "@semio-tech/mit-bestand-demonstrator",
    environment: "node",
    include: [],
    includeSource: ["./📜️script.ts", "./🪧️brand.ts"],
    passWithNoTests: false,
  },
});
