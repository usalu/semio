// #region 🔖Header

// [👤semio📚js💻indexts](semiorepo://file/SEMIO/JS/INDEX.TS)

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

// Barrel export for the core JavaScript workspace modules.

// #endregion 🔖Header

// #region 🔖Exports

// [👤semio📚js💻indexts🔖exports](semiorepo://section/SEMIO/JS/INDEX.TS/EXPORTS)
// Public API surface re-exporting sketchpad components, semio domain, and shared configs.
// MUST re-export all public types alongside their runtime counterparts.

import "./i18n";
import "./sketchpad/Home";
import "./sketchpad/Kit";
import "./sketchpad/Design";
import "./sketchpad/Type";
import "./sketchpad/Quality";
import "./sketchpad/Docs";
import "./sketchpad/Feedback";

export type { TooltipConfig } from "./sketchpad/elements";
export type { AppConfig, CompositeFileProviderConfig, FileProvider, FileProviderFactory, LocalFileProviderConfig, MemoryFileProviderConfig, RemoteFileProviderConfig, YProviderFactory } from "./sketchpad/shared";
export { default as Sketchpad, appRegistry, createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider, loadAppConfigs } from "./sketchpad/Sketchpad";

export { config as designConfig } from "./sketchpad/Design";
export { config as docsConfig } from "./sketchpad/Docs";
export { config as feedbackConfig } from "./sketchpad/Feedback";
export { config as homeConfig } from "./sketchpad/Home";
export { config as kitConfig } from "./sketchpad/Kit";
export { config as qualityConfig } from "./sketchpad/Quality";
export { config as typeConfig } from "./sketchpad/Type";

export { areKitsEqual, exportKit, importKit } from "./semio";
export type { KitImportResult } from "./semio";

export { Action, ActionDropdown, ActionGroup, ActionGroupItem, Aside, Avatar, AvatarFallback, AvatarImage, Card, CardGrid, DraggableAvatar, FileTree, Section, Steps, TableAvatar, Tabs, TabsContent, TabsList, TabsTrigger } from "./sketchpad/elements";
export type { ActionDropdownOption, ActionDropdownProps, ActionProps, AsideProps, CardGridProps, CardProps, DraggableAvatarProps, FileTreeNode, SectionProps, StepsProps, TableAvatarProps } from "./sketchpad/elements";

export { default as eslintConfig } from "./eslint.config";
export { default as postcssConfig } from "./postcss.config";
export { default as tailwindConfig } from "./tailwind.config";

// #endregion 🔖Exports
