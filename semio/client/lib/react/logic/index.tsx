// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link } reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
// Value/type re-exports follow the local `@semio/js` imports below (single binding per symbol).
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type { ReactNode } from "react";
import * as React from "react";
import type { Attribute, Coordinate, Entity, GraphRootKind, OffsetInput, PieceBlueprint, Plane, Position, PositionInput, SetError, SetResult } from "../../js";
import { Alternative, Author, Backbone, Concept, Connection, Connector, Design, File, Graph, Kit, LocalProvider, Piece, PiecesOperation, Port, Quality, RemoteProvider, Representation, Session, Side, Store, Tag, TheKit, Type } from "../../js";
// #endregion ⚛️Imports

// #region 🪝FieldBind
type FieldBindOptions<E, T> = Readonly<{
  /** @emoji 🧲 Single async read (one GraphQL selection / entity method). */
  read: (entity: E) => Promise<T>;
  /** @emoji 📡 When set, {@link Store#bus} {@code subscribeKind}; when omitted, only mount pull fresh data. */
  eventKind?: string;
  /** @emoji 🪝  source; re-invoked each render — keep stable via {@link React#useCallback}. */
  get: () => E | null;
}>;

const EMPTY_IDS = Object.freeze([]) as readonly string[];

/**
 * @emoji 🪝 Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
 * @returns Last resolved value, or {@code undefined} before the first successful read or when the entity is absent.
 */
function semioInternalFieldBind<E extends Entity, T>(opts: FieldBindOptions<E, T>): () => T | undefined {
  const { read, eventKind, get } = opts;
  return function use(): T | undefined {
    const entity = get();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const entityRef = React.useRef(entity);
    entityRef.current = entity;

    const refresh = React.useCallback(async () => {
      const e = entityRef.current;
      if (e == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await read(e));
      } catch {
        setValue(undefined);
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

    return value;
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
function semioInternalOperationBind<E extends Entity, Args extends unknown[] = []>(
  impl: (entity: E, ...args: Args) => SetResult | Promise<SetResult> | void | Promise<void>,
): (get: () => E | null) => readonly [(...args: Args) => void, OperationStatus] {
  return function useOperation(get: () => E | null): readonly [(...args: Args) => void, OperationStatus] {
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
          const raw = (await impl(e, ...args)) as SetResult | void | undefined;
          const result: SetResult = raw === undefined ? ({ ok: true } as const) : (raw as SetResult);
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

function semioInternalKitFieldBind<T>(opts: KitFieldBindOptions<T>): () => T | undefined {
  const { read, eventKind, getKit } = opts;
  return function useKitBound(): T | undefined {
    const kit = getKit();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const kitRef = React.useRef(kit);
    kitRef.current = kit;

    const refresh = React.useCallback(async () => {
      const k = kitRef.current;
      if (k == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await read(k));
      } catch {
        setValue(undefined);
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

    return value;
  };
}

export type StoreFieldBindOptions<T> = Readonly<{
  read: (store: Store) => Promise<T>;
  eventKind?: string;
  getStore: () => Store | null;
}>;

/** @emoji 🪝 Store-root field bind for session/backbone/root fields. */
function semioInternalStoreFieldBind<T>(opts: StoreFieldBindOptions<T>): () => T | undefined {
  const { read, eventKind, getStore } = opts;
  return function useStoreBound(): T | undefined {
    const store = getStore();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const storeRef = React.useRef(store);
    storeRef.current = store;

    const refresh = React.useCallback(async () => {
      const s = storeRef.current;
      if (s == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await read(s));
      } catch {
        setValue(undefined);
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

    return value;
  };
}
// #endregion 🪝KitFieldBind

// #region 🪝StoreOperationBind
/** @emoji 🪝 Binds a {@link Store} operation to `[run, status]`. */
function semioInternalStoreOperationBind<Args extends unknown[] = []>(
  impl: (store: Store, ...args: Args) => SetResult | Promise<SetResult> | void | Promise<void>,
): (getStore: () => Store | null) => readonly [(...args: Args) => void, OperationStatus] {
  return function useStoreOp(getStore: () => Store | null): readonly [(...args: Args) => void, OperationStatus] {
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
          const raw = (await impl(k, ...args)) as SetResult | void | undefined;
          const result: SetResult = raw === undefined ? ({ ok: true } as const) : (raw as SetResult);
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
function semioInternalSessionOperationBind<Args extends unknown[] = []>(
  impl: (session: Session, ...args: Args) => SetResult | Promise<SetResult> | void | Promise<void>,
): (getSession: () => Session | null) => readonly [(...args: Args) => void, OperationStatus] {
  return function useSessionOp(getSession: () => Session | null): readonly [(...args: Args) => void, OperationStatus] {
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
          const raw = (await impl(s, ...args)) as SetResult | void | undefined;
          const result: SetResult = raw === undefined ? ({ ok: true } as const) : (raw as SetResult);
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

function useCurrentEntityField<E extends Entity, T>(entity: E | null, read: (entity: E) => Promise<T>, eventKind?: string): T | undefined {
  return semioInternalFieldBind<E, T>({ get: () => entity, read, eventKind })();
}

function useCurrentEntityOperation<E extends Entity, Args extends unknown[] = []>(entity: E | null, useBound: (get: () => E | null) => readonly [(...args: Args) => void, OperationStatus]): readonly [(...args: Args) => void, OperationStatus] {
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

const StoreContext = React.createContext<string | null>(null);
const AlternativeContext = React.createContext<string | null>(null);
const KitContext = React.createContext<string | null>(null);
const DesignContext = React.createContext<string | null>(null);
const TypeContext = React.createContext<string | null>(null);
const AuthorContext = React.createContext<string | null>(null);
const QualityContext = React.createContext<string | null>(null);
const TagContext = React.createContext<string | null>(null);
const ConceptContext = React.createContext<string | null>(null);
const PieceContext = React.createContext<string | null>(null);
const ConnectionContext = React.createContext<string | null>(null);
const PortContext = React.createContext<string | null>(null);
const ConnectorContext = React.createContext<string | null>(null);
const RepresentationContext = React.createContext<string | null>(null);
const RemoteProviderUrlContext = React.createContext<string | null>(null);
const FileBackboneContext = React.createContext<string | null>(null);
const FolderBackboneContext = React.createContext<string | null>(null);
const WebsocketBackboneContext = React.createContext<string | null>(null);

const WipMarkerContext = React.createContext(false);
const StageMarkerContext = React.createContext(false);
const AuthoritativeMarkerContext = React.createContext(false);
const TheKitMarkerContext = React.createContext(false);

const PositionMarkerContext = React.createContext(false);
const FlatPositionMarkerContext = React.createContext(false);
const PlaneMarkerContext = React.createContext(false);
const OriginMarkerContext = React.createContext(false);

const PiecesBatchContext = React.createContext<Readonly<{ ids: readonly string[] }> | null>(null);

/** @emoji 🪢 Batch piece ids for {@link useDragPieces} under {@link DesignContext}. */
export function PiecesBatchContextProvider(props: Readonly<{ ids: readonly string[]; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ ids: props.ids }), [props.ids]);
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
  return React.createElement(StoreHandleContext.Provider, { value: store }, React.createElement(StoreContext.Provider, { value: props.id }, props.children));
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
  return React.createElement(RemoteProviderHandleContext.Provider, { value: rp }, React.createElement(RemoteProviderUrlContext.Provider, { value: props.id }, props.children));
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
  return React.createElement(AlternativeHandleContext.Provider, { value: alt }, React.createElement(AlternativeContext.Provider, { value: props.id }, props.children));
}

export function KitContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const store = React.useContext(StoreHandleContext);
  if (store == null) throw new Error("semio/react: KitContextProvider requires StoreContextProvider.");
  const kit = React.useMemo(() => new Kit(store.session, props.id, store.id), [store.session, store.id, props.id]);
  return React.createElement(KitHandleContext.Provider, { value: kit }, React.createElement(KitContext.Provider, { value: props.id }, props.children));
}

function mkIdProvider(C: React.Context<string | null>) {
  return function IdCtxProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
    return React.createElement(C.Provider, { value: props.id }, props.children);
  };
}

export const DesignContextProvider = mkIdProvider(DesignContext);
export const TypeContextProvider = mkIdProvider(TypeContext);
export const AuthorContextProvider = mkIdProvider(AuthorContext);
export const QualityContextProvider = mkIdProvider(QualityContext);
export const TagContextProvider = mkIdProvider(TagContext);
export const ConceptContextProvider = mkIdProvider(ConceptContext);
export const PieceContextProvider = mkIdProvider(PieceContext);
export const ConnectionContextProvider = mkIdProvider(ConnectionContext);
export const PortContextProvider = mkIdProvider(PortContext);
export const ConnectorContextProvider = mkIdProvider(ConnectorContext);
export const RepresentationContextProvider = mkIdProvider(RepresentationContext);

export function FileBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const lp = React.useContext(LocalProviderHandleContext);
  if (session == null || lp == null) throw new Error("semio/react: FileBackboneContextProvider requires Session + LocalProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, lp), [session, props.id, lp]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(FileBackboneContext.Provider, { value: props.id }, props.children));
}

export function FolderBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const lp = React.useContext(LocalProviderHandleContext);
  if (session == null || lp == null) throw new Error("semio/react: FolderBackboneContextProvider requires Session + LocalProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, lp), [session, props.id, lp]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(FolderBackboneContext.Provider, { value: props.id }, props.children));
}

export function WebsocketBackboneContextProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const session = React.useContext(SessionHandleContext);
  const rp = React.useContext(RemoteProviderHandleContext);
  if (session == null || rp == null) throw new Error("semio/react: WebsocketBackboneContextProvider requires Session + RemoteProvider.");
  const bb = React.useMemo(() => new Backbone(session, props.id, rp), [session, props.id, rp]);
  return React.createElement(BackboneHandleContext.Provider, { value: bb }, React.createElement(WebsocketBackboneContext.Provider, { value: props.id }, props.children));
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

/** @emoji 🏪 Active {@link Store} branch (requires {@link StoreContextProvider}). */
export function useJsStore(): Store {
  const s = React.useContext(StoreHandleContext);
  if (s == null) throw new Error("semio/react: useJsStore requires StoreContextProvider id={…}.");
  return s;
}

function useJsSession(): Session {
  const s = React.useContext(SessionHandleContext);
  if (s == null) throw new Error("semio/react: useJsSession requires SessionContextProvider.");
  return s;
}

function readOptional(ctx: React.Context<string | null>, id?: ID): string | null {
  const fromCtx = React.useContext(ctx);
  const rid = id ?? fromCtx;
  return rid == null || rid === "" ? null : rid;
}

function resolveKit(id?: ID): Kit | null {
  const k = React.useContext(KitHandleContext);
  const rid = readOptional(KitContext, id);
  if (k == null || rid == null) return null;
  return k.id === rid ? k : new Kit(k.session, rid, k.storeId);
}

function resolveDesign(id?: ID): Design | null {
  const st = React.useContext(StoreHandleContext);
  const did = readOptional(DesignContext, id);
  if (st == null || did == null) return null;
  return st.design(did);
}

function resolvePiece(id?: ID): Piece | null {
  const d = resolveDesign();
  const pid = readOptional(PieceContext, id);
  if (d == null || pid == null) return null;
  return d.piece(pid);
}

function resolveType(id?: ID): Type | null {
  const st = React.useContext(StoreHandleContext);
  const tid = readOptional(TypeContext, id);
  if (st == null || tid == null) return null;
  return st.type(tid);
}

function resolveConnection(id?: ID): Connection | null {
  const d = resolveDesign();
  const cid = readOptional(ConnectionContext, id);
  if (d == null || cid == null) return null;
  return d.connection(cid);
}

function resolvePort(id?: ID): Port | null {
  const t = resolveType();
  const pid = readOptional(PortContext, id);
  if (t == null || pid == null) return null;
  return t.port(pid);
}

function resolveConnector(id?: ID): Connector | null {
  const t = resolveType();
  const cid = readOptional(ConnectorContext, id);
  if (t == null || cid == null) return null;
  return t.connector(cid);
}

function resolveRepresentation(id?: ID): Representation | null {
  const t = resolveType();
  const rid = readOptional(RepresentationContext, id);
  if (t == null || rid == null) return null;
  return t.representation(rid);
}

function resolveQuality(id?: ID): Quality | null {
  const st = React.useContext(StoreHandleContext);
  const qid = readOptional(QualityContext, id);
  if (st == null || qid == null) return null;
  return st.quality(qid);
}

function resolveTag(id?: ID): Tag | null {
  const st = React.useContext(StoreHandleContext);
  const tid = readOptional(TagContext, id);
  if (st == null || tid == null) return null;
  return st.tag(tid);
}

function resolveConcept(id?: ID): Concept | null {
  const st = React.useContext(StoreHandleContext);
  const cid = readOptional(ConceptContext, id);
  if (st == null || cid == null) return null;
  return st.concept(cid);
}

function resolveAuthor(id?: ID): Author | null {
  const st = React.useContext(StoreHandleContext);
  const aid = readOptional(AuthorContext, id);
  if (st == null || aid == null) return null;
  return st.author(aid);
}

function resolveFile(id?: ID): File | null {
  const st = React.useContext(StoreHandleContext);
  const fid = id ?? null;
  if (st == null || fid == null) return null;
  return st.file(fid);
}

function resolveAlternative(id?: ID): Alternative | null {
  const g = React.useContext(GraphHandleContext);
  const fromCtx = React.useContext(AlternativeContext);
  const aid = id ?? fromCtx;
  if (g == null || aid == null || aid === "") return null;
  return g.alternative(aid);
}

function resolveLocalProvider(): LocalProvider | null {
  return React.useContext(LocalProviderHandleContext);
}

function resolveRemoteProvider(url?: string): RemoteProvider | null {
  const rp = React.useContext(RemoteProviderHandleContext);
  const fromCtx = React.useContext(RemoteProviderUrlContext);
  const urlKey = url ?? fromCtx;
  if (rp != null && (url == null || rp.url === url)) return rp;
  if (urlKey == null || urlKey === "") return null;
  return useJsSession().remoteProvider(urlKey);
}

function resolveBackbone(id?: ID): Backbone | null {
  const bb = React.useContext(BackboneHandleContext);
  const bid = id ?? React.useContext(FileBackboneContext) ?? React.useContext(FolderBackboneContext) ?? React.useContext(WebsocketBackboneContext) ?? null;
  if (bb != null && (bid == null || bb.id === bid)) return bb;
  return null;
}

function requireEntityId(ctx: React.Context<string | null>, hookName: string): string {
  const id = readOptional(ctx, undefined);
  if (id == null) throw new Error(`semio/react: ${hookName} requires a matching ContextProvider above.`);
  return id;
}

export function useSession(): string {
  if (React.useContext(SessionHandleContext) == null) throw new Error("semio/react: useSession requires SessionContextProvider.");
  return "__session__";
}

export function useStore(): string {
  return requireEntityId(StoreContext, "useStore");
}

export function useWip(): string {
  if (!React.useContext(WipMarkerContext)) throw new Error("semio/react: useWip requires WipContextProvider.");
  return "wip";
}
export function useStage(): string {
  if (!React.useContext(StageMarkerContext)) throw new Error("semio/react: useStage requires StageContextProvider.");
  return "stage";
}
export function useAuthoritative(): string {
  if (!React.useContext(AuthoritativeMarkerContext)) throw new Error("semio/react: useAuthoritative requires AuthoritativeContextProvider.");
  return "authoritative";
}

export function useTheKit(): string {
  if (!React.useContext(TheKitMarkerContext)) throw new Error("semio/react: useTheKit requires TheKitContextProvider.");
  const tk = React.useContext(TheKitHandleContext);
  if (tk == null) throw new Error("semio/react: useTheKit requires TheKitContextProvider.");
  return tk.id;
}

export function useAlternative(): string {
  return requireEntityId(AlternativeContext, "useAlternative");
}
export function useKit(): string {
  return requireEntityId(KitContext, "useKit");
}
export function useDesign(): string {
  return requireEntityId(DesignContext, "useDesign");
}
export function useType(): string {
  return requireEntityId(TypeContext, "useType");
}
export function useAuthor(): string {
  return requireEntityId(AuthorContext, "useAuthor");
}
export function useQuality(): string {
  return requireEntityId(QualityContext, "useQuality");
}
export function useTag(): string {
  return requireEntityId(TagContext, "useTag");
}
export function useConcept(): string {
  return requireEntityId(ConceptContext, "useConcept");
}
export function usePiece(): string {
  return requireEntityId(PieceContext, "usePiece");
}
export function useConnection(): string {
  return requireEntityId(ConnectionContext, "useConnection");
}
export function usePort(): string {
  return requireEntityId(PortContext, "usePort");
}
export function useConnector(): string {
  return requireEntityId(ConnectorContext, "useConnector");
}
export function useRepresentation(): string {
  return requireEntityId(RepresentationContext, "useRepresentation");
}
export function useLocalProvider(): string {
  const lp = resolveLocalProvider();
  if (lp == null) throw new Error("semio/react: useLocalProvider requires LocalProviderContextProvider.");
  return lp.id;
}
export function useRemoteProvider(): string {
  return requireEntityId(RemoteProviderUrlContext, "useRemoteProvider");
}

/** @emoji 🦴 Active backbone id from whichever of file / folder / websocket backbone contexts is mounted. */
export function useBackbone(): string {
  const file = React.useContext(FileBackboneContext);
  const folder = React.useContext(FolderBackboneContext);
  const ws = React.useContext(WebsocketBackboneContext);
  const id = file ?? folder ?? ws;
  if (id == null || id === "") throw new Error("semio/react: useBackbone requires FileBackboneContextProvider, FolderBackboneContextProvider, or WebsocketBackboneContextProvider.");
  return id;
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
/** @emoji 🧭 Active kit id from {@link KitContext}, if any. */
export function useKitContext(): string | null {
  return React.useContext(KitContext);
}

/** @emoji 🧭 Active store branch id from {@link StoreContext}, if any. */
export function useStoreContext(): string | null {
  return React.useContext(StoreContext);
}

/** @emoji 🧭 Active design id from {@link DesignContext}, if any. */
export function useDesignContext(): string | null {
  return React.useContext(DesignContext);
}

/** @emoji 🧭 Active piece id from {@link PieceContext}, if any. */
export function usePieceContext(): string | null {
  return React.useContext(PieceContext);
}

/** @emoji 🧭 Active connection id from {@link ConnectionContext}, if any. */
export function useConnectionContext(): string | null {
  return React.useContext(ConnectionContext);
}

/** @emoji 🧭 Active type id from {@link TypeContext}, if any. */
export function useTypeContext(): string | null {
  return React.useContext(TypeContext);
}

/** @emoji 🧭 Active quality id from {@link QualityContext}, if any. */
export function useQualityContext(): string | null {
  return React.useContext(QualityContext);
}

/** @emoji 🧭 Active author id from {@link AuthorContext}, if any. */
export function useAuthorContext(): string | null {
  return React.useContext(AuthorContext);
}

/** @emoji 🧷 {@link PieceContextProvider} using enclosing design scope. */
export function PieceUnderActiveDesignProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const designId = React.useContext(DesignContext);
  if (designId == null || designId === "") {
    throw new Error("semio/react: PieceUnderActiveDesignProvider requires DesignContextProvider.");
  }
  return React.createElement(PieceContext.Provider, { value: props.id }, props.children);
}

/** @emoji 🧷 {@link ConnectionContextProvider} using enclosing design scope. */
export function ConnectionUnderActiveDesignProvider(props: Readonly<{ id: string; children: ReactNode }>): React.ReactElement {
  const designId = React.useContext(DesignContext);
  if (designId == null || designId === "") {
    throw new Error("semio/react: ConnectionUnderActiveDesignProvider requires DesignContextProvider.");
  }
  return React.createElement(ConnectionContext.Provider, { value: props.id }, props.children);
}
// #endregion 🔖EntityContextHelpers

function useResolvedDesign(id?: ID): Design | null {
  return resolveDesign(id);
}

function useResolvedType(id?: ID): Type | null {
  return resolveType(id);
}

// #region 🪝IdStableEntityLists
/** @emoji 📚 Kit-level design ids via {@link Kit#designs}. */
export function useKitDesigns(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.designs()).map((d) => d.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Kit-level type ids via {@link Kit#types}. */
export function useKitTypes(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.types()).map((t) => t.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Kit-level author ids via {@link Kit#authors}. */
export function useKitAuthors(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.authors()).map((a) => a.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Kit-level quality ids via {@link Kit#qualities}. */
export function useKitQualities(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.qualities()).map((q) => q.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Kit-level tag ids via {@link Kit#tags}. */
export function useKitTags(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.tags()).map((t) => t.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Kit-level concept ids via {@link Kit#concepts}. */
export function useKitConcepts(): readonly string[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze((await k.concepts()).map((c) => c.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Piece ids in the active {@link DesignContext} design. */
export function useDesignPieces(): readonly string[] {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, async (design) => Object.freeze((await design.pieces()).map((p) => p.id))) ?? EMPTY_IDS;
}

/** @emoji 📚 Connection ids in the active {@link DesignContext} design. */
export function useDesignConnections(): readonly string[] {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, async (design) => Object.freeze((await design.connections()).map((c) => c.id))) ?? EMPTY_IDS;
}
// #endregion 🪝IdStableEntityLists

// #region 🪝HooksKit
// #region 📖KitReads
/** @emoji 📖 Live {@link Kit#name} + {@code kitRenamed}. */
export function useKitName(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.name(), "kitRenamed");
}

/** @emoji 📖 Live {@link Kit#description} + {@code changedDescription}. */
export function useKitDescription(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.description(), "changedDescription");
}

/** @emoji 📖 Live {@link Kit#icon}. */
export function useKitIcon(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.icon());
}

/** @emoji 📖 Live {@link Kit#image}. */
export function useKitImage(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.image());
}

/** @emoji 📖 Live {@link Kit#preview}. */
export function useKitPreview(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.preview());
}

/** @emoji 📖 Live {@link Kit#remote}. */
export function useKitRemote(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.remote());
}

/** @emoji 📖 Live {@link Kit#homepage}. */
export function useKitHomepage(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.homepage());
}

/** @emoji 📖 Live {@link Kit#license}. */
export function useKitLicense(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.license());
}

/** @emoji 📖 Live {@link Kit#uri}. */
export function useKitUri(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.uri());
}

/** @emoji 🧾 Exposes {@link Store#ensureChangeId} as a stable callback. */
export function useEnsureKitChange(): () => Promise<string> {
  const store = useJsStore();
  return React.useCallback(() => store.ensureChangeId(), [store]);
}
// #endregion 📖KitReads

// #region ✍️KitWrites
/** @emoji ✍️ {@link Kit#rename}. */
export function useRenameKit(): readonly [(newName: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, newName) => k.rename(newName))(() => kit);
}

/** @emoji ✍️ {@link Kit#changeDescription}. */
export function useChangeKitDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, d) => k.changeDescription(d))(() => kit);
}

/** @emoji ✍️ {@link Kit#createTag}. */
export function useCreateTag(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createTag(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTag}. */
export function useDeleteTag(): readonly [(id: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteTag(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTags}. */
export function useDeleteTags(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteTags(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createConcept}. */
export function useCreateConcept(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createConcept(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcept}. */
export function useDeleteConcept(): readonly [(id: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteConcept(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcepts}. */
export function useDeleteConcepts(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteConcepts(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createQuality}. */
export function useCreateQuality(): readonly [(key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, key, value, unit, definition, description, icon) =>
    k.createQuality(key, value, unit, definition, description, icon),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQuality}. */
export function useDeleteQuality(): readonly [(id: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteQuality(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQualities}. */
export function useDeleteQualities(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteQualities(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createType}. */
export function useCreateType(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createType(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteType}. */
export function useDeleteType(): readonly [(id: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteType(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTypes}. */
export function useDeleteTypes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteTypes(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createDesign}. */
export function useCreateDesign(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createDesign(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesign}. */
export function useDeleteDesign(): readonly [(id: string) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [string]>((k, id) => k.deleteDesign(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesigns}. */
export function useDeleteDesigns(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const kit = useWipKit();
  return semioInternalOperationBind<Kit, [readonly string[]]>((k, ids) => k.deleteDesigns(ids))(() => kit);
}

/** @emoji ✍️ {@link Store#saveChange}. */
export function useSaveKitChange(): readonly [() => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[]>(async (k) => {
    await k.saveChange();
    return { ok: true };
  })(() => store);
}

/** @emoji ✍️ {@link Store#createCheckpoint}. */
export function useCreateCheckpoint(): readonly [(message: string) => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[string]>((k, message) => k.createCheckpoint(message))(() => store);
}

/** @emoji ✍️ {@link Store#startAlternative}. */
export function useStartAlternative(): readonly [(name?: string | null) => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[string | null | undefined]>((k, name) => k.startAlternative(name ?? undefined))(() => store);
}

/** @emoji ✍️ {@link Store#integrateAlternative}. */
export function useIntegrateAlternative(): readonly [(id: string) => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[string]>((k, id) => k.integrateAlternative(id))(() => store);
}

/** @emoji ✍️ {@link Session#login}. */
export function useLogin(): readonly [(username: string, passwordHash: string, hubUrl?: string) => void, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOperationBind<[string, string, string | undefined]>((s, u, p, h) => s.login(u, p, h))(() => session);
}

/** @emoji ✍️ {@link Session#logout}. */
export function useLogout(): readonly [() => void, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOperationBind<[]>((s) => s.logout())(() => session);
}

/** @emoji ✍️ {@link Session#sessionStart}. */
export function useStartSession(): readonly [() => void, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOperationBind<[]>((s) => s.sessionStart())(() => session);
}

/** @emoji ✍️ {@link Session#sessionEnd}. */
export function useEndSession(): readonly [() => void, OperationStatus] {
  const session = useJsSession();
  return semioInternalSessionOperationBind<[]>((s) => s.sessionEnd())(() => session);
}

// #region 🪝BackboneOps
/** @emoji 🛜 {@link Store#attachBackbone} — attaches via {@link LocalProvider} for the active session. */
export function useAttachBackbone(): readonly [(uri: string) => void, OperationStatus] {
  const store = useJsStore();
  const session = useJsSession();
  return semioInternalStoreOperationBind<[string]>((s, uri) => s.attachBackbone(session.localProvider(), uri))(() => store);
}

/** @emoji 🛜 {@link Store#detachBackbone}. */
export function useDetachBackbone(): readonly [() => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[]>((s) => s.detachBackbone())(() => store);
}

/** @emoji 🛜 {@link Store#syncBackbone}. */
export function useBackboneSyncNow(): readonly [() => void, OperationStatus] {
  const store = useJsStore();
  return semioInternalStoreOperationBind<[]>((s) => s.syncBackbone())(() => store);
}

/** @emoji 🛜 Live {@link Session#backboneStatus} for the active store (refreshes on {@code commandSucceeded} bus events). */
export function useBackboneStatus(): Readonly<{ attachedUri: string | null; kind: string }> | undefined {
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
export function useDesignName(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.name()) ?? "";
}

/** @emoji 📖 Live {@link Design#description} + {@code changedDescription}. */
export function useDesignDescription(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.description(), "changedDescription") ?? "";
}

/** @emoji 📖 Live {@link Design#qualitySum}. */
export function useDesignQualitySum(id?: string): number {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.qualitySum()) ?? 0;
}

/** @emoji 📖 Live {@link Design#icon}. */
export function useDesignIcon(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.icon()) ?? "";
}

/** @emoji 📖 Live {@link Design#image}. */
export function useDesignImage(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.image()) ?? "";
}

/** @emoji 📖 Live {@link Design#unit}. */
export function useDesignUnit(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.unit()) ?? "";
}
// #endregion 📖DesignReads

// #region ✍️DesignWrites
/** @emoji ✍️ {@link Design#rename}. */
export function useRenameDesign(id?: string): readonly [(newName: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string]>((d, n) => d.rename(n))(() => entity);
}

/** @emoji ✍️ {@link Design#changeDescription}. */
export function useChangeDesignDescription(id?: string): readonly [(newDescription: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string]>((d, t) => d.changeDescription(t))(() => entity);
}

/** @emoji ✍️ {@link Design#changeIcon}. */
export function useChangeDesignIcon(id?: string): readonly [(newIcon: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string]>((d, i) => d.changeIcon(i))(() => entity);
}

/** @emoji ✍️ {@link Design#flatten}. */
export function useFlattenDesign(id?: string): readonly [() => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, []>((d) => d.flatten())(() => entity);
}

/** @emoji ✍️ {@link Design#addAttribute}. */
export function useAddDesignAttribute(id?: string): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string, string, string]>((d, k, v, def) => d.addAttribute(k, v, def))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttribute}. */
export function useRemoveDesignAttribute(id?: string): readonly [(attribute: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string]>((d, id) => d.removeAttribute(id))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttributes}. */
export function useRemoveDesignAttributes(id?: string): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [readonly string[]]>((d, ids) => d.removeAttributes(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#addFixedPiece}. */
export function useAddFixedPiece(id?: string): readonly [(blueprint: string, position: PositionInput, name?: string | null, description?: string | null) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string, PositionInput, string | null | undefined, string | null | undefined]>((d, bp, pos, n, desc) => d.addFixedPiece(bp, pos, n, desc))(() => entity);
}

/** @emoji ✍️ {@link Design#addChildPieceWithParentConnection}. */
export function useAddChildPieceWithParentConnection(
  id?: string,
): readonly [(blueprint: string, parentPiece: string, parentConnector: string, childConnector: string, name?: string | null, description?: string | null, position?: PositionInput | null, scale?: number | null) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string, string, string, string, string | null | undefined, string | null | undefined, PositionInput | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, n, desc, pos, sc) =>
    d.addChildPieceWithParentConnection(bp, pp, pc, cc, n, desc, pos, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#addHangingChildPieceWithParentConnection}. */
export function useAddHangingChildPieceWithParentConnection(
  id?: string,
): readonly [(blueprint: string, parentPiece: string, parentConnector: string, childConnector: string, position: PositionInput, name?: string | null, description?: string | null, scale?: number | null) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string, string, string, string, PositionInput, string | null | undefined, string | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, pos, n, desc, sc) =>
    d.addHangingChildPieceWithParentConnection(bp, pp, pc, cc, pos, n, desc, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiece}. */
export function useDeleteDesignPiece(id?: string): readonly [(piece: string) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [string]>((d, id) => d.deletePiece(id))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePieces}. */
export function useDeleteDesignPieces(id?: string): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [readonly string[]]>((d, ids) => d.deletePieces(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiecesAndConnections}. */
export function useDeleteDesignPiecesAndConnections(id?: string): readonly [(pieceIds: readonly string[], connectionIds: readonly string[]) => void, OperationStatus] {
  const entity = resolveDesign(id);
  return semioInternalOperationBind<Design, [readonly string[], readonly string[]]>((d, pieceIds, connectionIds) => d.deletePiecesAndConnections(pieceIds, connectionIds))(() => entity);
}

/**
 * @emoji ✍️ Legacy shallow {@link Design} patch runner: applies only keys backed by {@link DesignOperation} ({@code name}, {@code description}, {@code icon}, {@code flatten}); other keys are ignored with a {@code [DEBUG]} console warning.
 */
export function useUpdateDesign(): Readonly<{
  run: (id: string, patch: Record<string, unknown>) => void;
  status: OperationStatus;
}> {
  const store = useJsStore();
  const [run, status] = semioInternalOperationBind<Store, [string, Record<string, unknown>]>(async (st, id, patch) => {
    const d = st.design(id);
    for (const [key, raw] of Object.entries(patch)) {
      if (key === "name") {
        const r = await d.rename(String(raw ?? ""));
        if (!r.ok) return r;
      } else if (key === "description") {
        const r = await d.changeDescription(String(raw ?? ""));
        if (!r.ok) return r;
      } else if (key === "icon") {
        const r = await d.changeIcon(String(raw ?? ""));
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
  const runDesign = React.useCallback((id: string, patch: Record<string, unknown>) => run(id, patch), [run]);
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
const useTypeAddConnectorOperation = semioInternalOperationBind<Type, [string, string | null | undefined, string | null | undefined, string | null | undefined]>((t, code, description, icon, id) =>
  t.addConnector(code, description ?? null, icon ?? null, id ?? null),
);
const useTypeRemoveConnectorOperation = semioInternalOperationBind<Type, [string]>((t, id) => t.removeConnector(id));
const useTypeRemoveConnectorsOperation = semioInternalOperationBind<Type, [readonly string[]]>((t, ids) => t.removeConnectors(ids));

/** @emoji 📖 Live {@link Type#name}. */
export function useTypeName(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖 Live {@link Type#description}. */
export function useTypeDescription(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖 Live {@link Type#icon}. */
export function useTypeIcon(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖 Live {@link Type#image}. */
export function useTypeImage(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.image());
}

/** @emoji 📖 Live {@link Type#unit}. */
export function useTypeUnit(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.unit());
}

/** @emoji 📖 Connector ids for {@link Type#connectors}. */
export function useTypeConnectors(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.connectors()).map((c) => c.id)));
}

/** @emoji 📖 Representation ids for {@link Type#representations}. */
export function useTypeRepresentations(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.representations()).map((r) => r.id)));
}

/** @emoji 📖 Live {@link Type#attributes}. */
export function useTypeAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji 📖 Author ids for {@link Type#authors}. */
export function useTypeAuthors(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.authors()).map((a) => a.id)));
}

/** @emoji ✍️ {@link TypeOperationInput#rename}. */
export function useRenameType(): readonly [(newName: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeRenameOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeDescription}. */
export function useChangeTypeDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeIcon}. */
export function useChangeTypeIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addAttribute}. */
export function useAddTypeAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttribute}. */
export function useRemoveTypeAttribute(): readonly [(id: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttributes}. */
export function useRemoveTypeAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveAttributesOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#createPort}. */
export function useCreatePort(): readonly [(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) => void, OperationStatus] {
  const e = resolveType();
  return useTypeCreatePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePort}. */
export function useDeletePort(): readonly [(id: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeDeletePortOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePorts}. */
export function useDeletePorts(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const e = resolveType();
  return useTypeDeletePortsOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addConnector}. */
export function useAddConnector(): readonly [(code: string, description?: string | null, icon?: string | null, id?: string | null) => void, OperationStatus] {
  const e = resolveType();
  return useTypeAddConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnector}. */
export function useRemoveConnector(): readonly [(id: string) => void, OperationStatus] {
  const e = resolveType();
  return useTypeRemoveConnectorOperation(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnectors}. */
export function useRemoveConnectors(): readonly [(ids: readonly string[]) => void, OperationStatus] {
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
export function usePortCode(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.code());
}

/** @emoji 📖 Live {@link Port#label}. */
export function usePortLabel(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.label());
}

/** @emoji 📖 Live {@link Port#order}. */
export function usePortOrder(): number | null | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.order());
}

/** @emoji 📖 Live {@link Port#name}. */
export function usePortName(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖 Live {@link Port#description}. */
export function usePortDescription(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖 Live {@link Port#icon}. */
export function usePortIcon(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖 Bulky {@link Port#attributes}. */
export function usePortAttributes(): readonly Attribute[] | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji ✍️ {@link PortOperationInput#rename}. */
export function useRenamePort(): readonly [(newCode: string, newLabel?: string | null) => void, OperationStatus] {
  const e = resolvePort();
  return usePortRenameOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeDescription}. */
export function useChangePortDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolvePort();
  return usePortChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeIcon}. */
export function useChangePortIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolvePort();
  return usePortChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#addAttribute}. */
export function useAddPortAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolvePort();
  return usePortAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttribute}. */
export function useRemovePortAttribute(): readonly [(id: string) => void, OperationStatus] {
  const e = resolvePort();
  return usePortRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttributes}. */
export function useRemovePortAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const e = resolvePort();
  return usePortRemoveAttributesOperation(() => e);
}
// #endregion 🔘Port

// #region 🔗Connector
const useConnectorRenameOperation = semioInternalOperationBind<Connector, [string]>((c, newCode) => c.rename(newCode));
const useConnectorChangeDescriptionOperation = semioInternalOperationBind<Connector, [string]>((c, d) => c.changeDescription(d));
const useConnectorChangeIconOperation = semioInternalOperationBind<Connector, [string]>((c, i) => c.changeIcon(i));

/** @emoji 📖 Live {@link Connector#code}. */
export function useConnectorCode(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.code());
}

/** @emoji 📖 Live {@link Connector#description}. */
export function useConnectorDescription(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Connector#icon}. */
export function useConnectorIcon(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Linked port id for {@link Connector#port}. */
export function useConnectorPort(id?: string): string | null | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, async (c) => {
    const port = await c.port();
    return port?.id ?? null;
  });
}

/** @emoji 📖 Bulky {@link Connector#attributes}. */
export function useConnectorAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link ConnectorOperationInput#rename}. */
export function useRenameConnector(): readonly [(newCode: string) => void, OperationStatus] {
  const e = resolveConnector();
  return useConnectorRenameOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeDescription}. */
export function useChangeConnectorDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolveConnector();
  return useConnectorChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeIcon}. */
export function useChangeConnectorIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolveConnector();
  return useConnectorChangeIconOperation(() => e);
}
// #endregion 🔗Connector

// #region ✍️Author
/** @emoji 📖 Live {@link Author#name}. */
export function useAuthorName(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.name());
}

/** @emoji 📖 Live {@link Author#email}. */
export function useAuthorEmail(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.email());
}

/** @emoji 📖 Live {@link Author#rank}. */
export function useAuthorRank(id?: string): number | null | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.rank());
}

/** @emoji 📖 Live {@link Author#description}. */
export function useAuthorDescription(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.description());
}

/** @emoji 📖 Live {@link Author#icon}. */
export function useAuthorIcon(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.icon());
}

/** @emoji 📖 Live {@link Author#role}. */
export function useAuthorRole(id?: string): string | undefined {
  const entity = resolveAuthor(id);
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
export function useQualityKey(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.key());
}

/** @emoji 📖 Live {@link Quality#value}. */
export function useQualityValue(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.value());
}

/** @emoji 📖 Live {@link Quality#unit}. */
export function useQualityUnit(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.unit());
}

/** @emoji 📖 Live {@link Quality#definition}. */
export function useQualityDefinition(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.definition());
}

/** @emoji 📖 Live {@link Quality#description}. */
export function useQualityDescription(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.description());
}

/** @emoji 📖 Live {@link Quality#icon}. */
export function useQualityIcon(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.icon());
}

/** @emoji 📖 Live {@link Quality#attributes}. */
export function useQualityAttributes(): readonly Attribute[] | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.attributes());
}

/** @emoji 📖 Live {@link Quality#benchmarks} as ids. */
export function useQualityBenchmarks(): readonly string[] | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, async (q) => Object.freeze((await q.benchmarks()).map((b) => b.id)));
}

/** @emoji ✍️ {@link Quality#rename}. */
export function useRenameQuality(): readonly [(newKey: string) => void, OperationStatus] {
  const e = resolveQuality();
  return useQualityRenameOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeDescription}. */
export function useChangeQualityDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolveQuality();
  return useQualityChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Quality#changeIcon}. */
export function useChangeQualityIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolveQuality();
  return useQualityChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Quality#addAttribute}. */
export function useAddQualityAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolveQuality();
  return useQualityAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttribute}. */
export function useRemoveQualityAttribute(): readonly [(attribute: string) => void, OperationStatus] {
  const e = resolveQuality();
  return useQualityRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttributes}. */
export function useRemoveQualityAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
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
export function useTagName(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.name());
}

/** @emoji 📖 Live {@link Tag#description}. */
export function useTagDescription(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.description());
}

/** @emoji 📖 Live {@link Tag#icon}. */
export function useTagIcon(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.icon());
}

/** @emoji 📖 Live {@link Tag#order}. */
export function useTagOrder(): number | null | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.order());
}

/** @emoji 📖 Live {@link Tag#attributes}. */
export function useTagAttributes(): readonly Attribute[] | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.attributes());
}

/** @emoji ✍️ {@link Tag#rename}. */
export function useRenameTag(): readonly [(newName: string) => void, OperationStatus] {
  const e = resolveTag();
  return useTagRenameOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeDescription}. */
export function useChangeTagDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolveTag();
  return useTagChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Tag#changeIcon}. */
export function useChangeTagIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolveTag();
  return useTagChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Tag#addAttribute}. */
export function useAddTagAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolveTag();
  return useTagAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttribute}. */
export function useRemoveTagAttribute(): readonly [(attribute: string) => void, OperationStatus] {
  const e = resolveTag();
  return useTagRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttributes}. */
export function useRemoveTagAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
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
export function useConceptName(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖 Live {@link Concept#description}. */
export function useConceptDescription(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Concept#icon}. */
export function useConceptIcon(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Live {@link Concept#order}. */
export function useConceptOrder(): number | null | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.order());
}

/** @emoji 📖 Live {@link Concept#attributes}. */
export function useConceptAttributes(): readonly Attribute[] | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.attributes());
}

/** @emoji ✍️ {@link Concept#rename}. */
export function useRenameConcept(): readonly [(newName: string) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptRenameOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeDescription}. */
export function useChangeConceptDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Concept#changeIcon}. */
export function useChangeConceptIcon(): readonly [(newIcon: string) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptChangeIconOperation(() => e);
}

/** @emoji ✍️ {@link Concept#addAttribute}. */
export function useAddConceptAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttribute}. */
export function useRemoveConceptAttribute(): readonly [(attribute: string) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttributes}. */
export function useRemoveConceptAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const e = resolveConcept();
  return useConceptRemoveAttributesOperation(() => e);
}
// #endregion 💡Concept

// #region 🎨Representation
/** @emoji 📖 Live {@link Representation#url}. */
export function useRepresentationUrl(id?: string): string | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, (r) => r.url());
}

/** @emoji 📖 Live {@link Representation#description}. */
export function useRepresentationDescription(id?: string): string | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, (r) => r.description());
}

/** @emoji 📖 Tag ids for {@link Representation#tags}. */
export function useRepresentationTags(id?: string): readonly string[] | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, async (r) => Object.freeze((await r.tags()).map((t) => t.id)));
}

/** @emoji 📖 Quality ids for {@link Representation#qualities}. */
export function useRepresentationQualities(id?: string): readonly string[] | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, async (r) => Object.freeze((await r.qualities()).map((q) => q.id)));
}

/** @emoji 📖 Live {@link Representation#attributes}. */
export function useRepresentationAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, (r) => r.attributes());
}

/** @emoji 📖 Linked file id for {@link Representation#file}. */
export function useRepresentationFile(id?: string): string | null | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, async (r) => {
    const f = await r.file();
    return f?.id ?? null;
  });
}

/** @emoji 📖 Live {@link File#name}. */
export function useFileName(id?: string): string | undefined {
  const entity = resolveFile(id);
  return useCurrentEntityField(entity, (f) => f.name());
}
// #endregion 🎨Representation

// #region 🧩Piece
/** @emoji 📖 Live {@link Piece#name}. */
export function usePieceName(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.name());
}

/** @emoji 📖 Live {@link Piece#description}. */
export function usePieceDescription(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.description());
}

/** @emoji 📖 Live {@link Piece#icon}. */
export function usePieceIcon(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.icon());
}

/** @emoji 📖 Kit piece {@code type { id }} reference. */
export function usePieceTypeId(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => (await p.typeId()) ?? undefined);
}

/** @emoji 📖 Live {@link Piece#scale}. */
export function usePieceScale(id?: string): number | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.scale());
}

/** @emoji 📖 Live {@link Piece#position}. */
export function usePiecePosition(id?: string): Position | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.position());
}

/** @emoji 📖 Live {@link Piece#flatPosition}. */
export function usePieceFlatPosition(id?: string): Position | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.flatPosition());
}

/** @emoji 📖 Live {@link Piece#plane}. */
export function usePiecePlane(id?: string): Plane | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.position().plane());
}

/** @emoji 📖 Live {@link Piece#center}. */
export function usePieceCenter(id?: string): Coordinate | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.position().center());
}

/** @emoji 📖 Live {@link Piece#flatPlane}. */
export function usePieceFlatPlane(id?: string): Plane | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.flatPosition().plane());
}

/** @emoji 📖 Live {@link Piece#flatCenter}. */
export function usePieceFlatCenter(id?: string): Coordinate | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => p.flatPosition().center());
}

/** @emoji 📖 Live {@link Piece#blueprint}. */
export function usePieceBlueprint(id?: string): PieceBlueprint | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.blueprint());
}

/** @emoji 📖 Live {@link Piece#attributes}. */
export function usePieceAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.attributes());
}

/** @emoji 📖 Live {@link Piece#connectionKind}. */
export function usePieceConnectionKind(id?: string): "FIXED" | "CONNECTED" | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.connectionKind());
}

/** @emoji 📖 Parent piece id for {@link Piece#parentPiece}. */
export function usePieceParentPiece(id?: string): string | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => {
    const parent = await p.parentPiece();
    return parent?.id ?? null;
  });
}

/** @emoji 📖 Parent connection id for {@link Piece#parentConnection}. */
export function usePieceParentConnection(id?: string): string | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => {
    const pc = await p.parentConnection();
    return pc?.id ?? null;
  });
}

/** @emoji 📖 Child piece ids for {@link Piece#childPieces}. */
export function usePieceChildPieces(id?: string): readonly string[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => Object.freeze((await p.childPieces()).map((c) => c.id)));
}

/** @emoji 📖 Child connection ids for {@link Piece#childConnections}. */
export function usePieceChildConnections(id?: string): readonly string[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => Object.freeze((await p.childConnections()).map((c) => c.id)));
}

/** @emoji 📖 Live {@link Piece#depth}. */
export function usePieceDepth(id?: string): number | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.depth());
}

/** @emoji 📖 Live {@link Piece#path} as ordered piece node keys (assembly chain). */
export function usePiecePathPieces(id?: string): readonly string[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.pathPieces());
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
export function useRenamePiece(): readonly [(newName: string) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceRenameOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeDescription}. */
export function useChangePieceDescription(): readonly [(newDescription: string) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceChangeDescriptionOperation(() => e);
}

/** @emoji ✍️ {@link Piece#drag}. */
export function useDragPiece(): readonly [(offset: OffsetInput) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceDragOperation(() => e);
}

/** @emoji ✍️ {@link Piece#move}. */
export function useMovePiece(): readonly [(position: PositionInput) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceMoveOperation(() => e);
}

/** @emoji ✍️ {@link Piece#fix}. */
export function useFixPiece(): readonly [() => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceFixOperation(() => e);
}

/** @emoji ✍️ {@link Piece#changeBlueprint}. */
export function useChangePieceBlueprint(): readonly [(blueprint: string) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceChangeBlueprintOperation(() => e);
}

/** @emoji ✍️ {@link Piece#addAttribute}. */
export function useAddPieceAttribute(): readonly [(key: string, value: string, definition: string) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceAddAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttribute}. */
export function useRemovePieceAttribute(): readonly [(attribute: string) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceRemoveAttributeOperation(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttributes}. */
export function useRemovePieceAttributes(): readonly [(ids: readonly string[]) => void, OperationStatus] {
  const e = resolvePiece();
  return usePieceRemoveAttributesOperation(() => e);
}
// #endregion 🧩Piece

// #region 🪢Pieces
/**
 * @emoji 🪝 Binds {@link PiecesOperation} batch mutations (not an {@link Entity} — no cached kit state on the handle).
 * @typeParam Args — forwarded to the underlying {@link PiecesOperation} method after the ops handle.
 */
function semioInternalPiecesOperationBind<Args extends unknown[]>(
  impl: (ops: PiecesOperation, ...args: Args) => SetResult | Promise<SetResult> | void | Promise<void>,
): (getOps: () => PiecesOperation | null) => readonly [(...args: Args) => void, OperationStatus] {
  return function usePiecesBatchOp(getOps: () => PiecesOperation | null): readonly [(...args: Args) => void, OperationStatus] {
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
          const raw = (await impl(ops, ...args)) as SetResult | void | undefined;
          const result: SetResult = raw === undefined ? ({ ok: true } as const) : (raw as SetResult);
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

const usePiecesDragOperation = semioInternalPiecesOperationBind((ops, o: OffsetInput) => ops.drag(o));
const usePiecesMoveOperation = semioInternalPiecesOperationBind((ops, o: OffsetInput) => ops.move(o));
const usePiecesFixOperation = semioInternalPiecesOperationBind((ops) => ops.fix());
const usePiecesChangeBlueprintOperation = semioInternalPiecesOperationBind((ops, id: string) => ops.changeBlueprint(id));

/** @emoji ✍️ {@link PiecesOperation#drag} using {@link PiecesBatchContext} + {@link DesignContext}. */
export function useDragPieces(): readonly [(offset: OffsetInput) => void, OperationStatus] {
  const store = useJsStore();
  const id = React.useContext(DesignContext);
  const batch = React.useContext(PiecesBatchContext);
  const ids = batch?.ids ?? [];
  const getOps = React.useCallback(() => (id == null || ids.length === 0 ? null : new PiecesOperation(store.session, id, ids, store.id)), [store.session, store.id, id, ids]);
  return usePiecesDragOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#move}. */
export function useMovePieces(): readonly [(offset: OffsetInput) => void, OperationStatus] {
  const store = useJsStore();
  const id = React.useContext(DesignContext);
  const batch = React.useContext(PiecesBatchContext);
  const ids = batch?.ids ?? [];
  const getOps = React.useCallback(() => (id == null || ids.length === 0 ? null : new PiecesOperation(store.session, id, ids, store.id)), [store.session, store.id, id, ids]);
  return usePiecesMoveOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#fix}. */
export function useFixPieces(): readonly [() => void, OperationStatus] {
  const store = useJsStore();
  const id = React.useContext(DesignContext);
  const batch = React.useContext(PiecesBatchContext);
  const ids = batch?.ids ?? [];
  const getOps = React.useCallback(() => (id == null || ids.length === 0 ? null : new PiecesOperation(store.session, id, ids, store.id)), [store.session, store.id, id, ids]);
  return usePiecesFixOperation(getOps);
}

/** @emoji ✍️ {@link PiecesOperation#changeBlueprint}. */
export function useChangePiecesBlueprint(): readonly [(blueprint: string) => void, OperationStatus] {
  const store = useJsStore();
  const id = React.useContext(DesignContext);
  const batch = React.useContext(PiecesBatchContext);
  const ids = batch?.ids ?? [];
  const getOps = React.useCallback(() => (id == null || ids.length === 0 ? null : new PiecesOperation(store.session, id, ids, store.id)), [store.session, store.id, id, ids]);
  return usePiecesChangeBlueprintOperation(getOps);
}
// #endregion 🪢Pieces

// #region ⛓️Connection
/** @emoji 📖 Live {@link Connection#gap}. */
export function useConnectionGap(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.gap());
}

/** @emoji 📖 Live {@link Connection#shift}. */
export function useConnectionShift(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.shift());
}

/** @emoji 📖 Live {@link Connection#rise}. */
export function useConnectionRise(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rise());
}

/** @emoji 📖 Live {@link Connection#rotation}. */
export function useConnectionRotation(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rotation());
}

/** @emoji 📖 Live {@link Connection#turn}. */
export function useConnectionTurn(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.turn());
}

/** @emoji 📖 Live {@link Connection#tilt}. */
export function useConnectionTilt(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.tilt());
}

/** @emoji 📖 Live {@link Connection#u}. */
export function useConnectionU(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.u());
}

/** @emoji 📖 Live {@link Connection#v}. */
export function useConnectionV(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.v());
}

/** @emoji 📖 Live {@link Connection#connected}. */
export function useConnectionConnected(): Side | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.connected());
}

/** @emoji 📖 Live {@link Connection#connecting}. */
export function useConnectionConnecting(): Side | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.connecting());
}

/** @emoji 📖 Live {@link Connection#name}. */
export function useConnectionName(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.name());
}

/** @emoji 📖 Live {@link Connection#description}. */
export function useConnectionDescription(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.description());
}

/** @emoji 📖 Live {@link Connection#icon}. */
export function useConnectionIcon(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.icon());
}

/** @emoji 📖 Live {@link Connection#attributes}. */
export function useConnectionAttributes(): readonly Attribute[] | undefined {
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
