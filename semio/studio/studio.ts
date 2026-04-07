// #region 🔖Header
// [👤semio👥studio💻studio](repo://p/u/semio/b/l/studio/f/studio.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

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

import {
  type Kit,
  type KitChange,
  type KitDiff,
  type KitStore,
  type KitStoreSnapshot,
  type KitStoreStatus,
  type KitSyncState,
  type UndoableKitStore,
  KitSchema,
  applyKitDiff,
  getKitDiff,
  getSqlJs,
  guid,
  inverseKitDiff,
  kitToSqlite,
  sqliteToKit,
} from "@semio/js";

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
// Specs: Uses a folder with `.semio/kit.db` SQLite database for kit data. The folder serves as
// the root for relative file references. Files referenced by the kit are stored
// alongside the kit data in the folder. save() writes kit data back to `.semio/kit.db`.
// reload() re-reads the kit data from the database. Used by: desktop app for local kit editing.

/**
 * Adapter for folder-based kit storage I/O.
 *
 * Specs: readKit()/writeKit() handle the `.semio/kit.db` SQLite database as binary data.
 * readFile()/writeFile()/deleteFile() handle binary assets relative to the folder root.
 * listFiles() returns all file paths in the folder.
 * watch() optionally registers a callback for external changes.
 * [👤semio👥studio💻studio🔖folderkitstore🛠️kitfolderadapter](repo://p/u/semio/b/l/studio/f/studio.ts/s/FolderKitStore/d/i/KitFolderAdapter)
 **/
export interface KitFolderAdapter {
  readKit(): Promise<Uint8Array | null>;
  writeKit(data: Uint8Array): Promise<void>;
  readFile(path: string): Promise<Blob | null>;
  writeFile(path: string, blob: Blob): Promise<void>;
  deleteFile(path: string): Promise<void>;
  listFiles(): Promise<string[]>;
  watch?(callback: () => void): () => void;
}

/**
 * Folder-backed kit store with undo/redo.
 *
 * Specs: Holds a Kit in memory loaded from a folder's `.semio/kit.db` SQLite database.
 * apply() merges diffs, replace() swaps the Kit. transact() groups mutations.
 * save() serializes the Kit to SQLite and writes via adapter. reload() re-reads from the folder.
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
    const data = await adapter.readKit();
    if (data) {
      try {
        const SQL = await getSqlJs();
        const db = new SQL.Database(new Uint8Array(data));
        const kit = await sqliteToKit(db);
        db.close();
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
      const SQL = await getSqlJs();
      const db = new SQL.Database();
      await kitToSqlite(this.kit, db);
      const data = db.export();
      db.close();
      await this.adapter.writeKit(data);
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
      const data = await this.adapter.readKit();
      if (data) {
        const SQL = await getSqlJs();
        const db = new SQL.Database(new Uint8Array(data));
        this.kit = await sqliteToKit(db);
        db.close();
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

// #region 🔖SessionKitStore
// [👤semio👥studio💻studio🔖sessionkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/SessionKitStore)
// Server-backed kit store implementing UndoableKitStore.
// Specs: Connects to a semio-session backend via HTTP+WS. Commands are sent via HTTP POST,
// events received via WebSocket. Local Kit state is maintained in-memory and updated on
// accepted domain events. Baseline snapshots and incremental diffs are stored server-side.
// Supports undo/redo with a local command stack. Lookback history via server API.
// Used by: sketchpad, desktop, any frontend needing real-time collaborative kit editing.

/**
 * Configuration for creating a SessionKitStore.
 *
 * Specs: serverUrl is the base URL (e.g. http://localhost:8080). sessionId is optional —
 * if omitted, a new session is created. kitName is used when creating a new session.
 * personId and clientId identify this frontend instance for presence.
 * [👤semio👥studio💻studio🔖sessionkitstore🛠️sessionkitstoreconfig](repo://p/u/semio/b/l/studio/f/studio.ts/s/SessionKitStore/d/i/SessionKitStoreConfig)
 **/
export interface SessionKitStoreConfig {
  serverUrl: string;
  sessionId?: string;
  kitName?: string;
  personId?: string;
  clientId?: string;
}

/**
 * Server event received via WebSocket.
 *
 * Specs: Mirrors the Rust SessionEvent enum. Used internally by SessionKitStore
 * to update local state on server-side changes.
 **/
interface ServerEvent {
  event: string;
  command_id?: { "0": string };
  domain_version?: number;
  semio_version?: number;
  changes?: ServerEntityChange[];
  person_id?: { "0": string };
  frontend_id?: string;
  update?: ServerSemioUpdate;
}

interface ServerEntityChange {
  op: "Created" | "Updated" | "Deleted";
  entity_kind: string;
  entity_id: string;
  snapshot?: Record<string, any>;
  changed_fields?: Record<string, any>;
}

interface ServerSemioUpdate {
  kind: string;
  u?: number;
  v?: number;
  position?: [number, number, number];
  forward?: [number, number, number];
  up?: [number, number, number];
  piece_ids?: string[];
  design_ids?: string[];
}

/**
 * Presence state for one person on one frontend.
 *
 * Specs: Tracks cursor position, camera look, selection, and display metadata.
 * Updated by SemioUpdated events from the server.
 **/
export interface PresenceState {
  personId: string;
  frontendId: string;
  displayName?: string;
  color?: string;
  cursor?: { u: number; v: number };
  look?: { position: [number, number, number]; forward: [number, number, number]; up: [number, number, number] };
  selectedPieceIds: string[];
  selectedDesignIds: string[];
}

/**
 * Server-backed kit store with undo/redo and real-time sync.
 *
 * Specs: Connects to semio-session server via HTTP for commands and WS for events.
 * On connect: fetches snapshot to initialize local Kit. On mutation: sends DomainCommand
 * via POST, waits for Accepted event via WS. On WS event: applies entity changes to
 * local Kit and notifies subscribers. Undo/redo operates on local command stack.
 * Provides presence tracking via semio commands.
 * [👤semio👥studio💻studio🔖sessionkitstore🛠️sessionkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/SessionKitStore/d/i/SessionKitStore)
 **/
export class SessionKitStore implements UndoableKitStore {
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
  private ws: WebSocket | null = null;
  private domainVersion: number = 0;
  private semioVersion: number = 0;
  private presences: Map<string, PresenceState> = new Map();
  private presenceListeners: Set<() => void> = new Set();
  private entityListeners: Map<string, Set<() => void>> = new Map();
  private collectionListeners: Map<string, Set<() => void>> = new Map();
  private propertyListeners: Map<string, Set<() => void>> = new Map();

  readonly serverUrl: string;
  readonly sessionId: string;
  readonly personId: string;
  readonly clientId: string;

  private constructor(kit: Kit, config: SessionKitStoreConfig & { sessionId: string }, status: KitStoreStatus) {
    this.kit = kit;
    this.serverUrl = config.serverUrl;
    this.sessionId = config.sessionId;
    this.personId = config.personId ?? guid();
    this.clientId = config.clientId ?? guid();
    this.status = status;
  }

  /**
   * Creates a SessionKitStore by connecting to the server.
   * If sessionId is provided, fetches the existing session snapshot.
   * If not, creates a new session on the server.
   *
   * Specs: Factory method handling async connection. Establishes WebSocket
   * for real-time events after initial snapshot load.
   **/
  static async create(config: SessionKitStoreConfig): Promise<SessionKitStore> {
    let sessionId = config.sessionId;
    let kitName = config.kitName ?? "New Kit";

    if (!sessionId) {
      const resp = await fetch(`${config.serverUrl}/sessions`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ kit_name: kitName }),
      });
      if (!resp.ok) throw new Error(`Failed to create session: ${resp.statusText}`);
      const body = await resp.json();
      sessionId = body.session_id;
    }

    const snapResp = await fetch(`${config.serverUrl}/sessions/${sessionId}/snapshot`);
    if (!snapResp.ok) throw new Error(`Failed to load snapshot: ${snapResp.statusText}`);
    const snapshot = await snapResp.json();

    const kit: Kit = {
      guid: snapshot.kit?.kit_id ?? guid(),
      name: snapshot.kit?.name ?? kitName,
      version: snapshot.kit?.version,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    const store = new SessionKitStore(kit, { ...config, sessionId: sessionId! }, "ready");
    store.domainVersion = snapshot.domain_version ?? 0;
    store.semioVersion = snapshot.semio_version ?? 0;
    store.lastSyncedAt = new Date().toISOString();
    store.connectWebSocket();
    return store;
  }

  private connectWebSocket(): void {
    const wsUrl = this.serverUrl.replace(/^http/, "ws") + `/sessions/${this.sessionId}/ws`;
    this.ws = new WebSocket(wsUrl);
    this.ws.onmessage = (event) => {
      try {
        const data: ServerEvent = JSON.parse(typeof event.data === "string" ? event.data : "");
        this.handleServerEvent(data);
      } catch (e) {
        // Ignore unparseable messages
      }
    };
    this.ws.onclose = () => {
      if (!this.disposed) {
        this.status = "offline";
        this.notify();
        // Auto-reconnect after 2 seconds
        setTimeout(() => {
          if (!this.disposed) this.connectWebSocket();
        }, 2000);
      }
    };
    this.ws.onerror = () => {
      this.status = "offline";
      this.notify();
    };
    this.ws.onopen = () => {
      this.status = "ready";
      this.error = undefined;
      this.notify();
    };
  }

  private handleServerEvent(event: ServerEvent): void {
    switch (event.event) {
      case "DomainCommandAccepted": {
        if (event.domain_version !== undefined) {
          this.domainVersion = event.domain_version;
        }
        if (event.changes) {
          const before = this.kit;
          this.applyServerChanges(event.changes);
          this.lastSyncedAt = new Date().toISOString();
          // Notify granular listeners
          for (const change of event.changes) {
            this.notifyEntityListeners(change.entity_kind, change.entity_id);
            this.notifyCollectionListeners(change.entity_kind);
            if (change.op === "Updated" && change.changed_fields) {
              for (const field of Object.keys(change.changed_fields)) {
                this.notifyPropertyListeners(change.entity_kind, change.entity_id, field);
              }
            }
          }
        }
        this.dirty = false;
        this.notify();
        break;
      }
      case "SemioUpdated": {
        if (event.semio_version !== undefined) {
          this.semioVersion = event.semio_version;
        }
        if (event.person_id && event.frontend_id && event.update) {
          const key = `${event.person_id["0"]}:${event.frontend_id}`;
          let presence = this.presences.get(key) ?? {
            personId: event.person_id["0"],
            frontendId: event.frontend_id,
            selectedPieceIds: [],
            selectedDesignIds: [],
          };
          switch (event.update.kind) {
            case "CursorMoved":
              presence.cursor = { u: event.update.u!, v: event.update.v! };
              break;
            case "LookChanged":
              presence.look = { position: event.update.position!, forward: event.update.forward!, up: event.update.up! };
              break;
            case "SelectionChanged":
              presence.selectedPieceIds = event.update.piece_ids ?? [];
              presence.selectedDesignIds = event.update.design_ids ?? [];
              break;
            case "PresenceCleared":
              this.presences.delete(key);
              this.notifyPresenceListeners();
              this.notify();
              return;
          }
          this.presences.set(key, presence);
          this.notifyPresenceListeners();
        }
        this.notify();
        break;
      }
      case "SessionClosed":
        this.status = "offline";
        this.notify();
        break;
    }
  }

  private applyServerChanges(changes: ServerEntityChange[]): void {
    for (const change of changes) {
      switch (change.op) {
        case "Created":
          this.applyCreatedEntity(change.entity_kind, change.entity_id, change.snapshot ?? {});
          break;
        case "Updated":
          this.applyUpdatedEntity(change.entity_kind, change.entity_id, change.changed_fields ?? {});
          break;
        case "Deleted":
          this.applyDeletedEntity(change.entity_kind, change.entity_id);
          break;
      }
    }
  }

  private applyCreatedEntity(entityKind: string, entityId: string, snapshot: Record<string, any>): void {
    const entity = { guid: entityId, ...snapshot };
    switch (entityKind) {
      case "type":
        this.kit = { ...this.kit, types: [...(this.kit.types ?? []), entity as any] };
        break;
      case "design":
        this.kit = { ...this.kit, designs: [...(this.kit.designs ?? []), entity as any] };
        break;
      case "author":
        this.kit = { ...this.kit, authors: [...(this.kit.authors ?? []), entity as any] };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: [...(this.kit.tags ?? []), entity as any] };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: [...(this.kit.concepts ?? []), entity as any] };
        break;
      case "port":
        this.kit = { ...this.kit, ports: [...(this.kit.ports ?? []), entity as any] };
        break;
      case "quality":
        this.kit = { ...this.kit, qualities: [...(this.kit.qualities ?? []), entity as any] };
        break;
      case "file":
        this.kit = { ...this.kit, files: [...(this.kit.files ?? []), entity as any] };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: [...(this.kit.folders ?? []), entity as any] };
        break;
    }
  }

  private applyUpdatedEntity(entityKind: string, entityId: string, changedFields: Record<string, any>): void {
    const updateInArray = <T extends { guid: string }>(arr: T[] | undefined, id: string, fields: Record<string, any>): T[] => {
      return (arr ?? []).map((item) => (item.guid === id ? { ...item, ...fields } : item));
    };
    switch (entityKind) {
      case "kit":
        this.kit = { ...this.kit, ...changedFields };
        break;
      case "type":
        this.kit = { ...this.kit, types: updateInArray(this.kit.types, entityId, changedFields) };
        break;
      case "design":
        this.kit = { ...this.kit, designs: updateInArray(this.kit.designs, entityId, changedFields) };
        break;
      case "author":
        this.kit = { ...this.kit, authors: updateInArray(this.kit.authors, entityId, changedFields) };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: updateInArray(this.kit.tags, entityId, changedFields) };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: updateInArray(this.kit.concepts, entityId, changedFields) };
        break;
      case "port":
        this.kit = { ...this.kit, ports: updateInArray(this.kit.ports, entityId, changedFields) };
        break;
      case "quality":
        this.kit = { ...this.kit, qualities: updateInArray(this.kit.qualities, entityId, changedFields) };
        break;
      case "file":
        this.kit = { ...this.kit, files: updateInArray(this.kit.files, entityId, changedFields) };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: updateInArray(this.kit.folders, entityId, changedFields) };
        break;
    }
  }

  private applyDeletedEntity(entityKind: string, entityId: string): void {
    const removeFromArray = <T extends { guid: string }>(arr: T[] | undefined, id: string): T[] => {
      return (arr ?? []).filter((item) => item.guid !== id);
    };
    switch (entityKind) {
      case "type":
        this.kit = { ...this.kit, types: removeFromArray(this.kit.types, entityId) };
        break;
      case "design":
        this.kit = { ...this.kit, designs: removeFromArray(this.kit.designs, entityId) };
        break;
      case "author":
        this.kit = { ...this.kit, authors: removeFromArray(this.kit.authors, entityId) };
        break;
      case "tag":
        this.kit = { ...this.kit, tags: removeFromArray(this.kit.tags, entityId) };
        break;
      case "concept":
        this.kit = { ...this.kit, concepts: removeFromArray(this.kit.concepts, entityId) };
        break;
      case "port":
        this.kit = { ...this.kit, ports: removeFromArray(this.kit.ports, entityId) };
        break;
      case "quality":
        this.kit = { ...this.kit, qualities: removeFromArray(this.kit.qualities, entityId) };
        break;
      case "file":
        this.kit = { ...this.kit, files: removeFromArray(this.kit.files, entityId) };
        break;
      case "folder":
        this.kit = { ...this.kit, folders: removeFromArray(this.kit.folders, entityId) };
        break;
    }
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
    // Send diff as domain commands to server
    this.sendKitDiffToServer(diff).catch((e) => {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    });
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
    this.sendKitDiffToServer(getKitDiff(before, next)).catch((e) => {
      this.error = e instanceof Error ? e : new Error(String(e));
      this.status = "error";
    });
    this.notify();
  }

  private async sendKitDiffToServer(diff: KitDiff): Promise<void> {
    const commands: any[] = [];
    // Kit-level fields
    const kitFields: Record<string, any> = {};
    if (diff.name !== undefined) kitFields.name = diff.name;
    if (diff.version !== undefined) kitFields.version = diff.version;
    if (diff.description !== undefined) kitFields.description = diff.description;
    if (diff.icon !== undefined) kitFields.icon = diff.icon;
    if (diff.image !== undefined) kitFields.image = diff.image;
    if (diff.remote !== undefined) kitFields.remote = diff.remote;
    if (diff.homepage !== undefined) kitFields.homepage = diff.homepage;
    if (diff.license !== undefined) kitFields.license = diff.license;
    if (diff.preview !== undefined) kitFields.preview = diff.preview;
    if (Object.keys(kitFields).length > 0) {
      commands.push({ kind: "PatchKit", payload: { fields: kitFields } });
    }
    // Collection diffs
    const collectionMap: Record<string, { create: string; patch: string; delete: string }> = {
      types: { create: "CreateType", patch: "PatchType", delete: "DeleteType" },
      designs: { create: "CreateDesign", patch: "PatchDesign", delete: "DeleteDesign" },
      authors: { create: "CreateAuthor", patch: "PatchAuthor", delete: "DeleteAuthor" },
      tags: { create: "CreateTag", patch: "PatchTag", delete: "DeleteTag" },
      concepts: { create: "CreateConcept", patch: "PatchConcept", delete: "DeleteConcept" },
      ports: { create: "CreatePort", patch: "PatchPort", delete: "DeletePort" },
      qualities: { create: "CreateQuality", patch: "PatchQuality", delete: "DeleteQuality" },
      files: { create: "CreateFile", patch: "PatchFile", delete: "DeleteFile" },
      folders: { create: "CreateFolder", patch: "PatchFolder", delete: "DeleteFolder" },
    };
    for (const [key, ops] of Object.entries(collectionMap)) {
      const collDiff = (diff as any)[key];
      if (!collDiff) continue;
      if (collDiff.added) {
        for (const item of collDiff.added) {
          commands.push({ kind: ops.create, payload: { entity_id: item.guid ?? guid(), fields: item } });
        }
      }
      if (collDiff.updated) {
        for (const item of collDiff.updated) {
          commands.push({ kind: ops.patch, payload: { entity_id: item.guid, fields: item } });
        }
      }
      if (collDiff.removed) {
        for (const item of collDiff.removed) {
          commands.push({ kind: ops.delete, payload: { entity_id: item.guid } });
        }
      }
    }
    if (commands.length === 0) return;
    const batch = commands.length === 1 ? commands[0] : { kind: "Batch", payload: { commands } };
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/domain`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        command_id: { "0": guid() },
        client_id: { "0": this.clientId },
        request_id: { "0": guid() },
        actor_person_id: { "0": this.personId },
        base_domain_version: this.domainVersion,
        ...batch,
      }),
    });
    if (!resp.ok) {
      throw new Error(`Failed to send command: ${resp.statusText}`);
    }
  }

  async save(): Promise<void> {
    // Server-backed: save is implicit on command submission
    this.dirty = false;
    this.lastSyncedAt = new Date().toISOString();
    this.notify();
  }

  async reload(): Promise<void> {
    this.status = "loading";
    this.notify();
    try {
      const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/snapshot`);
      if (!resp.ok) throw new Error(`Failed to reload: ${resp.statusText}`);
      const snapshot = await resp.json();
      this.kit = {
        guid: snapshot.kit?.kit_id ?? this.kit.guid,
        name: snapshot.kit?.name ?? this.kit.name,
        version: snapshot.kit?.version,
        createdAt: this.kit.createdAt,
        updatedAt: new Date().toISOString(),
      };
      this.domainVersion = snapshot.domain_version ?? 0;
      this.semioVersion = snapshot.semio_version ?? 0;
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
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.listeners.clear();
    this.presenceListeners.clear();
    this.entityListeners.clear();
    this.collectionListeners.clear();
    this.propertyListeners.clear();
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
    this.sendKitDiffToServer(change.backward).catch(() => { });
    this.notify();
  }

  redo(): void {
    const change = this.redoStack.pop();
    if (!change) return;
    this.kit = applyKitDiff(this.kit, change.forward);
    this.undoStack.push(change);
    this.dirty = true;
    this.sendKitDiffToServer(change.forward).catch(() => { });
    this.notify();
  }

  // #region 🔖Presence

  /**
   * Send cursor position to server for this person.
   **/
  async sendCursor(u: number, v: number): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "UpsertCursor",
        payload: { u, v },
      }),
    });
  }

  /**
   * Send camera look to server for this person.
   **/
  async sendLook(position: [number, number, number], forward: [number, number, number], up: [number, number, number]): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "UpsertLook",
        payload: { position, forward, up },
      }),
    });
  }

  /**
   * Send piece/design selection to server for this person.
   **/
  async sendSelection(pieceIds: string[], designIds: string[]): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "SetSelection",
        payload: { piece_ids: pieceIds, design_ids: designIds },
      }),
    });
  }

  /**
   * Clear this person's presence from the server.
   **/
  async clearPresence(): Promise<void> {
    await fetch(`${this.serverUrl}/sessions/${this.sessionId}/commands/semio`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        client_id: { "0": this.clientId },
        person_id: { "0": this.personId },
        frontend_id: this.clientId,
        base_semio_version: this.semioVersion,
        kind: "ClearPresence",
        payload: null,
      }),
    });
  }

  /**
   * Get all currently known presences.
   **/
  getPresences(): PresenceState[] {
    return Array.from(this.presences.values());
  }

  /**
   * Subscribe to presence changes.
   **/
  subscribePresence(listener: () => void): () => void {
    this.presenceListeners.add(listener);
    return () => {
      this.presenceListeners.delete(listener);
    };
  }

  // #endregion 🔖Presence

  // #region 🔖History

  /**
   * Get kit state at a named lookback point (e.g. "1min", "5h", "1d").
   **/
  async getKitAtLookback(lookback: string): Promise<Kit> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/kit/at/${lookback}`);
    if (!resp.ok) throw new Error(`Failed to get kit at lookback ${lookback}: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Get kit state at a specific domain version.
   **/
  async getKitAtVersion(version: number): Promise<Kit> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/kit/at-version/${version}`);
    if (!resp.ok) throw new Error(`Failed to get kit at version ${version}: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Trigger history compaction on the server.
   **/
  async compactHistory(): Promise<{ snapshots_created: number; logs_deleted: number }> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/history/compact`, { method: "POST" });
    if (!resp.ok) throw new Error(`Failed to compact: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Get available lookback tokens.
   **/
  async getLookbackTokens(): Promise<string[]> {
    const resp = await fetch(`${this.serverUrl}/sessions/${this.sessionId}/history/lookback-tokens`);
    if (!resp.ok) throw new Error(`Failed to get tokens: ${resp.statusText}`);
    return resp.json();
  }

  /**
   * Current domain version from the server.
   **/
  getDomainVersion(): number {
    return this.domainVersion;
  }

  /**
   * Current semio version from the server.
   **/
  getSemioVersion(): number {
    return this.semioVersion;
  }

  // #endregion 🔖History

  // #region 🔖GranularSubscriptions

  /**
   * Subscribe to changes on a specific entity by kind and guid.
   * Listener fires only when that entity is created, updated, or deleted.
   **/
  subscribeEntity(entityKind: string, entityId: string, listener: () => void): () => void {
    const key = `${entityKind}:${entityId}`;
    if (!this.entityListeners.has(key)) this.entityListeners.set(key, new Set());
    this.entityListeners.get(key)!.add(listener);
    return () => {
      this.entityListeners.get(key)?.delete(listener);
    };
  }

  /**
   * Subscribe to changes on a collection (e.g. "type", "design").
   * Listener fires when any entity of that kind is created or deleted.
   **/
  subscribeCollection(entityKind: string, listener: () => void): () => void {
    if (!this.collectionListeners.has(entityKind)) this.collectionListeners.set(entityKind, new Set());
    this.collectionListeners.get(entityKind)!.add(listener);
    return () => {
      this.collectionListeners.get(entityKind)?.delete(listener);
    };
  }

  /**
   * Subscribe to changes on a specific property of a specific entity.
   * Listener fires only when that exact field changes.
   **/
  subscribeProperty(entityKind: string, entityId: string, field: string, listener: () => void): () => void {
    const key = `${entityKind}:${entityId}:${field}`;
    if (!this.propertyListeners.has(key)) this.propertyListeners.set(key, new Set());
    this.propertyListeners.get(key)!.add(listener);
    return () => {
      this.propertyListeners.get(key)?.delete(listener);
    };
  }

  private notifyEntityListeners(entityKind: string, entityId: string): void {
    const key = `${entityKind}:${entityId}`;
    this.entityListeners.get(key)?.forEach((l) => l());
  }

  private notifyCollectionListeners(entityKind: string): void {
    this.collectionListeners.get(entityKind)?.forEach((l) => l());
  }

  private notifyPropertyListeners(entityKind: string, entityId: string, field: string): void {
    const key = `${entityKind}:${entityId}:${field}`;
    this.propertyListeners.get(key)?.forEach((l) => l());
  }

  private notifyPresenceListeners(): void {
    this.presenceListeners.forEach((l) => l());
  }

  // #endregion 🔖GranularSubscriptions

  private notify(): void {
    for (const listener of this.listeners) {
      listener();
    }
  }
}

/**
 * Creates a SessionKitStore by connecting to a semio-session server.
 *
 * Specs: Factory function matching the provider pattern.
 * [👤semio👥studio💻studio🔖sessionkitstore🛠️createsessionkitstore](repo://p/u/semio/b/l/studio/f/studio.ts/s/SessionKitStore/d/i/createSessionKitStore)
 **/
export async function createSessionKitStore(config: SessionKitStoreConfig): Promise<SessionKitStore> {
  return SessionKitStore.create(config);
}

// #endregion 🔖SessionKitStore

// #region 🔖GranularHooks
// [👤semio👥studio💻studio🔖granularhooks](repo://p/u/semio/b/l/studio/f/studio.ts/s/GranularHooks)
// React hooks for 100% granular entity/collection/property-level subscriptions.
// Specs: Each hook uses useSyncExternalStore under the hood. Hooks subscribe to the
// minimum scope needed and only re-render when that scope changes. For SessionKitStore,
// hooks also subscribe to granular entity/collection/property listeners for optimal updates.
// For other KitStore implementations, hooks fall back to global subscribe with selector comparison.

import type { Author, Concept, Design, Port, Quality, Tag, Type } from "@semio/js";
import { useCallback, useRef, useSyncExternalStore } from "react";

// #region 🔖SelectorHook

/**
 * Core selector hook for any KitStore. Selects a value from the snapshot and only
 * re-renders when the selected value changes (via Object.is comparison).
 *
 * Specs: Uses useSyncExternalStore with a memoized getSnapshot that tracks the
 * previous selected value. Returns the cached value if Object.is(prev, next) is true.
 **/
export function useKitStoreSelector<T>(store: KitStore, selector: (snap: KitStoreSnapshot) => T): T {
  const cachedRef = useRef<{ value: T; initialized: boolean }>({ value: undefined as T, initialized: false });
  const getSnapshot = useCallback(() => {
    const next = selector(store.getSnapshot());
    if (cachedRef.current.initialized && Object.is(cachedRef.current.value, next)) {
      return cachedRef.current.value;
    }
    cachedRef.current = { value: next, initialized: true };
    return next;
  }, [store, selector]);
  const subscribe = useCallback((cb: () => void) => store.subscribe(cb), [store]);
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

// #endregion 🔖SelectorHook

// #region 🔖KitHooks

/**
 * Returns the full Kit snapshot. Re-renders on any kit change.
 **/
export function useKit(store: KitStore): Kit {
  return useKitStoreSelector(store, (s) => s.kit);
}

/**
 * Returns the kit sync state. Re-renders on status changes.
 **/
export function useKitSyncState(store: KitStore): KitSyncState {
  return useKitStoreSelector(store, (s) => s.sync);
}

/**
 * Returns the kit name. Re-renders only when name changes.
 **/
export function useKitName(store: KitStore): string {
  return useKitStoreSelector(store, (s) => s.kit.name);
}

/**
 * Returns the kit version. Re-renders only when version changes.
 **/
export function useKitVersion(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.version);
}

/**
 * Returns the kit description. Re-renders only when description changes.
 **/
export function useKitDescription(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.description);
}

/**
 * Returns the kit icon. Re-renders only when icon changes.
 **/
export function useKitIcon(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.icon);
}

/**
 * Returns the kit image. Re-renders only when image changes.
 **/
export function useKitImage(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.image);
}

/**
 * Returns the kit remote URL. Re-renders only when remote changes.
 **/
export function useKitRemote(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.remote);
}

/**
 * Returns the kit homepage URL. Re-renders only when homepage changes.
 **/
export function useKitHomepage(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.homepage);
}

/**
 * Returns the kit license. Re-renders only when license changes.
 **/
export function useKitLicense(store: KitStore): string | undefined {
  return useKitStoreSelector(store, (s) => s.kit.license);
}

/**
 * Returns any arbitrary kit field by key. Re-renders only when that field changes.
 **/
export function useKitField<K extends keyof Kit>(store: KitStore, field: K): Kit[K] {
  return useKitStoreSelector(store, (s) => s.kit[field]);
}

// #endregion 🔖KitHooks

// #region 🔖CollectionHooks

/**
 * Returns all types. Re-renders when types collection changes.
 **/
export function useTypes(store: KitStore): Type[] {
  return useKitStoreSelector(store, (s) => s.kit.types ?? []);
}

/**
 * Returns a single type by guid. Re-renders only when that type changes.
 **/
export function useType(store: KitStore, typeGuid: string): Type | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.types ?? []).find((t) => t.guid === typeGuid));
}

/**
 * Returns a single type field by guid and field key. Re-renders only when that field changes.
 **/
export function useTypeField<K extends keyof Type>(store: KitStore, typeGuid: string, field: K): Type[K] | undefined {
  return useKitStoreSelector(store, (s) => {
    const t = (s.kit.types ?? []).find((t) => t.guid === typeGuid);
    return t ? t[field] : undefined;
  });
}

/**
 * Returns all designs. Re-renders when designs collection changes.
 **/
export function useDesigns(store: KitStore): Design[] {
  return useKitStoreSelector(store, (s) => s.kit.designs ?? []);
}

/**
 * Returns a single design by guid. Re-renders only when that design changes.
 **/
export function useDesign(store: KitStore, designGuid: string): Design | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.designs ?? []).find((d) => d.guid === designGuid));
}

/**
 * Returns a single design field by guid and field key. Re-renders only when that field changes.
 **/
export function useDesignField<K extends keyof Design>(store: KitStore, designGuid: string, field: K): Design[K] | undefined {
  return useKitStoreSelector(store, (s) => {
    const d = (s.kit.designs ?? []).find((d) => d.guid === designGuid);
    return d ? d[field] : undefined;
  });
}

/**
 * Returns all authors. Re-renders when authors collection changes.
 **/
export function useAuthors(store: KitStore): Author[] {
  return useKitStoreSelector(store, (s) => s.kit.authors ?? []);
}

/**
 * Returns a single author by guid. Re-renders only when that author changes.
 **/
export function useAuthor(store: KitStore, authorGuid: string): Author | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.authors ?? []).find((a) => a.guid === authorGuid));
}

/**
 * Returns all tags. Re-renders when tags collection changes.
 **/
export function useTags(store: KitStore): Tag[] {
  return useKitStoreSelector(store, (s) => s.kit.tags ?? []);
}

/**
 * Returns a single tag by guid. Re-renders only when that tag changes.
 **/
export function useTag(store: KitStore, tagGuid: string): Tag | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.tags ?? []).find((t) => t.guid === tagGuid));
}

/**
 * Returns all concepts. Re-renders when concepts collection changes.
 **/
export function useConcepts(store: KitStore): Concept[] {
  return useKitStoreSelector(store, (s) => s.kit.concepts ?? []);
}

/**
 * Returns a single concept by guid. Re-renders only when that concept changes.
 **/
export function useConcept(store: KitStore, conceptGuid: string): Concept | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.concepts ?? []).find((c) => c.guid === conceptGuid));
}

/**
 * Returns all ports. Re-renders when ports collection changes.
 **/
export function usePorts(store: KitStore): Port[] {
  return useKitStoreSelector(store, (s) => s.kit.ports ?? []);
}

/**
 * Returns a single port by guid. Re-renders only when that port changes.
 **/
export function usePort(store: KitStore, portGuid: string): Port | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.ports ?? []).find((p) => p.guid === portGuid));
}

/**
 * Returns all qualities. Re-renders when qualities collection changes.
 **/
export function useQualities(store: KitStore): Quality[] {
  return useKitStoreSelector(store, (s) => s.kit.qualities ?? []);
}

/**
 * Returns a single quality by guid. Re-renders only when that quality changes.
 **/
export function useQuality(store: KitStore, qualityGuid: string): Quality | undefined {
  return useKitStoreSelector(store, (s) => (s.kit.qualities ?? []).find((q) => q.guid === qualityGuid));
}

// #endregion 🔖CollectionHooks

// #region 🔖PresenceHooks

/**
 * Returns all current presences. Re-renders when any presence changes.
 * Only works with SessionKitStore.
 **/
export function usePresences(store: KitStore): PresenceState[] {
  const subscribe = useCallback(
    (cb: () => void) => {
      if (store instanceof SessionKitStore) {
        return store.subscribePresence(cb);
      }
      return store.subscribe(cb);
    },
    [store],
  );
  const getSnapshot = useCallback(() => {
    if (store instanceof SessionKitStore) {
      return store.getPresences();
    }
    return [];
  }, [store]);
  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

/**
 * Returns presence for a specific person. Re-renders only when that person's presence changes.
 * Only works with SessionKitStore.
 **/
export function usePresence(store: KitStore, personId: string, frontendId: string): PresenceState | undefined {
  const presences = usePresences(store);
  return presences.find((p) => p.personId === personId && p.frontendId === frontendId);
}

// #endregion 🔖PresenceHooks

// #region 🔖SessionHooks

/**
 * Returns the current domain version. Only works with SessionKitStore.
 **/
export function useDomainVersion(store: KitStore): number {
  return useKitStoreSelector(store, () => {
    if (store instanceof SessionKitStore) return store.getDomainVersion();
    return 0;
  });
}

/**
 * Returns the current semio version. Only works with SessionKitStore.
 **/
export function useSemioVersion(store: KitStore): number {
  return useKitStoreSelector(store, () => {
    if (store instanceof SessionKitStore) return store.getSemioVersion();
    return 0;
  });
}

// #endregion 🔖SessionHooks

// #endregion 🔖GranularHooks
