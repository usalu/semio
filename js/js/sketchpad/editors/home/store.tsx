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

import * as Y from "yjs";
import { guid, Guid } from "../../../semio";
import { createObserver, identitySelector, PanelVisibility, registerHomeStoreFactory, SketchpadStore, Subscribe, useSketchpadStore, useSyncDeep } from "../../store";

export interface HomeSelection {
  kits?: Guid[];
}

export interface HomeSelectionDiff {
  added?: Guid[];
  removed?: Guid[];
}

export type HomeSortColumn = "name" | "type" | "updatedAt" | "createdAt";
export type HomeSortDirection = "asc" | "desc";

export interface HomeState {
  panelVisibility: PanelVisibility;
  selection?: HomeSelection;
  sortColumn?: HomeSortColumn;
  sortDirection?: HomeSortDirection;
}

export interface HomeDiff {
  panelVisibility?: Partial<PanelVisibility>;
  selection?: HomeSelectionDiff;
  sortColumn?: HomeSortColumn;
  sortDirection?: HomeSortDirection;
}

export interface HomeCommandContext {
  home: HomeState;
}

export interface HomeCommandResult {
  diff?: HomeDiff;
}

export class HomeStore {
  public readonly guid: string;
  public readonly parent: SketchpadStore;
  public readonly yMap: Y.Map<any>;
  protected readonly commandRegistry: Map<string, (context: HomeCommandContext, ...rest: any[]) => HomeCommandResult> = new Map();
  protected readonly transact: (fn: () => void) => void;
  protected cache?: HomeState;
  protected cacheHash?: string;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) {
    this.guid = guid();
    this.parent = parent;
    this.yMap = yMap;
    this.transact = transact;
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

  get selection(): HomeSelection | undefined {
    const yKits = this.yMap.get("selectedKits") as Y.Array<string>;
    if (!yKits || yKits.length === 0) return undefined;
    return {
      kits: yKits.toArray(),
    };
  }

  get sortColumn(): HomeSortColumn | undefined {
    return this.yMap.get("sortColumn") as HomeSortColumn | undefined;
  }

  get sortDirection(): HomeSortDirection | undefined {
    return this.yMap.get("sortDirection") as HomeSortDirection | undefined;
  }

  protected hash(state: HomeState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): HomeState {
    return {
      panelVisibility: this.panelVisibility,
      selection: this.selection,
      sortColumn: this.sortColumn,
      sortDirection: this.sortDirection,
    };
  }

  snapshot = (): HomeState => {
    const currentData = this.buildSnapshot();
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: HomeDiff) => {
    this.transact(() => {
      if (diff.panelVisibility !== undefined) {
        let yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
        if (!yPanelVisibility) {
          yPanelVisibility = new Y.Map<boolean>();
          this.yMap.set("panelVisibility", yPanelVisibility);
        }
        yPanelVisibility.set("__editor", "HOME" as any);
        Object.entries(diff.panelVisibility).forEach(([key, value]) => {
          if (value !== undefined) {
            yPanelVisibility.set(key, value);
          }
        });
      }
      if (diff.selection !== undefined) {
        let yKits = this.yMap.get("selectedKits") as Y.Array<string>;
        if (!yKits) {
          yKits = new Y.Array<string>();
          this.yMap.set("selectedKits", yKits);
        }
        if (diff.selection.removed) {
          diff.selection.removed.forEach((Guid) => {
            const index = yKits.toArray().indexOf(Guid);
            if (index !== -1) {
              yKits.delete(index, 1);
            }
          });
        }
        if (diff.selection.added) {
          diff.selection.added.forEach((Guid) => {
            if (!yKits.toArray().includes(Guid)) {
              yKits.push([Guid]);
            }
          });
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

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yMap, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yMap, subscribe, true);
  };

  registerCommand(command: string, callback: (context: HomeCommandContext, ...rest: any[]) => HomeCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  register(command: string, callback: (context: HomeCommandContext, ...rest: any[]) => HomeCommandResult): Disposable {
    return this.registerCommand(command, callback);
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      throw new Error(`Command "${command}" not found`);
    }
    const state = this.snapshot();
    const context: HomeCommandContext = { home: state };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    return result as T;
  }

  async execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand<T>(command, ...rest);
  }
}

registerHomeStoreFactory((parent, yMap, transact) => new HomeStore(parent, yMap, transact));

function useHomeStore<T>(selector?: (store: HomeStore) => T): T | HomeStore {
  const store = useSketchpadStore();
  const homeStore = store.home();
  return selector ? selector(homeStore) : homeStore;
}

export function useHome<T>(selector?: (state: HomeState) => T): T | HomeState {
  return useSyncDeep<HomeState, T>(useHomeStore(identitySelector) as HomeStore, selector ? selector : identitySelector);
}

export function useHomePanelVisibility(): PanelVisibility {
  return useHome((s) => s.panelVisibility) as PanelVisibility;
}

export function useHomeCommands() {
  const store = useHomeStore() as HomeStore;
  return {
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    selectKit: (Guid: Guid) => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
          added: [Guid],
        },
      });
    },
    addKitToSelection: (Guid: Guid) => {
      store.change({
        selection: {
          added: [Guid],
        },
      });
    },
    removeKitFromSelection: (Guid: Guid) => {
      store.change({
        selection: {
          removed: [Guid],
        },
      });
    },
    selectKits: (kitIds: Guid[]) => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
          added: kitIds,
        },
      });
    },
    deselectAll: () => {
      const current = store.snapshot();
      store.change({
        selection: {
          removed: current.selection?.kits ?? [],
        },
      });
    },
    setSortColumn: (column: HomeSortColumn) => {
      store.change({
        sortColumn: column,
      });
    },
    setSortDirection: (direction: HomeSortDirection) => {
      store.change({
        sortDirection: direction,
      });
    },
    toggleSort: (column: HomeSortColumn) => {
      const current = store.snapshot();
      if (current.sortColumn === column) {
        store.change({
          sortDirection: current.sortDirection === "asc" ? "desc" : "asc",
        });
      } else {
        store.change({
          sortColumn: column,
          sortDirection: "asc",
        });
      }
    },
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}

// #endregion Home Store
