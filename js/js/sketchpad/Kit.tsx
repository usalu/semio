// #region Header

// App.tsx

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

// #region Imports

import { DragEndEvent, DragOverEvent, DragStartEvent, useDroppable } from "@dnd-kit/core";
import {
  AddIcon,
  AlertCircleIcon,
  AwardIcon,
  CheckIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  CodeIcon,
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
  InterfaceIcon,
  LayoutIcon,
  LightbulbIcon,
  MonitorIcon,
  MoonIcon,
  MousePointerIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  SunIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
} from "@semio/assets";
import { useSelector } from "@xstate/react";
import { forceCenter, forceCollide, forceLink, forceManyBody, forceSimulation, Simulation, SimulationLinkDatum, SimulationNodeDatum } from "d3-force";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import React, { FC, memo, useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { Camera } from "three";
import * as Y from "yjs";
import i18n, { useLabel } from "../i18n";
import { Author, buildFileTree, Concept, Coord, Design, DesignDiff, DiffStatus, flattenFileTree, Folder, generateUniqueName, guid, Guid, Interface, Kit, KitDiff, Quality, File as SemioFile, Tag, Type, TypeDiff } from "../semio";
import type { KitStore as KitDataSource, SketchpadStore as SketchpadOrchestrator } from "./Sketchpad";
import {
  AppWindowConfig,
  Canvas,
  createDefaultKitAppState,
  createKitDiagramForceSelector,
  createKitExpandedRowsSelector,
  createKitFilterSearchSelector,
  createKitFullscreenSelector,
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
  Window,
} from "./Sketchpad";
import type { ConnectionLineComponentProps, Edge, EdgeProps, Node, NodeProps } from "./elements";
import {
  Action,
  Background,
  BaseEdge,
  getBezierPath,
  Handle,
  Input,
  NotFound,
  Position,
  ReactFlow,
  ReactFlowProvider,
  Scrollable,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Table,
  TableAvatar,
  Textarea,
  Toggle,
  ToggleGroup,
  Transaction,
  TransactionProvider,
  TreeContent,
  TreeItem,
  useInternalNode,
  useReactFlow,
} from "./elements";
import type { Device, HookNoSetResult, HookResult, KitAppId, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "./shared";
import { AppConfig, AppPlugin, conditionalHookResult, createPanelDefinition, Expertise, Mode, PanelKind, parseWindowLayout, registerAppPlugin, registerRuntimeAction, stringifyWindowLayout, Theme } from "./shared";

// #endregion Imports

// #region Design Family Helpers

/**
 * Gets all design GUIDs in a design family as a Set for efficient lookup.
 * @param kit - The kit containing the designs.
 * @param designGuid - The GUID of any design in the family.
 * @returns Set of all design GUIDs in the family tree.
 */
const getDesignFamilyGuids = (kit: Kit, designGuid: string): Set<string> => {
  const guids = new Set<string>();

  // Find the primitive (root) design
  let currentGuid = designGuid;
  let current = kit.designs?.find((d) => d.guid === currentGuid);
  while (current?.parent?.guid) {
    const parent = kit.designs?.find((d) => d.guid === current!.parent!.guid);
    if (!parent) break;
    current = parent;
    currentGuid = parent.guid;
  }

  // Collect all descendants
  const collectDescendants = (parentGuid: string) => {
    guids.add(parentGuid);
    const children = (kit.designs || []).filter((d) => d.parent?.guid === parentGuid);
    children.forEach((child) => collectDescendants(child.guid));
  };
  collectDescendants(currentGuid);

  return guids;
};

// #endregion Design Family Helpers

// #region Internal State Management

type YKitAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitApp = Y.Map<YKitAppVal>;
type YKitApps = Y.Map<YKitApp>;

export interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  interfaces?: Guid[];
  tags?: Guid[];
  concepts?: Guid[];
  files?: string[];
  folders?: Guid[];
  authors?: string[];
}
const emptyKitAppSelection: KitAppSelection = {};
export interface KitAppSelectionTypesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionDesignsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionQualitiesDiff {
  added?: string[];
  removed?: string[];
}
export interface KitAppSelectionInterfacesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionTagsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionConceptsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionFilesDiff {
  added?: string[];
  removed?: string[];
}
export interface KitAppSelectionFoldersDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitAppSelectionAuthorsDiff {
  added?: string[];
  removed?: string[];
}
export interface KitAppSelectionDiff {
  types?: KitAppSelectionTypesDiff;
  designs?: KitAppSelectionDesignsDiff;
  qualities?: KitAppSelectionQualitiesDiff;
  interfaces?: KitAppSelectionInterfacesDiff;
  tags?: KitAppSelectionTagsDiff;
  concepts?: KitAppSelectionConceptsDiff;
  files?: KitAppSelectionFilesDiff;
  folders?: KitAppSelectionFoldersDiff;
  authors?: KitAppSelectionAuthorsDiff;
}
export enum KitAppWindowKind {
  Table = "table",
  Diagram = "diagram",
}
export interface KitAppPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface KitAppHover {
  type?: Guid;
  design?: Guid;
}
export interface KitAppPresenceOther extends KitAppPresence {
  name: string;
}
export type KitAppSortColumn = "artifact" | "kind" | "authors" | "updatedAt" | "createdAt";
export type KitAppSortDirection = "asc" | "desc";

export interface DiagramForceSettings {
  chargeStrength: number;
  linkDistance: number;
  collideRadius: number;
  centerStrength: number;
}

export const defaultDiagramForceSettings: DiagramForceSettings = {
  chargeStrength: -80,
  linkDistance: 60,
  collideRadius: 30,
  centerStrength: 0.15,
};

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
}
export interface KitAppEdit extends KitDiffAppEdit<KitAppSelectionDiff> {}
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
}

export interface KitAppCommandContext extends KitCommandContext {
  kitApp: KitAppState;
}
export interface KitAppCommandResult {
  diff?: KitAppDiff;
  kitDiff?: KitDiff;
}

export const inverseKitAppSelectionDiff = (selection: KitAppSelection, diff: KitAppSelectionDiff): KitAppSelectionDiff => {
  const inverseDiff: KitAppSelectionDiff = {};

  // Inverse types diff
  if (diff.types) {
    inverseDiff.types = {};
    if (diff.types.added) {
      inverseDiff.types.removed = diff.types.added;
    }
    if (diff.types.removed) {
      inverseDiff.types.added = diff.types.removed;
    }
  }

  // Inverse designs diff
  if (diff.designs) {
    inverseDiff.designs = {};
    if (diff.designs.added) {
      inverseDiff.designs.removed = diff.designs.added;
    }
    if (diff.designs.removed) {
      inverseDiff.designs.added = diff.designs.removed;
    }
  }

  // Inverse qualities diff
  if (diff.qualities) {
    inverseDiff.qualities = {};
    if (diff.qualities.added) {
      inverseDiff.qualities.removed = diff.qualities.added;
    }
    if (diff.qualities.removed) {
      inverseDiff.qualities.added = diff.qualities.removed;
    }
  }

  // Inverse files diff
  if (diff.files) {
    inverseDiff.files = {};
    if (diff.files.added) {
      inverseDiff.files.removed = diff.files.added;
    }
    if (diff.files.removed) {
      inverseDiff.files.added = diff.files.removed;
    }
  }

  // Inverse folders diff
  if (diff.folders) {
    inverseDiff.folders = {};
    if (diff.folders.added) {
      inverseDiff.folders.removed = diff.folders.added;
    }
    if (diff.folders.removed) {
      inverseDiff.folders.added = diff.folders.removed;
    }
  }

  // Inverse authors diff
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
export const areSameKitApp = (kitApp: KitAppId, other: KitAppId): boolean => kitApp.kit === other.kit;
export const hasSameKitApp = (kitApp: KitAppId, others: KitAppId[]): boolean => others.some((other) => areSameKitApp(kitApp, other));

class KitStore extends KitDiffStore<KitAppState, KitAppDiff, KitAppSelectionDiff, KitAppEdit, KitAppCommandContext, KitAppCommandResult> {
  constructor(parent: SketchpadOrchestrator, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    yMap.set("kit", kit.guid);

    yMap.set("fullscreenWindow", state?.fullscreenWindow || KitAppFullscreenWindow.None);

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

  // KitApp-specific getters
  get fullscreenWindow(): KitAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as KitAppFullscreenWindow;
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: false,
        workbench: false,
        details: false,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? false,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? false,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): KitAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: KitAppSelection = {};

    // Get types
    const types = selection.get("types") as Y.Array<string>;
    if (types && types.length > 0) {
      result.types = types.toArray();
    }

    // Get designs
    const designs = selection.get("designs") as Y.Array<string>;
    if (designs && designs.length > 0) {
      result.designs = designs.toArray();
    }

    // Get qualities
    const qualities = selection.get("qualities") as Y.Array<string>;
    if (qualities && qualities.length > 0) {
      result.qualities = qualities.toArray();
    }

    // Get files
    const files = selection.get("files") as Y.Array<string>;
    if (files && files.length > 0) {
      result.files = files.toArray();
    }

    // Get folders
    const folders = selection.get("folders") as Y.Array<string>;
    if (folders && folders.length > 0) {
      result.folders = folders.toArray();
    }

    // Get authors
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

  // Implement abstract methods from App base class
  protected getSelection(): KitAppSelection {
    return this.selection;
  }

  protected hash(state: KitAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): KitAppState {
    return {
      fullscreenWindow: this.fullscreenWindow,
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
    } as any;
  }

  change = (diff: KitAppDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow !== undefined) {
        this.yMap.set("fullscreenWindow", diff.fullscreenWindow);
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
        // Efficiently update only changed elements instead of replacing entire array
        const currentRows = yExpandedRows.toArray();
        const newRows = diff.expandedRows;
        const currentSet = new Set(currentRows);
        const newSet = new Set(newRows);

        // Find rows to remove (in current but not in new)
        const toRemove: number[] = [];
        currentRows.forEach((row, index) => {
          if (!newSet.has(row)) {
            toRemove.push(index);
          }
        });
        // Remove in reverse order to maintain indices
        for (let i = toRemove.length - 1; i >= 0; i--) {
          yExpandedRows.delete(toRemove[i], 1);
        }

        // Find rows to add (in new but not in current)
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

    // Apply types diff
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

    // Apply designs diff
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

    // Apply qualities diff
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

    // Apply files diff
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

    // Apply folders diff
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

    // Apply authors diff
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

    // Origins are strings like "semio.sketchpad.app.kit.panel.details.name" (starts with semio.sketchpad)
    // Commands are strings like "semio.kitApp.startTransaction" (starts with semio. but NOT semio.sketchpad)
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

/**
 * Kit app plugin for the sketchpad machine.
 * Provides KIT.* events, actions, and guards.
 */
const kitAppPlugin: AppPlugin = {
  id: "kit",
  namespace: "KIT",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: () => ({
      panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
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
  registerStores: () => {
    // Store factory already registered above
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(kitAppPlugin);
  registerRuntimeAction("kitInit", (context: any, event: any) => {
    if (event.type !== "KIT.INIT") return {};
    return { kitApps: { ...context.kitApps, [event.kitGuid]: event.state } };
  });
  registerRuntimeAction("kitSync", (context: any, event: any) => {
    if (event.type !== "KIT.SYNC") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, ...event.state } } };
  });
  registerRuntimeAction("kitTogglePanel", (context: any, event: any) => {
    if (event.type !== "KIT.TOGGLE_PANEL") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] } } } };
  });
  registerRuntimeAction("kitSetPanelVisibility", (context: any, event: any) => {
    if (event.type !== "KIT.SET_PANEL_VISIBILITY") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, panelVisibility: event.panelVisibility } } };
  });
  registerRuntimeAction("kitSetFilter", (context: any, event: any) => {
    if (event.type !== "KIT.SET_FILTER") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, filterSearch: event.search } } };
  });
  registerRuntimeAction("kitToggleRow", (context: any, event: any) => {
    if (event.type !== "KIT.TOGGLE_ROW") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const expanded = new Set(app.expandedRows);
    if (expanded.has(event.rowId)) expanded.delete(event.rowId);
    else expanded.add(event.rowId);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, expandedRows: expanded } } };
  });
  registerRuntimeAction("kitSetExpandedRows", (context: any, event: any) => {
    if (event.type !== "KIT.SET_EXPANDED_ROWS") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, expandedRows: event.expandedRows } } };
  });
  registerRuntimeAction("kitSetSort", (context: any, event: any) => {
    if (event.type !== "KIT.SET_SORT") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, sortColumn: event.column, sortDirection: event.direction } } };
  });
  registerRuntimeAction("kitSelectType", (context: any, event: any) => {
    if (event.type !== "KIT.SELECT_TYPE") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const types = [...(app.selection?.types || [])];
    if (!types.includes(event.typeGuid)) types.push(event.typeGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
  });
  registerRuntimeAction("kitDeselectType", (context: any, event: any) => {
    if (event.type !== "KIT.DESELECT_TYPE") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const types = (app.selection?.types || []).filter((t: Guid) => t !== event.typeGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, types } } } };
  });
  registerRuntimeAction("kitSelectDesign", (context: any, event: any) => {
    if (event.type !== "KIT.SELECT_DESIGN") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const designs = [...(app.selection?.designs || [])];
    if (!designs.includes(event.designGuid)) designs.push(event.designGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, designs } } } };
  });
  registerRuntimeAction("kitDeselectDesign", (context: any, event: any) => {
    if (event.type !== "KIT.DESELECT_DESIGN") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const designs = (app.selection?.designs || []).filter((d: Guid) => d !== event.designGuid);
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: { ...app.selection, designs } } } };
  });
  registerRuntimeAction("kitSetSelection", (context: any, event: any) => {
    if (event.type !== "KIT.SET_SELECTION") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: event.selection } } };
  });
  registerRuntimeAction("kitClearSelection", (context: any, event: any) => {
    if (event.type !== "KIT.CLEAR_SELECTION") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, selection: undefined } } };
  });
  registerRuntimeAction("kitSetHover", (context: any, event: any) => {
    if (event.type !== "KIT.SET_HOVER") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, hover: event.hover } } };
  });
  registerRuntimeAction("kitClearHover", (context: any, event: any) => {
    if (event.type !== "KIT.CLEAR_HOVER") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, hover: undefined } } };
  });
  registerRuntimeAction("kitTransactionStart", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.START") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const tx = app.transaction;
    if (tx.isTransactionActive) {
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    }
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...tx, isTransactionActive: true, currentTransactionStack: [], redoStack: [] } } } };
  });
  registerRuntimeAction("kitTransactionCommit", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.COMMIT") return {};
    const app = context.kitApps[event.kitGuid];
    if (!app || !app.transaction.isTransactionActive) return {};
    const tx = app.transaction;
    const pastStack = [...tx.pastTransactionStack];
    if (tx.currentTransactionStack.length > 0) {
      const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
      pastStack.push(merged);
    }
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
  });
  registerRuntimeAction("kitTransactionAbort", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.ABORT") return {};
    const app = context.kitApps[event.kitGuid];
    if (!app || !app.transaction.isTransactionActive) return {};
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...app.transaction, isTransactionActive: false, currentTransactionStack: [] } } } };
  });
  registerRuntimeAction("kitTransactionUndo", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.UNDO") return {};
    const app = context.kitApps[event.kitGuid];
    if (!app) return {};
    const tx = app.transaction;
    if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
      const currentStack = [...tx.currentTransactionStack];
      currentStack.pop();
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...tx, currentTransactionStack: currentStack } } } };
    } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
      const pastStack = [...tx.pastTransactionStack];
      const edit = pastStack.pop()!;
      const redoStack = [...tx.redoStack, edit];
      return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    }
    return {};
  });
  registerRuntimeAction("kitTransactionRedo", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.REDO") return {};
    const app = context.kitApps[event.kitGuid];
    if (!app || app.transaction.isTransactionActive || app.transaction.redoStack.length === 0) return {};
    const tx = app.transaction;
    const redoStack = [...tx.redoStack];
    const edit = redoStack.pop()!;
    const pastStack = [...tx.pastTransactionStack, edit];
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
  });
  registerRuntimeAction("kitTransactionRecordEdit", (context: any, event: any) => {
    if (event.type !== "KIT.TRANSACTION.RECORD_EDIT") return {};
    const app = context.kitApps[event.kitGuid];
    if (!app || !app.transaction.isTransactionActive) return {};
    const currentStack = [...app.transaction.currentTransactionStack, event.edit];
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, transaction: { ...app.transaction, currentTransactionStack: currentStack, redoStack: [] } } } };
  });
  registerRuntimeAction("kitSetWindowLayout", (context: any, event: any) => {
    if (event.type !== "KIT.SET_WINDOW_LAYOUT") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, windowLayout: event.windowLayout } } };
  });
  registerRuntimeAction("kitSetDiagramForce", (context: any, event: any) => {
    if (event.type !== "KIT.SET_DIAGRAM_FORCE") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    const currentForce = app.diagramForce || { ...defaultDiagramForceSettings };
    const newForce = { ...currentForce, ...event.diagramForce };
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, diagramForce: newForce } } };
  });
  registerRuntimeAction("kitSetFullscreen", (context: any, event: any) => {
    if (event.type !== "KIT.SET_FULLSCREEN") return {};
    const app = context.kitApps[event.kitGuid] || createDefaultKitAppState();
    return { kitApps: { ...context.kitApps, [event.kitGuid]: { ...app, fullscreenWindow: event.window } } };
  });
}

// #endregion Kit App Plugin Registration

function useKitStore<T>(selector?: (controller: KitStore) => T, id?: KitAppId): T | KitStore | null {
  const orchestrator = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) {
    return null;
  }
  try {
    if (!orchestrator || !orchestrator.hasKit(resolvedKitId)) {
      return null;
    }
    const kitStore = orchestrator.kitApp(resolvedKitId);
    const result = selector ? selector(kitStore) : kitStore;
    return result;
  } catch {
    return null;
  }
}

/**
 * Get Kit app state from XState.
 * This is the new XState-based hook.
 */
export function useKitApp<T>(selector?: (state: KitAppState) => T, id?: KitAppId): T | KitAppState {
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? id?.kit;

  // Create a default state when no kitGuid is available
  const defaultState: KitAppState = {
    panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
    selection: undefined,
    hover: undefined,
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

  // Convert expandedRows Set back to array for compatibility
  const state: KitAppState = {
    ...xstateState,
    expandedRows: xstateState.expandedRows ? Array.from(xstateState.expandedRows) : [],
  };

  if (selector) {
    return selector(state) as T;
  }
  return state;
}

export function useKitAppSelection(): HookResult<KitAppSelection> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSelectionSelector(kitGuid), [kitGuid]);
  const selection = useSelector(actor, selector) ?? emptyKitAppSelection;
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

export function useKitAppOthers(): HookNoSetResult<KitAppPresenceOther[]> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitOthersSelector(kitGuid), [kitGuid]);
  const others = useSelector(actor, selector) ?? [];
  const canRead = kitScope !== null;
  return [others, undefined, canRead];
}

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

export function useKitAppSortColumn(): HookNoSetResult<string> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSortColumnSelector(kitGuid), [kitGuid]);
  const sortColumn = useSelector(actor, selector) ?? "artifact";
  const canRead = kitScope !== null;
  return [sortColumn, undefined, canRead];
}

export function useKitAppSortDirection(): HookNoSetResult<"asc" | "desc"> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitSortDirectionSelector(kitGuid), [kitGuid]);
  const sortDirection = useSelector(actor, selector) ?? "asc";
  const canRead = kitScope !== null;
  return [sortDirection, undefined, canRead];
}

export function useKitAppExpandedRows(): HookNoSetResult<Set<string>> {
  const kitScope = useKitScope();
  const actor = useSketchpadActor();
  const kitGuid = kitScope?.guid ?? "";
  const selector = useMemo(() => createKitExpandedRowsSelector(kitGuid), [kitGuid]);
  const expandedRows = useSelector(actor, selector) ?? new Set<string>();
  const canRead = kitScope !== null;
  return [expandedRows, undefined, canRead];
}

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

export function useKitAppCommands(id?: KitAppId) {
  const controller = useKitStore(undefined, id) as KitStore | null;
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
      selectInterface: noOp,
      selectInterfaces: noOp,
      addInterfaceToSelection: noOp,
      removeInterfaceFromSelection: noOp,
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
    selectInterface: (guid: Guid) => controller.execute("semio.kitApp.selectInterface", getOrigin(), guid),
    selectInterfaces: (guids: Guid[]) => controller.execute("semio.kitApp.selectInterfaces", getOrigin(), guids),
    addInterfaceToSelection: (guid: Guid) => controller.execute("semio.kitApp.addInterfaceToSelection", getOrigin(), guid),
    removeInterfaceFromSelection: (guid: Guid) => controller.execute("semio.kitApp.removeInterfaceFromSelection", getOrigin(), guid),
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

export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

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

export function useKitAppSetHover(): ActionHookResult<[hover: { type?: Guid; design?: Guid }]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.SET_HOVER" as const, kitGuid, hover: {} }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (hover: { type?: Guid; design?: Guid }) => actor.send({ type: "KIT.SET_HOVER", kitGuid, hover });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

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

export function useKitAppTogglePanel(): ActionHookResult<[panel: keyof PanelVisibility]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const canActEvent = useMemo(() => ({ type: "KIT.TOGGLE_PANEL" as const, kitGuid, panel: "workbench" as keyof PanelVisibility }), [kitGuid]);
  const canAct = useSelector(actor, (snapshot) => snapshot.can(canActEvent));
  const action = useMemo(() => {
    if (!canAct) return undefined;
    return (panel: keyof PanelVisibility) => actor.send({ type: "KIT.TOGGLE_PANEL", kitGuid, panel });
  }, [actor, kitGuid, canAct]);
  return [action, canAct];
}

//#endregion Action Hooks

// #endregion Kit App

// #region Types

export function useKitAppIsTypeHovered(): HookNoSetResult<boolean> {
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid;
  const isHovered = useKitApp((state) => (typeGuid ? state.hover?.type === typeGuid : false)) as boolean;
  const canRead = typeScope !== null;
  return [isHovered ?? false, undefined, canRead];
}

export function useKitAppTypeStatus(): HookNoSetResult<DiffStatus> {
  const typeScope = useTypeScope();
  const canRead = typeScope !== null;
  return [DiffStatus.Unchanged, undefined, canRead];
}

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

export function useKitAppIsDesignHovered(): HookNoSetResult<boolean> {
  const designScope = useDesignScope();
  const designGuid = designScope?.guid;
  const isHovered = useKitApp((state) => (designGuid ? state.hover?.design === designGuid : false)) as boolean;
  const canRead = designScope !== null;
  return [isHovered ?? false, undefined, canRead];
}

export function useKitAppDesignStatus(): HookNoSetResult<DiffStatus> {
  const designScope = useDesignScope();
  const canRead = designScope !== null;
  return [DiffStatus.Unchanged, undefined, canRead];
}

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

// #endregion Internal State Management

// #region Commands

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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
  "semio.kitApp.selectInterface": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          interfaces: {
            removed: currentSelection?.interfaces ?? [],
            added: [guid],
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.selectInterfaces": (context: KitAppCommandContext, guids: Guid[]): KitAppCommandResult => {
    const currentSelection = context.kitApp.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
          qualities: { removed: currentSelection?.qualities ?? [] },
          interfaces: {
            removed: currentSelection?.interfaces ?? [],
            added: guids,
          },
          files: { removed: currentSelection?.files ?? [] },
          folders: { removed: currentSelection?.folders ?? [] },
          authors: { removed: currentSelection?.authors ?? [] },
        },
      },
    };
  },
  "semio.kitApp.addInterfaceToSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          interfaces: { added: [guid] },
        },
      },
    };
  },
  "semio.kitApp.removeInterfaceFromSelection": (context: KitAppCommandContext, guid: Guid): KitAppCommandResult => {
    return {
      diff: {
        selection: {
          interfaces: { removed: [guid] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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
          interfaces: { removed: currentSelection?.interfaces ?? [] },
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

// #region Navbar

// #endregion Navbar

// #region Canvas

// #region Windows

// #region Table

type ArtifactKind = "designs" | "types" | "qualities" | "interfaces" | "tags" | "concepts" | "files" | "folders" | "authors";

const KitToolbarFilters: FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const kit = useKit() as Kit | undefined;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();

  const selectedKind = searchParams.get("kind") as ArtifactKind | null;
  const defaultDesignName = useLabel("semio.sketchpad.app.kit.defaultDesignName");
  const defaultTypeName = useLabel("semio.sketchpad.app.kit.defaultTypeName");

  const toggleKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === kind) {
      newParams.delete("kind");
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("kind", kind);
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    if (!kit || !kitCommands) return;
    switch (kind) {
      case "designs": {
        const existingNames = (kit.designs || []).map((d: Design) => d.name);
        const uniqueName = generateUniqueName(defaultDesignName, existingNames);
        const newDesign: Design = { guid: guid(), name: uniqueName, pieces: [], connections: [] };
        kitCommands.createDesign(newDesign);
        sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
        break;
      }
      case "types": {
        const existingNames = (kit.types || []).map((t: Type) => t.name);
        const uniqueName = generateUniqueName(defaultTypeName, existingNames);
        const newType: Type = { guid: guid(), name: uniqueName, ports: [] };
        kitCommands.createType(newType);
        sketchpadCommands.navigateToType(kit.guid, newType.guid);
        break;
      }
      default:
        break;
    }
  };

  return (
    <div className="flex items-center gap-single">
      <Toggle
        kind="withAction"
        pressed={selectedKind === "designs"}
        onPressedChange={() => toggleKind("designs")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("designs")}
        id="semio.sketchpad.app.kit.toolbar.showDesigns"
        actionId="semio.sketchpad.app.kit.toolbar.createDesign"
        icon={<LayoutIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "types"}
        onPressedChange={() => toggleKind("types")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("types")}
        id="semio.sketchpad.app.kit.toolbar.showTypes"
        actionId="semio.sketchpad.app.kit.toolbar.createType"
        icon={<TypeIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "qualities"}
        onPressedChange={() => toggleKind("qualities")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("qualities")}
        id="semio.sketchpad.app.kit.toolbar.showQualities"
        actionId="semio.sketchpad.app.kit.toolbar.createQuality"
        icon={<AwardIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "interfaces"}
        onPressedChange={() => toggleKind("interfaces")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("interfaces")}
        id="semio.sketchpad.app.kit.toolbar.showInterfaces"
        actionId="semio.sketchpad.app.kit.toolbar.createInterface"
        icon={<InterfaceIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "tags"}
        onPressedChange={() => toggleKind("tags")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("tags")}
        id="semio.sketchpad.app.kit.toolbar.showTags"
        actionId="semio.sketchpad.app.kit.toolbar.createTag"
        icon={<HashIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "concepts"}
        onPressedChange={() => toggleKind("concepts")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("concepts")}
        id="semio.sketchpad.app.kit.toolbar.showConcepts"
        actionId="semio.sketchpad.app.kit.toolbar.createConcept"
        icon={<LightbulbIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "files"}
        onPressedChange={() => toggleKind("files")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("files")}
        id="semio.sketchpad.app.kit.toolbar.showFiles"
        actionId="semio.sketchpad.app.kit.toolbar.createFile"
        icon={<DocumentIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "folders"}
        onPressedChange={() => toggleKind("folders")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("folders")}
        id="semio.sketchpad.app.kit.toolbar.showFolders"
        actionId="semio.sketchpad.app.kit.toolbar.createFolder"
        icon={<FolderIcon />}
      />
      <Toggle
        kind="withAction"
        pressed={selectedKind === "authors"}
        onPressedChange={() => toggleKind("authors")}
        actionIcon={<AddIcon />}
        onActionClick={() => handleCreateArtifact("authors")}
        id="semio.sketchpad.app.kit.toolbar.showAuthors"
        actionId="semio.sketchpad.app.kit.toolbar.createAuthor"
        icon={<UserIcon />}
      />
    </div>
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
  data: Design | Type | Quality | Interface | Tag | Concept | SemioFile | Author | Folder;
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
    // Zip files are now treated as regular files
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
    // Zip files are now treated as regular files - no special handling
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

  // Use shallow subscription (deep=false) since this component primarily cares about
  // array-level changes (adding/removing items), not deep property changes within items
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

  const [selectTypeAction, canSelectType] = useKitAppSelectType();
  const [selectDesignAction, canSelectDesign] = useKitAppSelectDesign();
  const [setSelectionAction, canSetSelection] = useKitAppSetSelection();
  const [clearSelectionAction, canClearSelection] = useKitAppClearSelection();
  const [setFilterAction, canSetFilter] = useKitAppSetFilter();
  const [toggleRowAction] = useKitAppToggleRow();
  const [setSortAction, canSetSort] = useKitAppSetSort();
  const [toggleSortAction, canToggleSort] = useKitAppToggleSort();

  const [isDragOver, setIsDragOver] = React.useState(false);
  const [showZipWarning, setShowZipWarning] = React.useState(false);
  const [activeId, setActiveId] = React.useState<string | null>(null);
  const [overId, setOverId] = React.useState<string | null>(null);
  const lastClickedIndexRef = React.useRef<number>(-1);
  const clickTimerRef = React.useRef<NodeJS.Timeout | null>(null);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();

  // Get default names for artifact creation
  const defaultDesignName = useLabel("semio.sketchpad.app.design.defaultName");
  const defaultTypeName = useLabel("semio.sketchpad.app.type.defaultName");
  const defaultQualityName = useLabel("semio.sketchpad.app.quality.defaultName");
  const defaultFolderName = useLabel("semio.sketchpad.app.folder.defaultName");
  const defaultInterfaceName = useLabel("semio.sketchpad.app.interface.defaultName");
  const defaultTagName = useLabel("semio.sketchpad.app.tag.defaultName");
  const defaultConceptName = useLabel("semio.sketchpad.app.concept.defaultName");
  const kitLoadingLabel = useLabel("semio.sketchpad.app.kit.loading");

  // Pre-call all useLabel hooks used in JSX to avoid conditional hook calls
  const labelSearch = useLabel("semio.sketchpad.common.search");
  const labelArtifact = useLabel("semio.sketchpad.app.kit.canvas.table.header.artifact");
  const labelKind = useLabel("semio.sketchpad.app.kit.canvas.table.header.kind");
  const labelUpdatedAt = useLabel("semio.sketchpad.app.kit.canvas.table.header.updatedAt");
  const labelCreatedAt = useLabel("semio.sketchpad.app.kit.canvas.table.header.createdAt");

  // Get filters from search params (?kind=&name=) - MOVED BEFORE EARLY RETURNS
  const selectedKind = searchParams.get("kind") as ArtifactKind | null;
  const selectedName = searchParams.get("name");

  // Get concepts and search from search params
  const selectedConcepts = searchParams.getAll("c");
  const searchQuery = searchParams.get("q") || "";

  // Get selection parameter for auto-selecting designs/types
  const selectParam = searchParams.get("select");
  const expandedRows = expandedRowsSet;

  const selectionTypes = selection?.types || [];
  const selectionDesigns = selection?.designs || [];
  const selectionQualities = selection?.qualities || [];
  const selectionInterfaces = selection?.interfaces || [];
  const selectionTags = selection?.tags || [];
  const selectionConcepts = selection?.concepts || [];
  const selectionFiles = selection?.files || [];
  const selectionFolders = selection?.folders || [];
  const selectionAuthors = selection?.authors || [];
  const selectionTypesKey = selectionTypes.join(",");
  const selectionDesignsKey = selectionDesigns.join(",");
  const selectionQualitiesKey = selectionQualities.join(",");
  const selectionInterfacesKey = selectionInterfaces.join(",");
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
      interfaces: selectionInterfaces,
      tags: selectionTags,
      concepts: selectionConcepts,
      files: selectionFiles,
      folders: selectionFolders,
      authors: selectionAuthors,
    }),
    [selectionTypesKey, selectionDesignsKey, selectionQualitiesKey, selectionInterfacesKey, selectionTagsKey, selectionConceptsKey, selectionFilesKey, selectionFoldersKey, selectionAuthorsKey],
  );

  const kitDesigns = kit?.designs;
  const kitTypes = kit?.types;
  const kitQualities = kit?.qualities;
  const kitInterfaces = kit?.interfaces;
  const kitTags = kit?.tags;
  const kitConcepts = kit?.concepts;
  const kitFiles = kit?.files;
  const kitFolders = kit?.folders;
  const kitAuthors = kit?.authors;
  const kitDesignsKey = useMemo(() => kitDesigns?.map((d) => `${d.guid}:${d.name}:${d.parent?.guid || ""}:${d.folder || ""}:${d.updatedAt || ""}`).join("|") || "", [kitDesigns]);
  const kitTypesKey = useMemo(() => kitTypes?.map((t) => `${t.guid}:${t.name}:${t.parent?.guid || ""}:${t.folder || ""}:${t.updatedAt || ""}`).join("|") || "", [kitTypes]);
  const kitQualitiesKey = useMemo(() => kitQualities?.map((q) => `${q.guid}:${q.name}:${q.folder || ""}`).join("|") || "", [kitQualities]);
  const kitInterfacesKey = useMemo(() => kitInterfaces?.map((i) => `${i.guid}:${i.name}`).join("|") || "", [kitInterfaces]);
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

  // Collect unique names for the selected kind (or unified when no kind selected)
  // Names are shown hierarchically based on selectedName filter
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();

    // Helper to get visible names from a hierarchy
    const collectVisibleNames = <T extends { guid: string; name: string; parent?: { guid: string } }>(entities: T[] | undefined) => {
      if (!entities) return;

      if (!selectedName) {
        // No name selected - show all root entity names
        const rootEntities = entities.filter((e) => !e.parent);
        rootEntities.forEach((e) => nameSet.add(e.name));
      } else {
        // Name is selected - show children names of all entities with that name
        const matchingEntities = entities.filter((e) => e.name === selectedName);
        matchingEntities.forEach((parent) => {
          const children = entities.filter((e) => e.parent?.guid === parent.guid);
          children.forEach((child) => nameSet.add(child.name));
        });
      }
    };

    if (!selectedKind || selectedKind === "designs") {
      collectVisibleNames(kitDesigns);
    }
    if (!selectedKind || selectedKind === "types") {
      collectVisibleNames(kitTypes);
    }

    return Array.from(nameSet).sort();
  }, [kitDesignsKey, kitTypesKey, selectedKind, selectedName]);

  // Kit app creation is now handled by useKitAppYjsToXStateSync hook

  useEffect(() => {
    if (appType !== "kit") {
      return;
    }

    const typesCount = selection?.types?.length || 0;
    const designsCount = selection?.designs?.length || 0;
    const qualitiesCount = selection?.qualities?.length || 0;
    const interfacesCount = selection?.interfaces?.length || 0;
    const tagsCount = selection?.tags?.length || 0;
    const conceptsCount = selection?.concepts?.length || 0;
    const filesCount = selection?.files?.length || 0;
    const foldersCount = selection?.folders?.length || 0;
    const authorsCount = selection?.authors?.length || 0;
    const totalSelectedKinds = [typesCount > 0, designsCount > 0, qualitiesCount > 0, interfacesCount > 0, tagsCount > 0, conceptsCount > 0, filesCount > 0, foldersCount > 0, authorsCount > 0].filter(Boolean).length;

    const artifactsMultipleId = "semio.sketchpad.app.kit.artifacts.multiple";

    removeSection("details", artifactsMultipleId);
    removeSection("details", "semio.sketchpad.app.design.properties");
    removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
    removeSection("details", "semio.sketchpad.app.type.properties");
    removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.interface.properties");
    removeSection("details", "semio.sketchpad.app.kit.interfaces.multipleTitle");
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

    if (interfacesCount > 0 && totalSelectedKinds === 1) {
      const interfaceSectionId = interfacesCount === 1 ? "semio.sketchpad.app.kit.interface.properties" : "semio.sketchpad.app.kit.interfaces.multipleTitle";
      addSection("details", {
        id: interfaceSectionId,
        specificity: 30,
        order: 25,
        content: () =>
          kit ? (
            <React.Suspense fallback={null}>
              <KitScopeProvider guid={kit.guid}>
                <InterfaceSection />
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
      removeSection("details", "semio.sketchpad.app.kit.interface.properties");
      removeSection("details", "semio.sketchpad.app.kit.interfaces.multipleTitle");
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
    if (appType !== "kit") return;
    addSection("settings", {
      id: "semio.sketchpad.app.kit.settings",
      specificity: 10,
      order: 0,
      content: () => <KitEditorSettingsContent />,
    });
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: () => <SketchpadSettingsContent />,
    });
    return () => {
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [appType, addSection, removeSection]);

  // Auto-select design/type when select parameter is present
  useEffect(() => {
    if (!selectParam) return;

    if (selectedKind === "designs") {
      const design = kitDesigns?.find((d: Design) => d.guid === selectParam);
      if (design && selectDesignAction) {
        selectDesignAction(selectParam);
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKind === "types") {
      const type = kitTypes?.find((t: Type) => t.guid === selectParam);
      if (type && selectTypeAction) {
        selectTypeAction(selectParam);
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    }
  }, [selectParam, selectedKind, kitDesigns, kitTypes, selectDesignAction, selectTypeAction, searchParams, setSearchParams]);

  const allRows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date | string) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };

    // Pre-build lookup maps for O(1) child lookups instead of O(n) filter operations
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

    if (!selectedKind || selectedKind === "designs") {
      const designGroups = new Map<string, Design[]>();
      kitDesigns?.forEach((design: Design) => {
        const key = design.name;
        if (!designGroups.has(key)) designGroups.set(key, []);
        designGroups.get(key)!.push(design);
      });

      // Helper function to recursively build design hierarchy
      const buildDesignHierarchy = (designs: Design[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childDesigns = designsByParent.get(parentGuid) || [];

        childDesigns.forEach((design) => {
          // Skip designs not in the input set (for filtered views)
          if (!designs.includes(design)) return;
          if (selectedConcepts.length > 0 && !design.concepts?.some((c) => selectedConcepts.includes(c.guid))) return;
          if (searchQuery && !design.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
          // Skip root designs that are in folders when not viewing the folders kind
          // Only filter at root level (parentGuid === undefined), not children
          if (!selectedKind && parentGuid === undefined && design.folder) return;

          const rowId = `design-${design.guid}`;
          const children = designsByParent.get(design.guid) || [];
          const hasChildren = children.length > 0;

          // Resolve concept GUIDs to names
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

      // Apply name filter - if selectedName is set, only include designs with that name and their descendants
      const allDesignsArray = kitDesigns || [];
      if (selectedName) {
        // Find all designs with the selected name
        const matchingDesignGuids = new Set(allDesignsArray.filter((d) => d.name === selectedName).map((d) => d.guid));

        // Collect all descendants of matching designs
        const includeGuids = new Set(matchingDesignGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = designsByParent.get(parentGuid) || [];
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingDesignGuids.forEach((guid) => collectDescendants(guid));

        // Filter to only included designs
        const filteredDesigns = allDesignsArray.filter((d) => includeGuids.has(d.guid));

        // Build hierarchy starting from matching designs (as roots)
        buildDesignHierarchy(filteredDesigns, undefined, 0);
      } else {
        // No name filter - start with root designs (no parent)
        buildDesignHierarchy(allDesignsArray, undefined, 0);
      }
    }

    if (!selectedKind || selectedKind === "types") {
      // Helper function to recursively build type hierarchy
      const buildTypeHierarchy = (types: Type[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childTypes = typesByParent.get(parentGuid) || [];

        childTypes.forEach((type) => {
          // Skip types not in the input set (for filtered views)
          if (!types.includes(type)) return;
          if (searchQuery && !type.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
          // Skip root types that are in folders when not viewing the folders kind
          // Only filter at root level (parentGuid === undefined), not children
          if (!selectedKind && parentGuid === undefined && type.folder) return;

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

      // Apply name filter - if selectedName is set, only include types with that name and their descendants
      const allTypesArray = kitTypes || [];
      if (selectedName) {
        // Find all types with the selected name
        const matchingTypeGuids = new Set(allTypesArray.filter((t) => t.name === selectedName).map((t) => t.guid));

        // Collect all descendants of matching types
        const includeGuids = new Set(matchingTypeGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = typesByParent.get(parentGuid) || [];
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingTypeGuids.forEach((guid) => collectDescendants(guid));

        // Filter to only included types
        const filteredTypes = allTypesArray.filter((t) => includeGuids.has(t.guid));

        // Build hierarchy starting from matching types (as roots)
        buildTypeHierarchy(filteredTypes, undefined, 0);
      } else {
        // No name filter - start with root types (no parent)
        buildTypeHierarchy(allTypesArray, undefined, 0);
      }
    }

    if (!selectedKind || selectedKind === "qualities") {
      kitQualities?.forEach((quality: Quality) => {
        if (searchQuery && !quality.name.toLowerCase().includes(searchQuery.toLowerCase()) && !quality.key.toLowerCase().includes(searchQuery.toLowerCase())) return;
        // Skip qualities that are in folders when not viewing the folders kind
        if (!selectedKind && quality.folder) return;
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

    if (!selectedKind || selectedKind === "interfaces") {
      kitInterfaces?.forEach((iface: Interface) => {
        if (searchQuery && !iface.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `interface-${iface.guid}`,
          kind: "interfaces",
          artifact: iface.name,
          authors: iface.compatibleInterfaces?.length ? `${iface.compatibleInterfaces.length} compatible` : "All compatible",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: iface,
        });
      });
    }

    if (!selectedKind || selectedKind === "tags") {
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

    if (!selectedKind || selectedKind === "concepts") {
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

    if (selectedKind === "files") {
      // Build file tree from files - only when specifically viewing files kind
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

    if (!selectedKind || selectedKind === "folders") {
      // Helper function to recursively build folder hierarchy
      const buildFolderHierarchy = (parentFolder: Folder | null, level: number, parentRowId?: string): void => {
        const parentGuid = parentFolder?.guid;
        const childFolders = foldersByParent.get(parentGuid) || [];

        childFolders.forEach((folder: Folder) => {
          if (searchQuery && !folder.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

          // Get artifacts in this folder using pre-built lookup maps
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

          // Add child artifacts (always build, visibility computed later)
          if (folderedArtifacts > 0) {
            // Add designs in folder with their full hierarchy
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

              // Recursively add design children (always build, visibility computed later)
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

            // Add types in folder with their full hierarchy
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

              // Recursively add type children (always build, visibility computed later)
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

            // Add qualities in folder
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

            // Add files in folder
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

            // Recursively add child folders
            buildFolderHierarchy(folder, level + 1, folderId);
          }
        });
      };

      // Start with root folders (no parent)
      buildFolderHierarchy(null, 0);
    }

    if (!selectedKind || selectedKind === "authors") {
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
    kitInterfaces,
    kitTags,
    kitConcepts,
    kitFiles,
    kitFolders,
    kitAuthors,
    kitDesignsKey,
    kitTypesKey,
    kitQualitiesKey,
    kitInterfacesKey,
    kitTagsKey,
    kitConceptsKey,
    kitFilesKey,
    kitFoldersKey,
    kitAuthorsKey,
    selectedKind,
    selectedName,
    selectedConcepts,
    searchQuery,
    sortColumn,
    sortDirection,
  ]);

  // Compute visible rows: filter allRows based on expandedRows (fast O(n) operation)
  const rows = useMemo<TableRow[]>(() => {
    // Build a Set of all visible row IDs:
    // A row is visible if all its ancestors are expanded
    const visibleRowIds = new Set<string>();
    const rowById = new Map<string, TableRow>();

    // First pass: index all rows by ID
    allRows.forEach((row) => rowById.set(row.id, row));

    // Second pass: determine visibility by checking ancestor chain
    allRows.forEach((row) => {
      let isVisible = true;
      let currentRow = row;

      // Walk up the parent chain to check if all ancestors are expanded
      while (currentRow.parentId) {
        const parent = rowById.get(currentRow.parentId);
        if (!parent) {
          // Parent not found, row is not visible
          isVisible = false;
          break;
        }
        if (!expandedRows.has(parent.id)) {
          // Parent is collapsed, row is not visible
          isVisible = false;
          break;
        }
        currentRow = parent;
      }

      if (isVisible) {
        visibleRowIds.add(row.id);
      }
    });

    // Filter to visible rows and set isExpanded
    const result = allRows
      .filter((row) => visibleRowIds.has(row.id))
      .map((row) => ({
        ...row,
        isExpanded: expandedRows.has(row.id),
      }));

    return result;
  }, [allRows, expandedRows]);

  // Compute selected row IDs for the Table component
  const selectedRows = useMemo(() => {
    const selectedSet = new Set<string>();
    rows.forEach((row) => {
      let isSelected = false;
      if (row.kind === "designs") isSelected = selection.designs?.includes((row.data as Design).guid) ?? false;
      else if (row.kind === "types") isSelected = selection.types?.includes((row.data as Type).guid) ?? false;
      else if (row.kind === "qualities") isSelected = selection.qualities?.includes((row.data as Quality).key) ?? false;
      else if (row.kind === "interfaces") isSelected = selection.interfaces?.includes((row.data as Interface).guid) ?? false;
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

    // Prevent dropping a folder onto itself
    if (draggedRow.kind === "folders" && over && over.id === active.id) {
      return;
    }

    let targetFolderId: string | undefined = undefined;
    let targetParentId: string | undefined = undefined;
    let shouldExpandFolder = false;
    let shouldExpandParent = false;

    if (over) {
      // Check if dropped on canvas root (empty space in the table)
      if (over.id === "canvas-root") {
        // Dropped on canvas background - move to root (unset folder/parent)
        targetFolderId = undefined;
        targetParentId = undefined;
      } else {
        // Dropped on a row
        const targetRow = rows.find((r) => r.id === over.id);
        if (targetRow) {
          if (targetRow.kind === "folders") {
            // Dropped directly on a folder
            const folder = targetRow.data as Folder;
            targetFolderId = folder.guid;
            shouldExpandFolder = true;
          } else if (targetRow.kind === "designs" && draggedRow.kind === "designs") {
            // Dropped design onto another design - set as parent
            const targetDesign = targetRow.data as Design;
            targetParentId = targetDesign.guid;
            shouldExpandParent = true;
          } else if (targetRow.kind === "types" && draggedRow.kind === "types") {
            // Dropped type onto another type - set as parent
            const targetType = targetRow.data as Type;
            targetParentId = targetType.guid;
            shouldExpandParent = true;
          } else if (targetRow.folderId) {
            // Dropped on a non-folder child of a folder - move to parent folder
            targetFolderId = targetRow.folderId;
          } else {
            // Dropped on root-level row that's not a folder/same-kind - move to root (unset folder/parent)
            targetFolderId = undefined;
            targetParentId = undefined;
          }
        } else {
          // No target row found - move to root
          targetFolderId = undefined;
          targetParentId = undefined;
        }
      }
    } else {
      // Dropped outside all droppable areas - move to root (unset folder/parent)
      targetFolderId = undefined;
      targetParentId = undefined;
    }

    // Don't move if already in the target location
    // For designs and types, check the actual folder property from the data
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

    // If dropped on root and item has parent, allow (to unparent)
    // If dropped on root and item has no parent and no folder, skip (already at root)
    // Otherwise check if target is same as current location
    if (targetFolderId === undefined && targetParentId === undefined && !hasParent && !currentFolderId) {
      return;
    }
    if (targetFolderId !== undefined && currentFolderId === targetFolderId) {
      return;
    }

    if (draggedRow.kind === "designs" && kitCommands) {
      const design = draggedRow.data as Design;

      // Handle parent reassignment
      if (targetParentId !== undefined) {
        // Dropped onto another design - set as parent
        if (design.parent?.guid !== targetParentId) {
          // Check if reparenting would violate the same-family constraint for design pieces
          // A design cannot have design pieces that reference designs in the same family
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
        // Dropped on root - unparent and remove from folder
        kitCommands.updateDesign(design.guid, { parent: undefined });
        if (design.folder) {
          kitCommands.moveToFolder("design", design.guid, null);
        }
      } else if (!design.parent) {
        // Root design (protodesign) - can be moved to folders or root
        kitCommands.moveToFolder("design", design.guid, targetFolderId ?? null);
      }
    } else if (draggedRow.kind === "types" && kitCommands) {
      const type = draggedRow.data as Type;

      // Handle parent reassignment
      if (targetParentId !== undefined) {
        // Dropped onto another type - set as parent
        if (type.parent?.guid !== targetParentId) {
          kitCommands.updateType(type.guid, { parent: { guid: targetParentId } });
        }
      } else if (targetFolderId === undefined && (type.parent || type.folder)) {
        // Dropped on root - unparent and remove from folder
        kitCommands.updateType(type.guid, { parent: undefined });
        if (type.folder) {
          kitCommands.moveToFolder("type", type.guid, null);
        }
      } else if (!type.parent) {
        // Root type (prototype) - can be moved to folders or root
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

    // Expand the target folder if moving into a folder
    if (shouldExpandFolder && targetFolderId) {
      const folderId = `folder-${targetFolderId}`;
      if (!expandedRows.has(folderId)) {
        toggleRow(folderId);
      }
    }

    // Expand the target parent if setting a parent
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
        const uniqueName = generateUniqueName(defaultDesignName, existingNames);
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
        const uniqueName = generateUniqueName(defaultTypeName, existingNames);
        const newType: Type = {
          guid: guid(),
          name: uniqueName,
          ports: [],
        };
        if (kitCommands) kitCommands.createType(newType);
        sketchpadCommands.navigateToType(kit.guid, newType.guid);
        break;
      }
      case "qualities": {
        const existingNames = (kit.qualities || []).map((q: Quality) => q.name || "");
        const uniqueName = generateUniqueName(defaultQualityName, existingNames);
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
      case "interfaces": {
        const existingNames = (kit.interfaces || []).map((i: Interface) => i.name);
        const uniqueName = generateUniqueName(defaultInterfaceName, existingNames);
        const newInterface: Interface = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createInterface(newInterface);
        setKind("interfaces");
        setSelectionAction?.({ interfaces: [newInterface.guid] });
        break;
      }
      case "tags": {
        const existingNames = (kit.tags || []).map((t: Tag) => t.name);
        const uniqueName = generateUniqueName(defaultTagName, existingNames);
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
        const uniqueName = generateUniqueName(defaultConceptName, existingNames);
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
        const uniqueName = generateUniqueName(defaultFolderName, existingNames);
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
        ports: [],
      };
      if (kitCommands) kitCommands.createType(newType);
      sketchpadCommands.navigateToType(kit.guid, newType.guid);
    }
  };

  const toggleKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === kind) {
      newParams.delete("kind");
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("kind", kind);
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

  const handleRowClick = (row: TableRow, index: number, e: React.MouseEvent) => {
    // Clear any pending click timer to prevent double execution on double-click
    if (clickTimerRef.current) {
      clearTimeout(clickTimerRef.current);
      clickTimerRef.current = null;
    }

    // Handle shift-click range selection (cross-kind)
    if (e.shiftKey && lastClickedIndexRef.current !== -1) {
      const start = Math.min(lastClickedIndexRef.current, index);
      const end = Math.max(lastClickedIndexRef.current, index);
      const rangeRows = rows.slice(start, end + 1);

      // Group selected items by kind
      const selectedByKind: {
        types: Guid[];
        designs: Guid[];
        qualities: string[];
        interfaces: Guid[];
        tags: Guid[];
        concepts: Guid[];
        files: string[];
        folders: Guid[];
        authors: string[];
      } = {
        types: [],
        designs: [],
        qualities: [],
        interfaces: [],
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
        else if (r.kind === "interfaces") selectedByKind.interfaces.push((r.data as Interface).guid);
        else if (r.kind === "tags") selectedByKind.tags.push((r.data as Tag).guid);
        else if (r.kind === "concepts") selectedByKind.concepts.push((r.data as Concept).guid);
        else if (r.kind === "files") selectedByKind.files.push((r.data as SemioFile).guid);
        else if (r.kind === "folders") selectedByKind.folders.push((r.data as Folder).guid);
        else if (r.kind === "authors") selectedByKind.authors.push((r.data as Author).name);
      });

      // Select all items at once via XState
      setSelectionAction?.(selectedByKind);

      // Don't update lastClickedIndexRef for shift-clicks - keep the anchor stable
      return;
    }

    // Handle ctrl/cmd multi-select
    if (e.metaKey || e.ctrlKey) {
      if (row.kind === "designs") {
        const designId = (row.data as Design).guid;
        if (selection.designs?.includes(designId)) {
          setSelectionAction?.({ ...selection, designs: selection.designs.filter((d) => d !== designId) });
        } else {
          setSelectionAction?.({ ...selection, designs: [...(selection.designs || []), designId] });
        }
      } else if (row.kind === "types") {
        const typeId = (row.data as Type).guid;
        if (selection.types?.includes(typeId)) {
          setSelectionAction?.({ ...selection, types: selection.types.filter((t) => t !== typeId) });
        } else {
          setSelectionAction?.({ ...selection, types: [...(selection.types || []), typeId] });
        }
      } else if (row.kind === "qualities") {
        const qualityKey = (row.data as Quality).key;
        if (selection.qualities?.includes(qualityKey)) {
          setSelectionAction?.({ ...selection, qualities: selection.qualities.filter((q) => q !== qualityKey) });
        } else {
          setSelectionAction?.({ ...selection, qualities: [...(selection.qualities || []), qualityKey] });
        }
      } else if (row.kind === "interfaces") {
        const interfaceId = (row.data as Interface).guid;
        if (selection.interfaces?.includes(interfaceId)) {
          setSelectionAction?.({ ...selection, interfaces: selection.interfaces.filter((i) => i !== interfaceId) });
        } else {
          setSelectionAction?.({ ...selection, interfaces: [...(selection.interfaces || []), interfaceId] });
        }
      } else if (row.kind === "tags") {
        const tagId = (row.data as Tag).guid;
        if (selection.tags?.includes(tagId)) {
          setSelectionAction?.({ ...selection, tags: selection.tags.filter((t) => t !== tagId) });
        } else {
          setSelectionAction?.({ ...selection, tags: [...(selection.tags || []), tagId] });
        }
      } else if (row.kind === "concepts") {
        const conceptId = (row.data as Concept).guid;
        if (selection.concepts?.includes(conceptId)) {
          setSelectionAction?.({ ...selection, concepts: selection.concepts.filter((c) => c !== conceptId) });
        } else {
          setSelectionAction?.({ ...selection, concepts: [...(selection.concepts || []), conceptId] });
        }
      } else if (row.kind === "files") {
        const fileGuid = (row.data as SemioFile).guid;
        if (selection.files?.includes(fileGuid)) {
          setSelectionAction?.({ ...selection, files: selection.files.filter((f) => f !== fileGuid) });
        } else {
          setSelectionAction?.({ ...selection, files: [...(selection.files || []), fileGuid] });
        }
      } else if (row.kind === "folders") {
        const folderId = (row.data as Folder).guid;
        if (selection.folders?.includes(folderId)) {
          setSelectionAction?.({ ...selection, folders: selection.folders.filter((f) => f !== folderId) });
        } else {
          setSelectionAction?.({ ...selection, folders: [...(selection.folders || []), folderId] });
        }
      } else if (row.kind === "authors") {
        const authorName = (row.data as Author).name;
        if (selection.authors?.includes(authorName)) {
          setSelectionAction?.({ ...selection, authors: selection.authors.filter((a) => a !== authorName) });
        } else {
          setSelectionAction?.({ ...selection, authors: [...(selection.authors || []), authorName] });
        }
      }
      // Don't update lastClickedIndexRef for ctrl/cmd clicks
      return;
    }

    // Handle normal single selection with delay to detect double-click
    clickTimerRef.current = setTimeout(() => {
      if (row.kind === "designs") {
        setSelectionAction?.({ designs: [(row.data as Design).guid] });
      } else if (row.kind === "types") {
        setSelectionAction?.({ types: [(row.data as Type).guid] });
      } else if (row.kind === "qualities") {
        setSelectionAction?.({ qualities: [(row.data as Quality).key] });
      } else if (row.kind === "interfaces") {
        setSelectionAction?.({ interfaces: [(row.data as Interface).guid] });
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

    // Update last clicked index for shift-selection
    lastClickedIndexRef.current = index;
  };

  const handleRowDoubleClick = (row: TableRow, index: number) => {
    // Clear the click timer to prevent single-click selection
    if (clickTimerRef.current) {
      clearTimeout(clickTimerRef.current);
      clickTimerRef.current = null;
    }

    if (row.kind === "designs") {
      sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
    } else if (row.kind === "types") {
      sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
    } else if (row.kind === "qualities") {
      sketchpadCommands.navigateToQuality(kit.guid, (row.data as Quality).key);
    }
  };

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

    // Check if any file is a zip file
    const hasZipFile = files.some((file) => file.name.toLowerCase().endsWith(".zip"));
    if (hasZipFile) {
      setShowZipWarning(true);
      // Auto-dismiss warning after 8 seconds
      setTimeout(() => setShowZipWarning(false), 8000);
    }

    for (const file of files) {
      // Treat all files (including zip files) as regular files
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

  // Early return checks after all hooks have been called
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
                <div className="flex items-center gap-single w-full">
                  <div className="flex items-center gap-single flex-1 min-w-0" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                    {row.hasChildren ? (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleRow(row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span className="size-small shrink-0" />
                    )}
                    <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
                    <span className="text-left min-w-0 truncate">{row.artifact}</span>
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
          data={rows}
          getRowId={(row) => row.id}
          selectedRows={selectedRows}
          onRowClick={handleRowClick}
          onRowDoubleClick={handleRowDoubleClick}
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
            ...(!selectedKind
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
                        {row.kind === "interfaces" && <InterfaceIcon />}
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
                <div className="flex items-center gap-single w-full">
                  <div className="flex items-center gap-single flex-1 min-w-0" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                    {row.hasChildren ? (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleRow(row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span className="size-small shrink-0" />
                    )}
                    <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
                    <span className="text-left min-w-0 truncate">{row.artifact}</span>
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
          data={rows}
          getRowId={(row) => row.id}
          selectedRows={selectedRows}
          onRowClick={handleRowClick}
          onRowDoubleClick={handleRowDoubleClick}
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

// Sync hook to keep Y.js controller state in sync with XState
function useKitAppYjsToXStateSync() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const sketchpadStore = useSketchpadStore();
  const sketchpadCommands = useSketchpadCommands();
  const hasKit = useHasKit(kitGuid);
  const initializedKeyRef = useRef<string | null>(null);

  // Ensure Kit app exists before syncing
  useLayoutEffect(() => {
    if (!kitGuid || !hasKit) return;

    if (!sketchpadStore.hasKitApp({ kit: kitGuid })) {
      sketchpadCommands.createKitApp("semio.sketchpad.app.kit.autoCreateForSync", { kit: kitGuid });
    }
  }, [kitGuid, hasKit, sketchpadStore, sketchpadCommands]);

  // Initialize XState with Y.js state synchronously (before paint)
  // Note: hasKit is included in deps so this effect runs when hasKit changes from false to true
  // (which is when the first effect creates the kit app)
  useLayoutEffect(() => {
    if (!kitGuid || !hasKit) return;
    const initKey = kitGuid;
    if (initializedKeyRef.current === initKey) return;

    let xstateInitialState;
    if (sketchpadStore.hasKitApp({ kit: kitGuid })) {
      const store = sketchpadStore.kitApp(kitGuid);
      const initialState = store.snapshot();
      // Convert expandedRows array to Set for XState and add transaction state
      xstateInitialState = {
        ...initialState,
        expandedRows: new Set(initialState.expandedRows || []),
        transaction: {
          isTransactionActive: false,
          currentTransactionStack: [],
          pastTransactionStack: [],
          redoStack: [],
        },
      };
    } else {
      // Use default state when Y.js store isn't ready yet
      // This ensures the machine transitions to 'kit' state for panel toggles
      xstateInitialState = {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
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

    // Initialize XState with current Y.js state (or default)
    actor.send({
      type: "KIT.INIT",
      kitGuid,
      state: xstateInitialState,
    });
    initializedKeyRef.current = initKey;
  }, [actor, sketchpadStore, kitGuid, hasKit]);

  // Continue syncing Y.js changes to XState
  const store = kitGuid && sketchpadStore.hasKitApp({ kit: kitGuid }) ? sketchpadStore.kitApp(kitGuid) : null;
  const state = useSyncDeep<KitAppState, KitAppState>(store, (s: KitAppState) => s);

  useEffect(() => {
    if (!state || !kitGuid || initializedKeyRef.current !== kitGuid) return;

    // Convert expandedRows array to Set for XState
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
  // Sync Y.js state to XState
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

type DiagramNodeKind = "type" | "design" | "quality" | "interface" | "tag" | "concept" | "file" | "folder" | "author";

interface KitDiagramNode extends Record<string, unknown> {
  guid: string;
  name: string;
  kind: DiagramNodeKind;
  icon?: string;
  parentGuid?: string;
  concepts?: string[];
}

interface KitDiagramEdge {
  id: string;
  source: string;
  target: string;
  relationship: "part-of" | "reference";
}

const KitArtifactNode: FC<NodeProps<Node<KitDiagramNode>>> = ({ data, selected }) => {
  const [selection] = useKitAppSelection();
  const hover = useKitApp((state) => state?.hover) as KitAppHover | undefined;

  const isHovered = useMemo(() => {
    if (!hover) return false;
    if (data.kind === "type") return hover.type === data.guid;
    if (data.kind === "design") return hover.design === data.guid;
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
      case "interface":
        return selection.interfaces?.includes(data.guid) ?? false;
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
      className={`flex items-center justify-center cursor-grab active:cursor-grabbing transition-colors w-[220px] h-[140px] pointer-events-none ${isSelected ? "ring-2 ring-active-base" : isHovered ? "ring-2 ring-hover-base" : ""}`}
      title={data.name || data.guid.substring(0, 8)}
    >
      <Handle type="target" position={Position.Top} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Bottom} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="target" position={Position.Left} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <Handle type="source" position={Position.Right} className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" />
      <TableAvatar name={data.name} icon={data.icon} className={isSelected ? "bg-active-base" : isHovered ? "bg-hover-base" : ""} />
    </div>
  );
};

const kitNodeTypes = {
  artifact: KitArtifactNode,
};

const edgeStyle = {
  "part-of": { stroke: "var(--foreground)", strokeWidth: 2 },
  reference: { stroke: "var(--foreground)", strokeWidth: 1, strokeDasharray: "5,5" },
};

// Floating edges utility functions
// Nodes are circular with size-small (40px), so radius is 20px
const NODE_RADIUS = 20;

function getNodeIntersection(intersectionNode: Node, targetNode: Node): { x: number; y: number } {
  const x = intersectionNode.position.x;
  const y = intersectionNode.position.y;

  const dx = targetNode.position.x - x;
  const dy = targetNode.position.y - y;
  const distance = Math.sqrt(dx * dx + dy * dy);

  if (distance === 0) {
    return { x, y };
  }

  // Calculate intersection point on circle boundary
  const ratio = NODE_RADIUS / distance;
  return {
    x: x + dx * ratio,
    y: y + dy * ratio,
  };
}

function getEdgePosition(node: Node, intersectionPoint: { x: number; y: number }): Position {
  const dx = intersectionPoint.x - node.position.x;
  const dy = intersectionPoint.y - node.position.y;

  // For circular nodes, determine position based on angle
  const angle = Math.atan2(dy, dx);
  const absAngle = Math.abs(angle);

  if (absAngle < Math.PI / 4 || absAngle > (3 * Math.PI) / 4) {
    return dx > 0 ? Position.Right : Position.Left;
  }
  return dy > 0 ? Position.Bottom : Position.Top;
}

function getEdgeParams(source: Node, target: Node) {
  const sourceIntersection = getNodeIntersection(source, target);
  const targetIntersection = getNodeIntersection(target, source);

  const sourcePos = getEdgePosition(source, sourceIntersection);
  const targetPos = getEdgePosition(target, targetIntersection);

  return {
    sx: sourceIntersection.x,
    sy: sourceIntersection.y,
    tx: targetIntersection.x,
    ty: targetIntersection.y,
    sourcePos,
    targetPos,
  };
}

// FloatingEdge component
const FloatingEdge: FC<EdgeProps> = ({ id, source, target, markerEnd, style }) => {
  const sourceNode = useInternalNode(source);
  const targetNode = useInternalNode(target);

  if (!sourceNode || !targetNode) {
    return null;
  }

  // Convert internal node format to position format
  // positionAbsolute is top-left corner, add NODE_RADIUS to get center
  const sourcePos = sourceNode.internals?.positionAbsolute ?? { x: 0, y: 0 };
  const targetPos = targetNode.internals?.positionAbsolute ?? { x: 0, y: 0 };

  const sourceNodeForCalc = { position: { x: sourcePos.x + NODE_RADIUS, y: sourcePos.y + NODE_RADIUS } } as Node;
  const targetNodeForCalc = { position: { x: targetPos.x + NODE_RADIUS, y: targetPos.y + NODE_RADIUS } } as Node;

  const { sx, sy, tx, ty, sourcePos: sPos, targetPos: tPos } = getEdgeParams(sourceNodeForCalc, targetNodeForCalc);

  const [edgePath] = getBezierPath({
    sourceX: sx,
    sourceY: sy,
    sourcePosition: sPos,
    targetX: tx,
    targetY: ty,
    targetPosition: tPos,
  });

  return <BaseEdge id={id} path={edgePath} markerEnd={markerEnd} style={style} />;
};

// FloatingConnectionLine component
const FloatingConnectionLine: FC<ConnectionLineComponentProps> = ({ fromX, fromY, toX, toY }) => {
  const edgePath = getBezierPath({
    sourceX: fromX,
    sourceY: fromY,
    targetX: toX,
    targetY: toY,
    sourcePosition: Position.Top,
    targetPosition: Position.Top,
  })[0];

  return <BaseEdge path={edgePath} style={{ stroke: "var(--foreground)", strokeWidth: 2 }} />;
};

const buildKitDiagramData = (kit: Kit): { nodes: Node<KitDiagramNode>[]; edges: Edge[] } => {
  const nodes: Node<KitDiagramNode>[] = [];
  const edges: Edge[] = [];

  const kindGroups: DiagramNodeKind[] = ["type", "design", "quality", "interface", "tag", "concept", "file", "folder", "author"];

  for (const kind of kindGroups) {
    let items: Array<{ guid: string; name: string; icon?: string; parentGuid?: string; concepts?: string[] }> = [];

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
      case "interface":
        items = (kit.interfaces ?? []).map((i) => ({ guid: i.guid, name: i.name, icon: i.icon }));
        break;
      case "tag":
        items = (kit.tags ?? []).map((t) => ({ guid: t.guid, name: t.name, icon: t.icon }));
        break;
      case "concept":
        items = (kit.concepts ?? []).map((c) => ({ guid: c.guid, name: c.name, icon: c.icon }));
        break;
      case "file":
        items = (kit.files ?? []).map((f) => ({ guid: f.guid, name: f.name, parentGuid: f.folder?.guid }));
        break;
      case "folder":
        items = (kit.folders ?? []).map((f) => ({ guid: f.guid, name: f.name, parentGuid: f.parent?.guid }));
        break;
      case "author":
        items = (kit.authors ?? []).map((a) => ({ guid: a.guid, name: a.name }));
        break;
    }

    for (const item of items) {
      nodes.push({
        id: item.guid,
        type: "artifact",
        position: { x: 0, y: 0 },
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
        edges.push({
          id: `${item.parentGuid}-${item.guid}`,
          source: item.parentGuid,
          target: item.guid,
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
        const edgeId = `ref-${design.guid}-${typeGuid}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: typeGuid,
            target: design.guid,
            type: "floating",
            style: edgeStyle["reference"],
            data: { relationship: "reference" },
          });
        }
      }
      if (piece.design?.guid) {
        const nestedDesignGuid = piece.design.guid;
        const edgeId = `ref-${design.guid}-${nestedDesignGuid}`;
        if (!edges.some((e) => e.id === edgeId)) {
          edges.push({
            id: edgeId,
            source: nestedDesignGuid,
            target: design.guid,
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
  const [setHover] = useKitAppSetHover();
  const [clearHover] = useKitAppClearHover();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? "";
  const actor = useSketchpadActor();
  const [diagramForce] = useKitAppDiagramForce();
  const simulationRef = useRef<Simulation<ForceNode, ForceLink> | null>(null);
  const [nodes, setNodes] = useState<Node<KitDiagramNode>[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [isSimulating, setIsSimulating] = useState(true);
  const draggingNodeRef = useRef<string | null>(null);
  const hasFittedRef = useRef(false);
  const { fitView } = useReactFlow();

  const filterSearchSelector = useMemo(() => createKitFilterSearchSelector(kitGuid), [kitGuid]);
  const expandedRowsSelector = useMemo(() => createKitExpandedRowsSelector(kitGuid), [kitGuid]);
  const filterSearch = useSelector(actor, filterSearchSelector) ?? "";
  const expandedRowsSet = useSelector(actor, expandedRowsSelector);
  const expandedRowsArray = useMemo(() => (expandedRowsSet ? Array.from(expandedRowsSet) : []), [expandedRowsSet]);
  const expandedRowsKey = expandedRowsArray.join(",");
  const expandedRows = useMemo(() => new Set(expandedRowsArray), [expandedRowsKey]);

  // Compute visible GUIDs based on table row visibility (expanded/collapsed state)
  const visibleGuids = useMemo(() => {
    if (!kit) return new Set<string>();
    const guids = new Set<string>();
    const searchLower = filterSearch.toLowerCase();

    // Build lookup maps for hierarchy
    const designByGuid = new Map((kit.designs ?? []).map((d) => [d.guid, d]));
    const typeByGuid = new Map((kit.types ?? []).map((t) => [t.guid, t]));
    const folderByGuid = new Map((kit.folders ?? []).map((f) => [f.guid, f]));

    // Helper to check if a row's ancestors are all expanded
    const isAncestorChainExpanded = (rowId: string, parentRowId: string | undefined): boolean => {
      if (!parentRowId) return true; // No parent, always visible
      if (!expandedRows.has(parentRowId)) return false; // Parent is collapsed
      // Check parent's parent recursively by extracting info from parentRowId
      // Format: "design-{guid}", "type-{guid}", "folder-{guid}"
      const [kind, guid] = parentRowId.split("-", 2);
      if (kind === "design") {
        const parentDesign = designByGuid.get(guid);
        if (parentDesign?.parent?.guid) {
          return isAncestorChainExpanded(parentRowId, `design-${parentDesign.parent.guid}`);
        }
        // Check if design is in a folder (folder is a string guid)
        if (parentDesign?.folder) {
          return isAncestorChainExpanded(parentRowId, `folder-${parentDesign.folder}`);
        }
      } else if (kind === "type") {
        const parentType = typeByGuid.get(guid);
        if (parentType?.parent?.guid) {
          return isAncestorChainExpanded(parentRowId, `type-${parentType.parent.guid}`);
        }
        // Check if type is in a folder (folder is a string guid)
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

    // Helper to check if an entity's row is visible in the table
    const isRowVisible = (rowId: string, parentRowId: string | undefined, name: string): boolean => {
      if (searchLower && !name.toLowerCase().includes(searchLower)) return false;
      return isAncestorChainExpanded(rowId, parentRowId);
    };

    // Designs - check hierarchy (parent is { guid }, folder is string)
    (kit.designs ?? []).forEach((d) => {
      const rowId = `design-${d.guid}`;
      let parentRowId: string | undefined;
      if (d.parent?.guid) {
        parentRowId = `design-${d.parent.guid}`;
      } else if (d.folder) {
        parentRowId = `folder-${d.folder}`;
      }
      if (isRowVisible(rowId, parentRowId, d.name)) {
        guids.add(d.guid);
      }
    });

    // Types - check hierarchy (parent is { guid }, folder is string)
    (kit.types ?? []).forEach((t) => {
      const rowId = `type-${t.guid}`;
      let parentRowId: string | undefined;
      if (t.parent?.guid) {
        parentRowId = `type-${t.parent.guid}`;
      } else if (t.folder) {
        parentRowId = `folder-${t.folder}`;
      }
      if (isRowVisible(rowId, parentRowId, t.name)) {
        guids.add(t.guid);
      }
    });

    // Qualities - check folder (folder is string)
    (kit.qualities ?? []).forEach((q) => {
      const rowId = `quality-${q.guid}`;
      const parentRowId = q.folder ? `folder-${q.folder}` : undefined;
      if (isRowVisible(rowId, parentRowId, q.name)) {
        guids.add(q.guid);
      }
    });

    // Interfaces - no hierarchy
    (kit.interfaces ?? []).forEach((i) => {
      if (!searchLower || i.name.toLowerCase().includes(searchLower)) {
        guids.add(i.guid);
      }
    });

    // Tags - no hierarchy
    (kit.tags ?? []).forEach((t) => {
      if (!searchLower || t.name.toLowerCase().includes(searchLower)) {
        guids.add(t.guid);
      }
    });

    // Concepts - no hierarchy
    (kit.concepts ?? []).forEach((c) => {
      if (!searchLower || c.name.toLowerCase().includes(searchLower)) {
        guids.add(c.guid);
      }
    });

    // Files - check folder (folder is { guid })
    (kit.files ?? []).forEach((f) => {
      const rowId = `file-${f.guid}`;
      const parentRowId = f.folder?.guid ? `folder-${f.folder.guid}` : undefined;
      if (isRowVisible(rowId, parentRowId, f.name)) {
        guids.add(f.guid);
      }
    });

    // Folders - check hierarchy (parent is { guid })
    (kit.folders ?? []).forEach((f) => {
      const rowId = `folder-${f.guid}`;
      const parentRowId = f.parent?.guid ? `folder-${f.parent.guid}` : undefined;
      if (isRowVisible(rowId, parentRowId, f.name)) {
        guids.add(f.guid);
      }
    });

    // Authors - no hierarchy
    (kit.authors ?? []).forEach((a) => {
      if (!searchLower || a.name.toLowerCase().includes(searchLower)) {
        guids.add(a.guid);
      }
    });

    return guids;
  }, [kit, filterSearch, expandedRows]);

  const { initialEdges, forceNodes, forceLinks } = useMemo(() => {
    if (!kit) return { initialEdges: [], forceNodes: [], forceLinks: [] };
    const { nodes: rfNodes, edges: rfEdges } = buildKitDiagramData(kit);

    const filteredNodes = rfNodes.filter((n) => visibleGuids.has(n.id));
    const filteredNodeIds = new Set(filteredNodes.map((n) => n.id));
    const filteredEdges = rfEdges.filter((e) => filteredNodeIds.has(e.source) && filteredNodeIds.has(e.target));

    const spread = Math.min(500, Math.max(100, Math.sqrt(filteredNodes.length) * 30));
    const fNodes: ForceNode[] = filteredNodes.map((n, i) => {
      const angle = (i / filteredNodes.length) * Math.PI * 2;
      const radius = spread * 0.3 + Math.random() * spread * 0.2;
      return {
        id: n.id,
        x: Math.cos(angle) * radius,
        y: Math.sin(angle) * radius,
        data: n.data,
      };
    });
    const fLinks: ForceLink[] = filteredEdges.map((e) => ({
      id: e.id,
      source: e.source,
      target: e.target,
      relationship: (e.data?.relationship as "part-of" | "reference") ?? "part-of",
    }));
    return { initialEdges: filteredEdges, forceNodes: fNodes, forceLinks: fLinks };
  }, [kit, visibleGuids]);

  useEffect(() => {
    hasFittedRef.current = false;
  }, [kit, visibleGuids]);

  useEffect(() => {
    if (forceNodes.length === 0) {
      setNodes([]);
      setEdges([]);
      return;
    }
    setEdges(initialEdges);
    const nodesCopy = forceNodes.map((n) => ({ ...n }));
    const linksCopy = forceLinks.map((l) => ({ ...l }));
    const simulation = forceSimulation<ForceNode, ForceLink>(nodesCopy)
      .force("charge", forceManyBody().strength(diagramForce.chargeStrength))
      .force(
        "link",
        forceLink<ForceNode, ForceLink>(linksCopy)
          .id((d) => d.id)
          .distance(diagramForce.linkDistance),
      )
      .force("collide", forceCollide().radius(diagramForce.collideRadius))
      .force("center", forceCenter(0, 0).strength(diagramForce.centerStrength))
      .stop();

    setIsSimulating(true);
    for (let i = 0; i < 120; i++) {
      simulation.tick();
    }

    setNodes(
      nodesCopy.map((n) => ({
        id: n.id,
        type: "artifact",
        position: { x: n.x ?? 0, y: n.y ?? 0 },
        data: n.data,
        style: { width: 220, height: 140 },
      })),
    );

    setIsSimulating(false);
    simulationRef.current = null;

    if (!hasFittedRef.current) {
      hasFittedRef.current = true;
      setTimeout(() => fitView({ padding: 0.3, duration: 200, minZoom: 1, maxZoom: 1 }), 50);
    }
    return () => {
      simulation.stop();
      simulationRef.current = null;
    };
  }, [forceNodes, forceLinks, initialEdges, diagramForce, fitView]);

  const handleNodeDragStart = useCallback((_: React.MouseEvent, node: Node<KitDiagramNode>) => {
    draggingNodeRef.current = node.id;
  }, []);

  const handleNodeDrag = useCallback((_: React.MouseEvent, node: Node<KitDiagramNode>) => {
    if (draggingNodeRef.current !== node.id) return;
    setNodes((prev) => prev.map((n) => (n.id === node.id ? { ...n, position: node.position } : n)));
  }, []);

  const handleNodeDragStop = useCallback((_: React.MouseEvent, node: Node<KitDiagramNode>) => {
    draggingNodeRef.current = null;
    setNodes((prev) => prev.map((n) => (n.id === node.id ? { ...n, position: node.position } : n)));
  }, []);

  const handlePaneClick = useCallback(() => {
    kitCommands.deselectAll?.();
  }, [kitCommands]);

  const handleNodeClick = useCallback(
    (e: React.MouseEvent, node: Node<KitDiagramNode>) => {
      e.stopPropagation();
      const kind = node.data?.kind;
      const guid = node.data?.guid;
      if (!kind || !guid) return;
      const eventTypeMap: Record<DiagramNodeKind, string> = {
        type: "KIT.SELECT_TYPE",
        design: "KIT.SELECT_DESIGN",
        quality: "KIT.SELECT_QUALITY",
        interface: "KIT.SELECT_INTERFACE",
        tag: "KIT.SELECT_TAG",
        concept: "KIT.SELECT_CONCEPT",
        file: "KIT.SELECT_FILE",
        folder: "KIT.SELECT_FOLDER",
        author: "KIT.SELECT_AUTHOR",
      };
      const guidFieldMap: Record<DiagramNodeKind, string> = {
        type: "typeGuid",
        design: "designGuid",
        quality: "qualityGuid",
        interface: "interfaceGuid",
        tag: "tagGuid",
        concept: "conceptGuid",
        file: "fileGuid",
        folder: "folderGuid",
        author: "authorGuid",
      };
      const eventType = eventTypeMap[kind];
      const guidField = guidFieldMap[kind];
      if (!eventType || !guidField) return;
      actor.send({
        type: eventType,
        kitGuid,
        [guidField]: guid,
      } as any);
    },
    [actor, kitGuid],
  );

  const handleNodeMouseEnter = useCallback(
    (_: any, node: Node<KitDiagramNode>) => {
      const kind = node.data?.kind;
      const guid = node.data?.guid;
      if (!kind || !guid) return;
      if (!setHover) return;
      if (kind === "type") setHover({ type: guid });
      else if (kind === "design") setHover({ design: guid });
    },
    [setHover],
  );

  const handleNodeMouseLeave = useCallback(() => {
    if (clearHover) clearHover();
  }, [clearHover]);

  if (!kit) return null;

  const edgeTypes = useMemo(
    () => ({
      floating: FloatingEdge,
    }),
    [],
  );

  return (
    <div ref={reactFlowWrapper} className="w-full h-full" data-testid="kit-diagram">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={kitNodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={FloatingConnectionLine}
        minZoom={0.1}
        maxZoom={4}
        defaultViewport={{ x: 0, y: 0, zoom: 1 }}
        panOnDrag={false}
        panOnScroll={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        zoomOnDoubleClick={false}
        onPaneClick={handlePaneClick}
        onNodeClick={handleNodeClick}
        onNodeMouseEnter={handleNodeMouseEnter}
        onNodeMouseLeave={handleNodeMouseLeave}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        nodesDraggable={true}
        proOptions={{ hideAttribution: true }}
      >
        <Background gap={20} size={1} />
      </ReactFlow>
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

  // Add toolbar section with artifact kind filter toggles
  useEffect(() => {
    if (appType !== "kit") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.kit.kitApp.filters",
      specificity: 20,
      order: 0,
      content: <KitToolbarFilters />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.kit.kitApp.filters");
    };
  }, [appType, addSection, removeSection]);

  // Wait for kit to be available before rendering GoldenLayout
  // This prevents a race condition where GoldenLayout renders before the kit is loaded
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

  // Wait for kit to be available before rendering GoldenLayout
  // This prevents a race condition where GoldenLayout windows render and show NotFound
  // before the kit is loaded into the store
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

// #endregion Windows

// #region Panels

// #region Right

// #region Details

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
    const kitDataSource = useKitStore() as any;
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

export const InterfaceSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const selectedInterfaces = selection?.interfaces || [];
  if (selectedInterfaces.length === 0) return null;
  if (selectedInterfaces.length === 1) return <SingleInterfaceSection interfaceGuid={selectedInterfaces[0]} />;
  return <MultipleInterfacesSection interfaceGuids={selectedInterfaces} />;
};

const SingleInterfaceSection: FC<{ interfaceGuid: string }> = ({ interfaceGuid }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const iface = kit?.interfaces?.find((i) => i.guid === interfaceGuid);
  if (!iface) return null;
  const compatibleCount = iface.compatibleInterfaces?.length || 0;
  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input id="semio.sketchpad.app.kit.panel.details.section.interface.name" value={iface.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.kit.panel.details.section.interface.description" value={iface.description || ""} placeholder={t("semio.sketchpad.app.kit.interface.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.kit.panel.details.section.interface.compatible"
            value={compatibleCount === 0 ? t("semio.sketchpad.app.kit.interface.allCompatible") : `${compatibleCount} ${t("semio.sketchpad.app.kit.interface.compatibleInterfaces")}`}
            readOnly
            showLabel
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

const MultipleInterfacesSection: FC<{ interfaceGuids: string[] }> = ({ interfaceGuids }) => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const interfaces = interfaceGuids.map((guid) => kit?.interfaces?.find((i) => i.guid === guid)).filter((i) => i !== undefined) as Interface[];
  return (
    <>
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.interfaces.multipleSelected")}</p>
        </TreeContent>
      </TreeItem>
      {interfaces.map((iface) => (
        <TreeItem key={iface.guid}>
          <TreeContent>
            <p className="text-sm font-medium">{iface.name}</p>
          </TreeContent>
        </TreeItem>
      ))}
    </>
  );
};

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

export const FolderSection: FC = () => {
  const { t } = useTranslation();
  const kit = useKit() as Kit;
  const kitDataSource = useKitStore() as any;
  const [selection] = useKitAppSelection();
  const selectedFolders = selection?.folders || [];

  if (selectedFolders.length === 0) return null;

  const folders = selectedFolders
    .map((folderGuid) => {
      return kit.folders?.find((f) => f.guid === folderGuid);
    })
    .filter(Boolean);

  if (folders.length === 0) return null;
  if (folders.length > 1) return null; // Show only single folder

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

export const MultipleArtifactsSection: FC = () => {
  const { t } = useTranslation();
  const [selection] = useKitAppSelection();
  const typesCount = selection?.types?.length || 0;
  const designsCount = selection?.designs?.length || 0;
  const qualitiesCount = selection?.qualities?.length || 0;
  const interfacesCount = selection?.interfaces?.length || 0;
  const tagsCount = selection?.tags?.length || 0;
  const conceptsCount = selection?.concepts?.length || 0;
  const filesCount = selection?.files?.length || 0;
  const authorsCount = selection?.authors?.length || 0;
  const kinds: string[] = [];
  if (typesCount > 0) kinds.push(t("semio.sketchpad.app.kit.types.multipleTitle", { count: typesCount }));
  if (designsCount > 0) kinds.push(t("semio.sketchpad.app.kit.designs.multipleTitle", { count: designsCount }));
  if (qualitiesCount > 0) kinds.push(t("semio.sketchpad.app.kit.qualities.multipleTitle", { count: qualitiesCount }));
  if (interfacesCount > 0) kinds.push(t("semio.sketchpad.app.kit.interfaces.multipleTitle", { count: interfacesCount }));
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

// #region Chat

// #endregion Chat

// #region Settings

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

// #region Tools

// #endregion Tools

// #endregion Canvas

// #region Footer

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

// #region App

// Main app export is in the Table section above

// #endregion App

// #region Config

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
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 2 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]);
  },
  order: 10,
};

// #endregion Config
