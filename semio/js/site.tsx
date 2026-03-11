// #region 🔖Header
// [👤semio📚js💻site](semiorepo://p/u/semio/b/l/js/f/site.tsx)

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
// [👤semio📚js💻site🔖entrypoint](semiorepo://p/u/semio/b/l/js/f/site.tsx/s/Entrypoint)
// Site entrypoint that mounts the Sketchpad React component into the DOM.
// Entrypoint MUST render into the root element defined in index.html.

import { createRoot } from "react-dom/client";
import Sketchpad, { appRegistry } from "./sketchpad/Sketchpad";
import "./globals.css";
import { config as designConfig } from "./sketchpad/Design";
import { config as docsConfig } from "./sketchpad/Docs";
import { config as feedbackConfig } from "./sketchpad/Feedback";
import { config as homeConfig } from "./sketchpad/Home";
import { config as kitConfig } from "./sketchpad/Kit";
import { config as qualityConfig } from "./sketchpad/Quality";
import { config as typeConfig } from "./sketchpad/Type";

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad />
  </div>,
);

// #endregion 🔖Entrypoint
