// #region 🔖Header

// 💻semio/desktop/renderer.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import "./globals.css";

import { Sketchpad } from "@semio/js";

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

const invokeWindowControl = (action: "minimize" | "maximize" | "close") => {
  if (window.windowControls) {
    return window.windowControls[action]();
  }
  console.warn(`Window controls not available for action: ${action}`);
  return Promise.resolve();
};

const windowEvents = {
  minimize: () => invokeWindowControl("minimize"),
  maximize: () => invokeWindowControl("maximize"),
  close: () => invokeWindowControl("close"),
};

const os = {
  getUserId: async () => await window.os.getUserId(),
};

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

  return <div className="h-screen w-screen">{userId ? <Sketchpad onWindowEvents={windowEvents} id={userId} /> : <div className="flex h-full w-full items-center justify-center">Loading user data...</div>}</div>;
}

export default App;

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
