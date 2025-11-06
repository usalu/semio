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

import React, { createContext, useContext, useMemo } from "react";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import {
  Attribute,
  Author,
  AuthorDiff,
  Benchmark,
  BenchmarkDiff,
  Camera,
  CameraDiff,
  Connection,
  ConnectionDiff,
  Coord,
  CoordDiff,
  Design,
  DesignDiff,
  DesignShallow,
  DiffStatus,
  FileDiff,
  Folder,
  FolderDiff,
  Group,
  GroupDiff,
  Guid,
  Kit,
  KitDiff,
  KitShallow,
  Layer,
  LayerDiff,
  Location,
  LocationDiff,
  Piece,
  PieceDiff,
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
  Vec,
  VecDiff,
  Vector,
  VectorDiff,
  applyDesignDiff,
  findPieceInDesign,
  findReplacableDesignsForDesignPiece,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  flattenDesign,
  getIncludedDesigns,
  getPieceRepresentationUrls,
  piecesMetadata,
} from "../../semio";
import type { FileProvider, RemoteProviders, SketchpadStore, Url } from "../store";
import { Disposable, Subscribe, createObserver, identitySelector, useSketchpadStore, useSync, useSyncDeep } from "../store";
import { commands as kitCommands } from "./commands";

// Re-export utilities from parent store for use in designAppIntegration
export { identitySelector };

// Note: useConnectionColor, useDiffedDesign, usePieceWithDiff, and usePortColoredTypes
// have been moved to designHelpers.ts and are NOT re-exported here to avoid circular dependencies.
// Internal use in this file imports directly from "./designHelpers".

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

function useAuthorStore<T>(selector?: (store: AuthorStore) => T, guid?: string): T | AuthorStore {
  const kitStore = useKitStore() as KitStore;
  const authorScope = useAuthorScope();
  const authorGuid = authorScope?.guid ?? guid;
  if (!authorGuid) throw new Error("useAuthorStore must be called within a AuthorScopeProvider or be directly provided with a guid");
  if (!kitStore.hasAuthor(authorGuid)) throw new Error(`Author store not found for author ${authorGuid}`);
  const authorStore = kitStore.author(authorGuid);
  return selector ? selector(authorStore) : authorStore;
}

export function useAuthor<T>(selector?: (author: Author) => T, id?: Guid, deep: boolean = false): T | Author | null {
  return useSync<Author, T>(useAuthorStore(identitySelector, id) as AuthorStore, selector ? selector : identitySelector, deep);
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
  private cache?: import("../../semio").Folder;
  private cacheHash?: string;

  constructor(yFolder: YFolder, folder: import("../../semio").Folder) {
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

  hashFolder = (folder: import("../../semio").Folder): string => {
    return JSON.stringify(folder);
  };

  snapshot = (): import("../../semio").Folder => {
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

  change = (diff: import("../../semio").FolderDiff) => {
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
  const kitStore = useKitStore() as KitStore;
  const typeScope = useTypeScope();
  const typeGuid = typeScope?.guid ?? guid;
  if (!typeGuid) return null;
  if (!kitStore.hasType(typeGuid)) return null;
  const typeStore = kitStore.type(typeGuid);
  if (!typeStore) return null;
  return selector ? selector(typeStore) : typeStore;
}

export function useType<T>(selector?: (type: Type) => T, id?: Guid, deep: boolean = false): T | Type | null {
  return useSync<Type, T>(useTypeStore(identitySelector, id) as TypeStore | null, selector ? selector : identitySelector, deep);
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
  const kitStore = useKitStore() as KitStore;
  const qualityScope = useQualityScope();
  const qualityGuid = qualityScope?.guid ?? guid;
  if (!qualityGuid) return null;
  if (!kitStore.hasQuality(qualityGuid)) return null;
  const qualityStore = kitStore.quality(qualityGuid);
  if (!qualityStore) return null;
  return selector ? selector(qualityStore) : qualityStore;
}

export function useQuality<T>(selector?: (quality: Quality) => T, id?: Guid, deep: boolean = false): T | Quality | null {
  return useSync<Quality, T>(useQualityStore(identitySelector, id) as QualityStore | null, selector ? selector : identitySelector, deep);
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
  return useSync<Piece, T>(usePieceStore(identitySelector, id) as PieceStore, selector ? selector : identitySelector, deep);
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
  return useSync<Connection, T>(useConnectionStore(identitySelector, id) as ConnectionStore, selector ? selector : identitySelector, deep);
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

function useDesignStore<T>(selector?: (store: DesignStore) => T, guid?: string): T | DesignStore {
  const kitStore = useKitStore() as KitStore;
  const designScope = useDesignScope();
  const designGuid = designScope?.guid ?? guid;
  if (!designGuid) throw new Error("useDesignStore must be called within a DesignScopeProvider or be directly provided with a guid");
  if (!kitStore.hasDesign(designGuid)) throw new Error(`Design store not found for design ${designGuid}`);
  const designStore = kitStore.design(designGuid);
  return selector ? selector(designStore) : designStore;
}

export function useDesign<T>(selector?: (design: DesignShallow | Design) => T, id?: Guid, deep: boolean = false): T | DesignShallow | Design | null {
  if (deep) {
    return useSyncDeep<Design, T>(useDesignStore(identitySelector, id) as DesignStore, selector ? selector : identitySelector);
  }
  return useSync<DesignShallow, T>(useDesignStore(identitySelector, id) as any, selector ? selector : identitySelector, deep);
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

export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}

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
      this.registerCommand(commandId, command);
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

export function useKitStore<T>(selector?: (store: KitStore) => T, guid?: string): T | KitStore {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid ?? guid;
  if (!kitGuid) throw new Error("useKitStore must be called within a KitScopeProvider or be directly provided with a guid");
  if (!store.hasKit(kitGuid)) throw new Error(`Kit store not found for kit ${kitGuid}`);
  const kitStore = store.kit(kitGuid);
  return selector ? selector(kitStore) : kitStore;
}

export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, guid?: Guid, deep: boolean = false): T | KitShallow | Kit | null {
  if (deep) {
    return useSyncDeep<Kit, T>(useKitStore(identitySelector, guid) as KitStore, selector ? selector : identitySelector);
  }
  return useSync<KitShallow, T>(useKitStore(identitySelector, guid) as any, selector ? selector : identitySelector, deep);
}

// useDiffedKit - moved to designAppIntegration.ts

export function useDesigns(): Design[] {
  return useKit((k) => k.designs ?? []) as Design[];
}

export function useFileUrls(): Map<Url, Url> {
  return (useKitStore() as KitStore).fileUrls;
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
