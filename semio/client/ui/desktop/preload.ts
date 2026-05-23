// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Electron preload script exposing safe APIs to the renderer process.

// #endregion 🧲Header

// #region 🎋Preload
// Electron preload script exposing window controls and OS APIs to the renderer.
// Preload MUST use contextBridge to safely expose IPC methods.

import { contextBridge, ipcRenderer } from "electron";

// E2E / automation: when set, desktop renderer opens this folder as the kit root without a native dialog.
const e2eKitFolder = process.env.SEMIO_E2E_KIT_FOLDER?.trim() ?? "";
if (e2eKitFolder.length > 0) {
  contextBridge.exposeInMainWorld("__SEMIO_E2E_KIT_FOLDER__", e2eKitFolder);
}

const e2eKitFile = process.env.SEMIO_E2E_KIT_FILE?.trim() ?? "";
if (e2eKitFile.length > 0) {
  contextBridge.exposeInMainWorld("__SEMIO_E2E_KIT_FILE__", e2eKitFile);
}

contextBridge.exposeInMainWorld("windowControls", {
  minimize: () => ipcRenderer.invoke("minimize-window"),
  maximize: () => ipcRenderer.invoke("maximize-window"),
  close: () => ipcRenderer.invoke("close-window"),
});

contextBridge.exposeInMainWorld("os", {
  getUserId: () => ipcRenderer.invoke("get-user-id"),
});

// #region 🎗️FolderBridge
// Exposes folder-based kit storage operations to the renderer process.
contextBridge.exposeInMainWorld("kitFolder", {
  selectFolder: () => ipcRenderer.invoke("select-folder"),
  readKit: (folderPath: string) => ipcRenderer.invoke("read-kit", folderPath),
  writeKit: (folderPath: string, data: ArrayBuffer) => ipcRenderer.invoke("write-kit", folderPath, data),
  readFile: (folderPath: string, filePath: string) => ipcRenderer.invoke("read-file", folderPath, filePath),
  writeFile: (folderPath: string, filePath: string, data: ArrayBuffer) => ipcRenderer.invoke("write-file", folderPath, filePath, data),
  deleteFile: (folderPath: string, filePath: string) => ipcRenderer.invoke("delete-file", folderPath, filePath),
  createDirectory: (folderPath: string, directoryPath: string) => ipcRenderer.invoke("create-directory", folderPath, directoryPath),
  moveEntry: (folderPath: string, fromPath: string, toPath: string) => ipcRenderer.invoke("move-entry", folderPath, fromPath, toPath),
  listFiles: (folderPath: string) => ipcRenderer.invoke("list-files", folderPath),
  getRecentFolders: () => ipcRenderer.invoke("get-recent-folders"),
  addRecentFolder: (folderPath: string) => ipcRenderer.invoke("add-recent-folder", folderPath),
  watchFolder: (folderPath: string, onChanged: () => void) => {
    ipcRenderer.send("kit-folder-watch-subscribe", folderPath);
    const handler = (_e: unknown, changedPath: string) => {
      if (changedPath === folderPath) onChanged();
    };
    ipcRenderer.on("kit-folder-changed", handler);
    return () => {
      ipcRenderer.removeListener("kit-folder-changed", handler);
      ipcRenderer.send("kit-folder-watch-unsubscribe", folderPath);
    };
  },
});
// #endregion 🎗️FolderBridge

// #region 📄FileBridge
// Exposes JSON-file-based kit storage operations to the renderer process.
contextBridge.exposeInMainWorld("kitFile", {
  selectFile: () => ipcRenderer.invoke("select-file"),
  readJson: (filePath: string) => ipcRenderer.invoke("read-kit-json-file", filePath),
  writeJson: (filePath: string, json: string) => ipcRenderer.invoke("write-kit-json-file", filePath, json),
});
// #endregion 📄FileBridge
// #endregion 🎋Preload
