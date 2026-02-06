// #region 🔖Header

// js/semio/sketchpad/Sketchpad.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

// #region 🔖Imports

import { closestCenter, DndContext, DragOverlay, PointerSensor, pointerWithin, rectIntersection, useSensor, useSensors } from "@dnd-kit/core";
import {
  AwardIcon,
  DocumentIcon,
  MessageCircle as FeedbackIcon,
  FocusIcon,
  HomeIcon,
  LayoutIcon,
  LocalKitIcon,
  Maximize2Icon,
  Minimize2Icon,
  NavigateBackIcon,
  NavigateForwardIcon,
  NavigateUpIcon,
  RemoteKitIcon,
  SearchIcon,
  TemporaryKitIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
} from "@semio/assets";
import { useSelector } from "@xstate/react";
import Fuse, { FuseResult } from "fuse.js";
import React, { ComponentType, createContext, FC, ReactNode, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { createPortal } from "react-dom";
import { createRoot } from "react-dom/client";
import { useHotkeys as useReactHotkeys } from "react-hotkeys-hook";
import { useTranslation as useI18nTranslation } from "react-i18next";
import { BrowserRouter, MemoryRouter, Outlet, Route, Routes, useLocation, useParams, useNavigate as useReactNavigate, useSearchParams } from "react-router";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import i18n, { useHotkey, useLabel } from "../i18n";
import {
  applyKitDiff,
  Attribute,
  Author,
  AuthorDiff,
  Benchmark,
  BenchmarkDiff,
  Camera,
  CameraDiff,
  colorPortsForTypes,
  Concept,
  ConceptDiff,
  Connection,
  ConnectionDiff,
  Connector,
  ConnectorDiff,
  Coord,
  CoordDiff,
  Design,
  DesignDiff,
  DesignShallow,
  DiffStatus,
  exportKit,
  FileDiff,
  FileId,
  findDesignInKit,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  Folder,
  FolderDiff,
  generateUniqueName,
  getClusterableGroups,
  getIncludedDesigns,
  Group,
  GroupDiff,
  guid,
  Guid,
  importKit,
  inverseKitDiff,
  Kit,
  KitDiff,
  KitShallow,
  Layer,
  LayerDiff,
  Location,
  LocationDiff,
  Model,
  ModelDiff,
  Piece,
  PieceDiff,
  piecesMetadata,
  Plane,
  PlaneDiff,
  Point,
  PointDiff,
  Port,
  PortDiff,
  Prop,
  PropDiff,
  Quality,
  QualityDiff,
  QualityId,
  File as SemioFile,
  Side,
  SideDiff,
  Stat,
  StatDiff,
  Tag,
  TagDiff,
  TagId,
  Type,
  TypeDiff,
  TypeShallow,
  Vec,
  VecDiff,
  Vector,
  VectorDiff,
} from "../semio";
import {
  Action,
  ActionGroup,
  ActionGroupItem,
  Breadcrumb,
  ButtonGroup,
  ButtonGroupItem,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  Footer,
  InteractionProvider,
  Layout as LayoutComponent,
  LevelProvider,
  Navbar,
  type NavbarItem,
  ReactFlowProvider,
  Strip,
  Toggle,
  Transaction,
  Window,
} from "./elements";
import {
  ActionField,
  AppCommandResult,
  AppConfig,
  AppDiff,
  AppEdit,
  AppKind,
  AppRegistration,
  AppStep,
  AppWindowConfig,
  BaseDependency,
  CompleteState,
  CompositeFileProviderConfig,
  conditionalHookResult,
  createAction as createActionValue,
  createField as createFieldValue,
  createPathObserver,
  DerivedNode,
  DerivedStore,
  DesignAppId,
  Device,
  Disposable,
  EnrichedPanelDefinition,
  enrichPanelDefinition,
  executeEventHandler,
  Expertise,
  ExtendedInitialState,
  Field,
  FileProvider,
  FileProviderFactory,
  FocusItem,
  FooterItem,
  getDesignAppHooks,
  getEventHandler,
  getValueAtPath,
  HookResult,
  HudPanelTab,
  KitAppId,
  KitCommandContext,
  KitCommandResult,
  KitDiffAppCommandResult,
  KitDiffAppEdit,
  KitDiffAppStep,
  LocalFileProviderConfig,
  MemoryFileProviderConfig,
  Mode,
  PanelConfig,
  PanelDefinition,
  PanelKey,
  panelKindConfigs,
  PanelPosition,
  PanelSection,
  PanelSections,
  PanelSizes,
  PanelVisibility,
  parseWindowLayout,
  QualityAppId,
  RemoteFileProviderConfig,
  RemoteProviders,
  RouteSegment,
  SidePanelTab,
  SketchpadCommandContext,
  SketchpadCommandResult,
  SketchpadDiff,
  SketchpadScope,
  SketchpadState,
  StoreState,
  StoreStatus,
  Subscribe,
  Synchronizable,
  Theme,
  ToolDefinition,
  ToolGroupProps,
  ToolKind,
  Transact,
  TypeAppId,
  Unsubscribe,
  Url,
  WindowControl,
  WindowEvents,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YPath,
  yPathMapKey,
  YStringArray,
} from "./shared";
import { Tutorial, TutorialProvider, TutorialStore, useAvailableTutorials } from "./Tutorials";

// #endregion 🔖Imports

// #region 🔖Store

export function identitySelector<T>(value: T): T {
  return value;
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
  protected dirty: boolean = true;
  private internalObserverDisposer?: Disposable;
  private fieldSubscribers: Map<string, Set<() => void>> = new Map();
  private fieldObservers: Map<string, Disposable> = new Map();

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
      this.setupDirtyTracking();
    } catch (error) {
      this.status = StoreStatus.ERROR;
      this.error = error instanceof Error ? error : new Error(String(error));
    }
  }

  protected setupDirtyTracking(): void {
    if (this.internalObserverDisposer) {
      this.internalObserverDisposer();
    }
    const callback = () => {
      this.dirty = true;
    };
    this.yMap.observeDeep(callback);
    this.internalObserverDisposer = () => this.yMap.unobserveDeep(callback);
  }

  protected abstract hash(state: TState): string;
  protected abstract buildSnapshot(): TState;

  snapshot(): TState {
    if (this.status === StoreStatus.ERROR) {
      throw this.error || new Error("Store is in error state");
    }
    if (!this.dirty && this.cache) {
      return this.cache;
    }
    const currentData = this.buildSnapshot();
    this.cache = currentData;
    this.dirty = false;
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

  onFieldChanged(key: string, subscribe: Subscribe, deep: boolean = false): Unsubscribe {
    const subscriberCallback = () => {
      subscribe(() => {});
    };

    if (!this.fieldSubscribers.has(key)) {
      this.fieldSubscribers.set(key, new Set());

      const fieldObserver = createFieldObserver(
        this.yMap,
        key,
        (callback: () => void) => {
          const subscribers = this.fieldSubscribers.get(key);
          if (subscribers) {
            subscribers.forEach((cb) => cb());
          }
          callback();
          return () => {};
        },
        deep,
      );
      this.fieldObservers.set(key, fieldObserver);
    }

    const subscribers = this.fieldSubscribers.get(key)!;
    subscribers.add(subscriberCallback);

    return () => {
      subscribers.delete(subscriberCallback);

      if (subscribers.size === 0) {
        const observer = this.fieldObservers.get(key);
        if (observer) {
          observer();
          this.fieldObservers.delete(key);
        }
        this.fieldSubscribers.delete(key);
      }
    };
  }

  onFieldsChanged(keys: string[], subscribe: Subscribe, deep: boolean = false): Unsubscribe {
    return createFieldsObserver(this.yMap, keys, subscribe, deep);
  }

  getFieldSnapshot(key: string): any {
    return (this.snapshot() as any)[key];
  }

  private pathSubscribers: Map<string, Set<() => void>> = new Map();
  private pathObservers: Map<string, Disposable> = new Map();

  onPathChanged(path: YPath, subscribe: Subscribe): Unsubscribe {
    const pathKey = JSON.stringify(path);
    const subscriberCallback = () => {
      subscribe(() => {});
    };
    if (!this.pathSubscribers.has(pathKey)) {
      this.pathSubscribers.set(pathKey, new Set());
      const pathObserver = createPathObserver(this.yMap, path, () => {
        const subscribers = this.pathSubscribers.get(pathKey);
        if (subscribers) subscribers.forEach((cb) => cb());
        return () => {};
      });
      this.pathObservers.set(pathKey, pathObserver);
    }
    const subscribers = this.pathSubscribers.get(pathKey)!;
    subscribers.add(subscriberCallback);
    return () => {
      subscribers.delete(subscriberCallback);
      if (subscribers.size === 0) {
        const observer = this.pathObservers.get(pathKey);
        if (observer) {
          observer();
          this.pathObservers.delete(pathKey);
        }
        this.pathSubscribers.delete(pathKey);
      }
    };
  }

  getPathSnapshot(path: YPath): any {
    return getValueAtPath(this.yMap, path);
  }

  // #endregion 🔖Store
}

export abstract class AppStore<TState, TDiff extends AppDiff<TSelectionDiff>, TSelectionDiff, TEdit extends AppEdit<TSelectionDiff>, TCommandContext, TCommandResult extends AppCommandResult<TDiff>> extends Store<TState> {
  protected readonly commandRegistry: Map<string, (context: TCommandContext, ...rest: any[]) => TCommandResult> = new Map();
  private lastDeletedTransactionEdit?: TEdit;

  private _cachedTransactionStack: TEdit[] | null = null;
  private _transactionStackObserverSet = false;

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
    if (!yStack) return [];

    if (!this._transactionStackObserverSet) {
      this._transactionStackObserverSet = true;
      yStack.observe(() => {
        this._cachedTransactionStack = null;
      });
    }

    if (this._cachedTransactionStack !== null) {
      return this._cachedTransactionStack;
    }

    this._cachedTransactionStack = yStack.toArray();
    return this._cachedTransactionStack;
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

// #region 🔖Plain App Store (No YJS)

export abstract class PlainAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult> {
  public readonly guid: Guid;
  protected state: TState;
  protected readonly listeners: Set<() => void> = new Set();
  protected readonly commandRegistry: Map<string, (context: TCommandContext, ...rest: any[]) => TCommandResult> = new Map();
  protected isTransactionActive: boolean = false;
  protected _currentTransactionStack: TEdit[] = [];
  protected pastTransactionsStack: TEdit[] = [];
  protected redoStack: TEdit[] = [];
  protected lastDeletedTransactionEdit?: TEdit;

  constructor(initialState: TState) {
    this.guid = guid();
    this.state = initialState;
  }

  get currentTransactionStack(): TEdit[] {
    return this._currentTransactionStack;
  }

  snapshot(): TState {
    return this.state;
  }

  subscribe(callback: () => void): () => void {
    this.listeners.add(callback);
    return () => this.listeners.delete(callback);
  }

  onChangedDeep(callback: () => void): Disposable {
    this.listeners.add(callback);
    return { dispose: () => this.listeners.delete(callback) };
  }

  onFieldChanged(key: string, subscribe: Subscribe, _deep?: boolean): Disposable {
    const wrappedCallback = () => {
      subscribe(() => {});
    };
    this.listeners.add(wrappedCallback);
    const dispose = () => this.listeners.delete(wrappedCallback);
    return Object.assign(dispose, { dispose });
  }

  getFieldSnapshot(key: string): any {
    return (this.state as any)?.[key];
  }

  protected notify(): void {
    this.listeners.forEach((cb) => cb());
  }

  abstract change(diff: TDiff): void;

  protected abstract applySelectionDiff(selectionDiff: TSelectionDiff): void;
  protected abstract inverseSelectionDiff(selection: any, diff: TSelectionDiff): TSelectionDiff;
  protected abstract getSelection(): any;

  startTransaction(): void {
    if (this.isTransactionActive) {
      this.finalizeTransaction();
    }
    this.isTransactionActive = true;
  }

  abortTransaction(): void {
    if (this.isTransactionActive) {
      for (let i = this._currentTransactionStack.length - 1; i >= 0; i--) {
        const edit = this._currentTransactionStack[i] as any;
        if (edit?.undo?.selectionDiff) {
          this.applySelectionDiff(edit.undo.selectionDiff);
        }
      }
      this._currentTransactionStack = [];
      this.isTransactionActive = false;
      this.notify();
    }
  }

  finalizeTransaction(): void {
    if (this.isTransactionActive) {
      this.redoStack = [];
      if (this._currentTransactionStack.length > 0) {
        const edits = this._currentTransactionStack;
        if (edits.length === 1) {
          this.pastTransactionsStack.push(edits[0]);
        } else if (edits.length > 1) {
          const firstEdit = edits[0] as any;
          const lastEdit = edits[edits.length - 1] as any;
          const mergedEdit = { do: lastEdit.do, undo: firstEdit.undo } as TEdit;
          this.pastTransactionsStack.push(mergedEdit);
        }
        this._currentTransactionStack = [];
      }
      this.isTransactionActive = false;
      this.notify();
    }
  }

  undo(): void {
    if (this.isTransactionActive) {
      if (this._currentTransactionStack.length > 0) {
        const edit = this._currentTransactionStack.pop() as any;
        this.lastDeletedTransactionEdit = edit;
        if (edit?.undo?.selectionDiff) {
          this.applySelectionDiff(edit.undo.selectionDiff);
        }
        this.notify();
      }
    } else {
      if (this.pastTransactionsStack.length > 0) {
        const edit = this.pastTransactionsStack.pop() as any;
        this.redoStack.push(edit);
        if (edit?.undo?.selectionDiff) {
          this.applySelectionDiff(edit.undo.selectionDiff);
        }
        this.notify();
      }
    }
  }

  redo(): void {
    if (this.isTransactionActive) {
      if (this.lastDeletedTransactionEdit) {
        this._currentTransactionStack.push(this.lastDeletedTransactionEdit);
        const edit = this.lastDeletedTransactionEdit as any;
        this.lastDeletedTransactionEdit = undefined;
        if (edit?.do?.selectionDiff) {
          this.applySelectionDiff(edit.do.selectionDiff);
        }
        this.notify();
      }
    } else {
      if (this.redoStack.length > 0) {
        const edit = this.redoStack.pop() as any;
        this.pastTransactionsStack.push(edit);
        if (edit?.do?.selectionDiff) {
          this.applySelectionDiff(edit.do.selectionDiff);
        }
        this.notify();
      }
    }
  }

  canUndo(): boolean {
    if (this.isTransactionActive) return this.currentTransactionStack.length > 0;
    return this.pastTransactionsStack.length > 0;
  }

  canRedo(): boolean {
    if (this.isTransactionActive) return false;
    return this.redoStack.length > 0;
  }

  protected recordEdit(result: TCommandResult): void {
    const res = result as any;
    if (this.isTransactionActive && res.diff) {
      this.redoStack = [];
      this.lastDeletedTransactionEdit = undefined;
      const selection = this.getSelection();
      const inversedSelectionDiff = res.diff?.selection ? this.inverseSelectionDiff(selection, res.diff.selection) : undefined;
      const doStep = { selectionDiff: res.diff?.selection };
      const undoStep = { selectionDiff: inversedSelectionDiff };
      const edit = { do: doStep, undo: undoStep } as TEdit;
      this._currentTransactionStack.push(edit);
    }
  }

  registerCommand(command: string, callback: (context: TCommandContext, ...rest: any[]) => TCommandResult): () => void {
    this.commandRegistry.set(command, callback);
    return () => this.commandRegistry.delete(command);
  }

  abstract executeCommand<T>(command: string, ...rest: any[]): Promise<T>;
}

export abstract class PlainKitDiffAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult> extends PlainAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult> {
  protected readonly parentStore: SketchpadStore;

  constructor(parent: SketchpadStore, initialState: TState) {
    super(initialState);
    this.parentStore = parent;
  }

  abstract kit(): KitStore;

  abortTransaction(): void {
    if (this.isTransactionActive) {
      for (let i = this._currentTransactionStack.length - 1; i >= 0; i--) {
        const edit = this._currentTransactionStack[i] as any;
        if (edit?.undo) {
          if (edit.undo.kitDiff) {
            this.kit().change(edit.undo.kitDiff);
          }
          if (edit.undo.selectionDiff) {
            this.applySelectionDiff(edit.undo.selectionDiff);
          }
        }
      }
      this._currentTransactionStack = [];
      this.isTransactionActive = false;
      this.notify();
    }
  }

  undo(): void {
    if (this.isTransactionActive) {
      if (this._currentTransactionStack.length > 0) {
        const edit = this._currentTransactionStack.pop() as any;
        this.lastDeletedTransactionEdit = edit;
        if (edit?.undo) {
          if (edit.undo.kitDiff) {
            this.kit().change(edit.undo.kitDiff);
          }
          if (edit.undo.selectionDiff) {
            this.applySelectionDiff(edit.undo.selectionDiff);
          }
        }
        this.notify();
      }
    } else {
      if (this.pastTransactionsStack.length > 0) {
        const edit = this.pastTransactionsStack.pop() as any;
        this.redoStack.push(edit);
        if (edit?.undo) {
          if (edit.undo.kitDiff) {
            this.kit().change(edit.undo.kitDiff);
          }
          if (edit.undo.selectionDiff) {
            this.applySelectionDiff(edit.undo.selectionDiff);
          }
        }
        this.notify();
      }
    }
  }

  redo(): void {
    if (this.isTransactionActive) {
      if (this.lastDeletedTransactionEdit) {
        this._currentTransactionStack.push(this.lastDeletedTransactionEdit);
        const edit = this.lastDeletedTransactionEdit as any;
        this.lastDeletedTransactionEdit = undefined;
        if (edit?.do) {
          if (edit.do.kitDiff) {
            this.kit().change(edit.do.kitDiff);
          }
          if (edit.do.selectionDiff) {
            this.applySelectionDiff(edit.do.selectionDiff);
          }
        }
        this.notify();
      }
    } else {
      if (this.redoStack.length > 0) {
        const edit = this.redoStack.pop() as any;
        this.pastTransactionsStack.push(edit);
        if (edit?.do) {
          if (edit.do.kitDiff) {
            this.kit().change(edit.do.kitDiff);
          }
          if (edit.do.selectionDiff) {
            this.applySelectionDiff(edit.do.selectionDiff);
          }
        }
        this.notify();
      }
    }
  }

  protected recordEdit(result: TCommandResult): void {
    const res = result as any;
    if (this.isTransactionActive && (res.diff || res.kitDiff)) {
      this.redoStack = [];
      this.lastDeletedTransactionEdit = undefined;
      const selection = this.getSelection();
      const inversedSelectionDiff = res.diff?.selection ? this.inverseSelectionDiff(selection, res.diff.selection) : undefined;
      const kitStore = this.kit();
      const kitState = kitStore.snapshot();
      const inversedKitDiff = res.kitDiff ? inverseKitDiff(kitState, res.kitDiff) : undefined;
      const doStep = { kitDiff: res.kitDiff, selectionDiff: res.diff?.selection };
      const undoStep = { kitDiff: inversedKitDiff, selectionDiff: inversedSelectionDiff };
      const edit = { do: doStep, undo: undoStep } as TEdit;
      this._currentTransactionStack.push(edit);
    }
  }
}

// #endregion 🔖Plain App Store (No YJS)

// #region 🔖File Provider

// #region 🔖Memory File Provider

export function createMemoryFileProvider(config?: MemoryFileProviderConfig): FileProviderFactory {
  const storage = new Map<string, Blob>();

  return async (kitId: string): Promise<FileProvider> => {
    const getKey = (kitId: string, fileId: string, path: string): string => {
      return `${kitId}/${fileId}/${path}`;
    };

    return {
      upload: async (kitId, fileId, path, blob) => {
        const key = getKey(kitId, fileId, path);
        storage.set(key, blob);
        return `memory://${key}`;
      },

      download: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        const blob = storage.get(key);

        if (!blob) {
          throw new Error(`File not found in memory: ${key}`);
        }

        return blob;
      },

      delete: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        storage.delete(key);
      },

      getUrl: (kitId, fileId, path) => {
        return `memory://${getKey(kitId, fileId, path)}`;
      },
    };
  };
}

// #endregion 🔖Memory File Provider

// #region 🔖Local File Provider (IndexedDB)

export function createLocalFileProvider(config?: LocalFileProviderConfig): FileProviderFactory {
  const dbName = config?.dbName || "semio-files";
  const storeName = config?.storeName || "files";

  const openDB = (): Promise<IDBDatabase> => {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(dbName, 1);

      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(storeName)) {
          db.createObjectStore(storeName);
        }
      };
    });
  };

  return async (kitId: string): Promise<FileProvider> => {
    const getKey = (kitId: string, fileId: string, path: string): string => {
      return `${kitId}/${fileId}/${path}`;
    };

    return {
      upload: async (kitId, fileId, path, blob) => {
        const key = getKey(kitId, fileId, path);
        const db = await openDB();

        return new Promise<string>((resolve, reject) => {
          const transaction = db.transaction([storeName], "readwrite");
          const store = transaction.objectStore(storeName);
          const request = store.put(blob, key);

          request.onsuccess = () => {
            resolve(`local://${key}`);
          };
          request.onerror = () => reject(request.error);

          transaction.oncomplete = () => db.close();
        });
      },

      download: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        const db = await openDB();

        return new Promise<Blob>((resolve, reject) => {
          const transaction = db.transaction([storeName], "readonly");
          const store = transaction.objectStore(storeName);
          const request = store.get(key);

          request.onsuccess = () => {
            const blob = request.result;
            if (!blob) {
              reject(new Error(`File not found in IndexedDB: ${key}`));
            } else {
              resolve(blob);
            }
          };
          request.onerror = () => reject(request.error);

          transaction.oncomplete = () => db.close();
        });
      },

      delete: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        const db = await openDB();

        return new Promise<void>((resolve, reject) => {
          const transaction = db.transaction([storeName], "readwrite");
          const store = transaction.objectStore(storeName);
          const request = store.delete(key);

          request.onsuccess = () => {
            resolve();
          };
          request.onerror = () => reject(request.error);

          transaction.oncomplete = () => db.close();
        });
      },

      getUrl: (kitId, fileId, path) => {
        return `local://${getKey(kitId, fileId, path)}`;
      },
    };
  };
}

// #endregion 🔖Local File Provider (IndexedDB)

// #region 🔖Remote File Provider

export function createRemoteFileProvider(config: RemoteFileProviderConfig): FileProviderFactory {
  return async (kitId: string): Promise<FileProvider> => {
    const getUrl = (kitId: string, fileId: string, path: string): string => {
      return `${config.baseUrl}/kits/${kitId}/files/${fileId}`;
    };

    const headers = {
      ...config.headers,
    };

    return {
      upload: async (kitId, fileId, path, blob) => {
        const formData = new FormData();
        formData.append("file", blob, path);

        const response = await fetch(getUrl(kitId, fileId, path), {
          method: "POST",
          headers,
          body: formData,
        });

        if (!response.ok) {
          throw new Error(`Remote upload failed: ${response.statusText}`);
        }

        const result = await response.json();
        return result.url || getUrl(kitId, fileId, path);
      },

      download: async (kitId, fileId, path) => {
        const response = await fetch(getUrl(kitId, fileId, path), {
          method: "GET",
          headers,
        });

        if (!response.ok) {
          throw new Error(`Remote download failed: ${response.statusText}`);
        }

        const blob = await response.blob();
        return blob;
      },

      delete: async (kitId, fileId, path) => {
        const response = await fetch(getUrl(kitId, fileId, path), {
          method: "DELETE",
          headers,
        });

        if (!response.ok) {
          throw new Error(`Remote delete failed: ${response.statusText}`);
        }
      },

      getUrl: (kitId, fileId, path) => {
        return getUrl(kitId, fileId, path);
      },
    };
  };
}

// #endregion 🔖Remote File Provider

// #region 🔖Composite File Provider

export function createCompositeFileProvider(config: CompositeFileProviderConfig): FileProviderFactory {
  return async (kitId: string): Promise<FileProvider> => {
    const providers: FileProvider[] = [];

    if (config.memory) {
      const memoryProvider = await createMemoryFileProvider()(kitId);
      providers.push(memoryProvider);
    }

    if (config.local) {
      const localConfig = typeof config.local === "object" ? config.local : undefined;
      const localProvider = await createLocalFileProvider(localConfig)(kitId);
      providers.push(localProvider);
    }

    if (config.remote) {
      const remoteProvider = await createRemoteFileProvider(config.remote)(kitId);
      providers.push(remoteProvider);
    }

    if (providers.length === 0) {
      throw new Error("At least one provider must be configured");
    }

    return {
      upload: async (kitId, fileId, path, blob) => {
        const results = await Promise.allSettled(providers.map((p) => p.upload(kitId, fileId, path, blob)));

        const successful = results.filter((r) => r.status === "fulfilled");
        if (successful.length === 0) {
          throw new Error(`All providers failed to upload file ${path}`);
        }

        const lastSuccessful = results.reverse().find((r) => r.status === "fulfilled") as PromiseFulfilledResult<string>;
        return lastSuccessful.value;
      },

      download: async (kitId, fileId, path) => {
        for (const provider of providers) {
          try {
            return await provider.download(kitId, fileId, path);
          } catch (error) {
            console.warn(`Provider failed to download ${path}, trying next:`, error);
          }
        }
        throw new Error(`All providers failed to download file ${path}`);
      },

      delete: async (kitId, fileId, path) => {
        await Promise.allSettled(providers.map((p) => p.delete(kitId, fileId, path)));
      },

      getUrl: (kitId, fileId, path) => {
        return providers[providers.length - 1].getUrl(kitId, fileId, path);
      },
    };
  };
}

// #endregion 🔖Composite File Provider

// #endregion 🔖File Provider

// #region 🔖Kits

type YAttributeVal = string;
type YAttribute = Y.Map<YAttributeVal>;

class AttributeStore {
  private yAttribute: YAttribute;
  private cache?: Attribute;
  private cacheHash?: string;

  constructor(yAttribute: YAttribute, attribute: Attribute) {
    this.yAttribute = yAttribute;
  }

  get key(): string {
    return this.yAttribute.get("key") as string;
  }
  set key(key: string) {
    this.yAttribute.set("key", key);
  }

  get value(): string | undefined {
    return this.yAttribute.get("value") as string | undefined;
  }
  set value(value: string | undefined) {
    this.yAttribute.set("value", value || "");
  }

  get definition(): string | undefined {
    return this.yAttribute.get("definition") as string | undefined;
  }
  set definition(definition: string | undefined) {
    this.yAttribute.set("definition", definition || "");
  }

  get guid(): string {
    return this.yAttribute.get("guid") as string;
  }
  set guid(guid: string) {
    this.yAttribute.set("guid", guid);
  }

  hash = (attribute: Attribute): string => {
    return JSON.stringify(attribute);
  };

  snapshot = (): Attribute => {
    const currentData = {
      guid: this.guid,
      key: this.key,
      value: this.value,
      definition: this.definition,
    };
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  change = (diff: any) => {
    if (diff.key !== undefined) this.key = diff.key;
    if (diff.value !== undefined) this.value = diff.value;
    if (diff.definition !== undefined) this.definition = diff.definition;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yAttribute, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yAttribute, subscribe, true);
  };
}

// #endregion 🔖Kits

// #region 🔖Coord

type YCoordVal = number;
type YCoord = Y.Map<YCoordVal>;

class YCoordStore {
  private yCoord: YCoord;
  private cache?: Coord;
  private cacheHash?: string;

  constructor(yCoord: YCoord, coord: Coord) {
    this.yCoord = yCoord;
    this.u = coord.u;
    this.v = coord.v;
  }

  get u(): number {
    return this.yCoord.get("u") as number;
  }
  set u(u: number) {
    this.yCoord.set("u", u);
  }

  get v(): number {
    return this.yCoord.get("v") as number;
  }
  set v(v: number) {
    this.yCoord.set("v", v);
  }

  hash = (coord: Coord): string => {
    return JSON.stringify(coord);
  };

  snapshot = (): Coord => {
    const currentData = {
      u: this.u,
      v: this.v,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: CoordDiff) => {
    if (diff.u !== undefined) this.u = diff.u;
    if (diff.v !== undefined) this.v = diff.v;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yCoord, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yCoord, subscribe, true);
  };
}

// #endregion 🔖Coord

// #region 🔖Vec

type YVecVal = number;
type YVec = Y.Map<YVecVal>;

class YVecStore {
  private yVec: YVec;
  private cache?: Vec;
  private cacheHash?: string;

  constructor(yVec: YVec, vec: Vec) {
    this.yVec = yVec;
    this.u = vec.u;
    this.v = vec.v;
  }

  get u(): number {
    return this.yVec.get("u") as number;
  }
  set u(u: number) {
    this.yVec.set("u", u);
  }

  get v(): number {
    return this.yVec.get("v") as number;
  }
  set v(v: number) {
    this.yVec.set("v", v);
  }

  hash = (vec: Vec): string => {
    return JSON.stringify(vec);
  };

  snapshot = (): Vec => {
    const currentData = {
      u: this.u,
      v: this.v,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: VecDiff) => {
    if (diff.u !== undefined) this.u = diff.u;
    if (diff.v !== undefined) this.v = diff.v;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yVec, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yVec, subscribe, true);
  };
}

// #endregion 🔖Vec

// #region 🔖Point

type YPointVal = number;
type YPoint = Y.Map<YPointVal>;

class YPointStore {
  private yPoint: YPoint;
  private cache?: Point;
  private cacheHash?: string;

  constructor(yPoint: YPoint, point: Point) {
    this.yPoint = yPoint;
    this.x = point.x;
    this.y = point.y;
    this.z = point.z;
  }

  get x(): number {
    return this.yPoint.get("x") as number;
  }
  set x(x: number) {
    this.yPoint.set("x", x);
  }

  get y(): number {
    return this.yPoint.get("y") as number;
  }
  set y(y: number) {
    this.yPoint.set("y", y);
  }

  get z(): number {
    return this.yPoint.get("z") as number;
  }
  set z(z: number) {
    this.yPoint.set("z", z);
  }

  hash = (point: Point): string => {
    return JSON.stringify(point);
  };

  snapshot = (): Point => {
    const currentData = {
      x: this.x,
      y: this.y,
      z: this.z,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: PointDiff) => {
    if (diff.x !== undefined) this.x = diff.x;
    if (diff.y !== undefined) this.y = diff.y;
    if (diff.z !== undefined) this.z = diff.z;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPoint, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPoint, subscribe, true);
  };
}

// #endregion 🔖Point

// #region 🔖Vector

type YVectorVal = number;
type YVector = Y.Map<YVectorVal>;

class YVectorStore {
  private yVector: YVector;
  private cache?: Vector;
  private cacheHash?: string;

  constructor(yVector: YVector, vector: Vector) {
    this.yVector = yVector;
    this.x = vector.x;
    this.y = vector.y;
    this.z = vector.z;
  }

  get x(): number {
    return this.yVector.get("x") as number;
  }
  set x(x: number) {
    this.yVector.set("x", x);
  }

  get y(): number {
    return this.yVector.get("y") as number;
  }
  set y(y: number) {
    this.yVector.set("y", y);
  }

  get z(): number {
    return this.yVector.get("z") as number;
  }
  set z(z: number) {
    this.yVector.set("z", z);
  }

  hash = (vector: Vector): string => {
    return JSON.stringify(vector);
  };

  snapshot = (): Vector => {
    const currentData = {
      x: this.x,
      y: this.y,
      z: this.z,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: VectorDiff) => {
    if (diff.x !== undefined) this.x = diff.x;
    if (diff.y !== undefined) this.y = diff.y;
    if (diff.z !== undefined) this.z = diff.z;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yVector, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yVector, subscribe, true);
  };
}

// #endregion 🔖Vector

// #region 🔖Plane

type YPlaneVal = YPoint | YVector;
type YPlane = Y.Map<YPlaneVal>;

class YPlaneStore {
  private yPlane: YPlane;
  private origin: YPointStore;
  private xAxis: YVectorStore;
  private yAxis: YVectorStore;
  private cache?: Plane;
  private cacheHash?: string;

  constructor(yPlane: YPlane, plane: Plane) {
    this.yPlane = yPlane;

    const yOrigin = new Y.Map<YPointVal>();
    this.yPlane.set("origin", yOrigin);
    this.origin = new YPointStore(yOrigin, plane.origin);

    const yXAxis = new Y.Map<YVectorVal>();
    this.yPlane.set("xAxis", yXAxis);
    this.xAxis = new YVectorStore(yXAxis, plane.xAxis);

    const yYAxis = new Y.Map<YVectorVal>();
    this.yPlane.set("yAxis", yYAxis);
    this.yAxis = new YVectorStore(yYAxis, plane.yAxis);
  }
  hash = (plane: Plane): string => {
    return JSON.stringify(plane);
  };

  snapshot = (): Plane => {
    const currentData = {
      origin: this.origin.snapshot(),
      xAxis: this.xAxis.snapshot(),
      yAxis: this.yAxis.snapshot(),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: PlaneDiff) => {
    if (diff.origin !== undefined) this.origin.change(diff.origin);
    if (diff.xAxis !== undefined) this.xAxis.change(diff.xAxis);
    if (diff.yAxis !== undefined) this.yAxis.change(diff.yAxis);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPlane, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPlane, subscribe, true);
  };
}

// #endregion 🔖Plane

// #region 🔖Camera

type YCameraVal = YPoint | YVector;
type YCamera = Y.Map<YCameraVal>;

class YCameraStore {
  private yCamera: YCamera;
  private position: YPointStore;
  private forward: YVectorStore;
  private up: YVectorStore;
  private cache?: Camera;
  private cacheHash?: string;

  constructor(yCamera: YCamera, camera: Camera) {
    this.yCamera = yCamera;

    const yPosition = new Y.Map<YPointVal>();
    this.yCamera.set("position", yPosition);
    this.position = new YPointStore(yPosition, camera.position);

    const yForward = new Y.Map<YVectorVal>();
    this.yCamera.set("forward", yForward);
    this.forward = new YVectorStore(yForward, camera.forward);

    const yUp = new Y.Map<YVectorVal>();
    this.yCamera.set("up", yUp);
    this.up = new YVectorStore(yUp, camera.up);
  }

  hash = (camera: Camera): string => {
    return JSON.stringify(camera);
  };

  snapshot = (): Camera => {
    const currentData = {
      position: this.position.snapshot(),
      forward: this.forward.snapshot(),
      up: this.up.snapshot(),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: CameraDiff) => {
    if (diff.position !== undefined) this.position.change(diff.position);
    if (diff.forward !== undefined) this.forward.change(diff.forward);
    if (diff.up !== undefined) this.up.change(diff.up);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yCamera, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yCamera, subscribe, true);
  };
}

// #endregion 🔖Camera

// #region 🔖Location

type YLocationVal = number | string | YAttributes;
type YLocation = Y.Map<YLocationVal>;

class YLocationStore {
  private yLocation: YLocation;
  private attributes: Map<string, AttributeStore>;
  private cache?: Location;
  private cacheHash?: string;

  constructor(yLocation: YLocation, location: Location) {
    this.yLocation = yLocation;
    this.guid = location.guid;
    this.latitude = location.latitude;
    this.longitude = location.longitude;
    this.altitude = location.altitude;
    this.attributes = new Map();
  }

  get guid(): string {
    return this.yLocation.get("guid") as string;
  }
  set guid(guid: string) {
    this.yLocation.set("guid", guid);
  }

  get latitude(): number {
    return this.yLocation.get("latitude") as number;
  }
  set latitude(latitude: number) {
    this.yLocation.set("latitude", latitude);
  }

  get longitude(): number {
    return this.yLocation.get("longitude") as number;
  }
  set longitude(longitude: number) {
    this.yLocation.set("longitude", longitude);
  }

  get altitude(): number | undefined {
    return this.yLocation.get("altitude") as number | undefined;
  }
  set altitude(altitude: number | undefined) {
    if (altitude !== undefined) {
      this.yLocation.set("altitude", altitude);
    } else {
      this.yLocation.delete("altitude");
    }
  }

  hash = (location: Location): string => {
    return JSON.stringify(location);
  };

  snapshot = (): Location => {
    const currentData = {
      guid: this.guid,
      latitude: this.latitude,
      longitude: this.longitude,
      altitude: this.altitude,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: LocationDiff) => {
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.latitude !== undefined) this.latitude = diff.latitude;
    if (diff.longitude !== undefined) this.longitude = diff.longitude;
    if (diff.altitude !== undefined) this.altitude = diff.altitude;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yLocation, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yLocation, subscribe, true);
  };
}

// #endregion 🔖Location

// #region 🔖Author

type YAuthorVal = string | YAttributes;
type YAuthor = Y.Map<YAuthorVal>;
type YAuthors = Y.Array<YAuthor>;
type YAuthorUuid = string;
type YAuthorUuids = Y.Array<YAuthorUuid>;

class AuthorStore {
  private yAuthor: YAuthor;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Author;
  private cacheHash?: string;

  constructor(yAuthor: YAuthor, author: Author) {
    this.yAuthor = yAuthor;
    this.guid = author.guid;
    this.name = author.name;
    this.email = author.email;
    this.yAttributes = new Y.Array<YAttribute>();
    this.yAuthor.set("attributes", this.yAttributes);
    this.attributes = new Map();
    author.attributes?.forEach((attribute) => this.createAttribute(attribute));
  }

  get guid(): string {
    return this.yAuthor.get("guid") as string;
  }
  set guid(guid: string) {
    this.yAuthor.set("guid", guid);
  }

  get name(): string {
    return this.yAuthor.get("name") as string;
  }
  set name(name: string) {
    this.yAuthor.set("name", name);
  }

  get email(): string {
    return this.yAuthor.get("email") as string;
  }
  set email(email: string) {
    this.yAuthor.set("email", email);
  }

  hasAttribute(guid: string): boolean {
    return this.attributes.has(guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  attribute(guid: string): AttributeStore {
    return this.attributes.get(guid)!;
  }

  hash = (author: Author): string => {
    return JSON.stringify(author);
  };

  snapshot = (): Author => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      email: this.email,
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
    };
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  change = (diff: AuthorDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.email !== undefined) this.email = diff.email;
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yAuthor, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yAuthor, subscribe, true);
  };
}

type AuthorScope = { guid: string };
const AuthorScopeContext = createContext<AuthorScope | null>(null);
export const AuthorScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(AuthorScopeContext.Provider, { value }, props.children as any);
};
const useAuthorScope = () => useContext(AuthorScopeContext);

function useAuthorStore<T>(selector?: (store: AuthorStore) => T, guid?: string): T | AuthorStore | null {
  const kitStore = useKitStore() as KitStore | null;
  const authorScope = useAuthorScope();
  const authorGuid = authorScope?.guid ?? guid;
  if (!kitStore) return null;
  if (!authorGuid) throw new Error("useAuthorStore must be called within a AuthorScopeProvider or be directly provided with a guid");
  if (!kitStore.hasAuthor(authorGuid)) throw new Error(`Author store not found for author ${authorGuid}`);
  const authorStore = kitStore.author(authorGuid);
  return selector ? selector(authorStore) : authorStore;
}

export function useAuthor<T>(selector?: (author: Author) => T, id?: Guid, deep: boolean = false): T | Author | null {
  const authorScope = useAuthorScope();
  const authorGuid = authorScope?.guid ?? id;
  const store = useAuthorStore(identitySelector, authorGuid ?? undefined) as AuthorStore | null;
  const synced = useSyncOptional<Author, T>(store as any, selector ? selector : (identitySelector as any));
  if (!store) return null;
  return synced;
}

// #endregion 🔖Author

// #region 🔖File

type YFile = Y.Map<string | number | YAttributes>;
type YFiles = Y.Array<YFile>;

class FileStore {
  private yFile: YFile;
  private cache?: SemioFile;
  private cacheHash?: string;

  constructor(yFile: YFile) {
    this.yFile = yFile;
  }

  get guid(): string {
    return this.yFile.get("guid") as string;
  }
  set guid(guid: string) {
    this.yFile.set("guid", guid);
  }

  get name(): string {
    return this.yFile.get("name") as string;
  }
  set name(name: string) {
    this.yFile.set("name", name);
  }
  get folder(): string | undefined {
    return this.yFile.get("folder") as string | undefined;
  }
  set folder(folder: string | undefined) {
    if (folder) this.yFile.set("folder", folder);
    else this.yFile.delete("folder");
  }
  get remote(): string | undefined {
    return this.yFile.get("remote") as string | undefined;
  }
  set remote(remote: string | undefined) {
    this.yFile.set("remote", remote || "");
  }
  get size(): number | undefined {
    return this.yFile.get("size") as number | undefined;
  }
  set size(size: number | undefined) {
    if (size !== undefined) {
      this.yFile.set("size", size);
    }
  }
  get fileHash(): string | undefined {
    return this.yFile.get("hash") as string | undefined;
  }
  set fileHash(hash: string | undefined) {
    this.yFile.set("hash", hash || "");
  }
  get createdAt(): Date | undefined {
    const date = this.yFile.get("createdAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set createdAt(createdAt: Date | string | undefined) {
    if (!createdAt) {
      this.yFile.set("createdAt", "");
    } else if (typeof createdAt === "string") {
      this.yFile.set("createdAt", createdAt);
    } else {
      this.yFile.set("createdAt", createdAt.toISOString());
    }
  }
  get updatedAt(): Date | undefined {
    const date = this.yFile.get("updatedAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set updatedAt(updatedAt: Date | string | undefined) {
    if (!updatedAt) {
      this.yFile.set("updatedAt", "");
    } else if (typeof updatedAt === "string") {
      this.yFile.set("updatedAt", updatedAt);
    } else {
      this.yFile.set("updatedAt", updatedAt.toISOString());
    }
  }
  get createdBy(): Guid | undefined {
    return this.yFile.get("createdBy") as string | undefined;
  }
  set createdBy(createdBy: Guid | undefined) {
    this.yFile.set("createdBy", createdBy || "");
  }
  get updatedBy(): Guid | undefined {
    return this.yFile.get("updatedBy") as string | undefined;
  }
  set updatedBy(updatedBy: Guid | undefined) {
    this.yFile.set("updatedBy", updatedBy || "");
  }

  hashFile = (file: SemioFile): string => {
    return JSON.stringify(file);
  };

  snapshot = (): SemioFile => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      folder: this.folder ? { guid: this.folder } : undefined,
      remote: this.remote,
      size: this.size,
      hash: this.fileHash,
      createdAt: this.createdAt?.toISOString(),
      updatedAt: this.updatedAt?.toISOString(),
      createdBy: this.createdBy,
      updatedBy: this.updatedBy,
    };
    const currentHash = this.hashFile(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: FileDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.folder !== undefined) this.folder = diff.folder?.guid;
    if (diff.remote !== undefined) this.remote = diff.remote;
    if (diff.size !== undefined) this.size = diff.size;
    if (diff.hash !== undefined) this.fileHash = diff.hash;
    if (diff.createdAt !== undefined) this.createdAt = diff.createdAt;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;
    if (diff.createdBy !== undefined) this.createdBy = diff.createdBy;
    if (diff.updatedBy !== undefined) this.updatedBy = diff.updatedBy;
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yFile, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yFile, subscribe, true);
  };
}

// #endregion 🔖File

// #region 🔖Folder

type YFolder = Y.Map<string | YAttributes>;
type YFolders = Y.Array<YFolder>;

class FolderStore {
  yFolder: YFolder;
  private cache?: Folder;
  private cacheHash?: string;

  constructor(yFolder: YFolder) {
    this.yFolder = yFolder;
  }

  get guid(): string {
    return this.yFolder.get("guid") as string;
  }
  set guid(guid: string) {
    this.yFolder.set("guid", guid);
  }

  get name(): string {
    return this.yFolder.get("name") as string;
  }
  set name(name: string) {
    this.yFolder.set("name", name);
  }

  get parent(): string | undefined {
    return this.yFolder.get("parent") as string | undefined;
  }
  set parent(parent: string | undefined) {
    if (parent) this.yFolder.set("parent", parent);
    else this.yFolder.delete("parent");
  }

  get description(): string | undefined {
    return this.yFolder.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yFolder.set("description", description || "");
  }

  get createdAt(): Date | undefined {
    const date = this.yFolder.get("createdAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set createdAt(createdAt: Date | string | undefined) {
    if (!createdAt) {
      this.yFolder.set("createdAt", "");
    } else if (typeof createdAt === "string") {
      this.yFolder.set("createdAt", createdAt);
    } else {
      this.yFolder.set("createdAt", createdAt.toISOString());
    }
  }

  get updatedAt(): Date | undefined {
    const date = this.yFolder.get("updatedAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set updatedAt(updatedAt: Date | string | undefined) {
    if (!updatedAt) {
      this.yFolder.set("updatedAt", "");
    } else if (typeof updatedAt === "string") {
      this.yFolder.set("updatedAt", updatedAt);
    } else {
      this.yFolder.set("updatedAt", updatedAt.toISOString());
    }
  }

  get createdBy(): Guid | undefined {
    return this.yFolder.get("createdBy") as string | undefined;
  }
  set createdBy(createdBy: Guid | undefined) {
    this.yFolder.set("createdBy", createdBy || "");
  }

  get updatedBy(): Guid | undefined {
    return this.yFolder.get("updatedBy") as string | undefined;
  }
  set updatedBy(updatedBy: Guid | undefined) {
    this.yFolder.set("updatedBy", updatedBy || "");
  }

  hashFolder = (folder: Folder): string => {
    return JSON.stringify(folder);
  };

  snapshot = (): Folder => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      parent: this.parent ? { guid: this.parent } : undefined,
      description: this.description,
      createdAt: this.createdAt?.toISOString(),
      updatedAt: this.updatedAt?.toISOString(),
      createdBy: this.createdBy,
      updatedBy: this.updatedBy,
    };
    const currentHash = this.hashFolder(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: FolderDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.parent !== undefined) this.parent = diff.parent?.guid;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.createdAt !== undefined) this.createdAt = diff.createdAt;
    if (diff.updatedAt !== undefined) this.updatedAt = diff.updatedAt;
    if (diff.createdBy !== undefined) this.createdBy = diff.createdBy;
    if (diff.updatedBy !== undefined) this.updatedBy = diff.updatedBy;
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yFolder, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yFolder, subscribe, true);
  };
}

// #endregion 🔖Folder

// #region 🔖Benchmark

type YBenchmark = Y.Map<string | number | boolean | YAttributes>;
type YBenchmarks = Y.Array<YBenchmark>;

class BenchmarkStore {
  private yBenchmark: YBenchmark;
  private cache?: Benchmark;
  private cacheHash?: string;

  constructor(yBenchmark: YBenchmark, benchmark: Benchmark) {
    this.yBenchmark = yBenchmark;
    this.guid = benchmark.guid;
    this.name = benchmark.name;
    this.icon = benchmark.icon;
    this.min = benchmark.min;
    this.minExcluded = benchmark.minExcluded;
    this.max = benchmark.max;
    this.maxExcluded = benchmark.maxExcluded;
  }

  get guid(): string {
    return this.yBenchmark.get("guid") as string;
  }
  set guid(guid: string) {
    this.yBenchmark.set("guid", guid);
  }

  get name(): string {
    return this.yBenchmark.get("name") as string;
  }
  set name(name: string) {
    this.yBenchmark.set("name", name);
  }

  get icon(): string | undefined {
    return this.yBenchmark.get("icon") as string | undefined;
  }
  set icon(icon: string | undefined) {
    this.yBenchmark.set("icon", icon || "");
  }

  get min(): number | undefined {
    return this.yBenchmark.get("min") as number | undefined;
  }
  set min(min: number | undefined) {
    this.yBenchmark.set("min", min || 0);
  }

  get minExcluded(): boolean | undefined {
    return this.yBenchmark.get("minExcluded") as boolean | undefined;
  }
  set minExcluded(minExcluded: boolean | undefined) {
    this.yBenchmark.set("minExcluded", minExcluded || false);
  }

  get max(): number | undefined {
    return this.yBenchmark.get("max") as number | undefined;
  }
  set max(max: number | undefined) {
    this.yBenchmark.set("max", max || 0);
  }

  get maxExcluded(): boolean | undefined {
    return this.yBenchmark.get("maxExcluded") as boolean | undefined;
  }
  set maxExcluded(maxExcluded: boolean | undefined) {
    this.yBenchmark.set("maxExcluded", maxExcluded || false);
  }

  hash = (benchmark: Benchmark): string => {
    return JSON.stringify(benchmark);
  };

  snapshot = (): Benchmark => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      icon: this.icon,
      min: this.min,
      minExcluded: this.minExcluded,
      max: this.max,
      maxExcluded: this.maxExcluded,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  id = (): Guid => {
    return this.guid;
  };

  change = (diff: BenchmarkDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.icon !== undefined) this.icon = diff.icon;
    if (diff.min !== undefined) this.min = diff.min;
    if (diff.minExcluded !== undefined) this.minExcluded = diff.minExcluded;
    if (diff.max !== undefined) this.max = diff.max;
    if (diff.maxExcluded !== undefined) this.maxExcluded = diff.maxExcluded;
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yBenchmark, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yBenchmark, subscribe, true);
  };
}

// #endregion 🔖Benchmark

// #region 🔖Quality

type YQuality = Y.Map<string | number | YAttributes>;
type YQualities = Y.Array<YQuality>;

export class QualityStore {
  private yQuality: YQuality;
  private cache?: Quality;
  private cacheHash?: string;

  constructor(yQuality: YQuality, quality: Quality) {
    this.yQuality = yQuality;
    this.guid = quality.guid;
    this.key = quality.key;
    this.name = quality.name;
    this.unit = quality.unit;
    this.description = quality.description;
  }

  get guid(): string {
    return this.yQuality.get("guid") as string;
  }
  set guid(guid: string) {
    this.yQuality.set("guid", guid);
  }

  get key(): string {
    return this.yQuality.get("key") as string;
  }
  set key(key: string) {
    this.yQuality.set("key", key);
  }

  get name(): string {
    return this.yQuality.get("name") as string;
  }
  set name(name: string) {
    this.yQuality.set("name", name);
  }

  get folder(): string | undefined {
    return this.yQuality.get("folder") as string | undefined;
  }
  set folder(folder: string | undefined) {
    if (folder) this.yQuality.set("folder", folder);
    else this.yQuality.delete("folder");
  }

  get unit(): string | undefined {
    return this.yQuality.get("unit") as string | undefined;
  }
  set unit(unit: string | undefined) {
    this.yQuality.set("unit", unit || "");
  }

  get description(): string | undefined {
    return this.yQuality.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yQuality.set("description", description || "");
  }

  id(): Guid {
    return this.guid;
  }

  hash = (quality: Quality): string => {
    return JSON.stringify(quality);
  };

  snapshot(): Quality {
    const currentHash = this.hash({
      guid: this.guid,
      key: this.key,
      name: this.name,
      folder: this.folder,
      unit: this.unit,
      description: this.description,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const quality: Quality = {
      guid: this.guid,
      key: this.key,
      name: this.name,
      folder: this.folder,
      unit: this.unit,
      description: this.description,
    };

    this.cache = quality;
    this.cacheHash = currentHash;
    return quality;
  }

  change = (diff: QualityDiff) => {
    if (diff.key !== undefined) this.key = diff.key;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.folder !== undefined) this.folder = diff.folder;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.description !== undefined) this.description = diff.description;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yQuality, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yQuality, subscribe, true);
  };
}

// #endregion 🔖Quality

// #region 🔖Prop

type YProp = Y.Map<string | number | boolean | YAttributes>;
type YProps = Y.Array<YProp>;

class PropStore {
  private yProp: YProp;
  private cache?: Prop;
  private cacheHash?: string;

  constructor(yProp: YProp, prop: Prop) {
    this.yProp = yProp;
    this.guid = prop.guid;
    this.quality = prop.quality;
    this.value = prop.value;
    this.unit = prop.unit;
  }

  get guid(): string {
    return this.yProp.get("guid") as string;
  }
  set guid(guid: string) {
    this.yProp.set("guid", guid);
  }

  get quality(): QualityId {
    return { guid: this.yProp.get("quality") as string };
  }
  set quality(quality: QualityId) {
    this.yProp.set("quality", quality.guid);
  }

  get value(): string | undefined {
    return this.yProp.get("value") as string | undefined;
  }
  set value(value: string | undefined) {
    this.yProp.set("value", value || "");
  }

  get unit(): string | undefined {
    return this.yProp.get("unit") as string | undefined;
  }
  set unit(unit: string | undefined) {
    this.yProp.set("unit", unit || "");
  }

  id(): Guid {
    return this.guid;
  }

  hash = (prop: Prop): string => {
    return JSON.stringify(prop);
  };

  snapshot(): Prop {
    const currentHash = this.hash({
      guid: this.guid,
      quality: this.quality,
      value: this.value || "",
      unit: this.unit,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const prop: Prop = {
      guid: this.guid,
      quality: this.quality,
      value: this.value || "",
      unit: this.unit,
    };

    this.cache = prop;
    this.cacheHash = currentHash;
    return prop;
  }

  change = (diff: PropDiff) => {
    if (diff.quality !== undefined) this.quality = diff.quality;
    if (diff.value !== undefined) this.value = diff.value;
    if (diff.unit !== undefined) this.unit = diff.unit;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yProp, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yProp, subscribe, true);
  };
}

// #endregion 🔖Prop

// #region 🔖Model

type YModelVal = string | Y.Array<string> | YAttributes;
type YModel = Y.Map<YModelVal>;
type YModels = Y.Array<YModel>;

class ModelStore {
  private yModel: YModel;
  private yTags: Y.Array<string>;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Model;
  private cacheHash?: string;

  constructor(yModel: YModel, model: Model) {
    this.yModel = yModel;
    this.guid = model.guid;

    this.yModel.set("file", typeof model.file === "string" ? model.file : model.file.guid);
    this.description = model.description;
    const yTags = new Y.Array<string>();
    this.yModel.set("tags", yTags);
    this.yTags = yTags;

    if (model.tags) this.yTags.push(model.tags.map((t) => (typeof t === "string" ? t : t.guid)));
    this.attributes = new Map();
    const yAttributes = new Y.Array<YAttribute>();
    this.yModel.set("attributes", yAttributes);
    this.yAttributes = yAttributes;
    if (model.attributes) {
      for (const attribute of model.attributes) {
        const yAttribute = new Y.Map<YAttributeVal>();

        yAttribute.set("guid", attribute.guid);
        yAttribute.set("key", attribute.key);
        yAttribute.set("value", attribute.value || "");
        yAttribute.set("definition", attribute.definition || "");
        this.yAttributes.push([yAttribute]);
        const attributeStore = new AttributeStore(yAttribute, attribute);
        this.attributes.set(attribute.guid, attributeStore);
      }
    }
  }

  get guid(): string {
    return this.yModel.get("guid") as string;
  }
  set guid(guid: string) {
    this.yModel.set("guid", guid);
  }

  get file(): FileId {
    const fileGuid = this.yModel.get("file") as string;
    return { guid: fileGuid };
  }
  set file(file: FileId | string) {
    this.yModel.set("file", typeof file === "string" ? file : file.guid);
  }

  get description(): string | undefined {
    return this.yModel.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yModel.set("description", description || "");
  }

  hash = (model: Model): string => {
    return JSON.stringify(model);
  };

  snapshot(): Model {
    const tags: TagId[] = this.yTags.toArray().map((guid) => ({ guid }));
    const currentHash = this.hash({
      guid: this.guid,
      file: this.file,
      description: this.description,
      tags,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const model: Model = {
      guid: this.guid,
      file: this.file,
      description: this.description,
      tags,
    };

    this.cache = model;
    this.cacheHash = currentHash;
    return model;
  }

  apply(diff: ModelDiff): void {
    if (diff.file !== undefined) this.file = diff.file;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.tags !== undefined) {
      this.yTags.delete(0, this.yTags.length);
      if (diff.tags.length > 0) {
        this.yTags.push(diff.tags.map((t) => (typeof t === "string" ? t : t.guid)));
      }
    }
  }

  change = (diff: ModelDiff) => {
    this.apply(diff);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yModel, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yModel, subscribe, true);
  };
}

// #endregion 🔖Model

// #region 🔖Connector

type YConnectorVal = string | number | boolean | YAttributes | Y.Array<string> | YPoint | YVector | YProps;
type YConnector = Y.Map<YConnectorVal>;
type YConnectors = Y.Array<YConnector>;

class ConnectorStore {
  private yConnector: YConnector;
  private yPoint: YPoint;
  private point: YPointStore;
  private yDirection: YVector;
  private direction: YVectorStore;
  private cache?: Connector;
  private cacheHash?: string;

  constructor(yConnector: YConnector, connector: Connector) {
    this.yConnector = yConnector;
    this.guid = connector.guid;
    this.localId = connector.name;
    this.description = connector.description;
    this.port = connector.port?.guid;
    this.mandatory = connector.mandatory;
    this.t = connector.t;

    this.yPoint = new Y.Map();
    this.yConnector.set("point", this.yPoint);
    this.point = new YPointStore(this.yPoint, connector.point);

    this.yDirection = new Y.Map();
    this.yConnector.set("direction", this.yDirection);
    this.direction = new YVectorStore(this.yDirection, connector.direction);
  }

  get guid(): string {
    return this.yConnector.get("guid") as string;
  }
  set guid(guid: string) {
    this.yConnector.set("guid", guid);
  }

  get localId(): string | undefined {
    return this.yConnector.get("id_") as string | undefined;
  }
  set localId(id_: string | undefined) {
    this.yConnector.set("id_", id_ || "");
  }

  get description(): string | undefined {
    return this.yConnector.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yConnector.set("description", description || "");
  }

  get port(): string | undefined {
    return this.yConnector.get("port") as string | undefined;
  }
  set port(port_: string | undefined) {
    this.yConnector.set("port", port_ || "");
  }

  get mandatory(): boolean | undefined {
    return this.yConnector.get("mandatory") as boolean | undefined;
  }
  set mandatory(mandatory: boolean | undefined) {
    if (mandatory !== undefined) this.yConnector.set("mandatory", mandatory);
  }

  get t(): number {
    return this.yConnector.get("t") as number;
  }
  set t(t: number) {
    this.yConnector.set("t", t);
  }

  hash = (connector: Connector): string => {
    return JSON.stringify(connector);
  };

  snapshot = (): Connector => {
    const currentData = {
      guid: this.guid,
      name: this.localId,
      description: this.description,
      port: this.port ? { guid: this.port } : undefined,
      mandatory: this.mandatory,
      t: this.t,
      point: this.point.snapshot(),
      direction: this.direction.snapshot(),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  apply(diff: ConnectorDiff): void {
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.port !== undefined) this.port = diff.port?.guid;
    if (diff.mandatory !== undefined) this.mandatory = diff.mandatory;
    if (diff.t !== undefined) this.t = diff.t;
  }

  change = (diff: ConnectorDiff) => {
    this.apply(diff);
    if (diff.point !== undefined) this.point.change(diff.point);
    if (diff.direction !== undefined) this.direction.change(diff.direction);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yConnector, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yConnector, subscribe, true);
  };
}

// #endregion 🔖Connector

// #region 🔖Type

type YTypeVal = string | number | boolean | YAuthorUuids | YAttributes | YModels | YConnectors | YProps | YLocation;
type YType = Y.Map<YTypeVal>;
type YTypes = Y.Array<YType>;

export class TypeStore {
  public readonly parent: KitStore;
  private yType: YType;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private yAuthors: YAuthorUuids;
  private authors: Map<string, AuthorStore>;
  private yModels: YModels;
  private yConnectors: YConnectors;
  public models: Map<string, ModelStore>;
  public connectors: Map<string, ConnectorStore>;
  private cache?: Type;
  private cacheHash?: string;

  constructor(parent: KitStore, yType: YType, type: Type) {
    this.parent = parent;
    this.yType = yType;
    this.models = new Map();
    this.connectors = new Map();

    this.guid = type.guid;
    this.name = type.name;
    this.parentGuid = type.parent?.guid;
    this.abstract = type.isAbstract;
    this.stock = type.stock;
    this.virtual = type.virtual;
    this.unit = type.unit;
    this.icon = type.icon;
    this.image = type.image;
    this.description = type.description;

    this.attributes = new Map();
    const yTypeAttributes = new Y.Array<YAttribute>();
    this.yType.set("attributes", yTypeAttributes);
    this.yAttributes = yTypeAttributes;

    if (type.attributes) {
      type.attributes.forEach((attribute) => this.createAttribute(attribute));
    }

    this.authors = new Map();
    const yTypeAuthors = new Y.Array<YAuthorUuid>();
    this.yType.set("authors", yTypeAuthors);
    this.yAuthors = yTypeAuthors;
    if (type.authors) {
      for (const author of type.authors) {
        if (!author?.guid) continue;
        const authorStore = this.parent.author(author.guid);
        if (!authorStore) continue;
        this.authors.set(authorStore.guid, authorStore);
        this.yAuthors.push([authorStore.guid]);
      }
    }

    const yTypeModels = new Y.Array<YModel>();
    this.yType.set("models", yTypeModels);
    this.yModels = yTypeModels;

    if (type.models) {
      type.models.forEach((model) => this.createModel(model));
    }

    const yTypePorts = new Y.Array<YConnector>();
    this.yType.set("connectors", yTypePorts);
    this.yConnectors = yTypePorts;
    if (type.connectors) {
      for (const connector of type.connectors) {
        this.createPort(connector);
      }
    }

    this.yType.set("createdAt", new Date().toISOString());
    this.updated();
  }

  get guid(): string {
    return this.yType.get("guid") as string;
  }
  set guid(guid: string) {
    this.yType.set("guid", guid);
  }

  get name(): string {
    return this.yType.get("name") as string;
  }
  set name(name: string) {
    this.yType.set("name", name);
  }
  get parentGuid(): string | undefined {
    return this.yType.get("parent") as string | undefined;
  }
  set parentGuid(parent: string | undefined) {
    if (parent) this.yType.set("parent", parent);
    else this.yType.delete("parent");
  }
  get folder(): string | undefined {
    return this.yType.get("folder") as string | undefined;
  }
  set folder(folder: string | undefined) {
    if (folder) this.yType.set("folder", folder);
    else this.yType.delete("folder");
  }
  get abstract(): boolean | undefined {
    return this.yType.get("isAbstract") as boolean | undefined;
  }
  set abstract(isAbstract: boolean | undefined) {
    if (isAbstract) this.yType.set("isAbstract", isAbstract);
    else this.yType.delete("isAbstract");
  }
  get stock(): number | undefined {
    return this.yType.get("stock") as number | undefined;
  }
  set stock(stock: number | undefined) {
    if (stock !== undefined) this.yType.set("stock", stock);
  }
  get virtual(): boolean | undefined {
    return this.yType.get("virtual") as boolean | undefined;
  }
  set virtual(virtual: boolean | undefined) {
    if (virtual !== undefined) this.yType.set("virtual", virtual);
  }
  get unit(): string | undefined {
    return this.yType.get("unit") as string | undefined;
  }
  set unit(unit: string | undefined) {
    this.yType.set("unit", unit || "");
  }
  get icon(): string | undefined {
    return this.yType.get("icon") as string | undefined;
  }
  set icon(icon: string | undefined) {
    this.yType.set("icon", icon || "");
  }
  get image(): string | undefined {
    return this.yType.get("image") as string | undefined;
  }
  set image(image: string | undefined) {
    this.yType.set("image", image || "");
  }
  get description(): string | undefined {
    return this.yType.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yType.set("description", description || "");
  }
  get createdAt(): Date {
    return new Date(this.yType.get("createdAt") as string);
  }
  get updatedAt(): Date {
    return new Date(this.yType.get("updatedAt") as string);
  }

  updated(): void {
    this.yType.set("updatedAt", new Date().toISOString());
  }

  hasAttribute(identifier: string): boolean {
    return this.findAttributeStore(identifier) !== undefined;
  }

  createAttribute(attribute: Attribute): void {
    if (!attribute.guid) throw new Error("Attribute guid is required.");
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  private findAttributeStore(identifier: string): AttributeStore | undefined {
    const byGuid = this.attributes.get(identifier);
    if (byGuid) return byGuid;
    for (const attribute of this.attributes.values()) {
      if (attribute.key === identifier) {
        return attribute;
      }
    }
    return undefined;
  }

  private findAttributeIndexByGuid(guid: string): number {
    for (let index = 0; index < this.yAttributes.length; index += 1) {
      const yAttribute = this.yAttributes.get(index) as YAttribute | undefined;
      if (!yAttribute) continue;
      if ((yAttribute.get("guid") as string | undefined) === guid) {
        return index;
      }
    }
    return -1;
  }

  createModel(model: Model): void {
    const yModel = new Y.Map<YModelVal>();
    this.yModels.push([yModel]);
    const yModelStore = new ModelStore(yModel, model);
    this.models.set(model.guid, yModelStore);
  }

  hasModel(guid: string): boolean {
    return this.models.has(guid);
  }

  model(guid: string): ModelStore {
    const rep = this.models.get(guid);
    if (!rep) throw new Error(`Model store not found for guid ${guid}`);
    return rep;
  }

  hasPort(guid: string): boolean {
    return this.connectors.has(guid);
  }

  createPort(connector: Connector): void {
    if (this.hasPort(connector.guid)) throw new Error(`Connector (${connector.guid}) already exists.`);
    const yConnector = new Y.Map<YConnectorVal>();
    this.yConnectors.push([yConnector]);
    const yConnectorStore = new ConnectorStore(yConnector, connector);
    this.connectors.set(connector.guid, yConnectorStore);
  }

  connector(guid: string): ConnectorStore {
    const p = this.connectors.get(guid);
    if (!p) throw new Error(`Connector store not found for guid ${guid}`);
    return p;
  }

  id(): Guid {
    return this.guid;
  }

  hash = (type: Type): string => {
    return JSON.stringify(type);
  };
  snapshot = (): Type => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      parent: this.parentGuid ? { guid: this.parentGuid } : undefined,
      folder: this.folder,
      isAbstract: this.abstract,
      stock: this.stock,
      virtual: this.virtual,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
      authors: Array.from(this.authors.values()).map((a) => ({ guid: a.guid })),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
      models: Array.from(this.models.values()).map((rep) => rep.snapshot()),
      connectors: Array.from(this.connectors.values()).map((connector) => connector.snapshot()),
      createdAt: this.createdAt?.toISOString(),
      updatedAt: this.updatedAt?.toISOString(),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: TypeDiff) => {
    this.parent.yDoc.transact(() => {
      if (diff.name !== undefined) this.yType.set("name", diff.name);
      if (diff.parent !== undefined) {
        if (diff.parent) this.yType.set("parent", diff.parent.guid);
        else this.yType.delete("parent");
      }
      if (diff.folder !== undefined) {
        if (diff.folder) this.yType.set("folder", diff.folder);
        else this.yType.delete("folder");
      }
      if (diff.isAbstract !== undefined) {
        if (diff.isAbstract) this.yType.set("isAbstract", diff.isAbstract);
        else this.yType.delete("isAbstract");
      }
      if (diff.stock !== undefined) this.yType.set("stock", diff.stock);
      if (diff.virtual !== undefined) this.yType.set("virtual", diff.virtual);
      if (diff.unit !== undefined) this.yType.set("unit", diff.unit);
      if (diff.icon !== undefined) this.yType.set("icon", diff.icon || "");
      if (diff.image !== undefined) this.yType.set("image", diff.image || "");
      if (diff.description !== undefined) this.yType.set("description", diff.description || "");
      if (diff.createdAt !== undefined) this.yType.set("createdAt", diff.createdAt);
      if (diff.updatedAt !== undefined) this.yType.set("updatedAt", diff.updatedAt);

      if (diff.authors !== undefined && diff.authors !== null) {
        this.yAuthors.delete(0, this.yAuthors.length);
        this.authors = new Map(
          diff.authors.map((authorId) => {
            const author = this.parent.author(authorId.guid);
            return [author.guid, author];
          }),
        );
        this.authors.forEach((author) => this.yAuthors.push([author.guid]));
      }

      if (diff.models) {
        if (diff.models.removed) {
          diff.models.removed.forEach((modelId) => {
            const guid = modelId.guid;
            const index = Array.from(this.models.keys()).indexOf(guid);
            if (index !== -1) {
              this.yModels.delete(index, 1);
              this.models.delete(guid);
            }
          });
        }
        if (diff.models.added) {
          diff.models.added.forEach((model) => {
            this.createModel(model);
          });
        }
        if (diff.models.updated) {
          diff.models.updated.forEach(({ model, diff: repDiff }) => {
            const rep = this.models.get(model.guid);
            if (rep) rep.apply(repDiff);
          });
        }
      }

      if (diff.connectors) {
        if (diff.connectors.removed) {
          diff.connectors.removed.forEach((connectorId) => {
            const guid = connectorId.guid;
            const index = Array.from(this.connectors.keys()).indexOf(guid);
            if (index !== -1) {
              this.yConnectors.delete(index, 1);
              this.connectors.delete(guid);
            }
          });
        }
        if (diff.connectors.added) {
          diff.connectors.added.forEach((connector) => {
            this.createPort(connector);
          });
        }
        if (diff.connectors.updated) {
          diff.connectors.updated.forEach(({ connector, diff: connectorDiff }) => {
            const p = this.connectors.get(connector.guid);
            if (p) p.change(connectorDiff);
          });
        }
      }

      if (diff.attributes) {
        if (diff.attributes.removed) {
          diff.attributes.removed.forEach((identifier) => {
            const attribute = this.findAttributeStore(identifier.guid);
            if (!attribute) return;
            const index = this.findAttributeIndexByGuid(attribute.guid);
            if (index !== -1) {
              this.yAttributes.delete(index, 1);
            }
            this.attributes.delete(attribute.guid);
          });
        }
        if (diff.attributes.added) {
          diff.attributes.added.forEach((attribute) => {
            this.createAttribute(attribute);
          });
        }
        if (diff.attributes.updated) {
          diff.attributes.updated.forEach(({ attribute, diff: attributeDiff }) => {
            const attr = this.findAttributeStore(attribute.guid);
            if (!attr) return;
            attr.change(attributeDiff);
          });
        }
      }

      this.cache = undefined;
      this.cacheHash = undefined;
    });
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, true);
  };
}

type TypeScope = { guid: string };
const TypeScopeContext = createContext<TypeScope | null>(null);
export const TypeScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(TypeScopeContext.Provider, { value }, props.children as any);
};
export const useTypeScope = () => useContext(TypeScopeContext);
export const useIsInTypeScope = () => useTypeScope() !== null;

function useTypeStore<T>(selector?: (store: TypeStore) => T, guid?: string): T | TypeStore | null {
  const kitStore = useKitStore() as KitStore | null;
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? guid;
  if (!kitStore) return null;
  if (!typeGuid) return null;
  if (!kitStore.hasType(typeGuid)) return null;
  const typeStore = kitStore.type(typeGuid);
  if (!typeStore) return null;
  return selector ? selector(typeStore) : typeStore;
}

export function useType<T>(selector?: (type: Type) => T, id?: Guid, deep: boolean = false): T | Type | null {
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? id;
  const store = useTypeStore(identitySelector, typeGuid ?? undefined);
  const syncedValue = useSyncOptional<Type, T>(store as TypeStore | null, selector ? selector : (identitySelector as any));
  if (!typeGuid || !store) return null;
  return syncedValue;
}

type QualityScope = { guid: string };
const QualityScopeContext = createContext<QualityScope | null>(null);
export const QualityScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(QualityScopeContext.Provider, { value }, props.children as any);
};
export const useQualityScope = () => useContext(QualityScopeContext);
export const useIsInQualityScope = () => useQualityScope() !== null;

function useQualityStore<T>(selector?: (store: QualityStore) => T, guid?: string): T | QualityStore | null {
  const kitStore = useKitStore() as KitStore | null;
  const qualityScope = useQualityScope();
  const qualityGuid = qualityScope?.guid ?? guid;
  if (!kitStore || !qualityGuid || !kitStore.hasQuality(qualityGuid)) return null;
  const qualityStore = kitStore.quality(qualityGuid);
  if (!qualityStore) return null;
  return selector ? selector(qualityStore) : qualityStore;
}

export function useQuality<T>(selector?: (quality: Quality) => T, id?: Guid, deep: boolean = false): T | Quality | null {
  const qualityScope = useQualityScope();
  const qualityGuid = qualityScope?.guid ?? id;
  const store = useQualityStore(identitySelector, qualityGuid ?? undefined) as QualityStore | null;
  const synced = useSyncOptional<Quality, T>(store as any, selector ? selector : (identitySelector as any));
  if (!store) return null;
  return synced;
}

// #endregion 🔖Type

// #region 🔖Layer

type YLayer = Y.Map<string | boolean | YAttributes>;
type YLayers = Y.Array<YLayer>;

class LayerStore {
  private yLayer: YLayer;
  private cache?: Layer;
  private cacheHash?: string;

  constructor(yLayer: YLayer, layer: Layer) {
    this.yLayer = yLayer;
    this.guid = layer.guid;
    this.path = layer.path;
    this.isHidden = layer.isHidden;
    this.isLocked = layer.isLocked;
    this.color = layer.color;
    this.description = layer.description;
  }

  get guid(): string {
    return this.yLayer.get("guid") as string;
  }
  set guid(guid: string) {
    this.yLayer.set("guid", guid);
  }

  get path(): string {
    return this.yLayer.get("path") as string;
  }
  set path(path: string) {
    this.yLayer.set("path", path);
  }

  get isHidden(): boolean | undefined {
    return this.yLayer.get("isHidden") as boolean | undefined;
  }
  set isHidden(isHidden: boolean | undefined) {
    if (isHidden !== undefined) this.yLayer.set("isHidden", isHidden);
  }

  get isLocked(): boolean | undefined {
    return this.yLayer.get("isLocked") as boolean | undefined;
  }
  set isLocked(isLocked: boolean | undefined) {
    if (isLocked !== undefined) this.yLayer.set("isLocked", isLocked);
  }

  get color(): string | undefined {
    return this.yLayer.get("color") as string | undefined;
  }
  set color(color: string | undefined) {
    this.yLayer.set("color", color || "");
  }

  get description(): string | undefined {
    return this.yLayer.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yLayer.set("description", description || "");
  }

  id(): string {
    return this.path;
  }

  hash = (layer: Layer): string => {
    return JSON.stringify(layer);
  };

  snapshot(): Layer {
    const currentHash = this.hash({
      guid: this.guid,
      path: this.path,
      isHidden: this.isHidden,
      isLocked: this.isLocked,
      color: this.color,
      description: this.description,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const layer: Layer = {
      guid: this.guid,
      path: this.path,
      isHidden: this.isHidden,
      isLocked: this.isLocked,
      color: this.color,
      description: this.description,
    };

    this.cache = layer;
    this.cacheHash = currentHash;
    return layer;
  }

  change = (diff: LayerDiff) => {
    if (diff.path !== undefined) this.path = diff.path;
    if (diff.isHidden !== undefined) this.isHidden = diff.isHidden;
    if (diff.isLocked !== undefined) this.isLocked = diff.isLocked;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.description !== undefined) this.description = diff.description;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yLayer, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yLayer, subscribe, true);
  };
}

// #endregion 🔖Layer

// #region 🔖Piece

type YPieceVal = string | number | boolean | YPlane | YAttributes | YCoord;
type YPiece = Y.Map<YPieceVal>;
type YPieces = Y.Array<YPiece>;

class PieceStore {
  public readonly parent: DesignStore;
  private yPiece: YPiece;
  private yPlane: YPlane | undefined;
  private plane: YPlaneStore | undefined;
  private yCenter: YCoord | undefined;
  private center: YCoordStore | undefined;
  private yMirrorPlane: YPlane | undefined;
  private mirrorPlane: YPlaneStore | undefined;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Piece;
  private cacheHash?: string;
  private dirty: boolean = true;

  constructor(parent: DesignStore, yPiece: YPiece, piece: Piece) {
    this.parent = parent;
    this.yPiece = yPiece;
    this.guid = piece.guid;
    this.attributes = new Map();

    this.localId = piece.guid;
    if (piece.type) {
      const type = this.parent.parent.type(piece.type.guid);
      if (type) this.yPiece.set("type", type.guid);
    } else {
      const design = this.parent.parent.design(piece.design!.guid);
      this.yPiece.set("design", design.guid);
    }
    this.name = piece.name;
    this.scale = piece.scale;
    this.isHidden = piece.isHidden;
    this.isLocked = piece.isLocked;
    this.color = piece.color;
    this.description = piece.description;

    if (piece.plane) {
      this.yPlane = new Y.Map();
      this.yPiece.set("plane", this.yPlane);
      this.plane = new YPlaneStore(this.yPlane, piece.plane);
    } else {
      this.yPlane = undefined;
      this.plane = undefined;
    }

    if (piece.center) {
      this.yCenter = new Y.Map();
      this.yPiece.set("center", this.yCenter);
      this.center = new YCoordStore(this.yCenter, piece.center);
    } else {
      this.yCenter = undefined;
      this.center = undefined;
    }

    if (piece.mirrorPlane) {
      this.yMirrorPlane = new Y.Map();
      this.yPiece.set("mirrorPlane", this.yMirrorPlane);
      this.mirrorPlane = new YPlaneStore(this.yMirrorPlane, piece.mirrorPlane);
    } else {
      this.yMirrorPlane = undefined;
      this.mirrorPlane = undefined;
    }

    const yPieceAttributes = new Y.Array<YAttribute>();
    this.yPiece.set("attributes", yPieceAttributes);
    this.yAttributes = yPieceAttributes;
    if (piece.attributes) {
      for (const attribute of piece.attributes) {
        this.createAttribute(attribute);
      }
    }
  }

  get guid(): string {
    return this.yPiece.get("guid") as string;
  }
  set guid(guid: string) {
    this.yPiece.set("guid", guid);
  }

  get localId(): string {
    return this.yPiece.get("id_") as string;
  }
  set localId(localId: string) {
    this.yPiece.set("id_", localId);
  }
  get type(): Guid | undefined {
    const typeUuid = this.yPiece.get("type") as string;
    const typeStore = typeUuid ? this.parent.parent.type(typeUuid) : undefined;
    return typeStore ? typeStore.id() : undefined;
  }
  set type(type: Guid | undefined) {
    if (type) {
      const typeStore = this.parent.parent.type(type);
      if (typeStore) this.yPiece.set("type", typeStore.guid);
    } else {
      this.yPiece.delete("type");
    }
  }
  get design(): Guid | undefined {
    const designUuid = this.yPiece.get("design") as string;
    return designUuid ? this.parent.parent.design(designUuid).id() : undefined;
  }
  set design(design: Guid | undefined) {
    if (design) {
      this.yPiece.set("design", this.parent.parent.design(design).guid);
    } else {
      this.yPiece.delete("design");
    }
  }
  get scale(): number {
    return (this.yPiece.get("scale") as number) ?? 1.0;
  }
  set scale(scale: number | undefined) {
    this.yPiece.set("scale", scale || 1.0);
  }
  get isHidden(): boolean {
    return (this.yPiece.get("isHidden") as boolean) ?? false;
  }
  set isHidden(isHidden: boolean | undefined) {
    this.yPiece.set("isHidden", isHidden || false);
  }
  get isLocked(): boolean {
    return (this.yPiece.get("isLocked") as boolean) ?? false;
  }
  set isLocked(isLocked: boolean | undefined) {
    this.yPiece.set("isLocked", isLocked || false);
  }
  get color(): string | undefined {
    return this.yPiece.get("color") as string | undefined;
  }
  set color(color: string | undefined) {
    this.yPiece.set("color", color || "");
  }
  get name(): string | undefined {
    return this.yPiece.get("name") as string | undefined;
  }
  set name(name: string | undefined) {
    this.yPiece.set("name", name || "");
  }
  get description(): string | undefined {
    return this.yPiece.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yPiece.set("description", description || "");
  }

  hasAttribute(guid: string): boolean {
    return this.attributes.has(guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  attribute(guid: string): AttributeStore {
    return this.attributes.get(guid)!;
  }

  id(): Guid {
    return this.guid;
  }

  public hash(piece: Piece): string {
    return JSON.stringify(piece);
  }

  snapshot = (): Piece => {
    if (!this.dirty && this.cache) {
      return this.cache;
    }

    const currentData = {
      guid: this.guid,
      id_: this.localId,
      name: this.name,
      type: this.type ? { guid: this.type } : undefined,
      design: this.design ? { guid: this.design } : undefined,
      scale: this.scale,
      isHidden: this.isHidden,
      isLocked: this.isLocked,
      color: this.color,
      description: this.description,
      plane: this.plane?.snapshot(),
      center: this.center?.snapshot(),
      mirrorPlane: this.mirrorPlane?.snapshot(),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
    };

    this.cache = currentData;
    this.dirty = false;
    return this.cache;
  };

  change = (diff: PieceDiff) => {
    this.dirty = true;
    this.parent.markDirty();
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.type !== undefined) this.type = diff.type?.guid;
    if (diff.design !== undefined) this.design = diff.design?.guid;
    if (diff.scale !== undefined) this.scale = diff.scale;
    if (diff.isHidden !== undefined) this.isHidden = diff.isHidden;
    if (diff.isLocked !== undefined) this.isLocked = diff.isLocked;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.description !== undefined) this.description = diff.description;

    if (diff.plane !== undefined) {
      if (diff.plane) {
        if (!this.plane) {
          const yPlane = new Y.Map() as YPlane;
          this.yPiece.set("plane", yPlane);
          this.yPlane = yPlane;
          this.plane = new YPlaneStore(this.yPlane, diff.plane as Plane);
        } else {
          this.plane.change(diff.plane);
        }
      } else {
        this.yPiece.delete("plane");
        this.plane = undefined;
        this.yPlane = undefined;
      }
    }

    if (diff.center !== undefined) {
      if (diff.center) {
        if (!this.center) {
          const yCenter = new Y.Map() as YCoord;
          this.yPiece.set("center", yCenter);
          this.yCenter = yCenter;
          this.center = new YCoordStore(this.yCenter, diff.center);
        } else {
          this.center.change(diff.center);
        }
      } else {
        this.yPiece.delete("center");
        this.center = undefined;
        this.yCenter = undefined;
      }
    }

    if (diff.mirrorPlane !== undefined) {
      if (diff.mirrorPlane) {
        if (!this.mirrorPlane) {
          const yMirrorPlane = new Y.Map() as YPlane;
          this.yPiece.set("mirrorPlane", yMirrorPlane);
          this.yMirrorPlane = yMirrorPlane;
          this.mirrorPlane = new YPlaneStore(this.yMirrorPlane, diff.mirrorPlane);
        } else {
          this.mirrorPlane.change(diff.mirrorPlane);
        }
      } else {
        this.yPiece.delete("mirrorPlane");
        this.mirrorPlane = undefined;
        this.yMirrorPlane = undefined;
      }
    }

    if (diff.attributes !== undefined) {
      this.attributes = new Map();
      this.yAttributes.delete(0, this.yAttributes.length);

      if (diff.attributes.added) {
        for (const attribute of diff.attributes.added) {
          this.createAttribute(attribute);
        }
      }
    }
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPiece, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPiece, subscribe, true);
  };
}

type PieceScope = { guid: string };
const PieceScopeContext = createContext<PieceScope | null>(null);
export const PieceScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(PieceScopeContext.Provider, { value }, props.children as any);
};
export const usePieceScope = () => useContext(PieceScopeContext);

function usePieceStore<T>(selector?: (store: PieceStore) => T, guid?: string): T | PieceStore {
  const designStore = useDesignStore() as DesignStore;
  const pieceScope = usePieceScope();
  const pieceGuid = pieceScope?.guid ?? guid;
  if (!pieceGuid) throw new Error("usePieceStore must be called within a PieceScopeProvider or be directly provided with a guid");
  const pieceStore = designStore.piece(pieceGuid);
  if (!pieceStore) throw new Error(`Piece store not found for piece ${pieceGuid}`);
  return selector ? selector(pieceStore) : pieceStore;
}

export function usePiece<T>(selector?: (piece: Piece) => T, id?: Guid, deep: boolean = false): T | Piece | null {
  return useSync<Piece, T>(usePieceStore(identitySelector, id) as PieceStore, selector ? selector : (identitySelector as any));
}

export function useCurrentPiecePlane(): Plane {
  const plane = usePiece((p) => p.plane) as Plane | undefined;

  if (!plane) {
    return {
      origin: { x: 0, y: 0, z: 0 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    };
  }

  return plane;
}

export type PieceMetadata = {
  plane: Plane;
  center: Coord;
  fixedPieceId: string;
  parentPieceId: string | null;
  depth: number;
};

export function usePiecesMetadataMap(): Map<string, PieceMetadata> {
  const kitStore = useKitStore(identitySelector) as KitStore | null;
  const designStore = useDesignStore(identitySelector) as DesignStore | null;
  const designScope = useDesignScope();
  const emptyMap = useMemo(() => new Map<string, PieceMetadata>(), []);
  const key = designScope ? `piecesMetadata:${designScope.guid}` : "";
  const deps: BaseDependency[] = useMemo(() => {
    if (!designStore) return [];
    return [
      { store: designStore, path: [yPathMapKey("pieces")] },
      { store: designStore, path: [yPathMapKey("connections")] },
    ];
  }, [designStore]);
  const compute = useCallback(() => {
    if (!kitStore || !designScope) return new Map<string, PieceMetadata>();
    const kit = kitStore.snapshot();
    return piecesMetadata(kit, designScope.guid);
  }, [kitStore, designScope?.guid]);
  if (!designStore || !designScope) return emptyMap;
  return useDerived(designStore.derived, key, deps, compute) ?? emptyMap;
}

export function usePieceMetadata(pieceId?: Guid): PieceMetadata | undefined {
  const pieceScope = usePieceScope();
  const resolvedPieceId = pieceId ?? pieceScope?.guid;
  const metadataMap = usePiecesMetadataMap();
  return resolvedPieceId ? metadataMap.get(resolvedPieceId) : undefined;
}

export function useFlatPiecePlane(id?: Guid): Plane {
  const meta = usePieceMetadata(id);
  return meta?.plane ?? { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
}

export function useFlatPieceCenter(id?: Guid): Coord {
  const meta = usePieceMetadata(id);
  return meta?.center ?? { u: 0, v: 0 };
}

export function useIsConnectedPiece(id?: Guid): boolean {
  const meta = usePieceMetadata(id);
  return meta ? meta.parentPieceId !== null : false;
}

export function usePieceDepth(id?: Guid): number {
  const meta = usePieceMetadata(id);
  return meta?.depth ?? 0;
}

export function useFixedPieceId(id?: Guid): string | undefined {
  const meta = usePieceMetadata(id);
  return meta?.fixedPieceId;
}

export function useParentPieceId(id?: Guid): string | null {
  const meta = usePieceMetadata(id);
  return meta?.parentPieceId ?? null;
}

export function usePieceParentConnection(id?: Guid): Connection | null {
  const pieceScope = usePieceScope();
  const pieceGuid = (typeof id === "string" ? id : (pieceScope?.guid ?? null)) as string | null;
  const designStore = useDesignStore(identitySelector) as DesignStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!designStore) return () => {};
      return designStore.onConnectionsChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [designStore],
  );

  const getSnapshot = useCallback(() => {
    if (!pieceGuid || !designStore) return null;
    const connections = designStore.snapshotConnections();
    return connections.find((c: Connection) => c.connecting.piece.guid === pieceGuid || c.connected.piece.guid === pieceGuid) ?? null;
  }, [pieceGuid, designStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

// #endregion 🔖Piece

// #region 🔖Group

type YGroupVal = string | Y.Array<string> | YAttributes;
type YGroup = Y.Map<YGroupVal>;
type YGroups = Y.Array<YGroup>;

class GroupStore {
  private yGroup: YGroup;
  private cache?: Group;
  private cacheHash?: string;

  constructor(yGroup: YGroup, group: Group) {
    this.yGroup = yGroup;
    this.guid = group.guid;
    this.color = group.color;
    this.name = group.name;
    this.description = group.description;

    if (group.pieces) {
      const yPieces = new Y.Array<string>();
      yPieces.insert(
        0,
        group.pieces.map((p) => p.guid),
      );
      this.yGroup.set("pieces", yPieces);
    }
  }

  get guid(): string {
    return this.yGroup.get("guid") as string;
  }
  set guid(guid: string) {
    this.yGroup.set("guid", guid);
  }

  get color(): string | undefined {
    return this.yGroup.get("color") as string | undefined;
  }
  set color(color: string | undefined) {
    this.yGroup.set("color", color || "");
  }

  get name(): string | undefined {
    return this.yGroup.get("name") as string | undefined;
  }
  set name(name: string | undefined) {
    this.yGroup.set("name", name || "");
  }

  get description(): string | undefined {
    return this.yGroup.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yGroup.set("description", description || "");
  }

  get pieces(): string[] {
    const yPieces = this.yGroup.get("pieces") as Y.Array<string> | undefined;
    if (!yPieces) return [];
    return yPieces.toArray();
  }
  set pieces(pieces: string[]) {
    const yPieces = this.yGroup.get("pieces") as Y.Array<string> | undefined;
    if (yPieces) {
      yPieces.delete(0, yPieces.length);
      yPieces.insert(0, pieces);
    } else {
      const newYPieces = new Y.Array<string>();
      newYPieces.insert(0, pieces);
      this.yGroup.set("pieces", newYPieces);
    }
  }

  hash = (group: Group): string => {
    return JSON.stringify(group);
  };

  snapshot = (): Group => {
    const currentData = {
      guid: this.guid,
      pieces: this.pieces.map((guid) => ({ guid })),
      color: this.color,
      name: this.name,
      description: this.description,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: GroupDiff) => {
    if (diff.pieces !== undefined) this.pieces = diff.pieces.map((p) => p.guid);
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) this.description = diff.description;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yGroup, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yGroup, subscribe, true);
  };
}

// #endregion 🔖Group

// #region 🔖Side

class SideStore {
  public readonly parent: DesignStore;
  private ySide: YSide;
  private cache?: Side;
  private cacheHash?: string;

  constructor(parent: DesignStore, ySide: YSide, side: Side) {
    this.parent = parent;
    this.ySide = ySide;

    const pieceStore = this.parent.piece(side.piece.guid);
    if (pieceStore) {
      this.ySide.set("piece", pieceStore.guid);
    }

    if (side.designPiece) {
      const designPieceStore = this.parent.piece(side.designPiece.guid);
      if (designPieceStore) {
        this.ySide.set("designPiece", designPieceStore.guid);
      }
    }

    if (pieceStore && side.connector) {
      const typeGuid = pieceStore.type;
      if (typeGuid) {
        const typeStore = this.parent.parent.type(typeGuid);
        if (typeStore) {
          const connectorStore = typeStore.connectors.get(side.connector.guid);
          if (connectorStore) {
            this.ySide.set("connector", connectorStore.guid);
          }
        }
      }
    }
  }

  get piece(): Guid {
    const pieceUuid = this.ySide.get("piece") as string;
    if (!pieceUuid) {
      throw new Error(`[ORIGIN] SideStore.piece: pieceUuid is undefined`);
    }
    return this.parent.piece(pieceUuid).guid;
  }
  set piece(piece: Guid) {
    const pieceStore = this.parent.piece(piece);
    if (pieceStore) {
      this.ySide.set("piece", pieceStore.guid);
    }
  }

  get designPiece(): Guid | undefined {
    const designPieceUuid = this.ySide.get("designPiece") as string | undefined;
    if (!designPieceUuid) return undefined;
    return this.parent.piece(designPieceUuid).guid;
  }
  set designPiece(designPiece: Guid | undefined) {
    if (designPiece) {
      const designPieceStore = this.parent.piece(designPiece);
      if (designPieceStore) {
        this.ySide.set("designPiece", designPieceStore.guid);
      }
    } else {
      this.ySide.delete("designPiece");
    }
  }

  get connector(): Guid {
    const connectorUuid = this.ySide.get("connector") as string;
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeGuid = pieceStore.type;
    if (typeGuid) {
      const typeStore = this.parent.parent.type(typeGuid);
      if (typeStore) {
        const connectorStore = typeStore.connector(connectorUuid);
        return connectorStore.guid;
      }
    }
    return connectorUuid;
  }
  set connector(connector: Guid) {
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeGuid = pieceStore.type;
    if (typeGuid) {
      const typeStore = this.parent.parent.type(typeGuid);
      if (typeStore) {
        const connectorStore = typeStore.connectors.get(connector);
        if (connectorStore) {
          this.ySide.set("connector", connectorStore.guid);
        }
      }
    }
  }

  hash = (side: Side): string => {
    return JSON.stringify(side);
  };

  snapshot = (): Side => {
    const currentData = {
      piece: { guid: this.piece },
      designPiece: this.designPiece ? { guid: this.designPiece } : undefined,
      connector: this.connector ? { guid: this.connector } : undefined,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  id = (): string => {
    return this.piece;
  };

  change = (diff: SideDiff) => {
    if (diff.piece !== undefined) this.piece = diff.piece.guid;
    if (diff.designPiece !== undefined) this.designPiece = diff.designPiece?.guid;
    if (diff.connector !== undefined && diff.connector !== null) this.connector = diff.connector.guid;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe, true);
  };
}

// #endregion 🔖Side

// #region 🔖Connection

type YSideVal = string | number | YAttributes;
type YSide = Y.Map<YSideVal>;
type YSides = Y.Array<YSide>;

type YConnectionVal = string | number | YAttributes | YSide;
type YConnection = Y.Map<YConnectionVal>;
type YConnections = Y.Array<YConnection>;

class ConnectionStore {
  public readonly parent: DesignStore;
  private yConnection: YConnection;
  private connected: SideStore;
  private connecting: SideStore;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Connection;
  private cacheHash?: string;

  constructor(parent: DesignStore, yConnection: YConnection, connection: Connection) {
    this.parent = parent;
    this.yConnection = yConnection;
    this.guid = connection.guid;
    const yConnected = new Y.Map<YSideVal>();
    this.yConnection.set("connected", yConnected);
    this.connected = new SideStore(parent, yConnected, connection.connected);
    const yConnecting = new Y.Map<YSideVal>();
    this.yConnection.set("connecting", yConnecting);
    this.connecting = new SideStore(parent, yConnecting, connection.connecting);
    this.gap = connection.gap;
    this.shift = connection.shift;
    this.rise = connection.rise;
    this.rotation = connection.rotation;
    this.turn = connection.turn;
    this.tilt = connection.tilt;
    this.u = connection.u;
    this.v = connection.v;
    this.description = connection.description;
    this.attributes = new Map();
    const yConnectionAttributes = new Y.Array<YAttribute>();
    this.yConnection.set("attributes", yConnectionAttributes);
    this.yAttributes = yConnectionAttributes;
    if (connection.attributes) {
      for (const attribute of connection.attributes) {
        this.createAttribute(attribute);
      }
    }
  }

  get guid(): string {
    return this.yConnection.get("guid") as string;
  }
  set guid(guid: string) {
    this.yConnection.set("guid", guid);
  }

  get description(): string | undefined {
    return this.yConnection.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yConnection.set("description", description || "");
  }

  get gap(): number | undefined {
    return this.yConnection.get("gap") as number | undefined;
  }
  set gap(gap: number | undefined) {
    if (gap !== undefined) this.yConnection.set("gap", gap);
  }

  get shift(): number | undefined {
    return this.yConnection.get("shift") as number | undefined;
  }
  set shift(shift: number | undefined) {
    if (shift !== undefined) this.yConnection.set("shift", shift);
  }

  get rise(): number | undefined {
    return this.yConnection.get("rise") as number | undefined;
  }
  set rise(rise: number | undefined) {
    if (rise !== undefined) this.yConnection.set("rise", rise);
  }

  get rotation(): number | undefined {
    return this.yConnection.get("rotation") as number | undefined;
  }
  set rotation(rotation: number | undefined) {
    if (rotation !== undefined) this.yConnection.set("rotation", rotation);
  }

  get turn(): number | undefined {
    return this.yConnection.get("turn") as number | undefined;
  }
  set turn(turn: number | undefined) {
    if (turn !== undefined) this.yConnection.set("turn", turn);
  }

  get tilt(): number | undefined {
    return this.yConnection.get("tilt") as number | undefined;
  }
  set tilt(tilt: number | undefined) {
    if (tilt !== undefined) this.yConnection.set("tilt", tilt);
  }

  get u(): number | undefined {
    return this.yConnection.get("u") as number | undefined;
  }
  set u(u: number | undefined) {
    if (u !== undefined) this.yConnection.set("u", u);
  }

  get v(): number | undefined {
    return this.yConnection.get("v") as number | undefined;
  }
  set v(v: number | undefined) {
    if (v !== undefined) this.yConnection.set("v", v);
  }

  id(): Guid {
    return this.guid;
  }

  hash = (connection: Connection): string => {
    return JSON.stringify(connection);
  };

  snapshot = (): Connection => {
    const currentData = {
      guid: this.guid,
      connected: this.connected.snapshot(),
      connecting: this.connecting.snapshot(),
      gap: this.gap,
      shift: this.shift,
      rise: this.rise,
      rotation: this.rotation,
      turn: this.turn,
      tilt: this.tilt,
      u: this.u,
      v: this.v,
      description: this.description,
      attributes: Array.from(this.attributes.values()).map((attr) => attr.snapshot()),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  hasAttribute(guid: string): boolean {
    return this.attributes.has(guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  change = (diff: ConnectionDiff): void => {
    if (diff.connected !== undefined) this.connected.change(diff.connected);
    if (diff.connecting !== undefined) this.connecting.change(diff.connecting);
    if (diff.gap !== undefined) this.gap = diff.gap;
    if (diff.shift !== undefined) this.shift = diff.shift;
    if (diff.rise !== undefined) this.rise = diff.rise;
    if (diff.rotation !== undefined) this.rotation = diff.rotation;
    if (diff.turn !== undefined) this.turn = diff.turn;
    if (diff.tilt !== undefined) this.tilt = diff.tilt;
    if (diff.u !== undefined) this.u = diff.u;
    if (diff.v !== undefined) this.v = diff.v;
    if (diff.description !== undefined) this.description = diff.description;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe, true);
  };
}

type ConnectionScope = { guid: string };
const ConnectionScopeContext = createContext<ConnectionScope | null>(null);
export const ConnectionScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(ConnectionScopeContext.Provider, { value }, props.children as any);
};
export const useConnectionScope = () => useContext(ConnectionScopeContext);

function useConnectionStore<T>(selector?: (store: ConnectionStore) => T, guid?: string): T | ConnectionStore {
  const designStore = useDesignStore() as DesignStore;
  const connectionScope = useConnectionScope();
  const connectionGuid = connectionScope?.guid ?? guid;
  if (!connectionGuid) throw new Error("useConnectionStore must be called within a ConnectionScopeProvider or be directly provided with a guid");
  const connectionStore = designStore.connection(connectionGuid);
  if (!connectionStore) throw new Error(`Connection store not found for connection ${connectionGuid}`);
  return selector ? selector(connectionStore) : connectionStore;
}

export function useConnection<T>(selector?: (connection: Connection) => T, id?: Guid, deep: boolean = false): T | Connection | null {
  return useSync<Connection, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ? selector : (identitySelector as any));
}

// #endregion 🔖Connection

// #region 🔖Stat

type YStat = Y.Map<string | number | boolean>;
type YStats = Y.Array<YStat>;

class StatStore {
  private yStat: YStat;
  private cache?: Stat;
  private cacheHash?: string;

  constructor(yStat: YStat, stat: Stat) {
    this.yStat = yStat;
    this.guid = stat.guid;
    this.quality = stat.quality;
    this.unit = stat.unit;
    this.min = stat.min;
    this.minExcluded = stat.minExcluded;
    this.max = stat.max;
    this.maxExcluded = stat.maxExcluded;
  }

  get guid(): string {
    return this.yStat.get("guid") as string;
  }
  set guid(guid: string) {
    this.yStat.set("guid", guid);
  }

  get quality(): QualityId {
    return { guid: this.yStat.get("quality") as string };
  }
  set quality(quality: QualityId) {
    this.yStat.set("quality", quality.guid);
  }

  get unit(): string | undefined {
    return this.yStat.get("unit") as string | undefined;
  }
  set unit(unit: string | undefined) {
    if (unit !== undefined) {
      this.yStat.set("unit", unit);
    }
  }

  get min(): number | undefined {
    return this.yStat.get("min") as number | undefined;
  }
  set min(min: number | undefined) {
    if (min !== undefined) {
      this.yStat.set("min", min);
    }
  }

  get minExcluded(): boolean | undefined {
    return this.yStat.get("minExcluded") as boolean | undefined;
  }
  set minExcluded(minExcluded: boolean | undefined) {
    if (minExcluded !== undefined) {
      this.yStat.set("minExcluded", minExcluded);
    }
  }

  get max(): number | undefined {
    return this.yStat.get("max") as number | undefined;
  }
  set max(max: number | undefined) {
    if (max !== undefined) {
      this.yStat.set("max", max);
    }
  }

  get maxExcluded(): boolean | undefined {
    return this.yStat.get("maxExcluded") as boolean | undefined;
  }
  set maxExcluded(maxExcluded: boolean | undefined) {
    if (maxExcluded !== undefined) {
      this.yStat.set("maxExcluded", maxExcluded);
    }
  }

  id = (): string => {
    return this.guid;
  };

  hash = (stat: Stat): string => {
    return JSON.stringify(stat);
  };

  snapshot(): Stat {
    const currentData = {
      guid: this.guid,
      quality: this.quality,
      unit: this.unit,
      min: this.min,
      minExcluded: this.minExcluded,
      max: this.max,
      maxExcluded: this.maxExcluded,
    };
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  }

  change = (diff: StatDiff) => {
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.quality !== undefined) this.quality = diff.quality;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.min !== undefined) this.min = diff.min;
    if (diff.minExcluded !== undefined) this.minExcluded = diff.minExcluded;
    if (diff.max !== undefined) this.max = diff.max;
    if (diff.maxExcluded !== undefined) this.maxExcluded = diff.maxExcluded;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yStat, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yStat, subscribe, true);
  };
}

// #endregion 🔖Stat

// #region 🔖Design

type YDesignVal = string | boolean | number | YAuthorUuids | YAttributes | YPieces | YConnections | YLayers | YGroups | YStats | YProps | YLocation | Y.Array<string>;
type YDesign = Y.Map<YDesignVal>;
type YDesigns = Y.Array<YDesign>;

export class DesignStore {
  public readonly parent: KitStore;
  private yDesign: YDesign;
  private yPieces: YPieces;
  private pieces: Map<string, PieceStore>;
  private yConnections: YConnections;
  private connections: Map<string, ConnectionStore>;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private yStats: YStats;
  private stats: Map<string, StatStore>;
  private props: Map<string, PropStore>;
  private yProps: YProps;
  private layers: Map<string, LayerStore>;
  private yLayers: YLayers;
  private groups: Map<string, GroupStore>;
  private yGroups: YGroups;
  private location?: YLocationStore;
  private yAuthors: YAuthorUuids;
  private authors: Map<string, AuthorStore>;
  private yConcepts: Y.Array<string>;
  private cache?: Design;
  private cacheHash?: string;
  private dirty: boolean = true;
  private _piecesCache?: Piece[];
  private _piecesVersion = 0;
  private _connectionsCache?: Connection[];
  private _connectionsVersion = 0;
  public readonly derived: DerivedStore = new DerivedStore();

  constructor(parent: KitStore, yDesign: YDesign, design: Design) {
    this.parent = parent;
    this.yDesign = yDesign;
    this.guid = design.guid;
    this.pieces = new Map();
    this.connections = new Map();
    this.attributes = new Map();
    this.stats = new Map();
    this.props = new Map();
    this.layers = new Map();
    this.groups = new Map();
    this.location = undefined;
    this.authors = new Map();

    this.name = design.name;
    this.parentGuid = design.parent?.guid;
    this.abstract = design.isAbstract;
    this.canScale = design.canScale;
    this.canMirror = design.canMirror;
    this.unit = design.unit;
    this.icon = design.icon;
    this.image = design.image;
    this.description = design.description;

    const yDesignPieces = new Y.Array<YPiece>();
    this.yDesign.set("pieces", yDesignPieces);
    this.yPieces = yDesignPieces;
    if (design.pieces) {
      for (const piece of design.pieces) {
        if (!piece?.guid) continue;
        this.createPiece(piece);
      }
    }

    const yDesignConnections = new Y.Array<YConnection>();
    this.yDesign.set("connections", yDesignConnections);
    this.yConnections = yDesignConnections;
    if (design.connections) {
      for (const connection of design.connections) {
        if (!connection?.guid) continue;
        this.createConnection(connection);
      }
    }

    const yDesignAttributes = new Y.Array<YAttribute>();
    this.yDesign.set("attributes", yDesignAttributes);
    this.yAttributes = yDesignAttributes;
    if (design.attributes) {
      for (const attribute of design.attributes) {
        if (!attribute?.guid) continue;
        this.createAttribute(attribute);
      }
    }

    const yDesignStats = new Y.Array<YStat>();
    this.yDesign.set("stats", yDesignStats);
    this.yStats = yDesignStats;
    if (design.stats) {
      for (const stat of design.stats) {
        if (!stat?.guid) continue;
        this.createStat(stat);
      }
    }

    const yDesignProps = new Y.Array<YProp>();
    this.yDesign.set("props", yDesignProps);
    this.yProps = yDesignProps;
    if (design.props) {
      for (const prop of design.props) {
        if (!prop?.guid) continue;
        this.createProp(prop);
      }
    }

    const yDesignLayers = new Y.Array<YLayer>();
    this.yDesign.set("layers", yDesignLayers);
    this.yLayers = yDesignLayers;
    if (design.layers) {
      for (const layer of design.layers) {
        if (!layer?.guid) continue;
        this.createLayer(layer);
      }
    }

    if (design.activeLayer) {
      this.yDesign.set("activeLayer", design.activeLayer.guid);
    }

    const yDesignGroups = new Y.Array<YGroup>();
    this.yDesign.set("groups", yDesignGroups);
    this.yGroups = yDesignGroups;
    if (design.groups) {
      for (const group of design.groups) {
        if (!group?.guid) continue;
        this.createGroup(group);
      }
    }

    if (design.location && "longitude" in design.location) {
      const yLocation = new Y.Map() as YLocation;
      this.yDesign.set("location", yLocation);
      this.location = new YLocationStore(yLocation, design.location as Location);
    }

    const yDesignConcepts = new Y.Array<string>();
    this.yDesign.set("concepts", yDesignConcepts);
    this.yConcepts = yDesignConcepts;
    if (design.concepts) {
      design.concepts.forEach((concept) => this.yConcepts.push([concept.guid]));
    }

    this.authors = new Map();
    if (design.authors) {
      design.authors.forEach((authorId) => {
        if (!authorId?.guid) return;
        const authorStore = this.parent.author(authorId.guid);
        if (!authorStore) return;
        this.authors.set(authorId.guid, authorStore);
      });
    }
    const yDesignAuthors = new Y.Array<YAuthorUuid>();
    this.yDesign.set("authors", yDesignAuthors);
    this.yAuthors = yDesignAuthors;
    this.authors.forEach((author) => author?.guid && this.yAuthors.push([author.guid]));

    this.yDesign.set("createdAt", new Date().toISOString());
    this.updated();
  }

  get guid(): string {
    return this.yDesign.get("guid") as string;
  }
  set guid(guid: string) {
    this.yDesign.set("guid", guid);
  }

  get name(): string {
    return this.yDesign.get("name") as string;
  }
  set name(name: string) {
    this.yDesign.set("name", name);
  }
  get parentGuid(): string | undefined {
    return this.yDesign.get("parent") as string | undefined;
  }
  set parentGuid(parent: string | undefined) {
    if (parent) this.yDesign.set("parent", parent);
    else this.yDesign.delete("parent");
  }
  get folder(): string | undefined {
    return this.yDesign.get("folder") as string | undefined;
  }
  set folder(folder: string | undefined) {
    if (folder) this.yDesign.set("folder", folder);
    else this.yDesign.delete("folder");
  }
  get abstract(): boolean | undefined {
    return this.yDesign.get("isAbstract") as boolean | undefined;
  }
  set abstract(isAbstract: boolean | undefined) {
    if (isAbstract) this.yDesign.set("isAbstract", isAbstract);
    else this.yDesign.delete("isAbstract");
  }
  get canScale(): boolean | undefined {
    return this.yDesign.get("canScale") as boolean | undefined;
  }
  set canScale(canScale: boolean | undefined) {
    if (canScale !== undefined) {
      this.yDesign.set("canScale", canScale);
    }
  }
  get canMirror(): boolean | undefined {
    return this.yDesign.get("canMirror") as boolean | undefined;
  }
  set canMirror(canMirror: boolean | undefined) {
    if (canMirror !== undefined) {
      this.yDesign.set("canMirror", canMirror);
    }
  }
  get unit(): string | undefined {
    return this.yDesign.get("unit") as string | undefined;
  }
  set unit(unit: string | undefined) {
    this.yDesign.set("unit", unit || "");
  }
  get icon(): string | undefined {
    return this.yDesign.get("icon") as string | undefined;
  }
  set icon(icon: string | undefined) {
    this.yDesign.set("icon", icon || "");
  }
  get image(): string | undefined {
    return this.yDesign.get("image") as string | undefined;
  }
  set image(image: string | undefined) {
    this.yDesign.set("image", image || "");
  }
  get description(): string | undefined {
    return this.yDesign.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yDesign.set("description", description || "");
  }
  get createdAt(): Date {
    return new Date(this.yDesign.get("createdAt") as string);
  }
  get updatedAt(): Date {
    return new Date(this.yDesign.get("updatedAt") as string);
  }

  updated(): void {
    this.yDesign.set("updatedAt", new Date().toISOString());
  }

  hasPiece(guid: string): boolean {
    return this.pieces.has(guid);
  }

  createPiece(piece: Piece): void {
    const yPiece = new Y.Map<YPieceVal>();
    this.yPieces!.push([yPiece]);
    const yPieceStore = new PieceStore(this, yPiece, piece);
    this.pieces.set(piece.guid, yPieceStore);
  }

  createConnection(connection: Connection): void {
    const yConnection = new Y.Map<YConnectionVal>();
    this.yConnections.push([yConnection]);
    const yConnectionStore = new ConnectionStore(this, yConnection, connection);
    this.connections.set(connection.guid, yConnectionStore);
  }

  createAttribute(attribute: Attribute): void {
    const yAttribute = new Y.Map<YAttributeVal>();

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  createStat(stat: Stat): void {
    const yStat = new Y.Map() as YStat;
    this.yStats.push([yStat]);
    const yStatStore = new StatStore(yStat, stat);
    this.stats.set(stat.guid, yStatStore);
  }

  createProp(prop: Prop): void {
    const yProp = new Y.Map() as YProp;
    this.yProps.push([yProp]);
    const yPropStore = new PropStore(yProp, prop);
    this.props.set(prop.guid, yPropStore);
  }

  createLayer(layer: Layer): void {
    const yLayer = new Y.Map() as YLayer;
    this.yLayers.push([yLayer]);
    const yLayerStore = new LayerStore(yLayer, layer);
    this.layers.set(layer.path, yLayerStore);
  }

  createGroup(group: Group): void {
    const yGroup = new Y.Map() as YGroup;
    this.yGroups.push([yGroup]);
    const yGroupStore = new GroupStore(yGroup, group);
    const groupKey = group.pieces.join(",");
    this.groups.set(groupKey, yGroupStore);
  }

  piece(guid: string): PieceStore {
    const p = this.pieces.get(guid);
    if (!p) throw new Error(`Piece store not found for guid ${guid}`);
    return p;
  }

  hasConnection(guid: string): boolean {
    return this.connections.has(guid);
  }

  connection(guid: string): ConnectionStore {
    const c = this.connections.get(guid);
    if (!c) throw new Error(`Connection store not found for guid ${guid}`);
    return c;
  }

  hasAttribute(guid: string): boolean {
    return this.attributes.has(guid);
  }

  attribute(guid: string): AttributeStore {
    const a = this.attributes.get(guid);
    if (!a) throw new Error(`Attribute store not found for guid ${guid}`);
    return a;
  }

  id(): Guid {
    return this.guid;
  }

  hash(design: Design): string {
    return JSON.stringify(design);
  }

  snapshot = (): Design => {
    if (!this.dirty && this.cache) {
      return this.cache;
    }

    const currentData = {
      guid: this.guid,
      name: this.name,
      parent: this.parentGuid ? { guid: this.parentGuid } : undefined,
      folder: this.folder,
      isAbstract: this.abstract,
      canScale: this.canScale,
      canMirror: this.canMirror,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
      pieces: Array.from(this.pieces.values()).map((piece) => piece.snapshot()),
      connections: Array.from(this.connections.values()).map((connection) => connection.snapshot()),
      stats: Array.from(this.stats.values()).map((stat) => stat.snapshot()),
      props: Array.from(this.props.values()).map((prop) => prop.snapshot()),
      layers: Array.from(this.layers.values()).map((layer) => layer.snapshot()),
      activeLayer: this.yDesign.get("activeLayer") ? { guid: this.yDesign.get("activeLayer") as string } : undefined,
      groups: Array.from(this.groups.values()).map((group) => group.snapshot()),
      location: this.location?.snapshot(),
      authors: Array.from(this.authors.values()).map((author) => ({ guid: author.guid })),
      concepts: (this.yDesign.get("concepts") as Y.Array<string> | undefined)?.toArray()?.map((g) => ({ guid: g })),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
      createdAt: this.createdAt?.toISOString(),
      updatedAt: this.updatedAt?.toISOString(),
    };

    this.cache = currentData;
    this.dirty = false;
    return this.cache;
  };

  markDirty = () => {
    this.dirty = true;
  };

  change = (diff: DesignDiff) => {
    this.dirty = true;
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.parent !== undefined) this.parentGuid = diff.parent?.guid;
    if (diff.folder !== undefined) this.folder = diff.folder;
    if (diff.isAbstract !== undefined) this.abstract = diff.isAbstract;
    if (diff.canScale !== undefined) this.canScale = diff.canScale;
    if (diff.canMirror !== undefined) this.canMirror = diff.canMirror;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.icon !== undefined) this.icon = diff.icon;
    if (diff.image !== undefined) this.image = diff.image;
    if (diff.description !== undefined) this.description = diff.description;

    if (diff.pieces !== undefined) {
      if (typeof diff.pieces === "object" && !Array.isArray(diff.pieces)) {
        if (diff.pieces.added) {
          diff.pieces.added.forEach((piece) => this.createPiece(piece));
        }
        if (diff.pieces.updated) {
          diff.pieces.updated.forEach(({ piece, diff: pieceDiff }) => {
            const pieceStore = this.pieces.get(piece.guid);
            if (pieceStore) {
              pieceStore.change(pieceDiff);
            }
          });
        }
        if (diff.pieces.removed) {
          diff.pieces.removed.forEach((pieceId) => {
            const guid = pieceId.guid;
            if (this.pieces.has(guid)) {
              const pieceArray = Array.from(this.pieces.values());
              const pieceIndex = pieceArray.findIndex((p) => p.guid === guid);
              if (pieceIndex !== -1) {
                this.pieces.delete(guid);
                this.yPieces!.delete(pieceIndex, 1);
              }
            }
          });
        }
      } else {
        this.pieces.clear();
        this.yPieces!.delete(0, this.yPieces!.length);

        if (diff.pieces) {
          for (const piece of diff.pieces as Piece[]) {
            this.createPiece(piece);
          }
        }
      }
    }

    if (diff.connections !== undefined) {
      if (typeof diff.connections === "object" && !Array.isArray(diff.connections)) {
        if (diff.connections.added) {
          diff.connections.added.forEach((connection) => this.createConnection(connection));
        }
        if (diff.connections.updated) {
          diff.connections.updated.forEach(({ connection, diff: connectionDiff }) => {
            const connectionStore = this.connections.get(connection.guid);
            if (connectionStore) {
              connectionStore.change(connectionDiff);
            }
          });
        }
        if (diff.connections.removed) {
          diff.connections.removed.forEach((connectionId) => {
            const guid = connectionId.guid;
            const connectionStore = this.connections.get(guid);
            if (connectionStore) {
              const connectionArray = Array.from(this.connections.values());
              const connectionIndex = connectionArray.findIndex((c) => c.guid === guid);
              if (connectionIndex !== -1) {
                this.connections.delete(guid);
                this.yConnections.delete(connectionIndex, 1);
              }
            }
          });
        }
      } else {
        this.connections.clear();
        this.yConnections.delete(0, this.yConnections.length);

        if (diff.connections) {
          for (const connection of diff.connections as Connection[]) {
            this.createConnection(connection);
          }
        }
      }
    }

    if (diff.stats !== undefined) {
      if (diff.stats.removed) {
        diff.stats.removed.forEach((statId) => {
          const guid = statId.guid;
          this.stats.delete(guid);
          const yStats = this.yDesign.get("stats") as Y.Array<YStat>;
          if (yStats) {
            const index = yStats.toArray().findIndex((yStat) => (yStat as Y.Map<unknown>).get("guid") === guid);
            if (index >= 0) yStats.delete(index, 1);
          }
        });
      }
      if (diff.stats.updated) {
        diff.stats.updated.forEach(({ stat, diff: statDiff }) => {
          const statStore = this.stats.get(stat.guid);
          if (statStore) statStore.change(statDiff);
        });
      }
      if (diff.stats.added) {
        diff.stats.added.forEach((stat) => {
          this.createStat(stat);
        });
      }
    }

    if (diff.props !== undefined) {
      if (diff.props.removed) {
        diff.props.removed.forEach((propId) => {
          const guid = propId.guid;
          this.props.delete(guid);
          const yProps = this.yDesign.get("props") as Y.Array<YProp>;
          if (yProps) {
            const index = yProps.toArray().findIndex((yProp) => (yProp as Y.Map<unknown>).get("guid") === guid);
            if (index >= 0) yProps.delete(index, 1);
          }
        });
      }
      if (diff.props.updated) {
        diff.props.updated.forEach(({ prop, diff: propDiff }) => {
          const propStore = this.props.get(prop.guid);
          if (propStore) propStore.change(propDiff);
        });
      }
      if (diff.props.added) {
        diff.props.added.forEach((prop) => {
          this.createProp(prop);
        });
      }
    }

    if (diff.layers !== undefined) {
      if (diff.layers.removed) {
        diff.layers.removed.forEach((layerId) => {
          const guid = layerId.guid;
          this.layers.delete(guid);
          const yLayers = this.yDesign.get("layers") as Y.Array<YLayer>;
          if (yLayers) {
            const index = yLayers.toArray().findIndex((yLayer) => (yLayer as Y.Map<unknown>).get("guid") === guid);
            if (index >= 0) yLayers.delete(index, 1);
          }
        });
      }
      if (diff.layers.updated) {
        diff.layers.updated.forEach(({ layer, diff: layerDiff }) => {
          const layerStore = this.layers.get(layer.guid);
          if (layerStore) layerStore.change(layerDiff);
        });
      }
      if (diff.layers.added) {
        diff.layers.added.forEach((layer) => {
          this.createLayer(layer);
        });
      }
    }

    if (diff.activeLayer !== undefined) {
      if (diff.activeLayer) {
        this.yDesign.set("activeLayer", diff.activeLayer.guid);
      } else {
        this.yDesign.delete("activeLayer");
      }
    }

    if (diff.groups !== undefined) {
      if (diff.groups.removed) {
        diff.groups.removed.forEach((groupId) => {
          const guid = groupId.guid;
          this.groups.delete(guid);
          const yGroups = this.yDesign.get("groups") as Y.Array<YGroup>;
          if (yGroups) {
            const index = yGroups.toArray().findIndex((yGroup) => (yGroup as Y.Map<unknown>).get("guid") === guid);
            if (index >= 0) yGroups.delete(index, 1);
          }
        });
      }
      if (diff.groups.updated) {
        diff.groups.updated.forEach(({ group, diff: groupDiff }) => {
          const groupStore = this.groups.get(group.guid);
          if (groupStore) groupStore.change(groupDiff);
        });
      }
      if (diff.groups.added) {
        diff.groups.added.forEach((group) => {
          this.createGroup(group);
        });
      }
    }

    if ("location" in diff) {
      if (diff.location) {
        if (!this.location) {
          const yLocation = new Y.Map() as YLocation;
          this.yDesign.set("location", yLocation);
          this.location = new YLocationStore(yLocation, diff.location as Location);
        } else {
          this.location.change(diff.location as LocationDiff);
        }
      } else {
        this.yDesign.delete("location");
        this.location = undefined;
      }
    }

    if (diff.authors !== undefined) {
      if (diff.authors.removed) {
        diff.authors.removed.forEach((authorId) => {
          this.authors.delete(authorId.guid);
        });
      }
      if (diff.authors.updated) {
        diff.authors.updated.forEach(({ author, diff: authorDiff }) => {
          const authorStore = this.authors.get(author.guid);
          if (authorStore) authorStore.change(authorDiff);
        });
      }
      if (diff.authors.added) {
        diff.authors.added.forEach((author) => {
          const authorStore = this.parent.author(author.guid);
          this.authors.set(author.guid, authorStore);
        });
      }
    }

    if (diff.concepts !== undefined) {
      if (diff.concepts) {
        const yConcepts = new Y.Array<string>();
        diff.concepts.forEach((concept) => yConcepts.push([concept.guid]));
        this.yDesign.set("concepts", yConcepts);
      } else {
        this.yDesign.delete("concepts");
      }
    }

    if ("attributes" in diff) {
      if (diff.attributes && typeof diff.attributes === "object" && ("added" in diff.attributes || "removed" in diff.attributes || "updated" in diff.attributes)) {
        if (diff.attributes.removed) {
          diff.attributes.removed.forEach((attrId) => {
            const guid = attrId.guid;
            const attr = this.attributes.get(guid);
            if (attr) {
              const yAttrIndex = Array.from(this.yAttributes).findIndex((yAttr: any) => {
                const yMap = yAttr[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (yAttrIndex !== -1) {
                this.yAttributes.delete(yAttrIndex, 1);
              }
              this.attributes.delete(guid);
            }
          });
        }
        if (diff.attributes.updated) {
          diff.attributes.updated.forEach(({ attribute, diff: attrDiff }) => {
            const attr = this.attributes.get(attribute.guid);
            if (attr) {
              attr.change(attrDiff);
            }
          });
        }
        if (diff.attributes.added) {
          diff.attributes.added.forEach((attribute) => this.createAttribute(attribute));
        }
      } else {
        this.attributes.clear();
        this.yAttributes.delete(0, this.yAttributes.length);

        if (diff.attributes && Array.isArray(diff.attributes)) {
          for (const attribute of diff.attributes) {
            this.createAttribute(attribute);
          }
        }
      }
    }

    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yDesign, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yDesign, subscribe, true);
  };

  snapshotPieces = (): Piece[] => {
    const currentVersion = (this.yPieces as any)._clock || this.pieces.size;
    if (this._piecesCache && this._piecesVersion === currentVersion) {
      return this._piecesCache;
    }
    this._piecesCache = Array.from(this.pieces.values()).map((piece) => piece.snapshot());
    this._piecesVersion = currentVersion;
    return this._piecesCache;
  };

  snapshotConnections = (): Connection[] => {
    const currentVersion = (this.yConnections as any)._clock || this.connections.size;
    if (this._connectionsCache && this._connectionsVersion === currentVersion) {
      return this._connectionsCache;
    }
    this._connectionsCache = Array.from(this.connections.values()).map((connection) => connection.snapshot());
    this._connectionsVersion = currentVersion;
    return this._connectionsCache;
  };

  onPiecesChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._piecesCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yPieces.observeDeep(notifySubscriber);
      return () => this.yPieces.unobserveDeep(notifySubscriber);
    }
    this.yPieces.observe(notifySubscriber);
    return () => this.yPieces.unobserve(notifySubscriber);
  };

  onConnectionsChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._connectionsCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yConnections.observeDeep(notifySubscriber);
      return () => this.yConnections.unobserveDeep(notifySubscriber);
    }
    this.yConnections.observe(notifySubscriber);
    return () => this.yConnections.unobserve(notifySubscriber);
  };

  onScalarFieldChanged = (key: string, subscribe: Subscribe): Disposable => {
    return createFieldObserver(this.yDesign, key, subscribe, false);
  };

  // #region 🔖YPath API

  private pathSubscribers: Map<string, Set<() => void>> = new Map();
  private pathObservers: Map<string, Disposable> = new Map();

  onPathChanged = (path: YPath, subscribe: Subscribe): Unsubscribe => {
    const pathKey = JSON.stringify(path);
    const subscriberCallback = () => {
      subscribe(() => {});
    };
    if (!this.pathSubscribers.has(pathKey)) {
      this.pathSubscribers.set(pathKey, new Set());
      const pathObserver = createPathObserver(this.yDesign, path, () => {
        const subscribers = this.pathSubscribers.get(pathKey);
        if (subscribers) subscribers.forEach((cb) => cb());
        return () => {};
      });
      this.pathObservers.set(pathKey, pathObserver);
    }
    const subscribers = this.pathSubscribers.get(pathKey)!;
    subscribers.add(subscriberCallback);
    return () => {
      subscribers.delete(subscriberCallback);
      if (subscribers.size === 0) {
        const observer = this.pathObservers.get(pathKey);
        if (observer) {
          observer();
          this.pathObservers.delete(pathKey);
        }
        this.pathSubscribers.delete(pathKey);
      }
    };
  };

  getPathSnapshot = (path: YPath): any => {
    return getValueAtPath(this.yDesign, path);
  };

  // #endregion 🔖YPath API
}

type DesignScope = { guid: string };
const DesignScopeContext = createContext<DesignScope | null>(null);
export const DesignScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(DesignScopeContext.Provider, { value }, props.children as any);
};
export const useDesignScope = () => useContext(DesignScopeContext);
export const useIsInDesignScope = () => useDesignScope() !== null;

function useDesignStore<T>(selector?: (store: DesignStore) => T, guid?: string): T | DesignStore | null {
  const kitStore = useKitStore() as KitStore | null;
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? guid;
  if (!kitStore || !designGuid || !kitStore.hasDesign(designGuid)) return null;
  const designStore = kitStore.design(designGuid);
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: Guid, deep: boolean = false): T | DesignShallow | Design | null {
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? id;
  const store = useDesignStore(identitySelector, designGuid ?? undefined) as DesignStore | null;
  const syncedDeep = useSyncDeep<Design, T>(store, selector ? selector : (identitySelector as any));
  const synced = useSyncOptional<DesignShallow, T>(store as any, selector ? selector : (identitySelector as any));
  if (!designGuid || !store) return null;
  return deep ? syncedDeep : synced;
}

const EMPTY_PIECES: Piece[] = [];
const EMPTY_CONNECTIONS: Connection[] = [];

export function usePieces(): Piece[] {
  const designStore = useDesignStore(identitySelector) as DesignStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!designStore) return () => {};
      return designStore.onPiecesChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [designStore],
  );

  const getSnapshot = useCallback(() => {
    if (!designStore) return EMPTY_PIECES;
    return designStore.snapshotPieces();
  }, [designStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useConnections(): Connection[] {
  const designStore = useDesignStore(identitySelector) as DesignStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!designStore) return () => {};
      return designStore.onConnectionsChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [designStore],
  );

  const getSnapshot = useCallback(() => {
    if (!designStore) return EMPTY_CONNECTIONS;
    return designStore.snapshotConnections();
  }, [designStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useIncludedDesigns() {
  const designScope = useDesignScope();
  const pieces = usePieces();
  const connections = useConnections();
  return useMemo(() => {
    if (!designScope) return [];
    const design = { guid: designScope.guid, pieces, connections } as Design;
    return getIncludedDesigns(design);
  }, [designScope?.guid, pieces, connections]);
}

export function useDesignId() {
  const designStore = useDesignStore(identitySelector) as DesignStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!designStore) return () => {};
      const unsubName = designStore.onScalarFieldChanged("name", () => {
        callback();
        return () => {};
      });
      const unsubParent = designStore.onScalarFieldChanged("parent", () => {
        callback();
        return () => {};
      });
      return () => {
        unsubName();
        unsubParent();
      };
    },
    [designStore],
  );

  const getSnapshot = useCallback(() => {
    if (!designStore) return { name: "", parent: undefined };
    return { name: designStore.name, parent: designStore.parentGuid ? { guid: designStore.parentGuid } : undefined };
  }, [designStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function usePiecesFromIds(pieceIds: Guid[]) {
  const pieces = usePieces();
  const includedDesigns = useIncludedDesigns();
  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.guid, d])), [includedDesigns]);
  const piecesMap = useMemo(() => new Map(pieces.map((p) => [p.guid, p])), [pieces]);

  return useMemo(() => {
    return pieceIds.map((id) => {
      const pieceIdString = typeof id === "string" ? id : (id as any).guid;
      const foundPiece = piecesMap.get(pieceIdString);
      if (foundPiece) {
        return {
          ...foundPiece,
          id_: foundPiece.guid,
        };
      }
      const includedDesign = includedDesignMap.get(pieceIdString);
      if (includedDesign) {
        return {
          id_: pieceIdString,
          type: {
            name: "design",
            variant: includedDesign.designGuid,
          },
          center: includedDesign.center,
          plane: includedDesign.plane,
          description: `${includedDesign.type === "fixed" ? "Fixed" : "Clustered"} design`,
        };
      }
      return {
        id_: pieceIdString,
        type: {
          name: "unknown",
          variant: "",
        },
        description: `Unknown piece: ${pieceIdString}`,
      };
    });
  }, [pieceIds, piecesMap, includedDesignMap]);
}

export function useReplacableTypes(pieceIds: Guid[], selectedVariants?: string[]) {
  const kitTypes = useKitTypes();
  const designScope = useDesignScope();
  const pieces = usePieces();
  const connections = useConnections();

  return useMemo(() => {
    if (!designScope) return [];
    const kit = { types: kitTypes } as Kit;
    const design = { guid: designScope.guid, pieces, connections } as Design;
    if (pieceIds.length === 1) {
      return findReplacableTypesForPieceInDesign(kit, design.guid, pieceIds[0], selectedVariants);
    } else {
      return findReplacableTypesForPiecesInDesign(kit, design.guid, pieceIds, selectedVariants);
    }
  }, [kitTypes, pieces, connections, designScope?.guid, pieceIds, selectedVariants]);
}

export function useReplacableDesigns(piece: Piece) {
  const kitDesigns = useKitDesigns();
  const designScope = useDesignScope();
  const pieces = usePieces();

  return useMemo(() => {
    if (!designScope) return [];
    const kit = { designs: kitDesigns } as Kit;
    return findReplacableDesignsForDesignPiece(kit, designScope.guid, piece);
  }, [kitDesigns, designScope?.guid, pieces, piece]);
}

export function useExplodeableDesignNodes(nodes: any[], selection: any) {
  const kitDesigns = useKitDesigns();
  return useMemo(() => {
    return nodes.filter((node) => {
      if (node.type !== "design") return false;
      const Guid = node.data.piece.id_;
      if (!selection.pieces?.includes(Guid)) return false;
      const designName = (node.data.piece as any).type?.variant;
      if (!designName) return false;
      if (!kitDesigns?.find((d: any) => d.name === designName)) return false;
      return true;
    });
  }, [nodes, selection.pieces, kitDesigns]);
}

// #endregion 🔖Design

// #region 🔖Kit

type YConceptVal = string | YAttributes;
type YConcept = Y.Map<YConceptVal>;
type YConcepts = Y.Array<YConcept>;

class ConceptStore {
  private yConcept: YConcept;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Concept;
  private cacheHash?: string;

  constructor(yConcept: YConcept, concept: Concept) {
    this.yConcept = yConcept;
    this.yAttributes = new Y.Array<YAttribute>();
    this.yConcept.set("attributes", this.yAttributes);
    this.attributes = new Map();
    this.guid = concept.guid;
    this.name = concept.name;
    this.description = concept.description;
    this.icon = concept.icon;
    concept.attributes?.forEach((attribute) => this.createAttribute(attribute));
  }

  get guid(): string {
    return this.yConcept.get("guid") as string;
  }
  set guid(guid: string) {
    this.yConcept.set("guid", guid);
  }

  get name(): string {
    return this.yConcept.get("name") as string;
  }
  set name(name: string) {
    this.yConcept.set("name", name);
  }

  get description(): string | undefined {
    return this.yConcept.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    if (description !== undefined) this.yConcept.set("description", description);
    else this.yConcept.delete("description");
  }

  get icon(): string | undefined {
    return this.yConcept.get("icon") as string | undefined;
  }
  set icon(icon: string | undefined) {
    if (icon !== undefined) this.yConcept.set("icon", icon);
    else this.yConcept.delete("icon");
  }

  hasAttribute(guid: string): boolean {
    return this.attributes.has(guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();
    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const attributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, attributeStore);
  }

  attribute(guid: string): AttributeStore {
    return this.attributes.get(guid)!;
  }

  private findAttributeStore = (guid: string): AttributeStore | undefined => this.attributes.get(guid);

  private findAttributeIndexByGuid = (guid: string): number => {
    return Array.from(this.yAttributes).findIndex((yAttribute: any) => {
      const yMap = yAttribute[0] as Y.Map<any>;
      return yMap.get("guid") === guid;
    });
  };

  hash = (concept: Concept): string => {
    return JSON.stringify(concept);
  };

  snapshot = (): Concept => {
    const attributes = Array.from(this.attributes.values()).map((attribute) => attribute.snapshot());
    const currentData: Concept = {
      guid: this.guid,
      name: this.name,
    };
    const description = this.description;
    if (description !== undefined) currentData.description = description;
    const icon = this.icon;
    if (icon !== undefined) currentData.icon = icon;
    if (attributes.length > 0) currentData.attributes = attributes;
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  change = (diff: ConceptDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.description !== undefined) {
      const value = diff.description ?? undefined;
      if (value !== undefined) this.description = value;
      else this.yConcept.delete("description");
    }
    if (diff.icon !== undefined) {
      const value = diff.icon ?? undefined;
      if (value !== undefined) this.icon = value;
      else this.yConcept.delete("icon");
    }
    if (diff.attributes) {
      if (diff.attributes.removed) {
        diff.attributes.removed.forEach((identifier) => {
          const attribute = this.findAttributeStore(identifier.guid);
          if (!attribute) return;
          const index = this.findAttributeIndexByGuid(attribute.guid);
          if (index !== -1) {
            this.yAttributes.delete(index, 1);
          }
          this.attributes.delete(attribute.guid);
        });
      }
      if (diff.attributes.added) {
        diff.attributes.added.forEach((attribute) => this.createAttribute(attribute));
      }
      if (diff.attributes.updated) {
        diff.attributes.updated.forEach(({ attribute, diff: attributeDiff }) => {
          const attr = this.findAttributeStore(attribute.guid);
          if (!attr) return;
          attr.change(attributeDiff);
        });
      }
    }
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yConcept, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yConcept, subscribe, true);
  };
}

type YIdMap = Y.Map<string>;
type YKitVal = string | Y.Array<string> | YIdMap | YAttributes | YAuthors | YFiles | YFolders | YBenchmarks | YQualities | YProps | YTypes | YDesigns | YConcepts;
type YKit = Y.Map<YKitVal>;
type YKits = Y.Array<YKit>;

export class KitStore {
  public readonly parent: SketchpadStore;
  private readonly remoteProviders: RemoteProviders | undefined;
  private fileProvider?: FileProvider;
  public readonly yDoc: Y.Doc;
  private readonly yKit: YKit;
  private readonly yConcepts: YConcepts;
  private readonly yTypes: YTypes;
  private readonly types: Map<string, TypeStore>;
  private readonly conceptStores: Map<string, ConceptStore>;
  private readonly yDesigns: YDesigns;
  private readonly designs: Map<string, DesignStore>;
  private readonly yFiles: YFiles;
  private readonly files: Map<string, FileStore>;
  private readonly yFolders: YFolders;
  private readonly folders: Map<string, FolderStore>;
  private readonly yQualities: YQualities;
  private readonly qualities: Map<string, QualityStore>;
  private readonly yBenchmarks: YBenchmarks;
  private readonly benchmarks: Map<string, BenchmarkStore>;
  private readonly yAuthors: YAuthors;
  private readonly authors: Map<string, AuthorStore>;
  private readonly yAttributes: YAttributes;
  private readonly attributes: Map<string, AttributeStore>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult>;
  private readonly regularFiles: Map<Guid, string>;
  private cache?: Kit;
  private cacheHash?: string;
  private dirty: boolean = true;
  private _filesCache?: SemioFile[];
  private _filesVersion = 0;
  private _typesCache?: Type[];
  private _typesVersion = 0;
  private _designsCache?: Design[];
  private _designsVersion = 0;
  private _qualitiesCache?: Quality[];
  private _qualitiesVersion = 0;
  private _authorsCache?: Author[];
  private _authorsVersion = 0;
  private _foldersCache?: Folder[];
  private _foldersVersion = 0;
  private _conceptsCache?: Concept[];
  private _conceptsVersion = 0;

  constructor(parent: SketchpadStore, kit: Kit, local?: boolean, remote?: boolean, remoteProviders?: RemoteProviders) {
    this.parent = parent;
    this.remoteProviders = remote ? remoteProviders : undefined;
    this.yDoc = new Y.Doc();

    this.commandRegistry = new Map();
    this.regularFiles = new Map();
    this.types = new Map();
    this.conceptStores = new Map();
    this.designs = new Map();
    this.files = new Map();
    this.folders = new Map();
    this.qualities = new Map();
    this.benchmarks = new Map();
    this.authors = new Map();
    this.attributes = new Map();

    this.yKit = this.yDoc.getMap() as YKit;
    this.yConcepts = this.yDoc.getArray("concepts");
    this.yTypes = this.yDoc.getArray("types");
    this.yDesigns = this.yDoc.getArray("designs");
    this.yFiles = this.yDoc.getArray("files");
    this.yFolders = this.yDoc.getArray("folders");
    this.yQualities = this.yDoc.getArray("qualities");
    this.yBenchmarks = this.yDoc.getArray("benchmarks");
    this.yAuthors = this.yDoc.getArray("authors");
    this.yAttributes = this.yDoc.getArray("attributes");

    this.yDoc.transact(() => {
      this.guid = kit.guid;
      this.name = kit.name;
      this.version = kit.version;
      this.remote = kit.remote;
      this.homepage = kit.homepage;
      this.license = kit.license;
      this.preview = kit.preview;
      this.concepts = kit.concepts;
      this.icon = kit.icon;
      this.image = kit.image;
      this.description = kit.description;

      kit.attributes?.forEach((attribute) => attribute?.guid && this.createAttribute(attribute));
      kit.authors?.forEach((author) => author?.guid && this.createAuthor(author));
      kit.folders?.forEach((folder) => folder?.guid && this.createFolder(folder));
      kit.qualities?.forEach((quality) => quality?.guid && this.createQuality(quality));
      kit.types?.forEach((type) => type?.guid && this.createType(type));
      kit.designs?.forEach((design) => design?.guid && this.createDesign(design));
      kit.files?.forEach((file) => file?.guid && this.createFile(file));

      this.yKit.set("createdAt", new Date().toISOString());
      this.updated();
    });

    if (local) {
      this.persistence = new IndexeddbPersistence(`semio-kit-${kit.guid}`, this.yDoc);
    }

    if (remote && this.remoteProviders) {
      this.remoteProviders.yProvider(this.yDoc, this.name + "@" + this.version);
      this.initializeFileProvider();
    }

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command as any);
    });
  }

  private async initializeFileProvider() {
    if (!this.remoteProviders) return;
    try {
      this.fileProvider = await this.remoteProviders.fileProvider(this.guid);
      await this.syncFiles();
    } catch (error) {
      console.error(`[KIT ${this.name}] Failed to initialize file provider:`, error);
    }
  }

  private async syncFiles() {
    if (!this.fileProvider) return;
    for (const [guid, fileStore] of this.files) {
      try {
        const file = fileStore.snapshot();
        const storagePath = this.getFileStoragePath(file);
        const blob = await this.fileProvider.download(this.guid, guid, storagePath);
        const objectUrl = URL.createObjectURL(blob);
        this.regularFiles.set(storagePath, objectUrl);
      } catch (error) {
        console.error(`[KIT ${this.name}] Failed to sync file ${guid}:`, error);
      }
    }
  }

  get guid(): string {
    return this.yKit.get("guid") as string;
  }
  set guid(guid: string) {
    this.yKit.set("guid", guid);
  }

  get name(): string {
    return this.yKit.get("name") as string;
  }
  set name(name: string) {
    this.yKit.set("name", name);
  }
  get version(): string | undefined {
    return this.yKit.get("version") as string | undefined;
  }
  set version(version: string | undefined) {
    this.yKit.set("version", version || "");
  }
  get remote(): string | undefined {
    return this.yKit.get("remote") as string | undefined;
  }
  set remote(remote: string | undefined) {
    this.yKit.set("remote", remote || "");
  }
  get homepage(): string | undefined {
    return this.yKit.get("homepage") as string | undefined;
  }
  set homepage(homepage: string | undefined) {
    this.yKit.set("homepage", homepage || "");
  }
  get license(): string | undefined {
    return this.yKit.get("license") as string | undefined;
  }
  set license(license: string | undefined) {
    this.yKit.set("license", license || "");
  }
  get preview(): string | undefined {
    return this.yKit.get("preview") as string | undefined;
  }
  set preview(preview: string | undefined) {
    this.yKit.set("preview", preview || "");
  }
  get concepts(): Concept[] | undefined {
    const concepts = this.snapshotConcepts();
    return concepts.length > 0 ? concepts : undefined;
  }
  set concepts(concepts: Concept[] | undefined) {
    this.yConcepts.delete(0, this.yConcepts.length);
    this.conceptStores.clear();
    this._conceptsCache = undefined;
    this._conceptsVersion = 0;
    if (concepts) concepts.forEach((concept) => this.createConcept(concept));
  }
  get icon(): string | undefined {
    return this.yKit.get("icon") as string | undefined;
  }
  set icon(icon: string | undefined) {
    this.yKit.set("icon", icon || "");
  }
  get image(): string | undefined {
    return this.yKit.get("image") as string | undefined;
  }
  set image(image: string | undefined) {
    this.yKit.set("image", image || "");
  }
  get description(): string | undefined {
    return this.yKit.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yKit.set("description", description || "");
  }
  get createdAt(): Date {
    return new Date(this.yKit.get("createdAt") as string);
  }
  get updatedAt(): Date {
    return new Date(this.yKit.get("updatedAt") as string);
  }

  get fileUrls(): Map<Url, Url> {
    return this.regularFiles;
  }

  get isLocallyPersisted(): boolean {
    return !!this.persistence;
  }

  get isRemotelySynced(): boolean {
    return !!this.remoteProviders;
  }

  get isTemporary(): boolean {
    return !this.isLocallyPersisted && !this.isRemotelySynced;
  }

  updated(): void {
    this.yKit.set("updatedAt", new Date().toISOString());
  }

  hasType(guid: string): boolean {
    return this.types.has(guid);
  }

  createType(type: Type): void {
    if (this.hasType(type.guid)) throw new Error(`Type (${type.name}) already exists.`);
    const yType = new Y.Map<YTypeVal>();
    this.yTypes.push([yType]);
    const yTypeStore = new TypeStore(this, yType, type);
    this.types.set(type.guid, yTypeStore);
  }

  type(guid: string): TypeStore | undefined {
    return this.types.get(guid);
  }

  hasDesign(guid: string): boolean {
    return this.designs.has(guid);
  }

  createDesign(design: Design): void {
    if (this.hasDesign(design.guid)) throw new Error(`Design (${design.name}) already exists.`);
    const yDesign = new Y.Map<YDesignVal>();
    this.yDesigns.push([yDesign]);
    const yDesignStore = new DesignStore(this, yDesign, design);
    this.designs.set(design.guid, yDesignStore);
  }

  design(guid: string): DesignStore {
    return this.designs.get(guid)!;
  }

  hasFile(guid: string): boolean {
    return this.files.has(guid);
  }

  createFile(file: SemioFile): void {
    if (this.hasFile(file.guid)) throw new Error(`File (${file.name}) already exists.`);
    const yFile = new Y.Map() as YFile;
    yFile.set("guid", file.guid);
    yFile.set("name", file.name);
    if (file.folder?.guid) yFile.set("folder", file.folder.guid);
    if (file.remote) yFile.set("remote", file.remote);
    if (file.size !== undefined) yFile.set("size", file.size);
    if (file.hash) yFile.set("hash", file.hash);
    if (file.createdAt) yFile.set("createdAt", file.createdAt);
    if (file.updatedAt) yFile.set("updatedAt", file.updatedAt);
    if (file.createdBy) yFile.set("createdBy", file.createdBy);
    if (file.updatedBy) yFile.set("updatedBy", file.updatedBy);
    this.yFiles.push([yFile]);
    const yFileStore = new FileStore(yFile);
    this.files.set(file.guid, yFileStore);
  }

  file(guid: string): FileStore {
    return this.files.get(guid)!;
  }

  hasFolder(guid: string): boolean {
    return this.folders.has(guid);
  }

  createFolder(folder: Folder): void {
    if (this.hasFolder(folder.guid)) throw new Error(`Folder (${folder.name}) already exists.`);
    const yFolder = new Y.Map() as YFolder;
    yFolder.set("guid", folder.guid);
    yFolder.set("name", folder.name);
    if (folder.parent?.guid) yFolder.set("parent", folder.parent.guid);
    if (folder.description) yFolder.set("description", folder.description);
    if (folder.createdAt) yFolder.set("createdAt", folder.createdAt);
    if (folder.updatedAt) yFolder.set("updatedAt", folder.updatedAt);
    if (folder.createdBy) yFolder.set("createdBy", folder.createdBy);
    if (folder.updatedBy) yFolder.set("updatedBy", folder.updatedBy);
    this.yFolders.push([yFolder]);
    const yFolderStore = new FolderStore(yFolder);
    this.folders.set(folder.guid, yFolderStore);
  }

  updateFolder(guid: string, folderDiff: FolderDiff): void {
    const folderStore = this.folders.get(guid);
    if (!folderStore) throw new Error(`Folder with guid ${guid} not found.`);
    folderStore.change(folderDiff);
  }

  deleteFolder(guid: string): void {
    const folderStore = this.folders.get(guid);
    if (!folderStore) throw new Error(`Folder with guid ${guid} not found.`);
    const index = this.yFolders.toArray().indexOf(folderStore.yFolder);
    if (index !== -1) {
      this.yFolders.delete(index, 1);
    }
    this.folders.delete(guid);
  }

  folder(guid: string): FolderStore {
    return this.folders.get(guid)!;
  }

  private resolveFolderPath(folderGuid?: string): string {
    if (!folderGuid) return "";
    const folderStore = this.folders.get(folderGuid);
    if (!folderStore) return "";
    const parentPath = this.resolveFolderPath(folderStore.parent);
    return parentPath ? `${parentPath}/${folderStore.name}` : folderStore.name;
  }

  private getFileStoragePath(file: SemioFile): string {
    const folderPath = this.resolveFolderPath(file.folder?.guid);
    return folderPath ? `${folderPath}/${file.name}` : file.name;
  }

  getFileUrl(fileGuid: string): string {
    const fileStore = this.files.get(fileGuid);
    if (!fileStore) return "";
    const file = fileStore.snapshot();
    if (this.fileProvider) {
      return this.fileProvider.getUrl(this.guid, fileGuid, this.getFileStoragePath(file));
    }
    return file.remote ?? "";
  }

  async getFileBlobUrl(fileGuid: string): Promise<string> {
    const fileStore = this.files.get(fileGuid);
    if (!fileStore) return "";
    const file = fileStore.snapshot();

    const storagePath = this.getFileStoragePath(file);
    const memoryUrl = this.regularFiles.get(storagePath);
    if (memoryUrl) {
      return memoryUrl;
    }

    if (file.remote && (file.remote.startsWith("http://") || file.remote.startsWith("https://"))) {
      return file.remote;
    }

    if (this.fileProvider) {
      try {
        const blob = await this.fileProvider.download(this.guid, fileGuid, storagePath);
        if (blob) {
          const blobUrl = URL.createObjectURL(blob);

          this.regularFiles.set(storagePath, blobUrl);
          return blobUrl;
        }
      } catch (error) {
        console.error("[KitStore] Failed to get blob for file:", fileGuid, error);
      }
    }

    return "";
  }

  buildFilePathMap(): Map<string, string> {
    const pathMap = new Map<string, string>();
    for (const [fileGuid, fileStore] of this.files) {
      const file = fileStore.snapshot();
      const storagePath = this.getFileStoragePath(file);
      pathMap.set(storagePath, fileGuid);
    }
    return pathMap;
  }

  async storeFileBlobs(blobs: Map<string, Blob>): Promise<void> {
    const pathMap = this.buildFilePathMap();

    for (const [path, blob] of blobs) {
      const fileGuid = pathMap.get(path);
      if (fileGuid) {
        const objectUrl = URL.createObjectURL(blob);
        this.regularFiles.set(path, objectUrl);

        if (this.fileProvider) {
          try {
            const remoteUrl = await this.fileProvider.upload(this.guid, fileGuid, path, blob);
            const fileStore = this.files.get(fileGuid);
            if (fileStore) {
              fileStore.change({ remote: remoteUrl });
            }
          } catch (error) {
            console.error(`[KIT ${this.name}] Failed to upload file ${path}:`, error);
          }
        }
      }
    }
  }

  hasQuality(guid: string): boolean {
    return this.qualities.has(guid);
  }

  createQuality(quality: Quality): void {
    if (this.hasQuality(quality.guid)) throw new Error(`Quality (${quality.key}) already exists.`);
    const yQuality = new Y.Map() as YQuality;
    this.yQualities.push([yQuality]);
    const yQualityStore = new QualityStore(yQuality, quality);
    this.qualities.set(quality.guid, yQualityStore);
  }

  quality(guid: string): QualityStore {
    return this.qualities.get(guid)!;
  }

  hasBenchmark(guid: string): boolean {
    return this.benchmarks.has(guid);
  }

  createBenchmark(benchmark: Benchmark): void {
    if (this.hasBenchmark(benchmark.guid)) throw new Error(`Benchmark (${benchmark.name}) already exists.`);
    const yBenchmark = new Y.Map() as YBenchmark;
    this.yBenchmarks.push([yBenchmark]);
    const yBenchmarkStore = new BenchmarkStore(yBenchmark, benchmark);
    this.benchmarks.set(benchmark.guid, yBenchmarkStore);
  }

  benchmark(guid: string): BenchmarkStore {
    return this.benchmarks.get(guid)!;
  }

  hasAuthor(guid: string): boolean {
    return this.authors.has(guid);
  }

  createAuthor(author: Author): void {
    if (this.hasAuthor(author.guid)) throw new Error(`Author (${author.email}) already exists.`);
    const yAuthor = new Y.Map<YAuthorVal>();
    this.yAuthors.push([yAuthor]);
    const yAuthorStore = new AuthorStore(yAuthor, author);
    this.authors.set(author.guid, yAuthorStore);
  }

  author(guid: string): AuthorStore {
    return this.authors.get(guid)!;
  }

  hasAttribute(attribute: string | Attribute): boolean {
    return Array.from(this.attributes.values()).some((a) => a.snapshot().key === (typeof attribute === "string" ? attribute : attribute.key));
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map() as YAttribute;

    yAttribute.set("guid", attribute.guid);
    yAttribute.set("key", attribute.key);
    yAttribute.set("value", attribute.value || "");
    yAttribute.set("definition", attribute.definition || "");
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  attribute(guid: string): AttributeStore {
    return this.attributes.get(guid)!;
  }

  hasConcept(guid: string): boolean {
    return this.conceptStores.has(guid);
  }

  createConcept(concept: Concept): void {
    if (this.hasConcept(concept.guid)) throw new Error(`Concept (${concept.name}) already exists.`);
    const yConcept = new Y.Map() as YConcept;
    yConcept.set("guid", concept.guid);
    yConcept.set("name", concept.name);
    if (concept.description !== undefined) yConcept.set("description", concept.description);
    if (concept.icon !== undefined) yConcept.set("icon", concept.icon);
    this.yConcepts.push([yConcept]);
    const yConceptStore = new ConceptStore(yConcept, concept);
    this.conceptStores.set(concept.guid, yConceptStore);
    this._conceptsCache = undefined;
    this._conceptsVersion = 0;
  }

  concept(guid: string): ConceptStore {
    return this.conceptStores.get(guid)!;
  }

  updateConcept(guid: string, conceptDiff: ConceptDiff): void {
    const conceptStore = this.conceptStores.get(guid);
    if (!conceptStore) throw new Error(`Concept with guid ${guid} not found.`);
    conceptStore.change(conceptDiff);
    this._conceptsCache = undefined;
    this._conceptsVersion = 0;
  }

  deleteConcept(guid: string): void {
    const conceptStore = this.conceptStores.get(guid);
    if (!conceptStore) throw new Error(`Concept with guid ${guid} not found.`);
    const index = this.yConcepts.toArray().findIndex((yConcept: any) => {
      const yMap = yConcept[0] as Y.Map<any>;
      return yMap.get("guid") === guid;
    });
    if (index !== -1) {
      this.yConcepts.delete(index, 1);
    }
    this.conceptStores.delete(guid);
    this._conceptsCache = undefined;
    this._conceptsVersion = 0;
  }

  hash(kit: Kit): string {
    return JSON.stringify(kit);
  }

  markDirty = () => {
    this.dirty = true;
  };

  snapshot = (): Kit => {
    if (!this.dirty && this.cache) {
      return this.cache;
    }

    const currentData = {
      guid: this.guid,
      name: this.name,
      version: this.version,
      remote: this.remote,
      homepage: this.homepage,
      license: this.license,
      preview: this.preview,
      concepts: this.snapshotConcepts(),
      icon: this.icon,
      image: this.image,
      description: this.description,
      types: Array.from(this.types.values()).map((type) => type.snapshot()),
      designs: Array.from(this.designs.values()).map((design) => design.snapshot()),
      qualities: Array.from(this.qualities.values()).map((quality) => quality.snapshot()),
      files: Array.from(this.files.values()).map((file) => file.snapshot()),
      folders: Array.from(this.folders.values()).map((folder) => folder.snapshot()),
      authors: Array.from(this.authors.values()).map((author) => author.snapshot()),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
      createdAt: this.createdAt.toISOString(),
      updatedAt: this.updatedAt.toISOString(),
    };
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
    this.dirty = false;
    return this.cache;
  };

  change = (diff: KitDiff) => {
    this.yDoc.transact(() => {
      if (diff.guid) this.guid = diff.guid;
      if (diff.name) this.name = diff.name;
      if (diff.version) this.version = diff.version;
      if (diff.remote) this.remote = diff.remote;
      if (diff.homepage) this.homepage = diff.homepage;
      if (diff.license) this.license = diff.license;

      if (diff.concepts) {
        if (diff.concepts.added) {
          diff.concepts.added.forEach((concept) => this.createConcept(concept));
        }
        if (diff.concepts.updated) {
          diff.concepts.updated.forEach(({ concept, diff: conceptDiff }) => {
            const conceptStore = this.conceptStores.get(concept.guid);
            if (conceptStore) {
              conceptStore.change(conceptDiff);
            }
          });
        }
        if (diff.concepts.removed) {
          diff.concepts.removed.forEach((conceptId) => {
            const conceptGuid = conceptId.guid;
            if (this.conceptStores.has(conceptGuid)) {
              this.deleteConcept(conceptGuid);
            }
          });
        }
        this._conceptsCache = undefined;
        this._conceptsVersion = 0;
      }

      if (diff.authors) {
        if (diff.authors.added) {
          diff.authors.added.forEach((author) => this.createAuthor(author));
        }
        if (diff.authors.updated) {
          diff.authors.updated.forEach(({ author, diff: authorDiff }) => {
            const authorStore = this.authors.get(author.guid);
            if (authorStore) {
              authorStore.change(authorDiff);
            }
          });
        }
        if (diff.authors.removed) {
          diff.authors.removed.forEach((authorId) => {
            const authorGuid = authorId.guid;
            if (this.authors.has(authorGuid)) {
              this.authors.delete(authorGuid);

              const index = Array.from(this.yAuthors).findIndex((yAuthor: any) => {
                const yMap = yAuthor[0] as Y.Map<any>;
                return yMap.get("guid") === authorGuid;
              });
              if (index !== -1) {
                this.yAuthors.delete(index, 1);
              }
            }
          });
        }
      }
      if (diff.types) {
        if (diff.types.added) {
          diff.types.added.forEach((type) => this.createType(type));
        }
        if (diff.types.updated) {
          diff.types.updated.forEach(({ type, diff: typeDiff }) => {
            const typeStore = this.types.get(type.guid);
            if (typeStore) {
              typeStore.change(typeDiff);
            }
          });
        }
        if (diff.types.removed) {
          diff.types.removed.forEach((typeId) => {
            const guid = typeId.guid;
            if (this.types.has(guid)) {
              this.types.delete(guid);

              const index = Array.from(this.yTypes).findIndex((yType: any) => {
                const yMap = yType[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (index !== -1) {
                this.yTypes.delete(index, 1);
              }
            }
          });
        }
      }
      if (diff.designs) {
        if (diff.designs.added) {
          diff.designs.added.forEach((design) => this.createDesign(design));
        }
        if (diff.designs.updated) {
          diff.designs.updated.forEach(({ design, diff: designDiff }) => {
            const designStore = this.designs.get(design.guid);
            if (designStore) {
              designStore.change(designDiff);
            }
          });
        }
        if (diff.designs.removed) {
          diff.designs.removed.forEach((designId) => {
            const guid = designId.guid;
            if (this.designs.has(guid)) {
              this.designs.delete(guid);

              const index = Array.from(this.yDesigns).findIndex((yDesign: any) => {
                const yMap = yDesign[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (index !== -1) {
                this.yDesigns.delete(index, 1);
              }
            }
          });
        }
      }
      if (diff.files) {
        if (diff.files.added) {
          diff.files.added.forEach((file) => this.createFile(file));
        }
        if (diff.files.updated) {
          diff.files.updated.forEach(({ file, diff: fileDiff }) => {
            const fileStore = this.files.get(file.guid);
            if (fileStore) {
              fileStore.change(fileDiff);
            }
          });
        }
        if (diff.files.removed) {
          diff.files.removed.forEach((fileId) => {
            const guid = fileId.guid;
            if (this.files.has(guid)) {
              this.files.delete(guid);

              const index = Array.from(this.yFiles).findIndex((yFile: any) => {
                const yMap = yFile[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (index !== -1) {
                this.yFiles.delete(index, 1);
              }
            }
          });
        }
      }
      if (diff.folders) {
        if (diff.folders.added) {
          diff.folders.added.forEach((folder) => this.createFolder(folder));
        }
        if (diff.folders.updated) {
          diff.folders.updated.forEach(({ folder, diff: folderDiff }) => {
            const folderStore = this.folders.get(folder.guid);
            if (folderStore) {
              folderStore.change(folderDiff);
            }
          });
        }
        if (diff.folders.removed) {
          diff.folders.removed.forEach((folderId) => {
            const guid = folderId.guid;
            if (this.folders.has(guid)) {
              this.folders.delete(guid);

              const index = Array.from(this.yFolders).findIndex((yFolder: any) => {
                const yMap = yFolder[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (index !== -1) {
                this.yFolders.delete(index, 1);
              }
            }
          });
        }
      }
      if (diff.qualities) {
        if (diff.qualities.added) {
          diff.qualities.added.forEach((quality) => this.createQuality(quality));
        }
        if (diff.qualities.updated) {
          diff.qualities.updated.forEach(({ quality, diff: qualityDiff }) => {
            const qualityStore = this.qualities.get(quality.guid);
            if (qualityStore) {
              qualityStore.change(qualityDiff);
            }
          });
        }
        if (diff.qualities.removed) {
          diff.qualities.removed.forEach((qualityId) => {
            const guid = qualityId.guid;
            if (this.qualities.has(guid)) {
              this.qualities.delete(guid);

              const index = Array.from(this.yQualities).findIndex((yQuality: any) => {
                const yMap = yQuality[0] as Y.Map<any>;
                return yMap.get("guid") === guid;
              });
              if (index !== -1) {
                this.yQualities.delete(index, 1);
              }
            }
          });
        }
      }
      this.yKit.set("updatedAt", new Date().toISOString());
      this.dirty = true;
      this.cache = undefined;
      this.cacheHash = undefined;
    });
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yKit, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yKit, subscribe, true);
  };

  // #region 🔖YPath API

  private pathSubscribers: Map<string, Set<() => void>> = new Map();
  private pathObservers: Map<string, Disposable> = new Map();
  public readonly derived: DerivedStore = new DerivedStore();

  onPathChanged = (path: YPath, subscribe: Subscribe): Unsubscribe => {
    const pathKey = JSON.stringify(path);
    const subscriberCallback = () => {
      subscribe(() => {});
    };
    if (!this.pathSubscribers.has(pathKey)) {
      this.pathSubscribers.set(pathKey, new Set());
      const pathObserver = createPathObserver(this.yKit, path, () => {
        const subscribers = this.pathSubscribers.get(pathKey);
        if (subscribers) subscribers.forEach((cb) => cb());
        return () => {};
      });
      this.pathObservers.set(pathKey, pathObserver);
    }
    const subscribers = this.pathSubscribers.get(pathKey)!;
    subscribers.add(subscriberCallback);
    return () => {
      subscribers.delete(subscriberCallback);
      if (subscribers.size === 0) {
        const observer = this.pathObservers.get(pathKey);
        if (observer) {
          observer();
          this.pathObservers.delete(pathKey);
        }
        this.pathSubscribers.delete(pathKey);
      }
    };
  };

  getPathSnapshot = (path: YPath): any => {
    return getValueAtPath(this.yKit, path);
  };

  // #endregion 🔖YPath API

  snapshotConcepts = (): Concept[] => {
    const currentVersion = (this.yConcepts as any)._clock || this.conceptStores.size;
    if (this._conceptsCache && this._conceptsVersion === currentVersion) {
      return this._conceptsCache;
    }
    this._conceptsCache = Array.from(this.conceptStores.values()).map((concept) => concept.snapshot());
    this._conceptsVersion = currentVersion;
    return this._conceptsCache;
  };

  snapshotFiles = (): SemioFile[] => {
    const currentVersion = (this.yFiles as any)._clock || this.files.size;
    if (this._filesCache && this._filesVersion === currentVersion) {
      return this._filesCache;
    }
    this._filesCache = Array.from(this.files.values()).map((file) => file.snapshot());
    this._filesVersion = currentVersion;
    return this._filesCache;
  };

  snapshotTypes = (): Type[] => {
    const currentVersion = (this.yTypes as any)._clock || this.types.size;
    if (this._typesCache && this._typesVersion === currentVersion) {
      return this._typesCache;
    }
    this._typesCache = Array.from(this.types.values()).map((type) => type.snapshot());
    this._typesVersion = currentVersion;
    return this._typesCache;
  };

  snapshotDesigns = (): Design[] => {
    const currentVersion = (this.yDesigns as any)._clock || this.designs.size;
    if (this._designsCache && this._designsVersion === currentVersion) {
      return this._designsCache;
    }
    this._designsCache = Array.from(this.designs.values()).map((design) => design.snapshot());
    this._designsVersion = currentVersion;
    return this._designsCache;
  };

  snapshotQualities = (): Quality[] => {
    const currentVersion = (this.yQualities as any)._clock || this.qualities.size;
    if (this._qualitiesCache && this._qualitiesVersion === currentVersion) {
      return this._qualitiesCache;
    }
    this._qualitiesCache = Array.from(this.qualities.values()).map((quality) => quality.snapshot());
    this._qualitiesVersion = currentVersion;
    return this._qualitiesCache;
  };

  snapshotAuthors = (): Author[] => {
    const currentVersion = (this.yAuthors as any)._clock || this.authors.size;
    if (this._authorsCache && this._authorsVersion === currentVersion) {
      return this._authorsCache;
    }
    this._authorsCache = Array.from(this.authors.values()).map((author) => author.snapshot());
    this._authorsVersion = currentVersion;
    return this._authorsCache;
  };

  snapshotFolders = (): Folder[] => {
    const currentVersion = (this.yFolders as any)._clock || this.folders.size;
    if (this._foldersCache && this._foldersVersion === currentVersion) {
      return this._foldersCache;
    }
    this._foldersCache = Array.from(this.folders.values()).map((folder) => folder.snapshot());
    this._foldersVersion = currentVersion;
    return this._foldersCache;
  };

  onConceptsChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._conceptsCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yConcepts.observeDeep(notifySubscriber);
      return () => this.yConcepts.unobserveDeep(notifySubscriber);
    }
    this.yConcepts.observe(notifySubscriber);
    return () => this.yConcepts.unobserve(notifySubscriber);
  };

  onTypesChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._typesCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yTypes.observeDeep(notifySubscriber);
      return () => this.yTypes.unobserveDeep(notifySubscriber);
    }
    this.yTypes.observe(notifySubscriber);
    return () => this.yTypes.unobserve(notifySubscriber);
  };

  onFilesChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._filesCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yFiles.observeDeep(notifySubscriber);
      return () => this.yFiles.unobserveDeep(notifySubscriber);
    }
    this.yFiles.observe(notifySubscriber);
    return () => this.yFiles.unobserve(notifySubscriber);
  };

  onDesignsChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._designsCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yDesigns.observeDeep(notifySubscriber);
      return () => this.yDesigns.unobserveDeep(notifySubscriber);
    }
    this.yDesigns.observe(notifySubscriber);
    return () => this.yDesigns.unobserve(notifySubscriber);
  };

  onQualitiesChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._qualitiesCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yQualities.observeDeep(notifySubscriber);
      return () => this.yQualities.unobserveDeep(notifySubscriber);
    }
    this.yQualities.observe(notifySubscriber);
    return () => this.yQualities.unobserve(notifySubscriber);
  };

  onAuthorsChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._authorsCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yAuthors.observeDeep(notifySubscriber);
      return () => this.yAuthors.unobserveDeep(notifySubscriber);
    }
    this.yAuthors.observe(notifySubscriber);
    return () => this.yAuthors.unobserve(notifySubscriber);
  };

  onFoldersChanged = (subscribe: Subscribe, deep: boolean = false): Disposable => {
    const notifySubscriber = () => {
      this._foldersCache = undefined;
      subscribe(() => {});
    };
    if (deep) {
      this.yFolders.observeDeep(notifySubscriber);
      return () => this.yFolders.unobserveDeep(notifySubscriber);
    }
    this.yFolders.observe(notifySubscriber);
    return () => this.yFolders.unobserve(notifySubscriber);
  };

  onScalarFieldChanged = (key: string, subscribe: Subscribe): Disposable => {
    return createFieldObserver(this.yKit, key, subscribe, false);
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

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit store`);
    const context: KitCommandContext = {
      kit: this.snapshot(),
      fileUrls: this.fileUrls,
      origin,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);

      if (result.diff.files) {
        if (result.diff.files.added && result.files) {
          for (let i = 0; i < result.diff.files.added.length; i++) {
            const file = result.diff.files.added[i];
            const blob = result.files[i];
            if (blob) {
              const objectUrl = URL.createObjectURL(blob);
              const fileStore = this.files.get(file.guid);
              const storagePath = fileStore ? this.getFileStoragePath(fileStore.snapshot()) : file.name;
              this.regularFiles.set(storagePath, objectUrl);

              if (this.fileProvider) {
                try {
                  const remoteUrl = await this.fileProvider.upload(this.guid, file.guid, storagePath, blob);
                  this.file(file.guid).change({ remote: remoteUrl });
                } catch (error) {
                  console.error(`[KIT ${this.name}] Failed to upload file ${storagePath}:`, error);
                }
              }
            }
          }
        }

        if (result.diff.files.removed) {
          for (const fileId of result.diff.files.removed) {
            const guid = fileId.guid;
            const fileStore = this.files.get(guid);
            if (fileStore) {
              const file = fileStore.snapshot();
              const storagePath = this.getFileStoragePath(file);

              const objectUrl = this.regularFiles.get(storagePath);
              if (objectUrl) {
                URL.revokeObjectURL(objectUrl);
                this.regularFiles.delete(storagePath);
              }

              if (this.fileProvider) {
                try {
                  await this.fileProvider.delete(this.guid, guid, storagePath);
                } catch (error) {
                  console.error(`[KIT ${this.name}] Failed to delete file ${storagePath}:`, error);
                }
              }
            }
          }
        }
      }
    }

    if (result.files) {
      result.files.forEach((file) => {
        const objectUrl = URL.createObjectURL(file);
        this.regularFiles.set(file.name, objectUrl);
      });
    }

    return result as T;
  }

  registerCommand(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }

  register(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable {
    return this.registerCommand(command, callback);
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }
}

type KitScope = { guid: string };
const KitScopeContext = createContext<KitScope | null>(null);
export const KitScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};
export const useKitScope = () => useContext(KitScopeContext);
export const useIsInKitScope = () => useKitScope() !== null;

export function useKitStore<T>(selector?: (store: KitStore) => T, guid?: string): T | KitStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? guid;
  if (!kitGuid || !store.hasKit(kitGuid)) return null;
  const kitStore = store.kit(kitGuid);
  return selector ? selector(kitStore) : kitStore;
}

export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, guid?: Guid, deep: boolean = false): T | KitShallow | Kit | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid ?? undefined) as KitStore | null;
  const syncedDeep = useSyncDeep<Kit, T>(kitStore, selector ? selector : (identitySelector as any));
  const synced = useSyncOptional<KitShallow, T>(kitStore as any, selector ? selector : (identitySelector as any));
  if (!resolvedGuid || !kitStore) return null;
  return deep ? syncedDeep : synced;
}

// #region 🔖Targeted Kit Hooks

const EMPTY_TYPES: Type[] = [];
const EMPTY_AUTHORS: Author[] = [];
const EMPTY_FILES: SemioFile[] = [];
const EMPTY_QUALITIES: Quality[] = [];
const EMPTY_DESIGNS: Design[] = [];
const EMPTY_FOLDERS: Folder[] = [];
const EMPTY_INTERFACES: Port[] = [];
const EMPTY_TAGS: Tag[] = [];
const EMPTY_CONCEPTS: Concept[] = [];

const selectTypes = (k: KitShallow | Kit) => k.types ?? EMPTY_TYPES;
const selectName = (k: KitShallow | Kit) => k.name;
const selectDescription = (k: KitShallow | Kit) => k.description;
const selectAuthors = (k: KitShallow | Kit) => k.authors ?? EMPTY_AUTHORS;
const selectFiles = (k: KitShallow | Kit) => k.files ?? EMPTY_FILES;
const selectQualities = (k: KitShallow | Kit) => k.qualities ?? EMPTY_QUALITIES;
const selectDesigns = (k: KitShallow | Kit) => k.designs ?? EMPTY_DESIGNS;
const selectFolders = (k: KitShallow | Kit) => k.folders ?? EMPTY_FOLDERS;
const selectPorts = (k: KitShallow | Kit) => k.ports ?? EMPTY_INTERFACES;
const selectTags = (k: KitShallow | Kit) => k.tags ?? EMPTY_TAGS;
const selectConcepts = (k: KitShallow | Kit) => k.concepts ?? EMPTY_CONCEPTS;

export function useKitTypes(guid?: Guid): Type[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onTypesChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_TYPES;
    return kitStore.snapshotTypes();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitName(guid?: Guid): string {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onScalarFieldChanged("name", () => {
        callback();
        return () => {};
      });
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return "";
    return kitStore.name;
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitDescription(guid?: Guid): string | undefined {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onScalarFieldChanged("description", () => {
        callback();
        return () => {};
      });
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return undefined;
    return kitStore.description;
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitAuthors(guid?: Guid): Author[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onAuthorsChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_AUTHORS;
    return kitStore.snapshotAuthors();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitFiles(guid?: Guid): SemioFile[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onFilesChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_FILES;
    return kitStore.snapshotFiles();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitQualities(guid?: Guid): Quality[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onQualitiesChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_QUALITIES;
    return kitStore.snapshotQualities();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitDesigns(guid?: Guid): Design[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onDesignsChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_DESIGNS;
    return kitStore.snapshotDesigns();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useDesigns(): Design[] {
  return useKitDesigns();
}

export function useKitFolders(guid?: Guid): Folder[] {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid) as KitStore | null;

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onFoldersChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_FOLDERS;
    return kitStore.snapshotFolders();
  }, [kitStore]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useKitPorts(guid?: Guid): Port[] {
  return useKit(selectPorts, guid, true) as Port[];
}

export function useKitTags(guid?: Guid): Tag[] {
  return useKit(selectTags, guid, true) as Tag[];
}

export function useKitConcepts(guid?: Guid): Concept[] {
  return useKit(selectConcepts, guid, true) as Concept[];
}

export function useTypeFromKit(typeGuid: Guid, kitGuid?: Guid): Type | undefined {
  const kitTypes = useKitTypes(kitGuid);
  return useMemo(() => kitTypes?.find((t) => t.guid === typeGuid), [kitTypes, typeGuid]);
}

export function useDesignFromKit(designGuid: Guid, kitGuid?: Guid): Design | undefined {
  const kitDesigns = useKitDesigns(kitGuid);
  return useMemo(() => kitDesigns?.find((d) => d.guid === designGuid), [kitDesigns, designGuid]);
}

export function useKitConnectorCompatibility(kitGuid?: Guid): { ports: Port[] } {
  const ports = useKitPorts(kitGuid);
  return useMemo(() => ({ ports }), [ports]);
}

// #endregion 🔖Targeted Kit Hooks

export function useFileUrls(): Map<Url, Url> {
  const kitStore = useKitStore() as KitStore | null;
  if (!kitStore) {
    return new Map();
  }
  return kitStore.fileUrls;
}

export function useKitTransaction(): Transaction {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid;
  const getOrigin = useOrigin();

  if (!kitGuid || !store.hasKit(kitGuid)) {
    return {};
  }

  const kitStore = store.kit(kitGuid);
  return {
    start: () => {
      kitStore.yDoc.transact(() => {}, getOrigin());
    },
    finalize: () => {},
    abort: () => {},
  };
}

export function useKitCommands() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid;
  const getOrigin = useOrigin();

  if (!kitGuid || !store.hasKit(kitGuid)) {
    return null;
  }

  const kitStore = store.kit(kitGuid);
  return {
    importKit: (url: string) => kitStore.execute("semio.kit.import", getOrigin(), url),
    exportKit: () => kitStore.execute("semio.kit.export", getOrigin()),
    createAuthor: (author: Author) => kitStore.execute("semio.kit.createAuthor", getOrigin(), author),
    updateAuthor: (Guid: Guid, authorDiff: AuthorDiff) => kitStore.execute("semio.kit.updateAuthor", getOrigin(), Guid, authorDiff),
    deleteAuthor: (Guid: Guid) => kitStore.execute("semio.kit.deleteAuthor", getOrigin(), Guid),
    createType: (type: Type) => kitStore.execute("semio.kit.createType", getOrigin(), type),
    updateType: (guid: Guid, diff: TypeDiff) => kitStore.execute("semio.kit.updateType", getOrigin(), guid, diff),
    deleteType: (guid: Guid) => kitStore.execute("semio.kit.deleteType", getOrigin(), guid),
    createDesign: (design: Design) => kitStore.execute("semio.kit.createDesign", getOrigin(), design),
    updateDesign: (guid: Guid, diff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", getOrigin(), guid, diff),
    deleteDesign: (guid: Guid) => kitStore.execute("semio.kit.deleteDesign", getOrigin(), guid),
    createQuality: (quality: Quality) => kitStore.execute("semio.kit.createQuality", getOrigin(), quality),
    updateQuality: (guid: Guid, diff: QualityDiff) => kitStore.execute("semio.kit.updateQuality", getOrigin(), guid, diff),
    deleteQuality: (guid: Guid) => kitStore.execute("semio.kit.deleteQuality", getOrigin(), guid),
    createPort: (iface: Port) => kitStore.execute("semio.kit.createPort", getOrigin(), iface),
    updatePort: (guid: Guid, diff: PortDiff) => kitStore.execute("semio.kit.updatePort", getOrigin(), guid, diff),
    deletePort: (guid: Guid) => kitStore.execute("semio.kit.deletePort", getOrigin(), guid),
    createTag: (tag: Tag) => kitStore.execute("semio.kit.createTag", getOrigin(), tag),
    updateTag: (guid: Guid, diff: TagDiff) => kitStore.execute("semio.kit.updateTag", getOrigin(), guid, diff),
    deleteTag: (guid: Guid) => kitStore.execute("semio.kit.deleteTag", getOrigin(), guid),
    createConcept: (concept: Concept) => kitStore.execute("semio.kit.createConcept", getOrigin(), concept),
    updateConcept: (guid: Guid, diff: ConceptDiff) => kitStore.execute("semio.kit.updateConcept", getOrigin(), guid, diff),
    deleteConcept: (guid: Guid) => kitStore.execute("semio.kit.deleteConcept", getOrigin(), guid),
    addFile: (file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", getOrigin(), file, blob),
    updateFile: (url: Url, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", getOrigin(), url, fileDiff, blob),
    removeFile: (url: Url) => kitStore.execute("semio.kit.removeFile", getOrigin(), url),
    createFolder: (folder: Folder) => kitStore.execute("semio.kit.createFolder", getOrigin(), folder),
    updateFolder: (guid: Guid, folderDiff: FolderDiff) => kitStore.execute("semio.kit.updateFolder", getOrigin(), guid, folderDiff),
    deleteFolder: (guid: Guid) => kitStore.execute("semio.kit.deleteFolder", getOrigin(), guid),
    moveToFolder: (artifactKind: string, artifactGuid: Guid, folderGuid: Guid | null) => kitStore.execute("semio.kit.moveToFolder", getOrigin(), artifactGuid, artifactKind, folderGuid),
    addPiece: (design: Guid, piece: Piece) => kitStore.execute("semio.kit.addPiece", getOrigin(), design, piece),
    addPieces: (design: Guid, pieces: Piece[]) => kitStore.execute("semio.kit.addPieces", getOrigin(), design, pieces),
    removePiece: (design: Guid, piece: Guid) => kitStore.execute("semio.kit.removePiece", getOrigin(), design, piece),
    removePieces: (design: Guid, pieces: Guid[]) => kitStore.execute("semio.kit.removePieces", getOrigin(), design, pieces),
    addConnection: (design: Guid, connection: Connection) => kitStore.execute("semio.kit.addConnection", getOrigin(), design, connection),
    addConnections: (design: Guid, connections: Connection[]) => kitStore.execute("semio.kit.addConnections", getOrigin(), design, connections),
    removeConnection: (design: Guid, connection: Guid) => kitStore.execute("semio.kit.removeConnection", getOrigin(), design, connection),
    removeConnections: (design: Guid, connections: Guid[]) => kitStore.execute("semio.kit.removeConnections", getOrigin(), design, connections),
    deleteSelected: (design: Guid, selectedPieces: Guid[], selectedConnections: Guid[]) => kitStore.execute("semio.kit.deleteSelected", getOrigin(), design, selectedPieces, selectedConnections),
  };
}

// #endregion 🔖Kit

// #region 🔖Commands

const sqlWasmUrl = "https://sql.js.org/dist/sql-wasm.wasm";

export const kitCommands = {
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, guid: Guid, diff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ author: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { authors: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, guid: Guid, diff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ type: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { types: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, guid: Guid, diff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ design: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { designs: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createQuality": (context: KitCommandContext, quality: Quality): KitCommandResult => {
    return {
      diff: { qualities: { added: [quality] } },
    };
  },
  "semio.kit.updateQuality": (context: KitCommandContext, guid: Guid, diff: QualityDiff): KitCommandResult => {
    return {
      diff: { qualities: { updated: [{ quality: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteQuality": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { qualities: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createPort": (context: KitCommandContext, iface: Port): KitCommandResult => {
    return {
      diff: { ports: { added: [iface] } },
    };
  },
  "semio.kit.updatePort": (context: KitCommandContext, guid: Guid, diff: PortDiff): KitCommandResult => {
    return {
      diff: { ports: { updated: [{ port: { guid }, diff }] } },
    };
  },
  "semio.kit.deletePort": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { ports: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createTag": (context: KitCommandContext, tag: Tag): KitCommandResult => {
    return {
      diff: { tags: { added: [tag] } },
    };
  },
  "semio.kit.updateTag": (context: KitCommandContext, guid: Guid, diff: TagDiff): KitCommandResult => {
    return {
      diff: { tags: { updated: [{ tag: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteTag": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { tags: { removed: [{ guid }] } },
    };
  },
  "semio.kit.createConcept": (context: KitCommandContext, concept: Concept): KitCommandResult => {
    return {
      diff: { concepts: { added: [concept] } },
    };
  },
  "semio.kit.updateConcept": (context: KitCommandContext, guid: Guid, diff: ConceptDiff): KitCommandResult => {
    return {
      diff: { concepts: { updated: [{ concept: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteConcept": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { concepts: { removed: [{ guid }] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], file.name)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.addFiles": (context: KitCommandContext, foldersToAdd: Folder[], filesToAdd: { file: SemioFile; blob?: Blob }[]): KitCommandResult => {
    const semioFiles: SemioFile[] = [];
    const files: File[] = [];
    for (const { file, blob } of filesToAdd) {
      semioFiles.push(file);
      if (blob) files.push(new File([blob], file.name));
    }
    return {
      diff: { folders: { added: foldersToAdd }, files: { added: semioFiles } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, fileGuid: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const existing = context.kit.files?.find((f) => f.guid === fileGuid);
    const fileName = fileDiff.name ?? existing?.name ?? "file";
    const files: File[] = blob ? [new File([blob], fileName)] : [];
    return {
      diff: { files: { updated: [{ file: { guid: fileGuid }, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, fileGuid: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [{ guid: fileGuid }] } },
    };
  },
  "semio.kit.createFolder": (context: KitCommandContext, folder: Folder): KitCommandResult => {
    return {
      diff: { folders: { added: [folder] } },
    };
  },
  "semio.kit.updateFolder": (context: KitCommandContext, guid: Guid, diff: FolderDiff): KitCommandResult => {
    return {
      diff: { folders: { updated: [{ folder: { guid }, diff }] } },
    };
  },
  "semio.kit.deleteFolder": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { folders: { removed: [{ guid }] } },
    };
  },
  "semio.kit.moveToFolder": (context: KitCommandContext, artifactGuid: Guid, artifactKind: "type" | "design" | "quality" | "file" | "folder", folderGuid?: Guid): KitCommandResult => {
    switch (artifactKind) {
      case "type": {
        const type = context.kit.types?.find((t) => t.guid === artifactGuid);
        if (!type) throw new Error(`Type ${artifactGuid} not found`);
        if (type.parent) throw new Error("Only prototypes (types without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { types: { updated: [{ type: { guid: artifactGuid }, diff: folderDiff }] } } };
      }
      case "design": {
        const design = context.kit.designs?.find((d) => d.guid === artifactGuid);
        if (!design) throw new Error(`Design ${artifactGuid} not found`);
        if (design.parent) throw new Error("Only protodesigns (designs without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { designs: { updated: [{ design: { guid: artifactGuid }, diff: folderDiff }] } } };
      }
      case "quality": {
        const folderDiff = { folder: folderGuid };
        return { diff: { qualities: { updated: [{ quality: { guid: artifactGuid }, diff: folderDiff }] } } };
      }
      case "file": {
        const folderDiff = { folder: folderGuid ? { guid: folderGuid } : undefined };
        return { diff: { files: { updated: [{ file: { guid: artifactGuid }, diff: folderDiff }] } } };
      }
      case "folder": {
        const parentDiff = { parent: folderGuid ? { guid: folderGuid } : undefined };
        return { diff: { folders: { updated: [{ folder: { guid: artifactGuid }, diff: parentDiff }] } } };
      }
    }
  },
  "semio.kit.import": (context: KitCommandContext, url: string): KitCommandResult => {
    (async () => {
      try {
        if (url.endsWith(".json")) {
          const response = await fetch(url);
          const kit: Kit = await response.json();
          const filesToFetch: { path: string; url: string }[] = [];
          const extractFileUrls = (obj: any) => {
            if (typeof obj === "object" && obj !== null) {
              if (Array.isArray(obj)) {
                obj.forEach((item) => extractFileUrls(item));
              } else {
                Object.entries(obj).forEach(([key, value]) => {
                  if (key === "url" && typeof value === "string" && !value.startsWith("http")) {
                    filesToFetch.push({ path: value, url: new URL(value, url).href });
                  }
                  extractFileUrls(value);
                });
              }
            }
          };
          extractFileUrls(kit);
          const files: KitCommandResult["files"] = [];
          for (const file of filesToFetch) {
            try {
              const fileResponse = await fetch(file.url);
              const fileBlob = await fileResponse.blob();
              const fileName = file.path.split("/").pop() || file.path;
              files.push(new File([fileBlob], fileName));
            } catch (error) {}
          }
          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types ? { added: kit.types } : undefined,
              designs: kit.designs ? { added: kit.designs } : undefined,
              files: kit.files ? { added: kit.files } : undefined,
            },
            files,
          };
        } else {
          const { kit, files: importedFiles } = await importKit(url);
          const files: KitCommandResult["files"] = [];

          for (const [path, blob] of importedFiles.entries()) {
            files.push(new File([blob], path));
          }

          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types && kit.types.length > 0 ? { added: kit.types } : undefined,
              designs: kit.designs && kit.designs.length > 0 ? { added: kit.designs } : undefined,
              files: kit.files && kit.files.length > 0 ? { added: kit.files } : undefined,
            },
            files,
          };
        }
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.export": (context: KitCommandContext): KitCommandResult => {
    (async () => {
      try {
        const kit = context.kit;
        const files = new Map<string, Blob>();

        for (const [path, url] of context.fileUrls.entries()) {
          try {
            const response = await fetch(url);
            if (response.ok) {
              const blob = await response.blob();
              files.set(path, blob);
            }
          } catch (error) {
          }
        }

        const zipBlob = await exportKit(kit, files);
        const url = URL.createObjectURL(zipBlob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${kit.name}-${kit.version || "latest"}.semio.zip`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.addPiece": (context: KitCommandContext, guid: Guid, piece: Piece): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, guid)?.connections ?? []).some((connection) => connection.connected.piece.guid === piece.guid || connection.connecting.piece.guid === piece.guid)
                      ? piece
                      : {
                          ...piece,
                          plane: {
                            origin: { x: 0, y: 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 },
                          },
                        },
                  ],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addPieces": (context: KitCommandContext, guid: Guid, pieces: Piece[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece.guid === candidate.guid || connection.connecting.piece.guid === candidate.guid)
                      ? candidate
                      : {
                          ...candidate,
                          plane: {
                            origin: { x: 0, y: 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 },
                          },
                        },
                  ),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePiece": (context: KitCommandContext, guid: Guid, piece: Guid): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { pieces: { removed: [{ guid: piece }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePieces": (context: KitCommandContext, guid: Guid, pieces: Guid[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { pieces: { removed: pieces.map((p) => ({ guid: p })) } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnection": (context: KitCommandContext, guid: Guid, connection: Connection): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnections": (context: KitCommandContext, guid: Guid, connections: Connection[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnection": (context: KitCommandContext, guid: Guid, connectionGuid: Guid): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    const connection = design?.connections?.find((c) => c.guid === connectionGuid);
    if (!connection) return { diff: {} };
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { connections: { removed: [{ guid: connection.guid }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, guid: Guid, connectionGuids: Guid[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { guid },
              diff: { connections: { removed: connectionGuids.map((connGuid) => ({ guid: connGuid })) } },
            },
          ],
        },
      },
    };
  },
};

// #endregion 🔖Commands

// #region 🔖Machine

// #region 🔖Types

export const defaultPanelVisibility: PanelVisibility = {
  toolbar: false,
  workbench: false,
  details: false,
  chat: false,
  settings: false,
};

// #region 🔖App State Types

export interface HomeAppSelection {
  kits?: Guid[];
}
export interface HomeAppState {
  panelVisibility: PanelVisibility;
  selection?: HomeAppSelection;
  hover?: { kits?: Guid[] };
  sortColumn?: string;
  sortDirection?: "asc" | "desc";
  loadingKits: Array<{ tempGuid: string; name: string }>;
}

export type FeedbackKind = "bug" | "idea";
export type FeedbackAppKind = "home" | "kit" | "design" | "type" | "quality" | "docs" | "feedback";
export interface FeedbackFormData {
  kind: FeedbackKind;
  title: string;
  description: string;
  app?: FeedbackAppKind;
  name?: string;
  email?: string;
}
export interface FeedbackAppState {
  panelVisibility: PanelVisibility;
  formData: FeedbackFormData;
  isSubmitting: boolean;
  isSubmitted: boolean;
  error?: string;
}

export interface KitAppSelection {
  types?: Guid[];
  designs?: Guid[];
  qualities?: Guid[];
  files?: Guid[];
  authors?: Guid[];
}
export interface DiagramForceSettings {
  chargeStrength: number;
  linkDistance: number;
  collideRadius: number;
  centerStrength: number;
}
export const defaultDiagramForceSettings: DiagramForceSettings = {
  chargeStrength: -150,
  linkDistance: 100,
  collideRadius: 40,
  centerStrength: 0.05,
};
export enum KitAppFullscreenWindow {
  None = "none",
  Table = "table",
  Diagram = "diagram",
}
export interface KitAppState {
  panelVisibility: PanelVisibility;
  selection?: KitAppSelection;
  hover?: any;
  fullscreenWindow: KitAppFullscreenWindow;
  others: any[];
  filterSearch?: string;
  expandedRows: Set<string>;
  sortColumn?: string;
  sortDirection?: "asc" | "desc";
  diagramForce?: DiagramForceSettings;

  transaction: AppTransactionState;
}

export interface TypeAppSelection {
  connectors?: Guid[];
  models?: Guid[];
}
export interface TypeAppHover {
  connector?: Guid;
  model?: Guid;
}
export enum TypeAppFullscreenWindow {
  None = "none",
  Scene = "scene",
}

export interface AppTransactionState<TEdit = any> {
  isTransactionActive: boolean;
  currentTransactionStack: TEdit[];
  pastTransactionStack: TEdit[];
  redoStack: TEdit[];
}

export interface TypeAppState {
  panelVisibility: PanelVisibility;
  activeTool: ToolKind;
  fullscreenWindow: TypeAppFullscreenWindow;
  selection?: TypeAppSelection;
  hover?: TypeAppHover;
  focusedConnector?: Guid;
  selectedModelTags: Guid[];
  selectedModelGuid?: Guid;
  camera?: { position: { x: number; y: number; z: number }; target: { x: number; y: number; z: number } };
  windowLayout?: any;

  transaction: AppTransactionState;
}

export interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: Array<{ piece: Guid; connector: Guid }>;
}
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  connectors?: Array<{ piece: Guid; connector: Guid }>;
  types?: Guid[];
  designs?: Guid[];
}
export enum DesignAppFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
export interface DesignAppState {
  panelVisibility: PanelVisibility;
  selection?: DesignAppSelection;
  hover?: DesignAppHover;
  focusedPiece?: Guid;
  selectedModelTags: Record<Guid, Guid[]>;
  diagramCenter?: { x: number; y: number };
  diagramScale?: number;
  camera?: any;
  activeTool?: ToolKind;
  fullscreenWindow?: DesignAppFullscreenWindow;

  transaction: AppTransactionState;
}

export interface QualityAppSelection {
  benchmarks?: Guid[];
}
export interface QualityAppState {
  panelVisibility: PanelVisibility;
  selection?: QualityAppSelection;
  hover?: any;
  expandedBenchmarks: Set<string>;

  transaction: AppTransactionState;
}

export interface TutorialStep {
  id: string;
  title: string;
  description?: string;
  target?: string;
  action?: string;
  completed?: boolean;
}

export interface TutorialContext {
  activeTutorial?: string;
  currentStepIndex: number;
  steps: TutorialStep[];
  completedSteps: Set<string>;
  isRecording: boolean;
  recordingState: "idle" | "recording" | "paused";
  recordedEvents: any[];
}

// #endregion 🔖App State Types

export interface SketchpadMachineInput {
  id?: string;
  initialState?: Partial<SketchpadState>;
}

export interface SketchpadContext {
  id?: string;
  sketchpad: SketchpadState;

  kits: Record<Guid, AnyActorRef>;

  homeApp: HomeAppState;
  kitApps: Record<Guid, KitAppState>;
  typeApps: Record<string, TypeAppState>;
  designApps: Record<string, DesignAppState>;
  qualityApps: Record<string, QualityAppState>;
  feedbackApp: FeedbackAppState;
  tutorial: TutorialContext;

  backgroundOperations: Record<string, { type: string; status: "pending" | "running" | "completed" | "failed"; error?: string }>;
}

export type SketchpadEvent =
  | { type: "NAVIGATE"; path: string }
  | { type: "NAVIGATE_BACK" }
  | { type: "NAVIGATE_FORWARD" }
  | { type: "SET_THEME"; theme: Theme }
  | { type: "SET_LANGUAGE"; language: string }
  | { type: "SET_EXPERTISE"; expertise: Expertise }
  | { type: "SET_MODE"; mode: Mode }
  | { type: "SET_DEVICE"; device: Device }
  | { type: "TOGGLE_FULLSCREEN" }
  | { type: "SET_PANEL_SIZE"; panel: keyof PanelSizes; size: number }
  | { type: "CREATE_KIT"; kit: Kit; local?: boolean; remote?: boolean }
  | { type: "DELETE_KIT"; guid: Guid }
  | { type: "CHANGE"; diff: SketchpadDiff }
  | { type: "HOME.TOGGLE_PANEL"; panel: keyof PanelVisibility }
  | { type: "HOME.SET_PANEL_VISIBILITY"; panelVisibility: PanelVisibility }
  | { type: "HOME.SET_SORT"; column: string; direction: "asc" | "desc" }
  | { type: "HOME.SELECT_KIT"; guid: Guid }
  | { type: "HOME.DESELECT_KIT"; guid: Guid }
  | { type: "HOME.CLEAR_SELECTION" }
  | { type: "HOME.SET_HOVER"; kits?: Guid[] }
  | { type: "HOME.CLEAR_HOVER" }
  | { type: "KIT.INIT"; kitGuid: Guid; state: KitAppState }
  | { type: "KIT.SYNC"; kitGuid: Guid; state: Partial<KitAppState> }
  | { type: "KIT.TOGGLE_PANEL"; kitGuid: Guid; panel: keyof PanelVisibility }
  | { type: "KIT.SET_PANEL_VISIBILITY"; kitGuid: Guid; panelVisibility: PanelVisibility }
  | { type: "KIT.SET_FILTER"; kitGuid: Guid; search: string }
  | { type: "KIT.TOGGLE_ROW"; kitGuid: Guid; rowId: string }
  | { type: "KIT.SET_EXPANDED_ROWS"; kitGuid: Guid; expandedRows: Set<string> }
  | { type: "KIT.SET_SORT"; kitGuid: Guid; column: string; direction: "asc" | "desc" }
  | { type: "KIT.SELECT_TYPE"; kitGuid: Guid; typeGuid: Guid }
  | { type: "KIT.DESELECT_TYPE"; kitGuid: Guid; typeGuid: Guid }
  | { type: "KIT.SELECT_DESIGN"; kitGuid: Guid; designGuid: Guid }
  | { type: "KIT.DESELECT_DESIGN"; kitGuid: Guid; designGuid: Guid }
  | { type: "KIT.SET_SELECTION"; kitGuid: Guid; selection: KitAppSelection }
  | { type: "KIT.CLEAR_SELECTION"; kitGuid: Guid }
  | { type: "KIT.SET_HOVER"; kitGuid: Guid; hover: any }
  | { type: "KIT.CLEAR_HOVER"; kitGuid: Guid }
  | { type: "KIT.SET_FULLSCREEN"; kitGuid: Guid; window: KitAppFullscreenWindow }
  | { type: "KIT.SET_WINDOW_LAYOUT"; kitGuid: Guid; windowLayout: any }
  | { type: "KIT.SET_DIAGRAM_FORCE"; kitGuid: Guid; diagramForce: Partial<DiagramForceSettings> }
  | { type: "TYPE.INIT"; kitGuid: Guid; typeGuid: Guid; state: TypeAppState }
  | { type: "TYPE.SYNC"; kitGuid: Guid; typeGuid: Guid; state: Partial<TypeAppState> }
  | { type: "TYPE.TOGGLE_PANEL"; kitGuid: Guid; typeGuid: Guid; panel: keyof PanelVisibility }
  | { type: "TYPE.SET_PANEL_VISIBILITY"; kitGuid: Guid; typeGuid: Guid; panelVisibility: PanelVisibility }
  | { type: "TYPE.SET_ACTIVE_TOOL"; kitGuid: Guid; typeGuid: Guid; tool: ToolKind }
  | { type: "TYPE.SET_FULLSCREEN_WINDOW"; kitGuid: Guid; typeGuid: Guid; window: TypeAppFullscreenWindow }
  | { type: "TYPE.SET_SELECTION"; kitGuid: Guid; typeGuid: Guid; selection: TypeAppSelection }
  | { type: "TYPE.CLEAR_SELECTION"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.SELECT_CONNECTOR"; kitGuid: Guid; typeGuid: Guid; connectorGuid: Guid }
  | { type: "TYPE.DESELECT_CONNECTOR"; kitGuid: Guid; typeGuid: Guid; connectorGuid: Guid }
  | { type: "TYPE.SET_HOVER"; kitGuid: Guid; typeGuid: Guid; hover: { connector?: Guid; model?: Guid } }
  | { type: "TYPE.CLEAR_HOVER"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.FOCUS_CONNECTOR"; kitGuid: Guid; typeGuid: Guid; connectorGuid?: Guid }
  | { type: "TYPE.SELECT_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tagGuid: Guid }
  | { type: "TYPE.DESELECT_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tagGuid: Guid }
  | { type: "TYPE.SET_MODEL_TAGS"; kitGuid: Guid; typeGuid: Guid; tags: Guid[] }
  | { type: "TYPE.SET_CAMERA"; kitGuid: Guid; typeGuid: Guid; camera: any }
  | { type: "TYPE.SELECT_ALL"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.DESELECT_ALL"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.CLEAR_FOCUS"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.SELECT_MODEL"; kitGuid: Guid; typeGuid: Guid; modelGuid: Guid }
  | { type: "TYPE.DESELECT_MODEL"; kitGuid: Guid; typeGuid: Guid; modelGuid: Guid }
  | { type: "TYPE.HOVER_CONNECTOR"; kitGuid: Guid; typeGuid: Guid; connectorGuid: Guid }
  | { type: "TYPE.HOVER_MODEL"; kitGuid: Guid; typeGuid: Guid; modelGuid: Guid }
  | { type: "TYPE.SET_SELECTED_MODEL"; kitGuid: Guid; typeGuid: Guid; modelGuid: Guid }
  | { type: "TYPE.ADD_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tag: string }
  | { type: "TYPE.REMOVE_MODEL_TAG"; kitGuid: Guid; typeGuid: Guid; tag: string }
  | { type: "TYPE.CLEAR_MODEL_TAGS"; kitGuid: Guid; typeGuid: Guid }
  | { type: "DESIGN.INIT"; kitGuid: Guid; designGuid: Guid; state: DesignAppState }
  | { type: "DESIGN.SYNC"; kitGuid: Guid; designGuid: Guid; state: Partial<DesignAppState> }
  | { type: "DESIGN.TOGGLE_PANEL"; kitGuid: Guid; designGuid: Guid; panel: keyof PanelVisibility }
  | { type: "DESIGN.SET_PANEL_VISIBILITY"; kitGuid: Guid; designGuid: Guid; panelVisibility: PanelVisibility }
  | { type: "DESIGN.SET_ACTIVE_TOOL"; kitGuid: Guid; designGuid: Guid; tool: ToolKind }
  | { type: "DESIGN.SET_FULLSCREEN"; kitGuid: Guid; designGuid: Guid; window: DesignAppFullscreenWindow }
  | { type: "DESIGN.SELECT_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid }
  | { type: "DESIGN.DESELECT_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid: Guid }
  | { type: "DESIGN.SELECT_CONNECTION"; kitGuid: Guid; designGuid: Guid; connectionGuid: Guid }
  | { type: "DESIGN.DESELECT_CONNECTION"; kitGuid: Guid; designGuid: Guid; connectionGuid: Guid }
  | { type: "DESIGN.SET_SELECTION"; kitGuid: Guid; designGuid: Guid; selection: DesignAppSelection }
  | { type: "DESIGN.CLEAR_SELECTION"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.SET_HOVER"; kitGuid: Guid; designGuid: Guid; hover: DesignAppHover }
  | { type: "DESIGN.CLEAR_HOVER"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.FOCUS_PIECE"; kitGuid: Guid; designGuid: Guid; pieceGuid?: Guid }
  | { type: "DESIGN.SELECT_MODEL_TAG"; kitGuid: Guid; designGuid: Guid; typeGuid: Guid; tagGuid: Guid }
  | { type: "DESIGN.DESELECT_MODEL_TAG"; kitGuid: Guid; designGuid: Guid; typeGuid: Guid; tagGuid: Guid }
  | { type: "DESIGN.SET_DIAGRAM_CENTER"; kitGuid: Guid; designGuid: Guid; center: { x: number; y: number } }
  | { type: "DESIGN.SET_DIAGRAM_SCALE"; kitGuid: Guid; designGuid: Guid; scale: number }
  | { type: "DESIGN.SET_CAMERA"; kitGuid: Guid; designGuid: Guid; camera: any }
  | { type: "DESIGN.SELECT_ALL"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.DELETE_SELECTED"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.START"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.COMMIT"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.ABORT"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.UNDO"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.REDO"; kitGuid: Guid; designGuid: Guid }
  | { type: "DESIGN.TRANSACTION.RECORD_EDIT"; kitGuid: Guid; designGuid: Guid; edit: any }
  | { type: "TYPE.TRANSACTION.START"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.TRANSACTION.COMMIT"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.TRANSACTION.ABORT"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.TRANSACTION.UNDO"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.TRANSACTION.REDO"; kitGuid: Guid; typeGuid: Guid }
  | { type: "TYPE.TRANSACTION.RECORD_EDIT"; kitGuid: Guid; typeGuid: Guid; edit: any }
  | { type: "KIT.TRANSACTION.START"; kitGuid: Guid }
  | { type: "KIT.TRANSACTION.COMMIT"; kitGuid: Guid }
  | { type: "KIT.TRANSACTION.ABORT"; kitGuid: Guid }
  | { type: "KIT.TRANSACTION.UNDO"; kitGuid: Guid }
  | { type: "KIT.TRANSACTION.REDO"; kitGuid: Guid }
  | { type: "KIT.TRANSACTION.RECORD_EDIT"; kitGuid: Guid; edit: any }
  | { type: "BACKGROUND.START"; operationId: string; operationType: string }
  | { type: "BACKGROUND.COMPLETE"; operationId: string }
  | { type: "BACKGROUND.FAIL"; operationId: string; error: string }
  | { type: "QUALITY.TOGGLE_PANEL"; kitGuid: Guid; qualityGuid: Guid; panel: keyof PanelVisibility }
  | { type: "QUALITY.TOGGLE_BENCHMARK"; kitGuid: Guid; qualityGuid: Guid; benchmarkGuid: Guid }
  | { type: "TUTORIAL.START"; tutorialId: string; steps: TutorialStep[] }
  | { type: "TUTORIAL.END" }
  | { type: "TUTORIAL.NEXT_STEP" }
  | { type: "TUTORIAL.PREV_STEP" }
  | { type: "TUTORIAL.GO_TO_STEP"; index: number }
  | { type: "TUTORIAL.COMPLETE_STEP"; stepId: string }
  | { type: "FEEDBACK.TOGGLE_PANEL"; panel: keyof PanelVisibility }
  | { type: "FEEDBACK.SET_FORM_DATA"; data: Partial<FeedbackFormData> }
  | { type: "FEEDBACK.RESET_FORM" }
  | { type: "FEEDBACK.SET_SUBMITTING"; isSubmitting: boolean }
  | { type: "FEEDBACK.SET_SUBMITTED"; isSubmitted: boolean }
  | { type: "FEEDBACK.SET_ERROR"; error: string | undefined };

// #endregion 🔖Types

// #region 🔖Helpers

function migratePath(path: string): string {
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

function buildSnapshot(ySketchpad: Y.Map<any>): SketchpadState {
  const settingsStr = ySketchpad.get("settings") as string;
  const settings = settingsStr
    ? JSON.parse(settingsStr)
    : {
        apps: {
          design: {
            diagram: { proximityConnectDistance: 10 },
            scene: { gridSize: 24 },
          },
        },
      };

  const panelSizesStr = ySketchpad.get("panelSizes") as string;
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

  const navigationHistoryStr = ySketchpad.get("navigationHistory") as string;
  const navigationHistory = navigationHistoryStr ? JSON.parse(navigationHistoryStr).map(migratePath) : ["/"];

  const recentSearchesStr = ySketchpad.get("recentSearches") as string;
  const recentSearches = recentSearchesStr ? JSON.parse(recentSearchesStr) : [];

  const recentFocusItemsStr = ySketchpad.get("recentFocusItems") as string;
  const recentFocusItems = recentFocusItemsStr ? JSON.parse(recentFocusItemsStr) : {};

  const hotkeyOverridesStr = ySketchpad.get("hotkeyOverrides") as string;
  const hotkeyOverrides = hotkeyOverridesStr ? JSON.parse(hotkeyOverridesStr) : {};

  const deviceStr = ySketchpad.get("device") as string;
  const device: Device = deviceStr ? JSON.parse(deviceStr) : "desktop";

  return {
    navigation: migratePath((ySketchpad.get("navigation") as string) || "/"),
    navigationHistory,
    navigationHistoryIndex: (ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
    recentSearches,
    recentFocusItems,
    theme: ySketchpad.get("theme") as Theme,
    language: (ySketchpad.get("language") as string) || "en",
    device,
    expertise: (ySketchpad.get("expertise") as Expertise) ?? Expertise.BEGINNER,
    mode: (ySketchpad.get("mode") as Mode) ?? Mode.USER,
    settings,
    panelSizes,
    isFullscreen: (ySketchpad.get("isFullscreen") as boolean) || false,
    isMobile: (ySketchpad.get("isMobile") as boolean) || false,
    activeInteraction: (ySketchpad.get("activeInteraction") as string) || undefined,
    hotkeyOverrides,
    activeHotkeySetting: (ySketchpad.get("activeHotkeySetting") as string) || undefined,
  };
}

function applyDiff(yDoc: Y.Doc, ySketchpad: Y.Map<any>, diff: SketchpadDiff): void {
  yDoc.transact(() => {
    if (diff.navigationHistory !== undefined) {
      ySketchpad.set("navigationHistory", JSON.stringify(diff.navigationHistory));
    }
    if (diff.navigationHistoryIndex !== undefined) {
      ySketchpad.set("navigationHistoryIndex", diff.navigationHistoryIndex);
    }
    if (diff.navigation) {
      ySketchpad.set("navigation", diff.navigation);
    }
    if ("recentSearches" in diff) {
      ySketchpad.set("recentSearches", JSON.stringify(diff.recentSearches || []));
    }
    if ("recentFocusItems" in diff) {
      const current = JSON.parse((ySketchpad.get("recentFocusItems") as string) || "{}");
      ySketchpad.set("recentFocusItems", JSON.stringify({ ...current, ...(diff.recentFocusItems || {}) }));
    }
    if (diff.theme) ySketchpad.set("theme", diff.theme);
    if (diff.language !== undefined) {
      ySketchpad.set("language", diff.language);
    }
    if (diff.device) ySketchpad.set("device", JSON.stringify(diff.device));
    if (diff.expertise) ySketchpad.set("expertise", diff.expertise);
    if (diff.mode) ySketchpad.set("mode", diff.mode);
    if (diff.isFullscreen !== undefined) ySketchpad.set("isFullscreen", diff.isFullscreen);
    if (diff.isMobile !== undefined) ySketchpad.set("isMobile", diff.isMobile);
    if ("activeInteraction" in diff) ySketchpad.set("activeInteraction", diff.activeInteraction || "");
    if (diff.settings) {
      const current = JSON.parse((ySketchpad.get("settings") as string) || "{}");
      const merged = { ...current, apps: { ...current.apps, ...diff.settings.apps } };
      ySketchpad.set("settings", JSON.stringify(merged));
    }
    if (diff.panelSizes) {
      const current = JSON.parse((ySketchpad.get("panelSizes") as string) || "{}");
      ySketchpad.set("panelSizes", JSON.stringify({ ...current, ...diff.panelSizes }));
    }
    if (diff.hotkeyOverrides) {
      const current = JSON.parse((ySketchpad.get("hotkeyOverrides") as string) || "{}");
      ySketchpad.set("hotkeyOverrides", JSON.stringify({ ...current, ...diff.hotkeyOverrides }));
    }
    if ("activeHotkeySetting" in diff) {
      ySketchpad.set("activeHotkeySetting", diff.activeHotkeySetting || "");
    }
  });
}

export function createDefaultTransactionState(): AppTransactionState {
  return {
    isTransactionActive: false,
    currentTransactionStack: [],
    pastTransactionStack: [],
    redoStack: [],
  };
}

export function createDefaultDesignAppState(): DesignAppState {
  return {
    panelVisibility: { ...defaultPanelVisibility, toolbar: true },
    selection: undefined,
    hover: undefined,
    focusedPiece: undefined,
    selectedModelTags: {},
    diagramCenter: undefined,
    diagramScale: undefined,
    camera: undefined,
    activeTool: undefined,
    fullscreenWindow: undefined,
    transaction: createDefaultTransactionState(),
  };
}

export function createDefaultTypeAppState(): TypeAppState {
  return {
    panelVisibility: { ...defaultPanelVisibility, toolbar: true },
    activeTool: ToolKind.SELECTION_NORMAL,
    fullscreenWindow: TypeAppFullscreenWindow.None,
    selection: undefined,
    hover: undefined,
    focusedConnector: undefined,
    selectedModelTags: [],
    selectedModelGuid: undefined,
    camera: undefined,
    windowLayout: undefined,
    transaction: createDefaultTransactionState(),
  };
}

export function createDefaultKitAppState(): KitAppState {
  return {
    panelVisibility: { ...defaultPanelVisibility, toolbar: true },
    selection: undefined,
    hover: undefined,
    fullscreenWindow: KitAppFullscreenWindow.None,
    others: [],
    filterSearch: undefined,
    expandedRows: new Set<string>(),
    sortColumn: undefined,
    sortDirection: undefined,
    diagramForce: { ...defaultDiagramForceSettings },
    transaction: createDefaultTransactionState(),
  };
}

export function createDefaultQualityAppState(): QualityAppState {
  return {
    panelVisibility: { ...defaultPanelVisibility, toolbar: true },
    selection: undefined,
    hover: undefined,
    expandedBenchmarks: new Set<string>(),
    transaction: createDefaultTransactionState(),
  };
}

function createDefaultSketchpadState(id?: string): SketchpadState {
  return {
    id,
    navigation: "/",
    navigationHistory: ["/"],
    navigationHistoryIndex: 0,
    recentSearches: [],
    recentFocusItems: {},
    theme: Theme.SYSTEM,
    language: "en",
    device: "desktop",
    expertise: Expertise.BEGINNER,
    mode: Mode.USER,
    settings: {
      apps: {
        design: {
          diagram: { proximityConnectDistance: 10 },
          scene: { gridSize: 24 },
        },
      },
    },
    panelSizes: {
      toolbarHeight: 52,
      workbenchWidth: 230,
      toolsWidth: 230,
      hudWidth: 230,
      statsWidth: 230,
      detailsWidth: 230,
      chatWidth: 230,
      settingsWidth: 230,
      consoleHeight: 200,
      leftSidePanelWidth: 280,
      rightSidePanelWidth: 280,
      hudPanelWidth: 400,
    },
    isFullscreen: false,
    isMobile: false,
    activeInteraction: undefined,
    hotkeyOverrides: undefined,
    activeHotkeySetting: undefined,
    persisted: false,
  };
}

function mergeSketchpadState(base: SketchpadState, partial?: Partial<SketchpadState>): SketchpadState {
  if (!partial) return base;
  return {
    ...base,
    ...partial,
    settings: {
      ...base.settings,
      ...partial.settings,
      apps: {
        ...(base.settings?.apps || {}),
        ...(partial.settings?.apps || {}),
      },
    },
    panelSizes: {
      ...base.panelSizes,
      ...(partial.panelSizes || {}),
    },
    navigationHistory: partial.navigationHistory ?? base.navigationHistory,
    recentSearches: partial.recentSearches ?? base.recentSearches,
    recentFocusItems: partial.recentFocusItems ?? base.recentFocusItems,
    hotkeyOverrides: partial.hotkeyOverrides ?? base.hotkeyOverrides,
  };
}

function readSketchpadStateFromLocalStorage(id: string): Partial<SketchpadState> | undefined {
  if (typeof window === "undefined") return undefined;
  try {
    const raw = window.localStorage.getItem(`semio.sketchpad.state.${id}`);
    if (!raw) return undefined;
    return JSON.parse(raw) as Partial<SketchpadState>;
  } catch {
    return undefined;
  }
}

function writeSketchpadStateToLocalStorage(id: string, state: SketchpadState): void {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(`semio.sketchpad.state.${id}`, JSON.stringify(state));
  } catch {}
}

function toSketchpadInitialState(initialState?: ExtendedInitialState): Partial<SketchpadState> | undefined {
  if (!initialState) return undefined;
  const { kits: _kits, ...rest } = initialState;
  return rest;
}

function applySketchpadDiffToState(state: SketchpadState, diff: SketchpadDiff): SketchpadState {
  let next = state;
  if (diff.navigationHistory !== undefined) next = { ...next, navigationHistory: diff.navigationHistory };
  if (diff.navigationHistoryIndex !== undefined) next = { ...next, navigationHistoryIndex: diff.navigationHistoryIndex };
  if (diff.navigation) next = { ...next, navigation: diff.navigation };
  if ("recentSearches" in diff) next = { ...next, recentSearches: diff.recentSearches || [] };
  if ("recentFocusItems" in diff) next = { ...next, recentFocusItems: { ...(next.recentFocusItems || {}), ...(diff.recentFocusItems || {}) } };
  if (diff.theme) next = { ...next, theme: diff.theme };
  if (diff.language !== undefined) next = { ...next, language: diff.language };
  if (diff.device) next = { ...next, device: diff.device };
  if (diff.expertise) next = { ...next, expertise: diff.expertise };
  if (diff.mode) next = { ...next, mode: diff.mode };
  if (diff.isFullscreen !== undefined) next = { ...next, isFullscreen: diff.isFullscreen };
  if (diff.isMobile !== undefined) next = { ...next, isMobile: diff.isMobile };
  if ("activeInteraction" in diff) next = { ...next, activeInteraction: diff.activeInteraction || undefined };
  if (diff.settings) {
    next = {
      ...next,
      settings: {
        ...next.settings,
        apps: {
          ...(next.settings?.apps || {}),
          ...(diff.settings.apps || {}),
        },
      },
    };
  }
  if (diff.panelSizes) next = { ...next, panelSizes: { ...(next.panelSizes || {}), ...diff.panelSizes } };
  if (diff.hotkeyOverrides) next = { ...next, hotkeyOverrides: { ...(next.hotkeyOverrides || {}), ...diff.hotkeyOverrides } };
  if ("activeHotkeySetting" in diff) next = { ...next, activeHotkeySetting: diff.activeHotkeySetting || undefined };
  return next;
}

// #endregion 🔖Helpers

// #region 🔖Sketchpad Machine

export const sketchpadMachine = setup({
  types: {
    context: {} as SketchpadContext,
    events: {} as SketchpadEvent,
    input: {} as SketchpadMachineInput,
  },
  guards: {
    canNavigateBack: ({ context }) => {
      return context.sketchpad.navigationHistoryIndex > 0;
    },
    canNavigateForward: ({ context }) => {
      return context.sketchpad.navigationHistoryIndex < context.sketchpad.navigationHistory.length - 1;
    },

    hasHomeHover: ({ context }) => {
      const hover = context.homeApp.hover;
      return hover !== undefined && (hover.kits?.length ?? 0) > 0;
    },

    hasDesignHover: ({ context, event }) => {
      const { kitGuid, designGuid } = event as any;
      const key = `${kitGuid}:${designGuid}`;
      const app = context.designApps[key];
      if (!app?.hover) return false;
      return (app.hover.pieces?.length ?? 0) > 0 || (app.hover.connections?.length ?? 0) > 0 || (app.hover.connectors?.length ?? 0) > 0 || (app.hover.types?.length ?? 0) > 0 || (app.hover.designs?.length ?? 0) > 0;
    },

    hasTypeHover: ({ context, event }) => {
      const { kitGuid, typeGuid } = event as any;
      const key = `${kitGuid}:${typeGuid}`;
      const app = context.typeApps[key];
      if (!app?.hover) return false;
      return app.hover.connector !== undefined || app.hover.model !== undefined;
    },

    hasKitHover: ({ context, event }) => {
      const { kitGuid } = event as any;
      const app = context.kitApps[kitGuid];
      return app?.hover !== undefined;
    },

    hasDesignSelection: ({ context, event }) => {
      const { kitGuid, designGuid } = event as any;
      const key = `${kitGuid}:${designGuid}`;
      const app = context.designApps[key];
      if (!app?.selection) return false;
      return (app.selection.pieces?.length ?? 0) > 0 || (app.selection.connections?.length ?? 0) > 0 || (app.selection.connectors?.length ?? 0) > 0;
    },
    hasTypeSelection: ({ context, event }) => {
      const { kitGuid, typeGuid } = event as any;
      const key = `${kitGuid}:${typeGuid}`;
      const app = context.typeApps[key];
      if (!app?.selection) return false;
      return (app.selection.connectors?.length ?? 0) > 0 || (app.selection.models?.length ?? 0) > 0;
    },

    designAppExists: ({ context, event }) => {
      const { kitGuid, designGuid } = event as any;
      const key = `${kitGuid}:${designGuid}`;
      return !!context.designApps[key];
    },
    typeAppExists: ({ context, event }) => {
      const { kitGuid, typeGuid } = event as any;
      const key = `${kitGuid}:${typeGuid}`;
      return !!context.typeApps[key];
    },
    kitAppExists: ({ context, event }) => {
      const { kitGuid } = event as any;
      return !!context.kitApps[kitGuid];
    },
  },
  actions: {
    navigate: () => {},
    navigateImpl: assign(({ context, event }) => {
      if (event.type !== "NAVIGATE") return {};
      const currentNav = context.sketchpad.navigation;
      const history = context.sketchpad.navigationHistory.length > 0 ? context.sketchpad.navigationHistory : ["/"];
      const index = context.sketchpad.navigationHistoryIndex ?? 0;
      if (currentNav === event.path) {
        return { sketchpad: { ...context.sketchpad, navigation: event.path } };
      }
      const newHistory = [...history.slice(0, index + 1), event.path];
      return { sketchpad: { ...context.sketchpad, navigation: event.path, navigationHistory: newHistory, navigationHistoryIndex: newHistory.length - 1 } };
    }),
    navigateBack: assign(({ context }) => {
      const history = context.sketchpad.navigationHistory.length > 0 ? context.sketchpad.navigationHistory : ["/"];
      const index = context.sketchpad.navigationHistoryIndex ?? 0;
      if (index <= 0) return {};
      const newIndex = index - 1;
      return { sketchpad: { ...context.sketchpad, navigation: history[newIndex], navigationHistoryIndex: newIndex } };
    }),
    navigateForward: assign(({ context }) => {
      const history = context.sketchpad.navigationHistory.length > 0 ? context.sketchpad.navigationHistory : ["/"];
      const index = context.sketchpad.navigationHistoryIndex ?? 0;
      if (index >= history.length - 1) return {};
      const newIndex = index + 1;
      return { sketchpad: { ...context.sketchpad, navigation: history[newIndex], navigationHistoryIndex: newIndex } };
    }),
    setTheme: assign(({ context, event }) => {
      if (event.type !== "SET_THEME") return {};
      return { sketchpad: { ...context.sketchpad, theme: event.theme } };
    }),
    setLanguage: assign(({ context, event }) => {
      if (event.type !== "SET_LANGUAGE") return {};
      return { sketchpad: { ...context.sketchpad, language: event.language } };
    }),
    setExpertise: assign(({ context, event }) => {
      if (event.type !== "SET_EXPERTISE") return {};
      return { sketchpad: { ...context.sketchpad, expertise: event.expertise } };
    }),
    setMode: assign(({ context, event }) => {
      if (event.type !== "SET_MODE") return {};
      return { sketchpad: { ...context.sketchpad, mode: event.mode } };
    }),
    setDevice: assign(({ context, event }) => {
      if (event.type !== "SET_DEVICE") return {};
      return { sketchpad: { ...context.sketchpad, device: event.device } };
    }),
    toggleFullscreen: assign(({ context }) => ({ sketchpad: { ...context.sketchpad, isFullscreen: !context.sketchpad.isFullscreen } })),
    setPanelSize: assign(({ context, event }) => {
      if (event.type !== "SET_PANEL_SIZE") return {};
      return { sketchpad: { ...context.sketchpad, panelSizes: { ...(context.sketchpad.panelSizes || {}), [event.panel]: event.size } } };
    }),
    applyChange: assign(({ context, event }) => {
      if (event.type !== "CHANGE") return {};
      return { sketchpad: applySketchpadDiffToState(context.sketchpad, event.diff) };
    }),
    markDirty: () => {},

    dispatchAppEvent: assign(({ context, event }) => executeEventHandler(context, event)),

    typeInit: assign(({ context, event }) => executeEventHandler(context, event)),

    designInit: assign(({ context, event }) => executeEventHandler(context, event)),

    kitInit: assign(({ context, event }) => executeEventHandler(context, event)),

    tutorialStart: assign(({ context, event }) => {
      if (event.type !== "TUTORIAL.START") return {};
      return {
        tutorial: {
          ...context.tutorial,
          activeTutorial: event.tutorialId,
          steps: event.steps,
          currentStepIndex: 0,
        },
      };
    }),
    tutorialEnd: assign(({ context }) => ({
      tutorial: {
        ...context.tutorial,
        activeTutorial: undefined,
        steps: [],
        currentStepIndex: 0,
      },
    })),
    tutorialNextStep: assign(({ context }) => ({
      tutorial: {
        ...context.tutorial,
        currentStepIndex: Math.min(context.tutorial.currentStepIndex + 1, context.tutorial.steps.length - 1),
      },
    })),
    tutorialPrevStep: assign(({ context }) => ({
      tutorial: {
        ...context.tutorial,
        currentStepIndex: Math.max(context.tutorial.currentStepIndex - 1, 0),
      },
    })),
    tutorialGoToStep: assign(({ context, event }) => {
      if (event.type !== "TUTORIAL.GO_TO_STEP") return {};
      return {
        tutorial: {
          ...context.tutorial,
          currentStepIndex: Math.max(0, Math.min(event.index, context.tutorial.steps.length - 1)),
        },
      };
    }),
    tutorialCompleteStep: assign(({ context, event }) => {
      if (event.type !== "TUTORIAL.COMPLETE_STEP") return {};
      const completed = new Set(context.tutorial.completedSteps);
      completed.add(event.stepId);
      return {
        tutorial: { ...context.tutorial, completedSteps: completed },
      };
    }),

    backgroundStart: assign(({ context, event }) => {
      if (event.type !== "BACKGROUND.START") return {};
      const updates: Partial<SketchpadContext> = {
        backgroundOperations: {
          ...context.backgroundOperations,
          [event.operationId]: { type: event.operationType, status: "running" as const },
        },
      };

      if (event.operationType.startsWith("kit-import:")) {
        const kitName = event.operationType.replace("kit-import:", "");
        updates.homeApp = {
          ...context.homeApp,
          loadingKits: [...context.homeApp.loadingKits, { tempGuid: event.operationId, name: kitName }],
        };
      }
      return updates;
    }),
    backgroundComplete: assign(({ context, event }) => {
      if (event.type !== "BACKGROUND.COMPLETE") return {};
      const operation = context.backgroundOperations[event.operationId];
      const { [event.operationId]: _, ...rest } = context.backgroundOperations;
      const updates: Partial<SketchpadContext> = { backgroundOperations: rest };

      if (operation?.type.startsWith("kit-import:")) {
        updates.homeApp = {
          ...context.homeApp,
          loadingKits: context.homeApp.loadingKits.filter((k) => k.tempGuid !== event.operationId),
        };
      }
      return updates;
    }),
    backgroundFail: assign(({ context, event }) => {
      if (event.type !== "BACKGROUND.FAIL") return {};
      const operation = context.backgroundOperations[event.operationId];
      const updates: Partial<SketchpadContext> = {
        backgroundOperations: {
          ...context.backgroundOperations,
          [event.operationId]: { ...context.backgroundOperations[event.operationId], status: "failed" as const, error: event.error },
        },
      };

      if (operation?.type.startsWith("kit-import:")) {
        updates.homeApp = {
          ...context.homeApp,
          loadingKits: context.homeApp.loadingKits.filter((k) => k.tempGuid !== event.operationId),
        };
      }
      return updates;
    }),
  },
}).createMachine({
  id: "sketchpad",
  type: "parallel",
  context: ({ input }) => ({
    id: input.id,
    sketchpad: mergeSketchpadState(createDefaultSketchpadState(input.id), input.initialState),
    kits: {},
    homeApp: {
      panelVisibility: { ...defaultPanelVisibility, toolbar: true },
      selection: undefined,
      hover: undefined,
      sortColumn: undefined,
      sortDirection: undefined,
      loadingKits: [],
    },
    kitApps: {},
    typeApps: {},
    designApps: {},
    qualityApps: {},
    feedbackApp: {
      panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
      formData: {
        kind: "bug" as const,
        title: "",
        description: "",
        app: undefined,
        name: undefined,
        email: undefined,
      },
      isSubmitting: false,
      isSubmitted: false,
      error: undefined,
    },
    tutorial: {
      activeTutorial: undefined,
      currentStepIndex: 0,
      steps: [],
      completedSteps: new Set<string>(),
      isRecording: false,
      recordingState: "idle" as const,
      recordedEvents: [],
    },

    backgroundOperations: {},
  }),
  on: {
    NAVIGATE: {
      actions: ["navigate", "navigateImpl"],
    },
    NAVIGATE_BACK: {
      guard: "canNavigateBack",
      actions: ["navigateBack"],
    },
    NAVIGATE_FORWARD: {
      guard: "canNavigateForward",
      actions: ["navigateForward"],
    },

    "KIT.INIT": {
      target: ".navigation.kit",
      actions: "kitInit",
    },
    "DESIGN.INIT": {
      target: ".navigation.design",
      actions: "designInit",
    },
    "TYPE.INIT": {
      target: ".navigation.type",
      actions: "typeInit",
    },

    SET_THEME: {
      actions: ["setTheme"],
    },
    SET_LANGUAGE: {
      actions: ["setLanguage"],
    },
    SET_EXPERTISE: {
      actions: ["setExpertise"],
    },
    SET_MODE: {
      actions: ["setMode"],
    },
    SET_DEVICE: {
      actions: ["setDevice"],
    },
    TOGGLE_FULLSCREEN: {
      actions: ["toggleFullscreen"],
    },
    SET_PANEL_SIZE: {
      actions: ["setPanelSize"],
    },
    CHANGE: {
      actions: ["applyChange"],
    },

    "TUTORIAL.START": { actions: "tutorialStart" },
    "TUTORIAL.END": { actions: "tutorialEnd" },
    "TUTORIAL.NEXT_STEP": { actions: "tutorialNextStep" },
    "TUTORIAL.PREV_STEP": { actions: "tutorialPrevStep" },
    "TUTORIAL.GO_TO_STEP": { actions: "tutorialGoToStep" },
    "TUTORIAL.COMPLETE_STEP": { actions: "tutorialCompleteStep" },

    "BACKGROUND.START": { actions: "backgroundStart" },
    "BACKGROUND.COMPLETE": { actions: "backgroundComplete" },
    "BACKGROUND.FAIL": { actions: "backgroundFail" },

    "*": { actions: "dispatchAppEvent" },
  },
  states: {
    navigation: {
      initial: "home",
      states: {
        home: {},

        kit: {},

        design: {},

        type: {},

        quality: {},

        docs: {},
      },
    },
  },
});

// #region 🔖Sketchpad Selectors

export type NavigationState = "home" | "kit" | "design" | "type" | "quality" | "docs";
export const selectNavigationState = (state: { value: any }): NavigationState => {
  const value = state.value;
  if (typeof value === "object" && "navigation" in value) {
    const nav = value.navigation;
    if (typeof nav === "string") return nav as NavigationState;
    if (typeof nav === "object") {
      return Object.keys(nav)[0] as NavigationState;
    }
  }
  return "home";
};
export const selectIsInHome = (state: { value: any }): boolean => selectNavigationState(state) === "home";
export const selectIsInKit = (state: { value: any }): boolean => selectNavigationState(state) === "kit";
export const selectIsInDesign = (state: { value: any }): boolean => selectNavigationState(state) === "design";
export const selectIsInType = (state: { value: any }): boolean => selectNavigationState(state) === "type";
export const selectIsInQuality = (state: { value: any }): boolean => selectNavigationState(state) === "quality";
export const selectIsInDocs = (state: { value: any }): boolean => selectNavigationState(state) === "docs";

export const selectHomeApp = (state: { context: SketchpadContext }) => state.context.homeApp;
export const selectHomePanelVisibility = (state: { context: SketchpadContext }) => state.context.homeApp.panelVisibility;
export const selectHomeSelection = (state: { context: SketchpadContext }) => state.context.homeApp.selection;
export const selectHomeHover = (state: { context: SketchpadContext }) => state.context.homeApp.hover;
export const selectHomeSortColumn = (state: { context: SketchpadContext }) => state.context.homeApp.sortColumn;
export const selectHomeSortDirection = (state: { context: SketchpadContext }) => state.context.homeApp.sortDirection;
export const selectHomeLoadingKits = (state: { context: SketchpadContext }) => state.context.homeApp.loadingKits;

export const selectBackgroundOperations = (state: { context: SketchpadContext }) => state.context.backgroundOperations;
export const selectKitImportOperations = (state: { context: SketchpadContext }) => {
  const ops = state.context.backgroundOperations;
  return Object.entries(ops)
    .filter(([_, op]) => op.type.startsWith("kit-import:"))
    .map(([id, op]) => ({
      operationId: id,
      kitName: op.type.replace("kit-import:", ""),
      status: op.status,
      error: op.error,
    }));
};

export const createDesignAppSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key] || createDefaultDesignAppState();
};

export const createDesignPanelVisibilitySelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.panelVisibility ?? defaultPanelVisibility;
};

export const createDesignSelectionSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.selection;
};

export const createDesignHoverSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.hover;
};

export const createDesignFocusedPieceSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.focusedPiece;
};

export const createDesignSelectedModelTagsSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.selectedModelTags ?? {};
};

export const createDesignDiagramCenterSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.diagramCenter;
};

export const createDesignDiagramScaleSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.diagramScale;
};

export const createDesignCameraSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.camera;
};

export const createDesignActiveToolSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.activeTool;
};

export const createDesignFullscreenWindowSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.fullscreenWindow;
};

export const createDesignOthersSelector = (kitGuid: Guid, designGuid: Guid) => {
  const key = `${kitGuid}:${designGuid}`;
  return (state: { context: SketchpadContext }) => state.context.designApps[key]?.others ?? [];
};

export const createTypeAppSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => {
    const app = state.context.typeApps[key];
    return app ?? createDefaultTypeAppState();
  };
};

export const createTypePanelVisibilitySelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.panelVisibility ?? defaultPanelVisibility;
};

export const createTypeSelectionSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.selection;
};

export const createTypeFocusedConnectorSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.focusedConnector;
};

export const createTypeSelectedModelTagsSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.selectedModelTags ?? [];
};

export const createTypeCameraSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.camera;
};

export const createTypeActiveToolSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.activeTool ?? ToolKind.SELECTION_NORMAL;
};

export const createTypeHoverSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.hover;
};

export const createTypeFullscreenWindowSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.fullscreenWindow ?? TypeAppFullscreenWindow.None;
};

export const createTypeOthersSelector = (kitGuid: Guid, typeGuid: Guid) => {
  const key = `${kitGuid}:${typeGuid}`;
  return (state: { context: SketchpadContext }) => state.context.typeApps[key]?.others ?? [];
};

export const createKitAppSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid] ?? createDefaultKitAppState();
};

export const createKitPanelVisibilitySelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.panelVisibility ?? defaultPanelVisibility;
};

export const createKitSelectionSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.selection;
};

export const createKitHoverSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.hover;
};

export const createKitFilterSearchSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.filterSearch ?? "";
};

export const createKitExpandedRowsSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.expandedRows ?? new Set<string>();
};

export const createKitSortColumnSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.sortColumn ?? "artifact";
};

export const createKitSortDirectionSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.sortDirection ?? "asc";
};

export const createKitFullscreenSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.fullscreenWindow ?? KitAppFullscreenWindow.None;
};

export const createKitOthersSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.others ?? [];
};

export const createKitWindowLayoutSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.windowLayout;
};

export const createKitDiagramForceSelector = (kitGuid: Guid) => {
  return (state: { context: SketchpadContext }) => state.context.kitApps[kitGuid]?.diagramForce;
};

export const createQualityAppSelector = (kitGuid: Guid, qualityGuid: Guid) => {
  const key = `${kitGuid}:${qualityGuid}`;
  return (state: { context: SketchpadContext }) => {
    const app = state.context.qualityApps[key];
    if (!app) {
      return {
        panelVisibility: defaultPanelVisibility,
        selection: undefined,
        hover: undefined,
        expandedBenchmarks: new Set<string>(),
      } as QualityAppState;
    }
    return app;
  };
};

export const createQualityPanelVisibilitySelector = (kitGuid: Guid, qualityGuid: Guid) => {
  const key = `${kitGuid}:${qualityGuid}`;
  return (state: { context: SketchpadContext }) => state.context.qualityApps[key]?.panelVisibility ?? defaultPanelVisibility;
};

export const selectTutorial = (state: { context: SketchpadContext }) => state.context.tutorial;
export const selectActiveTutorial = (state: { context: SketchpadContext }) => state.context.tutorial.activeTutorial;
export const selectTutorialCurrentStep = (state: { context: SketchpadContext }) => state.context.tutorial.currentStepIndex;
export const selectTutorialSteps = (state: { context: SketchpadContext }) => state.context.tutorial.steps;

export const selectSketchpadKits = (state: { context: SketchpadContext }) => state.context.kits;

export const selectSketchpadState = (state: { context: SketchpadContext }) => state.context.sketchpad;

export const selectSketchpadNavigation = (state: { context: SketchpadContext }) => migratePath(state.context.sketchpad.navigation || "/");
export const selectSketchpadTheme = (state: { context: SketchpadContext }) => state.context.sketchpad.theme;
export const selectSketchpadLanguage = (state: { context: SketchpadContext }) => state.context.sketchpad.language || "en";
export const selectSketchpadExpertise = (state: { context: SketchpadContext }) => state.context.sketchpad.expertise ?? Expertise.BEGINNER;
export const selectSketchpadMode = (state: { context: SketchpadContext }) => state.context.sketchpad.mode ?? Mode.USER;
export const selectSketchpadDevice = (state: { context: SketchpadContext }) => state.context.sketchpad.device || "desktop";
export const selectSketchpadIsFullscreen = (state: { context: SketchpadContext }) => state.context.sketchpad.isFullscreen || false;
export const selectSketchpadPanelSizes = (state: { context: SketchpadContext }) => state.context.sketchpad.panelSizes || createDefaultSketchpadState().panelSizes;
export const selectSketchpadNavigationHistory = (state: { context: SketchpadContext }) => (state.context.sketchpad.navigationHistory || ["/"]).map(migratePath);
export const selectSketchpadNavigationHistoryIndex = (state: { context: SketchpadContext }) => state.context.sketchpad.navigationHistoryIndex ?? 0;
export const selectSketchpadSettings = (state: { context: SketchpadContext }) => state.context.sketchpad.settings || createDefaultSketchpadState().settings;

const getAppTransaction = (context: SketchpadContext, appKey: string): AppTransactionState | undefined => {
  const parts = appKey.split("-");
  if (parts[0] === "design" && parts.length >= 3) {
    const key = `${parts[1]}:${parts.slice(2).join("-")}`;
    return context.designApps[key]?.transaction;
  }
  if (parts[0] === "type" && parts.length >= 3) {
    const key = `${parts[1]}:${parts.slice(2).join("-")}`;
    return context.typeApps[key]?.transaction;
  }
  if (parts[0] === "kit" && parts.length >= 2) {
    return context.kitApps[parts[1]]?.transaction;
  }
  return undefined;
};

const defaultTransactionState: AppTransactionState = {
  isTransactionActive: false,
  currentTransactionStack: [],
  pastTransactionStack: [],
  redoStack: [],
};

export const createTransactionSelector = (appKey: string) => (state: { context: SketchpadContext }) => getAppTransaction(state.context, appKey) || defaultTransactionState;

export const createTransactionIsActiveSelector = (appKey: string) => (state: { context: SketchpadContext }) => getAppTransaction(state.context, appKey)?.isTransactionActive ?? false;

export const createTransactionCanUndoSelector = (appKey: string) => (state: { context: SketchpadContext }) => {
  const tx = getAppTransaction(state.context, appKey);
  if (!tx) return false;
  return tx.isTransactionActive ? tx.currentTransactionStack.length > 0 : tx.pastTransactionStack.length > 0;
};

export const createTransactionCanRedoSelector = (appKey: string) => (state: { context: SketchpadContext }) => {
  const tx = getAppTransaction(state.context, appKey);
  if (!tx) return false;
  return !tx.isTransactionActive && tx.redoStack.length > 0;
};

// #endregion 🔖Sketchpad Selectors

// #endregion 🔖Sketchpad Machine

export type UiEntityKind = "kit" | "type" | "design" | "piece" | "connection" | "connector" | "model" | "quality" | "benchmark" | "file" | "folder" | "author" | "port" | "tag" | "concept";

export const selectUiActiveKitGuid = (state: { context: SketchpadContext }) => {
  const path = state.context.sketchpad?.navigation || "/";
  const match = path.match(/\/kit\/([^/]+)/);
  return match ? match[1] : undefined;
};
export const selectUiActiveDesignGuid = (state: { context: SketchpadContext }) => {
  const path = state.context.sketchpad?.navigation || "/";
  const match = path.match(/\/design\/([^/]+)/);
  return match ? match[1] : undefined;
};
export const selectUiActiveTypeGuid = (state: { context: SketchpadContext }) => {
  const path = state.context.sketchpad?.navigation || "/";
  const match = path.match(/\/type\/([^/]+)/);
  return match ? match[1] : undefined;
};
export const selectUiActiveQualityGuid = (state: { context: SketchpadContext }) => {
  const path = state.context.sketchpad?.navigation || "/";
  const match = path.match(/\/quality\/([^/]+)/);
  return match ? match[1] : undefined;
};
export const selectUiIsInHome = selectIsInHome;
export const selectUiIsInKit = selectIsInKit;
export const selectUiIsInDesign = selectIsInDesign;
export const selectUiIsInType = selectIsInType;
export const selectUiIsInQuality = selectIsInQuality;
export const selectUiIsInDocs = selectIsInDocs;

// #region 🔖Factory

export function createSketchpadActor(input: SketchpadMachineInput) {
  return createActor(sketchpadMachine, {
    input,
    inspect: (inspectionEvent) => {
      if (inspectionEvent.type === "@xstate.snapshot") {
        const { snapshot, event, actorRef } = inspectionEvent;
        if (event.type === "xstate.init") return;
        const stateValue = "value" in (snapshot as any) ? (typeof (snapshot as any).value === "object" ? JSON.stringify((snapshot as any).value) : (snapshot as any).value) : JSON.stringify(snapshot);
      }
    },
  });
}

// #endregion 🔖Factory

// #region 🔖Legacy Type Exports

export interface TransactionContext<TEdit = any> {
  isTransactionActive: boolean;
  currentTransactionStack: TEdit[];
  pastTransactionsStack: TEdit[];
  redoStack: TEdit[];
  lastDeletedEdit?: TEdit;
}

export interface AppMachineInput<TId = any> {
  id?: TId;
}

export interface AppMachineContext<TSelection = any, TId = any> {
  id?: TId;
  panelVisibility: PanelVisibility;
  selection?: TSelection;
  hover?: any;
  isTransactionActive: boolean;
  currentTransactionStack: any[];
  pastTransactionsStack: any[];
  redoStack: any[];
}

export interface KitMachineInput {
  yDoc: Y.Doc;
  yKit: Y.Map<any>;
  guid: Guid;
  local?: boolean;
  remote?: boolean;
}

export interface KitContext {
  yDoc: Y.Doc;
  yKit: Y.Map<any>;
  guid: Guid;
  local: boolean;
  remote: boolean;
  dirty: boolean;
  cache?: Kit;
}

export type KitEvent =
  | { type: "CHANGE"; diff: KitDiff }
  | { type: "CREATE_TYPE"; typeData: any }
  | { type: "UPDATE_TYPE"; guid: Guid; diff: any }
  | { type: "DELETE_TYPE"; guid: Guid }
  | { type: "CREATE_DESIGN"; design: any }
  | { type: "UPDATE_DESIGN"; guid: Guid; diff: any }
  | { type: "DELETE_DESIGN"; guid: Guid }
  | { type: "Y_UPDATE"; data: any }
  | { type: "MARK_DIRTY" };

function buildKitSnapshot(yKit: Y.Map<any>): Partial<Kit> {
  return {
    guid: yKit.get("guid") as string,
    name: yKit.get("name") as string,
    version: yKit.get("version") as string | undefined,
    description: yKit.get("description") as string | undefined,
    homepage: yKit.get("homepage") as string | undefined,
    license: yKit.get("license") as string | undefined,
    icon: yKit.get("icon") as string | undefined,
    image: yKit.get("image") as string | undefined,
    createdAt: yKit.get("createdAt") as string | undefined,
    updatedAt: yKit.get("updatedAt") as string | undefined,
  };
}

export function selectSnapshot(context: SketchpadContext): SketchpadState {
  return context.sketchpad;
}

export function selectNavigation(context: SketchpadContext): string {
  return migratePath(context.sketchpad.navigation || "/");
}

export function selectTheme(context: SketchpadContext): Theme {
  return context.sketchpad.theme;
}

export function selectLanguage(context: SketchpadContext): string {
  return context.sketchpad.language || "en";
}

export function selectExpertise(context: SketchpadContext): Expertise {
  return context.sketchpad.expertise ?? Expertise.BEGINNER;
}

export function selectMode(context: SketchpadContext): Mode {
  return context.sketchpad.mode ?? Mode.USER;
}

export function selectDevice(context: SketchpadContext): Device {
  return context.sketchpad.device || "desktop";
}

export function selectIsFullscreen(context: SketchpadContext): boolean {
  return context.sketchpad.isFullscreen || false;
}

export function selectPanelSizes(context: SketchpadContext): PanelSizes {
  return context.sketchpad.panelSizes || createDefaultSketchpadState().panelSizes;
}

export function selectKitGuid(context: KitContext): Guid {
  return context.yKit.get("guid") as Guid;
}

export function selectKitName(context: KitContext): string {
  return context.yKit.get("name") as string;
}

export function selectKitSnapshot(context: KitContext): Partial<Kit> {
  if (!context.dirty && context.cache) {
    return context.cache;
  }
  return buildKitSnapshot(context.yKit);
}

// #endregion 🔖Legacy Type Exports

// #region 🔖Actor Types

export type SketchpadActorRef = ActorRefFrom<typeof sketchpadMachine>;

export type SketchpadSnapshot = SnapshotFrom<typeof sketchpadMachine>;

export type SketchpadState$ = { context: SketchpadContext };

export const SketchpadActorContext = createContext<SketchpadActorRef | null>(null);

// #endregion 🔖Actor Types

// #endregion 🔖Machine

// #region 🔖Apps

// #region 🔖Design

export function useIsPieceSelected(): boolean {
  const piece = usePieceScope();
  const { useDesignAppIsPieceSelected } = getDesignAppHooks();

  return useDesignAppIsPieceSelected(undefined, piece?.guid ?? "");
}

export function useIsPieceHovered(): boolean {
  const pieceScope = usePieceScope();
  const { useDesignAppIsPieceHovered } = getDesignAppHooks();

  return useDesignAppIsPieceHovered(undefined, pieceScope?.guid ?? "");
}

export function useIsPieceTransitiveHovered(): boolean {
  const pieceScope = usePieceScope();
  const { useDesignAppIsPieceTransitiveHovered } = getDesignAppHooks();

  const isHovered = useDesignAppIsPieceTransitiveHovered(undefined, pieceScope?.guid ?? "");
  if (!pieceScope) return false;
  return isHovered;
}

export function usePieceStatus(): DiffStatus {
  const piece = usePieceScope();
  const designScope = useDesignScope();
  const { useDesignAppStore } = getDesignAppHooks();
  const designAppStore = useDesignAppStore(identitySelector) as any;

  if (!designAppStore || !piece || !designScope) {
    return DiffStatus.Unchanged;
  }

  const currentStack = designAppStore?.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) {
    return DiffStatus.Unchanged;
  }

  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.pieces?.added) {
          for (const addedPiece of designUpdate.diff.pieces.added) {
            if (addedPiece.guid === piece.guid) {
              return DiffStatus.Added;
            }
          }
        }
        if (designUpdate.diff.pieces?.removed) {
          for (const removedId of designUpdate.diff.pieces.removed) {
            if (removedId === piece.guid) {
              return DiffStatus.Removed;
            }
          }
        }
        if (designUpdate.diff.pieces?.updated) {
          for (const pieceUpdate of designUpdate.diff.pieces.updated) {
            if (pieceUpdate.id === piece.guid) {
              return DiffStatus.Modified;
            }
          }
        }
      }
    }
  }
  return DiffStatus.Unchanged;
}

export function useDiffedPiece<T>(selector?: (piece: Piece) => T, id?: string, deep: boolean = false): T | Piece {
  const originalPiece = usePiece(identitySelector, id, deep) as Piece;
  const pieceScope = usePieceScope();
  const designScope = useDesignScope();
  const { useDesignAppStore } = getDesignAppHooks();
  const designAppStore = useDesignAppStore(identitySelector) as any;

  if (!designAppStore || !pieceScope || !designScope) {
    return selector ? selector(originalPiece) : originalPiece;
  }

  const currentStack = designAppStore?.currentTransactionStack;
  if (!currentStack || currentStack.length === 0) {
    return selector ? selector(originalPiece) : originalPiece;
  }

  let diffedPiece = { ...originalPiece };
  for (const edit of currentStack) {
    if (edit.do?.kitDiff?.designs) {
      for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
        if (designUpdate.diff.pieces?.updated) {
          for (const pieceUpdate of designUpdate.diff.pieces.updated) {
            if (pieceUpdate.id === pieceScope.guid) {
              diffedPiece = { ...diffedPiece, ...pieceUpdate.diff };
            }
          }
        }
      }
    }
  }

  return selector ? selector(diffedPiece) : diffedPiece;
}

export function usePieceCenterU(): HookResult<number> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (pieceScope && piece) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.center.u", pieceScope.guid, { center: { u: value, v: piece.center?.v ?? 0 } });
    },
    [pieceScope, piece, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.center?.u ?? 0, setter);
}

export function usePieceCenterV(): HookResult<number> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (pieceScope && piece) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.center.v", pieceScope.guid, { center: { u: piece.center?.u ?? 0, v: value } });
    },
    [pieceScope, piece, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.center?.v ?? 0, setter);
}

export function usePieceScale(): HookResult<number> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.scale", pieceScope.guid, { scale: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.scale ?? 1, setter);
}

export function usePieceIsHidden(): HookResult<boolean> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: boolean) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.isHidden", pieceScope.guid, { isHidden: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.isHidden ?? false, setter);
}

export function usePieceIsLocked(): HookResult<boolean> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: boolean) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.isLocked", pieceScope.guid, { isLocked: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.isLocked ?? false, setter);
}

export function usePieceColor(): HookResult<string | undefined> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: string | undefined) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.color", pieceScope.guid, { color: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.color, setter);
}

export function usePieceDescription(): HookResult<string | undefined> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: string | undefined) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.description", pieceScope.guid, { description: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.description, setter);
}

export function usePieceName(): HookResult<string | undefined> {
  const pieceScope = usePieceScope();
  const piece = usePiece() as Piece | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: string | undefined) => {
      if (pieceScope) commands.updatePiece("semio.sketchpad.app.design.panel.details.section.piece.name", pieceScope.guid, { name: value });
    },
    [pieceScope, commands],
  );
  return conditionalHookResult(!!pieceScope && !!piece, piece?.name, setter);
}

export function useIsConnectionSelected(): boolean {
  const connectionScope = useConnectionScope();
  const { useDesignAppIsConnectionSelected } = getDesignAppHooks();

  return useDesignAppIsConnectionSelected(undefined, connectionScope?.guid ?? "");
}

export function useIsConnectionHovered(): boolean {
  const connectionScope = useConnectionScope();
  const { useDesignAppIsConnectionHovered } = getDesignAppHooks();

  return useDesignAppIsConnectionHovered(undefined, connectionScope?.guid ?? "");
}

export function useConnectionStatus(): DiffStatus {
  const connection = useConnectionScope();
  const { useDesignAppDiff } = getDesignAppHooks();
  const kitDiff = useDesignAppDiff();
  const designScope = useDesignScope();

  if (!connection || !designScope || !kitDiff?.designs?.updated) {
    return DiffStatus.Unchanged;
  }

  for (const designUpdate of kitDiff.designs.updated) {
    if (designUpdate.diff.connections?.added) {
      for (const conn of designUpdate.diff.connections.added) {
        if (conn.guid === connection.guid) {
          return DiffStatus.Added;
        }
      }
    }
    if (designUpdate.diff.connections?.removed) {
      for (const removedConn of designUpdate.diff.connections.removed) {
        if (typeof removedConn === "string" && removedConn === connection.guid) {
          return DiffStatus.Removed;
        }
      }
    }
    if (designUpdate.diff.connections?.updated) {
      for (const connUpdate of designUpdate.diff.connections.updated) {
        if (typeof connUpdate.id === "string" && connUpdate.id === connection.guid) {
          return DiffStatus.Modified;
        }
      }
    }
  }

  return DiffStatus.Unchanged;
}

export function useConnectionGap(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.gap", connectionScope.guid, { gap: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.gap ?? 0, setter);
}

export function useConnectionShift(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.shift", connectionScope.guid, { shift: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.shift ?? 0, setter);
}

export function useConnectionRise(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.rise", connectionScope.guid, { rise: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.rise ?? 0, setter);
}

export function useConnectionRotation(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.rotation", connectionScope.guid, { rotation: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.rotation ?? 0, setter);
}

export function useConnectionTurn(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.turn", connectionScope.guid, { turn: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.turn ?? 0, setter);
}

export function useConnectionTilt(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.tilt", connectionScope.guid, { tilt: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.tilt ?? 0, setter);
}

export function useConnectionU(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.u", connectionScope.guid, { u: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.u ?? 0, setter);
}

export function useConnectionV(): HookResult<number> {
  const connectionScope = useConnectionScope();
  const connection = useConnection() as Connection | null;
  const { useDesignAppCommands } = getDesignAppHooks();
  const commands = useDesignAppCommands();
  const setter = useCallback(
    (value: number) => {
      if (connectionScope) commands.updateConnection("semio.sketchpad.app.design.panel.details.section.connection.v", connectionScope.guid, { v: value });
    },
    [connectionScope, commands],
  );
  return conditionalHookResult(!!connectionScope && !!connection, connection?.v ?? 0, setter);
}

export function useClusterableGroups() {
  const designScope = useDesignScope();
  const pieces = usePieces();
  const connections = useConnections();
  const { useDesignAppSelection } = getDesignAppHooks();
  const selection = useDesignAppSelection();
  return useMemo(() => {
    if (!designScope) return [];
    const design = { guid: designScope.guid, pieces, connections } as Design;
    return getClusterableGroups(design, selection.pieces ?? []);
  }, [designScope?.guid, pieces, connections, selection.pieces]);
}

export function useDiffedKit(): Kit {
  const kit = useKit() as Kit;
  const { useDesignAppDiff } = getDesignAppHooks();
  const diff = useDesignAppDiff();
  return diff ? applyKitDiff(kit, diff) : kit;
}

export function usePortColoredTypes(): Type[] {
  const diffedKit = useDiffedKit();
  const kitTypes = useKitTypes();
  const typesWithColoredConnectors = useMemo(() => {
    if (!diffedKit.types || !kitTypes) return [];
    const colorDiff = colorPortsForTypes(diffedKit.types);
    const updatedIds = colorDiff.updated?.map((u) => u.type.guid) || [];
    return kitTypes.filter((t) => updatedIds.includes(t.guid));
  }, [diffedKit.types, kitTypes]);
  return typesWithColoredConnectors;
}

export function usePieceWithDiff(): { original: Piece; diffed: Piece | null; hasDiff: boolean } {
  const originalPiece = usePiece() as Piece;
  const diffedPiece = useDiffedPiece() as Piece;
  const status = usePieceStatus();

  const hasDiff = status !== DiffStatus.Unchanged;

  return {
    original: originalPiece,
    diffed: hasDiff ? diffedPiece : null,
    hasDiff,
  };
}

export function useConnectionColor(): { stroke: string; fill: string } {
  const connection = useConnectionScope();
  const { useDesignAppDiff } = getDesignAppHooks();
  const kitDiff = useDesignAppDiff();
  const designScope = useDesignScope();

  let diffStatus = DiffStatus.Unchanged;
  if (connection && designScope && kitDiff?.designs?.updated) {
    for (const designUpdate of kitDiff.designs.updated) {
      if (designUpdate.diff.connections?.added) {
        for (const conn of designUpdate.diff.connections.added) {
          if (conn.guid === connection.guid) {
            diffStatus = DiffStatus.Added;
            break;
          }
        }
      }
      if (designUpdate.diff.connections?.removed) {
        for (const removedConn of designUpdate.diff.connections.removed) {
          if (typeof removedConn === "string" && removedConn === connection.guid) {
            diffStatus = DiffStatus.Removed;
            break;
          }
        }
      }
      if (designUpdate.diff.connections?.updated) {
        for (const connUpdate of designUpdate.diff.connections.updated) {
          if (typeof connUpdate.id === "string" && connUpdate.id === connection.guid) {
            diffStatus = DiffStatus.Modified;
            break;
          }
        }
      }
    }
  }

  const stroke = diffStatus === DiffStatus.Added ? "#00ff00" : diffStatus === DiffStatus.Removed ? "#ff0000" : diffStatus === DiffStatus.Modified ? "#ffff00" : "#ffffff";
  const fill = diffStatus === DiffStatus.Added ? "#00ff0033" : diffStatus === DiffStatus.Removed ? "#ff000033" : diffStatus === DiffStatus.Modified ? "#ffff0033" : "#ffffff33";

  return { stroke, fill };
}

export function useDiffedDesign(): Design {
  const kit = useDiffedKit();
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useDiffedDesign must be called within a DesignScopeProvider");
  return findDesignInKit(kit, designScope.guid);
}

// #endregion 🔖Design

// #region 🔖Sketchpad

export function createObserver<T>(yMap: Y.Map<T> | Y.Array<T>, subscribe: Subscribe, deep: boolean = false): Disposable {
  const callback = () => {
    subscribe(() => {});
  };
  if (deep) {
    yMap.observeDeep(callback);
    return () => yMap.unobserveDeep(callback);
  } else {
    yMap.observe(callback);
    return () => yMap.unobserve(callback);
  }
}

export function createFieldObserver<T>(yMap: Y.Map<T>, key: string, subscribe: Subscribe, deep: boolean = false): Disposable {
  const disposables: Disposable[] = [];
  let currentValue = yMap.get(key);
  const notifySubscriber = () => subscribe(() => {});
  const setupNestedObserver = (value: any) => {
    if (deep && value instanceof Y.Map) {
      const nestedCallback = () => notifySubscriber();
      value.observeDeep(nestedCallback);
      disposables.push(() => value.unobserveDeep(nestedCallback));
    } else if (deep && value instanceof Y.Array) {
      const nestedCallback = () => notifySubscriber();
      value.observeDeep(nestedCallback);
      disposables.push(() => value.unobserveDeep(nestedCallback));
    }
  };
  if (currentValue !== undefined) setupNestedObserver(currentValue);
  const mapCallback = (event: Y.YMapEvent<T>) => {
    if (event.keysChanged.has(key)) {
      disposables.forEach((d) => d());
      disposables.length = 0;
      currentValue = yMap.get(key);
      if (currentValue !== undefined) setupNestedObserver(currentValue);
      notifySubscriber();
    }
  };
  yMap.observe(mapCallback);
  return () => {
    yMap.unobserve(mapCallback);
    disposables.forEach((d) => d());
  };
}

export function createFieldsObserver<T>(yMap: Y.Map<T>, keys: string[], subscribe: Subscribe, deep: boolean = false): Disposable {
  const disposables: Disposable[] = [];
  const keySet = new Set(keys);
  const nestedDisposables = new Map<string, Disposable[]>();
  const notifySubscriber = () => subscribe(() => {});
  const setupNestedObserver = (key: string, value: any) => {
    const keyDisposables: Disposable[] = [];
    if (deep && value instanceof Y.Map) {
      const nestedCallback = () => notifySubscriber();
      value.observeDeep(nestedCallback);
      keyDisposables.push(() => value.unobserveDeep(nestedCallback));
    } else if (deep && value instanceof Y.Array) {
      const nestedCallback = () => notifySubscriber();
      value.observeDeep(nestedCallback);
      keyDisposables.push(() => value.unobserveDeep(nestedCallback));
    }
    nestedDisposables.set(key, keyDisposables);
  };
  const cleanupNestedObserver = (key: string) => {
    const keyDisposables = nestedDisposables.get(key);
    if (keyDisposables) {
      keyDisposables.forEach((d) => d());
      nestedDisposables.delete(key);
    }
  };
  keys.forEach((key) => {
    const value = yMap.get(key);
    if (value !== undefined) setupNestedObserver(key, value);
  });
  const mapCallback = (event: Y.YMapEvent<T>) => {
    let shouldNotify = false;
    event.keysChanged.forEach((key) => {
      if (keySet.has(key)) {
        cleanupNestedObserver(key);
        const newValue = yMap.get(key);
        if (newValue !== undefined) setupNestedObserver(key, newValue);
        shouldNotify = true;
      }
    });
    if (shouldNotify) notifySubscriber();
  };
  yMap.observe(mapCallback);
  return () => {
    yMap.unobserve(mapCallback);
    nestedDisposables.forEach((keyDisposables) => keyDisposables.forEach((d) => d()));
    nestedDisposables.clear();
  };
}

export function createArrayItemMembershipObserver(getYArray: () => Y.Array<string> | undefined, itemId: string, subscribe: Subscribe): Disposable {
  let wasInArray = false;
  const notifySubscriber = () => subscribe(() => {});

  const checkMembership = () => {
    const yArray = getYArray();
    return yArray ? yArray.toArray().includes(itemId) : false;
  };
  wasInArray = checkMembership();

  let arrayDisposer: Disposable | null = null;

  const setupArrayObserver = () => {
    const yArray = getYArray();
    if (!yArray) {
      arrayDisposer = null;
      return;
    }

    const arrayCallback = () => {
      const isInArray = yArray.toArray().includes(itemId);
      if (isInArray !== wasInArray) {
        wasInArray = isInArray;
        notifySubscriber();
      }
    };

    yArray.observe(arrayCallback);
    arrayDisposer = () => yArray.unobserve(arrayCallback);
  };

  setupArrayObserver();

  return () => {
    if (arrayDisposer) arrayDisposer();
  };
}

export function createNestedArrayItemMembershipObserver(yMap: Y.Map<any>, mapKey: string, arrayKey: string, itemId: string, subscribe: Subscribe): Disposable {
  let wasInArray = false;
  let currentNestedMap: Y.Map<any> | undefined;
  let currentArray: Y.Array<string> | undefined;
  let nestedMapDisposer: Disposable | null = null;
  let arrayDisposer: Disposable | null = null;
  const notifySubscriber = () => subscribe(() => {});

  const checkMembership = (): boolean => {
    const nestedMap = yMap.get(mapKey) as Y.Map<any> | undefined;
    if (!nestedMap) return false;
    const arr = nestedMap.get(arrayKey) as Y.Array<string> | undefined;
    if (!arr) return false;
    return arr.toArray().includes(itemId);
  };

  const setupArrayObserver = () => {
    if (arrayDisposer) {
      arrayDisposer();
      arrayDisposer = null;
    }

    const nestedMap = yMap.get(mapKey) as Y.Map<any> | undefined;
    if (!nestedMap) return;

    const arr = nestedMap.get(arrayKey) as Y.Array<string> | undefined;
    if (!arr) return;

    currentArray = arr;

    const arrayCallback = () => {
      const isInArray = arr.toArray().includes(itemId);
      if (isInArray !== wasInArray) {
        wasInArray = isInArray;
        notifySubscriber();
      }
    };

    arr.observe(arrayCallback);
    arrayDisposer = () => arr.unobserve(arrayCallback);
  };

  const setupNestedMapObserver = () => {
    if (nestedMapDisposer) {
      nestedMapDisposer();
      nestedMapDisposer = null;
    }

    const nestedMap = yMap.get(mapKey) as Y.Map<any> | undefined;
    if (!nestedMap) return;

    currentNestedMap = nestedMap;

    const nestedMapCallback = (event: Y.YMapEvent<any>) => {
      if (event.keysChanged.has(arrayKey)) {
        const prevInArray = wasInArray;
        setupArrayObserver();
        wasInArray = checkMembership();
        if (wasInArray !== prevInArray) {
          notifySubscriber();
        }
      }
    };

    nestedMap.observe(nestedMapCallback);
    nestedMapDisposer = () => nestedMap.unobserve(nestedMapCallback);
  };

  const topLevelCallback = (event: Y.YMapEvent<any>) => {
    if (event.keysChanged.has(mapKey)) {
      const prevInArray = wasInArray;

      if (arrayDisposer) {
        arrayDisposer();
        arrayDisposer = null;
      }
      if (nestedMapDisposer) {
        nestedMapDisposer();
        nestedMapDisposer = null;
      }

      setupNestedMapObserver();
      setupArrayObserver();
      wasInArray = checkMembership();

      if (wasInArray !== prevInArray) {
        notifySubscriber();
      }
    }
  };

  wasInArray = checkMembership();
  setupNestedMapObserver();
  setupArrayObserver();
  yMap.observe(topLevelCallback);

  return () => {
    yMap.unobserve(topLevelCallback);
    if (nestedMapDisposer) nestedMapDisposer();
    if (arrayDisposer) arrayDisposer();
  };
}

let performanceLoggingEnabled = false;
const performanceLogCounts = new Map<string, number>();
const performanceLogTimestamps = new Map<string, number>();

export function enablePerformanceLogging(enabled: boolean = true) {
  performanceLoggingEnabled = enabled;
  if (!enabled) {
    performanceLogCounts.clear();
    performanceLogTimestamps.clear();
  }
}

function logStateAccess(hookName: string, storeType: string, selectorInfo?: string) {
  if (!performanceLoggingEnabled) return;
  const key = `${hookName}:${storeType}${selectorInfo ? `:${selectorInfo}` : ""}`;
  const count = (performanceLogCounts.get(key) || 0) + 1;
  performanceLogCounts.set(key, count);
  const now = Date.now();
  const lastTime = performanceLogTimestamps.get(key) || 0;
  performanceLogTimestamps.set(key, now);
  if (now - lastTime < 100) {
    console.warn(`[PERF] Rapid re-render: ${key} (${count}x, ${now - lastTime}ms apart)`);
  }
}

export function useSync<T, TSelected = T>(store: { onChanged: (subscribe: Subscribe) => Disposable; snapshot: () => T }, selector: (value: T) => TSelected = identitySelector as any, deep?: boolean): TSelected {
  const subscribe = useCallback(
    (callback: () => void) => {
      return store.onChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store],
  );
  const getSnapshot = useCallback(() => {
    logStateAccess("useSync", (store as any).constructor?.name || "unknown", selector === identitySelector ? "FULL_STATE" : "selector");
    return selector(store.snapshot());
  }, [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncOptional<T, TSelected = T>(store: { onChanged: (subscribe: Subscribe) => Disposable; snapshot: () => T } | null | undefined, selector: (value: T) => TSelected = identitySelector as any): TSelected | null {
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};
      return store.onChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store],
  );
  const getSnapshot = useCallback(() => {
    if (!store) return null as TSelected | null;
    logStateAccess("useSyncOptional", (store as any).constructor?.name || "unknown", selector === identitySelector ? "FULL_STATE" : "selector");
    return selector(store.snapshot());
  }, [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncDeep<T, TSelected = T>(store: { onChangedDeep: (subscribe: Subscribe) => Disposable; snapshot: () => T } | null | undefined, selector: (value: T) => TSelected = identitySelector as any, deep?: boolean): TSelected | null {
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};
      return store.onChangedDeep((cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store],
  );
  const getSnapshot = useCallback(() => {
    if (!store) return null as TSelected | null;
    logStateAccess("useSyncDeep", (store as any).constructor?.name || "unknown", selector === identitySelector ? "FULL_STATE_DEEP" : "selector");
    return selector(store.snapshot());
  }, [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncField<T, TSelected = T>(
  store: { onFieldChanged: (key: string, subscribe: Subscribe, deep?: boolean) => Disposable; snapshot: () => T; getFieldSnapshot?: (key: string) => any },
  key: string,
  selector: (value: T) => TSelected = identitySelector as any,
  deep: boolean = true,
  comparator?: (a: TSelected, b: TSelected) => boolean,
): TSelected {
  const subscribe = useCallback(
    (callback: () => void) => {
      return store.onFieldChanged(
        key,
        (cb: () => void) => {
          cb();
          callback();
          return () => {};
        },
        deep,
      );
    },
    [store, key, deep],
  );

  const lastResultRef = useRef<{ value: TSelected; json?: string }>({ value: undefined as any });

  const getSnapshot = useCallback(() => {
    let newValue: TSelected;
    if (store.getFieldSnapshot) {
      const fieldValue = store.getFieldSnapshot(key);
      newValue = selector({ [key]: fieldValue } as T);
    } else {
      newValue = selector(store.snapshot());
    }

    if (newValue === null || typeof newValue !== "object") {
      if (newValue === lastResultRef.current.value) {
        return lastResultRef.current.value;
      }
      lastResultRef.current = { value: newValue };
      return newValue;
    }

    if (comparator) {
      if (lastResultRef.current.value !== undefined && comparator(lastResultRef.current.value, newValue)) {
        return lastResultRef.current.value;
      }
      lastResultRef.current = { value: newValue };
      return newValue;
    }

    const newJson = JSON.stringify(newValue);
    if (newJson === lastResultRef.current.json) {
      return lastResultRef.current.value;
    }

    lastResultRef.current = { value: newValue, json: newJson };
    return newValue;
  }, [store, selector, key, comparator]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncFields<T, TSelected = T>(
  store: { onFieldsChanged: (keys: string[], subscribe: Subscribe, deep?: boolean) => Disposable; snapshot: () => T },
  keys: string[],
  selector: (value: T) => TSelected = identitySelector as any,
  deep: boolean = true,
  comparator?: (a: TSelected, b: TSelected) => boolean,
): TSelected {
  const keysRef = useRef(keys);
  if (keys.length !== keysRef.current.length || keys.some((k, i) => k !== keysRef.current[i])) keysRef.current = keys;
  const stableKeys = keysRef.current;
  const subscribe = useCallback(
    (callback: () => void) => {
      return store.onFieldsChanged(
        stableKeys,
        (cb: () => void) => {
          cb();
          callback();
          return () => {};
        },
        deep,
      );
    },
    [store, stableKeys, deep],
  );

  const lastResultRef = useRef<{ value: TSelected; json?: string }>({ value: undefined as any });

  const getSnapshot = useCallback(() => {
    const newValue = selector(store.snapshot());

    if (newValue === null || typeof newValue !== "object") {
      if (newValue === lastResultRef.current.value) {
        return lastResultRef.current.value;
      }
      lastResultRef.current = { value: newValue };
      return newValue;
    }

    if (comparator) {
      if (lastResultRef.current.value !== undefined && comparator(lastResultRef.current.value, newValue)) {
        return lastResultRef.current.value;
      }
      lastResultRef.current = { value: newValue };
      return newValue;
    }

    const newJson = JSON.stringify(newValue);
    if (newJson === lastResultRef.current.json) {
      return lastResultRef.current.value;
    }

    lastResultRef.current = { value: newValue, json: newJson };
    return newValue;
  }, [store, selector, comparator]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncNestedArrayItemMembership(store: { yMap: Y.Map<any> } | null, mapKey: string, arrayKey: string, itemId: string): boolean {
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};
      return createNestedArrayItemMembershipObserver(store.yMap, mapKey, arrayKey, itemId, (cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store, mapKey, arrayKey, itemId],
  );

  const getSnapshot = useCallback(() => {
    if (!store) return false;
    const nestedMap = store.yMap.get(mapKey) as Y.Map<any> | undefined;
    if (!nestedMap) return false;
    const arr = nestedMap.get(arrayKey) as Y.Array<string> | undefined;
    if (!arr) return false;
    return arr.toArray().includes(itemId);
  }, [store, mapKey, arrayKey, itemId]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncSelectionItemMembership(store: { yMap: Y.Map<any> } | null, arrayKey: string, itemId: string): boolean {
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};

      return createNestedArrayItemMembershipObserver(store.yMap, "selection", arrayKey, itemId, (cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store, arrayKey, itemId],
  );

  const getSnapshot = useCallback(() => {
    if (!store) return false;
    const selection = store.yMap.get("selection") as Y.Map<any> | undefined;
    if (!selection) return false;
    const arr = selection.get(arrayKey) as Y.Array<string> | undefined;
    if (!arr) return false;
    return arr.toArray().includes(itemId);
  }, [store, arrayKey, itemId]);

  return useSyncExternalStore(subscribe, getSnapshot);
}

export function usePath<T, TSelected = T>(
  store: { onPathChanged: (path: YPath, subscribe: Subscribe) => Disposable; getPathSnapshot: (path: YPath) => any } | null,
  path: YPath,
  selector: (value: any) => TSelected = identitySelector as any,
): TSelected | undefined {
  const pathKey = useMemo(() => JSON.stringify(path), [path]);
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!store) return () => {};
      return store.onPathChanged(path, () => {
        callback();
        return () => {};
      });
    },
    [store, pathKey],
  );
  const lastResultRef = useRef<{ value: TSelected; json?: string }>({ value: undefined as any });
  const getSnapshot = useCallback(() => {
    if (!store) return undefined;
    const rawValue = store.getPathSnapshot(path);
    const value = rawValue instanceof Y.Map || rawValue instanceof Y.Array ? rawValue.toJSON() : rawValue;
    const newValue = selector(value);
    if (newValue === null || typeof newValue !== "object") {
      if (newValue === lastResultRef.current.value) return lastResultRef.current.value;
      lastResultRef.current = { value: newValue };
      return newValue;
    }
    const newJson = JSON.stringify(newValue);
    if (newJson === lastResultRef.current.json) return lastResultRef.current.value;
    lastResultRef.current = { value: newValue, json: newJson };
    return newValue;
  }, [store, pathKey, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useDerived<T, TSelected = T>(derivedStore: DerivedStore | null, key: string, deps: BaseDependency[], compute: () => T, selector: (value: T) => TSelected = identitySelector as any): TSelected | undefined {
  const nodeRef = useRef<DerivedNode<T> | null>(null);
  const depsKey = useMemo(() => JSON.stringify(deps.map((d) => ({ path: d.path }))), [deps]);
  useEffect(() => {
    if (!derivedStore) return;
    nodeRef.current = derivedStore.getOrCreate(key, deps, compute);
    return () => {
      nodeRef.current = null;
    };
  }, [derivedStore, key, depsKey]);
  const subscribe = useCallback(
    (callback: () => void) => {
      if (!nodeRef.current) return () => {};
      return nodeRef.current.subscribe(callback);
    },
    [derivedStore, key, depsKey],
  );
  const lastResultRef = useRef<{ value: TSelected; json?: string }>({ value: undefined as any });
  const getSnapshot = useCallback(() => {
    if (!nodeRef.current) return undefined;
    const rawValue = nodeRef.current.snapshot();
    const newValue = selector(rawValue);
    if (newValue === null || typeof newValue !== "object") {
      if (newValue === lastResultRef.current.value) return lastResultRef.current.value;
      lastResultRef.current = { value: newValue };
      return newValue;
    }
    const newJson = JSON.stringify(newValue);
    if (newJson === lastResultRef.current.json) return lastResultRef.current.value;
    lastResultRef.current = { value: newValue, json: newJson };
    return newValue;
  }, [derivedStore, key, depsKey, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

const initialDocsPanelVisibility: PanelVisibility = {
  toolbar: false,
  workbench: false,
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

const nullStore: Synchronizable<null> = {
  onChanged: () => () => {},
  onChangedDeep: () => () => {},
  snapshot: () => null,
};

export function useSyncWithState<TAccessl, TSelected = TAccessl>(store: (Synchronizable<TAccessl> & Store<TAccessl>) | null, selector?: (state: TAccessl) => TSelected, deep: boolean = false): StoreState<TAccessl | TSelected> {
  const actualStore = store || (nullStore as unknown as Synchronizable<TAccessl> & Store<TAccessl>);
  const state = deep
    ? useSyncExternalStore(
        (onStoreChange: () => void) =>
          actualStore.onChangedDeep((cb: () => void) => {
            cb();
            onStoreChange();
            return () => {};
          }),
        actualStore.snapshot.bind(actualStore),
      )
    : useSyncExternalStore(
        (onStoreChange: () => void) =>
          actualStore.onChanged((cb: () => void) => {
            cb();
            onStoreChange();
            return () => {};
          }),
        actualStore.snapshot.bind(actualStore),
      );
  if (!store) {
    return { status: StoreStatus.IDLE, data: null as any };
  }
  const storeState = (store as Store<TAccessl>).getState();
  return {
    ...storeState,
    data: storeState.data && selector ? selector(storeState.data) : storeState.data,
  } as StoreState<TAccessl | TSelected>;
}

function areSameKit(kit1: Guid, kit2: Guid): boolean {
  return kit1 === kit2;
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

type YKitMetadata = Y.Map<string | boolean>;
type YKitMetadatas = Y.Array<YKitMetadata>;

type KitAppStoreInstance = any;
type DesignAppStoreInstance = any;
type TypeAppStoreInstance = any;
type QualityAppStoreInstance = any;
type HomeStoreInstance = any;
type DocsAppStoreInstance = any;

type KitAppStoreFactory = (parent: SketchpadStore, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) => KitAppStoreInstance;

type DesignAppStoreFactoryLocal = (parent: SketchpadStore, id: DesignAppId, state?: DesignAppState) => DesignAppStoreInstance;
type TypeAppStoreFactoryLocal = (parent: SketchpadStore, id: TypeAppId, state?: TypeAppState) => TypeAppStoreInstance;
type QualityAppStoreFactoryLocal = (parent: SketchpadStore, id: QualityAppId, state?: QualityAppState) => QualityAppStoreInstance;
type HomeStoreFactory = (parent: SketchpadStore) => HomeStoreInstance;
type DocsAppStoreFactory = (parent: SketchpadStore) => DocsAppStoreInstance;

import {
  registerDesignAppStoreFactory,
  registerKitAppStoreFactory,
  registerQualityAppStoreFactory,
  registerTypeAppStoreFactory,
  getDesignAppStoreFactory as resolveDesignAppStoreFactory,
  getKitAppStoreFactory as resolveKitAppStoreFactory,
  getQualityAppStoreFactory as resolveQualityAppStoreFactory,
  getTypeAppStoreFactory as resolveTypeAppStoreFactory,
} from "./shared";

let homeStoreFactory: HomeStoreFactory | undefined;
let docsAppStoreFactory: DocsAppStoreFactory | undefined;

export function registerHomeStoreFactory(factory: HomeStoreFactory) {
  homeStoreFactory = factory;
}

export function registerDocsAppStoreFactory(factory: DocsAppStoreFactory) {
  docsAppStoreFactory = factory;
}

function resolveHomeStoreFactory(): HomeStoreFactory {
  if (!homeStoreFactory) throw new Error("Home store factory not registered");
  return homeStoreFactory;
}

function resolveDocsAppStoreFactory(): DocsAppStoreFactory {
  if (!docsAppStoreFactory) throw new Error("Docs app store factory not registered");
  return docsAppStoreFactory;
}

export { registerDesignAppStoreFactory, registerKitAppStoreFactory, registerQualityAppStoreFactory, registerTypeAppStoreFactory };

type YSketchpadVal = string | number | boolean;
type YSketchpad = Y.Map<YSketchpadVal>;

export class SketchpadStore {
  private static _modulesLoaded = false;
  public static _loadModules() {
    if (SketchpadStore._modulesLoaded) return;
    SketchpadStore._modulesLoaded = true;
    Promise.all([
      import("./Design").then((m) => {
        designAppModuleCache = m;
      }),
      import("./Home").then((m) => {
        homeAppModuleCache = m;
      }),
      import("./Kit").then((m) => {
        kitAppModuleCache = m;
      }),
      import("./Type").then((m) => {
        typeAppModuleCache = m;
      }),
      import("./Quality").then((m) => {
        qualityAppModuleCache = m;
      }),
    ]).catch((err) => {});
  }
  private readonly id: string | undefined;
  private readonly remote: RemoteProviders | undefined;
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly kits: Map<string, KitStore>;
  private readonly yKits: YKitMetadatas;
  private homeStore?: HomeStoreInstance;
  private readonly yKitApps: YKitApps;
  private readonly kitApps: Map<string, KitAppStoreInstance>;
  private readonly typeApps: Map<string, TypeAppStoreInstance>;
  private readonly qualityApps: Map<string, QualityAppStoreInstance>;
  private readonly designApps: Map<string, Map<string, DesignAppStoreInstance>>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult>;
  private readonly commandMetadata: Map<string, { user?: boolean }>;
  private cache?: SketchpadState;
  private cacheHash?: string;
  private kitShallowsCache?: KitShallow[];
  private kitShallowsCacheHash?: string;
  private readonly kitCreatedSubscribers: Set<() => void>;
  private readonly kitDeletedSubscribers: Set<() => void>;
  private readonly kitAppCreatedSubscribers: Set<() => void>;
  private readonly kitAppDeletedSubscribers: Set<() => void>;
  private readonly typeAppCreatedSubscribers: Set<() => void>;
  private readonly typeAppDeletedSubscribers: Set<() => void>;
  private readonly qualityAppCreatedSubscribers: Set<() => void>;
  private readonly qualityAppDeletedSubscribers: Set<() => void>;
  private readonly designAppCreatedSubscribers: Set<() => void>;
  private readonly designAppDeletedSubscribers: Set<() => void>;
  private readonly tutorialStoreInstance: any;
  private actor?: SketchpadActorRef;
  private actorUnsubscribe?: () => void;

  constructor(id?: string, remote?: RemoteProviders, initialState?: ExtendedInitialState) {
    this.id = id;
    this.remote = remote;
    this.yDoc = new Y.Doc();
    this.kits = new Map();
    this.kitApps = new Map();
    this.typeApps = new Map();
    this.qualityApps = new Map();
    this.designApps = new Map();
    this.commandRegistry = new Map();
    this.commandMetadata = new Map();
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
      this.persistence.once("synced", () => {
        const isMobile = typeof window !== "undefined" ? window.innerWidth < 768 : false;
        this.ySketchpad.set("isMobile", isMobile);
      });
      if (this.remote) {
        this.remote.yProvider(this.yDoc, id);
      }
    }

    this.ySketchpad = this.yDoc.getMap("sketchpad");
    this.yKits = this.yDoc.getArray("kits");
    this.yKitApps = this.yDoc.getMap("kitApps");

    const yTutorials = this.yDoc.getMap("tutorials");
    this.tutorialStoreInstance = new TutorialStore(yTutorials, (fn) => this.yDoc.transact(fn));

    this.loadPersistedKits();

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
      if (!this.ySketchpad.has("theme")) {
        this.ySketchpad.set("theme", Theme.SYSTEM);
      }
      if (!this.ySketchpad.has("language")) {
        this.ySketchpad.set("language", "en");
      }
      if (!this.ySketchpad.has("device")) {
        this.ySketchpad.set("device", JSON.stringify("desktop"));
      }
      if (!this.ySketchpad.has("expertise")) {
        this.ySketchpad.set("expertise", Expertise.BEGINNER);
      }
      if (!this.ySketchpad.has("mode")) {
        this.ySketchpad.set("mode", Mode.USER);
      }
      if (!this.ySketchpad.has("isFullscreen")) {
        this.ySketchpad.set("isFullscreen", false);
      }
      const isMobile = typeof window !== "undefined" ? window.innerWidth < 768 : false;
      this.ySketchpad.set("isMobile", isMobile);
      if (!this.ySketchpad.has("activeInteraction")) {
        this.ySketchpad.set("activeInteraction", "");
      }
      if (!this.ySketchpad.has("settings")) {
        this.ySketchpad.set(
          "settings",
          JSON.stringify({
            apps: {
              design: {
                diagram: { proximityConnectDistance: 10 },
                scene: { gridSize: 24 },
              },
            },
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

    Object.entries(commands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
    Object.entries(devCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command, { user: false });
    });

    if (initialState) {
      this.yDoc.transact(() => {
        if (initialState.navigation !== undefined) this.ySketchpad.set("navigation", initialState.navigation);
        if (initialState.navigationHistory !== undefined) this.ySketchpad.set("navigationHistory", JSON.stringify(initialState.navigationHistory));
        if (initialState.navigationHistoryIndex !== undefined) this.ySketchpad.set("navigationHistoryIndex", initialState.navigationHistoryIndex);
        if (initialState.recentSearches !== undefined) this.ySketchpad.set("recentSearches", JSON.stringify(initialState.recentSearches));
        if (initialState.recentFocusItems !== undefined) this.ySketchpad.set("recentFocusItems", JSON.stringify(initialState.recentFocusItems));
        if (initialState.theme !== undefined) this.ySketchpad.set("theme", initialState.theme);
        if (initialState.language !== undefined) this.ySketchpad.set("language", initialState.language);
        if (initialState.device !== undefined) this.ySketchpad.set("device", JSON.stringify(initialState.device));
        if (initialState.expertise !== undefined) this.ySketchpad.set("expertise", initialState.expertise);
        if (initialState.mode !== undefined) this.ySketchpad.set("mode", initialState.mode);
        if (initialState.settings !== undefined) this.ySketchpad.set("settings", JSON.stringify(initialState.settings));
        if (initialState.panelSizes !== undefined) this.ySketchpad.set("panelSizes", JSON.stringify(initialState.panelSizes));
        if (initialState.isFullscreen !== undefined) this.ySketchpad.set("isFullscreen", initialState.isFullscreen);
        if (initialState.hotkeyOverrides !== undefined) this.ySketchpad.set("hotkeyOverrides", JSON.stringify(initialState.hotkeyOverrides));
        if (initialState.activeHotkeySetting !== undefined) this.ySketchpad.set("activeHotkeySetting", initialState.activeHotkeySetting);
      });

      if (initialState.kits) {
        initialState.kits.forEach(({ kit, local, remote }) => {
          this.createKit(kit, local, remote);
        });
      }
    }
  }

  setActor = (actor: SketchpadActorRef) => {
    if (this.actorUnsubscribe) {
      this.actorUnsubscribe();
      this.actorUnsubscribe = undefined;
    }
    this.actor = actor;
    this.cache = undefined;
    this.cacheHash = undefined;
    if (this.id) {
      writeSketchpadStateToLocalStorage(this.id, actor.getSnapshot().context.sketchpad);
      const subscription = actor.subscribe((snapshot) => {
        writeSketchpadStateToLocalStorage(this.id!, snapshot.context.sketchpad);
      });
      this.actorUnsubscribe = () => subscription.unsubscribe();
    }
  };

  hash = (state: SketchpadState): string => {
    return JSON.stringify(state);
  };

  snapshot = (): SketchpadState => {
    if (this.actor) {
      return this.actor.getSnapshot().context.sketchpad;
    }
    const settingsStr = this.ySketchpad.get("settings") as string;
    const settings = settingsStr
      ? JSON.parse(settingsStr)
      : {
          apps: {
            design: {
              diagram: { proximityConnectDistance: 10 },
              scene: { gridSize: 24 },
            },
          },
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
    const hotkeyOverridesStr = this.ySketchpad.get("hotkeyOverrides") as string;
    const hotkeyOverrides = hotkeyOverridesStr ? JSON.parse(hotkeyOverridesStr) : {};
    const deviceStr = this.ySketchpad.get("device") as string;
    const device: Device = deviceStr ? JSON.parse(deviceStr) : "desktop";
    const currentValues = {
      navigation: migratePath((this.ySketchpad.get("navigation") as string) || "/"),
      navigationHistory: navigationHistory,
      navigationHistoryIndex: (this.ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
      recentSearches: recentSearches,
      recentFocusItems: recentFocusItems,
      theme: this.ySketchpad.get("theme") as Theme,
      language: (this.ySketchpad.get("language") as string) || "en",
      device: device,
      expertise: (this.ySketchpad.get("expertise") as Expertise) ?? Expertise.BEGINNER,
      mode: (this.ySketchpad.get("mode") as Mode) ?? Mode.USER,
      settings: settings,
      panelSizes: panelSizes,
      isFullscreen: (this.ySketchpad.get("isFullscreen") as boolean) || false,
      isMobile: (this.ySketchpad.get("isMobile") as boolean) || false,
      activeInteraction: (this.ySketchpad.get("activeInteraction") as string) || undefined,
      hotkeyOverrides: hotkeyOverrides,
      activeHotkeySetting: (this.ySketchpad.get("activeHotkeySetting") as string) || undefined,
    };
    const currentHash = this.hash(currentValues);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentValues;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  createKit = (kit: Kit, local?: boolean, remote?: boolean) => {
    const kitStore = new KitStore(this, kit, local, remote, this.remote);
    this.kits.set(kit.guid, kitStore);

    this.yDoc.transact(() => {
      const kitMetadata = new Y.Map<string | boolean>();
      kitMetadata.set("guid", kit.guid);
      kitMetadata.set("local", local || false);
      kitMetadata.set("remote", remote || false);
      this.yKits.push([kitMetadata as any]);
    });

    this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  private loadKitFilesFromPublic = async (kitGuid: string, kitStore: KitStore) => {
    try {
      const zipUrl = `/public/${kitGuid}.zip`;
      const response = await fetch(zipUrl);

      if (!response.ok) {
        return;
      }

      const blob = await response.blob();

      const JSZip = (await import("jszip")).default;
      const zip = await JSZip.loadAsync(blob);

      const filePromises: Promise<void>[] = [];
      zip.forEach((relativePath, zipEntry) => {
        if (!zipEntry.dir) {
          filePromises.push(
            zipEntry.async("blob").then(async (fileBlob) => {
              const file: SemioFile = {
                guid: guid(),
                name: relativePath.split("/").pop() || relativePath,
                size: fileBlob.size,
                hash: undefined,
                createdAt: new Date().toISOString(),
                updatedAt: new Date().toISOString(),
              };
              await kitStore.execute("semio.kit.addFile", "system.loadKitFiles", file, fileBlob);
            }),
          );
        }
      });

      await Promise.all(filePromises);
    } catch (error) {}
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
      const kitApp = kitAppFactory(this, yKitApp, (fn: () => void) => this.yDoc.transact(fn), { kit });
      this.kitApps.set(kit, kitApp);
    });
    this.kitAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createDesignApp = (kit: Guid, design: Guid) => {
    const designAppFactory = resolveDesignAppStoreFactory();
    const designApp = designAppFactory(this, { kit, design });

    let designAppsMap = this.designApps.get(kit);
    if (!designAppsMap) {
      designAppsMap = new Map();
      this.designApps.set(kit, designAppsMap);
    }
    designAppsMap.set(design, designApp);
    this.designAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  change(diff: SketchpadDiff) {
    if (this.actor) {
      this.actor.send({ type: "CHANGE", diff });
      return;
    }
    this.yDoc.transact(() => {
      if (diff.navigationHistory !== undefined) {
        this.ySketchpad.set("navigationHistory", JSON.stringify(diff.navigationHistory));
      }
      if (diff.navigationHistoryIndex !== undefined) {
        this.ySketchpad.set("navigationHistoryIndex", diff.navigationHistoryIndex);
      }
      if (diff.navigation) {
        this.ySketchpad.set("navigation", diff.navigation);
      }
      if ("recentSearches" in diff) {
        this.ySketchpad.set("recentSearches", JSON.stringify(diff.recentSearches || []));
      }
      if ("recentFocusItems" in diff) {
        const current = JSON.parse((this.ySketchpad.get("recentFocusItems") as string) || "{}");
        this.ySketchpad.set("recentFocusItems", JSON.stringify({ ...current, ...(diff.recentFocusItems || {}) }));
      }
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.language !== undefined) {
        this.ySketchpad.set("language", diff.language);
      }
      if (diff.device) this.ySketchpad.set("device", JSON.stringify(diff.device));
      if (diff.expertise) this.ySketchpad.set("expertise", diff.expertise);
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.isFullscreen !== undefined) this.ySketchpad.set("isFullscreen", diff.isFullscreen);
      if (diff.isMobile !== undefined) this.ySketchpad.set("isMobile", diff.isMobile);
      if ("activeInteraction" in diff) this.ySketchpad.set("activeInteraction", diff.activeInteraction || "");
      if (diff.settings) {
        const current = JSON.parse((this.ySketchpad.get("settings") as string) || "{}");
        const merged = { ...current, apps: { ...current.apps, ...diff.settings.apps } };
        this.ySketchpad.set("settings", JSON.stringify(merged));
      }
      if (diff.panelSizes) {
        const current = JSON.parse((this.ySketchpad.get("panelSizes") as string) || "{}");
        this.ySketchpad.set("panelSizes", JSON.stringify({ ...current, ...diff.panelSizes }));
      }
      if (diff.hotkeyOverrides) {
        const current = JSON.parse((this.ySketchpad.get("hotkeyOverrides") as string) || "{}");
        this.ySketchpad.set("hotkeyOverrides", JSON.stringify({ ...current, ...diff.hotkeyOverrides }));
      }
      if ("activeHotkeySetting" in diff) this.ySketchpad.set("activeHotkeySetting", diff.activeHotkeySetting || "");
    });
  }

  deleteKit = (guid: Guid) => {
    const kitStore = this.kits.get(guid);
    if (kitStore) {
      this.yDoc.transact(() => {
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
      this.designAppDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  onKitCreated = (callback: () => void): Unsubscribe => {
    this.kitCreatedSubscribers.add(callback);
    return () => {
      this.kitCreatedSubscribers.delete(callback);
    };
  };

  onKitAppCreated = (callback: () => void): Unsubscribe => {
    this.kitAppCreatedSubscribers.add(callback);
    return () => {
      this.kitAppCreatedSubscribers.delete(callback);
    };
  };

  onDesignAppCreated = (callback: () => void): Unsubscribe => {
    this.designAppCreatedSubscribers.add(callback);
    return () => {
      this.designAppCreatedSubscribers.delete(callback);
    };
  };

  onKitDeleted = (callback: () => void): Unsubscribe => {
    this.kitDeletedSubscribers.add(callback);
    return () => {
      this.kitDeletedSubscribers.delete(callback);
    };
  };

  onKitAppDeleted = (callback: () => void): Unsubscribe => {
    this.kitAppDeletedSubscribers.add(callback);
    return () => {
      this.kitAppDeletedSubscribers.delete(callback);
    };
  };

  onDesignAppDeleted = (callback: () => void): Unsubscribe => {
    this.designAppDeletedSubscribers.add(callback);
    return () => {
      this.designAppDeletedSubscribers.delete(callback);
    };
  };

  onChanged = (subscribe: Subscribe): Unsubscribe => {
    if (this.actor) {
      const subscription = this.actor.subscribe(() => {
        subscribe(() => {});
      });
      return () => subscription.unsubscribe();
    }
    return createObserver(this.ySketchpad, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {
    if (this.actor) {
      const subscription = this.actor.subscribe(() => {
        subscribe(() => {});
      });
      return () => subscription.unsubscribe();
    }
    return createObserver(this.ySketchpad, subscribe, true);
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

    this.tutorialStoreInstance.checkCommandCompletion(command, origin, rest);
    const tutorialState = this.tutorialStoreInstance.snapshot();
    if (tutorialState.recordingState === "recording") {
      this.tutorialStoreInstance.recordEvent({
        type: "command",
        data: { command, origin, args: rest },
      });
    }

    if (command === "semio.sketchpad.createKit") {
      const kit = rest[0] as Kit;
      const local = rest[1] as boolean | undefined;
      const remote = rest[2] as boolean | undefined;
      this.createKit(kit, local, remote);
      return {} as T;
    }
    if (command === "semio.sketchpad.createKitApp") {
      const id = rest[0] as KitAppId;
      this.createKitApp(id.kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createDesignApp") {
      const id = rest[0] as DesignAppId;
      this.createDesignApp(id.kit, id.design);
      return {} as T;
    }
    if (command === "semio.sketchpad.importKit") {
      const Guid = rest[0] as Guid;
      const url = rest[1] as string;
      const kitStore = this.kits.get(Guid);
      if (kitStore) {
        await kitStore.execute("semio.kit.import", origin, url);
      }
      return {} as T;
    }
    if (command === "semio.sketchpad.freeze") {
      const completeState = this.dumpState();
      const stateJson = JSON.stringify(completeState, null, 2);
      const blob = new Blob([stateJson], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `semio-freeze-${new Date().toISOString().replace(/[:.]/g, "-")}.json`;
      a.click();
      URL.revokeObjectURL(url);
      return {} as T;
    }
    if (command === "semio.sketchpad.timetravel") {
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".json";
      input.onchange = async (e: Event) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) {
          const text = await file.text();
          const state = JSON.parse(text) as CompleteState;
          this.loadState(state);
        }
      };
      input.click();
      return {} as T;
    }
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      throw new Error(`Command "${command}" not found in sketchpad store`);
    }
    const context: SketchpadCommandContext = {
      sketchpad: this.snapshot(),
      origin,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }

  registerCommand(command: string, callback: (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult, metadata?: { user?: boolean }): Disposable {
    this.commandRegistry.set(command, callback);
    if (metadata) {
      this.commandMetadata.set(command, metadata);
    }
    return () => {
      this.commandRegistry.delete(command);
      this.commandMetadata.delete(command);
    };
  }

  isUserCommand(command: string): boolean {
    const metadata = this.commandMetadata.get(command);
    return metadata?.user !== false;
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
      isUserCommand: this.isUserCommand.bind(this),
    };
  }

  dumpState(): CompleteState {
    const sketchpad = this.snapshot();

    const kits = Array.from(this.kits.entries()).map(([guid, kitStore]) => {
      const kitMetadataArray = this.yKits.toArray();
      const kitMetadata = kitMetadataArray.find((m) => m.get("guid") === guid);
      return {
        guid,
        local: kitMetadata?.get("local") === true,
        remote: kitMetadata?.get("remote") === true,
        kit: kitStore.snapshot(),
      };
    });

    const kitApps: Record<string, any> = {};
    this.kitApps.forEach((app, guid) => {
      kitApps[guid] = app.snapshot();
    });

    const typeApps: Record<string, any> = {};
    this.typeApps.forEach((app, key) => {
      typeApps[key] = app.snapshot();
    });

    const qualityApps: Record<string, any> = {};
    this.qualityApps.forEach((app, key) => {
      qualityApps[key] = app.snapshot();
    });

    const designApps: Record<string, Record<string, any>> = {};
    this.designApps.forEach((designMap, kitGuid) => {
      designApps[kitGuid] = {};
      designMap.forEach((app, designGuid) => {
        designApps[kitGuid][designGuid] = app.snapshot();
      });
    });

    const home = this.homeStore?.snapshot();
    const tutorials = this.tutorialStoreInstance.snapshot();

    return {
      sketchpad,
      kits,
      kitApps,
      typeApps,
      qualityApps,
      designApps,
      home,
      tutorials,
    };
  }

  loadState(state: CompleteState): void {
    this.yDoc.transact(() => {
      this.kits.clear();
      this.kitApps.clear();
      this.typeApps.clear();
      this.qualityApps.clear();
      this.designApps.clear();

      this.yKits.delete(0, this.yKits.length);
      this.yKitApps.forEach((_, key) => this.yKitApps.delete(key));

      this.ySketchpad.set("navigation", state.sketchpad.navigation);
      this.ySketchpad.set("navigationHistory", JSON.stringify(state.sketchpad.navigationHistory));
      this.ySketchpad.set("navigationHistoryIndex", state.sketchpad.navigationHistoryIndex);
      this.ySketchpad.set("recentSearches", JSON.stringify(state.sketchpad.recentSearches));
      this.ySketchpad.set("recentFocusItems", JSON.stringify(state.sketchpad.recentFocusItems));
      this.ySketchpad.set("theme", state.sketchpad.theme);
      this.ySketchpad.set("device", JSON.stringify(state.sketchpad.device));
      this.ySketchpad.set("expertise", state.sketchpad.expertise);
      this.ySketchpad.set("mode", state.sketchpad.mode);
      this.ySketchpad.set("settings", JSON.stringify(state.sketchpad.settings));
      this.ySketchpad.set("panelSizes", JSON.stringify(state.sketchpad.panelSizes));
      this.ySketchpad.set("isFullscreen", state.sketchpad.isFullscreen);
      this.ySketchpad.set("isMobile", state.sketchpad.isMobile);
      if (state.sketchpad.activeInteraction) {
        this.ySketchpad.set("activeInteraction", state.sketchpad.activeInteraction);
      }
      if (state.sketchpad.hotkeyOverrides) {
        this.ySketchpad.set("hotkeyOverrides", JSON.stringify(state.sketchpad.hotkeyOverrides));
      }
      if (state.sketchpad.activeHotkeySetting) {
        this.ySketchpad.set("activeHotkeySetting", state.sketchpad.activeHotkeySetting);
      }

      state.kits.forEach(({ guid, local, remote, kit }) => {
        this.createKit(kit, local, remote);
        const kitStore = this.kit(kit.guid);
        this.loadKitFilesFromPublic(kit.guid, kitStore);
      });

      Object.entries(state.kitApps).forEach(([guid, appState]) => {
        this.createKitApp(guid);
        const kitApp = this.kitApps.get(guid);
        if (kitApp && kitApp.loadState) {
          kitApp.loadState(appState);
        }
      });

      Object.entries(state.typeApps).forEach(([key, appState]) => {
        const [kitGuid, typeGuid] = key.split(":");
        this.createTypeApp(kitGuid, typeGuid);
        const typeApp = this.typeApps.get(key);
        if (typeApp && typeApp.loadState) {
          typeApp.loadState(appState);
        }
      });

      Object.entries(state.qualityApps).forEach(([key, appState]) => {
        const [kitGuid, qualityGuid] = key.split(":");
        this.createQualityApp(kitGuid, qualityGuid);
        const qualityApp = this.qualityApps.get(key);
        if (qualityApp && qualityApp.loadState) {
          qualityApp.loadState(appState);
        }
      });

      Object.entries(state.designApps).forEach(([kitGuid, designs]) => {
        Object.entries(designs).forEach(([designGuid, appState]) => {
          this.createDesignApp(kitGuid, designGuid);
          const designApp = this.designApps.get(kitGuid)?.get(designGuid);
          if (designApp && designApp.loadState) {
            designApp.loadState(appState);
          }
        });
      });

      if (state.home && this.homeStore && this.homeStore.loadState) {
        this.homeStore.loadState(state.home);
      }

      if (state.tutorials && this.tutorialStoreInstance.loadState) {
        this.tutorialStoreInstance.loadState(state.tutorials);
      }
    });
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
      Array.from(this.kitApps.values())
        .map((kitAppStore) => {
          if (typeof kitAppStore?.id === "function") {
            return kitAppStore.id();
          }
          return null;
        })
        .filter((id): id is KitAppId => id !== null),
    );
  }

  home(): HomeStoreInstance {
    if (!this.homeStore) {
      const homeFactory = resolveHomeStoreFactory();
      this.homeStore = homeFactory(this);
    }
    return this.homeStore;
  }

  tutorialStore(): TutorialStore {
    return this.tutorialStoreInstance;
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
    return Array.from(this.kitApps.values())
      .map((kitAppStore) => {
        if (typeof kitAppStore?.id === "function") {
          return kitAppStore.id();
        }
        return null;
      })
      .filter((id): id is KitAppId => id !== null);
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
    const typeAppFactory = resolveTypeAppStoreFactory();
    const typeApp = typeAppFactory(this, id);
    this.typeApps.set(key, typeApp);
    this.typeAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteTypeApp = (kit: Guid, type: Guid) => {
    const key = `${kit}:${type}`;
    const typeApp = this.typeApps.get(key);
    if (typeApp) {
      this.typeApps.delete(key);
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
    const qualityAppFactory = resolveQualityAppStoreFactory();
    const qualityApp = qualityAppFactory(this, Guid);
    this.qualityApps.set(key, qualityApp);
    this.qualityAppCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  deleteQualityApp = (kit: Guid, quality: Guid) => {
    const key = `${kit}:${quality}`;
    const qualityApp = this.qualityApps.get(key);
    if (qualityApp) {
      this.qualityApps.delete(key);
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
    if (this.persistence) {
      await new Promise<void>((resolve) => {
        this.persistence!.once("synced", () => resolve());
      });
    }

    const kitMetadataArray = this.yKits.toArray();

    for (const kitMetadata of kitMetadataArray) {
      const kitGuid = kitMetadata.get("guid") as string;
      const local = kitMetadata.get("local") as boolean;
      const remote = kitMetadata.get("remote") as boolean;

      if (this.kits.has(kitGuid)) continue;

      if (local && typeof indexedDB !== "undefined") {
        try {
          const yDoc = new Y.Doc();
          const persistence = new IndexeddbPersistence(`semio-kit-${kitGuid}`, yDoc);

          await new Promise<void>((resolve) => {
            persistence.on("synced", () => resolve());
          });

          const yKit = yDoc.getMap();
          const conceptGuids = yKit.get("concepts") as string[] | undefined;
          const yConcepts = yDoc.getArray("concepts");
          const concepts =
            yConcepts.length > 0
              ? Array.from(yConcepts).map((yConcept: any) => {
                  const yMap = yConcept[0] as Y.Map<any>;
                  const concept: Concept = {
                    guid: yMap.get("guid") as string,
                    name: yMap.get("name") as string,
                  };
                  const description = yMap.get("description") as string | undefined;
                  if (description) concept.description = description;
                  const icon = yMap.get("icon") as string | undefined;
                  if (icon) concept.icon = icon;
                  const yAttrs = yMap.get("attributes") as Y.Array<any> | undefined;
                  if (yAttrs && yAttrs.length > 0) {
                    const attributes = Array.from(yAttrs).map((yAttr: any) => {
                      const attrMap = yAttr[0] as Y.Map<any>;
                      const attribute: Attribute = { guid: attrMap.get("guid") as string, key: attrMap.get("key") as string };
                      const value = attrMap.get("value") as string | undefined;
                      if (value) attribute.value = value;
                      const definition = attrMap.get("definition") as string | undefined;
                      if (definition) attribute.definition = definition;
                      return attribute;
                    });
                    if (attributes.length > 0) concept.attributes = attributes;
                  }
                  return concept;
                })
              : conceptGuids?.map((g) => ({ guid: g, name: g }));
          const kit: Kit = {
            guid: yKit.get("guid") as string,
            name: yKit.get("name") as string,
            version: yKit.get("version") as string,
            remote: yKit.get("remote") as string,
            homepage: yKit.get("homepage") as string,
            license: yKit.get("license") as string,
            preview: yKit.get("preview") as string,
            concepts,
            icon: yKit.get("icon") as string,
            image: yKit.get("image") as string,
            description: yKit.get("description") as string,
            createdAt: yKit.get("createdAt") as string | undefined,
            updatedAt: yKit.get("updatedAt") as string | undefined,
            types: [],
            designs: [],
            files: [],
            qualities: [],
            authors: [],
            attributes: [],
          };

          persistence.destroy();

          const kitStore = new KitStore(this, kit, local, remote, this.remote);
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

SketchpadStore._loadModules();

let stores: Map<Guid, SketchpadStore>;
if (import.meta.hot?.data.stores) {
  stores = import.meta.hot.data.stores;
} else {
  stores = new Map();
  if (import.meta.hot) {
    import.meta.hot.data.stores = stores;
  }
}

let actors: Map<Guid, SketchpadActorRef>;
if (import.meta.hot?.data.actors) {
  actors = import.meta.hot.data.actors;
} else {
  actors = new Map();
  if (import.meta.hot) {
    import.meta.hot.data.actors = actors;
  }
}

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);

export const SketchpadScopeProvider = (props: { id?: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents; initialState?: ExtendedInitialState; importKitUrls?: string[]; children: React.ReactNode }) => {
  const id = useMemo(() => props.id || guid(), [props.id]);
  const [configsReady, setConfigsReady] = useState(false);

  useEffect(() => {
    loadAppConfigs().then(() => setConfigsReady(true));
  }, []);

  if (!stores.has(id)) {
    const store = new SketchpadStore(id, props?.remote, props?.initialState);
    stores.set(id, store);

    const actor = createSketchpadActor({ id, initialState: mergeSketchpadState(mergeSketchpadState(store.snapshot(), readSketchpadStateFromLocalStorage(id)), toSketchpadInitialState(props?.initialState)) });
    actor.start();
    actors.set(id, actor);
    store.setActor(actor);

    if (typeof window !== "undefined") {
      (window as any).__SEMIO_STORE__ = store;
      (window as any).__SEMIO_ACTOR__ = actor;
      (window as any).__piecesMetadata = piecesMetadata;
    }
  }

  const actor = actors.get(id)!;
  const store = stores.get(id)!;

  useEffect(() => {
    if (!configsReady || !props.importKitUrls || props.importKitUrls.length === 0) return;

    const doImportKits = async () => {
      for (const url of props.importKitUrls!) {
        try {
          const { kit, files: importedFiles } = await importKit(url);

          await store.execute("semio.sketchpad.createKit", "semio.sketchpad.importKitUrls", kit, false, false);

          if (store.hasKit(kit.guid)) {
            const kitStore = store.kit(kit.guid);
            await kitStore.storeFileBlobs(importedFiles);
          }
        } catch (error) {
          console.error(`[Sketchpad] Failed to auto-import kit from ${url}:`, error);
        }
      }
    };

    doImportKits();
  }, [configsReady, props.importKitUrls, store]);

  return React.createElement(SketchpadScopeContext.Provider, { value: { id, remote: props.remote, onWindowEvents: props.onWindowEvents } }, React.createElement(SketchpadActorContext.Provider, { value: actor }, configsReady ? props.children : null));
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
  return useSync<SketchpadState, T>(useSketchpadStore(id), selector ? selector : (identitySelector as any));
}

export function useNavigation(): string {
  const location = useLocation();
  return location.pathname;
}

export function useAppType(): AppKind {
  const navigation = useNavigation();
  const apps = useSyncExternalStore(
    useCallback((cb) => appRegistry.subscribe(cb), []),
    () => appRegistry.getAllApps(),
  );

  return useMemo(() => {
    const pathParts = navigation.split("/").filter((p: string) => p);
    const app = appRegistry.getAppForPath(pathParts);
    return app?.id ?? "home";
  }, [navigation, apps]);
}

export function getAppTypeFromPath(path: string): AppKind {
  const pathParts = path.split("/").filter((p) => p);
  const app = appRegistry.getAppForPath(pathParts);
  return app?.id ?? "home";
}

export function useTheme(): HookResult<Theme> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectTheme(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "SET_THEME" as const, theme: Theme.LIGHT }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (theme: Theme) => actor.send({ type: "SET_THEME", theme });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useLanguage(): HookResult<string> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectLanguage(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "SET_LANGUAGE" as const, language: "en" }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (language: string) => actor.send({ type: "SET_LANGUAGE", language });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useDevice(): HookResult<Device> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectDevice(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "SET_DEVICE" as const, device: "desktop" as Device }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (device: Device) => actor.send({ type: "SET_DEVICE", device });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useMode(): HookResult<Mode> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectMode(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "SET_MODE" as const, mode: Mode.USER }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (mode: Mode) => actor.send({ type: "SET_MODE", mode });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useExpertise(): HookResult<Expertise> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectExpertise(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "SET_EXPERTISE" as const, expertise: Expertise.NORMAL }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (expertise: Expertise) => actor.send({ type: "SET_EXPERTISE", expertise });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useFullscreen(): HookResult<boolean> {
  const actor = useSketchpadActor();
  const value = useSelector(actor, (snapshot) => selectIsFullscreen(snapshot.context));
  const canSetEvent = useMemo(() => ({ type: "TOGGLE_FULLSCREEN" as const }), []);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (_value: boolean) => actor.send({ type: "TOGGLE_FULLSCREEN" });
  }, [actor, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTooltip(): (key: string) => string | undefined {
  const [expertise] = useExpertise();
  return (key: string) => {
    if (expertise === Expertise.EXPERT) return undefined;
    return key;
  };
}

export function useSemioTooltip() {
  const [mode] = useMode();
  return { mode };
}

export function useIsNavbarExpanded(): boolean {
  const [device] = useDevice();
  return typeof device === "object" ? device.isNavbarExpanded : false;
}

export function useIsFooterExpanded(): boolean {
  const [device] = useDevice();
  return typeof device === "object" ? device.isFooterExpanded : false;
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

// #region 🔖XState Hooks

export function useSketchpadActor(): SketchpadActorRef {
  const actor = useContext(SketchpadActorContext);
  if (!actor) {
    throw new Error("useSketchpadActor must be used within a SketchpadScopeProvider");
  }
  return actor;
}

export function useSketchpadActorSafe(): SketchpadActorRef | null {
  return useContext(SketchpadActorContext);
}

export function useSketchpadSelector<T>(selector: (snapshot: ReturnType<SketchpadActorRef["getSnapshot"]>) => T): T {
  const actor = useSketchpadActor();
  return useSelector(actor, selector);
}

export function useSketchpadSnapshot(): SketchpadState {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectSnapshot(snapshot.context));
}

export function useSketchpadCan(event: SketchpadEvent): boolean {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => {
    const handler = getEventHandler(event.type);
    if (!handler) return false;
    if (handler.guard && !handler.guard(snapshot.context as any, event as any)) return false;
    return true;
  });
}

export function useNavigationXState(): string {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectNavigation(snapshot.context));
}

export function useThemeXState(): Theme {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectTheme(snapshot.context));
}

export function useLanguageXState(): string {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectLanguage(snapshot.context));
}

export function useExpertiseXState(): Expertise {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectExpertise(snapshot.context));
}

export function useModeXState(): Mode {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectMode(snapshot.context));
}

export function useDeviceXState(): Device {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectDevice(snapshot.context));
}

export function useIsFullscreenXState(): boolean {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectIsFullscreen(snapshot.context));
}

export function usePanelSizesXState(): PanelSizes {
  const actor = useSketchpadActor();
  return useSelector(actor, (snapshot) => selectPanelSizes(snapshot.context));
}

export function useSketchpadActions() {
  const actor = useSketchpadActor();

  return useMemo(
    () => ({
      navigate: (path: string) => actor.send({ type: "NAVIGATE", path }),
      navigateBack: () => actor.send({ type: "NAVIGATE_BACK" }),
      navigateForward: () => actor.send({ type: "NAVIGATE_FORWARD" }),
      setTheme: (theme: Theme) => actor.send({ type: "SET_THEME", theme }),
      setLanguage: (language: string) => actor.send({ type: "SET_LANGUAGE", language }),
      setExpertise: (expertise: Expertise) => actor.send({ type: "SET_EXPERTISE", expertise }),
      setMode: (mode: Mode) => actor.send({ type: "SET_MODE", mode }),
      setDevice: (device: Device) => actor.send({ type: "SET_DEVICE", device }),
      toggleFullscreen: () => actor.send({ type: "TOGGLE_FULLSCREEN" }),
      setPanelSize: (panel: keyof PanelSizes, size: number) => actor.send({ type: "SET_PANEL_SIZE", panel, size }),
      change: (diff: SketchpadDiff) => actor.send({ type: "CHANGE", diff }),
    }),
    [actor],
  );
}

export function useXStateField<T, TEvent extends { type: string }>(value: T, canEvent: TEvent, createEvent: (next: T) => TEvent): Field<T> {
  const actor = useSketchpadActor();
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canEvent as Parameters<typeof snapshot.can>[0]));
  return useMemo(() => createFieldValue(value, (next: T) => actor.send(createEvent(next) as Parameters<typeof actor.send>[0]), canSet), [value, actor, createEvent, canSet]);
}

export function useXStateFieldWithScope<T, TEvent extends { type: string }>(value: T, canEvent: TEvent, createEvent: (next: T) => TEvent, hasScope: boolean): Field<T> {
  const actor = useSketchpadActor();
  const canSetFromSnapshot = useSelector(actor, (snapshot) => snapshot.can(canEvent as Parameters<typeof snapshot.can>[0]));
  const canSet = canSetFromSnapshot || hasScope;
  return useMemo(() => createFieldValue(value, (next: T) => actor.send(createEvent(next) as Parameters<typeof actor.send>[0]), canSet), [value, actor, createEvent, canSet]);
}

export function useXStateAction<TEvent extends { type: string }>(canEvent: TEvent, event: TEvent): ActionField {
  const actor = useSketchpadActor();
  const canExecute = useSelector(actor, (snapshot) => snapshot.can(canEvent as Parameters<typeof snapshot.can>[0]));
  return useMemo(() => createActionValue(() => actor.send(event as Parameters<typeof actor.send>[0]), canExecute), [actor, event, canExecute]);
}

// #endregion 🔖XState Hooks

export function useDesignAppXState(kitGuid: Guid, designGuid: Guid): DesignAppState {
  const actor = useSketchpadActor();
  const selector = useMemo(() => createDesignAppSelector(kitGuid, designGuid), [kitGuid, designGuid]);
  return useSelector(actor, selector);
}

export function useTypeAppXState(kitGuid: Guid, typeGuid: Guid): TypeAppState {
  const actor = useSketchpadActor();
  const selector = useMemo(() => createTypeAppSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  return useSelector(actor, selector);
}

export function useKitAppXState(kitGuid: Guid): KitAppState {
  const actor = useSketchpadActor();
  const selector = useMemo(() => createKitAppSelector(kitGuid), [kitGuid]);
  return useSelector(actor, selector);
}

export function useHomeApp(): HomeAppState {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeApp);
}

export function useHomePanelVisibility(): PanelVisibility {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomePanelVisibility);
}

export function useHomeSelection(): HomeAppSelection | undefined {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeSelection);
}

export function useHomeHover(): { kits?: Guid[] } | undefined {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeHover);
}

export function useHomeSortColumn(): string | undefined {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeSortColumn);
}

export function useHomeSortDirection(): "asc" | "desc" | undefined {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeSortDirection);
}

export function useHomeLoadingKits(): Array<{ tempGuid: string; name: string }> {
  const actor = useSketchpadActor();
  return useSelector(actor, selectHomeLoadingKits);
}

export function useKitImportOperations(): Array<{
  operationId: string;
  kitName: string;
  status: "pending" | "running" | "completed" | "failed";
  error?: string;
}> {
  const actor = useSketchpadActor();
  return useSelector(actor, selectKitImportOperations);
}

export function useHomeCommands() {
  const actor = useSketchpadActor();
  return useMemo(
    () => ({
      selectKit: (origin: string, kitGuid: Guid) => {
        actor.send({ type: "HOME.CLEAR_SELECTION" } as any);
        actor.send({ type: "HOME.SELECT_KIT", guid: kitGuid } as any);
      },
      selectKits: (origin: string, kitGuids: Guid[]) => {
        actor.send({ type: "HOME.CLEAR_SELECTION" } as any);
        for (const guid of kitGuids) {
          actor.send({ type: "HOME.SELECT_KIT", guid } as any);
        }
      },
      deselectKit: (origin: string, kitGuid: Guid) => actor.send({ type: "HOME.DESELECT_KIT", guid: kitGuid } as any),
      addKitToSelection: (origin: string, kitGuid: Guid) => actor.send({ type: "HOME.SELECT_KIT", guid: kitGuid } as any),
      removeKitFromSelection: (origin: string, kitGuid: Guid) => actor.send({ type: "HOME.DESELECT_KIT", guid: kitGuid } as any),
      clearSelection: () => actor.send({ type: "HOME.CLEAR_SELECTION" } as any),
      deselectAll: (origin: string) => actor.send({ type: "HOME.CLEAR_SELECTION" } as any),
      hoverKit: (origin: string, kitGuid: Guid) => actor.send({ type: "HOME.SET_HOVER", kits: [kitGuid] } as any),
      clearHover: (origin: string) => actor.send({ type: "HOME.CLEAR_HOVER", origin } as any),
      setSortColumn: (origin: string, column: string) => actor.send({ type: "HOME.SET_SORT_COLUMN", column } as any),
      setSortDirection: (origin: string, direction: "asc" | "desc") => actor.send({ type: "HOME.SET_SORT_DIRECTION", direction } as any),
      toggleSort: (origin: string, column: string) => {
        actor.send({ type: "HOME.SET_SORT_COLUMN", column } as any);
      },
      togglePanel: (_origin: string, panel: keyof PanelVisibility) => actor.send({ type: "HOME.TOGGLE_PANEL", panel } as any),
      addLoadingKit: (tempGuid: string, name: string) => actor.send({ type: "HOME.ADD_LOADING_KIT", tempGuid, name } as any),
      removeLoadingKit: (tempGuid: string) => actor.send({ type: "HOME.REMOVE_LOADING_KIT", tempGuid } as any),
    }),
    [actor],
  );
}

export function useKitShallows(): KitShallow[] {
  const store = useSketchpadStore();
  return useSyncExternalStore(
    (onStoreChange) => {
      const unsubscribeCreated = store.onKitCreated(onStoreChange);
      const unsubscribeDeleted = store.onKitDeleted(onStoreChange);
      const unsubscribers = store.kitShallows().map((kitShallow) => {
        const kitStore = store.kit(kitShallow.guid);
        return kitStore.onChanged((cb: () => void) => {
          cb();
          onStoreChange();
          return () => {};
        });
      });
      return () => {
        unsubscribeCreated();
        unsubscribeDeleted();
        unsubscribers.forEach((unsub) => unsub());
      };
    },
    () => store.kitShallows(),
  );
}

export function useHasKit(kitGuid: string): boolean {
  const store = useSketchpadStore();
  return useSyncExternalStore(
    (onStoreChange) => {
      const unsubscribeCreated = store.onKitCreated(onStoreChange);
      const unsubscribeDeleted = store.onKitDeleted(onStoreChange);
      return () => {
        unsubscribeCreated();
        unsubscribeDeleted();
      };
    },
    () => store.hasKit(kitGuid),
  );
}

export function useKitKind(kitGuid: string): "temporary" | "local" | "remote" | undefined {
  const store = useSketchpadStore();
  const hasKit = useHasKit(kitGuid);
  return useSyncExternalStore(
    (onStoreChange) => {
      if (!hasKit) return () => {};
      const kitStore = store.kit(kitGuid);
      return kitStore.onChanged((cb: () => void) => {
        cb();
        onStoreChange();
        return () => {};
      });
    },
    () => {
      if (!hasKit) return undefined;
      const kitStore = store.kit(kitGuid);
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) return "remote";
      if (kitStore.isLocallyPersisted) return "local";
      return "temporary";
    },
  );
}

export function useGetKitKind(): (kitGuid: string) => "temporary" | "local" | "remote" | undefined {
  const store = useSketchpadStore();
  return useCallback(
    (kitGuid: string) => {
      if (!store.hasKit(kitGuid)) return undefined;
      const kitStore = store.kit(kitGuid);
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) return "remote";
      if (kitStore.isLocallyPersisted) return "local";
      return "temporary";
    },
    [store],
  );
}

export function useFilteredKitShallows(kind?: "temporary" | "local" | "remote"): KitShallow[] {
  const store = useSketchpadStore();
  const allKits = useKitShallows();
  return useMemo(() => {
    if (!kind) return allKits;
    return allKits.filter((k) => {
      const ks = store.kit(k.guid);
      const kKind = ks.isLocallyPersisted && ks.isRemotelySynced ? "remote" : ks.isLocallyPersisted ? "local" : "temporary";
      return kKind === kind;
    });
  }, [allKits, kind, store]);
}

export function usePanelSizes(): PanelSizes {
  return useSketchpad((state) => state.panelSizes) as PanelSizes;
}

export function useSettings(): { apps: Record<string, any> } {
  return useSketchpad((state) => state.settings) as { apps: Record<string, any> };
}

export function useAppPanelVisibility(): PanelVisibility {
  const navigation = useNavigation();
  const appType = useAppType();
  const actor = useSketchpadActor();

  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const itemGuid = pathMatch?.[3];

  const docsPanelVisibility = useSyncExternalStore(subscribeDocsPanelVisibility, getDocsPanelVisibilitySnapshot, getDocsPanelVisibilitySnapshot);

  const selector = useMemo(() => {
    switch (appType) {
      case "home":
        return selectHomePanelVisibility;
      case "kit":
        if (kitGuid) return createKitPanelVisibilitySelector(kitGuid);
        return () => defaultPanelVisibility;
      case "design":
        if (kitGuid && itemGuid) return createDesignPanelVisibilitySelector(kitGuid, itemGuid);
        return () => defaultPanelVisibility;
      case "type":
        if (kitGuid && itemGuid) return createTypePanelVisibilitySelector(kitGuid, itemGuid);
        return () => defaultPanelVisibility;
      case "quality":
        if (kitGuid && itemGuid) return createQualityPanelVisibilitySelector(kitGuid, itemGuid);
        return () => defaultPanelVisibility;
      case "docs":
        return () => docsPanelVisibility;
      case "feedback":
        return (snapshot: any) => snapshot.context.feedbackApp?.panelVisibility ?? defaultPanelVisibility;
      default:
        return () => defaultPanelVisibility;
    }
  }, [appType, kitGuid, itemGuid, docsPanelVisibility]);

  const panelVisibility = useSelector(actor, selector);

  if (appType === "docs") {
    return docsPanelVisibility;
  }

  return panelVisibility;
}

export function useAppCommands() {
  const navigation = useNavigation();
  const appType = useAppType();
  const store = useSketchpadStore();
  const actor = useSketchpadActor();

  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const itemGuid = pathMatch?.[3];

  return useMemo(() => {
    switch (appType) {
      case "home":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            actor.send({ type: "HOME.TOGGLE_PANEL", panel: panelKey } as any);
          },
          execute: (_origin: string, _command: string, ..._args: any[]) => {},
        };
      case "kit":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            if (kitGuid) {
              actor.send({ type: "KIT.TOGGLE_PANEL", kitGuid, panel: panelKey } as any);
            }
          },
          execute: (origin: string, command: string, ...args: any[]) => {
            if (!kitGuid) return;
            try {
              const app = store.kitApp(kitGuid);
              return app?.execute(command, origin, ...args);
            } catch (e) {}
          },
        };
      case "design":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            if (kitGuid && itemGuid) {
              actor.send({ type: "DESIGN.TOGGLE_PANEL", kitGuid, designGuid: itemGuid, panel: panelKey } as any);
            }
          },
          execute: (origin: string, command: string, ...args: any[]) => {
            if (!kitGuid || !itemGuid) return;
            try {
              const app = store.designApp(kitGuid, itemGuid);
              return app?.execute(command, origin, ...args);
            } catch (e) {}
          },
        };
      case "type":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            if (kitGuid && itemGuid) {
              actor.send({ type: "TYPE.TOGGLE_PANEL", kitGuid, typeGuid: itemGuid, panel: panelKey } as any);
            }
          },
          execute: (origin: string, command: string, ...args: any[]) => {
            if (!kitGuid || !itemGuid) return;
            try {
              const app = store.typeApp(kitGuid, itemGuid);
              return app?.execute(command, origin, ...args);
            } catch (e) {}
          },
        };
      case "quality":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            if (kitGuid && itemGuid) {
              actor.send({ type: "QUALITY.TOGGLE_PANEL", kitGuid, qualityGuid: itemGuid, panel: panelKey } as any);
            }
          },
          execute: (origin: string, command: string, ...args: any[]) => {
            if (!kitGuid || !itemGuid) return;
            try {
              const app = store.qualityApp(kitGuid, itemGuid);
              return app?.execute(command, origin, ...args);
            } catch (e) {}
          },
        };
      case "docs":
        return {
          togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => {
            updateDocsPanelVisibilityState((prev) => ({
              ...prev,
              [panelKey]: !prev[panelKey],
            }));
          },
          execute: (_origin: string, _command: string, ..._args: any[]) => {},
        };
      default:
        return {
          togglePanel: (_origin: string, _panelKey: keyof PanelVisibility) => {},
          execute: (_origin: string, _command: string, ..._args: any[]) => {},
        };
    }
  }, [store, appType, kitGuid, itemGuid, actor]);
}

export function useUpdateRecentSearches() {
  const store = useSketchpadStore();
  return useCallback(
    (searches: string[]) => {
      store.change({ recentSearches: searches });
    },
    [store],
  );
}

export function useUpdateRecentFocusItems() {
  const store = useSketchpadStore();
  return useCallback(
    (appType: string, items: string[]) => {
      store.change({ recentFocusItems: { [appType]: items } });
    },
    [store],
  );
}

export function useNavigate() {
  const store = useSketchpadStore();
  const reactNavigate = useReactNavigate();
  return useCallback(
    (to: string | number, options?: { replace?: boolean; state?: any }) => {
      if (!reactNavigate) return;
      if (typeof to === "number") {
        reactNavigate(to);
        return;
      }
      const fullPath = to + (options?.state?.search || "");
      store.execute("semio.sketchpad.addNavigation", "semio.sketchpad.navigation", fullPath);
      reactNavigate(to, options);
    },
    [store, reactNavigate],
  );
}

export function useSketchpadCommands() {
  const store = useSketchpadStore();
  const navigate = useNavigate();
  const reactNavigate = useReactNavigate();
  return useMemo(
    () => ({
      setTheme: (origin: string, theme: Theme) => store.execute("semio.sketchpad.setTheme", origin, theme),
      setLanguage: (origin: string, language: string) => store.execute("semio.sketchpad.setLanguage", origin, language),
      setDevice: (origin: string, device: Device) => store.execute("semio.sketchpad.setDevice", origin, device),
      setExpertise: (origin: string, expertise: Expertise) => store.execute("semio.sketchpad.setExpertise", origin, expertise),
      setMode: (origin: string, mode: Mode) => store.execute("semio.sketchpad.setMode", origin, mode),
      exportState: (origin: string) => store.execute("semio.sketchpad.exportState", origin),
      setState: (origin: string, state: Partial<SketchpadState>) => store.execute("semio.sketchpad.setState", origin, state),
      toggleFullscreen: (origin: string) => store.execute("semio.sketchpad.toggleFullscreen", origin),
      toggleNavbarExpanded: (origin: string) => store.execute("semio.sketchpad.toggleNavbarExpanded", origin),
      toggleFooterExpanded: (origin: string) => store.execute("semio.sketchpad.toggleFooterExpanded", origin),
      setIsMobile: (origin: string, isMobile: boolean) => store.execute("semio.sketchpad.setIsMobile", origin, isMobile),
      setActiveInteraction: (origin: string, interactionId?: string) => store.execute("semio.sketchpad.setActiveInteraction", origin, interactionId),
      syncNavigation: (origin: string, path: string) => store.execute("semio.sketchpad.syncNavigation", origin, path),
      createKit: (origin: string, kit: Kit, local?: boolean, remote?: boolean) => store.execute("semio.sketchpad.createKit", origin, kit, local, remote),
      createKitApp: (origin: string, kitAppId: KitAppId) => store.execute("semio.sketchpad.createKitApp", origin, kitAppId),
      createDesignApp: (origin: string, designAppId: DesignAppId) => store.execute("semio.sketchpad.createDesignApp", origin, designAppId),
      navigateToKit: (kit: Guid, search?: string) => {
        const path = `/kits/${kit}${search ? (search.startsWith("?") ? search : `?${search}`) : ""}`;

        const globalNavigate = (window as any).__SEMIO_NAVIGATE__;
        if (globalNavigate) {
          globalNavigate(path);
        } else {
          navigate(path);
        }
      },
      navigateToDesign: (kit: Guid, design: Guid) => {
        const path = `/kits/${kit}/designs/${design}`;

        const globalNavigate = (window as any).__SEMIO_NAVIGATE__;
        if (globalNavigate) {
          globalNavigate(path);
        } else {
          navigate(path);
        }
      },
      navigateToType: (kit: Guid, type: Guid) => {
        const path = `/kits/${kit}/types/${type}`;

        const globalNavigate = (window as any).__SEMIO_NAVIGATE__;
        if (globalNavigate) {
          globalNavigate(path);
        } else {
          navigate(path);
        }
      },
      navigateToQuality: (kit: Guid, quality: Guid) => {
        const path = `/kits/${kit}/qualities/${quality}`;

        const globalNavigate = (window as any).__SEMIO_NAVIGATE__;
        if (globalNavigate) {
          globalNavigate(path);
        } else {
          navigate(path);
        }
      },
      navigateBack: (origin: string) => {
        store.execute("semio.sketchpad.navigateBack", origin);
        const state = store.snapshot();
        const targetPath = state.navigationHistory[state.navigationHistoryIndex];
        if (targetPath) {
          navigate(targetPath);
        }
      },
      navigateForward: (origin: string) => {
        store.execute("semio.sketchpad.navigateForward", origin);
        const state = store.snapshot();
        const targetPath = state.navigationHistory[state.navigationHistoryIndex];
        if (targetPath) {
          navigate(targetPath);
        }
      },
      setPanelSize: (origin: string, key: string, value: number) => {
        const state = store.snapshot();
        const currentSizes = state.panelSizes || {};
        const newSizes = { ...currentSizes, [key]: value };
        store.execute("semio.sketchpad.setState", origin, {
          panelSizes: newSizes,
        });
      },
      storeKitFileBlobs: async (kitGuid: Guid, files: Map<string, Blob>) => {
        if (!store.hasKit(kitGuid)) return;
        const kitStore = store.kit(kitGuid);
        await kitStore.storeFileBlobs(files);
      },
      getKitSnapshot: (kitGuid: Guid): Kit | null => {
        if (!store.hasKit(kitGuid)) return null;
        return store.kit(kitGuid).snapshot();
      },
    }),
    [store, navigate, reactNavigate],
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
        return kitStore.onChanged((cb: () => void) => {
          cb();
          onStoreChange();
          return () => {};
        });
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

export function useKitCommandsById(kitGuid?: string) {
  const store = useSketchpadStore();
  return useMemo(() => {
    if (!kitGuid || !store.hasKit(kitGuid)) return null;
    const kitStore = store.kit(kitGuid);
    return {
      importKit: (origin: string, url: string) => kitStore.execute("semio.kit.import", origin, url),
      exportKit: (origin: string) => kitStore.execute("semio.kit.export", origin),
      createAuthor: (origin: string, author: Author) => kitStore.execute("semio.kit.createAuthor", origin, author),
      updateAuthor: (origin: string, authorId: string, authorDiff: AuthorDiff) => kitStore.execute("semio.kit.updateAuthor", origin, authorId, authorDiff),
      deleteAuthor: (origin: string, authorId: string) => kitStore.execute("semio.kit.deleteAuthor", origin, authorId),
      createType: (origin: string, type: Type) => kitStore.execute("semio.kit.createType", origin, type),
      updateType: (origin: string, guid: Guid, diff: TypeDiff) => kitStore.execute("semio.kit.updateType", origin, guid, diff),
      deleteType: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteType", origin, guid),
      createDesign: (origin: string, design: Design) => kitStore.execute("semio.kit.createDesign", origin, design),
      updateDesign: (origin: string, guid: Guid, diff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", origin, guid, diff),
      deleteDesign: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteDesign", origin, guid),
      addFile: (origin: string, file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", origin, file, blob),
      updateFile: (origin: string, url: string, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", origin, url, fileDiff, blob),
      removeFile: (origin: string, url: string) => kitStore.execute("semio.kit.removeFile", origin, url),
      addPiece: (origin: string, design: Guid, piece: Piece) => kitStore.execute("semio.kit.addPiece", origin, design, piece),
      addPieces: (origin: string, design: Guid, pieces: Piece[]) => kitStore.execute("semio.kit.addPieces", origin, design, pieces),
      removePiece: (origin: string, design: Guid, piece: Guid) => kitStore.execute("semio.kit.removePiece", origin, design, piece),
      removePieces: (origin: string, design: Guid, pieces: Guid[]) => kitStore.execute("semio.kit.removePieces", origin, design, pieces),
      addConnection: (origin: string, design: Guid, connection: Connection) => kitStore.execute("semio.kit.addConnection", origin, design, connection),
      addConnections: (origin: string, design: Guid, connections: Connection[]) => kitStore.execute("semio.kit.addConnections", origin, design, connections),
      removeConnection: (origin: string, design: Guid, connection: Guid) => kitStore.execute("semio.kit.removeConnection", origin, design, connection),
      removeConnections: (origin: string, design: Guid, connections: Guid[]) => kitStore.execute("semio.kit.removeConnections", origin, design, connections),
      deleteSelected: (origin: string, design: Guid, selectedPieces: Guid[], selectedConnections: Guid[]) => kitStore.execute("semio.kit.deleteSelected", origin, design, selectedPieces, selectedConnections),
    };
  }, [kitGuid, store]);
}

// #endregion 🔖Sketchpad

// #region 🔖Commands

export const commands = {
  "semio.sketchpad.setTheme": (context: SketchpadCommandContext, theme: Theme): SketchpadCommandResult => {
    return {
      diff: { theme },
    };
  },
  "semio.sketchpad.setDevice": (context: SketchpadCommandContext, device: Device): SketchpadCommandResult => {
    return {
      diff: { device },
    };
  },
  "semio.sketchpad.setExpertise": (context: SketchpadCommandContext, expertise: Expertise): SketchpadCommandResult => {
    return {
      diff: { expertise },
    };
  },
  "semio.sketchpad.setMode": (context: SketchpadCommandContext, mode: Mode): SketchpadCommandResult => {
    return {
      diff: { mode },
    };
  },
  "semio.sketchpad.setLanguage": (context: SketchpadCommandContext, language: string): SketchpadCommandResult => {
    return {
      diff: { language },
    };
  },
  "semio.sketchpad.toggleFullscreen": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {
      diff: { isFullscreen: !context.sketchpad.isFullscreen },
    };
  },
  "semio.sketchpad.toggleNavbarExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const device = context.sketchpad.device;
    if (typeof device === "object") {
      return {
        diff: { device: { ...device, isNavbarExpanded: !device.isNavbarExpanded } },
      };
    }
    return {};
  },
  "semio.sketchpad.toggleFooterExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const device = context.sketchpad.device;
    if (typeof device === "object") {
      return {
        diff: { device: { ...device, isFooterExpanded: !device.isFooterExpanded } },
      };
    }
    return {};
  },
  "semio.sketchpad.setIsMobile": (context: SketchpadCommandContext, isMobile: boolean): SketchpadCommandResult => {
    if (context.sketchpad.isMobile !== isMobile) {
      return {
        diff: { isMobile },
      };
    }
    return {};
  },
  "semio.sketchpad.setActiveInteraction": (context: SketchpadCommandContext, interactionId?: string): SketchpadCommandResult => {
    return {
      diff: { activeInteraction: interactionId },
    };
  },
  "semio.sketchpad.addNavigation": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    const currentHistoryStr = context.sketchpad.navigationHistory;
    const currentHistory = currentHistoryStr || ["/"];
    const currentIndex = context.sketchpad.navigationHistoryIndex ?? 0;

    const newHistory = currentHistory.slice(0, currentIndex + 1);
    if (newHistory[newHistory.length - 1] !== path) {
      newHistory.push(path);
      return {
        diff: {
          navigation: path,
          navigationHistory: newHistory,
          navigationHistoryIndex: newHistory.length - 1,
        },
      };
    }
    return {
      diff: {
        navigation: path,
      },
    };
  },
  "semio.sketchpad.syncNavigation": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    if (context.sketchpad.navigation !== path) {
      return {
        diff: { navigation: path },
      };
    }
    return {};
  },
  "semio.sketchpad.navigateBack": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const { navigationHistory, navigationHistoryIndex } = context.sketchpad;
    if (navigationHistoryIndex > 0) {
      const newIndex = navigationHistoryIndex - 1;
      return {
        diff: {
          navigation: navigationHistory[newIndex],
          navigationHistoryIndex: newIndex,
        },
      };
    }
    return {};
  },
  "semio.sketchpad.navigateForward": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const { navigationHistory, navigationHistoryIndex } = context.sketchpad;
    if (navigationHistoryIndex < navigationHistory.length - 1) {
      const newIndex = navigationHistoryIndex + 1;
      return {
        diff: {
          navigation: navigationHistory[newIndex],
          navigationHistoryIndex: newIndex,
        },
      };
    }
    return {};
  },
  "semio.sketchpad.setHotkey": (context: SketchpadCommandContext, path: string, value: string): SketchpadCommandResult => {
    const overrides = { ...context.sketchpad.hotkeyOverrides };
    overrides[path] = value;
    return {
      diff: { hotkeyOverrides: overrides },
    };
  },
  "semio.sketchpad.resetHotkey": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    const overrides = { ...context.sketchpad.hotkeyOverrides };
    delete overrides[path];
    return {
      diff: { hotkeyOverrides: overrides },
    };
  },
  "semio.sketchpad.resetAllHotkeys": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {
      diff: { hotkeyOverrides: {} },
    };
  },
  "semio.sketchpad.navigateToHotkeySetting": (context: SketchpadCommandContext, path: string): SketchpadCommandResult => {
    return {
      diff: {
        navigation: "/",
        activeHotkeySetting: path,
      },
    };
  },
  "semio.sketchpad.setState": (context: SketchpadCommandContext, ...args: any[]): SketchpadCommandResult => {
    let state: Partial<SketchpadState>;
    if (args.length > 0 && typeof args[0] === "string" && (args[0] === "semio.sketchpad" || args[0].startsWith("semio.sketchpad."))) {
      state = args[1] as Partial<SketchpadState>;
    } else {
      state = args[0] as Partial<SketchpadState>;
    }
    if (!state || typeof state === "string") {
      return {};
    }
    const diff: SketchpadDiff = {};
    if (state.navigation !== undefined) diff.navigation = state.navigation;
    if (state.navigationHistory !== undefined) diff.navigationHistory = state.navigationHistory;
    if (state.navigationHistoryIndex !== undefined) diff.navigationHistoryIndex = state.navigationHistoryIndex;
    if (state.recentSearches !== undefined) diff.recentSearches = state.recentSearches;
    if (state.recentFocusItems !== undefined) diff.recentFocusItems = state.recentFocusItems;
    if (state.theme !== undefined) diff.theme = state.theme;
    if (state.language !== undefined) diff.language = state.language;
    if (state.device !== undefined) diff.device = state.device;
    if (state.expertise !== undefined) diff.expertise = state.expertise;
    if (state.mode !== undefined) diff.mode = state.mode;
    if (state.settings !== undefined) diff.settings = state.settings;
    if (state.panelSizes !== undefined) diff.panelSizes = state.panelSizes;
    if (state.isFullscreen !== undefined) diff.isFullscreen = state.isFullscreen;
    if (state.isMobile !== undefined) diff.isMobile = state.isMobile;
    if (state.activeInteraction !== undefined) diff.activeInteraction = state.activeInteraction;
    if (state.hotkeyOverrides !== undefined) diff.hotkeyOverrides = state.hotkeyOverrides;
    if (state.activeHotkeySetting !== undefined) diff.activeHotkeySetting = state.activeHotkeySetting;
    return { diff };
  },
};

export const devCommands = {
  "semio.sketchpad.freeze": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {};
  },
  "semio.sketchpad.timetravel": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {};
  },
};

// #endregion 🔖Commands

// #region 🔖Apps Registry

export async function loadAppPanels(appId: string): Promise<PanelConfig[]> {
  try {
    const module = await import(`./apps/${appId}/panels.ts`);
    if (module && module.panels) {
      return module.panels;
    }
  } catch (e) {}
  return [];
}

class AppRegistry {
  private apps: Map<string, AppRegistration> = new Map();
  private autoDiscovered = false;
  private _initialized = false;
  private listeners = new Set<() => void>();

  get isInitialized(): boolean {
    return this._initialized || this.apps.size > 0;
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private notify() {
    this.listeners.forEach((listener) => listener());
  }

  private async autoDiscover(): Promise<void> {
    if (this.autoDiscovered) return;
    this.autoDiscovered = true;

    const appModules = import.meta.glob<{ config: AppConfig }>("./apps/*/App.tsx");

    for (const [path, importFn] of Object.entries(appModules)) {
      const module = await importFn();
      if (module.config) {
        this.register(module.config);
      }
    }
  }

  register(registration: AppRegistration): void {
    if (this.apps.has(registration.id)) return;
    this.apps.set(registration.id, registration);
    this.cachedApps = null;
    this.notify();
  }

  unregister(id: string): void {
    this.apps.delete(id);
    this.cachedApps = null;
    this.notify();
  }

  getApp(id: string): AppRegistration | undefined {
    return this.apps.get(id);
  }

  private cachedApps: AppRegistration[] | null = null;

  getAllApps(): AppRegistration[] {
    if (!this.cachedApps) {
      this.cachedApps = Array.from(this.apps.values()).sort((a, b) => (a.order || 0) - (b.order || 0));
    }
    return this.cachedApps;
  }

  getAppForPath(pathParts: string[]): AppRegistration | undefined {
    for (const app of this.apps.values()) {
      if (app.matchesPath && app.matchesPath(pathParts)) {
        return app;
      }
    }
    return undefined;
  }

  getPanelConfigs(getLabelFn: (key: string) => string, getHotkeyFn?: (key: string) => string): Record<string, PanelDefinition[]> {
    const configs: Record<string, PanelDefinition[]> = {};
    for (const [id, app] of this.apps.entries()) {
      configs[id] = app.getPanels(getLabelFn, getHotkeyFn || getLabelFn);
    }
    return configs;
  }

  async initialize(): Promise<void> {
    if (this._initialized) return;
    await this.autoDiscover();
    await loadAppConfigs();
    this._initialized = true;
  }
}

const appRegistry = new AppRegistry();

import { ActorRefFrom, AnyActorRef, assign, createActor, setup, SnapshotFrom } from "xstate";

let appConfigsLoadPromise: Promise<void> | null = null;
async function loadAppConfigs() {
  if (appConfigsLoadPromise) return appConfigsLoadPromise;

  appConfigsLoadPromise = (async () => {
    const modules = import.meta.glob<any>("./*.tsx");
    for (const [path, importFn] of Object.entries(modules)) {
      if (path.endsWith("/Sketchpad.tsx")) continue;
      if (path.endsWith("/elements.tsx")) continue;
      if (path.endsWith("/shared.ts")) continue;
      if (path.endsWith("/Tutorials.tsx")) continue;

      const mod = await (importFn as any)();
      if (mod?.config) {
        appRegistry.register(mod.config as AppConfig);
      }
    }
  })();

  return appConfigsLoadPromise;
}

export { appRegistry, loadAppConfigs };

// #endregion 🔖Apps Registry

// #region 🔖Navbar

interface FocusContextValue {
  focusItems: FocusItem[];
  setFocusItems: (items: FocusItem[]) => void;
  setOnFocusItem: (callback: ((itemId: string) => void) | undefined) => void;
  triggerFocusItem: (itemId: string) => void;
}

const FocusContext = createContext<FocusContextValue | null>(null);

export const FocusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [focusItems, setFocusItems] = useState<FocusItem[]>([]);
  const onFocusItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);

  const setFocusItemsStable = useCallback((items: FocusItem[]) => {
    setFocusItems(items);
  }, []);

  const setOnFocusItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFocusItemCallbackRef.current = callback;
  }, []);

  const triggerFocusItem = useCallback((itemId: string) => {
    if (onFocusItemCallbackRef.current) {
      onFocusItemCallbackRef.current(itemId);
    }
  }, []);

  const contextValue = useMemo(
    () => ({ focusItems, setFocusItems: setFocusItemsStable, setOnFocusItem, triggerFocusItem }),

    [focusItems],
  );

  return <FocusContext.Provider value={contextValue}>{children}</FocusContext.Provider>;
};

export const useFocus = () => {
  const context = useContext(FocusContext);
  if (!context) throw new Error("useFocus must be used within FocusProvider");
  return context;
};

export const useFocusSafe = () => {
  const context = useContext(FocusContext);
  return context;
};

interface PanelSectionContextValue {
  sections: PanelSections;
  addSection: (panelKey: PanelKey, section: PanelSection) => void;
  removeSection: (panelKey: PanelKey, sectionId: string) => void;
}

const PanelSectionContext = createContext<PanelSectionContextValue | null>(null);

export const PanelSectionProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [sections, setSections] = useState<PanelSections>({
    details: [],
    workbench: [],
    tools: [],
    hud: [],
    stats: [],
    console: [],
    chat: [],
    settings: [],
    toolbar: [],
  });

  const addSection = useCallback((panelKey: PanelKey, section: PanelSection) => {
    setSections((prev) => {
      const updated = {
        ...prev,
        [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => {
          const specificityDiff = (b.specificity ?? 0) - (a.specificity ?? 0);
          if (specificityDiff !== 0) return specificityDiff;
          return (a.order || 0) - (b.order || 0);
        }),
      };
      return updated;
    });
  }, []);

  const removeSection = useCallback((panelKey: PanelKey, sectionId: string) => {
    setSections((prev) => ({ ...prev, [panelKey]: prev[panelKey].filter((s) => s.id !== sectionId) }));
  }, []);

  const contextValue = useMemo(() => ({ sections, addSection, removeSection }), [sections, addSection, removeSection]);

  return <PanelSectionContext.Provider value={contextValue}>{children}</PanelSectionContext.Provider>;
};

export const usePanelSections = (panelKey: PanelKey): PanelSection[] => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("usePanelSections must be used within PanelSectionProvider");
  const sections = context.sections[panelKey];
  return sections;
};

export const useAddPanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useAddPanelSection must be used within PanelSectionProvider");
  return context.addSection;
};

export const useRemovePanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useRemovePanelSection must be used within PanelSectionProvider");
  return context.removeSection;
};

// #endregion 🔖Navbar

// #region 🔖SidePanel Tabs

interface SidePanelTabsState {
  left: SidePanelTab[];
  right: SidePanelTab[];
}

interface HudPanelTabsState {
  tabs: HudPanelTab[];
}

interface SidePanelTabContextValue {
  sidePanelTabs: SidePanelTabsState;
  hudPanelTabs: HudPanelTabsState;
  addSidePanelTab: (position: "left" | "right", tab: SidePanelTab) => void;
  removeSidePanelTab: (position: "left" | "right", tabId: string) => void;
  addHudPanelTab: (tab: HudPanelTab) => void;
  removeHudPanelTab: (tabId: string) => void;
  activeLeftTabId: string | undefined;
  activeRightTabId: string | undefined;
  activeHudTabId: string | undefined;
  setActiveLeftTabId: (tabId: string) => void;
  setActiveRightTabId: (tabId: string) => void;
  setActiveHudTabId: (tabId: string) => void;
}

const SidePanelTabContext = createContext<SidePanelTabContextValue | null>(null);

export const SidePanelTabProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [sidePanelTabs, setSidePanelTabs] = useState<SidePanelTabsState>({ left: [], right: [] });
  const [hudPanelTabs, setHudPanelTabs] = useState<HudPanelTabsState>({ tabs: [] });
  const [activeLeftTabId, setActiveLeftTabId] = useState<string | undefined>(undefined);
  const [activeRightTabId, setActiveRightTabId] = useState<string | undefined>(undefined);
  const [activeHudTabId, setActiveHudTabId] = useState<string | undefined>(undefined);

  const addSidePanelTab = useCallback((position: "left" | "right", tab: SidePanelTab) => {
    setSidePanelTabs((prev) => {
      const updated = { ...prev, [position]: [...prev[position].filter((t) => t.id !== tab.id), tab].sort((a, b) => (a.order ?? 0) - (b.order ?? 0)) };
      return updated;
    });
  }, []);

  const removeSidePanelTab = useCallback((position: "left" | "right", tabId: string) => {
    setSidePanelTabs((prev) => ({ ...prev, [position]: prev[position].filter((t) => t.id !== tabId) }));
  }, []);

  const addHudPanelTab = useCallback((tab: HudPanelTab) => {
    setHudPanelTabs((prev) => ({ tabs: [...prev.tabs.filter((t) => t.id !== tab.id), tab].sort((a, b) => (a.order ?? 0) - (b.order ?? 0)) }));
  }, []);

  const removeHudPanelTab = useCallback((tabId: string) => {
    setHudPanelTabs((prev) => ({ tabs: prev.tabs.filter((t) => t.id !== tabId) }));
  }, []);

  const contextValue = useMemo(
    () => ({
      sidePanelTabs,
      hudPanelTabs,
      addSidePanelTab,
      removeSidePanelTab,
      addHudPanelTab,
      removeHudPanelTab,
      activeLeftTabId,
      activeRightTabId,
      activeHudTabId,
      setActiveLeftTabId,
      setActiveRightTabId,
      setActiveHudTabId,
    }),
    [sidePanelTabs, hudPanelTabs, addSidePanelTab, removeSidePanelTab, addHudPanelTab, removeHudPanelTab, activeLeftTabId, activeRightTabId, activeHudTabId],
  );

  return <SidePanelTabContext.Provider value={contextValue}>{children}</SidePanelTabContext.Provider>;
};

export const useSidePanelTabs = (position: "left" | "right"): SidePanelTab[] => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useSidePanelTabs must be used within SidePanelTabProvider");
  return context.sidePanelTabs[position];
};

export const useHudPanelTabs = (): HudPanelTab[] => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useHudPanelTabs must be used within SidePanelTabProvider");
  return context.hudPanelTabs.tabs;
};

export const useAddSidePanelTab = () => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useAddSidePanelTab must be used within SidePanelTabProvider");
  return context.addSidePanelTab;
};

export const useRemoveSidePanelTab = () => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useRemoveSidePanelTab must be used within SidePanelTabProvider");
  return context.removeSidePanelTab;
};

export const useAddHudPanelTab = () => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useAddHudPanelTab must be used within SidePanelTabProvider");
  return context.addHudPanelTab;
};

export const useRemoveHudPanelTab = () => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useRemoveHudPanelTab must be used within SidePanelTabProvider");
  return context.removeHudPanelTab;
};

export const useActiveLeftTabId = (): [string | undefined, (tabId: string) => void] => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useActiveLeftTabId must be used within SidePanelTabProvider");
  return [context.activeLeftTabId, context.setActiveLeftTabId];
};

export const useActiveRightTabId = (): [string | undefined, (tabId: string) => void] => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useActiveRightTabId must be used within SidePanelTabProvider");
  return [context.activeRightTabId, context.setActiveRightTabId];
};

export const useActiveHudTabId = (): [string | undefined, (tabId: string) => void] => {
  const context = useContext(SidePanelTabContext);
  if (!context) throw new Error("useActiveHudTabId must be used within SidePanelTabProvider");
  return [context.activeHudTabId, context.setActiveHudTabId];
};

// #endregion 🔖SidePanel Tabs

// #region 🔖Origin

type OriginStore = {
  subscribe: (callback: () => void) => () => void;
  getOrigin: () => string;
};

const DEFAULT_ORIGIN = "semio.sketchpad.unknown";

function createOriginStore(): OriginStore & { setOrigin: (origin: string) => void } {
  let origin = DEFAULT_ORIGIN;
  const listeners = new Set<() => void>();
  const subscribe = (callback: () => void) => {
    listeners.add(callback);
    return () => {
      listeners.delete(callback);
    };
  };
  const setOrigin = (next: string) => {
    if (origin === next) return;
    origin = next;
    listeners.forEach((cb) => cb());
  };
  return { subscribe, setOrigin, getOrigin: () => origin };
}

function resolveOriginFromTarget(target: EventTarget | null): string {
  if (!(target instanceof Element)) return DEFAULT_ORIGIN;
  const resolved = target.closest('[id^="semio.sketchpad."]')?.getAttribute("id") ?? "";
  if (!resolved) return DEFAULT_ORIGIN;
  if (!resolved.startsWith("semio.sketchpad.")) return DEFAULT_ORIGIN;
  return resolved;
}

const OriginContext = createContext<OriginStore | null>(null);

export const OriginProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const storeRef = useRef<ReturnType<typeof createOriginStore> | null>(null);
  if (!storeRef.current) storeRef.current = createOriginStore();
  useEffect(() => {
    const store = storeRef.current;
    if (!store) return;
    const handler = (event: Event) => {
      store.setOrigin(resolveOriginFromTarget(event.target));
    };
    document.addEventListener("pointerdown", handler, true);
    document.addEventListener("keydown", handler, true);
    document.addEventListener("focusin", handler, true);
    return () => {
      document.removeEventListener("pointerdown", handler, true);
      document.removeEventListener("keydown", handler, true);
      document.removeEventListener("focusin", handler, true);
    };
  }, []);
  return <OriginContext.Provider value={storeRef.current}>{children}</OriginContext.Provider>;
};

export function useOrigin(): () => string {
  const store = useContext(OriginContext);
  return useCallback(() => store?.getOrigin() ?? DEFAULT_ORIGIN, [store]);
}

export function useOriginValue(): string {
  const store = useContext(OriginContext);
  return useSyncExternalStore(
    useCallback((callback: () => void) => (store ? store.subscribe(callback) : () => {}), [store]),
    useCallback(() => store?.getOrigin() ?? DEFAULT_ORIGIN, [store]),
  );
}

// #endregion 🔖Origin

// #region 🔖Footer Items

interface FooterItemContextValue {
  items: FooterItem[];
  addItem: (item: FooterItem) => void;
  removeItem: (itemId: string) => void;
}

const FooterItemContext = createContext<FooterItemContextValue | null>(null);

export const FooterItemProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [items, setItems] = useState<FooterItem[]>([]);

  const addItem = useCallback((item: FooterItem) => {
    setItems((prev) => {
      const updated = [...prev.filter((i) => i.id !== item.id), item].sort((a, b) => (a.order || 0) - (b.order || 0));
      return updated;
    });
  }, []);

  const removeItem = useCallback((itemId: string) => {
    setItems((prev) => prev.filter((i) => i.id !== itemId));
  }, []);

  const contextValue = useMemo(() => ({ items, addItem, removeItem }), [items, addItem, removeItem]);

  return <FooterItemContext.Provider value={contextValue}>{children}</FooterItemContext.Provider>;
};

export const useFooterItems = (): FooterItem[] => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useFooterItems must be used within FooterItemProvider");
  return context.items;
};

export const useAddFooterItem = () => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useAddFooterItem must be used within FooterItemProvider");
  return context.addItem;
};

export const useRemoveFooterItem = () => {
  const context = useContext(FooterItemContext);
  if (!context) throw new Error("useRemoveFooterItem must be used within FooterItemProvider");
  return context.removeItem;
};

// #endregion 🔖Footer Items

// #region 🔖Global Footer Items

const GlobalFooterItems: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const navigate = useReactNavigate();

  useEffect(() => {
    addFooterItem({
      id: "semio.sketchpad.footer.feedback",
      icon: <FeedbackIcon size={14} />,
      order: 1000,
      onClick: () => navigate("/feedback"),
    });

    return () => {
      removeFooterItem("semio.sketchpad.footer.feedback");
    };
  }, [addFooterItem, removeFooterItem, navigate]);

  return null;
};

// #endregion 🔖Global Footer Items

// #region 🔖ConceptFilter

export const ConceptFilter: FC<{ allConcepts: string[]; paramName?: string }> = ({ allConcepts, paramName = "concepts" }) => {
  const [searchParams, setSearchParams] = useSearchParams();
  const selectedConcepts = paramName === "c" ? searchParams.getAll("c") : searchParams.get(paramName)?.split(",").filter(Boolean) || [];

  const toggleConcept = useCallback(
    (concept: string) => {
      const newParams = new URLSearchParams(searchParams);
      if (paramName === "c") {
        const currentConcepts = newParams.getAll("c");
        if (currentConcepts.includes(concept)) {
          newParams.delete("c");
          currentConcepts.filter((c) => c !== concept).forEach((c) => newParams.append("c", c));
        } else {
          newParams.append("c", concept);
        }
      } else {
        const currentConcepts = newParams.get(paramName)?.split(",").filter(Boolean) || [];
        if (currentConcepts.includes(concept)) {
          const updated = currentConcepts.filter((c) => c !== concept);
          if (updated.length > 0) {
            newParams.set(paramName, updated.join(","));
          } else {
            newParams.delete(paramName);
          }
        } else {
          newParams.set(paramName, [...currentConcepts, concept].join(","));
        }
      }
      setSearchParams(newParams);
    },
    [searchParams, setSearchParams, paramName],
  );

  if (allConcepts.length === 0) return null;

  return (
    <Strip
      id="semio.sketchpad.filter.concepts"
      items={allConcepts.map((concept) => ({
        key: concept,
        content: <Action onClick={() => toggleConcept(concept)} id={`semio.sketchpad.filter.concept.${concept}`} text={concept} className={selectedConcepts.includes(concept) ? "bg-active-base" : ""} />,
      }))}
    />
  );
};

// #endregion 🔖ConceptFilter

// #region 🔖ToolGroup

export const ToolGroup: FC<ToolGroupProps> = ({ tools, activeTool, onToolChange }) => {
  const getActiveToolDefinition = () => {
    for (const tool of tools) {
      const matchingMode = tool.modes.find((mode) => mode.id === activeTool);
      if (matchingMode) {
        return { tool, mode: matchingMode };
      }
    }
    return null;
  };

  const activeToolDef = getActiveToolDefinition();
  const currentTool = activeToolDef?.tool || tools[0];
  const currentMode = activeToolDef?.mode || currentTool.modes[0];

  const handleToolClick = (tool: ToolDefinition) => {
    if (tool.modes.length === 1) {
      onToolChange(tool.modes[0].id);
    } else {
      const currentIndex = tool.modes.findIndex((m) => m.id === activeTool);
      const nextIndex = currentIndex >= 0 && currentIndex < tool.modes.length - 1 ? currentIndex + 1 : 0;
      onToolChange(tool.modes[nextIndex].id);
    }
  };

  const handleModeSelect = (modeId: string) => {
    onToolChange(modeId);
  };

  if (tools.length === 0) return null;

  return (
    <div className="flex items-center gap-single">
      {tools.map((tool) => {
        const isActive = tool.modes.some((m) => m.id === activeTool);
        const activeMode = tool.modes.find((m) => m.id === activeTool) || tool.modes[0];

        if (tool.modes.length > 1) {
          return (
            <Toggle
              key={tool.id}
              kind="dropdown"
              id={`semio.sketchpad.tool.${tool.id}`}
              items={tool.modes.map((mode) => ({
                value: mode.id,
                label: mode.icon || mode.label || mode.id,
              }))}
              value={activeMode.id}
              onValueChange={handleModeSelect}
              pressed={isActive}
              onPressedChange={(pressed) => {
                if (pressed) handleToolClick(tool);
              }}
            />
          );
        } else {
          const mode = tool.modes[0];
          return <Toggle key={tool.id} id={mode.tooltipId || `semio.sketchpad.tool.${mode.id}`} pressed={isActive} onPressedChange={() => handleToolClick(tool)} icon={mode.icon} />;
        }
      })}
    </div>
  );
};

// #endregion 🔖ToolGroup

// #region 🔖DragDrop

interface DragDropContextValue {
  activeDraggedType: Type | null;
  activeDraggedDesign: Design | null;
  setActiveDraggedType: (type: Type | null) => void;
  setActiveDraggedDesign: (design: Design | null) => void;
}

const DragDropContext = createContext<DragDropContextValue | null>(null);

export const DragDropProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);

  return <DragDropContext.Provider value={{ activeDraggedType, activeDraggedDesign, setActiveDraggedType, setActiveDraggedDesign }}>{children}</DragDropContext.Provider>;
};

export const useDragDrop = () => {
  const context = useContext(DragDropContext);
  if (!context) throw new Error("useDragDrop must be used within DragDropProvider");
  return context;
};

// #endregion 🔖DragDrop

// #region 🔖Hotkeys

export function useHotkeys(hotkeyOrPath: string, callback: () => void, options?: { enableOnFormTags?: boolean }, deps?: React.DependencyList) {
  const hotkeyOverrides = useSketchpad((s) => s.hotkeyOverrides) as Record<string, string> | undefined;
  const resolvedHotkey = useHotkey(hotkeyOrPath);

  const finalHotkey = useMemo(() => {
    const override = hotkeyOverrides?.[hotkeyOrPath];
    if (override) return override;
    return resolvedHotkey || hotkeyOrPath;
  }, [hotkeyOrPath, hotkeyOverrides, resolvedHotkey]);

  useReactHotkeys(finalHotkey, callback, options || {}, deps || []);
}

// #endregion 🔖Hotkeys

export function usePanelConfigs(): Record<string, EnrichedPanelDefinition[]> {
  const { t } = useI18nTranslation();

  const getHotkeyFn = useCallback(
    (id: string) => {
      const value = t(id as any) as any;
      if (typeof value === "object" && value?.hotkey) {
        return typeof value.hotkey === "string" ? value.hotkey : "";
      }
      const hotkeyValue = t(`${id}.hotkey` as any) as any;
      if (typeof hotkeyValue === "string") return hotkeyValue;
      if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
        return typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : "";
      }
      return "";
    },
    [t],
  );

  const apps = useSyncExternalStore(
    useCallback((cb) => appRegistry.subscribe(cb), []),
    () => appRegistry.getAllApps(),
  );

  const panelConfigsByApp = useMemo(() => {
    const emptyLabelFn = () => "";
    const configs: Record<string, PanelDefinition[]> = {};
    for (const app of apps) {
      configs[app.id] = app.getPanels(emptyLabelFn, getHotkeyFn);
    }
    return configs;
  }, [apps, getHotkeyFn]);

  const allPanelIds = useMemo(() => {
    const ids: string[] = [];
    for (const panels of Object.values(panelConfigsByApp)) {
      panels.forEach((p) => ids.push(p.id));
    }
    return ids;
  }, [panelConfigsByApp]);

  const hotkeysMap = useMemo(() => {
    const map = new Map<string, string | undefined>();
    allPanelIds.forEach((id) => {
      const value = t(id as any) as any;
      let hotkey: string | undefined;
      if (typeof value === "object" && value?.hotkey) {
        hotkey = typeof value.hotkey === "string" ? value.hotkey : undefined;
      } else {
        const hotkeyValue = t(`${id}.hotkey` as any) as any;
        if (typeof hotkeyValue === "string") {
          hotkey = hotkeyValue;
        } else if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
          hotkey = typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : undefined;
        }
      }
      map.set(id, hotkey);
    });
    return map;
  }, [allPanelIds, t]);

  const enrich = (panels: PanelDefinition[]) =>
    panels.map((panel) => {
      const i18nHotkey = hotkeysMap.get(panel.id);
      const config = panelKindConfigs[panel.kind];
      return enrichPanelDefinition({
        ...panel,
        hotkey: panel.hotkey ?? i18nHotkey ?? config.hotkey,
      });
    });

  return useMemo(() => {
    const result: Record<string, EnrichedPanelDefinition[]> = {};
    for (const [appId, panels] of Object.entries(panelConfigsByApp)) {
      result[appId] = enrich(panels);
    }
    return result;
  }, [panelConfigsByApp, hotkeysMap]);
}

interface NavigationProps {
  mobile?: boolean;
}

const Navigation: FC<NavigationProps> = ({ mobile = false }) => {
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams] = useSearchParams();
  const kits = useKits();

  const [mode] = useMode();
  const isMobile = useIsMobile();
  const isNavbarExpanded = useIsNavbarExpanded();

  const pathParts = navigation.split("/").filter((p) => p);
  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
  const isKitsPath = pathParts[0] === "kits";

  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;

  const kitGuid = isKitsPath && pathParts[1] ? pathParts[1] : null;

  const secondPart = pathParts[2];
  const thirdPart = pathParts[3];
  const isDesignApp = isKitsPath && secondPart === "designs" && thirdPart && isUuidPattern(thirdPart);
  const isTypeApp = isKitsPath && secondPart === "types" && thirdPart && isUuidPattern(thirdPart);
  const isQualityApp = isKitsPath && secondPart === "qualities" && thirdPart && isUuidPattern(thirdPart);
  const itemGuid = isDesignApp || isTypeApp || isQualityApp ? thirdPart : null;

  const filteredKind = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? (searchParams.get("kind") as "designs" | "types" | "qualities" | "files" | "authors" | null) : null;
  const filteredName = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? searchParams.get("name") : null;

  const isKitApp = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp;

  const kitFromScope = useKit();
  const designFromScope = useDesign();
  const typeFromScope = useType();
  const kit: Kit | KitShallow | null | undefined = (kitFromScope as Kit | KitShallow | null | undefined) || kits.find((k) => k.guid === kitGuid);
  const kitKind = useKitKind(kitGuid || "");

  const kitKindItems = [
    { label: <TemporaryKitIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.temporary", href: "/?kind=temporary" },
    { label: <LocalKitIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.local", href: "/?kind=local" },
    { label: <RemoteKitIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.remote", href: "/?kind=remote" },
  ];

  const filteredKits = useFilteredKitShallows(kitKind);
  const createKitLabel = useLabel("semio.sketchpad.navbar.createKit");
  const kitItemsWithCreate = useMemo(() => {
    const items = filteredKits.map((k) => ({ label: k.name, href: `/kits/${k.guid}` }));
    items.push({ label: "+ " + createKitLabel, href: "#create-kit" });
    return items;
  }, [filteredKits, createKitLabel]);

  const sketchpadCommands = useSketchpadCommands();
  const kitCommands = useKitCommandsById(kitGuid || undefined);

  const artifactKinds = [
    { label: <LayoutIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.designs", kind: "designs", href: kitGuid ? `/kits/${kitGuid}?kind=designs` : "/kits?kind=designs" },
    { label: <TypeIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.types", kind: "types", href: kitGuid ? `/kits/${kitGuid}?kind=types` : "/kits?kind=types" },
    { label: <AwardIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.qualities", kind: "qualities", href: kitGuid ? `/kits/${kitGuid}?kind=qualities` : "/kits?kind=qualities" },
    { label: <DocumentIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.files", kind: "files", href: kitGuid ? `/kits/${kitGuid}?kind=files` : "/kits?kind=files" },
    { label: <UserIcon size={16} />, id: "semio.sketchpad.navbar.breadcrumb.authors", kind: "authors", href: kitGuid ? `/kits/${kitGuid}?kind=authors` : "/kits?kind=authors" },
  ];

  const allDesigns: Design[] = useMemo(() => {
    if (!kit?.designs) return [];
    return (kit.designs as any[]).filter((d): d is Design => typeof d === "object" && d.guid !== undefined);
  }, [kit?.designs]);

  const allTypes: Type[] = useMemo(() => {
    if (!kit?.types) return [];
    return (kit.types as any[]).filter((t): t is Type => typeof t === "object" && t.guid !== undefined);
  }, [kit?.types]);

  const allQualities: Quality[] = useMemo(() => {
    if (!kit?.qualities) return [];
    return (kit.qualities as any[]).filter((q): q is Quality => typeof q === "object" && q.guid !== undefined);
  }, [kit?.qualities]);

  const allFolders: Folder[] = useMemo(() => {
    if (!kit?.folders) return [];
    return (kit.folders as any[]).filter((f): f is Folder => typeof f === "object" && f.guid !== undefined);
  }, [kit?.folders]);

  const defaultKitName = useLabel("semio.sketchpad.app.kit.defaultName");
  const handleCreateKit = useCallback(
    (origin: string) => {
      const guid = crypto.randomUUID();
      const now = new Date().toISOString();
      const existingNames = kits.map((k) => k.name);
      const uniqueName = generateUniqueName(defaultKitName, existingNames);
      sketchpadCommands.createKit(origin, {
        guid,
        name: uniqueName,
        version: "",
        createdAt: now,
        updatedAt: now,
      });
      sketchpadCommands.navigateToKit(guid);
    },
    [sketchpadCommands, kits, defaultKitName],
  );

  const newVersionLabel = useLabel("semio.sketchpad.app.kit.newVersion");
  const handleCreateVersion = useCallback(
    (origin: string) => {
      if (!kit) return;
      const newGuid = crypto.randomUUID();
      const now = new Date().toISOString();
      const existingVersions = kits.filter((k) => k.name === kit.name).map((k) => k.version || "");
      const uniqueVersion = generateUniqueName(newVersionLabel, existingVersions);
      sketchpadCommands.createKit(origin, {
        guid: newGuid,
        name: kit.name,
        version: uniqueVersion,
        createdAt: now,
        updatedAt: now,
      });
      sketchpadCommands.navigateToKit(newGuid);
    },
    [kit, kits, sketchpadCommands, newVersionLabel],
  );

  const defaultDesignName = useLabel("semio.sketchpad.app.design.defaultName");
  const handleCreateDesign = useCallback(
    (origin: string, name?: string, parent?: string) => {
      if (!kitCommands || !kitGuid) return;
      const guid = crypto.randomUUID();
      const existingNames = allDesigns.map((d) => d.name);
      const uniqueName = name || generateUniqueName(defaultDesignName, existingNames);
      kitCommands.createDesign(origin, { guid, name: uniqueName, parent: parent ? { guid: parent } : undefined, pieces: [], connections: [] });
      sketchpadCommands.navigateToDesign(kitGuid, guid);
    },
    [kitCommands, kitGuid, sketchpadCommands, allDesigns, defaultDesignName],
  );

  const defaultTypeName = useLabel("semio.sketchpad.app.type.defaultName");
  const handleCreateType = useCallback(
    (origin: string, name?: string, parent?: string) => {
      if (!kitCommands || !kitGuid) return;
      const guid = crypto.randomUUID();
      const existingNames = allTypes.map((t) => t.name);
      const uniqueName = name || generateUniqueName(defaultTypeName, existingNames);
      kitCommands.createType(origin, { guid, name: uniqueName, parent: parent ? { guid: parent } : undefined, connectors: [] });
      sketchpadCommands.navigateToType(kitGuid, guid);
    },
    [kitCommands, kitGuid, sketchpadCommands, allTypes, defaultTypeName],
  );

  const handleCreateChild = useCallback(
    (origin: string, designOrType: Design | Type, isType: boolean) => {
      if (!kitCommands) return;
      const guid = crypto.randomUUID();
      if (!isType) {
        const d = designOrType as Design;
        const existingNames = allDesigns.filter((design) => design.parent?.guid === d.guid).map((design) => design.name);
        const uniqueName = generateUniqueName(d.name, existingNames);
        kitCommands.createDesign(origin, {
          guid,
          name: uniqueName,
          parent: { guid: d.guid },
          pieces: [],
          connections: [],
        });
        if (kitGuid) sketchpadCommands.navigateToDesign(kitGuid, guid);
      } else {
        const typeObj = designOrType as Type;
        const existingNames = allTypes.filter((type) => type.parent?.guid === typeObj.guid).map((type) => type.name);
        const uniqueName = generateUniqueName(typeObj.name, existingNames);
        kitCommands.createType(origin, {
          guid,
          name: uniqueName,
          parent: { guid: typeObj.guid },
          connectors: [],
        });
        navigate(`/kits/${kitGuid}/types/${guid}`);
      }
    },
    [kitCommands, kitGuid, navigate, allDesigns, allTypes],
  );

  const handleCreate = useCallback(
    (origin: string) => {
      if (!kit || !filteredKind || !kitCommands) return;

      switch (filteredKind) {
        case "designs":
          handleCreateDesign(origin);
          break;
        case "types":
          handleCreateType(origin);
          break;
        case "authors":
          const guid = crypto.randomUUID();
          kitCommands.createAuthor(origin, { guid, name: "New Author", email: "" });
          break;
        case "qualities":
          // TODO: Add createQuality command
          break;
        case "files":
          // TODO: Add createFile command
          break;
      }
    },
    [kit, filteredKind, kitCommands, handleCreateDesign, handleCreateType],
  );

  const design = designFromScope || (isDesignApp ? allDesigns.find((d) => d.guid === itemGuid) : undefined);
  const type = typeFromScope || (isTypeApp ? allTypes.find((t) => t.guid === itemGuid) : undefined);
  const quality = isQualityApp ? allQualities.find((q) => q.guid === itemGuid) : undefined;

  const designFolderChain = useMemo(() => {
    if (!design || typeof design !== "object" || !("parent" in design)) return [];
    const designObj = design as Design;

    let rootDesign = designObj;
    while (rootDesign.parent) {
      const parent = allDesigns.find((d) => d.guid === rootDesign.parent?.guid);
      if (!parent) break;
      rootDesign = parent;
    }

    if (!rootDesign.folder) return [];

    const chain: Folder[] = [];
    let currentFolderId: string | undefined = rootDesign.folder;
    while (currentFolderId) {
      const folder = allFolders.find((f) => f.guid === currentFolderId);
      if (!folder) break;
      chain.unshift(folder);
      currentFolderId = folder.parent?.guid;
    }
    return chain;
  }, [design, allDesigns, allFolders]);

  const typeFolderChain = useMemo(() => {
    if (!type || typeof type !== "object" || !("parent" in type)) return [];
    const typeObj = type as Type;

    let rootType = typeObj;
    while (rootType.parent) {
      const parent = allTypes.find((t) => t.guid === rootType.parent?.guid);
      if (!parent) break;
      rootType = parent;
    }

    if (!rootType.folder) return [];

    const chain: Folder[] = [];
    let currentFolderId: string | undefined = rootType.folder;
    while (currentFolderId) {
      const folder = allFolders.find((f) => f.guid === currentFolderId);
      if (!folder) break;
      chain.unshift(folder);
      currentFolderId = folder.parent?.guid;
    }
    return chain;
  }, [type, allTypes, allFolders]);

  const designParentChain = useMemo(() => {
    if (!design || typeof design !== "object" || !("parent" in design)) return [];
    const designObj = design as Design;
    const chain: Design[] = [];
    let current: Design | undefined = designObj;
    while (current) {
      if (!current.parent) break;
      const parentId: string = current.parent.guid;
      const parent: Design | undefined = allDesigns.find((d) => d.guid === parentId);
      if (!parent) break;
      chain.unshift(parent);
      current = parent;
    }
    return chain;
  }, [design, allDesigns]);

  const typeParentChain = useMemo(() => {
    if (!type || typeof type !== "object" || !("parent" in type)) return [];
    const typeObj = type as Type;
    const chain: Type[] = [];
    let current: Type | undefined = typeObj;
    while (current) {
      if (!current.parent) break;
      const parentId: string = current.parent.guid;
      const parent: Type | undefined = allTypes.find((t) => t.guid === parentId);
      if (!parent) break;
      chain.unshift(parent);
      current = parent;
    }
    return chain;
  }, [type, allTypes]);

  const createDesignLabel = useLabel("semio.sketchpad.navbar.createDesign");
  const createChildLabel = useLabel("semio.sketchpad.navbar.createChild");
  const createTypeLabel = useLabel("semio.sketchpad.navbar.createType");
  const createVersionLabel = useLabel("semio.sketchpad.navbar.createVersion");
  const defaultVersionLabel = useLabel("semio.sketchpad.app.kit.defaultVersion");

  const designNameItems = useMemo(() => {
    const currentDesignGuid = design && typeof design === "object" && "guid" in design ? (design as Design).guid : undefined;
    const items = allDesigns
      .filter((d) => d.guid !== currentDesignGuid)
      .map((d) => ({
        label: d.name,
        href: `/kits/${kitGuid}/designs/${d.guid}`,
      }));
    items.push({ label: "+ " + createDesignLabel, href: "#create-design" });
    return items;
  }, [allDesigns, kitGuid, createDesignLabel, design]);

  const designParentChildItems = useMemo(() => {
    return designParentChain.map((parent) => {
      const children = allDesigns.filter((d) => d.parent?.guid === parent.guid);
      const items = children.map((d) => ({
        label: d.name,
        href: `/kits/${kitGuid}/designs/${d.guid}`,
      }));
      items.push({ label: "+ " + createChildLabel, href: `#create-child-${parent.guid}` });
      return { parentGuid: parent.guid, items };
    });
  }, [designParentChain, allDesigns, kitGuid, createChildLabel]);

  const designChildItems = useMemo(() => {
    if (!design || typeof design !== "object" || !("guid" in design)) return [];
    const designObj = design as Design;
    const children = allDesigns.filter((d) => d.parent?.guid === designObj.guid);
    const items = children.map((d) => ({
      label: d.name,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + createChildLabel, href: "#create-child" });
    return items;
  }, [design, allDesigns, kitGuid, createChildLabel]);

  const typeNameItems = useMemo(() => {
    const currentTypeGuid = type && typeof type === "object" && "guid" in type ? (type as Type).guid : undefined;
    const items = allTypes
      .filter((t) => t.guid !== currentTypeGuid)
      .map((t) => ({
        label: t.name,
        href: `/kits/${kitGuid}/types/${t.guid}`,
      }));
    items.push({ label: "+ " + createTypeLabel, href: "#create-type" });
    return items;
  }, [allTypes, kitGuid, createTypeLabel, type]);

  const typeParentChildItems = useMemo(() => {
    return typeParentChain.map((parent) => {
      const children = allTypes.filter((t) => t.parent?.guid === parent.guid);
      const items = children.map((t) => ({
        label: t.name,
        href: `/kits/${kitGuid}/types/${t.guid}`,
      }));
      items.push({ label: "+ " + createChildLabel, href: `#create-child-${parent.guid}` });
      return { parentGuid: parent.guid, items };
    });
  }, [typeParentChain, allTypes, kitGuid, createChildLabel]);

  const typeChildItems = useMemo(() => {
    if (!type || typeof type !== "object" || !("guid" in type)) return [];
    const typeObj = type as Type;
    const children = allTypes.filter((t) => t.parent?.guid === typeObj.guid);
    const items = children.map((typeObj) => ({
      label: typeObj.name,
      href: `/kits/${kitGuid}/types/${typeObj.guid}`,
    }));
    items.push({ label: "+ " + createChildLabel, href: "#create-child" });
    return items;
  }, [type, allTypes, kitGuid, createChildLabel]);

  const kitVersionItems = useMemo(() => {
    if (!kit?.name) return [];
    const sameNameKits = kits.filter((k) => k.name === kit.name && k.guid !== kitGuid);
    const items = sameNameKits.map((k) => ({
      label: k.version || <span className="italic opacity-70">{defaultVersionLabel}</span>,
      href: `/kits/${k.guid}`,
    }));
    items.push({ label: "+ " + createVersionLabel, href: "#create-version" });
    return items;
  }, [kit, kits, defaultVersionLabel, createVersionLabel, kitGuid]);

  const homeKitsByKind = useFilteredKitShallows(homeKind || undefined);
  const homeKitsForKind = useMemo(() => {
    if (!homeKind) return [];
    const items = homeKitsByKind
      .filter((k) => k.name !== homeName)
      .map((k) => ({
        label: k.name,
        href: `/?kind=${homeKind}&name=${encodeURIComponent(k.name)}`,
      }));
    items.push({ label: "+ " + createKitLabel, href: "#create-kit" });
    return items;
  }, [homeKind, homeKitsByKind, createKitLabel, homeName]);

  const homeVersionsForName = useMemo(() => {
    if (!homeName || !homeKind) return [];
    return homeKitsByKind
      .filter((k) => k.name === homeName && k.guid !== kitGuid)
      .map((k) => ({
        label: k.version || <span className="italic opacity-70">{defaultVersionLabel}</span>,
        href: `/kits/${k.guid}`,
      }));
  }, [homeName, homeKind, homeKitsByKind, defaultVersionLabel, kitGuid]);

  const filteredNameItems = useMemo(() => {
    if (!kit || !filteredKind) return [];
    const nameSet = new Set<string>();

    if (filteredKind === "designs") {
      allDesigns.forEach((d) => nameSet.add(d.name));
    } else if (filteredKind === "types") {
      allTypes.forEach((t) => nameSet.add(t.name));
    }

    return Array.from(nameSet).map((name) => ({
      label: name,
      href: `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(name)}`,
    }));
  }, [kit, filteredKind, allDesigns, allTypes, kitGuid]);

  const isAtRoot = navigation === "/";
  const hasKindFilter = filteredKind || (kitGuid && kitKind);

  const breadcrumbItems: Array<{ id?: string; content: React.ReactNode; options?: any[]; onNavigate?: (href: string) => void }> = [];

  breadcrumbItems.push({
    id: "semio.sketchpad.navbar.home",
    content: (
      <a onClick={() => navigate("/")} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
        <HomeIcon size={16} />
      </a>
    ),
    options: kitKindItems,
    onNavigate: (href) => navigate(href),
  });

  if (kitGuid || homeKind) {
    if (kitKind || homeKind) {
      breadcrumbItems.push({
        id: `semio.sketchpad.navbar.breadcrumb.${kitKind || homeKind}`,
        content: (
          <a onClick={() => navigate(`/?kind=${kitKind || homeKind}`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            {(kitKind === "temporary" || homeKind === "temporary") && <TemporaryKitIcon size={16} />}
            {(kitKind === "local" || homeKind === "local") && <LocalKitIcon size={16} />}
            {(kitKind === "remote" || homeKind === "remote") && <RemoteKitIcon size={16} />}
          </a>
        ),
        options: kitGuid ? kitItemsWithCreate : homeKitsForKind,
        onNavigate: (href) => {
          if (href === "#create-kit") handleCreateKit("semio.sketchpad.navbar.kits");
          else navigate(href);
        },
      });
    }

    if (homeName) {
      const kindIndex = breadcrumbItems.findIndex((item) => item.id === `semio.sketchpad.navbar.breadcrumb.${kitKind || homeKind}`);
      if (kindIndex !== -1) {
        breadcrumbItems[kindIndex] = {
          ...breadcrumbItems[kindIndex],
          options: homeKitsForKind,
          onNavigate: (href) => {
            if (href === "#create-kit") handleCreateKit("semio.sketchpad.navbar.kits");
            else navigate(href);
          },
        };
      }

      breadcrumbItems.push({
        id: "semio.sketchpad.navbar.kitName",
        content: (
          <a onClick={() => navigate(`/?kind=${homeKind}&name=${encodeURIComponent(homeName)}`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            {homeName}
          </a>
        ),
        options: homeVersionsForName,
        onNavigate: (href) => navigate(href),
      });
      if (homeVersion !== null) {
        breadcrumbItems.push({
          id: "semio.sketchpad.navbar.kitVersion",
          content: <span className="text-foreground px-single flex items-center gap-single h-full">{homeVersion || <span className="italic opacity-70">{defaultVersionLabel}</span>}</span>,
        });
      }
    }

    if (kitGuid) {
      const existingNameIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitName");
      if (existingNameIndex === -1) {
        const kindIndex = breadcrumbItems.findIndex((item) => item.id === `semio.sketchpad.navbar.breadcrumb.${kitKind || homeKind}`);
        if (kindIndex !== -1) {
          breadcrumbItems[kindIndex] = {
            ...breadcrumbItems[kindIndex],
            options: kitItemsWithCreate,
            onNavigate: (href) => {
              if (href === "#create-kit") handleCreateKit("semio.sketchpad.navbar.kits");
              else navigate(href);
            },
          };
        }

        breadcrumbItems.push({
          id: "semio.sketchpad.navbar.kitName",
          content: (
            <a
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}`);
              }}
              className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
            >
              {kit?.name || kitGuid}
            </a>
          ),
          options: kitVersionItems,
          onNavigate: (href) => {
            if (href === "#create-version") handleCreateVersion("semio.sketchpad.navbar.versions");
            else navigate(href);
          },
        });
      } else {
        breadcrumbItems[existingNameIndex] = {
          ...breadcrumbItems[existingNameIndex],
          options: kitVersionItems,
          onNavigate: (href) => {
            if (href === "#create-version") handleCreateVersion("semio.sketchpad.navbar.versions");
            else navigate(href);
          },
        };
      }

      const existingVersionIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitVersion");
      if (existingVersionIndex === -1) {
        breadcrumbItems.push({
          id: "semio.sketchpad.navbar.kitVersion",
          content: (
            <a
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                const versionParam = kit?.version !== undefined ? `&version=${encodeURIComponent(kit.version)}` : "";
                navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}${versionParam}`);
              }}
              className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
            >
              {kit?.version || <span className="italic opacity-70">{defaultVersionLabel}</span>}
            </a>
          ),
          options: artifactKinds,
          onNavigate: (href) => navigate(href),
        });
      }
    }
  }

  if (isKitApp && filteredKind) {
    const versionIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitVersion");
    if (versionIndex !== -1) {
      breadcrumbItems[versionIndex] = {
        ...breadcrumbItems[versionIndex],
        options: artifactKinds,
        onNavigate: (href) => navigate(href),
      };
    }

    breadcrumbItems.push({
      id: `semio.sketchpad.navbar.breadcrumb.${filteredKind}`,
      content: (
        <a onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
          {filteredKind === "designs" && <LayoutIcon size={16} />}
          {filteredKind === "types" && <TypeIcon size={16} />}
          {filteredKind === "qualities" && <AwardIcon size={16} />}
          {filteredKind === "files" && <DocumentIcon size={16} />}
          {filteredKind === "authors" && <UserIcon size={16} />}
        </a>
      ),
      options: filteredKind === "designs" ? designNameItems : filteredKind === "types" ? typeNameItems : undefined,
      onNavigate: (href) => {
        if (href === "#create-design" && filteredKind === "designs") handleCreateDesign("semio.sketchpad.navbar.selectDesign");
        else if (href === "#create-type" && filteredKind === "types") handleCreateType("semio.sketchpad.navbar.selectType");
        else navigate(href);
      },
    });
    if (filteredName !== null) {
      breadcrumbItems.push({
        id: "semio.sketchpad.navbar.name",
        content: (
          <a
            onClick={() => {
              const firstMatchingDesign = (kit?.designs as any[])?.find((d: any) => d.name === filteredName);
              if (firstMatchingDesign) {
                navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&select=${firstMatchingDesign.guid}`);
              }
            }}
            className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
          >
            {filteredName}
          </a>
        ),
        options: filteredNameItems,
        onNavigate: (href) => navigate(href),
      });
    }
  }

  if (isDesignApp && design) {
    const versionIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitVersion");
    if (versionIndex !== -1) {
      breadcrumbItems[versionIndex] = {
        ...breadcrumbItems[versionIndex],
        options: artifactKinds,
        onNavigate: (href) => navigate(href),
      };
    }

    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.breadcrumb.designs",
      content: (
        <a onClick={() => navigate(`/kits/${kitGuid}?kind=designs`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
          <LayoutIcon size={16} />
        </a>
      ),
      options: designNameItems,
      onNavigate: (href) => {
        if (href === "#create-design") handleCreateDesign("semio.sketchpad.navbar.selectDesign");
        else navigate(href);
      },
    });
    designFolderChain.forEach((folder) => {
      breadcrumbItems.push({
        id: `semio.sketchpad.navbar.folder.${folder.guid}`,
        content: (
          <a onClick={() => navigate(`/kits/${kitGuid}?kind=folders`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            {folder.name}
          </a>
        ),
      });
    });
    designParentChain.forEach((parent, index) => {
      const childItems = designParentChildItems.find((s) => s.parentGuid === parent.guid)?.items || [];
      breadcrumbItems.push({
        id: `semio.sketchpad.navbar.design.parent.${parent.guid}`,
        content: (
          <button
            type="button"
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              navigate(`/kits/${kitGuid}/designs/${parent.guid}`);
            }}
            className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
          >
            {parent.name}
          </button>
        ),
        options: childItems,
        onNavigate: (href) => {
          if (href.startsWith("#create-child-")) {
            handleCreateChild(`semio.sketchpad.navbar.design.parent.${parent.guid}`, parent, false);
          } else {
            navigate(href);
          }
        },
      });
    });

    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.design",
      content: (
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            if (design && typeof design === "object" && "name" in design && "guid" in design) {
              const designObj = design as Design;
              navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(designObj.name)}&select=${designObj.guid}`);
            }
          }}
          className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
        >
          {design && typeof design === "object" && "name" in design ? String((design as Design).name) : ""}
        </button>
      ),
      options: designChildItems,
      onNavigate: (href) => {
        if (href === "#create-child" && design && typeof design === "object" && "guid" in design) {
          handleCreateChild("semio.sketchpad.navbar.design", design as Design, false);
        } else navigate(href);
      },
    });
  }

  if (isTypeApp && type) {
    const versionIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitVersion");
    if (versionIndex !== -1) {
      breadcrumbItems[versionIndex] = {
        ...breadcrumbItems[versionIndex],
        options: artifactKinds,
        onNavigate: (href) => navigate(href),
      };
    }

    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.breadcrumb.types",
      content: (
        <a onClick={() => navigate(`/kits/${kitGuid}?kind=types`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
          <TypeIcon size={16} />
        </a>
      ),
      options: typeNameItems,
      onNavigate: (href) => {
        if (href === "#create-type") handleCreateType("semio.sketchpad.navbar.selectType");
        else navigate(href);
      },
    });
    typeFolderChain.forEach((folder) => {
      breadcrumbItems.push({
        id: `semio.sketchpad.navbar.folder.${folder.guid}`,
        content: (
          <a onClick={() => navigate(`/kits/${kitGuid}?kind=folders`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            {folder.name}
          </a>
        ),
      });
    });

    typeParentChain.forEach((parent, index) => {
      const childItems = typeParentChildItems.find((s) => s.parentGuid === parent.guid)?.items || [];
      breadcrumbItems.push({
        id: `semio.sketchpad.navbar.type.parent.${parent.guid}`,
        content: (
          <button
            type="button"
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              navigate(`/kits/${kitGuid}/types/${parent.guid}`);
            }}
            className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
          >
            {parent.name}
          </button>
        ),
        options: childItems,
        onNavigate: (href) => {
          if (href.startsWith("#create-child-")) {
            handleCreateChild(`semio.sketchpad.navbar.type.parent.${parent.guid}`, parent, true);
          } else {
            navigate(href);
          }
        },
      });
    });

    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.type",
      content: (
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            if (type && typeof type === "object" && "name" in type && "guid" in type) {
              const typeObj = type as Type;
              navigate(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(typeObj.name)}&select=${typeObj.guid}`);
            }
          }}
          className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
        >
          {type && typeof type === "object" && "name" in type ? String((type as Type).name) : ""}
        </button>
      ),
      options: typeChildItems,
      onNavigate: (href) => {
        if (href === "#create-child" && type && typeof type === "object" && "guid" in type) {
          handleCreateChild("semio.sketchpad.navbar.type", type as Type, true);
        } else navigate(href);
      },
    });
  }

  if (isQualityApp && quality) {
    const versionIndex = breadcrumbItems.findIndex((item) => item.id === "semio.sketchpad.navbar.kitVersion");
    if (versionIndex !== -1) {
      breadcrumbItems[versionIndex] = {
        ...breadcrumbItems[versionIndex],
        options: artifactKinds,
        onNavigate: (href) => navigate(href),
      };
    }

    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.breadcrumb.qualities",
      content: (
        <a onClick={() => navigate(`/kits/${kitGuid}?kind=qualities`)} className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
          <AwardIcon size={16} />
        </a>
      ),
    });
    breadcrumbItems.push({
      id: "semio.sketchpad.navbar.quality",
      content: (
        <button
          type="button"
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            navigate(`/kits/${kitGuid}?kind=qualities&key=${encodeURIComponent(quality.key)}&select=${quality.guid}`);
          }}
          className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable"
        >
          {quality.name}
        </button>
      ),
    });
  }

  return <Breadcrumb className="flex-1 min-w-0" items={breadcrumbItems} />;
};

type SearchResult = {
  type: "kit" | "design" | "type" | "quality" | "tutorial";
  item: KitShallow | DesignShallow | TypeShallow | Quality | { id: string; name: string; description?: string };
  kitGuid?: string;
};

const buildSearchResultPath = (result: SearchResult): string => {
  if (result.type === "kit") return `/kits/${(result.item as KitShallow).guid}`;
  if (result.type === "design") return `/kits/${result.kitGuid}/designs/${(result.item as DesignShallow).guid}`;
  if (result.type === "type") return `/kits/${result.kitGuid}/types/${(result.item as TypeShallow).guid}`;
  if (result.type === "quality") return `/kits/${result.kitGuid}?kind=qualities&select=${(result.item as Quality).guid}`;
  if (result.type === "tutorial") return `/?tutorial=${(result.item as { id: string }).id}`;
  return "";
};

const Search: FC = ({}) => {
  const navigate = useNavigate();
  const recentSearches = (useSketchpad((s) => s.recentSearches) as string[]) || [];
  const updateRecentSearches = useUpdateRecentSearches();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const kits = useKits();
  const tutorials = useAvailableTutorials();
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "p") {
        const activeElement = document.activeElement as HTMLElement | null;
        if (!open && activeElement && (activeElement.tagName === "INPUT" || activeElement.tagName === "TEXTAREA" || activeElement.isContentEditable)) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [open, setOpen]);

  const searchData = useMemo(() => {
    const results: SearchResult[] = [];
    kits.forEach((kit) => {
      results.push({ type: "kit", item: kit as KitShallow, kitGuid: kit.guid });
      (kit.designs || []).forEach((design) => {
        if (typeof design === "object") results.push({ type: "design", item: design as DesignShallow, kitGuid: kit.guid });
      });
      (kit.types || []).forEach((type) => {
        if (typeof type === "object") results.push({ type: "type", item: type as TypeShallow, kitGuid: kit.guid });
      });
      (kit.qualities || []).forEach((quality) => {
        if (typeof quality === "object") results.push({ type: "quality", item: quality as Quality, kitGuid: kit.guid });
      });
    });
    tutorials.forEach((tutorial) => {
      results.push({ type: "tutorial", item: { id: tutorial.id, name: tutorial.name, description: tutorial.description } });
    });
    return results;
  }, [kits, tutorials]);

  const searchIndex = useMemo(() => {
    const map = new Map<string, SearchResult>();
    searchData.forEach((result) => {
      const path = buildSearchResultPath(result);
      if (path) map.set(path, result);
    });
    return map;
  }, [searchData]);

  const fuse = useMemo(
    () =>
      new Fuse(searchData, {
        keys: [
          { name: "item.name", weight: 2 },
          { name: "item.title", weight: 2 },
          { name: "item.description", weight: 0.5 },
          { name: "item.key", weight: 1.5 },
          { name: "item.path", weight: 1 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [searchData],
  );

  const recentResults = useMemo(() => {
    return recentSearches.map((path) => searchIndex.get(path)).filter((result): result is SearchResult => !!result);
  }, [recentSearches, searchIndex]);

  const searchResults = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);

    const base = recentResults.length > 0 ? recentResults : searchData.slice(0, 20);
    return base.map((item, idx) => ({ item, refIndex: idx }) as FuseResult<SearchResult>);
  }, [fuse, query, recentResults, searchData]);

  const groupedSearchResults = useMemo(() => {
    return {
      tutorials: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "tutorial"),
      kits: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "kit"),
      designs: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "design"),
      types: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "type"),
      qualities: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "quality"),
    };
  }, [searchResults]);

  const store = useSketchpadStore();
  const tutorialStore = store.tutorialStore();

  const handleSelect = useCallback(
    (result: SearchResult) => {
      const path = buildSearchResultPath(result);
      if (path) {
        const next = [path, ...recentSearches.filter((entry) => entry !== path)].slice(0, 20);
        const changed = next.length !== recentSearches.length || next.some((entry, index) => entry !== recentSearches[index]);
        if (changed) updateRecentSearches(next);
      }
      setOpen(false);
      setQuery("");

      if (result.type === "tutorial") {
        const tutorial = result.item as Tutorial;
        tutorialStore.startTutorial(tutorial);
      } else if (path) {
        navigate(path);
      } else {
        const { type, item, kitGuid } = result;
        if (type === "kit") navigate(`/kits/${(item as KitShallow).guid}`);
        else if (type === "design") navigate(`/kits/${kitGuid}/designs/${(item as DesignShallow).guid}`);
        else if (type === "type") navigate(`/kits/${kitGuid}/types/${(item as TypeShallow).guid}`);
        else if (type === "quality") navigate(`/kits/${kitGuid}?kind=qualities&select=${(item as Quality).guid}`);
      }
    },
    [navigate, recentSearches, updateRecentSearches, tutorialStore],
  );

  const getIcon = (type: SearchResult["type"]) => {
    if (type === "kit") return <LocalKitIcon size={16} />;
    if (type === "design") return <LayoutIcon size={16} />;
    if (type === "type") return <TypeIcon size={16} />;
    if (type === "quality") return <AwardIcon size={16} />;
    if (type === "tutorial") return <TutorialIcon size={16} />;
    return null;
  };

  const getDisplayName = (result: SearchResult) => {
    const { type, item } = result;
    if (type === "quality") return (item as Quality).name;
    if (type === "tutorial") return (item as Tutorial).name;
    return (item as any).name || "";
  };

  const searchTitle = useLabel("semio.sketchpad.navbar.search.title");
  const searchDescription = useLabel("semio.sketchpad.navbar.search.description");
  const searchPlaceholder = useLabel("semio.sketchpad.navbar.search.placeholder");
  const searchNoResults = useLabel("semio.sketchpad.navbar.search.noResults");
  const kitsLabel = useLabel("semio.sketchpad.navbar.kits");
  const designsLabel = useLabel("semio.sketchpad.navbar.breadcrumb.designs");
  const typesLabel = useLabel("semio.sketchpad.navbar.breadcrumb.types");
  const qualitiesLabel = useLabel("semio.sketchpad.navbar.breadcrumb.qualities");
  const tutorialsLabel = useLabel("semio.sketchpad.navbar.tutorials");

  return (
    <>
      <Toggle id="semio.sketchpad.navbar.search.open" i18nPressed="semio.sketchpad.navbar.search.close" pressed={open} onPressedChange={setOpen} icon={<SearchIcon size={16} />} />
      <CommandDialog title={searchTitle} description={searchDescription} open={open} onOpenChange={setOpen}>
        <CommandInput id="semio.sketchpad.navbar.searchInput" placeholder={searchPlaceholder} value={query} onValueChange={setQuery} />
        <CommandList>
          <CommandEmpty>{searchNoResults}</CommandEmpty>
          {groupedSearchResults.kits.length > 0 && (
            <CommandGroup heading={kitsLabel}>
              {groupedSearchResults.kits.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`kit-${(r.item.item as KitShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                  <div className="flex items-center gap-single">
                    {getIcon(r.item.type)}
                    <span>{getDisplayName(r.item)}</span>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
          {groupedSearchResults.designs.length > 0 && (
            <CommandGroup heading={designsLabel}>
              {groupedSearchResults.designs.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`design-${(r.item.item as DesignShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                  <div className="flex items-center gap-single">
                    {getIcon(r.item.type)}
                    <span>{getDisplayName(r.item)}</span>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
          {groupedSearchResults.types.length > 0 && (
            <CommandGroup heading={typesLabel}>
              {groupedSearchResults.types.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`type-${(r.item.item as TypeShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                  <div className="flex items-center gap-single">
                    {getIcon(r.item.type)}
                    <span>{getDisplayName(r.item)}</span>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
          {groupedSearchResults.qualities.length > 0 && (
            <CommandGroup heading={qualitiesLabel}>
              {groupedSearchResults.qualities.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`quality-${(r.item.item as Quality).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                  <div className="flex items-center gap-single">
                    {getIcon(r.item.type)}
                    <span>{getDisplayName(r.item)}</span>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
          {groupedSearchResults.tutorials.length > 0 && (
            <CommandGroup heading={tutorialsLabel}>
              {groupedSearchResults.tutorials.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`tutorial-${(r.item.item as Tutorial).id}-${idx}`} onSelect={() => handleSelect(r.item)}>
                  <div className="flex items-center gap-single">
                    {getIcon(r.item.type)}
                    <span>{getDisplayName(r.item)}</span>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
        </CommandList>
      </CommandDialog>
    </>
  );
};

const Focus: FC = ({}) => {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const focusContext = useFocusSafe();
  const appType = useAppType();
  const recentFocusMap = (useSketchpad((s) => s.recentFocusItems) as Record<string, string[]>) || {};
  const recentFocusIds = recentFocusMap[appType] || [];
  const updateRecentFocusItems = useUpdateRecentFocusItems();

  const focusItems = focusContext?.focusItems || [];
  const triggerFocusItem = focusContext?.triggerFocusItem;
  const focusItemIndex = useMemo(() => {
    const map = new Map<string, FocusItem>();
    focusItems.forEach((item) => map.set(item.id, item));
    return map;
  }, [focusItems]);
  const recentFocusItems = useMemo(() => {
    return recentFocusIds.map((id) => focusItemIndex.get(id)).filter((item): item is FocusItem => !!item);
  }, [recentFocusIds, focusItemIndex]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "f") {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const fuse = useMemo(
    () =>
      new Fuse(focusItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [focusItems],
  );

  const focusResults = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);

    const base = recentFocusItems.length > 0 ? recentFocusItems : focusItems.slice(0, 20);
    return base.map((item, idx) => ({ item, refIndex: idx }));
  }, [fuse, query, recentFocusItems, focusItems]);

  const handleSelect = useCallback(
    (item: FocusItem) => {
      const next = [item.id, ...recentFocusIds.filter((id) => id !== item.id)].slice(0, 20);
      const changed = next.length !== recentFocusIds.length || next.some((id, index) => id !== recentFocusIds[index]);
      if (changed) updateRecentFocusItems(appType, next);
      setOpen(false);
      setQuery("");
      if (triggerFocusItem) triggerFocusItem(item.id);
    },
    [appType, recentFocusIds, updateRecentFocusItems, triggerFocusItem],
  );

  const focusOtherLabel = useLabel("semio.sketchpad.navbar.focus.other");
  const groupedResults = useMemo(() => {
    const groups: Record<string, typeof focusResults> = {};
    focusResults.forEach((result) => {
      const category = result.item.category || focusOtherLabel;
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [focusResults, focusOtherLabel]);

  if (!focusContext) return null;

  const focusTitle = useLabel("semio.sketchpad.navbar.focus.title");
  const focusDescription = useLabel("semio.sketchpad.navbar.focus.description");
  const focusPlaceholder = useLabel("semio.sketchpad.navbar.focus.placeholder");
  const focusNoResults = useLabel("semio.sketchpad.navbar.search.noResults");

  return (
    <>
      <Toggle id="semio.sketchpad.navbar.focus.open" i18nPressed="semio.sketchpad.navbar.focus.close" pressed={open} onPressedChange={setOpen} icon={<FocusIcon size={16} />} />
      <CommandDialog title={focusTitle} description={focusDescription} open={open} onOpenChange={setOpen}>
        <CommandInput id="semio.sketchpad.navbar.focus.input" placeholder={focusPlaceholder} value={query} onValueChange={setQuery} />
        <CommandList>
          <CommandEmpty>{focusNoResults}</CommandEmpty>
          {Object.entries(groupedResults).map(([category, items]) => (
            <CommandGroup key={category} heading={category}>
              {items.map((result, idx) => (
                <CommandItem key={`${result.item.id}-${idx}`} onSelect={() => handleSelect(result.item)}>
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description && <span className="text-xs text-muted-foreground">{result.item.description}</span>}
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          ))}
        </CommandList>
      </CommandDialog>
    </>
  );
};

const PanelToggles: FC = ({}) => {
  const appType = useAppType();
  const visiblePanels = useAppPanelVisibility();
  const appCommands = useAppCommands();
  const leftTabs = useSidePanelTabs("left");
  const rightTabs = useSidePanelTabs("right");
  const hudTabs = useHudPanelTabs();

  const hasLeftTabs = leftTabs.length > 0;
  const hasRightTabs = rightTabs.length > 0;
  const hasHudTabs = hudTabs.length > 0;

  const isLeftOpen = visiblePanels.leftSidePanel ?? false;
  const isRightOpen = visiblePanels.rightSidePanel ?? false;
  const isHudOpen = visiblePanels.hudPanel ?? false;

  const handleLeftToggle = useCallback(
    (pressed: boolean) => {
      appCommands?.togglePanel?.("semio.sketchpad.navbar.panelToggle.leftSidePanel", "leftSidePanel");
    },
    [appCommands],
  );

  const handleHudToggle = useCallback(
    (pressed: boolean) => {
      appCommands?.togglePanel?.("semio.sketchpad.navbar.panelToggle.hudPanel", "hudPanel");
    },
    [appCommands],
  );

  const handleRightToggle = useCallback(
    (pressed: boolean) => {
      appCommands?.togglePanel?.("semio.sketchpad.navbar.panelToggle.rightSidePanel", "rightSidePanel");
    },
    [appCommands],
  );

  const LeftIcon = leftTabs[0]?.icon;
  const HudIcon = hudTabs[0]?.icon;
  const RightIcon = rightTabs[0]?.icon;

  if (!hasLeftTabs && !hasHudTabs && !hasRightTabs) return null;

  return (
    <div className="flex items-stretch border border-element overflow-hidden h-medium divide-x divide-element">
      {hasLeftTabs && (
        <Toggle kind="icon" id="semio.sketchpad.navbar.panelToggle.leftSidePanel" pressed={isLeftOpen} onPressedChange={handleLeftToggle} className="border-0">
          {LeftIcon ? <LeftIcon size={16} /> : <LayoutIcon size={16} />}
        </Toggle>
      )}
      {hasHudTabs && (
        <Toggle kind="icon" id="semio.sketchpad.navbar.panelToggle.hudPanel" pressed={isHudOpen} onPressedChange={handleHudToggle} className="border-0">
          {HudIcon ? <HudIcon size={16} /> : <FocusIcon size={16} />}
        </Toggle>
      )}
      {hasRightTabs && (
        <Toggle kind="icon" id="semio.sketchpad.navbar.panelToggle.rightSidePanel" pressed={isRightOpen} onPressedChange={handleRightToggle} className="border-0">
          {RightIcon ? <RightIcon size={16} /> : <DocumentIcon size={16} />}
        </Toggle>
      )}
    </div>
  );
};

// #region 🔖Canvas

export { createDefaultLayout } from "./shared";
export type { AppWindowConfig, WindowControl, WindowKindDefinition } from "./shared";

export type WindowConfig = {
  id: string;
  children: React.ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
  loading?: boolean;
  error?: Error | null;
  skeleton?: React.ReactNode;
  showControls?: boolean;
  onOpenInNewWindow?: () => void;
  onMaximize?: () => void;
  onMinimize?: () => void;
  onClose?: () => void;
  controls?: React.ReactNode;
};

const CanvasContext = createContext<{ activeWindow?: string; onActiveWindowChange?: (windowId: string) => void } | null>(null);

export function useCanvasContext() {
  const context = useContext(CanvasContext);
  return context;
}

export const Canvas: FC<{ children: ReactNode; id?: string }> = ({ children, id }) => {
  return (
    <div id={id} className="h-full w-full box-border p-single">
      {children}
    </div>
  );
};

export const HorizontalWindows: FC<{ children: ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full gap-single">{children}</div>;
};

export const VerticalWindows: FC<{ children: ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full gap-single">{children}</div>;
};

const WindowControlsGroup: FC<{ controls: WindowControl[] }> = ({ controls }) => {
  if (!controls || controls.length === 0) return null;

  return (
    <div className="flex items-stretch border overflow-hidden h-large">
      {controls.map((control) => {
        if (control.kind === "toggle") {
          return (
            <Toggle
              key={control.id}
              id={control.id}
              pressed={control.value === "true" || control.value === "1"}
              onPressedChange={(pressed) => {
                if (control.onChange) {
                  control.onChange(pressed ? "true" : "false");
                }
              }}
              icon={control.icon}
            />
          );
        } else if (control.kind === "dropdown" && control.options) {
          return (
            <Toggle
              key={control.id}
              kind="dropdown"
              id={control.id}
              value={control.value || control.options[0]?.value}
              onValueChange={(value) => {
                if (control.onChange) {
                  control.onChange(value);
                }
              }}
              items={control.options.map((opt) => ({
                value: opt.value,
                label: opt.icon,
                id: `${control.id}.${opt.value}`,
              }))}
              pressed={!!control.value}
            >
              {control.icon}
            </Toggle>
          );
        }
        return null;
      })}
    </div>
  );
};

interface LayoutErrorBoundaryProps {
  children: ReactNode;
  windowId: string;
  onError?: (error: Error, info: React.ErrorInfo) => void;
}
interface LayoutErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}
class LayoutErrorBoundary extends React.Component<LayoutErrorBoundaryProps, LayoutErrorBoundaryState> {
  constructor(props: LayoutErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }
  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }
  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.props.onError?.(error, errorInfo);
  }
  componentDidUpdate(prevProps: LayoutErrorBoundaryProps) {
    if (prevProps.children !== this.props.children && this.state.hasError) {
      this.setState({ hasError: false, error: null });
    }
  }
  render() {
    if (this.state.hasError) {
      return (
        <div className="flex flex-col items-center justify-center h-full gap-2 p-4">
          <p className="text-sm text-muted-foreground">Error rendering {this.props.windowId}</p>
          <p className="text-xs text-red-500 font-mono whitespace-pre-wrap max-w-full overflow-auto" data-testid="layout-error-message">
            {this.state.error?.message}
          </p>
          <p className="text-xs text-muted-foreground font-mono whitespace-pre-wrap max-w-full overflow-auto max-h-32" data-testid="layout-error-stack">
            {this.state.error?.stack?.split("\n").slice(0, 5).join("\n")}
          </p>
        </div>
      );
    }
    return this.props.children;
  }
}

export const LayoutCanvas: FC<{
  windowConfig: AppWindowConfig;
  layoutState?: any;
  onLayoutChange?: (layout: any) => void;
  activeWindow?: string;
  onActiveWindowChange?: (windowId: string) => void;
  children?: ReactNode;
}> = ({ windowConfig, layoutState, onLayoutChange, activeWindow, onActiveWindowChange, children }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<any>(null);
  const [layoutLoaded, setLayoutLoaded] = useState(false);
  const [hoveredSplitter, setHoveredSplitter] = useState<{ element: HTMLElement; direction: "horizontal" | "vertical" } | null>(null);
  const hoveredSplitterElementRef = useRef<HTMLElement | null>(null);
  const handleSplitterHoverRef = useRef<((e: MouseEvent) => void) | null>(null);
  const handleSplitterLeaveRef = useRef<((e: MouseEvent) => void) | null>(null);
  const sketchpadScope = useSketchpadScope();
  const sketchpadActor = useSketchpadActorSafe();
  const location = useLocation();
  const navigate = useNavigate();

  const scopeGuids = useMemo(() => {
    const pathMatch = location.pathname.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
    return {
      kit: pathMatch?.[1],
      itemType: pathMatch?.[2],
      item: pathMatch?.[3],
    };
  }, [location.pathname]);

  const LayoutScopeWrapper: FC<{ children: ReactNode }> = ({ children }) => {
    const { kit, itemType, item } = scopeGuids;

    let wrapped = <>{children}</>;

    if (item && kit) {
      if (itemType === "qualities") {
        wrapped = <QualityScopeProvider guid={item}>{wrapped}</QualityScopeProvider>;
      } else if (itemType === "types") {
        wrapped = <TypeScopeProvider guid={item}>{wrapped}</TypeScopeProvider>;
      } else if (itemType === "designs") {
        wrapped = <DesignScopeProvider guid={item}>{wrapped}</DesignScopeProvider>;
      }
    }
    if (kit) {
      wrapped = <KitScopeProvider guid={kit}>{wrapped}</KitScopeProvider>;
    }
    if (sketchpadActor) {
      wrapped = <SketchpadActorContext.Provider value={sketchpadActor}>{wrapped}</SketchpadActorContext.Provider>;
    }
    if (sketchpadScope) {
      wrapped = <SketchpadScopeContext.Provider value={sketchpadScope}>{wrapped}</SketchpadScopeContext.Provider>;
    }
    wrapped = (
      <OriginProvider>
        <FocusProvider>
          <PanelSectionProvider>
            <FooterItemProvider>{wrapped}</FooterItemProvider>
          </PanelSectionProvider>
        </FocusProvider>
      </OriginProvider>
    );

    return wrapped;
  };

  const contextValue = useMemo(
    () => ({
      activeWindow,
      onActiveWindowChange,
    }),
    [activeWindow, onActiveWindowChange],
  );

  const setActiveSplitter = useCallback((element: HTMLElement, direction: "horizontal" | "vertical") => {
    if (hoveredSplitterElementRef.current && hoveredSplitterElementRef.current !== element) {
      hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
    }
    hoveredSplitterElementRef.current = element;
    element.classList.add("relative", "overflow-visible");
    setHoveredSplitter({ element, direction });
  }, []);

  const clearHoveredSplitter = useCallback(() => {
    if (hoveredSplitterElementRef.current) {
      hoveredSplitterElementRef.current.classList.remove("relative", "overflow-visible");
      hoveredSplitterElementRef.current = null;
    }
    setHoveredSplitter(null);
  }, []);

  const handleAddWindow = useCallback(
    (windowTypeId: string, direction: "horizontal" | "vertical", splitterElement: HTMLElement) => {
      if (!layoutRef.current) {
        return;
      }
      if (!splitterElement) {
        return;
      }

      const windowType = windowConfig.windowKinds.find((wt) => wt.id === windowTypeId);
      if (!windowType) {
        return;
      }

      const newItemConfig = {
        type: "stack",
        content: [
          {
            type: "component",
            componentName: windowTypeId,
            title: typeof windowType.label === "string" ? windowType.label : windowTypeId,
          },
        ],
      };

      const splitter = splitterElement;

      try {
        const findAllItems = (item: any): any[] => {
          const items = [item];
          if (item.contentItems && Array.isArray(item.contentItems)) {
            item.contentItems.forEach((child: any) => {
              items.push(...findAllItems(child));
            });
          }
          return items;
        };

        let parent = null;
        let insertIndex = -1;
        const allItems = findAllItems(layoutRef.current.root);

        for (const item of allItems) {
          if (item.type === "row" || item.type === "column") {
            const itemEl = item.element?.[0] || item.element;
            if (itemEl && itemEl.contains && itemEl.contains(splitter)) {
              parent = item;
              if (parent.contentItems && Array.isArray(parent.contentItems)) {
                const children = Array.from(itemEl.children || []);
                let splitterIndex = -1;
                for (let i = 0; i < children.length; i++) {
                  if (children[i] === splitter) {
                    splitterIndex = i;
                    break;
                  }
                }
                if (splitterIndex === -1) {
                  for (let i = 0; i < children.length; i++) {
                    const child = children[i] as HTMLElement;
                    if (child.contains && child.contains(splitter)) {
                      splitterIndex = i;
                      break;
                    }
                  }
                }
                if (splitterIndex >= 0) {
                  let itemsBeforeSplitter = 0;
                  for (let i = 0; i < splitterIndex; i++) {
                    const child = children[i] as HTMLElement;
                    if (!child.classList.contains("lm_splitter")) {
                      itemsBeforeSplitter++;
                    }
                  }
                  insertIndex = itemsBeforeSplitter;
                }
              }
              break;
            }
          }
        }

        if (parent && insertIndex >= 0) {
          layoutRef.current.root.addItem(newItemConfig, parent, insertIndex);
        } else if (parent) {
          layoutRef.current.root.addItem(newItemConfig, parent);
        } else {
          layoutRef.current.root.addItem(newItemConfig);
        }
      } catch (error) {
        console.error("Error adding window:", error);
      }

      clearHoveredSplitter();
    },
    [windowConfig.windowKinds, clearHoveredSplitter],
  );

  useEffect(() => {
    if (!containerRef.current || layoutRef.current) return;

    const loadGoldenLayout = async () => {
      try {
        const goldenLayoutModule = await import("golden-layout");
        const GoldenLayout = (goldenLayoutModule as any).GoldenLayout;

        if (typeof GoldenLayout !== "function") {
          console.error("GoldenLayout is not a constructor", { goldenLayoutModule, GoldenLayout });
          return;
        }

        const normalizeLayoutConfig = (config: any): any => {
          if (!config || typeof config !== "object") return config;
          if (Array.isArray(config)) {
            return config.map((item) => normalizeLayoutConfig(item));
          }

          const normalized: any = {};

          for (const [key, value] of Object.entries(config)) {
            const unitKey = `${key}Unit` as string;
            const hasUnitField = unitKey in config;

            if (hasUnitField) {
              const unit = config[unitKey];
              if (typeof value === "string") {
                const numValue = parseFloat(value);
                normalized[key] = !isNaN(numValue) ? `${numValue}${unit}` : `1${unit}`;
              } else if (typeof value === "number") {
                normalized[key] = `${value}${unit}`;
              } else {
                normalized[key] = `1${unit}`;
              }
            } else if (key === "size" || key === "width" || key === "height") {
              if (typeof value === "string") {
                normalized[key] = value.trim() === "" && key === "size" ? "50%" : value;
              } else if (typeof value === "number") {
                normalized[key] = `${value}%`;
              } else {
                normalized[key] = "50%";
              }
            } else if (key === "title" || key === "componentName" || key === "componentType" || key === "type" || key === "id") {
              if (typeof value === "string") {
                if (value.trim() !== "") {
                  normalized[key] = value;
                }
              } else if (value !== null && value !== undefined) {
                normalized[key] = String(value);
              }
            } else if (key === "content" && Array.isArray(value)) {
              if (value.length > 0 || config.type !== "component") {
                normalized[key] = value.map((item) => normalizeLayoutConfig(item));
              }
            } else if (key === "componentState") {
              normalized[key] = value;
            } else if (typeof value === "object" && value !== null) {
              normalized[key] = normalizeLayoutConfig(value);
            } else {
              normalized[key] = value;
            }
          }

          return normalized;
        };

        const rawConfig = parseWindowLayout(layoutState) || parseWindowLayout(windowConfig.defaultLayout);
        if (!rawConfig) {
          console.error("[LayoutCanvas] No layout config provided!");
          return;
        }
        const config = normalizeLayoutConfig(rawConfig);

        const layout = new GoldenLayout(config, containerRef.current!);
        let isInitialized = false;

        windowConfig.windowKinds.forEach((windowType) => {
          layout.registerComponent(windowType.id, (container: any, componentState: any) => {
            const element = container.getElement();
            let domElement: HTMLElement;

            if (element instanceof HTMLElement) {
              domElement = element;
            } else if (Array.isArray(element) && element[0] instanceof HTMLElement) {
              domElement = element[0];
            } else if (element && (element as any)[0] instanceof HTMLElement) {
              domElement = (element as any)[0];
            } else if (element && element.nodeType === 1) {
              domElement = element as HTMLElement;
            } else {
              console.error("Could not extract DOM element from container", { element, container });
              return;
            }

            const root = createRoot(domElement);
            const WindowComponent = windowType.component;

            const WrappedComponent = () => {
              const clickGoldenLayoutControl = (selector: string) => {
                const stackElement = domElement.closest(".lm_item.lm_stack") as HTMLElement | null;
                const controlElement = (stackElement?.querySelector(selector) as HTMLElement | null) ?? null;
                if (controlElement) {
                  controlElement.click();
                }
              };
              return (
                <MemoryRouter initialEntries={[location.pathname + location.search]} initialIndex={0}>
                  <LayoutScopeWrapper>
                    <DragDropProvider>
                      <LayoutErrorBoundary
                        windowId={windowType.id}
                        onError={(error: Error, info: React.ErrorInfo) => {
                          console.error("Error in window:", windowType.id, error, info);
                        }}
                      >
                        <Window
                          id={windowType.id}
                          isVisible={true}
                          showControls={true}
                          onOpenInNewWindow={() => clickGoldenLayoutControl(".lm_popout")}
                          onMaximize={() => clickGoldenLayoutControl(".lm_maximise")}
                          onMinimize={() => clickGoldenLayoutControl(".lm_maximise")}
                          onClose={() => clickGoldenLayoutControl(".lm_close")}
                          controls={windowType.controls ? <WindowControlsGroup controls={windowType.controls} /> : undefined}
                        >
                          <ReactFlowProvider>
                            <WindowComponent />
                          </ReactFlowProvider>
                        </Window>
                      </LayoutErrorBoundary>
                    </DragDropProvider>
                  </LayoutScopeWrapper>
                </MemoryRouter>
              );
            };

            root.render(<WrappedComponent />);
            container.on("destroy", () => {
              setTimeout(() => {
                root.unmount();
              }, 0);
            });
          });
        });

        layout.on("stateChanged", () => {
          if (!onLayoutChange) return;

          try {
            if (isInitialized) {
              const config = layout.toConfig();
              onLayoutChange(config);
            }
          } catch (error: any) {
            if (error?.message?.includes("not yet initialised")) {
              return;
            }
            console.warn("Failed to get layout config:", error);
          }
        });

        layout.on("tab", (tab: any) => {
          if (tab._header) {
            tab._header.on("click", () => {
              const componentName = tab._contentItem?.config?.componentName;
              if (componentName && onActiveWindowChange) {
                onActiveWindowChange(componentName);
              }
            });
          }
        });

        const customizeHeaders = () => {
          try {
            const getAllStacks = (item: any): any[] => {
              const stacks: any[] = [];
              if (item.type === "stack") {
                stacks.push(item);
              }
              if (item.contentItems) {
                item.contentItems.forEach((child: any) => {
                  stacks.push(...getAllStacks(child));
                });
              }
              return stacks;
            };

            const stacks = getAllStacks(layout.root);
            stacks.forEach((stack: any) => {
              if (stack.header && stack.header.controlsContainer) {
                stack.header.controlsContainer.on("open", () => {
                  const activeContentItem = stack.getActiveContentItem?.();
                  if (activeContentItem) {
                    const componentName = activeContentItem.config?.componentName;
                    if (componentName && onActiveWindowChange) {
                      onActiveWindowChange(componentName);
                    }
                  }
                });
              }
            });
          } catch (error) {
            console.warn("Failed to customize headers:", error);
          }
        };

        layout.on("initialised", () => {
          customizeHeaders();
          isInitialized = true;
          setLayoutLoaded(true);
        });

        try {
          layout.init();
          layoutRef.current = layout;
        } catch (error: any) {
          console.error("[GoldenLayout] Failed to initialize:", error);
          console.error("[GoldenLayout] Config that failed:", JSON.stringify(config, null, 2));
          throw error;
        }

        handleSplitterHoverRef.current = (e: MouseEvent) => {
          const target = e.target as HTMLElement;
          const buttonContainer = target.closest("[data-splitter-buttons]") as HTMLElement | null;
          if (buttonContainer) {
            return;
          }
          const splitter = target.closest(".lm_splitter") as HTMLElement | null;
          if (splitter) {
            const isHorizontal = splitter.classList.contains("lm_splitter_horizontal");
            setActiveSplitter(splitter, isHorizontal ? "horizontal" : "vertical");
          } else {
            clearHoveredSplitter();
          }
        };

        handleSplitterLeaveRef.current = (e: MouseEvent) => {
          const target = e.target as HTMLElement;
          const relatedTarget = e.relatedTarget as Node | null;
          const splitter = target.closest(".lm_splitter") as HTMLElement | null;
          const buttonContainer = relatedTarget && ((relatedTarget as HTMLElement).closest?.("[data-splitter-buttons]") as HTMLElement | null);
          if (splitter && relatedTarget && splitter.contains(relatedTarget)) {
            return;
          }
          if (buttonContainer) {
            return;
          }
          if (splitter && relatedTarget && (relatedTarget as HTMLElement).closest?.(".lm_splitter") === splitter) {
            return;
          }
          clearHoveredSplitter();
        };

        if (containerRef.current && handleSplitterHoverRef.current && handleSplitterLeaveRef.current) {
          containerRef.current.addEventListener("mouseover", handleSplitterHoverRef.current);
          containerRef.current.addEventListener("mouseout", handleSplitterLeaveRef.current);
        }
      } catch (error) {
        console.error("Failed to load GoldenLayout:", error);
      }
    };

    loadGoldenLayout();

    return () => {
      clearHoveredSplitter();
      if (containerRef.current && handleSplitterHoverRef.current && handleSplitterLeaveRef.current) {
        containerRef.current.removeEventListener("mouseover", handleSplitterHoverRef.current);
        containerRef.current.removeEventListener("mouseout", handleSplitterLeaveRef.current);
      }
      if (layoutRef.current) {
        try {
          layoutRef.current.destroy();
        } catch (error) {
          console.error("Error destroying layout:", error);
        }
        layoutRef.current = null;
        setLayoutLoaded(false);
      }
    };
  }, [windowConfig, layoutState, onLayoutChange, onActiveWindowChange, sketchpadScope, location, navigate, scopeGuids, setActiveSplitter, clearHoveredSplitter]);

  useEffect(() => {
    if (!layoutRef.current || !layoutLoaded) return;

    const updateActiveTab = () => {
      if (!activeWindow) return;
      try {
        const getAllStacks = (item: any): any[] => {
          const stacks: any[] = [];
          if (item.type === "stack") {
            stacks.push(item);
          }
          if (item.contentItems) {
            item.contentItems.forEach((child: any) => {
              stacks.push(...getAllStacks(child));
            });
          }
          return stacks;
        };

        const stacks = getAllStacks(layoutRef.current.root);
        stacks.forEach((stack: any) => {
          const getAllComponents = (item: any): any[] => {
            const components: any[] = [];
            if (item.type === "component") {
              components.push(item);
            }
            if (item.contentItems) {
              item.contentItems.forEach((child: any) => {
                components.push(...getAllComponents(child));
              });
            }
            return components;
          };

          const components = getAllComponents(stack);
          components.forEach((item: any) => {
            if (item.config?.componentName === activeWindow && stack.setActiveContentItem) {
              stack.setActiveContentItem(item);
            }
          });
        });
      } catch (error) {
        console.warn("Failed to update active tab:", error);
      }
    };

    updateActiveTab();
  }, [activeWindow, layoutLoaded]);

  return (
    <CanvasContext.Provider value={contextValue}>
      <div ref={containerRef} className="h-full w-full">
        {children}
        {hoveredSplitter &&
          hoveredSplitter.element &&
          createPortal(
            <LevelProvider level="temporary">
              <div data-splitter-buttons className="pointer-events-auto absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-temporary border border-element p-single">
                <ActionGroup>
                  {windowConfig.windowKinds.map((windowType) => {
                    const typeId = windowType.id;
                    const direction = hoveredSplitter.direction;
                    const splitterElement = hoveredSplitter.element;
                    if (!splitterElement) {
                      return null;
                    }
                    return (
                      <ActionGroupItem
                        key={typeId}
                        type="button"
                        disabled={!layoutLoaded}
                        onClick={(e: React.MouseEvent) => {
                          e.stopPropagation();
                          if (!splitterElement) {
                            return;
                          }
                          if (!layoutRef.current) {
                            return;
                          }
                          handleAddWindow(typeId, direction, splitterElement);
                        }}
                        onMouseEnter={() => {
                          hoveredSplitterElementRef.current = splitterElement;
                        }}
                        title={typeof windowType.label === "string" ? windowType.label : typeId}
                        text={typeof windowType.label === "string" ? windowType.label : typeId}
                      />
                    );
                  })}
                </ActionGroup>
              </div>
            </LevelProvider>,
            hoveredSplitter.element,
          )}
      </div>
    </CanvasContext.Provider>
  );
};

// #endregion 🔖Canvas

// #region 🔖App Router

const ScopeWrapper: FC<{ ScopeProvider: ComponentType<{ guid: string; children: ReactNode }>; paramName: string; children: ReactNode }> = ({ ScopeProvider, paramName, children }) => {
  const params = useParams();
  const guid = params[paramName];
  if (!guid) return <>{children}</>;
  return <ScopeProvider guid={guid}>{children}</ScopeProvider>;
};

const AppRouter: FC = () => {
  const [appsInitialized, setAppsInitialized] = useState(() => appRegistry.isInitialized);

  useEffect(() => {
    if (!appsInitialized) {
      appRegistry.initialize().then(() => setAppsInitialized(true));
    }
  }, [appsInitialized]);

  const apps = useMemo(() => {
    if (!appsInitialized) return [];
    const sortedApps = appRegistry.getAllApps();
    return sortedApps;
  }, [appsInitialized]);

  const buildRoute = (app: AppRegistration, segments: RouteSegment[], index: number = 0): ReactNode => {
    if (index >= segments.length) {
      return <Route key={app.id} path="*" element={<app.component />} />;
    }

    const segment = segments[index];
    const paramName = segment.paramName;
    const ScopeProvider = segment.scopeProvider;
    const isLast = index === segments.length - 1;
    const routePath = index === 0 && !segment.path.startsWith("/") ? `/${segment.path}` : segment.path;

    if (ScopeProvider && paramName) {
      return (
        <Route
          key={`${app.id}-${index}`}
          path={routePath}
          element={
            <ScopeWrapper ScopeProvider={ScopeProvider} paramName={paramName}>
              {isLast ? <app.component /> : <Outlet />}
            </ScopeWrapper>
          }
        >
          {!isLast && buildRoute(app, segments, index + 1)}
        </Route>
      );
    }

    return (
      <Route key={`${app.id}-${index}`} path={routePath} element={isLast ? <app.component /> : <Outlet />}>
        {!isLast && buildRoute(app, segments, index + 1)}
      </Route>
    );
  };

  if (!appsInitialized) {
    return <div className="h-screen w-screen" />;
  }

  return (
    <Routes>
      {apps.map((app) => {
        if (app.routeSegments.length === 0) {
          const paths = app.additionalPaths || [];
          return [<Route key={`${app.id}-root`} path="/" element={<app.component />} />, ...paths.map((path) => <Route key={`${app.id}-${path}`} path={path} element={<app.component />} />)];
        }
        return buildRoute(app, app.routeSegments);
      })}
    </Routes>
  );
};

// #endregion 🔖App Router

// #region 🔖Sketchpad Components

const ToolbarScopeWrapper: FC<{ children: ReactNode }> = ({ children }) => {
  const location = useLocation();
  const scopeGuids = useMemo(() => {
    const pathMatch = location.pathname.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
    return {
      kit: pathMatch?.[1],
      itemType: pathMatch?.[2],
      item: pathMatch?.[3],
    };
  }, [location.pathname]);

  const { kit, itemType, item } = scopeGuids;
  let wrapped = <>{children}</>;

  if (item && kit) {
    if (itemType === "qualities") {
      wrapped = <QualityScopeProvider guid={item}>{wrapped}</QualityScopeProvider>;
    } else if (itemType === "types") {
      wrapped = <TypeScopeProvider guid={item}>{wrapped}</TypeScopeProvider>;
    } else if (itemType === "designs") {
      wrapped = <DesignScopeProvider guid={item}>{wrapped}</DesignScopeProvider>;
    }
  }
  if (kit) {
    wrapped = <KitScopeProvider guid={kit}>{wrapped}</KitScopeProvider>;
  }

  return wrapped;
};

const LayoutWrapper: FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const reactNavigate = useReactNavigate();
  const store = useSketchpadStore();
  const tutorialStore = store.tutorialStore();

  const navigation = useNavigation();
  const [theme] = useTheme();
  const [language] = useLanguage();
  const [device] = useDevice();
  const [isFullscreen] = useFullscreen();
  const isNavbarExpanded = useIsNavbarExpanded();
  const isFooterExpanded = useIsFooterExpanded();
  const panelVisibility = useAppPanelVisibility();
  const appType = useAppType();
  const panelSizes = usePanelSizes();
  const footerItems = useFooterItems();
  const workbenchSections = usePanelSections("workbench");
  const toolsSections = usePanelSections("tools");
  const toolbarSections = usePanelSections("toolbar");
  const hudSections = usePanelSections("hud");
  const statsSections = usePanelSections("stats");
  const detailsSections = usePanelSections("details");
  const chatSections = usePanelSections("chat");
  const settingsSections = usePanelSections("settings");
  const consoleSections = usePanelSections("console");

  const leftSidePanelTabs = useSidePanelTabs("left");
  const rightSidePanelTabs = useSidePanelTabs("right");
  const hudPanelTabs = useHudPanelTabs();
  const [activeLeftTabId, setActiveLeftTabId] = useActiveLeftTabId();
  const [activeRightTabId, setActiveRightTabId] = useActiveRightTabId();
  const [activeHudTabId, setActiveHudTabId] = useActiveHudTabId();

  const addSidePanelTab = useAddSidePanelTab();
  const removeSidePanelTab = useRemoveSidePanelTab();
  const addHudPanelTab = useAddHudPanelTab();
  const removeHudPanelTab = useRemoveHudPanelTab();
  const panelConfigs = usePanelConfigs();

  useEffect(() => {
    const panels = panelConfigs[appType] || [];
    const registeredIds: string[] = [];

    panels.forEach((panel) => {
      const config = panelKindConfigs[panel.kind];
      if (!config) return;

      const tab = {
        id: panel.id,
        icon: config.icon,
        order: 0,
        content: <></>,
      };

      if (config.position === PanelPosition.LEFT) {
        addSidePanelTab("left", tab);
        registeredIds.push(panel.id);
      } else if (config.position === PanelPosition.RIGHT) {
        addSidePanelTab("right", tab);
        registeredIds.push(panel.id);
      } else if (config.position === PanelPosition.MIDDLE) {
        addHudPanelTab(tab);
        registeredIds.push(panel.id);
      }
    });

    return () => {
      registeredIds.forEach((id) => {
        const panel = panels.find((p) => p.id === id);
        if (!panel) return;
        const config = panelKindConfigs[panel.kind];
        if (!config) return;

        if (config.position === PanelPosition.LEFT) removeSidePanelTab("left", id);
        else if (config.position === PanelPosition.RIGHT) removeSidePanelTab("right", id);
        else if (config.position === PanelPosition.MIDDLE) removeHudPanelTab(id);
      });
    };
  }, [appType, panelConfigs, addSidePanelTab, removeSidePanelTab, addHudPanelTab, removeHudPanelTab]);

  const sketchpadCommands = useSketchpadCommands();

  useEffect(() => {
    const migratedPath = migratePath(location.pathname);
    if (migratedPath !== location.pathname) {
      reactNavigate(migratedPath, { replace: true });
    }
  }, [location.pathname, reactNavigate]);

  useEffect(() => {
    const fullPath = location.pathname + location.search;
    const currentHistory = store.snapshot().navigationHistory || ["/"];
    const currentIndex = store.snapshot().navigationHistoryIndex ?? 0;
    const currentPath = currentHistory[currentIndex];

    if (currentPath !== fullPath) {
      store.execute("semio.sketchpad.addNavigation", "semio.sketchpad.sync", fullPath);
    } else {
      store.execute("semio.sketchpad.syncNavigation", "semio.sketchpad.sync", fullPath);
    }
  }, [location.pathname, location.search, store]);

  useEffect(() => {
    if (typeof document === "undefined") return;
    const handler = () => {
      const active = !!document.fullscreenElement;
      if (active !== isFullscreen) {
        sketchpadCommands.setState("semio.sketchpad.fullscreenChange", { isFullscreen: active });
      }
    };
    document.addEventListener("fullscreenchange", handler);
    return () => document.removeEventListener("fullscreenchange", handler);
  }, [isFullscreen, sketchpadCommands]);

  useEffect(() => {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    const shouldBeDark = theme === "dark" || (theme === "system" && prefersDark);
    if (shouldBeDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }, [theme]);

  useEffect(() => {
    if (language) {
      if (i18n.language !== language) {
        i18n
          .changeLanguage(language)
          .then(() => {})
          .catch((err) => {
            console.error("[Language Sync] Failed to change language:", err);
          });
      }
    }
  }, [language]);

  useEffect(() => {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    if (device === "tablet") {
      root.classList.add("touch");
    } else {
      root.classList.remove("touch");
    }
  }, [device]);

  useEffect(() => {
    if (typeof document === "undefined") return;
    const root = document.documentElement;
    if (isFullscreen) {
      root.classList.add("fullscreen");
    } else {
      root.classList.remove("fullscreen");
    }
  }, [isFullscreen]);

  const navigationHistory = useNavigationHistory();
  const currentPath = `${navigation}${location.search}`;
  const [searchParams] = useSearchParams();
  const kits = useKits();

  const pathParts = navigation.split("/").filter((p) => p);
  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
  const isKitsPath = pathParts[0] === "kits";
  const isDocsPath = pathParts[0] === "docs";
  const kitGuid = isKitsPath && pathParts[1] ? pathParts[1] : null;
  const secondPart = pathParts[2];
  const thirdPart = pathParts[3];
  const isDesignApp = isKitsPath && secondPart === "designs" && thirdPart && isUuidPattern(thirdPart);
  const isTypeApp = isKitsPath && secondPart === "types" && thirdPart && isUuidPattern(thirdPart);
  const isQualityApp = isKitsPath && secondPart === "qualities" && thirdPart && isUuidPattern(thirdPart);
  const isKitApp = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp;

  const kitFromScope = useKit();
  const designFromScope = useDesign();
  const typeFromScope = useType();
  const kit: Kit | KitShallow | null | undefined = (kitFromScope as Kit | KitShallow | null | undefined) || kits.find((k) => k.guid === kitGuid);
  const kitKind = useKitKind(kitGuid || "");

  const itemGuid = isDesignApp || isTypeApp || isQualityApp ? thirdPart : null;
  const allDesigns: Design[] = useMemo(() => {
    if (!kit?.designs) return [];
    return (kit.designs as any[]).filter((d): d is Design => typeof d === "object" && d.guid !== undefined);
  }, [kit?.designs]);
  const allTypes: Type[] = useMemo(() => {
    if (!kit?.types) return [];
    return (kit.types as any[]).filter((t): t is Type => typeof t === "object" && t.guid !== undefined);
  }, [kit?.types]);

  const design = useMemo(() => {
    if (isDesignApp && itemGuid) {
      return designFromScope || allDesigns.find((d) => d.guid === itemGuid) || null;
    }
    return null;
  }, [isDesignApp, itemGuid, designFromScope, allDesigns]);

  const type = useMemo(() => {
    if (isTypeApp && itemGuid) {
      return typeFromScope || allTypes.find((t) => t.guid === itemGuid) || null;
    }
    return null;
  }, [isTypeApp, itemGuid, typeFromScope, allTypes]);

  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;
  const filteredKind = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? (searchParams.get("kind") as "designs" | "types" | "qualities" | "files" | "authors" | null) : null;
  const filteredName = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? searchParams.get("name") : null;

  const upTarget = useMemo(() => {
    const pathWithoutQuery = currentPath.split("?")[0];

    if (homeKind) {
      if (homeVersion !== null && homeName) {
        const params = new URLSearchParams();
        params.set("kind", homeKind);
        params.set("name", homeName);
        return `/?${params.toString()}`;
      }
      if (homeName) {
        const params = new URLSearchParams();
        params.set("kind", homeKind);
        return `/?${params.toString()}`;
      }
      return "/";
    }

    if (pathWithoutQuery === "/") return undefined;
    if (pathWithoutQuery === "/kits") return "/";

    if (isDesignApp && design && typeof design === "object" && "parent" in design) {
      const designObj = design as Design;
      if (designObj.parent) {
        return `/kits/${kitGuid}/designs/${designObj.parent}`;
      }
      return `/kits/${kitGuid}?kind=designs`;
    }

    if (isTypeApp && type && typeof type === "object" && "parent" in type) {
      const typeObj = type as Type;
      if (typeObj.parent) {
        return `/kits/${kitGuid}/types/${typeObj.parent}`;
      }
      return `/kits/${kitGuid}?kind=types`;
    }

    if (isQualityApp) {
      return `/kits/${kitGuid}?kind=qualities`;
    }

    if (isKitApp) {
      if (filteredKind) {
        const kitObj = kit && typeof kit === "object" && "name" in kit ? (kit as Kit) : null;
        if (kitKind && kitObj) {
          const params = new URLSearchParams();
          params.set("kind", kitKind);
          if (kitObj.name) params.set("name", kitObj.name);
          if (kitObj.version !== undefined && kitObj.version !== null) {
            params.set("version", kitObj.version);
          }
          return `/?${params.toString()}`;
        }
        return `/kits/${kitGuid}`;
      }
      if (kitGuid && kitKind) {
        const kitObj = kit && typeof kit === "object" && "name" in kit ? (kit as Kit) : null;
        if (kitObj) {
          const params = new URLSearchParams();
          params.set("kind", kitKind);
          if (kitObj.name) params.set("name", kitObj.name);
          if (kitObj.version !== undefined && kitObj.version !== null) {
            params.set("version", kitObj.version);
          }
          return `/?${params.toString()}`;
        }
        return `/?kind=${kitKind}`;
      }
      return "/";
    }

    return pathWithoutQuery.split("/").slice(0, -1).join("/") || "/";
  }, [currentPath, isDesignApp, isTypeApp, isQualityApp, isKitApp, design, type, kitGuid, filteredKind, filteredName, kitKind, kit, homeKind, homeName, homeVersion]);
  const isAtRoot = currentPath === "/" || (currentPath === "/kits" && !kitGuid);

  const fullscreenToggleId = isFullscreen ? "semio.sketchpad.navbar.exitFullscreen" : "semio.sketchpad.navbar.fullscreen";

  const navbarItems = useMemo(() => {
    const items: NavbarItem[] = [];
    items.push({
      key: "navigationButtons",
      content: (
        <ButtonGroup id="semio.sketchpad.navbar.navigationButtons">
          <ButtonGroupItem value="back" id="semio.sketchpad.navbar.back" onClick={() => sketchpadCommands.navigateBack("semio.sketchpad.navbar.back")} disabled={!navigationHistory.canGoBack}>
            <NavigateBackIcon size={16} />
          </ButtonGroupItem>
          <ButtonGroupItem value="forward" id="semio.sketchpad.navbar.forward" onClick={() => sketchpadCommands.navigateForward("semio.sketchpad.navbar.forward")} disabled={!navigationHistory.canGoForward}>
            <NavigateForwardIcon size={16} />
          </ButtonGroupItem>
          <ButtonGroupItem
            value="up"
            id="semio.sketchpad.navbar.up"
            onClick={() => {
              if (upTarget) navigate(upTarget);
            }}
            disabled={isAtRoot}
          >
            <NavigateUpIcon size={16} />
          </ButtonGroupItem>
        </ButtonGroup>
      ),
    });
    items.push({ key: "navigation", content: <Navigation />, className: "flex-1 min-w-0" });
    items.push({ key: "search", content: <Search /> });
    items.push({ key: "focus", content: <Focus /> });
    items.push({ key: "panelToggles", content: <PanelToggles /> });
    items.push({
      key: "fullscreenToggle",
      content: (
        <Toggle
          id={fullscreenToggleId}
          pressed={isFullscreen}
          onPressedChange={() => {
            if (typeof document !== "undefined") {
              if (!isFullscreen) {
                const target = document.documentElement;
                if (target && target.requestFullscreen) {
                  const result = target.requestFullscreen();
                  if (result && typeof (result as any).then === "function") {
                    (result as Promise<void>).then(() => sketchpadCommands.toggleFullscreen(fullscreenToggleId)).catch(() => {});
                  } else {
                    sketchpadCommands.toggleFullscreen(fullscreenToggleId);
                  }
                }
              } else {
                if (document.fullscreenElement && document.exitFullscreen) {
                  const result = document.exitFullscreen();
                  if (result && typeof (result as any).then === "function") {
                    (result as Promise<void>).then(() => sketchpadCommands.toggleFullscreen(fullscreenToggleId)).catch(() => {});
                  } else {
                    sketchpadCommands.toggleFullscreen(fullscreenToggleId);
                  }
                }
              }
              return;
            }
            sketchpadCommands.toggleFullscreen(fullscreenToggleId);
          }}
          icon={isFullscreen ? <Minimize2Icon size={16} /> : <Maximize2Icon size={16} />}
        />
      ),
    });
    return items;
  }, [navigationHistory, upTarget, isAtRoot, navigate, sketchpadCommands, fullscreenToggleId, isFullscreen]);

  const activeInteraction = useActiveInteraction();
  const panelOpacity = activeInteraction === "dragging" ? 0.3 : 1;

  const [activeDragId, setActiveDragId] = useState<string | null>(null);
  const [activeDragData, setActiveDragData] = useState<any>(null);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
  );

  const customCollisionDetection = useCallback((args: any) => {
    const pointerCollisions = pointerWithin(args);

    if (pointerCollisions.length > 0) {
      return pointerCollisions;
    }

    const rectCollisions = rectIntersection(args);

    if (rectCollisions.length > 0) {
      return rectCollisions;
    }

    const centerCollisions = closestCenter(args);

    if (centerCollisions && centerCollisions.length > 0) {
      return centerCollisions;
    }
    return [];
  }, []);
  const kitShallows = useKitShallows();
  const getTypeOrDesignName = useCallback(() => {
    if (!activeDragData) return null;
    if (activeDragData.type === "type") {
      const kitId = activeDragData.typeGuid?.split("-")[0];
      const kit = kitShallows.find((k) => k.guid.startsWith(kitId));
      const type = kit?.types?.find((t: any) => typeof t === "object" && t.guid === activeDragData.typeGuid) as any;
      return type?.name || "Type";
    } else if (activeDragData.type === "design") {
      const kitId = activeDragData.designGuid?.split("-")[0];
      const kit = kitShallows.find((k) => k.guid.startsWith(kitId));
      const design = kit?.designs?.find((d: any) => typeof d === "object" && d.guid === activeDragData.designGuid) as any;
      return design?.name || "Design";
    }
    return null;
  }, [activeDragData, kitShallows]);

  return (
    <TutorialProvider store={tutorialStore}>
      <GlobalFooterItems />
      <DndContext
        sensors={sensors}
        collisionDetection={customCollisionDetection}
        onDragStart={(event) => {
          setActiveDragId(event.active.id as string);
          setActiveDragData(event.active.data.current);

          sketchpadCommands.setActiveInteraction("semio.sketchpad.drag", "dragging");
        }}
        onDragEnd={(event) => {
          setActiveDragId(null);
          setActiveDragData(null);

          sketchpadCommands.setActiveInteraction("semio.sketchpad.drag", undefined);

          const customEvent = new CustomEvent("design-drag-end", { detail: event });
          window.dispatchEvent(customEvent);
        }}
      >
        <LevelProvider level="base">
          <LayoutComponent
            className="bg-base text-foreground relative border"
            navbar={<Navbar items={navbarItems} />}
            footer={
              <Footer
                items={footerItems.map((item) => ({
                  id: item.id,
                  icon: item.icon,
                  text: item.text,
                  content: item.content,
                  order: item.order,
                  onClick: item.onClick,
                }))}
                isVisible={isFooterExpanded || !isFullscreen}
              />
            }
            leftPanel={
              panelVisibility.leftSidePanel || panelVisibility.workbench || panelVisibility.tools
                ? {
                    visible: true,
                    size: panelVisibility.tools ? panelSizes.toolsWidth : panelSizes.workbenchWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.tools ? "toolsWidth" : "workbenchWidth", size),
                    sections: panelVisibility.tools ? toolsSections : workbenchSections,
                    opacity: panelOpacity,
                    panelKey: panelVisibility.tools ? "tools" : "workbench",
                  }
                : undefined
            }
            middlePanel={
              panelVisibility.hudPanel || panelVisibility.hud || panelVisibility.stats
                ? {
                    visible: true,
                    size: panelVisibility.stats ? panelSizes.statsWidth : panelSizes.hudWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.stats ? "statsWidth" : "hudWidth", size),
                    sections: panelVisibility.stats ? statsSections : hudSections,
                    panelKey: panelVisibility.stats ? "stats" : "hud",
                  }
                : undefined
            }
            rightPanel={
              panelVisibility.rightSidePanel || panelVisibility.details || panelVisibility.chat || panelVisibility.settings
                ? {
                    visible: true,
                    size: panelVisibility.chat ? panelSizes.chatWidth : panelVisibility.settings ? panelSizes.settingsWidth : panelSizes.detailsWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.chat ? "chatWidth" : panelVisibility.settings ? "settingsWidth" : "detailsWidth", size),
                    sections: panelVisibility.chat ? chatSections : panelVisibility.settings ? settingsSections : detailsSections,
                    panelKey: panelVisibility.chat ? "chat" : panelVisibility.settings ? "settings" : "details",
                  }
                : undefined
            }
            bottomPanel={
              consoleSections.length > 0
                ? {
                    visible: true,
                    size: panelSizes.consoleHeight,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", "consoleHeight", size),
                    sections: consoleSections,
                    panelKey: "console",
                  }
                : undefined
            }
            leftSidePanel={
              leftSidePanelTabs.length > 0 && panelVisibility.leftSidePanel
                ? {
                    visible: true,
                    size: panelSizes.leftSidePanelWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", "leftSidePanelWidth", size),
                    tabs: leftSidePanelTabs,
                    activeTabId: activeLeftTabId,
                    onActiveTabChange: setActiveLeftTabId,
                  }
                : undefined
            }
            rightSidePanel={
              rightSidePanelTabs.length > 0 && panelVisibility.rightSidePanel
                ? {
                    visible: true,
                    size: panelSizes.rightSidePanelWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", "rightSidePanelWidth", size),
                    tabs: rightSidePanelTabs,
                    activeTabId: activeRightTabId,
                    onActiveTabChange: setActiveRightTabId,
                  }
                : undefined
            }
            hudPanel={
              hudPanelTabs.length > 0 && panelVisibility.hudPanel
                ? {
                    visible: true,
                    size: panelSizes.hudPanelWidth,
                    onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", "hudPanelWidth", size),
                    tabs: hudPanelTabs,
                    activeTabId: activeHudTabId,
                    onActiveTabChange: setActiveHudTabId,
                  }
                : undefined
            }
            toolbar={
              panelVisibility.toolbar || appType === "type" || appType === "design" || appType === "feedback" || appType === "kit" || appType === "home" ? (
                toolbarSections.length > 0 ? (
                  <div id="semio.sketchpad.toolbar" className="flex items-center justify-center pointer-events-auto">
                    <LevelProvider level="panel">
                      <div className="bg-panel flex items-center gap-single border p-single">
                        <ToolbarScopeWrapper>
                          {toolbarSections.map((section) => (
                            <div key={section.id}>{typeof section.content === "function" ? section.content() : section.content}</div>
                          ))}
                        </ToolbarScopeWrapper>
                      </div>
                    </LevelProvider>
                  </div>
                ) : (
                  <div id="semio.sketchpad.toolbar" className="hidden" />
                )
              ) : undefined
            }
            canvas={
              <div className="relative h-full w-full">
                <AppRouter />
              </div>
            }
          />
        </LevelProvider>
        <DragOverlay>
          {activeDragId && activeDragData ? (
            <div className="cursor-grabbing">
              <div className="border border-element rounded-full w-small h-small flex items-center justify-center shadow-lg">
                <span className="text-small font-medium select-none">{getTypeOrDesignName()?.substring(0, 2).toUpperCase() || "?"}</span>
              </div>
            </div>
          ) : null}
        </DragOverlay>
      </DndContext>
    </TutorialProvider>
  );
};

const SketchpadInteractionBridge: FC<{ children: React.ReactNode }> = ({ children }) => {
  const sketchpadCommands = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  const interactionCommands = useMemo(
    () => ({
      setActiveInteraction: (elementId?: string, interactionId?: string) => {
        sketchpadCommands.setActiveInteraction(elementId || "semio.interaction", interactionId);
      },
    }),
    [sketchpadCommands],
  );

  return (
    <InteractionProvider commands={interactionCommands} activeInteraction={activeInteraction}>
      {children}
    </InteractionProvider>
  );
};

const GlobalNavigationBridge: FC<{ children: React.ReactNode }> = ({ children }) => {
  const reactNavigate = useReactNavigate();

  useEffect(() => {
    if (typeof window !== "undefined") {
      (window as any).__SEMIO_NAVIGATE__ = reactNavigate;
    }
    return () => {
      if (typeof window !== "undefined") {
        delete (window as any).__SEMIO_NAVIGATE__;
      }
    };
  }, [reactNavigate]);

  return <>{children}</>;
};

const SketchpadContent: FC = () => {
  return <LayoutWrapper />;
};

const Sketchpad = ({
  id,
  remote,
  onWindowEvents,
  initialState,
  importKitUrls,
  embedded,
}: {
  id?: string;
  remote?: RemoteProviders;
  onWindowEvents?: WindowEvents;
  initialState?: ExtendedInitialState;
  importKitUrls?: string[];
  embedded?: boolean;
}): JSX.Element => {
  const initialEntries = useMemo(() => {
    if (!embedded) return undefined;
    if (typeof window !== "undefined" && window.location) {
      return [window.location.pathname + window.location.search];
    }
    return ["/"];
  }, [embedded]);

  const routerContent = (
    <GlobalNavigationBridge>
      <SketchpadScopeProvider id={id} remote={remote} onWindowEvents={onWindowEvents} initialState={initialState} importKitUrls={importKitUrls}>
        <SketchpadInteractionBridge>
          <OriginProvider>
            <FocusProvider>
              <PanelSectionProvider>
                <SidePanelTabProvider>
                  <FooterItemProvider>
                    <DragDropProvider>
                      <SketchpadContent />
                    </DragDropProvider>
                  </FooterItemProvider>
                </SidePanelTabProvider>
              </PanelSectionProvider>
            </FocusProvider>
          </OriginProvider>
        </SketchpadInteractionBridge>
      </SketchpadScopeProvider>
    </GlobalNavigationBridge>
  );

  if (embedded) {
    return <MemoryRouter initialEntries={initialEntries}>{routerContent}</MemoryRouter>;
  }

  return <BrowserRouter>{routerContent}</BrowserRouter>;
};

// #endregion 🔖Sketchpad Components

export { SectionSpecificity, Window } from "./elements";

export { Sketchpad };
export default Sketchpad;

// #endregion 🔖Apps
