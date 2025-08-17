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
  Author,
  Camera,
  Connection,
  ConnectionDiff,
  ConnectionId,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  Kit,
  KitDiff,
  KitId,
  Location,
  Piece,
  PieceDiff,
  PieceId,
  Plane,
  Point,
  Port,
  PortDiff,
  PortId,
  Quality,
  QualityDiff,
  QualityId,
  Representation,
  RepresentationDiff,
  RepresentationId,
  Type,
  TypeDiff,
  TypeId,
  Vector,
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

type Subscribe = () => void;
type Unsubscribe = () => void;

interface QualityState {
  quality: Quality;
}

interface QualityActions {
  update: (diff: QualityDiff) => void;
}

interface QualitySubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface QualityStore extends QualityState, QualityActions, QualitySubscriptions {}

interface AuthorState {
  author: Author;
}

interface AuthorActions {
  update: (diff: Partial<Author>) => void;
}

interface AuthorSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface AuthorStore extends AuthorState, AuthorActions, AuthorSubscriptions {}

interface LocationState {
  location: Location;
}

interface LocationActions {
  update: (diff: Partial<Location>) => void;
}

interface LocationSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface LocationStore extends LocationState, LocationActions, LocationSubscriptions {}

interface RepresentationState {
  representation: Representation;
}

interface RepresentationActions {
  create: {
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    representation: (diff: RepresentationDiff) => void;
  };
  delete: {
    quality: (qualityId: QualityId) => void;
    qualities: (qualityIds: QualityId[]) => void;
  };
}

interface RepresentationSubscriptions {
  on: {
    created: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      representation: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface RepresentationChildStores {
  qualities: Map<QualityId, QualityStore>;
}

interface RepresentationStore extends RepresentationState, RepresentationActions, RepresentationSubscriptions, RepresentationChildStores {}

interface DiagramPointState {
  diagramPoint: DiagramPoint;
}

interface DiagramPointActions {
  update: (diff: Partial<DiagramPoint>) => void;
}

interface DiagramPointSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface DiagramPointStore extends DiagramPointState, DiagramPointActions, DiagramPointSubscriptions {}

interface PointState {
  point: Point;
}

interface PointActions {
  update: (diff: Partial<Point>) => void;
}

interface PointSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface PointStore extends PointState, PointActions, PointSubscriptions {}

interface VectorState {
  vector: Vector;
}

interface VectorActions {
  update: (diff: Partial<Vector>) => void;
}

interface VectorSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface VectorStore extends VectorState, VectorActions, VectorSubscriptions {}

interface PlaneState {
  plane: Plane;
}

interface PlaneActions {
  update: (diff: Partial<Plane>) => void;
}

interface PlaneSubscriptions {
  on: {
    updated: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
  };
}

interface PlaneStore extends PlaneState, PlaneActions, PlaneSubscriptions {}
interface PortState {
  port: Port;
}
interface PortActions {
  create: {
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    port: (diff: PortDiff) => void;
  };
  delete: {
    quality: (qualityId: QualityId) => void;
    qualities: (qualityIds: QualityId[]) => void;
  };
}

interface PortSubscriptions {
  on: {
    created: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      port: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface PortChildStores {
  qualities: Map<QualityId, QualityStore>;
}

interface PortStore extends PortState, PortActions, PortSubscriptions, PortChildStores {}

interface TypeState {
  type: Type;
}
interface TypeActions {
  create: {
    representation: (representation: Representation) => void;
    representations: (representations: Representation[]) => void;
    port: (port: Port) => void;
    ports: (ports: Port[]) => void;
    author: (author: Author) => void;
    authors: (authors: Author[]) => void;
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    type: (diff: TypeDiff) => void;
  };
  delete: {
    representation: (id: RepresentationId) => void;
    representations: (ids: RepresentationId[]) => void;
    port: (id: PortId) => void;
    ports: (ids: PortId[]) => void;
    author: (id: string) => void;
    authors: (ids: string[]) => void;
    quality: (id: QualityId) => void;
    qualities: (ids: QualityId[]) => void;
  };
}

interface TypeSubscriptions {
  on: {
    created: {
      representation: (subscribe: Subscribe) => Unsubscribe;
      representations: (subscribe: Subscribe) => Unsubscribe;
      port: (subscribe: Subscribe) => Unsubscribe;
      ports: (subscribe: Subscribe) => Unsubscribe;
      author: (subscribe: Subscribe) => Unsubscribe;
      authors: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      type: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      representation: (subscribe: Subscribe) => Unsubscribe;
      representations: (subscribe: Subscribe) => Unsubscribe;
      port: (subscribe: Subscribe) => Unsubscribe;
      ports: (subscribe: Subscribe) => Unsubscribe;
      author: (subscribe: Subscribe) => Unsubscribe;
      authors: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface TypeChildStores {
  representations: Map<RepresentationId, RepresentationStore>;
  ports: Map<PortId, PortStore>;
  authors: Map<string, AuthorStore>;
  qualities: Map<QualityId, QualityStore>;
}

interface TypeStore extends TypeState, TypeActions, TypeSubscriptions, TypeChildStores {}

interface PieceState {
  piece: Piece;
}

interface PieceActions {
  create: {
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    piece: (diff: PieceDiff) => void;
  };
  delete: {
    quality: (id: QualityId) => void;
    qualities: (ids: QualityId[]) => void;
  };
}

interface PieceSubscriptions {
  on: {
    created: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      piece: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface PieceChildStores {
  qualities: Map<QualityId, QualityStore>;
}

interface PieceStore extends PieceState, PieceActions, PieceSubscriptions, PieceChildStores {}

interface ConnectionState {
  connection: Connection;
}

interface ConnectionActions {
  create: {
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    connection: (diff: ConnectionDiff) => void;
  };
  delete: {
    quality: (id: QualityId) => void;
    qualities: (ids: QualityId[]) => void;
  };
}

interface ConnectionSubscriptions {
  on: {
    created: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      connection: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface ConnectionChildStores {
  qualities: Map<QualityId, QualityStore>;
}

interface ConnectionStore extends ConnectionState, ConnectionActions, ConnectionSubscriptions, ConnectionChildStores {}

interface DesignState {
  design: Design;
}

interface DesignActions {
  create: {
    piece: (piece: Piece) => void;
    pieces: (pieces: Piece[]) => void;
    connection: (connection: Connection) => void;
    connections: (connections: Connection[]) => void;
    author: (author: Author) => void;
    authors: (authors: Author[]) => void;
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    design: (diff: DesignDiff) => void;
  };
  delete: {
    piece: (id: PieceId) => void;
    pieces: (ids: PieceId[]) => void;
    connection: (id: ConnectionId) => void;
    connections: (ids: ConnectionId[]) => void;
    author: (id: string) => void;
    authors: (ids: string[]) => void;
    quality: (id: QualityId) => void;
    qualities: (ids: QualityId[]) => void;
  };
}

interface DesignSubscriptions {
  on: {
    created: {
      piece: (subscribe: Subscribe) => Unsubscribe;
      pieces: (subscribe: Subscribe) => Unsubscribe;
      connection: (subscribe: Subscribe) => Unsubscribe;
      connections: (subscribe: Subscribe) => Unsubscribe;
      author: (subscribe: Subscribe) => Unsubscribe;
      authors: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      design: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      piece: (subscribe: Subscribe) => Unsubscribe;
      pieces: (subscribe: Subscribe) => Unsubscribe;
      connection: (subscribe: Subscribe) => Unsubscribe;
      connections: (subscribe: Subscribe) => Unsubscribe;
      author: (subscribe: Subscribe) => Unsubscribe;
      authors: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface DesignChildStores {
  pieces: Map<PieceId, PieceStore>;
  connections: Map<ConnectionId, ConnectionStore>;
  authors: Map<string, AuthorStore>;
  qualities: Map<QualityId, QualityStore>;
}

interface DesignStore extends DesignState, DesignActions, DesignSubscriptions, DesignChildStores {}

interface KitState {
  kit: Kit;
}

interface KitActions {
  create: {
    type: (type: Type) => void;
    types: (types: Type[]) => void;
    design: (design: Design) => void;
    designs: (designs: Design[]) => void;
    quality: (quality: Quality) => void;
    qualities: (qualities: Quality[]) => void;
  };
  update: {
    kit: (diff: KitDiff) => void;
  };
  delete: {
    type: (id: TypeId) => void;
    types: (ids: TypeId[]) => void;
    design: (id: DesignId) => void;
    designs: (ids: DesignId[]) => void;
    quality: (id: QualityId) => void;
    qualities: (ids: QualityId[]) => void;
  };
}

interface KitSubscriptions {
  on: {
    created: {
      type: (subscribe: Subscribe) => Unsubscribe;
      types: (subscribe: Subscribe) => Unsubscribe;
      design: (subscribe: Subscribe) => Unsubscribe;
      designs: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      kit: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      type: (subscribe: Subscribe) => Unsubscribe;
      types: (subscribe: Subscribe) => Unsubscribe;
      design: (subscribe: Subscribe) => Unsubscribe;
      designs: (subscribe: Subscribe) => Unsubscribe;
      quality: (subscribe: Subscribe) => Unsubscribe;
      qualities: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface KitChildStores {
  types: Map<TypeId, TypeStore>;
  designs: Map<DesignId, DesignStore>;
  qualities: Map<QualityId, QualityStore>;
}

interface KitStore extends KitState, KitActions, KitSubscriptions, KitChildStores {}

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

interface DesignEditorState {
  id: string;
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  presence: DesignEditorPresence;
  others: DesignEditorPresenceOther[];
  diff: KitDiff;
  diffs: KitDiff[];
}

interface DesignEditorActions {
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

interface DesignEditorSubscriptions {
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

interface DesignEditorStore extends DesignEditorState, DesignEditorActions, DesignEditorSubscriptions {}

interface SketchpadState {
  id: string;
  persisted: boolean;
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
}

interface SketchpadActions {
  create: {
    kit: (kit: Kit) => void;
    kits: (kits: Kit[]) => void;
    designEditor: (designId: DesignId) => void;
    designEditors: (designIds: DesignId[]) => void;
  };
  update: {
    kit: (kitId: KitId, diff: KitDiff) => void;
  };
  delete: {
    kit: (kitId: KitId) => void;
    kits: (kitIds: KitId[]) => void;
    designEditor: (editorId: string) => void;
    designEditors: (editorIds: string[]) => void;
  };
  set: {
    mode: (mode: Mode) => void;
    theme: (theme: Theme) => void;
    layout: (layout: Layout) => void;
    activeDesignEditorId: (id?: string) => void;
  };
}

interface SketchpadSubscriptions {
  on: {
    created: {
      kit: (subscribe: Subscribe) => Unsubscribe;
      designEditors: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      sketchpad: (subscribe: Subscribe) => Unsubscribe;
      kits: (subscribe: Subscribe) => Unsubscribe;
      designEditors: (subscribe: Subscribe) => Unsubscribe;
    };
    deleted: {
      kit: (subscribe: Subscribe) => Unsubscribe;
      designEditors: (subscribe: Subscribe) => Unsubscribe;
    };
    set: {
      mode: (subscribe: Subscribe) => Unsubscribe;
      theme: (subscribe: Subscribe) => Unsubscribe;
      layout: (subscribe: Subscribe) => Unsubscribe;
      activeDesignEditorId: (subscribe: Subscribe) => Unsubscribe;
    };
  };
}

interface SketchpadChildStores {
  kits: Map<KitId, KitStore>;
  designEditors: Map<DesignId, DesignEditorStore>;
}

interface SketchpadStore extends SketchpadState, SketchpadActions, SketchpadSubscriptions, SketchpadChildStores {}

// #endregion Api

// #region Stores

// IMPLEMENTATION NOTES:
// - Updated to follow old implementation pattern: no state mirroring, direct Yjs reads
// - Removed syncSketchpadState() and private state variables (_mode, _theme, etc.)
// - State properties read directly from Yjs using typed accessors from old implementation
// - Removed single-use observer helper methods (observeKitChanges, observeDesignsChanges, etc.)
// - Uses key helper from old implementation for consistent serialization
// - CRUD operations follow old implementation patterns but need full implementation
// - Child stores still need complete implementation following similar patterns

// Yjs type definitions
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

type YSketchpadVal = string | boolean;
type YSketchpad = Y.Map<YSketchpadVal>;

// Typed key maps from old implementation
type YSketchpadKeysMap = {
  mode: string;
  theme: string;
  layout: string;
  activeDesignEditorId: string;
};
const getSketchpadStore = <K extends keyof YSketchpadKeysMap>(m: YSketchpad, k: K): YSketchpadKeysMap[K] => m.get(k as string) as YSketchpadKeysMap[K];

// Helper functions for Yjs type conversion
function createQuality(quality: Quality): YQuality {
  const yMap = new Y.Map<string>();
  yMap.set("name", quality.name);
  if (quality.value !== undefined) yMap.set("value", quality.value);
  if (quality.unit !== undefined) yMap.set("unit", quality.unit);
  if (quality.definition !== undefined) yMap.set("definition", quality.definition);
  return yMap;
}

function createQualities(qualities: Quality[] | undefined): YQualities {
  const yArr = new Y.Array<YQuality>();
  (qualities || []).forEach((q) => yArr.push([createQuality(q)]));
  return yArr;
}

function getQualities(yArr: YQualities | undefined): Quality[] {
  if (!yArr) return [];
  const list: Quality[] = [];
  yArr.forEach((yMap: YQuality) => {
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

class SketchpadStore implements SketchpadStore {
  private sketchpadDoc: Y.Doc;
  private kitDocs: Map<string, Y.Doc> = new Map();
  private sketchpadIndexeddbProvider?: IndexeddbPersistence;
  private kitIndexeddbProviders: Map<string, IndexeddbPersistence> = new Map();

  private _kits: Map<KitId, KitStore> = new Map();
  private _designEditors: Map<DesignId, DesignEditorStore> = new Map();

  // Key helper from old implementation
  private key = {
    kit: (id: KitId) => {
      if (typeof id === "string") return id;
      if (typeof id === "object" && "name" in id) {
        return `${id.name}::${id.version || ""}`;
      }
      return String(id);
    },
  };

  constructor(
    private _id: string,
    private _persisted: boolean = true,
  ) {
    this.sketchpadDoc = new Y.Doc();

    if (_persisted) {
      this.sketchpadIndexeddbProvider = new IndexeddbPersistence(`semio-sketchpad:${_id}`, this.sketchpadDoc);
    }

    // Initialize sketchpad state following old implementation
    const ySketchpad = this.getYSketchpad();
    if (!ySketchpad.has("mode")) ySketchpad.set("mode", Mode.GUEST);
    if (!ySketchpad.has("theme")) ySketchpad.set("theme", Theme.SYSTEM);
    if (!ySketchpad.has("layout")) ySketchpad.set("layout", Layout.NORMAL);
    if (!ySketchpad.has("activeDesignEditorId")) ySketchpad.set("activeDesignEditorId", "");

    this.sketchpadDoc.getMap<YDesignEditorStore>("designEditors");
  }

  // State properties - read directly from Yjs following old pattern
  get id(): string {
    return this._id;
  }
  get persisted(): boolean {
    return this._persisted;
  }
  get mode(): Mode {
    return (getSketchpadStore(this.getYSketchpad(), "mode") as Mode) || Mode.GUEST;
  }
  get theme(): Theme {
    return (getSketchpadStore(this.getYSketchpad(), "theme") as Theme) || Theme.SYSTEM;
  }
  get layout(): Layout {
    return (getSketchpadStore(this.getYSketchpad(), "layout") as Layout) || Layout.NORMAL;
  }
  get activeDesignEditorId(): string | undefined {
    const id = getSketchpadStore(this.getYSketchpad(), "activeDesignEditorId");
    return id || undefined;
  }
  get kits(): Map<KitId, KitStore> {
    return this._kits;
  }
  get designEditors(): Map<DesignId, DesignEditorStore> {
    return this._designEditors;
  }

  private getYSketchpad(): YSketchpad {
    return this.sketchpadDoc.getMap("sketchpad");
  }

  private getKitDoc(kitId: KitId): Y.Doc {
    const key = this.key.kit(kitId);
    let doc = this.kitDocs.get(key);
    if (!doc) {
      doc = new Y.Doc();
      this.kitDocs.set(key, doc);
      if (this._persisted) {
        this.kitIndexeddbProviders.set(key, new IndexeddbPersistence(`semio-kit:${this._id}:${key}`, doc));
      }
      doc.getMap<YKit>("kit");
      doc.getMap<Uint8Array>("files");
    }
    return doc;
  }

  private getYKit(kitId: KitId): YKit {
    const doc = this.getKitDoc(kitId);
    const yKit = doc.getMap<YKitVal>("kit");

    // Initialize kit if empty following old implementation
    if (yKit.size === 0) {
      const kitIdObj = typeof kitId === "object" && "name" in kitId ? kitId : { name: String(kitId), version: "" };
      yKit.set("uuid", uuidv4());
      yKit.set("name", kitIdObj.name);
      yKit.set("version", kitIdObj.version || "");
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
    }

    return yKit as YKit;
  }

  // Create actions
  create = {
    kit: (kit: Kit) => {
      if (!kit.name) throw new Error("Kit name is required to create a kit.");

      const kitId = { name: kit.name, version: kit.version };
      const yKit = this.getYKit(kitId);

      // Update Yjs structure with kit data
      yKit.set("description", kit.description || "");
      yKit.set("icon", kit.icon || "");
      yKit.set("image", kit.image || "");
      yKit.set("preview", kit.preview || "");
      yKit.set("remote", kit.remote || "");
      yKit.set("homepage", kit.homepage || "");
      yKit.set("license", kit.license || "");
      yKit.set("qualities", createQualities(kit.qualities));
      yKit.set("updated", new Date().toISOString());

      const kitStore = new KitStoreImpl(kitId, kit, this);
      this._kits.set(kitId, kitStore);
    },
    kits: (kits: Kit[]) => {
      kits.forEach((kit) => this.create.kit(kit));
    },
    designEditor: (designId: DesignId) => {
      const editorStore = new DesignEditorStoreImpl(designId, this);
      this._designEditors.set(designId, editorStore);
    },
    designEditors: (designIds: DesignId[]) => {
      designIds.forEach((id) => this.create.designEditor(id));
    },
  };

  // Update actions
  update = {
    kit: (kitId: KitId, diff: KitDiff) => {
      const store = this._kits.get(kitId);
      if (store) {
        store.update.kit(diff);
      }
    },
  };

  // Delete actions
  delete = {
    kit: (kitId: KitId) => {
      const doc = this.getKitDoc(kitId);
      const key = this.key.kit(kitId);

      // Clear the kit document following old pattern
      doc.destroy();

      // Remove from maps
      this.kitDocs.delete(key);
      const provider = this.kitIndexeddbProviders.get(key);
      if (provider) {
        provider.destroy();
        this.kitIndexeddbProviders.delete(key);
      }
    },
    kits: (kitIds: KitId[]) => {
      kitIds.forEach((id) => this.delete.kit(id));
    },
    designEditor: (editorId: string) => {
      // Find and delete by editor ID
      for (const [designId, editor] of this._designEditors.entries()) {
        if (editor.id === editorId) {
          this._designEditors.delete(designId);
          break;
        }
      }
      // Yjs will handle synchronization, no need to emit events
    },
    designEditors: (editorIds: string[]) => {
      editorIds.forEach((id) => this.delete.designEditor(id));
    },
  };

  // Set actions
  set = {
    mode: (mode: Mode) => {
      const ySketchpad = this.getYSketchpad();
      ySketchpad.set("mode", mode);
    },
    theme: (theme: Theme) => {
      const ySketchpad = this.getYSketchpad();
      ySketchpad.set("theme", theme);
    },
    layout: (layout: Layout) => {
      const ySketchpad = this.getYSketchpad();
      ySketchpad.set("layout", layout);
    },
    activeDesignEditorId: (id?: string) => {
      const ySketchpad = this.getYSketchpad();
      if (id) {
        ySketchpad.set("activeDesignEditorId", id);
      } else {
        ySketchpad.delete("activeDesignEditorId");
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
      designEditors: (subscribe: Subscribe): Unsubscribe => {
        // Observe design editor stores map changes
        const yDesignEditorStores = this.sketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
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
      designEditors: (subscribe: Subscribe): Unsubscribe => {
        const yDesignEditorStores = this.sketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
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
      designEditors: (subscribe: Subscribe): Unsubscribe => {
        const yDesignEditorStores = this.sketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
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
      activeDesignEditorId: (subscribe: Subscribe): Unsubscribe => {
        const ySketchpad = this.getYSketchpad();
        const observer = () => subscribe();
        ySketchpad.observe(observer);
        return () => ySketchpad.unobserve(observer);
      },
    },
  };
}

// Full implementations for child stores using Yjs
class QualityStoreImpl implements QualityStore {
  private yQuality: YQuality;

  constructor(
    private qualityData: Quality,
    yQuality?: YQuality,
  ) {
    if (yQuality) {
      this.yQuality = yQuality;
    } else {
      this.yQuality = new Y.Map<string>();
      this.yQuality.set("name", qualityData.name);
      this.yQuality.set("value", qualityData.value || "");
      this.yQuality.set("unit", qualityData.unit || "");
      this.yQuality.set("definition", qualityData.definition || "");
    }
  }

  get quality(): Quality {
    return {
      name: this.yQuality.get("name") as string,
      value: (this.yQuality.get("value") as string) || undefined,
      unit: (this.yQuality.get("unit") as string) || undefined,
      definition: (this.yQuality.get("definition") as string) || undefined,
    };
  }

  update = (diff: QualityDiff) => {
    if (diff.name !== undefined) this.yQuality.set("name", diff.name);
    if (diff.value !== undefined) this.yQuality.set("value", diff.value);
    if (diff.unit !== undefined) this.yQuality.set("unit", diff.unit);
    if (diff.definition !== undefined) this.yQuality.set("definition", diff.definition);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yQuality.observeDeep(observer);
        return () => this.yQuality.unobserveDeep(observer);
      } else {
        this.yQuality.observe(observer);
        return () => this.yQuality.unobserve(observer);
      }
    },
  };
}

class AuthorStoreImpl implements AuthorStore {
  private yAuthor: YAuthor;

  constructor(
    private authorData: Author,
    yAuthor?: YAuthor,
  ) {
    if (yAuthor) {
      this.yAuthor = yAuthor;
    } else {
      this.yAuthor = new Y.Map<string>();
      this.yAuthor.set("name", authorData.name);
      this.yAuthor.set("email", authorData.email || "");
    }
  }

  get author(): Author {
    return {
      name: this.yAuthor.get("name") as string,
      email: (this.yAuthor.get("email") as string) || "",
    };
  }

  update = (diff: Partial<Author>) => {
    if (diff.name !== undefined) this.yAuthor.set("name", diff.name);
    if (diff.email !== undefined) this.yAuthor.set("email", diff.email);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yAuthor.observeDeep(observer);
        return () => this.yAuthor.unobserveDeep(observer);
      } else {
        this.yAuthor.observe(observer);
        return () => this.yAuthor.unobserve(observer);
      }
    },
  };
}

class LocationStoreImpl implements LocationStore {
  private yLocation: Y.Map<number>;

  constructor(
    private locationData: Location,
    yLocation?: Y.Map<number>,
  ) {
    if (yLocation) {
      this.yLocation = yLocation;
    } else {
      this.yLocation = new Y.Map<number>();
      this.yLocation.set("latitude", locationData.latitude);
      this.yLocation.set("longitude", locationData.longitude);
    }
  }

  get location(): Location {
    return {
      latitude: this.yLocation.get("latitude") as number,
      longitude: this.yLocation.get("longitude") as number,
    };
  }

  update = (diff: Partial<Location>) => {
    if (diff.latitude !== undefined) this.yLocation.set("latitude", diff.latitude);
    if (diff.longitude !== undefined) this.yLocation.set("longitude", diff.longitude);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yLocation.observeDeep(observer);
        return () => this.yLocation.unobserveDeep(observer);
      } else {
        this.yLocation.observe(observer);
        return () => this.yLocation.unobserve(observer);
      }
    },
  };
}

class RepresentationStoreImpl implements RepresentationStore {
  private yRepresentation: YRepresentation;

  constructor(
    private representationData: Representation,
    yRepresentation?: YRepresentation,
  ) {
    if (yRepresentation) {
      this.yRepresentation = yRepresentation;
    } else {
      this.yRepresentation = new Y.Map<any>();
      this.yRepresentation.set("url", representationData.url);
      this.yRepresentation.set("description", representationData.description || "");

      const yTags = new Y.Array<string>();
      (representationData.tags || []).forEach((t) => yTags.push([t]));
      this.yRepresentation.set("tags", yTags);

      const yQualities = new Y.Array<YQuality>();
      (representationData.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      this.yRepresentation.set("qualities", yQualities);
    }
  }

  get representation(): Representation {
    const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
    return {
      url: this.yRepresentation.get("url") as string,
      description: (this.yRepresentation.get("description") as string) || "",
      tags: yTags ? yTags.toArray() : [],
      qualities: this.getQualities(this.yRepresentation.get("qualities") as YQualities),
    };
  }

  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yRepresentation.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality: YQuality) => {
        const qualityId = { name: yQuality.get("name") as string };
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    quality: (quality: Quality) => {
      const yQualities = this.yRepresentation.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

  update = {
    representation: (diff: RepresentationDiff) => {
      if (diff.url !== undefined) this.yRepresentation.set("url", diff.url);
      if (diff.description !== undefined) this.yRepresentation.set("description", diff.description);
      if (diff.tags !== undefined) {
        const yTags = this.yRepresentation.get("tags") as Y.Array<string>;
        yTags.delete(0, yTags.length);
        diff.tags.forEach((tag) => yTags.push([tag]));
      }
      if (diff.qualities !== undefined) {
        const yQualities = this.yRepresentation.get("qualities") as YQualities;
        yQualities.delete(0, yQualities.length);
        diff.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      }
    },
  };

  delete = {
    quality: (qualityId: QualityId) => {
      const yQualities = this.yRepresentation.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === qualityId.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (qualityIds: QualityId[]) => {
      qualityIds.forEach((id) => this.delete.quality(id));
    },
  };

  on = {
    created: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yRepresentation.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.created.quality(subscribe),
    },
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
    deleted: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yRepresentation.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.deleted.quality(subscribe),
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private createQualities(qualities: Quality[]): YQuality[] {
    return qualities.map((q) => this.createQuality(q));
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class PortStoreImpl implements PortStore {
  private yPort: YPort;

  constructor(
    private portData: Port,
    yPort?: YPort,
  ) {
    if (yPort) {
      this.yPort = yPort;
    } else {
      this.yPort = new Y.Map<any>();
      this.yPort.set("id_", portData.id_ || "");
      this.yPort.set("description", portData.description || "");
      this.yPort.set("mandatory", portData.mandatory || false);
      this.yPort.set("family", portData.family || "");
      this.yPort.set("t", portData.t);

      const yCompatibleFamilies = new Y.Array<string>();
      (portData.compatibleFamilies || []).forEach((f) => yCompatibleFamilies.push([f]));
      this.yPort.set("compatibleFamilies", yCompatibleFamilies);

      // Store point
      const yPoint = new Y.Map<number>();
      yPoint.set("x", portData.point.x);
      yPoint.set("y", portData.point.y);
      yPoint.set("z", portData.point.z);
      this.yPort.set("point", yPoint);

      // Store direction
      const yDirection = new Y.Map<number>();
      yDirection.set("x", portData.direction.x);
      yDirection.set("y", portData.direction.y);
      yDirection.set("z", portData.direction.z);
      this.yPort.set("direction", yDirection);

      const yQualities = new Y.Array<YQuality>();
      (portData.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      this.yPort.set("qualities", yQualities);
    }
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
      qualities: this.getQualities(this.yPort.get("qualities") as YQualities),
    };
  }

  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yPort.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality: YQuality) => {
        const qualityId = { name: yQuality.get("name") as string };
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    quality: (quality: Quality) => {
      const yQualities = this.yPort.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

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
      if (diff.qualities !== undefined) {
        const yQualities = this.yPort.get("qualities") as YQualities;
        yQualities.delete(0, yQualities.length);
        diff.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      }
    },
  };

  delete = {
    quality: (qualityId: QualityId) => {
      const yQualities = this.yPort.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === qualityId.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (qualityIds: QualityId[]) => {
      qualityIds.forEach((id) => this.delete.quality(id));
    },
  };

  on = {
    created: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yPort.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.created.quality(subscribe),
    },
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
    deleted: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yPort.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.deleted.quality(subscribe),
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private createQualities(qualities: Quality[]): YQuality[] {
    return qualities.map((q) => this.createQuality(q));
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class DiagramPointStoreImpl implements DiagramPointStore {
  private yDiagramPoint: Y.Map<number>;

  constructor(
    private diagramPointData: DiagramPoint,
    yDiagramPoint?: Y.Map<number>,
  ) {
    if (yDiagramPoint) {
      this.yDiagramPoint = yDiagramPoint;
    } else {
      this.yDiagramPoint = new Y.Map<number>();
      this.yDiagramPoint.set("x", diagramPointData.x);
      this.yDiagramPoint.set("y", diagramPointData.y);
    }
  }

  get diagramPoint(): DiagramPoint {
    return {
      x: this.yDiagramPoint.get("x") as number,
      y: this.yDiagramPoint.get("y") as number,
    };
  }

  update = (diff: Partial<DiagramPoint>) => {
    if (diff.x !== undefined) this.yDiagramPoint.set("x", diff.x);
    if (diff.y !== undefined) this.yDiagramPoint.set("y", diff.y);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yDiagramPoint.observeDeep(observer);
        return () => this.yDiagramPoint.unobserveDeep(observer);
      } else {
        this.yDiagramPoint.observe(observer);
        return () => this.yDiagramPoint.unobserve(observer);
      }
    },
  };
}

class PointStoreImpl implements PointStore {
  private yPoint: Y.Map<number>;

  constructor(
    private pointData: Point,
    yPoint?: Y.Map<number>,
  ) {
    if (yPoint) {
      this.yPoint = yPoint;
    } else {
      this.yPoint = new Y.Map<number>();
      this.yPoint.set("x", pointData.x);
      this.yPoint.set("y", pointData.y);
      this.yPoint.set("z", pointData.z);
    }
  }

  get point(): Point {
    return {
      x: this.yPoint.get("x") as number,
      y: this.yPoint.get("y") as number,
      z: this.yPoint.get("z") as number,
    };
  }

  update = (diff: Partial<Point>) => {
    if (diff.x !== undefined) this.yPoint.set("x", diff.x);
    if (diff.y !== undefined) this.yPoint.set("y", diff.y);
    if (diff.z !== undefined) this.yPoint.set("z", diff.z);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yPoint.observeDeep(observer);
        return () => this.yPoint.unobserveDeep(observer);
      } else {
        this.yPoint.observe(observer);
        return () => this.yPoint.unobserve(observer);
      }
    },
  };
}

class VectorStoreImpl implements VectorStore {
  private yVector: Y.Map<number>;

  constructor(
    private vectorData: Vector,
    yVector?: Y.Map<number>,
  ) {
    if (yVector) {
      this.yVector = yVector;
    } else {
      this.yVector = new Y.Map<number>();
      this.yVector.set("x", vectorData.x);
      this.yVector.set("y", vectorData.y);
      this.yVector.set("z", vectorData.z);
    }
  }

  get vector(): Vector {
    return {
      x: this.yVector.get("x") as number,
      y: this.yVector.get("y") as number,
      z: this.yVector.get("z") as number,
    };
  }

  update = (diff: Partial<Vector>) => {
    if (diff.x !== undefined) this.yVector.set("x", diff.x);
    if (diff.y !== undefined) this.yVector.set("y", diff.y);
    if (diff.z !== undefined) this.yVector.set("z", diff.z);
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yVector.observeDeep(observer);
        return () => this.yVector.unobserveDeep(observer);
      } else {
        this.yVector.observe(observer);
        return () => this.yVector.unobserve(observer);
      }
    },
  };
}

class PlaneStoreImpl implements PlaneStore {
  private yPlane: Y.Map<any>;

  constructor(
    private planeData: Plane,
    yPlane?: Y.Map<any>,
  ) {
    if (yPlane) {
      this.yPlane = yPlane;
    } else {
      this.yPlane = new Y.Map<any>();

      // Store origin point
      const yOrigin = new Y.Map<number>();
      yOrigin.set("x", planeData.origin.x);
      yOrigin.set("y", planeData.origin.y);
      yOrigin.set("z", planeData.origin.z);
      this.yPlane.set("origin", yOrigin);

      // Store x axis vector
      const yXAxis = new Y.Map<number>();
      yXAxis.set("x", planeData.xAxis.x);
      yXAxis.set("y", planeData.xAxis.y);
      yXAxis.set("z", planeData.xAxis.z);
      this.yPlane.set("xAxis", yXAxis);

      // Store y axis vector
      const yYAxis = new Y.Map<number>();
      yYAxis.set("x", planeData.yAxis.x);
      yYAxis.set("y", planeData.yAxis.y);
      yYAxis.set("z", planeData.yAxis.z);
      this.yPlane.set("yAxis", yYAxis);
    }
  }

  get plane(): Plane {
    const yOrigin = this.yPlane.get("origin") as Y.Map<number>;
    const yXAxis = this.yPlane.get("xAxis") as Y.Map<number>;
    const yYAxis = this.yPlane.get("yAxis") as Y.Map<number>;

    return {
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

  update = (diff: Partial<Plane>) => {
    if (diff.origin !== undefined) {
      const yOrigin = this.yPlane.get("origin") as Y.Map<number>;
      if (diff.origin.x !== undefined) yOrigin.set("x", diff.origin.x);
      if (diff.origin.y !== undefined) yOrigin.set("y", diff.origin.y);
      if (diff.origin.z !== undefined) yOrigin.set("z", diff.origin.z);
    }
    if (diff.xAxis !== undefined) {
      const yXAxis = this.yPlane.get("xAxis") as Y.Map<number>;
      if (diff.xAxis.x !== undefined) yXAxis.set("x", diff.xAxis.x);
      if (diff.xAxis.y !== undefined) yXAxis.set("y", diff.xAxis.y);
      if (diff.xAxis.z !== undefined) yXAxis.set("z", diff.xAxis.z);
    }
    if (diff.yAxis !== undefined) {
      const yYAxis = this.yPlane.get("yAxis") as Y.Map<number>;
      if (diff.yAxis.x !== undefined) yYAxis.set("x", diff.yAxis.x);
      if (diff.yAxis.y !== undefined) yYAxis.set("y", diff.yAxis.y);
      if (diff.yAxis.z !== undefined) yYAxis.set("z", diff.yAxis.z);
    }
  };

  on = {
    updated: (subscribe: Subscribe, deep?: boolean) => {
      const observer = () => subscribe();
      if (deep) {
        this.yPlane.observeDeep(observer);
        return () => this.yPlane.unobserveDeep(observer);
      } else {
        this.yPlane.observe(observer);
        return () => this.yPlane.unobserve(observer);
      }
    },
  };
}

class TypeStoreImpl implements TypeStore {
  private yType: YType;

  constructor(
    private typeData: Type,
    yType?: YType,
    private parent?: SketchpadStore,
  ) {
    if (yType) {
      this.yType = yType;
    } else {
      // Initialize basic Yjs structure for Type
      this.yType = new Y.Map<any>();
      this.yType.set("name", typeData.name);
      this.yType.set("description", typeData.description || "");
      this.yType.set("variant", typeData.variant || "");
      this.yType.set("unit", typeData.unit);
      this.yType.set("stock", typeData.stock || Number.POSITIVE_INFINITY);
      this.yType.set("virtual", typeData.virtual || false);

      // Initialize collections
      this.yType.set("representations", new Y.Map() as YRepresentationMap);
      this.yType.set("ports", new Y.Map() as YPortMap);
      this.yType.set("authors", new Y.Map() as YAuthors);
      this.yType.set("qualities", new Y.Array<YQuality>());
    }
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
      authors: Array.from(this.authors.values()).map((store) => store.author),
      qualities: this.getQualities(this.yType.get("qualities") as YQualities),
    };
  }
  get representations(): Map<RepresentationId, RepresentationStore> {
    const map = new Map<RepresentationId, RepresentationStore>();
    const yRepresentations = this.yType.get("representations") as YRepresentationMap;
    if (yRepresentations) {
      yRepresentations.forEach((yRepresentation, key) => {
        const representationData: Representation = {
          url: yRepresentation.get("url") as string,
          description: (yRepresentation.get("description") as string) || "",
          tags: (yRepresentation.get("tags") as Y.Array<string>)?.toArray() || [],
          qualities: this.getQualities(yRepresentation.get("qualities") as YQualities),
        };
        const repId = this.deserializeRepresentationId(key);
        map.set(repId, new RepresentationStoreImpl(representationData, yRepresentation));
      });
    }
    return map;
  }
  get ports(): Map<PortId, PortStore> {
    const map = new Map<PortId, PortStore>();
    const yPorts = this.yType.get("ports") as YPortMap;
    if (yPorts) {
      yPorts.forEach((yPort, portId) => {
        const yPoint = yPort.get("point") as Y.Map<number>;
        const yDirection = yPort.get("direction") as Y.Map<number>;
        const portData: Port = {
          id_: yPort.get("id_") as string,
          description: (yPort.get("description") as string) || "",
          mandatory: yPort.get("mandatory") as boolean,
          family: (yPort.get("family") as string) || "",
          compatibleFamilies: (yPort.get("compatibleFamilies") as Y.Array<string>)?.toArray() || [],
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
          t: yPort.get("t") as number,
          qualities: this.getQualities(yPort.get("qualities") as YQualities),
        };
        map.set({ id_: portId }, new PortStoreImpl(portData, yPort));
      });
    }
    return map;
  }
  get authors(): Map<string, AuthorStore> {
    const map = new Map<string, AuthorStore>();
    const yAuthors = this.yType.get("authors") as YAuthors;
    if (yAuthors) {
      yAuthors.forEach((yAuthor, authorName) => {
        const authorData: Author = {
          name: yAuthor.get("name") as string,
          email: (yAuthor.get("email") as string) || "",
        };
        map.set(authorName, new AuthorStoreImpl(authorData, yAuthor));
      });
    }
    return map;
  }
  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yType.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality) => {
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        const qualityId = { name: qualityData.name };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    representation: (representation: Representation) => {
      const yRepresentations = this.yType.get("representations") as YRepresentationMap;
      const repId = { url: representation.url, tags: representation.tags };
      const yRepresentation = new Y.Map() as YRepresentation;
      yRepresentation.set("url", representation.url);
      yRepresentation.set("description", representation.description || "");
      const yTags = new Y.Array<string>();
      (representation.tags || []).forEach((tag) => yTags.push([tag]));
      yRepresentation.set("tags", yTags);
      const yQualities = new Y.Array<YQuality>();
      (representation.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      yRepresentation.set("qualities", yQualities);
      yRepresentations.set(this.serializeRepresentationId(repId), yRepresentation);
    },
    representations: (representations: Representation[]) => {
      representations.forEach((rep) => this.create.representation(rep));
    },
    port: (port: Port) => {
      const yPorts = this.yType.get("ports") as YPortMap;
      const yPort = new Y.Map() as YPort;
      yPort.set("id_", port.id_ || "");
      yPort.set("description", port.description || "");
      yPort.set("mandatory", port.mandatory || false);
      yPort.set("family", port.family || "");
      yPort.set("t", port.t || 0);
      const yCompatibleFamilies = new Y.Array<string>();
      (port.compatibleFamilies || []).forEach((family) => yCompatibleFamilies.push([family]));
      yPort.set("compatibleFamilies", yCompatibleFamilies);
      const yPoint = new Y.Map<number>();
      yPoint.set("x", port.point.x);
      yPoint.set("y", port.point.y);
      yPoint.set("z", port.point.z);
      yPort.set("point", yPoint);
      const yDirection = new Y.Map<number>();
      yDirection.set("x", port.direction.x);
      yDirection.set("y", port.direction.y);
      yDirection.set("z", port.direction.z);
      yPort.set("direction", yDirection);
      const yQualities = new Y.Array<YQuality>();
      (port.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      yPort.set("qualities", yQualities);
      yPorts.set(port.id_ || "", yPort);
    },
    ports: (ports: Port[]) => {
      ports.forEach((port) => this.create.port(port));
    },
    author: (author: Author) => {
      const yAuthors = this.yType.get("authors") as YAuthors;
      const yAuthor = new Y.Map<string>();
      yAuthor.set("name", author.name);
      yAuthor.set("email", author.email || "");
      yAuthors.set(author.name, yAuthor);
    },
    authors: (authors: Author[]) => {
      authors.forEach((author) => this.create.author(author));
    },
    quality: (quality: Quality) => {
      const yQualities = this.yType.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
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
      const yRepresentations = this.yType.get("representations") as YRepresentationMap;
      const key = this.serializeRepresentationId(id);
      yRepresentations.delete(key);
    },
    representations: (ids: RepresentationId[]) => {
      ids.forEach((id) => this.delete.representation(id));
    },
    port: (id: PortId) => {
      const yPorts = this.yType.get("ports") as YPortMap;
      if (id.id_) {
        yPorts.delete(id.id_);
      }
    },
    ports: (ids: PortId[]) => {
      ids.forEach((id) => this.delete.port(id));
    },
    author: (id: string) => {
      const yAuthors = this.yType.get("authors") as YAuthors;
      yAuthors.delete(id);
    },
    authors: (ids: string[]) => {
      ids.forEach((id) => this.delete.author(id));
    },
    quality: (id: QualityId) => {
      const yQualities = this.yType.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === id.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (ids: QualityId[]) => {
      ids.forEach((id) => this.delete.quality(id));
    },
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
      author: (subscribe: Subscribe) => {
        const yAuthors = this.yType.get("authors") as YAuthors;
        const observer = () => subscribe();
        yAuthors.observe(observer);
        return () => yAuthors.unobserve(observer);
      },
      authors: (subscribe: Subscribe) => () => {},
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yType.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => () => {},
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
      author: (subscribe: Subscribe) => {
        const yAuthors = this.yType.get("authors") as YAuthors;
        const observer = () => subscribe();
        yAuthors.observe(observer);
        return () => yAuthors.unobserve(observer);
      },
      authors: (subscribe: Subscribe) => () => {},
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yType.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => () => {},
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private serializeRepresentationId(id: RepresentationId): string {
    if (typeof id === "object" && "tags" in id) {
      return `tags:${(id.tags || []).join(",")}`;
    }
    return String(id);
  }

  private deserializeRepresentationId(key: string): RepresentationId {
    if (key.startsWith("tags:")) {
      const tags = key
        .substring(5)
        .split(",")
        .filter((t) => t.length > 0);
      return { tags };
    }
    return { tags: [] };
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class PieceStoreImpl implements PieceStore {
  private yPiece: YPiece;

  constructor(
    private pieceData: Piece,
    yPiece?: YPiece,
  ) {
    if (yPiece) {
      this.yPiece = yPiece;
    } else {
      this.yPiece = new Y.Map<any>();
      this.yPiece.set("id_", pieceData.id_);
      this.yPiece.set("description", pieceData.description || "");

      // Store type
      const yType = new Y.Map<string>();
      yType.set("name", pieceData.type.name);
      if (pieceData.type.variant) yType.set("variant", pieceData.type.variant);
      this.yPiece.set("type", yType);

      // Store design if it exists - Note: this may be application-specific extension
      // if (pieceData.design) {
      //   const yDesign = new Y.Map<string>();
      //   yDesign.set("name", pieceData.design.name);
      //   if (pieceData.design.variant) yDesign.set("variant", pieceData.design.variant);
      //   if (pieceData.design.view) yDesign.set("view", pieceData.design.view);
      //   this.yPiece.set("design", yDesign);
      // }

      // Store plane if it exists
      if (pieceData.plane) {
        const yPlane = new Y.Map<any>();

        const yOrigin = new Y.Map<number>();
        yOrigin.set("x", pieceData.plane.origin.x);
        yOrigin.set("y", pieceData.plane.origin.y);
        yOrigin.set("z", pieceData.plane.origin.z);
        yPlane.set("origin", yOrigin);

        const yXAxis = new Y.Map<number>();
        yXAxis.set("x", pieceData.plane.xAxis.x);
        yXAxis.set("y", pieceData.plane.xAxis.y);
        yXAxis.set("z", pieceData.plane.xAxis.z);
        yPlane.set("xAxis", yXAxis);

        const yYAxis = new Y.Map<number>();
        yYAxis.set("x", pieceData.plane.yAxis.x);
        yYAxis.set("y", pieceData.plane.yAxis.y);
        yYAxis.set("z", pieceData.plane.yAxis.z);
        yPlane.set("yAxis", yYAxis);

        this.yPiece.set("plane", yPlane);
      }

      // Store center if it exists
      if (pieceData.center) {
        const yCenter = new Y.Map<number>();
        yCenter.set("x", pieceData.center.x);
        yCenter.set("y", pieceData.center.y);
        this.yPiece.set("center", yCenter);
      }

      const yQualities = new Y.Array<YQuality>();
      (pieceData.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      this.yPiece.set("qualities", yQualities);
    }
  }

  get piece(): Piece {
    const yType = this.yPiece.get("type") as Y.Map<string>;
    // const yDesign = this.yPiece.get("design") as Y.Map<string> | undefined;
    const yPlane = this.yPiece.get("plane") as Y.Map<any> | undefined;
    const yCenter = this.yPiece.get("center") as Y.Map<number> | undefined;

    const piece: Piece = {
      id_: this.yPiece.get("id_") as string,
      description: (this.yPiece.get("description") as string) || "",
      type: {
        name: yType.get("name") as string,
        variant: yType.get("variant") as string | undefined,
      },
      qualities: this.getQualities(this.yPiece.get("qualities") as YQualities),
    };

    // if (yDesign) {
    //   piece.design = {
    //     name: yDesign.get("name") as string,
    //     variant: yDesign.get("variant") as string | undefined,
    //     view: yDesign.get("view") as string | undefined,
    //   };
    // }

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

  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yPiece.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality: YQuality) => {
        const qualityId = { name: yQuality.get("name") as string };
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    quality: (quality: Quality) => {
      const yQualities = this.yPiece.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

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
      if (diff.qualities !== undefined) {
        const yQualities = this.yPiece.get("qualities") as YQualities;
        yQualities.delete(0, yQualities.length);
        diff.qualities.forEach((q) => yQualities.push([this.createQuality(q)]));
      }
    },
  };

  delete = {
    quality: (qualityId: QualityId) => {
      const yQualities = this.yPiece.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === qualityId.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (qualityIds: QualityId[]) => {
      qualityIds.forEach((id) => this.delete.quality(id));
    },
  };

  on = {
    created: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yPiece.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.created.quality(subscribe),
    },
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
    deleted: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yPiece.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.deleted.quality(subscribe),
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class ConnectionStoreImpl implements ConnectionStore {
  private yConnection: YConnection;

  constructor(
    private connectionData: Connection,
    yConnection?: YConnection,
  ) {
    if (yConnection) {
      this.yConnection = yConnection;
    } else {
      this.yConnection = new Y.Map<any>();

      // Store connected side
      const yConnected = new Y.Map<any>();
      const yConnectedPiece = new Y.Map<string>();
      yConnectedPiece.set("id_", connectionData.connected.piece.id_);
      yConnected.set("piece", yConnectedPiece);

      const yConnectedPort = new Y.Map<string>();
      yConnectedPort.set("id_", connectionData.connected.port.id_ || "");
      yConnected.set("port", yConnectedPort);

      if (connectionData.connected.designId) {
        yConnected.set("designId", connectionData.connected.designId);
      }
      this.yConnection.set("connected", yConnected);

      // Store connecting side
      const yConnecting = new Y.Map<any>();
      const yConnectingPiece = new Y.Map<string>();
      yConnectingPiece.set("id_", connectionData.connecting.piece.id_);
      yConnecting.set("piece", yConnectingPiece);

      const yConnectingPort = new Y.Map<string>();
      yConnectingPort.set("id_", connectionData.connecting.port.id_ || "");
      yConnecting.set("port", yConnectingPort);

      if (connectionData.connecting.designId) {
        yConnecting.set("designId", connectionData.connecting.designId);
      }
      this.yConnection.set("connecting", yConnecting);

      // Store other properties
      this.yConnection.set("description", connectionData.description || "");
      this.yConnection.set("gap", connectionData.gap || 0);
      this.yConnection.set("shift", connectionData.shift || 0);
      this.yConnection.set("rise", connectionData.rise || 0);
      this.yConnection.set("rotation", connectionData.rotation || 0);
      this.yConnection.set("turn", connectionData.turn || 0);
      this.yConnection.set("tilt", connectionData.tilt || 0);
      this.yConnection.set("x", connectionData.x || 0);
      this.yConnection.set("y", connectionData.y || 0);

      const yQualities = new Y.Array<YQuality>();
      (connectionData.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      this.yConnection.set("qualities", yQualities);
    }
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
      qualities: this.getQualities(this.yConnection.get("qualities") as YQualities),
    };
  }

  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yConnection.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality: YQuality) => {
        const qualityId = { name: yQuality.get("name") as string };
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    quality: (quality: Quality) => {
      const yQualities = this.yConnection.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

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

  delete = {
    quality: (qualityId: QualityId) => {
      const yQualities = this.yConnection.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === qualityId.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (qualityIds: QualityId[]) => {
      qualityIds.forEach((id) => this.delete.quality(id));
    },
  };

  on = {
    created: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yConnection.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.created.quality(subscribe),
    },
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
    deleted: {
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yConnection.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.deleted.quality(subscribe),
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class DesignStoreImpl implements DesignStore {
  private yDesign: YDesign;

  constructor(
    private designData: Design,
    yDesign?: YDesign,
    private parent?: SketchpadStore,
  ) {
    if (yDesign) {
      this.yDesign = yDesign;
    } else {
      this.yDesign = new Y.Map<any>();
      this.yDesign.set("name", designData.name);
      this.yDesign.set("description", designData.description || "");
      this.yDesign.set("variant", designData.variant || "");
      this.yDesign.set("view", designData.view || "");

      // Initialize collections
      this.yDesign.set("pieces", new Y.Map() as YPieceMap);
      this.yDesign.set("connections", new Y.Map() as YConnectionMap);
      this.yDesign.set("authors", new Y.Map() as YAuthors);
      this.yDesign.set("qualities", new Y.Array<YQuality>());
    }
  }

  get design(): Design {
    const yPieces = this.yDesign.get("pieces") as YPieceMap;
    const yConnections = this.yDesign.get("connections") as YConnectionMap;
    const yAuthors = this.yDesign.get("authors") as YAuthors;
    const yQualities = this.yDesign.get("qualities") as YQualities;

    const pieces: Piece[] = [];
    if (yPieces) {
      yPieces.forEach((yPiece) => {
        const yType = yPiece.get("type") as Y.Map<string>;
        const yPlane = yPiece.get("plane") as Y.Map<any> | undefined;
        const yCenter = yPiece.get("center") as Y.Map<number> | undefined;
        const yPieceQualities = yPiece.get("qualities") as Y.Array<YQuality> | undefined;

        const piece: Piece = {
          id_: yPiece.get("id_") as string,
          description: (yPiece.get("description") as string) || "",
          type: {
            name: yType.get("name") as string,
            variant: (yType.get("variant") as string) || undefined,
          },
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

        if (yPieceQualities) {
          piece.qualities = this.getQualities(yPieceQualities);
        }

        pieces.push(piece);
      });
    }

    const connections: Connection[] = [];
    if (yConnections) {
      yConnections.forEach((yConnection) => {
        const yConnected = yConnection.get("connected") as Y.Map<any>;
        const yConnecting = yConnection.get("connecting") as Y.Map<any>;
        const yConnectedPiece = yConnected.get("piece") as Y.Map<string>;
        const yConnectedPort = yConnected.get("port") as Y.Map<string>;
        const yConnectingPiece = yConnecting.get("piece") as Y.Map<string>;
        const yConnectingPort = yConnecting.get("port") as Y.Map<string>;
        const yConnectionQualities = yConnection.get("qualities") as Y.Array<YQuality> | undefined;

        const connection: Connection = {
          connected: {
            piece: { id_: yConnectedPiece.get("id_") as string },
            port: { id_: (yConnectedPort.get("id_") as string) || "" },
            designId: (yConnected.get("designId") as string) || undefined,
          },
          connecting: {
            piece: { id_: yConnectingPiece.get("id_") as string },
            port: { id_: (yConnectingPort.get("id_") as string) || "" },
            designId: (yConnecting.get("designId") as string) || undefined,
          },
          description: (yConnection.get("description") as string) || "",
          gap: (yConnection.get("gap") as number) || 0,
          shift: (yConnection.get("shift") as number) || 0,
          rise: (yConnection.get("rise") as number) || 0,
          rotation: (yConnection.get("rotation") as number) || 0,
          turn: (yConnection.get("turn") as number) || 0,
          tilt: (yConnection.get("tilt") as number) || 0,
          x: (yConnection.get("x") as number) || 0,
          y: (yConnection.get("y") as number) || 0,
        };

        if (yConnectionQualities) {
          connection.qualities = this.getQualities(yConnectionQualities);
        }

        connections.push(connection);
      });
    }

    const authors: Author[] = [];
    if (yAuthors) {
      yAuthors.forEach((yAuthor) => {
        authors.push({
          name: yAuthor.get("name") as string,
          email: (yAuthor.get("email") as string) || "",
        });
      });
    }

    return {
      name: this.yDesign.get("name") as string,
      description: (this.yDesign.get("description") as string) || "",
      icon: (this.yDesign.get("icon") as string) || undefined,
      image: (this.yDesign.get("image") as string) || undefined,
      variant: (this.yDesign.get("variant") as string) || "",
      view: (this.yDesign.get("view") as string) || "",
      unit: (this.yDesign.get("unit") as string) || "mm",
      created: this.yDesign.get("created") as Date | undefined,
      updated: this.yDesign.get("updated") as Date | undefined,
      pieces,
      connections,
      authors,
      qualities: this.getQualities(yQualities),
    };
  }
  get pieces(): Map<PieceId, PieceStore> {
    const map = new Map<PieceId, PieceStore>();
    const yPieces = this.yDesign.get("pieces") as YPieceMap;
    if (yPieces) {
      yPieces.forEach((yPiece, pieceId) => {
        const yType = yPiece.get("type") as Y.Map<string>;
        const yPlane = yPiece.get("plane") as Y.Map<any> | undefined;
        const yCenter = yPiece.get("center") as Y.Map<number> | undefined;

        const pieceData: Piece = {
          id_: yPiece.get("id_") as string,
          description: (yPiece.get("description") as string) || "",
          type: {
            name: yType.get("name") as string,
            variant: yType.get("variant") as string | undefined,
          },
          qualities: this.getQualities(yPiece.get("qualities") as YQualities),
        };

        if (yPlane) {
          const yOrigin = yPlane.get("origin") as Y.Map<number>;
          const yXAxis = yPlane.get("xAxis") as Y.Map<number>;
          const yYAxis = yPlane.get("yAxis") as Y.Map<number>;
          pieceData.plane = {
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
          pieceData.center = {
            x: yCenter.get("x") as number,
            y: yCenter.get("y") as number,
          };
        }

        map.set({ id_: pieceId }, new PieceStoreImpl(pieceData, yPiece));
      });
    }
    return map;
  }
  get connections(): Map<ConnectionId, ConnectionStore> {
    const map = new Map<ConnectionId, ConnectionStore>();
    const yConnections = this.yDesign.get("connections") as YConnectionMap;
    if (yConnections) {
      yConnections.forEach((yConnection, connectionKey) => {
        const yConnected = yConnection.get("connected") as Y.Map<any>;
        const yConnecting = yConnection.get("connecting") as Y.Map<any>;
        const yConnectedPiece = yConnected.get("piece") as Y.Map<string>;
        const yConnectedPort = yConnected.get("port") as Y.Map<string>;
        const yConnectingPiece = yConnecting.get("piece") as Y.Map<string>;
        const yConnectingPort = yConnecting.get("port") as Y.Map<string>;

        const connectionData: Connection = {
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
          description: (yConnection.get("description") as string) || "",
          gap: yConnection.get("gap") as number,
          shift: yConnection.get("shift") as number,
          rise: yConnection.get("rise") as number,
          rotation: yConnection.get("rotation") as number,
          turn: yConnection.get("turn") as number,
          tilt: yConnection.get("tilt") as number,
          x: yConnection.get("x") as number,
          y: yConnection.get("y") as number,
          qualities: this.getQualities(yConnection.get("qualities") as YQualities),
        };

        const connectionId = this.deserializeConnectionId(connectionKey);
        map.set(connectionId, new ConnectionStoreImpl(connectionData, yConnection));
      });
    }
    return map;
  }
  get authors(): Map<string, AuthorStore> {
    const map = new Map<string, AuthorStore>();
    const yAuthors = this.yDesign.get("authors") as YAuthors;
    if (yAuthors) {
      yAuthors.forEach((yAuthor, authorName) => {
        const authorData: Author = {
          name: yAuthor.get("name") as string,
          email: (yAuthor.get("email") as string) || "",
        };
        map.set(authorName, new AuthorStoreImpl(authorData, yAuthor));
      });
    }
    return map;
  }
  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yDesign.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality) => {
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        const qualityId = { name: qualityData.name };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    piece: (piece: Piece) => {
      const yPieces = this.yDesign.get("pieces") as YPieceMap;
      const yPiece = new Y.Map() as YPiece;
      yPiece.set("id_", piece.id_);
      yPiece.set("description", piece.description || "");
      const yType = new Y.Map<string>();
      yType.set("name", piece.type.name);
      if (piece.type.variant) yType.set("variant", piece.type.variant);
      yPiece.set("type", yType);
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
        yPiece.set("plane", yPlane);
      }
      if (piece.center) {
        const yCenter = new Y.Map<number>();
        yCenter.set("x", piece.center.x);
        yCenter.set("y", piece.center.y);
        yPiece.set("center", yCenter);
      }
      const yQualities = new Y.Array<YQuality>();
      (piece.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      yPiece.set("qualities", yQualities);
      yPieces.set(piece.id_, yPiece);
    },
    pieces: (pieces: Piece[]) => {
      pieces.forEach((piece) => this.create.piece(piece));
    },
    connection: (connection: Connection) => {
      const yConnections = this.yDesign.get("connections") as YConnectionMap;
      const yConnection = new Y.Map() as YConnection;
      const yConnected = new Y.Map<any>();
      const yConnectedPiece = new Y.Map<string>();
      yConnectedPiece.set("id_", connection.connected.piece.id_);
      yConnected.set("piece", yConnectedPiece);
      const yConnectedPort = new Y.Map<string>();
      yConnectedPort.set("id_", connection.connected.port.id_ || "");
      yConnected.set("port", yConnectedPort);
      if (connection.connected.designId) {
        yConnected.set("designId", connection.connected.designId);
      }
      yConnection.set("connected", yConnected);
      const yConnecting = new Y.Map<any>();
      const yConnectingPiece = new Y.Map<string>();
      yConnectingPiece.set("id_", connection.connecting.piece.id_);
      yConnecting.set("piece", yConnectingPiece);
      const yConnectingPort = new Y.Map<string>();
      yConnectingPort.set("id_", connection.connecting.port.id_ || "");
      yConnecting.set("port", yConnectingPort);
      if (connection.connecting.designId) {
        yConnecting.set("designId", connection.connecting.designId);
      }
      yConnection.set("connecting", yConnecting);
      yConnection.set("description", connection.description || "");
      yConnection.set("gap", connection.gap || 0);
      yConnection.set("shift", connection.shift || 0);
      yConnection.set("rise", connection.rise || 0);
      yConnection.set("rotation", connection.rotation || 0);
      yConnection.set("turn", connection.turn || 0);
      yConnection.set("tilt", connection.tilt || 0);
      yConnection.set("x", connection.x || 0);
      yConnection.set("y", connection.y || 0);
      const yQualities = new Y.Array<YQuality>();
      (connection.qualities || []).forEach((q) => yQualities.push([this.createQuality(q)]));
      yConnection.set("qualities", yQualities);
      const connectionId = this.serializeConnectionId({ connected: connection.connected, connecting: connection.connecting });
      yConnections.set(connectionId, yConnection);
    },
    connections: (connections: Connection[]) => {
      connections.forEach((connection) => this.create.connection(connection));
    },
    author: (author: Author) => {
      const yAuthors = this.yDesign.get("authors") as YAuthors;
      const yAuthor = new Y.Map<string>();
      yAuthor.set("name", author.name);
      yAuthor.set("email", author.email || "");
      yAuthors.set(author.name, yAuthor);
    },
    authors: (authors: Author[]) => {
      authors.forEach((author) => this.create.author(author));
    },
    quality: (quality: Quality) => {
      const yQualities = this.yDesign.get("qualities") as YQualities;
      const yQuality = this.createQuality(quality);
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

  update = {
    design: (diff: DesignDiff) => {
      if (diff.name !== undefined) this.yDesign.set("name", diff.name);
      if (diff.description !== undefined) this.yDesign.set("description", diff.description);
      if (diff.variant !== undefined) this.yDesign.set("variant", diff.variant);
      if (diff.view !== undefined) this.yDesign.set("view", diff.view);
    },
  };

  delete = {
    piece: (id: PieceId) => {
      const yPieces = this.yDesign.get("pieces") as YPieceMap;
      yPieces.delete(id.id_);
    },
    pieces: (ids: PieceId[]) => {
      ids.forEach((id) => this.delete.piece(id));
    },
    connection: (id: ConnectionId) => {
      const yConnections = this.yDesign.get("connections") as YConnectionMap;
      const key = this.serializeConnectionId(id);
      yConnections.delete(key);
    },
    connections: (ids: ConnectionId[]) => {
      ids.forEach((id) => this.delete.connection(id));
    },
    author: (id: string) => {
      const yAuthors = this.yDesign.get("authors") as YAuthors;
      yAuthors.delete(id);
    },
    authors: (ids: string[]) => {
      ids.forEach((id) => this.delete.author(id));
    },
    quality: (id: QualityId) => {
      const yQualities = this.yDesign.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === id.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (ids: QualityId[]) => {
      ids.forEach((id) => this.delete.quality(id));
    },
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
      author: (subscribe: Subscribe) => {
        const yAuthors = this.yDesign.get("authors") as YAuthors;
        const observer = () => subscribe();
        yAuthors.observe(observer);
        return () => yAuthors.unobserve(observer);
      },
      authors: (subscribe: Subscribe) => () => {},
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yDesign.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => () => {},
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
      author: (subscribe: Subscribe) => {
        const yAuthors = this.yDesign.get("authors") as YAuthors;
        const observer = () => subscribe();
        yAuthors.observe(observer);
        return () => yAuthors.unobserve(observer);
      },
      authors: (subscribe: Subscribe) => () => {},
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yDesign.get("qualities") as YQualities;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => () => {},
    },
  };

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map<string>();
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private serializeConnectionId(id: ConnectionId): string {
    const connectedPart = id.connected.piece.id_;
    const connectingPart = id.connecting.piece.id_;
    return `${connectedPart}→${connectingPart}`;
  }

  private deserializeConnectionId(key: string): ConnectionId {
    const parts = key.split("→");
    return {
      connected: { piece: { id_: parts[0] || "" } },
      connecting: { piece: { id_: parts[1] || "" } },
    };
  }

  private getQualities(yQualities: YQualities | undefined): Quality[] {
    if (!yQualities) return [];
    const list: Quality[] = [];
    yQualities.forEach((yQuality: YQuality) => {
      list.push({
        name: yQuality.get("name") as string,
        value: yQuality.get("value") as string | undefined,
        unit: yQuality.get("unit") as string | undefined,
        definition: yQuality.get("definition") as string | undefined,
      });
    });
    return list;
  }
}

class KitStoreImpl implements KitStore {
  private yKit: YKit;
  private _types: Map<TypeId, TypeStore> = new Map();
  private _designs: Map<DesignId, DesignStore> = new Map();
  private _qualities: Map<QualityId, QualityStore> = new Map();

  constructor(
    private kitId: KitId,
    private kitData: Kit,
    private parent: SketchpadStore,
  ) {
    this.yKit = (parent as any).getYKit(kitId);

    // Initialize kit data if not present
    this.initializeKitData();
  }

  private initializeKitData(): void {
    // Update Yjs store with kit data
    this.yKit.set("name", this.kitData.name);
    this.yKit.set("description", this.kitData.description || "");
    this.yKit.set("version", this.kitData.version || "");
    this.yKit.set("updated", new Date().toISOString());

    // Initialize types if not present
    if (!this.yKit.has("types")) {
      this.yKit.set("types", new Y.Map<YType>());
    }

    // Initialize designs if not present
    if (!this.yKit.has("designs")) {
      this.yKit.set("designs", new Y.Map<YDesign>());
    }

    // Initialize qualities if not present
    if (!this.yKit.has("qualities")) {
      this.yKit.set("qualities", new Y.Array<YQuality>());
      // Add initial kit qualities
      if (this.kitData.qualities) {
        const yQualities = this.yKit.get("qualities") as YQualities;
        this.kitData.qualities.forEach((q) => {
          yQualities.push([this.createQuality(q)]);
        });
      }
    }
  }

  private createQuality(quality: Quality): YQuality {
    const yQuality = new Y.Map() as YQuality;
    yQuality.set("name", quality.name);
    if (quality.value !== undefined) yQuality.set("value", quality.value);
    if (quality.unit !== undefined) yQuality.set("unit", quality.unit);
    if (quality.definition !== undefined) yQuality.set("definition", quality.definition);
    return yQuality;
  }

  private deserializeTypeId(typeKey: string): TypeId {
    return { name: typeKey };
  }

  private deserializeDesignId(designKey: string): DesignId {
    return { name: designKey };
  }

  private getQualities(yQualities?: YQualities): Quality[] {
    if (!yQualities) return [];
    return yQualities.toArray().map((yQuality) => ({
      name: yQuality.get("name") as string,
      value: yQuality.get("value") as string | undefined,
      unit: yQuality.get("unit") as string | undefined,
      definition: yQuality.get("definition") as string | undefined,
    }));
  }

  get kit(): Kit {
    // In a full implementation, this would reconstruct the Kit from Yjs data
    return this.kitData;
  }
  get types(): Map<TypeId, TypeStore> {
    const map = new Map<TypeId, TypeStore>();
    const yTypes = this.yKit.get("types") as YTypeMap;
    if (yTypes) {
      yTypes.forEach((yType, typeKey) => {
        const typeData: Type = {
          name: yType.get("name") as string,
          description: (yType.get("description") as string) || "",
          variant: yType.get("variant") as string | undefined,
          unit: (yType.get("unit") as string) || "",
          stock: yType.get("stock") as number | undefined,
          virtual: yType.get("virtual") as boolean | undefined,
          representations: [],
          ports: [],
          authors: [],
          qualities: this.getQualities(yType.get("qualities") as YQualities),
        };
        const typeId = this.deserializeTypeId(typeKey);
        map.set(typeId, new TypeStoreImpl(typeData, yType, this.parent));
      });
    }
    return map;
  }
  get designs(): Map<DesignId, DesignStore> {
    const map = new Map<DesignId, DesignStore>();
    const yDesigns = this.yKit.get("designs") as YDesignMap;
    if (yDesigns) {
      yDesigns.forEach((yDesign, designKey) => {
        const designData: Design = {
          name: yDesign.get("name") as string,
          unit: (yDesign.get("unit") as string) || "",
          description: (yDesign.get("description") as string) || "",
          variant: yDesign.get("variant") as string | undefined,
          view: yDesign.get("view") as string | undefined,
          pieces: [],
          connections: [],
          authors: [],
          qualities: this.getQualities(yDesign.get("qualities") as YQualities),
        };
        const designId = this.deserializeDesignId(designKey);
        map.set(designId, new DesignStoreImpl(designData, yDesign, this.parent));
      });
    }
    return map;
  }
  get qualities(): Map<QualityId, QualityStore> {
    const map = new Map<QualityId, QualityStore>();
    const yQualities = this.yKit.get("qualities") as YQualities;
    if (yQualities) {
      yQualities.forEach((yQuality) => {
        const qualityData: Quality = {
          name: yQuality.get("name") as string,
          value: yQuality.get("value") as string | undefined,
          unit: yQuality.get("unit") as string | undefined,
          definition: yQuality.get("definition") as string | undefined,
        };
        const qualityId = { name: qualityData.name };
        map.set(qualityId, new QualityStoreImpl(qualityData, yQuality));
      });
    }
    return map;
  }

  create = {
    type: (type: Type) => {
      const yTypes = this.yKit.get("types") as Y.Map<YType>;
      const yType = new Y.Map() as YType;
      yType.set("name", type.name);
      yType.set("description", type.description || "");
      yType.set("variant", type.variant || "");
      yType.set("unit", type.unit);
      yType.set("stock", type.stock || Number.POSITIVE_INFINITY);
      yType.set("virtual", type.virtual || false);
      yType.set("representations", new Y.Map() as YRepresentationMap);
      yType.set("ports", new Y.Map() as YPortMap);
      yType.set("authors", createAuthors(type.authors));
      yType.set("qualities", createQualities(type.qualities));
      const key = typeof type === "object" ? `${type.name}::${type.variant || ""}` : String(type);
      yTypes.set(key, yType);
    },
    types: (types: Type[]) => {
      types.forEach((type) => this.create.type(type));
    },
    design: (design: Design) => {
      const yDesigns = this.yKit.get("designs") as Y.Map<YDesign>;
      const yDesign = new Y.Map() as YDesign;
      yDesign.set("name", design.name);
      yDesign.set("unit", design.unit);
      yDesign.set("description", design.description || "");
      yDesign.set("variant", design.variant || "");
      yDesign.set("view", design.view || "");
      yDesign.set("pieces", new Y.Map() as YPieceMap);
      yDesign.set("connections", new Y.Map() as YConnectionMap);
      yDesign.set("authors", createAuthors(design.authors));
      yDesign.set("qualities", createQualities(design.qualities));
      const key = typeof design === "object" ? `${design.name}::${design.variant || ""}::${design.view || ""}` : String(design);
      yDesigns.set(key, yDesign);
    },
    designs: (designs: Design[]) => {
      designs.forEach((design) => this.create.design(design));
    },
    quality: (quality: Quality) => {
      const yQualities = this.yKit.get("qualities") as Y.Array<YQuality>;
      const yQuality = new Y.Map<string>();
      yQuality.set("name", quality.name);
      yQuality.set("value", quality.value || "");
      yQuality.set("unit", quality.unit || "");
      yQuality.set("definition", quality.definition || "");
      yQualities.push([yQuality]);
    },
    qualities: (qualities: Quality[]) => {
      qualities.forEach((quality) => this.create.quality(quality));
    },
  };

  update = {
    kit: (diff: KitDiff) => {
      if (diff.name) this.yKit.set("name", diff.name);
      if (diff.description) this.yKit.set("description", diff.description);
      if (diff.version) this.yKit.set("version", diff.version);
      this.yKit.set("updated", new Date().toISOString());
    },
  };

  delete = {
    type: (id: TypeId) => {
      const yTypes = this.yKit.get("types") as Y.Map<YType>;
      // Create a simple string key for type ID
      const key = typeof id === "string" ? id : `${id.name}::${id.variant || ""}`;
      yTypes.delete(key);
    },
    types: (ids: TypeId[]) => {
      ids.forEach((id) => this.delete.type(id));
    },
    design: (id: DesignId) => {
      const yDesigns = this.yKit.get("designs") as Y.Map<YDesign>;
      const key = typeof id === "string" ? id : `${id.name}::${id.variant || ""}::${id.view || ""}`;
      yDesigns.delete(key);
    },
    designs: (ids: DesignId[]) => {
      ids.forEach((id) => this.delete.design(id));
    },
    quality: (id: QualityId) => {
      const yQualities = this.yKit.get("qualities") as YQualities;
      for (let i = 0; i < yQualities.length; i++) {
        const yQuality = yQualities.get(i) as YQuality;
        if (yQuality.get("name") === id.name) {
          yQualities.delete(i, 1);
          break;
        }
      }
    },
    qualities: (ids: QualityId[]) => {
      ids.forEach((id) => this.delete.quality(id));
    },
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
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yKit.get("qualities") as Y.Array<YQuality>;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.created.quality(subscribe),
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
      quality: (subscribe: Subscribe) => {
        const yQualities = this.yKit.get("qualities") as Y.Array<YQuality>;
        const observer = () => subscribe();
        yQualities.observe(observer);
        return () => yQualities.unobserve(observer);
      },
      qualities: (subscribe: Subscribe) => this.on.deleted.quality(subscribe),
    },
  };
}

class DesignEditorStoreImpl implements DesignEditorStore {
  private yDesignEditorStore: YDesignEditorStore;

  constructor(
    private designId: DesignId,
    private parent: SketchpadStore,
  ) {
    // Get or create the Yjs store for this design editor
    const sketchpadDoc = (parent as any).sketchpadDoc as Y.Doc;
    const yDesignEditorStores = sketchpadDoc.getMap("designEditorStores") as YDesignEditorStoreMap;
    const editorKey = this.serializeDesignId(designId);

    if (!yDesignEditorStores.has(editorKey)) {
      const yStore = new Y.Map() as YDesignEditorStore;
      yStore.set("fullscreenPanel", DesignEditorFullscreenPanel.None);
      yStore.set("selectedPieceIds", new Y.Array<string>());
      yStore.set("selectedConnections", new Y.Array<string>());
      yStore.set("isTransactionActive", false);
      yStore.set("presenceCursorX", 0);
      yStore.set("presenceCursorY", 0);
      yDesignEditorStores.set(editorKey, yStore);
    }

    this.yDesignEditorStore = yDesignEditorStores.get(editorKey) as YDesignEditorStore;
  }

  get id(): string {
    return this.serializeDesignId(this.designId);
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
  get diffs(): KitDiff[] {
    return [];
  }

  private serializeDesignId(designId: DesignId): string {
    if (typeof designId === "string") return designId;
    if (typeof designId === "object" && "name" in designId) {
      let result = designId.name;
      if (designId.variant) result += `@${designId.variant}`;
      if (designId.view) result += `#${designId.view}`;
      return result;
    }
    return String(designId);
  }

  undo = () => {
    // Placeholder implementation - undo/redo would need proper state tracking
  };

  redo = () => {
    // Placeholder implementation - undo/redo would need proper state tracking
  };

  private invertKitDiff(diff: KitDiff): KitDiff {
    // This would need proper implementation to invert diffs
    return {};
  }

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
}

const stores: Map<string, SketchpadStore> = new Map();

// Factory function to create or get a store
export function getOrCreateSketchpadStore(id: string, persisted: boolean = true): SketchpadStore {
  if (!stores.has(id)) {
    stores.set(id, new SketchpadStore(id, persisted));
  }
  return stores.get(id)!;
}

// #endregion Stores

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

export function useSketchpad(): SketchpadState;
export function useSketchpad<T>(selector: (sketchpad: SketchpadState) => T, id?: string): T;
export function useSketchpad<T>(selector?: (sketchpad: SketchpadState) => T, id?: string): T | SketchpadState {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  return useStore(store, store.on.updated.sketchpad, selector);
}

export function useDesignEditor(): DesignEditorState;
export function useDesignEditor<T>(selector: (editor: DesignEditorState) => T): T;
export function useDesignEditor<T>(selector: (editor: DesignEditorState) => T, id: DesignId): T;
export function useDesignEditor<T>(selector?: (editor: DesignEditorState) => T, id?: DesignId): T | DesignEditorState {
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

export function useDesignEditors(): Map<DesignId, DesignEditorStore>;
export function useDesignEditors<T>(selector: (editors: Map<DesignId, DesignEditorStore>) => T): T;
export function useDesignEditors<T>(selector?: (editors: Map<DesignId, DesignEditorStore>) => T): T | Map<DesignId, DesignEditorStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesignEditors must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  return useStore(store.designEditors, store.on.updated.designEditors, selector);
}

export function useKit(): Kit;
export function useKit<T>(selector: (kit: Kit) => T): T;
export function useKit<T>(selector: (kit: Kit) => T, id: KitId): T;
export function useKit<T>(selector?: (kit: Kit) => T, id?: KitId): T | Kit {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKit must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  const kitId = kitScope?.id ?? id;
  if (!kitId) throw new Error("useKit must be called within a KitScopeProvider or be directly provided with an id");
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore.kit, kitStore.on.updated.kit, selector);
}

export function useKits(): Map<KitId, KitStore>;
export function useKits<T>(selector: (kits: Map<KitId, KitStore>) => T): T;
export function useKits<T>(selector?: (kits: Map<KitId, KitStore>) => T): T | Map<KitId, KitStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKits must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  return useStore(store.kits, store.on.updated.kits, selector);
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

export function useDesigns(): Map<DesignId, DesignStore>;
export function useDesigns<T>(selector: (designs: Map<DesignId, DesignStore>) => T): T;
export function useDesigns<T>(selector?: (designs: Map<DesignId, DesignStore>) => T): T | Map<DesignId, DesignStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesigns must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useDesigns must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore.designs, kitStore.on.updated.kit, selector);
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

export function useTypes(): Map<TypeId, TypeStore>;
export function useTypes<T>(selector: (types: Map<TypeId, TypeStore>) => T): T;
export function useTypes<T>(selector?: (types: Map<TypeId, TypeStore>) => T): T | Map<TypeId, TypeStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useTypes must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useTypes must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore.types, kitStore.on.updated.kit, selector);
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

export function usePieces(): Map<PieceId, PieceStore>;
export function usePieces<T>(selector: (pieces: Map<PieceId, PieceStore>) => T): T;
export function usePieces<T>(selector?: (pieces: Map<PieceId, PieceStore>) => T): T | Map<PieceId, PieceStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePieces must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePieces must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("usePieces must be called within a DesignScopeProvider");
  const designId = designScope.id;
  if (!kit.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kit.designs.get(designId)!;
  return useStore(designStore.pieces, designStore.on.updated.design, selector);
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

export function useConnections(): Map<ConnectionId, ConnectionStore>;
export function useConnections<T>(selector: (connections: Map<ConnectionId, ConnectionStore>) => T): T;
export function useConnections<T>(selector?: (connections: Map<ConnectionId, ConnectionStore>) => T): T | Map<ConnectionId, ConnectionStore> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useConnections must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useConnections must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const designScope = useDesignScope();
  if (!designScope) throw new Error("useConnections must be called within a DesignScopeProvider");
  const designId = designScope.id;
  if (!kit.designs.has(designId)) throw new Error(`Design store not found for design ${designId}`);
  const designStore = kit.designs.get(designId)!;
  return useStore(designStore.connections, designStore.on.updated.design, selector);
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

export function usePorts(): Map<PortId, Port>;
export function usePorts<T>(selector: (ports: Map<PortId, Port>) => T): T;
export function usePorts<T>(selector?: (ports: Map<PortId, Port>) => T): T | Map<PortId, Port> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("usePorts must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("usePorts must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kit = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("usePorts must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  if (!kit.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kit.types.get(typeId)!;
  return useStore(typeStore.ports, typeStore.on.updated.type, (ports) => {
    const portsMap = new Map<PortId, Port>();
    for (const [id, portStore] of ports) {
      portsMap.set(id, portStore.port);
    }
    return selector ? selector(portsMap) : portsMap;
  });
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

export function useRepresentations(): Map<RepresentationId, Representation>;
export function useRepresentations<T>(selector: (representations: Map<RepresentationId, Representation>) => T): T;
export function useRepresentations<T>(selector?: (representations: Map<RepresentationId, Representation>) => T): T | Map<RepresentationId, Representation> {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useRepresentations must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useRepresentations must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  const typeScope = useTypeScope();
  if (!typeScope) throw new Error("useRepresentations must be called within a TypeScopeProvider");
  const typeId = typeScope.id;
  if (!kitStore.types.has(typeId)) throw new Error(`Type store not found for type ${typeId}`);
  const typeStore = kitStore.types.get(typeId)!;
  return useStore(typeStore.representations, typeStore.on.updated.type, (representations) => {
    const representationsMap = new Map<RepresentationId, Representation>();
    for (const [id, representationStore] of representations) {
      representationsMap.set(id, representationStore.representation);
    }
    return selector ? selector(representationsMap) : representationsMap;
  });
}
// #endregion Hooks
