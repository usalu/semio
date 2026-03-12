// #region 🔖Header
// [🔬coda🖱️desktop💻main](semiorepo://p/r/coda/b/u/desktop/f/main.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron main process managing windows and lifecycle.

// #endregion 🔖Header

// #region 🔖Main Process
// [🔬coda🖱️desktop💻main🔖mainprocess](semiorepo://p/r/coda/b/u/desktop/f/main.ts/s/Main%20Process)
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

import { app, BrowserWindow, ipcMain } from "electron";
import started from "electron-squirrel-startup";
import path from "node:path";
import os from "os";

if (started) {
  app.quit();
}

/**
 * Creates the main Electron browser window with preload and vite integration.
// [🔬coda🖱️desktop💻main🔖mainprocess🛠️createwindow](semiorepo://p/r/coda/b/u/desktop/f/main.ts/s/Main%20Process/d/i/createWindow)
 *
 * MUST load the vite dev server URL in development and the built file in production.
 **/
const createWindow = () => {
  const mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    autoHideMenuBar: true,
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
  app.setAppUserModelId("com.semio.coda");

  ipcMain.handle("minimize-window", () => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) window.minimize();
  });
  ipcMain.handle("maximize-window", () => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) {
      if (window.isMaximized()) {
        window.unmaximize();
      } else {
        window.maximize();
      }
    }
  });
  ipcMain.handle("close-window", () => {
    const window = BrowserWindow.getFocusedWindow();
    if (window) window.close();
  });

  ipcMain.handle("get-user-id", () => {
    return os.userInfo().username;
  });

  ipcMain.handle("coda-fetch", async (_event, uri: string) => {
    const base = process.env.CODA_MCP_URL ?? "http://127.0.0.1:8080";
    const endpoint = `${base}/mcp`;
    const fullUri = uri.startsWith("coda://") ? uri : `coda://${uri}`;
    const res = await fetch(endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "resources/read",
        params: { uri: fullUri },
      }),
    });
    const data = await res.json();
    return data;
  });

  ipcMain.handle("coda-tool", async (_event, name: string, args: Record<string, unknown>) => {
    const base = process.env.CODA_MCP_URL ?? "http://127.0.0.1:8080";
    const endpoint = `${base}/mcp`;
    const res = await fetch(endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 2,
        method: "tools/call",
        params: { name, arguments: args ?? {} },
      }),
    });
    const data = await res.json();
    return data;
  });
});

// #endregion 🔖Main Process
