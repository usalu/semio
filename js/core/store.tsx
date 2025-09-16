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

import JSZip from "jszip";
import React, { createContext, useContext, useMemo, useSyncExternalStore } from "react";
import type { Database, SqlJsStatic } from "sql.js";
import initSqlJs from "sql.js";
import sqlWasmUrl from "sql.js/dist/sql-wasm.wasm?url";
import { v4 as uuidv4 } from "uuid";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import {
  applyDesignDiff,
  applyKitDiff,
  Attribute,
  Author,
  Camera,
  colorPortsForTypes,
  Connection,
  ConnectionDiff,
  ConnectionId,
  connectionIdToString,
  Coord,
  Design,
  DesignDiff,
  DesignId,
  designIdToString,
  DesignShallow,
  DiffStatus,
  FileDiff,
  findDesignInKit,
  findPieceInDesign,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  flattenDesign,
  getClusterableGroups,
  getIncludedDesigns,
  getPieceRepresentationUrls,
  inverseKitDiff,
  Kit,
  KitDiff,
  KitId,
  KitIdLike,
  kitIdLikeToKitId,
  kitIdToString,
  KitShallow,
  Piece,
  PieceDiff,
  PieceId,
  pieceIdToString,
  piecesMetadata,
  Plane,
  Port,
  PortDiff,
  PortId,
  portIdToString,
  Representation,
  RepresentationDiff,
  RepresentationId,
  representationIdToString,
  File as SemioFile,
  Type,
  TypeDiff,
  TypeId,
  typeIdToString,
} from "./semio";

// #region Constants

export enum Mode {
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

// #endregion Constants

// #region Api

export type Subscribe = () => void;
export type Unsubscribe = () => void;
export type Disposable = () => void;
export type Transact = (fn: () => void) => void;
export type Url = string;

export interface Snapshot<TModel> {
  snapshot(): TModel;
}
export interface Actions<TDiff> {
  change: (diff: TDiff) => void;
}
export interface OtherPresence {
  name: string;
}
export interface EditorSelection<TSelection> {
  selection: TSelection;
}
export interface EditorPresence<TPresence> {
  presence: TPresence;
}
export interface EditorChangableState<TSelection, TPresence> extends EditorSelection<TSelection>, EditorPresence<TPresence> {}
export interface EditorEdit<TEditorStep> {
  do: TEditorStep;
  undo: TEditorStep;
}
export interface EditorState<TDiff, TPresenceOther, TSelection, TPresence, TEdit> extends EditorChangableState<TSelection, TPresence> {
  isTransactionActive: boolean;
  canUndo: boolean;
  canRedo: boolean;
  others: TPresenceOther[];
  diff: TDiff;
  currentTransactionStack: TEdit[];
  pastTransactionsStack: TEdit[];
}
export interface EditorSelectionActions<TSelectionDiff> {
  selectAll: () => void;
  deselectAll: () => void;
  changeSelection: (diff: TSelectionDiff) => void;
}
export interface EditorPresenceActions<TPresence> {
  setPresence: (presence: TPresence) => void;
}
export interface EditorActions<TDiff, TSelectionDiff, TPresence> extends Actions<TDiff>, EditorSelectionActions<TSelectionDiff>, EditorPresenceActions<TPresence> {
  undo: () => void;
  redo: () => void;
  startTransaction: () => void;
  abortTransaction: () => void;
  finalizeTransaction: () => void;
}
export interface Subscriptions {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
}
export interface EditorSubscriptions extends Subscriptions {
  onUndone: (subscribe: Subscribe) => Unsubscribe;
  onRedone: (subscribe: Subscribe) => Unsubscribe;
  onTransactionStarted: (subscribe: Subscribe) => Unsubscribe;
  onTransactionAborted: (subscribe: Subscribe) => Unsubscribe;
  onTransactionFinalized: (subscribe: Subscribe) => Unsubscribe;
}
export interface Store<TModel, TDiff> extends Snapshot<TModel>, Actions<TDiff>, Subscriptions {}
export interface Commands<TContext, TResult> {
  execute(command: string, ...rest: any[]): Promise<TResult>;
  register(command: string, callback: (context: TContext, ...rest: any[]) => TResult): Disposable;
}
export interface StoreWithCommands<TModel, TDiff, TContext, TResult> extends Store<TModel, TDiff>, Commands<TContext, TResult> {}
export interface Editor<TDiff, TSelection, TSelectionDiff, TPresence, TContext, TResult>
  extends Commands<TContext, TResult>,
    EditorActions<TDiff, TSelectionDiff, TPresence>,
    EditorSelection<TSelection>,
    EditorPresence<TPresence>,
    EditorSelectionActions<TSelectionDiff> {}

export interface FileStore extends Store<SemioFile, FileDiff> {}
export interface RepresentationStore extends Store<Representation, RepresentationDiff> {}
export interface PortStore extends Store<Port, PortDiff> {}
export interface TypeStore extends Store<Type, TypeDiff> {
  representations: Map<string, RepresentationStore>;
  ports: Map<string, PortStore>;
}
export interface PieceStore extends Store<Piece, PieceDiff> {}
export interface ConnectionStore extends Store<Connection, ConnectionDiff> {}
export interface DesignStore extends Store<Design, DesignDiff> {
  pieces: Map<string, PieceStore>;
  connections: Map<string, ConnectionStore>;
}
export interface KitStore extends StoreWithCommands<Kit, KitDiff, KitCommandContext, KitCommandResult> {
  types: Map<string, TypeStore>;
  designs: Map<string, DesignStore>;
  files: Map<Url, FileStore>;
  fileUrls(): Map<Url, Url>;
}
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
}
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
}
export interface DesignEditorId {
  kit: KitId;
  design: DesignId;
}
export interface DesignEditorSelection {
  pieces?: PieceId[];
  connections?: ConnectionId[];
  port?: { piece: PieceId; designPiece?: PieceId; port: PortId };
}
export interface DesignEditorSelectionPiecesDiff {
  added?: PieceId[];
  removed?: PieceId[];
}
export interface DesignEditorSelectionConnectionsDiff {
  added?: ConnectionId[];
  removed?: ConnectionId[];
}
export interface DesignEditorSelectionPortDiff {
  piece?: PieceId;
  designPiece?: PieceId;
  port?: PortId;
}
export interface DesignEditorSelectionDiff {
  pieces?: DesignEditorSelectionPiecesDiff;
  connections?: DesignEditorSelectionConnectionsDiff;
  port?: DesignEditorSelectionPortDiff;
}
export enum DesignEditorFullscreenPanel {
  None = "none",
  Diagram = "diagram",
  Model = "model",
}
export interface DesignEditorPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface DesignEditorHover {
  piece?: PieceId;
  connection?: ConnectionId;
  port?: PortId;
}
export interface DesignEditorPresenceOther extends OtherPresence, DesignEditorPresence {}
export interface DesignEditorChangableState extends EditorChangableState<DesignEditorSelection, DesignEditorPresence> {
  fullscreenPanel: DesignEditorFullscreenPanel;
}
export interface DesignEditorDiff {
  selection?: DesignEditorSelectionDiff;
  presence?: DesignEditorPresence;
  hover?: DesignEditorHover;
  fullscreenPanel?: DesignEditorFullscreenPanel;
}
export interface DesignEditorStep {
  kitDiff?: KitDiff;
  selectionDiff?: DesignEditorSelectionDiff;
}
export interface DesignEditorEdit extends EditorEdit<DesignEditorStep> {}
export interface DesignEditorState extends EditorState<KitDiff, DesignEditorPresenceOther, DesignEditorSelection, DesignEditorPresence, DesignEditorEdit> {
  hover?: DesignEditorHover;
  fullscreenPanel: DesignEditorFullscreenPanel;
}

export interface DesignEditorSnapshot {
  snapshot(): DesignEditorState;
}
export interface DesignEditorActions extends EditorActions<DesignEditorDiff, DesignEditorSelectionDiff, DesignEditorPresence> {}
export interface DesignEditorSubscriptions extends EditorSubscriptions {}
export interface DesignEditorCommandContext extends KitCommandContext {
  designEditor: DesignEditorState;
  designId: DesignId;
}
export interface DesignEditorCommandResult {
  diff?: DesignEditorDiff;
  kitDiff?: KitDiff;
}
export interface DesignEditorStore extends StoreWithCommands<DesignEditorState, DesignEditorDiff, DesignEditorCommandContext, DesignEditorCommandResult> {}

export interface SketchpadChangableState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditor?: DesignEditorId;
}
export interface SketchpadState extends SketchpadChangableState {
  persistantId?: string;
}

export interface SketchpadSnapshot {
  snapshot(): SketchpadState;
}
export interface SketchpadDiff {
  mode?: Mode;
  theme?: Theme;
  layout?: Layout;
  activeDesignEditor?: DesignEditorId;
}
export interface SketchpadActions extends Actions<SketchpadDiff> {
  createKit: (kit: Kit) => void;
  deleteKit: (id: KitIdLike) => void;
  createDesignEditor: (id: DesignEditorId) => void;
  deleteDesignEditor: (id: DesignEditorId) => void;
}
export interface SketchpadSubscriptions extends Subscriptions {
  onKitCreated: (subscribe: Subscribe) => Unsubscribe;
  onKitDeleted: (subscribe: Subscribe) => Unsubscribe;
  onDesignEditorCreated: (subscribe: Subscribe) => Unsubscribe;
  onDesignEditorDeleted: (subscribe: Subscribe) => Unsubscribe;
}
export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
}
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
}
export interface SketchpadStore extends StoreWithCommands<SketchpadState, SketchpadDiff, SketchpadCommandContext, SketchpadCommandResult> {
  kits: Map<string, KitStore>;
  designEditors: Map<string, Map<string, DesignEditorStore>>;
}

// #endregion Api

export const inverseDesignEditorSelectionDiff = (selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff => {
  // TODO
};

// #region Stores
type YUuid = string;
type YUuidArray = Y.Array<YUuid>;

type YAuthor = Y.Map<string>;
type YAuthors = Y.Map<YAuthor>;
type YAttribute = Y.Map<string>;
type YAttributes = Y.Array<YAttribute>;
type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;
type YVec3 = YLeafMapNumber;
type YPlane = Y.Map<YVec3>;

type YFile = Y.Map<string>;
type YFiles = Y.Map<YFile>;

type YBenchmark = Y.Map<string>;
type YBenchmarks = Y.Map<YBenchmark>;

type YQuality = Y.Map<string>;
type YQualities = Y.Map<YQuality>;

type YProp = Y.Map<string>;
type YProps = Y.Map<YProp>;

type YRepresentationVal = string | YStringArray | YAttributes;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentationArray = Y.Array<YRepresentation>;

type YPortVal = string | number | boolean | YLeafMapNumber | YAttributes | YStringArray;
type YPort = Y.Map<YPortVal>;
type YPortArray = Y.Array<YPort>;

type YTypeVal = string | number | boolean | YAuthors | YAttributes | YRepresentationArray | YPortArray;
type YType = Y.Map<YTypeVal>;
type YTypes = Y.Map<YType>;

type YLayer = Y.Map<string>;
type YLayers = Y.Map<YLayer>;

type YPieceVal = string | YLeafMapString | YLeafMapNumber | YPlane | YAttributes;
type YPiece = Y.Map<YPieceVal>;
type YPieceArray = Y.Array<YPiece>;
type YPieces = Y.Map<YPiece>;

type YGroup = Y.Map<string>;
type YGroups = Y.Map<YGroup>;

type YSide = Y.Map<YLeafMapString>;

type YConnectionVal = string | number | YAttributes | YSide;
type YConnection = Y.Map<YConnectionVal>;
type YConnectionArray = Y.Array<YConnection>;

type YStat = Y.Map<string>;
type YStats = Y.Map<YStat>;

type YDesignVal = string | YAuthors | YAttributes | YPieceArray | YConnectionArray;
type YDesign = Y.Map<YDesignVal>;
type YDesigns = Y.Map<YDesign>;

type YDesignEditorStoreVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignEditorStoreValMap = Y.Map<YDesignEditorStoreVal>;
type YDesignEditorStoreMap = Y.Map<YDesignEditorStore>;

type YIdMap = Y.Map<string>;
type YKitVal = string | YUuidArray | YIdMap | YAttributes;
type YKit = Y.Map<YKitVal>;

type YSketchpadVal = string | boolean;
type YSketchpad = Y.Map<YSketchpadVal>;

type YSketchpadKeysMap = {
  mode: string;
  theme: string;
  layout: string;
  activeDesignEditorDesign: YDesign;
};
const getSketchpadStore = <K extends keyof YSketchpadKeysMap>(m: YSketchpad, k: K): YSketchpadKeysMap[K] => m.get(k as string) as YSketchpadKeysMap[K];

// Helper functions for Yjs type conversion
function createAttribute(attribute: Attribute): YAttribute {
  const yMap = new Y.Map<string>();
  yMap.set("key", attribute.key);
  if (attribute.value !== undefined) yMap.set("value", attribute.value);
  if (attribute.definition !== undefined) yMap.set("definition", attribute.definition);
  return yMap;
}

function createAttributes(attributes: Attribute[] | undefined): YAttributes {
  const yArr = new Y.Array<YAttribute>();
  (attributes || []).forEach((attr) => yArr.push([createAttribute(attr)]));
  return yArr;
}

function getAttributes(yArr: YAttributes | undefined): Attribute[] {
  if (!yArr) return [];
  const attributes: Attribute[] = [];
  yArr.forEach((yMap: YAttribute) => {
    attributes.push({
      key: yMap.get("key") as string,
      value: yMap.get("value") as string | undefined,
      definition: yMap.get("definition") as string | undefined,
    });
  });
  return attributes;
}

function createAuthor(author: Author): YAuthor {
  const yAuthor = new Y.Map<string>();
  yAuthor.set("name", author.name);
  yAuthor.set("email", author.email || "");
  return yAuthor;
}

function createAuthors(authors: Author[] | undefined): YAuthors {
  const yAuthors = new Y.Map<YAuthor>();
  (authors || []).forEach((author) => yAuthors.set(author.name, createAuthor(author)));
  return yAuthors;
}

function getAuthors(yAuthors: YAuthors | undefined): Author[] {
  if (!yAuthors) return [];
  const authors: Author[] = [];
  yAuthors.forEach((yAuthor: YAuthor) => {
    authors.push({
      name: yAuthor.get("name") as string,
      email: (yAuthor.get("email") as string) || "",
    });
  });
  return authors;
}

// Helper for creating Vec3 Y.Map objects
function createVec3(vec: { x: number; y: number; z: number }): Y.Map<number> {
  const yVec = new Y.Map<number>();
  yVec.set("x", vec.x);
  yVec.set("y", vec.y);
  yVec.set("z", vec.z);
  return yVec;
}

function getVec3(yVec: Y.Map<number>): { x: number; y: number; z: number } {
  return {
    x: yVec.get("x") as number,
    y: yVec.get("y") as number,
    z: yVec.get("z") as number,
  };
}

function updateVec3(yVec: Y.Map<number>, diff: { x?: number; y?: number; z?: number }): void {
  if (diff.x !== undefined) yVec.set("x", diff.x);
  if (diff.y !== undefined) yVec.set("y", diff.y);
  if (diff.z !== undefined) yVec.set("z", diff.z);
}

// Helper for creating Vec2 Y.Map objects
function createVec2(vec: { x: number; y: number }): Y.Map<number> {
  const yVec = new Y.Map<number>();
  yVec.set("x", vec.x);
  yVec.set("y", vec.y);
  return yVec;
}

function getVec2(yVec: Y.Map<number>): { x: number; y: number } {
  return {
    x: yVec.get("x") as number,
    y: yVec.get("y") as number,
  };
}

function updateVec2(yVec: Y.Map<number>, diff: { x?: number; y?: number }): void {
  if (diff.x !== undefined) yVec.set("x", diff.x);
  if (diff.y !== undefined) yVec.set("y", diff.y);
}

// Helper for string arrays
function createStringArray(items: string[]): Y.Array<string> {
  const yArr = new Y.Array<string>();
  items.forEach((item) => yArr.push([item]));
  return yArr;
}

function updateStringArray(yArr: Y.Array<string>, items: string[]): void {
  yArr.delete(0, yArr.length);
  items.forEach((item) => yArr.push([item]));
}

// Helper for attributes update
function updateAttributes(yAttributes: YAttributes, attributes: Attribute[]): void {
  yAttributes.delete(0, yAttributes.length);
  attributes.forEach((attr) => yAttributes.push([createAttribute(attr)]));
}

function createObserver(yObject: Y.AbstractType<any>, subscribe: Subscribe, deep?: boolean): Unsubscribe {
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

// Helper for UUID-based ID mapping operations
class IdMapper<T> {
  private readonly idToUuid = new Map<T, string>();

  getId(item: T): string | undefined {
    return this.idToUuid.get(item);
  }

  setId(item: T, uuid?: string): string {
    const id = uuid || uuidv4();
    this.idToUuid.set(item, id);
    return id;
  }

  deleteId(item: T): boolean {
    return this.idToUuid.delete(item);
  }

  has(item: T): boolean {
    return this.idToUuid.has(item);
  }
}

// Helper for conditional property assignment
function setIfDefined<T>(map: Y.Map<any>, key: string, value: T | undefined): void {
  if (value !== undefined) {
    map.set(key, value);
  }
}

// Helper for safe property access with defaults
function getWithDefault<T>(map: Y.Map<any>, key: string, defaultValue: T): T {
  const value = map.get(key);
  return value !== undefined ? value : defaultValue;
}

// Helper for plane objects
function createPlane(plane: { origin: { x: number; y: number; z: number }; xAxis: { x: number; y: number; z: number }; yAxis: { x: number; y: number; z: number } }): Y.Map<any> {
  const yPlane = new Y.Map<any>();
  yPlane.set("origin", createVec3(plane.origin));
  yPlane.set("xAxis", createVec3(plane.xAxis));
  yPlane.set("yAxis", createVec3(plane.yAxis));
  return yPlane;
}

function getPlane(yPlane: Y.Map<any>): { origin: { x: number; y: number; z: number }; xAxis: { x: number; y: number; z: number }; yAxis: { x: number; y: number; z: number } } {
  return {
    origin: getVec3(yPlane.get("origin") as Y.Map<number>),
    xAxis: getVec3(yPlane.get("xAxis") as Y.Map<number>),
    yAxis: getVec3(yPlane.get("yAxis") as Y.Map<number>),
  };
}

// class YFileStore implements FileStore {
//   public readonly parent: YKitStore;
//   public readonly yFile: Y.Map<string>;
//   private cache?: SemioFile;
//   private cacheHash?: string;

//   private hash(file: SemioFile): string {
//     return JSON.stringify(file);
//   }

//   constructor(parent: YKitStore, file: SemioFile) {
//     this.parent = parent;
//     this.yFile = parent.getMap;
//     this.yFile.set("path", file.path);
//     this.yFile.set("remote", file.remote || "");
//     this.yFile.set("size", file.size?.toString() || "");
//     this.yFile.set("hash", file.hash || "");
//     this.yFile.set("createdAt", file.createdAt?.toISOString() || "");
//     this.yFile.set("updatedAt", file.updatedAt?.toISOString() || "");
//   }

//   get file(): SemioFile {
//     const path = this.yFile.get("path") as string;
//     const remote = this.yFile.get("remote") as string;
//     const file: SemioFile = { path, remote: remote || undefined };
//     const size = this.yFile.get("size");
//     if (size) file.size = parseInt(size as string);
//     const hash = this.yFile.get("hash");
//     if (hash) file.hash = hash as string;
//     const createdAt = this.yFile.get("createdAt");
//     if (createdAt) file.createdAt = new Date(createdAt as string);
//     const updatedAt = this.yFile.get("updatedAt");
//     if (updatedAt) file.updatedAt = new Date(updatedAt as string);
//     return file;
//   }

//   change = (diff: FileDiff) => {
//     if (diff.path !== undefined) this.yFile.set("path", diff.path);
//     if (diff.remote !== undefined) this.yFile.set("remote", diff.remote);
//     if (diff.size !== undefined) this.yFile.set("size", diff.size.toString());
//     if (diff.hash !== undefined) this.yFile.set("hash", diff.hash);
//     this.cache = undefined; // Invalidate cache when data changes
//     this.cacheHash = undefined; // Invalidate hash when data changes
//   };

//   snapshot = (): SemioFile => {
//     const currentData = {
//       path: this.yFile.get("path") as string,
//       remote: (this.yFile.get("remote") as string) || undefined,
//       size: this.yFile.get("size") ? parseInt(this.yFile.get("size") as string) : undefined,
//       hash: (this.yFile.get("hash") as string) || undefined,
//       createdAt: this.yFile.get("createdAt") ? new Date(this.yFile.get("createdAt") as string) : undefined,
//       updatedAt: this.yFile.get("updatedAt") ? new Date(this.yFile.get("updatedAt") as string) : undefined,
//     };
//     const currentHash = this.hash(currentData);

//     if (!this.cache || this.cacheHash !== currentHash) {
//       this.cache = currentData;
//       this.cacheHash = currentHash;
//     }

//     return this.cache;
//   };

//   onChanged = (subscribe: Subscribe) => {
//     return createObserver(this.yFile, subscribe, false);
//   };

//   onChangedDeep = (subscribe: Subscribe) => {
//     return createObserver(this.yFile, subscribe, true);
//   };
// }

// class YRepresentationStore implements RepresentationStore {
//   public readonly parent: YKitStore;
//   public readonly yRepresentation: YRepresentation;
//   private cache?: Representation;
//   private cacheHash?: string;

//   private hash(representation: Representation): string {
//     return JSON.stringify(representation);
//   }

//   constructor(parent: YKitStore, representation: Representation) {
//     this.parent = parent;
//     this.yRepresentation = new Y.Map<any>();
//     this.yRepresentation.set("url", representation.url);
//     this.yRepresentation.set("description", representation.description || "");
//     this.yRepresentation.set("tags", createStringArray(representation.tags || []));
//     this.yRepresentation.set("attributes", createAttributes(representation.attributes));
//   }

//   snapshot = (): Representation => {
//     const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
//     const yAttributes = this.yRepresentation.get("attributes") as YAttributes;

//     const attributes = getAttributes(yAttributes);
//     const url = this.yRepresentation.get("url") as string;
//     const description = (this.yRepresentation.get("description") as string) || "";
//     const tags = yTags ? yTags.toArray() : [];

//     const currentData = {
//       url: url,
//       description: description,
//       tags: tags,
//       attributes: attributes,
//     };
//     const currentHash = this.hash(currentData);

//     if (!this.cache || this.cacheHash !== currentHash) {
//       this.cache = currentData;
//       this.cacheHash = currentHash;
//     }

//     return this.cache;
//   };

//   change = (diff: RepresentationDiff) => {
//     if (diff.url !== undefined) this.yRepresentation.set("url", diff.url);
//     if (diff.description !== undefined) this.yRepresentation.set("description", diff.description);
//     if (diff.tags !== undefined) {
//       updateStringArray(this.yRepresentation.get("tags") as Y.Array<string>, diff.tags);
//     }
//     if (diff.attributes !== undefined) {
//       updateAttributes(this.yRepresentation.get("attributes") as YAttributes, diff.attributes);
//     }
//   };

//   onChanged = (subscribe: Subscribe) => {
//     return createObserver(this.yRepresentation, subscribe, false);
//   };

//   onChangedDeep = (subscribe: Subscribe) => {
//     return createObserver(this.yRepresentation, subscribe, true);
//   };
// }

// class YPortStore implements PortStore {
//   public readonly parent: YTypeStore;
//   public readonly yPort: YPort;
//   private cache?: Port;
//   private cacheHash?: string;

//   private hash(port: Port): string {
//     return JSON.stringify(port);
//   }

//   constructor(parent: YTypeStore, port: Port) {
//     this.parent = parent;
//     this.yPort = new Y.Map<any>();
//     this.yPort.set("id_", port.id_ || "");
//     this.yPort.set("description", port.description || "");
//     this.yPort.set("mandatory", port.mandatory || false);
//     this.yPort.set("family", port.family || "");
//     this.yPort.set("t", port.t);
//     this.yPort.set("compatibleFamilies", createStringArray(port.compatibleFamilies || []));
//     this.yPort.set("point", createVec3(port.point));
//     this.yPort.set("direction", createVec3(port.direction));
//     this.yPort.set("attributes", createAttributes(port.attributes));
//   }

//   snapshot = (): Port => {
//     const yCompatibleFamilies = this.yPort.get("compatibleFamilies") as Y.Array<string>;
//     const yPoint = this.yPort.get("point") as Y.Map<number>;
//     const yDirection = this.yPort.get("direction") as Y.Map<number>;
//     const yAttributes = this.yPort.get("attributes") as YAttributes;

//     const attributes = getAttributes(yAttributes);

//     const currentData = {
//       id_: this.yPort.get("id_") as string,
//       description: (this.yPort.get("description") as string) || "",
//       mandatory: this.yPort.get("mandatory") as boolean,
//       family: (this.yPort.get("family") as string) || "",
//       compatibleFamilies: yCompatibleFamilies ? yCompatibleFamilies.toArray() : [],
//       point: {
//         x: yPoint.get("x") as number,
//         y: yPoint.get("y") as number,
//         z: yPoint.get("z") as number,
//       },
//       direction: {
//         x: yDirection.get("x") as number,
//         y: yDirection.get("y") as number,
//         z: yDirection.get("z") as number,
//       },
//       t: this.yPort.get("t") as number,
//       attributes: attributes,
//     };
//     const currentHash = this.hash(currentData);

//     if (!this.cache || this.cacheHash !== currentHash) {
//       this.cache = currentData;
//       this.cacheHash = currentHash;
//     }

//     return this.cache;
//   };

//   change = (diff: PortDiff) => {
//     if (diff.id_ !== undefined) this.yPort.set("id_", diff.id_);
//     if (diff.description !== undefined) this.yPort.set("description", diff.description);
//     if (diff.mandatory !== undefined) this.yPort.set("mandatory", diff.mandatory);
//     if (diff.family !== undefined) this.yPort.set("family", diff.family);
//     if (diff.t !== undefined) this.yPort.set("t", diff.t);
//     if (diff.compatibleFamilies !== undefined) {
//       updateStringArray(this.yPort.get("compatibleFamilies") as Y.Array<string>, diff.compatibleFamilies);
//     }
//     if (diff.point !== undefined) {
//       updateVec3(this.yPort.get("point") as Y.Map<number>, diff.point);
//     }
//     if (diff.direction !== undefined) {
//       updateVec3(this.yPort.get("direction") as Y.Map<number>, diff.direction);
//     }
//     if (diff.attributes !== undefined) {
//       updateAttributes(this.yPort.get("attributes") as YAttributes, diff.attributes);
//     }
//   };

//   onChanged = (subscribe: Subscribe) => {
//     return createObserver(this.yPort, subscribe, false);
//   };

//   onChangedDeep = (subscribe: Subscribe) => {
//     return createObserver(this.yPort, subscribe, true);
//   };
// }

class YTypeStore implements TypeStore {
  // public readonly representations: Map<string, YRepresentationStore> = new Map();
  // public readonly ports: Map<string, YPortStore> = new Map();
  private yType: YType;
  private transact: Transact;
  private cache?: Type;
  private cacheHash?: string;

  private hash(type: Type): string {
    return JSON.stringify(type);
  }

  constructor(yType: YType, transact: Transact, type: Type) {
    this.yType = yType;
    this.transact = transact;
    this.yType.set("name", type.name);
    this.yType.set("variant", type.variant || "");

    this.yType.set("createdAt", new Date().toISOString());
    this.yType.set("updatedAt", new Date().toISOString());
  }

  change = (diff: TypeDiff) => {
    this.transact(() => {
      if (diff.name !== undefined) this.yType.set("name", diff.name);
      if (diff.variant !== undefined) this.yType.set("variant", diff.variant);
    });
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  snapshot = (): Type => {
    const name = this.yType.get("name") as string;
    const variant = this.yType.get("variant") as string | undefined;
    const currentData = {
      name: name,
      variant: variant,
      createdAt: this.yType.get("createdAt") as string,
      updatedAt: this.yType.get("updatedAt") as string,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, true);
  };
}

class YPieceStore implements PieceStore {
  private cache?: Piece;
  private cacheHash?: string;

  private hash(piece: Piece): string {
    return JSON.stringify(piece);
  }

  constructor(parent: YDesignStore, piece: Piece) {}

  snapshot = (): Piece => {};

  change = (diff: PieceDiff) => {};

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPiece, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPiece, subscribe, true);
  };
}

// class YConnectionStore implements ConnectionStore {
//   public readonly parent: YDesignStore;
//   public readonly yConnection: YConnection;
//   private cache?: Connection;
//   private cacheHash?: string;

//   private hash(connection: Connection): string {
//     return JSON.stringify(connection);
//   }

//   constructor(parent: YDesignStore, connection: Connection) {
//     this.parent = parent;
//     this.yConnection = new Y.Map<any>();
//     const yConnected = new Y.Map<any>();
//     const yConnectedPiece = new Y.Map<string>();
//     yConnectedPiece.set("id_", connection.connected.piece.id_);
//     yConnected.set("piece", yConnectedPiece);
//     const yConnectedPort = new Y.Map<string>();
//     yConnectedPort.set("id_", connection.connected.port.id_ || "");
//     yConnected.set("port", yConnectedPort);
//     if (connection.connected.designPiece) {
//       const yConnectedDesignPiece = new Y.Map<string>();
//       yConnectedDesignPiece.set("id_", connection.connected.designPiece.id_);
//       yConnected.set("designPiece", yConnectedDesignPiece);
//     }
//     this.yConnection.set("connected", yConnected);
//     const yConnecting = new Y.Map<any>();
//     const yConnectingPiece = new Y.Map<string>();
//     yConnectingPiece.set("id_", connection.connecting.piece.id_);
//     yConnecting.set("piece", yConnectingPiece);
//     const yConnectingPort = new Y.Map<string>();
//     yConnectingPort.set("id_", connection.connecting.port.id_ || "");
//     yConnecting.set("port", yConnectingPort);
//     if (connection.connecting.designPiece) {
//       const yConnectingDesignPiece = new Y.Map<string>();
//       yConnectingDesignPiece.set("id_", connection.connecting.designPiece.id_);
//       yConnecting.set("designPiece", yConnectingDesignPiece);
//     }
//     this.yConnection.set("connecting", yConnecting);
//     this.yConnection.set("description", connection.description || "");
//     this.yConnection.set("gap", connection.gap || 0);
//     this.yConnection.set("shift", connection.shift || 0);
//     this.yConnection.set("rise", connection.rise || 0);
//     this.yConnection.set("rotation", connection.rotation || 0);
//     this.yConnection.set("turn", connection.turn || 0);
//     this.yConnection.set("tilt", connection.tilt || 0);
//     this.yConnection.set("x", connection.x || 0);
//     this.yConnection.set("y", connection.y || 0);
//     this.yConnection.set("attributes", createAttributes(connection.attributes));
//   }

//   snapshot = (): Connection => {
//     const yConnected = this.yConnection.get("connected") as Y.Map<any>;
//     const yConnecting = this.yConnection.get("connecting") as Y.Map<any>;
//     const yConnectedPiece = yConnected.get("piece") as Y.Map<string>;
//     const yConnectedDesignPiece = yConnected.get("designPiece") as Y.Map<string> | undefined;
//     const yConnectedPort = yConnected.get("port") as Y.Map<string>;
//     const yConnectingPiece = yConnecting.get("piece") as Y.Map<string>;
//     const yConnectingDesignPiece = yConnecting.get("designPiece") as Y.Map<string> | undefined;
//     const yConnectingPort = yConnecting.get("port") as Y.Map<string>;

//     const yAttributes = this.yConnection.get("attributes") as YAttributes;
//     const attributes: Attribute[] = [];
//     if (yAttributes) {
//       yAttributes.forEach((yMap: YAttribute) => {
//         attributes.push({
//           key: yMap.get("key") as string,
//           value: yMap.get("value") as string | undefined,
//           definition: yMap.get("definition") as string | undefined,
//         });
//       });
//     }

//     const currentData = {
//       connected: {
//         piece: { id_: yConnectedPiece.get("id_") as string },
//         port: { id_: yConnectedPort.get("id_") as string },
//         designPiece: yConnectedDesignPiece ? { id_: yConnectedDesignPiece.get("id_") as string } : undefined,
//       },
//       connecting: {
//         piece: { id_: yConnectingPiece.get("id_") as string },
//         port: { id_: yConnectingPort.get("id_") as string },
//         designPiece: yConnectingDesignPiece ? { id_: yConnectingDesignPiece.get("id_") as string } : undefined,
//       },
//       description: (this.yConnection.get("description") as string) || "",
//       gap: this.yConnection.get("gap") as number,
//       shift: this.yConnection.get("shift") as number,
//       rise: this.yConnection.get("rise") as number,
//       rotation: this.yConnection.get("rotation") as number,
//       turn: this.yConnection.get("turn") as number,
//       tilt: this.yConnection.get("tilt") as number,
//       x: this.yConnection.get("x") as number,
//       y: this.yConnection.get("y") as number,
//       attributes: attributes,
//     };
//     const currentHash = this.hash(currentData);

//     if (!this.cache || this.cacheHash !== currentHash) {
//       this.cache = currentData;
//       this.cacheHash = currentHash;
//     }

//     return this.cache;
//   };

//   change = (diff: ConnectionDiff) => {
//     if (diff.description !== undefined) this.yConnection.set("description", diff.description);
//     if (diff.gap !== undefined) this.yConnection.set("gap", diff.gap);
//     if (diff.shift !== undefined) this.yConnection.set("shift", diff.shift);
//     if (diff.rise !== undefined) this.yConnection.set("rise", diff.rise);
//     if (diff.rotation !== undefined) this.yConnection.set("rotation", diff.rotation);
//     if (diff.turn !== undefined) this.yConnection.set("turn", diff.turn);
//     if (diff.tilt !== undefined) this.yConnection.set("tilt", diff.tilt);
//     if (diff.x !== undefined) this.yConnection.set("x", diff.x);
//     if (diff.y !== undefined) this.yConnection.set("y", diff.y);
//   };

//   onChanged = (subscribe: Subscribe) => {
//     return createObserver(this.yConnection, subscribe, false);
//   };

//   onChangedDeep = (subscribe: Subscribe) => {
//     return createObserver(this.yConnection, subscribe, true);
//   };
// }

class YDesignStore implements DesignStore {
  public readonly pieces: Map<string, YPieceStore> = new Map();
  private yDesign: YDesign;
  private transact: Transact;
  private yPieces: YPieces;
  private yDesignPieces: YUuidArray;
  private cache?: Design;
  private cacheHash?: string;

  private hash(design: Design): string {
    return JSON.stringify(design);
  }

  constructor(yDesign: YDesign, transact: Transact, design: Design) {
    this.yDesign = yDesign;
    this.transact = transact;
    this.yDesign = new Y.Map<any>();
    this.yDesign.set("name", design.name);
    this.yDesign.set("variant", design.variant || "");
    this.yDesign.set("view", design.view || "");

    this.yPieces = new Y.Map<YPiece>();
    this.yDesign.set("pieces", this.yPieces);

    if (design.pieces) {
      const uuids = new Array<string>();
      for (const piece of design.pieces) {
        const uuid = uuidv4();
        const yPiece = new Y.Map<YPiece>();
        this.yPieces.set(uuid, yPiece);
        const yPieceStore = new YPieceStore(yPiece, transact, piece);
        this.pieces.set(pieceIdToString(piece), yPieceStore);
        uuids.push(uuid);
      }
      this.yDesignPieces.insert(this.yDesignPieces.length, uuids);
    }

    this.yDesign.set("createdAt", new Date().toISOString());
    this.yDesign.set("updatedAt", new Date().toISOString());
  }

  snapshot = (): Design => {
    const name = this.yDesign.get("name") as string;
    const variant = this.yDesign.get("variant") as string | undefined;
    const view = this.yDesign.get("view") as string | undefined;

    const currentData = {
      name: name,
      variant: variant,
      view: view,

      createdAt: this.yDesign.get("createdAt") as string,
      updatedAt: this.yDesign.get("updatedAt") as string,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: DesignDiff) => {
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yDesign.observe(observer);
    return () => {
      this.yDesign.unobserve(observer);
    };
  };

  onChangedDeep = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yDesign.observeDeep(observer);
    return () => {
      this.yDesign.unobserveDeep(observer);
    };
  };
}

class YKitStore implements KitStore {
  public readonly types: Map<string, YTypeStore> = new Map();
  public readonly designs: Map<string, YDesignStore> = new Map();
  private readonly yDoc: Y.Doc;
  private readonly yKit: YKit;
  private readonly yAuthors: YAuthors;
  private readonly yKitAuthors: YUuidArray;
  private readonly yFiles: YFiles;
  private readonly yKitFiles: YUuidArray;
  private readonly yQualities: YQualities;
  private readonly yKitQualities: YUuidArray;
  private readonly yTypes: YTypes;
  private readonly yKitTypes: YUuidArray;
  private readonly yDesigns: YDesigns;
  private readonly yKitDesigns: YUuidArray;
  private readonly indexeddbProviders: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult> = new Map();
  private readonly regularFiles: Map<Url, string> = new Map();
  private cache?: Kit;
  private cacheHash?: string;

  private hash(kit: Kit): string {
    return JSON.stringify(kit);
  }

  constructor(kit: Kit) {
    this.yDoc = new Y.Doc();
    this.yKit = this.yDoc.getMap() as YKit;
    this.yAuthors = this.yDoc.getMap("authors");
    this.yKitAuthors = this.yDoc.getArray("authors");
    this.yFiles = this.yDoc.getMap("files");
    this.yKitFiles = this.yDoc.getArray("files");
    this.yQualities = this.yDoc.getMap("qualities");
    this.yKitQualities = this.yDoc.getArray("qualities");
    this.yTypes = this.yDoc.getMap("types");
    this.yKitTypes = this.yDoc.getArray("types");
    this.yDesigns = this.yDoc.getMap("designs");
    this.yKitDesigns = this.yDoc.getArray("designs");

    this.yDoc.transact(() => {
      this.yKit.set("name", kit.name);
      this.yKit.set("version", kit.version || "");

      if (kit.types) {
        const uuids = new Array<string>();
        for (const type of kit.types) {
          const uuid = uuidv4();
          const yType = new Y.Map<YTypeVal>();
          this.yTypes.set(uuid, yType);
          const yTypeStore = new YTypeStore(yType, this.yDoc.transact, type);
          this.types.set(typeIdToString(type), yTypeStore);
          uuids.push(uuid);
        }
        this.yKitTypes.insert(this.yKitTypes.length, uuids);
      }

      if (kit.designs) {
        const uuids = new Array<string>();
        for (const design of kit.designs) {
          const uuid = uuidv4();
          const yDesign = new Y.Map<YDesignVal>();
          this.yDesigns.set(uuid, yDesign);
          const yDesignStore = new YDesignStore(yDesign, this.yDoc.transact, design);
          this.designs.set(designIdToString(design), yDesignStore);
          uuids.push(uuid);
        }
        this.yKitDesigns.insert(this.yKitDesigns.length, uuids);
      }

      this.yKit.set("createdAt", new Date().toISOString());
      this.yKit.set("updatedAt", new Date().toISOString());
    });

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  snapshot = (): Kit => {
    const yAttributes = this.yKit.get("attributes") as YAttributes;
    const attributes = getAttributes(yAttributes);

    const currentData = {
      name: this.yKit.get("name") as string,
      version: this.yKit.get("version") as string | undefined,
      // description: this.yKit.get("description") as string | undefined,
      // icon: this.yKit.get("icon") as string | undefined,
      // image: this.yKit.get("image") as string | undefined,
      // preview: this.yKit.get("preview") as string | undefined,
      // remote: this.yKit.get("remote") as string | undefined,
      // homepage: this.yKit.get("homepage") as string | undefined,
      // license: this.yKit.get("license") as string | undefined,
      // createdAt: this.yKit.get("createdAt") ? new Date(this.yKit.get("createdAt") as string) : undefined,
      // updatedAt: this.yKit.get("updatedAt") ? new Date(this.yKit.get("updatedAt") as string) : undefined,
      attributes: attributes,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  fileUrls = (): Map<Url, Url> => {
    return this.regularFiles;
  };

  change = (diff: KitDiff) => {
    this.yKit.set("updatedAt", new Date().toISOString());
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    (this.yKit as unknown as Y.Map<any>).observe(observer);
    return () => {
      (this.yKit as unknown as Y.Map<any>).unobserve(observer);
    };
  };

  onChangedDeep = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    (this.yKit as unknown as Y.Map<any>).observeDeep(observer);
    return () => {
      (this.yKit as unknown as Y.Map<any>).unobserveDeep(observer);
    };
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit store`);
    const context: KitCommandContext = {
      kit: this.snapshot(),
      fileUrls: this.fileUrls(),
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
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

class YDesignEditorStore implements DesignEditorStore {
  public readonly yMap: YDesignEditorStoreValMap;
  private readonly commandRegistry: Map<string, (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult> = new Map();
  private readonly transact: (fn: () => void) => void;
  private cache?: DesignEditorState;
  private cacheHash?: string;

  private hash(state: DesignEditorState): string {
    return JSON.stringify(state);
  }

  constructor(yMap: YDesignEditorStoreValMap, transact: (fn: () => void) => void, state?: DesignEditorState) {
    this.yMap = yMap;
    this.transact = transact;
    this.transact(() => {
      yMap.set("fullscreenPanel", state?.fullscreenPanel || DesignEditorFullscreenPanel.None);
      yMap.set("selectedPieceIds", new Y.Array<string>());
      yMap.set("selectedConnections", new Y.Array<string>());
      yMap.set("selectedPiecePortPieceId", "");
      yMap.set("selectedPiecePortPortId", "");
      yMap.set("selectedPiecePortDesignPieceId", "");
      yMap.set("isTransactionActive", false);
      yMap.set("presence", new Y.Map<any>());
      yMap.set("others", new Y.Array<any>());
      yMap.set("diff", new Y.Map<any>());
      yMap.set("currentTransactionStack", new Y.Array<any>());
      yMap.set("pastTransactionsStack", new Y.Array<any>());
    });

    Object.entries(designEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenPanel(): DesignEditorFullscreenPanel {
    return (this.yMap.get("fullscreenPanel") as DesignEditorFullscreenPanel) || DesignEditorFullscreenPanel.None;
  }
  get selection(): DesignEditorSelection {
    const selectedPieceIds = this.yMap.get("selectedPieceIds") as Y.Array<string>;
    const selectedConnections = this.yMap.get("selectedConnections") as Y.Array<string>;
    const selectedPiecePortPieceId = this.yMap.get("selectedPiecePortPieceId") as string;
    const selectedPiecePortPortId = this.yMap.get("selectedPiecePortPortId") as string;
    const selectedPiecePortDesignPieceId = this.yMap.get("selectedPiecePortDesignPieceId") as string;

    const port =
      selectedPiecePortPieceId && selectedPiecePortPortId
        ? {
            piece: { id_: selectedPiecePortPieceId },
            port: { id_: selectedPiecePortPortId },
            ...(selectedPiecePortDesignPieceId && { designPiece: { id_: selectedPiecePortDesignPieceId } }),
          }
        : undefined;

    return {
      pieces: selectedPieceIds && selectedPieceIds.toArray ? selectedPieceIds.toArray().map((id) => ({ id_: id })) : [],
      connections:
        selectedConnections && selectedConnections.toArray
          ? selectedConnections.toArray().map((id) => ({
              connected: { piece: { id_: id.split("->")[0] || "" } },
              connecting: { piece: { id_: id.split("->")[1] || "" } },
            }))
          : [],
      port,
    };
  }
  get isTransactionActive(): boolean {
    return (this.yMap.get("isTransactionActive") as boolean) || false;
  }
  get presence(): DesignEditorPresence {
    return {
      cursor: {
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }
  get others(): DesignEditorPresenceOther[] {
    return [];
  }
  get diff(): KitDiff {
    return {};
  }
  get currentTransactionStack(): DesignEditorEdit[] {
    const yStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }
  get pastTransactionsStack(): DesignEditorEdit[] {
    const yStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  get canUndo(): boolean {
    return this.pastTransactionsStack.length > 0;
  }

  get canRedo(): boolean {
    return this.currentTransactionStack.length > 0 && !this.isTransactionActive;
  }

  snapshot = (): DesignEditorState => {
    const currentData = {
      fullscreenPanel: this.fullscreenPanel,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      canUndo: this.canUndo,
      canRedo: this.canRedo,
      presence: this.presence,
      others: this.others,
      diff: this.diff,
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: DesignEditorDiff) => {
    if (diff.fullscreenPanel) this.yMap.set("fullscreenPanel", diff.fullscreenPanel);
    if (diff.selection) {
      if (diff.selection.pieces) {
        const selectedPiecesIds = this.yMap.get("selectedPieceIds") as Y.Array<string>;
        if (diff.selection.pieces.removed) {
          diff.selection.pieces.removed.forEach((piece) => {
            const index = selectedPiecesIds.toArray().indexOf(piece.id_);
            if (index >= 0) {
              selectedPiecesIds.delete(index, 1);
            }
          });
        }
        if (diff.selection.pieces.added) {
          selectedPiecesIds.insert(
            selectedPiecesIds.length,
            diff.selection.pieces.added.map((piece) => pieceIdToString(piece)),
          );
        }
      }

      if (diff.selection.connections) {
        const selectedConnections = this.yMap.get("selectedConnections") as Y.Array<string>;
        if (diff.selection.connections.removed) {
          diff.selection.connections.removed.forEach((connection) => {
            const index = selectedConnections.toArray().indexOf(connectionIdToString(connection));
            if (index >= 0) {
              selectedConnections.delete(index, 1);
            }
          });
        }
        if (diff.selection.connections.added) {
          selectedConnections.insert(
            selectedConnections.length,
            diff.selection.connections.added.map((connection) => connectionIdToString(connection)),
          );
        }
      }

      if (diff.selection.port) {
        this.yMap.set("selectedPiecePortPieceId", pieceIdToString(diff.selection.port.piece!));
        this.yMap.set("selectedPiecePortPortId", portIdToString(diff.selection.port.port!));
        this.yMap.set("selectedPiecePortDesignPieceId", pieceIdToString(diff.selection.port.designPiece!));
      }
    }
  };

  onChanged = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    (this.yMap as unknown as Y.Map<any>).observe(observer);
    return () => {
      (this.yMap as unknown as Y.Map<any>).unobserve(observer);
    };
  };

  onChangedDeep = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    (this.yMap as unknown as Y.Map<any>).observeDeep(observer);
    return () => {
      (this.yMap as unknown as Y.Map<any>).unobserveDeep(observer);
    };
  };

  startTransaction = () => {
    this.yMap.set("isTransactionActive", true);
  };

  onTransactionStarted = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yMap.observe(observer);
    return () => {
      this.yMap.unobserve(observer);
    };
  };

  abortTransaction = () => {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack) {
        currentStack.delete(0, currentStack.length);
      }
      this.yMap.set("isTransactionActive", false);
    }
  };

  onTransactionAborted = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yMap.observe(observer);
    return () => {
      this.yMap.unobserve(observer);
    };
  };

  finalizeTransaction = () => {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      if (currentStack && pastStack && currentStack.length > 0) {
        pastStack.push(currentStack.toArray());
        currentStack.delete(0, currentStack.length);
      }
      this.yMap.set("isTransactionActive", false);
      this.yMap.set("isTransactionActive", false);
    }
  };

  onTransactionFinalized = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yMap.observe(observer);
    return () => {
      this.yMap.unobserve(observer);
    };
  };

  undo = () => {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack && currentStack.length > 0) {
        const edit = currentStack.get(currentStack.length - 1);
        currentStack.delete(currentStack.length - 1, 1);
        if (edit && edit.undo) {
          edit.undo.diff && this.change(edit.undo.diff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(pastStack.length - 1);
        pastStack.delete(pastStack.length - 1, 1);
        if (edit && edit.undo) {
          edit.undo.diff && this.change(edit.undo.diff);
        }
      }
    }
  };
  onUndone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yMap.observe(observer);
    return () => {
      this.yMap.unobserve(observer);
    };
  };
  redo = () => {
    if (this.isTransactionActive) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack && currentStack.length > 0) {
        const edit = currentStack.get(0);
        currentStack.delete(0, 1);
        if (edit && edit.do) {
          edit.do.diff && this.change(edit.do.diff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(0);
        pastStack.delete(0, 1);
        if (edit && edit.do) {
          edit.do.diff && this.change(edit.do.diff);
        }
      }
    }
  };
  onRedone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yMap.observe(observer);
    return () => {
      this.yMap.unobserve(observer);
    };
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.designEditor.startTransaction") {
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.abortTransaction") {
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.designEditor.redo") {
      this.redo();
      return {} as T;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design editor store`);
    const parent = this.parent as YSketchpadStore;
    const kitStore = parent.kits.get(kitIdToString(parent.activeDesignEditor!.kit))!;

    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const context: DesignEditorCommandContext = {
      designEditor: state,
      kit: kitState,
      designId: parent.activeDesignEditor!.design,
      fileUrls: kitStore.fileUrls(),
    };
    const result = callback(context, ...rest);

    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }

    if (this.isTransactionActive && (result.diff || result.kitDiff)) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      const inversedSelectionDiff = inverseDesignEditorSelectionDiff(state.selection, result.diff?.selection!);
      const inversedKitDiff = inverseKitDiff(kitState, result.kitDiff!);
      const edit: DesignEditorEdit = {
        do: {
          kitDiff: result.kitDiff,
          selectionDiff: result.diff?.selection,
        },
        undo: {
          kitDiff: inversedKitDiff,
          selectionDiff: inversedSelectionDiff,
        },
      };
      currentStack.push([edit]);
    }

    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }

  registerCommand(command: string, callback: (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  register(command: string, callback: (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult): Disposable {
    return this.registerCommand(command, callback);
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }
}

class YSketchpadStore implements SketchpadStore {
  public readonly sketchpadIndexeddbProvider?: IndexeddbPersistence;
  public readonly kits: Map<string, YKitStore> = new Map();
  public readonly designEditors: Map<string, Map<string, DesignEditorStore>> = new Map();
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult> = new Map();
  private cache?: SketchpadState;
  private cacheHash?: string;
  // private readonly clientId: string;
  // private readonly broadcastChannel: BroadcastChannel;

  private hash(state: SketchpadState): string {
    return JSON.stringify(state);
  }

  constructor(state?: SketchpadState) {
    // this.clientId = uuidv4();
    // this.broadcastChannel = new BroadcastChannel("semio-yjs");
    this.yDoc = new Y.Doc();
    if (state?.persistantId && state.persistantId !== "") {
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad-${state.persistantId}`, this.yDoc);
    }
    this.ySketchpad = this.yDoc.getMap("sketchpad");
    this.yDoc.transact(() => {
      this.ySketchpad.set("mode", state?.mode || Mode.GUEST);
      this.ySketchpad.set("theme", state?.theme || Theme.SYSTEM);
      this.ySketchpad.set("layout", state?.layout || Layout.NORMAL);
      this.ySketchpad.set("designEditorStores", new Y.Map<YDesignEditorStore>());
      this.ySketchpad.set("activeDesignEditor", state?.activeDesignEditor ? JSON.stringify(state.activeDesignEditor) : "");
    });
    // this.yDoc.on("update", (update: Uint8Array) => {
    //   this.broadcastChannel.postMessage({ client: this.clientId, update });
    // });
    // this.broadcastChannel.addEventListener("message", (msg) => {
    //   const { data } = msg;
    //   if (data.client !== this.clientId) {
    //     Y.applyUpdate(this.yDoc, data.update);
    //   }
    // });
  }

  get mode(): Mode {
    return this.ySketchpad.get("mode") as Mode;
  }
  get theme(): Theme {
    return this.ySketchpad.get("theme") as Theme;
  }
  get layout(): Layout {
    return this.ySketchpad.get("layout") as Layout;
  }
  get activeDesignEditor(): DesignEditorId | undefined {
    const designEditorIdStr = this.ySketchpad.get("activeDesignEditor");
    if (!designEditorIdStr || typeof designEditorIdStr !== "string") return undefined;
    return JSON.parse(designEditorIdStr) as DesignEditorId;
  }

  snapshot = (): SketchpadState => {
    const currentValues = {
      mode: this.mode,
      theme: this.theme,
      layout: this.layout,
      activeDesignEditor: this.activeDesignEditor,
    };
    const currentHash = this.hash(currentValues);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentValues;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  createKit = (kit: Kit) => {
    const store = new YKitStore(this, kit);
    const kitIdStr = kitIdToString(kit);
    this.kits.set(kitIdStr, store);
  };

  createDesignEditor = (id: DesignEditorId) => {
    const store = new YDesignEditorStore(this);
    const kitIdStr = kitIdToString(id.kit);
    const designIdStr = designIdToString(id.design);
    if (!this.designEditors.has(kitIdStr)) {
      this.designEditors.set(kitIdStr, new Map());
    }
    const kitEditors = this.designEditors.get(kitIdStr)!;
    kitEditors.set(designIdStr, store);
  };

  change(diff: SketchpadDiff) {
    this.yDoc.transact(() => {
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.layout) this.ySketchpad.set("layout", diff.layout);
      if (diff.activeDesignEditor) this.ySketchpad.set("activeDesignEditor", JSON.stringify(diff.activeDesignEditor));
    });
  }

  deleteKit = (id: KitIdLike) => {
    const kitId = kitIdLikeToKitId(id);
    const kitIdStr = kitIdToString(kitId);
    if (this.kits.has(kitIdStr)) {
      this.kits.delete(kitIdStr);
    }
    if (this.designEditors.has(kitIdStr)) {
      this.designEditors.delete(kitIdStr);
    }
  };

  deleteDesignEditor = (id: DesignEditorId) => {
    const kitIdStr = kitIdToString(id.kit);
    const designIdStr = designIdToString(id.design);
    const kitEditors = this.designEditors.get(kitIdStr);
    if (kitEditors) {
      kitEditors.delete(designIdStr);
      if (kitEditors.size === 0) {
        this.designEditors.delete(kitIdStr);
      }
    }
  };

  // Subscriptions using Yjs observers
  onKitCreated = (subscribe: Subscribe): Unsubscribe => {
    // For kit creation, we can observe all kit documents being added
    // This is a simplified approach - would need more sophisticated tracking in full implementation
    const observer = () => subscribe();
    // Note: Would need to observe kit document creation across all kit docs
    return () => {}; // Placeholder unsubscribe
  };

  onDesignEditorCreated = (subscribe: Subscribe): Unsubscribe => {
    // Observe design editor stores map changes
    const yDesignEditorStores = this.yDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
    const observer = () => subscribe();
    yDesignEditorStores.observe(observer);
    return () => {
      yDesignEditorStores.unobserve(observer);
    };
  };

  onKitDeleted = (subscribe: Subscribe): Unsubscribe => {
    // Would need to observe kit document removal
    return () => {}; // Placeholder
  };

  onDesignEditorDeleted = (subscribe: Subscribe): Unsubscribe => {
    // Would need to observe design editor removal
    return () => {}; // Placeholder
  };

  onChanged = (subscribe: Subscribe): Unsubscribe => {
    const observer = () => subscribe();
    this.getYSketchpad().observe(observer);
    return () => {
      this.getYSketchpad().unobserve(observer);
    };
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {
    const observer = () => subscribe();
    this.getYSketchpad().observeDeep(observer);
    return () => {
      this.getYSketchpad().unobserveDeep(observer);
    };
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.sketchpad.createKit") {
      const kit = rest[0] as Kit;
      if (!kit.name) throw new Error("Kit name is required to create a kit.");

      const kitId = kitIdLikeToKitId(kit);
      const kitIdStr = kitIdToString(kitId);
      if (this.kits.has(kitIdStr)) {
        throw new Error(`Kit (${kitId.name}, ${kitId.version || ""}) already exists.`);
      }

      this.createKit(kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createDesignEditor") {
      const id = rest[0] as DesignEditorId;
      this.createDesignEditor(id);
      return {} as T;
    }
    if (command === "semio.sketchpad.importKit") {
      const kitId = rest[0] as KitId;
      const url = rest[1] as string;
      const kitStore = this.kits.get(kitIdToString(kitId));
      if (kitStore) {
        await kitStore.execute("semio.kit.import", url);
      }
      return {} as T;
    }
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in sketchpad store`);
    const context: SketchpadCommandContext = {
      sketchpad: this.snapshot(),
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
}

const stores: Map<string, YSketchpadStore> = new Map();

// Factory function to create or get a store
export function getOrCreateSketchpadStore(id: string, persisted: boolean = true): SketchpadStore {
  if (!stores.has(id)) {
    const initialState: SketchpadState = {
      mode: Mode.GUEST,
      theme: Theme.SYSTEM,
      layout: Layout.NORMAL,
      persistantId: persisted ? id : undefined,
    };
    const store = new YSketchpadStore(initialState);

    Object.entries(sketchpadCommands).forEach(([commandId, command]) => {
      store.registerCommand(commandId, command);
    });

    stores.set(id, store);
  }
  return stores.get(id)!;
}

// #endregion Stores

// #region Commands

const sketchpadCommands = {
  "semio.sketchpad.setTheme": (context: SketchpadCommandContext, theme: Theme): SketchpadCommandResult => {
    return {
      diff: { theme },
    };
  },
  "semio.sketchpad.setMode": (context: SketchpadCommandContext, mode: Mode): SketchpadCommandResult => {
    return {
      diff: { mode },
    };
  },
  "semio.sketchpad.setLayout": (context: SketchpadCommandContext, layout: Layout): SketchpadCommandResult => {
    return {
      diff: { layout },
    };
  },
  "semio.sketchpad.setActiveDesignEditor": (context: SketchpadCommandContext, id: DesignEditorId): SketchpadCommandResult => {
    return {
      diff: { activeDesignEditor: id },
    };
  },
};

const kitCommands = {
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, typeId: TypeId, typeDiff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ id: typeId, diff: typeDiff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, typeId: TypeId): KitCommandResult => {
    return {
      diff: { types: { removed: [typeId] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, designId: DesignId, designDiff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ id: designId, diff: designDiff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, designId: DesignId): KitCommandResult => {
    return {
      diff: { designs: { removed: [designId] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], file.path.split("/").pop() || file.path)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, url: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], url.split("/").pop() || url)] : [];
    return {
      diff: { files: { updated: [{ id: { path: url }, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, url: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [{ path: url }] } },
    };
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
              files.push(new File([fileBlob], file.path));
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
                uri: (kitData.uri as string) || (kitData.name as string),
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
              types: kit.types ? { added: kit.types } : undefined,
              designs: kit.designs ? { added: kit.designs } : undefined,
              files: kit.files ? { added: kit.files } : undefined,
            },
            files,
          };
        }
      } catch (error) {
        console.error("Error importing kit:", error);
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
        const insertAuthors = (authors: Author[] | undefined, fkColumn: string, fkValue: number) => {
          if (!authors) return;
          const stmt = db.prepare(`INSERT INTO author (name, email, rank, ${fkColumn}) VALUES (?, ?, ?, ?)`);
          let rank = 0;
          authors.forEach((a) => stmt.run([a.name, a.email ?? "", rank++, fkValue]));
          stmt.free();
        };

        const kitStmt = db.prepare("INSERT INTO kit (uri, name, description, icon, image, preview, version, remote, homepage, license, createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const nowIso = new Date().toISOString();
        kitStmt.run([`urn:kit:${kit.name}:${kit.version || ""}`, kit.name, kit.description || "", kit.icon || "", kit.image || "", kit.preview || "", kit.version || "", kit.remote || "", kit.homepage || "", kit.license || "", nowIso, nowIso]);
        kitStmt.free();
        const kitId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
        insertQualities(kit.attributes, "kit_id", kitId);

        if (kit.concepts) {
          const conceptStmt = db.prepare('INSERT INTO concept (name, "order", kit_id) VALUES (?, ?, ?)');
          kit.concepts.forEach((concept, index) => conceptStmt.run([concept, index, kitId]));
          conceptStmt.free();
        }

        if (kit.types) {
          const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, variant, unit, createdAt, updatedAt, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
          const repStmt = db.prepare("INSERT INTO representation (url, description, type_id) VALUES (?, ?, ?)");
          const tagStmt = db.prepare('INSERT INTO tag (name, "order", representation_id) VALUES (?, ?, ?)');
          const portStmt = db.prepare("INSERT INTO port (local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

          for (const type of kit.types) {
            typeStmt.run([type.name, type.description || "", type.icon || "", type.image || "", type.variant || "", type.unit, nowIso, nowIso, kitId]);
            const typeDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
            insertQualities(type.attributes, "type_id", typeDbId);
            insertAuthors(type.authors, "type_id", typeDbId);

            if (type.representations) {
              for (const rep of type.representations) {
                repStmt.run([rep.url, rep.description ?? "", typeDbId]);
                const repDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                insertQualities(rep.attributes, "representation_id", repDbId);
                if (rep.tags) {
                  rep.tags.forEach((tag, index) => tagStmt.run([tag, index, repDbId]));
                }
                const fileUrl = context.fileUrls.get(rep.url);
                if (fileUrl) {
                  try {
                    const response = await fetch(fileUrl);
                    const fileBlob = await response.blob();
                    const fileData = await fileBlob.arrayBuffer();
                    zip.file(rep.url, fileData);
                  } catch (error) {}
                }
              }
            }

            if (type.ports) {
              for (const port of type.ports) {
                portStmt.run([
                  port.id_ || "",
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
        console.error("Error exporting kit:", error);
        throw error;
      } finally {
        if (db) {
          db.close();
        }
      }
    })();
    return { diff: {} };
  },
  "semio.kit.addPiece": (context: KitCommandContext, designId: DesignId, piece: Piece): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { pieces: { added: [piece] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addPieces": (context: KitCommandContext, designId: DesignId, pieces: Piece[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { pieces: { added: pieces } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePiece": (context: KitCommandContext, designId: DesignId, pieceId: PieceId): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { pieces: { removed: [pieceId] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePieces": (context: KitCommandContext, designId: DesignId, pieceIds: PieceId[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { pieces: { removed: pieceIds } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnection": (context: KitCommandContext, designId: DesignId, connection: Connection): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnections": (context: KitCommandContext, designId: DesignId, connections: Connection[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnection": (context: KitCommandContext, designId: DesignId, connectionId: ConnectionId): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { connections: { removed: [connectionId] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, designId: DesignId, connectionIds: ConnectionId[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { connections: { removed: connectionIds } },
            },
          ],
        },
      },
    };
  },
};

const designEditorCommands = {
  "semio.designEditor.setMode": (context: DesignEditorCommandContext, mode: Mode): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.setTheme": (context: DesignEditorCommandContext, theme: Theme): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.setLayout": (context: DesignEditorCommandContext, layout: Layout): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.toggleDiagramFullscreen": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const currentPanel = context.designEditor.fullscreenPanel;
    const newPanel = currentPanel === DesignEditorFullscreenPanel.Diagram ? DesignEditorFullscreenPanel.None : DesignEditorFullscreenPanel.Diagram;
    return {
      diff: {
        fullscreenPanel: newPanel,
      },
    };
  },
  "semio.designEditor.toggleModelFullscreen": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const currentPanel = context.designEditor.fullscreenPanel;
    const newPanel = currentPanel === DesignEditorFullscreenPanel.Model ? DesignEditorFullscreenPanel.None : DesignEditorFullscreenPanel.Model;
    return {
      diff: {
        fullscreenPanel: newPanel,
      },
    };
  },
  "semio.designEditor.selectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const design = findDesignInKit(context.kit, context.designId)!;
    const currentSelection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection.pieces ?? [],
            added: design.pieces ?? [],
          },
          connections: {
            removed: currentSelection.connections ?? [],
            added: design.connections ?? [],
          },
        },
      },
    };
  },
  "semio.designEditor.deselectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const currentSelection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: { removed: currentSelection.pieces ?? [] },
          connections: { removed: currentSelection.connections ?? [] },
        },
      },
    };
  },
  "semio.designEditor.selectPiece": (context: DesignEditorCommandContext, pieceId: PieceId): DesignEditorCommandResult => {
    const currentSelection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection.pieces ?? [],
            added: [pieceId],
          },
          connections: { removed: currentSelection.connections ?? [] },
        },
      },
    };
  },
  "semio.designEditor.selectPieces": (context: DesignEditorCommandContext, pieceIds: PieceId[]): DesignEditorCommandResult => {
    const currentSelection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: {
            removed: currentSelection.pieces ?? [],
            added: pieceIds,
          },
          connections: { removed: currentSelection.connections ?? [] },
        },
      },
    };
  },
  "semio.designEditor.addPieceToSelection": (context: DesignEditorCommandContext, pieceId: PieceId): DesignEditorCommandResult => {
    return {
      diff: {
        selection: {
          pieces: { added: [pieceId] },
        },
      },
    };
  },
  "semio.designEditor.removePieceFromSelection": (context: DesignEditorCommandContext, pieceId: PieceId): DesignEditorCommandResult => {
    return {
      diff: {
        selection: {
          pieces: { removed: [pieceId] },
        },
      },
    };
  },
  "semio.designEditor.selectPiecePort": (context: DesignEditorCommandContext, pieceId: PieceId, portId: PortId): DesignEditorCommandResult => {
    return {
      diff: {
        selection: { port: { piece: pieceId, port: portId } },
      },
    };
  },
  "semio.designEditor.deselectPiecePort": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    return {
      diff: {
        selection: { port: undefined },
      },
    };
  },
  "semio.designEditor.deleteSelected": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const selection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: { removed: selection.pieces ?? [] },
          connections: { removed: selection.connections ?? [] },
        },
      },
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { removed: selection.pieces }, connections: { removed: selection.connections } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.addPiece": (context: DesignEditorCommandContext, piece: Piece): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { added: [piece] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.addPieces": (context: DesignEditorCommandContext, pieces: Piece[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { added: pieces } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.removePiece": (context: DesignEditorCommandContext, pieceId: PieceId): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { removed: [pieceId] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.removePieces": (context: DesignEditorCommandContext, pieceIds: PieceId[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { removed: pieceIds } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.addConnection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.addConnections": (context: DesignEditorCommandContext, connections: Connection[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.removeConnection": (context: DesignEditorCommandContext, connectionId: ConnectionId): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { removed: [connectionId] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.removeConnections": (context: DesignEditorCommandContext, connectionIds: ConnectionId[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { removed: connectionIds } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.updatePiece": (context: DesignEditorCommandContext, pieceId: PieceId, pieceDiff: PieceDiff): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { updated: [{ id: pieceId, diff: pieceDiff }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.updatePieces": (context: DesignEditorCommandContext, updates: { id: PieceId; diff: PieceDiff }[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { updated: updates } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.updateConnection": (context: DesignEditorCommandContext, connectionId: ConnectionId, connectionDiff: ConnectionDiff): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { updated: [{ id: connectionId, diff: connectionDiff }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.updateConnections": (context: DesignEditorCommandContext, updates: { id: ConnectionId; diff: ConnectionDiff }[]): DesignEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { updated: updates } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.selectConnection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
    const currentSelection = context.designEditor.selection;
    return {
      diff: {
        selection: {
          pieces: { removed: currentSelection.pieces ?? [] },
          connections: {
            removed: currentSelection.connections ?? [],
            added: [connection],
          },
        },
      },
    };
  },
  "semio.designEditor.addConnectionToSelection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
    return {
      diff: {
        selection: {
          connections: { added: [connection] },
        },
      },
    };
  },
  "semio.designEditor.removeConnectionFromSelection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
    return {
      diff: {
        selection: {
          connections: { removed: [connection] },
        },
      },
    };
  },
};

// #endregion Commands

// #region Hooks

// #region Scoping

type SketchpadScope = { id: string; persisted: boolean };
const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
export const SketchpadScopeProvider = (props: { id: string; persisted: boolean; children: React.ReactNode }) => {
  const scope = { id: props.id, persisted: props.persisted };
  // Ensure store is created/initialized
  getOrCreateSketchpadStore(props.id, props.persisted);
  const value = scope;
  return React.createElement(SketchpadScopeContext.Provider, { value }, props.children as any);
};

type KitScope = { id: KitId };
const KitScopeContext = createContext<KitScope | null>(null);

type DesignScope = { id: DesignId };
const DesignScopeContext = createContext<DesignScope | null>(null);

type TypeScope = { id: TypeId };
const TypeScopeContext = createContext<TypeScope | null>(null);

type PieceScope = { id: PieceId };
const PieceScopeContext = createContext<PieceScope | null>(null);

const ConnectionScopeContext = createContext<ConnectionScope | null>(null);
type ConnectionScope = { id: ConnectionId };

const RepresentationScopeContext = createContext<RepresentationScope | null>(null);
type RepresentationScope = { id: RepresentationId };

type PortScope = { id: PortId };
const PortScopeContext = createContext<PortScope | null>(null);

type DesignEditorScope = { id: string };
const DesignEditorScopeContext = createContext<DesignEditorScope | null>(null);

export const KitScopeProvider = (props: { id: KitId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};

export const DesignScopeProvider = (props: { id: DesignId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignScopeContext.Provider, { value }, props.children as any);
};

export const TypeScopeProvider = (props: { id: TypeId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeScopeContext.Provider, { value }, props.children as any);
};

export const PieceScopeProvider = (props: { id: PieceId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(PieceScopeContext.Provider, { value }, props.children as any);
};

export const ConnectionScopeProvider = (props: { id: ConnectionId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(ConnectionScopeContext.Provider, { value }, props.children as any);
};

export const RepresentationScopeProvider = (props: { id: RepresentationId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(RepresentationScopeContext.Provider, { value }, props.children as any);
};

export const PortScopeProvider = (props: { id: PortId; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(PortScopeContext.Provider, { value }, props.children as any);
};

export const DesignEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignEditorScopeContext.Provider, { value }, props.children as any);
};

const useSketchpadScope = () => useContext(SketchpadScopeContext);
const useKitStoreScope = () => useContext(KitScopeContext);
const useDesignScope = () => useContext(DesignScopeContext);
const useTypeScope = () => useContext(TypeScopeContext);
const usePieceScope = () => useContext(PieceScopeContext);
const useConnectionScope = () => useContext(ConnectionScopeContext);
const useRepresentationScope = () => useContext(RepresentationScopeContext);
const usePortScope = () => useContext(PortScopeContext);
const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

// #endregion Scoping

const identitySelector = (state: any) => state;

function useSync<TSnapshot, TSelected = TSnapshot>(store: Store<TSnapshot>, selector?: (state: TSnapshot) => TSelected, deep: boolean = false): TSnapshot | TSelected {
  const state = deep ? useSyncExternalStore(store.onChangedDeep, store.snapshot) : useSyncExternalStore(store.onChanged, store.snapshot);
  return selector ? selector(state) : state;
}

function useSyncDeep<TSnapshot, TSelected = TSnapshot>(store: { snapshot: () => TSnapshot; onChangedDeep: (subscribe: Subscribe) => Unsubscribe }, selector?: (state: TSnapshot) => TSelected): TSnapshot | TSelected {
  const state = useSyncExternalStore(store.onChangedDeep, store.snapshot);
  return selector ? selector(state) : state;
}

function useSketchpadStore(id?: string) {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpadStore must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  return store;
}

export function useSketchpad<T>(selector?: (state: SketchpadState) => T, id?: string): T | SketchpadState {
  return useSync<SketchpadState, T>(useSketchpadStore(id), selector ? selector : identitySelector);
}

function useKitStore<T>(selector?: (store: KitStore) => T, id?: KitId): T | KitStore {
  const store = useSketchpadStore();
  const kitScope = useKitStoreScope();
  const kitId = kitScope?.id ?? id;
  if (!kitId) throw new Error("useKitStore must be called within a KitScopeProvider or be directly provided with an id");
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitIdStr)!;
  return selector ? selector(kitStore) : kitStore;
}

export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, id?: KitId, deep: boolean = false): T | KitShallow | Kit {
  if (deep) {
    return useSyncDeep<Kit, T>(useKitStore(identitySelector, id) as KitStore, selector ? selector : identitySelector);
  }
  return useSync<KitShallow, T>(useKitStore(identitySelector, id) as KitStore, selector ? selector : identitySelector, deep);
}

export function useDiffedKit(): Kit {
  const kit = useKit() as Kit;
  const diff = useDesignEditorDiff();
  return applyKitDiff(kit, diff);
}

function useDesignEditorStore<T>(selector?: (store: DesignEditorStore) => T, id?: DesignEditorId): T | DesignEditorStore {
  const store = useSketchpadStore();
  const kitScope = useKitStoreScope();
  const resolvedKitId = kitScope?.id ?? id?.kit;
  if (!resolvedKitId) throw new Error("useDesignEditorStore must be called within a KitScopeProvider or be directly provided with an id");
  const resolvedKitIdStr = kitIdToString(resolvedKitId);
  const designScope = useDesignScope();
  const resolvedDesignId = designScope?.id ?? id?.design;
  if (!resolvedDesignId) throw new Error("useDesignEditorStore must be called within a DesignScopeProvider or be directly provided with an id");
  const resolvedDesignIdStr = designIdToString(resolvedDesignId);
  if (!store.designEditors.has(resolvedKitIdStr)) throw new Error(`Design editor not found for kit ${resolvedKitId.name}`);
  const kitEditors = store.designEditors.get(resolvedKitIdStr)!;
  if (!kitEditors.has(resolvedDesignIdStr)) throw new Error(`Design editor not found for design ${resolvedDesignId.name}`);
  const designEditorStore = kitEditors.get(resolvedDesignIdStr)! as DesignEditorStore;
  return selector ? selector(designEditorStore) : designEditorStore;
}

export function useDesignEditor<T>(selector?: (state: DesignEditorState) => T, id?: DesignEditorId): T | DesignEditorState {
  return useSync<DesignEditorState, T>(useDesignEditorStore(identitySelector, id) as DesignEditorStore, selector ? selector : identitySelector);
}

function useDesignStore<T>(selector?: (store: DesignStore) => T, id?: DesignId): T | DesignStore {
  const kitStore = useKitStore() as KitStore;
  const designScope = useDesignScope();
  const designId = designScope?.id ?? id;
  if (!designId) throw new Error("useDesignStore must be called within a DesignScopeProvider or be directly provided with an id");
  const designIdStr = designIdToString(designId);
  if (!kitStore.designs.has(designIdStr)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kitStore.designs.get(designIdStr)! as DesignStore;
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: DesignId, deep: boolean = false): T | DesignShallow | Design {
  if (deep) {
    return useSyncDeep<Design, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector);
  }
  return useSync<DesignShallow, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector, deep);
}

export function useDiffedDesign(): Design {
  const kit = useDiffedKit();
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useDiffedDesign must be called within a DesignScopeProvider");
  return findDesignInKit(kit, designScope.id);
}

export function useFlattenDiff(): DesignDiff {
  const designScope = useDesignScope();
  const kit = useKit() as Kit;
  if (!designScope) throw new Error("useFlattenDiff must be called within a DesignScopeProvider");
  return flattenDesign(kit, designScope.id);
}

export function useFlatDesign(): Design {
  const design = useDesign() as Design;
  const diff = useFlattenDiff();
  return applyDesignDiff(design, diff, true);
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
  return piecesMetadata(kit, designScope.id);
}

function useTypeStore<T>(selector?: (store: TypeStore) => T, id?: TypeId): T | TypeStore {
  const kitStore = useKitStore() as KitStore;
  const typeScope = useTypeScope();
  const typeId = typeScope?.id ?? id;
  if (!typeId) throw new Error("useTypeStore must be called within a TypeScopeProvider or be directly provided with an id");
  const typeIdStr = typeIdToString(typeId);
  if (!kitStore.types.has(typeIdStr)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kitStore.types.get(typeIdStr)! as TypeStore;
  return selector ? selector(typeStore) : typeStore;
}

export function useType<T>(selector?: (type: Type) => T, id?: TypeId, deep: boolean = false): T | Type {
  return useSync<Type, T>(useTypeStore(identitySelector, id) as TypeStore, selector ? selector : identitySelector, deep);
}

export function usePortColoredTypes(): Type[] {
  const diffedKit = useDiffedKit();
  const typesWithColoredPorts = useMemo(() => {
    if (!diffedKit.types) return [];
    const colorDiff = colorPortsForTypes(diffedKit.types);
    return colorDiff.updated
      ? diffedKit.types.map((type) => {
          const update = colorDiff.updated?.find((u) => u.id.name === type.name && u.id.variant === type.variant);
          return update ? { ...type, ports: update.diff.ports } : type;
        })
      : diffedKit.types;
  }, [diffedKit.types]);
  const unified = useMemo(() => ({ ...diffedKit, types: typesWithColoredPorts }), [diffedKit, typesWithColoredPorts]);
  return unified.types;
}

function usePieceStore<T>(selector?: (store: PieceStore) => T, id?: PieceId): T | PieceStore {
  const designStore = useDesignStore() as DesignStore;
  const pieceScope = usePieceScope();
  const pieceId = pieceScope?.id ?? id;
  if (!pieceId) throw new Error("usePieceStore must be called within a PieceScopeProvider or be directly provided with an id");
  const pieceIdStr = pieceIdToString(pieceId);
  if (!designStore.pieces.has(pieceIdStr)) throw new Error(`Piece store not found for piece ${pieceId}`);
  const pieceStore = designStore.pieces.get(pieceIdStr)! as PieceStore;
  return selector ? selector(pieceStore) : pieceStore;
}

export function usePiece<T>(selector?: (piece: Piece) => T, id?: PieceId, deep: boolean = false): T | Piece {
  return useSync<Piece, T>(usePieceStore(identitySelector, id) as PieceStore, selector ? selector : identitySelector, deep);
}

export function useIsPieceSelected(): boolean {
  const piece = usePieceScope();
  const selection = useDesignEditorSelection();
  return selection.pieces?.some((p) => p.id_ === piece?.id) ?? false;
}

export function useIsPieceHovered(): boolean {
  // const hover = useDesignEditorHover();
  // return hover.piece?.id_ === piece.id_ ?? false;
  return false;
}

export function usePiecePlane(): Plane {
  const plane = usePiece((p) => p.plane);
  // TODO: integrate flat piece plane otherwise
  return plane as Plane;
}

export function usePieceStatus(): DiffStatus {
  // TODO: Check diff for status
  return DiffStatus.Unchanged;
}
usePieceStatus();

function useConnectionStore<T>(selector?: (store: ConnectionStore) => T, id?: ConnectionId): T | ConnectionStore {
  const designStore = useDesignStore() as DesignStore;
  const connectionScope = useConnectionScope();
  const connectionId = connectionScope?.id ?? id;
  if (!connectionId) throw new Error("useConnectionStore must be called within a ConnectionScopeProvider or be directly provided with an id");
  const connectionIdStr = connectionIdToString(connectionId);
  if (!designStore.connections.has(connectionIdStr)) throw new Error(`Connection store not found for connection ${connectionId}`);
  const connectionStore = designStore.connections.get(connectionIdStr)! as ConnectionStore;
  return selector ? selector(connectionStore) : connectionStore;
}

export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId, deep: boolean = false): T | Connection {
  return useSync<Connection, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ? selector : identitySelector, deep);
}

function usePortStore<T>(selector?: (store: PortStore) => T, id?: PortId): T | PortStore {
  const typeStore = useTypeStore() as TypeStore;
  const portScope = usePortScope();
  const portId = portScope?.id ?? id;
  if (!portId) throw new Error("usePortStore must be called within a PortScopeProvider or be directly provided with an id");
  const portIdStr = portIdToString(portId);
  if (!typeStore.ports.has(portIdStr)) throw new Error(`Port store not found for port ${portId}`);
  const portStore = typeStore.ports.get(portIdStr)! as PortStore;
  return selector ? selector(portStore) : portStore;
}

export function usePort<T>(selector?: (port: Port) => T, id?: PortId, deep: boolean = false): T | Port {
  return useSync<Port, T>(usePortStore(identitySelector, id) as PortStore, selector ? selector : identitySelector, deep);
}

function useRepresentationStore<T>(selector?: (store: RepresentationStore) => T, id?: RepresentationId): T | RepresentationStore {
  const typeStore = useTypeStore() as TypeStore;
  const representationScope = useRepresentationScope();
  const representationId = representationScope?.id ?? id;
  if (!representationId) throw new Error("useRepresentationStore must be called within a RepresentationScopeProvider or be directly provided with an id");
  const representationIdStr = representationIdToString(representationId);
  if (!typeStore.representations.has(representationIdStr)) throw new Error(`Representation store not found for representation ${representationId}`);
  const representationStore = typeStore.representations.get(representationIdStr)! as RepresentationStore;
  return selector ? selector(representationStore) : representationStore;
}

export function useRepresentation<T>(selector?: (representation: Representation) => T, id?: RepresentationId, deep: boolean = false): T | Representation {
  return useSync<Representation, T>(useRepresentationStore(identitySelector, id) as RepresentationStore, selector ? selector : identitySelector, deep);
}

// Additional utility hooks for the new store architecture

export function useMode(): Mode {
  return useSketchpad((s) => s.mode) as Mode;
}

export function useTheme(): Theme {
  return useSketchpad((s) => s.theme) as Theme;
}

export function useLayout(): Layout {
  return useSketchpad((s) => s.layout) as Layout;
}

export function useActiveDesignEditor(): DesignId | undefined {
  return (useSketchpad((s) => s.activeDesignEditor) as DesignEditorId)?.design;
}

export function useDesigns(): Design[] {
  return useKit((k) => k.designs ?? []) as Design[];
}

export function useSketchpadCommands() {
  const store = useSketchpadStore();
  return {
    setMode: (mode: Mode) => store.execute("semio.sketchpad.setMode", mode),
    setTheme: (theme: Theme) => store.execute("semio.sketchpad.setTheme", theme),
    setLayout: (layout: Layout) => store.execute("semio.sketchpad.setLayout", layout),
    createKit: (kit: Kit) => store.execute("semio.sketchpad.createKit", kit),
    createDesignEditor: (designEditorId: DesignEditorId) => store.execute("semio.sketchpad.createDesignEditor", designEditorId),
    setActiveDesignEditor: (designEditorId: DesignEditorId) => store.execute("semio.sketchpad.setActiveDesignEditor", designEditorId),
  };
}

export function useKitCommands() {
  const store = useKitStore() as KitStore;
  return {
    importKit: (url: string) => store.execute("semio.kit.import", url),
    exportKit: () => store.execute("semio.kit.export"),
    createType: (type: Type) => store.execute("semio.kit.createType", type),
    updateType: (typeId: TypeId, typeDiff: TypeDiff) => store.execute("semio.kit.updateType", typeId, typeDiff),
    deleteType: (typeId: TypeId) => store.execute("semio.kit.deleteType", typeId),
    createDesign: (design: Design) => store.execute("semio.kit.createDesign", design),
    updateDesign: (designId: DesignId, designDiff: DesignDiff) => store.execute("semio.kit.updateDesign", designId, designDiff),
    deleteDesign: (designId: DesignId) => store.execute("semio.kit.deleteDesign", designId),
    addFile: (file: SemioFile, blob?: Blob) => store.execute("semio.kit.addFile", file, blob),
    updateFile: (url: Url, fileDiff: FileDiff, blob?: Blob) => store.execute("semio.kit.updateFile", url, fileDiff, blob),
    removeFile: (url: Url) => store.execute("semio.kit.removeFile", url),
    addPiece: (designId: DesignId, piece: Piece) => store.execute("semio.kit.addPiece", designId, piece),
    addPieces: (designId: DesignId, pieces: Piece[]) => store.execute("semio.kit.addPieces", designId, pieces),
    removePiece: (designId: DesignId, pieceId: PieceId) => store.execute("semio.kit.removePiece", designId, pieceId),
    removePieces: (designId: DesignId, pieceIds: PieceId[]) => store.execute("semio.kit.removePieces", designId, pieceIds),
    addConnection: (designId: DesignId, connection: Connection) => store.execute("semio.kit.addConnection", designId, connection),
    addConnections: (designId: DesignId, connections: Connection[]) => store.execute("semio.kit.addConnections", designId, connections),
    removeConnection: (designId: DesignId, connectionId: ConnectionId) => store.execute("semio.kit.removeConnection", designId, connectionId),
    removeConnections: (designId: DesignId, connectionIds: ConnectionId[]) => store.execute("semio.kit.removeConnections", designId, connectionIds),
    deleteSelected: (designId: DesignId, selectedPieces: PieceId[], selectedConnections: ConnectionId[]) => store.execute("semio.kit.deleteSelected", designId, selectedPieces, selectedConnections),
  };
}

export function useDesignEditorCommands() {
  const store = useDesignEditorStore() as DesignEditorStore;
  return {
    startTransaction: () => store.execute("semio.designEditor.startTransaction"),
    finalizeTransaction: () => store.execute("semio.designEditor.finalizeTransaction"),
    abortTransaction: () => store.execute("semio.designEditor.abortTransaction"),
    undo: () => store.execute("semio.designEditor.undo"),
    redo: () => store.execute("semio.designEditor.redo"),
    selectAll: () => store.execute("semio.designEditor.selectAll"),
    deselectAll: () => store.execute("semio.designEditor.deselectAll"),
    selectPiece: (pieceId: PieceId) => store.execute("semio.designEditor.selectPiece", pieceId),
    selectPieces: (pieceIds: PieceId[]) => store.execute("semio.designEditor.selectPieces", pieceIds),
    addPieceToSelection: (pieceId: PieceId) => store.execute("semio.designEditor.addPieceToSelection", pieceId),
    removePieceFromSelection: (pieceId: PieceId) => store.execute("semio.designEditor.removePieceFromSelection", pieceId),
    selectConnection: (connection: Connection) => store.execute("semio.designEditor.selectConnection", connection),
    addConnectionToSelection: (connection: Connection) => store.execute("semio.designEditor.addConnectionToSelection", connection),
    removeConnectionFromSelection: (connection: Connection) => store.execute("semio.designEditor.removeConnectionFromSelection", connection),
    selectPiecePort: (pieceId: PieceId, portId: PortId) => store.execute("semio.designEditor.selectPiecePort", pieceId, portId),
    deselectPiecePort: () => store.execute("semio.designEditor.deselectPiecePort"),
    deleteSelected: () => store.execute("semio.designEditor.deleteSelected"),
    toggleDiagramFullscreen: () => store.execute("semio.designEditor.toggleDiagramFullscreen"),
    toggleModelFullscreen: () => store.execute("semio.designEditor.toggleModelFullscreen"),
    addPiece: (piece: Piece) => store.execute("semio.designEditor.addPiece", piece),
    addPieces: (pieces: Piece[]) => store.execute("semio.designEditor.addPieces", pieces),
    removePiece: (pieceId: PieceId) => store.execute("semio.designEditor.removePiece", pieceId),
    removePieces: (pieceIds: PieceId[]) => store.execute("semio.designEditor.removePieces", pieceIds),
    addConnection: (connection: Connection) => store.execute("semio.designEditor.addConnection", connection),
    addConnections: (connections: Connection[]) => store.execute("semio.designEditor.addConnections", connections),
    removeConnection: (connectionId: ConnectionId) => store.execute("semio.designEditor.removeConnection", connectionId),
    removeConnections: (connectionIds: ConnectionId[]) => store.execute("semio.designEditor.removeConnections", connectionIds),
    updatePiece: (pieceId: PieceId, pieceDiff: PieceDiff) => store.execute("semio.designEditor.updatePiece", pieceId, pieceDiff),
    updatePieces: (updates: { id: PieceId; diff: PieceDiff }[]) => store.execute("semio.designEditor.updatePieces", updates),
    updateConnection: (connectionId: ConnectionId, connectionDiff: ConnectionDiff) => store.execute("semio.designEditor.updateConnection", connectionId, connectionDiff),
    updateConnections: (updates: { id: ConnectionId; diff: ConnectionDiff }[]) => store.execute("semio.designEditor.updateConnections", updates),
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}
// Design editor state hooks

export function useDesignEditorSelection(): DesignEditorSelection {
  return useDesignEditor((s) => s.selection) as DesignEditorSelection;
}

export function useDesignEditorFullscreen(): DesignEditorFullscreenPanel {
  return useDesignEditor((s) => s.fullscreenPanel) as DesignEditorFullscreenPanel;
}

export function useDesignEditorDiff(): KitDiff {
  return useDesignEditor((s) => s.diff) as KitDiff;
}

export function useFileUrls(): Map<Url, Url> {
  return (useKitStore() as KitStore).fileUrls();
}

export function useDesignEditorOthers(): DesignEditorPresenceOther[] {
  return useDesignEditor((s) => s.others) as DesignEditorPresenceOther[];
}

// Domain-specific computed hooks

export function usePiecePlanes(): Plane[] {
  const flatDesign = useFlatDesign();
  return useMemo(() => flatDesign.pieces?.map((p: Piece) => p.plane!) || [], [flatDesign]);
}

export function usePieceRepresentationUrls(): Map<string, string> {
  const flatDesign = useFlatDesign();
  const types = usePortColoredTypes();
  return useMemo(() => getPieceRepresentationUrls(flatDesign, types), [flatDesign, types]);
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

export function useTypesByName(): Record<string, Type[]> {
  const kit = useKit();
  return useMemo(() => {
    if (!kit?.types) return {};
    return kit.types.reduce(
      (acc, type) => {
        acc[type.name] = acc[type.name] || [];
        acc[type.name].push(type);
        return acc;
      },
      {} as Record<string, Type[]>,
    );
  }, [kit?.types]);
}

export function useDesignsByName(): Record<string, Design[]> {
  const kit = useKit();
  return useMemo(() => {
    if (!kit?.designs) return {};
    return kit.designs.reduce(
      (acc, design) => {
        const nameKey = design.name;
        acc[nameKey] = acc[nameKey] || [];
        acc[nameKey].push(design);
        return acc;
      },
      {} as Record<string, Design[]>,
    );
  }, [kit?.designs]);
}

export function useIncludedDesigns() {
  const design = useDesign();
  return useMemo(() => getIncludedDesigns(design), [design]);
}

export function useReplacableTypes(pieceIds: PieceId[], selectedVariants?: string[]) {
  const kit = useKit();
  const design = useDesign();
  const designId = useMemo(() => ({ name: design.name, variant: design.variant, view: design.view }), [design.name, design.variant, design.view]);

  return useMemo(() => {
    if (pieceIds.length === 1) {
      return findReplacableTypesForPieceInDesign(kit, designId, pieceIds[0], selectedVariants);
    } else {
      return findReplacableTypesForPiecesInDesign(kit, designId, pieceIds, selectedVariants);
    }
  }, [kit, designId, pieceIds, selectedVariants]);
}

export function useReplacableDesigns(piece: Piece) {
  const kit = useKit();
  const design = useDesign();
  const designId = useMemo(() => ({ name: design.name, variant: design.variant, view: design.view }), [design.name, design.variant, design.view]);

  return useMemo(() => {
    return findReplacableDesignsForDesignPiece(kit, designId, piece);
  }, [kit, designId, piece]);
}

export function usePiecesFromIds(pieceIds: PieceId[]) {
  const design = useDesign();
  const includedDesigns = useIncludedDesigns();
  const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.id, d])), [includedDesigns]);

  return useMemo(() => {
    return pieceIds.map((id) => {
      try {
        const foundPiece = findPieceInDesign(design, id);
        return {
          ...foundPiece,
          id_: typeof foundPiece.id_ === "string" ? foundPiece.id_ : (foundPiece.id_ as any).id_,
        };
      } catch {
        const pieceIdString = typeof id === "string" ? id : (id as any).id_;
        const includedDesign = includedDesignMap.get(pieceIdString);
        if (includedDesign) {
          return {
            id_: pieceIdString,
            type: {
              name: "design",
              variant:
                includedDesign.type === "fixed"
                  ? `${includedDesign.designId.name}${includedDesign.designId.variant ? `-${includedDesign.designId.variant}` : ""}${includedDesign.designId.view ? `-${includedDesign.designId.view}` : ""}`
                  : includedDesign.designId.name,
            },
            center: includedDesign.center,
            plane: includedDesign.plane,
            description: `${includedDesign.type === "fixed" ? "Fixed" : "Clustered"} design: ${includedDesign.designId.name}`,
          };
        }

        console.warn(`Piece ${pieceIdString} not found in pieces or includedDesigns. Creating fallback piece.`);
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

export function useDesignId() {
  const design = useDesign();
  return useMemo(() => ({ name: design.name, variant: design.variant, view: design.view }), [design.name, design.variant, design.view]);
}

export function useClusterableGroups() {
  const design = useDesign();
  const selection = useDesignEditorSelection();
  return useMemo(() => {
    if (!design) return [];
    return getClusterableGroups(
      design,
      selection.pieces.map((p: any) => p.id_),
    );
  }, [design, selection.pieces]);
}

export function useExplodeableDesignNodes(nodes: any[], selection: any) {
  const kit = useKit();
  return useMemo(() => {
    return nodes.filter((node) => {
      if (node.type !== "design") return false;
      const pieceId = node.data.piece.id_;
      if (!selection.pieces?.some((p: any) => p.id_ === pieceId)) return false;
      const designName = (node.data.piece as any).type?.variant;
      if (!designName) return false;
      if (!kit?.designs?.find((d) => d.name === designName)) return false;
      return true;
    });
  }, [nodes, selection.pieces, kit]);
}

// #endregion Hooks
