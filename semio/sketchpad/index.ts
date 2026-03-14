// #region 🔖Header
// [👤semio🖱️sketchpad💻index](semiorepo://p/u/semio/b/u/sketchpad/f/index.ts)

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

// Public bundle exports for the semio sketchpad runtime and app configs.

// #endregion 🔖Header

// #region 🔖Exports
// [👤semio🖱️sketchpad💻index🔖exports](semiorepo://p/u/semio/b/u/sketchpad/f/index.ts/s/Exports)
// Public API surface for sketchpad runtime, shared helpers, and app configs.
// Exports MUST keep the sketchpad UI surface out of @semio/js.

import "./i18n";

export type { AppConfig, CompositeFileProviderConfig, FileProvider, FileProviderFactory, LocalFileProviderConfig, MemoryFileProviderConfig, RemoteFileProviderConfig, YProviderFactory } from "./sketchpad/shared";
export { default as Sketchpad, appRegistry, createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider, loadAppConfigs } from "./sketchpad/Sketchpad";
export { Canvas, HorizontalWindows, VerticalWindows } from "../../semio-elements/ui";

export { config as designConfig } from "./sketchpad/Design";
export { config as docsConfig } from "./sketchpad/Docs";
export { config as feedbackConfig } from "./sketchpad/Feedback";
export { config as homeConfig } from "./sketchpad/Home";
export { config as kitConfig } from "./sketchpad/Kit";
export { config as qualityConfig } from "./sketchpad/Quality";
export { config as typeConfig } from "./sketchpad/Type";

// #endregion 🔖Exports
