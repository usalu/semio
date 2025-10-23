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
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate } from "react-router";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { areSameKit, guid, Guid, inverseKitDiff, Kit, KitDiff, KitShallow } from "../semio";
import { commands as sketchpadCommands } from "./commands";
import { editorRegistry } from "./editors/registry";
import type { DesignEditorId, DesignEditorState } from "./editors/design/store";
import type { KitEditorId, KitEditorState } from "./editors/kit/store";
import type { QualityEditorId, QualityEditorState } from "./editors/quality/store";
import type { TypeEditorId, TypeEditorState } from "./editors/type/store";
import { KitStore } from "./kits/store";
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
  useClusterableGroups,
  useConnection,
  useConnectionStatus,
  useDesign,
  useDesignId,
  useDesigns,
  useDesignScope,
  useDiffedDesign,
  useDiffedKit,
  useExplodeableDesignNodes,
  useFileUrls,
  useFlatDesign,
  useFlatPieces,
  useFlattenDiff,
  useIncludedDesigns,
  useIsConnectionHovered,
  useIsConnectionSelected,
  useIsInDesignScope,
  useIsInKitScope,
  useIsInQualityScope,
  useIsInTypeScope,
  useIsPieceHovered,
  useIsPieceSelected,
  useIsPieceTransitiveHovered,
  useKit,
  useKitCommands,
  useKitCommandsSafe,
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
  usePieceStatus,
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

export type EditorType = string;

export enum ToolType {
  // Selection tools
  SELECTION_NORMAL = "selection-normal",
  SELECTION_ADDITIVE = "selection-additive",
  SELECTION_SUBTRACTIVE = "selection-subtractive",
  // Lasso tools
  LASSO_RECTANGULAR = "lasso-rectangular",
  LASSO_FREEFORM = "lasso-freeform",
  // Type editor tools
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

export abstract class Store<TState> {
  public readonly guid: Guid;
  public readonly parent: SketchpadStore;
  public readonly yMap: Y.Map<any>;
  protected readonly transact: Transact;
  protected cache?: TState;
  protected cacheHash?: string;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact) {
    this.guid = guid();
    this.parent = parent;
    this.yMap = yMap;
    this.transact = transact;
  }

  protected abstract hash(state: TState): string;
  protected abstract buildSnapshot(): TState;

  snapshot(): TState {
    const currentData = this.buildSnapshot();
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  }

  onChanged(subscribe: Subscribe): Unsubscribe {
    return createObserver(this.yMap, subscribe);
  }

  onChangedDeep(subscribe: Subscribe): Unsubscribe {
    return createObserver(this.yMap, subscribe, true);
  }
}

export interface EditorStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

export interface EditorEdit<TSelectionDiff = any> {
  do: EditorStep<TSelectionDiff>;
  undo: EditorStep<TSelectionDiff>;
}

export interface EditorDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenWindow?: any;
  panelVisibility?: Partial<PanelVisibility>;
}

export interface EditorCommandResult<TDiff = any> {
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

export abstract class EditorStore<TState, TDiff extends EditorDiff<TSelectionDiff>, TSelectionDiff, TEdit extends EditorEdit<TSelectionDiff>, TCommandContext, TCommandResult extends EditorCommandResult<TDiff>> extends Store<TState> {
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
      const doStep: EditorStep<TSelectionDiff> = { selectionDiff: result.diff?.selection };
      const undoStep: EditorStep<TSelectionDiff> = { selectionDiff: inversedSelectionDiff };
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

export interface KitDiffEditorStep<TSelectionDiff = any> extends EditorStep<TSelectionDiff> {
  kitDiff?: KitDiff;
}

export interface KitDiffEditorEdit<TSelectionDiff = any> {
  do: KitDiffEditorStep<TSelectionDiff>;
  undo: KitDiffEditorStep<TSelectionDiff>;
}

export interface KitDiffEditorCommandResult<TDiff = any> extends EditorCommandResult<TDiff> {
  kitDiff?: KitDiff;
}

export abstract class KitDiffEditorStore<
  TState,
  TDiff extends EditorDiff<TSelectionDiff>,
  TSelectionDiff,
  TEdit extends KitDiffEditorEdit<TSelectionDiff>,
  TCommandContext,
  TCommandResult extends KitDiffEditorCommandResult<TDiff>,
> extends EditorStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult> {
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
      const doStep: KitDiffEditorStep<TSelectionDiff> = { kitDiff: result.kitDiff, selectionDiff: result.diff?.selection };
      const undoStep: KitDiffEditorStep<TSelectionDiff> = { kitDiff: inversedKitDiff, selectionDiff: inversedSelectionDiff };
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

function areSameDesignEditor(designEditor: DesignEditorId, other: DesignEditorId): boolean {
  return !!designEditor && !!other && areSameKit(designEditor.kit, other.kit) && designEditor.design === other.design;
}

function hasSameDesignEditor(designEditor: DesignEditorId, others: DesignEditorId[]): boolean {
  return others.some((other) => areSameDesignEditor(designEditor, other));
}

function areSameKitEditor(kitEditor: KitEditorId, other: KitEditorId): boolean {
  return !!kitEditor && !!other && areSameKit(kitEditor.kit, other.kit);
}

function hasSameKitEditor(kitEditor: KitEditorId, others: KitEditorId[]): boolean {
  return others.some((other) => areSameKitEditor(kitEditor, other));
}

type YKitEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitEditor = Y.Map<YKitEditorVal>;
type YKitEditors = Y.Map<YKitEditor>;

type YDesignEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignEditor = Y.Map<YDesignEditorVal>;
type YDesignEditors = Y.Map<Y.Map<YDesignEditor>>;

type YTypeEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YTypeEditor = Y.Map<YTypeEditorVal>;
type YTypeEditors = Y.Map<YTypeEditor>;

type YQualityEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YQualityEditor = Y.Map<YQualityEditorVal>;
type YQualityEditors = Y.Map<YQualityEditor>;

type YKitMetadata = Y.Map<string | boolean>;
type YKits = Y.Array<YKitMetadata>;

type KitEditorStoreInstance = any;
type DesignEditorStoreInstance = any;
type TypeEditorStoreInstance = any;
type QualityEditorStoreInstance = any;
type HomeStoreInstance = any;

type KitEditorStoreFactory = (parent: SketchpadStore, yMap: YKitEditor, transact: (fn: () => void) => void, id: KitEditorId, state?: KitEditorState) => KitEditorStoreInstance;
type DesignEditorStoreFactory = (parent: SketchpadStore, yMap: YDesignEditor, transact: (fn: () => void) => void, id: DesignEditorId, state?: DesignEditorState) => DesignEditorStoreInstance;
type TypeEditorStoreFactory = (parent: SketchpadStore, yMap: YTypeEditor, transact: (fn: () => void) => void, id: TypeEditorId, state?: TypeEditorState) => TypeEditorStoreInstance;
type QualityEditorStoreFactory = (parent: SketchpadStore, yMap: YQualityEditor, transact: (fn: () => void) => void, id: QualityEditorId, state?: QualityEditorState) => QualityEditorStoreInstance;
type HomeStoreFactory = (parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) => HomeStoreInstance;

let kitEditorStoreFactory: KitEditorStoreFactory | undefined;
let designEditorStoreFactory: DesignEditorStoreFactory | undefined;
let typeEditorStoreFactory: TypeEditorStoreFactory | undefined;
let qualityEditorStoreFactory: QualityEditorStoreFactory | undefined;
let homeStoreFactory: HomeStoreFactory | undefined;

export function registerKitEditorStoreFactory(factory: KitEditorStoreFactory) {
  kitEditorStoreFactory = factory;
}

export function registerDesignEditorStoreFactory(factory: DesignEditorStoreFactory) {
  designEditorStoreFactory = factory;
}

export function registerTypeEditorStoreFactory(factory: TypeEditorStoreFactory) {
  typeEditorStoreFactory = factory;
}

export function registerQualityEditorStoreFactory(factory: QualityEditorStoreFactory) {
  qualityEditorStoreFactory = factory;
}

export function registerHomeStoreFactory(factory: HomeStoreFactory) {
  homeStoreFactory = factory;
}

function resolveKitEditorStoreFactory(): KitEditorStoreFactory {
  if (!kitEditorStoreFactory) throw new Error("Kit editor store factory not registered");
  return kitEditorStoreFactory;
}

function resolveDesignEditorStoreFactory(): DesignEditorStoreFactory {
  if (!designEditorStoreFactory) throw new Error("Design editor store factory not registered");
  return designEditorStoreFactory;
}

function resolveTypeEditorStoreFactory(): TypeEditorStoreFactory {
  if (!typeEditorStoreFactory) throw new Error("Type editor store factory not registered");
  return typeEditorStoreFactory;
}

function resolveQualityEditorStoreFactory(): QualityEditorStoreFactory {
  if (!qualityEditorStoreFactory) throw new Error("Quality editor store factory not registered");
  return qualityEditorStoreFactory;
}

function resolveHomeStoreFactory(): HomeStoreFactory {
  if (!homeStoreFactory) throw new Error("Home store factory not registered");
  return homeStoreFactory;
}

// #endregion General

// #region Sketchpad

type YSketchpadVal = string | number | boolean | YDesignEditors;
type YSketchpad = Y.Map<YSketchpadVal>;

export interface EditorSettings {
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
  access: Access;
  theme: Theme;
  layout: Layout;
  mode: Mode;
  editorSettings: EditorSettings;
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
  access?: Access;
  theme?: Theme;
  layout?: Layout;
  mode?: Mode;
  editorSettings?: EditorSettings;
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
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly kits: Map<string, KitStore>;
  private readonly yKits: YKits;
  private readonly yHome: Y.Map<any>;
  private homeStore?: HomeStoreInstance;
  private readonly yKitEditors: YKitEditors;
  private readonly kitEditors: Map<string, KitEditorStoreInstance>;
  private readonly yTypeEditors: YTypeEditors;
  private readonly typeEditors: Map<string, TypeEditorStoreInstance>;
  private readonly yQualityEditors: YQualityEditors;
  private readonly qualityEditors: Map<string, QualityEditorStoreInstance>;
  private readonly yDesignEditors: YDesignEditors;
  private readonly designEditors: Map<string, Map<string, DesignEditorStoreInstance>>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult>;
  private cache?: SketchpadState;
  private cacheHash?: string;
  private kitShallowsCache?: KitShallow[];
  private kitShallowsCacheHash?: string;
  private readonly kitCreatedSubscribers: Set<Subscribe>;
  private readonly kitDeletedSubscribers: Set<Subscribe>;
  private readonly kitEditorCreatedSubscribers: Set<Subscribe>;
  private readonly kitEditorDeletedSubscribers: Set<Subscribe>;
  private readonly typeEditorCreatedSubscribers: Set<Subscribe>;
  private readonly typeEditorDeletedSubscribers: Set<Subscribe>;
  private readonly qualityEditorCreatedSubscribers: Set<Subscribe>;
  private readonly qualityEditorDeletedSubscribers: Set<Subscribe>;
  private readonly designEditorCreatedSubscribers: Set<Subscribe>;
  private readonly designEditorDeletedSubscribers: Set<Subscribe>;
  // private readonly broadcastChannel: BroadcastChannel;

  constructor(id?: string, yProviderFactory?: YProviderFactory) {
    this.id = id;
    this.yProviderFactory = yProviderFactory;
    // this.broadcastChannel = new BroadcastChannel(`semio-sketchpad-${id}`);
    this.yDoc = new Y.Doc();
    this.kits = new Map();
    this.kitEditors = new Map();
    this.typeEditors = new Map();
    this.qualityEditors = new Map();
    this.designEditors = new Map();
    this.commandRegistry = new Map();
    this.kitCreatedSubscribers = new Set();
    this.kitDeletedSubscribers = new Set();
    this.kitEditorCreatedSubscribers = new Set();
    this.kitEditorDeletedSubscribers = new Set();
    this.typeEditorCreatedSubscribers = new Set();
    this.typeEditorDeletedSubscribers = new Set();
    this.qualityEditorCreatedSubscribers = new Set();
    this.qualityEditorDeletedSubscribers = new Set();
    this.designEditorCreatedSubscribers = new Set();
    this.designEditorDeletedSubscribers = new Set();

    if (id) {
      this.persistence = new IndexeddbPersistence(`semio-sketchpad-${id}`, this.yDoc);
      if (yProviderFactory) {
        yProviderFactory(this.yDoc, id);
      }
    }

    this.ySketchpad = this.yDoc.getMap("sketchpad");
    this.yKits = this.yDoc.getArray("kits");
    this.yHome = this.yDoc.getMap("home");
    this.yKitEditors = this.yDoc.getMap("kitEditors");
    this.yTypeEditors = this.yDoc.getMap("typeEditors");
    this.yQualityEditors = this.yDoc.getMap("qualityEditors");
    this.yDesignEditors = this.yDoc.getMap("designEditors");

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
        this.ySketchpad.set("mode", Mode.NORMAL);
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
      if (!this.ySketchpad.has("editorSettings")) {
        this.ySketchpad.set(
          "editorSettings",
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
    const editorSettingsStr = this.ySketchpad.get("editorSettings") as string;
    const editorSettings = editorSettingsStr
      ? JSON.parse(editorSettingsStr)
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
    const currentValues = {
      navigation: migratePath((this.ySketchpad.get("navigation") as string) || "/"),
      navigationHistory: navigationHistory,
      navigationHistoryIndex: (this.ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
      access: this.ySketchpad.get("access") as Access,
      theme: this.ySketchpad.get("theme") as Theme,
      layout: this.ySketchpad.get("layout") as Layout,
      mode: (this.ySketchpad.get("mode") as Mode) ?? Mode.NORMAL,
      editorSettings: editorSettings,
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
    const kitStore = new KitStore(this, kit, local, remote, this.yProviderFactory);
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

  createKitEditor = (kit: Guid) => {
    this.yDoc.transact(() => {
      const kitStore = this.kit(kit);
      let yKitEditor = this.yKitEditors.get(kit) as Y.Map<YKitEditorVal>;
      if (!yKitEditor) {
        yKitEditor = new Y.Map<YKitEditorVal>();
        this.yKitEditors.set(kit, yKitEditor);
      }
      const kitEditorFactory = resolveKitEditorStoreFactory();
      const kitEditor = kitEditorFactory(this, yKitEditor, this.yDoc.transact.bind(this.yDoc), { kit });
      this.kitEditors.set(kit, kitEditor);
    });
    this.kitEditorCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createDesignEditor = (kit: Guid, design: Guid) => {
    this.yDoc.transact(() => {
      let yKitMap = this.yDesignEditors.get(kit) as Y.Map<YDesignEditor>;
      if (!yKitMap) {
        yKitMap = new Y.Map<YDesignEditor>();
        this.yDesignEditors.set(kit, yKitMap);
      }
      let yDesignEditor = yKitMap.get(design) as Y.Map<YDesignEditorVal>;
      if (!yDesignEditor) {
        yDesignEditor = new Y.Map<YDesignEditorVal>();
        yKitMap.set(design, yDesignEditor);
      }
      const designEditorFactory = resolveDesignEditorStoreFactory();
      const designEditor = designEditorFactory(this, yDesignEditor, this.yDoc.transact.bind(this.yDoc), { kit, design });

      // Ensure the design editors map exists for this kit
      let designEditorsMap = this.designEditors.get(kit);
      if (!designEditorsMap) {
        designEditorsMap = new Map();
        this.designEditors.set(kit, designEditorsMap);
      }
      designEditorsMap.set(design, designEditor);
    });
    this.designEditorCreatedSubscribers.forEach((subscriber) => subscriber());
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
      if (diff.access) this.ySketchpad.set("access", diff.access);
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.layout) this.ySketchpad.set("layout", diff.layout);
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.isFullscreen !== undefined) this.ySketchpad.set("isFullscreen", diff.isFullscreen);
      if (diff.isNavbarExpanded !== undefined) this.ySketchpad.set("isNavbarExpanded", diff.isNavbarExpanded);
      if (diff.isMobile !== undefined) this.ySketchpad.set("isMobile", diff.isMobile);
      if ("activeInteraction" in diff) this.ySketchpad.set("activeInteraction", diff.activeInteraction || "");
      if (diff.editorSettings) {
        const current = JSON.parse((this.ySketchpad.get("editorSettings") as string) || "{}");
        this.ySketchpad.set("editorSettings", JSON.stringify({ ...current, ...diff.editorSettings }));
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

  deleteKitEditor = (kit: Guid) => {
    const kitEditor = this.kitEditors.get(kit);
    if (kitEditor) {
      this.kitEditors.delete(kit);
      this.yDoc.transact(() => {
        this.yKitEditors.delete(kit);
      });
      this.kitEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  deleteDesignEditor = (kit: Guid, design: Guid) => {
    const designEditor = this.designEditors.get(kit)?.get(design);
    if (designEditor) {
      this.designEditors.get(kit)?.delete(design);
      if (this.designEditors.get(kit)?.size === 0) {
        this.designEditors.delete(kit);
      }
      this.yDoc.transact(() => {
        const yKitMap = this.yDesignEditors.get(kit) as Y.Map<YDesignEditor> | undefined;
        if (yKitMap) {
          yKitMap.delete(design);
          if (yKitMap.size === 0) {
            this.yDesignEditors.delete(kit);
          }
        }
      });
      this.designEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  onKitCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitCreatedSubscribers.add(subscribe);
    return () => {
      this.kitCreatedSubscribers.delete(subscribe);
    };
  };

  onKitEditorCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitEditorCreatedSubscribers.add(subscribe);
    return () => {
      this.kitEditorCreatedSubscribers.delete(subscribe);
    };
  };

  onDesignEditorCreated = (subscribe: Subscribe): Unsubscribe => {
    this.designEditorCreatedSubscribers.add(subscribe);
    return () => {
      this.designEditorCreatedSubscribers.delete(subscribe);
    };
  };

  onKitDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitDeletedSubscribers.add(subscribe);
    return () => {
      this.kitDeletedSubscribers.delete(subscribe);
    };
  };

  onKitEditorDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitEditorDeletedSubscribers.add(subscribe);
    return () => {
      this.kitEditorDeletedSubscribers.delete(subscribe);
    };
  };

  onDesignEditorDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.designEditorDeletedSubscribers.add(subscribe);
    return () => {
      this.designEditorDeletedSubscribers.delete(subscribe);
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
    if (command === "semio.sketchpad.createKitEditor") {
      console.log(`Executing (special) command: "${command}"`);
      const id = rest[0] as KitEditorId;
      this.createKitEditor(id.kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createDesignEditor") {
      console.log(`Executing (special) command: "${command}"`);
      const id = rest[0] as DesignEditorId;
      this.createDesignEditor(id.kit, id.design);
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

  hasKitEditor(kitEditor: KitEditorId): boolean {
    return hasSameKitEditor(
      kitEditor,
      Array.from(this.kitEditors.values()).map((kitEditor) => kitEditor.id()),
    );
  }

  home(): HomeStoreInstance {
    if (!this.homeStore) {
      const homeFactory = resolveHomeStoreFactory();
      this.homeStore = homeFactory(this, this.yHome, this.yDoc.transact.bind(this.yDoc));
    }
    return this.homeStore;
  }

  kitEditor(guid: string): KitEditorStoreInstance {
    let editor = this.kitEditors.get(guid);
    if (!editor) {
      if (!this.hasKit(guid)) {
        throw new Error(`Cannot create kit editor: Kit with guid ${guid} does not exist`);
      }
      this.createKitEditor(guid);
      editor = this.kitEditors.get(guid)!;
    }
    return editor;
  }

  kitEditorIds(): KitEditorId[] {
    return Array.from(this.kitEditors.values()).map((k) => k.id());
  }

  getAllKitEditors(): KitEditorStoreInstance[] {
    return Array.from(this.kitEditors.values());
  }

  getAllDesignEditors(): DesignEditorStoreInstance[] {
    const allDesignEditors: DesignEditorStoreInstance[] = [];
    for (const kitMap of this.designEditors.values()) {
      allDesignEditors.push(...Array.from(kitMap.values()));
    }
    return allDesignEditors;
  }

  createTypeEditor = (kit: Guid, type: Guid) => {
    const id: TypeEditorId = { kit, type };
    const key = `${kit}:${type}`;
    this.yDoc.transact(() => {
      let yTypeEditor = this.yTypeEditors.get(key) as Y.Map<YTypeEditorVal>;
      if (!yTypeEditor) {
        yTypeEditor = new Y.Map<YTypeEditorVal>();
        this.yTypeEditors.set(key, yTypeEditor);
      }
      const typeEditorFactory = resolveTypeEditorStoreFactory();
      const typeEditor = typeEditorFactory(this, yTypeEditor, this.yDoc.transact.bind(this.yDoc), id);
      this.typeEditors.set(key, typeEditor);
    });
    this.typeEditorCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteTypeEditor = (kit: Guid, type: Guid) => {
    const key = `${kit}:${type}`;
    const typeEditor = this.typeEditors.get(key);
    if (typeEditor) {
      this.typeEditors.delete(key);
      this.yDoc.transact(() => {
        this.yTypeEditors.delete(key);
      });
      this.typeEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  hasTypeEditor(typeEditor: TypeEditorId): boolean {
    const key = `${typeEditor.kit}:${typeEditor.type}`;
    return this.typeEditors.has(key);
  }

  typeEditor(kit: Guid, type: Guid): TypeEditorStoreInstance {
    const key = `${kit}:${type}`;
    let editor = this.typeEditors.get(key);
    if (!editor) {
      this.createTypeEditor(kit, type);
      editor = this.typeEditors.get(key)!;
    }
    return editor;
  }

  typeEditorIds(): TypeEditorId[] {
    return Array.from(this.typeEditors.values()).map((t) => ({ kit: t.id.kit, type: t.id.type }));
  }

  createQualityEditor = (kit: Guid, quality: Guid) => {
    const Guid: QualityEditorId = { kit, quality };
    const key = `${kit}:${quality}`;
    this.yDoc.transact(() => {
      let yQualityEditor = this.yQualityEditors.get(key) as Y.Map<YQualityEditorVal>;
      if (!yQualityEditor) {
        yQualityEditor = new Y.Map<YQualityEditorVal>();
        this.yQualityEditors.set(key, yQualityEditor);
      }
      const qualityEditorFactory = resolveQualityEditorStoreFactory();
      const qualityEditor = qualityEditorFactory(this, yQualityEditor, this.yDoc.transact.bind(this.yDoc), Guid);
      this.qualityEditors.set(key, qualityEditor);
    });
    this.qualityEditorCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteQualityEditor = (kit: Guid, quality: Guid) => {
    const key = `${kit}:${quality}`;
    const qualityEditor = this.qualityEditors.get(key);
    if (qualityEditor) {
      this.qualityEditors.delete(key);
      this.yDoc.transact(() => {
        this.yQualityEditors.delete(key);
      });
      this.qualityEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  hasQualityEditor(qualityEditor: QualityEditorId): boolean {
    const key = `${qualityEditor.kit}:${qualityEditor.quality}`;
    return this.qualityEditors.has(key);
  }

  qualityEditor(kit: Guid, quality: Guid): QualityEditorStoreInstance {
    const key = `${kit}:${quality}`;
    let editor = this.qualityEditors.get(key);
    if (!editor) {
      this.createQualityEditor(kit, quality);
      editor = this.qualityEditors.get(key)!;
    }
    return editor;
  }

  qualityEditorIds(): QualityEditorId[] {
    return Array.from(this.qualityEditors.values()).map((q) => ({ kit: q.Guid.kit, quality: q.Guid.quality }));
  }

  hasDesignEditor(designEditor: DesignEditorId): boolean {
    const allDesignEditors: DesignEditorStoreInstance[] = [];
    for (const kitMap of this.designEditors.values()) {
      allDesignEditors.push(...Array.from(kitMap.values()));
    }
    return hasSameDesignEditor(
      designEditor,
      allDesignEditors.map((designEditor) => designEditor.id()),
    );
  }

  designEditor(kitGuid: string, designGuid: string): DesignEditorStoreInstance {
    let kitMap = this.designEditors.get(kitGuid);
    if (!kitMap) {
      kitMap = new Map();
      this.designEditors.set(kitGuid, kitMap);
    }
    let editor = kitMap.get(designGuid);
    if (!editor) {
      this.createDesignEditor(kitGuid, designGuid);
      editor = kitMap.get(designGuid)!;
    }
    return editor;
  }

  designEditorIds(): DesignEditorId[] {
    const allDesignEditors: DesignEditorStoreInstance[] = [];
    for (const kitMap of this.designEditors.values()) {
      allDesignEditors.push(...Array.from(kitMap.values()));
    }
    return allDesignEditors.map((d) => d.id());
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
          const kitStore = new KitStore(this, kit, local, remote, this.yProviderFactory);
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
export type SketchpadScope = { id: string; yProviderFactory?: YProviderFactory; onWindowEvents?: WindowEvents };
const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
export const SketchpadScopeProvider = (props: { id?: string; yProviderFactory?: YProviderFactory; onWindowEvents?: WindowEvents; children: React.ReactNode }) => {
  // Use useMemo to ensure the ID is stable across re-renders when props.id is undefined
  const id = useMemo(() => props.id || guid(), [props.id]);

  if (!stores.has(id)) {
    const store = new SketchpadStore(id, props?.yProviderFactory);
    stores.set(id, store);
  }
  return React.createElement(SketchpadScopeContext.Provider, { value: { id, onWindowEvents: props.onWindowEvents } }, props.children as any);
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

export function getEditorTypeFromPath(path: string): EditorType {
  const pathParts = path.split("/").filter((p) => p);
  const editor = editorRegistry.getEditorForPath(pathParts);
  return editor?.id || "home";
}

export function useEditorType(): EditorType {
  const navigation = useNavigation();
  return useMemo(() => getEditorTypeFromPath(navigation), [navigation]);
}

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
  const { t } = useTranslation();
  const mode = useMode();
  return (key: string) => {
    if (mode === Mode.EXPERT) return undefined;
    if (mode === Mode.BEGINNER) {
      const extensiveKey = `${key}.extensive`;
      return t(extensiveKey) !== extensiveKey ? t(extensiveKey) : t(key);
    }
    return t(key);
  };
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

export function useEditorPanelVisibility(): PanelVisibility {
  const navigation = useNavigation();
  const editorType = useEditorType();
  const store = useSketchpadStore();

  // Parse the navigation path to get IDs
  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const editorKind = pathMatch?.[2];
  const itemGuid = pathMatch?.[3];

  const [panelVisibility, setPanelVisibility] = useState<PanelVisibility>({
    toolbar: true,
    workbench: true,
    details: true,
    chat: true,
    settings: true,
  });

  useEffect(() => {
    try {
      let editor: any;
      switch (editorType) {
        case "home":
          editor = store.home();
          break;
        case "kit":
          if (kitGuid) {
            editor = store.kitEditor(kitGuid);
          } else {
          }
          break;
        case "design":
          if (kitGuid && itemGuid) editor = store.designEditor(kitGuid, itemGuid);
          break;
        case "type":
          if (kitGuid && itemGuid) editor = store.typeEditor(kitGuid, itemGuid);
          break;
        case "quality":
          if (kitGuid && itemGuid) editor = store.qualityEditor(kitGuid, itemGuid);
          break;
        default:
      }

      if (editor) {
        const unsubscribe = editor.onChangedDeep(() => {
          const newPanelVisibility = editor.snapshot().panelVisibility || {
            toolbar: true,
            workbench: true,
            details: true,
            chat: true,
            settings: true,
          };
          setPanelVisibility(newPanelVisibility);
        });

        const initialPanelVisibility = editor.snapshot().panelVisibility || {
          toolbar: true,
          workbench: true,
          details: true,
          chat: true,
          settings: true,
        };
        setPanelVisibility(initialPanelVisibility);

        return unsubscribe;
      }
    } catch (e) {}
  }, [store, editorType, kitGuid, itemGuid, navigation]);

  return panelVisibility;
}

export function useEditorCommands() {
  const navigation = useNavigation();
  const editorType = useEditorType();
  const store = useSketchpadStore();

  // Parse the navigation path to get IDs
  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const itemGuid = pathMatch?.[3];

  return useMemo(() => {
    let editor: any;
    try {
      switch (editorType) {
        case "home":
          editor = store.home();
          break;
        case "kit":
          if (kitGuid) editor = store.kitEditor(kitGuid);
          break;
        case "design":
          if (kitGuid && itemGuid) editor = store.designEditor(kitGuid, itemGuid);
          break;
        case "type":
          if (kitGuid && itemGuid) editor = store.typeEditor(kitGuid, itemGuid);
          break;
        case "quality":
          if (kitGuid && itemGuid) editor = store.qualityEditor(kitGuid, itemGuid);
          break;
      }
    } catch (e) {}

    return {
      togglePanel: (panelKey: keyof PanelVisibility) => {
        if (!editor) {
          return;
        }
        const current = editor.snapshot().panelVisibility;
        try {
          editor.change({
            panelVisibility: {
              [panelKey]: !current[panelKey],
            },
          });
        } catch (e) {}
      },
      execute: (command: string, ...args: any[]) => {
        if (!editor) return;
        return editor.execute(command, ...args);
      },
    };
  }, [store, editorType, kitGuid, itemGuid, navigation]);
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
      createKitEditor: (kitEditorId: KitEditorId) => store.execute("semio.sketchpad.createKitEditor", kitEditorId),
      createDesignEditor: (designEditorId: DesignEditorId) => store.execute("semio.sketchpad.createDesignEditor", designEditorId),
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
      updateEditorSettings: (editorType: "design" | "type" | "kit", settings: Record<string, any>) => {
        const current = store.snapshot().editorSettings;
        store.change({
          editorSettings: {
            ...current,
            [editorType]: { ...current[editorType], ...settings },
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
