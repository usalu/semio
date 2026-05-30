// #region 🧲Header

// 2024-2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>

// Render-agnostic sketchpad platform, domain sync, and kit helpers (@semio/sketchpad).

// #endregion 🧲Header

//#region 🔌Adapters
import type { Author, Concept, Connection, Connector, Design, Folder, Piece, Port, Quality, Representation, File as SemioFile, Tag, Type } from "@semio/js";
import { Camera, Kit, Plane, Session } from "@semio/js";
import { gunzipSync } from "fflate";
import { Euler, Matrix4, Vector3 } from "three";
import {
	CommandBus,
	Controller,
	PluginHost,
	Platform,
	buildPuzzle2dWindowBody,
	buildPuzzle5dWindowBody,
	buildCadWindowBody,
	buildPanelWindowBody,
	Component,
	Table,
	buildTableWindowBody,
	createDefaultLayout,
	createTabStackLayout,
	registerPlatformComponent,
	registerSidePanelBody,
	registerWindowBody,
	type ComponentKind,
	type CadModel,
	type PanelModel,
	type Puzzle2dModel,
	type Puzzle3dModel,
	type Puzzle5dModel,
	type TableModel,
	type PluginManifest,
	type PluginModule,
	type PlatformSpec,
	type WindowBodyViewContext,
	type UiNode,
} from "@framework/platform/core";
//#endregion 🔌Adapters



/** @emoji ðŸ‘Ÿ Registry kit-store factory until sketchpad runs purely on {@link SessionContextProvider}. */
export type SketchpadKitStoreFactory = (kit?: Kit) => Promise<KitHostStore>;

export type SketchpadKitKindAvailability = Readonly<{
  temporary: boolean;
  file: boolean;
  folder: boolean;
  remote: boolean;
}>;

/** @emoji ðŸ—„ Mutable kit snapshot host used by the registry bridge (being replaced by graph-tier providers). */
export type KitHostStore = {
  getSnapshot: () => { kit: Kit };
  subscribe?: (listener: () => void) => () => void;
  replace?: (next: Kit) => void;
} & Record<string, unknown>;

export type KitCommandContext = Record<string, unknown>;
export type KitHostGraphOperation = unknown;
export type KitFolderAdapter = Record<string, unknown>;
export type KitJsonFileAdapter = Record<string, unknown>;
export type KitRegistryValue = Record<string, unknown>;

/** @emoji ðŸ§¾ Recursively flattens `{ items: [...] }` and Relay `edges` for GraphQL `installProjection` payloads. */
function semioDenormalizeBundleValue(v: unknown): unknown {
  if (v == null || typeof v !== "object") return v;
  if (Array.isArray(v)) return v.map(semioDenormalizeBundleValue);
  const o = v as SemioBundleJson;
  if (Array.isArray(o["items"])) return (o["items"] as unknown[]).map(semioDenormalizeBundleValue);
  if (Array.isArray(o["edges"])) {
    const out: unknown[] = [];
    for (const e of o["edges"] as unknown[]) {
      if (e != null && typeof e === "object" && !Array.isArray(e) && "node" in (e as SemioBundleJson)) {
        out.push(semioDenormalizeBundleValue((e as SemioBundleJson)["node"]));
      }
    }
    return out;
  }
  const flat: SemioBundleJson = {};
  for (const [k, val] of Object.entries(o)) flat[k] = semioDenormalizeBundleValue(val) as never;
  return flat;
}

/** @emoji ðŸ§¾ Lifts `*.kit.semio.json` (`initialKit` / `wip.initialKit`) then flattens bundle lists for inline JSON bootstrap. */
export function decodeKitSemioEnvelopeToFullFromValue(v: unknown): unknown {
  let inner: unknown = v;
  if (inner && typeof inner === "object" && !Array.isArray(inner)) {
    const top = inner as SemioBundleJson;
    if (top["initialKit"] != null && typeof top["initialKit"] === "object" && !Array.isArray(top["initialKit"])) {
      inner = top["initialKit"];
    } else if (top["wip"] != null && typeof top["wip"] === "object" && !Array.isArray(top["wip"])) {
      const wr = (top["wip"] as SemioBundleJson)["initialKit"];
      if (wr != null && typeof wr === "object" && !Array.isArray(wr)) inner = wr;
    }
  }
  return semioDenormalizeBundleValue(inner);
}

function colorStringForIdText(text: string): string {
  let h = 0;
  for (let i = 0; i < text.length; i++) h = (h * 31 + text.charCodeAt(i)) | 0;
  const hue = Math.abs(h) % 360;
  return `hsl(${hue} 70% 45%)`;
}

export function colorPortsForTypes(types: Type[] | undefined) {
  if (!types?.length) return { updated: [] as { type: { id: string }; diff: any }[] };
  return {
    updated: types.map((t) => ({
      type: { id: t.id },
      diff: {
        connectors: {
          updated: ((t as any).connectors ?? ([] as Connector[])).map((c: Connector) => ({
            connector: { id: c.id },
            diff: { color: colorStringForIdText(c.id) },
          })),
        },
      },
    })),
  };
}

export function findTypeInKit(kitOrTypes: Kit | readonly Type[], typeId: string | null | undefined): Type | undefined {
  if (!typeId) return undefined;
  if (Array.isArray(kitOrTypes)) return kitOrTypes.find((t) => t.id === typeId);
  return (kitOrTypes as { types?: readonly Type[] }).types?.find((t) => t.id === typeId);
}

export function findDesignInKit(kitOrDesigns: Kit | readonly Design[], designId: string | null | undefined): Design | undefined {
  if (!designId) return undefined;
  if (Array.isArray(kitOrDesigns)) return kitOrDesigns.find((d) => d.id === designId);
  return (kitOrDesigns as { designs?: readonly Design[] }).designs?.find((d) => d.id === designId);
}

export function findPieceInDesign(design: Design, pieceId: string | null | undefined) {
  if (!pieceId) return undefined;
  return (design as any).pieces?.find((p: Piece) => p.id === pieceId) as Piece | undefined;
}

export function findRepresentation(t: Type, fileId: string | null | undefined): Representation | undefined {
  if (!fileId) return undefined;
  return (t as any).representations?.find((r: Representation) => (r as any).file === fileId || (r as any).file?.id === fileId) as Representation | undefined;
}

export function selectBestRepresentation(representations: Representation[] | null | undefined): Representation | null {
  if (!representations?.length) return null;
  return representations[0] ?? null;
}

export function areSameConnection(a: Connection, b: Connection): boolean {
  return a.id === b.id;
}

export function areDesignsInSameFamily(kit: Kit, a: Design, b: Design): boolean {
  const pa = a.pieces?.[0] && (a.pieces[0] as any).type ? findTypeInKit(kit, (a.pieces[0] as any).type.id) : undefined;
  const pb = b.pieces?.[0] && (b.pieces[0] as any).type ? findTypeInKit(kit, (b.pieces[0] as any).type.id) : undefined;
  const fa = (pa as any)?.families?.[0]?.id;
  const fb = (pb as any)?.families?.[0]?.id;
  return Boolean(fa && fb && fa === fb);
}

export function arePortsCompatible(kit: Kit, a: Port, b: Port): boolean {
  if (a.compatiblePorts?.length && a.compatiblePorts.some((x) => x.id === b.id)) return true;
  if (b.compatiblePorts?.length && b.compatiblePorts.some((x) => x.id === a.id)) return true;
  const af = a.compatibleFamilies?.map((x) => x.id) ?? [];
  const bf = b.compatibleFamilies?.map((x) => x.id) ?? [];
  for (const fam of kit.families ?? [])
    for (const p of fam.ports ?? []) {
      if (p.id === a.id)
        for (const id of bf) {
          if (fam.id === id) return true;
        }
      if (p.id === b.id)
        for (const id of af) {
          if (fam.id === id) return true;
        }
    }
  return false;
}

export function getClusterableGroups(_kit: Kit, _designId: string, _selectedPieceIds: readonly string[]): string[][] {
  return [];
}

export function getIncludedDesigns(allDesigns: readonly Design[], design: Design): Design[] {
  const out: Design[] = [];
  const seen = new Set<string>();
  const visit = (d: Design) => {
    if (seen.has(d.id)) return;
    seen.add(d.id);
    out.push(d);
    for (const p of (d as { pieces?: readonly { includedInDesigns?: string[] }[] }).pieces ?? []) {
      const ins = p.includedInDesigns;
      if (ins) for (const childId of ins) {
        const d2 = findDesignInKit(allDesigns, childId);
        if (d2) visit(d2);
      }
    }
  };
  visit(design);
  return out.filter((d) => d.id !== design.id);
}
export function sumQualityInDesign(design: Design): number {
  let s = 0;
  for (const p of (design as any).pieces || []) for (const q of (p as any).stats || []) s += Number((q as any).value) || 0;
  return s;
}

export function generateUniqueName(base: string, used: string[] | Set<string>) {
  const u = used instanceof Set ? used : new Set(used);
  if (!u.has(base)) return base;
  for (let i = 2; i < 1_000_000; i++) {
    const c = `${base} (${i})`;
    if (!u.has(c)) return c;
  }
  return `${base}-${Date.now()}`;
}

export type FileTreeNode = {
  name: string;
  path: string;
  parentPath?: string;
  isDirectory: boolean;
  level: number;
  isExpanded: boolean;
  children: FileTreeNode[];
  file?: SemioFile;
};

export function buildFileTree(folders: Folder[], files: SemioFile[]): FileTreeNode[] {
  const byParent = new Map<string | undefined, Folder[]>();
  for (const f of folders) {
    const p = f.path.split("/").slice(0, -1).join("/") || undefined;
    const k = f.path.includes("/") ? f.path.slice(0, f.path.lastIndexOf("/")) : "";
    const parent: string | undefined = k || undefined;
    if (!byParent.has(parent)) byParent.set(parent, []);
    byParent.get(parent)!.push(f);
  }
  const rootFiles = files.filter((x) => !x.folder);
  const nodes: FileTreeNode[] = [];
  for (const f of byParent.get(undefined) || byParent.get("") || []) {
    nodes.push({ name: f.name, path: f.path, isDirectory: true, level: 0, isExpanded: false, children: [] });
  }
  for (const f of files) {
    const folderPath = f.folder && typeof f.folder === "object" && "id" in f.folder ? folders.find((x) => x.id === (f.folder as { id: string }).id) : null;
    const path = folderPath ? `${folderPath.path}/${f.id}` : f.id;
    nodes.push({ name: f.name, path, parentPath: folderPath?.path, isDirectory: false, level: 0, isExpanded: false, children: [], file: f });
  }
  void rootFiles;
  return nodes;
}

export function flattenFileTree(tree: FileTreeNode[], level: number, expandedRows: string[]): FileTreeNode[] {
  const out: FileTreeNode[] = [];
  for (const n of tree) {
    const isExpanded = expandedRows.includes(n.path) || n.isExpanded;
    out.push({ ...n, level, isExpanded });
    if (n.isDirectory && n.children.length && isExpanded) out.push(...flattenFileTree(n.children, level + 1, expandedRows));
  }
  return out;
}

const _eulerToThree = new Euler(-Math.PI / 2, 0, 0, "XYZ");
const _mToThree = new Matrix4();
const _eulerToSemio = new Euler(Math.PI / 2, 0, 0, "XYZ");
const _mToSemio = new Matrix4();

export function toThreeRotation(): Matrix4 {
  return _mToThree.makeRotationFromEuler(_eulerToThree);
}

export function toSemioRotation(): Matrix4 {
  return _mToSemio.makeRotationFromEuler(_eulerToSemio);
}

export function planeToMatrix(plane: Plane | { origin: { x: number; y: number; z: number }; xAxis: { x: number; y: number; z: number }; yAxis: { x: number; y: number; z: number } }): Matrix4 {
  const o = plane.origin;
  const x = new Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z).normalize();
  const y0 = new Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
  const z = new Vector3().crossVectors(x, y0).normalize();
  const y = new Vector3().crossVectors(z, x).normalize();
  return new Matrix4().makeBasis(x, y, z).setPosition(o.x, o.y, o.z);
}

/**
 * @emoji ðŸ“¦ Decode gzip-or-JSON kit bytes into a live {@link Kit} handle via GraphQL {@link Store.installProjection}.
 */
export async function importKit(data: ArrayBuffer | Blob | File | string): Promise<{ kit: Kit; session: Session }> {
  let bytes: Uint8Array;
  if (typeof data === "string") {
    const res = await fetch(data);
    bytes = new Uint8Array(await res.arrayBuffer());
  } else if (data instanceof ArrayBuffer) {
    bytes = new Uint8Array(data);
  } else {
    bytes = new Uint8Array(await data.arrayBuffer());
  }
  if (bytes.length >= 2 && bytes[0] === 0x1f && bytes[1] === 0x8b) {
    bytes = gunzipSync(bytes);
  }
  const text = new TextDecoder().decode(bytes);
  const plainUnknown = decodeKitSemioEnvelopeToFullFromValue(JSON.parse(text));
  const payload = typeof plainUnknown === "object" && plainUnknown != null ? JSON.stringify(plainUnknown) : String(plainUnknown);
  const session = await Session.openInMemory();
  const stores = await session.stores();
  if (stores.length === 0) throw new Error("semio/sketchpad: importKit found zero stores after openInMemory");
  const store = stores[0]!;
  const installed = await store.installProjection(payload);
  if (!installed.ok) throw new Error(`semio/sketchpad: importKit installProjection failed: ${installed.error?.message ?? "unknown"}`);
  const kit = await store.wip().theKit().kit();
  return { kit, session };
}

// #endregion ðŸ”–SketchpadKitUiHelpers

// #region ðŸª¬SyncInterfaces
// Synchronized state interfaces for backend-agnostic state management.

/**
 * Event emitted when fields change in a SyncMap.
 **/
export interface SyncMapEvent {
  keysChanged: Set<string>;
}

/**
 * A synchronized field store supporting key-value access and change observation.
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
 **/
export function isSyncArray(value: any): value is SyncArray {
  return value != null && typeof value === "object" && typeof value.toArray === "function" && typeof value.push === "function" && typeof value.observe === "function";
}

/**
 * Factory for creating SyncDoc instances.
 **/
export type SyncDocFactory = () => SyncDoc;

// #endregion ðŸª¬SyncInterfaces

// #region ðŸ“°MemorySyncBackend
// In-memory SyncDoc / SyncMap / SyncArray (Yjs removed; sketchpad UI state is local + KitStore in wasm).

class MemoryMap<V> implements SyncMap<V> {
  private readonly _data = new Map<string, V>();
  private readonly _observers = new Set<(e: SyncMapEvent) => void>();
  private readonly _deep = new Set<() => void>();
  /** When this map is nested in another, parent registers here so their observeDeep runs. */
  readonly _bubblers = new Set<() => void>();
  /** Stable childâ†’parent bubble per key; removed when key is overwritten or deleted. */
  private readonly _nestedBubbles = new Map<string, () => void>();

  constructor(readonly _doc: MemoryDoc) {}

  _ping() {
    for (const f of this._deep) f();
    for (const b of this._bubblers) b();
  }

  get(key: string): V | undefined {
    return this._data.get(key);
  }
  set(key: string, value: V): void {
    this.detachNestedBubble(key);
    this._data.set(key, value);
    if (value instanceof MemoryMap) {
      const b = () => {
        this._ping();
      };
      value._bubblers.add(b);
      this._nestedBubbles.set(key, b);
    } else if (value instanceof MemoryArray) {
      const b = () => {
        this._ping();
      };
      value._bubblers.add(b);
      this._nestedBubbles.set(key, b);
    }
    const ev: SyncMapEvent = { keysChanged: new Set([key]) };
    for (const o of this._observers) o(ev);
    this._ping();
  }
  delete(key: string): void {
    if (!this._data.has(key)) return;
    this.detachNestedBubble(key);
    this._data.delete(key);
    const ev: SyncMapEvent = { keysChanged: new Set([key]) };
    for (const o of this._observers) o(ev);
    this._ping();
  }

  private detachNestedBubble(key: string): void {
    const b = this._nestedBubbles.get(key);
    if (!b) return;
    const prev = this._data.get(key);
    if (prev instanceof MemoryMap) prev._bubblers.delete(b);
    else if (prev instanceof MemoryArray) prev._bubblers.delete(b);
    this._nestedBubbles.delete(key);
  }
  has(key: string): boolean {
    return this._data.has(key);
  }
  forEach(f: (value: V, key: string, map: any) => void): void {
    for (const [k, v] of this._data) f(v, k, this);
  }
  toJSON(): Record<string, any> {
    const o: Record<string, any> = {};
    for (const [k, v] of this._data) {
      o[k] = isSyncMap(v) ? (v as SyncMap).toJSON() : isSyncArray(v) ? (v as SyncArray).toJSON() : v;
    }
    return o;
  }
  observe(f: (event: SyncMapEvent, ...args: any[]) => void): void {
    this._observers.add(f as (e: SyncMapEvent) => void);
  }
  unobserve(f: (event: SyncMapEvent, ...args: any[]) => void): void {
    this._observers.delete(f as (e: SyncMapEvent) => void);
  }
  observeDeep(f: (...args: any[]) => void): void {
    this._deep.add(f);
  }
  unobserveDeep(f: (...args: any[]) => void): void {
    this._deep.delete(f);
  }
}

class MemoryArray<V> implements SyncArray<V> {
  private _items: V[] = [];
  private readonly _observers = new Set<() => void>();
  private readonly _deep = new Set<() => void>();
  readonly _bubblers = new Set<() => void>();

  constructor(readonly _doc: MemoryDoc) {}

  _ping() {
    for (const f of this._observers) f();
    for (const f of this._deep) f();
    for (const b of this._bubblers) b();
  }

  get length() {
    return this._items.length;
  }
  get(index: number): V {
    return this._items[index];
  }
  push(items: V[]): void {
    this._items.push(...items);
    this._ping();
  }
  delete(start: number, count: number): void {
    this._items.splice(start, count);
    this._ping();
  }
  toArray(): V[] {
    return [...this._items];
  }
  toJSON(): any[] {
    return this._items.map((v) => (isSyncMap(v) ? (v as SyncMap).toJSON() : isSyncArray(v) ? (v as SyncArray).toJSON() : v));
  }
  forEach(f: (value: V, index: number, array: any) => void): void {
    this._items.forEach((v, i) => f(v, i, this));
  }
  observe(f: (...args: any[]) => void): void {
    this._observers.add(f);
  }
  unobserve(f: (...args: any[]) => void): void {
    this._observers.delete(f);
  }
  observeDeep(f: (...args: any[]) => void): void {
    this._deep.add(f);
  }
  unobserveDeep(f: (...args: any[]) => void): void {
    this._deep.delete(f);
  }
}

class MemoryDoc implements SyncDoc {
  private readonly _maps = new Map<string, MemoryMap<any>>();
  private readonly _arrays = new Map<string, MemoryArray<any>>();

  createMap<V = any>(): SyncMap<V> {
    return new MemoryMap(this);
  }
  createArray<V = any>(): SyncArray<V> {
    return new MemoryArray(this);
  }
  getMap<V = any>(name?: string): SyncMap<V> {
    const k = name !== undefined && name !== "" ? name : "";
    if (!this._maps.has(k)) this._maps.set(k, new MemoryMap(this));
    return this._maps.get(k)! as SyncMap<V>;
  }
  getArray<V = any>(name: string): SyncArray<V> {
    if (!this._arrays.has(name)) this._arrays.set(name, new MemoryArray(this));
    return this._arrays.get(name)! as SyncArray<V>;
  }
  transact(fn: () => void, _origin?: any): void {
    fn();
  }
  on(_event: string, _handler: (...args: any[]) => void): void {}
  off(_event: string, _handler: (...args: any[]) => void): void {}
}

/**
 * In-memory only; kit authority is `semio/rs` via `@semio/react` â†’ `@semio/js` `KitStore` (Worker).
 */
export function createSyncDocFactory(): SyncDocFactory {
  return () => new MemoryDoc();
}

// #endregion ðŸ“°MemorySyncBackend

// #region ðŸ‚PersistenceProviders
// Sketchpad UI state is in-memory; providers mark synced immediately (no Yjs binary round-trip).

/**
 * Abstract persistence provider for syncing a SyncDoc to a storage backend.
 **/
export interface PersistenceProvider {
  once(event: "synced", callback: () => void): void;
  on(event: "synced", callback: () => void): void;
  destroy(): void;
}

/**
 * Factory that creates a PersistenceProvider for a given SyncDoc and storage key.
 **/
export type PersistenceFactory = (syncDoc: SyncDoc, key: string) => PersistenceProvider;

// #endregion ðŸ‚PersistenceProviders

// #endregion 🔌Adapters

// #region ðŸ“Shared

/** @emoji ðŸªª Fresh UUID string for sketchpad-scoped entities (browser {@link crypto.randomUUID}). */
function id(): string {
  return crypto.randomUUID();
}

// #region âš™ï¸Types

// #region â­SyncPath Types
// MUST define path segment and path types for navigating sync document structures.

/**
 * A single segment in a sync document path, either a map key, array index, or array item by ID.
 **/
export type SyncPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

/**
 * An ordered sequence of SyncPathSegment values describing a path through a sync document.
 **/
export type SyncPath = SyncPathSegment[];

// #endregion â­SyncPath Types

// #region ðŸŽ™ï¸Granular Hook Types
// MUST define hook result tuples and field abstractions for granular reactive state access.

/**
 * A readonly tuple of value, optional setter, and canSet flag for granular hook access.
 **/
export type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];

/**
 * A readonly tuple of value, undefined setter, and canSet flag for read-only hook access.
 **/
export type HookNoSetResult<T> = readonly [T, undefined, boolean];

/**
 * Sentinel undefined value indicating that a hook result has no setter.
 **/
export const READONLY_SETTER = undefined as undefined;
/**
 * Sentinel false value indicating that a hook result is read-only.
 **/
export const READONLY_CAN = false;

/**
 * Wraps a value into a read-only HookResult tuple with no setter.
 * MUST return a frozen readonly tuple with undefined setter and false canSet.
 **/
export function readonlyHookResult<T>(value: T): HookResult<T> {
  return [value, READONLY_SETTER, READONLY_CAN] as const;
}

/**
 * Wraps a value and setter into a writable HookResult tuple.
 * MUST return a tuple with the setter included only when canSet is true.
 **/
export function writableHookResult<T>(value: T, setter: (value: T) => void, canSet: boolean = true): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * Wraps a value into a HookResult tuple with a setter conditional on canSet.
 * MUST return a tuple with the setter conditional on the canSet flag.
 **/
export function conditionalHookResult<T>(canSet: boolean, value: T, setter: ((value: T) => void) | undefined): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

/**
 * A reactive field with a value, canSet flag, and setter function.
 **/
export interface Field<T> {
  value: T;
  canSet: boolean;
  set: (next: T) => void;
}

/**
 * A reactive action field with canExecute flag and execute function.
 **/
export interface ActionField {
  canExecute: boolean;
  execute: () => void;
}

/**
 * NOOP_SETTER holds the data fields for a NOOP_SETTER record.
 **/
const NOOP_SETTER = () => {
  if (process.env.NODE_ENV === "development") {
    console.warn("[DEBUG] Attempted to set a disabled field");
  }
};

/**
 * Constructs a Field with a value, setter, and canSet flag.
 * MUST use the provided setter when canSet is true, otherwise a no-op setter.
 **/
export function createField<T>(value: T, setter: (next: T) => void, canSet: boolean): Field<T> {
  return {
    value,
    canSet,
    set: canSet ? setter : NOOP_SETTER,
  };
}

/**
 * Constructs a read-only Field with a fixed value and no-op setter.
 * MUST set canSet to false and use a no-op setter.
 **/
export function createReadonlyField<T>(value: T): Field<T> {
  return {
    value,
    canSet: false,
    set: NOOP_SETTER,
  };
}

/**
 * Constructs an ActionField with a guarded execute function.
 * MUST guard execute behind canExecute, logging a warning in dev mode when disabled.
 **/
export function createAction(execute: () => void, canExecute: boolean): ActionField {
  return {
    canExecute,
    execute: canExecute
      ? execute
      : () => {
          if (process.env.NODE_ENV === "development") {
            console.warn("[DEBUG] Attempted to execute a disabled action");
          }
        },
  };
}

/**
 * Converts a Field to a HookResult tuple.
 * MUST convert the field's canSet and set properties into the hook result format.
 **/
export function fieldToHookResult<T>(field: Field<T>): HookResult<T> {
  return [field.value, field.canSet ? field.set : undefined, field.canSet] as const;
}

/**
 * Converts a HookResult tuple back to a Field.
 * MUST reconstruct a Field from the tuple, using a no-op setter when undefined.
 **/
export function hookResultToField<T>(result: HookResult<T>): Field<T> {
  const [value, setter, canSet] = result;
  return {
    value,
    canSet,
    set: setter ?? NOOP_SETTER,
  };
}

// #endregion ðŸŽ™ï¸Granular Hook Types

// #region â²ï¸Standard Empty Constants
// MUST provide frozen singleton constants for empty collections and default panel visibility.

/**
 * Frozen empty array singleton for default array values.
 **/
export const EMPTY_ARRAY: readonly any[] = Object.freeze([]);
/**
 * Frozen empty object singleton for default record values.
 **/
export const EMPTY_OBJECT: Readonly<Record<string, never>> = Object.freeze({});
/**
 * Frozen empty Id array singleton for default id collections.
 **/
export const EMPTY_ID_ARRAY: readonly Id[] = Object.freeze([]);
/**
 * Frozen empty string array singleton for default string collections.
 **/
export const EMPTY_STRING_ARRAY: readonly string[] = Object.freeze([]);

/**
 * Frozen default panel visibility with side and detail panels closed.
 **/
export const EMPTY_PANEL_VISIBILITY: Readonly<PanelVisibility> = Object.freeze({
  toolbar: true,
  leftSidePanel: false,
  rightSidePanel: false,
  details: false,
  chat: false,
  settings: false,
});

// #endregion â²ï¸Standard Empty Constants

// #region ðŸŽ‹Generic Diff Types
// MUST define generic array and selection diff types with apply and inverse operations.

/**
 * Describes added and removed items for an array diff operation.
 **/
export interface ArrayDiff<T> {
  added?: T[];
  removed?: T[];
}

/**
 * Maps selection keys to their corresponding array diffs.
 **/
export type SelectionDiff<TSelection extends Record<string, any[]>> = {
  [K in keyof TSelection]?: ArrayDiff<TSelection[K][number]>;
};

/**
 * Inverts an array diff by swapping added and removed items.
 * MUST swap added and removed arrays to produce the inverse diff.
 **/
export function inverseArrayDiff<T>(diff: ArrayDiff<T>): ArrayDiff<T> {
  const inverse: ArrayDiff<T> = {};
  if (diff.added) inverse.removed = diff.added;
  if (diff.removed) inverse.added = diff.removed;
  return inverse;
}

/**
 * Inverts all array diffs within a selection diff.
 * MUST apply inverseArrayDiff to each key in the selection diff.
 **/
export function inverseSelectionDiff<T extends Record<string, ArrayDiff<any>>>(diff: T): T {
  const inverse = {} as T;
  for (const key in diff) {
    if (Object.prototype.hasOwnProperty.call(diff, key)) {
      inverse[key] = inverseArrayDiff(diff[key]) as T[typeof key];
    }
  }
  return inverse;
}

/**
 * Applies an array diff to a current array, removing then adding items.
 * MUST remove items first, then add non-duplicate items.
 **/
export function applyArrayDiff<T>(current: T[] | undefined, diff: ArrayDiff<T>): T[] {
  let result = current ? [...current] : [];
  if (diff.removed) result = result.filter((item) => !diff.removed!.includes(item));
  if (diff.added) result = [...result, ...diff.added.filter((item) => !result.includes(item))];
  return result;
}

/**
 * Applies a selection diff to a partial selection state.
 * MUST apply the array diff for each key present in the selection diff.
 **/
export function applySelectionDiff<TSelection extends Record<string, any[]>>(current: Partial<TSelection>, diff: SelectionDiff<TSelection>): Partial<TSelection> {
  const result = { ...current } as Partial<TSelection>;
  for (const key in diff) {
    if (Object.prototype.hasOwnProperty.call(diff, key)) {
      const typedKey = key as keyof TSelection;
      result[typedKey] = applyArrayDiff(current[typedKey], diff[typedKey]!) as TSelection[typeof typedKey];
    }
  }
  return result;
}

// #endregion ðŸŽ‹Generic Diff Types

/**
 * A string alias representing a URL.
 **/
export type Url = string;

/**
 * A callback subscription function that returns an unsubscribe disposer.
 **/
export type Subscribe = (callback: () => void) => () => void;

/**
 * A cleanup function that disposes of a resource.
 **/
export type Disposable = () => void;

/**
 * A function that executes a mutation within a transaction with optional origin.
 **/
export type Transact = (fn: () => void, origin?: string) => void;

/**
 * A function that unsubscribes a previously registered callback.
 **/
export type Unsubscribe = () => void;

/**
 * A factory function that creates a sync document provider for a given ID.
 **/
export type SyncProviderFactory = (doc: SyncDoc, id: string) => Promise<void>;

/**
 * A string alias identifying the kind of an app.
 **/
export type AppKind = string;

/**
 * Union type for desktop, tablet, or mobile device contexts.
 **/
export type Device = "desktop" | "tablet" | MobileDevice;

/**
 * Union of all panel identifier strings including side panels.
 **/
export type PanelKey = "workbench" | "details" | "tools" | "stats" | "console" | "toolbar" | "leftSidePanel" | "rightSidePanel";

/**
 * Union of left and right side panel keys.
 **/
export type SidePanelKey = "leftSidePanel" | "rightSidePanel";

/**
 * A string alias for a hotkey path identifier.
 **/
export type HotkeyPath = string;

/**
 * A string alias for a hotkey binding value.
 **/
export type HotkeyValue = string;

/**
 * A record mapping hotkey paths to their override values.
 **/
export type HotkeyOverrides = Record<HotkeyPath, HotkeyValue>;

/**
 * A factory function that creates a FileProvider for a given kit ID.
 **/
export type FileProviderFactory = (kitId: string) => Promise<FileProvider>;

/**
 * A string alias for a Y.js-compatible UUID.
 **/
export type SyncUuid = string;

/**
 * A sync array of UUID strings.
 **/
export type SyncUuidArray = SyncArray<SyncUuid>;

/**
 * A string alias for a Y.js concept name.
 **/
export type SyncConcept = string;

/**
 * A sync array of concept name strings.
 **/
export type SyncConcepts = SyncArray<string>;

/**
 * A sync array of strings.
 **/
export type SyncStringArray = SyncArray<string>;

/**
 * A sync map with string leaf values.
 **/
export type SyncLeafMapString = SyncMap<string>;

/**
 * A sync map with number leaf values.
 **/
export type SyncLeafMapNumber = SyncMap<number>;

/**
 * A sync array of sync maps representing attribute key-value pairs.
 **/
export type SyncAttributes = SyncArray<SyncMap<string>>;

// #endregion âš™ï¸Types

// #region ðŸ“©Enums
// MUST enumerate theme, expertise, mode, store status, tool, window, and panel kinds.

/**
 * Available UI theme options: system, light, or dark.
 **/
export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

/**
 * User expertise levels: beginner, normal, or expert.
 **/
export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

/**
 * Application modes: user or dev.
 **/
export enum Mode {
  USER = "user",
  DEV = "dev",
}

/**
 * Store lifecycle states: idle, loading, error, or ready.
 **/
export enum StoreStatus {
  IDLE = "idle",
  LOADING = "loading",
  ERROR = "error",
  READY = "ready",
}

/**
 * Available tool kinds for selection, lasso, connector, and hand interactions.
 **/
export enum ToolKind {
  SELECTION_NORMAL = "selection-normal",
  SELECTION_ADDITIVE = "selection-additive",
  SELECTION_SUBTRACTIVE = "selection-subtractive",
  SELECTION_INTERSECT = "selection-intersect",
  LASSO_RECTANGULAR = "lasso-rectangular",
  LASSO_FREEFORM = "lasso-freeform",
  CONNECTOR = "connector",
  HAND = "hand",
}

export type SelectionCompositionKind = "replace" | "additive" | "subtractive" | "intersect";

export interface SelectionKeyboardState {
  shiftKey?: boolean;
  altKey?: boolean;
  ctrlKey?: boolean;
  metaKey?: boolean;
}

export function isSelectionToolKind(toolKind: ToolKind | string): boolean {
  return toolKind === ToolKind.SELECTION_NORMAL || toolKind === ToolKind.SELECTION_ADDITIVE || toolKind === ToolKind.SELECTION_SUBTRACTIVE || toolKind === ToolKind.SELECTION_INTERSECT;
}

export function resolveSelectionCompositionKindFromTool(toolKind: ToolKind | string): SelectionCompositionKind {
  if (toolKind === ToolKind.SELECTION_ADDITIVE) return "additive";
  if (toolKind === ToolKind.SELECTION_SUBTRACTIVE) return "subtractive";
  if (toolKind === ToolKind.SELECTION_INTERSECT) return "intersect";
  return "replace";
}

export function resolveSelectionCompositionKind(toolKind: ToolKind | string, keyboard?: SelectionKeyboardState): SelectionCompositionKind {
  const hasAdditiveModifier = keyboard?.shiftKey === true;
  const hasSubtractiveModifier = keyboard?.altKey === true || keyboard?.ctrlKey === true || keyboard?.metaKey === true;
  if (hasAdditiveModifier && hasSubtractiveModifier) return "intersect";
  if (hasAdditiveModifier) return "additive";
  if (hasSubtractiveModifier) return "subtractive";
  return resolveSelectionCompositionKindFromTool(toolKind);
}

export function toSelectionToolKind(compositionKind: SelectionCompositionKind): ToolKind {
  if (compositionKind === "additive") return ToolKind.SELECTION_ADDITIVE;
  if (compositionKind === "subtractive") return ToolKind.SELECTION_SUBTRACTIVE;
  if (compositionKind === "intersect") return ToolKind.SELECTION_INTERSECT;
  return ToolKind.SELECTION_NORMAL;
}

export function applySelectionComposition<T>(previous: T[] | undefined, incoming: T[] | undefined, compositionKind: SelectionCompositionKind): T[] {
  const uniquePrevious = Array.from(new Set(previous ?? []));
  const uniqueIncoming = Array.from(new Set(incoming ?? []));
  if (compositionKind === "replace") return uniqueIncoming;
  if (compositionKind === "additive") {
    const previousSet = new Set(uniquePrevious);
    return [...uniquePrevious, ...uniqueIncoming.filter((value) => !previousSet.has(value))];
  }
  if (compositionKind === "subtractive") {
    const incomingSet = new Set(uniqueIncoming);
    return uniquePrevious.filter((value) => !incomingSet.has(value));
  }
  const incomingSet = new Set(uniqueIncoming);
  return uniquePrevious.filter((value) => incomingSet.has(value));
}

export type { UIWindowControl as WindowControl, UIWindowKindDefinition as WindowKindDefinition } from "@semio/ui";

/**
 * Panel layout positions: left, right, middle, or bottom.
 **/
export enum PanelPosition {
  LEFT = "left",
  RIGHT = "right",
  MIDDLE = "middle",
  BOTTOM = "bottom",
}

/**
 * Panel kinds: tools, toolbar, stats, details, params, or console.
 **/
export enum PanelKind {
  WORKBENCH = "workbench",
  TOOLS = "tools",
  TOOLBAR = "toolbar",
  STATS = "stats",
  DETAILS = "details",
  PARAMS = "params",
  CONSOLE = "console",
}

// #endregion ðŸ“©Enums

// #region ðŸ“ŠPorts

// #region ðŸ–²ï¸File Provider
// MUST define file storage provider interfaces for upload, download, and delete operations.

/**
 * Interface for file upload, download, delete, and URL retrieval operations.
 **/
export interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

/**
 * Configuration interface for in-memory file provider.
 **/
export interface MemoryFileProviderConfig {}

/**
 * Configuration interface for remote file provider with base URL and headers.
 **/
export interface RemoteFileProviderConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}

/**
 * Configuration interface combining memory and remote file providers.
 **/
export interface CompositeFileProviderConfig {
  memory?: boolean;
  remote?: RemoteFileProviderConfig;
}

// #region ðŸ”®Persistence
// Persistence types are defined inline above in ðŸ‚PersistenceProviders.
// #endregion ðŸ”®Persistence

/**
 * Interface for remote sync document and file provider factories.
 **/
export interface RemoteProviders {
  syncProvider: (syncDoc: SyncDoc, name: string) => void;
  fileProvider: FileProviderFactory;
}

/**
 * Describes a file operation with type, kit ID, file ID, path, and optional blob.
 **/
export interface FileOperation {
  type: "upload" | "download" | "delete";
  kitId: string;
  fileId: string;
  path: string;
  blob?: Blob;
}

// #endregion ðŸ–²ï¸File Provider

// #region ðŸª„App IDs
// MUST define identifier interfaces for design, kit, type, and quality app scopes.

/**
 * Identifier for a design app scope with kit and design IDs.
 **/
export interface DesignAppId {
  kit: Id;
  design: Id;
}

/**
 * Identifier for a kit app scope with a kit ID.
 **/
export interface KitAppId {
  kit: Id;
}

/**
 * Identifier for a type app scope with kit and type IDs.
 **/
export interface TypeAppId {
  kit: Id;
  type: Id;
}

/**
 * Identifier for a quality app scope with kit and quality IDs.
 **/
export interface QualityAppId {
  kit: Id;
  quality: Id;
}

// #endregion ðŸª„App IDs

// #region ðŸ¦‰Panel
// MUST define panel kind configurations, visibility, sizing, sections, and definition interfaces.

/**
 * Configuration for a panel kind including icon, position, group, and hotkey.
 **/
export interface PanelKindConfig {
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

/**
 * Registry mapping each PanelKind to its PanelKindConfig.
 **/
export const panelKindConfigs: Record<PanelKind, PanelKindConfig> = {
  [PanelKind.WORKBENCH]: {
    icon: WorkbenchIcon,
    position: PanelPosition.LEFT,
    group: "left",
    isGroupable: true,
  },
  [PanelKind.TOOLS]: {
    icon: ToolsIcon,
    position: PanelPosition.LEFT,
    group: "left",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLBAR]: {
    icon: ToolbarIcon,
    position: PanelPosition.BOTTOM,
  },
  [PanelKind.STATS]: {
    icon: StatsIcon,
    position: PanelPosition.MIDDLE,
    group: "hud",
    isGroupable: true,
    isTransparent: true,
    hotkey: "ctrl+k",
  },
  [PanelKind.DETAILS]: {
    icon: DetailsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.PARAMS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.CONSOLE]: {
    icon: CodeIcon,
    position: PanelPosition.BOTTOM,
    hotkey: "ctrl+k",
  },
};

/**
 * Side panel positions: left or right.
 **/
export enum SidePanelPosition {
  LEFT = "left",
  RIGHT = "right",
}

/**
 * A tab entry for a side panel with ID, icon, order, and content.
 **/
export interface SidePanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

/**
 * Visibility flags for left and right side panels.
 **/
export interface SidePanelVisibility {
  left: boolean;
  right: boolean;
}

/**
 * Optional visibility flags for all panel kinds.
 **/
export interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  params?: boolean;
  console?: boolean;
  chat?: boolean;
  settings?: boolean;
}

/**
 * Numeric sizes for all panel dimensions including widths and heights.
 **/
export interface PanelSizes {
  toolbarHeight: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  consoleHeight: number;
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  chatWidth: number;
  settingsWidth: number;
}

/**
 * A collapsible section within a panel with content, actions, and toolbar group.
 **/
export interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  toolbarGroup?: {
    id: string; // "selection", "filter", "create", "view", "actions"
    labelId?: string;
    order?: number;
    onActivate?: () => void;
  };
  actions?: Array<{
    id: string;
    icon: ReactNode;
    onClick: () => void;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
  toolbarPlaceholder?: boolean;
}

/**
 * Left and right arrays of side panel tabs.
 **/
export interface SidePanelTabs {
  left: SidePanelTab[];
  right: SidePanelTab[];
}

/**
 * Collections of panel sections and tabs organized by panel kind.
 **/
export interface PanelSections {
  workbench: PanelSection[];
  details: PanelSection[];
  tools: PanelSection[];
  hud: PanelSection[];
  stats: PanelSection[];
  console: PanelSection[];
  toolbar: PanelSection[];
  leftSidePanel: SidePanelTab[];
  rightSidePanel: SidePanelTab[];
}

/**
 * Definition of a panel with ID, kind, hotkey, and tooltip.
 **/
export interface PanelDefinition {
  id: string;
  kind: PanelKind;
  hotkey?: string;
  tooltip?: {
    labelKey?: string;
    manualPath?: string;
  };
}

/**
 * Extended panel definition with resolved icon, position, group, and transparency.
 **/
export interface EnrichedPanelDefinition extends PanelDefinition {
  key: string;
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

/**
 * Constructs a PanelDefinition from a kind, ID, hotkey, and tooltip.
 * MUST use the panelKindConfigs hotkey as fallback when no explicit hotkey is provided.
 **/
export function createPanelDefinition(kind: PanelKind, id: string, hotkey?: string, tooltip?: { labelKey?: string; manualPath?: string }): PanelDefinition {
  const config = panelKindConfigs[kind];
  return {
    id,
    kind,
    hotkey: hotkey ?? config.hotkey,
    tooltip,
  };
}

/**
 * Enriches a PanelDefinition with resolved config properties from panelKindConfigs.
 * MUST resolve all config properties from panelKindConfigs for the panel's kind.
 **/
export function enrichPanelDefinition(panel: PanelDefinition): EnrichedPanelDefinition {
  const config = panelKindConfigs[panel.kind];
  return {
    ...panel,
    key: panel.kind,
    icon: config.icon,
    position: config.position,
    group: config.group,
    isTransparent: config.isTransparent,
    isGroupable: config.isGroupable,
    hotkey: panel.hotkey ?? config.hotkey,
  };
}

/**
 * Configuration for a panel instance with ID, key, label, order, and content.
 **/
export interface PanelConfig {
  id: string;
  key: "workbench" | "details" | "tools" | "hud" | "stats" | "toolbar" | "console";
  label: string;
  order?: number;
  defaultOpen?: boolean;
  content: ReactNode | (() => ReactNode);
}

/**
 * Container for an array of panel configurations.
 **/
export interface AppPanels {
  panels: PanelConfig[];
}

// #endregion ðŸ¦‰Panel

// #region ðŸªµApp Registry
// MUST define route segment and app configuration interfaces for app registration.

/**
 * A URL route segment with path, optional param name, and id-bound context provider.
 **/
export interface RouteSegment {
  path: string;
  paramName?: string;
  contextProvider?: ComponentType<{ id: string; children: ReactNode }>;
}

/**
 * Full app configuration with ID, component, routes, panels, and order.
 **/
export interface AppConfig {
  id: string;
  component: ComponentType;
  routeSegments: RouteSegment[];
  additionalPaths?: string[];
  getPanels: (() => PanelDefinition[]) | ((getLabelFn: (key: string) => string) => PanelDefinition[]) | ((getLabelFn: (key: string) => string, getHotkeyFn: (key: string) => string) => PanelDefinition[]);
  matchesPath?: (pathParts: string[]) => boolean;
  order?: number;
}

/**
 * App registration entry extending AppConfig.
 **/
export interface AppRegistration extends AppConfig {}

// #endregion ðŸªµApp Registry

// #region ðŸ“¹Sketchpad State
// MUST define mutable and immutable sketchpad state interfaces with diff types.

/**
 * Mobile device state with navbar and footer expansion flags.
 **/
export interface MobileDevice {
  isNavbarExpanded: boolean;
  isFooterExpanded: boolean;
}

/**
 * Mutable fields of sketchpad state including navigation, theme, device, and settings.
 **/
export interface SketchpadChangableState {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  device: Device;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

/**
 * Full sketchpad state extending changeable state with ID and persistence flag.
 **/
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

/**
 * Partial diff of sketchpad state fields for incremental updates.
 **/
export interface SketchpadDiff {
  navigation?: string;
  navigationHistory?: string[];
  navigationHistoryIndex?: number;
  recentSearches?: string[];
  recentFocusItems?: Record<string, string[]>;
  theme?: Theme;
  language?: string;
  device?: Device;
  expertise?: Expertise;
  mode?: Mode;
  settings?: {
    apps?: Record<string, any>;
  };
  panelSizes?: Partial<PanelSizes>;
  isFullscreen?: boolean;
  isMobile?: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

/**
 * Initial kit state with kit data and local/remote flags.
 **/
export interface InitialStateKit {
  kit: Kit;
  /** @emoji ðŸ”Œ Owns WASM transport for this inline-imported kit; dispose when tab closes. */
  session?: Session;
  kind?: KitKind;
  source?: {
    kind: "folder" | "file" | "remote";
    path?: string;
    url?: string;
  };
}

/**
 * Extended initial state combining partial sketchpad state with initial kits.
 **/
export interface ExtendedInitialState extends Partial<SketchpadState> {
  kits?: InitialStateKit[];
}

/**
 * Desktop integration surface. When provided, sketchpad knows it is running in desktop mode and renders window controls.
 * Specs: Presence of the desktop prop is the ONLY signal that sketchpad is running as a desktop app; absence means browser mode.
 * Includes native file/folder kit callbacks for Electron IPC bridge.
 **/
export type Desktop = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
  kitFolder?: {
    selectFolder(): Promise<string | null>;
    readKit(folderPath: string): Promise<ArrayBuffer | null>;
    writeKit(folderPath: string, data: ArrayBuffer): Promise<void>;
    readFile(folderPath: string, filePath: string): Promise<ArrayBuffer | null>;
    writeFile(folderPath: string, filePath: string, data: ArrayBuffer): Promise<void>;
    deleteFile(folderPath: string, filePath: string): Promise<void>;
    listFiles(folderPath: string): Promise<string[]>;
    getRecentFolders(): Promise<string[]>;
    addRecentFolder(folderPath: string): Promise<void>;
    watchFolder(folderPath: string, onChanged: () => void): () => void;
  };
  kitFile?: {
    selectFile(): Promise<string | null>;
    readJson(filePath: string): Promise<string | null>;
    writeJson(filePath: string, json: string): Promise<void>;
  };
};

/**
 * Sketchpad instance record: id, optional remote providers, and desktop integration.
 **/
export type SketchpadInstance = { id: string; remote?: RemoteProviders; desktop?: Desktop };

// #endregion ðŸ“¹Sketchpad State

// #region ðŸ’§Commands
// Sketchpad command context/result; kit I/O is `@semio/react` (`KitCommandContext`, `KitCommandResult`, `applyKitHostGraphOperation`, `executeSemioKitCommand` â†’ `@semio/js` / rs).

/**
 * Context for sketchpad commands including sketchpad state and origin.
 **/
export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
  origin?: string;
}

/**
 * Result of a sketchpad command with optional diff and origin.
 **/
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
  origin?: string;
}

// #endregion ðŸ’§Commands

// #region ðŸŽˆStore
// MUST define store state, app step, edit, diff, and command result interfaces.

/**
 * Interface for objects that support change subscription and snapshot retrieval.
 **/
export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

/**
 * Wrapper for store status, data, and error.
 **/
export interface StoreState<TState> {
  status: StoreStatus;
  data?: TState;
  error?: Error;
}

/**
 * A single app step with optional selection diff.
 **/
export interface AppStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

/**
 * An undoable edit consisting of do and undo app steps.
 **/
export interface AppEdit<TSelectionDiff = any> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

/**
 * A diff containing selection, presence, hover, fullscreen, and panel visibility changes.
 **/
export interface AppDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenWindow?: any;
  panelVisibility?: Partial<PanelVisibility>;
}

/**
 * Result of an app command with optional diff and origin.
 **/
export interface AppCommandResult<TDiff = any> {
  diff?: TDiff;
  origin?: string;
}

/**
 * An app step; kit graph side-effects go through `semio/rs` undo history (`semio.kit.undo` / `semio.kit.redo`) â€” no host-side kit diffs.
 **/
export interface KitDiffAppStep<TSelectionDiff = any> extends AppStep<TSelectionDiff> {
  applyKitUndoOnUndo?: boolean;
  applyKitRedoOnRedo?: boolean;
}

/**
 * An undoable edit with kit diff-aware do and undo steps.
 **/
export interface KitDiffAppEdit<TSelectionDiff = any> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

/**
 * @emoji ðŸ§¾ Command result: optional typed {@link KitHostGraphOperation} runs through {@link applyKitHostGraphOperation} (`@semio/react` â†’ `@semio/js` â†’ `semio/rs`).
 **/
export interface KitDiffAppCommandResult<TDiff = any> extends AppCommandResult<TDiff> {
  kitGraph?: KitHostGraphOperation;
  kitCommandApplied?: boolean;
}

// #endregion ðŸŽˆStore

// #region ðŸŒComplete State
// MUST define the complete aggregated state interface for the entire sketchpad.

/**
 * Full aggregated state containing sketchpad, kits, and all app states.
 **/
export interface CompleteState {
  sketchpad: SketchpadState;
  kits: Array<{
    id: string;
    local: boolean;
    remote: boolean;
    kit: Kit;
  }>;
  kitApps: Record<string, any>;
  typeApps: Record<string, any>;
  qualityApps: Record<string, any>;
  designApps: Record<string, Record<string, any>>;
  home?: any;
  tutorials: any;
}

// #endregion ðŸŒComplete State

// #region ðŸŒŠWindow
// MUST define window configuration, control, layout parsing, and default layout creation.

/**
 * Configuration for a window with ID, title, icon, component, and default size.
 **/
export interface WindowConfig {
  id: string;
  title?: string;
  icon?: ReactNode;
  component: ComponentType<any>;
  componentProps?: any;
  defaultSize?: number;
}

// WindowControl and WindowKindDefinition are re-exported from elements.tsx above.

/**
 * App-level window configuration with window kinds and default layout.
 **/
export interface AppWindowConfig {
  windowKinds: UIWindowKindDefinition[];
  defaultLayout?: LayoutNode;
}

// parseWindowLayout, deduplicateWindowLayout, stringifyWindowLayout, createDefaultLayout
// are re-exported from elements.tsx above.

/**
 * Props for an app window component with kind, children, and className.
 **/
export interface AppWindowProps {
  kind: WindowKind;
  children: ReactNode;
  className?: string;
}

// createDefaultLayout is re-exported from elements.tsx above.

// #endregion ðŸŒŠWindow

// #region ðŸŽTool
// MUST define tool interfaces for selection, lasso, connector, and hand interactions.

/**
 * A tool with ID, icon, and render function returning scene, diagram, and table nodes.
 **/
export interface Tool<TState = any> {
  id: ToolKind | string;
  icon?: ReactNode;
  render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

/**
 * A tool mode with ID, icon, label, and tooltip.
 **/
export interface ToolMode {
  id: string;
  icon?: ReactNode;
  label?: string;
  tooltipId?: string;
}

/**
 * Definition of a tool with ID, default mode, and available modes.
 **/
export interface ToolDefinition {
  id: string;
  defaultMode: ToolKind | string;
  modes: ToolMode[];
}

/**
 * Context passed to a tool's render function containing the current state.
 **/
export interface ToolRenderContext<TState = any> {
  state: TState;
}

/**
 * Props for a tool group component with tools, active tool, and change handler.
 **/
export interface ToolGroupProps {
  tools: ToolDefinition[];
  activeTool: ToolKind | string;
  onToolChange: (tool: ToolKind | string) => void;
}

// #endregion ðŸŽTool

// #region âš¡Focus
// MUST define the focus item interface for search and navigation targets.

// FocusItem is re-exported from elements.tsx as UIFindItem.
export type { UIFindItem as FocusItem } from "@semio/ui";
// #endregion âš¡Focus

// #region ðŸŽ®Footer
// MUST define the footer item interface for status bar entries.

/**
 * A footer status bar item with ID, icon, text, content, and click handler.
 **/
export interface FooterItem {
  id: string;
  icon?: ReactNode;
  text?: string;
  content?: ReactNode;
  onClick?: () => void;
  order?: number;
  className?: string;
  disabled?: boolean;
}

// #endregion ðŸŽ®Footer

// #region ðŸ·ï¸Panel Props
// MUST define resizable panel props interface for panel width management.

/**
 * Props for a resizable panel with visibility, width, and width change handler.
 **/
export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

// #endregion ðŸ·ï¸Panel Props

// #endregion ðŸ“ŠPorts

// #region ðŸ¥ˆXState Integration

// #region â„ï¸XState Types
// MUST define XState machine context and event type interfaces for sketchpad, kit, and app machines.

/**
 * Base context for Y.js-synced machines with dirty flag and cache.
 **/
export interface StoreSyncContext {
  dirty: boolean;

  cache?: any;
}

/**
 * XState context for the sketchpad machine with navigation, theme, kits, and refs.
 **/
export interface SketchpadMachineContext extends StoreSyncContext {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  device: Device;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;

  openKitGuids: Id[];
  activeKitGuid: Id | undefined;

  homeRef?: AnyActorRef;

  docsRef?: AnyActorRef;
}

/**
 * Union of all events the sketchpad machine can receive.
 **/
export type SketchpadMachineEvent =
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
  | { type: "CREATE_KIT"; kit: Kit }
  | { type: "DELETE_KIT"; id: Id }
  | { type: "SYNC_UPDATE"; data: any }
  | { type: "SYNC_FIELD_UPDATE"; field: string; value: any };

/**
 * XState context for a kit machine with ID, kit data, types, designs, and files.
 **/
export interface KitMachineContext extends StoreSyncContext {
  id: Id;
  kit: Kit;

  types: Record<Id, any>;

  designs: Record<Id, any>;

  fileUrls: Map<string, string>;

  local: boolean;

  remote: boolean;
}

/**
 * Union of all events the kit machine can receive.
 **/
export type KitMachineEvent =
  | { type: "LOAD" }
  | { type: "CHANGE"; diff: KitDiff }
  | { type: "CREATE_TYPE"; typeData: any }
  | { type: "UPDATE_TYPE"; id: Id; diff: any }
  | { type: "DELETE_TYPE"; id: Id }
  | { type: "CREATE_DESIGN"; design: any }
  | { type: "UPDATE_DESIGN"; id: Id; diff: any }
  | { type: "DELETE_DESIGN"; id: Id }
  | { type: "SYNC_UPDATE"; data: any };

/**
 * XState context for an app machine with panels, selection, hover, and transaction state.
 **/
export interface AppMachineContext<TSelection = any> extends StoreSyncContext {
  panelVisibility: PanelVisibility;
  selection?: TSelection;
  hover?: any;
  presence?: any;
  others: any[];

  isTransactionActive: boolean;
  currentTransactionStack: any[];
  pastTransactionsStack: any[];
  redoStack: any[];
}

/**
 * Union of all events an app machine can receive.
 **/
export type AppMachineEvent<TSelectionDiff = any, TDiff = any> =
  | { type: "START_NEW_CHANGE" }
  | { type: "SAVE_CHANGE" }
  | { type: "DISCARD_CHANGE" }
  | { type: "UNDO" }
  | { type: "REDO" }
  | { type: "TOGGLE_PANEL"; panel: keyof PanelVisibility }
  | { type: "SELECT"; diff: TSelectionDiff }
  | { type: "DESELECT" }
  | { type: "HOVER"; data: any }
  | { type: "CLEAR_HOVER" }
  | { type: "CHANGE"; diff: TDiff }
  | { type: "SYNC_UPDATE"; data: any };

/**
 * Extended app machine context with a kit ID for kit-diff-aware apps.
 **/
export interface KitDiffAppMachineContext<TSelection = any> extends AppMachineContext<TSelection> {
  kitId: Id;
}

// #endregion â„ï¸XState Types

// #region ðŸ¬Sync-XState Bridge
// MUST bridge sync document observation to XState machine events.

/**
 * Creates an XState callback actor that observes a sync map and sends SYNC_UPDATE events.
 * MUST observe the sync map deeply and send SYNC_UPDATE events on every change.
 **/
export function createSyncActor(syncMap: SyncMap<any>) {
  return fromCallback<{ type: "SYNC_UPDATE"; data: any }>(({ sendBack }: { sendBack: (event: { type: "SYNC_UPDATE"; data: any }) => void }) => {
    const observer = () => {
      sendBack({ type: "SYNC_UPDATE", data: syncMap.toJSON() });
    };

    observer();

    syncMap.observeDeep(observer);

    return () => {
      syncMap.unobserveDeep(observer);
    };
  });
}

/**
 * Creates an XState callback actor that observes a single field in a sync map.
 * MUST observe a specific field in the sync map and send SYNC_FIELD_UPDATE events.
 **/
export function createFieldSyncActor(syncMap: SyncMap<any>, field: string) {
  return fromCallback<{ type: "SYNC_FIELD_UPDATE"; field: string; value: any }>(({ sendBack }: { sendBack: (event: { type: "SYNC_FIELD_UPDATE"; field: string; value: any }) => void }) => {
    const observer = (event: SyncMapEvent) => {
      if (event.keysChanged.has(field)) {
        sendBack({ type: "SYNC_FIELD_UPDATE", field, value: syncMap.get(field) });
      }
    };

    sendBack({ type: "SYNC_FIELD_UPDATE", field, value: syncMap.get(field) });

    syncMap.observe(observer);

    return () => {
      syncMap.unobserve(observer);
    };
  });
}

/**
 * Executes a function within a sync document transaction.
 * MUST delegate to the SyncDoc transact method with the given origin.
 **/
export function syncTransact(syncDoc: SyncDoc, fn: () => void, origin?: string): void {
  syncDoc.transact(fn, origin);
}

/**
 * Creates an XState assign action that marks dirty and caches SYNC_UPDATE event data.
 * MUST return an XState assign that sets dirty to true and caches event data.
 **/
export function createSyncUpdateAssign() {
  return assign({
    dirty: () => true,
    cache: ({ event }: { event: { type: "SYNC_UPDATE"; data: any } }) => (event as any).data,
  });
}

/**
 * Creates a memoized XState selector that rebuilds only when context is dirty.
 * MUST return cached snapshot when not dirty, rebuilding only when dirty.
 **/
export function createSyncSelector<TContext extends StoreSyncContext, TSnapshot>(buildSnapshot: (context: TContext) => TSnapshot): (context: TContext) => TSnapshot {
  return (context: TContext): TSnapshot => {
    if (!context.dirty && context.cache) {
      return context.cache as TSnapshot;
    }
    return buildSnapshot(context);
  };
}

// #endregion ðŸ¬Sync-XState Bridge

// #region â›‘ï¸Machine Factories
// MUST define machine input and transaction configuration interfaces for state machine creation.

/**
 * Input for creating an app machine with sync map and transact function.
 **/
export interface AppMachineInput {
  syncMap: SyncMap<any>;
  transact: Transact;
}

/**
 * Extended app machine input with a kit ID.
 **/
export interface KitDiffAppMachineInput extends AppMachineInput {
  kitId: Id;
}

/**
 * @emoji ðŸ§¾ Configuration for local batched kit UI edits (maps to VCS â€œchangeâ€ lifecycle naming).
 **/
export interface ChangeMachineConfig<TEdit = any> {
  applySelectionDiff: (selectionDiff: any) => void;

  inverseSelectionDiff: (selection: any, diff: any) => any;
}

// #endregion â›‘ï¸Machine Factories

// #endregion ðŸ¥ˆXState Integration

// #region ðŸ‘“SyncPath Helpers
// MUST provide path segment constructors, value retrieval, and observation functions for Y.js paths.

/**
 * Creates a SyncPathSegment for accessing a map key.
 * MUST return a mapKey segment with the given key.
 **/
export function syncPathMapKey(key: string): SyncPathSegment {
  return { kind: "mapKey", key };
}

/**
 * Creates a SyncPathSegment for accessing an array element by index.
 * MUST return an arrayIndex segment with the given index.
 **/
export function syncPathArrayIndex(index: number): SyncPathSegment {
  return { kind: "arrayIndex", index };
}

/**
 * Creates a SyncPathSegment for accessing an array item by its ID field.
 * MUST return an arrayItemById segment with the given ID and idKey.
 **/
export function syncPathArrayItemById(id: string, idKey: string = "id"): SyncPathSegment {
  return { kind: "arrayItemById", id, idKey };
}

/**
 * Traverses a sync map or array along a SyncPath and returns the value at the end.
 * MUST traverse each path segment, returning undefined when a segment cannot be resolved.
 **/
export function getValueAtPath(root: SyncMap<any> | SyncArray<any>, path: SyncPath): any {
  let current: any = root;
  for (const segment of path) {
    if (current === undefined || current === null) return undefined;
    if (segment.kind === "mapKey") {
      if (!isSyncMap(current)) return undefined;
      current = current.get(segment.key);
    } else if (segment.kind === "arrayIndex") {
      if (!isSyncArray(current)) return undefined;
      current = current.get(segment.index);
    } else if (segment.kind === "arrayItemById") {
      if (!isSyncArray(current)) return undefined;
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (isSyncMap(item)) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      current = item;
    }
  }
  return current;
}

/**
 * Sets up deep observers along a SyncPath and calls subscribe when the leaf value changes.
 * MUST set up nested observers along the path and notify when the leaf value changes.
 **/
export function createPathObserver(root: SyncMap<any>, path: SyncPath, subscribe: Subscribe): Disposable {
  if (path.length === 0) {
    const callback = () => subscribe(() => {});
    root.observeDeep(callback);
    return () => root.unobserveDeep(callback);
  }
  const disposables: Disposable[] = [];
  const serializeValue = (v: any) => JSON.stringify(isSyncMap(v) || isSyncArray(v) ? v.toJSON() : v);
  let lastJson = serializeValue(getValueAtPath(root, path));
  const notifyIfChanged = () => {
    const newJson = serializeValue(getValueAtPath(root, path));
    if (lastJson !== newJson) {
      lastJson = newJson;
      subscribe(() => {});
    }
  };
  const setupObservers = (current: any, remainingPath: SyncPath, depth: number) => {
    if (!current || remainingPath.length === 0) return;
    const segment = remainingPath[0];
    const rest = remainingPath.slice(1);
    if (segment.kind === "mapKey" && isSyncMap(current)) {
      const mapCallback = (event: SyncMapEvent) => {
        if (event.keysChanged.has(segment.key)) {
          disposables.slice(depth + 1).forEach((d) => d());
          disposables.length = depth + 1;
          const next = current.get(segment.key);
          if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
          notifyIfChanged();
        }
      };
      current.observe(mapCallback);
      disposables.push(() => current.unobserve(mapCallback));
      const next = current.get(segment.key);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
      else if (rest.length === 0 && isSyncMap(next)) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      } else if (rest.length === 0 && isSyncArray(next)) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      }
    } else if (segment.kind === "arrayIndex" && isSyncArray(current)) {
      const arrayCallback = () => notifyIfChanged();
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const next = current.get(segment.index);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
    } else if (segment.kind === "arrayItemById" && isSyncArray(current)) {
      const arrayCallback = () => {
        disposables.slice(depth + 1).forEach((d) => d());
        disposables.length = depth + 1;
        const arr = current.toArray();
        const item = arr.find((item: any) => {
          if (isSyncMap(item)) return item.get(segment.idKey) === segment.id;
          return item?.[segment.idKey] === segment.id;
        });
        if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
        notifyIfChanged();
      };
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (isSyncMap(item)) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
    }
  };
  setupObservers(root, path, 0);
  return () => disposables.forEach((d) => d());
}

// #endregion ðŸ‘“SyncPath Helpers

// #region ðŸŽ™ï¸Store Factory Registry
// MUST manage registration and retrieval of app-specific store factory functions.

/**
 * Factory function type for creating a design app store.
 **/
export type DesignAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a kit app store.
 **/
export type KitAppStoreFactory = (parent: any, syncMap: any, transact: (fn: () => void) => void, id: any, state?: any) => any;
/**
 * Factory function type for creating a type app store.
 **/
export type TypeAppStoreFactory = (parent: any, id: any, state?: any) => any;
/**
 * Factory function type for creating a quality app store.
 **/
export type QualityAppStoreFactory = (parent: any, id: any, state?: any) => any;

/**
 * designAppStoreFactory holds the data fields for a designAppStoreFactory record.
 **/
let designAppStoreFactory: DesignAppStoreFactory | undefined;
/**
 * kitAppStoreFactory holds the data fields for a kitAppStoreFactory record.
 **/
let kitAppStoreFactory: KitAppStoreFactory | undefined;
/**
 * typeAppStoreFactory holds the data fields for a typeAppStoreFactory record.
 **/
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
/**
 * qualityAppStoreFactory holds the data fields for a qualityAppStoreFactory record.
 **/
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;

/**
 * Registers the design app store factory.
 * MUST replace any previously registered design app store factory.
 **/
export function registerDesignAppStoreFactory(factory: DesignAppStoreFactory) {
  designAppStoreFactory = factory;
}

/**
 * Registers the kit app store factory.
 * MUST replace any previously registered kit app store factory.
 **/
export function registerKitAppStoreFactory(factory: KitAppStoreFactory) {
  kitAppStoreFactory = factory;
}

/**
 * Registers the type app store factory.
 * MUST replace any previously registered type app store factory.
 **/
export function registerTypeAppStoreFactory(factory: TypeAppStoreFactory) {
  typeAppStoreFactory = factory;
}

/**
 * Registers the quality app store factory.
 * MUST replace any previously registered quality app store factory.
 **/
export function registerQualityAppStoreFactory(factory: QualityAppStoreFactory) {
  qualityAppStoreFactory = factory;
}

/**
 * Retrieves the registered design app store factory or throws if not registered.
 * MUST throw if no design app store factory has been registered.
 **/
export function getDesignAppStoreFactory(): DesignAppStoreFactory {
  if (!designAppStoreFactory) throw new Error("Design app store factory not registered");
  return designAppStoreFactory;
}

/**
 * Retrieves the registered kit app store factory or throws if not registered.
 * MUST throw if no kit app store factory has been registered.
 **/
export function getKitAppStoreFactory(): KitAppStoreFactory {
  if (!kitAppStoreFactory) throw new Error("Kit app store factory not registered");
  return kitAppStoreFactory;
}

/**
 * Retrieves the registered type app store factory or throws if not registered.
 * MUST throw if no type app store factory has been registered.
 **/
export function getTypeAppStoreFactory(): TypeAppStoreFactory {
  if (!typeAppStoreFactory) throw new Error("Type app store factory not registered");
  return typeAppStoreFactory;
}

/**
 * Retrieves the registered quality app store factory or throws if not registered.
 * MUST throw if no quality app store factory has been registered.
 **/
export function getQualityAppStoreFactory(): QualityAppStoreFactory {
  if (!qualityAppStoreFactory) throw new Error("Quality app store factory not registered");
  return qualityAppStoreFactory;
}

// #endregion ðŸŽ™ï¸Store Factory Registry

// #region ðŸ“°App Plugin Registry
// MUST manage plugin registration, retrieval, and contribution composition for app extensions.

/**
 * Plugin contribution of event types, actions, guards, handlers, selectors, and default state.
 **/
export interface AppMachineContribution {
  eventTypes?: Record<string, any>;

  actions?: Record<string, (context: any, event: any) => any>;

  guards?: Record<string, (context: any, event: any) => boolean>;

  eventHandlers?: Record<string, { guard?: string; actions?: string | string[] }>;

  selectors?: Record<string, (context: any, ...args: any[]) => any>;

  createDefaultState?: () => any;
}

/**
 * An app plugin with ID, namespace, machine contribution, and lifecycle hooks.
 **/
export interface AppPlugin {
  id: string;

  namespace: string;

  machine: AppMachineContribution;

  registerStores?: () => void;

  onRegister?: () => void;
}

/**
 * appPlugins holds the data fields for a appPlugins record.
 **/
const appPlugins: Map<string, AppPlugin> = new Map();

/**
 * Registers an app plugin, invoking its store registration and onRegister hooks.
 * MUST store the plugin and invoke registerStores and onRegister hooks if present.
 **/
export function registerAppPlugin(plugin: AppPlugin): void {
  if (appPlugins.has(plugin.id)) {
    console.warn(`App plugin "${plugin.id}" already registered, replacing...`);
  }
  appPlugins.set(plugin.id, plugin);

  if (plugin.registerStores) {
    plugin.registerStores();
  }

  if (plugin.onRegister) {
    plugin.onRegister();
  }
}

/**
 * Returns all registered app plugins.
 * MUST return all registered plugins as an array.
 **/
export function getAppPlugins(): AppPlugin[] {
  return Array.from(appPlugins.values());
}

/**
 * Returns the registered app plugin with the given ID, or undefined.
 * MUST look up the plugin by ID in the registry.
 **/
export function getAppPlugin(id: string): AppPlugin | undefined {
  return appPlugins.get(id);
}

/**
 * Checks whether an app plugin with the given ID is registered.
 * MUST check the registry for the given plugin ID.
 **/
export function hasAppPlugin(id: string): boolean {
  return appPlugins.has(id);
}

/**
 * Merges actions, guards, event handlers, and selectors from all registered plugins.
 * MUST iterate all plugins and merge their contributions into single records.
 **/
export function composePluginContributions(): {
  actions: Record<string, (context: any, event: any) => any>;
  guards: Record<string, (context: any, event: any) => boolean>;
  eventHandlers: Record<string, { guard?: string; actions?: string | string[] }>;
  selectors: Record<string, (context: any, ...args: any[]) => any>;
} {
  const actions: Record<string, any> = {};
  const guards: Record<string, any> = {};
  const eventHandlers: Record<string, any> = {};
  const selectors: Record<string, any> = {};

  for (const plugin of appPlugins.values()) {
    const contribution = plugin.machine;

    if (contribution.actions) {
      for (const [name, fn] of Object.entries(contribution.actions)) {
        actions[name] = fn;
      }
    }

    if (contribution.guards) {
      for (const [name, fn] of Object.entries(contribution.guards)) {
        guards[name] = fn;
      }
    }

    if (contribution.eventHandlers) {
      for (const [eventType, handler] of Object.entries(contribution.eventHandlers)) {
        eventHandlers[eventType] = handler;
      }
    }

    if (contribution.selectors) {
      for (const [name, fn] of Object.entries(contribution.selectors)) {
        selectors[`${plugin.id}.${name}`] = fn;
      }
    }
  }

  return { actions, guards, eventHandlers, selectors };
}

/**
 * Collects default states from all registered plugins.
 * MUST call createDefaultState on each plugin that defines it.
 **/
export function getPluginDefaultStates(): Record<string, any> {
  for (const plugin of appPlugins.values()) {
    const createDefaultState = plugin.machine.createDefaultState;
    if (!createDefaultState) continue;
    defaults[plugin.id] = createDefaultState();
  }
  return defaults;
}

// #endregion ðŸ“°App Plugin Registry

// #region ðŸ“¸Dynamic Event Dispatch Registry
// MUST manage dynamic event handler and guard registration with namespace-based dispatch.

/**
 * Configuration for a dynamic event handler with optional guard and action.
 **/
export interface EventHandlerConfig<TContext = any, TEvent = any> {
  guard?: (context: TContext, event: TEvent) => boolean;

  action: (context: TContext, event: TEvent) => Partial<TContext>;
}

/**
 * eventHandlerRegistry holds the data fields for a eventHandlerRegistry record.
 **/
const eventHandlerRegistry: Map<string, EventHandlerConfig> = new Map();

/**
 * guardRegistry holds the data fields for a guardRegistry record.
 **/
const guardRegistry: Map<string, (context: any, event: any) => boolean> = new Map();

/**
 * Registers a dynamic event handler for a given event type.
 * MUST store the handler config in the registry keyed by event type.
 **/
export function registerEventHandler<TContext = any, TEvent = any>(eventType: string, config: EventHandlerConfig<TContext, TEvent>): void {
  eventHandlerRegistry.set(eventType, config as EventHandlerConfig);
}

/**
 * Removes a registered event handler for a given event type.
 * MUST remove the handler for the given event type.
 **/
export function unregisterEventHandler(eventType: string): void {
  eventHandlerRegistry.delete(eventType);
}

/**
 * Checks whether an event handler is registered for a given event type.
 * MUST check the registry for the given event type.
 **/
export function hasEventHandler(eventType: string): boolean {
  return eventHandlerRegistry.has(eventType);
}

/**
 * Retrieves the event handler configuration for a given event type.
 * MUST return the handler config or undefined.
 **/
export function getEventHandler(eventType: string): EventHandlerConfig | undefined {
  return eventHandlerRegistry.get(eventType);
}

/**
 * Executes the registered event handler for the given event, applying guard and action.
 * MUST run the guard before the action, returning empty context when guard fails.
 **/
export function executeEventHandler<TContext = any, TEvent extends { type: string } = any>(context: TContext, event: TEvent): Partial<TContext> {
  const handler = eventHandlerRegistry.get(event.type);
  if (!handler) return {};

  if (handler.guard && !handler.guard(context, event)) {
    return {};
  }

  return handler.action(context, event);
}

/**
 * Registers a named guard function.
 * MUST store the guard function keyed by name.
 **/
export function registerGuard(name: string, guard: (context: any, event: any) => boolean): void {
  guardRegistry.set(name, guard);
}

/**
 * Removes a registered guard function by name.
 * MUST remove the guard function by name.
 **/
export function unregisterGuard(name: string): void {
  guardRegistry.delete(name);
}

/**
 * Retrieves a registered guard function by name.
 * MUST return the guard function or undefined.
 **/
export function getGuard(name: string): ((context: any, event: any) => boolean) | undefined {
  return guardRegistry.get(name);
}

/**
 * Checks whether a guard with the given name is registered.
 * MUST check the guard registry for the given name.
 **/
export function hasGuard(name: string): boolean {
  return guardRegistry.has(name);
}

/**
 * Executes a registered guard and returns its boolean result.
 * MUST return false when the guard is not registered.
 **/
export function executeGuard(name: string, context: any, event: any): boolean {
  const guard = guardRegistry.get(name);
  if (!guard) return false;
  return guard(context, event);
}

/**
 * Returns all registered event types matching a given namespace prefix.
 * MUST filter event types by the namespace prefix.
 **/
export function getEventTypesForNamespace(namespace: string): string[] {
  const prefix = `${namespace}.`;
  return Array.from(eventHandlerRegistry.keys()).filter((key) => key.startsWith(prefix));
}

/**
 * Returns all unique namespaces from registered event types.
 * MUST extract unique namespace prefixes from all registered event types.
 **/
export function getRegisteredNamespaces(): string[] {
  const namespaces = new Set<string>();
  for (const eventType of eventHandlerRegistry.keys()) {
    const dotIndex = eventType.indexOf(".");
    if (dotIndex > 0) {
      namespaces.add(eventType.substring(0, dotIndex));
    }
  }
  return Array.from(namespaces);
}

/**
 * Returns all registered event type strings.
 * MUST return all event type strings from the registry.
 **/
export function getRegisteredEventTypes(): string[] {
  return Array.from(eventHandlerRegistry.keys());
}

// #endregion ðŸ“¸Dynamic Event Dispatch Registry

// #region ðŸ†App Event Handler Factories
// MUST provide factory functions for creating standard app event handlers for panels, hover, selection, and windows.

/**
 * Configuration for an app event handler with namespace, app key, and default state factory.
 **/
export interface AppEventHandlerConfig<TAppKey extends string, TAppState> {
  namespace: string;
  appKey: TAppKey;
  createDefaultState: () => TAppState;
}

/**
 * Registers a toggle panel event handler for the given app config.
 * MUST register a handler that toggles the specified panel in panelVisibility.
 **/
export function createTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.TOGGLE_PANEL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...app,
          panelVisibility: getNextPanelVisibilityFromToggle(app.panelVisibility, event.panel),
        },
      };
    },
  });
}

/**
 * Registers a set panel visibility event handler for the given app config.
 * MUST register a handler that replaces the entire panelVisibility.
 **/
export function createSetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_PANEL_VISIBILITY`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, panelVisibility: event.panelVisibility },
      };
    },
  });
}

/**
 * Registers a set hover event handler with a mapper for the given app config.
 * MUST register a handler that sets hover using the provided mapper.
 **/
export function createSetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const eventType = `${config.namespace}.SET_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, hover: hoverMapper(event) },
      };
    },
  });
}

/**
 * Registers a clear hover event handler with a guard for the given app config.
 * MUST register a handler with a guard that only clears non-empty hover state.
 **/
export function createClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_HOVER`;
  registerEventHandler(eventType, {
    guard: (context: any) => {
      const app = context[config.appKey];
      const hover = app?.hover;
      return hover !== undefined && Object.keys(hover).some((k) => hover[k] !== undefined && (Array.isArray(hover[k]) ? hover[k].length > 0 : true));
    },
    action: (context: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, hover: undefined },
      };
    },
  });
}

/**
 * Registers a set window layout event handler for the given app config.
 * MUST register a handler that sets the windowLayout from the event.
 **/
export function createSetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_WINDOW_LAYOUT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, windowLayout: event.windowLayout },
      };
    },
  });
}

/**
 * Registers a clear selection event handler for the given app config.
 * MUST register a handler that sets selection to undefined.
 **/
export function createClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: AppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any) => {
      const app = context[config.appKey] || config.createDefaultState();
      return {
        [config.appKey]: { ...app, selection: undefined },
      };
    },
  });
}

/**
 * Extended app event handler config with a getKey function for keyed state.
 **/
export interface KeyedAppEventHandlerConfig<TAppKey extends string, TAppState> extends AppEventHandlerConfig<TAppKey, TAppState> {
  getKey: (event: any) => string;
}

const EXCLUSIVE_SIDE_PANEL_KEYS: (keyof PanelVisibility)[] = ["rightSidePanel", "chat", "settings"];

/**
 * Returns the next panel visibility state for a toggle event.
 * MUST keep the dedicated right side panel visibility toggle isolated from other panel flags.
 **/
export function getNextPanelVisibilityFromToggle(panelVisibility: PanelVisibility, panel: keyof PanelVisibility): PanelVisibility {
  const nextValue = !panelVisibility[panel];
  if (!EXCLUSIVE_SIDE_PANEL_KEYS.includes(panel) || !nextValue) {
    return {
      ...panelVisibility,
      [panel]: nextValue,
    };
  }
  const nextPanelVisibility: PanelVisibility = {
    ...panelVisibility,
    [panel]: true,
  };
  EXCLUSIVE_SIDE_PANEL_KEYS.forEach((key) => {
    if (key !== panel) {
      nextPanelVisibility[key] = false;
    }
  });
  return nextPanelVisibility;
}

/**
 * Registers a keyed toggle panel event handler for multi-instance app state.
 * MUST register a keyed handler that toggles the panel for the resolved key.
 **/
export function createKeyedTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.TOGGLE_PANEL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: {
            ...app,
            panelVisibility: getNextPanelVisibilityFromToggle(app.panelVisibility, event.panel),
          },
        },
      };
    },
  });
}

/**
 * Registers a keyed set panel visibility event handler for multi-instance app state.
 * MUST register a keyed handler that replaces panelVisibility for the resolved key.
 **/
export function createKeyedSetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_PANEL_VISIBILITY`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, panelVisibility: event.panelVisibility },
        },
      };
    },
  });
}

/**
 * Registers a keyed set hover event handler for multi-instance app state.
 * MUST register a keyed handler that sets hover for the resolved key.
 **/
export function createKeyedSetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const eventType = `${config.namespace}.SET_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, hover: hoverMapper(event) },
        },
      };
    },
  });
}

/**
 * Registers a keyed clear hover event handler for multi-instance app state.
 * MUST register a keyed handler that clears hover for the resolved key.
 **/
export function createKeyedClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_HOVER`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, hover: undefined },
        },
      };
    },
  });
}

/**
 * Registers a keyed set selection event handler for multi-instance app state.
 * MUST register a keyed handler that sets selection for the resolved key.
 **/
export function createKeyedSetSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, selection: event.selection },
        },
      };
    },
  });
}

/**
 * Registers a keyed clear selection event handler for multi-instance app state.
 * MUST register a keyed handler that clears selection for the resolved key.
 **/
export function createKeyedClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.CLEAR_SELECTION`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, selection: undefined },
        },
      };
    },
  });
}

/**
 * Registers a keyed set window layout event handler for multi-instance app state.
 * MUST register a keyed handler that sets windowLayout for the resolved key.
 **/
export function createKeyedSetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_WINDOW_LAYOUT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, windowLayout: event.windowLayout },
        },
      };
    },
  });
}

/**
 * Registers a keyed set camera event handler for multi-instance app state.
 * MUST register a keyed handler that sets camera for the resolved key.
 **/
export function createKeyedSetCameraHandler<TAppKey extends string, TAppState extends { camera?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_CAMERA`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, camera: event.camera },
        },
      };
    },
  });
}

/**
 * Registers a keyed set active tool event handler for multi-instance app state.
 * MUST register a keyed handler that sets activeTool for the resolved key.
 **/
export function createKeyedSetActiveToolHandler<TAppKey extends string, TAppState extends { activeTool?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_ACTIVE_TOOL`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, activeTool: event.tool },
        },
      };
    },
  });
}

/**
 * Registers a keyed set fullscreen window event handler for multi-instance app state.
 * MUST register a keyed handler that sets fullscreenWindow for the resolved key.
 **/
export function createKeyedSetFullscreenWindowHandler<TAppKey extends string, TAppState extends { fullscreenWindow?: any }>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SET_FULLSCREEN_WINDOW`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, fullscreenWindow: event.window },
        },
      };
    },
  });
}

/**
 * Registers a keyed init event handler that sets initial keyed app state.
 * MUST register a keyed handler that initializes state for the resolved key.
 **/
export function createKeyedInitHandler<TAppKey extends string, TAppState>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.INIT`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      return {
        [config.appKey]: {
          ...apps,
          [key]: event.state,
        },
      };
    },
  });
}

/**
 * Registers a keyed sync event handler that merges state for keyed app state.
 * MUST register a keyed handler that merges state for the resolved key.
 **/
export function createKeyedSyncHandler<TAppKey extends string, TAppState>(config: KeyedAppEventHandlerConfig<TAppKey, TAppState>): void {
  const eventType = `${config.namespace}.SYNC`;
  registerEventHandler(eventType, {
    action: (context: any, event: any) => {
      const key = config.getKey(event);
      const apps = context[config.appKey] || {};
      const app = apps[key] || config.createDefaultState();
      return {
        [config.appKey]: {
          ...apps,
          [key]: { ...app, ...event.state },
        },
      };
    },
  });
}

/**
 * Registers all standard event handlers for a non-keyed app.
 * MUST register toggle panel, set panel visibility, hover, clear hover, window layout, and clear selection handlers.
 **/
export function registerStandardAppEventHandlers<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility; hover?: any; selection?: any; windowLayout?: any }>(
  config: AppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
): void {
  createTogglePanelHandler(config);
  createSetPanelVisibilityHandler(config);
  createSetHoverHandler(config, hoverMapper);
  createClearHoverHandler(config);
  createSetWindowLayoutHandler(config);
  createClearSelectionHandler(config);
}

/**
 * Registers all standard event handlers for a keyed multi-instance app.
 * MUST register init, sync, and all standard keyed handlers including camera, tool, and fullscreen.
 **/
export function registerKeyedAppEventHandlers<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility; hover?: any; selection?: any; windowLayout?: any; camera?: any; activeTool?: any; fullscreenWindow?: any }>(
  config: KeyedAppEventHandlerConfig<TAppKey, TAppState>,
  hoverMapper: (event: any) => any = (e) => e.hover,
): void {
  createKeyedInitHandler(config);
  createKeyedSyncHandler(config);
  createKeyedTogglePanelHandler(config);
  createKeyedSetPanelVisibilityHandler(config);
  createKeyedSetHoverHandler(config, hoverMapper);
  createKeyedClearHoverHandler(config);
  createKeyedSetSelectionHandler(config);
  createKeyedClearSelectionHandler(config);
  createKeyedSetWindowLayoutHandler(config);
  createKeyedSetCameraHandler(config);
  createKeyedSetActiveToolHandler(config);
  createKeyedSetFullscreenWindowHandler(config);
}

/**
 * Configuration for single-key event handlers with namespace, app key, key field, and default state.
 **/
export interface SingleKeyAppEventHandlerConfig<TAppKey extends string, TAppState> {
  namespace: string;
  appKey: TAppKey;
  keyField: string;
  createDefaultState: () => TAppState;
}

/**
 * Registers a single-key init event handler.
 * MUST register a handler that initializes state for the event's key field value.
 **/
export function createSingleKeyInitHandler<TAppKey extends string, TAppState>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.INIT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      return { [appKey]: { ...context[appKey], [key]: event.state } };
    },
  });
}

/**
 * Registers a single-key sync event handler.
 * MUST register a handler that merges state for the event's key field value.
 **/
export function createSingleKeySyncHandler<TAppKey extends string, TAppState>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SYNC`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, ...event.state } } };
    },
  });
}

/**
 * Registers a single-key toggle panel event handler.
 * MUST register a handler that toggles the panel for the event's key field value.
 **/
export function createSingleKeyTogglePanelHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.TOGGLE_PANEL`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, panelVisibility: getNextPanelVisibilityFromToggle(app.panelVisibility, event.panel) } } };
    },
  });
}

/**
 * Registers a single-key set panel visibility event handler.
 * MUST register a handler that replaces panelVisibility for the event's key field value.
 **/
export function createSingleKeySetPanelVisibilityHandler<TAppKey extends string, TAppState extends { panelVisibility: PanelVisibility }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_PANEL_VISIBILITY`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, panelVisibility: event.panelVisibility } } };
    },
  });
}

/**
 * Registers a single-key set hover event handler with a mapper.
 * MUST register a handler that sets hover for the event's key field value.
 **/
export function createSingleKeySetHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_HOVER`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, hover: hoverMapper(event) } } };
    },
  });
}

/**
 * Registers a single-key clear hover event handler.
 * MUST register a handler that clears hover for the event's key field value.
 **/
export function createSingleKeyClearHoverHandler<TAppKey extends string, TAppState extends { hover?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.CLEAR_HOVER`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, hover: undefined } } };
    },
  });
}

/**
 * Registers a single-key set selection event handler.
 * MUST register a handler that sets selection for the event's key field value.
 **/
export function createSingleKeySetSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_SELECTION`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, selection: event.selection } } };
    },
  });
}

/**
 * Registers a single-key clear selection event handler.
 * MUST register a handler that clears selection for the event's key field value.
 **/
export function createSingleKeyClearSelectionHandler<TAppKey extends string, TAppState extends { selection?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.CLEAR_SELECTION`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, selection: undefined } } };
    },
  });
}

/**
 * Registers a single-key set window layout event handler.
 * MUST register a handler that sets windowLayout for the event's key field value.
 **/
export function createSingleKeySetWindowLayoutHandler<TAppKey extends string, TAppState extends { windowLayout?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_WINDOW_LAYOUT`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, windowLayout: event.windowLayout } } };
    },
  });
}

/**
 * Registers a single-key set fullscreen window event handler.
 * MUST register a handler that sets fullscreenWindow for the event's key field value.
 **/
export function createSingleKeySetFullscreenWindowHandler<TAppKey extends string, TAppState extends { fullscreenWindow?: any }>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>): void {
  const { namespace, appKey, keyField, createDefaultState } = config;
  registerEventHandler(`${namespace}.SET_FULLSCREEN`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      return { [appKey]: { ...context[appKey], [key]: { ...app, fullscreenWindow: event.window } } };
    },
  });
}

/**
 * Registers all standard event handlers for a single-key app.
 * MUST register init, sync, and all standard single-key handlers.
 **/
export function registerSingleKeyAppEventHandlers<TAppKey extends string, TAppState extends object>(config: SingleKeyAppEventHandlerConfig<TAppKey, TAppState>, hoverMapper: (event: any) => any = (e) => e.hover) {
  createSingleKeyInitHandler(config);
  createSingleKeySyncHandler(config);
  createSingleKeyTogglePanelHandler(config);
  createSingleKeySetPanelVisibilityHandler(config);
  createSingleKeySetHoverHandler(config, hoverMapper);
  createSingleKeyClearHoverHandler(config);
  createSingleKeySetSelectionHandler(config);
  createSingleKeyClearSelectionHandler(config);
  createSingleKeySetWindowLayoutHandler(config);
  createSingleKeySetFullscreenWindowHandler(config);
}

// #endregion ðŸ†App Event Handler Factories

// #region ðŸŒªï¸Change Handler Factory
// MUST register undo/redo batch handlers keyed by app scope (local UI batching; distinct from persisted Yjs sync keys).

/**
 * Configuration for keyed change handlers with namespace, app key, key fields, and default state.
 **/
export interface KeyedChangeHandlerConfig {
  namespace: string;
  appKey: string;
  keyFields: [string, string];
  createDefaultState: () => { change: AppChangeState };
}

/**
 * @emoji ðŸ§¾ Local UI batch state mirrored into XState (field names align with legacy sync doc keys).
 **/
export interface AppChangeState<TEdit = any> {
  isTransactionActive: boolean;
  currentTransactionStack: TEdit[];
  pastTransactionStack: TEdit[];
  redoStack: TEdit[];
}

/**
 * Registers keyed change handlers (start/save/discard/undo/redo/run operation) for app state slices.
 **/
export function createKeyedChangeHandlers(config: KeyedChangeHandlerConfig): void {
  const { namespace, appKey, keyFields, createDefaultState } = config;
  const [keyField1, keyField2] = keyFields;

  registerEventHandler(`${namespace}.CHANGE.START_NEW_CHANGE`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key] || createDefaultState();
      const tx = app.change;
      if (tx.isTransactionActive) {
        const pastStack = [...tx.pastTransactionStack];
        if (tx.currentTransactionStack.length > 0) {
          const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
          pastStack.push(merged);
        }
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, isTransactionActive: true, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.SAVE_CHANGE`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      const tx = app.change;
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.DISCARD_CHANGE`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...app.change, isTransactionActive: false, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.UNDO`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app) return {};
      const tx = app.change;
      if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
        const currentStack = [...tx.currentTransactionStack];
        currentStack.pop();
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, currentTransactionStack: currentStack } } } };
      } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
        const pastStack = [...tx.pastTransactionStack];
        const edit = pastStack.pop()!;
        const redoStack = [...tx.redoStack, edit];
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
      }
      return {};
    },
  });

  registerEventHandler(`${namespace}.CHANGE.REDO`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || app.change.isTransactionActive || app.change.redoStack.length === 0) return {};
      const tx = app.change;
      const redoStack = [...tx.redoStack];
      const edit = redoStack.pop()!;
      const pastStack = [...tx.pastTransactionStack, edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.RUN_OPERATION`, {
    action: (context: any, event: any) => {
      const key = `${event[keyField1]}:${event[keyField2]}`;
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      const currentStack = [...app.change.currentTransactionStack, event.edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...app.change, currentTransactionStack: currentStack, redoStack: [] } } } };
    },
  });
}

/**
 * Configuration for single-key change handlers with namespace, app key, key field, and default state.
 **/
export interface SingleKeyChangeHandlerConfig {
  namespace: string;
  appKey: string;
  keyField: string;
  createDefaultState: () => { change: AppChangeState };
}

/**
 * Registers single-key change handlers for kit app state.
 **/
export function createSingleKeyChangeHandlers(config: SingleKeyChangeHandlerConfig): void {
  const { namespace, appKey, keyField, createDefaultState } = config;

  registerEventHandler(`${namespace}.CHANGE.START_NEW_CHANGE`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key] || createDefaultState();
      const tx = app.change;
      if (tx.isTransactionActive) {
        const pastStack = [...tx.pastTransactionStack];
        if (tx.currentTransactionStack.length > 0) {
          const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
          pastStack.push(merged);
        }
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, isTransactionActive: true, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.SAVE_CHANGE`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      const tx = app.change;
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.DISCARD_CHANGE`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...app.change, isTransactionActive: false, currentTransactionStack: [] } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.UNDO`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app) return {};
      const tx = app.change;
      if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
        const currentStack = [...tx.currentTransactionStack];
        currentStack.pop();
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, currentTransactionStack: currentStack } } } };
      } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
        const pastStack = [...tx.pastTransactionStack];
        const edit = pastStack.pop()!;
        const redoStack = [...tx.redoStack, edit];
        return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
      }
      return {};
    },
  });

  registerEventHandler(`${namespace}.CHANGE.REDO`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || app.change.isTransactionActive || app.change.redoStack.length === 0) return {};
      const tx = app.change;
      const redoStack = [...tx.redoStack];
      const edit = redoStack.pop()!;
      const pastStack = [...tx.pastTransactionStack, edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    },
  });

  registerEventHandler(`${namespace}.CHANGE.RUN_OPERATION`, {
    action: (context: any, event: any) => {
      const key = event[keyField];
      const app = context[appKey][key];
      if (!app || !app.change.isTransactionActive) return {};
      const currentStack = [...app.change.currentTransactionStack, event.edit];
      return { [appKey]: { ...context[appKey], [key]: { ...app, change: { ...app.change, currentTransactionStack: currentStack, redoStack: [] } } } };
    },
  });
}

// #endregion ðŸŒªï¸Change Handler Factory

// #region ðŸ§¿Selector Factory Pattern
// MUST provide factory functions for creating property selectors with app key scoping.

/**
 * Creates a factory for selectors that read a property from a non-keyed app state.
 * MUST return a factory that creates selectors reading from the given app key.
 **/
export function createAppPropertySelectorFactory<TApps extends Record<string, any>>(appKey: string) {
  return function createPropertySelector<TProperty>(propertyKey: keyof TApps[string], fallback: TProperty) {
    return (snapshot: { context: Record<string, TApps> }) => {
      const app = snapshot.context[appKey];
      return ((app as any)?.[propertyKey] ?? fallback) as TProperty;
    };
  };
}

/**
 * Creates a factory for selectors that read a property from a keyed app state.
 * MUST return a factory that creates keyed selectors reading from the given app key.
 **/
export function createKeyedAppPropertySelectorFactory<TAppState>(appKey: string) {
  return function createPropertySelector<TProperty>(propertyKey: keyof TAppState, fallback: TProperty) {
    return (key: string) => (snapshot: { context: Record<string, Record<string, TAppState>> }) => {
      const apps = snapshot.context[appKey] || {};
      const app = apps[key];
      return (app?.[propertyKey] ?? fallback) as TProperty;
    };
  };
}

/**
 * Joins scope strings into a colon-separated app key.
 * MUST join all scope strings with colon separators.
 **/
export function getAppKey(...scopes: string[]): string {
  return scopes.join(":");
}
/**
 * Retrieves existing app state or creates it from a default factory.
 * MUST return existing state or call the default factory to create it.
 **/
export function getOrCreateAppState<TState>(context: Record<string, Record<string, TState>>, appKey: string, key: string, defaultFactory: () => TState): TState {
  const apps = context[appKey] || {};
  return apps[key] || defaultFactory();
}

// #endregion ðŸ§¿Selector Factory Pattern

// #region â­App Hooks Registry
// MUST manage registration and retrieval of design and kit app hook implementations.

/**
 * Interface for design app hook functions including diff, hover, and selection.
 **/
export interface DesignAppHooks {
  useDesignAppDiff: () => any;
  useDesignAppHover: () => any;
  useDesignAppIsPieceHovered: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsPieceTransitiveHovered: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsConnectionHovered: (id?: DesignAppId, connectionId?: string) => boolean;
  useDesignAppSelection: () => any;
  useDesignAppIsPieceSelected: (id?: DesignAppId, pieceId?: string) => boolean;
  useDesignAppIsConnectionSelected: (id?: DesignAppId, connectionId?: string) => boolean;
  useDesignAppStore: <T>(selector?: (store: any) => T, id?: DesignAppId) => T | null;
}

/**
 * Interface for kit app hook functions including commands.
 **/
export interface KitAppHooks {
  useKitAppCommands: (id?: { kit: string }) => any;
}

/**
 * defaultDesignAppHooks holds the data fields for a defaultDesignAppHooks record.
 **/
const defaultDesignAppHooks: DesignAppHooks = {
  useDesignAppDiff: () => ({}),
  useDesignAppHover: () => undefined,
  useDesignAppIsPieceHovered: () => false,
  useDesignAppIsPieceTransitiveHovered: () => false,
  useDesignAppIsConnectionHovered: () => false,
  useDesignAppSelection: () => ({}),
  useDesignAppIsPieceSelected: () => false,
  useDesignAppIsConnectionSelected: () => false,
  useDesignAppStore: () => null,
};

/**
 * defaultKitAppHooks holds the data fields for a defaultKitAppHooks record.
 **/
const defaultKitAppHooks: KitAppHooks = {
  useKitAppCommands: () => ({ togglePanel: () => {}, execute: () => Promise.resolve({}) }),
};

/**
 * registeredDesignAppHooks holds the data fields for a registeredDesignAppHooks record.
 **/
let registeredDesignAppHooks: DesignAppHooks | null = null;
/**
 * registeredKitAppHooks holds the data fields for a registeredKitAppHooks record.
 **/
let registeredKitAppHooks: KitAppHooks | null = null;

/**
 * Registers design app hook implementations.
 * MUST store the provided hooks, replacing any previously registered.
 **/
export function registerDesignAppHooks(hooks: DesignAppHooks): void {
  registeredDesignAppHooks = hooks;
}

/**
 * Registers kit app hook implementations.
 * MUST store the provided hooks, replacing any previously registered.
 **/
export function registerKitAppHooks(hooks: KitAppHooks): void {
  registeredKitAppHooks = hooks;
}

/** getDesignAppHooks holds the data fields for a getDesignAppHooks record.
 * MUST fall back to default no-op hooks when none are registered.
 **/
export function getDesignAppHooks(): DesignAppHooks {
  return registeredDesignAppHooks ?? defaultDesignAppHooks;
}

/**
 * Returns registered kit app hooks or defaults.
 * MUST fall back to default no-op hooks when none are registered.
 **/
export function getKitAppHooks(): KitAppHooks {
  return registeredKitAppHooks ?? defaultKitAppHooks;
}

// #endregion â­App Hooks Registry

// #region ðŸŽ¸App Registry Exports
// MUST provide docs registry port interface and registration for documentation section access.

/**
 * Port interface for retrieving documentation section trees and pages.
 **/
export interface DocsRegistryPort {
  getSectionTree: (section: string) => any[];
  getAllPages: () => any[];
  getPage?: (path: string) => any;
}

/**
 * registeredDocsRegistry holds the data fields for a registeredDocsRegistry record.
 **/
let registeredDocsRegistry: DocsRegistryPort | null = null;

/**
 * Registers a docs registry implementation.
 * MUST store the given docs registry, replacing any previous one.
 **/
export function registerDocsRegistry(registry: DocsRegistryPort): void {
  registeredDocsRegistry = registry;
}

/**
 * Returns the registered docs registry or null.
 * MUST return the registered docs registry or null when none is registered.
 **/
export function getDocsRegistry(): DocsRegistryPort | null {
  return registeredDocsRegistry;
}

// #endregion ðŸŽ¸App Registry Exports

// #endregion â›©ï¸Shared

// #region ðŸ—ºï¸PortColor

/**
 * Compatibility state of a port relative to the selected port.
 *
 * MUST be one of none, selected, compatible, or incompatible.
 **/
export type PortCompatibilityState = "none" | "selected" | "compatible" | "incompatible";

/**
 * HSL color tones for rendering a port in the UI.
 *
 * MUST contain base, surface, surfaceStrong, border and text values.
 **/
export type PortTone = {
  base: string;
  surface: string;
  surfaceStrong: string;
  border: string;
  text: string;
};

/**
 * Builds a semio-themed tone from a base color token.
 *
 * MUST derive surface and border colors from the provided base token.
 **/
const createSemioPortTone = (base: string): PortTone => ({
  base,
  surface: `color-mix(in oklab, ${base} 24%, transparent)`,
  surfaceStrong: `color-mix(in oklab, ${base} 38%, transparent)`,
  border: `color-mix(in oklab, ${base} 70%, var(--color-dark) 30%)`,
  text: "var(--background)",
});

/**
 * Sentinel ID for ports without an assigned identity.
 *
 * MUST be used as the fallback key for tone generation.
 **/
/**
 * DEFAULT_PORT_ID holds the data fields for a DEFAULT_PORT_ID record.
 **/
/**
 * DEFAULT_PORT_ID holds the data fields for a DEFAULT_PORT_ID record.
 **/
const DEFAULT_PORT_ID = "__default__";

/**
 * Semio palette used to visualize compatible connector groups across a design.
 *
 * MUST stay within the semio brand color family and avoid success/danger colors reserved for selection feedback.
 **/
const SEMIO_PORT_TONES: readonly PortTone[] = [
  createSemioPortTone("var(--color-primary)"),
  createSemioPortTone("var(--color-secondary)"),
  createSemioPortTone("var(--color-tertiary)"),
  createSemioPortTone("color-mix(in oklab, var(--color-primary) 58%, var(--color-secondary) 42%)"),
  createSemioPortTone("color-mix(in oklab, var(--color-secondary) 56%, var(--color-tertiary) 44%)"),
  createSemioPortTone("color-mix(in oklab, var(--color-primary) 48%, var(--color-tertiary) 52%)"),
];

/**
 * Trims and normalizes a ID string, returning undefined for empty values.
 *
 * MUST return undefined for null, undefined, or whitespace-only input.
 **/
const normalizeId = (value: string | undefined | null): string | undefined => {
  if (!value) return undefined;
  const normalized = value.trim();
  return normalized.length > 0 ? normalized : undefined;
};

/**
 * Extracts a ID from a string or object with a id property.
 *
 * MUST handle both direct string IDs and port reference objects.
 **/
const normalizePortRef = (value: unknown): string | undefined => {
  if (typeof value === "string") return normalizeId(value);
  if (value && typeof value === "object" && "id" in (value as Record<string, unknown>)) {
    const id = (value as { id?: string }).id;
    return normalizeId(id);
  }
  return undefined;
};

/**
 * Produces a deterministic non-negative integer hash from a string.
 *
 * MUST return the absolute value of a 32-bit hash.
 **/
const hashString = (input: string): number => {
  let hash = 0;
  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index);
    hash |= 0;
  }
  return Math.abs(hash);
};

/**
 * Generates an HSL color tone from a port group key.
 *
 * MUST return a neutral grey tone for the default port ID.
 **/
const getToneForKey = (key: string): PortTone => {
  if (key === DEFAULT_PORT_ID) {
    return {
      base: "hsl(0 0% 48%)",
      surface: "hsla(0 0% 48% / 0.22)",
      surfaceStrong: "hsla(0 0% 48% / 0.35)",
      border: "hsl(0 0% 34%)",
      text: "hsl(0 0% 98%)",
    };
  }

  const hash = hashString(key);
  return SEMIO_PORT_TONES[hash % SEMIO_PORT_TONES.length];
};

/**
 * Builds a union-find map grouping compatible ports by root ID.
 *
 * MUST union ports linked via compatiblePorts relationships.
 **/
const createPortGroupMap = (ports: Port[]): Map<string, string> => {
  const parent = new Map<string, string>();

  for (const port of ports) {
    const id = normalizeId(port.id);
    if (!id) continue;
    parent.set(id, id);
  }

  const find = (id: string): string => {
    const direct = parent.get(id);
    if (!direct) return id;
    if (direct === id) return direct;
    const root = find(direct);
    parent.set(id, root);
    return root;
  };

  const union = (left: string, right: string) => {
    const leftRoot = find(left);
    const rightRoot = find(right);
    if (leftRoot === rightRoot) return;
    const leftHash = hashString(leftRoot);
    const rightHash = hashString(rightRoot);
    if (leftHash <= rightHash) parent.set(rightRoot, leftRoot);
    else parent.set(leftRoot, rightRoot);
  };

  for (const port of ports) {
    const id = normalizeId(port.id);
    if (!id) continue;
    const compatible = port.compatiblePorts ?? [];
    for (const relatedPort of compatible) {
      const relatedId = normalizeId(relatedPort.id);
      if (!relatedId || !parent.has(relatedId)) continue;
      union(id, relatedId);
    }
  }

  const groups = new Map<string, string>();
  for (const id of parent.keys()) {
    groups.set(id, find(id));
  }
  return groups;
};

/**
 * Extracts a normalized port ID from a string or port reference object.
 *
 * MUST delegate to normalizePortRef for consistent handling.
 **/
export const getPortId = (value: unknown): string | undefined => normalizePortRef(value);

/**
 * Extracts the port ID from a connector's port reference.
 *
 * MUST return undefined when the connector or its port is missing.
 **/
export const getConnectorPortId = (connector: Pick<Connector, "port"> | undefined | null): string | undefined => normalizePortRef(connector?.port);

/**
 * Resolves the color tone for a port based on its compatibility group.
 *
 * MUST return the default tone when the port ID is missing.
 **/
export const getPortTone = (portId: string | undefined, ports: Port[]): PortTone => {
  const normalizedId = normalizeId(portId);
  if (!normalizedId) return getToneForKey(DEFAULT_PORT_ID);
  const groups = createPortGroupMap(ports);
  const groupKey = groups.get(normalizedId) ?? normalizedId;
  return getToneForKey(groupKey);
};

/**
 * Determines the compatibility state of a candidate port relative to a selected port.
 *
 * MUST return none when no port is selected.
 **/
export const getPortCompatibilityState = (candidatePortId: string | undefined, selectedPortId: string | undefined, ports: Port[]): PortCompatibilityState => {
  const normalizedCandidate = normalizeId(candidatePortId);
  const normalizedSelected = normalizeId(selectedPortId);
  if (!normalizedSelected || !normalizedCandidate) return "none";
  const candidatePort = ports.find((port) => normalizeId(port.id) === normalizedCandidate);
  const selectedPort = ports.find((port) => normalizeId(port.id) === normalizedSelected);
  if (!candidatePort || !selectedPort) return "none";
  if (normalizedCandidate === normalizedSelected) return "compatible";
  return arePortsCompatible(candidatePort, selectedPort, ports) ? "compatible" : "incompatible";
};

// #region ðŸŽŠkitSelectionHelper
// Consolidated from kitSelectionHelper.ts

// #region 🔌Adapters
// Imports MUST include icon width constant and kit selection types.

// #endregion 🔌Adapters

// #region âš™ï¸Types
// Types MUST define selection value extraction for KitAppSelection dimensions.

/**
 * Extracts the element type from an array-valued KitAppSelection dimension.
 **/
export type SelectionValue<K extends keyof KitAppSelection> = NonNullable<KitAppSelection[K]> extends (infer T)[] ? T : never;

// #endregion âš™ï¸Types

// #region ðŸŽ­Generic Utilities
// Generic Utilities MUST provide immutable selection manipulation functions.

/**
 * Adds a value to the specified selection dimension array.
 * MUST return the original selection if the value is already present.
 **/
export function addToSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];

  if (currentArray.includes(value)) {
    return selection;
  }

  return {
    ...selection,
    [key]: [...currentArray, value],
  };
}

/**
 * Removes a value from the specified selection dimension array.
 * MUST remove the dimension key entirely when the array becomes empty.
 **/
export function removeFromSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  const newArray = currentArray.filter((v) => v !== value);

  if (newArray.length === 0) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }

  return {
    ...selection,
    [key]: newArray,
  };
}

/**
 * Toggles a value in the specified selection dimension array.
 * MUST add the value if absent or remove it if present.
 **/
export function toggleInSelection<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): KitAppSelection {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];

  if (currentArray.includes(value)) {
    return removeFromSelection(selection, key, value);
  } else {
    return addToSelection(selection, key, value);
  }
}

/**
 * Replaces an entire selection dimension with the given values.
 * MUST remove the dimension key when values are undefined or empty.
 **/
export function replaceSelectionDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, values: KitAppSelection[K] | undefined): KitAppSelection {
  if (!values || (Array.isArray(values) && values.length === 0)) {
    const { [key]: _, ...rest } = selection;
    return rest;
  }

  return {
    ...selection,
    [key]: values,
  };
}

/**
 * Removes an entire dimension from the selection.
 * MUST return a new selection object without the specified key.
 **/
export function clearSelectionDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K): KitAppSelection {
  const { [key]: _, ...rest } = selection;
  return rest;
}

/**
 * Returns an empty selection with all dimensions cleared.
 * MUST return a new empty KitAppSelection object.
 **/
export function clearSelection(): KitAppSelection {
  return {};
}

/**
 * Replaces a selection dimension with all available values.
 * MUST delegate to replaceSelectionDimension with the full value list.
 **/
export function selectAllInDimension<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, allValues: SelectionValue<K>[]): KitAppSelection {
  return replaceSelectionDimension(selection, key, allValues as KitAppSelection[K]);
}

/**
 * Checks whether a value is present in the specified selection dimension.
 * MUST return false when the dimension is undefined or empty.
 **/
export function isSelected<K extends keyof KitAppSelection>(selection: KitAppSelection, key: K, value: SelectionValue<K>): boolean {
  const currentArray = (selection[key] || []) as SelectionValue<K>[];
  return currentArray.includes(value);
}

// #endregion ðŸŽ­Generic Utilities

// #region ðŸ›’Kit Diagram Geometry
// Kit Diagram Geometry MUST provide geometry primitives, shape strategies, and anchor resolution.

/**
 * Union of diagram node kind identifiers mapped to shape strategies.
 **/
export type KitDiagramNodeKind = "type" | "design" | "quality" | "port" | "file" | "folder" | "author";
/**
 * Union of supported diagram shape identifiers.
 **/
export type KitDiagramShapeId = "circle" | "rectangle" | "triangle" | "long-rectangle";
/**
 * Union of cardinal snap sides for anchor point placement.
 **/
export type KitDiagramSnapSide = "top" | "right" | "bottom" | "left";

/**
 * Width and height dimensions of a diagram node frame.
 **/
export interface KitDiagramFrame {
  width: number;
  height: number;
}

/**
 * Two-dimensional coordinate point in diagram space.
 **/
export interface KitDiagramPoint {
  x: number;
  y: number;
}

/**
 * Named snap point on a shape boundary with directional side.
 **/
export interface KitDiagramSnapPoint extends KitDiagramPoint {
  id: string;
  side: KitDiagramSnapSide;
}

/**
 * Optional CSS class and style overrides for shape rendering.
 **/
export interface KitDiagramShapeRenderPayload {
  className?: string;
  style?: Record<string, string | number>;
}

/**
 * Shape strategy providing frame, snap points, and nearest-point resolution.
 **/
export interface KitDiagramShapeStrategy {
  id: KitDiagramShapeId;
  frame: KitDiagramFrame;
  getRenderPayload: () => KitDiagramShapeRenderPayload;
  getSnapPoints: (frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint[];
  resolveNearestPoint: (targetVector: KitDiagramPoint, frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint;
}

/**
 * Fully resolved anchor with local and absolute positions on a shape.
 **/
export interface KitDiagramResolvedAnchor {
  strategyId: KitDiagramShapeId;
  frame: KitDiagramFrame;
  localPoint: KitDiagramSnapPoint;
  absolutePoint: KitDiagramPoint;
  center: KitDiagramPoint;
}

/**
 * Input parameters for computing diagram node geometry.
 **/
export interface KitDiagramNodeGeometryInput {
  kind: KitDiagramNodeKind;
  position: KitDiagramPoint;
  frame?: Partial<KitDiagramFrame>;
}

/**
 * Pair of resolved anchors for source and target endpoints of a connection.
 **/
export interface KitDiagramResolvedAnchorPair {
  source: KitDiagramResolvedAnchor;
  target: KitDiagramResolvedAnchor;
}

/**
 * Proximity-based anchor result with distance from a target point.
 **/
export interface KitDiagramProximityAnchor {
  nodeId: string;
  distance: number;
  anchor: KitDiagramResolvedAnchor;
}

/**
 * @emoji ðŸ“ Kit diagram grid pitch in CSS pixels (view-only layout constant).
 **/
export const ICON_WIDTH = 50;

/**
 * Scale multiplier applied to icon width for diagram node sizing.
 **/
export const KIT_DIAGRAM_NODE_SCALE = 0.6;
/**
 * Base pixel size for diagram nodes derived from icon width and scale.
 **/
export const KIT_DIAGRAM_BASE_SIZE = ICON_WIDTH * KIT_DIAGRAM_NODE_SCALE;
/**
 * Default frame dimensions for circle-shaped diagram nodes.
 **/
export const KIT_DIAGRAM_CIRCLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
/**
 * Default frame dimensions for rectangle-shaped diagram nodes.
 **/
export const KIT_DIAGRAM_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.2), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.8) };
/**
 * Default frame dimensions for triangle-shaped diagram nodes.
 **/
export const KIT_DIAGRAM_TRIANGLE_FRAME: KitDiagramFrame = { width: KIT_DIAGRAM_BASE_SIZE, height: KIT_DIAGRAM_BASE_SIZE };
/**
 * Default frame dimensions for long-rectangle-shaped diagram nodes.
 **/
export const KIT_DIAGRAM_LONG_RECTANGLE_FRAME: KitDiagramFrame = { width: Math.round(KIT_DIAGRAM_BASE_SIZE * 1.6), height: Math.round(KIT_DIAGRAM_BASE_SIZE * 0.72) };
/**
 * Half of the largest frame dimension used as collision radius for force layout.
 **/
export const KIT_DIAGRAM_COLLIDE_RADIUS =
  Math.max(
    KIT_DIAGRAM_CIRCLE_FRAME.width,
    KIT_DIAGRAM_CIRCLE_FRAME.height,
    KIT_DIAGRAM_RECTANGLE_FRAME.width,
    KIT_DIAGRAM_RECTANGLE_FRAME.height,
    KIT_DIAGRAM_TRIANGLE_FRAME.width,
    KIT_DIAGRAM_TRIANGLE_FRAME.height,
    KIT_DIAGRAM_LONG_RECTANGLE_FRAME.width,
    KIT_DIAGRAM_LONG_RECTANGLE_FRAME.height,
  ) / 2;

/**
 * Validates and normalizes a partial frame to a complete frame with positive dimensions.
 **/
export const normalizeKitDiagramFrame = (frame?: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramFrame => {
  const width = frame?.width ?? fallback.width;
  const height = frame?.height ?? fallback.height;
  return {
    width: Number.isFinite(width) && width > 0 ? width : fallback.width,
    height: Number.isFinite(height) && height > 0 ? height : fallback.height,
  };
};

/**
 * Computes the center point of a diagram frame.
 **/
export const kitDiagramCenter = (frame: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramPoint => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  return { x: normalizedFrame.width / 2, y: normalizedFrame.height / 2 };
};

/**
 * Computes the direction vector from one point to another.
 **/
export const kitDiagramVector = (from: KitDiagramPoint, to: KitDiagramPoint): KitDiagramPoint => ({ x: to.x - from.x, y: to.y - from.y });
/**
 * Computes the Euclidean length of a vector.
 **/
export const kitDiagramVectorLength = (vector: KitDiagramPoint): number => Math.hypot(vector.x, vector.y);
/**
 * Returns a unit-length vector in the same direction or zero vector if length is zero.
 **/
export const kitDiagramNormalizeVector = (vector: KitDiagramPoint): KitDiagramPoint => {
  const length = kitDiagramVectorLength(vector);
  if (length === 0) return { x: 0, y: 0 };
  return { x: vector.x / length, y: vector.y / length };
};
/**
 * Computes the dot product of two vectors.
 **/
export const kitDiagramDot = (a: KitDiagramPoint, b: KitDiagramPoint): number => a.x * b.x + a.y * b.y;
/**
 * Computes the squared Euclidean distance between two points.
 **/
export const kitDiagramDistanceSquared = (a: KitDiagramPoint, b: KitDiagramPoint): number => {
  const dx = a.x - b.x;
  const dy = a.y - b.y;
  return dx * dx + dy * dy;
};
/**
 * Translates a local point to absolute coordinates by adding an origin offset.
 **/
export const kitDiagramToAbsolutePoint = (origin: KitDiagramPoint, localPoint: KitDiagramPoint): KitDiagramPoint => ({
  x: origin.x + localPoint.x,
  y: origin.y + localPoint.y,
});
/**
 * Infers the cardinal snap side of a point relative to the frame center.
 **/
export const kitDiagramInferSnapSide = (point: KitDiagramPoint, frame: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_CIRCLE_FRAME): KitDiagramSnapSide => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  const dx = point.x - center.x;
  const dy = point.y - center.y;
  if (Math.abs(dx) > Math.abs(dy)) {
    return dx >= 0 ? "right" : "left";
  }
  return dy >= 0 ? "bottom" : "top";
};

/**
 * createCircleSnapPoints holds the data fields for a createCircleSnapPoints record.
 **/
const createCircleSnapPoints = (frame?: Partial<KitDiagramFrame>): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, KIT_DIAGRAM_CIRCLE_FRAME);
  const center = kitDiagramCenter(normalizedFrame, KIT_DIAGRAM_CIRCLE_FRAME);
  return [
    { id: "n", x: center.x, y: 0, side: "top" },
    { id: "e", x: normalizedFrame.width, y: center.y, side: "right" },
    { id: "s", x: center.x, y: normalizedFrame.height, side: "bottom" },
    { id: "w", x: 0, y: center.y, side: "left" },
  ];
};

/**
 * createRectangleSnapPoints holds the data fields for a createRectangleSnapPoints record.
 **/
const createRectangleSnapPoints = (frame?: Partial<KitDiagramFrame>, fallback: KitDiagramFrame = KIT_DIAGRAM_RECTANGLE_FRAME): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  return [
    { id: "n", x: center.x, y: 0, side: "top" },
    { id: "e", x: normalizedFrame.width, y: center.y, side: "right" },
    { id: "s", x: center.x, y: normalizedFrame.height, side: "bottom" },
    { id: "w", x: 0, y: center.y, side: "left" },
  ];
};

/**
 * createTriangleSnapPoints holds the data fields for a createTriangleSnapPoints record.
 **/
const createTriangleSnapPoints = (frame?: Partial<KitDiagramFrame>): KitDiagramSnapPoint[] => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, KIT_DIAGRAM_TRIANGLE_FRAME);
  return [
    { id: "apex", x: normalizedFrame.width / 2, y: 0, side: "top" },
    { id: "base-left", x: 0, y: normalizedFrame.height, side: "left" },
    { id: "base-right", x: normalizedFrame.width, y: normalizedFrame.height, side: "right" },
  ];
};

/** rankSnapPointsByVector holds the data fields for a rankSnapPointsByVector record.
 **/
const rankSnapPointsByVector = (points: KitDiagramSnapPoint[], frame: Partial<KitDiagramFrame>, targetVector: KitDiagramPoint, fallback: KitDiagramFrame): Array<{ point: KitDiagramSnapPoint; alignment: number; orthogonal: number }> => {
  const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
  const center = kitDiagramCenter(normalizedFrame, fallback);
  const normalizedTarget = kitDiagramNormalizeVector(targetVector);
  return points
    .map((point) => {
      const fromCenter = kitDiagramVector(center, point);
      const normalizedDirection = kitDiagramNormalizeVector(fromCenter);
      const alignment = kitDiagramDot(normalizedDirection, normalizedTarget);
      const projection = kitDiagramDot(fromCenter, normalizedTarget);
      const projectedPoint = {
        x: normalizedTarget.x * projection,
        y: normalizedTarget.y * projection,
      };
      const orthogonal = kitDiagramVectorLength({
        x: fromCenter.x - projectedPoint.x,
        y: fromCenter.y - projectedPoint.y,
      });
      return { point, alignment, orthogonal };
    })
    .sort((a, b) => {
      if (b.alignment !== a.alignment) return b.alignment - a.alignment;
      if (a.orthogonal !== b.orthogonal) return a.orthogonal - b.orthogonal;
      return a.point.id.localeCompare(b.point.id);
    });
};

/**
 * Selects the snap point best aligned with a target vector direction.
 **/
export const resolveNearestKitDiagramSnapPoint = (points: KitDiagramSnapPoint[], frame: Partial<KitDiagramFrame>, targetVector: KitDiagramPoint, fallback: KitDiagramFrame): KitDiagramSnapPoint => {
  if (points.length === 0) {
    const normalizedFrame = normalizeKitDiagramFrame(frame, fallback);
    const center = kitDiagramCenter(normalizedFrame, fallback);
    return { id: "center", ...center, side: kitDiagramInferSnapSide(center, normalizedFrame, fallback) };
  }
  const ranked = rankSnapPointsByVector(points, frame, targetVector, fallback);
  return ranked[0]?.point ?? points[0];
};

/**
 * createStrategy holds the data fields for a createStrategy record.
 **/
const createStrategy = (id: KitDiagramShapeId, frame: KitDiagramFrame, getSnapPoints: (frame?: Partial<KitDiagramFrame>) => KitDiagramSnapPoint[], renderPayload: KitDiagramShapeRenderPayload): KitDiagramShapeStrategy => ({
  id,
  frame,
  getRenderPayload: () => renderPayload,
  getSnapPoints: (frameOverride?: Partial<KitDiagramFrame>) => getSnapPoints(frameOverride ?? frame),
  resolveNearestPoint: (targetVector: KitDiagramPoint, frameOverride?: Partial<KitDiagramFrame>) => {
    const resolvedFrame = normalizeKitDiagramFrame(frameOverride, frame);
    const points = getSnapPoints(resolvedFrame);
    return resolveNearestKitDiagramSnapPoint(points, resolvedFrame, targetVector, frame);
  },
});

/**
 * Shape strategy for circle-shaped diagram nodes.
 **/
export const kitDiagramCircleStrategy = createStrategy("circle", KIT_DIAGRAM_CIRCLE_FRAME, createCircleSnapPoints, {});
/**
 * Shape strategy for rectangle-shaped diagram nodes.
 **/
export const kitDiagramRectangleStrategy = createStrategy("rectangle", KIT_DIAGRAM_RECTANGLE_FRAME, createRectangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});
/**
 * Shape strategy for triangle-shaped diagram nodes.
 **/
export const kitDiagramTriangleStrategy = createStrategy("triangle", KIT_DIAGRAM_TRIANGLE_FRAME, createTriangleSnapPoints, {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
  style: { clipPath: "polygon(50% 0%, 0% 100%, 100% 100%)" },
});
/**
 * Shape strategy for long-rectangle-shaped diagram nodes.
 **/
export const kitDiagramLongRectangleStrategy = createStrategy("long-rectangle", KIT_DIAGRAM_LONG_RECTANGLE_FRAME, (frame) => createRectangleSnapPoints(frame, KIT_DIAGRAM_LONG_RECTANGLE_FRAME), {
  className: "!rounded-none [&_[data-slot=avatar-fallback]]:!rounded-none",
});

/**
 * Fallback shape strategy used when no kind-specific strategy is registered.
 **/
export const KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY = kitDiagramLongRectangleStrategy;

/**
 * Registry mapping each node kind to its associated shape strategy.
 **/
export const KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY: Record<KitDiagramNodeKind, KitDiagramShapeStrategy> = {
  design: kitDiagramCircleStrategy,
  type: kitDiagramRectangleStrategy,
  file: kitDiagramTriangleStrategy,
  quality: kitDiagramLongRectangleStrategy,
  port: kitDiagramLongRectangleStrategy,
  folder: kitDiagramLongRectangleStrategy,
  author: kitDiagramLongRectangleStrategy,
};

/**
 * Looks up the shape strategy for a given node kind with fallback to default.
 **/
export const getKitDiagramShapeStrategy = (kind: KitDiagramNodeKind): KitDiagramShapeStrategy => KIT_DIAGRAM_SHAPE_STRATEGY_REGISTRY[kind] ?? KIT_DIAGRAM_DEFAULT_SHAPE_STRATEGY;

/**
 * Returns the normalized frame dimensions for a given node kind with optional override.
 **/
export const getKitDiagramNodeFrameForKind = (kind: KitDiagramNodeKind, override?: Partial<KitDiagramFrame>): KitDiagramFrame => normalizeKitDiagramFrame(override, getKitDiagramShapeStrategy(kind).frame);

/**
 * Resolves the optimal anchor pair between two diagram nodes for edge routing.
 **/
export const resolveKitDiagramAnchorPair = (sourceNode: KitDiagramNodeGeometryInput, targetNode: KitDiagramNodeGeometryInput): KitDiagramResolvedAnchorPair => {
  const sourceStrategy = getKitDiagramShapeStrategy(sourceNode.kind);
  const targetStrategy = getKitDiagramShapeStrategy(targetNode.kind);
  const sourceFrame = normalizeKitDiagramFrame(sourceNode.frame, sourceStrategy.frame);
  const targetFrame = normalizeKitDiagramFrame(targetNode.frame, targetStrategy.frame);
  const sourceCenterLocal = kitDiagramCenter(sourceFrame, sourceStrategy.frame);
  const targetCenterLocal = kitDiagramCenter(targetFrame, targetStrategy.frame);
  const sourceCenterAbsolute = kitDiagramToAbsolutePoint(sourceNode.position, sourceCenterLocal);
  const targetCenterAbsolute = kitDiagramToAbsolutePoint(targetNode.position, targetCenterLocal);
  const direction = kitDiagramVector(sourceCenterAbsolute, targetCenterAbsolute);
  const reverseDirection = { x: -direction.x, y: -direction.y };
  const sourceRanked = rankSnapPointsByVector(sourceStrategy.getSnapPoints(sourceFrame), sourceFrame, direction, sourceStrategy.frame);
  const targetRanked = rankSnapPointsByVector(targetStrategy.getSnapPoints(targetFrame), targetFrame, reverseDirection, targetStrategy.frame);
  const sourceCandidates = sourceRanked.slice(0, Math.min(3, sourceRanked.length));
  const targetCandidates = targetRanked.slice(0, Math.min(3, targetRanked.length));
  let best:
    | {
        score: number;
        sourcePoint: KitDiagramSnapPoint;
        targetPoint: KitDiagramSnapPoint;
      }
    | undefined;

  for (const sourceCandidate of sourceCandidates) {
    for (const targetCandidate of targetCandidates) {
      const sourceAbsolute = kitDiagramToAbsolutePoint(sourceNode.position, sourceCandidate.point);
      const targetAbsolute = kitDiagramToAbsolutePoint(targetNode.position, targetCandidate.point);
      const distanceScore = kitDiagramDistanceSquared(sourceAbsolute, targetAbsolute);
      const alignmentScore = sourceCandidate.alignment + targetCandidate.alignment;
      const score = distanceScore - alignmentScore * (sourceFrame.width + targetFrame.width) * 24;
      if (!best || score < best.score) {
        best = {
          score,
          sourcePoint: sourceCandidate.point,
          targetPoint: targetCandidate.point,
        };
      }
    }
  }

  const sourcePoint = best?.sourcePoint ?? sourceStrategy.resolveNearestPoint(direction, sourceFrame);
  const targetPoint = best?.targetPoint ?? targetStrategy.resolveNearestPoint(reverseDirection, targetFrame);

  return {
    source: {
      strategyId: sourceStrategy.id,
      frame: sourceFrame,
      localPoint: sourcePoint,
      absolutePoint: kitDiagramToAbsolutePoint(sourceNode.position, sourcePoint),
      center: sourceCenterAbsolute,
    },
    target: {
      strategyId: targetStrategy.id,
      frame: targetFrame,
      localPoint: targetPoint,
      absolutePoint: kitDiagramToAbsolutePoint(targetNode.position, targetPoint),
      center: targetCenterAbsolute,
    },
  };
};

/**
 * Finds the closest snap point on a node to a given target point for proximity-based connections.
 **/
export const resolveKitDiagramProximityAnchor = (nodeId: string, node: KitDiagramNodeGeometryInput, targetPoint: KitDiagramPoint): KitDiagramProximityAnchor => {
  const strategy = getKitDiagramShapeStrategy(node.kind);
  const frame = normalizeKitDiagramFrame(node.frame, strategy.frame);
  const points = strategy.getSnapPoints(frame);
  const bestPoint = points.reduce(
    (best, point) => {
      const absolutePoint = kitDiagramToAbsolutePoint(node.position, point);
      const distance = Math.sqrt(kitDiagramDistanceSquared(absolutePoint, targetPoint));
      if (!best || distance < best.distance) {
        return { point, absolutePoint, distance };
      }
      return best;
    },
    null as null | { point: KitDiagramSnapPoint; absolutePoint: KitDiagramPoint; distance: number },
  );
  const resolvedPoint = bestPoint?.point ?? strategy.resolveNearestPoint(kitDiagramVector(kitDiagramCenter(frame, strategy.frame), targetPoint), frame);
  const resolvedAbsolutePoint = bestPoint?.absolutePoint ?? kitDiagramToAbsolutePoint(node.position, resolvedPoint);
  return {
    nodeId,
    distance: bestPoint?.distance ?? Math.sqrt(kitDiagramDistanceSquared(resolvedAbsolutePoint, targetPoint)),
    anchor: {
      strategyId: strategy.id,
      frame,
      localPoint: resolvedPoint,
      absolutePoint: resolvedAbsolutePoint,
      center: kitDiagramToAbsolutePoint(node.position, kitDiagramCenter(frame, strategy.frame)),
    },
  };
};

// #endregion ðŸ›’Kit Diagram Geometry

//#region ðŸ”–SketchpadDeclarativeShell
//#region 🔖SketchpadRouteScope
/** @emoji 🧭 Kit/design/type ids parsed from a sketchpad URL path (render-agnostic). */
export function parseSketchpadRouteScopeFromPath(path: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly docsPath: string;
} {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") {
		const docsPath = pathParts.slice(1).join("/") || "index";
		return { kitId: null, designId: null, typeId: null, docsPath };
	}
	if (pathParts[0] !== "kits") {
		return { kitId: null, designId: null, typeId: null, docsPath: "index" };
	}
	const kitId = pathParts[1] && isUuidPattern(pathParts[1]) ? pathParts[1] : null;
	const designId = pathParts[2] === "designs" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	const typeId = pathParts[2] === "types" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	return { kitId, designId, typeId, docsPath: "index" };
}
//#endregion 🔖SketchpadRouteScope

export const SKETCHPAD_SHELL_CONTROLLER_ID = "semio.sketchpad.shell";
const SKETCHPAD_EXTENSION_ID = "semio.sketchpad.builtin";
export const SKETCHPAD_HOME_APP_ID = "home";
export const SKETCHPAD_KIT_APP_ID = "kit";
export const SKETCHPAD_DESIGN_APP_ID = "design";
export const SKETCHPAD_TYPE_APP_ID = "type";
export const SKETCHPAD_DOCS_APP_ID = "docs";
export const SKETCHPAD_FEEDBACK_APP_ID = "feedback";
const SKETCHPAD_BODY_HOME = "semio.sketchpad.window.home";
const SKETCHPAD_BODY_KIT_TABLE = "semio.sketchpad.window.kit.table";
const SKETCHPAD_BODY_KIT_DIAGRAM = "semio.sketchpad.window.kit.diagram";
const SKETCHPAD_BODY_DESIGN_SCENE = "semio.sketchpad.window.design.scene";
const SKETCHPAD_BODY_DESIGN_DIAGRAM = "semio.sketchpad.window.design.diagram";
const SKETCHPAD_BODY_TYPE = "semio.sketchpad.window.type";
const SKETCHPAD_BODY_DOCS = "semio.sketchpad.window.docs";
const SKETCHPAD_BODY_FEEDBACK = "semio.sketchpad.window.feedback";
const SKETCHPAD_SURFACE_KIT_TABLE = "semio.sketchpad.surface.kit.table/v1";
const SKETCHPAD_SURFACE_KIT_DIAGRAM = "semio.sketchpad.surface.kit.diagram/v1";
const SKETCHPAD_SURFACE_DESIGN_SCENE = "semio.sketchpad.surface.design.scene/v1";
const SKETCHPAD_SURFACE_DESIGN_DIAGRAM = "semio.sketchpad.surface.design.diagram/v1";
const SKETCHPAD_SURFACE_PANEL_MAIN = "semio.sketchpad.surface.panel.main/v1";
const SKETCHPAD_SURFACE_HOME_TABLE = "semio.sketchpad.surface.home.table/v1";
const SKETCHPAD_SURFACE_TYPE_SCENE = "semio.sketchpad.surface.type.scene/v1";
const SKETCHPAD_SURFACE_DOCS_PAGE = "semio.sketchpad.surface.docs.page/v1";
const SKETCHPAD_SURFACE_FEEDBACK_FORM = "semio.sketchpad.surface.feedback.form/v1";
const SKETCHPAD_PANEL_WORKBENCH_BODY = "semio.sketchpad.panel.workbench";
const SKETCHPAD_PANEL_DETAILS_BODY = "semio.sketchpad.panel.details";

//#region 🔖SketchpadPlatformComponents
abstract class SketchpadRoutedComponent<TModel> extends Component<TModel> {
	protected route = parseSketchpadRouteScopeFromPath("/");
	private readonly detachRoute: () => void;
	private readonly detachKitRegistry?: () => void;
	private detachKitStore?: () => void;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialModel: TModel, platform: Platform) {
		super(componentKind, surfaceId, controllerId, initialModel);
		this.route = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
		this.detachRoute = platform.subscribe(() => {
			const nextRoute = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
			if (
				nextRoute.kitId !== this.route.kitId ||
				nextRoute.designId !== this.route.designId ||
				nextRoute.typeId !== this.route.typeId ||
				nextRoute.docsPath !== this.route.docsPath
			) {
				this.route = nextRoute;
				this.attachActiveKitStore();
				this.refresh();
			}
		});
		const registry = getKitRegistryBridge() as { subscribe?: (listener: () => void) => () => void } | null;
		if (registry?.subscribe) {
			this.detachKitRegistry = registry.subscribe(() => this.refresh());
		}
		this.attachActiveKitStore();
	}

	protected attachActiveKitStore(): void {
		this.detachKitStore?.();
		this.detachKitStore = undefined;
		const { kitId } = this.route;
		if (!kitId) return;
		const store = getKitRegistryBridge()?.get(kitId)?.store as { subscribe?: (listener: () => void) => () => void } | undefined;
		if (store?.subscribe) {
			this.detachKitStore = store.subscribe(() => this.refresh());
		}
	}

	dispose(): void {
		this.detachRoute();
		this.detachKitRegistry?.();
		this.detachKitStore?.();
	}
}

/** @emoji 🏠 Home kits table backed by the kit registry bridge. */
export class SketchpadHomeTable extends Table {
	constructor(platform: Platform) {
		super(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID);
		platform.subscribe(() => this.refresh());
		const registry = getKitRegistryBridge() as { subscribe?: (listener: () => void) => () => void } | null;
		if (registry?.subscribe) {
			registry.subscribe(() => this.refresh());
		}
	}

	override buildModel(): TableModel {
		const registry = getKitRegistryBridge();
		const ids = registry?.list() ?? [];
		return {
			columns: [
				{ id: "name", label: "Name" },
				{ id: "kind", label: "Kind" },
			],
			rows: ids.map((id) => {
				let name = id;
				let kind = "";
				try {
					const snapshot = registry?.get(id)?.store?.getSnapshot?.();
					const kit = snapshot?.kit as Kit | undefined;
					if (kit?.name) name = kit.name;
					kind = (registry?.get(id) as { kind?: string } | undefined)?.kind ?? "";
				} catch {
					/* registry row may still be opening */
				}
				return { id, cells: { name, kind }, navigateUri: `/kits/${id}` };
			}),
			emptyMessage: "No kits open — use Open to add kits",
		};
	}
}

/** @emoji 📊 Active kit table surface. */
export class SketchpadKitTable extends SketchpadRoutedComponent<TableModel> {
	constructor(platform: Platform) {
		super("table", SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, { columns: [], rows: [] }, platform);
	}

	override buildModel(): TableModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { columns: [], rows: [], emptyMessage: "Open a kit to view the table" };
		}
		const registry = getKitRegistryBridge();
		const store = registry?.get(kitId)?.store;
		if (!store) {
			return { columns: [], rows: [], emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit as Kit;
		const types = kit.types ?? [];
		const designs = kit.designs ?? [];
		return {
			columns: [
				{ id: "name", label: "Name" },
				{ id: "kind", label: "Kind" },
			],
			rows: [
				...types
					.filter((t): t is Type => typeof t === "object" && t !== null && "id" in t)
					.map((t) => ({
						id: `type:${t.id}`,
						cells: { name: t.name ?? t.id, kind: "type" },
						navigateUri: `/kits/${kitId}/types/${t.id}`,
					})),
				...designs
					.filter((d): d is Design => typeof d === "object" && d !== null && "id" in d)
					.map((d) => ({
						id: `design:${d.id}`,
						cells: { name: d.name ?? d.id, kind: "design" },
						navigateUri: `/kits/${kitId}/designs/${d.id}`,
					})),
			],
			emptyMessage: "No types or designs in this kit",
		};
	}
}

/** @emoji 📋 Kit diagram surface (topology summary as nodes). */
export class SketchpadKitDiagram extends SketchpadRoutedComponent<Puzzle2dModel> {
	constructor(platform: Platform) {
		super("puzzle2d", SKETCHPAD_SURFACE_KIT_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, { nodes: [], edges: [] }, platform);
	}

	override buildModel(): Puzzle2dModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { nodes: [], edges: [], emptyMessage: "Open a kit to view the diagram" };
		}
		const registry = getKitRegistryBridge();
		const store = registry?.get(kitId)?.store;
		if (!store) {
			return { nodes: [], edges: [], emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit as Kit;
		const nodes = (kit.types ?? []).map((t, index) => ({
			id: `type:${t.name}`,
			label: t.name,
			x: (index % 6) * 120,
			y: Math.floor(index / 6) * 80,
		}));
		return { nodes, edges: [], emptyMessage: nodes.length ? undefined : "No types to diagram" };
	}
}

/** @emoji 🎬 Design scene (5D volume). */
export class SketchpadDesignScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_SCENE,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE },
			platform,
		);
	}

	override buildModel(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE, emptyMessage: "Open a design to view the scene" };
		}
		return { presentation: "volume", instanceId: `${kitId}:${designId}:scene` };
	}
}

/** @emoji 📐 Design diagram (5D flat). */
export class SketchpadDesignDiagram extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_DIAGRAM,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM },
			platform,
		);
	}

	override buildModel(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM, emptyMessage: "Open a design to view the diagram" };
		}
		return { presentation: "flat", instanceId: `${kitId}:${designId}:diagram` };
	}
}

/** @emoji 📐 Type CAD surface. */
export class SketchpadTypeCad extends SketchpadRoutedComponent<CadModel> {
	constructor(platform: Platform) {
		super("cad", SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, {}, platform);
	}

	override buildModel(): CadModel {
		const { kitId, typeId } = this.route;
		if (!kitId || !typeId) {
			return { emptyMessage: "Open a type to view the CAD scene" };
		}
		return { instanceId: `${kitId}:${typeId}` };
	}
}

/** @emoji 📄 Docs panel surface. */
export class SketchpadDocsPanel extends SketchpadRoutedComponent<PanelModel> {
	constructor(platform: Platform) {
		super("panel", SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID, { body: { type: "text", value: "Docs" } }, platform);
	}

	override buildModel(): PanelModel {
		return {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: `Docs · ${this.route.docsPath}`, emphasize: true },
					{ type: "text", value: "Navigate to /docs/… to browse documentation." },
				],
			},
		};
	}
}

/** @emoji 💬 Feedback panel surface. */
export class SketchpadFeedbackPanel extends Panel {
	constructor(_platform: Platform) {
		super(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID, {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: "Feedback", emphasize: true },
					{ type: "text", value: "Send feedback from the footer or command palette." },
				],
			},
		});
	}
}

/** @emoji 🧩 Workbench side panel placeholder. */
class SketchpadWorkbenchPanel extends Panel {
	constructor() {
		super(SKETCHPAD_SURFACE_PANEL_MAIN, SKETCHPAD_SHELL_CONTROLLER_ID, {
			body: { type: "text", value: "Workbench panel" },
		});
	}
}

/** @emoji 🧱 Registers all sketchpad {@link Component} instances on a {@link Platform}. */
class SketchpadPlatformComponents {
	readonly components: readonly Component<unknown>[];

	constructor(platform: Platform) {
		this.components = [
			new SketchpadHomeTable(platform),
			new SketchpadKitTable(platform),
			new SketchpadKitDiagram(platform),
			new SketchpadDesignScene(platform),
			new SketchpadDesignDiagram(platform),
			new SketchpadTypeCad(platform),
			new SketchpadDocsPanel(platform),
			new SketchpadFeedbackPanel(platform),
			new SketchpadWorkbenchPanel(),
		];
		for (const component of this.components) {
			registerPlatformComponent(platform, component);
			component.refresh();
		}
		platform.subscribe(() => {
			for (const component of this.components) {
				component.refresh();
			}
		});
	}
}

let sketchpadPlatformComponents: SketchpadPlatformComponents | null = null;
//#endregion 🔖SketchpadPlatformComponents

/** @emoji 🧭 Routes sketchpad navigation and panel chrome through {@link CommandBus}. */
export class SketchpadShellController extends Controller {
	navigationPath = "/";
	panelVisibility = { leftSidePanel: false, rightSidePanel: false };

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SKETCHPAD_SHELL_CONTROLLER_ID, commandBus, hostNotify);
	}

	override run(command: string, args?: unknown): void {
		switch (command) {
			case "setNavigation": {
				this.navigationPath = (args as { path: string }).path;
				break;
			}
			case "togglePanel": {
				const panel = (args as { panel: "leftSidePanel" | "rightSidePanel" }).panel;
				this.panelVisibility = { ...this.panelVisibility, [panel]: !this.panelVisibility[panel] };
				break;
			}
			default:
				break;
		}
		this.emit();
	}
}

let sketchpadPlatformSingleton: Platform | null = null;
let sketchpadPluginHostSingleton: PluginHost | null = null;
let sketchpadShellReady: Promise<Platform> | null = null;
let sketchpadChromeRegistered = false;

export function sketchpadAppIdFromPath(path: string): string {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") return SKETCHPAD_DOCS_APP_ID;
	if (pathParts[0] === "feedback") return SKETCHPAD_FEEDBACK_APP_ID;
	if (pathParts[0] !== "kits") return SKETCHPAD_HOME_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "designs" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_DESIGN_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "types" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_TYPE_APP_ID;
	if (pathParts.length >= 2 && isUuidPattern(pathParts[1] ?? "")) return SKETCHPAD_KIT_APP_ID;
	return SKETCHPAD_HOME_APP_ID;
}

function buildSketchpadExtensionManifest(): PluginManifest {
	return {
		id: SKETCHPAD_EXTENSION_ID,
		label: "Semio Sketchpad",
		contributes: {
			apps: [
				{
					id: SKETCHPAD_HOME_APP_ID,
					label: "Home",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "home-main", label: "Home", bodyKey: SKETCHPAD_BODY_HOME }],
					defaultLayout: createTabStackLayout(["home-main"], ["Home"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_KIT_APP_ID,
					label: "Kit",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "table", label: "Table", bodyKey: SKETCHPAD_BODY_KIT_TABLE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_KIT_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["table", "diagram"], "row", [50, 50], ["Table", "Diagram"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_DESIGN_APP_ID,
					label: "Design",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "scene", label: "Scene", bodyKey: SKETCHPAD_BODY_DESIGN_SCENE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_DESIGN_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["scene", "diagram"], "row", [60, 40], ["Scene", "Diagram"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_TYPE_APP_ID,
					label: "Type",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "type-main", label: "Type", bodyKey: SKETCHPAD_BODY_TYPE }],
					defaultLayout: createTabStackLayout(["type-main"], ["Type"]),
					leftTabs: [{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY }],
					rightTabs: [{ id: "details", iconId: "semio.sketchpad.icon.details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY }],
				},
				{
					id: SKETCHPAD_DOCS_APP_ID,
					label: "Docs",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "docs-main", label: "Docs", bodyKey: SKETCHPAD_BODY_DOCS }],
					defaultLayout: createTabStackLayout(["docs-main"], ["Docs"]),
				},
				{
					id: SKETCHPAD_FEEDBACK_APP_ID,
					label: "Feedback",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "feedback-main", label: "Feedback", bodyKey: SKETCHPAD_BODY_FEEDBACK }],
					defaultLayout: createTabStackLayout(["feedback-main"], ["Feedback"]),
				},
			],
		},
	};
}

function declarativePanelMain(_ctx: WindowBodyViewContext): UiNode {
	return buildPanelWindowBody(SKETCHPAD_SURFACE_PANEL_MAIN, SKETCHPAD_SHELL_CONTROLLER_ID);
}

function registerSketchpadDeclarativeBodies(): void {
	registerWindowBody(SKETCHPAD_BODY_HOME, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "home-main"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_TABLE, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "table"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_DIAGRAM, () =>
		buildPuzzle2dWindowBody(SKETCHPAD_SURFACE_KIT_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_SCENE, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, "scene"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_DIAGRAM, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_TYPE, () => buildCadWindowBody(SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_DOCS, () => buildPanelWindowBody(SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_FEEDBACK, () => buildPanelWindowBody(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerSidePanelBody(SKETCHPAD_PANEL_WORKBENCH_BODY, declarativePanelMain);
	registerSidePanelBody(SKETCHPAD_PANEL_DETAILS_BODY, declarativePanelMain);
}

function registerSketchpadSurfaceHosts(): void {
	if (sketchpadChromeRegistered) return;
	sketchpadChromeRegistered = true;
	registerSketchpadDeclarativeBodies();
}

const SKETCHPAD_PLATFORM_SPEC: PlatformSpec = {
	id: "semio.sketchpad",
	name: "Semio Sketchpad",
	defaultActiveAppId: SKETCHPAD_HOME_APP_ID,
};

/** @emoji 🧱 Builds the declarative sketchpad {@link Platform} instance (apps, window kinds, surface bindings). */
export async function buildSketchpadPlatform(): Promise<Platform> {
	registerSketchpadSurfaceHosts();
	const platform = new Platform(SKETCHPAD_PLATFORM_SPEC);
	const controller = new SketchpadShellController(platform.commandBus, () => platform.notify());
	const host = new PluginHost(platform);
	host.register(buildSketchpadExtensionManifest(), {
		id: SKETCHPAD_EXTENSION_ID,
		activate() {},
	} satisfies PluginModule);
	await host.activateAll((controllerId) => (controllerId === SKETCHPAD_SHELL_CONTROLLER_ID ? controller : undefined));
	platform.activeAppId = SKETCHPAD_HOME_APP_ID;
	sketchpadPlatformComponents = new SketchpadPlatformComponents(platform);
	if (typeof window !== "undefined" && window.location) {
		platform.uri = `${window.location.pathname}${window.location.search}`;
		platform.activeAppId = sketchpadAppIdFromPath(platform.uri.split("?")[0] ?? "/");
	}
	platform.notify();
	sketchpadPlatformSingleton = platform;
	sketchpadPluginHostSingleton = host;
	return platform;
}

/** @emoji 🚀 Ensures the sketchpad {@link Platform} shell is initialized once per session. */
export async function ensureSketchpadPlatform(): Promise<Platform> {
	if (sketchpadPlatformSingleton) return sketchpadPlatformSingleton;
	if (!sketchpadShellReady) {
		sketchpadShellReady = buildSketchpadPlatform();
	}
	return sketchpadShellReady;
}

/** @emoji 🚀 @deprecated Use {@link ensureSketchpadPlatform}. */
export const ensureSketchpadDeclarativeShell = ensureSketchpadPlatform;

export function getSketchpadPlatform(): Platform | null {
	return sketchpadPlatformSingleton;
}

/** @emoji 🔍 @deprecated Use {@link getSketchpadPlatform}. */
export const getSketchpadProductRuntime = getSketchpadPlatform;


//#region 🔖KitRegistryBridge
/** @emoji 🌉 Global kit registry bridge wired by the React shell (no React in this package). */
export type SketchpadKitRegistryEntry = {
	readonly store?: KitHostStore;
	readonly kind?: string;
	readonly persistence?: { readonly kind?: string };
};

export type SketchpadKitRegistryBridge = {
	list(): readonly string[];
	get(id: string): SketchpadKitRegistryEntry | undefined;
	subscribe?(listener: () => void): () => void;
};

let sketchpadKitRegistryBridge: SketchpadKitRegistryBridge | null = null;

/** @emoji 🔌 Installs the kit registry bridge from the React shell. */
export function setSketchpadKitRegistryBridge(bridge: SketchpadKitRegistryBridge | null): void {
	sketchpadKitRegistryBridge = bridge;
}

/** @emoji 🔍 Returns the active kit registry bridge (shell-owned). */
export function getKitRegistryBridge(): SketchpadKitRegistryBridge | null {
	return sketchpadKitRegistryBridge;
}
//#endregion 🔖KitRegistryBridge

