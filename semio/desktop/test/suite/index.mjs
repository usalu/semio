// #region DesktopIntegrationSuite
// Default integration suite: runs in Electron main after the window loads (see main.ts SEMIO_EXTENSION_TESTS_PATH).
// Specs: Export `run(ctx)` like VS Code `src/test/suite/index.ts` + extensionTestsPath.

import assert from "node:assert";

/**
 * @param {{
 *   app: import("electron").App;
 *   BrowserWindow: typeof import("electron").BrowserWindow;
 *   path: typeof import("node:path");
 *   whenFirstWindowLoaded: () => Promise<void>;
 * }} ctx
 */
export async function run(ctx) {
  assert.ok(ctx.app, "ctx.app");
  assert.ok(ctx.BrowserWindow, "ctx.BrowserWindow");
  assert.strictEqual(typeof ctx.whenFirstWindowLoaded, "function");
  await ctx.whenFirstWindowLoaded();
  const win = ctx.BrowserWindow.getAllWindows()[0];
  assert.ok(win, "BrowserWindow");

  const consoleMessages = [];
  win.webContents.on("console-message", (_event, level, message) => {
    consoleMessages.push({ level, message });
  });

  const deadline = Date.now() + 120_000;
  let hasRoot = false;
  while (Date.now() < deadline) {
    hasRoot = await win.webContents.executeJavaScript(`
      (() => {
        const el = document.getElementById("root");
        return Boolean(el && el.childElementCount > 0);
      })()
    `);
    if (hasRoot) break;
    await new Promise((r) => setTimeout(r, 400));
  }
  assert.ok(
    hasRoot,
    "renderer #root still empty after 120s (white screen). Check devtools for ESM/CJS shim errors (e.g. use-sync-external-store).",
  );
  const unexpectedRendererMessages = consoleMessages.filter(({ level, message }) => {
    if (level < 2) return false;
    return (
      message.includes("does not provide an export named 'default'") ||
      message.includes("ReactDOMClient.createRoot() on a container that has already been passed to createRoot() before") ||
      message.includes("Failed to execute 'removeChild' on 'Node'")
    );
  });
  assert.deepStrictEqual(
    unexpectedRendererMessages,
    [],
    `renderer emitted fatal console messages: ${unexpectedRendererMessages.map(({ message }) => message).join(" | ")}`,
  );
  console.log("[semio desktop integration] suite passed");
}

// #endregion DesktopIntegrationSuite
