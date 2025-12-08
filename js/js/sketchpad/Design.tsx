// #region Header

// Design.tsx

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

// #region Commands

// Commands are defined after interfaces below

// #endregion Commands

// #region Internal State Management

import * as Y from "yjs";
import { ConnectionDiff, ConnectionId, Guid, KitDiff, PieceDiff, PieceId } from "../semio";
import type {
  AppConfig,
  AppWindowConfig,
  DesignAppId,
  GranularHookResult,
  KitCommandContext,
  KitDiffAppEdit,
  PanelDefinition,
  PanelVisibility,
  Tool,
  ToolDefinition,
  ToolRenderContext,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "./shared";
import { conditionalHookResult, createPanelDefinition, Expertise, Mode, PanelKind, readonlyHookResult, Theme, ToolKind } from "./shared";
import { identitySelector, useDesignScope, useKitScope, usePieceScope, useSketchpadActor } from "./Sketchpad";

// #endregion Internal State Management

// #region Imports

import { DragEndEvent, useDraggable } from "@dnd-kit/core";
import { arrayMove } from "@dnd-kit/sortable";
import { Edges, Line, Select, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import { AddIcon, AwardIcon, CodeIcon, ConnectionIcon, DiagramIcon, DisconnectIcon, HandIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoveIcon, SceneIcon, SelectToolIcon, SunIcon, TableViewIcon, TutorialIcon, UserIcon } from "@semio/assets";
import React, { createContext, FC, memo, ReactNode, Suspense, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { useLabel } from "../i18n";

import type { ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, Connection as RFConnection } from "@xyflow/react";
import { applyNodeChanges, BaseEdge, Handle, Position, ReactFlowInstance, ReactFlowProvider, useReactFlow, ViewportPortal } from "@xyflow/react";
import {
  arePortsCompatible,
  areSameConnection,
  Camera,
  Connection,
  Coord,
  Design,
  DiffStatus,
  findAttributeValue,
  findConnectionsInDesign,
  findDesignInKit,
  findModel,
  findPieceInDesign,
  findPortInType,
  findTypeInKit,
  generateUniqueName,
  getIncludedDesigns,
  guid,
  ICON_WIDTH,
  isPortInUse,
  Kit,
  Model,
  Piece,
  Plane,
  planeToMatrix,
  Port,
  selectBestModel,
  TOLERANCE,
  toThreeRotation,
  Type,
} from "../semio";
import {
  Avatar,
  AvatarFallback,
  Button,
  Combobox,
  Diagram,
  DraggableAvatar,
  Geometry,
  Input,
  Scene,
  SelectContent,
  SelectItem,
  SelectTrigger,
  Select as SelectUI,
  SelectValue,
  Slider,
  SortableTreeItems,
  Stepper,
  Textarea,
  ToggleGroup,
  TreeContent,
  TreeItem,
  TreeSection,
} from "./elements";
import {
  Canvas,
  ConnectionScopeProvider,
  DesignScopeProvider,
  DesignStore,
  getKitAppHooks,
  KitDiffAppStore,
  KitScopeProvider,
  KitStore,
  LayoutCanvas,
  PieceMetadata,
  PieceScopeProvider,
  SketchpadStore,
  ToolGroup,
  useAddFooterItem,
  useAddPanelSection,
  useAppPanelVisibility,
  useAppType,
  useClusterableGroups,
  useConnection,
  useConnectionGap,
  useConnectionRise,
  useConnectionRotation,
  useConnectionShift,
  useConnectionTilt,
  useConnectionTurn,
  useConnectionU,
  useConnectionV,
  useDesign,
  useDiffedPiece,
  useDragDrop,
  useExpertise,
  useExplodeableDesignNodes,
  useFlatPiecePlane,
  useFocusSafe,
  useIsConnectionHovered,
  useIsInDesignScope,
  useIsPieceHovered,
  useIsPieceSelected,
  useIsPieceTransitiveHovered,
  useKit,
  useKitCommands,
  useKitDesigns,
  useKitFiles,
  useKitStore,
  useKitTags,
  useKitTypes,
  useLanguage,
  useLayout,
  useMode,
  usePiece,
  usePiecesFromIds,
  usePiecesMetadataMap,
  usePieceStatus,
  useRemoveFooterItem,
  useRemovePanelSection,
  useReplacableDesigns,
  useReplacableTypes,
  useSketchpad,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
  useSyncField,
  useSyncNestedArrayItemMembership,
  useSyncSelectionItemMembership,
  useTheme,
  useTooltip,
  useType,
} from "./Sketchpad";
import { useDesignApp as useDesignAppXState } from "./xstate-hooks";

// Stub for controller factory registration - legacy pattern being phased out
const registerDesignAppControllerFactory = (_factory: any) => {
  // No-op: Controller pattern deprecated in favor of XState
};

let kitAppModuleCache: any = null;
if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
  kitAppModuleCache = (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache;
}
const getKitAppModule = () => {
  if (!kitAppModuleCache) {
    if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
      kitAppModuleCache = (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache;
    }
    if (!kitAppModuleCache) {
      throw new Error("Kit app module not loaded. This should not happen - ensure kit app is imported.");
    }
  }
  return kitAppModuleCache;
};

const KitSectionLazy = React.lazy(async () => {
  const module = await import("./Kit");
  kitAppModuleCache = module;
  if (typeof window !== "undefined") {
    if (!(window as any).__KIT_APP_MODULE_CACHE__) {
      (window as any).__KIT_APP_MODULE_CACHE__ = {};
    }
    (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache = module;
  }
  return { default: module.KitSection };
});

// Local components - will be consolidated into regions below
// import { DesignAppFooter } from "./Footer";
// import { ToolsToggleGroup } from "./Tools";

// designAppCommands will be set after commands are defined below
let designAppCommands: Record<string, (context: any, ...args: any[]) => Promise<any> | any>;

type YDesignAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | Y.Map<boolean> | YAttributes | YStringArray;
type YDesignApp = Y.Map<YDesignAppVal>;
type YDesignApps = Y.Map<Y.Map<YDesignApp>>;

export interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  port?: { piece: Guid; designPiece?: Guid; port: Guid };
}
export interface DesignAppSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignAppSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignAppSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  port?: Guid;
}
export interface DesignAppSelectionDiff {
  pieces?: DesignAppSelectionPiecesDiff;
  connections?: DesignAppSelectionConnectionsDiff;
  port?: DesignAppSelectionPortDiff;
}
export enum DesignAppFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
export enum DesignAppWindowKind {
  Diagram = "diagram",
  Scene = "scene",
}
export interface DesignAppPresence {
  cursor?: Coord;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  ports?: { piece: Guid; designPiece?: Guid; port: Guid }[];
  types?: Guid[];
  designs?: Guid[];
}
export interface DesignAppPresenceOther extends DesignAppPresence {
  name: string;
}
export interface DesignAppDiff {
  selection?: DesignAppSelectionDiff;
  presence?: DesignAppPresence;
  hover?: DesignAppHover;
  fullscreenWindow?: DesignAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolKind;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  focusedPieceGuid?: Guid | null;
  selectedModelTags?: Record<Guid, string[]>;
  windowLayout?: any;
}
export interface DesignAppEdit extends KitDiffAppEdit<DesignAppSelectionDiff> {}
export interface DesignAppState {
  fullscreenWindow: DesignAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool?: ToolKind;
  selection?: DesignAppSelection;
  hover?: DesignAppHover;
  presence?: DesignAppPresence;
  others: DesignAppPresenceOther[];
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  focusedPieceGuid?: Guid;
  currentTransactionStackLength?: number;
  selectedModelTags?: Record<Guid, string[]>;
  windowLayout?: any;
}

export interface DesignAppCommandContext extends KitCommandContext {
  designApp: DesignAppState;
  Guid: Guid;
  design: Design;
}
export interface DesignAppCommandResult {
  diff?: DesignAppDiff;
  kitDiff?: KitDiff;
}

export const commands: Record<string, (context: DesignAppCommandContext, ...args: any[]) => DesignAppCommandResult> = {
  "semio.designApp.selectAll": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const allPieceGuids = context.design.pieces?.map((p: Piece) => p.guid) || [];
    const allConnectionGuids = context.design.connections?.map((c: Connection) => c.guid) || [];
    return {
      diff: {
        selection: {
          pieces: {
            added: allPieceGuids,
          },
          connections: {
            added: allConnectionGuids,
          },
        },
      },
    };
  },
  "semio.designApp.deselectAll": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentPieces = currentSelection.pieces || [];
    const currentConnections = currentSelection.connections || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentPieces,
          },
          connections: {
            removed: currentConnections,
          },
          port: {},
        },
      },
    };
  },
  "semio.designApp.deleteSelected": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const selectedPieces = currentSelection.pieces || [];
    const selectedConnections = currentSelection.connections || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: selectedPieces,
          },
          connections: {
            removed: selectedConnections,
          },
        },
      },
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  removed: selectedPieces.map((g) => ({ guid: g })),
                },
                connections: {
                  removed: selectedConnections.map((g) => ({ guid: g })),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.hoverPiece": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          pieces: [guid],
        },
      },
    };
  },
  "semio.designApp.hoverPieces": (context: DesignAppCommandContext, guids: Guid[]): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          pieces: guids,
        },
      },
    };
  },
  "semio.designApp.hoverConnection": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          connections: [guid],
        },
      },
    };
  },
  "semio.designApp.hoverConnections": (context: DesignAppCommandContext, guids: Guid[]): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          connections: guids,
        },
      },
    };
  },
  "semio.designApp.hoverPort": (context: DesignAppCommandContext, pieceGuid: Guid, portGuid: Guid, designPieceGuid?: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          ports: [{ piece: pieceGuid, designPiece: designPieceGuid, port: portGuid }],
        },
      },
    };
  },
  "semio.designApp.hoverType": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          types: [guid],
        },
      },
    };
  },
  "semio.designApp.hoverTypes": (context: DesignAppCommandContext, guids: Guid[]): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          types: guids,
        },
      },
    };
  },
  "semio.designApp.hoverDesign": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          designs: [guid],
        },
      },
    };
  },
  "semio.designApp.hoverDesigns": (context: DesignAppCommandContext, guids: Guid[]): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          designs: guids,
        },
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
  "semio.designApp.setCamera": (context: DesignAppCommandContext, camera: Camera): DesignAppCommandResult => {
    return {
      diff: {
        camera,
      },
    };
  },
  "semio.designApp.focusPiece": (context: DesignAppCommandContext, pieceGuid: Guid): DesignAppCommandResult => {
    return {
      diff: {
        focusedPieceGuid: pieceGuid,
      },
    };
  },
  "semio.designApp.clearFocus": (context: DesignAppCommandContext): DesignAppCommandResult => {
    return {
      diff: {
        focusedPieceGuid: undefined,
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
  "semio.designApp.toggleDiagramFullscreen": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentFullscreen = context.designApp.fullscreenWindow;
    return {
      diff: {
        fullscreenWindow: currentFullscreen === DesignAppFullscreenWindow.Diagram ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Diagram,
      },
    };
  },
  "semio.designApp.toggleAccesslFullscreen": (context: DesignAppCommandContext): DesignAppCommandResult => {
    const currentFullscreen = context.designApp.fullscreenWindow;
    return {
      diff: {
        fullscreenWindow: currentFullscreen === DesignAppFullscreenWindow.Accessl ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Accessl,
      },
    };
  },
  "semio.designApp.setActiveTool": (context: DesignAppCommandContext, tool: ToolKind): DesignAppCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.designApp.selectPiece": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentPieces = currentSelection.pieces || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentPieces,
            added: [guid],
          },
          connections: {
            removed: currentSelection.connections || [],
          },
          port: {},
        },
      },
    };
  },
  "semio.designApp.selectPieces": (context: DesignAppCommandContext, guids: Guid[]): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentPieces = currentSelection.pieces || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentPieces,
            added: guids,
          },
          connections: {
            removed: currentSelection.connections || [],
          },
          port: {},
        },
      },
    };
  },
  "semio.designApp.addPieceToSelection": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentPieces = currentSelection.pieces || [];
    if (currentPieces.includes(guid)) {
      return { diff: {} };
    }
    return {
      diff: {
        selection: {
          pieces: {
            added: [guid],
          },
        },
      },
    };
  },
  "semio.designApp.removePieceFromSelection": (context: DesignAppCommandContext, guid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentPieces = currentSelection.pieces || [];
    if (!currentPieces.includes(guid)) {
      return { diff: {} };
    }
    return {
      diff: {
        selection: {
          pieces: {
            removed: [guid],
          },
        },
      },
    };
  },
  "semio.designApp.selectConnection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentConnections = currentSelection.connections || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection.pieces || [],
          },
          connections: {
            removed: currentConnections,
            added: [connectionGuid],
          },
          port: {},
        },
      },
    };
  },
  "semio.designApp.addConnectionToSelection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentConnections = currentSelection.connections || [];
    if (currentConnections.includes(connectionGuid)) {
      return { diff: {} };
    }
    return {
      diff: {
        selection: {
          connections: {
            added: [connectionGuid],
          },
        },
      },
    };
  },
  "semio.designApp.removeConnectionFromSelection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    const currentSelection = context.designApp.selection || {};
    const currentConnections = currentSelection.connections || [];
    if (!currentConnections.includes(connectionGuid)) {
      return { diff: {} };
    }
    return {
      diff: {
        selection: {
          connections: {
            removed: [connectionGuid],
          },
        },
      },
    };
  },
  "semio.designApp.selectPiecePort": (context: DesignAppCommandContext, piece: Guid, port: Guid, designPiece?: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          pieces: {
            removed: context.designApp.selection?.pieces || [],
          },
          connections: {
            removed: context.designApp.selection?.connections || [],
          },
          port: {
            piece,
            port,
            designPiece,
          },
        },
      },
    };
  },
  "semio.designApp.deselectPiecePort": (context: DesignAppCommandContext): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          port: {},
        },
      },
    };
  },
  "semio.designApp.addPiece": (context: DesignAppCommandContext, piece: Piece): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  added: [piece],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addPieces": (context: DesignAppCommandContext, pieces: Piece[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  added: pieces,
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removePiece": (context: DesignAppCommandContext, pieceGuid: Guid): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  removed: [{ guid: pieceGuid }],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removePieces": (context: DesignAppCommandContext, pieceGuids: Guid[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  removed: pieceGuids.map((g) => ({ guid: g })),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addConnection": (context: DesignAppCommandContext, connection: Connection): DesignAppCommandResult => {
    // PERF: Calculate u/v from piece centers here instead of in components
    // This avoids components needing to subscribe to the entire design
    if (connection.u === undefined || connection.v === undefined) {
      const parentPiece = context.design.pieces?.find((p: Piece) => p.guid === connection.connected?.piece?.guid);
      const childPiece = context.design.pieces?.find((p: Piece) => p.guid === connection.connecting?.piece?.guid);
      if (parentPiece?.center && childPiece?.center) {
        connection.u = childPiece.center.u - parentPiece.center.u;
        connection.v = childPiece.center.v - parentPiece.center.v;
      }
    }
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  added: [connection],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.addConnections": (context: DesignAppCommandContext, connections: Connection[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  added: connections,
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removeConnection": (context: DesignAppCommandContext, connectionGuid: Guid): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  removed: [{ guid: connectionGuid }],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.removeConnections": (context: DesignAppCommandContext, connectionGuids: Guid[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  removed: connectionGuids.map((g) => ({ guid: g })),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updatePiece": (context: DesignAppCommandContext, pieceGuid: Guid, pieceDiff: PieceDiff): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  updated: [{ piece: { guid: pieceGuid }, diff: pieceDiff }],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updatePieces": (context: DesignAppCommandContext, updates: { piece: PieceId; diff: PieceDiff }[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                pieces: {
                  updated: updates,
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updateConnection": (context: DesignAppCommandContext, connectionGuid: Guid, connectionDiff: ConnectionDiff): DesignAppCommandResult => {
    const connection = context.design.connections?.find((c) => c.guid === connectionGuid);
    if (!connection) {
      return {};
    }
    const connectionId = { connected: { piece: connection.connected.piece.guid }, connecting: { piece: connection.connecting.piece.guid } };
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  updated: [{ connection: { guid: connectionGuid }, diff: connectionDiff }],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.designApp.updateConnections": (context: DesignAppCommandContext, updates: { connection: ConnectionId; diff: ConnectionDiff }[]): DesignAppCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: {
                connections: {
                  updated: updates,
                },
              },
            },
          ],
        },
      },
    };
  },
};

designAppCommands = commands;

export const inverseDesignAppSelectionDiff = (selection: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelectionDiff => {
  const inverseDiff: DesignAppSelectionDiff = {};

  if (diff.pieces) {
    inverseDiff.pieces = {};
    if (diff.pieces.added) {
      inverseDiff.pieces.removed = diff.pieces.added;
    }
    if (diff.pieces.removed) {
      inverseDiff.pieces.added = diff.pieces.removed;
    }
  }

  if (diff.connections) {
    inverseDiff.connections = {};
    if (diff.connections.added) {
      inverseDiff.connections.removed = diff.connections.added;
    }
    if (diff.connections.removed) {
      inverseDiff.connections.added = diff.connections.removed;
    }
  }

  if (diff.port) {
    inverseDiff.port = {
      piece: selection.port?.piece,
      designPiece: selection.port?.designPiece,
      port: selection.port?.port,
    };
  }

  return inverseDiff;
};
export const areSameDesignApp = (designApp: DesignAppId, other: DesignAppId): boolean => designApp.kit === other.kit && designApp.design === other.design;
export const hasSameDesignApp = (designApp: DesignAppId, others: DesignAppId[]): boolean => others.some((other) => areSameDesignApp(designApp, other));

export class DesignAppController extends KitDiffAppStore<DesignAppState, DesignAppDiff, DesignAppSelectionDiff, DesignAppEdit, DesignAppCommandContext, DesignAppCommandResult> {
  private readonly kitGuid: Guid;
  private readonly designGuid: Guid;

  constructor(parent: SketchpadStore, yMap: YDesignApp, transact: (fn: () => void) => void, id: DesignAppId, state?: DesignAppState) {
    super(parent, yMap, transact);

    let kitGuid = yMap.get("kit") as string;
    let designGuid = yMap.get("design") as string;

    if (!kitGuid || !designGuid) {
      const kit = this.parent.kit(id.kit);
      const design = kit.design(id.design);
      kitGuid = kit.guid;
      designGuid = design.guid;
      yMap.set("kit", kitGuid);
      yMap.set("design", designGuid);
    }

    this.kitGuid = kitGuid;
    this.designGuid = designGuid;

    if (!yMap.has("fullscreenWindow")) {
      yMap.set("fullscreenWindow", state?.fullscreenWindow || DesignAppFullscreenWindow.None);
    }

    if (!yMap.has("selection")) {
      const selection = new Y.Map<any>();
      const selectedPieces = new Y.Array<Guid>();
      if (state?.selection?.pieces) {
        selectedPieces.push(state.selection.pieces);
      }
      const selectedConnections = new Y.Array<Guid>();
      if (state?.selection?.connections) {
        selectedConnections.push(state.selection.connections);
      }
      const selectionPort = new Y.Map<any>();
      if (state?.selection?.port) {
        selectionPort.set("piece", state.selection.port.piece);
        selectionPort.set("port", state.selection.port.port);
        if (state.selection.port.designPiece) {
          selectionPort.set("designPiece", state.selection.port.designPiece);
        }
      }
      selection.set("pieces", selectedPieces);
      selection.set("connections", selectedConnections);
      selection.set("port", selectionPort);
      yMap.set("selection", selection);
    }

    if (!yMap.has("isTransactionActive")) {
      yMap.set("isTransactionActive", false);
    }
    if (!yMap.has("presence")) {
      yMap.set("presence", new Y.Map<any>());
    }
    if (!yMap.has("others")) {
      yMap.set("others", new Y.Array<any>());
    }
    if (!yMap.has("diff")) {
      yMap.set("diff", new Y.Map<any>());
    }
    if (!yMap.has("currentTransactionStack")) {
      yMap.set("currentTransactionStack", new Y.Array<any>());
    }
    if (!yMap.has("pastTransactionsStack")) {
      yMap.set("pastTransactionsStack", new Y.Array<any>());
    }
    if (!yMap.has("panelVisibility")) {
      const yPanelVisibility = new Y.Map<boolean>();
      yPanelVisibility.set("toolbar", true);
      yPanelVisibility.set("workbench", false);
      yPanelVisibility.set("details", false);
      yPanelVisibility.set("chat", false);
      yPanelVisibility.set("settings", false);
      yMap.set("panelVisibility", yPanelVisibility as any);
    } else {
      // Ensure toolbar is always visible for existing instances
      const yPanelVisibility = yMap.get("panelVisibility") as Y.Map<boolean>;
      if (yPanelVisibility && yPanelVisibility.get("toolbar") !== true) {
        yPanelVisibility.set("toolbar", true);
      }
    }

    Object.entries(designAppCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenWindow(): DesignAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as DesignAppFullscreenWindow;
  }
  set fullscreenWindow(panel: DesignAppFullscreenWindow) {
    this.yMap.set("fullscreenWindow", panel);
  }
  get activeTool(): ToolKind {
    return (this.yMap.get("activeTool") as ToolKind) ?? ToolKind.SELECTION_NORMAL;
  }
  set activeTool(tool: ToolKind) {
    this.yMap.set("activeTool", tool);
  }
  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: true,
        workbench: false,
        details: false,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? true,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? false,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }
  get selection(): DesignAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: DesignAppSelection = {};

    const pieces = selection.get("pieces") as Y.Array<string>;
    if (pieces && pieces.length > 0) {
      result.pieces = pieces.toArray();
    }

    const connections = selection.get("connections") as Y.Array<string>;
    if (connections && connections.length > 0) {
      result.connections = connections.toArray();
    }

    const port = selection.get("port") as Y.Map<string>;
    if (port) {
      const piece = port.get("piece");
      const designPiece = port.get("designPiece");
      const portId = port.get("port");

      if (piece && portId) {
        result.port = {
          piece: piece,
          designPiece: designPiece,
          port: portId,
        };
      }
    }

    return result;
  }
  get presence(): DesignAppPresence {
    return {
      cursor: {
        u: (this.yMap.get("presenceCursorX") as number) || 0,
        v: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }
  get others(): DesignAppPresenceOther[] {
    return [];
  }
  get diff(): KitDiff {
    return {};
  }
  get hover(): DesignAppHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<any> | undefined;
    if (!hover) return undefined;
    const result: DesignAppHover = {};
    const pieces = hover.get("pieces") as Y.Array<Guid> | undefined;
    const connections = hover.get("connections") as Y.Array<Guid> | undefined;
    const ports = hover.get("ports") as Y.Array<Y.Map<string>> | undefined;
    const types = hover.get("types") as Y.Array<Guid> | undefined;
    const designs = hover.get("designs") as Y.Array<Guid> | undefined;
    if (pieces && pieces.length > 0) result.pieces = pieces.toArray();
    if (connections && connections.length > 0) result.connections = connections.toArray();
    if (ports && ports.length > 0) {
      result.ports = ports.toArray().map((yPort) => ({
        piece: yPort.get("piece") as Guid,
        designPiece: yPort.get("designPiece") as Guid | undefined,
        port: yPort.get("port") as Guid,
      }));
    }
    if (types && types.length > 0) result.types = types.toArray();
    if (designs && designs.length > 0) result.designs = designs.toArray();
    return Object.keys(result).length > 0 ? result : undefined;
  }

  get camera(): Camera | undefined {
    const cameraStr = this.yMap.get("camera") as string | undefined;
    return cameraStr ? JSON.parse(cameraStr) : undefined;
  }

  get diagramCenter(): Coord | undefined {
    const centerStr = this.yMap.get("diagramCenter") as string | undefined;
    return centerStr ? JSON.parse(centerStr) : undefined;
  }

  get diagramScale(): number | undefined {
    return this.yMap.get("diagramScale") as number | undefined;
  }

  get focusedPieceGuid(): Guid | undefined {
    return this.yMap.get("focusedPieceGuid") as Guid | undefined;
  }

  get selectedModelTags(): Record<Guid, string[]> {
    const yTagsMap = this.yMap.get("selectedModelTags") as Y.Map<Y.Array<string>> | undefined;
    if (!yTagsMap) return {};
    const result: Record<Guid, string[]> = {};
    yTagsMap.forEach((yTags, typeGuid) => {
      result[typeGuid] = yTags.toArray();
    });
    return result;
  }

  get windowLayout(): any {
    const layoutStr = this.yMap.get("windowLayout") as string | undefined;
    return layoutStr ? JSON.parse(layoutStr) : undefined;
  }
  set windowLayout(layout: any) {
    if (layout) {
      this.yMap.set("windowLayout", JSON.stringify(layout));
    } else {
      this.yMap.delete("windowLayout");
    }
  }

  kit(): KitStore {
    return this.parent.kit(this.kitGuid);
  }

  design(): DesignStore {
    return this.kit().design(this.designGuid);
  }

  protected getSelection(): DesignAppSelection {
    return this.selection;
  }

  protected hash(state: DesignAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DesignAppState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      activeTool: this.activeTool,
      selection: this.selection,
      hover: this.hover,
      presence: this.presence,
      others: this.others,
      camera: this.camera,
      diagramCenter: this.diagramCenter,
      diagramScale: this.diagramScale,
      focusedPieceGuid: this.focusedPieceGuid,
      currentTransactionStackLength: this.currentTransactionStack.length,
      selectedModelTags: this.selectedModelTags,
      windowLayout: this.windowLayout,
    };
  }

  // PERF: Override to directly access individual fields without rebuilding entire snapshot.
  // This is critical for performance - DesignAppController has 14+ getters, each reading from Y.js.
  // When a hook uses useSyncField("hover"), we only need the hover value, not all 14 fields.
  getFieldSnapshot(key: string): any {
    switch (key) {
      case "fullscreenWindow":
        return this.fullscreenWindow;
      case "panelVisibility":
        return this.panelVisibility;
      case "activeTool":
        return this.activeTool;
      case "selection":
        return this.selection;
      case "hover":
        return this.hover;
      case "presence":
        return this.presence;
      case "others":
        return this.others;
      case "camera":
        return this.camera;
      case "diagramCenter":
        return this.diagramCenter;
      case "diagramScale":
        return this.diagramScale;
      case "focusedPieceGuid":
        return this.focusedPieceGuid;
      case "currentTransactionStackLength":
        return this.currentTransactionStack.length;
      case "selectedModelTags":
        return this.selectedModelTags;
      case "windowLayout":
        return this.windowLayout;
      default:
        // Fall back to full snapshot for unknown fields
        return (this.snapshot() as any)[key];
    }
  }

  protected inverseSelectionDiff(selection: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelectionDiff {
    return inverseDesignAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: DesignAppSelectionDiff) => {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    if (selectionDiff.pieces) {
      let pieces = (selection.get("pieces") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("pieces")) {
        selection.set("pieces", pieces);
      }

      if (selectionDiff.pieces.added) {
        for (const piece of selectionDiff.pieces.added) {
          if (!pieces.toArray().includes(piece)) {
            pieces.push([piece]);
          }
        }
      }
      if (selectionDiff.pieces.removed) {
        for (const piece of selectionDiff.pieces.removed) {
          const index = pieces.toArray().indexOf(piece);
          if (index !== -1) {
            pieces.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.connections) {
      let connections = (selection.get("connections") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("connections")) {
        selection.set("connections", connections);
      }

      if (selectionDiff.connections.added) {
        for (const connectionGuid of selectionDiff.connections.added) {
          if (!connections.toArray().includes(connectionGuid)) {
            connections.push([connectionGuid]);
          }
        }
      }
      if (selectionDiff.connections.removed) {
        for (const connectionGuid of selectionDiff.connections.removed) {
          const index = connections.toArray().indexOf(connectionGuid);
          if (index !== -1) {
            connections.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.port) {
      const portSelection = new Y.Map();
      if (selectionDiff.port.piece !== undefined) {
        portSelection.set("piece", selectionDiff.port.piece);
      }
      if (selectionDiff.port.designPiece !== undefined) {
        portSelection.set("designPiece", selectionDiff.port.designPiece);
      }
      if (selectionDiff.port.port !== undefined) {
        portSelection.set("port", selectionDiff.port.port);
      }
      selection.set("port", portSelection);
    }
  };

  change = (diff: DesignAppDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow) this.fullscreenWindow = diff.fullscreenWindow;
      if (diff.activeTool) this.activeTool = diff.activeTool;
      if (diff.panelVisibility !== undefined) {
        let yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
        if (!yPanelVisibility) {
          yPanelVisibility = new Y.Map<boolean>();
          this.yMap.set("panelVisibility", yPanelVisibility);
        }
        Object.entries(diff.panelVisibility).forEach(([key, value]) => {
          if (value !== undefined) {
            yPanelVisibility.set(key, value);
          }
        });
      }
      if (diff.selection) {
        this.applySelectionDiff(diff.selection);
      }
      if (diff.presence) {
        // Handle presence changes if needed
      }
      if (diff.hover) {
        if (Object.keys(diff.hover).length === 0) {
          this.yMap.delete("hover");
        } else {
          let yHover = this.yMap.get("hover") as Y.Map<any>;
          if (!yHover) {
            yHover = new Y.Map<any>();
            this.yMap.set("hover", yHover);
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "pieces")) {
            const piecesValue = diff.hover.pieces;
            if (piecesValue && piecesValue.length > 0) {
              const yPieces = new Y.Array<Guid>();
              yPieces.push(piecesValue);
              yHover.set("pieces", yPieces);
            } else {
              yHover.delete("pieces");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "connections")) {
            const connectionsValue = diff.hover.connections;
            if (connectionsValue && connectionsValue.length > 0) {
              const yConnections = new Y.Array<Guid>();
              yConnections.push(connectionsValue);
              yHover.set("connections", yConnections);
            } else {
              yHover.delete("connections");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "ports")) {
            const portsValue = diff.hover.ports;
            if (portsValue && portsValue.length > 0) {
              const yPorts = new Y.Array<any>();
              portsValue.forEach((port) => {
                const yPort = new Y.Map<string>();
                yPort.set("piece", port.piece);
                if (port.designPiece) yPort.set("designPiece", port.designPiece);
                yPort.set("port", port.port);
                yPorts.push([yPort]);
              });
              yHover.set("ports", yPorts);
            } else {
              yHover.delete("ports");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "types")) {
            const typesValue = diff.hover.types;
            if (typesValue && typesValue.length > 0) {
              const yTypes = new Y.Array<Guid>();
              yTypes.push(typesValue);
              yHover.set("types", yTypes);
            } else {
              yHover.delete("types");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "designs")) {
            const designsValue = diff.hover.designs;
            if (designsValue && designsValue.length > 0) {
              const yDesigns = new Y.Array<Guid>();
              yDesigns.push(designsValue);
              yHover.set("designs", yDesigns);
            } else {
              yHover.delete("designs");
            }
          }
        }
      }
      if (diff.camera) {
        this.yMap.set("camera", JSON.stringify(diff.camera));
      }
      if (diff.diagramCenter) {
        this.yMap.set("diagramCenter", JSON.stringify(diff.diagramCenter));
      }
      if (diff.diagramScale !== undefined) {
        this.yMap.set("diagramScale", diff.diagramScale);
      }
      if (diff.focusedPieceGuid !== undefined) {
        if (diff.focusedPieceGuid === null) {
          this.yMap.delete("focusedPieceGuid");
        } else {
          this.yMap.set("focusedPieceGuid", diff.focusedPieceGuid);
        }
      }
      if (diff.selectedModelTags !== undefined) {
        let yTagsMap = this.yMap.get("selectedModelTags") as Y.Map<Y.Array<string>>;
        if (!yTagsMap) {
          yTagsMap = new Y.Map<Y.Array<string>>();
          this.yMap.set("selectedModelTags", yTagsMap);
        }
        Object.entries(diff.selectedModelTags).forEach(([typeGuid, tags]) => {
          if (tags.length === 0) {
            yTagsMap.delete(typeGuid);
          } else {
            let yTags = yTagsMap.get(typeGuid) as Y.Array<string>;
            if (!yTags) {
              yTags = new Y.Array<string>();
              yTagsMap.set(typeGuid, yTags);
            }
            yTags.delete(0, yTags.length);
            yTags.push(tags);
          }
        });
      }
      if (diff.windowLayout !== undefined) {
        this.windowLayout = diff.windowLayout;
      }
    });
  };

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    if (command === "semio.designApp.startTransaction") {
      console.group(`[${origin || "unknown"}] Transaction: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.finalizeTransaction") {
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.designApp.abortTransaction") {
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.designApp.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.designApp.redo") {
      this.redo();
      return {} as T;
    }

    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design app store`);

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const designStore = this.design();
    const design = designStore.snapshot();
    const context: DesignAppCommandContext = {
      designApp: state,
      kit: kitState,
      Guid: design.guid,
      design,
      fileUrls: kitStore.fileUrls,
      origin,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }
    this.recordEdit(result);
    console.groupEnd();
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

export function initializeDesignAppController() {
  registerDesignAppControllerFactory((parent: any, yMap: any, transact: any, id: any, state: any) => new DesignAppController(parent, yMap as any, transact, id, state));
}

if (typeof window !== "undefined") {
  setTimeout(() => initializeDesignAppController(), 0);
}

type DesignAppScope = { id: string };
const DesignAppScopeContext = createContext<DesignAppScope | null>(null);

// Design app now uses the unified sketchpadMachine - no separate actor needed
// Keep context for backwards compatibility with components that check for it
const DesignAppActorContext = createContext<any>(null);

// Internal component to run the Y.js → XState sync hook
const DesignAppSyncComponent = ({ children }: { children: React.ReactNode }) => {
  useDesignAppYjsToXStateSyncInternal();
  return <>{children}</>;
};

// Convert Y.js DesignAppState to XState DesignAppState format
function convertToXStateDesignAppState(state: DesignAppState): import("./hooks").DesignAppState {
  return {
    panelVisibility: state.panelVisibility,
    selection: state.selection,
    hover: state.hover,
    focusedPiece: state.focusedPieceGuid,
    selectedModelTags: state.selectedModelTags ?? {},
    diagramCenter: state.diagramCenter ? { x: state.diagramCenter.u, y: state.diagramCenter.v } : undefined,
    diagramScale: state.diagramScale,
    camera: state.camera,
    activeTool: state.activeTool,
    fullscreenWindow: state.fullscreenWindow,
  };
}

// Internal sync hook (used by DesignAppScopeProvider)
function useDesignAppYjsToXStateSyncInternal() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const sketchpadStore = useSketchpadStore();
  const hasInitialized = useRef(false);

  // Initialize XState with Y.js state synchronously (before paint)
  useLayoutEffect(() => {
    if (hasInitialized.current || !kitGuid || !designGuid) return;

    const store = sketchpadStore.designApp(kitGuid, designGuid);
    const initialState = store.snapshot();
    actor.send({
      type: "DESIGN.INIT",
      kitGuid,
      designGuid,
      state: convertToXStateDesignAppState(initialState),
    });
    hasInitialized.current = true;
  }, [actor, sketchpadStore, kitGuid, designGuid]);

  // Continue syncing Y.js changes to XState
  const store = kitGuid && designGuid ? sketchpadStore.designApp(kitGuid, designGuid) : null;
  const state = useSyncDeep<DesignAppState, DesignAppState>(store, (s) => s);

  useEffect(() => {
    if (!state || !kitGuid || !designGuid || !hasInitialized.current) return;

    actor.send({
      type: "DESIGN.SYNC",
      kitGuid,
      designGuid,
      state: convertToXStateDesignAppState(state),
    });
  }, [actor, state, kitGuid, designGuid]);
}

export const DesignAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  // Design app state is now managed by the unified sketchpadMachine
  // The sync component keeps XState in sync with Y.js
  return React.createElement(DesignAppScopeContext.Provider, { value }, React.createElement(DesignAppActorContext.Provider, { value: null }, React.createElement(DesignAppSyncComponent, null, props.children)));
};

const useDesignAppScope = () => useContext(DesignAppScopeContext);

// XState hook to get the Design app actor (deprecated - use useSketchpadActor instead)
export function useDesignAppActor(): any {
  return useContext(DesignAppActorContext);
}

export function useDesignAppController<T>(selector?: (store: DesignAppController) => T, id?: DesignAppId): T | DesignAppController | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedDesignId = designScope?.guid ?? id?.design;
  if (!resolvedKitId || !resolvedDesignId) {
    return null;
  }
  const designAppStore = store.designApp(resolvedKitId, resolvedDesignId);
  return selector ? selector(designAppStore) : designAppStore;
}

/**
 * Get Design app state from XState.
 * This is the new XState-based hook.
 */
export function useDesignApp<T>(selector?: (state: DesignAppState) => T, id?: DesignAppId): T | DesignAppState | null {
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit;
  const designGuid = designScope?.guid ?? id?.design;

  if (!kitGuid || !designGuid) return null;

  const state = useDesignAppXState(kitGuid, designGuid);
  if (selector) {
    return selector(state as unknown as DesignAppState) as T;
  }
  return state as unknown as DesignAppState;
}

// PERF: Stable empty values for hooks
const EMPTY_SELECTION: DesignAppSelection = {};
const EMPTY_OTHERS: DesignAppPresenceOther[] = [];
const EMPTY_MODEL_TAGS: Record<Guid, string[]> = {};
const DEFAULT_PANEL_VISIBILITY: PanelVisibility = { toolbar: false, workbench: false, details: false, chat: false, settings: false };

// Stable selectors for Y.js store reads
const selectSelection = (s: DesignAppState) => s.selection ?? EMPTY_SELECTION;
const selectFullscreen = (s: DesignAppState) => s.fullscreenWindow;
const selectActiveTool = (s: DesignAppState) => s.activeTool ?? ToolKind.SELECTION_NORMAL;
const selectOthers = (s: DesignAppState) => s.others ?? EMPTY_OTHERS;
const selectCamera = (s: DesignAppState) => s.camera;
const selectDiagramCenter = (s: DesignAppState) => s.diagramCenter;
const selectDiagramScale = (s: DesignAppState) => s.diagramScale;
const selectFocusedPiece = (s: DesignAppState) => s.focusedPieceGuid;
const selectModelTags = (s: DesignAppState) => s.selectedModelTags ?? EMPTY_MODEL_TAGS;
const selectHover = (s: DesignAppState) => s.hover;
const selectPanelVisibility = (s: DesignAppState) => s.panelVisibility;

// Hooks read from XState stores
// XState handles state transitions and commands
// All hooks follow the [state, setState, canSetState] pattern

export function useDesignAppSelection(): GranularHookResult<DesignAppSelection> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectSelection(state as DesignAppState) : EMPTY_SELECTION;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (selection: DesignAppSelection) => {
      const current = store.snapshot().selection ?? {};
      const piecesToAdd = selection.pieces?.filter((p) => !current.pieces?.includes(p)) ?? [];
      const piecesToRemove = current.pieces?.filter((p) => !selection.pieces?.includes(p)) ?? [];
      const connectionsToAdd = selection.connections?.filter((c) => !current.connections?.includes(c)) ?? [];
      const connectionsToRemove = current.connections?.filter((c) => !selection.connections?.includes(c)) ?? [];
      store.change({
        selection: {
          pieces: { added: piecesToAdd, removed: piecesToRemove },
          connections: { added: connectionsToAdd, removed: connectionsToRemove },
          port: selection.port,
        },
      });
    };
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppFullscreen(): GranularHookResult<DesignAppFullscreenWindow> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectFullscreen(state as DesignAppState) : DesignAppFullscreenWindow.None;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (fullscreen: DesignAppFullscreenWindow) => store.change({ fullscreenWindow: fullscreen });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppActiveTool(): GranularHookResult<ToolKind> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectActiveTool(state as DesignAppState) : ToolKind.SELECTION_NORMAL;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (tool: ToolKind) => store.change({ activeTool: tool });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppDiff(): GranularHookResult<KitDiff | undefined> {
  return readonlyHookResult(undefined);
}

export function useDesignAppOthers(): GranularHookResult<DesignAppPresenceOther[]> {
  const state = useDesignApp(identitySelector);
  const value = state ? selectOthers(state as DesignAppState) : EMPTY_OTHERS;
  return readonlyHookResult(value);
}

export function useDesignAppCamera(): GranularHookResult<Camera | undefined> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectCamera(state as DesignAppState) : undefined;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (camera: Camera | undefined) => store.change({ camera });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppDiagramCenter(): GranularHookResult<Coord | undefined> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectDiagramCenter(state as DesignAppState) : undefined;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (center: Coord | undefined) => store.change({ diagramCenter: center });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppDiagramScale(): GranularHookResult<number | undefined> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectDiagramScale(state as DesignAppState) : undefined;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (scale: number | undefined) => store.change({ diagramScale: scale });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppFocusedPieceGuid(): GranularHookResult<Guid | undefined> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectFocusedPiece(state as DesignAppState) : undefined;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (guid: Guid | undefined) => store.change({ focusedPieceGuid: guid ?? null });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppSelectedModelTags(): GranularHookResult<Record<Guid, string[]>> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectModelTags(state as DesignAppState) : EMPTY_MODEL_TAGS;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (tags: Record<Guid, string[]>) => store.change({ selectedModelTags: tags });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppHover(): GranularHookResult<DesignAppHover | undefined> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectHover(state as DesignAppState) : undefined;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (hover: DesignAppHover | undefined) => store.change({ hover: hover ?? {} });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

export function useDesignAppPanelVisibility(): GranularHookResult<PanelVisibility> {
  const state = useDesignApp(identitySelector);
  const store = useDesignAppController() as DesignAppController | null;
  const value = state ? selectPanelVisibility(state as DesignAppState) : DEFAULT_PANEL_VISIBILITY;
  const canSet = store !== null;
  const setter = useMemo(() => {
    if (!store) return undefined;
    return (visibility: PanelVisibility) => store.change({ panelVisibility: visibility });
  }, [store]);
  return conditionalHookResult(value, setter, canSet);
}

// PERF: Memoized empty commands object for when store is null
const EMPTY_COMMANDS = {
  togglePanel: () => {},
  execute: () => {},
  startTransaction: () => {},
  finalizeTransaction: () => {},
  abortTransaction: () => {},
  undo: () => {},
  redo: () => {},
  selectAll: () => {},
  deselectAll: () => {},
  selectPiece: () => {},
  selectPieces: () => {},
  addPieceToSelection: () => {},
  removePieceFromSelection: () => {},
  selectConnection: () => {},
  addConnectionToSelection: () => {},
  removeConnectionFromSelection: () => {},
  selectPiecePort: () => {},
  deselectPiecePort: () => {},
  deleteSelected: () => {},
  toggleDiagramFullscreen: () => {},
  toggleAccesslFullscreen: () => {},
  setActiveTool: () => {},
  addPiece: () => {},
  addPieces: () => {},
  removePiece: () => {},
  removePieces: () => {},
  addConnection: () => {},
  addConnections: () => {},
  removeConnection: () => {},
  removeConnections: () => {},
  updatePiece: () => {},
  updatePieces: () => {},
  updateConnection: () => {},
  updateConnections: () => {},
  setCamera: () => {},
  focusPiece: () => {},
  clearFocus: () => {},
  setDiagramCenter: () => {},
  setDiagramScale: () => {},
  hoverPiece: () => {},
  hoverPieces: () => {},
  hoverConnection: () => {},
  hoverConnections: () => {},
  hoverPort: () => {},
  hoverType: () => {},
  hoverTypes: () => {},
  hoverDesign: () => {},
  hoverDesigns: () => {},
  clearHover: () => {},
  setModelTagsForType: () => {},
  addModelTagForAllTypes: () => {},
  removeModelTagFromAllTypes: () => {},
} as any;

export function useDesignAppCommands(id?: DesignAppId) {
  const store = useDesignAppController(undefined, id) as DesignAppController | null;

  // PERF: All commands go through Y.js store for now
  // XState migration for commands requires hooks to also use XState (to be synced)
  return useMemo(() => {
    if (!store) {
      return EMPTY_COMMANDS;
    }
    return {
      startTransaction: (origin: string) => store.execute("semio.designApp.startTransaction", origin),
      finalizeTransaction: (origin: string) => store.execute("semio.designApp.finalizeTransaction", origin),
      abortTransaction: (origin: string) => store.execute("semio.designApp.abortTransaction", origin),
      undo: (origin: string) => store.execute("semio.designApp.undo", origin),
      redo: (origin: string) => store.execute("semio.designApp.redo", origin),
      selectAll: (origin: string) => store.execute("semio.designApp.selectAll", origin),
      deselectAll: (origin: string) => store.execute("semio.designApp.deselectAll", origin),
      selectPiece: (origin: string, guid: Guid) => store.execute("semio.designApp.selectPiece", origin, guid),
      selectPieces: (origin: string, guids: Guid[]) => store.execute("semio.designApp.selectPieces", origin, guids),
      addPieceToSelection: (origin: string, guid: Guid) => store.execute("semio.designApp.addPieceToSelection", origin, guid),
      removePieceFromSelection: (origin: string, guid: Guid) => store.execute("semio.designApp.removePieceFromSelection", origin, guid),
      selectConnection: (origin: string, connectionGuid: Guid) => store.execute("semio.designApp.selectConnection", origin, connectionGuid),
      addConnectionToSelection: (origin: string, connectionGuid: Guid) => store.execute("semio.designApp.addConnectionToSelection", origin, connectionGuid),
      removeConnectionFromSelection: (origin: string, connectionGuid: Guid) => store.execute("semio.designApp.removeConnectionFromSelection", origin, connectionGuid),
      selectPiecePort: (origin: string, piece: Guid, port: Guid) => store.execute("semio.designApp.selectPiecePort", origin, piece, port),
      deselectPiecePort: (origin: string) => store.execute("semio.designApp.deselectPiecePort", origin),
      deleteSelected: (origin: string) => store.execute("semio.designApp.deleteSelected", origin),
      toggleDiagramFullscreen: (origin: string) => store.execute("semio.designApp.toggleDiagramFullscreen", origin),
      toggleAccesslFullscreen: (origin: string) => store.execute("semio.designApp.toggleAccesslFullscreen", origin),
      setActiveTool: (origin: string, tool: ToolKind) => store.execute("semio.designApp.setActiveTool", origin, tool),
      addPiece: (origin: string, piece: Piece) => store.execute("semio.designApp.addPiece", origin, piece),
      addPieces: (origin: string, pieces: Piece[]) => store.execute("semio.designApp.addPieces", origin, pieces),
      removePiece: (origin: string, piece: Guid) => store.execute("semio.designApp.removePiece", origin, piece),
      removePieces: (origin: string, pieces: Guid[]) => store.execute("semio.designApp.removePieces", origin, pieces),
      addConnection: (origin: string, connection: Connection) => store.execute("semio.designApp.addConnection", origin, connection),
      addConnections: (origin: string, connections: Connection[]) => store.execute("semio.designApp.addConnections", origin, connections),
      removeConnection: (origin: string, connection: Guid) => store.execute("semio.designApp.removeConnection", origin, connection),
      removeConnections: (origin: string, connections: Guid[]) => store.execute("semio.designApp.removeConnections", origin, connections),
      updatePiece: (origin: string, piece: Guid, pieceDiff: PieceDiff) => store.execute("semio.designApp.updatePiece", origin, piece, pieceDiff),
      updatePieces: (origin: string, updates: { id: Guid; diff: PieceDiff }[]) => store.execute("semio.designApp.updatePieces", origin, updates),
      updateConnection: (origin: string, connection: Guid, connectionDiff: ConnectionDiff) => store.execute("semio.designApp.updateConnection", origin, connection, connectionDiff),
      updateConnections: (origin: string, updates: { id: Guid; diff: ConnectionDiff }[]) => store.execute("semio.designApp.updateConnections", origin, updates),
      setCamera: (origin: string, camera: Camera) => store.execute("semio.designApp.setCamera", origin, camera),
      focusPiece: (origin: string, pieceGuid: Guid) => store.execute("semio.designApp.focusPiece", origin, pieceGuid),
      clearFocus: (origin: string) => store.execute("semio.designApp.clearFocus", origin),
      setDiagramCenter: (origin: string, center: Coord) => store.execute("semio.designApp.setDiagramCenter", origin, center),
      setDiagramScale: (origin: string, scale: number) => store.execute("semio.designApp.setDiagramScale", origin, scale),
      hoverPiece: (origin: string, guid: Guid) => store.execute("semio.designApp.hoverPiece", origin, guid),
      hoverPieces: (origin: string, guids: Guid[]) => store.execute("semio.designApp.hoverPieces", origin, guids),
      hoverConnection: (origin: string, guid: Guid) => store.execute("semio.designApp.hoverConnection", origin, guid),
      hoverConnections: (origin: string, guids: Guid[]) => store.execute("semio.designApp.hoverConnections", origin, guids),
      hoverPort: (origin: string, pieceGuid: Guid, portGuid: Guid) => store.execute("semio.designApp.hoverPort", origin, pieceGuid, portGuid),
      hoverType: (origin: string, guid: Guid) => store.execute("semio.designApp.hoverType", origin, guid),
      hoverTypes: (origin: string, guids: Guid[]) => store.execute("semio.designApp.hoverTypes", origin, guids),
      hoverDesign: (origin: string, guid: Guid) => store.execute("semio.designApp.hoverDesign", origin, guid),
      hoverDesigns: (origin: string, guids: Guid[]) => store.execute("semio.designApp.hoverDesigns", origin, guids),
      clearHover: (origin: string) => store.execute("semio.designApp.clearHover", origin),
      togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
        const current = store.snapshot().panelVisibility;
        store.change({ panelVisibility: { [panelKey]: !current[panelKey] } });
      },
      setModelTagsForType: (origin: string, typeGuid: Guid, tags: string[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        store.change({ selectedModelTags: { ...current, [typeGuid]: tags } });
      },
      addModelTagForAllTypes: (origin: string, tagGuid: string, typeGuids: Guid[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        const updated: Record<Guid, string[]> = { ...current };
        typeGuids.forEach((typeGuid) => {
          const existing = updated[typeGuid] ?? [];
          if (!existing.includes(tagGuid)) updated[typeGuid] = [...existing, tagGuid];
        });
        store.change({ selectedModelTags: updated });
      },
      removeModelTagFromAllTypes: (origin: string, tagGuid: string, typeGuids: Guid[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        const updated: Record<Guid, string[]> = { ...current };
        typeGuids.forEach((typeGuid) => {
          const existing = updated[typeGuid] ?? [];
          updated[typeGuid] = existing.filter((t) => t !== tagGuid);
        });
        store.change({ selectedModelTags: updated });
      },
      execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
    };
  }, [store]);
}

/**
 * Hook to sync Y.js store state to XState machine.
 * Call this in a component that wraps Design app to ensure XState has the state.
 * This enables XState selectors to work while Y.js remains the source of truth.
 */
export function useDesignAppYjsToXStateSync(id?: DesignAppId) {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";

  // Watch Y.js state and sync to XState
  const state = useDesignApp((s) => s, id);

  useEffect(() => {
    if (!state || !kitGuid || !designGuid) return;

    // Sync Y.js state to XState
    actor.send({
      type: "DESIGN.SYNC",
      kitGuid,
      designGuid,
      state: {
        panelVisibility: state.panelVisibility,
        selection: state.selection,
        hover: state.hover,
        focusedPiece: state.focusedPieceGuid,
        selectedModelTags: state.selectedModelTags ?? {},
        diagramCenter: state.diagramCenter ? { x: state.diagramCenter.u, y: state.diagramCenter.v } : undefined,
        diagramScale: state.diagramScale,
        camera: state.camera,
        activeTool: state.activeTool,
        fullscreenWindow: state.fullscreenWindow,
      },
    });
  }, [actor, state, kitGuid, designGuid]);
}

// PERF: Helper to compute ALL pieces affected by the current transaction at once
// This is called ONCE per transaction change, not 180 times
function getTransactionAffectedPieces(store: DesignAppController | null): { changedPieces: Set<string>; statusMap: Map<string, DiffStatus> } {
  const changedPieces = new Set<string>();
  const statusMap = new Map<string, DiffStatus>();

  if (!store) return { changedPieces, statusMap };
  const currentStack = store.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) return { changedPieces, statusMap };

  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.pieces?.updated) {
          for (const pieceUpdate of designUpdate.diff.pieces.updated) {
            changedPieces.add(pieceUpdate.piece.guid);
            if (!statusMap.has(pieceUpdate.piece.guid)) {
              statusMap.set(pieceUpdate.piece.guid, DiffStatus.Modified);
            }
          }
        }
        if (designUpdate.diff.pieces?.added) {
          for (const piece of designUpdate.diff.pieces.added) {
            changedPieces.add(piece.guid);
            statusMap.set(piece.guid, DiffStatus.Added);
          }
        }
        if (designUpdate.diff.pieces?.removed) {
          for (const removedPieceId of designUpdate.diff.pieces.removed) {
            changedPieces.add(removedPieceId.guid);
            statusMap.set(removedPieceId.guid, DiffStatus.Removed);
          }
        }
      }
    }
  }
  return { changedPieces, statusMap };
}

// PERF: Shared context for transaction-affected pieces
// Instead of 360 individual subscriptions to currentTransactionStack,
// we have ONE subscription that computes all affected pieces at once
interface TransactionPiecesContextValue {
  changedPieces: Set<string>;
  statusMap: Map<string, DiffStatus>;
}

const EMPTY_TRANSACTION_CONTEXT: TransactionPiecesContextValue = {
  changedPieces: new Set(),
  statusMap: new Map(),
};

const TransactionPiecesContext = createContext<TransactionPiecesContextValue>(EMPTY_TRANSACTION_CONTEXT);

/**
 * PERF: Provider that subscribes to transaction changes ONCE and computes affected pieces.
 * All child components can then check membership in O(1) time without individual subscriptions.
 */
export function TransactionPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignAppController(identitySelector) as DesignAppController;

  // Track last computed value to ensure stable references
  const lastValueRef = useRef<TransactionPiecesContextValue>(EMPTY_TRANSACTION_CONTEXT);
  const lastJsonRef = useRef<string>("");

  // PERF: Memoize selector to prevent new function reference on each render
  const selector = useCallback(() => {
    const result = getTransactionAffectedPieces(store);
    // Create stable JSON for comparison (Sets aren't JSON-serializable directly)
    const newJson = JSON.stringify([...result.changedPieces].sort()) + JSON.stringify([...result.statusMap.entries()].sort());
    if (newJson === lastJsonRef.current) {
      return lastValueRef.current;
    }
    lastJsonRef.current = newJson;
    lastValueRef.current = result;
    return result;
  }, [store]);

  // Single subscription to transaction stack that computes all affected pieces
  const transactionData = useSyncField<DesignAppState, TransactionPiecesContextValue>(store, "currentTransactionStack", selector);

  return <TransactionPiecesContext.Provider value={transactionData}>{children}</TransactionPiecesContext.Provider>;
}

/**
 * PERF: Check if a piece is changed in the current transaction using the shared context.
 * O(1) lookup instead of iterating through the transaction stack.
 */
export function useIsDesignPieceChangedInTransaction(id: DesignAppId | undefined, pieceId: string): boolean {
  const { changedPieces } = useContext(TransactionPiecesContext);
  return changedPieces.has(pieceId);
}

export function useDesignAppIsPieceHovered(id: DesignAppId | undefined, pieceId: string): boolean {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Use granular subscription that only re-renders when this specific piece's hover state changes
  // Instead of re-rendering all 180 pieces when any hover changes, only the affected piece re-renders
  return useSyncNestedArrayItemMembership(store, "hover", "pieces", pieceId);
}

// PERF: Shared context for hover-affected pieces
// Instead of 180 individual subscriptions to "hover" that each call expensive design.snapshot(),
// we have ONE subscription that computes all transitively hovered pieces at once
interface HoverPiecesContextValue {
  transitivelyHoveredPieces: Set<string>;
  transitivelyHoveredTypes: Set<string>;
}

const EMPTY_HOVER_CONTEXT: HoverPiecesContextValue = {
  transitivelyHoveredPieces: new Set(),
  transitivelyHoveredTypes: new Set(),
};

const HoverPiecesContext = createContext<HoverPiecesContextValue>(EMPTY_HOVER_CONTEXT);

// PERF: Helper to compute hover data with stable references
function computeHoverData(store: DesignAppController | null, state: DesignAppState): HoverPiecesContextValue {
  const hover = state.hover;
  const transitivelyHoveredPieces = new Set<string>();
  const transitivelyHoveredTypes = new Set<string>();

  if (!hover) return { transitivelyHoveredPieces, transitivelyHoveredTypes };

  // Add directly hovered pieces
  hover.pieces?.forEach((pieceId) => transitivelyHoveredPieces.add(pieceId));

  // Add directly hovered types
  hover.types?.forEach((typeId) => transitivelyHoveredTypes.add(typeId));

  // For transitive hover, we need to find pieces that have hovered types/designs
  // Only do this expensive lookup if there are hovered types or designs
  if (store && (hover.types?.length || hover.designs?.length)) {
    const design = store.design().snapshot();
    design?.pieces?.forEach((piece) => {
      if (piece.type && hover.types?.includes(piece.type.guid)) {
        transitivelyHoveredPieces.add(piece.guid);
      }
      if (piece.design && hover.designs?.includes(piece.design.guid)) {
        transitivelyHoveredPieces.add(piece.guid);
      }
    });
  }

  // Also mark types as transitively hovered if their pieces are hovered
  if (store && hover.pieces?.length) {
    const design = store.design().snapshot();
    hover.pieces.forEach((pieceId) => {
      const piece = design?.pieces?.find((p) => p.guid === pieceId);
      if (piece?.type?.guid) {
        transitivelyHoveredTypes.add(piece.type.guid);
      }
    });
  }

  return { transitivelyHoveredPieces, transitivelyHoveredTypes };
}

/**
 * PERF: Provider that subscribes to hover changes ONCE and computes transitively hovered pieces.
 * This avoids 180 individual subscriptions each calling store.design().snapshot().
 */
export function HoverPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignAppController(identitySelector) as DesignAppController;

  // Track last computed value to ensure stable references
  const lastValueRef = useRef<HoverPiecesContextValue>(EMPTY_HOVER_CONTEXT);
  const lastJsonRef = useRef<string>("");

  // PERF: Memoize selector to prevent new function reference on each render
  const selector = useCallback(
    (state: DesignAppState) => {
      const result = computeHoverData(store, state);
      // Create stable JSON for comparison
      const newJson = JSON.stringify([...result.transitivelyHoveredPieces].sort()) + JSON.stringify([...result.transitivelyHoveredTypes].sort());
      if (newJson === lastJsonRef.current) {
        return lastValueRef.current;
      }
      lastJsonRef.current = newJson;
      lastValueRef.current = result;
      return result;
    },
    [store],
  );

  // Single subscription to hover that computes all transitively hovered pieces
  const hoverData = useSyncField<DesignAppState, HoverPiecesContextValue>(store, "hover", selector);

  return <HoverPiecesContext.Provider value={hoverData}>{children}</HoverPiecesContext.Provider>;
}

/**
 * PERF: Check if a piece is transitively hovered using the shared context.
 * O(1) lookup instead of running expensive selector per piece.
 */
export function useDesignAppIsPieceTransitiveHovered(id: DesignAppId | undefined, pieceId: string): boolean {
  const { transitivelyHoveredPieces } = useContext(HoverPiecesContext);
  return transitivelyHoveredPieces.has(pieceId);
}

/**
 * PERF: Check if a type is transitively hovered using the shared context.
 * O(1) lookup instead of running expensive selector per type.
 */
export function useDesignAppIsTypeTransitiveHovered(id: DesignAppId | undefined, typeId: string): boolean {
  const { transitivelyHoveredTypes } = useContext(HoverPiecesContext);
  return transitivelyHoveredTypes.has(typeId);
}

/**
 * PERF: Get piece diff status using the shared context.
 * O(1) lookup instead of iterating through the transaction stack.
 */
export function useDesignAppPieceStatus(id: DesignAppId | undefined, pieceId: string): DiffStatus {
  const { statusMap } = useContext(TransactionPiecesContext);
  return statusMap.get(pieceId) ?? DiffStatus.Unchanged;
}

export function useDesignAppIsPieceSelected(id: DesignAppId | undefined, pieceId: string): boolean {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Use granular subscription that only re-renders when this specific piece's selection state changes
  return useSyncSelectionItemMembership(store, "pieces", pieceId);
}

export function useDesignAppPieceColor(id: DesignAppId | undefined, pieceId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignAppIsPieceSelected(id, pieceId);
  const isHovered = useDesignAppIsPieceTransitiveHovered(id, pieceId);
  const status = useDesignAppPieceStatus(id, pieceId);
  const isChangedInTransaction = useIsDesignPieceChangedInTransaction(id, pieceId) as boolean;

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  if (status === DiffStatus.Added) {
    fill = "var(--color-success)";
    stroke = "var(--color-success)";
  } else if (status === DiffStatus.Removed) {
    fill = "var(--color-danger)";
    stroke = "var(--color-danger)";
    opacity = 0.2;
  } else if (status === DiffStatus.Modified) {
    fill = "var(--color-warning)";
    stroke = "var(--color-warning)";
  } else if (isChangedInTransaction) {
    fill = "var(--color-changed-base)";
    stroke = "var(--color-changed-base)";
  } else {
    fill = "transparent";
    stroke = "var(--foreground)";
  }

  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  if (isSelected) {
    if (isChangedInTransaction) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Added) {
      fill = "var(--color-selected-added)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Removed) {
      fill = "var(--color-selected-removed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Modified) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else {
      fill = "var(--active-base)";
      stroke = "var(--foreground)";
    }
    opacity = 1;
  }

  return { fill, stroke, opacity };
}

export function useDesignAppIsConnectionHovered(id: DesignAppId | undefined, connectionId: string): boolean {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Use granular subscription that only re-renders when this specific connection's hover state changes
  return useSyncNestedArrayItemMembership(store, "hover", "connections", connectionId);
}

export function useDesignAppIsConnectionSelected(id: DesignAppId | undefined, connectionId: string): boolean {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Use granular subscription that only re-renders when this specific connection's selection state changes
  return useSyncSelectionItemMembership(store, "connections", connectionId);
}

export function useDesignAppIsPortHovered(id: DesignAppId | undefined, pieceId: string, portId: string): boolean {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Memoize selector to prevent new function reference on each render
  const selector = useCallback((state: DesignAppState) => state.hover?.ports?.some((p) => p.piece === pieceId && p.port === portId) ?? false, [pieceId, portId]);
  if (!store) return false;
  return useSyncField<DesignAppState, boolean>(store, "hover", selector);
}

// PERF: Memoized undefined port for stable reference
const EMPTY_PORT: DesignAppSelection["port"] = undefined;

/**
 * PERF: Hook that watches ONLY the port field within selection.
 * Uses a specialized observer that fires only when port changes, not when pieces/connections change.
 * This prevents 180 callbacks from firing when piece selection changes.
 */
export function useDesignAppSelectedPort(id?: DesignAppId): DesignAppSelection["port"] | undefined {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;

  // Track the last port value for stability
  const lastPortRef = useRef<{ value: DesignAppSelection["port"]; json: string }>({ value: undefined, json: "" });

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};

      // Watch only the nested "port" key within "selection"
      // This observer fires only when selection.port changes, not when selection.pieces changes
      const selectionMap = store.yMap.get("selection") as Y.Map<any> | undefined;
      if (!selectionMap) return () => {};

      let lastPortJson = JSON.stringify(selectionMap.get("port")?.toJSON?.() ?? selectionMap.get("port") ?? null);

      const handler = (event: Y.YMapEvent<any>) => {
        // Only trigger if the "port" key specifically changed
        if (event.keysChanged.has("port")) {
          const newPortJson = JSON.stringify(selectionMap.get("port")?.toJSON?.() ?? selectionMap.get("port") ?? null);
          if (newPortJson !== lastPortJson) {
            lastPortJson = newPortJson;
            callback();
          }
        }
      };

      selectionMap.observe(handler);
      return () => selectionMap.unobserve(handler);
    },
    [store],
  );

  const getSnapshot = useCallback(() => {
    if (!store) return EMPTY_PORT;
    const selection = store.yMap.get("selection") as Y.Map<any> | undefined;
    if (!selection) return EMPTY_PORT;
    const portMap = selection.get("port") as Y.Map<string> | undefined;
    if (!portMap) return EMPTY_PORT;

    const piece = portMap.get("piece") as string | undefined;
    const port = portMap.get("port") as string | undefined;
    const designPiece = portMap.get("designPiece") as string | undefined;

    if (!piece || !port) return EMPTY_PORT;

    const newValue = { piece, port, designPiece };
    const newJson = JSON.stringify(newValue);

    // Return stable reference if unchanged
    if (newJson === lastPortRef.current.json) {
      return lastPortRef.current.value;
    }

    lastPortRef.current = { value: newValue, json: newJson };
    return newValue;
  }, [store]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

/**
 * PERF: Granular hook to check if a specific piece's port is selected.
 * Only re-renders when this specific piece's port selection changes.
 */
export function useDesignAppIsPiecePortSelected(pieceId: string, portId?: string): boolean {
  const store = useDesignAppController(identitySelector) as DesignAppController;
  // PERF: Memoize selector to prevent new function reference on each render
  const selector = useCallback((state: DesignAppState) => state.selection?.port?.piece === pieceId && state.selection?.port?.port === portId, [pieceId, portId]);
  if (!store || !portId) return false;
  return useSyncField<DesignAppState, boolean>(store, "selection", selector, false);
}

// PERF: Helper to get connection status from transaction stack
function getConnectionStatusFromTransactionStack(store: DesignAppController | null, connectionId: string): DiffStatus {
  if (!store) return DiffStatus.Unchanged;
  const currentStack = store.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) return DiffStatus.Unchanged;

  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.connections?.added) {
          for (const conn of designUpdate.diff.connections.added) {
            if (conn.guid === connectionId) return DiffStatus.Added;
          }
        }
        if (designUpdate.diff.connections?.removed) {
          for (const removedConn of designUpdate.diff.connections.removed) {
            if (removedConn.guid === connectionId) return DiffStatus.Removed;
          }
        }
        if (designUpdate.diff.connections?.updated) {
          for (const connUpdate of designUpdate.diff.connections.updated) {
            if (connUpdate.connection.guid === connectionId) return DiffStatus.Modified;
          }
        }
      }
    }
  }
  return DiffStatus.Unchanged;
}

// PERF: Use useSyncField for granular subscription to transaction stack changes only
export function useDesignAppConnectionStatus(id: DesignAppId | undefined, connectionId: string): DiffStatus {
  const store = useDesignAppController(identitySelector, id) as DesignAppController;
  // PERF: Memoize selector to prevent new function reference on each render
  const selector = useCallback(() => getConnectionStatusFromTransactionStack(store, connectionId), [store, connectionId]);
  return useSyncField<DesignAppState, DiffStatus>(store, "currentTransactionStack", selector);
}

export function useDesignAppConnectionColor(id: DesignAppId | undefined, connectionId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignAppIsConnectionSelected(id, connectionId);
  const isHovered = useDesignAppIsConnectionHovered(id, connectionId);
  const status = useDesignAppConnectionStatus(id, connectionId);

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  if (status === DiffStatus.Added) {
    fill = "var(--color-success)";
    stroke = "var(--color-success)";
  } else if (status === DiffStatus.Removed) {
    fill = "var(--color-danger)";
    stroke = "var(--color-danger)";
    opacity = 0.2;
  } else if (status === DiffStatus.Modified) {
    fill = "var(--color-warning)";
    stroke = "var(--color-warning)";
  }

  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  if (isSelected) {
    if (status === DiffStatus.Added) {
      fill = "var(--color-selected-added)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Removed) {
      fill = "var(--color-selected-removed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Modified) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else {
      fill = "var(--active-base)";
      stroke = "var(--foreground)";
    }
    opacity = 1;
  }

  return { fill, stroke, opacity };
}

export function useDesignAppPieceCenter(id?: DesignAppId, pieceId?: Guid): Coord | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.center : undefined;
}

export function useDesignAppPiecePlane(id?: DesignAppId, pieceId?: Guid): Plane | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.plane : undefined;
}

// #endregion Store

// #region Footer

export const DesignAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const design = useDesign() as Design | undefined;
  // Use targeted hooks instead of full kit to avoid unnecessary re-renders
  const types = useKitTypes();
  const tags = useKitTags();
  const selectedModelTags = useDesignAppSelectedModelTags();
  const { addModelTagForAllTypes, removeModelTagForAllTypes } = useDesignAppCommands();

  // Get all unique type guids from the design's pieces
  const designTypeGuids = useMemo(() => {
    if (!design?.pieces) return [];
    const typeGuids = new Set<string>();
    design.pieces.forEach((piece) => {
      if (piece.type?.guid) {
        typeGuids.add(piece.type.guid);
      }
    });
    return Array.from(typeGuids);
  }, [design?.pieces]);

  // Get all unique tag guids from all types in the design
  const allModelTagGuids = useMemo(() => {
    if (!types || designTypeGuids.length === 0) return [];
    const tagGuids = new Set<string>();
    designTypeGuids.forEach((typeGuid) => {
      const type = types.find((t) => t.guid === typeGuid);
      type?.models?.forEach((model) => {
        model.tags?.forEach((tag) => tagGuids.add(tag.guid));
      });
    });
    return Array.from(tagGuids);
  }, [types, designTypeGuids]);

  // Get tag names from kit
  const tagNameMap = useMemo(() => {
    const map = new Map<string, string>();
    tags.forEach((tag) => {
      map.set(tag.guid, tag.name);
    });
    return map;
  }, [tags]);

  // Store refs for callbacks to avoid recreating them in useEffect
  const typesRef = useRef(types);
  const designTypeGuidsRef = useRef(designTypeGuids);
  const selectedModelTagsRef = useRef(selectedModelTags);
  const addModelTagForAllTypesRef = useRef(addModelTagForAllTypes);
  const removeModelTagForAllTypesRef = useRef(removeModelTagForAllTypes);

  useEffect(() => {
    typesRef.current = types;
    designTypeGuidsRef.current = designTypeGuids;
    selectedModelTagsRef.current = selectedModelTags;
    addModelTagForAllTypesRef.current = addModelTagForAllTypes;
    removeModelTagForAllTypesRef.current = removeModelTagForAllTypes;
  }, [types, designTypeGuids, selectedModelTags, addModelTagForAllTypes, removeModelTagForAllTypes]);

  useEffect(() => {
    if (appType !== "design") return;

    // Helper functions using refs to avoid dependency issues
    const isTagSelected = (tagGuid: string): boolean => {
      return designTypeGuidsRef.current.some((typeGuid) => {
        const tags = selectedModelTagsRef.current[typeGuid] ?? [];
        return tags.includes(tagGuid);
      });
    };

    const getTypesWithTag = (tagGuid: string): Guid[] => {
      const currentTypes = typesRef.current;
      if (!currentTypes || currentTypes.length === 0) return [];
      return designTypeGuidsRef.current.filter((typeGuid) => {
        const type = currentTypes.find((t) => t.guid === typeGuid);
        return type?.models?.some((model) => model.tags?.some((tag) => tag.guid === tagGuid));
      });
    };

    // Remove previous tag items
    allModelTagGuids.forEach((tagGuid) => {
      removeFooterItem(`semio.sketchpad.app.design.footer.tag.${tagGuid}`);
    });

    // Add footer items for each tag
    allModelTagGuids.forEach((tagGuid, index) => {
      const tagName = tagNameMap.get(tagGuid) || tagGuid.slice(0, 8);
      const selected = isTagSelected(tagGuid);
      const typesWithTag = getTypesWithTag(tagGuid);

      addFooterItem({
        id: `semio.sketchpad.app.design.footer.tag.${tagGuid}`,
        content: <span className={`cursor-pointer transition-colors ${selected ? "text-foreground font-medium" : "text-muted-foreground hover:text-foreground"}`}>{tagName}</span>,
        onClick: () => {
          // Use refs in onClick to get current values at click time
          const currentSelected = isTagSelected(tagGuid);
          const currentTypesWithTag = getTypesWithTag(tagGuid);
          if (currentSelected) {
            removeModelTagForAllTypesRef.current("semio.sketchpad.app.design.footer.tag.remove", tagGuid, currentTypesWithTag);
          } else {
            addModelTagForAllTypesRef.current("semio.sketchpad.app.design.footer.tag.add", tagGuid, currentTypesWithTag);
          }
        },
        order: index,
      });
    });

    return () => {
      allModelTagGuids.forEach((tagGuid) => {
        removeFooterItem(`semio.sketchpad.app.design.footer.tag.${tagGuid}`);
      });
    };
    // Note: Intentionally excluding selectedModelTags, addModelTagForAllTypes, removeModelTagForAllTypes from deps
    // because they change on every render. We use refs to access current values.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [appType, addFooterItem, removeFooterItem, allModelTagGuids, tagNameMap]);

  return null;
};

// #endregion Footer

// #region Tools

export const SelectionNormalTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_NORMAL,
  icon: <SelectToolIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const SelectionAdditiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_ADDITIVE,
  icon: <AddIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const SelectionSubtractiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_SUBTRACTIVE,
  icon: <RemoveIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const LassoRectangularTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_RECTANGULAR,
  icon: <DiagramIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const LassoFreeformTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_FREEFORM,
  icon: <SceneIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

export const DesignAppTools: Tool<DesignAppState>[] = [SelectionNormalTool, SelectionAdditiveTool, SelectionSubtractiveTool, LassoRectangularTool, LassoFreeformTool];

const getDesignTools = (): ToolDefinition[] => [
  {
    id: "selection",
    defaultMode: ToolKind.SELECTION_NORMAL,
    modes: DesignAppTools.filter((tool) => tool.id.startsWith("selection")).map((tool) => ({
      id: tool.id,
      icon: tool.icon,
    })),
  },
  {
    id: "lasso",
    defaultMode: ToolKind.LASSO_RECTANGULAR,
    modes: DesignAppTools.filter((tool) => tool.id.startsWith("lasso")).map((tool) => ({
      id: tool.id,
      icon: tool.icon,
    })),
  },
];

export const ToolsToggleGroup: FC = () => {
  const { kit, design } = useParams();
  // PERF: Only subscribe to activeTool field, not entire app state
  const [activeTool, , canSetActiveTool] = useDesignAppActiveTool();
  const { setActiveTool } = useDesignAppCommands(kit && design ? { kit, design } : undefined);

  if (!kit || !design) return null;

  return <ToolGroup tools={getDesignTools()} activeTool={activeTool} onToolChange={(tool) => setActiveTool("toolbar", tool as ToolKind)} level="panel" />;
};

// #endregion Tools

// #region Panels

// #region WindowLibrary

interface WindowTemplate {
  id: string;
  label: string;
  icon: React.ReactNode;
  windowTypeId: string;
  componentProps?: any;
}

const windowTemplates: WindowTemplate[] = [
  {
    id: "scene-perspective",
    label: "Perspective View",
    icon: <SceneIcon size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "perspective" },
  },
  {
    id: "scene-top",
    label: "Top View",
    icon: <SceneIcon size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "top" },
  },
  {
    id: "scene-bottom",
    label: "Bottom View",
    icon: <SceneIcon size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "bottom" },
  },
  {
    id: "scene-left",
    label: "Left View",
    icon: <SceneIcon size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "left" },
  },
  {
    id: "scene-right",
    label: "Right View",
    icon: <SceneIcon size={16} />,
    windowTypeId: "scene",
    componentProps: { cameraMode: "orthographic", viewDirection: "right" },
  },
  {
    id: "diagram-full",
    label: "Full Diagram",
    icon: <ConnectionIcon size={16} />,
    windowTypeId: "diagram",
    componentProps: { graphType: "full" },
  },
  {
    id: "diagram-subgraph",
    label: "Subgraph",
    icon: <DiagramIcon size={16} />,
    windowTypeId: "diagram",
    componentProps: { graphType: "subgraph" },
  },
  {
    id: "table-pieces",
    label: "Pieces Table",
    icon: <TableViewIcon size={16} />,
    windowTypeId: "table",
    componentProps: { dataType: "pieces" },
  },
  {
    id: "table-connections",
    label: "Connections Table",
    icon: <TableViewIcon size={16} />,
    windowTypeId: "table",
    componentProps: { dataType: "connections" },
  },
];

interface DraggableWindowItemProps {
  template: WindowTemplate;
}

const DraggableWindowItem: FC<DraggableWindowItemProps> = ({ template }) => {
  const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
    id: template.id,
    data: {
      type: "window-template",
      windowTypeId: template.windowTypeId,
      componentProps: template.componentProps,
    },
  });

  return (
    <div ref={setNodeRef} {...listeners} {...attributes} className={`cursor-grab active:cursor-grabbing ${isDragging ? "opacity-50" : ""}`}>
      <TreeItem>
        <TreeContent>
          <div className="flex items-center gap-single">
            {template.icon}
            <span className="text-sm">{template.label}</span>
          </div>
        </TreeContent>
      </TreeItem>
    </div>
  );
};

export const WindowLibrary: FC = () => {
  const sceneTemplates = windowTemplates.filter((t) => t.windowTypeId === "scene");
  const diagramTemplates = windowTemplates.filter((t) => t.windowTypeId === "diagram");
  const tableTemplates = windowTemplates.filter((t) => t.windowTypeId === "table");

  return (
    <div>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.scene" defaultOpen={true}>
        {sceneTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.diagram" defaultOpen={true}>
        {diagramTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
      <TreeSection id="semio.sketchpad.app.design.windowLibrary.table" defaultOpen={false}>
        {tableTemplates.map((template) => (
          <DraggableWindowItem key={template.id} template={template} />
        ))}
      </TreeSection>
    </div>
  );
};

// #endregion WindowLibrary

// #region Details

export const DesignSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <DesignSectionForm />;
};

const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  // Removed unused useKit() call that was causing unnecessary re-renders
  const kitCommands = useKitCommands();
  const design = useDesign() as Design;

  if (!design) return null;

  const updateDesignField = (origin: string, diff: any) => {
    if (!kitCommands) return;
    kitCommands.updateDesign(origin, design.guid, diff);
  };

  const handleChange = (origin: string, updatedDesign: any) => {
    if (!kitCommands) return;
    kitCommands.updateDesign(origin, design.guid, updatedDesign);
  };

  const addLocation = () => {
    startTransaction("semio.sketchpad.app.design.panel.details.location.add");
    updateDesignField("semio.sketchpad.app.design.panel.details.location.add", { location: { guid: guid(), longitude: 0, latitude: 0 } });
    finalizeTransaction("semio.sketchpad.app.design.panel.details.location.add");
  };

  const removeLocation = () => {
    startTransaction("semio.sketchpad.app.design.panel.details.location.remove");
    updateDesignField("semio.sketchpad.app.design.panel.details.location.remove", { location: undefined });
    finalizeTransaction("semio.sketchpad.app.design.panel.details.location.remove");
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.name"
            value={design.name}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.name", { name: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.name"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.description"
            value={design.description || ""}
            placeholderId="semio.sketchpad.app.design.descriptionPlaceholder"
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.description", { description: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.description"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.icon"
            value={design.icon || ""}
            placeholderId="semio.sketchpad.app.design.iconPlaceholder"
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.icon", { icon: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.icon"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.image"
            value={design.image || ""}
            placeholderId="semio.sketchpad.app.design.imagePlaceholder"
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.image", { image: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.image"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.variant"
            value={(design as any).variant || ""}
            placeholderId="semio.sketchpad.app.design.variantPlaceholder"
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.variant", { variant: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.variant"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.view"
            value={(design as any).view || ""}
            placeholderId="semio.sketchpad.app.design.viewPlaceholder"
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.view", { view: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.view"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.unit"
            value={design.unit || ""}
            onLazyChange={(value) => updateDesignField("semio.sketchpad.app.design.panel.details.section.design.unit", { unit: value })}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.design.unit"),
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      {design.location ? (
        <TreeItem
          id="semio.sketchpad.app.design.location"
          actions={[
            {
              icon: <RemoveIcon />,
              onClick: removeLocation,
              id: "semio.sketchpad.common.remove",
            },
          ]}
        >
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.location.longitude"
                value={(design.location as any)?.longitude ?? 0}
                onChange={(value: number) =>
                  handleChange("semio.sketchpad.app.design.panel.details.section.location.longitude", {
                    ...design,
                    location: { ...(design.location as any)!, longitude: value },
                  })
                }
                transaction={{
                  start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude"),
                  finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude"),
                  abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.location.longitude"),
                }}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.location.latitude"
                value={(design.location as any)?.latitude ?? 0}
                onChange={(value: number) =>
                  handleChange("semio.sketchpad.app.design.panel.details.section.location.latitude", {
                    ...design,
                    location: { ...(design.location as any)!, latitude: value },
                  })
                }
                transaction={{
                  start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude"),
                  finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude"),
                  abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.location.latitude"),
                }}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      ) : (
        <TreeItem
          id="semio.sketchpad.app.design.location"
          actions={[
            {
              icon: <AddIcon />,
              onClick: addLocation,
              id: "semio.sketchpad.common.add",
            },
          ]}
        />
      )}
      <TreeItem
        id="semio.sketchpad.app.design.authors"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.design.panel.details.authors.add";
              startTransaction(origin);
              updateDesignField(origin, {
                authors: [...(design.authors || []), { name: "", email: "" }],
              });
              finalizeTransaction(origin);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {design.authors && design.authors.length > 0 && (
          <SortableTreeItems
            items={(design.authors || []).map((author: any, index: number) => ({
              ...author,
              id: `author-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.design.panel.details.authors.reorder";
              startTransaction(origin);
              updateDesignField(origin, {
                authors: arrayMove(design.authors!, oldIndex, newIndex),
              });
              finalizeTransaction(origin);
            }}
          >
            {(author, index) => (
              <TreeItem
                key={`author-${index}`}
                label={author.name || `${useLabel("semio.sketchpad.app.design.author")} ${index + 1}`}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <RemoveIcon />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.design.panel.details.authors.remove";
                      startTransaction(origin);
                      updateDesignField(origin, {
                        authors: design.authors?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.authors.name"
                      value={author.name}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          name: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.authors.name", { authors: updatedAuthors });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.name")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.authors.email"
                      value={author.email}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          email: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.authors.email", { authors: updatedAuthors });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.email")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.authors.email")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      <TreeItem
        id="semio.sketchpad.app.design.attributes"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.design.panel.details.attributes.add";
              startTransaction(origin);
              updateDesignField(origin, {
                attributes: [...(design.attributes || []), { key: "" }],
              });
              finalizeTransaction(origin);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {design.attributes && design.attributes.length > 0 && (
          <SortableTreeItems
            items={(design.attributes || []).map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.design.panel.details.attributes.reorder";
              startTransaction(origin);
              updateDesignField(origin, {
                attributes: arrayMove(design.attributes!, oldIndex, newIndex),
              });
              finalizeTransaction(origin);
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                label={attribute.key || `${useLabel("semio.sketchpad.app.design.attribute")} ${index + 1}`}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <RemoveIcon />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.design.panel.details.attributes.remove";
                      startTransaction(origin);
                      updateDesignField(origin, {
                        attributes: design.attributes?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.name"
                      value={attribute.key}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          key: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.name", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.name")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.value"
                      value={attribute.value || ""}
                      placeholderId="semio.sketchpad.app.design.attributeValuePlaceholder"
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          value: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.value", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.value")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.value")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.unit"
                      value={attribute.unit || ""}
                      placeholderId="semio.sketchpad.app.design.attributeUnitPlaceholder"
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          unit: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.unit", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.unit")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.unit")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.design.panel.details.section.attributes.definition"
                      value={attribute.definition || ""}
                      placeholderId="semio.sketchpad.app.design.attributeDefinitionPlaceholder"
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          definition: e.target.value,
                        };
                        updateDesignField("semio.sketchpad.app.design.panel.details.section.attributes.definition", { attributes: updatedAttributes });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.definition")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.attributes.definition")}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      {design.createdAt && (
        <TreeItem>
          <TreeContent>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.createdAt"
              value={(() => {
                const date = design.createdAt;
                if (typeof date === "string") return date.split("T")[0];
                if (date && typeof (date as any).toISOString === "function") return (date as any).toISOString().split("T")[0];
                return "";
              })()}
              disabled
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
      {design.updatedAt && (
        <TreeItem>
          <TreeContent>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.updatedAt"
              value={(() => {
                const date = design.updatedAt;
                if (typeof date === "string") return date.split("T")[0];
                if (date && typeof (date as any).toISOString === "function") return (date as any).toISOString().split("T")[0];
                return "";
              })()}
              disabled
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

export const PiecesSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PiecesSectionForm />;
};

const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction, updatePiece, updatePieces } = useDesignAppCommands();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;
  const metadata = new Map();
  const [selection] = useDesignAppSelection();
  const pieces = usePiecesFromIds(selection.pieces || []);

  const isSingle = pieces.length === 1;
  const piece = isSingle ? pieces[0] : null;

  const isDesignPiece = isSingle ? typeof piece?.type === "string" && piece?.type === "design" : pieces.every((p) => typeof p.type === "string" && p.type === "design");
  const hasDesignPieces = pieces.some((p) => typeof p.type === "string" && p.type === "design");
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => typeof p.type === "string" && p.type !== "design");

  const getCommonValue = <T,>(getter: (piece: any) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const getPieceId = (p: any): string => (p as any).guid || (p as any).id_;

  const handleTypeNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleTypeVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignViewChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const fixPieces = async () => {
    // TODO: Implement using execute command
  };

  const handleCenterXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.center.x";
    if (isSingle && piece) {
      updatePiece(origin, getPieceId(piece), { center: { u: value, v: piece.center?.v ?? 0 } });
    } else {
      const updates = pieces.map((p) => ({ id: getPieceId(p), diff: { center: { u: value, v: p.center?.v ?? 0 } } }));
      updatePieces(origin, updates);
    }
  };

  const handleCenterYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.center.y";
    if (isSingle && piece) {
      updatePiece(origin, getPieceId(piece), { center: { u: piece.center?.u ?? 0, v: value } });
    } else {
      const updates = pieces.map((p) => ({ id: getPieceId(p), diff: { center: { u: p.center?.u ?? 0, v: value } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneOriginZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneXAxisZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisXChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, x: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisYChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, y: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const handlePlaneYAxisZChange = (value: number) => {
    const origin = "semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z";
    if (isSingle && piece && piece.plane) {
      updatePiece(origin, getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, z: value } } } }));
      updatePieces(origin, updates);
    }
  };

  const commonTypeName = getCommonValue((p) => {
    const type = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : null;
    return type?.name;
  });
  const commonTypeVariant = getCommonValue((p) => {
    const type = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : null;
    return (type as any)?.variant;
  });
  const commonCenterX = getCommonValue((p) => p.center?.u);
  const commonCenterY = getCommonValue((p) => p.center?.v);
  const commonPlaneOriginX = getCommonValue((p) => p.plane?.origin.x);
  const commonPlaneOriginY = getCommonValue((p) => p.plane?.origin.y);
  const commonPlaneOriginZ = getCommonValue((p) => p.plane?.origin.z);
  const commonPlaneXAxisX = getCommonValue((p) => p.plane?.xAxis.x);
  const commonPlaneXAxisY = getCommonValue((p) => p.plane?.xAxis.y);
  const commonPlaneXAxisZ = getCommonValue((p) => p.plane?.xAxis.z);
  const commonPlaneYAxisX = getCommonValue((p) => p.plane?.yAxis.x);
  const commonPlaneYAxisY = getCommonValue((p) => p.plane?.yAxis.y);
  const commonPlaneYAxisZ = getCommonValue((p) => p.plane?.yAxis.z);

  const hasCenter = pieces.every((p) => p.center);
  const hasPlane = pieces.every((p) => p.plane);
  const hasVariant = pieces.some((p) => {
    const type = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : null;
    return (type as any)?.variant;
  });
  const hasUnfixedPieces = pieces.some((p) => !p.plane || !p.center);

  const pieceIds = useMemo(() => pieces.map((p) => getPieceId(p)), [pieces]);

  const selectedVariants = useMemo(
    () => [
      ...new Set(
        pieces
          .map((p) => {
            const type = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : null;
            return (type as any)?.variant;
          })
          .filter((v): v is string => Boolean(v)),
      ),
    ],
    [pieces, kit],
  );
  const availableTypes = useReplacableTypes(pieceIds, isDesignPiece ? [] : selectedVariants);
  const availableTypeNames = useMemo(() => [...new Set(availableTypes.map((t) => t.name))], [availableTypes]);
  const allReplacableTypes = useReplacableTypes(pieceIds, []);
  const availableVariants = useMemo(
    () =>
      commonTypeName && !isDesignPiece
        ? [
            ...new Set(
              allReplacableTypes
                .filter((t) => t.name === commonTypeName)
                .map((t) => (t as any).variant)
                .filter((v): v is string => Boolean(v)),
            ),
          ]
        : [],
    [commonTypeName, isDesignPiece, allReplacableTypes],
  );

  const replacableDesignsRaw = useReplacableDesigns(isSingle && piece && (piece as any).design ? (piece as Piece) : ({} as Piece));
  const availableDesigns = isDesignPiece && isSingle && piece ? replacableDesignsRaw : [];
  const availableDesignNames = useMemo(() => [...new Set(availableDesigns.map((d) => d.name))], [availableDesigns]);

  const pieceType = piece?.type && "guid" in piece.type ? findTypeInKit(kit, piece.type.guid) : null;
  const pieceDesign = piece && (piece as any).design && "guid" in (piece as any).design ? findDesignInKit(kit, (piece as any).design.guid) : null;

  const availableDesignVariants = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name)
            .map((d) => (d as any).variant)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  const availableDesignViews = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name && ((d as any).variant || "") === ((pieceDesign as any).variant || ""))
            .map((d) => (d as any).view)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  let parentConnection: Connection | null = null;
  let parentConnections: Connection[] = [];

  return (
    <>
      {hasMixedTypes ? (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.piece.mixedSelectionMessage")}</p>
          </TreeContent>
        </TreeItem>
      ) : (
        <>
          {isSingle && piece && (
            <TreeItem>
              <TreeContent>
                <Input id="semio.sketchpad.app.design.piece.id" value={getPieceId(piece)} disabled showLabel />
              </TreeContent>
            </TreeItem>
          )}

          {isDesignPiece ? (
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    id="semio.sketchpad.app.design.name"
                    options={availableDesignNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={pieceDesign?.name || pieceType?.name || ""}
                    placeholderId="semio.sketchpad.common.selectDesign"
                    onValueChange={handleDesignNameChange}
                    showLabel
                  />
                </TreeContent>
              </TreeItem>
              {availableDesignVariants.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      id="semio.sketchpad.app.design.variant"
                      options={availableDesignVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={(pieceDesign as any)?.variant || (pieceType as any)?.variant || ""}
                      placeholderId="semio.sketchpad.common.selectVariant"
                      onValueChange={handleDesignVariantChange}
                      allowClear={true}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              )}
              {availableDesignViews.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      id="semio.sketchpad.app.design.view"
                      options={availableDesignViews.map((view) => ({
                        value: view,
                        label: view,
                      }))}
                      value={(pieceDesign as any)?.view || ""}
                      placeholderId="semio.sketchpad.common.selectView"
                      onValueChange={handleDesignViewChange}
                      allowClear={true}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          ) : (
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    id="semio.sketchpad.app.design.piece.type"
                    options={availableTypeNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={isSingle && piece && piece.type && "guid" in piece.type ? findTypeInKit(kit, piece.type.guid)?.name || "" : commonTypeName || ""}
                    placeholder={!isSingle && commonTypeName === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.common.selectType")}
                    onValueChange={handleTypeNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      id="semio.sketchpad.app.type.variant"
                      options={availableVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={isSingle && piece && piece.type && "guid" in piece.type ? (findTypeInKit(kit, piece.type.guid) as any)?.variant || "" : commonTypeVariant || ""}
                      placeholder={!isSingle && commonTypeVariant === undefined ? useLabel("semio.sketchpad.common.mixedValues") : useLabel("semio.sketchpad.common.selectVariant")}
                      onValueChange={handleTypeVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          )}
        </>
      )}
      {hasCenter && (
        <TreeItem id="semio.sketchpad.app.design.piece.center">
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.piece.center.x"
                value={isSingle && piece ? piece.center?.u : commonCenterX}
                onChange={handleCenterXChange}
                transaction={{
                  start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x"),
                  finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x"),
                  abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.x"),
                }}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.piece.center.y"
                value={isSingle && piece ? piece.center?.v : commonCenterY}
                onChange={handleCenterYChange}
                transaction={{
                  start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y"),
                  finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y"),
                  abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.center.y"),
                }}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      )}
      {isSingle && piece && !piece.plane && (
        <TreeItem>
          <TreeContent>
            <div className="flex flex-col gap-single">
              <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.piece.connectedPieceInfo")}</p>
              <Button id="semio.sketchpad.app.design.piece.fixPiece">
                <DisconnectIcon className="size-tiny" />
                {useLabel("semio.sketchpad.app.design.piece.fixPiece")}
              </Button>
            </div>
          </TreeContent>
        </TreeItem>
      )}
      {hasPlane && (
        <TreeItem id="semio.sketchpad.app.design.piece.plane">
          <TreeItem id="semio.sketchpad.app.design.piece.planeOrigin">
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x"
                  value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX}
                  onChange={handlePlaneOriginXChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y"
                  value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY}
                  onChange={handlePlaneOriginYChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z"
                  value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ}
                  onChange={handlePlaneOriginZChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem id="semio.sketchpad.app.design.piece.planeXAxis">
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x"
                  value={isSingle && piece ? piece.plane?.xAxis.x : commonPlaneXAxisX}
                  onChange={handlePlaneXAxisXChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y"
                  value={isSingle && piece ? piece.plane?.xAxis.y : commonPlaneXAxisY}
                  onChange={handlePlaneXAxisYChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z"
                  value={isSingle && piece ? piece.plane?.xAxis.z : commonPlaneXAxisZ}
                  onChange={handlePlaneXAxisZChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem id="semio.sketchpad.app.design.piece.planeYAxis">
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x"
                  value={isSingle && piece ? piece.plane?.yAxis.x : commonPlaneYAxisX}
                  onChange={handlePlaneYAxisXChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y"
                  value={isSingle && piece ? piece.plane?.yAxis.y : commonPlaneYAxisY}
                  onChange={handlePlaneYAxisYChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z"
                  value={isSingle && piece ? piece.plane?.yAxis.z : commonPlaneYAxisZ}
                  onChange={handlePlaneYAxisZChange}
                  transaction={{
                    start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z"),
                    finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z"),
                    abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z"),
                  }}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
        </TreeItem>
      )}
      {(parentConnection || parentConnections.length > 0) && (
        <div style={{ marginTop: "var(--size-tiny)" }}>
          <ConnectionsSection connections={isSingle && parentConnection ? [parentConnection] : parentConnections} isSingle={isSingle} count={parentConnections.length} />
        </div>
      )}
    </>
  );
};

export const ConnectionsSection: FC<{
  connections: any[];
  isSingle: boolean;
  count: number;
}> = ({ connections, isSingle, count }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  const sectionLabel = isSingle ? useLabel("semio.sketchpad.app.design.panel.details.parentConnection") : useLabel("semio.sketchpad.app.design.panel.details.parentConnections");
  return <ConnectionsSectionForm connections={connections} sectionLabel={sectionLabel} />;
};

const SingleConnectionInfo: FC = () => {
  const connection = useConnection() as Connection;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPieceId" value={connection.connecting.piece.guid} disabled showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPortId" value={connection.connecting.port?.guid ?? ""} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {connection.connecting.designPiece && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingDesignPieceId" value={connection.connecting.designPiece?.guid ?? ""} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPieceId" value={connection.connected.piece.guid} disabled showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPortId" value={connection.connected.port?.guid ?? ""} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {connection.connected.designPiece && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedDesignPieceId" value={connection.connected.designPiece?.guid ?? ""} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

const SingleConnectionFields: FC = () => {
  const [gap, setGap] = useConnectionGap();
  const [shift, setShift] = useConnectionShift();
  const [rise, setRise] = useConnectionRise();
  const [rotation, setRotation] = useConnectionRotation();
  const [turn, setTurn] = useConnectionTurn();
  const [tilt, setTilt] = useConnectionTilt();
  const [u, setU] = useConnectionU();
  const [v, setV] = useConnectionV();
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.gap"
            value={gap}
            onChange={setGap!}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.gap"),
            }}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.shift"
            value={shift}
            onChange={setShift!}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.shift"),
            }}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.rise"
            value={rise}
            onChange={setRise!}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rise"),
            }}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.rotation")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.rotation"
              value={[rotation]}
              onValueChange={([value]) => setRotation!(value)}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation"),
                abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.rotation"),
              }}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.turn")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.turn"
              value={[turn]}
              onValueChange={([value]) => setTurn!(value)}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn"),
                abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.turn"),
              }}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.tilt")}</label>
            <Slider
              id="semio.sketchpad.app.design.panel.details.section.connection.tilt"
              value={[tilt]}
              onValueChange={([value]) => setTilt!(value)}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt"),
                abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.tilt"),
              }}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.u"
            value={u}
            onChange={setU!}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.u"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.u"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.u"),
            }}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            id="semio.sketchpad.app.design.panel.details.section.connection.v"
            value={v}
            onChange={setV!}
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.v"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.v"),
              abort: () => abortTransaction?.("semio.sketchpad.app.design.panel.details.section.connection.v"),
            }}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const ConnectionsSectionForm: FC<{
  connections: Connection[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const isSingle = connections.length === 1;
  const connection = isSingle ? connections[0] : null;
  if (isSingle && connection) {
    return (
      <ConnectionScopeProvider guid={connection.guid}>
        <SingleConnectionInfo />
        <SingleConnectionFields />
      </ConnectionScopeProvider>
    );
  }
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.panel.details.section.connection.multipleEditing")}</p>
        </TreeContent>
      </TreeItem>
    </>
  );
};

export const PortSection: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PortSectionForm pieceGuid={pieceGuid} portGuid={portGuid} />;
};

const PortSectionForm: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const { t } = useTranslation();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;

  const piece = (() => {
    try {
      return findPieceInDesign(design, pieceGuid);
    } catch {
      return null;
    }
  })();

  const type = piece?.type ? findTypeInKit(kit, piece.type.guid) : null;
  const port = type?.ports?.find((p) => p.guid === portGuid);

  if (!piece || !type || !port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.panel.details.section.port.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.id" value={port.guid || "~default~"} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {port.description && (
        <TreeItem>
          <TreeContent>
            <Textarea id="semio.sketchpad.app.design.panel.details.section.port.description" value={port.description} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {port.interface && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.port.interface" value={port.interface.guid} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {port.mandatory !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.port.mandatory" value={port.mandatory ? useLabel("semio.sketchpad.common.yes") : useLabel("semio.sketchpad.common.no")} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.port.direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {(port as any).compatibleInterfaces &&
        (port as any).compatibleInterfaces.map((interface_: string, index: number) => (
          <TreeItem key={`compatible-interface-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.port.compatibleInterface" value={interface_} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
      {port.attributes &&
        port.attributes.map((attribute: any, index: number) => (
          <TreeItem key={`port-attribute-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.port.attribute" value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
    </>
  );
};

// #endregion Details

// #endregion Panels

// #region Canvas

let globalHoverClearTimeout: NodeJS.Timeout | null = null;
let currentHoveredPieceGuid: string | null = null;
// PERF: Track panning/dragging state to skip hover updates (prevents cascade re-renders)
let isPanning: boolean = false;
let isDraggingNode: boolean = false;

type SemioConnection = Connection;

// PERF: Pre-computed per-piece rendering data to avoid per-node subscriptions
// This is computed ONCE at DesignDiagram level and passed down as context
interface PieceRenderData {
  isSelected: boolean;
  isHovered: boolean;
  fill: string;
  stroke: string;
  opacity: number;
  isChangedInTransaction: boolean;
  diffStatus: DiffStatus;
}

const EMPTY_PIECE_RENDER_DATA: PieceRenderData = {
  isSelected: false,
  isHovered: false,
  fill: "transparent",
  stroke: "var(--foreground)",
  opacity: 1,
  isChangedInTransaction: false,
  diffStatus: DiffStatus.Unchanged,
};

// Context to provide pre-computed render data for all pieces
const PieceRenderDataContext = createContext<Map<string, PieceRenderData>>(new Map());

// Hook to get render data for a specific piece - O(1) lookup, no subscription
function usePieceRenderData(pieceGuid: string): PieceRenderData {
  const dataMap = useContext(PieceRenderDataContext);
  return dataMap.get(pieceGuid) ?? EMPTY_PIECE_RENDER_DATA;
}

// #region Diagram

type ClusterMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onCluster: (clusterPieceIds: string[]) => void;
};

const ClusterMenu: FC<ClusterMenuProps> = ({ nodes, edges, onCluster }) => {
  const reactFlowInstance = useReactFlow();
  const clusterableGroups = useClusterableGroups();

  const getBoundingBoxForGroup = useCallback(
    (groupPieceIds: string[]) => {
      const groupNodes = nodes.filter((node) => groupPieceIds.includes(node.data.piece.guid));

      if (groupNodes.length === 0) return null;

      let minX = Infinity;
      let minY = Infinity;
      let maxX = -Infinity;
      let maxY = -Infinity;

      groupNodes.forEach((node) => {
        const x = node.position.x;
        const y = node.position.y;
        const width = ICON_WIDTH;
        const height = ICON_WIDTH;

        minX = Math.min(minX, x);
        minY = Math.min(minY, y);
        maxX = Math.max(maxX, x + width);
        maxY = Math.max(maxY, y + height);
      });

      const padding = 20;
      return {
        x: minX - padding,
        y: minY - padding,
        width: maxX - minX + padding * 2,
        height: maxY - minY + padding * 2,
      };
    },
    [nodes],
  );

  if (clusterableGroups.length === 0) {
    return null;
  }

  return (
    <ViewportPortal>
      {clusterableGroups.map((groupPieceIds, groupIndex) => {
        const boundingBox = getBoundingBoxForGroup(groupPieceIds);
        if (!boundingBox) return null;

        return (
          <div
            key={`cluster-group-${groupIndex}`}
            className="absolute pointer-events-none"
            style={{
              left: boundingBox.x,
              top: boundingBox.y,
              width: boundingBox.width,
              height: boundingBox.height,
            }}
          >
            <div className="absolute inset-0 border-2 border-dashed border-accent/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <Button id="semio.sketchpad.app.design.diagram.clusterMenu.cluster" level="temporary" className="px-3 py-single text-sm" onClick={() => onCluster(groupPieceIds)}>
                Cluster
              </Button>
            </div>
          </div>
        );
      })}
    </ViewportPortal>
  );
};

type ExpandMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onExpand: (designId: string) => void;
};

const ExpandMenu: FC<ExpandMenuProps> = ({ nodes, edges, onExpand }) => {
  const [selection] = useDesignAppSelection();
  const kit = useKit() as Kit;
  const explodeableDesignNodes = useExplodeableDesignNodes(nodes, selection);

  const getBoundingBoxForNode = useCallback((node: DiagramNode) => {
    const x = node.position.x;
    const y = node.position.y;
    const width = ICON_WIDTH;
    const height = ICON_WIDTH;

    const padding = 20;
    return {
      x: x - padding,
      y: y - padding,
      width: width + padding * 2,
      height: height + padding * 2,
    };
  }, []);

  if (explodeableDesignNodes.length === 0) {
    return null;
  }

  return (
    <ViewportPortal>
      {explodeableDesignNodes.map((node) => {
        const boundingBox = getBoundingBoxForNode(node);
        const piece = node.data.piece as Piece;
        const type = piece.type ? findTypeInKit(kit, piece.type.guid) : null;
        const designName = type?.name ?? "";

        return (
          <div
            key={`explode-design-${designName}`}
            className="absolute pointer-events-none"
            style={{
              left: boundingBox.x,
              top: boundingBox.y,
              width: boundingBox.width,
              height: boundingBox.height,
            }}
          >
            <div className="absolute inset-0 border-2 border-dashed border-accent/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <Button id="semio.sketchpad.app.design.diagram.expandMenu.expand" level="temporary" className="px-3 py-single text-sm" onClick={() => onExpand(designName)}>
                Expand
              </Button>
            </div>
          </div>
        );
      })}
    </ViewportPortal>
  );
};

const PresenceDiagram: FC<DesignAppPresenceOther> = ({ name, cursor, camera }) => {
  if (!cursor) return null;
  return (
    <ViewportPortal>
      <div
        style={{
          transform: `translate(${cursor.u * ICON_WIDTH}px, ${-cursor.v * ICON_WIDTH}px)`,
          position: "absolute",
          pointerEvents: "none",
          zIndex: 1000,
        }}
      >
        <div className="flex items-center gap-single bg-accent text-accent-foreground px-single py-single rounded-full text-xs">
          <div className="size-dot bg-accent-foreground rounded-full"></div>
          {name}
        </div>
      </div>
    </ViewportPortal>
  );
};

type HelperLine = {
  kind: "horizontal" | "vertical" | "equalDistance";
  position?: number;
  relatedPieceId: string;
  x1?: number;
  y1?: number;
  x2?: number;
  y2?: number;
  distance?: number;
  referencePieceIds?: string[];
};

type PieceNodeProps = {
  piece: Piece;
  type: Type;
};

type DesignNodeProps = {
  piece: Piece;
  externalConnections: SemioConnection[];
};

type PieceNode = Node<PieceNodeProps, "piece">;
type DesignNode = Node<DesignNodeProps, "design">;
type DiagramNode = PieceNode | DesignNode;

type ConnectionEdge = Edge<{ SemioConnection: SemioConnection; isParentConnection?: boolean }, "SemioConnection">;
type DiagramEdge = ConnectionEdge;

type PortHandleProps = {
  port: Port;
  pieceId: string;
  selected?: boolean;
  onPortClick: (port: Port) => void;
};

const getPortPositionStyle = (port: Port): { x: number; y: number } => {
  const { t } = port;
  if (t === undefined) {
    return { x: 0, y: 0 };
  }
  const angle = t * 2 * Math.PI;
  const radius = ICON_WIDTH / 2;
  return {
    x: radius * Math.sin(angle),
    y: -(radius * Math.cos(angle) - radius),
  };
};

const PortHandle: React.FC<PortHandleProps> = ({ port, pieceId, selected = false, onPortClick }) => {
  const { x, y } = getPortPositionStyle(port);
  const portColor = findAttributeValue(port, "semio.color", "var(--foreground)")!;
  const { hoverPort } = useDesignAppCommands();
  // Use granular hook that only re-renders when this specific port's hover state changes
  const isHovered = useDesignAppIsPortHovered(undefined, pieceId, port.guid ?? "");

  const onClick = (event: React.MouseEvent) => {
    event.stopPropagation();
    onPortClick(port);
  };

  return (
    <Handle
      id={port.guid ?? ""}
      type="source"
      className="left-1/2 top-0 cursor-selectable"
      style={{
        left: x + ICON_WIDTH / 2,
        top: y,
        backgroundColor: selected ? "var(--active-base)" : isHovered ? "var(--hover-base)" : portColor,
        border: selected || isHovered ? "2px solid var(--border-color)" : "0",
        zIndex: selected || isHovered ? 20 : 10,
      }}
      position={Position.Top}
      role="button"
      onClick={onClick}
      onPointerEnter={() => {
        if (port.guid) hoverPort("semio.sketchpad.app.design.canvas.diagram.portHandle.onPointerEnter", pieceId, port.guid);
      }}
      onPointerLeave={() => {
        // Do nothing - let parent handle hover clear
      }}
    />
  );
};

// PERF: Shared commands ref - created once and reused by all nodes
// This avoids each of 180 nodes calling useDesignAppCommands()
let sharedCommandsRef: ReturnType<typeof useDesignAppCommands> | null = null;

// PERF: Custom comparison function to prevent re-renders during node drag
// ReactFlow updates node positions internally which triggers re-renders,
// but the visual content of the node doesn't change during position updates
const pieceNodeAreEqual = (prevProps: NodeProps<PieceNode>, nextProps: NodeProps<PieceNode>) => {
  // Node ID is the same
  if (prevProps.id !== nextProps.id) return false;
  // Data reference might change but the piece guid should be the same
  const prevData = prevProps.data as PieceNodeProps;
  const nextData = nextProps.data as PieceNodeProps;
  if (prevData.piece.guid !== nextData.piece.guid) return false;
  // Check if the piece data actually changed (not just position)
  // Position is handled by ReactFlow wrapper, we don't need to re-render for it
  if (prevData.piece !== nextData.piece) {
    // Only compare content that affects rendering
    if (prevData.piece.type?.guid !== nextData.piece.type?.guid) return false;
    if (prevData.piece.design?.guid !== nextData.piece.design?.guid) return false;
    if (prevData.piece.description !== nextData.piece.description) return false;
    if (prevData.piece.isHidden !== nextData.piece.isHidden) return false;
  }
  // Type changes require re-render
  if (prevData.type.guid !== nextData.type.guid) return false;
  if (prevData.type.name !== nextData.type.name) return false;
  // Consider equal if core data matches (position changes are handled by ReactFlow)
  return true;
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data }) => {
  const {
    piece,
    piece: { guid, attributes },
    type,
  } = data as PieceNodeProps & { diffStatus: DiffStatus };
  const ports = type.ports;

  // PERF: Use centralized render data - O(1) lookup, no subscription
  const renderData = usePieceRenderData(guid);
  const isSelected = renderData.isSelected;

  // PERF: Use shared selected port from context instead of per-node hook
  const selectedPort = useContext(SelectedPortContext);

  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isDesignPiece = !!piece.design;

  // PERF: Use shared commands - stable reference, no re-renders
  const commands = sharedCommandsRef!;

  const selectPiecePort = useCallback(
    (piece: Guid, port: Guid) => {
      commands.selectPiecePort("semio.sketchpad.app.design.canvas.diagram.pieceNode", piece, port);
    },
    [commands],
  );

  const deselectPiecePort = useCallback(() => {
    commands.deselectPiecePort("semio.sketchpad.app.design.canvas.diagram.pieceNode");
  }, [commands]);

  const addConnection = useCallback(
    (connection: SemioConnection) => {
      commands.addConnection("semio.sketchpad.app.design.canvas.diagram.pieceNode", connection);
    },
    [commands],
  );

  const handleMouseEnter = useCallback(
    (event: React.PointerEvent) => {
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
        globalHoverClearTimeout = null;
      }
      // PERF: Skip hover updates during panning, node dragging, or when any mouse button is pressed
      // Checking event.buttons catches pan start before onMoveStart fires
      if (isPanning || isDraggingNode || event.buttons !== 0) return;
      if (currentHoveredPieceGuid !== guid) {
        currentHoveredPieceGuid = guid;
        commands.hoverPiece("semio.sketchpad.app.design.canvas.diagram.pieceNode.handleMouseEnter", guid);
      }
    },
    [guid, commands],
  );

  const handleMouseLeave = useCallback(
    (event: React.PointerEvent) => {
      // PERF: Skip hover clear during panning or when mouse button is pressed
      if (isPanning || isDraggingNode || event.buttons !== 0) return;
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
      }
      const pieceGuidAtLeave = guid;
      globalHoverClearTimeout = setTimeout(() => {
        if (currentHoveredPieceGuid === pieceGuidAtLeave) {
          commands.clearHover("semio.sketchpad.app.design.canvas.diagram.pieceNode.handleMouseLeave");
          currentHoveredPieceGuid = null;
        }
        globalHoverClearTimeout = null;
      }, 50);
    },
    [guid, commands],
  );

  return (
    <PieceNodeInner
      id={id}
      piece={piece}
      type={type}
      ports={ports}
      isSelected={isSelected}
      diff={diff}
      isDesignPiece={isDesignPiece}
      selectedPort={selectedPort}
      selectPiecePort={selectPiecePort}
      deselectPiecePort={deselectPiecePort}
      addConnection={addConnection}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    />
  );
}, pieceNodeAreEqual);

// Context for selected port - set once at DesignDiagram level
const SelectedPortContext = createContext<DesignAppSelection["port"] | undefined>(undefined);

type PieceNodeInnerProps = {
  id: string;
  piece: Piece;
  type: Type;
  ports: Port[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  isDesignPiece: boolean;
  selectedPort: DesignAppSelection["port"] | undefined;
  selectPiecePort: (piece: Guid, port: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: (event: React.PointerEvent) => void;
  onMouseLeave: (event: React.PointerEvent) => void;
};

const PieceNodeInner: React.FC<PieceNodeInnerProps> = ({ id, piece, type, ports, isSelected, diff, isDesignPiece, selectedPort, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  // PERF: Use centralized render data context - O(1) lookup, no subscription
  // This replaces useDesignAppPieceColor, useIsPieceHovered, and useDiffedPiece hooks
  const renderData = usePieceRenderData(piece.guid);
  const { fill, stroke, opacity: colorOpacity, isHovered } = renderData;

  // For diffedPiece, we use the piece from props directly since diff is already computed
  const diffedPiece = piece;

  // Check if piece has a center diff - only show ghost if there's an actual position change
  const hasCenterDiff = diff === DiffStatus.Modified && piece.center && diffedPiece.center && (piece.center.u !== diffedPiece.center.u || piece.center.v !== diffedPiece.center.v);

  const typeName = type.name || "";
  const displayVariant = typeName || piece.guid || "??";
  const initials = displayVariant.substring(0, 2).toUpperCase();
  const backgroundColor = fill === "transparent" ? undefined : fill;
  const showHoverBackground = fill === "var(--hover-base)";
  const textColor = isSelected ? "var(--active-foreground)" : backgroundColor && !showHoverBackground ? "var(--background)" : "var(--foreground)";
  const avatarTitle = typeName || piece.guid;
  const ringClass = isSelected ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : "";
  const fallbackStyle = backgroundColor ? { backgroundColor, color: textColor } : { color: textColor };

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selectedPort;

    if (!port.guid || !piece.guid) {
      console.error("[ORIGIN] Port or piece guid is undefined", { portGuid: port.guid, pieceGuid: piece.guid });
      return;
    }

    if (currentSelectedPort && (currentSelectedPort.piece !== piece.guid || currentSelectedPort.port !== port.guid)) {
      if (!currentSelectedPort.piece || !currentSelectedPort.port) {
        console.error("[ORIGIN] Selected port has undefined piece or port guid", {
          selectedPiece: currentSelectedPort.piece,
          selectedPort: currentSelectedPort.port,
        });
        return;
      }

      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          piece: { guid: currentSelectedPort.piece },
          port: { guid: currentSelectedPort.port },
        },
        connected: { piece: { guid: piece.guid }, port: { guid: port.guid } },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece === piece.guid && currentSelectedPort.port === port.guid) {
      deselectPiecePort();
    } else {
      selectPiecePort(piece.guid, port.guid);
    }
  };

  // Calculate original position in pixels for the ghost node
  const originalPixelPos = hasCenterDiff
    ? {
        x: (piece.center?.u ?? 0) * ICON_WIDTH,
        y: -(piece.center?.v ?? 0) * ICON_WIDTH,
      }
    : null;

  return (
    <div
      className="cursor-selectable"
      style={{
        opacity: colorOpacity,
        width: ICON_WIDTH,
        height: ICON_WIDTH,
        position: "relative",
        pointerEvents: "all",
      }}
      onPointerEnter={onMouseEnter}
      onPointerLeave={onMouseLeave}
    >
      {/* Original node (muted border only) - rendered at absolute position */}
      {hasCenterDiff && originalPixelPos && (
        <div
          style={{
            position: "absolute",
            left: originalPixelPos.x - (diffedPiece.center?.u ?? 0) * ICON_WIDTH,
            top: originalPixelPos.y - -(diffedPiece.center?.v ?? 0) * ICON_WIDTH,
            pointerEvents: "none",
            width: ICON_WIDTH,
            height: ICON_WIDTH,
          }}
        >
          <svg width={ICON_WIDTH} height={ICON_WIDTH}>
            <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className="stroke-[var(--muted-foreground)] fill-transparent" strokeDasharray="4 4" strokeWidth={isDesignPiece ? 4 : 2} />
            {piece.plane && <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 6} className="stroke-[var(--muted-foreground)] stroke-2 fill-transparent" strokeDasharray="4 4" />}
          </svg>
        </div>
      )}

      {/* Current/diffed node */}
      <Avatar role="button" title={avatarTitle} className={`w-full h-full border-[color:var(--border-color)] ${ringClass}`} style={{ borderColor: stroke, borderWidth: isDesignPiece ? 4 : undefined }}>
        <AvatarFallback className="select-none text-xs font-bold" style={fallbackStyle}>
          {initials}
        </AvatarFallback>
      </Avatar>
      {/* Second circle for fixed pieces */}
      {diffedPiece.plane && (
        <svg width={ICON_WIDTH} height={ICON_WIDTH} style={{ position: "absolute", top: 0, left: 0, pointerEvents: "none" }}>
          <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 6} className="stroke-[var(--foreground)] stroke-2 fill-transparent" />
        </svg>
      )}
      {ports?.map((port: Port, portIndex: number) => (
        <PortHandle key={`${id}-port-${portIndex}-${port.guid}`} port={port} pieceId={piece.guid} selected={selectedPort?.piece === piece.guid && selectedPort?.port === port.guid} onPortClick={onPortClick} />
      ))}
    </div>
  );
};

const DesignNodeComponent: React.FC<NodeProps<DesignNode>> = React.memo(({ id, data }) => {
  const {
    piece,
    piece: { guid, attributes },
    externalConnections,
  } = data as DesignNodeProps & { diffStatus: DiffStatus };
  const commands = useDesignAppCommands();
  // Use granular hook that only re-renders when this specific piece's selection state changes
  const isSelected = useDesignAppIsPieceSelected(undefined, guid);
  // Use granular hook for port selection to avoid re-renders when piece/connection selection changes
  const selectedPort = useDesignAppSelectedPort();
  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;

  // PERF: Removed useDesign() - u/v calculation moved to store command to avoid subscriptions

  const selectPiecePort = useCallback(
    (piece: Guid, port: Guid) => {
      commands.selectPiecePort("semio.sketchpad.app.design.canvas.diagram.designNode", piece, port);
    },
    [commands],
  );

  const deselectPiecePort = useCallback(() => {
    commands.deselectPiecePort("semio.sketchpad.app.design.canvas.diagram.designNode");
  }, [commands]);

  const addConnection = useCallback(
    (connection: SemioConnection) => {
      // u/v is now calculated in the store's addConnection command
      commands.addConnection("semio.sketchpad.app.design.canvas.diagram.designNode", connection);
    },
    [commands],
  );

  const handleMouseEnter = useCallback(
    (event: React.PointerEvent) => {
      // Clear any global pending clear hover
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
        globalHoverClearTimeout = null;
      }
      // PERF: Skip hover during pan - check buttons to catch pan before onMoveStart fires
      if (isPanning || isDraggingNode || event.buttons !== 0) return;

      // Only set hover if this is a different piece
      if (currentHoveredPieceGuid !== guid) {
        currentHoveredPieceGuid = guid;
        commands.hoverPiece("semio.sketchpad.app.design.canvas.diagram.designNode.handleMouseEnter", guid);
      }
    },
    [guid, commands],
  );

  const handleMouseLeave = useCallback(
    (event: React.PointerEvent) => {
      // PERF: Skip hover clear during pan
      if (isPanning || isDraggingNode || event.buttons !== 0) return;
      // Clear any existing global timeout
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
      }

      // Set a global timeout for clearing hover
      // Only clear if this piece is still the currently hovered one
      const pieceGuidAtLeave = guid;
      globalHoverClearTimeout = setTimeout(() => {
        if (currentHoveredPieceGuid === pieceGuidAtLeave) {
          commands.clearHover("semio.sketchpad.app.design.canvas.diagram.designNode.handleMouseLeave");
          currentHoveredPieceGuid = null;
        }
        globalHoverClearTimeout = null;
      }, 50);
    },
    [guid, commands],
  );

  const ports: Port[] = externalConnections.map((SemioConnection, portIndex) => {
    const connectedIsDesignPiece = SemioConnection.connected.piece.guid === piece.guid || SemioConnection.connected.designPiece?.guid === piece.guid;
    const connectingIsDesignPiece = SemioConnection.connecting.piece.guid === piece.guid || SemioConnection.connecting.designPiece?.guid === piece.guid;

    const designSide = connectedIsDesignPiece ? SemioConnection.connected : SemioConnection.connecting;
    const originalSide = connectedIsDesignPiece ? SemioConnection.connecting : SemioConnection.connected;

    const totalPorts = externalConnections.length;
    const t = portIndex / totalPorts;

    const angle = t * 2 * Math.PI;
    const radius = 0.5;

    const portX = radius * Math.sin(angle);
    const portY = radius * Math.cos(angle);
    const portZ = 0;

    const directionX = Math.sin(angle);
    const directionY = Math.cos(angle);
    const directionZ = 0;

    return {
      guid: `port-${portIndex}`,
      description: `Port for SemioConnection to ${originalSide.piece.guid}:${originalSide.port?.guid ?? ""}`,
      interface: { guid: "default" },
      mandatory: false,
      t: t,
      point: { x: portX, y: portY, z: portZ },
      direction: { x: directionX, y: directionY, z: directionZ },
      attributes: [
        {
          guid: crypto.randomUUID(),
          key: "semio.originalPieceId",
          value: designSide.piece.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.originalPortId",
          value: designSide.port?.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalPieceId",
          value: originalSide.piece.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalPortId",
          value: originalSide.port?.guid || "",
        },
      ],
    };
  });

  return (
    <PieceScopeProvider guid={guid}>
      <DesignNodeInner
        id={id}
        piece={piece}
        ports={ports}
        isSelected={isSelected}
        diff={diff}
        selectedPort={selectedPort}
        selectPiecePort={selectPiecePort}
        deselectPiecePort={deselectPiecePort}
        addConnection={addConnection}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
      />
    </PieceScopeProvider>
  );
});

type DesignNodeInnerProps = {
  id: string;
  piece: Piece;
  ports: Port[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  selectedPort: DesignAppSelection["port"] | undefined;
  selectPiecePort: (piece: Guid, port: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: (event: React.PointerEvent) => void;
  onMouseLeave: (event: React.PointerEvent) => void;
};

const DesignNodeInner: React.FC<DesignNodeInnerProps> = ({ id, piece, ports, isSelected, diff, selectedPort, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const isHovered = useIsPieceHovered();

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selectedPort;

    if (!port.guid || !piece.guid) {
      console.error("[ORIGIN] Port or piece guid is undefined in DesignNode", { portGuid: port.guid, pieceGuid: piece.guid });
      return;
    }

    if (currentSelectedPort && (currentSelectedPort.piece !== piece.guid || currentSelectedPort.port !== port.guid)) {
      if (!currentSelectedPort.piece || !currentSelectedPort.port) {
        console.error("[ORIGIN] Selected port has undefined piece or port guid in DesignNode", {
          selectedPiece: currentSelectedPort.piece,
          selectedPort: currentSelectedPort.port,
        });
        return;
      }

      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          piece: { guid: currentSelectedPort.piece },
          port: { guid: currentSelectedPort.port },
        },
        connected: { piece: { guid: piece.guid }, port: { guid: port.guid } },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece === piece.guid && currentSelectedPort.port === port.guid) {
      deselectPiecePort();
    } else {
      selectPiecePort(piece.guid, port.guid);
    }
  };

  let fillClass = "fill-transparent";
  let strokeColor = "var(--foreground)";
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    fillClass = "fill-[var(--color-success)]";
    strokeColor = "var(--color-success)";
  } else if (diff === DiffStatus.Removed) {
    fillClass = "fill-[var(--color-danger)]";
    strokeColor = "var(--color-danger)";
    opacity = 0.2;
  } else if (diff === DiffStatus.Modified) {
    fillClass = "fill-[var(--color-warning)]";
    strokeColor = "var(--color-warning)";
  }
  if (isHovered && !isSelected) {
    fillClass = "fill-[var(--hover-base)]";
    strokeColor = "var(--foreground)";
    opacity = 1;
  }
  if (isSelected) {
    fillClass = "fill-[var(--active-base)]";
    strokeColor = "var(--foreground)";
    opacity = 1;
  }

  return (
    <div
      className="cursor-selectable"
      style={{
        opacity,
        width: ICON_WIDTH,
        height: ICON_WIDTH,
        position: "relative",
        pointerEvents: "all",
      }}
      onPointerEnter={onMouseEnter}
      onPointerLeave={onMouseLeave}
    >
      <svg width={ICON_WIDTH} height={ICON_WIDTH} role="button" style={{ pointerEvents: "all" }}>
        <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className={fillClass} stroke={strokeColor} strokeWidth={4} />
        {piece.plane && <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 6} className="stroke-[var(--foreground)] stroke-2 fill-transparent" />}
        <text x={ICON_WIDTH / 2} y={ICON_WIDTH / 2} textAnchor="middle" dominantBaseline="middle" className={`text-xs font-bold ${isSelected ? "fill-[var(--active-foreground)]" : "fill-foreground"}`} style={{ pointerEvents: "none" }}>
          {piece.guid}
        </text>
      </svg>
      {ports?.map((port: Port, portIndex: number) => (
        <PortHandle key={`${id}-port-${portIndex}-${port.guid}`} port={port} pieceId={piece.guid} selected={selectedPort?.piece === piece.guid && selectedPort?.port === port.guid} onPortClick={onPortClick} />
      ))}
    </div>
  );
};
const nodeComponents = { piece: PieceNodeComponent, design: DesignNodeComponent };

const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = (props) => {
  const connectionGuid = props.data?.SemioConnection?.guid;
  if (!connectionGuid) {
    return <ConnectionEdgeFallback {...props} />;
  }
  return (
    <ConnectionScopeProvider guid={connectionGuid}>
      <ConnectionEdgeInner {...props} connectionGuid={connectionGuid} />
    </ConnectionScopeProvider>
  );
};

const ConnectionEdgeFallback: React.FC<EdgeProps<ConnectionEdge>> = ({ sourceX, sourceY, targetX, targetY, data, selected }) => {
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;
  const diff = (data?.SemioConnection?.attributes?.find((q: any) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = "var(--foreground)";
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = "var(--color-success)";
    dasharray = "5 5";
  } else if (diff === DiffStatus.Removed) {
    stroke = "var(--color-danger)";
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = "var(--color-warning)";
  }
  if (isParentConnection) {
    stroke = "var(--accent-secondary)";
    strokeWidth = 3;
  }
  if (selected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }

  return (
    <BaseEdge
      path={path}
      style={{
        stroke,
        strokeWidth,
        strokeDasharray: dasharray,
        opacity,
      }}
      className="transition-colors duration-200"
    />
  );
};

type ConnectionEdgeInnerProps = EdgeProps<ConnectionEdge> & { connectionGuid: Guid };

const ConnectionEdgeInner: React.FC<ConnectionEdgeInnerProps> = ({ sourceX, sourceY, targetX, targetY, data, connectionGuid }) => {
  const { hoverConnection, clearHover } = useDesignAppCommands();
  const isHovered = useIsConnectionHovered();
  // PERF: Use granular selection hook instead of prop from ReactFlow - prevents re-renders when other selections change
  const isSelected = useDesignAppIsConnectionSelected(undefined, connectionGuid);
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;

  const diff = (data?.SemioConnection?.attributes?.find((q: any) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = "var(--foreground)";
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = "var(--color-success)";
    dasharray = "5 5";
  } else if (diff === DiffStatus.Removed) {
    stroke = "var(--color-danger)";
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = "var(--color-warning)";
  }
  if (isParentConnection) {
    stroke = "var(--accent-secondary)";
    strokeWidth = 3;
  }
  if (isHovered && !isSelected) {
    stroke = "var(--hover-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }
  if (isSelected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }

  return (
    <g>
      <BaseEdge
        path={path}
        style={{
          stroke,
          strokeWidth,
          strokeDasharray: dasharray,
          opacity,
        }}
        className="transition-colors duration-200 pointer-events-none"
      />
      <path
        d={path}
        fill="none"
        stroke="transparent"
        strokeWidth={Math.max(strokeWidth, 6)}
        onPointerEnter={(e) => {
          // PERF: Skip hover during pan - check buttons to catch pan before onMoveStart fires
          if (isPanning || isDraggingNode || e.buttons !== 0) return;
          if (connectionGuid) hoverConnection("semio.sketchpad.app.design.canvas.diagram.connectionEdge.onPointerEnter", connectionGuid);
        }}
        onPointerLeave={(e) => {
          // PERF: Skip hover clear during pan
          if (isPanning || isDraggingNode || e.buttons !== 0) return;
          clearHover("semio.sketchpad.app.design.canvas.diagram.connectionEdge.onPointerLeave");
        }}
      />
    </g>
  );
};
const edgeComponents = { SemioConnection: ConnectionEdgeComponent };

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props;
  const HANDLE_HEIGHT = 5;
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`;
  return <BaseEdge path={path} style={{ stroke: "gray" }} className="opacity-70" />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={`${selected ? "fill-accent" : "fill-foreground"} transition-colors duration-200`} cx={x} cy={y} r="10" />;
};

const HelperLines: React.FC<{
  lines: HelperLine[];
  nodes: { id: string; position: { x: number; y: number } }[];
}> = ({ lines, nodes }) => {
  const { getViewport } = useReactFlow();

  if (lines.length === 0) return null;

  const viewport = getViewport();

  return (
    <div className="absolute inset-0 w-full h-full pointer-events-none z-modal overflow-hidden">
      {lines.map((line, index) => {
        if (line.kind === "horizontal" && line.position !== undefined) {
          const screenY = line.position * viewport.zoom + viewport.y;
          return <div key={`h-${line.relatedPieceId}-${index}`} className="absolute left-0 w-full h-px border-t border-dashed border-accent opacity-60" style={{ top: screenY }} />;
        } else if (line.kind === "vertical" && line.position !== undefined) {
          const screenX = line.position * viewport.zoom + viewport.x;
          return <div key={`v-${line.relatedPieceId}-${index}`} className="absolute top-0 w-px h-full border-l border-dashed border-accent opacity-60" style={{ left: screenX }} />;
        } else if (line.kind === "equalDistance" && line.x1 !== undefined && line.y1 !== undefined && line.x2 !== undefined && line.y2 !== undefined) {
          const screenX1 = line.x1 * viewport.zoom + viewport.x;
          const screenY1 = line.y1 * viewport.zoom + viewport.y;
          const screenX2 = line.x2 * viewport.zoom + viewport.x;
          const screenY2 = line.y2 * viewport.zoom + viewport.y;

          const isMidLine = line.relatedPieceId.startsWith("mid-");
          const strokeColor = "var(--accent)";
          const strokeWidth = isMidLine ? "3" : "2";
          const opacity = isMidLine ? 1 : 0.7;
          const dashArray = isMidLine ? "4 4" : "8 4";

          return (
            <svg key={`eq-${line.relatedPieceId}-${index}`} className="absolute inset-0 w-full h-full pointer-events-none">
              <line x1={screenX1} y1={screenY1} x2={screenX2} y2={screenY2} stroke={strokeColor} strokeWidth={strokeWidth} strokeDasharray={dashArray} opacity={opacity} />
            </svg>
          );
        }
        return null;
      })}
    </div>
  );
};

// PERF: Remove selection state from node creation - selection is handled by individual components via granular hooks
const pieceToNode = (piece: Piece, type: Type, center: Coord, index: number): PieceNode => ({
  type: "piece",
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.u * ICON_WIDTH || 0,
    y: -center.v * ICON_WIDTH || 0,
  },
  selected: false, // Selection is handled by components via useDesignAppIsPieceSelected
  draggable: true,
  data: { piece, type },
  className: "",
});

// PERF: Remove selection state from node creation - selection is handled by individual components via granular hooks
const designToNode = (piece: Piece, externalConnections: SemioConnection[], center: Coord, index: number): DesignNode => ({
  type: "design",
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.u * ICON_WIDTH || 0,
    y: -center.v * ICON_WIDTH || 0,
  },
  selected: false, // Selection is handled by components via useDesignAppIsPieceSelected
  draggable: true,
  data: { piece, externalConnections },
  className: "",
});

const extractPieceIdFromNodeId = (nodeId: string): { guid: string } => {
  return { guid: nodeId.split("-").slice(2).join("-") };
};

const getPieceIdFromNode = (node: DiagramNode): string => {
  return node.data.piece.guid;
};

const connectionToEdge = (
  SemioConnection: SemioConnection,
  selected: boolean,
  isParentConnection: boolean = false,
  pieceIndexMap: Map<string, number>,
  connectionIndex: number = 0,
  designPieces?: Piece[],
  allConnections?: SemioConnection[],
): ConnectionEdge => {
  let sourcePieceId = SemioConnection.connecting.piece;
  let targetPieceId = SemioConnection.connected.piece;
  let sourcePortId = SemioConnection.connecting.port ?? "undefined";
  let targetPortId = SemioConnection.connected.port ?? "undefined";

  if (SemioConnection.connecting.designPiece && allConnections) {
    const designPieceId = SemioConnection.connecting.designPiece;
    sourcePieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece === SemioConnection.connecting.designPiece;
      const connectingToDesign = conn.connecting.designPiece === SemioConnection.connecting.designPiece;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece === SemioConnection.connected.piece && conn.connecting.piece === SemioConnection.connecting.piece && conn.connected.port === SemioConnection.connected.port && conn.connecting.port === SemioConnection.connecting.port,
    );
    sourcePortId = portIndex >= 0 ? { guid: `port-${portIndex}` } : { guid: "port-0" };
  }

  if (SemioConnection.connected.designPiece && allConnections) {
    const designPieceId = SemioConnection.connected.designPiece;
    targetPieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece === SemioConnection.connected.designPiece;
      const connectingToDesign = conn.connecting.designPiece === SemioConnection.connected.designPiece;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece === SemioConnection.connected.piece && conn.connecting.piece === SemioConnection.connecting.piece && conn.connected.port === SemioConnection.connected.port && conn.connecting.port === SemioConnection.connecting.port,
    );
    targetPortId = portIndex >= 0 ? { guid: `port-${portIndex}` } : { guid: "port-0" };
  }

  const sourceIndex = pieceIndexMap.get(sourcePieceId.guid) ?? 0;
  const targetIndex = pieceIndexMap.get(targetPieceId.guid) ?? 0;
  const sourceNodeId = `piece-${sourceIndex}-${sourcePieceId.guid}`;
  const targetNodeId = `piece-${targetIndex}-${targetPieceId.guid}`;

  return {
    type: "SemioConnection",
    id: SemioConnection.guid,
    source: sourceNodeId,
    sourceHandle: typeof sourcePortId === "string" ? sourcePortId : sourcePortId.guid,
    target: targetNodeId,
    targetHandle: typeof targetPortId === "string" ? targetPortId : targetPortId.guid,
    data: { SemioConnection, isParentConnection },
    selected,
  };
};

// PERF: Remove selection parameter - selection is now handled by individual components via granular hooks
const designToNodesAndEdges = (design: Design, metadata: Map<string, PieceMetadata>, kit: any) => {
  if (!design) return null;

  const centerMap = new Map<string, Coord>();
  metadata.forEach((meta, pieceGuid) => {
    if (meta.center) {
      centerMap.set(pieceGuid, meta.center);
    }
  });

  const pieceNodes =
    design.pieces
      ?.map((piece, i) => {
        const center = centerMap.get(piece.guid) || piece.center || { u: 0, v: 0 };

        if (piece.design) {
          const design = kit.designs?.find((d: Design) => d.guid === piece.design?.guid);
          if (!design) {
            const fallbackType: Type = {
              guid: `fallback-${piece.design}`,
              name: `Unknown-${piece.design}`,
              unit: "m",
              description: `Missing design: ${piece.design}`,
              ports: [],
              models: [],
            };
            return pieceToNode(piece, fallbackType, center, i);
          }
          const designAsType: Type = {
            guid: design.guid,
            name: design.name,
            unit: design.unit || "m",
            description: design.description,
            ports: [],
            models: [],
          };
          return pieceToNode(piece, designAsType, center, i);
        }

        if (!piece.type) {
          return null;
        }

        const type = findTypeInKit(kit, typeof piece.type === "string" ? piece.type : piece.type?.guid);
        if (!type) {
          const fallbackType: Type = {
            guid: `fallback-${piece.type}`,
            name: `Unknown-${piece.type}`,
            unit: "m",
            description: `Missing type: ${piece.type}`,
            ports: [],
            models: [],
          };
          return pieceToNode(piece, fallbackType, center, i);
        }
        return pieceToNode(piece, type, center, i);
      })
      .filter((node): node is PieceNode => node !== null) ?? [];

  const includedDesigns = getIncludedDesigns(design);

  const designNodes = includedDesigns.map((includedDesign, i) => {
    if (includedDesign.type === "connected") {
      let calculatedCenter: Coord = { u: 0, v: 0 };
      if (includedDesign.externalConnections && includedDesign.externalConnections.length > 0) {
        const connectedPieceIds = new Set<string>();
        includedDesign.externalConnections.forEach((conn) => {
          if (conn.connected.designPiece?.guid === includedDesign.designGuid) {
            connectedPieceIds.add(conn.connecting.piece.guid);
          } else if (conn.connecting.designPiece?.guid === includedDesign.designGuid) {
            connectedPieceIds.add(conn.connected.piece.guid);
          }
        });

        const connectedPieceCenters: Coord[] = [];
        Array.from(connectedPieceIds).forEach((pieceId) => {
          const center = centerMap.get(pieceId);
          if (center) {
            connectedPieceCenters.push(center);
          }
        });

        if (connectedPieceCenters.length > 0) {
          const avgU = connectedPieceCenters.reduce((sum, center) => sum + center.u, 0) / connectedPieceCenters.length;
          const avgV = connectedPieceCenters.reduce((sum, center) => sum + center.v, 0) / connectedPieceCenters.length;

          calculatedCenter = {
            u: Math.round(avgU),
            v: Math.round(avgV),
          };
        }
      }

      const designPiece: Piece = {
        guid: includedDesign.guid,
        type: { guid: includedDesign.designGuid },
        center: calculatedCenter,
        description: `Clustered design: ${includedDesign.designGuid}`,
      };

      return designToNode(designPiece, includedDesign.externalConnections || [], calculatedCenter, design.pieces!.length + i);
    } else {
      const displayCenter = includedDesign.center || { u: 0, v: 0 };

      const designPiece: Piece = {
        guid: includedDesign.guid,
        type: { guid: includedDesign.designGuid },
        center: displayCenter,
        plane: includedDesign.plane,
        description: `Fixed design: ${includedDesign.designGuid}`,
      };

      return designToNode(designPiece, [], displayCenter, design.pieces!.length + i);
    }
  });

  const pieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    if (!pieceIndexMap.has(piece.guid)) {
      pieceIndexMap.set(piece.guid, index);
    }
  });

  includedDesigns.forEach((includedDesign, index) => {
    if (!pieceIndexMap.has(includedDesign.guid)) {
      pieceIndexMap.set(includedDesign.guid, design.pieces!.length + index);
    }
  });

  const nodeIdToPieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    nodeIdToPieceIndexMap.set(`piece-${index}-${piece.guid}`, index);
  });
  includedDesigns.forEach((includedDesign, index) => {
    const nodeIndex = design.pieces!.length + index;
    nodeIdToPieceIndexMap.set(`piece-${nodeIndex}-${includedDesign.guid}`, nodeIndex);
  });

  // PERF: Remove selection-dependent edge processing - selection state is handled by edge components
  const connectionEdges =
    design.connections?.map((SemioConnection, connectionIndex) => {
      // Selection is handled by edge components via granular hooks
      return connectionToEdge(SemioConnection, false, false, pieceIndexMap, connectionIndex, design.pieces, design.connections);
    }) ?? [];
  return { nodes: [...pieceNodes, ...designNodes], edges: connectionEdges };
};

interface DesignDiagramProps {
  reactFlowInstanceRef: React.MutableRefObject<ReactFlowInstance | null>;
}

const DesignDiagram: FC<DesignDiagramProps> = ({ reactFlowInstanceRef }) => {
  const {
    deselectAll,
    selectPiece,
    addPieceToSelection,
    removePieceFromSelection,
    selectConnection,
    addConnectionToSelection,
    removeConnectionFromSelection,
    toggleDiagramFullscreen,
    startTransaction,
    finalizeTransaction,
    abortTransaction,
    execute,
    addConnection,
    addConnections,
    updatePieces,
    updateConnections,
    addPiece,
    setDiagramCenter,
    setDiagramScale,
    focusPiece,
    hoverPiece,
    clearHover,
  } = useDesignAppCommands();

  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  // PERF: Use targeted hooks instead of full kit subscription to avoid re-renders during pan
  const kitTypes = useKitTypes();
  const kitDesigns = useKitDesigns();
  const kit = useKit() as Kit;
  // PERF: Use granular hook instead of useDesignApp to avoid re-renders on any state change
  const [activeTool] = useDesignAppActiveTool();

  // PERF: Use ref to hold selection to avoid callback recreation on selection changes
  const [selection] = useDesignAppSelection();
  const selectionRef = useRef(selection);
  selectionRef.current = selection;
  const [fullscreenWindow] = useDesignAppFullscreen();
  const [others] = useDesignAppOthers();
  const [savedDiagramCenter] = useDesignAppDiagramCenter();
  const [savedDiagramScale] = useDesignAppDiagramScale();
  const panelVisibility = useAppPanelVisibility();

  const design = useDesign() as Design | null;
  const metadata = usePiecesMetadataMap();

  // PERF: Set shared commands ref once - all 180 nodes reuse this instead of calling useDesignAppCommands
  const commands = useDesignAppCommands();
  sharedCommandsRef = commands;

  // PERF: Get hover and transaction state for centralized render data computation
  const { transitivelyHoveredPieces } = useContext(HoverPiecesContext);
  const { statusMap: transactionStatusMap } = useContext(TransactionPiecesContext);

  // PERF: Compute ALL piece render data in ONE pass - replaces 180 × 6 = 1080 individual subscriptions
  // This is the key architectural change: centralized computation instead of per-node hooks
  const pieceRenderDataMap = useMemo(() => {
    const map = new Map<string, PieceRenderData>();
    if (!design?.pieces) return map;

    const selectedPieces = new Set(selection?.pieces ?? []);

    for (const piece of design.pieces) {
      const pieceGuid = piece.guid;
      const isSelected = selectedPieces.has(pieceGuid);
      const isHovered = transitivelyHoveredPieces.has(pieceGuid);
      const diffStatus: DiffStatus = transactionStatusMap.get(pieceGuid) ?? DiffStatus.Unchanged;
      const isChangedInTransaction = diffStatus !== DiffStatus.Unchanged;

      // Compute fill/stroke/opacity based on state (same logic as useDesignAppPieceColor)
      let fill = "transparent";
      let stroke = "var(--foreground)";
      let opacity = 1;

      if (diffStatus === DiffStatus.Added) {
        fill = "var(--color-success)";
        stroke = "var(--color-success)";
      } else if (diffStatus === DiffStatus.Removed) {
        fill = "var(--color-danger)";
        stroke = "var(--color-danger)";
        opacity = 0.2;
      } else if (diffStatus === DiffStatus.Modified) {
        fill = "var(--color-warning)";
        stroke = "var(--color-warning)";
      } else if (isChangedInTransaction) {
        fill = "var(--color-changed-base)";
        stroke = "var(--color-changed-base)";
      }

      if (isHovered && !isSelected) {
        fill = "var(--hover-base)";
        stroke = "var(--foreground)";
        opacity = 1;
      }

      if (isSelected) {
        // Use string comparison to avoid TypeScript narrowing issues
        const status = diffStatus as string;
        if (isChangedInTransaction) {
          fill = "var(--color-selected-changed)";
        } else if (status === "added") {
          fill = "var(--color-selected-added)";
        } else if (status === "removed") {
          fill = "var(--color-selected-removed)";
        } else if (status === "modified") {
          fill = "var(--color-selected-changed)";
        } else {
          fill = "var(--active-base)";
        }
        stroke = "var(--foreground)";
        opacity = 1;
      }

      map.set(pieceGuid, {
        isSelected,
        isHovered,
        fill,
        stroke,
        opacity,
        isChangedInTransaction,
        diffStatus,
      });
    }
    return map;
  }, [design?.pieces, selection?.pieces, transitivelyHoveredPieces, transactionStatusMap]);

  // Selected port for context
  const selectedPort = selection?.port;

  // PERF: Compute base nodes/edges from design data
  const { baseNodes, edges } = useMemo(() => {
    if (!design) return { baseNodes: [], edges: [] };
    const minimalKit = { types: kitTypes, designs: kitDesigns } as Kit;
    const result = designToNodesAndEdges(design, metadata, minimalKit) ?? { nodes: [], edges: [] };
    return { baseNodes: result.nodes, edges: result.edges };
  }, [design, metadata, kitTypes, kitDesigns]);

  // PERF: Use local state for nodes during drag - ReactFlow updates this during drag operations
  // This decouples drag visual updates from store updates
  const [nodes, setNodes] = useState<typeof baseNodes>(baseNodes);

  // Sync base nodes to local state when design data changes
  useEffect(() => {
    setNodes(baseNodes);
  }, [baseNodes]);

  // PERF: Handle ReactFlow's node position changes during drag
  // CRITICAL: Skip all state updates during drag/pan - ReactFlow handles visual updates internally
  // This prevents React from re-rendering during drag/pan, which was causing ~2300ms delays
  const onNodesChangeReactFlow = useCallback((changes: any[]) => {
    // PERF: During drag or pan, don't update React state at all
    // ReactFlow maintains its own internal state for drag/pan operations
    // We only sync to React state when dragging ends (in onNodeDragStop)
    if (isDraggingNode || isPanning) return;

    // When not dragging/panning, apply changes normally
    const positionChanges = changes.filter((c: any) => c.type === "position");
    if (positionChanges.length === 0) return;
    setNodes((nds) => applyNodeChanges(positionChanges, nds) as typeof nds);
  }, []);

  const focusContext = useFocusSafe();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const prevItemsRef = useRef<string>("");
  const diagramId = useRef(guid()).current;

  useEffect(() => {
    if (!focusContext) return;
    const items = [
      ...nodes.map((n) => ({
        id: n.data.piece.guid,
        label: n.data.piece.description || `Piece ${n.data.piece.guid.substring(0, 8)}`,
        category: "Pieces",
      })),
      ...edges.map((e) => ({
        id: e.data?.SemioConnection?.guid || e.id,
        label: e.data?.SemioConnection?.description || `Connection ${e.id}`,
        category: "Connections",
      })),
    ];
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
  }, [nodes, edges]);

  useEffect(() => {
    if (!focusContext) return;
    const handleFocus = (itemId: string) => {
      const node = nodes.find((n) => n.data.piece.guid === itemId);
      if (node) {
        setFocusedItemId(node.id);
      }
      focusPiece("semio.sketchpad.app.design.canvas.diagram.handleFocus", itemId);
    };
    focusContext.setOnFocusItem(handleFocus);
    return () => {
      if (focusContext) focusContext.setOnFocusItem(undefined);
    };
  }, [focusContext, focusPiece, nodes]);

  if (!design) return null;

  // PERF: Track ALL drag state with refs to avoid re-renders during drag
  const dragPositionRef = useRef<{ x: number; y: number } | null>(null);
  const pendingPieceUpdatesRef = useRef<Array<{ id: string; diff: any }>>([]);
  // PERF: Helper lines are disabled for performance - uncomment to re-enable
  // const [helperLines, setHelperLines] = useState<HelperLine[]>([]);
  const helperLines: HelperLine[] = []; // No helper lines during drag for performance
  const fullscreen = fullscreenWindow === DesignAppFullscreenWindow.Diagram;
  const viewportRestoredRef = useRef(false);
  const isUpdatingViewportRef = useRef(false);
  const dropZoneRef = useRef<HTMLDivElement | null>(null);
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  // PERF: Handle pointer events to disable node interactions immediately during pan
  const handleDiagramPointerDown = useCallback(
    (e: PointerEvent) => {
      // Only track primary button (left click for panning)
      if (e.button === 0) {
        // Set flag immediately
        isPanning = true;
        // Also directly disable pointer events on the nodes container for maximum performance
        const nodesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__nodes`);
        if (nodesContainer) {
          (nodesContainer as HTMLElement).style.pointerEvents = "none";
        }
        const edgesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__edges`);
        if (edgesContainer) {
          (edgesContainer as HTMLElement).style.pointerEvents = "none";
        }
      }
    },
    [diagramId],
  );

  const handleDiagramPointerUp = useCallback(() => {
    // Re-enable pointer events on the nodes container
    const nodesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__nodes`);
    if (nodesContainer) {
      (nodesContainer as HTMLElement).style.pointerEvents = "";
    }
    const edgesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__edges`);
    if (edgesContainer) {
      (edgesContainer as HTMLElement).style.pointerEvents = "";
    }
    // Clear flag after re-enabling
    isPanning = false;
  }, [diagramId]);

  const setDropZoneRef = useCallback(
    (node: HTMLDivElement | null) => {
      // Clean up previous listeners if any
      if (dropZoneRef.current) {
        dropZoneRef.current.removeEventListener("pointerdown", handleDiagramPointerDown as any);
        dropZoneRef.current.removeEventListener("pointerup", handleDiagramPointerUp as any);
        dropZoneRef.current.removeEventListener("pointerleave", handleDiagramPointerUp as any);
      }
      if (node) {
        node.setAttribute("data-drop-zone", "diagram");
        node.setAttribute("data-drop-zone-id", diagramId);
        // PERF: Set data-panning immediately on pointer down to disable pointer events on nodes
        // This catches pan start before ReactFlow's onMoveStart fires
        node.addEventListener("pointerdown", handleDiagramPointerDown as any);
        node.addEventListener("pointerup", handleDiagramPointerUp as any);
        node.addEventListener("pointerleave", handleDiagramPointerUp as any);
      }
      dropZoneRef.current = node;
    },
    [diagramId, handleDiagramPointerDown, handleDiagramPointerUp],
  );

  const handleDragEnd = useCallback(
    (event: DragEndEvent) => {
      const { active, delta } = event;
      if (!dropZoneRef.current || !reactFlowInstanceRef.current) return;
      if (!(event.activatorEvent instanceof PointerEvent)) return;
      const dropX = event.activatorEvent.clientX + delta.x;
      const dropY = event.activatorEvent.clientY + delta.y;
      const dropZoneBounds = dropZoneRef.current.getBoundingClientRect();
      const isWithinBounds = dropX >= dropZoneBounds.left && dropX <= dropZoneBounds.right && dropY >= dropZoneBounds.top && dropY <= dropZoneBounds.bottom;
      if (!isWithinBounds) return;
      const dragData = active.data.current as { type: string; typeGuid?: string; designGuid?: string } | undefined;
      if (!dragData) return;
      const reactFlowWrapper = dropZoneRef.current.querySelector(".react-flow") as HTMLElement;
      const wrapperBounds = reactFlowWrapper?.getBoundingClientRect() ?? dropZoneBounds;
      const viewportEl = dropZoneRef.current.querySelector(".react-flow__viewport") as HTMLElement;
      const cssTransform = viewportEl?.style.transform ?? "translate(0px, 0px) scale(1)";
      const transformMatch = cssTransform.match(/translate\(([-\d.]+)px,\s*([-\d.]+)px\)\s*scale\(([\d.]+)\)/);
      const viewport = transformMatch ? { x: parseFloat(transformMatch[1]), y: parseFloat(transformMatch[2]), zoom: parseFloat(transformMatch[3]) } : { x: 0, y: 0, zoom: 1 };
      const localX = dropX - wrapperBounds.left;
      const localY = dropY - wrapperBounds.top;
      const flowX = (localX - viewport.x) / viewport.zoom;
      const flowY = (localY - viewport.y) / viewport.zoom;
      const centerU = (flowX - ICON_WIDTH / 2) / ICON_WIDTH;
      const centerV = -(flowY - ICON_WIDTH / 2) / ICON_WIDTH;
      if (dragData.type === "type" && dragData.typeGuid) {
        const droppedType = kitTypes?.find((t) => t.guid === dragData.typeGuid);
        if (!droppedType) return;
        startTransaction("semio.sketchpad.app.design.dragEnd.type");
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, type: { guid: droppedType.guid }, center: { u: centerU, v: centerV } };
        addPiece("semio.sketchpad.app.design.dragEnd.type", piece);
        finalizeTransaction("semio.sketchpad.app.design.dragEnd.type");
      } else if (dragData.type === "design" && dragData.designGuid) {
        const droppedDesign = kitDesigns?.find((d) => d.guid === dragData.designGuid);
        if (!droppedDesign) return;
        startTransaction("semio.sketchpad.app.design.dragEnd.design");
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          design: { guid: droppedDesign.guid },
          center: { u: centerU, v: centerV },
        };
        addPiece("semio.sketchpad.app.design.dragEnd.design", piece);
        finalizeTransaction("semio.sketchpad.app.design.dragEnd.design");
      }
      setActiveDraggedType(null);
      setActiveDraggedDesign(null);
    },
    [reactFlowInstanceRef, kitTypes, kitDesigns, startTransaction, addPiece, finalizeTransaction, setActiveDraggedType, setActiveDraggedDesign, diagramId],
  );

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleDragEnd(customEvent.detail);
    };
    window.addEventListener("design-drag-end", listener);
    return () => window.removeEventListener("design-drag-end", listener);
  }, [handleDragEnd]);

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape" && isDraggingRef.current) {
        abortTransaction("semio.sketchpad.app.design.canvas.diagram.handleEscape");
        isDraggingRef.current = false;
        dragPositionRef.current = null;
        pendingPieceUpdatesRef.current = [];
        pendingSelectionRef.current = null;
        if (reactFlowInstanceRef.current) {
          reactFlowInstanceRef.current.setNodes((nodes) => nodes.map((node) => ({ ...node })));
        }
      }
    };

    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [abortTransaction]);

  useEffect(() => {
    if (!viewportRestoredRef.current && savedDiagramCenter && savedDiagramScale !== undefined && reactFlowInstanceRef.current) {
      isUpdatingViewportRef.current = true;
      setTimeout(() => {
        if (reactFlowInstanceRef.current) {
          reactFlowInstanceRef.current.setViewport({ x: savedDiagramCenter.u * ICON_WIDTH, y: -savedDiagramCenter.v * ICON_WIDTH, zoom: savedDiagramScale });
          viewportRestoredRef.current = true;
          setTimeout(() => {
            isUpdatingViewportRef.current = false;
          }, 100);
        }
      }, 0);
    }
  }, [savedDiagramCenter, savedDiagramScale, reactFlowInstanceRef]);

  // PERF: Track panning state to skip hover updates during pan
  // Also set a data attribute to disable pointer events via CSS
  const onMoveStart = useCallback(() => {
    isPanning = true;
    // PERF: Set data attribute to disable pointer events on nodes during pan
    // This prevents the browser from processing mouse enter/leave events entirely
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      diagramElement.setAttribute("data-panning", "true");
    }
  }, [diagramId]);

  // PERF: Debounce viewport state updates to avoid re-renders during continuous panning
  // Use a longer delay (1000ms) to ensure panning feels responsive
  const pendingMoveEndRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onMoveEnd = useCallback(() => {
    // PERF: Mark panning as done to re-enable hover updates
    isPanning = false;
    // Remove the data attribute to re-enable pointer events on nodes
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      diagramElement.removeAttribute("data-panning");
    }

    if (isUpdatingViewportRef.current || !reactFlowInstanceRef.current) return;
    // Clear any pending update
    if (pendingMoveEndRef.current) {
      clearTimeout(pendingMoveEndRef.current);
    }
    // Debounce the state update with a longer delay to avoid re-renders during panning
    pendingMoveEndRef.current = setTimeout(() => {
      if (!reactFlowInstanceRef.current) return;
      const viewport = reactFlowInstanceRef.current.getViewport();
      setDiagramCenter("semio.sketchpad.app.design.canvas.diagram.onMoveEnd", { u: viewport.x / ICON_WIDTH, v: -viewport.y / ICON_WIDTH });
      pendingMoveEndRef.current = null;
    }, 1000);
  }, [reactFlowInstanceRef, setDiagramCenter, diagramId]);

  const centerViewport = useCallback(() => {
    if (!reactFlowInstanceRef.current) return;
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      const rect = diagramElement.getBoundingClientRect();
      if (rect.width > 0 && rect.height > 0) {
        const centerX = rect.width / 2;
        const centerY = rect.height / 2;
        reactFlowInstanceRef.current.setViewport({ x: centerX, y: centerY, zoom: 1 });
        // Save the centered position
        setDiagramCenter("semio.sketchpad.app.design.canvas.diagram.centerViewport", { u: centerX / ICON_WIDTH, v: -centerY / ICON_WIDTH });
      } else {
        console.warn("[Diagram] Element has no dimensions yet, retrying...");
        setTimeout(() => centerViewport(), 100);
      }
    } else {
      console.warn("[Diagram] Element not found, retrying...");
      setTimeout(() => centerViewport(), 100);
    }
  }, [reactFlowInstanceRef, diagramId, setDiagramCenter]);

  // PERF: Memoize all callbacks to prevent Diagram re-renders when DesignDiagram re-renders
  // Use refs for activeTool to avoid callback recreation when tool changes
  const activeToolRef = useRef(activeTool);
  activeToolRef.current = activeTool;

  const onNodeClick = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      e.stopPropagation();
      const pieceId = getPieceIdFromNode(node);
      if (e.ctrlKey || e.metaKey) removePieceFromSelection("semio.sketchpad.app.design.canvas.diagram.onNodeClick", pieceId);
      else if (e.shiftKey) addPieceToSelection("semio.sketchpad.app.design.canvas.diagram.onNodeClick", pieceId);
      else if (activeToolRef.current === ToolKind.SELECTION_ADDITIVE) addPieceToSelection("semio.sketchpad.app.design.canvas.diagram.onNodeClick", pieceId);
      else if (activeToolRef.current === ToolKind.SELECTION_SUBTRACTIVE) removePieceFromSelection("semio.sketchpad.app.design.canvas.diagram.onNodeClick", pieceId);
      else selectPiece("semio.sketchpad.app.design.canvas.diagram.onNodeClick", pieceId);
    },
    [removePieceFromSelection, addPieceToSelection, selectPiece],
  );

  // Use ref for kit to avoid callback recreation
  const kitRef = useRef(kit);
  kitRef.current = kit;

  const onNodeDoubleClick = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      e.stopPropagation();
      const kitData = kitRef.current as Kit;
      if (!kitData?.guid) return;
      const piece = node.data.piece;
      if (piece.type) sketchpadCommands.navigateToType(kitData.guid, typeof piece.type === "string" ? piece.type : piece.type.guid);
      else if (piece.design) sketchpadCommands.navigateToDesign(kitData.guid, typeof piece.design === "string" ? piece.design : piece.design.guid);
    },
    [sketchpadCommands],
  );

  const onEdgeClick = useCallback(
    (e: React.MouseEvent, edge: DiagramEdge) => {
      e.stopPropagation();
      const connectionId = edge.data!.SemioConnection.guid;
      if (e.ctrlKey || e.metaKey) removeConnectionFromSelection("semio.sketchpad.app.design.canvas.diagram.onEdgeClick", connectionId);
      else if (e.shiftKey) addConnectionToSelection("semio.sketchpad.app.design.canvas.diagram.onEdgeClick", connectionId);
      else if (activeToolRef.current === ToolKind.SELECTION_ADDITIVE) addConnectionToSelection("semio.sketchpad.app.design.canvas.diagram.onEdgeClick", connectionId);
      else if (activeToolRef.current === ToolKind.SELECTION_SUBTRACTIVE) removeConnectionFromSelection("semio.sketchpad.app.design.canvas.diagram.onEdgeClick", connectionId);
      else selectConnection("semio.sketchpad.app.design.canvas.diagram.onEdgeClick", connectionId);
    },
    [removeConnectionFromSelection, addConnectionToSelection, selectConnection],
  );

  const onPaneClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) {
        deselectAll("semio.sketchpad.app.design.canvas.diagram.onPaneClick");
      }
    },
    [deselectAll],
  );

  const onDoubleClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      toggleDiagramFullscreen("semio.sketchpad.app.design.canvas.diagram.onDoubleClick");
    },
    [toggleDiagramFullscreen],
  );

  const onCluster = useCallback((clusterPieceIds: string[]) => {
    // TODO: Implement cluster command
  }, []);

  const onExpand = useCallback((target: string) => {
    // TODO: Implement explode command
  }, []);

  const onNodeMouseEnter = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      // PERF: Skip hover during pan - check buttons to catch pan before onMoveStart fires
      if (isPanning || isDraggingNode || e.buttons !== 0) return;

      const pieceId = getPieceIdFromNode(node);

      // Clear any global pending clear hover
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
        globalHoverClearTimeout = null;
      }

      // Only set hover if this is a different piece
      if (currentHoveredPieceGuid !== pieceId) {
        currentHoveredPieceGuid = pieceId;
        hoverPiece("semio.sketchpad.app.design.canvas.diagram.onNodeMouseEnter", pieceId);
      }
    },
    [hoverPiece],
  );

  const onNodeMouseLeave = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      // PERF: Skip hover clear during pan
      if (isPanning || isDraggingNode || e.buttons !== 0) return;

      const pieceId = getPieceIdFromNode(node);

      // Clear any existing global timeout
      if (globalHoverClearTimeout) {
        clearTimeout(globalHoverClearTimeout);
      }

      // Set a global timeout for clearing hover
      // Only clear if this piece is still the currently hovered one
      const pieceGuidAtLeave = pieceId;
      globalHoverClearTimeout = setTimeout(() => {
        if (currentHoveredPieceGuid === pieceGuidAtLeave) {
          clearHover("semio.sketchpad.app.design.canvas.diagram.onNodeMouseLeave");
          currentHoveredPieceGuid = null;
        }
        globalHoverClearTimeout = null;
      }, 50);
    },
    [clearHover],
  );

  // PERF: Track pending selection changes to commit only on drag end
  const pendingSelectionRef = useRef<{ pieceId: string; action: "select" | "add" | "remove" } | null>(null);

  const onNodeDragStart = useCallback(
    (event: any, node: Node) => {
      const currentSelectedIds = selectionRef.current?.pieces ?? [];
      const pieceId = getPieceIdFromNode(node as DiagramNode);
      const isNodeSelected = currentSelectedIds.includes(pieceId);
      const ctrlKey = event.ctrlKey || event.metaKey;
      const shiftKey = event.shiftKey;

      // PERF: Defer Y.js updates - only track intended selection in ref
      // Selection will be committed on drag end to avoid re-renders during drag
      if (ctrlKey) {
        pendingSelectionRef.current = { pieceId, action: isNodeSelected ? "remove" : "add" };
      } else if (shiftKey) {
        pendingSelectionRef.current = { pieceId, action: isNodeSelected ? "select" : "add" };
      } else if (activeTool === ToolKind.SELECTION_ADDITIVE) {
        pendingSelectionRef.current = { pieceId, action: "add" };
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE) {
        pendingSelectionRef.current = { pieceId, action: "remove" };
      } else if (!isNodeSelected) {
        pendingSelectionRef.current = { pieceId, action: "select" };
      } else {
        pendingSelectionRef.current = null;
      }

      // PERF: Use refs instead of state to avoid re-renders during drag
      dragPositionRef.current = { x: node.position.x, y: node.position.y };
      pendingPieceUpdatesRef.current = [];
      // PERF: Use ref instead of state to track dragging
      isDraggingRef.current = true;
      // PERF: Set global flag to skip hover updates during drag
      isDraggingNode = true;
    },
    [activeTool],
  );

  // PERF: Use ref for isDragging to avoid re-renders
  const isDraggingRef = useRef(false);

  // PERF: Throttle onNodeDrag to prevent expensive helper line calculations from running on every frame
  const lastDragTimeRef = useRef<number>(0);
  const DRAG_THROTTLE_MS = 50; // Run at most every 50ms (20 fps for helper lines)

  const onNodeDrag = useCallback(
    (event: any, node: DiagramNode) => {
      // PERF: Completely skip all processing - just track position in ref
      // ReactFlow handles visual drag internally, we only need the final position
      dragPositionRef.current = { x: node.position.x, y: node.position.y };
      return;

      // The code below is kept for reference but not executed
      // Allow dragging for both piece and design nodes
      if (!isDraggingRef.current || !dragPositionRef.current || !reactFlowInstanceRef.current) return;

      // PERF: Skip ALL expensive calculations when alt is not pressed
      // This makes normal drag operations much faster - helper lines only show when alt is held
      if (!event.altKey) {
        // Just update position ref and return - ReactFlow handles visual drag internally
        dragPositionRef.current = { x: node.position.x, y: node.position.y };
        return;
      }

      // PERF: Throttle expensive helper line calculations
      const now = Date.now();
      if (now - lastDragTimeRef.current < DRAG_THROTTLE_MS) {
        // Still update node position but skip expensive helper line calculations
        if (node.type === "design") {
          // Just update position without helper lines
          return;
        }
      }
      lastDragTimeRef.current = now;

      const piece = node.data.piece as Piece;
      const MIN_DISTANCE = 150;
      const SNAP_THRESHOLD = 20;
      const lastPostition = dragPositionRef.current;
      if (!lastPostition || !reactFlowInstanceRef.current) return;

      const altPressed = event.altKey;

      const currentHelperLines: HelperLine[] = [];
      const nonSelectedNodes = nodes.filter((n) => !(selectionRef.current?.pieces ?? []).includes(getPieceIdFromNode(n)));
      const draggedCenterX = node.position.x + ICON_WIDTH / 2;
      const draggedCenterY = node.position.y + ICON_WIDTH / 2;

      const addedConnections: SemioConnection[] = [];
      const updatedPieces: Array<{ id: string; diff: any }> = [];

      let draggedX = node.position.x;
      let draggedY = node.position.y;

      for (const selectedNode of nodes.filter((n) => selectionRef.current?.pieces?.includes(getPieceIdFromNode(n)))) {
        const piece = selectedNode.data.piece;
        const selectedInternalNode = reactFlowInstanceRef.current!.getInternalNode(selectedNode.id)!;

        // Design nodes are moved without port snapping
        if (selectedNode.type === "design") {
          if (selectedNode.id === node.id) {
            selectedInternalNode.internals.positionAbsolute.x = draggedX;
            selectedInternalNode.internals.positionAbsolute.y = draggedY;
            node.position.x = draggedX;
            node.position.y = draggedY;
          }

          const scaledOffset = {
            x: (draggedX - lastPostition!.x) / ICON_WIDTH,
            y: -(draggedY - lastPostition!.y) / ICON_WIDTH,
          };
          updatedPieces.push({
            id: piece.guid,
            diff: {
              center: {
                u: (piece.center?.u ?? 0) + scaledOffset.x / ICON_WIDTH,
                v: (piece.center?.v ?? 0) - scaledOffset.y / ICON_WIDTH,
              },
            },
          });
          continue;
        }

        // Handle piece nodes with port snapping
        const type = (selectedNode as PieceNode).data.type;
        const fixedPieceId = metadata.get(piece.guid)?.fixedPieceId;
        let closestConnection: SemioConnection | null = null;
        let closestDistance = Number.MAX_VALUE;

        // PERF: Only run expensive O(n²) helper line calculation when alt is pressed
        // This calculation checks all pairs of non-selected nodes for alignment, which is O(n²)
        // With 180 pieces, that's ~16,000 comparisons per throttle period
        if (altPressed) {
          const EQUAL_DISTANCE_THRESHOLD = 15;
          let equalDistanceHelperLines: HelperLine[] = [];
          const displayedDistances = new Set<number>();

          for (let i = 0; i < nonSelectedNodes.length; i++) {
            for (let j = i + 1; j < nonSelectedNodes.length; j++) {
              const node1 = nonSelectedNodes[i];
              const node2 = nonSelectedNodes[j];

              const center1 = {
                x: node1.position.x + ICON_WIDTH / 2,
                y: node1.position.y + ICON_WIDTH / 2,
              };
              const center2 = {
                x: node2.position.x + ICON_WIDTH / 2,
                y: node2.position.y + ICON_WIDTH / 2,
              };

              if (Math.abs(center1.x - center2.x) < 5) {
                const distance = Math.abs(center2.y - center1.y);
                const minY = Math.min(center1.y, center2.y);
                const maxY = Math.max(center1.y, center2.y);
                const midY = (center1.y + center2.y) / 2;

                const isDistanceAlreadyDisplayed = Array.from(displayedDistances).some((existingDistance) => Math.abs(existingDistance - distance) < TOLERANCE);

                if (distance > 40 && !isDistanceAlreadyDisplayed) {
                  displayedDistances.add(distance);

                  if (Math.abs(draggedCenterY - midY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = midY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `upper-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `lower-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `mid-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: midY,
                        x2: center1.x + 30,
                        y2: midY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedMinY = minY - distance;
                  const extendedMaxY = maxY + distance;

                  if (Math.abs(draggedCenterY - extendedMinY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedMinY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: extendedMinY,
                        x2: center1.x + 30,
                        y2: extendedMinY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterY - extendedMaxY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedMaxY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: extendedMaxY,
                        x2: center1.x + 30,
                        y2: extendedMaxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedLeftX = center1.x - distance;
                  const extendedRightX = center1.x + distance;

                  if (Math.abs(draggedCenterX - extendedLeftX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedLeftX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-left-${node1.id}-${node2.id}`,
                        x1: extendedLeftX,
                        y1: midY - 30,
                        x2: extendedLeftX,
                        y2: midY + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: center1.x,
                        y1: midY - 50,
                        x2: center1.x,
                        y2: midY + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterX - extendedRightX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedRightX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-right-${node1.id}-${node2.id}`,
                        x1: extendedRightX,
                        y1: midY - 30,
                        x2: extendedRightX,
                        y2: midY + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: center1.x,
                        y1: midY - 50,
                        x2: center1.x,
                        y2: midY + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }
                }
              }

              if (Math.abs(center1.y - center2.y) < 5) {
                const distance = Math.abs(center2.x - center1.x);
                const minX = Math.min(center1.x, center2.x);
                const maxX = Math.max(center1.x, center2.x);
                const midX = (center1.x + center2.x) / 2;

                const isDistanceAlreadyDisplayed = Array.from(displayedDistances).some((existingDistance) => Math.abs(existingDistance - distance) < TOLERANCE);

                if (distance > 40 && !isDistanceAlreadyDisplayed) {
                  displayedDistances.add(distance);

                  if (Math.abs(draggedCenterX - midX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = midX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `left-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `right-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `mid-${node1.id}-${node2.id}`,
                        x1: midX,
                        y1: center1.y - 30,
                        x2: midX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedMinX = minX - distance;
                  const extendedMaxX = maxX + distance;

                  if (Math.abs(draggedCenterX - extendedMinX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedMinX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                        x1: extendedMinX,
                        y1: center1.y - 30,
                        x2: extendedMinX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterX - extendedMaxX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedMaxX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                        x1: extendedMaxX,
                        y1: center1.y - 30,
                        x2: extendedMaxX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedUpY = center1.y - distance;
                  const extendedDownY = center1.y + distance;

                  if (Math.abs(draggedCenterY - extendedUpY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedUpY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-up-${node1.id}-${node2.id}`,
                        x1: midX - 30,
                        y1: extendedUpY,
                        x2: midX + 30,
                        y2: extendedUpY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: midX - 50,
                        y1: center1.y,
                        x2: midX + 50,
                        y2: center1.y,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterY - extendedDownY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedDownY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-down-${node1.id}-${node2.id}`,
                        x1: midX - 30,
                        y1: extendedDownY,
                        x2: midX + 30,
                        y2: extendedDownY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        kind: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: midX - 50,
                        y1: center1.y,
                        x2: midX + 50,
                        y2: center1.y,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }
                }
              }
            }
          }

          const updatedDraggedCenterX = draggedX + ICON_WIDTH / 2;
          const updatedDraggedCenterY = draggedY + ICON_WIDTH / 2;

          for (const otherNode of nonSelectedNodes) {
            const centerY = otherNode.position.y + ICON_WIDTH / 2;
            const distance = Math.abs(updatedDraggedCenterY - centerY);
            if (distance < SNAP_THRESHOLD) {
              draggedY = centerY - ICON_WIDTH / 2;
              currentHelperLines.push({
                kind: "horizontal",
                position: centerY,
                relatedPieceId: otherNode.id,
              });
              break;
            }
          }

          for (const otherNode of nonSelectedNodes) {
            const centerX = otherNode.position.x + ICON_WIDTH / 2;
            const distance = Math.abs(updatedDraggedCenterX - centerX);
            if (distance < SNAP_THRESHOLD) {
              draggedX = centerX - ICON_WIDTH / 2;
              currentHelperLines.push({
                kind: "vertical",
                position: centerX,
                relatedPieceId: otherNode.id,
              });
              break;
            }
          }

          currentHelperLines.push(...equalDistanceHelperLines);

          // PERF: Helper lines disabled - setHelperLines(currentHelperLines);
        } else {
          // PERF: Helper lines disabled - setHelperLines([]);
        }

        if (selectedNode.id === node.id) {
          selectedInternalNode.internals.positionAbsolute.x = draggedX;
          selectedInternalNode.internals.positionAbsolute.y = draggedY;
          node.position.x = draggedX;
          node.position.y = draggedY;
        }

        if (!altPressed) {
          for (const otherNode of nodes.filter((n) => !(selection.pieces ?? []).includes(getPieceIdFromNode(n)))) {
            if (otherNode.type !== "piece") continue;
            const existingConnection = design?.connections?.find((c) =>
              areSameConnection(c, {
                guid: "",
                connected: { guid: "", piece: { guid: selectedNode.data.piece.guid }, port: { guid: "" } },
                connecting: { guid: "", piece: { guid: otherNode.data.piece.guid }, port: { guid: "" } },
              } as SemioConnection),
            );
            if (existingConnection) continue;
            if (!reactFlowInstanceRef.current) continue;
            const otherInternalNode = reactFlowInstanceRef.current!.getInternalNode(otherNode.id)!;
            for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
              if (!handle.id) {
                console.error("[ORIGIN] onNodeDrag: handle.id is undefined", { handle, selectedNode });
                continue;
              }
              const port = findPortInType(type, handle.id!);
              if (!port || !port.guid) {
                console.error("[ORIGIN] onNodeDrag: port or port.guid is undefined", { port, handleId: handle.id, type });
                continue;
              }
              for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
                if (!otherHandle.id) {
                  console.error("[ORIGIN] onNodeDrag: otherHandle.id is undefined", { otherHandle, otherNode });
                  continue;
                }
                const otherPort = findPortInType((otherNode as PieceNode).data.type, otherHandle.id!);
                if (!otherPort || !otherPort.guid) {
                  console.error("[ORIGIN] onNodeDrag: otherPort or otherPort.guid is undefined", { otherPort, otherHandleId: otherHandle.id, otherNode });
                  continue;
                }
                if (!selectedNode.data.piece.guid) {
                  console.error("[ORIGIN] onNodeDrag: selectedNode.data.piece.guid is undefined", { selectedNode });
                  continue;
                }
                if (!otherNode.data.piece.guid) {
                  console.error("[ORIGIN] onNodeDrag: otherNode.data.piece.guid is undefined", { otherNode });
                  continue;
                }
                const haveSameFixedPiece = fixedPieceId && fixedPieceId === metadata.get(otherNode.data.piece.guid)?.fixedPieceId;
                if (haveSameFixedPiece || !arePortsCompatible(port, otherPort) || (design && isPortInUse(design as Design, piece.guid, port.guid)) || (design && isPortInUse(design as Design, otherNode.data.piece.guid, otherPort.guid))) continue;
                const dx = selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x);
                const dy = selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y);
                const distance = Math.sqrt(dx * dx + dy * dy);
                if (distance < closestDistance && distance < MIN_DISTANCE) {
                  closestConnection = {
                    guid: crypto.randomUUID(),
                    connected: {
                      piece: { guid: otherNode.data.piece.guid },
                      port: { guid: otherHandle.id! },
                    },
                    connecting: {
                      piece: { guid: selectedNode.data.piece.guid },
                      port: { guid: handle.id! },
                    },
                    u: (selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x)) / ICON_WIDTH,
                    v: -((selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y)) / ICON_WIDTH),
                  };
                  closestDistance = distance;
                }
              }
            }
          }
        }

        if (closestConnection) {
          addedConnections.push(closestConnection!);
          updatedPieces.push({
            id: selectedNode.data.piece.guid,
            diff: {
              center: undefined,
              plane: undefined,
            },
          });
        } else {
          const scaledOffset = {
            x: (draggedX - lastPostition!.x) / ICON_WIDTH,
            y: -(draggedY - lastPostition!.y) / ICON_WIDTH,
          };
          updatedPieces.push({
            id: piece.guid,
            diff: {
              center: {
                u: (piece.center?.u ?? 0) + scaledOffset.x / ICON_WIDTH,
                v: (piece.center?.v ?? 0) - scaledOffset.y / ICON_WIDTH,
              },
            },
          });
        }
      }

      // PERF: Don't update Y.js during drag - let ReactFlow handle visual updates internally
      // Only update Y.js on drag stop to avoid ~47 re-render cycles during a typical drag
      if (addedConnections.length > 0) {
        addedConnections.forEach((conn) => addConnection("semio.sketchpad.app.design.canvas.diagram.onNodeDrag", conn));
      }
      // PERF: Update refs instead of state to avoid re-renders during drag
      dragPositionRef.current = { x: draggedX, y: draggedY };
      pendingPieceUpdatesRef.current = updatedPieces;
    },
    [addConnection, design, reactFlowInstanceRef, nodes, metadata],
  );

  const onNodeDragStop = useCallback(
    (event: any, node: DiagramNode) => {
      // PERF: All Y.js updates happen atomically at drag end
      // This prevents cascade re-renders during drag

      // Step 1: Apply pending selection (if any)
      const pendingSelection = pendingSelectionRef.current;
      if (pendingSelection) {
        const { pieceId, action } = pendingSelection;
        if (action === "select") selectPiece("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop", pieceId);
        else if (action === "add") addPieceToSelection("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop", pieceId);
        else if (action === "remove") removePieceFromSelection("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop", pieceId);
        pendingSelectionRef.current = null;
      }

      // Step 2: Start transaction for piece updates
      startTransaction("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop");

      // Step 3: Apply piece position updates
      const pendingUpdates = pendingPieceUpdatesRef.current;
      if (pendingUpdates && pendingUpdates.length > 0) {
        updatePieces("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop", pendingUpdates);
      } else if (node && selectionRef.current?.pieces?.length) {
        // If no pending updates but we have selection, calculate final positions
        const updatedPieces: Array<{ id: string; diff: any }> = [];
        for (const pieceId of selectionRef.current!.pieces!) {
          const pieceNode = nodes.find((n) => getPieceIdFromNode(n) === pieceId);
          if (pieceNode) {
            const newCenter = {
              u: pieceNode.position.x / ICON_WIDTH,
              v: -pieceNode.position.y / ICON_WIDTH,
            };
            updatedPieces.push({
              id: pieceId,
              diff: { center: newCenter },
            });
          }
        }
        if (updatedPieces.length > 0) {
          updatePieces("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop", updatedPieces);
        }
      }

      // Step 4: Finalize transaction
      finalizeTransaction("semio.sketchpad.app.design.canvas.diagram.onNodeDragStop");

      // Step 5: Reset drag state (refs, no re-render)
      isDraggingRef.current = false;
      isDraggingNode = false;
      dragPositionRef.current = null;
      pendingPieceUpdatesRef.current = [];
    },
    [finalizeTransaction, updatePieces, nodes, startTransaction, selectPiece, addPieceToSelection, removePieceFromSelection],
  );

  const onConnect = useCallback(
    (params: RFConnection) => {
      if (params.source === params.target || !reactFlowInstanceRef.current) return;

      const sourceInternalNode = reactFlowInstanceRef.current.getInternalNode(params.source);
      const targetInternalNode = reactFlowInstanceRef.current.getInternalNode(params.target);
      if (!sourceInternalNode || !targetInternalNode) return;

      const sourceHandle = (sourceInternalNode.internals.handleBounds?.source ?? []).find((h: any) => h.id === params.sourceHandle);
      const targetHandle = (targetInternalNode.internals.handleBounds?.source ?? []).find((h: any) => h.id === params.targetHandle);
      if (!sourceHandle || !targetHandle) return;

      const sourcePieceId = extractPieceIdFromNodeId(params.source!);
      const targetPieceId = extractPieceIdFromNodeId(params.target!);

      if (!sourcePieceId) {
        console.error("[ORIGIN] onConnect: sourcePieceId is undefined", { params, sourceInternalNode });
        return;
      }
      if (!targetPieceId) {
        console.error("[ORIGIN] onConnect: targetPieceId is undefined", { params, targetInternalNode });
        return;
      }
      if (!params.sourceHandle) {
        console.error("[ORIGIN] onConnect: params.sourceHandle is undefined", { params });
        return;
      }
      if (!params.targetHandle) {
        console.error("[ORIGIN] onConnect: params.targetHandle is undefined", { params });
        return;
      }

      const newConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connected: {
          piece: sourcePieceId,
          port: { guid: params.sourceHandle },
        },
        connecting: {
          piece: targetPieceId,
          port: { guid: params.targetHandle },
        },
        u: (sourceInternalNode.internals.positionAbsolute.x + sourceHandle.x - (targetInternalNode.internals.positionAbsolute.x + targetHandle.x)) / ICON_WIDTH,
        v: -((sourceInternalNode.internals.positionAbsolute.y + sourceHandle.y - (targetInternalNode.internals.positionAbsolute.y + targetHandle.y)) / ICON_WIDTH),
      };

      if (!design) return;
      if (((design as Design).connections ?? []).find((c: SemioConnection) => areSameConnection(c, newConnection))) return;
      addConnection("semio.sketchpad.app.design.canvas.diagram.onConnect", newConnection);
    },
    [addConnection, reactFlowInstanceRef, design],
  );

  return (
    <PieceRenderDataContext.Provider value={pieceRenderDataMap}>
      <SelectedPortContext.Provider value={selectedPort}>
        <div id="semio.sketchpad.app.design.canvas.diagram" data-diagram-id={diagramId} className="h-full w-full relative" ref={setDropZoneRef}>
          {/* PERF: CSS to disable pointer events on nodes during pan - prevents browser event processing overhead */}
          <style>{`
            [data-diagram-id="${diagramId}"][data-panning="true"] .react-flow__node,
            [data-diagram-id="${diagramId}"][data-panning="true"] .react-flow__edge {
              pointer-events: none !important;
            }
          `}</style>
          <Diagram
            nodes={nodes}
            edges={edges}
            onNodesChangeReactFlow={onNodesChangeReactFlow}
            nodeTypes={nodeComponents as NodeTypes}
            edgeTypes={edgeComponents as EdgeTypes}
            connectionMode="loose"
            connectionLineComponent={ConnectionConnectionLine}
            elementsSelectable={false}
            nodesFocusable={false}
            edgesFocusable={false}
            nodesDraggable={true}
            minZoom={0.1}
            defaultZoom={1}
            maxZoom={12}
            fitView={false}
            panOnDrag={[0]}
            zoomOnDoubleClick={false}
            onNodeClick={onNodeClick as any}
            onNodeDoubleClick={onNodeDoubleClick as any}
            onNodeMouseEnter={onNodeMouseEnter as any}
            onNodeMouseLeave={onNodeMouseLeave as any}
            onEdgeClick={onEdgeClick as any}
            onNodeDragStart={onNodeDragStart as any}
            onNodeDrag={onNodeDrag as any}
            onNodeDragStop={onNodeDragStop as any}
            onPaneClick={onPaneClick}
            onPaneDoubleClick={onDoubleClick}
            onMoveStart={onMoveStart}
            onMoveEnd={onMoveEnd}
            onConnect={onConnect}
            reactFlowInstanceRef={reactFlowInstanceRef}
            onInit={(instance) => {
              if (reactFlowInstanceRef) {
                reactFlowInstanceRef.current = instance;
              }
              const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
              if (diagramElement) {
                (diagramElement as any).__reactFlowInstance = instance;
              }
              // Center the viewport if no saved diagram center exists OR if it's at the default (0,0) origin
              const isAtDefaultOrigin = savedDiagramCenter && savedDiagramCenter.u === 0 && savedDiagramCenter.v === 0;
              if (!savedDiagramCenter || isAtDefaultOrigin) {
                isUpdatingViewportRef.current = true;
                setTimeout(() => {
                  centerViewport();
                  setTimeout(() => {
                    isUpdatingViewportRef.current = false;
                  }, 200);
                }, 100);
              }
            }}
            showControls={fullscreen && panelVisibility.toolbar}
            showMinimap={fullscreen && panelVisibility.toolbar}
            miniMapNodeComponent={MiniMapNode}
            focusedItemId={focusedItemId}
            onFocusComplete={() => setFocusedItemId(undefined)}
            panels={
              <>
                <ViewportPortal>
                  <div className="pointer-events-none">⌞</div>
                </ViewportPortal>
                {others.map((presence, idx) => (
                  <PresenceDiagram key={`presence-${idx}-${presence.name}-${presence.cursor?.u || 0}-${presence.cursor?.v || 0}`} {...presence} />
                ))}
              </>
            }
          />
          <HelperLines lines={helperLines} nodes={nodes} />
          {/* <ClusterMenu nodes={nodes} edges={edges} onCluster={onCluster} /> */}
          {/* <ExpandMenu nodes={nodes} edges={edges} onExpand={onExpand} /> */}
        </div>
      </SelectedPortContext.Provider>
    </PieceRenderDataContext.Provider>
  );
};

// #endregion Diagram

// #region Scene

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

const PresenceThree: FC<DesignAppPresenceOther> = ({ name, cursor, camera }) => {
  if (!camera) return null;
  const cameraHelper = useMemo(() => {
    const perspectiveCamera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1);
    perspectiveCamera.position.set(camera.position.x, camera.position.y, camera.position.z);
    perspectiveCamera.lookAt(new THREE.Vector3(camera.forward.x, camera.forward.y, camera.forward.z));
    perspectiveCamera.updateProjectionMatrix();
    perspectiveCamera.updateMatrixWorld();
    return new THREE.CameraHelper(perspectiveCamera);
  }, [camera.position.x, camera.position.y, camera.position.z, camera.forward.x, camera.forward.y, camera.forward.z]);

  return <primitive object={cameraHelper} />;
};

interface PlaneThreeProps {
  plane: Plane;
}

const PlaneThree: FC<PlaneThreeProps> = ({ plane }) => {
  const matrix = useMemo(() => planeToMatrix(plane), [plane]);
  return (
    <group matrix={matrix} matrixAutoUpdate={false}>
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 1, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
    </group>
  );
};

const GLTFMesh: FC<{ url: string }> = ({ url }) => {
  const gltf = useGLTF(url);
  const clonedScene = useMemo(() => {
    const cloned = gltf.scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [gltf.scene]);
  return <primitive object={clonedScene} />;
};

const FBXMesh: FC<{ url: string }> = ({ url }) => {
  const scene = useFBX(url);
  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [scene]);
  return <primitive object={clonedScene} />;
};

const OBJMesh: FC<{ url: string }> = ({ url }) => {
  const obj = useLoader(OBJLoader, url);
  const clonedScene = useMemo(() => {
    const cloned = obj.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [obj]);
  return <primitive object={clonedScene} />;
};

const LoadedPieceMesh: FC<{ url: string; fileExtension: string }> = ({ url, fileExtension }) => {
  const ext = fileExtension.toLowerCase();
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} />;
  } else {
    return <GLTFMesh url={url} />;
  }
};

const PieceMesh: FC = () => {
  const piece = usePiece() as Piece;
  const type = useType(undefined, typeof piece.type === "string" ? piece.type : piece.type?.guid) as Type | undefined;
  // Use targeted hook instead of deep kit subscription to avoid re-renders on unrelated kit changes
  const files = useKitFiles();
  const kitStore = useKitStore() as KitStore;
  const selectedModelTags = useDesignAppSelectedModelTags();
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { modelUrl, fileExtension, fileGuid } = useMemo(() => {
    if (!type?.models || type.models.length === 0) {
      console.warn("[PieceMesh] No models available for type:", type?.guid, type?.name);
      return { modelUrl: null, fileExtension: "", fileGuid: null };
    }
    const tagsForType = selectedModelTags[type.guid] ?? [];
    let model: Model | undefined;
    if (tagsForType.length > 0) {
      // Use manually selected tags with strict filtering
      model = selectBestModel(type.models, tagsForType);
      console.log("[PieceMesh] Selected model using manual tags:", model?.guid, "tags:", tagsForType);
    } else {
      // Use type's concepts as default tags for jaccard-based selection
      const conceptGuids = type.concepts?.map((c) => c.guid) ?? [];
      if (conceptGuids.length > 0) {
        // Use findModel directly (jaccard) instead of selectBestModel (which filters first)
        // This finds the model with highest jaccard similarity to the type's concepts
        model = findModel(type.models, conceptGuids);
        console.log("[PieceMesh] Selected model using type concepts:", model?.guid, "concepts:", conceptGuids);
      } else {
        // Fallback to default model (one with no tags) or first model
        const defaultRep = type.models.find((r) => !r.tags || r.tags.length === 0);
        model = defaultRep ?? type.models[0];
        console.log("[PieceMesh] Selected default/first model:", model?.guid);
      }
    }
    if (!model) {
      console.warn("[PieceMesh] No model found for type:", type.guid);
      return { modelUrl: null, fileExtension: "", fileGuid: null };
    }
    const fileId = typeof model.file === "string" ? model.file : model.file?.guid;
    const file = files.find((f) => f.guid === fileId);
    if (!file) {
      console.warn("[PieceMesh] File not found in kit for model:", model.guid, "file guid:", fileId);
      return { modelUrl: null, fileExtension: "", fileGuid: null };
    }
    const ext = file.name?.split(".").pop() || "";
    const url = kitStore.getFileUrl(file.guid);
    if (!url) {
      console.warn("[PieceMesh] File URL not available:", file.guid, file.name);
      return { modelUrl: null, fileExtension: ext, fileGuid: file.guid };
    }
    console.log("[PieceMesh] Model ready to load:", model.guid, "file:", file.name);
    return { modelUrl: url, fileExtension: ext, fileGuid: file.guid };
  }, [type, files, kitStore, selectedModelTags]);

  useEffect(() => {
    if (!fileGuid) {
      setBlobUrl(null);
      return;
    }
    let cancelled = false;
    let currentBlobUrl: string | null = null;
    (async () => {
      try {
        const url = await kitStore.getFileBlobUrl(fileGuid);
        if (!cancelled && url) {
          currentBlobUrl = url;
          setBlobUrl(url);
        }
      } catch (error) {
        console.error("[PieceMesh] Failed to get blob URL:", error);
      }
    })();
    return () => {
      cancelled = true;
      if (currentBlobUrl && currentBlobUrl.startsWith("blob:")) {
        URL.revokeObjectURL(currentBlobUrl);
      }
    };
  }, [fileGuid, kitStore]);

  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);

  if (!blobUrl) {
    // Render placeholder box when no model is loaded (same as design pieces)
    return (
      <mesh>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color={foregroundColor} emissive={foregroundColor} emissiveIntensity={0.45} />
        <Edges scale={1.001} color={foregroundColor} />
      </mesh>
    );
  }

  return (
    <Suspense fallback={null}>
      <LoadedPieceMesh url={blobUrl} fileExtension={fileExtension} />
    </Suspense>
  );
};

interface ModelPieceProps {}

const ModelPiece: FC<ModelPieceProps> = () => {
  const piece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const isSelected = useIsPieceSelected();
  const isHovered = useIsPieceTransitiveHovered();
  const status = usePieceStatus();
  const flatPlane = useFlatPiecePlane();

  const { selectPiece, removePieceFromSelection, addPieceToSelection, hoverPiece, clearHover, focusPiece } = useDesignAppCommands();

  const { fill } = useDesignAppPieceColor(undefined, piece.guid);

  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const mutedForegroundColor = useMemo(() => getComputedColor("--muted-foreground"), []);

  const originalPlane = piece.plane || flatPlane;
  const diffedPlane = diffedPiece.plane || flatPlane;

  const hasDiff = useMemo(() => {
    if (status === DiffStatus.Unchanged) return false;
    if (!originalPlane || !diffedPlane) return false;

    const p1 = originalPlane;
    const p2 = diffedPlane;
    return (
      p1.origin.x !== p2.origin.x ||
      p1.origin.y !== p2.origin.y ||
      p1.origin.z !== p2.origin.z ||
      p1.xAxis.x !== p2.xAxis.x ||
      p1.xAxis.y !== p2.xAxis.y ||
      p1.xAxis.z !== p2.xAxis.z ||
      p1.yAxis.x !== p2.yAxis.x ||
      p1.yAxis.y !== p2.yAxis.y ||
      p1.yAxis.z !== p2.yAxis.z
    );
  }, [status, originalPlane, diffedPlane]);

  const onSelect = useCallback(
    (e?: ThreeEvent<MouseEvent>) => {
      if (e?.ctrlKey || e?.metaKey) {
        removePieceFromSelection("semio.sketchpad.app.design.canvas.scene.modelPiece.removePieceFromSelection", piece.guid);
      } else if (e?.shiftKey) {
        addPieceToSelection("semio.sketchpad.app.design.canvas.scene.modelPiece.addPieceToSelection", piece.guid);
      } else {
        selectPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.selectPiece", piece.guid);
      }
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection, piece.guid],
  );

  const onDoubleClick = useCallback(
    (e?: ThreeEvent<MouseEvent>) => {
      e?.stopPropagation();
      focusPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.focusPiece", piece.guid);
    },
    [focusPiece, piece.guid],
  );

  const handlePointerEnter = useCallback(() => {
    if (currentHoveredPieceGuid !== piece.guid) {
      currentHoveredPieceGuid = piece.guid;
      hoverPiece("semio.sketchpad.app.design.canvas.scene.modelPiece.hoverPiece", piece.guid);
    }
  }, [piece.guid, hoverPiece]);

  const handlePointerLeave = useCallback(() => {
    if (currentHoveredPieceGuid === piece.guid) {
      currentHoveredPieceGuid = null;
      clearHover("semio.sketchpad.app.design.canvas.scene.modelPiece.clearHover");
    }
  }, [piece.guid, clearHover]);

  const materialColor = useMemo(() => {
    const tempDiv = document.createElement("div");
    tempDiv.style.position = "absolute";
    tempDiv.style.visibility = "hidden";
    tempDiv.style.pointerEvents = "none";
    document.body.appendChild(tempDiv);
    tempDiv.style.color = fill;
    const computedColor = getComputedStyle(tempDiv).color;
    document.body.removeChild(tempDiv);

    const canvas = document.createElement("canvas");
    canvas.width = 1;
    canvas.height = 1;
    const ctx = canvas.getContext("2d");
    if (!ctx) return computedColor;

    ctx.fillStyle = computedColor;
    ctx.fillRect(0, 0, 1, 1);
    const imageData = ctx.getImageData(0, 0, 1, 1);
    const [r, g, b, a] = imageData.data;

    if (a === 0) {
      return foregroundColor;
    }

    return `rgb(${r}, ${g}, ${b})`;
  }, [fill, foregroundColor]);
  const emissiveColor = materialColor;

  const originalMatrix = useMemo(() => {
    if (!originalPlane) return null;
    const planeMatrix = planeToMatrix(originalPlane as Plane);
    const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
    return threeMatrix;
  }, [originalPlane]);

  const diffedMatrix = useMemo(() => {
    if (!diffedPlane || !hasDiff) return null;
    const planeMatrix = planeToMatrix(diffedPlane as Plane);
    const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
    return threeMatrix;
  }, [diffedPlane, hasDiff]);

  const transformProps = useMemo(() => {
    const matrix = diffedMatrix || originalMatrix;
    if (!matrix || !originalPlane) return null;
    const position = new THREE.Vector3();
    const quaternion = new THREE.Quaternion();
    const scale = new THREE.Vector3();
    matrix.decompose(position, quaternion, scale);
    return { position, quaternion, scale };
  }, [diffedMatrix, originalMatrix, originalPlane]);

  const originalMeshContent =
    hasDiff && originalMatrix ? (
      <group matrix={originalMatrix} matrixAutoUpdate={false}>
        <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial transparent opacity={0} />
          <Edges scale={1.001} color={mutedForegroundColor} />
        </mesh>
      </group>
    ) : null;

  const userData = useMemo(() => ({ id: piece.guid }), [piece.guid]);

  const diffedMeshContent = piece.design ? (
    <Geometry
      selected={isSelected}
      hovered={isHovered}
      onClick={onSelect}
      onDoubleClick={onDoubleClick}
      onPointerEnter={handlePointerEnter}
      onPointerLeave={handlePointerLeave}
      color={materialColor}
      emissiveColor={emissiveColor}
      emissiveIntensity={0.45}
      showEdges
      edgeColor={foregroundColor}
      userData={userData}
    />
  ) : (
    <group onClick={onSelect} onDoubleClick={onDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      <PieceMesh />
    </group>
  );

  const pieceMatrix = diffedMatrix || originalMatrix;

  return (
    <>
      {originalMeshContent}
      {pieceMatrix && (
        <group matrix={pieceMatrix} matrixAutoUpdate={false}>
          {diffedMeshContent}
        </group>
      )}
    </>
  );
};

const ModelDesign: FC = () => {
  const commands = useDesignAppCommands();
  const [selection] = useDesignAppSelection();
  const [others] = useDesignAppOthers();
  const design = useDesign();
  const flatDesign = design as Design;

  const { selectPieces, startTransaction, finalizeTransaction, updatePiece } = commands;

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId).filter(Boolean);
      if (newSelectedPieceIds.length !== selection.pieces?.length || newSelectedPieceIds.some((id, index) => id !== selection.pieces?.[index])) {
        selectPieces("semio.sketchpad.app.design.canvas.scene.modelDesign.selectPieces", newSelectedPieceIds);
      }
    },
    [selectPieces, selection.pieces],
  );

  // TransformableModel represents a piece model that can be transformed in the scene
  type TransformableModel = { guid: string; plane: Plane | undefined; isTransformable: boolean; isSelected: boolean };
  const selectedModels = useMemo((): TransformableModel[] => {
    if (!selection.pieces || !flatDesign.pieces) return [];

    return flatDesign.pieces
      .filter((piece) => selection.pieces?.includes(piece.guid))
      .map((piece) => ({
        guid: piece.guid,
        plane: piece.plane,
        isTransformable: !piece.isLocked && piece.plane !== undefined,
        isSelected: true,
      }));
  }, [selection.pieces, flatDesign.pieces]);

  const handleMultiPlaneUpdate = useCallback(
    (updates: Array<{ modelGuid: string; newPlane: Plane }>) => {
      updates.forEach(({ modelGuid, newPlane }) => {
        updatePiece("semio.sketchpad.app.design.canvas.scene.modelDesign.updatePiece", modelGuid, { plane: newPlane });
      });
    },
    [updatePiece],
  );

  return (
    <>
      <Select box multiple onChange={onChange}>
        <group>
          {flatDesign.pieces?.map((piece: Piece) => (
            <PieceScopeProvider key={piece.guid} guid={piece.guid}>
              <ModelPiece />
            </PieceScopeProvider>
          ))}
          {others.map((presence, id) => (
            <PresenceThree key={id} {...presence} />
          ))}
        </group>
      </Select>
    </>
  );
};

const DesignAppScene: FC = () => {
  const { deselectAll, toggleAccesslFullscreen, setCamera, clearFocus, addPiece, startTransaction, finalizeTransaction } = useDesignAppCommands();
  const [fullscreenValue] = useDesignAppFullscreen();
  const fullscreen = fullscreenValue === DesignAppFullscreenWindow.Accessl;
  const [camera] = useDesignAppCamera();
  const [focusedPieceGuid] = useDesignAppFocusedPieceGuid();
  const panelVisibility = useAppPanelVisibility();
  const [projection, setProjection] = React.useState<"camera" | "orthographic">("orthographic");
  // PERF: Use targeted hooks to avoid re-renders during scene interactions
  const sceneTypes = useKitTypes();
  const sceneDesigns = useKitDesigns();
  const { setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();
  const sceneDropZoneRef = useRef<HTMLDivElement | null>(null);
  const sceneId = useRef(guid()).current;

  const handleSceneDragEnd = useCallback(
    (event: DragEndEvent) => {
      const { active, delta } = event;
      if (!sceneDropZoneRef.current) return;
      if (!(event.activatorEvent instanceof PointerEvent)) return;
      const dropX = event.activatorEvent.clientX + delta.x;
      const dropY = event.activatorEvent.clientY + delta.y;
      const dropZoneBounds = sceneDropZoneRef.current.getBoundingClientRect();
      const isWithinBounds = dropX >= dropZoneBounds.left && dropX <= dropZoneBounds.right && dropY >= dropZoneBounds.top && dropY <= dropZoneBounds.bottom;
      if (!isWithinBounds) return;
      const dragData = active.data.current as { type: string; typeGuid?: string; designGuid?: string } | undefined;
      if (!dragData) return;
      const localX = dropX - dropZoneBounds.left;
      const localY = dropY - dropZoneBounds.top;
      const ndcX = (localX / dropZoneBounds.width) * 2 - 1;
      const ndcY = -(localY / dropZoneBounds.height) * 2 + 1;
      const zoom = 50;
      const camPos = camera?.position ?? { x: 10, y: 10, z: 10 };
      const camForward = camera?.forward ?? { x: -1, y: -1, z: -1 };
      const camUp = camera?.up ?? { x: 0, y: 1, z: 0 };
      const fwdLen = Math.sqrt(camForward.x ** 2 + camForward.y ** 2 + camForward.z ** 2);
      const fwd = { x: camForward.x / fwdLen, y: camForward.y / fwdLen, z: camForward.z / fwdLen };
      const rightX = camUp.y * fwd.z - camUp.z * fwd.y;
      const rightY = camUp.z * fwd.x - camUp.x * fwd.z;
      const rightZ = camUp.x * fwd.y - camUp.y * fwd.x;
      const rightLen = Math.sqrt(rightX ** 2 + rightY ** 2 + rightZ ** 2);
      const right = { x: rightX / rightLen, y: rightY / rightLen, z: rightZ / rightLen };
      const upX = fwd.y * right.z - fwd.z * right.y;
      const upY = fwd.z * right.x - fwd.x * right.z;
      const upZ = fwd.x * right.y - fwd.y * right.x;
      const upLen = Math.sqrt(upX ** 2 + upY ** 2 + upZ ** 2);
      const actualUp = { x: upX / upLen, y: upY / upLen, z: upZ / upLen };
      const halfWidth = dropZoneBounds.width / (2 * zoom);
      const halfHeight = dropZoneBounds.height / (2 * zoom);
      const rayOrigin = {
        x: camPos.x + right.x * ndcX * halfWidth + actualUp.x * ndcY * halfHeight,
        y: camPos.y + right.y * ndcX * halfWidth + actualUp.y * ndcY * halfHeight,
        z: camPos.z + right.z * ndcX * halfWidth + actualUp.z * ndcY * halfHeight,
      };
      const t = Math.abs(fwd.y) > 0.0001 ? -rayOrigin.y / fwd.y : 0;
      const worldX = rayOrigin.x + fwd.x * t;
      const worldZ = rayOrigin.z + fwd.z * t;
      const plane: Plane = { origin: { x: worldX, y: 0, z: worldZ }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
      const center = { u: worldX * 0.3 + 6, v: worldZ * 0.3 - 7 };
      if (dragData.type === "type" && dragData.typeGuid) {
        const droppedType = sceneTypes?.find((t) => t.guid === dragData.typeGuid);
        if (!droppedType) return;
        startTransaction("semio.sketchpad.app.design.scene.dragEnd.type");
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, type: { guid: droppedType.guid }, plane, center };
        addPiece("semio.sketchpad.app.design.scene.dragEnd.type", piece);
        finalizeTransaction("semio.sketchpad.app.design.scene.dragEnd.type");
      } else if (dragData.type === "design" && dragData.designGuid) {
        const droppedDesign = sceneDesigns?.find((d) => d.guid === dragData.designGuid);
        if (!droppedDesign) return;
        startTransaction("semio.sketchpad.app.design.scene.dragEnd.design");
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, design: { guid: droppedDesign.guid }, plane, center };
        addPiece("semio.sketchpad.app.design.scene.dragEnd.design", piece);
        finalizeTransaction("semio.sketchpad.app.design.scene.dragEnd.design");
      }
      setActiveDraggedType(null);
      setActiveDraggedDesign(null);
    },
    [sceneTypes, sceneDesigns, camera, startTransaction, addPiece, finalizeTransaction, setActiveDraggedType, setActiveDraggedDesign],
  );

  useEffect(() => {
    const listener = (e: Event) => {
      const customEvent = e as CustomEvent<DragEndEvent>;
      handleSceneDragEnd(customEvent.detail);
    };
    window.addEventListener("design-drag-end", listener);
    return () => window.removeEventListener("design-drag-end", listener);
  }, [handleSceneDragEnd]);

  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      toggleAccesslFullscreen("semio.sketchpad.app.design.canvas.scene.doubleClickCapture");
    },
    [toggleAccesslFullscreen],
  );
  const onPointerMissed = useCallback(
    (e: MouseEvent) => {
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll("semio.sketchpad.app.design.canvas.scene.pointerMissed");
    },
    [deselectAll],
  );
  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      setCamera("semio.sketchpad.app.design.canvas.scene.cameraChange", newCamera);
    },
    [setCamera],
  );
  const onFocusComplete = useCallback(() => {
    setTimeout(() => {
      clearFocus("semio.sketchpad.app.design.canvas.scene.focusComplete");
    }, 100);
  }, [clearFocus]);

  return (
    <div ref={sceneDropZoneRef} data-drop-zone="scene" data-drop-zone-id={sceneId} className="h-full w-full">
      <Scene
        showGizmo={fullscreen && !!panelVisibility.toolbar}
        camera={camera}
        onCameraChange={onCameraChange}
        onDoubleClickCapture={onDoubleClickCapture}
        onPointerMissed={onPointerMissed}
        focusedItemId={focusedPieceGuid}
        onFocusComplete={onFocusComplete}
        orthographic={projection === "orthographic"}
        projection={projection}
        onProjectionChange={setProjection}
      >
        <ModelDesign />
      </Scene>
    </div>
  );
};

// #endregion Scene

// #endregion Canvas

export interface AppProps {}

const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => {
  return <DesignDiagram reactFlowInstanceRef={reactFlowInstanceRef} />;
});
DiagramWindow.displayName = "DiagramWindow";

const SceneWindow = memo(() => {
  return <DesignAppScene />;
});
SceneWindow.displayName = "SceneWindow";

const renderCountRef = { current: 0 };

const App: FC<AppProps> = () => {
  renderCountRef.current++;
  // Sync Y.js state to XState on mount and changes
  useDesignAppYjsToXStateSyncInternal();

  const { t } = useTranslation();
  const { selectAll, deselectAll, deleteSelected, undo, redo, toggleDiagramFullscreen, toggleAccesslFullscreen, addPiece, startTransaction, finalizeTransaction, togglePanel, setActiveTool, hoverTypes, hoverDesigns, clearHover } =
    useDesignAppCommands();
  // PERF: Use targeted hook instead of full state subscription
  const [activeTool] = useDesignAppActiveTool();

  const [selection] = useDesignAppSelection();
  const design = useDesign() as Design | undefined;
  // PERF: Use targeted hooks - only get what we need to avoid re-renders
  const kitGuid = useKitScope()?.guid;
  const workbenchTypes = useKitTypes();
  const workbenchDesigns = useKitDesigns();
  const appSettings = useSketchpad((s) => s.settings?.apps) as any;
  const panelVisibility = useAppPanelVisibility();
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const store = useDesignAppController() as DesignAppController | null;
  const storedWindowLayout = useDesignApp((s) => s.windowLayout);

  const defaultLayout = useMemo(() => {
    return {
      root: {
        type: "row",
        content: [
          {
            type: "stack",
            size: "50%",
            content: [
              {
                type: "component",
                componentName: DesignAppWindowKind.Diagram,
                title: "diagram",
                componentState: {},
              },
            ],
          },
          {
            type: "stack",
            size: "50%",
            content: [
              {
                type: "component",
                componentName: DesignAppWindowKind.Scene,
                title: "scene",
                componentState: {},
              },
            ],
          },
        ],
      },
    };
  }, []);

  // Validate layout has Scene window - if not, use undefined to trigger default
  const windowLayout = useMemo(() => {
    if (!storedWindowLayout) {
      return storedWindowLayout;
    }

    const hasSceneWindow = (layout: any): boolean => {
      if (!layout) return false;

      // Check if this is a component with Scene
      if (layout.type === "component" && layout.componentName === DesignAppWindowKind.Scene) return true;

      // Check GoldenLayout root structure
      if (layout.root && typeof layout.root === "object") {
        return hasSceneWindow(layout.root);
      }

      // Check content arrays (both regular and GoldenLayout contentItems)
      if (layout.content && Array.isArray(layout.content)) {
        return layout.content.some((item: any) => hasSceneWindow(item));
      }
      if (layout.contentItems && Array.isArray(layout.contentItems)) {
        return layout.contentItems.some((item: any) => hasSceneWindow(item));
      }

      return false;
    };

    const hasScene = hasSceneWindow(storedWindowLayout);
    if (!hasScene) {
      return undefined;
    }

    return storedWindowLayout;
  }, [storedWindowLayout]);

  // Clear invalid layout from store
  useEffect(() => {
    if (store && storedWindowLayout && windowLayout === undefined) {
      // Use try-catch to ensure it doesn't fail silently
      try {
        store.change({ windowLayout: undefined });
      } catch (error) {
        console.error("[DesignApp] Failed to clear layout:", error);
      }
    }
  }, [store, storedWindowLayout, windowLayout]);

  const windowConfig: AppWindowConfig = useMemo(() => {
    return {
      windowKinds: [
        {
          id: DesignAppWindowKind.Diagram,
          label: "diagram",
          component: (props: any) => <DiagramWindow reactFlowInstanceRef={reactFlowInstanceRef} />,
        },
        {
          id: DesignAppWindowKind.Scene,
          label: "scene",
          component: (props: any) => <SceneWindow />,
        },
      ],
      defaultLayout,
    };
  }, [defaultLayout, reactFlowInstanceRef]);

  const handleLayoutChange = useCallback(
    (config: any) => {
      if (store && typeof store.change === "function") {
        store.change({ windowLayout: config });
      }
    },
    [store],
  );

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const { useKitAppCommands } = getKitAppHooks();
  const kitAppCommands = useKitAppCommands();
  const sketchpadCommands = useSketchpadCommands();
  const { navigateToType, navigateToDesign, navigateToKit } = sketchpadCommands;

  useHotkeys("ctrl+a", () => selectAll("semio.sketchpad.app.design.hotkey.ctrlA"), { enableOnFormTags: true });
  useHotkeys("ctrl+d", () => deselectAll("semio.sketchpad.app.design.hotkey.ctrlD"), { enableOnFormTags: true });
  useHotkeys("delete", () => deleteSelected("semio.sketchpad.app.design.hotkey.delete"), { enableOnFormTags: true });
  useHotkeys("ctrl+z", () => undo("semio.sketchpad.app.design.hotkey.ctrlZ"), { enableOnFormTags: true });
  useHotkeys("ctrl+y", () => redo("semio.sketchpad.app.design.hotkey.ctrlY"), { enableOnFormTags: true });
  useHotkeys("ctrl+shift+z", () => redo("semio.sketchpad.app.design.hotkey.ctrlShiftZ"), { enableOnFormTags: true });

  const appType = useAppType();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          setActiveTool("semio.sketchpad.app.design.keyboard.shift", ToolKind.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          setActiveTool("semio.sketchpad.app.design.keyboard.ctrl", ToolKind.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_ADDITIVE && !e.shiftKey) {
        setActiveTool("semio.sketchpad.app.design.keyboard.shiftUp", ToolKind.SELECTION_NORMAL);
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        setActiveTool("semio.sketchpad.app.design.keyboard.ctrlUp", ToolKind.SELECTION_NORMAL);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  // Add/remove details panel sections based on selection
  useEffect(() => {
    if (appType !== "design") return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.port !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    const pieceSingleId = "semio.sketchpad.app.design.panel.details.section.piece.properties";
    const pieceMultipleId = "semio.sketchpad.app.design.panel.details.section.piece.multipleTitle";
    const connectionSingleId = "semio.sketchpad.app.design.panel.details.section.connection.properties";
    const connectionMultipleId = "semio.sketchpad.app.design.panel.details.section.connection.multipleTitle";
    const selectionMultipleId = "semio.sketchpad.app.design.panel.details.section.selection.multipleTitle";

    removeSection("details", "semio.sketchpad.app.design.properties");
    removeSection("details", "semio.sketchpad.app.type.port.properties");
    removeSection("details", pieceSingleId);
    removeSection("details", pieceMultipleId);
    removeSection("details", connectionSingleId);
    removeSection("details", connectionMultipleId);
    removeSection("details", selectionMultipleId);
    removeSection("details", "semio.sketchpad.app.kit.properties");

    if (!hasSelection) {
      addSection("details", {
        id: "semio.sketchpad.app.design.properties",
        specificity: 20,
        order: 50,
        content: () =>
          design ? (
            <DesignScopeProvider guid={design.guid}>
              <DesignSection />
            </DesignScopeProvider>
          ) : null,
      });
    } else if (hasPortSelected) {
      const portPieceId = selection.port!.piece;
      const portId = selection.port!.port;
      addSection("details", {
        id: "semio.sketchpad.app.type.port.properties",
        specificity: 30,
        order: 0,
        content: () => <PortSection pieceGuid={portPieceId} portGuid={portId} />,
      });
      addSection("details", {
        id: "semio.sketchpad.app.design.properties",
        specificity: 20,
        order: 50,
        content: () =>
          design ? (
            <DesignScopeProvider guid={design.guid}>
              <DesignSection />
            </DesignScopeProvider>
          ) : null,
      });
    } else {
      if (hasPieces) {
        const piecesCount = selection.pieces!.length;
        const piecesSectionId = piecesCount === 1 ? pieceSingleId : pieceMultipleId;
        addSection("details", {
          id: piecesSectionId,
          specificity: 30,
          order: 0,
          content: () => <PiecesSection />,
        });
      }
      if (hasConnections) {
        const connGuids = selection.connections!;
        const conns = findConnectionsInDesign(design!, connGuids);
        const connectionsSectionId = conns.length === 1 ? connectionSingleId : connectionMultipleId;
        addSection("details", {
          id: connectionsSectionId,
          specificity: 30,
          order: 10,
          content: () => <ConnectionsSection connections={conns} isSingle={conns.length === 1} count={conns.length} />,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: selectionMultipleId,
          specificity: 30,
          order: 20,
          content: () => (
            <TreeItem>
              <TreeContent>
                <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.selectOnlyPiecesOrConnections")}</p>
              </TreeContent>
            </TreeItem>
          ),
        });
      }
      addSection("details", {
        id: "semio.sketchpad.app.design.properties",
        specificity: 20,
        order: 50,
        content: () =>
          design ? (
            <DesignScopeProvider guid={design.guid}>
              <DesignSection />
            </DesignScopeProvider>
          ) : null,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.kit.properties",
      specificity: 10,
      order: 100,
      content: () =>
        kitGuid ? (
          <React.Suspense fallback={null}>
            <KitScopeProvider guid={kitGuid}>
              <KitSectionLazy />
            </KitScopeProvider>
          </React.Suspense>
        ) : null,
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.design.properties");
      removeSection("details", "semio.sketchpad.app.type.port.properties");
      removeSection("details", pieceSingleId);
      removeSection("details", pieceMultipleId);
      removeSection("details", connectionSingleId);
      removeSection("details", connectionMultipleId);
      removeSection("details", selectionMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.properties");
    };
  }, [selection, addSection, removeSection, appType, t, design]);

  const PiecesWorkbenchContent: FC = () => {
    const handleCreateTypeChild = (parentType: Type) => {
      const existingChildren = workbenchTypes?.filter((type) => type.parent?.guid === parentType.guid) || [];
      const uniqueName = generateUniqueName(
        parentType.name,
        existingChildren.map((type) => type.name),
      );
      const newType: Type = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: parentType.guid },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addType("semio.sketchpad.app.design.panel.workbench.types.createChild", newType);
      if (kitGuid) navigateToType(kitGuid, newType.guid);
    };

    const handleCreateDesignChild = (parentDesign: Design) => {
      const existingChildren = workbenchDesigns?.filter((design) => design.parent?.guid === parentDesign.guid) || [];
      const uniqueName = generateUniqueName(
        parentDesign.name,
        existingChildren.map((design) => design.name),
      );
      const newDesign: Design = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: parentDesign.guid },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addDesign("semio.sketchpad.app.design.panel.workbench.designs.createChild", newDesign);
      if (kitGuid) navigateToDesign(kitGuid, newDesign.guid);
    };

    const renderTypeTree = (types: Type[]): ReactNode[] => {
      return types.map((type) => {
        const children = workbenchTypes?.filter((item) => (typeof item.parent === "object" ? item.parent?.guid === type.guid : item.parent === type.guid)) || [];
        return (
          <div key={type.guid} onPointerEnter={() => hoverTypes("semio.sketchpad.app.design.panel.workbench.types.hover", [type.guid])} onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.types.leave")}>
            <TypeTreeItem type={type} onCreateChild={handleCreateTypeChild}>
              {children.length > 0 && renderTypeTree(children)}
            </TypeTreeItem>
          </div>
        );
      });
    };

    const renderDesignTree = (designs: Design[]): ReactNode[] => {
      return designs.map((design) => {
        const children = workbenchDesigns?.filter((child) => (typeof child.parent === "object" ? child.parent?.guid === design.guid : child.parent === design.guid)) || [];
        return (
          <div key={design.guid} onPointerEnter={() => hoverDesigns("semio.sketchpad.app.design.panel.workbench.designs.hover", [design.guid])} onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.designs.leave")}>
            <DesignTreeItem design={design} onCreateChild={handleCreateDesignChild}>
              {children.length > 0 && renderDesignTree(children)}
            </DesignTreeItem>
          </div>
        );
      });
    };

    const rootTypes = workbenchTypes?.filter((type) => !type.parent) || [];
    const rootDesigns = workbenchDesigns?.filter((design) => !design.parent) || [];

    const handleCreateType = () => {
      const existingTypes = workbenchTypes || [];
      const typeNumber = existingTypes.length + 1;
      const newType: Type = {
        guid: guid(),
        name: `Type ${typeNumber}`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addType("semio.sketchpad.app.design.panel.workbench.types.create", newType);
      if (kitGuid) navigateToType(kitGuid, newType.guid);
    };

    const handleCreateDesign = () => {
      const existingDesigns = workbenchDesigns || [];
      const designNumber = existingDesigns.length + 1;
      const newDesign: Design = {
        guid: guid(),
        name: `Design ${designNumber}`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addDesign("semio.sketchpad.app.design.panel.workbench.designs.create", newDesign);
      if (kitGuid) navigateToDesign(kitGuid, newDesign.guid);
    };

    return (
      <>
        <div
          onPointerEnter={() => {
            if (!workbenchTypes || workbenchTypes.length === 0) return;
            hoverTypes(
              "semio.sketchpad.app.design.panel.workbench.typesSection.hover",
              workbenchTypes.map((type) => type.guid),
            );
          }}
          onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.typesSection.leave")}
        >
          <TreeItem
            id="semio.sketchpad.app.kit.types"
            actions={[
              {
                id: "semio.sketchpad.common.addType",
                icon: <AddIcon size={12} />,
                onClick: handleCreateType,
              },
            ]}
            onDoubleClick={() => {
              if (!kitGuid) return;
              navigateToKit(kitGuid, "kind=types");
            }}
          >
            {renderTypeTree(rootTypes)}
          </TreeItem>
        </div>
        <div
          onPointerEnter={() => {
            if (!workbenchDesigns || workbenchDesigns.length === 0) return;
            hoverDesigns(
              "semio.sketchpad.app.design.panel.workbench.designsSection.hover",
              workbenchDesigns.map((design) => design.guid),
            );
          }}
          onPointerLeave={() => clearHover("semio.sketchpad.app.design.panel.workbench.designsSection.leave")}
        >
          <TreeItem
            id="semio.sketchpad.app.kit.designs"
            actions={[
              {
                id: "semio.sketchpad.common.addDesign",
                icon: <AddIcon size={12} />,
                onClick: handleCreateDesign,
              },
            ]}
            onDoubleClick={() => {
              if (!kitGuid) return;
              navigateToKit(kitGuid, "kind=designs");
            }}
          >
            {renderDesignTree(rootDesigns)}
          </TreeItem>
        </div>
      </>
    );
  };

  const TypeTreeItem: FC<{ type: Type; onCreateChild: (type: Type) => void; children?: ReactNode }> = ({ type, onCreateChild, children }) => {
    const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
      id: `type-${type.guid}`,
      data: { type: "type", typeGuid: type.guid },
    });

    const handleDragStart = () => {
      setActiveDraggedType(type);
    };

    useEffect(() => {
      if (isDragging) {
        handleDragStart();
      }
    }, [isDragging]);

    return (
      <TreeItem
        label={
          <div className="flex items-center gap-single min-w-0">
            <DraggableAvatar
              ref={setNodeRef}
              dragRef={setNodeRef}
              dragListeners={listeners}
              dragAttributes={attributes}
              content={type.name.substring(0, 2).toUpperCase()}
              isSelected={false}
              isHovered={false}
              shouldFade={isDragging}
              title={type.name}
            />
            <span className="truncate">{type.name}</span>
          </div>
        }
        onDoubleClick={(event) => {
          if ((event.target as HTMLElement).closest('[data-slot="action"]') || (event.target as HTMLElement).closest('[data-slot="avatar"]')) {
            return;
          }
          event.preventDefault();
          event.stopPropagation();
          if (kitGuid) navigateToType(kitGuid, type.guid);
        }}
        actions={[
          {
            icon: <AddIcon size={12} />,
            onClick: () => onCreateChild(type),
            id: "semio.sketchpad.common.addChild",
          },
        ]}
      >
        {children}
      </TreeItem>
    );
  };

  const DesignTreeItem: FC<{ design: Design; onCreateChild: (design: Design) => void; children?: ReactNode }> = ({ design, onCreateChild, children }) => {
    const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
      id: `design-${design.guid}`,
      data: { type: "design", designGuid: design.guid },
    });

    const handleDragStart = () => {
      setActiveDraggedDesign(design);
    };

    useEffect(() => {
      if (isDragging) {
        handleDragStart();
      }
    }, [isDragging]);

    return (
      <TreeItem
        label={
          <div className="flex items-center gap-single min-w-0">
            <DraggableAvatar
              ref={setNodeRef}
              dragRef={setNodeRef}
              dragListeners={listeners}
              dragAttributes={attributes}
              content={design.name.substring(0, 2).toUpperCase()}
              isSelected={false}
              isHovered={false}
              shouldFade={isDragging}
              title={design.name}
            />
            <span className="truncate">{design.name}</span>
          </div>
        }
        onDoubleClick={(event) => {
          if ((event.target as HTMLElement).closest('[data-slot="action"]') || (event.target as HTMLElement).closest('[data-slot="avatar"]')) {
            return;
          }
          event.preventDefault();
          event.stopPropagation();
          if (kitGuid) navigateToDesign(kitGuid, design.guid);
        }}
        actions={[
          {
            icon: <AddIcon size={12} />,
            onClick: () => onCreateChild(design),
            id: "semio.sketchpad.common.addChild",
          },
        ]}
      >
        {children}
      </TreeItem>
    );
  };

  // Toolbar section is now registered in DesignApp component for earlier initialization

  useEffect(() => {
    if (appType !== "design") return;

    addSection("workbench", {
      id: "semio.sketchpad.app.kit.pieces",
      specificity: 20,
      order: 0,
      content: () => <PiecesWorkbenchContent />,
    });

    addSection("workbench", {
      id: "semio.sketchpad.app.design.windows",
      specificity: 20,
      order: 1,
      content: () => <WindowLibrary />,
    });

    return () => {
      removeSection("workbench", "semio.sketchpad.app.kit.pieces");
      removeSection("workbench", "semio.sketchpad.app.design.windows");
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [appType, kitGuid, workbenchTypes?.length, workbenchDesigns?.length]);

  // Add settings sections
  useEffect(() => {
    const { setTheme, setLanguage, setLayout, setExpertise, setMode } = sketchpadCommands;

    const SketchpadSettingsContent = () => {
      const theme = useTheme();
      const language = useLanguage();
      const layout = useLayout();
      const expertise = useExpertise();
      const mode = useMode();

      const languageEnLabel = useLabel("semio.sketchpad.settings.language.en");
      const languageDeLabel = useLabel("semio.sketchpad.settings.language.de");
      const languagePlaceholder = useLabel("semio.sketchpad.app.home.settings.language.placeholder");

      return (
        <>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.theme"
                value={theme}
                onValueChange={(value: string) => setTheme("semio.sketchpad.settings.theme", value as Theme)}
                showLabel
                kind="single"
                items={[
                  { value: Theme.SYSTEM, id: "semio.sketchpad.settings.theme.system", icon: <MonitorIcon className="size-small" /> },
                  { value: Theme.LIGHT, id: "semio.sketchpad.settings.theme.light", icon: <SunIcon className="size-small" /> },
                  { value: Theme.DARK, id: "semio.sketchpad.settings.theme.dark", icon: <MoonIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <SelectUI id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage("semio.sketchpad.settings.language", value)} showLabel>
                <SelectTrigger>
                  <SelectValue placeholder={languagePlaceholder} />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="en">{languageEnLabel}</SelectItem>
                  <SelectItem value="de">{languageDeLabel}</SelectItem>
                </SelectContent>
              </SelectUI>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.layout"
                value={typeof layout === "object" ? "desktop" : layout}
                onValueChange={(value: string) => setLayout("semio.sketchpad.settings.layout", value as "desktop" | "tablet")}
                showLabel
                kind="single"
                items={[
                  { value: "desktop", id: "semio.sketchpad.settings.layout.desktop", icon: <MousePointerIcon className="size-small" /> },
                  { value: "tablet", id: "semio.sketchpad.settings.layout.tablet", icon: <HandIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.expertise"
                value={expertise}
                onValueChange={(value: string) => setExpertise("semio.sketchpad.settings.expertise", value as Expertise)}
                showLabel
                kind="single"
                items={[
                  { value: Expertise.BEGINNER, id: "semio.sketchpad.settings.expertise.beginner", icon: <TutorialIcon className="size-small" /> },
                  { value: Expertise.NORMAL, id: "semio.sketchpad.settings.expertise.normal", icon: <UserIcon className="size-small" /> },
                  { value: Expertise.EXPERT, id: "semio.sketchpad.settings.expertise.expert", icon: <AwardIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.mode"
                value={mode}
                onValueChange={(value: string) => setMode("semio.sketchpad.settings.mode", value as Mode)}
                showLabel
                kind="single"
                items={[
                  { value: Mode.USER, id: "semio.sketchpad.settings.mode.user", icon: <UserIcon className="size-small" /> },
                  { value: Mode.DEV, id: "semio.sketchpad.settings.mode.dev", icon: <CodeIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
        </>
      );
    };

    // Add Design-specific settings (most specific)
    addSection("settings", {
      id: "semio.sketchpad.app.design.settings",
      specificity: 30,
      order: 0,
      content: () => (
        <>
          <TreeItem>
            <TreeContent>
              <div className="flex flex-col gap-single">
                <label>
                  {useLabel("semio.sketchpad.app.design.proximityConnectDistance")}: {appSettings.design?.proximityConnectDistance}
                </label>
                <div className="w-full flex items-center" style={{ height: "20px" }}>
                  <div className="w-full relative" style={{ height: "4px", backgroundColor: "var(--border-color)" }}>
                    <div
                      style={{
                        position: "absolute",
                        left: `${((appSettings.design?.proximityConnectDistance || 10) / 20) * 100}%`,
                        top: "50%",
                        transform: "translate(-50%, -50%)",
                        width: "16px",
                        height: "16px",
                        backgroundColor: "var(--foreground)",
                        border: "1px solid var(--border-color)",
                      }}
                    />
                  </div>
                </div>
              </div>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              {useLabel("semio.sketchpad.app.design.gridSize")}: {appSettings.design?.gridSize || 24}px
            </TreeContent>
          </TreeItem>
        </>
      ),
    });

    // Add Kit settings (middle specificity)
    addSection("settings", {
      id: "semio.sketchpad.app.kit.settings",
      specificity: 10,
      order: 0,
      content: SketchpadSettingsContent,
    });

    // Add global Sketchpad settings (least specific)
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: SketchpadSettingsContent,
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.design.settings");
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <ReactFlowProvider>
      <Canvas id="semio.sketchpad.app.design.canvas">
        <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
      </Canvas>
      <DesignAppFooter />
    </ReactFlowProvider>
  );
};

const DesignApp: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useLayoutEffect(() => {
    addSection("toolbar", {
      id: "semio.sketchpad.app.design.tools",
      specificity: 20,
      order: 0,
      content: <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.design.tools");
    };
  }, [addSection, removeSection]);

  return (
    <TransactionPiecesProvider>
      <HoverPiecesProvider>
        <App />
      </HoverPiecesProvider>
    </TransactionPiecesProvider>
  );
};

// #region Config

export const config: AppConfig = {
  id: "design",
  component: DesignApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "designs/:design",
      paramName: "design",
      scopeProvider: DesignScopeProvider,
    },
  ],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
    createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
    createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
    createPanelDefinition(PanelKind.HUD, "semio.sketchpad.navbar.panelToggle.hud.show"),
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "designs" && isUuidPattern(pathParts[3]);
  },
  order: 20,
};

// #endregion Config

export default DesignApp;
