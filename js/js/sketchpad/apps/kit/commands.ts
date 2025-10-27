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

import { Design, DesignDiff, Guid, Type, TypeDiff } from "../../../semio";
import { Access, Layout, Theme } from "../../store";
import { KitAppCommandContext, KitAppCommandResult, KitAppFullscreenWindow, KitAppSortColumn, KitAppSortDirection } from "./store";

export const commands = {
  "semio.kitApp.setAccess": (context: KitAppCommandContext, access: Access): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.setTheme": (context: KitAppCommandContext, theme: Theme): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.setLayout": (context: KitAppCommandContext, layout: Layout): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.toggleTypesFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Types ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Types;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitApp.toggleDesignsFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Designs ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Designs;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitApp.selectAll": (context: KitAppCommandContext): KitAppCommandResult => {
    const kit = context.kit;
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: kit.types?.map((t) => t.guid) ?? [],
          },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: kit.designs?.map((d) => d.guid) ?? [],
          },
        },
      },
    };
  },
  "semio.kitApp.deselectAll": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectType": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: [Guid],
          },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectTypes": (context: KitAppCommandContext, typeIds: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: typeIds,
          },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addTypeToSelection": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          types: { added: [Guid] },
        },
      },
    };
  },
  "semio.kitApp.removeTypeFromSelection": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          types: { removed: [Guid] },
        },
      },
    };
  },
  "semio.kitApp.selectDesign": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: [Guid],
          },
        },
      },
    };
  },
  "semio.kitApp.selectDesigns": (context: KitAppCommandContext, designIds: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: designIds,
          },
        },
      },
    };
  },
  "semio.kitApp.addDesignToSelection": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          designs: { added: [Guid] },
        },
      },
    };
  },
  "semio.kitApp.removeDesignFromSelection": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          designs: { removed: [Guid] },
        },
      },
    };
  },
  "semio.kitApp.selectQuality": (context: KitAppCommandContext, key: string): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: {
            removed: currentSelection?.qualities ?? [],
            added: [key],
          },
          files: { removed: currentSelection?.files ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectQualities": (context: KitAppCommandContext, keys: string[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: {
            removed: currentSelection?.qualities ?? [],
            added: keys,
          },
          files: { removed: currentSelection?.files ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addQualityToSelection": (context: KitAppCommandContext, key: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          qualities: { added: [key] },
        },
      },
    };
  },
  "semio.kitApp.removeQualityFromSelection": (context: KitAppCommandContext, key: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          qualities: { removed: [key] },
        },
      },
    };
  },
  "semio.kitApp.selectFile": (context: KitAppCommandContext, path: string): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          files: {
            removed: currentSelection?.files ?? [],
            added: [path],
          },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectFiles": (context: KitAppCommandContext, paths: string[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          files: {
            removed: currentSelection?.files ?? [],
            added: paths,
          },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addFileToSelection": (context: KitAppCommandContext, path: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          files: { added: [path] },
        },
      },
    };
  },
  "semio.kitApp.removeFileFromSelection": (context: KitAppCommandContext, path: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          files: { removed: [path] },
        },
      },
    };
  },
  "semio.kitApp.selectAuthor": (context: KitAppCommandContext, name: string): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          authors: {
            removed: currentSelection?.authors ?? [],
            added: [name],
          },
        },
      },
    };
  },
  "semio.kitApp.selectAuthors": (context: KitAppCommandContext, names: string[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          authors: {
            removed: currentSelection?.authors ?? [],
            added: names,
          },
        },
      },
    };
  },
  "semio.kitApp.addAuthorToSelection": (context: KitAppCommandContext, name: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          authors: { added: [name] },
        },
      },
    };
  },
  "semio.kitApp.removeAuthorFromSelection": (context: KitAppCommandContext, name: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          authors: { removed: [name] },
        },
      },
    };
  },
  "semio.kitApp.deleteSelected": (context: KitAppCommandContext): KitAppCommandResult => {
    const selection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: selection?.types ?? [] },
          designs: { removed: selection?.designs ?? [] },
        },
      },
      kitDiff: {
        types: { removed: selection?.types },
        designs: { removed: selection?.designs },
      },
    };
  },
  "semio.kitApp.addType": (context: KitAppCommandContext, type: Type): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: [type] },
      },
    };
  },
  "semio.kitApp.addTypes": (context: KitAppCommandContext, types: Type[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: types },
      },
    };
  },
  "semio.kitApp.removeType": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: [Guid] },
      },
    };
  },
  "semio.kitApp.removeTypes": (context: KitAppCommandContext, typeIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: typeIds },
      },
    };
  },
  "semio.kitApp.addDesign": (context: KitAppCommandContext, design: Design): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: [design] },
      },
    };
  },
  "semio.kitApp.addDesigns": (context: KitAppCommandContext, designs: Design[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: designs },
      },
    };
  },
  "semio.kitApp.removeDesign": (context: KitAppCommandContext, Guid: Guid): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: [Guid] },
      },
    };
  },
  "semio.kitApp.removeDesigns": (context: KitAppCommandContext, designIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: designIds },
      },
    };
  },
  "semio.kitApp.updateType": (context: KitAppCommandContext, Guid: Guid, typeDiff: TypeDiff): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: [{ id: Guid, diff: typeDiff }] },
      },
    };
  },
  "semio.kitApp.updateTypes": (context: KitAppCommandContext, updates: { id: Guid; diff: TypeDiff }[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: updates },
      },
    };
  },
  "semio.kitApp.updateDesign": (context: KitAppCommandContext, Guid: Guid, designDiff: DesignDiff): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: [{ id: Guid, diff: designDiff }] },
      },
    };
  },
  "semio.kitApp.updateDesigns": (context: KitAppCommandContext, updates: { id: Guid; diff: DesignDiff }[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: updates },
      },
    };
  },
  "semio.kitApp.setFilterSearch": (context: KitAppCommandContext, search: string): KitAppCommandResult => {
    return {
      diff: {
        filterSearch: search,
      },
    };
  },
  "semio.kitApp.setExpandedRows": (context: KitAppCommandContext, rows: string[]): KitAppCommandResult => {
    return {
      diff: {
        expandedRows: rows,
      },
    };
  },
  "semio.kitApp.toggleExpandedRow": (context: KitAppCommandContext, rowId: string): KitAppCommandResult => {
    const currentRows = context.kitApp.expandedRows || [];
    const newRows = currentRows.includes(rowId) ? currentRows.filter((r) => r !== rowId) : [...currentRows, rowId];
    return {
      diff: {
        expandedRows: newRows,
      },
    };
  },
  "semio.kitApp.setSortColumn": (context: KitAppCommandContext, column: KitAppSortColumn): KitAppCommandResult => {
    return {
      diff: {
        sortColumn: column,
      },
    };
  },
  "semio.kitApp.setSortDirection": (context: KitAppCommandContext, direction: KitAppSortDirection): KitAppCommandResult => {
    return {
      diff: {
        sortDirection: direction,
      },
    };
  },
  "semio.kitApp.toggleSort": (context: KitAppCommandContext, column: KitAppSortColumn): KitAppCommandResult => {
    const current = context.kitApp;
    if (current.sortColumn === column) {
      return {
        diff: {
          sortDirection: current.sortDirection === "asc" ? "desc" : "asc",
        },
      };
    }
    return {
      diff: {
        sortColumn: column,
        sortDirection: "asc",
      },
    };
  },
};
