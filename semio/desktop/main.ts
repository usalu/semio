// #region 🔖Header

// [💻semio/desktop/main.ts](semiorepo://file/semio/desktop/main.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// [🔖semio/desktop/main.ts#Main Process](semiorepo://section/semio/desktop/main.ts/MAIN-PROCESS)
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

import { app, BrowserWindow, ipcMain } from "electron";
import started from "electron-squirrel-startup";
import path from "node:path";
import os from "os";

if (started) {
  app.quit();
}

// Creates the main Electron browser window with preload and vite integration.
// MUST load the vite dev server URL in development and the built file in production.
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
});

// #endregion 🔖Main Process
