// #region DesktopTestConfig
// High-level config helper mirroring @vscode/test-cli `defineConfig` for desktop integration tests.
// Specs: `.compose-test.mjs` default export must include `files` (absolute or relative suite entry).

/**
 * @param {{ files: string; label?: string; workspaceFolder?: string }} config
 */
export function defineDesktopTestConfig(config) {
  if (!config?.files || typeof config.files !== "string") {
    throw new Error("defineDesktopTestConfig: `files` (path to suite module exporting run(ctx)) is required");
  }
  return config;
}

// #endregion DesktopTestConfig
