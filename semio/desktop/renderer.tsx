// #region 🔖Header
// [👤semio🖱️desktop💻renderer](repo://p/u/semio/b/u/desktop/f/renderer.tsx)

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

// Entry point for the Electron renderer process mounting the React app.

// #endregion 🔖Header

// #region 🔖Renderer
// [👤semio🖱️desktop💻renderer🔖renderer](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer)
// Electron renderer process that mounts the Sketchpad React app with window controls.
// MUST resolve the user identity before rendering the sketchpad.

import React, { useEffect, useState, lazy, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { createFolderKitStore } from "@semio/studio";
import type { KitFolderAdapter } from "@semio/studio";
import type { KitStore } from "@semio/js/semio";

import "./globals.css";

// Lazy-load the heavy sketchpad module (500KB+) to avoid blocking the renderer.
const LazySketchpad = lazy(() => import("@semio/sketchpad").then((mod) => ({ default: mod.Sketchpad })));

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
      readKit(folderPath: string): Promise<string | null>;
      writeKit(folderPath: string, json: string): Promise<void>;
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
 * Root React component that shows a start page or the sketchpad.
// [👤semio🖱️desktop💻renderer🔖renderer🛠️app](repo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/App)
 *MUST show folder selection start page when no folder is open.
 **/

// #region 🔖FolderAdapter
function createElectronFolderAdapter(folderPath: string): KitFolderAdapter {
  return {
    readKit: () => window.kitFolder.readKit(folderPath),
    writeKit: (json: string) => window.kitFolder.writeKit(folderPath, json),
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

// #region 🔖StartPage
function StartPage({ onFolderSelected }: { onFolderSelected: (path: string) => void }) {
  const [recentFolders, setRecentFolders] = useState<string[]>([]);

  useEffect(() => {
    window.kitFolder.getRecentFolders().then(setRecentFolders);
  }, []);

  const handleOpenFolder = async () => {
    const folder = await window.kitFolder.selectFolder();
    if (folder) {
      await window.kitFolder.addRecentFolder(folder);
      onFolderSelected(folder);
    }
  };

  const handleCreateKit = async () => {
    const folder = await window.kitFolder.selectFolder();
    if (folder) {
      await window.kitFolder.addRecentFolder(folder);
      onFolderSelected(folder);
    }
  };

  return (
    <div className="flex h-full w-full flex-col items-center justify-center gap-8 bg-neutral-950 text-white">
      <div className="app-region-drag w-full h-8" />
      <h1 className="text-3xl font-bold">semio</h1>
      <p className="text-neutral-400">Select a folder to open or create a new local kit.</p>
      <div className="flex gap-4">
        <button onClick={handleOpenFolder} className="rounded-lg bg-blue-600 px-6 py-3 font-medium hover:bg-blue-700 transition-colors">
          Open Folder
        </button>
        <button onClick={handleCreateKit} className="rounded-lg border border-neutral-600 px-6 py-3 font-medium hover:bg-neutral-800 transition-colors">
          Create New Kit
        </button>
      </div>
      {recentFolders.length > 0 && (
        <div className="mt-4 w-80">
          <h2 className="mb-2 text-sm font-medium text-neutral-400">Recent</h2>
          <div className="flex flex-col gap-1">
            {recentFolders.map((folder) => (
              <button
                key={folder}
                onClick={() => {
                  window.kitFolder.addRecentFolder(folder);
                  onFolderSelected(folder);
                }}
                className="w-full rounded px-3 py-2 text-left text-sm hover:bg-neutral-800 transition-colors truncate"
                title={folder}
              >
                {folder.split(/[/\\]/).pop()} <span className="text-neutral-500 text-xs">{folder}</span>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
// #endregion 🔖StartPage

function App() {
  const [userId, setUserId] = useState<string>("");
  const [folderPath, setFolderPath] = useState<string | null>(null);
  const [kitStore, setKitStore] = useState<KitStore | undefined>(undefined);
  const [loading, setLoading] = useState(false);

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

  useEffect(() => {
    if (!folderPath) {
      setKitStore(undefined);
      return;
    }
    let disposed = false;
    setLoading(true);
    const adapter = createElectronFolderAdapter(folderPath);
    createFolderKitStore(adapter).then((store) => {
      if (disposed) {
        store.dispose();
        return;
      }
      setKitStore(store);
      setLoading(false);
    });
    return () => {
      disposed = true;
      setKitStore((prev) => {
        if (prev) prev.dispose();
        return undefined;
      });
    };
  }, [folderPath]);

  if (!folderPath) {
    return <StartPage onFolderSelected={setFolderPath} />;
  }

  if (loading || !userId || !kitStore) {
    return <div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading...</div>;
  }

  return (
    <div className="h-screen w-screen">
      <Suspense fallback={<div className="flex h-full w-full items-center justify-center bg-neutral-950 text-white">Loading sketchpad...</div>}>
        <LazySketchpad onWindowEvents={windowEvents} id={userId} kitStore={kitStore} />
      </Suspense>
    </div>
  );
}

export default App;

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// #endregion 🔖Renderer
