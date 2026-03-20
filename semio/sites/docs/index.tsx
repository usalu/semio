// #region 🔖Header
// [👤semio🌐docs💻index](repo://p/u/semio/b/w/docs/f/index.tsx)

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

// Entry point for the documentation site React app.

// #endregion 🔖Header

// #region 🔖Entrypoint
// [👤semio🌐docs💻index🔖entrypoint](repo://p/u/semio/b/w/docs/f/index.tsx/s/Entrypoint)
// Docs entrypoint that mounts the Sketchpad React component with StrictMode.
// Entrypoint MUST render into the root element defined in the docs index.html.

import React from "react";
import { createRoot } from "react-dom/client";
import { Sketchpad } from "@semio/sketchpad";
import "./globals.css";

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <div className="h-screen w-screen">
      <Sketchpad />
    </div>
  </React.StrictMode>,
);

// #endregion 🔖Entrypoint
