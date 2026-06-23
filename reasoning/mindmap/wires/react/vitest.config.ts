// #region 🧲Header
/** @emoji 🧪 Vitest for `@semio-tech/reasoning-mindmap-wires-react`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const reactDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: reactDir,
  resolve: {
    alias: {},
  },
  test: {
    environment: "node",
    include: ["index.ts"],
    passWithNoTests: false,
  },
});
