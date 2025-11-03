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

import { Expertise, Layout, migratePath, Mode, SketchpadCommandContext, SketchpadCommandResult, Theme } from "./store";

export const commands = {
  "semio.sketchpad.setTheme": (context: SketchpadCommandContext, theme: Theme): SketchpadCommandResult => {
    return {
      diff: { theme },
    };
  },
  "semio.sketchpad.setLayout": (context: SketchpadCommandContext, layout: Layout): SketchpadCommandResult => {
    return {
      diff: { layout },
    };
  },
  "semio.sketchpad.setExpertise": (context: SketchpadCommandContext, expertise: Expertise): SketchpadCommandResult => {
    return {
      diff: { expertise },
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
    const layout = context.sketchpad.layout;
    if (typeof layout === "object") {
      return {
        diff: { layout: { ...layout, isNavbarExpanded: !layout.isNavbarExpanded } },
      };
    }
    return {};
  },
  "semio.sketchpad.toggleFooterExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const layout = context.sketchpad.layout;
    if (typeof layout === "object") {
      return {
        diff: { layout: { ...layout, isFooterExpanded: !layout.isFooterExpanded } },
      };
    }
    return {};
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
  "semio.sketchpad.setHotkey": (context: SketchpadCommandContext, path: string, value: string): SketchpadCommandResult => {
    const overrides = { ...context.sketchpad.hotkeyOverrides };
    overrides[path] = value;
    return {
      diff: { hotkeyOverrides: overrides },
    };
  },
  "semio.sketchpad.resetHotkey": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    const overrides = { ...context.sketchpad.hotkeyOverrides };
    delete overrides[path];
    return {
      diff: { hotkeyOverrides: overrides },
    };
  },
  "semio.sketchpad.resetAllHotkeys": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {
      diff: { hotkeyOverrides: {} },
    };
  },
  "semio.sketchpad.navigateToHotkeySetting": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    return {
      diff: {
        navigation: "/",
        activeHotkeySetting: path,
      },
    };
  },
};

export const devCommands = {
  "semio.sketchpad.freeze": (context: SketchpadCommandContext): SketchpadCommandResult => {
    // This command needs access to the store, which will be passed via special handling
    return {};
  },
  "semio.sketchpad.timetravel": (context: SketchpadCommandContext): SketchpadCommandResult => {
    // This command needs access to the store, which will be passed via special handling
    return {};
  },
};
