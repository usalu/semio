// #region 🔖Header

// [👤semio🌐play💻indextsx](semiorepo://file/SEMIO/PLAY/INDEX.TSX)

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

// Entry point for the playground React app for interactive experimentation.

// #endregion 🔖Header

// #region 🔖Entrypoint

// [🔖semio/play/index.tsx#Entrypoint](semiorepo://section/semio/play/index.tsx/ENTRYPOINT)
// Play application entrypoint registering sketchpad apps and rendering the root.
// Entrypoint MUST register all app configs before rendering the Sketchpad component.

import { createRoot } from "react-dom/client";
import { Sketchpad, appRegistry, designConfig, docsConfig, feedbackConfig, homeConfig, kitConfig, qualityConfig, typeConfig } from "@semio/js";
import "./globals.css";

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad importKitUrls={["/metabolism.zip"]} />
  </div>,
);
// #endregion 🔖Entrypoint
