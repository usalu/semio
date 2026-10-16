// #region 🔌️Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { frameworkSourcePath } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/📖️source-text/🟦️.ts";
// #endregion 🔌️Adapters

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../..");

/** 🧪️ Vitest for `@semio-tech/animate-js`: the animate presentation app's canonical cases. The React deck renderer is the framework product `@semio-tech/presentation-react` and is tested by its owner. */
export default defineConfig({
  root,
  resolve: {
    alias: [{ find: "@semio-tech/presentation", replacement: frameworkSourcePath("🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts") }],
  },
  test: {
    root,
    name: "@semio-tech/animate-js",
    mode: "test",
    environment: "node",
    include: ["../../🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts"],
    passWithNoTests: false,
  },
});
