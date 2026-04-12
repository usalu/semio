// #region SemioDesktopTestCliConfig
// Mirrors `.vscode-test.mjs` from https://code.visualstudio.com/api/working-with-extensions/testing-extension
// Specs: `files` is the ESM suite path (must export `run(ctx)`).

import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineDesktopTestConfig } from "./test/defineConfig.mjs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineDesktopTestConfig({
  label: "integration",
  files: path.join(__dirname, "test/suite/index.mjs"),
});

// #endregion SemioDesktopTestCliConfig
