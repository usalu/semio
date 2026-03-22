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

export type { BlobAssetStore, KitStore, KitStoreSnapshot, KitStoreStatus, KitSyncState, ObservablePathStore, UndoableKitStore } from "@semio/js";
export { JsonFileKitStore, createJsonFileKitStore } from "@semio/studio";
export type { KitJsonFileAdapter } from "@semio/studio";
export { Canvas, HorizontalWindows, VerticalWindows } from "@semio/ui";
export {
  default as Sketchpad,
  SyncBinaryPersistenceProvider,
  appRegistry,
  createCompositeFileProvider,
  createJsonFilePersistenceFactory,
  createLocalFileProvider,
  createMemoryFileProvider,
  createRemoteFileProvider,
  createSqliteFolderPersistenceFactory,
  loadAppConfigs,
  designConfig,
  docsConfig,
  feedbackConfig,
  homeConfig,
  kitConfig,
  qualityConfig,
  typeConfig,
} from "./Sketchpad";
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
  SketchpadKitKindAvailability,
  SketchpadKitStoreFactory,
  SketchpadStore,
  SqliteAdapter,
  SyncProviderFactory,
} from "./Sketchpad";

// #endregion 🔖Exports
