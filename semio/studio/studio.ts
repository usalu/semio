// #region 🔖Header

// [👤semio👥studio💻studio](semiorepo://p/u/semio/b/l/studio/f/studio.ts)

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

// Specs: Defines abstract reactive data interfaces (RMap, RArray, RDoc) for
// provider-agnostic state management. Provides Yjs implementation and persistence factories.

// Abstract reactive state layer and kit persistence providers for semio sketchpad.

// #endregion Header

// #region 🔖ReactiveInterfaces

/**
 * Event emitted when keys change in an RMap.
 *
 * Specs: Mirrors Y.YMapEvent.keysChanged for provider-agnostic field observation.
 **/
export interface RMapEvent {
  keysChanged: Set<string>;
}

/**
 * Abstract reactive map with key-value storage and change observation.
 *
 * Specs: API mirrors Y.Map. Observe callbacks receive RMapEvent for shallow,
 * or no args for deep. Implementations: Yjs, in-memory, JSON file, SQLite.
 **/
export interface RMap<V = any> {
  get(key: string): V | undefined;
  set(key: string, value: V): void;
  delete(key: string): void;
  has(key: string): boolean;
  toJSON(): Record<string, any>;
  forEach(f: (value: V, key: string, map: any) => void): void;
  observe(f: (event: RMapEvent, ...args: any[]) => void): void;
  unobserve(f: (event: RMapEvent, ...args: any[]) => void): void;
  observeDeep(f: (...args: any[]) => void): void;
  unobserveDeep(f: (...args: any[]) => void): void;
}

/**
 * Abstract reactive array with indexed storage and change observation.
 *
 * Specs: API mirrors Y.Array. Push takes an array of items.
 * Delete takes start index and count. Implementations match RMap.
 **/
export interface RArray<V = any> {
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
 * Abstract reactive document root with named maps/arrays and transactions.
 *
 * Specs: API mirrors Y.Doc. createMap/createArray produce standalone instances
 * that become part of the document when inserted via set/push. Transact batches
 * mutations for atomic updates.
 **/
export interface RDoc {
  createMap<V = any>(): RMap<V>;
  createArray<V = any>(): RArray<V>;
  getMap<V = any>(name?: string): RMap<V>;
  getArray<V = any>(name: string): RArray<V>;
  transact(fn: () => void, origin?: any): void;
  on(event: string, handler: (...args: any[]) => void): void;
  off(event: string, handler: (...args: any[]) => void): void;
}

/**
 * Type guard for RMap instances.
 **/
export function isRMap(value: any): value is RMap {
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
 * Type guard for RArray instances.
 **/
export function isRArray(value: any): value is RArray {
  return value != null && typeof value === "object" && typeof value.toArray === "function" && typeof value.push === "function" && typeof value.observe === "function";
}

/**
 * Factory for creating RDoc instances with a specific backend.
 *
 * Specs: Each call to createDoc produces an independent reactive document.
 * The factory determines the backend (Yjs, in-memory, JSON file, SQLite).
 **/
export type RDocFactory = () => RDoc;

// #endregion ReactiveInterfaces

// #region 🔖YjsImplementation

import * as Y from "yjs";
export { Y };
export { IndexeddbPersistence } from "y-indexeddb";

/**
 * Yjs-backed RDoc implementation wrapping Y.Doc, Y.Map, and Y.Array.
 *
 * Specs: createMap/createArray delegate to Y.Map/Y.Array constructors.
 * getMap/getArray delegate to Y.Doc methods. All reactive interfaces
 * are structurally satisfied by Yjs types.
 **/
class YjsDoc implements RDoc {
  public readonly inner: Y.Doc;
  constructor() {
    this.inner = new Y.Doc();
  }
  createMap<V = any>(): RMap<V> {
    return new Y.Map<V>() as unknown as RMap<V>;
  }
  createArray<V = any>(): RArray<V> {
    return new Y.Array<V>() as unknown as RArray<V>;
  }
  getMap<V = any>(name?: string): RMap<V> {
    return (name !== undefined ? this.inner.getMap(name) : this.inner.getMap()) as unknown as RMap<V>;
  }
  getArray<V = any>(name: string): RArray<V> {
    return this.inner.getArray(name) as unknown as RArray<V>;
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
 * Creates an RDocFactory backed by Yjs CRDT documents.
 **/
export function createYjsDocFactory(): RDocFactory {
  return () => new YjsDoc();
}

/**
 * Extracts the underlying Y.Doc from a YjsDoc RDoc instance.
 * Throws if the RDoc is not a YjsDoc.
 *
 * Specs: Used by persistence and remote providers that need the raw Y.Doc.
 **/
export function getYDoc(rDoc: RDoc): Y.Doc {
  if (rDoc instanceof YjsDoc) return rDoc.inner;
  throw new Error("RDoc is not a YjsDoc");
}

// #endregion YjsImplementation

// #region 🔖Exports

// Re-export Yjs types for backwards compat transition
export type { Doc } from "yjs";

// #endregion Exports

// #region 🔖PersistenceProviders

/**
 * Abstract persistence provider for syncing an RDoc to a storage backend.
 * Implementations MUST call the synced callback once initial data is loaded. Implementations
 * MUST provide a destroy method to release resources.
 **/
export interface PersistenceProvider {
  once(event: "synced", callback: () => void): void;
  on(event: "synced", callback: () => void): void;
  destroy(): void;
}

import { IndexeddbPersistence as IndexeddbPersistenceImpl } from "y-indexeddb";

/**
 * Factory that creates a PersistenceProvider for a given RDoc and storage key.
 **/
export type PersistenceFactory = (rDoc: RDoc, key: string) => PersistenceProvider;

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
 * PersistenceProvider that syncs RDoc state via binary encoding.
 * Uses Y.encodeStateAsUpdate / Y.applyUpdate for round-tripping through an adapter.
 *
 * Specs: Uses binary state encoding for faithful CRDT persistence.
 * The key parameter maps to a storage location via the adapter.
 **/
export class YDocBinaryPersistenceProvider implements PersistenceProvider {
  private doc: Y.Doc;
  private key: string;
  private adapter: { read(key: string): Promise<Uint8Array | null>; write(key: string, data: Uint8Array): Promise<void> };
  private syncedCallbacks: (() => void)[] = [];
  private onceCallbacks: (() => void)[] = [];
  private destroyed = false;
  private updateHandler: (update: Uint8Array) => void;

  constructor(rDoc: RDoc, key: string, adapter: { read(key: string): Promise<Uint8Array | null>; write(key: string, data: Uint8Array): Promise<void> }) {
    this.doc = getYDoc(rDoc);
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
 **/
export function createIndexeddbPersistenceFactory(): PersistenceFactory {
  return (rDoc: RDoc, key: string) => new IndexeddbPersistenceImpl(key, getYDoc(rDoc));
}

/**
 * Creates a PersistenceFactory that persists RDoc state to a JSON file via an adapter.
 *
 * Specs: Wraps YDocBinaryPersistenceProvider with a JSON adapter that converts
 * between binary state and Base64 JSON for human-inspectable storage.
 **/
export function createJsonFilePersistenceFactory(adapter: JsonFileAdapter): PersistenceFactory {
  return (rDoc: RDoc, key: string) => {
    const binaryAdapter = {
      async read(k: string): Promise<Uint8Array | null> {
        const json = await adapter.read(k);
        if (!json) return null;
        const parsed = JSON.parse(json);
        if (parsed?.yDocState) {
          const binary = Uint8Array.from(atob(parsed.yDocState), (c) => c.charCodeAt(0));
          return binary;
        }
        return null;
      },
      async write(k: string, data: Uint8Array): Promise<void> {
        const base64 = btoa(String.fromCharCode(...data));
        await adapter.write(k, JSON.stringify({ yDocState: base64 }));
      },
    };
    return new YDocBinaryPersistenceProvider(rDoc, key, binaryAdapter);
  };
}

/**
 * Creates a PersistenceFactory that persists RDoc state to a SQLite database via an adapter.
 *
 * Specs: Uses binary state encoding stored as a BLOB in a SQLite table.
 * The adapter provides platform-specific SQLite file I/O.
 **/
export function createSqliteFolderPersistenceFactory(adapter: SqliteAdapter): PersistenceFactory {
  return (rDoc: RDoc, key: string) => {
    return new YDocBinaryPersistenceProvider(rDoc, key, adapter);
  };
}

// #endregion PersistenceProviders
