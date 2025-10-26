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

import { Camera, Guid } from "../../../semio";
import { ToolType } from "../../store";
import { TypeEditorCommandContext, TypeEditorCommandResult, TypeEditorFullscreenWindow } from "./store";

export const commands = {
  "semio.typeEditor.startTransaction": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return { diff: {} };
  },
  "semio.typeEditor.finalizeTransaction": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return { diff: {} };
  },
  "semio.typeEditor.abortTransaction": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return { diff: {} };
  },
  "semio.typeEditor.undo": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return { diff: {} };
  },
  "semio.typeEditor.redo": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return { diff: {} };
  },
  "semio.typeEditor.selectAll": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    const type = context.kit.types?.find((t) => t.guid === context.Guid);
    const currentSelection = context.typeEditor.selection;
    return {
      diff: {
        selection: {
          ports: {
            removed: currentSelection?.ports ?? [],
            added: type?.ports?.map((p) => p.guid) ?? [],
          },
          representations: {
            removed: currentSelection?.representations ?? [],
            added: type?.representations?.map((r) => r.guid) ?? [],
          },
        },
      },
    };
  },
  "semio.typeEditor.deselectAll": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    const currentSelection = context.typeEditor.selection;
    return {
      diff: {
        selection: {
          ports: { removed: currentSelection?.ports ?? [] },
          representations: { removed: currentSelection?.representations ?? [] },
        },
      },
    };
  },
  "semio.typeEditor.setActiveTool": (context: TypeEditorCommandContext, tool: ToolType): TypeEditorCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.typeEditor.setCamera": (context: TypeEditorCommandContext, camera: Camera): TypeEditorCommandResult => {
    return {
      diff: {
        camera,
      },
    };
  },
  "semio.typeEditor.togglePortsFullscreen": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    const currentPanel = context.typeEditor.fullscreenWindow;
    const newPanel = currentPanel === TypeEditorFullscreenWindow.Ports ? TypeEditorFullscreenWindow.None : TypeEditorFullscreenWindow.Ports;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.typeEditor.toggleRepresentationsFullscreen": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    const currentPanel = context.typeEditor.fullscreenWindow;
    const newPanel = currentPanel === TypeEditorFullscreenWindow.Representations ? TypeEditorFullscreenWindow.None : TypeEditorFullscreenWindow.Representations;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.typeEditor.selectPort": (context: TypeEditorCommandContext, portId: Guid): TypeEditorCommandResult => {
    const currentSelection = context.typeEditor.selection;
    const isSelected = currentSelection?.ports?.includes(portId);
    if (isSelected) return { diff: {} };
    return {
      diff: {
        selection: {
          ports: {
            removed: currentSelection?.ports ?? [],
            added: [portId],
          },
          representations: {
            removed: currentSelection?.representations ?? [],
          },
        },
      },
    };
  },
  "semio.typeEditor.deselectPort": (context: TypeEditorCommandContext, portId: Guid): TypeEditorCommandResult => {
    const currentSelection = context.typeEditor.selection;
    return {
      diff: {
        selection: {
          ports: {
            removed: [portId],
          },
        },
      },
    };
  },
  "semio.typeEditor.hoverPort": (context: TypeEditorCommandContext, portId: Guid): TypeEditorCommandResult => {
    return {
      diff: {
        hover: {
          port: portId,
        },
      },
    };
  },
  "semio.typeEditor.clearHover": (context: TypeEditorCommandContext): TypeEditorCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
};
