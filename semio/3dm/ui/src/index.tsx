// #region 🔖Header
// [👤semio📚3dm🖱️ui🗃️src💻index](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/index.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

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

// Entry point for the semio 3dm React UI embedded in Rhino WebView2.

// #endregion 🔖Header

// #region 🔖Entrypoint
// [👤semio📚3dm🖱️ui🗃️src💻index🔖entrypoint](repo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/index.tsx/s/Entrypoint)
// Entrypoint MUST initialize the bridge and render the RhinoPanel component.

import { createRoot } from "react-dom/client";
import "../globals.css";
import { initBridge } from "./bridge";
import { RhinoPanel } from "./RhinoPanel";

initBridge();

createRoot(document.getElementById("root")!).render(<RhinoPanel />);

// #endregion 🔖Entrypoint
