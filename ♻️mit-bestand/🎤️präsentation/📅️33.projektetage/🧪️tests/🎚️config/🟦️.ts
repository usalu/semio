//#region 🔌️Adapters
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { semioAssetsVitePlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
//#endregion 🔌️Adapters

const dir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(dir, "../../../../..");

/** @emoji 🧪️ Vitest for `@semio-tech/mit-bestand-praesentation-projektetage`. */
export default defineConfig({
  root: testRoot,
  plugins: [...semioAssetsVitePlugin(repoRoot), tailwindcss(), react()],
  resolve: {
    alias: [
:♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🧪️vitest.config.ts
      {
        find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
        replacement: resolve(dir, "🔖️spec.ts"),
      },
    ],
  },
  test: {
    root: testRoot,
    name: "@semio-tech/mit-bestand-praesentation-projektetage",
    mode: "test",
    environment: "node",
    include: [],
    coverage: { include: ["🟦️.ts"] },
    includeSource: ["🟦️.ts"],
    passWithNoTests: false,
  },
});
