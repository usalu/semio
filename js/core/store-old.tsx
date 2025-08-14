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
import React, { createContext, useContext, useMemo } from "react";
import { v4 as uuidv4 } from "uuid";
import { IndexeddbPersistence } from "y-indexeddb";
import * as Y from "yjs";
// Import initSqlJs
import type { Database, SqlJsStatic, SqlValue } from "sql.js";
import initSqlJs from "sql.js";
import sqlWasmUrl from "sql.js/dist/sql-wasm.wasm?url";
import {
  applyDiff,
  Author,
  Camera,
  Connection,
  ConnectionDiff,
  ConnectionId,
  ConnectionIdLike,
  connectionIdLikeToConnectionId,
  Design,
  DesignDiff,
  DesignId,
  DesignIdLike,
  designIdLikeToDesignId,
  DiagramPoint,
  diff,
  getDiff,
  Kit,
  KitDiff,
  KitId,
  KitIdLike,
  kitIdLikeToKitId,
  Piece,
  PieceDiff,
  PieceId,
  PieceIdLike,
  pieceIdLikeToPieceId,
  Plane,
  Point,
  Port,
  PortDiff,
  PortId,
  PortIdLike,
  portIdLikeToPortId,
  Quality,
  QualityDiff,
  Representation,
  RepresentationDiff,
  RepresentationId,
  RepresentationIdLike,
  representationIdLikeToRepresentationId,
  Side,
  Type,
  TypeDiff,
  TypeId,
  TypeIdLike,
  typeIdLikeToTypeId,
  Vector,
} from "./semio";

// import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';

enum Mode {
  USER = "user",
  GUEST = "guest",
}

enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

enum Layout {
  NORMAL = "normal",
  TOUCH = "touch",
}

type DesignEditorStoreSelection = { selectedPieceIds: PieceId[]; selectedConnections: ConnectionId[]; selectedPiecePortId?: { pieceId: PieceId; portId: PortId } };
enum DesignEditorStoreFullscreenPanel {
  None = "none",
  Diagram = "diagram",
  Model = "model",
}
interface DesignEditorStorePresence {
  cursor?: DiagramPoint;
  camera?: Camera;
}
interface DesignEditorStorePresenceOther extends DesignEditorStorePresence {
  name: string;
}

interface DesignEditorStoreOperationStackEntry {
  undo: DesignDiff;
  redo: DesignDiff;
  selection: DesignEditorStoreSelection;
}
interface DesignEditorStoreState {
  designId: DesignId;
  fullscreenPanel: DesignEditorStoreFullscreenPanel;
  selection: DesignEditorStoreSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  operationStack: DesignEditorStoreOperationStackEntry[];
  operationIndex: number;
  presence: DesignEditorStorePresence;
  others?: DesignEditorStorePresenceOther[];
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

type YDesignEditorStoreVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YQualities | YStringArray;
type YDesignEditorStore = Y.Map<YDesignEditorStoreVal>;
type YDesignEditorStoreMap = Y.Map<YDesignEditorStore>;

type YIdMap = Y.Map<string>;
type YKitVal = string | YTypeMap | YDesignMap | YIdMap | YQualities;
type YKit = Y.Map<YKitVal>;

// Typed key maps and getters
type YKitKeysMap = {
  uuid: string;
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
  pieceIds: YIdMap;
  connectionIds: YIdMap;
  portIds: YIdMap;
  representationIds: YIdMap;
  qualities: YQualities;
};
const getKit = <K extends keyof YKitKeysMap>(m: YKit, k: K): YKitKeysMap[K] => m.get(k as string) as YKitKeysMap[K];

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
const getType = <K extends keyof YTypeKeysMap>(m: YType, k: K): YTypeKeysMap[K] => m.get(k as string) as YTypeKeysMap[K];

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
const getDesign = <K extends keyof YDesignKeysMap>(m: YDesign, k: K): YDesignKeysMap[K] => m.get(k as string) as YDesignKeysMap[K];

type YRepresentationKeysMap = {
  url: string;
  description: string;
  tags: YStringArray;
  qualities: YQualities;
};
const getRep = <K extends keyof YRepresentationKeysMap>(m: YRepresentation, k: K): YRepresentationKeysMap[K] => m.get(k as string) as YRepresentationKeysMap[K];

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
const getPort = <K extends keyof YPortKeysMap>(m: YPort, k: K): YPortKeysMap[K] => m.get(k as string) as YPortKeysMap[K];

type YPieceKeysMap = {
  id_: string;
  description: string;
  type: YLeafMapString; // { name, variant }
  plane: YPlane;
  center: YLeafMapNumber; // { x,y }
  qualities: YQualities;
};
const getPiece = <K extends keyof YPieceKeysMap>(m: YPiece, k: K): YPieceKeysMap[K] => m.get(k as string) as YPieceKeysMap[K];

type YSideKeysMap = {
  piece: YLeafMapString; // { id_ }
  port: YLeafMapString; // { id_ }
};
const getSide = <K extends keyof YSideKeysMap>(m: YSide, k: K): YSideKeysMap[K] => m.get(k as string) as YSideKeysMap[K];

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
const getConn = <K extends keyof YConnectionKeysMap>(m: YConnection, k: K): YConnectionKeysMap[K] => m.get(k as string) as YConnectionKeysMap[K];

type YDesignEditorStoreKeysMap = {
  fullscreenPanel: string;
  selectedPieceIds: YStringArray;
  selectedConnections: YStringArray;
  selectedPiecePortPieceId: string;
  selectedPiecePortPortId: string;
  designDiffPiecesAdded: YStringArray;
  designDiffPiecesRemoved: YStringArray;
  designDiffPiecesUpdated: YStringArray;
  designDiffConnectionsAdded: YStringArray;
  designDiffConnectionsRemoved: YStringArray;
  designDiffConnectionsUpdated: YStringArray;
  isTransactionActive: boolean;
  presenceCursorX: number;
  presenceCursorY: number;
  presenceCameraPositionX: number;
  presenceCameraPositionY: number;
  presenceCameraPositionZ: number;
  presenceCameraForwardX: number;
  presenceCameraForwardY: number;
  presenceCameraForwardZ: number;
};
const getDesignEditorStore = <K extends keyof YDesignEditorStoreKeysMap>(m: YDesignEditorStore, k: K): YDesignEditorStoreKeysMap[K] => m.get(k as string) as YDesignEditorStoreKeysMap[K];

// Sketchpad state interface
interface SketchpadState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
}

// Typed key maps for sketchpad state
type YSketchpadVal = string | boolean;
type YSketchpad = Y.Map<YSketchpadVal>;

type YSketchpadKeysMap = {
  mode: string;
  theme: string;
  layout: string;
  activeDesignEditorId: string;
};
const getSketchpadStore = <K extends keyof YSketchpadKeysMap>(m: YSketchpad, k: K): YSketchpadKeysMap[K] => m.get(k as string) as YSketchpadKeysMap[K];

class SketchpadStore {
  private id?: string;
  private sketchpadDoc: Y.Doc;
  private kitDocs: Map<string, Y.Doc> = new Map();
  private sketchpadIndexeddbProvider?: IndexeddbPersistence;
  private kitIndexeddbProviders: Map<string, IndexeddbPersistence> = new Map();
  private listeners: Set<() => void> = new Set();
  private fileUrls: Map<string, string> = new Map();
  private kitTransactionStacks: Map<string, KitDiff[]> = new Map();
  private designEditorTransactionStacks: Map<string, { kitDiffs: KitDiff[]; editorStates: any[] }> = new Map();

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
      return portId.id_;
    },
    representation: (id: RepresentationIdLike) => {
      const repId = representationIdLikeToRepresentationId(id);
      return repId.tags?.join(",") || "";
    },
  };

  constructor(id?: string) {
    this.sketchpadDoc = new Y.Doc();
    if (id) {
      this.id = id;
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad:${id}`, this.sketchpadDoc);
    }
    const ySketchpad = this.getYSketchpad();
    if (!ySketchpad.has("mode")) ySketchpad.set("mode", Mode.USER);
    if (!ySketchpad.has("theme")) ySketchpad.set("theme", Theme.SYSTEM);
    if (!ySketchpad.has("layout")) ySketchpad.set("layout", Layout.NORMAL);
    if (!ySketchpad.has("activeDesignEditorId")) ySketchpad.set("activeDesignEditorId", "");
    this.sketchpadDoc.getMap<YDesignEditorStore>("designEditors");
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private getYSketchpad(): YSketchpad {
    return this.sketchpadDoc.getMap<YSketchpadVal>("sketchpad");
  }

  private getKitDoc(kitId: KitIdLike): Y.Doc {
    const key = this.key.kit(kitId);
    let doc = this.kitDocs.get(key);
    if (!doc) {
      doc = new Y.Doc();
      this.kitDocs.set(key, doc);
      if (this.id) this.kitIndexeddbProviders.set(key, new IndexeddbPersistence(`semio-kit:${this.id}:${key}`, doc));
      doc.getMap<YKit>("kit");
      doc.getMap<Uint8Array>("files");
    }
    return doc;
  }

  private getYKit(id: KitIdLike): YKit {
    const doc = this.getKitDoc(id);
    const yKit = doc.getMap<YKitVal>("kit");

    // Initialize kit if empty
    if (yKit.size === 0) {
      const kitId = kitIdLikeToKitId(id);
      yKit.set("uuid", uuidv4());
      yKit.set("name", kitId.name);
      yKit.set("version", kitId.version || "");
      yKit.set("description", "");
      yKit.set("icon", "");
      yKit.set("image", "");
      yKit.set("preview", "");
      yKit.set("remote", "");
      yKit.set("homepage", "");
      yKit.set("license", "");
      yKit.set("created", new Date().toISOString());
      yKit.set("updated", new Date().toISOString());
      yKit.set("types", new Y.Map<YType>());
      yKit.set("designs", new Y.Map<YDesign>());
      yKit.set("typeIds", new Y.Map<string>());
      yKit.set("designIds", new Y.Map<string>());
      yKit.set("pieceIds", new Y.Map<string>());
      yKit.set("connectionIds", new Y.Map<string>());
      yKit.set("portIds", new Y.Map<string>());
      yKit.set("representationIds", new Y.Map<string>());
      yKit.set("qualities", new Y.Array<YQuality>());
    } else {
      // Ensure UUID exists for existing kits
      if (!yKit.has("uuid")) {
        yKit.set("uuid", uuidv4());
      }
      // Ensure UUID mappings exist for existing data
      this.ensureUuidMappings(yKit);
    }

    return yKit as YKit;
  }

  private getKitUuid(id: KitIdLike): string | undefined {
    const yKit = this.getYKit(id);
    let kitUuid = yKit.get("uuid") as string | undefined;

    // If no UUID exists, generate one (for legacy kits)
    if (!kitUuid) {
      kitUuid = uuidv4();
      yKit.set("uuid", kitUuid);
    }

    return kitUuid;
  }

  private getYTypes(id: KitIdLike): YTypeMap {
    return getKit(this.getYKit(id), "types");
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

  private getPieceUuid(kitId: KitIdLike, designId: DesignIdLike, id: PieceIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const pieceIds = yKit.get("pieceIds") as Y.Map<string>;
    const designKey = this.key.design(designId);
    const pieceKey = this.key.piece(id);
    return pieceIds.get(`${designKey}::${pieceKey}`);
  }

  private getConnectionUuid(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const connectionIds = yKit.get("connectionIds") as Y.Map<string>;
    const designKey = this.key.design(designId);
    const connectionKey = this.key.connection(id);
    return connectionIds.get(`${designKey}::${connectionKey}`);
  }

  private getPortUuid(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const portIds = yKit.get("portIds") as Y.Map<string>;
    const typeKey = this.key.type(typeId);
    const portKey = this.key.port(id);
    return portIds.get(`${typeKey}::${portKey}`);
  }

  private getRepresentationUuid(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): string | undefined {
    const yKit = this.getYKit(kitId);
    const representationIds = yKit.get("representationIds") as Y.Map<string>;
    const typeKey = this.key.type(typeId);
    const repKey = this.key.representation(id);
    return representationIds.get(`${typeKey}::${repKey}`);
  }

  private ensureUuidMappings(yKit: YKit): void {
    // Initialize UUID mapping tables if they don't exist
    if (!yKit.has("typeIds")) yKit.set("typeIds", new Y.Map<string>());
    if (!yKit.has("designIds")) yKit.set("designIds", new Y.Map<string>());
    if (!yKit.has("pieceIds")) yKit.set("pieceIds", new Y.Map<string>());
    if (!yKit.has("connectionIds")) yKit.set("connectionIds", new Y.Map<string>());
    if (!yKit.has("portIds")) yKit.set("portIds", new Y.Map<string>());
    if (!yKit.has("representationIds")) yKit.set("representationIds", new Y.Map<string>());
  }

  private updateTypeIdMapping(kitId: KitIdLike, oldId: TypeIdLike, newId: TypeIdLike): void {
    const yKit = this.getYKit(kitId);
    const typeIds = yKit.get("typeIds") as Y.Map<string>;
    const oldKey = this.key.type(oldId);
    const newKey = this.key.type(newId);
    const uuid = typeIds.get(oldKey);
    if (uuid) {
      typeIds.delete(oldKey);
      typeIds.set(newKey, uuid);
    }
  }

  private updateDesignIdMapping(kitId: KitIdLike, oldId: DesignIdLike, newId: DesignIdLike): void {
    const yKit = this.getYKit(kitId);
    const designIds = yKit.get("designIds") as Y.Map<string>;
    const oldKey = this.key.design(oldId);
    const newKey = this.key.design(newId);
    const uuid = designIds.get(oldKey);
    if (uuid) {
      designIds.delete(oldKey);
      designIds.set(newKey, uuid);
    }
  }

  private updatePieceIdMapping(kitId: KitIdLike, designId: DesignIdLike, oldId: PieceIdLike, newId: PieceIdLike): void {
    const yKit = this.getYKit(kitId);
    const pieceIds = yKit.get("pieceIds") as Y.Map<string>;
    const designKey = this.key.design(designId);
    const oldKey = `${designKey}::${this.key.piece(oldId)}`;
    const newKey = `${designKey}::${this.key.piece(newId)}`;
    const uuid = pieceIds.get(oldKey);
    if (uuid) {
      pieceIds.delete(oldKey);
      pieceIds.set(newKey, uuid);
    }
  }

  private updateConnectionIdMapping(kitId: KitIdLike, designId: DesignIdLike, oldId: ConnectionIdLike, newId: ConnectionIdLike): void {
    const yKit = this.getYKit(kitId);
    const connectionIds = yKit.get("connectionIds") as Y.Map<string>;
    const designKey = this.key.design(designId);
    const oldKey = `${designKey}::${this.key.connection(oldId)}`;
    const newKey = `${designKey}::${this.key.connection(newId)}`;
    const uuid = connectionIds.get(oldKey);
    if (uuid) {
      connectionIds.delete(oldKey);
      connectionIds.set(newKey, uuid);
    }
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
    return getKit(this.getYKit(kitId), "designs");
  }

  private getYPiece(kitId: KitIdLike, designId: DesignIdLike, id: PieceIdLike): YPiece {
    const yDesign = this.getYDesign(kitId, designId);
    const yPieces = getDesign(yDesign, "pieces");
    const yPiece = yPieces.get(this.key.piece(id));
    if (!yPiece) throw new Error(`Piece (${JSON.stringify(pieceIdLikeToPieceId(id))}) not found`);
    return yPiece;
  }

  private getYConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): YConnection {
    const yDesign = this.getYDesign(kitId, designId);
    const yConnections = getDesign(yDesign, "connections");
    const yConn = yConnections.get(this.key.connection(id));
    if (!yConn) throw new Error("Connection not found");
    return yConn;
  }

  private getYPorts(kitId: KitIdLike, typeId: TypeIdLike): YPortMap {
    return getType(this.getYType(kitId, typeId), "ports");
  }

  private getYPort(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike): YPort {
    const yPort = this.getYPorts(kitId, typeId).get(this.key.port(id));
    if (!yPort) throw new Error("Port not found");
    return yPort;
  }

  private getYRepresentations(kitId: KitIdLike, typeId: TypeIdLike): YRepresentationMap {
    return getType(this.getYType(kitId, typeId), "representations");
  }

  private getYRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): YRepresentation {
    const rep = this.getYRepresentations(kitId, typeId).get(this.key.representation(id));
    if (!rep) throw new Error("Representation not found");
    return rep;
  }

  private getYDesignEditorStores(): YDesignEditorStoreMap {
    return this.sketchpadDoc.getMap<YDesignEditorStore>("designEditors");
  }

  private getYDesignEditorStore(id: string): YDesignEditorStore {
    const designEditors = this.getYDesignEditorStores();
    const yDesignEditorStore = designEditors.get(id);
    if (!yDesignEditorStore) throw new Error(`Design editor (${id}) not found`);
    return yDesignEditorStore;
  }

  private createYDesignEditorStore(): YDesignEditorStore {
    const yDesignEditorStore = new Y.Map<YDesignEditorStoreVal>();
    yDesignEditorStore.set("fullscreenPanel", "none");
    yDesignEditorStore.set("selectedPieceIds", new Y.Array<string>());
    yDesignEditorStore.set("selectedConnections", new Y.Array<string>());
    yDesignEditorStore.set("selectedPiecePortPieceId", "");
    yDesignEditorStore.set("selectedPiecePortPortId", "");
    yDesignEditorStore.set("designDiffPiecesAdded", new Y.Array<string>());
    yDesignEditorStore.set("designDiffPiecesRemoved", new Y.Array<string>());
    yDesignEditorStore.set("designDiffPiecesUpdated", new Y.Array<string>());
    yDesignEditorStore.set("designDiffConnectionsAdded", new Y.Array<string>());
    yDesignEditorStore.set("designDiffConnectionsRemoved", new Y.Array<string>());
    yDesignEditorStore.set("designDiffConnectionsUpdated", new Y.Array<string>());
    yDesignEditorStore.set("isTransactionActive", false);
    yDesignEditorStore.set("presenceCursorX", 0);
    yDesignEditorStore.set("presenceCursorY", 0);
    yDesignEditorStore.set("presenceCameraPositionX", 0);
    yDesignEditorStore.set("presenceCameraPositionY", 0);
    yDesignEditorStore.set("presenceCameraPositionZ", 0);
    yDesignEditorStore.set("presenceCameraForwardX", 0);
    yDesignEditorStore.set("presenceCameraForwardY", 0);
    yDesignEditorStore.set("presenceCameraForwardZ", 0);
    return yDesignEditorStore;
  }

  onKitIdsChange(callback: () => void) {
    // Since kits are now managed per doc, we observe all kit docs
    const observers = new Map<string, () => void>();
    const cleanup = () => {
      observers.forEach((observer, key) => {
        const doc = this.kitDocs.get(key);
        if (doc) {
          (doc.getMap<YKitVal>("kit") as unknown as Y.Map<any>).unobserve(observer);
        }
      });
      observers.clear();
    };

    // Set up observers for existing kits
    this.kitDocs.forEach((doc, key) => {
      const observer = () => callback();
      (doc.getMap<YKitVal>("kit") as unknown as Y.Map<any>).observe(observer);
      observers.set(key, observer);
    });

    return cleanup;
  }

  // TODO: Make all observers specific to the property they are observing
  onDesignEditorStoreChange(id: string, callback: () => void) {
    const yDesignEditorStore = this.getYDesignEditorStore(id);
    const observer = () => {
      // Clear stable objects when store changes
      this.stableObjects.delete(`selection-${id}`);
      this.stableObjects.delete(`designDiff-${id}`);
      this.stableObjects.delete(`state-${id}`);
      callback();
    };
    yDesignEditorStore.observe(observer);
    return () => yDesignEditorStore.unobserve(observer);
  }

  // Helper to get stable object references for useSyncExternalStore
  private getStableObject<T>(key: string, factory: () => T): T {
    if (!this.stableObjects.has(key)) {
      this.stableObjects.set(key, factory());
    }
    return this.stableObjects.get(key)!;
  }

  // Get stable selection object for useSyncExternalStore
  getStableSelection(id: string): DesignEditorStoreSelection {
    return this.getStableObject(`selection-${id}`, () => {
      const designEditorStore = this.getDesignEditorStoreStore(id);
      return designEditorStore?.getState().selection ?? { selectedPieceIds: [], selectedConnections: [] };
    });
  }

  // Get stable designDiff object for useSyncExternalStore
  getStableDesignDiff(id: string): DesignDiff {
    return this.getStableObject(`designDiff-${id}`, () => {
      const designEditorStore = this.getDesignEditorStoreStore(id);
      return designEditorStore?.getState().designDiff ?? { pieces: { added: [], removed: [], updated: [] }, connections: { added: [], removed: [], updated: [] } };
    });
  }

  onDesignEditorStoreDesignIdChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStoreFullscreenPanelChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStoreSelectionChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStoreDesignDiffChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStoreIsTransactionActiveChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStorePresenceChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
  }

  onDesignEditorStorePresenceOthersChange(id: string, callback: () => void) {
    return this.onDesignEditorStoreChange(id, callback);
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
    // KitSchema.parse(kit);

    if (!kit.name) throw new Error("Kit name is required to create a kit.");

    const yKit = this.getYKit(kit);
    // Check if kit already exists by looking at name
    if (yKit.get("name") && yKit.get("name") !== "") {
      const kitId = kitIdLikeToKitId(kit);
      throw new Error(`Kit (${kitId.name}, ${kitId.version || ""}) already exists.`);
    }

    yKit.set("uuid", uuidv4());
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
    yKit.set("pieceIds", new Y.Map<string>());
    yKit.set("connectionIds", new Y.Map<string>());
    yKit.set("portIds", new Y.Map<string>());
    yKit.set("representationIds", new Y.Map<string>());
    yKit.set("qualities", this.createQualities(kit.qualities));
    yKit.set("created", new Date().toISOString());
    yKit.set("updated", new Date().toISOString());

    kit.types?.forEach((t) => this.createType(kit, t));
    kit.designs?.forEach((d) => this.createDesign(kit, d));
  }

  getKit(id: KitIdLike): Kit {
    const yKit = this.getYKit(id);
    const yTypesMap = getKit(yKit, "types");
    const typeIds = getKit(yKit, "typeIds");
    const types = yTypesMap
      ? Array.from(typeIds.keys()).map((compound) => {
          const [typeName, typeVariant] = compound.split("::");
          return this.getType(id, { name: typeName, variant: typeVariant || undefined });
        })
      : [];
    const yDesignsMap = getKit(yKit, "designs");
    const designIds = getKit(yKit, "designIds");
    const designs = yDesignsMap
      ? Array.from(designIds.keys()).map((compound) => {
          const [dName, dVariant, dView] = compound.split("::");
          return this.getDesign(id, { name: dName, variant: dVariant || undefined, view: dView || undefined });
        })
      : [];
    return {
      name: getKit(yKit, "name"),
      description: getKit(yKit, "description"),
      icon: getKit(yKit, "icon"),
      image: getKit(yKit, "image"),
      version: getKit(yKit, "version"),
      preview: getKit(yKit, "preview"),
      remote: getKit(yKit, "remote"),
      homepage: getKit(yKit, "homepage"),
      license: getKit(yKit, "license"),
      created: new Date(getKit(yKit, "created")),
      updated: new Date(getKit(yKit, "updated")),
      designs,
      types,
      qualities: this.getQualities(getKit(yKit, "qualities")),
    };
  }

  private updateKitInternal(kitId: KitIdLike, kitDiff: KitDiff): Kit {
    // Apply diff directly to the kit
    const base = this.getKit(kitId);
    const updated = diff.apply.kit(base, kitDiff);

    // Update the Y.js store with the changes
    const yKit = this.getYKit(kitId);

    if (kitDiff.description !== undefined) yKit.set("description", kitDiff.description);
    if (kitDiff.icon !== undefined) yKit.set("icon", kitDiff.icon);
    if (kitDiff.image !== undefined) yKit.set("image", kitDiff.image);
    if (kitDiff.preview !== undefined) yKit.set("preview", kitDiff.preview);
    if (kitDiff.remote !== undefined) yKit.set("remote", kitDiff.remote);
    if (kitDiff.homepage !== undefined) yKit.set("homepage", kitDiff.homepage);
    if (kitDiff.license !== undefined) yKit.set("license", kitDiff.license);

    if (kitDiff.qualities !== undefined) {
      const kit = this.getKit(kitId);
      let updatedQualities = [...(kit.qualities || [])];

      // Apply QualitiesDiff
      const qualitiesDiff = kitDiff.qualities;

      // Remove qualities
      if (qualitiesDiff.removed) {
        updatedQualities = updatedQualities.filter((q) => !qualitiesDiff.removed!.some((removed) => removed.name === q.name));
      }

      // Update qualities
      if (qualitiesDiff.updated) {
        qualitiesDiff.updated.forEach((update) => {
          const index = updatedQualities.findIndex((q) => q.name === update.name);
          if (index !== -1) {
            updatedQualities[index] = { ...updatedQualities[index], ...update } as Quality;
          }
        });
      }

      // Add new qualities
      if (qualitiesDiff.added) {
        updatedQualities.push(...qualitiesDiff.added);
      }

      yKit.set("qualities", this.createQualities(updatedQualities));
    }

    if (kitDiff.types) {
      if (kitDiff.types.removed) {
        kitDiff.types.removed.forEach((typeId) => {
          // @ts-ignore
          this.deleteType(kitId, typeId);
        });
      }
      if (kitDiff.types.added) {
        kitDiff.types.added.forEach((type) => {
          this.createType(kitId, type);
        });
      }
      if (kitDiff.types.updated) {
        kitDiff.types.updated.forEach((typeDiff) => {
          // @ts-ignore
          this.updateType(kitId, typeDiff);
        });
      }
    }

    if (kitDiff.designs) {
      if (kitDiff.designs.removed) {
        kitDiff.designs.removed.forEach((designId) => {
          // @ts-ignore
          this.deleteDesign(kitId, designId);
        });
      }
      if (kitDiff.designs.added) {
        kitDiff.designs.added.forEach((design) => {
          // @ts-ignore
          this.createDesign(kitId, design);
        });
      }
      if (kitDiff.designs.updated) {
        kitDiff.designs.updated.forEach((designDiff) => {
          this.updateDesign(kitId, designDiff);
        });
      }
    }

    yKit.set("updated", new Date().toISOString());
    return this.getKit(kitId);
  }

  // Kit Transaction Management
  onKitTransactionStart(kitId: KitIdLike): void {
    const key = this.key.kit(kitId);
    if (this.kitTransactionStacks.has(key)) {
      throw new Error(`Kit transaction already active for ${key}`);
    }
    this.kitTransactionStacks.set(key, []);
  }

  onKitTransactionAbort(kitId: KitIdLike): void {
    const key = this.key.kit(kitId);
    if (!this.kitTransactionStacks.has(key)) {
      throw new Error(`No active kit transaction for ${key}`);
    }
    this.kitTransactionStacks.delete(key);
  }

  onKitTransactionFinalize(kitId: KitIdLike): void {
    const key = this.key.kit(kitId);
    const diffStack = this.kitTransactionStacks.get(key);

    if (!diffStack) {
      throw new Error(`No active kit transaction for ${key}`);
    }

    if (diffStack.length > 0) {
      // Merge all diffs in the stack
      const mergedDiff = diffStack.reduce((merged, kitDiff) => diff.merge.kit(merged, kitDiff), {} as KitDiff);

      // Apply the merged diff
      this.updateKitInternal(kitId, mergedDiff);
    }

    this.kitTransactionStacks.delete(key);
  }

  // Design Editor Transaction Management
  onDesignEditorStoreTransactionStart(editorId: string, kitId: KitIdLike): void {
    const kitKey = this.key.kit(kitId);
    if (this.kitTransactionStacks.has(kitKey)) {
      throw new Error(`Cannot start design editor transaction: kit transaction already active for ${kitKey}`);
    }

    if (this.designEditorTransactionStacks.has(editorId)) {
      throw new Error(`Design editor transaction already active for ${editorId}`);
    }

    // Start kit transaction internally
    this.onKitTransactionStart(kitId);

    // Track design editor transaction
    this.designEditorTransactionStacks.set(editorId, {
      kitDiffs: [],
      editorStates: [],
    });
  }

  onDesignEditorStoreTransactionAbort(editorId: string, kitId: KitIdLike): void {
    if (!this.designEditorTransactionStacks.has(editorId)) {
      throw new Error(`No active design editor transaction for ${editorId}`);
    }

    // Abort kit transaction
    this.onKitTransactionAbort(kitId);

    // Clean up design editor transaction
    this.designEditorTransactionStacks.delete(editorId);
  }

  onDesignEditorStoreTransactionFinalize(editorId: string, kitId: KitIdLike): void {
    const transaction = this.designEditorTransactionStacks.get(editorId);

    if (!transaction) {
      throw new Error(`No active design editor transaction for ${editorId}`);
    }

    // Finalize kit transaction
    this.onKitTransactionFinalize(kitId);

    // Add to command history stack (implementation depends on design editor store structure)
    // This should store the merged kit diff and the editor state for undo/redo

    this.designEditorTransactionStacks.delete(editorId);
  }

  // Hook-based update methods that use scoping and transactions
  onKitChange(kitDiff: KitDiff, scopedKitId?: KitId): void {
    const kitId = kitDiff.name ? { name: kitDiff.name, version: kitDiff.version } : scopedKitId;

    if (!kitId) {
      throw new Error("Kit ID is required either in diff or from scope");
    }

    const key = this.key.kit(kitId);
    const transactionStack = this.kitTransactionStacks.get(key);

    if (transactionStack) {
      // Add to transaction stack
      transactionStack.push(kitDiff);
    } else {
      // Apply immediately
      this.updateKitInternal(kitId, kitDiff);
    }
  }

  onDesignChange(designDiff: DesignDiff, scopedKitId?: KitId, scopedDesignId?: DesignId): void {
    const kitId = scopedKitId;
    const designId = designDiff.name ? { name: designDiff.name, variant: designDiff.variant || "", view: designDiff.view || "" } : scopedDesignId;

    if (!kitId || !designId) {
      throw new Error("Kit ID and Design ID are required either in diff or from scope");
    }

    // Convert design diff to kit diff
    const kitDiff: KitDiff = {
      name: kitId.name,
      version: kitId.version,
      designs: {
        updated: [designDiff],
      },
    };

    this.onKitChange(kitDiff, kitId);
  }

  onTypeChange(typeDiff: TypeDiff, scopedKitId?: KitId, scopedTypeId?: TypeId): void {
    const kitId = scopedKitId;
    const typeId = typeDiff.name ? { name: typeDiff.name, variant: typeDiff.variant || "" } : scopedTypeId;

    if (!kitId || !typeId) {
      throw new Error("Kit ID and Type ID are required either in diff or from scope");
    }

    // Convert type diff to kit diff
    const kitDiff: KitDiff = {
      name: kitId.name,
      version: kitId.version,
      types: {
        updated: [typeDiff],
      },
    };

    this.onKitChange(kitDiff, kitId);
  }

  onPieceChange(pieceDiff: PieceDiff, scopedKitId?: KitId, scopedDesignId?: DesignId): void {
    const kitId = scopedKitId;
    const designId = scopedDesignId;

    if (!kitId || !designId || !pieceDiff.id_) {
      throw new Error("Kit ID, Design ID, and Piece ID are required from scope and diff");
    }

    // Convert piece diff to design diff to kit diff
    const designDiff: DesignDiff = {
      name: designId.name,
      variant: designId.variant,
      view: designId.view,
      pieces: {
        updated: [pieceDiff],
      },
    };

    this.onDesignChange(designDiff, kitId, designId);
  }

  onConnectionChange(connectionDiff: ConnectionDiff, scopedKitId?: KitId, scopedDesignId?: DesignId): void {
    const kitId = scopedKitId;
    const designId = scopedDesignId;

    if (!kitId || !designId || !connectionDiff.connected?.piece?.id_ || !connectionDiff.connecting?.piece?.id_) {
      throw new Error("Kit ID, Design ID, and Connection IDs are required from scope and diff");
    }

    // Convert connection diff to design diff to kit diff
    const designDiff: DesignDiff = {
      name: designId.name,
      variant: designId.variant,
      view: designId.view,
      connections: {
        updated: [connectionDiff],
      },
    };

    this.onDesignChange(designDiff, kitId, designId);
  }

  // Get merged kit diff for active transaction
  useSketchpadStoreKitDiff(kitId: KitIdLike): KitDiff | undefined {
    const key = this.key.kit(kitId);
    const diffStack = this.kitTransactionStacks.get(key);

    if (!diffStack || diffStack.length === 0) {
      return undefined;
    }

    return diffStack.reduce((merged, kitDiff) => diff.merge.kit(merged, kitDiff), {} as KitDiff);
  }

  deleteKit(id: KitId): void {
    const doc = this.getKitDoc(id);
    const key = this.key.kit(id);

    // Clear the kit document
    doc.destroy();

    // Remove from our maps
    this.kitDocs.delete(key);
    const provider = this.kitIndexeddbProviders.get(key);
    if (provider) {
      provider.destroy();
      this.kitIndexeddbProviders.delete(key);
    }
  }

  createType(kitId: KitIdLike, type: Type): void {
    // TypeSchema.parse(type);
    const yKit = this.getYKit(kitId);
    const types = getKit(yKit, "types");
    const typeIds = getKit(yKit, "typeIds");
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

    const yRepresentationsMap = getType(yType, "representations");
    const representations = yRepresentationsMap
      ? Array.from(yRepresentationsMap.values())
          .map((rMap) => {
            const tags = getRep(rMap, "tags")?.toArray() || [];
            return this.getRepresentation(kitId, id, { tags });
          })
          .filter((r): r is Representation => r !== null)
      : [];
    const yPortsMap = getType(yType, "ports");
    const ports = yPortsMap
      ? Array.from(yPortsMap.values())
          .map((pMap) => {
            const portId = getPort(pMap, "id_") as string;
            return this.getPort(kitId, id, { id_: portId });
          })
          .filter((p): p is Port => p !== null)
      : [];
    return {
      name: getType(yType, "name"),
      description: getType(yType, "description"),
      icon: getType(yType, "icon"),
      image: getType(yType, "image"),
      variant: getType(yType, "variant"),
      stock: getType(yType, "stock"),
      virtual: getType(yType, "virtual"),
      unit: getType(yType, "unit"),
      ports,
      representations,
      qualities: this.getQualities(getType(yType, "qualities")),
      authors: this.getAuthors(getType(yType, "authors")),
      created: new Date(getType(yType, "created")),
      updated: new Date(getType(yType, "updated")),
    };
  }

  updateType(kitId: KitId, diff: TypeDiff): Type {
    // Extract identifying information from diff to find the base type
    if (!diff.name) {
      throw new Error("Type name is required to identify which type to update");
    }
    const typeId: TypeId = { name: diff.name, variant: diff.variant };
    const base = this.getType(kitId, typeId);
    const yType = this.getYType(kitId, base);

    // Check if name or variant changed (affecting the compound key)
    const nameChanged = diff.name !== undefined && diff.name !== base.name;
    const variantChanged = diff.variant !== undefined && diff.variant !== base.variant;

    if (nameChanged || variantChanged) {
      // Update the ID mapping before updating the yType
      const oldId = { name: base.name, variant: base.variant };
      const newId = {
        name: diff.name ?? base.name,
        variant: diff.variant ?? base.variant,
      };
      this.updateTypeIdMapping(kitId, oldId, newId);
    }

    if (diff.name !== undefined) yType.set("name", diff.name);
    if (diff.description !== undefined) yType.set("description", diff.description);
    if (diff.icon !== undefined) yType.set("icon", diff.icon);
    if (diff.image !== undefined) yType.set("image", diff.image);
    if (diff.variant !== undefined) yType.set("variant", diff.variant);
    if (diff.stock !== undefined) yType.set("stock", diff.stock);
    if (diff.virtual !== undefined) yType.set("virtual", diff.virtual);
    if (diff.unit !== undefined) yType.set("unit", diff.unit);

    if (diff.ports !== undefined) {
      const validPorts = diff.ports.filter((p) => p.id_ !== undefined);
      const portsMap = new Y.Map<YPort>();
      validPorts.forEach((p) => portsMap.set(p.id_!, this.buildYPort(p)));
      yType.set("ports", portsMap);
    }
    if (diff.qualities !== undefined) {
      yType.set("qualities", this.createQualities(base.qualities || []));
    }
    if (diff.representations !== undefined) {
      const reps = new Y.Map<YRepresentation>();
      diff.representations.forEach((r) => reps.set(`${r.tags?.join(",") || ""}`, this.buildYRepresentation(r)));
      yType.set("representations", reps);
    }
    if (diff.authors !== undefined) {
      yType.set("authors", this.createAuthors(diff.authors));
    }

    yType.set("updated", new Date().toISOString());
    const resultId = diff.name !== undefined || diff.variant !== undefined ? { name: diff.name ?? base.name, variant: diff.variant ?? base.variant } : base;
    return this.getType(kitId, resultId);
  }

  deleteType(kitId: KitId, id: TypeId): void {
    const yKit = this.getYKit(kitId);
    const types = getKit(yKit, "types");
    const typeIds = yKit.get("typeIds") as Y.Map<string>;
    const compound = this.key.type(id);
    const uuid = typeIds.get(compound);
    if (!uuid) throw new Error(`Type (${id}) not found in kit (${kitId})`);
    types.delete(uuid);
    typeIds.delete(compound);
  }

  getKits(): Map<string, string[]> {
    const kitsMap = new Map<string, string[]>();

    this.kitDocs.forEach((doc, key) => {
      const yKit = doc.getMap<YKitVal>("kit");
      const name = yKit.get("name") as string;
      const version = yKit.get("version") as string;

      if (name) {
        const arr = kitsMap.get(name) || [];
        arr.push(version || "");
        kitsMap.set(name, arr);
      }
    });

    return kitsMap;
  }

  getTypes(kitId: KitId): Type[] {
    const yKit = this.getYKit(kitId);
    const typeIds = getKit(yKit, "typeIds");
    return Array.from(typeIds.keys()).map((compound) => {
      const [name, variant] = (compound as string).split("::");
      return this.getType(kitId, typeIdLikeToTypeId([name, variant || ""]));
    });
  }

  getDesigns(kitId: KitId): Design[] {
    const yKit = this.getYKit(kitId);
    const designIds = getKit(yKit, "designIds");
    return Array.from(designIds.keys()).map((compound) => {
      const [name, variant, view] = (compound as string).split("::");
      return this.getDesign(kitId, designIdLikeToDesignId([name, variant || "", view || ""]));
    });
  }

  getPieces(kitId: KitId, designId: DesignId): Piece[] {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces: Piece[] = [];
    const yPieces = getDesign(yDesign, "pieces");
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
    const yConnections = getDesign(yDesign, "connections");
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
    const yRepresentations = getType(yType, "representations");
    if (yRepresentations) {
      yRepresentations.forEach((yRep) => {
        const tags = getRep(yRep, "tags")?.toArray() || [];
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
    const yPorts = getType(yType, "ports");
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
    // DesignSchema.parse(design);
    const yKit = this.getYKit(kitId);
    const designs = getKit(yKit, "designs");
    const designIds = getKit(yKit, "designIds");
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

    const yPieces = getDesign(yDesign, "pieces");
    const pieces = yPieces
      ? Array.from(yPieces.values())
          .map((pMap) => {
            const pieceId = getPiece(pMap, "id_") as string;
            return this.getPiece(kitId, id, { id_: pieceId });
          })
          .filter((p): p is Piece => p !== null)
      : [];
    const yConnections = getDesign(yDesign, "connections");
    const connections = yConnections
      ? Array.from(yConnections.values())
          .map((cMap) => {
            const connectedPieceId = getSide(getConn(cMap, "connected"), "piece").get("id_") as string;
            const connectingPieceId = getSide(getConn(cMap, "connecting"), "piece").get("id_") as string;
            return this.getConnection(kitId, id, { connected: { piece: { id_: connectedPieceId } }, connecting: { piece: { id_: connectingPieceId } } });
          })
          .filter((c): c is Connection => c !== null)
      : [];
    return {
      name: getDesign(yDesign, "name"),
      description: getDesign(yDesign, "description"),
      icon: getDesign(yDesign, "icon"),
      image: getDesign(yDesign, "image"),
      variant: getDesign(yDesign, "variant"),
      view: getDesign(yDesign, "view"),
      unit: getDesign(yDesign, "unit"),
      created: new Date(getDesign(yDesign, "created")),
      updated: new Date(getDesign(yDesign, "updated")),
      authors: this.getAuthors(getDesign(yDesign, "authors")),
      pieces,
      connections,
      qualities: this.getQualities(getDesign(yDesign, "qualities")),
    };
  }

  updateDesign(kitId: KitIdLike, diff: DesignDiff): Design {
    // Extract identifying information from diff to find the base design
    if (!diff.name) {
      throw new Error("Design name is required to identify which design to update");
    }
    const designId: DesignId = { name: diff.name, variant: diff.variant, view: diff.view };
    const base = this.getDesign(kitId, designId);
    const normalizedKitId: KitId = kitIdLikeToKitId(kitId);
    const yDesign = this.getYDesign(kitId, base);

    // Check if name, variant, or view changed (affecting the compound key)
    const nameChanged = diff.name !== undefined && diff.name !== base.name;
    const variantChanged = diff.variant !== undefined && diff.variant !== base.variant;
    const viewChanged = diff.view !== undefined && diff.view !== base.view;

    if (nameChanged || variantChanged || viewChanged) {
      // Update the ID mapping before updating the yDesign
      const oldId = {
        name: base.name,
        variant: base.variant,
        view: base.view,
      };
      const newId = {
        name: diff.name ?? base.name,
        variant: diff.variant ?? base.variant,
        view: diff.view ?? base.view,
      };
      this.updateDesignIdMapping(kitId, oldId, newId);
    }

    if (diff.name !== undefined) yDesign.set("name", diff.name);
    if (diff.description !== undefined) yDesign.set("description", diff.description);
    if (diff.icon !== undefined) yDesign.set("icon", diff.icon);
    if (diff.image !== undefined) yDesign.set("image", diff.image);
    if (diff.variant !== undefined) yDesign.set("variant", diff.variant);
    if (diff.view !== undefined) yDesign.set("view", diff.view);
    if (diff.unit !== undefined) yDesign.set("unit", diff.unit);

    if (diff.pieces) {
      const basePieces = base.pieces || [];
      const designId: DesignId = designIdLikeToDesignId(base);
      if (diff.pieces.removed) {
        diff.pieces.removed.forEach((pieceId) => {
          this.deletePiece(normalizedKitId, designId, pieceId);
        });
      }
      if (diff.pieces.added) {
        diff.pieces.added.forEach((piece) => {
          this.createPiece(normalizedKitId, designId, piece);
        });
      }
      if (diff.pieces.updated) {
        diff.pieces.updated.forEach((pieceDiff) => {
          const basePiece = basePieces.find((p) => p.id_ === pieceDiff.id_);
          if (basePiece) {
            this.updatePiece(normalizedKitId, designId, pieceDiff);
          }
        });
      }
    }

    if (diff.connections) {
      const baseConnections = base.connections || [];
      const designId: DesignId = designIdLikeToDesignId(base);
      if (diff.connections.removed) {
        diff.connections.removed.forEach((connectionId) => {
          this.deleteConnection(normalizedKitId, designId, connectionId);
        });
      }
      if (diff.connections.added) {
        diff.connections.added.forEach((connection) => {
          this.createConnection(normalizedKitId, designId, connection);
        });
      }
      if (diff.connections.updated) {
        diff.connections.updated.forEach((connectionDiff) => {
          const baseConnection = baseConnections.find((c) => c.connected.piece.id_ === connectionDiff.connected?.piece?.id_ && c.connecting.piece.id_ === connectionDiff.connecting?.piece?.id_);
          if (baseConnection) {
            this.updateConnection(normalizedKitId, designId, connectionDiff);
          }
        });
      }
    }

    if (diff.qualities !== undefined) {
      yDesign.set("qualities", this.createQualities(base.qualities || []));
    }

    yDesign.set("updated", new Date().toISOString());
    return this.getDesign(
      kitId,
      nameChanged || variantChanged || viewChanged
        ? {
            name: diff.name ?? base.name,
            variant: diff.variant ?? base.variant,
            view: diff.view ?? base.view,
          }
        : base,
    );
  }

  deleteDesign(kitId: KitId, id: DesignId): void {
    const yKit = this.getYKit(kitId);
    const designs = getKit(yKit, "designs");
    const designIds = getKit(yKit, "designIds");
    const compound = this.key.design(id);
    const uuid = designIds.get(compound);
    if (!uuid) throw new Error(`Design (${id}) not found in kit (${kitId})`);
    designs.delete(uuid);
    designIds.delete(compound);
  }

  createPiece(kitId: KitId, designId: DesignId, piece: Piece): void {
    // PieceSchema.parse(piece);
    const yDesign = this.getYDesign(kitId, designId);
    const yPieces = getDesign(yDesign, "pieces");
    const yKit = this.getYKit(kitId);
    const pieceIds = yKit.get("pieceIds") as Y.Map<string>;

    const pieceUuid = uuidv4();
    const designKey = this.key.design(designId);
    const pieceKey = this.key.piece(piece);
    const fullPieceKey = `${designKey}::${pieceKey}`;

    yPieces.set(pieceUuid, this.buildYPiece(piece));
    pieceIds.set(fullPieceKey, pieceUuid);
  }

  getPiece(kitId: KitIdLike, designId: DesignIdLike, id: PieceIdLike): Piece {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = getDesign(yDesign, "pieces");
    const uuid = this.getPieceUuid(kitId, designId, id);
    const yPiece = uuid ? pieces.get(uuid) : undefined;
    const pieceId = pieceIdLikeToPieceId(id);
    if (!yPiece) throw new Error(`Piece (${JSON.stringify(pieceId)}) not found in design (${JSON.stringify(designIdLikeToDesignId(designId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);

    const type = getPiece(yPiece, "type");
    const typeName = type.get("name") as string;
    const typeVariant = (type.get("variant") as string) || "";

    const yPlane = getPiece(yPiece, "plane") as YPlane | undefined;
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

    const yCenter = getPiece(yPiece, "center") as YLeafMapNumber | undefined;
    const center: DiagramPoint | null = yCenter
      ? {
          x: yCenter.get("x") as number,
          y: yCenter.get("y") as number,
        }
      : null;

    return {
      id_: getPiece(yPiece, "id_"),
      description: getPiece(yPiece, "description"),
      type: { name: typeName, variant: typeVariant },
      plane: plane ?? undefined,
      center: center ?? undefined,
      qualities: this.getQualities(getPiece(yPiece, "qualities")),
    };
  }

  updatePiece(kitId: KitId, designId: DesignId, diff: PieceDiff): Piece {
    if (!diff.id_) throw new Error("Piece ID is required for update.");
    const pieceId: PieceId = { id_: diff.id_ };
    const base = this.getPiece(kitId, designId, pieceId);
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = getDesign(yDesign, "pieces");
    const uuid = this.getPieceUuid(kitId, designId, base);
    const yPiece = uuid ? pieces.get(uuid) : undefined;
    if (!yPiece) throw new Error(`Piece ${diff.id_} not found in design ${designId} in kit ${kitId}`);

    // Check if piece ID changed (affecting the mapping)
    const idChanged = diff.id_ !== base.id_;
    if (idChanged) {
      this.updatePieceIdMapping(kitId, designId, { id_: base.id_ }, { id_: diff.id_ });
    }

    if (diff.id_ !== undefined) yPiece.set("id_", diff.id_);
    if (diff.description !== undefined) yPiece.set("description", diff.description);
    if (diff.type !== undefined) {
      const yType = new Y.Map<string>();
      yType.set("name", diff.type.name);
      yType.set("variant", diff.type.variant || "");
      yPiece.set("type", yType);
    }
    if (diff.center !== undefined && diff.center !== null) {
      const yCenter = new Y.Map<number>();
      yCenter.set("x", diff.center.x);
      yCenter.set("y", diff.center.y);
      yPiece.set("center", yCenter);
    }
    if (diff.plane !== undefined && diff.plane !== null) {
      const yPlane = new Y.Map<YVec3>();
      const yOrigin = new Y.Map<number>();
      yOrigin.set("x", diff.plane.origin.x);
      yOrigin.set("y", diff.plane.origin.y);
      yOrigin.set("z", diff.plane.origin.z);
      yPlane.set("origin", yOrigin);
      const yXAxis = new Y.Map<number>();
      yXAxis.set("x", diff.plane.xAxis.x);
      yXAxis.set("y", diff.plane.xAxis.y);
      yXAxis.set("z", diff.plane.xAxis.z);
      yPlane.set("xAxis", yXAxis);
      const yYAxis = new Y.Map<number>();
      yYAxis.set("x", diff.plane.yAxis.x);
      yYAxis.set("y", diff.plane.yAxis.y);
      yYAxis.set("z", diff.plane.yAxis.z);
      yPlane.set("yAxis", yYAxis);
      yPiece.set("plane", yPlane);
    }

    const resultId = diff.id_ !== undefined ? diff.id_ : base.id_;
    return this.getPiece(kitId, designId, resultId);
  }

  deletePiece(kitId: KitId, designId: DesignId, id: PieceId): boolean {
    const yDesign = this.getYDesign(kitId, designId);
    const pieces = getDesign(yDesign, "pieces");
    const yKit = this.getYKit(kitId);
    const pieceIds = yKit.get("pieceIds") as Y.Map<string>;

    const designKey = this.key.design(designId);
    const pieceKey = this.key.piece(id);
    const fullPieceKey = `${designKey}::${pieceKey}`;
    const uuid = pieceIds.get(fullPieceKey);

    if (!uuid) return false;

    const existed = pieces.has(uuid);
    pieces.delete(uuid);
    pieceIds.delete(fullPieceKey);
    return existed;
  }

  createConnection(kitId: KitId, designId: DesignId, connection: Connection): void {
    // ConnectionSchema.parse(connection);
    const yDesign = this.getYDesign(kitId, designId);
    if (!yDesign) throw new Error(`Design(${designId}) not found in kit(${kitId})`);
    const yKit = this.getYKit(kitId);
    const connectionIds = yKit.get("connectionIds") as Y.Map<string>;

    const designKey = this.key.design(designId);
    const connectionKey = this.key.connection(connection);
    const fullConnectionKey = `${designKey}::${connectionKey}`;
    const reverseConnectionKey = `${designKey}::${connection.connecting.piece.id_}--${connection.connected.piece.id_}`;

    const connections = getDesign(yDesign, "connections");
    const existingUuid = connectionIds.get(fullConnectionKey);
    const reverseUuid = connectionIds.get(reverseConnectionKey);

    if (existingUuid && connections.get(existingUuid)) {
      throw new Error(`Connection (${connectionKey}) already exists in design (${designId}) in kit (${kitId})`);
    }
    if (reverseUuid && connections.get(reverseUuid)) {
      throw new Error(`Reverse connection (${reverseConnectionKey}) already exists in design (${designId}) in kit (${kitId})`);
    }

    const connectionUuid = uuidv4();
    connections.set(connectionUuid, this.buildYConnection(connection));
    connectionIds.set(fullConnectionKey, connectionUuid);
  }

  getConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): Connection {
    const yDesign = this.getYDesign(kitId, designId);
    const connections = getDesign(yDesign, "connections");
    const uuid = this.getConnectionUuid(kitId, designId, id);
    const yConnection = uuid ? connections.get(uuid) : undefined;
    const connectionId = connectionIdLikeToConnectionId(id);
    if (!yConnection) throw new Error(`Connection (${JSON.stringify(connectionId)}) not found in design (${JSON.stringify(designIdLikeToDesignId(designId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    const yConnected = getConn(yConnection, "connected");
    const connectedSide: Side = {
      piece: { id_: getSide(yConnected, "piece").get("id_") || "" },
      port: { id_: getSide(yConnected, "port").get("id_") || "" },
    };
    const yConnecting = getConn(yConnection, "connecting");
    const connectingSide: Side = {
      piece: { id_: getSide(yConnecting, "piece").get("id_") || "" },
      port: { id_: getSide(yConnecting, "port").get("id_") || "" },
    };
    return {
      description: getConn(yConnection, "description"),
      connected: connectedSide,
      connecting: connectingSide,
      gap: getConn(yConnection, "gap"),
      shift: getConn(yConnection, "shift"),
      rise: getConn(yConnection, "rise"),
      rotation: getConn(yConnection, "rotation"),
      turn: getConn(yConnection, "turn"),
      tilt: getConn(yConnection, "tilt"),
      x: getConn(yConnection, "x"),
      y: getConn(yConnection, "y"),
      qualities: this.getQualities(getConn(yConnection, "qualities")),
    };
  }

  updateConnection(kitId: KitIdLike, designId: DesignIdLike, diff: ConnectionDiff): void {
    if (!diff.connected?.piece?.id_ || !diff.connecting?.piece?.id_) {
      throw new Error("Connected and connecting piece IDs are required for update.");
    }
    const id = `${diff.connected.piece.id_}--${diff.connecting.piece.id_}`;
    const yDesign = this.getYDesign(kitId, designId);
    if (!yDesign) throw new Error(`Design (${designId}) not found in kit (${kitId})`);
    const connections = getDesign(yDesign, "connections");
    const yConnection = connections.get(id);
    if (!yConnection) throw new Error(`Connection (${id}) not found in design (${designId}) in kit (${kitId})`);

    if (diff.description !== undefined) yConnection.set("description", diff.description);
    if (diff.connected !== undefined) {
      const ySide = new Y.Map<YLeafMapString>();
      const yPiece = new Y.Map<string>();
      yPiece.set("id_", diff.connected?.piece.id_ || "");
      const yPort = new Y.Map<string>();
      yPort.set("id_", diff.connected?.port?.id_ || "");
      ySide.set("piece", yPiece);
      ySide.set("port", yPort);
      yConnection.set("connected", ySide);
    }
    if (diff.connecting !== undefined) {
      const ySide = new Y.Map<YLeafMapString>();
      const yPiece = new Y.Map<string>();
      yPiece.set("id_", diff.connecting?.piece.id_ || "");
      const yPort = new Y.Map<string>();
      yPort.set("id_", diff.connecting?.port?.id_ || "");
      ySide.set("piece", yPiece);
      ySide.set("port", yPort);
      yConnection.set("connecting", ySide);
    }
    if (diff.gap !== undefined) yConnection.set("gap", diff.gap);
    if (diff.rotation !== undefined) yConnection.set("rotation", diff.rotation);
    if (diff.shift !== undefined) yConnection.set("shift", diff.shift);
    if (diff.rise !== undefined) yConnection.set("rise", diff.rise);
    if (diff.turn !== undefined) yConnection.set("turn", diff.turn);
    if (diff.tilt !== undefined) yConnection.set("tilt", diff.tilt);
    if (diff.x !== undefined) yConnection.set("x", diff.x);
    if (diff.y !== undefined) yConnection.set("y", diff.y);
  }

  deleteConnection(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike): void {
    const yDesign = this.getYDesign(kitId, designId);
    const connections = getDesign(yDesign, "connections");
    const yKit = this.getYKit(kitId);
    const connectionIds = yKit.get("connectionIds") as Y.Map<string>;

    const designKey = this.key.design(designId);
    const connectionKey = this.key.connection(id);
    const fullConnectionKey = `${designKey}::${connectionKey}`;
    const uuid = connectionIds.get(fullConnectionKey);

    if (uuid) {
      connections.delete(uuid);
      connectionIds.delete(fullConnectionKey);
    }
  }

  createRepresentation(kitId: KitIdLike, typeId: TypeIdLike, representation: Representation): void {
    // RepresentationSchema.parse(representation);
    const yType = this.getYType(kitId, typeId);
    const yKit = this.getYKit(kitId);
    const representationIds = yKit.get("representationIds") as Y.Map<string>;

    const representations = getType(yType, "representations");
    const representationUuid = uuidv4();
    const typeKey = this.key.type(typeId);
    const repKey = this.key.representation(representation);

    representations.set(representationUuid, this.buildYRepresentation(representation));
    representationIds.set(`${typeKey}::${repKey}`, representationUuid);
  }

  getRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): Representation {
    const yType = this.getYType(kitId, typeId);
    const representations = getType(yType, "representations");
    const uuid = this.getRepresentationUuid(kitId, typeId, id);
    const yRepresentation = uuid ? representations.get(uuid) : undefined;
    const repId = representationIdLikeToRepresentationId(id);
    if (!yRepresentation) throw new Error(`Representation (${JSON.stringify(repId)}) not found in type (${JSON.stringify(typeIdLikeToTypeId(typeId))}) in kit (${JSON.stringify(kitIdLikeToKitId(kitId))})`);
    return {
      url: getRep(yRepresentation, "url"),
      description: getRep(yRepresentation, "description"),
      tags: getRep(yRepresentation, "tags").toArray(),
      qualities: this.getQualities(getRep(yRepresentation, "qualities")),
    };
  }

  updateRepresentation(kitId: KitId, typeId: TypeId, representation: Partial<Representation>): void {
    const yType = this.getYType(kitId, typeId);
    const representations = getType(yType, "representations");
    const id = `${representation.tags?.join(",") || ""}`;
    const yRepresentation = representations.get(id);
    if (!yRepresentation) throw new Error(`Representation (${id}) not found in type (${typeId}) in kit (${kitId})`);

    if (representation.description !== undefined) yRepresentation.set("description", representation.description);
    if (representation.tags !== undefined) {
      const yTags = Y.Array.from(representation.tags || []);
      yRepresentation.set("tags", yTags);
    }
    if (representation.qualities !== undefined) {
      const yQualities = getRep(yRepresentation, "qualities") || new Y.Array<YQuality>();
      yQualities.delete(0, yQualities.length);
      representation.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      yRepresentation.set("qualities", yQualities);
    }
  }

  deleteRepresentation(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike): void {
    const yType = this.getYType(kitId, typeId);
    const representations = getType(yType, "representations");
    const yKit = this.getYKit(kitId);
    const representationIds = yKit.get("representationIds") as Y.Map<string>;

    const typeKey = this.key.type(typeId);
    const repKey = this.key.representation(id);
    const compoundKey = `${typeKey}::${repKey}`;
    const uuid = representationIds.get(compoundKey);

    if (uuid) {
      representations.delete(uuid);
      representationIds.delete(compoundKey);
    }
  }

  createPort(kitId: KitIdLike, typeId: TypeIdLike, port: Port): void {
    // PortSchema.parse(port);
    const yType = this.getYType(kitId, typeId);
    const yKit = this.getYKit(kitId);
    const portIds = yKit.get("portIds") as Y.Map<string>;

    const ports = getType(yType, "ports");
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

    const portUuid = uuidv4();
    const typeKey = this.key.type(typeId);
    const portKey = this.key.port(port);

    ports.set(portUuid, yPort);
    portIds.set(`${typeKey}::${portKey}`, portUuid);
  }

  getPort(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike): Port {
    const yType = this.getYType(kitId, typeId) as YType;
    const ports = getType(yType, "ports");
    const uuid = this.getPortUuid(kitId, typeId, id);
    const yPort = uuid ? ports.get(uuid) : undefined;
    const portId = portIdLikeToPortId(id);
    if (!yPort) throw new Error(`Port (${JSON.stringify(portId)}) not found in type (${JSON.stringify(typeIdLikeToTypeId(typeId))})`);

    const yDirection = getPort(yPort, "direction");
    if (!yDirection) throw new Error(`Direction not found in port (${JSON.stringify(portId)})`);
    const yPoint = getPort(yPort, "point");
    if (!yPoint) throw new Error(`Point not found in port (${JSON.stringify(portId)})`);
    return {
      id_: getPort(yPort, "id_"),
      description: getPort(yPort, "description"),
      mandatory: getPort(yPort, "mandatory"),
      family: getPort(yPort, "family"),
      compatibleFamilies: getPort(yPort, "compatibleFamilies")?.toArray() || [],
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
      t: getPort(yPort, "t"),
      qualities: this.getQualities(getPort(yPort, "qualities")),
    };
  }

  updatePort(kitId: KitId, typeId: TypeId, portId: string, port: Partial<Port>): void {
    const yType = this.getYType(kitId, typeId);
    const ports = getType(yType, "ports");
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

  updateRepresentationDiff(kitId: KitIdLike, typeId: TypeIdLike, before: Representation, after: Representation): Representation {
    const diff = getDiff.representation(before, after);
    return this.applyRepresentationDiff(kitId, typeId, before, diff);
  }

  applyRepresentationDiff(kitId: KitIdLike, typeId: TypeIdLike, base: Representation, diff: RepresentationDiff): Representation {
    const normalizedKitId: KitId = kitIdLikeToKitId(kitId);
    const normalizedTypeId: TypeId = typeIdLikeToTypeId(typeId);
    const updated = applyDiff.representation(base, diff);
    this.updateRepresentation(normalizedKitId, normalizedTypeId, updated);
    return updated;
  }

  updatePortDiff(kitId: KitIdLike, typeId: TypeIdLike, portId: string, before: Port, after: Port): Port {
    const diff = getDiff.port(before, after);
    return this.applyPortDiff(kitId, typeId, portId, before, diff);
  }

  applyPortDiff(kitId: KitIdLike, typeId: TypeIdLike, portId: string, base: Port, diff: PortDiff): Port {
    const normalizedKitId: KitId = kitIdLikeToKitId(kitId);
    const normalizedTypeId: TypeId = typeIdLikeToTypeId(typeId);
    const updated = applyDiff.port(base, diff);
    this.updatePort(normalizedKitId, normalizedTypeId, portId, updated);
    return updated;
  }

  updateQualityDiff(before: Quality, after: Quality): Quality {
    const diff = getDiff.quality(before, after);
    return this.applyQualityDiff(before, diff);
  }

  applyQualityDiff(base: Quality, diff: QualityDiff): Quality {
    return applyDiff.quality(base, diff);
  }

  deletePort(kitId: KitId, typeId: TypeId, portId: string): void {
    const yType = this.getYType(kitId, typeId);
    const ports = getType(yType, "ports");
    const yKit = this.getYKit(kitId);
    const portIds = yKit.get("portIds") as Y.Map<string>;

    const typeKey = this.key.type(typeId);
    const portKey = portId;
    const compoundKey = `${typeKey}::${portKey}`;
    const uuid = portIds.get(compoundKey);

    if (uuid) {
      ports.delete(uuid);
      portIds.delete(compoundKey);
    }
  }

  createFile(kitId: KitIdLike, url: string, data: Uint8Array): void {
    const doc = this.getKitDoc(kitId);
    doc.getMap<Uint8Array>("files").set(url, data);
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

  getFileData(kitId: KitIdLike | undefined, url: string): Uint8Array {
    if (!kitId) {
      throw new Error(`Cannot get file data for ${url} without kit ID`);
    }
    const doc = this.getKitDoc(kitId);
    const fileData = doc.getMap<Uint8Array>("files").get(url);
    if (!fileData) throw new Error(`File (${url}) not found`);
    return fileData as Uint8Array;
  }

  deleteFile(kitId: KitIdLike, url: string): void {
    const doc = this.getKitDoc(kitId);
    doc.getMap<Uint8Array>("files").delete(url);
    this.fileUrls.delete(url);
  }

  deleteFiles(kitId: KitIdLike): void {
    const doc = this.getKitDoc(kitId);
    doc.getMap<Uint8Array>("files").clear();
    this.fileUrls.clear();
  }

  getOrCreateDesignEditorStoreId(kitId: KitIdLike, designId: DesignIdLike): string {
    // Create a deterministic ID based on kit and design
    const kitKey = this.key.kit(kitId);
    const designKey = this.key.design(designId);
    const id = `${kitKey}|${designKey}`;

    // Check if design editor already exists
    const designEditors = this.getYDesignEditorStores();
    if (!designEditors.has(id)) {
      // Create the design editor state in Yjs
      const yDesignEditorStore = this.createYDesignEditorStore();
      designEditors.set(id, yDesignEditorStore);
    }

    return id;
  }

  private getDesignEditorStoreStateFromYjs(id: string): DesignEditorStoreState {
    const yDesignEditorStore = this.getYDesignEditorStore(id);

    // Parse the design ID from the editor ID
    const parseEditorId = (editorId: string): { kitId: KitId; designId: DesignId } => {
      const [kitPart, designPart] = editorId.split("|");
      // Kit format: name::version
      const [kitName, kitVersion = ""] = kitPart.split("::");
      // Design format: name::variant::view
      const [designName, designVariant = "", designView = ""] = designPart.split("::");
      return {
        kitId: { name: kitName, version: kitVersion },
        designId: { name: designName, variant: designVariant, view: designView },
      };
    };

    const { designId } = parseEditorId(id);

    const selectedPieceIds = (getDesignEditorStore(yDesignEditorStore, "selectedPieceIds") as YStringArray).toArray().map((id) => ({ id_: id }));
    const selectedConnections = (getDesignEditorStore(yDesignEditorStore, "selectedConnections") as YStringArray).toArray().map((connId) => {
      const [connectedId, connectingId] = connId.split("--");
      return {
        connected: {
          piece: { id_: connectedId },
          port: { id_: "" },
        },
        connecting: {
          piece: { id_: connectingId },
          port: { id_: "" },
        },
      };
    });

    const selectedPiecePortPieceId = getDesignEditorStore(yDesignEditorStore, "selectedPiecePortPieceId") as string;
    const selectedPiecePortPortId = getDesignEditorStore(yDesignEditorStore, "selectedPiecePortPortId") as string;
    const selectedPiecePortId = selectedPiecePortPieceId && selectedPiecePortPortId ? { pieceId: { id_: selectedPiecePortPieceId }, portId: { id_: selectedPiecePortPortId } } : undefined;

    return {
      designId,
      fullscreenPanel: getDesignEditorStore(yDesignEditorStore, "fullscreenPanel") as DesignEditorStoreFullscreenPanel,
      selection: {
        selectedPieceIds,
        selectedConnections,
        selectedPiecePortId,
      },
      designDiff: {
        pieces: { added: [], removed: [], updated: [] },
        connections: { added: [], removed: [], updated: [] },
      },
      isTransactionActive: getDesignEditorStore(yDesignEditorStore, "isTransactionActive") as boolean,
      operationStack: [],
      operationIndex: -1,
      presence: {
        cursor: {
          x: getDesignEditorStore(yDesignEditorStore, "presenceCursorX") as number,
          y: getDesignEditorStore(yDesignEditorStore, "presenceCursorY") as number,
        },
        camera: {
          position: {
            x: getDesignEditorStore(yDesignEditorStore, "presenceCameraPositionX") as number,
            y: getDesignEditorStore(yDesignEditorStore, "presenceCameraPositionY") as number,
            z: getDesignEditorStore(yDesignEditorStore, "presenceCameraPositionZ") as number,
          },
          forward: {
            x: getDesignEditorStore(yDesignEditorStore, "presenceCameraForwardX") as number,
            y: getDesignEditorStore(yDesignEditorStore, "presenceCameraForwardY") as number,
            z: getDesignEditorStore(yDesignEditorStore, "presenceCameraForwardZ") as number,
          },
          up: { x: 0, y: 1, z: 0 },
        },
      },
      others: [],
    };
  }

  getDesignEditorStoreStore(id: string): {
    updateDesignEditorStoreSelection: (selection: DesignEditorStoreSelection) => void;
    deleteSelectedPiecesAndConnections: () => void;
    pushOperation: (undo: DesignDiff, redo: DesignDiff, selection: DesignEditorStoreSelection) => void;
    undo: () => void;
    redo: () => void;
    transact: (operations: () => void) => void;
    subscribe: (callback: () => void) => () => void;
  } | null {
    const yDesignEditorStore = this.getYDesignEditorStore(id);
    if (!yDesignEditorStore) return null;

    // Parse kit and design IDs from the editor ID
    const parseEditorId = (editorId: string): { kitId: KitId; designId: DesignId } => {
      const [kitPart, designPart] = editorId.split("|");
      // Kit format: name::version
      const [kitName, kitVersion = ""] = kitPart.split("::");
      // Design format: name::variant::view
      const [designName, designVariant = "", designView = ""] = designPart.split("::");
      return {
        kitId: { name: kitName, version: kitVersion },
        designId: { name: designName, variant: designVariant, view: designView },
      };
    };

    const { kitId, designId } = parseEditorId(id);

    return {
      getState: () => this.getDesignEditorStoreStateFromYjs(id),
      setState: (state: DesignEditorStoreState) => {
        // Update the Yjs state with the new state
        this.updateYDesignEditorStoreFromState(id, state);
      },
      getDesignId: () => designId,
      getKitId: () => kitId,
      updateDesignEditorStoreSelection: (selection: DesignEditorStoreSelection) => {
        const selectedPieceIds = getDesignEditorStore(yDesignEditorStore, "selectedPieceIds") as YStringArray;
        const selectedConnections = getDesignEditorStore(yDesignEditorStore, "selectedConnections") as YStringArray;

        selectedPieceIds.delete(0, selectedPieceIds.length);
        selection.selectedPieceIds.forEach((pieceId) => selectedPieceIds.push([pieceId.id_]));

        selectedConnections.delete(0, selectedConnections.length);
        selection.selectedConnections.forEach((conn) => selectedConnections.push([`${conn.connected.piece.id_}--${conn.connecting.piece.id_}`]));

        if (selection.selectedPiecePortId) {
          yDesignEditorStore.set("selectedPiecePortPieceId", selection.selectedPiecePortId.pieceId.id_);
          yDesignEditorStore.set("selectedPiecePortPortId", selection.selectedPiecePortId.portId.id_ || "");
        } else {
          yDesignEditorStore.set("selectedPiecePortPieceId", "");
          yDesignEditorStore.set("selectedPiecePortPortId", "");
        }
      },
      deleteSelectedPiecesAndConnections: () => {
        console.warn("deleteSelectedPiecesAndConnections not implemented");
      },
      pushOperation: () => {
        console.warn("pushOperation not implemented");
      },
      invertDiff: (diff: DesignDiff) => diff, // TODO: Implement properly
      undo: () => {
        console.warn("undo not implemented");
      },
      redo: () => {
        console.warn("redo not implemented");
      },
      transact: (operations: () => void) => {
        this.sketchpadDoc.transact(operations, this.id);
      },
      subscribe: (callback: () => void) => {
        const observer = () => callback();
        yDesignEditorStore.observe(observer);
        return () => yDesignEditorStore.unobserve(observer);
      },
    };
  }

  private updateYDesignEditorStoreFromState(id: string, state: DesignEditorStoreState): void {
    const yDesignEditorStore = this.getYDesignEditorStore(id);
    yDesignEditorStore.set("fullscreenPanel", state.fullscreenPanel);
    yDesignEditorStore.set("isTransactionActive", state.isTransactionActive);

    // Update selection
    const selectedPieceIds = getDesignEditorStore(yDesignEditorStore, "selectedPieceIds") as YStringArray;
    selectedPieceIds.delete(0, selectedPieceIds.length);
    state.selection.selectedPieceIds.forEach((pieceId) => selectedPieceIds.push([pieceId.id_]));

    const selectedConnections = getDesignEditorStore(yDesignEditorStore, "selectedConnections") as YStringArray;
    selectedConnections.delete(0, selectedConnections.length);
    state.selection.selectedConnections.forEach((conn) => selectedConnections.push([`${conn.connected.piece.id_}--${conn.connecting.piece.id_}`]));

    if (state.selection.selectedPiecePortId) {
      yDesignEditorStore.set("selectedPiecePortPieceId", state.selection.selectedPiecePortId.pieceId.id_);
      yDesignEditorStore.set("selectedPiecePortPortId", state.selection.selectedPiecePortId.portId.id_ || "");
    } else {
      yDesignEditorStore.set("selectedPiecePortPieceId", "");
      yDesignEditorStore.set("selectedPiecePortPortId", "");
    }

    // Update presence
    yDesignEditorStore.set("presenceCursorX", state.presence.cursor?.x || 0);
    yDesignEditorStore.set("presenceCursorY", state.presence.cursor?.y || 0);
    yDesignEditorStore.set("presenceCameraPositionX", state.presence.camera?.position.x || 0);
    yDesignEditorStore.set("presenceCameraPositionY", state.presence.camera?.position.y || 0);
    yDesignEditorStore.set("presenceCameraPositionZ", state.presence.camera?.position.z || 0);
    yDesignEditorStore.set("presenceCameraForwardX", state.presence.camera?.forward.x || 0);
    yDesignEditorStore.set("presenceCameraForwardY", state.presence.camera?.forward.y || 0);
    yDesignEditorStore.set("presenceCameraForwardZ", state.presence.camera?.forward.z || 0);
  }

  deleteDesignEditorStoreStore(id: string): void {
    const designEditors = this.getYDesignEditorStores();
    designEditors.delete(id);
  }

  getSketchpadState(): SketchpadState {
    const ySketchpad = this.getYSketchpad();
    return {
      mode: (getSketchpadStore(ySketchpad, "mode") as Mode) || Mode.USER,
      theme: (getSketchpadStore(ySketchpad, "theme") as Theme) || Theme.SYSTEM,
      layout: (getSketchpadStore(ySketchpad, "layout") as Layout) || Layout.NORMAL,
      activeDesignEditorId: getSketchpadStore(ySketchpad, "activeDesignEditorId") || undefined,
    };
  }

  setSketchpadMode(mode: Mode): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("mode", mode);
  }

  setSketchpadTheme(theme: Theme): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("theme", theme);
  }

  setSketchpadLayout(layout: Layout): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("layout", layout);
  }

  setActiveDesignEditorStoreId(id?: string): void {
    const ySketchpad = this.getYSketchpad();
    ySketchpad.set("activeDesignEditorId", id || "");
  }

  onSketchpadStateChange(callback: () => void) {
    const ySketchpad = this.getYSketchpad() as unknown as Y.Map<any>;
    const observer = () => callback();
    ySketchpad.observe(observer);
    return () => ySketchpad.unobserve(observer);
  }

  transact(commands: () => void): void {
    // For now, we'll use the sketchpad doc for general transactions
    // Individual kit operations should use their respective docs
    this.sketchpadDoc.transact(commands, this.id);
  }

  async importFiles(url: string, kitId: KitIdLike, force = false): Promise<void> {
    try {
      const zipData = await fetch(url).then((res) => res.arrayBuffer());
      const zip = await JSZip.loadAsync(zipData);

      // Load all files from the zip and overwrite existing files in memory
      for (const fileEntry of Object.values(zip.files)) {
        if (!fileEntry.dir) {
          const fileData = await fileEntry.async("uint8array");
          if (!force && this.fileUrls.has(fileEntry.name)) {
            throw new Error(`File ${fileEntry.name} already exists. Use force=true to overwrite.`);
          }
          this.createFile(kitId, fileEntry.name, fileData);
        }
      }

      console.log(`All files imported successfully from ${url}`);
    } catch (error) {
      console.error("Error importing files:", error);
      throw error;
    }
  }

  private importKitData(kit: Kit, force = false): void {
    if (!force) {
      // Try to create kit normally - will throw if already exists
      this.createKit(kit);
      return;
    }

    // Force mode: check if kit exists and update or create accordingly
    try {
      const existingKit = this.getKit(kit);
      // Kit exists, create a diff to update it with new data
      const kitDiff: KitDiff = {
        name: kit.name,
        description: kit.description,
        icon: kit.icon,
        image: kit.image,
        preview: kit.preview,
        version: kit.version,
        remote: kit.remote,
        homepage: kit.homepage,
        license: kit.license,
        qualities: kit.qualities ? { added: kit.qualities } : undefined,
        types: kit.types ? { added: kit.types } : undefined,
        designs: kit.designs ? { added: kit.designs } : undefined,
      };
      this.updateKitInternal(kit, kitDiff);

      // Handle types with force logic
      kit.types?.forEach((type) => {
        try {
          this.createType(kit, type);
        } catch (error) {
          // Type already exists, force replace it
          this.forceReplaceType(kit, type);
        }
      });

      // Handle designs with force logic
      kit.designs?.forEach((design) => {
        try {
          this.createDesign(kit, design);
        } catch (error) {
          // Design already exists, force replace it by removing and recreating
          this.forceReplaceDesign(kit, design);
        }
      });

      console.log(`Kit "${kit.name}" updated (force mode)`);
    } catch (error) {
      // Kit doesn't exist, create it
      this.createKit(kit);
      console.log(`Kit "${kit.name}" created (force mode)`);
    }
  }

  private forceReplaceDesign(kitId: KitIdLike, design: Design): void {
    const yKit = this.getYKit(kitId);
    const designs = getKit(yKit, "designs");
    const designIds = getKit(yKit, "designIds");
    const compound = this.key.design(designIdLikeToDesignId(design));

    // Remove existing design if it exists
    const existingUuid = designIds.get(compound);
    if (existingUuid) {
      designs.delete(existingUuid);
      designIds.delete(compound);
    }

    // Create the design anew
    this.createDesign(kitIdLikeToKitId(kitId), design);
  }

  private forceReplaceType(kitId: KitIdLike, type: Type): void {
    const typeId = typeIdLikeToTypeId(type);
    const kitIdResolved = kitIdLikeToKitId(kitId);

    // Check if type exists and delete it
    try {
      this.getYType(kitIdResolved, typeId);
      this.deleteType(kitIdResolved, typeId);
    } catch {
      // Type doesn't exist, continue
    }

    // Create the type anew
    this.createType(kitIdResolved, type);
  }

  async importKit(url: string, complete = false, force = false): Promise<void> {
    // If URL ends with .json, load just the kit without files
    if (url.endsWith(".json")) {
      try {
        const response = await fetch(url);
        const kit = (await response.json()) as Kit;
        this.importKitData(kit, force);
        console.log(`Kit "${kit.name}" imported successfully from ${url}`);
        return;
      } catch (error) {
        console.error("Error importing kit from JSON:", error);
        throw error;
      }
    }

    // Original zip-based import logic
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
      const kitIdActual = { name: kit.name, version: kit.version || "" };
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
                  this.createFile(kitIdActual, representation.url, fileData);
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
      this.importKitData(kit, force);
      if (complete) {
        for (const fileEntry of Object.values(zip.files)) {
          const fileData = await fileEntry.async("uint8array");
          this.createFile(kitIdActual, fileEntry.name, fileData);
        }
      }
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
              const fileData = this.getFileData(kit, rep.url);
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

  // Observer methods for React hooks (different from update methods)
  observeKitChanges(id: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(id);
    const o = () => callback();
    (yKit as unknown as Y.Map<any>).observe(o);
    return () => (yKit as unknown as Y.Map<any>).unobserve(o);
  }

  observeDesignsChanges(id: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(id);
    const yDesigns = yKit.get("designs") as Y.Map<any>;
    const observer = () => callback();
    yDesigns.observe(observer);
    return () => yDesigns.unobserve(observer);
  }

  observeDesignChanges(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yDesign.observe(o);
    return () => yDesign.unobserve(o);
  }

  observeTypesChanges(id: KitIdLike, callback: () => void) {
    const yKit = this.getYKit(id);
    const yTypes = yKit.get("types") as Y.Map<any>;
    const observer = () => callback();
    yTypes.observe(observer);
    return () => yTypes.unobserve(observer);
  }

  observeTypeChanges(kitId: KitIdLike, id: TypeIdLike, callback: () => void) {
    const yType = this.getYType(kitId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yType.observe(o);
    return () => yType.unobserve(o);
  }

  observePiecesChanges(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id);
    const yPieces = getDesign(yDesign, "pieces") as unknown as Y.Map<any>;
    const o = () => callback();
    yPieces.observe(o);
    return () => yPieces.unobserve(o);
  }

  observePieceChanges(kitId: KitIdLike, id: DesignIdLike, pieceId: PieceIdLike, callback: () => void) {
    const yPiece = this.getYPiece(kitId, id, pieceId) as unknown as Y.Map<any>;
    const o = () => callback();
    yPiece.observe(o);
    return () => yPiece.unobserve(o);
  }

  observeConnectionsChanges(kitId: KitIdLike, id: DesignIdLike, callback: () => void) {
    const yDesign = this.getYDesign(kitId, id);
    const yConnections = getDesign(yDesign, "connections") as unknown as Y.Map<any>;
    const o = () => callback();
    yConnections.observe(o);
    return () => yConnections.unobserve(o);
  }

  observeConnectionChanges(kitId: KitIdLike, designId: DesignIdLike, id: ConnectionIdLike, callback: () => void) {
    const yConn = this.getYConnection(kitId, designId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yConn.observe(o);
    return () => yConn.unobserve(o);
  }

  observePortsChanges(kitId: KitIdLike, typeId: TypeIdLike, callback: () => void) {
    const yPorts = this.getYPorts(kitId, typeId) as unknown as Y.Map<any>;
    const o = () => callback();
    yPorts.observe(o);
    return () => yPorts.unobserve(o);
  }

  observePortChanges(kitId: KitIdLike, typeId: TypeIdLike, id: PortIdLike, callback: () => void) {
    const yPort = this.getYPort(kitId, typeId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yPort.observe(o);
    return () => yPort.unobserve(o);
  }

  observeRepresentationsChanges(kitId: KitIdLike, typeId: TypeIdLike, callback: () => void) {
    const yReps = this.getYRepresentations(kitId, typeId) as unknown as Y.Map<any>;
    const o = () => callback();
    yReps.observe(o);
    return () => yReps.unobserve(o);
  }

  observeRepresentationChanges(kitId: KitIdLike, typeId: TypeIdLike, id: RepresentationIdLike, callback: () => void) {
    const yRep = this.getYRepresentation(kitId, typeId, id) as unknown as Y.Map<any>;
    const o = () => callback();
    yRep.observe(o);
    return () => yRep.unobserve(o);
  }
}

// #region Scoping

type SketchpadScope = { id: string };
type KitScope = { id: KitId };
type DesignScope = { id: DesignId };
type TypeScope = { id: TypeId };
type PieceScope = { id: PieceId };
type ConnectionScope = { id: ConnectionId };
type RepresentationScope = { id: RepresentationId };
type PortypeScope = { id: PortId };
type DesignEditorStoreScope = { id: string };

const SketchpadScopeContext = createContext<SketchpadScope | null>(null);
const KitScopeContext = createContext<KitScope | null>(null);
const DesignScopeContext = createContext<DesignScope | null>(null);
const TypeScopeContext = createContext<TypeScope | null>(null);
const PieceScopeContext = createContext<PieceScope | null>(null);
const ConnectionScopeContext = createContext<ConnectionScope | null>(null);
const RepresentationScopeContext = createContext<RepresentationScope | null>(null);
const PortypeScopeContext = createContext<PortypeScope | null>(null);
const DesignEditorStoreScopeContext = createContext<DesignEditorStoreScope | null>(null);

const SketchpadScopeProvider = (props: { id?: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || "" }), [props.id]);
  return React.createElement(SketchpadScopeContext.Provider, { value }, props.children as any);
};

const KitScopeProvider = (props: { id?: KitId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { name: "", version: "" } }), [props.id]);
  return React.createElement(KitScopeContext.Provider, { value }, props.children as any);
};

const DesignScopeProvider = (props: { id?: DesignId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { name: "", variant: "", view: "" } }), [props.id]);
  return React.createElement(DesignScopeContext.Provider, { value }, props.children as any);
};

const TypeScopeProvider = (props: { id?: TypeId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { name: "", variant: "" } }), [props.id]);
  return React.createElement(TypeScopeContext.Provider, { value }, props.children as any);
};

const PieceScopeProvider = (props: { id?: PieceId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { id_: "" } }), [props.id]);
  return React.createElement(PieceScopeContext.Provider, { value }, props.children as any);
};

const ConnectionScopeProvider = (props: { id?: ConnectionId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { connected: { piece: { id_: "" } }, connecting: { piece: { id_: "" } } } }), [props.id]);
  return React.createElement(ConnectionScopeContext.Provider, { value }, props.children as any);
};

const RepresentationScopeProvider = (props: { id?: RepresentationId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { tags: [] } }), [props.id]);
  return React.createElement(RepresentationScopeContext.Provider, { value }, props.children as any);
};

const PortypeScopeProvider = (props: { id?: PortId; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || { id_: "" } }), [props.id]);
  return React.createElement(PortypeScopeContext.Provider, { value }, props.children as any);
};

export const DesignEditorStoreScopeProvider = (props: { id?: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || "" }), [props.id]);
  return React.createElement(DesignEditorStoreScopeContext.Provider, { value }, props.children as any);
};

const useSketchpadScope = () => useContext(SketchpadScopeContext);
const useKitScope = () => useContext(KitScopeContext);
const useDesignScope = () => useContext(DesignScopeContext);
const useTypeScope = () => useContext(TypeScopeContext);
const usePieceScope = () => useContext(PieceScopeContext);
const useConnectionScope = () => useContext(ConnectionScopeContext);
const useRepresentationScope = () => useContext(RepresentationScopeContext);
const usePortScope = () => useContext(PortypeScopeContext);
const useDesignEditorScope = () => useContext(DesignEditorStoreScopeContext);

// #endregion Scoping
