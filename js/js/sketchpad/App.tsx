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

import { AwardIcon, DocumentIcon, FocusIcon, HomeIcon, LayoutIcon, LocalKitIcon, Maximize2Icon, Minimize2Icon, NavigateBackIcon, NavigateForwardIcon, NavigateUpIcon, RemoteKitIcon, SearchIcon, TemporaryKitIcon, TutorialIcon, TypeIcon, UserIcon } from "@semio/assets";
import { ReactFlowProvider } from "@xyflow/react";
import Fuse, { FuseResult } from "fuse.js";
import JSZip from "jszip";
import React, { ComponentType, createContext, FC, Fragment, ReactNode, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { createPortal } from "react-dom";
import { createRoot } from "react-dom/client";
import { useHotkeys as useReactHotkeys } from "react-hotkeys-hook";
import { useTranslation as useI18nTranslation } from "react-i18next";
import { BrowserRouter, MemoryRouter, Outlet, Route, Routes, useLocation, useParams, useNavigate as useReactNavigate, useSearchParams } from "react-router";
import initSqlJs, { Database, SqlJsStatic } from "sql.js";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { useHotkey, useLabel } from "../i18n";
import {
  applyDesignDiff,
  applyKitDiff,
  Attribute,
  Author,
  AuthorDiff,
  Benchmark,
  BenchmarkDiff,
  Camera,
  CameraDiff,
  colorPortsForTypes,
  Connection,
  ConnectionDiff,
  Coord,
  CoordDiff,
  Design,
  DesignDiff,
  DesignShallow,
  DiffStatus,
  FileDiff,
  findDesignInKit,
  findPieceInDesign,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  flattenDesign,
  Folder,
  FolderDiff,
  generateUniqueName,
  getClusterableGroups,
  getIncludedDesigns,
  getPieceRepresentationUrls,
  Group,
  GroupDiff,
  guid,
  Guid,
  inverseKitDiff,
  Kit,
  KitDiff,
  KitShallow,
  Layer,
  LayerDiff,
  Location,
  LocationDiff,
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
  Representation,
  RepresentationDiff,
  File as SemioFile,
  Side,
  SideDiff,
  Stat,
  StatDiff,
  Type,
  TypeDiff,
  TypeShallow,
  Vec,
  VecDiff,
  Vector,
  VectorDiff,
} from "../semio";
import type { DesignAppState } from "./apps/design/App";
import type { KitAppState } from "./apps/kit/App";
import type { QualityAppState } from "./apps/quality/App";
import type { TypeAppState } from "./apps/type/App";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
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
  Navbar,
  NavbarItem,
  Toggle,
  Window,
} from "./elements";
import {
  AppCommandResult,
  AppConfig,
  AppDiff,
  AppEdit,
  AppKind,
  AppRegistration,
  AppStep,
  CompleteState,
  CompositeFileProviderConfig,
  DesignAppId,
  Disposable,
  EnrichedPanelDefinition,
  enrichPanelDefinition,
  Expertise,
  ExtendedInitialState,
  FileProvider,
  FileProviderFactory,
  FocusItem,
  FooterItem,
  KitAppId,
  KitCommandContext,
  KitCommandResult,
  KitDiffAppCommandResult,
  KitDiffAppEdit,
  KitDiffAppStep,
  Layout,
  LocalFileProviderConfig,
  MemoryFileProviderConfig,
  Mode,
  PanelConfig,
  PanelDefinition,
  PanelKey,
  PanelKind,
  panelKindConfigs,
  PanelSection,
  PanelSections,
  PanelSizes,
  PanelVisibility,
  QualityAppId,
  RemoteFileProviderConfig,
  RemoteProviders,
  RouteSegment,
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
  Transact,
  TypeAppId,
  Unsubscribe,
  Url,
  WindowEvents,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "./sketchpad";
import { Tutorial, TutorialProvider, TutorialStore, useAvailableTutorials } from "./tutorials";
// Lazy imports to break circular dependency - hooks are imported inside functions that use them
// Module cache for lazy-loaded hooks
let designAppModuleCache: any = null;
let homeAppModuleCache: any = null;
let kitAppModuleCache: any = null;
let qualityAppModuleCache: any = null;
let typeAppModuleCache: any = null;

const getDesignAppHooks = () => {
  return {
    useDesignAppCommands: (id?: { kit: string; design: string }) => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppCommands(id);
      }
      return {
        togglePanel: () => {},
        execute: () => Promise.resolve({}),
      };
    },
    useDesignAppDiff: () => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppDiff();
      }
      return {};
    },
    useDesignAppHover: () => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppHover();
      }
      return undefined;
    },
    useDesignAppIsPieceTransitiveHovered: (id?: DesignAppId, pieceId?: string) => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppIsPieceTransitiveHovered(id, pieceId);
      }
      return false;
    },
    useDesignAppSelection: () => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppSelection();
      }
      return {};
    },
    useDesignAppStore: <T,>(selector?: (store: any) => T, id?: DesignAppId) => {
      if (designAppModuleCache) {
        return designAppModuleCache.useDesignAppStore(selector, id);
      }
      return null;
    },
  };
};

const getHomeAppHooks = () => {
  return {
    useHomeCommands: () => {
      if (homeAppModuleCache) {
        return homeAppModuleCache.useHomeCommands();
      }
      return {
        togglePanel: () => {},
        selectKit: () => {},
        addKitToSelection: () => {},
        removeKitFromSelection: () => {},
        selectKits: () => {},
        deselectAll: () => {},
        setSortColumn: () => {},
        setSortDirection: () => {},
        toggleSort: () => {},
        execute: () => Promise.resolve({}),
      };
    },
  };
};

export const getKitAppHooks = () => {
  return {
    useKitAppCommands: (id?: { kit: string }) => {
      if (kitAppModuleCache) {
        return kitAppModuleCache.useKitAppCommands(id);
      }
      return {
        togglePanel: () => {},
        execute: () => Promise.resolve({}),
      };
    },
  };
};

const getQualityAppHooks = () => {
  return {
    useQualityAppCommands: (id?: { kit: string; quality: string }) => {
      if (qualityAppModuleCache) {
        return qualityAppModuleCache.useQualityAppCommands(id);
      }
      return {
        togglePanel: () => {},
        execute: () => Promise.resolve({}),
      };
    },
  };
};

const getTypeAppHooks = () => {
  return {
    useTypeAppCommands: (id?: { kit: string; type: string }) => {
      if (typeAppModuleCache) {
        return typeAppModuleCache.useTypeAppCommands(id);
      }
      return {
        togglePanel: () => {},
        execute: () => Promise.resolve({}),
      };
    },
  };
};

// #endregion Imports

// #region Store

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

// #region Lazy Imports

let docsRegistryCache: any = null;
let docsRegistryPromise: Promise<any> | null = null;

// Fallback registry that returns empty arrays until the real one loads
const fallbackDocsRegistry = {
  getAllSections: () => [],
  getAllPages: () => [],
};

function getDocsRegistry() {
  // Return fallback if not loaded yet
  if (!docsRegistryCache) {
    return fallbackDocsRegistry;
  }
  return docsRegistryCache;
}

function preloadDocsRegistry(): Promise<void> {
  if (docsRegistryCache) {
    return Promise.resolve();
  }
  if (!docsRegistryPromise) {
    docsRegistryPromise = import("./apps/docs/App").then((module) => {
      docsRegistryCache = module.docsRegistry;
      return docsRegistryCache;
    });
  }
  return docsRegistryPromise;
}

// Preload after AppStore is defined
if (typeof window !== "undefined") {
  preloadDocsRegistry().catch((err) => {
    console.error("Failed to preload docsRegistry:", err);
  });
}

// #endregion Lazy Imports

// #region File Provider

// #region Memory File Provider

/**
 * Creates an in-memory file provider.
 * Files are stored in memory using Map and will be lost on page reload.
 * Used for temporary kits.
 */
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
        console.log(`[MEMORY] Uploaded file ${path} (${blob.size} bytes)`);
        return `memory://${key}`;
      },

      download: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        const blob = storage.get(key);

        if (!blob) {
          throw new Error(`File not found in memory: ${key}`);
        }

        console.log(`[MEMORY] Downloaded file ${path} (${blob.size} bytes)`);
        return blob;
      },

      delete: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);
        storage.delete(key);
        console.log(`[MEMORY] Deleted file ${path}`);
      },

      getUrl: (kitId, fileId, path) => {
        return `memory://${getKey(kitId, fileId, path)}`;
      },
    };
  };
}

// #endregion Memory File Provider

// #region Local File Provider (IndexedDB)

/**
 * Creates a local file provider using IndexedDB.
 * Files are persisted locally in the browser.
 * Used for local kits.
 */
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
            console.log(`[LOCAL] Uploaded file ${path} (${blob.size} bytes)`);
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
              console.log(`[LOCAL] Downloaded file ${path} (${blob.size} bytes)`);
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
            console.log(`[LOCAL] Deleted file ${path}`);
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

// #endregion Local File Provider

// #region Remote File Provider

/**
 * Creates a remote file provider using HTTP/REST API.
 * Files are synchronized with a remote server.
 * Used for remote kits.
 */
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
        console.log(`[REMOTE] Uploaded file ${path} (${blob.size} bytes)`);
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
        console.log(`[REMOTE] Downloaded file ${path} (${blob.size} bytes)`);
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

        console.log(`[REMOTE] Deleted file ${path}`);
      },

      getUrl: (kitId, fileId, path) => {
        return getUrl(kitId, fileId, path);
      },
    };
  };
}

// #endregion Remote File Provider

// #region Composite File Provider

/**
 * Creates a composite file provider that combines memory, local, and remote storage.
 * This is the recommended way to create file providers.
 *
 * Behavior:
 * - memory only: Files in memory, lost on reload (temporary kits)
 * - memory + local: Files persisted locally (local kits)
 * - memory + local + remote: Files synced to remote, persisted locally (remote kits)
 *
 * @example
 * // Temporary kit
 * createCompositeFileProvider({ memory: true })
 *
 * // Local kit
 * createCompositeFileProvider({ memory: true, local: true })
 *
 * // Remote kit
 * createCompositeFileProvider({
 *   memory: true,
 *   local: true,
 *   remote: { baseUrl: 'https://api.example.com' }
 * })
 */
export function createCompositeFileProvider(config: CompositeFileProviderConfig): FileProviderFactory {
  return async (kitId: string): Promise<FileProvider> => {
    const providers: FileProvider[] = [];

    // Initialize providers in order: memory, local, remote
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

    // Composite provider that writes to all providers and reads from the first available
    return {
      upload: async (kitId, fileId, path, blob) => {
        // Write to all providers in parallel
        const results = await Promise.allSettled(providers.map((p) => p.upload(kitId, fileId, path, blob)));

        // Log any errors but don't fail if at least one succeeds
        const successful = results.filter((r) => r.status === "fulfilled");
        if (successful.length === 0) {
          throw new Error(`All providers failed to upload file ${path}`);
        }

        // Return the last successful URL (remote if available, otherwise local/memory)
        const lastSuccessful = results.reverse().find((r) => r.status === "fulfilled") as PromiseFulfilledResult<string>;
        return lastSuccessful.value;
      },

      download: async (kitId, fileId, path) => {
        // Try providers in order: memory > local > remote
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
        // Delete from all providers in parallel
        await Promise.allSettled(providers.map((p) => p.delete(kitId, fileId, path)));
      },

      getUrl: (kitId, fileId, path) => {
        // Return URL from the last provider (remote if available)
        return providers[providers.length - 1].getUrl(kitId, fileId, path);
      },
    };
  };
}

// #endregion Composite File Provider

// #endregion File Provider

// #region Kits

// Note: useConnectionColor, useDiffedDesign, usePieceWithDiff, and usePortColoredTypes
// have been moved to designHelpers.ts and are NOT re-exported here to avoid circular dependencies.
// Internal use in this file imports directly from "./designHelpers".

type YAttributeVal = string;
type YAttribute = Y.Map<YAttributeVal>;

class AttributeStore {
  private yAttribute: YAttribute;
  private cache?: Attribute;
  private cacheHash?: string;

  constructor(yAttribute: YAttribute, attribute: Attribute) {
    this.yAttribute = yAttribute;
    this.guid = attribute.guid;
    this.key = attribute.key;
    this.value = attribute.value;
    this.definition = attribute.definition;
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

// #endregion Attribute

// #region Coord

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

// #endregion Coord

// #region Vec

type YVecVal = number;
type YVec = Y.Map<YVecVal>;

class YVecStore {
  private yVec: YVec;
  private cache?: Vec;
  private cacheHash?: string;

  constructor(yVec: YVec, vec: Vec) {
    this.yVec = yVec;
    this.x = vec.x;
    this.y = vec.y;
  }

  get x(): number {
    return this.yVec.get("x") as number;
  }
  set x(x: number) {
    this.yVec.set("x", x);
  }

  get y(): number {
    return this.yVec.get("y") as number;
  }
  set y(y: number) {
    this.yVec.set("y", y);
  }

  hash = (vec: Vec): string => {
    return JSON.stringify(vec);
  };

  snapshot = (): Vec => {
    const currentData = {
      x: this.x,
      y: this.y,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: VecDiff) => {
    if (diff.x !== undefined) this.x = diff.x;
    if (diff.y !== undefined) this.y = diff.y;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yVec, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yVec, subscribe, true);
  };
}

// #endregion Vec

// #region Point

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

// #endregion Point

// #region Vector

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

// #endregion Vector

// #region Plane

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

// #endregion Plane

// #region Camera

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

// #endregion Camera

// #region Location

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

// #endregion Location

// #region Author

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
  if (!kitStore) return null;
  const authorScope = useAuthorScope();
  const authorGuid = authorScope?.guid ?? guid;
  if (!authorGuid) throw new Error("useAuthorStore must be called within a AuthorScopeProvider or be directly provided with a guid");
  if (!kitStore.hasAuthor(authorGuid)) throw new Error(`Author store not found for author ${authorGuid}`);
  const authorStore = kitStore.author(authorGuid);
  return selector ? selector(authorStore) : authorStore;
}

export function useAuthor<T>(selector?: (author: Author) => T, id?: Guid, deep: boolean = false): T | Author | null {
  const store = useAuthorStore(identitySelector, id);
  if (!store) return null;
  return useSync<Author, T>(store as AuthorStore, selector ? selector : (identitySelector as any));
}

// #endregion Author

// #region File

type YFile = Y.Map<string | number | YAttributes>;
type YFiles = Y.Array<YFile>;

class FileStore {
  private yFile: YFile;
  private cache?: SemioFile;
  private cacheHash?: string;

  constructor(yFile: YFile, file: SemioFile) {
    this.yFile = yFile;

    this.guid = file.guid;
    this.name = file.name;
    this.folder = file.folder;
    this.remote = file.remote;
    this.size = file.size;
    this.fileHash = file.hash;
    this.createdAt = file.createdAt;
    this.updatedAt = file.updatedAt;
    this.createdBy = file.createdBy;
    this.updatedBy = file.updatedBy;
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
      folder: this.folder,
      remote: this.remote,
      size: this.size,
      hash: this.fileHash,
      createdAt: this.createdAt,
      updatedAt: this.updatedAt,
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
    if (diff.folder !== undefined) this.folder = diff.folder;
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

// #endregion File

// #region Folder

type YFolder = Y.Map<string | YAttributes>;
type YFolders = Y.Array<YFolder>;

class FolderStore {
  yFolder: YFolder;
  private cache?: Folder;
  private cacheHash?: string;

  constructor(yFolder: YFolder, folder: Folder) {
    this.yFolder = yFolder;
    this.guid = folder.guid;
    this.name = folder.name;
    this.parent = folder.parent;
    this.description = folder.description;
    this.createdAt = folder.createdAt;
    this.updatedAt = folder.updatedAt;
    this.createdBy = folder.createdBy;
    this.updatedBy = folder.updatedBy;
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
  set createdAt(createdAt: Date | undefined) {
    this.yFolder.set("createdAt", createdAt?.toISOString() || "");
  }

  get updatedAt(): Date | undefined {
    const date = this.yFolder.get("updatedAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set updatedAt(updatedAt: Date | undefined) {
    this.yFolder.set("updatedAt", updatedAt?.toISOString() || "");
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
      parent: this.parent,
      description: this.description,
      createdAt: this.createdAt,
      updatedAt: this.updatedAt,
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
    if (diff.parent !== undefined) this.parent = diff.parent;
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

// #endregion Folder

// #region Benchmark

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

// #endregion Benchmark

// #region Quality

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

// #endregion Quality

// #region Prop

type YProp = Y.Map<string | number | boolean | YAttributes>;
type YProps = Y.Array<YProp>;

class PropStore {
  private yProp: YProp;
  private cache?: Prop;
  private cacheHash?: string;

  constructor(yProp: YProp, prop: Prop) {
    this.yProp = yProp;
    this.guid = prop.guid;
    this.key = prop.key;
    this.value = prop.value;
    this.unit = prop.unit;
  }

  get guid(): string {
    return this.yProp.get("guid") as string;
  }
  set guid(guid: string) {
    this.yProp.set("guid", guid);
  }

  get key(): string {
    return this.yProp.get("key") as string;
  }
  set key(key: string) {
    this.yProp.set("key", key);
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
      key: this.key,
      value: this.value || "",
      unit: this.unit,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const prop: Prop = {
      guid: this.guid,
      key: this.key,
      value: this.value || "",
      unit: this.unit,
    };

    this.cache = prop;
    this.cacheHash = currentHash;
    return prop;
  }

  change = (diff: PropDiff) => {
    if (diff.key !== undefined) this.key = diff.key;
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

// #endregion Prop

// #region Representation

type YRepresentationVal = string | Y.Array<string> | YAttributes;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentations = Y.Array<YRepresentation>;

class RepresentationStore {
  private yRepresentation: YRepresentation;
  private yTags: Y.Array<string>;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private cache?: Representation;
  private cacheHash?: string;

  constructor(yRepresentation: YRepresentation, representation: Representation) {
    this.yRepresentation = yRepresentation;
    this.guid = representation.guid;
    this.file = representation.file;
    this.description = representation.description;
    this.yTags = this.yRepresentation.set("tags", new Y.Array<string>());
    if (representation.tags) this.yTags.push(representation.tags);
    this.attributes = new Map();
    this.yAttributes = this.yRepresentation.set("attributes", new Y.Array<YAttribute>());
    if (representation.attributes) {
      for (const attribute of representation.attributes) {
        const yAttribute = new Y.Map<YAttributeVal>();
        this.yAttributes.push([yAttribute]);
        const attributeStore = new AttributeStore(yAttribute, attribute);
        this.attributes.set(attribute.guid, attributeStore);
      }
    }
  }

  get guid(): string {
    return this.yRepresentation.get("guid") as string;
  }
  set guid(guid: string) {
    this.yRepresentation.set("guid", guid);
  }

  get file(): string {
    return this.yRepresentation.get("file") as string;
  }
  set file(file: string) {
    this.yRepresentation.set("file", file);
  }

  get description(): string | undefined {
    return this.yRepresentation.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yRepresentation.set("description", description || "");
  }

  hash = (representation: Representation): string => {
    return JSON.stringify(representation);
  };

  snapshot(): Representation {
    const tags = this.yTags.toArray();
    const currentHash = this.hash({
      guid: this.guid,
      file: this.file,
      description: this.description,
      tags,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const representation: Representation = {
      guid: this.guid,
      file: this.file,
      description: this.description,
      tags,
    };

    this.cache = representation;
    this.cacheHash = currentHash;
    return representation;
  }

  apply(diff: RepresentationDiff): void {
    if (diff.file !== undefined) this.file = diff.file;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.tags !== undefined) {
      this.yTags.delete(0, this.yTags.length);
      if (diff.tags.length > 0) {
        this.yTags.push(diff.tags);
      }
    }
  }

  change = (diff: RepresentationDiff) => {
    this.apply(diff);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yRepresentation, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yRepresentation, subscribe, true);
  };
}

// #endregion Representation

// #region Port

type YPortVal = string | number | boolean | YAttributes | Y.Array<string> | YPoint | YVector | YProps;
type YPort = Y.Map<YPortVal>;
type YPorts = Y.Array<YPort>;

class PortStore {
  private yPort: YPort;
  private yPoint: YPoint;
  private point: YPointStore;
  private yDirection: YVector;
  private direction: YVectorStore;
  private cache?: Port;
  private cacheHash?: string;

  constructor(yPort: YPort, port: Port) {
    this.yPort = yPort;
    this.guid = port.guid;
    this.localId = port.guid;
    this.description = port.description;
    this.family = port.family;
    this.mandatory = port.mandatory;
    this.t = port.t;

    this.yPoint = new Y.Map();
    this.yPort.set("point", this.yPoint);
    this.point = new YPointStore(this.yPoint, port.point);

    this.yDirection = new Y.Map();
    this.yPort.set("direction", this.yDirection);
    this.direction = new YVectorStore(this.yDirection, port.direction);
  }

  get guid(): string {
    return this.yPort.get("guid") as string;
  }
  set guid(guid: string) {
    this.yPort.set("guid", guid);
  }

  get localId(): string | undefined {
    return this.yPort.get("id_") as string | undefined;
  }
  set localId(id_: string | undefined) {
    this.yPort.set("id_", id_ || "");
  }

  get description(): string | undefined {
    return this.yPort.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yPort.set("description", description || "");
  }

  get family(): string | undefined {
    return this.yPort.get("family") as string | undefined;
  }
  set family(family: string | undefined) {
    this.yPort.set("family", family || "");
  }

  get mandatory(): boolean | undefined {
    return this.yPort.get("mandatory") as boolean | undefined;
  }
  set mandatory(mandatory: boolean | undefined) {
    if (mandatory !== undefined) this.yPort.set("mandatory", mandatory);
  }

  get t(): number {
    return this.yPort.get("t") as number;
  }
  set t(t: number) {
    this.yPort.set("t", t);
  }

  hash = (port: Port): string => {
    return JSON.stringify(port);
  };

  snapshot = (): Port => {
    const currentData = {
      guid: this.guid,
      id_: this.localId,
      description: this.description,
      family: this.family,
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

  apply(diff: PortDiff): void {
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.family !== undefined) this.family = diff.family;
    if (diff.mandatory !== undefined) this.mandatory = diff.mandatory;
    if (diff.t !== undefined) this.t = diff.t;
  }

  change = (diff: PortDiff) => {
    this.apply(diff);
    if (diff.point !== undefined) this.point.change(diff.point);
    if (diff.direction !== undefined) this.direction.change(diff.direction);
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPort, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPort, subscribe, true);
  };
}

// #endregion Port

// #region Type

type YTypeVal = string | number | boolean | YAuthorUuids | YAttributes | YRepresentations | YPorts | YProps | YLocation;
type YType = Y.Map<YTypeVal>;
type YTypes = Y.Array<YType>;

export class TypeStore {
  public readonly parent: KitStore;
  private yType: YType;
  private yAttributes: YAttributes;
  private attributes: Map<string, AttributeStore>;
  private yAuthors: YAuthorUuids;
  private authors: Map<string, AuthorStore>;
  private yRepresentations: YRepresentations;
  private yPorts: YPorts;
  public representations: Map<string, RepresentationStore>;
  public ports: Map<string, PortStore>;
  private cache?: Type;
  private cacheHash?: string;

  constructor(parent: KitStore, yType: YType, type: Type) {
    this.parent = parent;
    this.yType = yType;
    this.representations = new Map();
    this.ports = new Map();

    this.guid = type.guid;
    this.name = type.name;
    this.parentGuid = type.parent;
    this.abstract = type.isAbstract;
    this.stock = type.stock;
    this.virtual = type.virtual;
    this.unit = type.unit;
    this.icon = type.icon;
    this.image = type.image;
    this.description = type.description;

    this.attributes = new Map();
    this.yAttributes = this.yType.set("attributes", new Y.Array<YAttribute>());
    // if (type.attributes) {
    //   for (const attribute of type.attributes) {
    //     this.createAttribute(attribute);
    //   }
    // }
    if (type.attributes) {
      type.attributes.forEach((attribute) => this.createAttribute(attribute));
    }

    this.authors = new Map();
    this.yAuthors = this.yType.set("authors", new Y.Array<YAuthorUuid>());
    if (type.authors) {
      for (const author of type.authors) {
        const authorStore = this.parent.author(author);
        this.authors.set(authorStore.guid, authorStore);
        this.yAuthors.push([authorStore.guid]);
      }
    }

    this.yRepresentations = this.yType.set("representations", new Y.Array<YRepresentation>());
    // if (type.representations) {
    //   for (const representation of type.representations) {
    //     this.createRepresentation(representation);
    //   }
    // }
    if (type.representations) {
      type.representations.forEach((representation) => this.createRepresentation(representation));
    }

    this.yPorts = this.yType.set("ports", new Y.Array<YPort>());
    if (type.ports) {
      for (const port of type.ports) {
        this.createPort(port);
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

  createRepresentation(representation: Representation): void {
    const yRepresentation = new Y.Map<YRepresentationVal>();
    this.yRepresentations.push([yRepresentation]);
    const yRepresentationStore = new RepresentationStore(yRepresentation, representation);
    this.representations.set(representation.guid, yRepresentationStore);
  }

  hasRepresentation(guid: string): boolean {
    return this.representations.has(guid);
  }

  representation(guid: string): RepresentationStore {
    const rep = this.representations.get(guid);
    if (!rep) throw new Error(`Representation store not found for guid ${guid}`);
    return rep;
  }

  hasPort(guid: string): boolean {
    return this.ports.has(guid);
  }

  createPort(port: Port): void {
    if (this.hasPort(port.guid)) throw new Error(`Port (${port.guid}) already exists.`);
    const yPort = new Y.Map<YPortVal>();
    this.yPorts.push([yPort]);
    const yPortStore = new PortStore(yPort, port);
    this.ports.set(port.guid, yPortStore);
  }

  port(guid: string): PortStore {
    const p = this.ports.get(guid);
    if (!p) throw new Error(`Port store not found for guid ${guid}`);
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
      parent: this.parentGuid,
      folder: this.folder,
      isAbstract: this.abstract,
      stock: this.stock,
      virtual: this.virtual,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
      authors: Array.from(this.authors.values()).map((a) => a.guid),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
      representations: Array.from(this.representations.values()).map((rep) => rep.snapshot()),
      ports: Array.from(this.ports.values()).map((port) => port.snapshot()),
      createdAt: this.createdAt,
      updatedAt: this.updatedAt,
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
        if (diff.parent) this.yType.set("parent", diff.parent);
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
      if (diff.icon !== undefined) this.yType.set("icon", diff.icon);
      if (diff.image !== undefined) this.yType.set("image", diff.image);
      if (diff.description !== undefined) this.yType.set("description", diff.description);
      if (diff.createdAt !== undefined) this.yType.set("createdAt", diff.createdAt.toISOString());
      if (diff.updatedAt !== undefined) this.yType.set("updatedAt", diff.updatedAt.toISOString());

      if (diff.authors !== undefined) {
        this.yAuthors.delete(0, this.yAuthors.length);
        this.authors = new Map(
          diff.authors.map((authorGuid) => {
            const author = this.parent.author(authorGuid);
            return [author.guid, author];
          }),
        );
        this.authors.forEach((author) => this.yAuthors.push([author.guid]));
      }

      if (diff.representations) {
        if (diff.representations.removed) {
          diff.representations.removed.forEach((guid) => {
            const index = Array.from(this.representations.keys()).indexOf(guid);
            if (index !== -1) {
              this.yRepresentations.delete(index, 1);
              this.representations.delete(guid);
            }
          });
        }
        if (diff.representations.added) {
          diff.representations.added.forEach((representation) => {
            this.createRepresentation(representation);
          });
        }
        if (diff.representations.updated) {
          diff.representations.updated.forEach(({ id, diff: repDiff }) => {
            const rep = this.representations.get(id);
            if (rep) rep.apply(repDiff);
          });
        }
      }

      if (diff.ports) {
        if (diff.ports.removed) {
          diff.ports.removed.forEach((guid) => {
            const index = Array.from(this.ports.keys()).indexOf(guid);
            if (index !== -1) {
              this.yPorts.delete(index, 1);
              this.ports.delete(guid);
            }
          });
        }
        if (diff.ports.added) {
          diff.ports.added.forEach((port) => {
            this.createPort(port);
          });
        }
        if (diff.ports.updated) {
          diff.ports.updated.forEach(({ id, diff: portDiff }) => {
            const port = this.ports.get(id);
            if (port) port.change(portDiff);
          });
        }
      }

      if (diff.attributes) {
        if (diff.attributes.removed) {
          diff.attributes.removed.forEach((identifier) => {
            const attribute = this.findAttributeStore(identifier);
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
          diff.attributes.updated.forEach(({ id, diff: attributeDiff }) => {
            const attribute = this.findAttributeStore(id);
            if (!attribute) return;
            attribute.change(attributeDiff);
          });
        }
      }

      // TODO: Handle location, props diffs

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
  if (!kitStore) return null;
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? guid;
  if (!typeGuid) return null;
  if (!kitStore.hasType(typeGuid)) return null;
  const typeStore = kitStore.type(typeGuid);
  if (!typeStore) return null;
  return selector ? selector(typeStore) : typeStore;
}

export function useType<T>(selector?: (type: Type) => T, id?: Guid, deep: boolean = false): T | Type | null {
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? id;
  if (!typeGuid) {
    return null;
  }
  const store = useTypeStore(identitySelector, typeGuid);
  if (!store) return null;
  return useSync<Type, T>(store as TypeStore, selector ? selector : (identitySelector as any));
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
  if (!kitStore) return null;
  const qualityScope = useQualityScope();
  const qualityGuid = qualityScope?.guid ?? guid;
  if (!qualityGuid) return null;
  if (!kitStore.hasQuality(qualityGuid)) return null;
  const qualityStore = kitStore.quality(qualityGuid);
  if (!qualityStore) return null;
  return selector ? selector(qualityStore) : qualityStore;
}

export function useQuality<T>(selector?: (quality: Quality) => T, id?: Guid, deep: boolean = false): T | Quality | null {
  const store = useQualityStore(identitySelector, id);
  if (!store) return null;
  return useSync<Quality, T>(store as QualityStore, selector ? selector : (identitySelector as any));
}

// #endregion Type

// #region Layer

type YLayer = Y.Map<string | boolean | YAttributes>;
type YLayers = Y.Array<YLayer>;

class LayerStore {
  private yLayer: YLayer;
  private cache?: Layer;
  private cacheHash?: string;

  constructor(yLayer: YLayer, layer: Layer) {
    this.yLayer = yLayer;
    this.path = layer.path;
    this.isHidden = layer.isHidden;
    this.isLocked = layer.isLocked;
    this.color = layer.color;
    this.description = layer.description;
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

// #endregion Layer

// #region Piece

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

  constructor(parent: DesignStore, yPiece: YPiece, piece: Piece) {
    this.parent = parent;
    this.yPiece = yPiece;
    this.guid = piece.guid;
    this.attributes = new Map();

    this.localId = piece.guid;
    if (piece.type) {
      const type = this.parent.parent.type(piece.type);
      if (type) this.yPiece.set("type", type.guid);
    } else {
      const design = this.parent.parent.design(piece.design!);
      this.yPiece.set("design", design.guid);
    }
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

    this.yAttributes = this.yPiece.set("attributes", new Y.Array<YAttribute>());
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
    const currentData = {
      guid: this.guid,
      id_: this.localId,
      type: this.type,
      design: this.design,
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
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: PieceDiff) => {
    if (diff.guid !== undefined) this.guid = diff.guid;
    if (diff.type !== undefined) this.type = diff.type;
    if (diff.design !== undefined) this.design = diff.design;
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
      // Clear existing attributes
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

// useIsPieceSelected, useIsPieceHovered, useIsPieceTransitiveHovered - moved to designAppIntegration.ts

export function usePiecePlane(): Plane {
  const plane = usePiece((p) => p.plane) as Plane | undefined;

  if (!plane) {
    // Return default flat piece plane (XY plane at origin)
    return {
      origin: { x: 0, y: 0, z: 0 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    };
  }

  return plane;
}

export function useFlatPiece<T>(selector?: (piece: Piece) => T, id?: Guid): T | Piece | null {
  const pieceScope = usePieceScope();
  const pieceGuid = (typeof id === "string" ? id : typeof pieceScope === "string" ? pieceScope : null) as string | null;
  const metadata = usePiecesMetadata();
  const piece = usePiece(identitySelector, pieceGuid || undefined) as Piece | null;

  if (!piece || !pieceGuid) return null;

  const meta = metadata.get(pieceGuid);
  if (!meta) return piece;

  const flatPiece: Piece = {
    ...piece,
    plane: meta.plane,
    center: meta.center,
  };

  return selector ? selector(flatPiece) : flatPiece;
}

export function useFlatPiecePlane(id?: Guid): Plane {
  const plane = useFlatPiece((p) => p.plane, id) as Plane | undefined;

  if (!plane) {
    return {
      origin: { x: 0, y: 0, z: 0 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    };
  }

  return plane;
}

export function useFlatPieceCenter(id?: Guid): Coord {
  const center = useFlatPiece((p) => p.center, id) as Coord | undefined;

  if (!center) {
    return { u: 0, v: 0 };
  }

  return center;
}

export function useIsConnectedPiece(id?: Guid): boolean {
  const pieceScope = usePieceScope();
  const pieceGuid = (typeof id === "string" ? id : typeof pieceScope === "string" ? pieceScope : null) as string | null;
  const metadata = usePiecesMetadata();

  if (!pieceGuid) return false;

  const meta = metadata.get(pieceGuid);
  return meta ? meta.parentPieceId !== null : false;
}

export function usePieceParentConnection(id?: Guid): Connection | null {
  const pieceScope = usePieceScope();
  const pieceGuid = (typeof id === "string" ? id : typeof pieceScope === "string" ? pieceScope : null) as string | null;
  const design = useDesign() as Design;

  if (!pieceGuid || !design.connections) return null;

  return design.connections.find((c: Connection) => c.connecting.piece === pieceGuid || c.connected.piece === pieceGuid) ?? null;
}

// usePieceStatus and useDiffedPiece - moved to designAppIntegration.ts

// #endregion Piece

// #region Group

type YGroupVal = string | Y.Array<string> | YAttributes;
type YGroup = Y.Map<YGroupVal>;
type YGroups = Y.Array<YGroup>;

class GroupStore {
  private yGroup: YGroup;
  private cache?: Group;
  private cacheHash?: string;

  constructor(yGroup: YGroup, group: Group) {
    this.yGroup = yGroup;
    this.color = group.color;
    this.name = group.name;
    this.description = group.description;

    if (group.pieces) {
      const yPieces = new Y.Array<string>();
      yPieces.insert(0, group.pieces);
      this.yGroup.set("pieces", yPieces);
    }
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
      pieces: this.pieces,
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
    if (diff.pieces !== undefined) this.pieces = diff.pieces;
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

// #endregion Group

// #region Side

class SideStore {
  public readonly parent: DesignStore;
  private ySide: YSide;
  private cache?: Side;
  private cacheHash?: string;

  constructor(parent: DesignStore, ySide: YSide, side: Side) {
    this.parent = parent;
    this.ySide = ySide;
    this.guid = side.guid;

    // Store piece UUID
    const pieceStore = this.parent.piece(side.piece);
    if (pieceStore) {
      this.ySide.set("piece", pieceStore.guid);
    }

    // Store designPiece UUID if present
    if (side.designPiece) {
      const designPieceStore = this.parent.piece(side.designPiece);
      if (designPieceStore) {
        this.ySide.set("designPiece", designPieceStore.guid);
      }
    }

    // Store port UUID - need to find it through the piece's type
    if (pieceStore) {
      const typeGuid = pieceStore.type;
      if (typeGuid) {
        const typeStore = this.parent.parent.type(typeGuid);
        if (typeStore) {
          const portStore = typeStore.ports.get(side.port);
          if (portStore) {
            this.ySide.set("port", portStore.guid);
          }
        }
      }
    }
  }

  get guid(): string {
    return this.ySide.get("guid") as string;
  }
  set guid(guid: string) {
    this.ySide.set("guid", guid);
  }

  get piece(): Guid {
    const pieceUuid = this.ySide.get("piece") as string;
    if (!pieceUuid) {
      throw new Error(`[ORIGIN] SideStore.piece: pieceUuid is undefined for side ${this.guid}`);
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

  get port(): Guid {
    const portUuid = this.ySide.get("port") as string;
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeGuid = pieceStore.type;
    if (typeGuid) {
      const typeStore = this.parent.parent.type(typeGuid);
      if (typeStore) {
        const portStore = typeStore.port(portUuid);
        return portStore.guid;
      }
    }
    return portUuid;
  }
  set port(port: Guid) {
    // Find the port through the piece's type
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeGuid = pieceStore.type;
    if (typeGuid) {
      const typeStore = this.parent.parent.type(typeGuid);
      if (typeStore) {
        const portStore = typeStore.ports.get(port);
        if (portStore) {
          this.ySide.set("port", portStore.guid);
        }
      }
    }
  }

  hash = (side: Side): string => {
    return JSON.stringify(side);
  };

  snapshot = (): Side => {
    const currentData = {
      guid: this.guid,
      piece: this.piece,
      designPiece: this.designPiece,
      port: this.port,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  id = (): string => {
    return this.guid;
  };

  change = (diff: SideDiff) => {
    if (diff.piece !== undefined) this.piece = diff.piece;
    if (diff.designPiece !== undefined) this.designPiece = diff.designPiece;
    if (diff.port !== undefined) this.port = diff.port;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe, true);
  };
}

// #endregion Side

// #region Connection

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
    const yConnected = this.yConnection.set("connected", new Y.Map<YSideVal>());
    this.connected = new SideStore(parent, yConnected, connection.connected);
    const yConnecting = this.yConnection.set("connecting", new Y.Map<YSideVal>());
    this.connecting = new SideStore(parent, yConnecting, connection.connecting);
    this.gap = connection.gap;
    this.shift = connection.shift;
    this.rise = connection.rise;
    this.rotation = connection.rotation;
    this.turn = connection.turn;
    this.tilt = connection.tilt;
    this.x = connection.x;
    this.y = connection.y;
    this.description = connection.description;
    this.attributes = new Map();
    this.yAttributes = this.yConnection.set("attributes", new Y.Array<YAttribute>());
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

  get x(): number | undefined {
    return this.yConnection.get("x") as number | undefined;
  }
  set x(x: number | undefined) {
    if (x !== undefined) this.yConnection.set("x", x);
  }

  get y(): number | undefined {
    return this.yConnection.get("y") as number | undefined;
  }
  set y(y: number | undefined) {
    if (y !== undefined) this.yConnection.set("y", y);
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
      x: this.x,
      y: this.y,
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
    if (diff.x !== undefined) this.x = diff.x;
    if (diff.y !== undefined) this.y = diff.y;
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

// useIsConnectionSelected, useIsConnectionHovered, useConnectionStatus - moved to designAppIntegration.ts

// #endregion Connection

// #region Stat

type YStat = Y.Map<string | number | boolean>;
type YStats = Y.Array<YStat>;

class StatStore {
  private yStat: YStat;
  private cache?: Stat;
  private cacheHash?: string;

  constructor(yStat: YStat, stat: Stat) {
    this.yStat = yStat;
    this.guid = stat.guid;
    this.key = stat.key;
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

  get key(): string {
    return this.yStat.get("key") as string;
  }
  set key(key: string) {
    this.yStat.set("key", key);
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
      key: this.key,
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
    if (diff.key !== undefined) this.key = diff.key;
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

// #endregion Stat

// #region Design

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
    this.parentGuid = design.parent;
    this.abstract = design.isAbstract;
    this.canScale = design.canScale;
    this.canMirror = design.canMirror;
    this.unit = design.unit;
    this.icon = design.icon;
    this.image = design.image;
    this.description = design.description;

    this.yPieces = this.yDesign.set("pieces", new Y.Array<YPiece>());
    if (design.pieces) {
      for (const piece of design.pieces) {
        this.createPiece(piece);
      }
    }

    this.yConnections = this.yDesign.set("connections", new Y.Array<YConnection>());
    if (design.connections) {
      for (const connection of design.connections) {
        this.createConnection(connection);
      }
    }

    this.yAttributes = this.yDesign.set("attributes", new Y.Array<YAttribute>());
    if (design.attributes) {
      for (const attribute of design.attributes) {
        this.createAttribute(attribute);
      }
    }

    this.yStats = this.yDesign.set("stats", new Y.Array<YStat>());
    if (design.stats) {
      for (const stat of design.stats) {
        this.createStat(stat);
      }
    }

    this.yProps = this.yDesign.set("props", new Y.Array<YProp>());
    if (design.props) {
      for (const prop of design.props) {
        this.createProp(prop);
      }
    }

    this.yLayers = this.yDesign.set("layers", new Y.Array<YLayer>());
    if (design.layers) {
      for (const layer of design.layers) {
        this.createLayer(layer);
      }
    }

    if (design.activeLayer) {
      this.yDesign.set("activeLayer", design.activeLayer);
    }

    this.yGroups = this.yDesign.set("groups", new Y.Array<YGroup>());
    if (design.groups) {
      for (const group of design.groups) {
        this.createGroup(group);
      }
    }

    if (design.location) {
      const yLocation = new Y.Map() as YLocation;
      this.yDesign.set("location", yLocation);
      this.location = new YLocationStore(yLocation, design.location);
    }

    this.yConcepts = this.yDesign.set("concepts", new Y.Array<string>());
    if (design.concepts) {
      design.concepts.forEach((concept) => this.yConcepts.push([concept]));
    }

    this.authors = new Map();
    if (design.authors) {
      design.authors.forEach((authorGuid) => {
        const authorStore = this.parent.author(authorGuid);
        this.authors.set(authorGuid, authorStore);
      });
    }
    this.yAuthors = this.yDesign.set("authors", new Y.Array<YAuthorUuid>());
    this.authors.forEach((author) => this.yAuthors.push([author.guid]));

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
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  createStat(stat: Stat): void {
    const yStat = new Y.Map() as YStat;
    this.yStats.push([yStat]);
    const yStatStore = new StatStore(yStat, stat);
    this.stats.set(stat.key, yStatStore);
  }

  createProp(prop: Prop): void {
    const yProp = new Y.Map() as YProp;
    this.yProps.push([yProp]);
    const yPropStore = new PropStore(yProp, prop);
    this.props.set(prop.key, yPropStore);
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
    const currentData = {
      guid: this.guid,
      name: this.name,
      parent: this.parentGuid,
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
      activeLayer: this.yDesign.get("activeLayer") as string | undefined,
      groups: Array.from(this.groups.values()).map((group) => group.snapshot()),
      location: this.location?.snapshot(),
      authors: Array.from(this.authors.values()).map((author) => author.guid),
      concepts: (this.yDesign.get("concepts") as Y.Array<string> | undefined)?.toArray(),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
      createdAt: this.createdAt,
      updatedAt: this.updatedAt,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: DesignDiff) => {
    if (diff.name !== undefined) this.name = diff.name;
    if (diff.parent !== undefined) this.parentGuid = diff.parent;
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
        // Handle incremental updates
        if (diff.pieces.added) {
          diff.pieces.added.forEach((piece) => this.createPiece(piece));
        }
        if (diff.pieces.updated) {
          diff.pieces.updated.forEach(({ id, diff: pieceDiff }) => {
            const pieceStore = this.pieces.get(id);
            if (pieceStore) {
              pieceStore.change(pieceDiff);
            } else {
            }
          });
        }
        if (diff.pieces.removed) {
          diff.pieces.removed.forEach((guid) => {
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
        // Handle complete replacement (legacy behavior)
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
        // Handle incremental updates
        if (diff.connections.added) {
          diff.connections.added.forEach((connection) => this.createConnection(connection));
        }
        if (diff.connections.updated) {
          diff.connections.updated.forEach(({ id, diff: connectionDiff }) => {
            // Find connection by composite id (connected/connecting pieces)
            const connectionStore = Array.from(this.connections.values()).find((c) => {
              const snapshot = c.snapshot();
              return snapshot.connected.piece === id.connected.piece && snapshot.connecting.piece === id.connecting.piece;
            });
            if (connectionStore) {
              connectionStore.change(connectionDiff);
            }
          });
        }
        if (diff.connections.removed) {
          diff.connections.removed.forEach((compositeId) => {
            // Find connection by composite id
            const connectionStore = Array.from(this.connections.values()).find((c) => {
              const snapshot = c.snapshot();
              return snapshot.connected.piece === compositeId.connected.piece && snapshot.connecting.piece === compositeId.connecting.piece;
            });
            if (connectionStore) {
              const connectionArray = Array.from(this.connections.values());
              const connectionIndex = connectionArray.findIndex((c) => c.guid === connectionStore.guid);
              if (connectionIndex !== -1) {
                this.connections.delete(connectionStore.guid);
                this.yConnections.delete(connectionIndex, 1);
              }
            }
          });
        }
      } else {
        // Handle complete replacement (legacy behavior)
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
        diff.stats.removed.forEach((guid) => {
          this.stats.delete(guid);
          const yStats = this.yDesign.get("stats") as Y.Array<YStat>;
          if (yStats) {
            const index = yStats.toArray().findIndex((yStat) => (yStat as Y.Map<unknown>).get("guid") === guid);
            if (index >= 0) yStats.delete(index, 1);
          }
        });
      }
      if (diff.stats.updated) {
        diff.stats.updated.forEach(({ id, diff: statDiff }) => {
          const statStore = this.stats.get(id);
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
        diff.props.removed.forEach((key) => {
          this.props.delete(key);
          const yProps = this.yDesign.get("props") as Y.Array<YProp>;
          if (yProps) {
            const index = yProps.toArray().findIndex((yProp) => (yProp as Y.Map<unknown>).get("key") === key);
            if (index >= 0) yProps.delete(index, 1);
          }
        });
      }
      if (diff.props.updated) {
        diff.props.updated.forEach(({ id, diff: propDiff }) => {
          const propStore = this.props.get(id);
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
        diff.layers.removed.forEach((path) => {
          this.layers.delete(path);
          const yLayers = this.yDesign.get("layers") as Y.Array<YLayer>;
          if (yLayers) {
            const index = yLayers.toArray().findIndex((yLayer) => (yLayer as Y.Map<unknown>).get("path") === path);
            if (index >= 0) yLayers.delete(index, 1);
          }
        });
      }
      if (diff.layers.updated) {
        diff.layers.updated.forEach(({ id, diff: layerDiff }) => {
          const layerStore = this.layers.get(id);
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
        this.yDesign.set("activeLayer", diff.activeLayer);
      } else {
        this.yDesign.delete("activeLayer");
      }
    }

    if (diff.groups !== undefined) {
      if (diff.groups.removed) {
        diff.groups.removed.forEach((pieces) => {
          const groupKey = pieces.join(",");
          this.groups.delete(groupKey);
          const yGroups = this.yDesign.get("groups") as Y.Array<YGroup>;
          if (yGroups) {
            const index = yGroups.toArray().findIndex((yGroup) => {
              const groupPieces = (yGroup as Y.Map<unknown>).get("pieces") as Y.Array<string>;
              return groupPieces?.toArray().join(",") === groupKey;
            });
            if (index >= 0) yGroups.delete(index, 1);
          }
        });
      }
      if (diff.groups.updated) {
        diff.groups.updated.forEach(({ id, diff: groupDiff }) => {
          const groupKey = id.join(",");
          const groupStore = this.groups.get(groupKey);
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
        diff.authors.removed.forEach((authorGuid) => {
          this.authors.delete(authorGuid);
        });
      }
      if (diff.authors.updated) {
        diff.authors.updated.forEach(({ id, diff: authorDiff }) => {
          const authorStore = this.authors.get(id);
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
        diff.concepts.forEach((concept) => yConcepts.push([concept]));
        this.yDesign.set("concepts", yConcepts);
      } else {
        this.yDesign.delete("concepts");
      }
    }

    if ("attributes" in diff) {
      if (diff.attributes && typeof diff.attributes === "object" && ("added" in diff.attributes || "removed" in diff.attributes || "updated" in diff.attributes)) {
        // Handle incremental updates
        if (diff.attributes.removed) {
          diff.attributes.removed.forEach((guid) => {
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
          diff.attributes.updated.forEach(({ id, diff: attrDiff }) => {
            const attr = this.attributes.get(id);
            if (attr) {
              attr.change(attrDiff);
            }
          });
        }
        if (diff.attributes.added) {
          diff.attributes.added.forEach((attribute) => this.createAttribute(attribute));
        }
      } else {
        // Handle complete replacement (array format)
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
  if (!kitStore) return null;
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? guid;
  if (!designGuid) throw new Error("useDesignStore must be called within a DesignScopeProvider or be directly provided with a guid");
  if (!kitStore.hasDesign(designGuid)) throw new Error(`Design store not found for design ${designGuid}`);
  const designStore = kitStore.design(designGuid);
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: Guid, deep: boolean = false): T | DesignShallow | Design | null {
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? id;
  if (!designGuid) {
    return null;
  }
  const store = useDesignStore(identitySelector, designGuid);
  if (!store) return null;
  if (deep) {
    return useSyncDeep<Design, T>(store as DesignStore, selector ? selector : (identitySelector as any));
  }
  return useSync<DesignShallow, T>(store as any, selector ? selector : (identitySelector as any));
}

export function usePieces(): Piece[] {
  const design = useDesign() as Design;
  return design.pieces ?? [];
}

export function useFlattenDiff(): DesignDiff {
  const designScope = useDesignScope();
  const kit = useKit() as Kit;
  if (!designScope) throw new Error("useFlattenDiff must be called within a DesignScopeProvider");
  return flattenDesign(kit, designScope.guid);
}

export function useFlatDesign(): Design {
  const design = useDesign() as Design;
  const diff = useFlattenDiff();
  return applyDesignDiff(design, diff);
}

export function useFlatPieces(): Piece[] {
  const design = useFlatDesign();
  return design.pieces ?? [];
}

export function usePiecesMetadata(): Map<
  string,
  {
    plane: Plane;
    center: Coord;
    fixedPieceId: string;
    parentPieceId: string | null;
    depth: number;
  }
> {
  const kit = useKit(undefined, undefined, true) as Kit;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePiecesMetadata must be called within a DesignScopeProvider");
  return piecesMetadata(kit, designScope.guid);
}

export function useIncludedDesigns() {
  const design = useDesign() as Design;
  return useMemo(() => getIncludedDesigns(design), [design]);
}

export function useDesignId() {
  const design = useDesign() as Design;
  return useMemo(() => ({ name: design.name, parent: design.parent }), [design.name, design.parent]);
}

// useClusterableGroups - moved to designAppIntegration.ts

export function usePiecePlanes(): Plane[] {
  const flatDesign = useFlatDesign();
  return useMemo(() => flatDesign.pieces?.map((p: Piece) => p.plane!) || [], [flatDesign]);
}

export function usePieceRepresentationUrls(): Map<string, string> {
  const flatDesign = useFlatDesign();
  // TODO: Re-enable once circular dependency is fully resolved
  // const types = usePortColoredTypes();
  const types = useKit((k) => k?.types || []) as Type[];
  const kit = useKit((k) => k as Kit) as Kit | null;
  const kitStore = useKitStore((s) => s) as KitStore;
  const files = kit?.files ?? [];
  const getFileUrl = React.useCallback(
    (fileGuid: string) => {
      return kitStore.getFileUrl(fileGuid);
    },
    [kitStore],
  );
  return useMemo(() => getPieceRepresentationUrls(flatDesign, types, files, getFileUrl), [flatDesign, types, files, getFileUrl]);
}

export function usePieceDiffStatuses(): DiffStatus[] {
  const flatDesign = useFlatDesign();
  return useMemo(() => {
    return (
      flatDesign.pieces?.map((piece: Piece) => {
        const diffAttribute = piece.attributes?.find((q: any) => q.key === "semio.diffStatus");
        return (diffAttribute?.value as DiffStatus) || DiffStatus.Unchanged;
      }) || []
    );
  }, [flatDesign]);
}

export function usePiecesFromIds(pieceIds: Guid[]) {
  const design = useDesign() as Design;
  const includedDesigns = useIncludedDesigns();
  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.guid, d])), [includedDesigns]);

  return useMemo(() => {
    return pieceIds.map((id) => {
      try {
        const foundPiece = findPieceInDesign(design, id);
        return {
          ...foundPiece,
          id_: foundPiece.guid,
        };
      } catch {
        const pieceIdString = typeof id === "string" ? id : (id as any).guid;
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
      }
    });
  }, [pieceIds, design, includedDesignMap]);
}

export function useReplacableTypes(pieceIds: Guid[], selectedVariants?: string[]) {
  const kit = useKit() as Kit;
  const design = useDesign() as Design;
  const designGuid = design.guid;

  return useMemo(() => {
    if (pieceIds.length === 1) {
      return findReplacableTypesForPieceInDesign(kit, designGuid, pieceIds[0], selectedVariants);
    } else {
      return findReplacableTypesForPiecesInDesign(kit, designGuid, pieceIds, selectedVariants);
    }
  }, [kit, designGuid, pieceIds, selectedVariants]);
}

export function useReplacableDesigns(piece: Piece) {
  const kit = useKit() as Kit;
  const design = useDesign() as Design;
  const designGuid = design.guid;

  return useMemo(() => {
    return findReplacableDesignsForDesignPiece(kit, designGuid, piece);
  }, [kit, designGuid, piece]);
}

export function useExplodeableDesignNodes(nodes: any[], selection: any) {
  const kit = useKit() as Kit;
  return useMemo(() => {
    return nodes.filter((node) => {
      if (node.type !== "design") return false;
      const Guid = node.data.piece.id_;
      if (!selection.pieces?.includes(Guid)) return false;
      const designName = (node.data.piece as any).type?.variant;
      if (!designName) return false;
      if (!kit?.designs?.find((d: any) => d.name === designName)) return false;
      return true;
    });
  }, [nodes, selection.pieces, kit]);
}

// #endregion Design

// #region Kit

type YIdMap = Y.Map<string>;
type YKitVal = string | Y.Array<string> | YIdMap | YAttributes | YAuthors | YFiles | YFolders | YBenchmarks | YQualities | YProps | YTypes | YDesigns;
type YKit = Y.Map<YKitVal>;
type YKits = Y.Array<YKit>;

export class KitStore {
  public readonly parent: SketchpadStore;
  private readonly remoteProviders: RemoteProviders | undefined;
  private fileProvider?: FileProvider;
  public readonly yDoc: Y.Doc;
  private readonly yKit: YKit;
  private readonly yTypes: YTypes;
  private readonly types: Map<string, TypeStore>;
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

  constructor(parent: SketchpadStore, kit: Kit, local?: boolean, remote?: boolean, remoteProviders?: RemoteProviders) {
    this.parent = parent;
    this.remoteProviders = remote ? remoteProviders : undefined;
    this.yDoc = new Y.Doc();

    this.commandRegistry = new Map();
    this.regularFiles = new Map();
    this.types = new Map();
    this.designs = new Map();
    this.files = new Map();
    this.folders = new Map();
    this.qualities = new Map();
    this.benchmarks = new Map();
    this.authors = new Map();
    this.attributes = new Map();

    this.yKit = this.yDoc.getMap() as YKit;
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

      kit.attributes?.forEach((attribute) => this.createAttribute(attribute));
      kit.authors?.forEach((author) => this.createAuthor(author));
      kit.folders?.forEach((folder) => this.createFolder(folder));
      kit.qualities?.forEach((quality) => this.createQuality(quality));
      kit.types?.forEach((type) => this.createType(type));
      kit.designs?.forEach((design) => this.createDesign(design));
      kit.files?.forEach((file) => this.createFile(file));

      this.yKit.set("createdAt", new Date().toISOString());
      this.updated();
    });

    if (local) {
      this.persistence = new IndexeddbPersistence(`semio-kit-${kit.guid}`, this.yDoc);
    }

    if (remote && this.remoteProviders) {
      this.remoteProviders.yProvider(this.yDoc, this.name + "@" + this.version);
      // Initialize file provider if remoteProviders are available
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
      // Sync existing files
      await this.syncFiles();
    } catch (error) {
      console.error(`[KIT ${this.name}] Failed to initialize file provider:`, error);
    }
  }

  private async syncFiles() {
    if (!this.fileProvider) return;
    // Download all files from remote storage and create object URLs
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
  get concepts(): string[] | undefined {
    const yConcepts = this.yKit.get("concepts") as Y.Array<string> | undefined;
    return yConcepts ? yConcepts.toArray() : undefined;
  }
  set concepts(concepts: string[] | undefined) {
    if (concepts) {
      const yConcepts = new Y.Array<string>();
      concepts.forEach((concept) => yConcepts.push([concept]));
      this.yKit.set("concepts", yConcepts);
    } else {
      this.yKit.delete("concepts");
    }
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
    const yTypeStore = new TypeStore(this, yType, type);
    this.yTypes.push([yType]);
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
    this.yFiles.push([yFile]);
    const yFileStore = new FileStore(yFile, file);
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
    this.yFolders.push([yFolder]);
    const yFolderStore = new FolderStore(yFolder, folder);
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
    const folderPath = this.resolveFolderPath(file.folder);
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

    // First, check if we have it in memory (regularFiles)
    // regularFiles uses storage path as key, not guid
    const storagePath = this.getFileStoragePath(file);
    const memoryUrl = this.regularFiles.get(storagePath);
    if (memoryUrl) {
      return memoryUrl;
    }

    // If there's a remote URL (http/https), use it directly
    if (file.remote && (file.remote.startsWith("http://") || file.remote.startsWith("https://"))) {
      return file.remote;
    }

    // If we have a file provider, download the blob and create a blob URL
    if (this.fileProvider) {
      try {
        const blob = await this.fileProvider.download(this.guid, fileGuid, storagePath);
        if (blob) {
          const blobUrl = URL.createObjectURL(blob);
          // Cache it in memory for future use (using storage path as key)
          this.regularFiles.set(storagePath, blobUrl);
          return blobUrl;
        }
      } catch (error) {
        console.error("[KitStore] Failed to get blob for file:", fileGuid, error);
      }
    }

    return "";
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
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.set(attribute.guid, yAttributeStore);
  }

  attribute(guid: string): AttributeStore {
    return this.attributes.get(guid)!;
  }

  hash(kit: Kit): string {
    return JSON.stringify(kit);
  }

  snapshot = (): Kit => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      version: this.version,
      remote: this.remote,
      homepage: this.homepage,
      license: this.license,
      preview: this.preview,
      concepts: this.concepts,
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
      createdAt: this.createdAt,
      updatedAt: this.updatedAt,
    };
    const currentHash = this.hash(currentData);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }
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

      if (diff.authors) {
        if (diff.authors.added) {
          diff.authors.added.forEach((author) => this.createAuthor(author));
        }
        if (diff.authors.updated) {
          diff.authors.updated.forEach(({ id, diff: authorDiff }) => {
            const authorStore = this.authors.get(id as string);
            if (authorStore) {
              authorStore.change(authorDiff);
            }
          });
        }
        if (diff.authors.removed) {
          diff.authors.removed.forEach((authorGuidOrObject) => {
            const authorGuid = typeof authorGuidOrObject === "string" ? authorGuidOrObject : (authorGuidOrObject as any).guid;
            if (this.authors.has(authorGuid)) {
              this.authors.delete(authorGuid);
              // Find and delete from Y.Array
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
          diff.types.updated.forEach(({ id, diff: typeDiff }) => {
            const typeStore = this.types.get(id);
            if (typeStore) {
              typeStore.change(typeDiff);
            }
          });
        }
        if (diff.types.removed) {
          diff.types.removed.forEach((Guid) => {
            if (this.types.has(Guid)) {
              this.types.delete(Guid);
              // Find and delete from Y.Array
              const index = Array.from(this.yTypes).findIndex((yType: any) => {
                const yMap = yType[0] as Y.Map<any>;
                return yMap.get("guid") === Guid;
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
          diff.designs.updated.forEach(({ id, diff: designDiff }) => {
            const designStore = this.designs.get(id);
            if (designStore) {
              designStore.change(designDiff);
            }
          });
        }
        if (diff.designs.removed) {
          diff.designs.removed.forEach((Guid) => {
            if (this.designs.has(Guid)) {
              this.designs.delete(Guid);
              // Find and delete from Y.Array
              const index = Array.from(this.yDesigns).findIndex((yDesign: any) => {
                const yMap = yDesign[0] as Y.Map<any>;
                return yMap.get("guid") === Guid;
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
          diff.files.updated.forEach(({ id, diff: fileDiff }) => {
            const fileStore = this.files.get(id);
            if (fileStore) {
              fileStore.change(fileDiff);
            }
          });
        }
        if (diff.files.removed) {
          diff.files.removed.forEach((fileId) => {
            if (this.files.has(fileId)) {
              this.files.delete(fileId);
              // Find and delete from Y.Array
              const index = Array.from(this.yFiles).findIndex((yFile: any) => {
                const yMap = yFile[0] as Y.Map<any>;
                return yMap.get("guid") === fileId;
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
          diff.folders.updated.forEach(({ id, diff: folderDiff }) => {
            const folderStore = this.folders.get(id);
            if (folderStore) {
              folderStore.change(folderDiff);
            }
          });
        }
        if (diff.folders.removed) {
          diff.folders.removed.forEach((folderGuid) => {
            if (this.folders.has(folderGuid)) {
              this.folders.delete(folderGuid);
              // Find and delete from Y.Array
              const index = Array.from(this.yFolders).findIndex((yFolder: any) => {
                const yMap = yFolder[0] as Y.Map<any>;
                return yMap.get("guid") === folderGuid;
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
          diff.qualities.updated.forEach(({ id, diff: qualityDiff }) => {
            const qualityStore = this.qualities.get(id);
            if (qualityStore) {
              qualityStore.change(qualityDiff);
            }
          });
        }
        if (diff.qualities.removed) {
          diff.qualities.removed.forEach((qualityGuid) => {
            if (this.qualities.has(qualityGuid)) {
              this.qualities.delete(qualityGuid);
              // Find and delete from Y.Array
              const index = Array.from(this.yQualities).findIndex((yQuality: any) => {
                const yMap = yQuality[0] as Y.Map<any>;
                return yMap.get("guid") === qualityGuid;
              });
              if (index !== -1) {
                this.yQualities.delete(index, 1);
              }
            }
          });
        }
      }
      this.yKit.set("updatedAt", new Date().toISOString());
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

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    // Origins are strings like "semio.sketchpad.app.type.panel.details.name" (starts with semio.sketchpad)
    // Commands are strings like "semio.kit.updateDesign" (starts with semio. but NOT semio.sketchpad)
    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
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

      // Handle file operations
      if (result.diff.files) {
        // Add new files
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
                  console.log(`[KIT ${this.name}] Uploaded file ${storagePath} to ${remoteUrl}`);
                  this.file(file.guid).change({ remote: remoteUrl });
                } catch (error) {
                  console.error(`[KIT ${this.name}] Failed to upload file ${storagePath}:`, error);
                }
              }
            }
          }
        }

        // Delete removed files
        if (result.diff.files.removed) {
          for (const fileId of result.diff.files.removed) {
            const fileStore = this.files.get(fileId);
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
                  await this.fileProvider.delete(this.guid, fileId, storagePath);
                  console.log(`[KIT ${this.name}] Deleted file ${storagePath}`);
                } catch (error) {
                  console.error(`[KIT ${this.name}] Failed to delete file ${storagePath}:`, error);
                }
              }
            }
          }
        }
      }
    }

    // Handle local files (in-memory or blob URLs)
    if (result.files) {
      result.files.forEach((file) => {
        const objectUrl = URL.createObjectURL(file);
        this.regularFiles.set(file.name, objectUrl);
      });
    }

    console.groupEnd();
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
  if (!kitGuid) {
    return null;
  }
  if (!store.hasKit(kitGuid)) {
    return null;
  }
  const kitStore = store.kit(kitGuid);
  return selector ? selector(kitStore) : kitStore;
}

export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, guid?: Guid, deep: boolean = false): T | KitShallow | Kit | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  if (!resolvedGuid) {
    return null;
  }
  const kitStore = useKitStore(identitySelector, resolvedGuid);
  if (!kitStore) {
    return null;
  }
  if (deep) {
    return useSyncDeep<Kit, T>(kitStore as KitStore, selector ? selector : (identitySelector as any));
  }
  return useSync<KitShallow, T>(kitStore as any, selector ? selector : (identitySelector as any));
}

// useDiffedKit - moved to designAppIntegration.ts

export function useDesigns(): Design[] {
  return useKit((k) => k.designs ?? []) as Design[];
}

export function useFileUrls(): Map<Url, Url> {
  const kitStore = useKitStore() as KitStore | null;
  if (!kitStore) {
    return new Map();
  }
  return kitStore.fileUrls;
}

export function useKitCommands() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid;

  if (!kitGuid || !store.hasKit(kitGuid)) {
    return null;
  }

  const kitStore = store.kit(kitGuid);
  return {
    startTransaction: (origin: string) => {
      console.group(`[${origin}] Transaction: "kit.startTransaction"`);
      kitStore.yDoc.transact(() => {}, origin);
    },
    finalizeTransaction: (origin: string) => {
      console.groupEnd();
    },
    abortTransaction: (origin: string) => {
      console.groupEnd();
    },
    importKit: (origin: string, url: string) => kitStore.execute("semio.kit.import", origin, url),
    exportKit: (origin: string) => kitStore.execute("semio.kit.export", origin),
    createAuthor: (origin: string, author: Author) => kitStore.execute("semio.kit.createAuthor", origin, author),
    updateAuthor: (origin: string, Guid: Guid, authorDiff: AuthorDiff) => kitStore.execute("semio.kit.updateAuthor", origin, Guid, authorDiff),
    deleteAuthor: (origin: string, Guid: Guid) => kitStore.execute("semio.kit.deleteAuthor", origin, Guid),
    createType: (origin: string, type: Type) => kitStore.execute("semio.kit.createType", origin, type),
    updateType: (origin: string, guid: Guid, diff: TypeDiff) => kitStore.execute("semio.kit.updateType", origin, guid, diff),
    deleteType: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteType", origin, guid),
    createDesign: (origin: string, design: Design) => kitStore.execute("semio.kit.createDesign", origin, design),
    updateDesign: (origin: string, guid: Guid, diff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", origin, guid, diff),
    deleteDesign: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteDesign", origin, guid),
    createQuality: (origin: string, quality: Quality) => kitStore.execute("semio.kit.createQuality", origin, quality),
    updateQuality: (origin: string, guid: Guid, diff: QualityDiff) => kitStore.execute("semio.kit.updateQuality", origin, guid, diff),
    deleteQuality: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteQuality", origin, guid),
    addFile: (origin: string, file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", origin, file, blob),
    updateFile: (origin: string, url: Url, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", origin, url, fileDiff, blob),
    removeFile: (origin: string, url: Url) => kitStore.execute("semio.kit.removeFile", origin, url),
    createFolder: (origin: string, folder: Folder) => kitStore.execute("semio.kit.createFolder", origin, folder),
    updateFolder: (origin: string, guid: Guid, folderDiff: FolderDiff) => kitStore.execute("semio.kit.updateFolder", origin, guid, folderDiff),
    deleteFolder: (origin: string, guid: Guid) => kitStore.execute("semio.kit.deleteFolder", origin, guid),
    moveToFolder: (origin: string, artifactKind: string, artifactGuid: Guid, folderGuid: Guid | null) => kitStore.execute("semio.kit.moveToFolder", origin, artifactGuid, artifactKind, folderGuid),
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
}

// #endregion Kit

// #region Commands

const sqlWasmUrl = "https://sql.js.org/dist/sql-wasm.wasm";

export const kitCommands = {
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, guid: Guid, diff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { authors: { removed: [guid] } },
    };
  },
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, guid: Guid, diff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { types: { removed: [guid] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, guid: Guid, diff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { designs: { removed: [guid] } },
    };
  },
  "semio.kit.createQuality": (context: KitCommandContext, quality: Quality): KitCommandResult => {
    return {
      diff: { qualities: { added: [quality] } },
    };
  },
  "semio.kit.updateQuality": (context: KitCommandContext, guid: Guid, diff: QualityDiff): KitCommandResult => {
    return {
      diff: { qualities: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteQuality": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { qualities: { removed: [guid] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], file.name)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, fileGuid: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const existing = context.kit.files?.find((f) => f.guid === fileGuid);
    const fileName = fileDiff.name ?? existing?.name ?? "file";
    const files: File[] = blob ? [new File([blob], fileName)] : [];
    return {
      diff: { files: { updated: [{ id: fileGuid, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, fileGuid: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [fileGuid] } },
    };
  },
  "semio.kit.createFolder": (context: KitCommandContext, folder: Folder): KitCommandResult => {
    return {
      diff: { folders: { added: [folder] } },
    };
  },
  "semio.kit.updateFolder": (context: KitCommandContext, guid: Guid, diff: FolderDiff): KitCommandResult => {
    return {
      diff: { folders: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteFolder": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { folders: { removed: [guid] } },
    };
  },
  "semio.kit.moveToFolder": (context: KitCommandContext, artifactGuid: Guid, artifactKind: "type" | "design" | "quality" | "file" | "folder", folderGuid?: Guid): KitCommandResult => {
    switch (artifactKind) {
      case "type": {
        const type = context.kit.types?.find((t) => t.guid === artifactGuid);
        if (!type) throw new Error(`Type ${artifactGuid} not found`);
        if (type.parent) throw new Error("Only prototypes (types without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { types: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "design": {
        const design = context.kit.designs?.find((d) => d.guid === artifactGuid);
        if (!design) throw new Error(`Design ${artifactGuid} not found`);
        if (design.parent) throw new Error("Only protodesigns (designs without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { designs: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "quality": {
        const folderDiff = { folder: folderGuid };
        return { diff: { qualities: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "file": {
        const folderDiff = { folder: folderGuid };
        return { diff: { files: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "folder": {
        const parentDiff = { parent: folderGuid };
        return { diff: { folders: { updated: [{ id: artifactGuid, diff: parentDiff }] } } };
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
              // Extract filename from path (handles both "file.glb" and "folder/file.glb")
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
          let SQL: SqlJsStatic;
          let db: Database;
          try {
            SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
          } catch (err) {
            throw new Error("SQL.js failed to initialize for import.");
          }
          const response = await fetch(url);
          const zipData = await response.arrayBuffer();
          const zip = await JSZip.loadAsync(zipData);
          let kit: Kit | null = null;
          const files: KitCommandResult["files"] = [];

          const kitDbFile = zip.file("kit.db");
          if (kitDbFile) {
            const dbData = await kitDbFile.async("uint8array");
            db = new SQL.Database(dbData);
            const kitResult = db.exec("SELECT * FROM kit LIMIT 1");
            if (kitResult.length > 0) {
              const kitRow = kitResult[0];
              const kitData = Object.fromEntries(kitRow.columns.map((col, i) => [col, kitRow.values[0][i]]));
              kit = {
                guid: (kitData.uri as string) || `urn:kit:${kitData.name as string}:${kitData.version as string}`,
                name: kitData.name as string,
                description: kitData.description as string,
                version: kitData.version as string,
                icon: kitData.icon as string,
                image: kitData.image as string,
                preview: kitData.preview as string,
                remote: kitData.remote as string,
                homepage: kitData.homepage as string,
                license: kitData.license as string,
                types: [],
                designs: [],
                files: [],
                createdAt: new Date(kitData.createdAt as string),
                updatedAt: new Date(kitData.updatedAt as string),
              };
            }
            db.close();
          } else {
            const kitJsonFile = zip.file("kit.json");
            if (kitJsonFile) {
              const kitData = await kitJsonFile.async("text");
              kit = JSON.parse(kitData);
            }
          }

          for (const [filename, file] of Object.entries(zip.files)) {
            if (!(file as any).dir && filename !== "kit.db" && filename !== "kit.json") {
              const fileData = await (file as any).async("uint8array");
              files.push(new File([new Uint8Array(fileData)], filename));
            }
          }

          if (!kit) {
            throw new Error("No kit.json or kit.db found in ZIP file");
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
      let SQL: SqlJsStatic;
      let db: Database;
      try {
        SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
      } catch (err) {
        throw new Error("SQL.js failed to initialize for export.");
      }

      db = new SQL.Database();
      const zip = new JSZip();
      const kit = context.kit;

      const schema = `
        CREATE TABLE kit ( uri VARCHAR(2048) NOT NULL UNIQUE, name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, preview VARCHAR(1024) NOT NULL, version VARCHAR(64) NOT NULL, remote VARCHAR(1024) NOT NULL, homepage VARCHAR(1024) NOT NULL, license VARCHAR(1024) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY );
        CREATE TABLE type ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE design ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, "view" VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, UNIQUE (name, variant, "view", kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE representation ( url VARCHAR(1024) NOT NULL, description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id) );
        CREATE TABLE tag ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id) );
        CREATE TABLE concept ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, type_id INTEGER, design_id INTEGER, FOREIGN KEY(kit_id) REFERENCES kit (id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE port ( description VARCHAR(512) NOT NULL, family VARCHAR(64) NOT NULL, t FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), point_x FLOAT, point_y FLOAT, point_z FLOAT, direction_x FLOAT, direction_y FLOAT, direction_z FLOAT, type_id INTEGER, CONSTRAINT "Unique local_id" UNIQUE (local_id, type_id), FOREIGN KEY(type_id) REFERENCES type (id) );
        CREATE TABLE compatible_family ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, port_id INTEGER, FOREIGN KEY(port_id) REFERENCES port (id) );
        CREATE TABLE plane ( id INTEGER NOT NULL PRIMARY KEY, origin_x FLOAT, origin_y FLOAT, origin_z FLOAT, x_axis_x FLOAT, x_axis_y FLOAT, x_axis_z FLOAT, y_axis_x FLOAT, y_axis_y FLOAT, y_axis_z FLOAT );
        CREATE TABLE piece ( description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), type_id INTEGER, plane_id INTEGER, center_x FLOAT, center_y FLOAT, design_id INTEGER, UNIQUE (local_id, design_id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(plane_id) REFERENCES plane (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE connection ( description VARCHAR(512) NOT NULL, gap FLOAT NOT NULL, shift FLOAT NOT NULL, rise FLOAT NOT NULL, rotation FLOAT NOT NULL, turn FLOAT NOT NULL, tilt FLOAT NOT NULL, x FLOAT NOT NULL, y FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, connected_piece_id INTEGER, connected_port_id INTEGER, connecting_piece_id INTEGER, connecting_port_id INTEGER, design_id INTEGER, CONSTRAINT "no reflexive connection" CHECK (connecting_piece_id != connected_piece_id), FOREIGN KEY(connected_piece_id) REFERENCES piece (id), FOREIGN KEY(connected_port_id) REFERENCES port (id), FOREIGN KEY(connecting_piece_id) REFERENCES piece (id), FOREIGN KEY(connecting_port_id) REFERENCES port (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE quality ( name VARCHAR(64) NOT NULL, value VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, definition VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, port_id INTEGER, type_id INTEGER, piece_id INTEGER, connection_id INTEGER, design_id INTEGER, kit_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id), FOREIGN KEY(port_id) REFERENCES port (id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(piece_id) REFERENCES piece (id), FOREIGN KEY(connection_id) REFERENCES connection (id), FOREIGN KEY(design_id) REFERENCES design (id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE author ( name VARCHAR(64) NOT NULL, email VARCHAR(128) NOT NULL, rank INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, design_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(design_id) REFERENCES design (id) );
      `;

      try {
        db.run(schema);
        const insertQualities = (qualities: Attribute[] | undefined, fkColumn: string, fkValue: number) => {
          if (!qualities) return;
          const stmt = db.prepare(`INSERT INTO quality (name, value, unit, definition, ${fkColumn}) VALUES (?, ?, ?, ?, ?)`);
          qualities.forEach((q) => stmt.run([q.key, q.value ?? "", "", q.definition ?? "", fkValue]));
          stmt.free();
        };
        const insertAuthors = (authorGuids: string[] | undefined, fkColumn: string, fkValue: number) => {
          if (!authorGuids) return;
          const stmt = db.prepare(`INSERT INTO author (name, email, rank, ${fkColumn}) VALUES (?, ?, ?, ?)`);
          let rank = 0;
          authorGuids.forEach((authorGuid) => {
            const author = kit.authors?.find((a) => a.guid === authorGuid);
            if (author) {
              stmt.run([author.name, author.email ?? "", rank++, fkValue]);
            }
          });
          stmt.free();
        };

        const kitStmt = db.prepare("INSERT INTO kit (uri, name, description, icon, image, preview, version, remote, homepage, license, createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const nowIso = new Date().toISOString();
        kitStmt.run([`urn:kit:${kit.name}:${kit.version || ""}`, kit.name, kit.description || "", kit.icon || "", kit.image || "", kit.preview || "", kit.version || "", kit.remote || "", kit.homepage || "", kit.license || "", nowIso, nowIso]);
        kitStmt.free();
        const Guid = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
        insertQualities(kit.attributes, "kit_id", Guid);

        if (kit.concepts) {
          const conceptStmt = db.prepare('INSERT INTO concept (name, "order", kit_id) VALUES (?, ?, ?)');
          kit.concepts.forEach((concept, index) => conceptStmt.run([concept, index, Guid]));
          conceptStmt.free();
        }

        if (kit.types) {
          const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, parent_id, is_abstract, unit, createdAt, updatedAt, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
          const repStmt = db.prepare("INSERT INTO representation (url, description, type_id) VALUES (?, ?, ?)");
          const tagStmt = db.prepare('INSERT INTO tag (name, "order", representation_id) VALUES (?, ?, ?)');
          const portStmt = db.prepare("INSERT INTO port (local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

          for (const type of kit.types) {
            typeStmt.run([type.name, type.description || "", type.icon || "", type.image || "", type.parent || null, type.isAbstract ? 1 : 0, type.unit || "", nowIso, nowIso, Guid]);
            const typeDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
            insertQualities(type.attributes, "type_id", typeDbId);
            insertAuthors(type.authors, "type_id", typeDbId);

            if (type.representations) {
              for (const rep of type.representations) {
                repStmt.run([rep.file, rep.description ?? "", typeDbId]);
                const repDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                insertQualities(rep.attributes, "representation_id", repDbId);
                if (rep.tags) {
                  rep.tags.forEach((tag, index) => tagStmt.run([tag, index, repDbId]));
                }
                const fileUrl = context.fileUrls.get(rep.file);
                if (fileUrl) {
                  try {
                    const response = await fetch(fileUrl);
                    const fileBlob = await response.blob();
                    const fileData = await fileBlob.arrayBuffer();
                    zip.file(rep.file, fileData);
                  } catch (error) {}
                }
              }
            }

            if (type.ports) {
              for (const port of type.ports) {
                portStmt.run([
                  port.guid || "",
                  port.description || "",
                  port.family || "default",
                  port.t || 0,
                  port.point?.x || 0,
                  port.point?.y || 0,
                  port.point?.z || 0,
                  port.direction?.x || 0,
                  port.direction?.y || 0,
                  port.direction?.z || 1,
                  typeDbId,
                ]);
                const portDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                insertQualities(port.attributes, "port_id", portDbId);
              }
            }
          }
          typeStmt.free();
          repStmt.free();
          tagStmt.free();
          portStmt.free();
        }

        const dbBuffer = db.export();
        zip.file("kit.db", dbBuffer);
        zip.file("kit.json", JSON.stringify(kit, null, 2));

        const blob = await zip.generateAsync({ type: "blob" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${kit.name}-${kit.version || "latest"}.zip`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        throw error;
      } finally {
        if (db) {
          db.close();
        }
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
              id: guid,
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, guid)?.connections ?? []).some((connection) => connection.connected.piece === piece.guid || connection.connecting.piece === piece.guid)
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
              id: guid,
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece === candidate.guid || connection.connecting.piece === candidate.guid)
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
              id: guid,
              diff: { pieces: { removed: [piece] } },
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
              id: guid,
              diff: { pieces: { removed: pieces } },
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
              id: guid,
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
              id: guid,
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
              id: guid,
              diff: { connections: { removed: [{ connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, guid: Guid, connectionGuids: Guid[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    const connectionsToRemove =
      connectionGuids
        .map((connGuid) => design?.connections?.find((c) => c.guid === connGuid))
        .filter((c): c is Connection => c !== undefined)
        .map((c) => ({ connected: { piece: c.connected.piece }, connecting: { piece: c.connecting.piece } })) ?? [];
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { connections: { removed: connectionsToRemove } },
            },
          ],
        },
      },
    };
  },
};

// #endregion Commands

// #endregion Store

// #region Apps

// #region Design

export function useIsPieceSelected(): boolean {
  const piece = usePieceScope();
  const { useDesignAppSelection } = getDesignAppHooks();
  const selection = useDesignAppSelection();
  return selection.pieces?.includes(piece?.guid ?? "") ?? false;
}

export function useIsPieceHovered(): boolean {
  const { useDesignAppHover } = getDesignAppHooks();
  const hover = useDesignAppHover();
  const pieceScope = usePieceScope();
  if (!pieceScope || !hover) return false;
  return hover.pieces?.includes(pieceScope.guid) ?? false;
}

export function useIsPieceTransitiveHovered(): boolean {
  const pieceScope = usePieceScope();
  if (!pieceScope) return false;
  const { useDesignAppIsPieceTransitiveHovered } = getDesignAppHooks();
  return useDesignAppIsPieceTransitiveHovered(undefined, pieceScope.guid);
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

export function useIsConnectionSelected(): boolean {
  const connectionScope = useConnectionScope();
  const { useDesignAppSelection } = getDesignAppHooks();
  const selection = useDesignAppSelection();
  if (!connectionScope) return false;
  return selection.connections?.some((guid: string) => guid === connectionScope.guid) ?? false;
}

export function useIsConnectionHovered(): boolean {
  const { useDesignAppHover } = getDesignAppHooks();
  const hover = useDesignAppHover();
  const connectionScope = useConnectionScope();
  if (!connectionScope || !hover) return false;
  return hover.connections?.includes(connectionScope.guid) ?? false;
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

export function useClusterableGroups() {
  const design = useDesign() as Design;
  const { useDesignAppSelection } = getDesignAppHooks();
  const selection = useDesignAppSelection();
  return useMemo(() => {
    if (!design) return [];
    return getClusterableGroups(design, selection.pieces ?? []);
  }, [design, selection.pieces]);
}

export function useDiffedKit(): Kit {
  const kit = useKit() as Kit;
  const { useDesignAppDiff } = getDesignAppHooks();
  const diff = useDesignAppDiff();
  return diff ? applyKitDiff(kit, diff) : kit;
}

export function usePortColoredTypes(): Type[] {
  const diffedKit = useDiffedKit();
  const kit = useKit() as Kit;
  const typesWithColoredPorts = useMemo(() => {
    if (!diffedKit.types || !kit.types) return [];
    const colorDiff = colorPortsForTypes(diffedKit.types);
    const updatedIds = colorDiff.updated?.map((u) => u.id) || [];
    return kit.types.filter((t) => updatedIds.includes(t.guid));
  }, [diffedKit.types, kit.types]);
  return typesWithColoredPorts;
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

// #endregion Design

// #region Sketchpad

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
    return selector(store.snapshot());
  }, [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
}

export function useSyncDeep<T, TSelected = T>(store: { onChangedDeep: (subscribe: Subscribe) => Disposable; snapshot: () => T }, selector: (value: T) => TSelected = identitySelector as any, deep?: boolean): TSelected {
  const subscribe = useCallback(
    (callback: () => void) => {
      return store.onChangedDeep((cb: () => void) => {
        cb();
        callback();
        return () => {};
      });
    },
    [store],
  );
  const getSnapshot = useCallback(() => {
    return selector(store.snapshot());
  }, [store, selector]);
  return useSyncExternalStore(subscribe, getSnapshot);
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
type YKitMetadatas = Y.Array<YKitMetadata>;

type KitAppStoreInstance = any;
type DesignAppStoreInstance = any;
type TypeAppStoreInstance = any;
type QualityAppStoreInstance = any;
type HomeStoreInstance = any;
type DocsAppStoreInstance = any;

type KitAppStoreFactory = (parent: SketchpadStore, yMap: YKitApp, transact: (fn: () => void) => void, id: KitAppId, state?: KitAppState) => KitAppStoreInstance;
type DesignAppStoreFactory = (parent: SketchpadStore, yMap: YDesignApp, transact: (fn: () => void) => void, id: DesignAppId, state?: DesignAppState) => DesignAppStoreInstance;
type TypeAppStoreFactory = (parent: SketchpadStore, yMap: YTypeApp, transact: (fn: () => void) => void, id: TypeAppId, state?: TypeAppState) => TypeAppStoreInstance;
type QualityAppStoreFactory = (parent: SketchpadStore, yMap: YQualityApp, transact: (fn: () => void) => void, id: QualityAppId, state?: QualityAppState) => QualityAppStoreInstance;
type HomeStoreFactory = (parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) => HomeStoreInstance;
type DocsAppStoreFactory = (parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) => DocsAppStoreInstance;

let kitAppStoreFactory: KitAppStoreFactory | undefined;
let designAppStoreFactory: DesignAppStoreFactory | undefined;
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;
let homeStoreFactory: HomeStoreFactory | undefined;
let docsAppStoreFactory: DocsAppStoreFactory | undefined;

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

export function registerDocsAppStoreFactory(factory: DocsAppStoreFactory) {
  docsAppStoreFactory = factory;
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

function resolveDocsAppStoreFactory(): DocsAppStoreFactory {
  if (!docsAppStoreFactory) throw new Error("Docs app store factory not registered");
  return docsAppStoreFactory;
}

type YSketchpadVal = string | number | boolean | YDesignApps;
type YSketchpad = Y.Map<YSketchpadVal>;

export class SketchpadStore {
  // Eagerly load modules after SketchpadStore class is defined
  private static _modulesLoaded = false;
  public static _loadModules() {
    if (SketchpadStore._modulesLoaded) return;
    SketchpadStore._modulesLoaded = true;
    Promise.all([
      import("./apps/design/App").then((m) => {
        designAppModuleCache = m;
      }),
      import("./apps/home/App").then((m) => {
        homeAppModuleCache = m;
      }),
      import("./apps/kit/App").then((m) => {
        kitAppModuleCache = m;
        if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
          (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache = m;
        }
      }),
      import("./apps/type/App").then((m) => {
        typeAppModuleCache = m;
      }),
      import("./apps/quality/App").then((m) => {
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
    this.yHome = this.yDoc.getMap("home");
    this.yKitApps = this.yDoc.getMap("kitApps");
    this.yTypeApps = this.yDoc.getMap("typeApps");
    this.yQualityApps = this.yDoc.getMap("qualityApps");
    this.yDesignApps = this.yDoc.getMap("designApps");

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
      if (!this.ySketchpad.has("layout")) {
        this.ySketchpad.set("layout", JSON.stringify("desktop"));
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
        if (initialState.layout !== undefined) this.ySketchpad.set("layout", JSON.stringify(initialState.layout));
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

  hash = (state: SketchpadState): string => {
    return JSON.stringify(state);
  };

  snapshot = (): SketchpadState => {
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
    const layoutStr = this.ySketchpad.get("layout") as string;
    const layout: Layout = layoutStr ? JSON.parse(layoutStr) : "desktop";
    const currentValues = {
      navigation: migratePath((this.ySketchpad.get("navigation") as string) || "/"),
      navigationHistory: navigationHistory,
      navigationHistoryIndex: (this.ySketchpad.get("navigationHistoryIndex") as number) ?? 0,
      recentSearches: recentSearches,
      recentFocusItems: recentFocusItems,
      theme: this.ySketchpad.get("theme") as Theme,
      layout: layout,
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
                createdAt: new Date(),
                updatedAt: new Date(),
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
      const designApp = designAppFactory(this, yDesignApp, (fn: () => void) => this.yDoc.transact(fn), { kit, design });

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
      if (diff.layout) this.ySketchpad.set("layout", JSON.stringify(diff.layout));
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
    return createObserver(this.ySketchpad, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {
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
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      const kit = rest[0] as Kit;
      const local = rest[1] as boolean | undefined;
      const remote = rest[2] as boolean | undefined;
      this.createKit(kit, local, remote);
      return {} as T;
    }
    if (command === "semio.sketchpad.createKitApp") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      const id = rest[0] as KitAppId;
      this.createKitApp(id.kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createDesignApp") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      const id = rest[0] as DesignAppId;
      this.createDesignApp(id.kit, id.design);
      return {} as T;
    }
    if (command === "semio.sketchpad.importKit") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      const Guid = rest[0] as Guid;
      const url = rest[1] as string;
      const kitStore = this.kits.get(Guid);
      if (kitStore) {
        await kitStore.execute("semio.kit.import", origin, url);
      }
      return {} as T;
    }
    if (command === "semio.sketchpad.freeze") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
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
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
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
    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
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
    console.groupEnd();
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
      this.yTypeApps.forEach((_, key) => this.yTypeApps.delete(key));
      this.yQualityApps.forEach((_, key) => this.yQualityApps.delete(key));
      this.yDesignApps.forEach((_, key) => this.yDesignApps.delete(key));
      this.yHome.forEach((_, key) => this.yHome.delete(key));

      this.ySketchpad.set("navigation", state.sketchpad.navigation);
      this.ySketchpad.set("navigationHistory", JSON.stringify(state.sketchpad.navigationHistory));
      this.ySketchpad.set("navigationHistoryIndex", state.sketchpad.navigationHistoryIndex);
      this.ySketchpad.set("recentSearches", JSON.stringify(state.sketchpad.recentSearches));
      this.ySketchpad.set("recentFocusItems", JSON.stringify(state.sketchpad.recentFocusItems));
      this.ySketchpad.set("theme", state.sketchpad.theme);
      this.ySketchpad.set("layout", JSON.stringify(state.sketchpad.layout));
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
      this.homeStore = homeFactory(this, this.yHome, (fn: () => void) => this.yDoc.transact(fn));
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
    this.yDoc.transact(() => {
      let yTypeApp = this.yTypeApps.get(key) as Y.Map<YTypeAppVal>;
      if (!yTypeApp) {
        yTypeApp = new Y.Map<YTypeAppVal>();
        this.yTypeApps.set(key, yTypeApp);
      }
      const typeAppFactory = resolveTypeAppStoreFactory();
      const typeApp = typeAppFactory(this, yTypeApp, (fn: () => void) => this.yDoc.transact(fn), id);
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
      const qualityApp = qualityAppFactory(this, yQualityApp, (fn: () => void) => this.yDoc.transact(fn), Guid);
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

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);

export const SketchpadScopeProvider = (props: { id?: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents; initialState?: ExtendedInitialState; children: React.ReactNode }) => {
  const id = useMemo(() => props.id || guid(), [props.id]);

  if (!stores.has(id)) {
    const store = new SketchpadStore(id, props?.remote, props?.initialState);
    stores.set(id, store);

    if (typeof window !== "undefined") {
      (window as any).__SEMIO_STORE__ = store;
    }
  }
  return React.createElement(SketchpadScopeContext.Provider, { value: { id, remote: props.remote, onWindowEvents: props.onWindowEvents } }, props.children as any);
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

export function useAppType(): AppKind {
  const navigation = useNavigation();
  return useMemo(() => {
    const pathParts = navigation.split("/").filter((p: string) => p);
    if (pathParts.length === 0) return "home";

    if (pathParts[0] === "kits" && pathParts.length >= 2) {
      if (pathParts.length >= 4 && pathParts[2] === "designs") return "design";
      if (pathParts.length >= 4 && pathParts[2] === "types") return "type";
      if (pathParts.length >= 4 && pathParts[2] === "qualities") return "quality";
      return "kit";
    }
    if (pathParts[0] === "docs") return "docs";
    return "home";
  }, [navigation]);
}

export function getAppTypeFromPath(path: string): AppKind {
  const pathParts = path.split("/").filter((p) => p);
  if (pathParts.length === 0) return "home";

  if (pathParts[0] === "kits" && pathParts.length >= 2) {
    if (pathParts.length >= 4 && pathParts[2] === "designs") return "design";
    if (pathParts.length >= 4 && pathParts[2] === "types") return "type";
    if (pathParts.length >= 4 && pathParts[2] === "qualities") return "quality";
    return "kit";
  }
  if (pathParts[0] === "docs") return "docs";
  return "home";
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

export function useExpertise(): Expertise {
  return useSketchpad((s) => s.expertise) as Expertise;
}

export function useTooltip(): (key: string) => string | undefined {
  const expertise = useExpertise();
  return (key: string) => {
    if (expertise === Expertise.EXPERT) return undefined;
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
  const layout = useLayout();
  return typeof layout === "object" ? layout.isNavbarExpanded : false;
}

export function useIsFooterExpanded(): boolean {
  const layout = useLayout();
  return typeof layout === "object" ? layout.isFooterExpanded : false;
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
  const store = useSketchpadStore();

  const pathMatch = navigation.match(/^\/kits\/([^/?]+)(?:\/(designs|types|qualities)\/([^/?]+))?/);
  const kitGuid = pathMatch?.[1];
  const appKind = pathMatch?.[2];
  const itemGuid = pathMatch?.[3];

  const docsPanelVisibility = useSyncExternalStore(subscribeDocsPanelVisibility, getDocsPanelVisibilitySnapshot, getDocsPanelVisibilitySnapshot);

  const app = useMemo(() => {
    if (appType === "docs") return null;
    try {
      switch (appType) {
        case "home":
          return store.home();
        case "kit":
          if (kitGuid) return store.kitApp(kitGuid);
          return null;
        case "design":
          if (kitGuid && itemGuid) return store.designApp(kitGuid, itemGuid);
          return null;
        case "type":
          if (kitGuid && itemGuid) return store.typeApp(kitGuid, itemGuid);
          return null;
        case "quality":
          if (kitGuid && itemGuid) return store.qualityApp(kitGuid, itemGuid);
          return null;
        default:
          return null;
      }
    } catch (e) {
      return null;
    }
  }, [store, appType, kitGuid, itemGuid]);

  const yPanelVisibilityMap = useMemo(() => {
    if (!app) return null;
    const yMap = (app as any).yMap;
    if (!yMap) return null;
    return yMap.get("panelVisibility") as Y.Map<boolean> | null;
  }, [app]);

  const subscribe = useCallback(
    (callback: () => void) => {
      if (!yPanelVisibilityMap) {
        return () => {};
      }
      const observer = () => {
        callback();
      };
      yPanelVisibilityMap.observe(observer);
      return () => {
        yPanelVisibilityMap.unobserve(observer);
      };
    },
    [yPanelVisibilityMap],
  );

  const getSnapshot = useCallback(() => {
    if (!app) {
      return defaultPanelVisibility;
    }
    const snapshot = app.snapshot();
    return snapshot?.panelVisibility || defaultPanelVisibility;
  }, [app]);

  const panelVisibility = useSyncExternalStore(subscribe, getSnapshot);

  // Return docs panel visibility for docs app, otherwise return app panel visibility
  if (appType === "docs") {
    return docsPanelVisibility;
  }

  return panelVisibility;
}

export function useAppCommands() {
  const navigation = useNavigation();
  const appType = useAppType();
  const store = useSketchpadStore();

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
            togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
              updateDocsPanelVisibilityState((prev) => ({
                ...prev,
                [panelKey]: !prev[panelKey],
              }));
            },
            execute: (origin: string, command: string, ...args: any[]) => {},
          };
      }
    } catch (e) {}

    return {
      togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
        if (!app) {
          return;
        }
        try {
          const current = app.snapshot()?.panelVisibility;
          if (!current) {
            return;
          }
          app.change({
            panelVisibility: {
              [panelKey]: !current[panelKey],
            },
          });
        } catch (e) {}
      },
      execute: (origin: string, command: string, ...args: any[]) => {
        if (!app) return;
        return app.execute(command, origin, ...args);
      },
    };
  }, [store, appType, kitGuid, itemGuid, navigation]);
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
      setLayout: (origin: string, layout: Layout) => store.execute("semio.sketchpad.setLayout", origin, layout),
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
        navigate(path);
      },
      navigateToDesign: (kit: Guid, design: Guid) => {
        navigate(`/kits/${kit}/designs/${design}`);
      },
      navigateToType: (kit: Guid, type: Guid) => {
        navigate(`/kits/${kit}/types/${type}`);
      },
      navigateToQuality: (kit: Guid, quality: Guid) => {
        navigate(`/kits/${kit}/qualities/${quality}`);
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

// #endregion Store

// #region Commands

export const commands = {
  "semio.sketchpad.setTheme": (context: SketchpadCommandContext, theme: Theme): SketchpadCommandResult => {
    return {
      diff: { theme },
    };
  },
  "semio.sketchpad.setLayout": (context: SketchpadCommandContext, layout: Layout): SketchpadCommandResult => {
    return {
      diff: { layout },
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
  "semio.sketchpad.toggleFullscreen": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {
      diff: { isFullscreen: !context.sketchpad.isFullscreen },
    };
  },
  "semio.sketchpad.toggleNavbarExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const layout = context.sketchpad.layout;
    if (typeof layout === "object") {
      return {
        diff: { layout: { ...layout, isNavbarExpanded: !layout.isNavbarExpanded } },
      };
    }
    return {};
  },
  "semio.sketchpad.toggleFooterExpanded": (context: SketchpadCommandContext): SketchpadCommandResult => {
    const layout = context.sketchpad.layout;
    if (typeof layout === "object") {
      return {
        diff: { layout: { ...layout, isFooterExpanded: !layout.isFooterExpanded } },
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
    if (state.layout !== undefined) diff.layout = state.layout;
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
    // This command needs access to the store, which will be passed via special handling
    return {};
  },
  "semio.sketchpad.timetravel": (context: SketchpadCommandContext): SketchpadCommandResult => {
    // This command needs access to the store, which will be passed via special handling
    return {};
  },
};

// #endregion Commands

// #region Apps Registry

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
  }

  unregister(id: string): void {
    this.apps.delete(id);
  }

  getApp(id: string): AppRegistration | undefined {
    return this.apps.get(id);
  }

  getAllApps(): AppRegistration[] {
    return Array.from(this.apps.values()).sort((a, b) => (a.order || 0) - (b.order || 0));
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
      if (app.getPanels.length === 2) {
        configs[id] = app.getPanels(getLabelFn, getHotkeyFn || getLabelFn);
      } else {
        configs[id] = app.getPanels(getLabelFn);
      }
    }
    return configs;
  }

  async initialize(): Promise<void> {
    await this.autoDiscover();
  }
}

const appRegistry = new AppRegistry();
export { appRegistry };

// #endregion Apps Registry

// #region Navbar

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

  // Separate the stable functions from the changing state
  // This prevents unnecessary re-renders of components that only use the functions
  const contextValue = useMemo(
    () => ({ focusItems, setFocusItems: setFocusItemsStable, setOnFocusItem, triggerFocusItem }),
    // Only include focusItems, as the functions are already stable
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
        [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => (a.order || 0) - (b.order || 0)),
      };
      return updated;
    });
  }, []);

  const removeSection = useCallback((panelKey: PanelKey, sectionId: string) => {
    setSections((prev) => ({ ...prev, [panelKey]: prev[panelKey].filter((s) => s.id !== sectionId) }));
  }, []);

  return <PanelSectionContext.Provider value={{ sections, addSection, removeSection }}>{children}</PanelSectionContext.Provider>;
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

// #endregion Panel Sections

// #region Footer Items

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

  return <FooterItemContext.Provider value={{ items, addItem, removeItem }}>{children}</FooterItemContext.Provider>;
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

// #endregion Footer Items

// #region ConceptFilter

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
    <div className="flex flex-wrap items-center gap-single p-single border-b">
      {allConcepts
        .filter((c) => !selectedConcepts.includes(c))
        .map((concept) => (
          <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} id={`semio.sketchpad.filter.concept.${concept}`} icon={concept} />
        ))}
    </div>
  );
};

// #endregion ConceptFilter

// #region ToolGroup

export const ToolGroup: FC<ToolGroupProps> = ({ tools, activeTool, onToolChange, level = "panel" }) => {
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
    <div className="flex items-stretch border overflow-hidden h-large">
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
                id: mode.tooltipId || `semio.sketchpad.tool.${mode.id}`,
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

// #endregion ToolGroup

// #region DragDrop

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

// #endregion DragDrop

// #region Hotkeys

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

// #endregion Hotkeys

export function usePanelConfigs(): Record<string, EnrichedPanelDefinition[]> {
  const apps = appRegistry.getAllApps();
  const homeApp = apps.find((a) => a.id === "home");
  const docsApp = apps.find((a) => a.id === "docs");
  const designApp = apps.find((a) => a.id === "design");
  const typeApp = apps.find((a) => a.id === "type");
  const qualityApp = apps.find((a) => a.id === "quality");
  const kitApp = apps.find((a) => a.id === "kit");

  const homeConfigs = homeApp?.getPanels.length === 0 ? homeApp.getPanels() : [];
  const designConfigs = designApp?.getPanels.length === 0 ? designApp.getPanels() : [];
  const typeConfigs = typeApp?.getPanels.length === 0 ? typeApp.getPanels() : [];
  const qualityConfigs = qualityApp?.getPanels.length === 0 ? qualityApp.getPanels() : [];
  const kitConfigs = kitApp?.getPanels.length === 0 ? kitApp.getPanels() : [];

  const { t } = useI18nTranslation();

  const docsConfigs = useMemo(() => {
    if (!docsApp || docsApp.getPanels.length !== 2) return [];
    const getLabelFn = () => "";
    const getHotkeyFn = (id: string) => {
      const value = t(id);
      if (typeof value === "object" && value?.hotkey) {
        return typeof value.hotkey === "string" ? value.hotkey : "";
      }
      const hotkeyValue = t(`${id}.hotkey`);
      if (typeof hotkeyValue === "string") return hotkeyValue;
      if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
        return typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : "";
      }
      return "";
    };
    return docsApp.getPanels(getLabelFn, getHotkeyFn);
  }, [docsApp, t]);

  const allPanelIds = useMemo(() => {
    const ids: string[] = [];
    homeConfigs.forEach((p) => ids.push(p.id));
    docsConfigs.forEach((p) => ids.push(p.id));
    designConfigs.forEach((p) => ids.push(p.id));
    typeConfigs.forEach((p) => ids.push(p.id));
    qualityConfigs.forEach((p) => ids.push(p.id));
    kitConfigs.forEach((p) => ids.push(p.id));
    return ids;
  }, [homeConfigs, docsConfigs, designConfigs, typeConfigs, qualityConfigs, kitConfigs]);

  const hotkeysMap = useMemo(() => {
    const map = new Map<string, string | undefined>();
    allPanelIds.forEach((id) => {
      const value = t(id);
      let hotkey: string | undefined;
      if (typeof value === "object" && value?.hotkey) {
        hotkey = typeof value.hotkey === "string" ? value.hotkey : undefined;
      } else {
        const hotkeyValue = t(`${id}.hotkey`);
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

  return useMemo(
    () => ({
      home: enrich(homeConfigs),
      docs: enrich(docsConfigs),
      design: enrich(designConfigs),
      type: enrich(typeConfigs),
      quality: enrich(qualityConfigs),
      kit: enrich(kitConfigs),
    }),
    [homeConfigs, docsConfigs, designConfigs, typeConfigs, qualityConfigs, kitConfigs, hotkeysMap],
  );
}

interface NavigationProps {
  mobile?: boolean;
}

const Navigation: FC<NavigationProps> = ({ mobile = false }) => {
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams] = useSearchParams();
  const kits = useKits();

  const mode = useMode();
  const isMobile = useIsMobile();
  const isNavbarExpanded = useIsNavbarExpanded();

  const pathParts = navigation.split("/").filter((p) => p);
  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
  const isKitsPath = pathParts[0] === "kits";
  const isDocsPath = pathParts[0] === "docs";

  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;

  const docsSection = isDocsPath && pathParts[1] ? pathParts[1] : null;
  const docsPagePath = isDocsPath && pathParts.length > 1 ? pathParts.slice(1).join("/") : null;
  const docsSectionsList = getDocsRegistry().getAllSections();

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
      const now = new Date();
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
      const now = new Date();
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
      kitCommands.createDesign(origin, { guid, name: uniqueName, parent, pieces: [], connections: [] });
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
      kitCommands.createType(origin, { guid, name: uniqueName, parent, ports: [] });
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
        const existingNames = allDesigns.filter((design) => design.parent === d.guid).map((design) => design.name);
        const uniqueName = generateUniqueName(d.name, existingNames);
        kitCommands.createDesign(origin, {
          guid,
          name: uniqueName,
          parent: d.guid,
          pieces: [],
          connections: [],
        });
        if (kitGuid) sketchpadCommands.navigateToDesign(kitGuid, guid);
      } else {
        const typeObj = designOrType as Type;
        const existingNames = allTypes.filter((type) => type.parent === typeObj.guid).map((type) => type.name);
        const uniqueName = generateUniqueName(typeObj.name, existingNames);
        kitCommands.createType(origin, {
          guid,
          name: uniqueName,
          parent: typeObj.guid,
          ports: [],
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

  // Find current design or type or quality
  const design = designFromScope || (isDesignApp ? allDesigns.find((d) => d.guid === itemGuid) : undefined);
  const type = typeFromScope || (isTypeApp ? allTypes.find((t) => t.guid === itemGuid) : undefined);
  const quality = isQualityApp ? allQualities.find((q) => q.guid === itemGuid) : undefined;

  // Build folder chain for design (find root design's folder even if this is a child)
  const designFolderChain = useMemo(() => {
    if (!design || typeof design !== "object" || !("parent" in design)) return [];
    const designObj = design as Design;

    // Find the root design (traverse up the parent chain)
    let rootDesign = designObj;
    while (rootDesign.parent) {
      const parent = allDesigns.find((d) => d.guid === rootDesign.parent);
      if (!parent) break;
      rootDesign = parent;
    }

    // If root design has no folder, return empty
    if (!rootDesign.folder) return [];

    // Build folder chain from root's folder
    const chain: Folder[] = [];
    let currentFolderId: string | undefined = rootDesign.folder;
    while (currentFolderId) {
      const folder = allFolders.find((f) => f.guid === currentFolderId);
      if (!folder) break;
      chain.unshift(folder);
      currentFolderId = folder.parent;
    }
    return chain;
  }, [design, allDesigns, allFolders]);

  // Build folder chain for type (find root type's folder even if this is a child)
  const typeFolderChain = useMemo(() => {
    if (!type || typeof type !== "object" || !("parent" in type)) return [];
    const typeObj = type as Type;

    // Find the root type (traverse up the parent chain)
    let rootType = typeObj;
    while (rootType.parent) {
      const parent = allTypes.find((t) => t.guid === rootType.parent);
      if (!parent) break;
      rootType = parent;
    }

    // If root type has no folder, return empty
    if (!rootType.folder) return [];

    // Build folder chain from root's folder
    const chain: Folder[] = [];
    let currentFolderId: string | undefined = rootType.folder;
    while (currentFolderId) {
      const folder = allFolders.find((f) => f.guid === currentFolderId);
      if (!folder) break;
      chain.unshift(folder);
      currentFolderId = folder.parent;
    }
    return chain;
  }, [type, allTypes, allFolders]);

  // Build parent chain for designs
  const designParentChain = useMemo(() => {
    if (!design || typeof design !== "object" || !("parent" in design)) return [];
    const designObj = design as Design;
    const chain: Design[] = [];
    let current: Design | undefined = designObj;
    while (current) {
      if (!current.parent) break;
      const parentId: string = current.parent;
      const parent: Design | undefined = allDesigns.find((d) => d.guid === parentId);
      if (!parent) break;
      chain.unshift(parent);
      current = parent;
    }
    return chain;
  }, [design, allDesigns]);

  // Build parent chain for types
  const typeParentChain = useMemo(() => {
    if (!type || typeof type !== "object" || !("parent" in type)) return [];
    const typeObj = type as Type;
    const chain: Type[] = [];
    let current: Type | undefined = typeObj;
    while (current) {
      if (!current.parent) break;
      const parentId: string = current.parent;
      const parent: Type | undefined = allTypes.find((t) => t.guid === parentId);
      if (!parent) break;
      chain.unshift(parent);
      current = parent;
    }
    return chain;
  }, [type, allTypes]);

  // Build breadcrumb items for root designs (no parent)
  const createDesignLabel = useLabel("semio.sketchpad.navbar.createDesign");
  const createChildLabel = useLabel("semio.sketchpad.navbar.createChild");
  const createTypeLabel = useLabel("semio.sketchpad.navbar.createType");
  const createVersionLabel = useLabel("semio.sketchpad.navbar.createVersion");
  const defaultVersionLabel = useLabel("semio.sketchpad.app.kit.defaultVersion");

  const designNameItems = useMemo(() => {
    const rootDesigns = allDesigns.filter((d) => !d.parent);
    const items = rootDesigns.map((d) => ({
      label: d.name,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + createDesignLabel, href: "#create-design" });
    return items;
  }, [allDesigns, kitGuid, createDesignLabel]);

  // Build sibling items for each parent in the chain
  const designParentSiblingItems = useMemo(() => {
    return designParentChain.map((parent) => {
      const siblings = allDesigns.filter((d) => d.parent === parent.parent);
      const items = siblings.map((d) => ({
        label: d.name,
        href: `/kits/${kitGuid}/designs/${d.guid}`,
      }));
      items.push({ label: "+ " + createChildLabel, href: `#create-sibling-${parent.guid}` });
      return { parentGuid: parent.guid, items };
    });
  }, [designParentChain, allDesigns, kitGuid, createChildLabel]);

  const designChildItems = useMemo(() => {
    if (!design || typeof design !== "object" || !("guid" in design)) return [];
    const designObj = design as Design;
    const children = allDesigns.filter((d) => d.parent === designObj.guid);
    const items = children.map((d) => ({
      label: d.name,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + createChildLabel, href: "#create-child" });
    return items;
  }, [design, allDesigns, kitGuid, createChildLabel]);

  // Build breadcrumb items for root types (no parent)
  const typeNameItems = useMemo(() => {
    const rootTypes = allTypes.filter((t) => !t.parent);
    const items = rootTypes.map((t) => ({
      label: t.name,
      href: `/kits/${kitGuid}/types/${t.guid}`,
    }));
    items.push({ label: "+ " + createTypeLabel, href: "#create-type" });
    return items;
  }, [allTypes, kitGuid, createTypeLabel]);

  // Build sibling items for each parent in the chain
  const typeParentSiblingItems = useMemo(() => {
    return typeParentChain.map((parent) => {
      const siblings = allTypes.filter((t) => t.parent === parent.parent);
      const items = siblings.map((t) => ({
        label: t.name,
        href: `/kits/${kitGuid}/types/${t.guid}`,
      }));
      items.push({ label: "+ " + createChildLabel, href: `#create-sibling-${parent.guid}` });
      return { parentGuid: parent.guid, items };
    });
  }, [typeParentChain, allTypes, kitGuid, createChildLabel]);

  const typeChildItems = useMemo(() => {
    if (!type || typeof type !== "object" || !("guid" in type)) return [];
    const typeObj = type as Type;
    const children = allTypes.filter((t) => t.parent === typeObj.guid);
    const items = children.map((typeObj) => ({
      label: typeObj.name,
      href: `/kits/${kitGuid}/types/${typeObj.guid}`,
    }));
    items.push({ label: "+ " + createChildLabel, href: "#create-child" });
    return items;
  }, [type, allTypes, kitGuid, createChildLabel]);

  // Build breadcrumb items for kit versions
  const kitVersionItems = useMemo(() => {
    if (!kit?.name) return [];
    const sameNameKits = kits.filter((k) => k.name === kit.name);
    const items = sameNameKits.map((k) => ({
      label: k.version || <span className="italic opacity-70">{defaultVersionLabel}</span>,
      href: `/kits/${k.guid}`,
    }));
    items.push({ label: "+ " + createVersionLabel, href: "#create-version" });
    return items;
  }, [kit, kits, defaultVersionLabel, createVersionLabel]);

  // Build breadcrumb items for home page kits filtered by kind
  const homeKitsByKind = useFilteredKitShallows(homeKind || undefined);
  const homeKitsForKind = useMemo(() => {
    if (!homeKind) return [];
    return homeKitsByKind.map((k) => ({
      label: k.name,
      href: `/?kind=${homeKind}&name=${encodeURIComponent(k.name)}`,
    }));
  }, [homeKind, homeKitsByKind]);

  // Build breadcrumb items for home page versions filtered by name
  const homeVersionsForName = useMemo(() => {
    if (!homeName || !homeKind) return [];
    return homeKitsByKind
      .filter((k) => k.name === homeName)
      .map((k) => ({
        label: k.version || <span className="italic opacity-70">{defaultVersionLabel}</span>,
        href: `/kits/${k.guid}`,
      }));
  }, [homeName, homeKind, homeKitsByKind, defaultVersionLabel]);

  // Build breadcrumb items for filtered names in kit app
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

  // Determine if we're at root or if a kind filter is active
  const isAtRoot = navigation === "/";
  const hasKindFilter = filteredKind || (kitGuid && kitKind);

  return (
    <Breadcrumb className="flex-1 min-w-0">
      <BreadcrumbList>
        {/* Always show Home icon with dropdown to select kinds */}
        <BreadcrumbItem id="semio.sketchpad.navbar.home" items={kitKindItems} onNavigate={(href) => navigate(href)}>
          <BreadcrumbLink onClick={() => navigate("/")} style={{ cursor: "pointer" }}>
            <HomeIcon size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>

        {/* If viewing a kit (or we have a selected home kind), show the kind breadcrumb */}
        {kitGuid || homeKind ? (
          <>
            {(kitKind || homeKind) && (
              <BreadcrumbItem id={`semio.sketchpad.navbar.breadcrumb.${kitKind || homeKind}`} items={!kitGuid ? homeKitsForKind : undefined} onNavigate={!kitGuid ? (href) => navigate(href) : undefined}>
                <BreadcrumbLink onClick={() => navigate(`/?kind=${kitKind || homeKind}`)} style={{ cursor: "pointer" }}>
                  {(kitKind === "temporary" || homeKind === "temporary") && <TemporaryKitIcon size={16} />}
                  {(kitKind === "local" || homeKind === "local") && <LocalKitIcon size={16} />}
                  {(kitKind === "remote" || homeKind === "remote") && <RemoteKitIcon size={16} />}
                </BreadcrumbLink>
              </BreadcrumbItem>
            )}

            {homeName && (
              <>
                <BreadcrumbItem id="semio.sketchpad.navbar.kitName" items={homeVersionsForName} onNavigate={(href) => navigate(href)}>
                  <BreadcrumbLink onClick={() => navigate(`/?kind=${homeKind}&name=${encodeURIComponent(homeName)}`)} style={{ cursor: "pointer" }}>
                    {homeName}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                {homeVersion !== null && (
                  <BreadcrumbItem id="semio.sketchpad.navbar.kitVersion">
                    <BreadcrumbLink style={{ cursor: "default" }}>{homeVersion || <span className="italic opacity-70">{defaultVersionLabel}</span>}</BreadcrumbLink>
                  </BreadcrumbItem>
                )}
              </>
            )}
            {kitGuid && (
              <>
                <BreadcrumbItem
                  id="semio.sketchpad.navbar.kit"
                  items={kitItemsWithCreate}
                  onNavigate={(href) => {
                    if (href === "#create-kit") handleCreateKit("semio.sketchpad.navbar.kits");
                    else navigate(href);
                  }}
                >
                  <BreadcrumbLink
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}`);
                    }}
                    style={{ cursor: "pointer" }}
                  >
                    {kit?.name || kitGuid}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbItem
                  id="semio.sketchpad.navbar.kitVersion"
                  items={kitVersionItems}
                  onNavigate={(href) => {
                    if (href === "#create-version") handleCreateVersion("semio.sketchpad.navbar.versions");
                    else navigate(href);
                  }}
                >
                  <BreadcrumbLink
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      const versionParam = kit?.version !== undefined ? `&version=${encodeURIComponent(kit.version)}` : "";
                      navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}${versionParam}`);
                    }}
                    style={{ cursor: "pointer" }}
                  >
                    {kit?.version || <span className="italic opacity-70">{defaultVersionLabel}</span>}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
          </>
        ) : null}
        {isKitApp && (
          <>
            <BreadcrumbItem id="semio.sketchpad.navbar.artifacts" items={artifactKinds} onNavigate={(href) => navigate(href)}>
              <BreadcrumbLink style={{ cursor: "default" }}>{/* Empty link for dropdown trigger */}</BreadcrumbLink>
            </BreadcrumbItem>
            {filteredKind && (
              <>
                <BreadcrumbItem id={`semio.sketchpad.navbar.breadcrumb.${filteredKind}`} items={filteredNameItems} onNavigate={(href) => navigate(href)}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}`)} style={{ cursor: "pointer" }}>
                    {filteredKind === "designs" && <LayoutIcon size={16} />}
                    {filteredKind === "types" && <TypeIcon size={16} />}
                    {filteredKind === "qualities" && <AwardIcon size={16} />}
                    {filteredKind === "files" && <DocumentIcon size={16} />}
                    {filteredKind === "authors" && <UserIcon size={16} />}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                {filteredName !== null && (
                  <>
                    <BreadcrumbItem id="semio.sketchpad.navbar.name">
                      <BreadcrumbLink
                        onClick={() => {
                          const firstMatchingDesign = (kit?.designs as any[])?.find((d: any) => d.name === filteredName);
                          if (firstMatchingDesign) {
                            navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&select=${firstMatchingDesign.guid}`);
                          }
                        }}
                        style={{ cursor: "pointer" }}
                        id={"semio.sketchpad.navbar.name"}
                      >
                        {filteredName}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                  </>
                )}
              </>
            )}
          </>
        )}
        {isDesignApp && design && (
          <>
            <BreadcrumbItem id="semio.sketchpad.navbar.breadcrumb.designs" items={artifactKinds} onNavigate={(href) => navigate(href)}>
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=designs`)} style={{ cursor: "pointer" }}>
                <LayoutIcon size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            {designFolderChain.map((folder, index) => (
              <Fragment key={folder.guid}>
                <BreadcrumbItem id={`semio.sketchpad.navbar.folder.${folder.guid}`}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=folders`)} style={{ cursor: "pointer" }}>
                    {folder.name}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </Fragment>
            ))}
            <BreadcrumbItem
              id="semio.sketchpad.navbar.design"
              items={designNameItems}
              onNavigate={(href) => {
                if (href === "#create-design") handleCreateDesign("semio.sketchpad.navbar.selectDesign");
                else navigate(href);
              }}
            >
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  if (design && typeof design === "object" && "name" in design && "guid" in design) {
                    const designObj = design as Design;
                    navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(designObj.name)}&select=${designObj.guid}`);
                  }
                }}
              >
                <button type="button">{design && typeof design === "object" && "name" in design ? String((design as Design).name) : ""}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            {designParentChain.map((parent, index) => {
              const siblingItems = designParentSiblingItems.find((s) => s.parentGuid === parent.guid)?.items || [];
              return (
                <Fragment key={parent.guid}>
                  <BreadcrumbItem
                    id={`semio.sketchpad.navbar.design.parent.${parent.guid}`}
                    items={siblingItems}
                    onNavigate={(href) => {
                      if (href.startsWith("#create-sibling-")) {
                        handleCreateChild(`semio.sketchpad.navbar.selectDesignSibling.${parent.guid}`, parent, false);
                      } else {
                        navigate(href);
                      }
                    }}
                  >
                    <BreadcrumbLink
                      asChild
                      onClick={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        navigate(`/kits/${kitGuid}/designs/${parent.guid}`);
                      }}
                    >
                      <button type="button">{parent.name}</button>
                    </BreadcrumbLink>
                  </BreadcrumbItem>
                </Fragment>
              );
            })}
            {designChildItems.length > 1 && (
              <BreadcrumbItem
                id="semio.sketchpad.navbar.selectChild"
                items={designChildItems}
                onNavigate={(href) => {
                  if (href === "#create-child" && design && typeof design === "object" && "guid" in design) {
                    handleCreateChild("semio.sketchpad.navbar.selectChild", design as Design, false);
                  } else navigate(href);
                }}
              >
                <BreadcrumbLink style={{ cursor: "default" }}>{/* Empty for dropdown trigger */}</BreadcrumbLink>
              </BreadcrumbItem>
            )}
          </>
        )}
        {isTypeApp && type && (
          <>
            <BreadcrumbItem id="semio.sketchpad.navbar.breadcrumb.types" items={artifactKinds} onNavigate={(href) => navigate(href)}>
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=types`)} style={{ cursor: "pointer" }}>
                <TypeIcon size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            {typeFolderChain.map((folder, index) => (
              <Fragment key={folder.guid}>
                <BreadcrumbItem id={`semio.sketchpad.navbar.folder.${folder.guid}`}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=folders`)} style={{ cursor: "pointer" }}>
                    {folder.name}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </Fragment>
            ))}
            <BreadcrumbItem
              id="semio.sketchpad.navbar.type"
              items={typeNameItems}
              onNavigate={(href) => {
                if (href === "#create-type") handleCreateType("semio.sketchpad.navbar.selectType");
                else navigate(href);
              }}
            >
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  if (type && typeof type === "object" && "name" in type && "guid" in type) {
                    const typeObj = type as Type;
                    navigate(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(typeObj.name)}&select=${typeObj.guid}`);
                  }
                }}
              >
                <button type="button">{type && typeof type === "object" && "name" in type ? String((type as Type).name) : ""}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            {typeParentChain.map((parent, index) => {
              const siblingItems = typeParentSiblingItems.find((s) => s.parentGuid === parent.guid)?.items || [];
              return (
                <Fragment key={parent.guid}>
                  <BreadcrumbItem
                    id={`semio.sketchpad.navbar.type.parent.${parent.guid}`}
                    items={siblingItems}
                    onNavigate={(href) => {
                      if (href.startsWith("#create-sibling-")) {
                        handleCreateChild(`semio.sketchpad.navbar.selectTypeSibling.${parent.guid}`, parent, true);
                      } else {
                        navigate(href);
                      }
                    }}
                  >
                    <BreadcrumbLink
                      asChild
                      onClick={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        navigate(`/kits/${kitGuid}/types/${parent.guid}`);
                      }}
                    >
                      <button type="button">{parent.name}</button>
                    </BreadcrumbLink>
                  </BreadcrumbItem>
                </Fragment>
              );
            })}
            <BreadcrumbItem
              id="semio.sketchpad.navbar.selectChild"
              items={typeChildItems}
              onNavigate={(href) => {
                if (href === "#create-child" && type && typeof type === "object" && "guid" in type) {
                  handleCreateChild("semio.sketchpad.navbar.selectChild", type as Type, true);
                } else navigate(href);
              }}
            >
              <BreadcrumbLink style={{ cursor: "default" }}>{/* Empty for dropdown trigger */}</BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {isQualityApp && quality && (
          <>
            <BreadcrumbItem id="semio.sketchpad.navbar.breadcrumb.qualities" items={artifactKinds} onNavigate={(href) => navigate(href)}>
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=qualities`)} style={{ cursor: "pointer" }}>
                <AwardIcon size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbItem id="semio.sketchpad.navbar.quality">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=qualities&key=${encodeURIComponent(quality.key)}&select=${quality.guid}`);
                }}
              >
                <button type="button">{quality.name}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {isDocsPath && (
          <>
            <BreadcrumbItem
              id="semio.sketchpad.navbar.docs"
              items={
                docsSection
                  ? docsSectionsList.map((s) => ({
                      label: (
                        <span className="flex items-center gap-single">
                          {s.icon && <span aria-hidden="true">{s.icon}</span>}
                          <span>{s.label}</span>
                        </span>
                      ),
                      href: `/docs/${s.id}`,
                    }))
                  : undefined
              }
              onNavigate={docsSection ? (href) => navigate(href) : undefined}
            >
              <BreadcrumbLink onClick={() => navigate("/docs")} style={{ cursor: "pointer" }}>
                <DocumentIcon size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            {docsSection && (
              <>
                <BreadcrumbItem>
                  <BreadcrumbLink onClick={() => navigate(`/docs/${docsSection}`)} style={{ cursor: "pointer" }}>
                    {(() => {
                      const sectionInfo = docsSectionsList.find((s) => s.id === docsSection);
                      if (!sectionInfo) return docsSection;
                      return (
                        <span className="flex items-center gap-single">
                          {sectionInfo.icon && <span aria-hidden="true">{sectionInfo.icon}</span>}
                          <span>{sectionInfo.label}</span>
                        </span>
                      );
                    })()}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
            {docsPagePath &&
              docsSection &&
              (() => {
                const pathAfterSection = docsPagePath.split("/").slice(1);
                const sectionPages = getDocsRegistry()
                  .getAllPages()
                  .filter((page) => page.section === docsSection);
                const breadcrumbItems: React.ReactElement[] = [];

                pathAfterSection.forEach((part, index) => {
                  const isLast = index === pathAfterSection.length - 1;
                  const partialParts = pathAfterSection.slice(0, index + 1);
                  const partialPath = `docs/${docsSection}/${partialParts.join("/")}`;
                  const parentParts = pathAfterSection.slice(0, index);
                  const siblings = sectionPages
                    .filter((page) => {
                      const segments = page.path.replace(/^docs\//, "").split("/");
                      const trimmedSegments = segments[segments.length - 1] === "index" ? segments.slice(0, -1) : segments;
                      if (trimmedSegments[0] !== docsSection) return false;
                      const relative = trimmedSegments.slice(1);
                      if (relative.length !== parentParts.length + 1) return false;
                      for (let i = 0; i < parentParts.length; i++) {
                        if (relative[i] !== parentParts[i]) return false;
                      }
                      return true;
                    })
                    .sort((a, b) => {
                      const orderDiff = (a.order ?? 999) - (b.order ?? 999);
                      if (orderDiff !== 0) return orderDiff;
                      return a.title.localeCompare(b.title);
                    });
                  const separatorItems = siblings.map((page) => ({
                    label: page.title,
                    href: `/${page.path.replace(/\/index$/, "")}`,
                  }));
                  const normalizedPartial = `${docsSection}/${partialParts.join("/")}`;
                  const match = siblings.find((page) => page.path.replace(/^docs\//, "").replace(/\/index$/, "") === normalizedPartial) || sectionPages.find((page) => page.path.replace(/^docs\//, "").replace(/\/index$/, "") === normalizedPartial);
                  const label = match?.title
                    ? match.title
                    : part
                        .split("-")
                        .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
                        .join(" ");

                  breadcrumbItems.push(
                    <BreadcrumbItem key={partialPath} items={!isLast ? separatorItems : undefined} onNavigate={!isLast ? (href) => navigate(href) : undefined}>
                      <BreadcrumbLink onClick={() => !isLast && navigate(`/${partialPath}`)} style={{ cursor: isLast ? "default" : "pointer" }}>
                        {label}
                      </BreadcrumbLink>
                    </BreadcrumbItem>,
                  );
                });

                return <>{breadcrumbItems}</>;
              })()}
          </>
        )}
      </BreadcrumbList>
    </Breadcrumb>
  );
};

type SearchResult = {
  type: "kit" | "design" | "type" | "quality" | "docs" | "tutorial";
  item: KitShallow | DesignShallow | TypeShallow | Quality | { title: string; description?: string; path: string } | { id: string; name: string; description?: string };
  kitGuid?: string;
};

const buildSearchResultPath = (result: SearchResult): string => {
  if (result.type === "kit") return `/kits/${(result.item as KitShallow).guid}`;
  if (result.type === "design") return `/kits/${result.kitGuid}/designs/${(result.item as DesignShallow).guid}`;
  if (result.type === "type") return `/kits/${result.kitGuid}/types/${(result.item as TypeShallow).guid}`;
  if (result.type === "quality") return `/kits/${result.kitGuid}?kind=qualities&select=${(result.item as Quality).guid}`;
  if (result.type === "docs") return `/${(result.item as { path: string }).path}`;
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
    const docsPages = getDocsRegistry().getAllPages();
    docsPages.forEach((page) => {
      results.push({ type: "docs", item: page });
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
    // Show all recent results without limit, or fallback to first 20 if no recent results
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
      docs: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "docs"),
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
        else if (type === "docs") navigate(`/${(item as { path: string }).path}`);
      }
    },
    [navigate, recentSearches, updateRecentSearches, tutorialStore],
  );

  const getIcon = (type: SearchResult["type"]) => {
    if (type === "kit") return <LocalKitIcon size={16} />;
    if (type === "design") return <LayoutIcon size={16} />;
    if (type === "type") return <TypeIcon size={16} />;
    if (type === "quality") return <AwardIcon size={16} />;
    if (type === "docs") return <DocumentIcon size={16} />;
    if (type === "tutorial") return <TutorialIcon size={16} />;
    return null;
  };

  const getDisplayName = (result: SearchResult) => {
    const { type, item } = result;
    if (type === "quality") return (item as Quality).name;
    if (type === "docs") return (item as { title: string }).title;
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
  const docsLabel = useLabel("semio.sketchpad.navbar.docs", "Documentation");
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
          {groupedSearchResults.docs.length > 0 && (
            <CommandGroup heading={docsLabel}>
              {groupedSearchResults.docs.map((r: FuseResult<SearchResult>, idx: number) => (
                <CommandItem key={`docs-${(r.item.item as { path: string }).path}-${idx}`} onSelect={() => handleSelect(r.item)}>
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
    // Show all recent focus items without limit, or fallback to first 20 if no recent items
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
  const panelConfigs = usePanelConfigs();
  const panelConfig = panelConfigs[appType];

  if (!panelConfig || panelConfig.length === 0) return null;

  const visiblePanels = useAppPanelVisibility();
  const appCommands = useAppCommands();

  const commands = useMemo<Record<string, any>>(() => {
    return {
      home: appCommands,
      kit: appCommands,
      design: appCommands,
      type: appCommands,
      quality: appCommands,
      docs: appCommands,
    };
  }, [appCommands]);
  const isMobile = useIsMobile();

  const groupedPanels = useMemo(() => {
    const groups: Record<string, EnrichedPanelDefinition[]> = {};
    const ungrouped: EnrichedPanelDefinition[] = [];

    panelConfig.forEach((config) => {
      if (config.group) {
        if (!groups[config.group]) {
          groups[config.group] = [];
        }
        groups[config.group].push(config);
      } else {
        ungrouped.push(config);
      }
    });

    return { groups, ungrouped };
  }, [panelConfig]);

  const workbenchConfigs = groupedPanels.groups["workbench"] || [];
  const hudConfigs = groupedPanels.groups["hud"] || [];
  const rightConfigs = groupedPanels.groups["right"] || [];
  const toolbarConfigs = panelConfig.filter((p) => p.kind === PanelKind.TOOLBAR);

  const workbenchDefaultKey = workbenchConfigs[0]?.key || "";
  const workbenchSelectionRef = useRef<string>(workbenchDefaultKey);
  if (!workbenchConfigs.some((config) => config.key === workbenchSelectionRef.current)) {
    workbenchSelectionRef.current = workbenchDefaultKey;
  }
  const openWorkbenchPanelKey = workbenchConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyWorkbenchPanelOpen = Boolean(openWorkbenchPanelKey);
  if (openWorkbenchPanelKey && workbenchSelectionRef.current !== openWorkbenchPanelKey) {
    workbenchSelectionRef.current = openWorkbenchPanelKey;
  }
  const activeWorkbenchPanel = workbenchSelectionRef.current || workbenchDefaultKey;

  const hudDefaultKey = hudConfigs[0]?.key || "";
  const hudSelectionRef = useRef<string>(hudDefaultKey);
  if (!hudConfigs.some((config) => config.key === hudSelectionRef.current)) {
    hudSelectionRef.current = hudDefaultKey;
  }
  const openHudPanelKey = hudConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyHudPanelOpen = Boolean(openHudPanelKey);
  if (openHudPanelKey && hudSelectionRef.current !== openHudPanelKey) {
    hudSelectionRef.current = openHudPanelKey;
  }
  const activeHudPanel = hudSelectionRef.current || hudDefaultKey;

  const rightDefaultKey = rightConfigs[0]?.key || "";
  const rightSelectionRef = useRef<string>(rightDefaultKey);
  if (!rightConfigs.some((config) => config.key === rightSelectionRef.current)) {
    rightSelectionRef.current = rightDefaultKey;
  }
  const openRightPanelKey = rightConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyRightPanelOpen = Boolean(openRightPanelKey);
  if (openRightPanelKey && rightSelectionRef.current !== openRightPanelKey) {
    rightSelectionRef.current = openRightPanelKey;
  }
  const activeRightPanel = rightSelectionRef.current || rightDefaultKey;

  const workbenchPanelKeys = workbenchConfigs.map((c) => c.key);
  const hudPanelKeys = hudConfigs.map((c) => c.key);
  const rightPanelKeys = rightConfigs.map((c) => c.key);

  const panelToggleTooltip = (panelKey: string, open: boolean) => (panelKey ? `semio.sketchpad.navbar.panelToggle.${panelKey}.${open ? "hide" : "show"}` : undefined);
  const rightDropdownAriaLabel = `semio.sketchpad.navbar.panelToggle.right.label`;

  const handleToggle = useCallback(
    (origin: string, panelKey: keyof PanelVisibility) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      const current = visiblePanels[panelKey];

      if (isMobile) {
        if (!current) {
          (Object.keys(visiblePanels) as Array<keyof PanelVisibility>).forEach((p) => {
            if (p !== panelKey && visiblePanels[p]) {
              togglePanel(origin, p);
            }
          });
        }
      } else {
        const config = panelConfig.find((c) => c.key === panelKey);
        if (config?.group) {
          const groupConfigs = groupedPanels.groups[config.group] || [];
          const groupKeys = groupConfigs.map((c) => c.key);
          if (!current) {
            (groupKeys as Array<keyof PanelVisibility>).forEach((p) => {
              if (p !== panelKey && visiblePanels[p]) {
                togglePanel(origin, p);
              }
            });
          }
        }
      }
      togglePanel(origin, panelKey);
    },
    [appType, commands, visiblePanels, isMobile, panelConfig, groupedPanels],
  );

  const handleWorkbenchPressedChange = useCallback(
    (origin: string, pressed: boolean) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (pressed) {
        if (activeWorkbenchPanel && !visiblePanels[activeWorkbenchPanel as keyof PanelVisibility]) {
          handleToggle(origin, activeWorkbenchPanel as keyof PanelVisibility);
        }
      } else {
        const openPanel = workbenchConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
        if (openPanel) {
          togglePanel(origin, openPanel.key as keyof PanelVisibility);
        }
      }
    },
    [appType, commands, visiblePanels, activeWorkbenchPanel, workbenchConfigs, handleToggle],
  );

  const handleWorkbenchValueChange = useCallback(
    (origin: string, value: string | undefined) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (!value) return;
      workbenchSelectionRef.current = value;

      (workbenchPanelKeys as Array<keyof PanelVisibility>).forEach((p) => {
        const isOpen = visiblePanels[p];
        const shouldOpen = p === value;

        if (isOpen && !shouldOpen) {
          togglePanel(origin, p);
        } else if (!isOpen && shouldOpen) {
          togglePanel(origin, p);
        }
      });
    },
    [appType, commands, visiblePanels, workbenchPanelKeys],
  );

  const handleHudPressedChange = useCallback(
    (origin: string, pressed: boolean) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (pressed) {
        if (activeHudPanel && !visiblePanels[activeHudPanel as keyof PanelVisibility]) {
          handleToggle(origin, activeHudPanel as keyof PanelVisibility);
        }
      } else {
        const openPanel = hudConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
        if (openPanel) {
          togglePanel(origin, openPanel.key as keyof PanelVisibility);
        }
      }
    },
    [appType, commands, visiblePanels, activeHudPanel, hudConfigs, handleToggle],
  );

  const handleHudValueChange = useCallback(
    (origin: string, value: string | undefined) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (!value) return;
      hudSelectionRef.current = value;

      (hudPanelKeys as Array<keyof PanelVisibility>).forEach((p) => {
        const isOpen = visiblePanels[p];
        const shouldOpen = p === value;

        if (isOpen && !shouldOpen) {
          togglePanel(origin, p);
        } else if (!isOpen && shouldOpen) {
          togglePanel(origin, p);
        }
      });
    },
    [appType, commands, visiblePanels, hudPanelKeys],
  );

  const handleRightPressedChange = useCallback(
    (origin: string, pressed: boolean) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (pressed) {
        if (activeRightPanel && !visiblePanels[activeRightPanel as keyof PanelVisibility]) {
          handleToggle(origin, activeRightPanel as keyof PanelVisibility);
        }
      } else {
        const openPanel = rightConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
        if (openPanel) {
          togglePanel(origin, openPanel.key as keyof PanelVisibility);
        }
      }
    },
    [appType, commands, visiblePanels, activeRightPanel, rightConfigs, handleToggle],
  );

  const handleRightValueChange = useCallback(
    (origin: string, value: string | undefined) => {
      const togglePanel = commands[appType]?.togglePanel || (() => {});
      if (!value) return;
      rightSelectionRef.current = value;

      (rightPanelKeys as Array<keyof PanelVisibility>).forEach((p) => {
        const isOpen = visiblePanels[p];
        const shouldOpen = p === value;

        if (isOpen && !shouldOpen) {
          togglePanel(origin, p);
        } else if (!isOpen && shouldOpen) {
          togglePanel(origin, p);
        }
      });
    },
    [appType, commands, visiblePanels, rightPanelKeys],
  );

  const workbenchItems = workbenchConfigs.map((config) => {
    const Icon = config.icon;
    return {
      value: config.key,
      label: Icon ? <Icon size={16} /> : undefined,
      id: `semio.sketchpad.navbar.panelToggle.${config.key}.show`,
    };
  });

  const activeWorkbenchConfig = workbenchConfigs.find((c) => c.key === activeWorkbenchPanel);
  const ActiveWorkbenchIcon = activeWorkbenchConfig?.icon;

  const hudItems = hudConfigs.map((config) => {
    const Icon = config.icon;
    return {
      value: config.key,
      label: Icon ? <Icon size={16} /> : undefined,
      id: `semio.sketchpad.navbar.panelToggle.${config.key}.show`,
    };
  });

  const activeHudConfig = hudConfigs.find((c) => c.key === activeHudPanel);
  const ActiveHudIcon = activeHudConfig?.icon;

  const rightItems = rightConfigs.map((config) => {
    const Icon = config.icon;
    return {
      value: config.key,
      label: Icon ? <Icon size={16} /> : undefined,
      id: `semio.sketchpad.navbar.panelToggle.${config.key}.show`,
    };
  });

  const activeRightConfig = rightConfigs.find((c) => c.key === activeRightPanel);
  const ActiveRightIcon = activeRightConfig?.icon;

  return (
    <div className="flex items-stretch">
      {workbenchConfigs.length > 0 && (
        <Toggle
          kind="dropdown"
          id="semio.sketchpad.navbar.panelToggle.workbench"
          items={workbenchItems}
          value={activeWorkbenchPanel}
          onValueChange={(value) => handleWorkbenchValueChange("semio.sketchpad.navbar.panelToggle.workbench", value)}
          pressed={isAnyWorkbenchPanelOpen}
          onPressedChange={(pressed) => handleWorkbenchPressedChange("semio.sketchpad.navbar.panelToggle.workbench", pressed)}
        />
      )}
      {hudConfigs.length > 0 && (
        <Toggle
          kind="dropdown"
          id="semio.sketchpad.navbar.panelToggle.hud"
          items={hudItems}
          value={activeHudPanel}
          onValueChange={(value) => handleHudValueChange("semio.sketchpad.navbar.panelToggle.hud", value)}
          pressed={isAnyHudPanelOpen}
          onPressedChange={(pressed) => handleHudPressedChange("semio.sketchpad.navbar.panelToggle.hud", pressed)}
        />
      )}
      {rightConfigs.length > 0 && (
        <Toggle
          kind="dropdown"
          id="semio.sketchpad.navbar.panelToggle.right"
          items={rightItems}
          value={activeRightPanel}
          onValueChange={(value) => handleRightValueChange("semio.sketchpad.navbar.panelToggle.right", value)}
          pressed={isAnyRightPanelOpen}
          onPressedChange={(pressed) => handleRightPressedChange("semio.sketchpad.navbar.panelToggle.right", pressed)}
        />
      )}
    </div>
  );
};

// #region Canvas

export interface WindowControl {
  kind: "toggle" | "dropdown";
  id: string;
  icon?: ReactNode;
  value?: string;
  options?: {
    id: string;
    value: string;
    icon?: ReactNode;
  }[];
  onChange?: (value: string) => void;
}

export interface WindowKindDefinition {
  id: string;
  label?: string | any;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
  controls?: WindowControl[];
  variants?: {
    id: string;
    icon?: ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

export interface AppWindowConfig {
  windowKinds: WindowKindDefinition[];
  defaultLayout?: any;
}

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

export const Canvas: FC<{ children: ReactNode }> = ({ children }) => {
  return <div className="h-full w-full">{children}</div>;
};

export const HorizontalWindows: FC<{ children: ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full">{children}</div>;
};

export const VerticalWindows: FC<{ children: ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full">{children}</div>;
};

export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[]): any {
  return {
    type: direction === "row" ? "row" : "column",
    content: windowIds.map((id, index) => ({
      type: "component",
      componentName: id,
      componentState: {},
      ...(sizes && sizes[index] !== undefined ? { size: `${sizes[index]}%` } : {}),
    })),
  };
}

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
    if (sketchpadScope) {
      wrapped = <SketchpadScopeContext.Provider value={sketchpadScope}>{wrapped}</SketchpadScopeContext.Provider>;
    }

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

        const normalizeLayoutConfig = (config: any, depth: number = 0, path: string = "root"): any => {
          if (!config || typeof config !== "object") return config;
          if (Array.isArray(config)) {
            return config.map((item, idx) => normalizeLayoutConfig(item, depth, `${path}[${idx}]`));
          }

          const normalized: any = {};
          const indent = "  ".repeat(depth);

          console.log(`${indent}[Normalize ${path}] Type: ${config.type || "unknown"}`);

          for (const [key, value] of Object.entries(config)) {
            console.log(`${indent}  Processing key: ${key}, value type: ${typeof value}, value:`, value);

            // Handle size/width/height - these need special handling based on whether a unit field exists
            if (key === "size" || key === "width" || key === "height") {
              const unitKey = `${key}Unit` as string;
              const hasUnitField = unitKey in config;
              console.log(`${indent}    ${key}: hasUnitField=${hasUnitField}, unitKey=${unitKey}, unitValue=${config[unitKey]}`);

              if (hasUnitField) {
                // When a unit field exists, the size should be a number
                if (typeof value === "string") {
                  // Convert string to number
                  const numValue = parseFloat(value);
                  if (!isNaN(numValue)) {
                    console.log(`${indent}    ${key}: Converting string "${value}" to number ${numValue}`);
                    normalized[key] = numValue;
                  } else {
                    console.log(`${indent}    ${key}: Failed to parse string "${value}" as number, using default 1`);
                    normalized[key] = 1;
                  }
                } else if (typeof value === "number") {
                  // Keep as number
                  console.log(`${indent}    ${key}: Keeping as number ${value}`);
                  normalized[key] = value;
                } else if (value === null || value === undefined) {
                  // Provide default value when null/undefined
                  console.log(`${indent}    ${key}: null/undefined with unit field, defaulting to 1`);
                  normalized[key] = 1;
                } else {
                  console.log(`${indent}    ${key}: Unexpected type with unit field, defaulting to 1:`, typeof value, value);
                  normalized[key] = 1;
                }
              } else {
                // When no unit field exists, the size should be a string with unit
                if (typeof value === "string") {
                  if (value.trim() === "") {
                    if (key === "size") {
                      console.log(`${indent}    ${key}: Empty string, defaulting to 50%`);
                      normalized[key] = "50%";
                    }
                  } else {
                    console.log(`${indent}    ${key}: Keeping string "${value}"`);
                    normalized[key] = value;
                  }
                } else if (typeof value === "number") {
                  // Add % suffix for numbers without unit field
                  console.log(`${indent}    ${key}: Converting number ${value} to string "${value}%"`);
                  normalized[key] = `${value}%`;
                } else if (value === null || value === undefined) {
                  // Provide default value when null/undefined
                  console.log(`${indent}    ${key}: null/undefined without unit field, defaulting to 50%`);
                  normalized[key] = "50%";
                } else {
                  console.log(`${indent}    ${key}: Unexpected type without unit field, defaulting to 50%:`, typeof value, value);
                  normalized[key] = "50%";
                }
              }
            } else if (key === "title" || key === "componentName" || key === "componentType" || key === "type" || key === "id") {
              // These should be strings
              if (typeof value === "string") {
                if (value.trim() === "") {
                  console.log(`${indent}    ${key}: Empty string, skipping`);
                  // Skip empty strings
                  continue;
                } else {
                  console.log(`${indent}    ${key}: Keeping string "${value}"`);
                  normalized[key] = value;
                }
              } else if (value !== null && value !== undefined) {
                console.log(`${indent}    ${key}: Converting ${typeof value} to string "${String(value)}"`);
                normalized[key] = String(value);
              }
            } else if (key === "content" && Array.isArray(value)) {
              // Only include content if it's not empty, or if this is not a component type
              if (value.length > 0 || config.type !== "component") {
                console.log(`${indent}    content: Processing array of ${value.length} items`);
                normalized[key] = value.map((item, idx) => normalizeLayoutConfig(item, depth + 1, `${path}.content[${idx}]`));
              } else {
                console.log(`${indent}    content: Skipping empty content array for component`);
              }
            } else if (key === "componentState") {
              console.log(`${indent}    componentState: Passing through as-is`);
              // componentState should be passed through as-is (it's data, not layout config)
              normalized[key] = value;
            } else if (typeof value === "object" && value !== null) {
              console.log(`${indent}    ${key}: Recursing into nested object`);
              // Recursively normalize other objects
              normalized[key] = normalizeLayoutConfig(value, depth + 1, `${path}.${key}`);
            } else {
              console.log(`${indent}    ${key}: Passing through ${typeof value} value:`, value);
              normalized[key] = value;
            }
          }

          console.log(`${indent}[Normalize ${path}] Result keys:`, Object.keys(normalized));
          return normalized;
        };

        console.log("[GoldenLayout] Raw config:", JSON.stringify(layoutState || windowConfig.defaultLayout, null, 2));
        const rawConfig = layoutState || windowConfig.defaultLayout || createDefaultLayout(windowConfig.windowKinds.map((wt) => wt.id));
        const config = normalizeLayoutConfig(rawConfig);

        console.log("[GoldenLayout] Normalized config:", JSON.stringify(config, null, 2));

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
              return (
                <MemoryRouter initialEntries={[location.pathname + location.search]} initialIndex={0}>
                  <LayoutScopeWrapper>
                    <ReactFlowProvider>
                      <Window id={windowType.id} isVisible={true} controls={windowType.controls ? <WindowControlsGroup controls={windowType.controls} /> : undefined}>
                        <WindowComponent />
                      </Window>
                    </ReactFlowProvider>
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
            <div data-splitter-buttons className="pointer-events-auto absolute left-1/2 top-1/2 flex flex-row -translate-x-1/2 -translate-y-1/2 gap-single border border-border bg-temporary p-single">
              {windowConfig.windowKinds.map((windowType) => {
                const typeId = windowType.id;
                const direction = hoveredSplitter.direction;
                const splitterElement = hoveredSplitter.element;
                if (!splitterElement) {
                  return null;
                }
                return (
                  <button
                    key={typeId}
                    type="button"
                    disabled={!layoutLoaded}
                    className="border border-border bg-panel p-single text-xs hover:bg-hover-panel disabled:opacity-50 disabled:cursor-not-allowed"
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
                  >
                    {typeof windowType.label === "string" ? windowType.label : typeId}
                  </button>
                );
              })}
            </div>,
            hoveredSplitter.element,
          )}
      </div>
    </CanvasContext.Provider>
  );
};

// #endregion Canvas

// #region App Router

const ScopeWrapper: FC<{ ScopeProvider: ComponentType<{ guid: string; children: ReactNode }>; paramName: string; children: ReactNode }> = ({ ScopeProvider, paramName, children }) => {
  const params = useParams();
  const guid = params[paramName];
  if (!guid) return <>{children}</>;
  return <ScopeProvider guid={guid}>{children}</ScopeProvider>;
};

const AppRouter: FC = () => {
  const [appsInitialized, setAppsInitialized] = useState(false);

  useEffect(() => {
    appRegistry.initialize().then(() => setAppsInitialized(true));
  }, []);

  const apps = useMemo(() => {
    if (!appsInitialized) return [];
    return appRegistry.getAllApps();
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

// #endregion App Router

// #region Sketchpad

const LayoutWrapper: FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const reactNavigate = useReactNavigate();
  const store = useSketchpadStore();
  const tutorialStore = store.tutorialStore();

  const navigation = useNavigation();
  const isFullscreen = useIsFullscreen();
  const isNavbarExpanded = useIsNavbarExpanded();
  const isFooterExpanded = useIsFooterExpanded();
  const panelVisibility = useAppPanelVisibility();
  const panelSizes = usePanelSizes();
  const footerItems = useFooterItems();
  const workbenchSections = usePanelSections("workbench");
  const toolsSections = usePanelSections("tools");
  const hudSections = usePanelSections("hud");
  const statsSections = usePanelSections("stats");
  const detailsSections = usePanelSections("details");
  const chatSections = usePanelSections("chat");
  const settingsSections = usePanelSections("settings");
  const consoleSections = usePanelSections("console");

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
      sketchpadCommands.syncNavigation("semio.sketchpad.sync", fullPath);
    }
  }, [location.pathname, location.search, sketchpadCommands, store]);

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
    const leftItems: NavbarItem[] = [];
    const centerItems: NavbarItem[] = [];
    const rightItems: NavbarItem[] = [];

    leftItems.push({
      id: "semio.sketchpad.navbar.navigationButtons",
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
      order: 0,
    });

    centerItems.push({
      id: "semio.sketchpad.navbar.navigation",
      content: <Navigation />,
      className: "flex-1 min-w-0",
      order: 0,
    });

    rightItems.push({
      id: "semio.sketchpad.navbar.search",
      content: <Search />,
      order: 0,
    });

    rightItems.push({
      id: "semio.sketchpad.navbar.fullscreenToggle",
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
                    (result as Promise<void>).then(() => sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: true })).catch(() => sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false }));
                  } else {
                    sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: true });
                  }
                } else {
                  sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false });
                }
              } else {
                if (document.fullscreenElement && document.exitFullscreen) {
                  const result = document.exitFullscreen();
                  if (result && typeof (result as any).then === "function") {
                    (result as Promise<void>).then(() => sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false })).catch(() => sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false }));
                  } else {
                    sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false });
                  }
                } else {
                  sketchpadCommands.setState(fullscreenToggleId, { isFullscreen: false });
                }
              }
              return;
            }
            sketchpadCommands.toggleFullscreen(fullscreenToggleId);
          }}
          icon={isFullscreen ? <Minimize2Icon size={16} /> : <Maximize2Icon size={16} />}
        />
      ),
      order: 1,
    });

    rightItems.push({
      id: "semio.sketchpad.navbar.focus",
      content: <Focus />,
      order: 2,
    });

    rightItems.push({
      id: "semio.sketchpad.navbar.panelToggles",
      content: <PanelToggles />,
      order: 3,
    });

    return { leftItems, centerItems, rightItems };
  }, [navigationHistory, upTarget, isAtRoot, navigate, sketchpadCommands, isFullscreen]);

  return (
    <TutorialProvider store={tutorialStore}>
      <LayoutComponent
        className="bg-base text-foreground relative border"
        navbar={<Navbar leftItems={navbarItems.leftItems} centerItems={navbarItems.centerItems} rightItems={navbarItems.rightItems} isExpanded={isNavbarExpanded} />}
        footer={
          !isFullscreen || isFooterExpanded ? (
            <Footer
              items={footerItems.map((item) => ({
                id: item.id,
                content: item.content,
                order: item.order,
                onClick: item.onClick,
              }))}
              isVisible={isFooterExpanded || !isFullscreen}
            />
          ) : undefined
        }
        leftPanel={
          panelVisibility.workbench || panelVisibility.tools
            ? {
                visible: panelVisibility.workbench || panelVisibility.tools,
                size: panelVisibility.workbench ? panelSizes.workbenchWidth : panelSizes.toolsWidth,
                onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.workbench ? "workbenchWidth" : "toolsWidth", size),
                sections: panelVisibility.workbench ? workbenchSections : toolsSections,
              }
            : undefined
        }
        middlePanel={
          panelVisibility.hud || panelVisibility.stats
            ? {
                visible: panelVisibility.hud || panelVisibility.stats,
                size: panelVisibility.hud ? panelSizes.hudWidth : panelSizes.statsWidth,
                onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.hud ? "hudWidth" : "statsWidth", size),
                sections: panelVisibility.hud ? hudSections : statsSections,
              }
            : undefined
        }
        rightPanel={
          panelVisibility.details || panelVisibility.chat || panelVisibility.settings
            ? {
                visible: panelVisibility.details || panelVisibility.chat || panelVisibility.settings,
                size: panelVisibility.details ? panelSizes.detailsWidth : panelVisibility.chat ? panelSizes.chatWidth : panelSizes.settingsWidth,
                onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", panelVisibility.details ? "detailsWidth" : panelVisibility.chat ? "chatWidth" : "settingsWidth", size),
                sections: panelVisibility.details ? detailsSections : panelVisibility.chat ? chatSections : settingsSections,
              }
            : undefined
        }
        bottomPanel={
          panelVisibility.chat
            ? {
                visible: panelVisibility.chat,
                size: panelSizes.consoleHeight,
                onSizeChange: (size: number) => sketchpadCommands.setPanelSize("semio.sketchpad", "consoleHeight", size),
                sections: consoleSections,
              }
            : undefined
        }
        canvas={<AppRouter />}
      />
    </TutorialProvider>
  );
};

// Bridge component that connects Sketchpad interaction system to generic InteractionProvider
const SketchpadInteractionBridge: FC<{ children: React.ReactNode }> = ({ children }) => {
  const commands = useSketchpadCommands();
  const activeInteraction = useActiveInteraction();

  return (
    <InteractionProvider commands={commands} activeInteraction={activeInteraction}>
      {children}
    </InteractionProvider>
  );
};

const SketchpadContent: FC = () => {
  return <LayoutWrapper />;
};

const Sketchpad: FC<{ id?: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents; initialState?: ExtendedInitialState; embedded?: boolean }> = ({ id, remote, onWindowEvents, initialState, embedded }) => {
  const initialEntries = useMemo(() => {
    if (!embedded) return undefined;
    if (typeof window !== "undefined" && window.location) {
      return [window.location.pathname + window.location.search];
    }
    return ["/"];
  }, [embedded]);

  const routerContent = (
    <SketchpadScopeProvider id={id} remote={remote} onWindowEvents={onWindowEvents} initialState={initialState}>
      <SketchpadInteractionBridge>
        <FocusProvider>
          <PanelSectionProvider>
            <FooterItemProvider>
              <DragDropProvider>
                <SketchpadContent />
              </DragDropProvider>
            </FooterItemProvider>
          </PanelSectionProvider>
        </FocusProvider>
      </SketchpadInteractionBridge>
    </SketchpadScopeProvider>
  );

  if (embedded) {
    return <MemoryRouter initialEntries={initialEntries}>{routerContent}</MemoryRouter>;
  }

  return <BrowserRouter>{routerContent}</BrowserRouter>;
};

// #endregion Sketchpad

export { Window } from "./elements";

export { Sketchpad };
export default Sketchpad;

// #endregion Apps
