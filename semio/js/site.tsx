// #region 🔖Header

// [👤semio📚js💻sitetsx](semiorepo://file/semio/js/site.tsx)

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

// Landing page and marketing site React component.

// #endregion 🔖Header

// #region 🔖Entrypoint

// [👤semio📚js💻sitetsx🔖entrypoint](semiorepo://section/semio/js/site.tsx/Entrypoint)
// Site entrypoint that mounts the Sketchpad React component into the DOM.
// Entrypoint MUST render into the root element defined in index.html.

import { createRoot } from "react-dom/client";
import { Sketchpad } from "@semio/js";
import "./globals.css";

createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad />
  </div>,
);

// #endregion 🔖Entrypoint
