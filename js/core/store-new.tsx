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
  selectedPieceIds: PieceId[];
  selectedConnections: ConnectionId[];
  selectedPiecePortId?: { pieceId: PieceId; portId: PortId };
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
      kits: (subscribe: Subscribe) => Unsubscribe;
      designEditor: (subscribe: Subscribe) => Unsubscribe;
      designEditors: (subscribe: Subscribe) => Unsubscribe;
    };
    updated: {
      kit: (subscribe: Subscribe, deep?: boolean) => Unsubscribe;
    };
    deleted: {
      kit: (subscribe: Subscribe) => Unsubscribe;
      kits: (subscribe: Subscribe) => Unsubscribe;
      designEditor: (subscribe: Subscribe) => Unsubscribe;
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

class SketchpadStore {}

// #region Hooks

// #region Scoping

type SketchpadScope = { id: string };
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

export const DesignEditorScopeProvider = (props: { id?: string; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id || "" }), [props.id]);
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

export function useSketchpad<T>(selector?: (sketchpad: SketchpadStore, id?: string) => T): T {
  throw new Error("Not implemented");
}

export function useDesignEditor<T>(selector?: (editor: DesignEditorStore, designId: DesignId) => T, id?: string): T {
  throw new Error("Not implemented");
}

export function useDesignEditors<T>(selector?: (editors: Map<DesignId, DesignEditorStore>) => T): T {
  throw new Error("Not implemented");
}

export function useKit<T>(selector?: (kit: KitStore) => T, id?: KitId): T {
  throw new Error("Not implemented");
}

export function useKits<T>(selector?: (kits: Map<KitId, KitStore>) => T): T {
  throw new Error("Not implemented");
}

export function useDesign<T>(selector?: (design: DesignStore) => T, kitId?: KitId, designId?: DesignId): T {
  throw new Error("Not implemented");
}

export function useDesigns<T>(selector?: (designs: Map<DesignId, DesignStore>) => T, kitId?: KitId): T {
  throw new Error("Not implemented");
}

export function useType<T>(selector?: (type: TypeStore) => T, kitId?: KitId, typeId?: TypeId): T {
  throw new Error("Not implemented");
}

export function useTypes<T>(selector?: (types: Map<TypeId, TypeStore>) => T, kitId?: KitId): T {
  throw new Error("Not implemented");
}

export function usePiece<T>(selector?: (piece: PieceStore) => T, kitId?: KitId, designId?: DesignId, pieceId?: PieceId): T {
  throw new Error("Not implemented");
}

export function usePieces<T>(selector?: (pieces: Map<PieceId, PieceStore>) => T, kitId?: KitId, designId?: DesignId): T {
  throw new Error("Not implemented");
}

export function useConnection<T>(selector?: (connection: ConnectionStore) => T, kitId?: KitId, designId?: DesignId, connectionId?: ConnectionId): T {
  throw new Error("Not implemented");
}

export function useConnections<T>(selector?: (connections: Map<ConnectionId, ConnectionStore>) => T, kitId?: KitId, designId?: DesignId): T {
  throw new Error("Not implemented");
}

export function usePort<T>(selector?: (port: PortStore) => T, kitId?: KitId, typeId?: TypeId, portId?: PortId): T {
  throw new Error("Not implemented");
}

export function usePorts<T>(selector?: (ports: Map<PortId, PortStore>) => T, kitId?: KitId, typeId?: TypeId): T {
  throw new Error("Not implemented");
}

export function useRepresentation<T>(selector?: (representation: RepresentationStore) => T, kitId?: KitId, typeId?: TypeId, representationId?: RepresentationId): T {
  throw new Error("Not implemented");
}

export function useRepresentations<T>(selector?: (representations: Map<RepresentationId, RepresentationStore>) => T, kitId?: KitId, typeId?: TypeId): T {
  throw new Error("Not implemented");
}

export function useQuality<T>(selector?: (quality: QualityStore) => T, parentId?: string, qualityId?: QualityId): T {
  throw new Error("Not implemented");
}

export function useQualities<T>(selector?: (qualities: Map<QualityId, QualityStore>) => T, parentId?: string): T {
  throw new Error("Not implemented");
}

export function useAuthor<T>(selector?: (author: AuthorStore) => T, parentId?: string, authorId?: string): T {
  throw new Error("Not implemented");
}

export function useAuthors<T>(selector?: (authors: Map<string, AuthorStore>) => T, parentId?: string): T {
  throw new Error("Not implemented");
}

// #endregion Hooks
