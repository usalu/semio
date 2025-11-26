// #region Header

// index.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import "./i18n";

export type { TooltipConfig } from "./sketchpad/elements";
export type { CompositeFileProviderConfig, FileProvider, FileProviderFactory, LocalFileProviderConfig, MemoryFileProviderConfig, RemoteFileProviderConfig, YProviderFactory } from "./sketchpad/shared";
export { createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider, default as Sketchpad } from "./sketchpad/Sketchpad";

export { areKitsEqual, exportKit, importKit } from "./semio";
export type { KitImportResult } from "./semio";

// Export docs elements for MDX
export { Action, ActionDropdown, ActionGroup, ActionGroupItem, Aside, Avatar, AvatarFallback, AvatarImage, Card, CardGrid, DraggableAvatar, FileTree, Section, Steps, TableAvatar, Tabs, TabsContent, TabsList, TabsTrigger } from "./sketchpad/elements";
export type { ActionDropdownOption, ActionDropdownProps, ActionProps, AsideProps, CardGridProps, CardProps, DraggableAvatarProps, FileTreeNode, SectionProps, StepsProps, TableAvatarProps } from "./sketchpad/elements";

export { default as eslintConfig } from "./eslint.config";
export { default as postcssConfig } from "./postcss.config";
export { default as tailwindConfig } from "./tailwind.config";
// Exporting vite configs blows up storybook and nextjs
// export { default as viteConfig } from './vite.config';
// export { default as vitestConfig } from './vitest.workspace';
