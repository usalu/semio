// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link } reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
// Value/type re-exports follow the local `@semio/js` imports below (single binding per symbol).
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type { ReactNode } from "react";
import * as React from "react";
import type { Attribute, Benchmark, Coordinate, Entity, FieldSpec, GraphRootKind, OffsetInput, PieceBlueprint, Plane, Position, PositionInput, SetError, SetResult } from "../js";
import {
  Alternative,
  Author,
  Backbone,
  Concept,
  Connection,
  Connector,
  defineField,
  Design,
  File,
  Graph,
  Kit,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  LocalProvider,
  Piece,
  PiecesOperation,
  Port,
  Quality,
  RemoteProvider,
  Representation,
  Session,
  Side,
  Store,
  Tag,
  TheKit,
  Type,
} from "../js";
// #endregion ⚛️Imports

// #region 🪝FieldBind
/** @emoji 📖 One materialized field read: value, loading, error, and manual refresh (no sync external store). */
export type FieldReadState<T> = Readonly<{
  value: T | undefined;
  loading: boolean;
  error: unknown;
  refresh: () => Promise<void>;
}>;

export type FieldBindOptions<E, T> = Readonly<{
  /** @emoji 🧲 Single async read (one GraphQL selection / entity method). */
  read: (entity: E) => Promise<T>;
  /** @emoji 📡 When set, {@link Store#bus} {@code subscribeKind}; when omitted, only mount + {@link FieldReadState#refresh} pull fresh data. */
  eventKind?: string;
  /** @emoji 🪝  source; re-invoked each render — keep stable via {@link React#useCallback}. */
  get: () => E | null;
}>;

/**
 * @emoji 🪝 Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam T — Parsed field value.
 */
function semioInternalFieldBind<E extends Entity, T>(opts: FieldBindOptions<E, T>): () => FieldReadState<T> {
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
    }, [refresh, entity?.id, entity?.storeId]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      if (eventKind == null || eventKind === "") return undefined;
      return e.session.bus.subscribeKind(eventKind, () => void refresh());
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
 * @emoji 🪝 Same as {@link semioInternalFieldBind} but connects {@link defineField} so callers share {@link FieldSpec} with tooling/docs.
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam T — Parsed field value.
 */
function semioInternalDefinedFieldBind<E extends Entity, T>(opts: DefinedFieldBindOptions<E, T>): () => FieldReadState<T> {
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
    }, [refresh, entity?.id, entity?.storeId]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      if (eventKind == null || eventKind === "") return undefined;
      return e.session.bus.subscribeKind(eventKind, () => void refresh());
    }, [entity, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝FieldBind

// #region 🪝OperationBind
/** @emoji 🎛️ UI-facing operation lifecycle for {@link semioInternalOperationBind} (idle → pending → settled). */
export type OperationStatus = { readonly kind: "idle" } | { readonly kind: "pending" } | { readonly kind: "settled"; readonly result: SetResult };

/**
 * @emoji 🗺️ Maps {@link SetErrorKind#NameTooLong} to a fixed max-length message; otherwise returns {@link SetError#message}.
 * @param maxChars — Upper bound communicated to the user (schema limit or UI policy).
 */
function mapTooLong(err: SetError, maxChars: number): string {
  if (err.kind === "NameTooLong") return `Name must be at most ${maxChars} characters.`;
  return err.message;
}

/**
 * @emoji 🪝 Binds an entity operation to `[run, status]`; `run` reads latest entity via {@code get} ref (no sync external store).
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam Args — Operation arguments after the entity receiver.
 */
function semioInternalOperationBind<E extends Entity, Args extends unknown[] = []>(impl: (entity: E, ...args: Args) => Promise<SetResult>): (get: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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
// #endregion 🪝OperationBind

// #region 🪝KitFieldBind
/** @emoji 🪝 Kit-scoped field bind (uses {@link Store#bus} like {@link semioInternalFieldBind}). */
export type KitFieldBindOptions<T> = Readonly<{
  read: (kit: Kit) => Promise<T>;
  eventKind?: string;
  getKit: () => Kit | null;
}>;

function semioInternalKitFieldBind<T>(opts: KitFieldBindOptions<T>): () => FieldReadState<T> {
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
      if (eventKind != null && eventKind !== "") return k.session.bus.subscribeKind(eventKind, () => void refresh());
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

/** @emoji 🪝 Store-root field bind for session/backbone/root fields. */
function semioInternalStoreFieldBind<T>(opts: StoreFieldBindOptions<T>): () => FieldReadState<T> {
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
      if (eventKind != null && eventKind !== "") return s.session.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [store, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝KitFieldBind

// #region 🪝StoreOperationBind
/** @emoji 🪝 Binds a {@link Store} operation to `[run, status]`. */
function semioInternalStoreOpBind<Args extends unknown[] = []>(impl: (store: Store, ...args: Args) => Promise<SetResult>): (getStore: () => Store | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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

/** @emoji 🪝 Binds a {@link Session} operation to `[run, status]`. */
function semioInternalSessionOpBind<Args extends unknown[] = []>(impl: (session: Session, ...args: Args) => Promise<SetResult>): (getSession: () => Session | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useSessionOp(getSession: () => Session | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(getSession);
    getRef.current = getSession;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

    const run = React.useCallback(
      async (...args: Args) => {
        const s = getRef.current();
        if (s == null) {
          const result: SetResult = { ok: false, error: { kind: "Disposed", message: "No session in React context.", field: undefined, entity: undefined } };
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
  };
}

function useCurrentEntityField<E extends Entity, T>(entity: E | null, read: (entity: E) => Promise<T>, eventKind?: string): FieldReadState<T> {
  return semioInternalFieldBind<E, T>({ get: () => entity, read, eventKind })();
}

function useCurrentEntityOperation<E extends Entity, Args extends unknown[] = []>(
  entity: E | null,
  useBound: (get: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus],
): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return useBound(() => entity);
}
// #endregion 🪝StoreOperationBind

// #region 🪪IdsAndProviders
const SessionHandleContext = React.createContext<Session | null>(null);
const StoreHandleContext = React.createContext<Store | null>(null);
const GraphHandleContext = React.createContext<Graph | null>(null);
const TheKitHandleContext = React.createContext<TheKit | null>(null);
const AlternativeHandleContext = React.createContext<Alternative | null>(null);
const KitHandleContext = React.createContext<Kit | null>(null);
const LocalProviderHandleContext = React.createContext<LocalProvider | null>(null);
const RemoteProviderHandleContext = React.createContext<RemoteProvider | null>(null);
const BackboneHandleContext = React.createContext<Backbone | null>(null);

const StoreIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const AlternativeIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const KitIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const DesignIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const TypeIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const AuthorIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const QualityIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const TagIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const ConceptIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const PieceIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const ConnectionIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const PortIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const ConnectorIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const RepresentationIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const RemoteProviderUrlContext = React.createContext<Readonly<{ id: string }> | null>(null);
const FileBackboneIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const FolderBackboneIdContext = React.createContext<Readonly<{ id: string }> | null>(null);
const WebsocketBackboneIdContext = React.createContext<Readonly<{ id: string }> | null>(null);

const WipMarkerContext = React.createContext(false);
const StageMarkerContext = React.createContext(false);
const AuthoritativeMarkerContext = React.createContext(false);
const TheKitMarkerContext = React.createContext(false);

const PositionMarkerContext = React.createContext(false);
const FlatPositionMarkerContext = React.createContext(false);
const PlaneMarkerContext = React.createContext(false);
const OriginMarkerContext = React.createContext(false);

const PiecesBatchContext = React.createContext<Readonly<{ pieceIds: readonly string[] }> | null>(null);

/** @emoji 🪢 Batch piece ids for {@link useDragPieces} under {@link DesignIdContext}. */
export function PiecesBatchContextProvider(props: Readonly<{ pieceIds: readonly string[]; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ pieceIds: props.pieceIds }), [props.pieceIds]);
  return React.createElement(PiecesBatchContext.Provider, { value: v }, props.children);
}

/** @emoji 🗂️ Publishes the semio/js session handle (opaque to public hooks). */
export function SessionContextProvider(props: Readonly<{ session: unknown; children: ReactNode }>): React.ReactElement {
  return React.createElement(SessionHandleContext.Provider, { value: props.session as Session }, props.children);
}

export type StoreContextProviderProps = Readonly<{
  id: string;
  children: ReactNode;
}>;

/** @emoji 🏪 Resolves {@link Store} from {@link SessionContextProvider} + id. */
export function StoreContextProvider(props: StoreContextProviderProps): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  if (session == null) throw new Error("semio/react: StoreContextProvider requires SessionContextProvider.");
  const store = React.useMemo(() => session.store(props.id), [session, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(StoreHandleContext.Provider, { value: store }, React.createElement(StoreIdContext.Provider, { value: idRow }, props.children));
}

export function LocalProviderContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  if (session == null) throw new Error("semio/react: LocalProviderContextProvider requires SessionContextProvider.");
  const lp = React.useMemo(() => session.localProvider(), [session]);
  return React.createElement(LocalProviderHandleContext.Provider, { value: lp }, props.children);
}

export function RemoteProviderContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  if (session == null) throw new Error("semio/react: RemoteProviderContextProvider requires SessionContextProvider.");
  const rp = React.useMemo(() => session.remoteProvider(props.id), [session, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(RemoteProviderHandleContext.Provider, { value: rp }, React.createElement(RemoteProviderUrlContext.Provider, { value: idRow }, props.children));
}

export function WipContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = React.useContext(StoreHandleContext);
  if (store == null) throw new Error("semio/react: WipContextProvider requires StoreContextProvider.");
  const graph = React.useMemo(() => store.wip(), [store]);
  return React.createElement(WipMarkerContext.Provider, { value: true }, React.createElement(GraphHandleContext.Provider, { value: graph }, props.children));
}

export function StageContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = React.useContext(StoreHandleContext);
  if (store == null) throw new Error("semio/react: StageContextProvider requires StoreContextProvider.");
  const graph = React.useMemo(() => store.stage(), [store]);
  return React.createElement(StageMarkerContext.Provider, { value: true }, React.createElement(GraphHandleContext.Provider, { value: graph }, props.children));
}

export function AuthoritativeContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const store = React.useContext(StoreHandleContext);
  if (store == null) throw new Error("semio/react: AuthoritativeContextProvider requires StoreContextProvider.");
  const graph = React.useMemo(() => store.authoritative(), [store]);
  return React.createElement(AuthoritativeMarkerContext.Provider, { value: true }, React.createElement(GraphHandleContext.Provider, { value: graph }, props.children));
}

export function TheKitContextProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const graph = React.useContext(GraphHandleContext);
  if (graph == null) throw new Error("semio/react: TheKitContextProvider requires a graph tier provider.");
  const tk = React.useMemo(() => graph.theKit(), [graph]);
  return React.createElement(TheKitMarkerContext.Provider, { value: true }, React.createElement(TheKitHandleContext.Provider, { value: tk }, props.children));
}

export function AlternativeContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const graph = React.useContext(GraphHandleContext);
  if (graph == null) throw new Error("semio/react: AlternativeContextProvider requires a graph tier provider.");
  const alt = React.useMemo(() => graph.alternative(props.id), [graph, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(AlternativeHandleContext.Provider, { value: alt }, React.createElement(AlternativeIdContext.Provider, { value: idRow }, props.children));
}

export function KitContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const store = React.useContext(StoreHandleContext);
  if (store == null) throw new Error("semio/react: KitContextProvider requires StoreContextProvider.");
  const kit = React.useMemo(() => new Kit(store.session, props.id, store.id), [store.session, store.id, props.id]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(KitHandleContext.Provider, { value: kit }, React.createElement(KitIdContext.Provider, { value: idRow }, props.children));
}

function mkIdProvider(C: React.Context<Readonly<{ id: string }> | null>) {
  return function IdCtxProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
    const v = React.useMemo(() => ({ id: props.id }), [props.id]);
    return React.createElement(C.Provider, { value: v }, props.children);
  };
}

export const DesignContextProvider = mkIdProvider(DesignIdContext);
export const TypeContextProvider = mkIdProvider(TypeIdContext);
export const AuthorContextProvider = mkIdProvider(AuthorIdContext);
export const QualityContextProvider = mkIdProvider(QualityIdContext);
export const TagContextProvider = mkIdProvider(TagIdContext);
export const ConceptContextProvider = mkIdProvider(ConceptIdContext);
export const PieceContextProvider = mkIdProvider(PieceIdContext);
export const ConnectionContextProvider = mkIdProvider(ConnectionIdContext);
export const PortContextProvider = mkIdProvider(PortIdContext);
export const ConnectorContextProvider = mkIdProvider(ConnectorIdContext);
export const RepresentationContextProvider = mkIdProvider(RepresentationIdContext);

export function FileBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const lp = React.useContext(LocalProviderHandleContext);
  if (session == null || lp == null) throw new Error("semio/react: FileBackboneContextProvider requires Session + LocalProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, lp), [session, props.id, lp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(FileBackboneIdContext.Provider, { value: idRow }, props.children));
}

export function FolderBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const lp = React.useContext(LocalProviderHandleContext);
  if (session == null || lp == null) throw new Error("semio/react: FolderBackboneContextProvider requires Session + LocalProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, lp), [session, props.id, lp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(FolderBackboneIdContext.Provider, { value: idRow }, props.children));
}

export function WebsocketBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const rp = React.useContext(RemoteProviderHandleContext);
  if (session == null || rp == null) throw new Error("semio/react: WebsocketBackboneContextProvider requires Session + RemoteProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, rp), [session, props.id, rp]);
  const idRow = React.useMemo(() => ({ id: props.id }), [props.id]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(WebsocketBackboneIdContext.Provider, { value: idRow }, props.children));
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

function useJsStore(): Store {
  const s = React.useContext(StoreHandleContext);
  if (s == null) throw new Error("semio/react: useJsStore requires StoreContextProvider id={…}.");
  return s;
}

function useJsSession(): Session {
  const s = React.useContext(SessionHandleContext);
  if (s == null) throw new Error("semio/react: useJsSession requires SessionContextProvider.");
  return s;
}

function readOptionalId(ctx: React.Context<Readonly<{ id: string }> | null>, override?: string): string | null {
  const row = React.useContext(ctx);
  const id = override ?? row?.id ?? null;
  return id == null || id === "" ? null : id;
}

function resolveKit(id?: string): Kit | null {
  const k = React.useContext(KitHandleContext);
  const rid = readOptionalId(KitIdContext, id);
  if (k == null || rid == null) return null;
  return k.id === rid ? k : new Kit(k.session, rid, k.storeId);
}

function resolveDesign(designId?: string): Design | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(DesignIdContext, designId);
  if (st == null || id == null) return null;
  return st.design(id);
}

function resolvePiece(pieceId?: string): Piece | null {
  const d = resolveDesign();
  const id = readOptionalId(PieceIdContext, pieceId);
  if (d == null || id == null) return null;
  return d.piece(id);
}

function resolveType(typeId?: string): Type | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(TypeIdContext, typeId);
  if (st == null || id == null) return null;
  return st.type(id);
}

function resolveConnection(connectionId?: string): Connection | null {
  const d = resolveDesign();
  const id = readOptionalId(ConnectionIdContext, connectionId);
  if (d == null || id == null) return null;
  return d.connection(id);
}

function resolvePort(portId?: string): Port | null {
  const t = resolveType();
  const id = readOptionalId(PortIdContext, portId);
  if (t == null || id == null) return null;
  return t.port(id);
}

function resolveConnector(connectorId?: string): Connector | null {
  const t = resolveType();
  const id = readOptionalId(ConnectorIdContext, connectorId);
  if (t == null || id == null) return null;
  return t.connector(id);
}

function resolveRepresentation(representationId?: string): Representation | null {
  const t = resolveType();
  const id = readOptionalId(RepresentationIdContext, representationId);
  if (t == null || id == null) return null;
  return t.representation(id);
}

function resolveQuality(qualityId?: string): Quality | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(QualityIdContext, qualityId);
  if (st == null || id == null) return null;
  return st.quality(id);
}

function resolveTag(tagId?: string): Tag | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(TagIdContext, tagId);
  if (st == null || id == null) return null;
  return st.tag(id);
}

function resolveConcept(conceptId?: string): Concept | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(ConceptIdContext, conceptId);
  if (st == null || id == null) return null;
  return st.concept(id);
}

function resolveAuthor(authorId?: string): Author | null {
  const st = React.useContext(StoreHandleContext);
  const id = readOptionalId(AuthorIdContext, authorId);
  if (st == null || id == null) return null;
  return st.author(id);
}

function resolveAlternative(alternativeId?: string): Alternative | null {
  const g = React.useContext(GraphHandleContext);
  const row = React.useContext(AlternativeIdContext);
  const id = alternativeId ?? row?.id ?? null;
  if (g == null || id == null) return null;
  return g.alternative(id);
}

function resolveLocalProvider(): LocalProvider | null {
  return React.useContext(LocalProviderHandleContext);
}

function resolveRemoteProvider(url?: string): RemoteProvider | null {
  const rp = React.useContext(RemoteProviderHandleContext);
  const row = React.useContext(RemoteProviderUrlContext);
  const id = url ?? row?.id ?? null;
  if (rp != null && (url == null || rp.url === url)) return rp;
  if (id == null) return null;
  return useJsSession().remoteProvider(id);
}

function resolveBackbone(backboneId?: string): Backbone | null {
  const bb = React.useContext(BackboneHandleContext);
  const id = backboneId ?? React.useContext(FileBackboneIdContext)?.id ?? React.useContext(FolderBackboneIdContext)?.id ?? React.useContext(WebsocketBackboneIdContext)?.id ?? null;
  if (bb != null && (backboneId == null || bb.id === backboneId)) return bb;
  return null;
}

/** @emoji 🪪 Entity hooks return {@link FieldReadState} of plain {@code { id }}. */
export type EntityReadState = FieldReadState<Readonly<{ id: string }>>;

/** @emoji 📇 Plain list row for id-stable lists. */
export type IdRow = Readonly<{ id: string }>;

function entityRead(get: () => Entity | null): EntityReadState {
  return semioInternalFieldBind({
    get,
    read: async (e) => ({ id: e.id }),
    eventKind: KIT_EVENT_STREAM_SUBSCRIPTION,
  })() as EntityReadState;
}

export function useSession(): EntityReadState {
  const session = React.useContext(SessionHandleContext);
  const refresh = React.useCallback(async () => {}, []);
  return { value: session ? { id: "__session__" } : undefined, loading: false, error: undefined, refresh };
}

export function useStore(id?: string): EntityReadState {
  const store = React.useContext(StoreHandleContext);
  const row = React.useContext(StoreIdContext);
  const resolved = id ?? row?.id ?? null;
  return entityRead(() => (store != null && resolved != null && store.id === resolved ? store : null));
}

export function useWip(): EntityReadState {
  const on = React.useContext(WipMarkerContext);
  const refresh = React.useCallback(async () => {}, []);
  return { value: on ? { id: "wip" } : undefined, loading: false, error: undefined, refresh };
}
export function useStage(): EntityReadState {
  const on = React.useContext(StageMarkerContext);
  const refresh = React.useCallback(async () => {}, []);
  return { value: on ? { id: "stage" } : undefined, loading: false, error: undefined, refresh };
}
export function useAuthoritative(): EntityReadState {
  const on = React.useContext(AuthoritativeMarkerContext);
  const refresh = React.useCallback(async () => {}, []);
  return { value: on ? { id: "authoritative" } : undefined, loading: false, error: undefined, refresh };
}

export function useTheKit(): EntityReadState {
  const on = React.useContext(TheKitMarkerContext);
  const tk = React.useContext(TheKitHandleContext);
  return entityRead(() => (on && tk ? (tk as unknown as Entity) : null));
}

export function useAlternative(id?: string): EntityReadState {
  return entityRead(() => resolveAlternative(id) as unknown as Entity | null);
}
export function useKit(id?: string): EntityReadState {
  return entityRead(() => resolveKit(id) as unknown as Entity | null);
}
export function useDesign(id?: string): EntityReadState {
  return entityRead(() => resolveDesign(id) as unknown as Entity | null);
}
export function useType(id?: string): EntityReadState {
  return entityRead(() => resolveType(id) as unknown as Entity | null);
}
export function useAuthor(id?: string): EntityReadState {
  return entityRead(() => resolveAuthor(id) as unknown as Entity | null);
}
export function useQuality(id?: string): EntityReadState {
  return entityRead(() => resolveQuality(id) as unknown as Entity | null);
}
export function useTag(id?: string): EntityReadState {
  return entityRead(() => resolveTag(id) as unknown as Entity | null);
}
export function useConcept(id?: string): EntityReadState {
  return entityRead(() => resolveConcept(id) as unknown as Entity | null);
}
export function usePiece(id?: string): EntityReadState {
  return entityRead(() => resolvePiece(id) as unknown as Entity | null);
}
export function useConnection(id?: string): EntityReadState {
  return entityRead(() => resolveConnection(id) as unknown as Entity | null);
}
export function usePort(id?: string): EntityReadState {
  return entityRead(() => resolvePort(id) as unknown as Entity | null);
}
export function useConnector(id?: string): EntityReadState {
  return entityRead(() => resolveConnector(id) as unknown as Entity | null);
}
export function useRepresentation(id?: string): EntityReadState {
  return entityRead(() => resolveRepresentation(id) as unknown as Entity | null);
}
export function useLocalProvider(): EntityReadState {
  return entityRead(() => resolveLocalProvider() as unknown as Entity | null);
}
export function useRemoteProvider(id?: string): EntityReadState {
  return entityRead(() => resolveRemoteProvider(id) as unknown as Entity | null);
}
export function useFileBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id) as unknown as Entity | null);
}
export function useFolderBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id) as unknown as Entity | null);
}
export function useWebsocketBackbone(id?: string): EntityReadState {
  return entityRead(() => resolveBackbone(id) as unknown as Entity | null);
}

/** @emoji 🧭 Optional {@link Store} handle for legacy call sites inside this module. */
export function useStoreOptional(): Store | null {
  return React.useContext(StoreHandleContext);
}

/** @emoji 🌐 WIP {@link Graph} from the active graph tier. */
export function useWipGraph(): Graph {
  const g = React.useContext(GraphHandleContext);
  if (g == null) throw new Error("semio/react: useWipGraph requires WipContextProvider.");
  return g;
}

/** @emoji 🏛 {@link TheKit} under the active graph tier. */
export function useWipVersion(): TheKit {
  const tk = React.useContext(TheKitHandleContext);
  if (tk == null) throw new Error("semio/react: useWipVersion requires TheKitContextProvider.");
  return tk;
}

/** @emoji 📦 {@link Kit} from {@link KitContextProvider}. */
export function useWipKit(): Kit {
  const k = React.useContext(KitHandleContext);
  if (k == null) throw new Error("semio/react: useWipKit requires KitContextProvider.");
  return k;
}

/** @emoji 🌐 Authoritative graph (bypasses tier marker; prefer {@link AuthoritativeContextProvider}). */
export function useAuthoritativeGraph(): Graph {
  return useJsStore().authoritative();
}

/** @emoji 🌐 {@link Graph} for {@link GraphContextProvider} (compat). */
export function useGraph(): Graph {
  const g = React.useContext(GraphHandleContext);
  if (g != null) return g;
  const store = useJsStore();
  const ctx = React.useContext(GraphRootContext);
  if (ctx == null) throw new Error("semio/react: useGraph requires GraphContextProvider or graph tier.");
  return React.useMemo(() => (ctx.root === "authoritative" ? store.authoritative() : store.wip()), [store, ctx.root]);
}

// #region 📐Design
/** @emoji 📐 Legacy design id context (compat: value uses {@code designId}). */
export type DesignContext = Readonly<{ designId: string }>;
const DesignContext = React.createContext<DesignContext | null>(null);
/** @emoji 📐 Prefer {@link DesignContextProvider} from id-only kit; this accepts {@code id} and mirrors {@code designId}. */
export function LegacyDesignContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ designId: props.id }), [props.id]);
  return React.createElement(DesignContext.Provider, { value: v }, React.createElement(DesignIdContext.Provider, { value: { id: props.id } }, props.children));
}
/** @emoji 📐 {@link Design} from legacy {@link DesignContext} or id-only {@link DesignIdContext}. */
export function useDesignJs(): Design | null {
  const store = useJsStore();
  const legacy = React.useContext(DesignContext);
  const id = legacy?.designId ?? React.useContext(DesignIdContext)?.id ?? null;
  return id == null ? null : store.design(id);
}
// #endregion 📐Design

// #endregion 🪪IdsAndProviders

// #region 🌐GraphContext
export type GraphContextValue = Readonly<{ root: GraphRootKind }>;

const GraphRootContext = React.createContext<GraphContextValue | null>(null);

/** @emoji 🌐 Binds {@link GraphRootKind} for {@link useGraph}. */
export function GraphContextProvider(props: Readonly<{ root: GraphRootKind; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo<GraphContextValue>(() => ({ root: props.root }), [props.root]);
  return React.createElement(GraphRootContext.Provider, { value: v }, props.children);
}

// #endregion 🌐GraphContext

// #endregion 🎭Contexts

// #region 🔖EntityContextHelpers
/** @emoji 🧭 `{ id }` from {@link DesignIdContext} or legacy {@link DesignContext}. */
export function useDesignContext(): Readonly<{ id: string }> | null {
  const idOnly = React.useContext(DesignIdContext);
  if (idOnly != null) return idOnly;
  const legacy = React.useContext(DesignContext);
  return legacy == null ? null : { id: legacy.designId };
}

/** @emoji 🧭 True when a design scope is mounted. */
export function useHasDesignContext(): boolean {
  return React.useContext(DesignIdContext) != null || React.useContext(DesignContext) != null;
}

/** @emoji 🧭 `{ id }` from {@link PieceIdContext}. */
export function usePieceContext(): Readonly<{ id: string }> | null {
  return React.useContext(PieceIdContext);
}

/** @emoji 🧭 True when {@link PieceContextProvider} is mounted above. */
export function useHasPieceContext(): boolean {
  return React.useContext(PieceIdContext) != null;
}

/** @emoji 🧭 `{ id }` from {@link ConnectionIdContext}. */
export function useConnectionContext(): Readonly<{ id: string }> | null {
  return React.useContext(ConnectionIdContext);
}

/** @emoji 🧭 True when {@link ConnectionContextProvider} is mounted above. */
export function useHasConnectionContext(): boolean {
  return React.useContext(ConnectionIdContext) != null;
}

/** @emoji 🧭 `{ id }` from {@link TypeIdContext}. */
export function useTypeContext(): Readonly<{ id: string }> | null {
  return React.useContext(TypeIdContext);
}

/** @emoji 🧭 True when {@link TypeContextProvider} is mounted above. */
export function useHasTypeContext(): boolean {
  return React.useContext(TypeIdContext) != null;
}

/** @emoji 🧭 `{ id }` from {@link QualityIdContext}. */
export function useQualityContext(): Readonly<{ id: string }> | null {
  return React.useContext(QualityIdContext);
}

/** @emoji 🧭 True when {@link QualityContextProvider} is mounted above. */
export function useHasQualityContext(): boolean {
  return React.useContext(QualityIdContext) != null;
}

/** @emoji 🧭 `{ id }` from {@link AuthorIdContext}. */
export function useAuthorContext(): Readonly<{ id: string }> | null {
  return React.useContext(AuthorIdContext);
}

/** @emoji 🧭 True when {@link AuthorContextProvider} is mounted above. */
export function useHasAuthorContext(): boolean {
  return React.useContext(AuthorIdContext) != null;
}

/** @emoji 🧷 {@link PieceContextProvider} using enclosing design scope. */
export function PieceUnderActiveDesignProvider(props: Readonly<{ pieceId: string; children: ReactNode }>): React.ReactElement {
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId;
  if (designId == null) {
    throw new Error("semio/react: PieceUnderActiveDesignProvider requires DesignContextProvider / DesignIdContext.");
  }
  return React.createElement(PieceIdContext.Provider, { value: { id: props.pieceId } }, props.children);
}

/** @emoji 🧷 {@link ConnectionContextProvider} using enclosing design scope. */
export function ConnectionUnderActiveDesignProvider(props: Readonly<{ connectionId: string; children: ReactNode }>): React.ReactElement {
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId;
  if (designId == null) {
    throw new Error("semio/react: ConnectionUnderActiveDesignProvider requires DesignContextProvider / DesignIdContext.");
  }
  return React.createElement(ConnectionIdContext.Provider, { value: { id: props.connectionId } }, props.children);
}
// #endregion 🔖EntityContextHelpers

function useResolvedDesign(designId?: string): Design | null {
  return resolveDesign(designId);
}

function useResolvedType(typeId?: string): Type | null {
  return resolveType(typeId);
}

// #region 🪝IdStableEntityLists
/** @emoji 📚 Kit-level designs via {@link Kit#readDesigns} (id-list-stable handles). */
export function useKitDesigns(): FieldReadState<readonly Design[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.designs());
}

/** @emoji 📚 Kit-level kinds via {@link Kit#readTypes}. */
export function useKitTypes(): FieldReadState<readonly Type[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.types());
}

/** @emoji 📚 Kit-level authors via {@link Kit#readAuthors}. */
export function useKitAuthors(): FieldReadState<readonly Author[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.authors());
}

/** @emoji 📚 Kit-level qualities via {@link Kit#readQualities}. */
export function useKitQualities(): FieldReadState<readonly Quality[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.qualities());
}

/** @emoji 📚 Kit-level tags via {@link Kit#readTags}. */
export function useKitTags(): FieldReadState<readonly Tag[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.tags());
}

/** @emoji 📚 Kit-level concepts via {@link Kit#readConcepts}. */
export function useKitConcepts(): FieldReadState<readonly Concept[]> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.concepts());
}

/** @emoji 📚 Design pieces ordered by {@link Design#readPieceIds} (stable {@link Piece} handles per id). */
export function useDesignPieces(): FieldReadState<readonly Piece[]> {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, (design) => design.pieces());
}

/** @emoji 📚 Design connections ordered by {@link Design#readConnectionIds}. */
export function useDesignConnections(): FieldReadState<readonly Connection[]> {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, (design) => design.connections());
}
// #endregion 🪝IdStableEntityLists

// #region 🪝AggregateListBundles
/** @emoji 📚 Sketchpad bundle: live kit designs (id-list-stable). */
export function useDesigns(): Readonly<{ designs: readonly Design[] }> {
  const { value } = useKitDesigns();
  return { designs: value ?? [] };
}

/** @emoji 📚 Sketchpad bundle: live kit kinds (id-list-stable). */
export function useTypes(): Readonly<{ types: readonly Type[] }> {
  const { value } = useKitTypes();
  return { types: value ?? [] };
}

/** @emoji 📚 Pieces in the active {@link DesignContextProvider} design (see {@link DesignContext}). */
export function usePieces(): readonly Piece[] {
  const { value } = useDesignPieces();
  return value ?? [];
}
// #endregion 🪝AggregateListBundles

// #region 🪝EntityContextReads
/** @emoji 📖 Resolves a kit {@link Piece} from context and/or ids, then applies {@code selector}. */
export function usePieceContextRead<T>(selector: (p: Piece) => T, pieceId?: string | undefined, _deep?: boolean): T {
  const p = resolvePiece(pieceId);
  if (p == null) {
    throw new Error("semio/react: usePieceContextRead requires PieceContextProvider or pieceId + Design scope.");
  }
  return selector(p);
}

/** @emoji 📖 Resolves a kit {@link Type} handle from context and/or id, then applies {@code selector}. */
export function useTypeContextRead<S>(selector: ((t: Type) => S) | undefined, kindId?: string | undefined, deep?: boolean): S | Type | undefined {
  const fromCtx = resolveType();
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

/** @emoji 📖 Resolves a kit {@link Quality} from context and/or id, then applies {@code selector}. */
export function useQualityContextRead<S>(selector: ((q: Quality) => S) | undefined, qualityId?: string | undefined, deep?: boolean): S | Quality | undefined {
  const fromCtx = resolveQuality();
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
// #endregion 🪝EntityContextReads

// #region 🪝HooksKit
// #region 📖KitReads
/** @emoji 📖 Live {@link Kit#name} + {@code kitRenamed}. */
export function useKitName(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.name(), "kitRenamed");
}

/** @emoji 📖 Live {@link Kit#description} + {@code changedDescription}. */
export function useKitDescription(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.description(), "changedDescription");
}

/** @emoji 📖 Live {@link Kit#id}. */
export function useKitId(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => k.id);
}

/** @emoji 📖 Live {@link Kit#icon}. */
export function useKitIcon(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.icon());
}

/** @emoji 📖 Live {@link Kit#image}. */
export function useKitImage(): FieldReadState<string> {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.image());
}

/** @emoji 🧾 Exposes {@link Store#ensureChangeId} as a stable callback. */
export function useEnsureKitChangeId(): () => Promise<string> {
  const store = useJsStore();
  return React.useCallback(() => store.ensureChangeId(), [store]);
}
// #endregion 📖KitReads

// #region ✍️KitWrites
/** @emoji ✍️ {@link Kit#rename}. */
export function useRenameKit(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, newName) => k.rename(newName))(() => kit);
}

/** @emoji ✍️ {@link Kit#changeDescription}. */
export function useChangeKitDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, d) => k.changeDescription(d))(() => kit);
}

/** @emoji ✍️ {@link Kit#createTag}. */
export function useCreateTag(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createTag(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTag}. */
export function useDeleteTag(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteTag(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTags}. */
export function useDeleteTags(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteTags(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createConcept}. */
export function useCreateConcept(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createConcept(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcept}. */
export function useDeleteConcept(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteConcept(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcepts}. */
export function useDeleteConcepts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteConcepts(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createQuality}. */
export function useCreateQuality(): readonly [(key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, key, value, unit, definition, description, icon) =>
    k.createQuality(key, value, unit, definition, description, icon),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQuality}. */
export function useDeleteQuality(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteQuality(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQualities}. */
export function useDeleteQualities(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteQualities(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createType}. */
export function useCreateType(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createType(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteType}. */
export function useDeleteType(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteType(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTypes}. */
export function useDeleteTypes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteTypes(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createDesign}. */
export function useCreateDesign(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createDesign(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesign}. */
export function useDeleteDesign(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteDesign(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesigns}. */
export function useDeleteDesigns(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteDesigns(ids))(() => kit);
}

/** @emoji ✍️ {@link Store#saveChange}. */
export function useSaveKitChange(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[]>(async (k) => {
    await k.saveChange();
    return { ok: true };
  })(() => store);
}

/** @emoji ✍️ {@link Store#createCheckpoint}. */
export function useCreateCheckpoint(): readonly [(message: string) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[string]>((k, message) => k.createCheckpoint(message))(() => store);
}

/** @emoji ✍️ {@link Store#startAlternative}. */
export function useStartAlternative(): readonly [(name?: string | null) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[string | null | undefined]>((k, name) => k.startAlternative(name ?? undefined))(() => store);
}

/** @emoji ✍️ {@link Store#integrateAlternative}. */
export function useIntegrateAlternative(): readonly [(alternativeId: string) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[string]>((k, id) => k.integrateAlternative(id))(() => store);
}

/** @emoji ✍️ {@link Session#login}. */
export function useLogin(): readonly [(username: string, passwordHash: string, hubUrl?: string) => Promise<SetResult>, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOpBind<[string, string, string | undefined]>((s, u, p, h) => s.login(u, p, h))(() => session);
}

/** @emoji ✍️ {@link Session#logout}. */
export function useLogout(): readonly [() => Promise<SetResult>, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOpBind<[]>((s) => s.logout())(() => session);
}

/** @emoji ✍️ {@link Session#sessionStart}. */
export function useStartSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOpBind<[]>((s) => s.sessionStart())(() => session);
}

/** @emoji ✍️ {@link Session#sessionEnd}. */
export function useEndSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOpBind<[]>((s) => s.sessionEnd())(() => session);
}

// #region 🪝BackboneOps
/** @emoji 🛜 {@link Store#attachBackbone} — attaches via {@link LocalProvider} for the active session. */
export function useAttachBackbone(): readonly [(uri: string) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  const session = useJsSession();
  return semioInternalStoreOpBind<[string]>((s, uri) => s.attachBackbone(session.localProvider(), uri))(() => store);
}

/** @emoji 🛜 {@link Store#detachBackbone}. */
export function useDetachBackbone(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[]>((s) => s.detachBackbone())(() => store);
}

/** @emoji 🛜 {@link Store#syncBackbone}. */
export function useBackboneSyncNow(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOpBind<[]>((s) => s.syncBackbone())(() => store);
}

/** @emoji 🛜 Live {@link Session#backboneStatus} for the active store (refreshes on {@code commandSucceeded} bus events). */
export function useBackboneStatus(): FieldReadState<Readonly<{ attachedUri: string | null; kind: string }>> {
  const store = useJsStore();
  return semioInternalStoreFieldBind<Readonly<{ attachedUri: string | null; kind: string }>>({
    getStore: () => store,
    read: (s) => s.session.backboneStatus(s.id),
    eventKind: "commandSucceeded",
  })();
}
// #endregion 🪝BackboneOps

// #endregion 🪝HooksKit

// #region 🪝HooksDesign
// #region 📖DesignReads
/** @emoji 📖 Live {@link Design#name}. */
export function useDesignName(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.name());
}

/** @emoji 📖 Live {@link Design#description} + {@code changedDescription}. */
export function useDesignDescription(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.description(), "changedDescription");
}

/** @emoji 📖 Live {@link Design#qualitySum}. */
export function useDesignQualitySum(designId?: string): FieldReadState<number> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.qualitySum());
}

/** @emoji 📖 Live {@link Design#icon}. */
export function useDesignIcon(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.icon());
}

/** @emoji 📖 Live {@link Design#image}. */
export function useDesignImage(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.image());
}

/** @emoji 📖 Live {@link Design#unit}. */
export function useDesignUnit(designId?: string): FieldReadState<string> {
  const entity = useResolvedDesign(designId);
  return useCurrentEntityField(entity, (d) => d.unit());
}
// #endregion 📖DesignReads

// #region ✍️DesignWrites
/** @emoji ✍️ {@link Design#rename}. */
export function useRenameDesign(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string]>((d, n) => d.rename(n))(() => entity);
}

/** @emoji ✍️ {@link Design#changeDescription}. */
export function useChangeDesignDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string]>((d, t) => d.changeDescription(t))(() => entity);
}

/** @emoji ✍️ {@link Design#flatten}. */
export function useFlattenDesign(): readonly [() => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, []>((d) => d.flatten())(() => entity);
}

/** @emoji ✍️ {@link Design#addAttribute}. */
export function useAddDesignAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string, string, string]>((d, k, v, def) => d.addAttribute(k, v, def))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttribute}. */
export function useRemoveDesignAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string]>((d, id) => d.removeAttribute(id))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttributes}. */
export function useRemoveDesignAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [readonly string[]]>((d, ids) => d.removeAttributes(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#addFixedPiece}. */
export function useAddFixedPiece(): readonly [(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string, PositionInput, string | null | undefined, string | null | undefined]>((d, bp, pos, n, desc) => d.addFixedPiece(bp, pos, n, desc))(() => entity);
}

/** @emoji ✍️ {@link Design#addChildPieceWithParentConnection}. */
export function useAddChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, name?: string | null, description?: string | null, position?: PositionInput | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string, string, string, string, string | null | undefined, string | null | undefined, PositionInput | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, n, desc, pos, sc) =>
    d.addChildPieceWithParentConnection(bp, pp, pc, cc, n, desc, pos, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#addHangingChildPieceWithParentConnection}. */
export function useAddHangingChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, position: PositionInput, name?: string | null, description?: string | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string, string, string, string, PositionInput, string | null | undefined, string | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, pos, n, desc, sc) =>
    d.addHangingChildPieceWithParentConnection(bp, pp, pc, cc, pos, n, desc, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiece}. */
export function useDeleteDesignPiece(): readonly [(pieceId: string) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [string]>((d, id) => d.deletePiece(id))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePieces}. */
export function useDeleteDesignPieces(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [readonly string[]]>((d, ids) => d.deletePieces(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiecesAndConnections}. */
export function useDeleteDesignPiecesAndConnections(): readonly [(pieceIds: readonly string[], connectionIds: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = resolveDesign();
  return semioInternalOperationBind<Design, [readonly string[], readonly string[]]>((d, p, c) => d.deletePiecesAndConnections(p, c))(() => entity);
}

/**
 * @emoji ✍️ Legacy shallow {@link Design} patch runner: applies only keys backed by {@link DesignOperation}; other keys are ignored with a {@code [DEBUG]} console warning.
 */
export function useUpdateDesign(): Readonly<{
  run: (designId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: OperationStatus;
}> {
  const store = useJsStore();
  const [run, status] = semioInternalOperationBind<Store, [string, Record<string, unknown>]>(async (st, designId, patch) => {
    const d = st.design(designId);
    for (const [key, raw] of Object.entries(patch)) {
      if (key === "name") {
        const r = await d.rename(String(raw ?? ""));
        if (!r.ok) return r;
      } else if (key === "description") {
        const r = await d.changeDescription(String(raw ?? ""));
        if (!r.ok) return r;
      } else if (key === "flatten" && raw === true) {
        const r = await d.flatten();
        if (!r.ok) return r;
      } else {
        console.warn(`[DEBUG] useUpdateDesign ignored non-DesignOperation patch key "${key}"`);
      }
    }
    return { ok: true } as const;
  })(() => store);
  const runDesign = React.useCallback((designId: string, patch: Record<string, unknown>) => run(designId, patch), [run]);
  return { run: runDesign, status };
}
// #endregion ✍️DesignWrites
// #endregion 🪝HooksDesign

// #region 🧰Type
const useTypeRenameOperation = semioInternalOperationBind<Type, [string]>((t, newName) => t.rename(newName));
const useTypeChangeDescriptionOperation = semioInternalOperationBind<Type, [string]>((t, d) => t.changeDescription(d));
const useTypeChangeIconOperation = semioInternalOperationBind<Type, [string]>((t, i) => t.changeIcon(i));
const useTypeAddAttributeOperation = semioInternalOperationBind<Type, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTypeRemoveAttributeOperation = semioInternalOperationBind<Type, [string]>((t, id) => t.removeAttribute(id));
const useTypeRemoveAttributesOperation = semioInternalOperationBind<Type, [readonly string[]]>((t, ids) => t.removeAttributes(ids));
const useTypeCreatePortOperation = semioInternalOperationBind<Type, [string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, number | null | undefined]>((t, code, label, description, icon, order) =>
  t.createPort(code ?? null, label ?? null, description ?? null, icon ?? null, order ?? null),
);
const useTypeDeletePortOperation = semioInternalOperationBind<Type, [string]>((t, id) => t.deletePort(id));
const useTypeDeletePortsOperation = semioInternalOperationBind<Type, [readonly string[]]>((t, ids) => t.deletePorts(ids));
const useTypeAddConnectorOperation = semioInternalOperationBind<Type, [string, string | null | undefined, string | null | undefined, string | null | undefined]>((t, code, description, icon, portId) =>
  t.addConnector(code, description ?? null, icon ?? null, portId ?? null),
);
const useTypeRemoveConnectorOperation = semioInternalOperationBind<Type, [string]>((t, id) => t.removeConnector(id));
const useTypeRemoveConnectorsOperation = semioInternalOperationBind<Type, [readonly string[]]>((t, ids) => t.removeConnectors(ids));

/** @emoji 📖 Live {@link Type#name}. */
export function useTypeName(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖 Live {@link Type#description}. */
export function useTypeDescription(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖 Live {@link Type#icon}. */
export function useTypeIcon(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖 Live {@link Type#image}. */
export function useTypeImage(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.image());
}

/** @emoji 📖 Live {@link Type#unit}. */
export function useTypeUnit(typeId?: string): FieldReadState<string> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.unit());
}

/** @emoji 📖 Bulky {@link Type#connectors}. */
export function useTypeConnectors(typeId?: string): FieldReadState<readonly Connector[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.connectors());
}

/** @emoji 📖 Bulky {@link Type#representations}. */
export function useTypeRepresentations(typeId?: string): FieldReadState<readonly Representation[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.representations());
}

/** @emoji 📖 Bulky {@link Type#attributes}. */
export function useTypeAttributes(typeId?: string): FieldReadState<readonly Attribute[]> {
  const entity = useResolvedType(typeId);
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji ✍️ {@link TypeOperationInput#rename}. */
export function useRenameType(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeRenameOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeDescription}. */
export function useChangeTypeDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeIcon}. */
export function useChangeTypeIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addAttribute}. */
export function useAddTypeAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttribute}. */
export function useRemoveTypeAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttributes}. */
export function useRemoveTypeAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveAttributesOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#createPort}. */
export function useCreatePort(): readonly [(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeCreatePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePort}. */
export function useDeletePort(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeDeletePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePorts}. */
export function useDeletePorts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeDeletePortsOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addConnector}. */
export function useAddConnector(): readonly [(code: string, description?: string | null, icon?: string | null, portId?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeAddConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnector}. */
export function useRemoveConnector(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnectors}. */
export function useRemoveConnectors(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveConnectorsOperation(() => e);
}
// #endregion 🧰Type

// #region 🔘Port
const usePortRenameOperation = semioInternalOperationBind<Port, [string, string | null | undefined]>((p, newCode, newLabel) => p.rename(newCode, newLabel));
const usePortChangeDescriptionOperation = semioInternalOperationBind<Port, [string]>((p, d) => p.changeDescription(d));
const usePortChangeIconOperation = semioInternalOperationBind<Port, [string]>((p, i) => p.changeIcon(i));
const usePortAddAttributeOperation = semioInternalOperationBind<Port, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePortRemoveAttributeOperation = semioInternalOperationBind<Port, [string]>((p, id) => p.removeAttribute(id));
const usePortRemoveAttributesOperation = semioInternalOperationBind<Port, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji 📖 Live {@link Port#code}. */
export function usePortCode(): FieldReadState<string> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.code());
}

/** @emoji 📖 Live {@link Port#label}. */
export function usePortLabel(): FieldReadState<string> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.label());
}

/** @emoji 📖 Live {@link Port#order}. */
export function usePortOrder(): FieldReadState<number | null> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.order());
}

/** @emoji 📖 Live {@link Port#name}. */
export function usePortName(): FieldReadState<string> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖 Live {@link Port#description}. */
export function usePortDescription(): FieldReadState<string> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖 Live {@link Port#icon}. */
export function usePortIcon(): FieldReadState<string> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖 Bulky {@link Port#attributes}. */
export function usePortAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji ✍️ {@link PortOperationInput#rename}. */
export function useRenamePort(): readonly [(newCode: string, newLabel?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortRenameOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeDescription}. */
export function useChangePortDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeIcon}. */
export function useChangePortIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#addAttribute}. */
export function useAddPortAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttribute}. */
export function useRemovePortAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttributes}. */
export function useRemovePortAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolvePort();
  return usePortRemoveAttributesOperation(() => e);
}
// #endregion 🔘Port

// #region 🔗Connector
const useConnectorRenameOperation = semioInternalOperationBind<Connector, [string]>((c, newCode) => c.rename(newCode));
const useConnectorChangeDescriptionOperation = semioInternalOperationBind<Connector, [string]>((c, d) => c.changeDescription(d));
const useConnectorChangeIconOperation = semioInternalOperationBind<Connector, [string]>((c, i) => c.changeIcon(i));

/** @emoji 📖 Live {@link Connector#code}. */
export function useConnectorCode(): FieldReadState<string> {
  const entity = resolveConnector();
  return useCurrentEntityField(entity, (c) => c.code());
}

/** @emoji 📖 Live {@link Connector#description}. */
export function useConnectorDescription(): FieldReadState<string> {
  const entity = resolveConnector();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Connector#icon}. */
export function useConnectorIcon(): FieldReadState<string> {
  const entity = resolveConnector();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Bulky {@link Connector#attributes}. */
export function useConnectorAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveConnector();
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link ConnectorOperationInput#rename}. */
export function useRenameConnector(): readonly [(newCode: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConnector();
  return useConnectorRenameOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeDescription}. */
export function useChangeConnectorDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConnector();
  return useConnectorChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeIcon}. */
export function useChangeConnectorIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConnector();
  return useConnectorChangeIconOperation(() => e);
}
// #endregion 🔗Connector

// #region ✍️Author
/** @emoji 📖 Live {@link Author#name}. */
export function useAuthorName(): FieldReadState<string> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.name());
}

/** @emoji 📖 Live {@link Author#email}. */
export function useAuthorEmail(): FieldReadState<string> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.email());
}

/** @emoji 📖 Live {@link Author#rank}. */
export function useAuthorRank(): FieldReadState<number | null> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.rank());
}

/** @emoji 📖 Live {@link Author#description}. */
export function useAuthorDescription(): FieldReadState<string> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.description());
}

/** @emoji 📖 Live {@link Author#icon}. */
export function useAuthorIcon(): FieldReadState<string> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.icon());
}

/** @emoji 📖 Live {@link Author#role}. */
export function useAuthorRole(): FieldReadState<string> {
  const entity = resolveAuthor();
  return useCurrentEntityField(entity, (a) => a.role());
}
// #endregion ✍️Author

// #region 💎Quality
const useQualityRenameOperation = semioInternalOperationBind<Quality, [string]>((q, k) => q.rename(k));
const useQualityChangeDescriptionOperation = semioInternalOperationBind<Quality, [string]>((q, d) => q.changeDescription(d));
const useQualityChangeIconOperation = semioInternalOperationBind<Quality, [string]>((q, i) => q.changeIcon(i));
const useQualityAddAttributeOperation = semioInternalOperationBind<Quality, [string, string, string]>((q, key, value, definition) => q.addAttribute(key, value, definition));
const useQualityRemoveAttributeOperation = semioInternalOperationBind<Quality, [string]>((q, id) => q.removeAttribute(id));
const useQualityRemoveAttributesOperation = semioInternalOperationBind<Quality, [readonly string[]]>((q, ids) => q.removeAttributes(ids));

/** @emoji 📖 Live {@link Quality#key}. */
export function useQualityKey(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.key());
}

/** @emoji 📖 Live {@link Quality#value}. */
export function useQualityValue(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.value());
}

/** @emoji 📖 Live {@link Quality#unit}. */
export function useQualityUnit(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.unit());
}

/** @emoji 📖 Live {@link Quality#definition}. */
export function useQualityDefinition(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.definition());
}

/** @emoji 📖 Live {@link Quality#description}. */
export function useQualityDescription(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.description());
}

/** @emoji 📖 Live {@link Quality#icon}. */
export function useQualityIcon(): FieldReadState<string> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.icon());
}

/** @emoji 📖 Live {@link Quality#attributes}. */
export function useQualityAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.attributes());
}

/** @emoji 📖 Live {@link Quality#benchmarks}. */
export function useQualityBenchmarks(): FieldReadState<readonly Benchmark[]> {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.benchmarks());
}

/** @emoji ✍️ {@link Quality#rename}. */
export function useRenameQuality(): readonly [(newKey: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityRenameOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeDescription}. */
export function useChangeQualityDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeIcon}. */
export function useChangeQualityIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Quality#addAttribute}. */
export function useAddQualityAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttribute}. */
export function useRemoveQualityAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttributes}. */
export function useRemoveQualityAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveQuality();
  return useQualityRemoveAttributesOperation(() => e);
}
// #endregion 💎Quality

// #region 🏷️Tag
const useTagRenameOperation = semioInternalOperationBind<Tag, [string]>((t, n) => t.rename(n));
const useTagChangeDescriptionOperation = semioInternalOperationBind<Tag, [string]>((t, d) => t.changeDescription(d));
const useTagChangeIconOperation = semioInternalOperationBind<Tag, [string]>((t, i) => t.changeIcon(i));
const useTagAddAttributeOperation = semioInternalOperationBind<Tag, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTagRemoveAttributeOperation = semioInternalOperationBind<Tag, [string]>((t, id) => t.removeAttribute(id));
const useTagRemoveAttributesOperation = semioInternalOperationBind<Tag, [readonly string[]]>((t, ids) => t.removeAttributes(ids));

/** @emoji 📖 Live {@link Tag#name}. */
export function useTagName(): FieldReadState<string> {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖 Live {@link Tag#description}. */
export function useTagDescription(): FieldReadState<string> {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖 Live {@link Tag#icon}. */
export function useTagIcon(): FieldReadState<string> {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖 Live {@link Tag#order}. */
export function useTagOrder(): FieldReadState<number | null> {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.order());
}

/** @emoji 📖 Live {@link Tag#attributes}. */
export function useTagAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji ✍️ {@link Tag#rename}. */
export function useRenameTag(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagRenameOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeDescription}. */
export function useChangeTagDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeIcon}. */
export function useChangeTagIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Tag#addAttribute}. */
export function useAddTagAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttribute}. */
export function useRemoveTagAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttributes}. */
export function useRemoveTagAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveTag();
  return useTagRemoveAttributesOperation(() => e);
}
// #endregion 🏷️Tag

// #region 💡Concept
const useConceptRenameOperation = semioInternalOperationBind<Concept, [string]>((c, n) => c.rename(n));
const useConceptChangeDescriptionOperation = semioInternalOperationBind<Concept, [string]>((c, d) => c.changeDescription(d));
const useConceptChangeIconOperation = semioInternalOperationBind<Concept, [string]>((c, i) => c.changeIcon(i));
const useConceptAddAttributeOperation = semioInternalOperationBind<Concept, [string, string, string]>((c, key, value, definition) => c.addAttribute(key, value, definition));
const useConceptRemoveAttributeOperation = semioInternalOperationBind<Concept, [string]>((c, id) => c.removeAttribute(id));
const useConceptRemoveAttributesOperation = semioInternalOperationBind<Concept, [readonly string[]]>((c, ids) => c.removeAttributes(ids));

/** @emoji 📖 Live {@link Concept#name}. */
export function useConceptName(): FieldReadState<string> {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖 Live {@link Concept#description}. */
export function useConceptDescription(): FieldReadState<string> {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Concept#icon}. */
export function useConceptIcon(): FieldReadState<string> {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Live {@link Concept#order}. */
export function useConceptOrder(): FieldReadState<number | null> {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.order());
}

/** @emoji 📖 Live {@link Concept#attributes}. */
export function useConceptAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link Concept#rename}. */
export function useRenameConcept(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptRenameOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeDescription}. */
export function useChangeConceptDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeIcon}. */
export function useChangeConceptIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Concept#addAttribute}. */
export function useAddConceptAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttribute}. */
export function useRemoveConceptAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttributes}. */
export function useRemoveConceptAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolveConcept();
  return useConceptRemoveAttributesOperation(() => e);
}
// #endregion 💡Concept

// #region 🎨Representation
/** @emoji 📖 Live {@link Representation#url}. */
export function useRepresentationUrl(): FieldReadState<string> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.url());
}

/** @emoji 📖 Live {@link Representation#description}. */
export function useRepresentationDescription(): FieldReadState<string> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.description());
}

/** @emoji 📖 Live {@link Representation#tags}. */
export function useRepresentationTags(): FieldReadState<readonly Tag[]> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.tags());
}

/** @emoji 📖 Live {@link Representation#qualities}. */
export function useRepresentationQualities(): FieldReadState<readonly Quality[]> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.qualities());
}

/** @emoji 📖 Live {@link Representation#attributes}. */
export function useRepresentationAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.attributes());
}

/** @emoji 📖 Live {@link Representation#file}. */
export function useRepresentationFile(): FieldReadState<File | null> {
  const entity = resolveRepresentation();
  return useCurrentEntityField(entity, (r) => r.file());
}
// #endregion 🎨Representation

// #region 🧩Piece
/** @emoji 📖 Live {@link Piece#name}. */
export function usePieceName(): FieldReadState<string> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖 Live {@link Piece#description}. */
export function usePieceDescription(): FieldReadState<string> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖 Live {@link Piece#icon}. */
export function usePieceIcon(): FieldReadState<string> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖 Live {@link Piece#scale}. */
export function usePieceScale(): FieldReadState<number | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.scale());
}

/** @emoji 📖 Live {@link Piece#position}. */
export function usePiecePosition(): FieldReadState<Position> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.position());
}

/** @emoji 📖 Live {@link Piece#flatPosition}. */
export function usePieceFlatPosition(): FieldReadState<Position> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition());
}

/** @emoji 📖 Live {@link Piece#plane}. */
export function usePiecePlane(): FieldReadState<Plane | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.position().plane());
}

/** @emoji 📖 Live {@link Piece#center}. */
export function usePieceCenter(): FieldReadState<Coordinate | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.position().center());
}

/** @emoji 📖 Live {@link Piece#flatPlane}. */
export function usePieceFlatPlane(): FieldReadState<Plane | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition().plane());
}

/** @emoji 📖 Live {@link Piece#flatCenter}. */
export function usePieceFlatCenter(): FieldReadState<Coordinate | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, async (p) => p.flatPosition().center());
}

/** @emoji 📖 Live {@link Piece#blueprint}. */
export function usePieceBlueprint(): FieldReadState<PieceBlueprint | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.blueprint());
}

/** @emoji 📖 Live {@link Piece#attributes}. */
export function usePieceAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji 📖 Live {@link Piece#connectionKind}. */
export function usePieceConnectionKind(): FieldReadState<"FIXED" | "CONNECTED" | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.connectionKind());
}

/** @emoji 📖 Live {@link Piece#parentPiece}. */
export function usePieceParentPiece(): FieldReadState<Piece | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.parentPiece());
}

/** @emoji 📖 Live {@link Piece#parentConnection}. */
export function usePieceParentConnection(): FieldReadState<Connection | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.parentConnection());
}

/** @emoji 📖 Live {@link Piece#childPieces}. */
export function usePieceChildPieces(): FieldReadState<readonly Piece[]> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.childPieces());
}

/** @emoji 📖 Live {@link Piece#childConnections}. */
export function usePieceChildConnections(): FieldReadState<readonly Connection[]> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.childConnections());
}

/** @emoji 📖 Live {@link Piece#depth}. */
export function usePieceDepth(): FieldReadState<number | null> {
  const entity = resolvePiece();
  return useCurrentEntityField(entity, (p) => p.depth());
}

const usePieceRenameOperation = semioInternalOperationBind<Piece, [string]>((p, n) => p.rename(n));
const usePieceChangeDescriptionOperation = semioInternalOperationBind<Piece, [string]>((p, d) => p.changeDescription(d));
const usePieceDragOperation = semioInternalOperationBind<Piece, [OffsetInput]>((p, o) => p.drag(o));
const usePieceMoveOperation = semioInternalOperationBind<Piece, [PositionInput]>((p, pos) => p.move(pos));
const usePieceFixOperation = semioInternalOperationBind<Piece, []>((p) => p.fix());
const usePieceChangeBlueprintOperation = semioInternalOperationBind<Piece, [string]>((p, id) => p.changeBlueprint(id));
const usePieceAddAttributeOperation = semioInternalOperationBind<Piece, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePieceRemoveAttributeOperation = semioInternalOperationBind<Piece, [string]>((p, id) => p.removeAttribute(id));
const usePieceRemoveAttributesOperation = semioInternalOperationBind<Piece, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji ✍️ {@link Piece#rename} bound to {@link PieceContext}. */
export function useRenamePiece(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceRenameOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeDescription}. */
export function useChangePieceDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Piece#drag}. */
export function useDragPiece(): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceDragOperation(() => e);
}

/** @emoji ✍️ {@link Piece#move}. */
export function useMovePiece(): readonly [(position: PositionInput) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceMoveOperation(() => e);
}

/** @emoji ✍️ {@link Piece#fix}. */
export function useFixPiece(): readonly [() => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceFixOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeBlueprint}. */
export function useChangePieceBlueprint(): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceChangeBlueprintOperation(() => e);
}

/** @emoji ✍️ {@link Piece#addAttribute}. */
export function useAddPieceAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttribute}. */
export function useRemovePieceAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttributes}. */
export function useRemovePieceAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = resolvePiece();
  return usePieceRemoveAttributesOperation(() => e);
}
// #endregion 🧩Piece

// #region 🪢Pieces
/**
 * @emoji 🪝 Binds {@link PiecesOperation} batch mutations (not an {@link Entity} — no cached kit state on the handle).
 * @typeParam Args — forwarded to the underlying {@link PiecesOperation} method after the ops handle.
 */
function semioInternalPiecesOpBind<Args extends unknown[]>(impl: (ops: PiecesOperation, ...args: Args) => Promise<SetResult>): (getOps: () => PiecesOperation | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function usePiecesBatchOp(getOps: () => PiecesOperation | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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

const usePiecesDragOperation = semioInternalPiecesOpBind((ops, o: OffsetInput) => ops.drag(o));
const usePiecesMoveOperation = semioInternalPiecesOpBind((ops, o: OffsetInput) => ops.move(o));
const usePiecesFixOperation = semioInternalPiecesOpBind((ops) => ops.fix());
const usePiecesChangeBlueprintOperation = semioInternalPiecesOpBind((ops, id: string) => ops.changeBlueprint(id));

/** @emoji ✍️ {@link PiecesOperation#drag} using {@link PiecesBatchContext} + {@link DesignIdContext}. */
export function useDragPieces(): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId ?? null;
  const batch = React.useContext(PiecesBatchContext);
  const pieceIds = batch?.pieceIds ?? [];
  const getOps = React.useCallback(() => (designId == null || pieceIds.length === 0 ? null : new PiecesOperation(store.session, designId, pieceIds, store.id)), [store.session, store.id, designId, pieceIds]);
  return usePiecesDragOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#move}. */
export function useMovePieces(): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId ?? null;
  const batch = React.useContext(PiecesBatchContext);
  const pieceIds = batch?.pieceIds ?? [];
  const getOps = React.useCallback(() => (designId == null || pieceIds.length === 0 ? null : new PiecesOperation(store.session, designId, pieceIds, store.id)), [store.session, store.id, designId, pieceIds]);
  return usePiecesMoveOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#fix}. */
export function useFixPieces(): readonly [() => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId ?? null;
  const batch = React.useContext(PiecesBatchContext);
  const pieceIds = batch?.pieceIds ?? [];
  const getOps = React.useCallback(() => (designId == null || pieceIds.length === 0 ? null : new PiecesOperation(store.session, designId, pieceIds, store.id)), [store.session, store.id, designId, pieceIds]);
  return usePiecesFixOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#changeBlueprint}. */
export function useChangePiecesBlueprint(): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const store = useJsStore();
  const designId = React.useContext(DesignIdContext)?.id ?? React.useContext(DesignContext)?.designId ?? null;
  const batch = React.useContext(PiecesBatchContext);
  const pieceIds = batch?.pieceIds ?? [];
  const getOps = React.useCallback(() => (designId == null || pieceIds.length === 0 ? null : new PiecesOperation(store.session, designId, pieceIds, store.id)), [store.session, store.id, designId, pieceIds]);
  return usePiecesChangeBlueprintOperation(getOps);
}
// #endregion 🪢Pieces

// #region ⛓️Connection
/** @emoji 📖 Live {@link Connection#gap}. */
export function useConnectionGap(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.gap());
}

/** @emoji 📖 Live {@link Connection#shift}. */
export function useConnectionShift(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.shift());
}

/** @emoji 📖 Live {@link Connection#rise}. */
export function useConnectionRise(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rise());
}

/** @emoji 📖 Live {@link Connection#rotation}. */
export function useConnectionRotation(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rotation());
}

/** @emoji 📖 Live {@link Connection#turn}. */
export function useConnectionTurn(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.turn());
}

/** @emoji 📖 Live {@link Connection#tilt}. */
export function useConnectionTilt(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.tilt());
}

/** @emoji 📖 Live {@link Connection#u}. */
export function useConnectionU(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.u());
}

/** @emoji 📖 Live {@link Connection#v}. */
export function useConnectionV(): FieldReadState<number | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.v());
}

/** @emoji 📖 Live {@link Connection#connected}. */
export function useConnectionConnected(): FieldReadState<Side | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.connected());
}

/** @emoji 📖 Live {@link Connection#connecting}. */
export function useConnectionConnecting(): FieldReadState<Side | null> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.connecting());
}

/** @emoji 📖 Live {@link Connection#name}. */
export function useConnectionName(): FieldReadState<string> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖 Live {@link Connection#description}. */
export function useConnectionDescription(): FieldReadState<string> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Connection#icon}. */
export function useConnectionIcon(): FieldReadState<string> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Live {@link Connection#attributes}. */
export function useConnectionAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.attributes());
}
// #endregion ⛓️Connection

// #region 🎛️WriteIndicator
type LegacyWriteRowStatus =
  | { readonly kind: "readonly" }
  | { readonly kind: "idle"; readonly pending: number }
  | { readonly kind: "pending"; readonly pending: number }
  | { readonly kind: "error"; readonly pending: number; readonly lastError: SetError };

/** @emoji 🎛️ Maps {@link OperationStatus} or legacy triad status into sketchpad row affordances. */
export function useWriteIndicator(status: OperationStatus | LegacyWriteRowStatus): Readonly<{
  disabled: boolean;
  spinning: boolean;
  error?: SetError;
}> {
  if (status.kind === "readonly") {
    return { disabled: true, spinning: false, error: undefined };
  }
  if (status.kind === "error" && "lastError" in status) {
    return { disabled: false, spinning: false, error: status.lastError };
  }
  if (status.kind === "pending" && "pending" in status) {
    return { disabled: true, spinning: true, error: undefined };
  }
  if (status.kind === "idle" && "pending" in status) {
    return { disabled: false, spinning: false, error: undefined };
  }
  const op = status as OperationStatus;
  if (op.kind === "pending") {
    return { disabled: true, spinning: true, error: undefined };
  }
  if (op.kind === "settled" && !op.result.ok) {
    return { disabled: false, spinning: false, error: op.result.error };
  }
  return { disabled: false, spinning: false, error: undefined };
}
// #endregion 🎛️WriteIndicator

// #region ⚛️Embedded tests
// @emoji 🧹 Legacy InMemoryKitStore embedded block removed during single-source Kit migration; restore with GraphQL Kit stubs only.
// #endregion ⚛️Embedded tests

// #region 🧪Vitest
if (import.meta.vitest) {
  const { existsSync, readFileSync } = await import("node:fs");
  const path = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const { describe, expect, it } = import.meta.vitest;
  const reactSrcPath = (() => {
    const candidates: string[] = [];
    try {
      const u = import.meta.url;
      if (typeof u === "string" && u.startsWith("file:")) {
        candidates.push(fileURLToPath(new URL("./index.tsx", import.meta.url)));
      }
    } catch {
      /* ignore */
    }
    candidates.push(path.join(process.cwd(), "index.tsx"), path.join(process.cwd(), "semio", "client", "lib", "react", "index.tsx"), path.join(process.cwd(), "client", "lib", "react", "index.tsx"));
    const hit = candidates.find((p) => existsSync(p));
    if (hit != null) return hit;
    throw new Error(`[DEBUG] semio/react vitest: cannot resolve index.tsx; cwd=${process.cwd()}; tried:\n${candidates.join("\n")}`);
  })();
  const reactSrc = readFileSync(reactSrcPath, "utf8");
  const vitestRegion = reactSrc.indexOf("// #region 🧪Vitest");
  const reactSrcForBannedScan = vitestRegion === -1 ? reactSrc : reactSrc.slice(0, vitestRegion);
  describe("semio/react sealed surface", () => {
    const exportNames = [...reactSrc.matchAll(/^export (?:function|const) (\w+)/gm)].map((m) => m[1]);
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
      "PiecesOperation",
      "EventBus",
      "Operation",
    ]);
    it("exports no semio/js entity-class symbols", () => {
      for (const n of exportNames) {
        expect.soft(bannedExports.has(n), `illegal export: ${n}`).toBe(false);
      }
    });
  });
  describe("schema-1:1 banned patterns (this file)", () => {
    const mustNotMatchCode = [
      /\bbindFieldToReact\b/,
      /\bbindOperationToReact\b/,
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
        expect.soft(reactSrcForBannedScan.match(re), String(re)).toBeNull();
      }
    });
  });
}
// #endregion 🧪Vitest
