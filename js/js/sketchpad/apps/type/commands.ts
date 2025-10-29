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
import { TypeAppCommandContext, TypeAppCommandResult, TypeAppFullscreenWindow } from "./store";

export const commands = {
  "semio.typeApp.startTransaction": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return { diff: {} };
  },
  "semio.typeApp.finalizeTransaction": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return { diff: {} };
  },
  "semio.typeApp.abortTransaction": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return { diff: {} };
  },
  "semio.typeApp.undo": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return { diff: {} };
  },
  "semio.typeApp.redo": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return { diff: {} };
  },
  "semio.typeApp.selectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const type = context.kit.types?.find((t) => t.guid === context.Guid);
    const currentSelection = context.typeApp.selection;
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
  "semio.typeApp.deselectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const currentSelection = context.typeApp.selection;
    return {
      diff: {
        selection: {
          ports: { removed: currentSelection?.ports ?? [] },
          representations: { removed: currentSelection?.representations ?? [] },
        },
      },
    };
  },
  "semio.typeApp.setActiveTool": (context: TypeAppCommandContext, tool: ToolType): TypeAppCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.typeApp.setCamera": (context: TypeAppCommandContext, camera: Camera): TypeAppCommandResult => {
    return {
      diff: {
        camera,
      },
    };
  },
  "semio.typeApp.focusPort": (context: TypeAppCommandContext, portGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        focusedPortGuid: portGuid,
      },
    };
  },
  "semio.typeApp.clearFocus": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        focusedPortGuid: null,
      },
    };
  },
  "semio.typeApp.togglePortsFullscreen": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const currentPanel = context.typeApp.fullscreenWindow;
    const newPanel = currentPanel === TypeAppFullscreenWindow.Ports ? TypeAppFullscreenWindow.None : TypeAppFullscreenWindow.Ports;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.typeApp.toggleRepresentationsFullscreen": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const currentPanel = context.typeApp.fullscreenWindow;
    const newPanel = currentPanel === TypeAppFullscreenWindow.Representations ? TypeAppFullscreenWindow.None : TypeAppFullscreenWindow.Representations;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.typeApp.selectPort": (context: TypeAppCommandContext, portId: Guid): TypeAppCommandResult => {
    const currentSelection = context.typeApp.selection;
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
  "semio.typeApp.deselectPort": (context: TypeAppCommandContext, portId: Guid): TypeAppCommandResult => {
    const currentSelection = context.typeApp.selection;
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
  "semio.typeApp.hoverPort": (context: TypeAppCommandContext, portId: Guid): TypeAppCommandResult => {
    return {
      diff: {
        hover: {
          port: portId,
        },
      },
    };
  },
  "semio.typeApp.clearHover": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
  "semio.typeApp.selectRepresentation": (context: TypeAppCommandContext, representationGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selectedRepresentationGuid: representationGuid,
      },
    };
  },
};
