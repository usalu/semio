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
  areSameDesign,
  areSameKit,
  areSamePiece,
  areSameType,
  Attribute,
  AttributeId,
  AttributeIdLike,
  Author,
  AuthorId,
  AuthorIdLike,
  Benchmark,
  BenchmarkDiff,
  BenchmarkId,
  BenchmarkIdLike,
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
  DesignIdLike,
  DesignShallow,
  DiffStatus,
  FileDiff,
  FileId,
  FileIdLike,
  findDesignInKit,
  findPieceInDesign,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  flattenDesign,
  getClusterableGroups,
  getIncludedDesigns,
  getPieceRepresentationUrls,
  Group,
  GroupId,
  hasSameDesign,
  hasSameKit,
  hasSamePiece,
  hasSameType,
  inverseKitDiff,
  Kit,
  KitDiff,
  KitId,
  KitIdLike,
  kitIdToString,
  KitShallow,
  Layer,
  LayerId,
  Location,
  Piece,
  PieceDiff,
  PieceId,
  PieceIdLike,
  pieceIdToString,
  piecesMetadata,
  Plane,
  Port,
  PortDiff,
  PortId,
  portIdToString,
  Prop,
  PropId,
  Quality,
  QualityId,
  QualityIdLike,
  Representation,
  RepresentationDiff,
  RepresentationId,
  File as SemioFile,
  Side,
  SideId,
  Stat,
  StatId,
  Type,
  TypeDiff,
  TypeId,
  TypeIdLike,
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

// #region General

export type Subscribe = () => void;
export type Unsubscribe = () => void;
export type Disposable = () => void;
export type Transact = (fn: () => void) => void;
export type Url = string;
export type SketchpadId = string;
export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

type YUuid = string;
type YUuidArray = Y.Array<YUuid>;

type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;

type YCoord = Y.Map<number>;
type YVec = Y.Map<number>;
type YPoint = Y.Map<number>;
type YVector = Y.Map<number>;
type YVec3 = YLeafMapNumber;
type YPlane = Y.Map<YVec3>;
type YCamera = Y.Map<YPoint | YVector | number>;
type YLocation = Y.Map<YPoint | YVector>;


export interface Snapshot<TModel> {
  snapshot(): TModel;
}
export interface Id<TId> {
  id(): TId;
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
export interface Store<TModel, TId, TDiff> extends Snapshot<TModel>, Id<TId>, Actions<TDiff>, Subscriptions {}
export interface Commands<TContext, TResult> {
  execute(command: string, ...rest: any[]): Promise<TResult>;
  register(command: string, callback: (context: TContext, ...rest: any[]) => TResult): Disposable;
}
export interface StoreWithCommands<TModel, TId, TDiff, TContext, TResult> extends Store<TModel, TId, TDiff>, Commands<TContext, TResult> {}
export interface Editor<TDiff, TSelection, TSelectionDiff, TPresence, TContext, TResult>
  extends Commands<TContext, TResult>,
    EditorActions<TDiff, TSelectionDiff, TPresence>,
    EditorSelection<TSelection>,
    EditorPresence<TPresence>,
    EditorSelectionActions<TSelectionDiff> {}

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

// #endregion General

// #region Attribute

class YAttributeStore implements Store<Attribute, any, any> {
  public readonly uuid: string;
  private yAttribute: YAttribute;
  private cache?: Attribute;
  private cacheHash?: string;

  private hash(attribute: Attribute): string {
    return JSON.stringify(attribute);
  }

  constructor(yAttribute: YAttribute, attribute: Attribute) {
    this.uuid = uuidv4();
    this.yAttribute = yAttribute;
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

  snapshot = (): Attribute => {
    const currentData = {
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

  id = () => ({ key: this.key });
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

// #endregion Coord

// #region Vec

// #endregion Vec

// #region Point

// #endregion Point

// #region Vector

// #endregion Vector

// #region Plane

// #endregion Plane

// #region Camera

// #endregion Camera

// #region Location

// #endregion Location

// #region Author

class YAuthorStore {
  public readonly uuid: string;
  private yAuthor: YAuthor;
  private cache?: Author;
  private cacheHash?: string;

  private hash(author: Author): string {
    return JSON.stringify(author);
  }

  constructor(yAuthor: YAuthor, author: Author) {
    this.uuid = uuidv4();
    this.yAuthor = yAuthor;
    this.name = author.name;
    this.email = author.email;
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

  get snapshot(): Author {
    const currentHash = this.hash({
      name: this.name,
      email: this.email,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const author: Author = {
      name: this.name,
      email: this.email,
    };

    this.cache = author;
    this.cacheHash = currentHash;
    return author;
  }

  get id(): AuthorId {
    return { email: this.email };
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yAuthor, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yAuthor, subscribe, true);
  };
}

// #endregion Author

// #region File

type YFile = Y.Map<string | YAttributes>;
type YFiles = Y.Array<YFile>;

type YAttribute = Y.Map<string>;
type YAttributes = Y.Array<YAttribute>;

type YBenchmark = Y.Map<string | number | YAttributes>;
type YBenchmarks = Y.Array<YBenchmark>;

type YQuality = Y.Map<string | number | YAttributes>;
type YQualities = Y.Array<YQuality>;

type YProp = Y.Map<string | number | boolean | YAttributes>;
type YProps = Y.Array<YProp>;

type YAuthor = Y.Map<string>;
type YAuthors = Y.Array<YAuthor>;

export interface FileStore extends Store<SemioFile, FileId, FileDiff> {}

class YFileStore implements FileStore {
  public readonly uuid: string;
  private yFile: YFile;
  private cache?: SemioFile;
  private cacheHash?: string;

  private hashFile(file: SemioFile): string {
    return JSON.stringify(file);
  }

  constructor(yFile: YFile, file: SemioFile) {
    this.uuid = uuidv4();
    this.yFile = yFile;

    this.path = file.path;
    this.remote = file.remote;
    this.size = file.size;
    this.fileHash = file.hash;
    this.createdAt = file.createdAt;
    this.updatedAt = file.updatedAt;
    this.createdBy = file.createdBy;
    this.updatedBy = file.updatedBy;
  }

  get path(): string {
    return this.yFile.get("path") as string;
  }
  set path(path: string) {
    this.yFile.set("path", path);
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
    this.yFile.set("size", size || 0);
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
  set createdAt(createdAt: Date | undefined) {
    this.yFile.set("createdAt", createdAt?.toISOString() || "");
  }
  get updatedAt(): Date | undefined {
    const date = this.yFile.get("updatedAt") as string | undefined;
    return date ? new Date(date) : undefined;
  }
  set updatedAt(updatedAt: Date | undefined) {
    this.yFile.set("updatedAt", updatedAt?.toISOString() || "");
  }
  get createdBy(): AuthorId | undefined {
    const email = this.yFile.get("createdBy") as string | undefined;
    return email ? { email } : undefined;
  }
  set createdBy(createdBy: AuthorId | undefined) {
    this.yFile.set("createdBy", createdBy?.email || "");
  }
  get updatedBy(): AuthorId | undefined {
    const email = this.yFile.get("updatedBy") as string | undefined;
    return email ? { email } : undefined;
  }
  set updatedBy(updatedBy: AuthorId | undefined) {
    this.yFile.set("updatedBy", updatedBy?.email || "");
  }

  snapshot = (): SemioFile => {
    const currentData = {
      path: this.path,
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

  id = (): FileId => {
    return { path: this.path };
  };

  change = (diff: FileDiff) => {
    if (diff.path !== undefined) this.path = diff.path;
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

// #region Benchmark

class YBenchmarkStore {
  public readonly uuid: string;
  private yBenchmark: YBenchmark;
  private cache?: Benchmark;
  private cacheHash?: string;

  private hash(benchmark: Benchmark): string {
    return JSON.stringify(benchmark);
  }

  constructor(yBenchmark: YBenchmark, benchmark: Benchmark) {
    this.uuid = uuidv4();
    this.yBenchmark = yBenchmark;
    this.name = benchmark.name;
    this.icon = benchmark.icon;
    this.min = benchmark.min;
    this.minExcluded = benchmark.minExcluded;
    this.max = benchmark.max;
    this.maxExcluded = benchmark.maxExcluded;
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

  snapshot = (): Benchmark => {
    const currentData = {
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

  id = (): BenchmarkId => {
    return { name: this.name };
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

class YQualityStore {
  public readonly uuid: string;
  private yQuality: YQuality;
  private cache?: Quality;
  private cacheHash?: string;

  private hash(quality: Quality): string {
    return JSON.stringify(quality);
  }

  constructor(yQuality: YQuality, quality: Quality) {
    this.uuid = uuidv4();
    this.yQuality = yQuality;
    this.key = quality.key;
    this.name = quality.name;
    this.unit = quality.unit;
    this.description = quality.description;
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

  get snapshot(): Quality {
    const currentHash = this.hash({
      key: this.key,
      name: this.name,
      unit: this.unit,
      description: this.description,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const quality: Quality = {
      key: this.key,
      name: this.name,
      unit: this.unit,
      description: this.description,
    };

    this.cache = quality;
    this.cacheHash = currentHash;
    return quality;
  }

  get id(): QualityId {
    return { key: this.key };
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yQuality, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yQuality, subscribe, true);
  };
}

// #endregion Quality

// #region Prop

class YPropStore {
  public readonly uuid: string;
  private yProp: YProp;
  private cache?: Prop;
  private cacheHash?: string;

  private hash(prop: Prop): string {
    return JSON.stringify(prop);
  }

  constructor(yProp: YProp, prop: Prop) {
    this.uuid = uuidv4();
    this.yProp = yProp;
    this.key = prop.key;
    this.value = prop.value;
    this.unit = prop.unit;
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

  get snapshot(): Prop {
    const currentHash = this.hash({
      key: this.key,
      value: this.value || "",
      unit: this.unit,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const prop: Prop = {
      key: this.key,
      value: this.value || "",
      unit: this.unit,
    };

    this.cache = prop;
    this.cacheHash = currentHash;
    return prop;
  }

  get id(): PropId {
    return { key: this.key };
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yProp, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yProp, subscribe, true);
  };
}

// #endregion Prop

// #region Representation

type YRepresentationVal = string | YStringArray | YAttributes;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentations = Y.Array<YRepresentation>;

export interface RepresentationStore extends Store<Representation, RepresentationId, RepresentationDiff> {}

class YRepresentationStore implements RepresentationStore {
  public readonly uuid: string;
  private yRepresentation: YRepresentation;
  private cache?: Representation;
  private cacheHash?: string;

  private hash(representation: Representation): string {
    return JSON.stringify(representation);
  }

  constructor(yRepresentation: YRepresentation, representation: Representation) {
    this.uuid = uuidv4();
    this.yRepresentation = yRepresentation;
    this.url = representation.url;
    this.description = representation.description;
  }

  get url(): string {
    return this.yRepresentation.get("url") as string;
  }
  set url(url: string) {
    this.yRepresentation.set("url", url);
  }

  get description(): string | undefined {
    return this.yRepresentation.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yRepresentation.set("description", description || "");
  }

  get snapshot(): Representation {
    const currentHash = this.hash({
      url: this.url,
      description: this.description,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const representation: Representation = {
      url: this.url,
      description: this.description,
    };

    this.cache = representation;
    this.cacheHash = currentHash;
    return representation;
  }

  get id(): RepresentationId {
    return { tags: this.snapshot.tags };
  }

  apply(diff: RepresentationDiff): void {
    if (diff.url !== undefined) this.url = diff.url;
    if (diff.description !== undefined) this.description = diff.description;
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yRepresentation, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yRepresentation, subscribe, true);
  };
}

// #endregion Representation

// #region Port

type YPortVal = string | number | boolean | YLeafMapNumber | YAttributes | YStringArray | YPoint | YVector | YProps;
type YPort = Y.Map<YPortVal>;
type YPorts = Y.Array<YPort>;

export interface PortStore extends Store<Port, PortId, PortDiff> {}

class YPortStore implements PortStore {
  public readonly uuid: string;
  private yPort: YPort;
  private cache?: Port;
  private cacheHash?: string;

  private hash(port: Port): string {
    return JSON.stringify(port);
  }

  constructor(yPort: YPort, port: Port) {
    this.uuid = uuidv4();
    this.yPort = yPort;
    this.id_ = port.id_;
    this.description = port.description;
    this.family = port.family;
    this.mandatory = port.mandatory;
    this.t = port.t;
  }

  get id_(): string | undefined {
    return this.yPort.get("id_") as string | undefined;
  }
  set id_(id_: string | undefined) {
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
    this.yPort.set("mandatory", mandatory);
  }

  get t(): number {
    return this.yPort.get("t") as number;
  }
  set t(t: number) {
    this.yPort.set("t", t);
  }

  get snapshot(): Port {
    const currentHash = this.hash({
      id_: this.id_,
      description: this.description,
      family: this.family,
      mandatory: this.mandatory,
      t: this.t,
      point: { x: 0, y: 0, z: 0 }, // TODO: implement point handling
      direction: { x: 0, y: 0, z: 1 }, // TODO: implement direction handling
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const port: Port = {
      id_: this.id_,
      description: this.description,
      family: this.family,
      mandatory: this.mandatory,
      t: this.t,
      point: { x: 0, y: 0, z: 0 }, // TODO: implement point handling
      direction: { x: 0, y: 0, z: 1 }, // TODO: implement direction handling
    };

    this.cache = port;
    this.cacheHash = currentHash;
    return port;
  }

  get id(): PortId {
    return { t: this.t };
  }

  apply(diff: PortDiff): void {
    if (diff.id_ !== undefined) this.id_ = diff.id_;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.family !== undefined) this.family = diff.family;
    if (diff.mandatory !== undefined) this.mandatory = diff.mandatory;
    if (diff.t !== undefined) this.t = diff.t;
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yPort, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yPort, subscribe, true);
  };
}

// #endregion Port

// #region Type

type YTypeVal = string | number | boolean | YAuthors | YAttributes | YRepresentations | YPorts | YProps | YLocation;
type YType = Y.Map<YTypeVal>;
type YTypes = Y.Array<YType>;

export interface TypeStore extends Store<Type, TypeId, TypeDiff> {
  representations: Map<string, RepresentationStore>;
  ports: Map<string, PortStore>;
}

class YTypeStore implements TypeStore {
  public readonly uuid: string;
  public readonly parent: YKitStore;
  private yType: YType;
  private yRepresentations: YRepresentations;
  private yPorts: YPorts;
  public representations: Map<string, RepresentationStore>;
  public ports: Map<string, PortStore>;
  private cache?: Type;
  private cacheHash?: string;

  private hash(type: Type): string {
    return JSON.stringify(type);
  }

  constructor(parent: YKitStore, yType: YType, type: Type) {
    this.uuid = uuidv4();
    this.parent = parent;
    this.yType = yType;
    this.representations = new Map();
    this.ports = new Map();

    this.name = type.name;
    this.variant = type.variant;
    this.stock = type.stock;
    this.virtual = type.virtual;
    this.unit = type.unit;
    this.icon = type.icon;
    this.image = type.image;
    this.description = type.description;

    // Initialize representations with proper YStore pattern
    this.yRepresentations = this.yType.set("representations", new Y.Array<YRepresentation>());
    if (type.representations) {
      for (const representation of type.representations) {
        this.createRepresentation(representation);
      }
    }

    // Initialize ports with proper YStore pattern
    this.yPorts = this.yType.set("ports", new Y.Array<YPort>());
    if (type.ports) {
      for (const port of type.ports) {
        this.createPort(port);
      }
    }

    this.yType.set("createdAt", new Date().toISOString());
    this.updated();
  }

  get name(): string {
    return this.yType.get("name") as string;
  }
  set name(name: string) {
    this.yType.set("name", name);
  }
  get variant(): string | undefined {
    return this.yType.get("variant") as string | undefined;
  }
  set variant(variant: string | undefined) {
    this.yType.set("variant", variant || "");
  }
  get stock(): number | undefined {
    return this.yType.get("stock") as number | undefined;
  }
  set stock(stock: number | undefined) {
    this.yType.set("stock", stock);
  }
  get virtual(): boolean | undefined {
    return this.yType.get("virtual") as boolean | undefined;
  }
  set virtual(virtual: boolean | undefined) {
    this.yType.set("virtual", virtual);
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

  createRepresentation(representation: Representation): void {
    const yRepresentation = new Y.Map<YRepresentationVal>();
    const yRepresentationStore = new YRepresentationStore(yRepresentation, representation);
    this.yRepresentations.push([yRepresentation]);
    this.representations.set(representation.tags.join(","), yRepresentationStore);
  }

  hasRepresentation(representation: any): boolean {
    const key = typeof representation === "string" ? representation : representation.tags.join(",");
    return this.representations.has(key);
  }

  representation(representation: any): YRepresentationStore {
    const key = typeof representation === "string" ? representation : representation.tags.join(",");
    if (!this.hasRepresentation(representation)) throw new Error(`Representation store not found for representation ${key}`);
    return this.representations.get(key)!;
  }

  representationByUuid(uuid: string): YRepresentationStore {
    for (const rep of this.representations.values()) {
      if (rep.uuid === uuid) return rep;
    }
    throw new Error(`Representation store not found for uuid ${uuid}`);
  }

  createPort(port: Port): void {
    const yPort = new Y.Map<YPortVal>();
    const yPortStore = new YPortStore(yPort, port);
    this.yPorts.push([yPort]);
    this.ports.set(port.id_, yPortStore);
  }

  hasPort(port: any): boolean {
    const key = typeof port === "string" ? port : port.id_;
    return this.ports.has(key);
  }

  port(port: any): YPortStore {
    const key = typeof port === "string" ? port : port.id_;
    if (!this.hasPort(port)) throw new Error(`Port store not found for port ${key}`);
    return this.ports.get(key)!;
  }

  portByUuid(uuid: string): YPortStore {
    for (const p of this.ports.values()) {
      if (p.uuid === uuid) return p;
    }
    throw new Error(`Port store not found for uuid ${uuid}`);
  }

  id = (): TypeId => {
    return { name: this.name, variant: this.variant } as TypeId;
  };

  snapshot = (): Type => {
    const currentData = {
      name: this.name,
      variant: this.variant,
      stock: this.stock,
      virtual: this.virtual,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
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
    if (diff.name !== undefined) this.yType.set("name", diff.name);
    if (diff.variant !== undefined) this.yType.set("variant", diff.variant);
    this.cache = undefined;
    this.cacheHash = undefined;
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, false);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yType, subscribe, true);
  };
}

// #endregion Type

// #region Layer

class YLayerStore {
  public readonly uuid: string;
  private yLayer: YLayer;
  private cache?: Layer;
  private cacheHash?: string;

  private hash(layer: Layer): string {
    return JSON.stringify(layer);
  }

  constructor(yLayer: YLayer, layer: Layer) {
    this.uuid = uuidv4();
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
    this.yLayer.set("isHidden", isHidden);
  }

  get isLocked(): boolean | undefined {
    return this.yLayer.get("isLocked") as boolean | undefined;
  }
  set isLocked(isLocked: boolean | undefined) {
    this.yLayer.set("isLocked", isLocked);
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

  get snapshot(): Layer {
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

  get id(): LayerId {
    return { path: this.path };
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yLayer, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yLayer, subscribe, true);
  };
}

// #endregion Layer

// #region Piece

type YPieceVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YPlane | YAttributes | YCoord;
type YPiece = Y.Map<YPieceVal>;
type YPieces = Y.Array<YPiece>;

export interface PieceStore extends Store<Piece, PieceId, PieceDiff> {}

class YPieceStore {
  public readonly uuid: string;
  public readonly parent: YDesignStore;
  private yPiece: YPiece;
  private yAttributes: YAttributes;
  private attributes: YAttributeStore[];
  private cache?: Piece;
  private cacheHash?: string;

  constructor(parent: YDesignStore, yPiece: YPiece, piece: Piece) {
    this.uuid = uuidv4();
    this.parent = parent;
    this.yPiece = yPiece;
    this.attributes = new Array();

    this.localId = piece.id_;
    if (piece.type) {
      const type = this.parent.parent.type(piece.type);
      this.yPiece.set("type", type.uuid);
    } else {
      const design = this.parent.parent.design(piece.design!);
      this.yPiece.set("design", design.uuid);
    }
    this.scale = piece.scale;
    this.isHidden = piece.isHidden;
    this.isLocked = piece.isLocked;
    this.color = piece.color;
    this.description = piece.description;

    // Handle plane as Y.js object (not a store since it's a simple nested object)
    if (piece.plane) {
      const yPlane = new Y.Map<YVec3>();
      if (piece.plane.origin) {
        const yOrigin = new Y.Map<number>();
        yOrigin.set("x", piece.plane.origin.x || 0);
        yOrigin.set("y", piece.plane.origin.y || 0);
        yOrigin.set("z", piece.plane.origin.z || 0);
        yPlane.set("origin", yOrigin);
      }
      if (piece.plane.xAxis) {
        const yXAxis = new Y.Map<number>();
        yXAxis.set("x", piece.plane.xAxis.x || 0);
        yXAxis.set("y", piece.plane.xAxis.y || 0);
        yXAxis.set("z", piece.plane.xAxis.z || 0);
        yPlane.set("xAxis", yXAxis);
      }
      if (piece.plane.yAxis) {
        const yYAxis = new Y.Map<number>();
        yYAxis.set("x", piece.plane.yAxis.x || 0);
        yYAxis.set("y", piece.plane.yAxis.y || 0);
        yYAxis.set("z", piece.plane.yAxis.z || 0);
        yPlane.set("yAxis", yYAxis);
      }
      this.yPiece.set("plane", yPlane);
    }

    // Handle center as Y.js object (not a store since it's a simple nested object)
    if (piece.center) {
      const yCenter = new Y.Map<number>();
      yCenter.set("x", piece.center.x || 0);
      yCenter.set("y", piece.center.y || 0);
      yCenter.set("z", piece.center.z || 0);
      this.yPiece.set("center", yCenter);
    }

    // Handle mirrorPlane as Y.js object (not a store since it's a simple nested object)
    if (piece.mirrorPlane) {
      const yMirrorPlane = new Y.Map<YVec3>();
      if (piece.mirrorPlane.origin) {
        const yOrigin = new Y.Map<number>();
        yOrigin.set("x", piece.mirrorPlane.origin.x || 0);
        yOrigin.set("y", piece.mirrorPlane.origin.y || 0);
        yOrigin.set("z", piece.mirrorPlane.origin.z || 0);
        yMirrorPlane.set("origin", yOrigin);
      }
      if (piece.mirrorPlane.xAxis) {
        const yXAxis = new Y.Map<number>();
        yXAxis.set("x", piece.mirrorPlane.xAxis.x || 0);
        yXAxis.set("y", piece.mirrorPlane.xAxis.y || 0);
        yXAxis.set("z", piece.mirrorPlane.xAxis.z || 0);
        yMirrorPlane.set("xAxis", yXAxis);
      }
      if (piece.mirrorPlane.yAxis) {
        const yYAxis = new Y.Map<number>();
        yYAxis.set("x", piece.mirrorPlane.yAxis.x || 0);
        yYAxis.set("y", piece.mirrorPlane.yAxis.y || 0);
        yYAxis.set("z", piece.mirrorPlane.yAxis.z || 0);
        yMirrorPlane.set("yAxis", yYAxis);
      }
      this.yPiece.set("mirrorPlane", yMirrorPlane);
    }

    // Handle attributes with proper YStore pattern
    this.yAttributes = this.yPiece.set("attributes", new Y.Array<YAttribute>());
    if (piece.attributes) {
      for (const attribute of piece.attributes) {
        this.createAttribute(attribute);
      }
    }
  }

  get localId(): string {
    return this.yPiece.get("id_") as string;
  }
  set localId(localId: string) {
    this.yPiece.set("id_", localId);
  }
  get type(): TypeId {
    return this.parent.parent.typeByUuid(this.yPiece.get("type") as string).id();
  }
  set type(type: TypeId | undefined) {
    if (type) {
      this.yPiece.set("type", this.parent.parent.type(type).uuid);
    } else {
      this.yPiece.set("type", "");
    }
  }
  get design(): DesignId {
    return this.parent.parent.designByUuid(this.yPiece.get("design") as string).id();
  }
  set design(design: DesignId | undefined) {
    if (design) {
      this.yPiece.set("design", this.parent.parent.design(design).uuid);
    } else {
      this.yPiece.set("design", "");
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

  createAttribute(attribute: Attribute): void {
    const yAttribute = new Y.Map<string>();
    const yAttributeStore = new YAttributeStore(yAttribute, attribute);
    this.yAttributes.push([yAttribute]);
    this.attributes.push(yAttributeStore);
  }

  hasAttribute(attribute: any): boolean {
    return this.attributes.some((a) => a.key === attribute.key || a.key === attribute);
  }

  attribute(attribute: any): YAttributeStore {
    if (!this.hasAttribute(attribute)) throw new Error(`Attribute store not found for attribute ${attribute}`);
    return this.attributes.find((a) => a.key === attribute.key || a.key === attribute)!;
  }

  attributeByUuid(uuid: string): YAttributeStore {
    return this.attributes.find((a) => a.uuid === uuid)!;
  }

  public hash(piece: Piece): string {
    return JSON.stringify(piece);
  }

  id = (): PieceId => {
    return { id_: this.localId } as PieceId;
  };

  snapshot = (): Piece => {
    // Extract complex types from Y.js objects
    let plane: Plane | undefined;
    const yPlane = this.yPiece.get("plane") as YPlane | undefined;
    if (yPlane) {
      const yOrigin = yPlane.get("origin") as Y.Map<number> | undefined;
      const yXAxis = yPlane.get("xAxis") as Y.Map<number> | undefined;
      const yYAxis = yPlane.get("yAxis") as Y.Map<number> | undefined;
      plane = {
        origin: yOrigin ? { x: yOrigin.get("x") || 0, y: yOrigin.get("y") || 0, z: yOrigin.get("z") || 0 } : undefined,
        xAxis: yXAxis ? { x: yXAxis.get("x") || 0, y: yXAxis.get("y") || 0, z: yXAxis.get("z") || 0 } : undefined,
        yAxis: yYAxis ? { x: yYAxis.get("x") || 0, y: yYAxis.get("y") || 0, z: yYAxis.get("z") || 0 } : undefined,
      };
    }

    let center: Coord | undefined;
    const yCenter = this.yPiece.get("center") as Y.Map<number> | undefined;
    if (yCenter) {
      center = {
        x: yCenter.get("x") || 0,
        y: yCenter.get("y") || 0,
        z: yCenter.get("z") || 0,
      };
    }

    let mirrorPlane: Plane | undefined;
    const yMirrorPlane = this.yPiece.get("mirrorPlane") as YPlane | undefined;
    if (yMirrorPlane) {
      const yOrigin = yMirrorPlane.get("origin") as Y.Map<number> | undefined;
      const yXAxis = yMirrorPlane.get("xAxis") as Y.Map<number> | undefined;
      const yYAxis = yMirrorPlane.get("yAxis") as Y.Map<number> | undefined;
      mirrorPlane = {
        origin: yOrigin ? { x: yOrigin.get("x") || 0, y: yOrigin.get("y") || 0, z: yOrigin.get("z") || 0 } : undefined,
        xAxis: yXAxis ? { x: yXAxis.get("x") || 0, y: yXAxis.get("y") || 0, z: yXAxis.get("z") || 0 } : undefined,
        yAxis: yYAxis ? { x: yYAxis.get("x") || 0, y: yYAxis.get("y") || 0, z: yYAxis.get("z") || 0 } : undefined,
      };
    }

    const currentData = {
      id_: this.localId,
      type: this.type,
      design: this.design,
      scale: this.scale,
      isHidden: this.isHidden,
      isLocked: this.isLocked,
      color: this.color,
      description: this.description,
      plane,
      center,
      mirrorPlane,
      attributes: this.attributes.map((attribute) => attribute.snapshot()),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: PieceDiff) => {
    if (diff.id_) this.localId = diff.id_;
    if (diff.type !== undefined) this.type = diff.type;
    if (diff.design !== undefined) this.design = diff.design;
    if (diff.scale !== undefined) this.scale = diff.scale;
    if (diff.isHidden !== undefined) this.isHidden = diff.isHidden;
    if (diff.isLocked !== undefined) this.isLocked = diff.isLocked;
    if (diff.color !== undefined) this.color = diff.color;
    if (diff.description !== undefined) this.description = diff.description;

    // Handle complex type changes
    if (diff.plane !== undefined) {
      if (diff.plane) {
        const yPlane = new Y.Map<YVec3>();
        if (diff.plane.origin) {
          const yOrigin = new Y.Map<number>();
          yOrigin.set("x", diff.plane.origin.x || 0);
          yOrigin.set("y", diff.plane.origin.y || 0);
          yOrigin.set("z", diff.plane.origin.z || 0);
          yPlane.set("origin", yOrigin);
        }
        if (diff.plane.xAxis) {
          const yXAxis = new Y.Map<number>();
          yXAxis.set("x", diff.plane.xAxis.x || 0);
          yXAxis.set("y", diff.plane.xAxis.y || 0);
          yXAxis.set("z", diff.plane.xAxis.z || 0);
          yPlane.set("xAxis", yXAxis);
        }
        if (diff.plane.yAxis) {
          const yYAxis = new Y.Map<number>();
          yYAxis.set("x", diff.plane.yAxis.x || 0);
          yYAxis.set("y", diff.plane.yAxis.y || 0);
          yYAxis.set("z", diff.plane.yAxis.z || 0);
          yPlane.set("yAxis", yYAxis);
        }
        this.yPiece.set("plane", yPlane);
      } else {
        this.yPiece.delete("plane");
      }
    }

    if (diff.center !== undefined) {
      if (diff.center) {
        const yCenter = new Y.Map<number>();
        yCenter.set("x", diff.center.x || 0);
        yCenter.set("y", diff.center.y || 0);
        yCenter.set("z", diff.center.z || 0);
        this.yPiece.set("center", yCenter);
      } else {
        this.yPiece.delete("center");
      }
    }

    if (diff.mirrorPlane !== undefined) {
      if (diff.mirrorPlane) {
        const yMirrorPlane = new Y.Map<YVec3>();
        if (diff.mirrorPlane.origin) {
          const yOrigin = new Y.Map<number>();
          yOrigin.set("x", diff.mirrorPlane.origin.x || 0);
          yOrigin.set("y", diff.mirrorPlane.origin.y || 0);
          yOrigin.set("z", diff.mirrorPlane.origin.z || 0);
          yMirrorPlane.set("origin", yOrigin);
        }
        if (diff.mirrorPlane.xAxis) {
          const yXAxis = new Y.Map<number>();
          yXAxis.set("x", diff.mirrorPlane.xAxis.x || 0);
          yXAxis.set("y", diff.mirrorPlane.xAxis.y || 0);
          yXAxis.set("z", diff.mirrorPlane.xAxis.z || 0);
          yMirrorPlane.set("xAxis", yXAxis);
        }
        if (diff.mirrorPlane.yAxis) {
          const yYAxis = new Y.Map<number>();
          yYAxis.set("x", diff.mirrorPlane.yAxis.x || 0);
          yYAxis.set("y", diff.mirrorPlane.yAxis.y || 0);
          yYAxis.set("z", diff.mirrorPlane.yAxis.z || 0);
          yMirrorPlane.set("yAxis", yYAxis);
        }
        this.yPiece.set("mirrorPlane", yMirrorPlane);
      } else {
        this.yPiece.delete("mirrorPlane");
      }
    }

    if (diff.attributes !== undefined) {
      // Clear existing attributes
      this.attributes = [];
      this.yAttributes.delete(0, this.yAttributes.length);

      if (diff.attributes) {
        for (const attribute of diff.attributes) {
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

// #endregion Piece

// #region Group

class YGroupStore {
  public readonly uuid: string;
  private yGroup: YGroup;
  private cache?: Group;
  private cacheHash?: string;

  private hash(group: Group): string {
    return JSON.stringify(group);
  }

  constructor(yGroup: YGroup, group: Group) {
    this.uuid = uuidv4();
    this.yGroup = yGroup;
    this.color = group.color;
    this.name = group.name;
    this.description = group.description;
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

  get snapshot(): Group {
    const currentHash = this.hash({
      pieces: [], // TODO: implement pieces handling
      color: this.color,
      name: this.name,
      description: this.description,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const group: Group = {
      pieces: [], // TODO: implement pieces handling
      color: this.color,
      name: this.name,
      description: this.description,
    };

    this.cache = group;
    this.cacheHash = currentHash;
    return group;
  }

  get id(): GroupId {
    return { pieces: [] }; // TODO: implement pieces handling
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yGroup, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yGroup, subscribe, true);
  };
}

// #endregion Group

// #region Side

class YSideStore {
  public readonly uuid: string;
  private ySide: YSide;
  private cache?: Side;
  private cacheHash?: string;

  private hash(side: Side): string {
    return JSON.stringify(side);
  }

  constructor(ySide: YSide, side: Side) {
    this.uuid = uuidv4();
    this.ySide = ySide;
    // TODO: implement side properties
  }

  get snapshot(): Side {
    const currentHash = this.hash({
      piece: { id_: "" }, // TODO: implement piece handling
      port: { t: 0 }, // TODO: implement port handling
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const side: Side = {
      piece: { id_: "" }, // TODO: implement piece handling
      port: { t: 0 }, // TODO: implement port handling
    };

    this.cache = side;
    this.cacheHash = currentHash;
    return side;
  }

  get id(): SideId {
    return { piece: { id_: "" }, port: { t: 0 } }; // TODO: implement handling
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.ySide, subscribe, true);
  };
}

// #endregion Side

// #region Connection

type YSide = Y.Map<YLeafMapString>;
type YSides = Y.Array<YSide>;

type YConnectionVal = string | number | YAttributes | YSide | YSides;
type YConnection = Y.Map<YConnectionVal>;
type YConnections = Y.Array<YConnection>;

export interface ConnectionStore extends Store<Connection, ConnectionId, ConnectionDiff> {}

class YConnectionStore implements ConnectionStore {
  public readonly uuid: string;
  private yConnection: YConnection;
  private cache?: Connection;
  private cacheHash?: string;

  private hash(connection: Connection): string {
    return JSON.stringify(connection);
  }

  constructor(yConnection: YConnection, connection: Connection) {
    this.uuid = uuidv4();
    this.yConnection = yConnection;
    this.id_ = connection.id_;
    this.description = connection.description;
    this.quality = connection.quality;
  }

  get id_(): string {
    return this.yConnection.get("id_") as string;
  }
  set id_(id_: string) {
    this.yConnection.set("id_", id_);
  }

  get description(): string | undefined {
    return this.yConnection.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yConnection.set("description", description || "");
  }

  get quality(): number | undefined {
    return this.yConnection.get("quality") as number | undefined;
  }
  set quality(quality: number | undefined) {
    this.yConnection.set("quality", quality);
  }

  get snapshot(): Connection {
    const currentHash = this.hash({
      id_: this.id_,
      description: this.description,
      quality: this.quality,
      sides: [], // TODO: implement sides handling
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const connection: Connection = {
      id_: this.id_,
      description: this.description,
      quality: this.quality,
      sides: [], // TODO: implement sides handling
    };

    this.cache = connection;
    this.cacheHash = currentHash;
    return connection;
  }

  get id(): ConnectionId {
    return { id_: this.id_ };
  }

  apply(diff: ConnectionDiff): void {
    if (diff.id_ !== undefined) this.id_ = diff.id_;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.quality !== undefined) this.quality = diff.quality;
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe, true);
  };
}

// #endregion Connection

// #region Stat

class YStatStore {
  public readonly uuid: string;
  private yStat: YStat;
class YConnectionStore implements ConnectionStore {
  public readonly uuid: string;
  private yConnection: YConnection;
  private cache?: Connection;
  private cacheHash?: string;

  private hash(connection: Connection): string {
    return JSON.stringify(connection);
  }

  constructor(yConnection: YConnection, connection: Connection) {
    this.uuid = uuidv4();
    this.yConnection = yConnection;
    this.id_ = connection.id_;
    this.description = connection.description;
    this.quality = connection.quality;
  }

  get id_(): string {
    return this.yConnection.get("id_") as string;
  }
  set id_(id_: string) {
    this.yConnection.set("id_", id_);
  }

  get description(): string | undefined {
    return this.yConnection.get("description") as string | undefined;
  }
  set description(description: string | undefined) {
    this.yConnection.set("description", description || "");
  }

  get quality(): number | undefined {
    return this.yConnection.get("quality") as number | undefined;
  }
  set quality(quality: number | undefined) {
    this.yConnection.set("quality", quality);
  }

  get snapshot(): Connection {
    const currentHash = this.hash({
      id_: this.id_,
      description: this.description,
      quality: this.quality,
      sides: [], // TODO: implement sides handling
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const connection: Connection = {
      id_: this.id_,
      description: this.description,
      quality: this.quality,
      sides: [], // TODO: implement sides handling
    };

    this.cache = connection;
    this.cacheHash = currentHash;
    return connection;
  }

  get id(): ConnectionId {
    return { id_: this.id_ };
  }

  apply(diff: ConnectionDiff): void {
    if (diff.id_ !== undefined) this.id_ = diff.id_;
    if (diff.description !== undefined) this.description = diff.description;
    if (diff.quality !== undefined) this.quality = diff.quality;
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yConnection, subscribe, true);
  };
}  private cache?: Stat;
  private cacheHash?: string;

  private hash(stat: Stat): string {
    return JSON.stringify(stat);
  }

  constructor(yStat: YStat, stat: Stat) {
    this.uuid = uuidv4();
    this.yStat = yStat;
    this.key = stat.key;
    this.unit = stat.unit;
    this.min = stat.min;
    this.minExcluded = stat.minExcluded;
    this.max = stat.max;
    this.maxExcluded = stat.maxExcluded;
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
    this.yStat.set("unit", unit || "");
  }

  get min(): number | undefined {
    return this.yStat.get("min") as number | undefined;
  }
  set min(min: number | undefined) {
    this.yStat.set("min", min);
  }

  get minExcluded(): boolean | undefined {
    return this.yStat.get("minExcluded") as boolean | undefined;
  }
  set minExcluded(minExcluded: boolean | undefined) {
    this.yStat.set("minExcluded", minExcluded);
  }

  get max(): number | undefined {
    return this.yStat.get("max") as number | undefined;
  }
  set max(max: number | undefined) {
    this.yStat.set("max", max);
  }

  get maxExcluded(): boolean | undefined {
    return this.yStat.get("maxExcluded") as boolean | undefined;
  }
  set maxExcluded(maxExcluded: boolean | undefined) {
    this.yStat.set("maxExcluded", maxExcluded);
  }

  get snapshot(): Stat {
    const currentHash = this.hash({
      key: this.key,
      unit: this.unit,
      min: this.min,
      minExcluded: this.minExcluded,
      max: this.max,
      maxExcluded: this.maxExcluded,
    });

    if (this.cache && this.cacheHash === currentHash) {
      return this.cache;
    }

    const stat: Stat = {
      key: this.key,
      unit: this.unit,
      min: this.min,
      minExcluded: this.minExcluded,
      max: this.max,
      maxExcluded: this.maxExcluded,
    };

    this.cache = stat;
    this.cacheHash = currentHash;
    return stat;
  }

  get id(): StatId {
    return { key: this.key };
  }

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yStat, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yStat, subscribe, true);
  };
}

// #endregion Stat

// #region Design

type YLayer = Y.Map<string | boolean | YAttributes>;
type YLayers = Y.Array<YLayer>;

type YGroup = Y.Map<string | YStringArray | YAttributes>;
type YGroups = Y.Array<YGroup>;

type YStat = Y.Map<string | number | boolean>;
type YStats = Y.Array<YStat>;

type YDesignVal = string | YAuthors | YAttributes | YPieces | YConnections | YLayers | YGroups | YStats;
type YDesign = Y.Map<YDesignVal>;
type YDesigns = Y.Array<YDesign>;

type YDesignEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignEditor = Y.Map<YDesignEditorVal>;
type YDesignEditors = Y.Array<YDesignEditor>;

export interface DesignStore extends Store<Design, DesignId, DesignDiff> {
  pieces: Map<string, PieceStore>;
  connections: Map<string, ConnectionStore>;
}

class YDesignStore {
  public readonly uuid: string;
  public readonly parent: YKitStore;
  private yDesign: YDesign;
  private yPieces: YPieces;
  private pieces: YPieceStore[];
  private yConnections: YConnections;
  private connections: YConnectionStore[];
  private yAttributes: YAttributes;
  private attributes: YAttributeStore[];
  private cache?: Design;
  private cacheHash?: string;

  constructor(parent: YKitStore, yDesign: YDesign, design: Design) {
    this.uuid = uuidv4();
    this.parent = parent;
    this.yDesign = yDesign;
    this.pieces = new Array();
    this.connections = new Array();
    this.attributes = new Array();

    this.name = design.name;
    this.variant = design.variant;
    this.view = design.view;
    this.canScale = design.canScale;
    this.canMirror = design.canMirror;
    this.unit = design.unit;
    this.icon = design.icon;
    this.image = design.image;
    this.description = design.description;

    // Initialize connections with proper YStore pattern
    this.yConnections = this.yDesign.set("connections", new Y.Array<YConnection>());
    if (design.connections) {
      for (const connection of design.connections) {
        this.createConnection(connection);
      }
    }

    if (design.stats) {
      const yStats = new Y.Array<YStat>();
      design.stats.forEach((stat) => {
        const yStat = new Y.Map();
        yStat.set("key", stat.key || "");
        yStat.set("value", stat.value);
        yStat.set("definition", stat.definition || "");
        yStats.push([yStat]);
      });
      this.yDesign.set("stats", yStats);
    }

    if (design.props) {
      const yProps = new Y.Array<YProp>();
      design.props.forEach((prop) => {
        const yProp = new Y.Map();
        yProp.set("key", prop.key || "");
        yProp.set("value", prop.value);
        if (prop.unit) yProp.set("unit", prop.unit);
        yProps.push([yProp]);
      });
      this.yDesign.set("props", yProps);
    }

    if (design.layers) {
      const yLayers = new Y.Array<YLayer>();
      design.layers.forEach((layer) => {
        const yLayer = new Y.Map();
        yLayer.set("path", layer.path || "");
        yLayer.set("isVisible", layer.isVisible);
        yLayers.push([yLayer]);
      });
      this.yDesign.set("layers", yLayers);
    }

    if (design.activeLayer) {
      this.yDesign.set("activeLayer", design.activeLayer.path || "");
    }

    if (design.groups) {
      const yGroups = new Y.Array<YGroup>();
      design.groups.forEach((group) => {
        const yGroup = new Y.Map();
        const pieceIds = new Y.Array<string>();
        group.pieces.forEach((piece) => pieceIds.push([piece.id_]));
        yGroup.set("pieces", pieceIds);
        yGroups.push([yGroup]);
      });
      this.yDesign.set("groups", yGroups);
    }

    if (design.location) {
      const yLocation = new Y.Map();
      if (design.location.position) {
        const yPosition = new Y.Map<number>();
        yPosition.set("x", design.location.position.x || 0);
        yPosition.set("y", design.location.position.y || 0);
        yPosition.set("z", design.location.position.z || 0);
        yLocation.set("position", yPosition);
      }
      if (design.location.direction) {
        const yDirection = new Y.Map<number>();
        yDirection.set("x", design.location.direction.x || 0);
        yDirection.set("y", design.location.direction.y || 0);
        yDirection.set("z", design.location.direction.z || 0);
        yLocation.set("direction", yDirection);
      }
      this.yDesign.set("location", yLocation);
    }

    if (design.authors) {
      const yAuthors = new Y.Array<YAuthor>();
      design.authors.forEach((author) => {
        const yAuthor = new Y.Map<string>();
        yAuthor.set("email", author.email || "");
        yAuthors.push([yAuthor]);
      });
      this.yDesign.set("authors", yAuthors);
    }

    if (design.concepts) {
      const yConcepts = new Y.Array<string>();
      design.concepts.forEach((concept) => yConcepts.push([concept]));
      this.yDesign.set("concepts", yConcepts);
    }

    // Initialize attributes with proper YStore pattern
    this.yAttributes = this.yDesign.set("attributes", new Y.Array<YAttribute>());
    if (design.attributes) {
      for (const attribute of design.attributes) {
        this.createAttribute(attribute);
      }
    }

    this.yPieces = this.yDesign.set("pieces", new Y.Array<YPiece>());
    if (design.pieces) {
      for (const piece of design.pieces) {
        this.createPiece(piece);
      }
    }

    this.yDesign.set("createdAt", new Date().toISOString());
    this.yDesign.set("updatedAt", new Date().toISOString());
  }

  get name(): string {
    return this.yDesign.get("name") as string;
  }
  set name(name: string) {
    this.yDesign.set("name", name);
  }
  get variant(): string | undefined {
    return this.yDesign.get("variant") as string | undefined;
  }
  set variant(variant: string | undefined) {
    this.yDesign.set("variant", variant || "");
  }
  get view(): string | undefined {
    return this.yDesign.get("view") as string | undefined;
  }
  set view(view: string | undefined) {
    this.yDesign.set("view", view || "");
  }
  get canScale(): boolean | undefined {
    return this.yDesign.get("canScale") as boolean | undefined;
  }
  set canScale(canScale: boolean | undefined) {
    this.yDesign.set("canScale", canScale);
  }
  get canMirror(): boolean | undefined {
    return this.yDesign.get("canMirror") as boolean | undefined;
  }
  set canMirror(canMirror: boolean | undefined) {
    this.yDesign.set("canMirror", canMirror);
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

  id(): DesignId {
    return { name: this.name, variant: this.variant, view: this.view } as DesignId;
  }

  hasPiece(piece: PieceIdLike): boolean {
    return hasSamePiece(
      piece,
      this.pieces.map((piece) => piece.id()),
    );
  }

  createPiece(piece: Piece): void {
    const yPiece = new Y.Map<YPieceVal>();
    const yPieceStore = new YPieceStore(this, yPiece, piece);
    this.yPieces!.push([yPiece]);
    this.pieces.push(yPieceStore);
  }

  createConnection(connection: Connection): void {
    const yConnection = new Y.Map<YConnectionVal>();
    const yConnectionStore = new YConnectionStore(yConnection, connection);
    this.yConnections.push([yConnection]);
    this.connections.push(yConnectionStore);
  }

  createAttribute(attribute: Attribute): void {
    const yAttribute = new Y.Map<string>();
    const yAttributeStore = new YAttributeStore(yAttribute, attribute);
    this.yAttributes.push([yAttribute]);
    this.attributes.push(yAttributeStore);
  }

  piece(piece: PieceIdLike): PieceStore {
    if (!this.hasPiece(piece)) throw new Error(`Piece store not found for piece ${piece}`);
    return this.pieces.find((p) => areSamePiece(p.id(), piece))!;
  }

  pieceByUuid(uuid: string): YPieceStore {
    return this.pieces.find((p) => p.uuid === uuid)!;
  }

  hasConnection(connection: any): boolean {
    return this.connections.some((c) => c.id.id_ === connection.id_ || c.id.id_ === connection);
  }

  connection(connection: any): YConnectionStore {
    if (!this.hasConnection(connection)) throw new Error(`Connection store not found for connection ${connection}`);
    return this.connections.find((c) => c.id.id_ === connection.id_ || c.id.id_ === connection)!;
  }

  connectionByUuid(uuid: string): YConnectionStore {
    return this.connections.find((c) => c.uuid === uuid)!;
  }

  hasAttribute(attribute: any): boolean {
    return this.attributes.some((a) => a.key === attribute.key || a.key === attribute);
  }

  attribute(attribute: any): YAttributeStore {
    if (!this.hasAttribute(attribute)) throw new Error(`Attribute store not found for attribute ${attribute}`);
    return this.attributes.find((a) => a.key === attribute.key || a.key === attribute)!;
  }

  attributeByUuid(uuid: string): YAttributeStore {
    return this.attributes.find((a) => a.uuid === uuid)!;
  }

  hash(design: Design): string {
    return JSON.stringify(design);
  }

  snapshot = (): Design => {
    let stats: Stat[] | undefined;
    const yStats = this.yDesign.get("stats") as Y.Array<YStat> | undefined;
    if (yStats) {
      stats = yStats.toArray().map((yStat) => ({
        key: yStat.get("key") || "",
        value: yStat.get("value"),
        definition: yStat.get("definition") || "",
      }));
    }

    let props: Prop[] | undefined;
    const yProps = this.yDesign.get("props") as Y.Array<YProp> | undefined;
    if (yProps) {
      props = yProps.toArray().map((yProp) => ({
        key: (yProp.get("key") as string) || "",
        value: (yProp.get("value") as string) || "",
        unit: yProp.get("unit") as string | undefined,
      }));
    }

    let layers: Layer[] | undefined;
    const yLayers = this.yDesign.get("layers") as Y.Array<YLayer> | undefined;
    if (yLayers) {
      layers = yLayers.toArray().map((yLayer) => ({
        path: yLayer.get("path") || "",
        isVisible: yLayer.get("isVisible") as boolean,
      }));
    }

    let activeLayer: LayerId | undefined;
    const activeLayerPath = this.yDesign.get("activeLayer") as string | undefined;
    if (activeLayerPath) {
      activeLayer = { path: activeLayerPath };
    }

    let groups: Group[] | undefined;
    const yGroups = this.yDesign.get("groups") as Y.Array<YGroup> | undefined;
    if (yGroups) {
      groups = yGroups.toArray().map((yGroup) => {
        const pieceIds = yGroup.get("pieces") as Y.Array<string>;
        return {
          pieces: pieceIds.toArray().map((id) => ({ id_: id })),
        };
      });
    }

    let location: Location | undefined;
    const yLocation = this.yDesign.get("location") as Y.Map<any> | undefined;
    if (yLocation) {
      const yPosition = yLocation.get("position") as Y.Map<number> | undefined;
      const yDirection = yLocation.get("direction") as Y.Map<number> | undefined;
      location = {
        position: yPosition ? { x: yPosition.get("x") || 0, y: yPosition.get("y") || 0, z: yPosition.get("z") || 0 } : undefined,
        direction: yDirection ? { x: yDirection.get("x") || 0, y: yDirection.get("y") || 0, z: yDirection.get("z") || 0 } : undefined,
      };
    }

    let authors: AuthorId[] | undefined;
    const yAuthors = this.yDesign.get("authors") as Y.Array<YAuthor> | undefined;
    if (yAuthors) {
      authors = yAuthors.toArray().map((yAuthor) => ({
        email: yAuthor.get("email") || "",
      }));
    }

    let concepts: string[] | undefined;
    const yConcepts = this.yDesign.get("concepts") as Y.Array<string> | undefined;
    if (yConcepts) {
      concepts = yConcepts.toArray();
    }

    const currentData = {
      name: this.name,
      variant: this.variant,
      view: this.view,
      canScale: this.canScale,
      canMirror: this.canMirror,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
      pieces: this.pieces.map((piece) => piece.snapshot()),
      connections: this.connections.map((connection) => connection.snapshot()),
      stats,
      props,
      layers,
      activeLayer,
      groups,
      location,
      authors,
      concepts,
      attributes: this.attributes.map((attribute) => attribute.snapshot()),
      createdAt: this.createdAt.toISOString(),
      updatedAt: this.updatedAt.toISOString(),
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
    if (diff.variant !== undefined) this.variant = diff.variant;
    if (diff.view !== undefined) this.view = diff.view;
    if (diff.canScale !== undefined) this.canScale = diff.canScale;
    if (diff.canMirror !== undefined) this.canMirror = diff.canMirror;
    if (diff.unit !== undefined) this.unit = diff.unit;
    if (diff.icon !== undefined) this.icon = diff.icon;
    if (diff.image !== undefined) this.image = diff.image;
    if (diff.description !== undefined) this.description = diff.description;

    // Handle complex type changes using proper YStore pattern
    if (diff.connections !== undefined) {
      // Clear existing connections
      this.connections = [];
      this.yConnections.delete(0, this.yConnections.length);

      if (diff.connections) {
        for (const connection of diff.connections) {
          this.createConnection(connection);
        }
      }
    }

    if (diff.stats !== undefined) {
      if (diff.stats) {
        const yStats = new Y.Array<YStat>();
        diff.stats.forEach((stat) => {
          const yStat = new Y.Map();
          yStat.set("key", stat.key || "");
          yStat.set("value", stat.value);
          yStat.set("definition", stat.definition || "");
          yStats.push([yStat]);
        });
        this.yDesign.set("stats", yStats);
      } else {
        this.yDesign.delete("stats");
      }
    }

    if (diff.props !== undefined) {
      if (diff.props) {
        const yProps = new Y.Array<YProp>();
        diff.props.forEach((prop) => {
          const yProp = new Y.Map();
          yProp.set("key", prop.key || "");
          yProp.set("value", prop.value);
          yProp.set("definition", prop.definition || "");
          yProps.push([yProp]);
        });
        this.yDesign.set("props", yProps);
      } else {
        this.yDesign.delete("props");
      }
    }

    if (diff.layers !== undefined) {
      if (diff.layers) {
        const yLayers = new Y.Array<YLayer>();
        diff.layers.forEach((layer) => {
          const yLayer = new Y.Map();
          yLayer.set("path", layer.path || "");
          yLayer.set("isVisible", layer.isVisible);
          yLayers.push([yLayer]);
        });
        this.yDesign.set("layers", yLayers);
      } else {
        this.yDesign.delete("layers");
      }
    }

    if (diff.activeLayer !== undefined) {
      if (diff.activeLayer) {
        this.yDesign.set("activeLayer", diff.activeLayer.path || "");
      } else {
        this.yDesign.delete("activeLayer");
      }
    }

    if (diff.groups !== undefined) {
      if (diff.groups) {
        const yGroups = new Y.Array<YGroup>();
        diff.groups.forEach((group) => {
          const yGroup = new Y.Map();
          const pieceIds = new Y.Array<string>();
          group.pieces.forEach((piece) => pieceIds.push([piece.id_]));
          yGroup.set("pieces", pieceIds);
          yGroups.push([yGroup]);
        });
        this.yDesign.set("groups", yGroups);
      } else {
        this.yDesign.delete("groups");
      }
    }

    if (diff.location !== undefined) {
      if (diff.location) {
        const yLocation = new Y.Map();
        if (diff.location.position) {
          const yPosition = new Y.Map<number>();
          yPosition.set("x", diff.location.position.x || 0);
          yPosition.set("y", diff.location.position.y || 0);
          yPosition.set("z", diff.location.position.z || 0);
          yLocation.set("position", yPosition);
        }
        if (diff.location.direction) {
          const yDirection = new Y.Map<number>();
          yDirection.set("x", diff.location.direction.x || 0);
          yDirection.set("y", diff.location.direction.y || 0);
          yDirection.set("z", diff.location.direction.z || 0);
          yLocation.set("direction", yDirection);
        }
        this.yDesign.set("location", yLocation);
      } else {
        this.yDesign.delete("location");
      }
    }

    if (diff.authors !== undefined) {
      if (diff.authors) {
        const yAuthors = new Y.Array<YAuthor>();
        diff.authors.forEach((author) => {
          const yAuthor = new Y.Map<string>();
          yAuthor.set("email", author.email || "");
          yAuthors.push([yAuthor]);
        });
        this.yDesign.set("authors", yAuthors);
      } else {
        this.yDesign.delete("authors");
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

    if (diff.attributes !== undefined) {
      // Clear existing attributes
      this.attributes = [];
      this.yAttributes.delete(0, this.yAttributes.length);

      if (diff.attributes) {
        for (const attribute of diff.attributes) {
          this.createAttribute(attribute);
        }
      }
    }

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

// #endregion Design

// #region Kit

type YIdMap = Y.Map<string>;
type YKitVal = string | YUuidArray | YIdMap | YAttributes | YAuthors | YFiles | YBenchmarks | YQualities | YProps | YTypes | YDesigns;
type YKit = Y.Map<YKitVal>;
type YKits = Y.Array<YKit>;

export interface KitStore extends StoreWithCommands<Kit, KitId, KitDiff, KitCommandContext, KitCommandResult> {
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

class YKitStore {
  public readonly uri: string;
  public readonly parent: YSketchpadStore;
  private readonly yProviderFactory: YProviderFactory | undefined;
  private readonly yDoc: Y.Doc;
  private readonly yKit: YKit;
  private readonly yTypes: YTypes;
  private readonly types: YTypeStore[];
  private readonly yDesigns: YDesigns;
  private readonly designs: YDesignStore[];
  private readonly yFiles: YFiles;
  private readonly files: YFileStore[];
  private readonly yQualities: YQualities;
  private readonly qualities: YQualityStore[];
  private readonly yBenchmarks: YBenchmarks;
  private readonly benchmarks: YBenchmarkStore[];
  private readonly yAuthors: YAuthors;
  private readonly authors: YAuthorStore[];
  private readonly yAttributes: YAttributes;
  private readonly attributes: YAttributeStore[];
  private readonly persistence: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult>;
  private readonly regularFiles: Map<Url, string>;
  private cache?: Kit;
  private cacheHash?: string;

  constructor(parent: YSketchpadStore, uri: string, kit: Kit, yProviderFactory?: YProviderFactory) {
    this.uri = uri;
    this.parent = parent;
    this.yProviderFactory = yProviderFactory;
    this.yDoc = new Y.Doc();

    if (yProviderFactory) {
      yProviderFactory(this.yDoc, this.uri);
    }

    this.commandRegistry = new Map();
    this.regularFiles = new Map();
    this.types = new Array();
    this.designs = new Array();
    this.files = new Array();
    this.qualities = new Array();
    this.benchmarks = new Array();
    this.authors = new Array();
    this.attributes = new Array();

    this.yKit = this.yDoc.getMap() as YKit;
    this.yTypes = this.yDoc.getArray("types");
    this.yDesigns = this.yDoc.getArray("designs");
    this.yFiles = this.yDoc.getArray("files");
    this.yQualities = this.yDoc.getArray("qualities");
    this.yBenchmarks = this.yDoc.getArray("benchmarks");
    this.yAuthors = this.yDoc.getArray("authors");
    this.yAttributes = this.yDoc.getArray("attributes");

    this.yDoc.transact(() => {
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

      if (kit.types) for (const type of kit.types) this.createType(type);
      if (kit.designs) for (const design of kit.designs) this.createDesign(design);
      if (kit.files) for (const file of kit.files) this.createFile(file);
      if (kit.qualities) for (const quality of kit.qualities) this.createQuality(quality);
      if (kit.authors) for (const author of kit.authors) this.createAuthor(author);
      if (kit.attributes) for (const attribute of kit.attributes) this.createAttribute(attribute);

      this.yKit.set("createdAt", new Date().toISOString());
      this.updated();
    });

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
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

  updated(): void {
    this.yKit.set("updatedAt", new Date().toISOString());
  }

  id = (): KitId => {
    return { name: this.name, version: this.version } as KitId;
  };

  hasType(type: TypeIdLike): boolean {
    return hasSameType(
      type,
      this.types.map((type) => type.id()),
    );
  }

  createType(type: Type): void {
    if (this.hasType(type)) throw new Error(`Type (${type.name}, ${type.variant || ""}) already exists.`);
    const yType = new Y.Map<YTypeVal>();
    const yTypeStore = new YTypeStore(this, yType, type);
    this.yTypes.push([yType]);
    this.types.push(yTypeStore);
  }

  type(type: TypeIdLike): YTypeStore {
    if (!this.hasType(type)) throw new Error(`Type store not found for type ${type}`);
    return this.types.find((t) => areSameType(t.id(), type))!;
  }

  typeByUuid(uuid: string): YTypeStore {
    return this.types.find((t) => t.uuid === uuid)!;
  }

  hasDesign(design: DesignIdLike): boolean {
    return hasSameDesign(
      design,
      this.designs.map((design) => design.id()),
    );
  }

  createDesign(design: Design): void {
    if (this.hasDesign(design)) throw new Error(`Design (${design.name}, ${design.variant || ""}, ${design.view || ""}) already exists.`);
    const yDesign = new Y.Map<YDesignVal>();
    const yDesignStore = new YDesignStore(this, yDesign, design);
    this.yDesigns.push([yDesign]);
    this.designs.push(yDesignStore);
  }

  design(design: DesignIdLike): YDesignStore {
    if (!this.hasDesign(design)) throw new Error(`Design store not found for design ${design}`);
    return this.designs.find((d) => areSameDesign(d.id(), design))!;
  }

  designByUuid(uuid: string): YDesignStore {
    return this.designs.find((d) => d.uuid === uuid)!;
  }

  hasFile(file: FileIdLike): boolean {
    return this.files.some((f) => f.id().path === (typeof file === "string" ? file : file.path));
  }

  createFile(file: SemioFile): void {
    if (this.hasFile(file)) throw new Error(`File (${file.path}) already exists.`);
    const yFile = new Y.Map<YFile>();
    const yFileStore = new YFileStore(yFile, file);
    this.yFiles.push([yFile]);
    this.files.push(yFileStore);
  }

  file(file: FileIdLike): YFileStore {
    if (!this.hasFile(file)) throw new Error(`File store not found for file ${file}`);
    return this.files.find((f) => f.id().path === (typeof file === "string" ? file : file.path))!;
  }

  fileByUuid(uuid: string): YFileStore {
    return this.files.find((f) => f.uuid === uuid)!;
  }

  hasQuality(quality: QualityIdLike): boolean {
    return this.qualities.some((q) => q.id.key === (typeof quality === "string" ? quality : quality.key));
  }

  createQuality(quality: Quality): void {
    if (this.hasQuality(quality)) throw new Error(`Quality (${quality.key}) already exists.`);
    const yQuality = new Y.Map<YQuality>();
    const yQualityStore = new YQualityStore(yQuality, quality);
    this.yQualities.push([yQuality]);
    this.qualities.push(yQualityStore);
  }

  quality(quality: QualityIdLike): YQualityStore {
    if (!this.hasQuality(quality)) throw new Error(`Quality store not found for quality ${quality}`);
    return this.qualities.find((q) => q.id.key === (typeof quality === "string" ? quality : quality.key))!;
  }

  qualityByUuid(uuid: string): YQualityStore {
    return this.qualities.find((q) => q.uuid === uuid)!;
  }

  hasBenchmark(benchmark: BenchmarkIdLike): boolean {
    return this.benchmarks.some((b) => b.id().name === (typeof benchmark === "string" ? benchmark : benchmark.name));
  }

  createBenchmark(benchmark: Benchmark): void {
    if (this.hasBenchmark(benchmark)) throw new Error(`Benchmark (${benchmark.name}) already exists.`);
    const yBenchmark = new Y.Map<YBenchmark>();
    const yBenchmarkStore = new YBenchmarkStore(yBenchmark, benchmark);
    this.yBenchmarks.push([yBenchmark]);
    this.benchmarks.push(yBenchmarkStore);
  }

  benchmark(benchmark: BenchmarkIdLike): YBenchmarkStore {
    if (!this.hasBenchmark(benchmark)) throw new Error(`Benchmark store not found for benchmark ${benchmark}`);
    return this.benchmarks.find((b) => b.id().name === (typeof benchmark === "string" ? benchmark : benchmark.name))!;
  }

  benchmarkByUuid(uuid: string): YBenchmarkStore {
    return this.benchmarks.find((b) => b.uuid === uuid)!;
  }

  hasAuthor(author: AuthorIdLike): boolean {
    return this.authors.some((a) => a.id.email === (typeof author === "string" ? author : author.email));
  }

  createAuthor(author: Author): void {
    if (this.hasAuthor(author)) throw new Error(`Author (${author.email}) already exists.`);
    const yAuthor = new Y.Map<YAuthor>();
    const yAuthorStore = new YAuthorStore(yAuthor, author);
    this.yAuthors.push([yAuthor]);
    this.authors.push(yAuthorStore);
  }

  author(author: AuthorIdLike): YAuthorStore {
    if (!this.hasAuthor(author)) throw new Error(`Author store not found for author ${author}`);
    return this.authors.find((a) => a.id.email === (typeof author === "string" ? author : author.email))!;
  }

  authorByUuid(uuid: string): YAuthorStore {
    return this.authors.find((a) => a.uuid === uuid)!;
  }

  hasAttribute(attribute: AttributeIdLike): boolean {
    return this.attributes.some((a) => a.id.key === (typeof attribute === "string" ? attribute : attribute.key));
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttribute>();
    const yAttributeStore = new YAttributeStore(yAttribute, attribute);
    this.yAttributes.push([yAttribute]);
    this.attributes.push(yAttributeStore);
  }

  attribute(attribute: AttributeIdLike): YAttributeStore {
    if (!this.hasAttribute(attribute)) throw new Error(`Attribute store not found for attribute ${attribute}`);
    return this.attributes.find((a) => a.id.key === (typeof attribute === "string" ? attribute : attribute.key))!;
  }

  attributeByUuid(uuid: string): YAttributeStore {
    return this.attributes.find((a) => a.uuid === uuid)!;
  }

  hash(kit: Kit): string {
    return JSON.stringify(kit);
  }

  snapshot = (): Kit => {
    const currentData = {
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
      types: this.types.map((type) => type.snapshot()),
      designs: this.designs.map((design) => design.snapshot()),
      qualities: this.qualities.map((quality) => quality.snapshot),
      files: this.files.map((file) => file.snapshot()),
      authors: this.authors.map((author) => author.id),
      attributes: this.attributes.map((attribute) => attribute.snapshot),
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
      if (diff.name) this.name = diff.name;
      if (diff.version) this.version = diff.version;
      if (diff.remote) this.remote = diff.remote;
      if (diff.homepage) this.homepage = diff.homepage;
      if (diff.license) this.license = diff.license;
      if (diff.types) {
        if (diff.types.added) {
          diff.types.added.forEach((type) => this.createType(type));
        }
      }
      if (diff.designs) {
        if (diff.designs.added) {
          diff.designs.added.forEach((design) => this.createDesign(design));
        }
      }
      this.yKit.set("updatedAt", new Date().toISOString());
      this.cache = undefined;
      this.cacheHash = undefined;
    });
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
      fileUrls: this.fileUrls,
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

// #endregion Kit

// #region Design Editor

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
export interface DesignEditorStore extends StoreWithCommands<DesignEditorState, DesignEditorId, DesignEditorDiff, DesignEditorCommandContext, DesignEditorCommandResult> {}

export const inverseDesignEditorSelectionDiff = (selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff => {
  // TODO
};
export const areSameDesignEditor = (designEditor: DesignEditorId, other: DesignEditorId): boolean => areSameKit(designEditor.kit, other.kit) && areSameDesign(designEditor.design, other.design);
export const hasSameDesignEditor = (designEditor: DesignEditorId, others: DesignEditorId[]): boolean => others.some((other) => areSameDesignEditor(designEditor, other));


class YDesignEditorStore {
  public readonly uuid: string;
  public readonly parent: YSketchpadStore;
  public readonly yMap: YDesignEditor;
  private readonly commandRegistry: Map<string, (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult> = new Map();
  private readonly transact: (fn: () => void) => void;
  private cache?: DesignEditorState;
  private cacheHash?: string;

  constructor(parent: YSketchpadStore, yMap: YDesignEditor, transact: (fn: () => void) => void, id: DesignEditorId, state?: DesignEditorState) {
    this.uuid = uuidv4();
    this.parent = parent;
    this.yMap = yMap;
    this.transact = transact;

    const kit = this.parent.kit(id.kit);
    const design = kit.design(id.design);
    yMap.set("kit", kit.uuid);
    yMap.set("design", design.uuid);

    yMap.set("fullscreenPanel", state?.fullscreenPanel || DesignEditorFullscreenPanel.None);

    const selection = new Y.Map<any>();
    const selectedPieces = new Y.Array<string>();
    if (state?.selection.pieces) {
      selectedPieces.push(state?.selection.pieces.map((piece) => pieceIdToString(piece)) || []);
    }
    const selectedConnections = new Y.Array<string>();
    if (state?.selection.connections) {
      selectedConnections.push(state?.selection.connections.map((connection) => connectionIdToString(connection)) || []);
    }
    const selectionPort = new Y.Map<any>();
    if (state?.selection.port) {
      selectionPort.set("piece", pieceIdToString(state?.selection.port.piece!));
      selectionPort.set("port", portIdToString(state?.selection.port.port!));
      selectionPort.set("designPiece", pieceIdToString(state?.selection.port.designPiece!));
    }
    selection.set("pieces", selectedPieces);
    selection.set("connections", selectedConnections);
    selection.set("port", selectionPort);
    yMap.set("selection", selection);

    yMap.set("isTransactionActive", false);
    yMap.set("presence", new Y.Map<any>());
    yMap.set("others", new Y.Array<any>());
    yMap.set("diff", new Y.Map<any>());
    yMap.set("currentTransactionStack", new Y.Array<any>());
    yMap.set("pastTransactionsStack", new Y.Array<any>());

    Object.entries(designEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenPanel(): DesignEditorFullscreenPanel {
    return this.yMap.get("fullscreenPanel") as DesignEditorFullscreenPanel;
  }
  set fullscreenPanel(panel: DesignEditorFullscreenPanel) {
    this.yMap.set("fullscreenPanel", panel);
  }
  get selection(): DesignEditorSelection {
    return {};
  }
  get isTransactionActive(): boolean {
    return (this.yMap.get("isTransactionActive") as boolean) || false;
  }
  set isTransactionActive(active: boolean) {
    this.yMap.set("isTransactionActive", active);
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

  canUndo(): boolean {
    return this.pastTransactionsStack.length > 0;
  }

  canRedo(): boolean {
    return this.currentTransactionStack.length > 0 && !this.isTransactionActive;
  }

  id(): DesignEditorId {
    const kit = this.parent.kitByUuid(this.yMap.get("kit") as string);
    const design = kit.designByUuid(this.yMap.get("design") as string);
    return {
      kit: kit.id(),
      design: design.id(),
    } as DesignEditorId;
  }

  hash(state: DesignEditorState): string {
    return JSON.stringify(state);
  }

  snapshot = (): DesignEditorState => {
    const currentData = {
      fullscreenPanel: this.fullscreenPanel,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      canUndo: this.canUndo(),
      canRedo: this.canRedo(),
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
    this.transact(() => {
      if (diff.fullscreenPanel) this.fullscreenPanel = diff.fullscreenPanel;
      if (diff.selection) {
        const selection = this.yMap.get("selection") as Y.Map<any>;
      }
    });
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
    this.isTransactionActive = true;
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
      this.isTransactionActive = false;
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
      this.isTransactionActive = false;
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

// #endregion Design Editor

// #region Sketchpad

type YSketchpadVal = string | boolean | YDesignEditors;
type YSketchpad = Y.Map<YSketchpadVal>;

type YSketchpadKeysMap = {
  mode: string;
  theme: string;
  layout: string;
  activeDesignEditorDesign: YDesign;
};

export interface SketchpadChangableState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditor?: DesignEditorId;
}
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
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
export interface SketchpadStore extends StoreWithCommands<SketchpadState, SketchpadId, SketchpadDiff, SketchpadCommandContext, SketchpadCommandResult> {
  kits: Map<string, KitStore>;
  designEditors: Map<string, Map<string, DesignEditorStore>>;
}

class YSketchpadStore {
  private readonly id: string | undefined;
  private readonly yProviderFactory: YProviderFactory | undefined;
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly kits: Array<YKitStore>;
  private readonly yDesignEditors: YDesignEditors;
  private readonly designEditors: Array<DesignEditorStore>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult>;
  private cache?: SketchpadState;
  private cacheHash?: string;
  // private readonly broadcastChannel: BroadcastChannel;

  private hash(state: SketchpadState): string {
    return JSON.stringify(state);
  }

  constructor(id?: string, yProviderFactory?: YProviderFactory) {
    this.id = id;
    this.yProviderFactory = yProviderFactory;
    // this.broadcastChannel = new BroadcastChannel(`semio-sketchpad-${id}`);
    this.yDoc = new Y.Doc();
    this.kits = new Array();
    this.designEditors = new Array();
    this.commandRegistry = new Map();

    // if (id) {
    //   this.persistence = new IndexeddbPersistence(`semio-sketchpad-${id}`, this.yDoc);
    //   this.persistence!.doc.on("update", () => {
    //     this.broadcastChannel.postMessage({ client: this.yDoc.clientID });
    //   });
    //   this.broadcastChannel.addEventListener("message", (msg) => {
    //     console.log("message", msg);
    //     const { data } = msg;
    //     if (data.client !== this.yDoc.clientID) {
    //     }
    //   });
    // } else {
    //   this.yDoc.on("update", (update: Uint8Array) => {
    //     this.broadcastChannel.postMessage({ client: this.yDoc.clientID, update });
    //   });
    //   this.broadcastChannel.addEventListener("message", (msg) => {
    //     const { data } = msg;
    //     if (data.client !== this.yDoc.clientID) {
    //       Y.applyUpdate(this.yDoc, data.update);
    //     }
    //   });
    // }

    // if (yProviderFactory) {
    //   yProviderFactory(this.yDoc, id);
    // }

    this.ySketchpad = this.yDoc.getMap("sketchpad");
    this.yDesignEditors = this.yDoc.getArray("designEditors");
    this.yDoc.transact(() => {
      this.ySketchpad.set("mode", Mode.GUEST);
      this.ySketchpad.set("theme", Theme.SYSTEM);
      this.ySketchpad.set("layout", Layout.NORMAL);
      this.ySketchpad.set("activeDesignEditor", "");
    });

    Object.entries(sketchpadCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  id = () => {
    return this.yDoc.clientID.toString();
  };

  snapshot = (): SketchpadState => {
    const activeDesignEditorIdStr = this.ySketchpad.get("activeDesignEditor") as string;
    const activeDesignEditor = activeDesignEditorIdStr ? (JSON.parse(activeDesignEditorIdStr) as DesignEditorId) : undefined;
    const currentValues = {
      mode: this.ySketchpad.get("mode") as Mode,
      theme: this.ySketchpad.get("theme") as Theme,
      layout: this.ySketchpad.get("layout") as Layout,
      activeDesignEditor: activeDesignEditor,
    };
    const currentHash = this.hash(currentValues);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentValues;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  createKit = (kit: Kit) => {
    if (this.hasKit(kit)) {
      throw new Error(`Kit (${kit.name}, ${kit.version || ""}) already exists.`);
    }
    this.kits.push(new YKitStore(this, kit));
  };

  createDesignEditor = (id: DesignEditorId) => {
    if (this.hasDesignEditor(id)) {
      throw new Error(`Design editor (${id.kit.name}, ${id.kit.version || ""}, ${id.design.name}, ${id.design.variant || ""}, ${id.design.view || ""}) already exists.`);
    }
    this.yDoc.transact(() => {
      const yDesignEditor = new Y.Map<YDesignEditorVal>();
      this.yDesignEditors.push([yDesignEditor]);
      const designEditor = new YDesignEditorStore(this, yDesignEditor, this.yDoc.transact, id);
      this.designEditors.push(designEditor);
    });
  };

  change(diff: SketchpadDiff) {
    this.yDoc.transact(() => {
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.layout) this.ySketchpad.set("layout", diff.layout);
      if (diff.activeDesignEditor) this.ySketchpad.set("activeDesignEditor", JSON.stringify(diff.activeDesignEditor));
    });
  }

  deleteKit = (id: KitIdLike) => {};

  deleteDesignEditor = (id: DesignEditorId) => {};

  // Subscriptions using Yjs observers
  onKitCreated = (subscribe: Subscribe): Unsubscribe => {};

  onDesignEditorCreated = (subscribe: Subscribe): Unsubscribe => {};

  onKitDeleted = (subscribe: Subscribe): Unsubscribe => {};

  onDesignEditorDeleted = (subscribe: Subscribe): Unsubscribe => {};

  onChanged = (subscribe: Subscribe): Unsubscribe => {
    const observer = () => subscribe();
    this.ySketchpad.observe(observer);
    return () => {
      this.ySketchpad.unobserve(observer);
    };
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {};

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.sketchpad.createKit") {
      const kit = rest[0] as Kit;
      if (!kit.name) throw new Error("Kit name is required to create a kit.");
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

  hasKit(kit: KitIdLike): boolean {
    return hasSameKit(
      kit,
      Array.from(this.kits.values()).map((kit) => kit.id()),
    );
  }

  kit(kit: KitIdLike): YKitStore {
    if (!this.hasKit(kit)) throw new Error(`Kit store not found for kit ${kit}`);
    return this.kits.find((k) => areSameKit(k.id(), kit))!;
  }

  kitByUuid(uuid: string): YKitStore {
    return this.kits.find((k) => k.uuid === uuid)!;
  }

  hasDesignEditor(designEditor: DesignEditorId): boolean {
    return hasSameDesignEditor(
      designEditor,
      Array.from(this.designEditors.values()).map((designEditor) => designEditor.id()),
    );
  }

  designEditor(designEditor: DesignEditorId): YDesignEditorStore {
    if (!this.hasDesignEditor(designEditor)) throw new Error(`Design editor store not found for design editor ${designEditor}`);
    return this.designEditors.find((d) => areSameDesignEditor(d.id(), designEditor))!;
  }

  designEditorByUuid(uuid: string): YDesignEditorStore {
    return this.designEditors.find((d) => d.uuid === uuid)!;
  }
}

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

// #endregion Sketchpad

const stores: Map<string, YSketchpadStore> = new Map();

// #region Hooks

// #region Scoping

type SketchpadScope = { id: string; yProviderFactory?: YProviderFactory };
const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
export const SketchpadScopeProvider = (props: { id?: string; yProviderFactory?: YProviderFactory; children: React.ReactNode }) => {
  const id = props.id || uuidv4();
  if (!stores.has(id)) {
    const store = new YSketchpadStore(props.id, props?.yProviderFactory);
    stores.set(id, store);
  }
  return React.createElement(SketchpadScopeContext.Provider, { value: { id } }, props.children as any);
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

function useSync<TModel, TId, TDiff, TSelected = TModel>(store: Store<TModel, TId, TDiff>, selector?: (state: TModel) => TSelected, deep: boolean = false): TModel | TSelected {
  const state = deep ? useSyncExternalStore(store.onChangedDeep, store.snapshot) : useSyncExternalStore(store.onChanged, store.snapshot);
  return selector ? selector(state) : state;
}

function useSyncDeep<TModel, TId, TDiff, TSelected = TModel>(store: Store<TModel, TId, TDiff>, selector?: (state: TModel) => TSelected): TModel | TSelected {
  const state = useSyncExternalStore(store.onChangedDeep, store.snapshot) as TModel;
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
  if (!store.hasKit(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kit(kitId);
  return selector ? selector(kitStore) : kitStore;
}

export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, id?: KitId, deep: boolean = false): T | KitShallow | Kit {
  if (deep) {
    return useSyncDeep<Kit, KitId, KitDiff, T>(useKitStore(identitySelector, id) as KitStore, selector ? selector : identitySelector);
  }
  return useSync<KitShallow, KitId, KitDiff, T>(useKitStore(identitySelector, id) as KitStore, selector ? selector : identitySelector, deep);
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
  const designScope = useDesignScope();
  const resolvedDesignId = designScope?.id ?? id?.design;
  if (!resolvedDesignId) throw new Error("useDesignEditorStore must be called within a DesignScopeProvider or be directly provided with an id");
  const designEditorStore = store.designEditor({ kit: resolvedKitId, design: resolvedDesignId });
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
  if (!kitStore.hasDesign(designId)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kitStore.design(designId);
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: DesignId, deep: boolean = false): T | DesignShallow | Design {
  if (deep) {
    return useSyncDeep<Design, DesignId, DesignDiff, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector);
  }
  return useSync<DesignShallow, DesignId, DesignDiff, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector, deep);
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
  if (!kitStore.hasType(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kitStore.type(typeId);
  return selector ? selector(typeStore) : typeStore;
}

export function useType<T>(selector?: (type: Type) => T, id?: TypeId, deep: boolean = false): T | Type {
  return useSync<Type, TypeId, TypeDiff, T>(useTypeStore(identitySelector, id) as TypeStore, selector ? selector : identitySelector, deep);
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
  const pieceStore = designStore.piece(pieceId);
  if (!pieceStore) throw new Error(`Piece store not found for piece ${pieceId}`);
  return selector ? selector(pieceStore) : pieceStore;
}

export function usePiece<T>(selector?: (piece: Piece) => T, id?: PieceId, deep: boolean = false): T | Piece {
  return useSync<Piece, PieceId, PieceDiff, T>(usePieceStore(identitySelector, id) as PieceStore, selector ? selector : identitySelector, deep);
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
  if (!designStore.hasConnection(connectionId)) throw new Error(`Connection store not found for connection ${connectionId}`);
  const connectionStore = designStore.connection(connectionId);
  return selector ? selector(connectionStore) : connectionStore;
}

export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId, deep: boolean = false): T | Connection {
  return useSync<Connection, ConnectionId, ConnectionDiff, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ? selector : identitySelector, deep);
}

function usePortStore<T>(selector?: (store: PortStore) => T, id?: PortId): T | PortStore {
  const typeStore = useTypeStore() as TypeStore;
  const portScope = usePortScope();
  const portId = portScope?.id ?? id;
  if (!portId) throw new Error("usePortStore must be called within a PortScopeProvider or be directly provided with an id");
  if (!typeStore.hasPort(portId)) throw new Error(`Port store not found for port ${portId}`);
  const portStore = typeStore.port(portId);
  return selector ? selector(portStore) : portStore;
}

export function usePort<T>(selector?: (port: Port) => T, id?: PortId, deep: boolean = false): T | Port {
  return useSync<Port, PortId, PortDiff, T>(usePortStore(identitySelector, id) as PortStore, selector ? selector : identitySelector, deep);
}

function useRepresentationStore<T>(selector?: (store: RepresentationStore) => T, id?: RepresentationId): T | RepresentationStore {
  const typeStore = useTypeStore() as TypeStore;
  const representationScope = useRepresentationScope();
  const representationId = representationScope?.id ?? id;
  if (!representationId) throw new Error("useRepresentationStore must be called within a RepresentationScopeProvider or be directly provided with an id");
  if (!typeStore.hasRepresentation(representationId)) throw new Error(`Representation store not found for representation ${representationId}`);
  const representationStore = typeStore.representation(representationId);
  return selector ? selector(representationStore) : representationStore;
}

export function useRepresentation<T>(selector?: (representation: Representation) => T, id?: RepresentationId, deep: boolean = false): T | Representation {
  return useSync<Representation, RepresentationId, RepresentationDiff, T>(useRepresentationStore(identitySelector, id) as RepresentationStore, selector ? selector : identitySelector, deep);
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
