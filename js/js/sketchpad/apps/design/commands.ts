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

import { Camera, Connection, ConnectionDiff, Coord, findDesignInKit, Guid, Piece, PieceDiff } from "../../../semio";
import { Access, Layout, Theme, ToolType } from "../../store";
import { DesignAppCommandContext, DesignAppCommandResult, DesignAppFullscreenWindow } from "./store";

export const commands = {
  "semio.designApp.setAccess": (context: DesignAppCommandContext, access: Access): DesignAppCommandResult => {
    return { diff: {} };
  },
  "semio.designApp.setTheme": (context: DesignAppCommandContext, theme: Theme): DesignAppCommandResult => {
    return { diff: {} };
  },
  "semio.designApp.setLayout": (context: DesignAppCommandContext, layout: Layout): DesignAppCommandResult => {
    return { diff: {} };
  },
  "semio.designApp.toggleDiagramFullscreen": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentPanel = context.designApp.fullscreenWindow;
    const newPanel = currentPanel === DesignAppFullscreenWindow.Diagram ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Diagram;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.designApp.toggleAccesslFullscreen": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentPanel = context.designApp.fullscreenWindow;
    const newPanel = currentPanel === DesignAppFullscreenWindow.Accessl ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Accessl;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.designApp.setActiveTool": (context: DesignAppCommandContext, tool: ToolType): DesignAppCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.designApp.selectAll": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid)!;
    const currentSelection = context.designApp.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection?.pieces ?? [],
            added: design.pieces?.map((p) => p.guid) ?? [],
          },
          connections: {
            removed: currentSelection?.connections ?? [],
            added: design.connections?.map((c) => c.guid) ?? [],
          },
        },
      },
    };
  },
  "semio.designApp.deselectAll": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection;
    return {
      diff: {
        selection: {
          pieces: { removed: currentSelection?.pieces ?? [] },
          connections: { removed: currentSelection?.connections ?? [] },
          port: undefined,
        },
      },
    };
  },
  "semio.designApp.selectPiece": (context: DesignAppCommandContext, Guid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection?.pieces ?? [],
            added: [Guid],
          },
          connections: { removed: currentSelection?.connections ?? [] },
          port: undefined,
        },
      },
    };
  },
  "semio.designApp.selectPieces": (context: DesignAppCommandContext, pieceIds: Guid[]): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection?.pieces ?? [],
            added: pieceIds,
          },
          connections: { removed: currentSelection?.connections ?? [] },
          port: undefined,
        },
      },
    };
  },
  "semio.designApp.addPieceToSelection": (context: DesignAppCommandContext, Guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          pieces: { added: [Guid] },
        },
      },
    };
  },
  "semio.designApp.removePieceFromSelection": (context: DesignAppCommandContext, Guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          pieces: { removed: [Guid] },
        },
      },
    };
  },
  "semio.designApp.selectPiecePort": (context: DesignAppCommandContext, pieceGuid: Guid, portGuid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: { port: { piece: pieceGuid, port: portGuid } },
      },
    };
  },
  "semio.designApp.deselectPiecePort": (context: DesignAppCommandContext): DesignAppCommandResult => {
    return {
      diff: {
        selection: { port: undefined },
      },
    };
  },
  "semio.designApp.deleteSelected": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const selection = context.designApp.selection;
    const design = findDesignInKit(context.kit, context.Guid);
    const selectedConnections =
      selection?.connections
        ?.map((connGuid) => design?.connections?.find((c) => c.guid === connGuid))
        .filter((c): c is Connection => c !== undefined)
        .map((c) => ({ connected: { piece: c.connected.piece }, connecting: { piece: c.connecting.piece } })) ?? [];
    return {
      diff: {
        selection: {
          pieces: { removed: selection?.pieces ?? [] },
          connections: { removed: selection?.connections ?? [] },
        },
      },
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { pieces: { removed: selection?.pieces ?? [] }, connections: { removed: selectedConnections } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addPiece": (context: DesignAppCommandContext, piece: Piece): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, context.Guid)?.connections ?? []).some((connection) => connection.connected.piece === piece.guid || connection.connecting.piece === piece.guid)
                      ? piece
                      : {
                        ...piece,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addPieces": (context: DesignAppCommandContext, pieces: Piece[]): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid);
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece === candidate.guid || connection.connecting.piece === candidate.guid)
                      ? candidate
                      : {
                        ...candidate,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removePiece": (context: DesignAppCommandContext, Guid: Guid): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { pieces: { removed: [Guid] } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removePieces": (context: DesignAppCommandContext, pieceIds: Guid[]): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { pieces: { removed: pieceIds } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addConnection": (context: DesignAppCommandContext, connection: Connection): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addConnections": (context: DesignAppCommandContext, connections: Connection[]): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removeConnection": (context: DesignAppCommandContext, Guid: Guid): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid);
    const connection = design?.connections?.find((c) => c.guid === Guid);
    if (!connection) return {};
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { removed: [{ connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removeConnections": (context: DesignAppCommandContext, connectionIds: Guid[]): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid);
    const connectionsToRemove =
      connectionIds
        .map((connGuid) => design?.connections?.find((c) => c.guid === connGuid))
        .filter((c): c is Connection => c !== undefined)
        .map((c) => ({ connected: { piece: c.connected.piece }, connecting: { piece: c.connecting.piece } })) ?? [];
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { removed: connectionsToRemove } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updatePiece": (context: DesignAppCommandContext, Guid: Guid, pieceDiff: PieceDiff): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { pieces: { updated: [{ id: Guid, diff: pieceDiff }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updatePieces": (context: DesignAppCommandContext, updates: { id: Guid; diff: PieceDiff }[]): DesignAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { pieces: { updated: updates } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updateConnection": (context: DesignAppCommandContext, Guid: Guid, connectionDiff: ConnectionDiff): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid);
    const connection = design?.connections?.find((c) => c.guid === Guid);
    if (!connection) return {};
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { updated: [{ id: { connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } }, diff: connectionDiff }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updateConnections": (context: DesignAppCommandContext, updates: { id: Guid; diff: ConnectionDiff }[]): DesignAppCommandResult => {
    const design = findDesignInKit(context.kit, context.Guid);
    const updatesWithConnectionIds = updates
      .map((update) => {
        const connection = design?.connections?.find((c) => c.guid === update.id);
        if (!connection) return null;
        return {
          id: { connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } },
          diff: update.diff,
        };
      })
      .filter((u): u is { id: { connected: { piece: string }; connecting: { piece: string } }; diff: ConnectionDiff } => u !== null);
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.Guid,
              diff: { connections: { updated: updatesWithConnectionIds } },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.selectConnection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection;
    return {
      diff: {
        selection: {
          pieces: { removed: currentSelection?.pieces ?? [] },
          connections: {
            removed: currentSelection?.connections ?? [],
            added: [connectionGuid],
          },
          port: undefined,
        },
      },
    };
  },
  "semio.designApp.addConnectionToSelection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          connections: { added: [connectionGuid] },
        },
      },
    };
  },
  "semio.designApp.removeConnectionFromSelection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          connections: { removed: [connectionGuid] },
        },
      },
    };
  },
  "semio.designApp.setCamera": (context: DesignAppCommandContext, camera: Camera): DesignAppCommandResult => {
    return {
      diff: {
        camera,
      },
    };
  },
  "semio.designApp.setDiagramCenter": (context: DesignAppCommandContext, center: Coord): DesignAppCommandResult => {
    return {
      diff: {
        diagramCenter: center,
      },
    };
  },
  "semio.designApp.setDiagramScale": (context: DesignAppCommandContext, scale: number): DesignAppCommandResult => {
    return {
      diff: {
        diagramScale: scale,
      },
    };
  },
  "semio.designApp.hoverPiece": (context: DesignAppCommandContext, pieceGuid: Guid): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    const designGuid = context.kit.designs?.find((design) => design.guid === context.Guid)?.pieces?.find((piece) => piece.guid === pieceGuid)?.design;
    return {
      diff: {
        hover: {
          ...currentHover,
          pieces: [pieceGuid],
          designs: designGuid ? [designGuid] : undefined,
        },
      },
    };
  },
  "semio.designApp.hoverPieces": (context: DesignAppCommandContext, pieceGuids: Guid[]): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    const designPieces = context.kit.designs?.find((design) => design.guid === context.Guid)?.pieces;
    const designGuidSet = new Set<Guid>();
    if (designPieces) {
      pieceGuids.forEach((guid) => {
        const candidate = designPieces.find((piece) => piece.guid === guid)?.design;
        if (candidate) designGuidSet.add(candidate);
      });
    }
    return {
      diff: {
        hover: {
          ...currentHover,
          pieces: pieceGuids,
          designs: designGuidSet.size > 0 ? Array.from(designGuidSet) : undefined,
        },
      },
    };
  },
  "semio.designApp.hoverConnection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, connections: [connectionGuid] },
      },
    };
  },
  "semio.designApp.hoverConnections": (context: DesignAppCommandContext, connectionGuids: Guid[]): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, connections: connectionGuids },
      },
    };
  },
  "semio.designApp.hoverPort": (context: DesignAppCommandContext, pieceGuid: Guid, portGuid: Guid): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    const designGuid = context.kit.designs?.find((design) => design.guid === context.Guid)?.pieces?.find((piece) => piece.guid === pieceGuid)?.design;
    return {
      diff: {
        hover: {
          ...currentHover,
          pieces: [pieceGuid],
          ports: [{ piece: pieceGuid, port: portGuid }],
          designs: designGuid ? [designGuid] : undefined,
        },
      },
    };
  },
  "semio.designApp.hoverType": (context: DesignAppCommandContext, typeGuid: Guid): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, types: [typeGuid] },
      },
    };
  },
  "semio.designApp.hoverTypes": (context: DesignAppCommandContext, typeGuids: Guid[]): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, types: typeGuids },
      },
    };
  },
  "semio.designApp.hoverDesign": (context: DesignAppCommandContext, designGuid: Guid): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, designs: [designGuid] },
      },
    };
  },
  "semio.designApp.hoverDesigns": (context: DesignAppCommandContext, designGuids: Guid[]): DesignAppCommandResult => {
    const currentHover = context.designApp.hover ?? {};
    return {
      diff: {
        hover: { ...currentHover, designs: designGuids },
      },
    };
  },
  "semio.designApp.clearHover": (context: DesignAppCommandContext): DesignAppCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
};
