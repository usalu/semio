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
import React, { createContext, useContext, useRef, useSyncExternalStore } from "react";
import { v4 as uuidv4 } from "uuid";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
import { UndoManager } from "yjs";
// Import initSqlJs
import type { Database, SqlJsStatic, SqlValue } from "sql.js";
import initSqlJs from "sql.js";
import sqlWasmUrl from "sql.js/dist/sql-wasm.wasm?url";
import {
  Author,
  Camera,
  Connection,
  ConnectionId,
  ConnectionIdLike,
  connectionIdLikeToConnectionId,
  ConnectionSchema,
  Design,
  DesignDiff,
  DesignId,
  DesignIdLike,
  designIdLikeToDesignId,
  DesignSchema,
  DiagramPoint,
  flattenDesign,
  Kit,
  KitId,
  KitIdLike,
  kitIdLikeToKitId,
  KitSchema,
  Piece,
  PieceId,
  PieceIdLike,
  pieceIdLikeToPieceId,
  PieceSchema,
  Plane,
  Point,
  Port,
  PortId,
  PortIdLike,
  portIdLikeToPortId,
  PortSchema,
  Quality,
  Representation,
  RepresentationId,
  RepresentationIdLike,
  representationIdLikeToRepresentationId,
  RepresentationSchema,
  Side,
  Type,
  TypeId,
  TypeIdLike,
  typeIdLikeToTypeId,
  TypeSchema,
  Vector,
} from "./semio";

// import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';

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

export type DesignEditorSelection = { selectedPieceIds: PieceId[]; selectedConnections: ConnectionId[]; selectedPiecePortId?: { pieceId: PieceId; portId: PortId } };
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

export interface DesignEditorOperationStackEntry {
  undo: DesignDiff;
  redo: DesignDiff;
  selection: DesignEditorSelection;
}
export interface DesignEditorState {
  designId: DesignId;
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  operationStack: DesignEditorOperationStackEntry[];
  operationIndex: number;
  presence: DesignEditorPresence;
  others?: DesignEditorPresenceOther[];
}

// Yjs alias value types used in this store (precise, no any)
type YAuthor = Y.Map<string>;
type YAuthors = Y.Map<YAuthor>;
type YQuality = Y.Map<string>;
type YQualities = Y.Array<YQuality>;
type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;
type YVec3 = YLeafMapNumber;
type YPlane = Y.Map<YVec3>;

type YRepresentationVal = string | YStringArray | YQualities;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentationMap = Y.Map<YRepresentation>;

type YPortVal = string | number | boolean | YLeafMapNumber | YQualities | YStringArray;
type YPort = Y.Map<YPortVal>;
type YPortMap = Y.Map<YPort>;

type YPieceVal = string | YLeafMapString | YLeafMapNumber | YPlane | YQualities;
type YPiece = Y.Map<YPieceVal>;
type YPieceMap = Y.Map<YPiece>;

type YSide = Y.Map<YLeafMapString>;
type YConnectionVal = string | number | YQualities | YSide;
type YConnection = Y.Map<YConnectionVal>;
type YConnectionMap = Y.Map<YConnection>;

type YTypeVal = string | number | boolean | YAuthors | YQualities | YRepresentationMap | YPortMap;
type YType = Y.Map<YTypeVal>;
type YTypeMap = Y.Map<YType>;

type YDesignVal = string | YAuthors | YQualities | YPieceMap | YConnectionMap;
type YDesign = Y.Map<YDesignVal>;
type YDesignMap = Y.Map<YDesign>;

type YIdMap = Y.Map<string>;
type YKitVal = string | YTypeMap | YDesignMap | YIdMap | YQualities;
type YKit = Y.Map<YKitVal>;

// Typed key maps and getters
type YKitKeysMap = {
  name: string;
  description: string;
  icon: string;
  image: string;
  version: string;
  preview: string;
  remote: string;
  homepage: string;
  license: string;
  created: string;
  updated: string;
  types: YTypeMap;
  designs: YDesignMap;
  typeIds: YIdMap;
  designIds: YIdMap;
  qualities: YQualities;
};
const gKit = <K extends keyof YKitKeysMap>(m: YKit, k: K): YKitKeysMap[K] => m.get(k as string) as YKitKeysMap[K];

type YTypeKeysMap = {
  name: string;
  description: string;
  icon: string;
  image: string;
  variant: string;
  stock: number;
  virtual: boolean;
  unit: string;
  representations: YRepresentationMap;
  ports: YPortMap;
  authors: YAuthors;
  qualities: YQualities;
  created: string;
  updated: string;
};
const gType = <K extends keyof YTypeKeysMap>(m: YType, k: K): YTypeKeysMap[K] => m.get(k as string) as YTypeKeysMap[K];

type YDesignKeysMap = {
  name: string;
  description: string;
  icon: string;
  image: string;
  variant: string;
  view: string;
  unit: string;
  created: string;
  updated: string;
  authors: YAuthors;
  pieces: YPieceMap;
  connections: YConnectionMap;
  qualities: YQualities;
};
const gDesign = <K extends keyof YDesignKeysMap>(m: YDesign, k: K): YDesignKeysMap[K] => m.get(k as string) as YDesignKeysMap[K];

type YRepresentationKeysMap = {
  url: string;
  description: string;
  tags: YStringArray;
  qualities: YQualities;
};
const gRep = <K extends keyof YRepresentationKeysMap>(m: YRepresentation, k: K): YRepresentationKeysMap[K] => m.get(k as string) as YRepresentationKeysMap[K];

type YPortKeysMap = {
  id_: string;
  description: string;
  mandatory: boolean;
  family: string;
  compatibleFamilies: YStringArray;
  direction: YVec3;
  point: YVec3;
  t: number;
  qualities: YQualities;
};
const gPort = <K extends keyof YPortKeysMap>(m: YPort, k: K): YPortKeysMap[K] => m.get(k as string) as YPortKeysMap[K];

type YPieceKeysMap = {
  id_: string;
  description: string;
  type: YLeafMapString; // { name, variant }
  plane: YPlane;
  center: YLeafMapNumber; // { x,y }
  qualities: YQualities;
};
const gPiece = <K extends keyof YPieceKeysMap>(m: YPiece, k: K): YPieceKeysMap[K] => m.get(k as string) as YPieceKeysMap[K];

type YSideKeysMap = {
  piece: YLeafMapString; // { id_ }
  port: YLeafMapString; // { id_ }
};
const gSide = <K extends keyof YSideKeysMap>(m: YSide, k: K): YSideKeysMap[K] => m.get(k as string) as YSideKeysMap[K];

type YConnectionKeysMap = {
  connected: YSide;
  connecting: YSide;
  description: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  qualities: YQualities;
};
const gConn = <K extends keyof YConnectionKeysMap>(m: YConnection, k: K): YConnectionKeysMap[K] => m.get(k as string) as YConnectionKeysMap[K];

export class SketchpadStore {
  private id?: string;
  private yDoc: Y.Doc;
  private undoManager: UndoManager;
  private designEditors: Map<string, { yKit: YKit; yDesign: YDesign; undoManager: UndoManager; state: DesignEditorState; listeners: Set<() => void> }> = new Map();
  private indexeddbProvider?: IndexeddbPersistence;
  private listeners: Set<() => void> = new Set();
  private fileUrls: Map<string, string> = new Map();

  private key = {
    kit: (id: KitIdLike) => {
      const kitId = kitIdLikeToKitId(id);
      return `${kitId.name}::${kitId.version || ""}`;
    },
    type: (id: TypeIdLike) => {
      const typeId = typeIdLikeToTypeId(id);
      return `${typeId.name}::${typeId.variant || ""}`;
    },
    design: (id: DesignIdLike) => {
      const designId = designIdLikeToDesignId(id);
      return `${designId.name}::${designId.variant || ""}::${designId.view || ""}`;
    },
    piece: (id: PieceIdLike) => {
      const pieceId = pieceIdLikeToPieceId(id);
      return pieceId.id_;
    },
    connection: (id: ConnectionIdLike) => {
      const connectionId = connectionIdLikeToConnectionId(id);
      return `${connectionId.connected.piece.id_}--${connectionId.connecting.piece.id_}`;
    },
    port: (id: PortIdLike) => {
      const portId = portIdLikeToPortId(id);
      return portId.id_ || "";
    },
    representation: (id: RepresentationIdLike) => {
      const repId = representationIdLikeToRepresentationId(id);
      return repId.tags?.join(",") || "";
    },
  };

  constructor(id?: string) {
    this.id = id;
    this.yDoc = new Y.Doc();
    if (id) this.indexeddbProvider = new IndexeddbPersistence(`semio-sketchpad:${id}`, this.yDoc);
    this.yDoc.getMap<YKit>("kits");
    this.yDoc.getMap<string>("kitIds");
    this.yDoc.getMap<Uint8Array>("files");
    this.undoManager = new UndoManager(this.yDoc.getMap<YKit>("kits"), { captureTimeout: 0 });
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private getKitUuid(id: KitIdLike): string | undefined {
    const kitIds = this.yDoc.getMap<string>("kitIds");
    return kitIds.get(this.key.kit(id));
  }

  private getYKit(id: KitIdLike): YKit {
    const kits = this.yDoc.getMap<YKit>("kits");
    const kitUuid = this.getKitUuid(id);
    if (!kitUuid) throw new Error(`Kit (${JSON.stringify(kitIdLikeToKitId(id))}) not found`);
    const yKit = kits.get(kitUuid) as YKit | undefined;
    if (!yKit) throw new Error(`Kit (${kitUuid}) not found`);
    return yKit;
  }

  private getYTypes(id: KitIdLike): YTypeMap {
    return gKit(this.getYKit(id), "types");
  }

  private getTypeUuid(kitId: KitIdLike, id: TypeIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const typeIds = yKit.get("typeIds") as Y.Map<string>;
    return typeIds.get(this.key.type(id));
  }

  private getYType(kitId: KitIdLike, id: TypeIdLike): YType {
    const yKit = this.getYKit(kitId);
    const types = yKit.get("types") as Y.Map<YType>;
    const uuid = this.getTypeUuid(kitId, id);
    const yType = uuid ? (types.get(uuid) as YType | undefined) : undefined;
    if (!yType) throw new Error(`Type (${JSON.stringify(typeIdLikeToTypeId(id))}) not found in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    return yType;
  }

  private getDesignUuid(kitId: KitIdLike, id: DesignIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const designIds = yKit.get("designIds") as Y.Map<string>;
    return designIds.get(this.key.design(id));
  }

  private getYDesign(kitId: KitIdLike, id: DesignIdLike): YDesign {
    const yKit = this.getYKit(kitId);
    const designs = yKit.get("designs") as Y.Map<YDesign>;
    const uuid = this.getDesignUuid(kitId, id);
    const yDesign = uuid ? (designs.get(uuid) as YDesign | undefined) : undefined;
    if (!yDesign) throw new Error(`Design (${JSON.stringify(designIdLikeToDesignId(id))}) not found in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    return yDesign;
  }

  private getYDesigns(kitId: KitIdLike): YDesignMap {
    return gKit(this.getYKit(kitId), "designs");
  }

  private getYPiece(kitId: KitIdLike, designId: DesignIdLike, id: PieceIdLike): YPiece {
    const yDesign = this.getYDesign(kitId, designId);
    const yPieces = gDesign(yDesign, "pieces");
    const yPiece = yPieces.get(this.key.piece(id));
    if (!yPiece) throw new Error(`Piece (${JSON.stringify(pieceIdLikeToPieceId(id))}) not found`);
    return yPiece;
  }

  private getYConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): YConnection {
    const yDesign = this.getYDesign(kitId, designId);
    const yConnections = gDesign(yDesign, "connections");
    const yConn = yConnections.get(this.key.connection(id));
    if (!yConn) throw new Error("Connection not found");
    return yConn;
  }

  private getYPorts(kitId: KitIdLike, typeId: TypeIdLike): YPortMap {
    return gType(this.getYType(kitId, typeId), "ports");
  }

  private getYPort(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike): YPort {
    const yPort = this.getYPorts(kitId, typeId).get(this.key.port(id));
    if (!yPort) throw new Error("Port not found");
    return yPort;
  }

  private getYRepresentations(kitId: KitIdLike, typeId: TypeIdLike): YRepresentationMap {
    return gType(this.getYType(kitId, typeId), "representations");
  }

  private getYRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): YRepresentation {
    const rep = this.getYRepresentations(kitId, typeId).get(this.key.representation(id));
    if (!rep) throw new Error("Representation not found");
    return rep;
  }

  onKitIdsChange(callback: () => void) {
    const kits = this.yDoc.getMap<YKit>("kits") as unknown as Y.Map<any>;
    const kitIds = this.yDoc.getMap<string>("kitIds") as unknown as Y.Map<any>;
    const o1 = () => callback();
    const o2 = () => callback();
    kits.observe(o1);
    kitIds.observe(o2);
    return () => {
      kits.unobserve(o1);
      kitIds.unobserve(o2);
    };
  }

  onKitChange(id: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(id);
    const o = () => callback();
    (yKit as unknown as Y.Map<any>).observe(o);
    return () => (yKit as unknown as Y.Map<any>).unobserve(o);
  }

  onTypesChange(id: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(id);
    const yTypes = yKit.get("types") as Y.Map<any>;
    const observer = () => callback();
    yTypes.observe(observer);
    return () => yTypes.unobserve(observer);
  }

  onTypeChange(kitId: KitIdLike, id: TypeIdLike, callback: () => void) {
    const yType = this.getYType(kitId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yType.observe(o);
    return () => yType.unobserve(o);
  }

  onDesignsChange(kitId: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(kitId);
    const yDesigns = yKit.get("designs") as Y.Map<any>;
    const observer = () => callback();
    yDesigns.observe(observer);
    return () => yDesigns.unobserve(observer);
  }

  onDesignChange(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yDesign.observe(o);
    return () => yDesign.unobserve(o);
  }

  onPiecesChange(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id);
    const yPieces = gDesign(yDesign, "pieces") as unknown as Y.Map<any>;
    const o = () => callback();
    yPieces.observe(o);
    return () => yPieces.unobserve(o);
  }

  onPieceChange(kitId: KitIdLike, id: DesignIdLike, pieceId: PieceIdLike, callback: () => void) {
    const yPiece = this.getYPiece(kitId, id, pieceId) as unknown as Y.Map<any>;
    const o = () => callback();
    yPiece.observe(o);
    return () => yPiece.unobserve(o);
  }

  onConnectionsChange(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id);
    const yConnections = gDesign(yDesign, "connections") as unknown as Y.Map<any>;
    const o = () => callback();
    yConnections.observe(o);
    return () => yConnections.unobserve(o);
  }

  onConnectionChange(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike, callback: () => void) {
    const yConn = this.getYConnection(kitId, designId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yConn.observe(o);
    return () => yConn.unobserve(o);
  }

  onPortsChange(kitId: KitIdLike, typeId: TypeIdLike, callback: () => void) {
    const yPorts = this.getYPorts(kitId, typeId) as unknown as Y.Map<any>;
    const o = () => callback();
    yPorts.observe(o);
    return () => yPorts.unobserve(o);
  }

  onPortChange(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike, callback: () => void) {
    const yPort = this.getYPort(kitId, typeId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yPort.observe(o);
    return () => yPort.unobserve(o);
  }

  onRepresentationsChange(kitId: KitIdLike, typeId: TypeIdLike, callback: () => void) {
    const yReps = this.getYRepresentations(kitId, typeId) as unknown as Y.Map<any>;
    const o = () => callback();
    yReps.observe(o);
    return () => yReps.unobserve(o);
  }

  onRepresentationChange(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike, callback: () => void) {
    const yRep = this.getYRepresentation(kitId, typeId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yRep.observe(o);
    return () => yRep.unobserve(o);
  }

  onQualitiesChange(kitId: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(kitId);
    const yQualities = yKit.get("qualities") as Y.Array<any>;
    const observer = () => callback();
    yQualities.observe(observer);
    return () => yQualities.unobserve(observer);
  }

  private createAuthor(author: Author): YAuthor {
    const yAuthor = new Y.Map<string>();
    yAuthor.set("name", author.name);
    yAuthor.set("email", author.email || "");
    return yAuthor;
  }

  private createAuthors(authors: Author[] | undefined): YAuthors {
    const yAuthors = new Y.Map<YAuthor>();
    (authors || []).forEach((a) => yAuthors.set(a.name, this.createAuthor(a)));
    return yAuthors;
  }

  private getAuthors(yAuthors: YAuthors | undefined): Author[] {
    if (!yAuthors) return [];
    const authors: Author[] = [];
    yAuthors.forEach((yAuthor: YAuthor) => authors.push({ name: yAuthor.get("name") as string, email: (yAuthor.get("email") as string) || "" }));
    return authors;
  }

  private createQuality(quality: Quality): YQuality {
    const yMap = new Y.Map<string>();
    yMap.set("name", quality.name);
    if (quality.value !== undefined) yMap.set("value", quality.value);
    if (quality.unit !== undefined) yMap.set("unit", quality.unit);
    if (quality.definition !== undefined) yMap.set("definition", quality.definition);
    return yMap;
  }

  private createQualities(qualities: Quality[] | undefined): YQualities {
    const yArr = new Y.Array<YQuality>();
    (qualities || []).forEach((q) => yArr.push([this.createQuality(q)]));
    return yArr;
  }

  private getQualities(yArr: YQualities | undefined): Quality[] {
    if (!yArr) return [];
    const list: Quality[] = [];
    yArr.forEach((yMap: YQuality) => {
      list.push({ name: yMap.get("name") as string, value: yMap.get("value") as string | undefined, unit: yMap.get("unit") as string | undefined, definition: yMap.get("definition") as string | undefined });
    });
    return list;
  }

  private buildYRepresentation(rep: Representation): YRepresentation {
    const yRep: YRepresentation = new Y.Map<YRepresentationVal>();
    yRep.set("url", rep.url);
    yRep.set("description", rep.description || "");
    const yTags = new Y.Array<string>();
    (rep.tags || []).forEach((t) => yTags.push([t]));
    yRep.set("tags", yTags);
    yRep.set("qualities", this.createQualities(rep.qualities || []));
    return yRep;
  }

  private buildYPort(port: Port): YPort {
    const yPort: YPort = new Y.Map<YPortVal>();
    yPort.set("id_", port.id_ || "");
    yPort.set("description", port.description || "");
    yPort.set("mandatory", port.mandatory === undefined ? false : port.mandatory);
    yPort.set("family", port.family || "");
    const yCompatibleFamilies = new Y.Array<string>();
    (port.compatibleFamilies || []).forEach((f) => yCompatibleFamilies.push([f]));
    yPort.set("compatibleFamilies", yCompatibleFamilies);
    const yDirection = new Y.Map<number>();
    yDirection.set("x", port.direction.x);
    yDirection.set("y", port.direction.y);
    yDirection.set("z", port.direction.z);
    yPort.set("direction", yDirection);
    const yPoint = new Y.Map<number>();
    yPoint.set("x", port.point.x);
    yPoint.set("y", port.point.y);
    yPoint.set("z", port.point.z);
    yPort.set("point", yPoint);
    yPort.set("t", port.t || 0);
    yPort.set("qualities", this.createQualities(port.qualities || []));
    return yPort;
  }

  private buildYPiece(piece: Piece): YPiece {
    const yPiece: YPiece = new Y.Map<YPieceVal>();
    yPiece.set("id_", piece.id_ || uuidv4());
    yPiece.set("description", piece.description || "");
    const yType = new Y.Map<string>();
    yType.set("name", piece.type.name);
    yType.set("variant", piece.type.variant || "");
    yPiece.set("type", yType);
    if (piece.plane) {
      const yPlane = new Y.Map<YVec3>();
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
      yPiece.set("plane", yPlane);
    }
    if (piece.center) {
      const yCenter = new Y.Map<number>();
      yCenter.set("x", piece.center.x);
      yCenter.set("y", piece.center.y);
      yPiece.set("center", yCenter);
    }
    yPiece.set("qualities", this.createQualities(piece.qualities) || []);
    return yPiece;
  }

  private buildYConnection(connection: Connection): YConnection {
    const yConnection: YConnection = new Y.Map<YConnectionVal>();
    const yConnectedSide = new Y.Map<YLeafMapString>();
    const yConnectedSidePiece = new Y.Map<string>();
    yConnectedSidePiece.set("id_", connection.connected.piece.id_ || "");
    const yConnectedSidePort = new Y.Map<string>();
    yConnectedSidePort.set("id_", connection.connected.port.id_ || "");
    yConnectedSide.set("piece", yConnectedSidePiece);
    yConnectedSide.set("port", yConnectedSidePort);
    yConnection.set("connected", yConnectedSide);
    const yConnectingSide = new Y.Map<YLeafMapString>();
    const yConnectingSidePiece = new Y.Map<string>();
    yConnectingSidePiece.set("id_", connection.connecting.piece.id_ || "");
    const yConnectingSidePort = new Y.Map<string>();
    yConnectingSidePort.set("id_", connection.connecting.port.id_ || "");
    yConnectingSide.set("piece", yConnectingSidePiece);
    yConnectingSide.set("port", yConnectingSidePort);
    yConnection.set("connecting", yConnectingSide);
    yConnection.set("description", connection.description || "");
    yConnection.set("gap", connection.gap || 0);
    yConnection.set("shift", connection.shift || 0);
    yConnection.set("rise", connection.rise || 0);
    yConnection.set("rotation", connection.rotation || 0);
    yConnection.set("turn", connection.turn || 0);
    yConnection.set("tilt", connection.tilt || 0);
    yConnection.set("x", connection.x || 0);
    yConnection.set("y", connection.y || 0);
    yConnection.set("qualities", this.createQualities(connection.qualities));
    return yConnection;
  }

  createKit(kit: Kit): void {
    KitSchema.parse(kit);
    if (!kit.name) throw new Error("Kit name is required to create a kit.");
    const kits = this.yDoc.getMap<YKit>("kits");
    const kitIds = this.yDoc.getMap<string>("kitIds");
    const compound = this.key.kit(kit);
    const kitId = kitIdLikeToKitId(kit);
    if (kitIds.has(compound)) throw new Error(`Kit (${kitId.name}, ${kitId.version || ""}) already exists.`);
    const kitUuid = uuidv4();
    const yKit: YKit = new Y.Map<YKitVal>();
    yKit.set("name", kit.name);
    yKit.set("description", kit.description || "");
    yKit.set("icon", kit.icon || "");
    yKit.set("image", kit.image || "");
    yKit.set("version", kit.version || "");
    yKit.set("preview", kit.preview || "");
    yKit.set("remote", kit.remote || "");
    yKit.set("homepage", kit.homepage || "");
    yKit.set("license", kit.license || "");
    yKit.set("types", new Y.Map<YType>());
    yKit.set("designs", new Y.Map<YDesign>());
    yKit.set("typeIds", new Y.Map<string>());
    yKit.set("designIds", new Y.Map<string>());
    yKit.set("qualities", this.createQualities(kit.qualities));
    yKit.set("created", new Date().toISOString());
    yKit.set("updated", new Date().toISOString());
    kits.set(kitUuid, yKit);
    kitIds.set(compound, kitUuid);
    kit.types?.forEach((t) => this.createType(kit, t));
    kit.designs?.forEach((d) => this.createDesign(kit, d));
  }

  getKit(id: KitIdLike): Kit {
    const yKit = this.getYKit(id);
    const yTypesMap = gKit(yKit, "types");
    const typeIds = gKit(yKit, "typeIds");
    const types = yTypesMap
      ? Array.from(typeIds.keys()).map((compound) => {
          const [typeName, typeVariant] = compound.split("::");
          return this.getType(id, { name: typeName, variant: typeVariant || undefined });
        })
      : [];
    const yDesignsMap = gKit(yKit, "designs");
    const designIds = gKit(yKit, "designIds");
    const designs = yDesignsMap
      ? Array.from(designIds.keys()).map((compound) => {
          const [dName, dVariant, dView] = compound.split("::");
          return this.getDesign(id, { name: dName, variant: dVariant || undefined, view: dView || undefined });
        })
      : [];
    return {
      name: gKit(yKit, "name"),
      description: gKit(yKit, "description"),
      icon: gKit(yKit, "icon"),
      image: gKit(yKit, "image"),
      version: gKit(yKit, "version"),
      preview: gKit(yKit, "preview"),
      remote: gKit(yKit, "remote"),
      homepage: gKit(yKit, "homepage"),
      license: gKit(yKit, "license"),
      created: new Date(gKit(yKit, "created")),
      updated: new Date(gKit(yKit, "updated")),
      designs,
      types,
      qualities: this.getQualities(gKit(yKit, "qualities")),
    };
  }

  updateKit(kit: Kit): Kit {
    const yKit = this.getYKit(kit);

    if (kit.description !== undefined) yKit.set("description", kit.description);
    if (kit.icon !== undefined) yKit.set("icon", kit.icon);
    if (kit.image !== undefined) yKit.set("image", kit.image);
    if (kit.preview !== undefined) yKit.set("preview", kit.preview);
    if (kit.remote !== undefined) yKit.set("remote", kit.remote);
    if (kit.homepage !== undefined) yKit.set("homepage", kit.homepage);
    if (kit.license !== undefined) yKit.set("license", kit.license); // Assuming direct set works, adjust if Y.Array

    // Updating nested structures (types, designs, qualities) is complex here.
    // Recommend using specific update methods like updateType, updateDesign.
    if (kit.qualities !== undefined) {
      yKit.set("qualities", this.createQualities(kit.qualities));
    }

    yKit.set("updated", new Date().toISOString());
    return this.getKit(kit);
  }

  deleteKit(id: KitId): void {
    const kits = this.yDoc.getMap<YKit>("kits");
    const kitIds = this.yDoc.getMap<string>("kitIds");
    const compound = this.key.kit(id);
    const kitUuid = kitIds.get(compound);
    if (!kitUuid) throw new Error(`Kit (${id}) not found, cannot delete.`);
    kits.delete(kitUuid);
    kitIds.delete(compound);
  }

  createType(kitId: KitIdLike, type: Type): void {
    TypeSchema.parse(type);
    const yKit = this.getYKit(kitId);
    const types = gKit(yKit, "types");
    const typeIds = gKit(yKit, "typeIds");
    const compound = this.key.type(type);
    const kitIdNormalized = kitIdLikeToKitId(kitId);
    if (typeIds.has(compound)) throw new Error(`Type (${type.name}, ${type.variant || ""}) already exists in kit (${JSON.stringify(kitIdNormalized)})`);
    const yType: YType = new Y.Map<YTypeVal>();
    yType.set("name", type.name);
    yType.set("description", type.description || "");
    yType.set("icon", type.icon || "");
    yType.set("image", type.image || "");
    yType.set("variant", type.variant || "");
    yType.set("stock", type.stock || Number("Infinity"));
    yType.set("virtual", type.virtual || false);
    yType.set("unit", type.unit);
    yType.set("representations", new Y.Map<YRepresentation>());
    yType.set("ports", new Y.Map<YPort>());
    yType.set("authors", this.createAuthors(type.authors));
    yType.set("qualities", this.createQualities(type.qualities) || []);
    const typeUuid = uuidv4();
    types.set(typeUuid, yType);
    typeIds.set(compound, typeUuid);
    type.representations?.map((r) => {
      this.createRepresentation(kitId, type, r);
    });
    type.ports?.map((p) => {
      this.createPort(kitId, type, p);
    });
    yType.set("created", new Date().toISOString());
    yType.set("updated", new Date().toISOString());
  }

  getType(kitId: KitIdLike, id: TypeIdLike): Type {
    const yType = this.getYType(kitId, id);

    const yRepresentationsMap = gType(yType, "representations");
    const representations = yRepresentationsMap
      ? Array.from(yRepresentationsMap.values())
          .map((rMap) => this.getRepresentation(kitId, id, { tags: gRep(rMap, "tags")?.toArray() || [] }))
          .filter((r): r is Representation => r !== null)
      : [];
    const yPortsMap = gType(yType, "ports");
    const ports = yPortsMap
      ? Array.from(yPortsMap.values())
          .map((pMap) => this.getPort(kitId, id, { id_: gPort(pMap, "id_") }))
          .filter((p): p is Port => p !== null)
      : [];
    return {
      name: gType(yType, "name"),
      description: gType(yType, "description"),
      icon: gType(yType, "icon"),
      image: gType(yType, "image"),
      variant: gType(yType, "variant"),
      stock: gType(yType, "stock"),
      virtual: gType(yType, "virtual"),
      unit: gType(yType, "unit"),
      ports,
      representations,
      qualities: this.getQualities(gType(yType, "qualities")),
      authors: this.getAuthors(gType(yType, "authors")),
      created: new Date(gType(yType, "created")),
      updated: new Date(gType(yType, "updated")),
    };
  }

  updateType(kitId: KitId, type: Type): Type {
    const yType = this.getYType(kitId, type);

    if (type.description !== undefined) yType.set("description", type.description);
    if (type.icon !== undefined) yType.set("icon", type.icon);
    if (type.image !== undefined) yType.set("image", type.image);
    if (type.stock !== undefined) yType.set("stock", type.stock);
    if (type.virtual !== undefined) yType.set("virtual", type.virtual);
    if (type.unit !== undefined) yType.set("unit", type.unit);

    if (type.ports !== undefined) {
      const validPorts = type.ports.filter((p) => p.id_ !== undefined);
      const portsMap = new Y.Map<YPort>();
      validPorts.forEach((p) => portsMap.set(p.id_!, this.buildYPort(p)));
      yType.set("ports", portsMap);
    }
    if (type.qualities !== undefined) {
      yType.set("qualities", this.createQualities(type.qualities));
    }
    if (type.representations !== undefined) {
      const reps = new Y.Map<YRepresentation>();
      type.representations.forEach((r) => reps.set(`${r.tags?.join(",") || ""}`, this.buildYRepresentation(r)));
      yType.set("representations", reps);
    }
    if (type.authors !== undefined) {
      yType.set("authors", this.createAuthors(type.authors));
    }

    yType.set("updated", new Date().toISOString());
    return this.getType(kitId, type);
  }

  deleteType(kitId: KitId, id: TypeId): void {
    const yKit = this.getYKit(kitId);
    const types = gKit(yKit, "types");
    const typeIds = yKit.get("typeIds") as Y.Map<string>;
    const compound = this.key.type(id);
    const uuid = typeIds.get(compound);
    if (!uuid) throw new Error(`Type (${id}) not found in kit (${kitId})`);
    types.delete(uuid);
    typeIds.delete(compound);
  }

  getKits(): Map<string, string[]> {
    const kitsMap = new Map<string, string[]>();
    const kitIds = this.yDoc.getMap<string>("kitIds");
    kitIds.forEach((_, compound) => {
      const [name, version] = (compound as string).split("::");
      const arr = kitsMap.get(name) || [];
      arr.push(version || "");
      kitsMap.set(name, arr);
    });
    return kitsMap;
  }

  getTypes(kitId: KitId): Type[] {
    const yKit = this.getYKit(kitId);
    const typeIds = gKit(yKit, "typeIds");
    return Array.from(typeIds.keys()).map((compound) => {
      const [name, variant] = (compound as string).split("::");
      return this.getType(kitId, typeIdLikeToTypeId([name, variant || ""]));
    });
  }

  getDesigns(kitId: KitId): Design[] {
    const yKit = this.getYKit(kitId);
    const designIds = gKit(yKit, "designIds");
    return Array.from(designIds.keys()).map((compound) => {
      const [name, variant, view] = (compound as string).split("::");
      return this.getDesign(kitId, designIdLikeToDesignId([name, variant || "", view || ""]));
    });
  }

  getPieces(kitId: KitId, designId: DesignId): Piece[] {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces: Piece[] = [];
    const yPieces = gDesign(yDesign, "pieces");
    if (yPieces) {
      yPieces.forEach((_, pieceId) => {
        try {
          pieces.push(this.getPiece(kitId, designId, pieceId as unknown as string));
        } catch {}
      });
    }
    return pieces;
  }

  getConnections(kitId: KitId, designId: DesignId): Connection[] {
    const yDesign = this.getYDesign(kitId, designId);
    const list: Connection[] = [];
    const yConnections = gDesign(yDesign, "connections");
    if (yConnections) {
      yConnections.forEach((_, id) => {
        const [connectedPieceId, connectingPieceId] = (id as string).split("--");
        if (connectedPieceId && connectingPieceId) {
          try {
            list.push(this.getConnection(kitId, designId, connectionIdLikeToConnectionId({ connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } })));
          } catch {}
        }
      });
    }
    return list;
  }

  getRepresentations(kitId: KitId, typeId: TypeId): Representation[] {
    const yType = this.getYType(kitId, typeId);
    const result: Representation[] = [];
    const yRepresentations = gType(yType, "representations");
    if (yRepresentations) {
      yRepresentations.forEach((yRep) => {
        const tags = gRep(yRep, "tags")?.toArray() || [];
        try {
          result.push(this.getRepresentation(kitId, typeId, tags));
        } catch {}
      });
    }
    return result;
  }

  getPorts(kitId: KitId, typeId: TypeId): Port[] {
    const yType = this.getYType(kitId, typeId) as YType;
    const ports: Port[] = [];
    const yPorts = gType(yType, "ports");
    if (yPorts) {
      yPorts.forEach((_, portId) => {
        try {
          ports.push(this.getPort(kitId, typeId, portId as unknown as string));
        } catch {}
      });
    }
    return ports;
  }

  createDesign(kitId: KitId, design: Design): void {
    DesignSchema.parse(design);
    const yKit = this.getYKit(kitId);
    const designs = gKit(yKit, "designs");
    const designIds = gKit(yKit, "designIds");
    const compound = this.key.design(designIdLikeToDesignId(design));
    if (designIds.has(compound)) throw new Error(`Design (${design.name}, ${design.variant || ""}, ${design.view || ""}) already exists in kit (${kitId})`);
    const yDesign: YDesign = new Y.Map<YDesignVal>();
    yDesign.set("name", design.name);
    yDesign.set("description", design.description || "");
    yDesign.set("icon", design.icon || "");
    yDesign.set("image", design.image || "");
    yDesign.set("variant", design.variant || "");
    yDesign.set("view", design.view || "");
    yDesign.set("unit", design.unit || "");
    yDesign.set("pieces", new Y.Map<YPiece>());
    yDesign.set("connections", new Y.Map<YConnection>());
    yDesign.set("qualities", this.createQualities(design.qualities) || []);
    yDesign.set("authors", this.createAuthors(design.authors));
    const designUuid = uuidv4();
    designs.set(designUuid, yDesign);
    designIds.set(compound, designUuid);
    design.pieces?.forEach((p: Piece) => this.createPiece(kitId, designIdLikeToDesignId(design), p));
    design.connections?.forEach((c: Connection) => this.createConnection(kitId, designIdLikeToDesignId(design), c));
    yDesign.set("created", new Date().toISOString());
    yDesign.set("updated", new Date().toISOString());
  }

  getDesign(kitId: KitIdLike, id: DesignIdLike): Design {
    const yDesign = this.getYDesign(kitId, id);

    const yPieces = gDesign(yDesign, "pieces");
    const pieces = yPieces
      ? Array.from(yPieces.values())
          .map((pMap) => this.getPiece(kitId, id, { id_: gPiece(pMap, "id_") }))
          .filter((p): p is Piece => p !== null)
      : [];
    const yConnections = gDesign(yDesign, "connections");
    const connections = yConnections
      ? Array.from(yConnections.values())
          .map((cMap) => {
            const connectedPieceId = gSide(gConn(cMap, "connected"), "piece").get("id_") as string;
            const connectingPieceId = gSide(gConn(cMap, "connecting"), "piece").get("id_") as string;
            return this.getConnection(kitId, id, { connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } });
          })
          .filter((c): c is Connection => c !== null)
      : [];
    return {
      name: gDesign(yDesign, "name"),
      description: gDesign(yDesign, "description"),
      icon: gDesign(yDesign, "icon"),
      image: gDesign(yDesign, "image"),
      variant: gDesign(yDesign, "variant"),
      view: gDesign(yDesign, "view"),
      unit: gDesign(yDesign, "unit"),
      created: new Date(gDesign(yDesign, "created")),
      updated: new Date(gDesign(yDesign, "updated")),
      authors: this.getAuthors(gDesign(yDesign, "authors")),
      pieces,
      connections,
      qualities: this.getQualities(gDesign(yDesign, "qualities")),
    };
  }

  updateDesign(kitId: KitIdLike, design: Design): Design {
    const yDesign = this.getYDesign(kitId, design);

    if (design.description !== undefined) yDesign.set("description", design.description);
    if (design.icon !== undefined) yDesign.set("icon", design.icon);
    if (design.image !== undefined) yDesign.set("image", design.image);
    if (design.unit !== undefined) yDesign.set("unit", design.unit);

    if (design.pieces !== undefined) {
      const validPieces = design.pieces.filter((p: Piece) => p.id_ !== undefined);
      const piecesMap = new Y.Map<YPiece>();
      validPieces.forEach((p: Piece) => piecesMap.set(p.id_!, this.buildYPiece(p)));
      yDesign.set("pieces", piecesMap);
    }
    if (design.connections !== undefined) {
      const validConnections = design.connections.filter((c: Connection) => c.connected?.piece?.id_ && c.connecting?.piece?.id_ && c.connected?.port?.id_ && c.connecting?.port?.id_);
      const getConnectionId = (c: Connection) => `${c.connected.piece.id_}--${c.connecting.piece.id_}`;
      const connectionsMap = new Y.Map<YConnection>();
      validConnections.forEach((c: Connection) => connectionsMap.set(getConnectionId(c), this.buildYConnection(c)));
      yDesign.set("connections", connectionsMap);
    }
    if (design.qualities !== undefined) {
      yDesign.set("qualities", this.createQualities(design.qualities));
    }

    return this.getDesign(kitId, design);
  }

  deleteDesign(kitId: KitId, id: DesignId): void {
    const yKit = this.getYKit(kitId);
    const designs = gKit(yKit, "designs");
    const designIds = gKit(yKit, "designIds");
    const compound = this.key.design(id);
    const uuid = designIds.get(compound);
    if (!uuid) throw new Error(`Design (${id}) not found in kit (${kitId})`);
    designs.delete(uuid);
    designIds.delete(compound);
  }

  createPiece(kitId: KitId, designId: DesignId, piece: Piece): void {
    PieceSchema.parse(piece);
    const yDesign = this.getYDesign(kitId, designId);
    const yPieces = gDesign(yDesign, "pieces");
    yPieces.set(piece.id_, this.buildYPiece(piece));
  }

  getPiece(kitId: KitIdLike, designId: DesignIdLike, id: PieceIdLike): Piece {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = gDesign(yDesign, "pieces");
    const yPiece = pieces.get(this.key.piece(id));
    const pieceId = pieceIdLikeToPieceId(id);
    if (!yPiece) throw new Error(`Piece (${JSON.stringify(pieceId)}) not found in design (${JSON.stringify(designIdLikeToDesignId(designId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);

    const type = gPiece(yPiece, "type");
    const typeName = type.get("name") as string;
    const typeVariant = (type.get("variant") as string) || "";

    const yPlane = gPiece(yPiece, "plane") as YPlane | undefined;
    const yOrigin = yPlane?.get("origin") as YVec3 | undefined;
    const yXAxis = yPlane?.get("xAxis") as YVec3 | undefined;
    const yYAxis = yPlane?.get("yAxis") as YVec3 | undefined;
    const origin: Point | null = yOrigin
      ? {
          x: yOrigin.get("x") as number,
          y: yOrigin.get("y") as number,
          z: yOrigin.get("z") as number,
        }
      : null;
    const xAxis: Vector | null = yXAxis
      ? {
          x: yXAxis.get("x") as number,
          y: yXAxis.get("y") as number,
          z: yXAxis.get("z") as number,
        }
      : null;
    const yAxis: Vector | null = yYAxis
      ? {
          x: yYAxis.get("x") as number,
          y: yYAxis.get("y") as number,
          z: yYAxis.get("z") as number,
        }
      : null;
    const plane: Plane | null =
      origin && xAxis && yAxis
        ? {
            origin,
            xAxis,
            yAxis,
          }
        : null;

    const yCenter = gPiece(yPiece, "center") as YLeafMapNumber | undefined;
    const center: DiagramPoint | null = yCenter
      ? {
          x: yCenter.get("x") as number,
          y: yCenter.get("y") as number,
        }
      : null;

    return {
      id_: gPiece(yPiece, "id_"),
      description: gPiece(yPiece, "description"),
      type: { name: typeName, variant: typeVariant },
      plane: plane ?? undefined,
      center: center ?? undefined,
      qualities: this.getQualities(gPiece(yPiece, "qualities")),
    };
  }

  updatePiece(kitId: KitId, designId: DesignId, piece: Piece): Piece {
    if (!piece.id_) throw new Error("Piece ID is required for update.");
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = gDesign(yDesign, "pieces");
    const yPiece = pieces.get(piece.id_);
    if (!yPiece) throw new Error(`Piece ${piece.id_} not found in design ${designId} in kit ${kitId}`);

    if (piece.description !== undefined) yPiece.set("description", piece.description);
    if (piece.type !== undefined) {
      const yType = new Y.Map<string>();
      yType.set("name", piece.type.name);
      yType.set("variant", piece.type.variant || "");
      yPiece.set("type", yType);
    }
    if (piece.center !== undefined && piece.center !== null) {
      const yCenter = new Y.Map<number>();
      yCenter.set("x", piece.center.x);
      yCenter.set("y", piece.center.y);
      yPiece.set("center", yCenter);
    }
    if (piece.plane !== undefined && piece.plane !== null) {
      const yPlane = new Y.Map<YVec3>();
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
      yPiece.set("plane", yPlane);
    }

    return this.getPiece(kitId, designId, piece.id_);
  }

  deletePiece(kitId: KitId, designId: DesignId, id: PieceId): boolean {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = gDesign(yDesign, "pieces");
    const existed = pieces.has(id.id_);
    pieces.delete(id.id_);
    return existed;
  }

  createConnection(kitId: KitId, designId: DesignId, connection: Connection): void {
    ConnectionSchema.parse(connection);
    const yDesign = this.getYDesign(kitId, designId);
    if (!yDesign) throw new Error(`Design(${designId}) not found in kit(${kitId})`);
    const connectionId = `${connection.connected.piece.id_}--${connection.connecting.piece.id_}`;
    const reverseConnectionId = `${connection.connecting.piece.id_}--${connection.connected.piece.id_}`;
    const connections = gDesign(yDesign, "connections");
    if (connections.get(connectionId) || connections.get(reverseConnectionId)) {
      throw new Error(`Connection (${connectionId}) already exists in design (${designId}) in kit (${kitId})`);
    }
    connections.set(connectionId, this.buildYConnection(connection));
  }

  getConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): Connection {
    const yDesign = this.getYDesign(kitId, designId);
    const connections = gDesign(yDesign, "connections");
    const yConnection = connections.get(this.key.connection(id));
    const connectionId = connectionIdLikeToConnectionId(id);
    if (!yConnection) throw new Error(`Connection (${JSON.stringify(connectionId)}) not found in design (${JSON.stringify(designIdLikeToDesignId(designId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    const yConnected = gConn(yConnection, "connected");
    const connectedSide: Side = {
      piece: { id_: gSide(yConnected, "piece").get("id_") || "" },
      port: { id_: gSide(yConnected, "port").get("id_") || "" },
    };
    const yConnecting = gConn(yConnection, "connecting");
    const connectingSide: Side = {
      piece: { id_: gSide(yConnecting, "piece").get("id_") || "" },
      port: { id_: gSide(yConnecting, "port").get("id_") || "" },
    };
    return {
      description: gConn(yConnection, "description"),
      connected: connectedSide,
      connecting: connectingSide,
      gap: gConn(yConnection, "gap"),
      shift: gConn(yConnection, "shift"),
      rise: gConn(yConnection, "rise"),
      rotation: gConn(yConnection, "rotation"),
      turn: gConn(yConnection, "turn"),
      tilt: gConn(yConnection, "tilt"),
      x: gConn(yConnection, "x"),
      y: gConn(yConnection, "y"),
      qualities: this.getQualities(gConn(yConnection, "qualities")),
    };
  }

  updateConnection(kitId: KitIdLike, designId: DesignIdLike, connection: Partial<Connection>): void {
    if (!connection.connected?.piece?.id_ || !connection.connecting?.piece?.id_) {
      throw new Error("Connected and connecting piece IDs are required for update.");
    }
    const id = `${connection.connected.piece.id_}--${connection.connecting.piece.id_}`;
    const yDesign = this.getYDesign(kitId, designId);
    if (!yDesign) throw new Error(`Design (${designId}) not found in kit (${kitId})`);
    const connections = gDesign(yDesign, "connections");
    const yConnection = connections.get(id);
    if (!yConnection) throw new Error(`Connection (${id}) not found in design (${designId}) in kit (${kitId})`);

    if (connection.description !== undefined) yConnection.set("description", connection.description);
    if (connection.connected !== undefined) {
      const ySide = new Y.Map<YLeafMapString>();
      const yPiece = new Y.Map<string>();
      yPiece.set("id_", connection.connected?.piece.id_ || "");
      const yPort = new Y.Map<string>();
      yPort.set("id_", connection.connected?.port.id_ || "");
      ySide.set("piece", yPiece);
      ySide.set("port", yPort);
      yConnection.set("connected", ySide);
    }
    if (connection.connecting !== undefined) {
      const ySide = new Y.Map<YLeafMapString>();
      const yPiece = new Y.Map<string>();
      yPiece.set("id_", connection.connecting?.piece.id_ || "");
      const yPort = new Y.Map<string>();
      yPort.set("id_", connection.connecting?.port.id_ || "");
      ySide.set("piece", yPiece);
      ySide.set("port", yPort);
      yConnection.set("connecting", ySide);
    }
    if (connection.gap !== undefined) yConnection.set("gap", connection.gap);
    if (connection.rotation !== undefined) yConnection.set("rotation", connection.rotation);
    if (connection.shift !== undefined) yConnection.set("shift", connection.shift);
    if (connection.tilt !== undefined) yConnection.set("tilt", connection.tilt);
    if (connection.x !== undefined) yConnection.set("x", connection.x);
    if (connection.y !== undefined) yConnection.set("y", connection.y);

    const yQualities = (gConn(yConnection, "qualities") as YQualities | undefined) || new Y.Array<YQuality>();
    if (connection.qualities) {
      yQualities.delete(0, yQualities.length);
      connection.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
    }
    yConnection.set("qualities", yQualities);
  }

  deleteConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): void {
    const yDesign = this.getYDesign(kitId, designId);
    const connections = gDesign(yDesign, "connections");
    connections.delete(this.key.connection(id));
  }

  createRepresentation(kitId: KitIdLike, typeId: TypeIdLike, representation: Representation): void {
    RepresentationSchema.parse(representation);
    const yType = this.getYType(kitId, typeId);
    const representations = gType(yType, "representations");
    const id = representation.tags?.join(",") || "";
    representations.set(id, this.buildYRepresentation(representation));
  }

  getRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): Representation {
    const yType = this.getYType(kitId, typeId);
    const representations = gType(yType, "representations");
    const yRepresentation = representations.get(this.key.representation(id));
    const repId = representationIdLikeToRepresentationId(id);
    if (!yRepresentation) throw new Error(`Representation (${JSON.stringify(repId)}) not found in type (${JSON.stringify(typeIdLikeToTypeId(typeId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    return {
      url: gRep(yRepresentation, "url"),
      description: gRep(yRepresentation, "description"),
      tags: gRep(yRepresentation, "tags").toArray(),
      qualities: this.getQualities(gRep(yRepresentation, "qualities")),
    };
  }

  updateRepresentation(kitId: KitId, typeId: TypeId, representation: Partial<Representation>): void {
    const yType = this.getYType(kitId, typeId);
    const representations = gType(yType, "representations");
    const id = `${representation.tags?.join(",") || ""}`;
    const yRepresentation = representations.get(id);
    if (!yRepresentation) throw new Error(`Representation (${id}) not found in type (${typeId}) in kit (${kitId})`);

    if (representation.description !== undefined) yRepresentation.set("description", representation.description);
    if (representation.tags !== undefined) {
      const yTags = Y.Array.from(representation.tags || []);
      yRepresentation.set("tags", yTags);
    }
    if (representation.qualities !== undefined) {
      const yQualities = gRep(yRepresentation, "qualities") || new Y.Array<YQuality>();
      yQualities.delete(0, yQualities.length);
      representation.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      yRepresentation.set("qualities", yQualities);
    }
  }

  deleteRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): void {
    const yType = this.getYType(kitId, typeId);
    const representations = gType(yType, "representations");
    representations.delete(this.key.representation(id));
  }

  createPort(kitId: KitIdLike, typeId: TypeIdLike, port: Port): void {
    PortSchema.parse(port);
    const yType = this.getYType(kitId, typeId);

    const ports = gType(yType, "ports");
    const yPort: YPort = new Y.Map<YPortVal>();
    const pid = port.id_ || "";
    yPort.set("id_", pid);
    yPort.set("description", port.description || "");
    yPort.set("mandatory", port.mandatory === undefined ? false : port.mandatory);
    yPort.set("family", port.family || "");

    const yCompatibleFamilies = new Y.Array<string>();
    (port.compatibleFamilies || []).forEach((f) => yCompatibleFamilies.push([f]));
    yPort.set("compatibleFamilies", yCompatibleFamilies);

    const yDirection = new Y.Map<number>();
    yDirection.set("x", port.direction.x);
    yDirection.set("y", port.direction.y);
    yDirection.set("z", port.direction.z);
    yPort.set("direction", yDirection);

    const yPoint = new Y.Map<number>();
    yPoint.set("x", port.point.x);
    yPoint.set("y", port.point.y);
    yPoint.set("z", port.point.z);
    yPort.set("point", yPoint);

    yPort.set("t", port.t || 0);
    yPort.set("qualities", this.createQualities(port.qualities || []));

    ports.set(pid, yPort);
  }

  getPort(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike): Port {
    const yType = this.getYType(kitId, typeId) as YType;
    const ports = gType(yType, "ports");
    const yPort = ports.get(this.key.port(id));
    const portId = portIdLikeToPortId(id);
    if (!yPort) throw new Error(`Port (${JSON.stringify(portId)}) not found in type (${JSON.stringify(typeIdLikeToTypeId(typeId))})`);

    const yDirection = gPort(yPort, "direction");
    if (!yDirection) throw new Error(`Direction not found in port (${JSON.stringify(portId)})`);
    const yPoint = gPort(yPort, "point");
    if (!yPoint) throw new Error(`Point not found in port (${JSON.stringify(portId)})`);
    return {
      id_: gPort(yPort, "id_"),
      description: gPort(yPort, "description"),
      mandatory: gPort(yPort, "mandatory"),
      family: gPort(yPort, "family"),
      compatibleFamilies: gPort(yPort, "compatibleFamilies")?.toArray() || [],
      direction: {
        x: yDirection.get("x") as number,
        y: yDirection.get("y") as number,
        z: yDirection.get("z") as number,
      },
      point: {
        x: yPoint.get("x") as number,
        y: yPoint.get("y") as number,
        z: yPoint.get("z") as number,
      },
      t: gPort(yPort, "t"),
      qualities: this.getQualities(gPort(yPort, "qualities")),
    };
  }

  updatePort(kitId: KitId, typeId: TypeId, portId: string, port: Partial<Port>): void {
    const yType = this.getYType(kitId, typeId);
    const ports = gType(yType, "ports");
    const yPort = ports.get(portId);
    if (!yPort) throw new Error(`Port (${portId}) not found in type (${typeId})`);

    if (port.description !== undefined) yPort.set("description", port.description);
    if (port.direction !== undefined) {
      const yDirection = new Y.Map<number>();
      yDirection.set("x", port.direction.x);
      yDirection.set("y", port.direction.y);
      yDirection.set("z", port.direction.z);
      yPort.set("direction", yDirection);
    }
    if (port.point !== undefined) {
      const yPoint = new Y.Map<number>();
      yPoint.set("x", port.point.x);
      yPoint.set("y", port.point.y);
      yPoint.set("z", port.point.z);
      yPort.set("point", yPoint);
    }
    if (port.t !== undefined) yPort.set("t", port.t);
    if (port.qualities !== undefined) {
      const yQualities = (yPort.get("qualities") as YQualities | undefined) || new Y.Array<YQuality>();
      yQualities.delete(0, yQualities.length);
      port.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      yPort.set("qualities", yQualities);
    }
    if (port.family !== undefined && yPort.set("family", port.family)) {
      const cf = yPort.get("compatibleFamilies") as YStringArray;
      cf.delete(0, cf.length);
      (port.compatibleFamilies || []).forEach((fam) => cf.push([fam]));
    }
    if (port.mandatory !== undefined && yPort.set("mandatory", port.mandatory)) {
      const cf = yPort.get("compatibleFamilies") as YStringArray;
      cf.delete(0, cf.length);
      (port.compatibleFamilies || []).forEach((fam) => cf.push([fam]));
    }
  }

  deletePort(kitId: KitId, typeId: TypeId, portId: string): void {
    const yType = this.getYType(kitId, typeId);
    const ports = gType(yType, "ports");
    ports.delete(portId);
  }

  createFile(url: string, data: Uint8Array): void {
    this.yDoc.getMap<Uint8Array>("files").set(url, data);
    const ab = new ArrayBuffer(data.byteLength);
    new Uint8Array(ab).set(new Uint8Array(data));
    const blob = new Blob([ab]);
    const blobUrl = URL.createObjectURL(blob);
    this.fileUrls.set(url, blobUrl);
  }

  getFileUrl(url: string): string {
    const fileUrl = this.fileUrls.get(url);
    if (!fileUrl) throw new Error(`File (${url}) not found`);
    return fileUrl;
  }

  getFileUrls(): Map<string, string> {
    return this.fileUrls;
  }

  getFileData(url: string): Uint8Array {
    const fileData = this.yDoc.getMap<Uint8Array>("files").get(url);
    if (!fileData) throw new Error(`File (${url}) not found`);
    return fileData as Uint8Array;
  }

  deleteFile(url: string): void {
    this.yDoc.getMap<Uint8Array>("files").delete(url);
    this.fileUrls.delete(url);
  }

  deleteFiles(): void {
    this.yDoc.getMap<Uint8Array>("files").clear();
    this.fileUrls.clear();
  }

  createDesignEditorStore(kitName: string, kitVersion: string, designName: string, designVariant: string, view: string): string {
    const yKit = this.getYKit(kitIdLikeToKitId([kitName, kitVersion]));
    const yDesign = this.getYDesign(kitIdLikeToKitId([kitName, kitVersion]), designIdLikeToDesignId([designName, designVariant, view]));
    const id = uuidv4();
    const undoManager = new UndoManager(yDesign, { captureTimeout: 0, trackedOrigins: new Set([id]) });
    const state: DesignEditorState = {
      designId: { name: gDesign(yDesign, "name"), variant: gDesign(yDesign, "variant"), view: gDesign(yDesign, "view") },
      fullscreenPanel: DesignEditorFullscreenPanel.None,
      selection: { selectedPieceIds: [], selectedConnections: [] },
      designDiff: { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as DesignEditorState["designDiff"],
      isTransactionActive: false,
      operationStack: [],
      operationIndex: -1,
    } as DesignEditorState;
    this.designEditors.set(id, { yKit, yDesign, undoManager, state, listeners: new Set() });
    return id;
  }

  getDesignEditorStore(id: string): {
    getState: () => DesignEditorState;
    setState: (s: DesignEditorState) => void;
    getDesignId: () => DesignId;
    getKitId: () => KitId;
    updateDesignEditorSelection: (selection: DesignEditorSelection) => void;
    deleteSelectedPiecesAndConnections: () => void;
    pushOperation: (undo: DesignDiff, redo: DesignDiff, selection: DesignEditorSelection) => void;
    invertDiff: (diff: DesignDiff) => DesignDiff;
    undo: () => void;
    redo: () => void;
    transact: (operations: () => void) => void;
    subscribe: (callback: () => void) => () => void;
  } | null {
    const entry = this.designEditors.get(id);
    if (!entry) return null;
    const getState = () => entry.state;
    const setState = (s: DesignEditorState) => {
      entry.state = s;
      entry.listeners.forEach((l) => l());
    };
    const getDesignId = (): DesignId => ({ name: gDesign(entry.yDesign, "name"), variant: gDesign(entry.yDesign, "variant"), view: gDesign(entry.yDesign, "view") });
    const getKitId = (): KitId => ({ name: gKit(entry.yKit, "name"), version: gKit(entry.yKit, "version") });
    const updateDesignEditorSelection = (selection: DesignEditorSelection) => setState({ ...getState(), selection });
    const ensureDiff = (d?: DesignDiff): DesignDiff => (d && d.pieces && d.connections ? d : { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } });
    const getPieceFromY = (pieceId: string): Piece => {
      const kid = getKitId();
      const did = getDesignId();
      return this.getPiece(kid, did, pieceIdLikeToPieceId(pieceId));
    };
    const getConnectionFromY = (connectedPieceId: string, connectingPieceId: string): Connection => {
      const kid = getKitId();
      const did = getDesignId();
      return this.getConnection(kid, did, connectionIdLikeToConnectionId({ connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } }));
    };
    const applyDiffToYDesign = (diff: DesignDiff) => {
      const safe = ensureDiff(diff);
      const yPieces = gDesign(entry.yDesign, "pieces");
      const yConnections = gDesign(entry.yDesign, "connections");
      // Pieces removed
      (safe.pieces?.removed || []).forEach((pid: any) => {
        const id = typeof pid === "string" ? pid : pid.id_;
        if (yPieces.has(id)) yPieces.delete(id);
      });
      // Pieces updated
      (safe.pieces?.updated || []).forEach((pd: any) => {
        const id = pd.id_;
        const yPiece = yPieces.get(id);
        if (!yPiece) return;
        if (pd.description !== undefined) (yPiece as any).set("description", pd.description);
        if (pd.type !== undefined && pd.type?.name) {
          const yType = new Y.Map<string>();
          yType.set("name", pd.type.name);
          yType.set("variant", pd.type.variant || "");
          (yPiece as any).set("type", yType);
        }
        if (pd.center !== undefined && pd.center !== null) {
          const yCenter = new Y.Map<number>();
          yCenter.set("x", pd.center.x);
          yCenter.set("y", pd.center.y);
          (yPiece as any).set("center", yCenter);
        }
        if (pd.plane !== undefined && pd.plane !== null) {
          const yPlane = new Y.Map<YVec3>();
          const yOrigin = new Y.Map<number>();
          yOrigin.set("x", pd.plane.origin.x);
          yOrigin.set("y", pd.plane.origin.y);
          yOrigin.set("z", pd.plane.origin.z);
          yPlane.set("origin", yOrigin);
          const yXAxis = new Y.Map<number>();
          yXAxis.set("x", pd.plane.xAxis.x);
          yXAxis.set("y", pd.plane.xAxis.y);
          yXAxis.set("z", pd.plane.xAxis.z);
          yPlane.set("xAxis", yXAxis);
          const yYAxis = new Y.Map<number>();
          yYAxis.set("x", pd.plane.yAxis.x);
          yYAxis.set("y", pd.plane.yAxis.y);
          yYAxis.set("z", pd.plane.yAxis.z);
          yPlane.set("yAxis", yYAxis);
          (yPiece as any).set("plane", yPlane);
        }
      });
      // Pieces added
      (safe.pieces?.added || []).forEach((p: Piece) => {
        if (!p.id_) return;
        yPieces.set(p.id_, this.buildYPiece(p));
      });
      // Connections removed
      (safe.connections?.removed || []).forEach((cid: any) => {
        const a = `${cid.connected?.piece?.id_}--${cid.connecting?.piece?.id_}`;
        const b = `${cid.connecting?.piece?.id_}--${cid.connected?.piece?.id_}`;
        if (gDesign(entry.yDesign, "connections").has(a)) (gDesign(entry.yDesign, "connections") as any).delete(a);
        else if (gDesign(entry.yDesign, "connections").has(b)) (gDesign(entry.yDesign, "connections") as any).delete(b);
      });
      // Connections updated
      (safe.connections?.updated || []).forEach((cd: any) => {
        const id = `${cd.connected?.piece?.id_}--${cd.connecting?.piece?.id_}`;
        const alt = `${cd.connecting?.piece?.id_}--${cd.connected?.piece?.id_}`;
        const yConn = (yConnections.get(id) || yConnections.get(alt)) as YConnection | undefined;
        if (!yConn) return;
        if (cd.description !== undefined) (yConn as any).set("description", cd.description);
        if (cd.connected !== undefined) {
          const ySide = new Y.Map<YLeafMapString>();
          const yPiece = new Y.Map<string>();
          yPiece.set("id_", cd.connected.piece.id_ || "");
          const yPort = new Y.Map<string>();
          yPort.set("id_", cd.connected.port?.id_ || "");
          ySide.set("piece", yPiece);
          ySide.set("port", yPort);
          (yConn as any).set("connected", ySide);
        }
        if (cd.connecting !== undefined) {
          const ySide = new Y.Map<YLeafMapString>();
          const yPiece = new Y.Map<string>();
          yPiece.set("id_", cd.connecting.piece.id_ || "");
          const yPort = new Y.Map<string>();
          yPort.set("id_", cd.connecting.port?.id_ || "");
          ySide.set("piece", yPiece);
          ySide.set("port", yPort);
          (yConn as any).set("connecting", ySide);
        }
        if (cd.gap !== undefined) (yConn as any).set("gap", cd.gap);
        if (cd.shift !== undefined) (yConn as any).set("shift", cd.shift);
        if (cd.rise !== undefined) (yConn as any).set("rise", cd.rise);
        if (cd.rotation !== undefined) (yConn as any).set("rotation", cd.rotation);
        if (cd.turn !== undefined) (yConn as any).set("turn", cd.turn);
        if (cd.tilt !== undefined) (yConn as any).set("tilt", cd.tilt);
        if (cd.x !== undefined) (yConn as any).set("x", cd.x);
        if (cd.y !== undefined) (yConn as any).set("y", cd.y);
      });
      // Connections added
      (safe.connections?.added || []).forEach((c: Connection) => {
        const id = `${c.connected.piece.id_}--${c.connecting.piece.id_}`;
        yConnections.set(id, this.buildYConnection(c));
      });
    };
    const invertDiffUsingCurrentY = (diff: DesignDiff): DesignDiff => {
      const safe = ensureDiff(diff);
      const inv: DesignDiff = { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } } as any;
      // pieces
      (safe.pieces?.added || []).forEach((p: Piece) => (inv.pieces!.removed = [...(inv.pieces!.removed || []), { id_: p.id_ } as any]));
      (safe.pieces?.removed || []).forEach((pid: any) => {
        const id = typeof pid === "string" ? pid : pid.id_;
        try {
          inv.pieces!.added = [...(inv.pieces!.added || []), getPieceFromY(id)];
        } catch {}
      });
      (safe.pieces?.updated || []).forEach((pd: any) => {
        const id = pd.id_;
        try {
          const current = getPieceFromY(id);
          const prev: any = { id_: id };
          if (current.description !== undefined) prev.description = current.description;
          if (current.type !== undefined) prev.type = current.type;
          if (current.center !== undefined) prev.center = current.center as any;
          if (current.plane !== undefined) prev.plane = current.plane as any;
          inv.pieces!.updated = [...(inv.pieces!.updated || []), prev];
        } catch {}
      });
      // connections
      (safe.connections?.added || []).forEach((c: Connection) => (inv.connections!.removed = [...(inv.connections!.removed || []), { connected: c.connected, connecting: c.connecting } as any]));
      (safe.connections?.removed || []).forEach((cid: any) => {
        const a = `${cid.connected?.piece?.id_}--${cid.connecting?.piece?.id_}`;
        const b = `${cid.connecting?.piece?.id_}--${cid.connected?.piece?.id_}`;
        const [cp, np] = a.includes("--") ? a.split("--") : [cid.connected?.piece?.id_, cid.connecting?.piece?.id_];
        try {
          inv.connections!.added = [...(inv.connections!.added || []), getConnectionFromY(cp, np)];
        } catch {
          try {
            inv.connections!.added = [...(inv.connections!.added || []), getConnectionFromY(np, cp)];
          } catch {}
        }
      });
      (safe.connections?.updated || []).forEach((cd: any) => {
        const cp = cd.connected?.piece?.id_;
        const np = cd.connecting?.piece?.id_;
        try {
          const current = getConnectionFromY(cp, np);
          inv.connections!.updated = [...(inv.connections!.updated || []), { ...current, connected: current.connected, connecting: current.connecting } as any];
        } catch {
          try {
            const current = getConnectionFromY(np, cp);
            inv.connections!.updated = [...(inv.connections!.updated || []), { ...current, connected: current.connected, connecting: current.connecting } as any];
          } catch {}
        }
      });
      return inv;
    };
    const deleteSelectedPiecesAndConnections = () => {
      const selection = getState().selection;
      const kitId = getKitId();
      const designId = getDesignId();
      const kit = this.getKit(kitId);
      const flatDesign = flattenDesign(kit, designId);
      const types = this.getTypes(kitId);
      const yConnections = gDesign(entry.yDesign, "connections");
      if (selection.selectedConnections.length > 0) {
        selection.selectedConnections.forEach((conn: { connectingPieceId: string; connectedPieceId: string }) => {
          const a = `${conn.connectedPieceId}--${conn.connectingPieceId}`;
          const b = `${conn.connectingPieceId}--${conn.connectedPieceId}`;
          if (yConnections.has(a)) yConnections.delete(a);
          else if (yConnections.has(b)) yConnections.delete(b);
        });
      }
      if (selection.selectedPieceIds.length > 0) {
        const yPieces = gDesign(entry.yDesign, "pieces");
        const toDelete: string[] = [];
        yConnections.forEach((_, cid) => {
          const [cA, cB] = (cid as string).split("--");
          if (selection.selectedPieceIds.includes(cA) || selection.selectedPieceIds.includes(cB)) toDelete.push(cid as string);
        });
        toDelete.forEach((cid) => yConnections.delete(cid));
        selection.selectedPieceIds.forEach((pid: string) => yPieces.delete(pid));
      }
      updateDesignEditorSelection({ selectedPieceIds: [], selectedConnections: [] });
    };
    const pushOperation = (undo: DesignDiff, redo: DesignDiff, selection: DesignEditorSelection) => {
      const undoDiff = ensureDiff(undo);
      const redoDiff = ensureDiff(redo);
      const s = getState();
      const base = s.operationIndex < s.operationStack.length - 1 ? s.operationStack.slice(0, s.operationIndex + 1) : s.operationStack;
      const nextStack = [...base, { undo: undoDiff, redo: redoDiff, selection }];
      setState({ ...s, operationStack: nextStack, operationIndex: nextStack.length - 1 });
    };
    const invertDiff = (diff: DesignDiff) => invertDiffUsingCurrentY(diff);
    const undo = () => {
      const s = getState();
      if (s.operationIndex < 0 || s.operationIndex >= s.operationStack.length) return;
      const op = s.operationStack[s.operationIndex];
      this.yDoc.transact(() => applyDiffToYDesign(op.undo), id);
      setState({ ...getState(), selection: op.selection, operationIndex: s.operationIndex - 1 });
    };
    const redo = () => {
      const s = getState();
      if (s.operationIndex + 1 >= s.operationStack.length) return;
      const op = s.operationStack[s.operationIndex + 1];
      this.yDoc.transact(() => applyDiffToYDesign(op.redo), id);
      setState({ ...getState(), selection: op.selection, operationIndex: s.operationIndex + 1 });
    };
    const transact = (operations: () => void) => {
      this.yDoc.transact(operations, id);
      entry.listeners.forEach((l) => l());
    };
    const subscribe = (callback: () => void) => {
      entry.listeners.add(callback);
      return () => entry.listeners.delete(callback);
    };
    return { getState, setState, getDesignId, getKitId, updateDesignEditorSelection, deleteSelectedPiecesAndConnections, pushOperation, invertDiff, undo, redo, transact, subscribe };
  }

  deleteDesignEditorStore(id: string): void {
    this.designEditors.delete(id);
  }

  undo(): void {
    this.undoManager.undo();
  }

  redo(): void {
    this.undoManager.redo();
  }

  transact(commands: () => void): void {
    this.yDoc.transact(commands, this.id || "local");
  }

  async importKit(url: string, complete = false): Promise<void> {
    let SQL: SqlJsStatic;
    let kitDb: Database;
    try {
      SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
      console.log("SQL.js initialized for import.");
    } catch (err) {
      console.error("Failed to initialize SQL.js for import:", err);
      throw new Error("SQL.js failed to initialize for import.");
    }
    const zipData = await fetch(url).then((res) => res.arrayBuffer());
    const zip = await JSZip.loadAsync(zipData);
    if (complete) {
      for (const fileEntry of Object.values(zip.files)) {
        const fileData = await fileEntry.async("uint8array");
        this.createFile(fileEntry.name, fileData);
      }
    }
    const kitDbFileEntry = zip.file(".semio/kit.db");
    if (!kitDbFileEntry) {
      throw new Error("kit.db not found in the zip file at path ./semio/kit.db");
    }
    const kitDbFile = await kitDbFileEntry.async("uint8array");
    kitDb = new SQL.Database(kitDbFile);
    let kit: Kit | null = null;

    try {
      const kitRes = kitDb.exec("SELECT uri, name, description, icon, image, preview, version, remote, homepage, license, created, updated FROM kit LIMIT 1");
      if (!kitRes || kitRes.length === 0 || !kitRes[0].values || kitRes[0].values.length === 0) throw new Error("Kit data not found in database.");
      const kitRow = kitRes[0].values[0] as SqlValue[];
      kit = {
        name: String(kitRow[1] || ""),
        description: (kitRow[2] as string) || undefined,
        icon: (kitRow[3] as string) || undefined,
        image: (kitRow[4] as string) || undefined,
        preview: (kitRow[5] as string) || undefined,
        version: (kitRow[6] as string) || undefined,
        remote: (kitRow[7] as string) || undefined,
        homepage: (kitRow[8] as string) || undefined,
        license: (kitRow[9] as string) || undefined,
        created: new Date(String(kitRow[10] || "")),
        updated: new Date(String(kitRow[11] || "")),
        types: [],
        designs: [],
        qualities: [],
      };
      if (!kit) throw new Error("Invalid kit row");
      const queryAll = (sql: string, params: SqlValue[] = []): SqlValue[][] => {
        const stmt = kitDb.prepare(sql);
        stmt.bind(params);
        const rows: SqlValue[][] = [];
        while (stmt.step()) rows.push(stmt.get());
        stmt.free();
        return rows;
      };
      const queryOne = (sql: string, params: SqlValue[] = []): SqlValue[] | null => {
        const rows = queryAll(sql, params);
        return rows.length > 0 ? rows[0] : null;
      };
      const kitIdRow = queryOne("SELECT id FROM kit WHERE name = ? AND version = ?", [kit.name, kit.version || ""]);
      const kitId = Number(kitIdRow ? (kitIdRow[0] as number) : 0);
      const getQualities = (fkColumn: string, fkValue: number | string): Quality[] => {
        const query = `SELECT name, value, unit, definition FROM quality WHERE ${fkColumn} = ?`;
        const rows = queryAll(query, [fkValue]);
        if (!rows || rows.length === 0) return [];
        return rows.map((row) => ({ name: String(row[0] || ""), value: (row[1] as string) || undefined, unit: (row[2] as string) || undefined, definition: (row[3] as string) || undefined }));
      };
      const getAuthors = (fkColumn: string, fkValue: number | string): Author[] => {
        const query = `SELECT name, email FROM author WHERE ${fkColumn} = ? ORDER BY rank`;
        const rows = queryAll(query, [fkValue]);
        if (!rows || rows.length === 0) return [];
        return rows.map((row) => ({ name: String(row[0] || ""), email: (row[1] as string) || undefined }) as Author);
      };
      kit.qualities = getQualities("kit_id", kitId);
      const typeRows = queryAll("SELECT id, name, description, icon, image, variant, stock, virtual, unit, created, updated, location_longitude, location_latitude FROM type WHERE kit_id = ?", [kitId]);
      if (typeRows && typeRows.length > 0) {
        for (const typeRow of typeRows) {
          const typeId = typeRow[0] as number as number;
          const type: Type = {
            name: String(typeRow[1] || ""),
            description: (typeRow[2] as string) || undefined,
            icon: (typeRow[3] as string) || undefined,
            image: (typeRow[4] as string) || undefined,
            variant: (typeRow[5] as string) || undefined,
            stock: (typeRow[6] as number) || undefined,
            virtual: Boolean(typeRow[7]),
            unit: String(typeRow[8] || ""),
            created: new Date(String(typeRow[9] || "")),
            updated: new Date(String(typeRow[10] || "")),
            location: {
              longitude: (typeRow[11] as number) ?? 0,
              latitude: (typeRow[12] as number) ?? 0,
            },
            representations: [],
            ports: [],
            qualities: [],
            authors: [],
          };
          const repRows = queryAll("SELECT id, url, description FROM representation WHERE type_id = ?", [typeId]);
          if (repRows && repRows.length > 0) {
            for (const repRow of repRows) {
              const representation: Representation = {
                url: String(repRow[1] || ""),
                description: (repRow[2] as string) || undefined,
                tags: [],
                qualities: [],
              };
              const repId = repRow[0] as number as number;
              const tagRows = queryAll('SELECT name FROM tag WHERE representation_id = ? ORDER BY "order"', [repId]);
              if (tagRows && tagRows.length > 0) representation.tags = tagRows.map((row) => (row as unknown[])[0] as string);
              representation.qualities = getQualities("representation_id", repId);
              if (!complete && !this.fileUrls.has(representation.url)) {
                const fileEntry = zip.file(representation.url);
                if (fileEntry) {
                  const fileData = await fileEntry.async("uint8array");
                  this.createFile(representation.url, fileData);
                } else if (complete && !representation.url.startsWith("http")) {
                  console.warn(`Representation file not found in zip: ${representation.url}`);
                }
              }
              type.representations!.push(representation);
            }
          }
          const portRows = queryAll("SELECT id, description, mandatory, family, t, local_id, point_x, point_y, point_z, direction_x, direction_y, direction_z FROM port WHERE type_id = ?", [typeId]);
          if (portRows && portRows.length > 0) {
            for (const portRow of portRows) {
              const port: Port = {
                description: (portRow[1] as string) || undefined,
                mandatory: Boolean(portRow[2]),
                family: (portRow[3] as string) || undefined,
                compatibleFamilies: [],
                t: Number(portRow[4] ?? 0),
                id_: (portRow[5] as string) || undefined,
                point: {
                  x: Number(portRow[6] ?? 0),
                  y: Number(portRow[7] ?? 0),
                  z: Number(portRow[8] ?? 0),
                },
                direction: {
                  x: Number(portRow[9] ?? 0),
                  y: Number(portRow[10] ?? 0),
                  z: Number(portRow[11] ?? 0),
                },
                qualities: [],
              };
              const portId = portRow[0] as number as number;
              const compFamRows = queryAll('SELECT name FROM compatible_family WHERE port_id = ? ORDER BY "order"', [portId]);
              if (compFamRows && compFamRows.length > 0) port.compatibleFamilies = compFamRows.map((row) => (row as unknown[])[0] as string);
              port.qualities = getQualities("port_id", portId);
              type.ports!.push(port);
            }
          }
          type.qualities = getQualities("type_id", typeId);
          type.authors = getAuthors("type_id", typeId);
          kit!.types!.push(type);
        }
      }
      const designRows = queryAll("SELECT id, name, description, icon, image, variant, view, unit, created, updated, location_longitude, location_latitude FROM design WHERE kit_id = ?", [kitId]);
      if (designRows && designRows.length > 0) {
        for (const designRow of designRows) {
          const design: Design = {
            name: String(designRow[1] || ""),
            description: (designRow[2] as string) || undefined,
            icon: (designRow[3] as string) || undefined,
            image: (designRow[4] as string) || undefined,
            variant: (designRow[5] as string) || undefined,
            view: (designRow[6] as string) || undefined,
            unit: String(designRow[7] || ""),
            created: new Date(String(designRow[8] || "")),
            updated: new Date(String(designRow[9] || "")),
            pieces: [],
            connections: [],
            qualities: [],
            authors: [],
          };
          const designId = designRow[0] as number as number;
          const pieceRows = queryAll(
            "SELECT p.id, p.local_id, p.description, t.name, t.variant, pl.origin_x, pl.origin_y, pl.origin_z, pl.x_axis_x, pl.x_axis_y, pl.x_axis_z, pl.y_axis_x, pl.y_axis_y, pl.y_axis_z, p.center_x, p.center_y FROM piece p JOIN type t ON p.type_id = t.id LEFT JOIN plane pl ON p.plane_id = pl.id WHERE p.design_id = ?",
            [designId],
          );
          const pieceMap: { [key: string]: Piece } = {};
          const pieceIdMap: { [dbId: number]: string } = {};
          if (pieceRows && pieceRows.length > 0) {
            for (const pieceRow of pieceRows) {
              const piece: Piece = {
                id_: String(pieceRow[1] || ""),
                description: (pieceRow[2] as string) || undefined,
                type: { name: String(pieceRow[3] || ""), variant: (pieceRow[4] as string) || "" },
                plane:
                  pieceRow[5] !== null
                    ? {
                        origin: {
                          x: (pieceRow[5] as number) ?? 0,
                          y: (pieceRow[6] as number) ?? 0,
                          z: (pieceRow[7] as number) ?? 0,
                        },
                        xAxis: {
                          x: (pieceRow[8] as number) ?? 0,
                          y: (pieceRow[9] as number) ?? 0,
                          z: (pieceRow[10] as number) ?? 0,
                        },
                        yAxis: {
                          x: (pieceRow[11] as number) ?? 0,
                          y: (pieceRow[12] as number) ?? 0,
                          z: (pieceRow[13] as number) ?? 0,
                        },
                      }
                    : undefined,
                center: pieceRow[14] !== null ? { x: (pieceRow[14] as number) ?? 0, y: (pieceRow[15] as number) ?? 0 } : undefined,
                qualities: [],
              };
              const pieceId = pieceRow[0] as number as number;
              piece.qualities = getQualities("piece_id", pieceId);
              design.pieces!.push(piece);
              pieceMap[piece.id_ as string] = piece;
              pieceIdMap[pieceId] = piece.id_;
            }
          }
          const connRows = queryAll(
            "SELECT c.id, c.description, c.gap, c.shift, c.rise, c.rotation, c.turn, c.tilt, c.x, c.y, c.connected_piece_id, cp.local_id AS connected_port_id, c.connecting_piece_id, cnp.local_id AS connecting_port_id FROM connection c JOIN port cp ON c.connected_port_id = cp.id JOIN port cnp ON c.connecting_port_id = cnp.id WHERE c.design_id = ?",
            [designId],
          );
          if (connRows && connRows.length > 0) {
            for (const connRow of connRows) {
              const connectedPieceLocalId = pieceIdMap[connRow[10] as number];
              const connectingPieceLocalId = pieceIdMap[connRow[12] as number];
              if (!connectedPieceLocalId || !connectingPieceLocalId) {
                console.warn(`Could not find piece local IDs for connection DB ID ${connRow[0]}`);
                continue;
              }
              const connection: Connection = {
                description: (connRow[1] as string) || undefined,
                gap: (connRow[2] as number) ?? 0,
                shift: (connRow[3] as number) ?? 0,
                rise: (connRow[4] as number) ?? 0,
                rotation: (connRow[5] as number) ?? 0,
                turn: (connRow[6] as number) ?? 0,
                tilt: (connRow[7] as number) ?? 0,
                x: (connRow[8] as number) ?? 0,
                y: (connRow[9] as number) ?? 0,
                connected: {
                  piece: { id_: connectedPieceLocalId },
                  port: { id_: String(connRow[11] || "") },
                },
                connecting: {
                  piece: { id_: connectingPieceLocalId },
                  port: { id_: String(connRow[13] || "") },
                },
                qualities: [],
              };
              const connId = connRow[0] as number as number;
              connection.qualities = getQualities("connection_id", connId);
              design.connections!.push(connection);
            }
          }
          design.qualities = getQualities("design_id", designId);
          design.authors = getAuthors("design_id", designId);
          kit!.designs!.push(design);
        }
      }
      if (!kit) throw new Error("No kit loaded");
      this.createKit(kit);
      console.log(`Kit "${kit.name}" imported successfully from ${url}`);
    } catch (error) {
      console.error("Error importing kit:", error);
      throw error;
    } finally {
      if (kitDb) {
        kitDb.close();
        console.log("SQL.js database closed for import.");
      }
    }
  }

  async exportKit(kitName: string, kitVersion: string, complete = false): Promise<Blob> {
    let SQL: SqlJsStatic;
    let db: Database;
    try {
      SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
      console.log("SQL.js initialized for export.");
    } catch (err) {
      console.error("Failed to initialize SQL.js for export:", err);
      throw new Error("SQL.js failed to initialize for export.");
    }
    const kit = this.getKit(kitIdLikeToKitId([kitName, kitVersion]));
    db = new SQL.Database();
    const zip = new JSZip();
    const schema = `
            CREATE TABLE kit ( uri VARCHAR(2048) NOT NULL UNIQUE, name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, preview VARCHAR(1024) NOT NULL, version VARCHAR(64) NOT NULL, remote VARCHAR(1024) NOT NULL, homepage VARCHAR(1024) NOT NULL, license VARCHAR(1024) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY );
            CREATE TABLE type ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
            CREATE TABLE design ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, "view" VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, UNIQUE (name, variant, "view", kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
            CREATE TABLE representation ( url VARCHAR(1024) NOT NULL, description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id) );
            CREATE TABLE tag ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id) );
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
      const insertQualities = (qualities: Quality[] | undefined, fkColumn: string, fkValue: number) => {
        if (!qualities) return;
        const stmt = db.prepare(`INSERT INTO quality (name, value, unit, definition, ${fkColumn}) VALUES (?, ?, ?, ?, ?)`);
        qualities.forEach((q) => stmt.run([q.name, q.value ?? "", q.unit ?? "", q.definition ?? "", fkValue]));
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
      kitStmt.run([
        `urn:kit:${kit.name}:${kit.version || ""}`,
        kit.name,
        kit.description || "",
        kit.icon || "",
        kit.image || "",
        kit.preview || "",
        kit.version || "",
        kit.remote || "",
        kit.homepage || "",
        kit.license || "",
        (kit.created || new Date(nowIso)).toISOString(),
        (kit.updated || new Date(nowIso)).toISOString(),
      ]);
      kitStmt.free();
      const kitId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
      insertQualities(kit.qualities, "kit_id", kitId);
      const typeIdMap: { [key: string]: number } = {}; // key: "name:variant"
      const portIdMap: { [typeDbId: number]: { [localId: string]: number } } = {};
      const repIdMap: { [typeDbId: number]: { [key: string]: number } } = {}; // key: "tags"
      if (kit.types) {
        const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, variant, unit, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const repStmt = db.prepare("INSERT INTO representation (url, description, type_id) VALUES (?, ?, ?)");
        const tagStmt = db.prepare('INSERT INTO tag (name, "order", representation_id) VALUES (?, ?, ?)');
        const portStmt = db.prepare("INSERT INTO port (local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const compFamStmt = db.prepare('INSERT INTO compatible_family (name, "order", port_id) VALUES (?, ?, ?)');
        for (const type of kit.types) {
          const typeKey = `${type.name}:${type.variant || ""}`;
          typeStmt.run([type.name, type.description || "", type.icon || "", type.image || "", type.variant || "", type.unit, (type.created || new Date(nowIso)).toISOString(), (type.updated || new Date(nowIso)).toISOString(), kitId]);
          const typeDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
          typeIdMap[typeKey] = typeDbId;
          portIdMap[typeDbId] = {};
          repIdMap[typeDbId] = {};
          insertQualities(type.qualities, "type_id", typeDbId);
          insertAuthors(type.authors, "type_id", typeDbId);
          if (type.representations) {
            for (const rep of type.representations) {
              const repKey = `${rep.tags?.join(",") || ""}`;
              repStmt.run([rep.url, rep.description ?? "", typeDbId]);
              const repDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
              repIdMap[typeDbId][repKey] = repDbId;
              insertQualities(rep.qualities, "representation_id", repDbId);
              if (rep.tags) {
                rep.tags.forEach((tag, index) => tagStmt.run([tag, index, repDbId]));
              }
              const fileData = this.getFileData(rep.url);
              if (fileData) {
                zip.file(rep.url, fileData);
              } else if (!complete && !rep.url.startsWith("http")) {
                console.warn(`File data for representation ${rep.url} not found in store.`);
              } else if (complete && !rep.url.startsWith("http")) {
                try {
                  const fetchedData = await fetch(rep.url).then((res) => res.arrayBuffer());
                  zip.file(rep.url, fetchedData);
                } catch (fetchErr) {
                  console.error(`Could not fetch representation ${rep.url} for complete export`, fetchErr);
                }
              }
            }
          }
          if (type.ports) {
            for (const port of type.ports) {
              if (!port.id_) {
                console.warn(`Skipping port without local_id in type ${type.name}:${type.variant}`);
                continue;
              }
              portStmt.run([
                port.id_,
                port.description ?? "",
                port.family ?? "",
                Number(port.t ?? 0),
                Number(port.point.x ?? 0),
                Number(port.point.y ?? 0),
                Number(port.point.z ?? 0),
                Number(port.direction.x ?? 0),
                Number(port.direction.y ?? 0),
                Number(port.direction.z ?? 0),
                typeDbId,
              ]);
              const portDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
              portIdMap[typeDbId][port.id_] = portDbId;
              insertQualities(port.qualities, "port_id", portDbId);
              if (port.compatibleFamilies) {
                port.compatibleFamilies.forEach((fam, index) => compFamStmt.run([fam, index, portDbId]));
              }
            }
          }
        }
        typeStmt.free();
        repStmt.free();
        tagStmt.free();
        portStmt.free();
        compFamStmt.free();
      }
      const designIdMap: { [key: string]: number } = {};
      const pieceIdMap: {
        [designDbId: number]: { [localId: string]: number };
      } = {};
      const planeIdMap: { [pieceDbId: number]: number } = {};
      let nextPlaneId = 1;
      if (kit.designs) {
        const designStmt = db.prepare('INSERT INTO design (name, description, icon, image, variant, "view", unit, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)');
        const planeStmt = db.prepare("INSERT INTO plane (id, origin_x, origin_y, origin_z, x_axis_x, x_axis_y, x_axis_z, y_axis_x, y_axis_y, y_axis_z) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const pieceStmt = db.prepare("INSERT INTO piece (local_id, description, type_id, plane_id, center_x, center_y, design_id) VALUES (?, ?, ?, ?, ?, ?, ?)");
        const connStmt = db.prepare(
          "INSERT INTO connection (description, gap, shift, rise, rotation, turn, tilt, x, y, connected_piece_id, connected_port_id, connecting_piece_id, connecting_port_id, design_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        );
        for (const design of kit.designs) {
          const designKey = `${design.name}:${design.variant || ""}:${design.view || ""}`;
          designStmt.run([
            design.name,
            design.description || "",
            design.icon || "",
            design.image || "",
            design.variant || "",
            design.view || "",
            design.unit,
            (design.created || new Date(nowIso)).toISOString(),
            (design.updated || new Date(nowIso)).toISOString(),
            kitId,
          ]);
          const designDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
          designIdMap[designKey] = designDbId;
          pieceIdMap[designDbId] = {};
          insertQualities(design.qualities, "design_id", designDbId);
          insertAuthors(design.authors, "design_id", designDbId);
          if (design.pieces) {
            for (const piece of design.pieces) {
              if (!piece.id_) {
                console.warn(`Skipping piece without local_id in design ${design.name}:${design.variant}:${design.view}`);
                continue;
              }
              const typeKey = `${piece.type.name}:${piece.type.variant || ""}`;
              const typeDbId = typeIdMap[typeKey];
              if (typeDbId === undefined) {
                console.warn(`Could not find type DB ID for piece ${piece.id_} (type: ${typeKey})`);
                continue;
              }
              let planeDbId: number | null = null;
              if (piece.plane) {
                planeDbId = nextPlaneId++;
                planeStmt.run([planeDbId, piece.plane.origin.x, piece.plane.origin.y, piece.plane.origin.z, piece.plane.xAxis.x, piece.plane.xAxis.y, piece.plane.xAxis.z, piece.plane.yAxis.x, piece.plane.yAxis.y, piece.plane.yAxis.z]);
              }
              pieceStmt.run([piece.id_, piece.description || "", typeDbId, planeDbId, piece.center?.x ?? null, piece.center?.y ?? null, designDbId]);
              const pieceDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
              pieceIdMap[designDbId][piece.id_] = pieceDbId;
              insertQualities(piece.qualities, "piece_id", pieceDbId);
              if (planeDbId !== null) {
                planeIdMap[pieceDbId] = planeDbId;
              }
            }
          }
          if (design.connections) {
            for (const conn of design.connections) {
              const connectedPieceDbId = pieceIdMap[designDbId][conn.connected.piece.id_];
              const connectingPieceDbId = pieceIdMap[designDbId][conn.connecting.piece.id_];
              const connectedPiece = design.pieces?.find((p: Piece) => p.id_ === conn.connected.piece.id_);
              const connectingPiece = design.pieces?.find((p: Piece) => p.id_ === conn.connecting.piece.id_);
              if (!connectedPieceDbId || !connectingPieceDbId || !connectedPiece || !connectingPiece) {
                console.warn(`Could not find piece DB IDs for connection between ${conn.connected.piece.id_} and ${conn.connecting.piece.id_}`);
                continue;
              }
              const connectedTypeKey = `${connectedPiece.type.name}:${connectedPiece.type.variant || ""}`;
              const connectingTypeKey = `${connectingPiece.type.name}:${connectingPiece.type.variant || ""}`;
              const connectedTypeDbId = typeIdMap[connectedTypeKey];
              const connectingTypeDbId = typeIdMap[connectingTypeKey];
              if (connectedTypeDbId === undefined || connectingTypeDbId === undefined) {
                console.warn(`Could not find type DB IDs for connection pieces`);
                continue;
              }
              const connectedPortDbId = conn.connected.port.id_ ? portIdMap[connectedTypeDbId]?.[conn.connected.port.id_] : undefined;
              const connectingPortDbId = conn.connecting.port.id_ ? portIdMap[connectingTypeDbId]?.[conn.connecting.port.id_] : undefined;
              if (connectedPortDbId === undefined || connectingPortDbId === undefined) {
                console.warn(`Could not find port DB IDs for connection between ${conn.connected.piece.id_}:${conn.connected.port.id_} and ${conn.connecting.piece.id_}:${conn.connecting.port.id_}`);
                continue;
              }
              connStmt.run([
                conn.description || "",
                Number(conn.gap ?? 0),
                Number(conn.shift ?? 0),
                Number(conn.rise ?? 0),
                Number(conn.rotation ?? 0),
                Number(conn.turn ?? 0),
                Number(conn.tilt ?? 0),
                Number(conn.x ?? 0),
                Number(conn.y ?? 0),
                connectedPieceDbId,
                connectedPortDbId,
                connectingPieceDbId,
                connectingPortDbId,
                designDbId,
              ]);
              const connDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
              insertQualities(conn.qualities, "connection_id", connDbId);
            }
          }
        }
        designStmt.free();
        planeStmt.free();
        pieceStmt.free();
        connStmt.free();
      }
      const dbData = db.export();
      zip.file(".semio/kit.db", dbData);
      const zipBlob = await zip.generateAsync({ type: "blob" });
      console.log(`Kit "${kit.name}" exported successfully.`);
      return zipBlob;
    } catch (error) {
      console.error("Error exporting kit:", error);
      throw error;
    } finally {
      if (db) {
        db.close();
        console.log("SQL.js database closed for export.");
      }
    }
  }
}

const __defaultStore = new SketchpadStore();
const __storeRegistry = new Map<string, SketchpadStore>();
const SketchpadContext = createContext<SketchpadStore>(__defaultStore);

// Scoping contexts
type SketchpadScope = { id: string };
type KitScope = { id: KitId };
type DesignScope = { id: DesignId };
type TypeScope = { id: TypeId };
type PieceScope = { id: PieceId };
type ConnectionScope = { id: ConnectionId };
type RepresentationScope = { id: RepresentationId };
type PortypeScope = { id: PortId };

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
const KitScopeContext = createContext<KitScope | null>(null);
const DesignScopeContext = createContext<DesignScope | null>(null);
const TypeScopeContext = createContext<TypeScope | null>(null);
const PieceScopeContext = createContext<PieceScope | null>(null);
const ConnectionScopeContext = createContext<ConnectionScope | null>(null);
const RepresentationScopeContext = createContext<RepresentationScope | null>(null);
const PortypeScopeContext = createContext<PortypeScope | null>(null);

export const SketchpadScopeProvider = (props: { id?: string; children: React.ReactNode }) => React.createElement(SketchpadScopeContext.Provider, { value: { id: props.id || "" } }, props.children as any);
export const KitScopeProvider = (props: { id?: KitId; children: React.ReactNode }) => React.createElement(KitScopeContext.Provider, { value: { id: props.id || { name: "", version: "" } } }, props.children as any);
export const DesignScopeProvider = (props: { id?: DesignId; children: React.ReactNode }) => React.createElement(DesignScopeContext.Provider, { value: { id: props.id || { name: "", variant: "", view: "" } } }, props.children as any);
export const TypeScopeProvider = (props: { id?: TypeId; children: React.ReactNode }) => React.createElement(TypeScopeContext.Provider, { value: { id: props.id || { name: "", variant: "" } } }, props.children as any);
export const PieceScopeProvider = (props: { id?: PieceId; children: React.ReactNode }) => React.createElement(PieceScopeContext.Provider, { value: { id: props.id || { id_: "" } } }, props.children as any);
export const ConnectionScopeProvider = (props: { id?: ConnectionId; children: React.ReactNode }) =>
  React.createElement(ConnectionScopeContext.Provider, { value: { id: props.id || { connected: { piece: { id_: "" } }, connecting: { piece: { id_: "" } } } } }, props.children as any);
export const RepresentationScopeProvider = (props: { id?: RepresentationId; children: React.ReactNode }) => React.createElement(RepresentationScopeContext.Provider, { value: { id: props.id || { tags: [] } } }, props.children as any);
export const PortypeScopeProvider = (props: { id?: PortId; children: React.ReactNode }) => React.createElement(PortypeScopeContext.Provider, { value: { id: props.id || { id_: "" } } }, props.children as any);

export const useSketchpadScope = () => useContext(SketchpadScopeContext);
export const useKitScope = () => useContext(KitScopeContext);
export const useDesignScope = () => useContext(DesignScopeContext);
export const useTypeScope = () => useContext(TypeScopeContext);
export const usePieceScope = () => useContext(PieceScopeContext);
export const useConnectionScope = () => useContext(ConnectionScopeContext);
export const useRepresentationScope = () => useContext(RepresentationScopeContext);
export const usePortScope = () => useContext(PortypeScopeContext);

export function SketchpadProvider(props: { id?: string; children: React.ReactNode }) {
  const ref = useRef<SketchpadStore | null>(null);
  if (!ref.current) ref.current = props.id ? __storeRegistry.get(props.id) || new SketchpadStore(props.id) : new SketchpadStore();
  if (props.id && !__storeRegistry.has(props.id)) __storeRegistry.set(props.id, ref.current);
  return React.createElement(SketchpadContext.Provider, { value: ref.current! }, props.children as any);
}

export function useSketchpadStore() {
  const ctx = useContext(SketchpadContext);
  const sScope = useSketchpadScope();
  const ref = useRef<SketchpadStore | null>(null);
  const finalId = sScope?.id;
  if (!finalId) return ctx || __defaultStore;
  if (!ref.current) ref.current = __storeRegistry.get(finalId) || new SketchpadStore(finalId);
  if (!__storeRegistry.has(finalId)) __storeRegistry.set(finalId, ref.current);
  return ref.current;
}

export function useKits() {
  const store = useSketchpadStore();
  return useSyncExternalStore(
    (l) => store.onKitIdsChange(l),
    () => store.getKits(),
  );
}

export function useKit(id?: KitId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitId = id ?? kitScope?.id;
  if (!kitId) throw new Error("useKit requires a kit id or must be inside a KitScope");
  return useSyncExternalStore(
    (l) => store.onKitChange(kitId, l),
    () => store.getKit(kitId),
  );
}

export function useDesigns() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useDesigns must be used within a KitScope");
  const { id } = kitScope;
  return useSyncExternalStore(
    (l) => store.onDesignsChange(id, l),
    () => store.getDesigns(id),
  );
}

export function useDesign(id?: DesignId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useDesign must be used within a KitScope");
  const designScope = useDesignScope();
  const designId = id ?? designScope?.id;
  if (!designId) throw new Error("useDesign requires a design id or must be inside a DesignScope");
  return useSyncExternalStore(
    (l) => store.onDesignChange(kitScope.id, designId, l),
    () => store.getDesign(kitScope.id, designId),
  );
}

export function useTypes() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useTypes must be used within a KitScope");
  const kitId = kitScope.id;
  return useSyncExternalStore(
    (l) => store.onTypesChange(kitId, l),
    () => store.getTypes(kitId),
  );
}

export function useType(id?: TypeId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useType must be used within a KitScope");
  const typeScope = useTypeScope();
  const typeId = id ?? typeScope?.id;
  if (!typeId) throw new Error("useType requires a type id or must be inside a TypeScope");
  return useSyncExternalStore(
    (l) => store.onTypeChange(kitScope.id, typeId, l),
    () => store.getType(kitScope.id, typeId),
  );
}

export function usePieces() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePieces must be used within a KitScope");
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePieces must be used within a DesignScope");
  return useSyncExternalStore(
    (l) => store.onPiecesChange(kitScope.id, designScope.id, l),
    () => store.getPieces(kitScope.id, designScope.id),
  );
}

export function usePiece(id?: PieceId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePiece must be used within a KitScope");
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePiece must be used within a DesignScope");
  const pieceScope = usePieceScope();
  const pieceId = id ?? pieceScope?.id;
  if (!pieceId) throw new Error("usePiece requires a piece id or must be inside a PieceScope");
  return useSyncExternalStore(
    (l) => store.onPieceChange(kitScope.id, designScope.id, pieceId, l),
    () => store.getPiece(kitScope.id, designScope.id, pieceId),
  );
}

export function useConnections() {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useConnections must be used within a KitScope");
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useConnections must be used within a DesignScope");
  return useSyncExternalStore(
    (l) => store.onConnectionsChange(kitScope.id, designScope.id, l),
    () => store.getConnections(kitScope.id, designScope.id),
  );
}

export function useConnection(id?: ConnectionId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useConnection must be used within a KitScope");
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useConnection must be used within a DesignScope");
  const connectionScope = useConnectionScope();
  const connectionId = id ?? connectionScope?.id;
  if (!connectionId) throw new Error("useConnection requires a connection id or must be inside a ConnectionScope");
  return useSyncExternalStore(
    (l) => store.onConnectionChange(kitScope.id, designScope.id, connectionId, l),
    () => store.getConnection(kitScope.id, designScope.id, connectionId),
  );
}

export function usePorts(id?: PortId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePorts must be used within a KitScope");
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("usePorts must be used within a TypeScope");
  return useSyncExternalStore(
    (l) => store.onPortsChange(kitScope.id, typeScope.id, l),
    () => store.getPorts(kitScope.id, typeScope.id),
  );
}

export function usePort(id?: PortId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePort must be used within a KitScope");
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("usePort must be used within a TypeScope");
  const portScope = usePortScope();
  const portId = id ?? portScope?.id;
  if (!portId) throw new Error("usePort requires a port id or must be inside a PortypeScope");
  return useSyncExternalStore(
    (l) => store.onPortChange(kitScope.id, typeScope.id, portId, l),
    () => store.getPort(kitScope.id, typeScope.id, portId),
  );
}

export function useRepresentations(id?: RepresentationId) {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useRepresentations must be used within a KitScope");
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("useRepresentations must be used within a TypeScope");
  return useSyncExternalStore(
    (l) => store.onRepresentationsChange(kitScope.id, typeScope.id, l),
    () => store.getRepresentations(kitScope.id, typeScope.id),
  );
}
