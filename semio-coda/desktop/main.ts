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

// Entry point for the Electron main process managing windows, sidecar lifecycle, and IPC.

// #endregion 🔖Header

// #region 🔖Sidecar Bridge
// [🔬coda🖱️desktop💻main🔖sidecarbridge](semiorepo://p/r/coda/b/u/desktop/f/main.ts/s/Sidecar%20Bridge)
// Manages the Python sidecar child process over JSON-over-stdio.
// MUST handle spawning, request/response correlation, heartbeats, timeouts, and auto-restart.

import { app, BrowserWindow, dialog, ipcMain } from "electron";
import started from "electron-squirrel-startup";
import { ChildProcess, spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { createInterface, Interface as ReadlineInterface } from "node:readline";
import os from "os";
import { randomUUID } from "node:crypto";

if (started) {
  app.quit();
}

// Sidecar configuration
const SIDECAR_CMD = process.env.CODA_SIDECAR_CMD ?? "coda";
const SIDECAR_ARGS = ["--sidecar"];
const HEARTBEAT_INTERVAL_MS = 10_000;
const HEARTBEAT_TIMEOUT_MS = 5_000;
const REQUEST_TIMEOUT_MS = 30_000;
const MAX_RESTART_ATTEMPTS = 5;
const RESTART_BACKOFF_MS = 1_000;

interface PendingRequest {
  resolve: (value: unknown) => void;
  reject: (reason: unknown) => void;
  timer: ReturnType<typeof setTimeout>;
}

let sidecar: ChildProcess | null = null;
let sidecarRL: ReadlineInterface | null = null;
let pendingRequests = new Map<string, PendingRequest>();
let heartbeatTimer: ReturnType<typeof setInterval> | null = null;
let restartCount = 0;
let shuttingDown = false;
let currentProjectPath: string | null = process.env.CODA_PROJECT ?? null;

function startSidecar(): void {
  if (shuttingDown) return;

  const args = [...SIDECAR_ARGS];
  if (currentProjectPath) {
    args.push("--project", currentProjectPath);
  }

  sidecar = spawn(SIDECAR_CMD, args, {
    stdio: ["pipe", "pipe", "pipe"],
    env: { ...process.env },
  });

  sidecarRL = createInterface({ input: sidecar.stdout! });

  sidecarRL.on("line", (line: string) => {
    if (!line.trim()) return;
    try {
      const msg = JSON.parse(line);
      const id = msg.id as string | null;
      if (id && pendingRequests.has(id)) {
        const pending = pendingRequests.get(id)!;
        clearTimeout(pending.timer);
        pendingRequests.delete(id);
        if (msg.error) {
          pending.reject(msg.error);
        } else {
          pending.resolve(msg.result);
        }
      }
      // id===null messages (e.g. ready signal) are informational
    } catch {
      // Ignore unparseable lines (e.g. Python warnings)
    }
  });

  sidecar.stderr?.on("data", (data: Buffer) => {
    console.error(`[coda sidecar stderr] ${data.toString().trim()}`);
  });

  sidecar.on("error", (err) => {
    console.error(`[coda sidecar] spawn error: ${err.message}`);
    cleanup();
  });

  sidecar.on("exit", (code, signal) => {
    console.error(`[coda sidecar] exited code=${code} signal=${signal}`);
    cleanup();
    if (!shuttingDown && restartCount < MAX_RESTART_ATTEMPTS) {
      restartCount++;
      const delay = RESTART_BACKOFF_MS * restartCount;
      console.error(`[coda sidecar] restarting in ${delay}ms (attempt ${restartCount}/${MAX_RESTART_ATTEMPTS})`);
      setTimeout(startSidecar, delay);
    } else if (!shuttingDown) {
      console.error("[coda sidecar] max restart attempts reached");
    }
  });

  startHeartbeat();
}

function cleanup(): void {
  if (heartbeatTimer) {
    clearInterval(heartbeatTimer);
    heartbeatTimer = null;
  }
  if (sidecarRL) {
    sidecarRL.close();
    sidecarRL = null;
  }
  // Reject all pending requests
  for (const [id, pending] of pendingRequests) {
    clearTimeout(pending.timer);
    pending.reject({ code: -32000, message: "Sidecar disconnected" });
  }
  pendingRequests.clear();
  sidecar = null;
}

function stopSidecar(): void {
  shuttingDown = true;
  cleanup();
  if (sidecar && !sidecar.killed) {
    sidecar.kill("SIGTERM");
    // Force kill after 2s if still alive
    setTimeout(() => {
      if (sidecar && !sidecar.killed) {
        sidecar.kill("SIGKILL");
      }
    }, 2000);
  }
}

function sendRequest(method: string, params: Record<string, unknown> = {}): Promise<unknown> {
  return new Promise((resolve, reject) => {
    if (!sidecar || !sidecar.stdin?.writable) {
      reject({ code: -32000, message: "Sidecar not running" });
      return;
    }

    const id = randomUUID();
    const timer = setTimeout(() => {
      pendingRequests.delete(id);
      reject({ code: -32000, message: `Request timed out: ${method}` });
    }, REQUEST_TIMEOUT_MS);

    pendingRequests.set(id, { resolve, reject, timer });

    const request = JSON.stringify({ id, method, params }) + "\n";
    sidecar.stdin.write(request);
  });
}

function startHeartbeat(): void {
  if (heartbeatTimer) clearInterval(heartbeatTimer);
  heartbeatTimer = setInterval(async () => {
    try {
      const hbPromise = sendRequest("heartbeat");
      const timeout = new Promise((_, reject) =>
        setTimeout(() => reject(new Error("heartbeat timeout")), HEARTBEAT_TIMEOUT_MS)
      );
      await Promise.race([hbPromise, timeout]);
      // Heartbeat succeeded, reset restart counter
      restartCount = 0;
    } catch {
      console.error("[coda sidecar] heartbeat failed, killing process");
      if (sidecar && !sidecar.killed) {
        sidecar.kill("SIGTERM");
      }
    }
  }, HEARTBEAT_INTERVAL_MS);
}

function switchProject(newPath: string): void {
  if (heartbeatTimer) {
    clearInterval(heartbeatTimer);
    heartbeatTimer = null;
  }
  if (sidecarRL) {
    sidecarRL.close();
    sidecarRL = null;
  }
  for (const [, pending] of pendingRequests) {
    clearTimeout(pending.timer);
    pending.reject({ code: -32000, message: "Project switched" });
  }
  pendingRequests.clear();
  if (sidecar && !sidecar.killed) {
    sidecar.kill("SIGTERM");
    setTimeout(() => {
      if (sidecar && !sidecar.killed) sidecar.kill("SIGKILL");
    }, 2000);
  }
  sidecar = null;
  currentProjectPath = newPath;
  shuttingDown = false;
  restartCount = 0;
  startSidecar();
}

// #endregion 🔖Sidecar Bridge

// #region 🔖Main Process
// [🔬coda🖱️desktop💻main🔖mainprocess](semiorepo://p/r/coda/b/u/desktop/f/main.ts/s/Main%20Process)
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

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

app.on("ready", () => {
  if (currentProjectPath) {
    startSidecar();
  }
  createWindow();
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    stopSidecar();
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

app.on("before-quit", () => {
  stopSidecar();
});

app.whenReady().then(() => {
  app.setAppUserModelId("com.semio.coda");

  // Window controls
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

  // Sidecar bridge: all coda operations go through the sidecar
  ipcMain.handle("coda-call", async (_event, method: string, params: Record<string, unknown>) => {
    try {
      return await sendRequest(method, params ?? {});
    } catch (err) {
      return { error: err };
    }
  });

  // Keep backward-compatible resource/tool endpoints that route through sidecar
  ipcMain.handle("coda-fetch", async (_event, uri: string) => {
    // Map resource URIs to sidecar method calls
    const resourceMapping: Record<string, { method: string; params?: Record<string, unknown> }> = {
      "coda://measures": { method: "get_measures" },
      "coda://targets": { method: "get_targets" },
      "coda://project": { method: "get_project" },
      "coda://report": { method: "get_report" },
      "coda://breachs": { method: "get_breachs" },
      "coda://iterations": { method: "get_iterations" },
    };
    const fullUri = uri.startsWith("coda://") ? uri : `coda://${uri}`;
    const mapped = resourceMapping[fullUri];
    if (mapped) {
      try {
        return await sendRequest(mapped.method, mapped.params ?? {});
      } catch (err) {
        return { error: err };
      }
    }
    // If not mapped, return error
    return { error: { message: `Unknown resource: ${fullUri}` } };
  });

  ipcMain.handle("coda-tool", async (_event, name: string, args: Record<string, unknown>) => {
    try {
      return await sendRequest(name, args ?? {});
    } catch (err) {
      return { error: err };
    }
  });

  // #region 🔖Project Management
  // IPC handlers for project selection, creation, and path queries.
  // MUST validate folders and scaffold .coda/project.json on creation.

  ipcMain.handle("get-project-path", () => {
    return currentProjectPath;
  });

  ipcMain.handle("dialog-open-folder", async () => {
    const win = BrowserWindow.getFocusedWindow();
    const result = await dialog.showOpenDialog(win!, {
      properties: ["openDirectory", "createDirectory"],
    });
    if (result.canceled || result.filePaths.length === 0) return null;
    return result.filePaths[0];
  });

  ipcMain.handle("project-open", async (_event, folder: string) => {
    const codaDir = path.join(folder, ".coda");
    const projectFile = path.join(codaDir, "project.json");
    if (!fs.existsSync(projectFile)) {
      return { success: false, error: "No .coda/project.json found in selected folder." };
    }
    switchProject(folder);
    return { success: true };
  });

  ipcMain.handle("project-create", async (_event, name: string, folder: string) => {
    try {
      const codaDir = path.join(folder, ".coda");
      fs.mkdirSync(codaDir, { recursive: true });
      const projectJson = JSON.stringify({ design: { id: name }, targets: [] }, null, 2);
      fs.writeFileSync(path.join(codaDir, "project.json"), projectJson, "utf8");
      switchProject(folder);
      return { success: true };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      return { success: false, error: message };
    }
  });

  // #endregion 🔖Project Management
});

// #endregion 🔖Main Process
