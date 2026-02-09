// #region 🔖Header

// 💻 semio/sketchpad/index.tsx

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

import { createRoot } from "react-dom/client";
import { Sketchpad } from "@semio/js";
import "./globals.css";

import { appRegistry } from "../semio/sketchpad/Sketchpad";

import { config as designConfig } from "../semio/sketchpad/Design";
import { config as docsConfig } from "../semio/sketchpad/Docs";
import { config as homeConfig } from "../semio/sketchpad/Home";
import { config as kitConfig } from "../semio/sketchpad/Kit";
import { config as qualityConfig } from "../semio/sketchpad/Quality";
import { config as typeConfig } from "../semio/sketchpad/Type";

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad importKitUrls={["/metabolism.zip"]} />
  </div>,
);
