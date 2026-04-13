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

  const waitFor = async (label, predicate, timeoutMs = 30_000) => {
    const deadlineMs = Date.now() + timeoutMs;
    while (Date.now() < deadlineMs) {
      const result = await predicate();
      if (result) return result;
      await new Promise((r) => setTimeout(r, 250));
    }
    throw new Error(`Timed out waiting for ${label}`);
  };

  const clickById = async (id) => {
    await win.webContents.executeJavaScript(`
      (() => {
        const el = document.getElementById(${JSON.stringify(id)});
        if (!el) return false;
        el.click();
        return true;
      })()
    `);
  };

  const readPersistedKits = async () =>
    await win.webContents.executeJavaScript(`
      (async () => {
        const userId = await window.os.getUserId();
        const raw = window.localStorage.getItem('semio.sketchpad.kits.' + userId);
        return raw ? JSON.parse(raw) : [];
      })()
    `);

  await waitFor(
    "home open actions",
    async () =>
      await win.webContents.executeJavaScript(`
        (() => Boolean(
          document.getElementById("semio.sketchpad.app.home.toolbar.openFolder") &&
          document.getElementById("semio.sketchpad.app.home.toolbar.openFile")
        ))()
      `),
  );

  await clickById("semio.sketchpad.app.home.toolbar.openFolder");
  const persistedAfterFolder = await waitFor("folder kit persistence", async () => {
    const kits = await readPersistedKits();
    return kits.find((entry) => entry?.kit?.name === "Metabolism" && (entry?.kit?.files?.length ?? 0) > 200) ?? null;
  });
  assert.ok(persistedAfterFolder.local, "folder-opened kit should be marked local");
  assert.strictEqual(persistedAfterFolder.remote, false, "folder-opened kit should not be marked remote");
  assert.ok((persistedAfterFolder.kit.types?.length ?? 0) > 40, "folder-opened kit should include types");
  assert.strictEqual(persistedAfterFolder.source?.kind, "folder", "folder-opened kit should persist folder source metadata");
  assert.ok(
    typeof persistedAfterFolder.source?.path === "string" && persistedAfterFolder.source.path.replaceAll("\\", "/").endsWith("/semio/assets/semio/metabolism"),
    "folder-opened kit should persist the opened folder path",
  );

  await clickById("semio.sketchpad.app.home.toolbar.openFile");
  const persistedAfterFile = await waitFor("file kit persistence", async () => {
    const kits = await readPersistedKits();
    return kits.find(
      (entry) =>
        entry?.kit?.name === "Metabolism" &&
        (entry?.kit?.files?.length ?? 0) > 200 &&
        typeof entry?.kit?.files?.[0]?.blob === "string",
    ) ?? null;
  });
  assert.ok(persistedAfterFile.local, "file-opened kit should be marked local");
  assert.strictEqual(persistedAfterFile.remote, false, "file-opened kit should not be marked remote");
  assert.strictEqual(persistedAfterFile.source?.kind, "file", "file-opened kit should persist file source metadata");
  assert.ok(
    typeof persistedAfterFile.source?.path === "string" && persistedAfterFile.source.path.replaceAll("\\", "/").endsWith("/semio/assets/semio/metabolism.kit.semio.json"),
    "file-opened kit should persist the opened file path",
  );
  assert.ok(
    persistedAfterFile.kit.files.some((file) => typeof file?.blob === "string" && file.blob.startsWith("data:model/")),
    "file-opened kit should preserve embedded model blobs",
  );

  console.log("[semio desktop integration] suite passed");
}

// #endregion DesktopIntegrationSuite
