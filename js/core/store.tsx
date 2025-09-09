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
import React, { createContext, useCallback, useContext, useMemo, useRef, useSyncExternalStore } from "react";
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
  Connection,
  ConnectionDiff,
  ConnectionId,
  connectionIdLikeToConnectionId,
  connectionIdToString,
  Design,
  DesignDiff,
  DesignId,
  designIdLikeToDesignId,
  designIdToString,
  DiagramPoint,
  FileDiff,
  fileIdLikeToFileId,
  findDesignInKit,
  flattenDesign,
  Kit,
  KitDiff,
  KitId,
  KitIdLike,
  kitIdLikeToKitId,
  kitIdToString,
  Piece,
  PieceDiff,
  PieceId,
  pieceIdLikeToPieceId,
  pieceIdToString,
  piecesMetadata,
  Plane,
  Port,
  PortDiff,
  PortId,
  portIdLikeToPortId,
  portIdToString,
  Representation,
  RepresentationDiff,
  RepresentationId,
  representationIdLikeToRepresentationId,
  representationIdToString,
  File as SemioFile,
  Type,
  TypeDiff,
  TypeId,
  typeIdLikeToTypeId,
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
export type Url = string;

export interface DesignEditorStep {
  diff?: KitDiff;
  selection?: DesignEditorSelection;
}

export interface DesignEditorEdit {
  do: DesignEditorStep;
  undo: DesignEditorStep;
}

export interface YStore {}

export interface FileSnapshot {
  snapshot(): SemioFile;
}
export interface FileActions {
  change: (diff: FileDiff) => void;
}

export interface FileSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}

export interface FileStore extends FileSnapshot {}

export interface FileStoreFull extends FileSnapshot, FileActions, FileSubscriptions {}

export interface RepresentationSnapshot {
  snapshot(): Representation;
}

export interface RepresentationActions {
  change: (diff: RepresentationDiff) => void;
}

export interface RepresentationSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}

export interface RepresentationStore extends RepresentationSnapshot {}

export interface RepresentationStoreFull extends RepresentationSnapshot, RepresentationActions, RepresentationSubscriptions {}

export interface PortSnapshot {
  snapshot(): Port;
}
export interface PortActions {
  change: (diff: PortDiff) => void;
}
export interface PortSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface PortStore extends PortSnapshot {}
export interface PortStoreFull extends PortSnapshot, PortActions, PortSubscriptions {}

export interface TypeSnapshot {
  snapshot(): Type;
}
export interface TypeActions {
  change: (diff: TypeDiff) => void;
}
export interface TypeSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface TypeChildStores {
  representations: Map<string, RepresentationStore>;
  ports: Map<string, PortStore>;
}
export interface TypeChildStoresFull {
  representations: Map<string, RepresentationStoreFull>;
  ports: Map<string, PortStoreFull>;
}
export interface TypeStore extends TypeSnapshot, TypeChildStores {}
export interface TypeStoreFull extends TypeSnapshot, TypeChildStoresFull, TypeActions, TypeSubscriptions {}

export interface PieceSnapshot {
  snapshot(): Piece;
}
export interface PieceActions {
  change: (diff: PieceDiff) => void;
}
export interface PieceSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface PieceChildStores {}
export interface PieceChildStoresFull {}
export interface PieceStore extends PieceSnapshot, PieceChildStores {}
export interface PieceStoreFull extends PieceSnapshot, PieceChildStoresFull, PieceActions, PieceSubscriptions {}

export interface ConnectionSnapshot {
  snapshot(): Connection;
}
export interface ConnectionActions {
  change: (diff: ConnectionDiff) => void;
}
export interface ConnectionSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface ConnectionStore extends ConnectionSnapshot {}
export interface ConnectionStoreFull extends ConnectionSnapshot, ConnectionActions, ConnectionSubscriptions {}

export interface DesignSnapshot {
  snapshot(): Design;
}
export interface DesignActions {
  change: (diff: DesignDiff) => void;
}
export interface DesignSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface DesignChildStores {
  pieces: Map<string, PieceStore>;
  connections: Map<string, ConnectionStore>;
}
export interface DesignChildStoresFull {
  pieces: Map<string, PieceStoreFull>;
  connections: Map<string, ConnectionStoreFull>;
}
export interface DesignStore extends DesignSnapshot, DesignChildStores {}
export interface DesignStoreFull extends DesignSnapshot, DesignActions, DesignSubscriptions, DesignChildStoresFull {}

export interface KitSnapshot {
  snapshot(): Kit;
}
export interface KitActions {
  change: (diff: KitDiff) => void;
}
export interface KitFileUrls {
  fileUrls(): Map<Url, Url>;
}
export interface KitSubscriptions {
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
}
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
}
export interface KitCommands {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
}
export interface KitCommandsFull {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
  register(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable;
}
export interface KitChildStores {
  types: Map<string, TypeStore>;
  designs: Map<string, DesignStore>;
  files: Map<Url, FileStore>;
}
export interface KitChildStoresFull {
  types: Map<string, TypeStoreFull>;
  designs: Map<string, DesignStoreFull>;
  files: Map<Url, FileStoreFull>;
}
export interface KitStore extends KitSnapshot, KitChildStores, KitFileUrls, KitCommands, KitSubscriptions {}
export interface KitStoreFull extends KitSnapshot, KitActions, KitSubscriptions, KitCommandsFull, KitChildStoresFull, KitFileUrls {}

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
  cursor?: DiagramPoint;
  camera?: Camera;
}
export interface DesignEditorPresenceOther extends DesignEditorPresence {
  name: string;
}
export interface DesignEditorState {
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  presence: DesignEditorPresence;
}
export interface DesignEditorStateFull extends DesignEditorState {
  isTransactionActive: boolean;
  others: DesignEditorPresenceOther[];
  diff: KitDiff;
  currentTransactionStack: DesignEditorEdit[];
  pastTransactionsStack: DesignEditorEdit[];
}

export interface DesignEditorSnapshot {
  snapshot(): DesignEditorStateFull;
}
export interface DesignEditorStateDiff {
  fullscreenPanel?: DesignEditorFullscreenPanel;
  selection?: DesignEditorSelectionDiff;
  presence?: DesignEditorPresence;
  others?: DesignEditorPresenceOther[];
}
export interface DesignEditorActions {
  change: (diff: DesignEditorStateDiff) => void;
  undo: () => void;
  redo: () => void;
  startTransaction: () => void;
  abortTransaction: () => void;
  finalizeTransaction: () => void;
}
export interface DesignEditorSubscriptions {
  onUndone: (subscribe: Subscribe) => Unsubscribe;
  onRedone: (subscribe: Subscribe) => Unsubscribe;
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onTransactionStarted: (subscribe: Subscribe) => Unsubscribe;
  onTransactionAborted: (subscribe: Subscribe) => Unsubscribe;
  onTransactionFinalized: (subscribe: Subscribe) => Unsubscribe;
}
export interface DesignEditorCommandContext extends KitCommandContext {
  designEditor: DesignEditorStateFull;
  designId: DesignId;
}
export interface DesignEditorCommandResult {
  diff?: DesignEditorStateDiff;
  kitDiff?: KitDiff;
}
export interface DesignEditorCommands {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
}
export interface DesignEditorCommandsFull {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
  register(command: string, callback: (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult): Disposable;
}
export interface DesignEditorStore extends DesignEditorSnapshot, DesignEditorCommands {}
export interface DesignEditorStoreFull extends DesignEditorSnapshot, DesignEditorCommandsFull, DesignEditorActions, DesignEditorSubscriptions {}
export interface SketchpadState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditor?: DesignEditorId;
}
export interface SketchpadStateFull extends SketchpadState {
  persistantId?: string;
}

export interface SketchpadSnapshot {
  snapshot(): SketchpadStateFull;
}
export interface SketchpadStateDiff {
  mode?: Mode;
  theme?: Theme;
  layout?: Layout;
  activeDesignEditor?: DesignEditorId;
}
export interface SketchpadActions {
  createKit: (kit: Kit) => void;
  createDesignEditor: (id: DesignEditorId) => void;
  change: (diff: SketchpadStateDiff) => void;
  deleteKit: (id: KitIdLike) => void;
  deleteDesignEditor: (id: DesignEditorId) => void;
}
export interface SketchpadSubscriptions {
  onKitCreated: (subscribe: Subscribe) => Unsubscribe;
  onDesignEditorCreated: (subscribe: Subscribe) => Unsubscribe;
  onChanged: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  onKitDeleted: (subscribe: Subscribe) => Unsubscribe;
  onDesignEditorDeleted: (subscribe: Subscribe) => Unsubscribe;
}
export interface SketchpadChildStores {
  kits: Map<string, KitStore>;
  designEditors: Map<string, Map<string, DesignEditorStore>>;
}
export interface SketchpadChildStoresFull {
  kits: Map<string, KitStoreFull>;
  designEditors: Map<string, Map<string, DesignEditorStoreFull>>;
}
export interface SketchpadCommandContext {
  sketchpad: SketchpadStateFull;
  store: YSketchpadStore;
}
export interface SketchpadCommandResult {
  diff?: SketchpadStateDiff;
}
export interface SketchpadCommands {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
}
export interface SketchpadCommandsFull {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
  register(command: string, callback: (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult): Disposable;
}
export interface SketchpadStore extends SketchpadSnapshot, SketchpadCommands, SketchpadChildStores {}
export interface SketchpadStoreFull extends SketchpadSnapshot, SketchpadCommands, SketchpadChildStoresFull, SketchpadActions {
  on: SketchpadSubscriptions;
}

// #endregion Api

// #region Stores

// Base class for common functionality
abstract class BaseYStore<TYObject extends Y.AbstractType<any>> {
  protected abstract yObject: TYObject;

  protected createObserver = (subscribe: Subscribe, deep?: boolean): Unsubscribe => {
    return createObserver(this.yObject, subscribe, deep);
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    return this.createObserver(subscribe, deep);
  };
}

type YAuthor = Y.Map<string>;
type YAuthors = Y.Map<YAuthor>;
type YAttribute = Y.Map<string>;
type YAttributes = Y.Array<YAttribute>;
type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;
type YVec3 = YLeafMapNumber;
type YPlane = Y.Map<YVec3>;

type YRepresentationVal = string | YStringArray | YAttributes;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentationMap = Y.Map<YRepresentation>;
type YRepresentationId = string;

type YPortVal = string | number | boolean | YLeafMapNumber | YAttributes | YStringArray;
type YPort = Y.Map<YPortVal>;
type YPortMap = Y.Map<YPort>;
type YPortId = string;

type YPieceVal = string | YLeafMapString | YLeafMapNumber | YPlane | YAttributes;
type YPiece = Y.Map<YPieceVal>;
type YPieceMap = Y.Map<YPiece>;
type YPieceId = string;

type YSide = Y.Map<YLeafMapString>;
type YConnectionVal = string | number | YAttributes | YSide;
type YConnection = Y.Map<YConnectionVal>;
type YConnectionMap = Y.Map<YConnection>;
type YConnectionId = string;

type YTypeVal = string | number | boolean | YAuthors | YAttributes | YRepresentationMap | YPortMap;
type YType = Y.Map<YTypeVal>;
type YTypeMap = Y.Map<YType>;
type YTypeId = string;

type YDesignVal = string | YAuthors | YAttributes | YPieceMap | YConnectionMap;
type YDesign = Y.Map<YDesignVal>;
type YDesignMap = Y.Map<YDesign>;
type YDesignId = string;

type YDesignEditorStoreVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignEditorStoreValMap = Y.Map<YDesignEditorStoreVal>;
type YDesignEditorStoreMap = Y.Map<YDesignEditorStore>;

type YIdMap = Y.Map<string>;
type YKitVal = string | YTypeMap | YDesignMap | YIdMap | YAttributes;
type YKit = Y.Map<YKitVal>;
type YKitMap = Y.Map<YKit>;
type YKitId = string;

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

// Generic observer helper
function createObserver(yObject: Y.AbstractType<any>, subscribe: Subscribe, deep?: boolean): Unsubscribe {
  const observer = () => subscribe();
  if (deep) {
    yObject.observeDeep(observer);
    return () => {
      try {
        yObject.unobserveDeep(observer);
      } catch (e) {
        // Y object no longer attached, ignore unobserve error
      }
    };
  } else {
    yObject.observe(observer);
    return () => {
      try {
        yObject.unobserve(observer);
      } catch (e) {
        // Y object no longer attached, ignore unobserve error
      }
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

class YFileStore implements FileStoreFull {
  public readonly parent: YKitStore;
  public readonly yFile: Y.Map<string>;
  private cachedSnapshot?: SemioFile;
  private lastSnapshotHash?: string;

  constructor(parent: YKitStore, file: SemioFile) {
    this.parent = parent;
    this.yFile = new Y.Map<string>();
    this.yFile.set("path", file.path);
    this.yFile.set("remote", file.remote || "");
    this.yFile.set("size", file.size?.toString() || "");
    this.yFile.set("hash", file.hash || "");
    this.yFile.set("created", file.created?.toISOString() || "");
    this.yFile.set("updated", file.updated?.toISOString() || "");
  }

  get file(): SemioFile {
    const path = this.yFile.get("path") as string;
    const remote = this.yFile.get("remote") as string;
    const file: SemioFile = { path, remote: remote || undefined };
    const size = this.yFile.get("size");
    if (size) file.size = parseInt(size as string);
    const hash = this.yFile.get("hash");
    if (hash) file.hash = hash as string;
    const created = this.yFile.get("created");
    if (created) file.created = new Date(created as string);
    const updated = this.yFile.get("updated");
    if (updated) file.updated = new Date(updated as string);
    return file;
  }

  change = (diff: FileDiff) => {
    if (diff.path !== undefined) this.yFile.set("path", diff.path);
    if (diff.remote !== undefined) this.yFile.set("remote", diff.remote);
    if (diff.size !== undefined) this.yFile.set("size", diff.size.toString());
    if (diff.hash !== undefined) this.yFile.set("hash", diff.hash);
    this.cachedSnapshot = undefined; // Invalidate cache when data changes
    this.lastSnapshotHash = undefined; // Invalidate hash when data changes
  };

  snapshot = (): SemioFile => {
    const currentData = {
      path: this.yFile.get("path") as string,
      remote: (this.yFile.get("remote") as string) || undefined,
      size: this.yFile.get("size") ? parseInt(this.yFile.get("size") as string) : undefined,
      hash: (this.yFile.get("hash") as string) || undefined,
      created: this.yFile.get("created") ? new Date(this.yFile.get("created") as string) : undefined,
      updated: this.yFile.get("updated") ? new Date(this.yFile.get("updated") as string) : undefined,
    };
    const currentHash = JSON.stringify({
      path: currentData.path,
      remote: currentData.remote,
      size: currentData.size,
      hash: currentData.hash,
      created: currentData.created?.toISOString(),
      updated: currentData.updated?.toISOString(),
    });

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yFile, subscribe, deep);
  };
}

class YRepresentationStore implements RepresentationStoreFull {
  public readonly parent: YKitStore;
  public readonly yRepresentation: YRepresentation;
  private cachedSnapshot?: Representation;
  private lastSnapshotHash?: string;

  constructor(parent: YKitStore, representation: Representation) {
    this.parent = parent;
    this.yRepresentation = new Y.Map<any>();
    this.yRepresentation.set("url", representation.url);
    this.yRepresentation.set("description", representation.description || "");
    this.yRepresentation.set("tags", createStringArray(representation.tags || []));
    this.yRepresentation.set("attributes", createAttributes(representation.attributes));
  }

  snapshot = (): Representation => {
    const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
    const yAttributes = this.yRepresentation.get("attributes") as YAttributes;
    
    const attributes = getAttributes(yAttributes);
    const url = this.yRepresentation.get("url") as string;
    const description = (this.yRepresentation.get("description") as string) || "";
    const tags = yTags ? yTags.toArray() : [];

    const currentData = {
      url: url,
      description: description,
      tags: tags,
      attributes: attributes,
    };
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: RepresentationDiff) => {
    if (diff.url !== undefined) this.yRepresentation.set("url", diff.url);
    if (diff.description !== undefined) this.yRepresentation.set("description", diff.description);
    if (diff.tags !== undefined) {
      updateStringArray(this.yRepresentation.get("tags") as Y.Array<string>, diff.tags);
    }
    if (diff.attributes !== undefined) {
      updateAttributes(this.yRepresentation.get("attributes") as YAttributes, diff.attributes);
    }
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yRepresentation, subscribe, deep);
  };
}

class YPortStore implements PortStoreFull {
  public readonly parent: YTypeStore;
  public readonly yPort: YPort;
  private cachedSnapshot?: Port;
  private lastSnapshotHash?: string;

  constructor(parent: YTypeStore, port: Port) {
    this.parent = parent;
    this.yPort = new Y.Map<any>();
    this.yPort.set("id_", port.id_ || "");
    this.yPort.set("description", port.description || "");
    this.yPort.set("mandatory", port.mandatory || false);
    this.yPort.set("family", port.family || "");
    this.yPort.set("t", port.t);
    this.yPort.set("compatibleFamilies", createStringArray(port.compatibleFamilies || []));
    this.yPort.set("point", createVec3(port.point));
    this.yPort.set("direction", createVec3(port.direction));
    this.yPort.set("attributes", createAttributes(port.attributes));
  }

  snapshot = (): Port => {
    const yCompatibleFamilies = this.yPort.get("compatibleFamilies") as Y.Array<string>;
    const yPoint = this.yPort.get("point") as Y.Map<number>;
    const yDirection = this.yPort.get("direction") as Y.Map<number>;
    const yAttributes = this.yPort.get("attributes") as YAttributes;

    const attributes: Attribute[] = [];
    if (yAttributes) {
      try {
        yAttributes.forEach((yMap: YAttribute) => {
          attributes.push({
            key: yMap.get("key") as string,
            value: yMap.get("value") as string | undefined,
            definition: yMap.get("definition") as string | undefined,
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    const currentData = {
      id_: this.yPort.get("id_") as string,
      description: (this.yPort.get("description") as string) || "",
      mandatory: this.yPort.get("mandatory") as boolean,
      family: (this.yPort.get("family") as string) || "",
      compatibleFamilies: yCompatibleFamilies ? yCompatibleFamilies.toArray() : [],
      point: {
        x: yPoint.get("x") as number,
        y: yPoint.get("y") as number,
        z: yPoint.get("z") as number,
      },
      direction: {
        x: yDirection.get("x") as number,
        y: yDirection.get("y") as number,
        z: yDirection.get("z") as number,
      },
      t: this.yPort.get("t") as number,
      attributes: attributes,
    };
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: PortDiff) => {
    if (diff.id_ !== undefined) this.yPort.set("id_", diff.id_);
    if (diff.description !== undefined) this.yPort.set("description", diff.description);
    if (diff.mandatory !== undefined) this.yPort.set("mandatory", diff.mandatory);
    if (diff.family !== undefined) this.yPort.set("family", diff.family);
    if (diff.t !== undefined) this.yPort.set("t", diff.t);
    if (diff.compatibleFamilies !== undefined) {
      updateStringArray(this.yPort.get("compatibleFamilies") as Y.Array<string>, diff.compatibleFamilies);
    }
    if (diff.point !== undefined) {
      updateVec3(this.yPort.get("point") as Y.Map<number>, diff.point);
    }
    if (diff.direction !== undefined) {
      updateVec3(this.yPort.get("direction") as Y.Map<number>, diff.direction);
    }
    if (diff.attributes !== undefined) {
      updateAttributes(this.yPort.get("attributes") as YAttributes, diff.attributes);
    }
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yPort, subscribe, deep);
  };
}

class YTypeStore implements TypeStoreFull {
  public readonly parent: YKitStore;
  public readonly yType: YType;
  public readonly representations: Map<string, YRepresentationStore> = new Map();
  public readonly ports: Map<string, YPortStore> = new Map();
  public readonly representationIds: Map<string, string> = new Map();
  public readonly portIds: Map<string, string> = new Map();
  private cachedSnapshot?: Type;
  private lastSnapshotHash?: string;

  constructor(parent: YKitStore, type: Type) {
    this.parent = parent;
    this.yType = new Y.Map<any>();
    this.yType.set("name", type.name);
    this.yType.set("description", type.description || "");
    this.yType.set("variant", type.variant || "");
    this.yType.set("unit", type.unit);
    this.yType.set("stock", type.stock || Number.POSITIVE_INFINITY);
    this.yType.set("virtual", type.virtual || false);
    this.yType.set("representations", new Y.Map() as YRepresentationMap);
    this.yType.set("ports", new Y.Map() as YPortMap);
    this.yType.set("authors", createAuthors(type.authors));
    this.yType.set("attributes", createAttributes(type.attributes));
  }

  type = (): Type => {
    return this.snapshot();
  };

  change = (diff: TypeDiff) => {
    if (diff.name !== undefined) this.yType.set("name", diff.name);
    if (diff.description !== undefined) this.yType.set("description", diff.description);
    if (diff.variant !== undefined) this.yType.set("variant", diff.variant);
    if (diff.unit !== undefined) this.yType.set("unit", diff.unit);
    if (diff.stock !== undefined) this.yType.set("stock", diff.stock);
    if (diff.virtual !== undefined) this.yType.set("virtual", diff.virtual);
    if (diff.representations !== undefined) {
      diff.representations.forEach((rep) => {
        const repId = representationIdLikeToRepresentationId(rep);
        const repIdStr = representationIdToString(repId);
        if (!this.representationIds.get(repIdStr)) {
          const uuid = uuidv4();
          this.representationIds.set(repIdStr, uuid);
          this.representations.set(repIdStr, new YRepresentationStore(this.parent, rep));
          (this.yType.get("representations") as YRepresentationMap).set(uuid, this.representations.get(repIdStr)!.yRepresentation);
        } else {
          this.representations.get(repIdStr)?.change(rep);
        }
      });
    }
    if (diff.ports !== undefined) {
      diff.ports.forEach((port) => {
        const portId = portIdLikeToPortId(port);
        const portIdStr = portIdToString(portId);
        if (!this.portIds.get(portIdStr)) {
          const uuid = uuidv4();
          this.portIds.set(portIdStr, uuid);
          this.ports.set(portIdStr, new YPortStore(this, port));
          (this.yType.get("ports") as YPortMap).set(uuid, this.ports.get(portIdStr)!.yPort);
        } else {
          this.ports.get(portIdStr)?.change(port);
        }
      });
    }
  };

  snapshot = (): Type => {
    const yAuthors = this.yType.get("authors") as YAuthors;
    const yAttributes = this.yType.get("attributes") as YAttributes;
    
    const authors = getAuthors(yAuthors);
    const attributes = getAttributes(yAttributes);
    
    const name = this.yType.get("name") as string;
    const description = (this.yType.get("description") as string) || "";
    const variant = this.yType.get("variant") as string | undefined;
    const unit = (this.yType.get("unit") as string) || "";
    const stock = this.yType.get("stock") as number | undefined;
    const virtual = this.yType.get("virtual") as boolean | undefined;

    const currentData = {
      name: name,
      description: description,
      variant: variant,
      unit: unit,
      stock: stock,
      virtual: virtual,
      representations: Array.from(this.representations.values()).map((store) => store.snapshot()),
      ports: Array.from(this.ports.values()).map((store) => store.snapshot()),
      authors: authors,
      attributes: attributes,
    };
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yType, subscribe, deep);
  };
}

class YPieceStore implements PieceStoreFull {
  public readonly parent: YDesignStore;
  public readonly yPiece: YPiece;
  private cachedSnapshot?: Piece;
  private lastSnapshotHash?: string;

  constructor(parent: YDesignStore, piece: Piece) {
    this.parent = parent;
    this.yPiece = new Y.Map<any>();
    try {
      this.yPiece.set("id_", piece.id_);
      this.yPiece.set("description", piece.description || "");
      if (piece.type) {
        const yType = new Y.Map<string>();
        yType.set("name", piece.type.name);
        if (piece.type.variant) yType.set("variant", piece.type.variant);
        this.yPiece.set("type", yType);
      } else {
        const yDesign = new Y.Map<string>();
        yDesign.set("name", piece.design?.name || "");
        if (piece.design?.variant) yDesign.set("variant", piece.design.variant);
        if (piece.design?.view) yDesign.set("view", piece.design.view);
        this.yPiece.set("design", yDesign);
      }
      if (piece.plane) {
        this.yPiece.set("plane", createPlane(piece.plane));
      }
      if (piece.center) {
        this.yPiece.set("center", createVec2(piece.center));
      }
      this.yPiece.set("attributes", createAttributes(piece.attributes));
    } catch (e) {
      // Y object not attached to document during construction, but this is expected
    }
  }

  snapshot = (): Piece => {
    const yType = this.yPiece.get("type") as Y.Map<string>;
    const yPlane = this.yPiece.get("plane") as Y.Map<any> | undefined;
    const yCenter = this.yPiece.get("center") as Y.Map<number> | undefined;
    const yAttributes = this.yPiece.get("attributes") as YAttributes;

    const attributes: Attribute[] = [];
    if (yAttributes) {
      try {
        yAttributes.forEach((yMap: YAttribute) => {
          attributes.push({
            key: yMap.get("key") as string,
            value: yMap.get("value") as string | undefined,
            definition: yMap.get("definition") as string | undefined,
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    const currentData: Piece = {
      id_: this.yPiece.get("id_") as string,
      description: (this.yPiece.get("description") as string) || "",
      type: {
        name: yType.get("name") as string,
        variant: yType.get("variant") as string | undefined,
      },
      attributes: attributes,
    };

    if (yPlane) {
      const yOrigin = yPlane.get("origin") as Y.Map<number>;
      const yXAxis = yPlane.get("xAxis") as Y.Map<number>;
      const yYAxis = yPlane.get("yAxis") as Y.Map<number>;
      currentData.plane = {
        origin: {
          x: yOrigin.get("x") as number,
          y: yOrigin.get("y") as number,
          z: yOrigin.get("z") as number,
        },
        xAxis: {
          x: yXAxis.get("x") as number,
          y: yXAxis.get("y") as number,
          z: yXAxis.get("z") as number,
        },
        yAxis: {
          x: yYAxis.get("x") as number,
          y: yYAxis.get("y") as number,
          z: yYAxis.get("z") as number,
        },
      };
    }

    if (yCenter) {
      currentData.center = {
        x: yCenter.get("x") as number,
        y: yCenter.get("y") as number,
      };
    }

    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: PieceDiff) => {
    if (diff.id_ !== undefined) this.yPiece.set("id_", diff.id_);
    if (diff.description !== undefined) this.yPiece.set("description", diff.description);
    if (diff.type !== undefined) {
      const yType = this.yPiece.get("type") as Y.Map<string>;
      yType.set("name", diff.type.name);
      if (diff.type.variant !== undefined) yType.set("variant", diff.type.variant);
    }
    if (diff.plane !== undefined) {
      const yPlane = this.yPiece.get("plane") as Y.Map<any>;
      if (yPlane) {
        if (diff.plane.origin !== undefined) {
          updateVec3(yPlane.get("origin") as Y.Map<number>, diff.plane.origin);
        }
        if (diff.plane.xAxis !== undefined) {
          updateVec3(yPlane.get("xAxis") as Y.Map<number>, diff.plane.xAxis);
        }
        if (diff.plane.yAxis !== undefined) {
          updateVec3(yPlane.get("yAxis") as Y.Map<number>, diff.plane.yAxis);
        }
      }
    }
    if (diff.center !== undefined) {
      const yCenter = this.yPiece.get("center") as Y.Map<number>;
      if (yCenter) {
        updateVec2(yCenter, diff.center);
      }
    }
    if (diff.attributes !== undefined) {
      updateAttributes(this.yPiece.get("attributes") as YAttributes, diff.attributes);
    }
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yPiece, subscribe, deep);
  };
}

class YConnectionStore implements ConnectionStoreFull {
  public readonly parent: YDesignStore;
  public readonly yConnection: YConnection;
  private cachedSnapshot?: Connection;
  private lastSnapshotHash?: string;

  constructor(parent: YDesignStore, connection: Connection) {
    this.parent = parent;
    this.yConnection = new Y.Map<any>();
    try {
      const yConnected = new Y.Map<any>();
      const yConnectedPiece = new Y.Map<string>();
      yConnectedPiece.set("id_", connection.connected.piece.id_);
      yConnected.set("piece", yConnectedPiece);
      const yConnectedPort = new Y.Map<string>();
      yConnectedPort.set("id_", connection.connected.port.id_ || "");
      yConnected.set("port", yConnectedPort);
      if (connection.connected.designPiece) {
        const yConnectedDesignPiece = new Y.Map<string>();
        yConnectedDesignPiece.set("id_", connection.connected.designPiece.id_);
        yConnected.set("designPiece", yConnectedDesignPiece);
      }
      this.yConnection.set("connected", yConnected);
      const yConnecting = new Y.Map<any>();
      const yConnectingPiece = new Y.Map<string>();
      yConnectingPiece.set("id_", connection.connecting.piece.id_);
      yConnecting.set("piece", yConnectingPiece);
      const yConnectingPort = new Y.Map<string>();
      yConnectingPort.set("id_", connection.connecting.port.id_ || "");
      yConnecting.set("port", yConnectingPort);
      if (connection.connecting.designPiece) {
        const yConnectingDesignPiece = new Y.Map<string>();
        yConnectingDesignPiece.set("id_", connection.connecting.designPiece.id_);
        yConnecting.set("designPiece", yConnectingDesignPiece);
      }
      this.yConnection.set("connecting", yConnecting);
      this.yConnection.set("description", connection.description || "");
      this.yConnection.set("gap", connection.gap || 0);
      this.yConnection.set("shift", connection.shift || 0);
      this.yConnection.set("rise", connection.rise || 0);
      this.yConnection.set("rotation", connection.rotation || 0);
      this.yConnection.set("turn", connection.turn || 0);
      this.yConnection.set("tilt", connection.tilt || 0);
      this.yConnection.set("x", connection.x || 0);
      this.yConnection.set("y", connection.y || 0);
      this.yConnection.set("attributes", createAttributes(connection.attributes));
    } catch (e) {
      // Y object not attached to document during construction, but this is expected
    }
  }

  snapshot = (): Connection => {
    const yConnected = this.yConnection.get("connected") as Y.Map<any>;
    const yConnecting = this.yConnection.get("connecting") as Y.Map<any>;
    const yConnectedPiece = yConnected.get("piece") as Y.Map<string>;
    const yConnectedDesignPiece = yConnected.get("designPiece") as Y.Map<string> | undefined;
    const yConnectedPort = yConnected.get("port") as Y.Map<string>;
    const yConnectingPiece = yConnecting.get("piece") as Y.Map<string>;
    const yConnectingDesignPiece = yConnecting.get("designPiece") as Y.Map<string> | undefined;
    const yConnectingPort = yConnecting.get("port") as Y.Map<string>;

    const yAttributes = this.yConnection.get("attributes") as YAttributes;
    const attributes: Attribute[] = [];
    if (yAttributes) {
      try {
        yAttributes.forEach((yMap: YAttribute) => {
          attributes.push({
            key: yMap.get("key") as string,
            value: yMap.get("value") as string | undefined,
            definition: yMap.get("definition") as string | undefined,
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    const currentData = {
      connected: {
        piece: { id_: yConnectedPiece.get("id_") as string },
        port: { id_: yConnectedPort.get("id_") as string },
        designPiece: yConnectedDesignPiece ? { id_: yConnectedDesignPiece.get("id_") as string } : undefined,
      },
      connecting: {
        piece: { id_: yConnectingPiece.get("id_") as string },
        port: { id_: yConnectingPort.get("id_") as string },
        designPiece: yConnectingDesignPiece ? { id_: yConnectingDesignPiece.get("id_") as string } : undefined,
      },
      description: (this.yConnection.get("description") as string) || "",
      gap: this.yConnection.get("gap") as number,
      shift: this.yConnection.get("shift") as number,
      rise: this.yConnection.get("rise") as number,
      rotation: this.yConnection.get("rotation") as number,
      turn: this.yConnection.get("turn") as number,
      tilt: this.yConnection.get("tilt") as number,
      x: this.yConnection.get("x") as number,
      y: this.yConnection.get("y") as number,
      attributes: attributes,
    };
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: ConnectionDiff) => {
    if (diff.description !== undefined) this.yConnection.set("description", diff.description);
    if (diff.gap !== undefined) this.yConnection.set("gap", diff.gap);
    if (diff.shift !== undefined) this.yConnection.set("shift", diff.shift);
    if (diff.rise !== undefined) this.yConnection.set("rise", diff.rise);
    if (diff.rotation !== undefined) this.yConnection.set("rotation", diff.rotation);
    if (diff.turn !== undefined) this.yConnection.set("turn", diff.turn);
    if (diff.tilt !== undefined) this.yConnection.set("tilt", diff.tilt);
    if (diff.x !== undefined) this.yConnection.set("x", diff.x);
    if (diff.y !== undefined) this.yConnection.set("y", diff.y);
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    return createObserver(this.yConnection, subscribe, deep);
  };
}

class YDesignStore implements DesignStoreFull {
  public readonly parent: YKitStore;
  public readonly yDesign: YDesign;
  public readonly pieces: Map<string, YPieceStore> = new Map();
  public readonly connections: Map<string, YConnectionStore> = new Map();
  public readonly pieceIds: Map<string, string> = new Map();
  public readonly connectionIds: Map<string, string> = new Map();
  private cachedSnapshot?: Design;
  private lastSnapshotHash?: string;

  constructor(parent: YKitStore, design: Design) {
    this.parent = parent;
    this.yDesign = new Y.Map<any>();
    try {
      this.yDesign.set("name", design.name);
      this.yDesign.set("description", design.description || "");
      this.yDesign.set("variant", design.variant || "");
      this.yDesign.set("view", design.view || "");
      this.yDesign.set("unit", design.unit);
      this.yDesign.set("pieces", new Y.Map() as YPieceMap);
      this.yDesign.set("connections", new Y.Map() as YConnectionMap);
      this.yDesign.set("authors", createAuthors(design.authors));
      this.yDesign.set("attributes", createAttributes(design.attributes));
    } catch (e) {
      // Y object not attached to document during construction, but this is expected
    }
  }

  snapshot = (): Design => {
    let yAuthors: YAuthors | undefined;
    let yAttributes: YAttributes | undefined;
    
    try {
      yAuthors = this.yDesign.get("authors") as YAuthors;
      yAttributes = this.yDesign.get("attributes") as YAttributes;
    } catch (e) {
      // Y object not yet attached to document, use empty defaults
      yAuthors = undefined;
      yAttributes = undefined;
    }

    const authors: Author[] = [];
    if (yAuthors) {
      try {
        yAuthors.forEach((yAuthor: YAuthor) => {
          authors.push({
            name: yAuthor.get("name") as string,
            email: (yAuthor.get("email") as string) || "",
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    const attributes: Attribute[] = [];
    if (yAttributes) {
      try {
        yAttributes.forEach((yMap: YAttribute) => {
          attributes.push({
            key: yMap.get("key") as string,
            value: yMap.get("value") as string | undefined,
            definition: yMap.get("definition") as string | undefined,
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    let name = "";
    let description: string | undefined;
    let variant: string | undefined;
    let view: string | undefined;
    let unit = "";
    
    try {
      name = this.yDesign.get("name") as string;
      description = this.yDesign.get("description") as string | undefined;
      variant = this.yDesign.get("variant") as string | undefined;
      view = this.yDesign.get("view") as string | undefined;
      unit = this.yDesign.get("unit") as string;
    } catch (e) {
      // Y object not properly attached, use defaults
    }

    const currentData = {
      name: name,
      description: description,
      variant: variant,
      view: view,
      unit: unit,
      pieces: Array.from(this.pieces.values()).map((p) => p.snapshot()),
      connections: Array.from(this.connections.values()).map((c) => c.snapshot()),
      authors: authors,
      attributes: attributes,
    };
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: DesignDiff) => {
    if (diff.name !== undefined) this.yDesign.set("name", diff.name);
    if (diff.description !== undefined) this.yDesign.set("description", diff.description);
    if (diff.variant !== undefined) this.yDesign.set("variant", diff.variant);
    if (diff.view !== undefined) this.yDesign.set("view", diff.view);
    if (diff.unit !== undefined) this.yDesign.set("unit", diff.unit);

    if (diff.pieces) {
      if (diff.pieces.added) {
        diff.pieces.added.forEach((piece) => {
          const pieceId = pieceIdLikeToPieceId(piece);
          const pieceIdStr = pieceIdToString(pieceId);
          if (!this.pieceIds.get(pieceIdStr)) {
            const uuid = uuidv4();
            const yPieceStore = new YPieceStore(this, piece);
            (this.yDesign.get("pieces") as YPieceMap).set(uuid, yPieceStore.yPiece);
            this.pieceIds.set(pieceIdStr, uuid);
            this.pieces.set(pieceIdStr, yPieceStore);
          }
        });
      }
      if (diff.pieces.removed) {
        diff.pieces.removed.forEach((pieceId) => {
          const id = pieceIdLikeToPieceId(pieceId);
          const idStr = pieceIdToString(id);
          const uuid = this.pieceIds.get(idStr);
          if (uuid) {
            (this.yDesign.get("pieces") as YPieceMap).delete(uuid);
            this.pieceIds.delete(idStr);
            this.pieces.delete(idStr);
          }
        });
      }
      if (diff.pieces.updated) {
        diff.pieces.updated.forEach((updatedItem) => {
          const matchingPiece = Array.from(this.pieces.entries()).find(([_, store]) => {
            const piece = store.snapshot();
            return !updatedItem.diff.id_ || piece.id_ === updatedItem.id.id_;
          });
          if (matchingPiece) {
            matchingPiece[1].change(updatedItem.diff);
          }
        });
      }
    }

    if (diff.connections) {
      if (diff.connections.added) {
        diff.connections.added.forEach((connection) => {
          const connectionId = connectionIdLikeToConnectionId(connection);
          const connectionIdStr = connectionIdToString(connectionId);
          if (!this.connectionIds.get(connectionIdStr)) {
            const uuid = uuidv4();
            const yConnectionStore = new YConnectionStore(this, connection);
            (this.yDesign.get("connections") as YConnectionMap).set(uuid, yConnectionStore.yConnection);
            this.connectionIds.set(connectionIdStr, uuid);
            this.connections.set(connectionIdStr, yConnectionStore);
          }
        });
      }
      if (diff.connections.removed) {
        diff.connections.removed.forEach((connectionId) => {
          const id = connectionIdLikeToConnectionId(connectionId);
          const idStr = connectionIdToString(id);
          const uuid = this.connectionIds.get(idStr);
          if (uuid) {
            (this.yDesign.get("connections") as YConnectionMap).delete(uuid);
            this.connectionIds.delete(idStr);
            this.connections.delete(idStr);
          }
        });
      }
      if (diff.connections.updated) {
        diff.connections.updated.forEach((updatedItem) => {
          const matchingConnection = Array.from(this.connections.entries()).find(([_, store]) => {
            const connection = store.snapshot();
            return connection.connected.piece.id_ === updatedItem.id.connected.piece.id_ && connection.connecting.piece.id_ === updatedItem.id.connecting.piece.id_;
          });
          if (matchingConnection) {
            matchingConnection[1].change(updatedItem.diff);
          }
        });
      }
    }
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    try {
      if (deep) {
        this.yDesign.observeDeep(observer);
        return () => {
          try {
            this.yDesign.unobserveDeep(observer);
          } catch (e) {
            // Y object not available during unobserve
          }
        };
      } else {
        this.yDesign.observe(observer);
        return () => {
          try {
            this.yDesign.unobserve(observer);
          } catch (e) {
            // Y object not available during unobserve
          }
        };
      }
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };
}

class YKitStore implements KitStoreFull {
  public readonly yKit: YKit;
  public readonly types: Map<string, YTypeStore> = new Map();
  public readonly designs: Map<string, YDesignStore> = new Map();
  public readonly files: Map<Url, YFileStore> = new Map();
  private readonly typeIds: Map<string, string> = new Map();
  private readonly designIds: Map<string, string> = new Map();
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult> = new Map();
  private readonly regularFiles: Map<Url, string> = new Map();
  private readonly parent: SketchpadStore;
  private cachedSnapshot?: Kit;
  private lastSnapshotHash?: string;

  constructor(parent: SketchpadStore, kit: Kit) {
    this.parent = parent;

    // Create new Y.Doc for this kit
    const yDoc = new Y.Doc();
    this.yKit = yDoc.getMap("kit") as YKit;

    // Set initial values synchronously to ensure proper Y.js integration
    this.yKit.set("uuid", uuidv4());
    this.yKit.set("uri", kit.uri);
    this.yKit.set("name", kit.name);
    this.yKit.set("description", kit.description || "");
    this.yKit.set("icon", kit.icon || "");
    this.yKit.set("image", kit.image || "");
    this.yKit.set("version", kit.version || "");
    this.yKit.set("preview", kit.preview || "");
    this.yKit.set("remote", kit.remote || "");
    this.yKit.set("homepage", kit.homepage || "");
    this.yKit.set("license", kit.license || "");
    this.yKit.set("types", new Y.Map<YType>());
    this.yKit.set("designs", new Y.Map<YDesign>());
    this.yKit.set("attributes", new Y.Array<YAttribute>());
    this.yKit.set("created", new Date().toISOString());
    this.yKit.set("updated", new Date().toISOString());

    try {
      kit.types?.forEach((type) => {
        const typeId = typeIdLikeToTypeId(type);
        const typeIdStr = typeIdToString(typeId);
        if (this.typeIds.has(typeIdStr)) {
          throw new Error(`Type (${typeId.name}, ${typeId.variant || ""}) already exists.`);
        }
        const uuid = uuidv4();
        const yTypeStore = new YTypeStore(this, type);
        (this.yKit.get("types") as YTypeMap).set(uuid, yTypeStore.yType);
        const representationsMap = yTypeStore.yType.get("representations") as YRepresentationMap;
        const portsMap = yTypeStore.yType.get("ports") as YPortMap;
        (type.representations || []).forEach((r) => {
          const repId = representationIdLikeToRepresentationId(r);
          const repIdStr = representationIdToString(repId);
          const repUuid = uuidv4();
          yTypeStore.representationIds.set(repIdStr, repUuid);
          yTypeStore.representations.set(repIdStr, new YRepresentationStore(this, r));
          representationsMap.set(repUuid, yTypeStore.representations.get(repIdStr)!.yRepresentation);
        });
        (type.ports || []).forEach((p) => {
          const portId = portIdLikeToPortId(p);
          const portIdStr = portIdToString(portId);
          const portUuid = uuidv4();
          yTypeStore.portIds.set(portIdStr, portUuid);
          yTypeStore.ports.set(portIdStr, new YPortStore(yTypeStore, p));
          portsMap.set(portUuid, yTypeStore.ports.get(portIdStr)!.yPort);
        });
        this.typeIds.set(typeIdStr, uuid);
        this.types.set(typeIdStr, yTypeStore);
      });
    } catch (e) {
      // Y.Map operations during type processing
    }
    try {
      kit.designs?.forEach((design) => {
        const designId = designIdLikeToDesignId(design);
        const designIdStr = designIdToString(designId);
        if (this.designIds.has(designIdStr)) {
          throw new Error(`Design (${designId.name}, ${designId.variant || ""}) already exists.`);
        }
        const uuid = uuidv4();
        const yDesignStore = new YDesignStore(this, design);
        (this.yKit.get("designs") as YDesignMap).set(uuid, yDesignStore.yDesign);
        const piecesMap = yDesignStore.yDesign.get("pieces") as YPieceMap;
        const connectionsMap = yDesignStore.yDesign.get("connections") as YConnectionMap;
        (design.pieces || []).forEach((piece) => {
          const pieceId = pieceIdLikeToPieceId(piece);
          const pieceUuid = uuidv4();
          const yPieceStore = new YPieceStore(yDesignStore, piece);
          piecesMap.set(pieceUuid, yPieceStore.yPiece);
          const pieceIdStr = pieceIdToString(pieceId);
          yDesignStore.pieceIds.set(pieceIdStr, pieceUuid);
          yDesignStore.pieces.set(pieceIdStr, yPieceStore);
        });
        (design.connections || []).forEach((connection) => {
          const connectionId = connectionIdLikeToConnectionId(connection);
          const connectionUuid = uuidv4();
          const yConnectionStore = new YConnectionStore(yDesignStore, connection);
          connectionsMap.set(connectionUuid, yConnectionStore.yConnection);
          const connectionIdStr = connectionIdToString(connectionId);
          yDesignStore.connectionIds.set(connectionIdStr, connectionUuid);
          yDesignStore.connections.set(connectionIdStr, yConnectionStore);
        });
        this.designIds.set(designIdStr, uuid);
        this.designs.set(designIdStr, yDesignStore);
      });
    } catch (e) {
      // Y.Map operations during design processing
    }

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  snapshot = (): Kit => {
    const yAttributes = this.yKit.get("attributes") as YAttributes;
    const attributes: Attribute[] = [];
    if (yAttributes) {
      try {
        yAttributes.forEach((yMap: YAttribute) => {
          attributes.push({
            key: yMap.get("key") as string,
            value: yMap.get("value") as string | undefined,
            definition: yMap.get("definition") as string | undefined,
          });
        });
      } catch (e) {
        // Y object not properly attached, skip
      }
    }

    let currentData;
    try {
      currentData = {
        uri: this.yKit.get("uri") as string,
        name: this.yKit.get("name") as string,
        version: this.yKit.get("version") as string | undefined,
        description: this.yKit.get("description") as string | undefined,
        icon: this.yKit.get("icon") as string | undefined,
        image: this.yKit.get("image") as string | undefined,
        preview: this.yKit.get("preview") as string | undefined,
        remote: this.yKit.get("remote") as string | undefined,
        homepage: this.yKit.get("homepage") as string | undefined,
        license: this.yKit.get("license") as string | undefined,
        created: this.yKit.get("created") ? new Date(this.yKit.get("created") as string) : undefined,
        updated: this.yKit.get("updated") ? new Date(this.yKit.get("updated") as string) : undefined,
        types: Array.from(this.types.values()).map((store) => store.snapshot()),
        designs: Array.from(this.designs.values()).map((store) => store.snapshot()),
        attributes: attributes,
      };
    } catch (e) {
      // Y.Map not available, use fallback data
      currentData = {
        uri: "",
        name: "",
        version: undefined,
        description: undefined,
        icon: undefined,
        image: undefined,
        preview: undefined,
        remote: undefined,
        homepage: undefined,
        license: undefined,
        created: undefined,
        updated: undefined,
        types: [],
        designs: [],
        attributes: attributes,
      };
    }
    const currentHash = JSON.stringify(currentData);

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = currentData;
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: KitDiff) => {
    try {
      if (diff.uri) this.yKit.set("uri", diff.uri);
      if (diff.name) this.yKit.set("name", diff.name);
      if (diff.description) this.yKit.set("description", diff.description);
      if (diff.version) this.yKit.set("version", diff.version);
      if (diff.icon) this.yKit.set("icon", diff.icon);
      if (diff.image) this.yKit.set("image", diff.image);
      if (diff.preview) this.yKit.set("preview", diff.preview);
      if (diff.remote) this.yKit.set("remote", diff.remote);
      if (diff.homepage) this.yKit.set("homepage", diff.homepage);
      if (diff.license) this.yKit.set("license", diff.license);
    } catch (e) {
      // Y.Map operations for basic properties not available
    }

    if (diff.types) {
      try {
        if (diff.types.added) {
          diff.types.added.forEach((type) => {
            const typeId = typeIdLikeToTypeId(type);
            const typeIdStr = typeIdToString(typeId);
            if (!this.typeIds.get(typeIdStr)) {
              const uuid = uuidv4();
              const yTypeStore = new YTypeStore(this, type);
              (this.yKit.get("types") as YTypeMap).set(uuid, yTypeStore.yType);
            const representationsMap = yTypeStore.yType.get("representations") as YRepresentationMap;
            const portsMap = yTypeStore.yType.get("ports") as YPortMap;
            (type.representations || []).forEach((r) => {
              const repId = representationIdLikeToRepresentationId(r);
              const repIdStr = representationIdToString(repId);
              const repUuid = uuidv4();
              yTypeStore.representationIds.set(repIdStr, repUuid);
              yTypeStore.representations.set(repIdStr, new YRepresentationStore(this, r));
              representationsMap.set(repUuid, yTypeStore.representations.get(repIdStr)!.yRepresentation);
            });
            (type.ports || []).forEach((p) => {
              const portId = portIdLikeToPortId(p);
              const portIdStr = portIdToString(portId);
              const portUuid = uuidv4();
              yTypeStore.portIds.set(portIdStr, portUuid);
              yTypeStore.ports.set(portIdStr, new YPortStore(yTypeStore, p));
              portsMap.set(portUuid, yTypeStore.ports.get(portIdStr)!.yPort);
            });
            this.typeIds.set(typeIdStr, uuid);
            this.types.set(typeIdStr, yTypeStore);
          }
        });
      }
      if (diff.types.removed) {
        diff.types.removed.forEach((typeId) => {
          const id = typeIdLikeToTypeId(typeId);
          const idStr = typeIdToString(id);
          const uuid = this.typeIds.get(idStr);
          if (uuid) {
            (this.yKit.get("types") as YTypeMap).delete(uuid);
            this.typeIds.delete(idStr);
            this.types.delete(idStr);
          }
        });
      }
      if (diff.types.updated) {
        diff.types.updated.forEach((updatedItem) => {
          const matchingType = Array.from(this.types.entries()).find(([_, store]) => {
            const type = store.snapshot();
            return (!updatedItem.diff.name || type.name === updatedItem.id.name) && (!updatedItem.diff.variant || type.variant === updatedItem.id.variant);
          });
          if (matchingType) {
            matchingType[1].change(updatedItem.diff);
          }
        });
        }
      } catch (e) {
        // Y.Map operations for types processing not available
      }
    }

    if (diff.designs) {
      try {
        if (diff.designs.added) {
          diff.designs.added.forEach((design) => {
            const designId = designIdLikeToDesignId(design);
            const designIdStr = designIdToString(designId);
            if (!this.designIds.get(designIdStr)) {
              const uuid = uuidv4();
              const yDesignStore = new YDesignStore(this, design);
              (this.yKit.get("designs") as YDesignMap).set(uuid, yDesignStore.yDesign);
              const piecesMap = yDesignStore.yDesign.get("pieces") as YPieceMap;
              const connectionsMap = yDesignStore.yDesign.get("connections") as YConnectionMap;
              (design.pieces || []).forEach((piece) => {
                const pieceId = pieceIdLikeToPieceId(piece);
                const pieceUuid = uuidv4();
                const yPieceStore = new YPieceStore(yDesignStore, piece);
                piecesMap.set(pieceUuid, yPieceStore.yPiece);
                const pieceIdStr = pieceIdToString(pieceId);
                yDesignStore.pieceIds.set(pieceIdStr, pieceUuid);
                yDesignStore.pieces.set(pieceIdStr, yPieceStore);
              });
              (design.connections || []).forEach((connection) => {
                const connectionId = connectionIdLikeToConnectionId(connection);
                const connectionUuid = uuidv4();
                const yConnectionStore = new YConnectionStore(yDesignStore, connection);
                connectionsMap.set(connectionUuid, yConnectionStore.yConnection);
                const connectionIdStr = connectionIdToString(connectionId);
                yDesignStore.connectionIds.set(connectionIdStr, connectionUuid);
                yDesignStore.connections.set(connectionIdStr, yConnectionStore);
              });
              this.designIds.set(designIdStr, uuid);
              this.designs.set(designIdStr, yDesignStore);
            }
          });
        }
        if (diff.designs.removed) {
          diff.designs.removed.forEach((designId) => {
            const id = designIdLikeToDesignId(designId);
            const idStr = designIdToString(id);
            const uuid = this.designIds.get(idStr);
            if (uuid) {
              (this.yKit.get("designs") as YDesignMap).delete(uuid);
              this.designIds.delete(idStr);
              this.designs.delete(idStr);
            }
          });
        }
        if (diff.designs.updated) {
          diff.designs.updated.forEach((updatedItem) => {
            const matchingDesign = Array.from(this.designs.entries()).find(([_, store]) => {
              const design = store.snapshot();
              return (!updatedItem.diff.name || design.name === updatedItem.id.name) && (!updatedItem.diff.variant || design.variant === updatedItem.id.variant) && (!updatedItem.diff.view || design.view === updatedItem.id.view);
            });
            if (matchingDesign) {
              matchingDesign[1].change(updatedItem.diff);
            }
          });
        }
      } catch (e) {
        // Y.Map operations for designs processing not available
      }
    }

    if (diff.files) {
      if (diff.files.added) {
        diff.files.added.forEach((file) => {
          const fileId = fileIdLikeToFileId(file);
          const yFileStore = new YFileStore(this, file);
          this.files.set(fileId.path, yFileStore);
        });
      }
      if (diff.files.removed) {
        diff.files.removed.forEach((fileId) => {
          const normalizedFileId = fileIdLikeToFileId(fileId);
          this.files.delete(normalizedFileId.path);
          this.regularFiles.delete(normalizedFileId.path);
        });
      }
      if (diff.files.updated) {
        diff.files.updated.forEach((update) => {
          const fileId = fileIdLikeToFileId(update.id);
          const yFileStore = this.files.get(fileId.path);
          if (yFileStore) {
            yFileStore.change(update.diff);
          }
        });
      }
    }

    try {
      this.yKit.set("updated", new Date().toISOString());
    } catch (e) {
      // Y.Map operation for updated timestamp not available
    }
  };

  fileUrls = (): Map<Url, Url> => {
    return this.regularFiles;
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    try {
      deep ? (this.yKit as unknown as Y.Map<any>).observeDeep(observer) : (this.yKit as unknown as Y.Map<any>).observe(observer);
      return () => {
        try {
          deep ? (this.yKit as unknown as Y.Map<any>).unobserveDeep(observer) : (this.yKit as unknown as Y.Map<any>).unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
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

class YDesignEditorStore implements DesignEditorStoreFull {
  public readonly yDesignEditorStore: YDesignEditorStoreValMap;
  private readonly commandRegistry: Map<string, (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult> = new Map();
  private readonly parent: SketchpadStore;
  private cachedSnapshot?: DesignEditorStateFull;
  private lastSnapshotHash?: string;

  constructor(parent: SketchpadStore, state?: DesignEditorState) {
    this.parent = parent;
    this.yDesignEditorStore = new Y.Map<YDesignEditorStoreVal>();
    try {
      this.yDesignEditorStore.set("fullscreenPanel", state?.fullscreenPanel || DesignEditorFullscreenPanel.None);
      this.yDesignEditorStore.set("selectedPieceIds", new Y.Array<string>());
      this.yDesignEditorStore.set("selectedConnections", new Y.Array<string>());
      this.yDesignEditorStore.set("selectedPiecePortPieceId", "");
      this.yDesignEditorStore.set("selectedPiecePortPortId", "");
      this.yDesignEditorStore.set("selectedPiecePortDesignPieceId", "");
      this.yDesignEditorStore.set("isTransactionActive", false);
      this.yDesignEditorStore.set("presence", new Y.Map<any>());
      this.yDesignEditorStore.set("others", new Y.Array<any>());
      this.yDesignEditorStore.set("diff", new Y.Map<any>());
      this.yDesignEditorStore.set("currentTransactionStack", new Y.Array<any>());
      this.yDesignEditorStore.set("pastTransactionsStack", new Y.Array<any>());
    } catch (e) {
      // Y object not attached to document during construction, but this is expected
    }

    Object.entries(designEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenPanel(): DesignEditorFullscreenPanel {
    try {
      return (this.yDesignEditorStore.get("fullscreenPanel") as DesignEditorFullscreenPanel) || DesignEditorFullscreenPanel.None;
    } catch (e) {
      return DesignEditorFullscreenPanel.None;
    }
  }
  get selection(): DesignEditorSelection {
    try {
      const selectedPieceIds = this.yDesignEditorStore.get("selectedPieceIds") as Y.Array<string>;
      const selectedConnections = this.yDesignEditorStore.get("selectedConnections") as Y.Array<string>;
      const selectedPiecePortPieceId = this.yDesignEditorStore.get("selectedPiecePortPieceId") as string;
      const selectedPiecePortPortId = this.yDesignEditorStore.get("selectedPiecePortPortId") as string;
      const selectedPiecePortDesignPieceId = this.yDesignEditorStore.get("selectedPiecePortDesignPieceId") as string;
      
      const port = selectedPiecePortPieceId && selectedPiecePortPortId 
        ? {
            piece: { id_: selectedPiecePortPieceId },
            port: { id_: selectedPiecePortPortId },
            ...(selectedPiecePortDesignPieceId && { designPiece: { id_: selectedPiecePortDesignPieceId } }),
          }
        : undefined;
      
      return {
        pieces: selectedPieceIds && selectedPieceIds.toArray ? selectedPieceIds.toArray().map((id) => ({ id_: id })) : [],
        connections: selectedConnections && selectedConnections.toArray
          ? selectedConnections.toArray().map((id) => ({
              connected: { piece: { id_: id.split("->")[0] || "" } },
              connecting: { piece: { id_: id.split("->")[1] || "" } },
            }))
          : [],
        port,
      };
    } catch (e) {
      // Y object not properly attached, return empty selection
      return {
        pieces: [],
        connections: [],
        port: undefined,
      };
    }
  }
  get designDiff(): DesignDiff {
    return {};
  }
  get isTransactionActive(): boolean {
    try {
      return (this.yDesignEditorStore.get("isTransactionActive") as boolean) || false;
    } catch (e) {
      return false;
    }
  }
  get presence(): DesignEditorPresence {
    try {
      return {
        cursor: {
          x: (this.yDesignEditorStore.get("presenceCursorX") as number) || 0,
          y: (this.yDesignEditorStore.get("presenceCursorY") as number) || 0,
        },
      };
    } catch (e) {
      return {
        cursor: { x: 0, y: 0 },
      };
    }
  }
  get others(): DesignEditorPresenceOther[] {
    return [];
  }
  get diff(): KitDiff {
    return {};
  }
  get currentTransactionStack(): DesignEditorEdit[] {
    try {
      const yStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
      return yStack ? yStack.toArray() : [];
    } catch (e) {
      // Y object not properly attached, return empty array
      return [];
    }
  }
  get pastTransactionsStack(): DesignEditorEdit[] {
    try {
      const yStack = this.yDesignEditorStore.get("pastTransactionsStack") as Y.Array<any>;
      return yStack ? yStack.toArray() : [];
    } catch (e) {
      // Y object not properly attached, return empty array
      return [];
    }
  }

  snapshot = (): DesignEditorStateFull => {
    const currentHash = JSON.stringify({
      fullscreenPanel: this.fullscreenPanel,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      presence: this.presence,
      others: this.others,
      diff: this.diff,
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
    });

    if (!this.cachedSnapshot || this.lastSnapshotHash !== currentHash) {
      this.cachedSnapshot = {
        fullscreenPanel: this.fullscreenPanel,
        selection: this.selection,
        isTransactionActive: this.isTransactionActive,
        presence: this.presence,
        others: this.others,
        diff: this.diff,
        currentTransactionStack: this.currentTransactionStack,
        pastTransactionsStack: this.pastTransactionsStack,
      };
      this.lastSnapshotHash = currentHash;
    }

    return this.cachedSnapshot;
  };

  change = (diff: DesignEditorStateDiff) => {
    if (diff.fullscreenPanel) this.yDesignEditorStore.set("fullscreenPanel", diff.fullscreenPanel);
    if (diff.selection) {
      if (diff.selection.pieces) {
        let yPieceIds = this.yDesignEditorStore.get("selectedPieceIds") as Y.Array<string>;
        if (!yPieceIds) {
          yPieceIds = new Y.Array<string>();
          this.yDesignEditorStore.set("selectedPieceIds", yPieceIds);
        }
        
        let currentIds: Set<string>;
        try {
          currentIds = new Set(yPieceIds.toArray());
        } catch (e) {
          currentIds = new Set();
        }
        
        if (diff.selection.pieces.removed) {
          diff.selection.pieces.removed.forEach((piece) => {
            try {
              const currentArray = yPieceIds.toArray();
              const index = currentArray.indexOf(piece.id_);
              if (index >= 0) {
                yPieceIds.delete(index, 1);
              }
            } catch (e) {
              // Y object not properly attached, skip
            }
          });
        }
        
        if (diff.selection.pieces.added) {
          const newIds = diff.selection.pieces.added
            .filter((piece) => !currentIds.has(piece.id_))
            .map((piece) => piece.id_);
          if (newIds.length > 0) {
            try {
              yPieceIds.insert(yPieceIds.length, newIds);
            } catch (e) {
              // Y object not properly attached, skip insertion
            }
          }
        }
      }
      
      if (diff.selection.connections) {
        let yConnectionIds = this.yDesignEditorStore.get("selectedConnections") as Y.Array<string>;
        if (!yConnectionIds) {
          yConnectionIds = new Y.Array<string>();
          this.yDesignEditorStore.set("selectedConnections", yConnectionIds);
        }
        
        let currentIds: Set<string>;
        try {
          currentIds = new Set(yConnectionIds.toArray());
        } catch (e) {
          currentIds = new Set();
        }
        
        if (diff.selection.connections.removed) {
          diff.selection.connections.removed.forEach((conn) => {
            const connStr = `${conn.connected.piece.id_}->${conn.connecting.piece.id_}`;
            try {
              const currentArray = yConnectionIds.toArray();
              const index = currentArray.indexOf(connStr);
              if (index >= 0) {
                yConnectionIds.delete(index, 1);
              }
            } catch (e) {
              // Y object not properly attached, skip
            }
          });
        }
        
        if (diff.selection.connections.added) {
          const newIds = diff.selection.connections.added
            .filter((conn) => {
              const connStr = `${conn.connected.piece.id_}->${conn.connecting.piece.id_}`;
              return !currentIds.has(connStr);
            })
            .map((conn) => `${conn.connected.piece.id_}->${conn.connecting.piece.id_}`);
          if (newIds.length > 0) {
            try {
              yConnectionIds.insert(yConnectionIds.length, newIds);
            } catch (e) {
              // Y object not properly attached, skip insertion
            }
          }
        }
      }
      
      if (diff.selection.port) {
        if (diff.selection.port.piece && diff.selection.port.port) {
          this.yDesignEditorStore.set("selectedPiecePortPieceId", diff.selection.port.piece.id_);
          this.yDesignEditorStore.set("selectedPiecePortPortId", diff.selection.port.port.id_ || "");
          if (diff.selection.port.designPiece) {
            this.yDesignEditorStore.set("selectedPiecePortDesignPieceId", diff.selection.port.designPiece.id_);
          }
        } else {
          this.yDesignEditorStore.set("selectedPiecePortPieceId", "");
          this.yDesignEditorStore.set("selectedPiecePortPortId", "");
          this.yDesignEditorStore.set("selectedPiecePortDesignPieceId", "");
        }
      }
    }
    if (diff.presence) {
      if (diff.presence.cursor) {
        this.yDesignEditorStore.set("presenceCursorX", diff.presence.cursor.x);
        this.yDesignEditorStore.set("presenceCursorY", diff.presence.cursor.y);
      }
    }
    if (diff.others) {
      // Others are typically managed by collaboration providers, not stored in Yjs directly
    }
  };

  onChanged = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    try {
      deep ? (this.yDesignEditorStore as unknown as Y.Map<any>).observeDeep(observer) : (this.yDesignEditorStore as unknown as Y.Map<any>).observe(observer);
      return () => {
        try {
          deep ? (this.yDesignEditorStore as unknown as Y.Map<any>).unobserveDeep(observer) : (this.yDesignEditorStore as unknown as Y.Map<any>).unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };

  startTransaction = () => {
    this.yDesignEditorStore.set("isTransactionActive", true);
  };

  onTransactionStarted = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    try {
      this.yDesignEditorStore.observe(observer);
      return () => {
        try {
          this.yDesignEditorStore.unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };

  abortTransaction = () => {
    if (this.isTransactionActive) {
      const currentStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
      if (currentStack) {
        currentStack.delete(0, currentStack.length);
      }
      this.yDesignEditorStore.set("isTransactionActive", false);
    }
  };

  onTransactionAborted = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    try {
      this.yDesignEditorStore.observe(observer);
      return () => {
        try {
          this.yDesignEditorStore.unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };

  finalizeTransaction = () => {
    if (this.isTransactionActive) {
      try {
        const currentStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
        const pastStack = this.yDesignEditorStore.get("pastTransactionsStack") as Y.Array<any>;
        if (currentStack && pastStack && currentStack.length > 0) {
          pastStack.push(currentStack.toArray());
          currentStack.delete(0, currentStack.length);
        }
        this.yDesignEditorStore.set("isTransactionActive", false);
      } catch (e) {
        // Y object not properly attached, just set transaction as inactive
        try {
          this.yDesignEditorStore.set("isTransactionActive", false);
        } catch (e2) {
          // Even setting the flag failed, Y object completely unattached
        }
      }
    }
  };

  onTransactionFinalized = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    try {
      this.yDesignEditorStore.observe(observer);
      return () => {
        try {
          this.yDesignEditorStore.unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };

  undo = () => {
    try {
      if (this.isTransactionActive) {
        const currentStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
        if (currentStack && currentStack.length > 0) {
          const edit = currentStack.get(currentStack.length - 1);
          currentStack.delete(currentStack.length - 1, 1);
          if (edit && edit.undo) {
            edit.undo.diff && this.change(edit.undo.diff);
          }
        }
      } else {
        const pastStack = this.yDesignEditorStore.get("pastTransactionsStack") as Y.Array<any>;
        if (pastStack && pastStack.length > 0) {
          const edit = pastStack.get(pastStack.length - 1);
          pastStack.delete(pastStack.length - 1, 1);
          if (edit && edit.undo) {
            edit.undo.diff && this.change(edit.undo.diff);
          }
        }
      }
    } catch (e) {
      // Y object not properly attached, skip undo operation
    }
  };
  onUndone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    try {
      this.yDesignEditorStore.observe(observer);
      return () => {
        try {
          this.yDesignEditorStore.unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
  };
  redo = () => {
    try {
      if (this.isTransactionActive) {
        const currentStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
        if (currentStack && currentStack.length > 0) {
          const edit = currentStack.get(0);
          currentStack.delete(0, 1);
          if (edit && edit.do) {
            edit.do.diff && this.change(edit.do.diff);
          }
        }
      } else {
        const pastStack = this.yDesignEditorStore.get("pastTransactionsStack") as Y.Array<any>;
        if (pastStack && pastStack.length > 0) {
          const edit = pastStack.get(0);
          pastStack.delete(0, 1);
          if (edit && edit.do) {
            edit.do.diff && this.change(edit.do.diff);
          }
        }
      }
    } catch (e) {
      // Y object not properly attached, skip redo operation
    }
  };
  onRedone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    try {
      this.yDesignEditorStore.observe(observer);
      return () => {
        try {
          this.yDesignEditorStore.unobserve(observer);
        } catch (e) {
          // Y object not available during unobserve
        }
      };
    } catch (e) {
      // Y object not available during observe
      return () => {};
    }
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

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design editor store`);
    const parent = this.parent as YSketchpadStore;
    const kitStore = parent.kits.get(kitIdToString(parent.activeDesignEditor!.kit))!;

    const beforeState = this.snapshot();

    const context: DesignEditorCommandContext = {
      designEditor: this.snapshot(),
      kit: kitStore.snapshot(),
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
      const currentStack = this.yDesignEditorStore.get("currentTransactionStack") as Y.Array<any>;
      const edit: DesignEditorEdit = {
        do: {
          diff: result.kitDiff,
          selection: this.snapshot().selection,
        },
        undo: {
          diff: undefined,
          selection: beforeState.selection,
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

  get on(): DesignEditorSubscriptions {
    return {
      onUndone: this.onUndone,
      onRedone: this.onRedone,
      onChanged: this.onChanged,
      onTransactionStarted: this.onTransactionStarted,
      onTransactionAborted: this.onTransactionAborted,
      onTransactionFinalized: this.onTransactionFinalized,
    };
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }
}

class YSketchpadStore implements SketchpadStoreFull {
  public readonly ySketchpadDoc: Y.Doc;
  public readonly sketchpadIndexeddbProvider?: IndexeddbPersistence;
  public readonly yKitDocs: Map<string, Y.Doc> = new Map();
  public readonly kitIndexeddbProviders: Map<string, IndexeddbPersistence> = new Map();
  public readonly kits: Map<string, YKitStore> = new Map();
  public readonly designEditors: Map<string, Map<string, DesignEditorStoreFull>> = new Map();
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult> = new Map();
  private cachedSnapshot?: SketchpadStateFull;
  private lastSnapshotValues?: { mode: Mode; theme: Theme; layout: Layout; activeDesignEditor?: DesignEditorId };

  private getYSketchpad(): YSketchpad {
    return this.ySketchpadDoc.getMap("sketchpad");
  }

  constructor(state: SketchpadStateFull) {
    console.log('🔧 YSketchpadStore constructor starting:', { persistantId: state.persistantId });
    
    this.ySketchpadDoc = new Y.Doc();
    console.log('🔧 Y.Doc created');
    
    // Initialize the sketchpad map immediately and ensure it's properly integrated
    const ySketchpad = this.ySketchpadDoc.getMap("sketchpad");
    console.log('🔧 Y.Map retrieved from document');
    
    // Set initial values synchronously to ensure proper Y.js integration
    try {
      ySketchpad.set("mode", state.mode);
      ySketchpad.set("theme", state.theme);
      ySketchpad.set("layout", state.layout);
      if (state.activeDesignEditor) {
        ySketchpad.set("activeDesignEditor", JSON.stringify(state.activeDesignEditor));
      }
      console.log('🔧 Initial values set successfully');
    } catch (error) {
      console.error('🚨 Error setting initial values:', error);
      throw error;
    }
    
    // Initialize IndexedDB persistence AFTER the document structure is set up
    if (state.persistantId && state.persistantId !== "") {
      try {
        this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad-${state.persistantId}`, this.ySketchpadDoc);
        console.log('🔧 IndexedDB persistence initialized');
      } catch (error) {
        console.error('🚨 Error initializing IndexedDB persistence:', error);
        throw error;
      }
    }
    
    console.log('🔧 YSketchpadStore constructor completed');
  }

  get mode(): Mode {
    try {
      return (this.getYSketchpad().get("mode") as Mode) || Mode.GUEST;
    } catch (e) {
      return Mode.GUEST;
    }
  }
  get theme(): Theme {
    try {
      return (this.getYSketchpad().get("theme") as Theme) || Theme.SYSTEM;
    } catch (e) {
      return Theme.SYSTEM;
    }
  }
  get layout(): Layout {
    try {
      return (this.getYSketchpad().get("layout") as Layout) || Layout.NORMAL;
    } catch (e) {
      return Layout.NORMAL;
    }
  }
  get activeDesignEditor(): DesignEditorId | undefined {
    try {
      const designEditorIdStr = this.getYSketchpad().get("activeDesignEditor");
      if (!designEditorIdStr || typeof designEditorIdStr !== "string") return undefined;
      return JSON.parse(designEditorIdStr) as DesignEditorId;
    } catch (e) {
      return undefined;
    }
  }

  snapshot = (): SketchpadStateFull => {
    const currentValues = {
      mode: this.mode,
      theme: this.theme,
      layout: this.layout,
      activeDesignEditor: this.activeDesignEditor,
    };

    if (
      !this.lastSnapshotValues ||
      this.lastSnapshotValues.mode !== currentValues.mode ||
      this.lastSnapshotValues.theme !== currentValues.theme ||
      this.lastSnapshotValues.layout !== currentValues.layout ||
      this.lastSnapshotValues.activeDesignEditor !== currentValues.activeDesignEditor
    ) {
      this.cachedSnapshot = currentValues;
      this.lastSnapshotValues = currentValues;
    }

    return this.cachedSnapshot!;
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

  change(diff: SketchpadStateDiff) {
    try {
      if (diff.mode) this.getYSketchpad().set("mode", diff.mode);
      if (diff.theme) this.getYSketchpad().set("theme", diff.theme);
      if (diff.layout) this.getYSketchpad().set("layout", diff.layout);
      if (diff.activeDesignEditor) this.getYSketchpad().set("activeDesignEditor", JSON.stringify(diff.activeDesignEditor));
    } catch (e) {
      // Y.Map not available during operation
    }
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
    try {
      // Observe design editor stores map changes
      const yDesignEditorStores = this.ySketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
      const observer = () => subscribe();
      yDesignEditorStores.observe(observer);
      return () => {
        try {
          yDesignEditorStores.unobserve(observer);
        } catch (e) {
          // Y.Map not available during unobserve
        }
      };
    } catch (e) {
      // Y.Map not available during observe
      return () => {};
    }
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
    try {
      const observer = () => subscribe();
      this.getYSketchpad().observe(observer);
      return () => {
        try {
          this.getYSketchpad().unobserve(observer);
        } catch (e) {
          // Y.Map not available during unobserve
        }
      };
    } catch (e) {
      // Y.Map not available during observe
      return () => {};
    }
  };

  get on(): SketchpadSubscriptions {
    return {
      onKitCreated: this.onKitCreated,
      onDesignEditorCreated: this.onDesignEditorCreated,
      onChanged: this.onChanged,
      onKitDeleted: this.onKitDeleted,
      onDesignEditorDeleted: this.onDesignEditorDeleted,
    };
  }

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in sketchpad store`);
    if (command === "semio.sketchpad.createKit") {
      const kit = rest[0] as Kit;
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
    const context: SketchpadCommandContext = {
      sketchpad: this.snapshot(),
      store: this,
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
    const initialState: SketchpadStateFull = {
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
  "semio.sketchpad.createKit": (context: SketchpadCommandContext, kit: Kit): SketchpadCommandResult => {
    if (!kit.name) throw new Error("Kit name is required to create a kit.");

    const kitId = kitIdLikeToKitId(kit);
    const kitIdStr = kitIdToString(kitId);
    if (context.store.kits.has(kitIdStr)) {
      throw new Error(`Kit (${kitId.name}, ${kitId.version || ""}) already exists.`);
    }

    context.store.createKit(kit);

    return {};
  },
  "semio.sketchpad.createDesignEditor": (context: SketchpadCommandContext, id: DesignEditorId): SketchpadCommandResult => {
    return {
      diff: {},
    };
  },
  "semio.sketchpad.setActiveDesignEditor": (context: SketchpadCommandContext, id: DesignEditorId): SketchpadCommandResult => {
    return {
      diff: { activeDesignEditor: id },
    };
  },
  "semio.sketchpad.importKit": (context: SketchpadCommandContext, kitId: KitId, url: string): SketchpadCommandResult => {
    return {
      diff: {},
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
            } catch (error) {
              console.warn(`Failed to fetch file ${file.path}:`, error);
            }
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
        CREATE TABLE kit ( uri VARCHAR(2048) NOT NULL UNIQUE, name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, preview VARCHAR(1024) NOT NULL, version VARCHAR(64) NOT NULL, remote VARCHAR(1024) NOT NULL, homepage VARCHAR(1024) NOT NULL, license VARCHAR(1024) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY );
        CREATE TABLE type ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE design ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, "view" VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, UNIQUE (name, variant, "view", kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
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

        const kitStmt = db.prepare("INSERT INTO kit (uri, name, description, icon, image, preview, version, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
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
          const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, variant, unit, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
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
                  } catch (error) {
                    console.warn(`Failed to fetch file ${rep.url}:`, error);
                  }
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
  "semio.kit.deleteSelected": (context: KitCommandContext, designId: DesignId, selectedPieces: PieceId[], selectedConnections: ConnectionId[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: designId,
              diff: { pieces: { removed: selectedPieces }, connections: { removed: selectedConnections } },
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
  "semio.designEditor.startTransaction": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.finalizeTransaction": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.abortTransaction": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    return { diff: {} };
  },
  "semio.designEditor.setDesign": (context: DesignEditorCommandContext, design: Design): DesignEditorCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [{ id: context.designId, diff: { name: design.name, description: design.description } }],
        },
      },
    };
  },
  "semio.designEditor.setPiece": (context: DesignEditorCommandContext, piece: Piece): DesignEditorCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { updated: [{ id: { id_: piece.id_ }, diff: piece }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.setPieces": (context: DesignEditorCommandContext, pieces: Piece[]): DesignEditorCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { updated: pieces.map((p) => ({ id: { id_: p.id_ }, diff: p })) } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.setConnection": (context: DesignEditorCommandContext, connection: Connection): DesignEditorCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { updated: [{ id: { connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } }, diff: connection }] } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.setConnections": (context: DesignEditorCommandContext, connections: Connection[]): DesignEditorCommandResult => {
    return {
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { connections: { updated: connections.map((c) => ({ id: { connected: { piece: c.connected.piece }, connecting: { piece: c.connecting.piece } }, diff: c })) } },
            },
          ],
        },
      },
    };
  },
  "semio.designEditor.addPiece": (context: DesignEditorCommandContext, piece: Piece): DesignEditorCommandResult => {
    return {
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
};

// #endregion Commands

// #region Hooks

// #region Scoping

type SketchpadScope = { id: string; persisted: boolean };
type KitScope = { id: KitId };
type DesignScope = { id: DesignId };
type TypeScope = { id: TypeId };
type PieceScope = { id: PieceId };
type ConnectionScope = { id: ConnectionId };
type RepresentationScope = { id: RepresentationId };
type PortypeScope = { id: PortId };
type DesignEditorScope = { id: string };

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
const KitScopeContext = createContext<KitScope | null>(null);
const DesignScopeContext = createContext<DesignScope | null>(null);
const TypeScopeContext = createContext<TypeScope | null>(null);
const PieceScopeContext = createContext<PieceScope | null>(null);
const ConnectionScopeContext = createContext<ConnectionScope | null>(null);
const RepresentationScopeContext = createContext<RepresentationScope | null>(null);
const PortypeScopeContext = createContext<PortypeScope | null>(null);
const DesignEditorScopeContext = createContext<DesignEditorScope | null>(null);

export const SketchpadScopeProvider = (props: { id: string; persisted: boolean; children: React.ReactNode }) => {
  const value = useMemo(() => {
    const scope = { id: props.id, persisted: props.persisted };
    // Ensure store is created/initialized
    getOrCreateSketchpadStore(props.id, props.persisted);
    return scope;
  }, [props.id, props.persisted]);
  return React.createElement(SketchpadScopeContext.Provider, { value }, props.children as any);
};

export const KitScopeProvider = (props: { id: KitId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};

export const DesignScopeProvider = (props: { id: DesignId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(DesignScopeContext.Provider, { value }, props.children as any);
};

export const TypeScopeProvider = (props: { id: TypeId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(TypeScopeContext.Provider, { value }, props.children as any);
};

export const PieceScopeProvider = (props: { id: PieceId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(PieceScopeContext.Provider, { value }, props.children as any);
};

export const ConnectionScopeProvider = (props: { id: ConnectionId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(ConnectionScopeContext.Provider, { value }, props.children as any);
};

export const RepresentationScopeProvider = (props: { id: RepresentationId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(RepresentationScopeContext.Provider, { value }, props.children as any);
};

export const PortypeScopeProvider = (props: { id: PortId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(PortypeScopeContext.Provider, { value }, props.children as any);
};

export const DesignEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(DesignEditorScopeContext.Provider, { value }, props.children as any);
};

const useSketchpadScope = () => useContext(SketchpadScopeContext);
const useKitStoreScope = () => useContext(KitScopeContext);
const useDesignScope = () => useContext(DesignScopeContext);
const useTypeScope = () => useContext(TypeScopeContext);
const usePieceScope = () => useContext(PieceScopeContext);
const useConnectionScope = () => useContext(ConnectionScopeContext);
const useRepresentationScope = () => useContext(RepresentationScopeContext);
const usePortScope = () => useContext(PortypeScopeContext);
const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

// #endregion Scoping
export function useSketchpad(): SketchpadStore;
export function useSketchpad<T>(selector: (store: SketchpadStore) => T, id?: string): T;
export function useSketchpad<T>(selector?: (store: SketchpadStore) => T, id?: string): T | SketchpadStore {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = useMemo(() => stores.get(storeId)!, [storeId]);
  const lastSnapshot = useRef<any>(null);
  const getSnapshot = useCallback(() => {
    const currentSnapshot = store.snapshot();
    if (!lastSnapshot.current || JSON.stringify(lastSnapshot.current) !== JSON.stringify(currentSnapshot)) {
      lastSnapshot.current = currentSnapshot;
    }
    return lastSnapshot.current;
  }, [store]);

  const state = useSyncExternalStore(store.onChanged, getSnapshot, getSnapshot);

  return selector ? selector(store) : store;
}

export function useDesignEditor(): DesignEditorStore;
export function useDesignEditor<T>(selector: (store: DesignEditorStore) => T): T;
export function useDesignEditor<T>(selector: (store: DesignEditorStore) => T, kitId: KitId, designId: DesignId): T;
export function useDesignEditor<T>(selector?: (store: DesignEditorStore) => T, kitId?: KitId, designId?: DesignId): T | DesignEditorStore {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesignEditor must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;

  const kitScope = useKitStoreScope();
  const resolvedKitId = kitScope?.id ?? kitId;
  const designScope = useDesignScope();
  const resolvedDesignId = designScope?.id ?? designId;

  if (!resolvedKitId) throw new Error("useDesignEditor must be called within a KitScopeProvider or be directly provided with a kitId");
  if (!resolvedDesignId) throw new Error("useDesignEditor must be called within a DesignScopeProvider or be directly provided with a designId");

  const resolvedKitIdStr = kitIdToString(resolvedKitId);
  const resolvedDesignIdStr = designIdToString(resolvedDesignId);
  if (!store.designEditors.has(resolvedKitIdStr)) throw new Error(`Design editor store not found for kit ${resolvedKitId.name}`);
  const kitEditors = store.designEditors.get(resolvedKitIdStr)!;
  if (!kitEditors.has(resolvedDesignIdStr)) throw new Error(`Design editor store not found for design ${resolvedDesignId.name}`);
  const designEditor = useMemo(() => kitEditors.get(resolvedDesignIdStr)!, [kitEditors, resolvedDesignIdStr]);
  const getSnapshot = useMemo(() => () => designEditor.snapshot(), [designEditor]);

  const state = useSyncExternalStore(designEditor.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(designEditor) : designEditor;
}

export function useKitStore(): KitStore;
export function useKitStore<T>(selector: (store: KitStore) => T): T;
export function useKitStore<T>(selector: (store: KitStore) => T, id: KitId): T;
export function useKitStore<T>(selector?: (store: KitStore) => T, id?: KitId): T | KitStore {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKitStore must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  const kitId = kitScope?.id ?? id;
  if (!kitId) throw new Error("useKitStore must be called within a KitScopeProvider or be directly provided with an id");
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitIdStr)!;
  return selector ? selector(kitStore) : kitStore;
}

export function useKit(): Kit;
export function useKit<T>(selector: (kit: Kit) => T): T;
export function useKit<T>(selector: (kit: Kit) => T, id: KitId): T;
export function useKit<T>(selector?: (kit: Kit) => T, id?: KitId): T | Kit {
  console.log('🔍 useKit called');
  const kitStore = useKitStore();
  console.log('🔍 useKit: got kitStore');
  const getSnapshot = useMemo(() => () => {
    console.log('🔍 useKit: calling kitStore.snapshot()');
    const result = kitStore.snapshot();
    console.log('🔍 useKit: snapshot result:', result ? `${result.name} v${result.version}` : 'null');
    return result;
  }, [kitStore]);
  console.log('🔍 useKit: about to call useSyncExternalStore');
  const kit = useSyncExternalStore(kitStore.onChanged, getSnapshot, getSnapshot);
  console.log('🔍 useKit: useSyncExternalStore completed');
  return selector ? selector(kit as Kit) : (kit as Kit);
}

export function useDesign(): Design;
export function useDesign<T>(selector: (design: Design) => T): T;
export function useDesign<T>(selector: (design: Design) => T, id: DesignId): T;
export function useDesign<T>(selector?: (design: Design) => T, id?: DesignId): T | Design {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesign must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("useDesign must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitIdStr)!;
  const designScope = useDesignScope();
  const designId = designScope?.id ?? id;
  if (!designId) throw new Error("useDesign must be called within a DesignScopeProvider or be directly provided with an id");
  const designIdStr = designIdToString(designId);
  if (!kitStore.designs.has(designIdStr)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = useMemo(() => kitStore.designs.get(designIdStr)!, [kitStore, designIdStr]);
  const getSnapshot = useMemo(() => () => designStore.snapshot(), [designStore]);
  const state = useSyncExternalStore(designStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

export function useFlattenDiff(): DesignDiff {
  const designScope = useDesignScope();
  const kit = useKit();
  if (!designScope) throw new Error("useFlattenDiff must be called within a DesignScopeProvider");
  return useMemo(() => flattenDesign(kit, designScope.id), [kit, designScope.id]);
}

export function useFlatDesign(): Design {
  const design = useDesign();
  const diff = useFlattenDiff();
  return useMemo(() => applyDesignDiff(design, diff, true), [design, diff]);
}

export function useFlatPieces(): Piece[] {
  const design = useFlatDesign();
  return design.pieces ?? [];
}

export function usePiecesMetadata(): Map<
  string,
  {
    plane: Plane;
    center: DiagramPoint;
    fixedPieceId: string;
    parentPieceId: string | null;
    depth: number;
  }
> {
  const kit = useKit();
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePiecesMetadata must be called within a DesignScopeProvider");
  return useMemo(() => piecesMetadata(kit, designScope.id), [kit, designScope.id]);
}

export function useType(): Type;
export function useType<T>(selector: (type: Type) => T): T;
export function useType<T>(selector: (type: Type) => T, id: TypeId): T;
export function useType<T>(selector?: (type: Type) => T, id?: TypeId): T | Type {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useType must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("useType must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitIdStr)!;
  const typeScope = useTypeScope();
  const typeId = typeScope?.id ?? id;
  if (!typeId) throw new Error("useType must be called within a TypeScopeProvider or be directly provided with an id");
  const typeIdStr = typeIdToString(typeId);
  if (!kit.types.has(typeIdStr)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = useMemo(() => kit.types.get(typeIdStr)!, [kit, typeIdStr]);
  const getSnapshot = useMemo(() => () => typeStore.snapshot(), [typeStore]);
  const state = useSyncExternalStore(typeStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

export function usePiece(): Piece;
export function usePiece<T>(selector: (piece: Piece) => T): T;
export function usePiece<T>(selector: (piece: Piece) => T, id: PieceId): T;
export function usePiece<T>(selector?: (piece: Piece) => T, id?: PieceId): T | Piece {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePiece must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("usePiece must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitIdStr)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePiece must be called within a DesignScopeProvider");
  const designId = designScope.id;
  const designIdStr = designIdToString(designId);
  if (!kit.designs.has(designIdStr)) throw new Error(`Design store not found for design ${designId}`);
  const design = kit.designs.get(designIdStr)!;
  const pieceScope = usePieceScope();
  const pieceId = pieceScope?.id ?? id;
  if (!pieceId) throw new Error("usePiece must be called within a PieceScopeProvider or be directly provided with an id");
  const pieceIdStr = pieceIdToString(pieceId);
  if (!design.pieces.has(pieceIdStr)) throw new Error(`Piece store not found for piece ${pieceId}`);
  const pieceStore = useMemo(() => design.pieces.get(pieceIdStr)!, [design, pieceIdStr]);
  const getSnapshot = useMemo(() => () => pieceStore.snapshot(), [pieceStore]);
  const state = useSyncExternalStore(pieceStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

export function useConnection(): Connection;
export function useConnection<T>(selector: (connection: Connection) => T): T;
export function useConnection<T>(selector: (connection: Connection) => T, id: ConnectionId): T;
export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId): T | Connection {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useConnection must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("useConnection must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitIdStr)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useConnection must be called within a DesignScopeProvider");
  const designId = designScope.id;
  const designIdStr = designIdToString(designId);
  if (!kit.designs.has(designIdStr)) throw new Error(`Design store not found for design ${designId}`);
  const design = kit.designs.get(designIdStr)!;
  const connectionScope = useConnectionScope();
  const connectionId = connectionScope?.id ?? id;
  if (!connectionId) throw new Error("useConnection must be called within a ConnectionScopeProvider or be directly provided with an id");
  const connectionIdStr = connectionIdToString(connectionId);
  if (!design.connections.has(connectionIdStr)) throw new Error(`Connection store not found for connection ${connectionId}`);
  const connectionStore = useMemo(() => design.connections.get(connectionIdStr)!, [design, connectionIdStr]);
  const getSnapshot = useMemo(() => () => connectionStore.snapshot(), [connectionStore]);
  const state = useSyncExternalStore(connectionStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

export function usePort(): Port;
export function usePort<T>(selector: (port: Port) => T): T;
export function usePort<T>(selector: (port: Port) => T, id: PortId): T;
export function usePort<T>(selector?: (port: Port) => T, id?: PortId): T | Port {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePort must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("usePort must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitIdStr)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("usePort must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  const typeIdStr = typeIdToString(typeId);
  if (!kit.types.has(typeIdStr)) throw new Error(`Type store not found for type ${typeId}`);
  const type = kit.types.get(typeIdStr)!;
  const portScope = usePortScope();
  const portId = portScope?.id ?? id;
  if (!portId) throw new Error("usePort must be called within a PortScopeProvider or be directly provided with an id");
  const portIdStr = portIdToString(portId);
  if (!type.ports.has(portIdStr)) throw new Error(`Port store not found for port ${portId}`);
  const portStore = useMemo(() => type.ports.get(portIdStr)!, [type, portIdStr]);
  const getSnapshot = useMemo(() => () => portStore.snapshot(), [portStore]);
  const state = useSyncExternalStore(portStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

export function useRepresentation(): Representation;
export function useRepresentation<T>(selector: (representation: Representation) => T): T;
export function useRepresentation<T>(selector: (representation: Representation) => T, id: RepresentationId): T;
export function useRepresentation<T>(selector?: (representation: Representation) => T, id?: RepresentationId): T | Representation {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useRepresentation must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  if (!kitScope) throw new Error("useRepresentation must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  const kitIdStr = kitIdToString(kitId);
  if (!store.kits.has(kitIdStr)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitIdStr)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("useRepresentation must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  const typeIdStr = typeIdToString(typeId);
  if (!kit.types.has(typeIdStr)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kit.types.get(typeIdStr)!;
  const representationScope = useRepresentationScope();
  const representationId = representationScope?.id ?? id;
  if (!representationId) throw new Error("useRepresentation must be called within a RepresentationScopeProvider or be directly provided with an id");
  const representationIdStr = representationIdToString(representationId);
  if (!typeStore.representations.has(representationIdStr)) throw new Error(`Representation store not found for representation ${representationId}`);
  const representationStore = useMemo(() => typeStore.representations.get(representationIdStr)!, [typeStore, representationIdStr]);
  const getSnapshot = useMemo(() => () => representationStore.snapshot(), [representationStore]);
  const state = useSyncExternalStore(representationStore.onChanged, getSnapshot, getSnapshot);
  return selector ? selector(state) : state;
}

// Additional utility hooks for the new store architecture
export function useMode(): Mode {
  return useSketchpad((store) => store.snapshot().mode);
}

export function useTheme(): Theme {
  return useSketchpad((store) => store.snapshot().theme);
}

export function useLayout(): Layout {
  return useSketchpad((store) => store.snapshot().layout);
}

export function useDesignId(): DesignId | undefined {
  const sketchpad = useSketchpad();
  return sketchpad.snapshot().activeDesignEditor?.design;
}

export function useDesigns(): DesignId[] {
  const kit = useKit();
  return kit.designs
    ? kit.designs.map((design) => ({
        name: design.name,
        variant: design.variant,
        view: design.view,
      }))
    : [];
}

export function useSketchpadCommands() {
  const sketchpad = useSketchpad();
  return {
    setMode: (mode: Mode) => sketchpad.execute("semio.sketchpad.setMode", mode),
    setTheme: (theme: Theme) => sketchpad.execute("semio.sketchpad.setTheme", theme),
    setLayout: (layout: Layout) => sketchpad.execute("semio.sketchpad.setLayout", layout),
    createKit: (kit: Kit) => sketchpad.execute("semio.sketchpad.createKit", kit),
    createDesignEditor: (designEditorId: DesignEditorId) => sketchpad.execute("semio.sketchpad.createDesignEditor", designEditorId),
    setActiveDesignEditor: (designEditorId: DesignEditorId) => sketchpad.execute("semio.sketchpad.setActiveDesignEditor", designEditorId),
  };
}

export function useKitCommands() {
  const kitStore = useKitStore();
  return {
    importKit: (url: string) => kitStore.execute("semio.kit.import", url),
    exportKit: () => kitStore.execute("semio.kit.export"),
    createType: (type: Type) => kitStore.execute("semio.kit.createType", type),
    updateType: (typeId: TypeId, typeDiff: TypeDiff) => kitStore.execute("semio.kit.updateType", typeId, typeDiff),
    deleteType: (typeId: TypeId) => kitStore.execute("semio.kit.deleteType", typeId),
    createDesign: (design: Design) => kitStore.execute("semio.kit.createDesign", design),
    updateDesign: (designId: DesignId, designDiff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", designId, designDiff),
    deleteDesign: (designId: DesignId) => kitStore.execute("semio.kit.deleteDesign", designId),
    addFile: (file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", file, blob),
    updateFile: (url: Url, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", url, fileDiff, blob),
    removeFile: (url: Url) => kitStore.execute("semio.kit.removeFile", url),
    addPiece: (designId: DesignId, piece: Piece) => kitStore.execute("semio.kit.addPiece", designId, piece),
    addPieces: (designId: DesignId, pieces: Piece[]) => kitStore.execute("semio.kit.addPieces", designId, pieces),
    removePiece: (designId: DesignId, pieceId: PieceId) => kitStore.execute("semio.kit.removePiece", designId, pieceId),
    removePieces: (designId: DesignId, pieceIds: PieceId[]) => kitStore.execute("semio.kit.removePieces", designId, pieceIds),
    addConnection: (designId: DesignId, connection: Connection) => kitStore.execute("semio.kit.addConnection", designId, connection),
    addConnections: (designId: DesignId, connections: Connection[]) => kitStore.execute("semio.kit.addConnections", designId, connections),
    removeConnection: (designId: DesignId, connectionId: ConnectionId) => kitStore.execute("semio.kit.removeConnection", designId, connectionId),
    removeConnections: (designId: DesignId, connectionIds: ConnectionId[]) => kitStore.execute("semio.kit.removeConnections", designId, connectionIds),
    deleteSelected: (designId: DesignId, selectedPieces: PieceId[], selectedConnections: ConnectionId[]) => kitStore.execute("semio.kit.deleteSelected", designId, selectedPieces, selectedConnections),
  };
}

export function useDesignEditorCommands() {
  const designEditor = useDesignEditor();
  return {
    startTransaction: () => designEditor.execute("semio.designEditor.startTransaction"),
    finalizeTransaction: () => designEditor.execute("semio.designEditor.finalizeTransaction"),
    abortTransaction: () => designEditor.execute("semio.designEditor.abortTransaction"),
    undo: () => designEditor.execute("semio.designEditor.undo"),
    redo: () => designEditor.execute("semio.designEditor.redo"),
    selectAll: () => designEditor.execute("semio.designEditor.selectAll"),
    deselectAll: () => designEditor.execute("semio.designEditor.deselectAll"),
    selectPiece: (pieceId: PieceId) => designEditor.execute("semio.designEditor.selectPiece", pieceId),
    selectPieces: (pieceIds: PieceId[]) => designEditor.execute("semio.designEditor.selectPieces", pieceIds),
    addPieceToSelection: (pieceId: PieceId) => designEditor.execute("semio.designEditor.addPieceToSelection", pieceId),
    removePieceFromSelection: (pieceId: PieceId) => designEditor.execute("semio.designEditor.removePieceFromSelection", pieceId),
    selectConnection: (connection: Connection) => designEditor.execute("semio.designEditor.selectConnection", connection),
    addConnectionToSelection: (connection: Connection) => designEditor.execute("semio.designEditor.addConnectionToSelection", connection),
    removeConnectionFromSelection: (connection: Connection) => designEditor.execute("semio.designEditor.removeConnectionFromSelection", connection),
    selectPiecePort: (pieceId: PieceId, portId: PortId) => designEditor.execute("semio.designEditor.selectPiecePort", pieceId, portId),
    deselectPiecePort: () => designEditor.execute("semio.designEditor.deselectPiecePort"),
    deleteSelected: () => designEditor.execute("semio.designEditor.deleteSelected"),
    toggleDiagramFullscreen: () => designEditor.execute("semio.designEditor.toggleDiagramFullscreen"),
    toggleModelFullscreen: () => designEditor.execute("semio.designEditor.toggleModelFullscreen"),
    executeCommand: (command: string, ...args: any[]) => designEditor.execute(command, ...args),
    execute: (command: string, ...args: any[]) => designEditor.execute(command, ...args),
  };
}
// Design editor state hooks
export function useSelection(): DesignEditorSelection {
  return useDesignEditor((store) => store.snapshot().selection);
}

export function useDesignEditorSelection(): DesignEditorSelection {
  return useDesignEditor((store) => store.snapshot().selection);
}

export function useFullscreen(): DesignEditorFullscreenPanel {
  return useDesignEditor((store) => store.snapshot().fullscreenPanel);
}

export function useDesignEditorFullscreen(): DesignEditorFullscreenPanel {
  return useDesignEditor((store) => store.snapshot().fullscreenPanel);
}

export function useDiff(): KitDiff {
  return useDesignEditor((store) => store.snapshot().diff);
}

export function useDesignEditorDesignDiff(): KitDiff {
  return useDesignEditor((store) => store.snapshot().diff);
}

export function useDiffedKit(): Kit {
  const kit = useKit();
  const diff = useDiff();
  return useMemo(() => applyKitDiff(kit, diff), [kit, diff]);
}

export function useFileUrls(): Map<Url, Url> {
  const kitStore = useKitStore();
  return kitStore.fileUrls();
}

export function useOthers(): DesignEditorPresenceOther[] {
  return useDesignEditor((store) => store.snapshot().others);
}

export function useDesignEditorOthers(): DesignEditorPresenceOther[] {
  return useDesignEditor((store) => store.snapshot().others);
}

// #endregion Hooks
