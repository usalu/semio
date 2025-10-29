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
import i18n from "./i18n";

export type { TooltipConfig } from "./elements/display/Tooltip";
export { createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider } from "./sketchpad/fileProviders";
export type { CompositeFileProviderConfig, LocalFileProviderConfig, MemoryFileProviderConfig, RemoteFileProviderConfig } from "./sketchpad/fileProviders";
export { default as Sketchpad } from "./sketchpad/Sketchpad";
export type { FileProvider, FileProviderFactory, YProviderFactory } from "./sketchpad/store";
export { i18n };

// Export docs elements for MDX
export { FileTree } from "./elements/aggregation/FileTree";
export type { FileTreeNode, FileTreeProps } from "./elements/aggregation/FileTree";

export { Tabs, TabsContent, TabsList, TabsTrigger } from "./elements/aggregation/Tabs";
export { Aside } from "./elements/display/Aside";
export type { AsideProps } from "./elements/display/Aside";
export { Card, CardGrid } from "./elements/display/Card";
export type { CardGridProps, CardProps } from "./elements/display/Card";
export { default as Section } from "./elements/display/Section";
export type { SectionProps } from "./elements/display/Section";
export { Steps } from "./elements/display/Steps";
export type { StepsProps } from "./elements/display/Steps";

export { default as eslintConfig } from "./eslint.config";
export { default as postcssConfig } from "./postcss.config";
export { default as tailwindConfig } from "./tailwind.config";
// Exporting vite configs blows up storybook and nextjs
// export { default as viteConfig } from './vite.config';
// export { default as vitestConfig } from './vitest.workspace';
