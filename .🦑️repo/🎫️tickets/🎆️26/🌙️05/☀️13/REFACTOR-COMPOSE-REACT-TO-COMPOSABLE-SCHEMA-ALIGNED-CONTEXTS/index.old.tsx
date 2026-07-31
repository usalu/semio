// #region ⚛️Header
// Standalone React hooks for compose: thin adapter over stateless {@link Kit} + {@link } reads/writes.
// #endregion ⚛️Header

// #region 🧷️JsReexports
// Value/type re-exports follow the local `@semio-tech/compose-js` imports below (single binding per symbol).
// #endregion 🧷️JsReexports

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
  Location,
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
} from "../client/lib/js";
import {
  Alternative,
  Author,
  Change,
  Checkpoint,
  Concept,
  Conflict,
  Connection,
  Connector,
  createKitStoreWorker,
  defineField,
  defineFields,
  defineOperation,
  defineOperations,
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
  openStore,
  Operation,
  Piece,
  PiecesOperations,
  Port,
  Prop,
  Quality,
  Representation,
  Session,
  Stat,
  Store,
  Tag,
  TheKit,
  theKitReadPoint,
  Type,
} from "../client/lib/js";
// #endregion ⚛️Imports

// #region 🧷️JsPublicExports
export {
  Alternative,
  Author,
  Change,
  Checkpoint,
  Concept,
  Conflict,
  Connection,
  Connector,
  createKitStoreWorker,
  defineField,
  defineFields,
  defineOperation,
  defineOperations,
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
  openStore,
  Operation,
  Piece,
  PiecesOperations,
  Port,
  Prop,
  Quality,
  Representation,
  Session,
  Stat,
  Store,
  Tag,
  TheKit,
  theKitReadPoint,
  Type,
};
export type { Attribute, Benchmark, Camera, Coordinate, GraphRootKind, Location, Place, Plane, Point, Side, Vector };
// #endregion 🧷️JsPublicExports

// #region 🪝️FieldBind
/** @emoji 📖️ One materialized field read: value, loading, error, and manual refresh (no sync external store). */
export type FieldReadState<T> = Readonly<{
  value: T | undefined;
  loading: boolean;
  error: unknown;
  refresh: () => Promise<void>;
}>;

export type FieldBindOptions<E, T> = Readonly<{
  /** @emoji 🧲️ Single async read (one GraphQL selection / entity method). */
  read: (entity: E) => Promise<T>;
  /** @emoji 📡️ When set, {@link Store#bus} {@code subscribeKind}; when omitted, only mount + {@link FieldReadState#refresh} pull fresh data. */
  eventKind?: string;
  /** @emoji 🪝️  source; re-invoked each render — keep stable via {@link React#useCallback}. */
  get: () => E | null;
}>;

/**
 * @emoji 🪝️ Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam T — Parsed field value.
 */
export function bindFieldToReact<E extends Entity, T>(opts: FieldBindOptions<E, T>): () => FieldReadState<T> {
  const { read, eventKind, get } = opts;
  return function use(): FieldReadState<T> {
    const entity = get();
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
    }, [refresh, entity?.id, entity?.store]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      const store = e.store;
      if (eventKind != null && eventKind !== "") return store.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [entity, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}

export type DefinedFieldBindOptions<E extends Entity, T> = Readonly<{
  spec: FieldSpec<T>;
  pathInKit: (self: E) => string;
  get: () => E | null;
  eventKind?: string;
}>;

/**
 * @emoji 🪝️ Same as {@link bindFieldToReact} but connects {@link defineField} so callers share {@link FieldSpec} with tooling/docs.
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam T — Parsed field value.
 */
export function bindDefinedFieldToReact<E extends Entity, T>(opts: DefinedFieldBindOptions<E, T>): () => FieldReadState<T> {
  const { spec, pathInKit, get, eventKind } = opts;
  return function useDefined(): FieldReadState<T> {
    const entity = get();
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
      const r = defineField(e, spec, pathInKit);
      setLoading(true);
      setError(undefined);
      try {
        setValue(await r());
      } catch (err) {
        setError(err);
      } finally {
        setLoading(false);
      }
    }, [spec, pathInKit]);

    React.useEffect(() => {
      void refresh();
    }, [refresh, entity?.id, entity?.store]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      const store = e.store;
      if (eventKind != null && eventKind !== "") return store.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [entity, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝️FieldBind

// #region 🪝️OperationBind
/** @emoji 🎛️ UI-facing operation lifecycle for {@link bindOperationToReact} (idle → pending → settled). */
export type OperationStatus = { readonly kind: "idle" } | { readonly kind: "pending" } | { readonly kind: "settled"; readonly result: SetResult };

/**
 * @emoji 🗺️ Maps {@link SetErrorKind#NameTooLong} to a fixed max-length message; otherwise returns {@link SetError#message}.
 * @param maxChars — Upper bound communicated to the user (schema limit or UI policy).
 */
export function mapTooLong(err: SetError, maxChars: number): string {
  if (err.kind === "NameTooLong") return `Name must be at most ${maxChars} characters.`;
  return err.message;
}

/**
 * @emoji 🪝️ Binds an entity operation to `[run, status]`; `run` reads latest entity via {@code get} ref (no sync external store).
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam Args — Operation arguments after the entity receiver.
 */
export function bindOperationToReact<E extends Entity, Args extends unknown[] = []>(impl: (entity: E, ...args: Args) => Promise<SetResult>): (get: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useOperation(get: () => E | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(get);
    getRef.current = get;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

    const run = React.useCallback(
      async (...args: Args) => {
        const e = getRef.current();
        if (e == null) {
          const result: SetResult = { ok: false, error: { kind: "Disposed", message: "No entity in React context.", field: undefined, entity: undefined } };
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
  };
}
// #endregion 🪝️OperationBind

// #region 🪝️KitFieldBind
/** @emoji 🪝️ Kit-scoped field bind (uses {@link Store#bus} like {@link bindFieldToReact}). */
export type KitFieldBindOptions<T> = Readonly<{
  read: (kit: Kit) => Promise<T>;
  eventKind?: string;
  getKit: () => Kit | null;
}>;

export function bindKitFieldToReact<T>(opts: KitFieldBindOptions<T>): () => FieldReadState<T> {
  const { read, eventKind, getKit } = opts;
  return function useKitBound(): FieldReadState<T> {
    const kit = getKit();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const [loading, setLoading] = React.useState(false);
    const [error, setError] = React.useState<unknown>(undefined);
    const kitRef = React.useRef(kit);
    kitRef.current = kit;

    const refresh = React.useCallback(async () => {
      const k = kitRef.current;
      if (k == null) {
        setValue(undefined);
        setError(undefined);
        setLoading(false);
        return;
      }
      setLoading(true);
      setError(undefined);
      try {
        setValue(await read(k));
      } catch (err) {
        setError(err);
      } finally {
        setLoading(false);
      }
    }, [read]);

    React.useEffect(() => {
      void refresh();
    }, [refresh, kit]);

    React.useEffect(() => {
      const k = kitRef.current;
      if (k == null) return;
      if (eventKind != null && eventKind !== "") return k.store.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [kit, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}

export type StoreFieldBindOptions<T> = Readonly<{
  read: (store: Store) => Promise<T>;
  eventKind?: string;
  getStore: () => Store | null;
}>;

/** @emoji 🪝️ Store-root field bind for session/backbone/root fields. */
export function bindStoreFieldToReact<T>(opts: StoreFieldBindOptions<T>): () => FieldReadState<T> {
  const { read, eventKind, getStore } = opts;
  return function useStoreBound(): FieldReadState<T> {
    const store = getStore();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const [loading, setLoading] = React.useState(false);
    const [error, setError] = React.useState<unknown>(undefined);
    const storeRef = React.useRef(store);
    storeRef.current = store;

    const refresh = React.useCallback(async () => {
      const s = storeRef.current;
      if (s == null) {
        setValue(undefined);
        setError(undefined);
        setLoading(false);
        return;
      }
      setLoading(true);
      setError(undefined);
      try {
        setValue(await read(s));
      } catch (err) {
        setError(err);
      } finally {
        setLoading(false);
      }
    }, [read]);

    React.useEffect(() => {
      void refresh();
    }, [refresh, store]);

    React.useEffect(() => {
      const s = storeRef.current;
      if (s == null) return;
      if (eventKind != null && eventKind !== "") return s.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [store, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝️KitFieldBind

// #region 🪝️StoreOperationBind
/** @emoji 🪝️ Binds a {@link Store} operation to `[run, status]`. */
export function bindStoreOperationToReact<Args extends unknown[] = []>(impl: (store: Store, ...args: Args) => Promise<SetResult>): (getStore: () => Store | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useStoreOp(getStore: () => Store | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(getStore);
    getRef.current = getStore;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

    const run = React.useCallback(
      async (...args: Args) => {
        const k = getRef.current();
        if (k == null) {
          const result: SetResult = { ok: false, error: { kind: "Disposed", message: "No store in React context.", field: undefined, entity: undefined } };
          setStatus({ kind: "settled", result });
          return result;
        }
        setStatus({ kind: "pending" });
        try {
          const result = await impl(k, ...args);
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

function useCurrentEntityField<E extends Entity, T>(entity: E | null, read: (entity: E) => Promise<T>, eventKind?: string): FieldReadState<T> {
  return bindFieldToReact<E, T>({ get: () => entity, read, eventKind })();
}

function useCurrentEntityOperation<E extends Entity, Args extends unknown[] = []>(
  entity: E | null,
  useBound: (get: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus],
): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return useBound(() => entity);
}
// #endregion 🪝️StoreOperationBind

// #region 🫳️ShellHost
/** @emoji 🪟️ Sketchpad kit-store factory signature (host wiring; store shape is host-owned). */
export type SketchpadKitStoreFactory = (store: Store) => Promise<unknown>;

/** @emoji 🪟️ Which persistence-backed kit open paths are available in the host shell. */
export type SketchpadKitKindAvailability = Readonly<Record<"temporary" | "file" | "folder" | "remote", boolean>>;

/** @emoji 🧭️ Tab-shell kit id for routing (not the GraphQL {@link Kit#readId} async field). */
export type ActiveKitTabValue = Readonly<{ id: string }>;

export const ActiveKitTabContext = React.createContext<ActiveKitTabValue | null>(null);

/** @emoji 🧭️ Binds the active tab kit id for sketchpad routing and machine events. */
export function ActiveKitTabContextProvider(props: { kitTabId: string; children: React }): React.ReactElement {
  const v = React.useMemo<ActiveKitTabValue>(() => ({ id: props.kitTabId }), [props.kitTabId]);
  return React.createElement(ActiveKitTabContext.Provider, { value: v }, props.children);
}

/** @emoji 🧭️ Reads {@link ActiveKitTabContextProvider} as the active tab id value. */
export function useActiveKitTab(): ActiveKitTabValue | null {
  return React.useContext(ActiveKitTabContext);
}

/** @emoji 🧭️ True when {@link ActiveKitTabContextProvider} is mounted above. */
export function useIsInActiveKitTab(): boolean {
  return React.useContext(ActiveKitTabContext) != null;
}

/** @emoji 🔌️ Optional WASM host bindings (store + client) parallel to {@link StoreContextProvider}. */
export type KitWasmHostState = Readonly<{ kitTabId: string; store: unknown; kitClient: unknown | null }>;

const KitWasmHostContext = React.createContext<KitWasmHostState | null>(null);

/** @emoji 🔌️ Reads {@link KitWasmMountProvider} host bindings (never a synthetic runtime umbrella). */
export function useKitWasmHost(): KitWasmHostState | null {
  return React.useContext(KitWasmHostContext);
}

export type KitWasmMountProviderProps = Readonly<{
  kitId?: string;
  hostStore: unknown;
  kitClient?: unknown;
  store?: Store | null;
  children: ReactNode;
}>;

/** @emoji 🔌️ Publishes host store/client and optionally wraps {@link StoreContextProvider} when {@code kit} is known. */
export function KitWasmMountProvider(props: KitWasmMountProviderProps): React.ReactElement {
  const host = React.useMemo<KitWasmHostState>(() => ({ kitTabId: props.kitId ?? "", store: props.hostStore, kitClient: props.kitClient ?? null }), [props.kitId, props.hostStore, props.kitClient]);
  const inner = props.store != null ? React.createElement(StoreContextProvider, { store: props.store, children: props.children }) : props.children;
  return React.createElement(KitWasmHostContext.Provider, { value: host }, inner);
}

const KitAlternativeSelectionContext = React.createContext<Readonly<{ kitId: string }> | null>(null);

/** @emoji 🌿️ Local alternative selection scope for sketchpad footer (host VCS wiring may replace reads later). */
export function KitAlternativeSelectionProvider(props: { kitId: string; children: React }): React.ReactElement {
  const v = React.useMemo(() => ({ kitId: props.kitId }), [props.kitId]);
  return React.createElement(KitAlternativeSelectionContext.Provider, { value: v }, props.children);
}

/** @emoji 🌿️ `[selectedId, setSelectedId]` for the current {@link KitAlternativeSelectionProvider}. */
export function useKitAlternativeSelection(): readonly [string | null, (next: string | null) => void] {
  const ctx = React.useContext(KitAlternativeSelectionContext);
  const kitId = ctx?.kitId ?? null;
  const [selected, setSelected] = React.useState<string | null>(null);
  React.useEffect(() => {
    setSelected(null);
  }, [kitId]);
  return [selected, setSelected] as const;
}

/** @emoji 🌿️ Stub list until VCS alternatives are bound to {@link Kit} GraphQL reads in the host. */
export function useKitAlternatives(): readonly unknown[] {
  return React.useMemo(() => [], []);
}
// #endregion 🫳️ShellHost

// #region 🎭️Contexts
// #region 🎒️Kit
const StoreContext = React.createContext<Store | null>(null);

export type StoreContextProviderProps = Readonly<{
  store: Store;
  initialReadPoint?: KitReadPoint;
  children: React;
}>;

/** @emoji 🧭️ Provides {@link Kit}; keeps {@link KitReadPoint} in React state and applies it with {@link Store#setReadPoint}. */
export function StoreContextProvider(props: StoreContextProviderProps): React.ReactElement {
  const [readPoint, setReadPointState] = React.useState<KitReadPoint>(props.initialReadPoint ?? theKitReadPoint);
  React.useEffect(() => {
    props.store.setReadPoint(readPoint);
  }, [props.store, readPoint]);
  return React.createElement(StoreContext.Provider, { value: props.store }, props.children);
}

/** @emoji 🧭️ Requires {@link StoreContextProvider}; returns the GraphQL {@link Store} root. */
export function useStore(): Store {
  const store = React.useContext(StoreContext);
  if (store == null) throw new Error("compose/react: useStore requires <StoreContextProvider>.");
  return store;
}

/** @emoji 🧭️ Optional {@link Store} when {@link StoreContextProvider} is absent (host-only subtrees). */
export function useStoreOptional(): Store | null {
  return React.useContext(StoreContext);
}

/** @emoji 🌐️ WIP {@link Graph} from {@link Store#wip} (no extra provider). */
export function useWipGraph(): Graph {
  const store = useStore();
  return React.useMemo(() => store.wip(), [store]);
}

/** @emoji 🏛️ Target-schema {@link TheKit} under {@code Store.wip()}. */
export function useWipVersion(): TheKit {
  const graph = useWipGraph();
  return React.useMemo(() => graph.theKit(), [graph]);
}

/** @emoji 📦️ Target-schema {@link Kit} under {@code Store.wip().theKit()}. */
export function useWipKit(): Kit {
  const store = useStore();
  return React.useMemo(() => new Kit(store, "kit"), [store]);
}

/** @emoji 🌐️ Authoritative {@link Graph} from {@link Store#authoritative}. */
export function useAuthoritativeGraph(): Graph {
  const store = useStore();
  return React.useMemo(() => store.authoritative(), [store]);
}

/** @emoji 🗂️ Root {@link Session} from {@link Store#session}. */
export function useSession(): Session {
  const store = useStore();
  return React.useMemo(() => store.session(), [store]);
}

// #endregion 🎒️Kit

// #region 🌐️GraphContext
export type GraphContextValue = Readonly<{ root: GraphRootKind }>;

const GraphRootContext = React.createContext<GraphContextValue | null>(null);

/** @emoji 🌐️ Binds {@link GraphRootKind} for {@link useGraph}. */
export function GraphContextProvider(props: { root: GraphRootKind; children: React }): React.ReactElement {
  const v = React.useMemo<GraphContextValue>(() => ({ root: props.root }), [props.root]);
  return React.createElement(GraphRootContext.Provider, { value: v }, props.children);
}

/** @emoji 🌐️ {@link Graph} for the current {@link GraphContextProvider} {@code root}. */
export function useGraph(): Graph {
  const store = useStore();
  const ctx = React.useContext(GraphRootContext);
  if (ctx == null) throw new Error('compose/react: useGraph requires <GraphContextProvider root="wip"|"authoritative">.');
  return React.useMemo(() => new Graph(store, ctx.root), [store, ctx.root]);
}

// #endregion 🌐️GraphContext

// #region 📐️Design
export type DesignContext = Readonly<{ designId: string }>;
const DesignContext = React.createContext<DesignContext | null>(null);
export function DesignContextProvider(props: { designId: string; children: React }): React.ReactElement {
  return React.createElement(DesignContext.Provider, { value: { designId: props.designId } }, props.children);
}
export function useDesign(): Design | null {
  const store = useStore();
  const ctx = React.useContext(DesignContext);
  return ctx == null ? null : store.design(ctx.designId);
}
// #endregion 📐️Design

// #endregion 🎭️Contexts

// #region 🪢️Contexts
export type PieceContext = Readonly<{ designId: string; pieceId: string }>;
const PieceContext = React.createContext<PieceContext | null>(null);
export function PieceContextProvider(props: PieceContext & { children: React }): React.ReactElement {
  return React.createElement(PieceContext.Provider, { value: { designId: props.designId, pieceId: props.pieceId } }, props.children);
}
export function usePiece(): Piece | null {
  const store = useStore();
  const ctx = React.useContext(PieceContext);
  return ctx == null ? null : store.design(ctx.designId).piece(ctx.pieceId);
}

export type TypeContext = Readonly<{ typeId: string }>;
const TypeContext = React.createContext<TypeContext | null>(null);
export function TypeContextProvider(props: { typeId: string; children: React }): React.ReactElement {
  return React.createElement(TypeContext.Provider, { value: { typeId: props.typeId } }, props.children);
}
export function useType(): Type | null {
  const store = useStore();
  const ctx = React.useContext(TypeContext);
  return ctx == null ? null : store.type(ctx.typeId);
}

export type ConnectionContext = Readonly<{ designId: string; connectionId: string }>;
const ConnectionContext = React.createContext<ConnectionContext | null>(null);
export function ConnectionContextProvider(props: ConnectionContext & { children: React }): React.ReactElement {
  return React.createElement(ConnectionContext.Provider, { value: { designId: props.designId, connectionId: props.connectionId } }, props.children);
}
export function useConnection(): Connection | null {
  const store = useStore();
  const ctx = React.useContext(ConnectionContext);
  return ctx == null ? null : store.design(ctx.designId).connection(ctx.connectionId);
}

export type PortContext = Readonly<{ typeId: string; portId: string }>;
const PortContext = React.createContext<PortContext | null>(null);
export function PortContextProvider(props: PortContext & { children: React }): React.ReactElement {
  return React.createElement(PortContext.Provider, { value: { typeId: props.typeId, portId: props.portId } }, props.children);
}
export function usePort(): Port | null {
  const store = useStore();
  const ctx = React.useContext(PortContext);
  return ctx == null ? null : store.type(ctx.typeId).port(ctx.portId);
}

export type ConnectorContext = Readonly<{ typeId: string; connectorId: string }>;
const ConnectorContext = React.createContext<ConnectorContext | null>(null);
export function ConnectorContextProvider(props: ConnectorContext & { children: React }): React.ReactElement {
  return React.createElement(ConnectorContext.Provider, { value: { typeId: props.typeId, connectorId: props.connectorId } }, props.children);
}
export function useConnector(): Connector | null {
  const store = useStore();
  const ctx = React.useContext(ConnectorContext);
  return ctx == null ? null : store.type(ctx.typeId).connector(ctx.connectorId);
}

export type QualityContext = Readonly<{ qualityId: string }>;
const QualityContext = React.createContext<QualityContext | null>(null);
export function QualityContextProvider(props: { qualityId: string; children: React }): React.ReactElement {
  return React.createElement(QualityContext.Provider, { value: { qualityId: props.qualityId } }, props.children);
}
export function useQuality(): Quality | null {
  const store = useStore();
  const ctx = React.useContext(QualityContext);
  return ctx == null ? null : store.quality(ctx.qualityId);
}

export type TagContext = Readonly<{ tagId: string }>;
const TagContext = React.createContext<TagContext | null>(null);
export function TagContextProvider(props: { tagId: string; children: React }): React.ReactElement {
  return React.createElement(TagContext.Provider, { value: { tagId: props.tagId } }, props.children);
}
export function useTag(): Tag | null {
  const store = useStore();
  const ctx = React.useContext(TagContext);
  return ctx == null ? null : store.tag(ctx.tagId);
}

export type ConceptContext = Readonly<{ conceptId: string }>;
const ConceptContext = React.createContext<ConceptContext | null>(null);
export function ConceptContextProvider(props: { conceptId: string; children: React }): React.ReactElement {
  return React.createElement(ConceptContext.Provider, { value: { conceptId: props.conceptId } }, props.children);
}
export function useConcept(): Concept | null {
  const store = useStore();
  const ctx = React.useContext(ConceptContext);
  return ctx == null ? null : store.concept(ctx.conceptId);
}

export type AuthorContext = Readonly<{ authorId: string }>;
const AuthorContext = React.createContext<AuthorContext | null>(null);
export function AuthorContextProvider(props: { authorId: string; children: React }): React.ReactElement {
  return React.createElement(AuthorContext.Provider, { value: { authorId: props.authorId } }, props.children);
}
export function useAuthor(): Author | null {
  const store = useStore();
  const ctx = React.useContext(AuthorContext);
  return ctx == null ? null : store.author(ctx.authorId);
}

export type RepresentationContext = Readonly<{ typeId: string; representationId: string }>;
const RepresentationContext = React.createContext<RepresentationContext | null>(null);
export function RepresentationContextProvider(props: RepresentationContext & { children: React }): React.ReactElement {
  return React.createElement(RepresentationContext.Provider, { value: { typeId: props.typeId, representationId: props.representationId } }, props.children);
}
export function useRepresentation(): Representation | null {
  const store = useStore();
  const ctx = React.useContext(RepresentationContext);
  return ctx == null ? null : store.type(ctx.typeId).representation(ctx.representationId);
}

function useResolvedDesign(designId?: string): Design | null {
  const fromContext = useDesign();
  const store = useStoreOptional();
  const resolvedId = designId ?? fromContext?.id ?? null;
  return React.useMemo(() => {
    if (resolvedId == null) return fromContext;
    return store != null ? store.design(resolvedId) : fromContext;
  }, [fromContext, resolvedId, store]);
}

function useResolvedType(typeId?: string): Type | null {
  const fromContext = useType();
  const store = useStoreOptional();
  const resolvedId = typeId ?? fromContext?.id ?? null;
  return React.useMemo(() => {
    if (resolvedId == null) return fromContext;
    return store != null ? store.type(resolvedId) : fromContext;
  }, [fromContext, resolvedId, store]);
}
// #endregion 🪢️Contexts

// #region 🔖️EntityContextHelpers
/** @emoji 🧭️ `{ id }` shaped like golden {@code } from {@link DesignContext} for sketchpad routing (no entity fetch). */
export function useDesignContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(DesignContext);
  return ctx == null ? null : { id: ctx.designId };
}

/** @emoji 🧭️ True when a {@link DesignContextProvider} is mounted above. */
export function useHasDesignContext(): boolean {
  return React.useContext(DesignContext) != null;
}

/** @emoji 🧭️ `{ id }` from {@link PieceContext} (piece id only), {@code }-shaped. */
export function usePieceContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(PieceContext);
  return ctx == null ? null : { id: ctx.pieceId };
}

/** @emoji 🧭️ True when {@link PieceContextProvider} is mounted above. */
export function useHasPieceContext(): boolean {
  return React.useContext(PieceContext) != null;
}

/** @emoji 🧭️ `{ id }` from {@link ConnectionContext}, {@code }-shaped. */
export function useConnectionContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(ConnectionContext);
  return ctx == null ? null : { id: ctx.connectionId };
}

/** @emoji 🧭️ True when {@link ConnectionContextProvider} is mounted above. */
export function useHasConnectionContext(): boolean {
  return React.useContext(ConnectionContext) != null;
}

/** @emoji 🧭️ `{ id }` from {@link TypeContext}, {@code }-shaped. */
export function useTypeContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(TypeContext);
  return ctx == null ? null : { id: ctx.typeId };
}

/** @emoji 🧭️ True when {@link TypeContextProvider} is mounted above. */
export function useHasTypeContext(): boolean {
  return React.useContext(TypeContext) != null;
}

/** @emoji 🧭️ `{ id }` from {@link QualityContext}, {@code }-shaped. */
export function useQualityContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(QualityContext);
  return ctx == null ? null : { id: ctx.qualityId };
}

/** @emoji 🧭️ True when {@link QualityContextProvider} is mounted above. */
export function useHasQualityContext(): boolean {
  return React.useContext(QualityContext) != null;
}

/** @emoji 🧭️ `{ id }` from {@link AuthorContext}, {@code }-shaped. */
export function useAuthorContext(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(AuthorContext);
  return ctx == null ? null : { id: ctx.authorId };
}

/** @emoji 🧭️ True when {@link AuthorContextProvider} is mounted above. */
export function useHasAuthorContext(): boolean {
  return React.useContext(AuthorContext) != null;
}

/** @emoji 🧷️ {@link PieceContextProvider} using the enclosing {@link DesignContextProvider} {@code designId}. */
export function PieceUnderActiveDesignProvider(props: { pieceId: string; children: React }): React.ReactElement {
  const d = React.useContext(DesignContext);
  if (d == null) {
    throw new Error('compose/react: PieceUnderActiveDesignProvider requires <DesignContextProvider designId="…">.');
  }
  return React.createElement(PieceContext.Provider, { value: { designId: d.designId, pieceId: props.pieceId } }, props.children);
}

/** @emoji 🧷️ {@link ConnectionContextProvider} using the enclosing {@link DesignContextProvider} {@code designId}. */
export function ConnectionUnderActiveDesignProvider(props: { connectionId: string; children: React }): React.ReactElement {
  const d = React.useContext(DesignContext);
  if (d == null) {
    throw new Error('compose/react: ConnectionUnderActiveDesignProvider requires <DesignContextProvider designId="…">.');
  }
  return React.createElement(ConnectionContext.Provider, { value: { designId: d.designId, connectionId: props.connectionId } }, props.children);
}
// #endregion 🔖️EntityContextHelpers

// #region 🪝️IdStableEntityLists
/** @emoji 📚️ Kit-level designs via {@link Kit#readDesigns} (id-list-stable handles). */
export function useKitDesigns(): FieldReadState<readonly Design[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.designs());
}

/** @emoji 📚️ Kit-level kinds via {@link Kit#readTypes}. */
export function useKitTypes(): FieldReadState<readonly Type[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.types());
}

/** @emoji 📚️ Kit-level authors via {@link Kit#readAuthors}. */
export function useKitAuthors(): FieldReadState<readonly Author[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.authors());
}

/** @emoji 📚️ Kit-level qualities via {@link Kit#readQualities}. */
export function useKitQualities(): FieldReadState<readonly Quality[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.qualities());
}

/** @emoji 📚️ Kit-level tags via {@link Kit#readTags}. */
export function useKitTags(): FieldReadState<readonly Tag[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.tags());
}

/** @emoji 📚️ Kit-level concepts via {@link Kit#readConcepts}. */
export function useKitConcepts(): FieldReadState<readonly Concept[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.concepts());
}

/** @emoji 📚️ Design pieces ordered by {@link Design#readPieceIds} (stable {@link Piece} handles per id). */
export function useDesignPieces(): FieldReadState<readonly Piece[]> {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, (design) => design.pieces());
}

/** @emoji 📚️ Design connections ordered by {@link Design#readConnectionIds}. */
export function useDesignConnections(): FieldReadState<readonly Connection[]> {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, (design) => design.connections());
}
// #endregion 🪝️IdStableEntityLists

// #region 🪝️AggregateListBundles
/** @emoji 📚️ Sketchpad bundle: live kit designs (id-list-stable). */
export function useDesigns(): Readonly<{ designs: readonly Design[] }> {
  const { value } = useKitDesigns();
  return { designs: value ?? [] };
}

/** @emoji 📚️ Sketchpad bundle: live kit kinds (id-list-stable). */
export function useTypes(): Readonly<{ types: readonly Type[] }> {
  const { value } = useKitTypes();
  return { types: value ?? [] };
}

/** @emoji 📚️ Pieces in the active {@link DesignContextProvider} design (see {@link DesignContext}). */
export function usePieces(): readonly Piece[] {
  const { value } = useDesignPieces();
  return value ?? [];
}
// #endregion 🪝️AggregateListBundles

// #region 🪝️EntityContextReads
/** @emoji 📖️ Resolves a kit {@link Piece} from context and/or ids, then applies {@code selector}. */
export function usePieceContextRead<T>(selector: (p: Piece) => T, pieceId?: string | undefined, _deep?: boolean): T {
  const ctxPiece = usePiece();
  const k = useStoreOptional();
  const dctx = React.useContext(DesignContext);
  const resolvedPieceId = pieceId ?? ctxPiece?.id ?? null;
  if (resolvedPieceId == null || k == null || dctx == null) {
    return selector(ctxPiece as Piece);
  }
  return selector(k.design(dctx.designId).piece(resolvedPieceId));
}

/** @emoji 📖️ Resolves a kit {@link Type} handle from context and/or id, then applies {@code selector}. */
export function useTypeContextRead<S>(selector: ((t: Type) => S) | undefined, kindId?: string | undefined, deep?: boolean): S | Type | undefined {
  const fromCtx = useType();
  const k = useStoreOptional();
  const resolvedId = kindId ?? fromCtx?.id ?? null;
  if (resolvedId == null || k == null) {
    if (fromCtx == null) return undefined;
    if (selector == null) return deep === true ? fromCtx : undefined;
    return selector(fromCtx);
  }
  const t = k.type(resolvedId);
  if (selector == null) return deep === true ? t : undefined;
  return selector(t);
}

/** @emoji 📖️ Resolves a kit {@link Quality} from context and/or id, then applies {@code selector}. */
export function useQualityContextRead<S>(selector: ((q: Quality) => S) | undefined, qualityId?: string | undefined, deep?: boolean): S | Quality | undefined {
  const fromCtx = useQuality();
  const k = useStoreOptional();
  const resolvedId = qualityId ?? fromCtx?.id ?? null;
  if (resolvedId == null || k == null) {
    if (fromCtx == null) return undefined;
    if (selector == null) return deep === true ? fromCtx : undefined;
    return selector(fromCtx);
  }
  const q = k.quality(resolvedId);
  if (selector == null) return deep === true ? q : undefined;
  return selector(q);
}
// #endregion 🪝️EntityContextReads

// #region 🪝️HooksKit
// #region 📖️KitReads
/** @emoji 📖️ Live {@link Kit#name} + {@code kitRenamed}. */
export function useKitName(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.name(), "kitRenamed");
}

/** @emoji 📖️ Live {@link Kit#description} + {@code changedDescription}. */
export function useKitDescription(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.description(), "changedDescription");
}

/** @emoji 📖️ Live {@link Kit#id}. */
export function useKitId(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => k.id);
}

/** @emoji 📖️ Live {@link Kit#icon}. */
export function useKitIcon(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.icon());
}

/** @emoji 📖️ Live {@link Kit#image}. */
export function useKitImage(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.image());
}

/** @emoji 🧾️ Exposes {@link Store#ensureChangeId} as a stable callback. */
export function useEnsureKitChangeId(): () => Promise<string> {
  const store = useStore();
  return React.useCallback(() => store.ensureChangeId(), [store]);
}
// #endregion 📖️KitReads

// #region ✍️KitWrites
/** @emoji ✍️ {@link Store#rename}. */
export function useRenameKit(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, newName) => k.rename(newName))(() => store);
}

/** @emoji ✍️ {@link Store#changeDescription}. */
export function useChangeKitDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, d) => k.changeDescription(d))(() => store);
}

/** @emoji ✍️ {@link Store#createTag}. */
export function useCreateTag(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createTag(n, d, i, o))(() => store);
}

/** @emoji ✍️ {@link Store#deleteTag}. */
export function useDeleteTag(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.deleteTag(id))(() => store);
}

/** @emoji ✍️ {@link Store#deleteTags}. */
export function useDeleteTags(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[readonly string[]]>((k, ids) => k.deleteTags(ids))(() => store);
}

/** @emoji ✍️ {@link Store#createConcept}. */
export function useCreateConcept(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createConcept(n, d, i, o))(() => store);
}

/** @emoji ✍️ {@link Store#deleteConcept}. */
export function useDeleteConcept(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.deleteConcept(id))(() => store);
}

/** @emoji ✍️ {@link Store#deleteConcepts}. */
export function useDeleteConcepts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[readonly string[]]>((k, ids) => k.deleteConcepts(ids))(() => store);
}

/** @emoji ✍️ {@link Store#createQuality}. */
export function useCreateQuality(): readonly [(key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, key, value, unit, definition, description, icon) =>
    k.createQuality(key, value, unit, definition, description, icon),
  )(() => store);
}

/** @emoji ✍️ {@link Store#deleteQuality}. */
export function useDeleteQuality(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.deleteQuality(id))(() => store);
}

/** @emoji ✍️ {@link Store#deleteQualities}. */
export function useDeleteQualities(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[readonly string[]]>((k, ids) => k.deleteQualities(ids))(() => store);
}

/** @emoji ✍️ {@link Store#createType}. */
export function useCreateType(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createType(n, d, i, im, u))(() => store);
}

/** @emoji ✍️ {@link Store#deleteType}. */
export function useDeleteType(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.deleteType(id))(() => store);
}

/** @emoji ✍️ {@link Store#deleteTypes}. */
export function useDeleteTypes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[readonly string[]]>((k, ids) => k.deleteTypes(ids))(() => store);
}

/** @emoji ✍️ {@link Store#createDesign}. */
export function useCreateDesign(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createDesign(n, d, i, im, u))(() => store);
}

/** @emoji ✍️ {@link Store#deleteDesign}. */
export function useDeleteDesign(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.deleteDesign(id))(() => store);
}

/** @emoji ✍️ {@link Store#deleteDesigns}. */
export function useDeleteDesigns(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[readonly string[]]>((k, ids) => k.deleteDesigns(ids))(() => store);
}

/** @emoji ✍️ {@link Store#saveChange}. */
export function useSaveKitChange(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[]>(async (k) => {
    await k.saveChange();
    return { ok: true };
  })(() => store);
}

/** @emoji ✍️ {@link Store#createCheckpoint}. */
export function useCreateCheckpoint(): readonly [(message: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, message) => k.createCheckpoint(message))(() => store);
}

/** @emoji ✍️ {@link Store#startAlternative}. */
export function useStartAlternative(): readonly [(name?: string | null) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string | null | undefined]>((k, name) => k.startAlternative(name ?? undefined))(() => store);
}

/** @emoji ✍️ {@link Store#integrateAlternative}. */
export function useIntegrateAlternative(): readonly [(alternativeId: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, id) => k.integrateAlternative(id))(() => store);
}

/** @emoji ✍️ {@link Store#login}. */
export function useLogin(): readonly [(username: string, passwordHash: string, hubUrl?: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string, string, string | undefined]>((k, u, p, h) => k.login(u, p, h))(() => store);
}

/** @emoji ✍️ {@link Store#logout}. */
export function useLogout(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[]>((k) => k.logout())(() => store);
}

/** @emoji ✍️ {@link Store#sessionStart}. */
export function useStartSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[]>((k) => k.sessionStart())(() => store);
}

/** @emoji ✍️ {@link Store#sessionEnd}. */
export function useEndSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[]>((k) => k.sessionEnd())(() => store);
}

// #region 🪝️BackboneOps
/** @emoji 🛜️ {@link Store#attachBackbone} — GraphQL session backbone attach (URI scheme dispatches backbone kind). */
export function useAttachBackbone(): readonly [(uri: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, uri) => k.attachBackbone(uri))(() => store);
}

/** @emoji 🛜️ {@link Store#detachBackbone}. */
export function useDetachBackbone(): readonly [(uri: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[string]>((k, uri) => k.detachBackbone(uri))(() => store);
}

/** @emoji 🛜️ {@link Store#backboneSyncNow}. */
export function useBackboneSyncNow(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  return bindStoreOperationToReact<[]>((k) => k.backboneSyncNow())(() => store);
}

/** @emoji 🛜️ Live {@link Store#backboneStatus} (refreshes on {@code commandSucceeded} bus events). */
export function useBackboneStatus(): FieldReadState<Readonly<{ attachedUri: string | null; kind: string }>> {
  const store = useStore();
  return bindStoreFieldToReact<Readonly<{ attachedUri: string | null; kind: string }>>({
    getStore: () => store,
    read: (s) => s.backboneStatus(),
    eventKind: "commandSucceeded",
  })();
}
// #endregion 🪝️BackboneOps

// #endregion 🪝️HooksKit

// #region 🪝️HooksDesign
// #region 📖️DesignReads
/** @emoji 📖️ Live {@link Design#name}. */
export function useDesignName(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.name());
}

/** @emoji 📖️ Live {@link Design#description} + {@code changedDescription}. */
export function useDesignDescription(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.description(), "changedDescription");
}

/** @emoji 📖️ Live {@link Design#qualitySum}. */
export function useDesignQualitySum(designId?: string): FieldReadState<number> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.qualitySum());
}
// #endregion 📖️DesignReads

// #region ✍️DesignWrites
/** @emoji ✍️ {@link Design#rename}. */
export function useRenameDesign(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string]>((d, n) => d.rename(n))(() => entity);
}

/** @emoji ✍️ {@link Design#changeDescription}. */
export function useChangeDesignDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string]>((d, t) => d.changeDescription(t))(() => entity);
}

/** @emoji ✍️ {@link Design#flatten}. */
export function useFlattenDesign(): readonly [() => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, []>((d) => d.flatten())(() => entity);
}

/** @emoji ✍️ {@link Design#addAttribute}. */
export function useAddDesignAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string, string, string]>((d, k, v, def) => d.addAttribute(k, v, def))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttribute}. */
export function useRemoveDesignAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string]>((d, id) => d.removeAttribute(id))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttributes}. */
export function useRemoveDesignAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [readonly string[]]>((d, ids) => d.removeAttributes(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#addFixedPiece}. */
export function useAddFixedPiece(): readonly [(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string, PositionInput, string | null | undefined, string | null | undefined]>((d, bp, pos, n, desc) => d.addFixedPiece(bp, pos, n, desc))(() => entity);
}

/** @emoji ✍️ {@link Design#addChildPieceWithParentConnection}. */
export function useAddChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, name?: string | null, description?: string | null, position?: PositionInput | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string, string, string, string, string | null | undefined, string | null | undefined, PositionInput | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, n, desc, pos, sc) =>
    d.addChildPieceWithParentConnection(bp, pp, pc, cc, n, desc, pos, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#addHangingChildPieceWithParentConnection}. */
export function useAddHangingChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, position: PositionInput, name?: string | null, description?: string | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string, string, string, string, PositionInput, string | null | undefined, string | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, pos, n, desc, sc) =>
    d.addHangingChildPieceWithParentConnection(bp, pp, pc, cc, pos, n, desc, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiece}. */
export function useDeleteDesignPiece(): readonly [(pieceId: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [string]>((d, id) => d.deletePiece(id))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePieces}. */
export function useDeleteDesignPieces(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [readonly string[]]>((d, ids) => d.deletePieces(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiecesAndConnections}. */
export function useDeleteDesignPiecesAndConnections(): readonly [(pieceIds: readonly string[], connectionIds: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOperationToReact<Design, [readonly string[], readonly string[]]>((d, p, c) => d.deletePiecesAndConnections(p, c))(() => entity);
}
// #endregion ✍️DesignWrites
// #endregion 🪝️HooksDesign

// #region 🧰️Type
const useTypeRenameOperation = bindOperationToReact<Type, [string]>((t, newName) => t.rename(newName));
const useTypeChangeDescriptionOperation = bindOperationToReact<Type, [string]>((t, d) => t.changeDescription(d));
const useTypeChangeIconOperation = bindOperationToReact<Type, [string]>((t, i) => t.changeIcon(i));
const useTypeAddAttributeOperation = bindOperationToReact<Type, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTypeRemoveAttributeOperation = bindOperationToReact<Type, [string]>((t, id) => t.removeAttribute(id));
const useTypeRemoveAttributesOperation = bindOperationToReact<Type, [readonly string[]]>((t, ids) => t.removeAttributes(ids));
const useTypeCreatePortOperation = bindOperationToReact<Type, [string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, number | null | undefined]>((t, code, label, description, icon, order) =>
  t.createPort(code ?? null, label ?? null, description ?? null, icon ?? null, order ?? null),
);
const useTypeDeletePortOperation = bindOperationToReact<Type, [string]>((t, id) => t.deletePort(id));
const useTypeDeletePortsOperation = bindOperationToReact<Type, [readonly string[]]>((t, ids) => t.deletePorts(ids));
const useTypeAddConnectorOperation = bindOperationToReact<Type, [string, string | null | undefined, string | null | undefined, string | null | undefined]>((t, code, description, icon, portId) =>
  t.addConnector(code, description ?? null, icon ?? null, portId ?? null),
);
const useTypeRemoveConnectorOperation = bindOperationToReact<Type, [string]>((t, id) => t.removeConnector(id));
const useTypeRemoveConnectorsOperation = bindOperationToReact<Type, [readonly string[]]>((t, ids) => t.removeConnectors(ids));

/** @emoji 📖️ Live {@link Type#name}. */
export function useTypeName(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖️ Live {@link Type#description}. */
export function useTypeDescription(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖️ Live {@link Type#icon}. */
export function useTypeIcon(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖️ Live {@link Type#image}. */
export function useTypeImage(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.image());
}

/** @emoji 📖️ Live {@link Type#unit}. */
export function useTypeUnit(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.unit());
}

/** @emoji 📖️ Bulky {@link Type#connectors}. */
export function useTypeConnectors(typeId?: string): FieldReadState<readonly Connector[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.connectors());
}

/** @emoji 📖️ Bulky {@link Type#representations}. */
export function useTypeRepresentations(typeId?: string): FieldReadState<readonly Representation[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.representations());
}

/** @emoji 📖️ Bulky {@link Type#attributes}. */
export function useTypeAttributes(typeId?: string): FieldReadState<readonly Attribute[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji ✍️ {@link TypeOperationInput#rename}. */
export function useRenameType(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRenameOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeDescription}. */
export function useChangeTypeDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeIcon}. */
export function useChangeTypeIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addAttribute}. */
export function useAddTypeAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttribute}. */
export function useRemoveTypeAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttributes}. */
export function useRemoveTypeAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveAttributesOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#createPort}. */
export function useCreatePort(): readonly [(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeCreatePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePort}. */
export function useDeletePort(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeDeletePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePorts}. */
export function useDeletePorts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeDeletePortsOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addConnector}. */
export function useAddConnector(): readonly [(code: string, description?: string | null, icon?: string | null, portId?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeAddConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnector}. */
export function useRemoveConnector(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnectors}. */
export function useRemoveConnectors(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveConnectorsOperation(() => e);
}
// #endregion 🧰️Type

// #region 🔘️Port
const usePortRenameOperation = bindOperationToReact<Port, [string, string | null | undefined]>((p, newCode, newLabel) => p.rename(newCode, newLabel));
const usePortChangeDescriptionOperation = bindOperationToReact<Port, [string]>((p, d) => p.changeDescription(d));
const usePortChangeIconOperation = bindOperationToReact<Port, [string]>((p, i) => p.changeIcon(i));
const usePortAddAttributeOperation = bindOperationToReact<Port, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePortRemoveAttributeOperation = bindOperationToReact<Port, [string]>((p, id) => p.removeAttribute(id));
const usePortRemoveAttributesOperation = bindOperationToReact<Port, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji 📖️ Live {@link Port#code}. */
export function usePortCode(): FieldReadState<string> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.code());
}

/** @emoji 📖️ Live {@link Port#label}. */
export function usePortLabel(): FieldReadState<string> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.label());
}

/** @emoji 📖️ Live {@link Port#order}. */
export function usePortOrder(): FieldReadState<number | null> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.order());
}

/** @emoji 📖️ Live {@link Port#name}. */
export function usePortName(): FieldReadState<string> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖️ Live {@link Port#description}. */
export function usePortDescription(): FieldReadState<string> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖️ Live {@link Port#icon}. */
export function usePortIcon(): FieldReadState<string> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖️ Bulky {@link Port#attributes}. */
export function usePortAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = usePort();
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji ✍️ {@link PortOperationInput#rename}. */
export function useRenamePort(): readonly [(newCode: string, newLabel?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRenameOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeDescription}. */
export function useChangePortDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeIcon}. */
export function useChangePortIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#addAttribute}. */
export function useAddPortAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttribute}. */
export function useRemovePortAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttributes}. */
export function useRemovePortAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRemoveAttributesOperation(() => e);
}
// #endregion 🔘️Port

// #region 🔗️Connector
const useConnectorRenameOperation = bindOperationToReact<Connector, [string]>((c, newCode) => c.rename(newCode));
const useConnectorChangeDescriptionOperation = bindOperationToReact<Connector, [string]>((c, d) => c.changeDescription(d));
const useConnectorChangeIconOperation = bindOperationToReact<Connector, [string]>((c, i) => c.changeIcon(i));

/** @emoji 📖️ Live {@link Connector#code}. */
export function useConnectorCode(): FieldReadState<string> {
  const entity = useConnector();
  return useCurrentEntityField(entity, (c) => c.code());
}

/** @emoji 📖️ Live {@link Connector#description}. */
export function useConnectorDescription(): FieldReadState<string> {
  const entity = useConnector();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖️ Live {@link Connector#icon}. */
export function useConnectorIcon(): FieldReadState<string> {
  const entity = useConnector();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖️ Bulky {@link Connector#attributes}. */
export function useConnectorAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConnector();
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link ConnectorOperationInput#rename}. */
export function useRenameConnector(): readonly [(newCode: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorRenameOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeDescription}. */
export function useChangeConnectorDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeIcon}. */
export function useChangeConnectorIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorChangeIconOperation(() => e);
}
// #endregion 🔗️Connector

// #region ✍️Author
/** @emoji 📖️ Live {@link Author#name}. */
export function useAuthorName(): FieldReadState<string> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.name());
}

/** @emoji 📖️ Live {@link Author#email}. */
export function useAuthorEmail(): FieldReadState<string> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.email());
}

/** @emoji 📖️ Live {@link Author#rank}. */
export function useAuthorRank(): FieldReadState<number | null> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.rank());
}

/** @emoji 📖️ Live {@link Author#description}. */
export function useAuthorDescription(): FieldReadState<string> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.description());
}

/** @emoji 📖️ Live {@link Author#icon}. */
export function useAuthorIcon(): FieldReadState<string> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.icon());
}

/** @emoji 📖️ Live {@link Author#role}. */
export function useAuthorRole(): FieldReadState<string> {
  const entity = useAuthor();
  return useCurrentEntityField(entity, (a) => a.role());
}
// #endregion ✍️Author

// #region 💎️Quality
const useQualityRenameOperation = bindOperationToReact<Quality, [string]>((q, k) => q.rename(k));
const useQualityChangeDescriptionOperation = bindOperationToReact<Quality, [string]>((q, d) => q.changeDescription(d));
const useQualityChangeIconOperation = bindOperationToReact<Quality, [string]>((q, i) => q.changeIcon(i));
const useQualityAddAttributeOperation = bindOperationToReact<Quality, [string, string, string]>((q, key, value, definition) => q.addAttribute(key, value, definition));
const useQualityRemoveAttributeOperation = bindOperationToReact<Quality, [string]>((q, id) => q.removeAttribute(id));
const useQualityRemoveAttributesOperation = bindOperationToReact<Quality, [readonly string[]]>((q, ids) => q.removeAttributes(ids));

/** @emoji 📖️ Live {@link Quality#key}. */
export function useQualityKey(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.key());
}

/** @emoji 📖️ Live {@link Quality#value}. */
export function useQualityValue(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.value());
}

/** @emoji 📖️ Live {@link Quality#unit}. */
export function useQualityUnit(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.unit());
}

/** @emoji 📖️ Live {@link Quality#definition}. */
export function useQualityDefinition(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.definition());
}

/** @emoji 📖️ Live {@link Quality#description}. */
export function useQualityDescription(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.description());
}

/** @emoji 📖️ Live {@link Quality#icon}. */
export function useQualityIcon(): FieldReadState<string> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.icon());
}

/** @emoji 📖️ Live {@link Quality#attributes}. */
export function useQualityAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.attributes());
}

/** @emoji 📖️ Live {@link Quality#benchmarks}. */
export function useQualityBenchmarks(): FieldReadState<readonly Benchmark[]> {
  const entity = useQuality();
  return useCurrentEntityField(entity, (q) => q.benchmarks());
}

/** @emoji ✍️ {@link Quality#rename}. */
export function useRenameQuality(): readonly [(newKey: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRenameOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeDescription}. */
export function useChangeQualityDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeIcon}. */
export function useChangeQualityIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Quality#addAttribute}. */
export function useAddQualityAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttribute}. */
export function useRemoveQualityAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttributes}. */
export function useRemoveQualityAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRemoveAttributesOperation(() => e);
}
// #endregion 💎️Quality

// #region 🏷️Tag
const useTagRenameOperation = bindOperationToReact<Tag, [string]>((t, n) => t.rename(n));
const useTagChangeDescriptionOperation = bindOperationToReact<Tag, [string]>((t, d) => t.changeDescription(d));
const useTagChangeIconOperation = bindOperationToReact<Tag, [string]>((t, i) => t.changeIcon(i));
const useTagAddAttributeOperation = bindOperationToReact<Tag, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTagRemoveAttributeOperation = bindOperationToReact<Tag, [string]>((t, id) => t.removeAttribute(id));
const useTagRemoveAttributesOperation = bindOperationToReact<Tag, [readonly string[]]>((t, ids) => t.removeAttributes(ids));

/** @emoji 📖️ Live {@link Tag#name}. */
export function useTagName(): FieldReadState<string> {
  const entity = useTag();
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖️ Live {@link Tag#description}. */
export function useTagDescription(): FieldReadState<string> {
  const entity = useTag();
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖️ Live {@link Tag#icon}. */
export function useTagIcon(): FieldReadState<string> {
  const entity = useTag();
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖️ Live {@link Tag#order}. */
export function useTagOrder(): FieldReadState<number | null> {
  const entity = useTag();
  return useCurrentEntityField(entity, (t) => t.order());
}

/** @emoji 📖️ Live {@link Tag#attributes}. */
export function useTagAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useTag();
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji ✍️ {@link Tag#rename}. */
export function useRenameTag(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRenameOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeDescription}. */
export function useChangeTagDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeIcon}. */
export function useChangeTagIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Tag#addAttribute}. */
export function useAddTagAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttribute}. */
export function useRemoveTagAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttributes}. */
export function useRemoveTagAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRemoveAttributesOperation(() => e);
}
// #endregion 🏷️Tag

// #region 💡️Concept
const useConceptRenameOperation = bindOperationToReact<Concept, [string]>((c, n) => c.rename(n));
const useConceptChangeDescriptionOperation = bindOperationToReact<Concept, [string]>((c, d) => c.changeDescription(d));
const useConceptChangeIconOperation = bindOperationToReact<Concept, [string]>((c, i) => c.changeIcon(i));
const useConceptAddAttributeOperation = bindOperationToReact<Concept, [string, string, string]>((c, key, value, definition) => c.addAttribute(key, value, definition));
const useConceptRemoveAttributeOperation = bindOperationToReact<Concept, [string]>((c, id) => c.removeAttribute(id));
const useConceptRemoveAttributesOperation = bindOperationToReact<Concept, [readonly string[]]>((c, ids) => c.removeAttributes(ids));

/** @emoji 📖️ Live {@link Concept#name}. */
export function useConceptName(): FieldReadState<string> {
  const entity = useConcept();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖️ Live {@link Concept#description}. */
export function useConceptDescription(): FieldReadState<string> {
  const entity = useConcept();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖️ Live {@link Concept#icon}. */
export function useConceptIcon(): FieldReadState<string> {
  const entity = useConcept();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖️ Live {@link Concept#order}. */
export function useConceptOrder(): FieldReadState<number | null> {
  const entity = useConcept();
  return useCurrentEntityField(entity, (c) => c.order());
}

/** @emoji 📖️ Live {@link Concept#attributes}. */
export function useConceptAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConcept();
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link Concept#rename}. */
export function useRenameConcept(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRenameOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeDescription}. */
export function useChangeConceptDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeIcon}. */
export function useChangeConceptIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Concept#addAttribute}. */
export function useAddConceptAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttribute}. */
export function useRemoveConceptAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttributes}. */
export function useRemoveConceptAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRemoveAttributesOperation(() => e);
}
// #endregion 💡️Concept

// #region 🎨️Representation
/** @emoji 📖️ Live {@link Representation#url}. */
export function useRepresentationUrl(): FieldReadState<string> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.url());
}

/** @emoji 📖️ Live {@link Representation#description}. */
export function useRepresentationDescription(): FieldReadState<string> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.description());
}

/** @emoji 📖️ Live {@link Representation#tags}. */
export function useRepresentationTags(): FieldReadState<readonly Tag[]> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.tags());
}

/** @emoji 📖️ Live {@link Representation#qualities}. */
export function useRepresentationQualities(): FieldReadState<readonly Quality[]> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.qualities());
}

/** @emoji 📖️ Live {@link Representation#attributes}. */
export function useRepresentationAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.attributes());
}

/** @emoji 📖️ Live {@link Representation#file}. */
export function useRepresentationFile(): FieldReadState<File | null> {
  const entity = useRepresentation();
  return useCurrentEntityField(entity, (r) => r.file());
}
// #endregion 🎨️Representation

// #region 🧩️Piece
/** @emoji 📖️ Live {@link Piece#name}. */
export function usePieceName(): FieldReadState<string> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖️ Live {@link Piece#description}. */
export function usePieceDescription(): FieldReadState<string> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖️ Live {@link Piece#icon}. */
export function usePieceIcon(): FieldReadState<string> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖️ Live {@link Piece#scale}. */
export function usePieceScale(): FieldReadState<number | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.scale());
}

/** @emoji 📖️ Live {@link Piece#position}. */
export function usePiecePosition(): FieldReadState<Position> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.position());
}

/** @emoji 📖️ Live {@link Piece#flatPosition}. */
export function usePieceFlatPosition(): FieldReadState<Position> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition());
}

/** @emoji 📖️ Live {@link Piece#plane}. */
export function usePiecePlane(): FieldReadState<Plane | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.position().plane());
}

/** @emoji 📖️ Live {@link Piece#center}. */
export function usePieceCenter(): FieldReadState<Coordinate | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.position().center());
}

/** @emoji 📖️ Live {@link Piece#flatPlane}. */
export function usePieceFlatPlane(): FieldReadState<Plane | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition().plane());
}

/** @emoji 📖️ Live {@link Piece#flatCenter}. */
export function usePieceFlatCenter(): FieldReadState<Coordinate | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition().center());
}

/** @emoji 📖️ Live {@link Piece#blueprint}. */
export function usePieceBlueprint(): FieldReadState<PieceBlueprint | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.blueprint());
}

/** @emoji 📖️ Live {@link Piece#attributes}. */
export function usePieceAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji 📖️ Live {@link Piece#connectionKind}. */
export function usePieceConnectionKind(): FieldReadState<"FIXED" | "CONNECTED" | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.connectionKind());
}

/** @emoji 📖️ Live {@link Piece#parentPiece}. */
export function usePieceParentPiece(): FieldReadState<Piece | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.parentPiece());
}

/** @emoji 📖️ Live {@link Piece#parentConnection}. */
export function usePieceParentConnection(): FieldReadState<Connection | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.parentConnection());
}

/** @emoji 📖️ Live {@link Piece#childPieces}. */
export function usePieceChildPieces(): FieldReadState<readonly Piece[]> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.childPieces());
}

/** @emoji 📖️ Live {@link Piece#childConnections}. */
export function usePieceChildConnections(): FieldReadState<readonly Connection[]> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.childConnections());
}

/** @emoji 📖️ Live {@link Piece#depth}. */
export function usePieceDepth(): FieldReadState<number | null> {
  const entity = usePiece();
  return useCurrentEntityField(entity, (p) => p.depth());
}

const usePieceRenameOperation = bindOperationToReact<Piece, [string]>((p, n) => p.rename(n));
const usePieceChangeDescriptionOperation = bindOperationToReact<Piece, [string]>((p, d) => p.changeDescription(d));
const usePieceDragOperation = bindOperationToReact<Piece, [OffsetInput]>((p, o) => p.drag(o));
const usePieceMoveOperation = bindOperationToReact<Piece, [PositionInput]>((p, pos) => p.move(pos));
const usePieceFixOperation = bindOperationToReact<Piece, []>((p) => p.fix());
const usePieceChangeBlueprintOperation = bindOperationToReact<Piece, [string]>((p, id) => p.changeBlueprint(id));
const usePieceAddAttributeOperation = bindOperationToReact<Piece, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePieceRemoveAttributeOperation = bindOperationToReact<Piece, [string]>((p, id) => p.removeAttribute(id));
const usePieceRemoveAttributesOperation = bindOperationToReact<Piece, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji ✍️ {@link Piece#rename} bound to {@link PieceContext}. */
export function useRenamePiece(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRenameOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeDescription}. */
export function useChangePieceDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Piece#drag}. */
export function useDragPiece(): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceDragOperation(() => e);
}

/** @emoji ✍️ {@link Piece#move}. */
export function useMovePiece(): readonly [(position: PositionInput) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceMoveOperation(() => e);
}

/** @emoji ✍️ {@link Piece#fix}. */
export function useFixPiece(): readonly [() => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceFixOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeBlueprint}. */
export function useChangePieceBlueprint(): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceChangeBlueprintOperation(() => e);
}

/** @emoji ✍️ {@link Piece#addAttribute}. */
export function useAddPieceAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttribute}. */
export function useRemovePieceAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttributes}. */
export function useRemovePieceAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRemoveAttributesOperation(() => e);
}
// #endregion 🧩️Piece

// #region 🪢️Pieces
/**
 * @emoji 🪝️ Binds {@link PiecesOperations} batch mutations (not an {@link } — no cached kit state on the handle).
 * @typeParam Args — forwarded to the underlying {@link PiecesOperations} method after the ops handle.
 */
function bindPiecesOperationsOperationToReact<Args extends unknown[]>(impl: (ops: PiecesOperations, ...args: Args) => Promise<SetResult>): (getOps: () => PiecesOperations | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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
            error: { kind: "Disposed", message: "No pieces batch scope (empty ids or missing store).", field: undefined, entity: undefined },
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

const usePiecesDragOperation = bindPiecesOperationsOperationToReact((ops, o: OffsetInput) => ops.drag(o));
const usePiecesMoveOperation = bindPiecesOperationsOperationToReact((ops, o: OffsetInput) => ops.move(o));
const usePiecesFixOperation = bindPiecesOperationsOperationToReact((ops) => ops.fix());
const usePiecesChangeBlueprintOperation = bindPiecesOperationsOperationToReact((ops, id: string) => ops.changeBlueprint(id));

/** @emoji ✍️ {@link PiecesOperations#drag} for {@code design.pieces(ids)}. */
export function useDragPieces(designId: string, pieceIds: readonly string[]): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : new PiecesOperations(store, designId, pieceIds)), [store, designId, pieceIds]);
  return usePiecesDragOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#move}. */
export function useMovePieces(designId: string, pieceIds: readonly string[]): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : new PiecesOperations(store, designId, pieceIds)), [store, designId, pieceIds]);
  return usePiecesMoveOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#fix}. */
export function useFixPieces(designId: string, pieceIds: readonly string[]): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : new PiecesOperations(store, designId, pieceIds)), [store, designId, pieceIds]);
  return usePiecesFixOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#changeBlueprint}. */
export function useChangePiecesBlueprint(designId: string, pieceIds: readonly string[]): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const store = useStore();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : new PiecesOperations(store, designId, pieceIds)), [store, designId, pieceIds]);
  return usePiecesChangeBlueprintOperation(getOps);
}
// #endregion 🪢️Pieces

// #region ⛓️Connection
/** @emoji 📖️ Live {@link Connection#gap}. */
export function useConnectionGap(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.gap());
}

/** @emoji 📖️ Live {@link Connection#shift}. */
export function useConnectionShift(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.shift());
}

/** @emoji 📖️ Live {@link Connection#rise}. */
export function useConnectionRise(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.rise());
}

/** @emoji 📖️ Live {@link Connection#rotation}. */
export function useConnectionRotation(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.rotation());
}

/** @emoji 📖️ Live {@link Connection#turn}. */
export function useConnectionTurn(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.turn());
}

/** @emoji 📖️ Live {@link Connection#tilt}. */
export function useConnectionTilt(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.tilt());
}

/** @emoji 📖️ Live {@link Connection#u}. */
export function useConnectionU(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.u());
}

/** @emoji 📖️ Live {@link Connection#v}. */
export function useConnectionV(): FieldReadState<number | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.v());
}

/** @emoji 📖️ Live {@link Connection#connected}. */
export function useConnectionConnected(): FieldReadState<ConnectionSide | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.connected());
}

/** @emoji 📖️ Live {@link Connection#connecting}. */
export function useConnectionConnecting(): FieldReadState<ConnectionSide | null> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.connecting());
}

/** @emoji 📖️ Live {@link Connection#name}. */
export function useConnectionName(): FieldReadState<string> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖️ Live {@link Connection#description}. */
export function useConnectionDescription(): FieldReadState<string> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖️ Live {@link Connection#icon}. */
export function useConnectionIcon(): FieldReadState<string> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖️ Live {@link Connection#attributes}. */
export function useConnectionAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConnection();
  return useCurrentEntityField(entity, (c) => c.attributes());
}
// #endregion ⛓️Connection

// #region ⚛️Embedded tests
// @emoji 🧹️ Legacy InMemoryKitStore embedded block removed during single-source Kit migration; restore with GraphQL Kit stubs only.
// #endregion ⚛️Embedded tests

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
  describe("compose/react kit binders", () => {
    it("mapTooLong surfaces max length for NameTooLong", () => {
      const msg = mapTooLong({ kind: "NameTooLong", message: "ignored", field: "name" }, 42);
      expect(msg).toContain("42");
    });
  });
  describe("schema-1:1 banned patterns (this file)", () => {
    const mustNotMatchCode = [
      /\buseSyncExternalStore\s*\(/,
      /\bapplyKitDiff\s*\(/,
      /\buseDesignAppCommands\s*\(/,
      /\bKitStoreSnapshot\b/,
      /\bapplyToCache\s*\(/,
      /\bdispatchSync\s*\(/,
      /\bfieldSync\b/,
      /\boptimistic\b/,
      /\breconcil/i,
      /\buseKitScope\s*\(/,
      /\bKitScope\b/,
      /\bKitShellScopeProvider\b/,
    ];
    it("react index has no banned substrings as live code calls", () => {
      for (const re of mustNotMatchCode) {
        expect.soft(reactSrcForBannedScan.match(re)).toBeNull();
      }
    });
  });
}
// #endregion 🧪️Vitest
