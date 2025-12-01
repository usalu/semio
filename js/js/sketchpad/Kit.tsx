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

import { DragEndEvent, useDroppable } from "@dnd-kit/core";
import {
  AddIcon,
  AwardIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  DocumentIcon,
  FileCodeIcon,
  FileImageIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTypeIcon,
  FileVideoIcon,
  FolderIcon,
  InterfaceIcon,
  LayoutIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  TypeIcon,
  UserIcon,
} from "@semio/assets";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import JSZip from "jszip";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { Camera } from "three";
import * as Y from "yjs";
import i18n, { useLabel } from "../i18n";
import { Author, buildFileTree, Coord, Design, DesignDiff, DiffStatus, flattenFileTree, Folder, generateUniqueName, guid, Guid, Kit, KitDiff, Quality, File as SemioFile, Type, TypeDiff } from "../semio";
import type { KitStore, SketchpadStore } from "./Sketchpad";
import {
  Canvas,
  ConceptFilter,
  identitySelector,
  KitDiffAppStore,
  KitScopeProvider,
  registerKitAppStoreFactory,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useExpertise,
  useFocus,
  useHasKit,
  useIsInKitScope,
  useIsMobile,
  useKit,
  useKitCommands,
  useKitScope,
  useKitStore,
  useLanguage,
  useLayout,
  useMode,
  useNavigation,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadCommands,
  useSketchpadStore,
  useSync,
  useSyncDeep,
  useTheme,
  Window,
} from "./Sketchpad";
import { Action, Input, NotFound, Scrollable, Strip, Table, TableAvatar, Textarea, Toggle, TreeContent, TreeItem } from "./elements";
import type { KitAppId, KitCommandContext, KitDiffAppEdit, Layout, PanelDefinition, PanelVisibility, Theme, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "./shared";
import { AppConfig, createPanelDefinition, PanelKind } from "./shared";

// #endregion Imports

// #region Store

type YKitAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitApp = Y.Map<YKitAppVal>;
type YKitApps = Y.Map<YKitApp>;

export interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  interfaces?: Guid[];
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
  files?: KitAppSelectionFilesDiff;
  folders?: KitAppSelectionFoldersDiff;
  authors?: KitAppSelectionAuthorsDiff;
}
export enum KitAppFullscreenWindow {
  None = "none",
  Types = "types",
  Designs = "designs",
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

class KitAppStore extends KitDiffAppStore<KitAppState, KitAppDiff, KitAppSelectionDiff, KitAppEdit, KitAppCommandContext, KitAppCommandResult> {
  constructor(parent: SketchpadStore, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) {
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

  kit(): KitStore {
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
      // diff: this.diff, // TODO: KitAppState doesn't have a diff property
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
      filterSearch: this.filterSearch,
      expandedRows: this.expandedRows,
      sortColumn: this.sortColumn,
      sortDirection: this.sortDirection,
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
      console.group(`[${origin || "unknown"}] Transaction: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.kitApp.finalizeTransaction") {
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitApp.abortTransaction") {
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitApp.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.kitApp.redo") {
      console.group(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      this.redo();
      console.groupEnd();
      return {} as T;
    }

    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit app store`);

    const kitStore = this.kit();
    const state = this.snapshot();

    const context: KitAppCommandContext = {
      kitApp: state,
      kit: kitStore.snapshot(),
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

if (typeof window !== "undefined") {
  registerKitAppStoreFactory((parent, yMap, transact, id, state) => new KitAppStore(parent, yMap, transact, id, state));
}

function useKitAppStore<T>(selector?: (store: KitAppStore) => T, id?: KitAppId): T | KitAppStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) {
    return null;
  }
  try {
    if (!store.hasKit(resolvedKitId)) {
      return null;
    }
    const kitAppStore = store.kitApp(resolvedKitId);
    const result = selector ? selector(kitAppStore) : kitAppStore;
    return result;
  } catch {
    return null;
  }
}

export function useKitApp<T>(selector?: (state: KitAppState) => T, id?: KitAppId): T | KitAppState | null {
  const store = useKitAppStore(undefined, id);
  if (!store) {
    return null;
  }
  const result = useSyncDeep<KitAppState>(store as KitAppStore, selector ? (state: KitAppState) => selector(state) as any : identitySelector);
  return result;
}

export function useKitAppSelection(id?: KitAppId): KitAppSelection {
  const store = useKitAppStore(identitySelector, id) as KitAppStore | null;
  if (!store) return emptyKitAppSelection;
  return (useSync<KitAppState>(store, (state) => (state.selection ?? emptyKitAppSelection) as any as KitAppState) as unknown as KitAppSelection) ?? emptyKitAppSelection;
}

export function useKitAppFullscreen(): KitAppFullscreenWindow {
  return useKitApp((s) => s.fullscreenWindow) as KitAppFullscreenWindow;
}

export function useKitAppOthers(): KitAppPresenceOther[] {
  return useKitApp((s) => s.others) as KitAppPresenceOther[];
}

export function useKitAppTransaction(origin: string, id?: KitAppId): Transaction {
  const store = useKitAppStore(undefined, id) as KitAppStore | null;
  if (!store) {
    return {};
  }
  return {
    start: () => store.execute("semio.kitApp.startTransaction", origin),
    finalize: () => store.execute("semio.kitApp.finalizeTransaction", origin),
    abort: () => store.execute("semio.kitApp.abortTransaction", origin),
  };
}

export function useKitAppCommands(id?: KitAppId) {
  const store = useKitAppStore(undefined, id) as KitAppStore | null;
  const noOp = () => {};
  if (!store) {
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
    undo: (origin: string) => store.execute("semio.kitApp.undo", origin),
    redo: (origin: string) => store.execute("semio.kitApp.redo", origin),
    selectAll: (origin: string) => store.execute("semio.kitApp.selectAll", origin),
    deselectAll: (origin: string) => store.execute("semio.kitApp.deselectAll", origin),
    selectType: (origin: string, Guid: Guid) => store.execute("semio.kitApp.selectType", origin, Guid),
    selectTypes: (origin: string, typeIds: Guid[]) => store.execute("semio.kitApp.selectTypes", origin, typeIds),
    addTypeToSelection: (origin: string, Guid: Guid) => store.execute("semio.kitApp.addTypeToSelection", origin, Guid),
    removeTypeFromSelection: (origin: string, Guid: Guid) => store.execute("semio.kitApp.removeTypeFromSelection", origin, Guid),
    selectDesign: (origin: string, Guid: Guid) => store.execute("semio.kitApp.selectDesign", origin, Guid),
    selectDesigns: (origin: string, designIds: Guid[]) => store.execute("semio.kitApp.selectDesigns", origin, designIds),
    addDesignToSelection: (origin: string, Guid: Guid) => store.execute("semio.kitApp.addDesignToSelection", origin, Guid),
    removeDesignFromSelection: (origin: string, Guid: Guid) => store.execute("semio.kitApp.removeDesignFromSelection", origin, Guid),
    selectQuality: (origin: string, key: string) => store.execute("semio.kitApp.selectQuality", origin, key),
    selectQualities: (origin: string, keys: string[]) => store.execute("semio.kitApp.selectQualities", origin, keys),
    addQualityToSelection: (origin: string, key: string) => store.execute("semio.kitApp.addQualityToSelection", origin, key),
    removeQualityFromSelection: (origin: string, key: string) => store.execute("semio.kitApp.removeQualityFromSelection", origin, key),
    selectInterface: (origin: string, guid: Guid) => store.execute("semio.kitApp.selectInterface", origin, guid),
    selectInterfaces: (origin: string, guids: Guid[]) => store.execute("semio.kitApp.selectInterfaces", origin, guids),
    addInterfaceToSelection: (origin: string, guid: Guid) => store.execute("semio.kitApp.addInterfaceToSelection", origin, guid),
    removeInterfaceFromSelection: (origin: string, guid: Guid) => store.execute("semio.kitApp.removeInterfaceFromSelection", origin, guid),
    selectFile: (origin: string, path: string) => store.execute("semio.kitApp.selectFile", origin, path),
    selectFiles: (origin: string, paths: string[]) => store.execute("semio.kitApp.selectFiles", origin, paths),
    addFileToSelection: (origin: string, path: string) => store.execute("semio.kitApp.addFileToSelection", origin, path),
    removeFileFromSelection: (origin: string, path: string) => store.execute("semio.kitApp.removeFileFromSelection", origin, path),
    selectFolder: (origin: string, guid: Guid) => store.execute("semio.kitApp.selectFolder", origin, guid),
    selectFolders: (origin: string, guids: Guid[]) => store.execute("semio.kitApp.selectFolders", origin, guids),
    addFolderToSelection: (origin: string, guid: Guid) => store.execute("semio.kitApp.addFolderToSelection", origin, guid),
    removeFolderFromSelection: (origin: string, guid: Guid) => store.execute("semio.kitApp.removeFolderFromSelection", origin, guid),
    selectAuthor: (origin: string, name: string) => store.execute("semio.kitApp.selectAuthor", origin, name),
    selectAuthors: (origin: string, names: string[]) => store.execute("semio.kitApp.selectAuthors", origin, names),
    addAuthorToSelection: (origin: string, name: string) => store.execute("semio.kitApp.addAuthorToSelection", origin, name),
    removeAuthorFromSelection: (origin: string, name: string) => store.execute("semio.kitApp.removeAuthorFromSelection", origin, name),
    deleteSelected: (origin: string) => store.execute("semio.kitApp.deleteSelected", origin),
    toggleTypesFullscreen: (origin: string) => store.execute("semio.kitApp.toggleTypesFullscreen", origin),
    toggleDesignsFullscreen: (origin: string) => store.execute("semio.kitApp.toggleDesignsFullscreen", origin),
    addType: (origin: string, type: Type) => store.execute("semio.kitApp.addType", origin, type),
    addTypes: (origin: string, types: Type[]) => store.execute("semio.kitApp.addTypes", origin, types),
    removeType: (origin: string, Guid: Guid) => store.execute("semio.kitApp.removeType", origin, Guid),
    removeTypes: (origin: string, typeIds: Guid[]) => store.execute("semio.kitApp.removeTypes", origin, typeIds),
    addDesign: (origin: string, design: Design) => store.execute("semio.kitApp.addDesign", origin, design),
    addDesigns: (origin: string, designs: Design[]) => store.execute("semio.kitApp.addDesigns", origin, designs),
    removeDesign: (origin: string, Guid: Guid) => store.execute("semio.kitApp.removeDesign", origin, Guid),
    removeDesigns: (origin: string, designIds: Guid[]) => store.execute("semio.kitApp.removeDesigns", origin, designIds),
    updateType: (origin: string, guid: Guid, typeDiff: TypeDiff) => store.execute("semio.kitApp.updateType", origin, guid, typeDiff),
    updateTypes: (origin: string, updates: { id: Guid; diff: TypeDiff }[]) => store.execute("semio.kitApp.updateTypes", origin, updates),
    updateDesign: (origin: string, guid: Guid, designDiff: DesignDiff) => store.execute("semio.kitApp.updateDesign", origin, guid, designDiff),
    updateDesigns: (origin: string, updates: { id: Guid; diff: DesignDiff }[]) => store.execute("semio.kitApp.updateDesigns", origin, updates),
    togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setFilterSearch: (origin: string, search: string) => store.execute("semio.kitApp.setFilterSearch", origin, search),
    setExpandedRows: (origin: string, rows: string[]) => store.execute("semio.kitApp.setExpandedRows", origin, rows),
    toggleExpandedRow: (origin: string, rowId: string) => store.execute("semio.kitApp.toggleExpandedRow", origin, rowId),
    setSortColumn: (origin: string, column: KitAppSortColumn) => store.execute("semio.kitApp.setSortColumn", origin, column),
    setSortDirection: (origin: string, direction: KitAppSortDirection) => store.execute("semio.kitApp.setSortDirection", origin, direction),
    toggleSort: (origin: string, column: KitAppSortColumn) => store.execute("semio.kitApp.toggleSort", origin, column),
    execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
  };
}

// #endregion Kit App

// #region Types

/**
 * Check if a specific type is directly hovered in Kit App
 */
export function useKitAppIsTypeHovered(typeId: string, id?: KitAppId): boolean {
  return (useKitApp((state) => state.hover?.type === typeId, id) as boolean) ?? false;
}

/**
 * Get the diff status of a type from the current kit diff
 */
export function useKitAppTypeStatus(typeId: string, id?: KitAppId): DiffStatus {
  const store = useKitAppStore(identitySelector, id) as KitAppStore;
  if (!store) return DiffStatus.Unchanged;

  const state = useSync<KitAppState>(store, identitySelector);
  const currentStack = store?.currentTransactionStack;

  if (currentStack && currentStack.length > 0) {
    for (const edit of currentStack) {
      if (edit.do?.kitDiff?.types) {
        // Check added types
        if (edit.do.kitDiff.types.added) {
          for (const type of edit.do.kitDiff.types.added) {
            if (type.guid === typeId) {
              return DiffStatus.Added;
            }
          }
        }
        // Check removed types
        if (edit.do.kitDiff.types.removed) {
          for (const removedId of edit.do.kitDiff.types.removed) {
            if (removedId === typeId) {
              return DiffStatus.Removed;
            }
          }
        }
        // Check modified types
        if (edit.do.kitDiff.types.updated) {
          for (const typeUpdate of edit.do.kitDiff.types.updated) {
            if (typeUpdate.id === typeId) {
              return DiffStatus.Modified;
            }
          }
        }
      }
    }
  }

  return DiffStatus.Unchanged;
}

/**
 * Get the color for a type based on its state
 */
export function useKitAppTypeColor(typeId: string, isSelected: boolean, id?: KitAppId): { fill: string; stroke: string; opacity: number } {
  const isHovered = useKitAppIsTypeHovered(typeId, id);
  const status = useKitAppTypeStatus(typeId, id);

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  // Base state colors
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

  // Hover state
  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  // Selected state
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

// #endregion Types

// #region Designs

/**
 * Check if a specific design is directly hovered in Kit App
 */
export function useKitAppIsDesignHovered(designId: string, id?: KitAppId): boolean {
  return (useKitApp((state) => state.hover?.design === designId, id) as boolean) ?? false;
}

/**
 * Get the diff status of a design from the current kit diff
 */
export function useKitAppDesignStatus(designId: string, id?: KitAppId): DiffStatus {
  const store = useKitAppStore(identitySelector, id) as KitAppStore;
  if (!store) return DiffStatus.Unchanged;

  const state = useSync<KitAppState>(store, identitySelector);
  const currentStack = store?.currentTransactionStack;

  if (currentStack && currentStack.length > 0) {
    for (const edit of currentStack) {
      if (edit.do?.kitDiff?.designs) {
        // Check added designs
        if (edit.do.kitDiff.designs.added) {
          for (const design of edit.do.kitDiff.designs.added) {
            if (design.guid === designId) {
              return DiffStatus.Added;
            }
          }
        }
        // Check removed designs
        if (edit.do.kitDiff.designs.removed) {
          for (const removedId of edit.do.kitDiff.designs.removed) {
            if (removedId === designId) {
              return DiffStatus.Removed;
            }
          }
        }
        // Check modified designs
        if (edit.do.kitDiff.designs.updated) {
          for (const designUpdate of edit.do.kitDiff.designs.updated) {
            if (designUpdate.id === designId) {
              return DiffStatus.Modified;
            }
          }
        }
      }
    }
  }

  return DiffStatus.Unchanged;
}

/**
 * Get the color for a design based on its state
 */
export function useKitAppDesignColor(designId: string, isSelected: boolean, id?: KitAppId): { fill: string; stroke: string; opacity: number } {
  const isHovered = useKitAppIsDesignHovered(designId, id);
  const status = useKitAppDesignStatus(designId, id);

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  // Base state colors
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

  // Hover state
  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  // Selected state
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

// #endregion Designs

// #endregion Store

// #region Commands

export const commands = {
  "semio.kitApp.setTheme": (context: KitAppCommandContext, theme: Theme): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.setLayout": (context: KitAppCommandContext, layout: Layout): KitAppCommandResult => {
    return { diff: {} };
  },
  "semio.kitApp.toggleTypesFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Types ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Types;
    return {
      diff: {
        fullscreenWindow: newPanel,
      },
    };
  },
  "semio.kitApp.toggleDesignsFullscreen": (context: KitAppCommandContext): KitAppCommandResult => {
    const currentPanel = context.kitApp.fullscreenWindow;
    const newPanel = currentPanel === KitAppFullscreenWindow.Designs ? KitAppFullscreenWindow.None : KitAppFullscreenWindow.Designs;
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
        types: { removed: selection?.types },
        designs: { removed: selection?.designs },
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
        types: { removed: [Guid] },
      },
    };
  },
  "semio.kitApp.removeTypes": (context: KitAppCommandContext, typeIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: typeIds },
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
        designs: { removed: [Guid] },
      },
    };
  },
  "semio.kitApp.removeDesigns": (context: KitAppCommandContext, designIds: Guid[]): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: designIds },
      },
    };
  },
  "semio.kitApp.updateType": (context: KitAppCommandContext, guid: Guid, typeDiff: TypeDiff): KitAppCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: [{ id: guid, diff: typeDiff }] },
      },
    };
  },
  "semio.kitApp.updateTypes": (context: KitAppCommandContext, updates: { id: Guid; diff: TypeDiff }[]): KitAppCommandResult => {
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
        designs: { updated: [{ id: guid, diff: designDiff }] },
      },
    };
  },
  "semio.kitApp.updateDesigns": (context: KitAppCommandContext, updates: { id: Guid; diff: DesignDiff }[]): KitAppCommandResult => {
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

type ArtifactKind = "designs" | "types" | "qualities" | "interfaces" | "files" | "folders" | "authors";

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
  data: Design | Type | Quality | Interface | SemioFile | Author | Folder;
  folderId?: string;
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
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      const hasZip = Array.from(e.dataTransfer.items).some((item) => item.kind === "file" && item.type === "application/zip");
      if (hasZip) {
        setIsDragging(true);
      }
    }
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

    const files = Array.from(e.dataTransfer.files);
    const zipFile = files.find((f) => f.name.endsWith(".zip"));

    if (zipFile && kitCommands) {
      try {
        const arrayBuffer = await zipFile.arrayBuffer();
        const zip = await JSZip.loadAsync(arrayBuffer);
        const hasKitDb = zip.file(".semio/kit.db") !== null;

        if (hasKitDb) {
          const tempBlob = new Blob([arrayBuffer], { type: "application/zip" });
          const tempUrl = URL.createObjectURL(tempBlob);
          await kitCommands.importKit("semio.sketchpad.app.kit.dropzone", tempUrl);
          URL.revokeObjectURL(tempUrl);
        } else {
          const filesToAdd: { path: string; blob: Blob }[] = [];
          for (const [path, zipEntry] of Object.entries(zip.files)) {
            if (!zipEntry.dir && !path.startsWith(".semio/")) {
              const blob = await zipEntry.async("blob");
              filesToAdd.push({ path, blob });
            }
          }
          for (const file of filesToAdd) {
            const fileToAdd: SemioFile = {
              guid: guid(),
              path: file.path,
              remoteUrl: "",
              description: "",
              attributes: [],
            };
            await kitCommands.addFile("semio.sketchpad.app.kit.dropzone", fileToAdd, file.blob);
          }
        }
      } catch (error) {}
    }
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

  const kit = useKit(undefined, kitScope?.guid, true) as Kit;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitAppCommands = useKitAppCommands();
  const kitApp = useKitApp() as KitAppState;
  const isMobile = useIsMobile();
  const store = useSketchpadStore();

  const [isDragOver, setIsDragOver] = React.useState(false);
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

  // Early return if no kit is loaded
  if (!hasKit || !kit) {
    return <NotFound title={t("semio.sketchpad.app.kit.notFound.label.normal")} description={t("semio.sketchpad.app.kit.notFound.description.normal")} parentPath="/" parentLabel={t("semio.sketchpad.app.home.title")} />;
  }

  // Early return if kit app is not initialized yet
  if (!kitApp) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.kit.loading")}</p>
      </div>
    );
  }

  // Get filters from search params (?kind=&name=)
  const selectedKind = searchParams.get("kind") as ArtifactKind | null;
  const selectedName = searchParams.get("name");

  // Get concepts and search from search params
  const selectedConcepts = searchParams.getAll("c");
  const searchQuery = searchParams.get("q") || "";

  // Get selection parameter for auto-selecting designs/types
  const selectParam = searchParams.get("select");
  const expandedRowsArray = kitApp?.expandedRows || [];
  const expandedRowsArrayKey = expandedRowsArray.join(",");
  const expandedRows = useMemo(() => new Set(expandedRowsArray), [expandedRowsArrayKey]);

  const selectionTypes = kitApp?.selection?.types || [];
  const selectionDesigns = kitApp?.selection?.designs || [];
  const selectionQualities = kitApp?.selection?.qualities || [];
  const selectionFiles = kitApp?.selection?.files || [];
  const selectionFolders = kitApp?.selection?.folders || [];
  const selectionAuthors = kitApp?.selection?.authors || [];
  const selectionTypesKey = selectionTypes.join(",");
  const selectionDesignsKey = selectionDesigns.join(",");
  const selectionQualitiesKey = selectionQualities.join(",");
  const selectionFilesKey = selectionFiles.join(",");
  const selectionFoldersKey = selectionFolders.join(",");
  const selectionAuthorsKey = selectionAuthors.join(",");
  const selection = useMemo(
    () => ({
      types: selectionTypes,
      designs: selectionDesigns,
      qualities: selectionQualities,
      files: selectionFiles,
      folders: selectionFolders,
      authors: selectionAuthors,
    }),
    [selectionTypesKey, selectionDesignsKey, selectionQualitiesKey, selectionFilesKey, selectionFoldersKey, selectionAuthorsKey],
  );
  const sortColumn = kitApp?.sortColumn;
  const sortDirection = kitApp?.sortDirection || "asc";

  const kitDesigns = kit?.designs;
  const kitTypes = kit?.types;
  const kitQualities = kit?.qualities;
  const kitInterfaces = kit?.interfaces;
  const kitFiles = kit?.files;
  const kitFolders = kit?.folders;
  const kitAuthors = kit?.authors;
  const kitDesignsKey = useMemo(() => kitDesigns?.map((d) => `${d.guid}:${d.name}:${d.parent?.guid || ""}:${d.folder || ""}:${d.updatedAt || ""}`).join("|") || "", [kitDesigns]);
  const kitTypesKey = useMemo(() => kitTypes?.map((t) => `${t.guid}:${t.name}:${t.parent?.guid || ""}:${t.folder || ""}:${t.updatedAt || ""}`).join("|") || "", [kitTypes]);
  const kitQualitiesKey = useMemo(() => kitQualities?.map((q) => `${q.guid}:${q.name}:${q.folder || ""}`).join("|") || "", [kitQualities]);
  const kitInterfacesKey = useMemo(() => kitInterfaces?.map((i) => `${i.guid}:${i.name}`).join("|") || "", [kitInterfaces]);
  const kitFilesKey = useMemo(() => kitFiles?.map((f) => `${f.guid}:${f.name}:${f.folder?.guid || ""}:${f.updatedAt || ""}`).join("|") || "", [kitFiles]);
  const kitFoldersKey = useMemo(() => kitFolders?.map((f) => `${f.guid}:${f.name}:${f.parent?.guid || ""}:${f.updatedAt || ""}`).join("|") || "", [kitFolders]);
  const kitAuthorsKey = useMemo(() => kitAuthors?.map((a) => `${a.guid}:${a.name}`).join("|") || "", [kitAuthors]);

  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kitDesigns?.forEach((d: Design) => d.concepts?.forEach((c: string) => conceptSet.add(c)));
    return Array.from(conceptSet).sort();
  }, [kitDesignsKey]);

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

  useEffect(() => {
    if (appType !== "kit" || !kitScope?.guid || !hasKit) {
      return;
    }
    if (!store.hasKitApp({ kit: kitScope.guid })) {
      sketchpadCommands.createKitApp("semio.sketchpad.app.kit.autoCreate", { kit: kitScope.guid });
    }
  }, [appType, kitScope?.guid, hasKit, sketchpadCommands, store]);

  useEffect(() => {
    if (appType !== "kit") {
      return;
    }

    const selection = kitApp?.selection;
    const typesCount = selection?.types?.length || 0;
    const designsCount = selection?.designs?.length || 0;
    const qualitiesCount = selection?.qualities?.length || 0;
    const interfacesCount = selection?.interfaces?.length || 0;
    const filesCount = selection?.files?.length || 0;
    const foldersCount = selection?.folders?.length || 0;
    const authorsCount = selection?.authors?.length || 0;
    const totalSelectedKinds = [typesCount > 0, designsCount > 0, qualitiesCount > 0, interfacesCount > 0, filesCount > 0, foldersCount > 0, authorsCount > 0].filter(Boolean).length;

    const artifactsMultipleId = "semio.sketchpad.app.kit.artifacts.multiple";

    removeSection("details", artifactsMultipleId);
    removeSection("details", "semio.sketchpad.app.design.title");
    removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
    removeSection("details", "semio.sketchpad.app.type.title");
    removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.interface.title");
    removeSection("details", "semio.sketchpad.app.kit.interfaces.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.file.title");
    removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.folder.title");
    removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.title");

    if (totalSelectedKinds > 1) {
      addSection("details", {
        id: artifactsMultipleId,
        specificity: 30,
        order: 0,
        content: () => <MultipleArtifactsSection />,
      });
    }

    if (designsCount > 0 && totalSelectedKinds === 1) {
      const designSectionId = designsCount === 1 ? "semio.sketchpad.app.design.title" : "semio.sketchpad.app.kit.designs.multipleTitle";
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
      const typeSectionId = typesCount === 1 ? "semio.sketchpad.app.type.title" : "semio.sketchpad.app.kit.types.multipleTitle";
      addSection("details", {
        id: typeSectionId,
        specificity: 30,
        order: 20,
        content: () => <TypeSection />,
      });
    }

    if (interfacesCount > 0 && totalSelectedKinds === 1) {
      const interfaceSectionId = interfacesCount === 1 ? "semio.sketchpad.app.kit.interface.title" : "semio.sketchpad.app.kit.interfaces.multipleTitle";
      addSection("details", {
        id: interfaceSectionId,
        specificity: 30,
        order: 25,
        content: () => <InterfaceSection />,
      });
    }

    if (filesCount > 0 && totalSelectedKinds === 1) {
      const fileSectionId = filesCount === 1 ? "semio.sketchpad.app.kit.file.title" : "semio.sketchpad.app.kit.files.multipleTitle";
      addSection("details", {
        id: fileSectionId,
        specificity: 30,
        order: 30,
        content: () => <FileSection />,
      });
    }

    if (foldersCount > 0 && totalSelectedKinds === 1) {
      const folderSectionId = foldersCount === 1 ? "semio.sketchpad.app.kit.folder.title" : "semio.sketchpad.app.kit.folders.multipleTitle";
      addSection("details", {
        id: folderSectionId,
        specificity: 30,
        order: 40,
        content: () => <FolderSection />,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.kit.title",
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
      removeSection("details", "semio.sketchpad.app.design.title");
      removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
      removeSection("details", "semio.sketchpad.app.type.title");
      removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.file.title");
      removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.folder.title");
      removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.title");
    };
  }, [addSection, removeSection, appType, kitApp?.selection]);

  // Add settings sections
  useEffect(() => {
    if (appType !== "kit") return;

    const { setTheme, setLanguage, setLayout, setExpertise, setMode } = sketchpadCommands;

    const SettingsContent = () => {
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
              <Select id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage("semio.sketchpad.settings.language", value)} showLabel>
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

    // Add Kit-specific settings (more specific)
    addSection("settings", {
      id: "semio.sketchpad.app.kit.settings",
      specificity: 10,
      order: 0,
      content: SettingsContent,
    });

    // Add global Sketchpad settings (least specific)
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: SettingsContent,
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [appType, addSection, removeSection, sketchpadCommands]);

  // Auto-select design/type when select parameter is present
  useEffect(() => {
    if (!selectParam) return;

    if (selectedKind === "designs") {
      const design = kitDesigns?.find((d: Design) => d.guid === selectParam);
      if (design) {
        kitAppCommands.selectDesign("semio.sketchpad.app.kit.autoselect.design", selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKind === "types") {
      const type = kitTypes?.find((t: Type) => t.guid === selectParam);
      if (type) {
        kitAppCommands.selectType("semio.sketchpad.app.kit.autoselect.type", selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectParam, selectedKind]);

  const allRows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date) => {
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
          if (selectedConcepts.length > 0 && !design.concepts?.some((c) => selectedConcepts.includes(c))) return;
          if (searchQuery && !design.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
          // Skip root designs that are in folders when not viewing the folders kind
          // Only filter at root level (parentGuid === undefined), not children
          if (!selectedKind && parentGuid === undefined && design.folder) return;

          const rowId = `design-${design.guid}`;
          const children = designsByParent.get(design.guid) || [];
          const hasChildren = children.length > 0;

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
    kitFiles,
    kitFolders,
    kitAuthors,
    kitDesignsKey,
    kitTypesKey,
    kitQualitiesKey,
    kitInterfacesKey,
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
      if (row.kind === "designs") isSelected = selection.designs.includes((row.data as Design).guid);
      else if (row.kind === "types") isSelected = selection.types.includes((row.data as Type).guid);
      else if (row.kind === "qualities") isSelected = selection.qualities.includes((row.data as Quality).key);
      else if (row.kind === "files") isSelected = selection.files.includes((row.data as SemioFile).guid);
      else if (row.kind === "folders") isSelected = selection.folders?.includes((row.data as Folder).guid);
      else if (row.kind === "authors") isSelected = selection.authors.includes((row.data as Author).name);

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
    // Only update if the items have actually changed
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevRowsRef.current !== itemsKey) {
      prevRowsRef.current = itemsKey;
      setFocusItems(items);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rows]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
    kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.toggleRow", rowId);
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
          kitCommands.updateDesign("semio.sketchpad.app.kit.canvas.table.setDesignParent", design.guid, { parent: { guid: targetParentId } });
        }
      } else if (targetFolderId === undefined && (design.parent || design.folder)) {
        // Dropped on root - unparent and remove from folder
        kitCommands.updateDesign("semio.sketchpad.app.kit.canvas.table.unparentDesign", design.guid, { parent: undefined });
        if (design.folder) {
          kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveDesignToRoot", "design", design.guid, null);
        }
      } else if (!design.parent) {
        // Root design (protodesign) - can be moved to folders or root
        kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveDesignToFolder", "design", design.guid, targetFolderId ?? null);
      }
    } else if (draggedRow.kind === "types" && kitCommands) {
      const type = draggedRow.data as Type;

      // Handle parent reassignment
      if (targetParentId !== undefined) {
        // Dropped onto another type - set as parent
        if (type.parent?.guid !== targetParentId) {
          kitCommands.updateType("semio.sketchpad.app.kit.canvas.table.setTypeParent", type.guid, { parent: { guid: targetParentId } });
        }
      } else if (targetFolderId === undefined && (type.parent || type.folder)) {
        // Dropped on root - unparent and remove from folder
        kitCommands.updateType("semio.sketchpad.app.kit.canvas.table.unparentType", type.guid, { parent: undefined });
        if (type.folder) {
          kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveTypeToRoot", "type", type.guid, null);
        }
      } else if (!type.parent) {
        // Root type (prototype) - can be moved to folders or root
        kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveTypeToFolder", "type", type.guid, targetFolderId ?? null);
      }
    } else if (draggedRow.kind === "qualities" && kitCommands) {
      const quality = draggedRow.data as Quality;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveQualityToFolder", "quality", quality.guid, targetFolderId ?? null);
    } else if (draggedRow.kind === "files" && kitCommands) {
      const file = draggedRow.data as SemioFile;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveFileToFolder", "file", file.guid, targetFolderId ?? null);
    } else if (draggedRow.kind === "folders" && kitCommands) {
      const folder = draggedRow.data as Folder;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveFolderToFolder", "folder", folder.guid, targetFolderId ?? null);
    }

    // Expand the target folder if moving into a folder
    if (shouldExpandFolder && targetFolderId) {
      const folderId = `folder-${targetFolderId}`;
      if (!expandedRows.has(folderId)) {
        kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.expandFolder", folderId);
      }
    }

    // Expand the target parent if setting a parent
    if (shouldExpandParent && targetParentId) {
      const parentRowId = draggedRow.kind === "designs" ? `design-${targetParentId}` : `type-${targetParentId}`;
      if (!expandedRows.has(parentRowId)) {
        kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.expandParent", parentRowId);
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
        if (kitCommands) kitCommands.createDesign("semio.sketchpad.app.kit.canvas.table.createDesign", newDesign);
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
        if (kitCommands) kitCommands.createType("semio.sketchpad.app.kit.canvas.table.createType", newType);
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
        if (kitCommands) kitCommands.createQuality("semio.sketchpad.app.kit.canvas.table.createQuality", newQuality);
        sketchpadCommands.navigateToQuality(kit.guid, newQuality.guid);
        break;
      }
      case "interfaces": {
        const existingNames = (kit.interfaces || []).map((i: Interface) => i.name);
        const uniqueName = generateUniqueName("New Interface", existingNames);
        const newInterface: Interface = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createInterface("semio.sketchpad.app.kit.canvas.table.createInterface", newInterface);
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
        if (kitCommands) kitCommands.createFolder("semio.sketchpad.app.kit.canvas.table.createFolder", newFolder);
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
      if (kitCommands) kitCommands.createDesign("semio.sketchpad.app.kit.canvas.table.createChild", newDesign);
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
      if (kitCommands) kitCommands.createType("semio.sketchpad.app.kit.canvas.table.createChild", newType);
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
      const selectedByKind = {
        types: [] as Guid[],
        designs: [] as Guid[],
        qualities: [] as string[],
        interfaces: [] as Guid[],
        files: [] as string[],
        folders: [] as Guid[],
        authors: [] as string[],
      };

      rangeRows.forEach((r) => {
        if (r.kind === "types") selectedByKind.types.push((r.data as Type).guid);
        else if (r.kind === "designs") selectedByKind.designs.push((r.data as Design).guid);
        else if (r.kind === "qualities") selectedByKind.qualities.push((r.data as Quality).key);
        else if (r.kind === "interfaces") selectedByKind.interfaces.push((r.data as Interface).guid);
        else if (r.kind === "files") selectedByKind.files.push((r.data as SemioFile).guid);
        else if (r.kind === "folders") selectedByKind.folders.push((r.data as Folder).guid);
        else if (r.kind === "authors") selectedByKind.authors.push((r.data as Author).name);
      });

      // Select all items at once
      if (selectedByKind.types.length > 0) kitAppCommands.selectTypes("semio.sketchpad.app.kit.canvas.table.selectTypesRange", selectedByKind.types);
      if (selectedByKind.designs.length > 0) kitAppCommands.selectDesigns("semio.sketchpad.app.kit.canvas.table.selectDesignsRange", selectedByKind.designs);
      if (selectedByKind.qualities.length > 0) kitAppCommands.selectQualities("semio.sketchpad.app.kit.canvas.table.selectQualitiesRange", selectedByKind.qualities);
      if (selectedByKind.interfaces.length > 0) kitAppCommands.selectInterfaces("semio.sketchpad.app.kit.canvas.table.selectInterfacesRange", selectedByKind.interfaces);
      if (selectedByKind.files.length > 0) kitAppCommands.selectFiles("semio.sketchpad.app.kit.canvas.table.selectFilesRange", selectedByKind.files);
      if (selectedByKind.folders.length > 0) kitAppCommands.selectFolders("semio.sketchpad.app.kit.canvas.table.selectFoldersRange", selectedByKind.folders);
      if (selectedByKind.authors.length > 0) kitAppCommands.selectAuthors("semio.sketchpad.app.kit.canvas.table.selectAuthorsRange", selectedByKind.authors);

      lastClickedIndexRef.current = index;
      return;
    }

    // Handle ctrl/cmd multi-select
    if (e.metaKey || e.ctrlKey) {
      if (row.kind === "designs") {
        const designId = (row.data as Design).guid;
        if (selection.designs.includes(designId)) {
          kitAppCommands.removeDesignFromSelection("semio.sketchpad.app.kit.canvas.table.removeDesignCtrl", designId);
        } else {
          kitAppCommands.addDesignToSelection("semio.sketchpad.app.kit.canvas.table.addDesignCtrl", designId);
        }
      } else if (row.kind === "types") {
        const typeId = (row.data as Type).guid;
        if (selection.types.includes(typeId)) {
          kitAppCommands.removeTypeFromSelection("semio.sketchpad.app.kit.canvas.table.removeTypeCtrl", typeId);
        } else {
          kitAppCommands.addTypeToSelection("semio.sketchpad.app.kit.canvas.table.addTypeCtrl", typeId);
        }
      } else if (row.kind === "qualities") {
        const qualityKey = (row.data as Quality).key;
        if (selection.qualities.includes(qualityKey)) {
          kitAppCommands.removeQualityFromSelection("semio.sketchpad.app.kit.canvas.table.removeQualityCtrl", qualityKey);
        } else {
          kitAppCommands.addQualityToSelection("semio.sketchpad.app.kit.canvas.table.addQualityCtrl", qualityKey);
        }
      } else if (row.kind === "interfaces") {
        const interfaceId = (row.data as Interface).guid;
        if (selection.interfaces && selection.interfaces.includes(interfaceId)) {
          kitAppCommands.removeInterfaceFromSelection("semio.sketchpad.app.kit.canvas.table.removeInterfaceCtrl", interfaceId);
        } else {
          kitAppCommands.addInterfaceToSelection("semio.sketchpad.app.kit.canvas.table.addInterfaceCtrl", interfaceId);
        }
      } else if (row.kind === "files") {
        const fileGuid = (row.data as SemioFile).guid;
        if (selection.files.includes(fileGuid)) {
          kitAppCommands.removeFileFromSelection("semio.sketchpad.app.kit.canvas.table.removeFileCtrl", fileGuid);
        } else {
          kitAppCommands.addFileToSelection("semio.sketchpad.app.kit.canvas.table.addFileCtrl", fileGuid);
        }
      } else if (row.kind === "folders") {
        const folderId = (row.data as Folder).guid;
        if (selection.folders && selection.folders.includes(folderId)) {
          kitAppCommands.removeFolderFromSelection("semio.sketchpad.app.kit.canvas.table.removeFolderCtrl", folderId);
        } else {
          kitAppCommands.addFolderToSelection("semio.sketchpad.app.kit.canvas.table.addFolderCtrl", folderId);
        }
      } else if (row.kind === "authors") {
        const authorName = (row.data as Author).name;
        if (selection.authors.includes(authorName)) {
          kitAppCommands.removeAuthorFromSelection("semio.sketchpad.app.kit.canvas.table.removeAuthorCtrl", authorName);
        } else {
          kitAppCommands.addAuthorToSelection("semio.sketchpad.app.kit.canvas.table.addAuthorCtrl", authorName);
        }
      }
      // Don't update lastClickedIndexRef for ctrl/cmd clicks
      return;
    }

    // Handle normal single selection with delay to detect double-click
    clickTimerRef.current = setTimeout(() => {
      if (row.kind === "designs") {
        kitAppCommands.selectDesign("semio.sketchpad.app.kit.canvas.table.selectDesign", (row.data as Design).guid);
      } else if (row.kind === "types") {
        kitAppCommands.selectType("semio.sketchpad.app.kit.canvas.table.selectType", (row.data as Type).guid);
      } else if (row.kind === "qualities") {
        kitAppCommands.selectQuality("semio.sketchpad.app.kit.canvas.table.selectQuality", (row.data as Quality).key);
      } else if (row.kind === "interfaces") {
        kitAppCommands.selectInterface("semio.sketchpad.app.kit.canvas.table.selectInterface", (row.data as Interface).guid);
      } else if (row.kind === "files") {
        kitAppCommands.selectFile("semio.sketchpad.app.kit.canvas.table.selectFile", (row.data as SemioFile).guid);
      } else if (row.kind === "folders") {
        kitAppCommands.selectFolder("semio.sketchpad.app.kit.canvas.table.selectFolder", (row.data as Folder).guid);
      } else if (row.kind === "authors") {
        kitAppCommands.selectAuthor("semio.sketchpad.app.kit.canvas.table.selectAuthor", (row.data as Author).name);
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
    kitAppCommands.toggleSort("semio.sketchpad.app.kit.canvas.table.toggleSort", column);
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

    for (const file of files) {
      // Check if file is a zip file
      if (file.name.toLowerCase().endsWith(".zip")) {
        try {
          // Import JSZip dynamically
          const JSZip = (await import("jszip")).default;
          const zip = await JSZip.loadAsync(file);

          // Extract all files from zip
          const folderByGuid = new Map<string, Folder>();
          (kit.folders || []).forEach((existingFolder) => folderByGuid.set(existingFolder.guid, existingFolder));
          const folderPathCache = new Map<string, string>();
          const folderPathMap = new Map<string, string>();
          const resolvePath = (folder: Folder): string => {
            const cached = folderPathCache.get(folder.guid);
            if (cached) return cached;
            const parentFolder = folder.parent ? folderByGuid.get(folder.parent.guid) : undefined;
            const path = parentFolder ? `${resolvePath(parentFolder)}/${folder.name}` : folder.name;
            folderPathCache.set(folder.guid, path);
            return path;
          };
          folderByGuid.forEach((folder) => {
            const path = resolvePath(folder);
            folderPathMap.set(path, folder.guid);
          });
          const ensureFolder = async (parts: string[]): Promise<string | undefined> => {
            let parentGuid: string | undefined = undefined;
            let currentPath = "";
            for (const part of parts) {
              currentPath = currentPath ? `${currentPath}/${part}` : part;
              let folderGuid = folderPathMap.get(currentPath);
              if (!folderGuid) {
                const newFolder: Folder = {
                  guid: guid(),
                  name: part,
                  parent: parentGuid ? { guid: parentGuid } : undefined,
                  createdAt: new Date(),
                  updatedAt: new Date(),
                };
                folderGuid = newFolder.guid;
                folderPathMap.set(currentPath, folderGuid);
                folderByGuid.set(folderGuid, newFolder);
                if (kitCommands) await kitCommands.createFolder("semio.sketchpad.app.kit.dropZip.createFolder", newFolder);
              }
              parentGuid = folderGuid;
            }
            return parentGuid;
          };
          let processedFiles = 0;
          for (const zipEntry of Object.values(zip.files)) {
            if (!zipEntry.dir) {
              const relativePath = zipEntry.name;
              const parts = relativePath.split("/").filter((part) => part.length > 0);
              const directories = parts.slice(0, -1);
              const parentFolderGuid = directories.length > 0 ? await ensureFolder(directories) : undefined;
              const fileBlob = await zipEntry.async("blob");
              const extractedFile: SemioFile = {
                guid: guid(),
                name: parts[parts.length - 1] || relativePath,
                folder: parentFolderGuid ? { guid: parentFolderGuid } : undefined,
                size: fileBlob.size,
                hash: undefined,
                createdAt: new Date(),
                updatedAt: new Date(),
              };
              await kitCommands?.addFile("semio.sketchpad.app.kit.dropZip", extractedFile, fileBlob);
              processedFiles += 1;
            }
          }
        } catch (error) {
          console.error(`Failed to extract zip file ${file.name}:`, error);
        }
      } else {
        // Handle regular file
        const newFile: SemioFile = {
          guid: guid(),
          name: file.name,
          size: file.size,
          hash: undefined,
          createdAt: new Date(),
          updatedAt: new Date(),
        };

        try {
          await kitCommands?.addFile("semio.sketchpad.app.kit.dropFile", newFile, file);
        } catch (error) {
          console.error(`Failed to add file ${file.name}:`, error);
        }
      }
    }
  };

  if (isMobile) {
    return (
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            kitAppCommands.deselectAll("semio.sketchpad.app.kit.canvas.table.deselect");
          }
        }}
      >
        <Strip
          direction="horizontal"
          id="semio.sketchpad.app.kit.filter.strip"
          items={[
            ...(selectedKind
              ? [
                  <Toggle
                    kind="withAction"
                    pressed={true}
                    onPressedChange={() => toggleKind(selectedKind)}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact(selectedKind)}
                    id="semio.sketchpad.app.kit.kitApp.hideKind"
                    actionId="semio.sketchpad.app.kit.kitApp.createArtifact"
                    icon={
                      <>
                        {selectedKind === "designs" && <LayoutIcon />}
                        {selectedKind === "types" && <TypeIcon />}
                        {selectedKind === "qualities" && <AwardIcon />}
                        {selectedKind === "files" && <DocumentIcon />}
                        {selectedKind === "folders" && <FolderIcon />}
                        {selectedKind === "authors" && <UserIcon />}
                      </>
                    }
                  />,
                ]
              : [
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("designs")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("designs")}
                    id="semio.sketchpad.app.kit.kitApp.showDesigns"
                    actionId="semio.sketchpad.app.kit.kitApp.createDesign"
                    icon={<LayoutIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("types")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("types")}
                    id="semio.sketchpad.app.kit.kitApp.showTypes"
                    actionId="semio.sketchpad.app.kit.kitApp.createType"
                    icon={<TypeIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("qualities")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("qualities")}
                    id="semio.sketchpad.app.kit.kitApp.showQualities"
                    actionId="semio.sketchpad.app.kit.kitApp.createQuality"
                    icon={<AwardIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("interfaces")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("interfaces")}
                    id="semio.sketchpad.app.kit.kitApp.showInterfaces"
                    actionId="semio.sketchpad.app.kit.kitApp.createInterface"
                    icon={<InterfaceIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("files")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("files")}
                    id="semio.sketchpad.app.kit.kitApp.showFiles"
                    actionId="semio.sketchpad.app.kit.kitApp.createFile"
                    icon={<DocumentIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("folders")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("folders")}
                    id="semio.sketchpad.app.kit.kitApp.showFolders"
                    actionId="semio.sketchpad.app.kit.kitApp.createFolder"
                    icon={<FolderIcon />}
                  />,
                  <Toggle
                    kind="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("authors")}
                    actionIcon={<AddIcon />}
                    onActionClick={() => handleCreateArtifact("authors")}
                    id="semio.sketchpad.app.kit.kitApp.showAuthors"
                    actionId="semio.sketchpad.app.kit.kitApp.createAuthor"
                    icon={<UserIcon />}
                  />,
                ]),
            ...(selectedName ? [<Toggle id="semio.sketchpad.app.kit.filter.name.hide" pressed={true} onPressedChange={() => toggleName(selectedName)} icon={selectedName} />] : []),
            ...(selectedKind && !selectedName && uniqueNames.length > 0 ? uniqueNames.map((name) => <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)} id="semio.sketchpad.app.kit.filter.name" icon={name} />) : []),
            <Input
              key="search"
              id="semio.sketchpad.app.kit.filter.search"
              className="flex-1 min-w-[160px]"
              placeholder={useLabel("semio.sketchpad.common.search")}
              value={searchQuery}
              onChange={(e) => kitAppCommands.setFilterSearch("semio.sketchpad.app.kit.filter.search", e.target.value)}
            />,
          ]}
        />
        <ConceptFilter allConcepts={allConcepts} paramName="c" />

        {/* Mobile table using general Table component */}
        <Table
          className="flex-1 min-h-0"
          columns={[
            {
              id: "artifact",
              header: (
                <div className="flex items-center justify-between w-full">
                  <span>{useLabel("semio.sketchpad.app.kit.canvas.table.header.artifact")}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.artifact.sortColumn", "artifact");
                      kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.artifact.sortDirection", value as "asc" | "desc");
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
                <div className="flex items-center gap-single justify-between" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                  <div className="flex items-center gap-single flex-1 min-w-0">
                    {row.hasChildren ? (
                      <Action
                        level="base"
                        onClick={(e) => {
                          e.stopPropagation();
                          kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.toggleRow", row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span className="size-small shrink-0" />
                    )}
                    <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
                    <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
                  </div>
                  <div className="flex items-center gap-single shrink-0">
                    {(row.kind === "designs" || row.kind === "types") && (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCreateChildForRow(row);
                        }}
                        id="semio.sketchpad.app.kit.kitApp.createChild"
                        level="base"
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
          kitAppCommands.deselectAll("semio.sketchpad.app.kit.canvas.table.deselect");
        }
      }}
    >
      <Strip
        direction="horizontal"
        id="semio.sketchpad.app.kit.filter.strip"
        items={[
          ...(selectedKind
            ? [
                <Toggle
                  kind="withAction"
                  pressed={true}
                  onPressedChange={() => toggleKind(selectedKind)}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact(selectedKind)}
                  id="semio.sketchpad.app.kit.kitApp.hideKind"
                  actionId="semio.sketchpad.app.kit.kitApp.createArtifact"
                  icon={
                    <>
                      {selectedKind === "designs" && <LayoutIcon />}
                      {selectedKind === "types" && <TypeIcon />}
                      {selectedKind === "qualities" && <AwardIcon />}
                      {selectedKind === "files" && <DocumentIcon />}
                      {selectedKind === "folders" && <FolderIcon />}
                      {selectedKind === "authors" && <UserIcon />}
                    </>
                  }
                />,
              ]
            : [
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("designs")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("designs")}
                  id="semio.sketchpad.app.kit.kitApp.showDesigns"
                  actionId="semio.sketchpad.app.kit.kitApp.createDesign"
                  icon={<LayoutIcon />}
                />,
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("types")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("types")}
                  id="semio.sketchpad.app.kit.kitApp.showTypes"
                  actionId="semio.sketchpad.app.kit.kitApp.createType"
                  icon={<TypeIcon />}
                />,
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("qualities")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("qualities")}
                  id="semio.sketchpad.app.kit.kitApp.showQualities"
                  actionId="semio.sketchpad.app.kit.kitApp.createQuality"
                  icon={<AwardIcon />}
                />,
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("files")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("files")}
                  id="semio.sketchpad.app.kit.kitApp.showFiles"
                  actionId="semio.sketchpad.app.kit.kitApp.createFile"
                  icon={<DocumentIcon />}
                />,
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("folders")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("folders")}
                  id="semio.sketchpad.app.kit.kitApp.showFolders"
                  actionId="semio.sketchpad.app.kit.kitApp.createFolder"
                  icon={<FolderIcon />}
                />,
                <Toggle
                  kind="withAction"
                  pressed={false}
                  onPressedChange={() => toggleKind("authors")}
                  actionIcon={<AddIcon />}
                  onActionClick={() => handleCreateArtifact("authors")}
                  id="semio.sketchpad.app.kit.kitApp.showAuthors"
                  actionId="semio.sketchpad.app.kit.kitApp.createAuthor"
                  icon={<UserIcon />}
                />,
              ]),
          ...(selectedName ? [<Toggle id="semio.sketchpad.app.kit.filter.name.hide" pressed={true} onPressedChange={() => toggleName(selectedName)} icon={selectedName} />] : []),
          ...(selectedKind && !selectedName && uniqueNames.length > 0 ? uniqueNames.map((name) => <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)} id="semio.sketchpad.app.kit.filter.name" icon={name} />) : []),
          <Input
            key="search"
            id="semio.sketchpad.app.kit.canvas.table.search"
            className="flex-1 min-w-[200px]"
            placeholder={useLabel("semio.sketchpad.common.search")}
            value={searchQuery}
            onChange={(e) => kitAppCommands.setFilterSearch("semio.sketchpad.app.kit.canvas.table.search", e.target.value)}
          />,
        ]}
      />
      <ConceptFilter allConcepts={allConcepts} paramName="c" />
      <Scrollable ref={scrollAreaRef} className="flex-1 min-h-0" onDragOver={handleFileDragOver} onDragLeave={handleFileDragLeave} onDrop={handleFileDrop}>
        {isDragOver && (
          <div className="absolute inset-0 bg-active-base/50 border-2 border-dashed border-active-foreground flex items-center justify-center z-panel">
            <div className="text-active-foreground text-lg font-medium">Drop files to add to kit</div>
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
                        <span>{useLabel("semio.sketchpad.app.kit.canvas.table.header.kind")}</span>
                        <Toggle
                          kind="dropdown"
                          pressed={sortColumn === "kind"}
                          value={sortColumn === "kind" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.kind.sortColumn", "kind");
                            kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.kind.sortDirection", value as "asc" | "desc");
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
                  <span>{useLabel("semio.sketchpad.app.kit.canvas.table.header.artifact")}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.artifact.sortColumn", "artifact");
                      kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.artifact.sortDirection", value as "asc" | "desc");
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
                <div className="flex items-center gap-single justify-between" style={{ paddingLeft: `calc(${row.level} * var(--size-small))` }}>
                  <div className="flex items-center gap-single flex-1 min-w-0">
                    {row.hasChildren ? (
                      <Action
                        level="base"
                        onClick={(e) => {
                          e.stopPropagation();
                          kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.toggleRow", row.id);
                        }}
                        icon={row.isExpanded ? <ChevronDownIcon /> : <ChevronRightIcon />}
                      />
                    ) : (
                      <span className="size-small shrink-0" />
                    )}
                    <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
                    <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
                  </div>
                  <div className="flex items-center gap-single shrink-0">
                    {(row.kind === "designs" || row.kind === "types") && (
                      <Action
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCreateChildForRow(row);
                        }}
                        id="semio.sketchpad.app.kit.kitApp.createChild"
                        level="base"
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
                  <span>{useLabel("semio.sketchpad.app.kit.canvas.table.header.updatedAt")}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "updatedAt"}
                    value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.updatedAt.sortColumn", "updatedAt");
                      kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.updatedAt.sortDirection", value as "asc" | "desc");
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
                  <span>{useLabel("semio.sketchpad.app.kit.canvas.table.header.createdAt")}</span>
                  <Toggle
                    kind="dropdown"
                    pressed={sortColumn === "createdAt"}
                    value={sortColumn === "createdAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.createdAt.sortColumn", "createdAt");
                      kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.createdAt.sortDirection", value as "asc" | "desc");
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

const App: FC = () => {
  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">Failed to load kit app</p>
        </div>
      }
    >
      <KitDropZone>
        <Canvas>
          <Window id="kit-table">
            <AppContent />
          </Window>
        </Canvas>
      </KitDropZone>
    </ErrorBoundary>
  );
};

export default App;

// #endregion Table

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
    const kitStore = useKitStore() as any;
    return (
      <>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.name"
              value={kit.name}
              onLazyChange={(value) => kitStore.change({ name: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.name")}
              showLabel
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.kit.panel.details.section.kit.version"
              value={kit.version || ""}
              placeholder={useLabel("semio.sketchpad.app.kit.versionPlaceholder.label")}
              onLazyChange={(value) => kitStore.change({ version: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.version")}
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
              onLazyChange={(value) => kitStore.change({ description: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.description")}
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
              onLazyChange={(value) => kitStore.change({ icon: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.icon")}
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
              onLazyChange={(value) => kitStore.change({ image: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.image")}
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
              onLazyChange={(value) => kitStore.change({ homepage: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.homepage")}
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
              onLazyChange={(value) => kitStore.change({ license: value })}
              transaction={useKitAppTransaction("semio.sketchpad.app.kit.panel.details.section.kit.license")}
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
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const selectedTypes = selection?.types || [];
  if (selectedTypes.length === 0) return null;
  if (selectedTypes.length === 1) return <SingleTypeSection typeGuid={selectedTypes[0]} />;
  return <MultipleTypesSection typeGuids={selectedTypes} />;
};

const SingleTypeSection: FC<{ typeGuid: string }> = ({ typeGuid }) => {
  const { t } = useTranslation();
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
          <Input id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.type.panel.details.section.type.description" value={type.description || ""} placeholder={useLabel("semio.sketchpad.app.type.descriptionPlaceholder.label")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
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
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
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

export const DesignSection: FC = () => {
  const { t } = useTranslation();
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const selectedDesigns = selection?.designs || [];
  if (selectedDesigns.length === 0) return null;
  if (selectedDesigns.length === 1) return <SingleDesignSection designGuid={selectedDesigns[0]} />;
  return <MultipleDesignsSection designGuids={selectedDesigns} />;
};

const SingleDesignSection: FC<{ designGuid: string }> = ({ designGuid }) => {
  const { t } = useTranslation();
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
          <Input id="semio.sketchpad.app.design.panel.details.section.design.name" value={design.name} readOnly showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea id="semio.sketchpad.app.design.panel.details.section.design.description" value={design.description || ""} placeholder={useLabel("semio.sketchpad.app.design.descriptionPlaceholder")} readOnly showLabel />
        </TreeContent>
      </TreeItem>
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
  const kitApp = useKitApp() as KitAppState;
  const kit = useKit() as Kit;
  const selection = kitApp?.selection;
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

  const formatDate = (date?: Date) => {
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
  const kitApp = useKitApp() as KitAppState;
  const kit = useKit() as Kit;
  const kitStore = useKitStore() as any;
  const { startTransaction, finalizeTransaction, abortTransaction } = useKitAppCommands();
  const selection = kitApp?.selection;
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

  const formatDate = (date?: Date) => {
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
              const folderStore = (kitStore as any).folder(folder.guid);
              folderStore.change({ name: value });
            }}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.name")}
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
                const folderStore = (kitStore as any).folder(folder.guid);
                folderStore.change({ description: value });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.kit.panel.details.section.folder.description")}
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
  const kitApp = useKitApp() as KitAppState;
  const selection = kitApp?.selection;
  const typesCount = selection?.types?.length || 0;
  const designsCount = selection?.designs?.length || 0;
  const qualitiesCount = selection?.qualities?.length || 0;
  const filesCount = selection?.files?.length || 0;
  const authorsCount = selection?.authors?.length || 0;
  const kinds: string[] = [];
  if (typesCount > 0) kinds.push(t("semio.sketchpad.app.kit.types.multipleTitle", { count: typesCount }));
  if (designsCount > 0) kinds.push(t("semio.sketchpad.app.kit.designs.multipleTitle", { count: designsCount }));
  if (qualitiesCount > 0) kinds.push(t("semio.sketchpad.app.kit.qualities.multipleTitle", { count: qualitiesCount }));
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
  component: App,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
  ],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 2 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]);
  },
  order: 10,
};

// #endregion Config
