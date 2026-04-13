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

  const readMetabolismFolderKitFromStore = async () =>
    await win.webContents.executeJavaScript(`
      (() => {
        const store = window.__SEMIO_STORE__;
        if (!store) return null;
        const list = store.kitShallows();
        const m = list.find((k) => k.name === "Metabolism" && (k.types?.length ?? 0) > 40 && (k.files?.length ?? 0) > 200);
        return m ? { name: m.name, typeCount: m.types.length, fileCount: m.files.length } : null;
      })()
    `);

  const readMetabolismFileKitFromStore = async () =>
    await win.webContents.executeJavaScript(`
      (() => {
        const store = window.__SEMIO_STORE__;
        if (!store) return null;
        for (const s of store.kitShallows()) {
          if (s.name !== "Metabolism") continue;
          const kit = store.kit(s.guid).snapshot().kit;
          const files = kit.files ?? [];
          if (
            files.length > 200 &&
            typeof files[0]?.blob === "string" &&
            files[0].blob.startsWith("data:model/")
          ) {
            return { guid: s.guid, fileCount: files.length, hasModelBlob: true };
          }
        }
        return null;
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
    120_000,
  );

  await clickById("semio.sketchpad.app.home.toolbar.openFolder");
  const loadedFolderKit = await waitFor("folder kit loaded in session store", readMetabolismFolderKitFromStore);
  assert.ok(loadedFolderKit.typeCount > 40, "folder-opened kit should expose types in session");
  assert.ok(loadedFolderKit.fileCount > 200, "folder-opened kit should expose files in session");
  const persistedAfterFolder = await readPersistedKits();
  assert.deepStrictEqual(
    persistedAfterFolder,
    [],
    "desktop must not persist folder kit snapshots to localStorage",
  );

  await clickById("semio.sketchpad.app.home.toolbar.openFile");
  const loadedFileKit = await waitFor("file kit loaded in session store", readMetabolismFileKitFromStore);
  assert.ok(loadedFileKit.hasModelBlob, "file-opened kit should keep embedded model blobs in session");
  assert.ok(loadedFileKit.fileCount > 200, "file-opened kit should expose files in session");
  const persistedAfterFile = await readPersistedKits();
  assert.deepStrictEqual(
    persistedAfterFile,
    [],
    "desktop must not persist file kit snapshots to localStorage",
  );

  console.log("[semio desktop integration] suite passed");
}

// #endregion DesktopIntegrationSuite
