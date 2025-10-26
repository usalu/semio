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
import { identitySelector, KitDiffEditorEdit, KitDiffEditorStore, PanelVisibility, registerKitEditorStoreFactory, SketchpadStore, useSketchpadStore, useSync, useSyncDeep, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "../../store";
import { commands as kitEditorCommands } from "./commands";

type YKitEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitEditor = Y.Map<YKitEditorVal>;
type YKitEditors = Y.Map<YKitEditor>;

export interface KitEditorId {
  kit: Guid;
}
export interface KitEditorSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: string[];
  files?: string[];
  authors?: string[];
}
const emptyKitEditorSelection: KitEditorSelection = {};
export interface KitEditorSelectionTypesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitEditorSelectionDesignsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface KitEditorSelectionQualitiesDiff {
  added?: string[];
  removed?: string[];
}
export interface KitEditorSelectionFilesDiff {
  added?: string[];
  removed?: string[];
}
export interface KitEditorSelectionAuthorsDiff {
  added?: string[];
  removed?: string[];
}
export interface KitEditorSelectionDiff {
  types?: KitEditorSelectionTypesDiff;
  designs?: KitEditorSelectionDesignsDiff;
  qualities?: KitEditorSelectionQualitiesDiff;
  files?: KitEditorSelectionFilesDiff;
  authors?: KitEditorSelectionAuthorsDiff;
}
export enum KitEditorFullscreenWindow {
  None = "none",
  Types = "types",
  Designs = "designs",
}
export interface KitEditorPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface KitEditorHover {
  type?: Guid;
  design?: Guid;
}
export interface KitEditorPresenceOther extends KitEditorPresence {
  name: string;
}
export type KitEditorSortColumn = "artifact" | "kind" | "authors" | "updatedAt" | "createdAt";
export type KitEditorSortDirection = "asc" | "desc";

export interface KitEditorDiff {
  selection?: KitEditorSelectionDiff;
  presence?: KitEditorPresence;
  hover?: KitEditorHover;
  fullscreenWindow?: KitEditorFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  filterSearch?: string;
  expandedRows?: string[];
  sortColumn?: KitEditorSortColumn;
  sortDirection?: KitEditorSortDirection;
}
export interface KitEditorEdit extends KitDiffEditorEdit<KitEditorSelectionDiff> {}
export interface KitEditorState {
  fullscreenWindow: KitEditorFullscreenWindow;
  panelVisibility: PanelVisibility;
  selection?: KitEditorSelection;
  hover?: KitEditorHover;
  presence?: KitEditorPresence;
  others: KitEditorPresenceOther[];
  filterSearch: string;
  expandedRows: string[];
  sortColumn?: KitEditorSortColumn;
  sortDirection?: KitEditorSortDirection;
}

export interface KitEditorCommandContext extends KitCommandContext {
  kitEditor: KitEditorState;
}
export interface KitEditorCommandResult {
  diff?: KitEditorDiff;
  kitDiff?: KitDiff;
}

export const inverseKitEditorSelectionDiff = (selection: KitEditorSelection, diff: KitEditorSelectionDiff): KitEditorSelectionDiff => {
  const inverseDiff: KitEditorSelectionDiff = {};

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
export const areSameKitEditor = (kitEditor: KitEditorId, other: KitEditorId): boolean => areSameKit(kitEditor.kit, other.kit);
export const hasSameKitEditor = (kitEditor: KitEditorId, others: KitEditorId[]): boolean => others.some((other) => areSameKitEditor(kitEditor, other));

class KitEditorStore extends KitDiffEditorStore<KitEditorState, KitEditorDiff, KitEditorSelectionDiff, KitEditorEdit, KitEditorCommandContext, KitEditorCommandResult> {
  constructor(parent: SketchpadStore, yMap: YKitEditor, transact: (fn: () => void) => void, id: KitEditorId, state?: KitEditorState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    yMap.set("kit", kit.guid);

    yMap.set("fullscreenWindow", state?.fullscreenWindow || KitEditorFullscreenWindow.None);

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

    Object.entries(kitEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  // KitEditor-specific getters
  get fullscreenWindow(): KitEditorFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as KitEditorFullscreenWindow;
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

  get selection(): KitEditorSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: KitEditorSelection = {};

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

  get presence(): KitEditorPresence {
    return {
      cursor: {
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }

  get others(): KitEditorPresenceOther[] {
    return [];
  }

  get filterSearch(): string {
    return (this.yMap.get("filterSearch") as string) || "";
  }

  get expandedRows(): string[] {
    const yExpandedRows = this.yMap.get("expandedRows") as Y.Array<string>;
    return yExpandedRows ? yExpandedRows.toArray() : [];
  }

  get sortColumn(): KitEditorSortColumn | undefined {
    return this.yMap.get("sortColumn") as KitEditorSortColumn | undefined;
  }

  get sortDirection(): KitEditorSortDirection | undefined {
    return this.yMap.get("sortDirection") as KitEditorSortDirection | undefined;
  }

  kit(): KitStore {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  // Implement abstract methods from Editor base class
  protected getSelection(): KitEditorSelection {
    return this.selection;
  }

  protected hash(state: KitEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): KitEditorState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      canUndo: this.canUndo(),
      canRedo: this.canRedo(),
      presence: this.presence,
      others: this.others,
      // diff: this.diff, // TODO: KitEditorState doesn't have a diff property
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
      filterSearch: this.filterSearch,
      expandedRows: this.expandedRows,
      sortColumn: this.sortColumn,
      sortDirection: this.sortDirection,
    } as any;
  }

  change = (diff: KitEditorDiff) => {
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

  protected inverseSelectionDiff(selection: KitEditorSelection, diff: KitEditorSelectionDiff): KitEditorSelectionDiff {
    return inverseKitEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: KitEditorSelectionDiff): void {
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
    if (command === "semio.kitEditor.startTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.kitEditor.finalizeTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitEditor.abortTransaction") {
      console.group(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitEditor.undo") {
      console.group(`Executing (special) command: "${command}"`);
      this.undo();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.kitEditor.redo") {
      console.group(`Executing (special) command: "${command}"`);
      this.redo();
      console.groupEnd();
      return {} as T;
    }

    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit editor store`);

    const kitStore = this.kit();
    const state = this.snapshot();

    const context: KitEditorCommandContext = {
      kitEditor: state,
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

registerKitEditorStoreFactory((parent, yMap, transact, id, state) => new KitEditorStore(parent, yMap, transact, id, state));

function useKitEditorStore<T>(selector?: (store: KitEditorStore) => T, id?: KitEditorId): T | KitEditorStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) {
    return null;
  }
  try {
    const kitEditorStore = store.kitEditor(resolvedKitId);
    const result = selector ? selector(kitEditorStore) : kitEditorStore;
    return result;
  } catch {
    return null;
  }
}

export function useKitEditor<T>(selector?: (state: KitEditorState) => T, id?: KitEditorId): T | KitEditorState | null {
  const store = useKitEditorStore(undefined, id);
  if (!store) {
    return null;
  }
  const result = useSyncDeep<KitEditorState, T>(store as KitEditorStore, selector ? selector : identitySelector);
  return result;
}

export function useKitEditorSelection(id?: KitEditorId): KitEditorSelection {
  const store = useKitEditorStore(identitySelector, id) as KitEditorStore | null;
  if (!store) return emptyKitEditorSelection;
  return (useSync<KitEditorState, KitEditorSelection>(store, (state) => state.selection ?? emptyKitEditorSelection, true) as KitEditorSelection) ?? emptyKitEditorSelection;
}

export function useKitEditorFullscreen(): KitEditorFullscreenWindow {
  return useKitEditor((s) => s.fullscreenWindow) as KitEditorFullscreenWindow;
}

export function useKitEditorOthers(): KitEditorPresenceOther[] {
  return useKitEditor((s) => s.others) as KitEditorPresenceOther[];
}

export function useKitEditorCommands(id?: KitEditorId) {
  const store = useKitEditorStore(undefined, id) as KitEditorStore | null;
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
    startTransaction: () => store.execute("semio.kitEditor.startTransaction"),
    finalizeTransaction: () => store.execute("semio.kitEditor.finalizeTransaction"),
    abortTransaction: () => store.execute("semio.kitEditor.abortTransaction"),
    undo: () => store.execute("semio.kitEditor.undo"),
    redo: () => store.execute("semio.kitEditor.redo"),
    selectAll: () => store.execute("semio.kitEditor.selectAll"),
    deselectAll: () => store.execute("semio.kitEditor.deselectAll"),
    selectType: (Guid: Guid) => store.execute("semio.kitEditor.selectType", Guid),
    selectTypes: (typeIds: Guid[]) => store.execute("semio.kitEditor.selectTypes", typeIds),
    addTypeToSelection: (Guid: Guid) => store.execute("semio.kitEditor.addTypeToSelection", Guid),
    removeTypeFromSelection: (Guid: Guid) => store.execute("semio.kitEditor.removeTypeFromSelection", Guid),
    selectDesign: (Guid: Guid) => store.execute("semio.kitEditor.selectDesign", Guid),
    selectDesigns: (designIds: Guid[]) => store.execute("semio.kitEditor.selectDesigns", designIds),
    addDesignToSelection: (Guid: Guid) => store.execute("semio.kitEditor.addDesignToSelection", Guid),
    removeDesignFromSelection: (Guid: Guid) => store.execute("semio.kitEditor.removeDesignFromSelection", Guid),
    selectQuality: (key: string) => store.execute("semio.kitEditor.selectQuality", key),
    selectQualities: (keys: string[]) => store.execute("semio.kitEditor.selectQualities", keys),
    addQualityToSelection: (key: string) => store.execute("semio.kitEditor.addQualityToSelection", key),
    removeQualityFromSelection: (key: string) => store.execute("semio.kitEditor.removeQualityFromSelection", key),
    selectFile: (path: string) => store.execute("semio.kitEditor.selectFile", path),
    selectFiles: (paths: string[]) => store.execute("semio.kitEditor.selectFiles", paths),
    addFileToSelection: (path: string) => store.execute("semio.kitEditor.addFileToSelection", path),
    removeFileFromSelection: (path: string) => store.execute("semio.kitEditor.removeFileFromSelection", path),
    selectAuthor: (name: string) => store.execute("semio.kitEditor.selectAuthor", name),
    selectAuthors: (names: string[]) => store.execute("semio.kitEditor.selectAuthors", names),
    addAuthorToSelection: (name: string) => store.execute("semio.kitEditor.addAuthorToSelection", name),
    removeAuthorFromSelection: (name: string) => store.execute("semio.kitEditor.removeAuthorFromSelection", name),
    deleteSelected: () => store.execute("semio.kitEditor.deleteSelected"),
    toggleTypesFullscreen: () => store.execute("semio.kitEditor.toggleTypesFullscreen"),
    toggleDesignsFullscreen: () => store.execute("semio.kitEditor.toggleDesignsFullscreen"),
    addType: (type: Type) => store.execute("semio.kitEditor.addType", type),
    addTypes: (types: Type[]) => store.execute("semio.kitEditor.addTypes", types),
    removeType: (Guid: Guid) => store.execute("semio.kitEditor.removeType", Guid),
    removeTypes: (typeIds: Guid[]) => store.execute("semio.kitEditor.removeTypes", typeIds),
    addDesign: (design: Design) => store.execute("semio.kitEditor.addDesign", design),
    addDesigns: (designs: Design[]) => store.execute("semio.kitEditor.addDesigns", designs),
    removeDesign: (Guid: Guid) => store.execute("semio.kitEditor.removeDesign", Guid),
    removeDesigns: (designIds: Guid[]) => store.execute("semio.kitEditor.removeDesigns", designIds),
    updateType: (Guid: Guid, typeDiff: TypeDiff) => store.execute("semio.kitEditor.updateType", Guid, typeDiff),
    updateTypes: (updates: { id: Guid; diff: TypeDiff }[]) => store.execute("semio.kitEditor.updateTypes", updates),
    updateDesign: (Guid: Guid, designDiff: DesignDiff) => store.execute("semio.kitEditor.updateDesign", Guid, designDiff),
    updateDesigns: (updates: { id: Guid; diff: DesignDiff }[]) => store.execute("semio.kitEditor.updateDesigns", updates),
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setFilterSearch: (search: string) => store.execute("semio.kitEditor.setFilterSearch", search),
    setExpandedRows: (rows: string[]) => store.execute("semio.kitEditor.setExpandedRows", rows),
    toggleExpandedRow: (rowId: string) => store.execute("semio.kitEditor.toggleExpandedRow", rowId),
    setSortColumn: (column: KitEditorSortColumn) => store.execute("semio.kitEditor.setSortColumn", column),
    setSortDirection: (direction: KitEditorSortDirection) => store.execute("semio.kitEditor.setSortDirection", direction),
    toggleSort: (column: KitEditorSortColumn) => store.execute("semio.kitEditor.toggleSort", column),
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}

// #endregion Kit Editor

// #region Types

/**
 * Check if a specific type is directly hovered in Kit Editor
 */
export function useKitEditorIsTypeHovered(typeId: string, id?: KitEditorId): boolean {
  return (useKitEditor((state) => state.hover?.type === typeId, id) as boolean) ?? false;
}

/**
 * Get the diff status of a type from the current kit diff
 */
export function useKitEditorTypeStatus(typeId: string, id?: KitEditorId): DiffStatus {
  const store = useKitEditorStore(identitySelector, id) as KitEditorStore;
  if (!store) return DiffStatus.Unchanged;

  return useSync<KitEditorState, DiffStatus>(
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
export function useKitEditorTypeColor(typeId: string, isSelected: boolean, id?: KitEditorId): { fill: string; stroke: string; opacity: number } {
  const isHovered = useKitEditorIsTypeHovered(typeId, id);
  const status = useKitEditorTypeStatus(typeId, id);

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
 * Check if a specific design is directly hovered in Kit Editor
 */
export function useKitEditorIsDesignHovered(designId: string, id?: KitEditorId): boolean {
  return (useKitEditor((state) => state.hover?.design === designId, id) as boolean) ?? false;
}

/**
 * Get the diff status of a design from the current kit diff
 */
export function useKitEditorDesignStatus(designId: string, id?: KitEditorId): DiffStatus {
  const store = useKitEditorStore(identitySelector, id) as KitEditorStore;
  if (!store) return DiffStatus.Unchanged;

  return useSync<KitEditorState, DiffStatus>(
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
export function useKitEditorDesignColor(designId: string, isSelected: boolean, id?: KitEditorId): { fill: string; stroke: string; opacity: number } {
  const isHovered = useKitEditorIsDesignHovered(designId, id);
  const status = useKitEditorDesignStatus(designId, id);

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
