// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron main process managing windows and lifecycle.

// #endregion 🧲Header

// #region 🐙Main Process
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

import { app, BrowserWindow, dialog, ipcMain } from "electron";
import started from "electron-squirrel-startup";
import path from "node:path";
import fs from "node:fs";
import os from "os";

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
  } else {
    mainWindow.loadFile(path.join(__dirname, `../renderer/${MAIN_WINDOW_VITE_NAME}/index.html`));
  }
};

app.on("ready", createWindow);

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

app.whenReady().then(() => {
  app.setAppUserModelId("com.electron");

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

  ipcMain.handle("read-file", async (_event, folderPath: string, filePath: string) => {
    const fullPath = path.join(folderPath, filePath);
    try {
      const buffer = fs.readFileSync(fullPath);
      return buffer.buffer;
    } catch {
      return null;
    }
  });

  ipcMain.handle("write-file", async (_event, folderPath: string, filePath: string, data: ArrayBuffer) => {
    const fullPath = path.join(folderPath, filePath);
    const dir = path.dirname(fullPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(fullPath, Buffer.from(data));
  });

  ipcMain.handle("delete-file", async (_event, folderPath: string, filePath: string) => {
    const fullPath = path.join(folderPath, filePath);
    try {
      fs.unlinkSync(fullPath);
    } catch {
      /* ignore */
    }
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
  // #endregion 🗂️FolderIPC
});

// #endregion 🐙Main Process
