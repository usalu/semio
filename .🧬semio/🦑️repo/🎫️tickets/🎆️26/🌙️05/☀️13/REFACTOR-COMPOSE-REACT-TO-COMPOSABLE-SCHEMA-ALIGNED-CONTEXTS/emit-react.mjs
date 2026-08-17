import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const outPath = path.resolve(__dirname, "../../../..", "compose/client/lib/react/index.tsx");

const header = `// #region ⚛️Header
/** @emoji ⚛️ Sealed React surface for compose: composable id-only contexts over compose/js (implementation detail). */
// #endregion ⚛️Header

// #region ⚛️Imports
import type { ReactNode } from "react";
import * as React from "react";
import type {
  Attribute,
  Benchmark,
  Camera,
  ConnectionSide,
  Coordinate,
  Entity,
  FieldSpec,
  GraphRootKind,
  KitReadPoint,
  OffsetInput,
  PieceBlueprint,
  Place,
  Plane,
  Point,
  Position,
  PositionInput,
  SetError,
  SetResult,
  Side,
  Vector,
} from "../js";
import {
  Alternative,
  Author,
  Backbone,
  Change,
  Checkpoint,
  Concept,
  Conflict,
  Connection,
  Connector,
  Design,
  Edit,
  EventBus,
  Family,
  File,
  Folder,
  Graph,
  Group,
  Kit,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  kitReadPointKey,
  Layer,
  LocalProvider,
  defineField,
  defineFields,
  defineOperation,
  defineOperations,
  Operation,
  Piece,
  PiecesOperations,
  Port,
  Prop,
  Quality,
  RemoteProvider,
  Representation,
  Session,
  Stat,
  Store,
  Tag,
  TheKit,
  theKitReadPoint,
  Type,
} from "../js";
// #endregion ⚛️Imports
`;

const types = `
// #region 🧬️Types
export type FieldReadState<T> = Readonly<{
  value: T | undefined;
  loading: boolean;
  error: unknown;
  refresh: () => Promise<void>;
}>;

/** @emoji 🪪️ Entity hooks return this shape with {@code value: { id }} (no compose/js class in {@code value}). */
export type EntityReadState = FieldReadState<Readonly<{ id: string }>>;

/** @emoji 🎛️ UI-facing operation lifecycle (idle → pending → settled). */
export type OperationStatus =
  | { readonly kind: "idle" }
  | { readonly kind: "pending" }
  | { readonly kind: "settled"; readonly result: SetResult };

/** @emoji 📇️ Plain list row for id-stable kit lists (consumers wrap with matching {@code XContextProvider}). */
export type IdRow = Readonly<{ id: string }>;

export type {
  Attribute,
  Benchmark,
  Camera,
  ConnectionSide,
  Coordinate,
  FieldSpec,
  GraphRootKind,
  KitReadPoint,
  OffsetInput,
  PieceBlueprint,
  Place,
  Plane,
  Point,
  Position,
  PositionInput,
  SetError,
  SetResult,
  Side,
  Vector,
};
// #endregion 🧬️Types
`;

const internals = `
// #region 🪝️Internals
function useJsSession(): Session {
  const s = React.useContext(SessionHandleContext);
  if (s == null) throw new Error("compose/react: SessionHandleContext missing; wrap with <SessionContextProvider>.");
  return s;
}

function useJsStore(): Store {
  const s = React.useContext(StoreHandleContext);
  if (s == null) throw new Error("compose/react: StoreHandleContext missing; wrap with <StoreContextProvider id={…}>.");
  return s;
}

/** @emoji 🪝️ Private field bind: async read + optional bus kind (no useSyncExternalStore). */
function useEntityField<E extends Entity, T>(
  getEntity: () => E | null,
  read: (entity: E) => Promise<T>,
  eventKind?: string,
): FieldReadState<T> {
  const entity = getEntity();
  const [value, setValue] = React.useState<T | undefined>(undefined);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<unknown>(undefined);
  const entityRef = React.useRef(entity);
  entityRef.current = entity;

  const refresh = React.useCallback(async () => {
    const e = entityRef.current;
    if (e == null) {
      setValue(undefined);
      setError(undefined);
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(undefined);
    try {
      setValue(await read(e));
    } catch (err) {
      setError(err);
    } finally {
      setLoading(false);
    }
  }, [read]);

  React.useEffect(() => {
    void refresh();
  }, [refresh, entity?.id, entity?.storeId]);

  React.useEffect(() => {
    const e = entityRef.current;
    if (e == null) return;
    const store = e.storeId != null && e.storeId !== "" ? e.session.store(e.storeId) : null;
    if (store == null) return;
    if (eventKind != null && eventKind !== "") return store.bus.subscribeKind(eventKind, () => void refresh());
    return undefined;
  }, [entity, eventKind, refresh]);

  return { value, loading, error, refresh };
}

/** @emoji 🪝️ Private operation bind returning {@link OperationStatus}. */
function useEntityOperation<E extends Entity, Args extends unknown[] = []>(
  getEntity: () => E | null,
  impl: (entity: E, ...args: Args) => Promise<SetResult>,
): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  const getRef = React.useRef(getEntity);
  getRef.current = getEntity;
  const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

  const run = React.useCallback(
    async (...args: Args) => {
      const e = getRef.current();
      if (e == null) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Disposed", message: "No entity in React context.", field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
      setStatus({ kind: "pending" });
      try {
        const result = await impl(e, ...args);
        setStatus({ kind: "settled", result });
        return result;
      } catch (err) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Internal", message: err instanceof Error ? err.message : String(err), field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
    },
    [impl],
  );

  return [run, status] as const;
}

function useEntityOperationStore<Args extends unknown[] = []>(
  getStore: () => Store | null,
  impl: (store: Store, ...args: Args) => Promise<SetResult>,
): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  const getRef = React.useRef(getStore);
  getRef.current = getStore;
  const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });
  const run = React.useCallback(
    async (...args: Args) => {
      const st = getRef.current();
      if (st == null) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Disposed", message: "No store in React context.", field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
      setStatus({ kind: "pending" });
      try {
        const result = await impl(st, ...args);
        setStatus({ kind: "settled", result });
        return result;
      } catch (err) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Internal", message: err instanceof Error ? err.message : String(err), field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
    },
    [impl],
  );
  return [run, status] as const;
}

function useEntityOperationSession<Args extends unknown[] = []>(
  getSession: () => Session | null,
  impl: (session: Session, ...args: Args) => Promise<SetResult>,
): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  const getRef = React.useRef(getSession);
  getRef.current = getSession;
  const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });
  const run = React.useCallback(
    async (...args: Args) => {
      const s = getRef.current();
      if (s == null) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Disposed", message: "No session in React context.", field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
      setStatus({ kind: "pending" });
      try {
        const result = await impl(s, ...args);
        setStatus({ kind: "settled", result });
        return result;
      } catch (err) {
        const result: SetResult = {
          ok: false,
          error: { kind: "Internal", message: err instanceof Error ? err.message : String(err), field: undefined, entity: undefined },
        };
        setStatus({ kind: "settled", result });
        return result;
      }
    },
    [impl],
  );
  return [run, status] as const;
}

function bindPiecesOperationsOperationToReact<Args extends unknown[]>(
  impl: (ops: PiecesOperations, ...args: Args) => Promise<SetResult>,
): (getOps: () => PiecesOperations | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function usePiecesOperationsOp(getOps: () => PiecesOperations | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(getOps);
    getRef.current = getOps;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });
    const run = React.useCallback(
      async (...args: Args) => {
        const ops = getRef.current();
        if (ops == null) {
          const result: SetResult = {
            ok: false,
            error: {
              kind: "Disposed",
              message: "No pieces batch scope (empty ids or missing store/design).",
              field: undefined,
              entity: undefined,
            },
          };
          setStatus({ kind: "settled", result });
          return result;
        }
        setStatus({ kind: "pending" });
        try {
          const result = await impl(ops, ...args);
          setStatus({ kind: "settled", result });
          return result;
        } catch (err) {
          const result: SetResult = {
            ok: false,
            error: { kind: "Internal", message: err instanceof Error ? err.message : String(err), field: undefined, entity: undefined },
          };
          setStatus({ kind: "settled", result });
          return result;
        }
      },
      [impl],
    );
    return [run, status] as const;
  };
}
// #endregion 🪝️Internals
`;

const idCtx = (name, emoji) => `
const ${name}Context = React.createContext<Readonly<{ id: string }> | null>(null);
/** @emoji ${emoji} Publishes {@code { id }} for ${name.replace("Id", "")} scope. */
export function ${name}ContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(${name}Context.Provider, { value: v }, props.children);
}
`;

const markerCtx = (name, emoji) => `
const ${name}MarkerContext = React.createContext<boolean>(false);
/** @emoji ${emoji} Marker: mount exactly one graph-tier provider under {@link StoreContextProvider}. */
export function ${name}ContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  return React.createElement(${name}MarkerContext.Provider, { value: true }, props.children);
}
`;

const pieces = `
// #region 🪪️Ids
const SessionHandleContext = React.createContext<Session | null>(null);
const StoreHandleContext = React.createContext<Store | null>(null);
const GraphHandleContext = React.createContext<Graph | null>(null);
const TheKitHandleContext = React.createContext<TheKit | null>(null);
const AlternativeHandleContext = React.createContext<Alternative | null>(null);
const KitHandleContext = React.createContext<Kit | null>(null);
const LocalProviderHandleContext = React.createContext<LocalProvider | null>(null);
const RemoteProviderHandleContext = React.createContext<RemoteProvider | null>(null);
const BackboneHandleContext = React.createContext<Backbone | null>(null);

const PiecesBatchContext = React.createContext<Readonly<{ pieceIds: readonly string[] }> | null>(null);

/** @emoji 🪢️ Batch piece ids for {@link useDragPieces} / {@link useMovePieces} (design from {@link DesignIdContext}). */
export function PiecesBatchContextProvider(props: Readonly<{ pieceIds: readonly string[]; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ pieceIds: props.pieceIds }), [props.pieceIds]);
  return React.createElement(PiecesBatchContext.Provider, { value: v }, props.children);
}

${markerCtx("Wip", "🌿️")}
${markerCtx("Stage", "🎭️")}
${markerCtx("Authoritative", "🏛️")}
${markerCtx("TheKit", "🏛️")}

${idCtx("StoreId", "🏪️")}
${idCtx("AlternativeId", "🔀️")}
${idCtx("KitId", "📦️")}
${idCtx("DesignId", "📐️")}
${idCtx("TypeId", "🧰️")}
${idCtx("AuthorId", "✍️")}
${idCtx("QualityId", "💎️")}
${idCtx("TagId", "🏷️")}
${idCtx("ConceptId", "💡️")}
${idCtx("PieceId", "🧩️")}
${idCtx("ConnectionId", "⛓️")}
${idCtx("PortId", "🔘️")}
${idCtx("ConnectorId", "🔗️")}
${idCtx("RepresentationId", "🎨️")}
${idCtx("RemoteProviderUrl", "🛜️")}
${idCtx("FileBackboneId", "📁️")}
${idCtx("FolderBackboneId", "📂️")}
${idCtx("WebsocketBackboneId", "🛰️")}

const PositionMarkerContext = React.createContext(false);
const FlatPositionMarkerContext = React.createContext(false);
const PlaneMarkerContext = React.createContext(false);
const OriginMarkerContext = React.createContext(false);
// #endregion 🪪️Ids
`;

const sessionProv = `
// #region 🎭️Providers
/** @emoji 🗂️ Root session handle (opaque to callers of public hooks). */
export function SessionContextProvider(props: Readonly<{ session: unknown; children: ReactNode }>): React.ReactElement {
  const s = props.session as Session;
  return React.createElement(SessionHandleContext.Provider, { value: s }, props.children);
}

export type StoreContextProviderProps = Readonly<{
  id: string;
  initialReadPoint?: KitReadPoint;
  children: ReactNode;
}>;

/** @emoji 🏪️ Resolves {@link Store} from session + id; applies optional {@link Store#setReadPoint}. */
export function StoreContextProvider(props: StoreContextProviderProps): React.ReactElement {
  const session = useJsSession();
  const store = React.useMemo(() => session.store(props.id), [session, props.id]);
  const [readPoint] = React.useState<KitReadPoint>(props.initialReadPoint ?? theKitReadPoint);
  React.useEffect(() => {
    store.setReadPoint(readPoint);
  }, [store, readPoint]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    StoreHandleContext.Provider,
    { value: store },
    React.createElement(StoreIdContext.Provider, { value: idRow }, props.children),
  );
}

export function LocalProviderContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const session = useJsSession();
  const lp = React.useMemo(() => session.localProvider(), [session]);
  return React.createElement(LocalProviderHandleContext.Provider, { value: lp }, props.children);
}

export function RemoteProviderContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = useJsSession();
  const rp = React.useMemo(() => session.remoteProvider(props.id), [session, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    RemoteProviderHandleContext.Provider,
    { value: rp },
    React.createElement(RemoteProviderUrlIdContext.Provider, { value: idRow }, props.children),
  );
}

export function WipContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = useJsStore();
  const graph = React.useMemo(() => store.wip(), [store]);
  return React.createElement(
    WipMarkerContext.Provider,
    { value: true },
    React.createElement(GraphHandleContext.Provider, { value: graph }, props.children),
  );
}

export function StageContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = useJsStore();
  const graph = React.useMemo(() => store.stage(), [store]);
  return React.createElement(
    StageMarkerContext.Provider,
    { value: true },
    React.createElement(GraphHandleContext.Provider, { value: graph }, props.children),
  );
}

export function AuthoritativeContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = useJsStore();
  const graph = React.useMemo(() => store.authoritative(), [store]);
  return React.createElement(
    AuthoritativeMarkerContext.Provider,
    { value: true },
    React.createElement(GraphHandleContext.Provider, { value: graph }, props.children),
  );
}

export function TheKitContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const graph = React.useContext(GraphHandleContext);
  if (graph == null) throw new Error("compose/react: TheKitContextProvider requires a graph tier (Wip|Stage|Authoritative) above.");
  const tk = React.useMemo(() => graph.theKit(), [graph]);
  return React.createElement(
    TheKitMarkerContext.Provider,
    { value: true },
    React.createElement(TheKitHandleContext.Provider, { value: tk }, props.children),
  );
}

export function AlternativeContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const graph = React.useContext(GraphHandleContext);
  if (graph == null) throw new Error("compose/react: AlternativeContextProvider requires a graph tier above.");
  const alt = React.useMemo(() => graph.alternative(props.id), [graph, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    AlternativeHandleContext.Provider,
    { value: alt },
    React.createElement(AlternativeIdContext.Provider, { value: idRow }, props.children),
  );
}

export function KitContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const store = useJsStore();
  const theKit = React.useContext(TheKitHandleContext);
  const alt = React.useContext(AlternativeHandleContext);
  const kit = React.useMemo(() => {
    if (theKit != null) {
      return new Kit(store.session, props.id, store.id);
    }
    if (alt != null) {
      return new Kit(store.session, props.id, store.id);
    }
    throw new Error("compose/react: KitContextProvider requires TheKitContextProvider or AlternativeContextProvider.");
  }, [store.session, store.id, props.id, theKit, alt]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(KitHandleContext.Provider, { value: kit }, React.createElement(KitIdContext.Provider, { value: idRow }, props.children));
}

export function DesignContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(DesignIdContextProvider, props);
}
export function TypeContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(TypeIdContextProvider, props);
}
export function AuthorContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(AuthorIdContextProvider, props);
}
export function QualityContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(QualityIdContextProvider, props);
}
export function TagContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(TagIdContextProvider, props);
}
export function ConceptContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(ConceptIdContextProvider, props);
}
export function PieceContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(PieceIdContextProvider, props);
}
export function ConnectionContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(ConnectionIdContextProvider, props);
}
export function PortContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(PortIdContextProvider, props);
}
export function ConnectorContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(ConnectorIdContextProvider, props);
}
export function RepresentationContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(RepresentationIdContextProvider, props);
}

export function FileBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const lp = React.useContext(LocalProviderHandleContext);
  if (lp == null) throw new Error("compose/react: FileBackboneContextProvider requires LocalProviderContextProvider.");
  const bb = React.useMemo(() => new Backbone(useJsSession(), props.id, lp), [props.id, lp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    BackboneHandleContext.Provider,
    { value: bb },
    React.createElement(FileBackboneIdContext.Provider, { value: idRow }, props.children),
  );
}

export function FolderBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const lp = React.useContext(LocalProviderHandleContext);
  if (lp == null) throw new Error("compose/react: FolderBackboneContextProvider requires LocalProviderContextProvider.");
  const bb = React.useMemo(() => new Backbone(useJsSession(), props.id, lp), [props.id, lp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    BackboneHandleContext.Provider,
    { value: bb },
    React.createElement(FolderBackboneIdContext.Provider, { value: idRow }, props.children),
  );
}

export function WebsocketBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const rp = React.useContext(RemoteProviderHandleContext);
  if (rp == null) throw new Error("compose/react: WebsocketBackboneContextProvider requires RemoteProviderContextProvider.");
  const bb = React.useMemo(() => new Backbone(useJsSession(), props.id, rp), [props.id, rp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(
    BackboneHandleContext.Provider,
    { value: bb },
    React.createElement(WebsocketBackboneIdContext.Provider, { value: idRow }, props.children),
  );
}

export function PositionContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  return React.createElement(PositionMarkerContext.Provider, { value: true }, props.children);
}
export function FlatPositionContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  return React.createElement(FlatPositionMarkerContext.Provider, { value: true }, props.children);
}
export function PlaneContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  return React.createElement(PlaneMarkerContext.Provider, { value: true }, props.children);
}
export function OriginContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  return React.createElement(OriginMarkerContext.Provider, { value: true }, props.children);
}
// #endregion 🎭️Providers
`;

const resolvers = `
// #region 🧭️Resolvers
function readOptionalId(ctx: React.Context<Readonly<{ id: string }> | null>, override?: string): string | null {
  const row = React.useContext(ctx);
  const id = override ?? row?.id ?? null;
  return id == null || id === "" ? null : id;
}

function resolveStore(override?: string): Store | null {
  try {
    return useJsStore();
  } catch {
    return null;
  }
}

function resolveGraph(): Graph | null {
  return React.useContext(GraphHandleContext);
}

function resolveKit(override?: string): Kit | null {
  const k = React.useContext(KitHandleContext);
  const id = readOptionalId(KitIdContext, override);
  if (k == null || id == null) return null;
  return k.id === id ? k : new Kit(k.session, id, k.storeId);
}

function resolveDesign(override?: string): Design | null {
  const store = resolveStore();
  const id = readOptionalId(DesignIdContext, override);
  if (store == null || id == null) return null;
  return store.design(id);
}

function resolveType(override?: string): Type | null {
  const store = resolveStore();
  const id = readOptionalId(TypeIdContext, override);
  if (store == null || id == null) return null;
  return store.type(id);
}

function resolvePiece(override?: string): Piece | null {
  const design = resolveDesign();
  const id = readOptionalId(PieceIdContext, override);
  if (design == null || id == null) return null;
  return design.piece(id);
}

function resolveConnection(override?: string): Connection | null {
  const design = resolveDesign();
  const id = readOptionalId(ConnectionIdContext, override);
  if (design == null || id == null) return null;
  return design.connection(id);
}

function resolvePort(override?: string): Port | null {
  const t = resolveType();
  const id = readOptionalId(PortIdContext, override);
  if (t == null || id == null) return null;
  return t.port(id);
}

function resolveConnector(override?: string): Connector | null {
  const t = resolveType();
  const id = readOptionalId(ConnectorIdContext, override);
  if (t == null || id == null) return null;
  return t.connector(id);
}

function resolveRepresentation(override?: string): Representation | null {
  const t = resolveType();
  const id = readOptionalId(RepresentationIdContext, override);
  if (t == null || id == null) return null;
  return t.representation(id);
}

function resolveQuality(override?: string): Quality | null {
  const store = resolveStore();
  const id = readOptionalId(QualityIdContext, override);
  if (store == null || id == null) return null;
  return store.quality(id);
}

function resolveTag(override?: string): Tag | null {
  const store = resolveStore();
  const id = readOptionalId(TagIdContext, override);
  if (store == null || id == null) return null;
  return store.tag(id);
}

function resolveConcept(override?: string): Concept | null {
  const store = resolveStore();
  const id = readOptionalId(ConceptIdContext, override);
  if (store == null || id == null) return null;
  return store.concept(id);
}

function resolveAuthor(override?: string): Author | null {
  const store = resolveStore();
  const id = readOptionalId(AuthorIdContext, override);
  if (store == null || id == null) return null;
  return store.author(id);
}

function resolveAlternative(override?: string): Alternative | null {
  const alt = React.useContext(AlternativeHandleContext);
  const id = readOptionalId(AlternativeIdContext, override);
  if (alt != null && (override == null || alt.id === override)) return alt;
  const g = resolveGraph();
  if (g == null || id == null) return null;
  return g.alternative(id);
}

function resolveLocalProvider(): LocalProvider | null {
  return React.useContext(LocalProviderHandleContext);
}

function resolveRemoteProvider(override?: string): RemoteProvider | null {
  const row = React.useContext(RemoteProviderUrlIdContext);
  const rp = React.useContext(RemoteProviderHandleContext);
  const id = override ?? row?.id ?? null;
  if (rp != null && (override == null || rp.url === override)) return rp;
  if (id == null) return null;
  try {
    return useJsSession().remoteProvider(id);
  } catch {
    return null;
  }
}

function resolveBackbone(override?: string): Backbone | null {
  const bb = React.useContext(BackboneHandleContext);
  const id =
    override ??
    React.useContext(FileBackboneIdContext)?.id ??
    React.useContext(FolderBackboneIdContext)?.id ??
    React.useContext(WebsocketBackboneIdContext)?.id ??
    null;
  if (bb != null && (override == null || bb.id === override)) return bb;
  return null;
}
// #endregion 🧭️Resolvers
`;

const entityHooks = `
// #region 🪝️EntityHooks
function entityRead<E extends Entity>(resolve: () => E | null, busKind?: string): EntityReadState {
  return useEntityField(
    resolve,
    async (e) => ({ id: e.id }),
    busKind ?? KIT_EVENT_STREAM_SUBSCRIPTION,
  ) as EntityReadState;
}

/** @emoji 🗂️ Marker: session is active (opaque handle; {@code value.id} is synthetic). */
export function useSession(): EntityReadState {
  const session = React.useContext(SessionHandleContext);
  return useEntityField(
    () => (session != null ? ({ id: "__session__", session } as unknown as Entity) : null),
    async (e) => ({ id: (e as { id: string }).id }),
    undefined,
  ) as EntityReadState;
}

export function useStore(id?: string): EntityReadState {
  const store = resolveStore();
  const row = React.useContext(StoreIdContext);
  const resolved = id ?? row?.id ?? null;
  return useEntityField(
    () => (store != null && resolved != null && store.id === resolved ? store : null),
    async (s) => ({ id: s.id }),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  ) as EntityReadState;
}

export function useWip(): EntityReadState {
  const on = React.useContext(WipMarkerContext);
  return useEntityField(
    () => (on ? ({ id: "wip" } as unknown as Entity) : null),
    async (e) => ({ id: (e as { id: string }).id }),
    undefined,
  ) as EntityReadState;
}

export function useStage(): EntityReadState {
  const on = React.useContext(StageMarkerContext);
  return useEntityField(
    () => (on ? ({ id: "stage" } as unknown as Entity) : null),
    async (e) => ({ id: (e as { id: string }).id }),
    undefined,
  ) as EntityReadState;
}

export function useAuthoritative(): EntityReadState {
  const on = React.useContext(AuthoritativeMarkerContext);
  return useEntityField(
    () => (on ? ({ id: "authoritative" } as unknown as Entity) : null),
    async (e) => ({ id: (e as { id: string }).id }),
    undefined,
  ) as EntityReadState;
}

export function useTheKit(): EntityReadState {
  const on = React.useContext(TheKitMarkerContext);
  const tk = React.useContext(TheKitHandleContext);
  return useEntityField(
    () => (on && tk != null ? (tk as unknown as Entity) : null),
    async (e) => ({ id: e.id }),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  ) as EntityReadState;
}

export function useAlternative(id?: string): EntityReadState {
  return entityRead(() => resolveAlternative(id));
}

export function useKit(id?: string): EntityReadState {
  return entityRead(() => resolveKit(id));
}

export function useDesign(id?: string): EntityReadState {
  return entityRead(() => resolveDesign(id));
}

export function useType(id?: string): EntityReadState {
  return entityRead(() => resolveType(id));
}

export function useAuthor(id?: string): EntityReadState {
  return entityRead(() => resolveAuthor(id));
}

export function useQuality(id?: string): EntityReadState {
  return entityRead(() => resolveQuality(id));
}

export function useTag(id?: string): EntityReadState {
  return entityRead(() => resolveTag(id));
}

export function useConcept(id?: string): EntityReadState {
  return entityRead(() => resolveConcept(id));
}

export function usePiece(id?: string): EntityReadState {
  return entityRead(() => resolvePiece(id));
}

export function useConnection(id?: string): EntityReadState {
  return entityRead(() => resolveConnection(id));
}

export function usePort(id?: string): EntityReadState {
  return entityRead(() => resolvePort(id));
}

export function useConnector(id?: string): EntityReadState {
  return entityRead(() => resolveConnector(id));
}

export function useRepresentation(id?: string): EntityReadState {
  return entityRead(() => resolveRepresentation(id));
}

export function useLocalProvider(): EntityReadState {
  return entityRead(() => resolveLocalProvider() as unknown as Entity | null);
}

export function useRemoteProvider(id?: string): EntityReadState {
  return entityRead(() => resolveRemoteProvider(id) as unknown as Entity | null);
}

export function useFileBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id));
}

export function useFolderBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id));
}

export function useWebsocketBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id));
}
// #endregion 🪝️EntityHooks
`;

const weakHooks = `
// #region 🪶️WeakGeometryHooks
export function usePosition(): FieldReadState<Position> {
  const piece = resolvePiece();
  const pos = React.useContext(PositionMarkerContext);
  return useEntityField(
    () => (piece != null && pos ? piece.position() : null),
    async (p) => p,
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useFlatPosition(): FieldReadState<Position> {
  const piece = resolvePiece();
  const pos = React.useContext(FlatPositionMarkerContext);
  return useEntityField(
    () => (piece != null && pos ? piece.flatPosition() : null),
    async (p) => p,
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function usePlane(): FieldReadState<Plane | null> {
  const piece = resolvePiece();
  const flat = React.useContext(FlatPositionMarkerContext);
  const reg = React.useContext(PositionMarkerContext);
  const planeMarker = React.useContext(PlaneMarkerContext);
  return useEntityField(
    () => {
      if (piece == null || !planeMarker) return null;
      const base = flat ? piece.flatPosition() : reg ? piece.position() : null;
      return base as unknown as Entity | null;
    },
    async (p) => (p as Position).plane(),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useOrigin(): FieldReadState<Point | null> {
  const piece = resolvePiece();
  const originMarker = React.useContext(OriginMarkerContext);
  const flat = React.useContext(FlatPositionMarkerContext);
  const reg = React.useContext(PositionMarkerContext);
  return useEntityField(
    () => {
      if (piece == null || !originMarker) return null;
      const base = flat ? piece.flatPosition() : reg ? piece.position() : null;
      return base as unknown as Entity | null;
    },
    async (p) => (p as Position).plane().origin(),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}
// #endregion 🪶️WeakGeometryHooks
`;

function fieldHook(name, resolverCall, readExpr, eventKind) {
  const ek = eventKind ? `, "${eventKind}"` : "";
  return `
/** @emoji 📖️ ${name} */
export function ${name}(id?: string): FieldReadState<unknown> {
  return useEntityField(() => ${resolverCall}(id), (e) => ${readExpr}${ek});
}
`;
}

const listHooks = `
// #region 🪝️ListHooks
function mapIds<T extends { id: string }>(xs: readonly T[]): readonly IdRow[] {
  return Object.freeze(xs.map((x) => ({ id: x.id })));
}

export function useKitDesigns(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.designs()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useKitTypes(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.types()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useKitAuthors(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.authors()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useKitQualities(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.qualities()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useKitTags(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.tags()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useKitConcepts(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveKit(id),
    async (k) => mapIds(await k.concepts()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useDesignPieces(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveDesign(id),
    async (d) => mapIds(await d.pieces()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useDesignConnections(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveDesign(id),
    async (d) => mapIds(await d.connections()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useTypePorts(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveType(id),
    async (t) => mapIds(await t.ports()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useTypeConnectors(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveType(id),
    async (t) => mapIds(await t.connectors()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function useTypeRepresentations(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolveType(id),
    async (t) => mapIds(await t.representations()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function usePieceChildPieces(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolvePiece(id),
    async (p) => mapIds(await p.childPieces()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}

export function usePieceChildConnections(id?: string): FieldReadState<readonly IdRow[]> {
  return useEntityField(
    () => resolvePiece(id),
    async (p) => mapIds(await p.childConnections()),
    KIT_EVENT_STREAM_SUBSCRIPTION,
  );
}
// #endregion 🪝️ListHooks
`;

const vitest = `
// #region 🧪️Vitest
if (import.meta.vitest) {
  const { readFileSync } = await import("node:fs");
  const path = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const { describe, expect, it } = import.meta.vitest;
  const reactSrcPath = (() => {
    try {
      return fileURLToPath(new URL("./index.tsx", import.meta.url));
    } catch {
      return path.join(process.cwd(), "compose", "react", "index.tsx");
    }
  })();
  const reactSrc = readFileSync(reactSrcPath, "utf8");
  const vitestRegion = reactSrc.indexOf("// #region 🧪️Vitest");
  const reactSrcForBannedScan = vitestRegion === -1 ? reactSrc : reactSrc.slice(0, vitestRegion);
  const exportNames = [...reactSrc.matchAll(/^export (?:function|const) (\\w+)/gm)].map((m) => m[1]);
  const bannedExports = new Set([
    "Kit",
    "Store",
    "Graph",
    "TheKit",
    "Alternative",
    "Session",
    "Design",
    "Type",
    "Piece",
    "Connection",
    "Port",
    "Connector",
    "Representation",
    "Quality",
    "Tag",
    "Concept",
    "Author",
    "Backbone",
    "LocalProvider",
    "RemoteProvider",
    "Family",
    "File",
    "Folder",
    "Layer",
    "Group",
    "Stat",
    "Prop",
    "Edit",
    "Checkpoint",
    "Change",
    "Conflict",
    "PiecesOperations",
    "EventBus",
    "createKitStoreWorker",
    "openStore",
    "theKitReadPoint",
    "kitReadPointKey",
    "defineField",
    "defineFields",
    "defineOperation",
    "defineOperations",
    "KIT_EVENT_STREAM_SUBSCRIPTION",
    "Operation",
  ]);

  describe("compose/react sealed surface", () => {
    it("exports no compose/js entity-class symbols", () => {
      for (const n of exportNames) {
        expect.soft(bannedExports.has(n), \`illegal export: \${n}\`).toBe(false);
      }
    });
    it("banned implementation strings are absent before vitest region", () => {
      const mustNotMatchCode = [
        /\\bbindFieldToReact\\b/,
        /\\bbindOperationToReact\\b/,
        /\\bbindStoreOperationToReact\\b/,
        /\\bbindPiecesOperationsOperationToReact\\b/,
        /\\buseSyncExternalStore\\s*\\(/,
        /\\bapplyKitDiff\\s*\\(/,
        /\\buseDesignAppCommands\\s*\\(/,
        /\\bKitStoreSnapshot\\b/,
        /\\bapplyToCache\\s*\\(/,
        /\\bdispatchSync\\s*\\(/,
        /\\bfieldSync\\b/,
        /\\breconcil/i,
        /\\buseKitScope\\s*\\(/,
        /\\bKitScope\\b/,
        /\\bKitShellScopeProvider\\b/,
      ];
      for (const re of mustNotMatchCode) {
        expect.soft(reactSrcForBannedScan.match(re), String(re)).toBeNull();
      }
    });
  });

  describe("useDesign id binding", () => {
    it("resolves design id from context provider", async () => {
      const { createElement: h } = React;
      const { render, screen } = await import("@testing-library/react");
      const sid = "store-test";
      const did = "design-test";
      function Read() {
        const st = useDesign();
        return h("span", { "data-testid": "id" }, st.value?.id ?? "");
      }
      const session = { store: () => ({ id: sid, session: { bus: { subscribeKind: () => () => {} } }, design: () => ({ id: did }) } ) } as unknown as Session;
      const tree = h(
        SessionContextProvider,
        { session },
        h(StoreContextProvider, { id: sid }, h(DesignContextProvider, { id: did }, h(Read, null))),
      );
      render(tree);
      expect(screen.getByTestId("id").textContent).toBe(did);
    });
  });
}
// #endregion 🧪️Vitest
`;

// NOTE: emit script is partial — field/operation hooks and useSession fix need completing in a follow-up patch.
// For now write a marker so we know emit ran.
const body = [
  header,
  types,
  internals,
  pieces,
  sessionProv,
  resolvers,
  entityHooks,
  weakHooks,
  listHooks,
  "// #region 🪝️FieldHooks\n// (generated below via patch)\n// #endregion 🪝️FieldHooks\n",
  "// #region 🪝️OperationHooks\n// #endregion 🪝️OperationHooks\n",
  vitest,
].join("\n");

fs.writeFileSync(outPath, body, "utf8");
console.log("wrote", outPath, "bytes", body.length);
