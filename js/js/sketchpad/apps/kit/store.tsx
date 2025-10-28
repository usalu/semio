// #region Header

// store.tsx

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

import { Camera } from "three";
import * as Y from "yjs";
import { areSameKit, Coord, Design, DesignDiff, DiffStatus, Guid, KitDiff, Type, TypeDiff } from "../../../semio";
import { KitCommandContext, KitStore, useKitScope } from "../../kits/store";
import { identitySelector, KitAppId, KitDiffAppEdit, KitDiffAppStore, PanelVisibility, registerKitAppStoreFactory, SketchpadStore, useSketchpadStore, useSync, useSyncDeep, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "../../store";
import { commands as kitAppCommands } from "./commands";

type YKitAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitApp = Y.Map<YKitAppVal>;
type YKitApps = Y.Map<YKitApp>;

export interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  files?: string[];
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
export interface KitAppSelectionFilesDiff {
  added?: string[];
  removed?: string[];
}
export interface KitAppSelectionAuthorsDiff {
  added?: string[];
  removed?: string[];
}
export interface KitAppSelectionDiff {
  types?: KitAppSelectionTypesDiff;
  designs?: KitAppSelectionDesignsDiff;
  qualities?: KitAppSelectionQualitiesDiff;
  files?: KitAppSelectionFilesDiff;
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
export const areSameKitApp = (kitApp: KitAppId, other: KitAppId): boolean => areSameKit(kitApp.kit, other.kit);
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
    selection.set("types", selectedTypes);
    selection.set("designs", selectedDesigns);
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

    Object.entries(kitAppCommands).forEach(([commandId, command]) => {
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
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
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
        yExpandedRows.delete(0, yExpandedRows.length);
        yExpandedRows.push(diff.expandedRows);
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

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.kitApp.startTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.kitApp.finalizeTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitApp.abortTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitApp.undo") {
      console.group(`Executing (special) command: "${command}"`);
      this.undo();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitApp.redo") {
      console.group(`Executing (special) command: "${command}"`);
      this.redo();
      console.groupEnd();
      return {} as T;
    }

    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit app store`);

    const kitStore = this.kit();
    const state = this.snapshot();

    const context: KitAppCommandContext = {
      kitApp: state,
      kit: kitStore.snapshot(),
      fileUrls: kitStore.fileUrls,
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

// Register the factory - deferred to avoid circular dependency issues
export function initializeKitAppStore() {
  registerKitAppStoreFactory((parent, yMap, transact, id, state) => new KitAppStore(parent, yMap, transact, id, state));
}

// Auto-initialize if this module is imported
if (typeof window !== 'undefined') {
  setTimeout(() => initializeKitAppStore(), 0);
}

function useKitAppStore<T>(selector?: (store: KitAppStore) => T, id?: KitAppId): T | KitAppStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) {
    return null;
  }
  try {
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
  const result = useSyncDeep<KitAppState, T>(store as KitAppStore, selector ? selector : identitySelector);
  return result;
}

export function useKitAppSelection(id?: KitAppId): KitAppSelection {
  const store = useKitAppStore(identitySelector, id) as KitAppStore | null;
  if (!store) return emptyKitAppSelection;
  return (useSync<KitAppState, KitAppSelection>(store, (state) => state.selection ?? emptyKitAppSelection, true) as KitAppSelection) ?? emptyKitAppSelection;
}

export function useKitAppFullscreen(): KitAppFullscreenWindow {
  return useKitApp((s) => s.fullscreenWindow) as KitAppFullscreenWindow;
}

export function useKitAppOthers(): KitAppPresenceOther[] {
  return useKitApp((s) => s.others) as KitAppPresenceOther[];
}

export function useKitAppCommands(id?: KitAppId) {
  const store = useKitAppStore(undefined, id) as KitAppStore | null;
  const noOp = () => {};
  if (!store) {
    return {
      startTransaction: noOp,
      finalizeTransaction: noOp,
      abortTransaction: noOp,
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
      selectFile: noOp,
      selectFiles: noOp,
      addFileToSelection: noOp,
      removeFileFromSelection: noOp,
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
    startTransaction: () => store.execute("semio.kitApp.startTransaction"),
    finalizeTransaction: () => store.execute("semio.kitApp.finalizeTransaction"),
    abortTransaction: () => store.execute("semio.kitApp.abortTransaction"),
    undo: () => store.execute("semio.kitApp.undo"),
    redo: () => store.execute("semio.kitApp.redo"),
    selectAll: () => store.execute("semio.kitApp.selectAll"),
    deselectAll: () => store.execute("semio.kitApp.deselectAll"),
    selectType: (Guid: Guid) => store.execute("semio.kitApp.selectType", Guid),
    selectTypes: (typeIds: Guid[]) => store.execute("semio.kitApp.selectTypes", typeIds),
    addTypeToSelection: (Guid: Guid) => store.execute("semio.kitApp.addTypeToSelection", Guid),
    removeTypeFromSelection: (Guid: Guid) => store.execute("semio.kitApp.removeTypeFromSelection", Guid),
    selectDesign: (Guid: Guid) => store.execute("semio.kitApp.selectDesign", Guid),
    selectDesigns: (designIds: Guid[]) => store.execute("semio.kitApp.selectDesigns", designIds),
    addDesignToSelection: (Guid: Guid) => store.execute("semio.kitApp.addDesignToSelection", Guid),
    removeDesignFromSelection: (Guid: Guid) => store.execute("semio.kitApp.removeDesignFromSelection", Guid),
    selectQuality: (key: string) => store.execute("semio.kitApp.selectQuality", key),
    selectQualities: (keys: string[]) => store.execute("semio.kitApp.selectQualities", keys),
    addQualityToSelection: (key: string) => store.execute("semio.kitApp.addQualityToSelection", key),
    removeQualityFromSelection: (key: string) => store.execute("semio.kitApp.removeQualityFromSelection", key),
    selectFile: (path: string) => store.execute("semio.kitApp.selectFile", path),
    selectFiles: (paths: string[]) => store.execute("semio.kitApp.selectFiles", paths),
    addFileToSelection: (path: string) => store.execute("semio.kitApp.addFileToSelection", path),
    removeFileFromSelection: (path: string) => store.execute("semio.kitApp.removeFileFromSelection", path),
    selectAuthor: (name: string) => store.execute("semio.kitApp.selectAuthor", name),
    selectAuthors: (names: string[]) => store.execute("semio.kitApp.selectAuthors", names),
    addAuthorToSelection: (name: string) => store.execute("semio.kitApp.addAuthorToSelection", name),
    removeAuthorFromSelection: (name: string) => store.execute("semio.kitApp.removeAuthorFromSelection", name),
    deleteSelected: () => store.execute("semio.kitApp.deleteSelected"),
    toggleTypesFullscreen: () => store.execute("semio.kitApp.toggleTypesFullscreen"),
    toggleDesignsFullscreen: () => store.execute("semio.kitApp.toggleDesignsFullscreen"),
    addType: (type: Type) => store.execute("semio.kitApp.addType", type),
    addTypes: (types: Type[]) => store.execute("semio.kitApp.addTypes", types),
    removeType: (Guid: Guid) => store.execute("semio.kitApp.removeType", Guid),
    removeTypes: (typeIds: Guid[]) => store.execute("semio.kitApp.removeTypes", typeIds),
    addDesign: (design: Design) => store.execute("semio.kitApp.addDesign", design),
    addDesigns: (designs: Design[]) => store.execute("semio.kitApp.addDesigns", designs),
    removeDesign: (Guid: Guid) => store.execute("semio.kitApp.removeDesign", Guid),
    removeDesigns: (designIds: Guid[]) => store.execute("semio.kitApp.removeDesigns", designIds),
    updateType: (Guid: Guid, typeDiff: TypeDiff) => store.execute("semio.kitApp.updateType", Guid, typeDiff),
    updateTypes: (updates: { id: Guid; diff: TypeDiff }[]) => store.execute("semio.kitApp.updateTypes", updates),
    updateDesign: (Guid: Guid, designDiff: DesignDiff) => store.execute("semio.kitApp.updateDesign", Guid, designDiff),
    updateDesigns: (updates: { id: Guid; diff: DesignDiff }[]) => store.execute("semio.kitApp.updateDesigns", updates),
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setFilterSearch: (search: string) => store.execute("semio.kitApp.setFilterSearch", search),
    setExpandedRows: (rows: string[]) => store.execute("semio.kitApp.setExpandedRows", rows),
    toggleExpandedRow: (rowId: string) => store.execute("semio.kitApp.toggleExpandedRow", rowId),
    setSortColumn: (column: KitAppSortColumn) => store.execute("semio.kitApp.setSortColumn", column),
    setSortDirection: (direction: KitAppSortDirection) => store.execute("semio.kitApp.setSortDirection", direction),
    toggleSort: (column: KitAppSortColumn) => store.execute("semio.kitApp.toggleSort", column),
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
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

  return useSync<KitAppState, DiffStatus>(
    store,
    (state) => {
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
    },
    true,
  ) as DiffStatus;
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

  return useSync<KitAppState, DiffStatus>(
    store,
    (state) => {
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
    },
    true,
  ) as DiffStatus;
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
