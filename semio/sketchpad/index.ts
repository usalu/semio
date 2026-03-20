// #region 🔖Header
// [👤semio🖱️sketchpad💻index](repo://p/u/semio/b/u/sketchpad/f/index.ts)

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
// [👤semio🖱️sketchpad💻index🔖exports](repo://p/u/semio/b/u/sketchpad/f/index.ts/s/Exports)
// Public API surface for sketchpad runtime, shared helpers, and app configs.
// Exports MUST keep the sketchpad UI surface out of @semio/js.

import "./i18n";

export type { BlobAssetStore, KitStore, KitStoreSnapshot, KitStoreStatus, KitSyncState, ObservablePathStore, UndoableKitStore } from "@semio/js/semio";
export { JsonFileKitStore, createJsonFileKitStore } from "@semio/studio";
export type { KitJsonFileAdapter } from "@semio/studio";
export { Canvas, HorizontalWindows, VerticalWindows } from "@semio/ui";
export { SyncBinaryPersistenceProvider, createJsonFilePersistenceFactory, createSqliteFolderPersistenceFactory } from "./sketchpad/shared";
export type {
  AppConfig,
  CompositeFileProviderConfig,
  FileProvider,
  FileProviderFactory,
  JsonFileAdapter,
  LocalFileProviderConfig,
  MemoryFileProviderConfig,
  PersistenceFactory,
  PersistenceProvider,
  RemoteFileProviderConfig,
  RemoteProviders,
  SqliteAdapter,
  SyncProviderFactory,
} from "./sketchpad/shared";
export { default as Sketchpad, appRegistry, createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider, loadAppConfigs } from "./sketchpad/Sketchpad";
export type { SketchpadKitKindAvailability, SketchpadKitStoreFactory, SketchpadStore } from "./sketchpad/Sketchpad";

export { config as designConfig } from "./sketchpad/Design";
export { config as docsConfig } from "./sketchpad/Docs";
export { config as feedbackConfig } from "./sketchpad/Feedback";
export { config as homeConfig } from "./sketchpad/Home";
export { config as kitConfig } from "./sketchpad/Kit";
export { config as qualityConfig } from "./sketchpad/Quality";
export { config as typeConfig } from "./sketchpad/Type";

// #endregion 🔖Exports
