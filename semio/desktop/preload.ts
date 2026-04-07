// #region 🔖Header
// [👤semio🖱️desktop💻preload](repo://p/u/semio/b/u/desktop/f/preload.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Electron preload script exposing safe APIs to the renderer process.

// #endregion 🔖Header

// #region 🔖Preload
// [👤semio🖱️desktop💻preload🔖preload](repo://p/u/semio/b/u/desktop/f/preload.ts/s/Preload)
// Electron preload script exposing window controls and OS APIs to the renderer.
// Preload MUST use contextBridge to safely expose IPC methods.

import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld("windowControls", {
  minimize: () => ipcRenderer.invoke("minimize-window"),
  maximize: () => ipcRenderer.invoke("maximize-window"),
  close: () => ipcRenderer.invoke("close-window"),
});

contextBridge.exposeInMainWorld("os", {
  getUserId: () => ipcRenderer.invoke("get-user-id"),
});

// #region 🔖FolderBridge
// Exposes folder-based kit storage operations to the renderer process.
contextBridge.exposeInMainWorld("kitFolder", {
  selectFolder: () => ipcRenderer.invoke("select-folder"),
  readKit: (folderPath: string) => ipcRenderer.invoke("read-kit", folderPath),
  writeKit: (folderPath: string, data: ArrayBuffer) => ipcRenderer.invoke("write-kit", folderPath, data),
  readFile: (folderPath: string, filePath: string) => ipcRenderer.invoke("read-file", folderPath, filePath),
  writeFile: (folderPath: string, filePath: string, data: ArrayBuffer) => ipcRenderer.invoke("write-file", folderPath, filePath, data),
  deleteFile: (folderPath: string, filePath: string) => ipcRenderer.invoke("delete-file", folderPath, filePath),
  listFiles: (folderPath: string) => ipcRenderer.invoke("list-files", folderPath),
  getRecentFolders: () => ipcRenderer.invoke("get-recent-folders"),
  addRecentFolder: (folderPath: string) => ipcRenderer.invoke("add-recent-folder", folderPath),
});
// #endregion 🔖FolderBridge
// #endregion 🔖Preload
