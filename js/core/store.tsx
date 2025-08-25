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

import React, { createContext, useContext, useEffect, useMemo, useState } from "react";
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
  Kit,
  KitDiff,
  KitId,
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

export interface YStore {}

export interface FileStore {}

export interface RepresentationState {
  representation: Representation;
}

export interface RepresentationActions {
  create: {};
  update: {
    representation: (diff: RepresentationDiff) => void;
  };
  delete: {};
}

export interface RepresentationSubscriptions {
  on: {
    created: {};
    updated: {
      representation: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {};
  };
}

export interface RepresentationChildStores {}

export interface RepresentationChildStoresFull {}

export interface RepresentationStore extends RepresentationState, RepresentationChildStores {}

export interface RepresentationStoreFull extends RepresentationState, RepresentationActions, RepresentationSubscriptions, RepresentationChildStoresFull {}

export interface PortState {
  port: Port;
}
export interface PortActions {
  create: {};
  update: {
    port: (diff: PortDiff) => void;
  };
  delete: {};
}

export interface PortSubscriptions {
  on: {
    created: {};
    updated: {
      port: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {};
  };
}

export interface PortChildStores {}

export interface PortChildStoresFull {}

export interface PortStore extends PortState, PortChildStores {}

export interface PortStoreFull extends PortState, PortActions, PortSubscriptions, PortChildStoresFull {}

export interface TypeState {
  type: Type;
}
export interface TypeActions {
  create: {
    representation: (representation: Representation) => void;
    representations: (representations: Representation[]) => void;
    port: (port: Port) => void;
    ports: (ports: Port[]) => void;
  };
  update: {
    type: (diff: TypeDiff) => void;
  };
  delete: {
    representation: (id: RepresentationId) => void;
    representations: (ids: RepresentationId[]) => void;
    port: (id: PortId) => void;
    ports: (ids: PortId[]) => void;
  };
}

export interface TypeSubscriptions {
  on: {
    created: {
      representation: (subscribe: Subscribe) => Unsubscribe;
      representations: (subscribe: Subscribe) => Unsubscribe;
      port: (subscribe: Subscribe) => Unsubscribe;
      ports: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      type: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      representation: (subscribe: Subscribe) => Unsubscribe;
      representations: (subscribe: Subscribe) => Unsubscribe;
      port: (subscribe: Subscribe) => Unsubscribe;
      ports: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

export interface TypeChildStores {
  representations: Map<RepresentationId, RepresentationStore>;
  ports: Map<PortId, PortStore>;
}

export interface TypeChildStoresFull {
  representations: Map<RepresentationId, RepresentationStoreFull>;
  ports: Map<PortId, PortStoreFull>;
}

export interface TypeStore extends TypeState, TypeChildStores {}

export interface TypeStoreFull extends TypeState, TypeActions, TypeSubscriptions, TypeChildStoresFull {}

export interface PieceState {
  piece: Piece;
}

export interface PieceActions {
  create: {};
  update: {
    piece: (diff: PieceDiff) => void;
  };
  delete: {};
}

export interface PieceSubscriptions {
  on: {
    created: {};
    updated: {
      piece: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {};
  };
}

export interface PieceChildStores {}

export interface PieceChildStoresFull {}

export interface PieceStore extends PieceState, PieceChildStores {}

export interface PieceStoreFull extends PieceState, PieceActions, PieceSubscriptions, PieceChildStoresFull {}

export interface ConnectionState {
  connection: Connection;
}

export interface ConnectionActions {
  create: {};
  update: {
    connection: (diff: ConnectionDiff) => void;
  };
  delete: {};
}

export interface ConnectionSubscriptions {
  on: {
    created: {};
    updated: {
      connection: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {};
  };
}

export interface ConnectionChildStores {}

export interface ConnectionChildStoresFull {}

export interface ConnectionStore extends ConnectionState, ConnectionChildStores {}

export interface ConnectionStoreFull extends ConnectionState, ConnectionActions, ConnectionSubscriptions, ConnectionChildStoresFull {}

export interface DesignState {
  design: Design;
}

export interface DesignActions {
  create: {
    piece: (piece: Piece) => void;
    pieces: (pieces: Piece[]) => void;
    connection: (connection: Connection) => void;
    connections: (connections: Connection[]) => void;
  };
  update: {
    design: (diff: DesignDiff) => void;
  };
  delete: {
    piece: (id: PieceId) => void;
    pieces: (ids: PieceId[]) => void;
    connection: (id: ConnectionId) => void;
    connections: (ids: ConnectionId[]) => void;
  };
}

export interface DesignSubscriptions {
  on: {
    created: {
      piece: (subscribe: Subscribe) => Unsubscribe;
      pieces: (subscribe: Subscribe) => Unsubscribe;
      connection: (subscribe: Subscribe) => Unsubscribe;
      connections: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      design: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      piece: (subscribe: Subscribe) => Unsubscribe;
      pieces: (subscribe: Subscribe) => Unsubscribe;
      connection: (subscribe: Subscribe) => Unsubscribe;
      connections: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

export interface DesignChildStores {
  pieces: Map<PieceId, PieceStore>;
  connections: Map<ConnectionId, ConnectionStore>;
}

export interface DesignChildStoresFull {
  pieces: Map<PieceId, PieceStoreFull>;
  connections: Map<ConnectionId, ConnectionStoreFull>;
}

export interface DesignStore extends DesignState, DesignChildStores {}

export interface DesignStoreFull extends DesignState, DesignActions, DesignSubscriptions, DesignChildStoresFull {}

export interface KitState {
  kit: Kit;
}

export interface KitActions {
  create: {
    type: (type: Type) => void;
    types: (types: Type[]) => void;
    design: (design: Design) => void;
    designs: (designs: Design[]) => void;
    file: (file: File) => void;
    files: (files: File[]) => void;
  };
  update: {
    kit: (diff: KitDiff) => void;
  };
  delete: {
    type: (id: TypeId) => void;
    types: (ids: TypeId[]) => void;
    design: (id: DesignId) => void;
    designs: (ids: DesignId[]) => void;
    file: (url: Url) => void;
    files: (urls: Url[]) => void;
  };
}

export interface KitSubscriptions {
  on: {
    created: {
      type: (subscribe: Subscribe) => Unsubscribe;
      types: (subscribe: Subscribe) => Unsubscribe;
      design: (subscribe: Subscribe) => Unsubscribe;
      designs: (subscribe: Subscribe) => Unsubscribe;
      file: (subscribe: Subscribe) => Unsubscribe;
      files: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      kit: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      type: (subscribe: Subscribe) => Unsubscribe;
      types: (subscribe: Subscribe) => Unsubscribe;
      design: (subscribe: Subscribe) => Unsubscribe;
      designs: (subscribe: Subscribe) => Unsubscribe;
      file: (subscribe: Subscribe) => Unsubscribe;
      files: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
}

export interface KitCommandResult {
  diff?: KitDiff;
}

export interface KitCommands {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
  };
}

export interface KitCommandsFull {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
    register(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable;
  };
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

export interface KitStore extends KitState, KitChildStores, KitCommands {}

export interface KitStoreFull extends KitState, KitActions, KitSubscriptions, KitChildStoresFull, KitCommandsFull {}

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
  isTransactionActive: boolean;
  presence: DesignEditorPresence;
  others: DesignEditorPresenceOther[];
  diff: KitDiff;
}

export interface DesignEditorActions {
  undo: () => void;
  redo: () => void;
  set: {
    fullscreenPanel: (fullscreenPanel: DesignEditorFullscreenPanel) => void;
    selection: (selection: DesignEditorSelection) => void;
    presence: (presence: DesignEditorPresence) => void;
    others: (others: DesignEditorPresenceOther[]) => void;
  };
  transaction: {
    start: () => void;
    abort: () => void;
    finalize: () => void;
  };
}

export interface DesignEditorSubscriptions {
  on: {
    undone: (subscribe: Subscribe) => Unsubscribe;
    redone: (subscribe: Subscribe) => Unsubscribe;
    updated: {
      designEditor: (subscribe: Subscribe) => Unsubscribe;
    };
    set: {
      fullscreenPanel: (subscribe: Subscribe) => Unsubscribe;
      selection: (subscribe: Subscribe) => Unsubscribe;
      presence: (subscribe: Subscribe) => Unsubscribe;
      others: (subscribe: Subscribe) => Unsubscribe;
    };
    transaction: {
      started: (subscribe: Subscribe) => Unsubscribe;
      aborted: (subscribe: Subscribe) => Unsubscribe;
      finalized: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

export interface DesignEditorCommandContext {
  sketchpadState: SketchpadState;
  state: DesignEditorState;
  fileUrls: Map<Url, Url>;
  kit: Kit;
  designId: DesignId;
}

export interface DesignEditorCommandResult {
  state?: DesignEditorState;
  diff?: KitDiff;
}

export interface DesignEditorCommands {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
  };
}

export interface DesignEditorCommandsFull {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
    register(command: string, callback: (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult): Disposable;
  };
}

export interface DesignEditorStore extends DesignEditorState, DesignEditorCommands {}

export interface DesignEditorStoreFull extends DesignEditorState, DesignEditorActions, DesignEditorSubscriptions, DesignEditorCommandsFull {}

export interface SketchpadState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorDesignId?: DesignId;
  persistantId?: string;
}

export interface SketchpadActions {
  create: {
    kits: (kits: Kit[]) => void;
    designEditors: (designIds: DesignId[]) => void;
  };
  update: {
    kit: (kitId: KitId, diff: KitDiff) => void;
  };
  delete: {
    kits: (kitIds: KitId[]) => void;
    designEditors: (designIds: DesignId[]) => void;
  };
  set: {
    mode: (mode: Mode) => void;
    theme: (theme: Theme) => void;
    layout: (layout: Layout) => void;
    activeDesignEditorDesignId: (id?: string) => void;
  };
}

export interface SketchpadSubscriptions {
  on: {
    created: {
      kit: (subscribe: Subscribe) => Unsubscribe;
      designEditor: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      sketchpad: (subscribe: Subscribe) => Unsubscribe;
    };
    deleted: {
      kit: (subscribe: Subscribe) => Unsubscribe;
      designEditor: (subscribe: Subscribe) => Unsubscribe;
    };
    set: {
      mode: (subscribe: Subscribe) => Unsubscribe;
      theme: (subscribe: Subscribe) => Unsubscribe;
      layout: (subscribe: Subscribe) => Unsubscribe;
      activeDesignEditorDesignId: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

export interface SketchpadChildStores {
  kits: Map<KitId, KitStore>;
  designEditors: Map<DesignId, DesignEditorStore>;
}

export interface SketchpadChildStoresFull {
  kits: Map<KitId, KitStoreFull>;
  designEditors: Map<DesignId, DesignEditorStoreFull>;
}

export interface SketchpadCommandContext {
  state: SketchpadState;
  kits: Kit[];
  fileUrls: Map<Url, Url>;
}

export interface SketchpadCommandResult {
  state?: SketchpadState;
  diffs?: Map<KitId, KitDiff>;
}

export interface SketchpadCommands {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
  };
}

export interface SketchpadCommandsFull {
  commands: {
    execute<T>(command: string, ...rest: any[]): Promise<T>;
    register(command: string, callback: (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult): Disposable;
  };
}

export interface SketchpadStore extends SketchpadState, SketchpadChildStores, SketchpadCommands {}

export interface SketchpadStoreFull extends SketchpadState, SketchpadActions, SketchpadSubscriptions, SketchpadChildStoresFull, SketchpadCommandsFull {}

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
      name: yMap.get("name") as string,
      value: yMap.get("value") as string | undefined,
      unit: yMap.get("unit") as string | undefined,
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

class YRepresentationStore implements RepresentationStoreFull {
  public readonly yRepresentation: YRepresentation = new Y.Map<any>();

  constructor(representation: Representation) {
    this.yRepresentation.set("url", representation.url);
    this.yRepresentation.set("description", representation.description || "");
    const yTags = new Y.Array<string>();
    this.yRepresentation.set("tags", yTags);
    (representation.tags || []).forEach((t) => yTags.push([t]));
    this.yRepresentation.set("attributes", createAttributes(representation.attributes));
  }

  get representation(): Representation {
    const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
    return {
      url: this.yRepresentation.get("url") as string,
      description: (this.yRepresentation.get("description") as string) || "",
      tags: yTags ? yTags.toArray() : [],
      attributes: getAttributes(this.yRepresentation.get("attributes") as YAttributes),
    };
  }

  create = {};

  update = {
    representation: (diff: RepresentationDiff) => {
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
    },
  };

  delete = {};

  on = {
    created: {},
    updated: {
      representation: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yRepresentation.observeDeep(observer);
          return () => this.yRepresentation.unobserveDeep(observer);
        } else {
          this.yRepresentation.observe(observer);
          return () => this.yRepresentation.unobserve(observer);
        }
      },
    },
    deleted: {},
  };
}

class YPortStore implements PortStoreFull {
  public readonly yPort: YPort = new Y.Map<any>();

  constructor(port: Port) {
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

  get port(): Port {
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
  }

  create = {};

  update = {
    port: (diff: PortDiff) => {
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
    },
  };

  delete = {};

  on = {
    created: {},
    updated: {
      port: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yPort.observeDeep(observer);
          return () => this.yPort.unobserveDeep(observer);
        } else {
          this.yPort.observe(observer);
          return () => this.yPort.unobserve(observer);
        }
      },
    },
    deleted: {},
  };
}

class YTypeStore implements TypeStoreFull {
  public readonly yType: YType = new Y.Map<any>();
  public readonly representations: Map<RepresentationId, YRepresentationStore> = new Map();
  public readonly ports: Map<PortId, YPortStore> = new Map();
  private readonly representationIds: Map<RepresentationId, string> = new Map();
  private readonly portIds: Map<PortId, string> = new Map();

  constructor(type: Type) {
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

  get type(): Type {
    return {
      name: this.yType.get("name") as string,
      description: (this.yType.get("description") as string) || "",
      variant: this.yType.get("variant") as string | undefined,
      unit: (this.yType.get("unit") as string) || "",
      stock: this.yType.get("stock") as number | undefined,
      virtual: this.yType.get("virtual") as boolean | undefined,
      representations: Array.from(this.representations.values()).map((store) => store.representation),
      ports: Array.from(this.ports.values()).map((store) => store.port),
      authors: getAuthors(this.yType.get("authors") as YAuthors),
      attributes: getAttributes(this.yType.get("attributes") as YAttributes),
    };
  }

  create = {
    representation: (representation: Representation) => {
      const repId = representationIdLikeToRepresentationId(representation);
      if (this.representationIds.get(repId)) throw new Error(`Representation ${repId} already exists`);
      const uuid = uuidv4();
      this.representationIds.set(repId, uuid);
      this.representations.set(repId, new YRepresentationStore(representation));
    },
    representations: (representations: Representation[]) => representations.forEach((rep) => this.create.representation(rep)),
    port: (port: Port) => {
      const portId = portIdLikeToPortId(port);
      if (this.portIds.get(portId)) throw new Error(`Port ${portId} already exists`);
      const uuid = uuidv4();
      this.portIds.set(portId, uuid);
      this.ports.set(portId, new YPortStore(port));
    },
    ports: (ports: Port[]) => ports.forEach((port) => this.create.port(port)),
  };
  update = {
    type: (diff: TypeDiff) => {
      if (diff.name !== undefined) this.yType.set("name", diff.name);
      if (diff.description !== undefined) this.yType.set("description", diff.description);
      if (diff.variant !== undefined) this.yType.set("variant", diff.variant);
      if (diff.unit !== undefined) this.yType.set("unit", diff.unit);
      if (diff.stock !== undefined) this.yType.set("stock", diff.stock);
      if (diff.virtual !== undefined) this.yType.set("virtual", diff.virtual);
    },
  };
  delete = {
    representation: (id: RepresentationId) => {
      const repId = representationIdLikeToRepresentationId(id);
      const uuid = this.representationIds.get(repId);
      if (!uuid) throw new Error(`Representation ${repId} does not exist`);
      this.representationIds.delete(repId);
      this.representations.delete(repId);
    },
    representations: (ids: RepresentationId[]) => ids.forEach((id) => this.delete.representation(id)),
    port: (id: PortId) => {
      const portId = portIdLikeToPortId(id);
      const uuid = this.portIds.get(portId);
      if (!uuid) throw new Error(`Port ${portId} does not exist`);
      this.portIds.delete(portId);
      this.ports.delete(portId);
    },
    ports: (ids: PortId[]) => ids.forEach((id) => this.delete.port(id)),
  };
  on = {
    created: {
      representation: (subscribe: Subscribe) => {
        const yRepresentations = this.yType.get("representations") as YRepresentationMap;
        const observer = () => subscribe();
        yRepresentations.observe(observer);
        return () => yRepresentations.unobserve(observer);
      },
      representations: (subscribe: Subscribe) => () => {},
      port: (subscribe: Subscribe) => {
        const yPorts = this.yType.get("ports") as YPortMap;
        const observer = () => subscribe();
        yPorts.observe(observer);
        return () => yPorts.unobserve(observer);
      },
      ports: (subscribe: Subscribe) => () => {},
    },
    updated: {
      type: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yType.observeDeep(observer);
          return () => this.yType.unobserveDeep(observer);
        } else {
          this.yType.observe(observer);
          return () => this.yType.unobserve(observer);
        }
      },
    },
    deleted: {
      representation: (subscribe: Subscribe) => {
        const yRepresentations = this.yType.get("representations") as YRepresentationMap;
        const observer = () => subscribe();
        yRepresentations.observe(observer);
        return () => yRepresentations.unobserve(observer);
      },
      representations: (subscribe: Subscribe) => () => {},
      port: (subscribe: Subscribe) => {
        const yPorts = this.yType.get("ports") as YPortMap;
        const observer = () => subscribe();
        yPorts.observe(observer);
        return () => yPorts.unobserve(observer);
      },
      ports: (subscribe: Subscribe) => () => {},
    },
  };
}

class YPieceStore implements PieceStoreFull {
  public readonly yPiece: YPiece = new Y.Map<any>();

  constructor(piece: Piece) {
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
  get piece(): Piece {
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
  }
  create = {};
  update = {
    piece: (diff: PieceDiff) => {
      if (diff.id_ !== undefined) this.yPiece.set("id_", diff.id_);
      if (diff.description !== undefined) this.yPiece.set("description", diff.description);
      if (diff.type !== undefined) {
        const yType = this.yPiece.get("type") as Y.Map<string>;
        yType.set("name", diff.type.name);
        if (diff.type.variant !== undefined) yType.set("variant", diff.type.variant);
      }
      // if (diff.design !== undefined) {
      //   const yDesign = this.yPiece.get("design") as Y.Map<string>;
      //   if (yDesign) {
      //     yDesign.set("name", diff.design.name);
      //     if (diff.design.variant !== undefined) yDesign.set("variant", diff.design.variant);
      //     if (diff.design.view !== undefined) yDesign.set("view", diff.design.view);
      //   }
      // }
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
    },
  };

  delete = {};
  on = {
    created: {},
    updated: {
      piece: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yPiece.observeDeep(observer);
          return () => this.yPiece.unobserveDeep(observer);
        } else {
          this.yPiece.observe(observer);
          return () => this.yPiece.unobserve(observer);
        }
      },
    },
    deleted: {},
  };
}

class YConnectionStore implements ConnectionStoreFull {
  public readonly yConnection: YConnection = new Y.Map<any>();

  constructor(connection: Connection) {
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

  get connection(): Connection {
    const yConnected = this.yConnection.get("connected") as Y.Map<any>;
    const yConnecting = this.yConnection.get("connecting") as Y.Map<any>;
    const yConnectedPiece = yConnected.get("piece") as Y.Map<string>;
    const yConnectedPort = yConnected.get("port") as Y.Map<string>;
    const yConnectingPiece = yConnecting.get("piece") as Y.Map<string>;
    const yConnectingPort = yConnecting.get("port") as Y.Map<string>;

    return {
      connected: {
        piece: { id_: yConnectedPiece.get("id_") as string },
        port: { id_: yConnectedPort.get("id_") as string },
        designId: yConnected.get("designId") as string | undefined,
      },
      connecting: {
        piece: { id_: yConnectingPiece.get("id_") as string },
        port: { id_: yConnectingPort.get("id_") as string },
        designId: yConnecting.get("designId") as string | undefined,
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
  }
  create = {};

  update = {
    connection: (diff: ConnectionDiff) => {
      if (diff.description !== undefined) this.yConnection.set("description", diff.description);
      if (diff.gap !== undefined) this.yConnection.set("gap", diff.gap);
      if (diff.shift !== undefined) this.yConnection.set("shift", diff.shift);
      if (diff.rise !== undefined) this.yConnection.set("rise", diff.rise);
      if (diff.rotation !== undefined) this.yConnection.set("rotation", diff.rotation);
      if (diff.turn !== undefined) this.yConnection.set("turn", diff.turn);
      if (diff.tilt !== undefined) this.yConnection.set("tilt", diff.tilt);
      if (diff.x !== undefined) this.yConnection.set("x", diff.x);
      if (diff.y !== undefined) this.yConnection.set("y", diff.y);
    },
  };

  delete = {};

  on = {
    created: {},
    updated: {
      connection: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yConnection.observeDeep(observer);
          return () => this.yConnection.unobserveDeep(observer);
        } else {
          this.yConnection.observe(observer);
          return () => this.yConnection.unobserve(observer);
        }
      },
    },
    deleted: {},
  };
}

class YDesignStore implements DesignStoreFull {
  public readonly yDesign: YDesign = new Y.Map<any>();
  public readonly pieces: Map<PieceId, YPieceStore> = new Map();
  public readonly connections: Map<ConnectionId, YConnectionStore> = new Map();
  private readonly pieceIds: Map<PieceId, string> = new Map();
  private readonly connectionIds: Map<ConnectionId, string> = new Map();

  constructor(design: Design) {
    this.yDesign.set("name", design.name);
    this.yDesign.set("description", design.description || "");
    this.yDesign.set("variant", design.variant || "");
    this.yDesign.set("view", design.view || "");
    this.yDesign.set("unit", design.unit);
    this.yDesign.set("pieces", new Y.Map() as YPieceMap);
    this.yDesign.set("connections", new Y.Map() as YConnectionMap);
    this.yDesign.set("authors", createAuthors(design.authors));
    this.yDesign.set("attributes", createAttributes(design.attributes));
    this.create.pieces(design.pieces || []);
    this.create.connections(design.connections || []);
  }

  get design(): Design {
    return {
      name: this.yDesign.get("name") as string,
      description: this.yDesign.get("description") as string | undefined,
      variant: this.yDesign.get("variant") as string | undefined,
      view: this.yDesign.get("view") as string | undefined,
      unit: this.yDesign.get("unit") as string,
      pieces: Array.from(this.pieces.values()).map((p) => p.piece),
      connections: Array.from(this.connections.values()).map((c) => c.connection),
      authors: getAuthors(this.yDesign.get("authors") as YAuthors),
      attributes: getAttributes(this.yDesign.get("attributes") as YAttributes),
    };
  }

  create = {
    piece: (piece: Piece) => {
      const pieceId = pieceIdLikeToPieceId(piece);
      if (this.pieceIds.get(pieceId)) throw new Error(`Piece ${pieceId} already exists`);
      const uuid = uuidv4();
      const yPieceStore = new YPieceStore(piece);
      (this.yDesign.get("pieces") as YPieceMap).set(uuid, yPieceStore.yPiece);
      this.pieceIds.set(pieceId, uuid);
      this.pieces.set(pieceId, yPieceStore);
    },
    pieces: (pieces: Piece[]) => pieces.forEach((piece) => this.create.piece(piece)),
    connection: (connection: Connection) => {
      const connectionId = connectionIdLikeToConnectionId(connection);
      if (this.connectionIds.get(connectionId)) throw new Error(`Connection ${connectionId} already exists`);
      const uuid = uuidv4();
      const yConnectionStore = new YConnectionStore(connection);
      (this.yDesign.get("connections") as YConnectionMap).set(uuid, yConnectionStore.yConnection);
      this.connectionIds.set(connectionId, uuid);
      this.connections.set(connectionId, yConnectionStore);
    },
    connections: (connections: Connection[]) => connections.forEach((connection) => this.create.connection(connection)),
  };

  update = {
    design: (diff: DesignDiff) => {
      if (diff.name !== undefined) this.yDesign.set("name", diff.name);
      if (diff.description !== undefined) this.yDesign.set("description", diff.description);
      if (diff.variant !== undefined) this.yDesign.set("variant", diff.variant);
      if (diff.view !== undefined) this.yDesign.set("view", diff.view);
      if (diff.unit !== undefined) this.yDesign.set("unit", diff.unit);
    },
  };

  delete = {
    piece: (id: PieceId) => {
      const pieceId = pieceIdLikeToPieceId(id);
      const uuid = this.pieceIds.get(pieceId);
      if (!uuid) throw new Error(`Piece ${pieceId} does not exist`);
      (this.yDesign.get("pieces") as YPieceMap).delete(uuid);
      this.pieceIds.delete(pieceId);
      this.pieces.delete(pieceId);
    },
    pieces: (ids: PieceId[]) => ids.forEach((id) => this.delete.piece(id)),
    connection: (id: ConnectionId) => {
      const connectionId = connectionIdLikeToConnectionId(id);
      const uuid = this.connectionIds.get(connectionId);
      if (!uuid) throw new Error(`Connection ${connectionId} does not exist`);
      (this.yDesign.get("connections") as YConnectionMap).delete(uuid);
      this.connectionIds.delete(connectionId);
      this.connections.delete(connectionId);
    },
    connections: (ids: ConnectionId[]) => ids.forEach((id) => this.delete.connection(id)),
  };

  on = {
    created: {
      piece: (subscribe: Subscribe) => {
        const yPieces = this.yDesign.get("pieces") as YPieceMap;
        const observer = () => subscribe();
        yPieces.observe(observer);
        return () => yPieces.unobserve(observer);
      },
      pieces: (subscribe: Subscribe) => () => {},
      connection: (subscribe: Subscribe) => {
        const yConnections = this.yDesign.get("connections") as YConnectionMap;
        const observer = () => subscribe();
        yConnections.observe(observer);
        return () => yConnections.unobserve(observer);
      },
      connections: (subscribe: Subscribe) => () => {},
    },
    updated: {
      design: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        if (deep) {
          this.yDesign.observeDeep(observer);
          return () => this.yDesign.unobserveDeep(observer);
        } else {
          this.yDesign.observe(observer);
          return () => this.yDesign.unobserve(observer);
        }
      },
    },
    deleted: {
      piece: (subscribe: Subscribe) => {
        const yPieces = this.yDesign.get("pieces") as YPieceMap;
        const observer = () => subscribe();
        yPieces.observe(observer);
        return () => yPieces.unobserve(observer);
      },
      pieces: (subscribe: Subscribe) => () => {},
      connection: (subscribe: Subscribe) => {
        const yConnections = this.yDesign.get("connections") as YConnectionMap;
        const observer = () => subscribe();
        yConnections.observe(observer);
        return () => yConnections.unobserve(observer);
      },
      connections: (subscribe: Subscribe) => () => {},
    },
  };
}

class YKitStore implements KitStoreFull {
  public readonly yKit: YKit = new Y.Map() as YKit;
  public readonly types: Map<TypeId, YTypeStore> = new Map();
  public readonly designs: Map<DesignId, YDesignStore> = new Map();
  private readonly typeIds: Map<TypeId, string> = new Map();
  private readonly designIds: Map<DesignId, string> = new Map();
  private readonly commandRegistry: Map<string, (context: KitCommandContext, ...rest: any[]) => KitCommandResult> = new Map();

  constructor(kit: Kit) {
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
    this.create.types(kit.types || []);
    this.create.designs(kit.designs || []);
  }

  get kit(): Kit {
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
      types: Array.from(this.types.values()).map((store) => store.type),
      designs: Array.from(this.designs.values()).map((store) => store.design),
      attributes: getAttributes(this.yKit.get("attributes") as YAttributes),
    };
  }

  create = {
    type: (type: Type) => {
      const typeId = typeIdLikeToTypeId(type);
      if (this.typeIds.get(typeId)) throw new Error(`Type ${typeId} already exists`);
      const uuid = uuidv4();
      const yTypeStore = new YTypeStore(type);
      (this.yKit.get("types") as YTypeMap).set(uuid, yTypeStore.yType);
      this.typeIds.set(typeId, uuid);
      this.types.set(typeId, yTypeStore);
    },
    types: (types: Type[]) => {
      types.forEach((type) => this.create.type(type));
    },
    design: (design: Design) => {
      const designId = designIdLikeToDesignId(design);
      if (this.designIds.get(designId)) throw new Error(`Design ${designId} already exists`);
      const uuid = uuidv4();
      const yDesignStore = new YDesignStore(design);
      (this.yKit.get("designs") as YDesignMap).set(uuid, yDesignStore.yDesign);
      this.designIds.set(designId, uuid);
      this.designs.set(designId, yDesignStore);
    },
    designs: (designs: Design[]) => {
      designs.forEach((design) => this.create.design(design));
    },
  };

  update = {
    kit: (diff: KitDiff) => {
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
        if (diff.types.added) diff.types.added.forEach((type) => this.create.type(type));
        if (diff.types.removed) diff.types.removed.forEach((typeId) => this.delete.type(typeId));
        if (diff.types.updated) {
          // For updated types, we need to find the existing store and update it
          // This is a limitation of the current diff structure - we need the full type ID
          diff.types.updated.forEach((typeDiff) => {
            // Find the matching type by partial data - this is imperfect but necessary
            const matchingType = Array.from(this.types.entries()).find(([_, store]) => {
              const type = store.type;
              return (!typeDiff.name || type.name === typeDiff.name) && (!typeDiff.variant || type.variant === typeDiff.variant);
            });
            if (matchingType) {
              matchingType[1].update.type(typeDiff);
            }
          });
        }
      }
      if (diff.designs) {
        if (diff.designs.added) diff.designs.added.forEach((design) => this.create.design(design));
        if (diff.designs.removed) diff.designs.removed.forEach((designId) => this.delete.design(designId));
        if (diff.designs.updated) {
          // For updated designs, we need to find the existing store and update it
          diff.designs.updated.forEach((designDiff) => {
            // Find the matching design by partial data - this is imperfect but necessary
            const matchingDesign = Array.from(this.designs.entries()).find(([_, store]) => {
              const design = store.design;
              return (!designDiff.name || design.name === designDiff.name) && (!designDiff.variant || design.variant === designDiff.variant) && (!designDiff.view || design.view === designDiff.view);
            });
            if (matchingDesign) {
              matchingDesign[1].update.design(designDiff);
            }
          });
        }
      }
      // TODO: attributes
      this.yKit.set("updated", new Date().toISOString());
    },
  };

  delete = {
    type: (id: TypeId) => {
      const typeId = typeIdLikeToTypeId(id);
      const uuid = this.typeIds.get(typeId);
      if (!uuid) throw new Error(`Type ${typeId} does not exist`);
      (this.yKit.get("types") as YTypeMap).delete(uuid);
      this.typeIds.delete(typeId);
      this.types.delete(typeId);
    },
    types: (ids: TypeId[]) => ids.forEach((id) => this.delete.type(id)),
    design: (id: DesignId) => {
      const designId = designIdLikeToDesignId(id);
      const uuid = this.designIds.get(designId);
      if (!uuid) throw new Error(`Design ${designId} does not exist`);
      (this.yKit.get("designs") as YDesignMap).delete(uuid);
      this.designIds.delete(designId);
      this.designs.delete(designId);
    },
    designs: (ids: DesignId[]) => ids.forEach((id) => this.delete.design(id)),
  };

  on = {
    created: {
      type: (subscribe: Subscribe) => {
        const yTypes = this.yKit.get("types") as Y.Map<YType>;
        const observer = () => subscribe();
        yTypes.observe(observer);
        return () => yTypes.unobserve(observer);
      },
      types: (subscribe: Subscribe) => this.on.created.type(subscribe),
      design: (subscribe: Subscribe) => {
        const yDesigns = this.yKit.get("designs") as Y.Map<YDesign>;
        const observer = () => subscribe();
        yDesigns.observe(observer);
        return () => yDesigns.unobserve(observer);
      },
      designs: (subscribe: Subscribe) => this.on.created.design(subscribe),
    },
    updated: {
      kit: (subscribe: Subscribe, deep?: boolean) => {
        const observer = () => subscribe();
        deep ? (this.yKit as unknown as Y.Map<any>).observeDeep(observer) : (this.yKit as unknown as Y.Map<any>).observe(observer);
        return () => (deep ? (this.yKit as unknown as Y.Map<any>).unobserveDeep(observer) : (this.yKit as unknown as Y.Map<any>).unobserve(observer));
      },
    },
    deleted: {
      type: (subscribe: Subscribe) => {
        const yTypes = this.yKit.get("types") as Y.Map<YType>;
        const observer = () => subscribe();
        yTypes.observe(observer);
        return () => yTypes.unobserve(observer);
      },
      types: (subscribe: Subscribe) => this.on.deleted.type(subscribe),
      design: (subscribe: Subscribe) => {
        const yDesigns = this.yKit.get("designs") as Y.Map<YDesign>;
        const observer = () => subscribe();
        yDesigns.observe(observer);
        return () => yDesigns.unobserve(observer);
      },
      designs: (subscribe: Subscribe) => this.on.deleted.design(subscribe),
    },
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in kit store`);
    const context: KitCommandContext = { kit: this.kit };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.update.kit(result.diff);
    }
    return result as T;
  }

  registerCommand(command: string, callback: (context: KitCommandContext, ...rest: any[]) => KitCommandResult): Disposable {
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

class YDesignEditorStore implements DesignEditorStore {
  public readonly yDesignEditorStore: YDesignEditorStoreValMap = new Y.Map<YDesignEditorStoreVal>();
  private readonly commandRegistry: Map<string, (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult> = new Map();

  constructor(state: DesignEditorState) {
    this.yDesignEditorStore.set("fullscreenPanel", state.fullscreenPanel);
    this.yDesignEditorStore.set("selectedPieceIds", new Y.Array<string>());
    this.yDesignEditorStore.set("selectedConnections", new Y.Array<string>());
    this.yDesignEditorStore.set("isTransactionActive", state.isTransactionActive);
    this.yDesignEditorStore.set("presenceCursorX", state.presence.cursor?.x || 0);
    this.yDesignEditorStore.set("presenceCursorY", state.presence.cursor?.y || 0);
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

  undo = () => {};
  redo = () => {};

  set = {
    fullscreenPanel: (fullscreenPanel: DesignEditorFullscreenPanel) => {
      this.yDesignEditorStore.set("fullscreenPanel", fullscreenPanel);
    },
    selection: (selection: DesignEditorSelection) => {
      if (selection.pieceIds) {
        const yPieceIds = this.yDesignEditorStore.get("selectedPieceIds") as Y.Array<string>;
        yPieceIds.delete(0, yPieceIds.length);
        const pieceIdStrings = selection.pieceIds.map((piece) => piece.id_);
        yPieceIds.insert(0, pieceIdStrings);
      }
      if (selection.connectionIds) {
        const yConnectionIds = this.yDesignEditorStore.get("selectedConnections") as Y.Array<string>;
        yConnectionIds.delete(0, yConnectionIds.length);
        const connectionIdStrings = selection.connectionIds.map((conn) => `${conn.connected.piece.id_}->${conn.connecting.piece.id_}`);
        yConnectionIds.insert(0, connectionIdStrings);
      }
    },
    presence: (presence: DesignEditorPresence) => {
      if (presence.cursor) {
        this.yDesignEditorStore.set("presenceCursorX", presence.cursor.x);
        this.yDesignEditorStore.set("presenceCursorY", presence.cursor.y);
      }
    },
    others: (others: DesignEditorPresenceOther[]) => {
      // Others are typically managed by collaboration providers, not stored in Yjs directly
    },
  };

  transaction = {
    start: () => {
      this.yDesignEditorStore.set("isTransactionActive", true);
    },
    abort: () => {
      this.yDesignEditorStore.set("isTransactionActive", false);
    },
    finalize: () => {
      if (this.isTransactionActive) {
        this.yDesignEditorStore.set("isTransactionActive", false);
      }
    },
  };

  on = {
    undone: (subscribe: Subscribe) => {
      // For undo/redo, we'd need to track specific state changes
      // This is a simplified placeholder
      return () => {};
    },
    redone: (subscribe: Subscribe) => {
      return () => {};
    },
    updated: {
      designEditor: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
    },
    set: {
      fullscreenPanel: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
      selection: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
      presence: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
      others: (subscribe: Subscribe) => {
        // Others updates are typically handled by collaboration providers
        return () => {};
      },
    },
    transaction: {
      started: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
      aborted: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
      finalized: (subscribe: Subscribe) => {
        const observer = () => subscribe();
        this.yDesignEditorStore.observe(observer);
        return () => this.yDesignEditorStore.unobserve(observer);
      },
    },
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design editor store`);
    const context: DesignEditorCommandContext = {
      store: this as any, // This would need proper context
      kitId: { name: "", version: "" }, // This would need to be provided
      designId: { name: "", variant: "", view: "" }, // This would need to be provided
    };
    const result = callback(context, ...rest);
    // Apply state and diff changes
    // This would need proper implementation
    return result as T;
  }

  registerCommand(command: string, callback: (context: DesignEditorCommandContext, ...rest: any[]) => DesignEditorCommandResult): Disposable {
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

class YSketchpadStore implements SketchpadStore {
  public readonly ySketchpadDoc: Y.Doc;
  public readonly sketchpadIndexeddbProvider?: IndexeddbPersistence;
  public readonly yKitDocs: Map<KitId, Y.Doc> = new Map();
  public readonly kitIndexeddbProviders: Map<KitId, IndexeddbPersistence> = new Map();
  public readonly kits: Map<KitId, KitStore> = new Map();
  public readonly designEditors: Map<DesignId, DesignEditorStore> = new Map();
  private readonly commandRegistry: Map<string, (context: SketchpadCommandContext, ...rest: any[]) => SketchpadCommandResult> = new Map();

  constructor(state: SketchpadState) {
    this.ySketchpadDoc = new Y.Doc();
    if (state.persistantId && state.persistantId !== "") {
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad-${state.persistantId}`, this.ySketchpadDoc);
    }
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("mode", state.mode);
    ySketchpad.set("theme", state.theme);
    ySketchpad.set("layout", state.layout);
    if (state.activeDesignEditorDesignId) {
      ySketchpad.set("activeDesignEditorDesignId", JSON.stringify(state.activeDesignEditorDesignId));
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
  get activeDesignEditorDesignId(): DesignId | undefined {
    const designIdStr = this.getYSketchpad().get("activeDesignEditorDesignId");
    if (!designIdStr || typeof designIdStr !== "string") return undefined;
    return JSON.parse(designIdStr) as DesignId;
  }

  private getYSketchpad(): YSketchpad {
    return this.ySketchpadDoc.getMap("sketchpad");
  }

  create = {
    kit: (kit: Kit) => {
      const kitId = kitIdLikeToKitId(kit);
      if (this.kits.has(kitId)) throw new Error(`Kit ${kitId} already exists`);
      const yKitStore = new YKitStore(kit);
      const yKitDoc = new Y.Doc();
      yKitDoc.getMap("kit").set("data", yKitStore.yKit);
      this.yKitDocs.set(kitId, yKitDoc);
      this.kits.set(kitId, yKitStore);
    },
    kits: (kits: Kit[]) => {
      kits.forEach((kit) => this.create.kit(kit));
    },
    designEditor: (designId: DesignId) => {
      const initialState: DesignEditorState = {
        fullscreenPanel: DesignEditorFullscreenPanel.None,
        selection: {},
        isTransactionActive: false,
        presence: {},
        others: [],
        diff: {},
      };
      const editorStore = new YDesignEditorStore(initialState);
      this.designEditors.set(designId, editorStore);
    },
    designEditors: (designIds: DesignId[]) => {
      designIds.forEach((id) => this.create.designEditor(id));
    },
  };

  update = {
    kit: (kitId: KitId, diff: KitDiff) => {
      const store = this.kits.get(kitId);
      if (store) {
        store.update.kit(diff);
      }
    },
  };

  delete = {
    kit: (kitId: KitId) => {
      this.kits.delete(kitId);
    },
    kits: (kitIds: KitId[]) => {
      kitIds.forEach((id) => this.delete.kit(id));
    },
    designEditor: (designId: DesignId) => {
      this.designEditors.delete(designId);
    },
    designEditors: (designIds: DesignId[]) => {
      designIds.forEach((id) => this.delete.designEditor(id));
    },
  };

  set = {
    mode: (mode: Mode) => {
      this.getYSketchpad().set("mode", mode);
    },
    theme: (theme: Theme) => {
      this.getYSketchpad().set("theme", theme);
    },
    layout: (layout: Layout) => {
      this.getYSketchpad().set("layout", layout);
    },
    activeDesignEditorDesignId: (id?: string) => {
      const ySketchpad = this.getYSketchpad();
      if (id) {
        ySketchpad.set("activeDesignEditorDesignId", id);
      } else {
        ySketchpad.delete("activeDesignEditorDesignId");
      }
    },
  };

  // Subscriptions using Yjs observers
  on = {
    created: {
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
    },
    updated: {
      sketchpad: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
      kits: (subscribe: Subscribe): Unsubscribe => {
        // For multiple kits, we'd need to observe all kit documents
        // This is a placeholder implementation
        const observer = () => subscribe();
        return () => {}; // Would need to unobserve all kit docs
      },
      designEditor: (subscribe: Subscribe): Unsubscribe => {
        const yDesignEditorStores = this.ySketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
        const observer = () => subscribe();
        yDesignEditorStores.observeDeep(observer);
        return () => yDesignEditorStores.unobserveDeep(observer);
      },
    },
    deleted: {
      kit: (subscribe: Subscribe): Unsubscribe => {
        // Kit deletion would be observed through document removal
        const observer = () => subscribe();
        return () => {}; // Placeholder
      },
      designEditor: (subscribe: Subscribe): Unsubscribe => {
        const yDesignEditorStores = this.ySketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
        const observer = () => subscribe();
        yDesignEditorStores.observe(observer);
        return () => yDesignEditorStores.unobserve(observer);
      },
    },
    set: {
      mode: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
      theme: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
      layout: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
      activeDesignEditorDesignId: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
    },
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in sketchpad store`);
    const context: SketchpadCommandContext = { store: this };
    const result = callback(context, ...rest);
    // Apply state and diffs changes
    if (result.state) {
      // Apply state changes - would need proper implementation
    }
    if (result.diffs) {
      result.diffs.forEach((diff, kitId) => {
        const kit = this.kits.get(kitId);
        if (kit) {
          kit.update.kit(diff);
        }
      });
    }
    return result as T;
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
    stores.set(id, new YSketchpadStore(initialState));
  }
  return stores.get(id)!;
}

// #endregion Stores

// #region Commands

const sketchpadCommands = {
  "semio.sketchpad.turnDark": (context: SketchpadCommandContext): SketchpadCommandResult => {
    return {
      state: { ...context.state, theme: Theme.DARK },
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
  "semio.designEditor.addPiece": (context: DesignEditorCommandContext, piece: Piece): DesignEditorCommandResult => {
    return {
      diff: {
        designs: {
          updated: {
            name: context.designId.name,
            variant: context.designId.variant,
            view: context.designId.view,
            pieces: { added: [piece] },
          },
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
const useKitScope = () => useContext(KitScopeContext);
const useDesignScope = () => useContext(DesignScopeContext);
const useTypeScope = () => useContext(TypeScopeContext);
const usePieceScope = () => useContext(PieceScopeContext);
const useConnectionScope = () => useContext(ConnectionScopeContext);
const useRepresentationScope = () => useContext(RepresentationScopeContext);
const usePortScope = () => useContext(PortypeScopeContext);
const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

// #endregion Scoping

function useStore<T, S>(store: S, subscribe?: (cb: () => void) => () => void, selector?: (store: S) => T): T {
  const [value, setValue] = useState(() => (selector ? selector(store) : store));

  useEffect(() => {
    if (!subscribe) return;
    const unsubscribe = subscribe(() => {
      setValue(selector ? selector(store) : (store as unknown as T));
    });
    return unsubscribe;
  }, [store, selector]);

  return value as T;
}

export function useSketchpad(): SketchpadStore;
export function useSketchpad<T>(selector: (store: SketchpadStore) => T, id?: string): T;
export function useSketchpad<T>(selector?: (store: SketchpadStore) => T, id?: string): T | SketchpadStore {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  return useStore(store, store.on.updated.sketchpad, selector);
}

export function useDesignEditor(): DesignEditorStore;
export function useDesignEditor<T>(selector: (store: DesignEditorStore) => T): T;
export function useDesignEditor<T>(selector: (store: DesignEditorStore) => T, id: DesignId): T;
export function useDesignEditor<T>(selector?: (store: DesignEditorStore) => T, id?: DesignId): T | DesignEditorStore {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesignEditor must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const designScope = useDesignScope();
  const designId = designScope?.id ?? id;
  if (!designId) throw new Error("useDesignEditor must be called within a DesignScopeProvider or be directly provided with an id");
  if (!store.designEditors.has(designId)) throw new Error(`Design editor store not found for design ${designId}`);
  const designEditor = store.designEditors.get(designId)!;
  return useStore(designEditor, designEditor.on.updated.designEditor, selector);
}

export function useKit(): KitStore;
export function useKit<T>(selector: (store: KitStore) => T): T;
export function useKit<T>(selector: (store: KitStore) => T, id: KitId): T;
export function useKit<T>(selector?: (store: KitStore) => T, id?: KitId): T | KitStore {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKit must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  const kitId = kitScope?.id ?? id;
  if (!kitId) throw new Error("useKit must be called within a KitScopeProvider or be directly provided with an id");
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore, kitStore.on.updated.kit, selector);
}

export function useDesign(): Design;
export function useDesign<T>(selector: (design: Design) => T): T;
export function useDesign<T>(selector: (design: Design) => T, id: DesignId): T;
export function useDesign<T>(selector?: (design: Design) => T, id?: DesignId): T | Design {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesign must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useDesign must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  const designId = designScope?.id ?? id;
  if (!designId) throw new Error("useDesign must be called within a DesignScopeProvider or be directly provided with an id");
  if (!kitStore.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kitStore.designs.get(designId)!;
  return useStore(designStore.design, designStore.on.updated.design, selector);
}

export function useType(): Type;
export function useType<T>(selector: (type: Type) => T): T;
export function useType<T>(selector: (type: Type) => T, id: TypeId): T;
export function useType<T>(selector?: (type: Type) => T, id?: TypeId): T | Type {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useType must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useType must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  const typeId = typeScope?.id ?? id;
  if (!typeId) throw new Error("useType must be called within a TypeScopeProvider or be directly provided with an id");
  if (!kit.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kit.types.get(typeId)!;
  return useStore(typeStore.type, typeStore.on.updated.type, selector);
}

export function usePiece(): Piece;
export function usePiece<T>(selector: (piece: Piece) => T): T;
export function usePiece<T>(selector: (piece: Piece) => T, id: PieceId): T;
export function usePiece<T>(selector?: (piece: Piece) => T, id?: PieceId): T | Piece {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePiece must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
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
  return useStore(pieceStore.piece, pieceStore.on.updated.piece, selector);
}

export function useConnection(): Connection;
export function useConnection<T>(selector: (connection: Connection) => T): T;
export function useConnection<T>(selector: (connection: Connection) => T, id: ConnectionId): T;
export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId): T | Connection {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useConnection must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
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
  return useStore(connectionStore.connection, connectionStore.on.updated.connection, selector);
}

export function usePort(): Port;
export function usePort<T>(selector: (port: Port) => T): T;
export function usePort<T>(selector: (port: Port) => T, id: PortId): T;
export function usePort<T>(selector?: (port: Port) => T, id?: PortId): T | Port {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePort must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
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
  return useStore(portStore.port, portStore.on.updated.port, selector);
}

export function useRepresentation(): Representation;
export function useRepresentation<T>(selector: (representation: Representation) => T): T;
export function useRepresentation<T>(selector: (representation: Representation) => T, id: RepresentationId): T;
export function useRepresentation<T>(selector?: (representation: Representation) => T, id?: RepresentationId): T | Representation {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useRepresentation must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
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
  return useStore(representationStore.representation, representationStore.on.updated.representation, selector);
}
// #endregion Hooks
