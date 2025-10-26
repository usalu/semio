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
import { KitEditorCommandContext, KitEditorCommandResult, KitEditorFullscreenWindow, KitEditorSortColumn, KitEditorSortDirection } from "./store";

export const commands = {
  "semio.kitEditor.setAccess": (context: KitEditorCommandContext, access: Access): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.setTheme": (context: KitEditorCommandContext, theme: Theme): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.setLayout": (context: KitEditorCommandContext, layout: Layout): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.toggleTypesFullscreen": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentPanel = context.kitEditor.fullscreenWindow;
    const newPanel = currentPanel === KitEditorFullscreenWindow.Types ? KitEditorFullscreenWindow.None : KitEditorFullscreenWindow.Types;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitEditor.toggleDesignsFullscreen": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentPanel = context.kitEditor.fullscreenWindow;
    const newPanel = currentPanel === KitEditorFullscreenWindow.Designs ? KitEditorFullscreenWindow.None : KitEditorFullscreenWindow.Designs;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitEditor.selectAll": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const kit = context.kit;
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.deselectAll": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitEditor.selectType": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.selectTypes": (context: KitEditorCommandContext, typeIds: Guid[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.addTypeToSelection": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          types: { added: [Guid] },
        },
      },
    };
  },
  "semio.kitEditor.removeTypeFromSelection": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          types: { removed: [Guid] },
        },
      },
    };
  },
  "semio.kitEditor.selectDesign": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.selectDesigns": (context: KitEditorCommandContext, designIds: Guid[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.addDesignToSelection": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          designs: { added: [Guid] },
        },
      },
    };
  },
  "semio.kitEditor.removeDesignFromSelection": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          designs: { removed: [Guid] },
        },
      },
    };
  },
  "semio.kitEditor.selectQuality": (context: KitEditorCommandContext, key: string): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.selectQualities": (context: KitEditorCommandContext, keys: string[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.addQualityToSelection": (context: KitEditorCommandContext, key: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          qualities: { added: [key] },
        },
      },
    };
  },
  "semio.kitEditor.removeQualityFromSelection": (context: KitEditorCommandContext, key: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          qualities: { removed: [key] },
        },
      },
    };
  },
  "semio.kitEditor.selectFile": (context: KitEditorCommandContext, path: string): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.selectFiles": (context: KitEditorCommandContext, paths: string[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.addFileToSelection": (context: KitEditorCommandContext, path: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          files: { added: [path] },
        },
      },
    };
  },
  "semio.kitEditor.removeFileFromSelection": (context: KitEditorCommandContext, path: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          files: { removed: [path] },
        },
      },
    };
  },
  "semio.kitEditor.selectAuthor": (context: KitEditorCommandContext, name: string): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.selectAuthors": (context: KitEditorCommandContext, names: string[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
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
  "semio.kitEditor.addAuthorToSelection": (context: KitEditorCommandContext, name: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          authors: { added: [name] },
        },
      },
    };
  },
  "semio.kitEditor.removeAuthorFromSelection": (context: KitEditorCommandContext, name: string): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          authors: { removed: [name] },
        },
      },
    };
  },
  "semio.kitEditor.deleteSelected": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const selection = context.kitEditor.selection;
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
  "semio.kitEditor.addType": (context: KitEditorCommandContext, type: Type): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: [type] },
      },
    };
  },
  "semio.kitEditor.addTypes": (context: KitEditorCommandContext, types: Type[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: types },
      },
    };
  },
  "semio.kitEditor.removeType": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: [Guid] },
      },
    };
  },
  "semio.kitEditor.removeTypes": (context: KitEditorCommandContext, typeIds: Guid[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: typeIds },
      },
    };
  },
  "semio.kitEditor.addDesign": (context: KitEditorCommandContext, design: Design): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: [design] },
      },
    };
  },
  "semio.kitEditor.addDesigns": (context: KitEditorCommandContext, designs: Design[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: designs },
      },
    };
  },
  "semio.kitEditor.removeDesign": (context: KitEditorCommandContext, Guid: Guid): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: [Guid] },
      },
    };
  },
  "semio.kitEditor.removeDesigns": (context: KitEditorCommandContext, designIds: Guid[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: designIds },
      },
    };
  },
  "semio.kitEditor.updateType": (context: KitEditorCommandContext, Guid: Guid, typeDiff: TypeDiff): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: [{ id: Guid, diff: typeDiff }] },
      },
    };
  },
  "semio.kitEditor.updateTypes": (context: KitEditorCommandContext, updates: { id: Guid; diff: TypeDiff }[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: updates },
      },
    };
  },
  "semio.kitEditor.updateDesign": (context: KitEditorCommandContext, Guid: Guid, designDiff: DesignDiff): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: [{ id: Guid, diff: designDiff }] },
      },
    };
  },
  "semio.kitEditor.updateDesigns": (context: KitEditorCommandContext, updates: { id: Guid; diff: DesignDiff }[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: updates },
      },
    };
  },
  "semio.kitEditor.setFilterSearch": (context: KitEditorCommandContext, search: string): KitEditorCommandResult => {
    return {
      diff: {
        filterSearch: search,
      },
    };
  },
  "semio.kitEditor.setExpandedRows": (context: KitEditorCommandContext, rows: string[]): KitEditorCommandResult => {
    return {
      diff: {
        expandedRows: rows,
      },
    };
  },
  "semio.kitEditor.toggleExpandedRow": (context: KitEditorCommandContext, rowId: string): KitEditorCommandResult => {
    const currentRows = context.kitEditor.expandedRows || [];
    const newRows = currentRows.includes(rowId) ? currentRows.filter((r) => r !== rowId) : [...currentRows, rowId];
    return {
      diff: {
        expandedRows: newRows,
      },
    };
  },
  "semio.kitEditor.setSortColumn": (context: KitEditorCommandContext, column: KitEditorSortColumn): KitEditorCommandResult => {
    return {
      diff: {
        sortColumn: column,
      },
    };
  },
  "semio.kitEditor.setSortDirection": (context: KitEditorCommandContext, direction: KitEditorSortDirection): KitEditorCommandResult => {
    return {
      diff: {
        sortDirection: direction,
      },
    };
  },
  "semio.kitEditor.toggleSort": (context: KitEditorCommandContext, column: KitEditorSortColumn): KitEditorCommandResult => {
    const current = context.kitEditor;
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
