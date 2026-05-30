// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Electron renderer: mounts sketchpad via generic {@link mountPlatform}.

// #endregion 🧲Header

// #region ⛩️Renderer

// #region 🔌Adapters
import { mountPlatform } from "@framework/platform/renderer/react";
import { ensureSketchpadPlatform } from "@semio/sketchpad";
import { createRoot } from "react-dom/client";
// #endregion 🔌Adapters

import "./globals.css";

declare global {
  interface Window {
    windowControls: {
      minimize(): Promise<unknown>;
      maximize(): Promise<unknown>;
      close(): Promise<unknown>;
    };
    os: {
      getUserId(): Promise<string>;
    };
  }
}

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Renderer root element '#root' is missing.");
}

createRoot(rootElement).render(
  <div className="flex h-screen w-screen items-center justify-center bg-neutral-950 text-white">Loading sketchpad…</div>,
);

void (async () => {
  try {
    await window.os.getUserId();
  } catch (error) {
    console.error("Failed to get user ID:", error);
  }
  try {
    await mountPlatform(ensureSketchpadPlatform);
  } catch (error) {
    console.error("[semio.desktop] sketchpad mount failed:", error);
  }
})();

// #endregion ⛩️Renderer
