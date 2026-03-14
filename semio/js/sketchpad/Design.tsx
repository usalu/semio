// #region Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Design app providing diagram and scene windows for editing designs.

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚js🗃️sketchpad💻design🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports)
// Imports for Design app MUST include all shared sketchpad, React, and UI dependencies.

import { useSelector } from "@xstate/react";
import { ConnectionDiff, ConnectionId, Guid, KitDiff, PieceDiff, PieceId } from "../semio";
import type { AppConfig, AppPlugin, AppWindowConfig, DesignAppId, Field, HookResult, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolRenderContext } from "./shared";
import {
    applySelectionComposition,
    conditionalHookResult,
    createField as createFieldValue,
    createKeyedTransactionHandlers,
    createPanelDefinition,
    fieldToHookResult,
    isSelectionToolKind,
    PanelKind,
    readonlyHookResult,
    registerAppPlugin,
    registerEventHandler,
    registerKeyedAppEventHandlers,
    resolveSelectionCompositionKind,
    ToolKind,
    toSelectionToolKind
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
    useKitScope,
    usePieceScope,
    useSketchpadActor,
    useSketchpadActorSafe,
} from "./Sketchpad";

// #endregion Internal State Management

// #region Imports

import { DragEndEvent, useDraggable } from "@dnd-kit/core";
import { arrayMove } from "@dnd-kit/sortable";
import { Edges, Line, Select, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import { AddIcon, ChatIcon, ConnectionIcon, DiagramIcon, DisconnectIcon, HandIcon, IntersectIcon, PieceIcon, PortIcon, RemoveIcon, SceneIcon, SelectToolIcon, SettingsIcon, TableViewIcon, TypeIcon } from "@semio/assets";
import React, { createContext, FC, memo, ReactNode, Suspense, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { useLocation, useSearchParams } from "react-router";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { useLabel } from "../i18n";
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
    dragPiecesInDesign,
    expandDesignPieces,
    findDesignInKit,
    findPieceInDesign,
    findTypeInKit,
    fixPiecesInDesign,
    generateUniqueName,
    getDesignDiff,
    getIncludedDesigns,
    guid,
    ICON_WIDTH,
    Kit,
    Piece,
    Plane,
    planeToMatrix,
    Port,
    replaceClusterWithDesign,
    selectBestModel,
    TOLERANCE,
    toSemioRotation,
    toThreeRotation,
    Type
} from "../semio";
import type { ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Connection as RFConnection } from "@semio-elements/ui";
import {
    applyNodeChanges,
    Avatar,
    AvatarFallback,
    BasicChatPanel,
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
    sceneFrameControlRef,
    Slider,
    SortableTreeItems,
    Stepper,
    Textarea,
    Toggle,
    ToggleGroup,
    ToolbarGroup,
    TransactionProvider,
    Tree,
    TreeItem,
    TreeRow,
    TreeSection,
    TreeStateProvider,
    useReactFlow,
    useStoreApi,
    ViewportPortal
} from "@semio-elements/ui";
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
    useAddSidePanelTab,
    useAppPanelVisibility,
    useAppType,
    useClusterableGroups,
    useConnection,
    useConnectionDescription,
    useConnectionGap,
    useConnectionRise,
    useConnectionRotation,
    useConnections,
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
    useIsConnectedPiece,
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
    useRemoveSidePanelTab,
    useReplacableDesigns,
    useReplacableTypes,
    useSketchpad,
    useSketchpadCommands,
    useSketchpadStore,
    useTooltip,
    useType
} from "./Sketchpad";

const KitSectionLazy = React.lazy(() => import("./Kit").then((module) => ({ default: module.KitSection })));

// #endregion Imports

// #region Types

// [👤semio📚js🗃️sketchpad💻designtsx🔖statemanagement](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/STATE-MANAGEMENT)
// State management types and interfaces MUST define the Design app selection, presence, hover, diff, and state shape.

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🪨designappcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/designAppCommands)
 * designAppCommands holds the data fields for a designAppCommands record.
 **/
let designAppCommands: Record<string, (context: any, ...args: any[]) => Promise<any> | any>;

/**
 * Tracks the current piece, connection, and connector selection state for the Design app.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppSelection)
 **/
export interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: Array<{ piece: Guid; connector: Guid }>;
  connector?: { piece: Guid; designPiece?: Guid; connector: Guid };
}
/**
 * Diff for added/removed piece GUIDs in a selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappselectionpiecesdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppSelectionPiecesDiff)
 **/
export interface DesignAppSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed connection GUIDs in a selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappselectionconnectionsdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppSelectionConnectionsDiff)
 **/
export interface DesignAppSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for a selected port change identifying the piece and connector.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappselectionportdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppSelectionPortDiff)
 **/
export interface DesignAppSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  connector?: Guid;
}
/**
 * Composite diff combining pieces, connections, and connector selection changes.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppSelectionDiff)
 **/
export interface DesignAppSelectionDiff {
  pieces?: DesignAppSelectionPiecesDiff;
  connections?: DesignAppSelectionConnectionsDiff;
  connector?: DesignAppSelectionPortDiff;
}
/**
 * Enumeration of fullscreen window modes for the Design app.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappfullscreenwindow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppFullscreenWindow)
 **/
export enum DesignAppFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
/**
 * Enumeration of window kinds available in the Design app.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappwindowkind](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppWindowKind)
 **/
export enum DesignAppWindowKind {
  Diagram = "diagram",
  Scene = "scene",
  Settings = "settings",
  Chat = "chat",
}
/**
 * Presence state for a Design app user including cursor, camera, and diagram viewport.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designapppresence](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppPresence)
 **/
export interface DesignAppPresence {
  cursor?: Coord;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
/**
 * Hover state tracking which pieces, connections, connectors, types, and designs are hovered.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designapphover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppHover)
 **/
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: { piece: Guid; designPiece?: Guid; connector: Guid }[];
  types?: Guid[];
  designs?: Guid[];
}
/**
 * Extended presence for other collaborators including their display name.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designapppresenceother](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppPresenceOther)
 **/
export interface DesignAppPresenceOther extends DesignAppPresence {
  name: string;
}
/**
 * Complete diff describing all mutable Design app state changes.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppDiff)
 **/
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
/**
 * Edit record extending KitDiffAppEdit with Design app selection diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappedit](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppEdit)
 **/
export interface DesignAppEdit extends KitDiffAppEdit<DesignAppSelectionDiff> { }
/**
 * Complete runtime state for a Design app instance.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppState)
 **/
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

/**
 * Context passed to Design app commands including app state, GUID, and design data.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappcommandcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppCommandContext)
 **/
export interface DesignAppCommandContext extends KitCommandContext {
  designApp: DesignAppState;
  Guid: Guid;
  design: Design;
}
/**
 * Result returned by Design app commands containing diffs to apply.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖statemanagement🛠️designappcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/State%20Management/d/i/DesignAppCommandResult)
 **/
export interface DesignAppCommandResult {
  diff?: DesignAppDiff;
  kitDiff?: KitDiff;
}

// #endregion Types

// #region Commands
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Commands)
// Commands MUST define all executable Design app actions dispatched by keyboard shortcuts and UI interactions.

/**
 * Registry of all named Design app commands mapped to their handler functions.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖commands🪨commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Commands/d/i/commands)
 **/
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
    const designChange = replaceClusterWithDesign(context.design, validPieceGuids, clusteredDesign, externalConnections);
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
              diff: designChange.forward,
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

// #endregion Commands

// #region Store
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store)
// Store MUST implement DesignStore extending PlainKitDiffAppStore with undo/redo, selection diff inversion, and state persistence.

/**
 * Computes the inverse of a Design app selection diff for undo support.
 *
 * MUST return a diff that reverses the given selection diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨inversedesignappselectiondiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/inverseDesignAppSelectionDiff)
 **/
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
/**
 * Checks whether two Design app identifiers refer to the same design.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨aresamedesignapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/areSameDesignApp)
 **/
export const areSameDesignApp = (designApp: DesignAppId, other: DesignAppId): boolean => designApp.kit === other.kit && designApp.design === other.design;
/**
 * Checks whether a Design app identifier matches any in a list.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨hassamedesignapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/hasSameDesignApp)
 **/
export const hasSameDesignApp = (designApp: DesignAppId, others: DesignAppId[]): boolean => others.some((other) => areSameDesignApp(designApp, other));

/**
 * DesignStore manages Design app state persistence, undo/redo stacks, and Y.js synchronization.
 *
 * MUST extend PlainKitDiffAppStore and synchronize state with the Y.js shared document.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️designstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/DesignStore)
 **/
export class DesignStore extends PlainKitDiffAppStore<DesignAppState, DesignAppDiff, DesignAppSelectionDiff, DesignAppEdit, DesignAppCommandContext, DesignAppCommandResult> {
  private readonly kitGuid: Guid;
  private _draggingPieceIds: Set<string> = new Set();
  get draggingPieceIds(): Set<string> { return this._draggingPieceIds; }
  setDraggingPieces(ids: Set<string>): void { this._draggingPieceIds = ids; this.notify(); }
  clearDraggingPieces(): void { if (this._draggingPieceIds.size === 0) return; this._draggingPieceIds = new Set(); this.notify(); }
  private readonly designGuid: Guid;

  constructor(parent: SketchpadStore, id: DesignAppId, initialState?: DesignAppState) {
    const defaultState: DesignAppState = {
      fullscreenWindow: initialState?.fullscreenWindow || DesignAppFullscreenWindow.None,
      panelVisibility: initialState?.panelVisibility || { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
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
    const actor = this.parentStore.actor;
    let designAppState = state;
    if (actor) {
      const machineKey = `${this.kitGuid}:${this.designGuid}`;
      const machineContext = actor.getSnapshot().context;
      const machineDesignApp = machineContext.designApps?.[machineKey];
      if (machineDesignApp) {
        designAppState = { ...state, selection: machineDesignApp.selection };
      }
    }
    const context: DesignAppCommandContext = {
      designApp: designAppState,
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
    this.recordEdit(result);
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }
    if (result.diff?.selection && actor) {
      actor.send({ type: "DESIGN.SET_SELECTION", kitGuid: this.kitGuid, designGuid: this.designGuid, selection: this.state.selection || {} } as any);
    }
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨designstoreinitialized](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/designStoreInitialized)
 * designStoreInitialized holds the data fields for a designStoreInitialized record.
 **/
let designStoreInitialized = false;
/**
 * Initializes the Design app store factory registration.
 *
 * MUST register the DesignStore factory exactly once via registerDesignAppStoreFactory.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖store🛠️initializedesignstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/initializeDesignStore)
 **/
function initializeDesignStore() {
  designStoreInitialized = true;
  registerDesignAppStoreFactory((parent: any, id: any, state: any) => new DesignStore(parent, id, state));
}

// #region Design App Plugin Registration

// [👤semio📚js🗃️sketchpad💻designtsx🔖store🔖designapppluginregistration](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/STORE/DESIGN-APP-PLUGIN-REGISTRATION)
// Design app plugin registration MUST register the Design app plugin with machine actions, guards, and default state.

/**
 * designAppPlugin holds the data fields for a designAppPlugin record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖designapppluginregistration🪨designappplugin](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Design%20App%20Plugin%20Registration/d/i/designAppPlugin)
 **/
const designAppPlugin: AppPlugin = {
  id: "design",
  namespace: "DESIGN",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): DesignAppState => ({
      panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
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

// #endregion Design App Plugin Registration

/**
 * DesignAppScope holds the data fields for a DesignAppScope record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store✂️designappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/DesignAppScope)
 **/
type DesignAppScope = { id: string };
/**
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨designappscopecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/DesignAppScopeContext)
/**
 * DesignAppScopeContext holds the data fields for a DesignAppScopeContext record.
 **/
const DesignAppScopeContext = createContext<DesignAppScope | null>(null);
/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨designappsynccomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/DesignAppSyncComponent)
 * DesignAppActorContext holds the data fields for a DesignAppActorContext record.
 **/
const DesignAppActorContext = createContext<any>(null);

// [👤semio📚js🗃️sketchpad💻design🔖store🪨designappsynccomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/DesignAppSyncComponent)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🪨designappsynccomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/DesignAppSyncComponent)
 * DesignAppSyncComponent holds the data fields for a DesignAppSyncComponent record.
 **/
const DesignAppSyncComponent = ({ children }: { children: React.ReactNode }) => {
  useDesignAppInitialize();
  return <>{children}</>;
};

// #region 🔖Hooks
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖hooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Hooks)
// Hooks MUST provide the Design app initialization lifecycle within the React component tree.

/** useDesignAppInitialize holds the data fields for a useDesignAppInitialize record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖hooks🛠️usedesignappinitialize](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Hooks/d/i/useDesignAppInitialize)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🔖hooks🪨usedesignappinitialize](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/s/Hooks/d/i/useDesignAppInitialize)
 **/
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
        panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
        selection: undefined,
        hover: undefined,
        camera: undefined,
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

// #endregion Store

// #region Components
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components)
// Components MUST provide Design app scope, actor context, and synchronization wrapper components.

/**
 * Provider component that establishes Design app scope and actor context.
 *
 * MUST wrap children with DesignAppScopeContext and DesignAppActorContext providers.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🪨usedesignappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppScope)
 **/
export const DesignAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignAppScopeContext.Provider, { value }, React.createElement(DesignAppActorContext.Provider, { value: null }, React.createElement(DesignAppSyncComponent, null, props.children)));
};

// [👤semio📚js🗃️sketchpad💻design🔖store🔖components🪨usedesignappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/s/Components/d/i/useDesignAppScope)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🔖components🪨usedesignappscope](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/s/Components/d/i/useDesignAppScope)
 * useDesignAppScope holds the data fields for a useDesignAppScope record.
 **/
const useDesignAppScope = () => useContext(DesignAppScopeContext);

/**
 * Returns the current Design app XState actor from context.
 *
 * MUST return the actor from DesignAppActorContext.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappactor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppActor)
 **/
export function useDesignAppActor(): any {
  return useContext(DesignAppActorContext);
}

/**
 * Selects derived state from the Design app store.
 *
 * MUST resolve the DesignStore from the orchestrator and apply the selector.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignStore)
 **/
export function useDesignStore<T = DesignStore>(selector?: (store: DesignStore) => T, id?: DesignAppId): T | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedDesignId = designScope?.guid ?? id?.design;
  if (!resolvedKitId || !resolvedDesignId) {
    return null;
  }
  const designAppStore = store.designApp(resolvedKitId, resolvedDesignId);
  return selector ? selector(designAppStore) : (designAppStore as any);
}

export { useDesignStore as useDesignAppStore };

/**
 * Selects derived state from the Design app XState snapshot.
 *
 * MUST use useSelector to reactively track the Design app state slice.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignApp)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🪨emptyselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/EMPTY_SELECTION)
 * EMPTY_SELECTION holds the data fields for a EMPTY_SELECTION record.
 **/
const EMPTY_SELECTION: DesignAppSelection = {};
/**
 * EMPTY_OTHERS holds the data fields for a EMPTY_OTHERS record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🪨emptyothers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/EMPTY_OTHERS)
 **/
const EMPTY_OTHERS: DesignAppPresenceOther[] = [];
/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🪨emptymodeltags](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/EMPTY_MODEL_TAGS)
 * EMPTY_MODEL_TAGS holds the data fields for a EMPTY_MODEL_TAGS record.
 **/
const EMPTY_MODEL_TAGS: Record<Guid, string[]> = {};
const DEFAULT_PANEL_VISIBILITY: PanelVisibility = { toolbar: false, details: true, rightSidePanel: true };

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components✂️granularselectorfactory](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/GranularSelectorFactory)
 * GranularSelectorFactory holds the data fields for a GranularSelectorFactory record.
 **/
type GranularSelectorFactory<T> = (kitGuid: Guid, designGuid: Guid) => (state: any) => T | undefined;

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components✂️usedesignappfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppField)
 * UseDesignAppFieldOptions holds the data fields for a UseDesignAppFieldOptions record.
 **/
interface UseDesignAppFieldOptions<T, TEvent extends { type: string }> {
  createGranularSelector: GranularSelectorFactory<T>;
  fallback: T;
  createCanEvent: (kitGuid: Guid, designGuid: Guid) => TEvent;
  createSendEvent: (kitGuid: Guid, designGuid: Guid, value: T) => TEvent;
  useWildcardFallback?: boolean;
}

// [👤semio📚js🗃️sketchpad💻design🔖store🔖components🛠️usedesignappfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/s/Components/d/i/useDesignAppField)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🔖components🪨usedesignappfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/s/Components/d/i/useDesignAppField)
 * useDesignAppField holds the data fields for a useDesignAppField record.
 **/
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

/**
 * Returns a reactive field for a Design app selection property.
 *
 * MUST create a Field wrapping the selection value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappselectionfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppSelectionField)
 **/
export function useDesignAppSelectionField(): Field<DesignAppSelection> {
  return useDesignAppField<DesignAppSelection, { type: "DESIGN.SET_SELECTION"; kitGuid: Guid; designGuid: Guid; selection: DesignAppSelection }>({
    createGranularSelector: createDesignSelectionSelector,
    fallback: EMPTY_SELECTION,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection: {} as DesignAppSelection }),
    createSendEvent: (kitGuid, designGuid, selection) => ({ type: "DESIGN.SET_SELECTION", kitGuid, designGuid, selection }),
  });
}

/**
 * Returns a hook result for the current Design app selection.
 *
 * MUST provide the current selection, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppSelection)
 **/
export function useDesignAppSelection(): HookResult<DesignAppSelection> {
  return fieldToHookResult(useDesignAppSelectionField());
}

/**
 * Returns a reactive field for the Design app fullscreen window.
 *
 * MUST create a Field wrapping the fullscreen value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappfullscreenfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppFullscreenField)
 **/
export function useDesignAppFullscreenField(): Field<DesignAppFullscreenWindow> {
  return useDesignAppField<DesignAppFullscreenWindow, { type: "DESIGN.SET_FULLSCREEN"; kitGuid: Guid; designGuid: Guid; window: DesignAppFullscreenWindow }>({
    createGranularSelector: createDesignFullscreenWindowSelector,
    fallback: DesignAppFullscreenWindow.None,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: DesignAppFullscreenWindow.None }),
    createSendEvent: (kitGuid, designGuid, fullscreen) => ({ type: "DESIGN.SET_FULLSCREEN", kitGuid, designGuid, window: fullscreen }),
  });
}

/**
 * Returns a hook result for the Design app fullscreen window state.
 *
 * MUST provide the current fullscreen window, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappfullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppFullscreen)
 **/
export function useDesignAppFullscreen(): HookResult<DesignAppFullscreenWindow> {
  return fieldToHookResult(useDesignAppFullscreenField());
}

/**
 * Returns a reactive field for the Design app active tool.
 *
 * MUST create a Field wrapping the active tool value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappactivetoolfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppActiveToolField)
 **/
export function useDesignAppActiveToolField(): Field<ToolKind> {
  return useDesignAppField<ToolKind, { type: "DESIGN.SET_ACTIVE_TOOL"; kitGuid: Guid; designGuid: Guid; tool: ToolKind }>({
    createGranularSelector: createDesignActiveToolSelector,
    fallback: ToolKind.SELECTION_NORMAL,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool: ToolKind.SELECTION_NORMAL }),
    createSendEvent: (kitGuid, designGuid, tool) => ({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool }),
    useWildcardFallback: true,
  });
}

/**
 * Returns a hook result for the Design app active tool.
 *
 * MUST provide the current active tool, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappactivetool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppActiveTool)
 **/
export function useDesignAppActiveTool(): HookResult<ToolKind> {
  return fieldToHookResult(useDesignAppActiveToolField());
}

/**
 * Returns a hook result for the Design app diff state.
 *
 * MUST provide the current diff, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppDiff)
 **/
export function useDesignAppDiff(): HookResult<KitDiff | undefined> {
  return readonlyHookResult<KitDiff | undefined>(undefined);
}

/**
 * Returns other collaborators' presence state for the Design app.
 *
 * MUST return a read-only list of other users' presence data.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappothers](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppOthers)
 **/
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

/**
 * Returns a reactive field for the Design app camera.
 *
 * MUST create a Field wrapping the camera value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappcamerafield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppCameraField)
 **/
export function useDesignAppCameraField(): Field<Camera | undefined> {
  return useDesignAppField<Camera | undefined, { type: "DESIGN.SET_CAMERA"; kitGuid: Guid; designGuid: Guid; camera: Camera | undefined }>({
    createGranularSelector: createDesignCameraSelector,
    fallback: undefined,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera: undefined }),
    createSendEvent: (kitGuid, designGuid, camera) => ({ type: "DESIGN.SET_CAMERA", kitGuid, designGuid, camera }),
  });
}

/**
 * Returns a hook result for the Design app camera state.
 *
 * MUST provide the current camera, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappcamera](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppCamera)
 **/
export function useDesignAppCamera(): HookResult<Camera | undefined> {
  return fieldToHookResult(useDesignAppCameraField());
}

/**
 * Returns a hook result for the Design app diagram center coordinate.
 *
 * MUST provide the current diagram center, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappdiagramcenter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppDiagramCenter)
 **/
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

/**
 * Returns a hook result for the Design app diagram scale.
 *
 * MUST provide the current diagram scale, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappdiagramscale](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppDiagramScale)
 **/
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

/**
 * Returns a reactive field for the focused piece GUID.
 *
 * MUST create a Field wrapping the focused piece GUID value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappfocusedpieceguidfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppFocusedPieceGuidField)
 **/
export function useDesignAppFocusedPieceGuidField(): Field<Guid | undefined> {
  return useDesignAppField<Guid | undefined, { type: "DESIGN.FOCUS_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid | undefined }>({
    createGranularSelector: createDesignFocusedPieceSelector,
    fallback: undefined,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid: undefined }),
    createSendEvent: (kitGuid, designGuid, pieceGuid) => ({ type: "DESIGN.FOCUS_PIECE", kitGuid, designGuid, pieceGuid }),
  });
}

/**
 * Returns a hook result for the focused piece GUID.
 *
 * MUST provide the current focused piece GUID, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappfocusedpieceguid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppFocusedPieceGuid)
 **/
export function useDesignAppFocusedPieceGuid(): HookResult<Guid | undefined> {
  return fieldToHookResult(useDesignAppFocusedPieceGuidField());
}

/**
 * Returns a hook result for the Design app selected model tags.
 *
 * MUST provide the current selected model tags, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappselectedmodeltags](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppSelectedModelTags)
 **/
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

/**
 * Returns a hook result for the Design app hover state.
 *
 * MUST provide the current hover, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignapphover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppHover)
 **/
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
      setTimeout(() => {
        if (hover && (hover.pieces?.length || hover.connections?.length)) {
          actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover });
        } else {
          actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid });
        }
      }, 0);
    };
  }, [actor, kitGuid, designGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Returns a reactive field for Design app panel visibility.
 *
 * MUST create a Field wrapping the panel visibility value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignapppanelvisibilityfield](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppPanelVisibilityField)
 **/
export function useDesignAppPanelVisibilityField(): Field<PanelVisibility> {
  return useDesignAppField<PanelVisibility, { type: "DESIGN.SET_PANEL_VISIBILITY"; kitGuid: Guid; designGuid: Guid; panelVisibility: PanelVisibility }>({
    createGranularSelector: createDesignPanelVisibilitySelector,
    fallback: DEFAULT_PANEL_VISIBILITY,
    createCanEvent: (kitGuid, designGuid) => ({ type: "DESIGN.SET_PANEL_VISIBILITY", kitGuid, designGuid, panelVisibility: {} as PanelVisibility }),
    createSendEvent: (kitGuid, designGuid, panelVisibility) => ({ type: "DESIGN.SET_PANEL_VISIBILITY", kitGuid, designGuid, panelVisibility }),
  });
}

/**
 * Returns a hook result for Design app panel visibility.
 *
 * MUST provide the current panel visibility, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignapppanelvisibility](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppPanelVisibility)
 **/
export function useDesignAppPanelVisibility(): HookResult<PanelVisibility> {
  return fieldToHookResult(useDesignAppPanelVisibilityField());
}

// #region Action Hooks

// [👤semio📚js🗃️sketchpad💻designtsx🔖store🔖components🔖actionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/STORE/COMPONENTS/ACTION-HOOKS)
// Action hooks MUST provide composable React hooks for Design app selection, hover, focus, panel, and transaction actions.

/**
 * Tuple type for action hook results pairing an action callback with a canAct flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️actionhookresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/ActionHookResult)
 **/
export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

/**
 * Returns an action to set hover state to a single piece.
 *
 * MUST return a callback that sets hover to the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphoverpiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverPiece)
 **/
export function useDesignAppHoverPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuid: string) => setHover({ pieces: [pieceGuid] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to set hover state to multiple pieces.
 *
 * MUST return a callback that sets hover to the given piece GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphoverpieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverPieces)
 **/
export function useDesignAppHoverPieces(): ActionHookResult<[pieceGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuids: string[]) => setHover({ pieces: pieceGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to set hover state to a single connection.
 *
 * MUST return a callback that sets hover to the given connection GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphoverconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverConnection)
 **/
export function useDesignAppHoverConnection(): ActionHookResult<[connectionGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (connectionGuid: string) => setHover({ connections: [connectionGuid] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to set hover state to a single port.
 *
 * MUST return a callback that sets hover to the given port identifiers.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphoverport](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverPort)
 **/
export function useDesignAppHoverPort(): ActionHookResult<[pieceGuid: string, connectorGuid: string]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (pieceGuid: string, connectorGuid: string) => setHover({ connectors: [{ piece: pieceGuid, connector: connectorGuid }] });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to set hover state to types.
 *
 * MUST return a callback that sets hover to the given type GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphovertypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverTypes)
 **/
export function useDesignAppHoverTypes(): ActionHookResult<[typeGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (typeGuids: string[]) => setHover({ types: typeGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to set hover state to designs.
 *
 * MUST return a callback that sets hover to the given design GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapphoverdesigns](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppHoverDesigns)
 **/
export function useDesignAppHoverDesigns(): ActionHookResult<[designGuids: string[]]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (designGuids: string[]) => setHover({ designs: designGuids });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to clear the Design app hover state.
 *
 * MUST return a callback that clears all hover state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappclearhover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppClearHover)
 **/
export function useDesignAppClearHover(): ActionHookResult<[]> {
  const [, setHover, canSetHover] = useDesignAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return () => setHover(undefined);
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Returns an action to select a single piece.
 *
 * MUST return a callback that selects the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappselectpiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppSelectPiece)
 **/
export function useDesignAppSelectPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuid: string) => setSelection({ pieces: [pieceGuid] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Returns an action to select multiple pieces.
 *
 * MUST return a callback that selects the given piece GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappselectpieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppSelectPieces)
 **/
export function useDesignAppSelectPieces(): ActionHookResult<[pieceGuids: string[]]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (pieceGuids: string[]) => setSelection({ pieces: pieceGuids });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Returns an action to add a piece to the current selection.
 *
 * MUST return a callback that adds the given piece GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddpiecetoselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddPieceToSelection)
 **/
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

/**
 * Returns an action to remove a piece from the current selection.
 *
 * MUST return a callback that removes the given piece GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremovepiecefromselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemovePieceFromSelection)
 **/
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

/**
 * Returns an action to select a single connection.
 *
 * MUST return a callback that selects the given connection GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappselectconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppSelectConnection)
 **/
export function useDesignAppSelectConnection(): ActionHookResult<[connectionGuid: string]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectionGuid: string) => setSelection({ connections: [connectionGuid] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Returns an action to add a connection to the current selection.
 *
 * MUST return a callback that adds the given connection GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddconnectiontoselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddConnectionToSelection)
 **/
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

/**
 * Returns an action to remove a connection from the current selection.
 *
 * MUST return a callback that removes the given connection GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremoveconnectionfromselection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemoveConnectionFromSelection)
 **/
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

/**
 * Returns an action to select a piece port.
 *
 * MUST return a callback that selects the given piece-connector port.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappselectpieceport](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppSelectPiecePort)
 **/
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

/**
 * Returns an action to deselect a piece port.
 *
 * MUST return a callback that deselects the given piece-connector port.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappdeselectpieceport](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppDeselectPiecePort)
 **/
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

/**
 * Returns an action to deselect all items in the Design app.
 *
 * MUST return a callback that clears all selection state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappdeselectall](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppDeselectAll)
 **/
export function useDesignAppDeselectAll(): ActionHookResult<[]> {
  const [, setSelection, canSetSelection] = useDesignAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => setSelection({});
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Returns an action to select all pieces and connections.
 *
 * MUST return a callback that adds all piece and connection GUIDs to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappselectall](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppSelectAll)
 **/
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

/**
 * Returns an action to focus on a specific piece.
 *
 * MUST return a callback that sets the focused piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappfocuspiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppFocusPiece)
 **/
export function useDesignAppFocusPiece(): ActionHookResult<[pieceGuid: string]> {
  const [, setFocusedPieceGuid, canSetFocus] = useDesignAppFocusedPieceGuid();
  const action = useMemo(() => {
    if (!canSetFocus || !setFocusedPieceGuid) return undefined;
    return (pieceGuid: string) => setFocusedPieceGuid(pieceGuid);
  }, [setFocusedPieceGuid, canSetFocus]);
  return [action, canSetFocus];
}

/**
 * Returns an action to clear the focused piece.
 *
 * MUST return a callback that clears the focused piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappclearfocus](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppClearFocus)
 **/
export function useDesignAppClearFocus(): ActionHookResult<[]> {
  const [, setFocusedPieceGuid, canSetFocus] = useDesignAppFocusedPieceGuid();
  const action = useMemo(() => {
    if (!canSetFocus || !setFocusedPieceGuid) return undefined;
    return () => setFocusedPieceGuid(undefined);
  }, [setFocusedPieceGuid, canSetFocus]);
  return [action, canSetFocus];
}

/**
 * Returns an action to toggle diagram fullscreen mode.
 *
 * MUST return a callback that toggles the diagram fullscreen window state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapptogglediagramfullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppToggleDiagramFullscreen)
 **/
export function useDesignAppToggleDiagramFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useDesignAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === DesignAppFullscreenWindow.Diagram ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Diagram);
  }, [fullscreen, setFullscreen, canSetFullscreen]);
  return [action, canSetFullscreen];
}

/**
 * Returns an action to toggle accessl fullscreen mode.
 *
 * MUST return a callback that toggles the accessl fullscreen window state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapptoggleaccesslfullscreen](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppToggleAccesslFullscreen)
 **/
export function useDesignAppToggleAccesslFullscreen(): ActionHookResult<[]> {
  const [fullscreen, setFullscreen, canSetFullscreen] = useDesignAppFullscreen();
  const action = useMemo(() => {
    if (!canSetFullscreen || !setFullscreen) return undefined;
    return () => setFullscreen(fullscreen === DesignAppFullscreenWindow.Accessl ? DesignAppFullscreenWindow.None : DesignAppFullscreenWindow.Accessl);
  }, [fullscreen, setFullscreen, canSetFullscreen]);
  return [action, canSetFullscreen];
}

/**
 * Returns an action to toggle a specific panel's visibility.
 *
 * MUST return a callback that toggles the given panel's visibility.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapptogglepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppTogglePanel)
 **/
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

/**
 * Returns an action to add a model tag for all types.
 *
 * MUST return a callback that adds the given tag to all type entries.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddmodeltagforalltypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddModelTagForAllTypes)
 **/
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

/**
 * Returns an action to remove a model tag from all types.
 *
 * MUST return a callback that removes the given tag from all type entries.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremovemodeltagfromalltypes](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemoveModelTagFromAllTypes)
 **/
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

/**
 * Interface for transaction action callbacks including start, finalize, and abort.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️transactionactions](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/TransactionActions)
 **/
export interface TransactionActions {
  start: () => void;
  finalize: () => void;
  abort: () => void;
}

/**
 * Returns the Design app transaction controller.
 *
 * MUST provide start, finalize, and abort transaction actions.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignapptransaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppTransaction)
 **/
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

/**
 * Provider component that establishes Design app transaction context.
 *
 * MUST wrap children with the Design app transaction provider.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🪨designapptransactionprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/DesignAppTransactionProvider)
 **/
export const DesignAppTransactionProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [transaction] = useDesignAppTransaction();
  return <TransactionProvider transaction={transaction}>{children}</TransactionProvider>;
};

/**
 * Returns an action to undo the last Design app transaction.
 *
 * MUST return a callback that undoes the most recent transaction.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappundo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppUndo)
 **/
export function useDesignAppUndo(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.undo", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to redo the last undone Design app transaction.
 *
 * MUST return a callback that redoes the most recently undone transaction.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappredo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRedo)
 **/
export function useDesignAppRedo(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.redo", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to delete all currently selected items.
 *
 * MUST return a callback that removes all selected pieces and connections.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappdeleteselected](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppDeleteSelected)
 **/
export function useDesignAppDeleteSelected(): ActionHookResult<[]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return () => store.execute("semio.designApp.deleteSelected", getOrigin());
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to add a piece to the design.
 *
 * MUST return a callback that adds a piece with the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddpiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddPiece)
 **/
export function useDesignAppAddPiece(): ActionHookResult<[piece: Piece]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (piece: Piece) => store.execute("semio.designApp.addPiece", getOrigin(), piece);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to add multiple pieces to the design.
 *
 * MUST return a callback that adds pieces with the given type GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddpieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddPieces)
 **/
export function useDesignAppAddPieces(): ActionHookResult<[pieces: Piece[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieces: Piece[]) => store.execute("semio.designApp.addPieces", getOrigin(), pieces);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to remove a piece from the design.
 *
 * MUST return a callback that removes the piece with the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremovepiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemovePiece)
 **/
export function useDesignAppRemovePiece(): ActionHookResult<[pieceGuid: Guid]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuid: Guid) => store.execute("semio.designApp.removePiece", getOrigin(), pieceGuid);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to remove multiple pieces from the design.
 *
 * MUST return a callback that removes the pieces with the given GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremovepieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemovePieces)
 **/
export function useDesignAppRemovePieces(): ActionHookResult<[pieceGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuids: Guid[]) => store.execute("semio.designApp.removePieces", getOrigin(), pieceGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to update a piece in the design.
 *
 * MUST return a callback that updates the piece with the given GUID and partial data.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappupdatepiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppUpdatePiece)
 **/
export function useDesignAppUpdatePiece(): ActionHookResult<[pieceGuid: Guid, diff: PieceDiff]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuid: Guid, diff: PieceDiff) => store.execute("semio.designApp.updatePiece", getOrigin(), pieceGuid, diff);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to update multiple pieces in the design.
 *
 * MUST return a callback that updates the pieces with the given GUID-data pairs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappupdatepieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppUpdatePieces)
 **/
export function useDesignAppUpdatePieces(): ActionHookResult<[updates: { id: Guid; diff: PieceDiff }[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (updates: { id: Guid; diff: PieceDiff }[]) => {
      return store.execute(
        "semio.designApp.updatePieces",
        getOrigin(),
        updates.map((u) => ({ piece: { guid: u.id }, diff: u.diff })),
      );
    };
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to add a connection to the design.
 *
 * MUST return a callback that adds a connection with the given data.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddConnection)
 **/
export function useDesignAppAddConnection(): ActionHookResult<[connection: Connection]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connection: Connection) => store.execute("semio.designApp.addConnection", getOrigin(), connection);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to add multiple connections to the design.
 *
 * MUST return a callback that adds connections with the given data array.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappaddconnections](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppAddConnections)
 **/
export function useDesignAppAddConnections(): ActionHookResult<[connections: Connection[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connections: Connection[]) => store.execute("semio.designApp.addConnections", getOrigin(), connections);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to remove a connection from the design.
 *
 * MUST return a callback that removes the connection with the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremoveconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemoveConnection)
 **/
export function useDesignAppRemoveConnection(): ActionHookResult<[connectionGuid: Guid]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuid: Guid) => store.execute("semio.designApp.removeConnection", getOrigin(), connectionGuid);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to remove multiple connections from the design.
 *
 * MUST return a callback that removes the connections with the given GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappremoveconnections](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppRemoveConnections)
 **/
export function useDesignAppRemoveConnections(): ActionHookResult<[connectionGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuids: Guid[]) => store.execute("semio.designApp.removeConnections", getOrigin(), connectionGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to update a connection in the design.
 *
 * MUST return a callback that updates the connection with the given GUID and partial data.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappupdateconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppUpdateConnection)
 **/
export function useDesignAppUpdateConnection(): ActionHookResult<[connectionGuid: Guid, diff: ConnectionDiff]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (connectionGuid: Guid, diff: ConnectionDiff) => store.execute("semio.designApp.updateConnection", getOrigin(), connectionGuid, diff);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to update multiple connections in the design.
 *
 * MUST return a callback that updates the connections with the given GUID-data pairs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappupdateconnections](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppUpdateConnections)
 **/
export function useDesignAppUpdateConnections(): ActionHookResult<[updates: { id: Guid; diff: ConnectionDiff }[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (updates: { id: Guid; diff: ConnectionDiff }[]) =>
      store.execute(
        "semio.designApp.updateConnections",
        getOrigin(),
        updates.map((u) => ({ connection: { guid: u.id }, diff: u.diff })),
      );
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to cluster selected pieces into a new design.
 *
 * MUST return a callback that clusters the given piece GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappclusterpieces](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppClusterPieces)
 **/
export function useDesignAppClusterPieces(): ActionHookResult<[pieceGuids: Guid[]]> {
  const store = useDesignStore() as DesignStore | null;
  const getOrigin = useOrigin();
  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuids: Guid[]) => store.execute("semio.designApp.clusterPieces", getOrigin(), pieceGuids);
  }, [store, getOrigin]);
  return [action, !!store];
}

/**
 * Returns an action to expand a nested design into inline pieces.
 *
 * MUST return a callback that expands the design with the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🔖actionhooks🛠️usedesignappexpanddesign](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/s/Action%20Hooks/d/i/useDesignAppExpandDesign)
 **/
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

/**
 * EMPTY_COMMANDS holds the data fields for a EMPTY_COMMANDS record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🪨emptycommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/EMPTY_COMMANDS)
 **/
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

/**
 * Returns the full Design app commands API for programmatic access.
 *
 * MUST expose all Design app commands through the store controller.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppCommands)
 **/
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
      hoverPiece: (_origin: string, guid: Guid) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: [guid] } }), 0),
      hoverPieces: (_origin: string, guids: Guid[]) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { pieces: guids } }), 0),
      hoverConnection: (_origin: string, guid: Guid) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connections: [guid] } }), 0),
      hoverConnections: (_origin: string, guids: Guid[]) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connections: guids } }), 0),
      hoverPort: (_origin: string, pieceGuid: Guid, connectorGuid: Guid) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { connectors: [{ piece: pieceGuid, connector: connectorGuid }] } }), 0),
      hoverType: (_origin: string, guid: Guid) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { types: [guid] } }), 0),
      hoverTypes: (_origin: string, guids: Guid[]) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { types: guids } }), 0),
      hoverDesign: (_origin: string, guid: Guid) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { designs: [guid] } }), 0),
      hoverDesigns: (_origin: string, guids: Guid[]) => setTimeout(() => actor.send({ type: "DESIGN.SET_HOVER", kitGuid, designGuid, hover: { designs: guids } }), 0),
      clearHover: (_origin: string) => setTimeout(() => actor.send({ type: "DESIGN.CLEAR_HOVER", kitGuid, designGuid }), 0),
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

/**
 * Synchronizes Y.js document changes to XState Design app state.
 *
 * MUST observe Y.js map changes and dispatch corresponding XState events.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🔖components🛠️usedesignappyjstoxstatesync](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/s/Components/d/i/useDesignAppYjsToXStateSync)
 **/
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

function getTransactionAffectedPieces(store: DesignStore | null): { changedPieces: Set<string>; statusMap: Map<string, DiffStatus> } {
  const changedPieces = new Set<string>();
  const statusMap = new Map<string, DiffStatus>();

  if (!store) return { changedPieces, statusMap };
  for (const pieceId of store.draggingPieceIds) {
    changedPieces.add(pieceId);
    if (!statusMap.has(pieceId)) statusMap.set(pieceId, DiffStatus.Modified);
  }
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store✂️transactionpiecescontextvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/TransactionPiecesContextValue)
 * TransactionPiecesContextValue holds the data fields for a TransactionPiecesContextValue record.
 **/
interface TransactionPiecesContextValue {
  changedPieces: Set<string>;
  statusMap: Map<string, DiffStatus>;
}

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨emptytransactioncontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/EMPTY_TRANSACTION_CONTEXT)
 * EMPTY_TRANSACTION_CONTEXT holds the data fields for a EMPTY_TRANSACTION_CONTEXT record.
 **/
const EMPTY_TRANSACTION_CONTEXT: TransactionPiecesContextValue = {
  changedPieces: new Set(),
  statusMap: new Map(),
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨transactionpiecescontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/TransactionPiecesContext)
 * TransactionPiecesContext holds the data fields for a TransactionPiecesContext record.
 **/
const TransactionPiecesContext = createContext<TransactionPiecesContextValue>(EMPTY_TRANSACTION_CONTEXT);

/**
 * areSetsEqual holds the data fields for a areSetsEqual record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️aresetsequal](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/areSetsEqual)
 **/
function areSetsEqual<T>(a: Set<T>, b: Set<T>): boolean {
  if (a.size !== b.size) return false;
  for (const item of a) if (!b.has(item)) return false;
  return true;
}

/**
 * areMapsEqual holds the data fields for a areMapsEqual record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️aremapsequal](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/areMapsEqual)
 **/
function areMapsEqual<K, V>(a: Map<K, V>, b: Map<K, V>): boolean {
  if (a.size !== b.size) return false;
  for (const [key, value] of a) if (b.get(key) !== value) return false;
  return true;
}

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️transactionpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/TransactionPiecesProviderInner)
 * areTransactionContextsEqual holds the data fields for a areTransactionContextsEqual record.
 **/
function areTransactionContextsEqual(a: TransactionPiecesContextValue, b: TransactionPiecesContextValue): boolean {
  return areSetsEqual(a.changedPieces, b.changedPieces) && areMapsEqual(a.statusMap, b.statusMap);
}

// [👤semio📚js🗃️sketchpad💻design🔖store🛠️transactionpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/TransactionPiecesProviderInner)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🪨transactionpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/TransactionPiecesProviderInner)
 * TransactionPiecesProviderInner holds the data fields for a TransactionPiecesProviderInner record.
 **/
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

/**
 * Provider that makes transaction-changed piece GUIDs available to children.
 *
 * MUST compute and provide the set of piece GUIDs changed in the current transaction.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️transactionpiecesprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/TransactionPiecesProvider)
 **/
export function TransactionPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignStore(identitySelector) as DesignStore | null;

  if (!store) {
    return <TransactionPiecesContext.Provider value={EMPTY_TRANSACTION_CONTEXT}>{children}</TransactionPiecesContext.Provider>;
  }

  return <TransactionPiecesProviderInner store={store}>{children}</TransactionPiecesProviderInner>;
}

/**
 * Returns whether a piece is changed in the current transaction.
 *
 * MUST check the transaction pieces context for the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️useisdesignpiecechangedintransaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useIsDesignPieceChangedInTransaction)
 **/
export function useIsDesignPieceChangedInTransaction(id: DesignAppId | undefined, pieceId: string): boolean {
  const { changedPieces } = useContext(TransactionPiecesContext);
  return changedPieces.has(pieceId);
}

/**
 * Returns whether a piece is currently hovered in the Design app.
 *
 * MUST check the hover state for the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappispiecehovered](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsPieceHovered)
 **/
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

/**
 * HoverPiecesContextValue holds the data fields for a HoverPiecesContextValue record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store✂️hoverpiecescontextvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/HoverPiecesContextValue)
 **/
interface HoverPiecesContextValue {
  transitivelyHoveredPieces: Set<string>;
  transitivelyHoveredTypes: Set<string>;
}

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨emptyhovercontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/EMPTY_HOVER_CONTEXT)
 * EMPTY_HOVER_CONTEXT holds the data fields for a EMPTY_HOVER_CONTEXT record.
 **/
const EMPTY_HOVER_CONTEXT: HoverPiecesContextValue = {
  transitivelyHoveredPieces: new Set(),
  transitivelyHoveredTypes: new Set(),
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨computehoverdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/computeHoverData)
 * HoverPiecesContext holds the data fields for a HoverPiecesContext record.
 **/
const HoverPiecesContext = createContext<HoverPiecesContextValue>(EMPTY_HOVER_CONTEXT);

// [👤semio📚js🗃️sketchpad💻design🔖store🛠️computehoverdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/computeHoverData)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🪨computehoverdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/computeHoverData)
 * computeHoverData holds the data fields for a computeHoverData record.
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️hoverpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/HoverPiecesProviderInner)
 * areHoverContextsEqual holds the data fields for a areHoverContextsEqual record.
 **/
function areHoverContextsEqual(a: HoverPiecesContextValue, b: HoverPiecesContextValue): boolean {
  return areSetsEqual(a.transitivelyHoveredPieces, b.transitivelyHoveredPieces) && areSetsEqual(a.transitivelyHoveredTypes, b.transitivelyHoveredTypes);
}

// [👤semio📚js🗃️sketchpad💻design🔖store🛠️hoverpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/HoverPiecesProviderInner)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖store🪨hoverpiecesproviderinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Store/d/i/HoverPiecesProviderInner)
 * HoverPiecesProviderInner holds the data fields for a HoverPiecesProviderInner record.
 **/
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

/**
 * Provider that makes transitively hovered piece GUIDs available to children.
 *
 * MUST compute and provide the set of piece GUIDs that are transitively hovered.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️hoverpiecesprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/HoverPiecesProvider)
 **/
export function HoverPiecesProvider({ children }: { children: ReactNode }) {
  const store = useDesignStore(identitySelector) as DesignStore | null;

  if (!store) {
    return <HoverPiecesContext.Provider value={EMPTY_HOVER_CONTEXT}>{children}</HoverPiecesContext.Provider>;
  }

  return <HoverPiecesProviderInner store={store}>{children}</HoverPiecesProviderInner>;
}

/**
 * Returns whether a piece is transitively hovered via type or design hierarchy.
 *
 * MUST check the transitive hover pieces for the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappispiecetransitivehovered](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsPieceTransitiveHovered)
 **/
export function useDesignAppIsPieceTransitiveHovered(id?: DesignAppId, pieceId?: string): boolean {
  const { transitivelyHoveredPieces } = useContext(HoverPiecesContext);
  if (!pieceId) return false;
  return transitivelyHoveredPieces.has(pieceId);
}

/**
 * Returns whether a type is transitively hovered in the Design app.
 *
 * MUST check the hover state for the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappistypetransitivehovered](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsTypeTransitiveHovered)
 **/
export function useDesignAppIsTypeTransitiveHovered(id: DesignAppId | undefined, typeId: string): boolean {
  const { transitivelyHoveredTypes } = useContext(HoverPiecesContext);
  return transitivelyHoveredTypes.has(typeId);
}

/**
 * Returns the diff status of a piece for visual indication.
 *
 * MUST return DiffStatus from the design diff for the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignapppiecestatus](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppPieceStatus)
 **/
export function useDesignAppPieceStatus(id: DesignAppId | undefined, pieceId: string): DiffStatus {
  const { statusMap } = useContext(TransactionPiecesContext);
  return statusMap.get(pieceId) ?? DiffStatus.Unchanged;
}

/**
 * Returns whether a piece is currently selected in the Design app.
 *
 * MUST check the selection state for the given piece GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappispieceselected](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsPieceSelected)
 **/
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

/**
 * Returns the computed color for a piece based on its status.
 *
 * MUST derive the color from selection, hover, diff status, and type mapping.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignapppiececolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppPieceColor)
 **/
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

/**
 * Returns whether a connection is currently hovered in the Design app.
 *
 * MUST check the hover state for the given connection GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappisconnectionhovered](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsConnectionHovered)
 **/
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

/**
 * Returns whether a connection is currently selected in the Design app.
 *
 * MUST check the selection state for the given connection GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappisconnectionselected](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsConnectionSelected)
 **/
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

/**
 * Returns whether a port is currently hovered in the Design app.
 *
 * MUST check the hover state for the given piece-connector port.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappisporthovered](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsPortHovered)
 **/
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

/**
 * SelectedConnector holds the data fields for a SelectedConnector record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store✂️selectedconnector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/SelectedConnector)
 **/
type SelectedConnector = { piece: Guid; connector: Guid } | undefined;
/**
 * EMPTY_CONNECTOR holds the data fields for a EMPTY_CONNECTOR record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🪨emptyconnector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/EMPTY_CONNECTOR)
 **/
const EMPTY_CONNECTOR: SelectedConnector = undefined;

/**
 * Returns the selected connector for the Design app.
 *
 * MUST return the currently selected connector from the selection state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappselectedconnector](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppSelectedConnector)
 **/
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

/**
 * Returns whether a specific piece port is currently selected.
 *
 * MUST check the selection connector state for the given piece-connector pair.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappispieceportselected](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppIsPiecePortSelected)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️getconnectionstatusfromtransactionstack](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/getConnectionStatusFromTransactionStack)
 * getConnectionStatusFromTransactionStack holds the data fields for a getConnectionStatusFromTransactionStack record.
 **/
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

/**
 * Returns the diff status of a connection for visual indication.
 *
 * MUST return DiffStatus from the design diff for the given connection GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappconnectionstatus](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppConnectionStatus)
 **/
export function useDesignAppConnectionStatus(id: DesignAppId | undefined, connectionId: string): DiffStatus {
  const store = useDesignStore(identitySelector, id) as DesignStore | null;
  return useMemo(() => getConnectionStatusFromTransactionStack(store, connectionId), [store, connectionId]);
}

/**
 * Returns the computed color for a connection based on its status.
 *
 * MUST derive the color from selection, hover, and diff status.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignappconnectioncolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppConnectionColor)
 **/
export function useDesignAppConnectionColor(id: DesignAppId | undefined, connectionId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignAppIsConnectionSelected(id, connectionId);
  const isHovered = useDesignAppIsConnectionHovered(id, connectionId);
  const status = useDesignAppConnectionStatus(id, connectionId);

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  if (status === DiffStatus.Added) {
    stroke = "var(--color-success)";
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

/**
 * Returns the center position of a piece on the canvas.
 *
 * MUST look up the piece metadata for the given GUID and return its center.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignapppiececenter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppPieceCenter)
 **/
export function useDesignAppPieceCenter(id?: DesignAppId, pieceId?: Guid): Coord | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.center : undefined;
}

/**
 * Returns the plane orientation of a piece.
 *
 * MUST look up the piece metadata for the given GUID and return its plane.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖store🛠️usedesignapppieceplane](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Store/d/i/useDesignAppPiecePlane)
 **/
export function useDesignAppPiecePlane(id?: DesignAppId, pieceId?: Guid): Plane | undefined {
  const scope = useDesignAppScope();
  const appId = id ?? (scope ? JSON.parse(scope.id) : undefined);
  const pieceScope = usePieceScope();
  const finalPieceId = pieceId ?? pieceScope?.guid;
  const metadata = usePiecesMetadataMap();
  return finalPieceId ? metadata.get(finalPieceId)?.plane : undefined;
}

// #region Footer

// [👤semio📚js🗃️sketchpad💻designtsx🔖footer](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/FOOTER)
// Footer MUST render dynamic Design app footer items showing selection and transaction state.

/**
 * Footer component that renders dynamic Design app footer status items.
 *
 * MUST register and unregister footer items based on selection and transaction state.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖footer🪨designappfooter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Footer/d/i/DesignAppFooter)
 **/
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

  const designTypeGuidsRef = useRef(designTypeGuids);
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

// #endregion Footer

// #region Filters

// [👤semio📚js🗃️sketchpad💻designtsx🔖filters](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/FILTERS)
// Design filter context MUST provide visibility state for pieces, connections, and ports via URL search params.

type DesignFilterKind = "pieces" | "connections" | "ports";

interface DesignFilterState {
  showPieces: boolean;
  showConnections: boolean;
  showPorts: boolean;
}

const DEFAULT_FILTER_STATE: DesignFilterState = { showPieces: true, showConnections: true, showPorts: true };

const DESIGN_FILTER_STORE_KEY = "__semio_design_filter_store__";
type DesignFilterStore = { state: DesignFilterState; listeners: Set<() => void> };
const getDesignFilterStore = (): DesignFilterStore => {
  const g = globalThis as any;
  if (!g[DESIGN_FILTER_STORE_KEY]) g[DESIGN_FILTER_STORE_KEY] = { state: DEFAULT_FILTER_STATE, listeners: new Set<() => void>() };
  return g[DESIGN_FILTER_STORE_KEY];
};
const setDesignFilterState = (next: DesignFilterState) => {
  const store = getDesignFilterStore();
  store.state = next;
  store.listeners.forEach((l) => l());
};
const subscribeDesignFilter = (listener: () => void) => {
  const store = getDesignFilterStore();
  store.listeners.add(listener);
  return () => { store.listeners.delete(listener); };
};
const getDesignFilterSnapshot = (): DesignFilterState => getDesignFilterStore().state;

const DesignFilterContext = createContext<DesignFilterState>(DEFAULT_FILTER_STATE);

const DesignFilterProvider: FC<{ children: React.ReactNode }> = ({ children }) => {
  const sharedState = useSyncExternalStore(subscribeDesignFilter, getDesignFilterSnapshot);
  return <DesignFilterContext.Provider value={sharedState}>{children}</DesignFilterContext.Provider>;
};

const useDesignFilters = () => useContext(DesignFilterContext);

// #endregion Filters

// #region Tools

// [👤semio📚js🗃️sketchpad💻designtsx🔖tools](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/TOOLS)
// Tools MUST define all Design app tool configurations for selection, lasso, and hand modes.

/**
 * Tool configuration for normal selection mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨selectionnormaltool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/SelectionNormalTool)
 **/
export const SelectionNormalTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_NORMAL,
  icon: <SelectToolIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Tool configuration for additive selection mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨selectionadditivetool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/SelectionAdditiveTool)
 **/
export const SelectionAdditiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_ADDITIVE,
  icon: <AddIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Tool configuration for subtractive selection mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨selectionsubtractivetool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/SelectionSubtractiveTool)
 **/
export const SelectionSubtractiveTool: Tool<DesignAppState> = {
  id: ToolKind.SELECTION_SUBTRACTIVE,
  icon: <RemoveIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Tool configuration for rectangular lasso selection mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨lassorectangulartool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/LassoRectangularTool)
 **/
export const LassoRectangularTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_RECTANGULAR,
  icon: <DiagramIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Tool configuration for freeform lasso selection mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨lassofreeformtool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/LassoFreeformTool)
 **/
export const LassoFreeformTool: Tool<DesignAppState> = {
  id: ToolKind.LASSO_FREEFORM,
  icon: <SceneIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Tool configuration for hand/pan mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨handtool](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/HandTool)
 **/
export const HandTool: Tool<DesignAppState> = {
  id: ToolKind.HAND,
  icon: <HandIcon className="size-tiny" />,
  render: (context: ToolRenderContext<DesignAppState>) => ({}),
};

/**
 * Array of all Design app tool configurations.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨designapptools](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/DesignAppTools)
 **/
export const DesignAppTools: Tool<DesignAppState>[] = [SelectionAdditiveTool, SelectionSubtractiveTool, LassoRectangularTool, LassoFreeformTool, HandTool];

/**
 * Settings component for the selection tool group with additive and subtractive toggles.
 *
 * MUST render toggle buttons for each selection sub-mode.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨designselectsettings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/DesignSelectSettings)
 **/
export const DesignSelectSettings: FC = () => {
  const [activeTool, setActiveTool] = useDesignAppActiveTool();
  const additiveLabel = useLabel("semio.sketchpad.app.design.tools.select.mode.additive");
  const subtractiveLabel = useLabel("semio.sketchpad.app.design.tools.select.mode.subtractive");
  const intersectLabel = useLabel("semio.sketchpad.app.design.tools.select.mode.intersect");
  const rectangularLabel = useLabel("semio.sketchpad.app.design.tools.select.shape.rectangular");
  const lassoLabel = useLabel("semio.sketchpad.app.design.tools.select.shape.lasso");
  const handLabel = useLabel("semio.sketchpad.app.design.tools.select.navigation.hand");

  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <Toggle
        id="semio.sketchpad.app.design.tools.select.mode.additive"
        icon={<AddIcon className="size-tiny" />}
        text={additiveLabel}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_ADDITIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.mode.subtractive"
        icon={<RemoveIcon className="size-tiny" />}
        text={subtractiveLabel}
        pressed={activeTool === ToolKind.SELECTION_SUBTRACTIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_SUBTRACTIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.mode.intersect"
        icon={<IntersectIcon className="size-tiny" />}
        text={intersectLabel}
        pressed={activeTool === ToolKind.SELECTION_INTERSECT}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_INTERSECT : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.shape.rectangular"
        icon={<DiagramIcon className="size-tiny" />}
        text={rectangularLabel}
        pressed={activeTool === ToolKind.LASSO_RECTANGULAR}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.LASSO_RECTANGULAR : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.shape.lasso"
        icon={<SceneIcon className="size-tiny" />}
        text={lassoLabel}
        pressed={activeTool === ToolKind.LASSO_FREEFORM}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.LASSO_FREEFORM : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.design.tools.select.navigation.hand"
        icon={<HandIcon className="size-tiny" />}
        text={handLabel}
        pressed={activeTool === ToolKind.HAND}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.HAND : ToolKind.SELECTION_NORMAL)}
      />
    </div>
  );
};

/**
 * Settings component for the hand tool that activates hand mode.
 *
 * MUST activate the hand tool on mount.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨designhandsettings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/DesignHandSettings)
 **/
export const DesignHandSettings: FC = () => {
  const [activeTool, setActiveTool] = useDesignAppActiveTool();

  useEffect(() => {
    if (activeTool !== ToolKind.HAND && setActiveTool) {
      setActiveTool(ToolKind.HAND);
    }
  }, [setActiveTool]);

  return null;
};

/**
 * Settings component for the lasso tool with rectangular and freeform toggles.
 *
 * MUST render toggle group for lasso sub-modes.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖tools🪨designlassosettings](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Tools/d/i/DesignLassoSettings)
 **/
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
        onValueChange={(vals: string[]) => vals[0] && setActiveTool && setActiveTool(Number(vals[0]) as unknown as ToolKind)}
        kind="single"
      />
    </div>
  );
};

// #endregion Tools

// #region Toolbar

// [👤semio📚js🗃️sketchpad💻designtsx🔖toolbar](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/TOOLBAR)
// Toolbar components MUST provide filter functionality for the Design app.

/**
 * Filter toolbar component for the Design app with toggles for pieces, connections, and ports visibility.
 *
 * MUST render toggle buttons to filter design elements. MUST use URL state for filter persistence.
 *
 *  * [👤semio📚js🗃️sketchpad💻designtsx🔖toolbar🪨designtoolbarfilters](semiorepo://definition/SEMIO/JS/SKETCHPAD/DESIGN.TSX/TOOLBAR/DESIGN-TOOLBAR-FILTERS)
 **/
const DesignToolbarFilters: FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const selectedFiltersFromUrl = useMemo(() => searchParams.getAll("filter") as DesignFilterKind[], [searchParams]);
  const selectedFilters = useMemo(() => new Set(selectedFiltersFromUrl), [selectedFiltersFromUrl]);
  const syncFilterStore = useCallback((params: URLSearchParams) => {
    const kinds = params.getAll("filter") as DesignFilterKind[];
    if (kinds.length === 0) { setDesignFilterState(DEFAULT_FILTER_STATE); return; }
    setDesignFilterState({ showPieces: kinds.includes("pieces"), showConnections: kinds.includes("connections"), showPorts: kinds.includes("ports") });
  }, []);
  useEffect(() => { syncFilterStore(searchParams); }, [searchParams, syncFilterStore]);
  const toggleFilter = (kind: DesignFilterKind) => {
    const allKinds: DesignFilterKind[] = ["pieces", "connections", "ports"];
    const newParams = new URLSearchParams(searchParams);
    const filters = newParams.getAll("filter") as DesignFilterKind[];
    if (filters.length === 0) {
      newParams.delete("filter");
      allKinds.filter((k) => k !== kind).forEach((k) => newParams.append("filter", k));
    } else if (filters.includes(kind)) {
      const remaining = filters.filter((k) => k !== kind);
      newParams.delete("filter");
      remaining.forEach((k) => newParams.append("filter", k));
    } else {
      const updated = [...filters, kind];
      newParams.delete("filter");
      if (updated.length < allKinds.length) {
        updated.forEach((k) => newParams.append("filter", k));
      }
    }
    const nextKinds = newParams.getAll("filter") as DesignFilterKind[];
    if (nextKinds.length === 0) { setDesignFilterState(DEFAULT_FILTER_STATE); }
    else { setDesignFilterState({ showPieces: nextKinds.includes("pieces"), showConnections: nextKinds.includes("connections"), showPorts: nextKinds.includes("ports") }); }
    setSearchParams(newParams);
  };
  const isActive = (kind: DesignFilterKind) => selectedFiltersFromUrl.length === 0 || selectedFilters.has(kind);
  const labelPieces = useLabel("semio.sketchpad.app.design.toolbar.showPieces");
  const labelConnections = useLabel("semio.sketchpad.app.design.toolbar.showConnections");
  const labelPorts = useLabel("semio.sketchpad.app.design.toolbar.showPorts");
  return (
    <ToolbarGroup>
      <Toggle pressed={isActive("pieces")} onPressedChange={() => toggleFilter("pieces")} id="semio.sketchpad.app.design.toolbar.showPieces" icon={<PieceIcon className="size-tiny" />} text={labelPieces} />
      <Toggle pressed={isActive("connections")} onPressedChange={() => toggleFilter("connections")} id="semio.sketchpad.app.design.toolbar.showConnections" icon={<ConnectionIcon className="size-tiny" />} text={labelConnections} />
      <Toggle pressed={isActive("ports")} onPressedChange={() => toggleFilter("ports")} id="semio.sketchpad.app.design.toolbar.showPorts" icon={<PortIcon className="size-tiny" />} text={labelPorts} />
    </ToolbarGroup>
  );
};



// #endregion Toolbar

// #endregion Tools

// #region Panels

// [🔖semio/js/sketchpad/Design.tsx#Panels](semiorepo://section/semio/js/sketchpad/Design.tsx/PANELS)

// #region 🔖WindowLibrary
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels)
// WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windowlibrary✂️windowtemplate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/WindowLibrary/d/i/WindowTemplate)
 * WindowTemplate holds the data fields for a WindowTemplate record.
 **/
interface WindowTemplate {
  id: string;
  label: string;
  icon: React.ReactNode;
  windowTypeId: string;
  componentProps?: any;
}

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windowlibrary🪨windowtemplates](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/WindowLibrary/d/i/windowTemplates)
 * windowTemplates holds the data fields for a windowTemplates record.
 **/
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
    icon: <TableViewIcon size={16} />,
    windowTypeId: "table",
    componentProps: { dataType: "connections" },
  },
];

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windowlibrary✂️draggablewindowitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/WindowLibrary/d/i/DraggableWindowItem)
 * DraggableWindowItemProps holds the data fields for a DraggableWindowItemProps record.
 **/
interface DraggableWindowItemProps {
  template: WindowTemplate;
}

// [👤semio📚js🗃️sketchpad💻design🔖panels🔖windowlibrary🪨draggablewindowitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/WindowLibrary/d/i/DraggableWindowItem)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖panels🔖windowlibrary🪨draggablewindowitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/WindowLibrary/d/i/DraggableWindowItem)
 * DraggableWindowItem holds the data fields for a DraggableWindowItem record.
 **/
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
      <TreeRow>
          <div className="flex items-center gap-single">
            {template.icon}
            <span className="text-sm">{template.label}</span>
          </div>
        </TreeRow>
    </div>
  );
};

/**
 * Panel component that renders the draggable window template library.
 *
 * MUST render categorized window templates for scene, diagram, and table types.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windowlibrary🪨windowlibrary](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/WindowLibrary/d/i/WindowLibrary)
 **/
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

// [👤semio📚js🗃️sketchpad💻designtsx🔖panels🔖details](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/PANELS/DETAILS)
// Details MUST render the Design app detail panels for design, pieces, connections, and connector sections.

/**
 * Detail section component for the currently open design.
 *
 * MUST render the design form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨designsection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/DesignSection)
 **/
export const DesignSection: FC = () => {
  return <DesignSectionForm />;
};

// [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨designsectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/DesignSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨designsectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/DesignSectionForm)
 * DesignSectionForm holds the data fields for a DesignSectionForm record.
 **/
const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const location = useLocation();
  const [transaction] = useDesignAppTransaction();
  const kitCommands = useKitCommands();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const pathScope = useMemo(() => {
    const match = location.pathname.match(/^\/kits\/([^/?]+)(?:\/designs\/([^/?]+))?/);
    return {
      kitGuid: match?.[1],
      designGuid: match?.[2],
    };
  }, [location.pathname]);
  const scopedKitGuid = kitScope?.guid ?? pathScope.kitGuid;
  const scopedDesignGuid = designScope?.guid ?? pathScope.designGuid;
  const kit = useKit(identitySelector, scopedKitGuid as Guid | undefined, true) as Kit | null;
  const kitDesigns = useKitDesigns(scopedKitGuid as Guid | undefined);
  const designFromScope = useDesign() as Design | null;
  const design = useMemo(() => {
    if (designFromScope) return designFromScope;
    if (scopedDesignGuid) {
      const designFromKitList = kitDesigns.find((entry) => entry.guid === scopedDesignGuid);
      if (designFromKitList) return designFromKitList;
    }
    if (!kit || !scopedDesignGuid) return null;
    return kit.designs?.find((entry) => entry.guid === scopedDesignGuid) ?? null;
  }, [designFromScope, kitDesigns, kit, scopedDesignGuid]);

  const authorLabel = useLabel("semio.sketchpad.app.design.author");
  const attributeLabel = useLabel("semio.sketchpad.app.design.attribute");

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
      <TreeRow>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.name" value={design.name} onLazyChange={(value) => updateDesignField({ name: value })} showLabel />
        </TreeRow>
      <TreeRow>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.description"
            value={design.description || ""}
            placeholderId="semio.sketchpad.app.design.descriptionPlaceholder"
            onLazyChange={(value) => updateDesignField({ description: value })}
            showLabel
          />
        </TreeRow>
      <TreeRow>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.icon" value={design.icon || ""} placeholderId="semio.sketchpad.app.design.iconPlaceholder" onLazyChange={(value) => updateDesignField({ icon: value })} showLabel />
        </TreeRow>
      <TreeRow>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.image" value={design.image || ""} placeholderId="semio.sketchpad.app.design.imagePlaceholder" onLazyChange={(value) => updateDesignField({ image: value })} showLabel />
        </TreeRow>
      <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.variant"
            value={(design as any).variant || ""}
            placeholderId="semio.sketchpad.app.design.variantPlaceholder"
            onLazyChange={(value) => updateDesignField({ variant: value })}
            showLabel
          />
        </TreeRow>
      <TreeRow>
          <Input
            lazy
            id="semio.sketchpad.app.design.panel.details.section.design.view"
            value={(design as any).view || ""}
            placeholderId="semio.sketchpad.app.design.viewPlaceholder"
            onLazyChange={(value) => updateDesignField({ view: value })}
            showLabel
          />
        </TreeRow>
      <TreeRow>
          <Input lazy id="semio.sketchpad.app.design.panel.details.section.design.unit" value={design.unit || ""} onLazyChange={(value) => updateDesignField({ unit: value })} showLabel />
        </TreeRow>
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
          <TreeRow>
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
            </TreeRow>
          <TreeRow>
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
            </TreeRow>
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
                label={author.name || `${authorLabel} ${index + 1}`}
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
                <TreeRow>
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
                  </TreeRow>
                <TreeRow>
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
                  </TreeRow>
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
                label={attribute.key || `${attributeLabel} ${index + 1}`}
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
                <TreeRow>
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
                  </TreeRow>
                <TreeRow>
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
                  </TreeRow>
                <TreeRow>
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
                  </TreeRow>
                <TreeRow>
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
                  </TreeRow>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      {design.createdAt && (
        <TreeRow>
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
          </TreeRow>
      )}
      {design.updatedAt && (
        <TreeRow>
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
          </TreeRow>
      )}
      {design.pieces && (
        <TreeRow>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.pieceCount"
              value={`${(design.pieces || []).length}`}
              disabled
              showLabel
            />
          </TreeRow>
      )}
      {design.connections && (
        <TreeRow>
            <Input
              id="semio.sketchpad.app.design.panel.details.section.design.connectionCount"
              value={`${(design.connections || []).length}`}
              disabled
              showLabel
            />
          </TreeRow>
      )}
    </>
  );
};

/**
 * Detail section component for the design pieces list.
 *
 * MUST render each piece with its type, name, and selection interactions.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨piecessection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/PiecesSection)
 **/
export const PiecesSection: FC = () => {
  return <PiecesSectionForm />;
};

// [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨piecessectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/PiecesSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨piecessectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/PiecesSectionForm)
 * PiecesSectionForm holds the data fields for a PiecesSectionForm record.
 **/
const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const [transaction] = useDesignAppTransaction();
  const [updatePiece] = useDesignAppUpdatePiece();
  const [updatePieces] = useDesignAppUpdatePieces();
  const design = useDesign() as Design;
  const kit = useKit(undefined, undefined, true) as Kit;
  const kitCommands = useKitCommands();
  const includedDesigns = useIncludedDesigns();
  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((includedDesign) => [includedDesign.guid, includedDesign])), [includedDesigns]);
  const metadata = usePiecesMetadataMap();
  const allConnections = useConnections();
  const [selection] = useDesignAppSelection();
  const knownSelectablePieceIds = useMemo(() => [...new Set([...(design?.pieces || []).map((entry) => entry.guid), ...includedDesigns.map((entry) => entry.guid)])], [design?.pieces, includedDesigns]);
  const selectedPieceIds = useMemo(
    () =>
      (selection.pieces || [])
        .map((entry) => resolveSelectionEntryGuidByKnownIds(entry, knownSelectablePieceIds))
        .filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0),
    [selection.pieces, knownSelectablePieceIds],
  );
  const pieceGuidSet = useMemo(() => new Set((design?.pieces || []).map((entry) => entry.guid)), [design?.pieces]);
  const validSelectedPieceIds = useMemo(() => selectedPieceIds.filter((guid) => pieceGuidSet.has(guid) || includedDesignMap.has(guid)), [selectedPieceIds, pieceGuidSet, includedDesignMap]);
  const fallbackKnownSelectedPieceIds = useMemo(() => {
    if (validSelectedPieceIds.length > 0) return validSelectedPieceIds;
    const rawSelectionPieces = JSON.stringify(selection.pieces || []);
    return knownSelectablePieceIds.filter((knownId) => rawSelectionPieces.includes(knownId));
  }, [validSelectedPieceIds, selection.pieces, knownSelectablePieceIds]);
  const selectedPiecesRaw = usePiecesFromIds(fallbackKnownSelectedPieceIds);
  const directPiecesMap = useMemo(() => new Map((design?.pieces || []).map((entry) => [entry.guid, entry])), [design?.pieces]);
  const pieces = useMemo(
    () =>
      selectedPiecesRaw.map((entry, index) => {
        if ((entry as any)?.type?.name !== "unknown") return entry;
        const pieceGuid = fallbackKnownSelectedPieceIds[index];
        if (!pieceGuid) return entry;
        const directPiece = directPiecesMap.get(pieceGuid);
        if (directPiece) {
          return {
            ...directPiece,
            id_: directPiece.guid,
          };
        }
        const includedDesign = includedDesignMap.get(pieceGuid);
        if (includedDesign) {
          return {
            id_: pieceGuid,
            type: {
              name: "design",
              variant: includedDesign.designGuid,
            },
            center: includedDesign.center,
            plane: includedDesign.plane,
            description: `${includedDesign.type === "fixed" ? "Fixed" : "Clustered"} design`,
          };
        }
        return entry;
      }),
    [selectedPiecesRaw, fallbackKnownSelectedPieceIds, directPiecesMap, includedDesignMap],
  );

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
  const resolveTypeInKit = (entry: any): Type | null => {
    if (!entry?.type) return null;
    if (typeof entry.type === "string") {
      try {
        return findTypeInKit(kit, entry.type);
      } catch (_error) {
        return null;
      }
    }
    if (typeof entry.type === "object" && "guid" in entry.type && entry.type.guid) {
      try {
        return findTypeInKit(kit, entry.type.guid);
      } catch (_error) {
        return null;
      }
    }
    if (typeof entry.type === "object" && entry.type.name) {
      const matchByNameVariant = (kit.types || []).find((type) => {
        const typeVariant = (type as any).variant || "";
        const entryVariant = (entry.type as any).variant || "";
        return type.name === entry.type.name && typeVariant === entryVariant;
      });
      return matchByNameVariant || null;
    }
    return null;
  };
  const parseDesignVariant = (variant: string) => {
    const [name, variantPart, viewPart] = variant.split("-");
    return { name, variant: variantPart || undefined, view: viewPart || undefined };
  };
  const buildDesignVariant = (name: string, variant?: string, view?: string) => {
    const parts = [name, variant, view].filter((part) => part && part.length > 0) as string[];
    return parts.join("-");
  };
  const resolveDesignInKitSafe = (designGuid?: string): Design | null => {
    if (!designGuid) return null;
    try {
      return findDesignInKit(kit, designGuid);
    } catch (_error) {
      return null;
    }
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
      const currentType = piece.type && typeof piece.type === "string" ? findTypeInKit(kit, piece.type) : (piece.type && typeof piece.type === "object" && "guid" in piece.type) ? findTypeInKit(kit, piece.type.guid) : null;
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
        const currentType = p.type && typeof p.type === "string" ? findTypeInKit(kit, p.type) : (p.type && typeof p.type === "object" && "guid" in p.type) ? findTypeInKit(kit, p.type.guid) : null;
        if (!currentType) return null;
        const match = resolveType(currentType.name, variantValue);
        if (!match) return null;
        return { id: getPieceId(p), diff: { type: { guid: match.guid } } };
      })
      .filter((update): update is { id: Guid; diff: { type: { guid: string } } } => update !== null && update.diff.type !== undefined);

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates);
    transaction?.finalize();
  };

  const handleDesignNameChange = (value: string) => {
    if (!isDesignPiece || !value) return;
    const updateDesignGuid = (targetPiece: any, name: string) => {
      const currentDesign = ("design" in targetPiece && targetPiece.design?.guid) ? resolveDesignInKitSafe(targetPiece.design.guid) : null;
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
      if ("design" in piece && piece.design?.guid) {
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
        if ("design" in p && p.design?.guid) {
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
      .filter((update) => update !== null) as { id: Guid; diff: Partial<PieceDiff> }[];

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates as { id: Guid; diff: PieceDiff }[]);
    transaction?.finalize();
  };

  const handleDesignVariantChange = (value: string) => {
    if (!isDesignPiece) return;
    const nextVariant = value || undefined;
    const updateDesignGuid = (targetPiece: any, variant?: string) => {
      const currentDesign = ("design" in targetPiece && targetPiece.design?.guid) ? resolveDesignInKitSafe(targetPiece.design.guid) : null;
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
      if ("design" in piece && piece.design?.guid) {
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
        if ("design" in p && p.design?.guid) {
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
      .filter((update) => update !== null) as { id: Guid; diff: Partial<PieceDiff> }[];

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates as { id: Guid; diff: PieceDiff }[]);
    transaction?.finalize();
  };

  const handleDesignViewChange = (value: string) => {
    if (!isDesignPiece) return;
    const nextView = value || undefined;
    const updateDesignGuid = (targetPiece: any, view?: string) => {
      const currentDesign = ("design" in targetPiece && targetPiece.design?.guid) ? resolveDesignInKitSafe(targetPiece.design.guid) : null;
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
      if ("design" in piece && piece.design?.guid) {
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
        if ("design" in p && p.design?.guid) {
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
      .filter((update) => update !== null) as { id: Guid; diff: Partial<PieceDiff> }[];

    if (updates.length === 0) return;
    transaction?.start();
    updatePieces?.(updates as { id: Guid; diff: PieceDiff }[]);
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

  const commonTypeName = getCommonValue((p) => resolveTypeInKit(p)?.name);
  const commonTypeVariant = getCommonValue((p) => (resolveTypeInKit(p) as any)?.variant);
  const commonName = getCommonValue((p) => p.name);
  const commonDescription = getCommonValue((p) => p.description);
  const commonScale = getCommonValue((p) => p.scale);
  const commonColor = getCommonValue((p) => p.color);
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

  const hasCenter = pieces.some((p) => p.center);
  const hasPlane = pieces.some((p) => p.plane);
  const hasVariant = pieces.some((p) => (resolveTypeInKit(p) as any)?.variant);
  const hasUnfixedPieces = pieces.some((p) => !p.plane || !p.center);

  const handleNameChange = (value: string) => {
    if (isSingle && piece && isRealPiece(piece)) {
      transaction?.start();
      updatePiece?.(getPieceId(piece), { name: value });
      transaction?.finalize();
    } else {
      const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { name: value } }));
      if (updates.length === 0) return;
      transaction?.start();
      updatePieces?.(updates);
      transaction?.finalize();
    }
  };

  const handleDescriptionChange = (value: string) => {
    if (isSingle && piece && isRealPiece(piece)) {
      transaction?.start();
      updatePiece?.(getPieceId(piece), { description: value });
      transaction?.finalize();
    } else {
      const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { description: value } }));
      if (updates.length === 0) return;
      transaction?.start();
      updatePieces?.(updates);
      transaction?.finalize();
    }
  };

  const handleScaleChange = (value: number) => {
    if (isSingle && piece && isRealPiece(piece)) {
      updatePiece?.(getPieceId(piece), { scale: value });
    } else {
      const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { scale: value } }));
      if (updates.length > 0) updatePieces?.(updates);
    }
  };

  const handleColorChange = (value: string) => {
    if (isSingle && piece && isRealPiece(piece)) {
      transaction?.start();
      updatePiece?.(getPieceId(piece), { color: value || undefined });
      transaction?.finalize();
    } else {
      const updates = pieces.filter(isRealPiece).map((p) => ({ id: getPieceId(p), diff: { color: value || undefined } }));
      if (updates.length === 0) return;
      transaction?.start();
      updatePieces?.(updates);
      transaction?.finalize();
    }
  };

  const handleAttributeAdd = () => {
    if (!isSingle || !piece || !isRealPiece(piece)) return;
    transaction?.start();
    updatePiece?.(getPieceId(piece), { attributes: { added: [{ guid: guid(), key: "" }], removed: [], updated: [] } });
    transaction?.finalize();
  };

  const handleAttributeRemove = (attributeGuid: string) => {
    if (!isSingle || !piece || !isRealPiece(piece)) return;
    transaction?.start();
    updatePiece?.(getPieceId(piece), { attributes: { added: [], removed: [{ guid: attributeGuid }], updated: [] } });
    transaction?.finalize();
  };

  const handleAttributeUpdate = (attributeGuid: string, field: string, value: string) => {
    if (!isSingle || !piece || !isRealPiece(piece)) return;
    updatePiece?.(getPieceId(piece), { attributes: { added: [], removed: [], updated: [{ attribute: { guid: attributeGuid }, diff: { [field]: value } }] } });
  };

  const pieceIds = useMemo(() => pieces.map((p) => getPieceId(p)), [pieces]);

  const selectedVariants = useMemo(
    () => [
      ...new Set(
        pieces
          .map((p) => {
            const type = resolveTypeInKit(p);
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

  const pieceType = piece?.type && "guid" in piece.type ? resolveTypeInKit(piece) : null;
  const pieceDesign = piece && (piece as any).design && "guid" in (piece as any).design ? resolveDesignInKitSafe((piece as any).design.guid) : null;

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

  const hasNoValidPieces = pieces.length === 0 || pieces.every((p) => (p as any)?.type?.name === "unknown");

  const findParentConnectionForDesignPiece = (pieceGuid: string): Connection | null => {
    const includedDesign = includedDesignMap.get(pieceGuid);
    if (!includedDesign) return null;
    const pieceMeta = metadata.get(pieceGuid);
    if (!pieceMeta?.parentPieceId) return null;
    return (
      allConnections.find((connection) => {
        const parentGuid = pieceMeta.parentPieceId as Guid;
        const includedDesignGuid = includedDesign.designGuid as Guid;
        const isParentConnecting = connection.connecting.piece.guid === parentGuid && connection.connected.designPiece?.guid === includedDesignGuid;
        const isParentConnected = connection.connected.piece.guid === parentGuid && connection.connecting.designPiece?.guid === includedDesignGuid;
        return isParentConnecting || isParentConnected;
      }) || null
    );
  };

  let parentConnection: Connection | null = null;
  let parentConnections: Connection[] = [];
  if (isSingle && piece) {
    const pieceId = getPieceId(piece);
    const pieceMeta = metadata.get(pieceId);
    if (pieceMeta?.parentPieceId) {
      parentConnection = allConnections.find((connection) =>
        (connection.connected.piece.guid === pieceId && connection.connecting.piece.guid === pieceMeta.parentPieceId) ||
        (connection.connecting.piece.guid === pieceId && connection.connected.piece.guid === pieceMeta.parentPieceId)
      ) || null;
    }
    if (isDesignPiece) {
      const includedParentConnection = findParentConnectionForDesignPiece(pieceId);
      if (includedParentConnection) {
        parentConnection = includedParentConnection;
      }
    }
  } else if (!isSingle) {
    parentConnections = pieces
      .map((targetPiece) => {
        const pieceId = getPieceId(targetPiece);
        const pieceMeta = metadata.get(pieceId);
        if (pieceMeta?.parentPieceId) {
          return allConnections.find((connection) =>
            (connection.connected.piece.guid === pieceId && connection.connecting.piece.guid === pieceMeta.parentPieceId) ||
            (connection.connecting.piece.guid === pieceId && connection.connected.piece.guid === pieceMeta.parentPieceId)
          ) || null;
        }
        if (isDesignPieceEntry(targetPiece)) {
          return findParentConnectionForDesignPiece(pieceId);
        }
        return null;
      })
      .filter((connection): connection is Connection => connection !== null);
  }

  const mixedSelectionMessageLabel = useLabel("semio.sketchpad.app.design.piece.mixedSelectionMessage");
  const mixedValuesLabel = useLabel("semio.sketchpad.common.mixedValues");
  const selectTypeLabel = useLabel("semio.sketchpad.common.selectType");
  const selectVariantLabel = useLabel("semio.sketchpad.common.selectVariant");
  const pieceAttributeLabel = useLabel("semio.sketchpad.app.design.panel.details.section.piece.attribute");
  const connectedPieceInfoLabel = useLabel("semio.sketchpad.app.design.piece.connectedPieceInfo");
  const fixPieceLabel = useLabel("semio.sketchpad.app.design.piece.fixPiece");

  return (
    <>
      {hasNoValidPieces ? (
        <TreeRow>
            <p className="text-sm text-muted-foreground">No valid pieces found in selection.</p>
          </TreeRow>
      ) : null}
      {!hasNoValidPieces && hasMixedTypes ? (
        <TreeRow>
            <p className="text-sm text-muted-foreground">{mixedSelectionMessageLabel}</p>
          </TreeRow>
      ) : !hasNoValidPieces ? (
        <TreeItem id="semio.sketchpad.app.design.panel.details.section.piece.pieceInfo" defaultOpen={true}>
          {isDesignPiece ? (
            <>
              <TreeRow>
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
                </TreeRow>
              {availableDesignVariants.length > 0 && (
                <TreeRow>
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
                  </TreeRow>
              )}
              {availableDesignViews.length > 0 && (
                <TreeRow>
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
                  </TreeRow>
              )}
            </>
          ) : (
            <>
              <TreeRow>
                  <Combobox
                    id="semio.sketchpad.app.design.piece.type"
                    options={availableTypeNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={isSingle && piece && piece.type && "guid" in piece.type ? findTypeInKit(kit, piece.type.guid)?.name || "" : commonTypeName || ""}
                    placeholder={!isSingle && commonTypeName === undefined ? mixedValuesLabel : selectTypeLabel}
                    onValueChange={handleTypeNameChange}
                    showLabel
                  />
                </TreeRow>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeRow>
                    <Combobox
                      id="semio.sketchpad.app.type.variant"
                      options={availableVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={isSingle && piece && piece.type && "guid" in piece.type ? (findTypeInKit(kit, piece.type.guid) as any)?.variant || "" : commonTypeVariant || ""}
                      placeholder={!isSingle && commonTypeVariant === undefined ? mixedValuesLabel : selectVariantLabel}
                      onValueChange={handleTypeVariantChange}
                      allowClear={true}
                      showLabel
                    />
                  </TreeRow>
              )}
            </>
          )}
          {isSingle && piece && (
            <TreeRow>
                <Input id="semio.sketchpad.app.design.piece.id" value={getPieceId(piece)} disabled showLabel />
              </TreeRow>
          )}
          <TreeRow>
              <Input
                lazy
                id="semio.sketchpad.app.design.panel.details.section.piece.name"
                value={isSingle && piece ? (piece as any).name || "" : commonName || ""}
                placeholderId="semio.sketchpad.app.design.panel.details.section.piece.namePlaceholder"
                onLazyChange={handleNameChange}
                showLabel
              />
            </TreeRow>
          <TreeRow>
              <Textarea
                lazy
                id="semio.sketchpad.app.design.panel.details.section.piece.description"
                value={isSingle && piece ? piece.description || "" : commonDescription || ""}
                placeholderId="semio.sketchpad.app.design.panel.details.section.piece.descriptionPlaceholder"
                onLazyChange={handleDescriptionChange}
                showLabel
              />
            </TreeRow>
          <TreeItem
            id="semio.sketchpad.app.design.panel.details.section.piece.attributes"
            actions={[
              {
                icon: <AddIcon />,
                onClick: handleAttributeAdd,
                id: "semio.sketchpad.common.add",
              },
            ]}
          >
            {isSingle && piece && (piece as any).attributes && (piece as any).attributes.length > 0 && (
              <SortableTreeItems
                items={((piece as any).attributes || []).map((attribute: any, index: number) => ({
                  ...attribute,
                  id: `piece-attribute-${attribute.guid || index}`,
                  index,
                }))}
                onReorder={() => {}}
              >
                {(attribute, index) => (
                  <TreeItem
                    key={`piece-attribute-${attribute.guid || index}`}
                    label={attribute.key || `${pieceAttributeLabel} ${index + 1}`}
                    sortable={true}
                    sortableId={`piece-attribute-${attribute.guid || index}`}
                    isDragHandle={true}
                    actions={[
                      {
                        icon: <RemoveIcon />,
                        onClick: () => handleAttributeRemove(attribute.guid),
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeRow>
                        <Input
                          lazy
                          id="semio.sketchpad.app.design.panel.details.section.piece.attributes.name"
                          value={attribute.key || ""}
                          onLazyChange={(value) => handleAttributeUpdate(attribute.guid, "key", value)}
                          showLabel
                        />
                      </TreeRow>
                    <TreeRow>
                        <Input
                          lazy
                          id="semio.sketchpad.app.design.panel.details.section.piece.attributes.value"
                          value={attribute.value || ""}
                          onLazyChange={(value) => handleAttributeUpdate(attribute.guid, "value", value)}
                          showLabel
                        />
                      </TreeRow>
                    <TreeRow>
                        <Input
                          lazy
                          id="semio.sketchpad.app.design.panel.details.section.piece.attributes.unit"
                          value={attribute.unit || ""}
                          onLazyChange={(value) => handleAttributeUpdate(attribute.guid, "unit", value)}
                          showLabel
                        />
                      </TreeRow>
                    <TreeRow>
                        <Input
                          lazy
                          id="semio.sketchpad.app.design.panel.details.section.piece.attributes.definition"
                          value={attribute.definition || ""}
                          onLazyChange={(value) => handleAttributeUpdate(attribute.guid, "definition", value)}
                          showLabel
                        />
                      </TreeRow>
                  </TreeItem>
                )}
              </SortableTreeItems>
            )}
          </TreeItem>
          <TreeRow>
              <Stepper
                id="semio.sketchpad.app.design.panel.details.section.piece.scale"
                value={isSingle && piece ? (piece as any).scale ?? 1 : commonScale ?? 1}
                onChange={handleScaleChange}
                step={0.01}
              />
            </TreeRow>
          <TreeRow>
              <Input
                lazy
                id="semio.sketchpad.app.design.panel.details.section.piece.color"
                value={isSingle && piece ? (piece as any).color || "" : commonColor || ""}
                placeholderId="semio.sketchpad.app.design.panel.details.section.piece.colorPlaceholder"
                onLazyChange={handleColorChange}
                showLabel
              />
            </TreeRow>
          {hasCenter && (
            <TreeItem id="semio.sketchpad.app.design.piece.center">
              <TreeRow>
                  <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.center.x" value={isSingle && piece ? piece.center?.u : commonCenterX} onChange={handleCenterXChange} step={0.1} />
                </TreeRow>
              <TreeRow>
                  <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.center.y" value={isSingle && piece ? piece.center?.v : commonCenterY} onChange={handleCenterYChange} step={0.1} />
                </TreeRow>
            </TreeItem>
          )}
          {isSingle && piece && !piece.plane && (
            <TreeRow>
                <div className="flex flex-col gap-single">
                  <p className="text-sm text-muted-foreground">{connectedPieceInfoLabel}</p>
                  <Button id="semio.sketchpad.app.design.piece.fixPiece" onClick={fixPieces}>
                    <DisconnectIcon className="size-tiny" />
                    {fixPieceLabel}
                  </Button>
                </div>
              </TreeRow>
          )}
          {hasPlane && (
            <TreeItem id="semio.sketchpad.app.design.piece.plane">
              <TreeItem id="semio.sketchpad.app.design.piece.planeOrigin">
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.x" value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX} onChange={handlePlaneOriginXChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.y" value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY} onChange={handlePlaneOriginYChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.origin.z" value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ} onChange={handlePlaneOriginZChange} step={0.1} />
                  </TreeRow>
              </TreeItem>
              <TreeItem id="semio.sketchpad.app.design.piece.planeXAxis">
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.x" value={isSingle && piece ? piece.plane?.xAxis.x : commonPlaneXAxisX} onChange={handlePlaneXAxisXChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.y" value={isSingle && piece ? piece.plane?.xAxis.y : commonPlaneXAxisY} onChange={handlePlaneXAxisYChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.xaxis.z" value={isSingle && piece ? piece.plane?.xAxis.z : commonPlaneXAxisZ} onChange={handlePlaneXAxisZChange} step={0.1} />
                  </TreeRow>
              </TreeItem>
              <TreeItem id="semio.sketchpad.app.design.piece.planeYAxis">
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.x" value={isSingle && piece ? piece.plane?.yAxis.x : commonPlaneYAxisX} onChange={handlePlaneYAxisXChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.y" value={isSingle && piece ? piece.plane?.yAxis.y : commonPlaneYAxisY} onChange={handlePlaneYAxisYChange} step={0.1} />
                  </TreeRow>
                <TreeRow>
                    <Stepper id="semio.sketchpad.app.design.panel.details.section.piece.plane.yaxis.z" value={isSingle && piece ? piece.plane?.yAxis.z : commonPlaneYAxisZ} onChange={handlePlaneYAxisZChange} step={0.1} />
                  </TreeRow>
              </TreeItem>
            </TreeItem>
          )}
        </TreeItem>
      ) : null}
      {!hasNoValidPieces && parentConnection && (
        <TreeItem id="semio.sketchpad.app.design.panel.details.parentConnection">
          <ConnectionScopeProvider guid={parentConnection.guid}>
            <SingleConnectionInfo />
            <SingleConnectionFields />
          </ConnectionScopeProvider>
        </TreeItem>
      )}
      {!hasNoValidPieces && !isSingle && parentConnections.length > 0 && (
        <TreeItem id="semio.sketchpad.app.design.panel.details.parentConnections">
          <ConnectionsSectionForm connections={parentConnections} sectionLabel={undefined} />
        </TreeItem>
      )}
    </>
  );
};

/**
 * Detail section component for the design connections list.
 *
 * MUST render each connection with its connected pieces and ports.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨singleconnectionfields](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/SingleConnectionFields)
 **/
export const ConnectionsSection: FC<{
  connections: any[];
  isSingle: boolean;
  count: number;
}> = ({ connections, isSingle, count }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <ConnectionsSectionForm connections={connections} sectionLabel={undefined} />;
};

// [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨singleconnectioninfo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/SingleConnectionInfo)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨singleconnectioninfo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/SingleConnectionInfo)
 * SingleConnectionInfo holds the data fields for a SingleConnectionInfo record.
 **/
const SingleConnectionInfo: FC = () => {
  const connection = useConnection() as Connection;
  return (
    <>
      <TreeItem id="semio.sketchpad.app.design.panel.details.section.connection.connecting">
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPieceId" value={connection.connecting.piece.guid} disabled showLabel />
          </TreeRow>
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingPortId" value={connection.connecting.connector?.guid ?? ""} disabled showLabel />
          </TreeRow>
        {connection.connecting.designPiece && (
          <TreeRow>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectingDesignPieceId" value={connection.connecting.designPiece?.guid ?? ""} disabled showLabel />
            </TreeRow>
        )}
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.design.panel.details.section.connection.connected">
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPieceId" value={connection.connected.piece.guid} disabled showLabel />
          </TreeRow>
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedPortId" value={connection.connected.connector?.guid ?? ""} disabled showLabel />
          </TreeRow>
        {connection.connected.designPiece && (
          <TreeRow>
              <Input id="semio.sketchpad.app.design.panel.details.section.connection.connectedDesignPieceId" value={connection.connected.designPiece?.guid ?? ""} disabled showLabel />
            </TreeRow>
        )}
      </TreeItem>
    </>
  );
};

// [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨singleconnectionfields](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/SingleConnectionFields)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖panels🔖details🪨singleconnectionfields](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Panels/s/Details/d/i/SingleConnectionFields)
 * SingleConnectionFields holds the data fields for a SingleConnectionFields record.
 **/
const SingleConnectionFields: FC = () => {
  const [gap, setGap] = useConnectionGap();
  const [shift, setShift] = useConnectionShift();
  const [rise, setRise] = useConnectionRise();
  const [rotation, setRotation] = useConnectionRotation();
  const [turn, setTurn] = useConnectionTurn();
  const [tilt, setTilt] = useConnectionTilt();
  const [u, setU] = useConnectionU();
  const [v, setV] = useConnectionV();
  const [description, setDescription] = useConnectionDescription();
  return (
    <>
      <TreeRow>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.connection.description"
            value={description}
            placeholderId="semio.sketchpad.app.design.panel.details.section.connection.descriptionPlaceholder"
            onLazyChange={setDescription!}
            showLabel
          />
        </TreeRow>
      <TreeItem id="semio.sketchpad.app.design.connection.plane">
        <TreeItem id="semio.sketchpad.app.design.connection.translation" defaultOpen={true}>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.gap" value={[gap]} onValueChange={([value]) => setGap!(value)} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.shift" value={[shift]} onValueChange={([value]) => setShift!(value)} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.rise" value={[rise]} onValueChange={([value]) => setRise!(value)} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
        </TreeItem>
        <TreeItem id="semio.sketchpad.app.design.connection.orientation">
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.rotation" value={[rotation]} onValueChange={([value]) => setRotation!(value)} min={-180} max={180} step={1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.turn" value={[turn]} onValueChange={([value]) => setTurn!(value)} min={-180} max={180} step={1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.tilt" value={[tilt]} onValueChange={([value]) => setTilt!(value)} min={-180} max={180} step={1} showLabel />
            </TreeRow>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.design.connection.diagram">
        <TreeRow>
            <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={u} onChange={setU!} step={0.1} />
          </TreeRow>
        <TreeRow>
            <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.y" value={v} onChange={setV!} step={0.1} />
          </TreeRow>
      </TreeItem>
    </>
  );
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨connectionssectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/ConnectionsSectionForm)
 * ConnectionsSectionForm holds the data fields for a ConnectionsSectionForm record.
 **/
const ConnectionsSectionForm: FC<{
  connections: Connection[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const [transaction] = useDesignAppTransaction();
  const [updateConnections] = useDesignAppUpdateConnections();
  const isSingle = connections.length === 1;
  const connection = isSingle ? connections[0] : null;
  const getCommonValue = <T,>(getter: (currentConnection: Connection) => T | undefined): T | undefined => {
    const values = connections.map(getter).filter((value) => value !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((value) => value === firstValue) ? firstValue : undefined;
  };

  const handleBulkUpdate = (diffFactory: (connection: Connection) => ConnectionDiff) => {
    const updates = connections.map((currentConnection) => ({ id: currentConnection.guid, diff: diffFactory(currentConnection) }));
    if (updates.length === 0) return;
    transaction?.start();
    updateConnections?.(updates);
    transaction?.finalize();
  };

  const commonGap = getCommonValue((currentConnection) => currentConnection.gap);
  const commonShift = getCommonValue((currentConnection) => currentConnection.shift);
  const commonRise = getCommonValue((currentConnection) => currentConnection.rise);
  const commonRotation = getCommonValue((currentConnection) => currentConnection.rotation);
  const commonTurn = getCommonValue((currentConnection) => currentConnection.turn);
  const commonTilt = getCommonValue((currentConnection) => currentConnection.tilt);
  const commonU = getCommonValue((currentConnection) => currentConnection.u);
  const commonV = getCommonValue((currentConnection) => currentConnection.v);
  const commonConnectionDescription = getCommonValue((currentConnection) => currentConnection.description);

  const multipleEditingLabel = useLabel("semio.sketchpad.app.design.panel.details.section.connection.multipleEditing");
  const connectionGapLabel = useLabel("semio.sketchpad.app.design.panel.details.section.connection.gap");
  const connectionShiftLabel = useLabel("semio.sketchpad.app.design.panel.details.section.connection.shift");
  const connectionRiseLabel = useLabel("semio.sketchpad.app.design.panel.details.section.connection.rise");
  const connectionRotationLabel = useLabel("semio.sketchpad.app.design.connection.rotation");
  const connectionTurnLabel = useLabel("semio.sketchpad.app.design.connection.turn");
  const connectionTiltLabel = useLabel("semio.sketchpad.app.design.connection.tilt");

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
      <TreeRow>
          <p className="text-sm text-muted-foreground">{(multipleEditingLabel || "").replace("{{count}}", String(connections.length))}</p>
        </TreeRow>
      <TreeRow>
          <Textarea
            lazy
            id="semio.sketchpad.app.design.panel.details.section.connection.description"
            value={commonConnectionDescription || ""}
            placeholderId="semio.sketchpad.app.design.panel.details.section.connection.descriptionPlaceholder"
            onLazyChange={(value) => handleBulkUpdate(() => ({ description: value || undefined }))}
            showLabel
          />
        </TreeRow>
      <TreeItem id="semio.sketchpad.app.design.connection.plane">
        <TreeItem id="semio.sketchpad.app.design.connection.translation" defaultOpen={true}>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.gap" value={[commonGap ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ gap: value }))} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.shift" value={[commonShift ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ shift: value }))} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.rise" value={[commonRise ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ rise: value }))} min={-100} max={100} step={0.1} showLabel />
            </TreeRow>
        </TreeItem>
        <TreeItem id="semio.sketchpad.app.design.connection.orientation">
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.rotation" value={[commonRotation ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ rotation: value }))} min={-180} max={180} step={1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.turn" value={[commonTurn ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ turn: value }))} min={-180} max={180} step={1} showLabel />
            </TreeRow>
          <TreeRow>
              <Slider id="semio.sketchpad.app.design.panel.details.section.connection.tilt" value={[commonTilt ?? 0]} onValueChange={([value]) => handleBulkUpdate(() => ({ tilt: value }))} min={-180} max={180} step={1} showLabel />
            </TreeRow>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.design.connection.diagram">
        <TreeRow>
            <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={commonU ?? 0} onChange={(value) => handleBulkUpdate(() => ({ u: value }))} step={0.1} />
          </TreeRow>
        <TreeRow>
            <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.y" value={commonV ?? 0} onChange={(value) => handleBulkUpdate(() => ({ v: value }))} step={0.1} />
          </TreeRow>
      </TreeItem>
    </>
  );
};

/**
 * Detail section component for the currently selected connector.
 *
 * MUST render the connector detail form for the selected port.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨connectorsection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/ConnectorSection)
 **/
export const ConnectorSection: FC<{ pieceGuid: Guid; connectorGuid: Guid }> = ({ pieceGuid, connectorGuid }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <ConnectorSectionForm pieceGuid={pieceGuid} connectorGuid={connectorGuid} />;
};
/**
 * ConnectorSectionForm holds the data fields for a ConnectorSectionForm record.
 *
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖details🪨connectorsectionform](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Details/d/i/ConnectorSectionForm)
 **/
const ConnectorSectionForm: FC<{ pieceGuid: Guid; connectorGuid: Guid }> = ({ pieceGuid, connectorGuid }) => {
  const { t } = useTranslation();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;
  const connectorNotFoundLabel = useLabel("semio.sketchpad.app.design.panel.details.section.connector.notFound");
  const yesLabel = useLabel("semio.sketchpad.common.yes");
  const noLabel = useLabel("semio.sketchpad.common.no");

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
      <TreeRow>
          <p className="text-sm text-muted-foreground">{connectorNotFoundLabel}</p>
        </TreeRow>
    );
  }

  return (
    <>
      <TreeRow>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.id" value={connector.guid || "~default~"} disabled showLabel />
        </TreeRow>
      {connector.name && (
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connector.name" value={connector.name} disabled showLabel />
          </TreeRow>
      )}
      <TreeRow>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.t" value={connector.t.toFixed(4)} disabled showLabel />
        </TreeRow>
      {connector.description && (
        <TreeRow>
            <Textarea id="semio.sketchpad.app.design.panel.details.section.connector.description" value={connector.description} disabled showLabel />
          </TreeRow>
      )}
      {connector.port && (
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connector.port" value={connector.port.guid} disabled showLabel />
          </TreeRow>
      )}
      {connector.mandatory !== undefined && (
        <TreeRow>
            <Input id="semio.sketchpad.app.design.panel.details.section.connector.mandatory" value={connector.mandatory ? yesLabel : noLabel} disabled showLabel />
          </TreeRow>
      )}
      <TreeRow>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.position" value={`(${connector.point.x.toFixed(2)}, ${connector.point.y.toFixed(2)}, ${connector.point.z.toFixed(2)})`} disabled showLabel />
        </TreeRow>
      <TreeRow>
          <Input id="semio.sketchpad.app.design.panel.details.section.connector.direction" value={`(${connector.direction.x.toFixed(2)}, ${connector.direction.y.toFixed(2)}, ${connector.direction.z.toFixed(2)})`} disabled showLabel />
        </TreeRow>
      {(connector as any).compatiblePorts &&
        (connector as any).compatiblePorts.map((port_: string, index: number) => (
          <TreeRow key={`compatible-interface-${index}`}>
              <Input id="semio.sketchpad.app.design.panel.details.section.connector.compatiblePort" value={port_} disabled showLabel />
            </TreeRow>
        ))}
      {connector.attributes &&
        connector.attributes.map((attribute: any, index: number) => (
          <TreeRow key={`connector-attribute-${index}`}>
              <Input id="semio.sketchpad.app.design.panel.details.section.connector.attribute" value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled showLabel />
            </TreeRow>
        ))}
    </>
  );
};

// #endregion Details

// #endregion Panels

// #region Canvas

// [🔖semio/js/sketchpad/Design.tsx#Canvas](semiorepo://section/semio/js/sketchpad/Design.tsx/CANVAS)
// #region 🔖Hover Intent Context
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas)
// Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖hoverintentcontext✂️hoverintentcontextvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Hover%20Intent%20Context/d/i/HoverIntentContextValue)
 * HoverIntentContextValue holds the data fields for a HoverIntentContextValue record.
 **/
interface HoverIntentContextValue {
  hoverClearTimeoutRef: React.MutableRefObject<NodeJS.Timeout | null>;
  currentHoveredPieceGuidRef: React.MutableRefObject<string | null>;
  isPanningRef: React.MutableRefObject<boolean>;
  isDraggingNodeRef: React.MutableRefObject<boolean>;
}

/**
 * HoverIntentContext holds the data fields for a HoverIntentContext record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖hoverintentcontext🪨hoverintentprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Hover%20Intent%20Context/d/i/HoverIntentProvider)
 **/
const HoverIntentContext = createContext<HoverIntentContextValue | null>(null);

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖hoverintentcontext🛠️hoverintentprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Hover%20Intent%20Context/d/i/HoverIntentProvider)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖hoverintentcontext🪨hoverintentprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Hover%20Intent%20Context/d/i/HoverIntentProvider)
 * HoverIntentProvider holds the data fields for a HoverIntentProvider record.
 **/
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

/** useHoverIntent holds the data fields for a useHoverIntent record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖hoverintentcontext🛠️usehoverintent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Hover%20Intent%20Context/d/i/useHoverIntent)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖hoverintentcontext🪨usehoverintent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Hover%20Intent%20Context/d/i/useHoverIntent)
 **/
function useHoverIntent(): HoverIntentContextValue {
  const context = useContext(HoverIntentContext);
  if (!context) throw new Error("useHoverIntent must be used within HoverIntentProvider");
  return context;
}

// #endregion Hover Intent Context

/**
 * SemioConnection holds the data fields for a SemioConnection record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas✂️semioconnection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/d/i/SemioConnection)
 **/
type SemioConnection = Connection;

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas✂️piecerenderdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/d/i/PieceRenderData)
 * PieceRenderData holds the data fields for a PieceRenderData record.
 **/
interface PieceRenderData {
  isSelected: boolean;
  isHovered: boolean;
  fill: string;
  stroke: string;
  opacity: number;
  isChangedInTransaction: boolean;
  diffStatus: DiffStatus;
}

/**
 * EMPTY_PIECE_RENDER_DATA holds the data fields for a EMPTY_PIECE_RENDER_DATA record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🪨emptypiecerenderdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/d/i/EMPTY_PIECE_RENDER_DATA)
 **/
const EMPTY_PIECE_RENDER_DATA: PieceRenderData = {
  isSelected: false,
  isHovered: false,
  fill: "transparent",
  stroke: "var(--foreground)",
  opacity: 1,
  isChangedInTransaction: false,
  diffStatus: DiffStatus.Unchanged,
};

type PieceRenderDataStoreApi = {
  data: Map<string, PieceRenderData>;
  pieceVersions: Map<string, number>;
  listeners: Set<() => void>;
  subscribe: (cb: () => void) => () => void;
  get: (guid: string) => PieceRenderData;
  getVersion: (guid: string) => number;
};
function createPieceRenderDataStore(): PieceRenderDataStoreApi {
  const s: PieceRenderDataStoreApi = {
    data: new Map(),
    pieceVersions: new Map(),
    listeners: new Set(),
    subscribe(cb) { s.listeners.add(cb); return () => { s.listeners.delete(cb); }; },
    get(guid) { return s.data.get(guid) ?? EMPTY_PIECE_RENDER_DATA; },
    getVersion(guid) { return s.pieceVersions.get(guid) ?? 0; },
  };
  return s;
}
function updatePieceRenderDataStore(store: PieceRenderDataStoreApi, newData: Map<string, PieceRenderData>) {
  let anyChange = false;
  for (const [guid, entry] of newData) {
    const old = store.data.get(guid);
    if (!old || old.isSelected !== entry.isSelected || old.isHovered !== entry.isHovered || old.fill !== entry.fill || old.stroke !== entry.stroke || old.opacity !== entry.opacity || old.isChangedInTransaction !== entry.isChangedInTransaction || old.diffStatus !== entry.diffStatus) {
      store.pieceVersions.set(guid, (store.pieceVersions.get(guid) ?? 0) + 1);
      anyChange = true;
    }
  }
  for (const guid of store.data.keys()) {
    if (!newData.has(guid)) {
      store.pieceVersions.set(guid, (store.pieceVersions.get(guid) ?? 0) + 1);
      anyChange = true;
    }
  }
  store.data = newData;
  if (anyChange) store.listeners.forEach(cb => cb());
}
const PieceRenderDataStoreContext = createContext<PieceRenderDataStoreApi>(createPieceRenderDataStore());

/** usePieceRenderData holds the data fields for a usePieceRenderData record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🛠️usepiecerenderdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/d/i/usePieceRenderData)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🪨usepiecerenderdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/d/i/usePieceRenderData)
 **/
function usePieceRenderData(pieceGuid: string): PieceRenderData {
  const store = useContext(PieceRenderDataStoreContext);
  const subscribe = useCallback((cb: () => void) => store.subscribe(cb), [store]);
  const lastRef = useRef<{ version: number; data: PieceRenderData }>({ version: -1, data: EMPTY_PIECE_RENDER_DATA });
  const getSnapshot = useCallback(() => {
    const ver = store.getVersion(pieceGuid);
    if (ver === lastRef.current.version) return lastRef.current.data;
    const data = store.get(pieceGuid);
    lastRef.current = { version: ver, data };
    return data;
  }, [store, pieceGuid]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

function syncPieceRenderData(store: PieceRenderDataStoreApi, designStore: DesignStore, hover: DesignAppHover | undefined, selection: DesignAppSelection | undefined) {
  const design = designStore.design().snapshot() as Design | null;
  const hoverData = computeHoverData(designStore, { hover } as DesignAppState);
  const transactionData = getTransactionAffectedPieces(designStore);
  const m = new Map<string, PieceRenderData>();
  if (!design?.pieces) { updatePieceRenderDataStore(store, m); return; }
  const selectedPieces = new Set(selection?.pieces ?? []);
  for (const piece of design.pieces) {
    const pieceGuid = piece.guid;
    const isSelected = selectedPieces.has(pieceGuid);
    const isHovered = hoverData.transitivelyHoveredPieces.has(pieceGuid);
    const diffStatus: DiffStatus = transactionData.statusMap.get(pieceGuid) ?? DiffStatus.Unchanged;
    const isChangedInTransaction = diffStatus !== DiffStatus.Unchanged;
    let fill = "transparent";
    let stroke = "var(--foreground)";
    let opacity = 1;
    if (diffStatus === DiffStatus.Added) { fill = "var(--color-success)"; stroke = "var(--color-success)"; }
    else if (diffStatus === DiffStatus.Removed) { fill = "var(--color-danger)"; stroke = "var(--color-danger)"; opacity = 0.2; }
    else if (diffStatus === DiffStatus.Modified) { fill = "var(--color-warning)"; stroke = "var(--color-warning)"; }
    else if (isChangedInTransaction) { fill = "var(--color-changed-base)"; stroke = "var(--color-changed-base)"; }
    if (isHovered && !isSelected) { fill = "var(--hover-base)"; stroke = "var(--foreground)"; opacity = 1; }
    if (isSelected) {
      const status = diffStatus as string;
      if (isChangedInTransaction) { fill = "var(--color-selected-changed)"; }
      else if (status === "added") { fill = "var(--color-selected-added)"; }
      else if (status === "removed") { fill = "var(--color-selected-removed)"; }
      else if (status === "modified") { fill = "var(--color-selected-changed)"; }
      else { fill = "var(--active-base)"; }
      stroke = "var(--foreground)";
      opacity = 1;
    }
    m.set(pieceGuid, { isSelected, isHovered, fill, stroke, opacity, isChangedInTransaction, diffStatus });
  }
  updatePieceRenderDataStore(store, m);
}

// #region Diagram

// [👤semio📚js🗃️sketchpad💻designtsx🔖canvas🔖diagram](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/CANVAS/DIAGRAM)
// Diagram MUST render the interactive React Flow design diagram with nodes, edges, minimap, and controls.

/**
 * ClusterMenuProps holds the data fields for a ClusterMenuProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️clustermenuprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ClusterMenuProps)
 **/
type ClusterMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onCluster: (clusterPieceIds: string[]) => void;
};

/** ClusterMenu holds the data fields for a ClusterMenu record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨clustermenu](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ClusterMenu)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨clustermenu](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ClusterMenu)
 **/
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

/**
 * ExpandMenuProps holds the data fields for a ExpandMenuProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️expandmenuprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ExpandMenuProps)
 **/
type ExpandMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onExpand: (designId: string) => void;
};

/** ExpandMenu holds the data fields for a ExpandMenu record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨expandmenu](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ExpandMenu)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨expandmenu](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ExpandMenu)
 **/
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

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨presencediagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/PresenceDiagram)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨presencediagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/PresenceDiagram)
 * PresenceDiagram holds the data fields for a PresenceDiagram record.
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️helperline](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/HelperLine)
 * HelperLine holds the data fields for a HelperLine record.
 **/
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
const EMPTY_HELPER_LINES: HelperLine[] = [];

/**
 * PieceNodeProps holds the data fields for a PieceNodeProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️piecenodeprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/PieceNodeProps)
 **/
type PieceNodeProps = {
  piece: Piece;
  type: Type;
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️designnodeprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignNodeProps)
 * DesignNodeProps holds the data fields for a DesignNodeProps record.
 **/
type DesignNodeProps = {
  piece: Piece;
  externalConnections: SemioConnection[];
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️piecenode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/PieceNode)
 * PieceNode holds the data fields for a PieceNode record.
 **/
type PieceNode = Node<PieceNodeProps, "piece">;
/**
 * DesignNode holds the data fields for a DesignNode record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️designnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignNode)
 **/
type DesignNode = Node<DesignNodeProps, "design">;
/**
 * DiagramNode holds the data fields for a DiagramNode record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️diagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DiagramNode)
 **/
type DiagramNode = PieceNode | DesignNode;

/**
 * ConnectionEdge holds the data fields for a ConnectionEdge record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️connectionedge](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ConnectionEdge)
 **/
type ConnectionEdge = Edge<{ SemioConnection: SemioConnection; isParentConnection?: boolean }, "SemioConnection">;
/**
 * DiagramEdge holds the data fields for a DiagramEdge record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️diagramedge](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DiagramEdge)
 **/
type DiagramEdge = ConnectionEdge;

/**
 * ConnectorHandleProps holds the data fields for a ConnectorHandleProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️connectorhandleprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ConnectorHandleProps)
 **/
type ConnectorHandleProps = {
  connector: Connector;
  pieceId: string;
  selected?: boolean;
  onPortClick: (connector: Connector) => void;
};

/** getConnectorPositionStyle holds the data fields for a getConnectorPositionStyle record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨getconnectorpositionstyle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/getConnectorPositionStyle)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨getconnectorpositionstyle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/getConnectorPositionStyle)
 **/
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

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectorhandle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectorHandle)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectorhandle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectorHandle)
 * ConnectorHandle holds the data fields for a ConnectorHandle record.
 **/
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

/**
 * sharedCommandsRef holds the data fields for a sharedCommandsRef record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨sharedcommandsref](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/sharedCommandsRef)
 **/
let sharedCommandsRef: ReturnType<typeof useDesignAppCommands> | null = null;

/** pieceNodeAreEqual holds the data fields for a pieceNodeAreEqual record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨piecenodeareequal](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/pieceNodeAreEqual)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨piecenodeareequal](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/pieceNodeAreEqual)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨piecenodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/PieceNodeComponent)
 * PieceNodeComponent holds the data fields for a PieceNodeComponent record.
 **/
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

/**
 * SelectedConnectorContext holds the data fields for a SelectedConnectorContext record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨selectedconnectorcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/SelectedConnectorContext)
 **/
const SelectedConnectorContext = createContext<DesignAppSelection["connector"] | undefined>(undefined);
/**
 * SelectedConnectorPortContext holds the data fields for a SelectedConnectorPortContext record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨selectedconnectorportcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/SelectedConnectorPortContext)
 **/
const SelectedConnectorPortContext = createContext<string | undefined>(undefined);

/**
 * PieceNodeInnerProps holds the data fields for a PieceNodeInnerProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️piecenodeinnerprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/PieceNodeInnerProps)
 **/
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

/** PieceNodeInner holds the data fields for a PieceNodeInner record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨piecenodeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/PieceNodeInner)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨piecenodeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/PieceNodeInner)
 **/
const PieceNodeInner: React.FC<PieceNodeInnerProps> = ({ id, piece, type, connectors, isSelected, diff, isDesignPiece, selectedConnector, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const renderData = usePieceRenderData(piece.guid);
  const { fill, stroke, opacity: colorOpacity, isHovered } = renderData;
  const { showPorts } = useDesignFilters();

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
      {showPorts && connectors?.map((connector: Connector, connectorIndex: number) => (
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

/**
 * DesignNodeComponent holds the data fields for a DesignNodeComponent record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨designnodecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignNodeComponent)
 **/
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

/**
 * DesignNodeInnerProps holds the data fields for a DesignNodeInnerProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️designnodeinnerprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignNodeInnerProps)
 **/
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

/** DesignNodeInner holds the data fields for a DesignNodeInner record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨designnodeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignNodeInner)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨designnodeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/DesignNodeInner)
 **/
const DesignNodeInner: React.FC<DesignNodeInnerProps> = ({ id, piece, connectors, isSelected, diff, selectedConnector, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const isHovered = useIsPieceHovered();
  const { showPorts } = useDesignFilters();

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
      {showPorts && connectors?.map((connector: Connector, connectorIndex: number) => (
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
/**
 * nodeComponents holds the data fields for a nodeComponents record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨nodecomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/nodeComponents)
 **/
const nodeComponents = { piece: PieceNodeComponent, design: DesignNodeComponent };

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeComponent)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgecomponent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeComponent)
 * ConnectionEdgeComponent holds the data fields for a ConnectionEdgeComponent record.
 **/
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

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgefallback](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeFallback)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgefallback](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeFallback)
 * ConnectionEdgeFallback holds the data fields for a ConnectionEdgeFallback record.
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️connectionedgeinnerprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/ConnectionEdgeInnerProps)
 * ConnectionEdgeInnerProps holds the data fields for a ConnectionEdgeInnerProps record.
 **/
type ConnectionEdgeInnerProps = EdgeProps<ConnectionEdge> & { connectionGuid: Guid };

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeInner)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨connectionedgeinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/ConnectionEdgeInner)
 * ConnectionEdgeInner holds the data fields for a ConnectionEdgeInner record.
 **/
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
/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨edgecomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/edgeComponents)
 * edgeComponents holds the data fields for a edgeComponents record.
 **/
const edgeComponents = { SemioConnection: ConnectionEdgeComponent };

//#region 🔖CustomDesignEdgeLayer
const EMPTY_EDGES_ARRAY: DiagramEdge[] = [];
const EDGE_HANDLE_HEIGHT = 5;
const CustomDesignEdgeLayer: React.FC<{
  edges: DiagramEdge[];
  suppressRecomputeRef?: React.MutableRefObject<boolean>;
  onEdgeClick?: (event: React.MouseEvent, edge: any) => void;
  onEdgeMouseEnter?: (event: React.PointerEvent, edge: any) => void;
  onEdgeMouseLeave?: (event: React.PointerEvent, edge: any) => void;
}> = memo(({ edges, suppressRecomputeRef, onEdgeClick, onEdgeMouseEnter, onEdgeMouseLeave }) => {
  const storeApi = useStoreApi();
  const [tick, setTick] = useState(0);
  const edgesRef = useRef(edges);
  edgesRef.current = edges;
  const pathCacheRef = useRef<Map<string, string>>(new Map());
  const needsRecomputeRef = useRef(false);
  useEffect(() => {
    let rafId: number | null = null;
    const recompute = () => {
      const { nodeLookup } = storeApi.getState();
      const cache = pathCacheRef.current;
      let changed = false;
      for (const edge of edgesRef.current) {
        const src = nodeLookup.get(edge.source);
        const tgt = nodeLookup.get(edge.target);
        if (!src || !tgt) continue;
        const sh = edge.sourceHandle ? src.internals.handleBounds?.source?.find((h: any) => h.id === edge.sourceHandle) : src.internals.handleBounds?.source?.[0];
        const th = edge.targetHandle ? tgt.internals.handleBounds?.target?.find((h: any) => h.id === edge.targetHandle) ?? tgt.internals.handleBounds?.source?.find((h: any) => h.id === edge.targetHandle) : tgt.internals.handleBounds?.target?.[0] ?? tgt.internals.handleBounds?.source?.[0];
        const sx = src.internals.positionAbsolute.x + (sh?.x ?? 0) + (sh?.width ?? 0) / 2;
        const sy = src.internals.positionAbsolute.y + (sh?.y ?? 0) + (sh?.height ?? 0) / 2;
        const tx = tgt.internals.positionAbsolute.x + (th?.x ?? 0) + (th?.width ?? 0) / 2;
        const ty = tgt.internals.positionAbsolute.y + (th?.y ?? 0) + (th?.height ?? 0) / 2;
        const d = `M ${sx} ${sy + EDGE_HANDLE_HEIGHT / 2} L ${tx} ${ty + EDGE_HANDLE_HEIGHT / 2}`;
        if (cache.get(edge.id) !== d) { cache.set(edge.id, d); changed = true; }
      }
      if (changed) setTick(t => t + 1);
    };
    recompute();
    const unsub = storeApi.subscribe(() => {
      if (suppressRecomputeRef?.current) { needsRecomputeRef.current = true; return; }
      if (rafId !== null) return;
      rafId = requestAnimationFrame(() => { rafId = null; recompute(); });
    });
    return () => { unsub(); if (rafId !== null) cancelAnimationFrame(rafId); };
  }, [storeApi, suppressRecomputeRef]);
  useEffect(() => {
    const { nodeLookup } = storeApi.getState();
    const cache = pathCacheRef.current;
    for (const edge of edges) {
      const src = nodeLookup.get(edge.source);
      const tgt = nodeLookup.get(edge.target);
      if (!src || !tgt) continue;
      const sh = edge.sourceHandle ? src.internals.handleBounds?.source?.find((h: any) => h.id === edge.sourceHandle) : src.internals.handleBounds?.source?.[0];
      const th = edge.targetHandle ? tgt.internals.handleBounds?.target?.find((h: any) => h.id === edge.targetHandle) ?? tgt.internals.handleBounds?.source?.find((h: any) => h.id === edge.targetHandle) : tgt.internals.handleBounds?.target?.[0] ?? tgt.internals.handleBounds?.source?.[0];
      const sx = src.internals.positionAbsolute.x + (sh?.x ?? 0) + (sh?.width ?? 0) / 2;
      const sy = src.internals.positionAbsolute.y + (sh?.y ?? 0) + (sh?.height ?? 0) / 2;
      const tx = tgt.internals.positionAbsolute.x + (th?.x ?? 0) + (th?.width ?? 0) / 2;
      const ty = tgt.internals.positionAbsolute.y + (th?.y ?? 0) + (th?.height ?? 0) / 2;
      cache.set(edge.id, `M ${sx} ${sy + EDGE_HANDLE_HEIGHT / 2} L ${tx} ${ty + EDGE_HANDLE_HEIGHT / 2}`);
    }
    setTick(t => t + 1);
  }, [edges, storeApi]);
  const edgeMap = useMemo(() => new Map(edges.map(e => [e.id, e])), [edges]);
  void tick;
  return (
    <svg style={{ position: "absolute", top: 0, left: 0, width: 0, height: 0, overflow: "visible", zIndex: 0, pointerEvents: "none" }}>
      {edges.map(edge => {
        const d = pathCacheRef.current.get(edge.id);
        if (!d) return null;
        const data = edge.data;
        const diff = (data?.SemioConnection?.attributes?.find((q: any) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
        const isParentConnection = data?.isParentConnection ?? false;
        const isSelected = edge.selected ?? false;
        let stroke = "var(--foreground)";
        let strokeWidth = 2;
        let dasharray: string | undefined;
        let opacity = 1;
        if (diff === DiffStatus.Added) { stroke = "var(--color-success)"; dasharray = "5 5"; }
        else if (diff === DiffStatus.Removed) { stroke = "var(--color-danger)"; opacity = 0.25; }
        else if (diff === DiffStatus.Modified) { stroke = "var(--color-warning)"; }
        if (isParentConnection) { stroke = "var(--accent-secondary)"; strokeWidth = 3; }
        if (isSelected) { stroke = "var(--active-base)"; strokeWidth = 3; dasharray = undefined; opacity = 1; }
        return (
          <g key={edge.id} className="react-flow__edge" data-testid={`rf__edge-${edge.id}`}>
            <path d={d} fill="none" stroke={stroke} strokeWidth={strokeWidth} strokeDasharray={dasharray} opacity={opacity} className="transition-colors duration-200" style={{ pointerEvents: "none" }} />
            <path d={d} fill="none" stroke="transparent" strokeWidth={Math.max(strokeWidth, 6)} style={{ pointerEvents: "stroke", cursor: "pointer" }} onClick={(e) => { const ed = edgeMap.get(edge.id); if (ed && onEdgeClick) onEdgeClick(e, ed); }} onPointerEnter={(e) => { const ed = edgeMap.get(edge.id); if (ed && onEdgeMouseEnter) onEdgeMouseEnter(e as any, ed); }} onPointerLeave={(e) => { const ed = edgeMap.get(edge.id); if (ed && onEdgeMouseLeave) onEdgeMouseLeave(e as any, ed); }} />
          </g>
        );
      })}
    </svg>
  );
});
//#endregion 🔖CustomDesignEdgeLayer

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props;
  const HANDLE_HEIGHT = 5;
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`;
  return <BaseEdge path={path} style={{ stroke: "gray" }} className="opacity-70" />;
};

/**
 * Custom minimap node component rendering a colored circle.
 *
 * MUST render a circle at the given position with accent color when selected.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨minimapnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/MiniMapNode)
 **/
export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={`${selected ? "fill-accent" : "fill-foreground"} transition-colors duration-200`} cx={x} cy={y} r="10" />;
};

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨helperlines](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/HelperLines)
 * HelperLines holds the data fields for a HelperLines record.
 **/
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

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨piecetonode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/pieceToNode)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨piecetonode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/pieceToNode)
 * pieceToNode holds the data fields for a pieceToNode record.
 **/
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

/** designToNode holds the data fields for a designToNode record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨designtonode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/designToNode)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨designtonode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/designToNode)
 **/
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

/** extractPieceIdFromNodeId holds the data fields for a extractPieceIdFromNodeId record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨extractpieceidfromnodeid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/extractPieceIdFromNodeId)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨extractpieceidfromnodeid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/extractPieceIdFromNodeId)
 **/
const extractPieceIdFromNodeId = (nodeId: string): { guid: string } => {
  return { guid: nodeId.split("-").slice(2).join("-") };
};

/** getPieceIdFromNode holds the data fields for a getPieceIdFromNode record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨getpieceidfromnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/getPieceIdFromNode)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨getpieceidfromnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/getPieceIdFromNode)
 **/
const getPieceIdFromNode = (node: DiagramNode): string => {
  return node.data.piece.guid;
};

const GUID_PATTERN = /[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}/i;

const resolveSelectionEntryGuid = (entry: any): Guid | undefined => {
  const guidFromString = (value: string): Guid | undefined => {
    if (value.length === 0) return undefined;
    if (value.startsWith("piece-") || value.startsWith("connection-")) {
      const extractedFromNodeId = value.split("-").slice(2).join("-");
      if (extractedFromNodeId.length > 0) return extractedFromNodeId as Guid;
    }
    const guidLikeMatch = value.match(GUID_PATTERN);
    if (guidLikeMatch?.[0]) return guidLikeMatch[0] as Guid;
    return value as Guid;
  };
  const guidFromNestedObject = (value: any, depth: number = 0): Guid | undefined => {
    if (depth > 5 || value === null || value === undefined) return undefined;
    if (typeof value === "string") return guidFromString(value);
    if (Array.isArray(value)) {
      for (const item of value) {
        const result = guidFromNestedObject(item, depth + 1);
        if (result) return result;
      }
      return undefined;
    }
    if (typeof value !== "object") return undefined;
    for (const key of Object.keys(value)) {
      const result = guidFromNestedObject((value as any)[key], depth + 1);
      if (result) return result;
    }
    const fallbackMatch = JSON.stringify(value).match(GUID_PATTERN);
    return fallbackMatch?.[0] as Guid | undefined;
  };
  if (typeof entry === "string") {
    return guidFromString(entry);
  }
  if (!entry || typeof entry !== "object") return undefined;
  const directGuid = entry.guid || entry.id_ || entry.id || entry.pieceGuid || entry.connectionGuid || entry.modelGuid || entry.pieceId;
  if (typeof directGuid === "string" && directGuid.length > 0) return guidFromString(directGuid);
  const pieceGuid = entry.piece?.guid || entry.piece || entry.piece?.id_ || entry.piece?.id;
  if (typeof pieceGuid === "string" && pieceGuid.length > 0) return guidFromString(pieceGuid);
  const dataPieceGuid = entry.data?.piece?.guid || entry.data?.piece?.id_ || entry.data?.piece?.id || entry.data?.pieceId || entry.data?.id;
  if (typeof dataPieceGuid === "string" && dataPieceGuid.length > 0) return guidFromString(dataPieceGuid);
  return guidFromNestedObject(entry);
};

const resolveSelectionEntryGuidByKnownIds = (entry: any, knownIds: string[]): Guid | undefined => {
  const knownIdSet = new Set(knownIds);
  const resolvedGuid = resolveSelectionEntryGuid(entry);
  if (resolvedGuid && knownIdSet.has(resolvedGuid)) return resolvedGuid;
  const rawValue = typeof entry === "string" ? entry : JSON.stringify(entry);
  if (typeof rawValue !== "string" || rawValue.length === 0) return undefined;
  const sortedKnownIds = [...knownIdSet].sort((a, b) => b.length - a.length);
  const matchedKnownId = sortedKnownIds.find((knownId) => rawValue.includes(knownId));
  if (matchedKnownId) return matchedKnownId as Guid;
  return undefined;
};

const getDownstreamDescendants = (metadata: Map<string, PieceMetadata>, rootGuids: Set<string>): Set<string> => {
  const childrenMap = new Map<string, string[]>();
  for (const [guid, meta] of metadata) {
    if (meta.parentPieceId) {
      const siblings = childrenMap.get(meta.parentPieceId);
      if (siblings) siblings.push(guid);
      else childrenMap.set(meta.parentPieceId, [guid]);
    }
  }
  const descendants = new Set<string>();
  const queue = [...rootGuids];
  while (queue.length > 0) {
    const current = queue.pop()!;
    const children = childrenMap.get(current);
    if (!children) continue;
    for (const child of children) {
      if (!rootGuids.has(child) && !descendants.has(child)) {
        descendants.add(child);
        queue.push(child);
      }
    }
  }
  return descendants;
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

/** designToNodesAndEdges holds the data fields for a designToNodesAndEdges record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram🪨designtonodesandedges](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/designToNodesAndEdges)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨designtonodesandedges](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/designToNodesAndEdges)
 **/
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

/**
 * DesignDiagramProps holds the data fields for a DesignDiagramProps record.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖diagram✂️designdiagramprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Diagram/d/i/DesignDiagramProps)
 **/
interface DesignDiagramProps {
  reactFlowInstanceRef: React.MutableRefObject<ReactFlowInstance | null>;
}

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨designdiagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/DesignDiagram)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖diagram🪨designdiagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Diagram/d/i/DesignDiagram)
 * DesignDiagram holds the data fields for a DesignDiagram record.
 **/
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
  const suppressEdgeRecomputeRef = useRef(false);
  const rfStoreApi = useStoreApi();
  const designStore = useDesignStore(identitySelector) as DesignStore | null;

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

  const design = useDesign(undefined, undefined, true) as Design | null;
  const metadata = usePiecesMetadataMap();

  const commands = useDesignAppCommands();
  sharedCommandsRef = commands;

  const selectedConnector = selection?.connector ?? selection?.connectors?.[0];
  const selectedConnectorPortGuid = useMemo(() => {
    if (!selectedConnector?.piece || !selectedConnector.connector || !design) return undefined;
    const selectedPiece = design.pieces?.find((piece) => piece.guid === selectedConnector.piece);
    if (!selectedPiece) return undefined;
    if (selectedPiece.design?.guid) return "default";
    const selectedType = selectedPiece.type?.guid ? kitTypes?.find((type) => type.guid === selectedPiece.type?.guid) : undefined;
    const selectedTypeConnector = selectedType?.connectors?.find((connector) => connector.guid === selectedConnector.connector);
    return selectedTypeConnector?.port?.guid;
  }, [selectedConnector, design, kitTypes]);

  const designFilters = useDesignFilters();

  const { baseNodes, edges } = useMemo(() => {
    if (!design) return { baseNodes: [], edges: [] };
    const minimalKit = { types: kitTypes, designs: kitDesigns } as Kit;
    const result = designToNodesAndEdges(design, metadata, minimalKit) ?? { nodes: [], edges: [] };
    const filteredNodes = designFilters.showPieces ? result.nodes : [];
    const filteredEdges = (designFilters.showPieces && designFilters.showConnections) ? result.edges : [];
    return { baseNodes: filteredNodes, edges: filteredEdges };
  }, [design, metadata, kitTypes, kitDesigns, designFilters.showPieces, designFilters.showConnections]);

  const [nodes, setNodes] = useState<typeof baseNodes>(baseNodes);

  const isSyncingSelectionRef = useRef(false);
  const prevBaseNodesRef = useRef(baseNodes);
  useEffect(() => {
    const selectedPieces = new Set((selection?.pieces ?? []).map((entry) => resolveSelectionEntryGuid(entry)).filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0));
    const selectedConnections = new Set((selection?.connections ?? []).map((entry) => resolveSelectionEntryGuid(entry)).filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0));
    const baseNodesChanged = baseNodes !== prevBaseNodesRef.current;
    prevBaseNodesRef.current = baseNodes;
    isSyncingSelectionRef.current = true;
    setNodes(oldNodes => {
      if (!baseNodesChanged && oldNodes.length === baseNodes.length) {
        let changed = false;
        const result = oldNodes.map(node => {
          const pieceId = getPieceIdFromNode(node as DiagramNode);
          const shouldBeSelected = selectedPieces.has(pieceId) || selectedConnections.has(pieceId);
          if (node.selected === shouldBeSelected) return node;
          changed = true;
          return { ...node, selected: shouldBeSelected };
        });
        return changed ? result : oldNodes;
      }
      const oldNodeMap = new Map(oldNodes.map(n => [n.id, n]));
      return baseNodes.map(newNode => {
        const pieceId = getPieceIdFromNode(newNode as DiagramNode);
        const shouldBeSelected = selectedPieces.has(pieceId) || selectedConnections.has(pieceId);
        const oldNode = oldNodeMap.get(newNode.id);
        if (!oldNode) return { ...newNode, selected: shouldBeSelected };
        const measured = oldNode.measured;
        const positionSame = oldNode.position?.x === newNode.position?.x && oldNode.position?.y === newNode.position?.y;
        const oldData = oldNode.data as PieceNodeProps | undefined;
        const newData = newNode.data as PieceNodeProps | undefined;
        const dataSame = oldData?.piece?.guid === newData?.piece?.guid && oldData?.type?.guid === newData?.type?.guid && oldData?.piece?.description === newData?.piece?.description && oldData?.piece?.isHidden === newData?.piece?.isHidden && oldData?.piece?.type?.guid === newData?.piece?.type?.guid && oldData?.piece?.design?.guid === newData?.piece?.design?.guid && oldData?.type?.name === newData?.type?.name;
        if (positionSame && dataSame && oldNode.type === newNode.type && oldNode.selected === shouldBeSelected && oldNode.measured === measured) return oldNode;
        return { ...newNode, measured, selected: shouldBeSelected };
      });
    });
    requestAnimationFrame(() => { isSyncingSelectionRef.current = false; });
  }, [baseNodes, selection]);

  const onNodesChangeReactFlow = useCallback(
    (changes: any[]) => {
      if (changes.length === 0) return;
      const dimensionChanges = changes.filter((c: any) => c.type === "dimensions");
      if (isDraggingNodeRef.current || isPanningRef.current) {
        if (dimensionChanges.length > 0) {
          setNodes((nds) => applyNodeChanges(dimensionChanges, nds) as typeof nds);
        }
        return;
      }
      const nonSelectChanges = changes.filter((c: any) => c.type !== "select");
      if (nonSelectChanges.length === 0) return;
      setNodes((nds) => applyNodeChanges(nonSelectChanges, nds) as typeof nds);
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

  const selChangeCountRef = useRef(0);
  const pendingSelectionRafRef = useRef<number | null>(null);
  const onSelectionChange = useCallback(
    ({ nodes, edges }: { nodes: Array<Node>; edges: Array<Edge> }) => {
      selChangeCountRef.current++;
      if (isDraggingNodeRef.current || isPanningRef.current) return;
      if (isSyncingSelectionRef.current) return;

      const selectedPieceGuids = nodes.filter((n) => n.id.startsWith("piece-")).map((n) => getPieceIdFromNode(n as DiagramNode));

      const selectedConnectionGuids = edges
        .filter((e) => e.type === "SemioConnection" || e.id.startsWith("connection-") || (e as any).data?.SemioConnection)
        .map((e) => (e as any).data?.SemioConnection?.guid || e.id.split("-").pop())
        .filter((guid): guid is string => !!guid);

      let finalPieceGuids = selectedPieceGuids;
      let finalConnectionGuids = selectedConnectionGuids;

      if (isLassoingRef.current) {
        const base = baseSelectionRef.current || { pieces: [], connections: [] };
        const compositionKind = resolveSelectionCompositionKind(activeTool);
        finalPieceGuids = applySelectionComposition(base.pieces, selectedPieceGuids, compositionKind);
        finalConnectionGuids = applySelectionComposition(base.connections, selectedConnectionGuids, compositionKind);
      } else if (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) {
        return;
      }

      if (pendingSelectionRafRef.current !== null) {
        cancelAnimationFrame(pendingSelectionRafRef.current);
      }
      pendingSelectionRafRef.current = requestAnimationFrame(() => {
        pendingSelectionRafRef.current = null;
        if (isDraggingNodeRef.current || isPanningRef.current) return;
        const currentSelection = selectionRef.current || {};
        const currentPieces = currentSelection.pieces || [];
        const currentConnections = currentSelection.connections || [];
        const piecesChanged = finalPieceGuids.length !== currentPieces.length || finalPieceGuids.some((id) => !currentPieces.includes(id));
        const connectionsChanged =
          finalConnectionGuids.length !== currentConnections.length || finalConnectionGuids.some((id) => !currentConnections.includes(id));
        if ((piecesChanged || connectionsChanged) && setSelection) {
          setSelection({
            ...currentSelection,
            pieces: finalPieceGuids,
            connections: finalConnectionGuids,
          });
        }
      });
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
  const dragStartPositionRef = useRef<{ x: number; y: number } | null>(null);
  const pendingPieceUpdatesRef = useRef<Array<{ id: string; diff: any }>>([]);
  const dragDescendantsRef = useRef<Set<string>>(new Set());
  const dragDescendantOffsetsRef = useRef<Map<string, { dx: number; dy: number }>>(new Map());
  const dragDescendantNodeIdsRef = useRef<Map<string, string>>(new Map());
  const dragSelectedNodesRef = useRef<DiagramNode[]>([]);
  const dragNonSelectedNodesRef = useRef<DiagramNode[]>([]);
  const helperLinesDomRef = useRef<HTMLDivElement | null>(null);
  const updateHelperLinesDom = useCallback((lines: HelperLine[]) => {
    const container = helperLinesDomRef.current;
    if (!container) return;
    if (lines.length === 0) { container.innerHTML = ""; container.style.display = "none"; return; }
    container.style.display = "block";
    const viewport = reactFlowInstanceRef.current?.getViewport();
    if (!viewport) { container.innerHTML = ""; return; }
    let html = "";
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      if (line.kind === "horizontal" && line.position !== undefined) {
        html += `<div style="position:absolute;left:0;width:100%;height:1px;top:${line.position * viewport.zoom + viewport.y}px;border-top:1px dashed var(--accent);opacity:0.6"></div>`;
      } else if (line.kind === "vertical" && line.position !== undefined) {
        html += `<div style="position:absolute;top:0;width:1px;height:100%;left:${line.position * viewport.zoom + viewport.x}px;border-left:1px dashed var(--accent);opacity:0.6"></div>`;
      } else if (line.kind === "equalDistance" && line.x1 !== undefined && line.y1 !== undefined && line.x2 !== undefined && line.y2 !== undefined) {
        const sx1 = line.x1 * viewport.zoom + viewport.x;
        const sy1 = line.y1 * viewport.zoom + viewport.y;
        const sx2 = line.x2 * viewport.zoom + viewport.x;
        const sy2 = line.y2 * viewport.zoom + viewport.y;
        const isMid = line.relatedPieceId.startsWith("mid-");
        html += `<svg style="position:absolute;inset:0;width:100%;height:100%;pointer-events:none"><line x1="${sx1}" y1="${sy1}" x2="${sx2}" y2="${sy2}" stroke="var(--accent)" stroke-width="${isMid ? 3 : 2}" stroke-dasharray="${isMid ? "4 4" : "8 4"}" opacity="${isMid ? 1 : 0.7}" /></svg>`;
      }
    }
    container.innerHTML = html;
  }, [reactFlowInstanceRef]);
  const fullscreen = fullscreenWindow === DesignAppFullscreenWindow.Diagram;
  const viewportRestoredRef = useRef(false);
  const isUpdatingViewportRef = useRef(false);
  const dropZoneRef = useRef<HTMLDivElement | null>(null);
  const { activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign } = useDragDrop();

  const handleDiagramPointerDown = useCallback(
    (e: PointerEvent) => {
      const target = e.currentTarget as HTMLElement | null;
      if (target && document.activeElement !== target) {
        target.focus({ preventScroll: true });
      }
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
  const handleDiagramKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key.toLowerCase() !== "c" || (!e.metaKey && !e.ctrlKey) || e.altKey || e.shiftKey) return;
      const target = e.target as HTMLElement | null;
      if (!target) return;
      if (target.isContentEditable) return;
      const tagName = target.tagName;
      if (tagName === "INPUT" || tagName === "TEXTAREA" || tagName === "SELECT") return;
      if (!dropZoneRef.current || !dropZoneRef.current.contains(target)) return;
      e.preventDefault();
      e.stopPropagation();
      if (!design) return;
      const currentSelection = selectionRef.current;
      const selectedPieceGuids = new Set((currentSelection?.pieces ?? []).map((entry) => resolveSelectionEntryGuid(entry)).filter((g): g is Guid => typeof g === "string" && g.length > 0));
      const selectedConnectionGuids = new Set((currentSelection?.connections ?? []).map((entry) => resolveSelectionEntryGuid(entry)).filter((g): g is Guid => typeof g === "string" && g.length > 0));
      const hasSelection = selectedPieceGuids.size > 0 || selectedConnectionGuids.size > 0;
      let payload: { pieces: typeof design.pieces; connections: typeof design.connections };
      if (hasSelection) {
        const pieces = (design.pieces ?? []).filter((p) => selectedPieceGuids.has(p.guid));
        const pieceGuidSet = new Set(pieces.map((p) => p.guid));
        const connections = (design.connections ?? []).filter((c) => selectedConnectionGuids.has(c.guid) || (c.connected?.piece?.guid && c.connecting?.piece?.guid && pieceGuidSet.has(c.connected.piece.guid) && pieceGuidSet.has(c.connecting.piece.guid)));
        payload = { pieces, connections };
      } else {
        payload = { pieces: design.pieces ?? [], connections: design.connections ?? [] };
      }
      sketchpadCommands.copyJsonToClipboard("semio.sketchpad.app.design.canvas.diagram.keydown.cmdC", payload);
    },
    [sketchpadCommands, design],
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
        dropZoneRef.current.removeEventListener("keydown", handleDiagramKeyDown as any);
      }
      if (node) {
        node.setAttribute("data-drop-zone", "diagram");
        node.setAttribute("data-drop-zone-id", diagramId);
        node.tabIndex = 0;
        node.addEventListener("pointerdown", handleDiagramPointerDown as any);
        node.addEventListener("pointerup", handleDiagramPointerUp as any);
        node.addEventListener("pointerleave", handleDiagramPointerUp as any);
        node.addEventListener("keydown", handleDiagramKeyDown as any);
      }
      dropZoneRef.current = node;
    },
    [diagramId, handleDiagramPointerDown, handleDiagramPointerUp, handleDiagramKeyDown],
  );

  const handleDragEnd = useCallback(
    (event: DragEndEvent) => {
      const { active, delta } = event;
      if (!dropZoneRef.current || !reactFlowInstanceRef.current) return;
      if (!(event.activatorEvent instanceof MouseEvent)) return;
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
      const center = { u: centerU, v: centerV };
      const worldX = (center.u - 6) / 0.3;
      const worldZ = (center.v + 7) / 0.3;
      const plane: Plane = { origin: { x: worldX, y: 0, z: worldZ }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
      if (dragData.type === "type" && dragData.typeGuid) {
        const droppedType = kitTypes?.find((t) => t.guid === dragData.typeGuid);
        if (!droppedType) return;
        transaction?.start();
        const pieceGuid = guid();
        const piece = { guid: pieceGuid, type: { guid: droppedType.guid }, center, plane };
        addPiece?.(piece);
        transaction?.finalize();
      } else if (dragData.type === "design" && dragData.designGuid) {
        const droppedDesign = kitDesigns?.find((d) => d.guid === dragData.designGuid);
        if (!droppedDesign) return;
        transaction?.start();
        const pieceGuid = guid();
        const piece = {
          guid: pieceGuid,
          design: { guid: droppedDesign.guid },
          center,
          plane,
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
        isDraggingNodeRef.current = false;
        dragPositionRef.current = null;
        dragStartPositionRef.current = null;
        pendingPieceUpdatesRef.current = [];
        pendingSelectionRef.current = null;
        dragDescendantsRef.current = new Set();
        dragDescendantOffsetsRef.current = new Map();
        dragDescendantNodeIdsRef.current = new Map();
        updateHelperLinesDom(EMPTY_HELPER_LINES);
        if (reactFlowInstanceRef.current) {
          reactFlowInstanceRef.current.setNodes((nodes) => nodes.map((node) => ({ ...node })));
        }
      }
    };

    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [transaction, isDraggingNodeRef]);

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
    suppressEdgeRecomputeRef.current = true;
    (rfStoreApi as any).__suppressTransform = true;
    sceneFrameControlRef.current?.pause();
    const diagramElement = document.querySelector(`[data-diagram-id="${diagramId}"]`);
    if (diagramElement) {
      diagramElement.setAttribute("data-panning", "true");
    }
  }, [diagramId, isPanningRef, rfStoreApi]);

  const pendingMoveEndRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onMoveEnd = useCallback(() => {
    isPanningRef.current = false;
    suppressEdgeRecomputeRef.current = false;
    (rfStoreApi as any).__suppressTransform = false;
    sceneFrameControlRef.current?.resume();
    const pending = (rfStoreApi as any).__pendingTransform;
    if (pending) {
      (rfStoreApi as any).__original?.({ transform: pending });
      (rfStoreApi as any).__pendingTransform = null;
    }
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
  }, [reactFlowInstanceRef, setDiagramCenter, diagramId, isPanningRef, rfStoreApi]);

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
    (e: React.MouseEvent, node: DiagramNode) => {
      if (!setSelection) return;
      e.stopPropagation();
      const pieceGuid = getPieceIdFromNode(node);
      if (!pieceGuid) return;
      const compositionKind = resolveSelectionCompositionKind(activeToolRef.current, {
        shiftKey: e.shiftKey,
        altKey: e.altKey,
        ctrlKey: e.ctrlKey,
        metaKey: e.metaKey,
      });
      const currentPieces = selectionRef.current?.pieces || [];
      const newPieces = applySelectionComposition(currentPieces, [pieceGuid], compositionKind);
      setSelection({
        ...(selectionRef.current || {}),
        pieces: newPieces,
        connections: compositionKind === "replace" ? [] : (selectionRef.current?.connections || []),
      });
    },
    [setSelection],
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
    (e: React.MouseEvent, edge: DiagramEdge) => {
      if (!setSelection) return;
      e.stopPropagation();
      const connectionGuid = (edge as any).data?.SemioConnection?.guid || edge.id.split("-").pop();
      if (!connectionGuid) return;
      const compositionKind = resolveSelectionCompositionKind(activeToolRef.current, {
        shiftKey: e.shiftKey,
        altKey: e.altKey,
        ctrlKey: e.ctrlKey,
        metaKey: e.metaKey,
      });
      const currentConnections = selectionRef.current?.connections || [];
      const newConnections = applySelectionComposition(currentConnections, [connectionGuid], compositionKind);
      setSelection({
        ...(selectionRef.current || {}),
        pieces: compositionKind === "replace" ? [] : (selectionRef.current?.pieces || []),
        connections: newConnections,
      });
    },
    [setSelection],
  );

  const onPaneClick = useCallback(
    (e: React.MouseEvent) => {
      if (e.shiftKey || e.ctrlKey || e.metaKey || e.altKey) return;
      if (deselectAll) deselectAll();
    },
    [deselectAll],
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

  const pendingSelectionRef = useRef<{ pieceId: string; compositionKind: "replace" | "additive" | "subtractive" | "intersect" } | null>(null);

  const onNodeDragStart = useCallback(
    (event: any, node: Node) => {
      const currentSelectedIds = selectionRef.current?.pieces ?? [];
      const pieceId = getPieceIdFromNode(node as DiagramNode);
      const isNodeSelected = currentSelectedIds.includes(pieceId);
      const compositionKind = resolveSelectionCompositionKind(activeTool, {
        shiftKey: event.shiftKey === true,
        altKey: event.altKey === true,
        ctrlKey: event.ctrlKey === true,
        metaKey: event.metaKey === true,
      });
      if (compositionKind === "replace" && isNodeSelected) pendingSelectionRef.current = null;
      else pendingSelectionRef.current = { pieceId, compositionKind };

      dragPositionRef.current = { x: node.position.x, y: node.position.y };
      dragStartPositionRef.current = { x: node.position.x, y: node.position.y };
      pendingPieceUpdatesRef.current = [];
      isDraggingRef.current = true;
      isDraggingNodeRef.current = true;
      suppressEdgeRecomputeRef.current = true;
      const dragRoots = new Set(isNodeSelected && currentSelectedIds.length > 0 ? currentSelectedIds : [pieceId]);
      const descendants = getDownstreamDescendants(metadata, dragRoots);
      dragDescendantsRef.current = descendants;
      const offsets = new Map<string, { dx: number; dy: number }>();
      const descNodeIds = new Map<string, string>();
      if (descendants.size > 0) {
        const dragNodePos = { x: node.position.x, y: node.position.y };
        for (const descGuid of descendants) {
          const descNode = nodes.find((n) => getPieceIdFromNode(n) === descGuid);
          if (descNode) {
            offsets.set(descGuid, { dx: descNode.position.x - dragNodePos.x, dy: descNode.position.y - dragNodePos.y });
            descNodeIds.set(descGuid, descNode.id);
          }
        }
      }
      dragDescendantOffsetsRef.current = offsets;
      dragDescendantNodeIdsRef.current = descNodeIds;
      const diagramEl = document.querySelector(`[data-diagram-id="${diagramId}"]`);
      if (diagramEl) (diagramEl as HTMLElement).dataset.dragging = "true";
      const selectedIds = new Set(isNodeSelected && currentSelectedIds.length > 0 ? currentSelectedIds : [pieceId]);
      const selected: DiagramNode[] = [];
      const nonSelected: DiagramNode[] = [];
      for (const n of nodes) {
        if (selectedIds.has(getPieceIdFromNode(n))) selected.push(n);
        else nonSelected.push(n);
      }
      dragSelectedNodesRef.current = selected;
      dragNonSelectedNodesRef.current = nonSelected;
      sceneFrameControlRef.current?.pause();
      const allDraggedIds = new Set([...selectedIds, ...descendants]);
      designStore?.setDraggingPieces(allDraggedIds);
      setTimeout(() => transaction?.start(), 0);
    },
    [activeTool, isDraggingNodeRef, transaction, metadata, nodes, diagramId, designStore],
  );

  const isDraggingRef = useRef(false);

  const lastDragTimeRef = useRef<number>(0);
  const DRAG_THROTTLE_MS = 50;

  const onNodeDrag = useCallback(
    (event: any, node: DiagramNode) => {
      dragPositionRef.current = { x: node.position.x, y: node.position.y };
      if (!isDraggingRef.current || !reactFlowInstanceRef.current) return;

      const now = Date.now();
      if (now - lastDragTimeRef.current < DRAG_THROTTLE_MS) {
        return;
      }
      lastDragTimeRef.current = now;

      const piece = node.data.piece as Piece;
      const SNAP_THRESHOLD = 20;
      const lastPostition = dragPositionRef.current;
      if (!lastPostition || !reactFlowInstanceRef.current) return;

      const altPressed = event.altKey;

      const currentHelperLines: HelperLine[] = [];
      const nonSelectedNodes = dragNonSelectedNodesRef.current;
      const draggedCenterX = node.position.x + ICON_WIDTH / 2;
      const draggedCenterY = node.position.y + ICON_WIDTH / 2;

      const updatedPieces: Array<{ id: string; diff: any }> = [];

      let draggedX = node.position.x;
      let draggedY = node.position.y;

      const selectedNodes = dragSelectedNodesRef.current;
      for (const selectedNode of selectedNodes) {
        const piece = selectedNode.data.piece;
        const selectedInternalNode = reactFlowInstanceRef.current!.getInternalNode(selectedNode.id)!;

        if (selectedNode.type === "design") {
          if (selectedNode.id === node.id) {
            selectedInternalNode.internals.positionAbsolute.x = draggedX;
            selectedInternalNode.internals.positionAbsolute.y = draggedY;
            node.position.x = draggedX;
            node.position.y = draggedY;
          }

          updatedPieces.push({
            id: piece.guid,
            diff: {
              center: {
                u: selectedInternalNode.internals.positionAbsolute.x / ICON_WIDTH,
                v: -selectedInternalNode.internals.positionAbsolute.y / ICON_WIDTH,
              },
            },
          });
          continue;
        }

        const type = (selectedNode as PieceNode).data.type;
        const fixedPieceId = metadata.get(piece.guid)?.fixedPieceId;

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

          updateHelperLinesDom(currentHelperLines);
        } else {
          updateHelperLinesDom(EMPTY_HELPER_LINES);
        }

        if (selectedNode.id === node.id) {
          selectedInternalNode.internals.positionAbsolute.x = draggedX;
          selectedInternalNode.internals.positionAbsolute.y = draggedY;
          node.position.x = draggedX;
          node.position.y = draggedY;
        }

        updatedPieces.push({
          id: piece.guid,
          diff: {
            center: {
              u: selectedInternalNode.internals.positionAbsolute.x / ICON_WIDTH,
              v: -selectedInternalNode.internals.positionAbsolute.y / ICON_WIDTH,
            },
          },
        });
      }

      const draggedPieceGuid = (node.data.piece as Piece).guid;
      if (!updatedPieces.some((u) => u.id === draggedPieceGuid)) {
        updatedPieces.push({
          id: draggedPieceGuid,
          diff: {
            center: {
              u: draggedX / ICON_WIDTH,
              v: -draggedY / ICON_WIDTH,
            },
          },
        });
      }

      if (dragDescendantsRef.current.size > 0 && reactFlowInstanceRef.current) {
        for (const descGuid of dragDescendantsRef.current) {
          const offset = dragDescendantOffsetsRef.current.get(descGuid);
          if (!offset) continue;
          const newX = draggedX + offset.dx;
          const newY = draggedY + offset.dy;
          const descNodeId = dragDescendantNodeIdsRef.current.get(descGuid);
          if (descNodeId) {
            const descInternalNode = reactFlowInstanceRef.current!.getInternalNode(descNodeId);
            if (descInternalNode) {
              descInternalNode.internals.positionAbsolute.x = newX;
              descInternalNode.internals.positionAbsolute.y = newY;
              descInternalNode.position.x = newX;
              descInternalNode.position.y = newY;
            }
          }
          updatedPieces.push({
            id: descGuid,
            diff: {
              center: {
                u: newX / ICON_WIDTH,
                v: -newY / ICON_WIDTH,
              },
            },
          });
        }
      }
      dragPositionRef.current = { x: draggedX, y: draggedY };
      pendingPieceUpdatesRef.current = updatedPieces;
      if (designStore && updatedPieces.length > 0) {
        designStore.setDraggingPieces(new Set(updatedPieces.map((u) => u.id)));
      }
    },
    [design, reactFlowInstanceRef, metadata, designStore],
  );

  const onNodeDragStop = useCallback(
    (event: any, node: DiagramNode) => {
      const diagramEl = document.querySelector(`[data-diagram-id="${diagramId}"]`);
      if (diagramEl) (diagramEl as HTMLElement).dataset.dragging = "false";
      isDraggingRef.current = false;
      suppressEdgeRecomputeRef.current = false;
      const savedSelectedNodes = dragSelectedNodesRef.current;
      const savedNonSelectedNodes = dragNonSelectedNodesRef.current;
      const savedDescendantOffsets = dragDescendantOffsetsRef.current;
      dragSelectedNodesRef.current = [];
      dragNonSelectedNodesRef.current = [];
      const pendingSelection = pendingSelectionRef.current;
      const currentSelection = selectionRef.current;
      pendingSelectionRef.current = null;
      pendingPieceUpdatesRef.current = [];
      dragPositionRef.current = null;
      dragDescendantsRef.current = new Set();
      dragDescendantOffsetsRef.current = new Map();
      dragDescendantNodeIdsRef.current = new Map();
      designStore?.clearDraggingPieces();
      if (pendingSelection) {
        const { pieceId, compositionKind } = pendingSelection;
        if (setSelection) setTimeout(() => setSelection({ ...(currentSelection || {}), pieces: applySelectionComposition(currentSelection?.pieces, [pieceId], compositionKind) }), 650);
      }
      const finalX = node.position.x;
      const finalY = node.position.y;
      const draggedPieceId = getPieceIdFromNode(node);
      const selectedPieceIds = currentSelection?.pieces ?? [];
      const pieceIdsToUpdate = selectedPieceIds.length > 0 ? selectedPieceIds : [draggedPieceId];
      const startPos = dragStartPositionRef.current;
      dragStartPositionRef.current = null;
      const finalUpdates: Array<{ id: string; diff: any }> = [];
      const visualPositions = new Map<string, { x: number; y: number }>();
      if (design && startPos) {
        const offsetU = (finalX - startPos.x) / ICON_WIDTH;
        const offsetV = -(finalY - startPos.y) / ICON_WIDTH;
        for (const pieceGuid of pieceIdsToUpdate) {
          const originalPiece = design.pieces?.find((p) => p.guid === pieceGuid);
          if (!originalPiece) continue;
          const baseCenter = originalPiece.center ?? metadata.get(pieceGuid)?.center ?? { u: 0, v: 0 };
          const newCenter = { u: (baseCenter.u ?? 0) + offsetU, v: (baseCenter.v ?? 0) + offsetV };
          finalUpdates.push({ id: pieceGuid, diff: { center: newCenter } });
          visualPositions.set(pieceGuid, { x: newCenter.u * ICON_WIDTH, y: -newCenter.v * ICON_WIDTH });
        }
        for (const [descGuid, offset] of savedDescendantOffsets) {
          visualPositions.set(descGuid, { x: finalX + offset.dx, y: finalY + offset.dy });
        }
        const piecesDesign = { guid: "", name: "", pieces: pieceIdsToUpdate.map((g) => ({ guid: g })) } as Design;
        const dragDiff = dragPiecesInDesign(design, piecesDesign, { u: offsetU, v: offsetV });
        const connectionDiffUpdates = dragDiff.connections?.updated ?? [];
        if (connectionDiffUpdates.length > 0) {
          const connUpdates = connectionDiffUpdates.map((cu) => {
            const originalConn = design.connections?.find((c) => c.guid === cu.connection.guid);
            return { id: cu.connection.guid, diff: { u: (originalConn?.u ?? 0) + (cu.diff.u ?? 0), v: (originalConn?.v ?? 0) + (cu.diff.v ?? 0) } };
          });
          updateConnections?.(connUpdates);
        }
      }
      if (visualPositions.size > 0) {
        setNodes(prevNodes => prevNodes.map(n => {
          const pieceGuid = (n as any).data?.piece?.guid;
          if (!pieceGuid) return n;
          const pos = visualPositions.get(pieceGuid);
          if (!pos) return n;
          return { ...n, position: pos };
        }));
      }
      if (finalUpdates.length > 0) {
        updatePieces?.(finalUpdates);
      }
      if (reactFlowInstanceRef.current && !event.altKey) {
        const rfInstance = reactFlowInstanceRef.current;
        const selectedNodes = savedSelectedNodes;
        const nonSelectedNodes = savedNonSelectedNodes;
        setTimeout(() => {
          if (!rfInstance) return;
          const MIN_DISTANCE = 150;
          const spatialThreshold = MIN_DISTANCE + ICON_WIDTH;
          const connections = design?.connections ?? [];
          const connectedPiecePairs = new Set<string>();
          const usedPorts = new Set<string>();
          for (const conn of connections) {
            const p1 = conn.connected?.piece?.guid ?? "";
            const p2 = conn.connecting?.piece?.guid ?? "";
            if (p1 && p2) connectedPiecePairs.add(p1 < p2 ? `${p1}::${p2}` : `${p2}::${p1}`);
            if (p1 && conn.connected?.connector?.guid) usedPorts.add(`${p1}::${conn.connected.connector.guid}`);
            if (p2 && conn.connecting?.connector?.guid) usedPorts.add(`${p2}::${conn.connecting.connector.guid}`);
          }
          const portMap = new Map<string, Port>();
          for (const port of kit?.ports ?? []) portMap.set(port.guid, port);
          const connectorCache = new Map<string, Map<string, Connector>>();
          const getConnector = (type: Type, connectorGuid: string): Connector | undefined => {
            let map = connectorCache.get(type.guid);
            if (!map) {
              map = new Map<string, Connector>();
              for (const c of type.connectors ?? []) { if (c.guid) map.set(c.guid, c); }
              connectorCache.set(type.guid, map);
            }
            return map.get(connectorGuid);
          };
          for (const selectedNode of selectedNodes) {
            if (selectedNode.type === "design") continue;
            const piece = selectedNode.data.piece;
            const type = (selectedNode as PieceNode).data.type;
            const fixedPieceId = metadata.get(piece.guid)?.fixedPieceId;
            const selectedInternalNode = rfInstance.getInternalNode(selectedNode.id);
            if (!selectedInternalNode) continue;
            const selX = selectedInternalNode.internals.positionAbsolute.x;
            const selY = selectedInternalNode.internals.positionAbsolute.y;
            let closestConnection: SemioConnection | null = null;
            let closestDist = Number.MAX_VALUE;
            for (const otherNode of nonSelectedNodes) {
              if (otherNode.type !== "piece") continue;
              const otherInternalNode = rfInstance.getInternalNode(otherNode.id);
              if (!otherInternalNode) continue;
              const otherX = otherInternalNode.internals.positionAbsolute.x;
              const otherY = otherInternalNode.internals.positionAbsolute.y;
              if (Math.abs(selX - otherX) > spatialThreshold || Math.abs(selY - otherY) > spatialThreshold) continue;
              const pairKey = piece.guid < otherNode.data.piece.guid ? `${piece.guid}::${otherNode.data.piece.guid}` : `${otherNode.data.piece.guid}::${piece.guid}`;
              if (connectedPiecePairs.has(pairKey)) continue;
              for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
                if (!handle.id) continue;
                const connector = getConnector(type, handle.id);
                if (!connector || !connector.guid) continue;
                if (usedPorts.has(`${piece.guid}::${connector.guid}`)) continue;
                const connectorPort = connector.port?.guid ? portMap.get(connector.port.guid) : undefined;
                for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
                  if (!otherHandle.id) continue;
                  const otherConnector = getConnector((otherNode as PieceNode).data.type, otherHandle.id);
                  if (!otherConnector || !otherConnector.guid) continue;
                  if (usedPorts.has(`${otherNode.data.piece.guid}::${otherConnector.guid}`)) continue;
                  if (!selectedNode.data.piece.guid || !otherNode.data.piece.guid) continue;
                  if (fixedPieceId && fixedPieceId === metadata.get(otherNode.data.piece.guid)?.fixedPieceId) continue;
                  const otherPort = otherConnector.port?.guid ? portMap.get(otherConnector.port.guid) : undefined;
                  if (!arePortsCompatible(connectorPort, otherPort, kit?.ports || [])) continue;
                  const dx = selX + handle.x - (otherX + otherHandle.x);
                  const dy = selY + handle.y - (otherY + otherHandle.y);
                  const distance = Math.sqrt(dx * dx + dy * dy);
                  if (distance < closestDist && distance < MIN_DISTANCE) {
                    closestConnection = {
                      guid: crypto.randomUUID(),
                      connected: { piece: { guid: otherNode.data.piece.guid }, connector: { guid: otherHandle.id! } },
                      connecting: { piece: { guid: selectedNode.data.piece.guid }, connector: { guid: handle.id! } },
                      u: dx / ICON_WIDTH,
                      v: -(dy / ICON_WIDTH),
                    };
                    closestDist = distance;
                  }
                }
              }
            }
            if (closestConnection) {
              addConnection?.(closestConnection);
              updatePieces?.([{ id: piece.guid, diff: { center: undefined, plane: undefined } }]);
            }
          }
        }, 550);
      }
      setTimeout(() => { isDraggingNodeRef.current = false; }, 0);
      setTimeout(() => transaction?.finalize(), 700);
      updateHelperLinesDom(EMPTY_HELPER_LINES);
      sceneFrameControlRef.current?.resume();
    },
    [transaction, updatePieces, updateConnections, nodes, isDraggingNodeRef, setSelection, addConnection, design, metadata, reactFlowInstanceRef, kit, diagramId, designStore],
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

  const pieceRenderDataStoreRef = useRef<PieceRenderDataStoreApi>(createPieceRenderDataStore());
  const designStoreForSync = useDesignStore(identitySelector) as DesignStore | null;
  const actorForSync = useSketchpadActorSafe();
  const kitScopeForSync = useKitScope();
  const designScopeForSync = useDesignScope();
  const syncKeyRef = useRef("");
  syncKeyRef.current = `${kitScopeForSync?.guid ?? ""}:${designScopeForSync?.guid ?? ""}`;
  useEffect(() => {
    if (!designStoreForSync || !actorForSync) return;
    const getHoverAndSelection = () => {
      const appState = actorForSync.getSnapshot().context.designApps[syncKeyRef.current];
      return { hover: appState?.hover, selection: appState?.selection };
    };
    const sync = () => {
      const { hover, selection } = getHoverAndSelection();
      syncPieceRenderData(pieceRenderDataStoreRef.current, designStoreForSync, hover, selection);
    };
    sync();
    const unsubStore = designStoreForSync.subscribe(sync);
    const actorSub = actorForSync.subscribe(sync);
    return () => { unsubStore(); actorSub.unsubscribe(); };
  }, [designStoreForSync, actorForSync]);

  return (
    <PieceRenderDataStoreContext.Provider value={pieceRenderDataStoreRef.current}>
      <SelectedConnectorContext.Provider value={selectedConnector}>
        <SelectedConnectorPortContext.Provider value={selectedConnectorPortGuid}>
          <div id="semio.sketchpad.app.design.canvas.diagram" data-diagram-id={diagramId} className="h-full w-full relative" ref={setDropZoneRef}>
            <style>{`
            [data-diagram-id="${diagramId}"][data-panning="true"] .react-flow__node,
            [data-diagram-id="${diagramId}"][data-panning="true"] .react-flow__edge,
            [data-diagram-id="${diagramId}"][data-dragging="true"] .react-flow__node,
            [data-diagram-id="${diagramId}"][data-dragging="true"] .react-flow__edge {
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
              autoPanOnNodeDrag={false}
              selectNodesOnDrag={false}
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
            <div ref={helperLinesDomRef} className="absolute inset-0 w-full h-full pointer-events-none z-modal overflow-hidden" style={{ display: 'none' }} />
            <ClusterMenu nodes={nodes} edges={edges} onCluster={onCluster} />
            <ExpandMenu nodes={nodes} edges={edges} onExpand={onExpand} />
          </div>
        </SelectedConnectorPortContext.Provider>
      </SelectedConnectorContext.Provider>
    </PieceRenderDataStoreContext.Provider>
  );
};

// #endregion Diagram

// #region Scene

// [👤semio📚js🗃️sketchpad💻designtsx🔖canvas🔖scene](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/CANVAS/SCENE)
// Scene MUST render the Three.js 3D scene view of design pieces with selection and hover highlighting.

/** getComputedColor holds the data fields for a getComputedColor record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene🪨getcomputedcolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/getComputedColor)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨getcomputedcolor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/getComputedColor)
 **/
const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
/** applyHighlightToLoadedScene holds the data fields for a applyHighlightToLoadedScene record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene🪨applyhighlighttoloadedscene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/applyHighlightToLoadedScene)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨applyhighlighttoloadedscene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/applyHighlightToLoadedScene)
 **/
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

/** PresenceThree holds the data fields for a PresenceThree record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene🪨presencethree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/PresenceThree)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨presencethree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/PresenceThree)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene✂️planethreeprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/PlaneThreeProps)
 * PlaneThreeProps holds the data fields for a PlaneThreeProps record.
 **/
interface PlaneThreeProps {
  plane: Plane;
}

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨planethree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/PlaneThree)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨planethree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/PlaneThree)
 * PlaneThree holds the data fields for a PlaneThree record.
 **/
const PlaneThree: FC<PlaneThreeProps> = ({ plane }) => {
  const matrix = useMemo(() => planeToMatrix(plane), [plane]);
  return (
    <group matrix={matrix} matrixAutoUpdate={false}>
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
      <Line points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 1, 0)]} color={new THREE.Color(getComputedColor("--color-primary"))} />
    </group>
  );
};

interface DesignMeshEventProps {
  onClick?: (e: ThreeEvent<MouseEvent>) => void;
  onDoubleClick?: (e: ThreeEvent<MouseEvent>) => void;
  onPointerEnter?: (e: ThreeEvent<PointerEvent>) => void;
  onPointerLeave?: (e: ThreeEvent<PointerEvent>) => void;
}

const GLTFMesh: FC<{ url: string; highlightColor: string | null } & DesignMeshEventProps> = ({ url, highlightColor, onClick, onDoubleClick, onPointerEnter, onPointerLeave }) => {
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
  return <primitive object={clonedScene} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
};

const FBXMesh: FC<{ url: string; highlightColor: string | null } & DesignMeshEventProps> = ({ url, highlightColor, onClick, onDoubleClick, onPointerEnter, onPointerLeave }) => {
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
  return <primitive object={clonedScene} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
};

const OBJMesh: FC<{ url: string; highlightColor: string | null } & DesignMeshEventProps> = ({ url, highlightColor, onClick, onDoubleClick, onPointerEnter, onPointerLeave }) => {
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
  return <primitive object={clonedScene} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
};

const LoadedPieceMesh: FC<{ url: string; fileExtension: string; highlightColor: string | null } & DesignMeshEventProps> = ({ url, fileExtension, highlightColor, onClick, onDoubleClick, onPointerEnter, onPointerLeave }) => {
  const ext = fileExtension.toLowerCase();
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} highlightColor={highlightColor} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} highlightColor={highlightColor} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} highlightColor={highlightColor} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
  } else {
    return <GLTFMesh url={url} highlightColor={highlightColor} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />;
  }
};

const PieceMesh: FC<{ highlightColor: string | null } & DesignMeshEventProps> = ({ highlightColor, onClick, onDoubleClick, onPointerEnter, onPointerLeave }) => {
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
    const model = selectBestModel(type.models, tagsForType);
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
      <LoadedPieceMesh url={blobUrl} fileExtension={fileExtension} highlightColor={highlightColor} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} />
    </Suspense>
  );
};

interface ModelPieceProps {}

// [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨modelpiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/ModelPiece)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨modelpiece](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/ModelPiece)
 * ModelPiece holds the data fields for a ModelPiece record.
 **/
const ModelPiece: FC<ModelPieceProps> = () => {
  const piece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const isSelected = useIsPieceSelected();
  const isHovered = useIsPieceTransitiveHovered();
  const status = usePieceStatus();
  const flatPlane = useFlatPiecePlane();

  const [selection, setSelection] = useDesignAppSelection();
  const [hoverPiece] = useDesignAppHoverPiece();
  const [clearHover] = useDesignAppClearHover();
  const [focusPiece] = useDesignAppFocusPiece();
  const { currentHoveredPieceGuidRef } = useHoverIntent();
  const [activeTool] = useDesignAppActiveTool();

  const { fill } = useDesignAppPieceColor(undefined, piece.guid);

  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);
  const mutedForegroundColor = useMemo(() => getComputedColor("--muted-foreground"), []);
  const activeBaseColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = useMemo(() => getComputedColor("--hover-base"), []);
  const highlightColor = useMemo(() => (isSelected ? activeBaseColor : isHovered ? hoverBaseColor : null), [isSelected, isHovered, activeBaseColor, hoverBaseColor]);

  const isConnectedChild = useIsConnectedPiece();
  const originalPlane = isConnectedChild ? flatPlane : (piece.plane || flatPlane);
  const diffedPlane = isConnectedChild ? flatPlane : (diffedPiece.plane || flatPlane);

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
      if (!setSelection) return;
      const compositionKind = resolveSelectionCompositionKind(activeTool, {
        shiftKey: e?.shiftKey === true,
        altKey: e?.altKey === true,
        ctrlKey: e?.ctrlKey === true,
        metaKey: e?.metaKey === true,
      });
      setSelection({
        ...(selection || {}),
        pieces: applySelectionComposition(selection?.pieces, [piece.guid], compositionKind),
        connections: compositionKind === "replace" ? [] : (selection?.connections || []),
      });
    },
    [selection, setSelection, piece.guid, activeTool],
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
    <group userData={userData}>
      <PieceMesh highlightColor={highlightColor} onClick={onSelect} onDoubleClick={onDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave} />
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

/** ModelDesign holds the data fields for a ModelDesign record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene🪨modeldesign](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/ModelDesign)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨modeldesign](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/ModelDesign)
 **/
const ModelDesign: FC = () => {
  const [transaction] = useDesignAppTransaction();
  const [updatePiece] = useDesignAppUpdatePiece();
  const [selection] = useDesignAppSelection();
  const [others] = useDesignAppOthers();
  const design = useDesign();
  const flatDesign = design as Design;
  const { showPieces } = useDesignFilters();

  const [selectPieces] = useDesignAppSelectPieces();

  // Memoize resolvePieceGuid to avoid recreating it on every onChange
  const resolvePieceGuid = useCallback((object: THREE.Object3D | undefined): string | undefined => {
    let current: THREE.Object3D | null | undefined = object;
    while (current) {
      const pieceId = current.userData?.pieceId;
      if (typeof pieceId === "string" && pieceId.length > 0) return pieceId;
      const id = current.userData?.id;
      if (typeof id === "string" && id.length > 0) return id;
      current = current.parent;
    }
    return undefined;
  }, []);

  // Create a stable Set reference for previous selection for O(1) lookups
  const previousSelectionSet = useMemo(
    () => new Set((selection.pieces ?? []).map((entry) => resolveSelectionEntryGuid(entry)).filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0)),
    [selection.pieces],
  );

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      // Resolve piece GUIDs
      const newSelectedPieceIds = Array.from(new Set(selected.map((item) => resolvePieceGuid(item)).filter((value): value is string => !!value)));
      
      // Fast Set-based comparison - O(n) instead of O(n²)
      const newSelectionSet = new Set(newSelectedPieceIds);
      
      // Check if selection changed using Set comparison
      const changed =
        newSelectionSet.size !== previousSelectionSet.size ||
        Array.from(newSelectionSet).some((id) => !previousSelectionSet.has(id));
      
      if (changed) {
        if (selectPieces) selectPieces(newSelectedPieceIds);
      }
    },
    [selectPieces, resolvePieceGuid, previousSelectionSet],
  );

  type TransformableModel = { guid: string; plane: Plane | undefined; isTransformable: boolean; isSelected: boolean };
  
  // Optimize selectedModels using Set for O(1) lookup instead of O(n) includes
  const selectedModels = useMemo((): TransformableModel[] => {
    if (!selection.pieces || !flatDesign?.pieces) return [];

    const selectedPiecesSet = new Set((selection.pieces || []).map((entry) => resolveSelectionEntryGuid(entry)).filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0));
    return flatDesign.pieces
      .filter((piece) => selectedPiecesSet.has(piece.guid))
      .map((piece) => ({
        guid: piece.guid,
        plane: piece.plane,
        isTransformable: !piece.isLocked && piece.plane !== undefined,
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
          {showPieces && flatDesign?.pieces?.map((piece: Piece) => (
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

/** DesignAppScene holds the data fields for a DesignAppScene record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖canvas🔖scene🪨designappscene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Canvas/s/Scene/d/i/DesignAppScene)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖canvas🔖scene🪨designappscene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Canvas/s/Scene/d/i/DesignAppScene)
 **/
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
      if (!(event.activatorEvent instanceof MouseEvent)) return;
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

// #endregion Scene

// #endregion Canvas

// #region 🔖Windows
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windows](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Windows)
// Window components MUST wrap diagram and scene views with hover and transaction providers.

/** Props interface for the Design app root component.
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windows🛠️appprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Windows/d/i/AppProps)
**/
export interface AppProps { }

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windows🪨diagramwindow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Windows/d/i/DiagramWindow)
 * DiagramWindow holds the data fields for a DiagramWindow record.
 **/
const DiagramWindow = memo<{ reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> }>(({ reactFlowInstanceRef }) => {
  return (
    <DesignFilterProvider>
      <HoverIntentProvider>
        <TransactionPiecesProvider>
          <HoverPiecesProvider>
            <DesignDiagram reactFlowInstanceRef={reactFlowInstanceRef} />
          </HoverPiecesProvider>
        </TransactionPiecesProvider>
      </HoverIntentProvider>
    </DesignFilterProvider>
  );
});
DiagramWindow.displayName = "DiagramWindow";

/**
 * [👤semio📚js🗃️sketchpad💻design🔖imports🔖panels🔖windows🪨scenewindow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/Panels/s/Windows/d/i/SceneWindow)
 * SceneWindow holds the data fields for a SceneWindow record.
 **/
const SceneWindow = memo(() => {
  return (
    <DesignFilterProvider>
      <HoverIntentProvider>
        <TransactionPiecesProvider>
          <HoverPiecesProvider>
            <DesignAppScene />
          </HoverPiecesProvider>
        </TransactionPiecesProvider>
      </HoverIntentProvider>
    </DesignFilterProvider>
  );
});
SceneWindow.displayName = "SceneWindow";

// #endregion 🔖Windows

// #endregion 🔖Panels
// #region App
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/App)
// App MUST compose all Design app panels, canvas, toolbar, and footer into the main Design app layout.


/** App holds the data fields for a App record.
 **/
// [👤semio📚js🗃️sketchpad💻design🔖imports🔖app🪨designapp](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/App/d/i/DesignApp)
/**
 * [👤semio📚js🗃️sketchpad💻design🔖app🪨app](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/App/d/i/App)
 **/
const App: FC<AppProps> = () => {
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
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const kit = useKit(undefined, undefined, true) as Kit | null;
  const designFromScope = useDesign() as Design | null;
  const design = useMemo(() => {
    if (designFromScope) return designFromScope;
    if (!kit || !designScope) return undefined;
    return kit.designs?.find((entry) => entry.guid === designScope.guid);
  }, [designFromScope, kit, designScope?.guid]);
  const kitGuid = kitScope?.guid ?? kit?.guid;
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
    const removeLegacySideTabsFromWindowLayout = (layoutNode: any): any => {
      if (!layoutNode || typeof layoutNode !== "object") return layoutNode;
      if (
        layoutNode.type === "component" &&
        (layoutNode.componentName === "workbench" || layoutNode.componentName === DesignAppWindowKind.Settings || layoutNode.componentName === DesignAppWindowKind.Chat)
      ) {
        return null;
      }
      if (layoutNode.root && typeof layoutNode.root === "object") {
        const root = removeLegacySideTabsFromWindowLayout(layoutNode.root);
        if (!root) return undefined;
        return { ...layoutNode, root };
      }
      if (Array.isArray(layoutNode.content)) {
        const content = layoutNode.content.map((item: any) => removeLegacySideTabsFromWindowLayout(item)).filter(Boolean);
        if (content.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, content };
      }
      if (Array.isArray(layoutNode.contentItems)) {
        const contentItems = layoutNode.contentItems.map((item: any) => removeLegacySideTabsFromWindowLayout(item)).filter(Boolean);
        if (contentItems.length === 0 && (layoutNode.type === "stack" || layoutNode.type === "row" || layoutNode.type === "column")) return null;
        return { ...layoutNode, contentItems };
      }
      return layoutNode;
    };
    const sanitizedLayout = removeLegacySideTabsFromWindowLayout(storedWindowLayout);

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

    const hasScene = hasSceneWindow(sanitizedLayout);
    if (!hasScene) {
      return undefined;
    }

    return sanitizedLayout;
  }, [storedWindowLayout]);

  useEffect(() => {
    if (!store || !storedWindowLayout) return;
    if (windowLayout === undefined) {
      try {
        store.change({ windowLayout: undefined });
      } catch (error) {
        console.error("[DesignApp] Failed to clear layout:", error);
      }
      return;
    }
    if (windowLayout !== storedWindowLayout) {
      try {
        store.change({ windowLayout });
      } catch (error) {
        console.error("[DesignApp] Failed to migrate layout:", error);
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
  const addSidePanelTab = useAddSidePanelTab();
  const removeSection = useRemovePanelSection();
  const removeSidePanelTab = useRemoveSidePanelTab();
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
  useHotkeys("delete,backspace", () => deleteSelected?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+z", () => undo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+y", () => redo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+shift+z", () => redo?.(), { enableOnFormTags: true });

  const appType = useAppType();

  useEffect(() => {
    if (appType !== "design") return;
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.design.settings",
      icon: SettingsIcon,
      order: 100,
      content: () => (
        <TreeStateProvider>
          <Tree className="min-w-0 overflow-hidden p-double">
            <DesignSettingsContent />
          </Tree>
        </TreeStateProvider>
      ),
    });
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.design.chat",
      icon: ChatIcon,
      order: 101,
      content: () => <BasicChatPanel id="semio.sketchpad.app.design.chat" title="Design" />,
    });
    return () => {
      removeSidePanelTab("right", "semio.sketchpad.app.design.settings");
      removeSidePanelTab("right", "semio.sketchpad.app.design.chat");
    };
  }, [appType, addSidePanelTab, removeSidePanelTab]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind === ToolKind.SELECTION_NORMAL && activeTool !== ToolKind.SELECTION_NORMAL) setActiveTool(ToolKind.SELECTION_NORMAL);
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
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

    const selectionPieceEntries = (selection.pieces || []) as any[];
    const selectionConnectionEntries = (selection.connections || []) as any[];
    const designConnections = design?.connections || [];
    const knownPieceIds = [...new Set([...(design?.pieces || []).map((entry) => entry.guid), ...getIncludedDesigns(design || ({} as Design)).map((entry) => entry.guid)])];
    const knownConnectionGuids = new Set(designConnections.map((entry) => entry.guid));
    const selectedPieceGuids = selectionPieceEntries.map((entry) => resolveSelectionEntryGuidByKnownIds(entry, knownPieceIds)).filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0);
    const knownPieceGuids = new Set(knownPieceIds);
    const selectedKnownPieceGuids = selectedPieceGuids.filter((guid) => knownPieceGuids.has(guid));
    const selectedConnectionGuids = selectionConnectionEntries
      .map((entry) => resolveSelectionEntryGuid(entry))
      .filter((entry): entry is Guid => typeof entry === "string" && entry.length > 0 && knownConnectionGuids.has(entry));
    const selectedConnectionIds = selectionConnectionEntries.filter((entry) => {
      if (typeof entry !== "object" || entry === null) return false;
      const resolvedGuid = resolveSelectionEntryGuid(entry);
      return resolvedGuid === undefined;
    }) as ConnectionId[];
    const guidConnections = designConnections.filter((connection) => selectedConnectionGuids.includes(connection.guid));
    const idConnections = designConnections.filter((connection) => selectedConnectionIds.some((connectionId) => areSameConnection(connection, connectionId as any)));
    const selectedConnections = [...guidConnections, ...idConnections.filter((connection) => !guidConnections.some((guidConnection) => guidConnection.guid === connection.guid))];
    const primaryConnector = selection.connector ?? selection.connectors?.[0];
    const hasPieces = selectedKnownPieceGuids.length > 0;
    const hasConnections = selectedConnections.length > 0;
    const hasPortSelected = primaryConnector !== undefined;
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
        defaultOpen: true,
        content: () =>
          design && kitGuid ? (
            <KitScopeProvider guid={kitGuid}>
              <DesignScopeProvider guid={design.guid}>
                <DesignSection />
              </DesignScopeProvider>
            </KitScopeProvider>
          ) : null,
      });
    } else if (hasPortSelected) {
      const connectorPieceId = primaryConnector!.piece;
      const connectorId = primaryConnector!.connector;
      addSection("details", {
        id: "semio.sketchpad.app.type.connector.properties",
        specificity: 30,
        order: 0,
        defaultOpen: true,
        content: () =>
          design && kitGuid ? (
            <KitScopeProvider guid={kitGuid}>
              <DesignScopeProvider guid={design.guid}>
                <ConnectorSection pieceGuid={connectorPieceId} connectorGuid={connectorId} />
              </DesignScopeProvider>
            </KitScopeProvider>
          ) : null,
      });
      addSection("details", {
        id: "semio.sketchpad.app.design.properties",
        specificity: 20,
        order: 50,
        defaultOpen: true,
        content: () =>
          design && kitGuid ? (
            <KitScopeProvider guid={kitGuid}>
              <DesignScopeProvider guid={design.guid}>
                <DesignSection />
              </DesignScopeProvider>
            </KitScopeProvider>
          ) : null,
      });
    } else {
      if (hasPieces) {
        const piecesCount = selectedKnownPieceGuids.length;
        const piecesSectionId = piecesCount === 1 ? pieceSingleId : pieceMultipleId;
        addSection("details", {
          id: piecesSectionId,
          specificity: 30,
          order: 0,
          defaultOpen: true,
          content: () =>
            design && kitGuid ? (
              <KitScopeProvider guid={kitGuid}>
                <DesignScopeProvider guid={design.guid}>
                  <PiecesSection />
                </DesignScopeProvider>
              </KitScopeProvider>
            ) : null,
        });
      }
      if (hasConnections) {
        const connectionsSectionId = selectedConnections.length === 1 ? connectionSingleId : connectionMultipleId;
        addSection("details", {
          id: connectionsSectionId,
          specificity: 30,
          order: 10,
          defaultOpen: true,
          content: () =>
            design && kitGuid ? (
              <KitScopeProvider guid={kitGuid}>
                <DesignScopeProvider guid={design.guid}>
                  <ConnectionsSection connections={selectedConnections} isSingle={selectedConnections.length === 1} count={selectedConnections.length} />
                </DesignScopeProvider>
              </KitScopeProvider>
            ) : null,
        });
      }
      if (hasPieces && hasConnections) {
        addSection("details", {
          id: selectionMultipleId,
          specificity: 30,
          order: 20,
          content: () => (
            <TreeRow>
                <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.design.selectOnlyPiecesOrConnections")}</p>
              </TreeRow>
          ),
        });
      }
      addSection("details", {
        id: "semio.sketchpad.app.design.properties",
        specificity: 20,
        order: 50,
        defaultOpen: true,
        content: () =>
          design && kitGuid ? (
            <KitScopeProvider guid={kitGuid}>
              <DesignScopeProvider guid={design.guid}>
                <DesignSection />
              </DesignScopeProvider>
            </KitScopeProvider>
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
  }, [selection, addSection, removeSection, appType, t, design, kitGuid]);

  useEffect(() => {
    if (appType !== "design") return;
    addSection("workbench", {
      id: "semio.sketchpad.app.kit.pieces",
      specificity: 20,
      order: 1,
      content: () => <PiecesWorkbenchContent />,
    });
    addSection("tools", {
      id: "semio.sketchpad.app.design.windows",
      specificity: 20,
      order: 2,
      content: () => <WindowLibrary />,
    });
    return () => {
      removeSection("workbench", "semio.sketchpad.app.kit.pieces");
      removeSection("tools", "semio.sketchpad.app.design.windows");
    };
  }, [appType, addSection, removeSection]);

  const PiecesWorkbenchContent: FC = () => {
    const kit = useKit() as Kit;
    const resolveParentGuid = (parent: any): string | undefined => (typeof parent === "string" ? parent : parent?.guid);

    const handleCreateTypeChild = (parentType: Type) => {
      const existingChildren = workbenchTypes?.filter((type) => resolveParentGuid(type.parent) === parentType.guid) || [];
      const uniqueName = generateUniqueName(
        parentType.name,
        [parentType.name, ...existingChildren.map((type) => type.name)],
      );
      const newType: Type = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: parentType.guid },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addType(newType);
    };

    const handleCreateDesignChild = (parentDesign: Design) => {
      const existingChildren = workbenchDesigns?.filter((design) => resolveParentGuid(design.parent) === parentDesign.guid) || [];
      const uniqueName = generateUniqueName(
        parentDesign.name,
        [parentDesign.name, ...existingChildren.map((design) => design.name)],
      );
      const newDesign: Design = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: parentDesign.guid },
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addDesign(newDesign);
      if (kitGuid) navigateToDesign(kitGuid, newDesign.guid);
    };

    const renderTypeTree = (types: Type[]): ReactNode[] => {
      return types.map((type) => {
        const children = workbenchTypes?.filter((item) => resolveParentGuid(item.parent) === type.guid) || [];
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
        const children = workbenchDesigns?.filter((child) => resolveParentGuid(child.parent) === workbenchDesign.guid) || [];

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

    const rootTypes = workbenchTypes?.filter((type) => !resolveParentGuid(type.parent)) || [];
    const rootDesigns = workbenchDesigns?.filter((design) => !resolveParentGuid(design.parent)) || [];

    const handleCreateType = () => {
      const existingTypes = workbenchTypes || [];
      const typeNumber = existingTypes.length + 1;
      const newType: Type = {
        guid: guid(),
        name: `Type ${typeNumber}`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      kitAppCommands.addType(newType);
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
      kitAppCommands.addDesign(newDesign);
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
              dataDragKind="type"
              dataDragGuid={type.guid}
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
            onClick: () => {
              const center = { u: 0, v: 0 };
              const worldX = (center.u - 6) / 0.3;
              const worldZ = (center.v + 7) / 0.3;
              const plane: Plane = { origin: { x: worldX, y: 0, z: worldZ }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
              transaction?.start();
              addPiece?.({ guid: guid(), type: { guid: type.guid }, center, plane });
              transaction?.finalize();
            },
            id: "semio.sketchpad.app.design.panel.workbench.types.addPiece",
          },
          {
            icon: <TypeIcon size={12} />,
            onClick: () => onCreateChild(type),
            id: "semio.sketchpad.common.duplicateType",
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
              dataDragKind="design"
              dataDragGuid={design.guid}
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
            onClick: () => {
              if (disabled) return;
              const center = { u: 0, v: 0 };
              const worldX = (center.u - 6) / 0.3;
              const worldZ = (center.v + 7) / 0.3;
              const plane: Plane = { origin: { x: worldX, y: 0, z: worldZ }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
              transaction?.start();
              addPiece?.({ guid: guid(), design: { guid: design.guid }, center, plane });
              transaction?.finalize();
            },
            id: "semio.sketchpad.app.design.panel.workbench.designs.addPiece",
          },
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

  return (
    <ReactFlowProvider>
      <Canvas id="semio.sketchpad.app.design.canvas">
        <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
      </Canvas>
      <DesignAppFooter />
    </ReactFlowProvider>
  );
};

// #region Settings

const DesignSettingsContent: FC = () => {
  const [panelVisibility, setPanelVisibility, canSetPanelVisibility] = useDesignAppPanelVisibility();
  const showToolbarLabel = useLabel("semio.sketchpad.navbar.panelToggle.toolbar.show");
  const showWorkbenchLabel = useLabel("semio.sketchpad.navbar.panelToggle.workbench.show");
  const showDetailsLabel = useLabel("semio.sketchpad.navbar.panelToggle.details.show");
  const showWindowsLabel = useLabel("semio.sketchpad.navbar.panelToggle.tools.show");
  const togglePanelVisibility = useCallback(
    (panelKey: "toolbar" | "leftSidePanel" | "rightSidePanel" | "details") => {
      if (!canSetPanelVisibility || !setPanelVisibility) return;
      setPanelVisibility({ ...panelVisibility, [panelKey]: !panelVisibility[panelKey] });
    },
    [canSetPanelVisibility, panelVisibility, setPanelVisibility],
  );
  return (
    <>
      <TreeRow>
        <Toggle id="semio.sketchpad.app.design.settings.panel.toolbar" pressed={!!panelVisibility.toolbar} onPressedChange={() => togglePanelVisibility("toolbar")} text={showToolbarLabel} disabled={!canSetPanelVisibility} />
      </TreeRow>
      <TreeRow>
        <Toggle id="semio.sketchpad.app.design.settings.panel.workbench" pressed={!!panelVisibility.leftSidePanel} onPressedChange={() => togglePanelVisibility("leftSidePanel")} text={showWorkbenchLabel} disabled={!canSetPanelVisibility} />
      </TreeRow>
      <TreeRow>
        <Toggle id="semio.sketchpad.app.design.settings.panel.windows" pressed={!!panelVisibility.rightSidePanel} onPressedChange={() => togglePanelVisibility("rightSidePanel")} text={showWindowsLabel} disabled={!canSetPanelVisibility} />
      </TreeRow>
      <TreeRow>
        <Toggle id="semio.sketchpad.app.design.settings.panel.details" pressed={!!panelVisibility.details} onPressedChange={() => togglePanelVisibility("details")} text={showDetailsLabel} disabled={!canSetPanelVisibility} />
      </TreeRow>
    </>
  );
};

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
      },
      content: <DesignSelectSettings />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.design.toolbar.filters",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "filter",
        labelId: "semio.sketchpad.toolbar.parent.filter",
        order: 20,
      },
      content: <DesignToolbarFilters />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.design.tools.select");
      removeSection("toolbar", "semio.sketchpad.app.design.toolbar.filters");
    };
  }, [appType, addSection, removeSection]);

  return (
    <DesignFilterProvider>
      <DesignAppTransactionProvider>
        <HoverIntentProvider>
          <TransactionPiecesProvider>
            <HoverPiecesProvider>
              <App />
            </HoverPiecesProvider>
          </TransactionPiecesProvider>
        </HoverIntentProvider>
      </DesignAppTransactionProvider>
    </DesignFilterProvider>
  );
};

// #endregion App

// #region Config

// [👤semio📚js🗃️sketchpad💻designtsx🔖app🔖config](semiorepo://section/SEMIO/JS/SKETCHPAD/DESIGN.TSX/APP/CONFIG)
// Config MUST export the Design app configuration with route segments, panel definitions, and path matching.

/**
 * Exported Design app configuration including routes, panels, and path matching.
 *
 *  * [👤semio📚js🗃️sketchpad💻design🔖imports🔖app🔖config🪨config](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Design.tsx/s/Imports/s/App/s/Config/d/i/config)
 **/
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
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "designs" && isUuidPattern(pathParts[3]);
  },
  order: 20,
};

export default DesignApp;

// #endregion Config
