// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Electron preload script exposing safe APIs to the renderer process.

// #endregion 🧲Header

// #region 🔌Adapters
// Electron preload script exposing window controls, OS APIs, coda sidecar bridge, and event system.
// Preload MUST use contextBridge to safely expose IPC methods.
// Preload MUST expose event listener registration for sidecar events and connection status.

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
  call: (method: string, params: Record<string, unknown> = {}) =>
    ipcRenderer.invoke("coda-call", method, params),
  fetch: (uri: string) => ipcRenderer.invoke("coda-fetch", uri),
  tool: (name: string, args: Record<string, unknown>) =>
    ipcRenderer.invoke("coda-tool", name, args),
  getConnectionStatus: () => ipcRenderer.invoke("coda-connection-status"),
  onEvent: (callback: (event: { event: string; data: Record<string, unknown>; timestamp: number }) => void) => {
    const handler = (_ipcEvent: Electron.IpcRendererEvent, evt: { event: string; data: Record<string, unknown>; timestamp: number }) => callback(evt);
    ipcRenderer.on("coda-event", handler);
    return () => { ipcRenderer.removeListener("coda-event", handler); };
  },
  onConnectionStatus: (callback: (connected: boolean) => void) => {
    const handler = (_ipcEvent: Electron.IpcRendererEvent, connected: boolean) => callback(connected);
    ipcRenderer.on("coda-connection-status", handler);
    return () => { ipcRenderer.removeListener("coda-connection-status", handler); };
  },
});

contextBridge.exposeInMainWorld("dialog", {
  openFolder: () => ipcRenderer.invoke("dialog-open-folder"),
});

contextBridge.exposeInMainWorld("project", {
  getPath: () => ipcRenderer.invoke("project-get-path"),
  open: (folder: string) => ipcRenderer.invoke("project-open", folder),
  create: (name: string, folder: string) => ipcRenderer.invoke("project-create", name, folder),
});
// #endregion 🎋Preload
