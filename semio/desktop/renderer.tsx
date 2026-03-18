// #region 🔖Header
// [👤semio🖱️desktop💻renderer](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx)

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
// [👤semio🖱️desktop💻renderer🔖renderer](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer)
// Electron renderer process that mounts the Sketchpad React app with window controls.
// MUST resolve the user identity before rendering the sketchpad.

import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { createIndexeddbPersistenceFactory } from "@semio/studio";

import "./globals.css";

import { Sketchpad } from "@semio/sketchpad";

const indexeddbPersistenceFactory = createIndexeddbPersistenceFactory();

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
  }
}

/**
 * Invokes a window control action via the preload bridge.
// [👤semio🖱️desktop💻renderer🔖renderer🛠️invokewindowcontrol](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/invokeWindowControl)
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
// [👤semio🖱️desktop💻renderer🔖renderer🪨windowevents](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/windowEvents)
 *MUST delegate to invokeWindowControl for each action.
 **/
const windowEvents = {
  minimize: () => invokeWindowControl("minimize"),
  maximize: () => invokeWindowControl("maximize"),
  close: () => invokeWindowControl("close"),
};

/**
 * OS bridge for retrieving the current user identity.
// [👤semio🖱️desktop💻renderer🔖renderer🪨os](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/os)
 *MUST use the preload-exposed getUserId API.
 **/
const os = {
  getUserId: async () => await window.os.getUserId(),
};

/**
 * Root React component that loads the user identity and renders the sketchpad.
// [👤semio🖱️desktop💻renderer🔖renderer🛠️app](semiorepo://p/u/semio/b/u/desktop/f/renderer.tsx/s/Renderer/d/i/App)
 *MUST show a loading state until the user ID is resolved.
 **/
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

  return (
    <div className="h-screen w-screen">
      {userId ? <Sketchpad onWindowEvents={windowEvents} id={userId} persistenceFactory={indexeddbPersistenceFactory} /> : <div className="flex h-full w-full items-center justify-center">Loading user data...</div>}
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
