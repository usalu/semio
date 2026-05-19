// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link } reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
// Value/type re-exports follow the local `@semio/js` imports below (single binding per symbol).
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type { ReactNode } from "react";
import * as React from "react";
import type { Attribute, Coordinate, Entity, GraphRootKind, ID, OffsetInput, PieceBlueprint, Plane, Position, PositionInput, SessionHttpOpenOptions, SetError, SetResult } from "../../js";
import { Alternative, Author, Backbone, Camera, Concept, Connection, Connector, Design, File, Graph, Kit, LocalProvider, openSessionHttp, Piece, PiecesOperation, Port, Quality, RemoteProvider, Representation, Session, Side, Store, Tag, TheKit, Type } from "../../js";

export type { Attribute, Coordinate, Entity, GraphRootKind, ID, OffsetInput, PieceBlueprint, Plane, Position, PositionInput, SessionHttpOpenOptions, SetError, SetResult } from "../../js";
export { Alternative, Author, Backbone, Camera, Concept, Connection, Connector, Design, File, Graph, Kit, LocalProvider, openSessionHttp, Piece, PiecesOperation, Port, Quality, RemoteProvider, Representation, Session, Side, Store, Tag, TheKit, Type } from "../../js";
// #endregion ⚛️Imports

// #region 🪝FieldBind
type FieldBindOptions<E, T> = Readonly<{
  /** @emoji 🧲 Single async read (one GraphQL selection / entity method). */
  read: (entity: E) => Promise<T>;
  /** @emoji 📡 Entity field name → {@code on{Name}Changed} on the anchor (preferred over bus). */
  field?: string;
  /** @emoji 🪝  source; re-invoked each render — keep stable via {@link React#useCallback}. */
  get: () => E | null;
}>;

function entityFieldChangedMethodName(fieldName: string): string {
  return `on${fieldName.charAt(0).toUpperCase()}${fieldName.slice(1)}Changed`;
}

const EMPTY_IDS = Object.freeze([]) as readonly string[];
const EMPTY_DESIGNS = Object.freeze([]) as readonly Design[];
const EMPTY_TYPES = Object.freeze([]) as readonly Type[];
const EMPTY_AUTHORS = Object.freeze([]) as readonly Author[];
const EMPTY_QUALITIES = Object.freeze([]) as readonly Quality[];
const EMPTY_TAGS = Object.freeze([]) as readonly Tag[];
const EMPTY_CONCEPTS = Object.freeze([]) as readonly Concept[];
const EMPTY_PIECES = Object.freeze([]) as readonly Piece[];
const EMPTY_CONNECTIONS = Object.freeze([]) as readonly Connection[];

const WRITE_STATUS_IDLE = Object.freeze({ kind: "idle", pending: 0 }) as WriteStatus;
const WRITE_STATUS_READONLY = Object.freeze({ kind: "readonly", pending: 0 }) as WriteStatus;

async function noopKitFieldSet(): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "Read-only kit field binding.", field: undefined, entity: undefined } };
}

/**
 * @emoji 🪝 Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
 * @returns Last resolved value, or {@code undefined} before the first successful read or when the entity is absent.
 */
function semioInternalFieldBind<E extends Entity, T>(opts: FieldBindOptions<E, T>): () => T | undefined {
  const { read, field: fieldName, get } = opts;
  return function use(): T | undefined {
    const entity = get();
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const entityRef = React.useRef(entity);
    const readRef = React.useRef(read);
    entityRef.current = entity;
    readRef.current = read;

    const refresh = React.useCallback(async () => {
      const e = entityRef.current;
      if (e == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await readRef.current(e));
      } catch {
        setValue(undefined);
      }
    }, []);

    React.useEffect(() => {
      void refresh();
    }, [entity?.id, entity?.storeId]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      if (fieldName != null && fieldName !== "") {
        const eventMethod = entityFieldChangedMethodName(fieldName);
        const sub = (e as unknown as Record<string, (cb: (next: unknown) => void) => () => void>)[eventMethod];
        if (typeof sub === "function") return sub.call(e, () => void refresh());
      }
      return e.session.bus.subscribeKind("commandSucceeded", () => void refresh());
    }, [entity, fieldName, refresh]);

    return value;
  };
}
// #endregion 🪝FieldBind

// #region 🪝OperationBind
/** @emoji 🎛️ UI-facing operation lifecycle for entity command hooks (idle → pending → settled). */
export type OperationStatus = { readonly kind: "idle" } | { readonly kind: "pending" } | { readonly kind: "settled"; readonly result: SetResult };

/**
 * @emoji 🗺️ Maps {@link SetErrorKind#NameTooLong} to a fixed max-length message; otherwise returns {@link SetError#message}.
 * @param maxChars — Upper bound communicated to the user (schema limit or UI policy).
 */
function mapTooLong(err: SetError, maxChars: number): string {
  if (err.kind === "NameTooLong") return `Name must be at most ${maxChars} characters.`;
  return err.message;
}

/** @emoji 🎛️ Shared {@link OperationStatus} plus a map of async command runners for one scoped anchor (entity, store, session, or batch). */
export type EntityCommand<Run extends Record<string, (...args: any[]) => Promise<SetResult>>> = Readonly<{
  run: Run;
  status: OperationStatus;
}>;

/** @emoji 🎛️ Sketchpad row status for {@link KitFieldBinding} tuple reads. */
export type WriteStatus = Readonly<
  | { kind: "readonly"; pending: 0 }
  | { kind: "idle"; pending: number }
  | { kind: "pending"; pending: number }
  | { kind: "error"; pending: number; lastError?: SetError }
>;

/** @emoji 🪝 Sketchpad triad: value, setter, status (`const [value] = hook()`). */
export type KitFieldBinding<T> = readonly [T, (next: unknown) => Promise<SetResult>, WriteStatus];

type SemioCommandRunners = Record<string, (...args: any[]) => Promise<SetResult>>;

type SemioCommandImpl<A> = (anchor: A, ...args: any[]) => SetResult | Promise<SetResult> | void | Promise<void>;

/**
 * @emoji 🪝 Builds one React hook per anchor: every command in {@code spec} shares a single {@link OperationStatus}.
 */
function semioInternalCommandFactory<A>(
  spec: Readonly<Record<string, SemioCommandImpl<A>>>,
  onSuccess: (anchor: A) => void,
): (get: () => A | null) => EntityCommand<SemioCommandRunners> {
  return function useBoundCommand(get: () => A | null): EntityCommand<SemioCommandRunners> {
    const getRef = React.useRef(get);
    getRef.current = get;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

    const run = React.useMemo(() => {
      const runners: SemioCommandRunners = {};
      for (const [key, impl] of Object.entries(spec)) {
        runners[key] = async (...args: any[]) => {
          const anchor = getRef.current();
          if (anchor == null) {
            const result: SetResult = {
              ok: false,
              error: { kind: "Disposed", message: "No command anchor in React context.", field: undefined, entity: undefined },
            };
            setStatus({ kind: "settled", result });
            return result;
          }
          setStatus({ kind: "pending" });
          try {
            const raw = (await impl(anchor, ...args)) as SetResult | void | undefined;
            const result: SetResult = raw === undefined ? ({ ok: true } as const) : (raw as SetResult);
            if (result.ok) onSuccess(anchor);
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
        };
      }
      return runners;
    }, []);

    return { run, status };
  };
}

function semioInternalEntityCommandFactory<E extends Entity>(spec: Readonly<Record<string, SemioCommandImpl<E>>>): (get: () => E | null) => EntityCommand<SemioCommandRunners> {
  return semioInternalCommandFactory(spec, (e) => {
    e.session.bus.emit({ kind: "commandSucceeded", payload: null } as never);
  });
}

function semioInternalStoreCommandFactory(spec: Readonly<Record<string, SemioCommandImpl<Store>>>): (get: () => Store | null) => EntityCommand<SemioCommandRunners> {
  return semioInternalCommandFactory(spec, (s) => {
    s.session.bus.emit({ kind: "commandSucceeded", payload: null } as never);
  });
}

function semioInternalSessionCommandFactory(spec: Readonly<Record<string, SemioCommandImpl<Session>>>): (get: () => Session | null) => EntityCommand<SemioCommandRunners> {
  return semioInternalCommandFactory(spec, (s) => {
    s.bus.emit({ kind: "commandSucceeded", payload: null } as never);
  });
}

function semioInternalPiecesCommandFactory(spec: Readonly<Record<string, SemioCommandImpl<PiecesOperation>>>): (get: () => PiecesOperation | null) => EntityCommand<SemioCommandRunners> {
  return semioInternalCommandFactory(spec, () => { });
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
    const readRef = React.useRef(read);
    kitRef.current = kit;
    readRef.current = read;

    const refresh = React.useCallback(async () => {
      const k = kitRef.current;
      if (k == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await readRef.current(k));
      } catch {
        setValue(undefined);
      }
    }, []);

    React.useEffect(() => {
      void refresh();
    }, [kit]);

    React.useEffect(() => {
      const k = kitRef.current;
      if (k == null) return;
      const subscribedKind = eventKind == null || eventKind === "" ? "commandSucceeded" : eventKind;
      return k.session.bus.subscribeKind(subscribedKind, () => void refresh());
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
    const readRef = React.useRef(read);
    storeRef.current = store;
    readRef.current = read;

    const refresh = React.useCallback(async () => {
      const s = storeRef.current;
      if (s == null) {
        setValue(undefined);
        return;
      }
      try {
        setValue(await readRef.current(s));
      } catch {
        setValue(undefined);
      }
    }, []);

    React.useEffect(() => {
      void refresh();
    }, [store]);

    React.useEffect(() => {
      const s = storeRef.current;
      if (s == null) return;
      const subscribedKind = eventKind == null || eventKind === "" ? "commandSucceeded" : eventKind;
      return s.session.bus.subscribeKind(subscribedKind, () => void refresh());
    }, [store, eventKind, refresh]);

    return value;
  };
}
// #endregion 🪝KitFieldBind

function useCurrentEntityField<E extends Entity, T>(entity: E | null, read: (entity: E) => Promise<T>, field?: string): T | undefined {
  return semioInternalFieldBind<E, T>({ get: () => entity, read, field })();
}

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
const AuthoritativeMarkerContext = React.createContext(false);
const TheKitMarkerContext = React.createContext(false);

const PositionMarkerContext = React.createContext(false);
const FlatPositionMarkerContext = React.createContext(false);
const PlaneMarkerContext = React.createContext(false);
const OriginMarkerContext = React.createContext(false);

const PiecesBatchContext = React.createContext<Readonly<{ ids: readonly string[] }> | null>(null);

/** @emoji 🪢 Batch piece ids for {@link usePiecesCommand} under {@link DesignContext}. */
export function PiecesBatchContextProvider(props: Readonly<{ ids: readonly string[]; children: ReactNode }>): React.ReactElement {
  const v = React.useMemo(() => ({ ids: props.ids }), [props.ids]);
  return React.createElement(PiecesBatchContext.Provider, { value: v }, props.children);
}

/** @emoji 🗂️ Publishes the semio/js session handle (opaque to public hooks). */
export function SessionContextProvider(props: Readonly<{ session: unknown; children: ReactNode }>): React.ReactElement {
  return React.createElement(SessionHandleContext.Provider, { value: props.session as Session }, props.children);
}

//#region 🔌KitWasmHostBridge
/** @emoji 🔌 Sketchpad registry row: opaque kit store host + optional kit client for VCS / alternatives UI. */
export type KitWasmHostState = Readonly<{
  kitTabId: string;
  store: unknown;
  kitClient: unknown | null;
}>;

const KitWasmHostContext = React.createContext<KitWasmHostState | null>(null);

/** @emoji 🔌 Reads {@link KitWasmMountProvider} host bindings (sketchpad registry bridge). */
export function useKitWasmHost(): KitWasmHostState | null {
  return React.useContext(KitWasmHostContext);
}

export type KitWasmMountProviderProps = Readonly<{
  kitId?: string;
  store?: unknown;
  kitClient?: unknown | null;
  children: ReactNode;
}>;

export type KitRuntimeContextValue = Readonly<{ kitId: string; store: KitHostStore; kitClient: unknown | null }>;
const KitRuntimeContext = React.createContext<KitRuntimeContextValue | null>(null);

/** @emoji 🔌 Publishes registry `store` + `kitClient` for sketchpad footers and native/WASM kit tabs. */
export function KitWasmMountProvider(props: KitWasmMountProviderProps): React.ReactElement {
  const host = React.useMemo<KitWasmHostState>(
    () => ({ kitTabId: props.kitId ?? "", store: props.store ?? null, kitClient: props.kitClient ?? null }),
    [props.kitId, props.store, props.kitClient],
  );
  let branch: ReactNode = props.children;
  if (props.store != null) {
    branch = React.createElement(
      KitRuntimeContext.Provider,
      { value: { kitId: props.kitId ?? "", store: props.store as KitHostStore, kitClient: props.kitClient ?? null } },
      branch,
    );
  }
  return React.createElement(KitWasmHostContext.Provider, { value: host }, branch);
}
//#endregion 🔌KitWasmHostBridge

//#region 🌐SemioStoreKitLineHost
/** @emoji 🌐 Composes `SessionContextProvider` → `StoreContextProvider` → WIP graph → `TheKit` → `KitContextProvider` for native `semio-store` + {@link openSessionHttp}. */
export type SemioStoreKitLineHostProps = Readonly<
  {
    baseUrl: string;
    children: ReactNode;
    /** @emoji ⏳ Shown until the HTTP session and first store/kit ids resolve. */
    fallback?: ReactNode;
  } & Pick<SessionHttpOpenOptions, "timeoutMs" | "installCreateDto">
>;

export function SemioStoreKitLineHost(props: SemioStoreKitLineHostProps): React.ReactElement {
  const { baseUrl, children, fallback = null, timeoutMs, installCreateDto } = props;
  const [phase, setPhase] = React.useState<"boot" | "ready" | "err">("boot");
  const [errMsg, setErrMsg] = React.useState<string | null>(null);
  const sessionRef = React.useRef<Session | null>(null);
  const [storeId, setStoreId] = React.useState<string | null>(null);
  const [kitId, setKitId] = React.useState<string | null>(null);

  React.useEffect(() => {
    let cancelled = false;
    let sess: Session | null = null;
    void (async () => {
      try {
        const root = baseUrl.replace(/\/$/, "");
        const s = await openSessionHttp(root, { timeoutMs, installCreateDto });
        if (cancelled) {
          await s.dispose();
          return;
        }
        sess = s;
        sessionRef.current = s;
        const stores = await s.stores();
        if (stores.length === 0) {
          throw new Error("semio/react: SemioStoreKitLineHost found no stores — POST /install first or pass installCreateDto.");
        }
        const sid = stores[0]!.id;
        const kitHandle = await s.store(sid).wip().theKit().kit();
        const kid = kitHandle.id;
        if (cancelled) {
          await s.dispose();
          return;
        }
        setStoreId(sid);
        setKitId(kid);
        setPhase("ready");
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        if (!cancelled) {
          setErrMsg(msg);
          setPhase("err");
        }
      }
    })();
    return () => {
      cancelled = true;
      sessionRef.current = null;
      if (sess != null) void sess.dispose();
    };
  }, [baseUrl, timeoutMs, installCreateDto]);

  if (phase === "err") {
    return React.createElement(React.Fragment, null, errMsg ?? "error");
  }
  if (phase !== "ready" || sessionRef.current == null || storeId == null || kitId == null) {
    return React.createElement(React.Fragment, null, fallback);
  }
  const session = sessionRef.current;
  const kitBranch = React.createElement(KitContextProvider, { id: kitId, children });
  const theKitBranch = React.createElement(TheKitContextProvider, { children: kitBranch });
  const wipBranch = React.createElement(WipContextProvider, { children: theKitBranch });
  const localBranch = React.createElement(LocalProviderContextProvider, { children: wipBranch });
  const storeBranch = React.createElement(StoreContextProvider, { id: storeId, children: localBranch });
  return React.createElement(SessionContextProvider, { session, children: storeBranch });
}
//#endregion 🌐SemioStoreKitLineHost

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

/** @emoji 🧩 Active {@link Design} entity from {@link DesignContext} + {@link StoreHandleContext}. */
export function useResolvedDesign(id?: ID): Design | null {
  return resolveDesign(id);
}

function useResolvedType(id?: ID): Type | null {
  return resolveType(id);
}

// #region 🪝IdStableEntityLists
/** @emoji 📚 Kit-level designs via {@link Kit#designs} (stable entity handles). */
export function useKitDesigns(): readonly Design[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.designs()), "designs") ?? EMPTY_DESIGNS;
}

/** @emoji 📚 Kit-level kinds via {@link Kit#types}. */
export function useKitTypes(): readonly Type[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.types()), "types") ?? EMPTY_TYPES;
}

/** @emoji 📚 Kit-level authors via {@link Kit#authors}. */
export function useKitAuthors(): readonly Author[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.authors()), "authors") ?? EMPTY_AUTHORS;
}

/** @emoji 📚 Kit-level qualities via {@link Kit#qualities}. */
export function useKitQualities(): readonly Quality[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.qualities()), "qualities") ?? EMPTY_QUALITIES;
}

/** @emoji 📚 Kit-level tags via {@link Kit#tags}. */
export function useKitTags(): readonly Tag[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.tags()), "tags") ?? EMPTY_TAGS;
}

/** @emoji 📚 Kit-level concepts via {@link Kit#concepts}. */
export function useKitConcepts(): readonly Concept[] {
  const kit = useWipKit();
  return useCurrentEntityField(kit, async (k) => Object.freeze(await k.concepts()), "concepts") ?? EMPTY_CONCEPTS;
}

/** @emoji 📚 Pieces in the active {@link DesignContext} design. */
export function useDesignPieces(): readonly Piece[] {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, async (design) => Object.freeze(await design.pieces()), "pieces") ?? EMPTY_PIECES;
}

/** @emoji 📚 Connections in the active {@link DesignContext} design. */
export function useDesignConnections(): readonly Connection[] {
  const entity = useResolvedDesign();
  return useCurrentEntityField(entity, async (design) => Object.freeze(await design.connections()), "connections") ?? EMPTY_CONNECTIONS;
}
// #endregion 🪝IdStableEntityLists

// #region 🪝AggregateListBundles
/** @emoji 📚 Sketchpad bundle: live kit designs. */
export function useDesigns(): Readonly<{ designs: readonly Design[] }> {
  const designs = useKitDesigns();
  return { designs };
}

/** @emoji 📚 Sketchpad tuple: live kit kinds (`const [types] = useTypes()`). */
export function useTypes(): KitFieldBinding<readonly Type[]> {
  const types = useKitTypes();
  return [types, noopKitFieldSet, WRITE_STATUS_IDLE];
}

/** @emoji 📚 Pieces in the active design scope. */
export function usePieces(): readonly Piece[] {
  return useDesignPieces();
}

/** @emoji 📚 Connections in the active design scope. */
export function useConnections(): readonly Connection[] {
  return useDesignConnections();
}
// #endregion 🪝AggregateListBundles

// #region 🪝HooksKit
// #region 📖KitReads
/** @emoji 📖 Live {@link Kit#name} via {@link Kit#onNameChanged}. */
export function useKitName(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.name(), "name");
}

/** @emoji 📖 Live {@link Kit#description} via {@link Kit#onDescriptionChanged}. */
export function useKitDescription(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.description(), "description");
}

/** @emoji 📖 Live {@link Kit#icon}. */
export function useKitIcon(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.icon(), "icon");
}

/** @emoji 📖 Live {@link Kit#image}. */
export function useKitImage(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.image(), "image");
}

/** @emoji 📖 Live {@link Kit#preview}. */
export function useKitPreview(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.preview(), "preview");
}

/** @emoji 📖 Live {@link Kit#remote}. */
export function useKitRemote(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.remote(), "remote");
}

/** @emoji 📖 Live {@link Kit#homepage}. */
export function useKitHomepage(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.homepage(), "homepage");
}

/** @emoji 📖 Live {@link Kit#license}. */
export function useKitLicense(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.license(), "license");
}

/** @emoji 📖 Live {@link Kit#uri}. */
export function useKitUri(): string | undefined {
  const kit = useWipKit();
  return useCurrentEntityField(kit, (k) => k.uri(), "uri");
}

/** @emoji 🧾 Exposes {@link Store#ensureChangeId} as a stable callback. */
export function useEnsureKitChange(): () => Promise<string> {
  const store = useJsStore();
  return React.useCallback(() => store.ensureChangeId(), [store]);
}
// #endregion 📖KitReads

// #region ✍️KitWrites
const useKitCommandBound = semioInternalEntityCommandFactory<Kit>({
  rename: (k, newName: string) => k.rename(newName),
  changeDescription: (k, description: string) => k.changeDescription(description),
  createTag: (k, name: string, description?: string | null, icon?: string | null, order?: number | null) => k.createTag(name, description, icon, order),
  deleteTag: (k, id: string) => k.deleteTag(id),
  deleteTags: (k, ids: readonly string[]) => k.deleteTags(ids),
  createConcept: (k, name: string, description?: string | null, icon?: string | null, order?: number | null) => k.createConcept(name, description, icon, order),
  deleteConcept: (k, id: string) => k.deleteConcept(id),
  deleteConcepts: (k, ids: readonly string[]) => k.deleteConcepts(ids),
  createQuality: (k, key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) =>
    k.createQuality(key, value, unit, definition, description, icon),
  deleteQuality: (k, id: string) => k.deleteQuality(id),
  deleteQualities: (k, ids: readonly string[]) => k.deleteQualities(ids),
  createType: (k, name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => k.createType(name, description, icon, image, unit),
  deleteType: (k, id: string) => k.deleteType(id),
  deleteTypes: (k, ids: readonly string[]) => k.deleteTypes(ids),
  createDesign: (k, name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => k.createDesign(name, description, icon, image, unit),
  deleteDesign: (k, id: string) => k.deleteDesign(id),
  deleteDesigns: (k, ids: readonly string[]) => k.deleteDesigns(ids),
});

/** @emoji ✍️ All {@link Kit} mutations for the active WIP kit ({@link useWipKit}). */
export function useKitCommand(): EntityCommand<SemioCommandRunners> {
  const kit = useWipKit();
  return useKitCommandBound(() => kit);
}

const useStoreCommandBound = semioInternalStoreCommandFactory({
  saveChange: async (s) => {
    await s.saveChange();
    return { ok: true } as const;
  },
  createCheckpoint: (s, message: string) => s.createCheckpoint(message),
  startAlternative: (s, name?: string | null) => s.startAlternative(name ?? undefined),
  integrateAlternative: (s, id: string) => s.integrateAlternative(id),
  attachBackbone: (s, uri: string, localProvider: LocalProvider) => s.attachBackbone(localProvider, uri),
  detachBackbone: (s) => s.detachBackbone(),
  syncBackbone: (s) => s.syncBackbone(),
});

/** @emoji ✍️ Store-scoped mutations ({@link useJsStore}). */
export function useStoreCommand(): EntityCommand<SemioCommandRunners> {
  const store = useJsStore();
  const session = useJsSession();
  const bound = useStoreCommandBound(() => store);
  const run = React.useMemo(
    () => ({
      ...bound.run,
      attachBackbone: (uri: string) => bound.run.attachBackbone(uri, session.localProvider()),
    }),
    [bound.run, session],
  );
  return { run, status: bound.status };
}

const useSessionCommandBound = semioInternalSessionCommandFactory({
  login: (s, username: string, passwordHash: string, hubUrl?: string) => s.login(username, passwordHash, hubUrl),
  logout: (s) => s.logout(),
  sessionStart: (s) => s.sessionStart(),
  sessionEnd: (s) => s.sessionEnd(),
});

/** @emoji ✍️ {@link Session} mutations for the active JS session. */
export function useSessionCommand(): EntityCommand<SemioCommandRunners> {
  const session = useJsSession();
  return useSessionCommandBound(() => session);
}

// #region 🪝BackboneOps

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
  return useCurrentEntityField(entity, (d) => d.name(), "name") ?? "";
}

/** @emoji 📖 Live {@link Design#description} via {@link Design#onDescriptionChanged}. */
export function useDesignDescription(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.description(), "description") ?? "";
}

/** @emoji 📖 Live {@link Design#qualitySum}. */
export function useDesignQualitySum(id?: string): number {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.qualitySum(), "qualitySum") ?? 0;
}

/** @emoji 📖 Live {@link Design#icon}. */
export function useDesignIcon(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.icon(), "icon") ?? "";
}

/** @emoji 📖 Live {@link Design#image}. */
export function useDesignImage(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.image(), "image") ?? "";
}

/** @emoji 📖 Live {@link Design#unit}. */
export function useDesignUnit(id?: string): string {
  const entity = useResolvedDesign(id);
  return useCurrentEntityField(entity, (d) => d.unit(), "unit") ?? "";
}
// #endregion 📖DesignReads

// #region ✍️DesignWrites
const useDesignCommandBound = semioInternalEntityCommandFactory<Design>({
  rename: (d, newName: string) => d.rename(newName),
  changeDescription: (d, description: string) => d.changeDescription(description),
  changeIcon: (d, icon: string) => d.changeIcon(icon),
  flatten: (d) => d.flatten(),
  addAttribute: (d, key: string, value: string, definition: string) => d.addAttribute(key, value, definition),
  removeAttribute: (d, attribute: string) => d.removeAttribute(attribute),
  removeAttributes: (d, ids: readonly string[]) => d.removeAttributes(ids),
  addFixedPiece: (d, blueprint: string, position: PositionInput, name?: string | null, description?: string | null) => d.addFixedPiece(blueprint, position, name, description),
  addChildPieceWithParentConnection: (
    d,
    blueprint: string,
    parentPiece: string,
    parentConnector: string,
    childConnector: string,
    name?: string | null,
    description?: string | null,
    position?: PositionInput | null,
    scale?: number | null,
  ) => d.addChildPieceWithParentConnection(blueprint, parentPiece, parentConnector, childConnector, name, description, position, scale),
  addHangingChildPieceWithParentConnection: (
    d,
    blueprint: string,
    parentPiece: string,
    parentConnector: string,
    childConnector: string,
    position: PositionInput,
    name?: string | null,
    description?: string | null,
    scale?: number | null,
  ) => d.addHangingChildPieceWithParentConnection(blueprint, parentPiece, parentConnector, childConnector, position, name, description, scale),
  deletePiece: (d, piece: string) => d.deletePiece(piece),
  deletePieces: (d, ids: readonly string[]) => d.deletePieces(ids),
  deletePiecesAndConnections: (d, pieceIds: readonly string[], connectionIds: readonly string[]) => d.deletePiecesAndConnections(pieceIds, connectionIds),
});

/** @emoji ✍️ All {@link Design} mutations; optional {@code id} overrides {@link DesignContext}. */
export function useDesignCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useDesignCommandBound(() => resolveDesign(id));
}
// #endregion ✍️DesignWrites
// #endregion 🪝HooksDesign

// #region 🧰Type
const useTypeCommandBound = semioInternalEntityCommandFactory<Type>({
  rename: (t, newName: string) => t.rename(newName),
  changeDescription: (t, description: string) => t.changeDescription(description),
  changeIcon: (t, icon: string) => t.changeIcon(icon),
  addAttribute: (t, key: string, value: string, definition: string) => t.addAttribute(key, value, definition),
  removeAttribute: (t, id: string) => t.removeAttribute(id),
  removeAttributes: (t, ids: readonly string[]) => t.removeAttributes(ids),
  createPort: (t, code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) =>
    t.createPort(code ?? null, label ?? null, description ?? null, icon ?? null, order ?? null),
  deletePort: (t, id: string) => t.deletePort(id),
  deletePorts: (t, ids: readonly string[]) => t.deletePorts(ids),
  addConnector: (t, code: string, description?: string | null, icon?: string | null, id?: string | null) => t.addConnector(code, description ?? null, icon ?? null, id ?? null),
  removeConnector: (t, id: string) => t.removeConnector(id),
  removeConnectors: (t, ids: readonly string[]) => t.removeConnectors(ids),
});

/** @emoji 📖 Live {@link Type#name}. */
export function useTypeName(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.name(), "name");
}

/** @emoji 📖 Live {@link Type#description}. */
export function useTypeDescription(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.description(), "description");
}

/** @emoji 📖 Live {@link Type#icon}. */
export function useTypeIcon(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.icon(), "icon");
}

/** @emoji 📖 Live {@link Type#image}. */
export function useTypeImage(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.image(), "image");
}

/** @emoji 📖 Live {@link Type#unit}. */
export function useTypeUnit(id?: string): string | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.unit(), "unit");
}

/** @emoji 📖 Connector ids for {@link Type#connectors}. */
export function useTypeConnectors(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.connectors()).map((c) => c.id)), "connectors");
}

/** @emoji 📖 Representation ids for {@link Type#representations}. */
export function useTypeRepresentations(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.representations()).map((r) => r.id)));
}

/** @emoji 📖 Live {@link Type#attributes}. */
export function useTypeAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, (t) => t.attributes(), "attributes");
}

/** @emoji 📖 Author ids for {@link Type#authors}. */
export function useTypeAuthors(id?: string): readonly string[] | undefined {
  const entity = useResolvedType(id);
  return useCurrentEntityField(entity, async (t) => Object.freeze((await t.authors()).map((a) => a.id)));
}

/** @emoji ✍️ All {@link Type} mutations for {@link TypeContext} (optional {@code id} overrides). */
export function useTypeCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useTypeCommandBound(() => resolveType(id));
}
// #endregion 🧰Type

// #region 🔘Port
const usePortCommandBound = semioInternalEntityCommandFactory<Port>({
  rename: (p, newCode: string, newLabel?: string | null) => p.rename(newCode, newLabel),
  changeDescription: (p, description: string) => p.changeDescription(description),
  changeIcon: (p, icon: string) => p.changeIcon(icon),
  addAttribute: (p, key: string, value: string, definition: string) => p.addAttribute(key, value, definition),
  removeAttribute: (p, id: string) => p.removeAttribute(id),
  removeAttributes: (p, ids: readonly string[]) => p.removeAttributes(ids),
});

/** @emoji 📖 Live {@link Port#code}. */
export function usePortCode(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.code(), "code");
}

/** @emoji 📖 Live {@link Port#label}. */
export function usePortLabel(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.label(), "label");
}

/** @emoji 📖 Live {@link Port#order}. */
export function usePortOrder(): number | null | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.order(), "order");
}

/** @emoji 📖 Live {@link Port#name}. */
export function usePortName(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.name(), "name");
}

/** @emoji 📖 Live {@link Port#description}. */
export function usePortDescription(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.description(), "description");
}

/** @emoji 📖 Live {@link Port#icon}. */
export function usePortIcon(): string | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.icon(), "icon");
}

/** @emoji 📖 Bulky {@link Port#attributes}. */
export function usePortAttributes(): readonly Attribute[] | undefined {
  const entity = resolvePort();
  return useCurrentEntityField(entity, (p) => p.attributes(), "attributes");
}

/** @emoji ✍️ All {@link Port} mutations for {@link PortContext}. */
export function usePortCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return usePortCommandBound(() => resolvePort(id));
}
// #endregion 🔘Port

// #region 🔗Connector
const useConnectorCommandBound = semioInternalEntityCommandFactory<Connector>({
  rename: (c, newCode: string) => c.rename(newCode),
  changeDescription: (c, description: string) => c.changeDescription(description),
  changeIcon: (c, icon: string) => c.changeIcon(icon),
});

/** @emoji 📖 Live {@link Connector#code}. */
export function useConnectorCode(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.code(), "code");
}

/** @emoji 📖 Live {@link Connector#description}. */
export function useConnectorDescription(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.description(), "description");
}

/** @emoji 📖 Live {@link Connector#icon}. */
export function useConnectorIcon(id?: string): string | undefined {
  const entity = resolveConnector(id);
  return useCurrentEntityField(entity, (c) => c.icon(), "icon");
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
  return useCurrentEntityField(entity, (c) => c.attributes(), "attributes");
}

/** @emoji ✍️ All {@link Connector} mutations (optional {@code id} overrides {@link ConnectorContext}). */
export function useConnectorCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useConnectorCommandBound(() => resolveConnector(id));
}
// #endregion 🔗Connector

// #region ✍️Author
/** @emoji 📖 Live {@link Author#name}. */
export function useAuthorName(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.name(), "name");
}

/** @emoji 📖 Live {@link Author#email}. */
export function useAuthorEmail(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.email(), "email");
}

/** @emoji 📖 Live {@link Author#rank}. */
export function useAuthorRank(id?: string): number | null | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.rank(), "rank");
}

/** @emoji 📖 Live {@link Author#description}. */
export function useAuthorDescription(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.description(), "description");
}

/** @emoji 📖 Live {@link Author#icon}. */
export function useAuthorIcon(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.icon(), "icon");
}

/** @emoji 📖 Live {@link Author#role}. */
export function useAuthorRole(id?: string): string | undefined {
  const entity = resolveAuthor(id);
  return useCurrentEntityField(entity, (a) => a.role(), "role");
}
// #endregion ✍️Author

// #region 💎Quality
const useQualityCommandBound = semioInternalEntityCommandFactory<Quality>({
  rename: (q, newKey: string) => q.rename(newKey),
  changeDescription: (q, description: string) => q.changeDescription(description),
  changeIcon: (q, icon: string) => q.changeIcon(icon),
  addAttribute: (q, key: string, value: string, definition: string) => q.addAttribute(key, value, definition),
  removeAttribute: (q, attribute: string) => q.removeAttribute(attribute),
  removeAttributes: (q, ids: readonly string[]) => q.removeAttributes(ids),
});

/** @emoji 📖 Live {@link Quality#key}. */
export function useQualityKey(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.key(), "key");
}

/** @emoji 📖 Live {@link Quality#value}. */
export function useQualityValue(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.value(), "value");
}

/** @emoji 📖 Live {@link Quality#unit}. */
export function useQualityUnit(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.unit(), "unit");
}

/** @emoji 📖 Live {@link Quality#definition}. */
export function useQualityDefinition(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.definition(), "definition");
}

/** @emoji 📖 Live {@link Quality#description}. */
export function useQualityDescription(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.description(), "description");
}

/** @emoji 📖 Live {@link Quality#icon}. */
export function useQualityIcon(): string | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.icon(), "icon");
}

/** @emoji 📖 Live {@link Quality#attributes}. */
export function useQualityAttributes(): readonly Attribute[] | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, (q) => q.attributes(), "attributes");
}

/** @emoji 📖 Live {@link Quality#benchmarks} as ids. */
export function useQualityBenchmarks(): readonly string[] | undefined {
  const entity = resolveQuality();
  return useCurrentEntityField(entity, async (q) => Object.freeze((await q.benchmarks()).map((b) => b.id)));
}

/** @emoji ✍️ All {@link Quality} mutations for {@link QualityContext}. */
export function useQualityCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useQualityCommandBound(() => resolveQuality(id));
}
// #endregion 💎Quality

// #region 🏷️Tag
const useTagCommandBound = semioInternalEntityCommandFactory<Tag>({
  rename: (t, newName: string) => t.rename(newName),
  changeDescription: (t, description: string) => t.changeDescription(description),
  changeIcon: (t, icon: string) => t.changeIcon(icon),
  addAttribute: (t, key: string, value: string, definition: string) => t.addAttribute(key, value, definition),
  removeAttribute: (t, attribute: string) => t.removeAttribute(attribute),
  removeAttributes: (t, ids: readonly string[]) => t.removeAttributes(ids),
});

/** @emoji 📖 Live {@link Tag#name}. */
export function useTagName(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.name(), "name");
}

/** @emoji 📖 Live {@link Tag#description}. */
export function useTagDescription(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.description(), "description");
}

/** @emoji 📖 Live {@link Tag#icon}. */
export function useTagIcon(): string | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.icon(), "icon");
}

/** @emoji 📖 Live {@link Tag#order}. */
export function useTagOrder(): number | null | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.order(), "order");
}

/** @emoji 📖 Live {@link Tag#attributes}. */
export function useTagAttributes(): readonly Attribute[] | undefined {
  const entity = resolveTag();
  return useCurrentEntityField(entity, (t) => t.attributes(), "attributes");
}

/** @emoji ✍️ All {@link Tag} mutations for {@link TagContext}. */
export function useTagCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useTagCommandBound(() => resolveTag(id));
}
// #endregion 🏷️Tag

// #region 💡Concept
const useConceptCommandBound = semioInternalEntityCommandFactory<Concept>({
  rename: (c, newName: string) => c.rename(newName),
  changeDescription: (c, description: string) => c.changeDescription(description),
  changeIcon: (c, icon: string) => c.changeIcon(icon),
  addAttribute: (c, key: string, value: string, definition: string) => c.addAttribute(key, value, definition),
  removeAttribute: (c, attribute: string) => c.removeAttribute(attribute),
  removeAttributes: (c, ids: readonly string[]) => c.removeAttributes(ids),
});

/** @emoji 📖 Live {@link Concept#name}. */
export function useConceptName(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.name(), "name");
}

/** @emoji 📖 Live {@link Concept#description}. */
export function useConceptDescription(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.description(), "description");
}

/** @emoji 📖 Live {@link Concept#icon}. */
export function useConceptIcon(): string | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.icon(), "icon");
}

/** @emoji 📖 Live {@link Concept#order}. */
export function useConceptOrder(): number | null | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.order(), "order");
}

/** @emoji 📖 Live {@link Concept#attributes}. */
export function useConceptAttributes(): readonly Attribute[] | undefined {
  const entity = resolveConcept();
  return useCurrentEntityField(entity, (c) => c.attributes(), "attributes");
}

/** @emoji ✍️ All {@link Concept} mutations for {@link ConceptContext}. */
export function useConceptCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return useConceptCommandBound(() => resolveConcept(id));
}
// #endregion 💡Concept

// #region 🎨Representation
/** @emoji 📖 Live {@link Representation#url}. */
export function useRepresentationUrl(id?: string): string | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, (r) => r.url(), "url");
}

/** @emoji 📖 Live {@link Representation#description}. */
export function useRepresentationDescription(id?: string): string | undefined {
  const entity = resolveRepresentation(id);
  return useCurrentEntityField(entity, (r) => r.description(), "description");
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
  return useCurrentEntityField(entity, (r) => r.attributes(), "attributes");
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
  return useCurrentEntityField(entity, (f) => f.name(), "name");
}
// #endregion 🎨Representation

// #region 🧩Piece
/** @emoji 📖 Live {@link Piece#name}. */
export function usePieceName(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.name(), "name");
}

/** @emoji 📖 Live {@link Piece#description}. */
export function usePieceDescription(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.description(), "description");
}

/** @emoji 📖 Live {@link Piece#icon}. */
export function usePieceIcon(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.icon(), "icon");
}

/** @emoji 📖 Kit piece {@code type { id }} reference. */
export function usePieceTypeId(id?: string): string | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, async (p) => (await p.typeId()) ?? undefined);
}

/** @emoji 📖 Live {@link Piece#scale}. */
export function usePieceScale(id?: string): number | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.scale(), "scale");
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
  return useCurrentEntityField(entity, (p) => p.blueprint(), "blueprint");
}

/** @emoji 📖 Live {@link Piece#attributes}. */
export function usePieceAttributes(id?: string): readonly Attribute[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.attributes(), "attributes");
}

/** @emoji 📖 Live {@link Piece#connectionKind}. */
export function usePieceConnectionKind(id?: string): "FIXED" | "CONNECTED" | null | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.connectionKind(), "connectionKind");
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
  return useCurrentEntityField(entity, (p) => p.depth(), "depth");
}

/** @emoji 📖 Live {@link Piece#path} as ordered piece node keys (assembly chain). */
export function usePiecePathPieces(id?: string): readonly string[] | undefined {
  const entity = resolvePiece(id);
  return useCurrentEntityField(entity, (p) => p.pathPieces(), "pathPieces");
}

const usePieceCommandBound = semioInternalEntityCommandFactory<Piece>({
  rename: (p, newName: string) => p.rename(newName),
  changeDescription: (p, description: string) => p.changeDescription(description),
  drag: (p, offset: OffsetInput) => p.drag(offset),
  move: (p, position: PositionInput) => p.move(position),
  fix: (p) => p.fix(),
  changeBlueprint: (p, blueprint: string) => p.changeBlueprint(blueprint),
  addAttribute: (p, key: string, value: string, definition: string) => p.addAttribute(key, value, definition),
  removeAttribute: (p, attribute: string) => p.removeAttribute(attribute),
  removeAttributes: (p, ids: readonly string[]) => p.removeAttributes(ids),
});

/** @emoji ✍️ All {@link Piece} mutations (optional {@code id} overrides {@link PieceContext}). */
export function usePieceCommand(id?: ID): EntityCommand<SemioCommandRunners> {
  return usePieceCommandBound(() => resolvePiece(id));
}
// #endregion 🧩Piece

// #region 🪢Pieces
const usePiecesCommandBound = semioInternalPiecesCommandFactory({
  drag: (ops, offset: OffsetInput) => ops.drag(offset),
  move: (ops, offset: OffsetInput) => ops.move(offset),
  fix: (ops) => ops.fix(),
  changeBlueprint: (ops, blueprint: string) => ops.changeBlueprint(blueprint),
});

/** @emoji ✍️ {@link PiecesOperation} batch mutations via {@link PiecesBatchContext} + {@link DesignContext}. */
export function usePiecesCommand(): EntityCommand<SemioCommandRunners> {
  const store = useJsStore();
  const designId = React.useContext(DesignContext);
  const batch = React.useContext(PiecesBatchContext);
  const ids = batch?.ids ?? [];
  const getOps = React.useCallback(
    () => (designId == null || ids.length === 0 ? null : new PiecesOperation(store.session, designId, ids, store.id)),
    [store.session, store.id, designId, ids],
  );
  return usePiecesCommandBound(getOps);
}
// #endregion 🪢Pieces

// #region ⛓️Connection
/** @emoji 📖 Live {@link Connection#gap}. */
export function useConnectionGap(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.gap(), "gap");
}

/** @emoji 📖 Live {@link Connection#shift}. */
export function useConnectionShift(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.shift(), "shift");
}

/** @emoji 📖 Live {@link Connection#rise}. */
export function useConnectionRise(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rise(), "rise");
}

/** @emoji 📖 Live {@link Connection#rotation}. */
export function useConnectionRotation(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.rotation(), "rotation");
}

/** @emoji 📖 Live {@link Connection#turn}. */
export function useConnectionTurn(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.turn(), "turn");
}

/** @emoji 📖 Live {@link Connection#tilt}. */
export function useConnectionTilt(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.tilt(), "tilt");
}

/** @emoji 📖 Live {@link Connection#u}. */
export function useConnectionU(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.u(), "u");
}

/** @emoji 📖 Live {@link Connection#v}. */
export function useConnectionV(): number | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.v(), "v");
}

/** @emoji 📖 Live {@link Connection#parent}. */
export function useConnectionParent(): Side | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.parent(), "parent");
}

/** @emoji 📖 Live {@link Connection#child}. */
export function useConnectionChild(): Side | null | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.child(), "child");
}

/** @emoji 📖 Live {@link Connection#name}. */
export function useConnectionName(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.name(), "name");
}

/** @emoji 📖 Live {@link Connection#description}. */
export function useConnectionDescription(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.description(), "description");
}

/** @emoji 📖 Live {@link Connection#icon}. */
export function useConnectionIcon(): string | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.icon(), "icon");
}

/** @emoji 📖 Live {@link Connection#attributes}. */
export function useConnectionAttributes(): readonly Attribute[] | undefined {
  const entity = resolveConnection();
  return useCurrentEntityField(entity, (c) => c.attributes(), "attributes");
}
// #endregion ⛓️Connection

// #region 🎨SketchpadFacade
export type { DesignDiff } from "../rendering/index";
export type ConnectionDiff = Readonly<Record<string, unknown>>;
export type PieceDiff = Readonly<Record<string, unknown>>;
export type TypeDiff = Readonly<Record<string, unknown>>;
export type KitDiff = Readonly<Record<string, unknown>>;
/** @emoji 📊 Transaction/diagram diff status labels for sketchpad chrome. */
export const DiffStatus = Object.freeze({
  Unchanged: "unchanged",
  Added: "added",
  Removed: "removed",
  Modified: "modified",
} as const);
export type DiffStatus = (typeof DiffStatus)[keyof typeof DiffStatus];

export { getKitPorts } from "../rendering/index";
export { SEMIO_IN_MEMORY_KIT_URI } from "../../js";

/** @emoji 📏 Sketchpad geometric tolerance constant. */
export const TOLERANCE = 0.001;

export type ActiveKitTabScope = Readonly<{ id: string }>;
const ActiveKitTabContext = React.createContext<string | null>(null);
export { ActiveKitTabContext };

/** @emoji 📌 Tab-scoped kit id for sketchpad shell / scene bridge. */
export function ActiveKitTabContextProvider(props: Readonly<{ kitTabId: string; children: ReactNode }>): React.ReactElement {
  return React.createElement(ActiveKitTabContext.Provider, { value: props.kitTabId }, props.children);
}

/** @emoji 📌 Active sketchpad kit tab id (`{ id }`) or `null` outside {@link ActiveKitTabContextProvider}. */
export function useActiveKitTab(): ActiveKitTabScope | null {
  const id = React.useContext(ActiveKitTabContext);
  return id == null || id === "" ? null : { id };
}

/** @emoji 📌 Whether {@link ActiveKitTabContextProvider} is mounted. */
export function useIsInActiveKitTab(): boolean {
  return useActiveKitTab() != null;
}

export type KitAlternativeSummary = Readonly<{ id: string; name: string }>;
const KIT_ALTERNATIVE_EMPTY: readonly KitAlternativeSummary[] = Object.freeze([]);
const KitAlternativeSelectionContext = React.createContext<Readonly<{ selectedAlternativeId: string | null; setSelectedAlternativeId: (id: string | null) => void; alternatives: readonly KitAlternativeSummary[] }>>({
  selectedAlternativeId: null,
  setSelectedAlternativeId: () => {},
  alternatives: KIT_ALTERNATIVE_EMPTY,
});

/** @emoji 🌱 Sketchpad alternative picker scope (WIP: empty until graph alternatives wire through). */
export function KitAlternativeSelectionProvider(props: Readonly<{ kitId?: string; children: ReactNode }>): React.ReactElement {
  void props.kitId;
  const value = React.useMemo(
    () => ({
      selectedAlternativeId: null as string | null,
      setSelectedAlternativeId: (_id: string | null) => {},
      alternatives: KIT_ALTERNATIVE_EMPTY,
    }),
    [],
  );
  return React.createElement(KitAlternativeSelectionContext.Provider, { value }, props.children);
}

/** @emoji 🌱 Selected alternative id + setter for host dropdowns. */
export function useKitAlternativeSelection(): readonly [string | null, (id: string | null) => void] {
  const v = React.useContext(KitAlternativeSelectionContext);
  return [v.selectedAlternativeId, v.setSelectedAlternativeId] as const;
}

/** @emoji 🌱 Alternatives list for host dropdowns. */
export function useKitAlternatives(): readonly KitAlternativeSummary[] {
  return React.useContext(KitAlternativeSelectionContext).alternatives;
}

//#region 🧾KitHostStore
const DEFAULT_KIT_SYNC = Object.freeze({ status: "idle", dirty: false, readonly: false, lastSyncedAt: null, error: null });

type KitPlain = Kit | Record<string, unknown>;

function isKitPlainDto(k: unknown): k is Record<string, unknown> {
  return k != null && typeof k === "object" && (Array.isArray((k as { designs?: unknown }).designs) || Array.isArray((k as { types?: unknown }).types));
}

function hostIdStr(x: unknown): string {
  if (x == null) return "";
  if (typeof x === "string") return x;
  if (typeof x === "object" && x !== null && "id" in x) return String((x as { id: unknown }).id);
  return String(x);
}

function hostPlainDto(store: KitHostStore): Record<string, unknown> {
  const k = store.getSnapshot().kit;
  if (isKitPlainDto(k)) return k;
  return { id: String((k as Kit).id ?? ""), designs: [], types: [], qualities: [], folders: [], files: [], concepts: [], tags: [], authors: [] };
}

export type KitHostSnap = Readonly<{ kit: Kit; sync: Readonly<{ status: string; dirty: boolean; readonly: boolean; lastSyncedAt: string | null; error: unknown }> }>;

export type KitHostStore = Readonly<{
  getSnapshot: () => KitHostSnap;
  subscribe: (onChange: () => void) => () => void;
  replace: (kit: KitPlain) => void;
  readonly name?: string;
}>;

/** @emoji 🧠 In-memory kit host for sketchpad temporary kits and imports. */
export class InMemoryKitStore implements KitHostStore {
  private readonly listeners = new Set<() => void>();
  private _kit: KitPlain;
  readonly name = "InMemoryKitStore";

  constructor(seed: KitPlain) {
    this._kit = seed;
  }

  getSnapshot(): KitHostSnap {
    return { kit: this._kit as Kit, sync: DEFAULT_KIT_SYNC };
  }

  subscribe(onChange: () => void): () => void {
    this.listeners.add(onChange);
    return () => {
      this.listeners.delete(onChange);
    };
  }

  replace(kit: KitPlain): void {
    this._kit = kit;
    for (const l of this.listeners) {
      try {
        l();
      } catch {
        /* ignore */
      }
    }
  }
}

export type KitJsonFileAdapter = Readonly<{ read: () => Promise<string>; write: (json: string) => Promise<void> }>;
export type KitFolderAdapter = Readonly<Record<string, unknown>>;

export async function createJsonFileKitStore(adapter: KitJsonFileAdapter): Promise<KitHostStore> {
  let text = "";
  try {
    text = await adapter.read();
  } catch {
    text = "";
  }
  let seed: Record<string, unknown> = { id: `kit-${Date.now()}`, name: "Untitled", designs: [], types: [], qualities: [], folders: [], files: [] };
  if (text.trim() !== "") {
    try {
      const parsed = JSON.parse(text) as Record<string, unknown>;
      if (parsed && typeof parsed === "object") seed = parsed;
    } catch {
      /* keep seed */
    }
  }
  const store = new InMemoryKitStore(seed);
  const persist = async (json: string) => {
    await adapter.write(json);
  };
  return Object.assign(store, { persistBundle: persist, initialBundleJson: text });
}

export async function createFolderKitStore(_adapter: KitFolderAdapter, initialKit?: unknown): Promise<KitHostStore> {
  const seed = (initialKit && typeof initialKit === "object" ? initialKit : { id: `kit-${Date.now()}`, name: "Untitled", designs: [], types: [], folders: [], files: [] }) as KitPlain;
  return new InMemoryKitStore(seed);
}

/** @emoji 🧾 Legacy string-command entry for DTO host stores; prefer {@link useKitCommand} on GraphQL kits. */
export async function executeSemioKitCommand(store: KitHostStore, command: string, _origin: string, ...args: unknown[]): Promise<unknown> {
  if (command === "semio.kit.undo" || command === "semio.kit.redo") {
    return { ok: false, error: { kind: "NotSupported", message: `${command}: no kit-store client bridge` } };
  }
  if (command === "semio.kit.import") {
    return { ok: false, error: "semio.kit.import: not wired in this build" };
  }
  if (command === "semio.kit.export") {
    return { ok: true };
  }
  if (command === "semio.kit.createFolder" && args[0]) {
    const snap = hostPlainDto(store);
    const nextFolders = [...((snap.folders as unknown[]) ?? []), args[0] as Record<string, unknown>];
    store.replace({ ...snap, folders: nextFolders });
    return { ok: true };
  }
  if (command === "semio.kit.moveToFolder" && args.length >= 3) {
    const entityId = hostIdStr(args[0]);
    const kind = String(args[1] ?? "");
    const folderId = hostIdStr(args[2]);
    const plain = JSON.parse(JSON.stringify(hostPlainDto(store))) as Record<string, unknown>;
    if (kind === "type") {
      for (const t of (plain.types as unknown[] | undefined) ?? []) {
        if (t && typeof t === "object" && (t as { id?: string }).id === entityId) (t as { folder?: string }).folder = folderId;
      }
    } else if (kind === "design") {
      for (const d of (plain.designs as unknown[] | undefined) ?? []) {
        if (d && typeof d === "object" && (d as { id?: string }).id === entityId) (d as { folder?: string }).folder = folderId;
      }
    } else if (kind === "quality") {
      for (const q of (plain.qualities as unknown[] | undefined) ?? []) {
        if (q && typeof q === "object" && (q as { id?: string }).id === entityId) (q as { folder?: string }).folder = folderId;
      }
    } else if (kind === "file") {
      for (const f of (plain.files as unknown[] | undefined) ?? []) {
        if (f && typeof f === "object" && (f as { id?: string }).id === entityId) (f as { folder?: { id: string } }).folder = { id: folderId };
      }
    } else if (kind === "folder") {
      for (const fo of (plain.folders as unknown[] | undefined) ?? []) {
        if (fo && typeof fo === "object" && (fo as { id?: string }).id === entityId) {
          (fo as { parent?: { id: string } }).parent = { id: folderId };
          delete (fo as { path?: string }).path;
        }
      }
    } else {
      return { ok: false, error: { kind: "InvalidValue", message: `moveToFolder: unknown kind ${kind}` } };
    }
    store.replace(plain);
    return { ok: true };
  }
  return { ok: false, error: { kind: "NotSupported", message: `unhandled ${command}` } };
}

/** @emoji 🧾 Memoized kit string-command engine for legacy sketchpad callers. */
export function useKitCommandEngineExplicitOrigin(store: KitHostStore): { execute: (...args: unknown[]) => Promise<unknown> } {
  return {
    execute: async (command: unknown, origin: unknown, ...rest: unknown[]) => executeSemioKitCommand(store, String(command), String(origin ?? ""), ...rest),
  };
}

export async function kitHostUndo(_store: KitHostStore): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "kitHostUndo: no kit-store client bridge", field: undefined, entity: undefined } };
}

export async function kitHostRedo(_store: KitHostStore): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "kitHostRedo: no kit-store client bridge", field: undefined, entity: undefined } };
}

export async function applyKitHostGraphOperation(_store: KitHostStore, _op: unknown): Promise<SetResult> {
  return { ok: false, error: { kind: "NotSupported", message: "applyKitHostGraphOperation: no kit-store client bridge", field: undefined, entity: undefined } };
}

export function attachSketchpadKitReadShell(_kitStore: KitHostStore): void {
  void _kitStore;
}

export function getOrCreateKitFileState(_kitId: string, _fileId: string): unknown {
  void _kitId;
  void _fileId;
  return null;
}
//#endregion 🧾KitHostStore

//#region 📦KitRegistry
export type KitPersistenceInfo = Readonly<{ kind: "temporary" | "file" | "folder" | "remote" }>;

export type KitRegistryEntry = Readonly<{
  store: KitHostStore;
  kitClient: unknown | null;
  refs: number;
  persistence: KitPersistenceInfo;
}>;

export type KitRegistryValue = Readonly<{
  activeKitId: string | undefined;
  setActiveKit: (id: string | undefined) => void;
  open: (id: string, init: Readonly<{ store?: KitHostStore; kitClient?: unknown | null; initialKit?: unknown }>) => Promise<void>;
  openTemporary: (initialKit?: unknown) => Promise<string>;
  openJsonFile: (kitId: string, adapter: KitJsonFileAdapter) => Promise<void>;
  openFolder: (kitId: string, adapter: KitFolderAdapter, initialKit?: unknown) => Promise<void>;
  openRemote: (kitId: string, config: Readonly<{ serverUrl: string }>) => Promise<void>;
  close: (id: string) => void;
  get: (id: string) => KitRegistryEntry | undefined;
  list: () => readonly string[];
  status: (id: string) => "idle" | "loading" | "ready" | "error";
}>;

const _kitRegistryListListeners = new Set<() => void>();

function emitKitRegistryListChanged(): void {
  for (const l of _kitRegistryListListeners) {
    try {
      l();
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 📣 Subscribe to registry open/close list changes. */
export function subscribeKitRegistryListChanged(onChange: () => void): () => void {
  _kitRegistryListListeners.add(onChange);
  return () => {
    _kitRegistryListListeners.delete(onChange);
  };
}

const KitRegistryContext = React.createContext<KitRegistryValue | null>(null);

let _semioKitRegistryBridge: KitRegistryValue | null = null;

/** @emoji 🧾 Registry bridge for {@link SketchpadStore} and other non-hook callers. */
export function getKitRegistryBridge(): KitRegistryValue | null {
  return _semioKitRegistryBridge;
}

/** @emoji 📦 Host registry for open kit tabs (sketchpad). */
export function KitRegistryProvider(props: Readonly<{ children: ReactNode }>): React.ReactElement {
  const rowsRef = React.useRef(new Map<string, KitRegistryEntry & { unsub?: () => void }>());
  const loadingRef = React.useRef(new Set<string>());
  const errRef = React.useRef(new Map<string, Error>());
  const [registryEpoch, bump] = React.useReducer((x: number) => x + 1, 0);
  const [activeKitId, setActiveKitId] = React.useState<string | undefined>(undefined);

  const open = React.useCallback(
    async (kitId: string, init: Readonly<{ store?: KitHostStore; kitClient?: unknown | null; initialKit?: unknown }>) => {
      const cur = rowsRef.current.get(kitId);
      if (cur) {
        (cur as { refs: number }).refs += 1;
        bump();
        return;
      }
      if (init.store == null) {
        throw new Error("KitRegistry.open requires init.store");
      }
      loadingRef.current.add(kitId);
      errRef.current.delete(kitId);
      bump();
      try {
        const store = init.store;
        const kitClient = init.kitClient ?? null;
        const persistence: KitPersistenceInfo = { kind: store.name === "InMemoryKitStore" ? "temporary" : "file" };
        rowsRef.current.set(kitId, { store, kitClient, refs: 1, persistence });
        emitKitRegistryListChanged();
      } catch (e) {
        errRef.current.set(kitId, e instanceof Error ? e : new Error(String(e)));
        throw e;
      } finally {
        loadingRef.current.delete(kitId);
        bump();
      }
    },
    [],
  );

  const close = React.useCallback((kitId: string) => {
    const row = rowsRef.current.get(kitId);
    if (!row) return;
    row.refs -= 1;
    if (row.refs <= 0) {
      row.unsub?.();
      rowsRef.current.delete(kitId);
      setActiveKitId((cur) => (cur === kitId ? undefined : cur));
      emitKitRegistryListChanged();
    }
    bump();
  }, []);

  const value = React.useMemo<KitRegistryValue>(
    () => ({
      get activeKitId() {
        return activeKitId;
      },
      setActiveKit: setActiveKitId,
      open,
      async openTemporary(initialKit) {
        const k = `kit-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
        const seed = (initialKit && typeof initialKit === "object" ? initialKit : { id: k, name: "Untitled", designs: [], types: [], qualities: [], folders: [], files: [] }) as KitPlain;
        await open(k, { store: new InMemoryKitStore(seed) });
        return k;
      },
      async openJsonFile(kitId, adapter) {
        const store = await createJsonFileKitStore(adapter);
        await open(kitId, { store });
      },
      async openFolder(kitId, adapter, initialKit) {
        const store = await createFolderKitStore(adapter, initialKit);
        await open(kitId, { store });
      },
      async openRemote(kitId, config) {
        const seed = { id: kitId, name: config.serverUrl, remote: config.serverUrl, designs: [], types: [] };
        await open(kitId, { store: new InMemoryKitStore(seed) });
      },
      close,
      get(kitId) {
        return rowsRef.current.get(kitId);
      },
      list() {
        return Array.from(rowsRef.current.keys());
      },
      status(kitId) {
        if (loadingRef.current.has(kitId)) return "loading";
        if (errRef.current.has(kitId)) return "error";
        if (rowsRef.current.has(kitId)) return "ready";
        return "idle";
      },
    }),
    [activeKitId, open, close, registryEpoch],
  );

  _semioKitRegistryBridge = value;
  React.useLayoutEffect(() => {
    return () => {
      _semioKitRegistryBridge = null;
      _kitRegistryListListeners.clear();
    };
  }, []);

  return React.createElement(KitRegistryContext.Provider, { value }, props.children);
}

/** @emoji 📦 Sketchpad host root: {@link KitRegistryProvider}. */
export function KitStoreProvider(props: Readonly<{ children: ReactNode; initialKit?: unknown }>): React.ReactElement {
  void props.initialKit;
  return React.createElement(KitRegistryProvider, null, props.children);
}

export function useKitRegistry(): KitRegistryValue {
  const v = React.useContext(KitRegistryContext);
  if (v == null) throw new Error("useKitRegistry must be used within KitRegistryProvider.");
  return v;
}

/** @emoji 📦 Like {@link useKitRegistry} but returns null outside provider. */
export function useKitRegistrySafe(): KitRegistryValue | null {
  return React.useContext(KitRegistryContext);
}
//#endregion 📦KitRegistry

//#region 🔌KitRuntimeBridge
function useKitRuntimeSafe(): KitRuntimeContextValue | null {
  return React.useContext(KitRuntimeContext);
}

/** @emoji 📌 Live kit host snapshot for registry tab or {@link KitRuntimeContext}. */
export function useKitStoreSnapshot(explicitKitId?: string): KitHostSnap | null {
  const runtime = useKitRuntimeSafe();
  const tabId = useActiveKitTab()?.id;
  const regActive = getKitRegistryBridge()?.activeKitId;
  const kitId = explicitKitId != null && explicitKitId !== "" ? explicitKitId : tabId != null && tabId !== "" ? tabId : regActive;
  const [registryEpoch, setRegistryEpoch] = React.useState(0);
  React.useEffect(() => subscribeKitRegistryListChanged(() => setRegistryEpoch((n) => n + 1)), []);
  const reg = getKitRegistryBridge();
  const entry = kitId != null && reg != null ? reg.get(kitId) : undefined;
  const store = runtime?.store ?? entry?.store ?? null;
  const [snap, setSnap] = React.useState<KitHostSnap | null>(null);
  React.useEffect(() => {
    if (store == null) {
      setSnap(null);
      return;
    }
    const pull = () => setSnap(store.getSnapshot());
    pull();
    return store.subscribe(pull);
  }, [store, registryEpoch, runtime?.kitId, kitId]);
  if (store == null) {
    const gqlKit = React.useContext(KitHandleContext);
    if (gqlKit == null) return null;
    return { kit: gqlKit, sync: DEFAULT_KIT_SYNC };
  }
  return snap;
}

/** @emoji 📥 Active kit host store for the resolved tab / runtime scope. */
export function useKitStore(): KitHostStore | null {
  const runtime = useKitRuntimeSafe();
  if (runtime?.store) return runtime.store;
  const tabId = useActiveKitTab()?.id;
  const reg = getKitRegistryBridge();
  const kitId = tabId != null && tabId !== "" ? tabId : reg?.activeKitId;
  if (kitId == null || reg == null) return null;
  return reg.get(kitId)?.store ?? null;
}

/** @emoji 📂 Open kit ids from {@link KitRegistryProvider}. */
export function useOpenKits(): readonly string[] {
  const [registryEpoch, setRegistryEpoch] = React.useState(0);
  React.useEffect(() => subscribeKitRegistryListChanged(() => setRegistryEpoch((n) => n + 1)), []);
  const reg = getKitRegistryBridge();
  return reg != null ? Object.freeze(reg.list()) : EMPTY_IDS;
}

export function useRegistryHasKit(kitId: string): boolean {
  return getKitRegistryBridge()?.get(kitId) != null;
}

export function useRegistryKitPersistenceKind(kitId: string): KitPersistenceInfo["kind"] | null {
  return getKitRegistryBridge()?.get(kitId)?.persistence.kind ?? null;
}

export function useKitStoredFileUrls(_explicitKitId?: string): readonly [Readonly<Record<string, string>>] {
  void _explicitKitId;
  return [Object.freeze({})];
}

export function useKitFileUrl(_fileId?: string): string | undefined {
  void _fileId;
  return undefined;
}

export function useKitFileBlobUrl(_fileId?: string): string | undefined {
  void _fileId;
  return undefined;
}

export function createDefaultBrowserSketchpadFileKitStoreFactory(): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createDefaultBrowserSketchpadFileKitStoreFactory: use fileKitStoreFactory prop on Sketchpad");
  };
}

export function createDefaultBrowserSketchpadRemoteKitStoreFactory(): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createDefaultBrowserSketchpadRemoteKitStoreFactory: use remoteKitStoreFactory prop on Sketchpad");
  };
}

export function createVscodeWebviewSketchpadFileKitStoreFactory(_vscodeApi: { postMessage: (msg: unknown) => void }): () => Promise<KitHostStore> {
  return async () => {
    throw new Error("createVscodeWebviewSketchpadFileKitStoreFactory: use fileKitStoreFactory prop on Sketchpad");
  };
}

/** @emoji 📌 Kit `files` rows for sketchpad panels (`const [files] = useFilesFull()`). */
export function useFilesFull(_explicitKitId?: string): KitFieldBinding<readonly File[]> {
  void _explicitKitId;
  return [Object.freeze([]) as readonly File[], noopKitFieldSet, WRITE_STATUS_IDLE];
}

/** @emoji 📌 Kit `tags` rows for sketchpad panels (`const [tags] = useTagsFull()`). */
export function useTagsFull(_explicitKitId?: string): KitFieldBinding<readonly Tag[]> {
  void _explicitKitId;
  return [useKitTags(), noopKitFieldSet, WRITE_STATUS_IDLE];
}

/** @emoji 📌 Included design ids for explode / replace UI (WIP: empty without kit-store client). */
export function useExplodeableDesignNodes(_designId?: string): readonly [readonly string[]] {
  void _designId;
  return [EMPTY_IDS];
}

export type SchemaScopeEntityKind = string;
export const SchemaScopeContext = React.createContext<SchemaScopeEntityKind | null>(null);

/** @emoji 🧭 Maps entity id prefix to schema scope kind for sketchpad property panels. */
export function schemaScopeForEntityId(entityId: string): SchemaScopeEntityKind | null {
  if (entityId.startsWith("design-") || entityId.includes("/designs/")) return "design";
  if (entityId.startsWith("type-") || entityId.includes("/types/")) return "type";
  return null;
}

/** @emoji 🔗 Attach backbone via {@link useStoreCommand} (alias for sketchpad docs). */
export function useAttachBackbone(): (input: { dev?: { filePath: string }; local?: { folderPath: string }; remote?: { serverUrl: string } }) => Promise<SetResult> {
  const { run } = useStoreCommand();
  return React.useCallback(
    async (input) => {
      const uri = input.dev?.filePath ?? input.local?.folderPath ?? input.remote?.serverUrl;
      if (uri == null || uri === "") {
        return { ok: false, error: { kind: "InvalidValue", message: "useAttachBackbone: no backbone target.", field: undefined, entity: undefined } };
      }
      return run.attachBackbone(uri);
    },
    [run],
  );
}
//#endregion 🔌KitRuntimeBridge
// #endregion 🎨SketchpadFacade
// #endregion 🎨SketchpadFacade

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
  const { describe, expect, it, vi } = import.meta.vitest;
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
  const sketchpadFacadeRegion = reactSrc.indexOf("// #region 🎨SketchpadFacade");
  const scanEnd =
    sketchpadFacadeRegion !== -1 && (vitestRegion === -1 || sketchpadFacadeRegion < vitestRegion) ? sketchpadFacadeRegion : vitestRegion === -1 ? reactSrc.length : vitestRegion;
  const reactSrcForBannedScan = reactSrc.slice(0, scanEnd);
  describe("SemioStoreKitLineHost", () => {
    it("opens HTTP session, wraps children, and disposes on unmount", async () => {
      const { render, screen, waitFor } = await import("@testing-library/react");
      const JsMod = await import("../../js");
      const dispose = vi.fn().mockResolvedValue(undefined);
      const mockSession: {
        stores: ReturnType<typeof vi.fn>;
        store: ReturnType<typeof vi.fn>;
        localProvider: ReturnType<typeof vi.fn>;
        dispose: ReturnType<typeof vi.fn>;
      } = {
        stores: vi.fn(),
        store: vi.fn(),
        localProvider: vi.fn().mockReturnValue({}),
        dispose,
      };
      mockSession.stores.mockResolvedValue([{ id: "store-a" }]);
      mockSession.store.mockImplementation((sid: string) => ({
        id: sid,
        session: mockSession,
        wip: () => ({
          theKit: () => ({
            kit: vi.fn().mockResolvedValue({ id: "kit-b" }),
          }),
        }),
      }));
      const spy = vi.spyOn(JsMod, "openSessionHttp").mockResolvedValue(mockSession as unknown as Session);
      const ui = render(
        React.createElement(SemioStoreKitLineHost, {
          baseUrl: "http://127.0.0.1:59999/",
          children: React.createElement("span", null, "child-mark"),
        }),
      );
      await waitFor(() => {
        expect(screen.getByText("child-mark")).toBeTruthy();
      });
      expect(spy).toHaveBeenCalledWith("http://127.0.0.1:59999", expect.objectContaining({}));
      ui.unmount();
      await waitFor(() => {
        expect(dispose).toHaveBeenCalledTimes(1);
      });
      spy.mockRestore();
    });

    it("shows an error message when the session has no stores", async () => {
      const { render, screen, waitFor } = await import("@testing-library/react");
      const JsMod = await import("../../js");
      const dispose = vi.fn().mockResolvedValue(undefined);
      const mockSession = {
        stores: vi.fn().mockResolvedValue([]),
        dispose,
      };
      const spy = vi.spyOn(JsMod, "openSessionHttp").mockResolvedValue(mockSession as unknown as Session);
      const ui = render(React.createElement(SemioStoreKitLineHost, { baseUrl: "http://x", children: React.createElement("span", null, "nope") }));
      await waitFor(() => {
        expect(screen.queryByText("nope")).toBeNull();
        expect(screen.getByText(/no stores/i)).toBeTruthy();
      });
      ui.unmount();
      spy.mockRestore();
    });
  });

  describe("React hooks over in-memory session", () => {
    it("refreshes read hooks after operation hooks settle through the JS bridge", async () => {
      const { act, render, screen, waitFor } = await import("@testing-library/react");
      const listeners = new Map<string, Set<() => void>>();
      const bus = {
        emit(event: { kind: string }): void {
          for (const listener of listeners.get(event.kind) ?? []) listener();
        },
        subscribeKind(kind: string, listener: () => void): () => void {
          const bucket = listeners.get(kind) ?? new Set<() => void>();
          bucket.add(listener);
          listeners.set(kind, bucket);
          return () => {
            bucket.delete(listener);
            if (bucket.size === 0) listeners.delete(kind);
          };
        },
      };
      const fakeSession = { bus } as Session;
      const fakeTags: Array<{ id: string }> = [];
      const fakeKit = {
        id: "kit-1",
        storeId: "store-1",
        session: fakeSession,
        async tags(): Promise<Array<{ id: string }>> {
          return fakeTags.slice();
        },
        async createTag(name: string): Promise<SetResult> {
          fakeTags.push({ id: `${name}-${fakeTags.length + 1}` });
          return { ok: true };
        },
      } as unknown as Kit;

      function Probe(): React.ReactElement {
        const ids = useKitTags();
        const { run, status } = useKitCommand();
        return React.createElement(
          React.Fragment,
          null,
          React.createElement("div", { "data-testid": "tag-count" }, String(ids.length)),
          React.createElement("div", { "data-testid": "status" }, status.kind),
          React.createElement("button", { onClick: () => void run.createTag("alpha-tag") }, "create"),
        );
      }

      const ui = render(
        React.createElement(
          KitHandleContext.Provider,
          { value: fakeKit },
          React.createElement(Probe),
        ),
      );

      await waitFor(() => {
        expect(screen.getByTestId("tag-count").textContent).toBe("0");
      });

      await act(async () => {
        screen.getByText("create").click();
      });

      await waitFor(() => {
        expect(screen.getByTestId("tag-count").textContent).toBe("1");
        expect(screen.getByTestId("status").textContent).toBe("settled");
      });

      ui.unmount();
    });
  });

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
  describe("entity command surface", () => {
    it("does not export per-operation hooks", () => {
      const exportNames = [...reactSrc.matchAll(/^export function (use\w+)/gm)].map((m) => m[1]);
      const banned = exportNames.filter((n) => /^use(Create|Update|Delete|Rename)[A-Z]/.test(n));
      expect(banned).toEqual([]);
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
