// #region 🔖Header

// [💻semio/desktop/preload.ts](semiorepo://file/semio/desktop/preload.ts)

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

// Electron preload script exposing safe APIs to the renderer process.

// #endregion 🔖Header

// #region 🔖Preload
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
// #endregion 🔖Preload
