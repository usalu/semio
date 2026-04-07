// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron main process managing windows, sidecar lifecycle, IPC, and event forwarding.

// #endregion 🧲Header

// #region 🔮Sidecar Bridge
// Manages the Python sidecar child process over JSON-over-stdio.
// MUST handle spawning, request/response correlation, heartbeats, timeouts, auto-restart, and event forwarding.

import { app, BrowserWindow, dialog, ipcMain, net } from "electron";
import started from "electron-squirrel-startup";
import { ChildProcess, spawn } from "node:child_process";
import path from "node:path";
import { createInterface, Interface as ReadlineInterface } from "node:readline";
import os from "os";
import { randomUUID } from "node:crypto";
import fs from "node:fs";

// Disable Chromium sandbox in containerized environments (devcontainers, Docker)
// where PID/network namespace creation is not permitted.
if (process.env.ELECTRON_DISABLE_SANDBOX === "1") {
  app.commandLine.appendSwitch("no-sandbox");
}

if (started) {
  app.quit();
}

// Sidecar configuration
// In dev, __dirname is desktop/.vite/build. The engine dir is a sibling of desktop/.
// 🛤️app.getAppPath() returns the desktop/ dir in dev.
const SIDECAR_PY_DIR = path.resolve(app.isPackaged ? path.join(process.resourcesPath, "engine") : path.join(app.getAppPath(), "..", "engine"));
const SIDECAR_CMD = process.env.CODA_SIDECAR_CMD ?? "uv";
const SIDECAR_BASE_ARGS = process.env.CODA_SIDECAR_CMD ? ["--sidecar"] : ["run", "--active", "coda.py", "--sidecar"];
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
let sidecarConnected = false;
let currentProjectPath: string | null = process.env.CODA_PROJECT ?? null;

function broadcastToRenderers(channel: string, ...args: unknown[]): void {
  for (const win of BrowserWindow.getAllWindows()) {
    win.webContents.send(channel, ...args);
  }
}

function setSidecarConnected(connected: boolean): void {
  if (sidecarConnected !== connected) {
    sidecarConnected = connected;
    broadcastToRenderers("coda-connection-status", connected);
  }
}

function startSidecar(): void {
  if (shuttingDown) return;

  const args = [...SIDECAR_BASE_ARGS];
  const projectPath = process.env.CODA_PROJECT;
  if (projectPath) {
    args.push("--project", projectPath);
  }

  console.error(`[coda sidecar] spawning: ${SIDECAR_CMD} ${args.join(" ")} (cwd: ${SIDECAR_PY_DIR})`);
  sidecar = spawn(SIDECAR_CMD, args, {
    stdio: ["pipe", "pipe", "pipe"],
    cwd: SIDECAR_PY_DIR,
    env: { ...process.env },
  });

  sidecarRL = createInterface({ input: sidecar.stdout! });

  sidecarRL.on("line", (line: string) => {
    if (!line.trim()) return;
    try {
      const msg = JSON.parse(line);
      const id = msg.id as string | null;

      // Event messages: id===null and have "event" field
      if (id === null && msg.event) {
        broadcastToRenderers("coda-event", {
          event: msg.event,
          data: msg.data ?? {},
          timestamp: msg.timestamp ?? Date.now() / 1000,
        });
        return;
      }

      // Ready signal
      if (id === null && msg.result?.status === "ready") {
        setSidecarConnected(true);
        broadcastToRenderers("coda-event", {
          event: "sidecar_ready",
          data: { pid: msg.result.pid },
          timestamp: Date.now() / 1000,
        });
        return;
      }

      // Response to a pending request
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
    setSidecarConnected(false);
    broadcastToRenderers("coda-event", {
      event: "sidecar_disconnected",
      data: { code, signal },
      timestamp: Date.now() / 1000,
    });
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
  for (const [, pending] of pendingRequests) {
    clearTimeout(pending.timer);
    pending.reject({ code: -32000, message: "Sidecar disconnected" });
  }
  pendingRequests.clear();
  sidecar = null;
}

function stopSidecar(): void {
  shuttingDown = true;
  setSidecarConnected(false);
  cleanup();
  if (sidecar && !sidecar.killed) {
    sidecar.kill("SIGTERM");
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
      const timeout = new Promise((_, reject) => setTimeout(() => reject(new Error("heartbeat timeout")), HEARTBEAT_TIMEOUT_MS));
      await Promise.race([hbPromise, timeout]);
      restartCount = 0;
      setSidecarConnected(true);
    } catch {
      console.error("[coda sidecar] heartbeat failed, killing process");
      setSidecarConnected(false);
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

// #endregion 🔮Sidecar Bridge

// #region 🐙Main Process
// Electron main process that creates the browser window and registers IPC handlers.
// MUST quit on all windows closed except on macOS.

/**
 * Creates the main Electron browser window with preload and vite integration.
 *
 * MUST load the vite dev server URL in development and the built file in production.
 **/
const createWindow = async () => {
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
    // Pre-warm the Vite module graph by fetching the renderer AND its heavy
    // dependencies. The elements.tsx file is 1.2MB and takes seconds to
    // transform. If we don't warm it up, the browser's HTTP/1.1 connection
    // pool (6 connections) stalls while Vite transforms it on-demand,
    // 🌳blocking the entire ESM module tree from resolving.
    const baseUrl = MAIN_WINDOW_VITE_DEV_SERVER_URL.replace(/\/$/, "");
    const warmupUrls = [`${baseUrl}/renderer.tsx`, `${baseUrl}/@fs${path.resolve(app.getAppPath(), "..", "..", "elements", "ui", "elements.tsx").replace(/\\/g, "/")}`];
    console.log(`[coda desktop] Pre-warming Vite module graph (${warmupUrls.length} entries)...`);
    try {
      await Promise.all(
        warmupUrls.map(async (url) => {
          try {
            const response = await net.fetch(url, { signal: AbortSignal.timeout(120000) });
            await response.text();
            console.log(`[coda desktop] Warm: ${url.split("/").pop()}`);
          } catch (e) {
            console.log(`[coda desktop] Warmup failed for ${url.split("/").pop()}: ${e}`);
          }
        }),
      );
      console.log("[coda desktop] Vite module graph warm.");
    } catch (e) {
      console.log(`[coda desktop] Vite warmup failed: ${e}, proceeding anyway.`);
    }

    // 🖥️Retry loading with backoff to handle Vite dev server startup race
    const loadWithRetry = async (retries = 5, delay = 1000) => {
      for (let i = 0; i < retries; i++) {
        try {
          await mainWindow.loadURL(MAIN_WINDOW_VITE_DEV_SERVER_URL);
          return;
        } catch {
          if (i < retries - 1) {
            console.log(`[coda desktop] Vite not ready, retrying in ${delay}ms... (${i + 1}/${retries})`);
            await new Promise((r) => setTimeout(r, delay));
            delay *= 2;
          }
        }
      }
    };
    await loadWithRetry();

    // [DEBUG] Forward renderer console to main process stdout (Electron 40 API)
    mainWindow.webContents.on("console-message", (event: { level: number; message: string }) => {
      const tag = ["LOG", "WARN", "ERR", "INFO"][event.level] ?? "?";
      console.log(`[DEBUG] renderer[${tag}]: ${event.message?.substring(0, 500)}`);
    });
    // [DEBUG] Monitor failed network requests
    mainWindow.webContents.session.webRequest.onErrorOccurred((details: { url: string; error: string }) => {
      console.log(`[DEBUG] net-error: ${details.error} ${details.url.substring(0, 200)}`);
    });
    // 📨[DEBUG] Monitor completed requests to track module loading progress
    let requestCount = 0;
    mainWindow.webContents.session.webRequest.onCompleted((details: { url: string; statusCode: number }) => {
      requestCount++;
      if (requestCount <= 5 || requestCount % 20 === 0) {
        console.log(`[DEBUG] req #${requestCount}: ${details.statusCode} ${details.url.replace("http://localhost:5173/", "").substring(0, 100)}`);
      }
    });
    // [DEBUG] Check DOM state at 10s and 30s
    for (const delay of [10000, 30000, 60000]) {
      setTimeout(async () => {
        try {
          const diag = await mainWindow.webContents.executeJavaScript(`
            JSON.stringify({
              t: ${delay / 1000},
              rootChildren: document.getElementById('root')?.children.length ?? -1,
              rootHTML: document.getElementById('root')?.innerHTML?.substring(0, 200) ?? 'EMPTY',
              errors: window.__coda_errors || [],
              resources: performance.getEntriesByType('resource').length,
              pendingFetches: performance.getEntriesByType('resource').filter(function(r) { return r.responseEnd === 0; }).length,
              lastFive: performance.getEntriesByType('resource').slice(-5).map(function(r) { return r.name.replace('http://localhost:5173/', '').substring(0, 80); }),
            })
          `);
          console.log("[DEBUG] DOM@" + delay / 1000 + "s: " + diag);
        } catch (e) {
          console.error("[DEBUG] DOM check failed:", e);
        }
      }, delay);
    }
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

  // Connection status query
  ipcMain.handle("coda-connection-status", () => {
    return sidecarConnected;
  });

  // Open folder dialog
  ipcMain.handle("dialog-open-folder", async () => {
    const result = await dialog.showOpenDialog({
      properties: ["openDirectory", "createDirectory"],
    });
    return result.canceled ? null : (result.filePaths[0] ?? null);
  });

  // Project management
  ipcMain.handle("project-get-path", () => {
    return currentProjectPath;
  });

  ipcMain.handle("project-open", async (_event, folder: string) => {
    const projectJson = path.join(folder, ".coda", "project.json");
    if (!fs.existsSync(projectJson)) {
      return { success: false, error: "No .coda/project.json found in this folder." };
    }
    switchProject(folder);
    return { success: true };
  });

  ipcMain.handle("project-create", async (_event, name: string, folder: string) => {
    const codaDir = path.join(folder, ".coda");
    const projectJsonPath = path.join(codaDir, "project.json");
    try {
      fs.mkdirSync(codaDir, { recursive: true });
      fs.writeFileSync(projectJsonPath, JSON.stringify({ design: { id: name }, targets: [] }, null, 2), "utf-8");
      switchProject(folder);
      return { success: true };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : "Unknown error";
      return { success: false, error: message };
    }
  });

  // Sidecar bridge: all coda operations go through the sidecar
  ipcMain.handle("coda-call", async (_event, method: string, params: Record<string, unknown>) => {
    if (!sidecarConnected) {
      return { error: { code: -32000, message: "Sidecar not connected. Desktop is in offline mode." } };
    }
    try {
      return await sendRequest(method, params ?? {});
    } catch (err) {
      return { error: err };
    }
  });

  // Resource fetch — wraps sidecar responses into MCP resource format
  ipcMain.handle("coda-fetch", async (_event, uri: string) => {
    if (!sidecarConnected) {
      return { error: { code: -32000, message: "Sidecar not connected. Desktop is in offline mode." } };
    }

    function wrapAsResource(data: unknown, resourceUri: string) {
      return {
        jsonrpc: "2.0",
        id: 1,
        result: {
          contents: [{ uri: resourceUri, mimeType: "application/json", text: JSON.stringify(data) }],
        },
      };
    }

    const resourceMapping: Record<string, { method: string; params?: Record<string, unknown> }> = {
      "coda://property-kinds": { method: "get_property_kinds" },
      "coda://correlation": { method: "get_correlation" },
      "coda://properties": { method: "get_properties" },
      "coda://frameworks": { method: "get_frameworks" },
      "coda://measures": { method: "get_measures" },
      "coda://targets": { method: "get_targets" },
      "coda://platforms": { method: "get_platforms" },
      "coda://project": { method: "get_project" },
      "coda://report": { method: "get_report" },
      "coda://breachs": { method: "get_breachs" },
      "coda://iterations": { method: "get_iterations" },
      "coda://current-run": { method: "get_current_run" },
      "coda://current-iteration": { method: "get_current_iteration" },
    };
    const fullUri = uri.startsWith("coda://") ? uri : `coda://${uri}`;

    // 🎛️Handle parameterized URIs like coda://translation/{target_id}
    const translationMatch = fullUri.match(/^coda:\/\/translation\/(.+)$/);
    if (translationMatch) {
      try {
        const result = await sendRequest("get_translation", { target_id: translationMatch[1] });
        return wrapAsResource(result, fullUri);
      } catch (err) {
        return { error: err };
      }
    }

    const mapped = resourceMapping[fullUri];
    if (mapped) {
      try {
        const result = await sendRequest(mapped.method, mapped.params ?? {});
        return wrapAsResource(result, fullUri);
      } catch (err) {
        return { error: err };
      }
    }
    return { error: { message: `Unknown resource: ${fullUri}` } };
  });

  // Tool call — wraps sidecar responses into MCP tool format
  ipcMain.handle("coda-tool", async (_event, name: string, args: Record<string, unknown>) => {
    if (!sidecarConnected) {
      return { error: { code: -32000, message: "Sidecar not connected. Desktop is in offline mode." } };
    }
    try {
      const result = await sendRequest(name, args ?? {});
      return {
        jsonrpc: "2.0",
        id: 1,
        result: {
          content: [{ type: "text", text: JSON.stringify(result) }],
        },
      };
    } catch (err) {
      return { error: err };
    }
  });
});

// #endregion 🐙Main Process
