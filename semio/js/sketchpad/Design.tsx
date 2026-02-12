// #region 🔖Header

// 💻semio/js/sketchpad/Design.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

// #region 🔖Imports
// Imports for Design app MUST include all shared sketchpad, React, and UI dependencies.

import { useSelector } from "@xstate/react";
import { ConnectionDiff, ConnectionId, Guid, KitDiff, PieceDiff, PieceId } from "../semio";
import type { AppConfig, AppPlugin, AppWindowConfig, DesignAppId, Field, HookResult, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolRenderContext } from "./shared";
import {
  conditionalHookResult,
  createField as createFieldValue,
  createKeyedTransactionHandlers,
  createPanelDefinition,
  Expertise,
  fieldToHookResult,
  Mode,
  PanelKind,
  readonlyHookResult,
  registerAppPlugin,
  registerEventHandler,
  registerKeyedAppEventHandlers,
  Theme,
  ToolKind,
} from "./shared";
import type { DesignStore as DesignEntityStore } from "./Sketchpad";
import {
  createDefaultDesignAppState,
  createDesignActiveToolSelector,
  createDesignCameraSelector,
  createDesignDiagramCenterSelector,
  createDesignDiagramScaleSelector,
  createDesignFocusedPieceSelector,
  createDesignFullscreenWindowSelector,
  createDesignHoverSelector,
  createDesignOthersSelector,
  createDesignPanelVisibilitySelector,
  createDesignSelectedModelTagsSelector,
  createDesignSelectionSelector,
  identitySelector,
  useDesignScope,
  useDevice,
  useExpertise,
  useKitScope,
  useLanguage,
  useMode,
  usePieceScope,
  useSketchpadActor,
  useSketchpadActorSafe,
  useTheme,
} from "./Sketchpad";

import { DragEndEvent, useDraggable } from "@dnd-kit/core";
import { arrayMove } from "@dnd-kit/sortable";
import { Edges, Line, Select, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import { AddIcon, AwardIcon, CodeIcon, ConnectionIcon, DiagramIcon, DisconnectIcon, HandIcon, IntersectIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoveIcon, SceneIcon, SelectToolIcon, SunIcon, TableViewIcon, TutorialIcon, UserIcon } from "@semio/assets";
import React, { createContext, FC, memo, ReactNode, Suspense, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { AddIcon, AwardIcon, CodeIcon, ConnectionIcon, DiagramIcon, DisconnectIcon, HandIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoveIcon, SceneIcon, SelectToolIcon, SunIcon, TableViewIcon, TutorialIcon, UserIcon } from "@semio/assets";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { useLabel } from "../i18n";

const KitSectionLazy = React.lazy(() => import("./Kit").then((module) => ({ default: module.KitSection })));

import {
  areDesignsInSameFamily,
  arePortsCompatible,
  areSameConnection,
  Camera,
  Connection,
  Connector,
  Coord,
  createClusteredDesign,
  Design,
  DiffStatus,
  expandDesignPieces,
  findConnectionsInDesign,
  findConnectorInType,
  findDesignInKit,
  findModel,
  findPieceInDesign,
  findTypeInKit,
  fixPiecesInDesign,
  generateUniqueName,
  getDesignDiff,
  getIncludedDesigns,
  guid,
  ICON_WIDTH,
  isPortInUse,
  Kit,
  Model,
  Piece,
  Plane,
  planeToMatrix,
  replaceClusterWithDesign,
  selectBestModel,
  TOLERANCE,
  toSemioRotation,
  toThreeRotation,
  Type,
} from "../semio";
import type { ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Connection as RFConnection } from "./elements";
import {
  applyNodeChanges,
  Avatar,
  AvatarFallback,
  BaseEdge,
  Button,
  Combobox,
  Diagram,
  DraggableAvatar,
  Geometry,
  Handle,
  Input,
  Position,
  ReactFlowProvider,
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
  TransactionProvider,
  TreeContent,
  TreeItem,
  TreeSection,
  useReactFlow,
  ViewportPortal,
} from "./elements";
import { getConnectorPortGuid, getPortCompatibilityState, getPortTone } from "./portColor";
import { getKitAppHooks, registerDesignAppHooks, registerDesignAppStoreFactory } from "./shared";
import {
  Canvas,
  ConnectionScopeProvider,
  DesignScopeProvider,
  KitScopeProvider,
  KitStore,
  LayoutCanvas,
  PieceMetadata,
  PieceScopeProvider,
  PlainKitDiffAppStore,
  SketchpadStore,
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
  useDesignAppXState,
  useDiffedPiece,
  useDragDrop,
  useExplodeableDesignNodes,
  useFlatPiecePlane,
  useFocusSafe,
  useIncludedDesigns,
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
  useOrigin,
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
  useTooltip,
  useType
} from "./Sketchpad";

// #endregion 🔖Imports

// #region 🔖State Management
// State management types and interfaces MUST define the Design app selection, presence, hover, diff, and state shape.

let designAppCommands: Record<string, (context: any, ...args: any[]) => Promise<any> | any>;

// Tracks the current piece, connection, and connector selection state for the Design app.
export interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: Array<{ piece: Guid; connector: Guid }>;
  connector?: { piece: Guid; designPiece?: Guid; connector: Guid };
}
// Diff for added/removed piece GUIDs in a selection change.
export interface DesignAppSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
// Diff for added/removed connection GUIDs in a selection change.
export interface DesignAppSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
// Diff for a selected port change identifying the piece and connector.
export interface DesignAppSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  connector?: Guid;
}
// Composite diff combining pieces, connections, and connector selection changes.
export interface DesignAppSelectionDiff {
  pieces?: DesignAppSelectionPiecesDiff;
  connections?: DesignAppSelectionConnectionsDiff;
  connector?: DesignAppSelectionPortDiff;
}
// Enumeration of fullscreen window modes for the Design app.
export enum DesignAppFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
// Enumeration of window kinds available in the Design app.
export enum DesignAppWindowKind {
  Diagram = "diagram",
  Scene = "scene",
}
// Presence state for a Design app user including cursor, camera, and diagram viewport.
export interface DesignAppPresence {
  cursor?: Coord;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
// Hover state tracking which pieces, connections, connectors, types, and designs are hovered.
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: { piece: Guid; designPiece?: Guid; connector: Guid }[];
  types?: Guid[];
  designs?: Guid[];
}
// Extended presence for other collaborators including their display name.
export interface DesignAppPresenceOther extends DesignAppPresence {
  name: string;
}
// Complete diff describing all mutable Design app state changes.
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
// Edit record extending KitDiffAppEdit with Design app selection diff.
export interface DesignAppEdit extends KitDiffAppEdit<DesignAppSelectionDiff> { }
// Complete runtime state for a Design app instance.
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

// Context passed to Design app commands including app state, GUID, and design data.
export interface DesignAppCommandContext extends KitCommandContext {
  designApp: DesignAppState;
  Guid: Guid;
  design: Design;
}
// Result returned by Design app commands containing diffs to apply.
export interface DesignAppCommandResult {
  diff?: DesignAppDiff;
  kitDiff?: KitDiff;
}

// #endregion 🔖State Management

// #region Commands
// Commands MUST define all executable Design app actions dispatched by keyboard shortcuts and UI interactions.

// Registry of all named Design app commands mapped to their handler functions.
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
          connector: {},
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
  "semio.designApp.hoverPort": (context: DesignAppCommandContext, pieceGuid: Guid, connectorGuid: Guid, designPieceGuid?: Guid): DesignAppCommandResult => {
    return {
      diff: {
        hover: {
          connectors: [{ piece: pieceGuid, designPiece: designPieceGuid, connector: connectorGuid }],
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
          connector: {},
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
          connector: {},
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
          connector: {},
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
  "semio.designApp.selectPiecePort": (context: DesignAppCommandContext, piece: Guid, connector: Guid, designPiece?: Guid): DesignAppCommandResult => {
    return {
      diff: {
        selection: {
          pieces: {
            removed: context.designApp.selection?.pieces || [],
          },
          connections: {
            removed: context.designApp.selection?.connections || [],
          },
          connector: {
            piece,
            connector,
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
          connector: {},
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
  "semio.designApp.clusterPieces": (context: DesignAppCommandContext, pieceGuids: Guid[]): DesignAppCommandResult => {
    if (!pieceGuids || pieceGuids.length === 0) {
      return {};
    }
    const designPieceGuids = new Set((context.design.pieces || []).map((piece) => piece.guid));
    const validPieceGuids = pieceGuids.filter((guid) => designPieceGuids.has(guid));
    if (validPieceGuids.length === 0) {
      return {};
    }
    const existingNames = (context.kit.designs || []).map((d) => d.name);
    const clusterName = generateUniqueName(`${context.design.name} Cluster`, existingNames);
    const { clusteredDesign, externalConnections } = createClusteredDesign(context.design, validPieceGuids, clusterName);
    const designDiff = replaceClusterWithDesign(context.design, validPieceGuids, clusteredDesign, externalConnections);
    const currentSelection = context.designApp.selection || {};
    const piecesRemoved = currentSelection.pieces || [];
    const connectionsRemoved = currentSelection.connections || [];
    return {
      diff: {
        selection: {
          pieces: {
            removed: piecesRemoved,
            added: [clusteredDesign.guid],
          },
          connections: {
            removed: connectionsRemoved,
          },
        },
      },
      kitDiff: {
        designs: {
          added: [clusteredDesign],
          updated: [
            {
              design: { guid: context.design.guid },
              diff: designDiff,
            },
          ],
        },
      },
    };
  },
  "semio.designApp.expandDesign": (context: DesignAppCommandContext, designGuid: Guid): DesignAppCommandResult => {
    if (!designGuid) {
      return {};
    }
    const referencedDesign = (context.kit.designs || []).find((d) => d.guid === designGuid);
    if (!referencedDesign) {
      return {};
    }

    const expandedReferencedDesign = expandDesignPieces(referencedDesign, context.kit);
    const existingPieceGuids = new Set((context.design.pieces || []).map((piece) => piece.guid));
    const addedPieces = (expandedReferencedDesign.pieces || []).filter((piece) => !existingPieceGuids.has(piece.guid));
    const existingConnections = context.design.connections || [];
    const addedConnections = (expandedReferencedDesign.connections || []).filter((connection) => !existingConnections.some((existing) => areSameConnection(existing, connection)));

    const updatedExternalConnections = (context.design.connections || []).map((connection) => {
      if (connection.connected.designPiece?.guid === designGuid) {
        return {
          ...connection,
          connected: {
            ...connection.connected,
            designPiece: undefined,
          },
        };
      }
      if (connection.connecting.designPiece?.guid === designGuid) {
        return {
          ...connection,
          connecting: {
            ...connection.connecting,
            designPiece: undefined,
          },
        };
      }
      return connection;
    });

    const expandedDesign: Design = {
      ...context.design,
      pieces: [...(context.design.pieces || []), ...addedPieces],
      connections: [...updatedExternalConnections, ...addedConnections],
    };

    const designDiff = getDesignDiff(context.design, expandedDesign);
    const currentSelection = context.designApp.selection || {};
    const piecesRemoved = currentSelection.pieces || [];
    const connectionsRemoved = currentSelection.connections || [];

    return {
      diff: {
        selection: {
          pieces: {
            removed: piecesRemoved,
          },
          connections: {
            removed: connectionsRemoved,
          },
        },
      },
      kitDiff: {
        designs: {
          updated: [
            {
              design: { guid: context.design.guid },
              diff: designDiff,
            },
          ],
        },
      },
    };
  },
};

designAppCommands = commands;

// #endregion Commands

// #region Store
// Store MUST implement DesignStore extending PlainKitDiffAppStore with undo/redo, selection diff inversion, and state persistence.

// MUST return a diff that reverses the given selection diff.
// Computes the inverse of a Design app selection diff for undo support.
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

  if (diff.connector) {
    inverseDiff.connector = {
      piece: selection.connector?.piece,
      designPiece: selection.connector?.designPiece,
      connector: selection.connector?.connector,
    };
  }

  return inverseDiff;
};
// Checks whether two Design app identifiers refer to the same design.
export const areSameDesignApp = (designApp: DesignAppId, other: DesignAppId): boolean => designApp.kit === other.kit && designApp.design === other.design;
// Checks whether a Design app identifier matches any in a list.
export const hasSameDesignApp = (designApp: DesignAppId, others: DesignAppId[]): boolean => others.some((other) => areSameDesignApp(designApp, other));

// MUST extend PlainKitDiffAppStore and synchronize state with the Y.js shared document.
// DesignStore manages Design app state persistence, undo/redo stacks, and Y.js synchronization.
export class DesignStore extends PlainKitDiffAppStore<DesignAppState, DesignAppDiff, DesignAppSelectionDiff, DesignAppEdit, DesignAppCommandContext, DesignAppCommandResult> {
  private readonly kitGuid: Guid;
  private readonly designGuid: Guid;

  constructor(parent: SketchpadStore, id: DesignAppId, initialState?: DesignAppState) {
    const defaultState: DesignAppState = {
      fullscreenWindow: initialState?.fullscreenWindow || DesignAppFullscreenWindow.None,
      panelVisibility: initialState?.panelVisibility || { toolbar: true, workbench: false, details: true, chat: false, settings: false },
      activeTool: initialState?.activeTool || ToolKind.SELECTION_NORMAL,
      selection: initialState?.selection,
      hover: initialState?.hover,
      presence: initialState?.presence,
      others: initialState?.others || [],
      camera: initialState?.camera,
      diagramCenter: initialState?.diagramCenter,
      diagramScale: initialState?.diagramScale,
      focusedPieceGuid: initialState?.focusedPieceGuid,
      selectedModelTags: initialState?.selectedModelTags || {},
      windowLayout: initialState?.windowLayout,
    };
    super(parent, defaultState);

    this.kitGuid = id.kit;
    this.designGuid = id.design;

    Object.entries(designAppCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  kit(): KitStore {
    return this.parentStore.kit(this.kitGuid);
  }

  design(): DesignEntityStore {
    return this.kit().design(this.designGuid);
  }

  protected getSelection(): DesignAppSelection {
    return this.state.selection || {};
  }

  protected inverseSelectionDiff(selection: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelectionDiff {
    return inverseDesignAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: DesignAppSelectionDiff): void {
    const currentSelection = this.state.selection || {};
    const newSelection: DesignAppSelection = { ...currentSelection };

    if (selectionDiff.pieces) {
      const currentPieces = new Set(currentSelection.pieces || []);
      if (selectionDiff.pieces.added) {
        selectionDiff.pieces.added.forEach((p) => currentPieces.add(p));
      }
      if (selectionDiff.pieces.removed) {
        selectionDiff.pieces.removed.forEach((p) => currentPieces.delete(p));
      }
      newSelection.pieces = currentPieces.size > 0 ? Array.from(currentPieces) : undefined;
    }

    if (selectionDiff.connections) {
      const currentConnections = new Set(currentSelection.connections || []);
      if (selectionDiff.connections.added) {
        selectionDiff.connections.added.forEach((c) => currentConnections.add(c));
      }
      if (selectionDiff.connections.removed) {
        selectionDiff.connections.removed.forEach((c) => currentConnections.delete(c));
      }
      newSelection.connections = currentConnections.size > 0 ? Array.from(currentConnections) : undefined;
    }

    if (selectionDiff.connector) {
      if (selectionDiff.connector.piece && selectionDiff.connector.connector) {
        newSelection.connector = {
          piece: selectionDiff.connector.piece,
          designPiece: selectionDiff.connector.designPiece,
          connector: selectionDiff.connector.connector,
        };
      } else {
        newSelection.connector = undefined;
      }
    }

    this.state = { ...this.state, selection: newSelection };
    this.notify();
  }

  change(diff: DesignAppDiff): void {
    const newState = { ...this.state };

    if (diff.fullscreenWindow !== undefined) newState.fullscreenWindow = diff.fullscreenWindow;
    if (diff.activeTool !== undefined) newState.activeTool = diff.activeTool;
    if (diff.panelVisibility !== undefined) {
      newState.panelVisibility = { ...newState.panelVisibility, ...diff.panelVisibility };
    }
    if (diff.selection) {
      this.applySelectionDiff(diff.selection);
      return;
    }
    if (diff.hover !== undefined) {
      newState.hover = Object.keys(diff.hover).length === 0 ? undefined : diff.hover;
    }
    if (diff.camera !== undefined) newState.camera = diff.camera;
    if (diff.diagramCenter !== undefined) newState.diagramCenter = diff.diagramCenter;
    if (diff.diagramScale !== undefined) newState.diagramScale = diff.diagramScale;
    if (diff.focusedPieceGuid !== undefined) {
      newState.focusedPieceGuid = diff.focusedPieceGuid === null ? undefined : diff.focusedPieceGuid;
    }
    if (diff.selectedModelTags !== undefined) {
      newState.selectedModelTags = { ...(newState.selectedModelTags || {}), ...diff.selectedModelTags };
    }
    if (Object.prototype.hasOwnProperty.call(diff, "windowLayout")) {
      newState.windowLayout = (diff as any).windowLayout;
    }

    this.state = newState;
    this.notify();
  }

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
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.abortTransaction") {
      this.abortTransaction();
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
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

let designStoreInitialized = false;
// MUST register the DesignStore factory exactly once via registerDesignAppStoreFactory.
// Initializes the Design app store factory registration.
export function initializeDesignStore() {
  if (designStoreInitialized) return;
  designStoreInitialized = true;
  registerDesignAppStoreFactory((parent: any, id: any, state: any) => new DesignStore(parent, id, state));
}

// #region 🔖Design App Plugin Registration
// Design app plugin registration MUST register the Design app plugin with machine actions, guards, and default state.

const designAppPlugin: AppPlugin = {
  id: "design",
  namespace: "DESIGN",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): DesignAppState => ({
      panelVisibility: { toolbar: true, workbench: false, details: true, chat: false, settings: false },
      selection: undefined,
      hover: undefined,
      presence: undefined,
      others: [],
      camera: undefined,
      diagramCenter: undefined,
      diagramScale: undefined,
      focusedPieceGuid: undefined,
      selectedModelTags: {},
      windowLayout: undefined,
      fullscreenWindow: DesignAppFullscreenWindow.None,
      activeTool: ToolKind.SELECTION_NORMAL,
    }),
  },
  registerStores: () => {
    initializeDesignStore();
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(designAppPlugin);
  registerDesignAppHooks({
    useDesignAppCommands,
    useDesignAppDiff,
    useDesignAppHover,
    useDesignAppIsPieceHovered,
    useDesignAppIsPieceTransitiveHovered,
    useDesignAppIsConnectionHovered,
    useDesignAppSelection,
    useDesignAppIsPieceSelected,
    useDesignAppIsConnectionSelected,
    useDesignAppStore: useDesignStore,
  });
  const designAppEventConfig = {
    namespace: "DESIGN" as const,
    appKey: "designApps" as const,
    getKey: (event: any) => `${event.kitGuid}:${event.designGuid}`,
    createDefaultState: createDefaultDesignAppState,
  };
  registerKeyedAppEventHandlers(designAppEventConfig);
  registerEventHandler("DESIGN.FOCUS_PIECE", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      return { designApps: { ...context.designApps, [key]: { ...app, focusedPiece: event.pieceGuid } } };
    },
  });
  registerEventHandler("DESIGN.SET_DIAGRAM_CENTER", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      return { designApps: { ...context.designApps, [key]: { ...app, diagramCenter: event.center } } };
    },
  });
  registerEventHandler("DESIGN.SET_DIAGRAM_SCALE", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      return { designApps: { ...context.designApps, [key]: { ...app, diagramScale: event.scale } } };
    },
  });
  registerEventHandler("DESIGN.SELECT_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const tags = app.selectedModelTags[event.typeGuid] || [];
      if (tags.includes(event.tagGuid)) return {};
      return { designApps: { ...context.designApps, [key]: { ...app, selectedModelTags: { ...app.selectedModelTags, [event.typeGuid]: [...tags, event.tagGuid] } } } };
    },
  });
  registerEventHandler("DESIGN.DESELECT_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const tags = app.selectedModelTags[event.typeGuid] || [];
      return { designApps: { ...context.designApps, [key]: { ...app, selectedModelTags: { ...app.selectedModelTags, [event.typeGuid]: tags.filter((g: Guid) => g !== event.tagGuid) } } } };
    },
  });
  registerEventHandler("DESIGN.SELECT_PIECE", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const pieces = [...(app.selection?.pieces || [])];
      if (!pieces.includes(event.pieceGuid)) pieces.push(event.pieceGuid);
      return { designApps: { ...context.designApps, [key]: { ...app, selection: { ...app.selection, pieces } } } };
    },
  });
  registerEventHandler("DESIGN.DESELECT_PIECE", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const pieces = (app.selection?.pieces || []).filter((p: Guid) => p !== event.pieceGuid);
      return { designApps: { ...context.designApps, [key]: { ...app, selection: { ...app.selection, pieces } } } };
    },
  });
  registerEventHandler("DESIGN.SELECT_CONNECTION", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const connections = [...(app.selection?.connections || [])];
      if (!connections.includes(event.connectionGuid)) connections.push(event.connectionGuid);
      return { designApps: { ...context.designApps, [key]: { ...app, selection: { ...app.selection, connections } } } };
    },
  });
  registerEventHandler("DESIGN.DESELECT_CONNECTION", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      const connections = (app.selection?.connections || []).filter((c: Guid) => c !== event.connectionGuid);
      return { designApps: { ...context.designApps, [key]: { ...app, selection: { ...app.selection, connections } } } };
    },
  });
  registerEventHandler("DESIGN.SELECT_ALL", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      return { designApps: { ...context.designApps, [key]: { ...app, selection: { pieces: [], connections: [] } } } };
    },
  });
  registerEventHandler("DESIGN.DELETE_SELECTED", {
    action: (context: any, event: any) => {
      const key = `${event.kitGuid}:${event.designGuid}`;
      const app = context.designApps[key] || createDefaultDesignAppState();
      return { designApps: { ...context.designApps, [key]: { ...app, selection: undefined } } };
    },
  });

  createKeyedTransactionHandlers({
    namespace: "DESIGN",
    appKey: "designApps",
    keyFields: ["kitGuid", "designGuid"],
    createDefaultState: createDefaultDesignAppState,
  });
}

// #endregion 🔖Design App Plugin Registration

type DesignAppScope = { id: string };
const DesignAppScopeContext = createContext<DesignAppScope | null>(null);

const DesignAppActorContext = createContext<any>(null);

const DesignAppSyncComponent = ({ children }: { children: React.ReactNode }) => {
  useDesignAppInitialize();
  return <>{children}</>;
};

// #region 🔖Hooks
// Hooks MUST provide the Design app initialization lifecycle within the React component tree.

function useDesignAppInitialize() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const initializedKeyRef = useRef<string | null>(null);

  useLayoutEffect(() => {
    if (!kitGuid || !designGuid) return;
    const initKey = `${kitGuid}:${designGuid}`;
    if (initializedKeyRef.current === initKey) return;
    actor.send({
      type: "DESIGN.INIT",
      kitGuid,
      designGuid,
      state: {
        panelVisibility: { toolbar: true, workbench: false, details: true, chat: false, settings: false },
        selection: undefined,
        hover: undefined,
        focusedPiece: undefined,
        camera: undefined,
        activeTool: ToolKind.SELECTION_NORMAL,
        fullscreenWindow: DesignAppFullscreenWindow.None,
        selectedModelTags: {},
        transaction: {
          isTransactionActive: false,
          currentTransactionStack: [],
          pastTransactionStack: [],
          redoStack: [],
        },
      },
    });
    initializedKeyRef.current = initKey;
  }, [actor, kitGuid, designGuid]);
}

// #endregion 🔖Hooks

// #region Components
// Components MUST provide Design app scope, actor context, and synchronization wrapper components.

// MUST wrap children with DesignAppScopeContext and DesignAppActorContext providers.
// Provider component that establishes Design app scope and actor context.
export const DesignAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignAppScopeContext.Provider, { value }, React.createElement(DesignAppActorContext.Provider, { value: null }, React.createElement(DesignAppSyncComponent, null, props.children)));
};

const useDesignAppScope = () => useContext(DesignAppScopeContext);

// MUST return the actor from DesignAppActorContext.
// Returns the current Design app XState actor from context.
export function useDesignAppActor(): any {
  return useContext(DesignAppActorContext);
}

// MUST resolve the DesignStore from the orchestrator and apply the selector.
// Selects derived state from the Design app store.
export function useDesignStore<T>(selector?: (store: DesignStore) => T, id?: DesignAppId): T | DesignStore | null {
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

export { useDesignStore as useDesignAppStore };

// MUST use useSelector to reactively track the Design app state slice.
// Selects derived state from the Design app XState snapshot.
export function useDesignApp<T>(selector?: (state: DesignAppState) => T, id?: DesignAppId): T | DesignAppState | null {
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";

  const state = useDesignAppXState(kitGuid, designGuid);

  if (!kitGuid || !designGuid) return null;

  if (selector) {
    return selector(state as unknown as DesignAppState) as T;
  }
  return state as unknown as DesignAppState;
}

const EMPTY_SELECTION: DesignAppSelection = {};
const EMPTY_OTHERS: DesignAppPresenceOther[] = [];
const EMPTY_MODEL_TAGS: Record<Guid, string[]> = {};
const DEFAULT_PANEL_VISIBILITY: PanelVisibility = { toolbar: false, workbench: false, details: true, chat: false, settings: false };

type GranularSelectorFactory<T> = (kitGuid: Guid, designGuid: Guid) => (state: any) => T | undefined;

interface UseDesignAppFieldOptions<T, TEvent extends { type: string }> {
  createGranularSelector: GranularSelectorFactory<T>;
  fallback: T;
  createCanEvent: (kitGuid: Guid, designGuid: Guid) => TEvent;
  createSendEvent: (kitGuid: Guid, designGuid: Guid, value: T) => TEvent;
  useWildcardFallback?: boolean;
}

function useDesignAppField<T, TEvent extends { type: string }>(options: UseDesignAppFieldOptions<T, TEvent>): Field<T> {
  const { createGranularSelector, fallback, createCanEvent, createSendEvent, useWildcardFallback = false } = options;
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const granularSelector = useMemo(() => createGranularSelector(kitGuid, designGuid), [createGranularSelector, kitGuid, designGuid]);
  const rawValue = useSelector(actor, granularSelector);
  const value = rawValue ?? fallback;
  const canEvent = useMemo(() => createCanEvent(kitGuid, designGuid), [createCanEvent, kitGuid, designGuid]);
  const canSetFromSnapshot = useSelector(actor, (snapshot) => snapshot.can(canEvent as Parameters<typeof snapshot.can>[0]));
  const hasScope = kitGuid !== "" && designGuid !== "";
  const canSet = useWildcardFallback ? canSetFromSnapshot || hasScope : canSetFromSnapshot;
  const setter = useMemo(
    () => (next: T) => {
      if (canSet) {
        actor.send(createSendEvent(kitGuid, designGuid, next) as Parameters<typeof actor.send>[0]);
      }
    },
    [actor, kitGuid, designGuid, canSet, createSendEvent],
  );
  return useMemo(() => createFieldValue(value, setter, canSet), [value, setter, canSet]);
}

// MUST create a Field wrapping the selection value and setter.
// Returns a reactive field for a Design app selection property.
export function useDesignAppSelectionField(): Field<DesignAppSelection> {
  return useDesignAppField<DesignAppSelection, { type: "DESIGN.SET_SELECTION"; kitGuid: Guid; designGuid: Guid; selection: DesignAppSelection }>({
    createGranularSelector: createDesignSelectionSelector,
    fallback: EMPTY_SELECTION,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: {} as DesignAppSelection }),
    createSendEvent: (kitGuid, designGuid, selection) => ({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection }),
  });
}

// MUST provide the current selection, a setter, and a canSet flag.
// Returns a hook result for the current Design app selection.
export function useDesignAppSelection(): HookResult<DesignAppSelection> {
  return fieldToHookResult(useDesignAppSelectionField());
}

// MUST create a Field wrapping the fullscreen value and setter.
// Returns a reactive field for the Design app fullscreen window.
export function useDesignAppFullscreenField(): Field<DesignAppFullscreenWindow> {
  return useDesignAppField<DesignAppFullscreenWindow, { type: "DESIGN.SET_FULLSCREEN"; kitGuid: Guid; designGuid: Guid; window: DesignAppFullscreenWindow }>({
    createGranularSelector: createDesignFullscreenWindowSelector,
    fallback: DesignAppFullscreenWindow.None,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: DesignAppFullscreenWindow.None }),
    createSendEvent: (kitGuid, designGuid, fullscreen) => ({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: fullscreen }),
  });
}

// MUST provide the current fullscreen window, a setter, and a canSet flag.
// Returns a hook result for the Design app fullscreen window state.
export function useDesignAppFullscreen(): HookResult<DesignAppFullscreenWindow> {
  return fieldToHookResult(useDesignAppFullscreenField());
}

// MUST create a Field wrapping the active tool value and setter.
// Returns a reactive field for the Design app active tool.
export function useDesignAppActiveToolField(): Field<ToolKind> {
  return useDesignAppField<ToolKind, { type: "DESIGN.SET_ACTIVE_TOOL"; kitGuid: Guid; designGuid: Guid; tool: ToolKind }>({
    createGranularSelector: createDesignActiveToolSelector,
    fallback: ToolKind.SELECTION_NORMAL,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool: ToolKind.SELECTION_NORMAL }),
    createSendEvent: (kitGuid, designGuid, tool) => ({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool }),
    useWildcardFallback: true,
  });
}

// MUST provide the current active tool, a setter, and a canSet flag.
// Returns a hook result for the Design app active tool.
export function useDesignAppActiveTool(): HookResult<ToolKind> {
  return fieldToHookResult(useDesignAppActiveToolField());
}

// MUST provide the current diff, a setter, and a canSet flag.
// Returns a hook result for the Design app diff state.
export function useDesignAppDiff(): HookResult<KitDiff | undefined> {
  return readonlyHookResult<KitDiff | undefined>(undefined);
}

// MUST return a read-only list of other users' presence data.
// Returns other collaborators' presence state for the Design app.
export function useDesignAppOthers(): HookResult<DesignAppPresenceOther[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignOthersSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_OTHERS;
  return readonlyHookResult(value);
}

// MUST create a Field wrapping the camera value and setter.
// Returns a reactive field for the Design app camera.
export function useDesignAppCameraField(): Field<Camera | undefined> {
  return useDesignAppField<Camera | undefined, { type: "DESIGN.SET_CAMERA"; kitGuid: Guid; designGuid: Guid; camera: Camera | undefined }>({
    createGranularSelector: createDesignCameraSelector,
    fallback: undefined,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera: undefined }),
    createSendEvent: (kitGuid, designGuid, camera) => ({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera }),
  });
}

// MUST provide the current camera, a setter, and a canSet flag.
// Returns a hook result for the Design app camera state.
export function useDesignAppCamera(): HookResult<Camera | undefined> {
  return fieldToHookResult(useDesignAppCameraField());
}

// MUST provide the current diagram center, a setter, and a canSet flag.
// Returns a hook result for the Design app diagram center coordinate.
export function useDesignAppDiagramCenter(): HookResult<Coord | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignDiagramCenterSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const rawValue = useSelector(actor, selector);
  const value = useMemo(() => (rawValue ? { u: rawValue.x, v: rawValue.y } : undefined), [rawValue]);
  const canSetEvent = useMemo(() => ({ type: "DESIGN.SET_DIAGRAM_CENTER" as const, kitGuid, designGuid, center: { x: 0, y: 0 } }), [kitGuid, designGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (center: Coord | undefined) => {
      if (center) {
        actor.send({ type: "DESIGN.SET_DIAGRAM_CENTER", kitGuid, designGuid, center: { x: center.u, y: center.v } });
      }
    };
  }, [actor, kitGuid, designGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

// MUST provide the current diagram scale, a setter, and a canSet flag.
// Returns a hook result for the Design app diagram scale.
export function useDesignAppDiagramScale(): HookResult<number | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignDiagramScaleSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "DESIGN.SET_DIAGRAM_SCALE" as const, kitGuid, designGuid, scale: 1 }), [kitGuid, designGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (scale: number | undefined) => {
      if (scale !== undefined) {
        actor.send({ type: "DESIGN.SET_DIAGRAM_SCALE", kitGuid, designGuid, scale });
      }
    };
  }, [actor, kitGuid, designGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

// MUST create a Field wrapping the focused piece GUID value and setter.
// Returns a reactive field for the focused piece GUID.
export function useDesignAppFocusedPieceGuidField(): Field<Guid | undefined> {
  return useDesignAppField<Guid | undefined, { type: "DESIGN.FOCUS_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid | undefined }>({
    createGranularSelector: createDesignFocusedPieceSelector,
    fallback: undefined,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid: undefined }),
    createSendEvent: (kitGuid, designGuid, pieceGuid) => ({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid }),
  });
}

// MUST provide the current focused piece GUID, a setter, and a canSet flag.
// Returns a hook result for the focused piece GUID.
export function useDesignAppFocusedPieceGuid(): HookResult<Guid | undefined> {
  return fieldToHookResult(useDesignAppFocusedPieceGuidField());
}

// MUST provide the current selected model tags, a setter, and a canSet flag.
// Returns a hook result for the Design app selected model tags.
export function useDesignAppSelectedModelTags(): HookResult<Record<Guid, string[]>> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignSelectedModelTagsSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_MODEL_TAGS;
  const canSetEvent = useMemo(() => ({ type: "DESIGN.SYNC" as const, kitGuid, designGuid, state: {} }), [kitGuid, designGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (tags: Record<Guid, string[]>) => {
      actor.send({ type: "DESIGN.SYNC", kitGuid, designGuid, state: { selectedModelTags: tags } });
    };
  }, [actor, kitGuid, designGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

// MUST provide the current hover, a setter, and a canSet flag.
// Returns a hook result for the Design app hover state.
export function useDesignAppHover(): HookResult<DesignAppHover | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignHoverSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "DESIGN.SET_HOVER" as const, kitGuid, designGuid, hover: {} }), [kitGuid, designGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (hover: DesignAppHover | undefined) => {
      if (hover && (hover.pieces?.length || hover.connections?.length)) {
        actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover });
      } else {
        actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid });
      }
    };
  }, [actor, kitGuid, designGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

// MUST create a Field wrapping the panel visibility value and setter.
// Returns a reactive field for Design app panel visibility.
export function useDesignAppPanelVisibilityField(): Field<PanelVisibility> {
  return useDesignAppField<PanelVisibility, { type: "DESIGN.SET_PANEL_VISIBILITY"; kitGuid: Guid; designGuid: Guid; panelVisibility: PanelVisibility }>({
    createGranularSelector: createDesignPanelVisibilitySelector,
    fallback: DEFAULT_PANEL_VISIBILITY,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_PANEL_VISIBILITY", kitGuid, designGuid, panelVisibility: {} as PanelVisibility }),
    createSendEvent: (kitGuid, designGuid, panelVisibility) => ({ type: "DESIGN.SET_PANEL_VISIBILITY", kitGuid, designGuid, panelVisibility }),
  });
}

// MUST provide the current panel visibility, a setter, and a canSet flag.
// Returns a hook result for Design app panel visibility.
export function useDesignAppPanelVisibility(): HookResult<PanelVisibility> {
  return fieldToHookResult(useDesignAppPanelVisibilityField());
}

//#region 🔖Action Hooks
// Action hooks MUST provide composable React hooks for Design app selection, hover, focus, panel, and transaction actions.

// Tuple type for action hook results pairing an action callback with a canAct flag.
export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

// MUST return a callback that sets hover to the given piece GUID.
// Returns an action to set hover state to a single piece.
export function useDesignAppHoverPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuid: string) => setHover({ pieces: [pieceGuid] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that sets hover to the given piece GUIDs.
// Returns an action to set hover state to multiple pieces.
export function useDesignAppHoverPieces(): ActionHookResult<[pieceGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuids: string[]) => setHover({ pieces: pieceGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that sets hover to the given connection GUID.
// Returns an action to set hover state to a single connection.
export function useDesignAppHoverConnection(): ActionHookResult<[connectionGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (connectionGuid: string) => setHover({ connections: [connectionGuid] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that sets hover to the given port identifiers.
// Returns an action to set hover state to a single port.
export function useDesignAppHoverPort(): ActionHookResult<[pieceGuid: string, connectorGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuid: string, connectorGuid: string) => setHover({ connectors: [{ piece: pieceGuid, connector: connectorGuid }] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that sets hover to the given type GUIDs.
// Returns an action to set hover state to types.
export function useDesignAppHoverTypes(): ActionHookResult<[typeGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (typeGuids: string[]) => setHover({ types: typeGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that sets hover to the given design GUIDs.
// Returns an action to set hover state to designs.
export function useDesignAppHoverDesigns(): ActionHookResult<[designGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (designGuids: string[]) => setHover({ designs: designGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that clears all hover state.
// Returns an action to clear the Design app hover state.
export function useDesignAppClearHover(): ActionHookResult<[]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return () => setHover(undefined);
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

// MUST return a callback that selects the given piece GUID.
// Returns an action to select a single piece.
export function useDesignAppSelectPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuid: string) => setSelection({ pieces: [pieceGuid] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that selects the given piece GUIDs.
// Returns an action to select multiple pieces.
export function useDesignAppSelectPieces(): ActionHookResult<[pieceGuids: string[]]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuids: string[]) => setSelection({ pieces: pieceGuids });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that adds the given piece GUID to selection.
// Returns an action to add a piece to the current selection.
export function useDesignAppAddPieceToSelection(): ActionHookResult<[pieceGuid: string]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuid: string) => {
      const current = selection?.pieces ?? [];
      if (!current.includes(pieceGuid)) {
        setSelection({ ...selection, pieces: [...current, pieceGuid] });
      }
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that removes the given piece GUID from selection.
// Returns an action to remove a piece from the current selection.
export function useDesignAppRemovePieceFromSelection(): ActionHookResult<[pieceGuid: string]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuid: string) => {
      const current = selection?.pieces ?? [];
      setSelection({ ...selection, pieces: current.filter((p) => p !== pieceGuid) });
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that selects the given connection GUID.
// Returns an action to select a single connection.
export function useDesignAppSelectConnection(): ActionHookResult<[connectionGuid: string]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectionGuid: string) => setSelection({ connections: [connectionGuid] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that adds the given connection GUID to selection.
// Returns an action to add a connection to the current selection.
export function useDesignAppAddConnectionToSelection(): ActionHookResult<[connectionGuid: string]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectionGuid: string) => {
      const current = selection?.connections ?? [];
      if (!current.includes(connectionGuid)) {
        setSelection({ ...selection, connections: [...current, connectionGuid] });
      }
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that removes the given connection GUID from selection.
// Returns an action to remove a connection from the current selection.
export function useDesignAppRemoveConnectionFromSelection(): ActionHookResult<[connectionGuid: string]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectionGuid: string) => {
      const current = selection?.connections ?? [];
      setSelection({ ...selection, connections: current.filter((c) => c !== connectionGuid) });
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that selects the given piece-connector port.
// Returns an action to select a piece port.
export function useDesignAppSelectPiecePort(): ActionHookResult<[pieceGuid: string, connectorGuid: string, designPieceGuid?: string]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuid: string, connectorGuid: string, designPieceGuid?: string) => {
      setSelection({ ...selection, connector: { piece: pieceGuid, connector: connectorGuid, designPiece: designPieceGuid } });
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that deselects the given piece-connector port.
// Returns an action to deselect a piece port.
export function useDesignAppDeselectPiecePort(): ActionHookResult<[]> {
  const [selection, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => {
      const { connector: _, ...rest } = selection ?? {};
      setSelection(rest);
    };
  }, [selection, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that clears all selection state.
// Returns an action to deselect all items in the Design app.
export function useDesignAppDeselectAll(): ActionHookResult<[]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => setSelection({});
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that adds all piece and connection GUIDs to selection.
// Returns an action to select all pieces and connections.
export function useDesignAppSelectAll(): ActionHookResult<[]> {
  const design = useDesign() as Design | null;
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection || !design) return undefined;
    return () => {
      const allPieces = design.pieces?.map((p) => p.guid) ?? [];
      const allConnections = design.connections?.map((c) => c.guid) ?? [];
      setSelection({ pieces: allPieces, connections: allConnections });
    };
  }, [design, setSelection, canSetSelection]);
  return [action, canSetSelection];
}

// MUST return a callback that sets the focused piece GUID.
// Returns an action to focus on a specific piece.
export function useDesignAppFocusPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setFocusedPieceGuid, canSetFocus] = useDesignAppFocusedPieceGuid();
  const action = useMemo(() => {
    if (!canSetFocus || !setFocusedPieceGuid) return undefined;
    return (pieceGuid: string) => setFocusedPieceGuid(pieceGuid);
  }, [setFocusedPieceGuid, canSetFocus]);
  return [action, canSetFocus];
}

// MUST return a callback that clears the focused piece GUID.
// Returns an action to clear the focused piece.
export function useDesignAppClearFocus(): ActionHookResult<[]> {
  const [, setFocusedPieceGuid, canSetFocus] = useDesignAppFocusedPieceGuid();
  const action = useMemo(() => {
    if (!canSetFocus || !setFocusedPieceGuid) return undefined;
    return () => setFocusedPieceGuid(undefined);
  }, [setFocusedPieceGuid, canSetFocus]);
  return [action, canSetFocus];
}

// MUST return a callback that toggles the diagram fullscreen window state.
// Returns an action to toggle diagram fullscreen mode.
export function useDesignAppToggleDiagramFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useDesignAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === DesignAppFullscreenWindow.Diagram ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Diagram);
  }, [fullscreen, setFullscreen, canSetFullscreen]);
  return [action, canSetFullscreen];
}

// MUST return a callback that toggles the accessl fullscreen window state.
// Returns an action to toggle accessl fullscreen mode.
export function useDesignAppToggleAccesslFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useDesignAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === DesignAppFullscreenWindow.Accessl ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Accessl);
  }, [fullscreen, setFullscreen, canSetFullscreen]);
  return [action, canSetFullscreen];
}

// MUST return a callback that toggles the given panel's visibility.
// Returns an action to toggle a specific panel's visibility.
export function useDesignAppTogglePanel(): ActionHookResult<[panelKey: keyof PanelVisibility]> {
  const [panelVisibility, setPanelVisibility, canSetPanelVisibility] = useDesignAppPanelVisibility();
  const action = useMemo(() => {
    if (!canSetPanelVisibility || !setPanelVisibility) return undefined;
    return (panelKey: keyof PanelVisibility) => {
      setPanelVisibility({ ...panelVisibility, [panelKey]: !panelVisibility[panelKey] });
    };
  }, [panelVisibility, setPanelVisibility, canSetPanelVisibility]);
  return [action, canSetPanelVisibility];
}

// MUST return a callback that adds the given tag to all type entries.
// Returns an action to add a model tag for all types.
export function useDesignAppAddModelTagForAllTypes(): ActionHookResult<[tagGuid: string, typeGuids: string[]]> {
  const [selectedModelTags, setSelectedModelTags, canSetSelectedModelTags] = useDesignAppSelectedModelTags();
  const action = useMemo(() => {
    if (!canSetSelectedModelTags || !setSelectedModelTags) return undefined;
    return (tagGuid: string, typeGuids: string[]) => {
      const updated: Record<Guid, string[]> = { ...selectedModelTags };
      typeGuids.forEach((typeGuid) => {
        const existing = updated[typeGuid] ?? [];
        if (!existing.includes(tagGuid)) updated[typeGuid] = [...existing, tagGuid];
      });
      setSelectedModelTags(updated);
    };
  }, [selectedModelTags, setSelectedModelTags, canSetSelectedModelTags]);
  return [action, canSetSelectedModelTags];
}

// MUST return a callback that removes the given tag from all type entries.
// Returns an action to remove a model tag from all types.
export function useDesignAppRemoveModelTagFromAllTypes(): ActionHookResult<[tagGuid: string, typeGuids: string[]]> {
  const [selectedModelTags, setSelectedModelTags, canSetSelectedModelTags] = useDesignAppSelectedModelTags();
  const action = useMemo(() => {
    if (!canSetSelectedModelTags || !setSelectedModelTags) return undefined;
    return (tagGuid: string, typeGuids: string[]) => {
      const updated: Record<Guid, string[]> = { ...selectedModelTags };
      typeGuids.forEach((typeGuid) => {
        const existing = updated[typeGuid] ?? [];
        updated[typeGuid] = existing.filter((t) => t !== tagGuid);
      });
      setSelectedModelTags(updated);
    };
  }, [selectedModelTags, setSelectedModelTags, canSetSelectedModelTags]);
  return [action, canSetSelectedModelTags];
}

// Interface for transaction action callbacks including start, finalize, and abort.
export interface TransactionActions {
  start: () => void;
  finalize: () => void;
  abort: () => void;
}

// MUST provide start, finalize, and abort transaction actions.
// Returns the Design app transaction controller.
export function useDesignAppTransaction(): [TransactionActions | undefined, boolean] {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const canTransact = !!store;
  const actions = useMemo(() => {
    if (!store) return undefined;
    return {
      start: () => store.execute("semio.designApp.startTransaction", getOrigin()),
      finalize: () => store.execute("semio.designApp.finalizeTransaction", getOrigin()),
      abort: () => store.execute("semio.designApp.abortTransaction", getOrigin()),
    };
  }, [store, getOrigin]);
  return [actions, canTransact];
}

// MUST wrap children with the Design app transaction provider.
// Provider component that establishes Design app transaction context.
export const DesignAppTransactionProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [transaction] = useDesignAppTransaction();
  return <TransactionProvider transaction={transaction}>{children}</TransactionProvider>;
};

// MUST return a callback that undoes the most recent transaction.
// Returns an action to undo the last Design app transaction.
export function useDesignAppUndo(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.undo", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that redoes the most recently undone transaction.
// Returns an action to redo the last undone Design app transaction.
export function useDesignAppRedo(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.redo", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that removes all selected pieces and connections.
// Returns an action to delete all currently selected items.
export function useDesignAppDeleteSelected(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.deleteSelected", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that adds a piece with the given type GUID.
// Returns an action to add a piece to the design.
export function useDesignAppAddPiece(): ActionHookResult<[piece: Piece]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (piece: Piece) => store.execute("semio.designApp.addPiece", getOrigin(), piece);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that adds pieces with the given type GUIDs.
// Returns an action to add multiple pieces to the design.
export function useDesignAppAddPieces(): ActionHookResult<[pieces: Piece[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieces: Piece[]) => store.execute("semio.designApp.addPieces", getOrigin(), pieces);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that removes the piece with the given GUID.
// Returns an action to remove a piece from the design.
export function useDesignAppRemovePiece(): ActionHookResult<[pieceGuid: Guid]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuid: Guid) => store.execute("semio.designApp.removePiece", getOrigin(), pieceGuid);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that removes the pieces with the given GUIDs.
// Returns an action to remove multiple pieces from the design.
export function useDesignAppRemovePieces(): ActionHookResult<[pieceGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuids: Guid[]) => store.execute("semio.designApp.removePieces", getOrigin(), pieceGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that updates the piece with the given GUID and partial data.
// Returns an action to update a piece in the design.
export function useDesignAppUpdatePiece(): ActionHookResult<[pieceGuid: Guid, diff: PieceDiff]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuid: Guid, diff: PieceDiff) => store.execute("semio.designApp.updatePiece", getOrigin(), { piece: pieceGuid, diff });
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that updates the pieces with the given GUID-data pairs.
// Returns an action to update multiple pieces in the design.
export function useDesignAppUpdatePieces(): ActionHookResult<[updates: { id: Guid; diff: PieceDiff }[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (updates: { id: Guid; diff: PieceDiff }[]) =>
      store.execute(
        "semio.designApp.updatePieces",
        getOrigin(),
        updates.map((u) => ({ piece: u.id, diff: u.diff })),
      );
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that adds a connection with the given data.
// Returns an action to add a connection to the design.
export function useDesignAppAddConnection(): ActionHookResult<[connection: Connection]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connection: Connection) => store.execute("semio.designApp.addConnection", getOrigin(), connection);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that adds connections with the given data array.
// Returns an action to add multiple connections to the design.
export function useDesignAppAddConnections(): ActionHookResult<[connections: Connection[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connections: Connection[]) => store.execute("semio.designApp.addConnections", getOrigin(), connections);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that removes the connection with the given GUID.
// Returns an action to remove a connection from the design.
export function useDesignAppRemoveConnection(): ActionHookResult<[connectionGuid: Guid]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuid: Guid) => store.execute("semio.designApp.removeConnection", getOrigin(), connectionGuid);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that removes the connections with the given GUIDs.
// Returns an action to remove multiple connections from the design.
export function useDesignAppRemoveConnections(): ActionHookResult<[connectionGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuids: Guid[]) => store.execute("semio.designApp.removeConnections", getOrigin(), connectionGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that updates the connection with the given GUID and partial data.
// Returns an action to update a connection in the design.
export function useDesignAppUpdateConnection(): ActionHookResult<[connectionGuid: Guid, diff: ConnectionDiff]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuid: Guid, diff: ConnectionDiff) => store.execute("semio.designApp.updateConnection", getOrigin(), { connection: connectionGuid, diff });
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that updates the connections with the given GUID-data pairs.
// Returns an action to update multiple connections in the design.
export function useDesignAppUpdateConnections(): ActionHookResult<[updates: { id: Guid; diff: ConnectionDiff }[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (updates: { id: Guid; diff: ConnectionDiff }[]) =>
      store.execute(
        "semio.designApp.updateConnections",
        getOrigin(),
        updates.map((u) => ({ connection: u.id, diff: u.diff })),
      );
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that clusters the given piece GUIDs.
// Returns an action to cluster selected pieces into a new design.
export function useDesignAppClusterPieces(): ActionHookResult<[pieceGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuids: Guid[]) => store.execute("semio.designApp.clusterPieces", getOrigin(), pieceGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

// MUST return a callback that expands the design with the given piece GUID.
// Returns an action to expand a nested design into inline pieces.
export function useDesignAppExpandDesign(): ActionHookResult<[designGuid: Guid]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (designGuid: Guid) => store.execute("semio.designApp.expandDesign", getOrigin(), designGuid);
  }, [store, getOrigin]);
  return [action, !!store];
}

// #endregion Action Hooks

const EMPTY_COMMANDS = {
  togglePanel: () => { },
  execute: () => { },
  startTransaction: () => { },
  finalizeTransaction: () => { },
  abortTransaction: () => { },
  undo: () => { },
  redo: () => { },
  selectAll: () => { },
  deselectAll: () => { },
  selectPiece: () => { },
  selectPieces: () => { },
  addPieceToSelection: () => { },
  removePieceFromSelection: () => { },
  selectConnection: () => { },
  addConnectionToSelection: () => { },
  removeConnectionFromSelection: () => { },
  selectPiecePort: () => { },
  deselectPiecePort: () => { },
  deleteSelected: () => { },
  toggleDiagramFullscreen: () => { },
  toggleAccesslFullscreen: () => { },
  setActiveTool: () => { },
  addPiece: () => { },
  addPieces: () => { },
  removePiece: () => { },
  removePieces: () => { },
  addConnection: () => { },
  addConnections: () => { },
  removeConnection: () => { },
  removeConnections: () => { },
  updatePiece: () => { },
  updatePieces: () => { },
  updateConnection: () => { },
  updateConnections: () => { },
  setCamera: () => { },
  focusPiece: () => { },
  clearFocus: () => { },
  setDiagramCenter: () => { },
  setDiagramScale: () => { },
  hoverPiece: () => { },
  hoverPieces: () => { },
  hoverConnection: () => { },
  hoverConnections: () => { },
  hoverPort: () => { },
  hoverType: () => { },
  hoverTypes: () => { },
  hoverDesign: () => { },
  hoverDesigns: () => { },
  clearHover: () => { },
  setModelTagsForType: () => { },
  addModelTagForAllTypes: () => { },
  removeModelTagFromAllTypes: () => { },
} as any;

// MUST expose all Design app commands through the store controller.
// Returns the full Design app commands API for programmatic access.
export function useDesignAppCommands(id?: DesignAppId) {
  const store = useDesignStore(undefined, id) as DesignStore | null;
  const actor = useSketchpadActorSafe();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";

  return useMemo(() => {
    if (!store || !actor) {
      return EMPTY_COMMANDS;
    }
    return {
      startTransaction: (origin: string) => store.execute("semio.designApp.startTransaction", origin),
      finalizeTransaction: (origin: string) => store.execute("semio.designApp.finalizeTransaction", origin),
      abortTransaction: (origin: string) => store.execute("semio.designApp.abortTransaction", origin),
      undo: (origin: string) => store.execute("semio.designApp.undo", origin),
      redo: (origin: string) => store.execute("semio.designApp.redo", origin),
      selectAll: (_origin: string) => actor.send({ type: "DESIGN.SELECT_ALL", kitGuid, designGuid }),
      deselectAll: (_origin: string) => actor.send({ type: "DESIGN.CLEAR_SELECTION", kitGuid, designGuid }),
      selectPiece: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SELECT_PIECE", kitGuid, designGuid, pieceGuid: guid }),
      selectPieces: (_origin: string, guids: Guid[]) => guids.forEach((g) => actor.send({ type: "DESIGN.SELECT_PIECE", kitGuid, designGuid, pieceGuid: g })),
      addPieceToSelection: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SELECT_PIECE", kitGuid, designGuid, pieceGuid: guid }),
      removePieceFromSelection: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.DESELECT_PIECE", kitGuid, designGuid, pieceGuid: guid }),
      selectConnection: (_origin: string, connectionGuid: Guid) => actor.send({ type: "DESIGN.SELECT_CONNECTION", kitGuid, designGuid, connectionGuid }),
      addConnectionToSelection: (_origin: string, connectionGuid: Guid) => actor.send({ type: "DESIGN.SELECT_CONNECTION", kitGuid, designGuid, connectionGuid }),
      removeConnectionFromSelection: (_origin: string, connectionGuid: Guid) => actor.send({ type: "DESIGN.DESELECT_CONNECTION", kitGuid, designGuid, connectionGuid }),
      selectPiecePort: (_origin: string, piece: Guid, connector: Guid) => {
        const current = store.snapshot().selection || {};
        actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: current.pieces, connections: current.connections, connectors: [{ piece, connector }] } });
      },
      deselectPiecePort: (_origin: string) => {
        const current = store.snapshot().selection || {};
        actor.send({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: { pieces: current.pieces, connections: current.connections, connectors: undefined } });
      },
      deleteSelected: (origin: string) => store.execute("semio.designApp.deleteSelected", origin),
      toggleDiagramFullscreen: (_origin: string) => {
        const current = store.snapshot().fullscreenWindow;
        actor.send({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: current === DesignAppFullscreenWindow.Diagram ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Diagram });
      },
      toggleAccesslFullscreen: (_origin: string) => {
        const current = store.snapshot().fullscreenWindow;
        actor.send({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: current === DesignAppFullscreenWindow.Accessl ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Accessl });
      },
      setActiveTool: (_origin: string, tool: ToolKind) => actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool }),
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
      setCamera: (_origin: string, camera: Camera) => actor.send({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera }),
      focusPiece: (_origin: string, pieceGuid: Guid) => actor.send({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid }),
      clearFocus: (_origin: string) => actor.send({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid: undefined }),
      setDiagramCenter: (_origin: string, center: Coord) => actor.send({ type: "DESIGN.SET_DIAGRAM_CENTER", kitGuid, designGuid, center: { x: center.u, y: center.v } }),
      setDiagramScale: (_origin: string, scale: number) => actor.send({ type: "DESIGN.SET_DIAGRAM_SCALE", kitGuid, designGuid, scale }),
      hoverPiece: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: [guid] } }),
      hoverPieces: (_origin: string, guids: Guid[]) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: guids } }),
      hoverConnection: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connections: [guid] } }),
      hoverConnections: (_origin: string, guids: Guid[]) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connections: guids } }),
      hoverPort: (_origin: string, pieceGuid: Guid, connectorGuid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connectors: [{ piece: pieceGuid, connector: connectorGuid }] } }),
      hoverType: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { types: [guid] } }),
      hoverTypes: (_origin: string, guids: Guid[]) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { types: guids } }),
      hoverDesign: (_origin: string, guid: Guid) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { designs: [guid] } }),
      hoverDesigns: (_origin: string, guids: Guid[]) => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { designs: guids } }),
      clearHover: (_origin: string) => actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid }),
      togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => actor.send({ type: "DESIGN.TOGGLE_PANEL", kitGuid, designGuid, panel: panelKey }),
      setModelTagsForType: (_origin: string, typeGuid: Guid, tags: string[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        actor.send({ type: "DESIGN.SYNC", kitGuid, designGuid, state: { selectedModelTags: { ...current, [typeGuid]: tags } } });
      },
      addModelTagForAllTypes: (_origin: string, tagGuid: string, typeGuids: Guid[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        const updated: Record<Guid, string[]> = { ...current };
        typeGuids.forEach((typeGuid) => {
          const existing = updated[typeGuid] ?? [];
          if (!existing.includes(tagGuid)) updated[typeGuid] = [...existing, tagGuid];
        });
        actor.send({ type: "DESIGN.SYNC", kitGuid, designGuid, state: { selectedModelTags: updated } });
      },
      removeModelTagFromAllTypes: (_origin: string, tagGuid: string, typeGuids: Guid[]) => {
        const current = store.snapshot().selectedModelTags ?? {};
        const updated: Record<Guid, string[]> = { ...current };
        typeGuids.forEach((typeGuid) => {
          const existing = updated[typeGuid] ?? [];
          updated[typeGuid] = existing.filter((t) => t !== tagGuid);
        });
        actor.send({ type: "DESIGN.SYNC", kitGuid, designGuid, state: { selectedModelTags: updated } });
      },
      execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
    };
  }, [store, actor, kitGuid, designGuid]);
}

// MUST observe Y.js map changes and dispatch corresponding XState events.
// Synchronizes Y.js document changes to XState Design app state.
export function useDesignAppYjsToXStateSync(id?: DesignAppId) {
  const actor = useSketchpadActorSafe();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";

  const state = useDesignApp((s) => s, id);

  useEffect(() => {
    if (!actor || !state || !kitGuid || !designGuid) return;

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

// #endregion 🔖Components

function getTransactionAffectedPieces(store: DesignStore | null): { changedPieces: Set<string>; statusMap: Map<string, DiffStatus> } {
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

interface TransactionPiecesContextValue {
  changedPieces: Set<string>;
  statusMap: Map<string, DiffStatus>;
}

const EMPTY_TRANSACTION_CONTEXT: TransactionPiecesContextValue = {
  changedPieces: new Set(),
  statusMap: new Map(),
};

const TransactionPiecesContext = createContext<TransactionPiecesContextValue>(EMPTY_TRANSACTION_CONTEXT);

function areSetsEqual<T>(a: Set<T>, b: Set<T>): boolean {
  if (a.size !== b.size) return false;
  for (const item of a) if (!b.has(item)) return false;
  return true;
}

function areMapsEqual<K, V>(a: Map<K, V>, b: Map<K, V>): boolean {
  if (a.size !== b.size) return false;
  for (const [key, value] of a) if (b.get(key) !== value) return false;
  return true;
}

function areTransactionContextsEqual(a: TransactionPiecesContextValue, b: TransactionPiecesContextValue): boolean {
  return areSetsEqual(a.changedPieces, b.changedPieces) && areMapsEqual(a.statusMap, b.statusMap);
}

function TransactionPiecesProviderInner({ store, children }: { store: DesignStore; children: ReactNode }) {
  const lastResultRef = useRef<TransactionPiecesContextValue>(EMPTY_TRANSACTION_CONTEXT);
  const subscribe = useCallback((callback: () => void) => store.subscribe(callback), [store]);
  const getSnapshot = useCallback(() => {
    const result = getTransactionAffectedPieces(store);
    if (areTransactionContextsEqual(result, lastResultRef.current)) return lastResultRef.current;
    lastResultRef.current = result;
    return result;
  }, [store]);
  const transactionData = useSyncExternalStore(subscribe, getSnapshot);
  return <TransactionPiecesContext.Provider value={transactionData}>{children}</TransactionPiecesContext.Provider>;
}

// MUST compute and provide the set of piece GUIDs changed in the current transaction.
// Provider that makes transaction-changed piece GUIDs available to children.
export function TransactionPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignStore(identitySelector) as DesignStore | null;

  if (!store) {
    return <TransactionPiecesContext.Provider value={EMPTY_TRANSACTION_CONTEXT}>{children}</TransactionPiecesContext.Provider>;
  }

  return <TransactionPiecesProviderInner store={store}>{children}</TransactionPiecesProviderInner>;
}

// MUST check the transaction pieces context for the given GUID.
// Returns whether a piece is changed in the current transaction.
export function useIsDesignPieceChangedInTransaction(id: DesignAppId | undefined, pieceId: string): boolean {
  const { changedPieces } = useContext(TransactionPiecesContext);
  return changedPieces.has(pieceId);
}

// MUST check the hover state for the given piece GUID.
// Returns whether a piece is currently hovered in the Design app.
export function useDesignAppIsPieceHovered(id?: DesignAppId, pieceId?: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignHoverSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const hover = useSelector(actor, selector);
  if (!pieceId) return false;
  return hover?.pieces?.includes(pieceId) ?? false;
}

interface HoverPiecesContextValue {
  transitivelyHoveredPieces: Set<string>;
  transitivelyHoveredTypes: Set<string>;
}

const EMPTY_HOVER_CONTEXT: HoverPiecesContextValue = {
  transitivelyHoveredPieces: new Set(),
  transitivelyHoveredTypes: new Set(),
};

const HoverPiecesContext = createContext<HoverPiecesContextValue>(EMPTY_HOVER_CONTEXT);

function computeHoverData(store: DesignStore | null, state: DesignAppState): HoverPiecesContextValue {
  const hover = state.hover;
  const transitivelyHoveredPieces = new Set<string>();
  const transitivelyHoveredTypes = new Set<string>();

  if (!hover) return { transitivelyHoveredPieces, transitivelyHoveredTypes };

  hover.pieces?.forEach((pieceId) => transitivelyHoveredPieces.add(pieceId));

  hover.types?.forEach((typeId) => transitivelyHoveredTypes.add(typeId));

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

function areHoverContextsEqual(a: HoverPiecesContextValue, b: HoverPiecesContextValue): boolean {
  return areSetsEqual(a.transitivelyHoveredPieces, b.transitivelyHoveredPieces) && areSetsEqual(a.transitivelyHoveredTypes, b.transitivelyHoveredTypes);
}

function HoverPiecesProviderInner({ store, children }: { store: DesignStore; children: ReactNode }) {
  const lastResultRef = useRef<HoverPiecesContextValue>(EMPTY_HOVER_CONTEXT);
  const subscribe = useCallback((callback: () => void) => store.subscribe(callback), [store]);
  const getSnapshot = useCallback(() => {
    const state = store.snapshot();
    const result = computeHoverData(store, state);
    if (areHoverContextsEqual(result, lastResultRef.current)) return lastResultRef.current;
    lastResultRef.current = result;
    return result;
  }, [store]);
  const hoverData = useSyncExternalStore(subscribe, getSnapshot);
  return <HoverPiecesContext.Provider value={hoverData}>{children}</HoverPiecesContext.Provider>;
}

// MUST compute and provide the set of piece GUIDs that are transitively hovered.
// Provider that makes transitively hovered piece GUIDs available to children.
export function HoverPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignStore(identitySelector) as DesignStore | null;

  if (!store) {
    return <HoverPiecesContext.Provider value={EMPTY_HOVER_CONTEXT}>{children}</HoverPiecesContext.Provider>;
  }

  return <HoverPiecesProviderInner store={store}>{children}</HoverPiecesProviderInner>;
}

// MUST check the transitive hover pieces for the given GUID.
// Returns whether a piece is transitively hovered via type or design hierarchy.
export function useDesignAppIsPieceTransitiveHovered(id?: DesignAppId, pieceId?: string): boolean {
  const { transitivelyHoveredPieces } = useContext(HoverPiecesContext);
  if (!pieceId) return false;
  return transitivelyHoveredPieces.has(pieceId);
}

// MUST check the hover state for the given type GUID.
// Returns whether a type is transitively hovered in the Design app.
export function useDesignAppIsTypeTransitiveHovered(id: DesignAppId | undefined, typeId: string): boolean {
  const { transitivelyHoveredTypes } = useContext(HoverPiecesContext);
  return transitivelyHoveredTypes.has(typeId);
}

// MUST return DiffStatus from the design diff for the given piece GUID.
// Returns the diff status of a piece for visual indication.
export function useDesignAppPieceStatus(id: DesignAppId | undefined, pieceId: string): DiffStatus {
  const { statusMap } = useContext(TransactionPiecesContext);
  return statusMap.get(pieceId) ?? DiffStatus.Unchanged;
}

// MUST check the selection state for the given piece GUID.
// Returns whether a piece is currently selected in the Design app.
export function useDesignAppIsPieceSelected(id?: DesignAppId, pieceId?: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignSelectionSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const selection = useSelector(actor, selector);
  if (!pieceId) return false;
  return selection?.pieces?.includes(pieceId) ?? false;
}

// MUST derive the color from selection, hover, diff status, and type mapping.
// Returns the computed color for a piece based on its status.
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

// MUST check the hover state for the given connection GUID.
// Returns whether a connection is currently hovered in the Design app.
export function useDesignAppIsConnectionHovered(id?: DesignAppId, connectionId?: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignHoverSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const hover = useSelector(actor, selector);
  if (!connectionId) return false;
  return hover?.connections?.includes(connectionId) ?? false;
}

// MUST check the selection state for the given connection GUID.
// Returns whether a connection is currently selected in the Design app.
export function useDesignAppIsConnectionSelected(id?: DesignAppId, connectionId?: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignSelectionSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const selection = useSelector(actor, selector);
  if (!connectionId) return false;
  return selection?.connections?.includes(connectionId) ?? false;
}

// MUST check the hover state for the given piece-connector port.
// Returns whether a port is currently hovered in the Design app.
export function useDesignAppIsPortHovered(id: DesignAppId | undefined, pieceId: string, connectorId: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignHoverSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const hover = useSelector(actor, selector);
  return hover?.connectors?.some((p) => p.piece === pieceId && p.connector === connectorId) ?? false;
}

type SelectedConnector = { piece: Guid; connector: Guid } | undefined;
const EMPTY_CONNECTOR: SelectedConnector = undefined;

// MUST return the currently selected connector from the selection state.
// Returns the selected connector for the Design app.
export function useDesignAppSelectedConnector(id?: DesignAppId): SelectedConnector {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const designGuid = designScope?.guid ?? id?.design ?? "";
  const selector = useMemo(() => createDesignSelectionSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const selection = useSelector(actor, selector);
  const connector = selection?.connectors?.[0];
  if (!connector?.piece || !connector?.connector) return EMPTY_CONNECTOR;
  return { piece: connector.piece, connector: connector.connector };
}

// MUST check the selection connector state for the given piece-connector pair.
// Returns whether a specific piece port is currently selected.
export function useDesignAppIsPiecePortSelected(pieceId: string, connectorId?: string): boolean {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kitGuid = kitScope?.guid ?? "";
  const designGuid = designScope?.guid ?? "";
  const selector = useMemo(() => createDesignSelectionSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  const selection = useSelector(actor, selector);
  if (!connectorId) return false;
  return selection?.connectors?.some((c) => c.piece === pieceId && c.connector === connectorId) ?? false;
}

function getConnectionStatusFromTransactionStack(store: DesignStore | null, connectionId: string): DiffStatus {
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

// MUST return DiffStatus from the design diff for the given connection GUID.
// Returns the diff status of a connection for visual indication.
export function useDesignAppConnectionStatus(id: DesignAppId | undefined, connectionId: string): DiffStatus {
  const store = useDesignStore(identitySelector, id) as DesignStore | null;
  return useMemo(() => getConnectionStatusFromTransactionStack(store, connectionId), [store, connectionId]);
}

// MUST derive the color from selection, hover, and diff status.
// Returns the computed color for a connection based on its status.
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

// MUST look up the piece metadata for the given GUID and return its center.
// Returns the center position of a piece on the canvas.
export function useDesignAppPieceCenter(id?: DesignAppId, pieceId?: Guid): Coord | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.center : undefined;
}

// MUST look up the piece metadata for the given GUID and return its plane.
// Returns the plane orientation of a piece.
export function useDesignAppPiecePlane(id?: DesignAppId, pieceId?: Guid): Plane | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.plane : undefined;
}

// #endregion 🔖Store

// #region 🔖Footer
// Footer MUST render dynamic Design app footer items showing selection and transaction state.

// MUST register and unregister footer items based on selection and transaction state.
// Footer component that renders dynamic Design app footer status items.
export const DesignAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const design = useDesign() as Design | undefined;
  const types = useKitTypes();
  const tags = useKitTags();
  const [selectedModelTags] = useDesignAppSelectedModelTags();
  const [addModelTagForAllTypes] = useDesignAppAddModelTagForAllTypes();
  const [removeModelTagFromAllTypes] = useDesignAppRemoveModelTagFromAllTypes();

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

  const { allModelTagGuids, tagNameMap } = useMemo(() => {
    if (!types || designTypeGuids.length === 0) return { allModelTagGuids: [], tagNameMap: new Map<string, string>() };
    const tagGuids = new Set<string>();
    const nameMap = new Map<string, string>();
    designTypeGuids.forEach((typeGuid) => {
      const type = types.find((t) => t.guid === typeGuid);
      type?.models?.forEach((model) => {
        model.tags?.forEach((tag) => {
          tagGuids.add(tag.guid);
        });
      });
    });

    tags.forEach((tag) => {
      if (!nameMap.has(tag.guid)) {
        nameMap.set(tag.guid, tag.name);
      }
    });
    return { allModelTagGuids: Array.from(tagGuids), tagNameMap: nameMap };
  }, [types, designTypeGuids, tags]);

  const typesRef = useRef(types);
  const designTypeGuidsRef = useRef(designTypeGuids);
  const selectedModelTagsRef = useRef(selectedModelTags);
  const addModelTagForAllTypesRef = useRef(addModelTagForAllTypes);
  const removeModelTagFromAllTypesRef = useRef(removeModelTagFromAllTypes);

  useEffect(() => {
    typesRef.current = types;
    designTypeGuidsRef.current = designTypeGuids;
    selectedModelTagsRef.current = selectedModelTags;
    addModelTagForAllTypesRef.current = addModelTagForAllTypes;
    removeModelTagFromAllTypesRef.current = removeModelTagFromAllTypes;
  }, [types, designTypeGuids, selectedModelTags, addModelTagForAllTypes, removeModelTagFromAllTypes]);

  useEffect(() => {
    if (appType !== "design") return;

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

    allModelTagGuids.forEach((tagGuid) => {
      removeFooterItem(`semio.sketchpad.app.design.footer.tag.${tagGuid}`);
    });

    allModelTagGuids.forEach((tagGuid, index) => {
      const tagName = tagNameMap.get(tagGuid) || tagGuid.slice(0, 8);
      const selected = isTagSelected(tagGuid);
      const typesWithTag = getTypesWithTag(tagGuid);

      addFooterItem({
        id: `semio.sketchpad.app.design.footer.tag.${tagGuid}`,
        text: tagName,
        className: selected ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground",
        onClick: () => {
          const currentSelected = isTagSelected(tagGuid);
          const currentTypesWithTag = getTypesWithTag(tagGuid);
          if (currentSelected) {
            removeModelTagFromAllTypesRef.current?.(tagGuid, currentTypesWithTag);
          } else {
            addModelTagForAllTypesRef.current?.(tagGuid, currentTypesWithTag);
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
  }, [appType, addFooterItem, removeFooterItem, allModelTagGuids, tagNameMap, selectedModelTags, designTypeGuids]);

  return null;
};

// #endregion 🔖Footer

// #region 🔖Tools
// Tools MUST define all Design app tool configurations for selection, lasso, and hand modes.

// Tool configuration for normal selection mode.
export const SelectionNormalTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_NORMAL,
  icon: <SelectToolIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Tool configuration for additive selection mode.
export const SelectionAdditiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_ADDITIVE,
  icon: <AddIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Tool configuration for subtractive selection mode.
export const SelectionSubtractiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_SUBTRACTIVE,
  icon: <RemoveIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Tool configuration for rectangular lasso selection mode.
export const LassoRectangularTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_RECTANGULAR,
  icon: <DiagramIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Tool configuration for freeform lasso selection mode.
export const LassoFreeformTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_FREEFORM,
  icon: <SceneIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Tool configuration for hand/pan mode.
export const HandTool: Tool<DesignAppState> = {
  id: ToolKind.HAND,
  icon: <HandIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

// Array of all Design app tool configurations.
export const DesignAppTools: Tool<DesignAppState>[] = [SelectionNormalTool, SelectionAdditiveTool, SelectionSubtractiveTool, LassoRectangularTool, LassoFreeformTool, HandTool];

// MUST render toggle buttons for each selection sub-mode.
// Settings component for the selection tool group with additive, subtractive, and intersect toggles.
export const DesignSelectSettings: FC = () => {
  const [activeTool, setActiveTool] = useDesignAppActiveTool();
  const additiveLabel = useLabel("semio.sketchpad.app.design.tools.select.additive");
  const subtractiveLabel = useLabel("semio.sketchpad.app.design.tools.select.subtractive");
  const intersectLabel = useLabel("semio.sketchpad.app.design.tools.select.intersect");

  useEffect(() => {
    if (activeTool === ToolKind.HAND && setActiveTool) {
      setActiveTool(ToolKind.SELECTION_NORMAL);
    }
  }, [setActiveTool]);

  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <Toggle
        id="semio.sketchpad.app.design.tools.select.additive"
        icon={<AddIcon className="size-tiny" />}
        text={additiveLabel}
        pressed={activeTool === ToolKind.SELECTION_ADDITIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_ADDITIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.subtractive"
        icon={<RemoveIcon className="size-tiny" />}
        text={subtractiveLabel}
        pressed={activeTool === ToolKind.SELECTION_SUBTRACTIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_SUBTRACTIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.intersect"
        icon={<IntersectIcon className="size-tiny" />}
        text={intersectLabel}
        pressed={activeTool === ToolKind.SELECTION_INTERSECT}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_INTERSECT : ToolKind.SELECTION_NORMAL)}
      />
    </div>
  );
};

// MUST activate the hand tool on mount.
// Settings component for the hand tool that activates hand mode.
export const DesignHandSettings: FC = () => {
  const [activeTool, setActiveTool] = useDesignAppActiveTool();

  useEffect(() => {
    if (activeTool !== ToolKind.HAND && setActiveTool) {
      setActiveTool(ToolKind.HAND);
    }
  }, [setActiveTool]);

  return null;
};

// MUST render toggle group for lasso sub-modes.
// Settings component for the lasso tool with rectangular and freeform toggles.
export const DesignLassoSettings: FC = () => {
  const [activeTool, setActiveTool] = useDesignAppActiveTool();
  const rectangularLabel = useLabel("semio.sketchpad.app.design.tools.lasso.rectangular");
  const freeformLabel = useLabel("semio.sketchpad.app.design.tools.lasso.freeform");

  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <ToggleGroup
        items={[
          { value: String(ToolKind.LASSO_RECTANGULAR), icon: <DiagramIcon className="size-tiny" />, text: rectangularLabel, id: "semio.sketchpad.app.design.tools.lasso.rectangular" },
          { value: String(ToolKind.LASSO_FREEFORM), icon: <SceneIcon className="size-tiny" />, text: freeformLabel, id: "semio.sketchpad.app.design.tools.lasso.freeform" }
        ]}
        value={activeTool !== undefined ? [String(activeTool)] : []}
        onValueChange={(vals) => vals[0] && setActiveTool && setActiveTool(Number(vals[0]) as ToolKind)}
        kind="single"
      />
    </div>
  );
};

// #endregion Tools

// #region 🔖Panels

// #region 🔖WindowLibrary
// WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

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

// MUST render categorized window templates for scene, diagram, and table types.
// Panel component that renders the draggable window template library.
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

// #endregion 🔖WindowLibrary

// #region 🔖Details
// Details MUST render the Design app detail panels for design, pieces, connections, and connector sections.

// MUST render the design form fields within a detail panel section.
// Detail section component for the currently open design.
export const DesignSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <DesignSectionForm />;
};

const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const [transaction] = useDesignAppTransaction();
  const kitCommands = useKitCommands();
  const design = useDesign() as Design;

  if (!design) return null;

  const updateDesignField = (diff: any) => {
    if (!kitCommands) return;
    kitCommands.updateDesign(design.guid, diff);
  };

  const addLocation = () => {
    transaction?.start();
    updateDesignField({ location: { guid: guid(), longitude: 0, latitude: 0 } });
    transaction?.finalize();
  };

  const removeLocation = () => {
    transaction?.start();
    updateDesignField({ location: undefined });
    transaction?.finalize();
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.name" value={design.name} onLazyChange={(value) => updateDesignField({ name: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.description"
            value={design.description || ""}
            placeholderId="semio.sketchpad.app.design.descriptionPlaceholder"
            onLazyChange={(value) => updateDesignField({ description: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.icon" value={design.icon || ""} placeholderId="semio.sketchpad.app.design.iconPlaceholder" onLazyChange={(value) => updateDesignField({ icon: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.image" value={design.image || ""} placeholderId="semio.sketchpad.app.design.imagePlaceholder" onLazyChange={(value) => updateDesignField({ image: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.variant"
            value={(design as any).variant || ""}
            placeholderId="semio.sketchpad.app.design.variantPlaceholder"
            onLazyChange={(value) => updateDesignField({ variant: value })}
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
            onLazyChange={(value) => updateDesignField({ view: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.unit" value={design.unit || ""} onLazyChange={(value) => updateDesignField({ unit: value })} showLabel />
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
                  updateDesignField({
                    location: { ...(design.location as any)!, longitude: value },
                  })
                }
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
                  updateDesignField({
                    location: { ...(design.location as any)!, latitude: value },
                  })
                }
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
              transaction?.start();
              updateDesignField({
                authors: [...(design.authors || []), { name: "", email: "" }],
              });
              transaction?.finalize();
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
              transaction?.start();
              updateDesignField({
                authors: arrayMove(design.authors!, oldIndex, newIndex),
              });
              transaction?.finalize();
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
                      transaction?.start();
                      updateDesignField({
                        authors: design.authors?.filter((_: any, i: number) => i !== index),
                      });
                      transaction?.finalize();
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
                        updateDesignField({ authors: updatedAuthors });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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
                        updateDesignField({ authors: updatedAuthors });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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
              transaction?.start();
              updateDesignField({
                attributes: [...(design.attributes || []), { key: "" }],
              });
              transaction?.finalize();
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
              transaction?.start();
              updateDesignField({
                attributes: arrayMove(design.attributes!, oldIndex, newIndex),
              });
              transaction?.finalize();
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
                      transaction?.start();
                      updateDesignField({
                        attributes: design.attributes?.filter((_: any, i: number) => i !== index),
                      });
                      transaction?.finalize();
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
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={() => transaction?.start()}
                      onBlur={() => transaction?.finalize()}
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

// MUST render each piece with its type, name, and selection interactions.
// Detail section component for the design pieces list.
export const PiecesSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PiecesSectionForm />;
};

const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const [transaction] = useDesignAppTransaction();
  const [updatePiece] = useDesignAppUpdatePiece();
  const [updatePieces] = useDesignAppUpdatePieces();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;
  const kitCommands = useKitCommands();
  const includedDesigns = useIncludedDesigns();
  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((includedDesign) => [includedDesign.guid, includedDesign])), [includedDesigns]);
  const metadata = new Map();
  const [selection] = useDesignAppSelection();
  const pieces = usePiecesFromIds(selection.pieces || []);

  const isSingle = pieces.length === 1;
  const piece = isSingle ? pieces[0] : null;

  const isDesignPieceEntry = (target: any) => {
    if (target?.design) return true;
    if (typeof target?.type === "string") return target.type === "design";
    return target?.type?.name === "design";
  };

  const isDesignPiece = isSingle ? isDesignPieceEntry(piece) : pieces.every((p) => isDesignPieceEntry(p));
  const hasDesignPieces = pieces.some((p) => isDesignPieceEntry(p));
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => !isDesignPieceEntry(p));

  const getCommonValue = <T,>(getter: (piece: any) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const getPieceId = (p: any): string => (p as any).guid || (p as any).id_;
  const isRealPiece = (p: any): boolean => typeof (p as any).guid === "string";
  const parseDesignVariant = (variant: string) => {
    const [name, variantPart, viewPart] = variant.split("-");
    return { name, variant: variantPart || undefined, view: viewPart || undefined };
  };
  const buildDesignVariant = (name: string, variant?: string, view?: string) => {
    const parts = [name, variant, view].filter((part) => part && part.length > 0) as string[];
    return parts.join("-");
  };

  const handleTypeNameChange = (value: string) => {
    if (!value) return;
    if (isDesignPiece) return;
    const match = availableTypes.find((t) => t.name === value) || allReplacableTypes.find((t) => t.name === value);
    if (!match) return;
    if (isSingle && piece && isRealPiece(piece)) {
      transaction?.start();
      updatePiece?.(getPieceId(piece), { type: { guid: match.guid } });
      transaction?.finalize();
      return;
    }
    const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { type: { guid: match.guid } } }));
    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const handleTypeVariantChange = (value: string) => {
    if (isDesignPiece) return;
    const variantValue = value || undefined;
    const resolveType = (name: string, variant?: string) => {
      const candidates = allReplacableTypes.filter((t) => t.name === name);
      if (variant !== undefined) {
        const exact = candidates.find((t) => ((t as any).variant || "") === variant);
        if (exact) return exact;
      } else {
        const base = candidates.find((t) => !((t as any).variant));
        if (base) return base;
      }
      return candidates[0];
    };

    if (isSingle && piece && isRealPiece(piece)) {
      const currentType = piece.type && typeof piece.type === "string" ? findTypeInKit(kit, piece.type) : piece.type?.guid ? findTypeInKit(kit, piece.type.guid) : null;
      if (!currentType) return;
      const match = resolveType(currentType.name, variantValue);
      if (!match) return;
      transaction?.start();
      updatePiece?.(getPieceId(piece), { type: { guid: match.guid } });
      transaction?.finalize();
      return;
    }

    const updates = pieces
      .filter(isRealPiece)
      .map((p) => {
        const currentType = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : p.type?.guid ? findTypeInKit(kit, p.type.guid) : null;
        if (!currentType) return null;
        const match = resolveType(currentType.name, variantValue);
        if (!match) return null;
        return { id: getPieceId(p), diff: { type: { guid: match.guid } } };
      })
      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const handleDesignNameChange = (value: string) => {
    if (!isDesignPiece || !value) return;
    const updateDesignGuid = (targetPiece: any, name: string) => {
      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
      const variant = (currentDesign as any)?.variant || undefined;
      const view = (currentDesign as any)?.view || undefined;
      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
      return match?.guid;
    };

    if (isSingle && piece) {
      const pieceId = getPieceId(piece);
      const includedDesign = includedDesignMap.get(pieceId);
      if (includedDesign?.type === "connected") {
        console.warn("Connected design pieces cannot be renamed - they represent clustered designs");
        return;
      }
      if (!isRealPiece(piece)) return;
      if (piece.design?.guid) {
        const matchGuid = updateDesignGuid(piece, value);
        if (!matchGuid) return;
        transaction?.start();
        updatePiece?.(pieceId, { design: { guid: matchGuid } });
        transaction?.finalize();
        return;
      }
      const current = parseDesignVariant((piece as any).type?.variant || "");
      const newVariant = buildDesignVariant(value, current.variant, current.view);
      transaction?.start();
      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
      transaction?.finalize();
      return;
    }

    const updates = pieces
      .filter(isRealPiece)
      .map((p) => {
        if (p.design?.guid) {
          const matchGuid = updateDesignGuid(p, value);
          if (!matchGuid) return null;
          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
        }
        if ((p as any).type?.name === "design") {
          const current = parseDesignVariant((p as any).type?.variant || "");
          const newVariant = buildDesignVariant(value, current.variant, current.view);
          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
        }
        return null;
      })
      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const handleDesignVariantChange = (value: string) => {
    if (!isDesignPiece) return;
    const nextVariant = value || undefined;
    const updateDesignGuid = (targetPiece: any, variant?: string) => {
      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
      const name = currentDesign?.name || "";
      const view = (currentDesign as any)?.view || undefined;
      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
      return match?.guid;
    };

    if (isSingle && piece) {
      const pieceId = getPieceId(piece);
      const includedDesign = includedDesignMap.get(pieceId);
      if (includedDesign?.type === "connected") {
        console.warn("Connected design pieces cannot have their variants changed - they represent clustered designs");
        return;
      }
      if (!isRealPiece(piece)) return;
      if (piece.design?.guid) {
        const matchGuid = updateDesignGuid(piece, nextVariant);
        if (!matchGuid) return;
        transaction?.start();
        updatePiece?.(pieceId, { design: { guid: matchGuid } });
        transaction?.finalize();
        return;
      }
      const current = parseDesignVariant((piece as any).type?.variant || "");
      const newVariant = buildDesignVariant(current.name, nextVariant, current.view);
      transaction?.start();
      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
      transaction?.finalize();
      return;
    }

    const updates = pieces
      .filter(isRealPiece)
      .map((p) => {
        if (p.design?.guid) {
          const matchGuid = updateDesignGuid(p, nextVariant);
          if (!matchGuid) return null;
          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
        }
        if ((p as any).type?.name === "design") {
          const current = parseDesignVariant((p as any).type?.variant || "");
          const newVariant = buildDesignVariant(current.name, nextVariant, current.view);
          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
        }
        return null;
      })
      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const handleDesignViewChange = (value: string) => {
    if (!isDesignPiece) return;
    const nextView = value || undefined;
    const updateDesignGuid = (targetPiece: any, view?: string) => {
      const currentDesign = targetPiece.design?.guid ? findDesignInKit(kit, targetPiece.design.guid) : null;
      const name = currentDesign?.name || "";
      const variant = (currentDesign as any)?.variant || undefined;
      const options = currentDesign ? [currentDesign, ...availableDesigns] : availableDesigns.length > 0 ? availableDesigns : kit.designs || [];
      const match = options.find((d) => d.name === name && ((d as any).variant || "") === (variant || "") && ((d as any).view || "") === (view || ""));
      return match?.guid;
    };

    if (isSingle && piece) {
      const pieceId = getPieceId(piece);
      const includedDesign = includedDesignMap.get(pieceId);
      if (includedDesign?.type === "connected") {
        console.warn("Connected design pieces cannot have views changed - they represent clustered designs");
        return;
      }
      if (!isRealPiece(piece)) return;
      if (piece.design?.guid) {
        const matchGuid = updateDesignGuid(piece, nextView);
        if (!matchGuid) return;
        transaction?.start();
        updatePiece?.(pieceId, { design: { guid: matchGuid } });
        transaction?.finalize();
        return;
      }
      const current = parseDesignVariant((piece as any).type?.variant || "");
      const newVariant = buildDesignVariant(current.name, current.variant, nextView);
      transaction?.start();
      updatePiece?.(pieceId, { type: { ...(piece as any).type, name: "design", variant: newVariant } as any });
      transaction?.finalize();
      return;
    }

    const updates = pieces
      .filter(isRealPiece)
      .map((p) => {
        if (p.design?.guid) {
          const matchGuid = updateDesignGuid(p, nextView);
          if (!matchGuid) return null;
          return { id: getPieceId(p), diff: { design: { guid: matchGuid } } };
        }
        if ((p as any).type?.name === "design") {
          const current = parseDesignVariant((p as any).type?.variant || "");
          const newVariant = buildDesignVariant(current.name, current.variant, nextView);
          return { id: getPieceId(p), diff: { type: { ...(p as any).type, name: "design", variant: newVariant } as any } };
        }
        return null;
      })
      .filter((update): update is { id: Guid; diff: PieceDiff } => update !== null);

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const fixPieces = async () => {
    if (!design || !kit) return;
    const pieceGuids = pieces.filter(isRealPiece).map((p) => getPieceId(p));
    if (pieceGuids.length === 0) return;
    const diff = fixPiecesInDesign(kit, design.guid, pieceGuids);
    transaction?.start();
    kitCommands?.updateDesign(design.guid, diff);
    transaction?.finalize();
  };

  const handleCenterXChange = (value: number) => {
    if (isSingle && piece) {
      updatePiece?.(getPieceId(piece), { center: { u: value, v: piece.center?.v ?? 0 } });
    } else {
      const updates = pieces.map((p) => ({ id: getPieceId(p), diff: { center: { u: value, v: p.center?.v ?? 0 } } }));
      updatePieces?.(updates);
    }
  };

  const handleCenterYChange = (value: number) => {
    if (isSingle && piece) {
      updatePiece?.(getPieceId(piece), { center: { u: piece.center?.u ?? 0, v: value } });
    } else {
      const updates = pieces.map((p) => ({ id: getPieceId(p), diff: { center: { u: p.center?.u ?? 0, v: value } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneOriginXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, x: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneOriginYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, y: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneOriginZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, z: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneXAxisXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, x: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneXAxisYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, y: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneXAxisZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, z: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneYAxisXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, x: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneYAxisYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, y: value } } } }));
      updatePieces?.(updates);
    }
  };

  const handlePlaneYAxisZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece?.(getPieceId(piece), { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: getPieceId(p), diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, z: value } } } }));
      updatePieces?.(updates);
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
              <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.center.x" value={isSingle && piece ? piece.center?.u : commonCenterX} onChange={handleCenterXChange} step={0.1} />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.center.y" value={isSingle && piece ? piece.center?.v : commonCenterY} onChange={handleCenterYChange} step={0.1} />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      )}
      {isSingle && piece && !piece.plane && (
        <TreeItem>
          <TreeContent>
            <div className="flex flex-col gap-single">
              <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.piece.connectedPieceInfo")}</p>
              <Button id="semio.sketchpad.app.design.piece.fixPiece" onClick={fixPieces}>
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
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x" value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX} onChange={handlePlaneOriginXChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y" value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY} onChange={handlePlaneOriginYChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z" value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ} onChange={handlePlaneOriginZChange} step={0.1} />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem id="semio.sketchpad.app.design.piece.planeXAxis">
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x" value={isSingle && piece ? piece.plane?.xAxis.x : commonPlaneXAxisX} onChange={handlePlaneXAxisXChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y" value={isSingle && piece ? piece.plane?.xAxis.y : commonPlaneXAxisY} onChange={handlePlaneXAxisYChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z" value={isSingle && piece ? piece.plane?.xAxis.z : commonPlaneXAxisZ} onChange={handlePlaneXAxisZChange} step={0.1} />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem id="semio.sketchpad.app.design.piece.planeYAxis">
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x" value={isSingle && piece ? piece.plane?.yAxis.x : commonPlaneYAxisX} onChange={handlePlaneYAxisXChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y" value={isSingle && piece ? piece.plane?.yAxis.y : commonPlaneYAxisY} onChange={handlePlaneYAxisYChange} step={0.1} />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z" value={isSingle && piece ? piece.plane?.yAxis.z : commonPlaneYAxisZ} onChange={handlePlaneYAxisZChange} step={0.1} />
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

// MUST render each connection with its connected pieces and ports.
// Detail section component for the design connections list.
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
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingConnectorId" value={connection.connecting.connector?.guid ?? ""} disabled showLabel />
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
          <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedConnectorId" value={connection.connected.connector?.guid ?? ""} disabled showLabel />
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
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.gap" value={gap} onChange={setGap!} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.shift" value={shift} onChange={setShift!} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.rise" value={rise} onChange={setRise!} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.rotation")}</label>
            <Slider id="semio.sketchpad.app.design.panel.details.section.connection.rotation" value={[rotation]} onValueChange={([value]) => setRotation!(value)} min={-180} max={180} step={1} />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.turn")}</label>
            <Slider id="semio.sketchpad.app.design.panel.details.section.connection.turn" value={[turn]} onValueChange={([value]) => setTurn!(value)} min={-180} max={180} step={1} />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-single">
            <label className="text-xs">{useLabel("semio.sketchpad.app.design.connection.tilt")}</label>
            <Slider id="semio.sketchpad.app.design.panel.details.section.connection.tilt" value={[tilt]} onValueChange={([value]) => setTilt!(value)} min={-180} max={180} step={1} />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.u" value={u} onChange={setU!} step={0.1} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.v" value={v} onChange={setV!} step={0.1} />
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

// MUST render the connector detail form for the selected port.
// Detail section component for the currently selected connector.
export const ConnectorSection: FC<{ pieceGuid: Guid; connectorGuid: Guid }> = ({ pieceGuid, connectorGuid }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <ConnectorSectionForm pieceGuid={pieceGuid} connectorGuid={connectorGuid} />;
};

const ConnectorSectionForm: FC<{ pieceGuid: Guid; connectorGuid: Guid }> = ({ pieceGuid, connectorGuid }) => {
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
  const connector = type?.connectors?.find((p) => p.guid === connectorGuid);

  if (!piece || !type || !connector) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.panel.details.section.connector.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.id" value={connector.guid || "~default~"} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {connector.description && (
        <TreeItem>
          <TreeContent>
            <Textarea id="semio.sketchpad.app.design.panel.details.section.connector.description" value={connector.description} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {connector.port && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.connector.port" value={connector.port.guid} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      {connector.mandatory !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.design.panel.details.section.connector.mandatory" value={connector.mandatory ? useLabel("semio.sketchpad.common.yes") : useLabel("semio.sketchpad.common.no")} disabled showLabel />
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.position" value={`(${connector.point.x.toFixed(2)}, ${connector.point.y.toFixed(2)}, ${connector.point.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.direction" value={`(${connector.direction.x.toFixed(2)}, ${connector.direction.y.toFixed(2)}, ${connector.direction.z.toFixed(2)})`} disabled showLabel />
        </TreeContent>
      </TreeItem>
      {(connector as any).compatiblePorts &&
        (connector as any).compatiblePorts.map((port_: string, index: number) => (
          <TreeItem key={`compatible-interface-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connector.compatiblePort" value={port_} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
      {connector.attributes &&
        connector.attributes.map((attribute: any, index: number) => (
          <TreeItem key={`connector-attribute-${index}`}>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.connector.attribute" value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled showLabel />
            </TreeContent>
          </TreeItem>
        ))}
    </>
  );
};

// #endregion 🔖Details

// #endregion 🔖Panels

// #region 🔖Canvas

// #region 🔖Hover Intent Context
// Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

interface HoverIntentContextValue {
  hoverClearTimeoutRef: React.MutableRefObject<NodeJS.Timeout | null>;
  currentHoveredPieceGuidRef: React.MutableRefObject<string | null>;
  isPanningRef: React.MutableRefObject<boolean>;
  isDraggingNodeRef: React.MutableRefObject<boolean>;
}

const HoverIntentContext = createContext<HoverIntentContextValue | null>(null);

function HoverIntentProvider({ children }: { children: ReactNode }) {
  const hoverClearTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const currentHoveredPieceGuidRef = useRef<string | null>(null);
  const isPanningRef = useRef<boolean>(false);
  const isDraggingNodeRef = useRef<boolean>(false);
  const value = useMemo(
    () => ({
      hoverClearTimeoutRef,
      currentHoveredPieceGuidRef,
      isPanningRef,
      isDraggingNodeRef,
    }),
    [],
  );
  return <HoverIntentContext.Provider value={value}>{children}</HoverIntentContext.Provider>;
}

function useHoverIntent(): HoverIntentContextValue {
  const context = useContext(HoverIntentContext);
  if (!context) throw new Error("useHoverIntent must be used within HoverIntentProvider");
  return context;
}

// #endregion 🔖Hover Intent Context

type SemioConnection = Connection;

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

const PieceRenderDataContext = createContext<Map<string, PieceRenderData>>(new Map());

function usePieceRenderData(pieceGuid: string): PieceRenderData {
  const dataMap = useContext(PieceRenderDataContext);
  return dataMap.get(pieceGuid) ?? EMPTY_PIECE_RENDER_DATA;
}

// #region 🔖Diagram
// Diagram MUST render the interactive React Flow design diagram with nodes, edges, minimap, and controls.

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
              <Button id="semio.sketchpad.app.design.diagram.clusterMenu.cluster" className="px-3 py-single text-sm" onClick={() => onCluster(groupPieceIds)}>
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
        const designGuid = piece.type?.guid;
        const design = designGuid ? findDesignInKit(kit, designGuid) : null;
        const designName = design?.name ?? "";

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
              <Button id="semio.sketchpad.app.design.diagram.expandMenu.expand" className="px-3 py-single text-sm" onClick={() => designGuid && onExpand(designGuid)}>
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

type ConnectorHandleProps = {
  connector: Connector;
  pieceId: string;
  selected?: boolean;
  onPortClick: (connector: Connector) => void;
};

const getConnectorPositionStyle = (connector: Connector): { x: number; y: number } => {
  const { t } = connector;
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

const ConnectorHandle: React.FC<ConnectorHandleProps> = ({ connector, pieceId, selected = false, onPortClick }) => {
  const { x, y } = getConnectorPositionStyle(connector);
  const kit = useKit() as Kit | undefined;
  const selectedPortGuid = useContext(SelectedConnectorPortContext);
  const connectorPortGuid = getConnectorPortGuid(connector);
  const tone = getPortTone(connectorPortGuid, kit?.ports ?? []);
  const compatibilityState = getPortCompatibilityState(connectorPortGuid, selectedPortGuid, kit?.ports ?? []);
  const [hoverPort] = useDesignAppHoverPort();

  const isHovered = useDesignAppIsPortHovered(undefined, pieceId, connector.guid ?? "");

  const onClick = (event: React.MouseEvent) => {
    event.stopPropagation();
    onPortClick(connector);
  };

  return (
    <Handle
      id={connector.guid ?? ""}
      type="source"
      className="left-1/2 top-0 cursor-selectable"
      style={{
        left: x + ICON_WIDTH / 2,
        top: y,
        backgroundColor:
          selected
            ? "var(--active-base)"
            : isHovered
              ? "var(--hover-base)"
              : compatibilityState === "compatible"
                ? tone.surfaceStrong
                : compatibilityState === "incompatible"
                  ? "hsla(0 72% 52% / 0.32)"
                  : tone.base,
        border:
          selected || isHovered
            ? "2px solid var(--border-element-color)"
            : compatibilityState === "compatible"
              ? "1px solid hsl(141 57% 40%)"
              : compatibilityState === "incompatible"
                ? "1px solid hsl(0 74% 44%)"
                : `1px solid ${tone.border}`,
        zIndex: selected || isHovered ? 20 : 10,
      }}
      position={Position.Top}
      role="button"
      onClick={onClick}
      onPointerEnter={() => {
        if (connector.guid && hoverPort) hoverPort(pieceId, connector.guid);
      }}
      onPointerLeave={() => { }}
    />
  );
};

let sharedCommandsRef: ReturnType<typeof useDesignAppCommands> | null = null;

const pieceNodeAreEqual = (prevProps: NodeProps<PieceNode>, nextProps: NodeProps<PieceNode>) => {
  if (prevProps.id !== nextProps.id) return false;
  const prevData = prevProps.data as PieceNodeProps;
  const nextData = nextProps.data as PieceNodeProps;
  if (prevData.piece.guid !== nextData.piece.guid) return false;
  if (prevData.piece !== nextData.piece) {
    if (prevData.piece.type?.guid !== nextData.piece.type?.guid) return false;
    if (prevData.piece.design?.guid !== nextData.piece.design?.guid) return false;
    if (prevData.piece.description !== nextData.piece.description) return false;
    if (prevData.piece.isHidden !== nextData.piece.isHidden) return false;
  }
  if (prevData.type.guid !== nextData.type.guid) return false;
  if (prevData.type.name !== nextData.type.name) return false;
  return true;
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data }) => {
  const {
    piece,
    piece: { guid, attributes },
    type,
  } = data as PieceNodeProps & { diffStatus: DiffStatus };
  const connectors = type.connectors;

  const renderData = usePieceRenderData(guid);
  const isSelected = renderData.isSelected;

  const selectedConnector = useContext(SelectedConnectorContext);

  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isDesignPiece = !!piece.design;

  const commands = sharedCommandsRef!;
  const { hoverClearTimeoutRef, currentHoveredPieceGuidRef, isPanningRef, isDraggingNodeRef } = useHoverIntent();

  const selectPiecePort = useCallback(
    (piece: Guid, connector: Guid) => {
      commands.selectPiecePort("semio.sketchpad.app.design.canvas.diagram.pieceNode", piece, connector);
    },
    [commands],
  );

  const deselectPiecePort = useCallback(() => {
    commands.deselectPiecePort("semio.sketchpad.app.design.canvas.diagram.pieceNode");
  }, [commands]);

  const addConnection = useCallback(
    (connection: SemioConnection) => {
      commands.addConnection(connection);
    },
    [commands],
  );

  const handleMouseEnter = useCallback(
    (event: React.PointerEvent) => {
      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
        hoverClearTimeoutRef.current = null;
      }
      if (isPanningRef.current || isDraggingNodeRef.current || event.buttons !== 0) return;
      if (currentHoveredPieceGuidRef.current !== guid) {
        currentHoveredPieceGuidRef.current = guid;
        commands.hoverPiece("semio.sketchpad.app.design.canvas.diagram.pieceNode.handleMouseEnter", guid);
      }
    },
    [guid, commands, hoverClearTimeoutRef, isPanningRef, isDraggingNodeRef, currentHoveredPieceGuidRef],
  );

  const handleMouseLeave = useCallback(
    (event: React.PointerEvent) => {
      if (isPanningRef.current || isDraggingNodeRef.current || event.buttons !== 0) return;
      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
      }
      const pieceGuidAtLeave = guid;
      hoverClearTimeoutRef.current = setTimeout(() => {
        if (currentHoveredPieceGuidRef.current === pieceGuidAtLeave) {
          commands.clearHover("semio.sketchpad.app.design.canvas.diagram.pieceNode.handleMouseLeave");
          currentHoveredPieceGuidRef.current = null;
        }
        hoverClearTimeoutRef.current = null;
      }, 50);
    },
    [guid, commands, hoverClearTimeoutRef, isPanningRef, isDraggingNodeRef, currentHoveredPieceGuidRef],
  );

  return (
    <PieceNodeInner
      id={id}
      piece={piece}
      type={type}
      connectors={connectors}
      isSelected={isSelected}
      diff={diff}
      isDesignPiece={isDesignPiece}
      selectedConnector={selectedConnector}
      selectPiecePort={selectPiecePort}
      deselectPiecePort={deselectPiecePort}
      addConnection={addConnection}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    />
  );
}, pieceNodeAreEqual);

const SelectedConnectorContext = createContext<DesignAppSelection["connector"] | undefined>(undefined);
const SelectedConnectorPortContext = createContext<string | undefined>(undefined);

type PieceNodeInnerProps = {
  id: string;
  piece: Piece;
  type: Type;
  connectors: Connector[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  isDesignPiece: boolean;
  selectedConnector: DesignAppSelection["connector"] | undefined;
  selectPiecePort: (piece: Guid, connector: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: (event: React.PointerEvent) => void;
  onMouseLeave: (event: React.PointerEvent) => void;
};

const PieceNodeInner: React.FC<PieceNodeInnerProps> = ({ id, piece, type, connectors, isSelected, diff, isDesignPiece, selectedConnector, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const renderData = usePieceRenderData(piece.guid);
  const { fill, stroke, opacity: colorOpacity, isHovered } = renderData;

  const diffedPiece = piece;

  const hasCenterDiff = diff === DiffStatus.Modified && piece.center && diffedPiece.center && (piece.center.u !== diffedPiece.center.u || piece.center.v !== diffedPiece.center.v);

  const typeName = type.name || "";
  const displayVariant = typeName || piece.guid || "??";
  const initials = displayVariant.substring(0, 2).toUpperCase();
  const backgroundColor = fill === "transparent" ? undefined : fill;
  const showHoverBackground = fill === "var(--hover-base)";
  const textColor = isSelected ? "var(--active-foreground)" : backgroundColor && !showHoverBackground ? "var(--background)" : "var(--foreground)";
  const avatarTitle = typeName || piece.guid;
  const ringClass = isSelected ? "ring-1 ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-[color:var(--hover-base)]" : "";
  const fallbackStyle = backgroundColor ? { backgroundColor, color: textColor } : { color: textColor };

  const onPortClick = (connector: Connector) => {
    const currentSelectedConnector = selectedConnector;

    if (!connector.guid || !piece.guid) {
      console.error("[ORIGIN] Connector or piece guid is undefined", { connectorGuid: connector.guid, pieceGuid: piece.guid });
      return;
    }

    if (currentSelectedConnector && (currentSelectedConnector.piece !== piece.guid || currentSelectedConnector.connector !== connector.guid)) {
      if (!currentSelectedConnector.piece || !currentSelectedConnector.connector) {
        console.error("[ORIGIN] Selected connector has undefined piece or connector guid", {
          selectedPiece: currentSelectedConnector.piece,
          selectedConnector: currentSelectedConnector.connector,
        });
        return;
      }

      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          piece: { guid: currentSelectedConnector.piece },
          connector: { guid: currentSelectedConnector.connector },
        },
        connected: { piece: { guid: piece.guid }, connector: { guid: connector.guid } },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedConnector && currentSelectedConnector.piece === piece.guid && currentSelectedConnector.connector === connector.guid) {
      deselectPiecePort();
    } else {
      selectPiecePort(piece.guid, connector.guid);
    }
  };

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

      <Avatar role="button" title={avatarTitle} className={`w-full h-full border-element ${ringClass}`} style={{ borderColor: stroke, borderWidth: isDesignPiece ? 4 : undefined }}>
        <AvatarFallback className="select-none text-xs font-bold" style={fallbackStyle}>
          {initials}
        </AvatarFallback>
      </Avatar>
      {diffedPiece.plane && (
        <svg width={ICON_WIDTH} height={ICON_WIDTH} style={{ position: "absolute", top: 0, left: 0, pointerEvents: "none" }}>
          <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 6} className="stroke-[var(--foreground)] stroke-2 fill-transparent" />
        </svg>
      )}
      {connectors?.map((connector: Connector, connectorIndex: number) => (
        <ConnectorHandle
          key={`${id}-port-${connectorIndex}-${connector.guid}`}
          connector={connector}
          pieceId={piece.guid}
          selected={selectedConnector?.piece === piece.guid && selectedConnector?.connector === connector.guid}
          onPortClick={onPortClick}
        />
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
  const [addConnectionAction] = useDesignAppAddConnection();
  const isSelected = useDesignAppIsPieceSelected(undefined, guid);
  const selectedConnector = useDesignAppSelectedConnector();
  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;

  const [selectPiecePortAction] = useDesignAppSelectPiecePort();
  const [deselectPiecePortAction] = useDesignAppDeselectPiecePort();
  const [hoverPiece] = useDesignAppHoverPiece();
  const [clearHover] = useDesignAppClearHover();
  const { hoverClearTimeoutRef, currentHoveredPieceGuidRef, isPanningRef, isDraggingNodeRef } = useHoverIntent();

  const selectPiecePort = useCallback(
    (piece: Guid, connector: Guid) => {
      if (selectPiecePortAction) selectPiecePortAction(piece, connector);
    },
    [selectPiecePortAction],
  );

  const deselectPiecePort = useCallback(() => {
    if (deselectPiecePortAction) deselectPiecePortAction();
  }, [deselectPiecePortAction]);

  const addConnection = useCallback(
    (connection: SemioConnection) => {
      addConnectionAction?.(connection);
    },
    [addConnectionAction],
  );

  const handleMouseEnter = useCallback(
    (event: React.PointerEvent) => {
      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
        hoverClearTimeoutRef.current = null;
      }
      if (isPanningRef.current || isDraggingNodeRef.current || event.buttons !== 0) return;
      if (currentHoveredPieceGuidRef.current !== guid) {
        currentHoveredPieceGuidRef.current = guid;
        if (hoverPiece) hoverPiece(guid);
      }
    },
    [guid, hoverPiece, hoverClearTimeoutRef, isPanningRef, isDraggingNodeRef, currentHoveredPieceGuidRef],
  );

  const handleMouseLeave = useCallback(
    (event: React.PointerEvent) => {
      if (isPanningRef.current || isDraggingNodeRef.current || event.buttons !== 0) return;
      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
      }

      const pieceGuidAtLeave = guid;
      hoverClearTimeoutRef.current = setTimeout(() => {
        if (currentHoveredPieceGuidRef.current === pieceGuidAtLeave) {
          if (clearHover) clearHover();
          currentHoveredPieceGuidRef.current = null;
        }
        hoverClearTimeoutRef.current = null;
      }, 50);
    },
    [guid, clearHover, hoverClearTimeoutRef, isPanningRef, isDraggingNodeRef, currentHoveredPieceGuidRef],
  );

  const connectors: Connector[] = externalConnections.map((SemioConnection, connectorIndex) => {
    const connectedIsDesignPiece = SemioConnection.connected.piece.guid === piece.guid || SemioConnection.connected.designPiece?.guid === piece.guid;
    const connectingIsDesignPiece = SemioConnection.connecting.piece.guid === piece.guid || SemioConnection.connecting.designPiece?.guid === piece.guid;

    const designSide = connectedIsDesignPiece ? SemioConnection.connected : SemioConnection.connecting;
    const originalSide = connectedIsDesignPiece ? SemioConnection.connecting : SemioConnection.connected;

    const totalConnectors = externalConnections.length;
    const t = connectorIndex / totalConnectors;

    const angle = t * 2 * Math.PI;
    const radius = 0.5;

    const connectorX = radius * Math.sin(angle);
    const connectorY = radius * Math.cos(angle);
    const connectorZ = 0;

    const directionX = Math.sin(angle);
    const directionY = Math.cos(angle);
    const directionZ = 0;

    return {
      guid: `connector-${connectorIndex}`,
      description: `Connector for SemioConnection to ${originalSide.piece.guid}:${originalSide.connector?.guid ?? ""}`,
      port: { guid: "default" },
      mandatory: false,
      t: t,
      point: { x: connectorX, y: connectorY, z: connectorZ },
      direction: { x: directionX, y: directionY, z: directionZ },
      attributes: [
        {
          guid: crypto.randomUUID(),
          key: "semio.originalPieceId",
          value: designSide.piece.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.originalConnectorId",
          value: designSide.connector?.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalPieceId",
          value: originalSide.piece.guid || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalConnectorId",
          value: originalSide.connector?.guid || "",
        },
      ],
    };
  });

  return (
    <PieceScopeProvider guid={guid}>
      <DesignNodeInner
        id={id}
        piece={piece}
        connectors={connectors}
        isSelected={isSelected}
        diff={diff}
        selectedConnector={selectedConnector}
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
  connectors: Connector[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  selectedConnector: DesignAppSelection["connector"] | undefined;
  selectPiecePort: (piece: Guid, connector: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: (event: React.PointerEvent) => void;
  onMouseLeave: (event: React.PointerEvent) => void;
};

const DesignNodeInner: React.FC<DesignNodeInnerProps> = ({ id, piece, connectors, isSelected, diff, selectedConnector, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const isHovered = useIsPieceHovered();

  const onPortClick = (connector: Connector) => {
    const currentSelectedConnector = selectedConnector;

    if (!connector.guid || !piece.guid) {
      console.error("[ORIGIN] Connector or piece guid is undefined in DesignNode", { connectorGuid: connector.guid, pieceGuid: piece.guid });
      return;
    }

    if (currentSelectedConnector && (currentSelectedConnector.piece !== piece.guid || currentSelectedConnector.connector !== connector.guid)) {
      if (!currentSelectedConnector.piece || !currentSelectedConnector.connector) {
        console.error("[ORIGIN] Selected connector has undefined piece or connector guid in DesignNode", {
          selectedPiece: currentSelectedConnector.piece,
          selectedConnector: currentSelectedConnector.connector,
        });
        return;
      }

      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          piece: { guid: currentSelectedConnector.piece },
          connector: { guid: currentSelectedConnector.connector },
        },
        connected: { piece: { guid: piece.guid }, connector: { guid: connector.guid } },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedConnector && currentSelectedConnector.piece === piece.guid && currentSelectedConnector.connector === connector.guid) {
      deselectPiecePort();
    } else {
      selectPiecePort(piece.guid, connector.guid);
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
      {connectors?.map((connector: Connector, connectorIndex: number) => (
        <ConnectorHandle
          key={`${id}-port-${connectorIndex}-${connector.guid}`}
          connector={connector}
          pieceId={piece.guid}
          selected={selectedConnector?.piece === piece.guid && selectedConnector?.connector === connector.guid}
          onPortClick={onPortClick}
        />
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
  const [hoverConnection] = useDesignAppHoverConnection();
  const [clearHover] = useDesignAppClearHover();
  const { isPanningRef, isDraggingNodeRef } = useHoverIntent();
  const isHovered = useIsConnectionHovered();
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
          if (isPanningRef.current || isDraggingNodeRef.current || e.buttons !== 0) return;
          if (connectionGuid && hoverConnection) hoverConnection(connectionGuid);
        }}
        onPointerLeave={(e) => {
          if (isPanningRef.current || isDraggingNodeRef.current || e.buttons !== 0) return;
          if (clearHover) clearHover();
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

// MUST render a circle at the given position with accent color when selected.
// Custom minimap node component rendering a colored circle.
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

const pieceToNode = (piece: Piece, type: Type, center: Coord, index: number, selected: boolean = false): PieceNode => ({
  type: "piece",
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.u * ICON_WIDTH || 0,
    y: -center.v * ICON_WIDTH || 0,
  },
  selected,
  draggable: true,
  data: { piece, type },
  className: "",
});

const designToNode = (piece: Piece, externalConnections: SemioConnection[], center: Coord, index: number, selected: boolean = false): DesignNode => ({
  type: "design",
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.u * ICON_WIDTH || 0,
    y: -center.v * ICON_WIDTH || 0,
  },
  selected,
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
  let sourcePortId = SemioConnection.connecting.connector ?? "undefined";
  let targetConnectorId = SemioConnection.connected.connector ?? "undefined";

  if (SemioConnection.connecting.designPiece && allConnections) {
    const designPieceId = SemioConnection.connecting.designPiece;
    const designPieceGuid = designPieceId.guid;
    sourcePieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece?.guid === designPieceGuid;
      const connectingToDesign = conn.connecting.designPiece?.guid === designPieceGuid;
      return connectedToDesign || connectingToDesign;
    });

    const connectorIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece.guid === SemioConnection.connected.piece.guid &&
        conn.connecting.piece.guid === SemioConnection.connecting.piece.guid &&
        conn.connected.connector?.guid === SemioConnection.connected.connector?.guid &&
        conn.connecting.connector?.guid === SemioConnection.connecting.connector?.guid,
    );
    sourcePortId = connectorIndex >= 0 ? { guid: `connector-${connectorIndex}` } : { guid: "connector-0" };
  }

  if (SemioConnection.connected.designPiece && allConnections) {
    const designPieceId = SemioConnection.connected.designPiece;
    const designPieceGuid = designPieceId.guid;
    targetPieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece?.guid === designPieceGuid;
      const connectingToDesign = conn.connecting.designPiece?.guid === designPieceGuid;
      return connectedToDesign || connectingToDesign;
    });

    const connectorIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece.guid === SemioConnection.connected.piece.guid &&
        conn.connecting.piece.guid === SemioConnection.connecting.piece.guid &&
        conn.connected.connector?.guid === SemioConnection.connected.connector?.guid &&
        conn.connecting.connector?.guid === SemioConnection.connecting.connector?.guid,
    );
    targetConnectorId = connectorIndex >= 0 ? { guid: `connector-${connectorIndex}` } : { guid: "connector-0" };
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
    targetHandle: typeof targetConnectorId === "string" ? targetConnectorId : targetConnectorId.guid,
    data: { SemioConnection, isParentConnection },
    selected,
  };
};

const designToNodesAndEdges = (design: Design, metadata: Map<string, PieceMetadata>, kit: any, selection?: DesignAppSelection) => {
  if (!design) return null;

  const selectedPieces = new Set(selection?.pieces ?? []);
  const selectedConnections = new Set(selection?.connections ?? []);

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
        const selected = selectedPieces.has(piece.guid);

        if (piece.design) {
          const design = kit.designs?.find((d: Design) => d.guid === piece.design?.guid);
          if (!design) {
            const fallbackType: Type = {
              guid: `fallback-${piece.design}`,
              name: `Unknown-${piece.design}`,
              unit: "m",
              description: `Missing design: ${piece.design}`,
              connectors: [],
              models: [],
            };
            return pieceToNode(piece, fallbackType, center, i, selected);
          }
          const designAsType: Type = {
            guid: design.guid,
            name: design.name,
            unit: design.unit || "m",
            description: design.description,
            connectors: [],
            models: [],
          };
          return pieceToNode(piece, designAsType, center, i, selected);
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
            connectors: [],
            models: [],
          };
          return pieceToNode(piece, fallbackType, center, i, selected);
        }
        return pieceToNode(piece, type, center, i, selected);
      })
      .filter((node): node is PieceNode => node !== null) ?? [];

  const includedDesigns = getIncludedDesigns(design);

  const designNodes = includedDesigns.map((includedDesign, i) => {
    const selected = selectedPieces.has(includedDesign.guid);
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

      return designToNode(designPiece, includedDesign.externalConnections || [], calculatedCenter, design.pieces!.length + i, selected);
    } else {
      const displayCenter = includedDesign.center || { u: 0, v: 0 };

      const designPiece: Piece = {
        guid: includedDesign.guid,
        type: { guid: includedDesign.designGuid },
        center: displayCenter,
        plane: includedDesign.plane,
        description: `Fixed design: ${includedDesign.designGuid}`,
      };

      return designToNode(designPiece, [], displayCenter, design.pieces!.length + i, selected);
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

  const connectionEdges =
    design.connections?.map((SemioConnection, connectionIndex) => {
      const selected = selectedConnections.has(SemioConnection.guid);
      return connectionToEdge(SemioConnection, selected, false, pieceIndexMap, connectionIndex, design.pieces, design.connections);
    }) ?? [];
  return { nodes: [...pieceNodes, ...designNodes], edges: connectionEdges };
};

interface DesignDiagramProps {
  reactFlowInstanceRef: React.MutableRefObject<ReactFlowInstance | null>;
}

const DesignDiagram: FC<DesignDiagramProps> = ({ reactFlowInstanceRef }) => {
  const [transaction] = useDesignAppTransaction();
  const [addPiece] = useDesignAppAddPiece();
  const [updatePieces] = useDesignAppUpdatePieces();
  const [addConnection] = useDesignAppAddConnection();
  const [addConnections] = useDesignAppAddConnections();
  const [updateConnections] = useDesignAppUpdateConnections();
  const [clusterPieces] = useDesignAppClusterPieces();
  const [expandDesign] = useDesignAppExpandDesign();

  const [deselectAll] = useDesignAppDeselectAll();
  const [selectPiece] = useDesignAppSelectPiece();
  const [addPieceToSelection] = useDesignAppAddPieceToSelection();
  const [removePieceFromSelection] = useDesignAppRemovePieceFromSelection();
  const [selectConnection] = useDesignAppSelectConnection();
  const [addConnectionToSelection] = useDesignAppAddConnectionToSelection();
  const [removeConnectionFromSelection] = useDesignAppRemoveConnectionFromSelection();
  const [toggleDiagramFullscreen] = useDesignAppToggleDiagramFullscreen();
  const [, setDiagramCenter] = useDesignAppDiagramCenter();
  const [, setDiagramScale] = useDesignAppDiagramScale();
  const [focusPiece] = useDesignAppFocusPiece();
  const [hoverPiece] = useDesignAppHoverPiece();
  const [clearHover] = useDesignAppClearHover();
  const { hoverClearTimeoutRef, currentHoveredPieceGuidRef, isPanningRef, isDraggingNodeRef } = useHoverIntent();

  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitTypes = useKitTypes();
  const kitDesigns = useKitDesigns();
  const kit = useKit() as Kit;
  const [activeTool] = useDesignAppActiveTool();

  const [selection, setSelection] = useDesignAppSelection();
  const selectionRef = useRef(selection);
  selectionRef.current = selection;
  const [fullscreenWindow] = useDesignAppFullscreen();
  const [others] = useDesignAppOthers();
  const [savedDiagramCenter] = useDesignAppDiagramCenter();
  const [savedDiagramScale] = useDesignAppDiagramScale();
  const panelVisibility = useAppPanelVisibility();

  const design = useDesign() as Design | null;
  const metadata = usePiecesMetadataMap();

  const commands = useDesignAppCommands();
  sharedCommandsRef = commands;

  const { transitivelyHoveredPieces } = useContext(HoverPiecesContext);
  const { statusMap: transactionStatusMap } = useContext(TransactionPiecesContext);

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

  const selectedConnector = selection?.connector;
  const selectedConnectorPortGuid = useMemo(() => {
    if (!selectedConnector?.piece || !selectedConnector.connector || !design) return undefined;
    const selectedPiece = design.pieces?.find((piece) => piece.guid === selectedConnector.piece);
    if (!selectedPiece) return undefined;
    if (selectedPiece.design?.guid) return "default";
    const selectedType = selectedPiece.type?.guid ? kitTypes?.find((type) => type.guid === selectedPiece.type?.guid) : undefined;
    const selectedTypeConnector = selectedType?.connectors?.find((connector) => connector.guid === selectedConnector.connector);
    return selectedTypeConnector?.port?.guid;
  }, [selectedConnector, design, kitTypes]);

  const { baseNodes, edges } = useMemo(() => {
    if (!design) return { baseNodes: [], edges: [] };
    const minimalKit = { types: kitTypes, designs: kitDesigns } as Kit;
    const result = designToNodesAndEdges(design, metadata, minimalKit, selection) ?? { nodes: [], edges: [] };
    return { baseNodes: result.nodes, edges: result.edges };
  }, [design, metadata, kitTypes, kitDesigns, selection]);

  const [nodes, setNodes] = useState<typeof baseNodes>(baseNodes);

  useEffect(() => {
    setNodes(baseNodes);
  }, [baseNodes]);

  const onNodesChangeReactFlow = useCallback(
    (changes: any[]) => {
      if (isDraggingNodeRef.current || isPanningRef.current) return;
      if (changes.length === 0) return;
      setNodes((nds) => applyNodeChanges(changes, nds) as typeof nds);
    },
    [isDraggingNodeRef, isPanningRef],
  );

  const isLassoingRef = useRef(false);
  const baseSelectionRef = useRef<{ pieces?: string[]; connections?: string[] } | null>(null);

  const onSelectionStart = useCallback(() => {
    isLassoingRef.current = true;
    baseSelectionRef.current = selectionRef.current;
  }, []);

  const onSelectionEnd = useCallback(() => {
    isLassoingRef.current = false;
    baseSelectionRef.current = null;
  }, []);

  const onSelectionChange = useCallback(
    ({ nodes, edges }: { nodes: Array<Node>; edges: Array<Edge> }) => {
      if (isDraggingNodeRef.current || isPanningRef.current) return;

      const selectedPieceGuids = nodes.filter((n) => n.id.startsWith("piece-")).map((n) => getPieceIdFromNode(n as DiagramNode));

      const selectedConnectionGuids = edges
        .filter((e) => e.type === "SemioConnection" || e.id.startsWith("connection-") || (e as any).data?.SemioConnection)
        .map((e) => (e as any).data?.SemioConnection?.guid || e.id.split("-").pop())
        .filter((guid): guid is string => !!guid);

      let finalPieceGuids = selectedPieceGuids;
      let finalConnectionGuids = selectedConnectionGuids;

      if (isLassoingRef.current && (activeTool === ToolKind.SELECTION_ADDITIVE || activeTool === ToolKind.SELECTION_SUBTRACTIVE || activeTool === ToolKind.SELECTION_INTERSECT)) {
        const base = baseSelectionRef.current || { pieces: [], connections: [] };
        const basePieces = base.pieces || [];
        const baseConnections = base.connections || [];

        if (activeTool === ToolKind.SELECTION_ADDITIVE) {
          finalPieceGuids = [...new Set([...basePieces, ...selectedPieceGuids])];
          finalConnectionGuids = [...new Set([...baseConnections, ...selectedConnectionGuids])];
        } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE) {
          finalPieceGuids = basePieces.filter((id) => !selectedPieceGuids.includes(id));
          finalConnectionGuids = baseConnections.filter((id) => !selectedConnectionGuids.includes(id));
        } else if (activeTool === ToolKind.SELECTION_INTERSECT) {
          finalPieceGuids = basePieces.filter((id) => selectedPieceGuids.includes(id));
          finalConnectionGuids = baseConnections.filter((id) => selectedConnectionGuids.includes(id));
        }
      }

      const currentSelection = selectionRef.current || {};
      const currentPieces = currentSelection.pieces || [];
      const currentConnections = currentSelection.connections || [];

      const piecesChanged = finalPieceGuids.length !== currentPieces.length || finalPieceGuids.some((id) => !currentPieces.includes(id));
      const connectionsChanged =
        finalConnectionGuids.length !== currentConnections.length || finalConnectionGuids.some((id) => !currentConnections.includes(id));

      if (piecesChanged || connectionsChanged) {
        if (setSelection) {
          setSelection({
            ...currentSelection,
            pieces: finalPieceGuids,
            connections: finalConnectionGuids,
          });
        }
      }
    },
    [setSelection, isDraggingNodeRef, isPanningRef, activeTool],
  );

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
      if (focusPiece) focusPiece(itemId);
    };
    focusContext.setOnFocusItem(handleFocus);
    return () => {
      if (focusContext) focusContext.setOnFocusItem(undefined);
    };
  }, [focusContext, focusPiece, nodes]);

  if (!design) return null;

  const dragPositionRef = useRef<{ x: number; y: number } | null>(null);
  const pendingPieceUpdatesRef = useRef<Array<{ id: string; diff: any }>>([]);
  const [helperLines, setHelperLines] = useState<HelperLine[]>([]);
  const fullscreen = fullscreenWindow === DesignAppFullscreenWindow.Diagram;
  const viewportRestoredRef = useRef(false);
  const isUpdatingViewportRef = useRef(false);
  const dropZoneRef = useRef<HTMLDivElement | null>(null);
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const handleDiagramPointerDown = useCallback(
    (e: PointerEvent) => {
      if (e.button === 1 || e.button === 2) {
        isPanningRef.current = true;
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
    [diagramId, isPanningRef],
  );

  const handleDiagramPointerUp = useCallback(() => {
    const nodesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__nodes`);
    if (nodesContainer) {
      (nodesContainer as HTMLElement).style.pointerEvents = "";
    }
    const edgesContainer = document.querySelector(`[data-diagram-id="${diagramId}"] .react-flow__edges`);
    if (edgesContainer) {
      (edgesContainer as HTMLElement).style.pointerEvents = "";
    }
    isPanningRef.current = false;
  }, [diagramId, isPanningRef]);

  const setDropZoneRef = useCallback(
    (node: HTMLDivElement | null) => {
      if (dropZoneRef.current) {
        dropZoneRef.current.removeEventListener("pointerdown", handleDiagramPointerDown as any);
        dropZoneRef.current.removeEventListener("pointerup", handleDiagramPointerUp as any);
        dropZoneRef.current.removeEventListener("pointerleave", handleDiagramPointerUp as any);
      }
      if (node) {
        node.setAttribute("data-drop-zone", "diagram");
        node.setAttribute("data-drop-zone-id", diagramId);
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
        transaction?.start();
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, type: { guid: droppedType.guid }, center: { u: centerU, v: centerV } };
        addPiece?.(piece);
        transaction?.finalize();
      } else if (dragData.type === "design" && dragData.designGuid) {
        const droppedDesign = kitDesigns?.find((d) => d.guid === dragData.designGuid);
        if (!droppedDesign) return;
        transaction?.start();
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          id_: pieceGuid,
          design: { guid: droppedDesign.guid },
          center: { u: centerU, v: centerV },
        };
        addPiece?.(piece);
        transaction?.finalize();
      }
      setActiveDraggedType(null);
      setActiveDraggedDesign(null);
    },
    [reactFlowInstanceRef, kitTypes, kitDesigns, transaction, addPiece, setActiveDraggedType, setActiveDraggedDesign, diagramId],
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
        transaction?.abort();
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
  }, [transaction]);

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

  const onMoveStart = useCallback(() => {
    isPanningRef.current = true;
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      diagramElement.setAttribute("data-panning", "true");
    }
  }, [diagramId, isPanningRef]);

  const pendingMoveEndRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onMoveEnd = useCallback(() => {
    isPanningRef.current = false;
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      diagramElement.removeAttribute("data-panning");
    }

    if (isUpdatingViewportRef.current || !reactFlowInstanceRef.current) return;
    if (pendingMoveEndRef.current) {
      clearTimeout(pendingMoveEndRef.current);
    }
    pendingMoveEndRef.current = setTimeout(() => {
      if (!reactFlowInstanceRef.current) return;
      const viewport = reactFlowInstanceRef.current.getViewport();
      if (setDiagramCenter) setDiagramCenter({ u: viewport.x / ICON_WIDTH, v: -viewport.y / ICON_WIDTH });
      pendingMoveEndRef.current = null;
    }, 1000);
  }, [reactFlowInstanceRef, setDiagramCenter, diagramId, isPanningRef]);

  const centerViewport = useCallback(() => {
    if (!reactFlowInstanceRef.current) return;
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      const rect = diagramElement.getBoundingClientRect();
      if (rect.width > 0 && rect.height > 0) {
        const centerX = rect.width / 2;
        const centerY = rect.height / 2;
        reactFlowInstanceRef.current.setViewport({ x: centerX, y: centerY, zoom: 1 });
        if (setDiagramCenter) setDiagramCenter({ u: centerX / ICON_WIDTH, v: -centerY / ICON_WIDTH });
      } else {
        console.warn("[Diagram] Element has no dimensions yet, retrying...");
        setTimeout(() => centerViewport(), 100);
      }
    } else {
      console.warn("[Diagram] Element not found, retrying...");
      setTimeout(() => centerViewport(), 100);
    }
  }, [reactFlowInstanceRef, diagramId, setDiagramCenter]);

  const activeToolRef = useRef(activeTool);
  activeToolRef.current = activeTool;

  const onNodeClick = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => { },
    [],
  );

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
    (e: React.MouseEvent, edge: DiagramEdge) => { },
    [],
  );

  const onPaneClick = useCallback(
    (e: React.MouseEvent) => { },
    [],
  );

  const onDoubleClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      if (toggleDiagramFullscreen) toggleDiagramFullscreen();
    },
    [toggleDiagramFullscreen],
  );

  const onCluster = useCallback(
    (clusterPieceIds: string[]) => {
      if (!clusterPieces || clusterPieceIds.length === 0) return;
      transaction?.start();
      clusterPieces(clusterPieceIds);
      transaction?.finalize();
    },
    [clusterPieces, transaction],
  );

  const onExpand = useCallback(
    (target: string) => {
      if (!expandDesign || !target) return;
      transaction?.start();
      expandDesign(target);
      transaction?.finalize();
    },
    [expandDesign, transaction],
  );

  const onNodeMouseEnter = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      if (isPanningRef.current || isDraggingNodeRef.current || e.buttons !== 0) return;

      const pieceId = getPieceIdFromNode(node);

      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
        hoverClearTimeoutRef.current = null;
      }

      if (currentHoveredPieceGuidRef.current !== pieceId) {
        currentHoveredPieceGuidRef.current = pieceId;
        if (hoverPiece) hoverPiece(pieceId);
      }
    },
    [hoverPiece, isPanningRef, isDraggingNodeRef, hoverClearTimeoutRef, currentHoveredPieceGuidRef],
  );

  const onNodeMouseLeave = useCallback(
    (e: React.MouseEvent, node: DiagramNode) => {
      if (isPanningRef.current || isDraggingNodeRef.current || e.buttons !== 0) return;

      const pieceId = getPieceIdFromNode(node);

      if (hoverClearTimeoutRef.current) {
        clearTimeout(hoverClearTimeoutRef.current);
      }

      const pieceGuidAtLeave = pieceId;
      hoverClearTimeoutRef.current = setTimeout(() => {
        if (currentHoveredPieceGuidRef.current === pieceGuidAtLeave) {
          if (clearHover) clearHover();
          currentHoveredPieceGuidRef.current = null;
        }
        hoverClearTimeoutRef.current = null;
      }, 50);
    },
    [clearHover, isPanningRef, isDraggingNodeRef, hoverClearTimeoutRef, currentHoveredPieceGuidRef],
  );

  const pendingSelectionRef = useRef<{ pieceId: string; action: "select" | "add" | "remove" } | null>(null);

  const onNodeDragStart = useCallback(
    (event: any, node: Node) => {
      const currentSelectedIds = selectionRef.current?.pieces ?? [];
      const pieceId = getPieceIdFromNode(node as DiagramNode);
      const isNodeSelected = currentSelectedIds.includes(pieceId);
      const ctrlKey = event.ctrlKey || event.metaKey;
      const shiftKey = event.shiftKey;

      if (ctrlKey) {
        pendingSelectionRef.current = { pieceId, action: isNodeSelected ? "remove" : "add" };
      } else if (shiftKey) {
        pendingSelectionRef.current = { pieceId, action: isNodeSelected ? "select" : "add" };
      } else if (activeTool === ToolKind.SELECTION_ADDITIVE) {
        pendingSelectionRef.current = { pieceId, action: "add" };
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE) {
        pendingSelectionRef.current = { pieceId, action: "remove" };
      } else if (activeTool === ToolKind.SELECTION_INTERSECT) {
        pendingSelectionRef.current = { pieceId, action: "intersect" as any };
      } else if (!isNodeSelected) {
        pendingSelectionRef.current = { pieceId, action: "select" };
      } else {
        pendingSelectionRef.current = null;
      }

      dragPositionRef.current = { x: node.position.x, y: node.position.y };
      pendingPieceUpdatesRef.current = [];
      isDraggingRef.current = true;
      isDraggingNodeRef.current = true;
    },
    [activeTool, isDraggingNodeRef],
  );

  const isDraggingRef = useRef(false);

  const lastDragTimeRef = useRef<number>(0);
  const DRAG_THROTTLE_MS = 50;

  const onNodeDrag = useCallback(
    (event: any, node: DiagramNode) => {
      if (!isDraggingRef.current || !dragPositionRef.current || !reactFlowInstanceRef.current) {
        dragPositionRef.current = { x: node.position.x, y: node.position.y };
        return;
      }

      if (!event.altKey) {
        dragPositionRef.current = { x: node.position.x, y: node.position.y };
        return;
      }

      const now = Date.now();
      if (now - lastDragTimeRef.current < DRAG_THROTTLE_MS) {
        if (node.type === "design") {
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

        const type = (selectedNode as PieceNode).data.type;
        const fixedPieceId = metadata.get(piece.guid)?.fixedPieceId;
        let closestConnection: SemioConnection | null = null;
        let closestDistance = Number.MAX_VALUE;

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

          setHelperLines(currentHelperLines);
        } else {
          setHelperLines([]);
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
                connected: { guid: "", piece: { guid: selectedNode.data.piece.guid }, connector: { guid: "" } },
                connecting: { guid: "", piece: { guid: otherNode.data.piece.guid }, connector: { guid: "" } },
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
              const connector = findConnectorInType(type, handle.id!);
              if (!connector || !connector.guid) {
                console.error("[ORIGIN] onNodeDrag: connector or connector.guid is undefined", { connector, handleId: handle.id, type });
                continue;
              }
              for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
                if (!otherHandle.id) {
                  console.error("[ORIGIN] onNodeDrag: otherHandle.id is undefined", { otherHandle, otherNode });
                  continue;
                }
                const otherPort = findConnectorInType((otherNode as PieceNode).data.type, otherHandle.id!);
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
                if (haveSameFixedPiece || !arePortsCompatible(connector, otherPort) || (design && isPortInUse(design as Design, piece.guid, connector.guid)) || (design && isPortInUse(design as Design, otherNode.data.piece.guid, otherPort.guid)))
                  continue;
                const dx = selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x);
                const dy = selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y);
                const distance = Math.sqrt(dx * dx + dy * dy);
                if (distance < closestDistance && distance < MIN_DISTANCE) {
                  closestConnection = {
                    guid: crypto.randomUUID(),
                    connected: {
                      piece: { guid: otherNode.data.piece.guid },
                      connector: { guid: otherHandle.id! },
                    },
                    connecting: {
                      piece: { guid: selectedNode.data.piece.guid },
                      connector: { guid: handle.id! },
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

      if (addedConnections.length > 0) {
        addedConnections.forEach((conn) => addConnection?.(conn));
      }
      dragPositionRef.current = { x: draggedX, y: draggedY };
      pendingPieceUpdatesRef.current = updatedPieces;
    },
    [addConnection, design, reactFlowInstanceRef, nodes, metadata],
  );

  const onNodeDragStop = useCallback(
    (event: any, node: DiagramNode) => {
      const pendingSelection = pendingSelectionRef.current;
      if (pendingSelection) {
        const { pieceId, action } = pendingSelection;
        if (action === "select" && selectPiece) selectPiece(pieceId);
        else if (action === "add" && addPieceToSelection) addPieceToSelection(pieceId);
        else if (action === "remove" && removePieceFromSelection) removePieceFromSelection(pieceId);
        else if (action === "intersect") {
          if (selectionRef.current?.pieces?.includes(pieceId) && selectPiece) {
            selectPiece(pieceId);
          } else if (deselectAll) {
            deselectAll();
          }
        }
        pendingSelectionRef.current = null;
      }

      transaction?.start();

      const pendingUpdates = pendingPieceUpdatesRef.current;
      if (pendingUpdates && pendingUpdates.length > 0) {
        updatePieces?.(pendingUpdates);
      } else if (node && selectionRef.current?.pieces?.length) {
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
          updatePieces?.(updatedPieces);
        }
      }

      transaction?.finalize();

      isDraggingRef.current = false;
      isDraggingNodeRef.current = false;
      dragPositionRef.current = null;
      pendingPieceUpdatesRef.current = [];
    },
    [transaction, updatePieces, nodes, selectPiece, addPieceToSelection, removePieceFromSelection, isDraggingNodeRef, deselectAll],
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
          connector: { guid: params.sourceHandle },
        },
        connecting: {
          piece: targetPieceId,
          connector: { guid: params.targetHandle },
        },
        u: (sourceInternalNode.internals.positionAbsolute.x + sourceHandle.x - (targetInternalNode.internals.positionAbsolute.x + targetHandle.x)) / ICON_WIDTH,
        v: -((sourceInternalNode.internals.positionAbsolute.y + sourceHandle.y - (targetInternalNode.internals.positionAbsolute.y + targetHandle.y)) / ICON_WIDTH),
      };

      if (!design) return;
      if (((design as Design).connections ?? []).find((c: SemioConnection) => areSameConnection(c, newConnection))) return;
      addConnection?.(newConnection);
    },
    [addConnection, reactFlowInstanceRef, design],
  );

  return (
    <PieceRenderDataContext.Provider value={pieceRenderDataMap}>
      <SelectedConnectorContext.Provider value={selectedConnector}>
        <SelectedConnectorPortContext.Provider value={selectedConnectorPortGuid}>
          <div id="semio.sketchpad.app.design.canvas.diagram" data-diagram-id={diagramId} className="h-full w-full relative" ref={setDropZoneRef}>
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
              elementsSelectable={true}
              nodesFocusable={true}
              edgesFocusable={true}
              nodesDraggable={true}
              minZoom={0.1}
              defaultZoom={1}
              maxZoom={12}
              fitView={false}
              panOnDrag={[1, 2]}
              selectionOnDrag={true}
              zoomOnDoubleClick={false}
              onSelectionChange={onSelectionChange}
              onSelectionStart={onSelectionStart}
              onSelectionEnd={onSelectionEnd}
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
            <ClusterMenu nodes={nodes} edges={edges} onCluster={onCluster} />
            <ExpandMenu nodes={nodes} edges={edges} onExpand={onExpand} />
          </div>
        </SelectedConnectorPortContext.Provider>
      </SelectedConnectorContext.Provider>
    </PieceRenderDataContext.Provider>
  );
};

// #endregion 🔖Diagram

// #region 🔖Scene
// Scene MUST render the Three.js 3D scene view of design pieces with selection and hover highlighting.

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
const applyHighlightToLoadedScene = (scene: THREE.Object3D, highlightThreeColor: THREE.Color | null, plasterColor: THREE.Color, plasterEdgeColor: THREE.Color): void => {
  scene.traverse((child) => {
    if (child instanceof THREE.Mesh) {
      const materials = Array.isArray(child.material) ? child.material : [child.material];
      materials.forEach((material) => {
        if (material && "color" in material && material.color instanceof THREE.Color) {
          material.color.copy(highlightThreeColor ?? plasterColor);
        }
      });
    } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
      const material = (child as any).material;
      if (material && material.color instanceof THREE.Color) {
        material.color.copy(highlightThreeColor ?? plasterEdgeColor);
      }
    }
  });
};

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

const GLTFMesh: FC<{ url: string; highlightColor: string | null }> = ({ url, highlightColor }) => {
  const gltf = useGLTF(url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColor("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColor("--plaster-edge")), []);
  const highlightThreeColor = useMemo(() => (highlightColor ? new THREE.Color(highlightColor) : null), [highlightColor]);

  const clonedScene = useMemo(() => {
    const cloned = gltf.scene.clone();
    cloned.applyMatrix4(toSemioRotation());
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [gltf.scene, plasterColor, plasterEdgeColor]);
  useEffect(() => {
    applyHighlightToLoadedScene(clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor);
  }, [clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor]);
  return <primitive object={clonedScene} />;
};

const FBXMesh: FC<{ url: string; highlightColor: string | null }> = ({ url, highlightColor }) => {
  const scene = useFBX(url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColor("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColor("--plaster-edge")), []);
  const highlightThreeColor = useMemo(() => (highlightColor ? new THREE.Color(highlightColor) : null), [highlightColor]);

  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    cloned.applyMatrix4(toSemioRotation());
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [scene, plasterColor, plasterEdgeColor]);
  useEffect(() => {
    applyHighlightToLoadedScene(clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor);
  }, [clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor]);
  return <primitive object={clonedScene} />;
};

const OBJMesh: FC<{ url: string; highlightColor: string | null }> = ({ url, highlightColor }) => {
  const obj = useLoader(OBJLoader, url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColor("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColor("--plaster-edge")), []);
  const highlightThreeColor = useMemo(() => (highlightColor ? new THREE.Color(highlightColor) : null), [highlightColor]);

  const clonedScene = useMemo(() => {
    const cloned = obj.clone();
    cloned.applyMatrix4(toSemioRotation());
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [obj, plasterColor, plasterEdgeColor]);
  useEffect(() => {
    applyHighlightToLoadedScene(clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor);
  }, [clonedScene, highlightThreeColor, plasterColor, plasterEdgeColor]);
  return <primitive object={clonedScene} />;
};

const LoadedPieceMesh: FC<{ url: string; fileExtension: string; highlightColor: string | null }> = ({ url, fileExtension, highlightColor }) => {
  const ext = fileExtension.toLowerCase();
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} highlightColor={highlightColor} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} highlightColor={highlightColor} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} highlightColor={highlightColor} />;
  } else {
    return <GLTFMesh url={url} highlightColor={highlightColor} />;
  }
};

const PieceMesh: FC<{ highlightColor: string | null }> = ({ highlightColor }) => {
  const piece = usePiece() as Piece;
  const type = useType(undefined, typeof piece.type === "string" ? piece.type : piece.type?.guid) as Type | undefined;
  const files = useKitFiles();
  const kitStore = useKitStore() as KitStore;
  const [selectedModelTags] = useDesignAppSelectedModelTags();
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { modelUrl, fileExtension, fileGuid } = useMemo(() => {
    if (!type?.models || type.models.length === 0) {
      console.warn("[PieceMesh] No models available for type:", type?.guid, type?.name);
      return { modelUrl: null, fileExtension: "", fileGuid: null };
    }
    const tagsForType = selectedModelTags[type.guid] ?? [];
    let model: Model | undefined;
    if (tagsForType.length > 0) {
      model = selectBestModel(type.models, tagsForType);
    } else {
      const conceptGuids = type.concepts?.map((c) => c.guid) ?? [];
      if (conceptGuids.length > 0) {
        model = findModel(type.models, conceptGuids);
      } else {
        const defaultRep = type.models.find((r) => !r.tags || r.tags.length === 0);
        model = defaultRep ?? type.models[0];
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
    };
  }, [fileGuid, kitStore]);

  if (!blobUrl) {
    return null;
  }

  return (
    <Suspense fallback={null}>
      <LoadedPieceMesh url={blobUrl} fileExtension={fileExtension} highlightColor={highlightColor} />
    </Suspense>
  );
};

interface ModelPieceProps { }

const ModelPiece: FC<ModelPieceProps> = () => {
  const piece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const isSelected = useIsPieceSelected();
  const isHovered = useIsPieceTransitiveHovered();
  const status = usePieceStatus();
  const flatPlane = useFlatPiecePlane();

  const [selectPiece] = useDesignAppSelectPiece();
  const [removePieceFromSelection] = useDesignAppRemovePieceFromSelection();
  const [addPieceToSelection] = useDesignAppAddPieceToSelection();
  const [hoverPiece] = useDesignAppHoverPiece();
  const [clearHover] = useDesignAppClearHover();
  const [focusPiece] = useDesignAppFocusPiece();
  const { currentHoveredPieceGuidRef } = useHoverIntent();

  const { fill } = useDesignAppPieceColor(undefined, piece.guid);

  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const mutedForegroundColor = useMemo(() => getComputedColor("--muted-foreground"), []);
  const activeBaseColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = useMemo(() => getComputedColor("--hover-base"), []);
  const highlightColor = useMemo(() => (isSelected ? activeBaseColor : isHovered ? hoverBaseColor : null), [isSelected, isHovered, activeBaseColor, hoverBaseColor]);

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
        if (removePieceFromSelection) removePieceFromSelection(piece.guid);
      } else if (e?.shiftKey) {
        if (addPieceToSelection) addPieceToSelection(piece.guid);
      } else {
        if (selectPiece) selectPiece(piece.guid);
      }
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection, piece.guid],
  );

  const onDoubleClick = useCallback(
    (e?: ThreeEvent<MouseEvent>) => {
      e?.stopPropagation();
      if (focusPiece) focusPiece(piece.guid);
    },
    [focusPiece, piece.guid],
  );

  const handlePointerEnter = useCallback(() => {
    if (currentHoveredPieceGuidRef.current !== piece.guid) {
      currentHoveredPieceGuidRef.current = piece.guid;
      if (hoverPiece) hoverPiece(piece.guid);
    }
  }, [piece.guid, hoverPiece, currentHoveredPieceGuidRef]);

  const handlePointerLeave = useCallback(() => {
    if (currentHoveredPieceGuidRef.current === piece.guid) {
      currentHoveredPieceGuidRef.current = null;
      if (clearHover) clearHover();
    }
  }, [piece.guid, clearHover, currentHoveredPieceGuidRef]);

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

  const userData = useMemo(() => ({ id: piece.guid, pieceId: piece.guid }), [piece.guid]);

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
      userData={userData}
    />
  ) : (
    <group userData={userData} onClick={onSelect} onDoubleClick={onDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      <PieceMesh highlightColor={highlightColor} />
    </group>
  );

  const pieceMatrix = diffedMatrix || originalMatrix;

  return (
    <>
      {originalMeshContent}
      {pieceMatrix && (
        <group userData={userData} matrix={pieceMatrix} matrixAutoUpdate={false}>
          {diffedMeshContent}
        </group>
      )}
    </>
  );
};

const ModelDesign: FC = () => {
  const [transaction] = useDesignAppTransaction();
  const [updatePiece] = useDesignAppUpdatePiece();
  const [selection] = useDesignAppSelection();
  const [others] = useDesignAppOthers();
  const design = useDesign();
  const flatDesign = design as Design;

  const [selectPieces] = useDesignAppSelectPieces();

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const resolvePieceGuid = (object: THREE.Object3D | undefined): string | undefined => {
        let current: THREE.Object3D | null | undefined = object;
        while (current) {
          const pieceId = current.userData?.pieceId;
          if (typeof pieceId === "string" && pieceId.length > 0) return pieceId;
          const id = current.userData?.id;
          if (typeof id === "string" && id.length > 0) return id;
          current = current.parent;
        }
        return undefined;
      };
      const newSelectedPieceIds = Array.from(new Set(selected.map((item) => resolvePieceGuid(item)).filter((value): value is string => !!value)));
      const previousSelectedPieceIds = selection.pieces ?? [];
      const changed =
        newSelectedPieceIds.length !== previousSelectedPieceIds.length || newSelectedPieceIds.some((id) => !previousSelectedPieceIds.includes(id)) || previousSelectedPieceIds.some((id) => !newSelectedPieceIds.includes(id));
      if (changed) {
        if (selectPieces) selectPieces(newSelectedPieceIds);
      }
    },
    [selectPieces, selection.pieces],
  );

  type TransformableModel = { guid: string; plane: Plane | undefined; isTransformable: boolean; isSelected: boolean };
  const selectedModels = useMemo((): TransformableModel[] => {
    if (!selection.pieces || !flatDesign?.pieces) return [];

    return flatDesign.pieces
      .filter((piece) => selection.pieces?.includes(piece.guid))
      .map((piece) => ({
        guid: piece.guid,
        plane: piece.plane,
        isTransformable: !piece.isLocked && piece.plane !== undefined,
        isSelected: true,
      }));
  }, [selection.pieces, flatDesign?.pieces]);

  const handleMultiPlaneUpdate = useCallback(
    (updates: Array<{ modelGuid: string; newPlane: Plane }>) => {
      updates.forEach(({ modelGuid, newPlane }) => {
        updatePiece?.(modelGuid, { plane: newPlane });
      });
    },
    [updatePiece],
  );

  return (
    <>
      <Select box multiple onChange={onChange}>
        <group>
          {flatDesign?.pieces?.map((piece: Piece) => (
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
  const [transaction] = useDesignAppTransaction();
  const [addPiece] = useDesignAppAddPiece();
  const [deselectAll] = useDesignAppDeselectAll();
  const [toggleAccesslFullscreen] = useDesignAppToggleAccesslFullscreen();
  const [, setCamera] = useDesignAppCamera();
  const [clearFocus] = useDesignAppClearFocus();
  const [fullscreenValue] = useDesignAppFullscreen();
  const fullscreen = fullscreenValue === DesignAppFullscreenWindow.Accessl;
  const [camera] = useDesignAppCamera();
  const [focusedPieceGuid] = useDesignAppFocusedPieceGuid();
  const panelVisibility = useAppPanelVisibility();
  const [projection, setProjection] = React.useState<"camera" | "orthographic">("orthographic");
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
        transaction?.start();
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, type: { guid: droppedType.guid }, plane, center };
        addPiece?.(piece);
        transaction?.finalize();
      } else if (dragData.type === "design" && dragData.designGuid) {
        const droppedDesign = sceneDesigns?.find((d) => d.guid === dragData.designGuid);
        if (!droppedDesign) return;
        transaction?.start();
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, id_: pieceGuid, design: { guid: droppedDesign.guid }, plane, center };
        addPiece?.(piece);
        transaction?.finalize();
      }
      setActiveDraggedType(null);
      setActiveDraggedDesign(null);
    },
    [sceneTypes, sceneDesigns, camera, transaction, addPiece, setActiveDraggedType, setActiveDraggedDesign],
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
      if (toggleAccesslFullscreen) toggleAccesslFullscreen();
    },
    [toggleAccesslFullscreen],
  );
  const onPointerMissed = useCallback(
    (e: MouseEvent) => {
      if (!(e.ctrlKey || e.metaKey) && !e.shiftKey && deselectAll) deselectAll();
    },
    [deselectAll],
  );
  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      if (setCamera) setCamera(newCamera);
    },
    [setCamera],
  );
  const onFocusComplete = useCallback(() => {
    setTimeout(() => {
      if (clearFocus) clearFocus();
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

// #endregion 🔖Scene

// #endregion 🔖Canvas

// #region 🔖Windows
// Window components MUST wrap diagram and scene views with hover and transaction providers.
// Props interface for the Design app root component.
export interface AppProps { }

const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => {
  return (
    <HoverIntentProvider>
      <TransactionPiecesProvider>
        <HoverPiecesProvider>
          <DesignDiagram reactFlowInstanceRef={reactFlowInstanceRef} />
        </HoverPiecesProvider>
      </TransactionPiecesProvider>
    </HoverIntentProvider>
  );
});
DiagramWindow.displayName = "DiagramWindow";

const SceneWindow = memo(() => {
  return (
    <HoverIntentProvider>
      <TransactionPiecesProvider>
        <HoverPiecesProvider>
          <DesignAppScene />
        </HoverPiecesProvider>
      </TransactionPiecesProvider>
    </HoverIntentProvider>
  );
});
SceneWindow.displayName = "SceneWindow";

// #endregion 🔖Windows

// #endregion Components

// #region App
// App MUST compose all Design app panels, canvas, toolbar, and footer into the main Design app layout.

const renderCountRef = { current: 0 };

const App: FC<AppProps> = () => {
  renderCountRef.current++;
  useDesignAppInitialize();

  const { t } = useTranslation();
  const [transaction] = useDesignAppTransaction();
  const [deleteSelected] = useDesignAppDeleteSelected();
  const [undo] = useDesignAppUndo();
  const [redo] = useDesignAppRedo();
  const [addPiece] = useDesignAppAddPiece();
  const [selectAll] = useDesignAppSelectAll();
  const [deselectAll] = useDesignAppDeselectAll();
  const [toggleDiagramFullscreen] = useDesignAppToggleDiagramFullscreen();
  const [toggleAccesslFullscreen] = useDesignAppToggleAccesslFullscreen();
  const [togglePanel] = useDesignAppTogglePanel();
  const [, setActiveTool] = useDesignAppActiveTool();
  const [hoverTypes] = useDesignAppHoverTypes();
  const [hoverDesigns] = useDesignAppHoverDesigns();
  const [clearHover] = useDesignAppClearHover();
  const [activeTool] = useDesignAppActiveTool();

  const [selection] = useDesignAppSelection();
  const design = useDesign() as Design | undefined;
  const kitGuid = useKitScope()?.guid;
  const workbenchTypes = useKitTypes();
  const workbenchDesigns = useKitDesigns();
  const appSettings = useSketchpad((s) => s.settings?.apps) as any;
  const panelVisibility = useAppPanelVisibility();
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  const store = useDesignStore() as DesignStore | null;
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

  const windowLayout = useMemo(() => {
    if (!storedWindowLayout) {
      return storedWindowLayout;
    }

    const hasSceneWindow = (layout: any): boolean => {
      if (!layout) return false;

      if (layout.type === "component" && layout.componentName === DesignAppWindowKind.Scene) return true;

      if (layout.root && typeof layout.root === "object") {
        return hasSceneWindow(layout.root);
      }

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

  useEffect(() => {
    if (store && storedWindowLayout && windowLayout === undefined) {
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

  useHotkeys(
    "ctrl+a",
    () => {
      if (selectAll) selectAll();
    },
    { enableOnFormTags: true },
  );
  useHotkeys(
    "ctrl+d",
    () => {
      if (deselectAll) deselectAll();
    },
    { enableOnFormTags: true },
  );
  useHotkeys("delete", () => deleteSelected?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+z", () => undo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+y", () => redo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+shift+z", () => redo?.(), { enableOnFormTags: true });

  const appType = useAppType();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          if (setActiveTool) setActiveTool(ToolKind.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          if (setActiveTool) setActiveTool(ToolKind.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_ADDITIVE && !e.shiftKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  useEffect(() => {
    if (appType !== "design") return;

    const hasPieces = (selection.pieces || []).length > 0;
    const hasConnections = (selection.connections || []).length > 0;
    const hasPortSelected = selection.connector !== undefined;
    const hasSelection = hasPieces || hasConnections || hasPortSelected;

    const pieceSingleId = "semio.sketchpad.app.design.panel.details.section.piece.properties";
    const pieceMultipleId = "semio.sketchpad.app.design.panel.details.section.piece.multipleTitle";
    const connectionSingleId = "semio.sketchpad.app.design.panel.details.section.connection.properties";
    const connectionMultipleId = "semio.sketchpad.app.design.panel.details.section.connection.multipleTitle";
    const selectionMultipleId = "semio.sketchpad.app.design.panel.details.section.selection.multipleTitle";

    removeSection("details", "semio.sketchpad.app.design.properties");
    removeSection("details", "semio.sketchpad.app.type.connector.properties");
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
      const connectorPieceId = selection.connector!.piece;
      const connectorId = selection.connector!.connector;
      addSection("details", {
        id: "semio.sketchpad.app.type.connector.properties",
        specificity: 30,
        order: 0,
        content: () => <ConnectorSection pieceGuid={connectorPieceId} connectorGuid={connectorId} />,
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
      removeSection("details", "semio.sketchpad.app.type.connector.properties");
      removeSection("details", pieceSingleId);
      removeSection("details", pieceMultipleId);
      removeSection("details", connectionSingleId);
      removeSection("details", connectionMultipleId);
      removeSection("details", selectionMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.properties");
    };
  }, [selection, addSection, removeSection, appType, t, design]);

  const PiecesWorkbenchContent: FC = () => {
    const kit = useKit() as Kit;

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
          <div
            key={type.guid}
            onPointerEnter={() => {
              if (hoverTypes) hoverTypes([type.guid]);
            }}
            onPointerLeave={() => {
              if (clearHover) clearHover();
            }}
          >
            <TypeTreeItem type={type} onCreateChild={handleCreateTypeChild}>
              {children.length > 0 && renderTypeTree(children)}
            </TypeTreeItem>
          </div>
        );
      });
    };

    const renderDesignTree = (designs: Design[]): ReactNode[] => {
      return designs.map((workbenchDesign) => {
        const children = workbenchDesigns?.filter((child) => (typeof child.parent === "object" ? child.parent?.guid === workbenchDesign.guid : child.parent === workbenchDesign.guid)) || [];

        const isDisabled = design && kit ? areDesignsInSameFamily(kit, design.guid, workbenchDesign.guid) : false;
        return (
          <div
            key={workbenchDesign.guid}
            onPointerEnter={() => {
              if (hoverDesigns) hoverDesigns([workbenchDesign.guid]);
            }}
            onPointerLeave={() => {
              if (clearHover) clearHover();
            }}
          >
            <DesignTreeItem design={workbenchDesign} onCreateChild={handleCreateDesignChild} disabled={isDisabled}>
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
            if (hoverTypes) hoverTypes(workbenchTypes.map((type) => type.guid));
          }}
          onPointerLeave={() => {
            if (clearHover) clearHover();
          }}
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
            if (hoverDesigns) hoverDesigns(workbenchDesigns.map((design) => design.guid));
          }}
          onPointerLeave={() => {
            if (clearHover) clearHover();
          }}
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

  const DesignTreeItem: FC<{ design: Design; onCreateChild: (design: Design) => void; disabled?: boolean; children?: ReactNode }> = ({ design, onCreateChild, disabled, children }) => {
    const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
      id: `design-${design.guid}`,
      data: { type: "design", designGuid: design.guid },
      disabled: disabled,
    });

    const handleDragStart = () => {
      if (!disabled) {
        setActiveDraggedDesign(design);
      }
    };

    useEffect(() => {
      if (isDragging && !disabled) {
        handleDragStart();
      }
    }, [isDragging, disabled]);

    return (
      <TreeItem
        label={
          <div className={`flex items-center gap-single min-w-0 ${disabled ? "opacity-50 cursor-not-allowed" : ""}`}>
            <DraggableAvatar
              ref={setNodeRef}
              dragRef={setNodeRef}
              dragListeners={disabled ? {} : listeners}
              dragAttributes={disabled ? {} : attributes}
              content={design.name.substring(0, 2).toUpperCase()}
              isSelected={false}
              isHovered={false}
              shouldFade={isDragging}
              title={disabled ? `${design.name} (same design family - cannot be used as design piece)` : design.name}
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
  }, [appType, kitGuid, workbenchTypes?.length, workbenchDesigns?.length, addSection, removeSection]);

  useEffect(() => {
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
                  <div className="w-full relative" style={{ height: "4px", backgroundColor: "var(--border-element-color)" }}>
                    <div
                      style={{
                        position: "absolute",
                        left: `${((appSettings.design?.proximityConnectDistance || 10) / 20) * 100}%`,
                        top: "50%",
                        transform: "translate(-50%, -50%)",
                        width: "16px",
                        height: "16px",
                        backgroundColor: "var(--foreground)",
                        border: "1px solid var(--border-element-color)",
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

    addSection("settings", {
      id: "semio.sketchpad.app.kit.settings",
      specificity: 10,
      order: 0,
      content: () => <DesignSettingsContent />,
    });

    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: () => <DesignSettingsContent />,
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.design.settings");
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [addSection, removeSection, appSettings.design?.proximityConnectDistance, appSettings.design?.gridSize]);

  return (
    <ReactFlowProvider>
      <Canvas id="semio.sketchpad.app.design.canvas">
        <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
      </Canvas>
      <DesignAppFooter />
    </ReactFlowProvider>
  );
};

// #region 🔖Settings
// Settings MUST render the Design app settings panel with theme, language, device, expertise, and mode toggles.

const DesignSettingsContent: FC = () => {
  const [theme, setTheme, canSetTheme] = useTheme();
  const [language, setLanguage, canSetLanguage] = useLanguage();
  const [device, setDevice, canSetDevice] = useDevice();
  const [expertise, setExpertise, canSetExpertise] = useExpertise();
  const [mode, setMode, canSetMode] = useMode();

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
            onValueChange={(value: string) => setTheme?.(value as Theme)}
            showLabel
            kind="single"
            disabled={!canSetTheme}
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
          <SelectUI id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
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
            id="semio.sketchpad.settings.device"
            value={typeof device === "object" ? "desktop" : device}
            onValueChange={(value: string) => setDevice?.(value as "desktop" | "tablet")}
            showLabel
            kind="single"
            disabled={!canSetDevice}
            items={[
              { value: "desktop", id: "semio.sketchpad.settings.device.desktop", icon: <MousePointerIcon className="size-small" /> },
              { value: "tablet", id: "semio.sketchpad.settings.device.tablet", icon: <HandIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.settings.expertise"
            value={expertise}
            onValueChange={(value: string) => setExpertise?.(value as Expertise)}
            showLabel
            kind="single"
            disabled={!canSetExpertise}
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
            onValueChange={(value: string) => setMode?.(value as Mode)}
            showLabel
            kind="single"
            disabled={!canSetMode}
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

// #endregion 🔖Settings

const DesignApp: FC = () => {
  initializeDesignStore();

  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    if (appType !== "design") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.design.tools.select",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "selection",
        labelId: "semio.sketchpad.toolbar.parent.selection",
        order: 10,
        subToolId: "select",
        subToolLabelId: "semio.sketchpad.toolbar.subtool.select",
        subToolIcon: <SelectToolIcon className="size-tiny" />,
      },
      content: <DesignSelectSettings />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.design.tools.hand",
      specificity: 20,
      order: 10,
      toolbarGroup: {
        id: "hand",
        labelId: "semio.sketchpad.toolbar.parent.hand",
        order: 5,
      },
      content: <DesignHandSettings />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.design.tools.select");
      removeSection("toolbar", "semio.sketchpad.app.design.tools.hand");
    };
  }, [appType, addSection, removeSection]);

  return (
    <DesignAppTransactionProvider>
      <HoverIntentProvider>
        <TransactionPiecesProvider>
          <HoverPiecesProvider>
            <App />
          </HoverPiecesProvider>
        </TransactionPiecesProvider>
      </HoverIntentProvider>
    </DesignAppTransactionProvider>
  );
};

// #region 🔖Config
// Config MUST export the Design app configuration with route segments, panel definitions, and path matching.

// Exported Design app configuration including routes, panels, and path matching.
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
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "designs" && isUuidPattern(pathParts[3]);
  },
  order: 20,
};

// #endregion 🔖Config
