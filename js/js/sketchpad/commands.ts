// #region Header

// commands.ts

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

import { Access, Layout, migratePath, Mode, SketchpadCommandContext, SketchpadCommandResult, Theme } from "./store";

export const commands = {
    "semio.sketchpad.setTheme": (context: SketchpadCommandContext, theme: Theme): SketchpadCommandResult => {
        return {
            diff: { theme },
        };
    },
    "semio.sketchpad.setAccess": (context: SketchpadCommandContext, access: Access): SketchpadCommandResult => {
        return {
            diff: { access },
        };
    },
    "semio.sketchpad.setLayout": (context: SketchpadCommandContext, layout: Layout): SketchpadCommandResult => {
        return {
            diff: { layout },
        };
    },
    "semio.sketchpad.setMode": (context: SketchpadCommandContext, mode: Mode): SketchpadCommandResult => {
        return {
            diff: { mode },
        };
    },
    "semio.sketchpad.toggleFullscreen": (context: SketchpadCommandContext): SketchpadCommandResult => {
        return {
            diff: { isFullscreen: !context.sketchpad.isFullscreen },
        };
    },
    "semio.sketchpad.toggleNavbarExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
        return {
            diff: { isNavbarExpanded: !context.sketchpad.isNavbarExpanded },
        };
    },
    "semio.sketchpad.setIsMobile": (context: SketchpadCommandContext, isMobile: boolean): SketchpadCommandResult => {
        if (context.sketchpad.isMobile !== isMobile) {
            return {
                diff: { isMobile },
            };
        }
        return {};
    },
    "semio.sketchpad.setActiveInteraction": (context: SketchpadCommandContext, interactionId?: string): SketchpadCommandResult => {
        return {
            diff: { activeInteraction: interactionId },
        };
    },
    "semio.sketchpad.syncNavigation": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
        const migratedPath = migratePath(path);
        if (context.sketchpad.navigation !== migratedPath) {
            return {
                diff: { navigation: migratedPath },
            };
        }
        return {};
    },
    "semio.sketchpad.navigateBack": (context: SketchpadCommandContext): SketchpadCommandResult => {
        const { navigationHistory, navigationHistoryIndex } = context.sketchpad;
        if (navigationHistoryIndex > 0) {
            return {
                diff: {
                    navigationHistoryIndex: navigationHistoryIndex - 1,
                },
            };
        }
        return {};
    },
    "semio.sketchpad.navigateForward": (context: SketchpadCommandContext): SketchpadCommandResult => {
        const { navigationHistory, navigationHistoryIndex } = context.sketchpad;
        if (navigationHistoryIndex < navigationHistory.length - 1) {
            return {
                diff: {
                    navigationHistoryIndex: navigationHistoryIndex + 1,
                },
            };
        }
        return {};
    },
};
