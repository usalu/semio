// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron main process managing windows and lifecycle.

// #endregion 🧲Header

// #region 🐙Main Process
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

import { app, BrowserWindow, dialog, ipcMain, type WebContents } from "electron";
import started from "electron-squirrel-startup";
import path from "node:path";
import fs from "node:fs";
import os from "os";
import { pathToFileURL } from "node:url";

if (started) {
  app.quit();
}

// Disable GPU acceleration in containerized environments where GPU/WebGL is unavailable.
// Without this, the GPU process crashes with "WebGL1 blocklisted" and kills the app.
if (process.env.REMOTE_CONTAINERS || process.env.CODESPACES || process.env.CONTAINER) {
  app.disableHardwareAcceleration();
  app.commandLine.appendSwitch("disable-gpu");
  app.commandLine.appendSwitch("disable-gpu-compositing");
  app.commandLine.appendSwitch("disable-software-rasterizer");
  app.commandLine.appendSwitch("in-process-gpu");
}

/**
 * Creates the main Electron browser window with preload and vite integration.
 * MUST load the vite dev server URL in development and the built file in production.
 **/
const createWindow = () => {
  const mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    autoHideMenuBar: true,
    // TODO: Make webkit app region work for electron
    frame: false,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false,
    },
  });

  if (MAIN_WINDOW_VITE_DEV_SERVER_URL) {
    mainWindow.loadURL(MAIN_WINDOW_VITE_DEV_SERVER_URL);
    mainWindow.webContents.openDevTools({ mode: "detach" });
  } else {
    mainWindow.loadFile(path.join(__dirname, `../renderer/${MAIN_WINDOW_VITE_NAME}/index.html`));
  }
};

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

app.whenReady().then(async () => {
  app.setAppUserRepresentationId("com.electron");

  const resolveKitEntryPath = (folderPath: string, entryPath: string) => {
    const rootPath = path.resolve(folderPath);
    const targetPath = path.resolve(rootPath, entryPath);
    const relativePath = path.relative(rootPath, targetPath);
    if (relativePath === "" || (!relativePath.startsWith("..") && !path.isAbsolute(relativePath))) {
      return targetPath;
    }
    throw new Error(`Kit path escapes selected folder: ${entryPath}`);
  };

  ipcMain.handle("minimize-window", (event) => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) window.minimize();
  });
  ipcMain.handle("maximize-window", (event) => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) {
      if (window.isMaximized()) {
        window.unmaximize();
      } else {
        window.maximize();
      }
    }
  });
  ipcMain.handle("close-window", (event) => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) window.close();
  });

  ipcMain.handle("get-user-id", (event) => {
    return os.userInfo().username;
  });

  // #region 🗂️FolderIPC
  // IPC handlers for folder-based kit storage.
  ipcMain.handle("select-folder", async () => {
    const result = await dialog.showOpenDialog({
      properties: ["openDirectory"],
      title: "Select Kit Folder",
    });
    if (result.canceled || result.filePaths.length === 0) return null;
    return result.filePaths[0];
  });

  ipcMain.handle("read-kit", async (_event, folderPath: string) => {
    const kitPath = path.join(folderPath, ".semio", "kit.db");
    try {
      const buffer = fs.readFileSync(kitPath);
      return buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength);
    } catch {
      return null;
    }
  });

  ipcMain.handle("write-kit", async (_event, folderPath: string, data: ArrayBuffer) => {
    const semioDir = path.join(folderPath, ".semio");
    if (!fs.existsSync(semioDir)) {
      fs.mkdirSync(semioDir, { recursive: true });
    }
    fs.writeFileSync(path.join(semioDir, "kit.db"), Buffer.from(data));
  });

  ipcMain.handle("select-file", async () => {
    const result = await dialog.showOpenDialog({
      properties: ["openFile"],
      title: "Select Kit JSON File",
      filters: [{ name: "Semio Kit JSON", extensions: ["json"] }],
    });
    if (result.canceled || result.filePaths.length === 0) return null;
    return result.filePaths[0];
  });

  ipcMain.handle("read-kit-json-file", async (_event, filePath: string) => {
    try {
      return fs.readFileSync(filePath, "utf-8");
    } catch {
      return null;
    }
  });

  ipcMain.handle("write-kit-json-file", async (_event, filePath: string, json: string) => {
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(filePath, json, "utf-8");
  });

  ipcMain.handle("read-file", async (_event, folderPath: string, filePath: string) => {
    const fullPath = resolveKitEntryPath(folderPath, filePath);
    try {
      const buffer = fs.readFileSync(fullPath);
      return buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength);
    } catch {
      return null;
    }
  });

  ipcMain.handle("write-file", async (_event, folderPath: string, filePath: string, data: ArrayBuffer) => {
    const fullPath = resolveKitEntryPath(folderPath, filePath);
    const dir = path.dirname(fullPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(fullPath, Buffer.from(data));
  });

  ipcMain.handle("delete-file", async (_event, folderPath: string, filePath: string) => {
    const fullPath = resolveKitEntryPath(folderPath, filePath);
    try {
      fs.unlinkSync(fullPath);
    } catch {
      /* ignore */
    }
  });

  ipcMain.handle("create-directory", async (_event, folderPath: string, directoryPath: string) => {
    const fullPath = resolveKitEntryPath(folderPath, directoryPath);
    fs.mkdirSync(fullPath, { recursive: true });
  });

  ipcMain.handle("move-entry", async (_event, folderPath: string, fromPath: string, toPath: string) => {
    const sourcePath = resolveKitEntryPath(folderPath, fromPath);
    const targetPath = resolveKitEntryPath(folderPath, toPath);
    fs.mkdirSync(path.dirname(targetPath), { recursive: true });
    fs.renameSync(sourcePath, targetPath);
  });

  ipcMain.handle("list-files", async (_event, folderPath: string) => {
    const results: string[] = [];
    function walk(dir: string, base: string) {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const entry of entries) {
        if (entry.name === ".semio" || entry.name === "node_modules") continue;
        const rel = base ? `${base}/${entry.name}` : entry.name;
        if (entry.isDirectory()) {
          walk(path.join(dir, entry.name), rel);
        } else {
          results.push(rel);
        }
      }
    }
    try {
      walk(folderPath, "");
    } catch {
      /* ignore */
    }
    return results;
  });

  ipcMain.handle("get-recent-folders", () => {
    const configPath = path.join(app.getPath("userData"), "recent-folders.json");
    try {
      return JSON.parse(fs.readFileSync(configPath, "utf-8"));
    } catch {
      return [];
    }
  });

  ipcMain.handle("add-recent-folder", (_event, folderPath: string) => {
    const configPath = path.join(app.getPath("userData"), "recent-folders.json");
    let recent: string[] = [];
    try {
      recent = JSON.parse(fs.readFileSync(configPath, "utf-8"));
    } catch {
      /* ignore */
    }
    recent = [folderPath, ...recent.filter((f: string) => f !== folderPath)].slice(0, 10);
    fs.writeFileSync(configPath, JSON.stringify(recent), "utf-8");
  });

  // Specs: One recursive watcher per kit folder; debounced IPC notifies renderer to reload `.semio/kit.db` and asset files.
  type KitFolderWatchEntry = { watcher: fs.FSWatcher; subscribers: Set<WebContents>; debounce?: NodeJS.Timeout };
  const kitFolderWatches = new Map<string, KitFolderWatchEntry>();

  const notifyKitFolderSubscribers = (folderPath: string) => {
    const entry = kitFolderWatches.get(folderPath);
    if (!entry) return;
    if (entry.debounce) clearTimeout(entry.debounce);
    entry.debounce = setTimeout(() => {
      entry!.debounce = undefined;
      for (const wc of entry!.subscribers) {
        if (!wc.isDestroyed()) wc.send("kit-folder-changed", folderPath);
      }
    }, 120);
  };

  ipcMain.on("kit-folder-watch-subscribe", (event, folderPath: string) => {
    const wc = event.sender;
    let entry = kitFolderWatches.get(folderPath);
    if (!entry) {
      try {
        const watcher = fs.watch(folderPath, { recursive: true }, () => notifyKitFolderSubscribers(folderPath));
        entry = { watcher, subscribers: new Set() };
        kitFolderWatches.set(folderPath, entry);
      } catch {
        return;
      }
    }
    entry.subscribers.add(wc);
  });

  ipcMain.on("kit-folder-watch-unsubscribe", (event, folderPath: string) => {
    const wc = event.sender;
    const entry = kitFolderWatches.get(folderPath);
    if (!entry) return;
    entry.subscribers.delete(wc);
    if (entry.subscribers.size === 0) {
      if (entry.debounce) clearTimeout(entry.debounce);
      try {
        entry.watcher.close();
      } catch {
        /* ignore */
      }
      kitFolderWatches.delete(folderPath);
    }
  });
  // #endregion 🗂️FolderIPC

  createWindow();

  // #region DesktopIntegrationTests
  // VS Code-style integration tests: SEMIO_EXTENSION_TESTS_PATH points at an ESM file that exports run(ctx).
  // Specs: Mirrors extensionTestsPath; launcher is node semio/desktop/test/runDesktopTests.mjs (see .semio-test.mjs).
  const extensionTestsPath = process.env.SEMIO_EXTENSION_TESTS_PATH?.trim();
  if (extensionTestsPath) {
    try {
      const suiteUrl = pathToFileURL(path.resolve(extensionTestsPath)).href;
      const mod = await import(suiteUrl);
      if (typeof mod.run !== "function") {
        throw new Error("Integration suite must export async function run(ctx)");
      }
      const whenFirstWindowLoaded = async (): Promise<void> => {
        const w = BrowserWindow.getAllWindows()[0];
        if (!w) {
          throw new Error("No BrowserWindow available");
        }
        await new Promise<void>((resolve, reject) => {
          if (w.webContents.isLoading()) {
            w.webContents.once("did-finish-load", () => resolve());
            w.webContents.once("did-fail-load", (_e, code, desc) => reject(new Error(`did-fail-load: ${code} ${desc}`)));
          } else {
            resolve();
          }
        });
      };
      await mod.run({ app, BrowserWindow, path, whenFirstWindowLoaded });
      app.exit(0);
    } catch (err) {
      console.error("[semio desktop integration tests]", err);
      app.exit(1);
    }
  }
  // #endregion DesktopIntegrationTests
});

// #endregion 🐙Main Process
