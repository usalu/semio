// #region 🧲Header
// Vitest project for in-source tests in `index.tsx` (e.g. 🧪NegativeGrep).
// #endregion 🧲Header

import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "sketchpad",
    includeSource: ["./index.tsx"],
    environment: "node",
  },
});
