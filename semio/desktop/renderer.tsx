// #region 🔖Header
// [👤semio🖱️desktop💻renderer](repo://p/u/semio/b/u/desktop/f/renderer.tsx)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the Electron renderer process mounting the React app.

// #endregion 🔖Header

// #region 🔖Renderer
// [👤semio🖱️desktop💻renderer🔖renderer](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer)
// Electron renderer process that mounts the Sketchpad React app with window controls.
// MUST resolve the user identity before rendering the sketchpad.

import React, { useEffect, useState, useCallback, lazy, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { createFolderKitStore } from "@semio/studio";
import type { KitFolderAdapter } from "@semio/studio";
import type { SketchpadKitStoreFactory } from "@semio/sketchpad";

import "./globals.css";

// Lazy-load the heavy sketchpad module (500KB+) to avoid blocking the renderer.
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
    };
  }
}

/**
 * Invokes a window control action via the preload bridge.
// [👤semio🖱️desktop💻renderer🔖renderer🛠️invokewindowcontrol](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/invokeWindowControl)
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
 * Window event handlers for minimize, maximize and close actions.
// [👤semio🖱️desktop💻renderer🔖renderer🪨windowevents](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/windowEvents)
 *MUST delegate to invokeWindowControl for each action.
 **/
const windowEvents = {
  minimize: () => invokeWindowControl("minimize"),
  maximize: () => invokeWindowControl("maximize"),
  close: () => invokeWindowControl("close"),
};

/**
 * OS bridge for retrieving the current user identity.
// [👤semio🖱️desktop💻renderer🔖renderer🪨os](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/os)
 *MUST use the preload-exposed getUserId API.
 **/
const os = {
  getUserId: async () => await window.os.getUserId(),
};

/**
 * Root React component rendering the sketchpad with folder kit store factory.
// [👤semio🖱️desktop💻renderer🔖renderer🛠️app](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/App)
 *MUST render sketchpad directly with folder kit store factory for local kit persistence.
 **/

// #region 🔖FolderAdapter
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
  };
}
// #endregion 🔖FolderAdapter

// #region 🔖App
// [👤semio🖱️desktop💻renderer🔖renderer🔖app](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/s/App)
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

  // Folder kit store factory for creating/opening local kits via Electron IPC.
  const folderKitStoreFactory: SketchpadKitStoreFactory = useCallback(async (kit) => {
    const selectedFolder = await window.kitFolder.selectFolder();
    if (!selectedFolder) {
      throw new Error("No folder selected for kit storage");
    }
    await window.kitFolder.addRecentFolder(selectedFolder);
    const adapter = createElectronFolderAdapter(selectedFolder);
    return createFolderKitStore(adapter);
  }, []);

  if (!userId) {
    return <div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading...</div>;
  }

  return (
    <div className="h-screen w-screen">
      <Suspense fallback={<div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading sketchpad...</div>}>
        <LazySketchpad onWindowEvents={windowEvents} id={userId} folderKitStoreFactory={folderKitStoreFactory} />
      </Suspense>
    </div>
  );
}
// #endregion 🔖App

export default App;

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// #endregion 🔖Renderer
