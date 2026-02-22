// #region Header

// [👤semio📚js🗃️sketchpad💻kittsx](semiorepo://file/SEMIO/JS/SKETCHPAD/KIT.TSX)

// SPDX-License-Identifier: LGPL-3.0-or-later

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

// Kit editor app for managing types, designs and qualities.

// #endregion Header

// #region Imports

// [👤semio📚js🗃️sketchpad💻kittsx🔖imports](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/IMPORTS)
// Imports for Kit app MUST include all shared sketchpad, React, DnD, and UI dependencies.

import { DragEndEvent, DragOverEvent, DragStartEvent, useDroppable } from "@dnd-kit/core";
import {
  AddIcon,
  AlertCircleIcon,
  AwardIcon,
  CheckIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  CodeIcon,
  DiagramIcon,
  DocumentIcon,
  FileCodeIcon,
  FileImageIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTypeIcon,
  FileVideoIcon,
  FolderIcon,
  HandIcon,
  HashIcon,
  IntersectIcon,
  LayoutIcon,
  LightbulbIcon,
  MonitorIcon,
  MoonIcon,
  MousePointerIcon,
  PortIcon,
  RemoveIcon,
  SceneIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  SunIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
} from "@semio/assets";
import { useSelector } from "@xstate/react";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import React, { FC, memo, useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { Camera } from "three";
import * as Y from "yjs";
import i18n, { useLabel } from "../i18n";
import { Author, buildFileTree, Concept, Coord, Design, DesignDiff, DiffStatus, flattenFileTree, Folder, generateUniqueName, guid, Guid, Kit, KitDiff, Port, Quality, File as SemioFile, Tag, Type, TypeDiff } from "../semio";
import type { KitStore as KitDataSource, SketchpadStore as SketchpadOrchestrator } from "./Sketchpad";
import {
  AppWindowConfig,
  Canvas,
  createDefaultKitAppState,
  createKitDiagramForceSelector,
  createKitExpandedRowsSelector,
  createKitFilterSearchSelector,
  createKitFullscreenSelector,
  createKitHoverSelector,
  createKitOthersSelector,
  createKitSelectionSelector,
  createKitSortColumnSelector,
  createKitSortDirectionSelector,
  createKitWindowLayoutSelector,
  defaultPanelVisibility,
  KitAppFullscreenWindow,
  KitDiffAppStore as KitDiffStore,
  KitScopeProvider,
  LayoutCanvas,
  registerKitAppStoreFactory as registerKitStoreFactory,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useDesignScope,
  useDevice,
  useExpertise,
  useFocus,
  useHasKit,
  useIsInKitScope,
  useIsMobile,
  useKit,
  useKitAppXState,
  useKitCommands,
  useKitScope,
  useLanguage,
  useMode,
  useNavigation,
  useOrigin,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadActor,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
  useTheme,
  useTypeScope,
  Window
} from "./Sketchpad";
import type { ConnectionLineComponentProps, Edge, EdgeProps, Node, NodeProps, Simulation, SimulationLinkDatum, SimulationNodeDatum } from "./elements";
import {
  Action,
  applyNodeChanges,
  BaseEdge,
  Button,
  Diagram,
  forceCollide,
  forceLink,
  forceManyBody,
  forceSimulation,
  forceX,
  forceY,
  getBezierPath,
  Handle,
  Input,
  NotFound,
  Position,
  ReactFlowProvider,
  Scrollable,
  Select,
  SelectContent,
  SelectionMode,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Table,
  TableAvatar,
  Textarea,
  Toggle,
  ToggleGroup,
  ToolbarDivider,
  ToolbarGroup,
  Transaction,
  TransactionProvider,
  Tree,
  TreeContent,
  TreeItem,
  TreeStateProvider,
  useInternalNode,
  useReactFlow
} from "./elements";
import {
  addToSelection,
  clearSelectionDimension,
  getKitDiagramNodeFrameForKind,
  getKitDiagramShapeStrategy,
  KIT_DIAGRAM_COLLIDE_RADIUS,
  KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY,
  kitDiagramInferSnapSide,
  kitDiagramToAbsolutePoint,
  kitDiagramVector,
  normalizeKitDiagramFrame,
  removeFromSelection,
  replaceSelectionDimension,
  resolveKitDiagramAnchorPair,
  resolveKitDiagramProximityAnchor,
  toggleInSelection,
  type KitDiagramNodeKind,
  type KitDiagramShapeRenderPayload,
  type SelectionValue,
} from "./kitSelectionHelper";
import type { Device, HookNoSetResult, HookResult, KitAppId, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "./shared";
import {
  AppConfig,
  applySelectionComposition,
  AppPlugin,
  conditionalHookResult,
  createField,
  createPanelDefinition,
  createSingleKeyTransactionHandlers,
  Expertise,
  Field,
  isSelectionToolKind,
  Mode,
  PanelKind,
  parseWindowLayout,
  registerAppPlugin,
  registerEventHandler,
  registerKitAppHooks,
  registerSingleKeyAppEventHandlers,
  resolveSelectionCompositionKind,
  stringifyWindowLayout,
  Theme,
  ToolKind,
  toSelectionToolKind,
} from "./shared";

// #endregion Imports

// #region Design Family Helpers

// [👤semio📚js🗃️sketchpad💻kittsx🔖designfamilyhelpers](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/DESIGN-FAMILY-HELPERS)
// Design family helper functions MUST traverse the design hierarchy to collect related design GUIDs.

const getDesignFamilyGuids = (kit: Kit, designGuid: string): Set<string> => {
  const guids = new Set<string>();

  let currentGuid = designGuid;
  let current = kit.designs?.find((d) => d.guid === currentGuid);
  while (current?.parent?.guid) {
    const parent = kit.designs?.find((d) => d.guid === current!.parent!.guid);
    if (!parent) break;
    current = parent;
    currentGuid = parent.guid;
  }

  const collectDescendants = (parentGuid: string) => {
    guids.add(parentGuid);
    const children = (kit.designs || []).filter((d) => d.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(currentGuid);

  return guids;
};

// #endregion Design Family Helpers

// #region Constants

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT)
// Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.

const artifactKinds = ["designs", "types", "qualities", "ports", "tags", "concepts", "files", "folders", "authors"];

// #endregion Constants

// #region Internal State Management

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT)
// Internal state management MUST define all Kit app types, interfaces, store, and Y.js synchronization.

type YKitAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitApp = Y.Map<YKitAppVal>;
type YKitApps = Y.Map<YKitApp>;

/**
 * Tracks the current entity selection state across all artifact kinds for the Kit app.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION)
 **/
export interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  ports?: Guid[];
  tags?: Guid[];
  concepts?: Guid[];
  files?: string[];
  folders?: Guid[];
  authors?: string[];
}
const EMPTY_KIT_SELECTION: KitAppSelection = {};
/**
 * Diff for added/removed type GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectiontypesdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-TYPES-DIFF)
 **/
export interface KitAppSelectionTypesDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed design GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectiondesignsdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-DESIGNS-DIFF)
 **/
export interface KitAppSelectionDesignsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed quality strings in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionqualitiesdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-QUALITIES-DIFF)
 **/
export interface KitAppSelectionQualitiesDiff {
  added?: string[];
  removed?: string[];
}
/**
 * Diff for added/removed port GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionportsdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-PORTS-DIFF)
 **/
export interface KitAppSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed tag GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectiontagsdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-TAGS-DIFF)
 **/
export interface KitAppSelectionTagsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed concept GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionconceptsdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-CONCEPTS-DIFF)
 **/
export interface KitAppSelectionConceptsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed file strings in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionfilesdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-FILES-DIFF)
 **/
export interface KitAppSelectionFilesDiff {
  added?: string[];
  removed?: string[];
}
/**
 * Diff for added/removed folder GUIDs in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionfoldersdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-FOLDERS-DIFF)
 **/
export interface KitAppSelectionFoldersDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added/removed author strings in a Kit app selection change.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectionauthorsdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-AUTHORS-DIFF)
 **/
export interface KitAppSelectionAuthorsDiff {
  added?: string[];
  removed?: string[];
}
/**
 * Composite diff combining all artifact-kind selection changes for the Kit app.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappselectiondiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SELECTION-DIFF)
 **/
export interface KitAppSelectionDiff {
  types?: KitAppSelectionTypesDiff;
  designs?: KitAppSelectionDesignsDiff;
  qualities?: KitAppSelectionQualitiesDiff;
  ports?: KitAppSelectionPortsDiff;
  tags?: KitAppSelectionTagsDiff;
  concepts?: KitAppSelectionConceptsDiff;
  files?: KitAppSelectionFilesDiff;
  folders?: KitAppSelectionFoldersDiff;
  authors?: KitAppSelectionAuthorsDiff;
}
/**
 * Enumeration of window kinds available in the Kit app.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappwindowkind](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-WINDOW-KIND)
 **/
export enum KitAppWindowKind {
  Table = "table",
  Diagram = "diagram",
  Settings = "settings",
  Chat = "chat",
}
/**
 * Presence state for a Kit app user including cursor and camera.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitapppresence](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-PRESENCE)
 **/
export interface KitAppPresence {
  cursor?: Coord;
  camera?: Camera;
}
/**
 * Hover state tracking which single entity is hovered per artifact kind.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitapphover](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-HOVER)
 **/
export interface KitAppHover {
  type?: Guid;
  design?: Guid;
  quality?: Guid;
  port?: Guid;
  tag?: Guid;
  concept?: Guid;
  file?: Guid;
  folder?: Guid;
  author?: Guid;
}
/**
 * Extended presence for other Kit app collaborators including their display name.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitapppresenceother](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-PRESENCE-OTHER)
 **/
export interface KitAppPresenceOther extends KitAppPresence {
  name: string;
}
/**
 * Column identifier type for Kit app table sorting.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappsortcolumn](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SORT-COLUMN)
 **/
export type KitAppSortColumn = "artifact" | "kind" | "authors" | "updatedAt" | "createdAt";
/**
 * Sort direction type for Kit app table sorting.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappsortdirection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-SORT-DIRECTION)
 **/
export type KitAppSortDirection = "asc" | "desc";

/**
 * Configuration interface for Kit diagram force simulation parameters.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️diagramforcesettings](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/DIAGRAM-FORCE-SETTINGS)
 **/
export interface DiagramForceSettings {
  chargeStrength: number;
  linkDistance: number;
  collideRadius: number;
  centerStrength: number;
}

/**
 * Default force simulation settings for the Kit diagram layout.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🪨defaultdiagramforcesettings](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/DEFAULT-DIAGRAM-FORCE-SETTINGS)
 **/
export const defaultDiagramForceSettings: DiagramForceSettings = {
  chargeStrength: -15000,
  linkDistance: 400,
  collideRadius: KIT_DIAGRAM_COLLIDE_RADIUS * 1.5,
  centerStrength: 0.1,
};

/**
 * Complete diff describing all mutable Kit app state changes.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappdiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-DIFF)
 **/
export interface KitAppDiff {
  selection?: KitAppSelectionDiff;
  presence?: KitAppPresence;
  hover?: KitAppHover;
  fullscreenWindow?: KitAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  filterSearch?: string;
  expandedRows?: string[];
  sortColumn?: KitAppSortColumn;
  sortDirection?: KitAppSortDirection;
  windowLayout?: any;
  diagramForce?: Partial<DiagramForceSettings>;
  activeTool?: ToolKind;
}
/**
 * Edit record extending KitDiffAppEdit with Kit app selection diff.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappedit](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-EDIT)
 **/
export interface KitAppEdit extends KitDiffAppEdit<KitAppSelectionDiff> {}
/**
 * Complete runtime state for a Kit app instance.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappstate](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-STATE)
 **/
export interface KitAppState {
  fullscreenWindow: KitAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  selection?: KitAppSelection;
  hover?: KitAppHover;
  presence?: KitAppPresence;
  others: KitAppPresenceOther[];
  filterSearch: string;
  expandedRows: string[];
  sortColumn?: KitAppSortColumn;
  sortDirection?: KitAppSortDirection;
  windowLayout?: any;
  diagramForce: DiagramForceSettings;
  activeTool: ToolKind;
}

/**
 * Context passed to Kit app commands including the current app state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappcommandcontext](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-COMMAND-CONTEXT)
 **/
export interface KitAppCommandContext extends KitCommandContext {
  kitApp: KitAppState;
}
/**
 * Result returned by Kit app commands containing diffs to apply.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️kitappcommandresult](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-COMMAND-RESULT)
 **/
export interface KitAppCommandResult {
  diff?: KitAppDiff;
  kitDiff?: KitDiff;
}

/**
 * Computes the inverse of a Kit app selection diff for undo support.
 *
 * MUST return a diff that reverses the given selection diff across all artifact kinds.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🪨inversekitappselectiondiff](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/INVERSE-KIT-APP-SELECTION-DIFF)
 **/
export const inverseKitAppSelectionDiff = (selection: KitAppSelection, diff: KitAppSelectionDiff): KitAppSelectionDiff => {
  const inverseDiff: KitAppSelectionDiff = {};

  if (diff.types) {
    inverseDiff.types = {};
    if (diff.types.added) {
      inverseDiff.types.removed = diff.types.added;
    }
    if (diff.types.removed) {
      inverseDiff.types.added = diff.types.removed;
    }
  }

  if (diff.designs) {
    inverseDiff.designs = {};
    if (diff.designs.added) {
      inverseDiff.designs.removed = diff.designs.added;
    }
    if (diff.designs.removed) {
      inverseDiff.designs.added = diff.designs.removed;
    }
  }

  if (diff.qualities) {
    inverseDiff.qualities = {};
    if (diff.qualities.added) {
      inverseDiff.qualities.removed = diff.qualities.added;
    }
    if (diff.qualities.removed) {
      inverseDiff.qualities.added = diff.qualities.removed;
    }
  }

  if (diff.files) {
    inverseDiff.files = {};
    if (diff.files.added) {
      inverseDiff.files.removed = diff.files.added;
    }
    if (diff.files.removed) {
      inverseDiff.files.added = diff.files.removed;
    }
  }

  if (diff.folders) {
    inverseDiff.folders = {};
    if (diff.folders.added) {
      inverseDiff.folders.removed = diff.folders.added;
    }
    if (diff.folders.removed) {
      inverseDiff.folders.added = diff.folders.removed;
    }
  }

  if (diff.authors) {
    inverseDiff.authors = {};
    if (diff.authors.added) {
      inverseDiff.authors.removed = diff.authors.added;
    }
    if (diff.authors.removed) {
      inverseDiff.authors.added = diff.authors.removed;
    }
  }

  return inverseDiff;
};
/**
 * Checks whether two Kit app identifiers refer to the same kit.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🪨aresamekitapp](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ARE-SAME-KIT-APP)
 **/
export const areSameKitApp = (kitApp: KitAppId, other: KitAppId): boolean => kitApp.kit === other.kit;
/**
 * Checks whether a Kit app identifier matches any in a list.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🪨hassamekitapp](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/HAS-SAME-KIT-APP)
 **/
export const hasSameKitApp = (kitApp: KitAppId, others: KitAppId[]): boolean => others.some((other) => areSameKitApp(kitApp, other));

class KitStore extends KitDiffStore<KitAppState, KitAppDiff, KitAppSelectionDiff, KitAppEdit, KitAppCommandContext, KitAppCommandResult> {
  constructor(parent: SketchpadOrchestrator, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    yMap.set("kit", kit.guid);

    yMap.set("fullscreenWindow", state?.fullscreenWindow || KitAppFullscreenWindow.None);
    yMap.set("activeTool", state?.activeTool ?? ToolKind.SELECTION_NORMAL);
    
    if (state?.hover) {
      const hoverMap = new Y.Map<any>();
      if (state.hover.type) hoverMap.set("type", state.hover.type);
      if (state.hover.design) hoverMap.set("design", state.hover.design);
      if (state.hover.quality) hoverMap.set("quality", state.hover.quality);
      if (state.hover.port) hoverMap.set("port", state.hover.port);
      if (state.hover.tag) hoverMap.set("tag", state.hover.tag);
      if (state.hover.concept) hoverMap.set("concept", state.hover.concept);
      if (state.hover.file) hoverMap.set("file", state.hover.file);
      if (state.hover.folder) hoverMap.set("folder", state.hover.folder);
      if (state.hover.author) hoverMap.set("author", state.hover.author);
      yMap.set("hover", hoverMap);
    }

    if (state?.diagramForce) {
      const forceMap = new Y.Map<any>();
      forceMap.set("chargeStrength", state.diagramForce.chargeStrength ?? -200);
      forceMap.set("linkDistance", state.diagramForce.linkDistance ?? 100);
      forceMap.set("collideRadius", state.diagramForce.collideRadius ?? 0);
      yMap.set("diagramForce", forceMap);
    }

    const selection = new Y.Map<any>();
    const selectedTypes = new Y.Array<string>();
    if (state?.selection?.types?.length) {
      selectedTypes.push(state.selection.types);
    }
    const selectedDesigns = new Y.Array<string>();
    if (state?.selection?.designs?.length) {
      selectedDesigns.push(state.selection.designs);
    }
    const selectedQualities = new Y.Array<string>();
    if (state?.selection?.qualities?.length) {
      selectedQualities.push(state.selection.qualities);
    }
    const selectedFiles = new Y.Array<string>();
    if (state?.selection?.files?.length) {
      selectedFiles.push(state.selection.files);
    }
    const selectedFolders = new Y.Array<string>();
    if (state?.selection?.folders?.length) {
      selectedFolders.push(state.selection.folders);
    }
    const selectedAuthors = new Y.Array<string>();
    if (state?.selection?.authors?.length) {
      selectedAuthors.push(state.selection.authors);
    }
    selection.set("types", selectedTypes);
    selection.set("designs", selectedDesigns);
    selection.set("qualities", selectedQualities);
    selection.set("files", selectedFiles);
    selection.set("folders", selectedFolders);
    selection.set("authors", selectedAuthors);
    yMap.set("selection", selection);

    yMap.set("isTransactionActive", false);
    yMap.set("presence", new Y.Map<any>());
    yMap.set("others", new Y.Array<any>());
    yMap.set("diff", new Y.Map<any>());
    yMap.set("currentTransactionStack", new Y.Array<any>());
    yMap.set("pastTransactionsStack", new Y.Array<any>());

    yMap.set("filterSearch", state?.filterSearch || "");

    const expandedRows = new Y.Array<string>();
    if (state?.expandedRows) {
      expandedRows.push(state.expandedRows);
    }
    yMap.set("expandedRows", expandedRows);

    Object.entries(commands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenWindow(): KitAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as KitAppFullscreenWindow;
  }

  get activeTool(): ToolKind {
    return (this.yMap.get("activeTool") as ToolKind) ?? ToolKind.SELECTION_NORMAL;
  }

  get hover(): KitAppHover | undefined {
    const hoverMap = this.yMap.get("hover") as Y.Map<any>;
    if (!hoverMap) return undefined;
    return {
      type: hoverMap.get("type"),
      design: hoverMap.get("design"),
      quality: hoverMap.get("quality"),
      port: hoverMap.get("port"),
      tag: hoverMap.get("tag"),
      concept: hoverMap.get("concept"),
      file: hoverMap.get("file"),
      folder: hoverMap.get("folder"),
      author: hoverMap.get("author"),
    };
  }

  get diagramForce(): DiagramForceSettings {
    const forceMap = this.yMap.get("diagramForce") as Y.Map<any>;
    if (!forceMap) {
      return { chargeStrength: -200, linkDistance: 100, collideRadius: 0, centerStrength: 0.1 };
    }
    return {
      chargeStrength: forceMap.get("chargeStrength") ?? -200,
      linkDistance: forceMap.get("linkDistance") ?? 100,
      collideRadius: forceMap.get("collideRadius") ?? 0,
      centerStrength: forceMap.get("centerStrength") ?? 0.1,
    };
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: false,
        details: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? false,
      details: yPanelVisibility.get("details") ?? false,
    };
  }

  get selection(): KitAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: KitAppSelection = {};

    const types = selection.get("types") as Y.Array<string>;
    if (types && types.length > 0) {
      result.types = types.toArray();
    }

    const designs = selection.get("designs") as Y.Array<string>;
    if (designs && designs.length > 0) {
      result.designs = designs.toArray();
    }

    const qualities = selection.get("qualities") as Y.Array<string>;
    if (qualities && qualities.length > 0) {
      result.qualities = qualities.toArray();
    }

    const files = selection.get("files") as Y.Array<string>;
    if (files && files.length > 0) {
      result.files = files.toArray();
    }

    const folders = selection.get("folders") as Y.Array<string>;
    if (folders && folders.length > 0) {
      result.folders = folders.toArray();
    }

    const authors = selection.get("authors") as Y.Array<string>;
    if (authors && authors.length > 0) {
      result.authors = authors.toArray();
    }

    return result;
  }

  get presence(): KitAppPresence {
    return {
      cursor: {
        u: (this.yMap.get("presenceCursorX") as number) || 0,
        v: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }

  get others(): KitAppPresenceOther[] {
    return [];
  }

  get filterSearch(): string {
    return (this.yMap.get("filterSearch") as string) || "";
  }

  get expandedRows(): string[] {
    const yExpandedRows = this.yMap.get("expandedRows") as Y.Array<string>;
    return yExpandedRows ? yExpandedRows.toArray() : [];
  }

  get sortColumn(): KitAppSortColumn | undefined {
    return this.yMap.get("sortColumn") as KitAppSortColumn | undefined;
  }

  get sortDirection(): KitAppSortDirection | undefined {
    return this.yMap.get("sortDirection") as KitAppSortDirection | undefined;
  }

  get windowLayout(): any {
    return parseWindowLayout(this.yMap.get("windowLayout"));
  }

  set windowLayout(layout: any) {
    const value = stringifyWindowLayout(layout);
    if (value) {
      this.yMap.set("windowLayout", value);
    } else {
      this.yMap.delete("windowLayout");
    }
  }

  kit(): KitDataSource {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  protected getSelection(): KitAppSelection {
    return this.selection;
  }

  protected hash(state: KitAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): KitAppState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      activeTool: this.activeTool,
      hover: this.hover,
      panelVisibility: this.panelVisibility,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      canUndo: this.canUndo(),
      canRedo: this.canRedo(),
      presence: this.presence,
      others: this.others,
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
      filterSearch: this.filterSearch,
      expandedRows: this.expandedRows,
      sortColumn: this.sortColumn,
      sortDirection: this.sortDirection,
      windowLayout: this.windowLayout,
      diagramForce: this.diagramForce,
      transaction: {
        isTransactionActive: this.isTransactionActive,
        currentTransactionStack: this.currentTransactionStack,
        pastTransactionStack: this.pastTransactionsStack,
        redoStack: [],
      },
    } as any;
  }

  change = (diff: KitAppDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow !== undefined) {
        this.yMap.set("fullscreenWindow", diff.fullscreenWindow);
      }
      if (diff.activeTool !== undefined) {
        this.yMap.set("activeTool", diff.activeTool);
      }
      if (diff.hover !== undefined) {
        if (diff.hover === null) {
          this.yMap.delete("hover");
        } else {
          const hoverMap = this.yMap.get("hover") as Y.Map<any> || new Y.Map<any>();
          if (diff.hover.type !== undefined) hoverMap.set("type", diff.hover.type);
          if (diff.hover.design !== undefined) hoverMap.set("design", diff.hover.design);
          if (diff.hover.quality !== undefined) hoverMap.set("quality", diff.hover.quality);
          if (diff.hover.port !== undefined) hoverMap.set("port", diff.hover.port);
          if (diff.hover.tag !== undefined) hoverMap.set("tag", diff.hover.tag);
          if (diff.hover.concept !== undefined) hoverMap.set("concept", diff.hover.concept);
          if (diff.hover.file !== undefined) hoverMap.set("file", diff.hover.file);
          if (diff.hover.folder !== undefined) hoverMap.set("folder", diff.hover.folder);
          if (diff.hover.author !== undefined) hoverMap.set("author", diff.hover.author);
          this.yMap.set("hover", hoverMap);
        }
      }
      if (diff.diagramForce !== undefined) {
        const forceMap = this.yMap.get("diagramForce") as Y.Map<any> || new Y.Map<any>();
        if (diff.diagramForce.chargeStrength !== undefined) forceMap.set("chargeStrength", diff.diagramForce.chargeStrength);
        if (diff.diagramForce.linkDistance !== undefined) forceMap.set("linkDistance", diff.diagramForce.linkDistance);
        if (diff.diagramForce.collideRadius !== undefined) forceMap.set("collideRadius", diff.diagramForce.collideRadius);
        this.yMap.set("diagramForce", forceMap);
      }
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
      if (diff.filterSearch !== undefined) {
        this.yMap.set("filterSearch", diff.filterSearch);
      }
      if (diff.expandedRows !== undefined) {
        let yExpandedRows = this.yMap.get("expandedRows") as Y.Array<string>;
        if (!yExpandedRows) {
          yExpandedRows = new Y.Array<string>();
          this.yMap.set("expandedRows", yExpandedRows);
        }

        const currentRows = yExpandedRows.toArray();
        const newRows = diff.expandedRows;
        const currentSet = new Set(currentRows);
        const newSet = new Set(newRows);

        const toRemove: number[] = [];
        currentRows.forEach((row, index) => {
          if (!newSet.has(row)) {
            toRemove.push(index);
          }
        });

        for (let i = toRemove.length - 1; i >= 0; i--) {
          yExpandedRows.delete(toRemove[i], 1);
        }

        const toAdd = newRows.filter((row) => !currentSet.has(row));
        if (toAdd.length > 0) {
          yExpandedRows.push(toAdd);
        }
      }
      if (diff.sortColumn !== undefined) {
        this.yMap.set("sortColumn", diff.sortColumn);
      }
      if (diff.sortDirection !== undefined) {
        this.yMap.set("sortDirection", diff.sortDirection);
      }
      if (Object.prototype.hasOwnProperty.call(diff, "windowLayout")) {
        this.windowLayout = (diff as any).windowLayout;
      }
    });
  };

  protected inverseSelectionDiff(selection: KitAppSelection, diff: KitAppSelectionDiff): KitAppSelectionDiff {
    return inverseKitAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: KitAppSelectionDiff): void {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    if (selectionDiff.types) {
      let types = (selection.get("types") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("types")) {
        selection.set("types", types);
      }

      if (selectionDiff.types.added) {
        for (const type of selectionDiff.types.added) {
          if (!types.toArray().includes(type)) {
            types.push([type]);
          }
        }
      }
      if (selectionDiff.types.removed) {
        for (const type of selectionDiff.types.removed) {
          const index = types.toArray().indexOf(type);
          if (index !== -1) {
            types.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.designs) {
      let designs = (selection.get("designs") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("designs")) {
        selection.set("designs", designs);
      }

      if (selectionDiff.designs.added) {
        for (const design of selectionDiff.designs.added) {
          if (!designs.toArray().includes(design)) {
            designs.push([design]);
          }
        }
      }
      if (selectionDiff.designs.removed) {
        for (const design of selectionDiff.designs.removed) {
          const index = designs.toArray().indexOf(design);
          if (index !== -1) {
            designs.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.qualities) {
      let qualities = (selection.get("qualities") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("qualities")) {
        selection.set("qualities", qualities);
      }

      if (selectionDiff.qualities.added) {
        for (const quality of selectionDiff.qualities.added) {
          if (!qualities.toArray().includes(quality)) {
            qualities.push([quality]);
          }
        }
      }
      if (selectionDiff.qualities.removed) {
        for (const quality of selectionDiff.qualities.removed) {
          const index = qualities.toArray().indexOf(quality);
          if (index !== -1) {
            qualities.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.files) {
      let files = (selection.get("files") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("files")) {
        selection.set("files", files);
      }

      if (selectionDiff.files.added) {
        for (const file of selectionDiff.files.added) {
          if (!files.toArray().includes(file)) {
            files.push([file]);
          }
        }
      }
      if (selectionDiff.files.removed) {
        for (const file of selectionDiff.files.removed) {
          const index = files.toArray().indexOf(file);
          if (index !== -1) {
            files.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.folders) {
      let folders = (selection.get("folders") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("folders")) {
        selection.set("folders", folders);
      }

      if (selectionDiff.folders.added) {
        for (const folder of selectionDiff.folders.added) {
          if (!folders.toArray().includes(folder)) {
            folders.push([folder]);
          }
        }
      }
      if (selectionDiff.folders.removed) {
        for (const folder of selectionDiff.folders.removed) {
          const index = folders.toArray().indexOf(folder);
          if (index !== -1) {
            folders.delete(index, 1);
          }
        }
      }
    }

    if (selectionDiff.authors) {
      let authors = (selection.get("authors") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("authors")) {
        selection.set("authors", authors);
      }

      if (selectionDiff.authors.added) {
        for (const author of selectionDiff.authors.added) {
          if (!authors.toArray().includes(author)) {
            authors.push([author]);
          }
        }
      }
      if (selectionDiff.authors.removed) {
        for (const author of selectionDiff.authors.removed) {
          const index = authors.toArray().indexOf(author);
          if (index !== -1) {
            authors.delete(index, 1);
          }
        }
      }
    }
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

    if (command === "semio.kitApp.startTransaction") {
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.kitApp.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.kitApp.abortTransaction") {
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.kitApp.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.kitApp.redo") {
      this.redo();
      return {} as T;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit app controller`);

    const kitData = this.kit();
    const state = this.snapshot();

    const context: KitAppCommandContext = {
      kitApp: state,
      kit: kitData.snapshot(),
      fileUrls: kitData.fileUrls,
      origin,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitData.change(result.kitDiff);
    }
    this.recordEdit(result);
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

if (typeof window !== "undefined") {
  registerKitStoreFactory((parent, yMap, transact, id, state) => new KitStore(parent, yMap, transact, id, state as any));
}

// #region Kit App Plugin Registration

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖kitapppluginregistration](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/KIT-APP-PLUGIN-REGISTRATION)
// Kit app plugin registration MUST register the Kit app plugin with machine actions, guards, and default state.

const kitAppPlugin: AppPlugin = {
  id: "kit",
  namespace: "KIT",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: () => ({
      panelVisibility: { toolbar: true, details: false },
      selection: undefined,
      hover: undefined,
      fullscreenWindow: KitAppFullscreenWindow.None,
      others: [],
      filterSearch: "",
      expandedRows: new Set<string>(),
      sortColumn: undefined,
      sortDirection: undefined,
      windowLayout: undefined,
      diagramForce: { ...defaultDiagramForceSettings },
    }),
  },
  registerStores: () => {},
};

if (typeof window !== "undefined") {
  registerAppPlugin(kitAppPlugin);
  registerKitAppHooks({
    useKitAppCommands,
  });
  const kitAppEventConfig = {
    namespace: "KIT" as const,
    appKey: "kitApps" as const,
    keyField: "kitGuid",
    createDefaultState: createDefaultKitAppState,
  };
  registerSingleKeyAppEventHandlers(kitAppEventConfig);
  registerEventHandler("KIT.SET_FILTER", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, filterSearch: event.search } } };
    },
  });
  registerEventHandler("KIT.TOGGLE_ROW", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const expanded = new Set(app.expandedRows);
      if (expanded.has(event.rowId)) expanded.delete(event.rowId);
      else expanded.add(event.rowId);
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, expandedRows: expanded } } };
    },
  });
  registerEventHandler("KIT.SET_EXPANDED_ROWS", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, expandedRows: event.expandedRows } } };
    },
  });
  registerEventHandler("KIT.SET_SORT", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, sortColumn: event.column, sortDirection: event.direction } } };
    },
  });
  registerEventHandler("KIT.SELECT_TYPE", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const types = [...(app.selection?.types || [])];
      if (!types.includes(event.typeGuid)) types.push(event.typeGuid);
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
    },
  });
  registerEventHandler("KIT.DESELECT_TYPE", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const types = (app.selection?.types || []).filter((t: Guid) => t !== event.typeGuid);
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
    },
  });
  registerEventHandler("KIT.SELECT_DESIGN", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const designs = [...(app.selection?.designs || [])];
      if (!designs.includes(event.designGuid)) designs.push(event.designGuid);
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, designs } } } };
    },
  });
  registerEventHandler("KIT.DESELECT_DESIGN", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const designs = (app.selection?.designs || []).filter((d: Guid) => d !== event.designGuid);
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, designs } } } };
    },
  });
  registerEventHandler("KIT.SET_DIAGRAM_FORCE", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      const currentForce = app.diagramForce || { ...defaultDiagramForceSettings };
      const newForce = { ...currentForce, ...event.diagramForce };
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, diagramForce: newForce } } };
    },
  });
  registerEventHandler("KIT.SET_ACTIVE_TOOL", {
    action: (context: any, event: any) => {
      const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, activeTool: event.tool } } };
    },
  });

  registerEventHandler("KIT.INIT", {
    action: (context: any, event: any) => {
      return { kitApps: { ...context.kitApps, [event.kitGuid]: event.state } };
    },
  });

  createSingleKeyTransactionHandlers({
    namespace: "KIT",
    appKey: "kitApps",
    keyField: "kitGuid",
    createDefaultState: createDefaultKitAppState,
  });
}

// #endregion Kit App Plugin Registration

/**
 * Overload: returns the KitStore instance when no selector is provided.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappstore](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-STORE)
 **/
export function useKitAppStore(selector?: undefined, id?: KitAppId): KitStore | null;
/**
 * Overload: returns a derived value when a selector function is provided.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappstore](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-STORE)
 **/
export function useKitAppStore<T>(selector: (controller: KitStore) => T, id?: KitAppId): T | null;
/**
 * Selects derived state or the raw KitStore from the sketchpad orchestrator.
 *
 * MUST resolve the KitStore for the current kit scope and apply the optional selector.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappstore](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-STORE)
 **/
export function useKitAppStore<T>(selector?: (controller: KitStore) => T, id?: KitAppId): T | KitStore | null {
  const orchestrator = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) {
    return null;
  }
  try {
    if (!orchestrator || !orchestrator.hasKitApp({ kit: resolvedKitId })) {
      return null;
    }
    const kitStore = orchestrator.kitApp(resolvedKitId) as unknown as KitStore;
    const result = selector ? selector(kitStore) : kitStore;
    return result;
  } catch {
    return null;
  }
}

/**
 * Selects derived state from the Kit app XState snapshot.
 *
 * MUST use the sketchpad actor to reactively track the Kit app state slice.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitapp](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP)
 **/
export function useKitApp<T>(selector?: (state: KitAppState) => T, id?: KitAppId): T | KitAppState {
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? id?.kit;

  const defaultState: KitAppState = {
    panelVisibility: { toolbar: true, details: false },
    selection: undefined,
    hover: undefined,
    activeTool: ToolKind.SELECTION_NORMAL,
    fullscreenWindow: KitAppFullscreenWindow.None,
    filterSearch: "",
    expandedRows: [],
    sortColumn: "artifact",
    sortDirection: "asc",
    others: [],
    diagramForce: { ...defaultDiagramForceSettings },
  };

  if (!kitGuid) {
    if (selector) {
      return selector(defaultState) as T;
    }
    return defaultState;
  }

  const xstateState = useKitAppXState(kitGuid) as any;

  const state: KitAppState = {
    ...xstateState,
    expandedRows: xstateState.expandedRows ? Array.from(xstateState.expandedRows) : [],
  };

  if (selector) {
    return selector(state) as T;
  }
  return state;
}

/**
 * Returns a hook result for the current Kit app selection.
 *
 * MUST provide the current selection, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-SELECTION)
 **/
export function useKitAppSelection(): HookResult<KitAppSelection> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSelectionSelector(kitGuid), [kitGuid]);
  const selection = useSelector(actor, selector) ?? EMPTY_KIT_SELECTION;
  const canSetEvent = useMemo(() => ({ type: "KIT.SET_SELECTION" as const, kitGuid, selection: {} as KitAppSelection }), [kitGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setSelection = useMemo(() => {
    if (!canSet) return undefined;
    return (value: KitAppSelection) => {
      actor.send({ type: "KIT.SET_SELECTION", kitGuid, selection: value });
    };
  }, [actor, kitGuid, canSet]);
  return conditionalHookResult(canSet, selection, setSelection);
}

/**
 * Returns a hook result for the Kit app fullscreen window state.
 *
 * MUST provide the current fullscreen window, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappfullscreen](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-FULLSCREEN)
 **/
export function useKitAppFullscreen(): HookResult<KitAppFullscreenWindow> {
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const actor = useSketchpadActor();
  const selector = useMemo(() => createKitFullscreenSelector(kitGuid), [kitGuid]);
  const fullscreen = useSelector(actor, selector) ?? KitAppFullscreenWindow.None;
  const canSetEvent = useMemo(() => ({ type: "KIT.SET_FULLSCREEN" as const, kitGuid, window: KitAppFullscreenWindow.None }), [kitGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setFullscreen = useMemo(() => {
    if (!canSet) return undefined;
    return (value: KitAppFullscreenWindow) => {
      actor.send({ type: "KIT.SET_FULLSCREEN", kitGuid, window: value });
    };
  }, [actor, kitGuid, canSet]);
  return conditionalHookResult(canSet, fullscreen, setFullscreen);
}

/**
 * Returns other collaborators' presence state for the Kit app.
 *
 * MUST return a read-only list of other users' presence data.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappothers](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-OTHERS)
 **/
export function useKitAppOthers(): HookNoSetResult<KitAppPresenceOther[]> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitOthersSelector(kitGuid), [kitGuid]);
  const others = useSelector(actor, selector) ?? [];
  const canRead = kitScope !== null;
  return [others, undefined, canRead];
}

/**
 * Returns a hook result for the Kit app window layout.
 *
 * MUST provide the current window layout, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappwindowlayout](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-WINDOW-LAYOUT)
 **/
export function useKitAppWindowLayout(): HookResult<any> {
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const actor = useSketchpadActor();
  const selector = useMemo(() => createKitWindowLayoutSelector(kitGuid), [kitGuid]);
  const windowLayout = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "KIT.SET_WINDOW_LAYOUT" as const, kitGuid, windowLayout: {} }), [kitGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setWindowLayout = useMemo(() => {
    if (!canSet) return undefined;
    return (value: any) => {
      actor.send({ type: "KIT.SET_WINDOW_LAYOUT", kitGuid, windowLayout: value });
    };
  }, [actor, kitGuid, canSet]);
  return conditionalHookResult(canSet, windowLayout, setWindowLayout);
}

/**
 * Returns the Kit app diagram force settings with an updater.
 *
 * MUST provide the current force settings, an updater, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappdiagramforce](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-DIAGRAM-FORCE)
 **/
export function useKitAppDiagramForce(): readonly [DiagramForceSettings, ((value: Partial<DiagramForceSettings>) => void) | undefined, boolean] {
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const actor = useSketchpadActor();
  const selector = useMemo(() => createKitDiagramForceSelector(kitGuid), [kitGuid]);
  const force = useSelector(actor, selector) as DiagramForceSettings | undefined;
  const resolvedForce = force ?? defaultDiagramForceSettings;
  const canSet = !!kitGuid && !!actor;
  const setForce = useMemo(() => {
    if (!canSet || !kitGuid) return undefined;
    return (value: Partial<DiagramForceSettings>) => {
      actor.send({ type: "KIT.SET_DIAGRAM_FORCE", kitGuid, diagramForce: value } as any);
    };
  }, [kitGuid, canSet, actor]);
  return [resolvedForce, canSet ? setForce : undefined, canSet] as const;
}

/**
 * Returns a hook result for the Kit app active tool.
 *
 * MUST provide the current active tool, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappactivetool](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-ACTIVE-TOOL)
 **/
export function useKitAppActiveTool(): HookResult<ToolKind> {
  const actor = useSketchpadActor();
  const kitGuid = useKitScope()?.guid;

  const canSet = useSelector(actor, (snapshot) => {
    if (!kitGuid) return false;
    const app = snapshot.context.kitApps?.[kitGuid];
    return !!app;
  });

  const activeTool = useSelector(actor, (snapshot) => {
    if (!kitGuid) return ToolKind.SELECTION_NORMAL;
    return snapshot.context.kitApps?.[kitGuid]?.activeTool ?? ToolKind.SELECTION_NORMAL;
  });

  const setActiveTool = useMemo(() => {
    if (!canSet || !kitGuid) return undefined;
    return (tool: ToolKind) => actor.send({ type: "KIT.SET_ACTIVE_TOOL", kitGuid, tool } as any);
  }, [actor, canSet, kitGuid]);

  return conditionalHookResult(canSet, activeTool, setActiveTool);
}

/**
 * Returns a reactive field for the Kit app active tool.
 *
 * MUST create a Field wrapping the active tool value and setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappactivetoolfield](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-ACTIVE-TOOL-FIELD)
 **/
export function useKitAppActiveToolField(): Field<ToolKind> {
  const [value, setValue, canSet] = useKitAppActiveTool();
  return createField(value, setValue ?? (() => { }), canSet);
}

/**
 * Returns a read-only hook result for the Kit app sort column.
 *
 * MUST provide the current sort column from the XState snapshot.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappsortcolumn](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-SORT-COLUMN)
 **/
export function useKitAppSortColumn(): HookNoSetResult<string> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSortColumnSelector(kitGuid), [kitGuid]);
  const sortColumn = useSelector(actor, selector) ?? "artifact";
  const canRead = kitScope !== null;
  return [sortColumn, undefined, canRead];
}

/**
 * Returns a read-only hook result for the Kit app sort direction.
 *
 * MUST provide the current sort direction from the XState snapshot.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappsortdirection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-SORT-DIRECTION)
 **/
export function useKitAppSortDirection(): HookNoSetResult<"asc" | "desc"> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSortDirectionSelector(kitGuid), [kitGuid]);
  const sortDirection = useSelector(actor, selector) ?? "asc";
  const canRead = kitScope !== null;
  return [sortDirection, undefined, canRead];
}

/**
 * Returns a read-only hook result for the Kit app expanded rows.
 *
 * MUST provide the current expanded row set from the XState snapshot.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappexpandedrows](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-EXPANDED-ROWS)
 **/
export function useKitAppExpandedRows(): HookNoSetResult<Set<string>> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitExpandedRowsSelector(kitGuid), [kitGuid]);
  const expandedRows = useSelector(actor, selector) ?? new Set<string>();
  const canRead = kitScope !== null;
  return [expandedRows, undefined, canRead];
}

/**
 * Returns the Kit app transaction controller with start, finalize, and abort.
 *
 * MUST provide transaction actions dispatching to the XState actor.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitapptransaction](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-TRANSACTION)
 **/
export function useKitAppTransaction(): Transaction {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";

  if (!kitGuid) {
    return {};
  }
  return {
    start: () => actor.send({ type: "KIT.TRANSACTION.START", kitGuid }),
    finalize: () => actor.send({ type: "KIT.TRANSACTION.COMMIT", kitGuid }),
    abort: () => actor.send({ type: "KIT.TRANSACTION.ABORT", kitGuid }),
  };
}

/**
 * Returns the full Kit app commands API for programmatic access.
 *
 * MUST expose all Kit app commands through the store controller.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🛠️usekitappcommands](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/USE-KIT-APP-COMMANDS)
 **/
export function useKitAppCommands(id?: KitAppId) {
  const controller = useKitAppStore(undefined, id) as KitStore | null;
  const getOrigin = useOrigin();
  const noOp = () => {};
  if (!controller) {
    return {
      undo: noOp,
      redo: noOp,
      selectAll: noOp,
      deselectAll: noOp,
      selectType: noOp,
      selectTypes: noOp,
      addTypeToSelection: noOp,
      removeTypeFromSelection: noOp,
      selectDesign: noOp,
      selectDesigns: noOp,
      addDesignToSelection: noOp,
      removeDesignFromSelection: noOp,
      selectQuality: noOp,
      selectQualities: noOp,
      addQualityToSelection: noOp,
      removeQualityFromSelection: noOp,
      selectPort: noOp,
      selectPorts: noOp,
      addPortToSelection: noOp,
      removePortFromSelection: noOp,
      selectTag: noOp,
      selectTags: noOp,
      addTagToSelection: noOp,
      removeTagFromSelection: noOp,
      selectConcept: noOp,
      selectConcepts: noOp,
      addConceptToSelection: noOp,
      removeConceptFromSelection: noOp,
      selectFile: noOp,
      selectFiles: noOp,
      addFileToSelection: noOp,
      removeFileFromSelection: noOp,
      selectFolder: noOp,
      selectFolders: noOp,
      addFolderToSelection: noOp,
      removeFolderFromSelection: noOp,
      selectAuthor: noOp,
      selectAuthors: noOp,
      addAuthorToSelection: noOp,
      removeAuthorFromSelection: noOp,
      deleteSelected: noOp,
      toggleTypesFullscreen: noOp,
      toggleDesignsFullscreen: noOp,
      addType: noOp,
      addTypes: noOp,
      removeType: noOp,
      removeTypes: noOp,
      addDesign: noOp,
      addDesigns: noOp,
      removeDesign: noOp,
      removeDesigns: noOp,
      updateType: noOp,
      updateTypes: noOp,
      updateDesign: noOp,
      updateDesigns: noOp,
      togglePanel: noOp,
      setFilterSearch: noOp,
      setExpandedRows: noOp,
      toggleExpandedRow: noOp,
      setSortColumn: noOp,
      setSortDirection: noOp,
      toggleSort: noOp,
      execute: noOp,
    };
  }
  return {
    undo: () => controller.execute("semio.kitApp.undo", getOrigin()),
    redo: () => controller.execute("semio.kitApp.redo", getOrigin()),
    selectAll: () => controller.execute("semio.kitApp.selectAll", getOrigin()),
    deselectAll: () => controller.execute("semio.kitApp.deselectAll", getOrigin()),
    selectType: (guid: Guid) => controller.execute("semio.kitApp.selectType", getOrigin(), guid),
    selectTypes: (typeIds: Guid[]) => controller.execute("semio.kitApp.selectTypes", getOrigin(), typeIds),
    addTypeToSelection: (guid: Guid) => controller.execute("semio.kitApp.addTypeToSelection", getOrigin(), guid),
    removeTypeFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeTypeFromSelection", getOrigin(), guid),
    selectDesign: (guid: Guid) => controller.execute("semio.kitApp.selectDesign", getOrigin(), guid),
    selectDesigns: (designIds: Guid[]) => controller.execute("semio.kitApp.selectDesigns", getOrigin(), designIds),
    addDesignToSelection: (guid: Guid) => controller.execute("semio.kitApp.addDesignToSelection", getOrigin(), guid),
    removeDesignFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeDesignFromSelection", getOrigin(), guid),
    selectQuality: (key: string) => controller.execute("semio.kitApp.selectQuality", getOrigin(), key),
    selectQualities: (keys: string[]) => controller.execute("semio.kitApp.selectQualities", getOrigin(), keys),
    addQualityToSelection: (key: string) => controller.execute("semio.kitApp.addQualityToSelection", getOrigin(), key),
    removeQualityFromSelection: (key: string) => controller.execute("semio.kitApp.removeQualityFromSelection", getOrigin(), key),
    selectPort: (guid: Guid) => controller.execute("semio.kitApp.selectPort", getOrigin(), guid),
    selectPorts: (guids: Guid[]) => controller.execute("semio.kitApp.selectPorts", getOrigin(), guids),
    addPortToSelection: (guid: Guid) => controller.execute("semio.kitApp.addPortToSelection", getOrigin(), guid),
    removePortFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removePortFromSelection", getOrigin(), guid),
    selectTag: (guid: Guid) => controller.execute("semio.kitApp.selectTag", getOrigin(), guid),
    selectTags: (guids: Guid[]) => controller.execute("semio.kitApp.selectTags", getOrigin(), guids),
    addTagToSelection: (guid: Guid) => controller.execute("semio.kitApp.addTagToSelection", getOrigin(), guid),
    removeTagFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeTagFromSelection", getOrigin(), guid),
    selectConcept: (guid: Guid) => controller.execute("semio.kitApp.selectConcept", getOrigin(), guid),
    selectConcepts: (guids: Guid[]) => controller.execute("semio.kitApp.selectConcepts", getOrigin(), guids),
    addConceptToSelection: (guid: Guid) => controller.execute("semio.kitApp.addConceptToSelection", getOrigin(), guid),
    removeConceptFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeConceptFromSelection", getOrigin(), guid),
    selectFile: (path: string) => controller.execute("semio.kitApp.selectFile", getOrigin(), path),
    selectFiles: (paths: string[]) => controller.execute("semio.kitApp.selectFiles", getOrigin(), paths),
    addFileToSelection: (path: string) => controller.execute("semio.kitApp.addFileToSelection", getOrigin(), path),
    removeFileFromSelection: (path: string) => controller.execute("semio.kitApp.removeFileFromSelection", getOrigin(), path),
    selectFolder: (guid: Guid) => controller.execute("semio.kitApp.selectFolder", getOrigin(), guid),
    selectFolders: (guids: Guid[]) => controller.execute("semio.kitApp.selectFolders", getOrigin(), guids),
    addFolderToSelection: (guid: Guid) => controller.execute("semio.kitApp.addFolderToSelection", getOrigin(), guid),
    removeFolderFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeFolderFromSelection", getOrigin(), guid),
    selectAuthor: (name: string) => controller.execute("semio.kitApp.selectAuthor", getOrigin(), name),
    selectAuthors: (names: string[]) => controller.execute("semio.kitApp.selectAuthors", getOrigin(), names),
    addAuthorToSelection: (name: string) => controller.execute("semio.kitApp.addAuthorToSelection", getOrigin(), name),
    removeAuthorFromSelection: (name: string) => controller.execute("semio.kitApp.removeAuthorFromSelection", getOrigin(), name),
    deleteSelected: () => controller.execute("semio.kitApp.deleteSelected", getOrigin()),
    toggleTypesFullscreen: () => controller.execute("semio.kitApp.toggleTypesFullscreen", getOrigin()),
    toggleDesignsFullscreen: () => controller.execute("semio.kitApp.toggleDesignsFullscreen", getOrigin()),
    addType: (type: Type) => controller.execute("semio.kitApp.addType", getOrigin(), type),
    addTypes: (types: Type[]) => controller.execute("semio.kitApp.addTypes", getOrigin(), types),
    removeType: (guid: Guid) => controller.execute("semio.kitApp.removeType", getOrigin(), guid),
    removeTypes: (typeIds: Guid[]) => controller.execute("semio.kitApp.removeTypes", getOrigin(), typeIds),
    addDesign: (design: Design) => controller.execute("semio.kitApp.addDesign", getOrigin(), design),
    addDesigns: (designs: Design[]) => controller.execute("semio.kitApp.addDesigns", getOrigin(), designs),
    removeDesign: (guid: Guid) => controller.execute("semio.kitApp.removeDesign", getOrigin(), guid),
    removeDesigns: (designIds: Guid[]) => controller.execute("semio.kitApp.removeDesigns", getOrigin(), designIds),
    updateType: (guid: Guid, typeDiff: TypeDiff) => controller.execute("semio.kitApp.updateType", getOrigin(), guid, typeDiff),
    updateTypes: (updates: { type: { guid: Guid }; diff: TypeDiff }[]) => controller.execute("semio.kitApp.updateTypes", getOrigin(), updates),
    updateDesign: (guid: Guid, designDiff: DesignDiff) => controller.execute("semio.kitApp.updateDesign", getOrigin(), guid, designDiff),
    updateDesigns: (updates: { design: { guid: Guid }; diff: DesignDiff }[]) => controller.execute("semio.kitApp.updateDesigns", getOrigin(), updates),
    togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
      const current = controller.snapshot().panelVisibility;
      controller.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setFilterSearch: (search: string) => controller.execute("semio.kitApp.setFilterSearch", getOrigin(), search),
    setExpandedRows: (rows: string[]) => controller.execute("semio.kitApp.setExpandedRows", getOrigin(), rows),
    toggleExpandedRow: (rowId: string) => controller.execute("semio.kitApp.toggleExpandedRow", getOrigin(), rowId),
    setSortColumn: (column: KitAppSortColumn) => controller.execute("semio.kitApp.setSortColumn", getOrigin(), column),
    setSortDirection: (direction: KitAppSortDirection) => controller.execute("semio.kitApp.setSortDirection", getOrigin(), direction),
    toggleSort: (column: KitAppSortColumn) => controller.execute("semio.kitApp.toggleSort", getOrigin(), column),
    execute: (command: string, ...args: any[]) => controller.execute(command, getOrigin(), ...args),
  };
}

//#region Action Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS)
// Action hooks MUST provide composable React hooks for Kit app selection, hover, sort, filter, and transaction actions.

/**
 * Tuple type for action hook results pairing an action callback with a canAct flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️actionhookresult](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/ACTION-HOOK-RESULT)
 **/
export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

/**
 * Returns an action to select a single type in the Kit app.
 *
 * MUST return a callback that selects the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappselecttype](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SELECT-TYPE)
 **/
export function useKitAppSelectType(): ActionHookResult<[typeGuid: Guid]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SELECT_TYPE" as const, kitGuid, typeGuid: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (typeGuid: Guid) => actor.send({ type: "KIT.SELECT_TYPE", kitGuid, typeGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to deselect a single type in the Kit app.
 *
 * MUST return a callback that deselects the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappdeselecttype](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-DESELECT-TYPE)
 **/
export function useKitAppDeselectType(): ActionHookResult<[typeGuid: Guid]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.DESELECT_TYPE" as const, kitGuid, typeGuid: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (typeGuid: Guid) => actor.send({ type: "KIT.DESELECT_TYPE", kitGuid, typeGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select a single design in the Kit app.
 *
 * MUST return a callback that selects the given design GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappselectdesign](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SELECT-DESIGN)
 **/
export function useKitAppSelectDesign(): ActionHookResult<[designGuid: Guid]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SELECT_DESIGN" as const, kitGuid, designGuid: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (designGuid: Guid) => actor.send({ type: "KIT.SELECT_DESIGN", kitGuid, designGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to deselect a single design in the Kit app.
 *
 * MUST return a callback that deselects the given design GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappdeselectdesign](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-DESELECT-DESIGN)
 **/
export function useKitAppDeselectDesign(): ActionHookResult<[designGuid: Guid]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.DESELECT_DESIGN" as const, kitGuid, designGuid: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (designGuid: Guid) => actor.send({ type: "KIT.DESELECT_DESIGN", kitGuid, designGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to set the full Kit app selection.
 *
 * MUST return a callback that replaces the entire selection state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappsetselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SET-SELECTION)
 **/
export function useKitAppSetSelection(): ActionHookResult<[selection: KitAppSelection]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SET_SELECTION" as const, kitGuid, selection: {} as KitAppSelection }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (selection: KitAppSelection) => actor.send({ type: "KIT.SET_SELECTION", kitGuid, selection });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear the full Kit app selection.
 *
 * MUST return a callback that clears all selection state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappclearselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-CLEAR-SELECTION)
 **/
export function useKitAppClearSelection(): ActionHookResult<[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.CLEAR_SELECTION" as const, kitGuid }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return () => actor.send({ type: "KIT.CLEAR_SELECTION", kitGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

// #region Selection Helper Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS)
// Selection helper hooks MUST provide entity-specific add, remove, toggle, select-single, select-all, and clear operations.

function createDimensionSelectionHooks<K extends keyof KitAppSelection>(dimensionKey: K) {
  function useAdd(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = addToSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  function useRemove(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = removeFromSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  function useToggle(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = toggleInSelection(selection || {}, dimensionKey, value);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  function useSelectSingle(): ActionHookResult<[value: SelectionValue<K>]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return (value: SelectionValue<K>) => {
        const newSelection = replaceSelectionDimension(selection || {}, dimensionKey, [value] as KitAppSelection[K]);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  function useSelect(): ActionHookResult<[values: SelectionValue<K>[]]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return (values: SelectionValue<K>[]) => {
        const newSelection = replaceSelectionDimension(selection || {}, dimensionKey, values as KitAppSelection[K]);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  function useClear(): ActionHookResult<[]> {
    const [selection, setSelection] = useKitAppSelection();
    const canAct = setSelection !== undefined;
    const action = useMemo(() => {
      if (!canAct || !setSelection) return undefined;
      return () => {
        const newSelection = clearSelectionDimension(selection || {}, dimensionKey);
        setSelection(newSelection);
      };
    }, [selection, setSelection, canAct]);
    return [action, canAct];
  }

  return { useAdd, useRemove, useToggle, useSelectSingle, useSelect, useClear };
}

// #region Types Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS)
// Types selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for type selection.

/**
 * Returns an action to add a type to the Kit app selection.
 *
 * MUST return a callback that adds the given type GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitappaddtypetoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-ADD-TYPE-TO-SELECTION)
 **/
export function useKitAppAddTypeToSelection(): ActionHookResult<[typeGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (typeGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "types", typeGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a type from the Kit app selection.
 *
 * MUST return a callback that removes the given type GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitappremovetypefromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-REMOVE-TYPE-FROM-SELECTION)
 **/
export function useKitAppRemoveTypeFromSelection(): ActionHookResult<[typeGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (typeGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "types", typeGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a type in the Kit app selection.
 *
 * MUST return a callback that toggles the given type GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitapptoggletypeinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-TYPE-IN-SELECTION)
 **/
export function useKitAppToggleTypeInSelection(): ActionHookResult<[typeGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (typeGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "types", typeGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single type, clearing others.
 *
 * MUST return a callback that clears types and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitappselectsingletype](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-TYPE)
 **/
export function useKitAppSelectSingleType(): ActionHookResult<[typeGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (typeGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "types", [typeGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple types in the Kit app.
 *
 * MUST return a callback that selects the given type GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitappselecttypes](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-SELECT-TYPES)
 **/
export function useKitAppSelectTypes(): ActionHookResult<[typeGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (typeGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "types", typeGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all type selections.
 *
 * MUST return a callback that clears all type GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖typesselectionhooks🛠️usekitappcleartypes](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TYPES-SELECTION-HOOKS/USE-KIT-APP-CLEAR-TYPES)
 **/
export function useKitAppClearTypes(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "types");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Types Selection Hooks

// #region Designs Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS)
// Designs selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for design selection.

/**
 * Returns an action to add a design to the Kit app selection.
 *
 * MUST return a callback that adds the given design GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitappadddesigntoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-ADD-DESIGN-TO-SELECTION)
 **/
export function useKitAppAddDesignToSelection(): ActionHookResult<[designGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (designGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "designs", designGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a design from the Kit app selection.
 *
 * MUST return a callback that removes the given design GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitappremovedesignfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-DESIGN-FROM-SELECTION)
 **/
export function useKitAppRemoveDesignFromSelection(): ActionHookResult<[designGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (designGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "designs", designGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a design in the Kit app selection.
 *
 * MUST return a callback that toggles the given design GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitapptoggledesigninselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-DESIGN-IN-SELECTION)
 **/
export function useKitAppToggleDesignInSelection(): ActionHookResult<[designGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (designGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "designs", designGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single design, clearing others.
 *
 * MUST return a callback that clears designs and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitappselectsingledesign](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-DESIGN)
 **/
export function useKitAppSelectSingleDesign(): ActionHookResult<[designGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (designGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "designs", [designGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple designs in the Kit app.
 *
 * MUST return a callback that selects the given design GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitappselectdesigns](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-SELECT-DESIGNS)
 **/
export function useKitAppSelectDesigns(): ActionHookResult<[designsGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (designsGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "designs", designsGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all design selections.
 *
 * MUST return a callback that clears all design GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖designsselectionhooks🛠️usekitappcleardesigns](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/DESIGNS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-DESIGNS)
 **/
export function useKitAppClearDesigns(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "designs");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Designs Selection Hooks

// #region Qualities Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS)
// Qualities selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for quality selection.

/**
 * Returns an action to add a quality to the Kit app selection.
 *
 * MUST return a callback that adds the given quality string to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitappaddqualitytoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-ADD-QUALITY-TO-SELECTION)
 **/
export function useKitAppAddQualityToSelection(): ActionHookResult<[qualitie: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (qualitie: string) => {
      const newSelection = addToSelection(selection || {}, "qualities", qualitie);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a quality from the Kit app selection.
 *
 * MUST return a callback that removes the given quality string from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitappremovequalityfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-REMOVE-QUALITY-FROM-SELECTION)
 **/
export function useKitAppRemoveQualityFromSelection(): ActionHookResult<[qualitie: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (qualitie: string) => {
      const newSelection = removeFromSelection(selection || {}, "qualities", qualitie);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a quality in the Kit app selection.
 *
 * MUST return a callback that toggles the given quality string in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitapptogglequalityinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-QUALITY-IN-SELECTION)
 **/
export function useKitAppToggleQualityInSelection(): ActionHookResult<[qualitie: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (qualitie: string) => {
      const newSelection = toggleInSelection(selection || {}, "qualities", qualitie);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single quality, clearing others.
 *
 * MUST return a callback that clears qualities and selects the given string.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitappselectsinglequality](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-QUALITY)
 **/
export function useKitAppSelectSingleQuality(): ActionHookResult<[qualitie: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (qualitie: string) => {
      const newSelection = replaceSelectionDimension(selection || {}, "qualities", [qualitie]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple qualities in the Kit app.
 *
 * MUST return a callback that selects the given quality strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitappselectqualities](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-SELECT-QUALITIES)
 **/
export function useKitAppSelectQualities(): ActionHookResult<[qualitiesNames: string[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (qualitiesNames: string[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "qualities", qualitiesNames);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all quality selections.
 *
 * MUST return a callback that clears all quality strings from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖qualitiesselectionhooks🛠️usekitappclearqualities](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/QUALITIES-SELECTION-HOOKS/USE-KIT-APP-CLEAR-QUALITIES)
 **/
export function useKitAppClearQualities(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "qualities");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Qualities Selection Hooks

// #region Ports Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS)
// Ports selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for port selection.

/**
 * Returns an action to add a port to the Kit app selection.
 *
 * MUST return a callback that adds the given port GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitappaddporttoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-ADD-PORT-TO-SELECTION)
 **/
export function useKitAppAddPortToSelection(): ActionHookResult<[portGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (portGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "ports", portGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a port from the Kit app selection.
 *
 * MUST return a callback that removes the given port GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitappremoveportfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-PORT-FROM-SELECTION)
 **/
export function useKitAppRemovePortFromSelection(): ActionHookResult<[portGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (portGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "ports", portGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a port in the Kit app selection.
 *
 * MUST return a callback that toggles the given port GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitapptoggleportinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-PORT-IN-SELECTION)
 **/
export function useKitAppTogglePortInSelection(): ActionHookResult<[portGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (portGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "ports", portGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single port, clearing others.
 *
 * MUST return a callback that clears ports and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitappselectsingleport](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-PORT)
 **/
export function useKitAppSelectSinglePort(): ActionHookResult<[portGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (portGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "ports", [portGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple ports in the Kit app.
 *
 * MUST return a callback that selects the given port GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitappselectports](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-SELECT-PORTS)
 **/
export function useKitAppSelectPorts(): ActionHookResult<[portsGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (portsGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "ports", portsGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all port selections.
 *
 * MUST return a callback that clears all port GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖portsselectionhooks🛠️usekitappclearports](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/PORTS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-PORTS)
 **/
export function useKitAppClearPorts(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "ports");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Ports Selection Hooks

// #region Tags Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS)
// Tags selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for tag selection.

/**
 * Returns an action to add a tag to the Kit app selection.
 *
 * MUST return a callback that adds the given tag GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitappaddtagtoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-ADD-TAG-TO-SELECTION)
 **/
export function useKitAppAddTagToSelection(): ActionHookResult<[tagGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (tagGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "tags", tagGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a tag from the Kit app selection.
 *
 * MUST return a callback that removes the given tag GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitappremovetagfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-TAG-FROM-SELECTION)
 **/
export function useKitAppRemoveTagFromSelection(): ActionHookResult<[tagGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (tagGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "tags", tagGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a tag in the Kit app selection.
 *
 * MUST return a callback that toggles the given tag GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitapptoggletaginselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-TAG-IN-SELECTION)
 **/
export function useKitAppToggleTagInSelection(): ActionHookResult<[tagGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (tagGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "tags", tagGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single tag, clearing others.
 *
 * MUST return a callback that clears tags and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitappselectsingletag](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-TAG)
 **/
export function useKitAppSelectSingleTag(): ActionHookResult<[tagGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (tagGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "tags", [tagGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple tags in the Kit app.
 *
 * MUST return a callback that selects the given tag GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitappselecttags](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-SELECT-TAGS)
 **/
export function useKitAppSelectTags(): ActionHookResult<[tagsGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (tagsGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "tags", tagsGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all tag selections.
 *
 * MUST return a callback that clears all tag GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖tagsselectionhooks🛠️usekitappcleartags](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/TAGS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-TAGS)
 **/
export function useKitAppClearTags(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "tags");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Tags Selection Hooks

// #region Concepts Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS)
// Concepts selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for concept selection.

/**
 * Returns an action to add a concept to the Kit app selection.
 *
 * MUST return a callback that adds the given concept GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitappaddconcepttoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-ADD-CONCEPT-TO-SELECTION)
 **/
export function useKitAppAddConceptToSelection(): ActionHookResult<[conceptGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (conceptGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "concepts", conceptGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a concept from the Kit app selection.
 *
 * MUST return a callback that removes the given concept GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitappremoveconceptfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-CONCEPT-FROM-SELECTION)
 **/
export function useKitAppRemoveConceptFromSelection(): ActionHookResult<[conceptGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (conceptGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "concepts", conceptGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a concept in the Kit app selection.
 *
 * MUST return a callback that toggles the given concept GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitapptoggleconceptinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-CONCEPT-IN-SELECTION)
 **/
export function useKitAppToggleConceptInSelection(): ActionHookResult<[conceptGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (conceptGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "concepts", conceptGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single concept, clearing others.
 *
 * MUST return a callback that clears concepts and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitappselectsingleconcept](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-CONCEPT)
 **/
export function useKitAppSelectSingleConcept(): ActionHookResult<[conceptGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (conceptGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "concepts", [conceptGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple concepts in the Kit app.
 *
 * MUST return a callback that selects the given concept GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitappselectconcepts](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-SELECT-CONCEPTS)
 **/
export function useKitAppSelectConcepts(): ActionHookResult<[conceptsGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (conceptsGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "concepts", conceptsGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all concept selections.
 *
 * MUST return a callback that clears all concept GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖conceptsselectionhooks🛠️usekitappclearconcepts](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/CONCEPTS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-CONCEPTS)
 **/
export function useKitAppClearConcepts(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "concepts");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Concepts Selection Hooks

// #region Files Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS)
// Files selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for file selection.

/**
 * Returns an action to add a file to the Kit app selection.
 *
 * MUST return a callback that adds the given file string to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitappaddfiletoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-ADD-FILE-TO-SELECTION)
 **/
export function useKitAppAddFileToSelection(): ActionHookResult<[file: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (file: string) => {
      const newSelection = addToSelection(selection || {}, "files", file);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a file from the Kit app selection.
 *
 * MUST return a callback that removes the given file string from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitappremovefilefromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-REMOVE-FILE-FROM-SELECTION)
 **/
export function useKitAppRemoveFileFromSelection(): ActionHookResult<[file: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (file: string) => {
      const newSelection = removeFromSelection(selection || {}, "files", file);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a file in the Kit app selection.
 *
 * MUST return a callback that toggles the given file string in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitapptogglefileinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-FILE-IN-SELECTION)
 **/
export function useKitAppToggleFileInSelection(): ActionHookResult<[file: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (file: string) => {
      const newSelection = toggleInSelection(selection || {}, "files", file);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single file, clearing others.
 *
 * MUST return a callback that clears files and selects the given string.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitappselectsinglefile](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-FILE)
 **/
export function useKitAppSelectSingleFile(): ActionHookResult<[file: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (file: string) => {
      const newSelection = replaceSelectionDimension(selection || {}, "files", [file]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple files in the Kit app.
 *
 * MUST return a callback that selects the given file strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitappselectfiles](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-SELECT-FILES)
 **/
export function useKitAppSelectFiles(): ActionHookResult<[filesNames: string[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (filesNames: string[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "files", filesNames);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all file selections.
 *
 * MUST return a callback that clears all file strings from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖filesselectionhooks🛠️usekitappclearfiles](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FILES-SELECTION-HOOKS/USE-KIT-APP-CLEAR-FILES)
 **/
export function useKitAppClearFiles(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "files");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Files Selection Hooks

// #region Folders Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS)
// Folders selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for folder selection.

/**
 * Returns an action to add a folder to the Kit app selection.
 *
 * MUST return a callback that adds the given folder GUID to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitappaddfoldertoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-ADD-FOLDER-TO-SELECTION)
 **/
export function useKitAppAddFolderToSelection(): ActionHookResult<[folderGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (folderGuid: Guid) => {
      const newSelection = addToSelection(selection || {}, "folders", folderGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove a folder from the Kit app selection.
 *
 * MUST return a callback that removes the given folder GUID from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitappremovefolderfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-FOLDER-FROM-SELECTION)
 **/
export function useKitAppRemoveFolderFromSelection(): ActionHookResult<[folderGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (folderGuid: Guid) => {
      const newSelection = removeFromSelection(selection || {}, "folders", folderGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a folder in the Kit app selection.
 *
 * MUST return a callback that toggles the given folder GUID in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitapptogglefolderinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-FOLDER-IN-SELECTION)
 **/
export function useKitAppToggleFolderInSelection(): ActionHookResult<[folderGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (folderGuid: Guid) => {
      const newSelection = toggleInSelection(selection || {}, "folders", folderGuid);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single folder, clearing others.
 *
 * MUST return a callback that clears folders and selects the given GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitappselectsinglefolder](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-FOLDER)
 **/
export function useKitAppSelectSingleFolder(): ActionHookResult<[folderGuid: Guid]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (folderGuid: Guid) => {
      const newSelection = replaceSelectionDimension(selection || {}, "folders", [folderGuid]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple folders in the Kit app.
 *
 * MUST return a callback that selects the given folder GUIDs.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitappselectfolders](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-SELECT-FOLDERS)
 **/
export function useKitAppSelectFolders(): ActionHookResult<[foldersGuids: Guid[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (foldersGuids: Guid[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "folders", foldersGuids);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all folder selections.
 *
 * MUST return a callback that clears all folder GUIDs from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖foldersselectionhooks🛠️usekitappclearfolders](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/FOLDERS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-FOLDERS)
 **/
export function useKitAppClearFolders(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "folders");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Folders Selection Hooks

// #region Authors Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS)
// Authors selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for author selection.

/**
 * Returns an action to add an author to the Kit app selection.
 *
 * MUST return a callback that adds the given author string to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitappaddauthortoselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-ADD-AUTHOR-TO-SELECTION)
 **/
export function useKitAppAddAuthorToSelection(): ActionHookResult<[author: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (author: string) => {
      const newSelection = addToSelection(selection || {}, "authors", author);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to remove an author from the Kit app selection.
 *
 * MUST return a callback that removes the given author string from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitappremoveauthorfromselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-REMOVE-AUTHOR-FROM-SELECTION)
 **/
export function useKitAppRemoveAuthorFromSelection(): ActionHookResult<[author: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (author: string) => {
      const newSelection = removeFromSelection(selection || {}, "authors", author);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle an author in the Kit app selection.
 *
 * MUST return a callback that toggles the given author string in selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitapptoggleauthorinselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-TOGGLE-AUTHOR-IN-SELECTION)
 **/
export function useKitAppToggleAuthorInSelection(): ActionHookResult<[author: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (author: string) => {
      const newSelection = toggleInSelection(selection || {}, "authors", author);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select only a single author, clearing others.
 *
 * MUST return a callback that clears authors and selects the given string.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitappselectsingleauthor](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-SELECT-SINGLE-AUTHOR)
 **/
export function useKitAppSelectSingleAuthor(): ActionHookResult<[author: string]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (author: string) => {
      const newSelection = replaceSelectionDimension(selection || {}, "authors", [author]);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to select multiple authors in the Kit app.
 *
 * MUST return a callback that selects the given author strings.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitappselectauthors](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-SELECT-AUTHORS)
 **/
export function useKitAppSelectAuthors(): ActionHookResult<[authorsNames: string[]]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return (authorsNames: string[]) => {
      const newSelection = replaceSelectionDimension(selection || {}, "authors", authorsNames);
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear all author selections.
 *
 * MUST return a callback that clears all author strings from selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖authorsselectionhooks🛠️usekitappclearauthors](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/AUTHORS-SELECTION-HOOKS/USE-KIT-APP-CLEAR-AUTHORS)
 **/
export function useKitAppClearAuthors(): ActionHookResult<[]> {
  const [selection, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection) return undefined;
    return () => {
      const newSelection = clearSelectionDimension(selection || {}, "authors");
      setSelection(newSelection);
    };
  }, [selection, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Authors Selection Hooks

// #region Global Selection Hooks

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖globalselectionhooks](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/GLOBAL-SELECTION-HOOKS)
// Global selection hooks MUST provide select-all across all artifact kinds.

/**
 * Returns an action to select all entities across all artifact kinds.
 *
 * MUST return a callback that adds all artifact GUIDs to selection.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🔖selectionhelperhooks🔖globalselectionhooks🛠️usekitappselectall](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/SELECTION-HELPER-HOOKS/GLOBAL-SELECTION-HOOKS/USE-KIT-APP-SELECT-ALL)
 **/
export function useKitAppSelectAll(): ActionHookResult<[]> {
  const kit = useKit() as Kit | undefined;
  const [, setSelection] = useKitAppSelection();
  const canAct = setSelection !== undefined && kit !== null && kit !== undefined;
  const action = useMemo(() => {
    if (!canAct || !setSelection || !kit) return undefined;
    return () => {
      const allSelection: KitAppSelection = {};
      const types = kit.types?.map((t: Type) => t.guid);
      const designs = kit.designs?.map((d: Design) => d.guid);
      const qualities = kit.qualities?.map((q: Quality) => q.name);
      const ports = kit.ports?.map((p: Port) => p.guid);
      const tags = kit.tags?.map((t: Tag) => t.guid);
      const concepts = kit.concepts?.map((c: Concept) => c.guid);
      const files = kit.files?.map((f: SemioFile) => f.name);
      const folders = kit.folders?.map((f: Folder) => f.guid);
      const authors = kit.authors?.map((a: Author) => a.name);
      
      if (types && types.length > 0) allSelection.types = types;
      if (designs && designs.length > 0) allSelection.designs = designs;
      if (qualities && qualities.length > 0) allSelection.qualities = qualities;
      if (ports && ports.length > 0) allSelection.ports = ports;
      if (tags && tags.length > 0) allSelection.tags = tags;
      if (concepts && concepts.length > 0) allSelection.concepts = concepts;
      if (files && files.length > 0) allSelection.files = files;
      if (folders && folders.length > 0) allSelection.folders = folders;
      if (authors && authors.length > 0) allSelection.authors = authors;
      
      setSelection(allSelection);
    };
  }, [kit, setSelection, canAct]);
  return [action, canAct];
}

// #endregion Global Selection Hooks

// #endregion Selection Helper Hooks

/**
 * Returns an action to set the Kit app filter search query.
 *
 * MUST return a callback that sets the filter search string.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappsetfilter](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SET-FILTER)
 **/
export function useKitAppSetFilter(): ActionHookResult<[search: string]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SET_FILTER" as const, kitGuid, search: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (search: string) => actor.send({ type: "KIT.SET_FILTER", kitGuid, search });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a row's expanded state in the Kit table.
 *
 * MUST return a callback that toggles the given row GUID in expanded rows.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitapptogglerow](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-TOGGLE-ROW)
 **/
export function useKitAppToggleRow(): ActionHookResult<[rowId: string]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.TOGGLE_ROW" as const, kitGuid, rowId: "" }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (rowId: string) => actor.send({ type: "KIT.TOGGLE_ROW", kitGuid, rowId });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to set the Kit table sort column.
 *
 * MUST return a callback that sets the sort column identifier.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappsetsort](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SET-SORT)
 **/
export function useKitAppSetSort(): ActionHookResult<[column: string, direction: "asc" | "desc"]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SET_SORT" as const, kitGuid, column: "", direction: "asc" as const }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (column: string, direction: "asc" | "desc") => actor.send({ type: "KIT.SET_SORT", kitGuid, column, direction });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle the Kit table sort direction.
 *
 * MUST return a callback that toggles between ascending and descending.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitapptogglesort](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-TOGGLE-SORT)
 **/
export function useKitAppToggleSort(): ActionHookResult<[column: KitAppSortColumn]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const [selection] = useKitAppSelection();
  const kitApp = useKitAppXState(kitGuid);
  const sortColumn = kitApp?.sortColumn;
  const sortDirection = kitApp?.sortDirection;
  const canActEvent = useMemo(() => ({ type: "KIT.SET_SORT" as const, kitGuid, column: "", direction: "asc" as const }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (column: KitAppSortColumn) => {
      const newDirection = sortColumn === column && sortDirection === "asc" ? "desc" : "asc";
      actor.send({ type: "KIT.SET_SORT", kitGuid, column, direction: newDirection });
    };
  }, [actor, kitGuid, canAct, sortColumn, sortDirection]);
  return [action, canAct];
}

/**
 * Returns a hook result for the Kit app hover state.
 *
 * MUST provide the current hover, a setter, and a canSet flag.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitapphover](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-HOVER)
 **/
export function useKitAppHover(): HookNoSetResult<KitAppHover | undefined> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitHoverSelector(kitGuid), [kitGuid]);
  const hover = useSelector(actor, selector);
  const canRead = kitScope !== null;
  return [hover, undefined, canRead];
}

/**
 * Returns an action to set the Kit app hover state.
 *
 * MUST return a callback that sets hover to the given entity.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappsethover](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-SET-HOVER)
 **/
export function useKitAppSetHover(): ActionHookResult<[hover: KitAppHover]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SET_HOVER" as const, kitGuid, hover: {} as KitAppHover }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (hover: KitAppHover) => actor.send({ type: "KIT.SET_HOVER", kitGuid, hover });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to clear the Kit app hover state.
 *
 * MUST return a callback that clears all hover state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitappclearhover](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-CLEAR-HOVER)
 **/
export function useKitAppClearHover(): ActionHookResult<[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.CLEAR_HOVER" as const, kitGuid }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return () => actor.send({ type: "KIT.CLEAR_HOVER", kitGuid });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

/**
 * Returns an action to toggle a specific panel's visibility.
 *
 * MUST return a callback that toggles the given panel's visibility.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖internalstatemanagement🔖actionhooks🛠️usekitapptogglepanel](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/INTERNAL-STATE-MANAGEMENT/ACTION-HOOKS/USE-KIT-APP-TOGGLE-PANEL)
 **/
export function useKitAppTogglePanel(): ActionHookResult<[panel: keyof PanelVisibility]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.TOGGLE_PANEL" as const, kitGuid, panel: "toolbar" as keyof PanelVisibility }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (panel: keyof PanelVisibility) => actor.send({ type: "KIT.TOGGLE_PANEL", kitGuid, panel });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

//#endregion Action Hooks

// #endregion Internal State Management

// #region Types

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖types](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/TYPES)
// Types MUST provide hover status and color hooks for type visual indication in the Kit app.

/**
 * Returns whether a type is currently hovered in the Kit app.
 *
 * MUST check the hover state for the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖types🛠️usekitappistypehovered](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/TYPES/USE-KIT-APP-IS-TYPE-HOVERED)
 **/
export function useKitAppIsTypeHovered(): HookNoSetResult<boolean> {
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid;
  const isHovered = useKitApp((state) => (typeGuid ? state.hover?.type === typeGuid : false)) as boolean;
  const canRead = typeScope !== null;
  return [isHovered ?? false, undefined, canRead];
}

/**
 * Returns the selection/hover status of a type for visual indication.
 *
 * MUST derive status from selection and hover states for the given type GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖types🛠️usekitapptypestatus](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/TYPES/USE-KIT-APP-TYPE-STATUS)
 **/
export function useKitAppTypeStatus(): HookNoSetResult<DiffStatus> {
  const typeScope = useTypeScope();
  const canRead = typeScope !== null;
  return [DiffStatus.Unchanged, undefined, canRead];
}

/**
 * Returns the computed color for a type based on its status.
 *
 * MUST derive the color from the type's hovered and selected state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖types🛠️usekitapptypecolor](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/TYPES/USE-KIT-APP-TYPE-COLOR)
 **/
export function useKitAppTypeColor(isSelected: boolean): HookNoSetResult<{ fill: string; stroke: string; opacity: number }> {
  const typeScope = useTypeScope();
  const [isHovered] = useKitAppIsTypeHovered();
  const [status] = useKitAppTypeStatus();
  const canRead = typeScope !== null;

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

  return [{ fill, stroke, opacity }, undefined, canRead];
}

// #endregion Types

// #region Designs

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖designs](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/DESIGNS)
// Designs MUST provide hover status and color hooks for design visual indication in the Kit app.

/**
 * Returns whether a design is currently hovered in the Kit app.
 *
 * MUST check the hover state for the given design GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖designs🛠️usekitappisdesignhovered](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/DESIGNS/USE-KIT-APP-IS-DESIGN-HOVERED)
 **/
export function useKitAppIsDesignHovered(): HookNoSetResult<boolean> {
  const designScope = useDesignScope();
  const designGuid = designScope?.guid;
  const isHovered = useKitApp((state) => (designGuid ? state.hover?.design === designGuid : false)) as boolean;
  const canRead = designScope !== null;
  return [isHovered ?? false, undefined, canRead];
}

/**
 * Returns the selection/hover status of a design for visual indication.
 *
 * MUST derive status from selection and hover states for the given design GUID.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖designs🛠️usekitappdesignstatus](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/DESIGNS/USE-KIT-APP-DESIGN-STATUS)
 **/
export function useKitAppDesignStatus(): HookNoSetResult<DiffStatus> {
  const designScope = useDesignScope();
  const canRead = designScope !== null;
  return [DiffStatus.Unchanged, undefined, canRead];
}

/**
 * Returns the computed color for a design based on its status.
 *
 * MUST derive the color from the design's hovered and selected state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖designs🛠️usekitappdesigncolor](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/DESIGNS/USE-KIT-APP-DESIGN-COLOR)
 **/
export function useKitAppDesignColor(isSelected: boolean): HookNoSetResult<{ fill: string; stroke: string; opacity: number }> {
  const designScope = useDesignScope();
  const [isHovered] = useKitAppIsDesignHovered();
  const [status] = useKitAppDesignStatus();
  const canRead = designScope !== null;

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

  return [{ fill, stroke, opacity }, undefined, canRead];
}

// #endregion Designs

// #region Commands

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖commands](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/COMMANDS)
// Commands MUST define all executable Kit app actions for artifact CRUD, import, and export.

/**
 * Registry of all named Kit app commands mapped to their handler functions.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖commands🪨commands](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/COMMANDS/COMMANDS)
 **/
export const commands = {
  "semio.kitApp.setTheme": (context: KitAppCommandContext, theme: Theme): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.setDevice": (context: KitAppCommandContext, device: Device): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.toggleTableFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Table ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Table;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitApp.toggleDiagramFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Diagram ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Diagram;
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
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          tags: { removed: currentSelection?.tags ?? [] },
          concepts: { removed: currentSelection?.concepts ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
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
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
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
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
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
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
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
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
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
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
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
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
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
  "semio.kitApp.selectPort": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: {
            removed: currentSelection?.ports ?? [],
            added: [guid],
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectPorts": (context: KitAppCommandContext, guids: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: {
            removed: currentSelection?.ports ?? [],
            added: guids,
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addPortToSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          ports: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removePortFromSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          ports: { removed: [guid] },
        },
      },
    };
  },
  "semio.kitApp.selectTag": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          tags: {
            removed: currentSelection?.tags ?? [],
            added: [guid],
          },
          concepts: { removed: currentSelection?.concepts ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectTags": (context: KitAppCommandContext, guids: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          tags: {
            removed: currentSelection?.tags ?? [],
            added: guids,
          },
          concepts: { removed: currentSelection?.concepts ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addTagToSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          tags: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removeTagFromSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          tags: { removed: [guid] },
        },
      },
    };
  },
  "semio.kitApp.selectConcept": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          tags: { removed: currentSelection?.tags ?? [] },
          concepts: {
            removed: currentSelection?.concepts ?? [],
            added: [guid],
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectConcepts": (context: KitAppCommandContext, guids: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          tags: { removed: currentSelection?.tags ?? [] },
          concepts: {
            removed: currentSelection?.concepts ?? [],
            added: guids,
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addConceptToSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          concepts: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removeConceptFromSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          concepts: { removed: [guid] },
        },
      },
    };
  },
  "semio.kitApp.selectFile": (context: KitAppCommandContext, guid: string): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: {
            removed: currentSelection?.files ?? [],
            added: [guid],
          },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectFiles": (context: KitAppCommandContext, guids: string[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: {
            removed: currentSelection?.files ?? [],
            added: guids,
          },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addFileToSelection": (context: KitAppCommandContext, guid: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          files: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removeFileFromSelection": (context: KitAppCommandContext, guid: string): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          files: { removed: [guid] },
        },
      },
    };
  },
  "semio.kitApp.selectFolder": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: {
            removed: currentSelection?.folders ?? [],
            added: [guid],
          },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectFolders": (context: KitAppCommandContext, guids: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: {
            removed: currentSelection?.folders ?? [],
            added: guids,
          },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addFolderToSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          folders: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removeFolderFromSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          folders: { removed: [guid] },
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
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
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
          ports: { removed: currentSelection?.ports ?? [] },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
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
        types: { removed: selection?.types?.map((g) => ({ guid: g })) },
        designs: { removed: selection?.designs?.map((g) => ({ guid: g })) },
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
        types: { removed: [{ guid: Guid }] },
      },
    };
  },
  "semio.kitApp.removeTypes": (context: KitAppCommandContext, typeIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: typeIds.map((g) => ({ guid: g })) },
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
        designs: { removed: [{ guid: Guid }] },
      },
    };
  },
  "semio.kitApp.removeDesigns": (context: KitAppCommandContext, designIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: designIds.map((g) => ({ guid: g })) },
      },
    };
  },
  "semio.kitApp.updateType": (context: KitAppCommandContext, guid: Guid, typeDiff: TypeDiff): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: [{ type: { guid }, diff: typeDiff }] },
      },
    };
  },
  "semio.kitApp.updateTypes": (context: KitAppCommandContext, updates: { type: { guid: Guid }; diff: TypeDiff }[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: updates },
      },
    };
  },
  "semio.kitApp.updateDesign": (context: KitAppCommandContext, guid: Guid, designDiff: DesignDiff): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: [{ design: { guid }, diff: designDiff }] },
      },
    };
  },
  "semio.kitApp.updateDesigns": (context: KitAppCommandContext, updates: { design: { guid: Guid }; diff: DesignDiff }[]): KitAppCommandResult => {
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

// #endregion Commands

// #region Canvas

// [🔖semio/js/sketchpad/Kit.tsx#Canvas](semiorepo://section/semio/js/sketchpad/Kit.tsx/CANVAS)

// #region Windows

// [🔖semio/js/sketchpad/Kit.tsx#Windows](semiorepo://section/semio/js/sketchpad/Kit.tsx/WINDOWS)

// #region Table

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS)
// Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.

type ArtifactKind = "designs" | "types" | "qualities" | "ports" | "tags" | "concepts" | "files" | "folders" | "authors";

const KitKindToggles: FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const selectedKindsFromUrl = useMemo(() => searchParams.getAll("kind") as ArtifactKind[], [searchParams]);
  const selectedKinds = useMemo(() => new Set(selectedKindsFromUrl), [selectedKindsFromUrl]);

  const toggleKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    const kinds = newParams.getAll("kind") as ArtifactKind[];

    if (kinds.length === 0) {
      newParams.append("kind", kind);
    } else if (kinds.includes(kind)) {
      const remaining = kinds.filter((k) => k !== kind);
      newParams.delete("kind");
      remaining.forEach((k) => newParams.append("kind", k));
    } else {
      newParams.append("kind", kind);
    }

    newParams.delete("name");
    newParams.delete("variant");
    newParams.delete("view");
    setSearchParams(newParams);
  };

  const labelDesigns = useLabel("semio.sketchpad.app.kit.toolbar.showDesigns");
  const labelTypes = useLabel("semio.sketchpad.app.kit.toolbar.showTypes");
  const labelQualities = useLabel("semio.sketchpad.app.kit.toolbar.showQualities");
  const labelPorts = useLabel("semio.sketchpad.app.kit.toolbar.showPorts");
  const labelTags = useLabel("semio.sketchpad.app.kit.toolbar.showTags");
  const labelConcepts = useLabel("semio.sketchpad.app.kit.toolbar.showConcepts");
  const labelFiles = useLabel("semio.sketchpad.app.kit.toolbar.showFiles");
  const labelFolders = useLabel("semio.sketchpad.app.kit.toolbar.showFolders");
  const labelAuthors = useLabel("semio.sketchpad.app.kit.toolbar.showAuthors");

  return (
    <ToolbarGroup>
      <Toggle pressed={selectedKinds.has("designs")} onPressedChange={() => toggleKind("designs")} id="semio.sketchpad.app.kit.toolbar.showDesigns" icon={<LayoutIcon />} text={labelDesigns} />
      <Toggle pressed={selectedKinds.has("types")} onPressedChange={() => toggleKind("types")} id="semio.sketchpad.app.kit.toolbar.showTypes" icon={<TypeIcon />} text={labelTypes} />
      <Toggle pressed={selectedKinds.has("qualities")} onPressedChange={() => toggleKind("qualities")} id="semio.sketchpad.app.kit.toolbar.showQualities" icon={<AwardIcon />} text={labelQualities} />
      <Toggle pressed={selectedKinds.has("ports")} onPressedChange={() => toggleKind("ports")} id="semio.sketchpad.app.kit.toolbar.showPorts" icon={<PortIcon />} text={labelPorts} />
      <Toggle pressed={selectedKinds.has("tags")} onPressedChange={() => toggleKind("tags")} id="semio.sketchpad.app.kit.toolbar.showTags" icon={<HashIcon />} text={labelTags} />
      <Toggle pressed={selectedKinds.has("concepts")} onPressedChange={() => toggleKind("concepts")} id="semio.sketchpad.app.kit.toolbar.showConcepts" icon={<LightbulbIcon />} text={labelConcepts} />
      <Toggle pressed={selectedKinds.has("files")} onPressedChange={() => toggleKind("files")} id="semio.sketchpad.app.kit.toolbar.showFiles" icon={<DocumentIcon />} text={labelFiles} />
      <Toggle pressed={selectedKinds.has("folders")} onPressedChange={() => toggleKind("folders")} id="semio.sketchpad.app.kit.toolbar.showFolders" icon={<FolderIcon />} text={labelFolders} />
      <Toggle pressed={selectedKinds.has("authors")} onPressedChange={() => toggleKind("authors")} id="semio.sketchpad.app.kit.toolbar.showAuthors" icon={<UserIcon />} text={labelAuthors} />
    </ToolbarGroup>
  );
};

const KitCreateActions: FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const kit = useKit() as Kit | undefined;
    const kitCommands = useKitCommands();
    const sketchpadCommands = useSketchpadCommands();
  
    const defaultDesignName = useLabel("semio.sketchpad.app.kit.defaultDesignName");
    const defaultTypeName = useLabel("semio.sketchpad.app.kit.defaultTypeName");
    const defaultQualityName = useLabel("semio.sketchpad.app.quality.defaultName");
    const defaultPortName = useLabel("semio.sketchpad.app.port.defaultName");
    const defaultTagName = useLabel("semio.sketchpad.app.tag.defaultName");
    const defaultConceptName = useLabel("semio.sketchpad.app.concept.defaultName");
    const defaultFolderName = useLabel("semio.sketchpad.app.folder.defaultName");
  
    const setKindActive = (kind: ArtifactKind) => {
      const newParams = new URLSearchParams(searchParams);
      newParams.delete("kind");
      newParams.append("kind", kind);
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
      setSearchParams(newParams);
    };
  
    const handleCreateArtifact = (kind: ArtifactKind) => {
      if (!kit || !kitCommands) return;
      switch (kind) {
        case "designs": {
          const existingNames = (kit.designs || []).map((d: Design) => d.name);
          const uniqueName = generateUniqueName(defaultDesignName || "", existingNames);
          const newDesign: Design = { guid: guid(), name: uniqueName, pieces: [], connections: [] };
          kitCommands.createDesign(newDesign);
          sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
          break;
        }
        case "types": {
          const existingNames = (kit.types || []).map((t: Type) => t.name);
          const uniqueName = generateUniqueName(defaultTypeName || "", existingNames);
          const newType: Type = { guid: guid(), name: uniqueName, connectors: [] };
          kitCommands.createType(newType);
          sketchpadCommands.navigateToType(kit.guid, newType.guid);
          break;
        }
        case "qualities": {
          const existingNames = (kit.qualities || []).map((q: Quality) => q.name || "");
          const uniqueName = generateUniqueName(defaultQualityName || "", existingNames);
          const existingKeys = (kit.qualities || []).map((q: Quality) => q.key);
          const uniqueKey = generateUniqueName("new.quality", existingKeys, ".");
          const newQuality: Quality = {
            guid: guid(),
            key: uniqueKey,
            name: uniqueName,
          };
          kitCommands.createQuality(newQuality);
          setKindActive("qualities");
          sketchpadCommands.navigateToQuality(kit.guid, newQuality.guid);
          break;
        }
        case "ports": {
          const existingNames = (kit.ports || []).map((p: Port) => p.name);
          const uniqueName = generateUniqueName(defaultPortName || "", existingNames);
          const newPort: Port = {
            guid: guid(),
            name: uniqueName,
          };
          kitCommands.createPort(newPort);
          setKindActive("ports");
          break;
        }
        case "tags": {
          const existingNames = (kit.tags || []).map((t: Tag) => t.name);
          const uniqueName = generateUniqueName(defaultTagName || "", existingNames);
          const newTag: Tag = {
            guid: guid(),
            name: uniqueName,
          };
          kitCommands.createTag(newTag);
          setKindActive("tags");
          break;
        }
        case "concepts": {
          const existingNames = (kit.concepts || []).map((c: Concept) => c.name);
          const uniqueName = generateUniqueName(defaultConceptName || "", existingNames);
          const newConcept: Concept = {
            guid: guid(),
            name: uniqueName,
          };
          kitCommands.createConcept(newConcept);
          setKindActive("concepts");
          break;
        }
        case "folders": {
          const existingNames = (kit.folders || []).map((f: Folder) => f.name);
          const uniqueName = generateUniqueName(defaultFolderName || "", existingNames);
          const newFolder: Folder = {
            guid: guid(),
            name: uniqueName,
          };
          kitCommands.createFolder(newFolder);
          setKindActive("folders");
          break;
        }
      }
    };
  
    const labelDesign = useLabel("semio.sketchpad.app.kit.toolbar.createDesign");
  const labelType = useLabel("semio.sketchpad.app.kit.toolbar.createType");
  const labelQuality = useLabel("semio.sketchpad.app.kit.toolbar.createQuality");
  const labelPort = useLabel("semio.sketchpad.app.kit.toolbar.createPort");
  const labelTag = useLabel("semio.sketchpad.app.kit.toolbar.createTag");
  const labelConcept = useLabel("semio.sketchpad.app.kit.toolbar.createConcept");
  const labelFolder = useLabel("semio.sketchpad.app.kit.toolbar.createFolder");

  return (
    <ToolbarGroup>
      <Button onClick={() => handleCreateArtifact("designs")} id="semio.sketchpad.app.kit.toolbar.createDesign" icon={<LayoutIcon />} text={labelDesign} />
      <Button onClick={() => handleCreateArtifact("types")} id="semio.sketchpad.app.kit.toolbar.createType" icon={<TypeIcon />} text={labelType} />
      <Button onClick={() => handleCreateArtifact("qualities")} id="semio.sketchpad.app.kit.toolbar.createQuality" icon={<AwardIcon />} text={labelQuality} />
      <Button onClick={() => handleCreateArtifact("ports")} id="semio.sketchpad.app.kit.toolbar.createPort" icon={<PortIcon />} text={labelPort} />
      <Button onClick={() => handleCreateArtifact("tags")} id="semio.sketchpad.app.kit.toolbar.createTag" icon={<HashIcon />} text={labelTag} />
      <Button onClick={() => handleCreateArtifact("concepts")} id="semio.sketchpad.app.kit.toolbar.createConcept" icon={<LightbulbIcon />} text={labelConcept} />
      <Button onClick={() => handleCreateArtifact("folders")} id="semio.sketchpad.app.kit.toolbar.createFolder" icon={<FolderIcon />} text={labelFolder} />
    </ToolbarGroup>
  );
};

type TableRow = {
  id: string;
  kind: ArtifactKind;
  artifact: string;
  authors: string;
  updatedAt: string;
  createdAt: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  data: Design | Type | Quality | Port | Tag | Concept | SemioFile | Author | Folder;
  folderId?: string;
  concepts?: string[];
};

const ChevronRight: FC<{ className?: string }> = ({ className }) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="m9 18 6-6-6-6" />
  </svg>
);

const ChevronDown: FC<{ className?: string }> = ({ className }) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="m6 9 6 6 6-6" />
  </svg>
);

const getFileIcon = (fileName: string) => {
  if (!fileName) return <DocumentIcon className="size-tiny" />;
  const extension = fileName.split(".").pop()?.toLowerCase();
  switch (extension) {
    case "jpg":
    case "jpeg":
    case "png":
    case "gif":
    case "svg":
    case "webp":
    case "bmp":
      return <FileImageIcon className="size-tiny" />;
    case "mp4":
    case "avi":
    case "mov":
    case "mkv":
    case "webm":
      return <FileVideoIcon className="size-tiny" />;
    case "json":
      return <FileJsonIcon className="size-tiny" />;
    case "js":
    case "ts":
    case "jsx":
    case "tsx":
    case "py":
    case "java":
    case "cpp":
    case "c":
    case "h":
    case "cs":
    case "rb":
    case "go":
    case "rs":
    case "php":
    case "html":
    case "css":
    case "scss":
    case "xml":
      return <FileCodeIcon className="size-tiny" />;
    case "csv":
    case "xlsx":
    case "xls":
      return <FileSpreadsheetIcon className="size-tiny" />;
    case "txt":
    case "md":
    case "pdf":
    case "doc":
    case "docx":
      return <FileTypeIcon className="size-tiny" />;
    default:
      return <DocumentIcon className="size-tiny" />;
  }
};

const getRowIcon = (row: TableRow): string | React.ReactNode | undefined => {
  switch (row.kind) {
    case "designs":
      return (row.data as Design).icon;
    case "types":
      return (row.data as Type).icon;
    case "qualities":
      return (row.data as Quality).icon;
    case "files":
      return getFileIcon((row.data as SemioFile).name);
    case "folders":
      return <FolderIcon className="size-tiny" />;
    case "authors":
      return <UserIcon className="size-tiny" />;
    default:
      return undefined;
  }
};

const DroppableTableWrapper: FC<{ children: React.ReactNode }> = ({ children }) => {
  const { setNodeRef } = useDroppable({
    id: "canvas-root",
    data: { isCanvas: true },
  });

  return (
    <div ref={setNodeRef} className="w-full min-h-full">
      {children}
    </div>
  );
};

const KitDropZone: FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isDragging, setIsDragging] = useState(false);
  const kitCommands = useKitCommands();
  const { t } = useTranslation();

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.currentTarget === e.target) {
      setIsDragging(false);
    }
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  };

  return (
    <div className="relative h-full w-full" onDragOver={handleDragOver} onDragLeave={handleDragLeave} onDrop={handleDrop}>
      {children}
      {isDragging && (
        <div className="absolute inset-0 z-50 flex items-center justify-center bg-base/80 backdrop-blur-sm">
          <div className="flex flex-col items-center gap-2 text-center">
            <DocumentIcon className="h-12 w-12 text-muted-foreground" />
            <p className="text-lg font-medium">{t("semio.sketchpad.app.kit.dropzone.label")}</p>
            <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.dropzone.description")}</p>
          </div>
        </div>
      )}
    </div>
  );
};

const AppContent: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const params = useParams();
  const [searchParams, setSearchParams] = useSearchParams();

  const kitScope = useKitScope();
  const hasKit = useHasKit(kitScope?.guid || "");

  const kit = useKit(undefined, kitScope?.guid, false) as Kit;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitAppCommands = useKitAppCommands();
  const isMobile = useIsMobile();
  const orchestrator = useSketchpadStore();

  const [selection] = useKitAppSelection();
  const [expandedRowsSet] = useKitAppExpandedRows();
  const [sortColumn] = useKitAppSortColumn();
  const [sortDirection] = useKitAppSortDirection();
  const kitApp = useKitAppXState(kitScope?.guid ?? "");
  const [hover] = useKitAppHover();
  const [activeTool] = useKitAppActiveTool();

  const [selectTypeAction, canSelectType] = useKitAppSelectType();
  const [selectDesignAction, canSelectDesign] = useKitAppSelectDesign();
  const [setSelectionAction, canSetSelection] = useKitAppSetSelection();
  const [clearSelectionAction, canClearSelection] = useKitAppClearSelection();
  const [setFilterAction, canSetFilter] = useKitAppSetFilter();
  const [toggleRowAction] = useKitAppToggleRow();
  const [setSortAction, canSetSort] = useKitAppSetSort();
  const [toggleSortAction, canToggleSort] = useKitAppToggleSort();
  const [setHover] = useKitAppSetHover();
  const [clearHover] = useKitAppClearHover();

  const [isDragOver, setIsDragOver] = React.useState(false);
  const [showZipWarning, setShowZipWarning] = React.useState(false);
  const [activeId, setActiveId] = React.useState<string | null>(null);
  const [overId, setOverId] = React.useState<string | null>(null);
  const lastClickedIndexRef = React.useRef<number>(-1);
  const clickTimerRef = React.useRef<NodeJS.Timeout | null>(null);
  const lastDoubleClickRef = React.useRef<{ rowId: string | null; at: number }>({ rowId: null, at: 0 });

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();

  const defaultDesignName = useLabel("semio.sketchpad.app.design.defaultName");
  const defaultTypeName = useLabel("semio.sketchpad.app.type.defaultName");
  const defaultQualityName = useLabel("semio.sketchpad.app.quality.defaultName");
  const defaultFolderName = useLabel("semio.sketchpad.app.folder.defaultName");
  const defaultPortName = useLabel("semio.sketchpad.app.port.defaultName");
  const defaultTagName = useLabel("semio.sketchpad.app.tag.defaultName");
  const defaultConceptName = useLabel("semio.sketchpad.app.concept.defaultName");
  const kitLoadingLabel = useLabel("semio.sketchpad.app.kit.loading");

  const labelSearch = useLabel("semio.sketchpad.common.search");
  const labelArtifact = useLabel("semio.sketchpad.app.kit.canvas.table.header.artifact");
  const labelKind = useLabel("semio.sketchpad.app.kit.canvas.table.header.kind");
  const labelUpdatedAt = useLabel("semio.sketchpad.app.kit.canvas.table.header.updatedAt");
  const labelCreatedAt = useLabel("semio.sketchpad.app.kit.canvas.table.header.createdAt");

  const selectedKinds = useMemo(() => new Set(searchParams.getAll("kind") as ArtifactKind[]), [searchParams]);
  const selectedName = searchParams.get("name");

  const selectedConcepts = searchParams.getAll("c");
  const searchQuery = searchParams.get("q") || "";

  const selectParam = searchParams.get("select");
  const expandedRows = expandedRowsSet;

  const selectionTypes = selection?.types || [];
  const selectionDesigns = selection?.designs || [];
  const selectionQualities = selection?.qualities || [];
  const selectionPorts = selection?.ports || [];
  const selectionTags = selection?.tags || [];
  const selectionConcepts = selection?.concepts || [];
  const selectionFiles = selection?.files || [];
  const selectionFolders = selection?.folders || [];
  const selectionAuthors = selection?.authors || [];
  const selectionTypesKey = selectionTypes.join(",");
  const selectionDesignsKey = selectionDesigns.join(",");
  const selectionQualitiesKey = selectionQualities.join(",");
  const selectionPortsKey = selectionPorts.join(",");
  const selectionTagsKey = selectionTags.join(",");
  const selectionConceptsKey = selectionConcepts.join(",");
  const selectionFilesKey = selectionFiles.join(",");
  const selectionFoldersKey = selectionFolders.join(",");
  const selectionAuthorsKey = selectionAuthors.join(",");
  const selectionMemo = useMemo(
    () => ({
      types: selectionTypes,
      designs: selectionDesigns,
      qualities: selectionQualities,
      ports: selectionPorts,
      tags: selectionTags,
      concepts: selectionConcepts,
      files: selectionFiles,
      folders: selectionFolders,
      authors: selectionAuthors,
    }),
    [selectionTypesKey, selectionDesignsKey, selectionQualitiesKey, selectionPortsKey, selectionTagsKey, selectionConceptsKey, selectionFilesKey, selectionFoldersKey, selectionAuthorsKey],
  );

  const kitDesigns = kit?.designs;
  const kitTypes = kit?.types;
  const kitQualities = kit?.qualities;
  const kitPorts = kit?.ports;
  const kitTags = kit?.tags;
  const kitConcepts = kit?.concepts;
  const kitFiles = kit?.files;
  const kitFolders = kit?.folders;
  const kitAuthors = kit?.authors;
  const kitDesignsKey = useMemo(() => kitDesigns?.map((d) => `${d.guid}:${d.name}:${d.parent?.guid || ""}:${d.folder || ""}:${d.updatedAt || ""}`).join("|") || "", [kitDesigns]);
  const kitTypesKey = useMemo(() => kitTypes?.map((t) => `${t.guid}:${t.name}:${t.parent?.guid || ""}:${t.folder || ""}:${t.updatedAt || ""}`).join("|") || "", [kitTypes]);
  const kitQualitiesKey = useMemo(() => kitQualities?.map((q) => `${q.guid}:${q.name}:${q.folder || ""}`).join("|") || "", [kitQualities]);
  const kitPortsKey = useMemo(() => kitPorts?.map((i) => `${i.guid}:${i.name}`).join("|") || "", [kitPorts]);
  const kitTagsKey = useMemo(() => kitTags?.map((tag) => `${tag.guid}:${tag.name}`).join("|") || "", [kitTags]);
  const kitConceptsKey = useMemo(() => kitConcepts?.map((c) => `${c.guid}:${c.name}`).join("|") || "", [kitConcepts]);
  const kitFilesKey = useMemo(() => kitFiles?.map((f) => `${f.guid}:${f.name}:${f.folder?.guid || ""}:${f.updatedAt || ""}`).join("|") || "", [kitFiles]);
  const kitFoldersKey = useMemo(() => kitFolders?.map((f) => `${f.guid}:${f.name}:${f.parent?.guid || ""}:${f.updatedAt || ""}`).join("|") || "", [kitFolders]);
  const kitAuthorsKey = useMemo(() => kitAuthors?.map((a) => `${a.guid}:${a.name}`).join("|") || "", [kitAuthors]);

  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kitDesigns?.forEach((d: Design) =>
      d.concepts?.forEach((c) => {
        const concept = kitConcepts?.find((kc) => kc.guid === c.guid);
        if (concept?.name) conceptSet.add(concept.name);
      }),
    );
    return Array.from(conceptSet).sort();
  }, [kitDesignsKey, kitConcepts]);

  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();

    const collectVisibleNames = <T extends { guid: string; name: string; parent?: { guid: string } }>(entities: T[] | undefined) => {
      if (!entities) return;

      if (!selectedName) {
        const rootEntities = entities.filter((e) => !e.parent);
        rootEntities.forEach((e) => nameSet.add(e.name));
      } else {
        const matchingEntities = entities.filter((e) => e.name === selectedName);
        matchingEntities.forEach((parent) => {
          const children = entities.filter((e) => e.parent?.guid === parent.guid);
          children.forEach((child) => nameSet.add(child.name));
        });
      }
    };

    if (selectedKinds.size === 0 || selectedKinds.has("designs")) {
      collectVisibleNames(kitDesigns);
    }
    if (selectedKinds.size === 0 || selectedKinds.has("types")) {
      collectVisibleNames(kitTypes);
    }

    return Array.from(nameSet).sort();
  }, [kitDesignsKey, kitTypesKey, selectedKinds, selectedName]);

  useEffect(() => {
    if (appType !== "kit") {
      return;
    }

    const typesCount = selection?.types?.length || 0;
    const designsCount = selection?.designs?.length || 0;
    const qualitiesCount = selection?.qualities?.length || 0;
    const portsCount = selection?.ports?.length || 0;
    const tagsCount = selection?.tags?.length || 0;
    const conceptsCount = selection?.concepts?.length || 0;
    const filesCount = selection?.files?.length || 0;
    const foldersCount = selection?.folders?.length || 0;
    const authorsCount = selection?.authors?.length || 0;
    const totalSelectedKinds = [typesCount > 0, designsCount > 0, qualitiesCount > 0, portsCount > 0, tagsCount > 0, conceptsCount > 0, filesCount > 0, foldersCount > 0, authorsCount > 0].filter(Boolean).length;

    const artifactsMultipleId = "semio.sketchpad.app.kit.artifacts.multiple";

    removeSection("details", artifactsMultipleId);
    removeSection("details", "semio.sketchpad.app.design.properties");
    removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
    removeSection("details", "semio.sketchpad.app.type.properties");
    removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.port.properties");
    removeSection("details", "semio.sketchpad.app.kit.ports.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.tag.properties");
    removeSection("details", "semio.sketchpad.app.kit.tags.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.concept.properties");
    removeSection("details", "semio.sketchpad.app.kit.concepts.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.file.properties");
    removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.folder.properties");
    removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.properties");

    if (totalSelectedKinds > 1) {
      addSection("details", {
        id: artifactsMultipleId,
        specificity: 30,
        order: 0,
        content: () => <MultipleArtifactsSection />,
      });
    }

    if (designsCount > 0 && totalSelectedKinds === 1) {
      const designSectionId = designsCount === 1 ? "semio.sketchpad.app.design.properties" : "semio.sketchpad.app.kit.designs.multipleTitle";
      addSection("details", {
        id: designSectionId,
        specificity: 30,
        order: 10,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <DesignSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (typesCount > 0 && totalSelectedKinds === 1) {
      const typeSectionId = typesCount === 1 ? "semio.sketchpad.app.type.properties" : "semio.sketchpad.app.kit.types.multipleTitle";
      addSection("details", {
        id: typeSectionId,
        specificity: 30,
        order: 20,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <TypeSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (portsCount > 0 && totalSelectedKinds === 1) {
      const portSectionId = portsCount === 1 ? "semio.sketchpad.app.kit.port.properties" : "semio.sketchpad.app.kit.ports.multipleTitle";
      addSection("details", {
        id: portSectionId,
        specificity: 30,
        order: 25,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <PortSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (tagsCount > 0 && totalSelectedKinds === 1) {
      const tagSectionId = tagsCount === 1 ? "semio.sketchpad.app.kit.tag.properties" : "semio.sketchpad.app.kit.tags.multipleTitle";
      addSection("details", {
        id: tagSectionId,
        specificity: 30,
        order: 26,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <TagSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (conceptsCount > 0 && totalSelectedKinds === 1) {
      const conceptSectionId = conceptsCount === 1 ? "semio.sketchpad.app.kit.concept.properties" : "semio.sketchpad.app.kit.concepts.multipleTitle";
      addSection("details", {
        id: conceptSectionId,
        specificity: 30,
        order: 27,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <ConceptSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (filesCount > 0 && totalSelectedKinds === 1) {
      const fileSectionId = filesCount === 1 ? "semio.sketchpad.app.kit.file.properties" : "semio.sketchpad.app.kit.files.multipleTitle";
      addSection("details", {
        id: fileSectionId,
        specificity: 30,
        order: 30,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <FileSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    if (foldersCount > 0 && totalSelectedKinds === 1) {
      const folderSectionId = foldersCount === 1 ? "semio.sketchpad.app.kit.folder.properties" : "semio.sketchpad.app.kit.folders.multipleTitle";
      addSection("details", {
        id: folderSectionId,
        specificity: 30,
        order: 40,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <FolderSection />
              </KitScopeProvider>
            </React.Suspense>
          ) : null,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.kit.properties",
      specificity: 10,
      order: 100,
      content: () =>
        kit ? (
          <React.Suspense fallback={null}>
            <KitScopeProvider guid={kit.guid}>
              <KitSection />
            </KitScopeProvider>
          </React.Suspense>
        ) : null,
    });

    return () => {
      removeSection("details", artifactsMultipleId);
      removeSection("details", "semio.sketchpad.app.design.properties");
      removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
      removeSection("details", "semio.sketchpad.app.type.properties");
      removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.port.properties");
      removeSection("details", "semio.sketchpad.app.kit.ports.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.tag.properties");
      removeSection("details", "semio.sketchpad.app.kit.tags.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.concept.properties");
      removeSection("details", "semio.sketchpad.app.kit.concepts.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.file.properties");
      removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.folder.properties");
      removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.properties");
    };
  }, [addSection, removeSection, appType, selection]);

  useEffect(() => {
    if (!selectParam) return;

    if (selectedKinds.has("designs")) {
      const design = kitDesigns?.find((d: Design) => d.guid === selectParam);
      if (design && selectDesignAction) {
        selectDesignAction(selectParam);
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKinds.has("types")) {
      const type = kitTypes?.find((t: Type) => t.guid === selectParam);
      if (type && selectTypeAction) {
        selectTypeAction(selectParam);
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    }
  }, [selectParam, selectedKinds, kitDesigns, kitTypes, selectDesignAction, selectTypeAction, searchParams, setSearchParams]);

  const allRows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date | string) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };

    const designsByParent = new Map<string | undefined, Design[]>();
    const typesByParent = new Map<string | undefined, Type[]>();
    const foldersByParent = new Map<string | undefined, Folder[]>();
    const designsByFolder = new Map<string, Design[]>();
    const typesByFolder = new Map<string, Type[]>();
    const qualitiesByFolder = new Map<string, Quality[]>();
    const filesByFolder = new Map<string, SemioFile[]>();

    kitDesigns?.forEach((d: Design) => {
      const parentKey = d.parent?.guid;
      if (!designsByParent.has(parentKey)) designsByParent.set(parentKey, []);
      designsByParent.get(parentKey)!.push(d);
      if (d.folder) {
        if (!designsByFolder.has(d.folder)) designsByFolder.set(d.folder, []);
        designsByFolder.get(d.folder)!.push(d);
      }
    });

    kitTypes?.forEach((t: Type) => {
      const parentKey = t.parent?.guid;
      if (!typesByParent.has(parentKey)) typesByParent.set(parentKey, []);
      typesByParent.get(parentKey)!.push(t);
      if (t.folder) {
        if (!typesByFolder.has(t.folder)) typesByFolder.set(t.folder, []);
        typesByFolder.get(t.folder)!.push(t);
      }
    });

    kitFolders?.forEach((f: Folder) => {
      const parentKey = f.parent?.guid;
      if (!foldersByParent.has(parentKey)) foldersByParent.set(parentKey, []);
      foldersByParent.get(parentKey)!.push(f);
    });

    kitQualities?.forEach((q: Quality) => {
      if (q.folder) {
        if (!qualitiesByFolder.has(q.folder)) qualitiesByFolder.set(q.folder, []);
        qualitiesByFolder.get(q.folder)!.push(q);
      }
    });

    kitFiles?.forEach((f: SemioFile) => {
      if (f.folder?.guid) {
        if (!filesByFolder.has(f.folder.guid)) filesByFolder.set(f.folder.guid, []);
        filesByFolder.get(f.folder.guid)!.push(f);
      }
    });

    if (selectedKinds.size === 0 || selectedKinds.has("designs")) {
      const designGroups = new Map<string, Design[]>();
      kitDesigns?.forEach((design: Design) => {
        const key = design.name;
        if (!designGroups.has(key)) designGroups.set(key, []);
        designGroups.get(key)!.push(design);
      });

      const buildDesignHierarchy = (designs: Design[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childDesigns = designsByParent.get(parentGuid) || [];

        childDesigns.forEach((design) => {
          if (!designs.includes(design)) return;
          if (selectedConcepts.length > 0 && !design.concepts?.some((c) => selectedConcepts.includes(c.guid))) return;
          if (searchQuery && !design.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

          if (selectedKinds.size === 0 && parentGuid === undefined && design.folder) return;

          const rowId = `design-${design.guid}`;
          const children = designsByParent.get(design.guid) || [];
          const hasChildren = children.length > 0;

          const designConceptNames = design.concepts?.map((c) => kitConcepts?.find((kc) => kc.guid === c.guid)?.name).filter((name): name is string => name !== undefined) || [];

          result.push({
            id: rowId,
            kind: "designs",
            artifact: design.name,
            authors: design.authors?.join(", ") || "",
            updatedAt: formatDate(design.updatedAt),
            createdAt: formatDate(design.createdAt),
            level,
            parentId: parentRowId,
            hasChildren,
            isExpanded: false, // Computed in visibleRows
            data: design,
            concepts: designConceptNames,
          });

          if (hasChildren) {
            buildDesignHierarchy(designs, design.guid, level + 1, rowId);
          }
        });
      };

      const allDesignsArray = kitDesigns || [];
      if (selectedName) {
        const matchingDesignGuids = new Set(allDesignsArray.filter((d) => d.name === selectedName).map((d) => d.guid));

        const includeGuids = new Set(matchingDesignGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = designsByParent.get(parentGuid) || [];
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingDesignGuids.forEach((guid) => collectDescendants(guid));

        const filteredDesigns = allDesignsArray.filter((d) => includeGuids.has(d.guid));

        buildDesignHierarchy(filteredDesigns, undefined, 0);
      } else {
        buildDesignHierarchy(allDesignsArray, undefined, 0);
      }
    }

    if (selectedKinds.size === 0 || selectedKinds.has("types")) {
      const buildTypeHierarchy = (types: Type[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childTypes = typesByParent.get(parentGuid) || [];

        childTypes.forEach((type) => {
          if (!types.includes(type)) return;
          if (searchQuery && !type.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

          if (selectedKinds.size === 0 && parentGuid === undefined && type.folder) return;

          const rowId = `type-${type.guid}`;
          const children = typesByParent.get(type.guid) || [];
          const hasChildren = children.length > 0;

          result.push({
            id: rowId,
            kind: "types",
            artifact: type.name,
            authors: type.authors?.join(", ") || "",
            updatedAt: formatDate(type.updatedAt),
            createdAt: formatDate(type.createdAt),
            level,
            parentId: parentRowId,
            hasChildren,
            isExpanded: false, // Computed in visibleRows
            data: type,
          });

          if (hasChildren) {
            buildTypeHierarchy(types, type.guid, level + 1, rowId);
          }
        });
      };

      const allTypesArray = kitTypes || [];
      if (selectedName) {
        const matchingTypeGuids = new Set(allTypesArray.filter((t) => t.name === selectedName).map((t) => t.guid));

        const includeGuids = new Set(matchingTypeGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = typesByParent.get(parentGuid) || [];
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingTypeGuids.forEach((guid) => collectDescendants(guid));

        const filteredTypes = allTypesArray.filter((t) => includeGuids.has(t.guid));

        buildTypeHierarchy(filteredTypes, undefined, 0);
      } else {
        buildTypeHierarchy(allTypesArray, undefined, 0);
      }
    }

    if (selectedKinds.size === 0 || selectedKinds.has("qualities")) {
      kitQualities?.forEach((quality: Quality) => {
        if (searchQuery && !quality.name.toLowerCase().includes(searchQuery.toLowerCase()) && !quality.key.toLowerCase().includes(searchQuery.toLowerCase())) return;

        if (selectedKinds.size === 0 && quality.folder) return;
        result.push({
          id: `quality-${quality.guid}`,
          kind: "qualities",
          artifact: quality.name,
          authors: quality.key,
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: quality,
        });
      });
    }

    if (selectedKinds.size === 0 || selectedKinds.has("ports")) {
      kitPorts?.forEach((iface: Port) => {
        if (searchQuery && !iface.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `port-${iface.guid}`,
          kind: "ports",
          artifact: iface.name,
          authors: iface.compatiblePorts?.length ? `${iface.compatiblePorts.length} compatible` : "All compatible",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: iface,
        });
      });
    }

    if (selectedKinds.size === 0 || selectedKinds.has("tags")) {
      kitTags?.forEach((tag: Tag) => {
        if (searchQuery && !tag.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `tag-${tag.guid}`,
          kind: "tags",
          artifact: tag.name,
          authors: tag.description || "",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: tag,
        });
      });
    }

    if (selectedKinds.size === 0 || selectedKinds.has("concepts")) {
      kitConcepts?.forEach((concept: Concept) => {
        if (searchQuery && !concept.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `concept-${concept.guid}`,
          kind: "concepts",
          artifact: concept.name,
          authors: concept.description || "",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: concept,
        });
      });
    }

    if (selectedKinds.size === 0 || selectedKinds.has("files")) {
      const fileTree = buildFileTree(kitFolders || [], kitFiles || []);
      const flatTree = flattenFileTree(fileTree, 0, expandedRows);

      flatTree.forEach((node) => {
        if (searchQuery && !node.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

        result.push({
          id: `file-${node.path}`,
          kind: "files",
          artifact: node.name,
          authors: node.isDirectory ? `${node.children.length} items` : node.file?.size ? `${(node.file.size / 1024).toFixed(1)} KB` : "",
          updatedAt: node.file ? formatDate(node.file.updatedAt) : "",
          createdAt: node.file ? formatDate(node.file.createdAt) : "",
          level: node.level,
          parentId: node.parentPath ? `file-${node.parentPath}` : undefined,
          hasChildren: node.isDirectory && node.children.length > 0,
          isExpanded: node.isExpanded,
          data: node.file || ({ guid: node.path, name: node.name } as SemioFile),
        });
      });
    }

    if (selectedKinds.size === 0 || selectedKinds.has("folders")) {
      const buildFolderHierarchy = (parentFolder: Folder | null, level: number, parentRowId?: string): void => {
        const parentGuid = parentFolder?.guid;
        const childFolders = foldersByParent.get(parentGuid) || [];

        childFolders.forEach((folder: Folder) => {
          if (searchQuery && !folder.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

          const folderedDesigns = designsByFolder.get(folder.guid) || [];
          const folderedTypes = typesByFolder.get(folder.guid) || [];
          const folderedQualities = qualitiesByFolder.get(folder.guid) || [];
          const folderedFiles = filesByFolder.get(folder.guid) || [];
          const folderedSubFolders = foldersByParent.get(folder.guid) || [];
          const folderedArtifacts = folderedDesigns.length + folderedTypes.length + folderedQualities.length + folderedFiles.length + folderedSubFolders.length;

          const folderId = `folder-${folder.guid}`;
          result.push({
            id: folderId,
            kind: "folders",
            artifact: folder.name,
            authors: `${folderedArtifacts} items`,
            updatedAt: formatDate(folder.updatedAt),
            createdAt: formatDate(folder.createdAt),
            level,
            hasChildren: folderedArtifacts > 0,
            isExpanded: false, // Computed in visibleRows
            data: folder,
            folderId: folder.parent?.guid,
            parentId: parentRowId,
          });

          if (folderedArtifacts > 0) {
            const rootFolderedDesigns = folderedDesigns.filter((d: Design) => !d.parent);
            rootFolderedDesigns.forEach((design: Design) => {
              if (!design.guid) return;
              const rowId = `design-${design.guid}`;
              const children = designsByParent.get(design.guid) || [];
              const hasChildren = children.length > 0;

              result.push({
                id: rowId,
                kind: "designs",
                artifact: design.name,
                authors: (design.authors || []).join(", "),
                updatedAt: formatDate(design.updatedAt),
                createdAt: formatDate(design.createdAt),
                level: level + 1,
                hasChildren,
                isExpanded: false, // Computed in visibleRows
                data: design,
                folderId: folder.guid,
                parentId: folderId,
              });

              if (hasChildren) {
                const buildDesignChildrenInFolder = (parentDesignGuid: string, childLevel: number, parentRowId: string): void => {
                  const childDesigns = designsByParent.get(parentDesignGuid) || [];
                  childDesigns.forEach((childDesign) => {
                    const childRowId = `design-${childDesign.guid}`;
                    const grandChildren = designsByParent.get(childDesign.guid) || [];
                    const hasGrandChildren = grandChildren.length > 0;

                    result.push({
                      id: childRowId,
                      kind: "designs",
                      artifact: childDesign.name,
                      authors: (childDesign.authors || []).join(", "),
                      updatedAt: formatDate(childDesign.updatedAt),
                      createdAt: formatDate(childDesign.createdAt),
                      level: childLevel,
                      hasChildren: hasGrandChildren,
                      isExpanded: false, // Computed in visibleRows
                      data: childDesign,
                      folderId: folder.guid,
                      parentId: parentRowId,
                    });

                    if (hasGrandChildren) {
                      buildDesignChildrenInFolder(childDesign.guid, childLevel + 1, childRowId);
                    }
                  });
                };
                buildDesignChildrenInFolder(design.guid, level + 2, rowId);
              }
            });

            const rootFolderedTypes = folderedTypes.filter((t: Type) => !t.parent);
            rootFolderedTypes.forEach((type: Type) => {
              if (!type.guid) return;
              const rowId = `type-${type.guid}`;
              const children = typesByParent.get(type.guid) || [];
              const hasChildren = children.length > 0;

              result.push({
                id: rowId,
                kind: "types",
                artifact: type.name,
                authors: (type.authors || []).join(", "),
                updatedAt: formatDate(type.updatedAt),
                createdAt: formatDate(type.createdAt),
                level: level + 1,
                hasChildren,
                isExpanded: false, // Computed in visibleRows
                data: type,
                folderId: folder.guid,
                parentId: folderId,
              });

              if (hasChildren) {
                const buildTypeChildrenInFolder = (parentTypeGuid: string, childLevel: number, parentRowId: string): void => {
                  const childTypes = typesByParent.get(parentTypeGuid) || [];
                  childTypes.forEach((childType) => {
                    const childRowId = `type-${childType.guid}`;
                    const grandChildren = typesByParent.get(childType.guid) || [];
                    const hasGrandChildren = grandChildren.length > 0;

                    result.push({
                      id: childRowId,
                      kind: "types",
                      artifact: childType.name,
                      authors: (childType.authors || []).join(", "),
                      updatedAt: formatDate(childType.updatedAt),
                      createdAt: formatDate(childType.createdAt),
                      level: childLevel,
                      hasChildren: hasGrandChildren,
                      isExpanded: false, // Computed in visibleRows
                      data: childType,
                      folderId: folder.guid,
                      parentId: parentRowId,
                    });

                    if (hasGrandChildren) {
                      buildTypeChildrenInFolder(childType.guid, childLevel + 1, childRowId);
                    }
                  });
                };
                buildTypeChildrenInFolder(type.guid, level + 2, rowId);
              }
            });

            folderedQualities.forEach((quality: Quality) => {
              result.push({
                id: `quality-${quality.guid}`,
                kind: "qualities",
                artifact: quality.name,
                authors: "",
                updatedAt: "",
                createdAt: "",
                level: level + 1,
                hasChildren: false,
                isExpanded: false,
                data: quality,
                folderId: folder.guid,
                parentId: folderId,
              });
            });

            folderedFiles.forEach((file: SemioFile) => {
              result.push({
                id: `file-${file.guid}`,
                kind: "files",
                artifact: file.name,
                authors: file.size ? `${(file.size / 1024).toFixed(1)} KB` : "",
                updatedAt: formatDate(file.updatedAt),
                createdAt: formatDate(file.createdAt),
                level: level + 1,
                hasChildren: false,
                isExpanded: false,
                data: file,
                folderId: folder.guid,
                parentId: folderId,
              });
            });

            buildFolderHierarchy(folder, level + 1, folderId);
          }
        });
      };

      buildFolderHierarchy(null, 0);
    }

    if (selectedKinds.size === 0 || selectedKinds.has("authors")) {
      kitAuthors?.forEach((author: Author) => {
        if (searchQuery && !author.name.toLowerCase().includes(searchQuery.toLowerCase()) && !author.email.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `author-${author.guid}`,
          kind: "authors",
          artifact: author.name,
          authors: author.email,
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: author,
        });
      });
    }

    if (sortColumn) {
      const level0Rows = result.filter((r) => r.level === 0);
      const level1Rows = result.filter((r) => r.level === 1);
      const level2Rows = result.filter((r) => r.level === 2);
      level0Rows.sort((a, b) => {
        let comparison = 0;
        switch (sortColumn) {
          case "artifact":
            comparison = a.artifact.localeCompare(b.artifact);
            break;
          case "kind":
            comparison = a.kind.localeCompare(b.kind);
            break;
          case "authors":
            comparison = a.authors.localeCompare(b.authors);
            break;
          case "updatedAt":
            comparison = a.updatedAt.localeCompare(b.updatedAt);
            break;
          case "createdAt":
            comparison = a.createdAt.localeCompare(b.createdAt);
            break;
        }
        return sortDirection === "asc" ? comparison : -comparison;
      });
      const sortedResult: TableRow[] = [];
      level0Rows.forEach((parent) => {
        sortedResult.push(parent);
        const children = level1Rows.filter((c) => c.parentId === parent.id);
        children.sort((a, b) => {
          let comparison = 0;
          switch (sortColumn) {
            case "artifact":
              comparison = a.artifact.localeCompare(b.artifact);
              break;
            case "kind":
              comparison = a.kind.localeCompare(b.kind);
              break;
            case "authors":
              comparison = a.authors.localeCompare(b.authors);
              break;
            case "updatedAt":
              comparison = a.updatedAt.localeCompare(b.updatedAt);
              break;
            case "createdAt":
              comparison = a.createdAt.localeCompare(b.createdAt);
              break;
          }
          return sortDirection === "asc" ? comparison : -comparison;
        });
        children.forEach((child) => {
          sortedResult.push(child);
          const grandchildren = level2Rows.filter((gc) => gc.parentId === child.id);
          grandchildren.sort((a, b) => {
            let comparison = 0;
            switch (sortColumn) {
              case "artifact":
                comparison = a.artifact.localeCompare(b.artifact);
                break;
              case "kind":
                comparison = a.kind.localeCompare(b.kind);
                break;
              case "authors":
                comparison = a.authors.localeCompare(b.authors);
                break;
              case "updatedAt":
                comparison = a.updatedAt.localeCompare(b.updatedAt);
                break;
              case "createdAt":
                comparison = a.createdAt.localeCompare(b.createdAt);
                break;
            }
            return sortDirection === "asc" ? comparison : -comparison;
          });
          sortedResult.push(...grandchildren);
        });
      });
      return sortedResult;
    }

    return result;
  }, [
    kitDesigns,
    kitTypes,
    kitQualities,
    kitPorts,
    kitTags,
    kitConcepts,
    kitFiles,
    kitFolders,
    kitAuthors,
    kitDesignsKey,
    kitTypesKey,
    kitQualitiesKey,
    kitPortsKey,
    kitTagsKey,
    kitConceptsKey,
    kitFilesKey,
    kitFoldersKey,
    kitAuthorsKey,
    selectedKinds,
    selectedName,
    selectedConcepts,
    searchQuery,
    sortColumn,
    sortDirection,
  ]);

  const rows = useMemo<TableRow[]>(() => {
    const visibleRowIds = new Set<string>();
    const rowById = new Map<string, TableRow>();

    allRows.forEach((row) => rowById.set(row.id, row));

    allRows.forEach((row) => {
      let isVisible = true;
      let currentRow = row;

      while (currentRow.parentId) {
        const parent = rowById.get(currentRow.parentId);
        if (!parent) {
          isVisible = false;
          break;
        }
        if (!expandedRows.has(parent.id)) {
          isVisible = false;
          break;
        }
        currentRow = parent;
      }

      if (isVisible) {
        visibleRowIds.add(row.id);
      }
    });

    const result = allRows
      .filter((row) => visibleRowIds.has(row.id))
      .map((row) => ({
        ...row,
        isExpanded: expandedRows.has(row.id),
      }));

    return result;
  }, [allRows, expandedRows]);

  const selectedRows = useMemo(() => {
    const selectedSet = new Set<string>();
    rows.forEach((row) => {
      let isSelected = false;
      if (row.kind === "designs") isSelected = selection.designs?.includes((row.data as Design).guid) ?? false;
      else if (row.kind === "types") isSelected = selection.types?.includes((row.data as Type).guid) ?? false;
      else if (row.kind === "qualities") isSelected = selection.qualities?.includes((row.data as Quality).key) ?? false;
      else if (row.kind === "ports") isSelected = selection.ports?.includes((row.data as Port).guid) ?? false;
      else if (row.kind === "tags") isSelected = selection.tags?.includes((row.data as Tag).guid) ?? false;
      else if (row.kind === "concepts") isSelected = selection.concepts?.includes((row.data as Concept).guid) ?? false;
      else if (row.kind === "files") isSelected = selection.files?.includes((row.data as SemioFile).guid) ?? false;
      else if (row.kind === "folders") isSelected = selection.folders?.includes((row.data as Folder).guid) ?? false;
      else if (row.kind === "authors") isSelected = selection.authors?.includes((row.data as Author).name) ?? false;

      if (isSelected) {
        selectedSet.add(row.id);
      }
    });
    return selectedSet;
  }, [rows, selection]);
  const rowHoverClassName = useCallback(
    (row: TableRow) => {
      if (selectedRows.has(row.id)) return "";
      if (!hover) return "";
      if (row.kind === "designs") return hover.design === (row.data as Design).guid ? "bg-hover-base" : "";
      if (row.kind === "types") return hover.type === (row.data as Type).guid ? "bg-hover-base" : "";
      if (row.kind === "qualities") return hover.quality === (row.data as Quality).guid ? "bg-hover-base" : "";
      if (row.kind === "ports") return hover.port === (row.data as Port).guid ? "bg-hover-base" : "";
      if (row.kind === "tags") return hover.tag === (row.data as Tag).guid ? "bg-hover-base" : "";
      if (row.kind === "concepts") return hover.concept === (row.data as Concept).guid ? "bg-hover-base" : "";
      if (row.kind === "files") return hover.file === (row.data as SemioFile).guid ? "bg-hover-base" : "";
      if (row.kind === "folders") return hover.folder === (row.data as Folder).guid ? "bg-hover-base" : "";
      if (row.kind === "authors") return hover.author === (row.data as Author).guid ? "bg-hover-base" : "";
      return "";
    },
    [selectedRows, hover],
  );
  const handleRowMouseEnter = useCallback(
    (row: TableRow) => {
      if (!setHover) return;
      if (row.kind === "designs") {
        const guid = (row.data as Design).guid;
        if (hover?.design !== guid) setHover({ design: guid });
      } else if (row.kind === "types") {
        const guid = (row.data as Type).guid;
        if (hover?.type !== guid) setHover({ type: guid });
      } else if (row.kind === "qualities") {
        const guid = (row.data as Quality).guid;
        if (hover?.quality !== guid) setHover({ quality: guid });
      } else if (row.kind === "ports") {
        const guid = (row.data as Port).guid;
        if (hover?.port !== guid) setHover({ port: guid });
      } else if (row.kind === "tags") {
        const guid = (row.data as Tag).guid;
        if (hover?.tag !== guid) setHover({ tag: guid });
      } else if (row.kind === "concepts") {
        const guid = (row.data as Concept).guid;
        if (hover?.concept !== guid) setHover({ concept: guid });
      } else if (row.kind === "files") {
        const guid = (row.data as SemioFile).guid;
        if (hover?.file !== guid) setHover({ file: guid });
      } else if (row.kind === "folders") {
        const guid = (row.data as Folder).guid;
        if (hover?.folder !== guid) setHover({ folder: guid });
      } else if (row.kind === "authors") {
        const guid = (row.data as Author).guid;
        if (hover?.author !== guid) setHover({ author: guid });
      }
    },
    [setHover, hover],
  );
  const handleRowMouseLeave = useCallback(() => {
    if (clearHover) clearHover();
  }, [clearHover]);

  const { setFocusItems, setOnFocusItem } = useFocus();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const prevRowsRef = useRef<string>("");

  useEffect(() => {
    const items = rows.map((row) => ({
      id: row.id,
      label: row.artifact,
      category: row.kind.charAt(0).toUpperCase() + row.kind.slice(1),
    }));
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevRowsRef.current !== itemsKey) {
      prevRowsRef.current = itemsKey;
      setFocusItems(items);
    }
  }, [rows, setFocusItems]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
  }, [setOnFocusItem]);

  useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const tbody = scrollAreaRef.current.querySelector("tbody");
      if (tbody) {
        const rowElements = tbody.querySelectorAll("tr");
        const focusedIndex = rows.findIndex((row) => row.id === focusedItemId);
        if (focusedIndex >= 0 && rowElements[focusedIndex]) {
          rowElements[focusedIndex].scrollIntoView({ behavior: "smooth", block: "center" });
          setTimeout(() => setFocusedItemId(undefined), 600);
        }
      }
    }
  }, [focusedItemId, rows]);

  const toggleRow = (rowId: string) => {
    if (toggleRowAction) {
      toggleRowAction(rowId);
    }
    kitAppCommands.toggleExpandedRow(rowId);
  };

  const handleDragStart = (event: DragStartEvent) => {
    setActiveId(event.active.id as string);
  };

  const handleDragOver = (event: DragOverEvent) => {
    setOverId(event.over?.id as string | null);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    setActiveId(null);
    setOverId(null);

    if (!active) return;

    const draggedRow = rows.find((r) => r.id === active.id);
    if (!draggedRow) return;

    if (draggedRow.kind === "folders" && over && over.id === active.id) {
      return;
    }

    let targetFolderId: string | undefined = undefined;
    let targetParentId: string | undefined = undefined;
    let shouldExpandFolder = false;
    let shouldExpandParent = false;

    if (over) {
      if (over.id === "canvas-root") {
        targetFolderId = undefined;
        targetParentId = undefined;
      } else {
        const targetRow = rows.find((r) => r.id === over.id);
        if (targetRow) {
          if (targetRow.kind === "folders") {
            const folder = targetRow.data as Folder;
            targetFolderId = folder.guid;
            shouldExpandFolder = true;
          } else if (targetRow.kind === "designs" && draggedRow.kind === "designs") {
            const targetDesign = targetRow.data as Design;
            targetParentId = targetDesign.guid;
            shouldExpandParent = true;
          } else if (targetRow.kind === "types" && draggedRow.kind === "types") {
            const targetType = targetRow.data as Type;
            targetParentId = targetType.guid;
            shouldExpandParent = true;
          } else if (targetRow.folderId) {
            targetFolderId = targetRow.folderId;
          } else {
            targetFolderId = undefined;
            targetParentId = undefined;
          }
        } else {
          targetFolderId = undefined;
          targetParentId = undefined;
        }
      }
    } else {
      targetFolderId = undefined;
      targetParentId = undefined;
    }

    let currentFolderId: string | undefined = undefined;
    let hasParent = false;

    if (draggedRow.kind === "designs") {
      const design = draggedRow.data as Design;
      currentFolderId = design.folder;
      hasParent = !!design.parent;
    } else if (draggedRow.kind === "types") {
      const type = draggedRow.data as Type;
      currentFolderId = type.folder;
      hasParent = !!type.parent;
    } else if (draggedRow.kind === "qualities") {
      currentFolderId = (draggedRow.data as Quality).folder;
    } else if (draggedRow.kind === "files") {
      currentFolderId = (draggedRow.data as SemioFile).folder?.guid;
    } else if (draggedRow.kind === "folders") {
      currentFolderId = (draggedRow.data as Folder).parent?.guid;
    }

    if (targetFolderId === undefined && targetParentId === undefined && !hasParent && !currentFolderId) {
      return;
    }
    if (targetFolderId !== undefined && currentFolderId === targetFolderId) {
      return;
    }

    if (draggedRow.kind === "designs" && kitCommands) {
      const design = draggedRow.data as Design;

      if (targetParentId !== undefined) {
        if (design.parent?.guid !== targetParentId) {
          const designPieces = (design.pieces || []).filter((p) => p.design?.guid);
          const targetFamily = kit ? getDesignFamilyGuids(kit, targetParentId) : new Set<string>();
          const wouldViolateConstraint = designPieces.some((p) => p.design?.guid && targetFamily.has(p.design.guid));

          if (wouldViolateConstraint) {
            console.warn(`Cannot reparent design "${design.name}" to "${targetParentId}": would violate same-family constraint for design pieces`);
            return;
          }

          kitCommands.updateDesign(design.guid, { parent: { guid: targetParentId } });
        }
      } else if (targetFolderId === undefined && (design.parent || design.folder)) {
        kitCommands.updateDesign(design.guid, { parent: undefined });
        if (design.folder) {
          kitCommands.moveToFolder("design", design.guid, null);
        }
      } else if (!design.parent) {
        kitCommands.moveToFolder("design", design.guid, targetFolderId ?? null);
      }
    } else if (draggedRow.kind === "types" && kitCommands) {
      const type = draggedRow.data as Type;

      if (targetParentId !== undefined) {
        if (type.parent?.guid !== targetParentId) {
          kitCommands.updateType(type.guid, { parent: { guid: targetParentId } });
        }
      } else if (targetFolderId === undefined && (type.parent || type.folder)) {
        kitCommands.updateType(type.guid, { parent: undefined });
        if (type.folder) {
          kitCommands.moveToFolder("type", type.guid, null);
        }
      } else if (!type.parent) {
        kitCommands.moveToFolder("type", type.guid, targetFolderId ?? null);
      }
    } else if (draggedRow.kind === "qualities" && kitCommands) {
      const quality = draggedRow.data as Quality;
      kitCommands.moveToFolder("quality", quality.guid, targetFolderId ?? null);
    } else if (draggedRow.kind === "files" && kitCommands) {
      const file = draggedRow.data as SemioFile;
      kitCommands.moveToFolder("file", file.guid, targetFolderId ?? null);
    } else if (draggedRow.kind === "folders" && kitCommands) {
      const folder = draggedRow.data as Folder;
      kitCommands.moveToFolder("folder", folder.guid, targetFolderId ?? null);
    }

    if (shouldExpandFolder && targetFolderId) {
      const folderId = `folder-${targetFolderId}`;
      if (!expandedRows.has(folderId)) {
        toggleRow(folderId);
      }
    }

    if (shouldExpandParent && targetParentId) {
      const parentRowId = draggedRow.kind === "designs" ? `design-${targetParentId}` : `type-${targetParentId}`;
      if (!expandedRows.has(parentRowId)) {
        toggleRow(parentRowId);
      }
    }
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    switch (kind) {
      case "designs": {
        const existingNames = (kit.designs || []).map((d: Design) => d.name);
        const uniqueName = generateUniqueName(defaultDesignName || "", existingNames);
        const newDesign: Design = {
          guid: guid(),
          name: uniqueName,
          pieces: [],
          connections: [],
        };
        if (kitCommands) kitCommands.createDesign(newDesign);
        sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
        break;
      }
      case "types": {
        const existingNames = (kit.types || []).map((t: Type) => t.name);
        const uniqueName = generateUniqueName(defaultTypeName || "", existingNames);
        const newType: Type = {
          guid: guid(),
          name: uniqueName,
          connectors: [],
        };
        if (kitCommands) kitCommands.createType(newType);
        sketchpadCommands.navigateToType(kit.guid, newType.guid);
        break;
      }
      case "qualities": {
        const existingNames = (kit.qualities || []).map((q: Quality) => q.name || "");
        const uniqueName = generateUniqueName(defaultQualityName || "", existingNames);
        const existingKeys = (kit.qualities || []).map((q: Quality) => q.key);
        const uniqueKey = generateUniqueName("new.quality", existingKeys, ".");
        const newQuality: Quality = {
          guid: guid(),
          key: uniqueKey,
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createQuality(newQuality);
        sketchpadCommands.navigateToQuality(kit.guid, newQuality.guid);
        break;
      }
      case "ports": {
        const existingNames = (kit.ports || []).map((i: Port) => i.name);
        const uniqueName = generateUniqueName(defaultPortName || "", existingNames);
        const newPort: Port = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createPort(newPort);
        setKind("ports");
        setSelectionAction?.({ ports: [newPort.guid] });
        break;
      }
      case "tags": {
        const existingNames = (kit.tags || []).map((t: Tag) => t.name);
        const uniqueName = generateUniqueName(defaultTagName || "", existingNames);
        const newTag: Tag = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createTag(newTag);
        setKind("tags");
        setSelectionAction?.({ tags: [newTag.guid] });
        break;
      }
      case "concepts": {
        const existingNames = (kit.concepts || []).map((c: Concept) => c.name);
        const uniqueName = generateUniqueName(defaultConceptName || "", existingNames);
        const newConcept: Concept = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createConcept(newConcept);
        setKind("concepts");
        setSelectionAction?.({ concepts: [newConcept.guid] });
        break;
      }
      case "files": {
        // TODO: Implement file creation
        break;
      }
      case "folders": {
        const existingNames = (kit.folders || []).map((f: Folder) => f.name);
        const uniqueName = generateUniqueName(defaultFolderName || "", existingNames);
        const newFolder: Folder = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createFolder(newFolder);
        setKind("folders");
        setSelectionAction?.({ folders: [newFolder.guid] });
        break;
      }
      case "authors": {
        // TODO: Implement author creation
        break;
      }
    }
  };

  const handleCreateChildForRow = (row: TableRow) => {
    if (row.kind === "designs") {
      const design = row.data as Design;
      const existingNames = (kit.designs || []).filter((d: Design) => d.parent?.guid === design.guid).map((d: Design) => d.name);
      const uniqueName = generateUniqueName(design.name, existingNames);
      const newDesign: Design = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: design.guid },
        pieces: [],
        connections: [],
      };
      if (kitCommands) kitCommands.createDesign(newDesign);
      sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
    } else if (row.kind === "types") {
      const type = row.data as Type;
      const existingNames = (kit.types || []).filter((t: Type) => t.parent?.guid === type.guid).map((t: Type) => t.name);
      const uniqueName = generateUniqueName(type.name, existingNames);
      const newType: Type = {
        guid: guid(),
        name: uniqueName,
        parent: { guid: type.guid },
        connectors: [],
      };
      if (kitCommands) kitCommands.createType(newType);
      sketchpadCommands.navigateToType(kit.guid, newType.guid);
    }
  };

  const toggleKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    const kinds = newParams.getAll("kind");
    if (kinds.includes(kind)) {
      const remaining = kinds.filter((k) => k !== kind);
      newParams.delete("kind");
      remaining.forEach((k) => newParams.append("kind", k));
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.append("kind", kind);
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };
  const setKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set("kind", kind);
    newParams.delete("name");
    newParams.delete("variant");
    newParams.delete("view");
    setSearchParams(newParams);
  };

  const toggleConcept = (concept: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentConcepts = newParams.getAll("c");

    if (currentConcepts.includes(concept)) {
      newParams.delete("c");
      currentConcepts.filter((c) => c !== concept).forEach((c) => newParams.append("c", c));
    } else {
      newParams.append("c", concept);
    }

    setSearchParams(newParams);
  };

  const toggleName = (name: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedName === name) {
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("name", name);
      newParams.delete("variant");
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };

  const handleRowClick = useCallback(
    (row: TableRow, index: number, e: React.MouseEvent) => {
      if (clickTimerRef.current) {
        clearTimeout(clickTimerRef.current);
        clickTimerRef.current = null;
      }

      if (e.detail > 1) {
        lastDoubleClickRef.current = { rowId: row.id, at: Date.now() };
        if (row.kind === "designs") {
          sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
          return;
        }
        if (row.kind === "types") {
          sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
          return;
        }
        if (row.kind === "qualities") {
          sketchpadCommands.navigateToQuality(kit.guid, (row.data as Quality).key);
          return;
        }
        return;
      }

      const compositionKind = resolveSelectionCompositionKind(activeTool, {
        shiftKey: e.shiftKey,
        altKey: e.altKey,
        ctrlKey: e.ctrlKey,
        metaKey: e.metaKey,
      });
      const useRangeSelection = e.shiftKey && !e.altKey && !e.ctrlKey && !e.metaKey;

      if (useRangeSelection && lastClickedIndexRef.current !== -1) {
        const start = Math.min(lastClickedIndexRef.current, index);
        const end = Math.max(lastClickedIndexRef.current, index);
        const rangeRows = rows.slice(start, end + 1);

        const selectedByKind: {
          types: Guid[];
          designs: Guid[];
          qualities: string[];
          ports: Guid[];
          tags: Guid[];
          concepts: Guid[];
          files: string[];
          folders: Guid[];
          authors: string[];
        } = {
          types: [],
          designs: [],
          qualities: [],
          ports: [],
          tags: [],
          concepts: [],
          files: [],
          folders: [],
          authors: [],
        };

        rangeRows.forEach((r) => {
          if (r.kind === "types") selectedByKind.types.push((r.data as Type).guid);
          else if (r.kind === "designs") selectedByKind.designs.push((r.data as Design).guid);
          else if (r.kind === "qualities") selectedByKind.qualities.push((r.data as Quality).key);
          else if (r.kind === "ports") selectedByKind.ports.push((r.data as Port).guid);
          else if (r.kind === "tags") selectedByKind.tags.push((r.data as Tag).guid);
          else if (r.kind === "concepts") selectedByKind.concepts.push((r.data as Concept).guid);
          else if (r.kind === "files") selectedByKind.files.push((r.data as SemioFile).guid);
          else if (r.kind === "folders") selectedByKind.folders.push((r.data as Folder).guid);
          else if (r.kind === "authors") selectedByKind.authors.push((r.data as Author).name);
        });

        setSelectionAction?.(selectedByKind);

        return;
      }

      if (compositionKind !== "replace") {
        let selectionKind: keyof KitAppSelection | null = null;
        let selectionValue: string | null = null;
        if (row.kind === "designs") {
          selectionKind = "designs";
          selectionValue = (row.data as Design).guid;
        } else if (row.kind === "types") {
          selectionKind = "types";
          selectionValue = (row.data as Type).guid;
        } else if (row.kind === "qualities") {
          selectionKind = "qualities";
          selectionValue = (row.data as Quality).key;
        } else if (row.kind === "ports") {
          selectionKind = "ports";
          selectionValue = (row.data as Port).guid;
        } else if (row.kind === "tags") {
          selectionKind = "tags";
          selectionValue = (row.data as Tag).guid;
        } else if (row.kind === "concepts") {
          selectionKind = "concepts";
          selectionValue = (row.data as Concept).guid;
        } else if (row.kind === "files") {
          selectionKind = "files";
          selectionValue = (row.data as SemioFile).guid;
        } else if (row.kind === "folders") {
          selectionKind = "folders";
          selectionValue = (row.data as Folder).guid;
        } else if (row.kind === "authors") {
          selectionKind = "authors";
          selectionValue = (row.data as Author).name;
        }
        if (selectionKind && selectionValue) {
          const currentValues = (selection[selectionKind] ?? []) as string[];
          setSelectionAction?.({
            ...selection,
            [selectionKind]: applySelectionComposition(currentValues, [selectionValue], compositionKind),
          });
        }
        return;
      }

      clickTimerRef.current = setTimeout(() => {
        if (row.kind === "designs") {
          setSelectionAction?.({ designs: [(row.data as Design).guid] });
        } else if (row.kind === "types") {
          setSelectionAction?.({ types: [(row.data as Type).guid] });
        } else if (row.kind === "qualities") {
          setSelectionAction?.({ qualities: [(row.data as Quality).key] });
        } else if (row.kind === "ports") {
          setSelectionAction?.({ ports: [(row.data as Port).guid] });
        } else if (row.kind === "tags") {
          setSelectionAction?.({ tags: [(row.data as Tag).guid] });
        } else if (row.kind === "concepts") {
          setSelectionAction?.({ concepts: [(row.data as Concept).guid] });
        } else if (row.kind === "files") {
          setSelectionAction?.({ files: [(row.data as SemioFile).guid] });
        } else if (row.kind === "folders") {
          setSelectionAction?.({ folders: [(row.data as Folder).guid] });
        } else if (row.kind === "authors") {
          setSelectionAction?.({ authors: [(row.data as Author).name] });
        }
        clickTimerRef.current = null;
      }, 200);

      lastClickedIndexRef.current = index;
    },
    [kit.guid, sketchpadCommands, setSelectionAction, selection, rows, activeTool],
  );

  const handleRowDoubleClick = useCallback(
    (row: TableRow, index: number) => {
      const now = Date.now();
      if (lastDoubleClickRef.current.rowId === row.id && now - lastDoubleClickRef.current.at < 400) {
        lastDoubleClickRef.current = { rowId: null, at: 0 };
        return;
      }

      if (clickTimerRef.current) {
        clearTimeout(clickTimerRef.current);
        clickTimerRef.current = null;
      }

      if (row.kind === "designs") sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
      else if (row.kind === "types") sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
      else if (row.kind === "qualities") sketchpadCommands.navigateToQuality(kit.guid, (row.data as Quality).key);
    },
    [kit.guid, sketchpadCommands],
  );

  const handleSortClick = (column: "artifact" | "kind" | "authors" | "updatedAt" | "createdAt") => {
    kitAppCommands.toggleSort(column);
  };

  const handleFileDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer.types.includes("Files")) {
      setIsDragOver(true);
    }
  };

  const handleFileDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  };

  const handleFileDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length === 0) return;

    const hasZipFile = files.some((file) => file.name.toLowerCase().endsWith(".zip"));
    if (hasZipFile) {
      setShowZipWarning(true);

      setTimeout(() => setShowZipWarning(false), 8000);
    }

    for (const file of files) {
      const newFile: SemioFile = {
        guid: guid(),
        name: file.name,
        size: file.size,
        hash: undefined,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };

      try {
        await kitCommands?.addFile(newFile, file);
      } catch (error) {
        console.error(`Failed to add file ${file.name}:`, error);
      }
    }
  };

  if (!hasKit || !kit) {
    return <NotFound title={t("semio.sketchpad.app.kit.notFound.label.normal")} description={t("semio.sketchpad.app.kit.notFound.description.normal")} parentPath="/" parentLabel={t("semio.sketchpad.app.home.title")} />;
  }

  if (!kitApp) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{kitLoadingLabel}</p>
      </div>
    );
  }

  if (isMobile) {
    return (
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            kitAppCommands.deselectAll();
          }
        }}
      >
        {/* Mobile table using general Table component */}
        <Table
          className="flex-1 min-h-0"
          columns={[
            {
              id: "artifact",
              header: (
                <div className="flex items-center justify-between w-full">
                  <span>{labelArtifact}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("artifact");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                      { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                    ]}
                    id="semio.sketchpad.app.kit.sortByArtifact"
                  />
                </div>
              ),
              accessor: (row: TableRow) => (
                <div className="flex items-center gap-single w-full h-full">
                  <div className="flex items-center gap-single flex-1 min-w-0 h-full" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                    {row.hasChildren ? (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleRow(row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span id="semio.sketchpad.app.kit.mobileTable.row.expandSpacer" className="size-small shrink-0" />
                    )}
                    <TableAvatar id="semio.sketchpad.app.kit.mobileTable.row.avatar" className="size-small" name={row.artifact} icon={getRowIcon(row)} />
                    <span id="semio.sketchpad.app.kit.mobileTable.row.name" className="text-left min-w-0 truncate">{row.artifact}</span>
                  </div>
                  {row.concepts && row.concepts.length > 0 && (
                    <Scrollable orientation="horizontal" className="shrink-0 min-w-0 max-w-[200px]">
                      <div className="flex items-center gap-single px-single h-medium w-fit">
                        {row.concepts.map((concept) => (
                          <Action key={concept} onClick={() => toggleConcept(concept)} id={`semio.sketchpad.app.kit.row.concept.${concept}`} text={concept} className={selectedConcepts.includes(concept) ? "bg-active-base" : ""} />
                        ))}
                      </div>
                    </Scrollable>
                  )}
                  <div className="flex items-center gap-single shrink-0 ml-auto">
                    {(row.kind === "designs" || row.kind === "types") && (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCreateChildForRow(row);
                        }}
                        id="semio.sketchpad.app.kit.kitApp.createChild"
                        icon={<AddIcon />}
                      />
                    )}
                  </div>
                </div>
              ),
            },
          ]}
          rowClassName={rowHoverClassName}
          data={rows}
          getRowId={(row) => row.id}
          selectedRows={selectedRows}
          onRowClick={handleRowClick}
          onRowDoubleClick={handleRowDoubleClick}
          onRowMouseEnter={handleRowMouseEnter}
          onRowMouseLeave={handleRowMouseLeave}
          dragDrop={{
            enabled: true,
            onDragStart: (rowId) => setActiveId(rowId),
            onDragEnd: (event) => {
              const activeRow = rows.find((r) => r.id === event.active);
              const overRow = event.over ? rows.find((r) => r.id === event.over) : null;
              if (activeRow) {
                handleDragEnd({
                  active: { id: event.active, data: { current: { row: activeRow } } },
                  over: overRow ? { id: event.over!, data: { current: { row: overRow } } } : event.over === "canvas-root" ? { id: "canvas-root", data: { current: { isCanvas: true } } } : null,
                } as any);
              }
            },
            canDrag: (rowId) => {
              const row = rows.find((r) => r.id === rowId);
              return row ? row.kind !== "authors" : false;
            },
          }}
          wrapperComponent={DroppableTableWrapper}
          isMobile={true}
        />
      </div>
    );
  }

  return (
    <div
      className="flex flex-col h-full"
      onClick={(e: React.MouseEvent) => {
        if (e.target === e.currentTarget) {
          kitAppCommands.deselectAll();
        }
      }}
    >
      <Scrollable ref={scrollAreaRef} className="flex-1 min-h-0" onDragOver={handleFileDragOver} onDragLeave={handleFileDragLeave} onDrop={handleFileDrop}>
        {isDragOver && (
          <div className="absolute inset-0 bg-active-base/50 border-2 border-dashed border-active-foreground flex items-center justify-center z-panel">
            <div className="text-active-foreground text-lg font-medium">Drop files to add to kit</div>
          </div>
        )}
        {showZipWarning && (
          <div className="absolute top-4 left-1/2 -translate-x-1/2 z-[60] max-w-md">
            <div className="bg-warning/90 backdrop-blur-sm border border-warning-foreground/20 rounded-lg shadow-lg p-4 flex items-start gap-3">
              <AlertCircleIcon className="h-5 w-5 text-warning-foreground flex-shrink-0 mt-0.5" />
              <div className="flex-1 text-sm">
                <p className="font-medium text-warning-foreground mb-1">Zip file detected</p>
                <p className="text-warning-foreground/90">If this zip file contains a kit, please navigate to the home screen to import it properly. Zip files dropped here are added as regular files.</p>
              </div>
              <button onClick={() => setShowZipWarning(false)} className="text-warning-foreground/70 hover:text-warning-foreground transition-colors">
                ×
              </button>
            </div>
          </div>
        )}
        <Table
          columns={[
            ...(selectedKinds.size !== 1
              ? [
                  {
                    id: "kind",
                    header: (
                      <div className="inline-flex items-center gap-single">
                        <span>{labelKind}</span>
                        <Toggle
                          kind="dropdown"
                          pressed={sortColumn === "kind"}
                          value={sortColumn === "kind" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            kitAppCommands.setSortColumn("kind");
                            kitAppCommands.setSortDirection(value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                            { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                          ]}
                          id="semio.sketchpad.app.kit.sortByKind"
                        />
                      </div>
                    ),
                    accessor: (row: TableRow) => (
                      <>
                        {row.kind === "designs" && <LayoutIcon />}
                        {row.kind === "types" && <TypeIcon />}
                        {row.kind === "qualities" && <AwardIcon />}
                        {row.kind === "ports" && <PortIcon />}
                        {row.kind === "tags" && <HashIcon />}
                        {row.kind === "concepts" && <LightbulbIcon />}
                        {row.kind === "files" && <DocumentIcon />}
                        {row.kind === "folders" && <FolderIcon />}
                        {row.kind === "authors" && <UserIcon />}
                      </>
                    ),
                    width: "w-small",
                    headerClassName: "relative group w-0 whitespace-nowrap",
                  },
                ]
              : []),
            {
              id: "artifact",
              header: (
                <div className="flex items-center justify-between w-full">
                  <span>{labelArtifact}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("artifact");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                      { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                    ]}
                    id="semio.sketchpad.app.kit.sortByArtifact"
                  />
                </div>
              ),
              accessor: (row: TableRow) => (
                <div className="flex items-center gap-single w-full h-full">
                  <div className="flex items-center gap-single flex-1 min-w-0 h-full" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                    {row.hasChildren ? (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleRow(row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span id="semio.sketchpad.app.kit.desktopTable.row.expandSpacer" className="size-small shrink-0" />
                    )}
                    <TableAvatar id="semio.sketchpad.app.kit.desktopTable.row.avatar" className="size-small" name={row.artifact} icon={getRowIcon(row)} />
                    <span id="semio.sketchpad.app.kit.desktopTable.row.name" className="text-left min-w-0 truncate">{row.artifact}</span>
                  </div>
                  {row.concepts && row.concepts.length > 0 && (
                    <Scrollable orientation="horizontal" className="shrink-0 min-w-0 max-w-[200px]">
                      <div className="flex items-center gap-single px-single h-medium w-fit">
                        {row.concepts.map((concept) => (
                          <Action key={concept} onClick={() => toggleConcept(concept)} id={`semio.sketchpad.app.kit.row.concept.${concept}`} text={concept} className={selectedConcepts.includes(concept) ? "bg-active-base" : ""} />
                        ))}
                      </div>
                    </Scrollable>
                  )}
                  <div className="flex items-center gap-single shrink-0 ml-auto">
                    {(row.kind === "designs" || row.kind === "types") && (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCreateChildForRow(row);
                        }}
                        id="semio.sketchpad.app.kit.kitApp.createChild"
                        icon={<AddIcon />}
                      />
                    )}
                  </div>
                </div>
              ),
            },
            {
              id: "updatedAt",
              header: (
                <div className="flex items-center justify-between w-full">
                  <span>{labelUpdatedAt}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "updatedAt"}
                    value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("updatedAt");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                      { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                    ]}
                    id="semio.sketchpad.app.kit.sortByUpdatedAt"
                  />
                </div>
              ),
              accessor: (row: TableRow) => row.updatedAt,
              width: "w-1/4",
            },
            {
              id: "createdAt",
              header: (
                <div className="flex items-center justify-between w-full">
                  <span>{labelCreatedAt}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "createdAt"}
                    value={sortColumn === "createdAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("createdAt");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <SortAscendingIcon />, id: "semio.sketchpad.sort.ascending" },
                      { value: "desc", label: <SortDescendingIcon />, id: "semio.sketchpad.sort.descending" },
                    ]}
                    id="semio.sketchpad.app.kit.sortByCreatedAt"
                  />
                </div>
              ),
              accessor: (row: TableRow) => row.createdAt,
              width: "w-1/4",
            },
          ]}
          rowClassName={rowHoverClassName}
          data={rows}
          getRowId={(row) => row.id}
          selectedRows={selectedRows}
          onRowClick={handleRowClick}
          onRowDoubleClick={handleRowDoubleClick}
          onRowMouseEnter={handleRowMouseEnter}
          onRowMouseLeave={handleRowMouseLeave}
          dragDrop={{
            enabled: true,
            onDragStart: (rowId) => setActiveId(rowId),
            onDragEnd: (event) => {
              const activeRow = rows.find((r) => r.id === event.active);
              const overRow = event.over ? rows.find((r) => r.id === event.over) : null;
              if (activeRow) {
                handleDragEnd({
                  active: { id: event.active, data: { current: { row: activeRow } } },
                  over: overRow ? { id: event.over!, data: { current: { row: overRow } } } : event.over === "canvas-root" ? { id: "canvas-root", data: { current: { isCanvas: true } } } : null,
                } as any);
              }
            },
            canDrag: (rowId) => {
              const row = rows.find((r) => r.id === rowId);
              return row ? row.kind !== "authors" : false;
            },
          }}
          wrapperComponent={DroppableTableWrapper}
          isMobile={false}
        />
      </Scrollable>
    </div>
  );
};

class ErrorBoundary extends React.Component<{ children: React.ReactNode; fallback: React.ReactNode }, { hasError: boolean; error: Error | null }> {
  constructor(props: { children: React.ReactNode; fallback: React.ReactNode }) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {}

  componentDidUpdate(prevProps: { children: React.ReactNode; fallback: React.ReactNode }) {
    if (prevProps.children !== this.props.children && this.state.hasError) {
      this.setState({ hasError: false, error: null });
    }
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback;
    }
    return this.props.children;
  }
}

function useKitAppYjsToXStateSync() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const sketchpadStore = useSketchpadStore();
  const sketchpadCommands = useSketchpadCommands();
  const hasKit = useHasKit(kitGuid);
  const initializedKeyRef = useRef<string | null>(null);

  useLayoutEffect(() => {
    if (!kitGuid || !hasKit) return;

    if (!sketchpadStore.hasKitApp({ kit: kitGuid })) {
      sketchpadCommands.createKitApp("semio.sketchpad.app.kit.autoCreateForSync", { kit: kitGuid });
    }
  }, [kitGuid, hasKit, sketchpadStore, sketchpadCommands]);

  useLayoutEffect(() => {
    if (!kitGuid || !hasKit) return;
    const initKey = kitGuid;
    if (initializedKeyRef.current === initKey) return;

    let xstateInitialState;
    if (sketchpadStore.hasKitApp({ kit: kitGuid })) {
      const store = sketchpadStore.kitApp(kitGuid);
      const initialState = store.snapshot();
      console.log('[DEBUG] [Kit Init] Store snapshot:', initialState);

      xstateInitialState = {
        ...initialState,
        activeTool: initialState.activeTool ?? ToolKind.SELECTION_NORMAL,
        expandedRows: new Set(initialState.expandedRows || []),
        transaction: {
          isTransactionActive: false,
          currentTransactionStack: [],
          pastTransactionStack: [],
          redoStack: [],
        },
      };
    } else {
      console.log('[DEBUG] [Kit Init] No store, using defaults');
      xstateInitialState = {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        activeTool: ToolKind.SELECTION_NORMAL,
        fullscreenWindow: "none",
        others: [],
        filterSearch: undefined,
        expandedRows: new Set(),
        sortColumn: undefined,
        sortDirection: undefined,
        transaction: {
          isTransactionActive: false,
          currentTransactionStack: [],
          pastTransactionStack: [],
          redoStack: [],
        },
      };
    }

    actor.send({
      type: "KIT.INIT",
      kitGuid,
      state: xstateInitialState,
    });
    initializedKeyRef.current = initKey;
  }, [actor, sketchpadStore, kitGuid, hasKit]);

  const store = kitGuid && sketchpadStore.hasKitApp({ kit: kitGuid }) ? sketchpadStore.kitApp(kitGuid) : null;
  const state = useSyncDeep<KitAppState, KitAppState>(store, (s: KitAppState) => s);

  useEffect(() => {
    if (!state || !kitGuid || initializedKeyRef.current !== kitGuid) return;

    const xstateState = {
      ...state,
      expandedRows: new Set(state.expandedRows || []),
    };

    actor.send({
      type: "KIT.SYNC",
      kitGuid,
      state: xstateState,
    });
  }, [actor, state, kitGuid]);
}

const App: FC = () => {
  useKitAppYjsToXStateSync();
  const transaction = useKitAppTransaction();

  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">Failed to load kit app</p>
        </div>
      }
    >
      <TransactionProvider transaction={transaction}>
        <KitDropZone>
          <Canvas>
            <Window id="kit-table">
              <AppContent />
            </Window>
          </Canvas>
        </KitDropZone>
      </TransactionProvider>
    </ErrorBoundary>
  );
};

// #endregion Table

// #region Diagram

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖windows🔖diagram](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/WINDOWS/DIAGRAM)
// Diagram MUST render the interactive force-directed Kit diagram with type and design nodes.

interface KitDiagramNode extends Record<string, unknown> {
  guid: string;
  name: string;
  kind: KitDiagramNodeKind;
  icon?: string | React.ReactNode;
  parentGuid?: string;
  concepts?: string[];
}

interface KitDiagramEdge {
  id: string;
  source: string;
  target: string;
  relationship: "part-of" | "reference";
}

const KitArtifactNode: FC<NodeProps<Node<KitDiagramNode>>> = ({ data }) => {
  const [selection] = useKitAppSelection();
  const [hover] = useKitAppHover();
  const strategy = useMemo(() => getKitDiagramShapeStrategy(data.kind), [data.kind]);
  const frame = useMemo(() => getKitDiagramNodeFrameForKind(data.kind), [data.kind]);
  const renderPayload = useMemo<KitDiagramShapeRenderPayload>(() => strategy.getRenderPayload(), [strategy]);

  const isHovered = useMemo(() => {
    if (!hover) return false;
    if (data.kind === "type") return hover.type === data.guid;
    if (data.kind === "design") return hover.design === data.guid;
    if (data.kind === "quality") return hover.quality === data.guid;
    if (data.kind === "port") return hover.port === data.guid;
    if (data.kind === "tag") return hover.tag === data.guid;
    if (data.kind === "concept") return hover.concept === data.guid;
    if (data.kind === "file") return hover.file === data.guid;
    if (data.kind === "folder") return hover.folder === data.guid;
    if (data.kind === "author") return hover.author === data.guid;
    return false;
  }, [hover, data.kind, data.guid]);

  const isSelected = useMemo(() => {
    if (!selection) return false;
    switch (data.kind) {
      case "type":
        return selection.types?.includes(data.guid) ?? false;
      case "design":
        return selection.designs?.includes(data.guid) ?? false;
      case "quality":
        return selection.qualities?.includes(data.guid) ?? false;
      case "port":
        return selection.ports?.includes(data.guid) ?? false;
      case "tag":
        return selection.tags?.includes(data.guid) ?? false;
      case "concept":
        return selection.concepts?.includes(data.guid) ?? false;
      case "file":
        return selection.files?.includes(data.guid) ?? false;
      case "folder":
        return selection.folders?.includes(data.guid) ?? false;
      case "author":
        return selection.authors?.includes(data.guid) ?? false;
      default:
        return false;
    }
  }, [selection, data.kind, data.guid]);

  return (
    <div
      data-kit-node="v3"
      data-kit-node-shape={strategy.id}
      data-kit-node-kind={data.kind}
      style={{
        width: `${frame.width}px`,
        height: `${frame.height}px`,
        position: "relative",
        background: "transparent",
        border: "0",
        outline: "0",
        boxShadow: "none",
        pointerEvents: "auto",
        padding: 0,
        margin: 0
      }}
      title={data.name || data.guid.substring(0, 8)}
    >
      <Handle type="target" position={Position.Top} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Bottom} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="target" position={Position.Left} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Right} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <TableAvatar
        id="semio.sketchpad.app.kit.diagram.node.avatar"
        className={`!absolute !inset-0 ${renderPayload.className ?? ""}`}
        name={data.name}
        icon={data.icon}
        isSelected={isSelected}
        isHovered={isHovered}
        style={{ width: `${frame.width}px`, height: `${frame.height}px`, ...(renderPayload.style as React.CSSProperties | undefined) }}
      />
    </div>
  );
};

const kitNodeTypes = {
  artifact: KitArtifactNode,
};

const edgeStyle = {
  "part-of": { stroke: "var(--accent-secondary)", strokeWidth: 3 },
  reference: { stroke: "var(--foreground)", strokeWidth: 1, strokeDasharray: "5,5" },
};

const KIT_DIAGRAM_DEBUG = false;
const KIT_DIAGRAM_DEBUG_INTERVAL_MS = 400;

const KIT_DIAGRAM_FALLBACK_KIND: KitDiagramNodeKind = "quality";
const KIT_DIAGRAM_PROXIMITY_CONNECT_DISTANCE = Math.max(KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY.frame.width, KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY.frame.height) * 0.55;

const isKitDiagramNodeKind = (value: string): value is KitDiagramNodeKind =>
  value === "type" || value === "design" || value === "quality" || value === "port" || value === "tag" || value === "concept" || value === "file" || value === "folder" || value === "author";

const toReactFlowPosition = (side: "top" | "right" | "bottom" | "left"): Position => {
  if (side === "top") return Position.Top;
  if (side === "right") return Position.Right;
  if (side === "bottom") return Position.Bottom;
  return Position.Left;
};

const resolveDiagramNodeKind = (node: any): KitDiagramNodeKind => {
  const dataKind = (node?.data as KitDiagramNode | undefined)?.kind;
  if (dataKind && isKitDiagramNodeKind(dataKind)) return dataKind;
  const idKind = typeof node?.id === "string" ? node.id.split(":", 2)[0] : "";
  if (isKitDiagramNodeKind(idKind)) return idKind;
  return KIT_DIAGRAM_FALLBACK_KIND;
};

const resolveDiagramNodePosition = (node: any): { x: number; y: number } => {
  const position = node?.internals?.positionAbsolute ?? node?.positionAbsolute ?? node?.position;
  if (position && Number.isFinite(position.x) && Number.isFinite(position.y)) return position;
  return { x: 0, y: 0 };
};

const resolveDiagramNodeFrame = (node: any, kind: KitDiagramNodeKind) => normalizeKitDiagramFrame({ width: node?.width, height: node?.height }, getKitDiagramNodeFrameForKind(kind));

const resolveDiagramEdgeAnchors = (sourceNode: any, targetNode: any) => {
  const sourceKind = resolveDiagramNodeKind(sourceNode);
  const targetKind = resolveDiagramNodeKind(targetNode);
  const sourcePosition = resolveDiagramNodePosition(sourceNode);
  const targetPosition = resolveDiagramNodePosition(targetNode);
  const sourceFrame = resolveDiagramNodeFrame(sourceNode, sourceKind);
  const targetFrame = resolveDiagramNodeFrame(targetNode, targetKind);
  const anchors = resolveKitDiagramAnchorPair(
    { kind: sourceKind, position: sourcePosition, frame: sourceFrame },
    { kind: targetKind, position: targetPosition, frame: targetFrame },
  );
  return {
    sx: anchors.source.absolutePoint.x,
    sy: anchors.source.absolutePoint.y,
    tx: anchors.target.absolutePoint.x,
    ty: anchors.target.absolutePoint.y,
    sourcePos: toReactFlowPosition(anchors.source.localPoint.side),
    targetPos: toReactFlowPosition(anchors.target.localPoint.side),
  };
};

const FloatingEdge: FC<EdgeProps> = ({ id, source, target, markerEnd, style, selected, data }) => {
  const sourceNode = useInternalNode(source);
  const targetNode = useInternalNode(target);
  const debugLogRef = useRef(0);

  if (!sourceNode || !targetNode) {
    return null;
  }

  const { sx, sy, tx, ty, sourcePos: sPos, targetPos: tPos } = resolveDiagramEdgeAnchors(sourceNode, targetNode);

  const [edgePath] = getBezierPath({
    sourceX: sx,
    sourceY: sy,
    sourcePosition: sPos,
    targetX: tx,
    targetY: ty,
    targetPosition: tPos,
  });

  const relationship = data?.relationship as "part-of" | "reference";
  let stroke = relationship === "part-of" ? "var(--accent-secondary)" : "var(--foreground)";
  let strokeWidth = relationship === "reference" ? 1 : 3;
  let dasharray = relationship === "reference" ? "5 5" : undefined;
  let opacity = 1;

  if (selected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }



  return (
    <g>
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          ...style,
          stroke,
          strokeWidth,
          strokeDasharray: dasharray,
          opacity,
        }}
        className="transition-colors duration-200"
      />
      {KIT_DIAGRAM_DEBUG ? <circle cx={sx} cy={sy} r={3} fill="var(--accent-secondary)" stroke="none" pointerEvents="none" /> : null}
      {KIT_DIAGRAM_DEBUG ? <circle cx={tx} cy={ty} r={3} fill="var(--foreground)" stroke="none" pointerEvents="none" /> : null}
    </g>
  );
};

const FloatingConnectionLine: FC<ConnectionLineComponentProps> = ({ fromX, fromY, toX, toY, fromNode, toNode, pointer }) => {
  const { getNodes } = useReactFlow();
  const fromKind = resolveDiagramNodeKind(fromNode);
  const fromPosition = resolveDiagramNodePosition(fromNode);
  const fromFrame = resolveDiagramNodeFrame(fromNode, fromKind);
  const targetPoint = pointer ?? { x: toX, y: toY };
  const sourceDirection = kitDiagramVector(
    kitDiagramToAbsolutePoint(fromPosition, { x: fromFrame.width / 2, y: fromFrame.height / 2 }),
    targetPoint,
  );
  const sourceAnchor = getKitDiagramShapeStrategy(fromKind).resolveNearestPoint(sourceDirection, fromFrame);
  let sourceX = kitDiagramToAbsolutePoint(fromPosition, sourceAnchor).x;
  let sourceY = kitDiagramToAbsolutePoint(fromPosition, sourceAnchor).y;
  let sourcePosition = toReactFlowPosition(sourceAnchor.side);
  let targetX = toX;
  let targetY = toY;
  let targetPosition = toNode ? Position.Top : toY >= fromY ? Position.Bottom : Position.Top;

  if (toNode) {
    const resolved = resolveDiagramEdgeAnchors(fromNode, toNode);
    sourceX = resolved.sx;
    sourceY = resolved.sy;
    sourcePosition = resolved.sourcePos;
    targetX = resolved.tx;
    targetY = resolved.ty;
    targetPosition = resolved.targetPos;
  } else {
    const proximity = getNodes()
      .filter((node) => node.id !== fromNode.id)
      .map((node) => {
        const kind = resolveDiagramNodeKind(node);
        const position = resolveDiagramNodePosition(node);
        const frame = resolveDiagramNodeFrame(node, kind);
        return resolveKitDiagramProximityAnchor(node.id, { kind, position, frame }, targetPoint);
      })
      .sort((a, b) => a.distance - b.distance)[0];

    if (proximity && proximity.distance <= KIT_DIAGRAM_PROXIMITY_CONNECT_DISTANCE) {
      targetX = proximity.anchor.absolutePoint.x;
      targetY = proximity.anchor.absolutePoint.y;
      targetPosition = toReactFlowPosition(proximity.anchor.localPoint.side);
    } else {
      targetPosition = toReactFlowPosition(kitDiagramInferSnapSide({ x: toX - fromPosition.x, y: toY - fromPosition.y }, fromFrame, fromFrame));
    }
  }
  const edgePath = getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
  })[0];

  return <BaseEdge path={edgePath} style={{ stroke: "var(--active-base)", strokeWidth: 3 }} />;
};

const buildKitDiagramData = (kit: Kit): { nodes: Node<KitDiagramNode>[]; edges: Edge[] } => {
  const nodes: Node<KitDiagramNode>[] = [];
  const edges: Edge[] = [];

  const kindGroups: KitDiagramNodeKind[] = ["type", "design", "quality", "port", "tag", "concept", "file", "folder", "author"];

  for (const kind of kindGroups) {
    let items: Array<{ guid: string; name: string; icon?: any; parentGuid?: string; concepts?: string[] }> = [];

    switch (kind) {
      case "type":
        items = (kit.types ?? []).map((t) => ({
          guid: t.guid,
          name: t.name,
          icon: t.icon,
          parentGuid: t.parent?.guid,
          concepts: t.concepts?.map((c) => c.guid),
        }));
        break;
      case "design":
        items = (kit.designs ?? []).map((d) => ({
          guid: d.guid,
          name: d.name,
          icon: d.icon,
          parentGuid: d.parent?.guid,
          concepts: d.concepts?.map((c) => c.guid),
        }));
        break;
      case "quality":
        items = (kit.qualities ?? []).map((q) => ({ guid: q.guid, name: q.name, icon: q.icon }));
        break;
      case "port":
        items = (kit.ports ?? []).map((i) => ({ guid: i.guid, name: i.name, icon: i.icon }));
        break;
      case "tag":
        items = (kit.tags ?? []).map((t) => ({ guid: t.guid, name: t.name, icon: t.icon }));
        break;
      case "concept":
        items = (kit.concepts ?? []).map((c) => ({ guid: c.guid, name: c.name, icon: c.icon }));
        break;
      case "file":
        items = (kit.files ?? []).map((f) => ({ guid: f.guid, name: f.name, icon: getFileIcon(f.name), parentGuid: f.folder?.guid }));
        break;
      case "folder":
        items = (kit.folders ?? []).map((f) => ({ guid: f.guid, name: f.name, icon: <FolderIcon className="size-tiny" />, parentGuid: f.parent?.guid }));
        break;
      case "author":
        items = (kit.authors ?? []).map((a) => ({ guid: a.guid, name: a.name, icon: <UserIcon className="size-tiny" /> }));
        break;
    }

    for (const item of items) {
      const nodeId = `${kind}:${item.guid}`;
      const frame = getKitDiagramNodeFrameForKind(kind);
      nodes.push({
        id: nodeId,
        type: "artifact",
        position: { x: 0, y: 0 },
        width: frame.width,
        height: frame.height,
        data: {
          guid: item.guid,
          name: item.name,
          kind,
          icon: item.icon,
          parentGuid: item.parentGuid,
          concepts: item.concepts,
        },
      });

      if (item.parentGuid) {
        let parentKind = kind;
        if (kind === "file") parentKind = "folder";
        edges.push({
          id: `${kind}-${item.parentGuid}-${item.guid}`,
          source: `${parentKind}:${item.parentGuid}`,
          target: nodeId,
          type: "floating",
          style: edgeStyle["part-of"],
          data: { relationship: "part-of" },
        });
      }
    }
  }

  for (const design of kit.designs ?? []) {
    for (const piece of design.pieces ?? []) {
      if (piece.type?.guid) {
        const typeGuid = piece.type.guid;
        const sourceId = `type:${typeGuid}`;
        const targetId = `design:${design.guid}`;
        const edgeId = `ref-${sourceId}-${targetId}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });
        }
      }
      if (piece.design?.guid) {
        const nestedDesignGuid = piece.design.guid;
        const sourceId = `design:${nestedDesignGuid}`;
        const targetId = `design:${design.guid}`;
        const edgeId = `ref-${sourceId}-${targetId}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: sourceId,
            target: targetId,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });
        }
      }
    }
  }

  return { nodes, edges };
};

interface KitDiagramProps {}

interface ForceNode extends SimulationNodeDatum {
  id: string;
  data: KitDiagramNode;
}

interface ForceLink extends SimulationLinkDatum<ForceNode> {
  id: string;
  relationship: "part-of" | "reference";
}

const KitDiagramInner: FC = () => {
  const kit = useKit() as Kit | undefined;
  const reactFlowWrapper = useRef<HTMLDivElement>(null);
  const kitCommands = useKitAppCommands();
  const [selection] = useKitAppSelection();
  const [setHover] = useKitAppSetHover();
  const [clearHover] = useKitAppClearHover();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const [activeTool] = useKitAppActiveTool();
  const isHandTool = activeTool === ToolKind.HAND;
  const actor = useSketchpadActor();
  const [diagramForce] = useKitAppDiagramForce();
  const [diagramNodes, setDiagramNodes] = useState<Node<KitDiagramNode>[]>([]);
  const [diagramEdges, setDiagramEdges] = useState<Edge[]>([]);
  const diagramNodesRef = useRef<Node<KitDiagramNode>[]>([]);
  const diagramEdgesRef = useRef<Edge[]>([]);
  const simulationRef = useRef<Simulation<ForceNode, ForceLink> | null>(null);
  const draggingNodeIdRef = useRef<string | null>(null);
  const pinnedNodeIdsRef = useRef<Set<string>>(new Set());
  const isDragReheatActiveRef = useRef(false);
  const debugTickRef = useRef(0);
  const diagramForceConfig = useMemo(() => ({ ...defaultDiagramForceSettings, ...diagramForce }), [diagramForce]);

  const filterSearchSelector = useMemo(() => createKitFilterSearchSelector(kitGuid), [kitGuid]);
  const expandedRowsSelector = useMemo(() => createKitExpandedRowsSelector(kitGuid), [kitGuid]);
  const filterSearch = useSelector(actor, filterSearchSelector) ?? "";
  const expandedRowsSet = useSelector(actor, expandedRowsSelector);
  const expandedRowsArray = useMemo(() => (expandedRowsSet ? Array.from(expandedRowsSet) : []), [expandedRowsSet]);
  const expandedRowsKey = expandedRowsArray.join(",");
  const expandedRows = useMemo(() => new Set(expandedRowsArray), [expandedRowsKey]);

  const [searchParams] = useSearchParams();
  const selectedKinds = useMemo(() => new Set(searchParams.getAll("kind") as ArtifactKind[]), [searchParams]);

  const visibleGuids = useMemo(() => {
    if (!kit) return new Set<string>();
    const guids = new Set<string>();
    const searchLower = filterSearch.toLowerCase();

    const designByGuid = new Map((kit.designs ?? []).map((d) => [d.guid, d]));
    const typeByGuid = new Map((kit.types ?? []).map((t) => [t.guid, t]));
    const folderByGuid = new Map((kit.folders ?? []).map((f) => [f.guid, f]));

    const isAncestorChainExpanded = (rowId: string, parentRowId: string | undefined): boolean => {
      if (!parentRowId) return true;
      if (!expandedRows.has(parentRowId)) return false;

      const [kind, guid] = parentRowId.split("-", 2);
      if (kind === "design") {
        const parentDesign = designByGuid.get(guid);
        if (parentDesign?.parent?.guid) {
          return isAncestorChainExpanded(parentRowId, `design-${parentDesign.parent.guid}`);
        }

        if (parentDesign?.folder) {
          return isAncestorChainExpanded(parentRowId, `folder-${parentDesign.folder}`);
        }
      } else if (kind === "type") {
        const parentType = typeByGuid.get(guid);
        if (parentType?.parent?.guid) {
          return isAncestorChainExpanded(parentRowId, `type-${parentType.parent.guid}`);
        }

        if (parentType?.folder) {
          return isAncestorChainExpanded(parentRowId, `folder-${parentType.folder}`);
        }
      } else if (kind === "folder") {
        const parentFolder = folderByGuid.get(guid);
        if (parentFolder?.parent?.guid) {
          return isAncestorChainExpanded(parentRowId, `folder-${parentFolder.parent.guid}`);
        }
      }
      return true;
    };

    const isRowVisible = (rowId: string, parentRowId: string | undefined, name: string, kind: ArtifactKind): boolean => {
      if (searchLower && !name.toLowerCase().includes(searchLower)) return false;
      if (selectedKinds.size > 0 && !selectedKinds.has(kind)) return false;
      return isAncestorChainExpanded(rowId, parentRowId);
    };

    (kit.designs ?? []).forEach((d) => {
      const rowId = `design-${d.guid}`;
      let parentRowId: string | undefined;
      if (d.parent?.guid) {
        parentRowId = `design-${d.parent.guid}`;
      } else if (d.folder) {
        parentRowId = `folder-${d.folder}`;
      }
      if (isRowVisible(rowId, parentRowId, d.name, "designs")) {
        guids.add(d.guid);
      }
    });

    (kit.types ?? []).forEach((t) => {
      const rowId = `type-${t.guid}`;
      let parentRowId: string | undefined;
      if (t.parent?.guid) {
        parentRowId = `type-${t.parent.guid}`;
      } else if (t.folder) {
        parentRowId = `folder-${t.folder}`;
      }
      if (isRowVisible(rowId, parentRowId, t.name, "types")) {
        guids.add(t.guid);
      }
    });

    (kit.qualities ?? []).forEach((q) => {
      const rowId = `quality-${q.guid}`;
      const parentRowId = q.folder ? `folder-${q.folder}` : undefined;
      if (isRowVisible(rowId, parentRowId, q.name, "qualities")) {
        guids.add(q.guid);
      }
    });

    (kit.ports ?? []).forEach((i) => {
      if ((!searchLower || i.name.toLowerCase().includes(searchLower)) && (selectedKinds.size === 0 || selectedKinds.has("ports"))) {
        guids.add(i.guid);
      }
    });

    (kit.tags ?? []).forEach((t) => {
      if ((!searchLower || t.name.toLowerCase().includes(searchLower)) && (selectedKinds.size === 0 || selectedKinds.has("tags"))) {
        guids.add(t.guid);
      }
    });

    (kit.concepts ?? []).forEach((c) => {
      if ((!searchLower || c.name.toLowerCase().includes(searchLower)) && (selectedKinds.size === 0 || selectedKinds.has("concepts"))) {
        guids.add(c.guid);
      }
    });

    (kit.files ?? []).forEach((f) => {
      const rowId = `file-${f.guid}`;
      const parentRowId = f.folder?.guid ? `folder-${f.folder.guid}` : undefined;
      if (isRowVisible(rowId, parentRowId, f.name, "files")) {
        guids.add(f.guid);
      }
    });

    (kit.folders ?? []).forEach((f) => {
      const rowId = `folder-${f.guid}`;
      const parentRowId = f.parent?.guid ? `folder-${f.parent.guid}` : undefined;
      if (isRowVisible(rowId, parentRowId, f.name, "folders")) {
        guids.add(f.guid);
      }
    });

    (kit.authors ?? []).forEach((a) => {
      if ((!searchLower || a.name.toLowerCase().includes(searchLower)) && (selectedKinds.size === 0 || selectedKinds.has("authors"))) {
        guids.add(a.guid);
      }
    });

    return guids;
  }, [kit, filterSearch, expandedRows, selectedKinds]);

  const { nodes: baseNodes, edges: baseEdges } = useMemo(() => {
    if (!kit) return { nodes: [], edges: [] };
    const { nodes: rfNodes, edges: rfEdges } = buildKitDiagramData(kit);

    const filteredNodes = rfNodes.filter((n) => visibleGuids.has(n.data.guid)).map((node) => {
      const [kind, guid] = node.id.split(":");
      let isSelected = false;
      if (kind === "type") isSelected = selection?.types?.includes(guid) ?? false;
      else if (kind === "design") isSelected = selection?.designs?.includes(guid) ?? false;
      else if (kind === "quality") isSelected = selection?.qualities?.includes(guid) ?? false;
      else if (kind === "port") isSelected = selection?.ports?.includes(guid) ?? false;
      else if (kind === "tag") isSelected = selection?.tags?.includes(guid) ?? false;
      else if (kind === "concept") isSelected = selection?.concepts?.includes(guid) ?? false;
      else if (kind === "file") isSelected = selection?.files?.includes(guid) ?? false;
      else if (kind === "folder") isSelected = selection?.folders?.includes(guid) ?? false;
      else if (kind === "author") isSelected = selection?.authors?.includes(guid) ?? false;

      return {
        ...node,
        selected: isSelected,
        style: { ...node.style, width: node.width ?? getKitDiagramNodeFrameForKind(node.data.kind).width, height: node.height ?? getKitDiagramNodeFrameForKind(node.data.kind).height },
      };
    });

    const filteredNodeIds = new Set(filteredNodes.map((n) => n.id));
    const filteredEdges = rfEdges.filter((e) => filteredNodeIds.has(e.source) && filteredNodeIds.has(e.target));

    return { nodes: filteredNodes, edges: filteredEdges };
  }, [kit, visibleGuids, selection]);
  const baseNodeIdsKey = useMemo(() => baseNodes.map((node) => node.id).join("|"), [baseNodes]);
  const baseEdgeIdsKey = useMemo(() => baseEdges.map((edge) => edge.id).join("|"), [baseEdges]);

  const commitDiagramNodes = useCallback(
    (nextNodes: Node<KitDiagramNode>[]) => {
      diagramNodesRef.current = nextNodes;
      setDiagramNodes(nextNodes);
    },
    [],
  );

  const syncSimulationPositions = useCallback((positions: Map<string, Node["position"]>) => {
    const simulation = simulationRef.current;
    if (!simulation) return;
    for (const simNode of simulation.nodes()) {
      const pos = positions.get(simNode.id);
      if (pos) {
        simNode.x = pos.x;
        simNode.y = pos.y;
      }
    }
  }, []);

  const setPinnedPositions = useCallback((positions: Map<string, Node["position"]>) => {
    pinnedNodeIdsRef.current = new Set(positions.keys());
    const simulation = simulationRef.current;
    if (!simulation) return;
    for (const simNode of simulation.nodes()) {
      const pos = positions.get(simNode.id);
      if (pos) {
        simNode.fx = pos.x;
        simNode.fy = pos.y;
      }
    }
  }, []);

  const clearPinnedPositions = useCallback(() => {
    pinnedNodeIdsRef.current = new Set();
    const simulation = simulationRef.current;
    if (!simulation) return;
    for (const simNode of simulation.nodes()) {
      simNode.fx = null;
      simNode.fy = null;
    }
  }, []);

  const logSimulationState = useCallback(
    (label: string, node?: Node) => {
      if (!KIT_DIAGRAM_DEBUG) return;
      const simulation = simulationRef.current;
      if (!simulation) return;

    },
    [],
  );

  const startDragReheat = useCallback(() => {
    const simulation = simulationRef.current;
    if (!simulation) return;
    if (!isDragReheatActiveRef.current) {
      isDragReheatActiveRef.current = true;
    }
    simulation.alphaTarget(0.3).restart();
  }, []);

  const stopDragReheat = useCallback(() => {
    const simulation = simulationRef.current;
    if (simulation) {
      simulation.alphaTarget(0);
    }
    isDragReheatActiveRef.current = false;
    clearPinnedPositions();
    draggingNodeIdRef.current = null;
    logSimulationState("drag-end");
  }, [clearPinnedPositions, logSimulationState]);

  const updateNodesFromSimulation = useCallback(() => {
    const simulation = simulationRef.current;
    if (!simulation) return;
    const pinnedNodeIds = pinnedNodeIdsRef.current;
    const simNodeById = new Map(simulation.nodes().map((node) => [node.id, node]));
    const nextNodes = diagramNodesRef.current.map((node) => {
      if (pinnedNodeIds.has(node.id)) {
        return node;
      }
      const simNode = simNodeById.get(node.id);
      if (!simNode) return node;
      return { ...node, position: { x: simNode.x ?? 0, y: simNode.y ?? 0 } };
    });
    commitDiagramNodes(nextNodes);

  }, [commitDiagramNodes, baseEdgeIdsKey, baseNodeIdsKey]);

  useEffect(() => {
    const previousPositions = new Map(diagramNodesRef.current.map((node) => [node.id, node.position]));
    const nextNodes = baseNodes.map((node) => {
      const previousPosition = previousPositions.get(node.id);
      return previousPosition ? { ...node, position: previousPosition } : node;
    });
    commitDiagramNodes(nextNodes);
    diagramEdgesRef.current = baseEdges;
    setDiagramEdges(baseEdges);
  }, [baseNodes, baseEdges, commitDiagramNodes]);

  useEffect(() => {
    if (diagramNodesRef.current.length === 0) {
      if (simulationRef.current) {
        simulationRef.current.alphaTarget(0);
        simulationRef.current.stop();
        simulationRef.current = null;
      }
      return;
    }
    const nodesSnapshot = diagramNodesRef.current;
    const edgesSnapshot = diagramEdgesRef.current;
    const nodesCopy: ForceNode[] = nodesSnapshot.map((node) => ({
      id: node.id,
      x: node.position.x,
      y: node.position.y,
      data: node.data,
    }));
    const linksCopy: ForceLink[] = edgesSnapshot.map((edge) => ({
      id: edge.id,
      source: edge.source,
      target: edge.target,
      relationship: (edge.data?.relationship as "part-of" | "reference") ?? "reference",
    }));
    const simulation = forceSimulation<ForceNode, ForceLink>(nodesCopy)
      .force("charge", forceManyBody().strength(diagramForceConfig.chargeStrength))
      .force(
        "link",
        forceLink<ForceNode, ForceLink>(linksCopy)
          .id((d) => d.id)
          .distance(diagramForceConfig.linkDistance),
      )
      .force("collide", forceCollide().radius(diagramForceConfig.collideRadius))
      .force("x", forceX(0).strength(diagramForceConfig.centerStrength))
      .force("y", forceY(0).strength(diagramForceConfig.centerStrength));
    simulationRef.current = simulation;
    const snapshotPositions = new Map(nodesSnapshot.map((node) => [node.id, node.position]));
    syncSimulationPositions(snapshotPositions);
    const numTicks = Math.ceil(Math.log(simulation.alphaMin()) / Math.log(1 - simulation.alphaDecay()));
    for (let i = 0; i < numTicks; i += 1) {
      simulation.tick();
    }
    updateNodesFromSimulation();
    simulation.on("tick", updateNodesFromSimulation);

    simulation.alpha(1).restart();
    return () => {
      stopDragReheat();
      simulation.alphaTarget(0);
      simulation.stop();
      simulationRef.current = null;
    };
  }, [
    baseEdgeIdsKey,
    baseNodeIdsKey,
    diagramForceConfig.centerStrength,
    diagramForceConfig.chargeStrength,
    diagramForceConfig.collideRadius,
    diagramForceConfig.linkDistance,
    stopDragReheat,
    syncSimulationPositions,
    updateNodesFromSimulation,
  ]);

  const handleNodesChange = useCallback(
    (changes: any[]) => {
      const updatedNodes = applyNodeChanges(changes, diagramNodesRef.current);
      const draggingNodeId = draggingNodeIdRef.current;
      if (draggingNodeId && simulationRef.current) {
        const selectedNodes = updatedNodes.filter((node) => node.selected);
        const selectedPositions = new Map(selectedNodes.map((node) => [node.id, node.position]));
        if (selectedPositions.size > 1 && selectedPositions.has(draggingNodeId)) {
          setPinnedPositions(selectedPositions);
        } else {
          const draggedNode = updatedNodes.find((node) => node.id === draggingNodeId);
          if (draggedNode) {
            setPinnedPositions(new Map([[draggedNode.id, draggedNode.position]]));
          }
        }
      }
      commitDiagramNodes(updatedNodes);
    },
    [commitDiagramNodes, setPinnedPositions],
  );

  const handleNodeDragStart = useCallback(
    (_: any, node: Node) => {
      draggingNodeIdRef.current = node.id;
      const currentPositions = new Map(diagramNodesRef.current.map((item) => [item.id, item.position]));
      currentPositions.set(node.id, node.position);
      syncSimulationPositions(currentPositions);
      const selectedNodes = diagramNodesRef.current.filter((item) => item.selected);
      const selectedPositions = new Map(selectedNodes.map((item) => [item.id, item.position]));
      if (node.selected) {
        selectedPositions.set(node.id, node.position);
      }
      if (selectedPositions.size > 1 && node.selected) {
        setPinnedPositions(selectedPositions);
      } else {
        setPinnedPositions(new Map([[node.id, node.position]]));
      }
      startDragReheat();
      logSimulationState("drag-start", node);
    },
    [logSimulationState, setPinnedPositions, startDragReheat, syncSimulationPositions],
  );

  const handleNodeDrag = useCallback(
    (_: any, node: Node) => {
      if (draggingNodeIdRef.current !== node.id) return;
      const selectedNodes = diagramNodesRef.current.filter((item) => item.selected);
      const selectedPositions = new Map(selectedNodes.map((item) => [item.id, item.position]));
      if (node.selected) {
        selectedPositions.set(node.id, node.position);
      }
      if (selectedPositions.size > 1 && node.selected) {
        setPinnedPositions(selectedPositions);
      } else {
        setPinnedPositions(new Map([[node.id, node.position]]));
      }
      startDragReheat();
      logSimulationState("drag", node);
    },
    [logSimulationState, setPinnedPositions, startDragReheat],
  );

  const handleNodeDragStop = useCallback(
    (_: any, _node: Node) => {
      stopDragReheat();
    },
    [stopDragReheat],
  );

  const handleDragEndFallback = useCallback(() => {
    if (!draggingNodeIdRef.current && !isDragReheatActiveRef.current) return;
    stopDragReheat();
  }, [stopDragReheat]);

  useEffect(() => {
    const wrapper = reactFlowWrapper.current;
    window.addEventListener("pointerup", handleDragEndFallback);
    window.addEventListener("pointercancel", handleDragEndFallback);
    window.addEventListener("blur", handleDragEndFallback);
    if (wrapper) {
      wrapper.addEventListener("pointerleave", handleDragEndFallback);
    }
    return () => {
      window.removeEventListener("pointerup", handleDragEndFallback);
      window.removeEventListener("pointercancel", handleDragEndFallback);
      window.removeEventListener("blur", handleDragEndFallback);
      if (wrapper) {
        wrapper.removeEventListener("pointerleave", handleDragEndFallback);
      }
    };
  }, [handleDragEndFallback]);

  const onSelectionChange = useCallback(
    ({ nodes: selectedNodes }: any) => {
      const newSelection: KitAppSelection = {};
      selectedNodes.forEach((node: any) => {
        const [kind, guid] = node.id.split(":");
        if (kind === "type") {
          if (!newSelection.types) newSelection.types = [];
          newSelection.types.push(guid);
        } else if (kind === "design") {
          if (!newSelection.designs) newSelection.designs = [];
          newSelection.designs.push(guid);
        } else if (kind === "quality") {
          if (!newSelection.qualities) newSelection.qualities = [];
          newSelection.qualities.push(guid);
        } else if (kind === "port") {
          if (!newSelection.ports) newSelection.ports = [];
          newSelection.ports.push(guid);
        } else if (kind === "tag") {
          if (!newSelection.tags) newSelection.tags = [];
          newSelection.tags.push(guid);
        } else if (kind === "concept") {
          if (!newSelection.concepts) newSelection.concepts = [];
          newSelection.concepts.push(guid);
        } else if (kind === "file") {
          if (!newSelection.files) newSelection.files = [];
          newSelection.files.push(guid);
        } else if (kind === "folder") {
          if (!newSelection.folders) newSelection.folders = [];
          newSelection.folders.push(guid);
        } else if (kind === "author") {
          if (!newSelection.authors) newSelection.authors = [];
          newSelection.authors.push(guid);
        }
      });

      actor.send({ type: "KIT.SET_SELECTION", kitGuid, selection: newSelection });
    },
    [actor, kitGuid],
  );

  const handlePaneClick = useCallback(() => {
    kitCommands.deselectAll?.();
  }, [kitCommands]);

  const handleNodeMouseEnter = useCallback(
    (_: any, node: any) => {
      const data = node.data as KitDiagramNode;
      const kind = data?.kind;
      const guid = data?.guid;
      if (!kind || !guid) return;
      if (!setHover) return;
      if (kind === "type") setHover({ type: guid });
      else if (kind === "design") setHover({ design: guid });
      else if (kind === "quality") setHover({ quality: guid });
      else if (kind === "port") setHover({ port: guid });
      else if (kind === "tag") setHover({ tag: guid });
      else if (kind === "concept") setHover({ concept: guid });
      else if (kind === "file") setHover({ file: guid });
      else if (kind === "folder") setHover({ folder: guid });
      else if (kind === "author") setHover({ author: guid });
    },
    [setHover],
  );

  const handleNodeMouseLeave = useCallback(() => {
    if (clearHover) clearHover();
  }, [clearHover]);

  if (!kit) return null;

  return (
    <div ref={reactFlowWrapper} className="w-full h-full" data-testid="kit-diagram">
      <Diagram
        nodes={diagramNodes}
        edges={diagramEdges}
        nodeTypes={kitNodeTypes}
        edgeTypes={{ floating: FloatingEdge }}
        connectionLineComponent={FloatingConnectionLine}
        forceConfig={{ enabled: false }}
        elementsSelectable={true}
        nodesFocusable={true}
        edgesFocusable={true}
        onSelectionChange={onSelectionChange}
        onNodesChangeReactFlow={handleNodesChange}
        onNodeMouseEnter={handleNodeMouseEnter}
        onNodeMouseLeave={handleNodeMouseLeave}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onPaneClick={handlePaneClick}
        selectionMode={SelectionMode.Partial}
        panOnScroll={false}
        panOnDrag={isHandTool ? true : [1, 2]}
        selectionOnDrag={!isHandTool}
        nodesDraggable={!isHandTool}
        proOptions={{ hideAttribution: true }}
      />
    </div>
  );
};

const KitDiagram: FC<KitDiagramProps> = () => {
  return (
    <ReactFlowProvider>
      <KitDiagramInner />
    </ReactFlowProvider>
  );
};

const TableWindow = memo(() => {
  return <AppContent />;
});
TableWindow.displayName = "TableWindow";

const DiagramWindow = memo(() => {
  return <KitDiagram />;
});
DiagramWindow.displayName = "DiagramWindow";

const MultiWindowApp: FC = () => {
  useKitAppYjsToXStateSync();
  const transaction = useKitAppTransaction();
  const actor = useSketchpadActor();
  const sketchpadStore = useSketchpadStore();
  const kitGuid = useKitScope()?.guid;
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const [activeWindow, setActiveWindow] = useState<string>(KitAppWindowKind.Table);
  const [activeTool, setActiveTool] = useKitAppActiveTool();

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
    if (appType !== "kit") return;
    if (!kitGuid) return;

    console.log("[DEBUG] Kit.tsx registering toolbar sections for kit:", kitGuid);

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.selection",
      specificity: 20,
      order: 10,
      toolbarGroup: {
        id: "selection",
        labelId: "semio.sketchpad.toolbar.parent.selection",
        order: 10,
        subToolId: "select",
        subToolLabelId: "semio.sketchpad.toolbar.subtool.select",
        subToolIcon: <MousePointerIcon className="size-tiny" />,
      },
      content: () => (
        <KitScopeProvider guid={kitGuid}>
          <KitToolbarSelection />
        </KitScopeProvider>
      ),
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.filters",
      specificity: 20,
      order: 20,
      toolbarGroup: {
        id: "filter",
        labelId: "semio.sketchpad.toolbar.parent.filter",
        order: 20,
      },
      content: () => (
        <KitScopeProvider guid={kitGuid}>
            <KitFilters />
        </KitScopeProvider>
      ),
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.toolbar.create",
      specificity: 20,
      order: 30,
      toolbarGroup: {
        id: "create",
        labelId: "semio.sketchpad.toolbar.parent.create",
        order: 30,
      },
      content: () => {
        console.log("[DEBUG] Rendering KitCreateActions content wrapper");
        return (
        <KitScopeProvider guid={kitGuid}>
            <KitCreateActions />
        </KitScopeProvider>
      )},
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.selection");
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.filters");
      removeSection("toolbar", "semio.sketchpad.app.kit.toolbar.create");
      removeSection("toolbar", "semio.sketchpad.app.kit.kitApp.toolsGroup");
    };
  }, [appType, addSection, removeSection, kitGuid]);

  const hasKit = useHasKit(kitGuid || "");

  const store = useMemo(() => {
    if (!kitGuid || !sketchpadStore?.hasKitApp?.({ kit: kitGuid })) return null;
    return sketchpadStore.kitApp(kitGuid);
  }, [sketchpadStore, kitGuid]);

  const storedWindowLayout = useSyncDeep<any, any>(store, (s: KitAppState | null) => s?.windowLayout);

  const defaultLayout = useMemo(
    () => ({
      root: {
        type: "row",
        content: [
          {
            type: "stack",
            size: "50%",
            content: [
              {
                type: "component",
                componentName: KitAppWindowKind.Table,
                title: "table",
                componentState: {},
              },
              {
                type: "component",
                componentName: KitAppWindowKind.Settings,
                title: "settings",
                componentState: {},
              },
              {
                type: "component",
                componentName: KitAppWindowKind.Chat,
                title: "chat",
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
                componentName: KitAppWindowKind.Diagram,
                title: "diagram",
                componentState: {},
              },
            ],
          },
        ],
      },
    }),
    [],
  );

  const windowLayout = useMemo(() => storedWindowLayout || defaultLayout, [storedWindowLayout, defaultLayout]);

  const windowConfig: AppWindowConfig = useMemo(
    () => ({
      windowKinds: [
        {
          id: KitAppWindowKind.Table,
          label: "table",
          component: () => <TableWindow />,
        },
        {
          id: KitAppWindowKind.Diagram,
          label: "diagram",
          component: () => <DiagramWindow />,
        },
        {
          id: KitAppWindowKind.Settings,
          label: "settings",
          component: () => (
            <TreeStateProvider>
              <Tree className="min-w-0 overflow-hidden p-double">
                <KitEditorSettingsContent />
                <SketchpadSettingsContent />
              </Tree>
            </TreeStateProvider>
          ),
        },
        {
          id: KitAppWindowKind.Chat,
          label: "chat",
          component: () => (
            <TreeStateProvider>
              <Tree className="min-w-0 overflow-hidden p-double">
                <TreeItem>
                  <TreeContent>
                    <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.panel.chat.placeholder")}</p>
                  </TreeContent>
                </TreeItem>
              </Tree>
            </TreeStateProvider>
          ),
        },
      ],
      defaultLayout,
    }),
    [defaultLayout],
  );

  useEffect(() => {
    const isValid = windowConfig.windowKinds.some((k) => k.id === activeWindow);
    if (!isValid) {
      setActiveWindow(KitAppWindowKind.Table);
    }
  }, [activeWindow, windowConfig.windowKinds]);

  const handleLayoutChange = useCallback(
    (config: any) => {
      if (store && typeof store.change === "function") {
        store.change({ windowLayout: config });
      }
    },
    [store],
  );

  if (!hasKit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">Loading kit...</p>
      </div>
    );
  }

  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">Failed to load kit app</p>
        </div>
      }
    >
      <TransactionProvider transaction={transaction}>
        <KitDropZone>
          <Canvas id="semio.sketchpad.app.kit.canvas">
            <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
          </Canvas>
        </KitDropZone>
      </TransactionProvider>
    </ErrorBoundary>
  );
};

export default MultiWindowApp;

// #endregion Diagram

// #region Tools

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖windows🔖tools](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/WINDOWS/TOOLS)
// Tools MUST define Kit app toolbar filter and selection tool components.

/**
 * Returns a hook for the Kit app filter search input state.
 *
 * MUST provide the current filter string and a setter.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖windows🔖tools🛠️usekitappfiltersearch](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/WINDOWS/TOOLS/USE-KIT-APP-FILTER-SEARCH)
 **/
export function useKitAppFilterSearch(): HookResult<string> {
  const store = useKitAppStore();
  const filterSearch = useSyncDeep(store, (s: KitAppState | null) => s?.filterSearch ?? "") || "";
  const setFilterSearch = useCallback(
    (value: string) => {
      store?.change({ filterSearch: value });
    },
    [store],
  );
  return [filterSearch, setFilterSearch, !!store];
}

/**
 * Filter toolbar component rendering the search input for Kit artifacts.
 *
 * MUST render a filter input connected to the Kit app filter search state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖windows🔖tools🪨kitfilters](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/WINDOWS/TOOLS/KIT-FILTERS)
 **/
export const KitFilters: FC = () => {
  return (
    <ToolbarGroup>
      <KitKindToggles />
    </ToolbarGroup>
  );
};

/**
 * Toolbar selection tool component for the Kit app.
 *
 * MUST render selection mode toggle buttons.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖windows🔖tools🪨kittoolbarselection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/WINDOWS/TOOLS/KIT-TOOLBAR-SELECTION)
 **/
export const KitToolbarSelection: FC = () => {
  const [activeTool, setActiveTool] = useKitAppActiveTool();
  const additiveLabel = useLabel("semio.sketchpad.app.kit.tools.select.mode.additive");
  const subtractiveLabel = useLabel("semio.sketchpad.app.kit.tools.select.mode.subtractive");
  const intersectLabel = useLabel("semio.sketchpad.app.kit.tools.select.mode.intersect");
  const rectangularLabel = useLabel("semio.sketchpad.app.kit.tools.select.shape.rectangular");
  const lassoLabel = useLabel("semio.sketchpad.app.kit.tools.select.shape.lasso");
  const handLabel = useLabel("semio.sketchpad.app.kit.tools.select.navigation.hand");

  return (
    <ToolbarGroup>
      <ToolbarGroup>
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.mode.additive"
          icon={<AddIcon className="size-tiny" />}
          text={additiveLabel}
          pressed={activeTool === ToolKind.SELECTION_ADDITIVE}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.SELECTION_ADDITIVE : ToolKind.SELECTION_NORMAL)}
        />
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.mode.subtractive"
          icon={<RemoveIcon className="size-tiny" />}
          text={subtractiveLabel}
          pressed={activeTool === ToolKind.SELECTION_SUBTRACTIVE}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.SELECTION_SUBTRACTIVE : ToolKind.SELECTION_NORMAL)}
        />
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.mode.intersect"
          icon={<IntersectIcon className="size-tiny" />}
          text={intersectLabel}
          pressed={activeTool === ToolKind.SELECTION_INTERSECT}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.SELECTION_INTERSECT : ToolKind.SELECTION_NORMAL)}
        />
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.shape.rectangular"
          icon={<DiagramIcon className="size-tiny" />}
          text={rectangularLabel}
          pressed={activeTool === ToolKind.LASSO_RECTANGULAR}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.LASSO_RECTANGULAR : ToolKind.SELECTION_NORMAL)}
        />
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.shape.lasso"
          icon={<SceneIcon className="size-tiny" />}
          text={lassoLabel}
          pressed={activeTool === ToolKind.LASSO_FREEFORM}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.LASSO_FREEFORM : ToolKind.SELECTION_NORMAL)}
        />
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <Toggle
          id="semio.sketchpad.app.kit.tools.select.navigation.hand"
          icon={<HandIcon className="size-tiny" />}
          text={handLabel}
          pressed={activeTool === ToolKind.HAND}
          onPressedChange={(pressed) => setActiveTool?.(pressed ? ToolKind.HAND : ToolKind.SELECTION_NORMAL)}
        />
      </ToolbarGroup>
    </ToolbarGroup>
  );
};

// #endregion Tools

// #endregion Windows

// #region Panels

// [🔖semio/js/sketchpad/Kit.tsx#Panels](semiorepo://section/semio/js/sketchpad/Kit.tsx/PANELS)

// #region Right

// [🔖semio/js/sketchpad/Kit.tsx#Right](semiorepo://section/semio/js/sketchpad/Kit.tsx/RIGHT)

// #region Details

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS)
// Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.

/**
 * Detail section component for the currently open kit.
 *
 * MUST render the kit metadata form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨kitsection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/KIT-SECTION)
 **/
export const KitSection: FC = () => {
  const isInKitScope = useIsInKitScope();
  if (!isInKitScope) return null;
  return <KitSectionForm />;
};

const KitSectionForm: FC = () => {
  const { t } = useTranslation();
  try {
    const kit = useKit() as Kit;
    if (!kit) {
      return (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.notAvailable")}</p>
          </TreeContent>
        </TreeItem>
      );
    }
    const kitDataSource = useKitAppStore() as any;
    return (
      <>
        <TreeItem>
          <TreeContent>
            <Input lazy id="semio.sketchpad.app.kit.panel.details.section.kit.name" value={kit.name} onLazyChange={(value) => kitDataSource.change({ name: value })} showLabel />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.version"
              value={kit.version || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.versionPlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ version: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Textarea
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.description"
              value={kit.description || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.descriptionPlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ description: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.icon"
              value={kit.icon || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.iconPlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ icon: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.image"
              value={kit.image || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.imagePlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ image: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.homepage"
              value={kit.homepage || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.homepagePlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ homepage: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.license"
              value={kit.license || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.licensePlaceholder.label")}
              onLazyChange={(value) => kitDataSource.change({ license: value })}
              showLabel
            />
          </TreeContent>
        </TreeItem>
      </>
    );
  } catch (error) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.notFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }
};

/**
 * Detail section component for the selected type.
 *
 * MUST render the type form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨typesection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/TYPE-SECTION)
 **/
export const TypeSection: FC = () => {
  const [selection] = useKitAppSelection();
  const selectedTypes = selection?.types || [];
  if (selectedTypes.length === 0) return null;
  if (selectedTypes.length === 1) return <SingleTypeSection typeGuid={selectedTypes[0]} />;
  return <MultipleTypesSection typeGuids={selectedTypes} />;
};

const SingleTypeSection: FC<{ typeGuid: string }> = ({ typeGuid }) => {
  const kit = useKit() as Kit;
  const type = kit?.types?.find((t) => t.guid === typeGuid);
  if (!type) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.type.panel.details.section.type.description" value={type.description || ""} placeholderId="semio.sketchpad.app.type.descriptionPlaceholder.label" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.type.panel.details.section.type.icon" value={type.icon || ""} placeholderId="semio.sketchpad.app.type.iconPlaceholder.label" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.type.panel.details.section.type.image" value={type.image || ""} placeholderId="semio.sketchpad.app.type.imagePlaceholder.label" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.type.panel.details.section.type.parent" value={type.parent?.guid || ""} placeholderId="semio.sketchpad.app.type.parentPlaceholder.label" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Toggle id="semio.sketchpad.app.type.panel.details.section.type.abstract" pressed={type.isAbstract || false} disabled showLabel icon={<CheckIcon />} />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input id="semio.sketchpad.app.type.panel.details.section.type.unit" value={type.unit} readOnly showLabel />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

const MultipleTypesSection: FC<{ typeGuids: string[] }> = ({ typeGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const types = typeGuids.map((guid) => kit?.types?.find((t) => t.guid === guid)).filter((t) => t !== undefined) as Type[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.types.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {types.map((type) => (
        <TreeItem key={type.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{type.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected port.
 *
 * MUST render the port form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨portsection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/PORT-SECTION)
 **/
export const PortSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const selectedPorts = selection?.ports || [];
  if (selectedPorts.length === 0) return null;
  if (selectedPorts.length === 1) return <SinglePortSection portGuid={selectedPorts[0]} />;
  return <MultiplePortsSection portGuids={selectedPorts} />;
};

const SinglePortSection: FC<{ portGuid: string }> = ({ portGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const iface = kit?.ports?.find((i) => i.guid === portGuid);
  if (!iface) return null;
  const compatibleCount = iface.compatiblePorts?.length || 0;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.kit.panel.details.section.port.name" value={iface.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.kit.panel.details.section.port.description" value={iface.description || ""} placeholder={t("semio.sketchpad.app.kit.port.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.kit.panel.details.section.port.compatible"
            value={compatibleCount === 0 ? t("semio.sketchpad.app.kit.port.allCompatible") : `${compatibleCount} ${t("semio.sketchpad.app.kit.port.compatiblePorts")}`}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultiplePortsSection: FC<{ portGuids: string[] }> = ({ portGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const ports = portGuids.map((guid) => kit?.ports?.find((i) => i.guid === guid)).filter((i) => i !== undefined) as Port[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.ports.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {ports.map((iface) => (
        <TreeItem key={iface.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{iface.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected tag.
 *
 * MUST render the tag form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨tagsection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/TAG-SECTION)
 **/
export const TagSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const selectedTags = selection?.tags || [];
  if (selectedTags.length === 0) return null;
  if (selectedTags.length === 1) return <SingleTagSection tagGuid={selectedTags[0]} />;
  return <MultipleTagsSection tagGuids={selectedTags} />;
};

const SingleTagSection: FC<{ tagGuid: string }> = ({ tagGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const tag = kit?.tags?.find((t) => t.guid === tagGuid);
  if (!tag) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.kit.panel.details.section.tag.name" value={tag.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.kit.panel.details.section.tag.description" value={tag.description || ""} placeholder={t("semio.sketchpad.app.kit.tag.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleTagsSection: FC<{ tagGuids: string[] }> = ({ tagGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const tags = tagGuids.map((guid) => kit?.tags?.find((t) => t.guid === guid)).filter((t) => t !== undefined) as Tag[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.tags.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {tags.map((tag) => (
        <TreeItem key={tag.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{tag.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected concept.
 *
 * MUST render the concept form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨conceptsection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/CONCEPT-SECTION)
 **/
export const ConceptSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const selectedConcepts = selection?.concepts || [];
  if (selectedConcepts.length === 0) return null;
  if (selectedConcepts.length === 1) return <SingleConceptSection conceptGuid={selectedConcepts[0]} />;
  return <MultipleConceptsSection conceptGuids={selectedConcepts} />;
};

const SingleConceptSection: FC<{ conceptGuid: string }> = ({ conceptGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const concept = kit?.concepts?.find((c) => c.guid === conceptGuid);
  if (!concept) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.kit.panel.details.section.concept.name" value={concept.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.kit.panel.details.section.concept.description" value={concept.description || ""} placeholder={t("semio.sketchpad.app.kit.concept.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleConceptsSection: FC<{ conceptGuids: string[] }> = ({ conceptGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const concepts = conceptGuids.map((guid) => kit?.concepts?.find((c) => c.guid === guid)).filter((c) => c !== undefined) as Concept[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.concepts.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {concepts.map((concept) => (
        <TreeItem key={concept.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{concept.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected design.
 *
 * MUST render the design form fields within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨designsection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/DESIGN-SECTION)
 **/
export const DesignSection: FC = () => {
  const [selection] = useKitAppSelection();
  const selectedDesigns = selection?.designs || [];
  if (selectedDesigns.length === 0) return null;
  if (selectedDesigns.length === 1) return <SingleDesignSection designGuid={selectedDesigns[0]} />;
  return <MultipleDesignsSection designGuids={selectedDesigns} />;
};

const SingleDesignSection: FC<{ designGuid: string }> = ({ designGuid }) => {
  const kit = useKit() as Kit;
  const design = kit?.designs?.find((d) => d.guid === designGuid);
  if (!design) return null;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.name" value={design.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.design.panel.details.section.design.description" value={design.description || ""} placeholderId="semio.sketchpad.app.design.descriptionPlaceholder" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.icon" value={design.icon || ""} placeholderId="semio.sketchpad.app.design.iconPlaceholder" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.image" value={design.image || ""} placeholderId="semio.sketchpad.app.design.imagePlaceholder" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.variant" value={(design as any).variant || ""} placeholderId="semio.sketchpad.app.design.variantPlaceholder" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.view" value={(design as any).view || ""} placeholderId="semio.sketchpad.app.design.viewPlaceholder" readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.design.panel.details.section.design.unit" value={design.unit || ""} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      {design.location && (
        <>
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.location.longitude" value={String((design.location as any)?.longitude ?? 0)} disabled showLabel />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input id="semio.sketchpad.app.design.panel.details.section.location.latitude" value={String((design.location as any)?.latitude ?? 0)} disabled showLabel />
            </TreeContent>
          </TreeItem>
        </>
      )}
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

const MultipleDesignsSection: FC<{ designGuids: string[] }> = ({ designGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const designs = designGuids.map((guid) => kit?.designs?.find((d) => d.guid === guid)).filter((d) => d !== undefined) as Design[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.designs.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {designs.map((design) => (
        <TreeItem key={design.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{design.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected file.
 *
 * MUST render the file metadata within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨filesection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/FILE-SECTION)
 **/
export const FileSection: FC = () => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const [selection] = useKitAppSelection();
  const selectedFiles = selection?.files || [];

  if (selectedFiles.length === 0) return null;

  const files = selectedFiles
    .map((fileGuid) => {
      return kit.files?.find((f) => f.guid === fileGuid);
    })
    .filter(Boolean);

  if (files.length === 0) return null;

  const formatFileSize = (bytes?: number) => {
    if (!bytes) return "0 KB";
    return `${(bytes / 1024).toFixed(1)} KB`;
  };

  const formatDate = (date?: Date | string) => {
    if (!date) return "";
    const parsedDate = date instanceof Date ? date : new Date(date);
    if (isNaN(parsedDate.getTime())) return "";
    return parsedDate.toLocaleDateString();
  };

  return (
    <>
      {files.map((file) => (
        <TreeItem key={file!.guid}>
          <TreeContent>
            <div className="space-y-2">
              <div>
                <label className="text-xs text-muted-foreground">{useLabel("semio.file.name")}</label>
                <p className="text-sm">{file!.name}</p>
              </div>
              <div>
                <label className="text-xs text-muted-foreground">{useLabel("semio.file.size")}</label>
                <p className="text-sm">{formatFileSize(file!.size)}</p>
              </div>
              {file!.createdAt && (
                <div>
                  <label className="text-xs text-muted-foreground">{useLabel("semio.file.created")}</label>
                  <p className="text-sm">{formatDate(file!.createdAt)}</p>
                </div>
              )}
              {file!.updatedAt && (
                <div>
                  <label className="text-xs text-muted-foreground">{useLabel("semio.file.updated")}</label>
                  <p className="text-sm">{formatDate(file!.updatedAt)}</p>
                </div>
              )}
            </div>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

/**
 * Detail section component for the selected folder.
 *
 * MUST render the folder metadata within a detail panel section.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨foldersection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/FOLDER-SECTION)
 **/
export const FolderSection: FC = () => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const kitDataSource = useKitAppStore() as any;
  const [selection] = useKitAppSelection();
  const selectedFolders = selection?.folders || [];

  if (selectedFolders.length === 0) return null;

  const folders = selectedFolders
    .map((folderGuid) => {
      return kit.folders?.find((f) => f.guid === folderGuid);
    })
    .filter(Boolean);

  if (folders.length === 0) return null;
  if (folders.length > 1) return null;

  const folder = folders[0]!;

  const formatDate = (date?: Date | string) => {
    if (!date) return "";
    const parsedDate = date instanceof Date ? date : new Date(date);
    if (isNaN(parsedDate.getTime())) return "";
    return parsedDate.toLocaleDateString();
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.kit.panel.details.section.folder.name"
            value={folder.name}
            onLazyChange={(value) => {
              const folderDataSource = (kitDataSource as any).folder(folder.guid);
              folderDataSource.change({ name: value });
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      {folder.description && (
        <TreeItem>
          <TreeContent>
            <Textarea
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.folder.description"
              value={folder.description || ""}
              placeholder={useLabel("semio.sketchpad.app.folder.descriptionPlaceholder.label")}
              onLazyChange={(value) => {
                const folderDataSource = (kitDataSource as any).folder(folder.guid);
                folderDataSource.change({ description: value });
              }}
              showLabel
            />
          </TreeContent>
        </TreeItem>
      )}
      {folder.createdAt && (
        <TreeItem>
          <TreeContent>
            <div>
              <label className="text-xs text-muted-foreground">{useLabel("semio.folder.created")}</label>
              <p className="text-sm">{formatDate(folder.createdAt)}</p>
            </div>
          </TreeContent>
        </TreeItem>
      )}
      {folder.updatedAt && (
        <TreeItem>
          <TreeContent>
            <div>
              <label className="text-xs text-muted-foreground">{useLabel("semio.folder.updated")}</label>
              <p className="text-sm">{formatDate(folder.updatedAt)}</p>
            </div>
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

/**
 * Detail section component for multiple selected artifacts.
 *
 * MUST render a summary of all selected artifacts across kinds.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖details🪨multipleartifactssection](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/DETAILS/MULTIPLE-ARTIFACTS-SECTION)
 **/
export const MultipleArtifactsSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const typesCount = selection?.types?.length || 0;
  const designsCount = selection?.designs?.length || 0;
  const qualitiesCount = selection?.qualities?.length || 0;
  const portsCount = selection?.ports?.length || 0;
  const tagsCount = selection?.tags?.length || 0;
  const conceptsCount = selection?.concepts?.length || 0;
  const filesCount = selection?.files?.length || 0;
  const authorsCount = selection?.authors?.length || 0;
  const kinds: string[] = [];
  if (typesCount > 0) kinds.push(t("semio.sketchpad.app.kit.types.multipleTitle", { count: typesCount }));
  if (designsCount > 0) kinds.push(t("semio.sketchpad.app.kit.designs.multipleTitle", { count: designsCount }));
  if (qualitiesCount > 0) kinds.push(t("semio.sketchpad.app.kit.qualities.multipleTitle", { count: qualitiesCount }));
  if (portsCount > 0) kinds.push(t("semio.sketchpad.app.kit.ports.multipleTitle", { count: portsCount }));
  if (tagsCount > 0) kinds.push(t("semio.sketchpad.app.kit.tags.multipleTitle", { count: tagsCount }));
  if (conceptsCount > 0) kinds.push(t("semio.sketchpad.app.kit.concepts.multipleTitle", { count: conceptsCount }));
  if (filesCount > 0) kinds.push(t("semio.sketchpad.app.kit.files.multipleTitle", { count: filesCount }));
  if (authorsCount > 0) kinds.push(t("semio.sketchpad.app.kit.authors.multipleTitle", { count: authorsCount }));
  if (kinds.length <= 1) return null;
  return (
    <TreeItem>
      <TreeContent>
        <p className="text-sm text-muted-foreground">{kinds.join(", ")}</p>
      </TreeContent>
    </TreeItem>
  );
};

// #endregion Details

// #region Settings

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖canvas🔖panels🔖right🔖settings](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CANVAS/PANELS/RIGHT/SETTINGS)
// Settings MUST render the Kit app settings panel with theme, language, device, expertise, mode, and diagram force controls.

const KitEditorSettingsContent: FC = () => {
  const [diagramForce, setDiagramForce, canSetDiagramForce] = useKitAppDiagramForce();
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.kit.settings.diagram.chargeStrength"
            showLabel
            min={-500}
            max={0}
            step={10}
            value={[diagramForce.chargeStrength]}
            onValueChange={(value: number[]) => setDiagramForce?.({ chargeStrength: value[0] })}
            disabled={!canSetDiagramForce}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.kit.settings.diagram.linkDistance"
            showLabel
            min={20}
            max={300}
            step={10}
            value={[diagramForce.linkDistance]}
            onValueChange={(value: number[]) => setDiagramForce?.({ linkDistance: value[0] })}
            disabled={!canSetDiagramForce}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.kit.settings.diagram.collideRadius"
            showLabel
            min={10}
            max={100}
            step={5}
            value={[diagramForce.collideRadius]}
            onValueChange={(value: number[]) => setDiagramForce?.({ collideRadius: value[0] })}
            disabled={!canSetDiagramForce}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.kit.settings.diagram.centerStrength"
            showLabel
            min={0}
            max={1}
            step={0.01}
            value={[diagramForce.centerStrength]}
            onValueChange={(value: number[]) => setDiagramForce?.({ centerStrength: value[0] })}
            disabled={!canSetDiagramForce}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const SketchpadSettingsContent: FC = () => {
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
            id="semio.sketchpad.app.kit.settings.theme"
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
          <Select id="semio.sketchpad.app.kit.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
            <SelectTrigger>
              <SelectValue placeholder={languagePlaceholder} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="en">{languageEnLabel}</SelectItem>
              <SelectItem value="de">{languageDeLabel}</SelectItem>
            </SelectContent>
          </Select>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.app.kit.settings.device"
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
            id="semio.sketchpad.app.kit.settings.expertise"
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
            id="semio.sketchpad.app.kit.settings.mode"
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

// #endregion Settings

// #endregion Right

// #endregion Panels

// #endregion Canvas

// #region Footer

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖footer](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/FOOTER)
// Footer MUST render the Kit app footer with selection count status.

/**
 * Footer component that renders the Kit app selection count status.
 *
 * MUST register and unregister footer items based on current selection state.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖footer🪨kitappfooter](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/FOOTER/KIT-APP-FOOTER)
 **/
export const KitAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();

  useEffect(() => {
    if (appType !== "kit") return;

    // TODO: Add kit-specific footer items here

    return () => {
      // Cleanup
    };
  }, [appType, addFooterItem, removeFooterItem]);

  return null;
};

// #endregion Footer

// #region Config

// [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖config](semiorepo://section/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CONFIG)
// Config MUST export the Kit app configuration with route segments, panel definitions, and path matching.

/**
 * Exported Kit app configuration including routes, panels, and path matching.
 *
 *  * [👤semio📚js🗃️sketchpad💻kittsx🔖internalstatemanagement🔖config🪨config](semiorepo://definition/SEMIO/JS/SKETCHPAD/KIT.TSX/INTERNAL-STATE-MANAGEMENT/CONFIG/CONFIG)
 **/
export const config: AppConfig = {
  id: "kit",
  component: MultiWindowApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
  ],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 2 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]);
  },
  order: 10,
};

// #endregion Config
