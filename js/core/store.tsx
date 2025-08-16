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

class SketchpadStore implements SketchpadStore {}

const stores: Map<string, SketchpadStore> = new Map();

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

const SketchpadScopeProvider = (props: { id: string; persisted: boolean; children: React.ReactNode }) => {
  const value = useMemo(() => ({ id: props.id, persisted: props.persisted }), [props.id, props.persisted]);
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

const useSketchpadScope = () => useContext(SketchpadScopeContext);
const useKitScope = () => useContext(KitScopeContext);
const useDesignScope = () => useContext(DesignScopeContext);
const useTypeScope = () => useContext(TypeScopeContext);
const usePieceScope = () => useContext(PieceScopeContext);
const useConnectionScope = () => useContext(ConnectionScopeContext);
const useRepresentationScope = () => useContext(RepresentationScopeContext);
const usePortScope = () => useContext(PortypeScopeContext);

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

export function useSketchpad<T>(selector?: (sketchpad: SketchpadState) => T, id?: string): T {
  const scope = useSketchpadScope();
  const storeId = scope?.id ?? id;
  if (!storeId) throw new Error("useSketchpad must be called within a SketchpadScopeProvider or be directly provided with an id");
  if (!stores.has(storeId)) throw new Error(`Sketchpad store was not found for id ${storeId}`);
  const store = stores.get(storeId)!;
  return useStore(store, store.on.updated.sketchpad, selector);
}

export function useDesignEditor<T>(selector?: (editor: DesignEditorState) => T, id?: DesignId): T {
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

export function useDesignEditors<T>(selector?: (editors: Map<DesignId, DesignEditorState>) => T): T {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesignEditors must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  return useStore(store, store.on.updated.designEditors, selector);
}

export function useKit<T>(selector?: (kit: Kit) => T, id?: KitId): T {
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

export function useKits<T>(selector?: (kits: Map<KitId, Kit>) => T): T {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useKits must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  return useStore(store, store.on.updated.kits, selector);
}

export function useDesign<T>(selector?: (design: Design) => T, id?: DesignId): T {
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
  return useStore(designStore, designStore.on.updated.design, selector);
}

export function useDesigns<T>(selector?: (designs: Map<DesignId, Design>) => T): T {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useDesigns must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useDesigns must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore, kitStore.on.updated.kit, selector);
}

export function useType<T>(selector?: (type: Type) => T, id?: TypeId): T {
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
  return useStore(typeStore, typeStore.on.updated.type, selector);
}

export function useTypes<T>(selector?: (types: Map<TypeId, Type>) => T): T {
  const sketchpadScope = useSketchpadScope();
  if (!sketchpadScope) throw new Error("useTypes must be called within a SketchpadScopeProvider");
  const store = stores.get(sketchpadScope.id)!;
  const kitScope = useKitScope();
  if (!kitScope) throw new Error("useTypes must be called within a KitScopeProvider");
  const kitId = kitScope.id;
  if (!store.kits.has(kitId)) throw new Error(`Kit store not found for kit ${kitId}`);
  const kitStore = store.kits.get(kitId)!;
  return useStore(kitStore, kitStore.on.updated.kit, selector);
}

export function usePiece<T>(selector?: (piece: Piece) => T, id?: PieceId): T {
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
  return useStore(pieceStore, pieceStore.on.updated.piece, selector);
}

export function usePieces<T>(selector?: (pieces: Map<PieceId, Piece>) => T): T {
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
  return useStore(designStore, designStore.on.updated.design, selector);
}

export function useConnection<T>(selector?: (connection: Connection) => T, id?: ConnectionId): T {
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
  return useStore(connectionStore, connectionStore.on.updated.connection, selector);
}

export function useConnections<T>(selector?: (connections: Map<ConnectionId, Connection>) => T): T {
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
  return useStore(designStore, designStore.on.updated.design, selector);
}

export function usePort<T>(selector?: (port: Port) => T, id?: PortId): T {
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
  return useStore(portStore, portStore.on.updated.port, selector);
}

export function usePorts<T>(selector?: (ports: Map<PortId, Port>) => T): T {
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
  return useStore(typeStore, typeStore.on.updated.type, selector);
}

export function useRepresentation<T>(selector?: (representation: Representation) => T, id?: RepresentationId): T {
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
  return useStore(representationStore, representationStore.on.updated.representation, selector);
}

export function useRepresentations<T>(selector?: (representations: Map<RepresentationId, Representation>) => T): T {
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
  return useStore(typeStore, typeStore.on.updated.type, selector);
}
// #endregion Hooks
