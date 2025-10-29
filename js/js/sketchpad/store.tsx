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

import React, { createContext, useContext, useEffect, useMemo, useState, useSyncExternalStore } from "react";
import { useLocation, useNavigate } from "react-router";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { areSameKit, guid, Guid, inverseKitDiff, Kit, KitDiff, KitShallow } from "../semio";
import { useAppType } from "./appType";
import { commands as sketchpadCommands } from "./commands";
import { KitStore } from "./kits/store";

// Forward type declarations to avoid circular dependencies with app stores
export interface DesignAppId {
  kit: Guid;
  design: Guid;
}
export interface KitAppId {
  kit: Guid;
}
export interface TypeAppId {
  kit: Guid;
  type: Guid;
}
export interface QualityAppId {
  kit: Guid;
  quality: Guid;
}

// Placeholder types - these will be augmented by actual app store modules
type DesignAppState = any;
type KitAppState = any;
type TypeAppState = any;
type QualityAppState = any;
export {
  AuthorScopeProvider,
  ConnectionScopeProvider,
  DesignScopeProvider,
  DesignStore,
  KitScopeProvider,
  KitStore,
  PieceScopeProvider,
  QualityScopeProvider,
  TypeScopeProvider,
  useAuthor,
  useConnection,
  useDesign,
  useDesignId,
  useDesigns,
  useDesignScope,
  useDiffedDesign,
  useExplodeableDesignNodes,
  useFileUrls,
  useFlatDesign,
  useFlatPieces,
  useFlattenDiff,
  useIncludedDesigns,
  useIsInDesignScope,
  useIsInKitScope,
  useIsInQualityScope,
  useIsInTypeScope,
  useKit,
  useKitCommands,
  useKitScope,
  useKitStore,
  usePiece,
  usePieceDiffStatuses,
  usePiecePlane,
  usePiecePlanes,
  usePieceRepresentationUrls,
  usePieces,
  usePiecesFromIds,
  usePiecesMetadata,
  usePortColoredTypes,
  useQuality,
  useQualityScope,
  useReplacableDesigns,
  useReplacableTypes,
  useType,
  useTypeScope,
} from "./kits/store";
export type { KitCommandContext, KitCommandResult } from "./kits/store";

// #region Constants

export enum Access {
  USER = "user",
  GUEST = "guest",
}

export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export enum Layout {
  NORMAL = "normal",
  TOUCH = "touch",
}

export enum Mode {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

export type AppType = string;

export enum ToolType {
  // Selection tools
  SELECTION_NORMAL = "selection-normal",
  SELECTION_ADDITIVE = "selection-additive",
  SELECTION_SUBTRACTIVE = "selection-subtractive",
  // Lasso tools
  LASSO_RECTANGULAR = "lasso-rectangular",
  LASSO_FREEFORM = "lasso-freeform",
  // Type app tools
  PORT = "port",
}

// #endregion Constants

// #region General

export type Subscribe = () => void;
export type Unsubscribe = () => void;
export type Disposable = () => void;
export type Transact = (fn: () => void) => void;
export type Url = string;
export type SketchpadId = string;
export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

export interface FileOperation {
  type: "upload" | "download" | "delete";
  kitId: string;
  fileId: string;
  path: string;
  blob?: Blob;
}

export interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

export type FileProviderFactory = (kitId: string) => Promise<FileProvider>;

export type YUuid = string;
export type YUuidArray = Y.Array<YUuid>;

export type YConcept = string;
export type YConcepts = Y.Array<string>;

export type YStringArray = Y.Array<string>;
export type YLeafMapString = Y.Map<string>;
export type YLeafMapNumber = Y.Map<number>;
export type YAttributes = Y.Array<Y.Map<string>>;

export function createObserver(yObject: Y.AbstractType<any>, subscribe: Subscribe, deep?: boolean): Unsubscribe {
  if (deep) {
    yObject.observeDeep(subscribe);
    return () => {
      yObject.unobserveDeep(subscribe);
    };
  } else {
    yObject.observe(subscribe);
    return () => {
      yObject.unobserve(subscribe);
    };
  }
}

// #endregion General

// #region Store Hierarchy

export enum StoreStatus {
  IDLE = "idle",
  LOADING = "loading",
  ERROR = "error",
  READY = "ready",
}

export interface StoreState<TState> {
  status: StoreStatus;
  data?: TState;
  error?: Error;
}

export abstract class Store<TState> {
  public readonly guid: Guid;
  public readonly parent: SketchpadStore;
  public readonly yMap: Y.Map<any>;
  protected readonly transact: Transact;
  protected cache?: TState;
  protected cacheHash?: string;
  protected status: StoreStatus = StoreStatus.IDLE;
  protected error?: Error;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact) {
    this.guid = guid();
    this.parent = parent;
    this.yMap = yMap;
    this.transact = transact;
    this.initialize();
  }

  protected initialize(): void {
    try {
      this.status = StoreStatus.LOADING;
      this.buildSnapshot();
      this.status = StoreStatus.READY;
    } catch (error) {
      this.status = StoreStatus.ERROR;
      this.error = error instanceof Error ? error : new Error(String(error));
    }
  }

  protected abstract hash(state: TState): string;
  protected abstract buildSnapshot(): TState;

  snapshot(): TState {
    if (this.status === StoreStatus.ERROR) {
      throw this.error || new Error("Store is in error state");
    }
    const currentData = this.buildSnapshot();
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  }

  getState(): StoreState<TState> {
    return {
      status: this.status,
      data: this.status === StoreStatus.READY ? this.snapshot() : undefined,
      error: this.error,
    };
  }

  isReady(): boolean {
    return this.status === StoreStatus.READY;
  }

  isLoading(): boolean {
    return this.status === StoreStatus.LOADING;
  }

  isError(): boolean {
    return this.status === StoreStatus.ERROR;
  }

  protected handleError(error: unknown, context: string): void {
    this.status = StoreStatus.ERROR;
    this.error = error instanceof Error ? error : new Error(`${context}: ${String(error)}`);
    console.error(`[Store ${this.guid}] ${context}:`, error);
  }

  protected clearError(): void {
    this.error = undefined;
    if (this.status === StoreStatus.ERROR) {
      this.status = StoreStatus.READY;
    }
  }

  onChanged(subscribe: Subscribe): Unsubscribe {
    return createObserver(this.yMap, subscribe);
  }

  onChangedDeep(subscribe: Subscribe): Unsubscribe {
    return createObserver(this.yMap, subscribe, true);
  }
}

export interface AppStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

export interface AppEdit<TSelectionDiff = any> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

export interface AppDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenWindow?: any;
  panelVisibility?: Partial<PanelVisibility>;
}

export interface AppCommandResult<TDiff = any> {
  diff?: TDiff;
}

export interface PanelVisibility {
  toolbar?: boolean;
  workbench?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
}

const defaultPanelVisibility: PanelVisibility = {
  toolbar: true,
  workbench: true,
  details: true,
  chat: true,
  settings: true,
};

const initialDocsPanelVisibility: PanelVisibility = {
  toolbar: false,
  workbench: true,
  details: false,
  chat: false,
  settings: false,
  tools: false,
  hud: false,
  stats: false,
};

let docsPanelVisibilityState: PanelVisibility = initialDocsPanelVisibility;
const docsPanelVisibilityListeners = new Set<() => void>();

type DocsPanelVisibilityUpdate = PanelVisibility | ((prev: PanelVisibility) => PanelVisibility);

function getDocsPanelVisibilitySnapshot(): PanelVisibility {
  return docsPanelVisibilityState;
}

function subscribeDocsPanelVisibility(listener: () => void) {
  docsPanelVisibilityListeners.add(listener);
  return () => {
    docsPanelVisibilityListeners.delete(listener);
  };
}

function updateDocsPanelVisibilityState(update: DocsPanelVisibilityUpdate) {
  const next = typeof update === "function" ? (update as (prev: PanelVisibility) => PanelVisibility)(docsPanelVisibilityState) : update;
  const merged = { ...docsPanelVisibilityState, ...next };
  docsPanelVisibilityState = merged;
  docsPanelVisibilityListeners.forEach((listener) => listener());
}

export abstract class AppStore<TState, TDiff extends AppDiff<TSelectionDiff>, TSelectionDiff, TEdit extends AppEdit<TSelectionDiff>, TCommandContext, TCommandResult extends AppCommandResult<TDiff>> extends Store<TState> {
  protected readonly commandRegistry: Map<string, (context: TCommandContext, ...rest: any[]) => TCommandResult> = new Map();
  private lastDeletedTransactionEdit?: TEdit;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact) {
    super(parent, yMap, transact);
  }

  protected abstract applySelectionDiff(selectionDiff: TSelectionDiff): void;
  protected abstract inverseSelectionDiff(selection: any, diff: TSelectionDiff): TSelectionDiff;
  protected abstract getSelection(): any;

  get isTransactionActive(): boolean {
    return (this.yMap.get("isTransactionActive") as boolean) || false;
  }

  set isTransactionActive(active: boolean) {
    this.yMap.set("isTransactionActive", active);
  }

  get currentTransactionStack(): TEdit[] {
    const yStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  get pastTransactionsStack(): TEdit[] {
    const yStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  get redoStack(): TEdit[] {
    const yStack = this.yMap.get("redoStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  canUndo(): boolean {
    if (this.isTransactionActive) return this.currentTransactionStack.length > 0;
    return this.pastTransactionsStack.length > 0;
  }

  canRedo(): boolean {
    if (this.isTransactionActive) return false;
    return this.redoStack.length > 0;
  }

  change(diff: TDiff): void {
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
    });
  }

  startTransaction(): void {
    // If there's an ongoing transaction, finalize it first
    if (this.isTransactionActive) {
      this.finalizeTransaction();
    }
    this.isTransactionActive = true;
  }

  abortTransaction(): void {
    if (this.isTransactionActive) {
      this.transact(() => {
        const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
        if (currentStack && currentStack.length > 0) {
          for (let i = currentStack.length - 1; i >= 0; i--) {
            const edit = currentStack.get(i);
            if (edit?.undo?.selectionDiff) {
              this.applySelectionDiff(edit.undo.selectionDiff);
            }
          }
          currentStack.delete(0, currentStack.length);
        }
        this.isTransactionActive = false;
      });
    }
  }

  finalizeTransaction(): void {
    if (this.isTransactionActive) {
      this.transact(() => {
        let redoStack = this.yMap.get("redoStack") as Y.Array<any>;
        if (redoStack && redoStack.length > 0) {
          redoStack.delete(0, redoStack.length);
        }
        const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
        let pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
        if (!pastStack) {
          pastStack = new Y.Array<any>();
          this.yMap.set("pastTransactionsStack", pastStack);
        }
        if (currentStack && currentStack.length > 0) {
          const edits = currentStack.toArray();
          if (edits.length === 1) {
            pastStack.push([edits[0]]);
          } else if (edits.length > 1) {
            const firstEdit = edits[0];
            const lastEdit = edits[edits.length - 1];
            const mergedEdit = { do: lastEdit.do, undo: firstEdit.undo };
            pastStack.push([mergedEdit]);
          }
          currentStack.delete(0, currentStack.length);
        }
        this.isTransactionActive = false;
      });
    }
  }

  undo(): void {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack && currentStack.length > 0) {
        const edit = currentStack.get(currentStack.length - 1);
        this.lastDeletedTransactionEdit = edit;
        currentStack.delete(currentStack.length - 1, 1);
        if (edit?.undo?.selectionDiff) {
          this.applySelectionDiff(edit.undo.selectionDiff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      let redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (!redoStack) {
        redoStack = new Y.Array<any>();
        this.yMap.set("redoStack", redoStack);
      }
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(pastStack.length - 1);
        pastStack.delete(pastStack.length - 1, 1);
        redoStack.push([edit]);
        if (edit?.undo?.selectionDiff) {
          this.applySelectionDiff(edit.undo.selectionDiff);
        }
      }
    }
  }

  redo(): void {
    if (this.isTransactionActive) {
      let currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (!currentStack) {
        currentStack = new Y.Array<any>();
        this.yMap.set("currentTransactionStack", currentStack);
      }
      const lastDeletedEdit = this.lastDeletedTransactionEdit;
      if (lastDeletedEdit) {
        currentStack.push([lastDeletedEdit]);
        this.lastDeletedTransactionEdit = undefined;
        if (lastDeletedEdit.do?.selectionDiff) {
          this.applySelectionDiff(lastDeletedEdit.do.selectionDiff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      const redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (redoStack && redoStack.length > 0) {
        const edit = redoStack.get(redoStack.length - 1);
        redoStack.delete(redoStack.length - 1, 1);
        if (pastStack) {
          pastStack.push([edit]);
        }
        if (edit?.do?.selectionDiff) {
          this.applySelectionDiff(edit.do.selectionDiff);
        }
      }
    }
  }

  protected recordEdit(result: TCommandResult): void {
    if (this.isTransactionActive && result.diff) {
      let redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (redoStack && redoStack.length > 0) {
        redoStack.delete(0, redoStack.length);
      }
      this.lastDeletedTransactionEdit = undefined;
      let currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (!currentStack) {
        currentStack = new Y.Array<any>();
        this.yMap.set("currentTransactionStack", currentStack);
      }
      const selection = this.getSelection();
      const inversedSelectionDiff = result.diff?.selection ? this.inverseSelectionDiff(selection, result.diff.selection) : undefined;
      const doStep: AppStep<TSelectionDiff> = { selectionDiff: result.diff?.selection };
      const undoStep: AppStep<TSelectionDiff> = { selectionDiff: inversedSelectionDiff };
      const edit = { do: doStep, undo: undoStep };
      currentStack.push([edit]);
    }
  }

  registerCommand(command: string, callback: (context: TCommandContext, ...rest: any[]) => TCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  abstract executeCommand<T>(command: string, ...rest: any[]): Promise<T>;
}

export interface KitDiffAppStep<TSelectionDiff = any> extends AppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
}

export interface KitDiffAppEdit<TSelectionDiff = any> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

export interface KitDiffAppCommandResult<TDiff = any> extends AppCommandResult<TDiff> {
  kitDiff?: KitDiff;
}

export abstract class KitDiffAppStore<TState, TDiff extends AppDiff<TSelectionDiff>, TSelectionDiff, TEdit extends KitDiffAppEdit<TSelectionDiff>, TCommandContext, TCommandResult extends KitDiffAppCommandResult<TDiff>> extends AppStore<
  TState,
  TDiff,
  TSelectionDiff,
  TEdit,
  TCommandContext,
  TCommandResult
> {
  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact) {
    super(parent, yMap, transact);
  }

  abstract kit(): KitStore;

  abortTransaction(): void {
    if (this.isTransactionActive) {
      this.transact(() => {
        const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
        if (currentStack && currentStack.length > 0) {
          for (let i = currentStack.length - 1; i >= 0; i--) {
            const edit = currentStack.get(i);
            if (edit?.undo) {
              if (edit.undo.kitDiff) {
                this.kit().change(edit.undo.kitDiff);
              }
              if (edit.undo.selectionDiff) {
                this.applySelectionDiff(edit.undo.selectionDiff);
              }
            }
          }
          currentStack.delete(0, currentStack.length);
        }
        this.isTransactionActive = false;
      });
    }
  }

  undo(): void {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack && currentStack.length > 0) {
        const edit = currentStack.get(currentStack.length - 1);
        (this as any).lastDeletedTransactionEdit = edit;
        currentStack.delete(currentStack.length - 1, 1);
        if (edit?.undo) {
          if (edit.undo.kitDiff) {
            this.kit().change(edit.undo.kitDiff);
          }
          if (edit.undo.selectionDiff) {
            this.applySelectionDiff(edit.undo.selectionDiff);
          }
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      let redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (!redoStack) {
        redoStack = new Y.Array<any>();
        this.yMap.set("redoStack", redoStack);
      }
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(pastStack.length - 1);
        pastStack.delete(pastStack.length - 1, 1);
        redoStack.push([edit]);
        if (edit?.undo) {
          if (edit.undo.kitDiff) {
            this.kit().change(edit.undo.kitDiff);
          }
          if (edit.undo.selectionDiff) {
            this.applySelectionDiff(edit.undo.selectionDiff);
          }
        }
      }
    }
  }

  redo(): void {
    if (this.isTransactionActive) {
      let currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (!currentStack) {
        currentStack = new Y.Array<any>();
        this.yMap.set("currentTransactionStack", currentStack);
      }
      const lastDeletedEdit = (this as any).lastDeletedTransactionEdit;
      if (lastDeletedEdit) {
        currentStack.push([lastDeletedEdit]);
        (this as any).lastDeletedTransactionEdit = undefined;
        if (lastDeletedEdit.do) {
          if (lastDeletedEdit.do.kitDiff) {
            this.kit().change(lastDeletedEdit.do.kitDiff);
          }
          if (lastDeletedEdit.do.selectionDiff) {
            this.applySelectionDiff(lastDeletedEdit.do.selectionDiff);
          }
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      const redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (redoStack && redoStack.length > 0) {
        const edit = redoStack.get(redoStack.length - 1);
        redoStack.delete(redoStack.length - 1, 1);
        if (pastStack) {
          pastStack.push([edit]);
        }
        if (edit?.do) {
          if (edit.do.kitDiff) {
            this.kit().change(edit.do.kitDiff);
          }
          if (edit.do.selectionDiff) {
            this.applySelectionDiff(edit.do.selectionDiff);
          }
        }
      }
    }
  }

  protected recordEdit(result: TCommandResult): void {
    if (this.isTransactionActive && (result.diff || result.kitDiff)) {
      let redoStack = this.yMap.get("redoStack") as Y.Array<any>;
      if (redoStack && redoStack.length > 0) {
        redoStack.delete(0, redoStack.length);
      }
      (this as any).lastDeletedTransactionEdit = undefined;
      let currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (!currentStack) {
        currentStack = new Y.Array<any>();
        this.yMap.set("currentTransactionStack", currentStack);
      }
      const selection = this.getSelection();
      const inversedSelectionDiff = result.diff?.selection ? this.inverseSelectionDiff(selection, result.diff.selection) : undefined;
      const kitStore = this.kit();
      const kitState = kitStore.snapshot();
      const inversedKitDiff = result.kitDiff ? inverseKitDiff(kitState, result.kitDiff) : undefined;
      const doStep: KitDiffAppStep<TSelectionDiff> = { kitDiff: result.kitDiff, selectionDiff: result.diff?.selection };
      const undoStep: KitDiffAppStep<TSelectionDiff> = { kitDiff: inversedKitDiff, selectionDiff: inversedSelectionDiff };
      const edit = { do: doStep, undo: undoStep };
      currentStack.push([edit]);
    }
  }
}

// #endregion Store Hierarchy

// #region Synchronizable

export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

export const identitySelector = (state: any) => state;

const nullStore: Synchronizable<null> = {
  onChanged: () => () => {},
  onChangedDeep: () => () => {},
  snapshot: () => null,
};

export function useSync<TAccessl, TSelected = TAccessl>(store: Synchronizable<TAccessl> | null, selector?: (state: TAccessl) => TSelected, deep: boolean = false): TAccessl | TSelected | null {
  const actualStore = store || (nullStore as unknown as Synchronizable<TAccessl>);
  const state = deep ? useSyncExternalStore(actualStore.onChangedDeep.bind(actualStore), actualStore.snapshot.bind(actualStore)) : useSyncExternalStore(actualStore.onChanged.bind(actualStore), actualStore.snapshot.bind(actualStore));
  if (!store) return null;
  return selector ? selector(state) : state;
}

export function useSyncDeep<TAccessl, TSelected = TAccessl>(store: Synchronizable<TAccessl> | null, selector?: (state: TAccessl) => TSelected): TAccessl | TSelected | null {
  return useSync(store, selector, true);
}

export function useSyncWithState<TAccessl, TSelected = TAccessl>(store: (Synchronizable<TAccessl> & Store<TAccessl>) | null, selector?: (state: TAccessl) => TSelected, deep: boolean = false): StoreState<TAccessl | TSelected> {
  const actualStore = store || (nullStore as unknown as Synchronizable<TAccessl> & Store<TAccessl>);
  const state = deep ? useSyncExternalStore(actualStore.onChangedDeep.bind(actualStore), actualStore.snapshot.bind(actualStore)) : useSyncExternalStore(actualStore.onChanged.bind(actualStore), actualStore.snapshot.bind(actualStore));
  if (!store) {
    return { status: StoreStatus.IDLE, data: null as any };
  }
  const storeState = (store as Store<TAccessl>).getState();
  return {
    ...storeState,
    data: storeState.data && selector ? selector(storeState.data) : storeState.data,
  } as StoreState<TAccessl | TSelected>;
}

function areSameDesignApp(designApp: DesignAppId, other: DesignAppId): boolean {
  return !!designApp && !!other && areSameKit(designApp.kit, other.kit) && designApp.design === other.design;
}

function hasSameDesignApp(designApp: DesignAppId, others: DesignAppId[]): boolean {
  return others.some((other) => areSameDesignApp(designApp, other));
}

function areSameKitApp(kitApp: KitAppId, other: KitAppId): boolean {
  return !!kitApp && !!other && areSameKit(kitApp.kit, other.kit);
}

function hasSameKitApp(kitApp: KitAppId, others: KitAppId[]): boolean {
  return others.some((other) => areSameKitApp(kitApp, other));
}

type YKitAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitApp = Y.Map<YKitAppVal>;
type YKitApps = Y.Map<YKitApp>;

type YDesignAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignApp = Y.Map<YDesignAppVal>;
type YDesignApps = Y.Map<Y.Map<YDesignApp>>;

type YTypeAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YTypeApp = Y.Map<YTypeAppVal>;
type YTypeApps = Y.Map<YTypeApp>;

type YQualityAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YQualityApp = Y.Map<YQualityAppVal>;
type YQualityApps = Y.Map<YQualityApp>;

type YKitMetadata = Y.Map<string | boolean>;
type YKits = Y.Array<YKitMetadata>;

type KitAppStoreInstance = any;
type DesignAppStoreInstance = any;
type TypeAppStoreInstance = any;
type QualityAppStoreInstance = any;
type HomeStoreInstance = any;

type KitAppStoreFactory = (parent: SketchpadStore, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) => KitAppStoreInstance;
type DesignAppStoreFactory = (parent: SketchpadStore, yMap: YDesignApp, transact: (fn: () => void) => void, id: DesignAppId, state?: DesignAppState) => DesignAppStoreInstance;
type TypeAppStoreFactory = (parent: SketchpadStore, yMap: YTypeApp, transact: (fn: () => void) => void, id: TypeAppId, state?: TypeAppState) => TypeAppStoreInstance;
type QualityAppStoreFactory = (parent: SketchpadStore, yMap: YQualityApp, transact: (fn: () => void) => void, id: QualityAppId, state?: QualityAppState) => QualityAppStoreInstance;
type HomeStoreFactory = (parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) => HomeStoreInstance;

let kitAppStoreFactory: KitAppStoreFactory | undefined;
let designAppStoreFactory: DesignAppStoreFactory | undefined;
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;
let homeStoreFactory: HomeStoreFactory | undefined;

export function registerKitAppStoreFactory(factory: KitAppStoreFactory) {
  kitAppStoreFactory = factory;
}

export function registerDesignAppStoreFactory(factory: DesignAppStoreFactory) {
  designAppStoreFactory = factory;
}

export function registerTypeAppStoreFactory(factory: TypeAppStoreFactory) {
  typeAppStoreFactory = factory;
}

export function registerQualityAppStoreFactory(factory: QualityAppStoreFactory) {
  qualityAppStoreFactory = factory;
}

export function registerHomeStoreFactory(factory: HomeStoreFactory) {
  homeStoreFactory = factory;
}

function resolveKitAppStoreFactory(): KitAppStoreFactory {
  if (!kitAppStoreFactory) throw new Error("Kit app store factory not registered");
  return kitAppStoreFactory;
}

function resolveDesignAppStoreFactory(): DesignAppStoreFactory {
  if (!designAppStoreFactory) throw new Error("Design app store factory not registered");
  return designAppStoreFactory;
}

function resolveTypeAppStoreFactory(): TypeAppStoreFactory {
  if (!typeAppStoreFactory) throw new Error("Type app store factory not registered");
  return typeAppStoreFactory;
}

function resolveQualityAppStoreFactory(): QualityAppStoreFactory {
  if (!qualityAppStoreFactory) throw new Error("Quality app store factory not registered");
  return qualityAppStoreFactory;
}

function resolveHomeStoreFactory(): HomeStoreFactory {
  if (!homeStoreFactory) throw new Error("Home store factory not registered");
  return homeStoreFactory;
}

// #endregion General

// #region Sketchpad

type YSketchpadVal = string | number | boolean | YDesignApps;
type YSketchpad = Y.Map<YSketchpadVal>;

export interface AppSettings {
  design?: {
    snappiness?: number;
    gridSize?: number;
  };
  type?: Record<string, any>;
  kit?: Record<string, any>;
}

export interface PanelSizes {
  toolbarHeight: number;
  workbenchWidth: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  chatWidth: number;
  settingsWidth: number;
  consoleHeight: number;
}

export interface SketchpadChangableState {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  access: Access;
  theme: Theme;
  layout: Layout;
  mode: Mode;
  appSettings: AppSettings;
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isNavbarExpanded: boolean;
  isMobile: boolean;
  activeInteraction?: string;
}
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

export interface SketchpadDiff {
  navigation?: string;
  navigationHistoryIndex?: number;
  recentSearches?: string[];
  recentFocusItems?: Record<string, string[]>;
  access?: Access;
  theme?: Theme;
  layout?: Layout;
  mode?: Mode;
  appSettings?: AppSettings;
  panelSizes?: Partial<PanelSizes>;
  isFullscreen?: boolean;
  isNavbarExpanded?: boolean;
  isMobile?: boolean;
  activeInteraction?: string;
}

export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
}
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
}

export class SketchpadStore {
  private readonly id: string | undefined;
  private readonly yProviderFactory: YProviderFactory | undefined;
  private readonly fileProviderFactory: FileProviderFactory | undefined;
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly kits: Map<string, KitStore>;
  private readonly yKits: YKits;
  private readonly yHome: Y.Map<any>;
  private homeStore?: HomeStoreInstance;
  private readonly yKitApps: YKitApps;
  private readonly kitApps: Map<string, KitAppStoreInstance>;
  private readonly yTypeApps: YTypeApps;
  private readonly typeApps: Map<string, TypeAppStoreInstance>;
  private readonly yQualityApps: YQualityApps;
  private readonly qualityApps: Map<string, QualityAppStoreInstance>;
  private readonly yDesignApps: YDesignApps;
  private readonly designApps: Map<string, Map<string, DesignAppStoreInstance>>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult>;
  private cache?: SketchpadState;
  private cacheHash?: string;
  private kitShallowsCache?: KitShallow[];
  private kitShallowsCacheHash?: string;
  private readonly kitCreatedSubscribers: Set<Subscribe>;
  private readonly kitDeletedSubscribers: Set<Subscribe>;
  private readonly kitAppCreatedSubscribers: Set<Subscribe>;
  private readonly kitAppDeletedSubscribers: Set<Subscribe>;
  private readonly typeAppCreatedSubscribers: Set<Subscribe>;
  private readonly typeAppDeletedSubscribers: Set<Subscribe>;
  private readonly qualityAppCreatedSubscribers: Set<Subscribe>;
  private readonly qualityAppDeletedSubscribers: Set<Subscribe>;
  private readonly designAppCreatedSubscribers: Set<Subscribe>;
  private readonly designAppDeletedSubscribers: Set<Subscribe>;
  // private readonly broadcastChannel: BroadcastChannel;

  constructor(id?: string, yProviderFactory?: YProviderFactory, fileProviderFactory?: FileProviderFactory) {
    this.id = id;
    this.yProviderFactory = yProviderFactory;
    this.fileProviderFactory = fileProviderFactory;
    // this.broadcastChannel = new BroadcastChannel(`semio-sketchpad-${id}`);
    this.yDoc = new Y.Doc();
    this.kits = new Map();
    this.kitApps = new Map();
    this.typeApps = new Map();
    this.qualityApps = new Map();
    this.designApps = new Map();
    this.commandRegistry = new Map();
    this.kitCreatedSubscribers = new Set();
    this.kitDeletedSubscribers = new Set();
    this.kitAppCreatedSubscribers = new Set();
    this.kitAppDeletedSubscribers = new Set();
    this.typeAppCreatedSubscribers = new Set();
    this.typeAppDeletedSubscribers = new Set();
    this.qualityAppCreatedSubscribers = new Set();
    this.qualityAppDeletedSubscribers = new Set();
    this.designAppCreatedSubscribers = new Set();
    this.designAppDeletedSubscribers = new Set();

    if (id) {
      this.persistence = new IndexeddbPersistence(`semio-sketchpad-${id}`, this.yDoc);
      if (yProviderFactory) {
        yProviderFactory(this.yDoc, id);
      }
    }

    this.ySketchpad = this.yDoc.getMap("sketchpad");
    this.yKits = this.yDoc.getArray("kits");
    this.yHome = this.yDoc.getMap("home");
    this.yKitApps = this.yDoc.getMap("kitApps");
    this.yTypeApps = this.yDoc.getMap("typeApps");
    this.yQualityApps = this.yDoc.getMap("qualityApps");
    this.yDesignApps = this.yDoc.getMap("designApps");

    // Load persisted kits from IndexedDB
    this.loadPersistedKits();

    // Only initialize sketchpad settings if they don't exist (preserve on reload)
    this.yDoc.transact(() => {
      if (!this.ySketchpad.has("navigation")) {
        this.ySketchpad.set("navigation", "/");
      }
      if (!this.ySketchpad.has("navigationHistory")) {
        this.ySketchpad.set("navigationHistory", JSON.stringify(["/"]));
      }
      if (!this.ySketchpad.has("navigationHistoryIndex")) {
        this.ySketchpad.set("navigationHistoryIndex", 0);
      }
      if (!this.ySketchpad.has("recentSearches")) {
        this.ySketchpad.set("recentSearches", JSON.stringify([]));
      }
      if (!this.ySketchpad.has("recentFocusItems")) {
        this.ySketchpad.set("recentFocusItems", JSON.stringify({}));
      }
      if (!this.ySketchpad.has("access")) {
        this.ySketchpad.set("access", Access.GUEST);
      }
      if (!this.ySketchpad.has("theme")) {
        this.ySketchpad.set("theme", Theme.SYSTEM);
      }
      if (!this.ySketchpad.has("layout")) {
        this.ySketchpad.set("layout", Layout.NORMAL);
      }
      if (!this.ySketchpad.has("mode")) {
        this.ySketchpad.set("mode", Mode.BEGINNER);
      }
      if (!this.ySketchpad.has("isFullscreen")) {
        this.ySketchpad.set("isFullscreen", false);
      }
      if (!this.ySketchpad.has("isNavbarExpanded")) {
        this.ySketchpad.set("isNavbarExpanded", false);
      }
      if (!this.ySketchpad.has("isMobile")) {
        this.ySketchpad.set("isMobile", false);
      }
      if (!this.ySketchpad.has("activeInteraction")) {
        this.ySketchpad.set("activeInteraction", "");
      }
      if (!this.ySketchpad.has("appSettings")) {
        this.ySketchpad.set(
          "appSettings",
          JSON.stringify({
            design: { snappiness: 10, gridSize: 24 },
            type: {},
            kit: {},
          }),
        );
      }
      if (!this.ySketchpad.has("panelSizes")) {
        this.ySketchpad.set(
          "panelSizes",
          JSON.stringify({
            toolbarHeight: 52,
            workbenchWidth: 230,
            toolsWidth: 230,
            hudWidth: 230,
            statsWidth: 230,
            detailsWidth: 230,
            chatWidth: 230,
            settingsWidth: 230,
            consoleHeight: 200,
          }),
        );
      }
    });

    Object.entries(sketchpadCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  hash = (state: SketchpadState): string => {
    return JSON.stringify(state);
  };

  snapshot = (): SketchpadState => {
    const appSettingsStr = this.ySketchpad.get("appSettings") as string;
    const appSettings = appSettingsStr
      ? JSON.parse(appSettingsStr)
      : {
          design: { snappiness: 10, gridSize: 24 },
          type: {},
          kit: {},
        };
    const panelSizesStr = this.ySketchpad.get("panelSizes") as string;
    const panelSizes = panelSizesStr
      ? JSON.parse(panelSizesStr)
      : {
          toolbarHeight: 52,
          workbenchWidth: 230,
          toolsWidth: 230,
          hudWidth: 230,
          statsWidth: 230,
          detailsWidth: 230,
          chatWidth: 230,
          settingsWidth: 230,
          consoleHeight: 200,
        };
    const navigationHistoryStr = this.ySketchpad.get("navigationHistory") as string;
    const navigationHistory = navigationHistoryStr ? JSON.parse(navigationHistoryStr).map(migratePath) : ["/"];
    const recentSearchesStr = this.ySketchpad.get("recentSearches") as string;
    const recentSearches = recentSearchesStr ? JSON.parse(recentSearchesStr) : [];
    const recentFocusItemsStr = this.ySketchpad.get("recentFocusItems") as string;
    const recentFocusItems = recentFocusItemsStr ? JSON.parse(recentFocusItemsStr) : {};
    const currentValues = {
      navigation: migratePath((this.ySketchpad.get("navigation") as string) || "/"),
      navigationHistory: navigationHistory,
      navigationHistoryIndex: (this.ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
      recentSearches: recentSearches,
      recentFocusItems: recentFocusItems,
      access: this.ySketchpad.get("access") as Access,
      theme: this.ySketchpad.get("theme") as Theme,
      layout: this.ySketchpad.get("layout") as Layout,
      mode: (this.ySketchpad.get("mode") as Mode) ?? Mode.BEGINNER,
      appSettings: appSettings,
      panelSizes: panelSizes,
      isFullscreen: (this.ySketchpad.get("isFullscreen") as boolean) || false,
      isNavbarExpanded: (this.ySketchpad.get("isNavbarExpanded") as boolean) || false,
      isMobile: (this.ySketchpad.get("isMobile") as boolean) || false,
      activeInteraction: (this.ySketchpad.get("activeInteraction") as string) || undefined,
    };
    const currentHash = this.hash(currentValues);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentValues;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  createKit = (kit: Kit, local?: boolean, remote?: boolean) => {
    const kitStore = new KitStore(this, kit, local, remote, this.yProviderFactory, this.fileProviderFactory);
    this.kits.set(kit.guid, kitStore);

    // Store kit metadata in Y.Doc for persistence
    this.yDoc.transact(() => {
      const kitMetadata = new Y.Map<string | boolean>();
      kitMetadata.set("guid", kit.guid);
      kitMetadata.set("local", local || false);
      kitMetadata.set("remote", remote || false);
      this.yKits.push([kitMetadata]);
    });

    this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createKitApp = (kit: Guid) => {
    this.yDoc.transact(() => {
      const kitStore = this.kit(kit);
      let yKitApp = this.yKitApps.get(kit) as Y.Map<YKitAppVal>;
      if (!yKitApp) {
        yKitApp = new Y.Map<YKitAppVal>();
        this.yKitApps.set(kit, yKitApp);
      }
      const kitAppFactory = resolveKitAppStoreFactory();
      const kitApp = kitAppFactory(this, yKitApp, this.yDoc.transact.bind(this.yDoc), { kit });
      this.kitApps.set(kit, kitApp);
    });
    this.kitAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createDesignApp = (kit: Guid, design: Guid) => {
    this.yDoc.transact(() => {
      let yKitMap = this.yDesignApps.get(kit) as Y.Map<YDesignApp>;
      if (!yKitMap) {
        yKitMap = new Y.Map<YDesignApp>();
        this.yDesignApps.set(kit, yKitMap);
      }
      let yDesignApp = yKitMap.get(design) as Y.Map<YDesignAppVal>;
      if (!yDesignApp) {
        yDesignApp = new Y.Map<YDesignAppVal>();
        yKitMap.set(design, yDesignApp);
      }
      const designAppFactory = resolveDesignAppStoreFactory();
      const designApp = designAppFactory(this, yDesignApp, this.yDoc.transact.bind(this.yDoc), { kit, design });

      // Ensure the design apps map exists for this kit
      let designAppsMap = this.designApps.get(kit);
      if (!designAppsMap) {
        designAppsMap = new Map();
        this.designApps.set(kit, designAppsMap);
      }
      designAppsMap.set(design, designApp);
    });
    this.designAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  change(diff: SketchpadDiff) {
    this.yDoc.transact(() => {
      if (diff.navigationHistoryIndex !== undefined) {
        // History index changed (back/forward navigation)
        this.ySketchpad.set("navigationHistoryIndex", diff.navigationHistoryIndex);
      }

      if (diff.navigation) {
        // Update navigation history when navigation changes
        const currentHistoryStr = this.ySketchpad.get("navigationHistory") as string;
        const currentHistory = currentHistoryStr ? JSON.parse(currentHistoryStr) : ["/"];
        const currentIndex = (this.ySketchpad.get("navigationHistoryIndex") as number) ?? 0;

        // Check if the navigation matches the current index (back/forward navigation)
        const isHistoryNavigation = currentHistory[currentIndex] === diff.navigation;

        if (!isHistoryNavigation) {
          // Truncate future history and add new navigation
          const newHistory = currentHistory.slice(0, currentIndex + 1);
          if (newHistory[newHistory.length - 1] !== diff.navigation) {
            newHistory.push(diff.navigation);
            this.ySketchpad.set("navigationHistory", JSON.stringify(newHistory));
            this.ySketchpad.set("navigationHistoryIndex", newHistory.length - 1);
          }
        }

        this.ySketchpad.set("navigation", diff.navigation);
      }
      if ("recentSearches" in diff) {
        this.ySketchpad.set("recentSearches", JSON.stringify(diff.recentSearches || []));
      }
      if ("recentFocusItems" in diff) {
        const current = JSON.parse((this.ySketchpad.get("recentFocusItems") as string) || "{}");
        this.ySketchpad.set("recentFocusItems", JSON.stringify({ ...current, ...(diff.recentFocusItems || {}) }));
      }
      if (diff.access) this.ySketchpad.set("access", diff.access);
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.layout) this.ySketchpad.set("layout", diff.layout);
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.isFullscreen !== undefined) this.ySketchpad.set("isFullscreen", diff.isFullscreen);
      if (diff.isNavbarExpanded !== undefined) this.ySketchpad.set("isNavbarExpanded", diff.isNavbarExpanded);
      if (diff.isMobile !== undefined) this.ySketchpad.set("isMobile", diff.isMobile);
      if ("activeInteraction" in diff) this.ySketchpad.set("activeInteraction", diff.activeInteraction || "");
      if (diff.appSettings) {
        const current = JSON.parse((this.ySketchpad.get("appSettings") as string) || "{}");
        this.ySketchpad.set("appSettings", JSON.stringify({ ...current, ...diff.appSettings }));
      }
      if (diff.panelSizes) {
        const current = JSON.parse((this.ySketchpad.get("panelSizes") as string) || "{}");
        this.ySketchpad.set("panelSizes", JSON.stringify({ ...current, ...diff.panelSizes }));
      }
    });
  }

  deleteKit = (guid: Guid) => {
    const kitStore = this.kits.get(guid);
    if (kitStore) {
      this.yDoc.transact(() => {
        // Find and remove the kit metadata from yKits
        const index = this.yKits.toArray().findIndex((kitMeta) => kitMeta.get("guid") === guid);
        if (index !== -1) {
          this.yKits.delete(index, 1);
        }
      });
      this.kits.delete(guid);
      this.kitDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  deleteKitApp = (kit: Guid) => {
    const kitApp = this.kitApps.get(kit);
    if (kitApp) {
      this.kitApps.delete(kit);
      this.yDoc.transact(() => {
        this.yKitApps.delete(kit);
      });
      this.kitAppDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  deleteDesignApp = (kit: Guid, design: Guid) => {
    const designApp = this.designApps.get(kit)?.get(design);
    if (designApp) {
      this.designApps.get(kit)?.delete(design);
      if (this.designApps.get(kit)?.size === 0) {
        this.designApps.delete(kit);
      }
      this.yDoc.transact(() => {
        const yKitMap = this.yDesignApps.get(kit) as Y.Map<YDesignApp> | undefined;
        if (yKitMap) {
          yKitMap.delete(design);
          if (yKitMap.size === 0) {
            this.yDesignApps.delete(kit);
          }
        }
      });
      this.designAppDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  onKitCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitCreatedSubscribers.add(subscribe);
    return () => {
      this.kitCreatedSubscribers.delete(subscribe);
    };
  };

  onKitAppCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitAppCreatedSubscribers.add(subscribe);
    return () => {
      this.kitAppCreatedSubscribers.delete(subscribe);
    };
  };

  onDesignAppCreated = (subscribe: Subscribe): Unsubscribe => {
    this.designAppCreatedSubscribers.add(subscribe);
    return () => {
      this.designAppCreatedSubscribers.delete(subscribe);
    };
  };

  onKitDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitDeletedSubscribers.add(subscribe);
    return () => {
      this.kitDeletedSubscribers.delete(subscribe);
    };
  };

  onKitAppDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitAppDeletedSubscribers.add(subscribe);
    return () => {
      this.kitAppDeletedSubscribers.delete(subscribe);
    };
  };

  onDesignAppDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.designAppDeletedSubscribers.add(subscribe);
    return () => {
      this.designAppDeletedSubscribers.delete(subscribe);
    };
  };

  onChanged = (subscribe: Subscribe): Unsubscribe => {
    return createObserver(this.ySketchpad, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {
    return createObserver(this.ySketchpad, subscribe, true);
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.sketchpad.createKit") {
      console.log(`Executing (special) command: "${command}"`);
      const kit = rest[0] as Kit;
      const local = rest[1] as boolean | undefined;
      const remote = rest[2] as boolean | undefined;
      this.createKit(kit, local, remote);
      return {} as T;
    }
    if (command === "semio.sketchpad.createKitApp") {
      console.log(`Executing (special) command: "${command}"`);
      const id = rest[0] as KitAppId;
      this.createKitApp(id.kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createDesignApp") {
      console.log(`Executing (special) command: "${command}"`);
      const id = rest[0] as DesignAppId;
      this.createDesignApp(id.kit, id.design);
      return {} as T;
    }
    if (command === "semio.sketchpad.importKit") {
      console.log(`Executing (special) command: "${command}"`);
      const Guid = rest[0] as Guid;
      const url = rest[1] as string;
      const kitStore = this.kits.get(Guid);
      if (kitStore) {
        await kitStore.execute("semio.kit.import", url);
      }
      return {} as T;
    }
    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in sketchpad store`);
    const context: SketchpadCommandContext = {
      sketchpad: this.snapshot(),
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    console.groupEnd();
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }

  registerCommand(command: string, callback: (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }

  hasKit(guid: string): boolean {
    return this.kits.has(guid);
  }

  kit(guid: string): KitStore {
    const kitStore = this.kits.get(guid);
    if (!kitStore) {
      throw new Error(`Kit with guid ${guid} not found`);
    }
    return kitStore;
  }

  kitShallows(): KitShallow[] {
    const currentKits = Array.from(this.kits.values()).map((k) => k.snapshot() as KitShallow);
    const currentHash = JSON.stringify(currentKits);

    if (!this.kitShallowsCache || this.kitShallowsCacheHash !== currentHash) {
      this.kitShallowsCache = currentKits;
      this.kitShallowsCacheHash = currentHash;
    }

    return this.kitShallowsCache;
  }

  hasKitApp(kitApp: KitAppId): boolean {
    return hasSameKitApp(
      kitApp,
      Array.from(this.kitApps.values()).map((kitApp) => kitApp.id()),
    );
  }

  home(): HomeStoreInstance {
    if (!this.homeStore) {
      const homeFactory = resolveHomeStoreFactory();
      this.homeStore = homeFactory(this, this.yHome, this.yDoc.transact.bind(this.yDoc));
    }
    return this.homeStore;
  }

  kitApp(guid: string): KitAppStoreInstance {
    let app = this.kitApps.get(guid);
    if (!app) {
      if (!this.hasKit(guid)) {
        throw new Error(`Cannot create kit app: Kit with guid ${guid} does not exist`);
      }
      this.createKitApp(guid);
      app = this.kitApps.get(guid)!;
    }
    return app;
  }

  kitAppIds(): KitAppId[] {
    return Array.from(this.kitApps.values()).map((k) => k.id());
  }

  getAllKitApps(): KitAppStoreInstance[] {
    return Array.from(this.kitApps.values());
  }

  getAllDesignApps(): DesignAppStoreInstance[] {
    const allDesignApps: DesignAppStoreInstance[] = [];
    for (const kitMap of this.designApps.values()) {
      allDesignApps.push(...Array.from(kitMap.values()));
    }
    return allDesignApps;
  }

  createTypeApp = (kit: Guid, type: Guid) => {
    const id: TypeAppId = { kit, type };
    const key = `${kit}:${type}`;
    this.yDoc.transact(() => {
      let yTypeApp = this.yTypeApps.get(key) as Y.Map<YTypeAppVal>;
      if (!yTypeApp) {
        yTypeApp = new Y.Map<YTypeAppVal>();
        this.yTypeApps.set(key, yTypeApp);
      }
      const typeAppFactory = resolveTypeAppStoreFactory();
      const typeApp = typeAppFactory(this, yTypeApp, this.yDoc.transact.bind(this.yDoc), id);
      this.typeApps.set(key, typeApp);
    });
    this.typeAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteTypeApp = (kit: Guid, type: Guid) => {
    const key = `${kit}:${type}`;
    const typeApp = this.typeApps.get(key);
    if (typeApp) {
      this.typeApps.delete(key);
      this.yDoc.transact(() => {
        this.yTypeApps.delete(key);
      });
      this.typeAppDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  hasTypeApp(typeApp: TypeAppId): boolean {
    const key = `${typeApp.kit}:${typeApp.type}`;
    return this.typeApps.has(key);
  }

  typeApp(kit: Guid, type: Guid): TypeAppStoreInstance {
    const key = `${kit}:${type}`;
    let app = this.typeApps.get(key);
    if (!app) {
      this.createTypeApp(kit, type);
      app = this.typeApps.get(key)!;
    }
    return app;
  }

  typeAppIds(): TypeAppId[] {
    return Array.from(this.typeApps.values()).map((t) => ({ kit: t.id.kit, type: t.id.type }));
  }

  createQualityApp = (kit: Guid, quality: Guid) => {
    const Guid: QualityAppId = { kit, quality };
    const key = `${kit}:${quality}`;
    this.yDoc.transact(() => {
      let yQualityApp = this.yQualityApps.get(key) as Y.Map<YQualityAppVal>;
      if (!yQualityApp) {
        yQualityApp = new Y.Map<YQualityAppVal>();
        this.yQualityApps.set(key, yQualityApp);
      }
      const qualityAppFactory = resolveQualityAppStoreFactory();
      const qualityApp = qualityAppFactory(this, yQualityApp, this.yDoc.transact.bind(this.yDoc), Guid);
      this.qualityApps.set(key, qualityApp);
    });
    this.qualityAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteQualityApp = (kit: Guid, quality: Guid) => {
    const key = `${kit}:${quality}`;
    const qualityApp = this.qualityApps.get(key);
    if (qualityApp) {
      this.qualityApps.delete(key);
      this.yDoc.transact(() => {
        this.yQualityApps.delete(key);
      });
      this.qualityAppDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  hasQualityApp(qualityApp: QualityAppId): boolean {
    const key = `${qualityApp.kit}:${qualityApp.quality}`;
    return this.qualityApps.has(key);
  }

  qualityApp(kit: Guid, quality: Guid): QualityAppStoreInstance {
    const key = `${kit}:${quality}`;
    let app = this.qualityApps.get(key);
    if (!app) {
      this.createQualityApp(kit, quality);
      app = this.qualityApps.get(key)!;
    }
    return app;
  }

  qualityAppIds(): QualityAppId[] {
    return Array.from(this.qualityApps.values()).map((q) => ({ kit: q.Guid.kit, quality: q.Guid.quality }));
  }

  hasDesignApp(designApp: DesignAppId): boolean {
    const allDesignApps: DesignAppStoreInstance[] = [];
    for (const kitMap of this.designApps.values()) {
      allDesignApps.push(...Array.from(kitMap.values()));
    }
    return hasSameDesignApp(
      designApp,
      allDesignApps.map((designApp) => designApp.id()),
    );
  }

  designApp(kitGuid: string, designGuid: string): DesignAppStoreInstance {
    let kitMap = this.designApps.get(kitGuid);
    if (!kitMap) {
      kitMap = new Map();
      this.designApps.set(kitGuid, kitMap);
    }
    let app = kitMap.get(designGuid);
    if (!app) {
      this.createDesignApp(kitGuid, designGuid);
      app = kitMap.get(designGuid)!;
    }
    return app;
  }

  designAppIds(): DesignAppId[] {
    const allDesignApps: DesignAppStoreInstance[] = [];
    for (const kitMap of this.designApps.values()) {
      allDesignApps.push(...Array.from(kitMap.values()));
    }
    return allDesignApps.map((d) => d.id());
  }

  private async loadPersistedKits() {
    // Wait for the sketchpad's Y.Doc to sync with IndexedDB
    if (this.persistence) {
      await new Promise<void>((resolve) => {
        this.persistence!.once("synced", () => resolve());
      });
    }

    // Load kits from yKits metadata stored in the sketchpad's Y.Doc
    const kitMetadataArray = this.yKits.toArray();

    for (const kitMetadata of kitMetadataArray) {
      const kitGuid = kitMetadata.get("guid") as string;
      const local = kitMetadata.get("local") as boolean;
      const remote = kitMetadata.get("remote") as boolean;

      // Skip if kit is already loaded
      if (this.kits.has(kitGuid)) continue;

      if (local && typeof indexedDB !== "undefined") {
        // Load from IndexedDB for local/remote kits
        try {
          const yDoc = new Y.Doc();
          const persistence = new IndexeddbPersistence(`semio-kit-${kitGuid}`, yDoc);

          // Wait for persistence to load
          await new Promise<void>((resolve) => {
            persistence.on("synced", () => resolve());
          });

          // Extract kit data from the Y.Doc
          const yKit = yDoc.getMap();
          const kit: Kit = {
            guid: yKit.get("guid") as string,
            name: yKit.get("name") as string,
            version: yKit.get("version") as string,
            remote: yKit.get("remote") as string,
            homepage: yKit.get("homepage") as string,
            license: yKit.get("license") as string,
            preview: yKit.get("preview") as string,
            concepts: yKit.get("concepts") as string[],
            icon: yKit.get("icon") as string,
            image: yKit.get("image") as string,
            description: yKit.get("description") as string,
            createdAt: yKit.get("createdAt") ? new Date(yKit.get("createdAt") as string) : undefined,
            updatedAt: yKit.get("updatedAt") ? new Date(yKit.get("updatedAt") as string) : undefined,
            types: [],
            designs: [],
            files: [],
            qualities: [],
            authors: [],
            attributes: [],
          };

          // Destroy the temporary persistence and doc
          persistence.destroy();

          // Create the kit store (this will set up its own persistence)
          // Don't add to yKits again since it's already there
          const kitStore = new KitStore(this, kit, local, remote, this.yProviderFactory, this.fileProviderFactory);
          this.kits.set(kit.guid, kitStore);
          this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
        } catch (error) {}
      } else {
        this.yDoc.transact(() => {
          const index = kitMetadataArray.findIndex((meta) => meta.get("guid") === kitGuid);
          if (index !== -1) {
            this.yKits.delete(index, 1);
          }
        });
      }
    }
  }
}

// Persist stores across HMR reloads
let stores: Map<Guid, SketchpadStore>;
if (import.meta.hot?.data.stores) {
  stores = import.meta.hot.data.stores;
} else {
  stores = new Map();
  if (import.meta.hot) {
    import.meta.hot.data.stores = stores;
  }
}

// TODO: Find clean way to hide Scope and extra hook and still pass window events to navbar
export type WindowEvents = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
};
export type SketchpadScope = { id: string; yProviderFactory?: YProviderFactory; fileProviderFactory?: FileProviderFactory; onWindowEvents?: WindowEvents };
const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
export const SketchpadScopeProvider = (props: { id?: string; yProviderFactory?: YProviderFactory; fileProviderFactory?: FileProviderFactory; onWindowEvents?: WindowEvents; children: React.ReactNode }) => {
  // Use useMemo to ensure the ID is stable across re-renders when props.id is undefined
  const id = useMemo(() => props.id || guid(), [props.id]);

  if (!stores.has(id)) {
    const store = new SketchpadStore(id, props?.yProviderFactory, props?.fileProviderFactory);
    stores.set(id, store);
  }
  return React.createElement(SketchpadScopeContext.Provider, { value: { id, fileProviderFactory: props.fileProviderFactory, onWindowEvents: props.onWindowEvents } }, props.children as any);
};
export const useSketchpadScope = () => useContext(SketchpadScopeContext);

export function useSketchpadStore(id?: string): SketchpadStore {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpadStore must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  return store;
}

export function useSketchpad<T>(selector?: (state: SketchpadState) => T, id?: string): T | SketchpadState | null {
  return useSync<SketchpadState, T>(useSketchpadStore(id), selector ? selector : identitySelector);
}

export function useNavigation(): string {
  const location = useLocation();
  return location.pathname;
}

export function migratePath(path: string): string {
  if (path.match(/^\/kit\/([^/]+)\/design\/([^/]+)/)) {
    return path.replace(/^\/kit\/([^/]+)\/design\/([^/]+)/, "/kits/$1/designs/$2");
  }
  if (path.match(/^\/kit\/([^/]+)\/type\/([^/]+)/)) {
    return path.replace(/^\/kit\/([^/]+)\/type\/([^/]+)/, "/kits/$1/types/$2");
  }
  if (path.match(/^\/kit\/([^/]+)/)) {
    return path.replace(/^\/kit\/([^/]+)/, "/kits/$1");
  }
  if (path.match(/^\/kit\?/)) {
    return path.replace(/^\/kit\?/, "/kits?");
  }
  return path;
}

// Moved to appType.ts to avoid circular dependency
export { getAppTypeFromPath, useAppType } from "./appType";

export function useAccess(): Access {
  return useSketchpad((s) => s.access) as Access;
}

export function useTheme(): Theme {
  return useSketchpad((s) => s.theme) as Theme;
}

export function useLayout(): Layout {
  return useSketchpad((s) => s.layout) as Layout;
}

export function useMode(): Mode {
  return useSketchpad((s) => s.mode) as Mode;
}

export function useTooltip(): (key: string) => string | undefined {
  const mode = useMode();
  return (key: string) => {
    if (mode === Mode.EXPERT) return undefined;
    return key;
  };
}

export function useSemioTooltip() {
  const mode = useMode();
  return { mode };
}

export function useIsFullscreen(): boolean {
  return useSketchpad((s) => s.isFullscreen) as boolean;
}

export function useIsNavbarExpanded(): boolean {
  return useSketchpad((s) => s.isNavbarExpanded) as boolean;
}

export function useActiveInteraction(): string | undefined {
  return useSketchpad((s) => s.activeInteraction) as string | undefined;
}

export function useIsMobile(): boolean {
  return useSketchpad((s) => s.isMobile) as boolean;
}

export function useNavigationHistory(): {
  history: string[];
  currentIndex: number;
  canGoBack: boolean;
  canGoForward: boolean;
} {
  const history = useSketchpad((s) => s.navigationHistory) as string[];
  const currentIndex = useSketchpad((s) => s.navigationHistoryIndex) as number;
  return {
    history,
    currentIndex,
    canGoBack: currentIndex > 0,
    canGoForward: currentIndex < history.length - 1,
  };
}

export function useAppPanelVisibility(): PanelVisibility {
  const navigation = useNavigation();
  const appType = useAppType();
  const store = useSketchpadStore();

  // Parse the navigation path to get IDs
  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const appKind = pathMatch?.[2];
  const itemGuid = pathMatch?.[3];

  const docsPanelVisibility = useSyncExternalStore(subscribeDocsPanelVisibility, getDocsPanelVisibilitySnapshot, getDocsPanelVisibilitySnapshot);

  const [panelVisibility, setPanelVisibility] = useState<PanelVisibility>(() => (appType === "docs" ? docsPanelVisibility : { ...defaultPanelVisibility }));

  useEffect(() => {
    if (appType === "docs") {
      setPanelVisibility(docsPanelVisibility);
      return;
    }

    try {
      let app: any;
      switch (appType) {
        case "home":
          app = store.home();
          break;
        case "kit":
          if (kitGuid) {
            app = store.kitApp(kitGuid);
          } else {
          }
          break;
        case "design":
          if (kitGuid && itemGuid) app = store.designApp(kitGuid, itemGuid);
          break;
        case "type":
          if (kitGuid && itemGuid) app = store.typeApp(kitGuid, itemGuid);
          break;
        case "quality":
          if (kitGuid && itemGuid) app = store.qualityApp(kitGuid, itemGuid);
          break;
        default:
      }

      if (app) {
        const unsubscribe = app.onChangedDeep(() => {
          const newPanelVisibility = app.snapshot().panelVisibility || { ...defaultPanelVisibility };
          setPanelVisibility(newPanelVisibility);
        });

        const initialPanelVisibility = app.snapshot().panelVisibility || { ...defaultPanelVisibility };
        setPanelVisibility(initialPanelVisibility);

        return unsubscribe;
      }
    } catch (e) {}
  }, [store, appType, kitGuid, itemGuid, navigation, docsPanelVisibility]);

  return panelVisibility;
}

export function useAppCommands() {
  const navigation = useNavigation();
  const appType = useAppType();
  const store = useSketchpadStore();

  // Parse the navigation path to get IDs
  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const itemGuid = pathMatch?.[3];

  return useMemo(() => {
    let app: any;
    try {
      switch (appType) {
        case "home":
          app = store.home();
          break;
        case "kit":
          if (kitGuid) app = store.kitApp(kitGuid);
          break;
        case "design":
          if (kitGuid && itemGuid) app = store.designApp(kitGuid, itemGuid);
          break;
        case "type":
          if (kitGuid && itemGuid) app = store.typeApp(kitGuid, itemGuid);
          break;
        case "quality":
          if (kitGuid && itemGuid) app = store.qualityApp(kitGuid, itemGuid);
          break;
        case "docs":
          return {
            togglePanel: (panelKey: keyof PanelVisibility) => {
              updateDocsPanelVisibilityState((prev) => ({
                ...prev,
                [panelKey]: !prev[panelKey],
              }));
            },
            execute: (command: string, ...args: any[]) => {},
          };
      }
    } catch (e) {}

    return {
      togglePanel: (panelKey: keyof PanelVisibility) => {
        if (!app) {
          return;
        }
        const current = app.snapshot().panelVisibility;
        try {
          app.change({
            panelVisibility: {
              [panelKey]: !current[panelKey],
            },
          });
        } catch (e) {}
      },
      execute: (command: string, ...args: any[]) => {
        if (!app) return;
        return app.execute(command, ...args);
      },
    };
  }, [store, appType, kitGuid, itemGuid, navigation]);
}

export function useSketchpadCommands() {
  const store = useSketchpadStore();
  const navigate = useNavigate();
  return useMemo(
    () => ({
      setAccess: (access: Access) => store.execute("semio.sketchpad.setAccess", access),
      setTheme: (theme: Theme) => store.execute("semio.sketchpad.setTheme", theme),
      setLayout: (layout: Layout) => store.execute("semio.sketchpad.setLayout", layout),
      setMode: (mode: Mode) => store.execute("semio.sketchpad.setMode", mode),
      toggleFullscreen: () => store.execute("semio.sketchpad.toggleFullscreen"),
      toggleNavbarExpanded: () => store.execute("semio.sketchpad.toggleNavbarExpanded"),
      setIsMobile: (isMobile: boolean) => store.execute("semio.sketchpad.setIsMobile", isMobile),
      setActiveInteraction: (interactionId?: string) => store.execute("semio.sketchpad.setActiveInteraction", interactionId),
      syncNavigation: (path: string) => store.execute("semio.sketchpad.syncNavigation", path),
      createKit: (kit: Kit, local?: boolean, remote?: boolean) => store.execute("semio.sketchpad.createKit", kit, local, remote),
      createKitApp: (kitAppId: KitAppId) => store.execute("semio.sketchpad.createKitApp", kitAppId),
      createDesignApp: (designAppId: DesignAppId) => store.execute("semio.sketchpad.createDesignApp", designAppId),
      navigateToKit: (kit: Guid, search?: string) => navigate(`/kits/${kit}${search ? (search.startsWith("?") ? search : `?${search}`) : ""}`),
      navigateToDesign: (kit: Guid, design: Guid) => navigate(`/kits/${kit}/designs/${design}`),
      navigateToType: (kit: Guid, type: Guid) => navigate(`/kits/${kit}/types/${type}`),
      navigateToQuality: (kit: Guid, quality: Guid) => navigate(`/kits/${kit}/qualities/${quality}`),
      navigateBack: () => {
        store.execute("semio.sketchpad.navigateBack");
        const state = store.snapshot();
        const targetPath = state.navigationHistory[state.navigationHistoryIndex];
        if (targetPath) {
          navigate(targetPath);
        }
      },
      navigateForward: () => {
        store.execute("semio.sketchpad.navigateForward");
        const state = store.snapshot();
        const targetPath = state.navigationHistory[state.navigationHistoryIndex];
        if (targetPath) {
          navigate(targetPath);
        }
      },
      updateAppSettings: (appType: "design" | "type" | "kit", settings: Record<string, any>) => {
        const current = store.snapshot().appSettings;
        store.change({
          appSettings: {
            ...current,
            [appType]: { ...current[appType], ...settings },
          },
        });
      },
      setPanelSize: (panelKey: keyof PanelSizes, size: number) => {
        store.change({
          panelSizes: {
            [panelKey]: size,
          },
        });
      },
    }),
    [store, navigate],
  );
}

export function useKits(): KitShallow[] {
  const store = useSketchpadStore();

  const kits = useSyncExternalStore(
    (onStoreChange) => {
      const unsubscribeCreated = store.onKitCreated(onStoreChange);
      const unsubscribeDeleted = store.onKitDeleted(onStoreChange);
      const unsubscribers = store.kitShallows().map((kitShallow) => {
        const kitStore = store.kit(kitShallow.guid);
        return kitStore.onChanged(onStoreChange);
      });
      return () => {
        unsubscribeCreated();
        unsubscribeDeleted();
        unsubscribers.forEach((unsub) => unsub());
      };
    },
    () => store.kitShallows(),
  );

  return kits;
}

// #endregion Sketchpad
