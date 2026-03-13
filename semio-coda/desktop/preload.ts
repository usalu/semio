// #region 🔖Header
// [🔬coda🖱️desktop💻preload](semiorepo://p/r/coda/b/u/desktop/f/preload.ts)

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

// Electron preload script exposing safe APIs to the renderer process.

// #endregion 🔖Header

// #region 🔖Preload
// [🔬coda🖱️desktop💻preload🔖preload](semiorepo://p/r/coda/b/u/desktop/f/preload.ts/s/Preload)
// Electron preload script exposing window controls, OS APIs, and coda sidecar bridge.
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

contextBridge.exposeInMainWorld("coda", {
  // Direct sidecar method call (preferred)
  call: (method: string, params: Record<string, unknown> = {}) =>
    ipcRenderer.invoke("coda-call", method, params),
  // Backward-compatible resource fetch
  fetch: (uri: string) => ipcRenderer.invoke("coda-fetch", uri),
  // Backward-compatible tool call
  tool: (name: string, args: Record<string, unknown>) =>
    ipcRenderer.invoke("coda-tool", name, args),
});

contextBridge.exposeInMainWorld("dialog", {
  openFolder: (): Promise<string | null> => ipcRenderer.invoke("dialog-open-folder"),
});

contextBridge.exposeInMainWorld("project", {
  getPath: (): Promise<string | null> => ipcRenderer.invoke("get-project-path"),
  open: (folder: string): Promise<{ success: boolean; error?: string }> =>
    ipcRenderer.invoke("project-open", folder),
  create: (name: string, folder: string): Promise<{ success: boolean; error?: string }> =>
    ipcRenderer.invoke("project-create", name, folder),
});
// #endregion 🔖Preload
