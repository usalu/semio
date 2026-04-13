// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron renderer process mounting the React app.

// #endregion 🧲Header

// #region ⛩️Renderer
// Electron renderer process that mounts the Sketchpad React app with window controls.
// MUST resolve the user identity before rendering the sketchpad.

import React, { useEffect, useState, useCallback, lazy, Suspense } from "react";
import { createRoot, type Root } from "react-dom/client";
import { createFolderKitStore, createSessionKitStore } from "@semio/studio";
import type { KitFolderAdapter, KitJsonFileAdapter } from "@semio/studio";
import type { SketchpadKitStoreFactory } from "@semio/sketchpad";
import { createJsonFileKitStore } from "@semio/sketchpad";
import { InMemoryKitStore } from "@semio/js";

import "./globals.css";

// 🎨Lazy-load the heavy sketchpad module (500KB+) to avoid blocking the renderer.
const LazySketchpad = lazy(() =>
  import("@semio/sketchpad").then((mod) => {
    mod.appRegistry.register(mod.designConfig);
    mod.appRegistry.register(mod.docsConfig);
    mod.appRegistry.register(mod.feedbackConfig);
    mod.appRegistry.register(mod.homeConfig);
    mod.appRegistry.register(mod.kitConfig);
    mod.appRegistry.register(mod.qualityConfig);
    mod.appRegistry.register(mod.typeConfig);
    return { default: mod.Sketchpad };
  }),
);

declare global {
  interface Window {
    /** Set by preload when `SEMIO_E2E_KIT_FOLDER` is defined (desktop E2E / automation). */
    __SEMIO_E2E_KIT_FOLDER__?: string;
    /** Set by preload when `SEMIO_E2E_KIT_FILE` is defined (desktop E2E / automation). */
    __SEMIO_E2E_KIT_FILE__?: string;
    windowControls: {
      minimize(): Promise<any>;
      maximize(): Promise<any>;
      close(): Promise<any>;
    };
    os: {
      getUserId(): Promise<string>;
    };
    kitFolder: {
      selectFolder(): Promise<string | null>;
      readKit(folderPath: string): Promise<ArrayBuffer | null>;
      writeKit(folderPath: string, data: ArrayBuffer): Promise<void>;
      readFile(folderPath: string, filePath: string): Promise<ArrayBuffer | null>;
      writeFile(folderPath: string, filePath: string, data: ArrayBuffer): Promise<void>;
      deleteFile(folderPath: string, filePath: string): Promise<void>;
      listFiles(folderPath: string): Promise<string[]>;
      getRecentFolders(): Promise<string[]>;
      addRecentFolder(folderPath: string): Promise<void>;
      watchFolder(folderPath: string, onChanged: () => void): () => void;
    };
    kitFile: {
      selectFile(): Promise<string | null>;
      readJson(filePath: string): Promise<string | null>;
      writeJson(filePath: string, json: string): Promise<void>;
    };
  }
}

/**
 * Invokes a window control action via the preload bridge.
 *MUST fall back gracefully when window controls are unavailable.
 **/
const invokeWindowControl = (action: "minimize" | "maximize" | "close") => {
  if (window.windowControls) {
    return window.windowControls[action]();
  }
  console.warn(`Window controls not available for action: ${action}`);
  return Promise.resolve();
};

/**
 * Desktop integration surface. Presence of this object is how sketchpad detects desktop mode.
 *MUST delegate to invokeWindowControl for each action.
 **/
const desktop = {
  minimize: () => invokeWindowControl("minimize"),
  maximize: () => invokeWindowControl("maximize"),
  close: () => invokeWindowControl("close"),
};

/**
 * OS bridge for retrieving the current user identity.
 *MUST use the preload-exposed getUserId API.
 **/
const os = {
  getUserId: async () => await window.os.getUserId(),
};

/**
 * Root React component rendering the sketchpad with folder kit store factory.
 *MUST render sketchpad directly with folder kit store factory for local kit persistence.
 **/

// 🔄#region 🗄️FolderAdapter
function createElectronFolderAdapter(folderPath: string): KitFolderAdapter {
  return {
    readKit: async () => {
      const data = await window.kitFolder.readKit(folderPath);
      return data ? new Uint8Array(data) : null;
    },
    writeKit: (data: Uint8Array) => window.kitFolder.writeKit(folderPath, data.buffer),
    readFile: async (path: string) => {
      const data = await window.kitFolder.readFile(folderPath, path);
      if (!data) return null;
      return new Blob([data]);
    },
    writeFile: async (path: string, blob: Blob) => {
      const buffer = await blob.arrayBuffer();
      await window.kitFolder.writeFile(folderPath, path, buffer);
    },
    deleteFile: (path: string) => window.kitFolder.deleteFile(folderPath, path),
    listFiles: () => window.kitFolder.listFiles(folderPath),
    watch: (callback: () => void) => window.kitFolder.watchFolder(folderPath, callback),
  };
}
// #endregion 🗄️FolderAdapter

// #region 🖲️App
// Root app renders the Sketchpad directly with folder kit store factory for local kit persistence.
// No welcome/start page — the home app toolbar provides open/create actions.

function App() {
  const [userId, setUserId] = useState<string>("");

  useEffect(() => {
    async function fetchUserId() {
      try {
        const id = await os.getUserId();
        setUserId(id);
      } catch (error) {
        console.error("Failed to get user ID:", error);
        setUserId("anonymous-user");
      }
    }

    fetchUserId();
  }, []);

  // 🏭Folder kit store factory for creating/opening local kits via Electron IPC.
  const folderKitStoreFactory: SketchpadKitStoreFactory = useCallback(async (kit) => {
    const e2eFolder = typeof window !== "undefined" ? window.__SEMIO_E2E_KIT_FOLDER__ : undefined;
    const source = (kit as any)?.__semioKitPersistenceSource as { kind?: string; path?: string } | undefined;
    const requestedFolder = source?.kind === "folder" && source.path ? source.path : undefined;
    const selectedFolder = requestedFolder ?? (e2eFolder && e2eFolder.length > 0 ? e2eFolder : await window.kitFolder.selectFolder());
    if (!selectedFolder) {
      throw new Error("No folder selected for kit storage");
    }
    await window.kitFolder.addRecentFolder(selectedFolder);
    const adapter = createElectronFolderAdapter(selectedFolder);
    const store = await createFolderKitStore(adapter);
    (store as any).__semioKitPersistenceSource = { kind: "folder", path: selectedFolder };
    return store;
  }, []);

  // 🏭Temporary kit store factory for in-memory kits.
  const temporaryKitStoreFactory: SketchpadKitStoreFactory = useCallback((kit) => new InMemoryKitStore(kit), []);

  // 🏭File kit store factory for opening JSON kit files via native file dialog.
  // Specs: In Electron, uses dialog.showOpenDialog via IPC for native file picker.
  // Falls back to File System Access API only when the preload bridge is unavailable.
  const fileKitStoreFactory: SketchpadKitStoreFactory = useCallback(async (_kit) => {
    const e2eFile = typeof window !== "undefined" ? window.__SEMIO_E2E_KIT_FILE__ : undefined;
    const source = (_kit as any)?.__semioKitPersistenceSource as { kind?: string; path?: string } | undefined;
    if (window.kitFile) {
      const selectedFile = source?.kind === "file" && source.path ? source.path : e2eFile && e2eFile.length > 0 ? e2eFile : await window.kitFile.selectFile();
      if (!selectedFile) {
        throw new Error("No file selected for kit storage");
      }
      const adapter: KitJsonFileAdapter = {
        read: async () => await window.kitFile.readJson(selectedFile),
        write: async (json: string) => await window.kitFile.writeJson(selectedFile, json),
      };
      const store = await createJsonFileKitStore(adapter);
      (store as any).__semioKitPersistenceSource = { kind: "file", path: selectedFile };
      return store;
    }
    if (typeof window !== "undefined" && "showOpenFilePicker" in window) {
      const [fileHandle] = await (window as any).showOpenFilePicker({
        types: [{ description: "Semio Kit JSON", accept: { "application/json": [".json"] } }],
      });
      const adapter: KitJsonFileAdapter = {
        read: async () => {
          const file = await fileHandle.getFile();
          return file.text();
        },
        write: async (json: string) => {
          const writable = await fileHandle.createWritable();
          await writable.write(json);
          await writable.close();
        },
      };
      return createJsonFileKitStore(adapter);
    }
    throw new Error("File kit store not available in this environment");
  }, []);

  // 🏭Remote kit store factory for connecting to semio/server.
  // Specs: The server URL is passed in kit.name by the openKit command.
  const remoteKitStoreFactory: SketchpadKitStoreFactory = useCallback(async (kit) => {
    const source = (kit as any)?.__semioKitPersistenceSource as { kind?: string; url?: string } | undefined;
    const serverUrl = source?.kind === "remote" && source.url ? source.url : kit.name;
    if (!serverUrl) throw new Error("No server URL provided for remote kit");
    const store = await createSessionKitStore({ serverUrl });
    (store as any).__semioKitPersistenceSource = { kind: "remote", url: serverUrl };
    return store;
  }, []);

  if (!userId) {
    return <div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading...</div>;
  }

  return (
    <div className="h-screen w-screen">
      <Suspense fallback={<div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading sketchpad...</div>}>
        {/* Specs: desktop skips browser kit snapshot persistence (folder/file/remote); reopen kits via Open each session. */}
        <LazySketchpad
          desktop={desktop}
          id={userId}
          folderKitStoreFactory={folderKitStoreFactory}
          fileKitStoreFactory={fileKitStoreFactory}
          temporaryKitStoreFactory={temporaryKitStoreFactory}
          remoteKitStoreFactory={remoteKitStoreFactory}
        />
      </Suspense>
    </div>
  );
}
// #endregion 🖲️App

export default App;

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Renderer root element '#root' is missing.");
}
const SEMIO_REACT_ROOT = Symbol.for("semio.desktop.reactRoot");
type RootHost = HTMLElement & { [SEMIO_REACT_ROOT]?: Root };
const host = rootElement as RootHost;
const reactRoot = host[SEMIO_REACT_ROOT] ?? createRoot(rootElement);
host[SEMIO_REACT_ROOT] = reactRoot;
reactRoot.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// #endregion ⛩️Renderer
