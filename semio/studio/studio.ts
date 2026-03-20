// #region 🔖Header

// [👤semio👥studio💻studio](repo://p/u/semio/b/l/studio/f/studio.ts)

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

// Specs: Defines abstract synchronizable state interfaces (SyncMap, SyncArray, SyncDoc) for
// backend-agnostic state management. Provides CRDT implementation and persistence factories.
// The abstraction separates the 'what' (synchronized kit data) from the 'how' (CRDT, JSON, SQLite).

// Synchronizable state layer and kit persistence providers for semio studio.

// #endregion Header

// #region 🔖SyncInterfaces

/**
 * Event emitted when fields change in a SyncMap.
 *
 * Specs: Backend-agnostic field change notification. `changedFields` contains
 * the set of field keys that were modified in the change.
 **/
export interface SyncMapEvent {
  keysChanged: Set<string>;
}

/**
 * A synchronized field store supporting key-value access and change observation.
 *
 * Specs: Provides field-level get/set/delete with shallow and deep change
 * observation. Multiple backends (CRDT, in-memory, JSON, SQLite) can implement
 * this interface to provide different synchronization strategies.
 **/
export interface SyncMap<V = any> {
  get(key: string): V | undefined;
  set(key: string, value: V): void;
  delete(key: string): void;
  has(key: string): boolean;
  toJSON(): Record<string, any>;
  forEach(f: (value: V, key: string, map: any) => void): void;
  observe(f: (event: SyncMapEvent, ...args: any[]) => void): void;
  unobserve(f: (event: SyncMapEvent, ...args: any[]) => void): void;
  observeDeep(f: (...args: any[]) => void): void;
  unobserveDeep(f: (...args: any[]) => void): void;
}

/**
 * A synchronized ordered collection supporting indexed access and change observation.
 *
 * Specs: Provides indexed access, push/delete, iteration, and change observation.
 * Multiple backends can implement this interface.
 **/
export interface SyncArray<V = any> {
  get(index: number): V;
  push(items: V[]): void;
  delete(start: number, count: number): void;
  toArray(): V[];
  toJSON(): any[];
  forEach(f: (value: V, index: number, array: any) => void): void;
  readonly length: number;
  observe(f: (...args: any[]) => void): void;
  unobserve(f: (...args: any[]) => void): void;
  observeDeep(f: (...args: any[]) => void): void;
  unobserveDeep(f: (...args: any[]) => void): void;
}

/**
 * A synchronizable document root containing named field stores and collections.
 *
 * Specs: Root container for synchronized state. Provides named access to field
 * stores and collections, standalone store/collection creation for nesting, and
 * transactional batching for atomic updates. The backend determines synchronization
 * strategy (CRDT for real-time collaboration or plain storage for local persistence).
 **/
export interface SyncDoc {
  createMap<V = any>(): SyncMap<V>;
  createArray<V = any>(): SyncArray<V>;
  getMap<V = any>(name?: string): SyncMap<V>;
  getArray<V = any>(name: string): SyncArray<V>;
  transact(fn: () => void, origin?: any): void;
  on(event: string, handler: (...args: any[]) => void): void;
  off(event: string, handler: (...args: any[]) => void): void;
}

/**
 * Type guard for SyncMap instances.
 *
 * Specs: Detects SyncMap by checking for get/set/observe/toJSON methods
 * while excluding arrays.
 **/
export function isSyncMap(value: any): value is SyncMap {
  return (
    value != null &&
    typeof value === "object" &&
    typeof value.get === "function" &&
    typeof value.set === "function" &&
    typeof value.observe === "function" &&
    typeof value.toJSON === "function" &&
    !Array.isArray(value) &&
    typeof value.toArray !== "function"
  );
}

/**
 * Type guard for SyncArray instances.
 *
 * Specs: Detects SyncArray by checking for toArray/push/observe methods.
 **/
export function isSyncArray(value: any): value is SyncArray {
  return value != null && typeof value === "object" && typeof value.toArray === "function" && typeof value.push === "function" && typeof value.observe === "function";
}

/**
 * Factory for creating SyncDoc instances with a specific backend.
 *
 * Specs: Each call produces an independent synchronizable document.
 * The factory determines the backend (CRDT, in-memory, JSON file, SQLite).
 **/
export type SyncDocFactory = () => SyncDoc;

// #endregion SyncInterfaces

// #region 🔖CrdtBackend

import * as Y from "yjs";

/**
 * CRDT-backed SyncDoc implementation using Yjs for real-time collaboration.
 *
 * Specs: createMap/createArray delegate to Y.Map/Y.Array constructors.
 * getMap/getArray delegate to Y.Doc methods. All sync interfaces
 * are structurally satisfied by the CRDT types.
 **/
class CrdtDoc implements SyncDoc {
  public readonly inner: Y.Doc;
  constructor() {
    this.inner = new Y.Doc();
  }
  createMap<V = any>(): SyncMap<V> {
    return new Y.Map<V>() as unknown as SyncMap<V>;
  }
  createArray<V = any>(): SyncArray<V> {
    return new Y.Array<V>() as unknown as SyncArray<V>;
  }
  getMap<V = any>(name?: string): SyncMap<V> {
    return (name !== undefined ? this.inner.getMap(name) : this.inner.getMap()) as unknown as SyncMap<V>;
  }
  getArray<V = any>(name: string): SyncArray<V> {
    return this.inner.getArray(name) as unknown as SyncArray<V>;
  }
  transact(fn: () => void, origin?: any): void {
    this.inner.transact(fn, origin);
  }
  on(event: string, handler: (...args: any[]) => void): void {
    this.inner.on(event as any, handler);
  }
  off(event: string, handler: (...args: any[]) => void): void {
    this.inner.off(event as any, handler);
  }
}

/**
 * Creates a SyncDocFactory backed by CRDT documents for real-time collaboration.
 *
 * Specs: Default backend for sketchpad. Produces CrdtDoc instances wrapping Y.Doc.
 **/
export function createSyncDocFactory(): SyncDocFactory {
  return () => new CrdtDoc();
}

/**
 * Extracts the underlying backend document for persistence and remote provider use.
 * Throws if the SyncDoc is not backed by a CRDT implementation.
 *
 * Specs: Used internally by persistence and remote providers that need
 * direct backend access for binary state encoding.
 **/
export function getSyncBackendDoc(syncDoc: SyncDoc): Y.Doc {
  if (syncDoc instanceof CrdtDoc) return syncDoc.inner;
  throw new Error("SyncDoc is not backed by a CRDT implementation");
}

// #endregion CrdtBackend

// #region 🔖PersistenceProviders

/**
 * Abstract persistence provider for syncing a SyncDoc to a storage backend.
 * Implementations MUST call the synced callback once initial data is loaded.
 * Implementations MUST provide a destroy method to release resources.
 *
 * Specs: Persistence providers bridge between SyncDoc state and durable storage.
 * They handle initial load and ongoing change persistence.
 **/
export interface PersistenceProvider {
  once(event: "synced", callback: () => void): void;
  on(event: "synced", callback: () => void): void;
  destroy(): void;
}

import { IndexeddbPersistence as IndexeddbPersistenceImpl } from "y-indexeddb";

/**
 * Factory that creates a PersistenceProvider for a given SyncDoc and storage key.
 *
 * Specs: The factory pattern allows different persistence strategies
 * (IndexedDB, JSON file, SQLite) to be injected at construction time.
 **/
export type PersistenceFactory = (syncDoc: SyncDoc, key: string) => PersistenceProvider;

/**
 * I/O adapter for reading and writing kit data as a JSON string.
 * Implementations provide platform-specific file system access.
 **/
export interface JsonFileAdapter {
  read(key: string): Promise<string | null>;
  write(key: string, json: string): Promise<void>;
}

/**
 * I/O adapter for reading and writing kit data as SQLite binary.
 * Implementations provide platform-specific file system access.
 **/
export interface SqliteAdapter {
  read(key: string): Promise<Uint8Array | null>;
  write(key: string, data: Uint8Array): Promise<void>;
}

/**
 * PersistenceProvider that syncs SyncDoc state via binary encoding.
 *
 * Specs: Uses binary state encoding for faithful CRDT persistence.
 * The key parameter maps to a storage location via the adapter.
 * On construction, loads persisted state and subscribes to future updates.
 **/
export class SyncBinaryPersistenceProvider implements PersistenceProvider {
  private doc: Y.Doc;
  private key: string;
  private adapter: { read(key: string): Promise<Uint8Array | null>; write(key: string, data: Uint8Array): Promise<void> };
  private syncedCallbacks: (() => void)[] = [];
  private onceCallbacks: (() => void)[] = [];
  private destroyed = false;
  private updateHandler: (update: Uint8Array) => void;

  constructor(syncDoc: SyncDoc, key: string, adapter: { read(key: string): Promise<Uint8Array | null>; write(key: string, data: Uint8Array): Promise<void> }) {
    this.doc = getSyncBackendDoc(syncDoc);
    this.key = key;
    this.adapter = adapter;

    this.updateHandler = () => {
      if (!this.destroyed) {
        const state = Y.encodeStateAsUpdate(this.doc);
        this.adapter.write(this.key, state);
      }
    };

    this.doc.on("update", this.updateHandler);

    this.adapter.read(this.key).then((data) => {
      if (data && !this.destroyed) {
        Y.applyUpdate(this.doc, data);
      }
      this.onceCallbacks.forEach((cb) => cb());
      this.onceCallbacks = [];
      this.syncedCallbacks.forEach((cb) => cb());
    });
  }

  once(event: "synced", callback: () => void): void {
    if (event === "synced") this.onceCallbacks.push(callback);
  }
  on(event: "synced", callback: () => void): void {
    if (event === "synced") this.syncedCallbacks.push(callback);
  }
  destroy(): void {
    this.destroyed = true;
    this.doc.off("update", this.updateHandler);
    this.syncedCallbacks = [];
    this.onceCallbacks = [];
  }
}

/**
 * Creates a PersistenceFactory that uses IndexedDB for browser-based persistence.
 *
 * Specs: Delegates to IndexeddbPersistence with the raw backend document.
 **/
export function createIndexeddbPersistenceFactory(): PersistenceFactory {
  return (syncDoc: SyncDoc, key: string) => new IndexeddbPersistenceImpl(key, getSyncBackendDoc(syncDoc));
}

/**
 * Creates a PersistenceFactory that persists SyncDoc state to a JSON file via an adapter.
 *
 * Specs: Wraps SyncBinaryPersistenceProvider with a JSON adapter that converts
 * between binary state and Base64 JSON for human-inspectable storage.
 **/
export function createJsonFilePersistenceFactory(adapter: JsonFileAdapter): PersistenceFactory {
  return (syncDoc: SyncDoc, key: string) => {
    const binaryAdapter = {
      async read(k: string): Promise<Uint8Array | null> {
        const json = await adapter.read(k);
        if (!json) return null;
        const parsed = JSON.parse(json);
        if (parsed?.syncDocState) {
          const binary = Uint8Array.from(atob(parsed.syncDocState), (c) => c.charCodeAt(0));
          return binary;
        }
        return null;
      },
      async write(k: string, data: Uint8Array): Promise<void> {
        const base64 = btoa(String.fromCharCode(...data));
        await adapter.write(k, JSON.stringify({ syncDocState: base64 }));
      },
    };
    return new SyncBinaryPersistenceProvider(syncDoc, key, binaryAdapter);
  };
}

/**
 * Creates a PersistenceFactory that persists SyncDoc state to a SQLite database via an adapter.
 *
 * Specs: Uses binary state encoding stored as a BLOB in a SQLite table.
 * The adapter provides platform-specific SQLite file I/O.
 **/
export function createSqliteFolderPersistenceFactory(adapter: SqliteAdapter): PersistenceFactory {
  return (syncDoc: SyncDoc, key: string) => {
    return new SyncBinaryPersistenceProvider(syncDoc, key, adapter);
  };
}

// #endregion PersistenceProviders

// #region 🔖JsonFileKitStore
// [👤semio👥studio💻studio🔖jsonfilekitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/JsonFileKitStore)
// JSON file-backed kit store implementing UndoableKitStore.
// Specs: Loads a Kit from a JSON file via adapter, holds an in-memory working copy,
// persists on save() by serializing the full Kit back to JSON. Supports undo/redo
// with a command stack. reload() re-reads state from the file, discarding changes.
// Used by: VS Code extension for *.kit.json file editing.

import { type Kit, type KitDiff, type KitChange, type KitStore, type KitStoreSnapshot, type KitStoreStatus, type KitSyncState, type UndoableKitStore, KitSchema, applyKitDiff, getKitDiff, inverseKitDiff, guid } from "@semio/js/semio";

/**
 * Adapter for reading/writing Kit JSON to a file.
 *
 * Specs: read() returns the raw JSON string from the file, or null if not found.
 * write() writes the JSON string to the file. Implementations provide
 * platform-specific I/O (Node fs, VS Code workspace, etc.).
 * [👤semio👥studio💻studio🔖jsonfilekitstore🛠️kitjsonfileadapter](repo://p/u/semio/b/l/studio/f/studio.ts/s/JsonFileKitStore/d/i/KitJsonFileAdapter)
 **/
export interface KitJsonFileAdapter {
  read(): Promise<string | null>;
  write(json: string): Promise<void>;
}

/**
 * JSON file-backed kit store with undo/redo.
 *
 * Specs: Holds a Kit in memory loaded from a JSON file. apply() merges diffs,
 * replace() swaps the Kit. transact() groups mutations into one undo entry.
 * save() serializes the Kit to JSON and writes via adapter. reload() re-reads
 * from the file, resetting the working copy. Undo/redo uses a command stack.
 * [👤semio👥studio💻studio🔖jsonfilekitstore🛠️jsonfilekitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/JsonFileKitStore/d/i/JsonFileKitStore)
 **/
export class JsonFileKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus;
  private transacting: boolean = false;
  private error?: Error;
  private lastSyncedAt?: string;
  private readonly adapter: KitJsonFileAdapter;

  private constructor(kit: Kit, adapter: KitJsonFileAdapter, status: KitStoreStatus) {
    this.kit = kit;
    this.adapter = adapter;
    this.status = status;
  }

  /**
   * Creates a JsonFileKitStore by loading Kit data from the adapter.
   * If the file does not exist or is empty, creates a new empty kit.
   *
   * Specs: Factory method that handles async loading. Parses JSON with KitSchema
   * validation. On parse failure, reports error status.
   **/
  static async create(adapter: KitJsonFileAdapter): Promise<JsonFileKitStore> {
    const json = await adapter.read();
    if (json) {
      try {
        const parsed = JSON.parse(json);
        const kit = KitSchema.parse(parsed);
        const store = new JsonFileKitStore(kit, adapter, "ready");
        store.lastSyncedAt = new Date().toISOString();
        return store;
      } catch (e) {
        const emptyKit: Kit = {
          guid: guid(),
          name: "New Kit",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };
        const store = new JsonFileKitStore(emptyKit, adapter, "error");
        store.error = e instanceof Error ? e : new Error(String(e));
        return store;
      }
    }
    const emptyKit: Kit = {
      guid: guid(),
      name: "New Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    const store = new JsonFileKitStore(emptyKit, adapter, "ready");
    return store;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
        lastSyncedAt: this.lastSyncedAt,
        error: this.error,
      },
    };
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  transact<T>(label: string, run: () => T): T {
    const before = this.kit;
    this.transacting = true;
    try {
      const result = run();
      const after = this.kit;
      if (before !== after && !this.disposed) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
    }
  }

  apply(diff: KitDiff, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = applyKitDiff(this.kit, diff);
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, this.kit);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  replace(next: Kit, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = next;
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, next);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  async save(): Promise<void> {
    this.status = "saving";
    this.notify();
    try {
      const json = JSON.stringify(this.kit, null, 2);
      await this.adapter.write(json);
      this.dirty = false;
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const json = await this.adapter.read();
      if (json) {
        const parsed = JSON.parse(json);
        this.kit = KitSchema.parse(parsed);
      }
      this.dirty = false;
      this.undoStack = [];
      this.redoStack = [];
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
    this.undoStack = [];
    this.redoStack = [];
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  undo(): void {
    const change = this.undoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.backward);
    this.redoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  redo(): void {
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  /**
   * Applies an external update to the kit (e.g., file changed on disk).
   * Does NOT create undo entries.
   *
   * Specs: Used by file watchers to push external changes into the store.
   * Resets dirty flag since the file is the source of truth.
   **/
  applyExternalUpdate(kit: Kit): void {
    this.kit = kit;
    this.dirty = false;
    this.undoStack = [];
    this.redoStack = [];
    this.lastSyncedAt = new Date().toISOString();
    this.error = undefined;
    this.status = "ready";
    this.notify();
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a JsonFileKitStore by loading kit data from a file adapter.
 *
 * Specs: Factory function matching the provider pattern. Returns a ready-to-use
 * JsonFileKitStore. The adapter provides platform-specific file I/O.
 * [👤semio👥studio💻studio🔖jsonfilekitstore🛠️createjsonfilekitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/JsonFileKitStore/d/i/createJsonFileKitStore)
 **/
export async function createJsonFileKitStore(adapter: KitJsonFileAdapter): Promise<JsonFileKitStore> {
  return JsonFileKitStore.create(adapter);
}

// #endregion 🔖JsonFileKitStore

// #region 🔖FolderKitStore
// [👤semio👥studio💻studio🔖folderkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/FolderKitStore)
// Folder-backed kit store implementing UndoableKitStore.
// Specs: Uses a folder with `.semio/kit.json` for kit data. The folder serves as
// the root for relative file references. Files referenced by the kit are stored
// alongside the kit data in the folder. save() writes kit data back to `.semio/kit.json`.
// reload() re-reads the kit data from the file. Used by: desktop app for local kit editing.

/**
 * Adapter for folder-based kit storage I/O.
 *
 * Specs: readKit()/writeKit() handle the `.semio/kit.json` file.
 * readFile()/writeFile()/deleteFile() handle binary assets relative to the folder root.
 * listFiles() returns all file paths in the folder.
 * watch() optionally registers a callback for external changes.
 * [👤semio👥studio💻studio🔖folderkitstore🛠️kitfolderadapter](repo://p/u/semio/b/l/studio/f/studio.ts/s/FolderKitStore/d/i/KitFolderAdapter)
 **/
export interface KitFolderAdapter {
  readKit(): Promise<string | null>;
  writeKit(json: string): Promise<void>;
  readFile(path: string): Promise<Blob | null>;
  writeFile(path: string, blob: Blob): Promise<void>;
  deleteFile(path: string): Promise<void>;
  listFiles(): Promise<string[]>;
  watch?(callback: () => void): () => void;
}

/**
 * Folder-backed kit store with undo/redo.
 *
 * Specs: Holds a Kit in memory loaded from a folder's `.semio/kit.json`.
 * apply() merges diffs, replace() swaps the Kit. transact() groups mutations.
 * save() writes the Kit as JSON. reload() re-reads from the folder.
 * Undo/redo uses a command stack identical to JsonFileKitStore.
 * [👤semio👥studio💻studio🔖folderkitstore🛠️folderkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/FolderKitStore/d/i/FolderKitStore)
 **/
export class FolderKitStore implements UndoableKitStore {
  private kit: Kit;
  private listeners: Set<() => void> = new Set();
  private undoStack: KitChange[] = [];
  private redoStack: KitChange[] = [];
  private dirty: boolean = false;
  private disposed: boolean = false;
  private status: KitStoreStatus;
  private transacting: boolean = false;
  private error?: Error;
  private lastSyncedAt?: string;
  private readonly adapter: KitFolderAdapter;
  private unwatchFn?: () => void;

  private constructor(kit: Kit, adapter: KitFolderAdapter, status: KitStoreStatus) {
    this.kit = kit;
    this.adapter = adapter;
    this.status = status;
    if (adapter.watch) {
      this.unwatchFn = adapter.watch(() => {
        this.reload().catch(console.error);
      });
    }
  }

  static async create(adapter: KitFolderAdapter): Promise<FolderKitStore> {
    const json = await adapter.readKit();
    if (json) {
      try {
        const parsed = JSON.parse(json);
        const kit = KitSchema.parse(parsed);
        const store = new FolderKitStore(kit, adapter, "ready");
        store.lastSyncedAt = new Date().toISOString();
        return store;
      } catch (e) {
        const emptyKit: Kit = {
          guid: guid(),
          name: "New Kit",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };
        const store = new FolderKitStore(emptyKit, adapter, "error");
        store.error = e instanceof Error ? e : new Error(String(e));
        return store;
      }
    }
    const emptyKit: Kit = {
      guid: guid(),
      name: "New Kit",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    const store = new FolderKitStore(emptyKit, adapter, "ready");
    return store;
  }

  getSnapshot(): KitStoreSnapshot {
    return {
      kit: this.kit,
      sync: {
        status: this.status,
        dirty: this.dirty,
        readonly: false,
        lastSyncedAt: this.lastSyncedAt,
        error: this.error,
      },
    };
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  transact<T>(label: string, run: () => T): T {
    const before = this.kit;
    this.transacting = true;
    try {
      const result = run();
      const after = this.kit;
      if (before !== after && !this.disposed) {
        const forward = getKitDiff(before, after);
        const backward = inverseKitDiff(before, forward);
        this.undoStack.push({ forward, backward });
        this.redoStack = [];
      }
      return result;
    } finally {
      this.transacting = false;
    }
  }

  apply(diff: KitDiff, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = applyKitDiff(this.kit, diff);
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, this.kit);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  replace(next: Kit, meta?: { origin?: string }): void {
    const before = this.kit;
    this.kit = next;
    this.dirty = true;
    if (!this.transacting && !this.disposed) {
      const forward = getKitDiff(before, next);
      const backward = inverseKitDiff(before, forward);
      this.undoStack.push({ forward, backward });
      this.redoStack = [];
    }
    this.notify();
  }

  async save(): Promise<void> {
    this.status = "saving";
    this.notify();
    try {
      const json = JSON.stringify(this.kit, null, 2);
      await this.adapter.writeKit(json);
      this.dirty = false;
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const json = await this.adapter.readKit();
      if (json) {
        const parsed = JSON.parse(json);
        this.kit = KitSchema.parse(parsed);
      }
      this.dirty = false;
      this.undoStack = [];
      this.redoStack = [];
      this.lastSyncedAt = new Date().toISOString();
      this.error = undefined;
      this.status = "ready";
    } catch (e) {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    }
    this.notify();
  }

  dispose(): void {
    this.disposed = true;
    if (this.unwatchFn) {
      this.unwatchFn();
      this.unwatchFn = undefined;
    }
    this.listeners.clear();
    this.undoStack = [];
    this.redoStack = [];
  }

  canUndo(): boolean {
    return this.undoStack.length > 0;
  }

  canRedo(): boolean {
    return this.redoStack.length > 0;
  }

  undo(): void {
    const change = this.undoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.backward);
    this.redoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  redo(): void {
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.notify();
  }

  applyExternalUpdate(kit: Kit): void {
    this.kit = kit;
    this.dirty = false;
    this.undoStack = [];
    this.redoStack = [];
    this.lastSyncedAt = new Date().toISOString();
    this.error = undefined;
    this.status = "ready";
    this.notify();
  }

  async writeFile(path: string, blob: Blob): Promise<void> {
    await this.adapter.writeFile(path, blob);
  }

  async readFile(path: string): Promise<Blob | null> {
    return this.adapter.readFile(path);
  }

  async deleteFile(path: string): Promise<void> {
    await this.adapter.deleteFile(path);
  }

  async listFiles(): Promise<string[]> {
    return this.adapter.listFiles();
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a FolderKitStore by loading kit data from a folder adapter.
 *
 * Specs: Factory function matching the provider pattern. Returns a ready-to-use
 * FolderKitStore. The adapter provides platform-specific folder I/O.
 * [👤semio👥studio💻studio🔖folderkitstore🛠️createfolderkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/FolderKitStore/d/i/createFolderKitStore)
 **/
export async function createFolderKitStore(adapter: KitFolderAdapter): Promise<FolderKitStore> {
  return FolderKitStore.create(adapter);
}

// #endregion 🔖FolderKitStore
