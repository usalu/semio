// #region ComposeDesktopTestCliConfig
// Mirrors `.vscode-test.mjs` from https://code.visualstudio.com/api/working-with-extensions/testing-extension
// Specs: `files` is the ESM suite path (must export `run(ctx)`).

import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/**
 * @param {{ files: string; label?: string; workspaceFolder?: string }} config
 */
function defineDesktopTestConfig(config) {
  if (!config?.files || typeof config.files !== "string") {
    throw new Error("defineDesktopTestConfig: `files` (path to suite module exporting run(ctx)) is required");
  }
  return config;
}

export default defineDesktopTestConfig({
  label: "integration",
  files: path.join(__dirname, "test/suite/index.mjs"),
});

// #endregion ComposeDesktopTestCliConfig
