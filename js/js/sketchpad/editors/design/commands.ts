import { Connection } from "@xyflow/react";
import { Camera } from "three";
import { ConnectionDiff, Coord, findDesignInKit, Guid, Piece, PieceDiff } from "../../../semio";
import { Access, Layout, Theme, ToolType } from "../../store";
import { DesignEditorCommandContext, DesignEditorCommandResult, DesignEditorFullscreenWindow } from "./store";

export const commands = {
    "semio.designEditor.setAccess": (context: DesignEditorCommandContext, access: Access): DesignEditorCommandResult => {
        return { diff: {} };
    },
    "semio.designEditor.setTheme": (context: DesignEditorCommandContext, theme: Theme): DesignEditorCommandResult => {
        return { diff: {} };
    },
    "semio.designEditor.setLayout": (context: DesignEditorCommandContext, layout: Layout): DesignEditorCommandResult => {
        return { diff: {} };
    },
    "semio.designEditor.toggleDiagramFullscreen": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        const currentPanel = context.designEditor.fullscreenWindow;
        const newPanel = currentPanel === DesignEditorFullscreenWindow.Diagram ? DesignEditorFullscreenWindow.None : DesignEditorFullscreenWindow.Diagram;
        return {
            diff: {
                fullscreenWindow: newPanel,
            },
        };
    },
    "semio.designEditor.toggleAccesslFullscreen": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        const currentPanel = context.designEditor.fullscreenWindow;
        const newPanel = currentPanel === DesignEditorFullscreenWindow.Accessl ? DesignEditorFullscreenWindow.None : DesignEditorFullscreenWindow.Accessl;
        return {
            diff: {
                fullscreenWindow: newPanel,
            },
        };
    },
    "semio.designEditor.setActiveTool": (context: DesignEditorCommandContext, tool: ToolType): DesignEditorCommandResult => {
        return {
            diff: {
                activeTool: tool,
            },
        };
    },
    "semio.designEditor.selectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        const design = findDesignInKit(context.kit, context.Guid)!;
        const currentSelection = context.designEditor.selection;
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
    "semio.designEditor.deselectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        const currentSelection = context.designEditor.selection;
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
    "semio.designEditor.selectPiece": (context: DesignEditorCommandContext, Guid: Guid): DesignEditorCommandResult => {
        const currentSelection = context.designEditor.selection;
        return {
            diff: {
                selection: {
                    pieces: {
                        removed: currentSelection?.pieces ?? [],
                        added: [Guid],
                    },
                    connections: { removed: currentSelection?.connections ?? [] },
                },
            },
        };
    },
    "semio.designEditor.selectPieces": (context: DesignEditorCommandContext, pieceIds: Guid[]): DesignEditorCommandResult => {
        const currentSelection = context.designEditor.selection;
        return {
            diff: {
                selection: {
                    pieces: {
                        removed: currentSelection?.pieces ?? [],
                        added: pieceIds,
                    },
                    connections: { removed: currentSelection?.connections ?? [] },
                },
            },
        };
    },
    "semio.designEditor.addPieceToSelection": (context: DesignEditorCommandContext, Guid: Guid): DesignEditorCommandResult => {
        return {
            diff: {
                selection: {
                    pieces: { added: [Guid] },
                },
            },
        };
    },
    "semio.designEditor.removePieceFromSelection": (context: DesignEditorCommandContext, Guid: Guid): DesignEditorCommandResult => {
        return {
            diff: {
                selection: {
                    pieces: { removed: [Guid] },
                },
            },
        };
    },
    "semio.designEditor.selectPiecePort": (context: DesignEditorCommandContext, pieceGuid: Guid, portGuid: Guid): DesignEditorCommandResult => {
        return {
            diff: {
                selection: { port: { piece: pieceGuid, port: portGuid } },
            },
        };
    },
    "semio.designEditor.deselectPiecePort": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        return {
            diff: {
                selection: { port: undefined },
            },
        };
    },
    "semio.designEditor.deleteSelected": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        const selection = context.designEditor.selection;
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
    "semio.designEditor.addPiece": (context: DesignEditorCommandContext, piece: Piece): DesignEditorCommandResult => {
        return {
            diff: {},
            kitDiff: {
                designs: {
                    updated: [
                        {
                            id: context.Guid,
                            diff: { pieces: { added: [piece] } },
                        },
                    ],
                },
            },
        };
    },
    "semio.designEditor.addPieces": (context: DesignEditorCommandContext, pieces: Piece[]): DesignEditorCommandResult => {
        return {
            diff: {},
            kitDiff: {
                designs: {
                    updated: [
                        {
                            id: context.Guid,
                            diff: { pieces: { added: pieces } },
                        },
                    ],
                },
            },
        };
    },
    "semio.designEditor.removePiece": (context: DesignEditorCommandContext, Guid: Guid): DesignEditorCommandResult => {
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
    "semio.designEditor.removePieces": (context: DesignEditorCommandContext, pieceIds: Guid[]): DesignEditorCommandResult => {
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
    "semio.designEditor.addConnection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
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
    "semio.designEditor.addConnections": (context: DesignEditorCommandContext, connections: Connection[]): DesignEditorCommandResult => {
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
    "semio.designEditor.removeConnection": (context: DesignEditorCommandContext, Guid: Guid): DesignEditorCommandResult => {
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
    "semio.designEditor.removeConnections": (context: DesignEditorCommandContext, connectionIds: Guid[]): DesignEditorCommandResult => {
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
    "semio.designEditor.updatePiece": (context: DesignEditorCommandContext, Guid: Guid, pieceDiff: PieceDiff): DesignEditorCommandResult => {
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
    "semio.designEditor.updatePieces": (context: DesignEditorCommandContext, updates: { id: Guid; diff: PieceDiff }[]): DesignEditorCommandResult => {
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
    "semio.designEditor.updateConnection": (context: DesignEditorCommandContext, Guid: Guid, connectionDiff: ConnectionDiff): DesignEditorCommandResult => {
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
    "semio.designEditor.updateConnections": (context: DesignEditorCommandContext, updates: { id: Guid; diff: ConnectionDiff }[]): DesignEditorCommandResult => {
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
    "semio.designEditor.selectConnection": (context: DesignEditorCommandContext, connectionGuid: Guid): DesignEditorCommandResult => {
        const currentSelection = context.designEditor.selection;
        return {
            diff: {
                selection: {
                    pieces: { removed: currentSelection?.pieces ?? [] },
                    connections: {
                        removed: currentSelection?.connections ?? [],
                        added: [connectionGuid],
                    },
                },
            },
        };
    },
    "semio.designEditor.addConnectionToSelection": (context: DesignEditorCommandContext, connectionGuid: Guid): DesignEditorCommandResult => {
        return {
            diff: {
                selection: {
                    connections: { added: [connectionGuid] },
                },
            },
        };
    },
    "semio.designEditor.removeConnectionFromSelection": (context: DesignEditorCommandContext, connectionGuid: Guid): DesignEditorCommandResult => {
        return {
            diff: {
                selection: {
                    connections: { removed: [connectionGuid] },
                },
            },
        };
    },
    "semio.designEditor.setCamera": (context: DesignEditorCommandContext, camera: Camera): DesignEditorCommandResult => {
        return {
            diff: {
                camera,
            },
        };
    },
    "semio.designEditor.setDiagramCenter": (context: DesignEditorCommandContext, center: Coord): DesignEditorCommandResult => {
        return {
            diff: {
                diagramCenter: center,
            },
        };
    },
    "semio.designEditor.setDiagramScale": (context: DesignEditorCommandContext, scale: number): DesignEditorCommandResult => {
        return {
            diff: {
                diagramScale: scale,
            },
        };
    },
    "semio.designEditor.hoverPiece": (context: DesignEditorCommandContext, pieceGuid: Guid): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        const designGuid =
            context.kit.designs?.find((design) => design.guid === context.Guid)?.pieces?.find((piece) => piece.guid === pieceGuid)?.design;
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
    "semio.designEditor.hoverPieces": (context: DesignEditorCommandContext, pieceGuids: Guid[]): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
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
    "semio.designEditor.hoverConnection": (context: DesignEditorCommandContext, connectionGuid: Guid): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, connections: [connectionGuid] },
            },
        };
    },
    "semio.designEditor.hoverConnections": (context: DesignEditorCommandContext, connectionGuids: Guid[]): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, connections: connectionGuids },
            },
        };
    },
    "semio.designEditor.hoverPort": (context: DesignEditorCommandContext, pieceGuid: Guid, portGuid: Guid): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        const designGuid =
            context.kit.designs?.find((design) => design.guid === context.Guid)?.pieces?.find((piece) => piece.guid === pieceGuid)?.design;
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
    "semio.designEditor.hoverType": (context: DesignEditorCommandContext, typeGuid: Guid): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, types: [typeGuid] },
            },
        };
    },
    "semio.designEditor.hoverTypes": (context: DesignEditorCommandContext, typeGuids: Guid[]): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, types: typeGuids },
            },
        };
    },
    "semio.designEditor.hoverDesign": (context: DesignEditorCommandContext, designGuid: Guid): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, designs: [designGuid] },
            },
        };
    },
    "semio.designEditor.hoverDesigns": (context: DesignEditorCommandContext, designGuids: Guid[]): DesignEditorCommandResult => {
        const currentHover = context.designEditor.hover ?? {};
        return {
            diff: {
                hover: { ...currentHover, designs: designGuids },
            },
        };
    },
    "semio.designEditor.clearHover": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
        return {
            diff: {
                hover: {},
            },
        };
    },
};
