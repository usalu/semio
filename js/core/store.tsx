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
import { useNavigate } from "react-router";
import type { Database, SqlJsStatic } from "sql.js";
import initSqlJs from "sql.js";
import sqlWasmUrl from "sql.js/dist/sql-wasm.wasm?url";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { guid } from "./lib/utils";
import {
  applyDesignDiff,
  applyKitDiff,
  Attribute,
  Author,
  AuthorDiff,
  AuthorId,
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
  DesignId,
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
  Group,
  GroupDiff,
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
  QualityId,
  Representation,
  RepresentationDiff,
  File as SemioFile,
  Side,
  SideDiff,
  Stat,
  StatDiff,
  Type,
  TypeDiff,
  Vec,
  VecDiff,
  Vector,
  VectorDiff,
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

export enum EditorType {
  HOME = "home",
  KIT = "kit",
  DESIGN = "design",
  TYPE = "type",
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

type YConcept = string;
type YConcepts = Y.Array<string>;

type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;

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

export interface Synchronizable<TModel> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TModel;
}

const identitySelector = (state: any) => state;

function useSync<TModel, TSelected = TModel>(store: Synchronizable<TModel>, selector?: (state: TModel) => TSelected, deep: boolean = false): TModel | TSelected {
  const state = deep ? useSyncExternalStore(store.onChangedDeep, store.snapshot) : useSyncExternalStore(store.onChanged, store.snapshot);
  return selector ? selector(state) : state;
}

function useSyncDeep<TModel, TSelected = TModel>(store: Synchronizable<TModel>, selector?: (state: TModel) => TSelected): TModel | TSelected {
  const state = useSyncExternalStore(store.onChangedDeep, store.snapshot) as TModel;
  return selector ? selector(state) : state;
}

// #endregion General

// #region Attribute

type YAttributeVal = string;
type YAttribute = Y.Map<YAttributeVal>;
type YAttributes = Y.Array<YAttribute>;

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
    this.x = coord.x;
    this.y = coord.y;
  }

  get x(): number {
    return this.yCoord.get("x") as number;
  }
  set x(x: number) {
    this.yCoord.set("x", x);
  }

  get y(): number {
    return this.yCoord.get("y") as number;
  }
  set y(y: number) {
    this.yCoord.set("y", y);
  }

  hash = (coord: Coord): string => {
    return JSON.stringify(coord);
  };

  snapshot = (): Coord => {
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

  change = (diff: CoordDiff) => {
    if (diff.x !== undefined) this.x = diff.x;
    if (diff.y !== undefined) this.y = diff.y;
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

type YCameraVal = YPoint | YVector | number;
type YCamera = Y.Map<YCameraVal>;

class YCameraStore {
  private yCamera: YCamera;
  private cache?: Camera;
  private cacheHash?: string;

  constructor(yCamera: YCamera, camera: Camera) {
    this.yCamera = yCamera;
  }

  hash = (camera: Camera): string => {
    return JSON.stringify(camera);
  };

  snapshot = (): Camera => {
    const currentData = {};
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: CameraDiff) => {
    if (diff.distance !== undefined) this.distance = diff.distance;
    if (diff.phi !== undefined) this.phi = diff.phi;
    if (diff.theta !== undefined) this.theta = diff.theta;
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

type YLocationVal = number;
type YLocation = Y.Map<YLocationVal>;

class YLocationStore {
  private yLocation: YLocation;
  private cache?: Location;
  private cacheHash?: string;

  constructor(yLocation: YLocation, location: Location) {
    this.yLocation = yLocation;
    this.latitude = location.latitude;
    this.longitude = location.longitude;
    this.altitude = location.altitude;
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
    this.yLocation.set("altitude", altitude);
  }

  hash = (location: Location): string => {
    return JSON.stringify(location);
  };

  snapshot = (): Location => {
    const currentData = {
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

function useAuthorStore<T>(selector?: (store: AuthorStore) => T, guid?: string): T | AuthorStore {
  const kitStore = useKitStore() as KitStore;
  const authorScope = useAuthorScope();
  const authorGuid = authorScope?.guid ?? guid;
  if (!authorGuid) throw new Error("useAuthorStore must be called within a AuthorScopeProvider or be directly provided with a guid");
  if (!kitStore.hasAuthor(authorGuid)) throw new Error(`Author store not found for author ${authorGuid}`);
  const authorStore = kitStore.author(authorGuid);
  return selector ? selector(authorStore) : authorStore;
}

export function useAuthor<T>(selector?: (author: Author) => T, id?: AuthorId, deep: boolean = false): T | Author {
  return useSync<Author, T>(useAuthorStore(identitySelector, id) as AuthorStore, selector ? selector : identitySelector, deep);
}

// #endregion Author

// #region File

type YFile = Y.Map<string | YAttributes>;
type YFiles = Y.Array<YFile>;

class FileStore {
  private yFile: YFile;
  private cache?: SemioFile;
  private cacheHash?: string;

  constructor(yFile: YFile, file: SemioFile) {
    this.yFile = yFile;

    this.guid = file.guid;
    this.path = file.path;
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
    return this.yFile.get("createdBy") as string | undefined;
  }
  set createdBy(createdBy: AuthorId | undefined) {
    this.yFile.set("createdBy", createdBy || "");
  }
  get updatedBy(): AuthorId | undefined {
    return this.yFile.get("updatedBy") as string | undefined;
  }
  set updatedBy(updatedBy: AuthorId | undefined) {
    this.yFile.set("updatedBy", updatedBy || "");
  }

  hashFile = (file: SemioFile): string => {
    return JSON.stringify(file);
  };

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

type YBenchmark = Y.Map<string | number | YAttributes>;
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

type YQuality = Y.Map<string | number | YAttributes>;
type YQualities = Y.Array<YQuality>;

class QualityStore {
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

  id(): QualityId {
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

  id(): PropId {
    return { key: this.key };
  }

  hash = (prop: Prop): string => {
    return JSON.stringify(prop);
  };

  snapshot(): Prop {
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

type YRepresentationVal = string | YStringArray | YAttributes;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentations = Y.Array<YRepresentation>;

class RepresentationStore {
  private yRepresentation: YRepresentation;
  private yTags: YStringArray;
  private yAttributes: YAttributes;
  private attributes: Attribute[];
  private cache?: Representation;
  private cacheHash?: string;

  constructor(yRepresentation: YRepresentation, representation: Representation) {
    this.yRepresentation = yRepresentation;
    this.guid = representation.guid;
    this.url = representation.url;
    this.description = representation.description;
    this.yTags = this.yRepresentation.set("tags", new Y.Array<string>());
    if (representation.tags) this.yTags.push(representation.tags);
    this.attributes = new Array();
    this.yAttributes = this.yRepresentation.set("attributes", new Y.Array<YAttribute>());
    if (representation.attributes) {
      for (const attribute of representation.attributes) {
        this.createAttribute(attribute);
      }
    }
  }

  get guid(): string {
    return this.yRepresentation.get("guid") as string;
  }
  set guid(guid: string) {
    this.yRepresentation.set("guid", guid);
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

  hash = (representation: Representation): string => {
    return JSON.stringify(representation);
  };

  snapshot(): Representation {
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

  apply(diff: RepresentationDiff): void {
    if (diff.url !== undefined) this.url = diff.url;
    if (diff.description !== undefined) this.description = diff.description;
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

type YPortVal = string | number | boolean | YLeafMapNumber | YAttributes | YStringArray | YPoint | YVector | YProps;
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
    this.localId = port.id_;
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
    this.yPort.set("mandatory", mandatory);
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
      id_: this.id_,
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
    if (diff.id_ !== undefined) this.id_ = diff.id_;
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

class TypeStore {
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
    this.variant = type.variant;
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

  hasAttribute(guid: string): boolean {
    return this.attributes.some((a) => a.guid === guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.push(yAttributeStore);
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
    if (this.hasPort(port.guid)) throw new Error(`Port (${port.id_}) already exists.`);
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

  hash = (type: Type): string => {
    return JSON.stringify(type);
  };
  snapshot = (): Type => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      variant: this.variant,
      stock: this.stock,
      virtual: this.virtual,
      unit: this.unit,
      icon: this.icon,
      image: this.image,
      description: this.description,
      authors: Array.from(this.authors.values()).map((a) => a.guid),
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
    if (diff.stock !== undefined) this.yType.set("stock", diff.stock);
    if (diff.virtual !== undefined) this.yType.set("virtual", diff.virtual);
    if (diff.unit !== undefined) this.yType.set("unit", diff.unit);
    if (diff.icon !== undefined) this.yType.set("icon", diff.icon);
    if (diff.image !== undefined) this.yType.set("image", diff.image);
    if (diff.description !== undefined) this.yType.set("description", diff.description);
    if (diff.createdAt !== undefined) this.yType.set("createdAt", diff.createdAt);
    if (diff.updatedAt !== undefined) this.yType.set("updatedAt", diff.updatedAt);

    if (diff.authors !== undefined) {
      this.yAuthors.delete(0, this.yAuthors.length);
      this.authors = diff.authors.map((author) => this.parent.author(author));
      this.authors.forEach((author) => this.yAuthors.push([author.guid]));
    }

    // TODO: Handle location, representations, ports, props, attributes diffs

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

type TypeScope = { guid: string };
const TypeScopeContext = createContext<TypeScope | null>(null);
export const TypeScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(TypeScopeContext.Provider, { value }, props.children as any);
};
const useTypeScope = () => useContext(TypeScopeContext);

function useTypeStore<T>(selector?: (store: TypeStore) => T, guid?: string): T | TypeStore {
  const kitStore = useKitStore() as KitStore;
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? guid;
  if (!typeGuid) throw new Error("useTypeStore must be called within a TypeScopeProvider or be directly provided with a guid");
  if (!kitStore.hasType(typeGuid)) throw new Error(`Type store not found for type ${typeGuid}`);
  const typeStore = kitStore.type(typeGuid);
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
          const update = colorDiff.updated?.find((u) => u.id === type.guid);
          return update ? { ...type, ports: update.diff.ports } : type;
        })
      : diffedKit.types;
  }, [diffedKit.types]);
  const unified = useMemo(() => ({ ...diffedKit, types: typesWithColoredPorts }), [diffedKit, typesWithColoredPorts]);
  return unified.types;
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

  id(): LayerId {
    return { path: this.path };
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

type YPieceVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YPlane | YAttributes | YCoord;
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

    this.localId = piece.id_;
    if (piece.type) {
      const type = this.parent.parent.type(piece.type);
      this.yPiece.set("type", type.guid);
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
  get type(): TypeId | undefined {
    const typeUuid = this.yPiece.get("type") as string;
    return typeUuid ? this.parent.parent.type(typeUuid).id() : undefined;
  }
  set type(type: TypeId | undefined) {
    if (type) {
      this.yPiece.set("type", this.parent.parent.type(type).guid);
    } else {
      this.yPiece.delete("type");
    }
  }
  get design(): DesignId | undefined {
    const designUuid = this.yPiece.get("design") as string;
    return designUuid ? this.parent.parent.design(designUuid).id() : undefined;
  }
  set design(design: DesignId | undefined) {
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

  public hash(piece: Piece): string {
    return JSON.stringify(piece);
  }

  snapshot = (): Piece => {
    const currentData = {
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
    if (diff.id_) this.localId = diff.id_;
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
          const yPlane = new Y.Map();
          this.yPiece.set("plane", yPlane);
          this.yPlane = yPlane;
          this.plane = new YPlaneStore(this.yPlane, diff.plane);
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
          const yCenter = new Y.Map();
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
          const yMirrorPlane = new Y.Map();
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

type PieceScope = { guid: string };
const PieceScopeContext = createContext<PieceScope | null>(null);
export const PieceScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(PieceScopeContext.Provider, { value }, props.children as any);
};
const usePieceScope = () => useContext(PieceScopeContext);

function usePieceStore<T>(selector?: (store: PieceStore) => T, guid?: string): T | PieceStore {
  const designStore = useDesignStore() as DesignStore;
  const pieceScope = usePieceScope();
  const pieceGuid = pieceScope?.guid ?? guid;
  if (!pieceGuid) throw new Error("usePieceStore must be called within a PieceScopeProvider or be directly provided with a guid");
  const pieceStore = designStore.piece(pieceGuid);
  if (!pieceStore) throw new Error(`Piece store not found for piece ${pieceGuid}`);
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

export function usePieceStatus(): DiffStatus {
  const piece = usePieceScope();
  const kitDiff = useDesignEditorDiff();
  const designScope = useDesignScope();

  if (!piece || !designScope || !kitDiff?.designs?.updated) {
    return DiffStatus.Unchanged;
  }

  // Find the design diff for the current design
  const designDiff = kitDiff.designs.updated.find((d) => d.id.guid === designScope.id.guid);

  if (!designDiff?.diff.pieces) {
    return DiffStatus.Unchanged;
  }

  const piecesDiff = designDiff.diff.pieces;

  // Check if piece is removed
  if (piecesDiff.removed?.some((p) => p.id_ === piece.id.id_)) {
    return DiffStatus.Removed;
  }

  // Check if piece is added
  if (piecesDiff.added?.some((p) => p.id_ === piece.id.id_)) {
    return DiffStatus.Added;
  }

  // Check if piece is updated
  if (piecesDiff.updated?.some((p) => p.id.id_ === piece.id.id_)) {
    return DiffStatus.Modified;
  }

  return DiffStatus.Unchanged;
}

// #endregion Piece

// #region Group

type YGroupVal = string | YStringArray | YAttributes;
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
        group.pieces.map((p) => p.id_),
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

  get pieces(): PieceId[] {
    const yPieces = this.yGroup.get("pieces") as Y.Array<string> | undefined;
    if (!yPieces) return [];
    return yPieces.toArray().map((id_) => ({ id_ }));
  }
  set pieces(pieces: PieceId[]) {
    const yPieces = this.yGroup.get("pieces") as Y.Array<string> | undefined;
    if (yPieces) {
      yPieces.delete(0, yPieces.length);
      yPieces.insert(
        0,
        pieces.map((p) => p.id_),
      );
    } else {
      const newYPieces = new Y.Array<string>();
      newYPieces.insert(
        0,
        pieces.map((p) => p.id_),
      );
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
    const pieceStore = this.parent.pieces.find((p) => areSamePiece(p.id(), side.piece));
    if (pieceStore) {
      this.ySide.set("piece", pieceStore.guid);
    }

    // Store designPiece UUID if present
    if (side.designPiece) {
      const designPieceStore = this.parent.pieces.find((p) => areSamePiece(p.id(), side.designPiece!));
      if (designPieceStore) {
        this.ySide.set("designPiece", designPieceStore.guid);
      }
    }

    // Store port UUID - need to find it through the piece's type
    if (pieceStore) {
      const typeId = pieceStore.type;
      const typeStore = this.parent.parent.type(typeId);
      const portStore = typeStore.ports.get(side.port.id_);
      if (portStore) {
        this.ySide.set("port", portStore.guid);
      }
    }
  }

  get guid(): string {
    return this.ySide.get("guid") as string;
  }
  set guid(guid: string) {
    this.ySide.set("guid", guid);
  }

  get piece(): PieceId {
    const pieceUuid = this.ySide.get("piece") as string;
    return this.parent.piece(pieceUuid).id();
  }
  set piece(piece: PieceId) {
    const pieceStore = this.parent.pieces.find((p) => areSamePiece(p.id(), piece));
    if (pieceStore) {
      this.ySide.set("piece", pieceStore.guid);
    }
  }

  get designPiece(): PieceId | undefined {
    const designPieceUuid = this.ySide.get("designPiece") as string | undefined;
    if (!designPieceUuid) return undefined;
    return this.parent.piece(designPieceUuid).id();
  }
  set designPiece(designPiece: PieceId | undefined) {
    if (designPiece) {
      const designPieceStore = this.parent.pieces.find((p) => areSamePiece(p.id(), designPiece));
      if (designPieceStore) {
        this.ySide.set("designPiece", designPieceStore.guid);
      }
    } else {
      this.ySide.delete("designPiece");
    }
  }

  get port(): PortId {
    const portUuid = this.ySide.get("port") as string;
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeId = pieceStore.type;
    const typeStore = this.parent.parent.type(typeId);
    const portStore = typeStore.port(portUuid);
    return portStore.id();
  }
  set port(port: PortId) {
    // Find the port through the piece's type
    const pieceUuid = this.ySide.get("piece") as string;
    const pieceStore = this.parent.piece(pieceUuid);
    const typeId = pieceStore.type;
    const typeStore = this.parent.parent.type(typeId);
    const portStore = typeStore.ports.get(port.id_);
    if (portStore) {
      this.ySide.set("port", portStore.guid);
    }
  }

  hash = (side: Side): string => {
    return JSON.stringify(side);
  };

  snapshot = (): Side => {
    const currentData = {
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

  id = (): SideId => {
    return { piece: this.piece, designPiece: this.designPiece };
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
    const yConnected = new Y.Map<YSideVal>();
    this.connected = new SideStore(parent, yConnected, connection.connected);
    const yConnecting = new Y.Map<YSideVal>();
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
    this.yConnection.set("gap", gap);
  }

  get shift(): number | undefined {
    return this.yConnection.get("shift") as number | undefined;
  }
  set shift(shift: number | undefined) {
    this.yConnection.set("shift", shift);
  }

  get rise(): number | undefined {
    return this.yConnection.get("rise") as number | undefined;
  }
  set rise(rise: number | undefined) {
    this.yConnection.set("rise", rise);
  }

  get rotation(): number | undefined {
    return this.yConnection.get("rotation") as number | undefined;
  }
  set rotation(rotation: number | undefined) {
    this.yConnection.set("rotation", rotation);
  }

  get turn(): number | undefined {
    return this.yConnection.get("turn") as number | undefined;
  }
  set turn(turn: number | undefined) {
    this.yConnection.set("turn", turn);
  }

  get tilt(): number | undefined {
    return this.yConnection.get("tilt") as number | undefined;
  }
  set tilt(tilt: number | undefined) {
    this.yConnection.set("tilt", tilt);
  }

  get x(): number | undefined {
    return this.yConnection.get("x") as number | undefined;
  }
  set x(x: number | undefined) {
    this.yConnection.set("x", x);
  }

  get y(): number | undefined {
    return this.yConnection.get("y") as number | undefined;
  }
  set y(y: number | undefined) {
    this.yConnection.set("y", y);
  }

  hash = (connection: Connection): string => {
    return JSON.stringify(connection);
  };

  snapshot = (): Connection => {
    const currentData = {
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
      attributes: this.attributes.map((attr) => attr.snapshot()),
    };
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  hasAttribute(guid: string): boolean {
    return this.attributes.some((a) => a.guid === guid);
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttributeVal>();
    this.yAttributes.push([yAttribute]);
    const yAttributeStore = new AttributeStore(yAttribute, attribute);
    this.attributes.push(yAttributeStore);
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
const useConnectionScope = () => useContext(ConnectionScopeContext);

function useConnectionStore<T>(selector?: (store: ConnectionStore) => T, guid?: string): T | ConnectionStore {
  const designStore = useDesignStore() as DesignStore;
  const connectionScope = useConnectionScope();
  const connectionGuid = connectionScope?.guid ?? guid;
  if (!connectionGuid) throw new Error("useConnectionStore must be called within a ConnectionScopeProvider or be directly provided with a guid");
  const connectionStore = designStore.connection(connectionGuid);
  if (!connectionStore) throw new Error(`Connection store not found for connection ${connectionGuid}`);
  return selector ? selector(connectionStore) : connectionStore;
}

export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId, deep: boolean = false): T | Connection {
  return useSync<Connection, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ? selector : identitySelector, deep);
}

export function useIsConnectionSelected(): boolean {
  const connection = useConnectionScope();
  const selection = useDesignEditorSelection();
  return selection.connections?.some((c) => c.connected.piece.id_ === connection?.id.connected.piece.id_ && c.connecting.piece.id_ === connection?.id.connecting.piece.id_) ?? false;
}

export function useIsConnectionHovered(): boolean {
  // const hover = useDesignEditorHover();
  // return hover.connection?.id_ === connection.id_ ?? false;
  return false;
}

export function useConnectionStatus(): DiffStatus {
  const connection = useConnectionScope();
  const kitDiff = useDesignEditorDiff();
  const designScope = useDesignScope();

  if (!connection || !designScope || !kitDiff?.designs?.updated) {
    return DiffStatus.Unchanged;
  }

  // Find the design diff for the current design
  const designDiff = kitDiff.designs.updated.find((d) => d.id.guid === designScope.id.guid);

  if (!designDiff?.diff.connections) {
    return DiffStatus.Unchanged;
  }

  const connectionsDiff = designDiff.diff.connections;

  // Check if connection is removed
  if (connectionsDiff.removed?.some((c) => c.id_ === connection.id.id_)) {
    return DiffStatus.Removed;
  }

  // Check if connection is added
  if (connectionsDiff.added?.some((c) => c.id_ === connection.id.id_)) {
    return DiffStatus.Added;
  }

  // Check if connection is updated
  if (connectionsDiff.updated?.some((c) => c.id.id_ === connection.id.id_)) {
    return DiffStatus.Modified;
  }

  return DiffStatus.Unchanged;
}

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

  id(): StatId {
    return { key: this.key };
  }

  hash = (stat: Stat): string => {
    return JSON.stringify(stat);
  };

  snapshot(): Stat {
    const currentData = {
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

type YDesignVal = string | YAuthorUuids | YAttributes | YPieces | YConnections | YLayers | YGroups | YStats;
type YDesign = Y.Map<YDesignVal>;
type YDesigns = Y.Array<YDesign>;

class DesignStore {
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
  private groups: Map<string, GroupStore>;
  private yGroups: YGroups;
  private location?: YLocationStore;
  private yAuthors: YAuthorUuids;
  private authors: Map<string, AuthorStore>;
  private yConcepts: YConcepts;
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
    this.variant = design.variant;
    this.view = design.view;
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

    // if (design.stats) {
    //   const yStats = new Y.Array<YStat>();
    //   this.yDesign.set("stats", yStats);
    //   for (const stat of design.stats) {
    //     this.createStat(stat);
    //   }
    // }

    // if (design.props) {
    //   const yProps = new Y.Array<YProp>();
    //   this.yDesign.set("props", yProps);
    //   for (const prop of design.props) {
    //     this.createProp(prop);
    //   }
    // }

    // if (design.layers) {
    //   const yLayers = new Y.Array<YLayer>();
    //   this.yDesign.set("layers", yLayers);
    //   for (const layer of design.layers) {
    //     this.createLayer(layer);
    //   }
    // }

    // if (design.activeLayer) {
    //   this.yDesign.set("activeLayer", design.activeLayer.path || "");
    // }

    // if (design.groups) {
    //   const yGroups = new Y.Array<YGroup>();
    //   this.yDesign.set("groups", yGroups);
    //   for (const group of design.groups) {
    //     this.createGroup(group);
    //   }
    // }

    // if (design.location) {
    //   const yLocation = new Y.Map();
    //   this.yDesign.set("location", yLocation);
    //   this.location = new YLocationStore(yLocation, design.location);
    // }

    // if (design.concepts) {
    //   const yConcepts = new Y.Array<string>();
    //   design.concepts.forEach((concept) => yConcepts.push([concept]));
    //   this.yDesign.set("concepts", yConcepts);
    // }

    this.authors = design.authors?.map((author) => this.parent.author(author)) || [];
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
    const yStat = new Y.Map<YStatVal>();
    this.yStats.push([yStat]);
    const yStatStore = new StatStore(yStat, stat);
    this.stats.set(stat.key, yStatStore);
  }

  createProp(prop: Prop): void {
    const yProp = new Y.Map<YPropVal>();
    this.yProps.push([yProp]);
    const yPropStore = new PropStore(yProp, prop);
    this.props.set(prop.key, yPropStore);
  }

  createLayer(layer: Layer): void {
    const yLayer = new Y.Map<YLayerVal>();
    this.yLayers.push([yLayer]);
    const yLayerStore = new LayerStore(yLayer, layer);
    this.layers.set(layer.guid, yLayerStore);
  }

  createGroup(group: Group): void {
    const yGroup = new Y.Map<YGroupVal>();
    this.yGroups.push([yGroup]);
    const yGroupStore = new GroupStore(yGroup, group);
    this.groups.set(group.guid, yGroupStore);
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

  hash(design: Design): string {
    return JSON.stringify(design);
  }

  snapshot = (): Design => {
    const currentData = {
      guid: this.guid,
      name: this.name,
      variant: this.variant,
      view: this.view,
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
      activeLayer: this.yDesign.get("activeLayer") ? { path: this.yDesign.get("activeLayer") as string } : undefined,
      groups: Array.from(this.groups.values()).map((group) => group.snapshot()),
      location: this.location?.snapshot(),
      authors: Array.from(this.authors.values()),
      concepts: (this.yDesign.get("concepts") as Y.Array<string> | undefined)?.toArray(),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot()),
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
    if (diff.authors !== undefined) this.authors = diff.authors;

    if (diff.pieces !== undefined) {
      if (typeof diff.pieces === "object" && !Array.isArray(diff.pieces)) {
        // Handle incremental updates
        if (diff.pieces.added) {
          diff.pieces.added.forEach((piece) => this.createPiece(piece));
        }
        if (diff.pieces.updated) {
          diff.pieces.updated.forEach(({ id, diff: pieceDiff }) => {
            const pieceStore = this.pieces.find((p) => areSamePiece(p.id(), id));
            if (pieceStore) {
              pieceStore.change(pieceDiff);
            }
          });
        }
        if (diff.pieces.removed) {
          diff.pieces.removed.forEach((pieceId) => {
            const pieceIndex = this.pieces.findIndex((p) => areSamePiece(p.id(), pieceId));
            if (pieceIndex !== -1) {
              this.pieces.splice(pieceIndex, 1);
              this.yPieces!.delete(pieceIndex, 1);
            }
          });
        }
      } else {
        // Handle complete replacement (legacy behavior)
        this.pieces = [];
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
            const connectionStore = this.connections.find((c) => c.id.id_ === id.id_ || c.id.id_ === id);
            if (connectionStore) {
              connectionStore.change(connectionDiff);
            }
          });
        }
        if (diff.connections.removed) {
          diff.connections.removed.forEach((connectionId) => {
            const connectionIndex = this.connections.findIndex((c) => c.id.id_ === connectionId.id_ || c.id.id_ === connectionId);
            if (connectionIndex !== -1) {
              this.connections.splice(connectionIndex, 1);
              this.yConnections.delete(connectionIndex, 1);
            }
          });
        }
      } else {
        // Handle complete replacement (legacy behavior)
        this.connections = [];
        this.yConnections.delete(0, this.yConnections.length);

        if (diff.connections) {
          for (const connection of diff.connections as Connection[]) {
            this.createConnection(connection);
          }
        }
      }
    }

    if (diff.stats !== undefined) {
      if (diff.stats) {
        // Clear existing stats
        this.stats.clear();
        const yStats = this.yDesign.get("stats") as Y.Array<YStat>;
        if (yStats) {
          yStats.delete(0, yStats.length);
        } else {
          const newYStats = new Y.Array<YStat>();
          this.yDesign.set("stats", newYStats);
        }
        for (const stat of diff.stats) {
          this.createStat(stat);
        }
      } else {
        this.yDesign.delete("stats");
        this.stats.clear();
      }
    }

    if (diff.props !== undefined) {
      if (diff.props) {
        // Clear existing props
        this.props.clear();
        const yProps = this.yDesign.get("props") as Y.Array<YProp>;
        if (yProps) {
          yProps.delete(0, yProps.length);
        } else {
          const newYProps = new Y.Array<YProp>();
          this.yDesign.set("props", newYProps);
        }
        for (const prop of diff.props) {
          this.createProp(prop);
        }
      } else {
        this.yDesign.delete("props");
        this.props.clear();
      }
    }

    if (diff.layers !== undefined) {
      if (diff.layers) {
        // Clear existing layers
        this.layers.clear();
        const yLayers = this.yDesign.get("layers") as Y.Array<YLayer>;
        if (yLayers) {
          yLayers.delete(0, yLayers.length);
        } else {
          const newYLayers = new Y.Array<YLayer>();
          this.yDesign.set("layers", newYLayers);
        }
        for (const layer of diff.layers) {
          this.createLayer(layer);
        }
      } else {
        this.yDesign.delete("layers");
        this.layers.clear();
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
        // Clear existing groups
        this.groups.clear();
        const yGroups = this.yDesign.get("groups") as Y.Array<YGroup>;
        if (yGroups) {
          yGroups.delete(0, yGroups.length);
        } else {
          const newYGroups = new Y.Array<YGroup>();
          this.yDesign.set("groups", newYGroups);
        }
        for (const group of diff.groups) {
          this.createGroup(group);
        }
      } else {
        this.yDesign.delete("groups");
        this.groups.clear();
      }
    }

    if (diff.location !== undefined) {
      if (diff.location) {
        if (!this.location) {
          const yLocation = new Y.Map();
          this.yDesign.set("location", yLocation);
          this.location = new YLocationStore(yLocation, diff.location);
        } else {
          this.location.change(diff.location);
        }
      } else {
        this.yDesign.delete("location");
        this.location = undefined;
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
const useDesignScope = () => useContext(DesignScopeContext);

function useDesignStore<T>(selector?: (store: DesignStore) => T, guid?: string): T | DesignStore {
  const kitStore = useKitStore() as KitStore;
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? guid;
  if (!designGuid) throw new Error("useDesignStore must be called within a DesignScopeProvider or be directly provided with a guid");
  if (!kitStore.hasDesign(designGuid)) throw new Error(`Design store not found for design ${designGuid}`);
  const designStore = kitStore.design(designGuid);
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: DesignId, deep: boolean = false): T | DesignShallow | Design {
  if (deep) {
    return useSyncDeep<Design, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector);
  }
  return useSync<DesignShallow, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector, deep);
}

export function usePieces(): Piece[] {
  const design = useDesign() as Design;
  return design.pieces ?? [];
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

export function useIncludedDesigns() {
  const design = useDesign();
  return useMemo(() => getIncludedDesigns(design), [design]);
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

// #endregion Design

// #region Kit

type YIdMap = Y.Map<string>;
type YKitVal = string | YUuidArray | YIdMap | YAttributes | YAuthors | YFiles | YBenchmarks | YQualities | YProps | YTypes | YDesigns;
type YKit = Y.Map<YKitVal>;
type YKits = Y.Array<YKit>;

export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
}

export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
}

class KitStore {
  public readonly parent: SketchpadStore;
  private readonly yProviderFactory: YProviderFactory | undefined;
  private readonly yDoc: Y.Doc;
  private readonly yKit: YKit;
  private readonly yTypes: YTypes;
  private readonly types: Map<string, TypeStore>;
  private readonly yDesigns: YDesigns;
  private readonly designs: Map<string, DesignStore>;
  private readonly yFiles: YFiles;
  private readonly files: Map<string, FileStore>;
  private readonly yQualities: YQualities;
  private readonly qualities: Map<string, QualityStore>;
  private readonly yBenchmarks: YBenchmarks;
  private readonly benchmarks: Map<string, BenchmarkStore>;
  private readonly yAuthors: YAuthors;
  private readonly authors: Map<string, AuthorStore>;
  private readonly yAttributes: YAttributes;
  private readonly attributes: Map<string, AttributeStore>;
  private readonly persistence: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult>;
  private readonly regularFiles: Map<Url, string>;
  private cache?: Kit;
  private cacheHash?: string;

  constructor(parent: SketchpadStore, kit: Kit, yProviderFactory?: YProviderFactory) {
    this.parent = parent;
    this.yProviderFactory = yProviderFactory;
    this.yDoc = new Y.Doc();

    if (yProviderFactory) {
      yProviderFactory(this.yDoc, this.name + "@" + this.version);
    }

    this.commandRegistry = new Map();
    this.regularFiles = new Map();
    this.types = new Map();
    this.designs = new Map();
    this.files = new Map();
    this.qualities = new Map();
    this.benchmarks = new Map();
    this.authors = new Map();
    this.attributes = new Map();

    this.yKit = this.yDoc.getMap() as YKit;
    this.yTypes = this.yDoc.getArray("types");
    this.yDesigns = this.yDoc.getArray("designs");
    this.yFiles = this.yDoc.getArray("files");
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
      kit.qualities?.forEach((quality) => this.createQuality(quality));
      kit.types?.forEach((type) => this.createType(type));
      kit.designs?.forEach((design) => this.createDesign(design));
      kit.files?.forEach((file) => this.createFile(file));

      this.yKit.set("createdAt", new Date().toISOString());
      this.updated();
    });

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
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

  updated(): void {
    this.yKit.set("updatedAt", new Date().toISOString());
  }

  hasType(guid: string): boolean {
    return this.types.has(guid);
  }

  createType(type: Type): void {
    if (this.hasType(type.guid)) throw new Error(`Type (${type.name}, ${type.variant || ""}) already exists.`);
    const yType = new Y.Map<YTypeVal>();
    const yTypeStore = new TypeStore(this, yType, type);
    this.yTypes.push([yType]);
    this.types.set(type.guid, yTypeStore);
  }

  type(guid: string): TypeStore {
    return this.types.get(guid)!;
  }

  hasDesign(guid: string): boolean {
    return this.designs.has(guid);
  }

  createDesign(design: Design): void {
    if (this.hasDesign(design.guid)) throw new Error(`Design (${design.name}, ${design.variant || ""}, ${design.view || ""}) already exists.`);
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
    if (this.hasFile(file.guid)) throw new Error(`File (${file.path}) already exists.`);
    const yFile = new Y.Map<YFile>();
    this.yFiles.push([yFile]);
    const yFileStore = new FileStore(yFile, file);
    this.files.set(file.guid, yFileStore);
  }

  file(guid: string): FileStore {
    return this.files.get(guid)!;
  }

  hasQuality(guid: string): boolean {
    return this.qualities.has(guid);
  }

  createQuality(quality: Quality): void {
    if (this.hasQuality(quality.guid)) throw new Error(`Quality (${quality.key}) already exists.`);
    const yQuality = new Y.Map<YQuality>();
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
    const yBenchmark = new Y.Map<YBenchmark>();
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

  hasAttribute(attribute: AttributeIdLike): boolean {
    return this.attributes.some((a) => a.id.key === (typeof attribute === "string" ? attribute : attribute.key));
  }

  createAttribute(attribute: Attribute): void {
    if (this.hasAttribute(attribute.guid)) throw new Error(`Attribute (${attribute.key}) already exists.`);
    const yAttribute = new Y.Map<YAttribute>();
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
      qualities: Array.from(this.qualities.values()).map((quality) => quality.snapshot),
      files: Array.from(this.files.values()).map((file) => file.snapshot()),
      authors: Array.from(this.authors.values()).map((author) => author.snapshot()),
      attributes: Array.from(this.attributes.values()).map((attribute) => attribute.snapshot),
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
          diff.authors.removed.forEach((authorId) => {
            const authorGuid = typeof authorId === "string" ? authorId : authorId.email;
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
          diff.types.removed.forEach((typeId) => {
            if (this.types.has(typeId)) {
              this.types.delete(typeId);
              // Find and delete from Y.Array
              const index = Array.from(this.yTypes).findIndex((yType: any) => {
                const yMap = yType[0] as Y.Map<any>;
                return yMap.get("guid") === typeId;
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
          diff.designs.removed.forEach((designId) => {
            if (this.designs.has(designId)) {
              this.designs.delete(designId);
              // Find and delete from Y.Array
              const index = Array.from(this.yDesigns).findIndex((yDesign: any) => {
                const yMap = yDesign[0] as Y.Map<any>;
                return yMap.get("guid") === designId;
              });
              if (index !== -1) {
                this.yDesigns.delete(index, 1);
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
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, authorId: AuthorId, authorDiff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ id: authorId, diff: authorDiff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, authorId: AuthorId): KitCommandResult => {
    return {
      diff: { authors: { removed: [authorId] } },
    };
  },
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

type KitScope = { guid: string };
const KitScopeContext = createContext<KitScope | null>(null);
export const KitScopeProvider = (props: { guid: string; children: React.ReactNode }) => {
  const value = { guid: props.guid };
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};
const useKitStoreScope = () => useContext(KitScopeContext);

function useKitStore<T>(selector?: (store: KitStore) => T, guid?: string): T | KitStore {
  const store = useSketchpadStore();
  const kitScope = useKitStoreScope();
  const kitGuid = kitScope?.guid ?? guid;
  if (!kitGuid) throw new Error("useKitStore must be called within a KitScopeProvider or be directly provided with a guid");
  if (!store.hasKit(kitGuid)) throw new Error(`Kit store not found for kit ${kitGuid}`);
  const kitStore = store.kit(kitGuid);
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

export function useDesigns(): Design[] {
  return useKit((k) => k.designs ?? []) as Design[];
}

export function useFileUrls(): Map<Url, Url> {
  return (useKitStore() as KitStore).fileUrls();
}

export function useKitCommands() {
  const store = useKitStore() as KitStore;
  return {
    importKit: (url: string) => store.execute("semio.kit.import", url),
    exportKit: () => store.execute("semio.kit.export"),
    createAuthor: (author: Author) => store.execute("semio.kit.createAuthor", author),
    updateAuthor: (authorId: AuthorId, authorDiff: AuthorDiff) => store.execute("semio.kit.updateAuthor", authorId, authorDiff),
    deleteAuthor: (authorId: AuthorId) => store.execute("semio.kit.deleteAuthor", authorId),
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

// #endregion Kit

// #region Editor

interface EditorStep<TSelectionDiff = any> {
  kitDiff?: KitDiff;
  selectionDiff?: TSelectionDiff;
}

interface EditorEdit<TSelectionDiff = any> {
  do: EditorStep<TSelectionDiff>;
  undo: EditorStep<TSelectionDiff>;
}

interface EditorDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenPanel?: any;
}

interface EditorCommandResult<TDiff = any> {
  diff?: TDiff;
  kitDiff?: KitDiff;
}

abstract class Editor<TState, TDiff extends EditorDiff<TSelectionDiff>, TSelectionDiff, TEdit extends EditorEdit<TSelectionDiff>, TCommandContext, TCommandResult extends EditorCommandResult<TDiff>> {
  public readonly guid: string;
  public readonly parent: SketchpadStore;
  public readonly yMap: Y.Map<any>;
  protected readonly commandRegistry: Map<string, (context: TCommandContext, ...rest: any[]) => TCommandResult> = new Map();
  protected readonly transact: (fn: () => void) => void;
  protected cache?: TState;
  protected cacheHash?: string;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: (fn: () => void) => void) {
    this.guid = guid();
    this.parent = parent;
    this.yMap = yMap;
    this.transact = transact;
  }

  protected abstract applySelectionDiff(selectionDiff: TSelectionDiff): void;
  protected abstract inverseSelectionDiff(selection: any, diff: TSelectionDiff): TSelectionDiff;
  protected abstract getSelection(): any;
  protected abstract hash(state: TState): string;
  protected abstract buildSnapshot(): TState;
  abstract kit(): KitStore;

  get isTransactionActive(): boolean {
    return (this.yMap.get("isTransactionActive") as boolean) || false;
  }

  set isTransactionActive(active: boolean) {
    this.yMap.set("isTransactionActive", active);
  }

  get diff(): KitDiff {
    return {};
  }

  get currentTransactionStack(): TEdit[] {
    const yStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  get pastTransactionsStack(): TEdit[] {
    const yStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
    return yStack ? yStack.toArray() : [];
  }

  canUndo(): boolean {
    return this.pastTransactionsStack.length > 0;
  }

  canRedo(): boolean {
    return this.currentTransactionStack.length > 0 && !this.isTransactionActive;
  }

  snapshot = (): TState => {
    const currentData = this.buildSnapshot();
    const currentHash = this.hash(currentData);

    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentData;
      this.cacheHash = currentHash;
    }

    return this.cache;
  };

  change = (diff: TDiff) => {
    this.transact(() => {
      if (diff.fullscreenPanel !== undefined) {
        this.yMap.set("fullscreenPanel", diff.fullscreenPanel);
      }
      if (diff.selection) {
        this.applySelectionDiff(diff.selection);
      }
      if (diff.presence) {
        // Handle presence changes if needed
      }
      if (diff.hover) {
        // Handle hover changes if needed
      }
    });
  };

  onChanged = (subscribe: Subscribe) => {
    return createObserver(this.yMap, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe) => {
    return createObserver(this.yMap, subscribe, true);
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
          edit.undo.kitDiff && this.kit().change(edit.undo.kitDiff);
          edit.undo.selectionDiff && this.applySelectionDiff(edit.undo.selectionDiff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(pastStack.length - 1);
        pastStack.delete(pastStack.length - 1, 1);
        if (edit && edit.undo) {
          edit.undo.diff && this.change(edit.undo.diff);
          edit.undo.kitDiff && this.kit().change(edit.undo.kitDiff);
          edit.undo.selectionDiff && this.applySelectionDiff(edit.undo.selectionDiff);
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
          edit.do.kitDiff && this.kit().change(edit.do.kitDiff);
          edit.do.selectionDiff && this.applySelectionDiff(edit.do.selectionDiff);
        }
      }
    } else {
      const pastStack = this.yMap.get("pastTransactionsStack") as Y.Array<any>;
      if (pastStack && pastStack.length > 0) {
        const edit = pastStack.get(0);
        pastStack.delete(0, 1);
        if (edit && edit.do) {
          edit.do.diff && this.change(edit.do.diff);
          edit.do.kitDiff && this.kit().change(edit.do.kitDiff);
          edit.do.selectionDiff && this.applySelectionDiff(edit.do.selectionDiff);
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

  protected recordEdit(state: any, result: TCommandResult) {
    if (this.isTransactionActive && (result.diff || result.kitDiff)) {
      const currentStack = this.yMap.get("currentTransactionStack") as Y.Array<any>;
      const selection = this.getSelection();
      const inversedSelectionDiff = result.diff?.selection ? this.inverseSelectionDiff(selection, result.diff.selection) : undefined;
      const kitStore = this.kit();
      const kitState = kitStore.snapshot();
      const inversedKitDiff = result.kitDiff ? inverseKitDiff(kitState, result.kitDiff) : undefined;

      const edit: TEdit = {
        do: {
          kitDiff: result.kitDiff,
          selectionDiff: result.diff?.selection,
        },
        undo: {
          kitDiff: inversedKitDiff,
          selectionDiff: inversedSelectionDiff,
        },
      } as TEdit;
      currentStack.push([edit]);
    }
  }

  registerCommand(command: string, callback: (context: TCommandContext, ...rest: any[]) => TCommandResult): Disposable {
    this.commandRegistry.set(command, callback);
    return () => {
      this.commandRegistry.delete(command);
    };
  }

  register(command: string, callback: (context: TCommandContext, ...rest: any[]) => TCommandResult): Disposable {
    return this.registerCommand(command, callback);
  }

  get commands() {
    return {
      execute: this.executeCommand.bind(this),
      register: this.registerCommand.bind(this),
    };
  }

  abstract executeCommand<T>(command: string, ...rest: any[]): Promise<T>;
  abstract execute<T>(command: string, ...rest: any[]): Promise<T>;
}

// #endregion Editor

// #region Kit Editor

type YKitEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YKitEditor = Y.Map<YKitEditorVal>;
type YKitEditors = Y.Array<YKitEditor>;

export interface KitEditorId {
  kit: KitId;
}
export interface KitEditorSelection {
  types?: TypeId[];
  designs?: DesignId[];
}
export interface KitEditorSelectionTypesDiff {
  added?: TypeId[];
  removed?: TypeId[];
}
export interface KitEditorSelectionDesignsDiff {
  added?: DesignId[];
  removed?: DesignId[];
}
export interface KitEditorSelectionDiff {
  types?: KitEditorSelectionTypesDiff;
  designs?: KitEditorSelectionDesignsDiff;
}
export enum KitEditorFullscreenPanel {
  None = "none",
  Types = "types",
  Designs = "designs",
}
export interface KitEditorPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface KitEditorHover {
  type?: TypeId;
  design?: DesignId;
}
export interface KitEditorPresenceOther extends KitEditorPresence {
  name: string;
}
export interface KitEditorDiff {
  selection?: KitEditorSelectionDiff;
  presence?: KitEditorPresence;
  hover?: KitEditorHover;
  fullscreenPanel?: KitEditorFullscreenPanel;
}
export interface KitEditorStep {
  kitDiff?: KitDiff;
  selectionDiff?: KitEditorSelectionDiff;
}
export interface KitEditorEdit {
  do: KitEditorStep;
  undo: KitEditorStep;
}
export interface KitEditorState {
  fullscreenPanel: KitEditorFullscreenPanel;
  selection?: KitEditorSelection;
  hover?: KitEditorHover;
  presence?: KitEditorPresence;
  others: KitEditorPresenceOther[];
}

export interface KitEditorCommandContext extends KitCommandContext {
  kitEditor: KitEditorState;
}
export interface KitEditorCommandResult {
  diff?: KitEditorDiff;
  kitDiff?: KitDiff;
}

export const inverseKitEditorSelectionDiff = (selection: KitEditorSelection, diff: KitEditorSelectionDiff): KitEditorSelectionDiff => {
  const inverseDiff: KitEditorSelectionDiff = {};

  // Inverse types diff
  if (diff.types) {
    inverseDiff.types = {};
    if (diff.types.added) {
      inverseDiff.types.removed = diff.types.added;
    }
    if (diff.types.removed) {
      inverseDiff.types.added = diff.types.removed;
    }
  }

  // Inverse designs diff
  if (diff.designs) {
    inverseDiff.designs = {};
    if (diff.designs.added) {
      inverseDiff.designs.removed = diff.designs.added;
    }
    if (diff.designs.removed) {
      inverseDiff.designs.added = diff.designs.removed;
    }
  }

  return inverseDiff;
};
export const areSameKitEditor = (kitEditor: KitEditorId, other: KitEditorId): boolean => areSameKit(kitEditor.kit, other.kit);
export const hasSameKitEditor = (kitEditor: KitEditorId, others: KitEditorId[]): boolean => others.some((other) => areSameKitEditor(kitEditor, other));

class KitEditorStore extends Editor<KitEditorState, KitEditorDiff, KitEditorSelectionDiff, KitEditorEdit, KitEditorCommandContext, KitEditorCommandResult> {
  constructor(parent: SketchpadStore, yMap: YKitEditor, transact: (fn: () => void) => void, id: KitEditorId, state?: KitEditorState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    yMap.set("kit", kit.guid);

    yMap.set("fullscreenPanel", state?.fullscreenPanel || KitEditorFullscreenPanel.None);

    const selection = new Y.Map<any>();
    const selectedTypes = new Y.Array<string>();
    if (state?.selection?.types) {
      selectedTypes.push(state?.selection.types.map((type) => typeIdToString(type)) || []);
    }
    const selectedDesigns = new Y.Array<string>();
    if (state?.selection?.designs) {
      selectedDesigns.push(state?.selection.designs.map((design) => designIdToString(design)) || []);
    }
    selection.set("types", selectedTypes);
    selection.set("designs", selectedDesigns);
    yMap.set("selection", selection);

    yMap.set("isTransactionActive", false);
    yMap.set("presence", new Y.Map<any>());
    yMap.set("others", new Y.Array<any>());
    yMap.set("diff", new Y.Map<any>());
    yMap.set("currentTransactionStack", new Y.Array<any>());
    yMap.set("pastTransactionsStack", new Y.Array<any>());

    Object.entries(kitEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  // KitEditor-specific getters
  get fullscreenPanel(): KitEditorFullscreenPanel {
    return this.yMap.get("fullscreenPanel") as KitEditorFullscreenPanel;
  }

  get selection(): KitEditorSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: KitEditorSelection = {};

    // Get types
    const types = selection.get("types") as Y.Array<string>;
    if (types && types.length > 0) {
      result.types = types.toArray().map((id_) => ({ id_ }));
    }

    // Get designs
    const designs = selection.get("designs") as Y.Array<string>;
    if (designs && designs.length > 0) {
      result.designs = designs.toArray().map((id_) => ({ id_ }));
    }

    return result;
  }

  get presence(): KitEditorPresence {
    return {
      cursor: {
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }

  get others(): KitEditorPresenceOther[] {
    return [];
  }

  kit(): KitStore {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  id(): KitEditorId {
    return {
      kit: this.kit().id(),
    } as KitEditorId;
  }

  // Implement abstract methods from Editor base class
  protected getSelection(): KitEditorSelection {
    return this.selection;
  }

  protected hash(state: KitEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): KitEditorState {
    return {
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
    } as any;
  }

  protected inverseSelectionDiff(selection: KitEditorSelection, diff: KitEditorSelectionDiff): KitEditorSelectionDiff {
    return inverseKitEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff(selectionDiff: KitEditorSelectionDiff): void {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    // Apply types diff
    if (selectionDiff.types) {
      let types = (selection.get("types") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("types")) {
        selection.set("types", types);
      }

      if (selectionDiff.types.added) {
        for (const type of selectionDiff.types.added) {
          if (!types.toArray().includes(type.id_)) {
            types.push([type.id_]);
          }
        }
      }
      if (selectionDiff.types.removed) {
        for (const type of selectionDiff.types.removed) {
          const index = types.toArray().indexOf(type.id_);
          if (index !== -1) {
            types.delete(index, 1);
          }
        }
      }
    }

    // Apply designs diff
    if (selectionDiff.designs) {
      let designs = (selection.get("designs") as Y.Array<string>) || new Y.Array<string>();
      if (!selection.has("designs")) {
        selection.set("designs", designs);
      }

      if (selectionDiff.designs.added) {
        for (const design of selectionDiff.designs.added) {
          if (!designs.toArray().includes(design.id_)) {
            designs.push([design.id_]);
          }
        }
      }
      if (selectionDiff.designs.removed) {
        for (const design of selectionDiff.designs.removed) {
          const index = designs.toArray().indexOf(design.id_);
          if (index !== -1) {
            designs.delete(index, 1);
          }
        }
      }
    }
  }

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.kitEditor.startTransaction") {
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.kitEditor.finalizeTransaction") {
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.kitEditor.abortTransaction") {
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.kitEditor.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.kitEditor.redo") {
      this.redo();
      return {} as T;
    }

    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit editor store`);

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const context: KitEditorCommandContext = {
      kitEditor: state,
      kit: kitState,
      fileUrls: kitStore.fileUrls,
    };
    const result = callback(context, ...rest);

    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }

    // Use base class recordEdit method
    this.recordEdit(state, result);

    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

const kitEditorCommands = {
  "semio.kitEditor.setMode": (context: KitEditorCommandContext, mode: Mode): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.setTheme": (context: KitEditorCommandContext, theme: Theme): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.setLayout": (context: KitEditorCommandContext, layout: Layout): KitEditorCommandResult => {
    return { diff: {} };
  },
  "semio.kitEditor.toggleTypesFullscreen": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentPanel = context.kitEditor.fullscreenPanel;
    const newPanel = currentPanel === KitEditorFullscreenPanel.Types ? KitEditorFullscreenPanel.None : KitEditorFullscreenPanel.Types;
    return {
      diff: {
        fullscreenPanel: newPanel,
      },
    };
  },
  "semio.kitEditor.toggleDesignsFullscreen": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentPanel = context.kitEditor.fullscreenPanel;
    const newPanel = currentPanel === KitEditorFullscreenPanel.Designs ? KitEditorFullscreenPanel.None : KitEditorFullscreenPanel.Designs;
    return {
      diff: {
        fullscreenPanel: newPanel,
      },
    };
  },
  "semio.kitEditor.selectAll": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const kit = context.kit;
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: kit.types ?? [],
          },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: kit.designs ?? [],
          },
        },
      },
    };
  },
  "semio.kitEditor.deselectAll": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitEditor.selectType": (context: KitEditorCommandContext, typeId: TypeId): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: [typeId],
          },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitEditor.selectTypes": (context: KitEditorCommandContext, typeIds: TypeId[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: {
            removed: currentSelection?.types ?? [],
            added: typeIds,
          },
          designs: { removed: currentSelection?.designs ?? [] },
        },
      },
    };
  },
  "semio.kitEditor.addTypeToSelection": (context: KitEditorCommandContext, typeId: TypeId): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          types: { added: [typeId] },
        },
      },
    };
  },
  "semio.kitEditor.removeTypeFromSelection": (context: KitEditorCommandContext, typeId: TypeId): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          types: { removed: [typeId] },
        },
      },
    };
  },
  "semio.kitEditor.selectDesign": (context: KitEditorCommandContext, designId: DesignId): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: [designId],
          },
        },
      },
    };
  },
  "semio.kitEditor.selectDesigns": (context: KitEditorCommandContext, designIds: DesignId[]): KitEditorCommandResult => {
    const currentSelection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: { removed: currentSelection?.types ?? [] },
          designs: {
            removed: currentSelection?.designs ?? [],
            added: designIds,
          },
        },
      },
    };
  },
  "semio.kitEditor.addDesignToSelection": (context: KitEditorCommandContext, designId: DesignId): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          designs: { added: [designId] },
        },
      },
    };
  },
  "semio.kitEditor.removeDesignFromSelection": (context: KitEditorCommandContext, designId: DesignId): KitEditorCommandResult => {
    return {
      diff: {
        selection: {
          designs: { removed: [designId] },
        },
      },
    };
  },
  "semio.kitEditor.deleteSelected": (context: KitEditorCommandContext): KitEditorCommandResult => {
    const selection = context.kitEditor.selection;
    return {
      diff: {
        selection: {
          types: { removed: selection?.types ?? [] },
          designs: { removed: selection?.designs ?? [] },
        },
      },
      kitDiff: {
        types: { removed: selection?.types },
        designs: { removed: selection?.designs },
      },
    };
  },
  "semio.kitEditor.addType": (context: KitEditorCommandContext, type: Type): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: [type] },
      },
    };
  },
  "semio.kitEditor.addTypes": (context: KitEditorCommandContext, types: Type[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { added: types },
      },
    };
  },
  "semio.kitEditor.removeType": (context: KitEditorCommandContext, typeId: TypeId): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: [typeId] },
      },
    };
  },
  "semio.kitEditor.removeTypes": (context: KitEditorCommandContext, typeIds: TypeId[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { removed: typeIds },
      },
    };
  },
  "semio.kitEditor.addDesign": (context: KitEditorCommandContext, design: Design): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: [design] },
      },
    };
  },
  "semio.kitEditor.addDesigns": (context: KitEditorCommandContext, designs: Design[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { added: designs },
      },
    };
  },
  "semio.kitEditor.removeDesign": (context: KitEditorCommandContext, designId: DesignId): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: [designId] },
      },
    };
  },
  "semio.kitEditor.removeDesigns": (context: KitEditorCommandContext, designIds: DesignId[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { removed: designIds },
      },
    };
  },
  "semio.kitEditor.updateType": (context: KitEditorCommandContext, typeId: TypeId, typeDiff: TypeDiff): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: [{ id: typeId, diff: typeDiff }] },
      },
    };
  },
  "semio.kitEditor.updateTypes": (context: KitEditorCommandContext, updates: { id: TypeId; diff: TypeDiff }[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        types: { updated: updates },
      },
    };
  },
  "semio.kitEditor.updateDesign": (context: KitEditorCommandContext, designId: DesignId, designDiff: DesignDiff): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: [{ id: designId, diff: designDiff }] },
      },
    };
  },
  "semio.kitEditor.updateDesigns": (context: KitEditorCommandContext, updates: { id: DesignId; diff: DesignDiff }[]): KitEditorCommandResult => {
    return {
      diff: {},
      kitDiff: {
        designs: { updated: updates },
      },
    };
  },
};

type KitEditorScope = { id: string };
const KitEditorScopeContext = createContext<KitEditorScope | null>(null);
export const KitEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(KitEditorScopeContext.Provider, { value }, props.children as any);
};
const useKitEditorScope = () => useContext(KitEditorScopeContext);

function useKitEditorStore<T>(selector?: (store: KitEditorStore) => T, id?: KitEditorId): T | KitEditorStore {
  const store = useSketchpadStore();
  const kitScope = useKitStoreScope();
  const resolvedKitId = kitScope?.id ?? id?.kit;
  if (!resolvedKitId) throw new Error("useKitEditorStore must be called within a KitScopeProvider or be directly provided with an id");
  const kitEditorStore = store.kitEditor({ kit: resolvedKitId });
  return selector ? selector(kitEditorStore) : kitEditorStore;
}

export function useKitEditor<T>(selector?: (state: KitEditorState) => T, id?: KitEditorId): T | KitEditorState {
  return useSyncDeep<KitEditorState, T>(useKitEditorStore(identitySelector, id) as KitEditorStore, selector ? selector : identitySelector);
}

export function useKitEditorSelection(): KitEditorSelection {
  return useKitEditor((s) => s.selection) as KitEditorSelection;
}

export function useKitEditorFullscreen(): KitEditorFullscreenPanel {
  return useKitEditor((s) => s.fullscreenPanel) as KitEditorFullscreenPanel;
}

export function useKitEditorOthers(): KitEditorPresenceOther[] {
  return useKitEditor((s) => s.others) as KitEditorPresenceOther[];
}

export function useKitEditorCommands() {
  const store = useKitEditorStore() as KitEditorStore;
  return {
    startTransaction: () => store.execute("semio.kitEditor.startTransaction"),
    finalizeTransaction: () => store.execute("semio.kitEditor.finalizeTransaction"),
    abortTransaction: () => store.execute("semio.kitEditor.abortTransaction"),
    undo: () => store.execute("semio.kitEditor.undo"),
    redo: () => store.execute("semio.kitEditor.redo"),
    selectAll: () => store.execute("semio.kitEditor.selectAll"),
    deselectAll: () => store.execute("semio.kitEditor.deselectAll"),
    selectType: (typeId: TypeId) => store.execute("semio.kitEditor.selectType", typeId),
    selectTypes: (typeIds: TypeId[]) => store.execute("semio.kitEditor.selectTypes", typeIds),
    addTypeToSelection: (typeId: TypeId) => store.execute("semio.kitEditor.addTypeToSelection", typeId),
    removeTypeFromSelection: (typeId: TypeId) => store.execute("semio.kitEditor.removeTypeFromSelection", typeId),
    selectDesign: (designId: DesignId) => store.execute("semio.kitEditor.selectDesign", designId),
    selectDesigns: (designIds: DesignId[]) => store.execute("semio.kitEditor.selectDesigns", designIds),
    addDesignToSelection: (designId: DesignId) => store.execute("semio.kitEditor.addDesignToSelection", designId),
    removeDesignFromSelection: (designId: DesignId) => store.execute("semio.kitEditor.removeDesignFromSelection", designId),
    deleteSelected: () => store.execute("semio.kitEditor.deleteSelected"),
    toggleTypesFullscreen: () => store.execute("semio.kitEditor.toggleTypesFullscreen"),
    toggleDesignsFullscreen: () => store.execute("semio.kitEditor.toggleDesignsFullscreen"),
    addType: (type: Type) => store.execute("semio.kitEditor.addType", type),
    addTypes: (types: Type[]) => store.execute("semio.kitEditor.addTypes", types),
    removeType: (typeId: TypeId) => store.execute("semio.kitEditor.removeType", typeId),
    removeTypes: (typeIds: TypeId[]) => store.execute("semio.kitEditor.removeTypes", typeIds),
    addDesign: (design: Design) => store.execute("semio.kitEditor.addDesign", design),
    addDesigns: (designs: Design[]) => store.execute("semio.kitEditor.addDesigns", designs),
    removeDesign: (designId: DesignId) => store.execute("semio.kitEditor.removeDesign", designId),
    removeDesigns: (designIds: DesignId[]) => store.execute("semio.kitEditor.removeDesigns", designIds),
    updateType: (typeId: TypeId, typeDiff: TypeDiff) => store.execute("semio.kitEditor.updateType", typeId, typeDiff),
    updateTypes: (updates: { id: TypeId; diff: TypeDiff }[]) => store.execute("semio.kitEditor.updateTypes", updates),
    updateDesign: (designId: DesignId, designDiff: DesignDiff) => store.execute("semio.kitEditor.updateDesign", designId, designDiff),
    updateDesigns: (updates: { id: DesignId; diff: DesignDiff }[]) => store.execute("semio.kitEditor.updateDesigns", updates),
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}

// #endregion Kit Editor

// #region Design Editor

type YDesignEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YDesignEditor = Y.Map<YDesignEditorVal>;
type YDesignEditors = Y.Array<YDesignEditor>;

export interface DesignEditorSelection {
  pieces?: Guid[];
  connections?: Guid[];
  port?: { piece: Guid; designPiece?: Guid; port: Guid };
}
export interface DesignEditorSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignEditorSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignEditorSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  port?: Guid;
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
  piece?: Guid;
  connection?: Guid;
  port?: Guid;
}
export interface DesignEditorPresenceOther extends DesignEditorPresence {
  name: string;
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
export interface DesignEditorEdit {
  do: DesignEditorStep;
  undo: DesignEditorStep;
}
export interface DesignEditorState {
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection?: DesignEditorSelection;
  hover?: DesignEditorHover;
  presence?: DesignEditorPresence;
  others: DesignEditorPresenceOther[];
}

export interface DesignEditorCommandContext extends KitCommandContext {
  designEditor: DesignEditorState;
  designId: DesignId;
}
export interface DesignEditorCommandResult {
  diff?: DesignEditorDiff;
  kitDiff?: KitDiff;
}

export const inverseDesignEditorSelectionDiff = (selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff => {
  const inverseDiff: DesignEditorSelectionDiff = {};

  // Inverse pieces diff
  if (diff.pieces) {
    inverseDiff.pieces = {};
    if (diff.pieces.added) {
      inverseDiff.pieces.removed = diff.pieces.added;
    }
    if (diff.pieces.removed) {
      inverseDiff.pieces.added = diff.pieces.removed;
    }
  }

  // Inverse connections diff
  if (diff.connections) {
    inverseDiff.connections = {};
    if (diff.connections.added) {
      inverseDiff.connections.removed = diff.connections.added;
    }
    if (diff.connections.removed) {
      inverseDiff.connections.added = diff.connections.removed;
    }
  }

  // Inverse port diff - restore the original port from selection
  if (diff.port) {
    inverseDiff.port = {
      piece: selection.port?.piece,
      designPiece: selection.port?.designPiece,
      port: selection.port?.port,
    };
  }

  return inverseDiff;
};
export const areSameDesignEditor = (designEditor: DesignEditorId, other: DesignEditorId): boolean => areSameKit(designEditor.kit, other.kit) && designEditor.design === other.design;
export const hasSameDesignEditor = (designEditor: DesignEditorId, others: DesignEditorId[]): boolean => others.some((other) => areSameDesignEditor(designEditor, other));

class DesignEditorStore extends Editor<DesignEditorState, DesignEditorDiff, DesignEditorSelectionDiff, DesignEditorEdit, DesignEditorCommandContext, DesignEditorCommandResult> {
  constructor(parent: SketchpadStore, yMap: YDesignEditor, transact: (fn: () => void) => void, id: DesignEditorId, state?: DesignEditorState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    const design = kit.design(id.design);
    yMap.set("kit", kit.guid);
    yMap.set("design", design.guid);

    yMap.set("fullscreenPanel", state?.fullscreenPanel || DesignEditorFullscreenPanel.None);

    const selection = new Y.Map<any>();
    const selectedPieces = new Y.Array<Guid>();
    if (state?.selection.pieces) {
      selectedPieces.push(state?.selection.pieces.map((piece) => pieceIdToString(piece)) || []);
    }
    const selectedConnections = new Y.Array<Guid>();
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
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: DesignEditorSelection = {};

    // Get pieces
    const pieces = selection.get("pieces") as Y.Array<string>;
    if (pieces && pieces.length > 0) {
      result.pieces = pieces.toArray().map((id_) => ({ id_ }));
    }

    // Get connections
    const connections = selection.get("connections") as Y.Array<Y.Map<any>>;
    if (connections && connections.length > 0) {
      result.connections = connections.toArray().map((conn) => ({
        connected: { piece: { id_: conn.get("connected") } },
        connecting: { piece: { id_: conn.get("connecting") } },
      }));
    }

    // Get port
    const port = selection.get("port") as Y.Map<string>;
    if (port) {
      const piece = port.get("piece");
      const designPiece = port.get("designPiece");
      const portId = port.get("port");

      if (piece && portId) {
        result.port = {
          piece: { id_: piece },
          designPiece: designPiece ? { id_: designPiece } : undefined,
          port: { id_: portId },
        };
      }
    }

    return result;
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

  kit(): KitStore {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  design(): DesignStore {
    return this.kit().design(this.yMap.get("design") as string);
  }

  protected getSelection(): DesignEditorSelection {
    return this.selection;
  }

  protected hash(state: DesignEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DesignEditorState {
    return {
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
  }

  protected inverseSelectionDiff(selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff {
    return inverseDesignEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: DesignEditorSelectionDiff) => {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    // Apply pieces diff
    if (selectionDiff.pieces) {
      let pieces = (selection.get("pieces") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("pieces")) {
        selection.set("pieces", pieces);
      }

      if (selectionDiff.pieces.added) {
        for (const piece of selectionDiff.pieces.added) {
          if (!pieces.toArray().includes(piece)) {
            pieces.push([piece]);
          }
        }
      }
      if (selectionDiff.pieces.removed) {
        for (const piece of selectionDiff.pieces.removed) {
          const index = pieces.toArray().indexOf(piece);
          if (index !== -1) {
            pieces.delete(index, 1);
          }
        }
      }
    }

    // Apply connections diff
    if (selectionDiff.connections) {
      let connections = (selection.get("connections") as Y.Array<Y.Map<any>>) || new Y.Array<Y.Map<any>>();
      if (!selection.has("connections")) {
        selection.set("connections", connections);
      }

      if (selectionDiff.connections.added) {
        for (const connection of selectionDiff.connections.added) {
          const connectionMap = new Y.Map();
          connectionMap.set("connected", connection.connected.piece);
          connectionMap.set("connecting", connection.connecting.piece);
          connections.push([connectionMap]);
        }
      }
      if (selectionDiff.connections.removed) {
        for (const connection of selectionDiff.connections.removed) {
          const connectionsArray = connections.toArray();
          const index = connectionsArray.findIndex((conn) => conn.get("connected") === connection.connected.piece && conn.get("connecting") === connection.connecting.piece);
          if (index !== -1) {
            connections.delete(index, 1);
          }
        }
      }
    }

    // Apply port diff
    if (selectionDiff.port) {
      const portSelection = new Y.Map();
      if (selectionDiff.port.piece !== undefined) {
        portSelection.set("piece", selectionDiff.port.piece);
      }
      if (selectionDiff.port.designPiece !== undefined) {
        portSelection.set("designPiece", selectionDiff.port.designPiece);
      }
      if (selectionDiff.port.port !== undefined) {
        portSelection.set("port", selectionDiff.port.port);
      }
      selection.set("port", portSelection);
    }
  };

  change = (diff: DesignEditorDiff) => {
    this.transact(() => {
      if (diff.fullscreenPanel) this.fullscreenPanel = diff.fullscreenPanel;
      if (diff.selection) {
        this.applySelectionDiff(diff.selection);
      }
      if (diff.presence) {
        // Handle presence changes if needed
      }
      if (diff.hover) {
        // Handle hover changes if needed
      }
    });
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

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const context: DesignEditorCommandContext = {
      designEditor: state,
      kit: kitState,
      designId: this.design().guid,
      fileUrls: kitStore.fileUrls,
    };
    const result = callback(context, ...rest);

    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }

    this.recordEdit(state, result);

    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
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
  "semio.designEditor.selectPiece": (context: DesignEditorCommandContext, pieceId: Guid): DesignEditorCommandResult => {
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
  "semio.designEditor.selectPieces": (context: DesignEditorCommandContext, pieceIds: Guid[]): DesignEditorCommandResult => {
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
  "semio.designEditor.addPieceToSelection": (context: DesignEditorCommandContext, pieceId: Guid): DesignEditorCommandResult => {
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

type DesignEditorScope = { id: string };
const DesignEditorScopeContext = createContext<DesignEditorScope | null>(null);
export const DesignEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignEditorScopeContext.Provider, { value }, props.children as any);
};
const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

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
  return useSyncDeep<DesignEditorState, T>(useDesignEditorStore(identitySelector, id) as DesignEditorStore, selector ? selector : identitySelector);
}

export function useDesignEditorSelection(): DesignEditorSelection {
  return useDesignEditor((s) => s.selection) as DesignEditorSelection;
}

export function useDesignEditorFullscreen(): DesignEditorFullscreenPanel {
  return useDesignEditor((s) => s.fullscreenPanel) as DesignEditorFullscreenPanel;
}

export function useDesignEditorDiff(): KitDiff {
  return useDesignEditor((s) => s.diff) as KitDiff;
}

export function useDesignEditorOthers(): DesignEditorPresenceOther[] {
  return useDesignEditor((s) => s.others) as DesignEditorPresenceOther[];
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

// #endregion Design Editor

// #region Sketchpad

type YSketchpadVal = string | boolean | YDesignEditors;
type YSketchpad = Y.Map<YSketchpadVal>;

export interface PanelVisibility {
  workbench?: boolean;
  details?: boolean;
  console?: boolean;
  chat?: boolean;
  settings?: boolean;
}

export interface EditorSettings {
  design?: {
    snappiness?: number;
    gridSize?: number;
  };
  type?: Record<string, any>;
  kit?: Record<string, any>;
}

export interface PanelSizes {
  workbenchWidth: number;
  detailsWidth: number;
  chatWidth: number;
  settingsWidth: number;
  consoleHeight: number;
}

export interface SketchpadChangableState {
  navigation: string;
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditor?: DesignEditorId;
  panelVisibility: {
    [EditorType.HOME]: PanelVisibility;
    [EditorType.KIT]: PanelVisibility;
    [EditorType.DESIGN]: PanelVisibility;
    [EditorType.TYPE]: PanelVisibility;
  };
  editorSettings: EditorSettings;
  panelSizes: PanelSizes;
}
export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

export interface SketchpadDiff {
  navigation?: string;
  mode?: Mode;
  theme?: Theme;
  layout?: Layout;
  activeDesignEditor?: DesignEditorId;
  panelVisibility?: {
    [EditorType.HOME]?: PanelVisibility;
    [EditorType.KIT]?: PanelVisibility;
    [EditorType.DESIGN]?: PanelVisibility;
    [EditorType.TYPE]?: PanelVisibility;
  };
  editorSettings?: EditorSettings;
  panelSizes?: Partial<PanelSizes>;
}

export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
}
export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
}

class SketchpadStore {
  private readonly id: string | undefined;
  private readonly yProviderFactory: YProviderFactory | undefined;
  private readonly yDoc: Y.Doc;
  private readonly ySketchpad: YSketchpad;
  private readonly kits: Map<string, KitStore>;
  private readonly yKitEditors: YKitEditors;
  private readonly kitEditors: Map<string, KitEditorStore>;
  private readonly yDesignEditors: YDesignEditors;
  private readonly designEditors: Map<string, DesignEditorStore>;
  private readonly persistence?: IndexeddbPersistence;
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult>;
  private cache?: SketchpadState;
  private cacheHash?: string;
  private kitShallowsCache?: KitShallow[];
  private kitShallowsCacheHash?: string;
  private readonly kitCreatedSubscribers: Set<Subscribe>;
  private readonly kitDeletedSubscribers: Set<Subscribe>;
  private readonly kitEditorCreatedSubscribers: Set<Subscribe>;
  private readonly kitEditorDeletedSubscribers: Set<Subscribe>;
  private readonly designEditorCreatedSubscribers: Set<Subscribe>;
  private readonly designEditorDeletedSubscribers: Set<Subscribe>;
  // private readonly broadcastChannel: BroadcastChannel;

  constructor(id?: string, yProviderFactory?: YProviderFactory) {
    this.id = id;
    this.yProviderFactory = yProviderFactory;
    // this.broadcastChannel = new BroadcastChannel(`semio-sketchpad-${id}`);
    this.yDoc = new Y.Doc();
    this.kits = new Map();
    this.kitEditors = new Map();
    this.designEditors = new Map();
    this.commandRegistry = new Map();
    this.kitCreatedSubscribers = new Set();
    this.kitDeletedSubscribers = new Set();
    this.kitEditorCreatedSubscribers = new Set();
    this.kitEditorDeletedSubscribers = new Set();
    this.designEditorCreatedSubscribers = new Set();
    this.designEditorDeletedSubscribers = new Set();

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
    this.yKitEditors = this.yDoc.getArray("kitEditors");
    this.yDesignEditors = this.yDoc.getArray("designEditors");
    this.yDoc.transact(() => {
      this.ySketchpad.set("navigation", "/");
      this.ySketchpad.set("mode", Mode.GUEST);
      this.ySketchpad.set("theme", Theme.SYSTEM);
      this.ySketchpad.set("layout", Layout.NORMAL);
      this.ySketchpad.set(
        "panelVisibility",
        JSON.stringify({
          [EditorType.HOME]: {},
          [EditorType.KIT]: { console: false, settings: false },
          [EditorType.DESIGN]: { workbench: false, details: false, console: false, chat: false, settings: false },
          [EditorType.TYPE]: { workbench: false, console: false, settings: false },
        }),
      );
      this.ySketchpad.set(
        "editorSettings",
        JSON.stringify({
          design: { snappiness: 10, gridSize: 24 },
          type: {},
          kit: {},
        }),
      );
      this.ySketchpad.set(
        "panelSizes",
        JSON.stringify({
          workbenchWidth: 230,
          detailsWidth: 230,
          chatWidth: 230,
          settingsWidth: 230,
          consoleHeight: 200,
        }),
      );
    });

    Object.entries(sketchpadCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  hash = (state: SketchpadState): string => {
    return JSON.stringify(state);
  };

  snapshot = (): SketchpadState => {
    const panelVisibilityStr = this.ySketchpad.get("panelVisibility") as string;
    const panelVisibility = panelVisibilityStr
      ? JSON.parse(panelVisibilityStr)
      : {
          [EditorType.HOME]: { chat: false, settings: false },
          [EditorType.KIT]: { console: false, settings: false },
          [EditorType.DESIGN]: { workbench: false, details: false, console: false, chat: false, settings: false },
          [EditorType.TYPE]: { workbench: false, console: false, settings: false },
        };
    const editorSettingsStr = this.ySketchpad.get("editorSettings") as string;
    const editorSettings = editorSettingsStr
      ? JSON.parse(editorSettingsStr)
      : {
          design: { snappiness: 10, gridSize: 24 },
          type: {},
          kit: {},
        };
    const panelSizesStr = this.ySketchpad.get("panelSizes") as string;
    const panelSizes = panelSizesStr
      ? JSON.parse(panelSizesStr)
      : {
          workbenchWidth: 230,
          detailsWidth: 230,
          chatWidth: 230,
          settingsWidth: 230,
          consoleHeight: 200,
        };
    const currentValues = {
      navigation: this.ySketchpad.get("navigation") as string,
      mode: this.ySketchpad.get("mode") as Mode,
      theme: this.ySketchpad.get("theme") as Theme,
      layout: this.ySketchpad.get("layout") as Layout,
      panelVisibility: panelVisibility,
      editorSettings: editorSettings,
      panelSizes: panelSizes,
    };
    const currentHash = this.hash(currentValues);
    if (!this.cache || this.cacheHash !== currentHash) {
      this.cache = currentValues;
      this.cacheHash = currentHash;
    }
    return this.cache;
  };

  createKit = (kit: Kit) => {
    // if (this.hasKit(kit)) {
    //   throw new Error(`Kit (${kit.name}, ${kit.version || ""}) already exists.`);
    // }
    const kitStore = new KitStore(this, kit, this.yProviderFactory);
    this.kits.set(kit.guid, kitStore);
    this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createKitEditor = (id: KitEditorId) => {
    // if (this.hasKitEditor(id)) {
    //   throw new Error(`Kit editor (${id.kit.name}, ${id.kit.version || ""}) already exists.`);
    // }
    this.yDoc.transact(() => {
      const yKitEditor = new Y.Map<YKitEditorVal>();
      this.yKitEditors.push([yKitEditor]);
      const kitEditor = new KitEditorStore(this, yKitEditor, this.yDoc.transact.bind(this.yDoc), id);
      const kitStore = this.kit(id.kit);
      this.kitEditors.set(kitStore.guid, kitEditor);
    });
    this.kitEditorCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  createDesignEditor = (id: DesignEditorId) => {
    if (this.hasDesignEditor(id)) {
      throw new Error(`Design editor (${id.kit.name}, ${id.kit.version || ""}, ${id.design.name}, ${id.design.variant || ""}, ${id.design.view || ""}) already exists.`);
    }
    this.yDoc.transact(() => {
      const yDesignEditor = new Y.Map<YDesignEditorVal>();
      this.yDesignEditors.push([yDesignEditor]);
      const designEditor = new DesignEditorStore(this, yDesignEditor, this.yDoc.transact.bind(this.yDoc), id);
      const kitStore = this.kit(id.kit);
      const designStore = kitStore.design(id.design);
      this.designEditors.set(`${kitStore.guid}:${designStore.guid}`, designEditor);
    });
    this.designEditorCreatedSubscribers.forEach((subscriber) => subscriber());
  };

  change(diff: SketchpadDiff) {
    this.yDoc.transact(() => {
      if (diff.navigation) this.ySketchpad.set("navigation", diff.navigation);
      if (diff.mode) this.ySketchpad.set("mode", diff.mode);
      if (diff.theme) this.ySketchpad.set("theme", diff.theme);
      if (diff.layout) this.ySketchpad.set("layout", diff.layout);
      if (diff.panelVisibility) {
        const current = JSON.parse((this.ySketchpad.get("panelVisibility") as string) || "{}");
        this.ySketchpad.set("panelVisibility", JSON.stringify({ ...current, ...diff.panelVisibility }));
      }
      if (diff.editorSettings) {
        const current = JSON.parse((this.ySketchpad.get("editorSettings") as string) || "{}");
        this.ySketchpad.set("editorSettings", JSON.stringify({ ...current, ...diff.editorSettings }));
      }
      if (diff.panelSizes) {
        const current = JSON.parse((this.ySketchpad.get("panelSizes") as string) || "{}");
        this.ySketchpad.set("panelSizes", JSON.stringify({ ...current, ...diff.panelSizes }));
      }
    });
  }

  deleteKit = (id: KitIdLike) => {
    const kitStore = Array.from(this.kits.values()).find((k) => areSameKit(k.id(), id));
    if (kitStore) {
      this.kits.delete(kitStore.guid);
      this.kitDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  deleteKitEditor = (id: KitEditorId) => {
    const kitEditorStore = Array.from(this.kitEditors.entries()).find(([_, k]) => areSameKitEditor(k.id(), id));
    if (kitEditorStore) {
      const [guid, editor] = kitEditorStore;
      this.kitEditors.delete(guid);
      const index = Array.from(this.yKitEditors.toArray()).findIndex((y) => {
        const yMap = y as Y.Map<any>;
        return yMap.get("kit") === guid;
      });
      if (index !== -1) {
        this.yDoc.transact(() => {
          this.yKitEditors.delete(index, 1);
        });
      }
      this.kitEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  deleteDesignEditor = (id: DesignEditorId) => {
    const designEditorStore = Array.from(this.designEditors.entries()).find(([_, d]) => areSameDesignEditor(d.id(), id));
    if (designEditorStore) {
      const [guid, editor] = designEditorStore;
      this.designEditors.delete(guid);
      const index = Array.from(this.yDesignEditors.toArray()).findIndex((y) => {
        const yMap = y as Y.Map<any>;
        const kitGuid = yMap.get("kit");
        const designGuid = yMap.get("design");
        return guid === `${kitGuid}:${designGuid}`;
      });
      if (index !== -1) {
        this.yDoc.transact(() => {
          this.yDesignEditors.delete(index, 1);
        });
      }
      this.designEditorDeletedSubscribers.forEach((subscriber) => subscriber());
    }
  };

  onKitCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitCreatedSubscribers.add(subscribe);
    return () => {
      this.kitCreatedSubscribers.delete(subscribe);
    };
  };

  onKitEditorCreated = (subscribe: Subscribe): Unsubscribe => {
    this.kitEditorCreatedSubscribers.add(subscribe);
    return () => {
      this.kitEditorCreatedSubscribers.delete(subscribe);
    };
  };

  onDesignEditorCreated = (subscribe: Subscribe): Unsubscribe => {
    this.designEditorCreatedSubscribers.add(subscribe);
    return () => {
      this.designEditorCreatedSubscribers.delete(subscribe);
    };
  };

  onKitDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitDeletedSubscribers.add(subscribe);
    return () => {
      this.kitDeletedSubscribers.delete(subscribe);
    };
  };

  onKitEditorDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.kitEditorDeletedSubscribers.add(subscribe);
    return () => {
      this.kitEditorDeletedSubscribers.delete(subscribe);
    };
  };

  onDesignEditorDeleted = (subscribe: Subscribe): Unsubscribe => {
    this.designEditorDeletedSubscribers.add(subscribe);
    return () => {
      this.designEditorDeletedSubscribers.delete(subscribe);
    };
  };

  onChanged = (subscribe: Subscribe): Unsubscribe => {
    return createObserver(this.ySketchpad, subscribe);
  };

  onChangedDeep = (subscribe: Subscribe): Unsubscribe => {
    return createObserver(this.ySketchpad, subscribe, true);
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.sketchpad.createKit") {
      const kit = rest[0] as Kit;
      this.createKit(kit);
      return {} as T;
    }
    if (command === "semio.sketchpad.createKitEditor") {
      const id = rest[0] as KitEditorId;
      this.createKitEditor(id);
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

  hasKit(guid: string): boolean {
    return this.kits.has(guid);
  }

  kit(guid: string): KitStore {
    return this.kits.get(guid)!;
  }

  kitShallows(): KitShallow[] {
    const currentKits = Array.from(this.kits.values()).map((k) => k.snapshot() as KitShallow);
    const currentHash = JSON.stringify(currentKits.map((k) => [k.name, k.version, k.description]));

    if (!this.kitShallowsCache || this.kitShallowsCacheHash !== currentHash) {
      this.kitShallowsCache = currentKits;
      this.kitShallowsCacheHash = currentHash;
    }

    return this.kitShallowsCache;
  }

  hasKitEditor(kitEditor: KitEditorId): boolean {
    return hasSameKitEditor(
      kitEditor,
      Array.from(this.kitEditors.values()).map((kitEditor) => kitEditor.id()),
    );
  }

  kitEditor(guid: string): KitEditorStore {
    return this.kitEditors.get(guid)!;
  }

  kitEditorIds(): KitEditorId[] {
    return Array.from(this.kitEditors.values()).map((k) => k.id());
  }

  hasDesignEditor(designEditor: DesignEditorId): boolean {
    return hasSameDesignEditor(
      designEditor,
      Array.from(this.designEditors.values()).map((designEditor) => designEditor.id()),
    );
  }

  designEditor(guid: string): DesignEditorStore {
    return this.designEditors.get(guid)!;
  }

  designEditorIds(): DesignEditorId[] {
    return Array.from(this.designEditors.values()).map((d) => d.id());
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
};

const stores: Map<Guid, SketchpadStore> = new Map();

const loadPersistedKits = () => {
  // TODO: proper edge case handeling
  const semioRaw = localStorage.getItem("semio");
  if (semio) {
    const semio = JSON.parse(semio);
    const { kits } = semio;
    for (const kit in kits) {
    }
  }
};

// TODO: Find clean way to hide Scope and extra hook and still pass window events to navbar
export type WindowEvents = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
};
export type SketchpadScope = { id: string; yProviderFactory?: YProviderFactory; onWindowEvents?: WindowEvents };
const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
export const SketchpadScopeProvider = (props: { id?: string; yProviderFactory?: YProviderFactory; onWindowEvents?: WindowEvents; children: React.ReactNode }) => {
  const id = props.id || guid();
  if (!stores.has(id)) {
    const store = new SketchpadStore(props.id, props?.yProviderFactory);
    stores.set(id, store);
  }
  return React.createElement(SketchpadScopeContext.Provider, { value: { id, onWindowEvents: props.onWindowEvents } }, props.children as any);
};
export const useSketchpadScope = () => useContext(SketchpadScopeContext);

function useSketchpadStore(id?: string): SketchpadStore {
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

export function useNavigation(): string {
  return useSketchpad((s) => s.navigation) as string;
}

export function getEditorTypeFromPath(path: string): EditorType {
  if (path === "/") return EditorType.HOME;
  if (path.match(/^\/[^/]+\/d\/[^/]+/)) return EditorType.DESIGN;
  if (path.match(/^\/[^/]+\/t\/[^/]+/)) return EditorType.TYPE;
  if (path.match(/^\/[^/]+$/)) return EditorType.KIT;
  return EditorType.HOME;
}

export function useEditorType(): EditorType {
  const navigation = useNavigation();
  return useMemo(() => getEditorTypeFromPath(navigation), [navigation]);
}

export function useMode(): Mode {
  return useSketchpad((s) => s.mode) as Mode;
}

export function useTheme(): Theme {
  return useSketchpad((s) => s.theme) as Theme;
}

export function useLayout(): Layout {
  return useSketchpad((s) => s.layout) as Layout;
}

export function useSketchpadCommands() {
  const store = useSketchpadStore();
  const navigate = useNavigate();
  return {
    setMode: (mode: Mode) => store.execute("semio.sketchpad.setMode", mode),
    setTheme: (theme: Theme) => store.execute("semio.sketchpad.setTheme", theme),
    setLayout: (layout: Layout) => store.execute("semio.sketchpad.setLayout", layout),
    createKit: (kit: Kit) => store.execute("semio.sketchpad.createKit", kit),
    createKitEditor: (kitEditorId: KitEditorId) => store.execute("semio.sketchpad.createKitEditor", kitEditorId),
    createDesignEditor: (designEditorId: DesignEditorId) => store.execute("semio.sketchpad.createDesignEditor", designEditorId),
    navigateToKit: (guid: Guid) => navigate(`/k/${guid}`),
    navigateToDesign: (guid: Guid) => navigate(`/d/${guid}`),
    navigateToType: (guid: Guid) => navigate(`/t/${guid}`),
    togglePanel: (editorType: EditorType, panelKey: string) => {
      const current = store.snapshot().panelVisibility[editorType] || {};
      store.change({
        panelVisibility: {
          [editorType]: {
            ...current,
            [panelKey]: !current[panelKey],
          },
        },
      });
    },
    updateEditorSettings: (editorType: "design" | "type" | "kit", settings: Record<string, any>) => {
      const current = store.snapshot().editorSettings;
      store.change({
        editorSettings: {
          ...current,
          [editorType]: { ...current[editorType], ...settings },
        },
      });
    },
    setPanelSize: (panelKey: keyof PanelSizes, size: number) => {
      store.change({
        panelSizes: {
          [panelKey]: size,
        },
      });
    },
  };
}

export function useKits(): KitShallow[] {
  const store = useSketchpadStore();

  const kits = useSyncExternalStore(
    (onStoreChange) => {
      const unsubscribeCreated = store.onKitCreated(onStoreChange);
      const unsubscribeDeleted = store.onKitDeleted(onStoreChange);
      const unsubscribers = Array.from(store.kits.values()).map((kit) => {
        const kitStore = store.kit(kit.guid);
        return kitStore.onChanged(onStoreChange);
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

// #endregion Sketchpad
