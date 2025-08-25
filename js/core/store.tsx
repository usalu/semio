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

import React, { createContext, useContext, useMemo, useSyncExternalStore } from "react";
import { v4 as uuidv4 } from "uuid";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import {
  Attribute,
  Author,
  Camera,
  Connection,
  ConnectionDiff,
  ConnectionId,
  connectionIdLikeToConnectionId,
  Design,
  DesignDiff,
  DesignId,
  designIdLikeToDesignId,
  DiagramPoint,
  FileDiff,
  findDesignInKit,
  Kit,
  KitDiff,
  KitId,
  KitIdLike,
  kitIdLikeToKitId,
  Piece,
  PieceDiff,
  PieceId,
  pieceIdLikeToPieceId,
  Port,
  PortDiff,
  PortId,
  portIdLikeToPortId,
  Representation,
  RepresentationDiff,
  RepresentationId,
  representationIdLikeToRepresentationId,
  File as SemioFile,
  Type,
  TypeDiff,
  TypeId,
  typeIdLikeToTypeId,
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
  diff: KitDiff;
  selection: DesignEditorSelection;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface TypeChildStores {
  representations: Map<RepresentationId, RepresentationStore>;
  ports: Map<PortId, PortStore>;
}
export interface TypeChildStoresFull {
  representations: Map<RepresentationId, RepresentationStoreFull>;
  ports: Map<PortId, PortStoreFull>;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
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
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface DesignChildStores {
  pieces: Map<PieceId, PieceStore>;
  connections: Map<ConnectionId, ConnectionStore>;
}
export interface DesignChildStoresFull {
  pieces: Map<PieceId, PieceStoreFull>;
  connections: Map<ConnectionId, ConnectionStoreFull>;
}
export interface DesignStore extends DesignSnapshot, DesignChildStores {}
export interface DesignStoreFull extends DesignSnapshot, DesignActions, DesignSubscriptions, DesignChildStoresFull {}

export interface KitSnapshot {
  snapshot(): Kit;
  fileUrls(): Map<Url, Url>;
}
export interface KitActions {
  change: (diff: KitDiff) => void;
}
export interface KitSubscriptions {
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
}
export interface KitCommandContext extends KitSnapshot {}
export interface KitCommandResult {
  diff: KitDiff;
}
export interface KitCommands {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
}
export interface KitCommandsFull {
  execute<T>(command: string, ...rest: any[]): Promise<T>;
  register(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable;
}
export interface KitChildStores {
  types: Map<TypeId, TypeStore>;
  designs: Map<DesignId, DesignStore>;
  files: Map<Url, FileStore>;
}
export interface KitChildStoresFull {
  types: Map<TypeId, TypeStoreFull>;
  designs: Map<DesignId, DesignStoreFull>;
  files: Map<Url, FileStoreFull>;
}
export interface KitStore extends KitSnapshot, KitChildStores {}
export interface KitStoreFull extends KitSnapshot, KitActions, KitSubscriptions, KitCommandsFull, KitChildStoresFull {}

export interface DesignEditorId {
  kitId: KitId;
  designId: DesignId;
}
export interface DesignEditorSelection {
  pieceIds?: PieceId[];
  connectionIds?: ConnectionId[];
  portId?: { pieceId: PieceId; designPieceId?: PieceId; portId: PortId };
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
  selection?: DesignEditorSelection;
  presence?: DesignEditorPresence;
  others?: DesignEditorPresenceOther[];
}
export interface DesignEditorActions {
  undo: () => void;
  redo: () => void;
  change: (diff: DesignEditorStateDiff) => void;
  transaction: {
    start: () => void;
    abort: () => void;
    finalize: () => void;
  };
}
export interface DesignEditorSubscriptions {
  undone: (subscribe: Subscribe) => Unsubscribe;
  redone: (subscribe: Subscribe) => Unsubscribe;
  changed: (subscribe: Subscribe) => Unsubscribe;
  transaction: {
    started: (subscribe: Subscribe) => Unsubscribe;
    aborted: (subscribe: Subscribe) => Unsubscribe;
    finalized: (subscribe: Subscribe) => Unsubscribe;
  };
}
export interface DesignEditorCommandContext {
  sketchpadState: SketchpadStateFull;
  state: DesignEditorStateFull;
  fileUrls: Map<Url, Url>;
  kit: Kit;
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
export interface DesignEditorStore extends DesignEditorSnapshot, DesignEditorCommands {
  changed: (subscribe: Subscribe) => Unsubscribe;
}
export interface DesignEditorStoreFull extends DesignEditorSnapshot, DesignEditorCommandsFull, DesignEditorActions {
  on: DesignEditorSubscriptions;
  changed: (subscribe: Subscribe) => Unsubscribe;
}
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
  create: {
    kit: (kit: Kit) => void;
    designEditor: (id: DesignEditorId) => void;
  };
  change: (diff: SketchpadStateDiff) => void;
  delete: {
    kit: (id: KitIdLike) => void;
    designEditor: (id: DesignEditorId) => void;
  };
}
export interface SketchpadSubscriptions {
  created: {
    kit: (subscribe: Subscribe) => Unsubscribe;
    designEditor: (subscribe: Subscribe) => Unsubscribe;
  };
  changed: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  deleted: {
    kit: (subscribe: Subscribe) => Unsubscribe;
    designEditor: (subscribe: Subscribe) => Unsubscribe;
  };
}
export interface SketchpadChildStores {
  kits: Map<KitId, KitStore>;
  designEditors: Map<KitId, Map<DesignId, DesignEditorStore>>;
}
export interface SketchpadChildStoresFull {
  kits: Map<KitId, KitStoreFull>;
  designEditors: Map<KitId, Map<DesignId, DesignEditorStoreFull>>;
}
export interface SketchpadCommandContext {
  state: SketchpadStateFull;
  kits: Kit[];
  fileUrls: Map<Url, Url>;
}
export interface SketchpadCommandResult {
  diff: SketchpadStateDiff;
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
  (attributes || []).forEach((q) => yArr.push([createAttribute(q)]));
  return yArr;
}

function getAttributes(yArr: YAttributes | undefined): Attribute[] {
  if (!yArr) return [];
  const list: Attribute[] = [];
  yArr.forEach((yMap: YAttribute) => {
    list.push({
      key: yMap.get("key") as string,
      value: yMap.get("value") as string | undefined,
      definition: yMap.get("definition") as string | undefined,
    });
  });
  return list;
}

function createAuthor(author: Author): YAuthor {
  const yAuthor = new Y.Map<string>();
  yAuthor.set("name", author.name);
  yAuthor.set("email", author.email || "");
  return yAuthor;
}

function createAuthors(authors: Author[] | undefined): YAuthors {
  const yAuthors = new Y.Map<YAuthor>();
  (authors || []).forEach((a) => yAuthors.set(a.name, createAuthor(a)));
  return yAuthors;
}

function getAuthors(yAuthors: YAuthors | undefined): Author[] {
  if (!yAuthors) return [];
  const authors: Author[] = [];
  yAuthors.forEach((yAuthor: YAuthor) =>
    authors.push({
      name: yAuthor.get("name") as string,
      email: (yAuthor.get("email") as string) || "",
    }),
  );
  return authors;
}

class YFileStore implements FileStoreFull {
  public readonly parent: YKitStore;
  public readonly yFile: Y.Map<string> = new Y.Map<string>();

  constructor(parent: YKitStore, file: SemioFile) {
    this.parent = parent;
    this.yFile.set("url", file.url);
    this.yFile.set("data", file.data);
    this.yFile.set("size", file.size?.toString() || "");
    this.yFile.set("hash", file.hash || "");
    this.yFile.set("created", file.created?.toISOString() || "");
    this.yFile.set("updated", file.updated?.toISOString() || "");
  }

  get file(): SemioFile {
    const url = this.yFile.get("url") as string;
    const data = this.yFile.get("data") as string;
    const file: SemioFile = { url, data };
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
    if (diff.url !== undefined) this.yFile.set("url", diff.url);
    if (diff.data !== undefined) this.yFile.set("data", diff.data);
    if (diff.size !== undefined) this.yFile.set("size", diff.size.toString());
    if (diff.hash !== undefined) this.yFile.set("hash", diff.hash);
  };

  snapshot = (): SemioFile => {
    return this.file;
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yFile.observeDeep(observer);
      return () => this.yFile.unobserveDeep(observer);
    } else {
      this.yFile.observe(observer);
      return () => this.yFile.unobserve(observer);
    }
  };
}

class YRepresentationStore implements RepresentationStoreFull {
  public readonly parent: YKitStore;
  public readonly yRepresentation: YRepresentation = new Y.Map<any>();

  constructor(parent: YKitStore, representation: Representation) {
    this.parent = parent;
    this.yRepresentation.set("url", representation.url);
    this.yRepresentation.set("description", representation.description || "");
    const yTags = new Y.Array<string>();
    this.yRepresentation.set("tags", yTags);
    (representation.tags || []).forEach((t) => yTags.push([t]));
    this.yRepresentation.set("attributes", createAttributes(representation.attributes));
  }

  snapshot = (): Representation => {
    const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
    return {
      url: this.yRepresentation.get("url") as string,
      description: (this.yRepresentation.get("description") as string) || "",
      tags: yTags ? yTags.toArray() : [],
      attributes: getAttributes(this.yRepresentation.get("attributes") as YAttributes),
    };
  };

  change = (diff: RepresentationDiff) => {
    if (diff.url !== undefined) this.yRepresentation.set("url", diff.url);
    if (diff.description !== undefined) this.yRepresentation.set("description", diff.description);
    if (diff.tags !== undefined) {
      const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
      yTags.delete(0, yTags.length);
      diff.tags.forEach((tag) => yTags.push([tag]));
    }
    if (diff.attributes !== undefined) {
      const yAttributes = this.yRepresentation.get("attributes") as YAttributes;
      yAttributes.delete(0, yAttributes.length);
      diff.attributes.forEach((q) => yAttributes.push([createAttribute(q)]));
    }
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yRepresentation.observeDeep(observer);
      return () => this.yRepresentation.unobserveDeep(observer);
    } else {
      this.yRepresentation.observe(observer);
      return () => this.yRepresentation.unobserve(observer);
    }
  };
}

class YPortStore implements PortStoreFull {
  public readonly parent: YTypeStore;
  public readonly yPort: YPort = new Y.Map<any>();

  constructor(parent: YTypeStore, port: Port) {
    this.parent = parent;
    this.yPort.set("id_", port.id_ || "");
    this.yPort.set("description", port.description || "");
    this.yPort.set("mandatory", port.mandatory || false);
    this.yPort.set("family", port.family || "");
    this.yPort.set("t", port.t);
    const yCompatibleFamilies = new Y.Array<string>();
    this.yPort.set("compatibleFamilies", yCompatibleFamilies);
    (port.compatibleFamilies || []).forEach((f) => yCompatibleFamilies.push([f]));
    const yPoint = new Y.Map<number>();
    yPoint.set("x", port.point.x);
    yPoint.set("y", port.point.y);
    yPoint.set("z", port.point.z);
    this.yPort.set("point", yPoint);
    const yDirection = new Y.Map<number>();
    yDirection.set("x", port.direction.x);
    yDirection.set("y", port.direction.y);
    yDirection.set("z", port.direction.z);
    this.yPort.set("direction", yDirection);
    const yAttributes = new Y.Array<YAttribute>();
    this.yPort.set("attributes", yAttributes);
    (port.attributes || []).forEach((q) => yAttributes.push([createAttribute(q)]));
  }

  snapshot = (): Port => {
    const yCompatibleFamilies = this.yPort.get("compatibleFamilies") as Y.Array<string>;
    const yPoint = this.yPort.get("point") as Y.Map<number>;
    const yDirection = this.yPort.get("direction") as Y.Map<number>;

    return {
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
      attributes: getAttributes(this.yPort.get("attributes") as YAttributes),
    };
  };

  change = (diff: PortDiff) => {
    if (diff.id_ !== undefined) this.yPort.set("id_", diff.id_);
    if (diff.description !== undefined) this.yPort.set("description", diff.description);
    if (diff.mandatory !== undefined) this.yPort.set("mandatory", diff.mandatory);
    if (diff.family !== undefined) this.yPort.set("family", diff.family);
    if (diff.t !== undefined) this.yPort.set("t", diff.t);
    if (diff.compatibleFamilies !== undefined) {
      const yCompatibleFamilies = this.yPort.get("compatibleFamilies") as Y.Array<string>;
      yCompatibleFamilies.delete(0, yCompatibleFamilies.length);
      diff.compatibleFamilies.forEach((family) => yCompatibleFamilies.push([family]));
    }
    if (diff.point !== undefined) {
      const yPoint = this.yPort.get("point") as Y.Map<number>;
      if (diff.point.x !== undefined) yPoint.set("x", diff.point.x);
      if (diff.point.y !== undefined) yPoint.set("y", diff.point.y);
      if (diff.point.z !== undefined) yPoint.set("z", diff.point.z);
    }
    if (diff.direction !== undefined) {
      const yDirection = this.yPort.get("direction") as Y.Map<number>;
      if (diff.direction.x !== undefined) yDirection.set("x", diff.direction.x);
      if (diff.direction.y !== undefined) yDirection.set("y", diff.direction.y);
      if (diff.direction.z !== undefined) yDirection.set("z", diff.direction.z);
    }
    if (diff.attributes !== undefined) {
      const yAttributes = this.yPort.get("attributes") as YAttributes;
      yAttributes.delete(0, yAttributes.length);
      createAttributes(diff.attributes).forEach((q) => yAttributes.push([q]));
    }
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yPort.observeDeep(observer);
      return () => this.yPort.unobserveDeep(observer);
    } else {
      this.yPort.observe(observer);
      return () => this.yPort.unobserve(observer);
    }
  };
}

class YTypeStore implements TypeStoreFull {
  public readonly parent: YKitStore;
  public readonly yType: YType = new Y.Map<any>();
  public readonly representations: Map<RepresentationId, YRepresentationStore> = new Map();
  public readonly ports: Map<PortId, YPortStore> = new Map();
  private readonly representationIds: Map<RepresentationId, string> = new Map();
  private readonly portIds: Map<PortId, string> = new Map();

  constructor(parent: YKitStore, type: Type) {
    this.parent = parent;
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
    (type.representations || []).forEach((r) => this.representationIds.set(representationIdLikeToRepresentationId(r), uuidv4()));
    (type.ports || []).forEach((p) => this.portIds.set(portIdLikeToPortId(p), uuidv4()));
  }

  type = (): Type => {
    return {
      name: this.yType.get("name") as string,
      description: (this.yType.get("description") as string) || "",
      variant: this.yType.get("variant") as string | undefined,
      unit: (this.yType.get("unit") as string) || "",
      stock: this.yType.get("stock") as number | undefined,
      virtual: this.yType.get("virtual") as boolean | undefined,
      representations: Array.from(this.representations.values()).map((store) => store.snapshot()),
      ports: Array.from(this.ports.values()).map((store) => store.snapshot()),
      authors: getAuthors(this.yType.get("authors") as YAuthors),
      attributes: getAttributes(this.yType.get("attributes") as YAttributes),
    };
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
        if (!this.representationIds.get(repId)) {
          const uuid = uuidv4();
          this.representationIds.set(repId, uuid);
          this.representations.set(repId, new YRepresentationStore(this.parent, rep));
        } else {
          this.representations.get(repId)?.change(rep);
        }
      });
    }
    if (diff.ports !== undefined) {
      diff.ports.forEach((port) => {
        const portId = portIdLikeToPortId(port);
        if (!this.portIds.get(portId)) {
          const uuid = uuidv4();
          this.portIds.set(portId, uuid);
          this.ports.set(portId, new YPortStore(this, port));
        } else {
          this.ports.get(portId)?.change(port);
        }
      });
    }
  };

  snapshot = (): Type => {
    return this.type();
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yType.observeDeep(observer);
      return () => this.yType.unobserveDeep(observer);
    } else {
      this.yType.observe(observer);
      return () => this.yType.unobserve(observer);
    }
  };
}

class YPieceStore implements PieceStoreFull {
  public readonly parent: YDesignStore;
  public readonly yPiece: YPiece = new Y.Map<any>();

  constructor(parent: YDesignStore, piece: Piece) {
    this.parent = parent;
    this.yPiece.set("id_", piece.id_);
    this.yPiece.set("description", piece.description || "");
    const yType = new Y.Map<string>();
    yType.set("name", piece.type.name);
    if (piece.type.variant) yType.set("variant", piece.type.variant);
    this.yPiece.set("type", yType);
    if (piece.plane) {
      const yPlane = new Y.Map<any>();
      const yOrigin = new Y.Map<number>();
      yOrigin.set("x", piece.plane.origin.x);
      yOrigin.set("y", piece.plane.origin.y);
      yOrigin.set("z", piece.plane.origin.z);
      yPlane.set("origin", yOrigin);
      const yXAxis = new Y.Map<number>();
      yXAxis.set("x", piece.plane.xAxis.x);
      yXAxis.set("y", piece.plane.xAxis.y);
      yXAxis.set("z", piece.plane.xAxis.z);
      yPlane.set("xAxis", yXAxis);
      const yYAxis = new Y.Map<number>();
      yYAxis.set("x", piece.plane.yAxis.x);
      yYAxis.set("y", piece.plane.yAxis.y);
      yYAxis.set("z", piece.plane.yAxis.z);
      yPlane.set("yAxis", yYAxis);
      this.yPiece.set("plane", yPlane);
    }
    if (piece.center) {
      const yCenter = new Y.Map<number>();
      yCenter.set("x", piece.center.x);
      yCenter.set("y", piece.center.y);
      this.yPiece.set("center", yCenter);
    }
    const yAttributes = new Y.Array<YAttribute>();
    this.yPiece.set("attributes", yAttributes);
    (piece.attributes || []).forEach((q) => yAttributes.push([createAttribute(q)]));
  }

  snapshot = (): Piece => {
    const yType = this.yPiece.get("type") as Y.Map<string>;
    const yPlane = this.yPiece.get("plane") as Y.Map<any> | undefined;
    const yCenter = this.yPiece.get("center") as Y.Map<number> | undefined;

    const piece: Piece = {
      id_: this.yPiece.get("id_") as string,
      description: (this.yPiece.get("description") as string) || "",
      type: {
        name: yType.get("name") as string,
        variant: yType.get("variant") as string | undefined,
      },
      attributes: getAttributes(this.yPiece.get("attributes") as YAttributes),
    };
    if (yPlane) {
      const yOrigin = yPlane.get("origin") as Y.Map<number>;
      const yXAxis = yPlane.get("xAxis") as Y.Map<number>;
      const yYAxis = yPlane.get("yAxis") as Y.Map<number>;
      piece.plane = {
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
      piece.center = {
        x: yCenter.get("x") as number,
        y: yCenter.get("y") as number,
      };
    }
    return piece;
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
          const yOrigin = yPlane.get("origin") as Y.Map<number>;
          if (diff.plane.origin.x !== undefined) yOrigin.set("x", diff.plane.origin.x);
          if (diff.plane.origin.y !== undefined) yOrigin.set("y", diff.plane.origin.y);
          if (diff.plane.origin.z !== undefined) yOrigin.set("z", diff.plane.origin.z);
        }
        if (diff.plane.xAxis !== undefined) {
          const yXAxis = yPlane.get("xAxis") as Y.Map<number>;
          if (diff.plane.xAxis.x !== undefined) yXAxis.set("x", diff.plane.xAxis.x);
          if (diff.plane.xAxis.y !== undefined) yXAxis.set("y", diff.plane.xAxis.y);
          if (diff.plane.xAxis.z !== undefined) yXAxis.set("z", diff.plane.xAxis.z);
        }
        if (diff.plane.yAxis !== undefined) {
          const yYAxis = yPlane.get("yAxis") as Y.Map<number>;
          if (diff.plane.yAxis.x !== undefined) yYAxis.set("x", diff.plane.yAxis.x);
          if (diff.plane.yAxis.y !== undefined) yYAxis.set("y", diff.plane.yAxis.y);
          if (diff.plane.yAxis.z !== undefined) yYAxis.set("z", diff.plane.yAxis.z);
        }
      }
    }
    if (diff.center !== undefined) {
      const yCenter = this.yPiece.get("center") as Y.Map<number>;
      if (yCenter) {
        if (diff.center.x !== undefined) yCenter.set("x", diff.center.x);
        if (diff.center.y !== undefined) yCenter.set("y", diff.center.y);
      }
    }
    if (diff.attributes !== undefined) {
      const yAttributes = this.yPiece.get("attributes") as YAttributes;
      yAttributes.delete(0, yAttributes.length);
      diff.attributes.forEach((q) => yAttributes.push([createAttribute(q)]));
    }
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yPiece.observeDeep(observer);
      return () => this.yPiece.unobserveDeep(observer);
    } else {
      this.yPiece.observe(observer);
      return () => this.yPiece.unobserve(observer);
    }
  };
}

class YConnectionStore implements ConnectionStoreFull {
  public readonly parent: YDesignStore;
  public readonly yConnection: YConnection = new Y.Map<any>();

  constructor(parent: YDesignStore, connection: Connection) {
    this.parent = parent;
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
    const yAttributes = new Y.Array<YAttribute>();
    this.yConnection.set("attributes", yAttributes);
    (connection.attributes || []).forEach((q) => yAttributes.push([createAttribute(q)]));
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

    return {
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
      attributes: getAttributes(this.yConnection.get("attributes") as YAttributes),
    };
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

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yConnection.observeDeep(observer);
      return () => this.yConnection.unobserveDeep(observer);
    } else {
      this.yConnection.observe(observer);
      return () => this.yConnection.unobserve(observer);
    }
  };
}

class YDesignStore implements DesignStoreFull {
  public readonly parent: YKitStore;
  public readonly yDesign: YDesign = new Y.Map<any>();
  public readonly pieces: Map<PieceId, YPieceStore> = new Map();
  public readonly connections: Map<ConnectionId, YConnectionStore> = new Map();
  private readonly pieceIds: Map<PieceId, string> = new Map();
  private readonly connectionIds: Map<ConnectionId, string> = new Map();

  constructor(parent: YKitStore, design: Design) {
    this.parent = parent;
    this.yDesign.set("name", design.name);
    this.yDesign.set("description", design.description || "");
    this.yDesign.set("variant", design.variant || "");
    this.yDesign.set("view", design.view || "");
    this.yDesign.set("unit", design.unit);
    this.yDesign.set("pieces", new Y.Map() as YPieceMap);
    this.yDesign.set("connections", new Y.Map() as YConnectionMap);
    this.yDesign.set("authors", createAuthors(design.authors));
    this.yDesign.set("attributes", createAttributes(design.attributes));
    (design.pieces || []).forEach((piece) => {
      const pieceId = pieceIdLikeToPieceId(piece);
      const uuid = uuidv4();
      const yPieceStore = new YPieceStore(this, piece);
      (this.yDesign.get("pieces") as YPieceMap).set(uuid, yPieceStore.yPiece);
      this.pieceIds.set(pieceId, uuid);
      this.pieces.set(pieceId, yPieceStore);
    });
    (design.connections || []).forEach((connection) => {
      const connectionId = connectionIdLikeToConnectionId(connection);
      const uuid = uuidv4();
      const yConnectionStore = new YConnectionStore(this, connection);
      (this.yDesign.get("connections") as YConnectionMap).set(uuid, yConnectionStore.yConnection);
      this.connectionIds.set(connectionId, uuid);
      this.connections.set(connectionId, yConnectionStore);
    });
  }

  snapshot = (): Design => {
    return {
      name: this.yDesign.get("name") as string,
      description: this.yDesign.get("description") as string | undefined,
      variant: this.yDesign.get("variant") as string | undefined,
      view: this.yDesign.get("view") as string | undefined,
      unit: this.yDesign.get("unit") as string,
      pieces: Array.from(this.pieces.values()).map((p) => p.snapshot()),
      connections: Array.from(this.connections.values()).map((c) => c.snapshot()),
      authors: getAuthors(this.yDesign.get("authors") as YAuthors),
      attributes: getAttributes(this.yDesign.get("attributes") as YAttributes),
    };
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
          if (!this.pieceIds.get(pieceId)) {
            const uuid = uuidv4();
            const yPieceStore = new YPieceStore(this, piece);
            (this.yDesign.get("pieces") as YPieceMap).set(uuid, yPieceStore.yPiece);
            this.pieceIds.set(pieceId, uuid);
            this.pieces.set(pieceId, yPieceStore);
          }
        });
      }
      if (diff.pieces.removed) {
        diff.pieces.removed.forEach((pieceId) => {
          const id = pieceIdLikeToPieceId(pieceId);
          const uuid = this.pieceIds.get(id);
          if (uuid) {
            (this.yDesign.get("pieces") as YPieceMap).delete(uuid);
            this.pieceIds.delete(id);
            this.pieces.delete(id);
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
          if (!this.connectionIds.get(connectionId)) {
            const uuid = uuidv4();
            const yConnectionStore = new YConnectionStore(this, connection);
            (this.yDesign.get("connections") as YConnectionMap).set(uuid, yConnectionStore.yConnection);
            this.connectionIds.set(connectionId, uuid);
            this.connections.set(connectionId, yConnectionStore);
          }
        });
      }
      if (diff.connections.removed) {
        diff.connections.removed.forEach((connectionId) => {
          const id = connectionIdLikeToConnectionId(connectionId);
          const uuid = this.connectionIds.get(id);
          if (uuid) {
            (this.yDesign.get("connections") as YConnectionMap).delete(uuid);
            this.connectionIds.delete(id);
            this.connections.delete(id);
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

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    if (deep) {
      this.yDesign.observeDeep(observer);
      return () => this.yDesign.unobserveDeep(observer);
    } else {
      this.yDesign.observe(observer);
      return () => this.yDesign.unobserve(observer);
    }
  };
}

class YKitStore implements KitStoreFull {
  public readonly yKit: YKit = new Y.Map() as YKit;
  public readonly types: Map<TypeId, YTypeStore> = new Map();
  public readonly designs: Map<DesignId, YDesignStore> = new Map();
  public readonly files: Map<Url, YFileStore> = new Map();
  private readonly typeIds: Map<TypeId, string> = new Map();
  private readonly designIds: Map<DesignId, string> = new Map();
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult> = new Map();
  private readonly parent: SketchpadStore;

  constructor(parent: SketchpadStore, kit: Kit) {
    this.parent = parent;
    this.yKit.set("name", kit.name);
    this.yKit.set("version", kit.version || "");
    this.yKit.set("description", kit.description || "");
    this.yKit.set("icon", kit.icon || "");
    this.yKit.set("image", kit.image || "");
    this.yKit.set("preview", kit.preview || "");
    this.yKit.set("remote", kit.remote || "");
    this.yKit.set("homepage", kit.homepage || "");
    this.yKit.set("license", kit.license || "");
    this.yKit.set("created", new Date().toISOString());
    this.yKit.set("updated", new Date().toISOString());
    this.yKit.set("types", new Y.Map<YType>());
    this.yKit.set("designs", new Y.Map<YDesign>());
    this.yKit.set("attributes", new Y.Array<YAttribute>());
    (kit.types || []).forEach((type) => {
      const typeId = typeIdLikeToTypeId(type);
      const uuid = uuidv4();
      const yTypeStore = new YTypeStore(this, type);
      (this.yKit.get("types") as YTypeMap).set(uuid, yTypeStore.yType);
      this.typeIds.set(typeId, uuid);
      this.types.set(typeId, yTypeStore);
    });
    (kit.designs || []).forEach((design) => {
      const designId = designIdLikeToDesignId(design);
      const uuid = uuidv4();
      const yDesignStore = new YDesignStore(this, design);
      (this.yKit.get("designs") as YDesignMap).set(uuid, yDesignStore.yDesign);
      this.designIds.set(designId, uuid);
      this.designs.set(designId, yDesignStore);
    });

    Object.entries(kitCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  snapshot = (): Kit => {
    return {
      name: this.yKit.get("name") as string,
      version: this.yKit.get("version") as string | undefined,
      description: this.yKit.get("description") as string | undefined,
      icon: this.yKit.get("icon") as string | undefined,
      image: this.yKit.get("image") as string | undefined,
      preview: this.yKit.get("preview") as string | undefined,
      remote: this.yKit.get("remote") as string | undefined,
      homepage: this.yKit.get("homepage") as string | undefined,
      license: this.yKit.get("license") as string | undefined,
      created: this.yKit.get("created") as Date | undefined,
      updated: this.yKit.get("updated") as Date | undefined,
      types: Array.from(this.types.values()).map((store) => store.snapshot()),
      designs: Array.from(this.designs.values()).map((store) => store.snapshot()),
      attributes: getAttributes(this.yKit.get("attributes") as YAttributes),
    };
  };

  change = (diff: KitDiff) => {
    if (diff.name) this.yKit.set("name", diff.name);
    if (diff.description) this.yKit.set("description", diff.description);
    if (diff.version) this.yKit.set("version", diff.version);
    if (diff.icon) this.yKit.set("icon", diff.icon);
    if (diff.image) this.yKit.set("image", diff.image);
    if (diff.preview) this.yKit.set("preview", diff.preview);
    if (diff.remote) this.yKit.set("remote", diff.remote);
    if (diff.homepage) this.yKit.set("homepage", diff.homepage);
    if (diff.license) this.yKit.set("license", diff.license);

    if (diff.types) {
      if (diff.types.added) {
        diff.types.added.forEach((type) => {
          const typeId = typeIdLikeToTypeId(type);
          if (!this.typeIds.get(typeId)) {
            const uuid = uuidv4();
            const yTypeStore = new YTypeStore(this, type);
            (this.yKit.get("types") as YTypeMap).set(uuid, yTypeStore.yType);
            this.typeIds.set(typeId, uuid);
            this.types.set(typeId, yTypeStore);
          }
        });
      }
      if (diff.types.removed) {
        diff.types.removed.forEach((typeId) => {
          const id = typeIdLikeToTypeId(typeId);
          const uuid = this.typeIds.get(id);
          if (uuid) {
            (this.yKit.get("types") as YTypeMap).delete(uuid);
            this.typeIds.delete(id);
            this.types.delete(id);
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
    }

    if (diff.designs) {
      if (diff.designs.added) {
        diff.designs.added.forEach((design) => {
          const designId = designIdLikeToDesignId(design);
          if (!this.designIds.get(designId)) {
            const uuid = uuidv4();
            const yDesignStore = new YDesignStore(this, design);
            (this.yKit.get("designs") as YDesignMap).set(uuid, yDesignStore.yDesign);
            this.designIds.set(designId, uuid);
            this.designs.set(designId, yDesignStore);
          }
        });
      }
      if (diff.designs.removed) {
        diff.designs.removed.forEach((designId) => {
          const id = designIdLikeToDesignId(designId);
          const uuid = this.designIds.get(id);
          if (uuid) {
            (this.yKit.get("designs") as YDesignMap).delete(uuid);
            this.designIds.delete(id);
            this.designs.delete(id);
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
    }

    this.yKit.set("updated", new Date().toISOString());
  };

  fileUrls = (): Map<Url, Url> => {
    return new Map();
  };

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    deep ? (this.yKit as unknown as Y.Map<any>).observeDeep(observer) : (this.yKit as unknown as Y.Map<any>).observe(observer);
    return () => (deep ? (this.yKit as unknown as Y.Map<any>).unobserveDeep(observer) : (this.yKit as unknown as Y.Map<any>).unobserve(observer));
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit store`);
    const result = callback(this, ...rest);
    if (result.diff) {
      this.change(result.diff);
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
  public readonly yDesignEditorStore: YDesignEditorStoreValMap = new Y.Map<YDesignEditorStoreVal>();
  private readonly commandRegistry: Map<string, (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult> = new Map();
  private readonly parent: SketchpadStore;

  constructor(parent: SketchpadStore, state: DesignEditorStateFull) {
    this.parent = parent;
    this.yDesignEditorStore.set("fullscreenPanel", state.fullscreenPanel);
    this.yDesignEditorStore.set("selectedPieceIds", new Y.Array<string>());
    this.yDesignEditorStore.set("selectedConnections", new Y.Array<string>());
    this.yDesignEditorStore.set("isTransactionActive", state.isTransactionActive);
    this.yDesignEditorStore.set("presenceCursorX", state.presence.cursor?.x || 0);
    this.yDesignEditorStore.set("presenceCursorY", state.presence.cursor?.y || 0);

    Object.entries(designEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenPanel(): DesignEditorFullscreenPanel {
    return (this.yDesignEditorStore.get("fullscreenPanel") as DesignEditorFullscreenPanel) || DesignEditorFullscreenPanel.None;
  }
  get selection(): DesignEditorSelection {
    const selectedPieceIds = this.yDesignEditorStore.get("selectedPieceIds") as Y.Array<string>;
    const selectedConnections = this.yDesignEditorStore.get("selectedConnections") as Y.Array<string>;
    return {
      pieceIds: selectedPieceIds ? selectedPieceIds.toArray().map((id) => ({ id_: id })) : [],
      connectionIds: selectedConnections
        ? selectedConnections.toArray().map((id) => ({
            connected: { piece: { id_: id.split("->")[0] || "" } },
            connecting: { piece: { id_: id.split("->")[1] || "" } },
          }))
        : [],
    };
  }
  get designDiff(): DesignDiff {
    return {};
  }
  get isTransactionActive(): boolean {
    return (this.yDesignEditorStore.get("isTransactionActive") as boolean) || false;
  }
  get presence(): DesignEditorPresence {
    return {
      cursor: {
        x: (this.yDesignEditorStore.get("presenceCursorX") as number) || 0,
        y: (this.yDesignEditorStore.get("presenceCursorY") as number) || 0,
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
    return [];
  }
  get pastTransactionsStack(): DesignEditorEdit[] {
    return [];
  }

  snapshot = (): DesignEditorStateFull => {
    return {
      fullscreenPanel: this.fullscreenPanel,
      selection: this.selection,
      isTransactionActive: this.isTransactionActive,
      presence: this.presence,
      others: this.others,
      diff: this.diff,
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
    };
  };

  change = (diff: DesignEditorStateDiff) => {
    if (diff.fullscreenPanel) this.yDesignEditorStore.set("fullscreenPanel", diff.fullscreenPanel);
    if (diff.selection) {
      if (diff.selection.pieceIds) {
        const yPieceIds = this.yDesignEditorStore.get("selectedPieceIds") as Y.Array<string>;
        yPieceIds.delete(0, yPieceIds.length);
        const pieceIdStrings = diff.selection.pieceIds.map((piece) => piece.id_);
        yPieceIds.insert(0, pieceIdStrings);
      }
      if (diff.selection.connectionIds) {
        const yConnectionIds = this.yDesignEditorStore.get("selectedConnections") as Y.Array<string>;
        yConnectionIds.delete(0, yConnectionIds.length);
        const connectionIdStrings = diff.selection.connectionIds.map((conn) => `${conn.connected.piece.id_}->${conn.connecting.piece.id_}`);
        yConnectionIds.insert(0, connectionIdStrings);
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

  changed = (subscribe: Subscribe, deep?: boolean) => {
    const observer = () => subscribe();
    deep ? (this.yDesignEditorStore as unknown as Y.Map<any>).observeDeep(observer) : (this.yDesignEditorStore as unknown as Y.Map<any>).observe(observer);
    return () => (deep ? (this.yDesignEditorStore as unknown as Y.Map<any>).unobserveDeep(observer) : (this.yDesignEditorStore as unknown as Y.Map<any>).unobserve(observer));
  };

  transaction = {
    start: () => {
      this.yDesignEditorStore.set("isTransactionActive", true);
    },
    started: (subscribe: Subscribe) => {
      const observer = () => subscribe();
      this.yDesignEditorStore.observe(observer);
      return () => this.yDesignEditorStore.unobserve(observer);
    },
    abort: () => {
      this.yDesignEditorStore.set("isTransactionActive", false);
    },
    aborted: (subscribe: Subscribe) => {
      const observer = () => subscribe();
      this.yDesignEditorStore.observe(observer);
      return () => this.yDesignEditorStore.unobserve(observer);
    },
    finalize: () => {
      if (this.isTransactionActive) {
        this.yDesignEditorStore.set("isTransactionActive", false);
      }
    },
    finalized: (subscribe: Subscribe) => {
      const observer = () => subscribe();
      this.yDesignEditorStore.observe(observer);
      return () => this.yDesignEditorStore.unobserve(observer);
    },
  };

  undo = () => {};
  undone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yDesignEditorStore.observe(observer);
    return () => this.yDesignEditorStore.unobserve(observer);
  };
  redo = () => {};
  redone = (subscribe: Subscribe) => {
    const observer = () => subscribe();
    this.yDesignEditorStore.observe(observer);
    return () => this.yDesignEditorStore.unobserve(observer);
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design editor store`);
    const parent = this.parent as YSketchpadStore;
    const context: DesignEditorCommandContext = {
      sketchpadState: parent.snapshot(),
      state: this.snapshot(),
      fileUrls: new Map(),
      kit: parent.kits.get(parent.activeDesignEditor!.kitId)!.snapshot(),
      designId: parent.activeDesignEditor!.designId,
    };
    const result = callback(context, ...rest);
    // Apply state and diff changes
    // This would need proper implementation
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
      undone: this.undone,
      redone: this.redone,
      changed: this.changed,
      transaction: {
        started: this.transaction.started,
        aborted: this.transaction.aborted,
        finalized: this.transaction.finalized,
      },
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
  public readonly yKitDocs: Map<KitId, Y.Doc> = new Map();
  public readonly kitIndexeddbProviders: Map<KitId, IndexeddbPersistence> = new Map();
  public readonly kits: Map<KitId, YKitStore> = new Map();
  public readonly designEditors: Map<KitId, Map<DesignId, DesignEditorStoreFull>> = new Map();
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult> = new Map();

  private getYSketchpad(): YSketchpad {
    return this.ySketchpadDoc.getMap("sketchpad");
  }

  constructor(state: SketchpadStateFull) {
    this.ySketchpadDoc = new Y.Doc();
    if (state.persistantId && state.persistantId !== "") {
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad-${state.persistantId}`, this.ySketchpadDoc);
    }
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("mode", state.mode);
    ySketchpad.set("theme", state.theme);
    ySketchpad.set("layout", state.layout);
    if (state.activeDesignEditor) {
      ySketchpad.set("activeDesignEditor", JSON.stringify(state.activeDesignEditor));
    }
  }

  get mode(): Mode {
    return (this.getYSketchpad().get("mode") as Mode) || Mode.GUEST;
  }
  get theme(): Theme {
    return (this.getYSketchpad().get("theme") as Theme) || Theme.SYSTEM;
  }
  get layout(): Layout {
    return (this.getYSketchpad().get("layout") as Layout) || Layout.NORMAL;
  }
  get activeDesignEditor(): DesignEditorId | undefined {
    const designEditorIdStr = this.getYSketchpad().get("activeDesignEditor");
    if (!designEditorIdStr || typeof designEditorIdStr !== "string") return undefined;
    return JSON.parse(designEditorIdStr) as DesignEditorId;
  }

  snapshot = (): SketchpadStateFull => {
    return {
      mode: this.mode,
      theme: this.theme,
      layout: this.layout,
      activeDesignEditor: this.activeDesignEditor,
    };
  };

  create = {
    kit: (kit: Kit) => {
      const kitId = kitIdLikeToKitId(kit);
      if (this.kits.has(kitId)) throw new Error(`Kit ${kitId} already exists`);
      const yKitStore = new YKitStore(this, kit);
      const yKitDoc = new Y.Doc();
      yKitDoc.getMap("kit").set("data", yKitStore.yKit);
      this.yKitDocs.set(kitId, yKitDoc);
      this.kits.set(kitId, yKitStore);
    },
    designEditor: (id: DesignEditorId) => {
      const initialState: DesignEditorStateFull = {
        fullscreenPanel: DesignEditorFullscreenPanel.None,
        selection: {},
        isTransactionActive: false,
        presence: {},
        others: [],
        diff: {},
        currentTransactionStack: [],
        pastTransactionsStack: [],
      };
      const editorStore = new YDesignEditorStore(this, initialState);

      // Ensure the kitId map exists
      if (!this.designEditors.has(id.kitId)) {
        this.designEditors.set(id.kitId, new Map());
      }

      const kitEditors = this.designEditors.get(id.kitId)!;
      kitEditors.set(id.designId, editorStore);
    },
  };

  change(diff: SketchpadStateDiff) {
    if (diff.mode) this.getYSketchpad().set("mode", diff.mode);
    if (diff.theme) this.getYSketchpad().set("theme", diff.theme);
    if (diff.layout) this.getYSketchpad().set("layout", diff.layout);
    if (diff.activeDesignEditor) this.getYSketchpad().set("activeDesignEditor", JSON.stringify(diff.activeDesignEditor));
  }

  delete = {
    kit: (id: KitIdLike) => {
      const kitId = kitIdLikeToKitId(id);
      if (this.kits.has(kitId)) {
        this.kits.delete(kitId);
      }
      if (this.designEditors.has(kitId)) {
        this.designEditors.delete(kitId);
      }
    },
    designEditor: (id: DesignEditorId) => {
      const kitEditors = this.designEditors.get(id.kitId);
      if (kitEditors) {
        kitEditors.delete(id.designId);
        if (kitEditors.size === 0) {
          this.designEditors.delete(id.kitId);
        }
      }
    },
  };

  // Subscriptions using Yjs observers
  created = {
    kit: (subscribe: Subscribe): Unsubscribe => {
      // For kit creation, we can observe all kit documents being added
      // This is a simplified approach - would need more sophisticated tracking in full implementation
      const observer = () => subscribe();
      // Note: Would need to observe kit document creation across all kit docs
      return () => {}; // Placeholder unsubscribe
    },
    designEditor: (subscribe: Subscribe): Unsubscribe => {
      // Observe design editor stores map changes
      const yDesignEditorStores = this.ySketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
      const observer = () => subscribe();
      yDesignEditorStores.observe(observer);
      return () => yDesignEditorStores.unobserve(observer);
    },
  };

  deleted = {
    kit: (subscribe: Subscribe): Unsubscribe => {
      // Would need to observe kit document removal
      return () => {}; // Placeholder
    },
    designEditor: (subscribe: Subscribe): Unsubscribe => {
      // Would need to observe design editor removal
      return () => {}; // Placeholder
    },
  };

  changed = (subscribe: Subscribe): Unsubscribe => {
    const observer = () => subscribe();
    this.getYSketchpad().observe(observer);
    return () => this.getYSketchpad().unobserve(observer);
  };

  get on(): SketchpadSubscriptions {
    return {
      created: this.created,
      changed: this.changed,
      deleted: this.deleted,
    };
  }

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in sketchpad store`);
    const context: SketchpadCommandContext = {
      state: this.snapshot(),
      kits: Array.from(this.kits.values()).map((k) => k.snapshot()),
      fileUrls: new Map(),
    };
    const result = callback(context, ...rest);
    // Apply diff changes
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
};

const kitCommands = {
  "semio.kit.addType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
};

const designEditorCommands = {
  "semio.designEditor.selectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const design = findDesignInKit(context.kit, context.designId)!;
    return {
      diff: {
        selection: { pieceIds: design.pieces ?? [], connectionIds: design.connections ?? [] },
      },
    };
  },
  "semio.designEditor.deselectAll": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    return {
      diff: {
        selection: { pieceIds: [], connectionIds: [] },
      },
    };
  },
  "semio.designEditor.deleteSelected": (context: DesignEditorCommandContext): DesignEditorCommandResult => {
    const selection = context.state.selection;
    return {
      diff: {
        selection: { pieceIds: [], connectionIds: [] },
      },
      kitDiff: {
        designs: {
          updated: [
            {
              id: context.designId,
              diff: { pieces: { removed: selection.pieceIds }, connections: { removed: selection.connectionIds } },
            },
          ],
        },
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

function useSketchpadStore(id?: string): SketchpadStore {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  const state = useSyncExternalStore(
    store.changed,
    () => store.snapshot(),
    () => store.snapshot(),
  );
  return store;
}

export function useSketchpad(): SketchpadStore;
export function useSketchpad<T>(selector: (store: SketchpadStore) => T, id?: string): T;
export function useSketchpad<T>(selector?: (store: SketchpadStore) => T, id?: string): T | SketchpadStore {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;

  const state = useSyncExternalStore(
    store.changed,
    () => store.snapshot(),
    () => store.snapshot(),
  );

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

  if (!store.designEditors.has(resolvedKitId)) throw new Error(`Design editor store not found for kit ${resolvedKitId.name}`);
  const kitEditors = store.designEditors.get(resolvedKitId)!;
  if (!kitEditors.has(resolvedDesignId)) throw new Error(`Design editor store not found for design ${resolvedDesignId.name}`);
  const designEditor = kitEditors.get(resolvedDesignId)!;

  const state = useSyncExternalStore(
    designEditor.changed,
    () => designEditor.snapshot(),
    () => designEditor.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  const state = useSyncExternalStore(
    kitStore.changed,
    () => kitStore.snapshot(),
    () => kitStore.snapshot(),
  );
  return selector ? selector(kitStore) : kitStore;
}

export function useKit(): Kit;
export function useKit<T>(selector: (kit: Kit) => T): T;
export function useKit<T>(selector: (kit: Kit) => T, id: KitId): T;
export function useKit<T>(selector?: (kit: Kit) => T, id?: KitId): T | Kit {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKitStore must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitStoreScope();
  const kitId = kitScope?.id ?? id;
  if (!kitId) throw new Error("useKitStore must be called within a KitScopeProvider or be directly provided with an id");
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  const kit = useSyncExternalStore(
    kitStore.changed,
    () => kitStore.snapshot(),
    () => kitStore.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  const designId = designScope?.id ?? id;
  if (!designId) throw new Error("useDesign must be called within a DesignScopeProvider or be directly provided with an id");
  if (!kitStore.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kitStore.designs.get(designId)!;
  const state = useSyncExternalStore(
    designStore.changed,
    () => designStore.snapshot(),
    () => designStore.snapshot(),
  );
  return selector ? selector(state) : state;
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  const typeId = typeScope?.id ?? id;
  if (!typeId) throw new Error("useType must be called within a TypeScopeProvider or be directly provided with an id");
  if (!kit.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kit.types.get(typeId)!;
  const state = useSyncExternalStore(
    typeStore.changed,
    () => typeStore.snapshot(),
    () => typeStore.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePiece must be called within a DesignScopeProvider");
  const designId = designScope.id;
  if (!kit.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const design = kit.designs.get(designId)!;
  const pieceScope = usePieceScope();
  const pieceId = pieceScope?.id ?? id;
  if (!pieceId) throw new Error("usePiece must be called within a PieceScopeProvider or be directly provided with an id");
  if (!design.pieces.has(pieceId)) throw new Error(`Piece store not found for piece ${pieceId}`);
  const pieceStore = design.pieces.get(pieceId)!;
  const state = useSyncExternalStore(
    pieceStore.changed,
    () => pieceStore.snapshot(),
    () => pieceStore.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useConnection must be called within a DesignScopeProvider");
  const designId = designScope.id;
  if (!kit.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const design = kit.designs.get(designId)!;
  const connectionScope = useConnectionScope();
  const connectionId = connectionScope?.id ?? id;
  if (!connectionId) throw new Error("useConnection must be called within a ConnectionScopeProvider or be directly provided with an id");
  if (!design.connections.has(connectionId)) throw new Error(`Connection store not found for connection ${connectionId}`);
  const connectionStore = design.connections.get(connectionId)!;
  const state = useSyncExternalStore(
    connectionStore.changed,
    () => connectionStore.snapshot(),
    () => connectionStore.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("usePort must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  if (!kit.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const type = kit.types.get(typeId)!;
  const portScope = usePortScope();
  const portId = portScope?.id ?? id;
  if (!portId) throw new Error("usePort must be called within a PortScopeProvider or be directly provided with an id");
  if (!type.ports.has(portId)) throw new Error(`Port store not found for port ${portId}`);
  const portStore = type.ports.get(portId)!;
  const state = useSyncExternalStore(
    portStore.changed,
    () => portStore.snapshot(),
    () => portStore.snapshot(),
  );
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
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("useRepresentation must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  if (!kit.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kit.types.get(typeId)!;
  const representationScope = useRepresentationScope();
  const representationId = representationScope?.id ?? id;
  if (!representationId) throw new Error("useRepresentation must be called within a RepresentationScopeProvider or be directly provided with an id");
  if (!typeStore.representations.has(representationId)) throw new Error(`Representation store not found for representation ${representationId}`);
  const representationStore = typeStore.representations.get(representationId)!;
  const state = useSyncExternalStore(
    representationStore.changed,
    () => representationStore.snapshot(),
    () => representationStore.snapshot(),
  );
  return selector ? selector(state) : state;
}
// #endregion Hooks
